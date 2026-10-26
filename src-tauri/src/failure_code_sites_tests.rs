//! #1942: a failure code can reach the egress receipt, so each one should be a declared variant
//! (`agent_failure_code.rs`), never a string a site writes. Sites not yet converted are counted
//! here per production file: a snake_case literal made into a `String` (`.to_string()`,
//! `.to_owned()`, `.into()`, `String::from(..)`, `ToolFailure::from(..)`), or a code built by
//! `format!` with a name after its colon. The count may only fall; a converted site lowers its
//! file's pin. Test modules (`#[cfg(test)] mod name {` to the end of a file, and `*tests.rs`)
//! never count.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Sites per production file under `src`, relative to the crate directory. May only shrink.
const PINNED: &[(&str, usize)] = &[
    ("src/agent.rs", 38),
    ("src/agent_audit_run.rs", 2),
    ("src/agent_bank_statement.rs", 44),
    ("src/agent_bill_trail.rs", 2),
    ("src/agent_cash_flow.rs", 1),
    ("src/agent_catalog.rs", 10),
    ("src/agent_company.rs", 7),
    ("src/agent_company_checkpoint.rs", 6),
    ("src/agent_company_features.rs", 1),
    ("src/agent_delivery.rs", 3),
    ("src/agent_desktop_journal.rs", 10),
    ("src/agent_desktop_journal_review.rs", 5),
    ("src/agent_egress.rs", 21),
    ("src/agent_gst_status.rs", 8),
    ("src/agent_gst_uncertain.rs", 1),
    ("src/agent_import.rs", 128),
    ("src/agent_import_ack.rs", 23),
    ("src/agent_import_amend.rs", 10),
    ("src/agent_import_approval.rs", 12),
    ("src/agent_import_invoice.rs", 21),
    ("src/agent_import_ledger.rs", 30),
    ("src/agent_import_numbering.rs", 1),
    ("src/agent_import_persistence.rs", 15),
    ("src/agent_import_post.rs", 56),
    ("src/agent_import_statement_facts.rs", 2),
    ("src/agent_import_stop.rs", 8),
    ("src/agent_import_transfers.rs", 4),
    ("src/agent_import_verification.rs", 11),
    ("src/agent_lab.rs", 30),
    ("src/agent_lab_import.rs", 20),
    ("src/agent_ledger_candidates.rs", 5),
    ("src/agent_ledgers.rs", 10),
    ("src/agent_master_ledger.rs", 1),
    ("src/agent_masters.rs", 2),
    ("src/agent_movement.rs", 12),
    ("src/agent_negative_stock.rs", 1),
    ("src/agent_outstandings.rs", 5),
    ("src/agent_presence.rs", 8),
    ("src/agent_protocol.rs", 15),
    ("src/agent_read_cost.rs", 2),
    ("src/agent_read_profiles.rs", 3),
    ("src/agent_read_validation.rs", 1),
    ("src/agent_register.rs", 6),
    ("src/agent_responses.rs", 4),
    ("src/agent_statement_tie_out.rs", 16),
    ("src/agent_statements.rs", 1),
    ("src/agent_stock_summary.rs", 1),
    ("src/agent_trial_balance.rs", 1),
    ("src/agent_voucher_parse.rs", 31),
    ("src/agent_voucher_scalars.rs", 3),
    ("src/agent_voucher_search.rs", 3),
    ("src/agent_voucher_summary.rs", 15),
    ("src/agent_voucher_type_class.rs", 9),
    ("src/agent_voucher_window.rs", 12),
    ("src/agent_vouchers.rs", 13),
    ("src/commands.rs", 3),
    ("src/endpoint_coordination.rs", 3),
    ("src/sync/reconciliation.rs", 27),
    ("src/sync/snapshot.rs", 8),
    ("src/tally/approved_import.rs", 28),
    ("src/tally/connection.rs", 24),
    ("src/tally/connector.rs", 10),
    ("src/tally/runtime.rs", 4),
];

