use super::*;

/// Each declared variant beside the string its failure site wrote before it was typed, written out
/// here rather than read from the enum, so a changed string fails a test (#1942).
const GST_UNCERTAIN: &[(GstUncertainCode, &str)] = &[
    (
        GstUncertainCode::WindowTooLarge,
        "gst_uncertain_window_too_large",
    ),
    (
        GstUncertainCode::WideReadShort,
        "gst_uncertain_wide_read_short",
    ),
    (
        GstUncertainCode::RowsOverLimit,
        "gst_uncertain_rows_over_limit",
    ),
];

/// The codes of the invoice number read after the post (#2162), each beside its string, written
/// out here as the codes above are.
const INVOICE_NUMBER_AFTER_POST: &[(InvoiceNumberAfterPostCode, &str)] = &[
    (
        InvoiceNumberAfterPostCode::ReadFailed,
        "invoice_number_read_failed",
    ),
    (
        InvoiceNumberAfterPostCode::TypeUnread,
        "invoice_number_type_unread",
    ),
    (
        InvoiceNumberAfterPostCode::InAnotherType,
        "invoice_number_in_another_type",
    ),
    (
        InvoiceNumberAfterPostCode::ReadMissedInvoice,
        "invoice_number_read_missed_invoice",
    ),
    (
        InvoiceNumberAfterPostCode::NotUnique,
        "invoice_number_not_unique",
    ),
];

/// Every declared code with its pinned string, enum by enum.
fn every_pin() -> Vec<(FailureCode, &'static str, &'static str)> {
    let mut pins: Vec<(FailureCode, &'static str, &'static str)> = GST_UNCERTAIN
        .iter()
        .map(|(code, string)| (FailureCode::from(*code), code.as_str(), *string))
        .collect();
    pins.extend(
        INVOICE_NUMBER_AFTER_POST
            .iter()
            .map(|(code, string)| (FailureCode::from(*code), code.as_str(), *string)),
    );
    pins
}

/// The pins cover every variant, once each and in declaration order, so a variant added without a
/// pin fails here.
#[test]
fn every_variant_has_one_pinned_string() {
    let pinned: Vec<GstUncertainCode> = GST_UNCERTAIN.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, GstUncertainCode::ALL);
    let pinned: Vec<InvoiceNumberAfterPostCode> = INVOICE_NUMBER_AFTER_POST
        .iter()
        .map(|(code, _)| *code)
        .collect();
    assert_eq!(pinned, InvoiceNumberAfterPostCode::ALL);
}

/// Each declared code is the string its site wrote, byte for byte, as a variant and as the
/// [`FailureCode`] made from it, so typing a site changes nothing a caller or a receipt sees.
#[test]
fn each_declared_code_is_the_string_its_site_wrote() {
    for (failure, declared, string) in every_pin() {
        assert_eq!(declared, string);
        assert_eq!(failure.as_str(), string);
    }
}

/// A tool failure made from a declared code carries exactly that code's pinned string.
#[test]
fn a_tool_failure_from_a_declared_code_carries_its_string() {
    for (code, string) in GST_UNCERTAIN {
        assert_eq!(super::super::ToolFailure::from(*code).code, *string);
    }
    for (code, string) in INVOICE_NUMBER_AFTER_POST {
        assert_eq!(super::super::ToolFailure::from(*code).code, *string);
    }
}

/// Every declared code is one the receipt keeps as it stands: 1 to 64 bytes of lower-case ASCII
/// letters, digits and underscores, with no colon, so the receipt never drops a declared code. No
/// two variants share a string, in any enum.
#[test]
fn every_declared_code_has_the_receipts_code_shape_and_is_unique() {
    let mut seen = std::collections::BTreeSet::new();
    for (_, text, _) in every_pin() {
        assert!((1..=64).contains(&text.len()), "{text}");
        assert!(
            text.bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'),
            "{text}"
        );
        assert!(seen.insert(text), "{text} is declared twice");
    }
}
