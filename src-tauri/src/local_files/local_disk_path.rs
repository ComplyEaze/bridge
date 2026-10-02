//! A caller-supplied file path, admitted by its text alone before anything is opened.
use std::path::{Path, PathBuf};

/// Which system's path rule a text is checked against. The check is on the
/// text and does not touch the filesystem, so both rules can be tested on
/// any host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PathRule {
    /// A drive-letter root: `C:\...` or `C:/...`.
    Windows,
    /// A single leading `/`.
    Unix,
}

impl PathRule {
    pub(crate) const fn host() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else {
            Self::Unix
        }
    }
}

/// Why a path text was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LocalDiskPathRefusal {
    /// The text begins with two separators (`\\host\share`, `//host/share`,
    /// `\\?\...`, `\\.\...`, `\\host@80\...`, or any mix of the two
    /// separators). No such text names a file on a local disk by a drive
    /// letter or a single root, so it is refused on every system.
    DoubleSeparatorPrefix,
    /// The text is not rooted on a local disk for the rule in force: on
    /// Windows anything but a drive letter, a colon and a separator; elsewhere
    /// anything but a single leading `/`.
    NotALocalRoot,
}

/// A path that passed [`LocalDiskPath::parse_for`]. The only way to obtain
/// one is from a text, so a function taking `&LocalDiskPath` cannot be handed
/// a path nobody checked.
///
/// This is a check on the text. It does not establish that the drive is a
/// fixed disk (a mapped drive letter can still name a share), that no
/// component is a link or a reparse point, or that the path names the same
/// file between this check and the open.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalDiskPath(PathBuf);

fn is_separator(character: char) -> bool {
    character == '\\' || character == '/'
}

impl LocalDiskPath {
    pub(crate) fn parse(text: &str) -> Result<Self, LocalDiskPathRefusal> {
        Self::parse_for(text, PathRule::host())
    }

    pub(crate) fn parse_for(text: &str, rule: PathRule) -> Result<Self, LocalDiskPathRefusal> {
        let mut characters = text.chars();
        let first = characters.next();
        let second = characters.next();
        let third = characters.next();
        if first.is_some_and(is_separator) && second.is_some_and(is_separator) {
            return Err(LocalDiskPathRefusal::DoubleSeparatorPrefix);
        }
        let rooted = match rule {
            PathRule::Windows => {
                first.is_some_and(|character| character.is_ascii_alphabetic())
                    && second == Some(':')
                    && third.is_some_and(is_separator)
            }
            PathRule::Unix => first == Some('/'),
        };
        if rooted {
            Ok(Self(PathBuf::from(text)))
        } else {
            Err(LocalDiskPathRefusal::NotALocalRoot)
        }
    }

    pub(crate) fn as_path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
#[path = "local_disk_path_tests.rs"]
mod tests;
