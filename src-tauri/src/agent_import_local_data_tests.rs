//! The local-data report reads and never writes (#local-data slice 1).
#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: the report command is run as a process of its own"
)]
use super::*;
use crate::agent::agent_import::tests::{sentinel_batch_id, sentinel_uuid};

/// The company of the master journal tests: the merge gate's one registered synthetic GUID.
const COMPANY_GUID: &str = "00000000-0000-4000-8000-000000000002";

fn write(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

/// A minimal journal: one built batch, optionally sent.
pub(in crate::agent::agent_import) fn journal(sent: bool) -> Vec<u8> {
    let line: super::super::ImportLedgerLine = serde_json::from_value(serde_json::json!({
        "batch_id":"b1","company_guid":"synthetic-guid","company":null,
        "txn_ids":["txn"],"date_from":"20260901","date_to":"20260901",
        "sha256":"a".repeat(64),"built_at":"2026-09-07T00:00:00Z","status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":1,"master_value":1},
        "vouchers":[{"bridge_txn_id":"txn","date":"20260901","voucher_type":"Journal",
            "narration":"n","reference":null,"voucher_number":null,"entries":[
                {"ledger":"Cash","amount":"1.00","side":"Dr"},
                {"ledger":"Synthetic Ledger","amount":"1.00","side":"Cr"}]}]
    }))
    .unwrap();
    let mut bytes = serde_json::to_vec(&line).unwrap();
    bytes.push(b'\n');
    if sent {
        bytes.extend(
            serde_json::to_vec(&super::super::ledger::StatusRecord::dispatch(&line)).unwrap(),
        );
        bytes.push(b'\n');
    }
    bytes
}

fn class<'a>(report: &'a Report, name: &str) -> &'a Class {
    &report.classes[name]
}

#[test]
fn files_are_classed_by_name_and_sizes_add_up() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let journal_bytes = journal(false);
    write(&root.join("agent-import-ledger.jsonl"), &journal_bytes);
    write(&root.join("agent-import-statement-skips.jsonl"), b"123");
    write(&root.join("agent-master-ledger.jsonl"), b"abc");
    write(
        &root.join(format!(
            "master-pre-image-{}.json",
            sentinel_batch_id(0x1545)
        )),
        b"{}",
    );
    // A look-alike with no batch id in its name is not one of ours.
    write(&root.join("master-pre-image-x.json"), b"xy");
    write(&root.join("agent-egress.jsonl"), b"abcd");
    write(&root.join("agent-import-admission.lock"), b"");
    write(&root.join("notes.txt"), b"12345");
    write(&root.join("imports/b.xml"), b"0123456789");
    write(&root.join("imports/b.proof.json"), b"12345");
    write(&root.join("imports/b.proof.md"), b"123456");
    let saved = format!("b.proof.20261009T101500123Z.{}", "a".repeat(64));
    write(&root.join(format!("imports/{saved}.json")), b"1");
    write(&root.join(format!("imports/{saved}.md")), b"12");
    write(&root.join("imports/b.masters_verdict.json"), b"123");
    write(&root.join("imports/b.baseline.1.json"), b"1234");
    write(&root.join("imports/b.baseline.x.json"), b"1");
    write(&root.join("imports/b.approval_lapse.json"), b"1234567");
    write(&root.join("imports/b.masters_doubt.json"), b"12345678");
    write(&root.join("imports/b.baseline.json"), b"123456789");
    write(&root.join("imports/b.batch_step_ack.json"), b"1");
    write(&root.join("imports/stray.txt"), b"12");
    write(&root.join("bank-statements/s.json"), b"12345678901");
    write(&root.join("lab/x.request.xml"), b"123");
    write(&root.join("native-dispatch-leases/k.lock"), b"");
    write(&root.join("misc/whatever.bin"), b"123456789012345");
    write(&root.join("imports/.proof-publication/update.json"), b"{}");
    let report = build(root, &[]);
    assert_eq!(report.root, Root::Present);
    let expect = |name: &str, files: u64, bytes: u64| {
        assert_eq!(*class(&report, name), Class { files, bytes }, "{name}");
    };
    expect("journal", 4, journal_bytes.len() as u64 + 8);
    expect("egress_log", 1, 4);
    expect("locks", 2, 0);
    expect("other", 4, 10);
    expect("import_files", 1, 10);
    expect("proofs", 4, 14);
    expect("approval_notes", 1, 7);
    expect("review_records", 5, 25);
    expect("bank_statements", 1, 11);
    expect("lab", 1, 3);
    assert_eq!(
        report.other_directories, 1,
        "an unknown folder is counted, not entered"
    );
    assert_eq!(report.interrupted_writes, 1, "the publication folder");
    assert_eq!(report.links + report.special_files + report.unreadable, 0);
    assert!(report.folders_not_read.is_empty() && !report.entry_cap_reached);
}

#[test]
fn a_lock_folder_outside_the_data_folder_is_counted_too_and_not_twice() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("data");
    let coordination = directory.path().join("shared");
    write(&root.join("native-dispatch-leases/a.lock"), b"");
    write(&coordination.join("native-dispatch-leases/b.lock"), b"");
    let separate = build(&root, &[coordination.as_path()]);
    assert_eq!(class(&separate, "locks").files, 2, "both lock folders");
    // The default: the shared folder is the data folder itself.
    let same = build(&root, &[root.as_path()]);
    assert_eq!(class(&same, "locks").files, 1, "one folder, counted once");
    // Windows takes each lock under two coordination folders: both are counted
    let profile = directory.path().join("profile");
    write(&profile.join("native-dispatch-leases/c.lock"), b"");
    let both = build(&root, &[coordination.as_path(), profile.as_path()]);
    assert_eq!(class(&both, "locks").files, 3, "every lock folder");
}

/// A lab folder that is a link is not entered, so a kept read below the link's
/// target is not counted either.
#[cfg(unix)]
#[test]
fn a_kept_audit_read_behind_a_linked_lab_folder_is_not_counted() {
    let directory = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    write(
        &outside
            .path()
            .join("audit-reads")
            .join("a".repeat(64))
            .join("manifest.json"),
        b"12",
    );
    std::os::unix::fs::symlink(outside.path(), directory.path().join("lab")).unwrap();
    let report = build(directory.path(), &[]);
    assert_eq!(*class(&report, "lab"), Class::default());
    assert_eq!(report.folders_not_read, ["lab"]);
}

#[cfg(unix)]
#[test]
fn a_symlink_is_counted_and_never_followed() {
    let directory = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&outside.path().join("big.bin"), &[0_u8; 4096]);
    fs::create_dir_all(root.join("imports")).unwrap();
    std::os::unix::fs::symlink(
        outside.path().join("big.bin"),
        root.join("imports/link.xml"),
    )
    .unwrap();
    std::os::unix::fs::symlink(outside.path(), root.join("bank-statements")).unwrap();
    let report = build(root, &[]);
    assert_eq!(
        report.links, 2,
        "the symlinked file and the symlinked directory"
    );
    assert_eq!(
        report.folders_not_read,
        ["bank-statements"],
        "a linked folder is not entered, so its class is unseen, and says so"
    );
    assert_eq!(report.incomplete_reason(), Some("folder_not_listed"));
    assert_eq!(*class(&report, "import_files"), Class::default());
    // A symlinked directory is not entered: its contents are not counted.
    assert_eq!(*class(&report, "bank_statements"), Class::default());
}

/// A data folder the person moved to another disk and linked back is followed
/// (only that one link); the report must not read as empty.
#[cfg(unix)]
#[test]
fn a_symlinked_data_folder_itself_is_followed() {
    let directory = tempfile::tempdir().unwrap();
    let real = directory.path().join("real");
    write(&real.join("agent-egress.jsonl"), b"abcd");
    let link = directory.path().join("Bridge");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    let report = build(&link, &[]);
    assert_eq!(report.root, Root::Present);
    assert_eq!(*class(&report, "egress_log"), Class { files: 1, bytes: 4 });
}

#[test]
fn a_missing_folder_is_not_an_empty_one_and_not_an_unreadable_one() {
    let directory = tempfile::tempdir().unwrap();
    assert_eq!(
        build(&directory.path().join("nope"), &[]).root,
        Root::Missing
    );
    let empty = build(directory.path(), &[]);
    assert_eq!(empty.root, Root::Present);
    assert!(empty
        .classes
        .values()
        .all(|class| *class == Class::default()));
    assert_eq!(empty.journal, Journal::Absent);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let locked = directory.path().join("locked");
        fs::create_dir(&locked).unwrap();
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
        let unreadable = build(&locked, &[]);
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o700)).unwrap();
        // Running as root would read it; the distinction only exists for a user.
        if unreadable.root != Root::Present {
            assert_eq!(unreadable.root, Root::Unreadable);
        }
    }
}

#[test]
fn the_report_changes_nothing_and_takes_no_lock() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&root.join("agent-import-ledger.jsonl"), &journal(true));
    write(&root.join("agent-import-admission.lock"), b"");
    write(&root.join("imports/b.xml"), b"<x/>");
    fn walk(dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            out.push(path.clone());
            if path.is_dir() {
                out.extend(walk(&path));
            }
        }
        out
    }
    let listing = || {
        let mut names = Vec::new();
        for entry in walk(root) {
            let metadata = fs::symlink_metadata(&entry).unwrap();
            names.push((entry, metadata.len(), metadata.modified().unwrap()));
        }
        names.sort();
        names
    };
    let before = listing();
    // A process that holds the admission lock exclusively (a post in flight)
    // must not stop the report, and the report must not disturb it.
    let holder = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(root.join("agent-import-admission.lock"))
        .unwrap();
    holder.lock().unwrap();
    let report = build(root, &[]);
    assert!(matches!(report.journal, Journal::Read(_)), "{report:?}");
    drop(holder);
    let second = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(root.join("agent-import-admission.lock"))
        .unwrap();
    assert!(
        second.try_lock().is_ok(),
        "the report keeps no lock: a fresh handle takes it"
    );
    drop(second);
    assert_eq!(listing(), before);
    assert!(!root.join("agent-import-ledger.jsonl.tmp").exists());
}

#[test]
fn the_journal_is_absent_unreadable_or_counted() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-import-ledger.jsonl");
    assert_eq!(build(directory.path(), &[]).journal, Journal::Absent);
    write(&path, b"not json\n");
    assert_eq!(
        build(directory.path(), &[]).journal,
        Journal::NotRead(JOURNAL_INVALID)
    );
    write(&path, &journal(true));
    assert_eq!(
        build(directory.path(), &[]).journal,
        Journal::Read(super::super::ledger::Settlement {
            batches: 1,
            sent_or_found: 1,
            unsettled: 1,
            unsettled_no_response: 1,
            unsettled_binding_refused: 0,
            unsettled_reviewed: 0,
            unsettled_restored: 0,
            settled_by_person: 0,
            released_by_person: 0,
            no_dispatch_never_verified: 0
        })
    );
    write(&path, &journal(false));
    assert_eq!(
        build(directory.path(), &[]).journal,
        Journal::Read(super::super::ledger::Settlement {
            batches: 1,
            sent_or_found: 0,
            unsettled: 0,
            unsettled_no_response: 0,
            unsettled_binding_refused: 0,
            unsettled_reviewed: 0,
            unsettled_restored: 0,
            settled_by_person: 0,
            released_by_person: 0,
            no_dispatch_never_verified: 1
        })
    );
}

