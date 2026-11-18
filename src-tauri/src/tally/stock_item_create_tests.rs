//! Tests the stock-item catalogue and the name check on row 1381-R1's captured reads.
//! See `bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/PROVENANCE.md`.

use super::*;
use crate::tally::ledger_create_item::MasterNameError;
use bridge_tally_core::master_name_index::Via;
use bridge_tally_protocol::gst_details::GstRate;
use bridge_tally_protocol::{decode_tally_xml_response_bytes_limited, ExpectedTallyTextEncoding};

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
fn a_sibling_is_found_by_its_exact_name_its_alias_or_a_fold() {
    let aliased = catalogue(&decode(ALIAS));
    for value in ["SC Sibling Alias", "sc sibling goods", "SC Sibling Goods."] {
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
fn a_fold_two_items_share_is_ambiguous_but_an_exact_name_is_not() {
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
        SiblingRefusal::Ambiguous { items: 2 }
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
fn one_item_reached_twice_by_a_fold_is_one_sibling() {
    // This edit substitutes arbitrary text and is not a captured state.
    let xml = edited(
        &decode(ALIAS),
        "<NAME>SC Sibling Alias</NAME>",
        "<NAME>SC-Sibling Goods</NAME>",
    );
    let aliased = catalogue(&xml);
    assert_eq!(
        sibling(&aliased, "sc sibling goods")
            .expect("two folds of one item are one sibling")
            .guid(),
        SIBLING_GUID
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
