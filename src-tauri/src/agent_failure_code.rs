//! Failure codes as typed values (#1942). A code a tool failure carries can reach the egress
//! receipt, so it is declared here, in a closed enum per module, and a [`FailureCode`] is made only
//! from one of them: no string a failure site writes can become one. A site not yet converted still
//! builds its code from a string (`From<String>` for `ToolFailure`); `failure_code_sites_tests`
//! counts those per file, and the count may only fall.

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
    /// The refusals of `import_bank_statement` when it reads a proposals file.
    pub(crate) enum BankStatementImportCode {
        /// The proposals file does not hold what its parse would have written.
        ProposalsFileInvalid = "proposals_file_invalid",
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

#[cfg(test)]
#[path = "agent_failure_code_tests.rs"]
mod tests;
