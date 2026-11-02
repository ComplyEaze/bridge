//! Proof publication under the import admission lock, additive (#911), and
//! the build's recoverable publication. An interrupted build, or a journal
//! append whose outcome is unknown, keeps its transaction folder and blocks
//! admission until its state is reconciled. An empty folder records no unknown
//! outcome: admission clears it (`require_settled`, #2269).
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

/// Refuse while a transaction's marker stands. The caller holds the admission
/// lock, shared or exclusive; every writer holds it exclusively from creating
/// the folder to removing it, so no publication is running here. A folder with
/// nothing in it is removed and refuses nothing: `update.json` goes only after
/// the journal is synced whole or the append is rolled back and the XML removed,
/// and before it exists nothing is published. `remove_dir` removes only an empty
/// folder, so a marker still holding anything keeps refusing (#2269).
pub(super) fn require_settled(imports: &Path) -> Result<(), String> {
    for (name, error_code) in [
        (TRANSACTION, "proof_publication_recovery_required"),
        (BUILD_TRANSACTION, "import_publication_recovery_required"),
    ] {
        let path = imports.join(name);
        match fs::symlink_metadata(&path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(metadata) if metadata.is_dir() => match fs::remove_dir(&path) {
                Ok(()) => {}
                // Another shared holder removed it first.
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    if error.kind() != std::io::ErrorKind::DirectoryNotEmpty {
                        eprintln!("{name} not cleared: {}", os_cause(&error));
                    }
                    return Err(error_code.into());
                }
            },
            _ => return Err(error_code.into()),
        }
    }
    Ok(())
}

/// An OS error by its number alone: the text of one can carry a path.
fn os_cause(error: &std::io::Error) -> String {
    match error.raw_os_error() {
        Some(code) => format!("os error {code}"),
        None => format!("{:?}", error.kind()),
    }
}

/// Remove a transaction folder once its outcome is known, trying a plain rmdir
/// if `remove` fails, with no wait: the caller holds the admission lock, and a
/// folder left empty is cleared at the next admission. Stderr keeps the first
/// OS error whatever follows; `Err` carries it when the folder stayed (#2269).
fn clear_transaction(
    path: &Path,
    remove: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<(), String> {
    let Err(error) = remove(path) else {
        return Ok(());
    };
    let cause = os_cause(&error);
    eprintln!("import transaction folder not removed: {cause}");
    match fs::remove_dir(path) {
        Ok(()) => Ok(()),
        Err(retry) if retry.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(cause),
    }
}

fn clear_transaction_folder(path: &Path) -> Result<(), String> {
    clear_transaction(path, |path| fs::remove_dir_all(path))
}

/// What a build's publication came to.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Persisted {
    /// The journal holds the batch. `folder_left` is why its transaction
    /// folder is still there, which does not undo the build.
    Built { folder_left: Option<String> },
    /// Not known published: the caller returns the batch's recovery ID.
    Incomplete(String),
}

pub(super) fn persist_build(
    imports: &Path,
    line: &ImportLedgerLine,
    xml: &[u8],
    append_status: impl FnOnce() -> Result<(), String>,
) -> Result<Persisted, String> {
    persist_build_with_stage(
        imports,
        line,
        xml,
        append_status,
        write_private,
        clear_transaction_folder,
    )
}

fn persist_build_with_stage(
    imports: &Path,
    line: &ImportLedgerLine,
    xml: &[u8],
    append_status: impl FnOnce() -> Result<(), String>,
    stage_xml: impl FnOnce(&Path, &[u8]) -> Result<(), String>,
    clear: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<Persisted, String> {
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
        return Ok(Persisted::Incomplete(error));
    }
    let recovery = || Persisted::Incomplete("import_publication_recovery_required".into());
    match append_status() {
        Err(error) if error == "import_ledger_rollback_failed" => Ok(Persisted::Incomplete(error)),
        Err(error) => {
            if fs::remove_file(&path).is_err() {
                return Ok(recovery());
            }
            match clear(&transaction) {
                Ok(()) => Err(error),
                Err(_) => Ok(recovery()),
            }
        }
        Ok(()) => Ok(Persisted::Built {
            folder_left: clear(&transaction).err(),
        }),
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
    before: impl FnMut(PublicationStep) -> Result<(), String>,
) -> Result<ledger::ProofName, String> {
    publish_proofs_syncing(
        imports,
        update,
        json,
        markdown,
        append_status,
        before,
        (sync_directory, clear_transaction_folder),
    )
}

/// [`publish_proofs`] with the directory sync and the folder's removal given,
/// so a test can fail either.
fn publish_proofs_syncing(
    imports: &Path,
    update: &ImportLedgerLine,
    json: &[u8],
    markdown: &[u8],
    append_status: impl FnOnce(&ledger::StatusRecord) -> Result<(), String>,
    mut before: impl FnMut(PublicationStep) -> Result<(), String>,
    (sync, clear): (
        impl FnOnce(&Path) -> std::io::Result<()>,
        impl FnOnce(&Path) -> Result<(), String>,
    ),
) -> Result<ledger::ProofName, String> {
    let name = ledger::ProofName::of(json, Utc::now());
    before(PublicationStep::WriteJson)?;
    write_new_file(&imports.join(name.json_file(&update.batch_id)), json)?;
    before(PublicationStep::WriteMarkdown)?;
    write_new_file(
        &imports.join(name.markdown_file(&update.batch_id)),
        markdown,
    )?;
    // The status record appended next names this pair: it must not be durable
    // before the pair is. A failure refuses, leaving files no record names.
    sync(imports).map_err(|_| "import_file_write_failed".to_string())?;
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
            clear(&transaction).map_err(|_| "proof_publication_recovery_required".to_string())?;
            Err(error)
        }
        // The record is whole and names this pair: a folder that stays does
        // not undo that, and `clear` has kept why (#2269).
        Ok(()) => {
            let _ = clear(&transaction);
            Ok(name)
        }
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
/// When the directory sync that makes the new name durable fails, the record is
/// in place but not known durable: the result is `Failed`, the file stays whole,
/// and a retry finds `Exists`.
///
/// [`rename_new`]: crate::local_files::file::rename_new
pub(super) fn write_record_once(path: &Path, bytes: &[u8]) -> Result<(), RecordOnce> {
    write_record_once_syncing(path, bytes, sync_directory)
}

/// [`write_record_once`] with the directory sync given, so a test can fail it.
fn write_record_once_syncing(
    path: &Path,
    bytes: &[u8],
    sync: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<(), RecordOnce> {
    let staged = path.with_extension(format!("{}.next", Uuid::new_v4()));
    if write_private(&staged, bytes).is_err() {
        let _ = fs::remove_file(&staged);
        return Err(RecordOnce::Failed);
    }
    match crate::local_files::file::rename_new(&staged, path) {
        Ok(()) => {
            // Make the new name durable; a record lost to a power failure
            // reads as absent, never as someone else's.
            sync(path.parent().unwrap_or(path)).map_err(|_| RecordOnce::Failed)
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

/// Make the entries of `directory` durable. Windows has no directory handle to
/// sync, so there this does nothing and succeeds.
pub(super) fn sync_directory(directory: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        fs::File::open(directory)?.sync_all()
    }
    #[cfg(not(unix))]
    {
        let _ = directory;
        Ok(())
    }
}

#[cfg(test)]
#[path = "agent_import_persistence_tests.rs"]
mod tests;
