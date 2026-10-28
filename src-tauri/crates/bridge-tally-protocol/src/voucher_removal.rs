//! The answer to a request that cancels or deletes one voucher, read from the
//! counters `parse_import_result` returns, never from the XML again. Neither
//! outcome is a success by itself: protocol reference §9.14 keys a cancel on
//! `ALTERED` and a readback showing the same GUID with `ISCANCELLED=Yes`, and a
//! delete on `DELETED` and a readback without the voucher. Nothing sends such
//! a request yet (#2062).

use crate::TallyImportResult;

/// One of the seven counters of an import `RESPONSE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportCounter {
    Created,
    Altered,
    Deleted,
    Ignored,
    Errors,
    Cancelled,
    Exceptions,
}

/// Why an answer does not show exactly the one intended change (§9.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalRefusal {
    /// The response did not carry this counter, so its zero is not observed.
    CounterAbsent(ImportCounter),
    /// Tally reported this many line errors.
    LineErrors(u64),
    /// The intended counter was not 1.
    IntendedNotOne(ImportCounter, u64),
    /// Another counter was not 0.
    OtherNotZero(ImportCounter, u64),
}

/// The counters showed `ALTERED` 1 and nothing else. Only `CancelOutcome` can
/// make one, so `Altered` cannot be built from any other answer.
#[derive(Debug, PartialEq, Eq)]
pub struct OneAlteredCounted(());

/// The counters showed `DELETED` 1 and nothing else; only `DeleteOutcome` makes one.
#[derive(Debug, PartialEq, Eq)]
pub struct OneDeletedCounted(());

/// The answer to a cancel: `Altered` only when `ALTERED` is 1, every other
/// counter is reported and 0, and there is no line error. `CANCELLED` stayed 0
/// on the one observed cancel (§9.14), so it is held to 0 like the rest.
#[derive(Debug, PartialEq, Eq)]
pub enum CancelOutcome {
    Altered(OneAlteredCounted),
    Refused(RemovalRefusal),
}

/// The answer to a delete: `Deleted` only when `DELETED` is 1 on the same terms.
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteOutcome {
    Deleted(OneDeletedCounted),
    Refused(RemovalRefusal),
}

impl CancelOutcome {
    pub fn from_counters(result: &TallyImportResult) -> Self {
        match one_change(result, ImportCounter::Altered) {
            Ok(()) => Self::Altered(OneAlteredCounted(())),
            Err(refusal) => Self::Refused(refusal),
        }
    }
}

impl DeleteOutcome {
    pub fn from_counters(result: &TallyImportResult) -> Self {
        match one_change(result, ImportCounter::Deleted) {
            Ok(()) => Self::Deleted(OneDeletedCounted(())),
            Err(refusal) => Self::Refused(refusal),
        }
    }
}

/// §9.2's rule (as `TallyImportResult::is_clean_success_for` holds it), with
/// the reason for a refusal kept.
fn one_change(result: &TallyImportResult, intended: ImportCounter) -> Result<(), RemovalRefusal> {
    let presence = &result.counter_presence;
    let counters = [
        (ImportCounter::Created, result.created, presence.created),
        (ImportCounter::Altered, result.altered, presence.altered),
        (ImportCounter::Deleted, result.deleted, presence.deleted),
        (ImportCounter::Ignored, result.ignored, presence.ignored),
        (ImportCounter::Errors, result.errors, presence.errors),
        (
            ImportCounter::Cancelled,
            result.cancelled,
            presence.cancelled,
        ),
        (
            ImportCounter::Exceptions,
            result.exceptions,
            presence.exceptions,
        ),
    ];
    if let Some((counter, ..)) = counters.iter().find(|(.., present)| !present) {
        return Err(RemovalRefusal::CounterAbsent(*counter));
    }
    if result.line_error_count > 0 {
        return Err(RemovalRefusal::LineErrors(result.line_error_count));
    }
    for (counter, value, _) in counters {
        if counter == intended && value != 1 {
            return Err(RemovalRefusal::IntendedNotOne(counter, value));
        }
        if counter != intended && value != 0 {
            return Err(RemovalRefusal::OtherNotZero(counter, value));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "voucher_removal_tests.rs"]
mod tests;
