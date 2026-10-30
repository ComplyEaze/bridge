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
//!   still be computing an abandoned request (protocol reference §11b.2). So
//!   beside it, in the shared profile root only, `<key>.wire.debt` records a
//!   post in flight or cut (#2142), read and written only by the lock's holder,
//!   and the next post drains Tally first (the transport crate's `drain`
//!   module). The record is one JSON object, written content first; clear is
//!   an explicit state, an absent file is clear, and anything else unreadable
//!   is owed.
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
    DrainDebt, DrainPolicy, TallyEndpointConfig, TallyWireGate, WireLockHeld, WirePause,
    WireRefusal, WireRetryPolicy,
};
use std::path::{Path, PathBuf};
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
    /// How a post drains an endpoint after a cut one (#2142): the measured
    /// policy in a build, none in unit tests unless one asks for it.
    drain: Option<DrainPolicy>,
    /// Test only: the send, counted over every client of the runtime from 0,
    /// whose every try is refused busy (#869).
    #[cfg(test)]
    busy_at: Option<Arc<BusyAt>>,
    /// Test only: the sends, counted as for `busy_at`, before each of which
    /// the drain record is found owed and probed just now (#2142).
    #[cfg(test)]
    owe_at: Option<Arc<OweAt>>,
}

/// Counts the sends granted so far, and finds the drain owed before each of
/// the chosen ones.
#[cfg(test)]
pub(crate) struct OweAt {
    sends: Vec<usize>,
    granted: std::sync::atomic::AtomicUsize,
}

/// The file lock, except that the chosen sends find the drain record owed with
/// a probe sent just now, as another process that had just probed a draining
/// Tally would leave it.
#[cfg(test)]
struct OweAtGate {
    inner: FileWireGate,
    owe: Arc<OweAt>,
}

#[cfg(test)]
impl TallyWireGate for OweAtGate {
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        use std::sync::atomic::Ordering;
        let held = self.inner.try_acquire()?;
        let send = self.owe.granted.fetch_add(1, Ordering::SeqCst);
        if self.owe.sends.contains(&send) {
            let now = self.now_ms();
            held.record(DrainDebt::Owed {
                since_ms: now,
                quick: 0,
                slow: 0,
                last_probe_ms: Some(now),
                slow_since_ms: None,
            })?;
        }
        Ok(held)
    }

    fn pause(&self, delay: Duration) -> WirePause {
        self.inner.pause(delay)
    }
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
            #[cfg(not(test))]
            drain: production_drain(),
            #[cfg(test)]
            drain: None,
            #[cfg(test)]
            busy_at: None,
            #[cfg(test)]
            owe_at: None,
        }
    }
}

/// The drain policy every build posts under (#2142).
pub(crate) const fn production_drain() -> Option<DrainPolicy> {
    Some(DrainPolicy::MEASURED)
}

impl WireGateConfig {
    #[cfg(test)]
    pub(crate) fn root(&self) -> &WireRoot {
        &self.root
    }

    pub(crate) fn retry(&self) -> WireRetryPolicy {
        self.retry
    }

    pub(crate) fn drain(&self) -> Option<DrainPolicy> {
        self.drain
    }

    #[cfg(test)]
    pub(crate) fn with_drain(mut self, drain: DrainPolicy) -> Self {
        self.drain = Some(drain);
        self
    }