/// A batch whose binding was refused is counted in its own class, and in no
/// other class of unsettled batch (#1039).
#[test]
fn a_refused_binding_is_its_own_class_of_unsettled_batch() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.journal = Journal::Read(super::super::ledger::Settlement {
        batches: 4,
        sent_or_found: 4,
        unsettled: 3,
        unsettled_no_response: 1,
        unsettled_binding_refused: 1,
        unsettled_reviewed: 0,
        unsettled_restored: 0,
        settled_by_person: 0,
        released_by_person: 0,
        no_dispatch_never_verified: 0,
    });
    let journal = &to_json(&report, SystemTime::now(), None)["journal"];
    assert_eq!(journal["not_settled"], 3);
    assert_eq!(journal["not_settled_no_response"], 1);
    assert_eq!(journal["not_settled_binding_refused"], 1);
    assert_eq!(journal["not_settled_not_verified"], 1);
}

/// Whether serialized JSON `text` holds `path`, written as it is and as JSON
/// writes it (a Windows path's backslashes doubled), so the check means the same
/// on every platform.
fn names_the_path(text: &str, path: &Path) -> bool {
    let plain = path.to_string_lossy();
    let escaped = serde_json::to_string(&*plain).unwrap();
    text.contains(&*plain) || text.contains(&escaped[1..escaped.len() - 1])
}

/// A Windows-style path is found in JSON that writes its backslashes doubled, and
/// the raw-text check this helper replaced is not: so the no-path assertions below
/// mean something on Windows, and this proves it on any platform.
#[test]
fn a_path_with_backslashes_is_found_in_json_that_doubles_them() {
    let path = Path::new("C:\\Users\\runner\\AppData\\Local\\Temp\\.tmpAbCd");
    let json = serde_json::json!({"folder_path": path.display().to_string()}).to_string();
    assert!(
        json.contains("C:\\\\Users"),
        "the JSON doubles every backslash: {json}"
    );
    assert!(
        !json.contains(&*path.to_string_lossy()),
        "a raw substring check cannot find it: {json}"
    );
    assert!(names_the_path(&json, path));
    assert!(!names_the_path(&json, Path::new("C:\\Users\\someone-else")));
    assert!(!names_the_path("{}", path));
}

#[test]
fn no_path_is_in_the_json_unless_asked() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&root.join("imports/b.xml"), b"<x/>");
    let report = build(root, &[]);
    let plain_value = to_json(&report, SystemTime::now(), None);
    assert!(plain_value.get("package_copies").is_none(), "{plain_value}");
    let plain = plain_value.to_string();
    assert!(!names_the_path(&plain, root), "{plain}");
    assert!(!plain.contains("path"), "{plain}");
    // The value, not the serialized text: on Windows the text doubles every
    // backslash of the path, so a substring check on it proves nothing.
    let shown = to_json(&report, SystemTime::now(), Some((root, &[root])));
    let path = root.display().to_string();
    // each folder as built (#1459): its own final path is not trusted
    assert_eq!(shown["folder_path"], path, "{shown}");
    assert_eq!(shown["lock_folder_path"], path, "{shown}");
    assert_eq!(
        shown["lock_folder_paths"],
        serde_json::json!([path]),
        "{shown}"
    );
    // no file directly in the folder (imports is a folder); no lease folder
    assert_eq!(
        shown["folder_file_locations"],
        serde_json::json!({"folders": [], "files_read": 0, "files": 0, "listing_capped": false}),
        "{shown}"
    );
    // package copies are looked for only on Windows, under %LOCALAPPDATA%
    #[cfg(not(windows))]
    assert!(shown.get("package_copies").is_none(), "{shown}");
    #[cfg(windows)]
    {
        let copies = &shown["package_copies"];
        assert!(
            copies
                .as_array()
                .is_some_and(|all| all.iter().all(serde_json::Value::is_string))
                || copies == "not_listed"
                || copies == "not_under_local_app_data",
            "{shown}"
        );
    }
    assert_eq!(
        shown["lock_folder_file_locations"],
        serde_json::json!(["not_listed"]),
        "{shown}"
    );
    // two lock folders: the first is still lock_folder_path, both are listed in order
    let second = root.join("profile");
    let two = to_json(
        &report,
        SystemTime::now(),
        Some((root, &[root, second.as_path()])),
    );
    assert_eq!(two["lock_folder_path"], path, "{two}");
    assert_eq!(
        two["lock_folder_paths"],
        serde_json::json!([path, second.display().to_string()]),
        "{two}"
    );
    assert!(names_the_path(&shown.to_string(), root), "{shown}");
    assert!(shown.get("journal_path").is_none(), "{shown}");
}

/// #1459, #1584: a path is shown where the operating system reports each file
/// to be, never where the folder resolves. The resolver stands in for Windows
/// as the lab observed it inside a Store install's package once the real folder
/// also existed: the folder names the real one, while a file the package's copy
/// holds names that copy, the journal included, even where the real folder
/// holds a file of the same name.
#[test]
fn a_folder_is_shown_by_where_its_files_are_not_where_it_resolves() {
    let directory = tempfile::tempdir().unwrap();
    let local = directory.path().join("local");
    let root = local.join("Bridge").join("agent");
    let locks = directory.path().join("locks");
    write(&root.join("agent-import-ledger.jsonl"), &journal(true));
    write(&root.join("terms-acceptance.jsonl"), b"{}\n");
    write(&locks.join("native-dispatch-leases").join("a.lock"), b"");
    // one package holds a copy of the data folder, another does not
    let copy = local
        .join("Packages")
        .join("Store.App_1")
        .join("LocalCache")
        .join("Local")
        .join("Bridge")
        .join("agent");
    fs::create_dir_all(&copy).unwrap();
    fs::create_dir_all(
        local
            .join("Packages")
            .join("Other.App_2")
            .join("LocalCache"),
    )
    .unwrap();
    let package = PathBuf::from("/package/agent");
    let real = PathBuf::from("/real/agent");
    let leases = PathBuf::from("/package/leases");
    let resolve = |path: &Path| -> VisiblePath {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        match name.as_str() {
            "a.lock" => VisiblePath::Resolved(leases.join(&name)),
            _ if path.is_file() => VisiblePath::Resolved(package.join(&name)),
            _ => VisiblePath::Resolved(real.clone()),
        }
    };
    let report = build(&root, &[]);
    let shown = to_json_resolving(
        &report,
        SystemTime::now(),
        Some((&root, &[locks.as_path()])),
        &resolve,
        Some(&local),
    );
    assert_eq!(shown["folder_path"], root.display().to_string(), "{shown}");
    // the journal by its own location, not the resolved folder plus its name
    assert_eq!(
        shown["journal_path"],
        package
            .join("agent-import-ledger.jsonl")
            .display()
            .to_string(),
        "{shown}"
    );
    assert_eq!(shown["journal_path_state"], "resolved", "{shown}");
    // two files in one copy: that copy once, and both files read
    assert_eq!(
        shown["folder_file_locations"],
        serde_json::json!({
            "folders": [{"path": package.display().to_string(), "state": "resolved"}],
            "files_read": 2,
            "files": 2,
            "listing_capped": false,
        }),
        "{shown}"
    );
    assert_eq!(
        shown["lock_folder_file_locations"],
        serde_json::json!([{
            "folders": [{"path": leases.display().to_string(), "state": "resolved"}],
            "files_read": 1,
            "files": 1,
            "listing_capped": false,
        }]),
        "{shown}"
    );
    assert_eq!(
        shown["lock_folder_path"],
        locks.display().to_string(),
        "{shown}"
    );
    // the copy a process on the other side of the package would not see
    assert_eq!(
        shown["package_copies"],
        serde_json::json!([copy.display().to_string()]),
        "{shown}"
    );
    // a data folder set outside %LOCALAPPDATA% has no package copy to look for
    let elsewhere = to_json_resolving(
        &report,
        SystemTime::now(),
        Some((&locks, &[locks.as_path()])),
        &resolve,
        Some(&local),
    );
    assert_eq!(
        elsewhere["package_copies"], "not_under_local_app_data",
        "{elsewhere}"
    );

    // a data folder that could not be read: no journal path, whatever is there
    let not_a_folder = root.join("agent-import-ledger.jsonl");
    let unreadable = build(&not_a_folder, &[]);
    assert!(matches!(unreadable.root, Root::Unreadable));
    let shown = to_json_resolving(
        &unreadable,
        SystemTime::now(),
        Some((&not_a_folder, &[locks.as_path()])),
        &resolve,
        None,
    );
    assert!(shown.get("journal_path").is_none(), "{shown}");
}

/// Files that resolve to two folders list both, in the order of the files'
/// names, each once (the lab did not observe this from either side: each side
/// saw its own copy only).
#[test]
fn files_in_two_folders_list_both_once() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    for name in ["a.jsonl", "b.jsonl", "c.jsonl"] {
        write(&root.join(name), b"");
    }
    let resolve = |path: &Path| -> VisiblePath {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let folder = if name == "b.jsonl" { "/two" } else { "/one" };
        VisiblePath::Resolved(PathBuf::from(folder).join(&name))
    };
    assert_eq!(
        file_locations(root, &resolve),
        serde_json::json!({
            "folders": [
                {"path": PathBuf::from("/one").display().to_string(), "state": "resolved"},
                {"path": PathBuf::from("/two").display().to_string(), "state": "resolved"},
            ],
            "files_read": 3,
            "files": 3,
            "listing_capped": false,
        })
    );
}

/// The listing stops at its cap and says so; the files past it are not counted.
#[test]
fn a_listing_past_its_cap_says_so() {
    let directory = tempfile::tempdir().unwrap();
    for n in 0..5 {
        write(&directory.path().join(format!("f{n}")), b"");
    }
    let resolve = |path: &Path| VisiblePath::Resolved(path.to_path_buf());
    let capped = file_locations_capped(directory.path(), &resolve, 3);
    assert_eq!(capped["files"], 3, "{capped}");
    assert_eq!(capped["listing_capped"], true, "{capped}");
    let whole = file_locations_capped(directory.path(), &resolve, 5);
    assert_eq!(whole["files"], 5, "{whole}");
    assert_eq!(whole["listing_capped"], false, "{whole}");
}

/// A data folder under `%LOCALAPPDATA%` is matched as Windows matches a path:
/// without regard to letter case; another folder is not.
#[test]
fn a_folder_is_matched_under_local_app_data_without_regard_to_case() {
    // an absolute base on each platform (Windows needs a drive)
    let root = if cfg!(windows) { "C:\\" } else { "/" };
    let base = PathBuf::from(root)
        .join("srv")
        .join("Profile")
        .join("AppData")
        .join("Local");
    let lower = PathBuf::from(base.display().to_string().to_lowercase());
    assert_eq!(
        relative_to(&base, &lower.join("Bridge").join("agent")),
        Some(PathBuf::from("Bridge").join("agent"))
    );
    let sibling = base.parent().unwrap().join("LocalLow").join("Bridge");
    assert_eq!(relative_to(&base, &sibling), None);
    assert_eq!(relative_to(&base, base.parent().unwrap()), None);
    assert_eq!(relative_to(&base, &base), Some(PathBuf::new()));
    // a remainder that climbs out of the base, or a base that is not absolute,
    // names no folder inside it
    assert_eq!(relative_to(&base, &base.join("Bridge").join("..")), None);
    assert_eq!(
        relative_to(&base, &base.join("..").join("..").join("Bridge")),
        None
    );
    assert_eq!(relative_to(Path::new(""), &base), None);
    assert_eq!(
        relative_to(Path::new("Local"), &Path::new("Local").join("Bridge")),
        None
    );
}

