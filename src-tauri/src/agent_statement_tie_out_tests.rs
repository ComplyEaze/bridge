//! The tie-out's arithmetic, sign, reading text and file reading, as pure
//! functions. The values are synthetic and deliberately asymmetric, so a sign
//! flip, a swapped pair of figures or a forgotten leg cannot land on the same
//! answer.
use super::*;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

fn date(year: u16, month: u8, day: u8) -> Date {
    Date::new(year, month, day).unwrap()
}

/// A statement that opens at 1000.00 and closes at 2150.00 (statement side),
/// whose file would add a net 179.50 to the bank ledger.
fn file() -> StatementFile {
    StatementFile {
        bank_ledger: "Synthetic Bank Ledger".into(),
        opening: BankSide::from_printed("1000.00").unwrap(),
        closing: BankSide::from_printed("2150.00").unwrap(),
        window: Ok((date(2026, 8, 1), date(2026, 8, 7))),
        proposals_net: BankSide::from_printed("179.50").unwrap(),
    }
}

fn present(opening: &str) -> LedgerRead {
    LedgerRead::Present {
        opening: Some(opening.to_string()),
    }
}

/// The book at the start holds 1500.00 in the bank (a debit, so -1500.00 in
/// Tally's sign) and at the end 2000.00 (-2000.00).
fn observed() -> Observed {
    Observed {
        at_first: present("-1500.00"),
        after_last: present("-2000.00"),
        at_first_again: present("-1500.00"),
    }
}

fn amounts(gaps: &Gaps) -> [Option<String>; 3] {
    let amount = |figure: &Figure| match figure {
        Figure::Gap(gap) => Some(gap.text()),
        Figure::NotEstablished(_) => None,
    };
    [
        amount(&gaps.opening),
        amount(&gaps.closing),
        amount(&gaps.change),
    ]
}

fn all(reason: NotEstablished) -> Gaps {
    Gaps {
        opening: Figure::NotEstablished(reason),
        closing: Figure::NotEstablished(reason),
        change: Figure::NotEstablished(reason),
    }
}

// ---- the sign ---------------------------------------------------------------

#[test]
fn a_debit_balance_in_tallys_sign_is_money_in_the_statements_sign() {
    assert_eq!(BankSide::from_book("-1500.00").unwrap().text(), "1500.00");
    assert_eq!(BankSide::from_book("1500.00").unwrap().text(), "-1500.00");
    assert_eq!(BankSide::from_book("0").unwrap().text(), "0.00");
    assert_eq!(BankSide::from_printed("-250.5").unwrap().text(), "-250.50");
    // A zero never carries a sign, whichever way it was written.
    assert_eq!(BankSide::from_printed("-0.00").unwrap().text(), "0.00");
    assert_eq!(BankSide::from_book("0.00").unwrap().text(), "0.00");
}

#[test]
fn an_opening_of_minus_1500_against_a_statement_opening_of_1000_is_a_gap_of_plus_500() {
    let differing = gaps(Stage::AfterPost, &file(), &observed()).unwrap();
    assert_eq!(differing.opening.amount(), Some("500.00".to_string()));
    assert_eq!(differing.opening.state(), "differs");
}

#[test]
fn a_gap_of_exactly_zero_is_tied_whatever_its_spelling() {
    let observed = Observed {
        at_first: present("-1000"),
        after_last: present("-2150.000"),
        at_first_again: present("-1000.00"),
    };
    let tied = gaps(Stage::AfterPost, &file(), &observed).unwrap();
    assert_eq!(tied.opening.state(), "tied");
    assert_eq!(tied.closing.state(), "tied");
    assert_eq!(tied.change.state(), "tied");
    assert_eq!(
        amounts(&tied),
        [
            Some("0.00".into()),
            Some("0.00".into()),
            Some("0.00".into())
        ]
    );
}

#[test]
fn a_figure_with_more_than_two_places_keeps_every_digit() {
    assert_eq!(BankSide::from_printed("0.125").unwrap().text(), "0.125");
    assert_eq!(BankSide::from_printed("2.500").unwrap().text(), "2.50");
}

// ---- the three figures ------------------------------------------------------

#[test]
fn after_post_the_closing_gap_is_the_book_against_the_statement_and_nothing_else() {
    let after = gaps(Stage::AfterPost, &file(), &observed()).unwrap();
    // 2000.00 - 2150.00; the proposals' 179.50 is not added.
    assert_eq!(
        amounts(&after),
        [
            Some("500.00".into()),
            Some("-150.00".into()),
            Some("-650.00".into())
        ]
    );
}

#[test]
fn before_build_the_closing_gap_adds_what_the_file_would_add() {
    let before = gaps(Stage::BeforeBuild, &file(), &observed()).unwrap();
    // 2000.00 + 179.50 - 2150.00 = 29.50; the opening gap does not move.
    assert_eq!(
        amounts(&before),
        [
            Some("500.00".into()),
            Some("29.50".into()),
            Some("-470.50".into())
        ]
    );
}

