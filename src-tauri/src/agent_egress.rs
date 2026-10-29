//! Locked append-only egress receipts and bounded tail reads.
//!
//! Each record is one JSON object on its own line, so every record ends `}\n`. A writer that stops
//! part way through one (a killed process) leaves bytes after the last record with no such end: a
//! torn tail. Appends hold an exclusive lock until the newline is synced, so a lock holder that
//! finds a torn tail knows no writer is mid-record. The next append repairs it by appending, never
//! by truncating: [`TORN_SENTINEL`] and a `torn_record` marker naming the torn bytes' offset,
//! length and sha256 (#2172). A reader drops bytes only when such a marker vouches for them
//! exactly, and never reads them as a record. The marker is a repair note, not proof against
//! tampering: anyone who can write the file can already hide a line in it. A data folder on a
//! network share, where the lock may not hold, is not supported.
#[cfg(any(unix, test))]
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use chrono::{SecondsFormat, Utc};
use serde::Deserialize;

use super::sha256_hex;

pub(super) const EGRESS_TAIL_CHUNK_BYTES: usize = 64 * 1024;
const MAX_EGRESS_TAIL_BYTES: usize = 256 * 1024;
/// How every record ends.
const RECORD_END: &[u8] = b"}\n";
/// What a repair writes before its marker: whichever of its bytes lands first leaves the torn
/// line invalid JSON, so a record cut just before its newline is never later read as a record.
const TORN_SENTINEL: &[u8] = b"#\n";
const TORN_RECORD_TYPE: &str = "torn_record";
/// The most bytes one marker vouches for: one record's bound, and room for the debris of earlier
/// repairs that were themselves cut short.
const MAX_TORN_BYTES: usize = MAX_EGRESS_TAIL_BYTES + 16 * 1024;

pub(super) fn canonical_company_guid(value: &str) -> Option<String> {
    uuid::Uuid::parse_str(value)
        .ok()
        .map(|guid| guid.hyphenated().to_string())
}

pub(super) fn append_egress_line(path: &Path, line: &str) -> Result<(), String> {
    append_egress_line_unless(path, line, 0, |_| false).map(|_| ())
}

/// As [`append_egress_line`], but first reads the newest `take` entries under the same exclusive
/// lock, and appends nothing when `recorded` finds the line already there. `Ok(true)` when the line
/// was appended. One lock covers the repair, the read and the append, so nothing lands between them.
pub(super) fn append_egress_line_unless(
    path: &Path,
    line: &str,
    take: usize,
    recorded: impl FnOnce(&EgressTail) -> bool,
) -> Result<bool, String> {
    // Leave room for the preceding newline in a bounded reverse tail read.
    if line.len().saturating_add(1) >= MAX_EGRESS_TAIL_BYTES {
        return Err("egress_record_too_large".to_string());
    }
    // Every record is one JSON object on one line, so it ends `}` and the torn tail is found.
    if !line.starts_with('{') || !line.ends_with('}') || line.contains('\n') {
        return Err("egress_record_write_failed".to_string());
    }
    let mut file = super::local_file::open_local_file(path, true)
        .map_err(|_| "egress_record_write_failed".to_string())?;
    // Read/write access permits LockFileEx and rollback truncation on Windows.
    // Every append seeks to EOF while holding this exclusive lock.
    file.lock()
        .map_err(|_| "egress_record_write_failed".to_string())?;
    #[cfg(unix)]
    let permission_result = {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(fs::Permissions::from_mode(0o600))
            .map_err(|_| "egress_record_write_failed".to_string())
    };
    #[cfg(not(unix))]
    let permission_result: Result<(), String> = Ok(());
    let write_result = permission_result.and_then(|()| {
        let write = |file: &mut File, bytes: &[u8]| {
            file.write_all(bytes)?;
            file.sync_data()
        };
        if take > 0 {
            repair_locked(&mut file, write)?;
            let tail = read_tail_locked(&mut file, take)?;
            if recorded(&tail) {
                return Ok(false);
            }
        }
        append_locked(&mut file, line, write).map(|()| true)
    });
    let unlock_result = file
        .unlock()
        .map_err(|_| "egress_record_write_failed".to_string());
    write_result.and_then(|appended| unlock_result.map(|()| appended))
}

