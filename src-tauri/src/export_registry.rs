//! The files Bridge has written as exports, by the SHA-256 of their bytes, so
//! that the documents uploader leaves them out of a folder the user syncs
//! (bridge#833). An export is built from Tally data, and "Tally data is never
//! uploaded" must hold for a copy saved beside the user's own files too.
//!
//! Recognition is by content only: a renamed or moved export is still
//! recognised, and a copy the user has edited is theirs and uploads. Exports
//! written before a version with this registry are not in it.
//!
//! One line per export, the hash in upper-case hex (as the uploader hashes a
//! file), newest last, in the app-data directory. It holds at most
//! [`MAX_ENTRIES`] lines: when it is full, the oldest tenth is dropped as the
//! next export is recorded, so an export older than about that many later ones
//! would upload.
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs::{self, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

/// The most exports the registry remembers: about 1.3 MB of lines.
pub const MAX_ENTRIES: usize = 20_000;

const FILE_NAME: &str = "bridge-exports.sha256";
const LINE_BYTES: u64 = 65;

static REGISTRY: OnceLock<PathBuf> = OnceLock::new();
static WRITE: Mutex<()> = Mutex::new(());

/// Points the registry at `app_data_directory`, once per process. The desktop
/// app does this at start-up; a process that never does (the MCP server)
/// writes no exports and skips nothing.
pub fn init(app_data_directory: &Path) {
    let _ = REGISTRY.set(app_data_directory.join(FILE_NAME));
}

/// The hash the registry and the uploader compare: SHA-256, upper-case hex.
pub fn content_sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect()
}

/// Records `bytes` as an export Bridge is about to write. Called before the
/// file is written, so a file Bridge wrote is never missing from the registry:
/// if this fails, the caller must not write the export.
pub fn record(bytes: &[u8]) -> Result<(), String> {
    match REGISTRY.get() {
        Some(path) => record_in(path, &content_sha256(bytes)),
        None => Ok(()),
    }
}

/// The hashes of every export recorded, for one scan. Empty when there is no
/// registry yet; an unreadable one is an error, so a scan does not upload
/// exports it could not check.
pub fn recorded() -> Result<HashSet<String>, String> {
    match REGISTRY.get() {
        Some(path) => recorded_in(path),
        None => Ok(HashSet::new()),
    }
}

fn record_in(path: &Path, sha256: &str) -> Result<(), String> {
    let _guard = WRITE
        .lock()
        .map_err(|_| "export_registry_unavailable".to_string())?;
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory).map_err(|_| "export_registry_write_failed".to_string())?;
    }
    // A whole line is 65 bytes, so the size says when to prune without reading.
    let size = match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
        Err(_) => return Err("export_registry_unreadable".to_string()),
    };
    if size >= (MAX_ENTRIES as u64) * LINE_BYTES {
        // Down to nine tenths, so the next exports append again rather than
        // each rewriting the whole file.
        let mut entries = read_lines(path)?;
        entries.drain(..entries.len().saturating_sub(MAX_ENTRIES * 9 / 10 - 1));
        entries.push(sha256.to_string());
        // A staging name of its own and synced before the rename, so a power cut
        // never leaves a short registry and two instances never share a staging
        // file. A hash another instance appends during this prune can still be
        // lost: nothing makes the app single-instance (bridge#833).
        let staged = path.with_extension(format!("sha256.{}.next", uuid::Uuid::new_v4()));
        let written = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&staged)
            .and_then(|mut file| {
                file.write_all((entries.join("\n") + "\n").as_bytes())?;
                file.sync_all()
            })
            .and_then(|()| fs::rename(&staged, path));
        if written.is_err() {
            let _ = fs::remove_file(&staged);
        }
        written.map_err(|_| "export_registry_write_failed".to_string())
    } else {
        OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)
            .and_then(|mut file| {
                // A last line that does not end in a newline (torn, or left
                // zero-filled by a power cut) cannot swallow this one.
                let start = if ends_in_newline(&mut file, size)? {
                    ""
                } else {
                    "\n"
                };
                file.write_all(format!("{start}{sha256}\n").as_bytes())?;
                file.sync_data()
            })
            .map_err(|_| "export_registry_write_failed".to_string())
    }
}

/// Whether an empty file, or one whose last byte is a newline.
fn ends_in_newline(file: &mut fs::File, size: u64) -> std::io::Result<bool> {
    if size == 0 {
        return Ok(true);
    }
    let mut last = [0_u8];
    file.seek(SeekFrom::Start(size - 1))?;
    file.read_exact(&mut last)?;
    Ok(last[0] == b'\n')
}

fn recorded_in(path: &Path) -> Result<HashSet<String>, String> {
    Ok(read_lines(path)?.into_iter().collect())
}

/// The registry's hashes in order. A line that is not one is left out: the
/// file is Bridge's own, and a torn last line or a stray byte must not hide
/// the rest.
fn read_lines(path: &Path) -> Result<Vec<String>, String> {
    let text = match fs::read(path) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(_) => return Err("export_registry_unreadable".to_string()),
    };
    Ok(text
        .lines()
        .filter(|line| line.len() == 64 && line.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .map(str::to_ascii_uppercase)
        .collect())
}

/// Points the registry at one directory for the whole test process, the way
/// the app does at start-up. Every test that needs the registry calls this.
#[cfg(test)]
pub(crate) fn init_for_tests() {
    static DIRECTORY: OnceLock<tempfile::TempDir> = OnceLock::new();
    init(
        DIRECTORY
            .get_or_init(|| tempfile::tempdir().unwrap())
            .path(),
    );
}

#[cfg(test)]
#[path = "export_registry_tests.rs"]
mod tests;
