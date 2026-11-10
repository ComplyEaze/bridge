//! The several-currency movement's rules on answers captured from one synthetic book on
//! TallyPrime 7.2 (`FOREXGROUP_FIRST_DAY_CAPTURE_PROVENANCE.md`; reference §8.2n). The book's base
//! is identified from its own captured currency reads, as Bridge identifies it.
use super::*;
use bridge_tally_protocol::native_outstandings::{
    parse_company_currency_name, parse_currency_master_list,
};
use bridge_tally_protocol::native_trial_balance::{
    parse_native_trial_balance_with_currency, NativeTrialBalanceAmount,
};
use std::collections::BTreeSet;

const FOREXGROUP: &str = "1bb48be5-baba-45b2-90d8-7593244915a9";

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

const AS_FOUND_EXPORT: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/native/ledgers_native_forexgroup_first_day.utf16le.xml"
);
const DOLLAR_OPENING_EXPORT: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/native/ledgers_native_forexgroup_usd_opening_first_day.utf16le.xml"
);
const AS_FOUND_TRIAL_BALANCE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_currency_forexgroup_first_day.utf16le.xml"
);
const DOLLAR_OPENING_TRIAL_BALANCE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_currency_forexgroup_usd_opening_first_day.utf16le.xml"
);
const ALL_PLAIN_TRIAL_BALANCE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_currency_forexgroup_first_day_all_plain.utf16le.xml"
);
const BOOK_WINDOW: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/movement-forexgroup-book-20250401.utf16le.xml"
);
const USD_CUSTOMER_WINDOW: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/movement-forexgroup-usd-customer-20250401.utf16le.xml"
);

/// The six ledgers the as-found Trial Balance sets aside (row 683-TB's `tb1`).
const SET_ASIDE_AS_FOUND: [&str; 6] = [
    "BRIDGE Export Sales",
    "BRIDGE Import Purchases",
    "BRIDGE Lab Bank",
    "BRIDGE USD Customer",
    "BRIDGE USD Supplier",
    "Profit & Loss A/c",
];

/// The Trial Balance as `trial_balance` reads a several-currency book: its scope and its plain rows.
fn trial_balance(bytes: &[u8]) -> (TrialBalanceLedgerScope, NativeTrialBalance) {
    let base = parse_currency_master_list(&utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/native/currency_masters_originalname_forex_72.utf16le.xml"
    )))
    .unwrap()
    .identify_base(Some(
        parse_company_currency_name(
            &utf16(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/native/company_base_currency_forex_72.utf16le.xml"
            )),
            FOREXGROUP,
        )
        .unwrap()
        .as_str(),
    ))
    .unwrap();
    let scoped =
        parse_native_trial_balance_with_currency(&utf16(bytes), FOREXGROUP, base.base()).unwrap();
    (
        TrialBalanceLedgerScope::BaseCurrencyLedgersOnly {
            base_name: base.base().name().to_string(),
            decimal_places: base.decimal_places(),
            foreign: scoped.foreign_currency_ledgers,
            mixed: scoped.mixed_currency_ledgers,
        },
        scoped.report,
    )
}

fn catalogue(bytes: &[u8]) -> Vec<TallyLedger> {
    bridge_tally_protocol::parse_native_ledger_source_records_keeping_composite_openings(
        &utf16(bytes),
        FOREXGROUP,
    )
    .unwrap()
    .records
    .into_iter()
    .map(|record| record.record)
    .collect()
}

fn window(bytes: &[u8]) -> Vec<MovementVoucher> {
    let rows =
        parse_agent_changed_rows_naming_composite_ledgers(&utf16(bytes), FOREXGROUP).unwrap();
    parse_scoped_movement_rows(rows, WindowScope::Book, "20250401", "20250401")
        .unwrap()
        .rows
}

fn first_day(composite_openings: &[&str]) -> SeveralCurrencyFirstDay {
    SeveralCurrencyFirstDay::for_tests(
        composite_openings
            .iter()
            .map(|name| (*name).to_string())
            .collect(),
    )
}

fn set_aside_names(set_aside: &CurrencySetAside, catalogue: &[TallyLedger]) -> BTreeSet<String> {
    catalogue
        .iter()
        .filter(|ledger| set_aside.rule_of(&ledger.name).is_some())
        .map(|ledger| ledger.name.clone())
        .collect()
}

/// The ledger an inconclusive refusal names and where it found the composite.
fn inconclusive(failure: ToolFailure) -> (String, &'static str) {
    assert_eq!(failure.code, "ledger_currency_scope_inconclusive");
    failure
        .read_detail
        .and_then(|detail| detail.set_aside)
        .expect("the refusal names the ledger")
}

