//! The injected wire gate (#697 PR A): taken around exactly one send, never held
//! while waiting, retried within one operation's budget, and a refused send
//! sends nothing.
//!
//! The gate here is a recording stand-in with the real lock's one property that
//! matters to this crate: a second holder is refused at once. The file lock the
//! application injects is tested against another process in the application.
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

use bridge_tally_transport::{
    TallyEndpointConfig, TallyHttpTransport, TallyTransportError, TallyWireGate, WireLockHeld,
    WirePause, WireRefusal, WireRetryPolicy, WireWaitBudget, WIRE_WAIT_MAX,
};
use tally_protocol_simulator::{
    Delivery, Fixture, ProductStatus, ScenarioPlan, SequenceSimulator, WireEncoding,
};

#[derive(Default)]
struct Record {
    held: AtomicUsize,
    attempts: AtomicUsize,
    acquisitions: AtomicUsize,
    pauses: AtomicUsize,
    paused_for: Mutex<Vec<Duration>>,
    busy_first: AtomicUsize,
    always: Mutex<Option<WireRefusal>>,
}

struct RecordingGate(Arc<Record>);

struct RecordedLock(Arc<Record>);

impl WireLockHeld for RecordedLock {}

impl Drop for RecordedLock {
    fn drop(&mut self) {
        self.0.held.fetch_sub(1, Ordering::SeqCst);
    }
}

impl TallyWireGate for RecordingGate {
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        self.0.attempts.fetch_add(1, Ordering::SeqCst);
        if let Some(refusal) = *self.0.always.lock().unwrap() {
            return Err(refusal);
        }
        if self
            .0
            .busy_first
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |left| {
                left.checked_sub(1)
            })
            .is_ok()
        {
            return Err(WireRefusal::Busy);
        }
        // Exclusive, as the real lock is: a second holder is refused at once.
        if self
            .0
            .held
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(WireRefusal::Busy);
        }
        self.0.acquisitions.fetch_add(1, Ordering::SeqCst);
        Ok(Box::new(RecordedLock(Arc::clone(&self.0))))
    }

    fn pause(&self, delay: Duration) -> WirePause {
        self.0.pauses.fetch_add(1, Ordering::SeqCst);
        self.0.paused_for.lock().unwrap().push(delay);
        Box::pin(tokio::time::sleep(delay))
    }
}

fn gated(
    simulator: &SequenceSimulator,
    retry: WireRetryPolicy,
) -> (TallyHttpTransport, Arc<Record>) {
    let record = Arc::new(Record::default());
    let transport = TallyHttpTransport::new(TallyEndpointConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    })
    .unwrap()
    .with_wire_gate(Arc::new(RecordingGate(Arc::clone(&record))), retry);
    (transport, record)
}

/// Pauses of 1 ms, with a budget of `pauses` of them per operation.
fn quick(pauses: u32) -> WireRetryPolicy {
    WireRetryPolicy::new(
        Duration::from_millis(1),
        Duration::from_millis(pauses.into()),
    )
    .unwrap()
}

fn xml() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ExportStatusOne).with_encoding(WireEncoding::Utf16Le)
}

fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}

#[test]
fn the_retry_bound_never_exceeds_ten_seconds() {
    assert!(WireRetryPolicy::DEFAULT.total() <= WIRE_WAIT_MAX);
    assert_eq!(WIRE_WAIT_MAX, Duration::from_secs(10));
    // The shipped values, named in the docs and the tool descriptions.
    assert_eq!(WireRetryPolicy::DEFAULT.delay(), Duration::from_millis(500));
    assert_eq!(
        WireRetryPolicy::DEFAULT.total(),
        Duration::from_millis(9_500)
    );
    let half = Duration::from_millis(500);
    assert!(WireRetryPolicy::new(half, WIRE_WAIT_MAX).is_some());
    assert!(WireRetryPolicy::new(half, WIRE_WAIT_MAX + Duration::from_nanos(1)).is_none());
    // A zero pause would never use the budget up.
    assert!(WireRetryPolicy::new(Duration::ZERO, half).is_none());
    assert!(WireRetryPolicy::new(Duration::MAX, Duration::MAX).is_none());
    // A zero budget tries once and never waits.
    assert!(WireRetryPolicy::new(half, Duration::ZERO).is_some());
}