/// `package_copies` finds a copy of a data folder whose `%LOCALAPPDATA%` part
/// differs only in letter case, and none for `%LOCALAPPDATA%` itself.
#[test]
fn package_copies_match_without_regard_to_case() {
    let directory = tempfile::tempdir().unwrap();
    let base = directory.path().join("Local");
    let copy = base
        .join("Packages")
        .join("Store.App_1")
        .join("LocalCache")
        .join("Local")
        .join("Bridge")
        .join("agent");
    fs::create_dir_all(&copy).unwrap();
    let shouted = PathBuf::from(base.display().to_string().to_uppercase())
        .join("Bridge")
        .join("agent");
    assert_eq!(
        package_copies(&base, &shouted),
        serde_json::json!([copy.display().to_string()])
    );
    assert_eq!(package_copies(&base, &base), "not_under_local_app_data");
    // a folder spelled with `..` below the base is not a package copy's name
    assert_eq!(
        package_copies(&base, &base.join("Bridge").join("..")),
        "not_under_local_app_data"
    );
}

/// A data folder that is a link (a folder moved to another disk) is followed,
/// as the report counts it, so where its files are is still shown.
#[cfg(unix)]
#[test]
fn a_data_folder_that_is_a_link_is_still_located() {
    let directory = tempfile::tempdir().unwrap();
    let moved = directory.path().join("moved");
    write(&moved.join("terms-acceptance.jsonl"), b"{}\n");
    let root = directory.path().join("agent");
    std::os::unix::fs::symlink(&moved, &root).unwrap();
    let resolve = |path: &Path| VisiblePath::Resolved(path.to_path_buf());
    let shown = to_json_resolving(
        &build(&root, &[]),
        SystemTime::now(),
        Some((&root, &[])),
        &resolve,
        None,
    );
    assert_eq!(shown["folder_file_locations"]["files"], 1, "{shown}");
}

/// A lease folder that is itself a link is not entered, and a journal file
/// that is itself a link is not resolved; a link above them is followed (see
/// `a_data_folder_that_is_a_link_is_still_located`).
#[cfg(unix)]
#[test]
fn a_link_is_not_followed_for_a_path_shown() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("agent");
    let elsewhere = directory.path().join("elsewhere");
    write(&elsewhere.join("agent-import-ledger.jsonl"), &journal(true));
    write(&elsewhere.join("a.lock"), b"");
    fs::create_dir_all(&root).unwrap();
    std::os::unix::fs::symlink(
        elsewhere.join("agent-import-ledger.jsonl"),
        root.join("agent-import-ledger.jsonl"),
    )
    .unwrap();
    let locks = directory.path().join("locks");
    fs::create_dir_all(&locks).unwrap();
    std::os::unix::fs::symlink(&elsewhere, locks.join("native-dispatch-leases")).unwrap();
    let resolve = |path: &Path| {
        VisiblePath::Resolved(PathBuf::from("/followed").join(path.file_name().unwrap()))
    };
    let report = build(&root, &[]);
    assert!(matches!(report.root, Root::Present));
    let shown = to_json_resolving(
        &report,
        SystemTime::now(),
        Some((&root, &[locks.as_path()])),
        &resolve,
        None,
    );
    assert_eq!(shown["journal_path_state"], "link_not_followed", "{shown}");
    assert_eq!(
        shown["journal_path"],
        root.join("agent-import-ledger.jsonl").display().to_string(),
        "{shown}"
    );
    assert_eq!(
        shown["lock_folder_file_locations"],
        serde_json::json!(["link_not_followed"]),
        "{shown}"
    );
}

/// The locations come from the first `FILE_LOCATION_SAMPLE` files by name,
/// whatever order the folder lists them in, and the JSON says how many were read.
#[test]
fn the_sample_is_the_first_files_by_name() {
    let names: Vec<PathBuf> = (0..FILE_LOCATION_SAMPLE + 6)
        .rev()
        .map(|n| PathBuf::from(format!("f{n:03}")))
        .collect();
    let sample = file_sample(names);
    assert_eq!(sample.len(), FILE_LOCATION_SAMPLE);
    assert_eq!(sample[0], PathBuf::from("f000"));
    assert_eq!(
        sample[FILE_LOCATION_SAMPLE - 1],
        PathBuf::from(format!("f{:03}", FILE_LOCATION_SAMPLE - 1))
    );
    // through the folder: a file past the sample does not show its folder
    let directory = tempfile::tempdir().unwrap();
    for n in 0..=FILE_LOCATION_SAMPLE {
        write(&directory.path().join(format!("f{n:03}")), b"");
    }
    let last = format!("f{FILE_LOCATION_SAMPLE:03}");
    let resolve = |path: &Path| -> VisiblePath {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let folder = if name == last { "/late" } else { "/early" };
        VisiblePath::Resolved(PathBuf::from(folder).join(&name))
    };
    assert_eq!(
        file_locations(directory.path(), &resolve),
        serde_json::json!({
            "folders": [{"path": PathBuf::from("/early").display().to_string(), "state": "resolved"}],
            "files_read": FILE_LOCATION_SAMPLE,
            "files": FILE_LOCATION_SAMPLE + 1,
            "listing_capped": false,
        })
    );
}

fn server_over(directory: &Path) -> crate::agent::Server {
    crate::agent::Server::new(crate::agent::Settings {
        endpoint: bridge_tally_transport::TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    })
}

/// The tool answers with counts, sizes and ages and names no path, takes no
/// arguments, and reads only (#local-data slice 1).
#[tokio::test]
async fn the_tool_reports_without_a_path_and_takes_no_arguments() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&root.join("agent-egress.jsonl"), b"abcd");
    write(&root.join("bank-statements/s.json"), b"12345");
    write(&root.join("agent-import-ledger.jsonl"), &journal(true));
    let old = fs::File::options()
        .write(true)
        .open(root.join("bank-statements/s.json"))
        .unwrap();
    old.set_modified(SystemTime::now() - std::time::Duration::from_secs(40 * 86_400))
        .unwrap();
    let server = server_over(root);
    let response = server
        .call_tool("local_data_report", serde_json::json!({}))
        .await;
    assert_eq!(response["isError"], false, "{response}");
    let text = response.to_string();
    assert!(!names_the_path(&text, root), "no path: {text}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["folder"], "present");
    assert_eq!(result["classes"]["egress_log"]["files"], 1);
    assert_eq!(result["classes"]["egress_log"]["bytes"], 4);
    assert_eq!(result["classes"]["bank_statements"]["oldest_days"], 40);
    assert_eq!(result["journal"]["state"], "read");
    assert_eq!(result["journal"]["not_settled"], 1);
    assert_eq!(result["journal"]["not_settled_no_response"], 1);
    assert_eq!(result["journal"]["not_settled_binding_refused"], 0);
    assert_eq!(result["journal"]["not_settled_not_verified"], 0);
    assert_eq!(result["journal"]["no_dispatch_never_verified"], 0);
    assert_eq!(result["app_files_outside_this_folder_covered"], false);

    let refused = server
        .call_tool("local_data_report", serde_json::json!({"path": "/"}))
        .await;
    assert_eq!(refused["isError"], true, "{refused}");
}

#[test]
fn ages_count_whole_days_and_never_go_negative() {
    let now = SystemTime::now();
    assert_eq!(
        age_days(now - std::time::Duration::from_secs(3 * 86_400 + 5), now),
        3
    );
    assert_eq!(
        age_days(now + std::time::Duration::from_secs(86_400), now),
        0
    );
}

/// The report must never take the admission lock or a lease: a lock held for the
/// journal scan can make a post's response record fail to append (a
/// non-blocking exclusive lock). Checked on the source of the report and of the
/// two helpers it reads through (the journal scan and the file open), so a later
/// edit that adds a lock call to any of them fails here.
#[test]
fn the_report_and_what_it_reads_through_take_no_lock() {
    let sources = [
        ("report", include_str!("agent_import_local_data.rs")),
        ("journal scan", include_str!("agent_import_ledger.rs")),
        ("file open", include_str!("local_files/file.rs")),
    ];
    for (name, source) in sources {
        // The report names the admission lock file to count it; it never opens it.
        for forbidden in [
            ".try_lock",
            ".lock(",
            ".lock_shared",
            ".unlock",
            "lock_import_admission",
            "acquire(",
            "flock",
            "LockFileEx",
        ] {
            assert!(!source.contains(forbidden), "{name}: {forbidden}");
        }
    }
}

/// A journal that exists but cannot be opened is `journal_unreadable`, not
/// absent and not invalid: a symlink is refused by the no-follow open, on any
/// user, so this does not depend on file permissions.
#[cfg(unix)]
#[test]
fn a_journal_that_cannot_be_opened_is_unreadable_and_not_absent() {
    let directory = tempfile::tempdir().unwrap();
    let real = directory.path().join("real.jsonl");
    write(&real, &journal(true));
    std::os::unix::fs::symlink(&real, directory.path().join("agent-import-ledger.jsonl")).unwrap();
    assert_eq!(
        build(directory.path(), &[]).journal,
        Journal::NotRead(JOURNAL_UNREADABLE)
    );
}

/// A last line half written when the report read it is read again once; it
/// heals if the writer has finished, and is reported invalid if it has not.
#[test]
fn a_half_written_last_line_is_read_again_once() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-import-ledger.jsonl");
    let whole = journal(true);
    let mut cut = whole.clone();
    cut.extend_from_slice(b"{\"record_type\":\"dispat");
    write(&path, &cut);
    // The writer finishes between the two reads.
    let healed = read_journal(directory.path(), || fs::write(&path, &whole).unwrap());
    assert!(matches!(healed, Journal::Read(_)), "{healed:?}");
    // It does not: the second read fails the same way, and says so.
    write(&path, &cut);
    let stuck = read_journal(directory.path(), || {});
    assert_eq!(stuck, Journal::NotRead(JOURNAL_INVALID));
    // A journal that is whole is read once, with no second read.
    write(&path, &whole);
    let mut waited = false;
    assert!(matches!(
        read_journal(directory.path(), || waited = true),
        Journal::Read(_)
    ));
    assert!(!waited);
}

/// A line whose amount is not an import amount (#1872) is a line the report cannot read: the
/// journal is shown as invalid, as for any other record the build refuses, and it is read twice
/// before it is said.
#[test]
fn a_journal_line_with_a_signed_amount_is_shown_as_invalid() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-import-ledger.jsonl");
    let whole = String::from_utf8(journal(true)).unwrap();
    assert_eq!(whole.matches("\"amount\":\"1.00\"").count(), 2, "{whole}");
    write(
        &path,
        whole
            .replacen("\"amount\":\"1.00\"", "\"amount\":\"-1.00\"", 1)
            .as_bytes(),
    );
    let mut waited = false;
    assert_eq!(
        read_journal(directory.path(), || waited = true),
        Journal::NotRead(JOURNAL_INVALID)
    );
    assert!(waited);
}

