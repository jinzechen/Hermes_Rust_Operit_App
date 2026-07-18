//! Plugin installer — download, verify, extract, and install plugins.
//!
//! All operations are synchronous (reqwest::blocking).
//! Plugins are installed to `~/.hermes/plugins/{name}/`.

use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};

use super::index::{self, PluginIndex};

// ── download ───────────────────────────────────────────────────────

/// Download a file from `url` to a temporary location and return its path.
///
/// The caller is responsible for removing the file after use, or it will be
/// cleaned up on the next reboot (written to the system temp dir).
pub fn download_plugin(url: &str, dest: &Path) -> Result<PathBuf> {
    // If dest is a directory, append a filename derived from the URL
    let dest_path: PathBuf = if dest.is_dir() || dest.to_string_lossy().ends_with('/')
        || dest.to_string_lossy().ends_with('\\')
    {
        let filename = url
            .rsplit('/')
            .next()
            .unwrap_or("plugin.zip");
        dest.join(filename)
    } else {
        dest.to_path_buf()
    };

    // Ensure parent directory exists
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    log::info!("downloading {} -> {}", url, dest_path.display());

    let client = reqwest::blocking::Client::builder()
        .user_agent("Hermes-PluginInstaller/0.1")
        .build()
        .context("failed to build HTTP client")?;

    let response = client
        .get(url)
        .send()
        .context("download request failed")?;

    if !response.status().is_success() {
        bail!(
            "download failed with status {} for {}",
            response.status(),
            url
        );
    }

    let bytes = response.bytes().context("failed to read response body")?;

    let mut file = fs::File::create(&dest_path)
        .with_context(|| format!("failed to create file {}", dest_path.display()))?;
    file.write_all(&bytes)?;

    log::info!("downloaded {} bytes to {}", bytes.len(), dest_path.display());
    Ok(dest_path)
}

// ── checksum verification ──────────────────────────────────────────

/// Verify that a file matches the expected SHA-256 hex checksum.
///
/// Returns `Ok(true)` if the checksums match, `Ok(false)` if they don't,
/// or `Err` if the file cannot be read.
pub fn verify_checksum(path: &Path, expected_sha256: &str) -> Result<bool> {
    if expected_sha256.is_empty() {
        log::warn!("empty expected checksum — skipping verification for {}", path.display());
        return Ok(true);
    }

    let mut file =
        fs::File::open(path).with_context(|| format!("cannot open {} for checksum", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let computed = format!("{:x}", hasher.finalize());
    let matches = computed.eq_ignore_ascii_case(expected_sha256);

    if !matches {
        log::warn!(
            "checksum mismatch for {}: expected {} got {}",
            path.display(),
            expected_sha256,
            computed
        );
    }

    Ok(matches)
}

// ── archive extraction ─────────────────────────────────────────────

/// Extract a `.zip` archive to `dest_dir`.
///
/// Creates `dest_dir` if it does not exist. Handles nested directory
/// structures correctly.
pub fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<()> {
    fs::create_dir_all(dest_dir)?;

    let file = fs::File::open(zip_path)
        .with_context(|| format!("cannot open zip {}", zip_path.display()))?;
    let mut archive = zip::ZipArchive::new(file)
        .with_context(|| format!("cannot read zip {}", zip_path.display()))?;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .with_context(|| format!("cannot read zip entry {}", i))?;

        let entry_name = entry.name().to_string();

        // Skip directories (they are created implicitly via file extraction)
        if entry_name.ends_with('/') || entry_name.ends_with('\\') {
            continue;
        }

        let out_path = dest_dir.join(&entry_name);

        // Create parent directories
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut out_file = fs::File::create(&out_path)
            .with_context(|| format!("cannot create {}", out_path.display()))?;

        io::copy(&mut entry, &mut out_file)
            .with_context(|| format!("cannot write {}", out_path.display()))?;

        // On Unix, preserve executable bits; on Windows this is a no-op.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = entry.unix_mode() {
                let mut perms = out_file.metadata()?.permissions();
                perms.set_mode(mode);
                fs::set_permissions(&out_path, perms)?;
            }
        }
    }

    log::info!(
        "extracted {} to {}",
        zip_path.file_name().unwrap_or_default().to_string_lossy(),
        dest_dir.display()
    );
    Ok(())
}

// ── installation ───────────────────────────────────────────────────

