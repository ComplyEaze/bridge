//! Fixed-cardinality, local-only Tally transport observations.
//!
//! Folded from the former standalone `bridge-tally-observability` crate: it had exactly one
//! consumer (`bridge-tally-runtime`, itself folded into `crate::tally::runtime_control`), so the
//! crate boundary earned nothing and only hid this module's true dead code from the `dead_code`
//! lint (a `pub` item in a library crate is never flagged, since an external consumer might exist).
//! This module deliberately still has no runtime, HTTP, database, logging, tracing, persistence,
//! system-metrics, or exporter dependency. Nothing reads the counters out in a shipped build: the
//! privacy-reduced preview that did was never wired to anything, and was deleted.

use std::{fmt, sync::Mutex, time::Duration};

pub(crate) const LATENCY_UPPER_BOUNDS_MICROS: [u64; 8] = [
    1_000, 5_000, 25_000, 100_000, 500_000, 2_000_000, 10_000_000, 30_000_000,
];
pub(crate) const RESPONSE_BYTE_UPPER_BOUNDS: [u64; 6] = [
    0,
    1_024,
    64 * 1_024,
    1_024 * 1_024,
    8 * 1_024 * 1_024,
    32 * 1_024 * 1_024,
];

const LATENCY_BUCKETS: usize = LATENCY_UPPER_BOUNDS_MICROS.len() + 1;
const BYTE_BUCKETS: usize = RESPONSE_BYTE_UPPER_BOUNDS.len() + 1;
const QUEUE_OUTCOMES: usize = QueueOutcome::ALL.len();
const RESPONSE_OUTCOMES: usize = ResponseOutcome::ALL.len();
const CIRCUIT_REJECT_REASONS: usize = CircuitRejectReason::ALL.len();
const REQUEST_CLASSES: usize = RequestClass::ALL.len();
const QUEUE_CELLS: usize = REQUEST_CLASSES * QUEUE_OUTCOMES * LATENCY_BUCKETS;
const RESPONSE_LATENCY_CELLS: usize = REQUEST_CLASSES * RESPONSE_OUTCOMES * LATENCY_BUCKETS;
const RESPONSE_BYTE_CELLS: usize = REQUEST_CLASSES * RESPONSE_OUTCOMES * BYTE_BUCKETS;
const RESPONSE_BYTE_UNAVAILABLE_CELLS: usize = REQUEST_CLASSES * RESPONSE_OUTCOMES;
const CIRCUIT_REJECTION_CELLS: usize = REQUEST_CLASSES * CIRCUIT_REJECT_REASONS;
pub(crate) const FIXED_HISTOGRAM_CELL_COUNT: usize = QUEUE_CELLS
    + RESPONSE_LATENCY_CELLS
    + RESPONSE_BYTE_CELLS
    + RESPONSE_BYTE_UNAVAILABLE_CELLS
    + CIRCUIT_REJECTION_CELLS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RequestClass {
    Status,
    Capability,
    CompanyList,
    MasterExport,
    VoucherExport,
    ReportExport,
    Import,
    OtherRead,
}

