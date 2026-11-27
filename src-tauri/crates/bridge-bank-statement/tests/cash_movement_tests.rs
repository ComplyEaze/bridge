//! A cash withdrawal or deposit becomes the entry a person's answer names, or,
//! on an explicit "don't know", a tagged suspense line; never a silent default.
//!
//! The narrations are synthetic twins of shapes captured from real statements:
//! the SBI `ATM WDL` rule and the Union Bank `BY CASH` rule each came from a
//! captured statement, and neither parser recognises any other cash wording.

mod common;

use bridge_bank_statement::bank::Bank;
use bridge_bank_statement::cash::{
    CashAnswer, CashAnswerRow, CashAnswers, CashMovement, CASH_DEPOSIT, CASH_LIMIT_FLAG,
    CASH_WITHDRAWAL, PURPOSE_NOT_CONFIRMED,
};
use bridge_bank_statement::mapping::{Mapping, MappingRow};
use bridge_bank_statement::parse::Row;
use bridge_bank_statement::proposals::{
    build, is_statement_txn_id, selfcheck, suspense_tagged, Build, BuildOptions, Disposition, Side,
    VoucherType, CASH_PAYMENT_ID_SUFFIX, SUSPENSE_TAGS, UNIDENTIFIED,
};
use common::*;

fn options(answers: &CashAnswers) -> BuildOptions<'_> {
    BuildOptions {
        bank_ledger: "Bank",
        suspense_ledger: "Suspense",
        account_label: "ACC xx1234",
        account_number: "00000000001234",
        date_from: None,
        date_to: None,
        cash_answers: answers,
    }
}

fn no_mapping() -> Mapping {
    Mapping::from_rows(std::iter::empty::<MappingRow>()).unwrap()
}

fn mapping(rows: &[(&str, &str, Option<&str>)]) -> Result<Mapping, bridge_bank_statement::Refusal> {
    Mapping::from_rows(
        rows.iter()
            .enumerate()
            .map(|(index, (party, ledger, treatment))| MappingRow {
                origin: format!("mapping record {}", index + 2),
                party: party.to_string(),
                ledger: ledger.to_string(),
                treatment: treatment.map(str::to_string),
            }),
    )
}

fn answers(
    rows: &[(&str, &str, Option<&str>)],
) -> Result<CashAnswers, bridge_bank_statement::Refusal> {
    CashAnswers::from_rows(
        rows.iter()
            .enumerate()
            .map(|(index, (id, answer, ledger))| CashAnswerRow {
                origin: format!("cash_answers[{index}]"),
                bridge_txn_id: id.to_string(),
                answer: answer.to_string(),
                ledger: ledger.map(str::to_string),
                cash_ledger: None,
            }),
    )
}

/// Answers with a cash ledger: (id, answer, ledger, cash_ledger).
fn answers_through_cash(
    rows: &[(&str, &str, Option<&str>, Option<&str>)],
) -> Result<CashAnswers, bridge_bank_statement::Refusal> {
    CashAnswers::from_rows(rows.iter().enumerate().map(
        |(index, (id, answer, ledger, cash_ledger))| CashAnswerRow {
            origin: format!("cash_answers[{index}]"),
            bridge_txn_id: id.to_string(),
            answer: answer.to_string(),
            ledger: ledger.map(str::to_string),
            cash_ledger: cash_ledger.map(str::to_string),
        },
    ))
}

/// `paid_to_someone` for one row: the payee and the cash ledger.
fn paid(id: &str, payee: &str, cash: &str) -> CashAnswers {
    answers_through_cash(&[(id, "paid_to_someone", Some(payee), Some(cash))]).unwrap()
}

/// An SBI row; SBI names parties from the spaced narration and reference.
fn sbi(date: &str, narration: &str, dr: &str, cr: &str, bal: &str) -> Row {
    row(&[
        ("date", date),
        ("narr", narration),
        ("narr_spaced", narration),
        ("ref", ""),
        ("ref_spaced", ""),
        ("dr", dr),
        ("cr", cr),
        ("bal", bal),
    ])
}

