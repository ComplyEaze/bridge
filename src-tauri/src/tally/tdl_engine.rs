pub fn standard_ledger_identity_request(company: &str) -> String {
    bridge_tally_protocol::xml_read_profiles::compatibility::standard_ledger_identity_request(
        company,
    )
}

#[cfg(test)]
mod tests {
    /// The hazard gate records no builder that passes a spaced identifier to
    /// a `$$` function (Education answered one with a blocking dialog,
    /// bridge#45; the last, `render_ledgers`, was deleted under #1570), so a
    /// new such builder fails this test until it is listed here with the guard
    /// that keeps it from an Education responder.
    #[test]
    fn every_report_formula_hazard_is_kept_from_an_education_responder() {
        use bridge_tally_protocol::xml_read_profiles::first_spaced_function_argument;
        let script = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../scripts/check-tally-request-builder-hazards.mjs"),
        )
        .unwrap();
        let listed: std::collections::BTreeSet<&str> = script
            .lines()
            .filter_map(|line| line.trim().strip_prefix("\"function-argument-with-space|"))
            .map(|entry| {
                let builder = entry.split('|').next().unwrap();
                builder.rsplit("::").next().unwrap()
            })
            .collect();
        let guarded: [&str; 0] = [];
        assert_eq!(listed, guarded.into_iter().collect());
        assert_eq!(
            first_spaced_function_argument(&super::standard_ledger_identity_request(
                "BRIDGE SYNTHETIC BOOK"
            )),
            None
        );
    }
}
