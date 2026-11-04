// SPDX-License-Identifier: Apache-2.0
//! The scorer of `tests/accuracy`, on invented keys and findings over the committed synthetic
//! read's Book. Nothing here measures the engine: the tests over committed goldens check only that
//! the scorer reads the canonical dump's shape and that every finding they hold has a family.

mod accuracy;
mod common;

use std::collections::{BTreeMap, BTreeSet};

use accuracy::{
    family_of, score, FamilyScore, Item, KeyError, Kind, Miss, ScoreError, SealedKey, Unresolved,
    FAMILIES,
};
use bridge_tax_audit::book::Book;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const CREDITOR: &str = "stale_balances_41_1/s41_1/creditor";
const DEBTOR: &str = "stale_balances_41_1/recoverability/debtor";
const S41: &str = "stale_balances_41_1/s41_1/";
const RECOVERABILITY: &str = "stale_balances_41_1/recoverability/";
const LAG: &str = "book_keeping_quality/entry_order/lag_over_30/";
/// Two vouchers of the synthetic read, by GUID, as `synthetic.book_keeping_quality.json` cites them.
const JOURNAL_J2: &str = "6f1c2a3e-8b4d-4c5e-9a7f-0d1e2f3a4b5c-00000030";
const PAYMENT_P8: &str = "6f1c2a3e-8b4d-4c5e-9a7f-0d1e2f3a4b5c-00000013";

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

/// The Book of the committed synthetic read, which the synthetic goldens were run on.
fn book() -> Book {
    bridge_tax_audit::load_book(&common::engagement(
        &common::fixtures().join("synthetic-read"),
        false,
    ))
    .unwrap()
}

fn ledger(name: &str) -> Item {
    Item::Ledger {
        name: name.to_string(),
    }
}

fn voucher(voucher_type: &str, number: &str, date: &str) -> Item {
    Item::Voucher {
        voucher_type: voucher_type.to_string(),
        number: number.to_string(),
        date: date.to_string(),
    }
}

const KEY: &str = r#"
test_id = "stale_balances_41_1"

[[family]]
prefix = "stale_balances_41_1/s41_1/"

[[family.item]]
kind = "ledger"
name = "Invented Creditor A"
reason = "opening equals closing, nothing posted"

[[family.item]]
kind = "ledger"
name = "Invented Creditor B"
reason = "a credit balance untouched all year"

[[family]]
prefix = "stale_balances_41_1/recoverability/"

[[family.item]]
kind = "ledger"
name = "Invented Debtor C"
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

fn family<'a>(score: &'a accuracy::Score, name: &str) -> &'a FamilyScore {
    score.families.iter().find(|f| f.family == name).unwrap()
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
        &Book::default(),
    )
    .unwrap();
    assert_eq!(got.test_id, "stale_balances_41_1");
    // In family order: recoverability sorts before s41_1.
    assert_eq!(
        got.families,
        vec![
            FamilyScore {
                family: RECOVERABILITY.to_string(),
                expected: 1,
                listed: 1,
                matched: 1,
                missed: vec![],
                extra: vec![],
                unresolved: vec![],
                unscored: BTreeMap::new(),
            },
            FamilyScore {
                family: S41.to_string(),
                expected: 2,
                listed: 2,
                matched: 1,
                missed: vec![Miss {
                    item: ledger("Invented Creditor B"),
                    reason: "a credit balance untouched all year".to_string(),
                }],
                extra: vec![ledger("Invented Creditor X")],
                unresolved: vec![],
                unscored: BTreeMap::new(),
            },
        ]
    );
}

/// A family's items are the union of all its findings' items: a key naming one ledger listed by
/// each of two findings of one family matches both.
#[test]
fn a_family_pools_the_items_of_all_its_findings() {
    let key = open(KEY).unwrap();
    let got = score(
        &key,
        &dump(vec![
            finding(CREDITOR, &["Invented Creditor A"]),
            finding("stale_balances_41_1/s41_1/debtor", &["Invented Creditor B"]),
        ]),
        &Book::default(),
    )
    .unwrap();
    let s41 = family(&got, S41);
    assert_eq!((s41.expected, s41.listed, s41.matched), (2, 2, 2));
    assert!(s41.missed.is_empty() && s41.extra.is_empty());
}

