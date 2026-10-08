//! One error type for the whole slice. A consumer-rule refusal carries the rule's code exactly
//! as the reference engine's `ReadRefused` names it (`C3-content-hash`, `C8-window`, ...), so a
//! test can assert the rule that fired rather than matching a message.

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    /// The read does not satisfy a tally-read-v1 consumer rule. `code` names the rule.
    #[error("{code}: {detail}")]
    Refused { code: &'static str, detail: String },
    /// A part's bytes are not the XML (or JSON) the book needs: malformed markup, an amount
    /// or date that does not parse, a GUID that is missing. The reference engine raises here
    /// too (a parse error or a `ValueError`); neither side guesses.
    #[error("{part}: {detail}")]
    Parse { part: String, detail: String },
    /// `Book::population` refuses while any voucher's status is unknown.
    #[error("{0} vouchers have unknown status; refusing to form the books population")]
    UnknownVoucherStatus(usize),
    /// The engagement or rules configuration is missing a key or has the wrong type.
    #[error("config: {0}")]
    Config(String),
    /// Two different ledgers in the same Book normalise to the same non-blank Tally GUID (a
    /// corrupt read); see `ledger_ids::check_no_duplicate_ledger_guids`.
    #[error("duplicate ledger GUID: {0}")]
    DuplicateGuid(String),
    /// A group (or a caller asking for a GUID-only tag) has no Tally GUID, so no stable id can be
    /// derived; ledgers fall back to a name hash instead (`docs/tax-audit/parity-spec-v1.md` §11).
    #[error("{0} has no Tally GUID; refusing to derive a stable id from its name")]
    MissingGuid(String),
    /// A test named one figure id twice. The reference's `fig` raises there, so the test is
    /// refused rather than the process ended.
    #[error("figure id {0} would repeat")]
    DuplicateFigureId(String),
    /// The reference's bank-statement reader refuses this statement (no declared opening or closing
    /// balance, a row outside its own period, or a date that steps back in an order its running
    /// balance does not confirm). Not a malformed document: `bank_reconciliation::refused` reports
    /// it, with [`crate::documents::StatementRefusal::reason`].
    #[error("{0}")]
    StatementRefused(crate::documents::StatementRefusal),
    /// Two vouchers of one population would take one key (`book::voucher_keys`), which only a
    /// voucher GUID holding a NUL can make; no read produces one. The reference raises the same
    /// wherever it forms the keys; `partners_40b_194t` forms them only for a configured partner (as
    /// the reference does), so a firm with none is not refused there.
    #[error("a voucher GUID holds a NUL character, so two vouchers would take one key")]
    VoucherKeysNotUnique,
    #[error("{path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

impl AuditError {
    pub(crate) fn refused(code: &'static str, detail: impl Into<String>) -> Self {
        Self::Refused {
            code,
            detail: detail.into(),
        }
    }

    pub(crate) fn parse(part: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::Parse {
            part: part.into(),
            detail: detail.into(),
        }
    }

    /// The consumer-rule code, when this is a refusal.
    pub fn code(&self) -> Option<&'static str> {
        match self {
            Self::Refused { code, .. } => Some(code),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, AuditError>;
