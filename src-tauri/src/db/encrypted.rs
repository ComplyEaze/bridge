use sha2::{Digest, Sha256};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Connection, Executor, SqliteConnection, SqlitePool};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use zeroize::Zeroizing;

pub(crate) const MIRROR_KEY_BYTES: usize = 32;
const KEYRING_SERVICE: &str = "com.complyeaze.bridge.tally-mirror";
const KEYRING_ACCOUNT_PREFIX: &str = "database-key-v1";

#[derive(Debug, thiserror::Error)]
pub enum EncryptedOpenError {
    #[error("The encrypted Tally mirror rejected the supplied key or its encrypted contents")]
    UnreadableWithSuppliedKey {
        #[source]
        source: sqlx::Error,
    },
    #[error("The encrypted Tally mirror failed its integrity check")]
    IntegrityFailed,
    #[error("SQLCipher is unavailable for the encrypted Tally mirror")]
    CipherUnavailable,
    #[error("The encrypted Tally mirror security settings were not applied")]
    SecuritySettings,
    #[error("The encrypted Tally mirror storage could not be opened")]
    Storage {
        #[source]
        source: sqlx::Error,
    },
    #[error("The encrypted Tally mirror connection pool timed out")]
    PoolTimedOut,
}

/// Why the mirror's key could not be resolved. The two key states are permanent
/// until the credential is put back, so a retry cannot fix
/// them; every other failure is the credential store's or the file system's own.
#[derive(Debug, thiserror::Error)]
pub enum MirrorKeyError {
    /// The database exists and the credential store holds no key for it.
    #[error("The encrypted Tally mirror exists but its operating-system key is missing")]
    MissingForExistingMirror,
    /// The stored key is not a key of the length the mirror uses.
    #[error("The stored Tally mirror key is invalid")]
    StoredKeyInvalid,
    /// Whether the database exists could not be read, so no key is generated:
    /// a key made beside a database that does exist could never open it.
    #[error("The Tally mirror files could not be checked, so no key was created")]
    DatabaseStateUnreadable,
    /// The credential store or the random generator failed.
    #[error(transparent)]
    Store(anyhow::Error),
}

pub trait MirrorKeyStore: Send + Sync {
    fn load(&self) -> anyhow::Result<Option<Zeroizing<Vec<u8>>>>;
    fn save(&self, key: &[u8]) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct OsMirrorKeyStore {
    account: String,
}

impl OsMirrorKeyStore {
    pub fn for_database(database_path: &Path) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(database_path.as_os_str().to_string_lossy().as_bytes());
        let path_id = hex_key(&hasher.finalize());
        Self {
            account: format!("{KEYRING_ACCOUNT_PREFIX}:{path_id}"),
        }
    }

    fn entry(&self) -> anyhow::Result<keyring::Entry> {
        keyring::Entry::new(KEYRING_SERVICE, &self.account)
            .map_err(|_| anyhow::anyhow!("The operating-system credential store is unavailable"))
    }
}

impl MirrorKeyStore for OsMirrorKeyStore {
    fn load(&self) -> anyhow::Result<Option<Zeroizing<Vec<u8>>>> {
        match self.entry()?.get_secret() {
            Ok(secret) => Ok(Some(Zeroizing::new(secret))),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => anyhow::bail!(
                "The Tally mirror key could not be read from the operating-system credential store"
            ),
        }
    }

    fn save(&self, key: &[u8]) -> anyhow::Result<()> {
        self.entry()?
            .set_secret(key)
            .map_err(|_| anyhow::anyhow!("The Tally mirror key could not be stored securely"))
    }
}

pub struct MirrorInitializationLock {
    _file: File,
}

fn mirror_initialization_lock_file(database_path: &Path) -> anyhow::Result<File> {
    let file_name = database_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("The Tally mirror path has no file name"))?;
    let mut lock_name = file_name.to_os_string();
    lock_name.push(".init.lock");
    let lock_path = database_path.with_file_name(lock_name);
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)
        .map_err(|_| anyhow::anyhow!("The Tally mirror initialization lock could not be opened"))
}

pub fn lock_mirror_initialization(
    database_path: &Path,
) -> anyhow::Result<MirrorInitializationLock> {
    let file = mirror_initialization_lock_file(database_path)?;
    file.lock()
        .map_err(|_| anyhow::anyhow!("The Tally mirror is being initialized by another process"))?;
    Ok(MirrorInitializationLock { _file: file })
}

