pub fn company_list_request() -> String {
    bridge_tally_protocol::xml_read_profiles::compatibility::company_list_request()
}

pub fn standard_ledger_identity_request(company: &str) -> String {
    bridge_tally_protocol::xml_read_profiles::compatibility::standard_ledger_identity_request(
        company,
    )
}

#[cfg(test)]
fn legacy_company_list_request() -> String {
    r#"
<ENVELOPE>
    <HEADER>
        <VERSION>1</VERSION>
        <TALLYREQUEST>Export</TALLYREQUEST>
        <TYPE>Data</TYPE>
        <ID>Company Report</ID>
    </HEADER>
    <BODY>
        <DESC>
            <STATICVARIABLES>
                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
            </STATICVARIABLES>
            <TDL>
                <TDLMESSAGE>
                    <REPORT NAME="Company Report">
                        <FORMS>Company Form</FORMS>
                        <TITLE>"Company Details"</TITLE>
                    </REPORT>
                    <FORM NAME="Company Form">
                        <TOPPARTS>Company Part</TOPPARTS>
                        <HEIGHT>100% Page</HEIGHT>
                        <WIDTH>100% Page</WIDTH>
                    </FORM>
                    <PART NAME="Company Part">
                        <TOPLINES>Company Header, Company Details</TOPLINES>
                        <REPEAT>Company Details : CompanyCollection</REPEAT>
                        <SCROLLED>Vertical</SCROLLED>
                        <COMMONBORDERS>Yes</COMMONBORDERS>
                    </PART>
                    <LINE NAME="Company Header">
                        <LEFTFIELDS>
                            Company Name Header, Company GUID Header
                        </LEFTFIELDS>
                    </LINE>
                    <FIELD NAME="Company Name Header"><SET>"Company Name"</SET></FIELD>
                    <FIELD NAME="Company GUID Header"><SET>"Company GUID"</SET></FIELD>
                    <LINE NAME="Company Details">
                        <LEFTFIELDS>
                            Company Name Field, Company GUID Field
                        </LEFTFIELDS>
                        <XMLTAG>"CompanyInfo"</XMLTAG>
                    </LINE>
                    <FIELD NAME="Company Name Field"><SET>$Name</SET></FIELD>
                    <FIELD NAME="Company GUID Field"><SET>$GUID</SET></FIELD>
                    <COLLECTION NAME="CompanyCollection">
                        <TYPE>Company</TYPE>
                        <FETCH>Name, GUID</FETCH>
                    </COLLECTION>
                </TDLMESSAGE>
            </TDL>
        </DESC>
    </BODY>
</ENVELOPE>
"#
    .trim()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::{company_list_request, legacy_company_list_request};

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
            first_spaced_function_argument(&company_list_request()),
            None
        );
    }

    #[test]
    fn portable_read_profiles_preserve_the_existing_production_bytes() {
        assert_eq!(company_list_request(), legacy_company_list_request());
    }

    #[test]
    fn requests_only_fields_used_by_the_renderer() {
        let combined = company_list_request();
        for prohibited in [
            "NATIVEMETHOD>*",
            "$Address",
            "$INCOMETAXNUMBER",
            "$TANREGNO",
            "$TANUMBER",
            "$Website",
            "$Narration",
            "$StateName",
            "$PhoneNumber",
            "$Email",
            "$MOBILENUMBERS",
            "$Pincode",
            "$GSTRegNumber",
        ] {
            assert!(
                !combined.contains(prohibited),
                "unexpected TDL field: {prohibited}"
            );
        }
        assert!(company_list_request().contains("<FETCH>Name, GUID</FETCH>"));
    }
}