/// The invariant, as this crate can see it: every send takes the lock once,
/// releases it before returning, success or failure, and never pauses while
/// holding it.
#[tokio::test]
async fn each_send_holds_the_lock_for_that_send_only() {
    let simulator =
        SequenceSimulator::spawn(vec![xml(), status(), xml(), xml().with_http_status(500)])
            .unwrap();
    let (transport, record) = gated(&simulator, quick(2));
    transport.post_xml("<ENVELOPE/>".into()).await.unwrap();
    assert_eq!(record.held.load(Ordering::SeqCst), 0);
    transport.get_status_decoded().await.unwrap();
    assert_eq!(record.held.load(Ordering::SeqCst), 0);
    transport
        .post_xml_decoded("<ENVELOPE/>".into())
        .await
        .unwrap();
    assert_eq!(record.held.load(Ordering::SeqCst), 0);
    let failed = transport.post_xml_decoded("<ENVELOPE/>".into()).await;
    assert_eq!(
        failed.unwrap_err(),
        TallyTransportError::HttpStatus { status: 500 }
    );
    // Released on failure too.
    assert_eq!(record.held.load(Ordering::SeqCst), 0);
    assert_eq!(record.acquisitions.load(Ordering::SeqCst), 4);
    assert_eq!(record.pauses.load(Ordering::SeqCst), 0);
    assert_eq!(simulator.finish().unwrap().len(), 4);
}

#[tokio::test]
async fn a_busy_lock_is_tried_again_and_the_send_then_goes_through() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (transport, record) = gated(&simulator, quick(4));
    record.busy_first.store(3, Ordering::SeqCst);
    transport.post_xml("<ENVELOPE/>".into()).await.unwrap();
    assert_eq!(record.pauses.load(Ordering::SeqCst), 3);
    // The pausing caller held nothing: a refused attempt takes nothing.
    assert_eq!(record.acquisitions.load(Ordering::SeqCst), 1);
    assert_eq!(simulator.finish().unwrap().len(), 1);
}

#[tokio::test]
async fn a_lock_busy_past_the_bound_refuses_and_sends_nothing() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (transport, record) = gated(&simulator, quick(2));
    *record.always.lock().unwrap() = Some(WireRefusal::Busy);
    let refused = transport.post_xml_decoded("<ENVELOPE/>".into()).await;
    assert_eq!(
        refused.unwrap_err(),
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Busy
        }
    );
    // Three attempts, two pauses between them, then the refusal.
    assert_eq!(record.attempts.load(Ordering::SeqCst), 3);
    assert_eq!(record.pauses.load(Ordering::SeqCst), 2);
    assert_eq!(simulator.received(), 0);
    assert_eq!(
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Busy
        }
        .safe_code(),
        "tally_endpoint_busy"
    );
}

#[tokio::test]
async fn an_unusable_lock_is_not_waited_for() {
    let simulator = SequenceSimulator::spawn(vec![status()]).unwrap();
    let (transport, record) = gated(&simulator, quick(5));
    *record.always.lock().unwrap() = Some(WireRefusal::Unavailable);
    let refused = transport.get_status_decoded().await.unwrap_err();
    assert_eq!(
        refused,
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Unavailable
        }
    );
    assert_eq!(refused.safe_code(), "tally_endpoint_lock_unavailable");
    assert_eq!(record.pauses.load(Ordering::SeqCst), 0);
    assert_eq!(simulator.received(), 0);
}

/// Two sends at once through one gate: the second waits out the first, and the
/// lock never has two holders.
#[tokio::test]
async fn concurrent_sends_take_turns() {
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(Delivery::SlowHeaders(Duration::from_millis(300))),
        xml(),
    ])
    .unwrap();
    let (transport, record) = gated(
        &simulator,
        WireRetryPolicy::new(Duration::from_millis(10), Duration::from_secs(2)).unwrap(),
    );
    let first = transport.post_xml("<ENVELOPE/>".into());
    let second = async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        transport.post_xml("<ENVELOPE/>".into()).await
    };
    let (first, second) = tokio::join!(first, second);
    first.unwrap();
    second.unwrap();
    assert_eq!(record.acquisitions.load(Ordering::SeqCst), 2);
    assert!(record.pauses.load(Ordering::SeqCst) >= 1);
    assert_eq!(simulator.finish().unwrap().len(), 2);
}

/// While a send is in flight, the lock is held: each of the four send paths,
/// probed from outside with the response delayed, so that dropping the guard
/// before the send (a `let _ = ...` in place of a bound guard) fails here.
#[tokio::test]
async fn every_send_path_holds_the_lock_while_the_send_is_in_flight() {
    let slow = Duration::from_millis(250);
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(Delivery::SlowHeaders(slow)),
        xml().with_delivery(Delivery::SlowHeaders(slow)),
        status().with_delivery(Delivery::SlowHeaders(slow)),
        status().with_delivery(Delivery::SlowHeaders(slow)),
    ])
    .unwrap();
    let (transport, record) = gated(&simulator, quick(0));
    let probe = |record: &Arc<Record>| {
        let record = Arc::clone(record);
        async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            record.held.load(Ordering::SeqCst)
        }
    };
    let (sent, held) = tokio::join!(transport.post_xml("<ENVELOPE/>".into()), probe(&record));
    sent.unwrap();
    assert_eq!(held, 1, "post_xml");
    let (sent, held) = tokio::join!(
        transport.post_xml_decoded("<ENVELOPE/>".into()),
        probe(&record)
    );
    sent.unwrap();
    assert_eq!(held, 1, "post_xml_decoded");
    let (sent, held) = tokio::join!(transport.get_status(), probe(&record));
    sent.unwrap();
    assert_eq!(held, 1, "get_status");
    let (sent, held) = tokio::join!(transport.get_status_decoded(), probe(&record));
    sent.unwrap();
    assert_eq!(held, 1, "get_status_decoded");
    assert_eq!(record.held.load(Ordering::SeqCst), 0);
    assert_eq!(simulator.finish().unwrap().len(), 4);
}

