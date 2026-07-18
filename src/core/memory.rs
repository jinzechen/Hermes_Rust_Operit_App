use std::path::Path;

use anyhow::{Context, Result};
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};

// ── Table definitions ───────────────────────────────────────────────────────

const SESSIONS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("sessions");
const PREFERENCES_TABLE: TableDefinition<&str, &str> = TableDefinition::new("preferences");

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

// ── Memory store ────────────────────────────────────────────────────────────

/// Persistent session and preference storage backed by redb.
pub struct MemoryStore {
    db: Database,
}

impl MemoryStore {
    /// Open or create a redb database at the given path.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db = Database::create(path).context("Failed to open redb database")?;
        Ok(Self { db })
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
                Err(e) => return Err(anyhow::anyhow!("Failed to read session: {}", e)),
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
                Err(e) => return Err(anyhow::anyhow!("Failed to read preference: {}", e)),
            }
        };

        Ok(value)
    }
}
