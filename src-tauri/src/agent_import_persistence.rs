//! Proof publication under the import admission lock, additive (#911), and
//! the build's recoverable publication. An interrupted build, or a journal
//! append whose outcome is unknown, keeps its transaction folder and blocks
//! admission until its state is reconciled.
use super::*;

const TRANSACTION: &str = ".proof-publication";
const BUILD_TRANSACTION: &str = ".build-publication";

fn create_transaction(path: &Path) -> std::io::Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(path)
}

pub(super) fn require_settled(imports: &Path) -> Result<(), String> {
    for (name, error_code) in [
        (TRANSACTION, "proof_publication_recovery_required"),
        (BUILD_TRANSACTION, "import_publication_recovery_required"),
    ] {
        match fs::symlink_metadata(imports.join(name)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            _ => return Err(error_code.into()),
        }
    }
    Ok(())
}

pub(super) fn persist_build(
    imports: &Path,
    line: &ImportLedgerLine,
    xml: &[u8],
    append_status: impl FnOnce() -> Result<(), String>,
) -> Result<Option<String>, String> {
    persist_build_with_stage(imports, line, xml, append_status, write_private)
}

fn persist_build_with_stage(
    imports: &Path,
    line: &ImportLedgerLine,
    xml: &[u8],
    append_status: impl FnOnce() -> Result<(), String>,
    stage_xml: impl FnOnce(&Path, &[u8]) -> Result<(), String>,
) -> Result<Option<String>, String> {
    let path = imports.join(format!("{}.xml", line.batch_id));
    let transaction = imports.join(BUILD_TRANSACTION);
    create_transaction(&transaction)
        .map_err(|_| "import_publication_recovery_required".to_string())?;
    let publication = (|| {
        write_private(
            &transaction.join("update.json"),
            &serde_json::to_vec_pretty(line)
                .map_err(|_| "import_ledger_serialization_failed".to_string())?,
        )?;
        // Every partial write and process interruption now leaves an admission
        // marker. Expose the importable name only after the staged XML is synced.
        let staged_xml = transaction.join("batch.xml");
        stage_xml(&staged_xml, xml)?;
        fs::rename(&staged_xml, &path).map_err(|_| "import_file_write_failed".to_string())
    })();
    if let Err(error) = publication {
        // Once our marker exists, the caller must return this batch's recovery
        // ID even if no complete XML or journal record could be published.
        return Ok(Some(error));
    }
    match append_status() {
        Err(error) if error == "import_ledger_rollback_failed" => Ok(Some(error)),
        Err(error) => {
            if fs::remove_file(&path).is_err() {
                return Ok(Some("import_publication_recovery_required".into()));
            }
            match fs::remove_dir_all(&transaction) {
                Ok(()) => Err(error),
                Err(_) => Ok(Some("import_publication_recovery_required".into())),
            }
        }
        Ok(()) => match fs::remove_dir_all(&transaction) {
            Ok(()) => Ok(None),
            Err(_) => Ok(Some("import_publication_recovery_required".into())),
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PublicationStep {
    WriteJson,
    WriteMarkdown,
    MarkAppend,
    AppendStatus,
}

/// Save a verification's proof pair beside every earlier one and name it
/// current in the journal (#911). Nothing that existed before is replaced or
/// removed. The pair is written under names no earlier pair can hold (its
/// stamp and the digest of its JSON), and the status record appended after it
/// is what makes it current, so a stop at any step before the append leaves
/// the earlier proof current and only files no record names. Only the append
/// is indeterminate: it runs inside `.proof-publication`, created here and
/// removed once the journal is known whole, so a stop or a failed truncate
/// during it blocks admission as before (`require_settled`).
pub(super) fn publish_proofs(
    imports: &Path,
    update: &ImportLedgerLine,
    json: &[u8],
    markdown: &[u8],
    append_status: impl FnOnce(&ledger::StatusRecord) -> Result<(), String>,
    mut before: impl FnMut(PublicationStep) -> Result<(), String>,
) -> Result<ledger::ProofName, String> {
    let name = ledger::ProofName::of(json, Utc::now());
    before(PublicationStep::WriteJson)?;
    write_new_file(&imports.join(name.json_file(&update.batch_id)), json)?;
    before(PublicationStep::WriteMarkdown)?;
    write_new_file(
        &imports.join(name.markdown_file(&update.batch_id)),
        markdown,
    )?;
    sync_directory(imports);
    let record = ledger::StatusRecord::verified(update, name.clone());
    before(PublicationStep::MarkAppend)?;
    let transaction = imports.join(TRANSACTION);
    create_transaction(&transaction)
        .map_err(|_| "proof_publication_recovery_required".to_string())?;
    let appended = (|| {
        // Enough context for a person to reconcile after process death.
        write_private(
            &transaction.join("update.json"),
            &serde_json::to_vec_pretty(&record)
                .map_err(|_| "proof_serialization_failed".to_string())?,
        )?;
        before(PublicationStep::AppendStatus)?;
        append_status(&record)
    })();
    match appended {
        // The journal may hold part of the record: keep the marker, which
        // blocks every later append until a person has looked.
        Err(error) if error == "import_ledger_rollback_failed" => {
            Err("proof_publication_rollback_failed".into())
        }
        Err(error) => {
            fs::remove_dir_all(&transaction)
                .map_err(|_| "proof_publication_recovery_required".to_string())?;
            Err(error)
        }
        Ok(()) => fs::remove_dir_all(&transaction)
            .map(|()| name)
            .map_err(|_| "proof_publication_recovery_required".to_string()),
    }
}

/// Write `bytes` to a name nothing holds yet, synced. A file of the same
/// bytes already there is the same proof saved twice in one millisecond and
/// is accepted as it is; any other file there is never touched.
fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    match crate::local_files::file::create_new_local_file(path) {
        Ok(mut file) => file
            .write_all(bytes)
            .and_then(|()| file.sync_data())
            .map_err(|_| "import_file_write_failed".to_string()),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => match fs::read(path) {
            Ok(existing) if existing == bytes => Ok(()),
            _ => Err("proof_publication_failed".into()),
        },
        Err(_) => Err("import_file_write_failed".into()),
    }
}

/// Why a record could not be placed by [`write_record_once`].
#[derive(Debug, PartialEq, Eq)]
pub(super) enum RecordOnce {
    /// The name already holds a record, which stays as it is.
    Exists,
    Failed,
}

/// Place `bytes` at `path` whole, only if nothing is there: staged under a
/// name of its own, synced, then renamed into place by a rename that fails
/// when the name exists ([`rename_new`]), so a stop at any point leaves the
/// record whole and alone or absent, never with a second link, which every
/// reader refuses (`open_local_file`). A stage that is not placed is removed.
///
/// [`rename_new`]: crate::local_files::file::rename_new
pub(super) fn write_record_once(path: &Path, bytes: &[u8]) -> Result<(), RecordOnce> {
    let staged = path.with_extension(format!("{}.next", Uuid::new_v4()));
    if write_private(&staged, bytes).is_err() {
        let _ = fs::remove_file(&staged);
        return Err(RecordOnce::Failed);
    }
    match crate::local_files::file::rename_new(&staged, path) {
        Ok(()) => {
            // Make the new name durable; a record lost to a power failure
            // reads as absent, never as someone else's.
            sync_directory(path.parent().unwrap_or(path));
            Ok(())
        }
        Err(error) => {
            let _ = fs::remove_file(&staged);
            Err(if error.kind() == std::io::ErrorKind::AlreadyExists {
                RecordOnce::Exists
            } else {
                RecordOnce::Failed
            })
        }
    }
}

fn sync_directory(directory: &Path) {
    #[cfg(unix)]
    {
        let _ = fs::File::open(directory).and_then(|directory| directory.sync_all());
    }
    #[cfg(not(unix))]
    let _ = directory;
}

#[cfg(test)]
#[path = "agent_import_persistence_tests.rs"]
mod tests;
