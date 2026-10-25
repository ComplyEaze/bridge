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
    assert_eq!(
        compare(
            &new,
            &booked("inv-25-26-0001", Some("20251203"), "20251204")
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