#[test]
fn the_change_in_the_window_is_the_closing_gap_less_the_opening_gap() {
    let after = gaps(Stage::AfterPost, &file(), &observed()).unwrap();
    let (Figure::Gap(opening), Figure::Gap(closing), Figure::Gap(change)) =
        (after.opening, after.closing, after.change)
    else {
        panic!("all three are figures");
    };
    assert_eq!(change, closing.minus(&opening).unwrap());
    assert_ne!(change, opening.minus(&closing).unwrap());
}

// ---- what a file's vouchers add to the bank ledger ---------------------------

fn voucher(entries: &[(&str, &str, &str)]) -> Value {
    json!({
        "bridge_txn_id": "st-20260801-0000000000000000",
        "voucher_type": "Payment",
        "entries": entries
            .iter()
            .map(|(ledger, amount, side)| json!({"ledger": ledger, "amount": amount, "side": side}))
            .collect::<Vec<_>>(),
    })
}

fn document(vouchers: Vec<Value>) -> Value {
    json!({
        "schema": "bridge.bank_statement.proposals.v1",
        "bank_ledger": "Synthetic Bank Ledger",
        "controls": {"opening_balance": "1000.00", "closing_balance": "2150.00"},
        "window": {"first_row_date": "2026-08-01", "last_row_date": "2026-08-07", "whole_statement": true},
        "vouchers": vouchers,
    })
}

#[test]
fn a_debit_to_the_bank_ledger_adds_and_a_credit_takes_away_and_other_legs_are_ignored() {
    let net = |vouchers: Vec<Value>| {
        StatementFile::read(&document(vouchers))
            .unwrap()
            .proposals_net
            .text()
    };
    // A receipt: the bank is debited, money in.
    assert_eq!(
        net(vec![voucher(&[
            ("Synthetic Bank Ledger", "300.00", "Dr"),
            ("Customer", "300.00", "Cr")
        ])]),
        "300.00"
    );
    // A payment: the bank is credited, money out.
    assert_eq!(
        net(vec![voucher(&[
            ("Supplier", "120.50", "Dr"),
            ("Synthetic Bank Ledger", "120.50", "Cr")
        ])]),
        "-120.50"
    );
    // Both, with a leg on another ledger that must not count either way.
    assert_eq!(
        net(vec![
            voucher(&[
                ("Synthetic Bank Ledger", "300.00", "Dr"),
                ("Customer", "300.00", "Cr")
            ]),
            voucher(&[
                ("Supplier", "120.50", "Dr"),
                ("Synthetic Bank Ledger", "120.50", "Cr")
            ]),
            voucher(&[("Rent", "999.00", "Dr"), ("Cash", "999.00", "Cr")]),
        ]),
        "179.50"
    );
    // No vouchers at all.
    assert_eq!(net(vec![]), "0.00");
}

#[test]
fn only_an_exact_bank_ledger_name_counts() {
    let vouchers = vec![voucher(&[
        ("synthetic bank ledger", "300.00", "Dr"),
        ("Synthetic Bank Ledger ", "300.00", "Cr"),
    ])];
    assert_eq!(
        StatementFile::read(&document(vouchers))
            .unwrap()
            .proposals_net
            .text(),
        "0.00"
    );
}

#[test]
fn a_file_that_cannot_be_read_fails_closed_with_a_typed_error() {
    let refused = |document: Value| StatementFile::read(&document).err();
    let invalid = Some("proposals_file_invalid".to_string());
    // A side that is neither Dr nor Cr.
    assert_eq!(
        refused(document(vec![voucher(&[(
            "Synthetic Bank Ledger",
            "1.00",
            "Debit"
        )])])),
        invalid
    );
    // An amount that is not a decimal.
    assert_eq!(
        refused(document(vec![voucher(&[(
            "Synthetic Bank Ledger",
            "1,00",
            "Dr"
        )])])),
        invalid
    );
    // An amount below zero: an entry's amount is never negative.
    assert_eq!(
        refused(document(vec![voucher(&[(
            "Synthetic Bank Ledger",
            "-1.00",
            "Dr"
        )])])),
        invalid
    );
    // A statement figure that is not a decimal.
    let mut bad = document(vec![]);
    bad["controls"]["opening_balance"] = json!("one thousand");
    assert_eq!(refused(bad), invalid);
    let mut bad = document(vec![]);
    bad["controls"]["closing_balance"] = Value::Null;
    assert_eq!(refused(bad), invalid);
    // No bank ledger, no vouchers array.
    let mut bad = document(vec![]);
    bad["bank_ledger"] = json!(7);
    assert_eq!(refused(bad), invalid);
    let mut bad = document(vec![]);
    bad.as_object_mut().unwrap().remove("vouchers");
    assert_eq!(refused(bad), invalid);
}

// ---- the window the file recorded --------------------------------------------

#[test]
fn a_whole_statement_window_is_read_as_the_first_and_last_row_dates() {
    let file = StatementFile::read(&document(vec![])).unwrap();
    assert_eq!(file.window, Ok((date(2026, 8, 1), date(2026, 8, 7))));
}