/// The report names the line it refused, and only the report does (#1872). A bad amount sits
/// between two good lines; the field is the 1-based line, and a journal that reads has none.
/// A half-written last line is named too: it is the line after the last whole one.
#[test]
fn the_report_names_the_line_of_a_bad_amount_between_two_good_ones() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-import-ledger.jsonl");
    let first = String::from_utf8(journal(false)).unwrap();
    let with_id =
        |id: &str| first.replacen("\"batch_id\":\"b1\"", &format!("\"batch_id\":\"{id}\""), 1);
    let bad = with_id("b2").replacen("\"amount\":\"1.00\"", "\"amount\":\"-1.00\"", 1);
    write(&path, format!("{first}{bad}{}", with_id("b3")).as_bytes());
    let report = build(directory.path(), &[]);
    assert_eq!(report.journal, Journal::NotRead(JOURNAL_INVALID));
    assert_eq!(report.journal_line, Some(2));
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(shown["journal"]["state"], "not_read");
    assert_eq!(shown["journal"]["reason"], "journal_invalid");
    assert_eq!(shown["journal"]["line"], 2);
    // Without the bad line the journal reads and names no line.
    write(&path, format!("{first}{}", with_id("b3")).as_bytes());
    let report = build(directory.path(), &[]);
    assert!(matches!(report.journal, Journal::Read(_)), "{report:?}");
    assert_eq!(report.journal_line, None);
    assert!(to_json(&report, SystemTime::now(), None)["journal"]["line"].is_null());
    // A half-written last line is the line after the last whole one.
    let mut cut = format!("{first}{}", with_id("b3")).into_bytes();
    cut.extend_from_slice(b"{\"record_type\":\"dispat");
    write(&path, &cut);
    assert_eq!(
        read_journal_at(directory.path(), || {}),
        (Journal::NotRead(JOURNAL_INVALID), Some(3))
    );
    // Another reason a journal is not read names no line.
    let missing = tempfile::tempdir().unwrap();
    assert_eq!(
        read_journal_at(missing.path(), || {}),
        (Journal::Absent, None)
    );
}

/// A folder that exists but cannot be listed is named, not read as empty. A file
/// where the folder should be fails the listing on any user.
#[cfg(unix)]
#[test]
fn a_folder_that_cannot_be_listed_is_named_and_not_read_as_empty() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&root.join("imports"), b"not a folder");
    write(&root.join("bank-statements/s.json"), b"12345");
    let report = build(root, &[]);
    assert_eq!(report.folders_not_read, ["imports"]);
    assert_eq!(class(&report, "bank_statements").files, 1);
    let json = to_json(&report, SystemTime::now(), None);
    assert_eq!(
        json["folders_that_could_not_be_listed"],
        serde_json::json!(["imports"])
    );
    assert_eq!(json["entry_cap_reached"], false);
    assert_eq!(json["incomplete_reason"], "folder_not_listed");
}

/// A data folder that cannot be listed says the journal was not looked for,
/// not that it is absent. A file where the folder should be fails the listing
/// on any user (ENOTDIR).
#[cfg(unix)]
#[test]
fn an_unlistable_data_folder_does_not_read_as_an_absent_journal() {
    let directory = tempfile::tempdir().unwrap();
    let not_a_folder = directory.path().join("Bridge");
    fs::write(&not_a_folder, b"x").unwrap();
    let report = build(&not_a_folder, &[]);
    assert_eq!(report.root, Root::Unreadable);
    assert_eq!(report.journal, Journal::NotRead(DATA_FOLDER_UNREADABLE));
    let json = to_json(&report, SystemTime::now(), None);
    assert_eq!(json["journal"]["state"], "not_read");
    assert_eq!(json["incomplete_reason"], "data_folder_unreadable");
}

/// Text that is not UTF-8 is an I/O-class failure, read twice before it is
/// reported as `journal_read_failed`.
#[test]
fn a_journal_that_cannot_be_read_as_text_is_read_again_and_reported() {
    let directory = tempfile::tempdir().unwrap();
    write(
        &directory.path().join("agent-import-ledger.jsonl"),
        b"\xff\xfe\n",
    );
    let mut retried = false;
    let result = read_journal(directory.path(), || retried = true);
    assert_eq!(result, Journal::NotRead(JOURNAL_READ_FAILED));
    assert!(retried);
}

/// Every reason a report is incomplete is named, and a complete one names none.
#[test]
fn a_complete_report_has_no_reason_and_each_gap_has_its_own() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    assert_eq!(report.incomplete_reason(), None);
    assert!(to_json(&report, SystemTime::now(), None)["incomplete_reason"].is_null());
    report.entry_cap_reached = true;
    assert_eq!(report.incomplete_reason(), Some("entry_cap_reached"));
    assert_eq!(
        to_json(&report, SystemTime::now(), None)["entry_cap_reached"],
        true
    );
    report.unreadable = 1;
    assert_eq!(report.incomplete_reason(), Some("entries_not_read"));
    report.folders_not_read.push("lab");
    assert_eq!(report.incomplete_reason(), Some("folder_not_listed"));
    report.journal = Journal::NotRead(JOURNAL_INVALID);
    assert_eq!(report.incomplete_reason(), Some("journal_not_read"));
    report.root = Root::Unreadable;
    assert_eq!(report.incomplete_reason(), Some("data_folder_unreadable"));
}

/// Sockets, pipes and devices are counted apart from symlinks.
#[cfg(unix)]
#[test]
fn a_socket_is_not_a_symlink_and_is_not_a_file() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let _listener = std::os::unix::net::UnixListener::bind(root.join("s.sock")).unwrap();
    let report = build(root, &[]);
    assert_eq!(report.special_files, 1);
    assert_eq!(report.links, 0);
    assert_eq!(*class(&report, "other"), Class::default());
}

/// The exit status says whether the report is complete.
#[test]
fn the_exit_status_is_zero_only_for_a_complete_report() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    assert_eq!(exit_status(&report, true), 0);
    assert_eq!(exit_status(&report, false), 1);
    report.journal = Journal::NotRead(JOURNAL_UNREADABLE);
    assert_eq!(exit_status(&report, true), 3);
    report.journal = Journal::Absent;
    report.unreadable = 1;
    assert_eq!(exit_status(&report, true), 3);
    report.unreadable = 0;
    report.folders_not_read.push("imports");
    assert_eq!(exit_status(&report, true), 3);
    report.folders_not_read.clear();
    report.entry_cap_reached = true;
    assert_eq!(exit_status(&report, true), 3);
    report.entry_cap_reached = false;
    report.root = Root::Unreadable;
    assert_eq!(exit_status(&report, true), 2);
    report.root = Root::Missing;
    assert_eq!(exit_status(&report, true), 0);
}

/// An audit read a lab build kept is counted file by file as `lab`, its
/// manifest and every part, not as one folder with nothing in it.
#[test]
fn a_kept_audit_read_is_counted_with_its_files_and_bytes() {
    let directory = tempfile::tempdir().unwrap();
    let read = directory
        .path()
        .join("lab")
        .join("audit-reads")
        .join("a".repeat(64));
    write(&read.join("manifest.json"), b"12");
    write(&read.join("parts").join("ledgers.xml"), b"345");
    write(&read.join("parts").join("vouchers-2025-04-01.xml"), b"6789");
    write(&directory.path().join("lab").join("x.request.xml"), b"1");
    let report = build(directory.path(), &[]);
    assert_eq!(class(&report, "lab").files, 4);
    assert_eq!(class(&report, "lab").bytes, 10);
    assert_eq!(report.other_directories, 0);
}

/// A folder with more entries than the cap is listed up to the cap and reported
/// as capped, not as complete.
#[test]
fn a_folder_past_the_entry_cap_is_reported_as_capped() {
    let directory = tempfile::tempdir().unwrap();
    for name in ["a", "b", "c"] {
        write(&directory.path().join("lab").join(name), b"1");
    }
    let mut report = build(directory.path(), &[]);
    assert!(!report.entry_cap_reached);
    assert_eq!(class(&report, "lab").files, 3);
    report = build(&directory.path().join("absent"), &[]);
    scan_directory(
        &mut report,
        "lab",
        &directory.path().join("lab"),
        |_| "lab",
        &[],
        2,
    );
    assert!(report.entry_cap_reached);
    assert_eq!(class(&report, "lab").files, 2);
}

/// The tool's evidence says `partial` when the report could not read part of what
/// it reports on, and `complete` when it read all of it.
#[cfg(unix)]
#[tokio::test]
async fn the_tool_evidence_is_partial_when_a_folder_could_not_be_listed() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&root.join("imports"), b"not a folder");
    let server = server_over(root);
    let response = server
        .call_tool("local_data_report", serde_json::json!({}))
        .await;
    assert_eq!(response["isError"], false, "{response}");
    let evidence = &response["structuredContent"]["evidence"];
    assert_eq!(evidence["state"], "partial", "{response}");
    assert_eq!(evidence["reason_code"], "folder_not_listed");
    assert_eq!(
        response["structuredContent"]["result"]["incomplete_reason"],
        "folder_not_listed"
    );
}

/// A lease-lock folder that is a symlink is not entered, and the report says its
/// locks were not seen, once, however many candidates point at it.
#[cfg(unix)]
#[test]
fn a_linked_lock_folder_is_named_as_not_listed() {
    let directory = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&outside.path().join("a.lock"), b"");
    std::os::unix::fs::symlink(outside.path(), root.join("native-dispatch-leases")).unwrap();
    let report = build(root, &[root]);
    assert_eq!(report.folders_not_read, ["lock_folder"]);
    assert_eq!(class(&report, "locks").files, 0, "the link is not followed");
    assert_eq!(report.incomplete_reason(), Some("folder_not_listed"));
}

/// A data folder that is a link to a target that is gone is unreadable, not a
/// folder that was never made.
#[cfg(unix)]
#[test]
fn a_dangling_data_folder_link_is_unreadable_and_not_missing() {
    let directory = tempfile::tempdir().unwrap();
    let link = directory.path().join("Bridge");
    std::os::unix::fs::symlink(directory.path().join("gone"), &link).unwrap();
    let report = build(&link, &[]);
    assert_eq!(report.root, Root::Unreadable);
    assert_eq!(report.journal, Journal::NotRead(DATA_FOLDER_UNREADABLE));
    assert_eq!(exit_status(&report, true), 2);
}

/// The lease locks are the user's, so they are counted when the data folder does
/// not exist; a dangling coordination folder is named, not read as empty.
#[cfg(unix)]
#[test]
fn the_lock_folder_is_counted_when_the_data_folder_is_missing() {
    let directory = tempfile::tempdir().unwrap();
    let coordination = directory.path().join("shared");
    write(&coordination.join("native-dispatch-leases/a.lock"), b"");
    let missing = directory.path().join("nope");
    let report = build(&missing, &[coordination.as_path()]);
    assert_eq!(report.root, Root::Missing);
    assert_eq!(class(&report, "locks").files, 1);
    assert_eq!(report.incomplete_reason(), None);

    let dangling = directory.path().join("dangling");
    std::os::unix::fs::symlink(directory.path().join("gone"), &dangling).unwrap();
    let report = build(&missing, &[dangling.as_path()]);
    assert_eq!(report.folders_not_read, ["lock_folder"]);
    assert_eq!(report.incomplete_reason(), Some("folder_not_listed"));
}

fn master_journal_text(verdict: bool, truncated: bool) -> String {
    let id = sentinel_batch_id(0x15aa);
    let sha = "a".repeat(64);
    let batch = serde_json::json!({
        "record_type": "master_batch", "batch_id": id, "batch_sha256": sha,
        "class": "ledger_create",
        "company": {"guid": COMPANY_GUID, "name": "Synthetic",
            "number": "1", "books_from": "20260401"},
        "items": [{"name": "Synthetic One", "kind": "suspense", "bill_wise": false,
            "party_gstin": null, "gst_duty_head": null}]
    });
    let intent = serde_json::json!({
        "record_type": "master_dispatch_intent", "batch_id": id, "batch_sha256": sha,
        "native_request_sha256": "b".repeat(64),
        "items": [{"name": "Synthetic One", "canonical_key": "k", "absence_key": "a"}],
        "pre_image_sha256": "c".repeat(64), "pre_image_count": 2,
        "pre_post_master_marks": [{"company_guid": COMPANY_GUID,
            "altmstid": 7}]
    });
    let verdict_line = serde_json::json!({
        "record_type": "master_verdict", "batch_id": id, "batch_sha256": sha,
        "state": "posted_verified",
        "items": [{"name": "Synthetic One", "guid": sentinel_uuid(0x15ab).to_string(),
            "reason": null}]
    });
    // A verified create rests on Tally's answer, recorded before the verdict.
    let response = serde_json::json!({
        "record_type": "master_dispatch_response", "batch_id": id, "batch_sha256": sha,
        "response_sha256": "d".repeat(64), "counters": null
    });
    let mut text = format!("{batch}\n{intent}\n");
    if verdict {
        text.push_str(&format!("{response}\n{verdict_line}\n"));
    }
    if truncated {
        text.pop();
    }
    text
}

