//! Failure codes as typed values (#1942). A code a tool failure carries can reach the egress
//! receipt, so it is declared here, in a closed enum per module, and a [`FailureCode`] is made only
//! from one of them: no string a failure site writes can become one. A site not yet converted still
//! builds its code from a string (`From<String>` for `ToolFailure`); `failure_code_sites_tests`
//! counts those per file, and the count may only fall.

use crate::tally::stock_item_create::{
    CreateRenderRefusal, DraftRefusal, ItemNameNotAbsent, NotGoodsSibling, SiblingRefusal,
    StockItemCatalogueError,
};
use crate::tally::stock_item_verified::{ReadBackVerdict, StockItemImportOutcome};

/// A failure code made from a declared variant. Its constructor is private to this module, so
/// every value is one of the strings the enums below declare.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FailureCode(&'static str);

impl FailureCode {
    pub(crate) fn as_str(self) -> &'static str {
        self.0
    }
}

/// Declares a closed set of failure codes, each variant with its exact string, and the only ways
/// from it to a [`FailureCode`] and to a tool failure.
macro_rules! failure_codes {
    (
        $(#[$meta:meta])*
        pub(crate) enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident = $code:literal,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(crate) enum $name {
            $($(#[$variant_meta])* $variant,)+
        }

        impl $name {
            /// Every variant, for the test that pins each one's string.
            #[cfg(test)]
            pub(crate) const ALL: &'static [$name] = &[$($name::$variant,)+];

            pub(crate) fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $code,)+
                }
            }
        }

        impl From<$name> for FailureCode {
            fn from(code: $name) -> Self {
                FailureCode(code.as_str())
            }
        }

        impl From<$name> for super::ToolFailure {
            fn from(code: $name) -> Self {
                FailureCode::from(code).into()
            }
        }
    };
}

failure_codes! {
    /// The refusals of `gst_uncertain_vouchers` itself.
    pub(crate) enum GstUncertainCode {
        /// The count read found more vouchers in the window than the tool reads.
        WindowTooLarge = "gst_uncertain_window_too_large",
        /// The wide read returned fewer vouchers than the count read found.
        WideReadShort = "gst_uncertain_wide_read_short",
        /// More vouchers to list than this server's row limit.
        RowsOverLimit = "gst_uncertain_rows_over_limit",
    }
}

failure_codes! {
    /// The differences an invoice's read-back names for a stock item Tally holds under one of its
    /// ledger entries (#1379).
    pub(crate) enum InvoiceItemLineCode {
        /// An entry holds a stock item, and the build sent no item line.
        NotSent = "invoice_line_item_not_sent",
        /// An entry holds more than one stock item.
        Several = "invoice_line_items_several",
        /// The item's side or amount is not its entry's, or it carries a rate or a quantity.
        Differs = "invoice_line_item_differs",
        /// The item does not hold exactly one batch allocation, naming a godown and a batch, for
        /// its amount.
        Batch = "invoice_line_item_batch",
    }
}

failure_codes! {
    /// The differences an invoice's read-back names for the company registration Tally filed it
    /// under (#2464).
    pub(crate) enum InvoiceRegistrationCode {
        /// The registration's name or its GSTIN did not come back, or came back empty.
        Missing = "invoice_registration_missing",
        /// The GSTIN is not a valid GSTIN of the company's state the build recorded.
        Differs = "invoice_registration_differs",
    }
}

failure_codes! {
    /// The differences of the number's month reads sent again after a clean read-back (#2162). Each
    /// is a read-back difference, so `posted_verified` is never given; `InAnotherType` is followed
    /// in the difference by the names of the Sales types that hold the number.
    pub(crate) enum InvoiceNumberAfterPostCode {
        /// A month read after the post failed.
        ReadFailed = "invoice_number_read_failed",
        /// The posted type, or a returned row's type, could not be read.
        TypeUnread = "invoice_number_type_unread",
        /// A Sales voucher of another type holds the number.
        InAnotherType = "invoice_number_in_another_type",
        /// No Sales-class voucher holds the number, though the read-back found the invoice.
        ReadMissedInvoice = "invoice_number_read_missed_invoice",
        /// More than one Sales voucher of the posted type holds the number.
        NotUnique = "invoice_number_not_unique",
    }
}

failure_codes! {
    /// The refusals of a read for a listed company, before any request names it.
    pub(crate) enum CompanyReadCode {
        /// The company's name, as Tally reported it, is one a request cannot carry (#2433).
        NameInvalid = "company_name_invalid",
    }
}

