//! A read-only report of the local data Bridge keeps (`bridge_mcp
//! --local-data-report` and the `local_data_report` tool). It lists what is in
//! the agent data folder by class, how big and how old it is, and how settled
//! the import journal is. It creates, locks and deletes nothing: the journal is
//! read without the admission lock, so a report can never make a post's
//! response record fail to append. It reads content only to count the journal's
//! batches, and names no path unless the person asks on the command line.
use std::collections::BTreeMap;
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Files and bytes of one class.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Class {
    pub(super) files: u64,
    pub(super) bytes: u64,
}

/// What the report could establish about the folder itself. An unreadable
/// folder is not a clean one.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Root {
    Missing,
    Unreadable,
    Present,
}

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
    /// Symlinks: counted, never followed.
    pub(super) links: u64,
    /// Sockets, pipes and devices: not files, so not counted in any class.
    pub(super) special_files: u64,
    /// Folders the report does not know and did not enter.
    pub(super) other_directories: u64,
    /// Publication folders in `imports/`: what an interrupted write leaves. Bridge
    /// refuses to build or read until it has recovered them.
    pub(super) interrupted_writes: u64,
    /// Entries whose metadata could not be read.
    pub(super) unreadable: u64,
    /// Folders that exist but could not be listed: their classes read 0 because
    /// nothing was seen, not because nothing is there.
    pub(super) folders_not_read: Vec<&'static str>,
    /// A folder held more entries than the report will list.
    pub(super) entry_cap_reached: bool,
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
    fn account(&mut self, class: &'static str, path: &Path) {
        match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.is_file() => {
                let totals = self.classes.entry(class).or_default();
                totals.files += 1;
                totals.bytes += metadata.len();
                if let Ok(modified) = metadata.modified() {
                    let oldest = self.oldest.entry(class).or_insert(modified);
                    *oldest = (*oldest).min(modified);
                }
            }
            Ok(metadata) if metadata.file_type().is_symlink() => self.links += 1,
            Ok(_) => self.special_files += 1,
            Err(_) => self.unreadable += 1,
        }
    }
}

/// The most entries of one folder the report lists: a folder past this is
/// reported as capped, so a huge one cannot make the report run on.
const ENTRY_CAP: usize = 100_000;

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

type ClassOf = fn(&str) -> &'static str;

fn is_link(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_symlink())
}

/// List `dir` one level deep, giving each file `class(name)`. A subfolder is
/// counted as unknown and not entered unless it is one of `known_directories`
/// (which the caller scans itself). A folder that cannot be listed is named
/// `label` in `folders_not_read`.
fn scan_directory(
    report: &mut Report,
    label: &'static str,
    dir: &Path,
    class: impl Fn(&str) -> &'static str,
    known_directories: &[&str],
    cap: usize,
) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == ErrorKind::NotFound => return,
        Err(_) => {
            report.folders_not_read.push(label);
            return;
        }
    };
    for (seen, entry) in entries.enumerate() {
        if seen >= cap {
            report.entry_cap_reached = true;
            break;
        }
        let Ok(entry) = entry else {
            report.unreadable += 1;
            continue;
        };
        let name = entry.file_name();
        let name = name.to_string_lossy();
        match fs::symlink_metadata(entry.path()) {
            Ok(metadata) if metadata.is_dir() => {
                if name == ".proof-publication" || name == ".build-publication" {
                    report.interrupted_writes += 1;
                } else if !known_directories.contains(&&*name) {
                    report.other_directories += 1;
                }
            }
            Ok(_) => report.account(class(&name), &entry.path()),
            Err(_) => report.unreadable += 1,
        }
    }
}

