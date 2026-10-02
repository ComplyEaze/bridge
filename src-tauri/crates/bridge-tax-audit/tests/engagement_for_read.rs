// SPDX-License-Identifier: Apache-2.0
//! An engagement built for a book the caller already holds: no `[snapshot]`, no directory, never
//! given to `load_book`, and the same dumps as the directory engagement on the same book.

mod common;

use bridge_tax_audit::error::AuditError;
use bridge_tax_audit::registry::{self, CallerData};
use bridge_tax_audit::{load_book, rules_for, Engagement};

fn synthetic_text() -> String {
    std::fs::read_to_string(common::fixtures().join("synthetic-engagement.toml")).unwrap()
}

/// The synthetic engagement text without its `[snapshot]` table.
fn without_snapshot(text: &str) -> String {
    let start = text
        .find("[snapshot]")
        .expect("the [snapshot] anchor moved");
    let end = start
        + text[start..]
            .find("[roles]")
            .expect("the [roles] anchor moved");
    format!("{}{}", &text[..start], &text[end..])
}

#[test]
fn an_engagement_for_a_held_book_has_no_directory_and_cannot_load_one() {
    let e = Engagement::from_toml_for_read(&without_snapshot(&synthetic_text())).unwrap();
    assert!(e.read_dir.as_os_str().is_empty());
    assert!(!e.allow_unbracketed_read);
    let err = load_book(&e).unwrap_err();
    assert!(
        matches!(
            err,
            AuditError::Refused {
                code: "CFG-no-directory",
                ..
            }
        ),
        "{err:?}"
    );
}

#[test]
fn a_snapshot_table_is_refused_for_a_held_book() {
    let err = Engagement::from_toml_for_read(&synthetic_text()).unwrap_err();
    assert!(
        matches!(
            err,
            AuditError::Refused {
                code: "CFG-snapshot-for-read",
                ..
            }
        ),
        "{err:?}"
    );
}

#[test]
fn a_legacy_creditor_source_is_refused_for_a_held_book_and_not_for_a_directory() {
    let text = synthetic_text().replace(
        "trade_creditors_source = { kind = \"groups\" }",
        "trade_creditors_source = { kind = \"legacy_json\", path = \"x.json\" }",
    );
    assert!(text.contains("legacy_json"), "the [roles] anchor moved");
    let err = Engagement::from_toml_for_read(&without_snapshot(&text)).unwrap_err();
    assert!(
        matches!(
            err,
            AuditError::Refused {
                code: "CFG-legacy-for-read",
                ..
            }
        ),
        "{err:?}"
    );
    assert!(Engagement::from_toml(&text, &common::fixtures()).is_ok());
}

#[test]
fn the_other_keys_are_refused_as_the_directory_engagement_refuses_them() {
    let bad = |text: String| text.replace("start = \"2025-04-01\"", "start = \"2025-4-1\"");
    assert!(
        bad(synthetic_text()).contains("2025-4-1"),
        "the [period] anchor moved"
    );
    let held =
        Engagement::from_toml_for_read(&without_snapshot(&bad(synthetic_text()))).unwrap_err();
    let dir = Engagement::from_toml(&bad(synthetic_text()), &common::fixtures()).unwrap_err();
    assert!(matches!(held, AuditError::Config(_)), "{held:?}");
    assert!(matches!(dir, AuditError::Config(_)), "{dir:?}");
}

#[test]
fn a_held_book_gives_the_same_dumps_as_the_directory_engagement() {
    let dir = common::engagement(&common::fixtures().join("synthetic-read"), false);
    let held = Engagement::from_toml_for_read(&without_snapshot(&synthetic_text())).unwrap();
    let book = load_book(&dir).unwrap();
    let rules = rules_for(&dir).unwrap();
    let mut compared = 0;
    for id in [
        "cash_44ab",
        "cash_payments_40a3",
        "stale_balances_41_1",
        "trial_balance",
        "party_monthly",
    ] {
        let test = registry::find(id).unwrap();
        let a = (test.run_on)(&dir, &book, &rules, &CallerData::default()).unwrap();
        let b = (test.run_on)(&held, &book, &rules, &CallerData::default()).unwrap();
        assert!(
            a["figures"].as_array().unwrap().len() >= test.min_figures,
            "{id}"
        );
        assert_eq!(a, b, "{id}");
        compared += 1;
    }
    assert_eq!(compared, 5);
}