impl RequestClass {
    pub(crate) const ALL: [Self; 8] = [
        Self::Status,
        Self::Capability,
        Self::CompanyList,
        Self::MasterExport,
        Self::VoucherExport,
        Self::ReportExport,
        Self::Import,
        Self::OtherRead,
    ];

    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QueueOutcome {
    Acquired,
    Deadline,
    Cancelled,
}

impl QueueOutcome {
    pub(crate) const ALL: [Self; 3] = [Self::Acquired, Self::Deadline, Self::Cancelled];

    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResponseOutcome {
    Success,
    Cancelled,
    Timeout,
    Transport,
    HttpStatus,
    SizeLimit,
    Decode,
    Application,
    Parse,
    Validation,
}

impl ResponseOutcome {
    pub(crate) const ALL: [Self; 10] = [
        Self::Success,
        Self::Cancelled,
        Self::Timeout,
        Self::Transport,
        Self::HttpStatus,
        Self::SizeLimit,
        Self::Decode,
        Self::Application,
        Self::Parse,
        Self::Validation,
    ];

    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CircuitRejectReason {
    Cooldown,
    HalfOpenProbeInFlight,
}

impl CircuitRejectReason {
    pub(crate) const ALL: [Self; 2] = [Self::Cooldown, Self::HalfOpenProbeInFlight];

    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BodyBytesObservation {
    Observed(u64),
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AttemptObservation {
    CircuitRejected {
        class: RequestClass,
        reason: CircuitRejectReason,
    },
    QueueDeadline {
        class: RequestClass,
        queue_wait: Duration,
    },
    QueueCancelled {
        class: RequestClass,
        queue_wait: Duration,
    },
    Response {
        class: RequestClass,
        queue_wait: Duration,
        outcome: ResponseOutcome,
        /// Custom Bridge pipeline duration from send start through bounded
        /// body read and decode completion. This is not an OTel HTTP duration.
        response_pipeline_elapsed: Duration,
        /// Bytes consumed before the terminal outcome, including partial
        /// failed bodies. This is not an OTel HTTP response-body-size metric.
        observed_body_bytes: BodyBytesObservation,
    },
}

/// A sink accepts one caller-supplied terminal attempt and no dynamic labels
/// or text. It aggregates observations; it does not authenticate provenance or
/// detect duplicate calls.
pub(crate) trait ObservationSink {
    fn record_attempt(&self, observation: AttemptObservation);
}

#[derive(Clone)]
struct AggregateState {
    queue_latency: [u64; QUEUE_CELLS],
    response_latency: [u64; RESPONSE_LATENCY_CELLS],
    response_bytes: [u64; RESPONSE_BYTE_CELLS],
    response_bytes_unavailable: [u64; RESPONSE_BYTE_UNAVAILABLE_CELLS],
    circuit_rejections: [u64; CIRCUIT_REJECTION_CELLS],
    saturated_cell_increments: u64,
}

impl Default for AggregateState {
    fn default() -> Self {
        Self {
            queue_latency: [0; QUEUE_CELLS],
            response_latency: [0; RESPONSE_LATENCY_CELLS],
            response_bytes: [0; RESPONSE_BYTE_CELLS],
            response_bytes_unavailable: [0; RESPONSE_BYTE_UNAVAILABLE_CELLS],
            circuit_rejections: [0; CIRCUIT_REJECTION_CELLS],
            saturated_cell_increments: 0,
        }
    }
}

/// Fixed-memory coherent aggregation. Poison recovery retains observations;
/// measurement failure never changes a Tally operation result.
#[derive(Default)]
pub(crate) struct TelemetryCollector {
    state: Mutex<AggregateState>,
}

impl fmt::Debug for TelemetryCollector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TelemetryCollector")
            .field("fixed_histogram_cell_count", &FIXED_HISTOGRAM_CELL_COUNT)
            .finish_non_exhaustive()
    }
}

impl TelemetryCollector {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    fn snapshot(&self) -> AggregateState {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn increment(state: &mut AggregateState, cell: &mut u64) {
        if *cell == u64::MAX {
            state.saturated_cell_increments = state.saturated_cell_increments.saturating_add(1);
        } else {
            *cell += 1;
        }
    }
}

impl ObservationSink for TelemetryCollector {
    fn record_attempt(&self, observation: AttemptObservation) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match observation {
            AttemptObservation::CircuitRejected { class, reason } => {
                let index = class.index() * CIRCUIT_REJECT_REASONS + reason.index();
                let mut cell = state.circuit_rejections[index];
                Self::increment(&mut state, &mut cell);
                state.circuit_rejections[index] = cell;
            }
            AttemptObservation::QueueDeadline { class, queue_wait } => {
                increment_queue(&mut state, class, QueueOutcome::Deadline, queue_wait);
            }
            AttemptObservation::QueueCancelled { class, queue_wait } => {
                increment_queue(&mut state, class, QueueOutcome::Cancelled, queue_wait);
            }
            AttemptObservation::Response {
                class,
                queue_wait,
                outcome,
                response_pipeline_elapsed,
                observed_body_bytes,
            } => {
                increment_queue(&mut state, class, QueueOutcome::Acquired, queue_wait);
                let latency = latency_bucket(response_pipeline_elapsed);
                let series = class.index() * RESPONSE_OUTCOMES + outcome.index();
                let latency_index = series * LATENCY_BUCKETS + latency;
                let mut latency_cell = state.response_latency[latency_index];
                Self::increment(&mut state, &mut latency_cell);
                state.response_latency[latency_index] = latency_cell;
                match observed_body_bytes {
                    BodyBytesObservation::Observed(bytes) => {
                        let byte_index = series * BYTE_BUCKETS + byte_bucket(bytes);
                        let mut byte_cell = state.response_bytes[byte_index];
                        Self::increment(&mut state, &mut byte_cell);
                        state.response_bytes[byte_index] = byte_cell;
                    }
                    BodyBytesObservation::Unavailable => {
                        let mut cell = state.response_bytes_unavailable[series];
                        Self::increment(&mut state, &mut cell);
                        state.response_bytes_unavailable[series] = cell;
                    }
                }
            }
        }
    }
}

fn increment_queue(
    state: &mut AggregateState,
    class: RequestClass,
    outcome: QueueOutcome,
    elapsed: Duration,
) {
    let bucket = latency_bucket(elapsed);
    let index = (class.index() * QUEUE_OUTCOMES + outcome.index()) * LATENCY_BUCKETS + bucket;
    let mut cell = state.queue_latency[index];
    TelemetryCollector::increment(state, &mut cell);
    state.queue_latency[index] = cell;
}

fn latency_bucket(duration: Duration) -> usize {
    let micros = u64::try_from(duration.as_micros()).unwrap_or(u64::MAX);
    LATENCY_UPPER_BOUNDS_MICROS
        .iter()
        .position(|upper| micros <= *upper)
        .unwrap_or(LATENCY_BUCKETS - 1)
}

fn byte_bucket(bytes: u64) -> usize {
    RESPONSE_BYTE_UPPER_BOUNDS
        .iter()
        .position(|upper| bytes <= *upper)
        .unwrap_or(BYTE_BUCKETS - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn histogram_boundaries_are_inclusive_and_queue_semantics_are_explicit() {
        let collector = TelemetryCollector::new();
        for micros in [1_000, 1_001, 30_000_000, 30_000_001] {
            collector.record_attempt(AttemptObservation::Response {
                class: RequestClass::Status,
                queue_wait: Duration::from_micros(micros),
                outcome: ResponseOutcome::Success,
                response_pipeline_elapsed: Duration::ZERO,
                observed_body_bytes: BodyBytesObservation::Observed(0),
            });
        }
        for bytes in [
            0,
            1,
            1_024,
            1_025,
            32 * 1_024 * 1_024,
            32 * 1_024 * 1_024 + 1,
        ] {
            collector.record_attempt(AttemptObservation::Response {
                class: RequestClass::VoucherExport,
                queue_wait: Duration::ZERO,
                outcome: ResponseOutcome::SizeLimit,
                response_pipeline_elapsed: Duration::from_millis(10),
                observed_body_bytes: BodyBytesObservation::Observed(bytes),
            });
        }
        let state = collector.snapshot();
        let queue_base = (RequestClass::Status.index() * QUEUE_OUTCOMES
            + QueueOutcome::Acquired.index())
            * LATENCY_BUCKETS;
        assert_eq!(
            &state.queue_latency[queue_base..queue_base + LATENCY_BUCKETS],
            &[1, 1, 0, 0, 0, 0, 0, 1, 1]
        );
        let response_series = RequestClass::VoucherExport.index() * RESPONSE_OUTCOMES
            + ResponseOutcome::SizeLimit.index();
        let latency_base = response_series * LATENCY_BUCKETS;
        assert_eq!(state.response_latency[latency_base + 2], 6);
        assert_eq!(
            &state.response_bytes
                [response_series * BYTE_BUCKETS..response_series * BYTE_BUCKETS + BYTE_BUCKETS],
            &[1, 2, 1, 0, 0, 1, 1]
        );
    }

    #[test]
    fn snapshot_is_coherent_under_concurrent_recording() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Barrier,
        };

        let collector = Arc::new(TelemetryCollector::new());
        let start = Arc::new(Barrier::new(9));
        let active = Arc::new(AtomicUsize::new(8));
        let threads = (0..8)
            .map(|_| {
                let collector = Arc::clone(&collector);
                let start = Arc::clone(&start);
                let active = Arc::clone(&active);
                thread::spawn(move || {
                    start.wait();
                    for index in 0..20_000 {
                        collector.record_attempt(AttemptObservation::Response {
                            class: RequestClass::MasterExport,
                            queue_wait: Duration::from_millis(1),
                            outcome: ResponseOutcome::Success,
                            response_pipeline_elapsed: Duration::from_millis(5),
                            observed_body_bytes: BodyBytesObservation::Observed(1_024),
                        });
                        if index % 100 == 0 {
                            thread::yield_now();
                        }
                    }
                    active.fetch_sub(1, Ordering::Release);
                })
            })
            .collect::<Vec<_>>();
        start.wait();
        let response_series = RequestClass::MasterExport.index() * RESPONSE_OUTCOMES
            + ResponseOutcome::Success.index();
        let mut concurrent_snapshots = 0_u64;
        while active.load(Ordering::Acquire) > 0 {
            let state = collector.snapshot();
            let latency_total = state.response_latency
                [response_series * LATENCY_BUCKETS..(response_series + 1) * LATENCY_BUCKETS]
                .iter()
                .sum::<u64>();
            let byte_total = state.response_bytes
                [response_series * BYTE_BUCKETS..(response_series + 1) * BYTE_BUCKETS]
                .iter()
                .sum::<u64>();
            assert_eq!(latency_total, byte_total);
            concurrent_snapshots += 1;
            thread::yield_now();
        }
        for thread in threads {
            thread.join().expect("observation thread");
        }
        assert!(concurrent_snapshots > 0);
    }

