//! GST details on a voucher's ledger entries (#2027), on the live capture of a year of the
//! synthetic book `BRIDGE PILOT LAB` read by the request `vouchers` sends (697-S0A; provenance
//! `fixtures/agent/audit-read-pilot-lab/PROVENANCE.md`). The other cases change one element of
//! that capture; none is authored from nothing.
use super::*;

const COMPANY_GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

fn captured() -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/audit-read-pilot-lab/pilot-lab-audit-vouchers.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn parse(xml: &str) -> Vec<Value> {
    parse_agent_rows(xml, COMPANY_GUID).expect("the read still succeeds")
}

fn entries(rows: &[Value]) -> Vec<&Value> {
    rows.iter()
        .flat_map(|row| row["amounts"].as_array().into_iter().flatten())
        .collect()
}

fn value_rates(entry: &Value) -> Vec<&Value> {
    entry["gst_details"]["rates"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|rate| rate["rate"]["kind"] == "value")
        .collect()
}

/// The capture's 48 taxed entries each carry three non-zero rates (CGST and SGST/UTGST at the
/// same rate, or IGST alone with Cess and State Cess zero), a taxability and a type of supply.
#[test]
fn every_taxed_entry_of_the_capture_carries_its_rates_taxability_and_supply_type() {
    let rows = parse(&captured());
    let all = entries(&rows);
    let taxed: Vec<_> = all
        .iter()
        .copied()
        .filter(|entry| !value_rates(entry).is_empty())
        .collect();
    assert_eq!(taxed.len(), 48);
    assert_eq!(
        taxed
            .iter()
            .map(|entry| value_rates(entry).len())
            .sum::<usize>(),
        144
    );
    for entry in &taxed {
        let details = &entry["gst_details"];
        assert_eq!(
            details["taxability"],
            json!({"state": "value", "value": "Taxable"})
        );
        let supply = &details["type_of_supply"]["value"];
        assert!(supply == "Goods" || supply == "Services", "{details}");
        let rates = details["rates"].as_array().unwrap();
        assert_eq!(rates.len(), 5, "{details}");
        let heads: Vec<_> = rates
            .iter()
            .map(|rate| &rate["duty_head"]["value"])
            .collect();
        assert_eq!(heads, ["CGST", "SGST/UTGST", "IGST", "Cess", "State Cess"]);
        for rate in rates {
            match rate["rate"]["kind"].as_str() {
                Some("value") => {
                    let raw = rate["rate"]["raw"].as_str().unwrap();
                    assert!(
                        raw.starts_with(' '),
                        "the raw text is kept as sent: {raw:?}"
                    );
                    assert_eq!(rate["rate"]["decimal"], raw.trim());
                }
                Some("zero") => {}
                other => panic!("a taxed entry's rate is a value or zero, not {other:?}"),
            }
        }
    }
    assert!(all
        .iter()
        .all(|entry| entry.get("gst_details_unreadable").is_none()));
}

/// A party or tax entry carries an empty rate list: it is listed as no rate, never as zero.
#[test]
fn an_empty_rate_list_is_no_rate_not_zero() {
    let rows = parse(&captured());
    let untaxed: Vec<_> = entries(&rows)
        .into_iter()
        .filter(|entry| value_rates(entry).is_empty())
        .filter(|entry| entry.get("gst_details").is_some())
        .collect();
    assert!(!untaxed.is_empty());
    for entry in untaxed {
        let rates = entry["gst_details"]["rates"].as_array().unwrap();
        assert!(
            rates.iter().all(|rate| rate["rate"]["kind"] != "zero"),
            "an empty list holds no zero rate: {entry}"
        );
    }
}

/// The capture's first taxed rate row, changed to say `rate_text` (or with no `GSTRATE`).
fn with_first_rate(rate: Option<&str>) -> String {
    let xml = captured();
    let element = "<GSTRATE> 2.50</GSTRATE>";
    let at = xml.find(element).expect("the capture holds a 2.50 rate");
    let replacement = match rate {
        Some(text) => format!("<GSTRATE>{text}</GSTRATE>"),
        None => String::new(),
    };
    format!(
        "{}{}{}",
        &xml[..at],
        replacement,
        &xml[at + element.len()..]
    )
}

fn first_changed_rate(xml: &str) -> Value {
    let rows = parse(xml);
    let entry = entries(&rows)
        .into_iter()
        .find(|entry| {
            entry["gst_details"]["rates"]
                .as_array()
                .is_some_and(|rates| rates.iter().any(|rate| rate["rate"]["kind"] != "zero"))
        })
        .cloned()
        .expect("a taxed entry");
    entry["gst_details"]["rates"][0]["rate"].clone()
}

/// Absent, empty, zero and a value are four readings, never merged.
#[test]
fn an_absent_empty_or_zero_rate_is_reported_as_such() {
    assert_eq!(
        first_changed_rate(&with_first_rate(None)),
        json!({"kind": "absent"})
    );
    assert_eq!(
        first_changed_rate(&with_first_rate(Some(""))),
        json!({"kind": "empty"})
    );
    assert_eq!(
        first_changed_rate(&with_first_rate(Some("  "))),
        json!({"kind": "empty"})
    );
    assert_eq!(
        first_changed_rate(&with_first_rate(Some("0.00"))),
        json!({"kind": "zero"})
    );
    assert_eq!(
        first_changed_rate(&with_first_rate(Some("12"))),
        json!({"kind": "value", "raw": "12", "decimal": "12"})
    );
}