/// The lock is held until the body has been read, not only until the headers
/// arrive: the headers here come at once and the body in slow chunks, so a
/// guard dropped once the head is in fails at the probe.
#[tokio::test]
async fn every_send_path_holds_the_lock_until_the_body_is_read() {
    let slow_xml = || {
        xml().with_delivery(Delivery::SlowBody {
            chunk_bytes: 32,
            delay: Duration::from_millis(40),
        })
    };
    let slow_status = || {
        status().with_delivery(Delivery::SlowBody {
            chunk_bytes: 8,
            delay: Duration::from_millis(60),
        })
    };
    let simulator =
        SequenceSimulator::spawn(vec![slow_xml(), slow_xml(), slow_status(), slow_status()])
            .unwrap();
    let (transport, record) = gated(&simulator, quick(0));
    let probe = |record: &Arc<Record>| {
        let record = Arc::clone(record);
        async move {
            tokio::time::sleep(Duration::from_millis(120)).await;
            record.held.load(Ordering::SeqCst)
        }
    };
    let (sent, held) = tokio::join!(transport.post_xml("<ENVELOPE/>".into()), probe(&record));
    sent.unwrap();
    assert_eq!(held, 1, "post_xml");
    let (sent, held) = tokio::join!(
        transport.post_xml_decoded("<ENVELOPE/>".into()),
        probe(&record)
    );
    sent.unwrap();
    assert_eq!(held, 1, "post_xml_decoded");
    let (sent, held) = tokio::join!(transport.get_status(), probe(&record));
    sent.unwrap();
    assert_eq!(held, 1, "get_status");
    let (sent, held) = tokio::join!(transport.get_status_decoded(), probe(&record));
    sent.unwrap();
    assert_eq!(held, 1, "get_status_decoded");
    assert_eq!(record.held.load(Ordering::SeqCst), 0);
    assert_eq!(simulator.finish().unwrap().len(), 4);
}

/// A lock taken ahead of a send is held until that one send has been read,
/// refuses every other send meanwhile, and is released by it.
#[tokio::test]
async fn a_held_lock_is_spent_on_one_send() {
    let simulator = SequenceSimulator::spawn(vec![xml(), status()]).unwrap();
    let (transport, record) = gated(&simulator, quick(0));
    let held = transport.acquire_wire_lock().await.unwrap();
    assert_eq!(record.held.load(Ordering::SeqCst), 1);
    // While it is held, a second send through the same gate is refused.
    assert_eq!(
        transport.get_status_decoded().await.unwrap_err(),
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Busy
        }
    );
    assert_eq!(simulator.received(), 0);
    let response = held.post_xml_decoded("<ENVELOPE/>".into()).await.unwrap();
    assert!(response.request_body_sha256().is_some());
    assert_eq!(record.held.load(Ordering::SeqCst), 0);
    // Released: the next send takes it again.
    transport.get_status_decoded().await.unwrap();
    assert_eq!(record.acquisitions.load(Ordering::SeqCst), 2);
    assert_eq!(simulator.finish().unwrap().len(), 2);
}

/// A lock taken ahead of a send stays held while that send is in flight: the
/// response is delayed, and a probe from outside sees the lock still taken, so
/// a guard dropped before the send fails here.
#[tokio::test]
async fn a_lock_taken_ahead_of_a_send_is_held_while_that_send_is_in_flight() {
    let slow = Duration::from_millis(250);
    let simulator =
        SequenceSimulator::spawn(vec![xml().with_delivery(Delivery::SlowHeaders(slow))]).unwrap();
    let (transport, record) = gated(&simulator, quick(0));
    let held = transport.acquire_wire_lock().await.unwrap();
    let probe = async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        record.held.load(Ordering::SeqCst)
    };
    let (sent, during) = tokio::join!(held.post_xml_decoded("<ENVELOPE/>".into()), probe);
    sent.unwrap();
    assert_eq!(during, 1);
    assert_eq!(record.held.load(Ordering::SeqCst), 0);
    assert_eq!(simulator.finish().unwrap().len(), 1);
}

