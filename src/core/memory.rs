use std::path::{Path, PathBuf};
use std::sync::Mutex as StdMutex;

use anyhow::{anyhow, Context, Result};
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy};

// ── Table definitions ───────────────────────────────────────────────────────

const SESSIONS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("sessions");
const PREFERENCES_TABLE: TableDefinition<&str, &str> = TableDefinition::new("preferences");
const MEMORY_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("memories");

// ── Public types ────────────────────────────────────────────────────────────

/// A single chat message stored in session history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    /// One of: "system", "user", "assistant", "tool"
    pub role: String,
    /// The text content of the message.
    pub content: String,
    /// ISO-8601 timestamp of when the message was created.
    pub timestamp: String,
}

impl Message {
    /// Create a new message with the current UTC time as its timestamp.
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Convert this message into a JSON `Value` suitable for sending to an LLM.
    pub fn to_json_value(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "role".to_string(),
            serde_json::Value::String(self.role.clone()),
        );
        map.insert(
            "content".to_string(),
            serde_json::Value::String(self.content.clone()),
        );
        serde_json::Value::Object(map)
    }
}

// ── MemoryEntry ─────────────────────────────────────────────────────────────

/// A generic memory entry that can be stored, recalled, and full-text searched.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique identifier for this entry.
    pub id: String,
    /// The primary text content (full-text searchable).
    pub content: String,
    /// Arbitrary JSON metadata (tags, source, importance, etc.).
    pub metadata: serde_json::Value,
    /// ISO-8601 timestamp.
    pub timestamp: String,
    /// Category / type label for filtering.
    pub entry_type: String,
}

impl MemoryEntry {
    /// Create a new memory entry with the current UTC timestamp and a generated UUID v4 id.
    pub fn new(content: impl Into<String>, entry_type: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.into(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            timestamp: chrono::Utc::now().to_rfc3339(),
            entry_type: entry_type.into(),
        }
    }

    /// Create a new memory entry with a custom metadata value.
    pub fn with_metadata(
        content: impl Into<String>,
        entry_type: impl Into<String>,
        metadata: serde_json::Value,
    ) -> Self {
        let mut entry = Self::new(content, entry_type);
        entry.metadata = metadata;
        entry
    }
}

// ── Tantivy schema builder ──────────────────────────────────────────────────

/// Build the tantivy schema used for full-text indexing of memory entries.
fn build_tantivy_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("id", STRING | STORED);
    schema_builder.add_text_field("content", TEXT | STORED);
    schema_builder.add_text_field("entry_type", STRING | STORED);
    schema_builder.add_text_field("timestamp", STRING | STORED);
    schema_builder.add_text_field("metadata_json", TEXT | STORED);
    schema_builder.build()
}

// ── Memory store ────────────────────────────────────────────────────────────

/// Persistent session, preference, and memory storage backed by redb,
/// with full-text search powered by tantivy.
pub struct MemoryStore {
    db: Database,
    /// Tantivy full-text search index.
    tantivy_index: Index,
    /// Mutex-protected writer — tantivy IndexWriter is `Send` but not `Sync`.
    tantivy_writer: StdMutex<IndexWriter>,
    /// Reader for search queries, reloaded on each search to pick up new commits.
    tantivy_reader: IndexReader,
}

impl MemoryStore {
    /// Open or create a redb database at the given path.
    /// A companion tantivy index directory is created at `<path>.tantivy_index/`.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db_path = path.as_ref();
        let db = Database::create(db_path).context("Failed to open redb database")?;

        // Derive tantivy index path from the redb database path.
        let index_dir = db_path.with_extension("");
        let tantivy_index_path = if let Some(stem) = index_dir.file_stem() {
            let mut p = index_dir.clone();
            let mut new_name = stem.to_os_string();
            new_name.push(".tantivy_index");
            p.set_file_name(new_name);
            p
        } else {
            PathBuf::from("memory.tantivy_index")
        };

        // Ensure the tantivy index directory exists.
        std::fs::create_dir_all(&tantivy_index_path)
            .context("Failed to create tantivy index directory")?;

        let dir = MmapDirectory::open(&tantivy_index_path)
            .context("Failed to open tantivy index directory")?;

        let schema = build_tantivy_schema();
        let tantivy_index = if tantivy_index_path.join("meta.json").exists() {
            Index::open(dir).context("Failed to open existing tantivy index")?
        } else {
            Index::create(dir, schema, tantivy::IndexSettings::default())
                .context("Failed to create tantivy index")?
        };

        let tantivy_writer = StdMutex::new(
            tantivy_index
                .writer(50_000_000)
                .context("Failed to create tantivy index writer")?,
        );

        let tantivy_reader = tantivy_index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .context("Failed to create tantivy reader")?;

