//! #1942: a failure code can reach the egress receipt, so each one should be a declared variant
//! (`agent_failure_code.rs`), never a string a site writes. Sites not yet converted are counted
//! here per production file: a snake_case literal made into a `String` (`.to_string()`,
//! `.to_owned()`, `.into()`, `String::from(..)`, `ToolFailure::from(..)`), or a code built by
//! `format!` with a name after its colon. Each file's pin is an upper bound that may only be
//! lowered; a converted site lowers its file's pin. Test modules (an inline `#[cfg(test)] mod name {`
//! through its closing brace, and `*tests.rs`) never count; code after such a module does.
//!
//! Not counted yet, so not held by this test: a code kept in a `const NAME: &str` and made into a
//! `String` where the name is used, a code chosen by a `match` whose result is made into a
//! `String` after the match, and a code built by `format!` with its name before an underscore
//! (`"{key}_invalid"`). The steps of #1942 that convert those sites remove them without a count;
//! the last step deletes `From<String>` for `ToolFailure` and the receipt's string checks.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// Sites per production file under `src`, relative to the crate directory: upper bounds, which
/// may only be lowered.
const PINNED: &[(&str, usize)] = &[
    ("src/agent.rs", 39),
    ("src/agent_audit_run.rs", 2),
    ("src/agent_bank_statement.rs", 46),
    ("src/agent_bill_trail.rs", 2),
    ("src/agent_cash_flow.rs", 1),
    ("src/agent_cash_projection.rs", 4),
    ("src/agent_catalog.rs", 10),
    ("src/agent_company.rs", 7),
    ("src/agent_company_checkpoint.rs", 6),
    ("src/agent_company_features.rs", 1),
    ("src/agent_delivery.rs", 3),
    ("src/agent_desktop_journal.rs", 10),
    ("src/agent_desktop_journal_review.rs", 5),
    ("src/agent_egress.rs", 26),
    ("src/agent_gst_uncertain.rs", 1),
    ("src/agent_import.rs", 129),
    ("src/agent_import_ack.rs", 23),
    ("src/agent_import_amend.rs", 10),
    ("src/agent_import_approval.rs", 12),
    ("src/agent_import_content_review.rs", 4),
    ("src/agent_import_invoice.rs", 21),
    ("src/agent_import_ledger.rs", 36),
    ("src/agent_import_numbering.rs", 1),
    ("src/agent_import_persistence.rs", 12),
    ("src/agent_import_post.rs", 56),
    ("src/agent_import_release.rs", 4),
    ("src/agent_import_settle.rs", 4),
    ("src/agent_import_statement_facts.rs", 2),
    ("src/agent_import_stop.rs", 8),
    ("src/agent_import_transfers.rs", 4),
    ("src/agent_import_verification.rs", 11),
    ("src/agent_lab.rs", 38),
    ("src/agent_lab_import.rs", 20),
    ("src/agent_ledger_candidates.rs", 5),
    ("src/agent_ledgers.rs", 12),
    ("src/agent_master_ledger.rs", 1),
    ("src/agent_master_tools.rs", 29),
    ("src/agent_master_verify.rs", 9),
    ("src/agent_masters.rs", 2),
    ("src/agent_movement.rs", 1),
    ("src/agent_negative_ledgers.rs", 1),
    ("src/agent_negative_stock.rs", 1),
    ("src/agent_outstandings.rs", 1),
    ("src/agent_party_bills.rs", 4),
    ("src/agent_presence.rs", 1),
    ("src/agent_protocol.rs", 15),
    ("src/agent_read_cost.rs", 2),
    ("src/agent_read_profiles.rs", 3),
    ("src/agent_read_validation.rs", 1),
    ("src/agent_responses.rs", 4),
    ("src/agent_statements.rs", 2),
    ("src/agent_stock_by_batch.rs", 2),
    ("src/agent_stock_by_godown.rs", 3),
    ("src/agent_stock_summary.rs", 1),
    ("src/agent_trial_balance.rs", 1),
    ("src/agent_voucher_goods_lines.rs", 1),
    ("src/agent_voucher_parse.rs", 33),
    ("src/agent_voucher_scalars.rs", 3),
    ("src/agent_voucher_search.rs", 3),
    ("src/agent_voucher_summary.rs", 19),
    ("src/agent_voucher_type_class.rs", 9),
    ("src/agent_voucher_window.rs", 12),
    ("src/agent_vouchers.rs", 14),
    ("src/commands.rs", 3),
    ("src/endpoint_coordination.rs", 3),
    ("src/sync/reconciliation.rs", 27),
    ("src/sync/snapshot.rs", 8),
    ("src/tally/approved_import.rs", 24),
    ("src/tally/connection.rs", 20),
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

/// Whether `line` opens an inline module: `mod name {`, with any visibility (`pub`, `pub(crate)`,
/// `pub(in path)`).
fn opens_inline_module(line: &str) -> bool {
    let line = line.trim();
    let line = match line.strip_prefix("pub") {
        None => line,
        Some(rest) => {
            let rest = rest.trim_start();
            match rest.strip_prefix('(') {
                Some(inner) => inner
                    .split_once(')')
                    .map_or("", |(_, after)| after.trim_start()),
                None => rest,
            }
        }
    };
    line.starts_with("mod ") && line.ends_with('{')
}

/// `source` without its inline test modules: each `#[cfg(test)]` at the start of a line, then
/// `mod name {`, through the closing `}` that stands alone at the start of a line (rustfmt puts a
/// top-level module's last brace there). Code after such a module still counts, as production
/// code can follow a test seam in the middle of a file. A module that is only declared
/// (`mod tests;`) holds no text here.
fn without_inline_test_modules(source: &str) -> String {
    let mut kept = String::with_capacity(source.len());
    let mut lines = source.split_inclusive('\n').peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("#[cfg(test)]") {
            let mut ahead = lines.clone();
            let module = ahead.find(|next| !next.trim_start().starts_with("#["));
            if module.is_some_and(opens_inline_module) {
                for skipped in lines.by_ref() {
                    if skipped.trim_end_matches(['\r', '\n']) == "}" {
                        break;
                    }
                }
                continue;
            }
        }
        kept.push_str(line);
    }
    kept
}