#[test]
fn a_file_without_a_window_is_not_established_not_an_error() {
    let mut without = document(vec![]);
    without.as_object_mut().unwrap().remove("window");
    assert_eq!(
        StatementFile::read(&without).unwrap().window,
        Err(NotEstablished::WindowNotRecorded)
    );
}

#[test]
fn a_narrowed_window_is_not_established_not_an_error() {
    let mut narrowed = document(vec![]);
    narrowed["window"]["whole_statement"] = json!(false);
    assert_eq!(
        StatementFile::read(&narrowed).unwrap().window,
        Err(NotEstablished::WindowNarrowerThanStatement)
    );
}

#[test]
fn a_window_that_is_present_but_malformed_is_refused() {
    let refused = |window: Value| {
        let mut bad = document(vec![]);
        bad["window"] = window;
        StatementFile::read(&bad).err()
    };
    let invalid = Some("proposals_file_invalid".to_string());
    assert_eq!(refused(json!("2026-08-01")), invalid);
    assert_eq!(
        refused(json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-07"})),
        invalid
    );
    assert_eq!(
        refused(
            json!({"first_row_date": "2026-08-07", "last_row_date": "2026-08-01", "whole_statement": true})
        ),
        invalid,
        "first after last"
    );
    assert_eq!(
        refused(
            json!({"first_row_date": "2026-13-01", "last_row_date": "2026-08-07", "whole_statement": true})
        ),
        invalid
    );
    assert_eq!(
        refused(
            json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-07", "whole_statement": "yes"})
        ),
        invalid
    );
}

// ---- the three reads ---------------------------------------------------------

#[test]
fn the_dates_read_are_the_first_row_the_day_after_the_last_and_the_first_again() {
    let dates = |first, last| {
        read_dates(first, last)
            .unwrap()
            .map(|date| date.as_str().to_string())
    };
    assert_eq!(
        dates(date(2026, 8, 1), date(2026, 8, 7)),
        ["20260801", "20260808", "20260801"]
    );
    // Month end, year end, and the leap day.
    assert_eq!(dates(date(2026, 8, 1), date(2026, 8, 31))[1], "20260901");
    assert_eq!(dates(date(2026, 8, 1), date(2026, 12, 31))[1], "20270101");
    assert_eq!(dates(date(2028, 2, 1), date(2028, 2, 28))[1], "20280229");
    assert_eq!(dates(date(2028, 2, 1), date(2028, 2, 29))[1], "20280301");
    assert_eq!(dates(date(2027, 2, 1), date(2027, 2, 28))[1], "20270301");
}

// ---- what a read can fail to show --------------------------------------------

#[test]
fn a_bank_ledger_in_no_read_is_not_in_the_book() {
    let none = Observed {
        at_first: LedgerRead::Absent,
        after_last: LedgerRead::Absent,
        at_first_again: LedgerRead::Absent,
    };
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &none).unwrap(),
        all(NotEstablished::BankLedgerNotInBook)
    );
}

#[test]
fn a_bank_ledger_in_some_reads_and_not_others_means_the_book_changed() {
    let mut changed = observed();
    changed.at_first_again = LedgerRead::Absent;
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
    let mut changed = observed();
    changed.after_last = LedgerRead::Absent;
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
    let mut changed = observed();
    changed.at_first = LedgerRead::Absent;
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
}

#[test]
fn an_opening_that_moved_between_the_two_reads_at_the_first_date_means_the_book_changed() {
    let mut changed = observed();
    changed.at_first_again = present("-1500.01");
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
    // Present in one and absent in the other.
    let mut changed = observed();
    changed.at_first_again = LedgerRead::Present { opening: None };
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
    // The same figure spelled with another scale is not a change.
    let mut same = observed();
    same.at_first_again = present("-1500");
    assert_eq!(
        amounts(&gaps(Stage::AfterPost, &file(), &same).unwrap())[0],
        Some("500.00".into())
    );
}

#[test]
fn an_opening_tally_did_not_return_leaves_only_the_figures_that_need_it() {
    let missing = LedgerRead::Present { opening: None };
    // At the first date: the opening gap and the change need it; the closing gap does not.
    let no_start = Observed {
        at_first: missing.clone(),
        after_last: present("-2000.00"),
        at_first_again: missing.clone(),
    };
    let gaps_ = gaps(Stage::AfterPost, &file(), &no_start).unwrap();
    assert_eq!(
        gaps_.opening,
        Figure::NotEstablished(NotEstablished::OpeningBalanceNotObserved)
    );
    assert_eq!(gaps_.closing.amount(), Some("-150.00".to_string()));
    assert_eq!(
        gaps_.change,
        Figure::NotEstablished(NotEstablished::OpeningBalanceNotObserved)
    );
    // After the last day: the closing gap and the change need it.
    let no_end = Observed {
        at_first: present("-1500.00"),
        after_last: missing,
        at_first_again: present("-1500.00"),
    };
    let gaps_ = gaps(Stage::AfterPost, &file(), &no_end).unwrap();
    assert_eq!(gaps_.opening.amount(), Some("500.00".to_string()));
    assert_eq!(
        gaps_.closing,
        Figure::NotEstablished(NotEstablished::OpeningBalanceNotObserved)
    );
    assert_eq!(
        gaps_.change,
        Figure::NotEstablished(NotEstablished::OpeningBalanceNotObserved)
    );
}

