//! Resetting a Tally mirror that cannot be opened: deleting the database and its operating-system
//! key together so the user can start again (#1681).
//!
//! The mirror holds data Tally cannot give back (the hash-chained proof records, the history of
//! past observations, the reviewed company setup, the write-fixture attestations), and "no key
//! found" is not proof the key is gone: the credential-store account name is a hash of the
//! database path as a string, so a moved or renamed profile reads as "missing" while the key is
//! intact under the old name. So a reset is possible only through [`ResettableMirror`], a value
//! that can be built only by [`ResettableMirror::prove`], which
//!
//! - takes the initialization lock and the exclusive use lock (an open mirror holds the shared
//!   one for its whole life, in any process), and refuses while the mirror is open;
//! - lists every key the app has stored and tries each on the database, and refuses if any opens
//!   it, or if it cannot be told whether one does;
//! - carries the database path and the key store it proved, so a different path or store cannot be
//!   passed to the deletion.
//!
//! A store that cannot list its keys (today's operating-system store) cannot prove anything, so no
//! reset is offered for it. There is no command or screen yet.

// Nothing outside the tests calls this until the command that offers the reset exists.
#![cfg_attr(not(test), allow(dead_code))]

use std::ops::ControlFlow;
use std::path::{Path, PathBuf};

use zeroize::Zeroizing;

use super::encrypted::{
    connect_encrypted, lock_mirror_exclusively, lock_mirror_initialization, EncryptedOpenError,
    MirrorInitializationLock, MirrorKeyStore, MirrorUseLock, MirrorUseLockError, MIRROR_KEY_BYTES,
};

/// A key store a reset can be proved against and delete from.
pub trait ResettableKeyStore: MirrorKeyStore {
    /// Every key this app has stored for any mirror, under any account name: a key kept under the
    /// name another spelling of the path produced is found before a mirror is called keyless. A
    /// store that cannot list says so, and no reset is then offered.
    fn all_keys(&self) -> anyhow::Result<Vec<Zeroizing<Vec<u8>>>>;

    /// Removes the key stored for this database, and no other. Already absent is success.
    fn delete(&self) -> anyhow::Result<()>;
}

/// Which of the two permitted states the mirror is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResettableState {
    /// The database file exists and no stored key opens it.
    DatabaseNoKeyOpens,
    /// There is no database file, and the key stored for it is not a key of the length the mirror
    /// uses, which refuses every start.
    InvalidKeyNoDatabase,
}

#[derive(Debug, thiserror::Error)]
pub enum ResetRefusal {
    /// The mirror is open (here or in another process), or another reset has it.
    #[error("The Tally mirror is open or being reset, so it cannot be reset now")]
    InUse,
    #[error("The Tally mirror lock could not be taken")]
    LockUnavailable,
    /// Some stored key opens the database: a reset would destroy data that can be read.
    #[error("A stored key opens this Tally mirror, so it is not reset")]
    KeyOpensTheMirror,
    /// Neither permitted state: nothing is wrong that a reset would fix.
    #[error("The Tally mirror is not in a state a reset is offered for")]
    NotInAResettableState,
    #[error("Whether the Tally mirror exists could not be checked")]
    DatabaseStateUnreadable,
    /// The store cannot list every stored key, so a missing key cannot be told from one kept
    /// under another name.
    #[error("The stored Tally mirror keys could not be listed")]
    KeysUnlistable(#[source] anyhow::Error),
    /// A key could not be tried, or the database could not be opened for a reason that says
    /// nothing about the key (storage, a timeout, an integrity failure).
    #[error("Whether a stored key opens the Tally mirror could not be determined")]
    Undetermined,
}

/// One step of the deletion, in the order they run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetStep {
    /// The key stored for this database is gone and was read back as absent.
    KeyDeleted,
    WriteAheadLogRemoved,
    SharedMemoryRemoved,
    RollbackJournalRemoved,
    /// The database file itself, last: while it exists the reset is visibly unfinished and can be
    /// proved and run again.
    DatabaseRemoved,
}

#[derive(Debug, thiserror::Error)]
pub enum ResetFailure {
    #[error("The Tally mirror key could not be deleted; nothing else was touched")]
    KeyNotDeleted(#[source] anyhow::Error),
    /// The store reported success and still returns a key; no file was touched.
    #[error("The Tally mirror key is still readable after its deletion; nothing else was touched")]
    KeyStillReadable,
    #[error(
        "The Tally mirror key could not be checked after its deletion; nothing else was touched"
    )]
    KeyUnverifiable(#[source] anyhow::Error),
    #[error("A Tally mirror file could not be removed")]
    FileNotRemoved {
        step: ResetStep,
        #[source]
        source: std::io::Error,
    },
    /// The observer stopped the run after `0` (a crash point in a test).
    #[error("The reset was stopped after a step")]
    Interrupted(ResetStep),
}