/// Whether `text` has the shape of a failure code: lower-case ASCII letters, digits and
/// underscores, starting with a letter and holding at least one underscore.
fn code_shaped(text: &str) -> bool {
    (3..=64).contains(&text.len())
        && text.contains('_')
        && text.starts_with(|c: char| c.is_ascii_lowercase())
        && text
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

/// How many failure-code sites `source` holds as strings.
fn string_code_sites(source: &str) -> usize {
    let source = crate::hand_entry_texts_tests::production_part(source);
    crate::hand_entry_texts_tests::string_literal_spans(source)
        .into_iter()
        .filter(|(open, close, literal)| {
            let before = source[..*open].trim_end();
            let after = &source[*close..];
            let made_into_string = [".to_string()", ".to_owned()", ".into()"]
                .iter()
                .any(|call| after.starts_with(call))
                || before.ends_with("String::from(")
                || before.ends_with("ToolFailure::from(");
            let formatted = before.ends_with("format!(")
                && literal
                    .split_once(":{")
                    .is_some_and(|(code, _)| code_shaped(code));
            (code_shaped(literal) && made_into_string) || formatted
        })
        .count()
}

fn scan(directory: &Path, relative: &str, found: &mut BTreeMap<String, usize>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let relative_path = format!("{relative}/{name}");
        if path.is_dir() {
            if name != "tests" {
                scan(&path, &relative_path, found);
            }
        } else if name.ends_with(".rs") && !name.ends_with("tests.rs") {
            let source = fs::read_to_string(&path).expect("a production source file is readable");
            let sites = string_code_sites(&source);
            if sites > 0 {
                found.insert(relative_path.trim_start_matches('/').to_string(), sites);
            }
        }
    }
}

#[test]
fn the_scan_counts_string_codes_and_nothing_else() {
    assert_eq!(string_code_sites(r#"Err("some_code".to_string())"#), 1);
    assert_eq!(string_code_sites(r#"x.push("some_code".into());"#), 1);
    assert_eq!(string_code_sites(r#"ToolFailure::from( "some_code" )"#), 1);
    assert_eq!(
        string_code_sites(r#"// "some_code".to_string()"#),
        0,
        "a comment never counts"
    );
    assert_eq!(
        string_code_sites(r#"let q = '"'; Err("some_code".to_string())"#),
        1,
        "a quote in a char literal opens no string"
    );
    assert_eq!(
        string_code_sites(r#"let a = "say \"x\"".len(); Err("some_code".into())"#),
        1,
        "an escaped quote does not close a string"
    );
    assert_eq!(
        string_code_sites(r#"ToolFailure::from("some_code".to_string())"#),
        1
    );
    assert_eq!(string_code_sites(r#"String::from("some_code")"#), 1);
    assert_eq!(
        string_code_sites(r#"format!("argument_invalid:{name}")"#),
        1
    );
    assert_eq!(string_code_sites(r#"format!("{} rows", n)"#), 0);
    assert_eq!(string_code_sites(r#"let a = "Some words".to_string();"#), 0);
    assert_eq!(string_code_sites(r#"let a = "nocode".to_string();"#), 0);
    assert_eq!(string_code_sites(r#"json!({"key_name": 1})"#), 0);
    assert_eq!(
        string_code_sites(
            "fn f() {}\n#[cfg(test)]\nmod tests {\n    fn g() { \"some_code\".to_string(); }\n}"
        ),
        0,
        "a test module never counts"
    );
}

#[test]
fn string_failure_codes_only_fall() {
    let mut found = BTreeMap::new();
    scan(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        "src",
        &mut found,
    );
    assert!(found.len() > 10, "the scan read almost nothing: {found:?}");
    let pinned: BTreeMap<String, usize> = PINNED
        .iter()
        .map(|(file, count)| ((*file).to_string(), *count))
        .collect();
    assert_eq!(
        found, pinned,
        "a file's count of string failure codes changed: lower its pin when a site is typed; a new \
         string code is declared in agent_failure_code.rs instead"
    );
}
