use super::*;

fn reference(raw: &str) -> SupplierInvoiceRef {
    SupplierInvoiceRef::parse(raw).expect("a number")
}

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).expect("a date")
}

fn value(text: &str) -> ExactDecimal {
    ExactDecimal::parse(text).expect("a decimal")
}

fn incoming(raw: &str, invoice_date: &str) -> IncomingInvoice {
    IncomingInvoice {
        reference: reference(raw),
        invoice_date: date(invoice_date),
        document: SupplierDocument::Invoice,
        value: Some(value("11800.00")),
    }
}

fn booked(raw: &str, invoice_date: Option<&str>, voucher_date: &str) -> BookedInvoice {
    BookedInvoice {
        reference: reference(raw),
        invoice_date: invoice_date.map(date),
        voucher_date: date(voucher_date),
        document: Some(SupplierDocument::Invoice),
        value: Some(value("-11800.00")),
    }
}

#[test]
fn a_number_is_folded_to_upper_case_with_one_separator_and_refused_only_when_empty() {
    assert_eq!(reference("inv 12").exact(), "INV/12");
    assert_eq!(reference("INV-12").exact(), "INV/12");
    assert_eq!(reference(" INV//12. ").exact(), "INV/12");
    assert_eq!(reference("GS1/P/0001").exact(), "GS1/P/0001");
    assert_eq!(reference("inv 12").raw(), "inv 12");
    for empty in ["", "   ", "-/ .", "\t"] {
        assert_eq!(
            SupplierInvoiceRef::parse(empty),
            Err(SupplierInvoiceRefError::Empty),
            "{empty:?}"
        );
    }
}

#[test]
fn the_gstr1_shape_is_reported_never_required() {
    assert!(reference("INV/25-26/0001").fits_gstr1());
    assert!(reference("1234567890123456").fits_gstr1());
    assert!(
        !reference("12345678901234567").fits_gstr1(),
        "seventeen characters"
    );
    assert!(!reference("INV 12").fits_gstr1(), "a space");
    assert!(!reference("INV.12").fits_gstr1(), "a dot");
}

#[test]
fn the_financial_year_begins_on_1_april() {
    assert_eq!(FinancialYear::of(&date("20260331")).starting_year(), 2025);
    assert_eq!(FinancialYear::of(&date("20260401")).starting_year(), 2026);
}

#[test]
fn the_same_number_in_one_financial_year_is_the_same_invoice_whatever_its_date() {
    let new = incoming("INV/25-26/0001", "20251201");
    // Case and characters a GSTR-1 number cannot hold do not make another invoice.
    assert_eq!(
        compare(
            &new,
            &booked("inv/25-26/0001 ", Some("20251203"), "20251204")
        ),
        InvoiceMatch::Booked
    );
    // A supplier who restarts the series each year: another financial year is another invoice.
    assert_eq!(
        compare(
            &new,
            &booked("INV/25-26/0001", Some("20260415"), "20260416")
        ),
        InvoiceMatch::Different
    );
    // `-` and `/` are both GSTR-1 characters: two series may differ only there, so a match that
    // folding them made is only possible.
    assert_eq!(
        compare(
            &incoming("INV-1", "20251201"),
            &booked("INV/1", Some("20251203"), "20251204")
        ),
        InvoiceMatch::PossiblyBooked
    );
}

#[test]
fn a_financial_year_from_the_voucher_date_makes_a_match_only_possible() {
    // No invoice date in the books; the voucher is dated in May, so the invoice may be of the
    // year before, which is the incoming invoice's year.
    let new = incoming("GS1/P/0001", "20260310");
    assert_eq!(
        compare(&new, &booked("GS1/P/0001", None, "20260505")),
        InvoiceMatch::PossiblyBooked
    );
    // A voucher dated in December cannot hold an invoice of the year before.
    assert_eq!(
        compare(&new, &booked("GS1/P/0001", None, "20261205")),
        InvoiceMatch::Different
    );
}

#[test]
fn a_document_type_that_differs_or_is_unknown_never_makes_a_booked_match() {
    let new = incoming("GS1/1", "20251201");
    let mut note = booked("GS1/1", Some("20251202"), "20251203");
    note.document = Some(SupplierDocument::DebitNote);
    assert_eq!(compare(&new, &note), InvoiceMatch::Different);
    note.document = None;
    assert_eq!(compare(&new, &note), InvoiceMatch::PossiblyBooked);
}

