//! #918: a transport built with an observer tells it about every send, however
//! the send ends, in kinds, sizes, outcome codes and times, never text.
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bridge_tally_transport::{
    SendKind, SendObserver, SendRecord, TallyEndpointConfig, TallyHttpTransport, TallyWireGate,
    TransportPolicy, WireLockHeld, WirePause, WireRefusal, WireRetryPolicy,
};
use tally_protocol_simulator::{
    Delivery, Fixture, ProductStatus, ScenarioPlan, SequenceSimulator, WireEncoding,
};

#[derive(Default)]
struct Collect(Mutex<Vec<SendRecord>>);

impl SendObserver for Collect {
    fn observe(&self, record: SendRecord) {
        self.0.lock().unwrap().push(record);
    }
}

fn xml() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ExportStatusOne).with_encoding(WireEncoding::Utf16Le)
}

fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}

fn observed(
    simulator: &SequenceSimulator,
    policy: Option<TransportPolicy>,
) -> (TallyHttpTransport, Arc<Collect>) {
    let config = TallyEndpointConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    };
    let collect = Arc::new(Collect::default());
    let transport = match policy {
        Some(policy) => TallyHttpTransport::with_policy(config, policy),
        None => TallyHttpTransport::new(config),
    }
    .unwrap()
    .with_send_observer(collect.clone());
    (transport, collect)
}

#[tokio::test]
async fn an_answered_post_and_an_answered_status_are_recorded_with_sizes() {
    let simulator = SequenceSimulator::spawn(vec![xml(), status(), xml()]).unwrap();
    let (transport, collect) = observed(&simulator, None);
    let first = transport
        .post_xml_decoded("<ENVELOPE/>".into())
        .await
        .unwrap();
    transport.get_status_decoded().await.unwrap();
    let raw = transport.post_xml("<ENVELOPE/>".into()).await.unwrap();
    let records = collect.0.lock().unwrap().clone();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].kind, SendKind::Post);
    assert_eq!(records[0].outcome, "answered");
    assert!(records[0].request_bytes.unwrap() > 0);
    assert_eq!(records[0].response_bytes, Some(first.encoded_bytes()));
    assert_eq!(records[1].kind, SendKind::Status);
    assert_eq!(records[1].request_bytes, None);
    assert_eq!(records[1].outcome, "answered");
    // The undecoded POST path records too, and sizes the bytes it received.
    assert_eq!(records[2].kind, SendKind::Post);
    assert_eq!(records[2].response_bytes, Some(raw.encoded_bytes()));
    assert_eq!(records[2].request_bytes, records[0].request_bytes);
    assert!(records.iter().all(|record| record.held_ms < 5_000));
    simulator.finish().unwrap();
}

#[tokio::test]
async fn a_failed_send_is_recorded_with_its_outcome_code_and_no_response() {
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_http_status(500),
        status().with_http_status(503),
    ])
    .unwrap();
    let (transport, collect) = observed(&simulator, None);
    assert!(transport
        .post_xml_decoded("<ENVELOPE/>".into())
        .await
        .is_err());
    assert!(transport.get_status_decoded().await.is_err());
    let records = collect.0.lock().unwrap().clone();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].outcome, "http_status_failure");
    assert!(records[0].request_bytes.is_some());
    assert_eq!(records[0].response_bytes, None);
    assert_eq!(records[1].outcome, "http_status_failure");
    assert_eq!(records[1].kind, SendKind::Status);
    simulator.finish().unwrap();
}

#[tokio::test]
async fn a_send_past_its_deadline_is_recorded_as_timed_out_with_the_time_held() {
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(Delivery::SlowHeaders(Duration::from_millis(600)))
    ])
    .unwrap();
    let policy = TransportPolicy {
        request_timeout: Duration::from_millis(150),
        ..TransportPolicy::default()
    };
    let (transport, collect) = observed(&simulator, Some(policy));
    assert!(transport
        .post_xml_decoded("<ENVELOPE/>".into())
        .await
        .is_err());
    let records = collect.0.lock().unwrap().clone();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].outcome, "request_deadline_exceeded");
    assert!(records[0].held_ms >= 100, "{records:?}");
}

struct AlwaysBusy;

impl TallyWireGate for AlwaysBusy {
    fn try_acquire(&self) -> Result<Box<dyn WireLockHeld>, WireRefusal> {
        Err(WireRefusal::Busy)
    }

    fn pause(&self, _delay: Duration) -> WirePause {
        Box::pin(std::future::ready(()))
    }
}

