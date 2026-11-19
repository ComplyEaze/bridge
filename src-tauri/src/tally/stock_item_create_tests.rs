//! Tests the stock-item catalogue and the name check on row 1381-R1's captured reads.
//! See `bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/PROVENANCE.md`.

use super::*;
use crate::tally::master_name::MasterNameError;
use bridge_tally_core::master_name_index::Via;
use bridge_tally_protocol::gst_details::GstRate;
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
};

const COMPANY: &str = "BRIDGE STOCK CREATE LAB";
const COMPANY_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1";
const SIBLING_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1-000000d0";
// The two created items, as the captures hold them.
const PLAIN_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1-000000d1";
const MARKED_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1-000000d2";
const BEFORE: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-before.utf16le.xml"
);
const PLAIN: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-after-plain-create.utf16le.xml"
);
const MARKED: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-after-marked-create.utf16le.xml"
);
const ALIAS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-with-alias.utf16le.xml"
);
const TWO_DATES: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/masters_stock_items_shape_lab_two_dates_live.utf16le.xml"
);
const GROUPS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-groups.utf16le.xml"
);
const UNITS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-units.utf16le.xml"
);
const MARKED_REQUEST: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-marked-create.request.utf16le.xml"
);
const PLAIN_REQUEST: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-plain-create.request.utf16le.xml"
);

fn decode(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("the capture decodes")
    .text
}

fn company() -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture(COMPANY, COMPANY_GUID)
}

fn catalogue(xml: &str) -> StockItemCatalogue {
    StockItemCatalogue::from_candidate_read(&company(), xml).expect("the captured read catalogues")
}

fn name(value: &str) -> MasterName {
    MasterName::new(value).expect("the test name is admitted")
}

fn sibling<'c>(
    catalogue: &'c StockItemCatalogue,
    value: &str,
) -> Result<SiblingGst<'c>, SiblingRefusal> {
    SiblingGst::named(catalogue, &name(value))
}

fn edited(xml: &str, from: &str, to: &str) -> String {
    assert_eq!(xml.matches(from).count(), 1, "the edit starts in one place");
    xml.replacen(from, to, 1)
}

fn draft<'c>(
    catalogue: &'c StockItemCatalogue,
    new_name: &str,
    sibling_name: &str,
    groups: &str,
    units: &str,
) -> Result<StockItemDraft<'c>, DraftRefusal> {
    let absent = catalogue
        .name_absent(&name(new_name))
        .expect("the new name is absent");
    let sibling =
        SiblingGst::named(catalogue, &name(sibling_name)).expect("the sibling is complete");
    let sibling =
        GoodsSibling::from_sibling(sibling).expect("the sibling is the measured Goods case");
    StockItemDraft::new(absent, sibling, groups, units)
}

fn taken(guid: &str, via: Via, existing: &str) -> ItemNameNotAbsent {
    ItemNameNotAbsent::Taken(NameCollision {
        kind: MasterKind::StockItem,
        via,
        guid: guid.to_string(),
        name: existing.to_string(),
    })
}

fn cut(bytes: &[u8], from: &str, to: &str) -> String {
    let mut xml = decode(bytes);
    assert_eq!(xml.matches(from).count(), 1, "the cut starts in one place");
    let start = xml.find(from).expect("the cut start occurs");
    let end = xml.rfind(to).expect("the cut end occurs") + to.len();
    xml.replace_range(start..end, "");
    xml
}

#[test]
fn the_siblings_name_is_taken_in_every_case() {
    let before = catalogue(&decode(BEFORE));
    assert_eq!(before.items().len(), 1);
    for candidate in ["SC Sibling Goods", "sc sibling goods", "SC SIBLING GOODS"] {
        assert_eq!(
            before.name_absent(&name(candidate)).unwrap_err(),
            taken(SIBLING_GUID, Via::Name, "SC Sibling Goods")
        );
    }
}

#[test]
fn folds_not_measured_for_items_are_refused_too() {
    let before = catalogue(&decode(BEFORE));
    for candidate in [
        "SC-Sibling Goods",
        "SC/Sibling Goods",
        "SCSibling Goods",
        "SC Sibling Goods.",
    ] {
        assert_eq!(
            before.name_absent(&name(candidate)).unwrap_err(),
            taken(SIBLING_GUID, Via::Name, "SC Sibling Goods")
        );
    }
}

#[test]
fn a_doubled_space_is_refused_before_any_read() {
    assert_eq!(
        MasterName::new("SC Sibling  Goods"),
        Err(MasterNameError::RepeatedSpaces)
    );
}

#[test]
fn the_alias_is_taken_once_it_is_keyed() {
    let before = catalogue(&decode(BEFORE));
    assert!(before.name_absent(&name("SC Sibling Alias")).is_ok());

    let aliased = catalogue(&decode(ALIAS));
    assert_eq!(aliased.items().len(), 3);
    for candidate in ["SC Sibling Alias", "sc sibling alias"] {
        assert_eq!(
            aliased.name_absent(&name(candidate)).unwrap_err(),
            taken(SIBLING_GUID, Via::Alias, "SC Sibling Alias")
        );
    }
}

#[test]
fn a_created_item_is_absent_before_and_taken_after() {
    let before = catalogue(&decode(BEFORE));
    let absent = before
        .name_absent(&name("SC Copied Marked"))
        .expect("the new name is absent before the create");
    assert_eq!(absent.name().as_str(), "SC Copied Marked");
    assert!(std::ptr::eq(absent.catalogue(), &before));

    let marked = catalogue(&decode(MARKED));
    assert_eq!(
        marked.name_absent(&name("SC Copied Marked")).unwrap_err(),
        taken(MARKED_GUID, Via::Name, "SC Copied Marked")
    );

    let plain = catalogue(&decode(PLAIN));
    assert_eq!(
        plain.name_absent(&name("SC Copied Goods")).unwrap_err(),
        taken(PLAIN_GUID, Via::Name, "SC Copied Goods")
    );
}

#[test]
fn the_row_name_is_indexed_when_no_language_name_holds_it() {
    let xml = decode(BEFORE);
    assert_eq!(xml.matches("<NAME>SC Sibling Goods</NAME>").count(), 1);
    // This edit stands for the measured case where a folded Create changed an
    // item's language name and left its row name (#1381's measurement comment);
    // it is not a capture of that state.
    let renamed = xml.replacen(
        "<NAME>SC Sibling Goods</NAME>",
        "<NAME>SC Renamed Goods</NAME>",
        1,
    );
    let catalogue = catalogue(&renamed);
    assert_eq!(
        catalogue
            .name_absent(&name("SC Renamed Goods"))
            .unwrap_err(),
        taken(SIBLING_GUID, Via::Name, "SC Renamed Goods")
    );
    assert_eq!(
        catalogue
            .name_absent(&name("SC Sibling Goods"))
            .unwrap_err(),
        taken(SIBLING_GUID, Via::Alias, "SC Sibling Goods")
    );
}

