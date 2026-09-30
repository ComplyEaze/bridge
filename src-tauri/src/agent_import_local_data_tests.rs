//! The local-data report reads and never writes (#local-data slice 1).
use super::*;

fn write(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

/// A minimal journal: one built batch, optionally sent.
fn journal(sent: bool) -> Vec<u8> {
    let line: super::super::ImportLedgerLine = serde_json::from_value(serde_json::json!({
        "batch_id":"b1","company_guid":"synthetic-guid","company":null,
        "txn_ids":["txn"],"date_from":"20260901","date_to":"20260901",
        "sha256":"a".repeat(64),"built_at":"2026-09-07T00:00:00Z","status":"built",
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
    write(&root.join("agent-egress.jsonl"), b"abcd");
    write(&root.join("agent-import-admission.lock"), b"");
    write(&root.join("notes.txt"), b"12345");
    write(&root.join("imports/b.xml"), b"0123456789");
    write(&root.join("imports/b.proof.json"), b"12345");
    write(&root.join("imports/b.proof.md"), b"123456");
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
    let report = build(root, None);
    assert_eq!(report.root, Root::Present);
    let expect = |name: &str, files: u64, bytes: u64| {
        assert_eq!(*class(&report, name), Class { files, bytes }, "{name}");
    };
    expect("journal", 1, journal_bytes.len() as u64);
    expect("egress_log", 1, 4);
    expect("locks", 2, 0);
    expect("other", 2, 7);
    expect("import_files", 1, 10);
    expect("proofs", 2, 11);
    expect("approval_notes", 1, 7);
    expect("review_records", 3, 18);
    expect("bank_statements", 1, 11);
    expect("lab", 1, 3);
    assert_eq!(
        report.other_directories, 1,
        "an unknown folder is counted, not entered"
    );
    assert_eq!(report.interrupted_writes, 1, "the publication folder");
    assert_eq!(report.links + report.unreadable, 0);
}

#[test]
fn a_lock_folder_outside_the_data_folder_is_counted_too_and_not_twice() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("data");
    let coordination = directory.path().join("shared");
    write(&root.join("native-dispatch-leases/a.lock"), b"");
    write(&coordination.join("native-dispatch-leases/b.lock"), b"");
    let separate = build(&root, Some(&coordination));
    assert_eq!(class(&separate, "locks").files, 2, "both lock folders");
    // The default: the shared folder is the data folder itself.
    let same = build(&root, Some(&root));
    assert_eq!(class(&same, "locks").files, 1, "one folder, counted once");
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
    let report = build(root, None);
    assert_eq!(
        report.links, 2,
        "the symlinked file and the symlinked directory"
    );
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
    let report = build(&link, None);
    assert_eq!(report.root, Root::Present);
    assert_eq!(*class(&report, "egress_log"), Class { files: 1, bytes: 4 });
}

#[test]
fn a_missing_folder_is_not_an_empty_one_and_not_an_unreadable_one() {
    let directory = tempfile::tempdir().unwrap();
    assert_eq!(
        build(&directory.path().join("nope"), None).root,
        Root::Missing
    );
    let empty = build(directory.path(), None);
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
        let unreadable = build(&locked, None);
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
    let report = build(root, None);
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
    assert_eq!(build(directory.path(), None).journal, Journal::Absent);
    write(&path, b"not json\n");
    assert_eq!(
        build(directory.path(), None).journal,
        Journal::NotRead("journal_invalid_or_changing")
    );
    write(&path, &journal(true));
    assert_eq!(
        build(directory.path(), None).journal,
        Journal::Read(super::super::ledger::Settlement {
            batches: 1,
            sent_or_found: 1,
            unsettled: 1,
            unsettled_no_response: 1,
            never_sent: 0
        })
    );
    write(&path, &journal(false));
    assert_eq!(
        build(directory.path(), None).journal,
        Journal::Read(super::super::ledger::Settlement {
            batches: 1,
            sent_or_found: 0,
            unsettled: 0,
            unsettled_no_response: 0,
            never_sent: 1
        })
    );
}

#[test]
fn no_path_is_in_the_json_unless_asked() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    write(&root.join("imports/b.xml"), b"<x/>");
    let report = build(root, None);
    let plain = to_json(&report, SystemTime::now(), None).to_string();
    assert!(!plain.contains(&*root.to_string_lossy()), "{plain}");
    assert!(!plain.contains("path"), "{plain}");
    let shown = to_json(&report, SystemTime::now(), Some((root, Some(root)))).to_string();
    assert!(shown.contains(&*root.to_string_lossy()), "{shown}");
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
        writes_enabled: false,
        batch_post_enabled: false,
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
    assert!(!text.contains(&*root.to_string_lossy()), "no path: {text}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["folder"], "present");
    assert_eq!(result["classes"]["egress_log"]["files"], 1);
    assert_eq!(result["classes"]["egress_log"]["bytes"], 4);
    assert_eq!(result["classes"]["bank_statements"]["oldest_days"], 40);
    assert_eq!(result["journal"]["state"], "read");
    assert_eq!(result["journal"]["not_settled"], 1);
    assert_eq!(result["journal"]["not_settled_no_response"], 1);
    assert_eq!(result["journal"]["not_settled_not_verified"], 0);
    assert_eq!(result["journal"]["built_never_sent"], 0);
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
/// non-blocking exclusive lock). Checked on the source, so a later edit that
/// adds a lock call fails here.
#[test]
fn the_report_module_takes_no_lock() {
    let source = include_str!("agent_import_local_data.rs");
    for forbidden in [".try_lock", ".lock()", "lock_import_admission", "acquire("] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}
