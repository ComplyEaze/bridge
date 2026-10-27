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

/// No production path sends this. It compiles only for the tests below
/// that check its formula hazard and escaping.
#[cfg(test)]
pub fn ledgers_request(company: &str) -> String {
    bridge_tally_protocol::xml_read_profiles::compatibility::ledgers_request(company)
}

#[cfg(test)]
mod tests {
    use super::{company_list_request, ledgers_request, legacy_company_list_request};

    /// The builders the hazard gate records as passing a spaced identifier to
    /// a `$$` function (Education answered one with a blocking dialog,
    /// bridge#45) are exactly the ones listed here with their guards, so a new
    /// such builder fails this test until it is listed. The guards themselves
    /// are proven by their own tests (connector, runtime, transport, tools),
    /// not here, and a new caller of a listed builder is not caught here.
    #[test]
    fn every_report_formula_hazard_is_kept_from_an_education_responder() {
        use bridge_tally_protocol::xml_read_profiles::{
            first_spaced_function_argument, ReadOnlyProfileId,
        };
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
        let guarded = [
            // `ReadOnlyProfileId::education_refuses_report_formula`: the tools'
            // Education transport.
            "render_ledgers",
        ];
        assert_eq!(listed, guarded.into_iter().collect());
        let id = ReadOnlyProfileId::LedgersV1;
        assert!(id.education_refuses_report_formula(), "{}", id.as_str());
        assert!(first_spaced_function_argument(&ledgers_request("Synthetic Company")).is_some());
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
        let combined = format!(
            "{}{}",
            company_list_request(),
            ledgers_request("Synthetic Company")
        );
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

    #[test]
    fn exact_report_collection_is_shared_by_count_and_rows() {
        let ledgers = ledgers_request("BRIDGE SYNTHETIC BOOK");
        assert!(ledgers.contains("<TYPE>DATA</TYPE>"));
        assert!(ledgers.contains("<ID>BRIDGE Ledger Export V1</ID>"));
        assert!(ledgers.contains("$$NumItems:BRIDGE Ledger Collection V1"));
        assert!(ledgers
            .contains("<REPEAT>BRIDGE Ledger Row Line V1 : BRIDGE Ledger Collection V1</REPEAT>"));
        assert!(ledgers.contains("<XMLTAG>\"COMPANYCONTEXT\"</XMLTAG>"));
        assert!(ledgers.contains("<XMLATTR>\"GUID\" : $GUID</XMLATTR>"));
        assert!(ledgers.contains("<XMLATTR>\"REMOTEID\" : $RemoteID</XMLATTR>"));
        assert!(ledgers.contains("<XMLATTR>\"MASTERID\" : $MasterID</XMLATTR>"));
        assert!(ledgers.contains("<XMLATTR>\"ALTERID\" : $AlterID</XMLATTR>"));
        assert!(ledgers.contains("Name, GUID, RemoteID, MasterID, AlterID"));
    }

    #[test]
    fn report_requests_escape_all_user_controlled_static_variables() {
        let ledgers = ledgers_request("BRIDGE & <SYNTHETIC> \"BOOK\"");
        assert!(ledgers.contains("BRIDGE &amp; &lt;SYNTHETIC&gt; &quot;BOOK&quot;"));
        assert!(!ledgers.contains("BRIDGE & <SYNTHETIC>"));
    }
}