#[test]
fn a_read_that_cannot_show_every_name_gives_no_catalogue() {
    assert_eq!(
        StockItemCatalogue::from_candidate_read(
            &company(),
            &cut(BEFORE, "<LANGUAGENAME.LIST>", "</LANGUAGENAME.LIST>")
        )
        .unwrap_err(),
        StockItemCatalogueError::NoNameEntry { item: 0 }
    );
    assert_eq!(
        StockItemCatalogue::from_candidate_read(
            &company(),
            &cut(BEFORE, "<STOCKITEM NAME=", "</STOCKITEM>")
        )
        .unwrap_err(),
        StockItemCatalogueError::NoItems
    );
    // A language name of one no-break space passes the parser, which trims
    // ASCII only, and is refused by the index. An edit, not a captured state.
    let xml = decode(BEFORE);
    assert_eq!(xml.matches("<NAME>SC Sibling Goods</NAME>").count(), 1);
    assert_eq!(
        StockItemCatalogue::from_candidate_read(
            &company(),
            &xml.replacen("<NAME>SC Sibling Goods</NAME>", "<NAME>\u{a0}</NAME>", 1)
        )
        .unwrap_err(),
        StockItemCatalogueError::Index(MasterNameIndexError::BlankName {
            master: 0,
            position: 0
        })
    );
    let foreign =
        VerifiedCompanyIdentity::test_fixture(COMPANY, "3a6bd6e1-b835-4bff-89dd-8a6af138c346");
    assert_eq!(
        StockItemCatalogue::from_candidate_read(&foreign, &decode(BEFORE)).unwrap_err(),
        StockItemCatalogueError::Unreadable(NativeMastersError::RowGuidForeign)
    );
}

#[test]
fn the_catalogue_keeps_the_company_it_was_read_for() {
    let before = catalogue(&decode(BEFORE));
    assert_eq!(before.company(), &company());
    assert_eq!(before.company().company_guid(), COMPANY_GUID);
}

#[test]
fn the_named_sibling_carries_its_six_fields_as_read() {
    let before = catalogue(&decode(BEFORE));
    let item = sibling(&before, "SC Sibling Goods").expect("the sibling is complete");
    assert_eq!(item.guid(), SIBLING_GUID);
    assert_eq!(item.name(), "SC Sibling Goods");
    assert_eq!(item.parent(), "SC Goods Group");
    assert_eq!(item.unit(), "Nos");
    assert_eq!(
        item.applicability(),
        format!("{TALLY_SANITIZED_ROOT_MARKER} Applicable")
    );
    assert_eq!(item.supply(), "Goods");
    assert!(std::ptr::eq(item.catalogue(), &before));

    assert_eq!(item.gst().len(), 1);
    let gst = &item.gst()[0];
    assert_eq!(gst.applicable_from, "20250401");
    assert_eq!(gst.taxability.as_deref(), Some("Taxable"));
    assert_eq!(gst.source.as_deref(), Some("Specify Details Here"));
    assert_eq!(gst.ineligible_itc, None);
    assert!(!gst.other_content_skipped);
    assert_eq!(gst.states.len(), 1);
    let state = &gst.states[0];
    assert_eq!(state.state_name.as_deref(), Some("\u{fffd}#4; Any"));
    assert!(!state.slab_rates_present);
    assert_eq!(
        state
            .rates
            .iter()
            .map(|row| row.duty_head.as_deref())
            .collect::<Vec<_>>(),
        [
            Some("CGST"),
            Some("SGST/UTGST"),
            Some("IGST"),
            Some("Cess"),
            Some("State Cess")
        ]
    );
    assert_eq!(
        state
            .rates
            .iter()
            .map(|row| row.valuation_type.as_deref())
            .collect::<Vec<_>>(),
        [
            Some("Based on Value"),
            Some("Based on Value"),
            Some("Based on Value"),
            Some("\u{fffd}#4; Not Applicable"),
            Some("Based on Value")
        ]
    );
    let rates: Vec<_> = state.rates.iter().map(|row| row.rate.clone()).collect();
    assert_eq!(
        rates,
        vec![
            GstRate::Value {
                raw: " 9".to_string(),
                decimal: "9".to_string()
            },
            GstRate::Value {
                raw: " 9".to_string(),
                decimal: "9".to_string()
            },
            GstRate::Value {
                raw: " 18".to_string(),
                decimal: "18".to_string()
            },
            GstRate::Absent,
            GstRate::Absent,
        ]
    );

    assert_eq!(item.hsn().len(), 1);
    let hsn = &item.hsn()[0];
    assert_eq!(hsn.applicable_from, "20250401");
    assert_eq!(hsn.hsn_code.as_deref(), Some("39239090"));
    assert_eq!(
        hsn.description.as_deref(),
        Some("Plastic Articles for Conveyance")
    );
    assert_eq!(hsn.source.as_deref(), Some("Specify Details Here"));
    assert!(!hsn.other_content_skipped);
    let goods = GoodsSibling::from_sibling(item).expect("the measured sibling is Goods");
    assert_eq!(goods.sibling().guid(), SIBLING_GUID);
}

#[test]
fn a_sibling_is_found_by_its_exact_name_or_its_alias_and_never_by_a_fold() {
    let aliased = catalogue(&decode(ALIAS));
    for value in ["SC Sibling Goods", "SC Sibling Alias"] {
        assert_eq!(
            sibling(&aliased, value).expect("a sibling answers").guid(),
            SIBLING_GUID
        );
    }
    assert_eq!(
        sibling(&aliased, "SC Copied Marked")
            .expect("the copy answers")
            .guid(),
        MARKED_GUID
    );
    for value in [
        "sc sibling goods",
        "SC Sibling Goods.",
        "SC_Sibling_Goods",
        "SCSiblingGoods",
    ] {
        assert_eq!(
            sibling(&aliased, value).unwrap_err(),
            SiblingRefusal::OnlyByFold { items: 1 }
        );
    }
}

#[test]
fn the_marked_copy_is_a_sibling_equal_to_the_original() {
    let marked = catalogue(&decode(MARKED));
    let copy = sibling(&marked, "SC Copied Marked").expect("the copy is complete");
    let original = sibling(&marked, "SC Sibling Goods").expect("the original is complete");
    assert_eq!(copy.guid(), MARKED_GUID);
    assert_eq!(copy.name(), "SC Copied Marked");
    assert_eq!(copy.parent(), original.parent());
    assert_eq!(copy.unit(), original.unit());
    assert_eq!(copy.applicability(), original.applicability());
    assert_eq!(copy.supply(), original.supply());
    assert_eq!(copy.gst(), original.gst());
    assert_eq!(copy.hsn(), original.hsn());
    assert!(GoodsSibling::from_sibling(copy).is_ok());
}

