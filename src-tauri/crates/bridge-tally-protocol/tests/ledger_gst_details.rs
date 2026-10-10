//! `GSTDETAILS.LIST` read from a live `List of Ledgers` capture (fixture
//! `ledger_gst_details_live.utf16le.xml`, provenance in
//! `LEDGER_GST_DETAILS_CAPTURE_PROVENANCE.md`). The negative cases are edits
//! of those real bytes, each asserting the typed variant.
use bridge_tally_protocol::gst_details::{
    GstDetailsDefect, GstDetailsEntry, GstDetailsObservation, GstRate,
};
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_native_ledger_source_records_with_evidence,
    parse_native_party_ledger_master_records_with_evidence, ExpectedTallyTextEncoding,
};

const LIVE: &[u8] = include_bytes!("fixtures/ledger_gst_details_live.utf16le.xml");
const GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";
const ANY: &str = "\u{fffd}#4; Any";

/// The ledgers of the capture, in document order.
const LEDGERS: [&str; 14] = [
    "BRIDGE CGST 2.5%",
    "BRIDGE CGST 9%",
    "BRIDGE Goods 18%",
    "BRIDGE Output CGST XML",
    "BRIDGE Purchase Svc 5%",
    "BRIDGE Round Off",
    "BRIDGE SGST 2.5%",
    "BRIDGE SGST 9%",
    "BRIDGE Supplier RJ",
    "BRIDGE Svc 998313 5%",
    "BRIDGE Svc 998314 5%",
    "BRIDGE Walk-in",
    "Cash",
    "Profit &amp; Loss A/c",
];
/// Ledgers whose list holds a dated entry; the other nine hold an empty placeholder.
const WITH_ENTRY: [&str; 5] = [
    "BRIDGE CGST 2.5%",
    "BRIDGE Goods 18%",
    "BRIDGE Purchase Svc 5%",
    "BRIDGE Svc 998313 5%",
    "BRIDGE Svc 998314 5%",
];

fn live_text() -> String {
    decode_tally_xml_response_bytes_limited(
        LIVE,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        LIVE.len(),
    )
    .expect("the capture decodes")
    .text
}

/// Each ledger's name (as written) and its parsed observation.
fn parse(text: &str) -> anyhow::Result<Vec<(String, GstDetailsObservation)>> {
    Ok(
        parse_native_party_ledger_master_records_with_evidence(text, GUID)?
            .records
            .into_iter()
            .map(|r| (r.record.ledger.name, r.record.fields.gst_details))
            .collect(),
    )
}

fn observation_of(parsed: &[(String, GstDetailsObservation)], name: &str) -> GstDetailsObservation {
    parsed
        .iter()
        .find(|(n, _)| n == name || n.replace('&', "&amp;") == name)
        .unwrap_or_else(|| panic!("ledger {name} parsed"))
        .1
        .clone()
}

fn entries_of(parsed: &[(String, GstDetailsObservation)], name: &str) -> Vec<GstDetailsEntry> {
    match observation_of(parsed, name) {
        GstDetailsObservation::Entries { entries } => entries,
        other => panic!("{name}: {other:?}"),
    }
}

/// Replaces `from` with `to` once, inside the named ledger only.
fn edit(text: &str, ledger: &str, from: &str, to: &str) -> String {
    let start = text
        .find(&format!("<LEDGER NAME=\"{ledger}\""))
        .expect("ledger present");
    let end = start + text[start..].find("</LEDGER>").unwrap();
    let body = &text[start..end];
    assert_eq!(body.matches(from).count(), 1, "{from:?} once in {ledger}");
    format!(
        "{}{}{}",
        &text[..start],
        body.replacen(from, to, 1),
        &text[end..]
    )
}

const GOODS: &str = "BRIDGE Goods 18%";
const RATE_18: &str = "<GSTRATE TYPE=\"Number\"> 18</GSTRATE>";

fn goods_defect(text: &str) -> GstDetailsObservation {
    observation_of(&parse(text).expect("the book still parses"), GOODS)
}

#[test]
fn the_capture_parses_and_each_ledger_has_the_observation_its_bytes_support() {
    let parsed = parse(&live_text()).unwrap();
    assert_eq!(parsed.len(), LEDGERS.len());
    for name in LEDGERS {
        let entries = entries_of(&parsed, name);
        if WITH_ENTRY.contains(&name) {
            assert_eq!(entries.len(), 1, "{name}");
        } else {
            assert!(entries.is_empty(), "{name}: an empty placeholder");
        }
    }
}

