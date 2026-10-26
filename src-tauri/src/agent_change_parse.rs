//! Change parse for the local MCP adapter.
use super::*;

#[path = "agent_company_checkpoint.rs"]
mod company_checkpoint;
pub(super) use company_checkpoint::{
    company_voucher_high_water, company_voucher_mark, parse_all_company_marks,
    parse_company_high_water, parse_company_marks, parse_company_marks_as_answered,
    LoadedCompanyMarks,
};

/// The absent-voucher-axis error, named because `pre_import_mark` matches on it
/// to separate an empty book from a response it could not read. The guard test
/// `voucher_axis_absence_matches_its_named_code` pins this to what
/// `observed_checkpoint` actually formats, so the two cannot drift apart.
pub(super) const VOUCHER_CHECKPOINT_NOT_OBSERVED: &str = "voucher_checkpoint_not_observed";

pub(super) fn observed_checkpoint(value: Option<&String>, axis: &str) -> Result<u64, String> {
    value
        .ok_or_else(|| format!("{axis}_checkpoint_not_observed"))?
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("{axis}_checkpoint_invalid"))
}
