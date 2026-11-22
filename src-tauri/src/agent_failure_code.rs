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

#[cfg(test)]
#[path = "agent_failure_code_tests.rs"]
mod tests;