#[test]
fn the_plain_copy_is_refused_for_the_first_field_it_lacks() {
    let plain = catalogue(&decode(PLAIN));
    assert_eq!(
        sibling(&plain, "SC Copied Goods").unwrap_err(),
        SiblingRefusal::Incomplete(SiblingField::Applicability)
    );
    let before = catalogue(&decode(BEFORE));
    assert!(sibling(&before, "SC Sibling Goods").is_ok());
    let guid = "<GUID TYPE=\"String\">bac2b5e9-ada9-43c0-b8f8-a2812a813fa1-000000d1</GUID>";
    // This edit adds arbitrary applicability text and is not a captured state.
    let xml = edited(
        &decode(PLAIN),
        guid,
        &format!("{guid}<GSTAPPLICABLE TYPE=\"String\">&#4; Applicable</GSTAPPLICABLE>"),
    );
    let plain = catalogue(&xml);
    assert_eq!(
        sibling(&plain, "SC Copied Goods").unwrap_err(),
        SiblingRefusal::Incomplete(SiblingField::GstDetails)
    );
}

#[test]
fn a_name_no_item_answers_to_is_not_found() {
    let before = catalogue(&decode(BEFORE));
    for value in ["SC Copied Marked", "SC Sibling Alias"] {
        assert_eq!(
            sibling(&before, value).unwrap_err(),
            SiblingRefusal::NotFound
        );
    }
}

#[test]
fn a_fold_two_items_share_is_refused_with_their_count_but_an_exact_name_is_not() {
    // Each edit substitutes arbitrary text and is not a captured state.
    let xml = edited(
        &decode(MARKED),
        "NAME=\"SC Copied Goods\"",
        "NAME=\"SC-Sibling Goods\"",
    );
    // This second arbitrary-text edit is likewise not a captured state.
    let xml = edited(
        &xml,
        "<NAME>SC Copied Goods</NAME>",
        "<NAME>SC-Sibling Goods</NAME>",
    );
    let marked = catalogue(&xml);
    assert_eq!(
        sibling(&marked, "sc sibling goods").unwrap_err(),
        SiblingRefusal::OnlyByFold { items: 2 }
    );
    assert_eq!(
        sibling(&marked, "SC Sibling Goods")
            .expect("exact wins")
            .guid(),
        SIBLING_GUID
    );
    assert_eq!(
        sibling(&marked, "SC-Sibling Goods").unwrap_err(),
        SiblingRefusal::Incomplete(SiblingField::Applicability)
    );
    assert_eq!(
        marked
            .items()
            .iter()
            .find(|item| item.name == "SC-Sibling Goods")
            .map(|item| item.guid.as_str()),
        Some(PLAIN_GUID)
    );
}

#[test]
fn the_shipped_read_gives_no_sibling() {
    let company =
        VerifiedCompanyIdentity::test_fixture(COMPANY, "3a6bd6e1-b835-4bff-89dd-8a6af138c346");
    let shipped = StockItemCatalogue::from_candidate_read(&company, &decode(TWO_DATES))
        .expect("the shipped read catalogues");
    assert_eq!(shipped.items().len(), 12);
    assert_eq!(
        sibling(&shipped, "HSN Two Dates Test Item").unwrap_err(),
        SiblingRefusal::Incomplete(SiblingField::Unit)
    );
}

#[test]
fn each_missing_or_unmodelled_field_is_named() {
    let cases = [
        (
            "<PARENT TYPE=\"String\">SC Goods Group</PARENT>",
            "",
            SiblingField::Parent,
        ),
        (
            "<BASEUNITS TYPE=\"String\">Nos</BASEUNITS>",
            "<BASEUNITS TYPE=\"String\"></BASEUNITS>",
            SiblingField::Unit,
        ),
        (
            "<GSTAPPLICABLE TYPE=\"String\">&#4; Applicable</GSTAPPLICABLE>",
            "",
            SiblingField::Applicability,
        ),
        (
            "<GSTTYPEOFSUPPLY TYPE=\"String\">Goods</GSTTYPEOFSUPPLY>",
            "",
            SiblingField::TypeOfSupply,
        ),
        (
            "<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST>",
            "<GSTSLABRATES.LIST><SCSLAB>1</SCSLAB></GSTSLABRATES.LIST>",
            SiblingField::GstDetails,
        ),
        (
            "<TAXABILITY TYPE=\"String\">Taxable</TAXABILITY>",
            "<TAXABILITY TYPE=\"String\">Taxable</TAXABILITY><SCUNMODELLED>1</SCUNMODELLED>",
            SiblingField::GstDetails,
        ),
        (
            "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>",
            "",
            SiblingField::GstDetails,
        ),
        (
            "<HSNCODE TYPE=\"String\">39239090</HSNCODE>",
            "",
            SiblingField::HsnDetails,
        ),
    ];
    for (from, to, field) in cases {
        // Every case edits a capture with arbitrary text, not a captured state.
        let xml = edited(&decode(BEFORE), from, to);
        let before = catalogue(&xml);
        assert_eq!(
            sibling(&before, "SC Sibling Goods").unwrap_err(),
            SiblingRefusal::Incomplete(field)
        );
    }
}

#[test]
fn each_gst_and_hsn_sub_field_is_required() {
    let cases = [
        (
            "<TAXABILITY TYPE=\"String\">Taxable</TAXABILITY>",
            "",
            SiblingField::GstDetails,
        ),
        (
            "<SRCOFGSTDETAILS TYPE=\"String\">Specify Details Here</SRCOFGSTDETAILS>",
            "",
            SiblingField::GstDetails,
        ),
        (
            "<GSTRATEDUTYHEAD TYPE=\"String\">IGST</GSTRATEDUTYHEAD>",
            "",
            SiblingField::GstDetails,
        ),
        (
            "<GSTRATEVALUATIONTYPE TYPE=\"String\">&#4; Not Applicable</GSTRATEVALUATIONTYPE>",
            "",
            SiblingField::GstDetails,
        ),
        (
            "<HSN TYPE=\"String\">Plastic Articles for Conveyance</HSN>",
            "",
            SiblingField::HsnDetails,
        ),
        (
            "<SRCOFHSNDETAILS TYPE=\"String\">Specify Details Here</SRCOFHSNDETAILS>",
            "",
            SiblingField::HsnDetails,
        ),
        (
            "<HSNCODE TYPE=\"String\">39239090</HSNCODE>",
            "<HSNCODE TYPE=\"String\">39239090</HSNCODE><SCUNMODELLED>1</SCUNMODELLED>",
            SiblingField::HsnDetails,
        ),
        (
            "<PARENT TYPE=\"String\">SC Goods Group</PARENT>",
            "<PARENT TYPE=\"String\">   </PARENT>",
            SiblingField::Parent,
        ),
    ];
    for (from, to, field) in cases {
        // Every case edits a capture with arbitrary text, not a captured state.
        let xml = edited(&decode(BEFORE), from, to);
        let before = catalogue(&xml);
        assert_eq!(
            sibling(&before, "SC Sibling Goods").unwrap_err(),
            SiblingRefusal::Incomplete(field)
        );
    }
}