#[test]
fn a_master_journal_with_an_open_intent_is_shown_apart_from_the_voucher_journal() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let path = root.join("agent-master-ledger.jsonl");
    let master =
        |root: &Path| to_json(&build(root, &[]), SystemTime::now(), None)["master_journal"].clone();
    assert_eq!(master(root)["state"], "absent");
    write(&path, master_journal_text(false, false).as_bytes());
    let open = master(root);
    assert_eq!(open["state"], "read");
    assert_eq!(open["batches"], 1);
    assert_eq!(open["dispatched"], 1);
    assert_eq!(open["not_settled"], 1);
    // The voucher journal's own account is untouched by it.
    assert_eq!(
        to_json(&build(root, &[]), SystemTime::now(), None)["journal"]["state"],
        "absent"
    );
    fs::remove_file(&path).unwrap();
    write(&path, master_journal_text(true, false).as_bytes());
    assert_eq!(master(root)["not_settled"], 0);
}

#[test]
fn an_unreadable_master_journal_makes_the_report_incomplete() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(
        &root.join("agent-master-ledger.jsonl"),
        master_journal_text(false, true).as_bytes(),
    );
    let report = build(root, &[]);
    assert_eq!(
        report.master_journal,
        MasterJournal::NotRead(JOURNAL_INVALID)
    );
    assert_eq!(report.incomplete_reason(), Some("journal_not_read"));
}

#[test]
fn a_stage_left_beside_a_sidecar_is_counted_with_the_journal() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let id = sentinel_batch_id(0x15aa);
    write(
        &root.join(format!(
            "master-pre-image-{id}.{}.next",
            sentinel_uuid(0x15bb)
        )),
        b"123",
    );
    let report = build(root, &[]);
    assert_eq!(*class(&report, "journal"), Class { files: 1, bytes: 3 });
    assert_eq!(*class(&report, "other"), Class { files: 0, bytes: 0 });
}

/// A batch a person reviewed while it was matched by content only is counted
/// in its own class, and in no other class of unsettled batch (#1039).
#[test]
fn a_reviewed_batch_is_its_own_class_of_unsettled_batch() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.journal = Journal::Read(super::super::ledger::Settlement {
        batches: 5,
        sent_or_found: 5,
        unsettled: 4,
        unsettled_no_response: 1,
        unsettled_binding_refused: 1,
        unsettled_reviewed: 1,
        unsettled_restored: 0,
        settled_by_person: 0,
        released_by_person: 0,
        no_dispatch_never_verified: 0,
    });
    let journal = &to_json(&report, SystemTime::now(), None)["journal"];
    assert_eq!(journal["not_settled"], 4);
    assert_eq!(journal["not_settled_reviewed_not_attributed"], 1);
    assert_eq!(journal["not_settled_restored"], 0);
    assert_eq!(journal["not_settled_binding_refused"], 1);
    assert_eq!(journal["not_settled_not_verified"], 1);
}

/// Native posts a recorded restore covers are reported in their own count,
/// and `not_settled_not_verified` holds only what no other count does (#1050).
#[test]
fn restored_posts_are_their_own_class_of_unsettled_batch() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.journal = Journal::Read(super::super::ledger::Settlement {
        batches: 6,
        sent_or_found: 6,
        unsettled: 5,
        unsettled_no_response: 1,
        unsettled_binding_refused: 0,
        unsettled_reviewed: 0,
        unsettled_restored: 2,
        settled_by_person: 0,
        released_by_person: 0,
        no_dispatch_never_verified: 0,
    });
    let journal = &to_json(&report, SystemTime::now(), None)["journal"];
    assert_eq!(journal["not_settled"], 5);
    assert_eq!(journal["not_settled_restored"], 2);
    assert_eq!(journal["not_settled_no_response"], 1);
    assert_eq!(journal["not_settled_not_verified"], 2);
}

/// A batch a person settled after Tally answered that its post created none
/// of its vouchers is reported in its own count, outside `not_settled` (#1709).
#[test]
fn a_settled_batch_is_reported_outside_not_settled() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.journal = Journal::Read(super::super::ledger::Settlement {
        batches: 5,
        sent_or_found: 5,
        unsettled: 3,
        unsettled_no_response: 1,
        unsettled_binding_refused: 1,
        unsettled_reviewed: 0,
        unsettled_restored: 0,
        settled_by_person: 2,
        released_by_person: 0,
        no_dispatch_never_verified: 0,
    });
    let journal = &to_json(&report, SystemTime::now(), None)["journal"];
    assert_eq!(journal["not_settled"], 3);
    assert_eq!(journal["settled_by_person"], 2);
    assert_eq!(journal["not_settled_not_verified"], 1);
}

/// A restored batch whose rows a person released is reported in its own
/// count, outside `not_settled` and `not_settled_restored` (#1038).
#[test]
fn a_released_batch_is_reported_outside_not_settled() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.journal = Journal::Read(super::super::ledger::Settlement {
        batches: 4,
        sent_or_found: 4,
        unsettled: 2,
        unsettled_no_response: 0,
        unsettled_binding_refused: 0,
        unsettled_reviewed: 0,
        unsettled_restored: 1,
        settled_by_person: 0,
        released_by_person: 2,
        no_dispatch_never_verified: 0,
    });
    let journal = &to_json(&report, SystemTime::now(), None)["journal"];
    assert_eq!(journal["not_settled"], 2);
    assert_eq!(journal["not_settled_restored"], 1);
    assert_eq!(journal["released_by_person"], 2);
    assert_eq!(journal["settled_by_person"], 0);
    assert_eq!(journal["not_settled_not_verified"], 1);
}

// The tests below hold what the #2058 mutation record of this file shows the
// ones above leave open. Each says which kind of change it catches.
use serde_json::json;

/// Set the modification time of the file at `path` to `seconds` after the epoch.
fn set_mtime(path: &Path, seconds: u64) {
    fs::File::options()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(seconds))
        .unwrap();
}

/// A report of nothing, to account into by hand.
fn empty_report(directory: &Path) -> Report {
    build(&directory.join("absent"), &[])
}

/// A folder made unreadable or unsearchable for the length of a test; the
/// mode it had comes back when this is dropped, even by a failed assertion.
#[cfg(unix)]
struct Restricted {
    path: PathBuf,
    mode: u32,
}

#[cfg(unix)]
impl Restricted {
    fn new(path: &Path, mode: u32) -> Self {
        use std::os::unix::fs::PermissionsExt;
        let before = fs::metadata(path).unwrap().permissions().mode();
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
        Self {
            path: path.to_path_buf(),
            mode: before,
        }
    }
}

#[cfg(unix)]
impl Drop for Restricted {
    fn drop(&mut self) {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&self.path, fs::Permissions::from_mode(self.mode));
    }
}

/// Every suffix `import_class` knows gives its class, and the suffix cut short
/// gives none: a suffix changed, dropped or joined to the others by the wrong
/// operator moves a name from one class to another.
#[test]
fn each_import_file_suffix_has_its_class_and_a_near_miss_has_none() {
    let suffixes = [
        (".xml", "import_files"),
        (".proof.json", "proofs"),
        (".proof.md", "proofs"),
        (".approval_lapse.json", "approval_notes"),
        (".masters_check.json", "review_records"),
        (".masters_doubt.json", "review_records"),
        (".masters_ack.json", "review_records"),
        (".batch_step_doubt.json", "review_records"),
        (".batch_step_ack.json", "review_records"),
        (".masters_verdict.json", "review_records"),
        (".baseline.json", "review_records"),
    ];
    for (suffix, class) in suffixes {
        assert_eq!(import_class(&format!("b{suffix}")), class, "{suffix}");
        let cut = &suffix[..suffix.len() - 1];
        assert_eq!(import_class(&format!("b{cut}")), "other", "{cut}");
    }
    // A saved proof carries a stamp between `.proof.` and its type.
    assert_eq!(import_class("b.proof.2026-stamp.json"), "proofs");
    assert_eq!(import_class("b.proof.2026-stamp.md"), "proofs");
    // `.proof.` is the marker, not `.proof`.
    assert_eq!(import_class("b.proofread.json"), "other");
    assert_eq!(import_class("b.proofread.md"), "other");
}

/// `<batch>.baseline.<n>.json` is a baseline addition when `<n>` is one or more
/// decimal digits, and nothing else is.
#[test]
fn a_baseline_addition_is_a_batch_baseline_and_decimal_digits() {
    let names = [
        ("b.baseline.1.json", true),
        ("b.baseline.12.json", true),
        ("b.baseline.json", false),
        ("b.other.1.json", false),
        ("b.baseline..json", false),
        ("b.baseline.1x.json", false),
        ("b.baseline.x1.json", false),
        ("b.baseline.a.json", false),
        ("b.baseline.1.txt", false),
    ];
    for (name, expected) in names {
        assert_eq!(is_baseline_addition(name), expected, "{name}");
        assert_eq!(
            import_class(name),
            if expected || name == "b.baseline.json" {
                "review_records"
            } else {
                "other"
            },
            "{name}"
        );
    }
}

/// An empty folder lists every class, each at zero: a class the report never
/// names is not a class with nothing in it.
#[test]
fn an_empty_folder_lists_all_ten_classes_at_zero() {
    let directory = tempfile::tempdir().unwrap();
    let report = build(directory.path(), &[]);
    assert_eq!(
        report.classes.keys().copied().collect::<Vec<_>>(),
        [
            "approval_notes",
            "bank_statements",
            "egress_log",
            "import_files",
            "journal",
            "lab",
            "locks",
            "other",
            "proofs",
            "review_records"
        ]
    );
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(shown["classes"].as_object().unwrap().len(), 10, "{shown}");
}

/// The oldest time of a class is the smallest of its files', whichever file is
/// met first.
#[test]
fn the_oldest_time_of_a_class_is_the_smallest_whichever_file_is_met_first() {
    let directory = tempfile::tempdir().unwrap();
    let old = directory.path().join("old");
    let new = directory.path().join("new");
    write(&old, b"1");
    write(&new, b"22");
    set_mtime(&old, 1_000_000);
    set_mtime(&new, 2_000_000);
    for order in [[&old, &new], [&new, &old]] {
        let mut report = empty_report(directory.path());
        for path in order {
            report.account("lab", path);
        }
        assert_eq!(
            report.oldest["lab"],
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000)
        );
        assert_eq!(*class(&report, "lab"), Class { files: 2, bytes: 3 });
    }
}

/// A path whose metadata cannot be read is one unreadable entry: not a file,
/// not a link, not a special file, and counted once.
#[test]
fn a_path_whose_metadata_cannot_be_read_is_one_unreadable_entry() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = empty_report(directory.path());
    report.account("lab", &directory.path().join("gone"));
    assert_eq!(
        (report.unreadable, report.links, report.special_files),
        (1, 0, 0)
    );
    assert_eq!(*class(&report, "lab"), Class::default());
    assert!(report.oldest.is_empty());
}