/// A Union Bank row; it names parties from the remarks.
fn ubi(date: &str, remarks: &str, dr: &str, cr: &str, bal: &str) -> Row {
    row(&[
        ("date", date),
        ("narr", remarks),
        ("narr_spaced", remarks),
        ("ref", "A123456"),
        ("dr", dr),
        ("cr", cr),
        ("bal", bal),
    ])
}

/// An ATM withdrawal and an ordinary receipt.
fn withdrawal_statement() -> Vec<Row> {
    vec![
        sbi(
            "01Aug2026",
            "ATM WDL ATM CASH 1234 SYNTHETIC BRANCH",
            "500.00",
            "",
            "9500.00",
        ),
        sbi(
            "02Aug2026",
            "BY TRANSFER-NEFT*SYNTHETIC SUPPLIER",
            "",
            "100.00",
            "9600.00",
        ),
    ]
}

fn deposit_statement() -> Vec<Row> {
    vec![ubi("03-08-2026", "BY CASH", "", "750.50", "12000.00")]
}

/// The `bridge_txn_id` of the first row, as an unanswered build reports it.
fn first_id(rows: &[Row], bank: Bank) -> String {
    build(rows, bank, &no_mapping(), &options(CashAnswers::none()))
        .unwrap()
        .records[0]
        .bridge_txn_id
        .clone()
}

fn legs(built: &Build) -> ((&str, Side), (&str, Side)) {
    let entries = &built.proposals[0].entries;
    (
        (entries[0].ledger.as_str(), entries[0].side),
        (entries[1].ledger.as_str(), entries[1].side),
    )
}

#[test]
fn only_the_two_captured_cash_parties_are_cash_movements() {
    assert_eq!(
        CashMovement::of_party(CASH_WITHDRAWAL),
        Some(CashMovement::Withdrawal)
    );
    assert_eq!(
        CashMovement::of_party(CASH_DEPOSIT),
        Some(CashMovement::Deposit)
    );
    assert_eq!(
        CashMovement::of_party("atm  cash withdrawal"),
        Some(CashMovement::Withdrawal),
        "the mapping key's fold"
    );
    // A self cheque can move money to another own account: not a cash movement.
    for party in [
        "SELF",
        "CASH",
        "SYNTHETIC SUPPLIER",
        "UNRESOLVED",
        "BY CASH DEPOSIT",
    ] {
        assert_eq!(CashMovement::of_party(party), None, "{party}");
    }
    assert_eq!(Bank::Sbi.party(&withdrawal_statement()[0]), CASH_WITHDRAWAL);
    assert_eq!(Bank::Ubi.party(&deposit_statement()[0]), CASH_DEPOSIT);
}

#[test]
fn an_unanswered_cash_line_stays_open_and_never_reaches_suspense() {
    for (rows, bank, movement) in [
        (withdrawal_statement(), Bank::Sbi, CashMovement::Withdrawal),
        (deposit_statement(), Bank::Ubi, CashMovement::Deposit),
    ] {
        let built = build(&rows, bank, &no_mapping(), &options(CashAnswers::none())).unwrap();
        let open = &built.records[0];
        assert_eq!(open.disposition, Disposition::NeedsAnswer);
        assert_eq!(open.cash_movement, Some(movement));
        assert_eq!(open.cash_answer, None);
        assert!(!open.suspense);
        assert!(open.ledger.is_empty());
        assert!(!open.bridge_txn_id.is_empty());
        // No voucher for it; the transcription check still holds for the rest.
        assert!(built
            .proposals
            .iter()
            .all(|proposal| proposal.bridge_txn_id != open.bridge_txn_id));
        assert_eq!(selfcheck(&built, "Bank").unwrap().vouchers, rows.len() - 1);
    }
}