/// Append `line` to a file this process holds the exclusive lock on, first repairing a torn tail.
fn append_locked(
    file: &mut File,
    line: &str,
    mut append_and_sync: impl FnMut(&mut File, &[u8]) -> std::io::Result<()>,
) -> Result<(), String> {
    repair_locked(file, &mut append_and_sync)?;
    append_bytes_locked(file, format!("{line}\n").as_bytes(), append_and_sync)
}

/// Append `bytes` at the end, or take them back if the write or its sync fails.
fn append_bytes_locked(
    file: &mut File,
    bytes: &[u8],
    append_and_sync: impl FnOnce(&mut File, &[u8]) -> std::io::Result<()>,
) -> Result<(), String> {
    let original_length = file
        .metadata()
        .map_err(|_| "egress_record_write_failed".to_string())?
        .len();
    file.seek(SeekFrom::Start(original_length))
        .map_err(|_| "egress_record_write_failed".to_string())?;
    if append_and_sync(file, bytes).is_err() {
        file.set_len(original_length)
            .and_then(|_| file.sync_data())
            .map_err(|_| "egress_record_rollback_failed".to_string())?;
        return Err("egress_record_write_failed".to_string());
    }
    Ok(())
}

/// What lies after the last record in `bytes`, the end of a file.
#[derive(Debug, PartialEq, Eq)]
enum TornTail {
    /// The bytes end with a record, or are empty.
    None,
    /// A torn run starts at this index of `bytes`.
    From(usize),
    /// Bytes after the last record that no writer of this file leaves: refused, never repaired.
    Unrecognised,
}

/// Find the torn run at the end of `bytes`, which start at the file's first byte when
/// `from_file_start`. A run is the torn record and any debris of cut repairs, whose every newline is
/// the sentinel's own; a complete line that is not a record is never part of one.
fn find_torn_tail(bytes: &[u8], from_file_start: bool) -> TornTail {
    let start = match bytes
        .windows(RECORD_END.len())
        .rposition(|end| end == RECORD_END)
    {
        Some(index) => index + RECORD_END.len(),
        None if from_file_start || bytes.is_empty() => 0,
        None => return TornTail::Unrecognised,
    };
    let run = &bytes[start..];
    if run.is_empty() {
        TornTail::None
    } else if run.len() <= MAX_TORN_BYTES && torn_shape(run) {
        TornTail::From(start)
    } else {
        TornTail::Unrecognised
    }
}

/// Every newline in a torn run follows the sentinel's `#`.
fn torn_shape(run: &[u8]) -> bool {
    run.iter()
        .enumerate()
        .all(|(index, byte)| *byte != b'\n' || (index > 0 && run[index - 1] == b'#'))
}

/// Read up to `count` bytes ending at the end of a file of `length` bytes.
fn read_end(file: &mut File, length: u64, count: usize) -> std::io::Result<Vec<u8>> {
    let count = count.min(usize::try_from(length).unwrap_or(usize::MAX));
    file.seek(SeekFrom::Start(length - count as u64))?;
    let mut bytes = vec![0_u8; count];
    file.read_exact(&mut bytes)?;
    Ok(bytes)
}

/// Repair a torn tail of a file this process holds the exclusive lock on: append the sentinel and
/// a marker vouching for the run and the sentinel together. Nothing is removed.
fn repair_locked(
    file: &mut File,
    append_and_sync: impl FnOnce(&mut File, &[u8]) -> std::io::Result<()>,
) -> Result<(), String> {
    let length = file
        .metadata()
        .map_err(|_| "egress_record_write_failed".to_string())?
        .len();
    let bytes = read_end(file, length, MAX_TORN_BYTES + RECORD_END.len())
        .map_err(|_| "egress_record_write_failed".to_string())?;
    let from_file_start = bytes.len() as u64 == length;
    let start = match find_torn_tail(&bytes, from_file_start) {
        TornTail::None => return Ok(()),
        TornTail::Unrecognised => return Err("egress_log_incomplete".to_string()),
        TornTail::From(start) => start,
    };
    let mut vouched = bytes[start..].to_vec();
    vouched.extend_from_slice(TORN_SENTINEL);
    let marker = serde_json::json!({
        "record_type": TORN_RECORD_TYPE,
        "ts": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        "torn_offset": length - (bytes.len() - start) as u64,
        "torn_bytes": vouched.len(),
        "torn_sha256": sha256_hex(&vouched),
    });
    let mut repair = TORN_SENTINEL.to_vec();
    repair.extend_from_slice(marker.to_string().as_bytes());
    repair.push(b'\n');
    append_bytes_locked(file, &repair, append_and_sync)
}

