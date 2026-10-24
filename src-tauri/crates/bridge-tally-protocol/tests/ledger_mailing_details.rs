//! `LEDMAILINGDETAILS.LIST` read from a live `List of Ledgers` capture (fixture
//! `native-party-masters-mailing-details.utf16le.xml`, provenance in
//! `native-party-masters-mailing-details.PROVENANCE.md`), and the same read of
//! an earlier capture whose request did not name the list. The negative cases
//! are edits of the real bytes, each asserting the typed variant.
use bridge_tally_protocol::mailing_details::{
    MailingDetailsDefect, MailingDetailsEntry, MailingDetailsHistory,
};
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited,
    parse_native_party_ledger_master_records_with_evidence, ExpectedTallyTextEncoding,
    PartyLedgerMasterFieldObservation as Observation, PartyLedgerMasterFields,
};

const MAILING: &[u8] =
    include_bytes!("fixtures/agent/native-party-masters-mailing-details.utf16le.xml");
const WITHOUT_ELEMENT: &[u8] = include_bytes!("fixtures/agent/native-party-masters.utf16le.xml");
/// The same book read by the request that did not name the list.
const EARLIER_READ: &[u8] =
    include_bytes!("fixtures/agent/native-party-masters-without-mailing-details.utf16le.xml");
const MAILING_GUID: &str = "8e34483e-2bd3-472b-ae37-c4b197e8c15a";
const WITHOUT_ELEMENT_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const PROBE: &str = "Lab Party Duplicate Probe";

fn decoded(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("the capture decodes")
    .text
}

/// Each ledger's name (as written) and its parsed master fields, in document order.
fn parse(text: &str, guid: &str) -> anyhow::Result<Vec<(String, PartyLedgerMasterFields)>> {
    Ok(
        parse_native_party_ledger_master_records_with_evidence(text, guid)?
            .records
            .into_iter()
            .map(|r| (r.record.ledger.name, r.record.fields))
            .collect(),
    )
}

fn fields_of<'a>(
    parsed: &'a [(String, PartyLedgerMasterFields)],
    name: &str,
) -> &'a PartyLedgerMasterFields {
    &parsed
        .iter()
        .find(|(n, _)| n == name)
        .unwrap_or_else(|| panic!("ledger {name} parsed"))
        .1
}

fn probe_entry() -> MailingDetailsEntry {
    MailingDetailsEntry {
        applicable_from: bridge_tally_primitives::TallyDate::parse("20260401").unwrap(),
        mailing_name: Some(PROBE.to_string()),
        address_lines: vec!["1 Test Street".to_string(), "Test Area".to_string()],
        state: Some("Karnataka".to_string()),
        country: Some("India".to_string()),
        pin_code: Some("560001".to_string()),
        other_content_skipped: false,
    }
}

#[test]
fn the_probe_party_holds_one_dated_mailing_entry_and_every_other_ledger_an_empty_history() {
    let parsed = parse(&decoded(MAILING), MAILING_GUID).expect("the capture parses");
    assert_eq!(parsed.len(), 8);
    for (name, fields) in &parsed {
        if name == PROBE {
            assert_eq!(
                fields.mailing_details,
                MailingDetailsHistory::Entries {
                    entries: vec![probe_entry()]
                }
            );
        } else {
            assert_eq!(
                fields.mailing_details,
                MailingDetailsHistory::Entries { entries: vec![] },
                "{name}: a placeholder is an observed empty history"
            );
        }
    }
}

#[test]
fn the_flat_address_fields_stay_as_tally_returned_them_beside_the_mailing_entry() {
    let parsed = parse(&decoded(MAILING), MAILING_GUID).expect("the capture parses");
    let probe = fields_of(&parsed, PROBE);
    assert_eq!(probe.pin_code, Observation::Returned(String::new()));
    assert_eq!(probe.gst_pin_code, Observation::Returned(String::new()));
    assert_eq!(probe.address, Observation::Returned(String::new()));
    assert_eq!(probe.state, Observation::NotObserved);
}

#[test]
fn a_response_without_the_element_is_not_observed_not_empty() {
    let parsed =
        parse(&decoded(WITHOUT_ELEMENT), WITHOUT_ELEMENT_GUID).expect("the capture parses");
    assert!(!parsed.is_empty());
    for (name, fields) in &parsed {
        assert_eq!(
            fields.mailing_details,
            MailingDetailsHistory::NotObserved,
            "{name}"
        );
    }
}