/// A publication folder of either kind is an interrupted write wherever it is
/// found, and is not an unknown folder.
#[test]
fn an_interrupted_proof_or_build_publication_folder_is_counted_where_it_is_found() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    fs::create_dir_all(root.join(".build-publication")).unwrap();
    fs::create_dir_all(root.join("imports/.proof-publication")).unwrap();
    fs::create_dir_all(root.join("imports/.build-publication")).unwrap();
    let report = build(root, &[]);
    assert_eq!(report.interrupted_writes, 3);
    assert_eq!(report.other_directories, 0);
}

/// Each scan knows only the folders it is given: a folder of another name, in
/// the data folder, in each of its subfolders, in the lease folder, in a kept
/// audit read and in its parts, is counted and not entered.
#[test]
fn a_subfolder_is_unknown_to_every_scan_that_does_not_name_it() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let read = root.join("lab").join("audit-reads").join("r");
    for folder in [
        root.join("imports/x"),
        root.join("bank-statements/x"),
        root.join("lab/x"),
        root.join("native-dispatch-leases/x"),
        read.join("x"),
        read.join("parts").join("x"),
    ] {
        fs::create_dir_all(folder).unwrap();
    }
    let report = build(root, &[]);
    assert_eq!(report.other_directories, 6);
    assert_eq!(report.incomplete_reason(), None);
}

/// A kept audit read is counted whatever is in it: a file straight in
/// `audit-reads`, two files straight in a read, and its parts.
#[test]
fn a_file_among_the_kept_audit_reads_and_two_in_a_read_are_all_lab_files() {
    let directory = tempfile::tempdir().unwrap();
    let reads = directory.path().join("lab").join("audit-reads");
    write(&reads.join("stray.txt"), b"1");
    write(&reads.join("r").join("manifest.json"), b"22");
    write(&reads.join("r").join("notes.json"), b"333");
    write(&reads.join("r").join("parts").join("p.xml"), b"4444");
    let report = build(directory.path(), &[]);
    assert_eq!(
        *class(&report, "lab"),
        Class {
            files: 4,
            bytes: 10
        }
    );
    assert_eq!(*class(&report, "other"), Class::default());
    assert!(!report.entry_cap_reached);
    assert!(report.folders_not_read.is_empty());
}

/// The cap is 100,000 entries of one folder: a kept-reads folder with that many
/// is complete and one more is capped, with the first 100,000 counted. (The
/// same constant caps every listing the report makes.)
#[test]
fn a_folder_of_exactly_the_entry_cap_is_complete_and_one_more_is_capped() {
    let directory = tempfile::tempdir().unwrap();
    let reads = directory.path().join("lab").join("audit-reads");
    fs::create_dir_all(&reads).unwrap();
    for n in 0..100_000 {
        fs::File::create(reads.join(n.to_string())).unwrap();
    }
    let whole = build(directory.path(), &[]);
    assert!(!whole.entry_cap_reached);
    assert_eq!(class(&whole, "lab").files, 100_000);
    assert_eq!(whole.incomplete_reason(), None);
    fs::File::create(reads.join("one-more")).unwrap();
    let capped = build(directory.path(), &[]);
    assert!(capped.entry_cap_reached);
    assert_eq!(class(&capped, "lab").files, 100_000);
    assert_eq!(capped.incomplete_reason(), Some("entry_cap_reached"));
}

/// A lease folder is counted whole: two locks in it are two, and the listing
/// is not capped at one.
#[test]
fn two_locks_in_a_lease_folder_are_two_files_and_no_cap() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&root.join("native-dispatch-leases/a.lock"), b"");
    write(&root.join("native-dispatch-leases/b.lock"), b"");
    let report = build(root, &[]);
    assert_eq!(class(&report, "locks").files, 2);
    assert!(!report.entry_cap_reached);
}

/// One lease folder reached by two spellings is scanned once: the folders are
/// told apart by their canonical path.
#[test]
fn a_lease_folder_reached_by_two_spellings_is_counted_once() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("data");
    write(&root.join("native-dispatch-leases/a.lock"), b"");
    fs::create_dir_all(root.join("sub")).unwrap();
    let spelled = root.join("sub").join("..");
    let report = build(&root, &[spelled.as_path()]);
    assert_eq!(class(&report, "locks").files, 1);
}

/// A lease folder that is not a folder cannot be listed, and is named as the
/// lock folder, not as another.
#[cfg(unix)]
#[test]
fn a_lease_folder_that_is_a_file_is_named_as_the_lock_folder() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&root.join("native-dispatch-leases"), b"not a folder");
    let report = build(root, &[]);
    assert_eq!(report.folders_not_read, ["lock_folder"]);
}

/// A coordination folder that is a link to a folder that is there is not a
/// dangling link: its leases are read, and nothing is named as not listed.
#[cfg(unix)]
#[test]
fn a_coordination_folder_that_is_a_working_link_is_read_and_not_dangling() {
    let directory = tempfile::tempdir().unwrap();
    let real = directory.path().join("real");
    write(&real.join("native-dispatch-leases/a.lock"), b"");
    let link = directory.path().join("shared");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    let report = build(&directory.path().join("data"), &[link.as_path()]);
    assert_eq!(class(&report, "locks").files, 1);
    assert!(report.folders_not_read.is_empty());
    assert_eq!(report.incomplete_reason(), None);
}

/// A folder the report listed but cannot look inside (read, not searched):
/// each entry is one unreadable entry, and no file of the class.
#[cfg(unix)]
#[test]
fn entries_of_a_folder_that_cannot_be_searched_are_each_one_unreadable_entry() {
    let directory = tempfile::tempdir().unwrap();
    let folder = directory.path().join("closed");
    write(&folder.join("a"), b"1");
    write(&folder.join("b"), b"22");
    let _closed = Restricted::new(&folder, 0o444);
    if fs::symlink_metadata(folder.join("a")).is_ok() {
        // A user who reads through any mode (root) cannot make the case.
        return;
    }
    let mut listed = empty_report(directory.path());
    scan_directory(&mut listed, "lab", &folder, |_| "lab", &[], ENTRY_CAP);
    assert_eq!(listed.unreadable, 2);
    assert_eq!(*class(&listed, "lab"), Class::default());
    assert!(listed.folders_not_read.is_empty());
    // The same below `audit-reads`.
    let mut reads = empty_report(directory.path());
    scan_audit_reads(&mut reads, &folder);
    assert_eq!(reads.unreadable, 2);
    assert_eq!(*class(&reads, "lab"), Class::default());
    assert!(reads.folders_not_read.is_empty());
}

/// Whatever keeps a kept audit read from being listed is named `lab`, and only
/// that: `audit-reads` itself, a read, and its parts.
#[cfg(unix)]
#[test]
fn a_kept_audit_read_that_cannot_be_listed_is_named_lab_and_nothing_else() {
    // `audit-reads` is a file.
    let directory = tempfile::tempdir().unwrap();
    write(&directory.path().join("lab/audit-reads"), b"not a folder");
    let report = build(directory.path(), &[]);
    assert_eq!(report.folders_not_read, ["lab"]);
    // `parts` is a file.
    let directory = tempfile::tempdir().unwrap();
    let read = directory.path().join("lab/audit-reads/r");
    write(&read.join("manifest.json"), b"1");
    write(&read.join("parts"), b"not a folder");
    let report = build(directory.path(), &[]);
    assert_eq!(report.folders_not_read, ["lab"]);
    assert_eq!(class(&report, "lab").files, 2);
    // A read that cannot be listed.
    let directory = tempfile::tempdir().unwrap();
    let read = directory.path().join("lab/audit-reads/r");
    write(&read.join("manifest.json"), b"1");
    let _closed = Restricted::new(&read, 0o000);
    if fs::read_dir(&read).is_ok() {
        return;
    }
    let report = build(directory.path(), &[]);
    assert_eq!(report.folders_not_read, ["lab"]);
}

/// Links in the kept audit reads are counted and never followed: not
/// `audit-reads` as a link, not a link standing for a read, not a `parts` link,
/// and a dangling one is a link, not an unreadable entry.
#[cfg(unix)]
#[test]
fn links_among_the_kept_audit_reads_are_counted_and_never_entered() {
    let outside = tempfile::tempdir().unwrap();
    write(&outside.path().join("r").join("manifest.json"), b"1");
    write(&outside.path().join("p.xml"), b"22");
    // `audit-reads` itself is a link.
    let directory = tempfile::tempdir().unwrap();
    fs::create_dir_all(directory.path().join("lab")).unwrap();
    std::os::unix::fs::symlink(
        outside.path(),
        directory.path().join("lab").join("audit-reads"),
    )
    .unwrap();
    let report = build(directory.path(), &[]);
    assert_eq!(*class(&report, "lab"), Class::default());
    assert_eq!(report.links, 1);
    // A link for a read, a dangling one, and a read whose `parts` is a link.
    let directory = tempfile::tempdir().unwrap();
    let reads = directory.path().join("lab").join("audit-reads");
    fs::create_dir_all(&reads).unwrap();
    std::os::unix::fs::symlink(outside.path().join("r"), reads.join("linked")).unwrap();
    std::os::unix::fs::symlink(outside.path().join("gone"), reads.join("dangling")).unwrap();
    write(&reads.join("real").join("manifest.json"), b"333");
    std::os::unix::fs::symlink(outside.path(), reads.join("real").join("parts")).unwrap();
    let report = build(directory.path(), &[]);
    assert_eq!(*class(&report, "lab"), Class { files: 1, bytes: 3 });
    assert_eq!(report.links, 3);
    assert_eq!(report.unreadable, 0);
    assert!(report.folders_not_read.is_empty());
}

/// A link standing for a lock folder or a file in `--show-paths` is shown as
/// the files directly in a folder, not as whatever a link points at.
#[cfg(unix)]
#[test]
fn a_link_among_the_files_of_a_folder_is_not_a_file_of_it() {
    let directory = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    write(&outside.path().join("real.jsonl"), b"1");
    std::os::unix::fs::symlink(
        outside.path().join("real.jsonl"),
        directory.path().join("link.jsonl"),
    )
    .unwrap();
    let resolve = |path: &Path| VisiblePath::AsIs(path.to_path_buf());
    let shown = file_locations(directory.path(), &resolve);
    assert_eq!(shown["files"], 0, "{shown}");
    assert_eq!(shown["files_read"], 0, "{shown}");
}

/// The cap counts every entry of the folder, a subfolder as much as a file.
#[test]
fn the_listing_cap_counts_folders_as_well_as_files() {
    let directory = tempfile::tempdir().unwrap();
    for name in ["a", "b"] {
        write(&directory.path().join(name), b"");
    }
    for name in ["c", "d"] {
        fs::create_dir(directory.path().join(name)).unwrap();
    }
    let resolve = |path: &Path| VisiblePath::AsIs(path.to_path_buf());
    let capped = file_locations_capped(directory.path(), &resolve, 3);
    assert_eq!(capped["listing_capped"], true, "{capped}");
    let whole = file_locations_capped(directory.path(), &resolve, 4);
    assert_eq!(whole["listing_capped"], false, "{whole}");
    assert_eq!(whole["files"], 2, "{whole}");
}