    #[test]
    fn saturation_never_wraps() {
        let collector = TelemetryCollector::new();
        {
            let mut state = collector.state.lock().unwrap();
            state.queue_latency[0] = u64::MAX;
        }
        collector.record_attempt(AttemptObservation::Response {
            class: RequestClass::Status,
            queue_wait: Duration::ZERO,
            outcome: ResponseOutcome::Success,
            response_pipeline_elapsed: Duration::ZERO,
            observed_body_bytes: BodyBytesObservation::Observed(0),
        });
        let state = collector.snapshot();
        assert_eq!(state.queue_latency[0], u64::MAX);
        assert_eq!(state.saturated_cell_increments, 1);
    }

    #[test]
    fn taxonomy_and_bucket_bounds_are_golden() {
        assert_eq!(
            LATENCY_UPPER_BOUNDS_MICROS,
            [1_000, 5_000, 25_000, 100_000, 500_000, 2_000_000, 10_000_000, 30_000_000,]
        );
        assert_eq!(
            RESPONSE_BYTE_UPPER_BOUNDS,
            [0, 1_024, 65_536, 1_048_576, 8_388_608, 33_554_432]
        );
        assert_eq!(QueueOutcome::ALL.len(), 3);
        assert_eq!(ResponseOutcome::ALL.len(), 10);
        assert_eq!(CircuitRejectReason::ALL.len(), 2);
        assert_eq!(FIXED_HISTOGRAM_CELL_COUNT, 1_592);
    }