#[test]
fn a_family_the_dump_lacks_lists_nothing() {
    let key = open(KEY).unwrap();
    let got = score(
        &key,
        &dump(vec![finding(CREDITOR, &["Invented Creditor A"])]),
        &Book::default(),
    )
    .unwrap();
    let debtor = family(&got, RECOVERABILITY);
    assert_eq!((debtor.expected, debtor.listed, debtor.matched), (1, 0, 0));
    assert_eq!(debtor.missed[0].item, ledger("Invented Debtor C"));
}

#[test]
fn a_family_expected_empty_counts_what_is_listed_as_extra() {
    let key = open(
        r#"
test_id = "stale_balances_41_1"
[[family]]
prefix = "stale_balances_41_1/s41_1/"
[[family.item]]
kind = "ledger"
name = "Invented Creditor A"
reason = "untouched"
[[family]]
prefix = "stale_balances_41_1/recoverability/"
"#,
    )
    .unwrap();
    let got = score(
        &key,
        &dump(vec![
            finding(CREDITOR, &["Invented Creditor A"]),
            finding(DEBTOR, &["Invented Debtor Y"]),
        ]),
        &Book::default(),
    )
    .unwrap();
    let debtor = family(&got, RECOVERABILITY);
    assert_eq!((debtor.expected, debtor.matched), (0, 0));
    assert_eq!(debtor.extra, vec![ledger("Invented Debtor Y")]);
}

#[test]
fn a_name_is_compared_exactly_never_trimmed() {
    let key = open(KEY).unwrap();
    let got = score(
        &key,
        &dump(vec![finding(DEBTOR, &["Invented Debtor C "])]),
        &Book::default(),
    )
    .unwrap();
    let debtor = family(&got, RECOVERABILITY);
    assert_eq!(debtor.matched, 0);
    assert_eq!(debtor.extra, vec![ledger("Invented Debtor C ")]);
}

/// A dump may list one item twice (the canonical dump keeps every ref). A key naming it once
/// matches one copy, and the other is extra; a key naming it twice matches both.
#[test]
fn a_repeated_item_is_counted_with_its_repeats() {
    let d = dump(vec![finding(
        DEBTOR,
        &["Invented Debtor C", "Invented Debtor C"],
    )]);
    let once = score(&open(KEY).unwrap(), &d, &Book::default()).unwrap();
    let f = family(&once, RECOVERABILITY);
    assert_eq!((f.expected, f.listed, f.matched), (1, 2, 1));
    assert_eq!(f.extra, vec![ledger("Invented Debtor C")]);

    let twice = KEY.to_string()
        + "[[family.item]]\nkind = \"ledger\"\nname = \"Invented Debtor C\"\nreason = \"a second voucher\"\n";
    let both = score(&open(&twice).unwrap(), &d, &Book::default()).unwrap();
    let f = family(&both, RECOVERABILITY);
    assert_eq!((f.expected, f.listed, f.matched), (2, 2, 2));
    assert!(f.extra.is_empty());

    let missed = score(
        &open(&twice).unwrap(),
        &dump(vec![finding(DEBTOR, &["Invented Debtor C"])]),
        &Book::default(),
    )
    .unwrap();
    assert_eq!(
        family(&missed, RECOVERABILITY).missed,
        vec![Miss {
            item: ledger("Invented Debtor C"),
            reason: "a second voucher".to_string(),
        }]
    );
}

fn lag_key(items: &str) -> String {
    format!("test_id = \"book_keeping_quality\"\n[[family]]\nprefix = \"{LAG}\"\n{items}")
}

fn voucher_item(voucher_type: &str, number: &str, date: &str) -> String {
    format!(
        "[[family.item]]\nkind = \"voucher\"\nvoucher_type = \"{voucher_type}\"\nnumber = \"{number}\"\ndate = \"{date}\"\nreason = \"invented\"\n"
    )
}