#[test]
fn a_scalar_field_of_only_white_space_is_refused_like_a_missing_one() {
    let cases = [
        (
            "<BASEUNITS TYPE=\"String\">Nos</BASEUNITS>",
            "Nos",
            SiblingField::Unit,
        ),
        (
            "<GSTAPPLICABLE TYPE=\"String\">&#4; Applicable</GSTAPPLICABLE>",
            "&#4; Applicable",
            SiblingField::Applicability,
        ),
        (
            "<GSTTYPEOFSUPPLY TYPE=\"String\">Goods</GSTTYPEOFSUPPLY>",
            "Goods",
            SiblingField::TypeOfSupply,
        ),
        (
            "<PARENT TYPE=\"String\">SC Goods Group</PARENT>",
            "SC Goods Group",
            SiblingField::Parent,
        ),
    ];
    for (from, text, field) in cases {
        // Each case is a hand edit, not a captured state. A parent of only
        // white space is given as no parent by the parser, before `named`.
        let to = from.replacen(text, "&#160;", 1);
        let xml = edited(&decode(BEFORE), from, &to);
        let before = catalogue(&xml);
        assert_eq!(
            sibling(&before, "SC Sibling Goods").unwrap_err(),
            SiblingRefusal::Incomplete(field)
        );
    }
}

#[test]
fn a_padded_unit_and_parent_are_handed_back_exactly_as_read() {
    // Both hand edits add padding and are not captured states.
    let xml = edited(
        &decode(BEFORE),
        "<BASEUNITS TYPE=\"String\">Nos</BASEUNITS>",
        "<BASEUNITS TYPE=\"String\"> Nos </BASEUNITS>",
    );
    let xml = edited(
        &xml,
        "<PARENT TYPE=\"String\">SC Goods Group</PARENT>",
        "<PARENT TYPE=\"String\"> SC Goods Group </PARENT>",
    );
    let before = catalogue(&xml);
    let item = sibling(&before, "SC Sibling Goods").expect("padding is not blank");
    assert_eq!(item.unit().as_bytes(), b" Nos ");
    assert_eq!(item.parent().as_bytes(), b" SC Goods Group ");
}

#[test]
fn a_blank_gst_or_hsn_sub_field_is_refused_like_a_missing_one() {
    let cases = [
        (
            "<TAXABILITY TYPE=\"String\">Taxable</TAXABILITY>",
            "Taxable",
            SiblingField::GstDetails,
        ),
        (
            "<SRCOFGSTDETAILS TYPE=\"String\">Specify Details Here</SRCOFGSTDETAILS>",
            "Specify Details Here",
            SiblingField::GstDetails,
        ),
        (
            "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>",
            "&#4; Any",
            SiblingField::GstDetails,
        ),
        (
            "<GSTRATEDUTYHEAD TYPE=\"String\">IGST</GSTRATEDUTYHEAD>",
            "IGST",
            SiblingField::GstDetails,
        ),
        (
            "<GSTRATEVALUATIONTYPE TYPE=\"String\">&#4; Not Applicable</GSTRATEVALUATIONTYPE>",
            "&#4; Not Applicable",
            SiblingField::GstDetails,
        ),
        (
            "<HSNCODE TYPE=\"String\">39239090</HSNCODE>",
            "39239090",
            SiblingField::HsnDetails,
        ),
        (
            "<HSN TYPE=\"String\">Plastic Articles for Conveyance</HSN>",
            "Plastic Articles for Conveyance",
            SiblingField::HsnDetails,
        ),
        (
            "<SRCOFHSNDETAILS TYPE=\"String\">Specify Details Here</SRCOFHSNDETAILS>",
            "Specify Details Here",
            SiblingField::HsnDetails,
        ),
    ];
    for (from, text, field) in cases {
        for blank in ["", "   "] {
            // These edits substitute arbitrary text, not captured states.
            let to = from.replacen(text, blank, 1);
            let xml = edited(&decode(BEFORE), from, &to);
            let before = catalogue(&xml);
            assert_eq!(
                sibling(&before, "SC Sibling Goods").unwrap_err(),
                SiblingRefusal::Incomplete(field)
            );
        }
    }
}

#[test]
fn a_gst_entry_needs_a_state_row_and_a_state_row_needs_a_rate_row() {
    // Both cases edit a capture with arbitrary text, not a captured state.
    let xml = cut(
        BEFORE,
        "<STATEWISEDETAILS.LIST>",
        "</STATEWISEDETAILS.LIST>",
    );
    let before = catalogue(&xml);
    assert_eq!(
        sibling(&before, "SC Sibling Goods").unwrap_err(),
        SiblingRefusal::Incomplete(SiblingField::GstDetails)
    );
    let mut xml = decode(BEFORE);
    assert_eq!(xml.matches("<RATEDETAILS.LIST>").count(), 5);
    let start = xml
        .find("<RATEDETAILS.LIST>")
        .expect("the first rate row occurs");
    let closing = "</RATEDETAILS.LIST>";
    let end = xml.rfind(closing).expect("the last rate row occurs") + closing.len();
    xml.replace_range(start..end, "");
    let before = catalogue(&xml);
    assert_eq!(
        sibling(&before, "SC Sibling Goods").unwrap_err(),
        SiblingRefusal::Incomplete(SiblingField::GstDetails)
    );
}

#[test]
fn a_list_present_with_no_entry_is_refused() {
    // Both edits empty a captured list by hand; neither is a captured state.
    let mut xml = decode(BEFORE);
    let opening = "<GSTDETAILS.LIST>";
    let closing = "</GSTDETAILS.LIST>";
    let start = xml.find(opening).expect("the GST list occurs") + opening.len();
    let end = xml.find(closing).expect("the GST list closes");
    xml.replace_range(start..end, "");
    let before = catalogue(&xml);
    assert_eq!(
        before.items()[0].gst,
        GstDetailsObservation::Entries { entries: vec![] }
    );
    assert_eq!(
        sibling(&before, "SC Sibling Goods").unwrap_err(),
        SiblingRefusal::Incomplete(SiblingField::GstDetails)
    );

    let mut xml = decode(BEFORE);
    let opening = "<HSNDETAILS.LIST>";
    let closing = "</HSNDETAILS.LIST>";
    let start = xml.find(opening).expect("the HSN list occurs") + opening.len();
    let end = xml.find(closing).expect("the HSN list closes");
    xml.replace_range(start..end, "");
    let before = catalogue(&xml);
    assert_eq!(
        before.items()[0].hsn,
        HsnDetailsObservation::Entries { entries: vec![] }
    );
    assert_eq!(
        sibling(&before, "SC Sibling Goods").unwrap_err(),
        SiblingRefusal::Incomplete(SiblingField::HsnDetails)
    );
}