#[test]
fn a_ledger_with_a_rate_carries_leading_spaces_and_the_state_marker() {
    let parsed = parse(&live_text()).unwrap();
    let purchase = entries_of(&parsed, "BRIDGE Purchase Svc 5%");
    let entry = &purchase[0];
    assert_eq!(entry.applicable_from, "20260401");
    assert_eq!(entry.taxability.as_deref(), Some("Taxable"));
    assert_eq!(entry.source.as_deref(), Some("Specify Details Here"));
    assert_eq!(
        entry.itc_eligible, None,
        "this entry sends no GSTINELIGIBLEITC"
    );
    assert_eq!(
        entries_of(&parsed, "BRIDGE Svc 998313 5%")[0].itc_eligible,
        Some(true)
    );
    assert_eq!(entry.states.len(), 1);
    let state = &entry.states[0];
    assert_eq!(state.state_name.as_deref(), Some(ANY));
    assert!(
        !state.slab_rates_present,
        "the slab list is whitespace only"
    );
    let heads: Vec<_> = state
        .rates
        .iter()
        .map(|r| r.duty_head.as_deref().unwrap())
        .collect();
    assert_eq!(heads, ["CGST", "SGST/UTGST", "IGST", "Cess", "State Cess"]);
    assert_eq!(
        state.rates[0].rate,
        GstRate::Value {
            raw: " 2.50".to_string(),
            decimal: "2.50".to_string()
        }
    );
    assert_eq!(
        state.rates[2].rate,
        GstRate::Value {
            raw: " 5".to_string(),
            decimal: "5".to_string()
        }
    );
    assert_eq!(
        state.rates[3].rate,
        GstRate::Absent,
        "the Cess row sends no GSTRATE"
    );
    assert_eq!(
        state.rates[3].valuation_type.as_deref(),
        Some("\u{fffd}#4; Not Applicable")
    );
    assert_eq!(state.rates[4].rate, GstRate::Absent);
}

#[test]
fn the_first_ledger_sends_no_rate_element_on_any_row() {
    let parsed = parse(&live_text()).unwrap();
    let entries = entries_of(&parsed, "BRIDGE CGST 2.5%");
    let rates = &entries[0].states[0].rates;
    assert_eq!(rates.len(), 5);
    assert!(rates.iter().all(|r| r.rate == GstRate::Absent));
    assert_eq!(entries[0].taxability, None, "no TAXABILITY element");
}

#[test]
fn a_rate_edited_to_empty_or_zero_stays_empty_or_zero_not_absent() {
    let text = live_text();
    let rate = |replacement: &str| {
        let parsed = parse(&edit(&text, GOODS, RATE_18, replacement)).unwrap();
        entries_of(&parsed, GOODS)[0].states[0].rates[2]
            .rate
            .clone()
    };
    assert_eq!(rate("<GSTRATE TYPE=\"Number\"></GSTRATE>"), GstRate::Empty);
    assert_eq!(rate("<GSTRATE TYPE=\"Number\"/>"), GstRate::Empty);
    assert_eq!(rate("<GSTRATE TYPE=\"Number\">0</GSTRATE>"), GstRate::Zero);
    assert_eq!(
        rate("<GSTRATE TYPE=\"Number\"> 0.00</GSTRATE>"),
        GstRate::Zero
    );
    assert_eq!(
        rate("<GSTRATE TYPE=\"Number\"> 18</GSTRATE>"),
        GstRate::Value {
            raw: " 18".to_string(),
            decimal: "18".to_string()
        }
    );
}

#[test]
fn a_rate_that_is_not_a_plain_decimal_fails_that_ledger_only() {
    let text = live_text();
    for bad in ["-1", "1e2", "2,50"] {
        let edited = edit(
            &text,
            GOODS,
            RATE_18,
            &format!("<GSTRATE TYPE=\"Number\">{bad}</GSTRATE>"),
        );
        let parsed = parse(&edited).expect("one ledger's defect never fails the book");
        assert_eq!(
            observation_of(&parsed, GOODS),
            GstDetailsObservation::Unreadable {
                defect: GstDetailsDefect::RateNotDecimal
            },
            "{bad}"
        );
        assert_eq!(parsed.len(), LEDGERS.len());
        for name in WITH_ENTRY.iter().filter(|n| **n != GOODS) {
            assert_eq!(entries_of(&parsed, name).len(), 1, "{name} unaffected");
        }
    }
}