/// Take stock of `root` (the agent data folder) and, when given, the endpoint
/// coordination folder whose `native-dispatch-leases` holds the lease locks.
/// Reads only.
pub(super) fn build(root: &Path, coordination: Option<&Path>) -> Report {
    let mut report = Report {
        root: Root::Present,
        classes: CLASSES
            .iter()
            .map(|class| (*class, Class::default()))
            .collect(),
        oldest: BTreeMap::new(),
        links: 0,
        special_files: 0,
        other_directories: 0,
        interrupted_writes: 0,
        unreadable: 0,
        folders_not_read: Vec::new(),
        entry_cap_reached: false,
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
    // The folder itself may be a symlink the person made (a data folder moved
    // to another disk): `read_dir` and the joins below follow that one, and only
    // that one, because every entry inside is looked at without following.
    scan_directory(
        &mut report,
        "data_folder",
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
        ENTRY_CAP,
    );
    // A subfolder that is a symlink was counted as a link by the scan above.
    let subfolders: [(&'static str, ClassOf); 3] = [
        ("imports", import_class),
        ("bank-statements", |_| "bank_statements"),
        ("lab", |_| "lab"),
    ];
    for (name, class) in subfolders {
        let dir = root.join(name);
        if !is_link(&dir) {
            scan_directory(&mut report, name, &dir, class, &[], ENTRY_CAP);
        }
    }
    // The lease folder is the shared per-user one, which is the root's own
    // `native-dispatch-leases` by default and elsewhere with a custom folder:
    // scan each distinct one once.
    let mut leases: Vec<PathBuf> = Vec::new();
    for candidate in coordination
        .map(|dir| dir.join("native-dispatch-leases"))
        .into_iter()
        .chain([root.join("native-dispatch-leases")])
    {
        let canonical = fs::canonicalize(&candidate).unwrap_or_else(|_| candidate.clone());
        if !is_link(&candidate) && !leases.contains(&canonical) {
            leases.push(canonical);
        }
    }
    for dir in leases {
        scan_directory(
            &mut report,
            "lock_folder",
            &dir,
            |_| "locks",
            &[],
            ENTRY_CAP,
        );
    }
    report.journal = read_journal(root, || std::thread::sleep(Duration::from_millis(100)));
    report
}

/// The journal could not be opened (no permission, a link, another owner).
const JOURNAL_UNREADABLE: &str = "journal_unreadable";
/// The journal was opened but reading it failed, twice.
const JOURNAL_READ_FAILED: &str = "journal_read_failed";
/// The journal's records are refused as invalid, twice, 100 ms apart: a write
/// in progress would have finished by then.
const JOURNAL_INVALID: &str = "journal_invalid";

fn read_journal_once(path: &Path) -> Journal {
    let file = match super::super::local_file::open_local_file(path, false) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => return Journal::Absent,
        Err(_) => return Journal::NotRead(JOURNAL_UNREADABLE),
    };
    match super::ledger::settlement(std::io::BufReader::new(file)) {
        Ok(settlement) => Journal::Read(settlement),
        Err(reason) if reason == "import_ledger_invalid" => Journal::NotRead(JOURNAL_INVALID),
        Err(_) => Journal::NotRead(JOURNAL_READ_FAILED),
    }
}

/// Read the journal without the admission lock: holding it would make a post's
/// response record fail to append (a non-blocking exclusive lock) if the report
/// ran at that instant. An append in progress can leave a half-written last
/// line, so a failed read is tried once more (after `between`, 100 ms in the
/// report) before it is reported. A record rolled back after a failed append may
/// be counted once: this is a report.
fn read_journal(root: &Path, between: impl FnOnce()) -> Journal {
    let path = root.join("agent-import-ledger.jsonl");
    let first = read_journal_once(&path);
    if matches!(
        first,
        Journal::NotRead(JOURNAL_INVALID | JOURNAL_READ_FAILED)
    ) {
        between();
        return read_journal_once(&path);
    }
    first
}

/// Whole days since `time`, saturating at zero.
fn age_days(time: SystemTime, now: SystemTime) -> u64 {
    now.duration_since(time)
        .map_or(0, |elapsed| elapsed.as_secs() / 86_400)
}

/// The report as JSON. It names no path unless `paths` (the CLI's
/// `--show-paths`) gives them: the tool's result enters the AI conversation.
pub(super) fn to_json(
    report: &Report,
    now: SystemTime,
    paths: Option<(&Path, Option<&Path>)>,
) -> serde_json::Value {
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
    let journal = match &report.journal {
        Journal::Absent => json!({"state": "absent"}),
        Journal::NotRead(why) => json!({"state": "not_read", "reason": why}),
        Journal::Read(settlement) => json!({
            "state": "read",
            "batches": settlement.batches,
            "sent_or_found_posted": settlement.sent_or_found,
            "not_settled": settlement.unsettled,
            "not_settled_no_response": settlement.unsettled_no_response,
            "not_settled_not_verified": settlement.unsettled - settlement.unsettled_no_response,
            "no_dispatch_never_verified": settlement.no_dispatch_never_verified,
        }),
    };
    let mut value = json!({
        "folder": root,
        "classes": classes,
        "symlinks_not_followed": report.links,
        "special_files_not_counted": report.special_files,
        "folders_that_could_not_be_listed": report.folders_not_read,
        "entry_cap_reached": report.entry_cap_reached,
        "other_directories_not_entered": report.other_directories,
        "interrupted_write_folders": report.interrupted_writes,
        "entries_that_could_not_be_read": report.unreadable,
        "journal": journal,
        "app_files_outside_this_folder_covered": false,
    });
    if let Some((folder, locks)) = paths {
        value["folder_path"] = json!(folder.display().to_string());
        if let Some(locks) = locks {
            value["lock_folder_path"] = json!(locks.display().to_string());
        }
    }
    value
}

/// The `local_data_report` tool's payload for `root`: no path, and no file of the
/// folder is changed (the call itself is logged like any tool call).
/// The coordination folder is the default one.
pub(in crate::agent) fn tool_payload(root: &Path) -> serde_json::Value {
    let coordination = crate::local_files::paths::default_dispatch_coordination_dir();
    to_json(
        &build(root, coordination.as_deref()),
        SystemTime::now(),
        None,
    )
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

/// `bridge_mcp --local-data-report [--show-paths]`: prints the report as JSON.
/// The exit status is 0 when the report is complete (a missing folder is a
/// complete report), 2 when the folder cannot be read, 3 when the report is
/// incomplete (the journal was not read, or an entry or folder could not be
/// read, or a folder passed the listing cap) and 1 when it could not be printed.
pub(in crate::agent) fn run(show_paths: bool) -> i32 {
    let (root, coordination) = folders();
    let report = build(&root, coordination.as_deref());
    let paths = show_paths.then_some((root.as_path(), coordination.as_deref()));
    let value = to_json(&report, SystemTime::now(), paths);
    let printed = writeln!(
        std::io::stdout(),
        "{}",
        serde_json::to_string_pretty(&value).unwrap_or_default()
    );
    let _ = writeln!(std::io::stderr(), "Nothing was changed or deleted.");
    exit_status(&report, printed.is_ok())
}

fn exit_status(report: &Report, printed: bool) -> i32 {
    if !printed {
        1
    } else if report.root == Root::Unreadable {
        2
    } else if matches!(report.journal, Journal::NotRead(_))
        || report.unreadable > 0
        || !report.folders_not_read.is_empty()
        || report.entry_cap_reached
    {
        3
    } else {
        0
    }
}

#[cfg(test)]
#[path = "agent_import_local_data_tests.rs"]
mod tests;
