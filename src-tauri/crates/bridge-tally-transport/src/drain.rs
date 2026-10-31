//! The drain after a cut post (#2142).
//!
//! Bridge stops waiting for an answer at its limit, but Tally goes on with the
//! work, and the next request waits behind it (protocol reference §11b.2). So a
//! post that may have left Tally working owes a drain, recorded beside the
//! endpoint's wire lock for every process. The next post first sends `GET
//! /status`, which waits behind Tally's work, and goes ahead only once three
//! probes in a row have answered.

use std::time::Duration;

use crate::TallyTransportError;

/// How a transport drains an endpoint that owes one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrainPolicy {
    /// One probe's own deadline. A probe answered whole within it, at any
    /// speed, is an answer; one cut by it is not.
    pub probe_deadline: Duration,
    /// After an answer within this, the call probes again; after a slower one,
    /// which still counts, it is refused, so it does not hold the lock.
    pub quick_below: Duration,
    /// Answers in a row that clear the debt; a cut probe starts the count again.
    pub answers_in_a_row: u8,
    /// The least time from one probe's end to the next probe, across every
    /// process.
    pub spacing: Duration,
    /// The most probes one post sends before it is refused.
    pub probes_per_call: u8,
    /// A debt older than this is cleared on the next post, without probes:
    /// the escape for a Tally whose `/status` never answers within the
    /// deadline though it is not busy.
    pub clear_after: Duration,
}

impl DrainPolicy {
    /// The policy two lab rows on one synthetic book measured
    /// (`lab/2142-status-while-busy-answers`, `lab/2142-drain-repeat-answers`,
    /// 30 Oct 2026, one read cut at 20 s, five times):
    /// - a probe sent after the cut waited 9.3 s and 10.8 s while Tally's work
    ///   ran on, and the work ran 8.9 s to 25.4 s past the cut, so a 5 s probe
    ///   is cut while Tally is busy and nothing waits behind it;
    /// - at rest a probe took 3 ms to 5 ms (median), 46 ms at most;
    /// - each data read sent after three quick probes, 2 s apart, answered
    ///   within its at-rest threshold (four times). A read after fewer, or after
    ///   answers slower than 1 s, was not measured.
    ///
    /// The 10 minutes before a debt clears by itself is not measured: about 24
    /// times the longest drain these rows saw, but only a little over the 523 s
    /// one abandoned 600 s read held the gateway after its client left
    /// (protocol reference §11b.2), so a drain that long would be cleared
    /// shortly before it ended.
    pub const MEASURED: Self = Self {
        probe_deadline: Duration::from_secs(5),
        quick_below: Duration::from_secs(1),
        answers_in_a_row: 3,
        spacing: Duration::from_secs(2),
        probes_per_call: 3,
        clear_after: Duration::from_secs(600),
    };

    /// What a refused call is told to wait: the drain measured 8.9 s to 25.4 s
    /// past a cut, so a retry sooner would only probe again while Tally works.
    pub const fn retry_after(self) -> Duration {
        self.spacing.saturating_mul(5)
    }

    /// The most one call can spend on a drain before it posts or is refused:
    /// quick answers and the pauses between them, then one probe cut by its
    /// deadline.
    pub const fn worst_per_call(self) -> Duration {
        let before_last = self.probes_per_call.saturating_sub(1) as u32;
        self.quick_below
            .saturating_add(self.spacing)
            .saturating_mul(before_last)
            .saturating_add(self.probe_deadline)
    }
}

/// How one drain probe ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Probe {
    /// Answered within `quick_below`: a whole response, or a response head
    /// this crate refuses (a status other than 2xx, another content type or
    /// charset), since `/status` is not part of Tally's documented contract and
    /// any answer shows Tally has finished its work.
    Quick,
    /// Answered within the deadline, but not within `quick_below`.
    Slow,
    /// Cut by its deadline, or no response at all: Tally may still be working.
    Cut,
    /// Nothing listens on the port: no work can remain, so nothing is owed.
    Unreachable,
}

/// Whether a post that ended with `error` may have left Tally working on it.
/// Every error is named, so a new one is decided here before it compiles.
pub(crate) const fn owes_drain(error: &TallyTransportError) -> bool {
    match error {
        // Sent, or perhaps sent, and not answered: Tally may still be working
        // on it, or still sending an answer cut short.
        TallyTransportError::RequestTimedOut
        | TallyTransportError::RequestFailed
        | TallyTransportError::ResponseTooLarge {
            declared_by_peer: false,
            ..
        }
        | TallyTransportError::ResponseTruncated
        | TallyTransportError::ResponseReadFailed => true,
        // Tally answered with a whole head (a status, a declared length, a
        // content type or encoding): it had finished the work.
        TallyTransportError::HttpStatus { .. }
        | TallyTransportError::ResponseTooLarge {
            declared_by_peer: true,
            ..
        }
        | TallyTransportError::UnsupportedContentEncoding
        | TallyTransportError::InvalidEncoding { .. } => false,
        // Nothing reached Tally.
        TallyTransportError::EndpointInvalid { .. }
        | TallyTransportError::PolicyInvalid { .. }
        | TallyTransportError::ClientInitializationFailed
        | TallyTransportError::RequestTooLarge { .. }
        | TallyTransportError::ConnectionFailed
        | TallyTransportError::WireRefused { .. }
        | TallyTransportError::SendWithdrawn => false,
    }
}
