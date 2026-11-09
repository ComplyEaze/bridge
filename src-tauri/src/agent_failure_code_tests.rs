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

const READ: &[(ReadCode, &str)] = &[
    (ReadCode::InvalidDateRange, "invalid_date_range"),
    (ReadCode::ReadProtocolInvalid, "agent_read_protocol_invalid"),
    (ReadCode::VoucherAmountInvalid, "voucher_amount_invalid"),
    (ReadCode::LedgerSnapshotDrifted, "ledger_snapshot_drifted"),
    (ReadCode::WindowNotHonoured, "window_not_honoured"),
    (ReadCode::CurrentDateInvalid, "current_date_invalid"),
    (ReadCode::InvalidAsOf, "invalid_as_of"),
];

const GST_STATUS: &[(GstStatusCode, &str)] = &[
    (
        GstStatusCode::TypeNameInvalid,
        "gst_status_type_name_invalid",
    ),
    (GstStatusCode::WindowTooLong, "gst_status_window_too_long"),
    (GstStatusCode::BookTooLarge, "gst_status_book_too_large"),
    (
        GstStatusCode::ReadVoucherUnidentified,
        "gst_status_read_voucher_unidentified",
    ),
    (
        GstStatusCode::TypeNotHonoured,
        "gst_status_type_not_honoured",
    ),
    (GstStatusCode::VoucherUnread, "gst_status_voucher_unread"),
];

const GST_BY_LEDGER: &[(GstByLedgerCode, &str)] = &[(
    GstByLedgerCode::WindowPartial,
    "gst_by_ledger_window_partial",
)];

const REGISTER: &[(RegisterCode, &str)] = &[(
    RegisterCode::MasterMarkUnavailable,
    "register_master_mark_unavailable",
)];

const OUTSTANDINGS: &[(OutstandingsCode, &str)] = &[
    (OutstandingsCode::InvalidAgeingBasis, "invalid_ageing_basis"),
    (OutstandingsCode::InvalidDirection, "invalid_direction"),
    (
        OutstandingsCode::AmountInvalid,
        "outstandings_amount_invalid",
    ),
];

/// Every pin of every enum, as the failure code its variant makes, beside the pinned string.
fn pinned() -> Vec<(FailureCode, &'static str)> {
    fn of<C: Copy + Into<FailureCode>>(
        pins: &[(C, &'static str)],
    ) -> Vec<(FailureCode, &'static str)> {
        pins.iter()
            .map(|(code, text)| ((*code).into(), *text))
            .collect()
    }
    [
        of(GST_UNCERTAIN),
        of(READ),
        of(GST_STATUS),
        of(GST_BY_LEDGER),
        of(REGISTER),
        of(OUTSTANDINGS),
    ]
    .concat()
}

/// Every declared variant of every enum, as the failure code it makes.
fn declared() -> Vec<FailureCode> {
    fn of<C: Copy + Into<FailureCode>>(all: &[C]) -> Vec<FailureCode> {
        all.iter().map(|code| (*code).into()).collect()
    }
    [
        of(GstUncertainCode::ALL),
        of(ReadCode::ALL),
        of(GstStatusCode::ALL),
        of(GstByLedgerCode::ALL),
        of(RegisterCode::ALL),
        of(OutstandingsCode::ALL),
    ]
    .concat()
}

/// The pins cover every variant, once each and in declaration order, so a variant added without a
/// pin fails here.
#[test]
fn every_variant_has_one_pinned_string() {
    fn variants<C: Copy>(pins: &[(C, &str)]) -> Vec<C> {
        pins.iter().map(|(code, _)| *code).collect()
    }
    assert_eq!(variants(GST_UNCERTAIN), GstUncertainCode::ALL);
    assert_eq!(variants(READ), ReadCode::ALL);
    assert_eq!(variants(GST_STATUS), GstStatusCode::ALL);
    assert_eq!(variants(GST_BY_LEDGER), GstByLedgerCode::ALL);
    assert_eq!(variants(REGISTER), RegisterCode::ALL);
    assert_eq!(variants(OUTSTANDINGS), OutstandingsCode::ALL);
    let pinned: Vec<FailureCode> = pinned().into_iter().map(|(code, _)| code).collect();
    assert_eq!(pinned, declared());
}

/// Checks each pin of one enum against its variant directly: the variant's own string, the
/// [`FailureCode`] made from it, and the tool failure made from it by the impl the sites use.
fn check_pins<C>(pins: &[(C, &str)], as_str: fn(C) -> &'static str)
where
    C: Copy + Into<FailureCode> + Into<super::super::ToolFailure>,
{
    for (code, string) in pins {
        assert_eq!(as_str(*code), *string);
        let failure_code: FailureCode = (*code).into();
        assert_eq!(failure_code.as_str(), *string);
        let failure: super::super::ToolFailure = (*code).into();
        assert_eq!(failure.code, *string);
    }
}

/// Each declared code is the string its site wrote, byte for byte, as a variant, as the
/// [`FailureCode`] made from it and as the tool failure made from it, so typing a site changes
/// nothing a caller or a receipt sees.
#[test]
fn each_declared_code_is_the_string_its_site_wrote() {
    check_pins(GST_UNCERTAIN, GstUncertainCode::as_str);
    check_pins(READ, ReadCode::as_str);
    check_pins(GST_STATUS, GstStatusCode::as_str);
    check_pins(GST_BY_LEDGER, GstByLedgerCode::as_str);
    check_pins(REGISTER, RegisterCode::as_str);
    check_pins(OUTSTANDINGS, OutstandingsCode::as_str);
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
/// two variants, in the same enum or in two, share a string.
#[test]
fn every_declared_code_has_the_receipts_code_shape_and_is_unique() {
    let mut seen = std::collections::BTreeSet::new();
    let codes = declared();
    assert_eq!(codes.len(), 21, "the declared codes");
    for code in codes {
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
