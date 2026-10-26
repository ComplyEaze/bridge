use super::{
    ControlFlow, ResetFailure, ResetRefusal, ResetStep, ResettableKeyStore, ResettableMirror,
    ResettableState,
};
use crate::db::encrypted::{
    connect_encrypted, lock_for_open, lock_mirror_in_use, lock_mirror_initialization,
    resolve_mirror_key, MirrorKeyStore, MIRROR_KEY_BYTES,
};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;

const GOOD_KEY: [u8; MIRROR_KEY_BYTES] = [7; MIRROR_KEY_BYTES];
const OTHER_KEY: [u8; MIRROR_KEY_BYTES] = [9; MIRROR_KEY_BYTES];

/// One credential-store entry: the account name and the key under it.
type StoredAccount = (String, Vec<u8>);

/// A credential store with several accounts, of which `own` is the one this database is looked
/// up under. Clones share one state, so a test can read it after the store has been consumed.
#[derive(Clone, Default)]
struct FakeStore {
    accounts: Arc<Mutex<Vec<StoredAccount>>>,
    own: String,
    listing_fails: bool,
    load_fails: bool,
    delete_fails: bool,
    /// A store that reports a deletion and keeps the key.
    delete_is_a_lie: bool,
}

impl FakeStore {
    fn new(own: &str) -> Self {
        Self {
            own: own.to_string(),
            ..Self::default()
        }
    }

    fn with(self, account: &str, key: &[u8]) -> Self {
        self.accounts
            .lock()
            .unwrap()
            .push((account.to_string(), key.to_vec()));
        self
    }

    fn accounts(&self) -> Vec<String> {
        self.accounts
            .lock()
            .unwrap()
            .iter()
            .map(|(account, _)| account.clone())
            .collect()
    }

    fn own_key(&self) -> Option<Vec<u8>> {
        self.accounts
            .lock()
            .unwrap()
            .iter()
            .find(|(account, _)| *account == self.own)
            .map(|(_, key)| key.clone())
    }
}

impl MirrorKeyStore for FakeStore {
    fn load(&self) -> anyhow::Result<Option<Zeroizing<Vec<u8>>>> {
        if self.load_fails {
            anyhow::bail!("the credential store could not be read");
        }
        Ok(self.own_key().map(Zeroizing::new))
    }

    fn save(&self, key: &[u8]) -> anyhow::Result<()> {
        let mut accounts = self.accounts.lock().unwrap();
        accounts.retain(|(account, _)| *account != self.own);
        accounts.push((self.own.clone(), key.to_vec()));
        Ok(())
    }
}

impl ResettableKeyStore for FakeStore {
    fn all_keys(&self) -> anyhow::Result<Vec<Zeroizing<Vec<u8>>>> {
        if self.listing_fails {
            anyhow::bail!("the credential store cannot list its accounts");
        }
        Ok(self
            .accounts
            .lock()
            .unwrap()
            .iter()
            .map(|(_, key)| Zeroizing::new(key.clone()))
            .collect())
    }

    fn delete(&self) -> anyhow::Result<()> {
        if self.delete_fails {
            anyhow::bail!("the credential store refused the deletion");
        }
        if !self.delete_is_a_lie {
            self.accounts
                .lock()
                .unwrap()
                .retain(|(account, _)| *account != self.own);
        }
        Ok(())
    }
}

fn paths(directory: &Path) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let database = directory.join("tally-mirror-v1.db");
    let side = |suffix: &str| {
        let mut name = database.as_os_str().to_os_string();
        name.push(suffix);
        PathBuf::from(name)
    };
    (
        database.clone(),
        side("-wal"),
        side("-shm"),
        side("-journal"),
    )
}

/// A real encrypted mirror under `key`, closed again.
async fn encrypted_mirror(database: &Path, key: &[u8]) {
    let pool = connect_encrypted(database, Zeroizing::new(key.to_vec()))
        .await
        .expect("open the synthetic mirror");
    sqlx::query("CREATE TABLE marker(value TEXT); INSERT INTO marker VALUES ('kept');")
        .execute(&pool)
        .await
        .expect("write the synthetic mirror");
    pool.close().await;
}

async fn prove(
    database: &Path,
    store: &FakeStore,
) -> Result<ResettableMirror<FakeStore>, ResetRefusal> {
    ResettableMirror::prove(database, store.clone()).await
}