fn lag_dump(refs: Value) -> Value {
    json!({
        "test_id": "book_keeping_quality",
        "findings": [{"id": format!("{LAG}journal"), "evidence": refs}],
    })
}

/// A voucher ref is resolved by its GUID through the Book, and its label is never read: a ref
/// whose label is invented prose still matches the key's type, number and date. A key that writes
/// the number differently does not match.
#[test]
fn a_voucher_is_matched_by_type_number_and_date_through_its_guid() {
    let book = book();
    let d = lag_dump(json!([
        {"kind": "voucher", "id": JOURNAL_J2, "label": "prose the engine wrote, not the book"},
    ]));
    let got = score(
        &open(&lag_key(&voucher_item("Journal", "J/2", "2025-06-01"))).unwrap(),
        &d,
        &book,
    )
    .unwrap();
    let f = family(&got, LAG);
    assert_eq!((f.expected, f.listed, f.matched), (1, 1, 1));
    assert!(f.missed.is_empty() && f.extra.is_empty() && f.unresolved.is_empty());

    let other = score(
        &open(&lag_key(&voucher_item("Journal", "J/02", "2025-06-01"))).unwrap(),
        &d,
        &book,
    )
    .unwrap();
    let f = family(&other, LAG);
    assert_eq!(f.matched, 0);
    assert_eq!(f.extra, vec![voucher("Journal", "J/2", "2025-06-01")]);
}

/// A voucher ref with an empty GUID is counted as unresolved, never matched from its label, even
/// when the label spells the key's voucher exactly.
#[test]
fn a_voucher_with_an_empty_guid_is_unresolved_never_matched() {
    let got = score(
        &open(&lag_key(&voucher_item("Journal", "J/2", "2025-06-01"))).unwrap(),
        &lag_dump(json!([
            {"kind": "voucher", "id": "", "label": "Journal J/2 on 2025-06-01"},
        ])),
        &book(),
    )
    .unwrap();
    let f = family(&got, LAG);
    assert_eq!((f.expected, f.listed, f.matched), (1, 0, 0));
    assert_eq!(f.unresolved, vec![Unresolved::EmptyGuid]);
    assert_eq!(f.missed.len(), 1);
    assert!(f.extra.is_empty());
}

/// A GUID the Book gives two vouchers that differ is unresolved; one the Book does not hold means
/// the dump and the Book are not of one run, and refuses the score.
#[test]
fn a_guid_the_book_cannot_resolve_is_never_guessed() {
    let mut book = book();
    let mut twin = book
        .vouchers
        .iter()
        .find(|v| v.guid == JOURNAL_J2)
        .unwrap()
        .clone();
    twin.number = "J/3".to_string();
    book.vouchers.push(twin);
    let key = open(&lag_key(&voucher_item("Journal", "J/2", "2025-06-01"))).unwrap();
    let refs = json!([{"kind": "voucher", "id": JOURNAL_J2, "label": "Journal J/2 on 2025-06-01"}]);
    let got = score(&key, &lag_dump(refs), &book).unwrap();
    let f = family(&got, LAG);
    assert_eq!(f.matched, 0);
    assert_eq!(
        f.unresolved,
        vec![Unresolved::SharedGuid(JOURNAL_J2.to_string())]
    );

    let absent = "6f1c2a3e-8b4d-4c5e-9a7f-0d1e2f3a4b5c-ffffffff";
    assert_eq!(
        score(
            &key,
            &lag_dump(json!([{"kind": "voucher", "id": absent, "label": ""}])),
            &book
        ),
        Err(ScoreError::VoucherNotInBook(absent.to_string()))
    );
}

