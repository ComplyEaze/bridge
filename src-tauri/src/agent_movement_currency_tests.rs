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

/// The book as found, from its first day. The six ledgers `trial_balance` sets aside are
/// set aside; the two it reads are kept, and their movement over the captured window ties to
/// Tally's own Trial Balance for them.
#[test]
fn the_book_as_found_sets_aside_the_six_and_keeps_the_two_that_tie() {
    let catalogue = catalogue(AS_FOUND_EXPORT);
    let (scope, kept) = trial_balance(AS_FOUND_TRIAL_BALANCE);
    let mut set_aside =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    let vouchers = window(BOOK_WINDOW);
    set_aside.add_composite_entries(&vouchers);
    assert_eq!(
        set_aside_names(&set_aside, &catalogue),
        SET_ASIDE_AS_FOUND.map(String::from).into()
    );
    for ledger in SET_ASIDE_AS_FOUND {
        assert_eq!(
            set_aside.rule_of(ledger),
            Some(SetAsideRule::MixedCurrencyMovement),
            "{ledger}: rule 1 names it first"
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

/// Rule 2 alone, on the captured window: every ledger with a composite entry, which is every
/// ledger rule 1 names except Profit & Loss A/c (it has no entry in the window).
#[test]
fn rule_two_names_every_ledger_with_a_composite_entry_in_the_captured_window() {
    let mut rule_two_alone = CurrencySetAside {
        foreign: Vec::new(),
        mixed: BTreeMap::new(),
    };
    rule_two_alone.add_composite_entries(&window(BOOK_WINDOW));
    assert_eq!(
        rule_two_alone.mixed,
        SET_ASIDE_AS_FOUND[..5]
            .iter()
            .map(|name| (name.to_string(), SetAsideRule::CompositeEntryInWindow))
            .collect::<BTreeMap<_, _>>()
    );
    let scoped = window(USD_CUSTOMER_WINDOW);
    assert_eq!(scoped.len(), 4);
    assert!(scoped
        .iter()
        .flat_map(|voucher| &voucher.ledger_entries)
        .all(|entry| entry.amount == MovementAmount::Composite));
}

/// A dollar opening keyed on a ledger master (row 683-OP's `x1` and `t1`). Rule 1 sets the
/// ledger aside from the Trial Balance without rule 3, and with both it is named once, by rule 1.
/// Rule 3 alone is pinned on the same export in the runtime's tests.
#[test]
fn a_dollar_opening_on_a_master_is_set_aside_and_named_once() {
    let catalogue = catalogue(DOLLAR_OPENING_EXPORT);
    let (scope, kept) = trial_balance(DOLLAR_OPENING_TRIAL_BALANCE);
    let rule_one_alone =
        CurrencySetAside::before_window(&first_day(&[]), &scope, &kept, &catalogue).unwrap();
    assert_eq!(
        rule_one_alone.rule_of("BRIDGE USD Opening"),
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

/// Rule 3 names what the Trial Balance does not: on the as-found Trial Balance with a composite
/// opening admitted for one of its ledgers, that ledger is set aside by its opening.
#[test]
fn rule_three_sets_aside_a_ledger_rule_one_keeps() {
    let catalogue = catalogue(AS_FOUND_EXPORT);
    let (scope, kept) = trial_balance(AS_FOUND_TRIAL_BALANCE);
    let set_aside =
        CurrencySetAside::before_window(&first_day(&["Cash"]), &scope, &kept, &catalogue).unwrap();
    assert_eq!(
        set_aside.rule_of("Cash"),
        Some(SetAsideRule::CompositeOpening)
    );
    assert_eq!(set_aside.rule_of("BRIDGE INR Customer"), None);
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
        .unwrap_err(),
        MovementCurrencyCode::ScopeUnmatched
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
        .unwrap_err(),
        MovementCurrencyCode::ScopeUnmatched
    );
    // A Trial Balance read under the one-master admission means the book changed between reads.
    assert_eq!(
        CurrencySetAside::before_window(
            &first_day(&[]),
            &TrialBalanceLedgerScope::AllLedgers,
            &as_found_kept,
            &catalogue(AS_FOUND_EXPORT),
        )
        .unwrap_err(),
        MovementCurrencyCode::SnapshotDrifted
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