#[tokio::test]
async fn a_database_no_stored_key_opens_is_reset_key_first_and_the_lock_files_stay() {
    let directory = tempfile::tempdir().unwrap();
    let (database, wal, shm, journal) = paths(directory.path());
    encrypted_mirror(&database, &GOOD_KEY).await;
    // The write-ahead files a crashed process leaves, and another mirror's key beside this one's.
    for leftover in [&wal, &shm, &journal] {
        std::fs::write(leftover, b"leftover").unwrap();
    }
    // This database's key is absent; the only keys held are other mirrors' (so none opens it).
    let store = FakeStore::new("this-path")
        .with("another-path", &OTHER_KEY)
        .with("a-third-path", &[1; 5]);

    let proof = prove(&database, &store).await.expect("a resettable mirror");
    assert_eq!(proof.state(), ResettableState::DatabaseNoKeyOpens);
    proof.reset().expect("the reset");

    for removed in [&database, &wal, &shm, &journal] {
        assert!(!removed.exists(), "{}", removed.display());
    }
    // Lock files are not deleted, and no other mirror's key is touched.
    assert!(directory
        .path()
        .join("tally-mirror-v1.db.use.lock")
        .exists());
    assert!(directory
        .path()
        .join("tally-mirror-v1.db.init.lock")
        .exists());
    assert_eq!(store.accounts(), ["another-path", "a-third-path"]);

    // The user can start again: a new key and a new, empty mirror.
    let resolved = resolve_mirror_key(&database, &store).expect("a fresh key");
    let pool = connect_encrypted(&database, resolved.key)
        .await
        .expect("a fresh mirror");
    let tables: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sqlite_master")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(tables, 0);
    pool.close().await;
}

#[tokio::test]
async fn a_key_kept_under_another_account_name_opens_the_mirror_and_nothing_is_deleted() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    encrypted_mirror(&database, &GOOD_KEY).await;
    // A restored or moved profile: this path's account is empty, the old spelling's is intact.
    let store = FakeStore::new("new-path-spelling").with("old-path-spelling", &GOOD_KEY);

    let refusal = prove(&database, &store).await.err().expect("a refusal");
    assert!(
        matches!(refusal, ResetRefusal::KeyOpensTheMirror),
        "{refusal:?}"
    );
    assert!(database.exists());
    assert_eq!(store.accounts(), ["old-path-spelling"]);
}

#[tokio::test]
async fn a_key_under_this_accounts_own_name_that_opens_the_mirror_is_not_reset() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    encrypted_mirror(&database, &GOOD_KEY).await;
    let store = FakeStore::new("this-path").with("this-path", &GOOD_KEY);
    let refusal = prove(&database, &store).await.err().expect("a refusal");
    assert!(
        matches!(refusal, ResetRefusal::KeyOpensTheMirror),
        "{refusal:?}"
    );
}

#[tokio::test]
async fn an_invalid_key_with_no_database_is_the_other_resettable_state() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    let store = FakeStore::new("this-path")
        .with("this-path", &[3; 7])
        .with("another-path", &OTHER_KEY);

    let proof = prove(&database, &store).await.expect("a resettable mirror");
    assert_eq!(proof.state(), ResettableState::InvalidKeyNoDatabase);
    proof.reset().expect("the reset");
    assert_eq!(store.accounts(), ["another-path"]);
    resolve_mirror_key(&database, &store).expect("a fresh start after the reset");
}

#[tokio::test]
async fn states_a_reset_fixes_nothing_in_are_not_offered_one() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    // First run: no database, no key.
    let first_run = FakeStore::new("this-path");
    let refusal = prove(&database, &first_run).await.err().expect("a refusal");
    assert!(
        matches!(refusal, ResetRefusal::NotInAResettableState),
        "{refusal:?}"
    );
    // No database and a valid key: starts normally.
    let valid = FakeStore::new("this-path").with("this-path", &GOOD_KEY);
    let refusal = prove(&database, &valid).await.err().expect("a refusal");
    assert!(
        matches!(refusal, ResetRefusal::NotInAResettableState),
        "{refusal:?}"
    );
    assert_eq!(valid.accounts(), ["this-path"]);
}

#[tokio::test]
async fn a_store_that_cannot_list_its_keys_proves_nothing_in_either_state() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    let mut unlistable = FakeStore::new("this-path").with("this-path", &[3; 7]);
    unlistable.listing_fails = true;
    // No database and an invalid key.
    let refusal = prove(&database, &unlistable)
        .await
        .err()
        .expect("a refusal");
    assert!(
        matches!(refusal, ResetRefusal::KeysUnlistable(_)),
        "{refusal:?}"
    );
    // A database nobody can say a key opens.
    encrypted_mirror(&database, &GOOD_KEY).await;
    let refusal = prove(&database, &unlistable)
        .await
        .err()
        .expect("a refusal");
    assert!(
        matches!(refusal, ResetRefusal::KeysUnlistable(_)),
        "{refusal:?}"
    );
    assert!(database.exists());
    assert_eq!(unlistable.accounts(), ["this-path"]);
}

