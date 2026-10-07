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
        rows_without_voucher: 0,
    }
}

fn present(opening: &str) -> LedgerRead {
    LedgerRead::Present(BankSide::from_book(opening).unwrap())
}

/// The book at the start holds 1500.00 in the bank (a debit, so -1500.00 in
/// Tally's sign) and at the end 2000.00 (-2000.00).
fn observed() -> [LedgerRead; 3] {
    [
        present("-1500.00"),
        present("-2000.00"),
        present("-1500.00"),
    ]
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
    let differing = gaps(Stage::AfterPost, &file(), &observed(), false).unwrap();
    assert_eq!(differing.opening.amount(), Some("500.00".to_string()));
    assert_eq!(differing.opening.state(), "differs");
}

#[test]
fn a_gap_of_exactly_zero_is_tied_whatever_its_spelling() {
    let observed = [present("-1000"), present("-2150.000"), present("-1000.00")];
    let tied = gaps(Stage::AfterPost, &file(), &observed, false).unwrap();
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
    let after = gaps(Stage::AfterPost, &file(), &observed(), false).unwrap();
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
    let before = gaps(Stage::BeforeBuild, &file(), &observed(), false).unwrap();
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
    let after = gaps(Stage::AfterPost, &file(), &observed(), false).unwrap();
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
        "records": [],
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
fn rows_without_a_voucher_are_the_open_cash_lines_and_the_skipped_rows() {
    let mut with = document(vec![]);
    with["records"] = json!([
        {"disposition": "needs_answer"},
        {"disposition": "skipped"},
        {"disposition": {"voucher": "Contra"}},
        {"disposition": {"voucher": "Payment"}},
    ]);
    assert_eq!(StatementFile::read(&with).unwrap().rows_without_voucher, 2);
    assert_eq!(
        StatementFile::read(&document(vec![]))
            .unwrap()
            .rows_without_voucher,
        0
    );
}

#[test]
fn a_row_counts_as_carried_only_when_the_file_says_it_is_a_voucher() {
    use bridge_bank_statement::proposals::{Disposition, VoucherType};
    // the records as the parse writes them: the serialiser's own spelling
    let mut written = document(vec![]);
    written["records"] = json!([
        {"disposition": Disposition::NeedsAnswer},
        {"disposition": Disposition::Skipped},
        {"disposition": Disposition::Voucher(VoucherType::Contra)},
        {"disposition": Disposition::Voucher(VoucherType::Payment)},
        {"disposition": Disposition::Voucher(VoucherType::Receipt)},
    ]);
    assert_eq!(
        StatementFile::read(&written).unwrap().rows_without_voucher,
        2
    );
    // a disposition this check has never heard of is not a voucher either
    written["records"] = json!([{"disposition": "held_for_review"}, {"disposition": null}]);
    assert_eq!(
        StatementFile::read(&written).unwrap().rows_without_voucher,
        2
    );
}

#[test]
fn a_negative_amount_on_the_bank_ledger_is_refused_by_the_reader() {
    for side in ["Dr", "Cr"] {
        let refused = StatementFile::read(&document(vec![voucher(&[(
            "Synthetic Bank Ledger",
            "-1.00",
            side,
        )])]))
        .err();
        assert_eq!(
            refused,
            Some("proposals_file_invalid".to_string()),
            "{side}"
        );
    }
    // a negative amount on another ledger is never read
    assert!(StatementFile::read(&document(vec![voucher(&[("Rent", "-1.00", "Dr")])])).is_ok());
}

#[test]
fn a_statement_of_one_day_is_a_whole_window_and_a_reversed_one_is_refused() {
    let mut one_day = document(vec![]);
    one_day["window"] = json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-01", "whole_statement": true});
    assert_eq!(
        StatementFile::read(&one_day).unwrap().window,
        Ok((date(2026, 8, 1), date(2026, 8, 1)))
    );
    let mut reversed = document(vec![]);
    reversed["window"] = json!({"first_row_date": "2026-08-02", "last_row_date": "2026-08-01", "whole_statement": true});
    assert_eq!(
        StatementFile::read(&reversed).err(),
        Some("proposals_file_invalid".to_string())
    );
}

#[test]
fn only_an_exact_bank_ledger_name_counts() {
    let vouchers = vec![voucher(&[
        ("synthetic bank ledger", "300.00", "Dr"),
        ("Synthetic Bank Ledger ", "120.00", "Cr"),
    ])];
    // neither spelling is the bank ledger, so neither counts; a reader that
    // folded case or trimmed would net 180.00
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
    // A statement figure that is not a decimal.
    let mut bad = document(vec![]);
    bad["controls"]["opening_balance"] = json!("one thousand");
    assert_eq!(refused(bad), invalid);
    let mut bad = document(vec![]);
    bad["controls"]["closing_balance"] = Value::Null;
    assert_eq!(refused(bad), invalid);
    // No records array: the rows without a voucher cannot be counted.
    let mut bad = document(vec![]);
    bad.as_object_mut().unwrap().remove("records");
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
fn a_bank_ledger_in_some_reads_and_not_others_means_the_book_changed() {
    let mut changed = observed();
    changed[2] = LedgerRead::Absent;
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed, false).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
    let mut changed = observed();
    changed[1] = LedgerRead::Absent;
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed, false).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
    let mut changed = observed();
    changed[0] = LedgerRead::Absent;
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed, false).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
}

