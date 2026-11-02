//! The projection against what Tally itself answered: one synthetic book's voucher
//! window and its Trial Balance to each of eleven dates, captured on TallyPrime 7.2.

use super::*;
use crate::agent::movement::{day_moves_of, parse_movement_vouchers};
use bridge_tally_protocol::native_trial_balance::parse_native_trial_balance;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, WireEncoding,
};

const COMPANY_GUID: &str = "5342454a-478e-4d59-a663-16ec7b8c84a8";
const WINDOW: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/cash-day-end/window-20250401-20250430.utf16le.xml"
);

macro_rules! trial_balance {
    ($to:literal) => {
        (
            $to,
            include_bytes!(concat!(
                "../crates/bridge-tally-protocol/tests/fixtures/agent/cash-day-end/trial-balance-to-",
                $to,
                ".utf16le.xml"
            ))
            .as_slice(),
        )
    };
}

const TRIAL_BALANCES: [(&str, &[u8]); 11] = [
    trial_balance!("20250402"),
    trial_balance!("20250403"),
    trial_balance!("20250404"),
    trial_balance!("20250405"),
    trial_balance!("20250407"),
    trial_balance!("20250408"),
    trial_balance!("20250409"),
    trial_balance!("20250410"),
    trial_balance!("20250411"),
    trial_balance!("20250425"),
    trial_balance!("20250430"),
];

fn text(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn date(value: &str) -> TallyDate {
    normalized_date(value).unwrap()
}

fn amount(value: &str) -> ExactDecimal {
    ExactDecimal::parse(value.to_string()).unwrap()
}

fn row(xml: &str, ledger: &str) -> NativeTrialBalanceRow {
    parse_native_trial_balance(xml, COMPANY_GUID)
        .unwrap()
        .rows
        .into_iter()
        .find(|row| row.name == ledger)
        .unwrap()
}

fn moves(to: &str, ledger: &str) -> BTreeMap<String, ExactDecimal> {
    let vouchers =
        parse_movement_vouchers(&text(WINDOW), "20250401", "20250430", COMPANY_GUID).unwrap();
    let mut moves = day_moves_of(&vouchers, ledger).unwrap();
    moves.retain(|day, _| day.as_str() <= to);
    moves
}

/// The book as the projection reads it to 30 April.
fn cash_book() -> BookDayEnds {
    let row = row(&text(TRIAL_BALANCES[10].1), "Cash");
    BookDayEnds::tied(
        "Cash",
        debit_positive("Cash", &row.opening).unwrap(),
        &moves("20250430", "Cash"),
        debit_positive("Cash", &row.closing).unwrap(),
    )
    .unwrap()
}

fn outflow(ledger: &str, day: &str, value: &str) -> CashOutflow {
    CashOutflow {
        ledger: ledger.to_string(),
        date: date(day),
        amount: amount(value),
    }
}

fn project(outflows: &[CashOutflow]) -> Result<EstablishedCashProjection, CashProjectionRefusal> {
    established(date("20250401"), date("20260331"), &[cash_book()], outflows)
}

/// A captured Trial Balance, seeded for the simulated company: `Cash` holding
/// `balance`, as Tally signs it, with zero stated on both sides.
fn seeded_trial_balance(company_guid: &str, balance: &str) -> String {
    let lab = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_known_lab.xml"
    )
    .replace("eebb9a9f-1679-4468-9e8f-814c729674cb", company_guid);
    let (before, rest) = lab.split_once("<LEDGER NAME=\"Cash\"").unwrap();
    let (row, after) = rest.split_once("</LEDGER>").unwrap();
    let row = row
        .replace(">-4950.00<", ">0.00<")
        .replace(
            "<CREDITTOTALS TYPE=\"Amount\"></CREDITTOTALS>",
            "<CREDITTOTALS TYPE=\"Amount\">0.00</CREDITTOTALS>",
        )
        .replace(">-7550.00<", &format!(">{balance}<"))
        .replace(">-2600.00<", &format!(">{balance}<"));
    format!("{before}<LEDGER NAME=\"Cash\"{row}</LEDGER>{after}")
}

