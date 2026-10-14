// The voucher envelope's bytes as they shipped before this function was shared
// (agent_import.rs), and the masters envelope's (the lab request files). Both
// are written out in full so a change to the shared function cannot move them.
use super::{render_import_envelope, ImportReport};

#[test]
fn the_voucher_envelope_is_pinned_byte_for_byte() {
    assert_eq!(
        render_import_envelope("Co & <X>", ImportReport::Vouchers, "<TALLYMESSAGE/>"),
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>Co &amp; &lt;X&gt;</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE/></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>"
    );
}

#[test]
fn the_masters_envelope_differs_only_in_the_report_name() {
    let vouchers = render_import_envelope("BRIDGE MASTERS LAB", ImportReport::Vouchers, "<M/>");
    let masters = render_import_envelope("BRIDGE MASTERS LAB", ImportReport::AllMasters, "<M/>");
    assert_eq!(vouchers.replace("Vouchers", "All Masters"), masters);
    assert!(masters.contains("<REPORTNAME>All Masters</REPORTNAME>"));
}

#[test]
fn the_company_is_escaped_once_and_the_messages_are_not_touched() {
    let xml = render_import_envelope("A&B", ImportReport::Vouchers, "<X>&amp;</X>");
    assert!(xml.contains("<SVCURRENTCOMPANY>A&amp;B</SVCURRENTCOMPANY>"));
    assert!(xml.contains("<REQUESTDATA><X>&amp;</X></REQUESTDATA>"));
}