#[test]
fn an_opening_that_moved_between_the_two_reads_at_the_first_date_means_the_book_changed() {
    let mut changed = observed();
    changed[2] = present("-1500.01");
    assert_eq!(
        gaps(Stage::AfterPost, &file(), &changed, false).unwrap(),
        all(NotEstablished::BookChangedDuringRead)
    );
    // The same figure spelled with another scale is not a change.
    let mut same = observed();
    same[2] = present("-1500");
    assert_eq!(
        amounts(&gaps(Stage::AfterPost, &file(), &same, false).unwrap())[0],
        Some("500.00".into())
    );
}

#[test]
fn rows_without_a_voucher_or_a_partly_posted_file_leave_only_the_figures_that_count_the_file() {
    let mut without = file();
    without.rows_without_voucher = 2;
    let before = gaps(Stage::BeforeBuild, &without, &observed(), false).unwrap();
    assert_eq!(before.opening.amount(), Some("500.00".to_string()));
    assert_eq!(
        before.closing,
        Figure::NotEstablished(NotEstablished::RowsWithoutVoucher)
    );
    assert_eq!(
        before.change,
        Figure::NotEstablished(NotEstablished::RowsWithoutVoucher)
    );
    // The same file, after a post: the book as it stands is all that counts.
    let after = gaps(Stage::AfterPost, &without, &observed(), false).unwrap();
    assert_eq!(
        amounts(&after),
        [
            Some("500.00".into()),
            Some("-150.00".into()),
            Some("-650.00".into())
        ]
    );

    let posted = gaps(Stage::BeforeBuild, &file(), &observed(), true).unwrap();
    assert_eq!(posted.opening.amount(), Some("500.00".to_string()));
    assert_eq!(
        posted.closing,
        Figure::NotEstablished(NotEstablished::FileAlreadyPartlyPosted)
    );
    assert_eq!(posted.closing, posted.change);
    // Rows without a voucher are said first when both hold.
    let both = gaps(Stage::BeforeBuild, &without, &observed(), true).unwrap();
    assert_eq!(
        both.closing,
        Figure::NotEstablished(NotEstablished::RowsWithoutVoucher)
    );
    let after = gaps(Stage::AfterPost, &file(), &observed(), true).unwrap();
    assert_eq!(after.closing.amount(), Some("-150.00".to_string()));
}

#[test]
fn a_catalogue_opening_that_is_missing_or_not_a_decimal_is_an_error_not_a_figure() {
    let ledger = |opening: Option<&str>| TallyLedger {
        name: "Bank".to_string(),
        parent: Default::default(),
        party_gstin: Default::default(),
        opening_balance: opening.map(str::to_string),
    };
    assert_eq!(
        LedgerRead::of(&[ledger(None)], "Bank").err(),
        Some("ledger_opening_missing".to_string())
    );
    assert_eq!(
        LedgerRead::of(&[ledger(Some("1,500.00 Dr"))], "Bank").err(),
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
        Some("ledger_name_duplicated_in_catalogue".to_string())
    );
}

// ---- the reading text --------------------------------------------------------