/// The initialization lock, refusing at once instead of waiting: a reset must not queue behind
/// an opener (or hold one up) while it decides.
pub(crate) fn try_lock_mirror_initialization(
    database_path: &Path,
) -> Result<MirrorInitializationLock, MirrorUseLockError> {
    let file = mirror_initialization_lock_file(database_path)
        .map_err(|_| MirrorUseLockError::Unavailable)?;
    match file.try_lock() {
        Ok(()) => Ok(MirrorInitializationLock { _file: file }),
        Err(std::fs::TryLockError::WouldBlock) => Err(MirrorUseLockError::Held),
        Err(std::fs::TryLockError::Error(_)) => Err(MirrorUseLockError::Unavailable),
    }
}

/// The locks an opener of the mirror takes, in this order: the initialization lock (waits for
/// another opener to finish), then the shared use lock, which refuses at once while a reset holds
/// the mirror. The use lock is kept for as long as the mirror stays open.
pub fn lock_for_open(
    database_path: &Path,
) -> anyhow::Result<(MirrorInitializationLock, MirrorUseLock)> {
    let initialization = lock_mirror_initialization(database_path)?;
    let in_use = lock_mirror_in_use(database_path)?;
    Ok((initialization, in_use))
}

/// Held by an open mirror for its whole life, and taken exclusively by a reset, so a reset cannot
/// run while any handle in any process (this one included: each lock is on its own open file)
/// has the mirror open. It is a different file from the initialization lock, which is held only
/// while a mirror is being opened.
#[derive(Debug)]
pub struct MirrorUseLock {
    _file: File,
}

/// Why the use lock could not be taken.
#[derive(Debug, thiserror::Error)]
pub enum MirrorUseLockError {
    /// Another handle holds it in the way that excludes this one: the mirror is open elsewhere
    /// (for an exclusive request), or a reset is running (for a shared one).
    #[error("The Tally mirror is in use or being reset")]
    Held,
    #[error("The Tally mirror use lock could not be opened")]
    Unavailable,
}

fn mirror_use_lock_file(database_path: &Path) -> Result<File, MirrorUseLockError> {
    let file_name = database_path
        .file_name()
        .ok_or(MirrorUseLockError::Unavailable)?;
    let mut lock_name = file_name.to_os_string();
    lock_name.push(".use.lock");
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(database_path.with_file_name(lock_name))
        .map_err(|_| MirrorUseLockError::Unavailable)
}

/// The shared lock an open mirror holds. Refuses at once, without waiting, while a reset has it.
pub fn lock_mirror_in_use(database_path: &Path) -> Result<MirrorUseLock, MirrorUseLockError> {
    let file = mirror_use_lock_file(database_path)?;
    match file.try_lock_shared() {
        Ok(()) => Ok(MirrorUseLock { _file: file }),
        Err(std::fs::TryLockError::WouldBlock) => Err(MirrorUseLockError::Held),
        Err(std::fs::TryLockError::Error(_)) => Err(MirrorUseLockError::Unavailable),
    }
}

/// The exclusive lock a reset takes. Refuses at once, without waiting, while any handle has the
/// mirror open.
pub(crate) fn lock_mirror_exclusively(
    database_path: &Path,
) -> Result<MirrorUseLock, MirrorUseLockError> {
    let file = mirror_use_lock_file(database_path)?;
    match file.try_lock() {
        Ok(()) => Ok(MirrorUseLock { _file: file }),
        Err(std::fs::TryLockError::WouldBlock) => Err(MirrorUseLockError::Held),
        Err(std::fs::TryLockError::Error(_)) => Err(MirrorUseLockError::Unavailable),
    }
}

pub struct ResolvedMirrorKey {
    pub key: Zeroizing<Vec<u8>>,
}

pub fn resolve_mirror_key(
    database_path: &Path,
    key_store: &dyn MirrorKeyStore,
) -> Result<ResolvedMirrorKey, MirrorKeyError> {
    if let Some(key) = key_store.load().map_err(MirrorKeyError::Store)? {
        validate_key(&key)?;
        return Ok(ResolvedMirrorKey { key });
    }

    if mirror_file_exists(database_path)? {
        return Err(MirrorKeyError::MissingForExistingMirror);
    }

    let mut key = Zeroizing::new(vec![0_u8; MIRROR_KEY_BYTES]);
    getrandom::fill(&mut key).map_err(|_| {
        MirrorKeyError::Store(anyhow::anyhow!(
            "The operating system could not generate a Tally mirror key"
        ))
    })?;
    key_store.save(&key).map_err(MirrorKeyError::Store)?;
    Ok(ResolvedMirrorKey { key })
}

/// Whether the database file is there. A path that cannot be checked is not "absent"
/// (`Path::exists` says false on any error): it refuses, so a key is never made beside a
/// database. A leftover write-ahead file alone does not count: with no database file and no
/// key there is nothing to recover, and refusing would lock out a first-run user.
fn mirror_file_exists(database_path: &Path) -> Result<bool, MirrorKeyError> {
    database_path
        .try_exists()
        .map_err(|_| MirrorKeyError::DatabaseStateUnreadable)
}

