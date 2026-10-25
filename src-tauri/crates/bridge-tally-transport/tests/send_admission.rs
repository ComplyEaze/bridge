//! #778: a transport given a send admission starts a send only while it
//! admits it, asked before the wire lock is taken and again once it is held.
//! A refused send fails as `SendWithdrawn`, sends nothing, and is recorded.
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bridge_tally_transport::{
    SendAdmission, SendKind, SendObserver, SendRecord, TallyEndpointConfig, TallyHttpTransport,
    TallyTransportError, TallyWireGate, WireLockHeld, WirePause, WireRefusal, WireRetryPolicy,
};
use tally_protocol_simulator::{Fixture, ScenarioPlan, SequenceSimulator, WireEncoding};

#[derive(Default)]
struct Collect(Mutex<Vec<SendRecord>>);

impl SendObserver for Collect {
    fn observe(&self, record: SendRecord) {
        self.0.lock().unwrap().push(record);
    }
}

/// Admits a send until `withdrawn` is set.
struct Admission(Arc<AtomicBool>);

impl SendAdmission for Admission {
    fn admits(&self) -> bool {
        !self.0.load(Ordering::SeqCst)
    }
}

fn xml() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ExportStatusOne).with_encoding(WireEncoding::Utf16Le)
}

fn transport(simulator: &SequenceSimulator) -> (TallyHttpTransport, Arc<Collect>) {
    let collect = Arc::new(Collect::default());
    let transport = TallyHttpTransport::new(TallyEndpointConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    })
    .unwrap()
    .with_send_observer(collect.clone());
    (transport, collect)
}

#[tokio::test]
async fn a_withdrawn_send_is_refused_before_connecting_and_recorded() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (plain, collect) = transport(&simulator);
    let withdrawn = Arc::new(AtomicBool::new(false));
    let gated = plain.with_send_admission(Arc::new(Admission(withdrawn.clone())));
    // Admitted: the send goes out as any other.
    gated.post_xml_decoded("<ENVELOPE/>".into()).await.unwrap();
    let request_bytes = collect.0.lock().unwrap()[0].request_bytes;
    withdrawn.store(true, Ordering::SeqCst);
    for sender in [gated.clone(), gated] {
        assert_eq!(
            sender.post_xml_decoded("<ENVELOPE/>".into()).await.err(),
            Some(TallyTransportError::SendWithdrawn)
        );
    }
    assert_eq!(
        TallyTransportError::SendWithdrawn.safe_code(),
        "request_cancelled"
    );
    let records = collect.0.lock().unwrap().clone();
    let refused = SendRecord {
        kind: SendKind::Post,
        request_bytes,
        outcome: "request_cancelled",
        response_bytes: None,
        held_ms: 0,
    };
    assert_eq!(records[1..], [refused.clone(), refused]);
    // The plain transport it was cloned from is not withdrawn.
    assert_eq!(simulator.finish().unwrap().len(), 1);
}

/// Refuses the lock once and pauses before the second try; the pause sets
/// `withdrawn`, as a withdrawal landing during the lock wait would.
struct WithdrawnWhileWaiting {
    tries: AtomicUsize,
    withdrawn: Arc<AtomicBool>,
}

struct Held;
impl WireLockHeld for Held {}

impl TallyWireGate for WithdrawnWhileWaiting {
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        if self.tries.fetch_add(1, Ordering::SeqCst) == 0 {
            Err(WireRefusal::Busy)
        } else {
            Ok(Box::new(Held))
        }
    }

    fn pause(&self, _delay: Duration) -> WirePause {
        self.withdrawn.store(true, Ordering::SeqCst);
        Box::pin(async {})
    }
}

#[tokio::test]
async fn a_withdrawal_during_the_lock_wait_starts_no_send() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (plain, collect) = transport(&simulator);
    let withdrawn = Arc::new(AtomicBool::new(false));
    let gate = Arc::new(WithdrawnWhileWaiting {
        tries: AtomicUsize::new(0),
        withdrawn: withdrawn.clone(),
    });
    let gated = plain
        .with_wire_gate(
            gate.clone(),
            WireRetryPolicy::new(Duration::from_millis(1), Duration::from_secs(1)).unwrap(),
        )
        .with_send_admission(Arc::new(Admission(withdrawn)));
    assert_eq!(
        gated.get_status_decoded().await.err(),
        Some(TallyTransportError::SendWithdrawn)
    );
    // The lock was taken on the second try, then released unused.
    assert_eq!(gate.tries.load(Ordering::SeqCst), 2);
    assert_eq!(
        collect.0.lock().unwrap().clone(),
        [SendRecord {
            kind: SendKind::Status,
            request_bytes: None,
            outcome: "request_cancelled",
            response_bytes: None,
            held_ms: 0,
        }]
    );
    assert_eq!(simulator.received(), 0);
}

/// A lock taken for one send with `acquire_wire_lock` is spent on that send
/// whatever the admission says: the queued post records its intent between
/// taking the lock and sending, and an intent must always be followed by its
/// send.
#[tokio::test]
async fn a_send_whose_lock_was_taken_for_it_is_sent_after_a_withdrawal() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (plain, _) = transport(&simulator);
    let withdrawn = Arc::new(AtomicBool::new(false));
    let gated = plain.with_send_admission(Arc::new(Admission(withdrawn.clone())));
    let held = gated.acquire_wire_lock().await.unwrap();
    withdrawn.store(true, Ordering::SeqCst);
    held.post_xml_decoded("<ENVELOPE/>".into()).await.unwrap();
    assert_eq!(simulator.finish().unwrap().len(), 1);
}

/// Never grants the lock, counting its tries.
#[derive(Default)]
struct AlwaysBusy(AtomicUsize);

impl TallyWireGate for AlwaysBusy {
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Err(WireRefusal::Busy)
    }

    fn pause(&self, _delay: Duration) -> WirePause {
        Box::pin(async {})
    }
}

/// A send already withdrawn is refused before it tries the lock: it neither
/// waits out a busy lock nor reads as a busy port.
#[tokio::test]
async fn a_send_already_withdrawn_never_tries_the_lock() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (plain, _) = transport(&simulator);
    let gate = Arc::new(AlwaysBusy::default());
    let gated = plain
        .with_wire_gate(
            gate.clone(),
            WireRetryPolicy::new(Duration::from_millis(1), Duration::from_secs(1)).unwrap(),
        )
        .with_send_admission(Arc::new(Admission(Arc::new(AtomicBool::new(true)))));
    assert_eq!(
        gated.post_xml_decoded("<ENVELOPE/>".into()).await.err(),
        Some(TallyTransportError::SendWithdrawn)
    );
    assert_eq!(gate.0.load(Ordering::SeqCst), 0);
    assert_eq!(simulator.received(), 0);
}