/// The read whose FETCH did not name the list answered the same eight ledgers with the
/// same flat fields and no mailing element: adding the list to the request changed
/// nothing else the parser keeps.
#[test]
fn the_earlier_read_differs_from_the_mailing_read_only_by_the_mailing_history() {
    let earlier = parse(&decoded(EARLIER_READ), MAILING_GUID).expect("the earlier capture parses");
    let mailing = parse(&decoded(MAILING), MAILING_GUID).expect("the capture parses");
    assert_eq!(earlier.len(), 8);
    assert_eq!(
        earlier.iter().map(|(name, _)| name).collect::<Vec<_>>(),
        mailing.iter().map(|(name, _)| name).collect::<Vec<_>>()
    );
    for ((name, before), (_, after)) in earlier.iter().zip(&mailing) {
        assert_eq!(
            before.mailing_details,
            MailingDetailsHistory::NotObserved,
            "{name}"
        );
        let mut after = after.clone();
        after.mailing_details = MailingDetailsHistory::NotObserved;
        assert_eq!(*before, after, "{name}");
    }
}

/// The probe's element with `edit` applied to its text, parsed.
fn probe_history_after(edit: impl Fn(&str) -> String) -> MailingDetailsHistory {
    let text = decoded(MAILING);
    let probe_at = text.find(PROBE).expect("the probe ledger");
    let start = probe_at
        + text[probe_at..]
            .find("<LEDMAILINGDETAILS.LIST>")
            .expect("the probe's element");
    let end = start + text[start..].find("</LEDMAILINGDETAILS.LIST>").unwrap();
    let element = &text[start..end];
    let edited = edit(element);
    assert_ne!(edited, element, "the edit changed the element");
    let text = format!("{}{}{}", &text[..start], edited, &text[end..]);
    let parsed = parse(&text, MAILING_GUID).expect("the edited capture still parses");
    fields_of(&parsed, PROBE).mailing_details.clone()
}

fn unreadable(defect: MailingDetailsDefect) -> MailingDetailsHistory {
    MailingDetailsHistory::Unreadable { defect }
}

#[test]
fn an_edited_capture_fails_that_ledger_with_its_own_typed_defect() {
    assert_eq!(
        probe_history_after(|e| e.replace(
            "<APPLICABLEFROM TYPE=\"Date\">20260401</APPLICABLEFROM>",
            ""
        )),
        unreadable(MailingDetailsDefect::EntryWithoutDate)
    );
    assert_eq!(
        probe_history_after(|e| e.replace("20260401", "2026-04-01")),
        unreadable(MailingDetailsDefect::DateInvalid)
    );
    assert_eq!(
        probe_history_after(|e| e.replace(
            "<STATE TYPE=\"String\">Karnataka</STATE>",
            "<STATE TYPE=\"String\">Karnataka</STATE><STATE TYPE=\"String\">Goa</STATE>"
        )),
        unreadable(MailingDetailsDefect::EntryRepeatsAField)
    );
    // The same date twice with a different PIN code: the element, then a second
    // copy of it that says 560002.
    assert_eq!(
        probe_history_after(|e| {
            let body = e.trim_start_matches("<LEDMAILINGDETAILS.LIST>");
            format!(
                "{e}</LEDMAILINGDETAILS.LIST><LEDMAILINGDETAILS.LIST>{}",
                body.replace("560001", "560002")
            )
        }),
        unreadable(MailingDetailsDefect::ConflictingEntriesOnOneDate)
    );
}

#[test]
fn two_address_lists_in_one_entry_are_a_repeated_field_and_a_blank_line_is_dropped() {
    assert_eq!(
        probe_history_after(|e| format!(
            "{e}<ADDRESS.LIST TYPE=\"String\"><ADDRESS>Another</ADDRESS></ADDRESS.LIST>"
        )),
        unreadable(MailingDetailsDefect::EntryRepeatsAField)
    );
    let MailingDetailsHistory::Entries { entries } = probe_history_after(|e| {
        e.replace(
            "<ADDRESS>Test Area</ADDRESS>",
            "<ADDRESS></ADDRESS><ADDRESS>Test Area</ADDRESS><ADDRESS/>",
        )
    }) else {
        panic!("a readable history");
    };
    assert_eq!(entries[0].address_lines, ["1 Test Street", "Test Area"]);
}

#[test]
fn an_unread_child_alone_is_content_not_a_placeholder() {
    // A child this parser does not read, in an element with no date.
    assert_eq!(
        probe_history_after(|e| {
            let start = e.find("<ADDRESS.LIST").unwrap();
            format!("{}<SOMETHINGNEW>x</SOMETHINGNEW>", &e[..start])
        }),
        unreadable(MailingDetailsDefect::EntryWithoutDate)
    );
}
