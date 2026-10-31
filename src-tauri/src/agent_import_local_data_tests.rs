//! The local-data report reads and never writes (#local-data slice 1).
use super::*;
use crate::agent::agent_import::tests::{sentinel_batch_id, sentinel_uuid};

/// The company of the master journal tests: the merge gate's one registered synthetic GUID.
const COMPANY_GUID: &str = "00000000-0000-4000-8000-000000000002";

fn write(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

/// A minimal journal: one built batch, optionally sent.
fn journal(sent: bool) -> Vec<u8> {
    let line: super::super::ImportLedgerLine = serde_json::from_value(serde_json::json!({
        "batch_id":"b1","company_guid":"synthetic-guid","company":null,
        "txn_ids":["txn"],"date_from":"20260901","date_to":"20260901",
        "sha256":"a".repeat(64),"built_at":"2026-09-07T00:00:00Z","status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":1,"master_value":1},
        "vouchers":[{"bridge_txn_id":"txn","date":"20260901","voucher_type":"Journal",
            "narration":"n","reference":null,"voucher_number":null,"entries":[
                {"ledger":"Cash","amount":"1","side":"Dr"},
                {"ledger":"Synthetic Ledger","amount":"1","side":"Cr"}]}]
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
            settled_by_person: 0,
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
            settled_by_person: 0,
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
        settled_by_person: 0,
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
    let base = Path::new("/srv/Profile/AppData/Local");
    assert_eq!(
        relative_to(base, Path::new("/srv/profile/appdata/local/Bridge/agent")),
        Some(PathBuf::from("Bridge").join("agent"))
    );
    assert_eq!(
        relative_to(base, Path::new("/srv/Profile/AppData/LocalLow/Bridge")),
        None
    );
    assert_eq!(relative_to(base, Path::new("/srv/Profile")), None);
    assert_eq!(relative_to(base, base), Some(PathBuf::new()));
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

/// The report follows no link: a linked lease folder is not entered, and a
/// linked journal's target is not resolved.
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
        settled_by_person: 0,
        no_dispatch_never_verified: 0,
    });
    let journal = &to_json(&report, SystemTime::now(), None)["journal"];
    assert_eq!(journal["not_settled"], 4);
    assert_eq!(journal["not_settled_reviewed_not_attributed"], 1);
    assert_eq!(journal["not_settled_binding_refused"], 1);
    assert_eq!(journal["not_settled_not_verified"], 1);
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
        settled_by_person: 2,
        no_dispatch_never_verified: 0,
    });
    let journal = &to_json(&report, SystemTime::now(), None)["journal"];
    assert_eq!(journal["not_settled"], 3);
    assert_eq!(journal["settled_by_person"], 2);
    assert_eq!(journal["not_settled_not_verified"], 1);
}