/// How many failure-code sites `source` holds as strings.
fn string_code_sites(source: &str) -> usize {
    let source = without_inline_test_modules(source);
    let source = source.as_str();
    crate::hand_entry_texts_tests::string_literal_spans(source)
        .into_iter()
        .filter(|(open, close, literal)| {
            let before = source[..*open].trim_end();
            let after = source[*close..].trim_start();
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

fn scan(
    directory: &Path,
    relative: &str,
    found: &mut BTreeMap<String, usize>,
    visited: &mut BTreeSet<String>,
) {
    let entries = fs::read_dir(directory).expect("a source directory is readable");
    for entry in entries {
        let entry = entry.expect("a source directory entry is readable");
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let relative_path = format!("{relative}/{name}");
        if path.is_dir() {
            if name != "tests" {
                scan(&path, &relative_path, found, visited);
            }
        } else if name.ends_with(".rs") && !name.ends_with("tests.rs") {
            let source = fs::read_to_string(&path).expect("a production source file is readable");
            let relative_path = relative_path.trim_start_matches('/').to_string();
            visited.insert(relative_path.clone());
            let sites = string_code_sites(&source);
            if sites > 0 {
                found.insert(relative_path, sites);
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
        string_code_sites("Err(\"some_code\"\n        .to_string())"),
        1,
        "a call wrapped onto the next line still counts"
    );
    // Shapes the module documentation lists as not counted yet: these pin that gap, so widening the
    // scan changes them on purpose.
    assert_eq!(
        string_code_sites(r#"const CODE: &str = "some_code"; ToolFailure::from(CODE.to_string())"#),
        0
    );
    assert_eq!(string_code_sites(r#"format!("{key}_invalid")"#), 0);
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
    let mut visited = BTreeSet::new();
    scan(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        "src",
        &mut found,
        &mut visited,
    );
    let pinned: BTreeMap<&str, usize> = PINNED.iter().copied().collect();
    let mut problems = Vec::new();
    // The floor: a scan that read nothing would find nothing. Every pinned file must have been
    // read (it may hold fewer sites than its pin, or none), and the walk must have read the tree.
    for file in pinned.keys() {
        if !visited.contains(*file) {
            problems.push(format!(
                "{file}: pinned but not read by the scan (moved, or the tree is unreadable)"
            ));
        }
    }
    if visited.len() < 100 {
        problems.push(format!(
            "the scan read only {} production files",
            visited.len()
        ));
    }
    for (file, sites) in &found {
        match pinned.get(file.as_str()) {
            None => problems.push(format!(
                "{file}: {sites} string failure code(s), none allowed"
            )),
            Some(pin) if sites > pin => problems.push(format!("{file}: {sites} > at most {pin}")),
            Some(_) => {}
        }
    }
    let table: String = found
        .iter()
        .map(|(file, sites)| format!("    (\"{file}\", {sites}),\n"))
        .collect();
    assert!(
        problems.is_empty(),
        "A failure code is declared in agent_failure_code.rs, never made from a string (#1942). \
         The pins are upper bounds and may only be lowered.\n{}\nPins as they stand now:\n{table}",
        problems.join("\n")
    );
}

#[test]
fn an_inline_test_module_of_any_visibility_is_skipped_and_the_code_after_it_is_not() {
    for head in [
        "mod seam {",
        "pub mod seam {",
        "pub(crate) mod seam {",
        "pub(in crate::a) mod seam {",
    ] {
        let source = format!(
            "fn a() {{ f(\"first_code\".to_string()); }}\n#[cfg(test)]\n{head}\n    fn b() {{ f(\"test_code\".to_string()); }}\n}}\nfn c() {{ f(\"last_code\".to_string()); }}\n"
        );
        assert_eq!(string_code_sites(&source), 2, "{head}");
    }
    // A module that is only declared, a name that merely starts with `pub`, and a module that is
    // not a test module are all kept.
    for kept in [
        "#[cfg(test)]\nmod tests;\nfn a() { f(\"some_code\".to_string()); }\n",
        "#[cfg(test)]\npubx mod seam {\nfn a() { f(\"some_code\".to_string()); }\n}\n",
        "pub(crate) mod seam {\nfn a() { f(\"some_code\".to_string()); }\n}\n",
    ] {
        assert_eq!(string_code_sites(kept), 1, "{kept:?}");
    }
}
