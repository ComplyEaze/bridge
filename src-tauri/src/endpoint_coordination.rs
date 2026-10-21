//! Cross-process native-dispatch admission for one local Tally listener.
//!
//! This lease is deliberately outside `BRIDGE_AGENT_DATA_DIR`: two connector
//! processes may keep distinct recoverable journals while still talking to one
//! loopback Tally endpoint. The primary port lock is exclusive for native
//! dispatch, shared for snapshots and short durable observations. Snapshots
//! additionally take a secondary exclusive lock so they remain serialized.
//! A crash drops the kernel leases; the caller's durable local intent and
//! reconciliation remain the recovery boundary.
//!
//! Each lease is a set of files, one under each coordination root
//! (`paths::dispatch_coordination_roots`), taken all or none (#1584).
use crate::local_files::{
    directory::ensure_private_directory,
    lock_set::{try_lock_all, HeldLocks, LockMode, LockSetError},
    paths::dispatch_coordination_roots,
};
use bridge_tally_transport::{canonical_loopback_origin, TallyEndpointConfig};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

pub(crate) struct EndpointDispatchLease {
    _locks: HeldLocks,
}

pub(crate) struct EndpointSnapshotLease {
    _primary: EndpointObservationLease,
    _snapshot: HeldLocks,
}

pub(crate) struct EndpointObservationLease {
    _primary: HeldLocks,
}

/// The roots every lease is taken under, or the refusal when one cannot be
/// resolved.
fn roots() -> Result<Vec<PathBuf>, String> {
    dispatch_coordination_roots().ok_or_else(|| "import_admission_lock_unavailable".to_string())
}

fn as_paths(roots: &[PathBuf]) -> Vec<&Path> {
    roots.iter().map(PathBuf::as_path).collect()
}

fn lease_error(error: LockSetError) -> String {
    match error {
        LockSetError::Busy => "import_admission_busy",
        LockSetError::Unavailable => "import_admission_lock_unavailable",
    }
    .into()
}

/// The port's lease file under each root, with `extension` for a lock beside it.
fn lease_paths(
    roots: &[&Path],
    endpoint: &TallyEndpointConfig,
    extension: Option<&str>,
) -> Result<Vec<PathBuf>, String> {
    roots
        .iter()
        .map(|root| {
            let path = lease_path(root, endpoint)?;
            Ok(match extension {
                Some(extension) => path.with_extension(extension),
                None => path,
            })
        })
        .collect()
}

/// Acquire the existing nonblocking, exclusive per-user/per-port dispatch lease.
///
/// The endpoint is validated first. Every admitted loopback alias on the same
/// port shares one lease because IPv4, IPv6 and localhost can address the same
/// Tally listener. This is intentionally more conservative than the transport
/// endpoint identity.
pub(crate) fn acquire(endpoint: &TallyEndpointConfig) -> Result<EndpointDispatchLease, String> {
    acquire_at(&as_paths(&roots()?), endpoint)
}

/// Acquire a short shared observation lease. An active native dispatch excludes
/// it; a read-only snapshot does not.
pub(crate) fn acquire_observation(
    endpoint: &TallyEndpointConfig,
) -> Result<EndpointObservationLease, String> {
    acquire_observation_at(&as_paths(&roots()?), endpoint)
}

/// Acquire a serialized read-only snapshot lease: shared on the primary
/// dispatch lane and exclusive on its snapshot-only mutex.
pub(crate) fn acquire_snapshot(
    endpoint: &TallyEndpointConfig,
) -> Result<EndpointSnapshotLease, String> {
    acquire_snapshot_at(&as_paths(&roots()?), endpoint)
}

fn acquire_at(
    roots: &[&Path],
    endpoint: &TallyEndpointConfig,
) -> Result<EndpointDispatchLease, String> {
    let paths = lease_paths(roots, endpoint, None)?;
    let locks = try_lock_all(&paths, LockMode::Exclusive).map_err(lease_error)?;
    Ok(EndpointDispatchLease { _locks: locks })
}

fn acquire_observation_at(
    roots: &[&Path],
    endpoint: &TallyEndpointConfig,
) -> Result<EndpointObservationLease, String> {
    let paths = lease_paths(roots, endpoint, None)?;
    let locks = try_lock_all(&paths, LockMode::Shared).map_err(lease_error)?;
    Ok(EndpointObservationLease { _primary: locks })
}

fn acquire_snapshot_at(
    roots: &[&Path],
    endpoint: &TallyEndpointConfig,
) -> Result<EndpointSnapshotLease, String> {
    let primary = acquire_observation_at(roots, endpoint)?;
    let paths = lease_paths(roots, endpoint, Some("snapshot.lock"))?;
    let snapshot = try_lock_all(&paths, LockMode::Exclusive).map_err(lease_error)?;
    Ok(EndpointSnapshotLease {
        _primary: primary,
        _snapshot: snapshot,
    })
}

/// The per-port lease file. `endpoint_wire` derives its per-send wire lock
/// from this path, beside the lease, so both share one key and one directory;
/// it never opens this file itself.
pub(crate) fn lease_path(
    coordination_root: &Path,
    endpoint: &TallyEndpointConfig,
) -> Result<PathBuf, String> {
    // Validate the configured host before intentionally collapsing all loopback
    // aliases for this port into one conservative dispatch lane.
    canonical_loopback_origin(endpoint)
        .map_err(|_| "import_admission_lock_unavailable".to_string())?;
    let directory = coordination_root.join("native-dispatch-leases");
    ensure_private_directory(&directory)
        .map_err(|_| "import_admission_lock_unavailable".to_string())?;
    let mut key = String::with_capacity(64);
    for byte in Sha256::digest(format!("native-dispatch-port:{}", endpoint.port).as_bytes()) {
        use std::fmt::Write as _;
        let _ = write!(key, "{byte:02x}");
    }
    Ok(directory.join(format!("{key}.lock")))
}

#[cfg(test)]
#[path = "endpoint_coordination_tests.rs"]
mod tests;

#[cfg(all(test, target_os = "macos"))]
#[path = "endpoint_coordination_macos_tests.rs"]
mod macos_tests;