#[test]
fn one_item_reached_twice_by_a_fold_is_counted_once() {
    // This edit substitutes arbitrary text and is not a captured state.
    let xml = edited(
        &decode(ALIAS),
        "<NAME>SC Sibling Alias</NAME>",
        "<NAME>SC-Sibling Goods</NAME>",
    );
    let aliased = catalogue(&xml);
    assert_eq!(
        sibling(&aliased, "sc sibling goods").unwrap_err(),
        SiblingRefusal::OnlyByFold { items: 1 }
    );
    assert_eq!(
        sibling(&aliased, "SC-Sibling Goods")
            .expect("the second language name is exact")
            .guid(),
        SIBLING_GUID
    );
}

#[test]
fn two_items_holding_one_exact_name_are_ambiguous() {
    // This edit substitutes arbitrary text and is not a captured state.
    let xml = edited(
        &decode(MARKED),
        "<NAME>SC Copied Marked</NAME>",
        "<NAME>SC Sibling Goods</NAME>",
    );
    let marked = catalogue(&xml);
    assert_eq!(
        sibling(&marked, "SC Sibling Goods").unwrap_err(),
        SiblingRefusal::Ambiguous { items: 2 }
    );
    assert_eq!(
        sibling(&marked, "SC Copied Marked")
            .expect("the row name remains exact")
            .guid(),
        MARKED_GUID
    );
}

#[test]
fn only_a_goods_taxable_applicable_sibling_is_a_goods_sibling() {
    let cases = [
        (
            "<GSTTYPEOFSUPPLY TYPE=\"String\">Goods</GSTTYPEOFSUPPLY>",
            "<GSTTYPEOFSUPPLY TYPE=\"String\">Services</GSTTYPEOFSUPPLY>",
            NotGoodsSibling::NotGoods,
        ),
        (
            "<TAXABILITY TYPE=\"String\">Taxable</TAXABILITY>",
            "<TAXABILITY TYPE=\"String\">Exempt</TAXABILITY>",
            NotGoodsSibling::NotTaxable { entry: 0 },
        ),
        (
            "<GSTAPPLICABLE TYPE=\"String\">&#4; Applicable</GSTAPPLICABLE>",
            "<GSTAPPLICABLE TYPE=\"String\">&#4; Not Applicable</GSTAPPLICABLE>",
            NotGoodsSibling::GstNotApplicable,
        ),
        (
            "<GSTAPPLICABLE TYPE=\"String\">&#4; Applicable</GSTAPPLICABLE>",
            "<GSTAPPLICABLE TYPE=\"String\">Applicable</GSTAPPLICABLE>",
            NotGoodsSibling::GstNotApplicable,
        ),
    ];
    for (from, to, expected) in cases {
        // Every case edits a capture with arbitrary text, not a captured state.
        let xml = edited(&decode(BEFORE), from, to);
        let before = catalogue(&xml);
        let item = sibling(&before, "SC Sibling Goods").expect("the six fields remain complete");
        assert_eq!(GoodsSibling::from_sibling(item).unwrap_err(), expected);
    }
    let extra = "<GSTDETAILS.LIST><APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM><TAXABILITY TYPE=\"String\">Exempt</TAXABILITY><SRCOFGSTDETAILS TYPE=\"String\">Specify Details Here</SRCOFGSTDETAILS><STATEWISEDETAILS.LIST><STATENAME TYPE=\"String\">&#4; Any</STATENAME><RATEDETAILS.LIST><GSTRATEDUTYHEAD TYPE=\"String\">IGST</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE TYPE=\"String\">Based on Value</GSTRATEVALUATIONTYPE></RATEDETAILS.LIST></STATEWISEDETAILS.LIST></GSTDETAILS.LIST>";
    // This edit appends an arbitrary second entry and is not a captured state.
    let xml = edited(
        &decode(BEFORE),
        "</GSTDETAILS.LIST>",
        &format!("</GSTDETAILS.LIST>{extra}"),
    );
    let before = catalogue(&xml);
    let item = sibling(&before, "SC Sibling Goods").expect("both entries are complete");
    assert_eq!(item.gst().len(), 2);
    assert_eq!(item.gst()[1].applicable_from, "20250922");
    assert_eq!(
        GoodsSibling::from_sibling(item).unwrap_err(),
        NotGoodsSibling::NotTaxable { entry: 1 }
    );
}

#[test]
fn marked_constants_are_read_only_in_the_marked_form() {
    let m = TALLY_SANITIZED_ROOT_MARKER;
    for (text, expected) in [
        (format!("{m} Applicable"), MarkedConstant::Applicable),
        (format!("{m} Any"), MarkedConstant::Any),
        (format!("{m} Not Applicable"), MarkedConstant::NotApplicable),
    ] {
        assert_eq!(MarkedConstant::from_read(&text), Some(expected));
    }
    let before = catalogue(&decode(BEFORE));
    let item = sibling(&before, "SC Sibling Goods").expect("the sibling is complete");
    assert_eq!(
        MarkedConstant::from_read(item.applicability()),
        Some(MarkedConstant::Applicable)
    );
    assert_eq!(
        item.gst()[0].states[0]
            .state_name
            .as_deref()
            .and_then(MarkedConstant::from_read),
        Some(MarkedConstant::Any)
    );
    assert_eq!(
        item.gst()[0].states[0].rates[3]
            .valuation_type
            .as_deref()
            .and_then(MarkedConstant::from_read),
        Some(MarkedConstant::NotApplicable)
    );
    for text in [
        " Applicable".to_string(),
        " Any".to_string(),
        " Not Applicable".to_string(),
        format!("{m}Applicable"),
        format!("{m} Primary"),
        format!("{m} Any State"),
        "Applicable".to_string(),
        String::new(),
    ] {
        assert_eq!(MarkedConstant::from_read(&text), None);
    }
    let plain = decode_xml_bytes(PLAIN_REQUEST).expect("the plain request decodes");
    for text in [
        "<GSTAPPLICABLE> Applicable</GSTAPPLICABLE>",
        "<STATENAME> Any</STATENAME>",
        "<GSTRATEVALUATIONTYPE> Not Applicable</GSTRATEVALUATIONTYPE>",
    ] {
        assert_eq!(plain.matches(text).count(), 1);
    }
    assert_eq!(plain.matches("&#4;").count(), 0);
}