/// The last pause of a budget is clipped to what is left, so the pauses of an
/// operation add up to its budget and no more, even when the pause length does
/// not divide it.
#[tokio::test]
async fn the_last_pause_is_clipped_to_what_the_budget_has_left() {
    let simulator = SequenceSimulator::spawn(vec![status()]).unwrap();
    let policy = WireRetryPolicy::new(Duration::from_millis(2), Duration::from_millis(5)).unwrap();
    let (transport, record) = gated(&simulator, policy);
    *record.always.lock().unwrap() = Some(WireRefusal::Busy);
    assert!(transport.get_status_decoded().await.is_err());
    let paused = record.paused_for.lock().unwrap().clone();
    assert_eq!(
        paused,
        vec![
            Duration::from_millis(2),
            Duration::from_millis(2),
            Duration::from_millis(1)
        ]
    );
}

/// A budget asked for more than the ceiling gets the ceiling, however large.
#[test]
fn a_wait_budget_is_capped_at_the_ceiling() {
    assert_eq!(
        WireWaitBudget::new(WIRE_WAIT_MAX + Duration::from_secs(50)).remaining(),
        WIRE_WAIT_MAX
    );
    assert_eq!(
        WireWaitBudget::new(Duration::MAX).remaining(),
        WIRE_WAIT_MAX
    );
}

/// Taking the lock ahead of a send is one try: a taken lock is refused at once,
/// without a pause and without drawing on the operation's budget, so no wait
/// can land between a caller's own checks (the import POST's aim recheck and
/// its attempt record).
#[tokio::test]
async fn taking_the_lock_ahead_of_a_send_never_waits() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (transport, record) = gated(&simulator, quick(50));
    let operation = transport.for_operation(WireWaitBudget::new(Duration::from_millis(50)));
    record.busy_first.store(1, Ordering::SeqCst);
    let refused = operation.acquire_wire_lock().await.err().unwrap();
    assert_eq!(
        refused,
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Busy
        }
    );
    assert_eq!(record.pauses.load(Ordering::SeqCst), 0);
    assert_eq!(
        operation.wire_budget_remaining(),
        Some(Duration::from_millis(50))
    );
    assert_eq!(simulator.received(), 0);
}

/// #697 item (a): the budget is the operation's, not the send's. Two sends of
/// one operation, the second through a clone as the runtime's retries use,
/// draw on one budget; a new operation starts with a full one.
#[tokio::test]
async fn two_sends_in_one_operation_share_one_wait_budget() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (transport, record) = gated(&simulator, quick(3));
    let budget = || WireWaitBudget::new(Duration::from_millis(3));
    let operation = transport.for_operation(budget());
    assert_eq!(
        operation.wire_budget_remaining(),
        Some(Duration::from_millis(3))
    );
    // The first send waits out two of the operation's three pauses.
    record.busy_first.store(2, Ordering::SeqCst);
    operation.post_xml("<ENVELOPE/>".into()).await.unwrap();
    assert_eq!(record.pauses.load(Ordering::SeqCst), 2);
    assert_eq!(
        operation.wire_budget_remaining(),
        Some(Duration::from_millis(1))
    );
    // The second send has one pause left, not a fresh three.
    *record.always.lock().unwrap() = Some(WireRefusal::Busy);
    assert_eq!(
        operation.clone().get_status_decoded().await.unwrap_err(),
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Busy
        }
    );
    assert_eq!(record.pauses.load(Ordering::SeqCst), 3);
    assert_eq!(operation.wire_budget_remaining(), Some(Duration::ZERO));
    // Spent: a third send of the operation is refused without pausing.
    assert!(operation.get_status_decoded().await.is_err());
    assert_eq!(record.pauses.load(Ordering::SeqCst), 3);
    // Another operation has its own full budget.
    assert!(transport
        .for_operation(budget())
        .get_status_decoded()
        .await
        .is_err());
    assert_eq!(record.pauses.load(Ordering::SeqCst), 6);
    // Two transports given one budget share it, as the runtime's operations
    // within one tool call do.
    let shared = budget();
    assert!(transport
        .for_operation(shared.clone())
        .get_status_decoded()
        .await
        .is_err());
    assert_eq!(record.pauses.load(Ordering::SeqCst), 9);
    assert!(transport
        .for_operation(shared)
        .get_status_decoded()
        .await
        .is_err());
    assert_eq!(record.pauses.load(Ordering::SeqCst), 9);
    // An unscoped transport has no operation budget.
    assert_eq!(transport.wire_budget_remaining(), None);
    assert_eq!(simulator.finish().unwrap().len(), 1);
}
