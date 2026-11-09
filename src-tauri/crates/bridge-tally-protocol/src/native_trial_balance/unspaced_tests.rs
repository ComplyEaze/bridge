//! The native Trial Balance with currency of a book that holds foreign-currency vouchers, as
//! TallyPrime 7.2 wrote it (the lab's `trial_balance_forex_unspaced_composite`, #683 and #2071).
use super::*;
use crate::native_outstandings::BaseCurrencyName;

const COMPANY: &str = "1bb48be5-baba-45b2-90d8-7593244915a9";

fn trial_balance() -> String {
    String::from_utf16(
        &include_bytes!(
            "../../tests/fixtures/native/trial_balance_forex_unspaced_composite.utf16le.xml"
        )
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The base of the lab book, identified as Bridge identifies it from the lab's own reads on 7.2:
/// the Currency masters with `ORIGINALNAME` and the company's `CURRENCYNAME`.
fn rupee() -> BaseCurrencyName {
    let masters = crate::native_outstandings::parse_currency_master_list(&decode(include_bytes!(
        "../../tests/fixtures/native/currency_masters_originalname_forex_72.utf16le.xml"
    )))
    .expect("the captured masters");
    let company = crate::native_outstandings::parse_company_currency_name(
        &decode(include_bytes!(
            "../../tests/fixtures/native/company_base_currency_forex_72.utf16le.xml"
        )),
        COMPANY,
    )
    .expect("the captured company read");
    masters
        .identify_base(Some(&company))
        .expect("the base is identified")
        .base()
        .clone()
}

/// The refusal the lab saw (`trial_balance_amount_invalid`) comes from the Trial Balance parse:
/// the plain parse reads every amount, and a composite is not an amount.
#[test]
fn the_plain_parse_still_refuses_a_composite_amount() {
    let refused = parse_native_trial_balance(&trial_balance(), COMPANY);
    assert_eq!(refused, Err(NativeTrialBalanceError::InvalidAmount));
    assert_eq!(
        NativeTrialBalanceError::InvalidAmount.code(),
        "trial_balance_amount_invalid"
    );
}

/// With the currency variant, the rows whose values are composites in the 7.2 layout are set
/// aside by name and the plain rupee ledger is read, so the parse no longer refuses.
#[test]
fn the_currency_parse_of_the_72_capture_sets_composite_rows_aside_and_reads_the_rest() {
    let read = parse_native_trial_balance_with_currency(&trial_balance(), COMPANY, &rupee())
        .expect("the capture no longer refuses");
    assert!(read.foreign_currency_ledgers.is_empty());
    let kept = read
        .report
        .rows
        .iter()
        .map(|row| row.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(kept, ["Cash"]);
    let mut mixed = read.mixed_currency_ledgers.clone();
    mixed.sort();
    assert_eq!(
        mixed,
        [
            "BRIDGE Export Sales",
            "BRIDGE Lab Bank",
            "BRIDGE USD Customer",
            "Profit & Loss A/c"
        ]
    );
}
