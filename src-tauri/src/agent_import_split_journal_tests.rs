//! The second import journal of a Store install (#2233), with `%LOCALAPPDATA%`
//! and the resolver standing in for Windows as the lab observed it (#2231).
use super::*;
use crate::agent::agent_import::local_data::tests::journal;
use std::fs;

/// `%LOCALAPPDATA%`, the data folder under it, and the one Store package's copy
/// of that folder, as on the lab's computer.
pub(in crate::agent::agent_import) struct Layout {
    _directory: tempfile::TempDir,
    pub(in crate::agent::agent_import) local: PathBuf,
    pub(in crate::agent::agent_import) data: PathBuf,
    pub(in crate::agent::agent_import) copy: PathBuf,
}

pub(in crate::agent::agent_import) fn layout() -> Layout {
    let directory = tempfile::tempdir().unwrap();
    let local = directory.path().join("Local");
    let data = local.join("Bridge").join("agent");
    let copy = local
        .join("Packages")
        .join("Store.App_1")
        .join("LocalCache")
        .join("Local")
        .join("Bridge")
        .join("agent");
    fs::create_dir_all(&data).unwrap();
    fs::create_dir_all(&copy).unwrap();
    Layout {
        _directory: directory,
        local,
        data,
        copy,
    }
}

/// Outside Claude Desktop: each file is where it was built.
fn outside(path: &Path) -> Option<PathBuf> {
    path.exists().then(|| path.to_path_buf())
}

impl Layout {
    /// Inside Claude Desktop, once the package copy holds a journal: the data
    /// folder's journal is the copy's (#2231's probe).
    fn inside(&self) -> impl Fn(&Path) -> Option<PathBuf> + '_ {
        move |path: &Path| {
            if path == self.data.join(JOURNAL_FILE) {
                outside(&self.copy.join(JOURNAL_FILE))
            } else {
                outside(path)
            }
        }
    }

    fn admit(
        &self,
        resolve: &dyn Fn(&Path) -> Option<PathBuf>,
    ) -> Result<OneJournal, SplitJournal> {
        admit_at(&self.local, &self.data, resolve)
    }
}

/// The lab's case, outside: the package copy holds batches, so a build or a
/// post here is refused, with or without a journal of its own.
#[test]
fn outside_a_package_copy_with_a_batch_is_a_second_journal() {
    let at = layout();
    fs::write(at.copy.join(JOURNAL_FILE), journal(true)).unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OtherHoldsBatches));
    fs::write(at.data.join(JOURNAL_FILE), journal(false)).unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OtherHoldsBatches));
}

/// Inside, the package copy's journal is this side's own: no refusal.
#[test]
fn inside_the_package_copy_is_this_sides_own_journal() {
    let at = layout();
    fs::write(at.copy.join(JOURNAL_FILE), journal(true)).unwrap();
    assert_eq!(at.admit(&at.inside()), Ok(OneJournal::NoOtherJournal));
}

/// A copy with no journal, or one with no batch, is not a second journal; nor
/// is a computer with no packages folder.
#[test]
fn a_copy_with_no_batch_is_not_a_second_journal() {
    let at = layout();
    assert_eq!(at.admit(&outside), Ok(OneJournal::NoOtherJournal));
    fs::write(at.copy.join(JOURNAL_FILE), b"").unwrap();
    assert_eq!(at.admit(&outside), Ok(OneJournal::NoOtherJournal));
    fs::remove_dir_all(at.local.join("Packages")).unwrap();
    assert_eq!(at.admit(&outside), Ok(OneJournal::NoOtherJournal));
}

/// What cannot be told is refused, each under its own cause: a copy's journal
/// that does not read as one, and a journal of this side whose location Windows
/// did not report.
#[test]
fn what_cannot_be_told_is_refused() {
    let at = layout();
    fs::write(at.copy.join(JOURNAL_FILE), b"not a journal\n").unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OtherNotRead));

    let at = layout();
    fs::write(at.data.join(JOURNAL_FILE), journal(false)).unwrap();
    assert_eq!(at.admit(&|_| None), Err(SplitJournal::OwnNotLocated));
}

/// A packages folder that cannot be listed is refused too. A file in its place
/// stands in for it on Unix; Windows may report that as not found.
#[cfg(unix)]
#[test]
fn a_packages_folder_that_cannot_be_listed_is_refused() {
    let at = layout();
    fs::remove_dir_all(at.local.join("Packages")).unwrap();
    fs::write(at.local.join("Packages"), b"").unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::PackagesNotListed));
}

/// A data folder set outside `%LOCALAPPDATA%` has no package copy to look for.
#[test]
fn a_data_folder_elsewhere_is_not_redirected() {
    let at = layout();
    fs::write(at.copy.join(JOURNAL_FILE), journal(true)).unwrap();
    let elsewhere = at.local.parent().unwrap().join("elsewhere");
    assert_eq!(
        admit_at(&at.local, &elsewhere, &outside),
        Ok(OneJournal::NotRedirected)
    );
}

/// Off Windows nothing is looked for, whatever the folders hold.
#[cfg(not(windows))]
#[test]
fn off_windows_the_check_is_inert() {
    let at = layout();
    fs::write(at.copy.join(JOURNAL_FILE), journal(true)).unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OtherHoldsBatches));
    assert_eq!(admit(&at.data), Ok(OneJournal::NotRedirected));
}

/// Every refusal carries one code and its own cause.
#[test]
fn each_refusal_is_one_code_with_its_own_cause() {
    let causes = [
        SplitJournal::OtherHoldsBatches,
        SplitJournal::OtherNotRead,
        SplitJournal::PackagesNotListed,
        SplitJournal::OwnNotLocated,
    ]
    .map(|split| {
        let failure = super::super::ToolFailure::from(split);
        assert_eq!(failure.code, IMPORT_JOURNAL_SPLIT);
        failure.cause.unwrap()
    });
    assert_eq!(
        causes
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        causes.len()
    );
    assert!(super::super::super::refusal_remediation(IMPORT_JOURNAL_SPLIT).is_some());
}