/// Only `ledger` and `voucher` may be named in a key. A dump's refs of other kinds are counted per
/// kind, as not scored, never as extra.
#[test]
fn a_kind_not_ruled_on_is_refused_in_a_key() {
    for (word, kind) in [
        ("config", Kind::Config),
        ("document_row", Kind::DocumentRow),
        ("excluded_voucher", Kind::ExcludedVoucher),
        ("pan", Kind::Pan),
        ("payee_name", Kind::PayeeName),
        ("row", Kind::Row),
        ("rule", Kind::Rule),
        ("stock_item", Kind::StockItem),
    ] {
        assert_eq!(
            open(&lag_key(&format!(
                "[[family.item]]\nkind = \"{word}\"\nname = \"A\"\nreason = \"r\"\n"
            )))
            .unwrap_err(),
            KeyError::KindRefused(kind),
            "{word}"
        );
    }
    let got = score(
        &open(&lag_key(&voucher_item("Payment", "P/8", "2025-06-10"))).unwrap(),
        &lag_dump(json!([
            {"kind": "voucher", "id": PAYMENT_P8, "label": ""},
            {"kind": "row", "id": "r1", "label": "a row"},
            {"kind": "row", "id": "r2", "label": "a row"},
        ])),
        &book(),
    )
    .unwrap();
    let f = family(&got, LAG);
    assert_eq!((f.listed, f.matched), (1, 1));
    assert!(f.extra.is_empty());
    assert_eq!(f.unscored, BTreeMap::from([(Kind::Row, 2)]));
}

#[test]
fn a_finding_in_no_family_is_refused() {
    let key = open(KEY).unwrap();
    assert_eq!(
        score(
            &key,
            &dump(vec![finding("stale_balances_41_1/other", &["X"])]),
            &Book::default()
        ),
        Err(ScoreError::FindingInNoFamily(
            "stale_balances_41_1/other".to_string()
        ))
    );
    assert_eq!(family_of("stale_balances_41_1/s41_1"), None);
    assert_eq!(family_of(CREDITOR), Some(S41));
    // A closed family holds its own id only; an open one holds every id it begins, and the
    // longest open family that begins an id holds it.
    assert_eq!(family_of("stock/typed_in"), Some("stock/typed_in"));
    assert_eq!(family_of("stock/typed_in_more"), None);
    assert_eq!(
        family_of("tds_payees/not_deposited/194C_0a1b2c3d"),
        Some("tds_payees/not_deposited/")
    );
    assert_eq!(family_of("tds_payees/194C_0a1b2c3d"), Some("tds_payees/"));
    assert_eq!(
        family_of("tds_payees/deductor_status"),
        Some("tds_payees/deductor_status")
    );
}

#[test]
fn nothing_expected_and_nothing_listed_is_refused() {
    let key = open(
        r#"
test_id = "stale_balances_41_1"
[[family]]
prefix = "stale_balances_41_1/s41_1/"
[[family]]
prefix = "stale_balances_41_1/recoverability/"
"#,
    )
    .unwrap();
    assert_eq!(
        score(&key, &dump(vec![]), &Book::default()),
        Err(ScoreError::Empty)
    );
}

