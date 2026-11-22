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
    for empty in [
        "",
        "   ",
        "-/ .",
        "\t",
        "\u{200B}",
        "\u{200E}",
        " \u{202A}\u{202C} - \u{FEFF}",
        "\u{FE0F}",
    ] {
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
    assert!(reference("INV/25-26/000001").fits_gstr1());
    assert!(
        !reference("INV/25-26/0000001").fits_gstr1(),
        "seventeen characters"
    );
    assert!(!reference("INV 12").fits_gstr1(), "a space");
    assert!(!reference("INV.12").fits_gstr1(), "a dot");
}

#[test]
fn the_financial_year_begins_on_1_april() {
    assert_eq!(FinancialYear::of(&date("20260331")).starting_year(), 2025);
    assert_eq!(FinancialYear::of(&date("20260401")).starting_year(), 2026);
    // A leap day belongs to the year that began the previous April.
    assert_eq!(FinancialYear::of(&date("20240229")).starting_year(), 2023);
}

#[test]
fn the_earliest_dates_give_financial_year_zero_and_no_panic() {
    // January to March of year 1 belong to the financial year that starts in year 0.
    assert_eq!(FinancialYear::of(&date("00010115")).starting_year(), 0);
    let january = booked("INV/1", None, "00010115");
    assert_eq!(january.financial_years(), (vec![FinancialYear(0)], true));
    // The earliest voucher date whose fallback also looks a year back.
    let april = booked("INV/1", None, "00010401");
    assert_eq!(
        april.financial_years(),
        (vec![FinancialYear(1), FinancialYear(0)], true)
    );
}