        Ok(Self {
            db,
            tantivy_index,
            tantivy_writer,
            tantivy_reader,
        })
    }

    // ── Sessions ─────────────────────────────────────────────────────────

    /// Save a list of messages associated with a session ID.
    /// Overwrites any previously stored messages for this session.
    pub fn save_session(&self, session_id: &str, messages: &[Message]) -> Result<()> {
        let serialized =
            serde_json::to_vec(messages).context("Failed to serialize session messages")?;

        let write_txn = self
            .db
            .begin_write()
            .context("Failed to begin write transaction")?;
        {
            let mut table = write_txn
                .open_table(SESSIONS_TABLE)
                .context("Failed to open sessions table")?;
            table
                .insert(session_id, serialized.as_slice())
                .context("Failed to insert session")?;
        }
        write_txn
            .commit()
            .context("Failed to commit session write")?;
        Ok(())
    }

    /// Load all messages for a session. Returns an empty Vec if the session
    /// does not exist.
    pub fn load_session(&self, session_id: &str) -> Result<Vec<Message>> {
        let read_txn = self
            .db
            .begin_read()
            .context("Failed to begin read transaction")?;

        let raw: Vec<u8> = {
            let table = read_txn
                .open_table(SESSIONS_TABLE)
                .context("Failed to open sessions table")?;

            let result = table.get(session_id);
            match result {
                Ok(Some(guard)) => {
                    let bytes: &[u8] = guard.value();
                    bytes.to_vec()
                }
                Ok(None) => return Ok(Vec::new()),
                Err(e) => return Err(anyhow!("Failed to read session: {}", e)),
            }
        };

        serde_json::from_slice(&raw).context("Failed to deserialize session messages")
    }

    // ── Preferences ───────────────────────────────────────────────────────

    /// Save a key-value preference string.
    pub fn save_preference(&self, key: &str, value: &str) -> Result<()> {
        let write_txn = self
            .db
            .begin_write()
            .context("Failed to begin write transaction")?;
        {
            let mut table = write_txn
                .open_table(PREFERENCES_TABLE)
                .context("Failed to open preferences table")?;
            table
                .insert(key, value)
                .context("Failed to insert preference")?;
        }
        write_txn
            .commit()
            .context("Failed to commit preference write")?;
        Ok(())
    }

    /// Load a preference value by key. Returns `None` if not found.
    pub fn load_preference(&self, key: &str) -> Result<Option<String>> {
        let read_txn = self
            .db
            .begin_read()
            .context("Failed to begin read transaction")?;

        let value: Option<String> = {
            let table = read_txn
                .open_table(PREFERENCES_TABLE)
                .context("Failed to open preferences table")?;

            let result = table.get(key);
            match result {
                Ok(Some(guard)) => Some(guard.value().to_string()),
                Ok(None) => None,
                Err(e) => return Err(anyhow!("Failed to read preference: {}", e)),
            }
        };

        Ok(value)
    }

    // ── Memory: store / recall / search ──────────────────────────────────

    /// Store a memory entry in both the redb database (for persistence) and
    /// the tantivy index (for full-text search).
    pub fn store(&self, entry: &MemoryEntry) -> Result<()> {
        // 1. Persist to redb
        let serialized =
            serde_json::to_vec(entry).context("Failed to serialize memory entry")?;

        let write_txn = self
            .db
            .begin_write()
            .context("Failed to begin write transaction")?;
        {
            let mut table = write_txn
                .open_table(MEMORY_TABLE)
                .context("Failed to open memory table")?;
            table
                .insert(entry.id.as_str(), serialized.as_slice())
                .context("Failed to insert memory entry")?;
        }
        write_txn
            .commit()
            .context("Failed to commit memory write")?;

        // 2. Index in tantivy
        let metadata_json = serde_json::to_string(&entry.metadata).unwrap_or_default();

        let schema = self.tantivy_index.schema();
        let id_field = schema.get_field("id").unwrap();
        let content_field = schema.get_field("content").unwrap();
        let entry_type_field = schema.get_field("entry_type").unwrap();
        let timestamp_field = schema.get_field("timestamp").unwrap();
        let metadata_field = schema.get_field("metadata_json").unwrap();

        let tantivy_doc = doc!(
            id_field => entry.id.clone(),
            content_field => entry.content.clone(),
            entry_type_field => entry.entry_type.clone(),
            timestamp_field => entry.timestamp.clone(),
            metadata_field => metadata_json,
        );

        {
            let mut writer = self
                .tantivy_writer
                .lock()
                .map_err(|e| anyhow!("Tantivy writer lock poisoned: {}", e))?;
            writer
                .add_document(tantivy_doc)
                .context("Failed to add document to tantivy index")?;
            writer
                .commit()
                .context("Failed to commit tantivy index")?;
        }

        // Refresh the reader so it sees the new document.
        self.tantivy_reader.reload().ok();

        Ok(())
    }

    /// Recall a single memory entry by its unique ID from the redb store.
    pub fn recall(&self, id: &str) -> Result<Option<MemoryEntry>> {
        let read_txn = self
            .db
            .begin_read()
            .context("Failed to begin read transaction")?;

        let raw: Vec<u8> = {
            let table = read_txn
                .open_table(MEMORY_TABLE)
                .context("Failed to open memory table")?;

            let result = table.get(id);
            match result {
                Ok(Some(guard)) => {
                    let bytes: &[u8] = guard.value();
                    bytes.to_vec()
                }
                Ok(None) => return Ok(None),
                Err(e) => return Err(anyhow!("Failed to read memory entry: {}", e)),
            }
        };

        let entry: MemoryEntry =
            serde_json::from_slice(&raw).context("Failed to deserialize memory entry")?;

        Ok(Some(entry))
    }

    /// Full-text search across all stored memory entries using tantivy.
    /// Returns up to `limit` results, ranked by relevance score.
    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<MemoryEntry>> {
        // Refresh reader to pick up any uncommitted documents.
        self.tantivy_reader.reload().ok();

        let searcher = self
            .tantivy_reader
            .searcher();

        let schema = self.tantivy_index.schema();
        let content_field = schema.get_field("content").unwrap();

        let query_parser = QueryParser::for_index(&self.tantivy_index, vec![content_field]);
        let query = query_parser
            .parse_query(query_str)
            .context("Failed to parse search query")?;

        let collector = TopDocs::with_limit(limit).order_by_score();
        let top_docs = searcher
            .search(&query, &collector)
            .context("Tantivy search failed")?;

        let mut results: Vec<MemoryEntry> = Vec::with_capacity(top_docs.len());

        // Helper: extract a stored text field from a tantivy doc.
        fn get_stored_text(
            schema: &Schema,
            doc: &TantivyDocument,
            field_name: &str,
        ) -> String {
            schema
                .get_field(field_name)
                .ok()
                .and_then(|f| doc.get_first(f).and_then(|v| v.as_str().map(|s| s.to_string())))
                .unwrap_or_default()
        }

        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher
                .doc::<TantivyDocument>(doc_address)
                .context("Failed to retrieve document from tantivy index")?;

            let id = get_stored_text(&schema, &retrieved_doc, "id");
            let content = get_stored_text(&schema, &retrieved_doc, "content");
            let entry_type = get_stored_text(&schema, &retrieved_doc, "entry_type");
            let timestamp = get_stored_text(&schema, &retrieved_doc, "timestamp");

            let metadata: serde_json::Value = schema
                .get_field("metadata_json")
                .ok()
                .and_then(|f| {
                    retrieved_doc
                        .get_first(f)
                        .and_then(|v| v.as_str())
                        .and_then(|s| serde_json::from_str(s).ok())
                })
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

            results.push(MemoryEntry {
                id,
                content,
                metadata,
                timestamp,
                entry_type,
            });
        }

        Ok(results)
    }

    /// Delete a memory entry from both redb and tantivy by its ID.
    /// Note: tantivy deletion requires the document's internal doc id, so
    /// we delete from redb first; tantivy tombstone cleanup happens on merge.
    pub fn delete(&self, id: &str) -> Result<bool> {
        // Remove from redb
        let write_txn = self
            .db
            .begin_write()
            .context("Failed to begin write transaction")?;
        let existed = {
            let mut table = write_txn
                .open_table(MEMORY_TABLE)
                .context("Failed to open memory table")?;
            let removed = table.remove(id).is_ok();
            removed
        };
        write_txn.commit().context("Failed to commit delete")?;

        // For tantivy we use delete_term to remove by the id field.
        if existed {
            let schema = self.tantivy_index.schema();
            if let Ok(id_field) = schema.get_field("id") {
                let term = tantivy::Term::from_field_text(id_field, id);
                let mut writer = self
                    .tantivy_writer
                    .lock()
                    .map_err(|e| anyhow!("Tantivy writer lock poisoned: {}", e))?;
                writer.delete_term(term);
                writer.commit().ok();
            }
        }

        Ok(existed)
    }

    /// Return the number of stored memory entries.
    pub fn memory_count(&self) -> Result<usize> {
        let read_txn = self
            .db
            .begin_read()
            .context("Failed to begin read transaction")?;
        let table = read_txn
            .open_table(MEMORY_TABLE)
            .context("Failed to open memory table")?;
        Ok(table.len().unwrap_or(0) as usize)
    }
}