#[test]
fn each_answer_maps_to_its_one_entry() {
    let withdrawal = withdrawal_statement();
    let id = first_id(&withdrawal, Bank::Sbi);
    for (answer, ledger, voucher_type, debit, credit) in [
        (
            "business_cash",
            Some("Cash"),
            VoucherType::Contra,
            "Cash",
            "Bank",
        ),
        (
            "owner_use",
            Some("Drawings"),
            VoucherType::Payment,
            "Drawings",
            "Bank",
        ),
        ("dont_know", None, VoucherType::Payment, "Suspense", "Bank"),
    ] {
        let given = answers(&[(&id, answer, ledger)]).unwrap();
        let built = build(&withdrawal, Bank::Sbi, &no_mapping(), &options(&given)).unwrap();
        assert_eq!(built.proposals[0].voucher_type, voucher_type, "{answer}");
        assert_eq!(
            legs(&built),
            ((debit, Side::Dr), (credit, Side::Cr)),
            "{answer}"
        );
        assert_eq!(built.proposals[0].entries[0].amount, "500.00");
        assert_eq!(
            built.records[0].disposition,
            Disposition::Voucher(voucher_type)
        );
        assert_eq!(built.records[0].cash_answer, CashAnswer::parse(answer));
        assert_eq!(built.records[0].suspense, answer == "dont_know", "{answer}");
    }
    let deposit = deposit_statement();
    let id = first_id(&deposit, Bank::Ubi);
    for (answer, ledger, credit) in [
        (
            "customer_paid_in",
            Some("Synthetic Customer"),
            "Synthetic Customer",
        ),
        ("owner_brought_in", Some("Capital"), "Capital"),
        ("dont_know", None, "Suspense"),
    ] {
        let given = answers(&[(&id, answer, ledger)]).unwrap();
        let built = build(&deposit, Bank::Ubi, &no_mapping(), &options(&given)).unwrap();
        assert_eq!(
            built.proposals[0].voucher_type,
            VoucherType::Receipt,
            "{answer}"
        );
        assert_eq!(
            legs(&built),
            (("Bank", Side::Dr), (credit, Side::Cr)),
            "{answer}"
        );
        assert_eq!(built.records[0].suspense, answer == "dont_know", "{answer}");
    }
}

/// Cash withdrawn from one of our banks and paid into another passes through
/// cash in hand: each statement's line posts once, on its own date, against
/// Cash. The pin is the refusal of the three answers that moved it bank to
/// bank, where truthful answers on both statements posted it twice; the rest
/// shows the answers that remain.
#[test]
fn cash_moved_between_our_own_banks_posts_once_on_each_side() {
    let withdrawal = withdrawal_statement();
    let withdrawn = first_id(&withdrawal, Bank::Sbi);
    let deposit = deposit_statement();
    let paid_in = first_id(&deposit, Bank::Ubi);
    // The answers that once moved it bank to bank are gone.
    for answer in ["other_own_bank", "from_other_own_bank", "already_recorded"] {
        refuses(
            answers(&[(&withdrawn, answer, None)]),
            "unknown_cash_answer",
        );
    }
    // The truthful answers: the withdrawal became business cash; the deposit
    // came from the cash box, which needs its cash ledger, and until the build
    // has projected that ledger a person unsure of it answers dont_know.
    let taken = answers(&[(&withdrawn, "business_cash", Some("Cash"))]).unwrap();
    let mut sbi = options(&taken);
    sbi.bank_ledger = "SBI CA";
    let from_sbi = build(&withdrawal, Bank::Sbi, &no_mapping(), &sbi).unwrap();
    let mut ubi = options(&taken);
    ubi.bank_ledger = "UBI SB";
    let unknown = answers(&[(&paid_in, "dont_know", None)]).unwrap();
    ubi.cash_answers = &unknown;
    let from_ubi = build(&deposit, Bank::Ubi, &no_mapping(), &ubi).unwrap();
    let cash_vouchers = from_sbi
        .proposals
        .iter()
        .filter(|proposal| proposal.bridge_txn_id == withdrawn)
        .chain(
            from_ubi
                .proposals
                .iter()
                .filter(|proposal| proposal.bridge_txn_id == paid_in),
        )
        .collect::<Vec<_>>();
    assert_eq!(cash_vouchers.len(), 2, "one voucher per statement line");
    for proposal in &cash_vouchers {
        let ledgers = proposal
            .entries
            .iter()
            .map(|entry| entry.ledger.as_str())
            .collect::<Vec<_>>();
        assert!(
            !(ledgers.contains(&"SBI CA") && ledgers.contains(&"UBI SB")),
            "no bank-to-bank voucher: {ledgers:?}"
        );
    }
    assert_eq!(legs(&from_sbi), (("Cash", Side::Dr), ("SBI CA", Side::Cr)));
    assert_eq!(from_sbi.proposals[0].date, "2026-08-01");
    assert_eq!(
        legs(&from_ubi),
        (("UBI SB", Side::Dr), ("Suspense", Side::Cr))
    );
    assert_eq!(from_ubi.proposals[0].date, "2026-08-03");
    assert!(from_ubi.records[0].suspense);
}