failure_codes! {
    /// The refusal of a `vouchers` read that took a held census (#2399).
    pub(crate) enum CensusReuseCode {
        /// The read used a census counted by an earlier call, and a part it read
        /// was not the vouchers that census described. The census is dropped; the
        /// next call counts afresh. `cause` says how the part differed.
        Mismatch = "census_reused_mismatch",
    }
}

failure_codes! {
    /// The refusals of a `vouchers` search's criteria.
    pub(crate) enum SearchCode {
        /// A search criterion is blank.
        CriterionEmpty = "search_criterion_empty",
        /// A search criterion is longer than a search term may be.
        CriterionTooLong = "search_criterion_too_long",
        /// A narration phrase, or the suspense-tag filter, where narrations are withheld from the
        /// assistant.
        NarrationRedacted = "search_narration_redacted",
        /// A narration phrase whose search key is shorter than the shortest a search accepts.
        NarrationTooShort = "search_narration_too_short",
        /// `suspense_tagged` is given as anything but `true`.
        SuspenseTaggedInvalid = "search_suspense_tagged_invalid",
        /// `suspense_tagged` is given without a `ledger`, or with a blank one.
        SuspenseTaggedNeedsLedger = "search_suspense_tagged_needs_ledger",
        /// The `amount` criterion is not a plain unsigned decimal above zero, or does not read as
        /// one exactly.
        AmountInvalid = "search_amount_invalid",
    }
}

failure_codes! {
    /// Codes that several read tools write for the same failure, declared once.
    pub(crate) enum ReadCode {
        /// The window's `from` is after its `to`.
        InvalidDateRange = "invalid_date_range",
        /// An answer lacks a part its read's shape requires.
        ReadProtocolInvalid = "agent_read_protocol_invalid",
        /// An amount is not an exact decimal, or a sum of amounts overflows.
        VoucherAmountInvalid = "voucher_amount_invalid",
        /// A voucher names a ledger the masters read did not list, or the read lists one twice.
        LedgerSnapshotDrifted = "ledger_snapshot_drifted",
        /// Tally answered with a voucher outside the window the request asked for.
        WindowNotHonoured = "window_not_honoured",
        /// The host's current date is not a date.
        CurrentDateInvalid = "current_date_invalid",
        /// No `as_of` was given and the host's current date, which stands in for it, is not a date.
        InvalidAsOf = "invalid_as_of",
        /// The verified company's identity lacks a part the read needs.
        CompanyIdentityIncomplete = "company_identity_incomplete",
        /// A date is not a date.
        InvalidDate = "invalid_date",
        /// A voucher's entries do not sum to zero.
        VoucherEntriesUnbalanced = "voucher_entries_unbalanced",
    }
}

failure_codes! {
    /// Codes about a saved proposals file, or the vouchers a tool is given, that several tools
    /// write for the same failure.
    pub(crate) enum ProposalsCode {
        /// The proposals file does not read as the shape it was written in.
        FileInvalid = "proposals_file_invalid",
        /// No `proposals_id` was given.
        IdRequired = "proposals_id_required",
        /// No `vouchers` were given.
        VouchersRequired = "vouchers_required",
    }
}

failure_codes! {
    /// The refusals of `ledger_movement` itself.
    pub(crate) enum MovementCode {
        /// The vouchers changed between the reads the movement is built from.
        VoucherSnapshotDrifted = "voucher_snapshot_drifted",
        /// The ledger's opening balance was not observed, so the answer is partial.
        OpeningBalanceNotObserved = "opening_balance_not_observed",
        /// The ledger's movement read failed.
        ReadFailed = "ledger_movement_read_failed",
    }
}

failure_codes! {
    /// The refusals `ledger_movement` adds for a book with several Currency masters, read from its
    /// first day (#683). Its date range and drift refusals are [`ReadCode`]'s `InvalidDateRange`
    /// and `LedgerSnapshotDrifted`, the same strings.
    pub(crate) enum MovementCurrencyCode {
        /// The currency-scoped Trial Balance and the movement's catalogue name different ledgers.
        ScopeUnmatched = "ledger_currency_scope_unmatched",
        /// A composite opening or window entry is on a ledger the Trial Balance reads as plain.
        ScopeInconclusive = "ledger_currency_scope_inconclusive",
        /// The ledger asked for is set aside, so no figure is given for it.
        SetAside = "ledger_currency_set_aside",
    }
}