/// A `torn_record` marker as read back: the bytes it vouches for were left by a writer that
/// stopped mid-record, and are never a record.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(super) struct TornRecord {
    #[serde(rename = "torn_offset")]
    pub offset: u64,
    #[serde(rename = "torn_bytes")]
    pub bytes: u64,
    #[serde(rename = "torn_sha256")]
    pub sha256: String,
}

#[derive(Debug, Default)]
pub(super) struct EgressTail {
    /// Whole records, newest first. Never a torn line, and never a marker.
    pub records: Vec<String>,
    /// The markers among the same newest entries, newest first.
    pub torn: Vec<TornRecord>,
    /// Older receipts were omitted by the requested count or byte scan bound.
    pub truncated: bool,
    /// Bytes after the last record that no marker vouches for yet: a torn tail the next append
    /// repairs. Never read as a record.
    pub unrepaired_tail_bytes: Option<u64>,
}

/// One line of a log, by what it is.
enum Line<'a> {
    Record(&'a str),
    Marker(TornRecord),
    /// Not a record: dropped only when the marker after it vouches for it.
    Other,
}

fn classify(line: &[u8]) -> Line<'_> {
    #[derive(Deserialize)]
    struct Kind {
        record_type: Option<String>,
    }
    let Ok(text) = std::str::from_utf8(line) else {
        return Line::Other;
    };
    match serde_json::from_str::<Kind>(text) {
        Ok(Kind { record_type }) if record_type.as_deref() == Some(TORN_RECORD_TYPE) => {
            serde_json::from_str::<TornRecord>(text).map_or(Line::Other, Line::Marker)
        }
        Ok(_) => Line::Record(text),
        Err(_) => Line::Other,
    }
}

pub(super) fn read_egress_tail(path: &Path, take: usize) -> Result<EgressTail, String> {
    let mut file = match super::local_file::open_local_file(path, false) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(EgressTail::default())
        }
        Err(_) => return Err("egress_log_unreadable".to_string()),
    };
    file.lock_shared()
        .map_err(|_| "egress_log_unreadable".to_string())?;
    let tail = read_tail_locked(&mut file, take);
    let unlock = file
        .unlock()
        .map_err(|_| "egress_log_unreadable".to_string());
    tail.and_then(|tail| unlock.map(|()| tail))
}