/// The book as found, from its first day. The six ledgers `trial_balance` sets aside are
/// set aside; the two it reads are kept, and their movement over the captured window ties to
/// Tally's own Trial Balance for them.
#[test]
fn the_book_as_found_sets_aside_the_six_and_keeps_the_two_that_tie() {
    let catalogue = catalogue(AS_FOUND_EXPORT);
    let (scope, kept) = trial_balance(AS_FOUND_TRIAL_BALANCE);
    let set_aside =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    let vouchers = window(BOOK_WINDOW);
    assert!(set_aside.check_composite_entries(&vouchers).is_ok());
    assert_eq!(
        set_aside_names(&set_aside, &catalogue),
        SET_ASIDE_AS_FOUND.map(String::from).into()
    );
    for ledger in SET_ASIDE_AS_FOUND {
        assert_eq!(
            set_aside.rule_of(ledger),
            Some(SetAsideRule::MixedCurrencyMovement),
            "{ledger}"
        );
    }
    // Each kept ledger moved 3000.00 to the debit on the day; Tally's Trial Balance closes each
    // at -3000.00 from a zero opening (a debit is negative in Tally's XML).
    let three_thousand =
        |sign: &str| bridge_tally_core::ExactDecimal::parse(format!("{sign}3000.00")).unwrap();
    for ledger in ["BRIDGE INR Customer", "Cash"] {
        assert_eq!(set_aside.rule_of(ledger), None);
        let moves = day_moves_of(&vouchers, ledger).unwrap();
        assert_eq!(moves.keys().collect::<Vec<_>>(), ["20250401"], "{ledger}");
        assert!(
            moves["20250401"].numeric_eq(&three_thousand("")),
            "{ledger}"
        );
        let row = kept.rows.iter().find(|row| row.name == ledger).unwrap();
        assert_eq!(
            row.opening,
            NativeTrialBalanceAmount::Present(
                bridge_tally_core::ExactDecimal::parse("0.00").unwrap()
            )
        );
        assert_eq!(
            row.closing,
            NativeTrialBalanceAmount::Present(three_thousand("-"))
        );
    }
    assert_eq!(kept.rows.len(), 2);
    let mut result = serde_json::Map::new();
    set_aside.frame(&mut result, Redaction::None);
    assert_eq!(result["ledgers_scope"], "base_currency_ledgers_only");
    assert_eq!(result["foreign_currency_ledgers_excluded"]["count"], 0);
    assert_eq!(result["base_currency_ledgers_mixed_excluded"]["count"], 6);
    assert_eq!(
        result["base_currency_ledgers_mixed_excluded"]["reason"],
        "mixed_currency_movement"
    );
    assert_eq!(
        result["base_currency_ledgers_mixed_excluded"]["ledgers"],
        json!(SET_ASIDE_AS_FOUND)
    );
    assert!(result.contains_key("scope_limitation"));
    assert!(result.contains_key("scope_label"));
    let mut masked = serde_json::Map::new();
    set_aside.frame(&mut masked, Redaction::MaskParties);
    assert!(!masked["base_currency_ledgers_mixed_excluded"]["ledgers"]
        .to_string()
        .contains("BRIDGE"));
}

/// The captured window's composite entries are on five of the six ledgers the Trial Balance sets
/// aside (`Profit & Loss A/c` has no entry), so the as-found Trial Balance (row 683-TW's answer to
/// the same request, byte-equal to it) passes. Row 683-LD's
/// Trial Balance of the same window, read after two vouchers dated 30 April were saved and Tally's
/// current date had moved to 30 April, reads all eight ledgers as plain. It sets nothing aside, so
/// `Profit & Loss A/c` would be read; the window's composites show what it missed, and the read is
/// refused. The book's own window is used for both, as row 683-LD's year window answered the
/// eleven 1 April vouchers entry by entry as before.
#[test]
fn a_trial_balance_that_reads_a_ledger_with_a_composite_entry_as_plain_is_not_conclusive() {
    let vouchers = window(BOOK_WINDOW);
    let with_composite_entry = vouchers
        .iter()
        .flat_map(|voucher| &voucher.ledger_entries)
        .filter(|entry| entry.amount == MovementAmount::Composite)
        .map(|entry| entry.ledger_name.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        with_composite_entry,
        SET_ASIDE_AS_FOUND[..5].iter().copied().collect()
    );
    let catalogue = catalogue(AS_FOUND_EXPORT);
    let (scope, kept) = trial_balance(AS_FOUND_TRIAL_BALANCE);
    let as_found =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    assert!(as_found.check_composite_entries(&vouchers).is_ok());

    let (scope, kept) = trial_balance(ALL_PLAIN_TRIAL_BALANCE);
    let TrialBalanceLedgerScope::BaseCurrencyLedgersOnly { foreign, mixed, .. } = &scope else {
        panic!("not the several-currency scope");
    };
    assert!(foreign.is_empty() && mixed.is_empty());
    assert_eq!(kept.rows.len(), 8);
    let all_plain =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    assert_eq!(all_plain.rule_of("Profit & Loss A/c"), None);
    let first_composite = vouchers
        .iter()
        .flat_map(|voucher| &voucher.ledger_entries)
        .find(|entry| entry.amount == MovementAmount::Composite)
        .unwrap()
        .ledger_name
        .clone();
    assert_eq!(
        inconclusive(all_plain.check_composite_entries(&vouchers).unwrap_err()),
        (first_composite, "composite_entry_in_window")
    );

    let scoped = window(USD_CUSTOMER_WINDOW);
    assert_eq!(scoped.len(), 4);
    assert!(scoped
        .iter()
        .flat_map(|voucher| &voucher.ledger_entries)
        .all(|entry| entry.amount == MovementAmount::Composite));
    assert_eq!(
        inconclusive(all_plain.check_composite_entries(&scoped).unwrap_err()),
        (
            "BRIDGE USD Customer".to_string(),
            "composite_entry_in_window"
        )
    );
}