/// Only a tag at the end of a narration, as `build` writes it, marks a
/// suspense line: the same text in an account label or party does not.
#[test]
fn only_a_tag_where_build_writes_it_marks_a_suspense_line() {
    let rows = withdrawal_statement();
    let id = first_id(&rows, Bank::Sbi);
    let given = answers(&[(&id, "dont_know", None)]).unwrap();
    let built = build(&rows, Bank::Sbi, &no_mapping(), &options(&given)).unwrap();
    for (proposal, record) in built.proposals.iter().zip(&built.records) {
        let ledgers = proposal.entries.iter().map(|entry| entry.ledger.as_str());
        assert_eq!(
            suspense_tagged(&proposal.narration, ledgers),
            record.suspense,
            "{}",
            proposal.narration
        );
    }
    // A label carrying a tag's text marks nothing. (That statement holds no
    // cash line, so it takes no answers.)
    let mut labelled = options(CashAnswers::none());
    labelled.account_label = "Bridge: purpose not confirmed; reclassify";
    let mapped = mapping(&[("SYNTHETIC SUPPLIER", "Supplier", None)]).unwrap();
    let rows = [sbi(
        "02Aug2026",
        "BY TRANSFER-NEFT*SYNT0000001*N123456789*SYNTHETIC SUPPLIER",
        "",
        "100.00",
        "9600.00",
    )];
    let built = build(&rows, Bank::Sbi, &mapped, &labelled).unwrap();
    assert!(!built.records[0].suspense, "mapped, not suspense");
    let proposal = &built.proposals[0];
    assert_eq!(proposal.entries[1].ledger, "Supplier");
    assert!(proposal.narration.contains(PURPOSE_NOT_CONFIRMED));
    assert!(!suspense_tagged(
        &proposal.narration,
        proposal.entries.iter().map(|entry| entry.ledger.as_str())
    ));
    // The unidentified tag counts only with the voucher's own ledger after it.
    assert!(!suspense_tagged(
        "x | UNIDENTIFIED - reallocate from Suspense",
        ["Bank", "Cash"]
    ));
    assert!(suspense_tagged(
        "x | UNIDENTIFIED - reallocate from Suspense",
        ["Bank", "Suspense"]
    ));
}

/// A book that reads the ledger back in another case or spacing still matches the tag
/// written for it, by the same loose fold `build` decided "unidentified" with; a ledger
/// the tag does not name, or a blank one, never does.
#[test]
fn the_unidentified_tag_matches_its_ledger_under_the_builds_own_fold() {
    let tag = "x | UNIDENTIFIED - reallocate from Suspense A/c-2";
    assert!(suspense_tagged(tag, ["Bank", "SUSPENSE A/C 2"]));
    assert!(suspense_tagged(tag, ["suspense  a/c-2"]));
    assert!(!suspense_tagged(tag, ["Suspense A/c"]));
    assert!(!suspense_tagged(tag, ["Bank", "Cash"]));
    // A blank ledger name must not make the bare tag text match.
    assert!(!suspense_tagged(
        "x | UNIDENTIFIED - reallocate from",
        ["", "  ", "-", "--"]
    ));
}

/// Only an explicit "don't know" reaches suspense, and it says so in the
/// narration with its own tag, never the unidentified-party tag.
#[test]
fn dont_know_posts_to_suspense_tagged_for_the_ca() {
    assert_eq!(SUSPENSE_TAGS, [PURPOSE_NOT_CONFIRMED, UNIDENTIFIED]);
    let rows = withdrawal_statement();
    let id = first_id(&rows, Bank::Sbi);
    let given = answers(&[(&id, "dont_know", None)]).unwrap();
    let built = build(&rows, Bank::Sbi, &no_mapping(), &options(&given)).unwrap();
    let narration = &built.proposals[0].narration;
    assert!(
        narration.ends_with(&format!(" | {PURPOSE_NOT_CONFIRMED}")),
        "{narration}"
    );
    assert!(!narration.contains(UNIDENTIFIED), "{narration}");
    assert!(
        narration.contains(CASH_WITHDRAWAL),
        "names the party: {narration}"
    );
    // The unmapped ordinary party keeps the unidentified tag.
    let other = &built.proposals[1].narration;
    assert!(
        other.contains(&format!("{UNIDENTIFIED} Suspense")),
        "{other}"
    );
    assert!(!other.contains(PURPOSE_NOT_CONFIRMED), "{other}");
    assert!(built.records.iter().all(|record| record.suspense));

    // Any other answer that names the suspense ledger is refused.
    let named = answers(&[(&id, "owner_use", Some("suspense"))]).unwrap();
    refuses(
        build(&rows, Bank::Sbi, &no_mapping(), &options(&named)),
        "cash_answer_names_suspense",
    );
}

