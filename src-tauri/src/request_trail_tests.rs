use super::*;
use bridge_tally_transport::SendKind;

fn record(outcome: &'static str, held_ms: u64) -> SendRecord {
    SendRecord {
        kind: SendKind::Post,
        request_bytes: Some(700),
        outcome,
        response_bytes: (outcome == "answered").then_some(1_234),
        held_ms,
    }
}

#[test]
fn a_call_that_sent_nothing_keeps_no_trail() {
    assert_eq!(RequestTrail::default().snapshot(), None);
}

#[test]
fn the_trail_keeps_the_last_sends_and_counts_the_rest() {
    let trail = RequestTrail::default();
    for index in 0..(TRAIL_LAST + 10) {
        trail.push(record("answered", index as u64));
    }
    let snapshot = trail.snapshot().unwrap();
    assert_eq!(snapshot["sent"], TRAIL_LAST + 10);
    assert_eq!(snapshot["omitted"], 10);
    let last = snapshot["last"].as_array().unwrap();
    assert_eq!(last.len(), TRAIL_LAST);
    // Oldest first, numbered by their place in the call: the first ten dropped.
    assert_eq!(last[0]["seq"], 11);
    assert_eq!(last[TRAIL_LAST - 1]["seq"], TRAIL_LAST + 10);
    assert!(snapshot.get("last_failed_seq").is_none());
    assert_eq!(snapshot["failed"], 0);
}

/// A failure older than the kept sends is no longer listed, but it is counted.
#[test]
fn a_failure_the_cap_dropped_is_still_counted() {
    let trail = RequestTrail::default();
    trail.push(record("request_failed", 0));
    for _ in 0..TRAIL_LAST {
        trail.push(record("answered", 1));
    }
    let snapshot = trail.snapshot().unwrap();
    assert_eq!(snapshot["omitted"], 1);
    assert_eq!(snapshot["failed"], 1);
    assert!(snapshot.get("last_failed_seq").is_none());
}

#[test]
fn the_trail_names_the_last_send_that_did_not_end_answered() {
    let trail = RequestTrail::default();
    trail.push(record("answered", 5));
    trail.push(record("request_failed", 0));
    trail.push(record("answered", 7));
    trail.push(record("request_deadline_exceeded", 20_000));
    trail.push(record("answered", 9));
    let snapshot = trail.snapshot().unwrap();
    assert_eq!(snapshot["last_failed_seq"], 4);
    assert_eq!(snapshot["failed"], 2);
    assert_eq!(snapshot["last"][3]["outcome"], "request_deadline_exceeded");
    assert_eq!(snapshot["last"][3]["held_ms"], 20_000);
}

/// Only kinds, sizes, outcome codes and times can reach the trail: no text and
/// no hash of a request or response.
#[test]
fn nothing_but_the_whitelisted_fields_reaches_the_trail() {
    let trail = RequestTrail::default();
    trail.push(record("answered", 1));
    let snapshot = trail.snapshot().unwrap();
    let first = &snapshot["last"][0];
    let mut keys: Vec<_> = first.as_object().unwrap().keys().cloned().collect();
    keys.sort();
    assert_eq!(
        keys,
        [
            "held_ms",
            "kind",
            "outcome",
            "request_bytes",
            "response_bytes",
            "seq"
        ]
    );
    // No run of 64 hex digits anywhere in it.
    let text = snapshot.to_string();
    let hex_run = text
        .as_bytes()
        .windows(64)
        .any(|window| window.iter().all(u8::is_ascii_hexdigit));
    assert!(!hex_run, "{text}");
}

#[tokio::test]
async fn a_send_outside_a_call_is_not_recorded_and_inside_it_is() {
    let outside = observer();
    outside.observe(record("answered", 1)); // no task-local trail: dropped, no panic
    let ((), snapshot) = with_request_trail(Box::pin(async {
        observer().observe(record("answered", 2));
        observer().observe(record("request_failed", 3));
    }))
    .await;
    let snapshot = snapshot.unwrap();
    assert_eq!(snapshot["sent"], 2);
    assert_eq!(snapshot["last_failed_seq"], 2);
    // The next call starts with a trail of its own.
    let ((), again) = with_request_trail(Box::pin(async {})).await;
    assert_eq!(again, None);
}

/// The acceptance of #918, end to end through the real transport: a read of more
/// sends than the cap keeps the cap and the LAST sends, counts the rest, and a
/// failure on a later leg is named by its place.
#[tokio::test]
async fn a_long_read_keeps_the_last_sends_and_names_a_late_failure() {
    use bridge_tally_transport::{TallyEndpointConfig, TallyHttpTransport};
    use tally_protocol_simulator::{Fixture, ScenarioPlan, SequenceSimulator, WireEncoding};
    let plan = || ScenarioPlan::new(Fixture::ExportStatusOne).with_encoding(WireEncoding::Utf16Le);
    let mut plans = vec![plan(); 38];
    plans.push(plan().with_http_status(500)); // the 39th send fails
    plans.push(plan());
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let transport = TallyHttpTransport::new(TallyEndpointConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    })
    .unwrap()
    .with_send_observer(observer());
    let ((), snapshot) = with_request_trail(Box::pin(async {
        for _ in 0..40 {
            let _ = transport.post_xml_decoded("<ENVELOPE/>".into()).await;
        }
    }))
    .await;
    let snapshot = snapshot.unwrap();
    assert_eq!(snapshot["sent"], 40);
    assert_eq!(snapshot["omitted"], 40 - TRAIL_LAST);
    assert_eq!(snapshot["failed"], 1);
    assert_eq!(snapshot["last_failed_seq"], 39);
    let last = snapshot["last"].as_array().unwrap();
    assert_eq!(last.len(), TRAIL_LAST);
    assert_eq!(last[TRAIL_LAST - 1]["seq"], 40);
    let failed = last.iter().find(|send| send["seq"] == 39).unwrap();
    assert_eq!(failed["outcome"], "http_status_failure");
    simulator.finish().unwrap();
}