#[test]
fn an_opening_that_is_not_a_decimal_is_an_error_not_a_figure() {
    let mut bad = observed();
    bad.at_first = present("1,500.00 Dr");
    bad.at_first_again = present("1,500.00 Dr");
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &bad).err(),
        Some("ledger_opening_invalid".to_string())
    );
}

#[test]
fn a_ledger_named_twice_in_one_catalogue_is_refused() {
    let ledger = |name: &str, opening: &str| TallyLedger {
        name: name.to_string(),
        parent: Default::default(),
        party_gstin: Default::default(),
        opening_balance: Some(opening.to_string()),
    };
    let one = [ledger("Bank", "-1.00"), ledger("Other", "5.00")];
    assert_eq!(LedgerRead::of(&one, "Bank").unwrap(), present("-1.00"));
    assert_eq!(
        LedgerRead::of(&one, "bank").unwrap(),
        LedgerRead::Absent,
        "exact, not folded"
    );
    let twice = [ledger("Bank", "-1.00"), ledger("Bank", "-2.00")];
    assert_eq!(
        LedgerRead::of(&twice, "Bank").err(),
        Some("ledger_snapshot_drifted".to_string())
    );
}

// ---- the reading text --------------------------------------------------------

const CAUSES: &str = "Uncleared cheques and deposits in transit explain differences like these. So can a missing entry, a repeated entry or the wrong bank ledger. This check cannot tell them apart. To see which vouchers are involved, read the bank ledger's vouchers for these dates.";
const SCOPE: &str =
    "This checks the bank ledger only. A wrong party or expense ledger is not caught here.";

fn window() -> Option<(Date, Date)> {
    Some((date(2026, 8, 1), date(2026, 8, 7)))
}

#[test]
fn the_fixed_sentences_are_exactly_these() {
    let differing = gaps(Stage::AfterPost, &file(), &observed()).unwrap();
    let text = reading(Stage::AfterPost, window(), &differing);
    assert_eq!(text["possible_causes"], CAUSES);
    assert_eq!(text["scope"], SCOPE);
    assert_eq!(
        text["stage"],
        "The closing figure is the book as it stands now."
    );
    let before = reading(Stage::BeforeBuild, window(), &differing);
    assert_eq!(
        before["stage"],
        "The closing figure counts this file's vouchers as if they were already in the book. Once any of them has been imported, use stage after_post."
    );
}

#[test]
fn the_headline_names_every_gap_that_is_not_zero_with_its_sign() {
    let differing = gaps(Stage::AfterPost, &file(), &observed()).unwrap();
    let text = reading(Stage::AfterPost, window(), &differing);
    assert_eq!(
        text["headline"],
        "Bank ledger against the statement, 2026-08-01 to 2026-08-07: opening gap 500.00; closing gap -150.00; change in window -650.00. A positive amount means the book shows more money in the bank than the statement does; a negative amount, less."
    );
}

#[test]
fn the_headline_leaves_out_a_figure_that_is_zero_and_says_so_when_all_are() {
    // Opening ties, the closing does not: the change equals the closing gap.
    let observed = Observed {
        at_first: present("-1000.00"),
        after_last: present("-2000.00"),
        at_first_again: present("-1000.00"),
    };
    let one = gaps(Stage::AfterPost, &file(), &observed).unwrap();
    let text = reading(Stage::AfterPost, window(), &one);
    assert_eq!(
        text["headline"],
        "Bank ledger against the statement, 2026-08-01 to 2026-08-07: closing gap -150.00; change in window -150.00. A positive amount means the book shows more money in the bank than the statement does; a negative amount, less."
    );
    let tied = Observed {
        at_first: present("-1000.00"),
        after_last: present("-2150.00"),
        at_first_again: present("-1000.00"),
    };
    let none = gaps(Stage::AfterPost, &file(), &tied).unwrap();
    let text = reading(Stage::AfterPost, window(), &none);
    assert_eq!(
        text["headline"],
        "Bank ledger against the statement, 2026-08-01 to 2026-08-07: tied at the start and at the end."
    );
    // Nothing differs, so the causes are not offered; the scope always is.
    assert!(text.get("possible_causes").is_none(), "{text}");
    assert_eq!(text["scope"], SCOPE);
}