#[test]
fn the_same_number_across_the_31_march_and_1_april_boundary_is_another_invoice() {
    let new = incoming("INV/0001", "20260401");
    assert_eq!(
        compare(&new, &booked("INV/0001", Some("20260331"), "20260331")),
        InvoiceMatch::Different
    );
    assert_eq!(
        compare(&new, &booked("INV/0001", Some("20260402"), "20260402")),
        InvoiceMatch::Booked
    );
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
fn two_full_numbers_sharing_a_serial_are_different_even_with_a_second_signal() {
    let new = incoming("A/1", "20251201");
    let same_date = BookedInvoice {
        value: Some(value("5900.00")),
        ..booked("B/1", Some("20251201"), "20251203")
    };
    assert_eq!(compare(&new, &same_date), InvoiceMatch::Different);
    let same_value = booked("B/1", Some("20251205"), "20251206");
    assert_eq!(compare(&new, &same_value), InvoiceMatch::Different);
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
fn the_voucher_date_fallback_takes_the_year_after_in_march_only() {
    // An invoice dated 5 April booked on 31 March as a year-end entry, with no invoice date.
    let april_invoice = incoming("GS1/P/0001", "20260405");
    for (voucher_date, expected) in [
        ("20260331", InvoiceMatch::PossiblyBooked),
        ("20260301", InvoiceMatch::PossiblyBooked),
        ("20260228", InvoiceMatch::Different),
        ("20250331", InvoiceMatch::Different),
    ] {
        assert_eq!(
            compare(&april_invoice, &booked("GS1/P/0001", None, voucher_date)),
            expected,
            "{voucher_date}"
        );
    }
    assert_eq!(
        booked("GS1/P/0001", None, "20260331").financial_years(),
        (vec![FinancialYear(2025), FinancialYear(2026)], true)
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
        "INV-1\u{200E}2/A",
        "\u{202A}INV-12/A\u{202C}",
        "INV-\u{2066}12\u{2069}/A",
        "INV-1\u{061C}2/A",
        "INV-12\u{FE0F}/A",
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
fn full_width_latin_letters_read_as_ascii_letters() {
    assert_eq!(reference("ＩＮＶ－１２").exact(), "INV/12");
    assert_eq!(
        compare(
            &incoming("INV-12", "20251201"),
            &booked("ＩＮＶ－１２", Some("20251202"), "20251203")
        ),
        InvoiceMatch::Booked
    );
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
fn a_serial_before_a_four_digit_year_is_kept() {
    let new = incoming("INV/24/2025-26", "20251201");
    let copy = booked("INV/24", Some("20251201"), "20251203");
    assert_eq!(
        compare(&new, &copy),
        InvoiceMatch::PossiblyBooked,
        "INV/24/2025-26 INV/24"
    );
}

#[test]
fn leading_zeros_on_a_serial_before_the_year_are_kept() {
    let new = incoming("0124/2025-26", "20251201");
    let copy = booked("124", Some("20251201"), "20251203");
    assert_eq!(
        compare(&new, &copy),
        InvoiceMatch::PossiblyBooked,
        "0124/2025-26 124"
    );
}

#[test]
fn a_two_digit_serial_before_the_year_is_kept() {
    let new = incoming("24/2025-26", "20251201");
    let copy = booked("24", Some("20251201"), "20251203");
    assert_eq!(
        compare(&new, &copy),
        InvoiceMatch::PossiblyBooked,
        "24/2025-26 24"
    );
}

#[test]
fn a_serial_after_the_year_is_kept_with_leading_zeros() {
    let new = incoming("INV/2025-26/0127", "20251201");
    let copy = booked("INV/25-26/127", Some("20251201"), "20251203");
    assert_eq!(
        compare(&new, &copy),
        InvoiceMatch::PossiblyBooked,
        "INV/2025-26/0127 INV/25-26/127"
    );
}

#[test]
fn a_serial_after_the_year_is_kept() {
    let new = incoming("2025-26/27", "20251201");
    let copy = booked("25-26/27", Some("20251201"), "20251203");
    assert_eq!(
        compare(&new, &copy),
        InvoiceMatch::PossiblyBooked,
        "2025-26/27 25-26/27"
    );
}

#[test]
fn a_serial_after_a_two_digit_year_is_kept() {
    let new = incoming("GST/24-25/26", "20241201");
    let copy = booked("GST/2024-25/26", Some("20241201"), "20241203");
    assert_eq!(
        compare(&new, &copy),
        InvoiceMatch::PossiblyBooked,
        "GST/24-25/26 GST/2024-25/26"
    );
}

#[test]
fn different_running_numbers_beside_one_year_stay_different() {
    for (new, copy) in [
        ("INV/24/2025-26", "INV/25/2025-26"),
        ("0124/2025-26", "0125/2025-26"),
        ("INV/2025-26/0127", "INV/2025-26/0128"),
    ] {
        let new = incoming(new, "20251201");
        let copy = booked(copy, Some("20251201"), "20251203");
        assert_eq!(
            compare(&new, &copy),
            InvoiceMatch::Different,
            "{new:?} {copy:?}"
        );
    }
}

#[test]
fn a_running_number_beside_the_year_needs_a_second_signal() {
    for (new, copy) in [
        ("INV/24/2025-26", "INV/24"),
        ("0124/2025-26", "124"),
        ("24/2025-26", "24"),
        ("INV/2025-26/0127", "INV/25-26/127"),
        ("2025-26/27", "25-26/27"),
        ("GST/24-25/26", "GST/2024-25/26"),
    ] {
        let invoice_date = if new.starts_with("GST/") {
            "20241201"
        } else {
            "20251201"
        };
        let other_date = if new.starts_with("GST/") {
            "20241205"
        } else {
            "20251205"
        };
        let new = incoming(new, invoice_date);
        let mut copy = booked(copy, Some(other_date), other_date);
        copy.value = Some(value("100.00"));
        assert_eq!(
            compare(&new, &copy),
            InvoiceMatch::Different,
            "{new:?} {copy:?}"
        );
    }
}

#[test]
fn an_ambiguous_three_part_number_keeps_both_serial_readings() {
    let new = incoming("24/25/26", "20251201");
    for serial in ["24", "26"] {
        let copy = booked(serial, Some("20251201"), "20251203");
        assert_eq!(
            compare(&new, &copy),
            InvoiceMatch::PossiblyBooked,
            "24/25/26 {serial}"
        );
    }
    let copy = booked("24/25/26", Some("20251202"), "20251203");
    assert_eq!(compare(&new, &copy), InvoiceMatch::Booked);
}

#[test]
fn a_placeholder_with_a_year_is_never_booked_and_needs_a_second_signal() {
    for placeholder in ["NA/2025-26", "CASH/25-26", "0/2025-26", "2025-26"] {
        let new = incoming(placeholder, "20251201");
        let mut copy = booked(placeholder, Some("20251205"), "20251206");
        copy.value = Some(value("100.00"));
        assert_eq!(
            compare(&new, &copy),
            InvoiceMatch::Different,
            "{placeholder} without a second signal"
        );
        copy.invoice_date = Some(date("20251201"));
        assert_eq!(
            compare(&new, &copy),
            InvoiceMatch::PossiblyBooked,
            "{placeholder} with the same date"
        );
        copy.invoice_date = Some(date("20251205"));
        copy.value = Some(value("-11800.00"));
        assert_eq!(
            compare(&new, &copy),
            InvoiceMatch::PossiblyBooked,
            "{placeholder} with the same value"
        );
    }
}

#[test]
fn a_real_serial_with_a_year_can_still_be_booked() {
    let new = incoming("INV/7/2025-26", "20251201");
    let copy = booked("INV/7/2025-26", Some("20251202"), "20251203");
    assert_eq!(compare(&new, &copy), InvoiceMatch::Booked);
}

/// A lone year-like run is the number when nothing else is: it is not a placeholder.
#[test]
fn a_number_that_is_only_a_year_like_run_can_still_be_booked() {
    for number in ["INV/2025", "2025", "2025/INV"] {
        let new = incoming(number, "20251201");
        let copy = booked(number, Some("20251205"), "20251206");
        assert_eq!(compare(&new, &copy), InvoiceMatch::Booked, "{number}");
    }
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
    let assert_folded = |copied: &str, ascii: &str, expected: &str| {
        assert_eq!(reference(copied).exact(), expected, "{copied:?}");
        assert_eq!(
            compare(
                &incoming(ascii, "20251201"),
                &booked(copied, Some("20251202"), "20251203")
            ),
            InvoiceMatch::Booked,
            "{copied:?}"
        );
    };
    let invisible = ['\u{00AD}', '\u{034F}', '\u{061C}', '\u{180E}']
        .into_iter()
        .chain('\u{200B}'..='\u{200F}')
        .chain('\u{202A}'..='\u{202E}')
        .chain('\u{2060}'..='\u{2064}')
        .chain('\u{2066}'..='\u{2069}')
        .chain('\u{FE00}'..='\u{FE0F}')
        .chain(['\u{FEFF}']);
    for ch in invisible {
        let copied = format!("INV-1{ch}2/A");
        assert_folded(&copied, "INV-12/A", "INV/12/A");
    }
    for (ch, ascii) in [
        ('-', "INV-12/A"),
        ('/', "INV/12/A"),
        ('\\', "INV\\12/A"),
        ('.', "INV.12/A"),
        ('_', "INV_12/A"),
        ('\u{2010}', "INV-12/A"),
        ('\u{2011}', "INV-12/A"),
        ('\u{2012}', "INV-12/A"),
        ('\u{2013}', "INV-12/A"),
        ('\u{2014}', "INV-12/A"),
        ('\u{2015}', "INV-12/A"),
        ('\u{2044}', "INV/12/A"),
        ('\u{2212}', "INV-12/A"),
        ('\u{2215}', "INV/12/A"),
        ('\u{FE58}', "INV-12/A"),
        ('\u{FE63}', "INV-12/A"),
        ('\u{FF0D}', "INV-12/A"),
        ('\u{FF0E}', "INV.12/A"),
        ('\u{FF0F}', "INV/12/A"),
        ('\u{FF3C}', "INV\\12/A"),
        ('\u{FF3F}', "INV_12/A"),
    ] {
        let copied = format!("INV{ch}12/A");
        assert_folded(&copied, ascii, "INV/12/A");
    }
    for (copied, ascii, expected) in [
        ("INV-1\u{0966}/A", "INV-10/A", "INV/10/A"),
        ("INV-1\u{0967}/A", "INV-11/A", "INV/11/A"),
        ("INV-1\u{0968}/A", "INV-12/A", "INV/12/A"),
        ("INV-1\u{0969}/A", "INV-13/A", "INV/13/A"),
        ("INV-1\u{096A}/A", "INV-14/A", "INV/14/A"),
        ("INV-1\u{096B}/A", "INV-15/A", "INV/15/A"),
        ("INV-1\u{096C}/A", "INV-16/A", "INV/16/A"),
        ("INV-1\u{096D}/A", "INV-17/A", "INV/17/A"),
        ("INV-1\u{096E}/A", "INV-18/A", "INV/18/A"),
        ("INV-1\u{096F}/A", "INV-19/A", "INV/19/A"),
        ("INV-1\u{FF10}/A", "INV-10/A", "INV/10/A"),
        ("INV-1\u{FF11}/A", "INV-11/A", "INV/11/A"),
        ("INV-1\u{FF12}/A", "INV-12/A", "INV/12/A"),
        ("INV-1\u{FF13}/A", "INV-13/A", "INV/13/A"),
        ("INV-1\u{FF14}/A", "INV-14/A", "INV/14/A"),
        ("INV-1\u{FF15}/A", "INV-15/A", "INV/15/A"),
        ("INV-1\u{FF16}/A", "INV-16/A", "INV/16/A"),
        ("INV-1\u{FF17}/A", "INV-17/A", "INV/17/A"),
        ("INV-1\u{FF18}/A", "INV-18/A", "INV/18/A"),
        ("INV-1\u{FF19}/A", "INV-19/A", "INV/19/A"),
    ] {
        assert_folded(copied, ascii, expected);
    }
    for (full_width, ascii) in ('\u{FF21}'..='\u{FF3A}')
        .zip('A'..='Z')
        .chain(('\u{FF41}'..='\u{FF5A}').zip('A'..='Z'))
    {
        let copied = format!("INV-{full_width}12");
        let ascii_number = format!("INV-{ascii}12");
        let expected = format!("INV/{ascii}12");
        assert_folded(&copied, &ascii_number, &expected);
    }
    for (copied, expected, outcome) in [
        ("INV\u{2010}12/A", "INV/12/A", InvoiceMatch::Booked),
        (
            "INV-1\u{202F}2/A",
            "INV/1/2/A",
            InvoiceMatch::PossiblyBooked,
        ),
        (
            "INV-1\u{2065}2/A",
            "INV/1\u{2065}2/A",
            InvoiceMatch::Different,
        ),
        (
            "INV-1\u{206A}2/A",
            "INV/1\u{206A}2/A",
            InvoiceMatch::Different,
        ),
        (
            "INV-1\u{FE10}2/A",
            "INV/1\u{FE10}2/A",
            InvoiceMatch::Different,
        ),
        (
            "INV-1\u{0965}2/A",
            "INV/1\u{0965}2/A",
            InvoiceMatch::Different,
        ),
        (
            "INV-1\u{FF1A}2/A",
            "INV/1\u{FF1A}2/A",
            InvoiceMatch::Different,
        ),
        (
            "INV-1\u{FF3B}2/A",
            "INV/1\u{FF3B}2/A",
            InvoiceMatch::Different,
        ),
        (
            "INV-1\u{FF5B}2/A",
            "INV/1\u{FF5B}2/A",
            InvoiceMatch::Different,
        ),
    ] {
        assert_eq!(reference(copied).exact(), expected, "{copied:?}");
        assert_eq!(
            compare(
                &incoming("INV-12/A", "20251201"),
                &booked(copied, Some("20251202"), "20251203")
            ),
            outcome,
            "{copied:?}"
        );
    }
    // U+200A, U+2029, U+202F and U+205F are whitespace, so they read as separators.
    // U+002D, U+002E, U+002F, U+200F, U+2010..U+2015 and U+FF0F are folded too.
    // U+0030 and U+FF10 are ASCII and full-width zero, not separators.
    for (ch, expected, outcome) in [
        // Below and above each single invisible character.
        ('\u{00AC}', "INV/1\u{00AC}2/A", InvoiceMatch::Different),
        ('\u{00AE}', "INV/1\u{00AE}2/A", InvoiceMatch::Different),
        ('\u{034E}', "INV/1\u{034E}2/A", InvoiceMatch::Different),
        ('\u{0350}', "INV/1\u{0350}2/A", InvoiceMatch::Different),
        ('\u{061B}', "INV/1\u{061B}2/A", InvoiceMatch::Different),
        ('\u{061D}', "INV/1\u{061D}2/A", InvoiceMatch::Different),
        ('\u{180D}', "INV/1\u{180D}2/A", InvoiceMatch::Different),
        ('\u{180F}', "INV/1\u{180F}2/A", InvoiceMatch::Different),
        ('\u{FEFE}', "INV/1\u{FEFE}2/A", InvoiceMatch::Different),
        ('\u{FF00}', "INV/1\u{FF00}2/A", InvoiceMatch::Different),
        // Below and above each invisible range.
        ('\u{200A}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{200F}', "INV/12/A", InvoiceMatch::Booked),
        ('\u{2029}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{202F}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{205F}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{2065}', "INV/1\u{2065}2/A", InvoiceMatch::Different),
        ('\u{206A}', "INV/1\u{206A}2/A", InvoiceMatch::Different),
        ('\u{FDFF}', "INV/1\u{FDFF}2/A", InvoiceMatch::Different),
        ('\u{FE10}', "INV/1\u{FE10}2/A", InvoiceMatch::Different),
        // Below and above each ASCII separator.
        (',', "INV/1,2/A", InvoiceMatch::Different),
        ('-', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('.', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('/', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('0', "INV/102/A", InvoiceMatch::Different),
        ('[', "INV/1[2/A", InvoiceMatch::Different),
        (']', "INV/1]2/A", InvoiceMatch::Different),
        ('^', "INV/1^2/A", InvoiceMatch::Different),
        ('`', "INV/1`2/A", InvoiceMatch::Different),
        // Below and above the dash range and each single copied separator.
        ('\u{2010}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{2011}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{2012}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{2013}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{2014}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{2015}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{2016}', "INV/1\u{2016}2/A", InvoiceMatch::Different),
        ('\u{2043}', "INV/1\u{2043}2/A", InvoiceMatch::Different),
        ('\u{2045}', "INV/1\u{2045}2/A", InvoiceMatch::Different),
        ('\u{2211}', "INV/1\u{2211}2/A", InvoiceMatch::Different),
        ('\u{2213}', "INV/1\u{2213}2/A", InvoiceMatch::Different),
        ('\u{2214}', "INV/1\u{2214}2/A", InvoiceMatch::Different),
        ('\u{2216}', "INV/1\u{2216}2/A", InvoiceMatch::Different),
        ('\u{FE57}', "INV/1\u{FE57}2/A", InvoiceMatch::Different),
        ('\u{FE59}', "INV/1\u{FE59}2/A", InvoiceMatch::Different),
        ('\u{FE62}', "INV/1\u{FE62}2/A", InvoiceMatch::Different),
        ('\u{FE64}', "INV/1\u{FE64}2/A", InvoiceMatch::Different),
        ('\u{FF0C}', "INV/1\u{FF0C}2/A", InvoiceMatch::Different),
        ('\u{FF0F}', "INV/1/2/A", InvoiceMatch::PossiblyBooked),
        ('\u{FF10}', "INV/102/A", InvoiceMatch::Different),
        // Below and above each folded digit and Latin-letter range.
        ('\u{0965}', "INV/1\u{0965}2/A", InvoiceMatch::Different),
        ('\u{0970}', "INV/1\u{0970}2/A", InvoiceMatch::Different),
        ('\u{FF1A}', "INV/1\u{FF1A}2/A", InvoiceMatch::Different),
        ('\u{FF20}', "INV/1\u{FF20}2/A", InvoiceMatch::Different),
        ('\u{FF3B}', "INV/1\u{FF3B}2/A", InvoiceMatch::Different),
        ('\u{FF40}', "INV/1\u{FF40}2/A", InvoiceMatch::Different),
        ('\u{FF5B}', "INV/1\u{FF5B}2/A", InvoiceMatch::Different),
    ] {
        let copied = format!("INV-1{ch}2/A");
        assert_eq!(reference(&copied).exact(), expected, "{copied:?}");
        assert_eq!(
            compare(
                &incoming("INV-12/A", "20251201"),
                &booked(&copied, Some("20251202"), "20251203")
            ),
            outcome,
            "{copied:?}"
        );
    }
}

#[test]
fn a_number_with_multibyte_characters_is_kept_and_never_panics() {
    let kept = reference("INV-\u{1F4C4}-12\u{00C9}");
    assert!(kept.exact().contains('\u{1F4C4}'));
    assert!(!kept.fits_gstr1());
}

#[test]
fn the_full_width_full_stop_low_line_and_reverse_solidus_fold_like_their_ascii_forms() {
    for (wide, ascii) in [
        ("INV\u{FF0E}12\u{FF0E}A", "INV.12.A"),
        ("INV\u{FF3F}12\u{FF3F}A", "INV_12_A"),
        ("INV\u{FF3C}12\u{FF3C}A", "INV\\12\\A"),
    ] {
        assert_eq!(
            reference(wide).exact(),
            reference(ascii).exact(),
            "{wide:?}"
        );
        assert_eq!(reference(wide).exact(), "INV/12/A", "{wide:?}");
        assert_eq!(
            compare(
                &incoming(ascii, "20251201"),
                &booked(wide, Some("20251202"), "20251203")
            ),
            InvoiceMatch::Booked,
            "{wide:?}"
        );
    }
}
