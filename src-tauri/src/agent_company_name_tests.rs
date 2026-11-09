//! A company whose name a request cannot carry is listed and marked, and only
//! its own reads refuse (#2433). In-memory mutations of one retained capture.
use super::*;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const MARKED_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const MARKED_NAME: &str = "WR2 Unicode Lab";
const OTHER_GUID: &str = "c6afd306-00e1-4f51-802a-babe44daddd3";

fn captured() -> String {
    let bytes = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-companies.utf16le.xml");
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The capture with one company's name, in its attribute and its element,
/// replaced by `name` as XML text.
fn with_name(name_xml: &str) -> String {
    let xml = captured();
    assert_eq!(xml.matches(MARKED_NAME).count(), 2, "attribute and element");
    xml.replace(MARKED_NAME, name_xml)
}

fn row<'a>(companies: &'a [TallyCompany], guid: &str) -> &'a TallyCompany {
    companies
        .iter()
        .find(|company| company.guid.as_deref() == Some(guid))
        .unwrap()
}

#[test]
fn every_company_lists_and_only_the_unsendable_one_is_marked() {
    let long = "x".repeat(361);
    for (name_xml, held, refusal, shown) in [
        (
            long.as_str(),
            long.clone(),
            Some("company_name_length_not_established"),
            long.clone(),
        ),
        (
            "Lab&#x9;",
            "Lab\t".to_string(),
            Some("company_name_control_character"),
            "Lab\\u{0009}".to_string(),
        ),
        (
            "&#x202e;Lab&#x1;",
            "\u{202e}Lab\u{1}".to_string(),
            Some("company_name_control_character"),
            "\\u{202e}Lab\\u{0001}".to_string(),
        ),
        (
            "   ",
            "   ".to_string(),
            Some("company_name_blank"),
            "   ".to_string(),
        ),
        (
            "  Edge Lab  ",
            "  Edge Lab  ".to_string(),
            None,
            "  Edge Lab  ".to_string(),
        ),
        // A sendable name with a right-to-left override: not marked, but the
        // override is shown as its code so it cannot reorder the text.
        (
            "&#x202e;Lab",
            "\u{202e}Lab".to_string(),
            None,
            "\\u{202e}Lab".to_string(),
        ),
        // A joiner is part of how this Devanagari name is written: shown as is.
        (
            "\u{915}\u{94d}&#x200d;\u{937} Lab",
            "\u{915}\u{94d}\u{200d}\u{937} Lab".to_string(),
            None,
            "\u{915}\u{94d}\u{200d}\u{937} Lab".to_string(),
        ),
        // A non-joiner is part of how this Persian name is written: shown as is.
        (
            "\u{645}\u{6cc}&#x200c;\u{62e}\u{648}\u{627}\u{647}\u{645} Lab",
            "\u{645}\u{6cc}\u{200c}\u{62e}\u{648}\u{627}\u{647}\u{645} Lab".to_string(),
            None,
            "\u{645}\u{6cc}\u{200c}\u{62e}\u{648}\u{627}\u{647}\u{645} Lab".to_string(),
        ),
        // Every other invisible character in a sendable name is shown as its
        // code: a zero-width space would make two names draw alike, a tag
        // character carries text no person sees, and a name of one Hangul
        // filler would draw as nothing.
        (
            "Acme&#x200b; Lab",
            "Acme\u{200b} Lab".to_string(),
            None,
            "Acme\\u{200b} Lab".to_string(),
        ),
        (
            "Lab&#xe0041;",
            "Lab\u{e0041}".to_string(),
            None,
            "Lab\\u{e0041}".to_string(),
        ),
        (
            "&#x3164;",
            "\u{3164}".to_string(),
            None,
            "\\u{3164}".to_string(),
        ),
    ] {
        let xml = with_name(name_xml);
        let companies = bridge_tally_protocol::parse_companies_from_collection(&xml).unwrap();
        assert_eq!(companies.len(), 16, "{name_xml}: no company is left out");
        let marked = row(&companies, MARKED_GUID);
        assert_eq!(
            marked.name, held,
            "the name is held byte for byte, never trimmed"
        );
        let json = company_json(marked, &companies);
        assert_eq!(json["name_refusal"], json!(refusal));
        assert_eq!(json["name"], shown);
        // A blank name is also an incomplete identity, as it was before (#2433).
        let blank = refusal == Some("company_name_blank");
        assert_eq!(
            json["identity_state"],
            if blank {
                "incomplete_tuple"
            } else {
                "verified_tuple"
            }
        );
        let other = company_json(row(&companies, OTHER_GUID), &companies);
        assert!(
            other.get("name_refusal").is_none(),
            "an unmarked row has no such key"
        );
        // The unmarked row's whole block: the keys and values master gives it.
        let unmarked = row(&companies, OTHER_GUID);
        assert_eq!(
            other,
            json!({
                "name": "Bridge Validation Lab",
                "guid": OTHER_GUID,
                "company_number": unmarked.company_number,
                "books_from": "20250401",
                "identity_state": "verified_tuple",
                "missing_field": null,
            })
        );
        for company in &companies {
            let json = company_json(company, &companies);
            if let Some(code) = json["name_refusal"].as_str() {
                assert!(
                    !code.contains(&company.name),
                    "the reason carries no name text"
                );
            }
        }
    }
}

async fn verified(xml: &str, guid: &str) -> Result<String, String> {
    let plan = ScenarioPlan::new(Fixture::SyntheticXml(xml.to_string()))
        .with_encoding(WireEncoding::Utf16Le);
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    let simulator =
        SequenceSimulator::spawn(vec![plan.clone(), status.clone(), plan, status]).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = super::identity_tests::server(simulator.address(), directory.path());
    server
        .verified_company(guid)
        .await
        .map(|(company, _, _)| company.name)
        .map_err(|failure| failure.code)
}

#[tokio::test]
async fn only_the_marked_company_refuses_its_reads() {
    let xml = with_name(&"x".repeat(361));
    assert_eq!(
        verified(&xml, MARKED_GUID).await,
        Err("company_name_invalid".to_string())
    );
    assert_eq!(
        verified(&xml, OTHER_GUID).await,
        Ok("Bridge Validation Lab".to_string())
    );
    let edge = with_name("  Edge Lab  ");
    assert_eq!(
        verified(&edge, MARKED_GUID).await,
        Ok("  Edge Lab  ".to_string()),
        "a name with edge spaces is read under its own spelling"
    );
}
