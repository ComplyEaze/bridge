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

/// The pins cover every variant, once each and in declaration order, so a variant added without a
/// pin fails here.
#[test]
fn every_variant_has_one_pinned_string() {
    let pinned: Vec<GstUncertainCode> = GST_UNCERTAIN.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, GstUncertainCode::ALL);
}

/// Each declared code is the string its site wrote, byte for byte, as a variant and as the
/// [`FailureCode`] made from it, so typing a site changes nothing a caller or a receipt sees.
#[test]
fn each_declared_code_is_the_string_its_site_wrote() {
    for (code, string) in GST_UNCERTAIN {
        assert_eq!(code.as_str(), *string);
        assert_eq!(FailureCode::from(*code).as_str(), *string);
    }
}

/// A tool failure made from a declared code carries exactly that code's pinned string.
#[test]
fn a_tool_failure_from_a_declared_code_carries_its_string() {
    for (code, string) in GST_UNCERTAIN {
        assert_eq!(super::super::ToolFailure::from(*code).code, *string);
    }
}

/// Every declared code is one the receipt keeps as it stands: 1 to 64 bytes of lower-case ASCII
/// letters, digits and underscores, with no colon, so the receipt never drops a declared code. No
/// two variants share a string.
#[test]
fn every_declared_code_has_the_receipts_code_shape_and_is_unique() {
    let mut seen = std::collections::BTreeSet::new();
    for code in GstUncertainCode::ALL {
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

/// [`EInvoiceReadbackCode`]'s variants beside their strings, as [`GST_UNCERTAIN`] pins its own.
const E_INVOICE_READBACK: &[(EInvoiceReadbackCode, &str)] = &[
    (
        EInvoiceReadbackCode::ReadFailed,
        "e_invoice_readback_failed",
    ),
    (
        EInvoiceReadbackCode::Unreadable,
        "e_invoice_readback_unreadable",
    ),
    (EInvoiceReadbackCode::NotFound, "e_invoice_not_found"),
    (EInvoiceReadbackCode::NotUnique, "e_invoice_not_unique"),
];

/// The e-invoice read's codes: pinned once each in order, each its string, each of the receipt's
/// shape and none shared with another declared code.
#[test]
fn every_e_invoice_readback_code_is_pinned_and_shaped() {
    let pinned: Vec<EInvoiceReadbackCode> =
        E_INVOICE_READBACK.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, EInvoiceReadbackCode::ALL);
    let mut seen: std::collections::BTreeSet<&str> = GstUncertainCode::ALL
        .iter()
        .map(|code| code.as_str())
        .collect();
    for (code, string) in E_INVOICE_READBACK {
        assert_eq!(code.as_str(), *string);
        assert_eq!(FailureCode::from(*code).as_str(), *string);
        assert_eq!(super::super::ToolFailure::from(*code).code, *string);
        assert!((1..=64).contains(&string.len()), "{string}");
        assert!(
            string
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'),
            "{string}"
        );
        assert!(seen.insert(string), "{string} is declared twice");
    }
}
