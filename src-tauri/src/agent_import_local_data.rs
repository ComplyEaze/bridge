//! A read-only report of the local data Bridge keeps (`bridge_mcp
//! --local-data-report`). It changes and deletes nothing: it lists what is
//! there by class, how big it is, and whether the import journal is settled, so
//! a person can see what a later deletion would reach before any is built.
//! Content is never read except to count the journal's batches, and no path is
//! printed unless asked.
use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Files and bytes of one class.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Class {
    pub(super) files: u64,
    pub(super) bytes: u64,
}

/// What the report could establish about the folder itself. `Empty` and
/// `Unreadable` are different answers (an unreadable folder is not a clean one).
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Root {
    Missing,
    Unreadable,
    Present,
}

/// The admission lock, probed without creating it and without holding it.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Lock {
    NotPresent,
    Free,
    Busy,
    Unreadable,
}

/// The journal: read only when the admission lock could be taken shared.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Journal {
    Absent,
    NotRead(&'static str),
    Read(super::ledger::Settlement),
}

#[derive(Debug)]
pub(super) struct Report {
    pub(super) root: Root,
    pub(super) classes: BTreeMap<&'static str, Class>,
    /// The oldest modification time of a file of each class that has one.
    pub(super) oldest: BTreeMap<&'static str, SystemTime>,
    /// Symlinks and other non-regular entries: counted, never followed.
    pub(super) links: u64,
    /// Regular files with more than one hard link.
    pub(super) multi_link: u64,
    /// Entries whose metadata could not be read.
    pub(super) unreadable: u64,
    /// Publication directories left by an interrupted write: they block builds.
    pub(super) publication_dirs: u64,
    pub(super) admission_lock: Lock,
    pub(super) journal: Journal,
}

const CLASSES: [&str; 10] = [
    "journal",
    "import_files",
    "proofs",
    "review_records",
    "approval_notes",
    "bank_statements",
    "egress_log",
    "lab",
    "locks",
    "other",
];

impl Report {
    fn add(&mut self, class: &'static str, entry: &fs::Metadata) {
        let totals = self.classes.entry(class).or_default();
        totals.files += 1;
        totals.bytes += entry.len();
        if let Ok(modified) = entry.modified() {
            let oldest = self.oldest.entry(class).or_insert(modified);
            *oldest = (*oldest).min(modified);
        }
    }

    fn account(&mut self, class: &'static str, path: &Path) {
        match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.is_file() => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    if metadata.nlink() > 1 {
                        self.multi_link += 1;
                    }
                }
                self.add(class, &metadata);
            }
            Ok(_) => self.links += 1,
            Err(_) => self.unreadable += 1,
        }
    }
}

/// The class of a file directly inside `imports/`, by its name.
fn import_class(name: &str) -> &'static str {
    if name.ends_with(".xml") {
        "import_files"
    } else if name.ends_with(".proof.json") || name.ends_with(".proof.md") {
        "proofs"
    } else if name.ends_with(".approval_lapse.json") {
        "approval_notes"
    } else if name.ends_with(".masters_check.json")
        || name.ends_with(".masters_doubt.json")
        || name.ends_with(".masters_ack.json")
        || name.ends_with(".batch_step_doubt.json")
        || name.ends_with(".batch_step_ack.json")
        || name.ends_with(".baseline.json")
    {
        "review_records"
    } else {
        "other"
    }
}

/// List `dir` one level deep, giving each file `class(name)`; a subdirectory is
/// counted as `other` and not entered.
fn scan_directory(
    report: &mut Report,
    dir: &Path,
    class: impl Fn(&str) -> &'static str,
    known_directories: &[&str],
) {
    // A subfolder that is a symlink is never entered: it was counted as a link
    // by the scan of its parent (the root's own folders are the only ones that
    // can be reached that way).
    if fs::symlink_metadata(dir).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == ErrorKind::NotFound => return,
        Err(_) => {
            report.unreadable += 1;
            return;
        }
    };
    for entry in entries {
        let Ok(entry) = entry else {
            report.unreadable += 1;
            continue;
        };
        let name = entry.file_name();
        let name = name.to_string_lossy();
        match fs::symlink_metadata(entry.path()) {
            Ok(metadata) if metadata.is_dir() => {
                if name == ".proof-publication" || name == ".build-publication" {
                    report.publication_dirs += 1;
                } else if !known_directories.contains(&&*name) {
                    // Not entered: counted, with no size of its own.
                    report.classes.entry("other").or_default().files += 1;
                }
            }
            Ok(_) => report.account(class(&name), &entry.path()),
            Err(_) => report.unreadable += 1,
        }
    }
}

