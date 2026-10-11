//! Local filesystem admission and per-user artifact paths.
#[cfg(any(windows, test))]
pub(crate) mod access;
pub(crate) mod directory;
pub(crate) mod file;
pub(crate) mod local_disk_path;
pub(crate) mod paths;
