//! Tests the stock-item catalogue and the name check on row 1381-R1's captured reads.
//! See `bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/PROVENANCE.md`.

use super::*;
use crate::tally::ledger_create_item::MasterNameError;
use bridge_tally_core::master_name_index::Via;
use bridge_tally_protocol::{decode_tally_xml_response_bytes_limited, ExpectedTallyTextEncoding};

const COMPANY: &str = "BRIDGE STOCK CREATE LAB";
const COMPANY_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1";
const SIBLING_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1-000000d0";
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
    let guid = &marked
        .items()
        .iter()
        .find(|item| item.name == "SC Copied Marked")
        .expect("the marked capture holds the created item")
        .guid;
    assert_eq!(
        marked.name_absent(&name("SC Copied Marked")).unwrap_err(),
        taken(guid, Via::Name, "SC Copied Marked")
    );

    let plain = catalogue(&decode(PLAIN));
    assert!(matches!(
        plain.name_absent(&name("SC Copied Goods")),
        Err(ItemNameNotAbsent::Taken(NameCollision {
            kind: MasterKind::StockItem,
            via: Via::Name,
            ..
        }))
    ));
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
    let foreign =
        VerifiedCompanyIdentity::test_fixture(COMPANY, "3a6bd6e1-b835-4bff-89dd-8a6af138c346");
    assert_eq!(
        StockItemCatalogue::from_candidate_read(&foreign, &decode(BEFORE)).unwrap_err(),
        StockItemCatalogueError::Unreadable(NativeMastersError::RowGuidForeign)
    );
}