/// The entry with the changed rate row: its details, or why they could not be read.
fn entry_with(xml: &str) -> Value {
    let rows = parse(xml);
    entries(&rows)
        .into_iter()
        .find(|entry| entry.get("gst_details_unreadable").is_some())
        .cloned()
        .expect("the changed entry says why")
}

/// A rate that is not a decimal, or a duty head named twice, leaves the entry with a reason and
/// no partial details; the read and every other entry stand.
#[test]
fn an_unreadable_rate_list_leaves_a_reason_not_part_of_the_list() {
    let entry = entry_with(&with_first_rate(Some("2,50")));
    assert_eq!(
        entry["gst_details_unreadable"],
        "gst_details_rate_not_decimal"
    );
    assert!(entry.get("gst_details").is_none());

    let xml = captured();
    let head = "<GSTRATEDUTYHEAD>SGST/UTGST</GSTRATEDUTYHEAD>";
    let at = xml.find(head).expect("the capture holds an SGST/UTGST row");
    let repeated = format!(
        "{}<GSTRATEDUTYHEAD>CGST</GSTRATEDUTYHEAD>{}",
        &xml[..at],
        &xml[at + head.len()..]
    );
    let entry = entry_with(&repeated);
    assert_eq!(
        entry["gst_details_unreadable"],
        "gst_details_duty_head_repeated"
    );
    assert!(entry.get("gst_details").is_none());
}

/// A field met twice in an entry leaves the entry with a reason.
#[test]
fn a_taxability_sent_twice_is_unreadable() {
    let xml = captured();
    let field = "<GSTOVRDNTAXABILITY>Taxable</GSTOVRDNTAXABILITY>";
    let at = xml.find(field).expect("the capture holds a taxability");
    let twice = format!("{}{field}{field}{}", &xml[..at], &xml[at + field.len()..]);
    let entry = entry_with(&twice);
    assert_eq!(
        entry["gst_details_unreadable"],
        "gst_details_field_duplicated"
    );
}

/// Without `include_gst_details` an answer carries no GST key at all, so it is unchanged; with it
/// the details are kept.
#[test]
fn the_details_are_shown_only_when_asked_for() {
    assert_eq!(
        GstDetailsShown::from_args(&json!({})),
        Ok(GstDetailsShown::Withheld)
    );
    assert_eq!(
        GstDetailsShown::from_args(&json!({"include_gst_details": false})),
        Ok(GstDetailsShown::Withheld)
    );
    assert_eq!(
        GstDetailsShown::from_args(&json!({"include_gst_details": true})),
        Ok(GstDetailsShown::Shown)
    );
    let rows = parse(&captured());
    for row in &rows {
        let withheld = present_voucher_gst_details(row.clone(), GstDetailsShown::Withheld);
        let text = withheld.to_string();
        assert!(!text.contains("gst_details"), "{text}");
        let mut expected = row.clone();
        if let Some(entries) = expected["amounts"].as_array_mut() {
            for entry in entries {
                let entry = entry.as_object_mut().unwrap();
                entry.remove("gst_details");
                entry.remove("gst_details_unreadable");
            }
        }
        assert_eq!(withheld, expected);
        assert_eq!(
            present_voucher_gst_details(row.clone(), GstDetailsShown::Shown),
            *row
        );
    }
}

/// Every rate list of the capture that holds a row is listed, and only those: its 240 rows are
/// the 48 taxed entries' five each. Its other 119 lists are empty placeholders.
#[test]
fn only_a_rate_list_that_holds_a_row_is_listed() {
    let rows = parse(&captured());
    let listed: usize = entries(&rows)
        .iter()
        .map(|entry| entry["gst_details"]["rates"].as_array().map_or(0, Vec::len))
        .sum();
    assert_eq!(listed, 240);
}

/// A taxability Tally did not send, or sent empty, is reported as such, never as a value.
#[test]
fn an_absent_or_empty_taxability_is_reported_as_such() {
    let field = "<GSTOVRDNTAXABILITY>Taxable</GSTOVRDNTAXABILITY>";
    for (replacement, state) in [
        ("", "absent"),
        ("<GSTOVRDNTAXABILITY></GSTOVRDNTAXABILITY>", "empty"),
    ] {
        let xml = captured();
        let at = xml.find(field).expect("the capture holds a taxability");
        let changed = format!("{}{replacement}{}", &xml[..at], &xml[at + field.len()..]);
        let rows = parse(&changed);
        let entry = entries(&rows)
            .into_iter()
            .find(|entry| !value_rates(entry).is_empty())
            .cloned()
            .expect("a taxed entry");
        assert_eq!(entry["gst_details"]["taxability"], json!({"state": state}));
    }
}