#[test]
fn the_headline_names_every_figure_that_is_not_established_with_the_reason() {
    let missing = LedgerRead::Present { opening: None };
    let partial = Observed {
        at_first: present("-1500.00"),
        after_last: missing,
        at_first_again: present("-1500.00"),
    };
    let mixed = gaps(Stage::AfterPost, &file(), &partial).unwrap();
    let text = reading(Stage::AfterPost, window(), &mixed);
    assert_eq!(
        text["headline"],
        "Bank ledger against the statement, 2026-08-01 to 2026-08-07: opening gap 500.00; closing gap and change in window not established (Tally returned no opening balance for the bank ledger at one of the dates). A positive amount means the book shows more money in the bank than the statement does; a negative amount, less."
    );
    assert_eq!(text["possible_causes"], CAUSES);

    let none = reading(
        Stage::AfterPost,
        None,
        &all(NotEstablished::WindowNotRecorded),
    );
    assert_eq!(
        none["headline"],
        "Bank ledger against the statement: opening gap, closing gap and change in window not established (this proposals file was written before it recorded the statement's dates; parse the statement again)."
    );
    assert!(none.get("possible_causes").is_none(), "{none}");
}

#[test]
fn the_text_never_says_what_it_must_not() {
    let figures = [
        gaps(Stage::AfterPost, &file(), &observed()).unwrap(),
        all(NotEstablished::WindowNotRecorded),
        all(NotEstablished::WindowNarrowerThanStatement),
        all(NotEstablished::WindowPrecedesBooksFrom),
        all(NotEstablished::BankLedgerNotInBook),
        all(NotEstablished::BookChangedDuringRead),
        all(NotEstablished::OpeningBalanceNotObserved),
    ];
    for gaps in &figures {
        for stage in [Stage::BeforeBuild, Stage::AfterPost] {
            let text = reading(stage, window(), gaps).to_string().to_lowercase();
            for word in ["duplicate", "free", "offline", "preview", "likely"] {
                assert!(!text.contains(word), "{word} in {text}");
            }
        }
    }
}

// ---- what is returned --------------------------------------------------------

#[test]
fn the_result_carries_the_stage_the_dates_and_three_figures_and_no_balance() {
    let differing = gaps(Stage::BeforeBuild, &file(), &observed()).unwrap();
    let result = result_json(Stage::BeforeBuild, window(), &differing);
    assert_eq!(result["stage"], "before_build");
    assert_eq!(
        result["window"],
        json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-07"})
    );
    assert_eq!(
        result["opening_gap"],
        json!({"state": "differs", "amount": "500.00", "reason": null})
    );
    assert_eq!(
        result["closing_gap"],
        json!({"state": "differs", "amount": "29.50", "reason": null})
    );
    assert_eq!(
        result["change_in_window"],
        json!({"state": "differs", "amount": "-470.50", "reason": null})
    );
    // Every key, so nothing else can be added unseen.
    let mut keys = result
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    keys.sort();
    assert_eq!(
        keys,
        [
            "change_in_window",
            "closing_gap",
            "gap_basis",
            "opening_gap",
            "reading",
            "stage",
            "window"
        ]
    );
    // None of the four balances behind the gaps is in it, in any spelling.
    let text = result.to_string();
    for balance in ["1500", "2000", "1000", "2150"] {
        assert!(!text.contains(balance), "{balance} in {text}");
    }
}

#[test]
fn a_figure_that_is_not_established_carries_its_code_and_no_amount() {
    let result = result_json(
        Stage::AfterPost,
        None,
        &all(NotEstablished::BankLedgerNotInBook),
    );
    assert_eq!(result["window"], Value::Null);
    for figure in ["opening_gap", "closing_gap", "change_in_window"] {
        assert_eq!(
            result[figure],
            json!({"state": "not_established", "amount": null, "reason": "bank_ledger_not_in_book"})
        );
    }
}

// ---- the tool, against replayed captured responses -----------------------------
//
// The ledger catalogue is a captured native response from a synthetic lab with
// one value changed in memory: the opening balance of its Cash ledger, standing
// in for the bank ledger. The captures stay byte-exact on disk; no live
// behaviour of Tally is claimed by these tests.

const COMPANY_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const BANK: &str = "Cash";
const SCHEMA: &str = "bridge.bank_statement.proposals.v1";

fn utf16(bytes: &[u8]) -> String {
    let words = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    String::from_utf16(&words).unwrap()
}

fn plan(body: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(body))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

/// How the bank ledger shows in one catalogue read.
enum Bank<'a> {
    Opening(&'a str),
    NoOpening,
    Renamed,
}

fn catalogue(bank: Bank<'_>) -> ScenarioPlan {
    let base = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-period-opening.utf16le.xml"
    ));
    let opening = "<OPENINGBALANCE TYPE=\"Amount\">-12.50</OPENINGBALANCE>";
    assert_eq!(base.matches(opening).count(), 1);
    let body = match bank {
        Bank::Opening(value) => base.replace(
            opening,
            &format!("<OPENINGBALANCE TYPE=\"Amount\">{value}</OPENINGBALANCE>"),
        ),
        Bank::NoOpening => base.replace(opening, ""),
        Bank::Renamed => {
            let renamed = base
                .replace(
                    "NAME=\"Cash\" RESERVEDNAME",
                    "NAME=\"Cash Moved\" RESERVEDNAME",
                )
                .replace("<NAME>Cash</NAME>", "<NAME>Cash Moved</NAME>");
            assert_ne!(renamed, base);
            renamed
        }
    };
    plan(body)
}

