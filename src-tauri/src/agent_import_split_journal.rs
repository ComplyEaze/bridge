//! A second import journal in a Store package's copy of the data folder (#2233).
//!
//! On a Microsoft Store install of Claude Desktop, Windows keeps the files a
//! process inside Claude Desktop creates under `%LOCALAPPDATA%` in the package's
//! own copy of the data folder. That process and one outside Claude Desktop then
//! each keep an import journal, and no journal check of one sees the other's
//! batches (docs/security-and-privacy.md, section 4). Until one folder serves
//! both, a build or a post is refused while a journal other than this side's
//! holds a batch. Neither journal is merged, moved or deleted.
//!
//! Only the package copies can be looked at by name. Seen from inside Claude
//! Desktop, a journal at `%LOCALAPPDATA%\Bridge\agent` is hidden behind the
//! package's own copy once that copy holds one, so there only another package's
//! copy is found; the side outside Claude Desktop finds the package's.
use std::path::{Path, PathBuf};

use super::local_data::{self, Journal, PackageCopies, JOURNAL_FILE};

/// The import journal of this side's data folder is the only one found.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum OneJournal {
    /// Off Windows, or a data folder outside `%LOCALAPPDATA%`: nothing was looked
    /// for. Windows also keeps package copies of `%APPDATA%` and of `LocalLow`, so
    /// a `BRIDGE_AGENT_DATA_DIR` there leaves this check inert, not satisfied.
    NotRedirected,
    /// No package copy of the data folder holds a journal other than this side's.
    NoOtherJournal,
}

/// A second import journal, or no way to tell whether there is one. Each is a
/// refusal of the build or the post; the cause names which.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SplitJournal {
    /// A package copy of the data folder holds a journal, other than this
    /// side's, with at least one batch.
    OtherHoldsBatches,
    /// A package copy holds a journal, other than this side's, that could not
    /// be read as one.
    OtherNotRead,
    /// The Store apps' packages folder exists and could not be listed.
    PackagesNotListed,
    /// This side's journal exists, or whether it exists could not be read, and
    /// Windows did not say where it is, so it cannot be told apart from a
    /// package copy's.
    OwnNotLocated,
    /// A package copy holds a journal with a batch, or one that could not be
    /// read, and Windows did not say where that journal is, so it cannot be
    /// told apart from this side's own.
    CopyNotLocated,
}

/// The refusal code of every [`SplitJournal`]; the cause says which.
pub(super) const IMPORT_JOURNAL_SPLIT: &str = "import_journal_split";

impl SplitJournal {
    pub(super) fn cause(self) -> &'static str {
        match self {
            Self::OtherHoldsBatches => "other_journal_holds_batches",
            Self::OtherNotRead => "other_journal_not_read",
            Self::PackagesNotListed => "package_copies_not_listed",
            Self::OwnNotLocated => "journal_not_located",
            Self::CopyNotLocated => "package_copy_journal_not_located",
        }
    }
}

impl From<SplitJournal> for super::ToolFailure {
    fn from(split: SplitJournal) -> Self {
        let mut failure = Self::from(IMPORT_JOURNAL_SPLIT.to_string());
        failure.cause = Some(split.cause());
        failure
    }
}

/// Whether a build or a post from `data_dir` may go on: refused while a package
/// copy of that folder holds another import journal. Inert off Windows.
pub(super) fn admit(data_dir: &Path) -> Result<OneJournal, SplitJournal> {
    #[cfg(test)]
    let scripted = LOCAL_APP_DATA.try_with(Clone::clone).ok();
    #[cfg(not(test))]
    let scripted = None;
    let Some(local_app_data) = scripted.or_else(local_data::local_app_data) else {
        return Ok(OneJournal::NotRedirected);
    };
    admit_at(&local_app_data, data_dir, &|path| {
        std::fs::canonicalize(path).ok()
    })
}

#[cfg(test)]
tokio::task_local! {
    /// Test-only: `%LOCALAPPDATA%` as Windows would give it, so a build or a
    /// post on any host meets a package copy a test laid out.
    pub(super) static LOCAL_APP_DATA: PathBuf;
}

/// [`admit`] with `%LOCALAPPDATA%` and the resolver given, so a test can stand
/// in for Windows. `resolve` is where the operating system says an existing
/// file is (`None` when it does not say); a package copy whose journal resolves
/// to this side's own journal is this side's.
fn admit_at(
    local_app_data: &Path,
    data_dir: &Path,
    resolve: &dyn Fn(&Path) -> Option<PathBuf>,
) -> Result<OneJournal, SplitJournal> {
    let copies = match local_data::package_copy_folders(local_app_data, data_dir) {
        PackageCopies::NotUnderLocalAppData => return Ok(OneJournal::NotRedirected),
        PackageCopies::NotListed => return Err(SplitJournal::PackagesNotListed),
        PackageCopies::Found(copies) => copies,
    };
    let own_path = data_dir.join(JOURNAL_FILE);
    let own = resolve(&own_path);
    if own.is_none() {
        // Only "not found" says there is no journal of this side; any other
        // error leaves it unknown, and unknown is refused.
        match std::fs::symlink_metadata(&own_path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            _ => return Err(SplitJournal::OwnNotLocated),
        }
    }
    for copy in copies {
        let located = resolve(&copy.join(JOURNAL_FILE));
        if own.is_some() && located == own {
            continue;
        }
        // With a journal of this side to compare against, a copy's journal
        // Windows did not locate might be that same journal.
        let unlocated = own.is_some() && located.is_none();
        match local_data::read_journal(&copy, || {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }) {
            Journal::Absent => {}
            Journal::Read(settlement) if settlement.batches == 0 => {}
            _ if unlocated => return Err(SplitJournal::CopyNotLocated),
            Journal::Read(_) => return Err(SplitJournal::OtherHoldsBatches),
            Journal::NotRead(_) => return Err(SplitJournal::OtherNotRead),
        }
    }
    Ok(OneJournal::NoOtherJournal)
}

#[cfg(test)]
#[path = "agent_import_split_journal_tests.rs"]
pub(super) mod tests;
