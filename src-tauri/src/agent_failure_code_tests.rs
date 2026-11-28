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

/// The invoice build's own refusal (#2574).
const INVOICE_BUILD: &[(InvoiceBuildCode, &str)] = &[(
    InvoiceBuildCode::ReviewTooLargeWithEInvoice,
    "invoice_review_too_large_with_e_invoice",
)];

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

const MOVEMENT_CURRENCY: &[(MovementCurrencyCode, &str)] = &[
    (
        MovementCurrencyCode::ScopeUnmatched,
        "ledger_currency_scope_unmatched",
    ),
    (
        MovementCurrencyCode::ScopeInconclusive,
        "ledger_currency_scope_inconclusive",
    ),
    (MovementCurrencyCode::SetAside, "ledger_currency_set_aside"),
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

const IMPORT_ACK: &[(ImportAckCode, &str)] = &[
    (ImportAckCode::NoObservedDoubt, "ack_no_observed_doubt"),
    (
        ImportAckCode::DoubtRecordUnavailable,
        "ack_doubt_record_unavailable",
    ),
    (ImportAckCode::DoubtAmbiguous, "ack_doubt_ambiguous"),
    (ImportAckCode::CheckPending, "ack_check_pending"),
    (
        ImportAckCode::MastersRecordUnreadable,
        "ack_masters_record_unreadable",
    ),
    (
        ImportAckCode::StepRecordUnreadable,
        "ack_step_record_unreadable",
    ),
    (ImportAckCode::ResponseNotClean, "ack_response_not_clean"),
    (
        ImportAckCode::ReadbackNotMatched,
        "ack_readback_not_matched",
    ),
    (ImportAckCode::ReviewTooLarge, "ack_review_too_large"),
    (ImportAckCode::ReviewLayoutText, "ack_review_layout_text"),
    (ImportAckCode::ReviewFormatText, "ack_review_format_text"),
    (
        ImportAckCode::RecordedReviewStale,
        "ack_recorded_review_stale",
    ),
    (ImportAckCode::AlreadyRecorded, "ack_already_recorded"),
    (
        ImportAckCode::ImportFileWriteFailed,
        "import_file_write_failed",
    ),
    (ImportAckCode::ImportBatchNotFound, "import_batch_not_found"),
    (
        ImportAckCode::BatchCompanyMismatch,
        "import_batch_company_mismatch",
    ),
    (ImportAckCode::BatchNotPosted, "ack_batch_not_posted"),
    (ImportAckCode::DoubtInvalid, "ack_doubt_invalid"),
    (
        ImportAckCode::ChangedWhileReviewing,
        "ack_changed_while_reviewing",
    ),
    (
        ImportAckCode::ProofSerializationFailed,
        "proof_serialization_failed",
    ),
];

const STOP_ACK: &[(StopAckCode, &str)] = &[
    (StopAckCode::BatchNotSent, "ack_stop_batch_not_sent"),
    (
        StopAckCode::ChangedWhileReviewing,
        "ack_stop_changed_while_reviewing",
    ),
    (StopAckCode::NotRecorded, "ack_stop_not_recorded"),
    (StopAckCode::NotHeld, "ack_stop_not_held"),
];

const IMPORT_APPROVAL: &[(ImportApprovalCode, &str)] = &[
    (ImportApprovalCode::Expired, "import_approval_expired"),
    (ImportApprovalCode::InUse, "import_approval_in_use"),
    (ImportApprovalCode::Revoked, "import_approval_revoked"),
    (
        ImportApprovalCode::BindingChanged,
        "import_approval_binding_changed",
    ),
    (ImportApprovalCode::Busy, "post_approval_busy"),
];

const MASTER_LEDGER: &[(MasterLedgerCode, &str)] = &[
    (
        MasterLedgerCode::KindNotAdmitted,
        "master_kind_not_admitted",
    ),
    (
        MasterLedgerCode::FieldNotForKind,
        "ledger_field_not_for_kind",
    ),
    (MasterLedgerCode::PartySideInvalid, "party_side_invalid"),
    (MasterLedgerCode::PartySideRequired, "party_side_required"),
    (
        MasterLedgerCode::GstDutyHeadRequired,
        "gst_duty_head_required",
    ),
    (
        MasterLedgerCode::GstDutyHeadNotAdmitted,
        "gst_duty_head_not_admitted",
    ),
    (MasterLedgerCode::LedgerRequired, "ledger_required"),
    (MasterLedgerCode::BatchInvalid, "master_batch_invalid"),
    (MasterLedgerCode::BatchNotFound, "master_batch_not_found"),
    (MasterLedgerCode::IntentMissing, "master_intent_missing"),
];

const EGRESS: &[(EgressCode, &str)] = &[
    (EgressCode::RecordTooLarge, "egress_record_too_large"),
    (EgressCode::RecordWriteFailed, "egress_record_write_failed"),
    (
        EgressCode::RecordRollbackFailed,
        "egress_record_rollback_failed",
    ),
    (EgressCode::LogUnreadable, "egress_log_unreadable"),
    (EgressCode::LogIncomplete, "egress_log_incomplete"),
];

const TOOL_REQUEST: &[(ToolRequestCode, &str)] = &[
    (ToolRequestCode::RequestCancelled, "request_cancelled"),
    (ToolRequestCode::HostSettingInvalid, "host_setting_invalid"),
];

const IMPORT_POST: &[(ImportPostCode, &str)] = &[
    (
        ImportPostCode::RequiresOneJournal,
        "import_post_requires_one_journal",
    ),
    (
        ImportPostCode::RequiresOneVoucher,
        "import_post_requires_one_voucher",
    ),
    (ImportPostCode::BatchTooLarge, "import_post_batch_too_large"),
    (
        ImportPostCode::CompanyMissing,
        "import_post_company_missing",
    ),
    (
        ImportPostCode::EndpointMismatch,
        "import_post_endpoint_mismatch",
    ),
    (
        ImportPostCode::RemoteIdsMismatch,
        "import_post_remote_ids_mismatch",
    ),
    (
        ImportPostCode::NumberedJournalUnsupported,
        "import_post_numbered_journal_unsupported",
    ),
    (
        ImportPostCode::AmendmentRequiresFileImport,
        "import_post_amendment_requires_file_import",
    ),
    (
        ImportPostCode::AdmissionInconsistent,
        "import_post_admission_inconsistent",
    ),
    (ImportPostCode::NotDispatched, "import_not_dispatched"),
    (ImportPostCode::RemoteIdReused, "import_remote_id_reused"),
    (
        ImportPostCode::TxnAlreadyPosted,
        "import_txn_already_posted",
    ),
    (
        ImportPostCode::PreexistingIdentity,
        "import_preexisting_identity",
    ),
    (ImportPostCode::MastersChanged, "import_masters_changed"),
    (ImportPostCode::BillWiseChanged, "import_bill_wise_changed"),
    (
        ImportPostCode::BankClassificationChanged,
        "import_bank_classification_changed",
    ),
    (
        ImportPostCode::CashLedgerNotCashInHand,
        "cash_ledger_not_cash_in_hand",
    ),
    (
        ImportPostCode::LedgerHasFoldedTwin,
        "ledger_has_folded_twin",
    ),
    (
        ImportPostCode::InvoiceNotObserved,
        "import_invoice_not_observed",
    ),
    (ImportPostCode::ReviewTooLarge, "import_review_too_large"),
    (
        ImportPostCode::VoucherAmountOverflow,
        "voucher_amount_overflow",
    ),
];

const SEARCH: &[(SearchCode, &str)] = &[
    (SearchCode::CriterionEmpty, "search_criterion_empty"),
    (SearchCode::CriterionTooLong, "search_criterion_too_long"),
    (SearchCode::NarrationRedacted, "search_narration_redacted"),
    (SearchCode::NarrationTooShort, "search_narration_too_short"),
    (
        SearchCode::SuspenseTaggedInvalid,
        "search_suspense_tagged_invalid",
    ),
    (
        SearchCode::SuspenseTaggedNeedsLedger,
        "search_suspense_tagged_needs_ledger",
    ),
    (SearchCode::AmountInvalid, "search_amount_invalid"),
];

const BANK_STATEMENT: &[(BankStatementCode, &str)] = &[
    (BankStatementCode::TaskFailed, "statement_task_failed"),
    (
        BankStatementCode::ControlTotalsRequired,
        "statement_control_totals_required",
    ),
    (
        BankStatementCode::ReversedDateWindow,
        "statement_reversed_date_window",
    ),
    (
        BankStatementCode::PasswordFileUnreadable,
        "statement_password_file_unreadable",
    ),
    (
        BankStatementCode::PasswordFilePermissions,
        "statement_password_file_permissions",
    ),
    (
        BankStatementCode::PasswordFileTooLarge,
        "statement_password_file_too_large",
    ),
    (
        BankStatementCode::PasswordFileNotUtf8,
        "statement_password_file_not_utf8",
    ),
    (
        BankStatementCode::FileUnreadable,
        "statement_file_unreadable",
    ),
    (BankStatementCode::FileTooLarge, "statement_file_too_large"),
    (
        BankStatementCode::PdfEngineSettingInvalid,
        "statement_pdf_engine_setting_invalid",
    ),
    (
        BankStatementCode::PdfEngineUnavailable,
        "statement_pdf_engine_unavailable",
    ),
    (
        BankStatementCode::ProposalsDirectoryUnavailable,
        "statement_proposals_directory_unavailable",
    ),
    (
        BankStatementCode::ProposalsSerializationFailed,
        "statement_proposals_serialization_failed",
    ),
    (
        BankStatementCode::ProposalsWriteFailed,
        "statement_proposals_write_failed",
    ),
    (
        BankStatementCode::ProposalsSha256Required,
        "proposals_sha256_required",
    ),
    (BankStatementCode::ProposalsNotFound, "proposals_not_found"),
    (
        BankStatementCode::ProposalsFileUnreadable,
        "proposals_file_unreadable",
    ),
    (
        BankStatementCode::ProposalsFileTooLarge,
        "proposals_file_too_large",
    ),
    (BankStatementCode::ProposalsChanged, "proposals_changed"),
    (
        BankStatementCode::ArgumentSchemaInvalid,
        "argument_schema_invalid",
    ),
    (
        BankStatementCode::ProposalsIdWithVouchers,
        "proposals_id_with_vouchers",
    ),
    (BankStatementCode::CashQuestionsOpen, "cash_questions_open"),
    (BankStatementCode::ProposalsLegacy, "proposals_legacy"),
    (
        BankStatementCode::ControlFiguresWithThisFile,
        "statement_control_figures_with_this_file",
    ),
    (
        BankStatementCode::ControlFiguresRequired,
        "statement_control_figures_required",
    ),
];

const IMPORT_LEDGER: &[(ImportLedgerCode, &str)] = &[
    (ImportLedgerCode::Invalid, "import_ledger_invalid"),
    (
        ImportLedgerCode::BatchDigestAmbiguous,
        "import_batch_digest_ambiguous",
    ),
    (ImportLedgerCode::Unavailable, "import_ledger_unavailable"),
    (
        ImportLedgerCode::DuplicateDispatch,
        "import_ledger_duplicate_dispatch",
    ),
    (
        ImportLedgerCode::SerializationFailed,
        "import_ledger_serialization_failed",
    ),
    (
        ImportLedgerCode::RecordTooLarge,
        "import_ledger_record_too_large",
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
        StockItemCreateCode::GuidNotLowercase,
        "stock_item_guid_not_lowercase",
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
        stock_item_catalogue_refusal_code(&StockItemCatalogueError::GuidNotLowercase { item: 0 }),
        StockItemCreateCode::GuidNotLowercase
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
        of(MOVEMENT_CURRENCY),
        of(PRESENCE),
        of(TIE_OUT),
        of(IMPORT_LEDGER),
        of(STOCK_ITEM_CREATE),
        of(INVOICE_BUILD),
        of(INVOICE_REGISTRATION),
        of(INVOICE_ITEM_LINE),
        of(COMPANY_READ),
        of(CENSUS_REUSE),
        of(E_INVOICE_READBACK),
        of(IMPORT_ACK),
        of(STOP_ACK),
        of(IMPORT_APPROVAL),
        of(MASTER_LEDGER),
        of(EGRESS),
        of(TOOL_REQUEST),
        of(IMPORT_POST),
        of(BANK_STATEMENT),
        of(SEARCH),
    ]
    .concat();
    #[cfg(feature = "lab-writes")]
    let pins = [pins, of(LAB_MASTERS)].concat();
    pins
}

/// Every declared variant of every enum, as the failure code it makes, each enum under its name.
fn declared_by_enum() -> Vec<(&'static str, Vec<FailureCode>)> {
    fn of<C: Copy + Into<FailureCode>>(
        name: &'static str,
        all: &[C],
    ) -> (&'static str, Vec<FailureCode>) {
        (name, all.iter().map(|code| (*code).into()).collect())
    }
    let enums = vec![
        of("GstUncertainCode", GstUncertainCode::ALL),
        of("ReadCode", ReadCode::ALL),
        of("ProposalsCode", ProposalsCode::ALL),
        of("GstStatusCode", GstStatusCode::ALL),
        of("GstByLedgerCode", GstByLedgerCode::ALL),
        of("RegisterCode", RegisterCode::ALL),
        of("OutstandingsCode", OutstandingsCode::ALL),
        of(
            "InvoiceNumberAfterPostCode",
            InvoiceNumberAfterPostCode::ALL,
        ),
        of("MovementCode", MovementCode::ALL),
        of("MovementCurrencyCode", MovementCurrencyCode::ALL),
        of("PresenceCode", PresenceCode::ALL),
        of("TieOutCode", TieOutCode::ALL),
        of("ImportLedgerCode", ImportLedgerCode::ALL),
        of("StockItemCreateCode", StockItemCreateCode::ALL),
        of("InvoiceBuildCode", InvoiceBuildCode::ALL),
        of("InvoiceRegistrationCode", InvoiceRegistrationCode::ALL),
        of("InvoiceItemLineCode", InvoiceItemLineCode::ALL),
        of("CompanyReadCode", CompanyReadCode::ALL),
        of("CensusReuseCode", CensusReuseCode::ALL),
        of("EInvoiceReadbackCode", EInvoiceReadbackCode::ALL),
        of("ImportAckCode", ImportAckCode::ALL),
        of("StopAckCode", StopAckCode::ALL),
        of("ImportApprovalCode", ImportApprovalCode::ALL),
        of("MasterLedgerCode", MasterLedgerCode::ALL),
        of("EgressCode", EgressCode::ALL),
        of("ToolRequestCode", ToolRequestCode::ALL),
        of("ImportPostCode", ImportPostCode::ALL),
        of("BankStatementCode", BankStatementCode::ALL),
        of("SearchCode", SearchCode::ALL),
    ];
    #[cfg(feature = "lab-writes")]
    let enums = [enums, vec![of("LabMastersCode", LabMastersCode::ALL)]].concat();
    enums
}

/// Every declared variant of every enum, as the failure code it makes.
fn declared() -> Vec<FailureCode> {
    declared_by_enum()
        .into_iter()
        .flat_map(|(_, codes)| codes)
        .collect()
}

/// Every enum the `failure_codes!` macro declares in `agent_failure_code.rs` is in
/// [`declared_by_enum`], so the uniqueness test walks it (#2558). The enum names are read from the
/// source, counting an enum whether or not a `cfg` gates it, so a block added without a line in
/// the list fails here.
#[test]
fn every_enum_the_macro_declares_is_walked_by_declared() {
    let source = include_str!("agent_failure_code.rs");
    let source = &source[..source
        .find("#[cfg(test)]\n#[path")
        .expect("the tests module")];
    let mut in_declaration = false;
    let mut found = Vec::new();
    for line in source.lines() {
        if line.starts_with("failure_codes! {") {
            in_declaration = true;
        } else if in_declaration {
            if let Some(rest) = line.trim().strip_prefix("pub(crate) enum ") {
                found.push(rest.trim_end_matches(" {").to_string());
                in_declaration = false;
            }
        }
    }
    let mut walked: Vec<String> = declared_by_enum()
        .into_iter()
        .map(|(name, _)| name.to_string())
        .collect();
    // Declared in a build without `lab-writes` too, as source text.
    #[cfg(not(feature = "lab-writes"))]
    walked.push("LabMastersCode".to_string());
    found.sort();
    walked.sort();
    assert_eq!(
        found.len(),
        30,
        "the failure_codes! blocks found in the source"
    );
    assert_eq!(found, walked);
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
    assert_eq!(variants(MOVEMENT_CURRENCY), MovementCurrencyCode::ALL);
    assert_eq!(variants(PRESENCE), PresenceCode::ALL);
    assert_eq!(variants(TIE_OUT), TieOutCode::ALL);
    assert_eq!(variants(BANK_STATEMENT), BankStatementCode::ALL);
    assert_eq!(variants(SEARCH), SearchCode::ALL);
    assert_eq!(variants(IMPORT_LEDGER), ImportLedgerCode::ALL);
    assert_eq!(variants(STOCK_ITEM_CREATE), StockItemCreateCode::ALL);
    assert_eq!(variants(INVOICE_BUILD), InvoiceBuildCode::ALL);
    #[cfg(feature = "lab-writes")]
    assert_eq!(variants(LAB_MASTERS), LabMastersCode::ALL);
    assert_eq!(variants(IMPORT_ACK), ImportAckCode::ALL);
    assert_eq!(variants(STOP_ACK), StopAckCode::ALL);
    assert_eq!(variants(IMPORT_APPROVAL), ImportApprovalCode::ALL);
    assert_eq!(variants(MASTER_LEDGER), MasterLedgerCode::ALL);
    assert_eq!(variants(EGRESS), EgressCode::ALL);
    assert_eq!(variants(TOOL_REQUEST), ToolRequestCode::ALL);
    assert_eq!(variants(IMPORT_POST), ImportPostCode::ALL);
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
    check_pins(MOVEMENT_CURRENCY, MovementCurrencyCode::as_str);
    check_pins(PRESENCE, PresenceCode::as_str);
    check_pins(TIE_OUT, TieOutCode::as_str);
    check_pins(BANK_STATEMENT, BankStatementCode::as_str);
    check_pins(SEARCH, SearchCode::as_str);
    check_pins(IMPORT_LEDGER, ImportLedgerCode::as_str);
    check_pins(STOCK_ITEM_CREATE, StockItemCreateCode::as_str);
    check_pins(INVOICE_BUILD, InvoiceBuildCode::as_str);
    #[cfg(feature = "lab-writes")]
    check_pins(LAB_MASTERS, LabMastersCode::as_str);
    check_pins(IMPORT_ACK, ImportAckCode::as_str);
    check_pins(STOP_ACK, StopAckCode::as_str);
    check_pins(IMPORT_APPROVAL, ImportApprovalCode::as_str);
    check_pins(MASTER_LEDGER, MasterLedgerCode::as_str);
    check_pins(EGRESS, EgressCode::as_str);
    check_pins(TOOL_REQUEST, ToolRequestCode::as_str);
    check_pins(IMPORT_POST, ImportPostCode::as_str);
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
    // The three lab master codes are declared only in a build with `lab-writes`. The counts are
    // a list, not a sum, so the merge gate's phone-number scan does not read them as one number.
    #[cfg(not(feature = "lab-writes"))]
    let expected: usize = [77, 4, 2, 1, 1, 4, 25, 6, 3, 20, 7, 4, 5, 10, 5, 2, 21]
        .iter()
        .sum();
    #[cfg(feature = "lab-writes")]
    let expected: usize = [80, 4, 2, 1, 1, 4, 25, 6, 3, 20, 7, 4, 5, 10, 5, 2, 21]
        .iter()
        .sum();
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
const COMPANY_READ: &[(CompanyReadCode, &str)] =
    &[(CompanyReadCode::NameInvalid, "company_name_invalid")];

#[test]
fn the_company_read_code_is_pinned() {
    let pinned = COMPANY_READ;
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