/// What one Trial Balance read of the projection is answered with.
pub(in crate::agent::agent_import) fn cash_book_plans(
    company_guid: &str,
    company_number: &str,
    company: &str,
    balance: &str,
) -> Vec<ScenarioPlan> {
    let xml = |text: String| {
        ScenarioPlan::new(Fixture::SyntheticXml(text))
            .with_encoding(WireEncoding::Utf16Le)
            .with_framing(ResponseFraming::ContentLength)
    };
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
        .with_framing(ResponseFraming::ContentLength);
    let extent = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    );
    let extent = extent.replace(
        "<COMPANYNUMBER TYPE=\"Number\"> 100004<",
        &format!("<COMPANYNUMBER TYPE=\"Number\"> {company_number}<"),
    );
    let currency = text(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let mut plans = vec![
        status.clone(),
        xml(company.to_string()),
        xml(company.to_string()),
    ];
    for answer in [
        extent.clone(),
        currency,
        seeded_trial_balance(company_guid, balance),
        extent,
    ] {
        plans.extend([
            xml(answer.clone()),
            status.clone(),
            xml(answer),
            status.clone(),
        ]);
    }
    plans.extend([xml(company.to_string()), status, xml(company.to_string())]);
    plans
}

/// The record of `Cash` holding `balance`, as Tally signs it, with no movement.
pub(in crate::agent::agent_import) fn unmoved_book_record(balance: &str) -> CashProjectionRecord {
    let held = debit_positive("Cash", &NativeTrialBalanceAmount::Present(amount(balance))).unwrap();
    let book = BookDayEnds::tied("Cash", held.clone(), &BTreeMap::new(), held).unwrap();
    let (from, to) = (date("20260901"), date("20270331"));
    let book_sha256 = book_sha256(&from, &to, &[book]);
    CashProjectionRecord {
        ledgers: vec!["Cash".to_string()],
        from,
        to,
        book_sha256,
    }
}

#[test]
fn the_vouchers_reach_tallys_own_closing_at_every_captured_date() {
    for (to, bytes) in TRIAL_BALANCES {
        for ledger in ["Cash", "BRIDGE Petty Cash"] {
            let row = row(&text(bytes), ledger);
            let book = BookDayEnds::tied(
                ledger,
                debit_positive(ledger, &row.opening).unwrap(),
                &moves(to, ledger),
                debit_positive(ledger, &row.closing).unwrap(),
            );
            assert!(book.is_ok(), "{ledger} to {to}: {book:?}");
        }
    }
}

#[test]
fn the_day_ends_drop_optional_and_cancelled_and_count_post_dated() {
    let days = cash_book().days;
    for (date, balance) in [
        ("20250403", "6000"),
        ("20250404", "4500"),
        ("20250405", "-300"),
        ("20250407", "1700"),
        ("20250410", "1200"),
        ("20250425", "300"),
    ] {
        assert!(
            days[date].numeric_eq(&amount(balance)),
            "{date}: {:?}",
            days[date]
        );
    }
    // The optional voucher of the 8th and the cancelled one of the 9th left no day.
    assert!(!days.contains_key("20250408") && !days.contains_key("20250409"));
}

#[test]
fn a_book_already_below_zero_is_said_apart_from_a_deposit_that_would_take_it_there() {
    let refused = project(&[outflow("Cash", "20250404", "100")]).unwrap_err();
    assert_eq!(refused.code(), "cash_book_already_negative");
    let CashProjectionRefusal::AlreadyNegative { date, balance, .. } = refused else {
        panic!("{refused:?}");
    };
    assert_eq!(date, "20250405");
    assert!(balance.numeric_eq(&amount("-300")));
}

#[test]
fn a_day_before_the_ledgers_first_deposit_is_not_judged() {
    let built = project(&[outflow("Cash", "20250407", "100")]).unwrap();
    let low = &built.lowest[0];
    assert_eq!(
        (low.ledger.as_str(), low.date.as_str()),
        ("Cash", "20250425")
    );
    assert!(low.balance.numeric_eq(&amount("200")));
}

#[test]
fn a_deposit_lowers_every_later_day() {
    let refused = project(&[outflow("Cash", "20250407", "1000")]).unwrap_err();
    assert_eq!(refused.code(), "cash_projection_negative");
    let CashProjectionRefusal::WouldGoNegative { date, balance, .. } = refused else {
        panic!("{refused:?}");
    };
    assert_eq!(date, "20250425");
    assert!(balance.numeric_eq(&amount("-700")));
}

#[test]
fn two_deposits_of_a_file_are_counted_together() {
    let both = [
        outflow("Cash", "20250407", "200"),
        outflow("Cash", "20250425", "200"),
    ];
    assert!(project(&both[..1]).unwrap().lowest[0]
        .balance
        .numeric_eq(&amount("100")));
    assert_eq!(
        project(&both).unwrap_err().code(),
        "cash_projection_negative"
    );
}

#[test]
fn a_balance_of_exactly_zero_is_not_below_zero() {
    let built = project(&[outflow("Cash", "20250425", "300")]).unwrap();
    assert!(built.lowest[0].balance.is_zero());
}