#[test]
fn a_date_that_is_missing_impossible_or_repeated_is_a_typed_defect() {
    let text = live_text();
    let date = "<APPLICABLEFROM TYPE=\"Date\">20260401</APPLICABLEFROM>";
    assert_eq!(
        goods_defect(&edit(&text, GOODS, date, "")),
        GstDetailsObservation::Unreadable {
            defect: GstDetailsDefect::EntryWithoutDate
        }
    );
    assert_eq!(
        goods_defect(&edit(
            &text,
            GOODS,
            date,
            "<APPLICABLEFROM TYPE=\"Date\">20261301</APPLICABLEFROM>"
        )),
        GstDetailsObservation::Unreadable {
            defect: GstDetailsDefect::DateInvalid
        }
    );
    assert_eq!(
        goods_defect(&edit(&text, GOODS, date, &format!("{date}{date}"))),
        GstDetailsObservation::Unreadable {
            defect: GstDetailsDefect::EntryRepeatsAField
        }
    );
    let rate_head = "<GSTRATEDUTYHEAD TYPE=\"String\">IGST</GSTRATEDUTYHEAD>";
    assert_eq!(
        goods_defect(&edit(
            &text,
            GOODS,
            rate_head,
            &format!("{rate_head}{rate_head}")
        )),
        GstDetailsObservation::Unreadable {
            defect: GstDetailsDefect::EntryRepeatsAField
        },
        "a repeat inside a rate row"
    );
}

#[test]
fn input_tax_credit_other_than_yes_or_no_is_a_defect() {
    let text = live_text();
    let itc = "<GSTINELIGIBLEITC TYPE=\"Logical\">Yes</GSTINELIGIBLEITC>";
    let edited = edit(
        &text,
        GOODS,
        itc,
        "<GSTINELIGIBLEITC TYPE=\"Logical\">Maybe</GSTINELIGIBLEITC>",
    );
    assert_eq!(
        goods_defect(&edited),
        GstDetailsObservation::Unreadable {
            defect: GstDetailsDefect::ItcNotYesNo
        }
    );
    let no = edit(
        &text,
        GOODS,
        itc,
        "<GSTINELIGIBLEITC TYPE=\"Logical\">No</GSTINELIGIBLEITC>",
    );
    assert_eq!(
        entries_of(&parse(&no).unwrap(), GOODS)[0].itc_eligible,
        Some(false)
    );
}

#[test]
fn two_entries_are_sorted_and_two_on_one_date_must_agree() {
    let text = live_text();
    // The real entry, then a second one dated earlier, copied from the first.
    let start = text.find("<LEDGER NAME=\"BRIDGE Goods 18%\"").unwrap();
    let list_start = start + text[start..].find("<GSTDETAILS.LIST>").unwrap();
    let list_end = list_start
        + text[list_start..].find("</GSTDETAILS.LIST>").unwrap()
        + "</GSTDETAILS.LIST>".len();
    let real = &text[list_start..list_end];
    let earlier = real.replace("20260401", "20250401");
    let with = |extra: &str| format!("{}{real}{extra}{}", &text[..list_start], &text[list_end..]);
    let parsed = parse(&with(&earlier)).unwrap();
    let dates: Vec<_> = entries_of(&parsed, GOODS)
        .iter()
        .map(|e| e.applicable_from.clone())
        .collect();
    assert_eq!(
        dates,
        ["20250401", "20260401"],
        "sorted, not document order"
    );

    let identical = parse(&with(real)).unwrap();
    assert_eq!(entries_of(&identical, GOODS).len(), 1);

    let conflicting = real.replace("> 18<", "> 12<");
    assert_ne!(conflicting, real);
    assert_eq!(
        observation_of(&parse(&with(&conflicting)).unwrap(), GOODS),
        GstDetailsObservation::Unreadable {
            defect: GstDetailsDefect::ConflictingEntriesOnOneDate
        }
    );
}

#[test]
fn a_slab_list_with_content_is_reported_present() {
    let text = live_text();
    let edited = edit(
        &text,
        GOODS,
        "<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST>",
        "<GSTSLABRATES.LIST><SLABRATE>1</SLABRATE></GSTSLABRATES.LIST>",
    );
    let parsed = parse(&edited).unwrap();
    assert!(entries_of(&parsed, GOODS)[0].states[0].slab_rates_present);
    let parsed = parse(&live_text()).unwrap();
    assert!(!entries_of(&parsed, GOODS)[0].states[0].slab_rates_present);
}

#[test]
fn a_ledger_without_the_element_is_not_observed() {
    let text = live_text();
    let edited = edit(
        &text,
        "Cash",
        "<GSTDETAILS.LIST>     </GSTDETAILS.LIST>",
        "",
    );
    let parsed = parse(&edited).unwrap();
    assert_eq!(
        observation_of(&parsed, "Cash"),
        GstDetailsObservation::NotObserved
    );
    assert_eq!(entries_of(&parsed, "BRIDGE Walk-in").len(), 0);
}

#[test]
fn the_ordinary_ledger_parser_reads_the_same_response_without_error() {
    // This asserts only that the ordinary source-record parser accepts the
    // response; it does not inspect what that parser keeps.
    let text = live_text();
    let ordinary = parse_native_ledger_source_records_with_evidence(&text, GUID);
    assert!(ordinary.is_ok(), "{:?}", ordinary.err());
}

