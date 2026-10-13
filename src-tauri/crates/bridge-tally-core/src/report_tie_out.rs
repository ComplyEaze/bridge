use serde::{Deserialize, Serialize};

/// The state of a stored report tie-out. New runs no longer read the report,
/// so none is produced; window evidence saved by an older build can carry one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TieOutState {
    Passed,
    Mismatch,
    Unavailable,
}
