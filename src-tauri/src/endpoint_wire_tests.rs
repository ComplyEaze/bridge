//! The file wire lock against another process: the child is this test binary
//! re-invoked with an environment variable, never an executable written for
//! the test.
#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::*;
use crate::local_files::file::open_local_file;
use bridge_tally_transport::{TallyHttpTransport, TallyTransportError};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::Mutex;
use std::thread;
use std::time::Instant;
use tally_protocol_simulator::{Fixture, ScenarioPlan, SequenceSimulator, WireEncoding};

const CHILD_ROOT: &str = "BRIDGE_WIRE_LOCK_TEST_ROOT";
const CHILD_PORT: &str = "BRIDGE_WIRE_LOCK_TEST_PORT";
const CHILD_READY: &str = "BRIDGE_WIRE_LOCK_TEST_READY";
const CHILD_RELEASE: &str = "BRIDGE_WIRE_LOCK_TEST_RELEASE";

/// A forked child holds a copy of every open descriptor until it execs, so a
/// lock another test has just dropped stays taken for that instant. Spawning,
/// and any test that drops a lock and takes it again, exclude each other.
static FORK_WINDOW: Mutex<()> = Mutex::new(());

fn fork_window() -> std::sync::MutexGuard<'static, ()> {
    FORK_WINDOW
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn endpoint(host: &str, port: u16) -> TallyEndpointConfig {
    TallyEndpointConfig {
        host: host.into(),
        port,
    }
}

fn gate(root: &Path, endpoint: &TallyEndpointConfig) -> FileWireGate {
    FileWireGate::new(WireRoot::at(root.to_path_buf()), endpoint.clone())
}

fn xml() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ExportStatusOne).with_encoding(WireEncoding::Utf16Le)
}

fn transport(
    root: &Path,
    endpoint: &TallyEndpointConfig,
    retry: WireRetryPolicy,
) -> TallyHttpTransport {
    TallyHttpTransport::new(endpoint.clone())
        .unwrap()
        .with_wire_gate(Arc::new(gate(root, endpoint)), retry)
}

/// The outer dispatch lease, taken as `endpoint_coordination` takes it.
fn hold_dispatch_lease(root: &Path, endpoint: &TallyEndpointConfig) -> File {
    let lease = open_local_file(&lease_path(root, endpoint).unwrap(), true).unwrap();
    lease.try_lock().unwrap();
    lease
}

#[test]
fn loopback_aliases_share_one_wire_lock_and_ports_stay_independent() {
    let _window = fork_window();
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let held = gate(root, &endpoint("127.0.0.1", 9001))
        .try_acquire()
        .unwrap();
    for host in ["localhost", "::1"] {
        assert_eq!(
            gate(root, &endpoint(host, 9001)).try_acquire().err(),
            Some(WireRefusal::Busy)
        );
    }
    assert!(gate(root, &endpoint("127.0.0.1", 9002))
        .try_acquire()
        .is_ok());
    drop(held);
    assert!(gate(root, &endpoint("localhost", 9001))
        .try_acquire()
        .is_ok());
}

/// The wire lock is its own file beside the lease: a send under a held lease
/// never conflicts with that lease, and the lease's semantics are untouched.
#[test]
fn the_wire_lock_and_the_dispatch_lease_are_separate_files() {
    let _window = fork_window();
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let endpoint = endpoint("127.0.0.1", 9001);
    let lease = hold_dispatch_lease(root, &endpoint);
    let wire = gate(root, &endpoint)
        .try_acquire()
        .expect("a held lease does not refuse its own send");
    drop(lease);
    drop(hold_dispatch_lease(root, &endpoint));
    drop(wire);
    assert_ne!(
        wire_lock_path(&WireRoot::at(root.to_path_buf()), &endpoint),
        lease_path(root, &endpoint).unwrap()
    );
}

/// The wire lock is taken under every root: a send held under the second
/// root alone (a build on the other side of a package boundary) refuses this
/// one as busy, not as unavailable.
#[test]
fn a_wire_lock_held_under_any_one_root_refuses_a_send_as_busy() {
    let _window = fork_window();
    let directory = tempfile::tempdir().unwrap();
    let roots = vec![directory.path().join("old"), directory.path().join("new")];
    let endpoint = endpoint("127.0.0.1", 9001);
    for held in &roots {
        let path = wire_lock_path(&WireRoot::at(held.clone()), &endpoint);
        let holder = open_local_file(&path, true).unwrap();
        holder.try_lock().unwrap();
        let gate = FileWireGate::new(WireRoot::at_all(roots.clone()), endpoint.clone());
        assert_eq!(gate.try_acquire().err(), Some(WireRefusal::Busy));
        holder.unlock().unwrap();
        drop(holder);
        drop(gate.try_acquire().expect("taken once released"));
    }
}

