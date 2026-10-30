//! The drain after a cut post (#2142).
//!
//! Bridge stops waiting for an answer at its limit, but Tally goes on with the
//! work, and the next request waits behind it (protocol reference §11b.2). So a
//! post that may have left Tally working owes a drain, recorded beside the
//! endpoint's wire lock for every process. The next post first sends `GET
//! /status`, which waits behind Tally's work, and goes ahead only once enough
//! probes in a row have answered quickly.

use std::time::Duration;

use crate::TallyTransportError;

/// How a transport drains an endpoint that owes one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrainPolicy {
    /// One probe's own deadline: a probe cut by it counts as slow.
    pub probe_deadline: Duration,
    /// A probe answered within this counts as quick.
    pub quick_below: Duration,
    /// Quick probes in a row that clear the debt.
    pub quick_in_a_row: u8,
    /// The least time from one probe's end to the next probe, across every
    /// process.
    pub spacing: Duration,
    /// The most probes one post sends before it is refused.
    pub probes_per_call: u8,
    /// Owed for longer than this, a refusal names something that may need a
    /// person, such as a dialog on Tally's screen.
    pub attention_after: Duration,
}

impl DrainPolicy {
    /// The policy two lab rows on one synthetic book measured
    /// (`lab/2142-status-while-busy-answers`, `lab/2142-drain-repeat-answers`,
    /// 30 Oct 2026, one read cut at 20 s, five times):
    /// - a probe sent after the cut waited 9.3 s and 10.8 s while Tally's work
    ///   ran on, and the work ran 8.9 s to 25.4 s past the cut, so a 5 s probe
    ///   is cut while Tally is busy and nothing waits behind it;
    /// - at rest a probe took 3 ms to 5 ms (median), 46 ms at most, so 1 s
    ///   leaves a wide margin;
    /// - each data read sent after three quick probes, 2 s apart, answered
    ///   within its at-rest threshold (four times). A read after fewer was not
    ///   measured.
    ///
    /// The 120 s before a refusal names a person is not measured: it is well
    /// past the longest drain seen.
    pub const MEASURED: Self = Self {
        probe_deadline: Duration::from_secs(5),
        quick_below: Duration::from_secs(1),
        quick_in_a_row: 3,
        spacing: Duration::from_secs(2),
        probes_per_call: 3,
        attention_after: Duration::from_secs(120),
    };
}

/// How one drain probe ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Probe {
    /// Answered whole within `quick_below`.
    Quick,
    /// Answered slowly, cut by its deadline, or answered with anything but a
    /// whole answer: Tally may still be working.
    Slow,
    /// Nothing listens on the port: no work can remain, so nothing is owed.
    Unreachable,
}

impl DrainPolicy {
    /// What a call refused after a slow probe is told to wait: the drain
    /// measured 8.9 s to 25.4 s past a cut, so a retry sooner would only probe
    /// again while Tally works.
    pub(crate) const fn retry_after_slow(self) -> Duration {
        self.spacing.saturating_mul(5)
    }

    /// What a call refused as needing attention is told to wait.
    pub(crate) const fn retry_after_attention(self) -> Duration {
        self.spacing.saturating_mul(15)
    }
}

/// Whether a post that ended with `error` may have left Tally working on it.
/// Every error is named, so a new one is decided here before it compiles.
pub(crate) const fn owes_drain(error: &TallyTransportError) -> bool {
    match error {
        // Sent, or perhaps sent, and not answered whole.
        TallyTransportError::RequestTimedOut
        | TallyTransportError::RequestFailed
        | TallyTransportError::HttpStatus { .. }
        | TallyTransportError::ResponseTooLarge { .. }
        | TallyTransportError::ResponseTruncated
        | TallyTransportError::ResponseReadFailed
        | TallyTransportError::UnsupportedContentEncoding
        | TallyTransportError::InvalidEncoding { .. } => true,
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
