//! The request trail of one tool call (#918): what each Tally send of the call
//! was and how it ended, so a failed multi-request read can say WHICH request
//! failed, not only that the call did.
//!
//! The trail holds kinds, sizes, an outcome code and a time, taken from the
//! transport's [`SendRecord`]: never a request or response body, a company name
//! or a row value, and no hash of one (a request names the company, so a hash
//! of it would let a reader holding a guessed name confirm it). A failed read is
//! matched with `read_evidence` by the sequence number and the receipt's time. It keeps the last [`TRAIL_LAST`] sends of the
//! call and counts the rest, so a hundred-request read cannot grow the journal
//! without bound.
use std::collections::VecDeque;
use std::future::Future;
use std::sync::{Arc, Mutex};

use bridge_tally_transport::{SendObserver, SendRecord};
use serde_json::{json, Value};

/// The sends a trail keeps: the last ones; the earlier ones are counted.
pub(crate) const TRAIL_LAST: usize = 32;

#[derive(Default)]
pub(crate) struct RequestTrail {
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    sent: usize,
    /// Sends that did not end `answered`, counted over the whole call, the
    /// omitted ones included (a retry may have recovered any of them).
    failed: usize,
    records: VecDeque<(usize, SendRecord)>,
}

tokio::task_local! {
    /// The trail of the tool call running in this task. A task without one
    /// (a spawned snapshot run, a desktop command, a test) records nothing.
    static CALL_TRAIL: Arc<RequestTrail>;
}

impl RequestTrail {
    pub(crate) fn push(&self, record: SendRecord) {
        let mut inner = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.sent += 1;
        if record.outcome != "answered" {
            inner.failed += 1;
        }
        let sequence = inner.sent;
        inner.records.push_back((sequence, record));
        while inner.records.len() > TRAIL_LAST {
            inner.records.pop_front();
        }
    }

    /// The trail as the receipt keeps it, built field by field so that nothing
    /// but the fields below can reach it; `None` when the call sent nothing.
    pub(crate) fn snapshot(&self) -> Option<Value> {
        let inner = self
            .inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if inner.sent == 0 {
            return None;
        }
        let last: Vec<Value> = inner
            .records
            .iter()
            .map(|(sequence, record)| {
                let mut kept = json!({
                    "seq": sequence,
                    "kind": record.kind.label(),
                    "outcome": record.outcome,
                    "held_ms": record.held_ms,
                });
                if let Some(bytes) = record.request_bytes {
                    kept["request_bytes"] = json!(bytes);
                }
                if let Some(bytes) = record.response_bytes {
                    kept["response_bytes"] = json!(bytes);
                }
                kept
            })
            .collect();
        let mut trail = json!({
            "sent": inner.sent,
            "omitted": inner.sent - last.len(),
            "failed": inner.failed,
            "last": last,
        });
        // The last kept send that did not end answered, when there is one. A
        // later send may have recovered it (a retry), so it is where a failure
        // was seen, not always where the call stopped.
        if let Some((sequence, _)) = inner
            .records
            .iter()
            .rev()
            .find(|(_, record)| record.outcome != "answered")
        {
            trail["last_failed_seq"] = json!(sequence);
        }
        Some(trail)
    }
}

struct TrailObserver;

impl SendObserver for TrailObserver {
    fn observe(&self, record: SendRecord) {
        let _ = CALL_TRAIL.try_with(|trail| trail.push(record));
    }
}

/// The observer the application gives every Tally transport it builds.
pub(crate) fn observer() -> Arc<dyn SendObserver> {
    Arc::new(TrailObserver)
}

/// Run `call`, one tool call, with a trail of its own; its output and the
/// trail's snapshot. The caller boxes `call`: a tool call's state machine is
/// large, and held inline here (or in the caller's own state) it overflowed a
/// 2 MiB test thread's stack.
pub(crate) async fn with_request_trail<T>(
    call: std::pin::Pin<Box<dyn Future<Output = T> + Send + '_>>,
) -> (T, Option<Value>) {
    let trail = Arc::new(RequestTrail::default());
    let output = CALL_TRAIL.scope(Arc::clone(&trail), call).await;
    (output, trail.snapshot())
}

#[cfg(test)]
#[path = "request_trail_tests.rs"]
mod tests;