const CAUSES: &str = "Uncleared cheques and deposits in transit explain differences like these. So can a missing entry, a repeated entry or the wrong bank ledger. This check cannot tell them apart. To see which vouchers are involved, read the bank ledger's vouchers for these dates.";
const SCOPE: &str =
    "This checks the bank ledger only, and takes the ledger named in the file to be a bank account: an income or expense ledger opens at zero for the period, so its gaps would mean nothing. A wrong party or expense ledger is not caught here.";

fn window() -> Option<(Date, Date)> {
    Some((date(2026, 8, 1), date(2026, 8, 7)))
}

#[test]
fn the_fixed_sentences_are_exactly_these() {
    let differing = gaps(Stage::AfterPost, &file(), &observed(), false).unwrap();
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
    let differing = gaps(Stage::AfterPost, &file(), &observed(), false).unwrap();
    let text = reading(Stage::AfterPost, window(), &differing);
    assert_eq!(
        text["headline"],
        "Bank ledger against the statement, 2026-08-01 to 2026-08-07: opening gap 500.00; closing gap -150.00. A positive amount means the book shows more money in the bank than the statement does; a negative amount, less. Within these dates the book moved 650.00 further down than the statement did."
    );
}

#[test]
fn the_headline_leaves_out_a_figure_that_is_zero_and_says_so_when_all_are() {
    // Opening ties, the closing does not: the change equals the closing gap.
    let observed = [
        present("-1000.00"),
        present("-2000.00"),
        present("-1000.00"),
    ];
    let one = gaps(Stage::AfterPost, &file(), &observed, false).unwrap();
    let text = reading(Stage::AfterPost, window(), &one);
    assert_eq!(
        text["headline"],
        "Bank ledger against the statement, 2026-08-01 to 2026-08-07: closing gap -150.00. A positive amount means the book shows more money in the bank than the statement does; a negative amount, less. Within these dates the book moved 150.00 further down than the statement did."
    );
    let tied = [
        present("-1000.00"),
        present("-2150.00"),
        present("-1000.00"),
    ];
    let none = gaps(Stage::AfterPost, &file(), &tied, false).unwrap();
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
fn the_headline_lists_one_missing_figure_without_an_and() {
    let one_missing = Gaps {
        opening: Figure::Gap(
            BankSide::canonical(ExactDecimal::parse("0.00".to_string()).unwrap()).unwrap(),
        ),
        closing: Figure::NotEstablished(NotEstablished::BankLedgerNotInBook),
        change: Figure::Gap(
            BankSide::canonical(ExactDecimal::parse("0.00".to_string()).unwrap()).unwrap(),
        ),
    };
    let text = reading(Stage::AfterPost, window(), &one_missing);
    let headline = text["headline"].as_str().unwrap();
    assert!(
        headline.contains(": closing gap not established ("),
        "{headline}"
    );
    assert!(!headline.contains(" and closing gap"), "{headline}");
}

#[test]
fn the_headline_names_what_is_not_established_before_any_gap_that_differs() {
    let mut without = file();
    without.rows_without_voucher = 3;
    let mixed = gaps(Stage::BeforeBuild, &without, &observed(), false).unwrap();
    let text = reading(Stage::BeforeBuild, window(), &mixed);
    assert_eq!(
        text["headline"],
        "Bank ledger against the statement, 2026-08-01 to 2026-08-07: closing gap and change in window not established (some rows of the statement have no voucher in this file, so what the book would hold once it is imported is not known; use stage after_post once what is wanted is in the book); opening gap 500.00. "
            .to_string()
            + "A positive amount means the book shows more money in the bank than the statement does; a negative amount, less."
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

/// By direction: the book's movement against the statement's, up or down,
/// whether the money moved in or out.
#[test]
fn the_change_sentence_says_further_up_or_further_down_whichever_way_the_money_moved() {
    let sentence = |file: &StatementFile, reads: &[LedgerRead; 3]| {
        let text = reading(
            Stage::AfterPost,
            window(),
            &gaps(Stage::AfterPost, file, reads, false).unwrap(),
        );
        text["headline"].as_str().unwrap().to_string()
    };
    // Money in: statement 1000 to 2150, book 1500 to 2750. The book rose 1250
    // against the statement's 1150: 100.00 further up.
    let more = [
        present("-1500.00"),
        present("-2750.00"),
        present("-1500.00"),
    ];
    assert!(sentence(&file(), &more)
        .ends_with("Within these dates the book moved 100.00 further up than the statement did."));
    // Money out: statement 1000 to 800 (fell 200), book 700 to 600 (fell 100).
    // The book fell less, so it moved 100.00 further up, though its closing gap
    // (-200.00) is below zero.
    let mut out = file();
    out.closing = BankSide::from_printed("800.00").unwrap();
    out.proposals_net = BankSide::from_printed("0").unwrap();
    let fell = [present("-700.00"), present("-600.00"), present("-700.00")];
    let text = sentence(&out, &fell);
    assert!(text.contains("closing gap -200.00"), "{text}");
    assert!(text
        .ends_with("Within these dates the book moved 100.00 further up than the statement did."));
    // Statement 1000 to 800, book 1000 to 700: the book fell 300 against 200: 100.00 further down.
    let further = [present("-1000.00"), present("-700.00"), present("-1000.00")];
    assert!(sentence(&out, &further).ends_with(
        "Within these dates the book moved 100.00 further down than the statement did."
    ));
}

#[test]
fn the_text_never_says_what_it_must_not() {
    let figures = [
        gaps(Stage::AfterPost, &file(), &observed(), false).unwrap(),
        all(NotEstablished::WindowNotRecorded),
        all(NotEstablished::WindowNarrowerThanStatement),
        all(NotEstablished::WindowPrecedesBooksFrom),
        all(NotEstablished::BankLedgerNotInBook),
        all(NotEstablished::BookChangedDuringRead),
        all(NotEstablished::RowsWithoutVoucher),
        all(NotEstablished::FileAlreadyPartlyPosted),
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
    let differing = gaps(Stage::BeforeBuild, &file(), &observed(), false).unwrap();
    let result = result_json(Stage::BeforeBuild, window(), 0, &differing);
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
            "rows_without_voucher",
            "stage",
            "window"
        ]
    );
    assert_eq!(result["rows_without_voucher"], 0);
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
        0,
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
// in for the bank ledger. (The captured catalogues hold no ledger under Bank
// Accounts; the tie-out does not look at groups, so Cash serves.) The captures
// stay byte-exact on disk; no live behaviour of Tally is claimed by these
// tests. Their proposals files are written by hand, except in the round-trip
// test, whose file is written by the code `parse_bank_statement` uses.

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
enum Bank {
    Opening(String),
    NoOpening,
    Renamed,
}

fn catalogue(bank: Bank) -> ScenarioPlan {
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
            {"bridge_txn_id": "st-20260802-0000000000000001", "date": "2026-08-02",
             "voucher_type": "Receipt", "narration": "in",
             "entries": [
                {"ledger": bank_ledger, "amount": "300.00", "side": "Dr"},
                {"ledger": "Customer", "amount": "300.00", "side": "Cr"}]},
            {"bridge_txn_id": "st-20260803-0000000000000002", "date": "2026-08-03",
             "voucher_type": "Payment", "narration": "out",
             "entries": [
                {"ledger": "Supplier", "amount": "120.50", "side": "Dr"},
                {"ledger": bank_ledger, "amount": "120.50", "side": "Cr"}]},
        ],
        "records": [],
    });
    if let Some(window) = window {
        document["window"] = window;
    }
    document
}

