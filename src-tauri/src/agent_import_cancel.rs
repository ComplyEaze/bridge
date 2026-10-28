//! Candidate requests that remove a voucher ComplyEaze Bridge posted (#2062).
//! Nothing sends them: no tool, setting or dispatch path reaches this module,
//! and the first that may is a later lab-only change (AGENTS.md P1, P7). The
//! answer to such a request is read by `CancelOutcome` and `DeleteOutcome` in
//! the protocol crate.

use super::{render_native_vouchers_as, ImportVoucher, VoucherAction};
use uuid::Uuid;

/// The REMOTEID of a voucher a native post created, as its dispatch intent
/// records it (`native_remote_ids`). Only the text a native post mints is
/// admitted: a version 4 UUID, lowercase and hyphenated (`RemoteIds::mint`).
/// Anything else, from a model, a document or a Tally response, is refused, so
/// no such value can shape a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RemoteId(Uuid);

impl RemoteId {
    pub(super) fn parse(text: &str) -> Result<Self, &'static str> {
        const REFUSED: &str = "import_remote_id_not_native";
        let id = Uuid::try_parse(text).map_err(|_| REFUSED)?;
        let minted = id.get_version() == Some(uuid::Version::Random)
            && id.get_variant() == uuid::Variant::RFC4122
            && id.hyphenated().to_string() == text;
        if minted {
            Ok(Self(id))
        } else {
            Err(REFUSED)
        }
    }
}

/// The cancel candidate: the native post's request for these vouchers, byte
/// for byte, with `ACTION="Cancel"` in place of `ACTION="Create"`. Protocol
/// reference §9.14 observed once that a cancel with the full voucher body
/// cancels in place (PARTIAL, a Journal on Silver); no other type was tried.
pub(super) fn render_native_cancel_xml<'a>(
    company: &str,
    vouchers_with_remote_ids: impl Iterator<Item = (&'a ImportVoucher, RemoteId)>,
) -> String {
    render_native_vouchers_as(
        VoucherAction::Cancel,
        company,
        vouchers_with_remote_ids.map(|(voucher, remote_id)| (voucher, remote_id.0)),
    )
}

#[cfg(test)]
#[path = "agent_import_cancel_tests.rs"]
pub(super) mod tests;