#[test]
fn a_state_name_is_kept_exactly_as_sent_spaces_and_all() {
    let text = live_text();
    let edited = edit(
        &text,
        GOODS,
        "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>",
        "<STATENAME TYPE=\"String\">  &#4; Any  </STATENAME>",
    );
    let parsed = parse(&edited).unwrap();
    assert_eq!(
        entries_of(&parsed, GOODS)[0].states[0]
            .state_name
            .as_deref(),
        Some("  \u{fffd}#4; Any  ")
    );
}

const SGST_ROW_HEAD: &str = "<GSTRATEDUTYHEAD TYPE=\"String\">SGST/UTGST</GSTRATEDUTYHEAD>";

fn unreadable(defect: GstDetailsDefect) -> GstDetailsObservation {
    GstDetailsObservation::Unreadable { defect }
}

#[test]
fn the_unmodified_capture_skips_no_content_on_any_entry() {
    let parsed = parse(&live_text()).unwrap();
    for name in WITH_ENTRY {
        for entry in entries_of(&parsed, name) {
            assert!(!entry.other_content_skipped, "{name}");
        }
    }
}

#[test]
fn a_repeated_rate_inside_one_rate_row_repeats_a_field() {
    let text = live_text();
    assert_eq!(
        goods_defect(&edit(&text, GOODS, RATE_18, &format!("{RATE_18}{RATE_18}"))),
        unreadable(GstDetailsDefect::EntryRepeatsAField)
    );
}

#[test]
fn an_entry_whose_only_child_is_unknown_is_unrecognised_content_not_a_placeholder() {
    let text = live_text();
    let empty = "<GSTDETAILS.LIST>     </GSTDETAILS.LIST>";
    for only_child in [
        "<FUTUREELEMENT>x</FUTUREELEMENT>",
        "<FUTUREELEMENT/>",
        // A misplaced rate directly under the entry is not recognised there.
        "<GSTRATE TYPE=\"Number\"> 5</GSTRATE>",
    ] {
        let edited = edit(
            &text,
            "Cash",
            empty,
            &format!("<GSTDETAILS.LIST>{only_child}</GSTDETAILS.LIST>"),
        );
        assert_eq!(
            observation_of(&parse(&edited).unwrap(), "Cash"),
            unreadable(GstDetailsDefect::UnrecognisedContent),
            "{only_child}"
        );
    }
}

#[test]
fn an_unknown_child_beside_the_recognised_ones_is_kept_and_flagged() {
    let text = live_text();
    let date = "<APPLICABLEFROM TYPE=\"Date\">20260401</APPLICABLEFROM>";
    let state_name = "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>";
    let extras = [
        // at the entry
        (date, format!("{date}<HSNCODE>9983</HSNCODE>")),
        // in the state row
        (state_name, format!("{state_name}<FUTURE/>")),
        // in a rate row
        (RATE_18, format!("{RATE_18}<RATEFUTURE>1</RATEFUTURE>")),
    ];
    for (from, to) in extras {
        let parsed = parse(&edit(&text, GOODS, from, &to)).unwrap();
        let entries = entries_of(&parsed, GOODS);
        assert_eq!(entries.len(), 1, "{to}");
        assert!(entries[0].other_content_skipped, "{to}");
    }
    let plain = entries_of(&parse(&text).unwrap(), GOODS);
    assert!(!plain[0].other_content_skipped);
}

#[test]
fn two_rate_rows_with_one_duty_head_or_two_state_rows_with_one_name_are_a_duplicate_row() {
    let text = live_text();
    // Rename the SGST row's head to CGST: two CGST rows in one state row.
    let edited = edit(
        &text,
        GOODS,
        SGST_ROW_HEAD,
        "<GSTRATEDUTYHEAD TYPE=\"String\">CGST</GSTRATEDUTYHEAD>",
    );
    assert_eq!(
        goods_defect(&edited),
        unreadable(GstDetailsDefect::DuplicateRow)
    );
    // Repeat the whole state row.
    let start = text.find("<LEDGER NAME=\"BRIDGE Goods 18%\"").unwrap();
    let from = start + text[start..].find("<STATEWISEDETAILS.LIST>").unwrap();
    let to = from
        + text[from..].find("</STATEWISEDETAILS.LIST>").unwrap()
        + "</STATEWISEDETAILS.LIST>".len();
    let row = &text[from..to];
    let doubled = format!("{}{row}{row}{}", &text[..from], &text[to..]);
    assert_eq!(
        goods_defect(&doubled),
        unreadable(GstDetailsDefect::DuplicateRow)
    );
    // A second state row with a different name is fine.
    let other = row.replace("&#4; Any", "Rajasthan");
    assert_ne!(other, row);
    let two = format!("{}{row}{other}{}", &text[..from], &text[to..]);
    assert_eq!(entries_of(&parse(&two).unwrap(), GOODS)[0].states.len(), 2);
}
