//! The files this process exported, so that revealing one in the system file
//! manager accepts only those (#915).
//!
//! An export command writes a file and hands the webview the path as text. The
//! reveal command gets that text back from the webview, which is the one place
//! an untrusted path enters, so the text is parsed there, once, into an
//! [`ExportedFile`]. That type has no public constructor: the only way to get
//! one is [`ExportedFiles::admit`], which accepts exactly a path an export
//! recorded in this process and refuses everything else, including another
//! spelling of the same file.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// How many exports the record keeps. The oldest is dropped first, so a file
/// exported more than this many exports ago can no longer be revealed: export
/// it again. One entry is a path's text, so the record stays small.
pub const MAX_RECORDED_EXPORTS: usize = 1_000;

/// The record, held as Tauri state for the life of the process. Each entry maps
/// the path text an export returned to the webview to the path it wrote.
#[derive(Default)]
pub struct ExportedFiles {
    written: Mutex<Recorded>,
}

/// The entries, and their texts from oldest to newest.
#[derive(Default)]
struct Recorded {
    paths: HashMap<String, PathBuf>,
    order: VecDeque<String>,
}

/// A file this process exported. Only [`ExportedFiles::admit`] makes one.
#[derive(Debug, PartialEq, Eq)]
pub struct ExportedFile(PathBuf);

/// The path text names no file this process exported.
#[derive(Debug, PartialEq, Eq)]
pub struct NotExported;

impl ExportedFiles {
    /// Records a file an export just wrote, and returns the path text to hand
    /// to the webview: the text later accepted by [`Self::admit`].
    pub fn record(&self, written: PathBuf) -> String {
        let text = written.to_string_lossy().into_owned();
        let mut recorded = self
            .written
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if recorded.paths.insert(text.clone(), written).is_some() {
            // Recorded again: it is now the newest.
            recorded.order.retain(|known| known != &text);
        }
        recorded.order.push_back(text.clone());
        while recorded.order.len() > MAX_RECORDED_EXPORTS {
            if let Some(oldest) = recorded.order.pop_front() {
                recorded.paths.remove(&oldest);
            }
        }
        text
    }

    /// Parses path text from the webview into an exported file: exactly the
    /// text an export returned, or a refusal.
    pub fn admit(&self, text: &str) -> Result<ExportedFile, NotExported> {
        self.written
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .paths
            .get(text)
            .cloned()
            .map(ExportedFile)
            .ok_or(NotExported)
    }
}

impl ExportedFile {
    /// The path the export wrote.
    pub fn path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_an_export_recorded_is_admitted_as_the_file_it_wrote() {
        let exported = ExportedFiles::default();
        let directory = tempfile::tempdir().unwrap();
        let written = directory
            .path()
            .join("statement-synthetic-party-20260331.pdf");
        let text = exported.record(written.clone());
        assert_eq!(exported.admit(&text).unwrap().path(), written);
    }

    #[test]
    fn a_path_no_export_recorded_is_refused_even_when_the_file_exists() {
        let exported = ExportedFiles::default();
        let directory = tempfile::tempdir().unwrap();
        let elsewhere = directory.path().join("not-an-export.txt");
        std::fs::write(&elsewhere, b"synthetic").unwrap();
        assert_eq!(
            exported.admit(&elsewhere.to_string_lossy()),
            Err(NotExported)
        );
    }

    #[test]
    fn another_spelling_of_an_exported_file_is_refused() {
        let exported = ExportedFiles::default();
        let directory = tempfile::tempdir().unwrap();
        let written = directory.path().join("report.csv");
        let text = exported.record(written.clone());
        let spelled_otherwise = directory.path().join(".").join("report.csv");
        assert_ne!(spelled_otherwise.to_string_lossy(), text);
        assert_eq!(
            exported.admit(&spelled_otherwise.to_string_lossy()),
            Err(NotExported)
        );
        assert_eq!(exported.admit(""), Err(NotExported));
    }

    #[test]
    fn the_record_keeps_the_newest_exports_and_drops_the_oldest() {
        let exported = ExportedFiles::default();
        let directory = tempfile::tempdir().unwrap();
        let path = |n: usize| directory.path().join(format!("export-{n}.csv"));
        let first = exported.record(path(0));
        let second = exported.record(path(1));
        // Recording the first again makes it the newest, so it outlives the second.
        assert_eq!(exported.record(path(0)), first);
        for n in 2..=MAX_RECORDED_EXPORTS {
            exported.record(path(n));
        }
        assert_eq!(exported.admit(&second), Err(NotExported));
        assert_eq!(exported.admit(&first).unwrap().path(), path(0));
        let newest = path(MAX_RECORDED_EXPORTS).to_string_lossy().into_owned();
        assert_eq!(
            exported.admit(&newest).unwrap().path(),
            path(MAX_RECORDED_EXPORTS)
        );
        let recorded = exported.written.lock().unwrap();
        assert_eq!(recorded.paths.len(), MAX_RECORDED_EXPORTS);
        assert_eq!(recorded.order.len(), MAX_RECORDED_EXPORTS);
    }
}