/// A mirror proved to be in one of the two states a reset is offered for, still locked.
pub struct ResettableMirror<S: ResettableKeyStore> {
    database_path: PathBuf,
    key_store: S,
    state: ResettableState,
    // Held until the value is dropped: no mirror can open, and no second reset can start, between
    // the proof and the deletion. Dropped after the files are gone; the lock files stay.
    _use_lock: MirrorUseLock,
    _initialization_lock: MirrorInitializationLock,
}

impl<S: ResettableKeyStore> ResettableMirror<S> {
    pub async fn prove(database_path: &Path, key_store: S) -> Result<Self, ResetRefusal> {
        let initialization_lock =
            lock_mirror_initialization(database_path).map_err(|_| ResetRefusal::LockUnavailable)?;
        let use_lock = lock_mirror_exclusively(database_path).map_err(|error| match error {
            MirrorUseLockError::Held => ResetRefusal::InUse,
            MirrorUseLockError::Unavailable => ResetRefusal::LockUnavailable,
        })?;

        let exists = database_path
            .try_exists()
            .map_err(|_| ResetRefusal::DatabaseStateUnreadable)?;
        let state = if exists {
            let keys = key_store.all_keys().map_err(ResetRefusal::KeysUnlistable)?;
            for key in keys.into_iter().filter(|key| key.len() == MIRROR_KEY_BYTES) {
                match connect_encrypted(database_path, key).await {
                    Ok(pool) => {
                        pool.close().await;
                        return Err(ResetRefusal::KeyOpensTheMirror);
                    }
                    Err(error) => match error.downcast_ref::<EncryptedOpenError>() {
                        // SQLCipher says this key is not the one: the only answer that counts.
                        Some(EncryptedOpenError::UnreadableWithSuppliedKey { .. }) => {}
                        _ => return Err(ResetRefusal::Undetermined),
                    },
                }
            }
            ResettableState::DatabaseNoKeyOpens
        } else {
            // The listing is still required: a key under another account name does not make a
            // missing database resettable, but the store's inability to say is not "absent".
            key_store.all_keys().map_err(ResetRefusal::KeysUnlistable)?;
            match key_store.load().map_err(|_| ResetRefusal::Undetermined)? {
                Some(key) if key.len() != MIRROR_KEY_BYTES => ResettableState::InvalidKeyNoDatabase,
                _ => return Err(ResetRefusal::NotInAResettableState),
            }
        };
        Ok(Self {
            database_path: database_path.to_path_buf(),
            key_store,
            state,
            _use_lock: use_lock,
            _initialization_lock: initialization_lock,
        })
    }

    pub fn state(&self) -> ResettableState {
        self.state
    }

    /// Deletes the key, reads it back as gone, then the write-ahead files, then the database.
    /// The two lock files are not touched.
    pub fn reset(self) -> Result<(), ResetFailure> {
        self.reset_observed(|_| ControlFlow::Continue(()))
    }

    /// As [`Self::reset`], offering each finished step to `after`, which may stop the run there:
    /// the crash points the tests walk.
    pub(crate) fn reset_observed(
        self,
        mut after: impl FnMut(ResetStep) -> ControlFlow<()>,
    ) -> Result<(), ResetFailure> {
        self.key_store
            .delete()
            .map_err(ResetFailure::KeyNotDeleted)?;
        // The key goes first and is read back before any file goes, so no failure can leave a
        // readable key beside a database that is being deleted.
        match self.key_store.load() {
            Ok(None) => {}
            Ok(Some(_)) => return Err(ResetFailure::KeyStillReadable),
            Err(error) => return Err(ResetFailure::KeyUnverifiable(error)),
        }
        if after(ResetStep::KeyDeleted).is_break() {
            return Err(ResetFailure::Interrupted(ResetStep::KeyDeleted));
        }
        for (step, suffix) in [
            (ResetStep::WriteAheadLogRemoved, Some("-wal")),
            (ResetStep::SharedMemoryRemoved, Some("-shm")),
            (ResetStep::RollbackJournalRemoved, Some("-journal")),
            (ResetStep::DatabaseRemoved, None),
        ] {
            let path = match suffix {
                Some(suffix) => {
                    let mut name = self.database_path.as_os_str().to_os_string();
                    name.push(suffix);
                    PathBuf::from(name)
                }
                None => self.database_path.clone(),
            };
            match std::fs::remove_file(&path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(source) => return Err(ResetFailure::FileNotRemoved { step, source }),
            }
            if after(step).is_break() {
                return Err(ResetFailure::Interrupted(step));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "mirror_reset_tests.rs"]
mod tests;