#[test]
fn an_answer_bridge_cannot_build_yet_is_refused_with_the_way_forward() {
    let deposit = deposit_statement();
    let id = first_id(&deposit, Bank::Ubi);
    let given = answers(&[(&id, "unbooked_cash_sales", None)]).unwrap();
    let refusal = refuses(
        build(&deposit, Bank::Ubi, &no_mapping(), &options(&given)),
        "cash_answer_not_built",
    );
    assert!(refusal.message.contains("dont_know"), "{refusal}");
}

/// Cash paid straight to someone passes through the cash book: the row's
/// Contra fills the named cash ledger from the bank, and a cash Payment on the
/// same day empties it to the payee. The Contra keeps the row's id, as every
/// voucher with the bank leg does; the Payment's id is the row's and a suffix,
/// which is not a statement row id, so it is never read as half of a transfer.
#[test]
fn cash_paid_to_someone_is_a_contra_into_cash_then_a_cash_payment() {
    let withdrawal = withdrawal_statement();
    let id = first_id(&withdrawal, Bank::Sbi);
    let given = paid(&id, "Synthetic Wages", "Cash");
    let built = build(&withdrawal, Bank::Sbi, &no_mapping(), &options(&given)).unwrap();
    let contra = &built.proposals[0];
    assert_eq!(contra.bridge_txn_id, id);
    assert_eq!(contra.voucher_type, VoucherType::Contra);
    assert_eq!(legs(&built), (("Cash", Side::Dr), ("Bank", Side::Cr)));
    let payment = &built.proposals[1];
    assert_eq!(
        payment.bridge_txn_id,
        format!("{id}{CASH_PAYMENT_ID_SUFFIX}")
    );
    assert!(!is_statement_txn_id(&payment.bridge_txn_id));
    assert_eq!(payment.voucher_type, VoucherType::Payment);
    assert_eq!(payment.date, contra.date);
    let payment_legs: Vec<_> = payment
        .entries
        .iter()
        .map(|entry| (entry.ledger.as_str(), entry.amount.as_str(), entry.side))
        .collect();
    assert_eq!(
        payment_legs,
        [
            ("Synthetic Wages", "500.00", Side::Dr),
            ("Cash", "500.00", Side::Cr)
        ]
    );
    assert!(
        payment
            .narration
            .starts_with("Cash paid to Synthetic Wages from Cash | "),
        "{}",
        payment.narration
    );
    assert!(!payment.narration.contains(CASH_LIMIT_FLAG));
    // One record for the row: its Contra, and the Payment it adds.
    let record = &built.records[0];
    assert_eq!(
        record.disposition,
        Disposition::Voucher(VoucherType::Contra)
    );
    assert_eq!(record.ledger, "Cash");
    assert_eq!(record.cash_answer, Some(CashAnswer::PaidToSomeone));
    assert!(!record.suspense);
    let paid_to = record.paid_to.as_ref().unwrap();
    assert_eq!(paid_to.ledger, "Synthetic Wages");
    assert_eq!(paid_to.bridge_txn_id, payment.bridge_txn_id);
    assert!(!paid_to.over_cash_limit);
    assert!(built.records[1].paid_to.is_none());
    // The receipt row is the third voucher; only the Contra moves the bank.
    let check = selfcheck(&built, "Bank").unwrap();
    assert_eq!(check.vouchers, 3);
    assert_eq!(check.bank_out.as_str(), "500");
    // A record without its Payment is a miscount.
    let mut short = built.clone();
    short.proposals.remove(1);
    refuses(selfcheck(&short, "Bank"), "manifest_count_mismatch");
}