#[test]
fn a_loose_or_serial_match_counts_only_with_the_same_date_or_value() {
    let new = incoming("INV/25-26/0001", "20251201");
    // Leading zeros differ: the same value is the second signal.
    let mut copy = booked("INV/25-26/1", Some("20251202"), "20251203");
    assert_eq!(compare(&new, &copy), InvoiceMatch::PossiblyBooked);
    copy.value = Some(value("5900.00"));
    assert_eq!(compare(&new, &copy), InvoiceMatch::Different);
    // The same invoice date is a second signal too.
    copy.invoice_date = Some(date("20251201"));
    assert_eq!(compare(&new, &copy), InvoiceMatch::PossiblyBooked);
    // Only the serial keyed: its trailing digits match.
    let serial = booked("0001", Some("20251201"), "20251203");
    assert_eq!(compare(&new, &serial), InvoiceMatch::PossiblyBooked);
}

#[test]
fn numbers_that_only_run_together_or_share_a_serial_are_different_without_a_second_signal() {
    // "1/23" and "12/3" both read 123 without separators; two series may both hold a 1.
    let new = incoming("1/23", "20251201");
    let mut other = booked("12/3", Some("20251205"), "20251206");
    other.value = Some(value("100.00"));
    assert_eq!(compare(&new, &other), InvoiceMatch::Different);
    let series_a = incoming("A/1", "20251201");
    let mut series_b = booked("B/1", Some("20251205"), "20251206");
    series_b.value = Some(value("100.00"));
    assert_eq!(compare(&series_a, &series_b), InvoiceMatch::Different);
}

#[test]
fn the_voucher_date_fallback_takes_the_year_before_from_april_to_november_only() {
    let last_year = incoming("GS1/P/0001", "20260310");
    for (voucher_date, expected) in [
        ("20260401", InvoiceMatch::PossiblyBooked),
        ("20261130", InvoiceMatch::PossiblyBooked),
        ("20261201", InvoiceMatch::Different),
        ("20270331", InvoiceMatch::Different),
    ] {
        assert_eq!(
            compare(&last_year, &booked("GS1/P/0001", None, voucher_date)),
            expected,
            "{voucher_date}"
        );
    }
    // The voucher date's own year is always one of them.
    let this_year = incoming("GS1/P/0001", "20260610");
    assert_eq!(
        compare(&this_year, &booked("GS1/P/0001", None, "20260620")),
        InvoiceMatch::PossiblyBooked
    );
}

#[test]
fn a_known_other_document_is_different_on_every_path() {
    let new = incoming("INV/01", "20251201");
    let mut note = booked("INV/1", Some("20251202"), "20251203");
    note.document = Some(SupplierDocument::DebitNote);
    assert_eq!(
        compare(&new, &note),
        InvoiceMatch::Different,
        "loose, same value"
    );
    note.invoice_date = None;
    assert_eq!(
        compare(&new, &note),
        InvoiceMatch::Different,
        "voucher-date year"
    );
}

#[test]
fn no_value_or_a_zero_value_is_no_second_signal() {
    let mut new = incoming("INV/25-26/0001", "20251201");
    let mut copy = booked("INV/25-26/1", Some("20251202"), "20251203");
    new.value = None;
    copy.value = None;
    assert_eq!(compare(&new, &copy), InvoiceMatch::Different);
    new.value = Some(value("0.00"));
    copy.value = Some(value("-0.00"));
    assert_eq!(compare(&new, &copy), InvoiceMatch::Different);
}

#[test]
fn zeros_and_numbers_without_a_trailing_digit_fold_as_expected() {
    assert_eq!(loose_form(&reference("000").exact), "0");
    assert_eq!(loose_form(&reference("A/007/B").exact), "A7B");
    assert_eq!(serial_form(&reference("000").exact), Some("0".to_string()));
    assert_eq!(serial_form(&reference("INV/A").exact), None);
    assert!(
        reference(" INV/1 ").fits_gstr1(),
        "surrounding whitespace is not part of it"
    );
}

