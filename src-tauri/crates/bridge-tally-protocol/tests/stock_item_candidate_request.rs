// SPDX-License-Identifier: Apache-2.0
//! Checks the candidate stock-item request against its captured bytes.
//! Capture evidence: `fixtures/agent/stock-create-lab/PROVENANCE.md`.

use bridge_tally_protocol::native_masters::{
    render_candidate_stock_items_request, render_native_masters_request, NativeMasterKind,
};
use bridge_tally_protocol::xml_read_profiles::ValidatedCompanyName;

const REQUEST: &[u8] =
    include_bytes!("fixtures/agent/stock-create-lab/stock-create-lab-items.request.utf16le.xml");
const SHIPPED_FETCH: &str =
    "<FETCH>NAME, PARENT, GUID, MASTERID, ALTERID, HSNDETAILS.LIST, GSTDETAILS.LIST</FETCH>";
const CANDIDATE_FETCH: &str = "<FETCH>NAME, PARENT, GUID, MASTERID, ALTERID, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, HSNDETAILS.LIST, GSTDETAILS.LIST, LANGUAGENAME.LIST</FETCH>";

#[test]
fn candidate_request_matches_the_captured_bytes() {
    let company = ValidatedCompanyName::new("BRIDGE STOCK CREATE LAB").expect("an admitted name");
    let request = render_candidate_stock_items_request(&company);
    let mut bytes = vec![0xff, 0xfe];
    for unit in request.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }

    assert_eq!(REQUEST.len(), 1330);
    assert_eq!(bytes.as_slice(), REQUEST);
}

#[test]
fn candidate_request_changes_only_the_fetch_list() {
    for name in ["BRIDGE STOCK CREATE LAB", "BRIDGE ESCAPE & <LAB> \"Q\" 'A'"] {
        let shipped = render_native_masters_request(NativeMasterKind::StockItems, name);
        assert_eq!(shipped.matches(SHIPPED_FETCH).count(), 1);
        let changed = shipped.replacen(SHIPPED_FETCH, CANDIDATE_FETCH, 1);
        assert_ne!(changed, shipped);

        let company = ValidatedCompanyName::new(name).expect("an admitted name");
        assert_eq!(render_candidate_stock_items_request(&company), changed);
    }
}