#[test]
fn an_unusable_root_fails_closed() {
    let gate = FileWireGate::new(
        WireRoot {
            paths: None,
            _isolated: None,
        },
        endpoint("127.0.0.1", 9001),
    );
    assert_eq!(gate.try_acquire().err(), Some(WireRefusal::Unavailable));
}

/// Across processes: this process holds the outer dispatch lease and retries
/// the wire lock another process holds; when that holder releases, this send
/// goes through. It shows the wait ends with a held lease, not that no cycle
/// exists: that rests on the holder never waiting for anything while it holds.
#[tokio::test]
async fn a_send_waits_out_another_process_then_goes_through() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("coordination");
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let endpoint = endpoint("127.0.0.1", simulator.address().port());
    let ready = directory.path().join("ready");
    let release = directory.path().join("release");
    let child = ChildHolder::spawn(&root, endpoint.port, &ready, &release);
    wait_for(&ready).unwrap();
    let _lease = hold_dispatch_lease(&root, &endpoint);
    // The clock starts before the releaser's timer, so no stall between the two can
    // make a correct send look early (#1255).
    let started = Instant::now();
    let releaser = {
        let release = release.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(400));
            fs::write(release, b"release").unwrap();
        })
    };
    let response = transport(
        &root,
        &endpoint,
        WireRetryPolicy::new(Duration::from_millis(50), Duration::from_millis(4_950)).unwrap(),
    )
    .post_xml_decoded("<ENVELOPE/>".into())
    .await
    .expect("sent once the other process let go");
    assert!(started.elapsed() >= Duration::from_millis(400));
    assert!(response.request_body_sha256().is_some());
    releaser.join().unwrap();
    assert!(child.wait().success());
    assert_eq!(simulator.finish().unwrap().len(), 1);
}

#[tokio::test]
async fn a_send_refused_past_the_bound_sends_nothing() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("coordination");
    let simulator = SequenceSimulator::spawn(vec![xml()]).unwrap();
    let endpoint = endpoint("127.0.0.1", simulator.address().port());
    let ready = directory.path().join("ready");
    let release = directory.path().join("release");
    let child = ChildHolder::spawn(&root, endpoint.port, &ready, &release);
    wait_for(&ready).unwrap();
    let started = Instant::now();
    let refused = transport(
        &root,
        &endpoint,
        WireRetryPolicy::new(Duration::from_millis(50), Duration::from_millis(150)).unwrap(),
    )
    .post_xml_decoded("<ENVELOPE/>".into())
    .await;
    assert_eq!(
        refused.unwrap_err(),
        TallyTransportError::WireRefused {
            refusal: WireRefusal::Busy
        }
    );
    // It waited its three pauses, and no longer than its bound.
    let waited = started.elapsed();
    assert!(waited >= Duration::from_millis(150), "{waited:?}");
    assert!(waited < WIRE_TEST_BOUND, "{waited:?}");
    assert_eq!(simulator.received(), 0);
    fs::write(&release, b"release").unwrap();
    assert!(child.wait().success());
}

const WIRE_TEST_BOUND: Duration = Duration::from_secs(10);

/// #697 item (a) across runtime operations: every runtime operation of one
/// tool call or desktop command draws on one wait budget, and another call
/// starts with a full one. Nothing is sent: the lock is never free.
#[tokio::test]
async fn runtime_operations_in_one_call_share_one_wait_budget() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().to_path_buf();
    let endpoint = endpoint("127.0.0.1", 9);
    let budget = Duration::from_millis(600);
    let runtime = crate::tally::TallyRuntime::default().with_wire_gate_config(WireGateConfig {
        root: WireRoot::at(root.clone()),
        retry: WireRetryPolicy::new(Duration::from_millis(50), budget).unwrap(),
        drain: None,
        busy_at: None,
        owe_at: None,
    });
    let _other = gate(&root, &endpoint).try_acquire().unwrap();
    fn busy<T>(result: anyhow::Result<T>) -> bool {
        let error = result.err().expect("the wire lock is held");
        wire_refusal(&error) == Some(WireRefusal::Busy)
    }
    crate::tally::runtime::with_operation_wire_budget(async {
        let started = Instant::now();
        assert!(busy(runtime.fetch_companies(endpoint.clone()).await));
        assert!(started.elapsed() >= budget);
        // The call's second operation has nothing left to wait.
        let started = Instant::now();
        assert!(busy(runtime.fetch_companies(endpoint.clone()).await));
        // Nothing left to wait: well under a full budget, which a fresh one would take.
        assert!(started.elapsed() < budget);
    })
    .await;
    let started = Instant::now();
    assert!(busy(runtime.fetch_companies(endpoint.clone()).await));
    assert!(started.elapsed() >= budget);
}

