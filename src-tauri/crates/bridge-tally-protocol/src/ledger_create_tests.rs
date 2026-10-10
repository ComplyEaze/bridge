// Goldens with synthetic names and values. The first two are in the captured
// shape; the GST elements' positions are the renderer's own (see the module doc).
use super::LedgerCreate;

fn ledger<'a>(name: &'a str, parent: &'a str) -> LedgerCreate<'a> {
    LedgerCreate {
        name,
        parent,
        is_billwise_on: false,
        opening_balance: None,
        party_gstin: None,
        tax_type: None,
        gst_duty_head: None,
    }
}

#[test]
fn a_bank_ledger_with_an_opening_balance_renders_the_captured_shape() {
    let message = LedgerCreate {
        opening_balance: Some("-5013.35"),
        ..ledger("Test Bank 0001", "Bank Accounts")
    }
    .render_message();
    assert_eq!(
        message,
        "<TALLYMESSAGE><LEDGER NAME=\"Test Bank 0001\" ACTION=\"Create\">\
<NAME>Test Bank 0001</NAME><PARENT>Bank Accounts</PARENT><ISBILLWISEON>No</ISBILLWISEON>\
<OPENINGBALANCE>-5013.35</OPENINGBALANCE></LEDGER></TALLYMESSAGE>"
    );
}

#[test]
fn a_ledger_without_an_opening_balance_carries_no_opening_element() {
    assert_eq!(
        ledger("Sales", "Sales Accounts").render_message(),
        "<TALLYMESSAGE><LEDGER NAME=\"Sales\" ACTION=\"Create\">\
<NAME>Sales</NAME><PARENT>Sales Accounts</PARENT><ISBILLWISEON>No</ISBILLWISEON></LEDGER></TALLYMESSAGE>"
    );
}

#[test]
fn a_billwise_party_with_a_gstin_places_the_gstin_after_the_billwise_flag() {
    let message = LedgerCreate {
        is_billwise_on: true,
        party_gstin: Some("27ZZZZZ0000Z1Z5"),
        ..ledger("Sample Cables Private Limited", "Sundry Debtors")
    }
    .render_message();
    assert_eq!(
        message,
        "<TALLYMESSAGE><LEDGER NAME=\"Sample Cables Private Limited\" ACTION=\"Create\">\
<NAME>Sample Cables Private Limited</NAME><PARENT>Sundry Debtors</PARENT>\
<ISBILLWISEON>Yes</ISBILLWISEON><PARTYGSTIN>27ZZZZZ0000Z1Z5</PARTYGSTIN></LEDGER></TALLYMESSAGE>"
    );
}

#[test]
fn a_duty_ledger_escapes_its_parent_and_places_its_gst_fields_last() {
    let message = LedgerCreate {
        tax_type: Some("GST"),
        gst_duty_head: Some("State Tax"),
        ..ledger("Output State Tax 9%", "Duties & Taxes")
    }
    .render_message();
    assert_eq!(
        message,
        "<TALLYMESSAGE><LEDGER NAME=\"Output State Tax 9%\" ACTION=\"Create\">\
<NAME>Output State Tax 9%</NAME><PARENT>Duties &amp; Taxes</PARENT><ISBILLWISEON>No</ISBILLWISEON>\
<TAXTYPE>GST</TAXTYPE><GSTDUTYHEAD>State Tax</GSTDUTYHEAD></LEDGER></TALLYMESSAGE>"
    );
}

#[test]
fn a_name_with_markup_is_escaped_in_the_attribute_and_the_element() {
    let message = ledger("A \"Q\" <B> & 'C'", "Sundry Creditors").render_message();
    assert!(message.starts_with(
        "<TALLYMESSAGE><LEDGER NAME=\"A &quot;Q&quot; &lt;B&gt; &amp; &apos;C&apos;\" ACTION=\"Create\">\
<NAME>A &quot;Q&quot; &lt;B&gt; &amp; &apos;C&apos;</NAME>"
    ));
}