#[test]
fn leading_zeros_are_dropped_in_every_run_of_digits_not_only_the_last() {
    let new = incoming("2025/0001/A", "20251201");
    let copy = booked("2025/1/A", Some("20251201"), "20251203");
    assert_eq!(compare(&new, &copy), InvoiceMatch::PossiblyBooked);
    assert_eq!(loose_form(&reference("INV/25-26/0001").exact), "INV25261");
}

#[test]
fn a_number_copied_with_typographic_dashes_digits_or_invisible_characters_is_the_same_number() {
    let new = incoming("INV-12/A", "20251201");
    for copied in [
        "INV\u{2013}12/A",
        "INV\u{2212}12/A",
        "INV\u{FF0D}12\u{FF0F}A",
        "INV-1\u{200B}2/A",
        "INV-\u{00AD}12/A",
        "INV-\u{FF11}\u{FF12}/A",
        "INV-\u{0967}\u{0968}/A",
    ] {
        assert_eq!(reference(copied).exact(), "INV/12/A", "{copied:?}");
        assert_eq!(
            compare(&new, &booked(copied, Some("20251202"), "20251203")),
            InvoiceMatch::Booked,
            "{copied:?}"
        );
    }
}

#[test]
fn a_year_suffix_two_numbers_share_is_no_serial_match() {
    // Two invoices of one day, numbered within the year's series: the trailing "26" is the year.
    let new = incoming("002/2025-26", "20251201");
    let other = booked("001/2025-26", Some("20251201"), "20251201");
    assert_eq!(compare(&new, &other), InvoiceMatch::Different);
}

#[test]
fn a_year_written_into_the_number_does_not_hide_the_running_number() {
    let same_day = |incoming_raw: &str, booked_raw: &str| {
        let mut copy = booked(booked_raw, Some("20251201"), "20251203");
        copy.value = Some(value("100.00"));
        compare(&incoming(incoming_raw, "20251201"), &copy)
    };
    for (new, copy) in [
        ("001/2025-26", "1"),
        ("INV/001/25-26", "INV/001"),
        ("001/FY26", "1"),
        ("001/FY25-26", "1"),
        ("001/FY2025-26", "1"),
        ("FY/2025-26/001", "001"),
        ("INV/2025-26/001", "INV/25-26/001"),
        ("INV/0001/2025", "INV/1"),
    ] {
        assert_eq!(
            same_day(new, copy),
            InvoiceMatch::PossiblyBooked,
            "{new} {copy}"
        );
    }
    // Without a second signal it is still another invoice.
    let mut other_day = booked("1", Some("20251205"), "20251206");
    other_day.value = Some(value("100.00"));
    assert_eq!(
        compare(&incoming("001/2025-26", "20251201"), &other_day),
        InvoiceMatch::Different
    );
    // A number that is only a year keeps the year as its number.
    let mut next = booked("INV/2026", Some("20251201"), "20251203");
    next.value = Some(value("11800.00"));
    assert_eq!(
        compare(&incoming("INV/2025", "20251201"), &next),
        InvoiceMatch::Different
    );
}

#[test]
fn a_placeholder_is_never_booked_and_matches_only_with_a_second_signal() {
    for placeholder in ["NA", "N/A", "Cash", "0", "-0-"] {
        let new = incoming(placeholder, "20251201");
        let mut copy = booked(placeholder, Some("20251205"), "20251206");
        copy.value = Some(value("-500.00"));
        assert_eq!(
            compare(&new, &copy),
            InvoiceMatch::Different,
            "{placeholder}"
        );
        copy.value = Some(value("-11800.00"));
        assert_eq!(
            compare(&new, &copy),
            InvoiceMatch::PossiblyBooked,
            "{placeholder}"
        );
    }
}

#[test]
fn every_folded_character_reads_as_its_ascii_form() {
    for (copied, expected) in [
        ("A\u{2010}1", "A/1"),
        ("A\u{2011}1", "A/1"),
        ("A\u{2012}1", "A/1"),
        ("A\u{2014}1", "A/1"),
        ("A_1", "A/1"),
        ("A\\1", "A/1"),
        ("\u{FEFF}A1", "A1"),
        ("A\u{2060}1", "A1"),
        ("A\u{200C}1\u{200D}", "A1"),
        ("A\u{0966}\u{096F}", "A09"),
        ("A\u{FF10}\u{FF19}", "A09"),
    ] {
        assert_eq!(reference(copied).exact(), expected, "{copied:?}");
    }
}
