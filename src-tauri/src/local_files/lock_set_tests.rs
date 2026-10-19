use super::*;
use std::fs;
use std::path::Path;

fn paths(root: &Path, names: &[&str]) -> Vec<PathBuf> {
    names.iter().map(|name| root.join(name)).collect()
}

#[test]
fn an_exclusive_set_excludes_every_other_hold_and_shared_sets_coexist() {
    let directory = tempfile::tempdir().unwrap();
    let both = paths(directory.path(), &["old.lock", "new.lock"]);
    let shared = try_lock_all(&both, LockMode::Shared).unwrap();
    let also_shared = try_lock_all(&both, LockMode::Shared).unwrap();
    assert_eq!(
        try_lock_all(&both, LockMode::Exclusive).err(),
        Some(LockSetError::Busy)
    );
    drop(shared);
    drop(also_shared);
    let exclusive = try_lock_all(&both, LockMode::Exclusive).unwrap();
    assert_eq!(
        try_lock_all(&both, LockMode::Shared).err(),
        Some(LockSetError::Busy)
    );
    drop(exclusive);
    drop(try_lock_all(&both, LockMode::Exclusive).unwrap());
}

/// A build that takes only one of the files (an earlier one, or one on the
/// other side of a package boundary) still excludes a build that takes both,
/// whichever of the two holds exclusively.
#[test]
fn a_hold_on_any_one_file_of_the_set_makes_the_set_busy() {
    let directory = tempfile::tempdir().unwrap();
    let both = paths(directory.path(), &["old.lock", "new.lock"]);
    for one in [&both[..1], &both[1..]] {
        for (single_mode, set_mode) in [
            (LockMode::Exclusive, LockMode::Shared),
            (LockMode::Shared, LockMode::Exclusive),
        ] {
            let single = try_lock_all(one, single_mode).unwrap();
            assert_eq!(
                try_lock_all(&both, set_mode).err(),
                Some(LockSetError::Busy)
            );
            drop(single);
        }
    }
}

#[test]
fn an_empty_set_is_refused_rather_than_held() {
    assert_eq!(
        try_lock_all(&[], LockMode::Exclusive).err(),
        Some(LockSetError::Unavailable)
    );
}

/// The locks are released explicitly, not only by closing the handle: a
/// second handle to the same open file keeps it open past the drop, and the
/// lock can be taken again. A refused set drops what it took the same way.
#[cfg(unix)]
#[test]
fn a_set_releases_its_locks_itself_when_dropped() {
    let directory = tempfile::tempdir().unwrap();
    let both = paths(directory.path(), &["old.lock", "new.lock"]);
    let held = try_lock_all(&both, LockMode::Exclusive).unwrap();
    let kept_open = held.files[0].try_clone().unwrap();
    drop(held);
    drop(try_lock_all(&both, LockMode::Exclusive).expect("released on drop"));
    drop(kept_open);
}

/// A set refused by its last file has released its first one by the time the
/// refusal returns, so no lock is held through the caller's pause.
#[test]
fn a_busy_file_releases_every_lock_the_set_had_taken() {
    let directory = tempfile::tempdir().unwrap();
    let both = paths(directory.path(), &["old.lock", "new.lock"]);
    let holder = try_lock_all(&both[1..], LockMode::Exclusive).unwrap();
    assert_eq!(
        try_lock_all(&both, LockMode::Exclusive).err(),
        Some(LockSetError::Busy)
    );
    drop(try_lock_all(&both[..1], LockMode::Exclusive).expect("the first file was released"));
    drop(holder);
}

/// Two names for one file are locked once: a second lock on it from this
/// process would refuse the set for good.
#[test]
fn one_file_named_twice_is_locked_once() {
    let directory = tempfile::tempdir().unwrap();
    let twice = paths(directory.path(), &["same.lock", "same.lock"]);
    let held = try_lock_all(&twice, LockMode::Exclusive).unwrap();
    assert_eq!(held.files.len(), 1);
    drop(held);
}

#[test]
fn a_file_that_cannot_be_opened_makes_the_set_unavailable() {
    let directory = tempfile::tempdir().unwrap();
    fs::create_dir(directory.path().join("folder.lock")).unwrap();
    let set = paths(directory.path(), &["old.lock", "folder.lock"]);
    assert_eq!(
        try_lock_all(&set, LockMode::Exclusive).err(),
        Some(LockSetError::Unavailable)
    );
    // nothing it opened stays locked
    drop(try_lock_all(&set[..1], LockMode::Exclusive).unwrap());
}

/// A lock file a set holds cannot be deleted, so its name cannot be made
/// again for a second holder; a file opened the way earlier builds open it
/// still contends with it as busy, in either order.
#[cfg(windows)]
#[test]
fn a_held_lock_file_cannot_be_deleted_and_an_earlier_builds_open_contends_as_busy() {
    use std::fs::OpenOptions;
    let directory = tempfile::tempdir().unwrap();
    let set = paths(directory.path(), &["lease.lock"]);
    let held = try_lock_all(&set, LockMode::Exclusive).unwrap();
    // ERROR_SHARING_VIOLATION, not any error
    assert_eq!(
        fs::remove_file(&set[0])
            .err()
            .and_then(|error| error.raw_os_error()),
        Some(32)
    );
    let earlier = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&set[0])
        .expect("an earlier build's open is compatible");
    assert!(matches!(earlier.try_lock(), Err(TryLockError::WouldBlock)));
    drop(held);
    earlier.try_lock().unwrap();
    assert_eq!(
        try_lock_all(&set, LockMode::Exclusive).err(),
        Some(LockSetError::Busy)
    );
}