fn validate_key(key: &[u8]) -> Result<(), MirrorKeyError> {
    if key.len() != MIRROR_KEY_BYTES {
        return Err(MirrorKeyError::StoredKeyInvalid);
    }
    Ok(())
}

pub async fn connect_encrypted(
    database_path: &Path,
    key: Zeroizing<Vec<u8>>,
) -> anyhow::Result<SqlitePool> {
    validate_key(&key)?;
    // SqliteConnectOptions owns PRAGMA values as ordinary strings and the pool keeps those
    // options for its lifetime so that it can replace connections. Keeping SQLCipher's key in
    // `.pragma("key", ...)` would therefore leave an unprotected copy in the pool. Retain the
    // original bytes only in zeroizing storage and apply them to every new SQLite handle through
    // SQLCipher's C API instead.
    let connection_key = Arc::new(key);
    let options = SqliteConnectOptions::new()
        .filename(database_path)
        .create_if_missing(true)
        .disable_statement_logging();

    // SQLx retries `after_connect` failures until its pool deadline and then drops their cause.
    // Forward only observed non-retryable callback failures while they are still owned: SQLite
    // NOTADB, and the exact READONLY_DIRECTORY result reproduced for this initializer. Other
    // callback failures retain SQLx's retry behavior; a forwarded refusal still returns an error
    // to SQLx so an uninitialized connection can never enter the pool.
    let (startup_error_sender, mut startup_error_receiver) = oneshot::channel();
    let startup_error_sender = Arc::new(Mutex::new(Some(startup_error_sender)));
    let key_for_connections = Arc::clone(&connection_key);
    let startup_error_for_connections = Arc::clone(&startup_error_sender);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(0)
        .after_connect(move |connection, _metadata| {
            let key = Arc::clone(&key_for_connections);
            let startup_error_sender = Arc::clone(&startup_error_for_connections);
            Box::pin(async move {
                match configure_encrypted_connection(connection, &key).await {
                    Ok(()) => Ok(()),
                    Err(error) if !is_forwardable_encrypted_open_error(&error) => Err(error),
                    Err(error) => {
                        let sender = startup_error_sender
                            .lock()
                            .ok()
                            .and_then(|mut sender| sender.take());
                        match sender {
                            Some(sender) => match sender.send(error) {
                                Ok(()) => Err(sqlx::Error::Protocol(
                                    "encrypted Tally mirror initialization failed".into(),
                                )),
                                Err(error) => Err(error),
                            },
                            None => Err(error),
                        }
                    }
                }
            })
        })
        .connect_lazy_with(options);

    let mut initial_acquire = Box::pin(pool.acquire());
    let mut connection = tokio::select! {
        biased;
        startup_error = &mut startup_error_receiver => {
            drop(initial_acquire);
            pool.close().await;
            return match startup_error {
                Ok(error) => Err(encrypted_open_error(error).into()),
                Err(_) => Err(EncryptedOpenError::PoolTimedOut.into()),
            };
        }
        acquired = &mut initial_acquire => match acquired {
            Ok(connection) => connection,
            Err(error) => return Err(encrypted_open_error(error).into()),
        },
    };
    drop(startup_error_receiver);

    if let Err(error) = validate_encrypted_connection(&mut connection).await {
        let _ = connection.close().await;
        pool.close().await;
        return Err(error.into());
    }
    drop(connection);
    Ok(pool)
}

/// Whether `key` opens the existing database: one connection, configured and checked as
/// [`connect_encrypted`] does, and closed before this returns, on success and on failure alike.
///
/// A failed attempt through `connect_encrypted` leaves its connection to SQLx to drop, and SQLx
/// ends the connection's worker thread without waiting for it. A caller that goes on to delete the
/// database's files straight away can then find the write-ahead log still open (a sharing violation
/// on Windows), and a process that exits at that moment races the thread's last calls into the
/// crypto library (a crash after a passing test on macOS). `Connection::close` waits for the
/// worker, so neither can happen here. The database is never created.
pub(crate) async fn probe_encrypted_key(
    database_path: &Path,
    key: Zeroizing<Vec<u8>>,
) -> anyhow::Result<()> {
    validate_key(&key)?;
    let mut connection = SqliteConnectOptions::new()
        .filename(database_path)
        .create_if_missing(false)
        .disable_statement_logging()
        .connect()
        .await
        .map_err(encrypted_open_error)?;
    let checked = match configure_encrypted_connection(&mut connection, &key).await {
        Ok(()) => validate_encrypted_connection(&mut connection).await,
        Err(error) => Err(encrypted_open_error(error)),
    };
    // The attempt's answer is the result; a close that fails changes nothing about it.
    let _ = connection.close().await;
    checked.map_err(Into::into)
}

