//! The per-send wire gate (#697 PR A, option (1)).
//!
//! Tally serves one request at a time; a second request sent while it is busy
//! queues behind the first. The gate lets the application admit at most one
//! HTTP send at a time to an endpoint across every Bridge process on the
//! machine, without this crate knowing where the application keeps its lock
//! files: the application injects a [`TallyWireGate`] and this crate calls it
//! around each send.
//!
//! # Invariant
//!
//! **The wire lock is held for exactly one HTTP send, and its holder never waits
//! on anything while holding it.** It is taken (never waited for) immediately
//! before the request is sent and released when the response has been read to
//! its end or the send fails; nothing between those two points waits on another
//! process or on a lock. A refused attempt holds nothing, so the pause between
//! attempts is taken with no wire lock held. The only locks a caller may hold
//! while this crate retries are the ones it took before calling (an outer
//! dispatch or snapshot lease), and those are always taken before the wire lock,
//! never after it. So no cycle can form: whoever holds the wire lock is only
//! waiting on Tally, and Tally answers or the request deadline ends the send.
//!
//! A transport given a [`crate::DrainPolicy`] holds the lock across a drain
//! probe and the post it clears (#2142): one `GET /status` with its own short
//! deadline, then the post, so the lock can be held for both. Between probes it
//! releases the lock and pauses, as between refused attempts.
//!
//! The one caller-visible exception is [`TallyHttpTransport::acquire_wire_lock`]
//! (one try, never a wait): a caller takes the lock, does local work (a durable record on the local
//! disk, which does not wait on another process), and spends the lock on
//! exactly one send through the returned [`WireHeldTransport`], which is
//! consumed by that send, so it cannot cover a second one.
//!
//! # Waiting
//!
//! A busy lock is tried again, but one operation's waits all draw on one
//! [`WireWaitBudget`] of at most [`WIRE_WAIT_MAX`] (#697 item (a)), whatever
//! the number of its sends; see [`TallyHttpTransport::for_operation`].
//!
//! [`TallyHttpTransport::acquire_wire_lock`]: crate::TallyHttpTransport::acquire_wire_lock
//! [`TallyHttpTransport::for_operation`]: crate::TallyHttpTransport::for_operation
//! [`WireHeldTransport`]: crate::WireHeldTransport

use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

/// The most one operation may wait, in total over all its sends, for another
/// holder of the wire lock before it is refused as [`WireRefusal::Busy`].
pub const WIRE_WAIT_MAX: Duration = Duration::from_secs(10);

/// The retry hint returned with a [`WireRefusal::Busy`] refusal.
pub const WIRE_BUSY_RETRY_AFTER: Duration = Duration::from_secs(5);

/// Why the gate refused a send. Nothing was sent in any case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireRefusal {
    /// Another send to this endpoint held the wire lock for the whole bounded
    /// wait. Retryable after [`WIRE_BUSY_RETRY_AFTER`].
    Busy,
    /// The lock could not be opened, or its drain record could not be read or
    /// written. The gate fails closed: an unverifiable lock is not a held one.
    Unavailable,
    /// An earlier send to this endpoint was cut while Tally may still have been
    /// working on it, and `GET /status` has not yet answered quickly enough
    /// times since (#2142). Retryable after `retry_after_ms`.
    Draining { retry_after_ms: u64 },
    /// As [`Self::Draining`], when no probe has answered quickly for longer
    /// than a drain was measured to last: something on Tally's side, such as a
    /// dialog, may need a person.
    NeedsAttention { retry_after_ms: u64 },
}

impl WireRefusal {
    pub const fn safe_code(self) -> &'static str {
        match self {
            Self::Busy => "tally_endpoint_busy",
            Self::Unavailable => "tally_endpoint_lock_unavailable",
            Self::Draining { .. } => "tally_endpoint_draining",
            Self::NeedsAttention { .. } => "tally_endpoint_needs_attention",
        }
    }

    /// When the same call may be tried again, for a refusal that passes.
    pub const fn retry_after(self) -> Option<Duration> {
        match self {
            Self::Busy => Some(WIRE_BUSY_RETRY_AFTER),
            Self::Unavailable => None,
            Self::Draining { retry_after_ms } | Self::NeedsAttention { retry_after_ms } => {
                Some(Duration::from_millis(retry_after_ms))
            }
        }
    }
}

/// What an endpoint's drain record says (#2142), read and written only by the
/// holder of its wire lock, so every process sees one record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrainDebt {
    /// Nothing owed.
    Clear,
    /// A post began at `since_ms` and its sender has not settled it. A later
    /// holder finds its sender gone (the lock is released only after a send
    /// ends), so it is owed.
    InFlight { since_ms: u64 },
    /// A post was cut at `since_ms`. Since then `quick` probes in a row
    /// answered quickly, or `slow` in a row answered whole but slowly; the
    /// last probe was sent at `last_probe_ms`, and the current run of probes
    /// that were not quick began at `slow_since_ms`.
    Owed {
        since_ms: u64,
        quick: u8,
        slow: u8,
        last_probe_ms: Option<u64>,
        slow_since_ms: Option<u64>,
    },
}

/// One held wire lock, owned by the gate's implementation. Dropping it
/// releases the lock. A gate that keeps no drain record (lab tools, this
/// crate's tests) is always clear and records nothing.
pub trait WireLockHeld: Send + Sync {
    /// The endpoint's drain record, read under this lock.
    fn debt(&self) -> Result<DrainDebt, WireRefusal> {
        Ok(DrainDebt::Clear)
    }

