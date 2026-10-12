//! Lock files taken together, in one order, never waiting (#1584).
//!
//! One lock can live in more than one folder: on Windows a packaged process
//! and an unpackaged one see different AppData folders, so each lock is taken
//! in every folder a build of ComplyEaze Bridge may use. Every file is opened
//! first, holding nothing, so no open can stall while a lock is held. Then each
//! is try-locked in the order given. A busy file releases every lock already
//! taken, explicitly rather than by closing the handle (Windows frees a closed
//! handle's lock only after a delay), before the result returns, so a caller
//! that pauses and tries again never waits holding a lock. Two names for one
//! file are taken once: a second lock on a file this process already holds
//! would refuse it for good.
use super::file::open_lock_file;
use std::fs::{File, TryLockError};
use std::io;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LockMode {
    Shared,
    Exclusive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LockSetError {
    /// Another holder has one of the files.
    Busy,
    /// A file could not be opened, identified or locked.
    Unavailable,
}

/// Every lock of a set, released when dropped.
pub(crate) struct HeldLocks {
    files: Vec<File>,
}

impl Drop for HeldLocks {
    fn drop(&mut self) {
        for file in &self.files {
            // Closing the handle releases it too; this only makes it prompt.
            let _ = file.unlock();
        }
    }
}

/// Open each of `paths`, then lock each in order, all or none.
pub(crate) fn try_lock_all(paths: &[PathBuf], mode: LockMode) -> Result<HeldLocks, LockSetError> {
    let mut files: Vec<File> = Vec::with_capacity(paths.len());
    let mut seen: Vec<FileIdentity> = Vec::with_capacity(paths.len());
    for path in paths {
        let file = open_lock_file(path).map_err(|_| LockSetError::Unavailable)?;
        let identity = file_identity(&file).map_err(|_| LockSetError::Unavailable)?;
        if seen.contains(&identity) {
            continue;
        }
        seen.push(identity);
        files.push(file);
    }
    let mut held = HeldLocks {
        files: Vec::with_capacity(files.len()),
    };
    for file in files {
        let taken = match mode {
            LockMode::Shared => file.try_lock_shared(),
            LockMode::Exclusive => file.try_lock(),
        };
        match taken {
            Ok(()) => held.files.push(file),
            // `held` is dropped here, releasing what it took
            Err(TryLockError::WouldBlock) => return Err(LockSetError::Busy),
            Err(TryLockError::Error(_)) => return Err(LockSetError::Unavailable),
        }
    }
    Ok(held)
}

/// What names one file on one volume, whatever path reached it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FileIdentity {
    volume: u64,
    file: u128,
}

#[cfg(unix)]
fn file_identity(file: &File) -> io::Result<FileIdentity> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file.metadata()?;
    Ok(FileIdentity {
        volume: metadata.dev(),
        file: u128::from(metadata.ino()),
    })
}

/// The volume serial number and 128-bit file ID (the 64-bit file index is not
/// unique on ReFS). A lock file on a remote volume is refused: a lock taken
/// over the network is not one this design relies on.
#[cfg(windows)]
fn file_identity(file: &File) -> io::Result<FileIdentity> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        FileIdInfo, FileRemoteProtocolInfo, GetFileInformationByHandleEx, FILE_ID_INFO,
        FILE_REMOTE_PROTOCOL_INFO,
    };
    let handle = file.as_raw_handle();
    let mut remote = std::mem::MaybeUninit::<FILE_REMOTE_PROTOCOL_INFO>::zeroed();
    // SAFETY: the handle is live for the call and the buffer is writable for
    // the whole struct, whose size is passed. The call succeeds only for a
    // file on a remote volume.
    let is_remote = unsafe {
        GetFileInformationByHandleEx(
            handle,
            FileRemoteProtocolInfo,
            remote.as_mut_ptr().cast(),
            std::mem::size_of::<FILE_REMOTE_PROTOCOL_INFO>() as u32,
        )
    } != 0;
    if is_remote {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "lock file on a remote volume",
        ));
    }
    let mut info = std::mem::MaybeUninit::<FILE_ID_INFO>::zeroed();
    // SAFETY: as above; on success the struct is fully written.
    if unsafe {
        GetFileInformationByHandleEx(
            handle,
            FileIdInfo,
            info.as_mut_ptr().cast(),
            std::mem::size_of::<FILE_ID_INFO>() as u32,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: the call succeeded and wrote the whole struct.
    let info = unsafe { info.assume_init() };
    Ok(FileIdentity {
        volume: info.VolumeSerialNumber,
        file: u128::from_le_bytes(info.FileId.Identifier),
    })
}

#[cfg(test)]
#[path = "lock_set_tests.rs"]
mod tests;
