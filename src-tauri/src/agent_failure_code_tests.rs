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

/// The read-back's item-line differences (#1379), declared new, so each string is pinned as
/// declared.
const INVOICE_ITEM_LINE: &[(InvoiceItemLineCode, &str)] = &[
    (InvoiceItemLineCode::NotSent, "invoice_line_item_not_sent"),
    (InvoiceItemLineCode::Several, "invoice_line_items_several"),
    (InvoiceItemLineCode::Differs, "invoice_line_item_differs"),
    (InvoiceItemLineCode::Batch, "invoice_line_item_batch"),
];

/// Every pinned code with its string, across the enums.
fn pinned() -> Vec<(FailureCode, &'static str)> {
    GST_UNCERTAIN
        .iter()
        .map(|(code, text)| (FailureCode::from(*code), *text))
        .chain(
            INVOICE_ITEM_LINE
                .iter()
                .map(|(code, text)| (FailureCode::from(*code), *text)),
        )
        .collect()
}

/// The pins cover every variant, once each and in declaration order, so a variant added without a
/// pin fails here.
#[test]
fn every_variant_has_one_pinned_string() {
    let pinned: Vec<GstUncertainCode> = GST_UNCERTAIN.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, GstUncertainCode::ALL);
    let pinned: Vec<InvoiceItemLineCode> =
        INVOICE_ITEM_LINE.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, InvoiceItemLineCode::ALL);
}

/// Each declared code is the string its site wrote, byte for byte, as a variant and as the
/// [`FailureCode`] made from it, so typing a site changes nothing a caller or a receipt sees.
#[test]
fn each_declared_code_is_the_string_its_site_wrote() {
    for (code, string) in GST_UNCERTAIN {
        assert_eq!(code.as_str(), *string);
        assert_eq!(FailureCode::from(*code).as_str(), *string);
    }
    for (code, string) in INVOICE_ITEM_LINE {
        assert_eq!(code.as_str(), *string);
        assert_eq!(FailureCode::from(*code).as_str(), *string);
    }
}

/// A tool failure made from a declared code carries exactly that code's pinned string.
#[test]
fn a_tool_failure_from_a_declared_code_carries_its_string() {
    for (code, string) in pinned() {
        assert_eq!(super::super::ToolFailure::from(code).code, string);
    }
}

/// Every declared code is one the receipt keeps as it stands: 1 to 64 bytes of lower-case ASCII
/// letters, digits and underscores, with no colon, so the receipt never drops a declared code. No
/// two variants share a string.
#[test]
fn every_declared_code_has_the_receipts_code_shape_and_is_unique() {
    let mut seen = std::collections::BTreeSet::new();
    for (code, _) in pinned() {
        let text = code.as_str();
        assert!((1..=64).contains(&text.len()), "{text}");
        assert!(
            text.bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'),
            "{text}"
        );
        assert!(seen.insert(text), "{text} is declared twice");
    }
}