#[test]
fn a_dump_refused_before_any_count() {
    let key = open(KEY).unwrap();
    let none = Book::default();
    let mut other = dump(vec![]);
    other["test_id"] = json!("cash_44ab");
    assert_eq!(
        score(&key, &other, &none),
        Err(ScoreError::TestIdDiffers {
            key: "stale_balances_41_1".to_string(),
            dump: "cash_44ab".to_string(),
        })
    );
    let one_family = open(
        "test_id = \"stale_balances_41_1\"\n[[family]]\nprefix = \"stale_balances_41_1/s41_1/\"\n",
    )
    .unwrap();
    assert_eq!(
        score(&one_family, &dump(vec![finding(DEBTOR, &["X"])]), &none),
        Err(ScoreError::FamilyNotInKey(RECOVERABILITY.to_string()))
    );
    assert!(matches!(
        score(
            &key,
            &dump(vec![finding(DEBTOR, &[]), finding(DEBTOR, &[])]),
            &none
        ),
        Err(ScoreError::DumpInvalid(_))
    ));
    assert!(matches!(
        score(&key, &json!({"test_id": "stale_balances_41_1"}), &none),
        Err(ScoreError::DumpInvalid(_))
    ));
    for evidence in [
        json!([{"kind": "ledger", "label": ""}]),
        json!([{"kind": "ledger", "id": "A"}]),
        json!([{"kind": "Ledger", "id": "A", "label": ""}]),
    ] {
        let mut d = dump(vec![finding(DEBTOR, &[])]);
        d["findings"][0]["evidence"] = evidence;
        assert!(matches!(
            score(&key, &d, &none),
            Err(ScoreError::DumpInvalid(_))
        ));
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
    let head =
        "test_id = \"stale_balances_41_1\"\n[[family]]\nprefix = \"stale_balances_41_1/s41_1/\"\n";
    let parse = |text: &str| matches!(open(text), Err(KeyError::Parse(_)));
    assert!(parse(&format!("{head}extra = 1\n")));
    assert!(parse(&format!(
        "{head}[[family.item]]\nkind = \"ledger\"\nname = \"A\"\n"
    )));
    // A kind outside the crate's list, a misspelling or another case, does not open.
    for kind in ["Ledger", "ledgr", "ledger:x", ""] {
        assert!(
            parse(&format!(
                "{head}[[family.item]]\nkind = \"{kind}\"\nname = \"A\"\nreason = \"r\"\n"
            )),
            "{kind}"
        );
    }
    assert_eq!(open("test_id = \"t\"\n").unwrap_err(), KeyError::NoFamilies);
    assert_eq!(
        open("test_id = \" \"\n[[family]]\nprefix = \"stock/typed_in\"\n").unwrap_err(),
        KeyError::Blank("test_id")
    );
    // A family is one of the list, whole: not an id inside it, nor a shorter prefix.
    for prefix in [
        "stale_balances_41_1/s41_1/creditor",
        "stale_balances_41_1/s41_1",
        "stale_balances_41_1/",
    ] {
        assert_eq!(
            open(&format!(
                "test_id = \"stale_balances_41_1\"\n[[family]]\nprefix = \"{prefix}\"\n"
            ))
            .unwrap_err(),
            KeyError::UnknownFamily(prefix.to_string())
        );
    }
    assert_eq!(
        open("test_id = \"stale_balances_41_1\"\n[[family]]\nprefix = \"stock/typed_in\"\n")
            .unwrap_err(),
        KeyError::FamilyOfAnotherTest("stock/typed_in".to_string())
    );
    assert_eq!(
        open(&format!(
            "{head}[[family]]\nprefix = \"stale_balances_41_1/s41_1/\"\n"
        ))
        .unwrap_err(),
        KeyError::DuplicateFamily(S41.to_string())
    );
    assert_eq!(
        open(&format!(
            "{head}[[family.item]]\nkind = \"ledger\"\nname = \"A\"\nreason = \"\"\n"
        ))
        .unwrap_err(),
        KeyError::Blank("item.reason")
    );
    assert_eq!(
        open(&format!(
            "{head}[[family.item]]\nkind = \"ledger\"\nname = \" \"\nreason = \"r\"\n"
        ))
        .unwrap_err(),
        KeyError::Blank("item.name")
    );
    for item in [
        "kind = \"ledger\"\nreason = \"r\"\n",
        "kind = \"ledger\"\nname = \"A\"\ndate = \"2025-06-01\"\nreason = \"r\"\n",
        "kind = \"voucher\"\nvoucher_type = \"Journal\"\nnumber = \"J/2\"\nreason = \"r\"\n",
        "kind = \"voucher\"\nname = \"A\"\nvoucher_type = \"Journal\"\nnumber = \"J/2\"\ndate = \"2025-06-01\"\nreason = \"r\"\n",
        "kind = \"voucher\"\nvoucher_type = \"Journal\"\nnumber = \"J/2\"\ndate = \"01-06-2025\"\nreason = \"r\"\n",
    ] {
        assert!(
            matches!(
                open(&format!("{head}[[family.item]]\n{item}")),
                Err(KeyError::ItemFields(_))
            ),
            "{item}"
        );
    }
}

fn golden(name: &str) -> Value {
    common::golden_named(name.strip_suffix(".json").unwrap())
}

/// The committed synthetic golden is the canonical dump's own shape: a key naming its two listed
/// ledgers matches both, so the scorer reads `findings[].evidence[]` where the dump puts them.
#[test]
fn the_scorer_reads_the_canonical_dump_shape() {
    let golden = golden("synthetic.stale_balances_41_1.json");
    let key = open(
        r#"
test_id = "stale_balances_41_1"
[[family]]
prefix = "stale_balances_41_1/s41_1/"
[[family.item]]
kind = "ledger"
name = "Harbour Mill Supplies"
reason = "shape check only"
[[family]]
prefix = "stale_balances_41_1/recoverability/"
[[family.item]]
kind = "ledger"
name = "Quarry Lane Stores"
reason = "shape check only"
"#,
    )
    .unwrap();
    let got = score(&key, &golden, &book()).unwrap();
    for f in &got.families {
        assert_eq!((f.expected, f.listed, f.matched), (1, 1, 1), "{}", f.family);
        assert!(f.missed.is_empty() && f.extra.is_empty());
    }
    assert_eq!(got.families.len(), 2);
}

/// A golden family of four findings, one per voucher type, over the synthetic Book: every voucher
/// ref resolves through its GUID, and a key naming one voucher of the journal finding and one of
/// the payment finding matches both in the pooled family.
#[test]
fn a_golden_family_pools_its_findings_and_resolves_every_voucher() {
    let golden = golden("synthetic.book_keeping_quality.json");
    let key = format!(
        "test_id = \"book_keeping_quality\"\n\
[[family]]\nprefix = \"book_keeping_quality/entry_order/after_last_sale\"\n\
[[family]]\nprefix = \"book_keeping_quality/stock_untouched\"\n\
[[family]]\nprefix = \"{LAG}\"\n{}{}",
        voucher_item("Journal", "J/2", "2025-06-01"),
        voucher_item("Payment", "P/8", "2025-06-10"),
    );
    let got = score(&open(&key).unwrap(), &golden, &book()).unwrap();
    let f = family(&got, LAG);
    assert_eq!((f.expected, f.listed, f.matched), (2, 29, 2));
    assert!(f.missed.is_empty() && f.unresolved.is_empty() && f.unscored.is_empty());
    assert_eq!(f.extra.len(), 27);
    assert!(got.families.iter().all(|f| f.unresolved.is_empty()));
}

/// Every finding of every committed golden belongs to a family.
#[test]
fn every_committed_golden_finding_has_a_family() {
    let dir = common::fixtures().join("golden");
    let mut seen = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|e| e != "json") {
            continue;
        }
        let doc: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        for f in doc
            .get("findings")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let id = f["id"].as_str().unwrap();
            assert!(family_of(id).is_some(), "{}: {id}", path.display());
            seen += 1;
        }
    }
    assert!(seen > 1000, "{seen}");
}