/// `Profit & Loss A/c` has no entry in the window and a plain opening at the first day, so only the
/// Trial Balance can set it aside. The conclusive Trial Balance does, and its figure is never read;
/// with row 683-LD's Trial Balance it would be kept, so that read refuses before any figure.
#[test]
fn profit_and_loss_is_set_aside_only_by_a_conclusive_trial_balance() {
    const PROFIT_AND_LOSS: &str = "Profit & Loss A/c";
    let vouchers = window(BOOK_WINDOW);
    assert!(!vouchers
        .iter()
        .flat_map(|voucher| &voucher.ledger_entries)
        .any(|entry| entry.ledger_name == PROFIT_AND_LOSS));
    let catalogue = catalogue(AS_FOUND_EXPORT);
    assert!(catalogue
        .iter()
        .any(|ledger| ledger.name == PROFIT_AND_LOSS));

    let (scope, kept) = trial_balance(AS_FOUND_TRIAL_BALANCE);
    let conclusive =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    assert_eq!(
        conclusive.rule_of(PROFIT_AND_LOSS),
        Some(SetAsideRule::MixedCurrencyMovement)
    );
    assert!(conclusive.check_composite_entries(&vouchers).is_ok());

    let (scope, kept) = trial_balance(ALL_PLAIN_TRIAL_BALANCE);
    assert!(kept.rows.iter().any(|row| row.name == PROFIT_AND_LOSS));
    let inconclusive_trial_balance =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    assert_eq!(inconclusive_trial_balance.rule_of(PROFIT_AND_LOSS), None);
    assert!(inconclusive_trial_balance
        .check_composite_entries(&vouchers)
        .is_err());
}

/// A dollar opening keyed on a ledger master (row 683-OP's `x1` and `t1`). The Trial Balance sets
/// the ledger aside, so its composite opening passes the check and it is named once. The
/// composite opening alone is pinned on the same export in the runtime's tests.
#[test]
fn a_dollar_opening_on_a_master_is_set_aside_and_named_once() {
    let catalogue = catalogue(DOLLAR_OPENING_EXPORT);
    let (scope, kept) = trial_balance(DOLLAR_OPENING_TRIAL_BALANCE);
    let trial_balance_alone =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    assert_eq!(
        trial_balance_alone.rule_of("BRIDGE USD Opening"),
        Some(SetAsideRule::MixedCurrencyMovement)
    );
    let both = CurrencySetAside::before_window(
        &first_day(&["BRIDGE USD Opening"]),
        &scope,
        &kept,
        &catalogue,
    )
    .unwrap();
    assert_eq!(set_aside_names(&both, &catalogue).len(), 7);
    let mut result = serde_json::Map::new();
    both.frame(&mut result, Redaction::None);
    assert_eq!(result["base_currency_ledgers_mixed_excluded"]["count"], 7);
}

/// A composite opening on a ledger the Trial Balance reads as plain: the Trial Balance missed a
/// value in another currency, so the read is refused. The admission is made for the test; the
/// captured composite opening is on a ledger the Trial Balance sets aside (the test above).
#[test]
fn a_composite_opening_on_a_ledger_the_trial_balance_reads_as_plain_is_not_conclusive() {
    let catalogue = catalogue(AS_FOUND_EXPORT);
    let (scope, kept) = trial_balance(AS_FOUND_TRIAL_BALANCE);
    assert_eq!(
        inconclusive(
            CurrencySetAside::before_window(&first_day(&["Cash"]), &scope, &kept, &catalogue)
                .unwrap_err()
        ),
        ("Cash".to_string(), "composite_opening")
    );
    assert!(CurrencySetAside::before_window(
        &first_day(&["BRIDGE USD Customer"]),
        &scope,
        &kept,
        &catalogue
    )
    .is_ok());
}