#[tokio::test]
async fn a_send_the_wire_gate_refused_is_recorded_as_refused_and_sent_nothing() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (transport, collect) = observed(&simulator, None);
    let transport = transport.with_wire_gate(
        Arc::new(AlwaysBusy),
        WireRetryPolicy::new(Duration::from_millis(1), Duration::ZERO).unwrap(),
    );
    assert!(transport
        .post_xml_decoded("<ENVELOPE/>".into())
        .await
        .is_err());
    let records = collect.0.lock().unwrap().clone();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].outcome, "tally_endpoint_busy");
    assert_eq!(records[0].held_ms, 0);
    assert!(records[0].request_bytes.is_some());
    assert_eq!(records[0].response_bytes, None);
    assert_eq!(simulator.received(), 0);
}

/// The positive control (#944) is an observed transport beside the plain one,
/// on the same endpoint: it records its own send, a status, and nothing of the
/// plain one's post.
#[tokio::test]
async fn a_transport_without_an_observer_records_nothing_and_works() {
    let simulator = SequenceSimulator::spawn(vec![xml(), status()]).unwrap();
    let plain = TallyHttpTransport::new(TallyEndpointConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    })
    .unwrap();
    let (control, collect) = observed(&simulator, None);
    plain.post_xml_decoded("<ENVELOPE/>".into()).await.unwrap();
    control.get_status_decoded().await.unwrap();
    let records = collect.0.lock().unwrap().clone();
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(records[0].kind, SendKind::Status);
    assert_eq!(records[0].outcome, "answered");
    assert_eq!(simulator.finish().unwrap().len(), 2);
}

#[tokio::test]
async fn a_held_lock_refusal_and_a_held_lock_send_are_recorded() {
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let (transport, collect) = observed(&simulator, None);
    // Taken once, then spent on exactly one send through the held transport.
    let held = transport.acquire_wire_lock().await.unwrap();
    let response = held.post_xml_decoded("<ENVELOPE/>".into()).await.unwrap();
    let busy = transport.clone().with_wire_gate(
        Arc::new(AlwaysBusy),
        WireRetryPolicy::new(Duration::from_millis(1), Duration::ZERO).unwrap(),
    );
    assert!(busy.acquire_wire_lock().await.is_err());
    let records = collect.0.lock().unwrap().clone();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].outcome, "answered");
    assert_eq!(records[0].response_bytes, Some(response.encoded_bytes()));
    assert_eq!(records[1].outcome, "tally_endpoint_busy");
    assert_eq!(records[1].kind, SendKind::Post);
    assert_eq!(records[1].request_bytes, None);
    assert_eq!(records[1].held_ms, 0);
    simulator.finish().unwrap();
}

#[tokio::test]
async fn a_send_whose_future_is_dropped_is_recorded_as_abandoned() {
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(Delivery::SlowHeaders(Duration::from_secs(3)))
    ])
    .unwrap();
    let (transport, collect) = observed(&simulator, None);
    // The caller gives up (a cancelled call drops the send mid-flight): once the
    // request has reached the simulator and been held 150 ms. Both waits only
    // lengthen under a stall, so `held_ms >= 100` holds however late the request
    // starts, and the 3 s slow headers keep the send in flight (#1255).
    let mut send = Box::pin(transport.post_xml_decoded("<ENVELOPE/>".into()));
    let gives_up = async {
        while simulator.received() == 0 {
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
        tokio::time::sleep(Duration::from_millis(150)).await;
    };
    // `biased`, giving up first: if a long stall made both ready, the send must not win.
    let answered = tokio::select! {
        biased;
        () = gives_up => None,
        sent = &mut send => Some(sent),
    };
    assert!(answered.is_none(), "the send should still be in flight");
    drop(send);
    simulator.cancel();
    let records = collect.0.lock().unwrap().clone();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].outcome, "send_abandoned");
    assert_eq!(records[0].kind, SendKind::Post);
    assert!(records[0].request_bytes.is_some());
    assert_eq!(records[0].response_bytes, None);
    assert!(records[0].held_ms >= 100, "{records:?}");
}

#[tokio::test]
async fn the_undecoded_status_path_records_a_size_and_no_hash() {
    let simulator = SequenceSimulator::spawn(vec![status()]).unwrap();
    let (transport, collect) = observed(&simulator, None);
    let response = transport.get_status().await.unwrap();
    let records = collect.0.lock().unwrap().clone();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].kind, SendKind::Status);
    assert_eq!(records[0].outcome, "answered");
    assert_eq!(records[0].response_bytes, Some(response.encoded_bytes()));
    simulator.finish().unwrap();
}
