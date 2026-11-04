// SPDX-License-Identifier: Apache-2.0
//! The scorer of `tests/accuracy`, on invented keys and findings. Nothing here measures the
//! engine: the tests over committed goldens check only that the scorer reads the canonical dump's
//! shape, repeated refs included.

mod accuracy;

use accuracy::{score, FindingScore, Item, KeyError, Kind, Miss, ScoreError, SealedKey};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const CREDITOR: &str = "stale_balances_41_1/s41_1/creditor";
const DEBTOR: &str = "stale_balances_41_1/recoverability/debtor";

/// The sha256 of `text`, hashed here rather than by the scorer, so a seal check that hashed
/// nothing could not pass.
fn sealed(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn open(text: &str) -> Result<SealedKey, KeyError> {
    SealedKey::open(text.as_bytes(), &sealed(text))
}

fn item(kind: Kind, id: &str, label: &str) -> Item {
    Item {
        kind,
        id: id.to_string(),
        label: label.to_string(),
    }
}

fn ledger(id: &str) -> Item {
    item(Kind::Ledger, id, "")
}

const KEY: &str = r#"
test_id = "stale_balances_41_1"

[[finding]]
id = "stale_balances_41_1/s41_1/creditor"

[[finding.item]]
kind = "ledger"
id = "Invented Creditor A"
reason = "opening equals closing, nothing posted"

[[finding.item]]
kind = "ledger"
id = "Invented Creditor B"
reason = "a credit balance untouched all year"

[[finding]]
id = "stale_balances_41_1/recoverability/debtor"

[[finding.item]]
kind = "ledger"
id = "Invented Debtor C"
reason = "a debit balance untouched all year"
"#;

fn finding(id: &str, ledgers: &[&str]) -> Value {
    json!({
        "id": id,
        "evidence": ledgers
            .iter()
            .map(|l| json!({"kind": "ledger", "id": l, "label": ""}))
            .collect::<Vec<_>>(),
    })
}

fn dump(findings: Vec<Value>) -> Value {
    json!({"test_id": "stale_balances_41_1", "findings": findings})
}

#[test]
fn a_score_counts_matches_misses_with_reasons_and_extras() {
    let key = open(KEY).unwrap();
    let got = score(
        &key,
        &dump(vec![
            finding(CREDITOR, &["Invented Creditor A", "Invented Creditor X"]),
            finding(DEBTOR, &["Invented Debtor C"]),
        ]),
    )
    .unwrap();
    assert_eq!(got.test_id, "stale_balances_41_1");
    // In finding id order: the debtor id sorts before the creditor id.
    assert_eq!(
        got.findings,
        vec![
            FindingScore {
                finding_id: DEBTOR.to_string(),
                expected: 1,
                listed: 1,
                matched: 1,
                missed: vec![],
                extra: vec![],
            },
            FindingScore {
                finding_id: CREDITOR.to_string(),
                expected: 2,
                listed: 2,
                matched: 1,
                missed: vec![Miss {
                    item: ledger("Invented Creditor B"),
                    reason: "a credit balance untouched all year".to_string(),
                }],
                extra: vec![ledger("Invented Creditor X")],
            },
        ]
    );
}

#[test]
fn a_finding_the_dump_lacks_lists_nothing() {
    let key = open(KEY).unwrap();
    let got = score(
        &key,
        &dump(vec![finding(CREDITOR, &["Invented Creditor A"])]),
    )
    .unwrap();
    let debtor = &got.findings[0];
    assert_eq!(debtor.finding_id, DEBTOR);
    assert_eq!((debtor.expected, debtor.listed, debtor.matched), (1, 0, 0));
    assert_eq!(debtor.missed[0].item, ledger("Invented Debtor C"));
}

#[test]
fn a_finding_expected_empty_counts_what_is_listed_as_extra() {
    let key = open(
        r#"
test_id = "stale_balances_41_1"
[[finding]]
id = "stale_balances_41_1/s41_1/creditor"
[[finding.item]]
kind = "ledger"
id = "Invented Creditor A"
reason = "untouched"
[[finding]]
id = "stale_balances_41_1/recoverability/debtor"
"#,
    )
    .unwrap();
    let got = score(
        &key,
        &dump(vec![
            finding(CREDITOR, &["Invented Creditor A"]),
            finding(DEBTOR, &["Invented Debtor Y"]),
        ]),
    )
    .unwrap();
    assert_eq!(got.findings[0].finding_id, DEBTOR);
    assert_eq!((got.findings[0].expected, got.findings[0].matched), (0, 0));
    assert_eq!(got.findings[0].extra, vec![ledger("Invented Debtor Y")]);
}

#[test]
fn an_item_matches_only_with_its_kind_and_label() {
    let key = open(KEY).unwrap();
    let mut d = dump(vec![finding(DEBTOR, &[])]);
    d["findings"][0]["evidence"] = json!([
        {"kind": "voucher", "id": "Invented Debtor C", "label": ""},
        {"kind": "ledger", "id": "Invented Debtor C", "label": "a label the key does not give"},
    ]);
    let got = score(&key, &d).unwrap();
    assert_eq!(got.findings[0].matched, 0);
    assert_eq!(
        got.findings[0].extra,
        vec![
            item(
                Kind::Ledger,
                "Invented Debtor C",
                "a label the key does not give"
            ),
            item(Kind::Voucher, "Invented Debtor C", ""),
        ]
    );
}

#[test]
fn a_name_is_compared_exactly_never_trimmed() {
    let key = open(KEY).unwrap();
    let got = score(&key, &dump(vec![finding(DEBTOR, &["Invented Debtor C "])])).unwrap();
    assert_eq!(got.findings[0].matched, 0);
    assert_eq!(got.findings[0].extra, vec![ledger("Invented Debtor C ")]);
}

/// A dump may list one item twice (the canonical dump keeps every ref). A key naming it once
/// matches one copy, and the other is extra; a key naming it twice matches both.
#[test]
fn a_repeated_item_is_counted_with_its_repeats() {
    let d = dump(vec![finding(
        DEBTOR,
        &["Invented Debtor C", "Invented Debtor C"],
    )]);
    let once = score(&open(KEY).unwrap(), &d).unwrap();
    assert_eq!(
        (
            once.findings[0].expected,
            once.findings[0].listed,
            once.findings[0].matched
        ),
        (1, 2, 1)
    );
    assert_eq!(once.findings[0].extra, vec![ledger("Invented Debtor C")]);

    let twice = KEY.to_string()
        + "[[finding.item]]\nkind = \"ledger\"\nid = \"Invented Debtor C\"\nreason = \"a second voucher\"\n";
    let both = score(&open(&twice).unwrap(), &d).unwrap();
    assert_eq!(
        (
            both.findings[0].expected,
            both.findings[0].listed,
            both.findings[0].matched
        ),
        (2, 2, 2)
    );
    assert!(both.findings[0].extra.is_empty());

    let missed = score(
        &open(&twice).unwrap(),
        &dump(vec![finding(DEBTOR, &["Invented Debtor C"])]),
    )
    .unwrap();
    assert_eq!(
        missed.findings[0].missed,
        vec![Miss {
            item: ledger("Invented Debtor C"),
            reason: "a second voucher".to_string(),
        }]
    );
}

#[test]
fn nothing_expected_and_nothing_listed_is_refused() {
    let key = open(
        r#"
test_id = "stale_balances_41_1"
[[finding]]
id = "stale_balances_41_1/s41_1/creditor"
[[finding]]
id = "stale_balances_41_1/recoverability/debtor"
"#,
    )
    .unwrap();
    assert_eq!(score(&key, &dump(vec![])), Err(ScoreError::Empty));
}

#[test]
fn a_dump_refused_before_any_count() {
    let key = open(KEY).unwrap();
    let mut other = dump(vec![]);
    other["test_id"] = json!("cash_44ab");
    assert_eq!(
        score(&key, &other),
        Err(ScoreError::TestIdDiffers {
            key: "stale_balances_41_1".to_string(),
            dump: "cash_44ab".to_string(),
        })
    );
    assert_eq!(
        score(
            &key,
            &dump(vec![finding("stale_balances_41_1/other", &["X"])])
        ),
        Err(ScoreError::DumpFindingNotInKey(
            "stale_balances_41_1/other".to_string()
        ))
    );
    assert!(matches!(
        score(
            &key,
            &dump(vec![finding(DEBTOR, &[]), finding(DEBTOR, &[])])
        ),
        Err(ScoreError::DumpInvalid(_))
    ));
    assert!(matches!(
        score(&key, &json!({"test_id": "stale_balances_41_1"})),
        Err(ScoreError::DumpInvalid(_))
    ));
    for evidence in [
        json!([{"kind": "ledger", "label": ""}]),
        json!([{"kind": "ledger", "id": "A"}]),
        json!([{"kind": "Ledger", "id": "A", "label": ""}]),
    ] {
        let mut d = dump(vec![finding(DEBTOR, &[])]);
        d["findings"][0]["evidence"] = evidence;
        assert!(matches!(score(&key, &d), Err(ScoreError::DumpInvalid(_))));
    }
}

#[test]
fn a_key_opens_only_against_its_own_seal() {
    let edited = KEY.replace("Invented Creditor B", "Invented Creditor Q");
    assert!(matches!(
        SealedKey::open(edited.as_bytes(), &sealed(KEY)),
        Err(KeyError::SealMismatch { .. })
    ));
    assert!(matches!(
        SealedKey::open(KEY.as_bytes(), &sealed(KEY).to_uppercase()),
        Err(KeyError::SealMismatch { .. })
    ));
    let not_utf8 = [0xff_u8, 0xfe];
    let seal: String = Sha256::digest(not_utf8)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(
        SealedKey::open(&not_utf8, &seal).unwrap_err(),
        KeyError::NotUtf8
    );
}

#[test]
fn a_key_that_is_not_the_format_does_not_open() {
    let parse = |text: &str| matches!(open(text), Err(KeyError::Parse(_)));
    assert!(parse(
        "test_id = \"t\"\nextra = 1\n[[finding]]\nid = \"f\"\n"
    ));
    assert!(parse(
        "test_id = \"t\"\n[[finding]]\nid = \"f\"\n[[finding.item]]\nkind = \"ledger\"\nid = \"A\"\n"
    ));
    // A kind outside the crate's list, a misspelling or another case, does not open.
    for kind in ["Ledger", "ledgr", "ledger:x", ""] {
        assert!(
            parse(&format!(
                "test_id = \"t\"\n[[finding]]\nid = \"f\"\n[[finding.item]]\nkind = \"{kind}\"\nid = \"A\"\nreason = \"r\"\n"
            )),
            "{kind}"
        );
    }
    assert_eq!(open("test_id = \"t\"\n").unwrap_err(), KeyError::NoFindings);
    assert_eq!(
        open("test_id = \" \"\n[[finding]]\nid = \"f\"\n").unwrap_err(),
        KeyError::Blank("test_id")
    );
    assert_eq!(
        open("test_id = \"t\"\n[[finding]]\nid = \"f\"\n[[finding.item]]\nkind = \"ledger\"\nid = \"A\"\nreason = \"\"\n")
            .unwrap_err(),
        KeyError::Blank("item.reason")
    );
    assert_eq!(
        open("test_id = \"t\"\n[[finding]]\nid = \"f\"\n[[finding.item]]\nkind = \"voucher\"\nid = \"\"\nreason = \"r\"\n")
            .unwrap_err(),
        KeyError::Blank("item.id and item.label")
    );
    assert_eq!(
        open("test_id = \"t\"\n[[finding]]\nid = \"f\"\n[[finding]]\nid = \"f\"\n").unwrap_err(),
        KeyError::DuplicateFinding("f".to_string())
    );
}

fn golden(name: &str) -> Value {
    let path = format!(
        "{}/tests/fixtures/golden/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

/// A key naming each finding id of `dump`, with `items` (TOML `[[finding.item]]` tables) under
/// `with_items` only.
fn key_for(dump: &Value, with_items: &str, items: &str) -> String {
    let mut key = format!("test_id = \"{}\"\n", dump["test_id"].as_str().unwrap());
    for f in dump["findings"].as_array().unwrap() {
        let id = f["id"].as_str().unwrap();
        key += &format!("[[finding]]\nid = \"{id}\"\n");
        if id == with_items {
            key += items;
        }
    }
    key
}

/// The committed synthetic golden is the canonical dump's own shape: a key naming its two listed
/// ledgers matches both, so the scorer reads `findings[].evidence[]` where the dump puts them.
#[test]
fn the_scorer_reads_the_canonical_dump_shape() {
    let golden = golden("synthetic.stale_balances_41_1.json");
    let key = open(
        r#"
test_id = "stale_balances_41_1"
[[finding]]
id = "stale_balances_41_1/s41_1/creditor"
[[finding.item]]
kind = "ledger"
id = "Harbour Mill Supplies"
reason = "shape check only"
[[finding]]
id = "stale_balances_41_1/recoverability/debtor"
[[finding.item]]
kind = "ledger"
id = "Quarry Lane Stores"
reason = "shape check only"
"#,
    )
    .unwrap();
    let got = score(&key, &golden).unwrap();
    for f in &got.findings {
        assert_eq!(
            (f.expected, f.listed, f.matched),
            (1, 1, 1),
            "{}",
            f.finding_id
        );
        assert!(f.missed.is_empty() && f.extra.is_empty());
    }
    assert_eq!(got.findings.len(), 2);
}

/// A golden whose finding lists two vouchers with one kind and an empty id, told apart only by
/// label: a key names each by its label, and both match.
#[test]
fn refs_sharing_a_kind_and_an_empty_id_are_told_apart_by_label() {
    let golden = golden("edge.partners_shared_guid_tds.partners_40b_194t.json");
    let finding = "partners_40b_194t/off_capital/793d6d7c";
    let key = key_for(
        &golden,
        finding,
        "[[finding.item]]\nkind = \"voucher\"\nid = \"\"\nlabel = \"Payment 9 on 2025-08-20\"\nreason = \"shape check only\"\n\
[[finding.item]]\nkind = \"voucher\"\nid = \"\"\nlabel = \"Payment 10 on 2025-11-20\"\nreason = \"shape check only\"\n",
    );
    let got = score(&open(&key).unwrap(), &golden).unwrap();
    let f = got
        .findings
        .iter()
        .find(|f| f.finding_id == finding)
        .unwrap();
    assert_eq!((f.expected, f.listed, f.matched), (2, 2, 2));
    assert!(f.missed.is_empty() && f.extra.is_empty());
    assert_eq!(got.findings.len(), 9);
}

/// A golden whose finding lists one ref twice, kind, id and label alike. A key naming it twice
/// matches both copies, and its label written decomposed matches the dump's NFC form.
#[test]
fn a_ref_listed_twice_in_a_golden_matches_a_key_naming_it_twice() {
    let golden = golden("edge.creditor_ageing_shared_guid.creditor_ageing_43bh.json");
    let finding = "creditor_ageing_43bh/unknown_classification/729e41a8";
    let named = "[[finding.item]]\nkind = \"voucher\"\nid = \"edge-cg-uni\"\nlabel = \"Purchase e\u{301}1 on 2025-11-02\"\nreason = \"shape check only\"\n";
    let got = score(
        &open(&key_for(&golden, finding, &named.repeat(2))).unwrap(),
        &golden,
    )
    .unwrap();
    let f = got
        .findings
        .iter()
        .find(|f| f.finding_id == finding)
        .unwrap();
    assert_eq!((f.expected, f.listed, f.matched), (2, 8, 2));
    assert!(f.missed.is_empty());
    assert_eq!(f.extra.len(), 6);
    assert!(!f
        .extra
        .iter()
        .any(|i| i.label == "Purchase \u{e9}1 on 2025-11-02"));
}