#[test]
fn the_draft_holds_the_values_the_marked_request_sent() {
    let before = catalogue(&decode(BEFORE));
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let b = &before;
    let built = draft(b, "SC Copied Marked", "SC Sibling Goods", &g, &u)
        .expect("the measured values draft");
    assert_eq!(built.name().as_str(), "SC Copied Marked");
    assert_eq!(built.parent(), "SC Goods Group");
    assert_eq!(built.unit(), "Nos");
    assert_eq!(built.sibling().sibling().guid(), SIBLING_GUID);
    assert_eq!(built.gst().applicable_from(), "20250401");
    assert_eq!(
        built
            .gst()
            .rates()
            .iter()
            .map(|rate| (rate.duty_head(), rate.valuation(), rate.rate()))
            .collect::<Vec<_>>(),
        [
            ("CGST", RateValuation::BasedOnValue, Some("9")),
            ("SGST/UTGST", RateValuation::BasedOnValue, Some("9")),
            ("IGST", RateValuation::BasedOnValue, Some("18")),
            ("Cess", RateValuation::NotApplicable, None),
            ("State Cess", RateValuation::BasedOnValue, None),
        ]
    );
    assert_eq!(
        (
            built.hsn().applicable_from(),
            built.hsn().code(),
            built.hsn().description(),
        ),
        ("20250401", "39239090", "Plastic Articles for Conveyance")
    );
    let request = decode_xml_bytes(MARKED_REQUEST).expect("the marked request decodes");
    for text in [
        "<NAME>SC Copied Marked</NAME>",
        "<PARENT>SC Goods Group</PARENT>",
        "<BASEUNITS>Nos</BASEUNITS>",
        "<GSTAPPLICABLE>&#4; Applicable</GSTAPPLICABLE>",
        "<STATENAME>&#4; Any</STATENAME>",
        "<GSTDETAILS.LIST><APPLICABLEFROM>20250401</APPLICABLEFROM><TAXABILITY>Taxable</TAXABILITY><SRCOFGSTDETAILS>Specify Details Here</SRCOFGSTDETAILS>",
        "<RATEDETAILS.LIST><GSTRATEDUTYHEAD>CGST</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>9</GSTRATE></RATEDETAILS.LIST>",
        "<RATEDETAILS.LIST><GSTRATEDUTYHEAD>SGST/UTGST</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>9</GSTRATE></RATEDETAILS.LIST>",
        "<RATEDETAILS.LIST><GSTRATEDUTYHEAD>IGST</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>18</GSTRATE></RATEDETAILS.LIST>",
        "<RATEDETAILS.LIST><GSTRATEDUTYHEAD>Cess</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE>&#4; Not Applicable</GSTRATEVALUATIONTYPE></RATEDETAILS.LIST>",
        "<RATEDETAILS.LIST><GSTRATEDUTYHEAD>State Cess</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE></RATEDETAILS.LIST>",
        "<HSNDETAILS.LIST><APPLICABLEFROM>20250401</APPLICABLEFROM><HSNCODE>39239090</HSNCODE><HSN>Plastic Articles for Conveyance</HSN><SRCOFHSNDETAILS>Specify Details Here</SRCOFHSNDETAILS></HSNDETAILS.LIST>",
    ] {
        assert_eq!(request.matches(text).count(), 1, "{text}");
    }
    assert_eq!(request.matches("<RATEDETAILS.LIST>").count(), 5);
    assert_eq!(request.matches("<GSTDETAILS.LIST>").count(), 1);
    assert_eq!(request.matches("<GSTRATE> 9</GSTRATE>").count(), 0);
}

#[test]
fn a_name_and_a_sibling_from_two_catalogues_give_no_draft() {
    let first = catalogue(&decode(BEFORE));
    let second = catalogue(&decode(BEFORE));
    let absent = first
        .name_absent(&name("SC Copied Marked"))
        .expect("the new name is absent");
    let sibling =
        SiblingGst::named(&second, &name("SC Sibling Goods")).expect("the sibling is complete");
    let sibling = GoodsSibling::from_sibling(sibling).expect("the sibling is Goods");
    assert_eq!(
        StockItemDraft::new(absent, sibling, &decode(GROUPS), &decode(UNITS)).unwrap_err(),
        DraftRefusal::OtherCatalogue
    );
}

#[test]
fn a_parent_or_unit_the_book_does_not_hold_by_that_exact_name_is_refused() {
    use DraftRefusal as R;
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let parent = "<PARENT TYPE=\"String\">SC Goods Group</PARENT>";
    let unit = "<BASEUNITS TYPE=\"String\">Nos</BASEUNITS>";
    let cases = [
        (
            parent,
            "<PARENT TYPE=\"String\">SC Other Group</PARENT>",
            DraftRefusal::ParentNotInBook,
        ),
        (
            parent,
            "<PARENT TYPE=\"String\">sc goods group</PARENT>",
            R::ParentNotInBook,
        ),
        (
            parent,
            "<PARENT TYPE=\"String\">&#4; Primary</PARENT>",
            R::ParentNotInBook,
        ),
        (
            unit,
            "<BASEUNITS TYPE=\"String\">Kgs</BASEUNITS>",
            DraftRefusal::UnitNotInBook,
        ),
        (
            unit,
            "<BASEUNITS TYPE=\"String\">nos</BASEUNITS>",
            R::UnitNotInBook,
        ),
    ];
    for (from, to, expected) in cases {
        // These edits substitute arbitrary text; they are not captured states.
        let xml = edited(&decode(BEFORE), from, to);
        let b = catalogue(&xml);
        let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), expected);
    }
}

#[test]
fn a_group_or_unit_read_that_is_unreadable_or_foreign_is_refused() {
    let before = catalogue(&decode(BEFORE));
    let guid = "<BRIDGECOMPANYGUID TYPE=\"String\">bac2b5e9-ada9-43c0-b8f8-a2812a813fa1</BRIDGECOMPANYGUID>";
    let empty_guid = "<BRIDGECOMPANYGUID TYPE=\"String\"></BRIDGECOMPANYGUID>";
    let cases = [
        (
            decode(UNITS),
            decode(UNITS),
            DraftRefusal::GroupsUnreadable(NativeMastersError::ForeignChild),
        ),
        (
            decode(GROUPS),
            decode(GROUPS),
            DraftRefusal::UnitsUnreadable(NativeMastersError::ForeignChild),
        ),
        (
            edited(&decode(GROUPS), guid, empty_guid),
            decode(UNITS),
            DraftRefusal::GroupsUnreadable(NativeMastersError::RowCompanyMismatch),
        ),
        (
            decode(GROUPS),
            edited(&decode(UNITS), guid, empty_guid),
            DraftRefusal::UnitsUnreadable(NativeMastersError::RowCompanyMismatch),
        ),
    ];
    for (g, u, expected) in cases {
        let b = &before;
        let got = draft(b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), expected);
    }
}

#[test]
fn a_gst_source_other_than_the_measured_one_is_refused() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let xml = edited(
        &decode(BEFORE),
        "<SRCOFGSTDETAILS TYPE=\"String\">Specify Details Here</SRCOFGSTDETAILS>",
        "<SRCOFGSTDETAILS TYPE=\"String\">As per Company/Stock Group</SRCOFGSTDETAILS>",
    );
    let b = catalogue(&xml);
    let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
    assert_eq!(got.unwrap_err(), DraftRefusal::GstSourceNotMeasured);
}