/// Download, verify, extract, and install a plugin.
///
/// Steps:
/// 1. Download the zip from `plugin.download_url` to a temp file.
/// 2. Optionally verify the SHA-256 checksum.
/// 3. Extract to `~/.hermes/plugins/{name}/`.
/// 4. Make any binaries executable (Unix only).
/// 5. Return the installed plugin directory.
///
/// If the plugin directory already exists it is removed first (clean reinstall).
pub fn install_plugin(plugin: &PluginIndex) -> Result<PathBuf> {
    let plugins_root = index::plugins_dir()?;
    let dest_dir = plugins_root.join(&plugin.name);

    // Remove existing installation
    if dest_dir.exists() {
        log::info!("removing existing plugin at {}", dest_dir.display());
        fs::remove_dir_all(&dest_dir)?;
    }

    // Download to temp file
    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join(format!("{}.zip", plugin.name));

    let zip_path = download_plugin(&plugin.download_url, &tmp_file)?;

    // Verify checksum if provided
    if !plugin.sha256.is_empty() {
        let valid = verify_checksum(&zip_path, &plugin.sha256)?;
        if !valid {
            let _ = fs::remove_file(&zip_path);
            bail!(
                "checksum verification failed for plugin '{}'",
                plugin.name
            );
        }
    }

    // Extract
    extract_zip(&zip_path, &dest_dir)?;

    // Clean up temp file
    let _ = fs::remove_file(&zip_path);

    // Make executables runnable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for entry in walkdir::WalkDir::new(&dest_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            // Don't blindly chmod all files; only typical script/binary extensions
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            let should_chmod = ext.is_empty() // no extension = likely binary
                || ext == "sh"
                || ext == "bash"
                || ext == "py"
                || ext == "rb"
                || ext == "js"
                || ext == "ts"
                || name.ends_with("_server"); // MCP servers often have no extension

            if should_chmod {
                let mut perms = fs::metadata(path)?.permissions();
                let mode = perms.mode();
                // Add owner/group/other execute
                perms.set_mode(mode | 0o111);
                let _ = fs::set_permissions(path, perms);
            }
        }
    }

    log::info!(
        "plugin '{}' v{} installed to {}",
        plugin.name,
        plugin.version,
        dest_dir.display()
    );

    Ok(dest_dir)
}

// ── tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Helper: create a small zip file with a text file inside.
    fn create_test_zip(path: &Path, content: &str) {
        // We use the `zip` crate to build a test zip in-memory
        let file = fs::File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        zip.start_file("test.txt", options).unwrap();
        zip.write_all(content.as_bytes()).unwrap();
        zip.finish().unwrap();
    }

    #[test]
    fn test_verify_checksum_match() {
        let tmp = std::env::temp_dir().join("hermes_test_checksum.txt");
        fs::write(&tmp, b"hello world\n").unwrap();

        // SHA-256 of "hello world\n"
        let expected = "a948904f2f0f479b8f8197694b30184b0d2ed1c1cd2a1ec0fb85d299a192a447";
        assert!(verify_checksum(&tmp, expected).unwrap());
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_verify_checksum_mismatch() {
        let tmp = std::env::temp_dir().join("hermes_test_checksum2.txt");
        fs::write(&tmp, b"goodbye\n").unwrap();

        let expected = "a948904f2f0f479b8f8197694b30184b0d2ed1c1cd2a1ec0fb85d299a192a447";
        assert!(!verify_checksum(&tmp, expected).unwrap());
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_verify_checksum_empty_expected() {
        let tmp = std::env::temp_dir().join("hermes_test_checksum3.txt");
        fs::write(&tmp, b"anything").unwrap();

        // Empty checksum always passes
        assert!(verify_checksum(&tmp, "").unwrap());
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_extract_zip_roundtrip() {
        let tmp_dir = std::env::temp_dir().join("hermes_test_zip_extract");
        let _ = fs::remove_dir_all(&tmp_dir);

        let zip_path = tmp_dir.with_extension("zip");
        create_test_zip(&zip_path, "hello from zip\n");

        let extract_dir = tmp_dir.join("out");
        extract_zip(&zip_path, &extract_dir).unwrap();

        let extracted_file = extract_dir.join("test.txt");
        assert!(extracted_file.exists());
        let content = fs::read_to_string(&extracted_file).unwrap();
        assert_eq!(content, "hello from zip\n");

        let _ = fs::remove_file(&zip_path);
        let _ = fs::remove_dir_all(&tmp_dir);
    }

    #[test]
    fn test_extract_zip_nonexistent_file() {
        let result = extract_zip(
            Path::new("/nonexistent/path/archive.zip"),
            Path::new("/tmp/out"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_plugins_install_dir_exists() {
        let dir = index::plugins_dir().unwrap();
        assert!(dir.exists());
        assert!(dir.is_dir());
    }
}