async fn configure_encrypted_connection(
    connection: &mut SqliteConnection,
    key: &[u8],
) -> Result<(), sqlx::Error> {
    {
        let mut handle = connection.lock_handle().await?;
        let key_length = i32::try_from(key.len())
            .map_err(|_| sqlx::Error::Protocol("SQLCipher key is too long".into()))?;
        // SAFETY: `lock_handle()` excludes the SQLx worker for the duration of this call, the
        // handle is live, and `key` remains allocated for the whole call. SQLCipher copies the
        // key into its per-connection codec state.
        let status = unsafe {
            libsqlite3_sys::sqlite3_key(
                handle.as_raw_handle().as_ptr(),
                key.as_ptr().cast(),
                key_length,
            )
        };
        if status != libsqlite3_sys::SQLITE_OK {
            return Err(sqlx::Error::Protocol(
                "SQLCipher rejected the Tally mirror key".into(),
            ));
        }
    }
    // These settings must be applied after sqlite3_key(). Keeping them here means replacement
    // connections receive the same configuration without putting key material in connect options.
    connection
        .execute("PRAGMA cipher_log_level = ERROR;")
        .await?;
    connection
        .execute("PRAGMA cipher_memory_security = ON;")
        .await?;
    connection.execute("PRAGMA secure_delete = ON;").await?;
    connection.execute("PRAGMA foreign_keys = ON;").await?;
    connection.execute("PRAGMA journal_mode = WAL;").await?;
    Ok(())
}

async fn validate_encrypted_connection(
    connection: &mut SqliteConnection,
) -> Result<(), EncryptedOpenError> {
    let cipher_version = sqlx::query_scalar::<_, String>("PRAGMA cipher_version;")
        .fetch_one(&mut *connection)
        .await
        .map_err(encrypted_open_error)?;
    if cipher_version.trim().is_empty() {
        return Err(EncryptedOpenError::CipherUnavailable);
    }
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sqlite_master;")
        .fetch_one(&mut *connection)
        .await
        .map_err(encrypted_open_error)?;
    let integrity_errors = sqlx::query_scalar::<_, String>("PRAGMA cipher_integrity_check;")
        .fetch_all(&mut *connection)
        .await
        .map_err(encrypted_open_error)?;
    if !integrity_errors.is_empty() {
        return Err(EncryptedOpenError::IntegrityFailed);
    }

    let foreign_keys = sqlx::query_scalar::<_, i64>("PRAGMA foreign_keys;")
        .fetch_one(&mut *connection)
        .await
        .map_err(encrypted_open_error)?;
    let secure_delete = sqlx::query_scalar::<_, i64>("PRAGMA secure_delete;")
        .fetch_one(&mut *connection)
        .await
        .map_err(encrypted_open_error)?;
    if foreign_keys != 1 || secure_delete != 1 {
        return Err(EncryptedOpenError::SecuritySettings);
    }
    Ok(())
}

fn encrypted_open_error(error: sqlx::Error) -> EncryptedOpenError {
    if is_definitive_encrypted_open_error(&error) {
        return EncryptedOpenError::UnreadableWithSuppliedKey { source: error };
    }
    if matches!(error, sqlx::Error::PoolTimedOut) {
        return EncryptedOpenError::PoolTimedOut;
    }
    EncryptedOpenError::Storage { source: error }
}

fn is_definitive_encrypted_open_error(error: &sqlx::Error) -> bool {
    sqlite_primary_error_code(error).is_some_and(is_definitive_encrypted_open_sqlite_code)
}

fn is_forwardable_encrypted_open_error(error: &sqlx::Error) -> bool {
    sqlite_error_code(error).is_some_and(is_forwardable_encrypted_open_sqlite_code)
}

fn is_definitive_encrypted_open_sqlite_code(code: i32) -> bool {
    code & 0xff == libsqlite3_sys::SQLITE_NOTADB
}

fn is_forwardable_encrypted_open_sqlite_code(code: i32) -> bool {
    is_definitive_encrypted_open_sqlite_code(code)
        || code == libsqlite3_sys::SQLITE_READONLY_DIRECTORY
}

fn sqlite_primary_error_code(error: &sqlx::Error) -> Option<i32> {
    sqlite_error_code(error).map(|code| code & 0xff)
}

fn sqlite_error_code(error: &sqlx::Error) -> Option<i32> {
    let sqlx::Error::Database(database_error) = error else {
        return None;
    };
    database_error
        .code()
        .and_then(|code| code.parse::<i32>().ok())
}

fn hex_key(key: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(key.len() * 2);
    for byte in key {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

#[cfg(test)]
#[path = "encrypted_tests.rs"]
mod tests;