/// The replies of one ledger-catalogue read, in the order the runtime asks.
fn ledger_read(ledger: &ScenarioPlan) -> Vec<ScenarioPlan> {
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    let company = plan(utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    )));
    let extent = plan(
        include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )
        .to_string(),
    );
    let currency = plan(utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    )));
    [
        &status, &company, &company, &extent, &status, &extent, &status, &currency, &status,
        &currency, &status, ledger, &status, ledger, &status, &extent, &status, &extent, &status,
        &company, &status, &company,
    ]
    .into_iter()
    .cloned()
    .collect()
}

/// The replies the company check asks for before any ledger read.
fn company_check() -> Vec<ScenarioPlan> {
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    let company = plan(utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    )));
    vec![company.clone(), status.clone(), company, status]
}

fn server(directory: &std::path::Path, port: Option<std::net::SocketAddr>) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: port.map_or("127.0.0.1".to_string(), |address| address.ip().to_string()),
            port: port.map_or(9, |address| address.port()),
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

/// A proposals file as `parse_bank_statement` writes one, in the data
/// directory, and the id and digest the parse would have returned.
fn publish(directory: &std::path::Path, document: &Value) -> (String, String) {
    let proposals_id = format!("statement-{}", uuid::Uuid::new_v4());
    let bytes = serde_json::to_vec_pretty(&{
        let mut document = document.clone();
        document["proposals_id"] = json!(proposals_id);
        document
    })
    .unwrap();
    let folder = directory.join("bank-statements");
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join(format!("{proposals_id}.json"));
    std::fs::write(&path, &bytes).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
    (proposals_id, sha256_hex(&bytes))
}

fn proposals(bank_ledger: &str, window: Option<Value>) -> Value {
    let mut document = json!({
        "schema": SCHEMA,
        "bank_ledger": bank_ledger,
        "controls": {"opening_balance": "1000.00", "closing_balance": "2150.00"},
        "vouchers": [
            {"bridge_txn_id": "st-20260802-0000000000000001", "voucher_type": "Receipt",
             "entries": [
                {"ledger": bank_ledger, "amount": "300.00", "side": "Dr"},
                {"ledger": "Customer", "amount": "300.00", "side": "Cr"}]},
            {"bridge_txn_id": "st-20260803-0000000000000002", "voucher_type": "Payment",
             "entries": [
                {"ledger": "Supplier", "amount": "120.50", "side": "Dr"},
                {"ledger": bank_ledger, "amount": "120.50", "side": "Cr"}]},
        ],
    });
    if let Some(window) = window {
        document["window"] = window;
    }
    document
}

fn whole(first: &str, last: &str) -> Value {
    json!({"first_row_date": first, "last_row_date": last, "whole_statement": true})
}

async fn tie_out(server: &Server, (proposals_id, digest): &(String, String), stage: &str) -> Value {
    server
        .call_tool(
            "statement_tie_out",
            json!({
                "company_guid": COMPANY_GUID,
                "proposals_id": proposals_id,
                "proposals_sha256": digest,
                "stage": stage,
            }),
        )
        .await
}

fn figures(response: &Value) -> [(String, Option<String>, Option<String>); 3] {
    let result = &response["structuredContent"]["result"];
    ["opening_gap", "closing_gap", "change_in_window"].map(|key| {
        (
            result[key]["state"].as_str().unwrap().to_string(),
            result[key]["amount"].as_str().map(str::to_string),
            result[key]["reason"].as_str().map(str::to_string),
        )
    })
}

fn amount(state: &str, amount: &str) -> (String, Option<String>, Option<String>) {
    (state.to_string(), Some(amount.to_string()), None)
}

fn unestablished(reason: &str) -> (String, Option<String>, Option<String>) {
    (
        "not_established".to_string(),
        None,
        Some(reason.to_string()),
    )
}

