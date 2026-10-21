//! The location of a file or folder in Bridge's data folder, as a person sees it.
//!
//! Bridge builds every data path from `%LOCALAPPDATA%` (or the known-folder
//! equivalent). When the process runs with package identity, as the MCP server
//! does under a Microsoft Store (MSIX) install of Claude Desktop, Windows
//! redirects a write under `%LOCALAPPDATA%` into the package's private
//! `LocalCache\Local` folder, per file. The path Bridge built then does not exist
//! outside the package, and File Explorer and Tally do not find a file there
//! (#1459). The redirection happens when a file is opened, so the logical path
//! is still what Bridge opens and stores. Only the path shown to a person is
//! resolved here, to the location the operating system reports for the file.
use std::path::{Path, PathBuf};

/// A data path as it is shown to a person, with how it was obtained.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum VisiblePath {
    /// Windows: the real location the operating system reports for a path that
    /// exists, with the verbatim `\\?\` prefix removed for display.
    #[cfg_attr(
        not(windows),
        allow(dead_code, reason = "only Windows resolves a path through the OS")
    )]
    Resolved(PathBuf),
    /// Not Windows, the path exists, and it is shown as given. macOS and Linux
    /// have no per-file redirection, and resolving links would turn `/var` into
    /// `/private/var` for no benefit.
    #[cfg_attr(
        windows,
        allow(
            dead_code,
            reason = "only a host with no redirection returns a path as it is"
        )
    )]
    AsIs(PathBuf),
    /// Nothing at this path could be resolved (it does not exist yet, or could
    /// not be opened for resolution). It is the logical path Bridge uses, and a
    /// guess at where the file will appear: say so, do not present it as found.
    Unresolved(PathBuf),
}

impl VisiblePath {
    pub(crate) fn path(&self) -> &Path {
        match self {
            Self::Resolved(path) | Self::AsIs(path) | Self::Unresolved(path) => path,
        }
    }

    /// The word a JSON result carries beside the path.
    pub(crate) fn state(&self) -> &'static str {
        match self {
            Self::Resolved(_) => "resolved",
            Self::AsIs(_) => "as_is",
            Self::Unresolved(_) => "unresolved",
        }
    }

    pub(crate) fn display_string(&self) -> String {
        self.path().display().to_string()
    }

    /// Write `key` (the path) and `{key}_state` (how it was obtained) into a
    /// JSON object.
    pub(crate) fn put_into(&self, object: &mut serde_json::Value, key: &str) {
        object[key] = serde_json::json!(self.display_string());
        object[format!("{key}_state")] = serde_json::json!(self.state());
    }
}

/// `path` as a person should look for it. Use it only where a path is shown;
/// open and store by the logical path.
///
/// On Windows this is `std::fs::canonicalize`, which asks the operating system
/// for the final path of the open file (`GetFinalPathNameByHandleW`), so a
/// redirected file is reported where it really is. Nothing on a Mac or Linux
/// redirects a file, so there the path is returned unchanged.
pub(crate) fn user_visible_path(path: &Path) -> VisiblePath {
    #[cfg(windows)]
    {
        from_resolution(path, std::fs::canonicalize(path))
    }
    #[cfg(not(windows))]
    {
        unredirected(path)
    }
}

#[cfg(any(windows, test))]
fn from_resolution(path: &Path, resolved: std::io::Result<PathBuf>) -> VisiblePath {
    match resolved {
        Ok(real) => VisiblePath::Resolved(strip_verbatim_prefix(real)),
        Err(_) => VisiblePath::Unresolved(path.to_path_buf()),
    }
}

#[cfg(not(windows))]
fn unredirected(path: &Path) -> VisiblePath {
    if path.exists() {
        VisiblePath::AsIs(path.to_path_buf())
    } else {
        VisiblePath::Unresolved(path.to_path_buf())
    }
}

/// `\\?\C:\x` becomes `C:\x` and `\\?\UNC\server\share\x` becomes
/// `\\server\share\x`. The verbatim form is kept when there is no plain
/// equivalent (`\\?\Volume{...}\x`) or the plain one would not open: the
/// verbatim prefix is what lets Windows take a long path or a name that ends in
/// a dot or space or is a reserved device name.
#[cfg(any(windows, test))]
fn strip_verbatim_prefix(path: PathBuf) -> PathBuf {
    match path.to_str().and_then(plain_form) {
        Some(plain) => PathBuf::from(plain),
        None => path,
    }
}

/// The plain form of a verbatim path, if it has one that is usable.
#[cfg(any(windows, test))]
fn plain_form(text: &str) -> Option<String> {
    let rest = text.strip_prefix(r"\\?\")?;
    let plain = if rest
        .get(..4)
        .is_some_and(|head| head.eq_ignore_ascii_case(r"UNC\"))
    {
        let share = &rest[4..];
        if share.is_empty() {
            return None;
        }
        format!(r"\\{share}")
    } else {
        let bytes = rest.as_bytes();
        let drive = bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
        if !(drive && (bytes.len() == 2 || bytes[2] == b'\\')) {
            return None;
        }
        rest.to_string()
    };
    usable_without_prefix(&plain).then_some(plain)
}

/// Whether Win32 opens `plain` without the verbatim prefix: it is shorter than
/// the 248-character directory limit, and no component ends in a dot or a space
/// or is a reserved device name (CON, PRN, AUX, NUL, COM1-9, LPT1-9, with or
/// without an extension, in any case).
#[cfg(any(windows, test))]
fn usable_without_prefix(plain: &str) -> bool {
    const RESERVED: [&str; 4] = ["CON", "PRN", "AUX", "NUL"];
    if plain.chars().count() >= 248 {
        return false;
    }
    plain
        .split('\\')
        .filter(|component| !component.is_empty())
        .all(|component| {
            if component.ends_with('.') || component.ends_with(' ') {
                return false;
            }
            let stem = component.split('.').next().unwrap_or(component);
            let stem = stem.to_ascii_uppercase();
            let numbered = |prefix: &str| {
                stem.strip_prefix(prefix)
                    .is_some_and(|digit| matches!(digit.as_bytes(), [b'1'..=b'9']))
            };
            !(RESERVED.contains(&stem.as_str()) || numbered("COM") || numbered("LPT"))
        })
}

#[cfg(test)]
#[path = "visible_path_tests.rs"]
mod tests;