#[test]
fn cash_paid_to_someone_needs_both_ledgers_and_no_other_answer_takes_a_cash_ledger() {
    refuses(
        answers_through_cash(&[("st-x", "paid_to_someone", Some("Payee"), None)]),
        "cash_answer_without_cash_ledger",
    );
    refuses(
        answers_through_cash(&[("st-x", "paid_to_someone", Some("Payee"), Some("  "))]),
        "cash_answer_without_cash_ledger",
    );
    refuses(
        answers_through_cash(&[("st-x", "paid_to_someone", None, Some("Cash"))]),
        "cash_answer_without_ledger",
    );
    for answer in [
        "business_cash",
        "owner_use",
        "own_cash_box",
        "customer_paid_in",
    ] {
        refuses(
            answers_through_cash(&[("st-x", answer, Some("Cash"), Some("Cash"))]),
            "cash_ledger_not_used",
        );
    }
    refuses(
        answers_through_cash(&[("st-x", "dont_know", None, Some("Cash"))]),
        "cash_ledger_not_used",
    );
    assert!(CashAnswer::PaidToSomeone.ledger_prompt().is_some());
}

#[test]
fn cash_paid_to_someone_refuses_a_payee_that_is_the_cash_the_bank_or_suspense() {
    let withdrawal = withdrawal_statement();
    let id = first_id(&withdrawal, Bank::Sbi);
    let run = |payee: &str, cash: &str| {
        build(
            &withdrawal,
            Bank::Sbi,
            &no_mapping(),
            &options(&paid(&id, payee, cash)),
        )
    };
    refuses(run("cash", "Cash"), "self_cancelling_voucher");
    refuses(run("Bank", "Cash"), "cash_payment_to_statement_bank");
    refuses(run("Suspense", "Cash"), "cash_answer_names_suspense");
    refuses(
        run("Synthetic Wages", "suspense"),
        "cash_answer_names_suspense",
    );
    refuses(run("Synthetic Wages", "Bank"), "self_cancelling_voucher");
}

/// SBI cash withdrawals, one per (date, amount), each with its own balance.
fn withdrawals(amounts: &[(&str, &str)]) -> Vec<Row> {
    // The running balance in paise, so no float touches an amount.
    let mut balance: i64 = 10_000_000;
    amounts
        .iter()
        .map(|(date, amount)| {
            balance -= amount.replace('.', "").parse::<i64>().unwrap();
            sbi(
                date,
                "ATM WDL ATM CASH 1234 SYNTHETIC BRANCH",
                amount,
                "",
                &format!("{}.{:02}", balance / 100, balance % 100),
            )
        })
        .collect()
}

