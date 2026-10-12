use bridge_tally_core::TallyDate;
use bridge_tally_protocol::xml_text::escape_text as xml_escape;

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

/// No production path sends this: the live-read tool renders `ledgers_v1`
/// through `ReadOnlyProfile::LedgersV1`. It compiles only for the tests below
/// that check its formula hazard and escaping.
#[cfg(test)]
pub fn ledgers_request(company: &str) -> String {
    bridge_tally_protocol::xml_read_profiles::compatibility::ledgers_request(company)
}

/// Experimental Bridge-defined ledger-balance cross-view. The request emits no
/// ledger names: rows are joined to the canonical mirror by candidate native
/// identifiers. Exact semantics and applicability remain capability-gated.
///
/// `from`/`to` are interpolated into two places: ordinary `SVFROMDATE`/
/// `SVTODATE` character content, and a **quoted TDL literal**
/// (`<SET>"{from}"</SET>`/`<SET>"{to}"</SET>`, bridge#832). XML escaping
/// cannot protect the quoted-literal occurrence -- Tally decodes `&quot;`
/// back to a literal `"` before evaluating the `<SET>` formula, so an
/// escaped quote could still close the literal early. `TallyDate` closes
/// that off with a closed input alphabet (exactly 8 ASCII digits) instead of
/// escaping, the same fix used by
/// `native_outstandings::render_native_voucher_export_request`.
pub fn ledger_period_balances_request(company: &str, from: &TallyDate, to: &TallyDate) -> String {
    format!(
        r#"
<ENVELOPE>
    <HEADER>
        <VERSION>1</VERSION>
        <TALLYREQUEST>EXPORT</TALLYREQUEST>
        <TYPE>DATA</TYPE>
        <ID>BRIDGE Ledger Period Balances V1</ID>
    </HEADER>
    <BODY>
        <DESC>
            <STATICVARIABLES>
                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
                <SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY>
                <SVFROMDATE TYPE="Date">{}</SVFROMDATE>
                <SVTODATE TYPE="Date">{}</SVTODATE>
            </STATICVARIABLES>
            <TDL>
                <TDLMESSAGE>
                    <REPORT NAME="BRIDGE Ledger Period Balances V1">
                        <FORMS>BRIDGE Ledger Period Balances Form V1</FORMS>
                        <PLAINXML>Yes</PLAINXML>
                    </REPORT>
                    <FORM NAME="BRIDGE Ledger Period Balances Form V1">
                        <TOPPARTS>BRIDGE Ledger Period Context Part V1, BRIDGE Ledger Period Rows Part V1</TOPPARTS>
                    </FORM>
                    <PART NAME="BRIDGE Ledger Period Context Part V1">
                        <TOPLINES>BRIDGE Ledger Period Context Line V1</TOPLINES>
                    </PART>
                    <PART NAME="BRIDGE Ledger Period Rows Part V1">
                        <TOPLINES>BRIDGE Ledger Period Row Line V1</TOPLINES>
                        <REPEAT>BRIDGE Ledger Period Row Line V1 : BRIDGE Ledger Period Collection V1</REPEAT>
                    </PART>
                    <LINE NAME="BRIDGE Ledger Period Context Line V1">
                        <LEFTFIELDS>BRIDGE Ledger Period Schema V1, BRIDGE Ledger Period Object V1, BRIDGE Ledger Period Company GUID V1, BRIDGE Ledger Period From V1, BRIDGE Ledger Period To V1, BRIDGE Ledger Period Ordinary Books Requested V1, BRIDGE Ledger Period Count V1</LEFTFIELDS>
                        <XMLTAG>"COMPANYCONTEXT"</XMLTAG>
                    </LINE>
                    <FIELD NAME="BRIDGE Ledger Period Schema V1"><SET>"bridge.tally.ledger-period-balances/1"</SET><XMLTAG>"SCHEMA"</XMLTAG></FIELD>
                    <FIELD NAME="BRIDGE Ledger Period Object V1"><SET>"LEDGERPERIODBALANCE"</SET><XMLTAG>"OBJECTTYPE"</XMLTAG></FIELD>
                    <FIELD NAME="BRIDGE Ledger Period Company GUID V1"><SET>$GUID:Company:##SVCurrentCompany</SET><XMLTAG>"GUID"</XMLTAG></FIELD>
                    <FIELD NAME="BRIDGE Ledger Period From V1"><SET>"{}"</SET><XMLTAG>"FROMDATE"</XMLTAG></FIELD>
                    <FIELD NAME="BRIDGE Ledger Period To V1"><SET>"{}"</SET><XMLTAG>"TODATE"</XMLTAG></FIELD>
                    <FIELD NAME="BRIDGE Ledger Period Ordinary Books Requested V1"><SET>Yes</SET><TYPE>Logical</TYPE><XMLTAG>"ORDINARYBOOKSREQUESTED"</XMLTAG></FIELD>
                    <FIELD NAME="BRIDGE Ledger Period Count V1"><SET>$$NumItems:BRIDGE Ledger Period Collection V1</SET><XMLTAG>"RECORDCOUNT"</XMLTAG></FIELD>
                    <LINE NAME="BRIDGE Ledger Period Row Line V1">
                        <LEFTFIELDS>BRIDGE Ledger Period Opening V1, BRIDGE Ledger Period Closing V1</LEFTFIELDS>
                        <XMLTAG>"LEDGERPERIODBALANCE"</XMLTAG>
                        <XMLATTR>"GUID" : $GUID</XMLATTR>
                        <XMLATTR>"REMOTEID" : $RemoteID</XMLATTR>
                        <XMLATTR>"MASTERID" : $MasterID</XMLATTR>
                        <XMLATTR>"ALTERID" : $AlterID</XMLATTR>
                    </LINE>
                    <FIELD NAME="BRIDGE Ledger Period Opening V1"><SET>$TBalOpening</SET><TYPE>Amount</TYPE><FORMAT>"No Symbol, No Comma"</FORMAT><XMLTAG>"OPENINGBALANCE"</XMLTAG></FIELD>
                    <FIELD NAME="BRIDGE Ledger Period Closing V1"><SET>$TBalClosing</SET><TYPE>Amount</TYPE><FORMAT>"No Symbol, No Comma"</FORMAT><XMLTAG>"CLOSINGBALANCE"</XMLTAG></FIELD>
                    <COLLECTION NAME="BRIDGE Ledger Period Collection V1" ISMODIFY="No" ISFIXED="No" ISINITIALIZE="Yes" ISOPTION="No" ISINTERNAL="No">
                        <TYPE>Ledger</TYPE>
                        <FETCH>GUID, RemoteID, MasterID, AlterID, TBalOpening, TBalClosing</FETCH>
                    </COLLECTION>
                </TDLMESSAGE>
            </TDL>
        </DESC>
    </BODY>
</ENVELOPE>
"#,
        xml_escape(company),
        from.as_str(),
        to.as_str(),
        from.as_str(),
        to.as_str(),
    )
    .trim()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        company_list_request, ledger_period_balances_request, ledgers_request,
        legacy_company_list_request, TallyDate,
    };

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
            // `RuntimeTallyConnector::read_core_period_balance_report`.
            "ledger_period_balances_request",
        ];
        assert_eq!(listed, guarded.into_iter().collect());
        let id = ReadOnlyProfileId::LedgersV1;
        assert!(id.education_refuses_report_formula(), "{}", id.as_str());
        let synthetic_from = TallyDate::parse("20260401").unwrap();
        let synthetic_to = TallyDate::parse("20260430").unwrap();
        for (builder, request) in [
            ("ledgers_request", ledgers_request("Synthetic Company")),
            (
                "ledger_period_balances_request",
                ledger_period_balances_request("Synthetic Company", &synthetic_from, &synthetic_to),
            ),
        ] {
            assert!(
                first_spaced_function_argument(&request).is_some(),
                "{builder}"
            );
        }
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

        let from = TallyDate::parse("20260401").unwrap();
        let to = TallyDate::parse("20260430").unwrap();
        let period = ledger_period_balances_request("BRIDGE & PERIOD", &from, &to);
        assert!(period.contains("BRIDGE &amp; PERIOD"));

        // `from`/`to` also feed the quoted `<SET>"{from}"</SET>` TDL literal,
        // where XML escaping cannot protect the argument (Tally decodes
        // `&quot;` back to `"` before the formula runs). A closed-alphabet
        // `TallyDate` makes the previously-escaped injection strings simply
        // not constructible, rather than escaping them.
        assert!(TallyDate::parse("2026<0401").is_err());
        assert!(TallyDate::parse("2026&0430").is_err());
    }

    #[test]
    fn period_balance_request_is_identity_scoped_and_name_free() {
        let from = TallyDate::parse("20260401").unwrap();
        let to = TallyDate::parse("20260430").unwrap();
        let request = ledger_period_balances_request("Synthetic Company", &from, &to);
        assert!(request.contains("bridge.tally.ledger-period-balances/1"));
        assert!(request.contains("<SET>$TBalOpening</SET>"));
        assert!(request.contains("<SET>$TBalClosing</SET>"));
        assert!(request.contains("<XMLTAG>\"ORDINARYBOOKSREQUESTED\"</XMLTAG>"));
        assert!(!request.contains("<XMLTAG>\"ORDINARYBOOKS\"</XMLTAG>"));
        assert!(request.contains("<XMLATTR>\"GUID\" : $GUID</XMLATTR>"));
        assert!(request.contains("<XMLTAG>\"FROMDATE\"</XMLTAG>"));
        assert!(!request.contains("<XMLTAG>\"NAME\"</XMLTAG>"));
    }
}
