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

/// A copy's journal with a batch that Windows did not locate, beside a journal
/// of this side it did, may be this side's own: refused under its own cause,
/// never as another journal's batches. One without a batch is still no refusal.
#[test]
fn a_copy_journal_not_located_is_refused_under_its_own_cause() {
    let at = layout();
    fs::write(at.data.join(JOURNAL_FILE), journal(false)).unwrap();
    fs::write(at.copy.join(JOURNAL_FILE), journal(true)).unwrap();
    let copy_journal = at.copy.join(JOURNAL_FILE);
    let unlocated = |path: &Path| (path != copy_journal).then(|| path.to_path_buf());
    assert_eq!(at.admit(&unlocated), Err(SplitJournal::CopyNotLocated));
    fs::write(at.copy.join(JOURNAL_FILE), b"").unwrap();
    assert_eq!(at.admit(&unlocated), Ok(OneJournal::NoOtherJournal));
}

/// Whether this side's journal exists could not be read (here its folder is a
/// file): unknown, so refused as not located, never taken as absent.
#[cfg(unix)]
#[test]
fn a_journal_whose_presence_cannot_be_read_is_not_located() {
    let at = layout();
    fs::remove_dir_all(&at.data).unwrap();
    fs::write(&at.data, b"").unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OwnNotLocated));
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
        SplitJournal::CopyNotLocated,
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

/// The second Store package's copy of the data folder, beside the first the
/// layout made (package folders are read in name order).
fn second_copy(at: &Layout) -> PathBuf {
    let copy = at
        .local
        .join("Packages")
        .join("Store.App_2")
        .join("LocalCache")
        .join("Local")
        .join("Bridge")
        .join("agent");
    fs::create_dir_all(&copy).unwrap();
    copy
}

/// Outside Claude Desktop, but Windows does not say where `hidden` is.
fn not_saying(hidden: PathBuf) -> impl Fn(&Path) -> Option<PathBuf> {
    move |path: &Path| {
        if path == hidden {
            None
        } else {
            outside(path)
        }
    }
}

/// Each refusal is the one code with its own cause, written out in full.
#[test]
fn each_refusal_names_its_code_and_cause_in_full() {
    let causes = [
        (
            SplitJournal::OtherHoldsBatches,
            "other_journal_holds_batches",
        ),
        (SplitJournal::OtherNotRead, "other_journal_not_read"),
        (SplitJournal::PackagesNotListed, "package_copies_not_listed"),
        (SplitJournal::OwnNotLocated, "journal_not_located"),
        (
            SplitJournal::CopyNotLocated,
            "package_copy_journal_not_located",
        ),
    ];
    assert_eq!(IMPORT_JOURNAL_SPLIT, "import_journal_split");
    for (split, cause) in causes {
        assert_eq!(split.cause(), cause);
        let failure = super::super::ToolFailure::from(split);
        assert_eq!(failure.code, "import_journal_split");
        assert_eq!(failure.cause, Some(cause));
    }
}

/// The message follows what the check found: no way to tell, for the three
/// causes that found no second journal; where to post from, for the rest.
#[test]
fn the_message_follows_what_the_check_found() {
    for cause in [
        "package_copies_not_listed",
        "journal_not_located",
        "package_copy_journal_not_located",
    ] {
        assert_eq!(refused_message(Some(cause)), NOT_TOLD_MESSAGE, "{cause}");
    }
    for cause in ["other_journal_holds_batches", "other_journal_not_read"] {
        assert_eq!(refused_message(Some(cause)), REFUSED_MESSAGE, "{cause}");
    }
    assert_eq!(refused_message(None), REFUSED_MESSAGE);
    assert!(!REFUSED_MESSAGE.is_empty());
    assert!(!NOT_TOLD_MESSAGE.is_empty());
    assert_ne!(REFUSED_MESSAGE, NOT_TOLD_MESSAGE);
}

/// Every package copy is looked at, in name order: the first refusal found
/// is the one given, whichever copy holds it.
#[test]
fn every_package_copy_is_looked_at_in_name_order() {
    let at = layout();
    let second = second_copy(&at);
    let (first, batches, garbage) = (
        at.copy.join(JOURNAL_FILE),
        journal(true),
        b"not a journal\n".to_vec(),
    );
    assert_eq!(at.admit(&outside), Ok(OneJournal::NoOtherJournal));
    fs::write(second.join(JOURNAL_FILE), &batches).unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OtherHoldsBatches));
    fs::write(&first, &batches).unwrap();
    fs::remove_file(second.join(JOURNAL_FILE)).unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OtherHoldsBatches));
    fs::write(second.join(JOURNAL_FILE), &garbage).unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OtherHoldsBatches));
    fs::write(&first, &garbage).unwrap();
    fs::write(second.join(JOURNAL_FILE), &batches).unwrap();
    assert_eq!(at.admit(&outside), Err(SplitJournal::OtherNotRead));
}

