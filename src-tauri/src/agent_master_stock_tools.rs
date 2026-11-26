//! `prepare_stock_item` (#1381): the stock twin of `prepare_ledger_creates`.
//! No tool is registered for it yet, so nothing shipped calls it.
//!
//! It verifies the company, reads the company's stock reads once in one queued
//! execution, builds the stock-item draft from those reads, builds the preview
//! from the same draft and records the batch in the stock journal. It asks no
//! one and writes nothing to Tally: as for a ledger, the ask and the hold come
//! with the post.
use super::super::master_ledger::JournalRecord;
use super::super::master_post::stock_company_marks;
use super::super::master_review::{show_stock_item_draft, ShownStockItem};
use super::*;
use crate::agent::failure_code::{
    create_render_refusal_code, draft_refusal_code, item_name_refusal_code, not_goods_refusal_code,
    sibling_refusal_code, stock_item_catalogue_refusal_code, StockItemCreateCode,
};
use crate::tally::runtime::{
    AdmittedCandidateRead, AdmittedStockGroups, AdmittedStockUnits, MastersReadError,
};
use crate::tally::stock_item_create::{
    CompleteStockItemCatalogue, GoodsSibling, SiblingGst, StockItemDraft,
};

/// What a caller is told after a stock item is prepared. It names no tool: none
/// posts a stock item yet.
const PREPARED_STOCK_NEXT_STEP: &str = "Nothing was written to Tally. This stock item is recorded for approval; the local user must approve it in a native dialog before it is created.";

impl Server {
    /// Prepare ONE stock item, a copy of the sibling stock item `sibling` under
    /// `name`, in the company `company_guid`. Nothing is written to Tally.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(in crate::agent) async fn prepare_stock_item(
        &self,
        company_guid: &str,
        name: &str,
        sibling: &str,
    ) -> Result<ToolOutcome, ToolFailure> {
        let name = MasterName::new(name).map_err(|error| master_name_code(error).to_string())?;
        let sibling =
            MasterName::new(sibling).map_err(|error| master_name_code(error).to_string())?;
        let (_, identity, identity_evidence) = self.verified_company(company_guid).await?;
        let origin = self.master_preview_origin()?;
        let marks = crate::tally::agent_read_request::AgentReadRequest::parse(
            crate::agent::read_profiles::render_agent_company_high_water(identity.display_name()),
        )
        .map_err(|error| error.to_string())?;
        // The stock reads, once, in one queued execution; the draft and its
        // preview are built from them there.
        let shown = self
            .runtime
            .read_and_recheck_stock_item(
                self.tally_config(),
                &identity,
                marks,
                stock_company_marks,
                |reads| {
                    let (candidate, groups, units, _, _) = reads.into_parts();
                    Ok(show_fresh_draft(
                        &candidate, &groups, &units, &name, &sibling, &origin,
                    ))
                },
            )
            .await
            .map_err(stock_read_failure)?;
        let shown = shown.map_err(ToolFailure::from)?;
        let batch_id = format!("bridge-{}", Uuid::new_v4());
        let batch = (shown.batch(batch_id.clone()))
            .map_err(|error| ToolFailure::from(create_render_refusal_code(&error)))?;
        {
            let lock = ExclusiveAdmission::acquire(&self.settings.data_dir)?;
            append(&self.settings.data_dir, &lock, JournalRecord::Batch(batch))
                .map_err(|error| error.code().to_string())?;
        }
        Ok(ToolOutcome {
            payload: json!({"result":{
                "batch_id": batch_id,
                "preview": shown.preview(),
                "next_step": PREPARED_STOCK_NEXT_STEP,
            }}),
            evidence: identity_evidence,
            company_guid: Some(identity.company_guid().to_string()),
            truncated: false,
        })
    }
}

/// The draft of `name` copied from `sibling`, built from the one execution's
/// reads, and its preview; each refusal is its declared stock-item code.
fn show_fresh_draft(
    candidate: &AdmittedCandidateRead,
    groups: &AdmittedStockGroups,
    units: &AdmittedStockUnits,
    name: &MasterName,
    sibling: &MasterName,
    origin: &str,
) -> Result<ShownStockItem, StockItemCreateCode> {
    let catalogue = CompleteStockItemCatalogue::from_admitted(candidate)
        .map_err(|error| stock_item_catalogue_refusal_code(&error))?;
    let name = (catalogue.name_absent(name)).map_err(|error| item_name_refusal_code(&error))?;
    let sibling = SiblingGst::named(catalogue.catalogue(), sibling)
        .map_err(|error| sibling_refusal_code(&error))?;
    let sibling =
        GoodsSibling::from_sibling(sibling).map_err(|error| not_goods_refusal_code(&error))?;
    let draft = StockItemDraft::new_admitted(name, sibling, groups, units)
        .map_err(|error| draft_refusal_code(&error))?;
    show_stock_item_draft(draft, origin).map_err(|error| error.code())
}

/// A refused stock read. The read refuses Education itself, before any stock
/// request and before the reads could hand on a profile, so that refusal is the
/// ledger prepare's licensed-mode refusal; any other is a runtime read failure.
fn stock_read_failure(error: anyhow::Error) -> ToolFailure {
    let education = error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<MastersReadError>(),
            Some(MastersReadError::EducationUnqualified)
        )
    });
    let profile = if education {
        DateBoundaryProfile::EducationRestricted
    } else {
        DateBoundaryProfile::ModeAgnostic
    };
    match require_licensed_mode(profile) {
        Err(failure) => failure,
        Ok(()) => ToolFailure::from_runtime(crate::agent::GENERIC_RUNTIME_READ_FAILURE, error),
    }
}

#[cfg(test)]
#[path = "agent_master_stock_tools_tests.rs"]
mod tests;