/// The folder of a file with no parent is shown as no folder, and a folder is
/// listed once for each way it is found (its state is part of the key).
#[test]
fn a_parentless_path_is_shown_with_no_folder_and_a_state_is_part_of_the_key() {
    let directory = tempfile::tempdir().unwrap();
    write(&directory.path().join("a"), b"");
    let bare = |_: &Path| VisiblePath::AsIs(PathBuf::new());
    assert_eq!(
        file_locations(directory.path(), &bare)["folders"],
        json!([{"path": "", "state": "as_is"}])
    );
    write(&directory.path().join("b"), b"");
    let folder = PathBuf::from("/one");
    let mixed = |path: &Path| {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if name == "a" {
            VisiblePath::Resolved(folder.join(&name))
        } else {
            VisiblePath::Unresolved(folder.join(&name))
        }
    };
    assert_eq!(
        file_locations(directory.path(), &mixed)["folders"],
        json!([
            {"path": folder.display().to_string(), "state": "resolved"},
            {"path": folder.display().to_string(), "state": "unresolved"},
        ])
    );
}

/// The sample is 64 files: of seventy, 64 are read.
#[test]
fn of_seventy_files_the_locations_of_sixty_four_are_read() {
    let directory = tempfile::tempdir().unwrap();
    for n in 0..70 {
        write(&directory.path().join(format!("f{n:03}")), b"");
    }
    let resolve = |path: &Path| VisiblePath::AsIs(path.to_path_buf());
    let shown = file_locations(directory.path(), &resolve);
    assert_eq!(shown["files_read"], 64, "{shown}");
    assert_eq!(shown["files"], 70, "{shown}");
}

/// A package copy is a folder; a file of that name is none. The copies come in
/// name order, not the order the folder lists them in.
#[test]
fn package_copies_are_folders_and_come_in_name_order() {
    let directory = tempfile::tempdir().unwrap();
    let local = directory.path().join("Local");
    let tail = |package: &str| {
        local
            .join("Packages")
            .join(package)
            .join("LocalCache")
            .join("Local")
            .join("Bridge")
    };
    let names: Vec<String> = (0..12).map(|n| format!("Store.App_{n:02}")).collect();
    for name in names.iter().rev() {
        fs::create_dir_all(tail(name).join("agent")).unwrap();
    }
    write(
        &tail("Store.App_file").join("agent"),
        b"a file, not a folder",
    );
    let folder = local.join("Bridge").join("agent");
    let expected: Vec<PathBuf> = names.iter().map(|name| tail(name).join("agent")).collect();
    assert_eq!(
        package_copy_folders(&local, &folder),
        PackageCopies::Found(expected.clone())
    );
    assert_eq!(
        package_copies(&local, &folder),
        json!(expected
            .iter()
            .map(|copy| copy.display().to_string())
            .collect::<Vec<_>>())
    );
}

/// A packages folder that cannot be listed is said so: a copy may exist.
#[cfg(unix)]
#[test]
fn a_packages_folder_that_cannot_be_listed_is_not_listed() {
    let directory = tempfile::tempdir().unwrap();
    let local = directory.path().join("Local");
    write(&local.join("Packages"), b"not a folder");
    let folder = local.join("Bridge").join("agent");
    assert_eq!(
        package_copy_folders(&local, &folder),
        PackageCopies::NotListed
    );
    assert_eq!(package_copies(&local, &folder), json!("not_listed"));
    // No packages folder at all is no copy, not an unknown.
    let bare = directory.path().join("Bare");
    fs::create_dir_all(&bare).unwrap();
    assert_eq!(
        package_copy_folders(&bare, &bare.join("Bridge")),
        PackageCopies::Found(Vec::new())
    );
}

/// How old a file is counts whole days to the second, and `to_json` counts them
/// from the time it is given.
#[test]
fn an_age_changes_on_the_second_a_day_is_complete_and_to_json_uses_its_clock() {
    let now = SystemTime::now();
    let ago = |seconds| now - Duration::from_secs(seconds);
    assert_eq!(age_days(ago(86_399), now), 0);
    assert_eq!(age_days(ago(86_400), now), 1);
    assert_eq!(age_days(ago(172_799), now), 1);
    assert_eq!(age_days(ago(172_800), now), 2);
    let directory = tempfile::tempdir().unwrap();
    let file = directory.path().join("agent-egress.jsonl");
    write(&file, b"1");
    let modified = fs::metadata(&file).unwrap().modified().unwrap();
    let report = build(directory.path(), &[]);
    let later = modified + Duration::from_secs(5 * 86_400 + 3_600);
    let shown = to_json(&report, later, None);
    assert_eq!(shown["classes"]["egress_log"]["oldest_days"], 5, "{shown}");
}

/// The root is `present`, `missing` or `unreadable` in the JSON, and a folder
/// that is missing or unreadable has no journal line.
#[test]
fn the_folder_word_says_present_missing_or_unreadable() {
    let directory = tempfile::tempdir().unwrap();
    let present = build(directory.path(), &[]);
    assert_eq!(
        to_json(&present, SystemTime::now(), None)["folder"],
        "present"
    );
    let missing = empty_report(directory.path());
    assert_eq!(missing.root, Root::Missing);
    assert_eq!(missing.journal_line, None);
    assert_eq!(missing.master_journal, MasterJournal::Absent);
    assert_eq!(
        to_json(&missing, SystemTime::now(), None)["folder"],
        "missing"
    );
}

/// A data folder that cannot be listed: the report says so for both journals,
/// names no line, and still counts the user's own lock folder.
#[cfg(unix)]
#[test]
fn an_unlistable_data_folder_says_so_for_both_journals_and_still_counts_the_locks() {
    let directory = tempfile::tempdir().unwrap();
    let not_a_folder = directory.path().join("Bridge");
    fs::write(&not_a_folder, b"x").unwrap();
    let coordination = directory.path().join("shared");
    write(&coordination.join("native-dispatch-leases/a.lock"), b"");
    let report = build(&not_a_folder, &[coordination.as_path()]);
    assert_eq!(report.root, Root::Unreadable);
    assert_eq!(report.journal_line, None);
    assert_eq!(
        report.master_journal,
        MasterJournal::NotRead(DATA_FOLDER_UNREADABLE)
    );
    assert_eq!(class(&report, "locks").files, 1);
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(shown["folder"], "unreadable");
    let reason = json!({"state": "not_read", "reason": "data_folder_unreadable"});
    assert_eq!(shown["journal"], reason);
    assert_eq!(shown["master_journal"], reason);
}

/// Each count is shown under its own key, and a key that is not one of them
/// shows nothing.
#[test]
fn each_count_is_shown_under_its_own_key() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.links = 1;
    report.special_files = 2;
    report.other_directories = 3;
    report.interrupted_writes = 4;
    report.unreadable = 5;
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(shown["symlinks_not_followed"], 1, "{shown}");
    assert_eq!(shown["special_files_not_counted"], 2, "{shown}");
    assert_eq!(shown["other_directories_not_entered"], 3, "{shown}");
    assert_eq!(shown["interrupted_write_folders"], 4, "{shown}");
    assert_eq!(shown["entries_that_could_not_be_read"], 5, "{shown}");
    assert_eq!(shown["folders_that_could_not_be_listed"], json!([]));
}

/// A journal that is read is shown with each of its counts under its own key.
#[test]
fn a_journal_that_is_read_shows_each_count_under_its_own_key() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.journal = Journal::Read(super::super::ledger::Settlement {
        batches: 30,
        sent_or_found: 29,
        unsettled: 28,
        unsettled_no_response: 2,
        unsettled_binding_refused: 3,
        unsettled_reviewed: 4,
        unsettled_restored: 5,
        settled_by_person: 7,
        released_by_person: 8,
        no_dispatch_never_verified: 9,
    });
    report.master_journal = MasterJournal::Read(super::super::master_ledger::MasterSummary {
        batches: 13,
        dispatched: 11,
        not_settled: 6,
    });
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(
        shown["journal"],
        json!({
            "state": "read",
            "batches": 30,
            "sent_or_found_posted": 29,
            "not_settled": 28,
            "not_settled_no_response": 2,
            "not_settled_binding_refused": 3,
            "not_settled_reviewed_not_attributed": 4,
            "not_settled_restored": 5,
            "settled_by_person": 7,
            "released_by_person": 8,
            "not_settled_not_verified": 14,
            "no_dispatch_never_verified": 9,
        })
    );
    assert_eq!(
        shown["master_journal"],
        json!({"state": "read", "batches": 13, "dispatched": 11, "not_settled": 6})
    );
}

/// A journal that is not read is shown with its reason, and with its line only
/// when it was refused as invalid and the line is known.
#[test]
fn a_journal_that_is_not_read_shows_its_reason_and_a_line_only_for_an_invalid_one() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.journal = Journal::NotRead(JOURNAL_UNREADABLE);
    report.master_journal = MasterJournal::NotRead(JOURNAL_READ_FAILED);
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(
        shown["journal"],
        json!({"state": "not_read", "reason": "journal_unreadable"})
    );
    assert_eq!(
        shown["master_journal"],
        json!({"state": "not_read", "reason": "journal_read_failed"})
    );
    // A line is known for an invalid journal; the other reasons show none.
    report.journal_line = Some(7);
    report.journal = Journal::NotRead(JOURNAL_READ_FAILED);
    report.master_journal = MasterJournal::NotRead(JOURNAL_INVALID);
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(
        shown["journal"],
        json!({"state": "not_read", "reason": "journal_read_failed"})
    );
    assert_eq!(
        shown["master_journal"],
        json!({"state": "not_read", "reason": "journal_invalid"})
    );
    report.journal = Journal::NotRead(JOURNAL_INVALID);
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(
        shown["journal"],
        json!({"state": "not_read", "reason": "journal_invalid", "line": 7})
    );
    // An invalid journal with no known line shows none either.
    report.journal_line = None;
    let shown = to_json(&report, SystemTime::now(), None);
    assert_eq!(
        shown["journal"],
        json!({"state": "not_read", "reason": "journal_invalid"})
    );
}

/// Each reason a journal is not read is its own word, from a real file: a
/// folder where the journal should be cannot be opened, and text that is not
/// UTF-8 cannot be read as text.
#[test]
fn each_reason_a_journal_is_not_read_is_its_own_word_from_a_real_file() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    fs::create_dir(root.join("agent-import-ledger.jsonl")).unwrap();
    write(&root.join("agent-master-ledger.jsonl"), b"\xff\xfe\n");
    let shown = to_json(&build(root, &[]), SystemTime::now(), None);
    assert_eq!(
        shown["journal"],
        json!({"state": "not_read", "reason": "journal_unreadable"})
    );
    assert_eq!(
        shown["master_journal"],
        json!({"state": "not_read", "reason": "journal_invalid"})
    );
    fs::remove_dir(root.join("agent-import-ledger.jsonl")).unwrap();
    fs::remove_file(root.join("agent-master-ledger.jsonl")).unwrap();
    write(&root.join("agent-import-ledger.jsonl"), b"\xff\xfe\n");
    fs::create_dir(root.join("agent-master-ledger.jsonl")).unwrap();
    let shown = to_json(&build(root, &[]), SystemTime::now(), None);
    assert_eq!(
        shown["journal"],
        json!({"state": "not_read", "reason": "journal_read_failed"})
    );
    assert_eq!(
        shown["master_journal"],
        json!({"state": "not_read", "reason": "journal_unreadable"})
    );
}