/// With no journal of this side, a copy's journal Windows did not locate is
/// still read, and refused for what it holds: there is no journal of this
/// side it could be.
#[test]
fn a_copy_journal_not_located_is_read_when_this_side_has_none() {
    let at = layout();
    let hidden = not_saying(at.copy.join(JOURNAL_FILE));
    fs::write(at.copy.join(JOURNAL_FILE), journal(true)).unwrap();
    assert_eq!(at.admit(&hidden), Err(SplitJournal::OtherHoldsBatches));
    fs::write(at.copy.join(JOURNAL_FILE), b"not a journal\n").unwrap();
    assert_eq!(at.admit(&hidden), Err(SplitJournal::OtherNotRead));
}

/// Beside a journal of this side, a copy's journal that does not read, or that
/// is absent, and that Windows did not locate: the first is refused under the
/// not-located cause, the second is no refusal.
#[test]
fn an_unread_copy_journal_not_located_is_refused_and_an_absent_one_is_not() {
    let at = layout();
    fs::write(at.data.join(JOURNAL_FILE), journal(false)).unwrap();
    assert_eq!(at.admit(&outside), Ok(OneJournal::NoOtherJournal));
    fs::write(at.copy.join(JOURNAL_FILE), b"not a journal\n").unwrap();
    let hidden = not_saying(at.copy.join(JOURNAL_FILE));
    assert_eq!(at.admit(&hidden), Err(SplitJournal::CopyNotLocated));
}

/// A package copy whose journal is this side's own file under another name
/// (a link here, as Windows redirects it) is this side's: not read, not
/// refused. Held through `admit`, whose resolver is the operating system's.
#[cfg(unix)]
#[test]
fn a_copy_journal_that_is_this_sides_file_is_this_sides() {
    let at = layout();
    fs::write(at.data.join(JOURNAL_FILE), journal(true)).unwrap();
    std::os::unix::fs::symlink(at.data.join(JOURNAL_FILE), at.copy.join(JOURNAL_FILE)).unwrap();
    let admitted = LOCAL_APP_DATA.sync_scope(at.local.clone(), || admit(&at.data));
    assert_eq!(admitted, Ok(OneJournal::NoOtherJournal));
}

/// Whether this side's journal exists could not be read for want of access
/// (not for want of the file): unknown, so refused as not located.
#[cfg(unix)]
#[test]
fn a_journal_in_a_folder_that_denies_access_is_not_located() {
    use std::os::unix::fs::PermissionsExt;
    struct Reopen(PathBuf);
    impl Drop for Reopen {
        fn drop(&mut self) {
            let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o755));
        }
    }
    let at = layout();
    fs::write(at.data.join(JOURNAL_FILE), journal(false)).unwrap();
    let _reopen = Reopen(at.data.clone());
    fs::set_permissions(&at.data, fs::Permissions::from_mode(0o000)).unwrap();
    // The premise: this process is refused, as an ordinary user is.
    let kind = fs::symlink_metadata(at.data.join(JOURNAL_FILE))
        .unwrap_err()
        .kind();
    assert_eq!(kind, std::io::ErrorKind::PermissionDenied);
    assert_eq!(at.admit(&outside), Err(SplitJournal::OwnNotLocated));
}

/// Off Windows the host gives no `%LOCALAPPDATA%`, so a run here cannot show
/// whether `admit` asks the host for one when no test has scripted it.
#[cfg(not(windows))]
#[test]
fn off_windows_the_host_has_no_local_app_data() {
    assert_eq!(local_data::local_app_data(), None);
}

/// On Windows, with no `%LOCALAPPDATA%` scripted, `admit` asks the host for
/// it: a data folder named under the host's own that does not exist is looked
/// for among the package copies, not passed as never redirected. Nothing is
/// written; the packages folder is only listed.
#[cfg(windows)]
#[test]
fn on_windows_admit_asks_the_host_for_local_app_data() {
    let host = local_data::local_app_data().expect("LOCALAPPDATA is set on Windows");
    let data = host.join("BridgeSplitJournalTestAbsent").join("agent");
    assert!(!data.exists());
    assert_ne!(admit(&data), Ok(OneJournal::NotRedirected));
}
