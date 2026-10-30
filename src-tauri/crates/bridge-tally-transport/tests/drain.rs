//! The drain after a cut post (#2142): a post that may have left Tally working
//! leaves the endpoint owing a drain, and the next post sends `GET /status`
//! first, posting only once enough probes in a row answered quickly.
//!
//! The gate here keeps the drain record in memory and its clock in a counter
//! that each pause moves on, so spacing is exact; every send goes to the
//! framed simulator, so request counts are the simulator's own.
#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: a local socket bound only to find a port nothing listens on"
)]
use std::sync::{
    atomic::{AtomicU64, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

use bridge_tally_transport::{
    DrainDebt, DrainPolicy, TallyEndpointConfig, TallyHttpTransport, TallyTransportError,
    TallyWireGate, TransportPolicy, WireLockHeld, WirePause, WireRefusal, WireRetryPolicy,
};
use tally_protocol_simulator::{
    Delivery, Fixture, ProductStatus, ScenarioPlan, SequenceSimulator, WireEncoding,
};

/// The clock's start, in milliseconds since the epoch.
const T0: u64 = 1_000_000;

struct Shared {
    debt: Mutex<DrainDebt>,
    /// Every record written, in order.
    history: Mutex<Vec<DrainDebt>>,
    /// A record that cannot be written, as a full disk would leave it.
    unwritable: std::sync::atomic::AtomicBool,
    now: AtomicU64,
    held: AtomicUsize,
    pauses: AtomicUsize,
}

struct Gate(Arc<Shared>);

struct Held(Arc<Shared>);

impl WireLockHeld for Held {
    fn debt(&self) -> Result<DrainDebt, WireRefusal> {
        Ok(*self.0.debt.lock().unwrap())
    }

    fn record(&self, debt: DrainDebt) -> Result<(), WireRefusal> {
        if self.0.unwritable.load(Ordering::SeqCst) {
            return Err(WireRefusal::Unavailable);
        }
        *self.0.debt.lock().unwrap() = debt;
        self.0.history.lock().unwrap().push(debt);
        Ok(())
    }
}

impl Drop for Held {
    fn drop(&mut self) {
        self.0.held.fetch_sub(1, Ordering::SeqCst);
    }
}

impl TallyWireGate for Gate {
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        self.0
            .held
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .map_err(|_| WireRefusal::Busy)?;
        Ok(Box::new(Held(Arc::clone(&self.0))))
    }

    fn pause(&self, delay: Duration) -> WirePause {
        // Never paused holding the lock.
        assert_eq!(
            self.0.held.load(Ordering::SeqCst),
            0,
            "paused holding the lock"
        );
        self.0.pauses.fetch_add(1, Ordering::SeqCst);
        self.0
            .now
            .fetch_add(u64::try_from(delay.as_millis()).unwrap(), Ordering::SeqCst);
        Box::pin(std::future::ready(()))
    }

    fn now_ms(&self) -> u64 {
        self.0.now.load(Ordering::SeqCst)
    }
}

/// The measured policy with times a test can wait out: a probe is quick under
/// 100 ms and cut at 300 ms.
const POLICY: DrainPolicy = DrainPolicy {
    probe_deadline: Duration::from_millis(300),
    quick_below: Duration::from_millis(100),
    ..DrainPolicy::MEASURED
};

fn owed(quick: u8, last_probe_ms: Option<u64>) -> DrainDebt {
    DrainDebt::Owed {
        since_ms: T0,
        quick,
        last_probe_ms,
    }
}

fn transport(
    simulator: &SequenceSimulator,
    debt: DrainDebt,
    drain: Option<DrainPolicy>,
) -> (TallyHttpTransport, Arc<Shared>) {
    transport_limited(simulator, debt, drain, Duration::from_millis(500))
}

/// As [`transport`], with `limit` as every request's own deadline.
fn transport_limited(
    simulator: &SequenceSimulator,
    debt: DrainDebt,
    drain: Option<DrainPolicy>,
    limit: Duration,
) -> (TallyHttpTransport, Arc<Shared>) {
    let shared = Arc::new(Shared {
        debt: Mutex::new(debt),
        history: Mutex::new(Vec::new()),
        unwritable: std::sync::atomic::AtomicBool::new(false),
        now: AtomicU64::new(T0),
        held: AtomicUsize::new(0),
        pauses: AtomicUsize::new(0),
    });
    let mut transport = TallyHttpTransport::with_policy(
        TallyEndpointConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
        TransportPolicy {
            request_timeout: limit,
            ..TransportPolicy::default()
        },
    )
    .unwrap()
    .with_wire_gate(
        Arc::new(Gate(Arc::clone(&shared))),
        WireRetryPolicy::DEFAULT,
    );
    if let Some(policy) = drain {
        transport = transport.with_drain(policy);
    }
    (transport, shared)
}

fn xml() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ExportStatusOne).with_encoding(WireEncoding::Utf16Le)
}

fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}

fn slow_status(delay: Duration) -> ScenarioPlan {
    status().with_delivery(Delivery::SlowHeaders(delay))
}

/// The paths the simulator was sent, in order.
fn sent(simulator: SequenceSimulator) -> Vec<String> {
    simulator.cancel();
    simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .map(|request| request.path)
        .collect()
}

fn history(shared: &Shared) -> Vec<DrainDebt> {
    shared.history.lock().unwrap().clone()
}

fn debt(shared: &Shared) -> DrainDebt {
    *shared.debt.lock().unwrap()
}

fn draining(error: &TallyTransportError) -> u64 {
    match error {
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Draining { retry_after_ms },
        } => *retry_after_ms,
        other => panic!("not a draining refusal: {other:?}"),
    }
}

#[test]
fn the_measured_policy_is_pinned() {
    let measured = DrainPolicy::MEASURED;
    assert_eq!(measured.probe_deadline, Duration::from_secs(5));
    assert_eq!(measured.quick_below, Duration::from_secs(1));
    assert_eq!(measured.quick_in_a_row, 3);
    assert_eq!(measured.spacing, Duration::from_secs(2));
    assert_eq!(measured.probes_per_call, 3);
    assert_eq!(measured.attention_after, Duration::from_secs(120));
    assert_eq!(
        WireRefusal::Draining { retry_after_ms: 1 }.safe_code(),
        "tally_endpoint_draining"
    );
    assert_eq!(
        WireRefusal::NeedsAttention { retry_after_ms: 1 }.safe_code(),
        "tally_endpoint_needs_attention"
    );
}

#[tokio::test]
async fn a_post_cut_for_time_leaves_the_drain_owed_from_when_it_was_sent() {
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(Delivery::SlowHeaders(Duration::from_millis(900)))
    ])
    .unwrap();
    let (transport, shared) = transport(&simulator, DrainDebt::Clear, Some(POLICY));
    let error = transport
        .post_xml_decoded("<E />".into())
        .await
        .unwrap_err();
    assert_eq!(error, TallyTransportError::RequestTimedOut);
    assert_eq!(debt(&shared), owed(0, None));
    assert_eq!(sent(simulator), ["/"]);
}

#[tokio::test]
async fn an_answered_post_clears_a_record_left_in_flight_by_no_one_after_three_quick_probes() {
    let simulator = SequenceSimulator::spawn(vec![status(), status(), status(), xml()]).unwrap();
    let (transport, shared) = transport(
        &simulator,
        DrainDebt::InFlight { since_ms: T0 },
        Some(POLICY),
    );
    transport.post_xml_decoded("<E />".into()).await.unwrap();
    assert_eq!(debt(&shared), DrainDebt::Clear);
    assert_eq!(sent(simulator), ["/status", "/status", "/status", "/"]);
    // Two pauses of the spacing, each with the lock released.
    assert_eq!(shared.pauses.load(Ordering::SeqCst), 2);
    assert_eq!(shared.now.load(Ordering::SeqCst), T0 + 2 * 2_000);
}

#[tokio::test]
async fn a_slow_probe_refuses_the_post_at_once_and_resets_the_count() {
    let simulator =
        SequenceSimulator::spawn(vec![status(), slow_status(Duration::from_millis(150))]).unwrap();
    let (transport, shared) = transport(&simulator, owed(1, None), Some(POLICY));
    let error = transport
        .post_xml_decoded("<E />".into())
        .await
        .unwrap_err();
    assert_eq!(draining(&error), 10_000);
    assert_eq!(debt(&shared), owed(0, Some(T0 + 2_000)));
    assert_eq!(sent(simulator), ["/status", "/status"]);
}

#[tokio::test]
async fn a_probe_cut_by_its_own_deadline_refuses_the_post() {
    let simulator =
        SequenceSimulator::spawn(vec![slow_status(Duration::from_millis(900))]).unwrap();
    let (transport, shared) = transport(&simulator, owed(2, None), Some(POLICY));
    let error = transport
        .post_xml_decoded("<E />".into())
        .await
        .unwrap_err();
    assert_eq!(draining(&error), 10_000);
    assert_eq!(debt(&shared), owed(0, Some(T0)));
    assert_eq!(sent(simulator), ["/status"]);
}