#[test]
fn a_killed_holder_frees_the_lock() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("coordination");
    let endpoint = endpoint("127.0.0.1", 9001);
    let ready = directory.path().join("ready");
    let release = directory.path().join("release");
    let mut child = ChildHolder::spawn(&root, endpoint.port, &ready, &release);
    wait_for(&ready).unwrap();
    let gate = gate(&root, &endpoint);
    assert_eq!(gate.try_acquire().err(), Some(WireRefusal::Busy));
    // Our own child, by its handle: never by name or pattern.
    child.kill();
    // The OS releases the lock with the process; Windows may lag a moment.
    let deadline = Instant::now() + Duration::from_secs(5);
    let held = loop {
        match gate.try_acquire() {
            Ok(held) => break held,
            Err(WireRefusal::Busy) if Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(20));
            }
            Err(refusal) => panic!("lock not released by the crash: {refusal:?}"),
        }
    };
    drop(held);
}

/// The child: holds the wire lock until told to release it. A no-op unless
/// re-invoked by [`ChildHolder::spawn`].
#[test]
fn wire_lock_child() {
    let Some(root) = std::env::var_os(CHILD_ROOT) else {
        return;
    };
    let port = std::env::var(CHILD_PORT)
        .ok()
        .and_then(|value| value.parse().ok())
        .expect("child port");
    let ready = PathBuf::from(std::env::var_os(CHILD_READY).expect("child ready path"));
    let release = PathBuf::from(std::env::var_os(CHILD_RELEASE).expect("child release path"));
    // Another spelling of the same port: the key is the port.
    let gate = gate(&PathBuf::from(root), &endpoint("::1", port));
    let _held = gate.try_acquire().expect("child wire lock");
    fs::write(ready, b"ready").expect("child ready");
    let deadline = Instant::now() + Duration::from_secs(15);
    while !release.exists() {
        assert!(Instant::now() < deadline, "child release timeout");
        thread::sleep(Duration::from_millis(10));
    }
}

struct ChildHolder {
    child: Child,
    release: PathBuf,
}

impl ChildHolder {
    fn spawn(root: &Path, port: u16, ready: &Path, release: &Path) -> Self {
        let module = module_path!()
            .strip_prefix(concat!(env!("CARGO_CRATE_NAME"), "::"))
            .expect("crate module prefix");
        let _window = fork_window();
        let child = Command::new(std::env::current_exe().expect("test executable"))
            .arg("--exact")
            .arg(format!("{module}::wire_lock_child"))
            .arg("--nocapture")
            .env(CHILD_ROOT, root)
            .env(CHILD_PORT, port.to_string())
            .env(CHILD_READY, ready)
            .env(CHILD_RELEASE, release)
            .spawn()
            .expect("wire lock child");
        Self {
            child,
            release: release.to_path_buf(),
        }
    }

    fn wait(mut self) -> std::process::ExitStatus {
        self.child.wait().expect("child exit")
    }

    fn kill(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for ChildHolder {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = fs::write(&self.release, b"release");
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn wait_for(path: &Path) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !path.exists() {
        if Instant::now() >= deadline {
            return Err("child did not take the wire lock".into());
        }
        thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

// -- the drain record (#2142) -------------------------------------------------

#[test]
fn the_drain_record_is_one_file_in_the_last_root_beside_its_lock() {
    let first = tempfile::tempdir().unwrap();
    let shared = tempfile::tempdir().unwrap();
    let root = WireRoot::at_all(vec![
        first.path().to_path_buf(),
        shared.path().to_path_buf(),
    ]);
    let endpoint = endpoint("127.0.0.1", 9000);
    let held = FileWireGate::new(root.clone(), endpoint.clone())
        .try_acquire()
        .unwrap();
    assert_eq!(
        held.debt(),
        Ok(DrainDebt::Clear),
        "an absent record is clear"
    );
    let owed = DrainDebt::Owed {
        since_ms: 7,
        quick: 2,
        slow: 1,
        last_probe_ms: Some(9),
        slow_since_ms: Some(8),
    };
    held.record(owed).unwrap();
    assert_eq!(held.debt(), Ok(owed));
    let path = debt_path(&root, &endpoint).unwrap();
    assert!(path.starts_with(shared.path()), "{path:?}");
    assert_eq!(
        path.file_name().unwrap().to_str().unwrap(),
        wire_lock_path(&WireRoot::at(shared.path().to_path_buf()), &endpoint)
            .with_extension("debt")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    );
    assert!(fs::read_dir(first.path()).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".wire.debt")
    }));
    // A shorter record replaces a longer one whole.
    held.record(DrainDebt::Clear).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), r#"{"state":"clear"}"#);
    assert_eq!(held.debt(), Ok(DrainDebt::Clear));
    held.record(DrainDebt::InFlight { since_ms: 5 }).unwrap();
    drop(held);
    // Another holder, as a process replacing this one would be, reads it.
    let again = FileWireGate::new(root, endpoint).try_acquire().unwrap();
    assert_eq!(again.debt(), Ok(DrainDebt::InFlight { since_ms: 5 }));
}