/// The newest `take` entries of a file this process holds a lock on.
fn read_tail_locked(file: &mut File, take: usize) -> Result<EgressTail, String> {
    let file_len = file
        .metadata()
        .map_err(|_| "egress_log_unreadable".to_string())?
        .len();
    let mut position = file_len;
    let mut scanned = 0_usize;
    let mut newlines = 0_usize;
    let mut chunks = Vec::new();
    while position > 0 && scanned < MAX_EGRESS_TAIL_BYTES && newlines < take.saturating_add(1) {
        let chunk_size = EGRESS_TAIL_CHUNK_BYTES
            .min(MAX_EGRESS_TAIL_BYTES - scanned)
            .min(position as usize);
        position -= chunk_size as u64;
        file.seek(SeekFrom::Start(position))
            .map_err(|_| "egress_log_unreadable".to_string())?;
        let mut chunk = vec![0_u8; chunk_size];
        file.read_exact(&mut chunk)
            .map_err(|_| "egress_log_unreadable".to_string())?;
        newlines += chunk.iter().filter(|byte| **byte == b'\n').count();
        scanned += chunk_size;
        chunks.push(chunk);
    }
    chunks.reverse();
    let bytes = chunks.concat();
    // Appends commit newline-terminated rows while holding the same file lock.
    // A process interruption must not turn an unfinished receipt into evidence:
    // a torn tail is reported by size, never read.
    let (body_end, unrepaired_tail_bytes) = match find_torn_tail(&bytes, position == 0) {
        TornTail::None => (bytes.len(), None),
        TornTail::From(start) => (start, Some((bytes.len() - start) as u64)),
        TornTail::Unrecognised => return Err("egress_log_incomplete".to_string()),
    };
    // The discarded leading row can begin inside a UTF-8 code point.
    let body_start = if position > 0 {
        bytes[..body_end]
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(body_end, |index| index + 1)
    } else {
        0
    };
    let mut entries = Vec::new();
    // Where the current run of lines that are not records began, in the file.
    let mut pending: Option<u64> = None;
    let mut cursor = body_start;
    while cursor < body_end {
        let end = cursor
            + bytes[cursor..body_end]
                .iter()
                .position(|byte| *byte == b'\n')
                .unwrap_or(body_end - cursor);
        let at = position + cursor as u64;
        match classify(&bytes[cursor..end]) {
            Line::Record(text) => {
                if pending.is_some() {
                    return Err("egress_log_incomplete".to_string());
                }
                entries.push(Ok(text.to_string()));
            }
            Line::Marker(torn) => {
                let first_line = position + body_start as u64;
                vouched(file, &torn, at, pending, first_line)?;
                pending = None;
                entries.push(Err(torn));
            }
            Line::Other => {
                pending.get_or_insert(at);
            }
        }
        cursor = end + 1;
    }
    if pending.is_some() {
        return Err("egress_log_incomplete".to_string());
    }
    let mut newest = entries.into_iter().rev();
    let mut tail = EgressTail {
        unrepaired_tail_bytes,
        ..EgressTail::default()
    };
    for entry in newest.by_ref().take(take) {
        match entry {
            Ok(record) => tail.records.push(record),
            Err(torn) => tail.torn.push(torn),
        }
    }
    tail.truncated = position > 0 || newest.next().is_some();
    Ok(tail)
}

/// Whether `torn`, the marker on the line at `at`, vouches exactly for the lines before it that
/// are not records: its range must end at the marker, start where that run starts (or, for a run
/// that begins at the first whole line read, no later), follow a record end, hold only a torn
/// run, and hash to its sha256. A range is never dropped unchecked.
fn vouched(
    file: &mut File,
    torn: &TornRecord,
    at: u64,
    pending: Option<u64>,
    first_line: u64,
) -> Result<(), String> {
    let refused = || "egress_log_incomplete".to_string();
    let length = usize::try_from(torn.bytes).map_err(|_| refused())?;
    if length == 0 || length > MAX_TORN_BYTES || torn.offset.checked_add(torn.bytes) != Some(at) {
        return Err(refused());
    }
    let run_start = pending.unwrap_or(at);
    let starts_well =
        torn.offset == run_start || (run_start == first_line && torn.offset < run_start);
    if !starts_well {
        return Err(refused());
    }
    let before = if torn.offset >= RECORD_END.len() as u64 {
        RECORD_END.len()
    } else {
        0
    };
    file.seek(SeekFrom::Start(torn.offset - before as u64))
        .map_err(|_| "egress_log_unreadable".to_string())?;
    let mut bytes = vec![0_u8; before + length];
    file.read_exact(&mut bytes)
        .map_err(|_| "egress_log_unreadable".to_string())?;
    let (previous, range) = bytes.split_at(before);
    let follows_a_record = torn.offset == 0 || previous == RECORD_END;
    if follows_a_record
        && range.ends_with(TORN_SENTINEL)
        && torn_shape(range)
        && sha256_hex(range) == torn.sha256
    {
        Ok(())
    } else {
        Err(refused())
    }
}

#[cfg(test)]
#[path = "agent_egress_tests.rs"]
mod tests;