/// [`FAMILIES`] is exactly the set of finding-id `format!` prefixes in `src/`, read from each
/// module's text before its test module, with the module's `TEST_ID` written out.
#[test]
fn the_families_are_the_format_prefixes_of_src() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = BTreeSet::new();
    for entry in std::fs::read_dir(&src).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let body = ["#[cfg(test)]\nmod ", "#[cfg(test)]\npub(crate) mod "]
            .iter()
            .filter_map(|m| text.find(m))
            .min()
            .map_or(text.as_str(), |at| &text[..at]);
        let Some(at) = body.find("pub const TEST_ID: &str = \"") else {
            assert!(!body.contains("\"{TEST_ID}/"));
            continue;
        };
        let rest = &body[at + "pub const TEST_ID: &str = \"".len()..];
        let test_id = &rest[..rest.find('"').unwrap()];
        for (at, _) in body.match_indices("\"{TEST_ID}/") {
            let tail = &body[at + "\"{TEST_ID}/".len()..];
            let format = &tail[..tail.find('"').unwrap()];
            found.insert(match format.find('{') {
                Some(cut) => format!("{test_id}/{}{{", &format[..cut]),
                None => format!("{test_id}/{format}"),
            });
        }
    }
    let listed: BTreeSet<String> = FAMILIES.iter().map(|f| (*f).to_string()).collect();
    assert_eq!(listed.len(), FAMILIES.len(), "a family is listed twice");
    assert_eq!(found, listed);
}