#[test]
fn a_ledger_with_no_deposit_of_its_own_is_not_judged() {
    let other = BookDayEnds {
        ledger: "Petty Cash".to_string(),
        opening: amount("-1"),
        days: BTreeMap::new(),
    };
    let built = established(
        date("20250401"),
        date("20260331"),
        &[cash_book(), other],
        &[outflow("Cash", "20250425", "1")],
    )
    .unwrap();
    assert_eq!(built.lowest.len(), 1);
}

#[test]
fn opening_and_vouchers_that_miss_tallys_closing_refuse() {
    // The post-dated voucher left out of the read, as a scoped answer might.
    let row = row(&text(TRIAL_BALANCES[10].1), "Cash");
    let refused = BookDayEnds::tied(
        "Cash",
        debit_positive("Cash", &row.opening).unwrap(),
        &moves("20250411", "Cash"),
        debit_positive("Cash", &row.closing).unwrap(),
    )
    .unwrap_err();
    assert_eq!(refused.code(), "cash_projection_not_tied");
    assert_eq!(refused.to_json()["reason"], "cash_projection_not_tied");
}

#[test]
fn an_amount_tally_left_empty_is_not_zero() {
    assert_eq!(
        debit_positive("Cash", &NativeTrialBalanceAmount::PresentEmpty),
        Err(CashProjectionRefusal::BalanceNotRead {
            ledger: "Cash".to_string()
        })
    );
    let mut row = row(&text(TRIAL_BALANCES[0].1), "Cash");
    assert!(
        stated_unmoved(&row),
        "Tally states 0.00 on both sides to the 2nd"
    );
    row.credit = NativeTrialBalanceAmount::PresentEmpty;
    assert!(
        !stated_unmoved(&row),
        "an empty side is read from the vouchers"
    );
}

#[test]
fn tallys_debit_negative_reads_as_a_positive_balance() {
    let opening = row(&text(TRIAL_BALANCES[0].1), "Cash").closing;
    let opening = debit_positive("Cash", &opening).unwrap();
    assert!(
        opening.numeric_eq(&amount("1000")),
        "1,000.00 Dr: {opening:?}"
    );
    let closing = row(&text(TRIAL_BALANCES[3].1), "Cash").closing;
    let closing = debit_positive("Cash", &closing).unwrap();
    assert!(
        closing.numeric_eq(&amount("-300")),
        "300.00 Cr: {closing:?}"
    );
}

#[test]
fn only_a_change_to_the_day_end_table_changes_the_record() {
    let built = project(&[outflow("Cash", "20250425", "1")]).unwrap();
    let same = project(&[outflow("Cash", "20250407", "2")]).unwrap();
    assert_eq!(
        built.record(),
        same.record(),
        "the record is of the book alone"
    );
    let mut moved_book = cash_book();
    moved_book
        .days
        .insert("20250426".to_string(), amount("299"));
    let moved_digest = book_sha256(&date("20250401"), &date("20260331"), &[moved_book]);
    assert_ne!(moved_digest, built.record().book_sha256);
}

#[test]
fn the_window_ends_on_a_31_march() {
    for (day, end) in [
        ("20250403", "20260331"),
        ("20260331", "20260331"),
        ("20260401", "20270331"),
    ] {
        assert_eq!(year_end(&date(day)).unwrap(), date(end));
    }
}

#[test]
fn the_cash_ledger_outflows_are_the_cash_out_answers_alone() {
    let vouchers: Vec<ImportVoucher> = serde_json::from_value(json!([
        {"bridge_txn_id":"deposit","date":"20250407","voucher_type":"Contra",
         "entries":[{"ledger":"Bank","amount":"500.00","side":"Dr"},
                    {"ledger":"Cash","amount":"500.00","side":"Cr"}]},
        {"bridge_txn_id":"withdrawal","date":"20250408","voucher_type":"Contra",
         "entries":[{"ledger":"Cash","amount":"900.00","side":"Dr"},
                    {"ledger":"Bank","amount":"900.00","side":"Cr"}]}
    ]))
    .unwrap();
    let answered = |id: &str, cash_out: bool| AnsweredCashLedger {
        bridge_txn_id: id.to_string(),
        ledger: "Cash".to_string(),
        cash_in_hand: true,
        cash_out,
    };
    assert_eq!(
        outflows_of(
            &vouchers,
            &[answered("deposit", true), answered("withdrawal", false)]
        ),
        Ok(vec![outflow("Cash", "20250407", "500.00")])
    );
    // A voucher that does not hold the leg its answer promised.
    assert_eq!(
        outflows_of(&vouchers, &[answered("withdrawal", true)]),
        Err("proposals_file_invalid".to_string())
    );
}
