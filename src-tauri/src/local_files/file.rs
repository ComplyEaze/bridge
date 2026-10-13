//! Open local artifact leaves without following aliases or mutating them first.
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub(crate) fn open_local_file(path: &Path, writable: bool) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options
        .read(true)
        .write(writable)
        .create(writable)
        .truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        // Nonblocking admission prevents a non-regular leaf from blocking open.
        options
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    let file = options.open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "local artifact is not a regular file",
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        // SAFETY: geteuid only reads the process's effective user identifier.
        if metadata.uid() != unsafe { libc::geteuid() } || metadata.nlink() != 1 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "local artifact ownership is not exclusive",
            ));
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Storage::FileSystem::{
            GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION, FILE_ATTRIBUTE_REPARSE_POINT,
        };
        let mut info = BY_HANDLE_FILE_INFORMATION::default();
        // SAFETY: File owns a live handle; info is writable for the complete struct.
        if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut info) } == 0 {
            return Err(io::Error::last_os_error());
        }
        if info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 || info.nNumberOfLinks != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "local artifact is not an exclusive regular file",
            ));
        }
    }
    Ok(file)
}

/// Create a private file at a name nothing holds yet. A name that exists, a
/// link or an alias included, fails with `AlreadyExists` and is left as it is.
pub(crate) fn create_new_local_file(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)
}

/// Rename `from` to `to` only if nothing holds `to`, in one step, so a record
/// never has two names (a second link makes `open_local_file` refuse it); a
/// taken `to` fails with `AlreadyExists` and both files stay as they were.
/// Where the platform has no such rename, a hard link and an unlink stand in,
/// and a failed unlink removes the new name again.
pub(crate) fn rename_new(from: &Path, to: &Path) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;
        let from = CString::new(from.as_os_str().as_bytes())?;
        let to = CString::new(to.as_os_str().as_bytes())?;
        // SAFETY: both paths are NUL-terminated and outlive the call.
        if unsafe { libc::renamex_np(from.as_ptr(), to.as_ptr(), libc::RENAME_EXCL) } == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_WRITE_THROUGH};
        let wide = |path: &Path| {
            path.as_os_str()
                .encode_wide()
                .chain(Some(0))
                .collect::<Vec<u16>>()
        };
        let (from, to) = (wide(from), wide(to));
        // Without MOVEFILE_REPLACE_EXISTING a taken name fails the move.
        // SAFETY: both paths are NUL-terminated and outlive the call.
        if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), MOVEFILE_WRITE_THROUGH) } != 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::fs::hard_link(from, to)?;
        std::fs::remove_file(from).inspect_err(|_| {
            let _ = std::fs::remove_file(to);
        })
    }
}

/// A file's identity on its volume: the inode on Unix, the file index on
/// Windows. A file replaced by another, even with the same bytes, has another.
#[cfg(test)]
pub(crate) fn file_identity(path: &Path) -> u64 {
    #[cfg(unix)]
    {
        std::os::unix::fs::MetadataExt::ino(&std::fs::metadata(path).unwrap())
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Storage::FileSystem::{
            GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
        };
        let file = File::open(path).unwrap();
        let mut info = BY_HANDLE_FILE_INFORMATION::default();
        // SAFETY: File owns a live handle; info is writable for the complete struct.
        assert_ne!(
            unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut info) },
            0
        );
        (u64::from(info.nFileIndexHigh) << 32) | u64::from(info.nFileIndexLow)
    }
}

pub(crate) fn lock_error(error: std::fs::TryLockError) -> String {
    match error {
        std::fs::TryLockError::WouldBlock => "import_admission_busy",
        std::fs::TryLockError::Error(_) => "import_admission_lock_unavailable",
    }
    .into()
}