/// The two reports must name the same ledgers. Both pairs are captured answers of the same
/// book a ledger apart (row 683-OP before and after its write).
#[test]
fn a_trial_balance_and_catalogue_that_name_different_ledgers_refuse() {
    let (as_found_scope, as_found_kept) = trial_balance(AS_FOUND_TRIAL_BALANCE);
    // A catalogue ledger the Trial Balance does not name.
    assert_eq!(
        CurrencySetAside::before_window(
            &first_day(&[]),
            &as_found_scope,
            &as_found_kept,
            &catalogue(DOLLAR_OPENING_EXPORT),
        )
        .unwrap_err()
        .code,
        MovementCurrencyCode::ScopeUnmatched.as_str()
    );
    // A set-aside name the catalogue does not hold.
    let (opening_scope, opening_kept) = trial_balance(DOLLAR_OPENING_TRIAL_BALANCE);
    assert_eq!(
        CurrencySetAside::before_window(
            &first_day(&[]),
            &opening_scope,
            &opening_kept,
            &catalogue(AS_FOUND_EXPORT),
        )
        .unwrap_err()
        .code,
        MovementCurrencyCode::ScopeUnmatched.as_str()
    );
    // A Trial Balance read under the one-master admission means the book changed between reads.
    assert_eq!(
        CurrencySetAside::before_window(
            &first_day(&[]),
            &TrialBalanceLedgerScope::AllLedgers,
            &as_found_kept,
            &catalogue(AS_FOUND_EXPORT),
        )
        .unwrap_err()
        .code,
        MovementCurrencyCode::SnapshotDrifted.as_str()
    );
}

/// A named ledger that is set aside refuses, naming it and the rule, with no figure.
#[test]
fn a_named_ledger_that_is_set_aside_refuses_with_its_rule() {
    let catalogue = catalogue(AS_FOUND_EXPORT);
    let (scope, kept) = trial_balance(AS_FOUND_TRIAL_BALANCE);
    let set_aside =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    let refusal = set_aside
        .refuse_named(Some("BRIDGE USD Customer"))
        .unwrap_err();
    assert_eq!(refusal.code, "ledger_currency_set_aside");
    assert_eq!(
        refusal.read_detail.and_then(|detail| detail.set_aside),
        Some(("BRIDGE USD Customer".to_string(), "mixed_currency_movement"))
    );
    assert!(set_aside.refuse_named(Some("Cash")).is_ok());
    assert!(set_aside.refuse_named(None).is_ok());
}

/// Every other caller keeps refusing a composite, and the movement's own policy keeps an
/// entry composite but no allocation with a `BILLTYPE` and a composite amount. The window answers
/// every allocation amount-only (the `vouchers` capture of the day holds the same ones typed), so
/// each is a placeholder, skipped under every policy; a typed one has not been seen in this read.
#[test]
fn only_the_movement_policy_keeps_a_composite_entry_and_none_keeps_a_typed_composite_allocation() {
    let book = utf16(BOOK_WINDOW);
    assert_eq!(
        parse_agent_changed_rows(&book, FOREXGROUP).unwrap_err(),
        "voucher_amount_invalid"
    );
    let rows = parse_agent_changed_rows_naming_composite_ledgers(&book, FOREXGROUP).unwrap();
    assert_eq!(rows.len(), 11);
    let composites = rows
        .iter()
        .flat_map(|row| row["amounts"].as_array().unwrap())
        .filter(|entry| entry["currency_composite"] == true)
        .count();
    assert_eq!(composites, 16);
    // The day-moves read takes `Refuse`; given a marked entry, its sum refuses rather than read it.
    assert_eq!(
        day_moves_of(&window(BOOK_WINDOW), "BRIDGE USD Customer").unwrap_err(),
        "voucher_amount_invalid"
    );
    // DERIVED from the capture: a BILLTYPE added to the first composite allocation.
    let allocation = "<BILLALLOCATIONS.LIST>\r\n       <AMOUNT TYPE=\"Amount\">-$1000000.00";
    assert_eq!(book.matches(allocation).count(), 1);
    let typed = book.replacen(
        allocation,
        "<BILLALLOCATIONS.LIST>\r\n       <BILLTYPE TYPE=\"String\">New Ref</BILLTYPE>\r\n       <AMOUNT TYPE=\"Amount\">-$1000000.00",
        1,
    );
    assert_eq!(
        parse_agent_changed_rows_naming_composite_ledgers(&typed, FOREXGROUP).unwrap_err(),
        "bill_allocation_amount_invalid"
    );
    assert_eq!(
        parse_agent_changed_rows(&typed, FOREXGROUP).unwrap_err(),
        "bill_allocation_amount_invalid"
    );
}
