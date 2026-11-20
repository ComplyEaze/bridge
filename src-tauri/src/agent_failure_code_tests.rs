use super::*;
use crate::tally::stock_item_create::{
    CreateRenderRefusal, DraftRefusal, DraftText, ItemNameNotAbsent, NotGoodsSibling, SiblingField,
    SiblingRefusal, StockItemCatalogueError,
};
use crate::tally::stock_item_verified::{ReadBackVerdict, StockItemImportOutcome};
use bridge_tally_core::master_name_index::{MasterKind, MasterNameIndexError, NameCollision, Via};
use bridge_tally_protocol::native_masters::NativeMastersError;
use bridge_tally_protocol::ImportCounters;

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
    (
        ReadCode::CompanyIdentityIncomplete,
        "company_identity_incomplete",
    ),
    (ReadCode::InvalidDate, "invalid_date"),
    (
        ReadCode::VoucherEntriesUnbalanced,
        "voucher_entries_unbalanced",
    ),
];

const PROPOSALS: &[(ProposalsCode, &str)] = &[
    (ProposalsCode::FileInvalid, "proposals_file_invalid"),
    (ProposalsCode::IdRequired, "proposals_id_required"),
    (ProposalsCode::VouchersRequired, "vouchers_required"),
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

/// The codes of the invoice number read after the post (#2162).
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

/// The lab master tool's refusals that name what they refused (#2499).
#[cfg(feature = "lab-writes")]
const LAB_MASTERS: &[(LabMastersCode, &str)] = &[
    (
        LabMastersCode::OpeningBillsUnsupported,
        "lab_opening_bills_unsupported",
    ),
    (
        LabMastersCode::RootParentUnmeasured,
        "lab_root_parent_unmeasured",
    ),
    (
        LabMastersCode::NameSharesKey,
        "lab_master_name_shares_a_key",
    ),
];

const MOVEMENT: &[(MovementCode, &str)] = &[
    (
        MovementCode::VoucherSnapshotDrifted,
        "voucher_snapshot_drifted",
    ),
    (
        MovementCode::OpeningBalanceNotObserved,
        "opening_balance_not_observed",
    ),
    (MovementCode::ReadFailed, "ledger_movement_read_failed"),
];

const PRESENCE: &[(PresenceCode, &str)] = &[
    (
        PresenceCode::LedgerCatalogueIncomplete,
        "ledger_catalogue_incomplete",
    ),
    (
        PresenceCode::NumberingMethodDiffers,
        "presence_numbering_method_differs_from_book",
    ),
    (PresenceCode::NumberingRequired, "numbering_required"),
    (
        PresenceCode::ImportIdentityIncomplete,
        "presence_import_identity_incomplete",
    ),
];

const TIE_OUT: &[(TieOutCode, &str)] = &[
    (
        TieOutCode::ArithmeticOutOfRange,
        "tie_out_arithmetic_out_of_range",
    ),
    (TieOutCode::LedgerOpeningInvalid, "ledger_opening_invalid"),
    (TieOutCode::LedgerOpeningMissing, "ledger_opening_missing"),
    (
        TieOutCode::LedgerNameDuplicated,
        "ledger_name_duplicated_in_catalogue",
    ),
];

const STOCK_ITEM_CREATE: &[(StockItemCreateCode, &str)] = &[
    (
        StockItemCreateCode::ReadUnreadable,
        "stock_item_read_unreadable",
    ),
    (StockItemCreateCode::NoItems, "stock_item_no_items"),
    (
        StockItemCreateCode::NoNameEntry,
        "stock_item_name_entry_missing",
    ),
    (
        StockItemCreateCode::NameIndexInvalid,
        "stock_item_name_index_invalid",
    ),
    (
        StockItemCreateCode::NameUnchecked,
        "stock_item_name_unchecked",
    ),
    (StockItemCreateCode::NameTaken, "master_name_taken"),
    (
        StockItemCreateCode::SiblingNotFound,
        "stock_item_sibling_not_found",
    ),
    (
        StockItemCreateCode::SiblingAmbiguous,
        "stock_item_sibling_ambiguous",
    ),
    (
        StockItemCreateCode::SiblingOnlyByFold,
        "stock_item_sibling_only_by_fold",
    ),
    (
        StockItemCreateCode::SiblingIncomplete,
        "stock_item_sibling_incomplete",
    ),
    (
        StockItemCreateCode::SiblingNotGoods,
        "stock_item_sibling_not_goods",
    ),
    (
        StockItemCreateCode::SiblingNotTaxable,
        "stock_item_sibling_not_taxable",
    ),
    (
        StockItemCreateCode::SiblingGstNotApplicable,
        "stock_item_sibling_gst_not_applicable",
    ),
    (
        StockItemCreateCode::CatalogueChanged,
        "stock_item_catalogue_changed",
    ),
    (
        StockItemCreateCode::GroupsUnreadable,
        "stock_item_groups_unreadable",
    ),
    (
        StockItemCreateCode::ParentNotInBook,
        "stock_item_parent_not_in_book",
    ),
    (
        StockItemCreateCode::UnitsUnreadable,
        "stock_item_units_unreadable",
    ),
    (
        StockItemCreateCode::UnitNotInBook,
        "stock_item_unit_not_in_book",
    ),
    (
        StockItemCreateCode::ShapeNotMeasured,
        "stock_item_shape_not_measured",
    ),
    (
        StockItemCreateCode::TextNotPlain,
        "stock_item_text_not_plain",
    ),
    (
        StockItemCreateCode::TextNeedsEscaping,
        "stock_item_text_needs_escaping",
    ),
    (
        StockItemCreateCode::ReviewLayoutText,
        "import_review_layout_text",
    ),
    (
        StockItemCreateCode::ReviewFormatText,
        "import_review_format_text",
    ),
    (
        StockItemCreateCode::ReviewTooLarge,
        "master_review_too_large",
    ),
    (
        StockItemCreateCode::CreateApplicationFailure,
        "stock_item_create_application_failure",
    ),
    (
        StockItemCreateCode::AlteredExisting,
        "stock_item_altered_existing",
    ),
    (
        StockItemCreateCode::CountersNotClean,
        "stock_item_counters_not_clean",
    ),
    (
        StockItemCreateCode::CountersUnreported,
        "stock_item_counters_unreported",
    ),
    (
        StockItemCreateCode::ReadBackDiffers,
        "stock_item_read_back_differs",
    ),
    (
        StockItemCreateCode::ReadBackNotFoundOnce,
        "stock_item_read_back_not_found_once",
    ),
    (
        StockItemCreateCode::ReadBackNotNew,
        "stock_item_read_back_not_new",
    ),
    (
        StockItemCreateCode::ReadBackOtherCompany,
        "stock_item_read_back_other_company",
    ),
];

#[test]
fn stock_code_strings_match_addendum() {
    let variants: Vec<StockItemCreateCode> =
        STOCK_ITEM_CREATE.iter().map(|(code, _)| *code).collect();
    assert_eq!(variants, StockItemCreateCode::ALL);
    check_pins(STOCK_ITEM_CREATE, StockItemCreateCode::as_str);
}

#[test]
fn every_stock_refusal_variant_has_exactly_one_code() {
    let unreadable = NativeMastersError::CollectionAbsent;
    let index = || MasterNameIndexError::NoNames { master: 0 };
    let counters = ImportCounters {
        created: 0,
        altered: 0,
        deleted: 0,
        ignored: 0,
        errors: 0,
        cancelled: 0,
        exceptions: 0,
    };

    assert_eq!(
        stock_item_catalogue_refusal_code(&StockItemCatalogueError::Unreadable(unreadable)),
        StockItemCreateCode::ReadUnreadable
    );
    assert_eq!(
        stock_item_catalogue_refusal_code(&StockItemCatalogueError::NoItems),
        StockItemCreateCode::NoItems
    );
    assert_eq!(
        stock_item_catalogue_refusal_code(&StockItemCatalogueError::NoNameEntry { item: 0 }),
        StockItemCreateCode::NoNameEntry
    );
    assert_eq!(
        stock_item_catalogue_refusal_code(&StockItemCatalogueError::Index(index())),
        StockItemCreateCode::NameIndexInvalid
    );

    assert_eq!(
        item_name_refusal_code(&ItemNameNotAbsent::Taken(NameCollision {
            kind: MasterKind::StockItem,
            via: Via::Name,
            guid: crate::synthetic_guid(1).to_string(),
            name: "SC Existing".to_string(),
        })),
        StockItemCreateCode::NameTaken
    );
    assert_eq!(
        item_name_refusal_code(&ItemNameNotAbsent::Unchecked(index())),
        StockItemCreateCode::NameUnchecked
    );

    assert_eq!(
        sibling_refusal_code(&SiblingRefusal::NotFound),
        StockItemCreateCode::SiblingNotFound
    );
    assert_eq!(
        sibling_refusal_code(&SiblingRefusal::Ambiguous { items: 1 }),
        StockItemCreateCode::SiblingAmbiguous
    );
    assert_eq!(
        sibling_refusal_code(&SiblingRefusal::OnlyByFold { items: 1 }),
        StockItemCreateCode::SiblingOnlyByFold
    );
    assert_eq!(
        sibling_refusal_code(&SiblingRefusal::Incomplete(SiblingField::Parent)),
        StockItemCreateCode::SiblingIncomplete
    );
    assert_eq!(
        sibling_refusal_code(&SiblingRefusal::Unchecked(index())),
        StockItemCreateCode::NameUnchecked
    );

    assert_eq!(
        not_goods_refusal_code(&NotGoodsSibling::NotGoods),
        StockItemCreateCode::SiblingNotGoods
    );
    assert_eq!(
        not_goods_refusal_code(&NotGoodsSibling::NotTaxable { entry: 0 }),
        StockItemCreateCode::SiblingNotTaxable
    );
    assert_eq!(
        not_goods_refusal_code(&NotGoodsSibling::GstNotApplicable),
        StockItemCreateCode::SiblingGstNotApplicable
    );

    for (refusal, code) in [
        (
            DraftRefusal::OtherCatalogue,
            StockItemCreateCode::CatalogueChanged,
        ),
        (
            DraftRefusal::GroupsUnreadable(unreadable),
            StockItemCreateCode::GroupsUnreadable,
        ),
        (
            DraftRefusal::ParentNotInBook,
            StockItemCreateCode::ParentNotInBook,
        ),
        (
            DraftRefusal::UnitsUnreadable(unreadable),
            StockItemCreateCode::UnitsUnreadable,
        ),
        (
            DraftRefusal::UnitNotInBook,
            StockItemCreateCode::UnitNotInBook,
        ),
        (
            DraftRefusal::GstEntriesNotMeasured { entries: 0 },
            StockItemCreateCode::ShapeNotMeasured,
        ),
        (
            DraftRefusal::GstSourceNotMeasured,
            StockItemCreateCode::ShapeNotMeasured,
        ),
        (
            DraftRefusal::ItcNotMeasured,
            StockItemCreateCode::ShapeNotMeasured,
        ),
        (
            DraftRefusal::StateNotAny,
            StockItemCreateCode::ShapeNotMeasured,
        ),
        (
            DraftRefusal::DutyHeadNotMeasured { row: 0 },
            StockItemCreateCode::ShapeNotMeasured,
        ),
        (
            DraftRefusal::ValuationNotMeasured { row: 0 },
            StockItemCreateCode::ShapeNotMeasured,
        ),
        (
            DraftRefusal::RateNotMeasured { row: 0 },
            StockItemCreateCode::ShapeNotMeasured,
        ),
        (
            DraftRefusal::HsnEntriesNotMeasured { entries: 0 },
            StockItemCreateCode::ShapeNotMeasured,
        ),
        (
            DraftRefusal::HsnNotMeasured,
            StockItemCreateCode::ShapeNotMeasured,
        ),
    ] {
        assert_eq!(draft_refusal_code(&refusal), code);
    }

    assert_eq!(
        create_render_refusal_code(&CreateRenderRefusal::TextNotPlain {
            field: DraftText::HsnDescription,
        }),
        StockItemCreateCode::TextNotPlain
    );
    assert_eq!(
        create_render_refusal_code(&CreateRenderRefusal::TextNeedsEscaping {
            field: DraftText::HsnDescription,
        }),
        StockItemCreateCode::TextNeedsEscaping
    );

    assert_eq!(
        stock_item_outcome_code(&StockItemImportOutcome::ApplicationFailure),
        Some(StockItemCreateCode::CreateApplicationFailure)
    );
    assert_eq!(
        stock_item_outcome_code(&StockItemImportOutcome::AlteredExisting {
            counters,
            line_errors: 0,
        }),
        Some(StockItemCreateCode::AlteredExisting)
    );
    assert_eq!(
        stock_item_outcome_code(&StockItemImportOutcome::NotClean {
            counters,
            line_errors: 0,
        }),
        Some(StockItemCreateCode::CountersNotClean)
    );
    assert_eq!(
        stock_item_outcome_code(&StockItemImportOutcome::Unreported),
        Some(StockItemCreateCode::CountersUnreported)
    );

    assert_eq!(
        stock_item_read_back_code(&ReadBackVerdict::Differs {
            fields: vec![SiblingField::Parent],
        }),
        Some(StockItemCreateCode::ReadBackDiffers)
    );
    assert_eq!(
        stock_item_read_back_code(&ReadBackVerdict::NotFoundOnce { rows: 1 }),
        Some(StockItemCreateCode::ReadBackNotFoundOnce)
    );
    assert_eq!(
        stock_item_read_back_code(&ReadBackVerdict::NotNew),
        Some(StockItemCreateCode::ReadBackNotNew)
    );
    assert_eq!(
        stock_item_read_back_code(&ReadBackVerdict::OtherCompany),
        Some(StockItemCreateCode::ReadBackOtherCompany)
    );
}

/// Every pin of every enum, as the failure code its variant makes, beside the pinned string.
fn pinned() -> Vec<(FailureCode, &'static str)> {
    fn of<C: Copy + Into<FailureCode>>(
        pins: &[(C, &'static str)],
    ) -> Vec<(FailureCode, &'static str)> {
        pins.iter()
            .map(|(code, text)| ((*code).into(), *text))
            .collect()
    }
    let pins = [
        of(GST_UNCERTAIN),
        of(READ),
        of(PROPOSALS),
        of(GST_STATUS),
        of(GST_BY_LEDGER),
        of(REGISTER),
        of(OUTSTANDINGS),
        of(INVOICE_NUMBER_AFTER_POST),
        of(MOVEMENT),
        of(PRESENCE),
        of(TIE_OUT),
        of(STOCK_ITEM_CREATE),
    ]
    .concat();
    #[cfg(feature = "lab-writes")]
    let pins = [pins, of(LAB_MASTERS)].concat();
    pins
}

/// Every declared variant of every enum, as the failure code it makes.
fn declared() -> Vec<FailureCode> {
    fn of<C: Copy + Into<FailureCode>>(all: &[C]) -> Vec<FailureCode> {
        all.iter().map(|code| (*code).into()).collect()
    }
    let codes = [
        of(GstUncertainCode::ALL),
        of(ReadCode::ALL),
        of(ProposalsCode::ALL),
        of(GstStatusCode::ALL),
        of(GstByLedgerCode::ALL),
        of(RegisterCode::ALL),
        of(OutstandingsCode::ALL),
        of(InvoiceNumberAfterPostCode::ALL),
        of(MovementCode::ALL),
        of(PresenceCode::ALL),
        of(TieOutCode::ALL),
        of(StockItemCreateCode::ALL),
    ]
    .concat();
    #[cfg(feature = "lab-writes")]
    let codes = [codes, of(LabMastersCode::ALL)].concat();
    codes
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
    assert_eq!(
        variants(INVOICE_NUMBER_AFTER_POST),
        InvoiceNumberAfterPostCode::ALL
    );
    assert_eq!(variants(PROPOSALS), ProposalsCode::ALL);
    assert_eq!(variants(MOVEMENT), MovementCode::ALL);
    assert_eq!(variants(PRESENCE), PresenceCode::ALL);
    assert_eq!(variants(TIE_OUT), TieOutCode::ALL);
    assert_eq!(variants(STOCK_ITEM_CREATE), StockItemCreateCode::ALL);
    #[cfg(feature = "lab-writes")]
    assert_eq!(variants(LAB_MASTERS), LabMastersCode::ALL);
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
    check_pins(
        INVOICE_NUMBER_AFTER_POST,
        InvoiceNumberAfterPostCode::as_str,
    );
    check_pins(PROPOSALS, ProposalsCode::as_str);
    check_pins(MOVEMENT, MovementCode::as_str);
    check_pins(PRESENCE, PresenceCode::as_str);
    check_pins(TIE_OUT, TieOutCode::as_str);
    check_pins(STOCK_ITEM_CREATE, StockItemCreateCode::as_str);
    #[cfg(feature = "lab-writes")]
    check_pins(LAB_MASTERS, LabMastersCode::as_str);
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
    // The three lab master codes are declared only in a build with `lab-writes`.
    #[cfg(not(feature = "lab-writes"))]
    let expected = 75;
    #[cfg(feature = "lab-writes")]
    let expected = 78;
    assert_eq!(codes.len(), expected, "the declared codes");
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

/// The read-back's registration differences (#2464), declared new, so each string is pinned as
/// declared, with every check above made for them too.
const INVOICE_REGISTRATION: &[(InvoiceRegistrationCode, &str)] = &[
    (
        InvoiceRegistrationCode::Missing,
        "invoice_registration_missing",
    ),
    (
        InvoiceRegistrationCode::Differs,
        "invoice_registration_differs",
    ),
];

#[test]
fn the_registration_codes_are_pinned_and_have_the_receipts_code_shape() {
    let pinned: Vec<InvoiceRegistrationCode> =
        INVOICE_REGISTRATION.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, InvoiceRegistrationCode::ALL);
    let mut seen = GstUncertainCode::ALL
        .iter()
        .map(|code| code.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for (code, string) in INVOICE_REGISTRATION {
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

/// The read-back's item-line differences (#1379), declared new, so each string is pinned as
/// declared, with every check above made for them too.
const INVOICE_ITEM_LINE: &[(InvoiceItemLineCode, &str)] = &[
    (InvoiceItemLineCode::NotSent, "invoice_line_item_not_sent"),
    (InvoiceItemLineCode::Several, "invoice_line_items_several"),
    (InvoiceItemLineCode::Differs, "invoice_line_item_differs"),
    (InvoiceItemLineCode::Batch, "invoice_line_item_batch"),
];

#[test]
fn the_item_line_codes_are_pinned_and_have_the_receipts_code_shape() {
    let pinned: Vec<InvoiceItemLineCode> =
        INVOICE_ITEM_LINE.iter().map(|(code, _)| *code).collect();
    assert_eq!(pinned, InvoiceItemLineCode::ALL);
    let mut seen = GstUncertainCode::ALL
        .iter()
        .map(|code| code.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for (code, string) in INVOICE_ITEM_LINE {
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

/// The company-read code, pinned as above (#2433).
#[test]
fn the_company_read_code_is_pinned() {
    let pinned: &[(CompanyReadCode, &str)] =
        &[(CompanyReadCode::NameInvalid, "company_name_invalid")];
    assert_eq!(
        pinned.iter().map(|(code, _)| *code).collect::<Vec<_>>(),
        CompanyReadCode::ALL
    );
    for (code, string) in pinned {
        assert_eq!(code.as_str(), *string);
        assert_eq!(FailureCode::from(*code).as_str(), *string);
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
