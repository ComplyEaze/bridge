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
        Journal::NotRead(JOURNAL_INVALID)
    );
    write(&path, &journal(true));
    assert_eq!(
        build(directory.path(), None).journal,
        Journal::Read(super::super::ledger::Settlement {
            batches: 1,
            sent_or_found: 1,
            unsettled: 1,
            unsettled_no_response: 1,
            no_dispatch_never_verified: 0
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
            no_dispatch_never_verified: 1
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
        build(directory.path(), None).journal,
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
    let report = build(root, None);
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
    let report = build(&not_a_folder, None);
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
    let mut report = build(directory.path(), None);
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
    let report = build(root, None);
    assert_eq!(report.special_files, 1);
    assert_eq!(report.links, 0);
    assert_eq!(*class(&report, "other"), Class::default());
}

/// The exit status says whether the report is complete.
#[test]
fn the_exit_status_is_zero_only_for_a_complete_report() {
    let directory = tempfile::tempdir().unwrap();
    let mut report = build(directory.path(), None);
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

/// A folder with more entries than the cap is listed up to the cap and reported
/// as capped, not as complete.
#[test]
fn a_folder_past_the_entry_cap_is_reported_as_capped() {
    let directory = tempfile::tempdir().unwrap();
    for name in ["a", "b", "c"] {
        write(&directory.path().join("lab").join(name), b"1");
    }
    let mut report = build(directory.path(), None);
    assert!(!report.entry_cap_reached);
    assert_eq!(class(&report, "lab").files, 3);
    report = build(&directory.path().join("absent"), None);
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
    let report = build(root, Some(root));
    assert_eq!(report.folders_not_read, ["lock_folder"]);
    assert_eq!(class(&report, "locks").files, 0, "the link is not followed");
    assert_eq!(report.incomplete_reason(), Some("folder_not_listed"));
}