#[test]
fn an_ineligible_itc_flag_is_refused() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let source = "<SRCOFGSTDETAILS TYPE=\"String\">Specify Details Here</SRCOFGSTDETAILS>";
    for value in ["No", "Yes"] {
        let xml = edited(
            &decode(BEFORE),
            source,
            &format!("{source}<GSTINELIGIBLEITC TYPE=\"Logical\">{value}</GSTINELIGIBLEITC>"),
        );
        let before = catalogue(&xml);
        assert!(sibling(&before, "SC Sibling Goods").is_ok());
        let b = &before;
        let got = draft(b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), DraftRefusal::ItcNotMeasured);
    }
}

#[test]
fn a_state_row_other_than_the_marked_any_is_refused() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let from = "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>";
    for value in ["Karnataka", " Any", "&#4; Applicable"] {
        let xml = edited(
            &decode(BEFORE),
            from,
            &format!("<STATENAME TYPE=\"String\">{value}</STATENAME>"),
        );
        let b = catalogue(&xml);
        let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), DraftRefusal::StateNotAny);
    }
}

#[test]
fn a_second_dated_entry_is_refused_with_the_count() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let extra = "<GSTDETAILS.LIST><APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM><TAXABILITY TYPE=\"String\">Taxable</TAXABILITY><SRCOFGSTDETAILS TYPE=\"String\">Specify Details Here</SRCOFGSTDETAILS><STATEWISEDETAILS.LIST><STATENAME TYPE=\"String\">&#4; Any</STATENAME><RATEDETAILS.LIST><GSTRATEDUTYHEAD TYPE=\"String\">IGST</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE TYPE=\"String\">Based on Value</GSTRATEVALUATIONTYPE><GSTRATE TYPE=\"Number\"> 18</GSTRATE></RATEDETAILS.LIST></STATEWISEDETAILS.LIST></GSTDETAILS.LIST>";
    // This second GST entry is a hand edit of the capture, not a captured state.
    let xml = edited(
        &decode(BEFORE),
        "</GSTDETAILS.LIST>",
        &format!("</GSTDETAILS.LIST>{extra}"),
    );
    let b = catalogue(&xml);
    let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
    assert_eq!(
        got.unwrap_err(),
        DraftRefusal::GstEntriesNotMeasured { entries: 2 }
    );
}

#[test]
fn a_second_hsn_entry_is_refused_with_the_count() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let extra = "<HSNDETAILS.LIST><APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM><HSNCODE TYPE=\"String\">39239090</HSNCODE><HSN TYPE=\"String\">Plastic Articles for Conveyance</HSN><SRCOFHSNDETAILS TYPE=\"String\">Specify Details Here</SRCOFHSNDETAILS></HSNDETAILS.LIST>";
    // This second HSN entry is a hand edit of the capture, not a captured state.
    let xml = edited(
        &decode(BEFORE),
        "</HSNDETAILS.LIST>",
        &format!("</HSNDETAILS.LIST>{extra}"),
    );
    let b = catalogue(&xml);
    let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
    assert_eq!(
        got.unwrap_err(),
        DraftRefusal::HsnEntriesNotMeasured { entries: 2 }
    );
}

#[test]
fn the_rate_rows_must_be_the_five_measured_in_order() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    // Both head edits are hand edits of the capture, not captured states.
    for (from, to, row) in [
        (
            "<GSTRATEDUTYHEAD TYPE=\"String\">IGST</GSTRATEDUTYHEAD>",
            "<GSTRATEDUTYHEAD TYPE=\"String\">Integrated Tax</GSTRATEDUTYHEAD>",
            2,
        ),
        (
            "<GSTRATEDUTYHEAD TYPE=\"String\">State Cess</GSTRATEDUTYHEAD>",
            "<GSTRATEDUTYHEAD TYPE=\"String\">&#4; State Cess</GSTRATEDUTYHEAD>",
            4,
        ),
    ] {
        let xml = edited(&decode(BEFORE), from, to);
        let b = catalogue(&xml);
        assert!(
            sibling(&b, "SC Sibling Goods").is_ok(),
            "the edited head at row {row} remains complete"
        );
        let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), DraftRefusal::DutyHeadNotMeasured { row });
    }

    // Swap the first two captured rows by range after checking their count.
    let mut xml = decode(BEFORE);
    let opening = "<RATEDETAILS.LIST>";
    let closing = "</RATEDETAILS.LIST>";
    assert_eq!(xml.matches(opening).count(), 5);
    let first_start = xml.find(opening).expect("the first rate row occurs");
    let first_end = xml[first_start..]
        .find(closing)
        .expect("the first rate row closes")
        + first_start
        + closing.len();
    let second_start = xml[first_end..]
        .find(opening)
        .expect("the second rate row occurs")
        + first_end;
    let second_end = xml[second_start..]
        .find(closing)
        .expect("the second rate row closes")
        + second_start
        + closing.len();
    let first = xml[first_start..first_end].to_string();
    let second = xml[second_start..second_end].to_string();
    xml.replace_range(second_start..second_end, &first);
    xml.replace_range(first_start..first_end, &second);
    let b = catalogue(&xml);
    assert!(sibling(&b, "SC Sibling Goods").is_ok());
    let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
    assert_eq!(
        got.unwrap_err(),
        DraftRefusal::DutyHeadNotMeasured { row: 0 }
    );

    // Remove the last captured row by range after checking the row and head counts.
    let mut xml = decode(BEFORE);
    assert_eq!(xml.matches(opening).count(), 5);
    let head = "<GSTRATEDUTYHEAD TYPE=\"String\">State Cess</GSTRATEDUTYHEAD>";
    assert_eq!(xml.matches(head).count(), 1);
    let head_start = xml.find(head).expect("the State Cess head occurs");
    let start = xml[..head_start]
        .rfind(opening)
        .expect("the State Cess row starts");
    let end = xml[head_start..]
        .find(closing)
        .expect("the State Cess row closes")
        + head_start
        + closing.len();
    xml.replace_range(start..end, "");
    let b = catalogue(&xml);
    assert!(sibling(&b, "SC Sibling Goods").is_ok());
    let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
    assert_eq!(
        got.unwrap_err(),
        DraftRefusal::DutyHeadNotMeasured { row: 4 }
    );

    // This sixth rate row is a hand edit of the capture, not a captured state.
    let extra = "<RATEDETAILS.LIST><GSTRATEDUTYHEAD TYPE=\"String\">Other Tax</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE TYPE=\"String\">Based on Value</GSTRATEVALUATIONTYPE></RATEDETAILS.LIST>";
    let xml = edited(
        &decode(BEFORE),
        "<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST>",
        &format!("{extra}<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST>"),
    );
    let b = catalogue(&xml);
    assert!(sibling(&b, "SC Sibling Goods").is_ok());
    let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
    assert_eq!(
        got.unwrap_err(),
        DraftRefusal::DutyHeadNotMeasured { row: 5 }
    );
}