/// The probe is cut by its own deadline, not the request limit: with a limit far
/// above it, the call is refused within about the probe's deadline.
#[tokio::test]
async fn a_probe_is_cut_at_its_own_deadline_not_at_the_request_limit() {
    let simulator = SequenceSimulator::spawn(vec![slow_status(Duration::from_secs(3))]).unwrap();
    let (transport, _shared) = transport_limited(
        &simulator,
        owed(0, None),
        Some(POLICY),
        Duration::from_secs(10),
    );
    let started = std::time::Instant::now();
    let error = transport
        .post_xml_decoded("<E />".into())
        .await
        .unwrap_err();
    assert_eq!(draining(&error), 10_000);
    assert!(
        started.elapsed() < Duration::from_millis(1_500),
        "{:?}",
        started.elapsed()
    );
    let _ = sent(simulator);
}

/// A probe's time is recorded before it is sent, so a sender that is dropped
/// while its probe waits still spaces the next probe of any process.
#[tokio::test]
async fn a_probe_is_recorded_before_it_is_sent() {
    let simulator =
        SequenceSimulator::spawn(vec![slow_status(Duration::from_millis(400))]).unwrap();
    let (transport, shared) = transport(&simulator, owed(1, None), Some(POLICY));
    let post = transport.post_xml_decoded("<E />".into());
    assert!(tokio::time::timeout(Duration::from_millis(100), post)
        .await
        .is_err());
    assert_eq!(debt(&shared), owed(1, Some(T0)));
    assert_eq!(shared.held.load(Ordering::SeqCst), 0);
    let _ = sent(simulator);
}

#[tokio::test]
async fn a_post_within_the_spacing_of_the_last_probe_sends_nothing() {
    let simulator = SequenceSimulator::spawn(vec![status()]).unwrap();
    let (transport, shared) = transport(&simulator, owed(1, Some(T0 - 500)), Some(POLICY));
    let error = transport
        .post_xml_decoded("<E />".into())
        .await
        .unwrap_err();
    assert_eq!(draining(&error), 1_500);
    assert_eq!(debt(&shared), owed(1, Some(T0 - 500)));
    assert!(sent(simulator).is_empty());
}

#[tokio::test]
async fn a_last_probe_ahead_of_the_clock_counts_as_elapsed() {
    let simulator = SequenceSimulator::spawn(vec![status(), xml()]).unwrap();
    let (transport, shared) = transport(&simulator, owed(2, Some(T0 + 60_000)), Some(POLICY));
    transport.post_xml_decoded("<E />".into()).await.unwrap();
    assert_eq!(debt(&shared), DrainDebt::Clear);
    assert_eq!(sent(simulator), ["/status", "/"]);
}

#[tokio::test]
async fn a_drain_owed_past_the_attention_time_is_refused_as_needing_attention() {
    let simulator =
        SequenceSimulator::spawn(vec![slow_status(Duration::from_millis(150))]).unwrap();
    let (transport, shared) = transport(&simulator, owed(0, None), Some(POLICY));
    shared.now.store(T0 + 121_000, Ordering::SeqCst);
    let error = transport
        .post_xml_decoded("<E />".into())
        .await
        .unwrap_err();
    assert!(
        matches!(
            error,
            TallyTransportError::WireRefused {
                refusal: WireRefusal::NeedsAttention { .. }
            }
        ),
        "{error:?}"
    );
    assert_eq!(error.safe_code(), "tally_endpoint_needs_attention");
    assert_eq!(sent(simulator), ["/status"]);
}

#[tokio::test]
async fn a_post_that_never_reached_tally_leaves_the_record_clear() {
    // A port with nothing listening: the connection is refused.
    let unused = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = unused.local_addr().unwrap().port();
    drop(unused);
    let shared = Arc::new(Shared {
        debt: Mutex::new(DrainDebt::Clear),
        history: Mutex::new(Vec::new()),
        unwritable: std::sync::atomic::AtomicBool::new(false),
        now: AtomicU64::new(T0),
        held: AtomicUsize::new(0),
        pauses: AtomicUsize::new(0),
    });
    let transport = TallyHttpTransport::new(TallyEndpointConfig {
        host: "127.0.0.1".into(),
        port,
    })
    .unwrap()
    .with_wire_gate(
        Arc::new(Gate(Arc::clone(&shared))),
        WireRetryPolicy::DEFAULT,
    )
    .with_drain(POLICY);
    let error = transport
        .post_xml_decoded("<E />".into())
        .await
        .unwrap_err();
    assert_eq!(error, TallyTransportError::ConnectionFailed);
    assert_eq!(debt(&shared), DrainDebt::Clear);
    assert_eq!(
        history(&shared),
        [DrainDebt::InFlight { since_ms: T0 }, DrainDebt::Clear]
    );
    // Owed, with nothing listening: the probe cannot connect, so no work can
    // remain. The debt is cleared and the post fails as unreachable, never as
    // draining.
    *shared.debt.lock().unwrap() = owed(0, None);
    let error = transport
        .post_xml_decoded("<E />".into())
        .await
        .unwrap_err();
    assert_eq!(error, TallyTransportError::ConnectionFailed);
    assert_eq!(debt(&shared), DrainDebt::Clear);
}

