//! Thin MCP presentation of the shared company features read (#1231).
use super::*;
use bridge_tally_protocol::native_company_features::{
    NativeCompanyFeatures, NativeCurrencySymbol, NativeSetting,
};

const VERIFICATION: &str = "stable_paired_sources_with_company_mode_and_extent_guards";

/// The evidence a setting rests on, in one closed vocabulary. Only the
/// cost-centre setting has been compared with Tally's own screen (three
/// synthetic books). Batch-wise stock differed across the two committed books
/// and GST differed only on a third book that is not committed (it read Yes
/// and then No in one sitting); neither has been compared with the screen.
const COMPARED_WITH_SCREEN: &str = "compared_with_tally_screen_on_synthetic_books";
const BATCH_WISE_DIFFERED: &str = "differed_across_committed_books_not_compared_with_screen";
const GST_DIFFERED: &str = "differed_on_an_uncommitted_book_not_compared_with_screen";

/// A setting Tally did not send was compared with nothing, so it carries no
/// comparison label.
const NOT_SENT: &str = "not_reported_by_tally";

fn setting_json(setting: NativeSetting, evidence: &'static str) -> Value {
    let evidence = if setting == NativeSetting::NotReported {
        NOT_SENT
    } else {
        evidence
    };
    json!({"value": setting, "evidence": evidence})
}

fn settings(features: &NativeCompanyFeatures) -> Value {
    json!({
        "cost_centres": setting_json(features.cost_centres, COMPARED_WITH_SCREEN),
        "gst": setting_json(features.gst, GST_DIFFERED),
        "batch_wise": setting_json(features.batch_wise, BATCH_WISE_DIFFERED),
    })
}

fn currency(symbol: &NativeCurrencySymbol) -> Value {
    match symbol {
        NativeCurrencySymbol::Reported(symbol) => {
            json!({"state": "reported", "symbol": symbol, "kind": "symbol_not_iso_code"})
        }
        NativeCurrencySymbol::NotReported => json!({"state": "not_reported"}),
    }
}

impl Server {
    pub(super) async fn company_features(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let (company, identity, prior) = self.verified_company(guid).await?;
        let read = self
            .runtime
            .fetch_company_features(self.tally_config(), &identity)
            .await
            .map_err(|error| {
                ToolFailure::from_runtime("company_features_read_failed", error)
                    .with_prior_evidence(prior.clone())
            })?;
        let mut evidence = combine_evidence(prior, evidence_from_runtime_read(read.evidence));
        let basis = headline::CompanyFeaturesBasis::new(&read.features);
        // A setting Tally did not send is a gap in what this answer says: the
        // state an agent reads first says so, as `vouchers` and `outstandings` do.
        let state = if basis.all_reported() {
            "observed"
        } else {
            evidence.state = "partial";
            evidence.reason_code = Some("company_features_setting_not_reported".to_string());
            "partial"
        };
        let mut payload = json!({
            "company": company_json(&company, std::slice::from_ref(&company)),
            "result": {
                "state": state,
                "basis": "tally_company_collection_settings_today",
                "settings": settings(&read.features),
                "base_currency": currency(&read.features.base_currency),
                "verification": VERIFICATION,
                "limitations": [
                    "These are the settings Tally's company record holds today. They say nothing about what the books contain or what a setting was during a year: a cost-centre allocation can be stored while the cost-centre setting reads No",
                    "Only the cost-centre setting has been compared with Tally's own F11 screen, and that was on three synthetic books, not on this company. Batch-wise stock differed across the two committed books and GST differed only on a third book that is not committed; neither has been compared with the screen",
                    "A setting Tally did not send is not_reported, which is not the same as no",
                    "The currency is the symbol Tally holds, not an ISO code, and it does not say whether foreign currencies are used",
                    "Tally's other feature flags are fetched in the same request and are not returned: each has only ever read one value on the books captured, so none is known to follow its setting",
                    "Nothing this tool returns changes what another tool reads or refuses: stock_summary reads its own inventory, integrated and batch-wise flags from its own request",
                    "Measured on three synthetic books of one TallyPrime 7.1 Silver; other releases are not measured",
                    "Tally's answer carries no identity beyond the row's own GUID, name, company number and books-from date, which are compared with the verified company's, and the mode and book-extent checks around the read",
                    "The currency symbol is refused if it is over 16 characters or holds a control character, a bidirectional override or isolate, a zero-width or other invisible format character, or a line or paragraph separator",
                ],
            },
        });
        payload["headline"] = json!(basis.headline(&headline::CompanyName::new(&company.name)));
        Ok(ToolOutcome {
            payload,
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

#[cfg(test)]
#[path = "agent_company_features_tests.rs"]
mod tests;