failure_codes! {
    /// The refusals of `voucher_presence` itself.
    pub(crate) enum PresenceCode {
        /// The ledger catalogue the check needs was not read whole.
        LedgerCatalogueIncomplete = "ledger_catalogue_incomplete",
        /// The numbering the check was given differs from the book's.
        NumberingMethodDiffers = "presence_numbering_method_differs_from_book",
        /// No `numbering` was given.
        NumberingRequired = "numbering_required",
        /// A voucher to check lacks a part of its import identity.
        ImportIdentityIncomplete = "presence_import_identity_incomplete",
    }
}

failure_codes! {
    /// The refusals of `statement_tie_out` itself.
    pub(crate) enum TieOutCode {
        /// A sum or difference of the tie-out's amounts is out of range.
        ArithmeticOutOfRange = "tie_out_arithmetic_out_of_range",
        /// The ledger's opening balance is not an exact decimal.
        LedgerOpeningInvalid = "ledger_opening_invalid",
        /// The ledger's opening balance is missing.
        LedgerOpeningMissing = "ledger_opening_missing",
        /// The ledger catalogue names the ledger more than once.
        LedgerNameDuplicated = "ledger_name_duplicated_in_catalogue",
    }
}

failure_codes! {
    /// The refusals of `gst_status` itself.
    pub(crate) enum GstStatusCode {
        /// `voucher_type_name` is not a name the request can carry.
        TypeNameInvalid = "gst_status_type_name_invalid",
        /// The window is longer than the tool reads.
        WindowTooLong = "gst_status_window_too_long",
        /// The book's voucher mark is above the bound the selection formula is read under.
        BookTooLarge = "gst_status_book_too_large",
        /// A voucher in the answer has no date that parses.
        ReadVoucherUnidentified = "gst_status_read_voucher_unidentified",
        /// Tally answered with a voucher of another type.
        TypeNotHonoured = "gst_status_type_not_honoured",
        /// At least one voucher's flags were unread, so the answer is partial.
        VoucherUnread = "gst_status_voucher_unread",
    }
}

failure_codes! {
    /// The refusal of `gst_by_counter_ledger` itself.
    pub(crate) enum GstByLedgerCode {
        /// The window is partial or withholds a voucher, so it has no totals.
        WindowPartial = "gst_by_ledger_window_partial",
    }
}

failure_codes! {
    /// The refusal of the register tools themselves.
    pub(crate) enum RegisterCode {
        /// The masters read carried no master mark to pin the read under.
        MasterMarkUnavailable = "register_master_mark_unavailable",
    }
}

failure_codes! {
    /// The refusals of `outstandings` itself.
    pub(crate) enum OutstandingsCode {
        /// `ageing_basis` is not one the tool reads.
        InvalidAgeingBasis = "invalid_ageing_basis",
        /// `direction` is not one the tool reads.
        InvalidDirection = "invalid_direction",
        /// A bill's amount is not an exact decimal.
        AmountInvalid = "outstandings_amount_invalid",
    }
}