    /// Replace the endpoint's drain record with `debt`, under this lock.
    fn record(&self, _debt: DrainDebt) -> Result<(), WireRefusal> {
        Ok(())
    }
}

/// A pause between two refused attempts, with no wire lock held.
pub type WirePause = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// The application's per-endpoint wire lock.
///
/// Implementations must honour the module invariant: [`Self::try_acquire`]
/// never blocks (a taken lock is refused at once as [`WireRefusal::Busy`]),
/// and a refusal holds nothing.
pub trait TallyWireGate: Send + Sync {
    /// Try once, without waiting, to take the wire lock for one send.
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal>;

    /// Wait `delay` before the next attempt. This crate has no timer of its
    /// own, and a test can make the pause instant.
    fn pause(&self, delay: Duration) -> WirePause;

    /// The wall clock in milliseconds since the Unix epoch, for the drain
    /// record shared across processes. A test can set it.
    fn now_ms(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |since| {
                u64::try_from(since.as_millis()).unwrap_or(u64::MAX)
            })
    }
}

/// How far apart a busy wire lock is tried again, and how much waiting one
/// operation may spend on it in total, over all its sends, before a send is
/// refused. The total is at most [`WIRE_WAIT_MAX`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireRetryPolicy {
    delay: Duration,
    total: Duration,
}

impl WireRetryPolicy {
    /// 500 ms apart, 9.5 s per operation. The budget is charged each pause's
    /// requested length; the 0.5 s under [`WIRE_WAIT_MAX`] absorbs a timer's
    /// overrun of the at most 19 pauses, so the wall-clock wait stays within
    /// 10 s.
    pub const DEFAULT: Self = Self {
        delay: Duration::from_millis(500),
        total: Duration::from_millis(9_500),
    };

    /// `None` for a zero delay (a pause that charges nothing would never
    /// exhaust the budget) or a total over [`WIRE_WAIT_MAX`]. A zero total
    /// tries once and never waits.
    pub fn new(delay: Duration, total: Duration) -> Option<Self> {
        (!delay.is_zero() && total <= WIRE_WAIT_MAX).then_some(Self { delay, total })
    }

    pub const fn delay(self) -> Duration {
        self.delay
    }

    /// The most one operation waits for the wire lock before it is refused.
    pub const fn total(self) -> Duration {
        self.total
    }
}

impl Default for WireRetryPolicy {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// One operation's allowance for waiting on the wire lock, drawn on by every
/// send it makes (#697 item (a)). Clones share it, so every clone of a
/// transport scoped with [`crate::TallyHttpTransport::for_operation`] draws on
/// the same allowance, and retries of the operation draw on it too.
#[derive(Clone, Debug)]
pub struct WireWaitBudget {
    remaining_nanos: Arc<AtomicU64>,
}

impl WireWaitBudget {
    /// A fresh allowance of `total`, capped at [`WIRE_WAIT_MAX`].
    pub fn new(total: Duration) -> Self {
        let nanos = total.min(WIRE_WAIT_MAX).as_nanos();
        Self {
            remaining_nanos: Arc::new(AtomicU64::new(u64::try_from(nanos).unwrap_or(u64::MAX))),
        }
    }

    /// What is left of the allowance.
    pub fn remaining(&self) -> Duration {
        Duration::from_nanos(self.remaining_nanos.load(Ordering::SeqCst))
    }

    /// Take up to `step` from the allowance for one pause: the pause to take,
    /// or `None` once nothing is left. Atomic, so two sends of one operation
    /// racing on a shared clone can never draw the same time twice.
    fn draw(&self, step: Duration) -> Option<Duration> {
        // At least 1 ns, so every draw shrinks the allowance and the retry
        // loop ends even if a zero step ever reached it.
        let step = u64::try_from(step.as_nanos()).unwrap_or(u64::MAX).max(1);
        let before = self
            .remaining_nanos
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |left| {
                (left > 0).then_some(left.saturating_sub(step))
            })
            .ok()?;
        Some(Duration::from_nanos(before.min(step)))
    }
}

/// A gate that admits every send. It is the default of a transport built
/// without [`crate::TallyHttpTransport::with_wire_gate`]: lab tools and this
/// crate's own tests, which have always sent ungated. The application injects
/// its file lock for every Tally client it builds.
#[derive(Clone, Copy, Debug, Default)]
pub struct UngatedWire;

struct UngatedLock;

impl WireLockHeld for UngatedLock {}

impl TallyWireGate for UngatedWire {
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        Ok(Box::new(UngatedLock))
    }

    fn pause(&self, _delay: Duration) -> WirePause {
        Box::pin(std::future::ready(()))
    }
}

/// Take the wire lock for one send, trying again `delay` apart while another
/// holder has it, for as long as the operation's `budget` lasts. Each pause is
/// charged to the budget before it is taken, at its requested length, so the
/// operation's pauses never add up to more than the budget's total. Only
/// [`WireRefusal::Busy`] is tried again: an unusable lock will not change by
/// waiting.
pub(crate) async fn acquire(
    gate: &Arc<dyn TallyWireGate>,
    delay: Duration,
    budget: &WireWaitBudget,
) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
    loop {
        match gate.try_acquire() {
            Ok(held) => return Ok(held),
            Err(WireRefusal::Busy) => {
                let Some(pause) = budget.draw(delay) else {
                    return Err(WireRefusal::Busy);
                };
                // A refused attempt holds nothing; nothing is held here but
                // what the caller took before calling.
                gate.pause(pause).await;
            }
            Err(refusal) => return Err(refusal),
        }
    }
}