/// A run of three catalogue reads, in the order they are made, over a book
/// whose bank ledger shows these three states.
async fn three_reads(
    bank: [Bank<'_>; 3],
    bank_ledger: &str,
    window: Value,
    stage: &str,
) -> (Value, usize) {
    let mut plans = company_check();
    for state in bank {
        plans.extend(ledger_read(&catalogue(state)));
    }
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let file = publish(directory.path(), &proposals(bank_ledger, Some(window)));
    let response = tie_out(
        &server(directory.path(), Some(simulator.address())),
        &file,
        stage,
    )
    .await;
    (response, simulator.finish().unwrap().len())
}

const ALL_READS: usize = 4 + 3 * 22;

#[tokio::test]
async fn a_book_that_differs_gives_the_gaps_with_their_sign() {
    // Start 1500.00 against the statement's 1000.00; end 2000.00 against 2150.00.
    let (response, requests) = three_reads(
        [
            Bank::Opening("-1500.00"),
            Bank::Opening("-2000.00"),
            Bank::Opening("-1500.00"),
        ],
        BANK,
        whole("2026-08-01", "2026-08-07"),
        "after_post",
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    assert_eq!(
        figures(&response),
        [
            amount("differs", "500.00"),
            amount("differs", "-150.00"),
            amount("differs", "-650.00")
        ]
    );
    assert_eq!(requests, ALL_READS);
    assert_eq!(
        response["structuredContent"]["evidence"]["state"],
        "complete"
    );
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["window"],
        json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-07"})
    );
    assert_eq!(result["stage"], "after_post");
    // The company the answer is about is named beside it.
    assert_eq!(
        response["structuredContent"]["company"]["guid"],
        COMPANY_GUID
    );
}

#[tokio::test]
async fn before_build_counts_the_files_own_vouchers_in_the_closing_gap() {
    let (response, _) = three_reads(
        [
            Bank::Opening("-1500.00"),
            Bank::Opening("-2000.00"),
            Bank::Opening("-1500.00"),
        ],
        BANK,
        whole("2026-08-01", "2026-08-07"),
        "before_build",
    )
    .await;
    // 2000.00 + (300.00 - 120.50) - 2150.00.
    assert_eq!(
        figures(&response),
        [
            amount("differs", "500.00"),
            amount("differs", "29.50"),
            amount("differs", "-470.50")
        ]
    );
}

#[tokio::test]
async fn a_book_that_matches_the_statement_at_both_ends_is_tied() {
    let (response, _) = three_reads(
        [
            Bank::Opening("-1000.00"),
            Bank::Opening("-2150.00"),
            Bank::Opening("-1000"),
        ],
        BANK,
        whole("2026-08-01", "2026-08-07"),
        "after_post",
    )
    .await;
    assert_eq!(
        figures(&response),
        [
            amount("tied", "0.00"),
            amount("tied", "0.00"),
            amount("tied", "0.00")
        ]
    );
    assert!(
        response["structuredContent"]["result"]["reading"]["headline"]
            .as_str()
            .unwrap()
            .ends_with("tied at the start and at the end.")
    );
}

#[tokio::test]
async fn a_bank_ledger_the_book_does_not_have_is_not_established_not_an_error() {
    let (response, requests) = three_reads(
        [
            Bank::Opening("-1500.00"),
            Bank::Opening("-2000.00"),
            Bank::Opening("-1500.00"),
        ],
        "No Such Bank",
        whole("2026-08-01", "2026-08-07"),
        "after_post",
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    assert_eq!(
        figures(&response),
        [
            unestablished("bank_ledger_not_in_book"),
            unestablished("bank_ledger_not_in_book"),
            unestablished("bank_ledger_not_in_book"),
        ]
    );
    assert_eq!(requests, ALL_READS);
    // Not everything was established, and the evidence says so.
    assert_eq!(
        response["structuredContent"]["evidence"]["state"],
        "partial"
    );
    assert_eq!(
        response["structuredContent"]["evidence"]["reason_code"],
        "bank_ledger_not_in_book"
    );
}

#[tokio::test]
async fn a_book_that_changed_between_the_two_reads_at_the_first_date_is_not_established() {
    let (response, _) = three_reads(
        [
            Bank::Opening("-1500.00"),
            Bank::Opening("-2000.00"),
            Bank::Opening("-1600.00"),
        ],
        BANK,
        whole("2026-08-01", "2026-08-07"),
        "after_post",
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    assert_eq!(
        figures(&response),
        [
            unestablished("book_changed_during_read"),
            unestablished("book_changed_during_read"),
            unestablished("book_changed_during_read"),
        ]
    );
    // The ledger leaving the book between reads is the same finding.
    let (response, _) = three_reads(
        [
            Bank::Opening("-1500.00"),
            Bank::Opening("-2000.00"),
            Bank::Renamed,
        ],
        BANK,
        whole("2026-08-01", "2026-08-07"),
        "after_post",
    )
    .await;
    assert_eq!(
        figures(&response)[0],
        unestablished("book_changed_during_read")
    );
}

/// `opening_balance_not_observed` is reachable from a catalogue row that carries
/// no opening, but the native ledger reader refuses such a row (the whole read
/// fails closed: a missing balance is never read as zero), so no live read
/// reaches it today. The pure tests above hold the branch for the day it can.
#[tokio::test]
async fn a_catalogue_row_without_an_opening_refuses_the_read_before_any_figure() {
    let mut plans = company_check();
    plans.extend(
        ledger_read(&catalogue(Bank::NoOpening))
            .into_iter()
            .take(14),
    );
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let file = publish(
        directory.path(),
        &proposals(BANK, Some(whole("2026-08-01", "2026-08-07"))),
    );
    let response = tie_out(
        &server(directory.path(), Some(simulator.address())),
        &file,
        "after_post",
    )
    .await;
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"],
        "ledger_movement_read_failed"
    );
}

#[tokio::test]
async fn a_statement_that_starts_before_the_books_is_not_established_and_no_ledger_is_read() {
    let simulator = SequenceSimulator::spawn(company_check()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let file = publish(
        directory.path(),
        &proposals(BANK, Some(whole("2026-03-31", "2026-08-07"))),
    );
    let response = tie_out(
        &server(directory.path(), Some(simulator.address())),
        &file,
        "after_post",
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    assert_eq!(
        figures(&response),
        [
            unestablished("window_precedes_books_from"),
            unestablished("window_precedes_books_from"),
            unestablished("window_precedes_books_from"),
        ]
    );
    assert_eq!(simulator.finish().unwrap().len(), 4);
}

#[tokio::test]
async fn a_file_without_a_whole_window_is_not_established_and_tally_is_never_asked() {
    // Port 9 answers nothing: a request sent would fail the call.
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), None);
    for (window, reason) in [
        (None, "window_not_recorded"),
        (
            Some(
                json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-07", "whole_statement": false}),
            ),
            "window_narrower_than_statement",
        ),
    ] {
        let file = publish(directory.path(), &proposals(BANK, window));
        let response = tie_out(&server, &file, "before_build").await;
        assert_eq!(response["isError"], false, "{response}");
        assert_eq!(
            figures(&response),
            [
                unestablished(reason),
                unestablished(reason),
                unestablished(reason)
            ]
        );
        let result = &response["structuredContent"]["result"];
        assert_eq!(result["window"], Value::Null);
        assert_eq!(
            response["structuredContent"]["evidence"]["state"],
            "partial"
        );
    }
}

#[tokio::test]
async fn a_proposals_file_is_checked_as_build_import_xml_checks_it() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), None);
    let code = |response: &Value| {
        response["structuredContent"]["result"]["error"]["code"]
            .as_str()
            .map(str::to_string)
    };
    let file = publish(
        directory.path(),
        &proposals(BANK, Some(whole("2026-08-01", "2026-08-07"))),
    );
    // Not there.
    let missing = (
        format!("statement-{}", uuid::Uuid::new_v4()),
        file.1.clone(),
    );
    assert_eq!(
        code(&tie_out(&server, &missing, "after_post").await).as_deref(),
        Some("proposals_not_found")
    );
    // Changed since the parse returned its digest.
    let changed = (file.0.clone(), "0".repeat(64));
    assert_eq!(
        code(&tie_out(&server, &changed, "after_post").await).as_deref(),
        Some("proposals_changed")
    );
    // Not a proposals file of this schema.
    let mut other = proposals(BANK, None);
    other["schema"] = json!("something.else");
    let other = publish(directory.path(), &other);
    assert_eq!(
        code(&tie_out(&server, &other, "after_post").await).as_deref(),
        Some("proposals_file_invalid")
    );
    // Another file's id.
    let mut foreign = proposals(BANK, None);
    foreign["proposals_id"] = json!("statement-x");
    let foreign_id = format!("statement-{}", uuid::Uuid::new_v4());
    let bytes = serde_json::to_vec(&foreign).unwrap();
    let path = directory
        .path()
        .join("bank-statements")
        .join(format!("{foreign_id}.json"));
    std::fs::write(&path, &bytes).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
    assert_eq!(
        code(&tie_out(&server, &(foreign_id, sha256_hex(&bytes)), "after_post").await).as_deref(),
        Some("proposals_file_invalid")
    );
    // An id that is not one the parse issues.
    let bad_id = (
        "statement-not-a-uuid-and-padded-to-46-chars-xx".to_string(),
        file.1.clone(),
    );
    assert_eq!(
        code(&tie_out(&server, &bad_id, "after_post").await).as_deref(),
        Some("argument_invalid:proposals_id")
    );
}