/// Take stock of `root` (the agent data folder) and, when given, the endpoint
/// coordination folder whose `native-dispatch-leases` holds the lease locks.
/// Reads only; the admission lock is probed shared and released at once.
pub(super) fn build(root: &Path, coordination: Option<&Path>) -> Report {
    let mut report = Report {
        root: Root::Present,
        classes: CLASSES
            .iter()
            .map(|class| (*class, Class::default()))
            .collect(),
        oldest: BTreeMap::new(),
        links: 0,
        multi_link: 0,
        unreadable: 0,
        publication_dirs: 0,
        admission_lock: Lock::NotPresent,
        journal: Journal::Absent,
    };
    match fs::read_dir(root) {
        Err(error) if error.kind() == ErrorKind::NotFound => {
            report.root = Root::Missing;
            return report;
        }
        Err(_) => {
            report.root = Root::Unreadable;
            return report;
        }
        Ok(_) => {}
    }
    // The folder itself may be reached through a symlink the person made (a
    // data folder moved to another disk): follow that one, and only that one.
    let resolved = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    let root = resolved.as_path();
    scan_directory(
        &mut report,
        root,
        |name| match name {
            "agent-import-ledger.jsonl" => "journal",
            "agent-egress.jsonl" => "egress_log",
            "agent-import-admission.lock" => "locks",
            _ => "other",
        },
        &[
            "imports",
            "bank-statements",
            "lab",
            "native-dispatch-leases",
        ],
    );
    scan_directory(&mut report, &root.join("imports"), import_class, &[]);
    scan_directory(
        &mut report,
        &root.join("bank-statements"),
        |_| "bank_statements",
        &[],
    );
    scan_directory(&mut report, &root.join("lab"), |_| "lab", &[]);
    // The lease directory sits inside the root by default, so it is one of the
    // root's entries too: count its files as locks, not as "other".
    let leases = coordination.map(|dir| dir.join("native-dispatch-leases"));
    let leases = leases.or_else(|| Some(root.join("native-dispatch-leases")));
    if let Some(leases) = leases {
        scan_directory(&mut report, &leases, |_| "locks", &[]);
    }
    report.admission_lock = probe_admission_lock(root);
    report.journal = read_journal(root, &report.admission_lock);
    report
}

fn probe_admission_lock(root: &Path) -> Lock {
    let path = root.join("agent-import-admission.lock");
    // Read-only and never created: a probe that made the file would be a write.
    let file = match super::super::local_file::open_local_file(&path, false) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => return Lock::NotPresent,
        Err(_) => return Lock::Unreadable,
    };
    match file.try_lock_shared() {
        Ok(()) => Lock::Free,
        Err(std::fs::TryLockError::WouldBlock) => Lock::Busy,
        Err(_) => Lock::Unreadable,
    }
}

/// Read the journal only under the shared admission lock, as every other
/// reader does, so an append in progress is never seen half written.
fn read_journal(root: &Path, lock: &Lock) -> Journal {
    let path = root.join("agent-import-ledger.jsonl");
    let file = match super::super::local_file::open_local_file(&path, false) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => return Journal::Absent,
        Err(_) => return Journal::NotRead("journal_unreadable"),
    };
    let held = match lock {
        Lock::Free => {
            let lock_path = root.join("agent-import-admission.lock");
            match super::super::local_file::open_local_file(&lock_path, false) {
                Ok(held) if held.try_lock_shared().is_ok() => Some(held),
                _ => return Journal::NotRead("admission_lock_busy"),
            }
        }
        Lock::Busy => return Journal::NotRead("admission_lock_busy"),
        Lock::NotPresent => None,
        Lock::Unreadable => return Journal::NotRead("admission_lock_unreadable"),
    };
    let settlement = super::ledger::settlement(std::io::BufReader::new(file));
    drop(held);
    match settlement {
        Ok(settlement) => Journal::Read(settlement),
        Err(_) => Journal::NotRead("journal_invalid"),
    }
}

fn size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

/// Whole days since `time`, saturating at zero.
fn age_days(time: SystemTime, now: SystemTime) -> u64 {
    now.duration_since(time)
        .map_or(0, |elapsed| elapsed.as_secs() / 86_400)
}