/// A journal that cannot be opened is not read a second time, and names no
/// line; one the report reads as text it cannot is read again and names none;
/// the first line refused is line 1; and the line of the second read is the
/// one reported.
#[test]
fn the_journal_retry_follows_why_it_was_not_read_and_reports_the_second_line() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let path = root.join("agent-import-ledger.jsonl");
    fs::create_dir(&path).unwrap();
    let mut waited = false;
    assert_eq!(
        read_journal_at(root, || waited = true),
        (Journal::NotRead(JOURNAL_UNREADABLE), None)
    );
    assert!(!waited, "a journal that cannot be opened is not read again");
    fs::remove_dir(&path).unwrap();
    write(&path, b"\xff\xfe\n");
    assert_eq!(
        read_journal_at(root, || waited = true),
        (Journal::NotRead(JOURNAL_READ_FAILED), None)
    );
    assert!(waited);
    write(&path, b"not json\n");
    assert_eq!(
        read_journal_at(root, || {}),
        (Journal::NotRead(JOURNAL_INVALID), Some(1))
    );
    // Refused at line 2 the first time, and at line 3 the second.
    let good = String::from_utf8(journal(false)).unwrap();
    let with_id =
        |id: &str| good.replacen("\"batch_id\":\"b1\"", &format!("\"batch_id\":\"{id}\""), 1);
    let bad = with_id("b2").replacen("\"amount\":\"1.00\"", "\"amount\":\"-1.00\"", 1);
    write(&path, format!("{good}{bad}").as_bytes());
    let longer = format!("{good}{}{bad}", with_id("b3"));
    assert_eq!(
        read_journal_at(root, || fs::write(&path, &longer).unwrap()),
        (Journal::NotRead(JOURNAL_INVALID), Some(3))
    );
}

/// The master journal is read as the voucher journal is: once when it is whole,
/// absent or cannot be opened; again after a refusal a write in progress could
/// explain; and the second read is the one reported.
#[test]
fn the_master_journal_is_read_again_only_after_a_refusal_a_write_could_explain() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let path = root.join("agent-master-ledger.jsonl");
    let mut waited = false;
    assert_eq!(
        read_master_journal(root, || waited = true),
        MasterJournal::Absent
    );
    assert!(!waited);
    write(&path, master_journal_text(false, false).as_bytes());
    assert!(matches!(
        read_master_journal(root, || waited = true),
        MasterJournal::Read(_)
    ));
    assert!(!waited, "a whole journal is read once");
    // A half-written last line: read again, and healed if the writer finished.
    write(&path, master_journal_text(false, true).as_bytes());
    let healed = read_master_journal(root, || {
        fs::write(&path, master_journal_text(false, false)).unwrap();
    });
    assert!(matches!(healed, MasterJournal::Read(_)), "{healed:?}");
    // One that stays refused is said invalid, after the wait.
    write(&path, master_journal_text(false, true).as_bytes());
    assert_eq!(
        read_master_journal(root, || waited = true),
        MasterJournal::NotRead(JOURNAL_INVALID)
    );
    assert!(waited);
    // One that cannot be opened is not read again.
    fs::remove_file(&path).unwrap();
    fs::create_dir(&path).unwrap();
    waited = false;
    assert_eq!(
        read_master_journal(root, || waited = true),
        MasterJournal::NotRead(JOURNAL_UNREADABLE)
    );
    assert!(!waited);
}

/// The exit status is 1 whenever the report could not be printed, whatever it
/// says of the folder.
#[test]
fn a_report_that_could_not_be_printed_exits_one_whatever_it_says() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), &[]);
    report.root = Root::Unreadable;
    assert_eq!(exit_status(&report, false), 1);
    report.root = Root::Present;
    report.unreadable = 1;
    assert_eq!(exit_status(&report, false), 1);
}

/// The environment variable that names the child's work.
const CHILD_MODE: &str = "BRIDGE_LOCAL_DATA_REPORT_TEST_CHILD";

/// Run this test binary again, running only [`local_data_report_child`] in
/// `mode`, and return what it printed and how it ended. The report command
/// prints and exits, so it is run in a process of its own.
fn run_child(
    mode: &str,
    configure: impl FnOnce(&mut std::process::Command),
) -> std::process::Output {
    let module = module_path!()
        .strip_prefix(concat!(env!("CARGO_CRATE_NAME"), "::"))
        .expect("crate module prefix");
    let mut command = std::process::Command::new(std::env::current_exe().expect("test executable"));
    command
        .arg("--exact")
        .arg(format!("{module}::local_data_report_child"))
        .arg("--nocapture")
        .env(CHILD_MODE, mode);
    configure(&mut command);
    command.output().expect("local data report child")
}

/// The JSON a child printed: the test harness's own first lines come before it.
fn child_json(output: &std::process::Output) -> (serde_json::Value, String) {
    let text = String::from_utf8(output.stdout.clone()).unwrap();
    let start = text
        .find('{')
        .unwrap_or_else(|| panic!("no report on stdout: {text}"));
    let body = text[start..].to_string();
    let value = serde_json::from_str(&body).unwrap_or_else(|error| panic!("{error}: {text}"));
    (value, body)
}

/// The child: with [`CHILD_MODE`] set, does what the report command does (or
/// reports what a Windows-only helper finds) and ends the process; without it,
/// it does nothing, and passes.
#[test]
fn local_data_report_child() {
    let Ok(mode) = std::env::var(CHILD_MODE) else {
        return;
    };
    match mode.as_str() {
        "plain" => std::process::exit(run(false)),
        "paths" => std::process::exit(run(true)),
        "local_app_data" => {
            println!(
                "{}",
                json!({"local_app_data": local_app_data().map(|path| path.display().to_string())})
            );
            std::process::exit(0);
        }
        other => panic!("unknown mode {other}"),
    }
}

/// The report command prints the report to stdout, pretty, and a note to
/// stderr; it reads the folder named by `BRIDGE_AGENT_DATA_DIR`, counts ages
/// from the present, names no path, and ends with a status that says whether
/// the report is complete.
#[test]
fn the_report_command_prints_the_report_and_a_note_and_ends_with_a_status() {
    let directory = tempfile::tempdir().unwrap();
    let data = directory.path().join("data");
    write(&data.join("agent-egress.jsonl"), b"abcd");
    fs::File::options()
        .write(true)
        .open(data.join("agent-egress.jsonl"))
        .unwrap()
        .set_modified(SystemTime::now() - Duration::from_secs(40 * 86_400 + 3_600))
        .unwrap();
    let output = run_child("plain", |command| {
        command.env("BRIDGE_AGENT_DATA_DIR", &data);
    });
    let (shown, body) = child_json(&output);
    assert!(
        body.contains("\n  \"folder\": \"present\""),
        "pretty: {body}"
    );
    assert_eq!(
        shown["classes"]["egress_log"],
        json!({"files": 1, "bytes": 4, "oldest_days": 40}),
        "{shown}"
    );
    assert!(shown.get("folder_path").is_none(), "{shown}");
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .lines()
            .any(|line| line == "Nothing was changed or deleted."),
        "{:?}",
        output.stderr
    );
    assert!(!String::from_utf8_lossy(&output.stdout).contains("Nothing was changed"));
    // 0 for a complete report; 3 when the user's own lease folder, which this
    // read also covers, is the incomplete part. Never 1: it was printed.
    assert!(
        matches!(output.status.code(), Some(0 | 3)),
        "{:?}",
        output.status
    );
}

/// A data folder that cannot be listed ends the command with status 2.
#[cfg(unix)]
#[test]
fn the_report_command_ends_with_two_for_a_data_folder_it_cannot_list() {
    let directory = tempfile::tempdir().unwrap();
    let data = directory.path().join("data");
    fs::write(&data, b"not a folder").unwrap();
    let output = run_child("plain", |command| {
        command.env("BRIDGE_AGENT_DATA_DIR", &data);
    });
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(child_json(&output).0["folder"], "unreadable");
}

/// With `--show-paths` the command names the data folder it was given and every
/// coordination folder, ahead of the default; without a data folder named, it
/// uses the platform's default under the user's profile.
#[test]
fn the_report_command_names_the_folders_it_read_only_when_asked() {
    let directory = tempfile::tempdir().unwrap();
    let data = directory.path().join("data");
    let profile = directory.path().join("profile");
    fs::create_dir_all(&data).unwrap();
    fs::create_dir_all(&profile).unwrap();
    let with_profile = |command: &mut std::process::Command| {
        command
            .env("HOME", &profile)
            .env("LOCALAPPDATA", &profile)
            .env("APPDATA", &profile);
    };
    let output = run_child("paths", |command| {
        with_profile(command);
        command.env("BRIDGE_AGENT_DATA_DIR", &data);
    });
    let (shown, _) = child_json(&output);
    assert_eq!(shown["folder_path"], data.display().to_string(), "{shown}");
    // Every folder the locks are taken under, as the default resolves them.
    let expected: Vec<String> = crate::local_files::paths::coordination_roots()
        .into_iter()
        .flatten()
        .map(|folder| folder.display().to_string())
        .collect();
    assert!(!expected.is_empty());
    assert_eq!(shown["lock_folder_paths"], json!(expected), "{shown}");
    assert_eq!(shown["lock_folder_path"], json!(expected[0]), "{shown}");
    // Package copies are looked for under %LOCALAPPDATA%, on Windows only.
    #[cfg(windows)]
    assert_eq!(
        shown["package_copies"], "not_under_local_app_data",
        "{shown}"
    );
    #[cfg(not(windows))]
    assert!(shown.get("package_copies").is_none(), "{shown}");
    // No data folder named: the platform's default, inside the profile.
    // (Where the default is under the temporary folder, that moves too.)
    let output = run_child("paths", |command| {
        with_profile(command);
        command
            .env("TMPDIR", &profile)
            .env("TMP", &profile)
            .env("TEMP", &profile)
            .env_remove("BRIDGE_AGENT_DATA_DIR");
    });
    let (shown, _) = child_json(&output);
    let default = PathBuf::from(shown["folder_path"].as_str().unwrap());
    assert!(default.starts_with(&profile), "{shown}");
}

/// On Windows `%LOCALAPPDATA%` is the variable of that name, as it stands: a
/// folder as given, nothing when it is not set, and not `%APPDATA%`.
#[cfg(windows)]
#[test]
fn on_windows_local_app_data_is_the_variable_of_that_name_and_no_other() {
    let directory = tempfile::tempdir().unwrap();
    let local = directory.path().join("Local");
    let roaming = directory.path().join("Roaming");
    let found = run_child("local_app_data", |command| {
        command.env("LOCALAPPDATA", &local).env("APPDATA", &roaming);
    });
    assert_eq!(
        child_json(&found).0["local_app_data"],
        local.display().to_string()
    );
    let unset = run_child("local_app_data", |command| {
        command.env_remove("LOCALAPPDATA").env("APPDATA", &roaming);
    });
    assert!(child_json(&unset).0["local_app_data"].is_null());
}

/// The file locations of the lock folders come in the order of the folders, the
/// order the locks are taken in.
#[test]
fn the_file_locations_of_the_lock_folders_come_in_the_order_the_locks_are_taken() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("data");
    let first = directory.path().join("first");
    let second = directory.path().join("second");
    write(&second.join("native-dispatch-leases/a.lock"), b"");
    let locks = [first.as_path(), second.as_path()];
    let report = build(&root, &locks);
    let resolve = |path: &Path| VisiblePath::AsIs(path.to_path_buf());
    let shown = to_json_resolving(
        &report,
        SystemTime::now(),
        Some((&root, &locks)),
        &resolve,
        None,
    );
    let leases = second.join("native-dispatch-leases");
    assert_eq!(
        shown["lock_folder_file_locations"],
        json!([
            "not_listed",
            {
                "folders": [{"path": leases.display().to_string(), "state": "as_is"}],
                "files_read": 1,
                "files": 1,
                "listing_capped": false,
            }
        ]),
        "{shown}"
    );
}
