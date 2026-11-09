//! Open local artifact leaves without following aliases or mutating them first.
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub(crate) fn open_local_file(path: &Path, writable: bool) -> io::Result<File> {
    open_leaf(path, writable, false)
}

/// Open, creating it if needed, a lock file that is taken with a file lock. On
/// Windows it is shared for reading and writing only: a lock file another
/// process holds cannot then be deleted or renamed and made again under the
/// same name, which would give that name a second holder. Opens that share
/// everything, as earlier builds make, are compatible with it either way.
pub(crate) fn open_lock_file(path: &Path) -> io::Result<File> {
    open_leaf(path, true, true)
}

fn open_leaf(path: &Path, writable: bool, lock: bool) -> io::Result<File> {
    #[cfg(not(windows))]
    let _ = lock;
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
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ, FILE_SHARE_WRITE,
        };
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
        if lock {
            options.share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE);
        }
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

/// Write a whole file readable only by this user, replacing what `path` held.
/// A link or alias at `path` is refused, as by [`open_local_file`].
pub(crate) fn write_private_file(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut file = open_local_file(path, true)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    file.set_len(0)?;
    file.write_all(bytes)?;
    file.sync_all()
}

/// Write `bytes` privately to `staged`, then rename it onto `path`, so a reader
/// finds the old file or the whole new one and never a part. The stage is a
/// sibling of `path` that the caller names. It is removed if the rename fails.
pub(crate) fn replace_private_file(staged: &Path, path: &Path, bytes: &[u8]) -> io::Result<()> {
    let replaced = write_private_file(staged, bytes).and_then(|()| std::fs::rename(staged, path));
    if replaced.is_err() {
        let _ = std::fs::remove_file(staged);
    }
    replaced
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_replacement_that_cannot_land_leaves_no_stage_and_the_old_name_as_it_was() {
        let directory = tempfile::tempdir().unwrap();
        let taken = directory.path().join("record");
        std::fs::create_dir(&taken).unwrap();
        std::fs::write(taken.join("inside"), b"kept").unwrap();
        let staged = directory.path().join("record.next");

        assert!(replace_private_file(&staged, &taken, b"new").is_err());

        assert!(!staged.exists());
        assert_eq!(std::fs::read(taken.join("inside")).unwrap(), b"kept");
    }

    #[test]
    fn a_longer_stage_left_by_an_earlier_write_does_not_trail_the_new_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("record");
        let staged = directory.path().join("record.next");
        std::fs::write(&staged, b"an earlier, longer record").unwrap();

        replace_private_file(&staged, &path, b"new").unwrap();

        assert_eq!(std::fs::read(&path).unwrap(), b"new");
        assert!(!staged.exists());
    }

    #[cfg(unix)]
    #[test]
    fn a_replaced_file_is_private_whatever_the_old_one_was() {
        use std::os::unix::fs::PermissionsExt;
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("record");
        std::fs::write(&path, b"old").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();

        replace_private_file(&directory.path().join("record.next"), &path, b"new").unwrap();

        let mode = std::fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
}
