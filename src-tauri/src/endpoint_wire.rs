//! The per-port wire lock (#697 PR A, option (1)).
//!
//! `<key>.wire.lock` lives beside each port's dispatch lease
//! ([`crate::endpoint_coordination::lease_path`]), under each of the same
//! shared per-user coordination roots, so the desktop app and `bridge_mcp` see
//! the same one; one file per root, taken all or none (#1584). Every file is
//! opened before any is locked, and a busy one releases those already taken,
//! so the holder still never waits holding it. It is taken exclusively, with a
//! try-lock, around each single HTTP send
//! to that port, and released when the response has been read to its end or
//! the send fails. It is a different file from the lease, so a send made under
//! a held lease never conflicts with that lease; the lease (if any) is always
//! taken first. The transport crate's `wire_gate` module states the invariant
//! this relies on: the wire lock is held for one send only, and its holder
//! never waits on anything while holding it.
//!
//! Stated limits:
//! - The key is the port, as for the lease: every loopback spelling of one port
//!   shares it. A LAN spelling of a local Tally, or a second machine reading the
//!   same Tally, is invisible to it.
//! - A crash releases the lock: the OS drops it with the process.
//! - The lock follows Bridge's own request, not Tally's work: it is released at
//!   the client deadline, or when the send's future is dropped, while Tally may
//!   still be computing an abandoned request (protocol reference §11b.2). A
//!   later send can therefore reach a Tally that is still busy.
//! - There is no queue order: waiters poll, so a process that sends back to
//!   back can keep taking the lock between another process's polls, and that
//!   waiter can end as busy after its budget though each send was short. The
//!   import POST is one try after its checks, so this can refuse a post whose
//!   approval then lapses and is asked for again (#869).
use crate::endpoint_coordination::lease_path;
use crate::local_files::lock_set::{try_lock_all, HeldLocks, LockMode, LockSetError};
#[cfg(not(test))]
use crate::local_files::paths::dispatch_coordination_roots;
use bridge_tally_transport::{
    TallyEndpointConfig, TallyWireGate, WireLockHeld, WirePause, WireRefusal, WireRetryPolicy,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

/// Where the wire locks live: the shared per-user coordination roots in a
/// build, a directory of its own for each runtime in unit tests, so a lock one
/// test holds on a port can never refuse another test given the same port.
#[derive(Clone)]
pub(crate) struct WireRoot {
    paths: Option<Vec<PathBuf>>,
    #[cfg(test)]
    _isolated: Option<Arc<tempfile::TempDir>>,
}

impl WireRoot {
    #[cfg(not(test))]
    fn shared() -> Self {
        Self {
            paths: dispatch_coordination_roots(),
        }
    }

    #[cfg(test)]
    pub(crate) fn isolated() -> Self {
        match tempfile::tempdir() {
            Ok(directory) => Self {
                paths: Some(vec![directory.path().to_path_buf()]),
                _isolated: Some(Arc::new(directory)),
            },
            // No root fails closed: every send is refused as unavailable.
            Err(_) => Self {
                paths: None,
                _isolated: None,
            },
        }
    }

    /// Test only: every root a build takes the lock under, in order.
    #[cfg(test)]
    pub(crate) fn at_all(paths: Vec<PathBuf>) -> Self {
        Self {
            paths: Some(paths),
            _isolated: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn at(path: PathBuf) -> Self {
        Self {
            paths: Some(vec![path]),
            _isolated: None,
        }
    }
}

/// What every Tally client of one runtime gates its sends with.
#[derive(Clone)]
pub(crate) struct WireGateConfig {
    root: WireRoot,
    retry: WireRetryPolicy,
    /// Test only: the send, counted over every client of the runtime from 0,
    /// whose every try is refused busy (#869).
    #[cfg(test)]
    busy_at: Option<Arc<BusyAt>>,
}

/// Counts the sends granted so far, and refuses every try of one of them.
#[cfg(test)]
pub(crate) struct BusyAt {
    send: usize,
    granted: std::sync::atomic::AtomicUsize,
}

/// The file lock, except that every try of one chosen send is refused busy,
/// as another process holding the lock for that whole send would.
#[cfg(test)]
struct BusyAtGate {
    inner: FileWireGate,
    busy: Arc<BusyAt>,
}

#[cfg(test)]
impl TallyWireGate for BusyAtGate {
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        use std::sync::atomic::Ordering;
        if self.busy.granted.load(Ordering::SeqCst) == self.busy.send {
            return Err(WireRefusal::Busy);
        }
        let held = self.inner.try_acquire()?;
        self.busy.granted.fetch_add(1, Ordering::SeqCst);
        Ok(held)
    }

    fn pause(&self, delay: Duration) -> WirePause {
        self.inner.pause(delay)
    }
}

impl Default for WireGateConfig {
    fn default() -> Self {
        #[cfg(not(test))]
        let root = WireRoot::shared();
        #[cfg(test)]
        let root = WireRoot::isolated();
        Self {
            root,
            retry: WireRetryPolicy::DEFAULT,
            #[cfg(test)]
            busy_at: None,
        }
    }
}

impl WireGateConfig {
    #[cfg(test)]
    pub(crate) fn root(&self) -> &WireRoot {
        &self.root
    }

    pub(crate) fn retry(&self) -> WireRetryPolicy {
        self.retry
    }

    pub(crate) fn gate_for(&self, endpoint: &TallyEndpointConfig) -> Arc<dyn TallyWireGate> {
        let file = FileWireGate::new(self.root.clone(), endpoint.clone());
        #[cfg(test)]
        if let Some(busy) = &self.busy_at {
            return Arc::new(BusyAtGate {
                inner: file,
                busy: busy.clone(),
            });
        }
        Arc::new(file)
    }

    /// Test only: refuse every try of the `send`th send busy (0-based, over
    /// every client of the runtime), waiting at most `total` for it.
    #[cfg(test)]
    pub(crate) fn busy_at_send(mut self, send: usize, total: Duration) -> Self {
        self.busy_at = Some(Arc::new(BusyAt {
            send,
            granted: std::sync::atomic::AtomicUsize::new(0),
        }));
        self.retry =
            WireRetryPolicy::new(Duration::from_millis(1), total).expect("a test wire policy");
        self
    }

    #[cfg(test)]
    pub(crate) fn with_retry(mut self, retry: WireRetryPolicy) -> Self {
        self.retry = retry;
        self
    }
}

/// The wire lock's paths: same roots, same key, same directory as the
/// dispatch lease.
fn lock_paths(
    root: &WireRoot,
    endpoint: &TallyEndpointConfig,
) -> Result<Vec<PathBuf>, WireRefusal> {
    let roots = root.paths.as_deref().ok_or(WireRefusal::Unavailable)?;
    roots
        .iter()
        .map(|root| {
            let lease = lease_path(root, endpoint).map_err(|_| WireRefusal::Unavailable)?;
            Ok(lease.with_extension("wire.lock"))
        })
        .collect()
}

/// The file-lock gate the application injects into every Tally transport.
pub(crate) struct FileWireGate {
    root: WireRoot,
    endpoint: TallyEndpointConfig,
}

impl FileWireGate {
    pub(crate) fn new(root: WireRoot, endpoint: TallyEndpointConfig) -> Self {
        Self { root, endpoint }
    }
}

/// Dropping it releases every lock of the set.
struct HeldWireLock {
    _locks: HeldLocks,
}

impl WireLockHeld for HeldWireLock {}

impl TallyWireGate for FileWireGate {
    /// Try once, never waiting, to take the wire lock.
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        let paths = lock_paths(&self.root, &self.endpoint)?;
        match try_lock_all(&paths, LockMode::Exclusive) {
            Ok(locks) => Ok(Box::new(HeldWireLock { _locks: locks })),
            Err(LockSetError::Busy) => Err(WireRefusal::Busy),
            Err(LockSetError::Unavailable) => Err(WireRefusal::Unavailable),
        }
    }

    fn pause(&self, delay: Duration) -> WirePause {
        Box::pin(tokio::time::sleep(delay))
    }
}

/// The endpoint wire gate's refusal in a runtime error chain, if any.
pub(crate) fn wire_refusal(error: &anyhow::Error) -> Option<WireRefusal> {
    error.chain().find_map(|cause| {
        match cause.downcast_ref::<bridge_tally_transport::TallyTransportError>()? {
            bridge_tally_transport::TallyTransportError::WireRefused { refusal } => Some(*refusal),
            _ => None,
        }
    })
}

#[cfg(test)]
pub(crate) fn wire_lock_path(root: &WireRoot, endpoint: &TallyEndpointConfig) -> PathBuf {
    lock_paths(root, endpoint)
        .expect("test wire root resolves")
        .swap_remove(0)
}

#[cfg(test)]
#[path = "endpoint_wire_tests.rs"]
mod tests;
