//! The file wire lock against another process: the child is this test binary
//! re-invoked with an environment variable, never an executable written for
//! the test.
#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::*;
use bridge_tally_transport::{TallyHttpTransport, TallyTransportError};
use std::fs;
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

#[test]
fn an_unusable_root_fails_closed() {
    let gate = FileWireGate::new(
        WireRoot {
            path: None,
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
    let releaser = {
        let release = release.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(400));
            fs::write(release, b"release").unwrap();
        })
    };
    let started = Instant::now();
    let response = transport(
        &root,
        &endpoint,
        WireRetryPolicy::new(Duration::from_millis(50), Duration::from_millis(4_950)).unwrap(),
    )
    .post_xml_decoded("<ENVELOPE/>".into())
    .await
    .expect("sent once the other process let go");
    assert!(started.elapsed() >= Duration::from_millis(300));
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
    });
    let _other = gate(&root, &endpoint).try_acquire().unwrap();
    let busy = |result: anyhow::Result<crate::tally::ConnectionStatus>| {
        wire_refusal(&result.expect_err("the wire lock is held")) == Some(WireRefusal::Busy)
    };
    crate::tally::runtime::with_operation_wire_budget(async {
        let started = Instant::now();
        assert!(busy(runtime.check_connection(endpoint.clone()).await));
        assert!(started.elapsed() >= budget);
        // The call's second operation has nothing left to wait.
        let started = Instant::now();
        assert!(busy(runtime.check_connection(endpoint.clone()).await));
        assert!(started.elapsed() < budget / 2);
    })
    .await;
    let started = Instant::now();
    assert!(busy(runtime.check_connection(endpoint.clone()).await));
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