fn whole(first: &str, last: &str) -> Value {
    json!({"first_row_date": first, "last_row_date": last, "whole_statement": true})
}

async fn tie_out_for(
    guid: &str,
    server: &Server,
    (proposals_id, digest): &(String, String),
    stage: &str,
) -> Value {
    server
        .call_tool(
            "statement_tie_out",
            json!({
                "company_guid": guid,
                "proposals_id": proposals_id,
                "proposals_sha256": digest,
                "stage": stage,
            }),
        )
        .await
}

async fn tie_out(server: &Server, file: &(String, String), stage: &str) -> Value {
    tie_out_for(COMPANY_GUID, server, file, stage).await
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

/// The `SVFROMDATE` each ledger-catalogue request carried, one per read (a
/// read sends its request twice, to compare the two answers).
fn ledger_read_dates(requests: &[tally_protocol_simulator::ObservedRequest]) -> Vec<String> {
    let mut dates = requests
        .iter()
        .filter_map(|request| {
            let body = tally_protocol_simulator::decode(&request.request_body).ok()?;
            if !(body.contains("List of Ledgers") && body.contains("OPENINGBALANCE")) {
                return None;
            }
            let from = body.split("<SVFROMDATE TYPE=\"Date\">").nth(1)?;
            Some(from.split('<').next()?.to_string())
        })
        .collect::<Vec<_>>();
    dates.dedup();
    dates
}

/// Three catalogue reads, in the order they are made, over a book whose bank
/// ledger shows these three states, for the file `document`.
async fn reads_of(
    document: &Value,
    bank: [Bank; 3],
    stage: &str,
) -> (Value, Vec<tally_protocol_simulator::ObservedRequest>) {
    let mut plans = company_check();
    for state in bank {
        plans.extend(ledger_read(&catalogue(state)));
    }
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let file = publish(directory.path(), document);
    let response = tie_out(
        &server(directory.path(), Some(simulator.address())),
        &file,
        stage,
    )
    .await;
    (response, simulator.finish().unwrap())
}

async fn three_reads(
    bank: [Bank; 3],
    bank_ledger: &str,
    window: Value,
    stage: &str,
) -> (Value, Vec<tally_protocol_simulator::ObservedRequest>) {
    reads_of(&proposals(bank_ledger, Some(window)), bank, stage).await
}

const ALL_READS: usize = 4 + 3 * 22;

fn book(first: &str, second: &str, third: &str) -> [Bank; 3] {
    [first, second, third].map(|opening| Bank::Opening(opening.to_string()))
}

#[tokio::test]
async fn a_book_that_differs_gives_the_gaps_with_their_sign_and_reads_the_three_dates() {
    // Start 1500.00 against the statement's 1000.00; end 2000.00 against 2150.00.
    let (response, requests) = three_reads(
        book("-1500.00", "-2000.00", "-1500.00"),
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
    assert_eq!(requests.len(), ALL_READS);
    // On the wire: the first row's date, the day after the last, the first again.
    assert_eq!(
        ledger_read_dates(&requests),
        ["20260801", "20260808", "20260801"]
    );
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
async fn a_book_that_matches_the_statement_at_both_ends_is_tied() {
    let (response, _) = three_reads(
        book("-1000.00", "-2150.00", "-1000"),
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

/// The company's books begin on 2026-04-01: a statement whose first row is
/// dated that very day is inside the books, not before them.
#[tokio::test]
async fn a_statement_whose_first_row_is_the_books_first_day_is_read_not_refused() {
    let (response, requests) = three_reads(
        book("-1000.00", "-2150.00", "-1000.00"),
        BANK,
        whole("2026-04-01", "2026-04-07"),
        "after_post",
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    assert_eq!(
        figures(&response),
        [
            amount("tied", "0.00"),
            amount("tied", "0.00"),
            amount("tied", "0.00")
        ]
    );
    assert_eq!(requests.len(), ALL_READS);
    assert_eq!(
        ledger_read_dates(&requests),
        ["20260401", "20260408", "20260401"]
    );
}

/// The journal check before a build takes the shared import-admission lock; a
/// build or post holding it exclusively makes the tie-out refuse as busy,
/// before any ledger is read (the simulator holds only the company check).
#[tokio::test]
async fn an_import_holding_the_admission_lock_makes_a_before_build_tie_out_refuse_as_busy() {
    let simulator = SequenceSimulator::spawn(company_check()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), Some(simulator.address()));
    let file = publish(
        directory.path(),
        &proposals(BANK, Some(whole("2026-08-01", "2026-08-07"))),
    );
    let admission = server.lock_import_admission().unwrap();
    let busy = tie_out(&server, &file, "before_build").await;
    drop(admission);
    assert_eq!(busy["isError"], true, "{busy}");
    assert_eq!(
        busy["structuredContent"]["result"]["error"]["code"],
        "import_admission_busy"
    );
    drop(simulator);
}

#[tokio::test]
async fn a_bank_ledger_the_book_does_not_have_is_not_established_not_an_error() {
    let (response, requests) = three_reads(
        book("-1500.00", "-2000.00", "-1500.00"),
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
    assert_eq!(requests.len(), ALL_READS);
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
        book("-1500.00", "-2000.00", "-1600.00"),
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
            Bank::Opening("-1500.00".into()),
            Bank::Opening("-2000.00".into()),
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

/// The native ledger reader refuses a catalogue row with no opening (the whole
/// read fails closed: a missing balance is never read as zero), so the tie-out
/// never meets one.
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

/// The company is verified before any result, a file with no whole window
/// included; the file alone then answers and no ledger is read.
#[tokio::test]
async fn a_file_without_a_whole_window_is_answered_from_the_file_after_the_company_is_verified() {
    for (window, reason) in [
        (None, "window_not_recorded"),
        (
            Some(
                json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-07", "whole_statement": false}),
            ),
            "window_narrower_than_statement",
        ),
    ] {
        let simulator = SequenceSimulator::spawn(company_check()).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let file = publish(directory.path(), &proposals(BANK, window));
        let response = tie_out(
            &server(directory.path(), Some(simulator.address())),
            &file,
            "before_build",
        )
        .await;
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
        assert_eq!(
            response["structuredContent"]["company"]["guid"],
            COMPANY_GUID
        );
        assert_eq!(simulator.finish().unwrap().len(), 4);
    }
}

#[tokio::test]
async fn a_company_that_is_not_loaded_is_refused_whatever_the_file_says() {
    for window in [None, Some(whole("2026-08-01", "2026-08-07"))] {
        let simulator = SequenceSimulator::spawn(company_check()).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let file = publish(directory.path(), &proposals(BANK, window));
        let response = tie_out_for(
            "00000000-0000-4000-8000-000000000000",
            &server(directory.path(), Some(simulator.address())),
            &file,
            "after_post",
        )
        .await;
        assert_eq!(response["isError"], true, "{response}");
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"],
            "company_identity_not_found"
        );
    }
}

/// The loader has its own tests (`agent_import_bank_tests.rs`); one refusal
/// here shows the tool goes through it.
#[tokio::test]
async fn a_proposals_file_changed_since_the_parse_and_a_stage_that_is_neither_are_refused() {
    let directory = tempfile::tempdir().unwrap();
    let file = publish(
        directory.path(),
        &proposals(BANK, Some(whole("2026-08-01", "2026-08-07"))),
    );
    let server = server(directory.path(), None);
    let code = |response: Value| response["structuredContent"]["result"]["error"]["code"].clone();
    let changed = (file.0.clone(), "0".repeat(64));
    assert_eq!(
        code(tie_out(&server, &changed, "after_post").await),
        "proposals_changed"
    );
    assert_eq!(
        code(tie_out(&server, &file, "later").await),
        "argument_invalid:stage"
    );
    // The handler's own refusal, under the schema's: a stage that is not one.
    let direct = server
        .statement_tie_out(&json!({
            "company_guid": COMPANY_GUID, "proposals_id": file.0, "proposals_sha256": file.1,
            "stage": "later",
        }))
        .await;
    assert_eq!(direct.err().unwrap().code, "argument_invalid:stage");
}

/// Rows with no voucher (a cash line not yet answered, a row skipped because
/// another account carries it) stop `before_build` from saying what the book
/// would hold, and do not touch `after_post`. A file `build_import_xml` would
/// refuse for an open cash line is still read.
#[tokio::test]
async fn open_cash_lines_and_skipped_rows_stop_only_the_projection() {
    let mut document = proposals(BANK, Some(whole("2026-08-01", "2026-08-07")));
    document["records"] = json!([
        {"disposition": "needs_answer", "party": "ATM CASH"},
        {"disposition": "skipped", "party": "OWN ACCOUNT"},
        {"disposition": {"voucher": "Payment"}, "party": "SUPPLIER"},
    ]);
    let (after, requests) = reads_of(
        &document,
        book("-1500.00", "-2000.00", "-1500.00"),
        "after_post",
    )
    .await;
    assert_eq!(after["isError"], false, "{after}");
    assert_eq!(
        figures(&after),
        [
            amount("differs", "500.00"),
            amount("differs", "-150.00"),
            amount("differs", "-650.00")
        ]
    );
    assert_eq!(requests.len(), ALL_READS);
    assert_eq!(
        after["structuredContent"]["result"]["rows_without_voucher"],
        2
    );
    let (before, _) = reads_of(
        &document,
        book("-1500.00", "-2000.00", "-1500.00"),
        "before_build",
    )
    .await;
    assert_eq!(
        figures(&before),
        [
            amount("differs", "500.00"),
            unestablished("rows_without_voucher"),
            unestablished("rows_without_voucher"),
        ]
    );
    assert_eq!(
        before["structuredContent"]["result"]["rows_without_voucher"],
        2
    );
    // the opening gap is established and the closing is not: the evidence is
    // partial and names the first reason
    assert_eq!(before["structuredContent"]["evidence"]["state"], "partial");
    assert_eq!(
        before["structuredContent"]["evidence"]["reason_code"],
        "rows_without_voucher"
    );
}

/// The import journal is the build's own: a batch of this company with a row of
/// the file, sent or found posted, makes `before_build` say the file may
/// already be in the book. `after_post` is the book as it stands.
#[tokio::test]
async fn a_file_a_batch_has_already_posted_is_not_projected_again() {
    let document = proposals(BANK, Some(whole("2026-08-01", "2026-08-07")));
    let book_states = || book("-1500.00", "-2000.00", "-1500.00");
    let run = |stage: &'static str, journal: bool| {
        let document = document.clone();
        async move {
            let mut plans = company_check();
            for state in book_states() {
                plans.extend(ledger_read(&catalogue(state)));
            }
            let simulator = SequenceSimulator::spawn(plans).unwrap();
            let directory = tempfile::tempdir().unwrap();
            let server = server(directory.path(), Some(simulator.address()));
            if journal {
                server
                    .append_import_record_while_admitted(&json!({
                        "batch_id": "bridge-2b1c9f4e-9d3a-4f71-8c2e-5a6b7c8d9e01",
                        "identity_scheme": "batch_v1",
                        "company_guid": COMPANY_GUID,
                        "endpoint_origin": "http://127.0.0.1:9",
                        "company": {"name": "WR2 Unicode Lab", "guid": COMPANY_GUID,
                                    "company_number": "1", "books_from": "20260401"},
                        "txn_ids": ["st-20260802-0000000000000001", "st-20260803-0000000000000002"],
                        "date_from": "20260802", "date_to": "20260803",
                        "sha256": "a".repeat(64), "built_at": "2026-09-16T00:00:00Z",
                        "status": "posted_verified",
                        "pre_import_mark": {"kind": "company_high_water", "value": 1, "master_value": 1},
                        "vouchers": document["vouchers"],
                    }))
                    .unwrap();
            }
            let file = publish(directory.path(), &document);
            tie_out(&server, &file, stage).await
        }
    };
    let clean = run("before_build", false).await;
    assert_eq!(figures(&clean)[1], amount("differs", "29.50"));
    let posted = run("before_build", true).await;
    assert_eq!(
        figures(&posted),
        [
            amount("differs", "500.00"),
            unestablished("file_already_partly_posted"),
            unestablished("file_already_partly_posted"),
        ]
    );
    let after = run("after_post", true).await;
    assert_eq!(figures(&after)[1], amount("differs", "-150.00"));
}

/// The file here is written by the code `parse_bank_statement` runs after a
/// parse (`persist`), from a built statement, and then read by the tool: the
/// shape the tool reads is the shape the parse writes.
#[tokio::test]
async fn a_file_the_parse_writes_is_read_by_the_tie_out() {
    let run = |stage: &'static str| async move {
        let mut plans = company_check();
        for state in book("-1000.00", "-970.00", "-1000.00") {
            plans.extend(ledger_read(&catalogue(state)));
        }
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let file = super::super::bank_statement::tests::persisted_synthetic_statement(
            directory.path(),
            BANK,
        );
        let response = tie_out(
            &server(directory.path(), Some(simulator.address())),
            &file,
            stage,
        )
        .await;
        (response, simulator.finish().unwrap())
    };
    // Rows dated 5, 9 and 1 August, three payments of 10.00 from the bank.
    let (after, requests) = run("after_post").await;
    assert_eq!(after["isError"], false, "{after}");
    assert_eq!(
        figures(&after),
        [
            amount("tied", "0.00"),
            amount("tied", "0.00"),
            amount("tied", "0.00")
        ]
    );
    assert_eq!(
        after["structuredContent"]["result"]["window"],
        json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-09"})
    );
    assert_eq!(
        ledger_read_dates(&requests),
        ["20260801", "20260810", "20260801"]
    );
    let (before, _) = run("before_build").await;
    // 970.00 + (-30.00) - 970.00.
    assert_eq!(
        figures(&before),
        [
            amount("tied", "0.00"),
            amount("differs", "-30.00"),
            amount("differs", "-30.00")
        ]
    );
    assert_eq!(
        before["structuredContent"]["result"]["rows_without_voucher"],
        0
    );
}