failure_codes! {
    /// The typed refusals of the proposed stock-item Create flow (#1381).
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) enum StockItemCreateCode {
        ReadUnreadable = "stock_item_read_unreadable",
        NoItems = "stock_item_no_items",
        NoNameEntry = "stock_item_name_entry_missing",
        NameIndexInvalid = "stock_item_name_index_invalid",
        GuidNotLowercase = "stock_item_guid_not_lowercase",
        NameUnchecked = "stock_item_name_unchecked",
        NameTaken = "master_name_taken",
        SiblingNotFound = "stock_item_sibling_not_found",
        SiblingAmbiguous = "stock_item_sibling_ambiguous",
        SiblingOnlyByFold = "stock_item_sibling_only_by_fold",
        SiblingIncomplete = "stock_item_sibling_incomplete",
        SiblingNotGoods = "stock_item_sibling_not_goods",
        SiblingNotTaxable = "stock_item_sibling_not_taxable",
        SiblingGstNotApplicable = "stock_item_sibling_gst_not_applicable",
        CatalogueChanged = "stock_item_catalogue_changed",
        GroupsUnreadable = "stock_item_groups_unreadable",
        ParentNotInBook = "stock_item_parent_not_in_book",
        UnitsUnreadable = "stock_item_units_unreadable",
        UnitNotInBook = "stock_item_unit_not_in_book",
        ShapeNotMeasured = "stock_item_shape_not_measured",
        TextNotPlain = "stock_item_text_not_plain",
        TextNeedsEscaping = "stock_item_text_needs_escaping",
        ReviewLayoutText = "import_review_layout_text",
        ReviewFormatText = "import_review_format_text",
        ReviewTooLarge = "master_review_too_large",
        CreateApplicationFailure = "stock_item_create_application_failure",
        AlteredExisting = "stock_item_altered_existing",
        CountersNotClean = "stock_item_counters_not_clean",
        CountersUnreported = "stock_item_counters_unreported",
        ReadBackDiffers = "stock_item_read_back_differs",
        ReadBackNotFoundOnce = "stock_item_read_back_not_found_once",
        ReadBackNotNew = "stock_item_read_back_not_new",
        ReadBackOtherCompany = "stock_item_read_back_other_company",
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn stock_item_catalogue_refusal_code(
    error: &StockItemCatalogueError,
) -> StockItemCreateCode {
    match error {
        StockItemCatalogueError::Unreadable(_) => StockItemCreateCode::ReadUnreadable,
        StockItemCatalogueError::NoItems => StockItemCreateCode::NoItems,
        StockItemCatalogueError::NoNameEntry { .. } => StockItemCreateCode::NoNameEntry,
        StockItemCatalogueError::Index(_) => StockItemCreateCode::NameIndexInvalid,
        StockItemCatalogueError::GuidNotLowercase { .. } => StockItemCreateCode::GuidNotLowercase,
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn item_name_refusal_code(error: &ItemNameNotAbsent) -> StockItemCreateCode {
    match error {
        ItemNameNotAbsent::Taken(_) => StockItemCreateCode::NameTaken,
        ItemNameNotAbsent::Unchecked(_) => StockItemCreateCode::NameUnchecked,
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn sibling_refusal_code(error: &SiblingRefusal) -> StockItemCreateCode {
    match error {
        SiblingRefusal::NotFound => StockItemCreateCode::SiblingNotFound,
        SiblingRefusal::Ambiguous { .. } => StockItemCreateCode::SiblingAmbiguous,
        SiblingRefusal::OnlyByFold { .. } => StockItemCreateCode::SiblingOnlyByFold,
        SiblingRefusal::Incomplete(_) => StockItemCreateCode::SiblingIncomplete,
        SiblingRefusal::Unchecked(_) => StockItemCreateCode::NameUnchecked,
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn not_goods_refusal_code(error: &NotGoodsSibling) -> StockItemCreateCode {
    match error {
        NotGoodsSibling::NotGoods => StockItemCreateCode::SiblingNotGoods,
        NotGoodsSibling::NotTaxable { .. } => StockItemCreateCode::SiblingNotTaxable,
        NotGoodsSibling::GstNotApplicable => StockItemCreateCode::SiblingGstNotApplicable,
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn draft_refusal_code(error: &DraftRefusal) -> StockItemCreateCode {
    match error {
        DraftRefusal::OtherCatalogue => StockItemCreateCode::CatalogueChanged,
        DraftRefusal::GroupsUnreadable(_) => StockItemCreateCode::GroupsUnreadable,
        DraftRefusal::ParentNotInBook => StockItemCreateCode::ParentNotInBook,
        DraftRefusal::UnitsUnreadable(_) => StockItemCreateCode::UnitsUnreadable,
        DraftRefusal::UnitNotInBook => StockItemCreateCode::UnitNotInBook,
        DraftRefusal::GstEntriesNotMeasured { .. }
        | DraftRefusal::GstSourceNotMeasured
        | DraftRefusal::ItcNotMeasured
        | DraftRefusal::StateNotAny
        | DraftRefusal::DutyHeadNotMeasured { .. }
        | DraftRefusal::ValuationNotMeasured { .. }
        | DraftRefusal::RateNotMeasured { .. }
        | DraftRefusal::HsnEntriesNotMeasured { .. }
        | DraftRefusal::HsnNotMeasured => StockItemCreateCode::ShapeNotMeasured,
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn create_render_refusal_code(error: &CreateRenderRefusal) -> StockItemCreateCode {
    match error {
        CreateRenderRefusal::TextNotPlain { .. } => StockItemCreateCode::TextNotPlain,
        CreateRenderRefusal::TextNeedsEscaping { .. } => StockItemCreateCode::TextNeedsEscaping,
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn stock_item_outcome_code(
    outcome: &StockItemImportOutcome,
) -> Option<StockItemCreateCode> {
    match outcome {
        StockItemImportOutcome::ApplicationFailure => {
            Some(StockItemCreateCode::CreateApplicationFailure)
        }
        StockItemImportOutcome::CountersClean(_) => None,
        StockItemImportOutcome::AlteredExisting { .. } => {
            Some(StockItemCreateCode::AlteredExisting)
        }
        StockItemImportOutcome::NotClean { .. } => Some(StockItemCreateCode::CountersNotClean),
        StockItemImportOutcome::Unreported => Some(StockItemCreateCode::CountersUnreported),
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn stock_item_read_back_code(verdict: &ReadBackVerdict) -> Option<StockItemCreateCode> {
    match verdict {
        ReadBackVerdict::Equal(_) => None,
        ReadBackVerdict::Differs { .. } => Some(StockItemCreateCode::ReadBackDiffers),
        ReadBackVerdict::NotFoundOnce { .. } => Some(StockItemCreateCode::ReadBackNotFoundOnce),
        ReadBackVerdict::NotNew => Some(StockItemCreateCode::ReadBackNotNew),
        ReadBackVerdict::OtherCompany => Some(StockItemCreateCode::ReadBackOtherCompany),
    }
}

failure_codes! {
    /// The differences an invoice's e-invoice read gives that name no element (#2162 item 9).
    pub(crate) enum EInvoiceReadbackCode {
        /// The read was not answered, or its request could not be built.
        ReadFailed = "e_invoice_readback_failed",
        /// The answer did not parse as a vouchers read.
        Unreadable = "e_invoice_readback_unreadable",
        /// No voucher of the invoice's date carries the GUID its read-back returned.
        NotFound = "e_invoice_not_found",
        /// More than one voucher carries that GUID.
        NotUnique = "e_invoice_not_unique",
    }
}

failure_codes! {
    /// The build's own refusals of an invoice, where `post_import`'s code would not say the cause.
    pub(crate) enum InvoiceBuildCode {
        /// The approval text is over the window's limit, and the same invoice without its
        /// e-invoice details fits (#2574).
        ReviewTooLargeWithEInvoice = "invoice_review_too_large_with_e_invoice",
    }
}

failure_codes! {
    /// The refusals of the stock-item post before anything is sent (#1381), the strings the ledger
    /// post writes for the same refusals.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) enum MasterPostCode {
        BatchIdInvalid = "import_batch_identifier_invalid",
        BatchNotFound = "master_batch_not_found",
        CompanyIdentityMismatch = "company_identity_mismatch",
        BatchChanged = "import_batch_changed",
        ApprovalBindingChanged = "import_approval_binding_changed",
    }
}

#[cfg(feature = "lab-writes")]
failure_codes! {
    /// The refusals of `lab_import_masters` that name what they refused, each before any write.
    pub(crate) enum LabMastersCode {
        /// A ledger carries opening bill allocations (bridge#1500).
        OpeningBillsUnsupported = "lab_opening_bills_unsupported",
        /// A group's parent is the root, or absent (bridge#974).
        RootParentUnmeasured = "lab_root_parent_unmeasured",
        /// A requested name shares one of master's name-index keys with a target master whose name
        /// has another canonical key (#2499).
        NameSharesKey = "lab_master_name_shares_a_key",
    }
}

failure_codes! {
    /// The refusals of an import review acknowledgement and the records it reads and writes.
    pub(crate) enum ImportAckCode {
        /// No doubt of the requested kind is observed.
        NoObservedDoubt = "ack_no_observed_doubt",
        /// A doubt's own record is unavailable, so it cannot be reviewed.
        DoubtRecordUnavailable = "ack_doubt_record_unavailable",
        /// More than one observed doubt needs the caller to name one.
        DoubtAmbiguous = "ack_doubt_ambiguous",
        /// The check whose doubt would be reviewed is still pending.
        CheckPending = "ack_check_pending",
        /// The masters doubt record cannot be read.
        MastersRecordUnreadable = "ack_masters_record_unreadable",
        /// The batch-step doubt record cannot be read.
        StepRecordUnreadable = "ack_step_record_unreadable",
        /// The import response was not clean.
        ResponseNotClean = "ack_response_not_clean",
        /// The review's voucher read-back did not match what it must bind.
        ReadbackNotMatched = "ack_readback_not_matched",
        /// The review text exceeds a native dialog cap.
        ReviewTooLarge = "ack_review_too_large",
        /// The review text's layout is invalid.
        ReviewLayoutText = "ack_review_layout_text",
        /// The review text's format is invalid.
        ReviewFormatText = "ack_review_format_text",
        /// The recorded review covers another doubt.
        RecordedReviewStale = "ack_recorded_review_stale",
        /// A review is already recorded.
        AlreadyRecorded = "ack_already_recorded",
        /// The acknowledgement record could not be written.
        ImportFileWriteFailed = "import_file_write_failed",
        /// The import journal has no such batch.
        ImportBatchNotFound = "import_batch_not_found",
        /// The batch was not dispatched or has no vouchers.
        BatchNotPosted = "ack_batch_not_posted",
        /// The requested doubt name is invalid.
        DoubtInvalid = "ack_doubt_invalid",
        /// The observed review state changed while the dialog was open.
        ChangedWhileReviewing = "ack_changed_while_reviewing",
        /// The acknowledgement record could not be serialized.
        ProofSerializationFailed = "proof_serialization_failed",
    }
}

failure_codes! {
    /// The refusals of the bank-statement parse and proposals-loading tools.
    pub(crate) enum BankStatementCode {
        /// The blocking statement parse task failed.
        TaskFailed = "statement_task_failed",
        /// A bank whose statements print control totals was not given both.
        ControlTotalsRequired = "statement_control_totals_required",
        /// The statement window's `from` is after its `to`.
        ReversedDateWindow = "statement_reversed_date_window",
        /// The password file could not be read.
        PasswordFileUnreadable = "statement_password_file_unreadable",
        /// The password file is not private to the user.
        PasswordFilePermissions = "statement_password_file_permissions",
        /// The password file exceeds the size bound.
        PasswordFileTooLarge = "statement_password_file_too_large",
        /// The password file is not UTF-8.
        PasswordFileNotUtf8 = "statement_password_file_not_utf8",
        /// The statement file could not be read.
        FileUnreadable = "statement_file_unreadable",
        /// The statement file exceeds the reader's size bound.
        FileTooLarge = "statement_file_too_large",
        /// The test-only PDF engine setting is not an absolute path.
        #[cfg_attr(not(test), allow(dead_code))]
        PdfEngineSettingInvalid = "statement_pdf_engine_setting_invalid",
        /// The PDF engine's location could not be established.
        PdfEngineUnavailable = "statement_pdf_engine_unavailable",
        /// The private proposals directory could not be prepared.
        ProposalsDirectoryUnavailable = "statement_proposals_directory_unavailable",
        /// The proposals document could not be serialized.
        ProposalsSerializationFailed = "statement_proposals_serialization_failed",
        /// The private proposals file could not be written.
        ProposalsWriteFailed = "statement_proposals_write_failed",
        /// No proposals SHA-256 was given.
        ProposalsSha256Required = "proposals_sha256_required",
        /// No proposals file with the requested id exists.
        ProposalsNotFound = "proposals_not_found",
        /// The proposals file could not be read.
        ProposalsFileUnreadable = "proposals_file_unreadable",
        /// The proposals file exceeds the size bound.
        ProposalsFileTooLarge = "proposals_file_too_large",
        /// The proposals file's hash differs from the digest the parse returned.
        ProposalsChanged = "proposals_changed",
        /// The tool arguments are not an object.
        ArgumentSchemaInvalid = "argument_schema_invalid",
        /// Both a proposals id and inline vouchers were given.
        ProposalsIdWithVouchers = "proposals_id_with_vouchers",
        /// A cash line still needs an answer.
        CashQuestionsOpen = "cash_questions_open",
        /// The legacy proposals file lacks facts needed for its transfers.
        ProposalsLegacy = "proposals_legacy",
        /// Control figures were supplied although this file is their source.
        ControlFiguresWithThisFile = "statement_control_figures_with_this_file",
        /// The selected control source requires opening and closing figures.
        ControlFiguresRequired = "statement_control_figures_required",
    }
}

failure_codes! {
    /// The refusals of the import journal's readers and writers.
    pub(crate) enum ImportLedgerCode {
        /// A journal record or its place in the journal is invalid.
        Invalid = "import_ledger_invalid",
        /// One persisted request digest names more than one batch.
        BatchDigestAmbiguous = "import_batch_digest_ambiguous",
        /// The journal's bytes could not be read.
        Unavailable = "import_ledger_unavailable",
        /// A batch has more than one dispatch intent.
        DuplicateDispatch = "import_ledger_duplicate_dispatch",
        /// A journal record could not be serialized.
        SerializationFailed = "import_ledger_serialization_failed",
        /// A journal record exceeds the byte bound.
        RecordTooLarge = "import_ledger_record_too_large",
    }
}

#[cfg(test)]
#[path = "agent_failure_code_tests.rs"]
mod tests;