    #[test]
    fn terminal_attempt_shape_keeps_queue_failures_out_of_response_histograms() {
        let collector = TelemetryCollector::new();
        collector.record_attempt(AttemptObservation::QueueDeadline {
            class: RequestClass::Capability,
            queue_wait: Duration::from_secs(30),
        });
        collector.record_attempt(AttemptObservation::QueueCancelled {
            class: RequestClass::Capability,
            queue_wait: Duration::from_millis(5),
        });
        collector.record_attempt(AttemptObservation::Response {
            class: RequestClass::Capability,
            queue_wait: Duration::from_millis(1),
            outcome: ResponseOutcome::Timeout,
            response_pipeline_elapsed: Duration::from_secs(10),
            observed_body_bytes: BodyBytesObservation::Unavailable,
        });

        let state = collector.snapshot();
        let queue_base = RequestClass::Capability.index() * QUEUE_OUTCOMES * LATENCY_BUCKETS;
        let queue_total = state.queue_latency
            [queue_base..queue_base + QUEUE_OUTCOMES * LATENCY_BUCKETS]
            .iter()
            .sum::<u64>();
        let response_base = RequestClass::Capability.index() * RESPONSE_OUTCOMES * LATENCY_BUCKETS;
        let response_total = state.response_latency
            [response_base..response_base + RESPONSE_OUTCOMES * LATENCY_BUCKETS]
            .iter()
            .sum::<u64>();
        assert_eq!(queue_total, 3);
        assert_eq!(response_total, 1);
    }

    #[test]
    fn circuit_rejections_and_unavailable_byte_measurement_are_explicit() {
        let collector = TelemetryCollector::new();
        collector.record_attempt(AttemptObservation::CircuitRejected {
            class: RequestClass::CompanyList,
            reason: CircuitRejectReason::Cooldown,
        });
        collector.record_attempt(AttemptObservation::Response {
            class: RequestClass::CompanyList,
            queue_wait: Duration::ZERO,
            outcome: ResponseOutcome::Application,
            response_pipeline_elapsed: Duration::from_millis(2),
            observed_body_bytes: BodyBytesObservation::Unavailable,
        });
        let state = collector.snapshot();
        let circuit = RequestClass::CompanyList.index() * CIRCUIT_REJECT_REASONS
            + CircuitRejectReason::Cooldown.index();
        let response = RequestClass::CompanyList.index() * RESPONSE_OUTCOMES
            + ResponseOutcome::Application.index();
        assert_eq!(state.circuit_rejections[circuit], 1);
        assert_eq!(state.response_bytes_unavailable[response], 1);
    }
}
