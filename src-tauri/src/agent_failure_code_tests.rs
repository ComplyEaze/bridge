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

const BANK_STATEMENT_IMPORT: &[(BankStatementImportCode, &str)] = &[(
    BankStatementImportCode::ProposalsFileInvalid,
    "proposals_file_invalid",
)];

/// Every declared code with its pinned string, across the enums.
fn all_pinned() -> Vec<(FailureCode, &'static str)> {
    GST_UNCERTAIN
        .iter()
        .map(|(code, string)| (FailureCode::from(*code), *string))
        .chain(
            BANK_STATEMENT_IMPORT
                .iter()
                .map(|(code, string)| (FailureCode::from(*code), *string)),
        )
        .collect()
}

/// The pins cover every variant, once each and in declaration order, so a variant added without a
/// pin fails here.
#[test]
fn every_variant_has_one_pinned_string() {
    let pinned: Vec<GstUncertainCode> = GST_UNCERTAIN.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, GstUncertainCode::ALL);
    let pinned: Vec<BankStatementImportCode> = BANK_STATEMENT_IMPORT
        .iter()
        .map(|(code, _)| *code)
        .collect();
    assert_eq!(pinned, BankStatementImportCode::ALL);
}

/// Each declared code is the string its site wrote, byte for byte, as a variant and as the
/// [`FailureCode`] made from it, so typing a site changes nothing a caller or a receipt sees.
#[test]
fn each_declared_code_is_the_string_its_site_wrote() {
    for (code, string) in GST_UNCERTAIN {
        assert_eq!(code.as_str(), *string);
    }
    for (code, string) in BANK_STATEMENT_IMPORT {
        assert_eq!(code.as_str(), *string);
    }
    for (code, string) in all_pinned() {
        assert_eq!(code.as_str(), string);
    }
}

/// A tool failure made from a declared code carries exactly that code's pinned string.
#[test]
fn a_tool_failure_from_a_declared_code_carries_its_string() {
    for (code, string) in all_pinned() {
        assert_eq!(super::super::ToolFailure::from(code).code, string);
    }
}

/// Every declared code is one the receipt keeps as it stands: 1 to 64 bytes of lower-case ASCII
/// letters, digits and underscores, with no colon, so the receipt never drops a declared code. No
/// two variants share a string.
#[test]
fn every_declared_code_has_the_receipts_code_shape_and_is_unique() {
    let mut seen = std::collections::BTreeSet::new();
    for (code, _) in all_pinned() {
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

/// The census-reuse refusal (#2399), pinned the same way as the codes above: written out here, so
/// a changed string fails a test.
const CENSUS_REUSE: &[(CensusReuseCode, &str)] =
    &[(CensusReuseCode::Mismatch, "census_reused_mismatch")];

#[test]
fn the_census_reuse_code_is_pinned_once_and_is_the_string_a_caller_sees() {
    let pinned: Vec<CensusReuseCode> = CENSUS_REUSE.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, CensusReuseCode::ALL);
    for (code, string) in CENSUS_REUSE {
        assert_eq!(code.as_str(), *string);
        assert_eq!(FailureCode::from(*code).as_str(), *string);
        assert_eq!(super::super::ToolFailure::from(*code).code, *string);
        assert!((1..=64).contains(&string.len()));
        assert!(string
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'));
    }
}