#[tokio::test]
async fn a_key_that_cannot_be_read_is_not_an_invalid_one() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    let mut locked = FakeStore::new("this-path").with("this-path", &[3; 7]);
    locked.load_fails = true;
    let refusal = prove(&database, &locked).await.err().expect("a refusal");
    assert!(matches!(refusal, ResetRefusal::Undetermined), "{refusal:?}");
}

#[tokio::test]
async fn a_storage_failure_while_trying_a_key_is_not_a_wrong_key() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    // A path that exists and is not a database file cannot be opened for a reason that says
    // nothing about any key.
    std::fs::create_dir(&database).unwrap();
    let store = FakeStore::new("this-path").with("another-path", &OTHER_KEY);
    let refusal = prove(&database, &store).await.err().expect("a refusal");
    assert!(matches!(refusal, ResetRefusal::Undetermined), "{refusal:?}");
    assert!(database.exists());
}

#[tokio::test]
async fn an_open_mirror_refuses_a_reset_and_a_reset_refuses_an_open_at_once() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    encrypted_mirror(&database, &GOOD_KEY).await;
    let store = FakeStore::new("this-path").with("another-path", &OTHER_KEY);

    // This process has the mirror open: the opener's locks (the shared use lock is kept) exclude
    // the reset.
    let (initialization, open) = lock_for_open(&database).expect("the opener's locks");
    drop(initialization);
    let refusal = prove(&database, &store).await.err().expect("a refusal");
    assert!(matches!(refusal, ResetRefusal::InUse), "{refusal:?}");
    assert!(database.exists());
    // Several opens can share it.
    let second = lock_mirror_in_use(&database).expect("a second open");
    drop((open, second));

    // An opener still initializing refuses the reset too, without the reset waiting for it.
    let initializing = lock_mirror_initialization(&database).expect("an opener initializing");
    let refusal = prove(&database, &store).await.err().expect("a refusal");
    assert!(matches!(refusal, ResetRefusal::InUse), "{refusal:?}");
    drop(initializing);

    // While a proof holds the mirror, a real opener is refused at once: it must neither wait on
    // the proof nor get past it.
    let proof = prove(&database, &store).await.expect("a resettable mirror");
    let opener_path = database.clone();
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = sender.send(lock_for_open(&opener_path).is_err());
    });
    let refused = receiver
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("an opener must not wait on a held proof");
    assert!(refused);
    drop(proof);
    drop(lock_for_open(&database).expect("available again once the proof is dropped"));
}

#[tokio::test]
async fn a_plaintext_database_is_not_reset() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    let mut bytes = b"SQLite format 3\0".to_vec();
    bytes.extend_from_slice(&[0; 100]);
    std::fs::write(&database, bytes).unwrap();
    let store = FakeStore::new("this-path").with("another-path", &OTHER_KEY);
    let refusal = prove(&database, &store).await.err().expect("a refusal");
    assert!(matches!(refusal, ResetRefusal::NotEncrypted), "{refusal:?}");
    assert!(database.exists());
}

#[tokio::test]
async fn a_right_length_key_of_this_databases_own_that_does_not_open_it_is_never_discarded() {
    let directory = tempfile::tempdir().unwrap();
    let (database, ..) = paths(directory.path());
    encrypted_mirror(&database, &GOOD_KEY).await;
    // This path's own entry holds a right-length key that does not open the database: a wrong
    // key, or the right key and a damaged first page, which SQLCipher answers the same way.
    let store = FakeStore::new("this-path").with("this-path", &OTHER_KEY);
    let refusal = prove(&database, &store).await.err().expect("a refusal");
    assert!(
        matches!(refusal, ResetRefusal::StoredKeyDoesNotOpen),
        "{refusal:?}"
    );
    assert!(database.exists());
    assert_eq!(store.accounts(), ["this-path"]);

    // The same holds for a store that belongs to another mirror: its key is never deleted on
    // the strength of this database failing to open.
    let other_mirrors_store = FakeStore::new("mirror-b").with(
        "mirror-b",
        &GOOD_KEY[..].iter().map(|b| b ^ 1).collect::<Vec<u8>>(),
    );
    let refusal = prove(&database, &other_mirrors_store)
        .await
        .err()
        .expect("a refusal");
    assert!(
        matches!(refusal, ResetRefusal::StoredKeyDoesNotOpen),
        "{refusal:?}"
    );
    assert_eq!(other_mirrors_store.accounts(), ["mirror-b"]);
}

