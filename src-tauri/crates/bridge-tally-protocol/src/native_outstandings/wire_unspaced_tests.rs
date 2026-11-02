//! The compliance ledger snapshot of a book that holds foreign-currency vouchers, as TallyPrime
//! 7.2 wrote it (the lab's `ledger_snapshot_forex_unspaced_composite`, #683 and #2071): a rupee
//! ledger a dollar entry touched carries its balance as a composite with no space after the
//! foreign symbol, and is set aside by name, as the 7.1 layout is, instead of refusing the read.
use super::*;
use crate::native_outstandings::BaseCurrencyName;

const COMPANY: &str = "1bb48be5-baba-45b2-90d8-7593244915a9";

fn snapshot() -> String {
    String::from_utf16(
        &include_bytes!(
            "../../tests/fixtures/native/ledger_snapshot_forex_unspaced_composite.utf16le.xml"
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

#[test]
fn the_unspaced_72_snapshot_sets_its_composite_ledgers_aside_and_reads_the_plain_one() {
    let read = parse_compliance_ledger_snapshot_for_company(&snapshot(), COMPANY, &rupee())
        .expect("the capture no longer refuses");
    // Every ledger of the capture is kept in the base currency.
    assert!(read.foreign.is_empty());
    let base = read
        .base
        .iter()
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(base, ["Cash"], "only the plain rupee ledger is read");
    assert_eq!(read.mixed.len(), 4, "{:?}", read.mixed);
    for name in [
        "BRIDGE Export Sales",
        "BRIDGE Lab Bank",
        "BRIDGE USD Customer",
        "Profit & Loss A/c",
    ] {
        assert!(
            read.mixed.iter().any(|set_aside| set_aside == name),
            "{name}"
        );
    }
}

/// The strict parse that has no base to classify with is unchanged: it still refuses a
/// composite balance as a foreign-currency one, by the typed variant, so a path that reads the
/// snapshot without classifying it cannot read the value.
#[test]
fn the_snapshot_read_without_a_base_still_refuses_the_composite() {
    assert_eq!(
        parse_native_ledger_snapshot(&snapshot()),
        Err(NativeOutstandingsError::ForeignCurrencyLedgerBalance {
            ledger_name: "BRIDGE Export Sales".to_string()
        })
    );
}

/// The outstandings read classifies the same snapshot with `parse_native_ledger_snapshot_classified`
/// (`tally/runtime.rs`), the function under the compliance read (#642). On a book with several
/// Currency masters the 7.2 composite ledgers, parties among them, are named in `mixed` and never
/// read, so their balances and bills leave every figure and the result is partial, as for the 7.1
/// layout (`a_book_with_only_mixed_ledgers_set_aside_is_still_partial` in the agent tests). A book
/// with one Currency master keeps its refusal, decided at `runtime.rs` from the base's master
/// count. The tool is not tested end to end on this book: the lab's open-bills and book-extent
/// answers for it are not committed.
#[test]
fn the_outstandings_snapshot_read_sets_the_72_composite_ledgers_aside_too() {
    let read = parse_native_ledger_snapshot_classified(&snapshot(), &rupee())
        .expect("the outstandings classification no longer refuses the capture");
    assert!(read.foreign.is_empty());
    let base = read
        .base
        .iter()
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(base, ["Cash"], "only the plain rupee ledger is read");
    assert_eq!(read.mixed.len(), 4, "{:?}", read.mixed);
    for name in [
        "BRIDGE Export Sales",
        "BRIDGE Lab Bank",
        "BRIDGE USD Customer",
        "Profit & Loss A/c",
    ] {
        assert!(
            read.mixed.iter().any(|set_aside| set_aside == name),
            "{name}"
        );
    }
}

/// One closing balance of the capture with its layout changed in each way no capture shows is
/// not set aside: the row is kept as a plain amount, and the parse refuses it, as before.
#[test]
fn a_layout_the_capture_does_not_show_is_still_refused() {
    let capture = snapshot();
    let lab_bank = "-$100.00 @ \u{20b9} 87/$ = -\u{20b9} 8700.00";
    assert_eq!(capture.matches(lab_bank).count(), 1);
    for changed in [
        // a rate that is not a decimal (two points)
        "-$100.00 @ \u{20b9} 87.5.1/$ = -\u{20b9} 8700.00",
        // the base amount against its symbol
        "-$100.00 @ \u{20b9} 87/$ = -\u{20b9}8700.00",
        // two spaces before `=` with the foreign amount unspaced
        "-$100.00 @ \u{20b9} 87/$  = -\u{20b9} 8700.00",
        // a foreign amount with one decimal
        "-$100.0 @ \u{20b9} 87/$ = -\u{20b9} 8700.00",
        // symbols that disagree
        "-$100.00 @ \u{20b9} 87/\u{20ac} = -\u{20b9} 8700.00",
        // a sign after the symbol
        "$-100.00 @ \u{20b9} 87/$ = -\u{20b9} 8700.00",
    ] {
        let altered = capture.replace(lab_bank, changed);
        let refused = parse_compliance_ledger_snapshot_for_company(&altered, COMPANY, &rupee());
        assert!(
            matches!(
                refused,
                Err(NativeOutstandingsError::ForeignCurrencyLedgerBalance { .. }
                    | NativeOutstandingsError::InvalidAmount)
            ),
            "{changed}: {refused:?}"
        );
    }
}

/// The two reads that name the lab book's base, as Tally answered them on 7.2: two Currency masters
/// (`$` and the rupee master, whose `NAME` and `ORIGINALNAME` are the one symbol), the company's own
/// `CURRENCYNAME` that same symbol, and the base identified from them, with several masters in the
/// book.
#[test]
fn the_72_reads_identify_the_rupee_master_as_the_base_among_two() {
    let masters = crate::native_outstandings::parse_currency_master_list(&decode(include_bytes!(
        "../../tests/fixtures/native/currency_masters_forex_72.utf16le.xml"
    )))
    .unwrap();
    assert_eq!(masters.count(), 2);
    let base = rupee();
    assert_eq!(base.name(), "\u{20b9}");
    assert!(!base.is_single_master());
    // Every ledger of the snapshot carries that same name as its own currency.
    let capture = snapshot();
    assert_eq!(
        capture
            .matches("<CURRENCYNAME TYPE=\"String\">\u{20b9}</CURRENCYNAME>")
            .count(),
        5
    );
}

/// The same book after dollar purchases and payments, every one keyed at a whole-number rate
/// (`ledger_snapshot_forex_decimal_rate` and `trial_balance_forex_decimal_rate`, #2071): Tally
/// wrote the Profit & Loss balance and two Trial Balance movements with a rate that has decimals.
fn decimal_rate_snapshot() -> String {
    decode(include_bytes!(
        "../../tests/fixtures/native/ledger_snapshot_forex_decimal_rate.utf16le.xml"
    ))
}

fn decimal_rate_trial_balance() -> String {
    decode(include_bytes!(
        "../../tests/fixtures/native/trial_balance_forex_decimal_rate.utf16le.xml"
    ))
}

const DECIMAL_RATE_SET_ASIDE: [&str; 6] = [
    "BRIDGE Export Sales",
    "BRIDGE Import Purchases",
    "BRIDGE Lab Bank",
    "BRIDGE USD Customer",
    "BRIDGE USD Supplier",
    "Profit & Loss A/c",
];

/// The capture holds a balance with a rate with decimals, and the compliance and outstandings
/// classifications set it aside by name with the other composite ledgers; the two rupee ledgers
/// are read. Before, the Profit & Loss row refused the whole read.
#[test]
fn a_72_balance_with_a_decimal_rate_is_set_aside_by_name() {
    let capture = decimal_rate_snapshot();
    assert_eq!(capture.matches(" @ \u{20b9} 86.0058/$ = ").count(), 1);
    for read in [
        parse_compliance_ledger_snapshot_for_company(&capture, COMPANY, &rupee())
            .map(|read| (read.base, read.foreign, read.mixed)),
        parse_native_ledger_snapshot_classified(&capture, &rupee())
            .map(|read| (read.base, read.foreign, read.mixed)),
    ] {
        let (base, foreign, mixed) = read.expect("the capture is classified");
        assert!(foreign.is_empty());
        let base = base
            .iter()
            .map(|entry| entry.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(base, ["BRIDGE INR Customer", "Cash"]);
        let mut mixed = mixed;
        mixed.sort();
        assert_eq!(mixed, DECIMAL_RATE_SET_ASIDE);
    }
    // The read without a base still refuses, naming the first composite ledger.
    assert!(matches!(
        parse_native_ledger_snapshot(&capture),
        Err(NativeOutstandingsError::ForeignCurrencyLedgerBalance { .. })
    ));
}

/// The Trial Balance with currency on the same book sets the same six ledgers aside: a row is set
/// aside when any of its values is a composite, so its movements with decimal rates changed no
/// outcome there.
#[test]
fn the_decimal_rate_trial_balance_sets_the_same_ledgers_aside() {
    let capture = decimal_rate_trial_balance();
    for rate in ["87.50", "87.9999"] {
        assert_eq!(
            capture.matches(&format!(" @ \u{20b9} {rate}/$ = ")).count(),
            1,
            "{rate}"
        );
    }
    let read = crate::native_trial_balance::parse_native_trial_balance_with_currency(
        &capture,
        COMPANY,
        &rupee(),
    )
    .expect("the capture is read");
    assert!(read.foreign_currency_ledgers.is_empty());
    let mut mixed = read.mixed_currency_ledgers.clone();
    mixed.sort();
    assert_eq!(mixed, DECIMAL_RATE_SET_ASIDE);
}