    pub(crate) fn gate_for(&self, endpoint: &TallyEndpointConfig) -> Arc<dyn TallyWireGate> {
        let file = FileWireGate::new(self.root.clone(), endpoint.clone());
        #[cfg(test)]
        if let Some(owe) = &self.owe_at {
            return Arc::new(OweAtGate {
                inner: file,
                owe: owe.clone(),
            });
        }
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

    /// Test only: drain by the measured policy, and find the drain owed, with a
    /// probe sent just now, before each of `sends` (0-based, over every client
    /// of the runtime), so each of them is refused as draining with no probe.
    #[cfg(test)]
    pub(crate) fn owe_at_sends(mut self, sends: Vec<usize>) -> Self {
        self.drain = Some(DrainPolicy::MEASURED);
        self.owe_at = Some(Arc::new(OweAt {
            sends,
            granted: std::sync::atomic::AtomicUsize::new(0),
        }));
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

/// The drain record's path: beside the wire lock, in the shared profile root
/// only (the last root), so the packaged and unpackaged Windows roots cannot
/// disagree.
fn debt_path(root: &WireRoot, endpoint: &TallyEndpointConfig) -> Result<PathBuf, WireRefusal> {
    let shared = root
        .paths
        .as_deref()
        .and_then(<[PathBuf]>::last)
        .ok_or(WireRefusal::Unavailable)?;
    let lease = lease_path(shared, endpoint).map_err(|_| WireRefusal::Unavailable)?;
    Ok(lease.with_extension("wire.debt"))
}

/// The record as written: `state` and its fields.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
enum DebtRecord {
    Clear,
    InFlight {
        since_ms: u64,
    },
    Owed {
        since_ms: u64,
        quick: u8,
        slow: u8,
        last_probe_ms: Option<u64>,
        slow_since_ms: Option<u64>,
    },
}

fn read_debt(path: &Path, now_ms: u64) -> Result<DrainDebt, WireRefusal> {
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(DrainDebt::Clear),
        Err(_) => return Err(WireRefusal::Unavailable),
    };
    // Exactly the fields its state writes: serde's tagged form would let an
    // unknown field through a clear record.
    let fields = |count| {
        serde_json::from_slice::<serde_json::Map<String, serde_json::Value>>(&bytes)
            .is_ok_and(|object| object.len() == count)
    };
    let parsed = serde_json::from_slice(&bytes).ok().filter(|record| {
        fields(match record {
            DebtRecord::Clear => 1,
            DebtRecord::InFlight { .. } => 2,
            DebtRecord::Owed { .. } => 6,
        })
    });
    Ok(match parsed.ok_or(()) {
        Ok(DebtRecord::Clear) => DrainDebt::Clear,
        Ok(DebtRecord::InFlight { since_ms }) => DrainDebt::InFlight { since_ms },
        Ok(DebtRecord::Owed {
            since_ms,
            quick,
            slow,
            last_probe_ms,
            slow_since_ms,
        }) => DrainDebt::Owed {
            since_ms,
            quick,
            slow,
            last_probe_ms,
            slow_since_ms,
        },
        // Unreadable is owed: the cost is a probe.
        Err(_) => DrainDebt::Owed {
            since_ms: now_ms,
            quick: 0,
            slow: 0,
            last_probe_ms: None,
            slow_since_ms: None,
        },
    })
}

fn write_debt(path: &Path, debt: DrainDebt) -> Result<(), WireRefusal> {
    use std::io::Write;
    let record = match debt {
        DrainDebt::Clear => DebtRecord::Clear,
        DrainDebt::InFlight { since_ms } => DebtRecord::InFlight { since_ms },
        DrainDebt::Owed {
            since_ms,
            quick,
            slow,
            last_probe_ms,
            slow_since_ms,
        } => DebtRecord::Owed {
            since_ms,
            quick,
            slow,
            last_probe_ms,
            slow_since_ms,
        },
    };
    let bytes = serde_json::to_vec(&record).map_err(|_| WireRefusal::Unavailable)?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .map_err(|_| WireRefusal::Unavailable)?;
    // Content first, then the length: a reader never sees an empty record.
    file.write_all(&bytes)
        .and_then(|()| file.set_len(u64::try_from(bytes.len()).unwrap_or(u64::MAX)))
        .map_err(|_| WireRefusal::Unavailable)
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
    debt: Result<PathBuf, WireRefusal>,
}

impl WireLockHeld for HeldWireLock {
    fn debt(&self) -> Result<DrainDebt, WireRefusal> {
        read_debt(self.debt.as_ref().map_err(|refusal| *refusal)?, wall_ms())
    }

    fn record(&self, debt: DrainDebt) -> Result<(), WireRefusal> {
        write_debt(self.debt.as_ref().map_err(|refusal| *refusal)?, debt)
    }
}

fn wall_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| {
            u64::try_from(since.as_millis()).unwrap_or(u64::MAX)
        })
}

impl TallyWireGate for FileWireGate {
    /// Try once, never waiting, to take the wire lock.
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        let paths = lock_paths(&self.root, &self.endpoint)?;
        match try_lock_all(&paths, LockMode::Exclusive) {
            Ok(locks) => Ok(Box::new(HeldWireLock {
                _locks: locks,
                debt: debt_path(&self.root, &self.endpoint),
            })),
            Err(LockSetError::Busy) => Err(WireRefusal::Busy),
            Err(LockSetError::Unavailable) => Err(WireRefusal::Unavailable),
        }
    }

    fn pause(&self, delay: Duration) -> WirePause {
        Box::pin(tokio::time::sleep(delay))
    }
}

/// The seconds after which a call the wire gate refused, by its safe code, may
/// be tried again; `None` for a code that is not such a refusal or does not
/// pass by waiting.
pub(crate) fn wire_retry_after_s(code: &str) -> Option<u64> {
    let refusal = match code {
        "tally_endpoint_busy" => WireRefusal::Busy,
        "tally_endpoint_draining" => WireRefusal::Draining {
            retry_after_ms: millis(DrainPolicy::MEASURED.spacing.saturating_mul(5)),
        },
        "tally_endpoint_needs_attention" => WireRefusal::NeedsAttention {
            retry_after_ms: millis(DrainPolicy::MEASURED.spacing.saturating_mul(15)),
        },
        _ => return None,
    };
    refusal.retry_after().map(|after| after.as_secs())
}

fn millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
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