#[test]
fn a_valuation_other_than_the_one_measured_for_its_head_is_refused() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    // Change IGST by range after checking the head and valuation counts. This
    // hand edit uses the other modelled valuation and is not a captured state.
    let mut xml = decode(BEFORE);
    let head = "<GSTRATEDUTYHEAD TYPE=\"String\">IGST</GSTRATEDUTYHEAD>";
    let valuation = "<GSTRATEVALUATIONTYPE TYPE=\"String\">Based on Value</GSTRATEVALUATIONTYPE>";
    assert_eq!(xml.matches(head).count(), 1);
    assert_eq!(xml.matches(valuation).count(), 4);
    let after_head = xml.find(head).expect("the IGST head occurs") + head.len();
    let start = xml[after_head..]
        .find(valuation)
        .expect("the IGST valuation occurs")
        + after_head;
    xml.replace_range(
        start..start + valuation.len(),
        "<GSTRATEVALUATIONTYPE TYPE=\"String\">&#4; Not Applicable</GSTRATEVALUATIONTYPE>",
    );
    let b = catalogue(&xml);
    let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
    assert_eq!(
        got.unwrap_err(),
        DraftRefusal::ValuationNotMeasured { row: 2 }
    );

    // This Cess edit likewise uses the other modelled valuation.
    let xml = edited(
        &decode(BEFORE),
        "<GSTRATEVALUATIONTYPE TYPE=\"String\">&#4; Not Applicable</GSTRATEVALUATIONTYPE>",
        "<GSTRATEVALUATIONTYPE TYPE=\"String\">Based on Value</GSTRATEVALUATIONTYPE>",
    );
    let b = catalogue(&xml);
    let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
    assert_eq!(
        got.unwrap_err(),
        DraftRefusal::ValuationNotMeasured { row: 3 }
    );
}

#[test]
fn a_valuation_other_than_the_two_measured_is_refused() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let from = "<GSTRATEVALUATIONTYPE TYPE=\"String\">&#4; Not Applicable</GSTRATEVALUATIONTYPE>";
    for value in [" Not Applicable", "&#4; Applicable", "Based on Quantity"] {
        let xml = edited(
            &decode(BEFORE),
            from,
            &format!("<GSTRATEVALUATIONTYPE TYPE=\"String\">{value}</GSTRATEVALUATIONTYPE>"),
        );
        let b = catalogue(&xml);
        let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(
            got.unwrap_err(),
            DraftRefusal::ValuationNotMeasured { row: 3 }
        );
    }
}

#[test]
fn a_rate_other_than_as_measured_for_its_head_is_refused() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    // Every rate edit is a hand edit of the capture, not a captured state.
    for (head, rate, rate_count, row) in [
        (
            "<GSTRATEDUTYHEAD TYPE=\"String\">CGST</GSTRATEDUTYHEAD>",
            "<GSTRATE TYPE=\"Number\"> 9</GSTRATE>",
            2,
            0,
        ),
        (
            "<GSTRATEDUTYHEAD TYPE=\"String\">IGST</GSTRATEDUTYHEAD>",
            "<GSTRATE TYPE=\"Number\"> 18</GSTRATE>",
            1,
            2,
        ),
    ] {
        // Remove each rate by range after checking the head and rate counts.
        let mut xml = decode(BEFORE);
        assert_eq!(xml.matches(head).count(), 1);
        assert_eq!(xml.matches(rate).count(), rate_count);
        let after_head = xml.find(head).expect("the duty head occurs") + head.len();
        let start = xml[after_head..]
            .find(rate)
            .expect("the duty head's rate occurs")
            + after_head;
        xml.replace_range(start..start + rate.len(), "");
        let before = catalogue(&xml);
        assert!(SiblingGst::named(&before, &name("SC Sibling Goods")).is_ok());
        let got = draft(&before, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), DraftRefusal::RateNotMeasured { row });
    }
    for (head, row) in [
        ("<GSTRATEDUTYHEAD TYPE=\"String\">Cess</GSTRATEDUTYHEAD>", 3),
        (
            "<GSTRATEDUTYHEAD TYPE=\"String\">State Cess</GSTRATEDUTYHEAD>",
            4,
        ),
    ] {
        // Add each rate by range after checking the head and rate-row counts.
        let mut xml = decode(BEFORE);
        assert_eq!(xml.matches(head).count(), 1);
        assert_eq!(xml.matches("<RATEDETAILS.LIST>").count(), 5);
        let after_head = xml.find(head).expect("the Cess duty head occurs") + head.len();
        let end = xml[after_head..]
            .find("</RATEDETAILS.LIST>")
            .expect("the Cess rate row closes")
            + after_head;
        xml.insert_str(end, "<GSTRATE TYPE=\"Number\"> 1</GSTRATE>");
        let before = catalogue(&xml);
        assert!(SiblingGst::named(&before, &name("SC Sibling Goods")).is_ok());
        let got = draft(&before, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), DraftRefusal::RateNotMeasured { row });
    }
}

#[test]
fn a_zero_or_empty_rate_is_refused() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let from = "<GSTRATE TYPE=\"Number\"> 18</GSTRATE>";
    for (to, expected_rate) in [
        ("<GSTRATE TYPE=\"Number\"> 0</GSTRATE>", GstRate::Zero),
        ("<GSTRATE TYPE=\"Number\"></GSTRATE>", GstRate::Empty),
    ] {
        let xml = edited(&decode(BEFORE), from, to);
        let before = catalogue(&xml);
        let item = sibling(&before, "SC Sibling Goods").expect("the sibling is complete");
        assert_eq!(&item.gst()[0].states[0].rates[2].rate, &expected_rate);
        let b = &before;
        let got = draft(b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), DraftRefusal::RateNotMeasured { row: 2 });
    }
}

#[test]
fn an_hsn_entry_other_than_the_measured_shape_is_refused() {
    let g = decode(GROUPS);
    let u = decode(UNITS);
    let cases = [
        (
            "<SRCOFHSNDETAILS TYPE=\"String\">Specify Details Here</SRCOFHSNDETAILS>",
            "<SRCOFHSNDETAILS TYPE=\"String\">As per Company/Stock Group</SRCOFHSNDETAILS>",
        ),
        (
            "<HSNCODE TYPE=\"String\">39239090</HSNCODE>",
            "<HSNCODE TYPE=\"String\">&#4; 39239090</HSNCODE>",
        ),
        (
            "<HSN TYPE=\"String\">Plastic Articles for Conveyance</HSN>",
            "<HSN TYPE=\"String\">&#4; Plastic Articles</HSN>",
        ),
    ];
    for (from, to) in cases {
        let xml = edited(&decode(BEFORE), from, to);
        let b = catalogue(&xml);
        let got = draft(&b, "SC Copied Marked", "SC Sibling Goods", &g, &u);
        assert_eq!(got.unwrap_err(), DraftRefusal::HsnNotMeasured);
    }
}