#[tokio::test]
async fn open_cash_questions_do_not_stop_a_tie_out() {
    // The tie-out needs only the bank ledger, the controls, the window and the
    // vouchers; a file build_import_xml would refuse for an open cash line is
    // still read.
    let directory = tempfile::tempdir().unwrap();
    let mut document = proposals(BANK, None);
    document["records"] = json!([{"disposition": "needs_answer", "party": "ATM CASH"}]);
    let file = publish(directory.path(), &document);
    let response = tie_out(&server(directory.path(), None), &file, "after_post").await;
    assert_eq!(response["isError"], false, "{response}");
    assert_eq!(
        figures(&response)[0].2.as_deref(),
        Some("window_not_recorded")
    );
}

#[tokio::test]
async fn a_failed_catalogue_read_is_the_read_failure_ledger_movement_gives() {
    let mut plans = company_check();
    plans.extend(ledger_read(&plan("<ENVELOPE></ENVELOPE>".to_string())));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let file = publish(
        directory.path(),
        &proposals(BANK, Some(whole("2026-08-01", "2026-08-07"))),
    );
    let response = tie_out(
        &server(directory.path(), Some(simulator.address())),
        &file,
        "after_post",
    )
    .await;
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "ledger_movement_read_failed",
        "{response}"
    );
}