#[tokio::test]
async fn every_crash_point_leaves_no_readable_key_beside_the_database_and_a_rerun_finishes() {
    let steps = [
        ResetStep::KeyDeleted,
        ResetStep::WriteAheadLogRemoved,
        ResetStep::SharedMemoryRemoved,
        ResetStep::RollbackJournalRemoved,
        ResetStep::DatabaseRemoved,
    ];
    for stop_after in steps {
        let directory = tempfile::tempdir().unwrap();
        let (database, wal, shm, journal) = paths(directory.path());
        encrypted_mirror(&database, &GOOD_KEY).await;
        for leftover in [&wal, &shm, &journal] {
            std::fs::write(leftover, b"leftover").unwrap();
        }
        let store = FakeStore::new("this-path").with("another-path", &OTHER_KEY);
        // A key of the wrong length under this path's own account: the key is invalid, so the
        // reset may remove it.
        store.save(&[4; 7]).unwrap();

        let proof = prove(&database, &store).await.expect("a resettable mirror");
        let error = proof
            .reset_observed(|step| {
                // At every step the key is already unreadable, whatever files remain.
                assert!(store.own_key().is_none(), "key readable after {step:?}");
                if step == stop_after {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            })
            .expect_err("the run is stopped");
        assert!(matches!(error, ResetFailure::Interrupted(step) if step == stop_after));
        assert_eq!(store.accounts(), ["another-path"]);

        if stop_after == ResetStep::DatabaseRemoved {
            // Everything is gone: nothing further to reset, and a fresh start works.
            assert!(!database.exists());
            let refusal = prove(&database, &store).await.err().expect("a refusal");
            assert!(
                matches!(refusal, ResetRefusal::NotInAResettableState),
                "{refusal:?}"
            );
        } else {
            // The database file is removed last, so the reset is visibly unfinished.
            assert!(database.exists(), "{stop_after:?}");
            let proof = prove(&database, &store).await.expect("provable again");
            assert_eq!(proof.state(), ResettableState::DatabaseNoKeyOpens);
            proof.reset().expect("a rerun finishes the reset");
            for removed in [&database, &wal, &shm, &journal] {
                assert!(!removed.exists());
            }
        }
        resolve_mirror_key(&database, &store).expect("a fresh start");
    }
}

#[tokio::test]
async fn a_key_that_cannot_be_deleted_or_is_still_readable_leaves_every_file_alone() {
    for (store, expect_not_deleted) in [
        (
            {
                let mut store = FakeStore::new("this-path").with("another-path", &OTHER_KEY);
                store.delete_fails = true;
                store
            },
            true,
        ),
        (
            {
                let mut store = FakeStore::new("this-path").with("another-path", &OTHER_KEY);
                store.delete_is_a_lie = true;
                store.save(&[4; 7]).unwrap();
                store
            },
            false,
        ),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let (database, ..) = paths(directory.path());
        encrypted_mirror(&database, &GOOD_KEY).await;
        let error = prove(&database, &store)
            .await
            .expect("a resettable mirror")
            .reset()
            .expect_err("the key stays");
        if expect_not_deleted {
            assert!(matches!(error, ResetFailure::KeyNotDeleted(_)), "{error:?}");
        } else {
            assert!(matches!(error, ResetFailure::KeyStillReadable), "{error:?}");
        }
        assert!(database.exists());
    }
}

#[tokio::test]
async fn a_file_that_cannot_be_removed_stops_the_reset_at_that_step() {
    let directory = tempfile::tempdir().unwrap();
    let (database, wal, ..) = paths(directory.path());
    encrypted_mirror(&database, &GOOD_KEY).await;
    let store = FakeStore::new("this-path").with("another-path", &OTHER_KEY);
    let proof = prove(&database, &store).await.expect("a resettable mirror");
    // After the proof, a directory where the write-ahead log should be: remove_file refuses it.
    std::fs::create_dir(&wal).unwrap();
    let error = proof.reset().expect_err("the log cannot be removed");
    assert!(
        matches!(
            error,
            ResetFailure::FileNotRemoved {
                step: ResetStep::WriteAheadLogRemoved,
                ..
            }
        ),
        "{error:?}"
    );
    assert!(database.exists());
}