/// The report as JSON for the `local_data_report` tool. It names no path: the
/// result goes into the AI conversation.
pub(super) fn to_json(report: &Report, now: SystemTime) -> serde_json::Value {
    use serde_json::json;
    let root = match report.root {
        Root::Missing => "missing",
        Root::Unreadable => "unreadable",
        Root::Present => "present",
    };
    let classes = report
        .classes
        .iter()
        .map(|(name, totals)| {
            (
                (*name).to_string(),
                json!({
                    "files": totals.files,
                    "bytes": totals.bytes,
                    "oldest_days": report.oldest.get(name).map(|time| age_days(*time, now)),
                }),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    let lock = match report.admission_lock {
        Lock::NotPresent => "not_present",
        Lock::Free => "free",
        Lock::Busy => "busy",
        Lock::Unreadable => "unreadable",
    };
    let journal = match &report.journal {
        Journal::Absent => json!({"state": "absent"}),
        Journal::NotRead(why) => json!({"state": "not_read", "reason": why}),
        Journal::Read(settlement) => json!({
            "state": "read",
            "batches": settlement.batches,
            "sent_or_found_posted": settlement.sent_or_found,
            "not_settled": settlement.unsettled,
        }),
    };
    json!({
        "folder": root,
        "classes": classes,
        "symlinks_or_non_files_not_followed": report.links,
        "files_with_more_than_one_hard_link": report.multi_link,
        "entries_that_could_not_be_read": report.unreadable,
        "interrupted_write_directories": report.publication_dirs,
        "admission_lock": lock,
        "journal": journal,
        "desktop_app_files_covered": false,
    })
}

/// The report as text for the person's terminal. No path appears unless
/// `paths` names them: the output is meant to be safe to paste.
pub(super) fn render(report: &Report, paths: Option<(&Path, Option<&Path>)>) -> String {
    let mut out = String::from(
        "Bridge local data report. Nothing was changed or deleted.\n\
         Only the agent data folder is covered; the desktop app's own files are not.\n",
    );
    match &report.root {
        Root::Missing => {
            out.push_str("The agent data folder does not exist: nothing is kept there.\n");
        }
        Root::Unreadable => {
            out.push_str(
                "The agent data folder could not be read. This is NOT the same as empty: \
                 nothing can be said about it.\n",
            );
        }
        Root::Present => {
            match paths {
                Some((root, _)) => out.push_str(&format!("folder: {}\n", root.display())),
                None => out.push_str("folder: (path not shown; add --show-paths)\n"),
            }
            out.push_str("\nclass               files       size   oldest\n");
            let now = SystemTime::now();
            for class in CLASSES {
                let class_totals = &report.classes[class];
                let oldest = report.oldest.get(class).map_or_else(
                    || "-".to_string(),
                    |time| format!("{} d", age_days(*time, now)),
                );
                out.push_str(&format!(
                    "{class:<18} {:>6} {:>10} {oldest:>8}\n",
                    class_totals.files,
                    size(class_totals.bytes)
                ));
            }
            out.push_str(&format!(
                "\nsymlinks or other non-files (not followed): {}\n\
                 files with more than one hard link: {}\n\
                 entries that could not be read: {}\n",
                report.links, report.multi_link, report.unreadable
            ));
            out.push_str(&format!(
                "interrupted-write directories (block builds): {}\n",
                report.publication_dirs
            ));
            out.push_str(&format!(
                "admission lock: {}\n",
                match report.admission_lock {
                    Lock::NotPresent => "not present",
                    Lock::Free => "free",
                    Lock::Busy => "busy (a Bridge process is using the journal)",
                    Lock::Unreadable => "could not be probed",
                }
            ));
            match &report.journal {
                Journal::Absent => out.push_str("journal: none\n"),
                Journal::NotRead(why) => out.push_str(&format!("journal: not read ({why})\n")),
                Journal::Read(settlement) => out.push_str(&format!(
                    "journal: {} batches; {} sent or found posted (the double-post checks \
                     look these up); {} not settled\n",
                    settlement.batches, settlement.sent_or_found, settlement.unsettled
                )),
            }
            out.push_str(
                "\nDeleting the journal and the import files would make Bridge forget which \
                 batches it already sent to Tally. A later deletion tool refuses while any \
                 batch is not settled.\n",
            );
        }
    }
    if let Some((_, Some(coordination))) = paths {
        out.push_str(&format!("lock folder: {}\n", coordination.display()));
    }
    out
}

/// The `local_data_report` tool's payload for `root`: no path, nothing changed.
/// The coordination folder is the default one.
pub(in crate::agent) fn tool_payload(root: &Path) -> serde_json::Value {
    let coordination = crate::local_files::paths::default_dispatch_coordination_dir();
    to_json(&build(root, coordination.as_deref()), SystemTime::now())
}

/// Resolve the folders the way the MCP server does, without creating them.
fn folders() -> (PathBuf, Option<PathBuf>) {
    let root = std::env::var_os("BRIDGE_AGENT_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(crate::local_files::paths::default_data_dir);
    (
        root,
        crate::local_files::paths::default_dispatch_coordination_dir(),
    )
}

/// `bridge_mcp --local-data-report [--show-paths]`: the exit status, 0 when a
/// report was produced (even of a missing folder) and 2 when the folder cannot
/// be read.
pub(in crate::agent) fn run(show_paths: bool) -> i32 {
    let (root, coordination) = folders();
    let report = build(&root, coordination.as_deref());
    let paths = show_paths.then_some((root.as_path(), coordination.as_deref()));
    print!("{}", render(&report, paths));
    if report.root == Root::Unreadable {
        2
    } else {
        0
    }
}

#[cfg(test)]
#[path = "agent_import_local_data_tests.rs"]
mod tests;
