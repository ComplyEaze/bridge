//! Tally's own Godown Summary, requested by report name (#2017).
//!
//! A built-in report like the statements (protocol reference §12a.1): a success
//! carries no `HEADER` or `STATUS`, and nothing in the response identifies the
//! company, so a caller must bracket the read with identity and extent reads.
//! The one capture is the Stock Summary's grammar, one `DSPACCNAME` and
//! `DSPSTKINFO` pair per godown and no line for a group or an item, so it is
//! read by [`parse_native_stock_summary_report`]: a line here is a godown, its
//! quantity is present only when Tally sends one (it did not for a godown whose
//! items are in several units), and an empty amount is not zero. Whether a
//! godown under another godown is a line of its own is not measured.
//!
//! [`parse_native_stock_summary_report`]: crate::native_stock_summary::parse_native_stock_summary_report
use crate::{
    native_outstandings::NativeLedgerSnapshotPeriod,
    native_statement_reports::{render_built_in_report_request, BuiltInReport},
};

/// Renders the only supported request: Tally's own Godown Summary by name over
/// the period, for the named company. No TDL, and no variable that names a
/// godown.
pub fn render_native_godown_summary_request(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_built_in_report_request(BuiltInReport::GodownSummary, company, period)
}

#[cfg(test)]
#[path = "native_godown_summary_tests.rs"]
mod tests;