#[test]
fn an_unreadable_drain_record_is_owed() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("x.wire.debt");
    for text in [
        "",
        "{",
        r#"{"state":"done"}"#,
        r#"{"state":"clear","extra":1}"#,
        r#"{"state":"owed","since_ms":1}"#,
        "\u{0}\u{0}",
    ] {
        fs::write(&path, text).unwrap();
        assert!(
            matches!(
                read_debt(&path, 42),
                Ok(DrainDebt::Owed {
                    since_ms: 42,
                    quick: 0,
                    slow: 0,
                    last_probe_ms: None,
                    slow_since_ms: None
                })
            ),
            "{text:?}"
        );
    }
    let unreadable = directory.path().join("a-directory");
    fs::create_dir(&unreadable).unwrap();
    assert_eq!(read_debt(&unreadable, 42), Err(WireRefusal::Unavailable));
}

#[test]
fn every_wire_refusal_that_passes_by_waiting_names_its_retry() {
    assert_eq!(wire_retry_after_s("tally_endpoint_busy"), Some(5));
    assert_eq!(wire_retry_after_s("tally_endpoint_draining"), Some(10));
    assert_eq!(
        wire_retry_after_s("tally_endpoint_needs_attention"),
        Some(30)
    );
    assert_eq!(wire_retry_after_s("tally_endpoint_lock_unavailable"), None);
    assert_eq!(wire_retry_after_s("request_deadline_exceeded"), None);
}

#[test]
fn a_build_drains_by_the_measured_policy() {
    // Unit tests run with no drain unless one asks; a build's default is
    // `production_drain`.
    assert_eq!(production_drain(), Some(DrainPolicy::MEASURED));
    assert_eq!(WireGateConfig::default().drain(), None);
    assert_eq!(
        WireGateConfig::default()
            .with_drain(DrainPolicy::MEASURED)
            .drain(),
        Some(DrainPolicy::MEASURED)
    );
}

/// Two clients on one shared root, as two processes are: one client's post is
/// cut for time, and the other then finds the drain owed. Its probe is cut by
/// its own deadline while the simulator, serving one request at a time as Tally
/// does, is still on the cut post, so it is refused as draining with no post
/// sent: the probe never reached the head of the queue.
#[tokio::test]
async fn a_post_cut_by_one_client_holds_the_next_post_of_another() {
    let root = tempfile::tempdir().unwrap();
    let simulator = SequenceSimulator::spawn(vec![
        xml().with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
            Duration::from_millis(900),
        )),
        ScenarioPlan::new(Fixture::ProductStatus(
            tally_protocol_simulator::ProductStatus::TallyPrime,
        ))
        .with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
            Duration::from_millis(600),
        )),
    ])
    .unwrap();
    let endpoint = endpoint(
        &simulator.address().ip().to_string(),
        simulator.address().port(),
    );
    let policy = DrainPolicy {
        probe_deadline: Duration::from_millis(300),
        quick_below: Duration::from_millis(100),
        ..DrainPolicy::MEASURED
    };
    let client = || {
        TallyHttpTransport::with_policy(
            endpoint.clone(),
            bridge_tally_transport::TransportPolicy {
                request_timeout: Duration::from_millis(300),
                ..bridge_tally_transport::TransportPolicy::default()
            },
        )
        .unwrap()
        .with_wire_gate(
            Arc::new(gate(root.path(), &endpoint)),
            WireRetryPolicy::DEFAULT,
        )
        .with_drain(policy)
    };
    assert_eq!(
        client().post_xml_decoded("<E />".into()).await.unwrap_err(),
        TallyTransportError::RequestTimedOut
    );
    let path = debt_path(&WireRoot::at(root.path().to_path_buf()), &endpoint).unwrap();
    assert!(fs::read_to_string(&path)
        .unwrap()
        .contains(r#""state":"owed""#));
    let refused = client().post_xml_decoded("<E />".into()).await.unwrap_err();
    assert_eq!(refused.safe_code(), "tally_endpoint_draining");
    let after = fs::read_to_string(&path).unwrap();
    assert!(after.contains(r#""quick":0"#), "{after}");
    assert!(
        !after.contains(r#""last_probe_ms":null"#),
        "the probe was sent: {after}"
    );
    simulator.cancel();
    let paths: Vec<String> = simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .map(|request| request.path)
        .collect();
    assert_eq!(paths, ["/"]);
}