#[tokio::test]
async fn the_raw_post_arms_and_owes_like_the_decoded_one() {
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(Delivery::SlowHeaders(Duration::from_millis(900)))
    ])
    .unwrap();
    let (transport, shared) = transport(&simulator, DrainDebt::Clear, Some(POLICY));
    let error = transport.post_xml("<E />".into()).await.unwrap_err();
    assert_eq!(error, TallyTransportError::RequestTimedOut);
    assert_eq!(
        history(&shared),
        [DrainDebt::InFlight { since_ms: T0 }, owed(0, None)]
    );
    let _ = sent(simulator);
}

#[tokio::test]
async fn an_unwritable_record_refuses_the_import_lock_before_its_attempt() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (transport, shared) = transport(&simulator, DrainDebt::Clear, Some(POLICY));
    shared.unwritable.store(true, Ordering::SeqCst);
    let error = transport.acquire_wire_lock().await.err().unwrap();
    assert_eq!(
        error,
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Unavailable
        }
    );
    assert_eq!(shared.held.load(Ordering::SeqCst), 0);
    assert!(sent(simulator).is_empty());
}

#[tokio::test]
async fn the_import_path_refuses_an_owed_drain_at_once_with_no_probe() {
    let simulator = SequenceSimulator::spawn(vec![status(), xml()]).unwrap();
    let (transport, shared) = transport(&simulator, owed(0, None), Some(POLICY));
    let error = transport.acquire_wire_lock().await.err().unwrap();
    assert_eq!(draining(&error), 2_000);
    assert_eq!(debt(&shared), owed(0, None));
    assert_eq!(shared.held.load(Ordering::SeqCst), 0);
    assert!(sent(simulator).is_empty());
}

#[tokio::test]
async fn the_import_path_arms_and_clears_its_one_post() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (transport, shared) = transport(&simulator, DrainDebt::Clear, Some(POLICY));
    let held = transport.acquire_wire_lock().await.unwrap();
    // Armed when the lock is taken, before the caller records its attempt.
    assert_eq!(debt(&shared), DrainDebt::InFlight { since_ms: T0 });
    held.post_xml_decoded("<E />".into()).await.unwrap();
    assert_eq!(debt(&shared), DrainDebt::Clear);
    assert_eq!(
        history(&shared),
        [DrainDebt::InFlight { since_ms: T0 }, DrainDebt::Clear]
    );
    assert_eq!(sent(simulator), ["/"]);
}

#[tokio::test]
async fn a_post_abandoned_mid_send_leaves_the_record_in_flight() {
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(Delivery::SlowHeaders(Duration::from_millis(400)))
    ])
    .unwrap();
    let (transport, shared) = transport(&simulator, DrainDebt::Clear, Some(POLICY));
    let post = transport.post_xml_decoded("<E />".into());
    let cut = tokio::time::timeout(Duration::from_millis(100), post).await;
    assert!(cut.is_err(), "the send was abandoned");
    assert_eq!(debt(&shared), DrainDebt::InFlight { since_ms: T0 });
    assert_eq!(shared.held.load(Ordering::SeqCst), 0);
    let _ = sent(simulator);
}

#[tokio::test]
async fn a_status_send_never_reads_or_moves_the_record() {
    let simulator = SequenceSimulator::spawn(vec![status()]).unwrap();
    let (transport, shared) = transport(&simulator, owed(1, Some(T0)), Some(POLICY));
    transport.get_status_decoded().await.unwrap();
    assert_eq!(debt(&shared), owed(1, Some(T0)));
    assert_eq!(sent(simulator), ["/status"]);
}

#[tokio::test]
async fn without_a_drain_policy_a_post_ignores_the_record() {
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(Delivery::SlowHeaders(Duration::from_millis(900))),
        xml(),
    ])
    .unwrap();
    let (transport, shared) = transport(&simulator, owed(0, None), None);
    assert_eq!(
        transport
            .post_xml_decoded("<E />".into())
            .await
            .unwrap_err(),
        TallyTransportError::RequestTimedOut
    );
    transport.post_xml_decoded("<E />".into()).await.unwrap();
    assert_eq!(debt(&shared), owed(0, None));
    assert_eq!(sent(simulator), ["/", "/"]);
}