/// Every withdrawal of `rows` paid to the payee its index picks.
fn flags(rows: &[Row], payees: &[&str]) -> Vec<bool> {
    let ids: Vec<String> = build(
        rows,
        Bank::Sbi,
        &no_mapping(),
        &options(CashAnswers::none()),
    )
    .unwrap()
    .records
    .iter()
    .map(|record| record.bridge_txn_id.clone())
    .collect();
    let given = answers_through_cash(
        &ids.iter()
            .zip(payees)
            .map(|(id, payee)| (id.as_str(), "paid_to_someone", Some(*payee), Some("Cash")))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let built = build(rows, Bank::Sbi, &no_mapping(), &options(&given)).unwrap();
    built
        .records
        .iter()
        .map(|record| {
            let paid_to = record.paid_to.as_ref().unwrap();
            let payment = built
                .proposals
                .iter()
                .find(|proposal| proposal.bridge_txn_id == paid_to.bridge_txn_id)
                .unwrap();
            assert_eq!(
                payment
                    .narration
                    .ends_with(&format!(" | {CASH_LIMIT_FLAG}")),
                paid_to.over_cash_limit,
                "{}",
                payment.narration
            );
            paid_to.over_cash_limit
        })
        .collect()
}

/// s.40A(3) counts the cash paid to one person in a day. The flag sums this
/// file's cash Payments by payee and day, and marks every Payment of a
/// payee-day over the limit; one exactly at the limit is not over it.
#[test]
fn cash_paid_to_one_payee_in_a_day_over_the_limit_is_flagged_for_the_ca() {
    let one_day = withdrawals(&[("01Aug2026", "6000.00"), ("01Aug2026", "5000.00")]);
    assert_eq!(flags(&one_day, &["Payee A", "payee  a"]), [true, true]);
    assert_eq!(flags(&one_day, &["Payee A", "Payee B"]), [false, false]);
    let two_days = withdrawals(&[("01Aug2026", "6000.00"), ("02Aug2026", "5000.00")]);
    assert_eq!(flags(&two_days, &["Payee A", "Payee A"]), [false, false]);
    let at_limit = withdrawals(&[("01Aug2026", "4000.00"), ("01Aug2026", "6000.00")]);
    assert_eq!(flags(&at_limit, &["Payee A", "Payee A"]), [false, false]);
    let one_over = withdrawals(&[("01Aug2026", "10000.01")]);
    assert_eq!(flags(&one_over, &["Payee A"]), [true]);
}

#[test]
fn a_deposit_from_the_cash_box_is_a_contra_out_of_the_named_cash_ledger() {
    refuses(
        answers(&[("st-x", "own_cash_box", None)]),
        "cash_answer_without_ledger",
    );
    let deposit = deposit_statement();
    let id = first_id(&deposit, Bank::Ubi);
    let given = answers(&[(&id, "own_cash_box", Some("Petty Cash"))]).unwrap();
    let built = build(&deposit, Bank::Ubi, &no_mapping(), &options(&given)).unwrap();
    let voucher = built
        .proposals
        .iter()
        .find(|proposal| proposal.bridge_txn_id == id)
        .expect("the answered line's voucher");
    assert_eq!(voucher.voucher_type, VoucherType::Contra);
    let cash = voucher
        .entries
        .iter()
        .find(|entry| entry.ledger == "Petty Cash")
        .expect("the named ledger's leg");
    assert_eq!(cash.side, Side::Cr);
}

#[test]
fn an_answer_for_the_other_direction_is_refused() {
    let rows = withdrawal_statement();
    let id = first_id(&rows, Bank::Sbi);
    for (answer, ledger) in [
        ("owner_brought_in", Some("Capital")),
        ("own_cash_box", Some("Cash")),
    ] {
        let given = answers(&[(&id, answer, ledger)]).unwrap();
        refuses(
            build(&rows, Bank::Sbi, &no_mapping(), &options(&given)),
            "cash_answer_wrong_direction",
        );
    }
}

#[test]
fn answers_are_checked_when_they_are_parsed() {
    refuses(
        answers(&[("st-x", "contra", Some("Cash"))]),
        "unknown_cash_answer",
    );
    refuses(
        answers(&[("st-x", "owner_use", None)]),
        "cash_answer_without_ledger",
    );
    refuses(
        answers(&[("st-x", "owner_use", Some("  "))]),
        "cash_answer_without_ledger",
    );
    refuses(
        answers(&[("st-x", "dont_know", Some("Cash"))]),
        "cash_answer_ledger_not_used",
    );
    refuses(
        answers(&[
            ("st-x", "dont_know", None),
            ("st-x", "owner_use", Some("Drawings")),
        ]),
        "cash_answer_repeated",
    );
}

#[test]
fn an_answer_must_name_a_cash_line_of_this_statement() {
    let rows = withdrawal_statement();
    let unanswered = build(
        &rows,
        Bank::Sbi,
        &no_mapping(),
        &options(CashAnswers::none()),
    )
    .unwrap();
    let ordinary = &unanswered.records[1].bridge_txn_id;
    let given = answers(&[(ordinary, "dont_know", None)]).unwrap();
    refuses(
        build(&rows, Bank::Sbi, &no_mapping(), &options(&given)),
        "cash_answer_not_a_cash_line",
    );
    let stale = answers(&[("st-20260801-0000000000000000", "dont_know", None)]).unwrap();
    refuses(
        build(&rows, Bank::Sbi, &no_mapping(), &options(&stale)),
        "cash_answer_not_in_statement",
    );
}

/// An answer for a row the date window leaves out would post nothing and say
/// nothing, so it is refused.
#[test]
fn an_answer_for_a_row_outside_the_window_is_refused() {
    let rows = withdrawal_statement();
    let id = first_id(&rows, Bank::Sbi);
    let given = answers(&[(&id, "dont_know", None)]).unwrap();
    let mut windowed = options(&given);
    windowed.date_from = bridge_bank_statement::date::Date::parse_iso("2026-08-02");
    let refusal = refuses(
        build(&rows, Bank::Sbi, &no_mapping(), &windowed),
        "cash_answer_outside_window",
    );
    assert!(refusal.message.contains(&id), "{refusal}");
    // Inside the window it is used.
    let mut whole = options(&given);
    whole.date_from = bridge_bank_statement::date::Date::parse_iso("2026-08-01");
    assert!(build(&rows, Bank::Sbi, &no_mapping(), &whole).is_ok());
}

/// The same text covers business cash, drawings and a customer paying in, so
/// no per-party rule may decide it.
#[test]
fn a_cash_party_in_the_mapping_is_refused() {
    for treatment in [None, Some("auto"), Some("contra"), Some("skip")] {
        for party in [CASH_WITHDRAWAL, "cash deposit"] {
            let refusal = refuses(
                mapping(&[
                    ("SYNTHETIC SUPPLIER", "Supplier", None),
                    (party, "Cash", treatment),
                ]),
                "cash_party_in_mapping",
            );
            assert!(refusal.message.contains("mapping record 3"), "{refusal}");
            assert!(refusal.message.contains("cash_answers"), "{refusal}");
        }
    }
}

#[test]
fn cash_text_moving_the_other_way_is_refused() {
    let rows = [ubi("03-08-2026", "BY CASH", "750.50", "", "11249.50")];
    refuses(
        build(
            &rows,
            Bank::Ubi,
            &no_mapping(),
            &options(CashAnswers::none()),
        ),
        "cash_movement_direction_mismatch",
    );
}

#[test]
fn each_question_offers_every_answer_of_its_direction_and_none_other() {
    let (withdrawal, taken) = CashMovement::Withdrawal.question();
    let (deposit, given) = CashMovement::Deposit.question();
    assert!(withdrawal.contains("withdrawn") && deposit.contains("deposited"));
    assert_eq!(
        taken.len() + given.len(),
        9,
        "four and five, sharing only dont_know"
    );
    for answer in taken.iter().chain(given) {
        assert_eq!(CashAnswer::parse(answer.as_str()), Some(*answer));
        assert!(!answer.text().is_empty());
        // The words a person reads carry no accounting jargon.
        for jargon in ["Contra", "debit", "credit", "ledger"] {
            assert!(!answer.text().contains(jargon), "{answer:?}: {jargon}");
        }
    }
    let shared = taken
        .iter()
        .filter(|answer| given.contains(answer))
        .collect::<Vec<_>>();
    assert_eq!(shared, [&CashAnswer::DontKnow]);
    assert_eq!(taken.last(), Some(&CashAnswer::DontKnow));
    assert_eq!(given.last(), Some(&CashAnswer::DontKnow));
}

/// The answer ComplyEaze Bridge cannot build yet, with its whole reason, which reaches the assistant beside the answer and in the
/// `cash_answer_not_built` refusal (#962). Every reason any answer of either
/// movement gives names the product in full, never by its bare short name.
#[test]
fn every_not_built_reason_names_the_product_in_full() {
    assert_eq!(CashAnswer::PaidToSomeone.not_built(), None);
    assert_eq!(CashAnswer::OwnCashBox.not_built(), None);
    assert_eq!(
        CashAnswer::UnbookedCashSales.not_built(),
        Some("the cash sales or collections behind this deposit are not in the books, and ComplyEaze Bridge does not build those sales yet; answer dont_know and the line is built to the suspense ledger, marked for the CA to move")
    );
    for movement in [CashMovement::Withdrawal, CashMovement::Deposit] {
        for answer in movement.question().1 {
            let Some(reason) = answer.not_built() else {
                continue;
            };
            for (at, _) in reason.match_indices("Bridge") {
                assert!(
                    reason[..at].ends_with("ComplyEaze "),
                    "{answer:?} names a bare Bridge: {reason}"
                );
            }
        }
    }
}
