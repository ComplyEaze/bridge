use bridge_tally_core::TallyDate;
use bridge_tally_protocol::{
    native_outstandings::{
        render_party_ledger_master_request,
        render_party_ledger_master_request_with_mailing_details, NativeLedgerExportPeriod,
    },
    outstandings_shared::DateBoundaryProfile,
    parse_native_party_ledger_master_records_with_evidence, GstDutyHead, GstDutyHeadObservation,
    PartyLedgerMasterFieldObservation,
};

use super::*;

const COMPANY_GUID: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";

/// A minimized parser fixture built from the task's measured field values
/// and record shape. It is not a live-response evidence capture.
fn measured_ledger_master_response() -> String {
    let ledger = |name: &str, master_id: u8, tax_type: &str, duty_head: Option<&str>| {
        let duty_head = match duty_head {
            Some("") => "<GSTDUTYHEAD/>".to_string(),
            Some(value) => format!("<GSTDUTYHEAD>{value}</GSTDUTYHEAD>"),
            None => String::new(),
        };
        format!(
            "<LEDGER NAME=\"{name}\" RESERVEDNAME=\"\"><GUID>{COMPANY_GUID}-000000{master_id:02x}</GUID><BRIDGECOMPANYGUID>{COMPANY_GUID}</BRIDGECOMPANYGUID><MASTERID>{master_id}</MASTERID><ALTERID>{master_id}</ALTERID><PARENT>Duties &amp; Taxes</PARENT><TAXTYPE>{tax_type}</TAXTYPE>{duty_head}<OPENINGBALANCE>0.00</OPENINGBALANCE><LANGUAGENAME.LIST><NAME.LIST><NAME>Localized {name}</NAME></NAME.LIST></LANGUAGENAME.LIST></LEDGER>"
        )
    };
    format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><CMPINFO><LEDGER>107</LEDGER></CMPINFO><COLLECTION>{}{}{}{}</COLLECTION></DATA></BODY></ENVELOPE>",
        ledger("Input CGST", 1, "GST", Some("CGST")),
        ledger("Input SGST", 2, "GST", Some("SGST")),
        ledger("GST Head Absent", 3, "GST", Some("")),
        ledger("Non-tax ledger", 4, "Others", Some("")),
    )
}

fn captured_partial_alter_ledger(name: &str) -> String {
    let capture = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/native/master_fields_lab_partial_alter_after.response.xml"
    );
    let start_tag = format!(r#"<LEDGER NAME="{name}" RESERVEDNAME="">"#);
    let start = capture
        .find(&start_tag)
        .expect("captured partial-alter response contains the expected ledger");
    let end = start
        + capture[start..]
            .find("</LEDGER>")
            .expect("captured ledger closes")
        + "</LEDGER>".len();
    capture[start..end].to_string()
}

fn native_party_master_collection(fields: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><LEDGER NAME=\"Captured partial-alter ledger\" RESERVEDNAME=\"\"><GUID>{COMPANY_GUID}-000000ce</GUID><BRIDGECOMPANYGUID>{COMPANY_GUID}</BRIDGECOMPANYGUID><MASTERID>206</MASTERID><ALTERID>208</ALTERID><PARENT>Sundry Debtors</PARENT>{fields}<OPENINGBALANCE>0.00</OPENINGBALANCE></LEDGER></COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

#[test]
fn party_ledger_master_request_fetches_tax_type_and_gst_duty_head() {
    let period = NativeLedgerExportPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20260401").unwrap(),
        TallyDate::parse("20260910").unwrap(),
    )
    .unwrap();

    let request = render_party_ledger_master_request("BRIDGE GST RECON LAB", &period);
    assert!(request.contains("TAXTYPE, GSTDUTYHEAD"));
    assert!(request.contains(", LEDGSTREGDETAILS.LIST</FETCH>"));
    // The read that also names the dated mailing details (`include_mailing_details`, the workbook).
    let with_mailing =
        render_party_ledger_master_request_with_mailing_details("BRIDGE GST RECON LAB", &period);
    assert!(with_mailing.contains(", LEDGSTREGDETAILS.LIST, LEDMAILINGDETAILS.LIST</FETCH>"));
}

#[test]
fn compliance_ledger_duty_heads_preserve_raw_values_and_name_attributes() {
    let parsed = parse_native_party_ledger_master_records_with_evidence(
        &measured_ledger_master_response(),
        COMPANY_GUID,
    )
    .expect("measured duty-head response shape parses");

    assert_eq!(parsed.records.len(), 4, "CMPINFO ledger count is not a row");
    let cgst = &parsed.records[0].record;
    assert_eq!(
        cgst.ledger.name, "Input CGST",
        "identity is the NAME attribute"
    );
    assert_eq!(
        cgst.fields.tax_type,
        PartyLedgerMasterFieldObservation::Returned("GST".to_string())
    );
    assert_eq!(
        cgst.fields.gst_duty_head,
        GstDutyHeadObservation::Recognized {
            raw: "CGST".to_string(),
            head: GstDutyHead::Cgst,
        }
    );
    assert_eq!(
        parsed.records[1].record.fields.gst_duty_head,
        GstDutyHeadObservation::Unrecognized {
            raw: "SGST".to_string(),
        },
        "unmeasured SGST spelling must not be normalized into State Tax"
    );
    assert_eq!(
        parsed.records[2].record.fields.gst_duty_head,
        GstDutyHeadObservation::Absent,
        "a GST ledger with no returned head is not a default tax head"
    );
    assert_eq!(
        parsed.records[3].record.fields.gst_duty_head,
        GstDutyHeadObservation::NotTaxLedger {
            tax_type: "Others".to_string(),
        },
        "a non-GST ledger remains distinct from an absent GST duty head"
    );

    let compliance = serde_json::to_value(&cgst.fields).unwrap();
    assert_eq!(compliance["tax_type"], "GST");
    assert_eq!(compliance["gst_duty_head"]["observation"], "recognized");
    assert_eq!(compliance["gst_duty_head"]["raw"], "CGST");
    assert_eq!(compliance["gst_duty_head"]["head"], "cgst");
}

#[test]
fn captured_empty_duty_head_uses_tax_type_classification() {
    let ledger = captured_partial_alter_ledger("BRIDGE MFLAB PARTIAL ALTER PROBE");
    let fields = ["<TAXTYPE>Others</TAXTYPE>", "<GSTDUTYHEAD/>"]
        .into_iter()
        .map(|field| {
            let start = ledger
                .find(field)
                .expect("the real capture contains the expected duty-head field shape");
            &ledger[start..start + field.len()]
        })
        .collect::<String>();

    let parsed = parse_native_party_ledger_master_records_with_evidence(
        &native_party_master_collection(&fields),
        COMPANY_GUID,
    )
    .expect("the captured duty-head fields parse in the collection profile");
    assert_eq!(parsed.records.len(), 1);
    assert_eq!(
        parsed.records[0].record.fields.gst_duty_head,
        GstDutyHeadObservation::NotTaxLedger {
            tax_type: "Others".to_string(),
        }
    );
}

fn captured_live_ledger_masters() -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-masters-duty-heads.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[test]
fn nested_markup_in_a_classification_scalar_is_refused() {
    // <GSTDUTYHEAD><VALUE>CGST</VALUE></GSTDUTYHEAD> flattened to "CGST" and was
    // released as a recognised duty head. A field that only ever carries a scalar,
    // and whose value drives a classification, must fail at the boundary on an
    // unexpected shape rather than become compliance data.
    for (field, nested) in [
        (
            "GSTDUTYHEAD",
            "<GSTDUTYHEAD><VALUE>CGST</VALUE></GSTDUTYHEAD>",
        ),
        ("TAXTYPE", "<TAXTYPE><VALUE>GST</VALUE></TAXTYPE>"),
    ] {
        let response = format!(
            "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><CMPINFO><LEDGER>1</LEDGER></CMPINFO><COLLECTION><LEDGER NAME=\"Nested\" RESERVEDNAME=\"\"><GUID>{COMPANY_GUID}-00000044</GUID><BRIDGECOMPANYGUID>{COMPANY_GUID}</BRIDGECOMPANYGUID><MASTERID>68</MASTERID><ALTERID>68</ALTERID><PARENT>Duties &amp; Taxes</PARENT>{nested}<OPENINGBALANCE>0.00</OPENINGBALANCE><LANGUAGENAME.LIST><NAME.LIST><NAME>Localized Nested</NAME></NAME.LIST></LANGUAGENAME.LIST></LEDGER></COLLECTION></DATA></BODY></ENVELOPE>"
        );
        assert!(
            parse_native_party_ledger_master_records_with_evidence(&response, COMPANY_GUID)
                .is_err(),
            "nested markup in {field} must be refused, not flattened"
        );
    }
}

#[test]
fn a_duty_head_on_a_non_gst_ledger_is_contradictory_not_recognised() {
    // <TAXTYPE>Others</TAXTYPE><GSTDUTYHEAD>CGST</GSTDUTYHEAD> is a response
    // contradicting itself. Classifying head-first recognised it and never
    // consulted TAXTYPE, releasing the contradiction as valid compliance data.
    // Neither field is now asserted, and both raw values are kept so a
    // reviewer can see what Tally actually returned.
    let response = format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><CMPINFO><LEDGER>1</LEDGER></CMPINFO><COLLECTION><LEDGER NAME=\"Contradictory\" RESERVEDNAME=\"\"><GUID>{COMPANY_GUID}-00000042</GUID><BRIDGECOMPANYGUID>{COMPANY_GUID}</BRIDGECOMPANYGUID><MASTERID>66</MASTERID><ALTERID>66</ALTERID><PARENT>Duties &amp; Taxes</PARENT><TAXTYPE>Others</TAXTYPE><GSTDUTYHEAD>CGST</GSTDUTYHEAD><OPENINGBALANCE>0.00</OPENINGBALANCE><LANGUAGENAME.LIST><NAME.LIST><NAME>Localized Contradictory</NAME></NAME.LIST></LANGUAGENAME.LIST></LEDGER></COLLECTION></DATA></BODY></ENVELOPE>"
    );
    let parsed = parse_native_party_ledger_master_records_with_evidence(&response, COMPANY_GUID)
        .expect("a contradictory response still parses; it is classified, not refused");
    assert_eq!(
        parsed.records[0].record.fields.gst_duty_head,
        GstDutyHeadObservation::Contradictory {
            tax_type: "Others".to_string(),
            raw: "CGST".to_string(),
        }
    );
}

#[test]
fn an_unobserved_tax_type_does_not_contradict_a_duty_head() {
    // Scoped deliberately. An absent or empty TAXTYPE is not evidence that the
    // ledger is non-GST, so it must not block recognition -- that would refuse
    // real GST ledgers on any Tally version that omits the field. Only an
    // OBSERVED non-GST value contradicts.
    let response = format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><CMPINFO><LEDGER>1</LEDGER></CMPINFO><COLLECTION><LEDGER NAME=\"NoTaxType\" RESERVEDNAME=\"\"><GUID>{COMPANY_GUID}-00000043</GUID><BRIDGECOMPANYGUID>{COMPANY_GUID}</BRIDGECOMPANYGUID><MASTERID>67</MASTERID><ALTERID>67</ALTERID><PARENT>Duties &amp; Taxes</PARENT><GSTDUTYHEAD>IGST</GSTDUTYHEAD><OPENINGBALANCE>0.00</OPENINGBALANCE><LANGUAGENAME.LIST><NAME.LIST><NAME>Localized NoTaxType</NAME></NAME.LIST></LANGUAGENAME.LIST></LEDGER></COLLECTION></DATA></BODY></ENVELOPE>"
    );
    let parsed = parse_native_party_ledger_master_records_with_evidence(&response, COMPANY_GUID)
        .expect("parses");
    assert_eq!(
        parsed.records[0].record.fields.gst_duty_head,
        GstDutyHeadObservation::Recognized {
            raw: "IGST".to_string(),
            head: GstDutyHead::Igst,
        }
    );
}

#[test]
fn live_capture_backs_the_recognised_duty_head_vocabulary() {
    // Captured from a live TallyPrime 7.1 Silver response to
    // render_party_ledger_master_request, so the vocabulary is checked against
    // bytes Tally actually sent rather than against a fixture written from the
    // same understanding as the parser. A hand-authored response can encode a
    // wrong vocabulary in both places and agree with itself.
    //
    // All FIVE recognised spellings are now covered. UT Tax and Cess were
    // absent from this book, so two ledgers carrying those heads were created
    // and the capture retaken -- they are no longer accepted on the strength of
    // a hand-written table alone.
    let parsed = parse_native_party_ledger_master_records_with_evidence(
        &captured_live_ledger_masters(),
        "ae1490be-52c5-4544-9ffc-4b7da85f9797",
    )
    .expect("live ledger-master capture parses");

    let mut recognised: Vec<(String, GstDutyHead)> = parsed
        .records
        .iter()
        .filter_map(|row| match &row.record.fields.gst_duty_head {
            GstDutyHeadObservation::Recognized { raw, head } => Some((raw.clone(), *head)),
            _ => None,
        })
        .collect();
    recognised.sort_by(|left, right| left.0.cmp(&right.0));
    recognised.dedup();

    assert_eq!(
        recognised,
        vec![
            ("CGST".to_string(), GstDutyHead::Cgst),
            ("Cess".to_string(), GstDutyHead::Cess),
            ("IGST".to_string(), GstDutyHead::Igst),
            ("State Tax".to_string(), GstDutyHead::StateTax),
            ("UT Tax".to_string(), GstDutyHead::UtTax),
        ],
        "EVERY recognised spelling must be backed by bytes Tally actually sent"
    );

    // The state head really is spelled `State Tax` on the wire, not `SGST`.
    // That irregularity is the reason this vocabulary is enumerated at all.
    let capture = captured_live_ledger_masters();
    assert!(capture.contains(">State Tax</GSTDUTYHEAD>"));
    assert!(!capture.contains(">SGST</GSTDUTYHEAD>"));

    // This instance OMITS the element for non-GST ledgers rather than emitting
    // it empty, so the captured empty-element shape is exercised separately by
    // captured_empty_duty_head_uses_tax_type_classification.
    assert!(!capture.contains("<GSTDUTYHEAD/>"));
    assert!(
        parsed.records.iter().any(|row| matches!(
            row.record.fields.gst_duty_head,
            GstDutyHeadObservation::NotTaxLedger { .. }
        )),
        "the same capture must also carry ordinary non-tax ledgers"
    );
}

fn captured_live_ledger_masters_with_sgst_utgst() -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-masters-sgst-utgst.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[test]
fn a_live_capture_backs_sgst_utgst_as_a_recognised_head() {
    // Bytes TallyPrime 7.1 Silver sent for ledger_masters fields=compliance (see the
    // fixture's JSON sidecar for the binary, relay and times): ledgers created with the
    // literal head `SGST/UTGST` come back with exactly that string, and TAXTYPE GST.
    // Before this head was recognised the same bytes classified as `unrecognized`,
    // and a caller could not tell those ledgers from a misspelt head.
    let period = NativeLedgerExportPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20250401").unwrap(),
        TallyDate::parse("20250804").unwrap(),
    )
    .unwrap();
    assert_eq!(
        sha256_hex(render_party_ledger_master_request("BRIDGE GST RECON LAB", &period).as_bytes()),
        "f700339d10d4bdc845f7ab4af482e407fc03d9901a7eaebdf34feb86f8d7ea68",
        "the fixture answers exactly the request ledger_masters sends"
    );
    let parsed = parse_native_party_ledger_master_records_with_evidence(
        &captured_live_ledger_masters_with_sgst_utgst(),
        COMPANY_GUID,
    )
    .expect("the live SGST/UTGST capture parses");
    assert_eq!(parsed.records.len(), 36);

    let mut heads: Vec<(String, GstDutyHead)> = parsed
        .records
        .iter()
        .filter_map(|row| match &row.record.fields.gst_duty_head {
            GstDutyHeadObservation::Recognized { raw, head } => Some((raw.clone(), *head)),
            _ => None,
        })
        .collect();
    heads.sort_by(|left, right| left.0.cmp(&right.0));
    heads.dedup();
    assert_eq!(
        heads,
        vec![
            ("CGST".to_string(), GstDutyHead::Cgst),
            ("Cess".to_string(), GstDutyHead::Cess),
            ("IGST".to_string(), GstDutyHead::Igst),
            ("SGST/UTGST".to_string(), GstDutyHead::SgstUtgst),
            ("State Tax".to_string(), GstDutyHead::StateTax),
            ("UT Tax".to_string(), GstDutyHead::UtTax),
        ],
        "every recognised spelling is backed by bytes Tally sent"
    );

    let sgst_utgst_rows: Vec<_> = parsed
        .records
        .iter()
        .filter(|row| {
            matches!(
                &row.record.fields.gst_duty_head,
                GstDutyHeadObservation::Recognized {
                    head: GstDutyHead::SgstUtgst,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(
        sgst_utgst_rows.len(),
        2,
        "the probe ledger and the batch ledger"
    );
    for row in sgst_utgst_rows {
        assert_eq!(
            row.record.ledger.parent.returned_text(),
            Some("Duties & Taxes")
        );
        assert_eq!(row.record.fields.tax_type.returned_text(), Some("GST"));
    }
    assert!(captured_live_ledger_masters_with_sgst_utgst().contains(">SGST/UTGST</GSTDUTYHEAD>"));

    // The serialised wire shape a caller of ledger_masters sees.
    let wire = serde_json::to_value(&GstDutyHeadObservation::Recognized {
        raw: "SGST/UTGST".to_string(),
        head: GstDutyHead::SgstUtgst,
    })
    .unwrap();
    assert_eq!(
        wire,
        json!({"observation": "recognized", "raw": "SGST/UTGST", "head": "sgst_utgst"})
    );
}

#[test]
fn sgst_utgst_on_a_non_gst_ledger_is_contradictory_not_recognised() {
    assert_eq!(
        GstDutyHeadObservation::from_observations(
            &PartyLedgerMasterFieldObservation::Returned("Others".to_string()),
            &PartyLedgerMasterFieldObservation::Returned("SGST/UTGST".to_string()),
        ),
        GstDutyHeadObservation::Contradictory {
            tax_type: "Others".to_string(),
            raw: "SGST/UTGST".to_string(),
        }
    );
}

#[test]
fn a_gstin_held_only_in_the_dated_registration_history_is_reported_in_force() {
    // bridge#624, over a live TallyPrime 7.1 Silver capture of the request this
    // tool sends: party A's GSTIN is only in its second dated entry, and the
    // flat PARTYGSTIN is empty. Before the fix A read as `party_gstin: null`.
    let period = NativeLedgerExportPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20250401").unwrap(),
        TallyDate::parse("20250401").unwrap(),
    )
    .unwrap();
    assert_eq!(
        sha256_hex(render_party_ledger_master_request("BRIDGE READS LAB", &period).as_bytes()),
        "c2f9f8e9fefab44077b222402254a2be3dfd9a9690d4e577efbae99d531c819c",
        "the fixture answers exactly the request ledger_masters sends"
    );
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-masters-gst-registrations.utf16le.xml"
    );
    let capture = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let parsed = parse_native_party_ledger_master_records_with_evidence(
        &capture,
        "de2e15f2-6d42-4715-b6e7-b7a95a68abe8",
    )
    .expect("the live registration-history capture parses");
    let answer = |name: &str, as_of: &str| {
        let row = parsed
            .records
            .iter()
            .find(|row| row.record.ledger.name == name)
            .unwrap_or_else(|| panic!("{name} is in the capture"));
        let gstin = party_gstin_on(
            row.record.ledger.party_gstin.returned_text(),
            &row.record.fields.gst_registrations,
            as_of,
        );
        assert!(
            !gstin.sources_disagree,
            "{name}: the capture's sources agree"
        );
        (
            gstin.gstin,
            gstin.status,
            gstin.registration_type,
            gstin.flat,
        )
    };
    let text = |value: &str| Some(value.to_string());
    assert_eq!(
        answer("G1 Party A Later GSTIN", "20250930"),
        (text("27ZZZZZ0000Z1Z5"), "in_force", text("Regular"), None)
    );
    assert_eq!(
        answer("G1 Party A Later GSTIN", "20250630"),
        (
            None,
            "no_gstin_in_force",
            text("Unregistered/Consumer"),
            None
        ),
        "the first entry is dated but carries no GSTIN"
    );
    assert_eq!(
        answer("G1 Party B Flat GSTIN", "20250930"),
        (
            text("29ZZZZZ0000Z1Z5"),
            "flat_field",
            None,
            text("29ZZZZZ0000Z1Z5")
        ),
        "Tally returns an empty placeholder list beside the flat field"
    );
    assert_eq!(
        answer("G1 Party C Unregistered", "20250930"),
        (None, "not_reported", None, None)
    );
    assert_eq!(
        answer("G1 Party D Two GSTINs", "20250930"),
        (text("27ZZZZZ0000Z1Z5"), "in_force", text("Regular"), None)
    );
    assert_eq!(
        answer("G1 Party D Two GSTINs", "20251001"),
        (text("29ZZZZZ0000Z1Z5"), "in_force", text("Regular"), None),
        "the later registration applies from its own date"
    );
}

/// bridge#1520, over populated values: the live registration-history capture,
/// whose rows hold dated registrations, serialised through the helper equals
/// the golden that the expression the row used before `gst_details` existed
/// produced on the same parse (see the provenance note). The golden holds the
/// existing `mark_party_field` behaviour too: a key whose value is not a
/// string is dropped, not re-inserted.
#[test]
fn populated_compliance_objects_are_byte_for_byte_the_ones_built_before_gst_details() {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-masters-gst-registrations.utf16le.xml"
    );
    let capture = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let parsed = parse_native_party_ledger_master_records_with_evidence(
        &capture,
        "de2e15f2-6d42-4715-b6e7-b7a95a68abe8",
    )
    .expect("the live registration-history capture parses");
    let compliance = parsed
        .records
        .into_iter()
        .map(|row| compliance_json(row.record.fields, false, false))
        .collect::<Vec<_>>();
    assert!(!compliance.is_empty());
    assert!(compliance.iter().all(|value| value.is_object()));
    let actual = serde_json::to_string(&compliance).unwrap();
    let golden = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/ledger_masters_compliance_objects_registrations_before_gst_details.json"
    );
    let first_difference = actual
        .bytes()
        .zip(golden.bytes())
        .position(|(left, right)| left != right)
        .unwrap_or(actual.len().min(golden.len()));
    assert!(
        actual == golden,
        "{} objects differ from the golden ({} bytes, golden {}) from byte {first_difference}",
        compliance.len(),
        actual.len(),
        golden.len()
    );
}

#[test]
fn both_gstin_sources_are_reported_and_a_difference_is_flagged_not_resolved() {
    use bridge_tally_protocol::gst_registration::GstRegistrationEntry;
    let entry = |date: &str, gstin: Option<&str>, kind: &str| GstRegistrationEntry {
        applicable_from: date.to_string(),
        gstin: gstin.map(str::to_string),
        registration_type: Some(kind.to_string()),
    };
    let history = |entries| GstRegistrationHistory::Entries { entries };
    const FLAT: &str = "27ZZZZZ0000Z1Z5";
    const DATED: &str = "29ZZZZZ0000Z1Z5";
    let text = |value: &str| Some(value.to_string());

    // The flat field names a GSTIN; the history says none on that date.
    let unregistered = history(vec![entry("20170701", None, "Unregistered/Consumer")]);
    let got = party_gstin_on(Some(FLAT), &unregistered, "20260331");
    assert_eq!((got.gstin, got.status), (None, "no_gstin_in_force"));
    assert_eq!(got.flat, text(FLAT), "the flat GSTIN is still reported");
    assert!(got.sources_disagree);

    // The two sources name different GSTINs.
    let other = history(vec![entry("20170701", Some(DATED), "Regular")]);
    let got = party_gstin_on(Some(FLAT), &other, "20260331");
    assert_eq!((got.gstin, got.status), (text(DATED), "in_force"));
    assert_eq!(got.flat, text(FLAT));
    assert!(got.sources_disagree);

    // Agreement is not a disagreement, and an absent flat field is not one.
    assert!(!party_gstin_on(Some(DATED), &other, "20260331").sources_disagree);
    assert!(!party_gstin_on(None, &other, "20260331").sources_disagree);

    // An explicitly empty flat field (`<PARTYGSTIN/>`, seen live) names no
    // GSTIN: it is reported as read but neither disagrees nor is used.
    let got = party_gstin_on(Some(""), &other, "20260331");
    assert_eq!((got.status, got.flat.as_deref()), ("in_force", Some("")));
    assert!(!got.sources_disagree);
    let got = party_gstin_on(Some(""), &unregistered, "20260331");
    assert!(!got.sources_disagree);
    let got = party_gstin_on(Some(""), &history(vec![]), "20260331");
    assert_eq!((got.gstin, got.status), (None, "not_reported"));

    // A history that starts after the date names nothing yet.
    let future = history(vec![entry("20270401", Some(DATED), "Regular")]);
    let got = party_gstin_on(None, &future, "20260331");
    assert_eq!(
        (got.gstin, got.status, got.registration_type),
        (None, "no_gstin_in_force", None)
    );

    // Every source is reported under its own key.
    let fields = party_gstin_fields(party_gstin_on(Some(FLAT), &other, "20260331"), "20260331");
    assert_eq!(
        Value::Object(fields),
        json!({
            "party_gstin": DATED,
            "party_gstin_status": "in_force",
            "party_gstin_registration_type": "Regular",
            "party_gstin_as_of": "20260331",
            "party_gstin_flat": FLAT,
            "gstin_sources_disagree": true,
        })
    );

    // Registered with no GSTIN recorded is not read as unregistered.
    let regular = history(vec![entry("20170701", None, "Regular")]);
    let got = party_gstin_on(None, &regular, "20260331");
    assert_eq!(
        (got.status, got.registration_type),
        ("no_gstin_in_force", text("Regular"))
    );

    // An unreadable history never falls back to the flat field.
    let unreadable = GstRegistrationHistory::Unreadable {
        defect: bridge_tally_protocol::gst_registration::GstRegistrationDefect::DateInvalid,
    };
    let got = party_gstin_on(Some(FLAT), &unreadable, "20260331");
    assert_eq!((got.gstin, got.status), (None, "history_unreadable"));
    assert_eq!(got.flat, text(FLAT), "reported as read, not used");
    assert!(!got.sources_disagree, "nothing readable to compare");
}

#[test]
fn a_repeated_registration_field_fails_its_own_ledger_not_the_read() {
    let ledger = |name: &str, id: u8, registrations: &str| {
        format!(
            "<LEDGER NAME=\"{name}\" RESERVEDNAME=\"\"><GUID>{COMPANY_GUID}-000000{id:02x}</GUID><BRIDGECOMPANYGUID>{COMPANY_GUID}</BRIDGECOMPANYGUID><MASTERID>{id}</MASTERID><ALTERID>{id}</ALTERID><PARENT>Sundry Creditors</PARENT>{registrations}<OPENINGBALANCE>0.00</OPENINGBALANCE></LEDGER>"
        )
    };
    let repeated = |gstins: &str| {
        format!("<LEDGSTREGDETAILS.LIST><APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>{gstins}</LEDGSTREGDETAILS.LIST>")
    };
    let response = format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>{}{}{}{}</COLLECTION></DATA></BODY></ENVELOPE>",
        ledger(
            "Repeated",
            1,
            &repeated("<GSTIN>27ZZZZZ0000Z1Z5</GSTIN><GSTIN>29ZZZZZ0000Z1Z5</GSTIN>")
        ),
        ledger("Empty element", 2, "<LEDGSTREGDETAILS.LIST/>"),
        ledger("Empty then value", 3, &repeated("<GSTIN></GSTIN><GSTIN>29ZZZZZ0000Z1Z5</GSTIN>")),
        ledger("Self-closing then value", 4, &repeated("<GSTIN/><GSTIN>29ZZZZZ0000Z1Z5</GSTIN>")),
    );
    let parsed = parse_native_party_ledger_master_records_with_evidence(&response, COMPANY_GUID)
        .expect("one ledger's defect does not refuse the book");
    let histories = parsed
        .records
        .iter()
        .map(|row| serde_json::to_value(&row.record.fields.gst_registrations).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        histories,
        vec![
            json!({"observation": "unreadable", "defect": "entry_repeats_a_field"}),
            json!({"observation": "entries", "entries": []}),
            json!({"observation": "unreadable", "defect": "entry_repeats_a_field"}),
            json!({"observation": "unreadable", "defect": "entry_repeats_a_field"}),
        ],
        "a field seen twice is a repeat even when the first carried no text"
    );
}

#[test]
fn gst_duty_head_vocabulary_is_explicit_and_irregular() {
    for (raw, head) in [
        ("CGST", GstDutyHead::Cgst),
        ("IGST", GstDutyHead::Igst),
        ("State Tax", GstDutyHead::StateTax),
        ("SGST/UTGST", GstDutyHead::SgstUtgst),
        ("UT Tax", GstDutyHead::UtTax),
        ("Cess", GstDutyHead::Cess),
    ] {
        assert_eq!(
            GstDutyHeadObservation::from_observations(
                &PartyLedgerMasterFieldObservation::Returned("GST".to_string()),
                &PartyLedgerMasterFieldObservation::Returned(raw.to_string()),
            ),
            GstDutyHeadObservation::Recognized {
                raw: raw.to_string(),
                head,
            }
        );
    }
    for raw in [
        "SGST",
        "Central Tax",
        "Integrated Tax",
        "Central",
        "State",
        "Integrated",
        "Union Territory Tax",
        "sgst/utgst",
        "SGST / UTGST",
        "SGST/UTGST ",
        "UTGST",
    ] {
        assert_eq!(
            GstDutyHeadObservation::from_observations(
                &PartyLedgerMasterFieldObservation::Returned("GST".to_string()),
                &PartyLedgerMasterFieldObservation::Returned(raw.to_string()),
            ),
            GstDutyHeadObservation::Unrecognized {
                raw: raw.to_string(),
            }
        );
    }
}

// -- ledger_masters ancestry exposure ---------------------------------------
//
// GroupIndex::ancestry_chain itself is proven (including by mutation) in
// bridge_tally_protocol::group_ancestry's own test module. These cover the
// wire-shaping layer this file owns: rendering a chain to JSON, mapping every
// AncestryGap to a stable wire string, and the group/group_scope filter that
// now reads that chain.

fn group(
    name: &str,
    parent: &str,
    reserved: Option<&str>,
) -> bridge_tally_protocol::TallyNamedMaster {
    bridge_tally_protocol::TallyNamedMaster {
        name: name.into(),
        parent: PartyLedgerMasterFieldObservation::Returned(parent.into()),
        reserved_name: reserved.map(str::to_string),
    }
}

/// Mirrors the lab's own tree: `HDFC CC` under `Bank OD A/c` under `Loans
/// (Liability)`, and `Cash` under `Cash-in-Hand`, both reaching the reserved
/// root directly.
fn lab_shaped_groups() -> bridge_tally_protocol::group_ancestry::GroupIndex {
    bridge_tally_protocol::group_ancestry::GroupIndex::build([
        group("Bank OD A/c", "Loans (Liability)", Some("Bank OD A/c")),
        group(
            "Loans (Liability)",
            "\u{fffd}#4; Primary",
            Some("Loans (Liability)"),
        ),
        group("Cash-in-Hand", "\u{fffd}#4; Primary", Some("Cash-in-Hand")),
    ])
}

#[test]
fn ancestry_json_renders_a_complete_multi_level_chain() {
    let chain = lab_shaped_groups().ancestry_chain(Some("Bank OD A/c"));
    let rendered = ancestry_json(&chain);
    assert_eq!(rendered["complete"], true);
    assert_eq!(rendered["gap"], Value::Null);
    assert_eq!(
        rendered["chain"],
        json!([
            {"name": "Bank OD A/c", "reserved_name": "Bank OD A/c"},
            {"name": "Loans (Liability)", "reserved_name": "Loans (Liability)"},
        ])
    );
}

#[test]
fn ancestry_json_renders_a_single_hop_directly_under_a_primary_group() {
    let chain = lab_shaped_groups().ancestry_chain(Some("Cash-in-Hand"));
    let rendered = ancestry_json(&chain);
    assert_eq!(rendered["complete"], true);
    assert_eq!(rendered["gap"], Value::Null);
    assert_eq!(
        rendered["chain"],
        json!([{"name": "Cash-in-Hand", "reserved_name": "Cash-in-Hand"}])
    );
}

#[test]
fn ancestry_json_reports_an_incomplete_chain_without_padding_or_guessing() {
    // "Bank OD A/c" is present but its own parent "Loans (Liability)" is not
    // in this narrower index, so the resolved prefix must stop exactly there.
    let narrow = bridge_tally_protocol::group_ancestry::GroupIndex::build([group(
        "Bank OD A/c",
        "Loans (Liability)",
        Some("Bank OD A/c"),
    )]);
    let chain = narrow.ancestry_chain(Some("Bank OD A/c"));
    let rendered = ancestry_json(&chain);
    assert_eq!(rendered["complete"], false);
    assert_eq!(rendered["gap"], "group_absent");
    assert_eq!(
        rendered["chain"],
        json!([{"name": "Bank OD A/c", "reserved_name": "Bank OD A/c"}]),
        "the one hop actually resolved must still be reported, not dropped"
    );
}

#[test]
fn every_ancestry_gap_has_a_distinct_stable_wire_code() {
    let mut codes = std::collections::BTreeSet::new();
    for gap in [
        AncestryGap::NoParent,
        AncestryGap::ReachedRoot,
        AncestryGap::GroupAbsent,
        AncestryGap::GroupNameRepeated,
        AncestryGap::ReservedNameMissing,
        AncestryGap::Cycle,
        AncestryGap::Exhausted,
    ] {
        assert!(
            codes.insert(ancestry_gap_code(gap)),
            "{gap:?} must render to a code no other gap also uses"
        );
    }
}

#[test]
fn group_scope_defaults_to_immediate_and_rejects_an_unknown_value() {
    assert_eq!(group_scope(&json!({})), Ok(GroupScope::Immediate));
    assert_eq!(
        group_scope(&json!({"group_scope": "immediate"})),
        Ok(GroupScope::Immediate)
    );
    assert_eq!(
        group_scope(&json!({"group_scope": "ancestry"})),
        Ok(GroupScope::Ancestry)
    );
    assert_eq!(
        group_scope(&json!({"group_scope": "everything"})),
        Err("argument_invalid:group_scope".to_string())
    );
}

#[test]
fn immediate_scope_never_reaches_past_the_ledgers_own_parent() {
    let index = lab_shaped_groups();
    let chain = index.ancestry_chain(Some("Bank OD A/c"));
    let hop_names = chain
        .hops
        .iter()
        .map(|hop| hop.name.clone())
        .collect::<Vec<_>>();
    assert!(hop_names.contains(&"Loans (Liability)".to_string()));
    // The immediate parent itself still matches under either scope.
    assert!(group_matches(
        GroupScope::Immediate,
        "Bank OD A/c",
        Some("Bank OD A/c"),
        &hop_names
    ));
    // But the original tool behaviour is preserved: a deeper ancestor is
    // invisible to Immediate, exactly as it always was.
    assert!(!group_matches(
        GroupScope::Immediate,
        "Loans (Liability)",
        Some("Bank OD A/c"),
        &hop_names
    ));
}

#[test]
fn ancestry_scope_matches_any_hop_but_never_an_unresolved_tail() {
    let index = lab_shaped_groups();
    let chain = index.ancestry_chain(Some("Bank OD A/c"));
    let hop_names = chain
        .hops
        .iter()
        .map(|hop| hop.name.clone())
        .collect::<Vec<_>>();
    assert!(group_matches(
        GroupScope::Ancestry,
        "Loans (Liability)",
        Some("Bank OD A/c"),
        &hop_names
    ));
    // A name that is not anywhere in the resolved chain must not match --
    // ancestry scope broadens what counts as a hit, it never invents one.
    assert!(!group_matches(
        GroupScope::Ancestry,
        "Sundry Debtors",
        Some("Bank OD A/c"),
        &hop_names
    ));

    // A ledger whose ancestry has a gap must never match a name that only
    // the unresolved tail could have reached: the resolved prefix is all
    // `group_matches` is given, and it must not be treated as the full chain.
    let narrow = bridge_tally_protocol::group_ancestry::GroupIndex::build([group(
        "Bank OD A/c",
        "Loans (Liability)",
        Some("Bank OD A/c"),
    )]);
    let gapped = narrow.ancestry_chain(Some("Bank OD A/c"));
    let gapped_names = gapped
        .hops
        .iter()
        .map(|hop| hop.name.clone())
        .collect::<Vec<_>>();
    assert!(!gapped.is_complete());
    assert!(!group_matches(
        GroupScope::Ancestry,
        "Loans (Liability)",
        Some("Bank OD A/c"),
        &gapped_names
    ));
}

/// The filter report is not paged with the rows, so its sub-group list is
/// bounded where it is built; the counts still cover every row.
#[test]
fn a_filter_report_names_at_most_twenty_sub_groups_and_counts_them_all() {
    let mut groups = vec![group(
        "Sundry Debtors",
        "\u{fffd}#4; Primary",
        Some("Sundry Debtors"),
    )];
    let mut rows = Vec::new();
    for n in 0..25 {
        let name = format!("Debtor Group {n:02}");
        groups.push(group(&name, "Sundry Debtors", Some("")));
        rows.push(json!({"name": format!("Ledger {n:02}"), "parent": name}));
    }
    // A second ledger in one sub-group: ledgers and sub-groups are counted
    // separately.
    rows.push(json!({"name": "Ledger 00b", "parent": "Debtor Group 00"}));
    let report = apply_group_filter(
        &mut rows,
        GroupScope::Immediate,
        "Sundry Debtors",
        &bridge_tally_protocol::group_ancestry::GroupIndex::build(groups),
    );
    assert!(rows.is_empty());
    let excluded = &report["excluded_subgroup_ledgers"];
    assert_eq!(excluded["count"], 26);
    assert_eq!(excluded["group_count"], 25);
    let named = excluded["groups"].as_array().unwrap();
    assert_eq!(named.len(), 20);
    assert_eq!(named[0], "Debtor Group 00");
    assert_eq!(report["unresolved_ancestry_ledgers"], 0);
}

// -- ledger_masters ancestry through the tool call ---------------------------
//
// The tests above call the helpers directly, so they stay green if
// `Server::ledger_masters` stops calling them: an inverted or deleted
// refusal, the wrong collection handed to `GroupIndex::build`, or a `retain`
// that no longer applies. These drive `call_tool("ledger_masters", ..)` over
// a replayed Tally sequence built from the captured party-master fixtures.

mod through_the_tool {
    use super::*;
    use tally_protocol_simulator::{
        Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
    };

    const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";

    fn captured(bytes: &[u8]) -> String {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }

    fn companies() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
        ))
    }

    fn masters() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-masters.utf16le.xml"
        ))
    }

    /// The captured answer to the compliance read that names the dated mailing
    /// details, moved to this harness's company: its company GUID (the
    /// `BRIDGECOMPANYGUID` of every row and the prefix of every ledger GUID) is
    /// replaced with the one the identity read here returns. No other byte of
    /// the capture is changed.
    fn mailing_masters() -> String {
        let capture = captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-masters-mailing-details.utf16le.xml"
        ));
        let captured_guid = "8e34483e-2bd3-472b-ae37-c4b197e8c15a";
        assert_eq!(captured_guid.len(), GUID.len());
        assert!(capture.contains(captured_guid));
        capture.replace(captured_guid, GUID)
    }

    /// A stand-in for the balance read of the mailing capture's company: the
    /// captured balance answer's envelope with one all-zero row per ledger of
    /// `masters`, in the same order, with the same name, reserved name and
    /// parent. It is not evidence of anything Tally sent; the tool joins the
    /// balance read to the masters, and no balance answer was captured for that
    /// company.
    fn balances_matching(masters: &str) -> String {
        let template = balances();
        let start = template.find("<LEDGER NAME=").expect("a captured row");
        let end = template
            .rfind("</COLLECTION>")
            .expect("the collection's end");
        let mut rows = String::new();
        for part in masters.split("<LEDGER NAME=\"").skip(1) {
            let name = part.split('"').next().unwrap();
            let reserved = part
                .split("RESERVEDNAME=\"")
                .nth(1)
                .and_then(|rest| rest.split('"').next())
                .unwrap();
            let parent = part
                .split("<PARENT TYPE=\"String\">")
                .nth(1)
                .and_then(|rest| rest.split("</PARENT>").next())
                .unwrap();
            rows.push_str(&format!(
                "    <LEDGER NAME=\"{name}\" RESERVEDNAME=\"{reserved}\">\n     <PARENT TYPE=\"String\">{parent}</PARENT>\n     <ISBILLWISEON TYPE=\"Logical\">No</ISBILLWISEON>\n     <CLOSINGBALANCE TYPE=\"Amount\">0.00</CLOSINGBALANCE>\n     <OPENINGBALANCE TYPE=\"Amount\">0.00</OPENINGBALANCE>\n     <BRIDGECOMPANYGUID TYPE=\"String\">{GUID}</BRIDGECOMPANYGUID>\n    </LEDGER>\n"
            ));
        }
        format!("{}{}{}", &template[..start], rows, &template[end..])
    }

    fn balances() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-balances.utf16le.xml"
        ))
    }

    fn groups() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-groups.utf16le.xml"
        ))
    }

    fn xml(body: String) -> ScenarioPlan {
        ScenarioPlan::new(Fixture::SyntheticXml(body)).with_encoding(WireEncoding::Utf16Le)
    }

    fn status() -> ScenarioPlan {
        ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
    }

    fn pair(plans: &mut Vec<ScenarioPlan>, source: ScenarioPlan) {
        plans.extend([source.clone(), status(), source, status()]);
    }

    /// The paired company-identity read every tool call starts with.
    fn identity_plans() -> Vec<ScenarioPlan> {
        let mut plans = Vec::new();
        pair(&mut plans, xml(companies()));
        plans
    }

    /// The whole successful `fields=compliance` sequence: identity, the
    /// extent-bracketed currency read, then the profile probe and the
    /// extent-bracketed master/balance/group triple.
    fn compliance_plans(masters: String, balances: String) -> Vec<ScenarioPlan> {
        let company = xml(companies());
        let extent = xml(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )
        .to_owned());
        let currency = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        )));
        let mut plans = identity_plans();
        plans.push(company.clone());
        pair(&mut plans, extent.clone());
        pair(&mut plans, currency);
        pair(&mut plans, extent.clone());
        plans.push(company.clone());
        plans.extend([status(), company.clone(), company.clone()]);
        pair(&mut plans, extent.clone());
        for source in [masters, balances, groups()] {
            pair(&mut plans, xml(source));
        }
        pair(&mut plans, extent);
        plans.extend([company.clone(), status(), company]);
        plans
    }

    // -- #637: the compliance read is sized before its master request -------

    /// The captured extents with only this company's master mark (`ALTMSTID`)
    /// changed. The same text serves every extent read of the call, so the
    /// brackets stay equal unless a test changes the closing one.
    fn extent_with_master_mark(mark: u64) -> String {
        extent_with_marks(mark, Some(None))
    }

    /// `extent_with_master_mark` with the captured company's voucher
    /// high-water (`ALTVCHID`) kept (`Some(None)`), moved to another value
    /// (`Some(Some(value))`) or removed (`None`).
    fn extent_with_marks(mark: u64, voucher: Option<Option<u64>>) -> String {
        let extent = include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        );
        let at = extent.find(GUID).expect("the captured company's extent");
        let start = extent[..at].rfind("<COMPANY ").unwrap();
        let end = at + extent[at..].find("</COMPANY>").unwrap();
        let from = "<ALTMSTID TYPE=\"Number\"> 219</ALTMSTID>";
        assert_eq!(extent[start..end].matches(from).count(), 1);
        let mut company = extent[start..end].replace(
            from,
            &format!("<ALTMSTID TYPE=\"Number\"> {mark}</ALTMSTID>"),
        );
        let voucher_line = company
            .lines()
            .find(|line| line.contains("<ALTVCHID "))
            .expect("the captured company has a voucher high-water")
            .to_owned();
        match voucher {
            Some(None) => {}
            Some(Some(value)) => {
                company = company.replace(
                    &voucher_line,
                    &format!("     <ALTVCHID TYPE=\"Number\"> {value}</ALTVCHID>"),
                );
            }
            None => {
                company = company
                    .split_inclusive('\n')
                    .filter(|line| !line.contains("<ALTVCHID "))
                    .collect();
            }
        }
        assert_eq!(voucher.is_none(), !company.contains("<ALTVCHID "));
        format!("{}{}{}", &extent[..start], company, &extent[end..])
    }

    /// The compliance sequence on a book whose master mark is `mark`, with the
    /// source reads in the order given. `closing` is the source's closing
    /// extent; `None` ends the replay after the reads, for a refusal that sends
    /// nothing more.
    fn marked_compliance_plans(
        mark: u64,
        reads: Vec<String>,
        closing: Option<String>,
    ) -> Vec<ScenarioPlan> {
        marked_plans_over(extent_with_master_mark(mark), reads, closing)
    }

    /// `marked_compliance_plans` over an extent text of the caller's making,
    /// used for every extent read before the source's closing one.
    fn marked_plans_over(
        extent: String,
        reads: Vec<String>,
        closing: Option<String>,
    ) -> Vec<ScenarioPlan> {
        let company = xml(companies());
        let extent = xml(extent);
        let currency = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        )));
        let mut plans = identity_plans();
        plans.push(company.clone());
        pair(&mut plans, extent.clone());
        pair(&mut plans, currency);
        pair(&mut plans, extent.clone());
        plans.push(company.clone());
        plans.extend([status(), company.clone(), company.clone()]);
        pair(&mut plans, extent);
        for source in reads {
            pair(&mut plans, xml(source));
        }
        if let Some(closing) = closing {
            pair(&mut plans, xml(closing));
            plans.extend([company.clone(), status(), company]);
        }
        plans
    }

    /// A master mark past what the census covers (400,000) refuses right after
    /// the source's opening extent: no census, catalogue, ledger, balance or
    /// group request is sent, because a response past the transport's cap is
    /// cut off mid-read (#679). The refusal names the mark as an upper bound.
    #[tokio::test]
    async fn a_book_whose_mark_is_past_the_census_is_refused_before_any_ledger_read() {
        for mark in [400_001_u64, 1_000_000] {
            let plans = marked_compliance_plans(mark, Vec::new(), None);
            let total = plans.len();
            let (response, requests) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(requests, total, "nothing is sent after the opening extent");
            let error = refusal(&response);
            assert_eq!(error["code"], "party_ledger_master_read_failed");
            assert_eq!(error["cause"], "ledger_catalogue_too_large");
            assert_eq!(
                error["size"],
                json!({"master_alter_id": mark, "estimated_bytes": mark * 1_400, "limit_bytes": 32_000_000, "limit_master_alter_id": 400_000})
            );
            let remediation = error["remediation"].as_str().unwrap();
            assert!(remediation.contains("UPPER BOUND"), "{error}");
            assert!(remediation.contains("fields=basic"), "{error}");
        }
    }

    /// The captured nine-ledger catalogue.
    fn catalogue() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
        ))
    }

    /// One generated ledger: a distinct name and GUID under a named parent.
    struct Generated {
        index: usize,
        name: String,
        parent: &'static str,
    }

    /// `spec` parents with that many ledgers each, in spec order.
    fn generated(spec: &[(&'static str, usize)]) -> Vec<Generated> {
        let mut rows = Vec::new();
        for (parent, count) in spec {
            for _ in 0..*count {
                let index = rows.len();
                rows.push(Generated {
                    index,
                    name: format!("Generated Ledger {index:05}"),
                    parent,
                });
            }
        }
        rows
    }

    /// The rows whose parent is one of `parents`.
    fn under<'a>(rows: &'a [Generated], parents: &[&str]) -> Vec<&'a Generated> {
        rows.iter()
            .filter(|row| parents.contains(&row.parent))
            .collect()
    }

    /// A captured party-master response with its ledgers replaced by
    /// `rows`, each cut from the captured first ledger. The catalogue, master
    /// and balance reads all use it; the opening balance is the row's index, so
    /// the master and the balance agree.
    fn with_ledgers(captured_response: String, rows: &[&Generated]) -> String {
        const MARK: &str = "</LEDGER>";
        let first = captured_response.find("    <LEDGER ").unwrap();
        let first_end = first + captured_response[first..].find(MARK).unwrap() + MARK.len();
        let last_end = captured_response.rfind(MARK).unwrap() + MARK.len();
        let template = captured_response[first..first_end].trim_start();
        let mut out = captured_response[..first].to_owned();
        for (position, row) in rows.iter().enumerate() {
            if position > 0 {
                out.push_str("\n    ");
            }
            out.push_str(
                &template
                    .replace("Bridge Nested Debtors WR4", row.parent)
                    .replace("Bridge Nested Debtor WR4", &row.name)
                    .replace("-000000d5", &format!("-{:08x}", 0x1000 + row.index))
                    .replace(
                        "<ALTERID TYPE=\"Number\"> 215<",
                        &format!("<ALTERID TYPE=\"Number\"> {}<", 1_000 + row.index),
                    )
                    .replace(
                        "<MASTERID TYPE=\"Number\"> 213<",
                        &format!("<MASTERID TYPE=\"Number\"> {}<", 10 + row.index),
                    )
                    .replace(">-50000.00<", &format!(">-{}.00<", 1 + row.index)),
            );
        }
        out.push_str(&captured_response[last_end..]);
        out
    }

    fn generated_catalogue(rows: &[&Generated]) -> String {
        with_ledgers(catalogue(), rows)
    }

    fn generated_masters(rows: &[&Generated]) -> String {
        with_ledgers(masters(), rows)
    }

    fn generated_balances(rows: &[&Generated]) -> String {
        with_ledgers(balances(), rows)
    }

    /// A mark past the master bound but within the catalogue's reach is
    /// counted first, and the count admits it: the catalogue pair, then the
    /// same three reads in the same order, and the same rows as the unsized
    /// read (#668).
    #[tokio::test]
    async fn a_book_whose_counted_ledgers_fit_is_read_though_its_mark_does_not() {
        let plans = marked_compliance_plans(
            5_000,
            vec![catalogue(), masters(), balances(), groups()],
            Some(extent_with_master_mark(5_000)),
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total);
        let (unsized_response, _) = call(
            compliance_plans(masters(), balances()),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        assert_eq!(items(&response), items(&unsized_response));
    }

    /// A book counted at 4,267 ledgers under one parent, one more than a
    /// read holds, cannot be split by parent: it refuses right after the
    /// catalogue, and no master request is sent (#679).
    #[tokio::test]
    async fn a_parent_holding_more_ledgers_than_one_read_is_refused_before_the_master_read() {
        let rows = generated(&[("Sundry Debtors", 4_267)]);
        let plans = marked_compliance_plans(
            6_000,
            vec![generated_catalogue(&under(&rows, &["Sundry Debtors"]))],
            None,
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "nothing is sent after the catalogue pair");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "parent_over_budget");
    }

    const BIG: &str = "Sundry Debtors";
    const NESTED: &str = "Bridge Nested Debtors WR4";
    const OTHER: &str = "Sundry Creditors";

    /// 4,300 ledgers under three parents. Largest first, the debtors and the
    /// nested group fill the first read (3,300) and the creditors the second.
    fn split_book() -> Vec<Generated> {
        generated(&[(BIG, 2_200), (OTHER, 1_000), (NESTED, 1_100)])
    }

    /// The whole sequence for the split book: the catalogue, then one master
    /// and one balance read per part, then the groups.
    fn split_plans(
        mark: u64,
        first: (String, String),
        second: (String, String),
    ) -> Vec<ScenarioPlan> {
        split_plans_closing(mark, first, second, extent_with_master_mark(mark))
    }

    /// `split_plans` whose source ends on the given closing extent.
    fn split_plans_closing(
        mark: u64,
        first: (String, String),
        second: (String, String),
        closing: String,
    ) -> Vec<ScenarioPlan> {
        let rows = split_book();
        let mut plans = marked_compliance_plans(
            mark,
            vec![
                generated_catalogue(&rows.iter().collect::<Vec<_>>()),
                first.0,
                first.1,
                second.0,
                second.1,
                groups(),
            ],
            None,
        );
        pair(&mut plans, xml(closing));
        plans
    }

    fn part_reads(rows: &[&Generated]) -> (String, String) {
        (generated_masters(rows), generated_balances(rows))
    }

    /// A book too large for one read but with no parent too large is read in
    /// parts, one filtered master and balance pair per part, and every ledger
    /// comes back once, whatever its mark: 6,000 is within the old mark cap,
    /// 10,001 and 20,000 were refused on the mark alone before #679 (#679).
    #[tokio::test]
    async fn a_book_too_large_for_one_read_is_read_in_parts_by_parent() {
        for mark in [6_000_u64, 10_001, 20_000] {
            let rows = split_book();
            let mut plans = split_plans(
                mark,
                part_reads(&under(&rows, &[BIG, NESTED])),
                part_reads(&under(&rows, &[OTHER])),
            );
            plans.extend([xml(companies()), status(), xml(companies())]);
            let total = plans.len();
            let (response, requests) = call_with_max_bytes(
                plans,
                json!({"company_guid":GUID,"fields":"compliance"}),
                2_000_000,
            )
            .await;
            assert_eq!(requests, total, "mark {mark}");
            assert_ne!(response["isError"], true, "mark {mark}: {response}");
            assert_eq!(response["structuredContent"]["result"]["total"], 4_300);
            let names = items(&response)
                .iter()
                .map(|item| item["name"].as_str().unwrap())
                .collect::<Vec<_>>();
            assert!(
                names.windows(2).all(|pair| pair[0] <= pair[1]),
                "sorted by name"
            );
        }
    }

    async fn split_refusal(first: (String, String), second: (String, String)) -> String {
        let plans = split_plans(6_000, first, second);
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "the whole bracket is read before coverage");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        error["cause"].as_str().unwrap().to_owned()
    }

    /// The reads of a split book that end where Bridge must stop: the
    /// catalogue, then each read in `reads`, and nothing after it. Fails if
    /// Bridge sent a request past the refusal, and returns the cause.
    async fn stops_after(rows: &[Generated], reads: Vec<String>) -> String {
        let mut sequence = vec![generated_catalogue(&rows.iter().collect::<Vec<_>>())];
        sequence.extend(reads);
        let plans = marked_compliance_plans(6_000, sequence, None);
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "a request was sent past the refusal");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        error["cause"].as_str().unwrap().to_owned()
    }

    /// A part that omits one of its parent's ledgers, as when a ledger was
    /// deleted, or a filter was ignored in the wrong direction, stops the read
    /// right after its master: its balance and every later part are never
    /// requested (#679).
    #[tokio::test]
    async fn a_part_that_omits_a_ledger_of_its_parents_stops_the_read_after_its_master() {
        let rows = split_book();
        let mut short = under(&rows, &[BIG, NESTED]);
        short.pop();
        let cause = stops_after(&rows, vec![part_reads(&short).0]).await;
        assert_eq!(cause, "parent_part_row_count_differs");
    }

    /// A part that carries another part's ledger, as when Tally ignored the
    /// filter, stops the read the same way rather than being deduplicated.
    #[tokio::test]
    async fn a_part_that_carries_another_parts_ledger_stops_the_read_after_its_master() {
        let rows = split_book();
        let mut wide = under(&rows, &[BIG, NESTED]);
        wide.push(under(&rows, &[OTHER])[0]);
        let cause = stops_after(&rows, vec![part_reads(&wide).0]).await;
        assert_eq!(cause, "parent_part_row_count_differs");
    }

    /// A part whose answer passes the response cap, as when Tally ignored its
    /// filter and returned the whole book, is refused under its own cause, not
    /// as a bare read failure, and nothing is sent after it (#679).
    #[tokio::test]
    async fn a_part_past_the_response_cap_is_refused_under_its_own_cause() {
        let rows = split_book();
        let mut plans = marked_compliance_plans(
            6_000,
            vec![generated_catalogue(&rows.iter().collect::<Vec<_>>())],
            None,
        );
        plans.push(
            xml(part_reads(&under(&rows, &[BIG, NESTED])).0).with_framing(
                ResponseFraming::DeclaredContentLength {
                    bytes: bridge_tally_transport::XML_RESPONSE_MAX_BYTES + 1,
                },
            ),
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(
            requests, total,
            "a request was sent past the oversized answer"
        );
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "parent_part_response_too_large");
        assert_eq!(
            error["remediation"],
            crate::agent::refusal_remediation("parent_part_response_too_large").unwrap()
        );
    }

    /// A part with the right number of ledgers, one of them another part's,
    /// passes the count and is refused by coverage after the whole bracket.
    #[tokio::test]
    async fn a_part_that_swaps_in_another_parts_ledger_is_refused_by_coverage() {
        let rows = split_book();
        let mut swapped = under(&rows, &[BIG, NESTED]);
        swapped.pop();
        swapped.push(under(&rows, &[OTHER])[0]);
        let cause = split_refusal(part_reads(&swapped), part_reads(&under(&rows, &[OTHER]))).await;
        assert_eq!(cause, "parent_part_row_outside_parents");
    }

    /// A ledger renamed between the catalogue and its part is refused.
    #[tokio::test]
    async fn a_part_whose_ledger_differs_from_the_catalogue_is_refused() {
        let rows = split_book();
        let (masters_0, balances_0) = part_reads(&under(&rows, &[BIG, NESTED]));
        let renamed = |text: String| text.replace("Generated Ledger 00005", "Renamed Ledger");
        let cause = split_refusal(
            (renamed(masters_0), renamed(balances_0)),
            part_reads(&under(&rows, &[OTHER])),
        )
        .await;
        assert_eq!(cause, "parent_part_row_differs_from_catalogue");
    }

    /// The voucher high-water moving between the parts of a split read, as when
    /// a voucher is posted after the first part's balances were read, refuses the
    /// whole read on its closing extent: no rows are released (#679).
    #[tokio::test]
    async fn a_split_read_whose_book_changes_between_parts_is_refused() {
        let rows = split_book();
        let plans = split_plans_closing(
            6_000,
            part_reads(&under(&rows, &[BIG, NESTED])),
            part_reads(&under(&rows, &[OTHER])),
            extent_with_marks(6_000, Some(Some(999_999))),
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(
            requests, total,
            "every part was read before the closing extent"
        );
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "party_ledger_extent_changed");
    }

    /// A book too large for one read whose Tally reports no voucher high-water
    /// cannot be proved unchanged across parts, so it is refused after the
    /// catalogue and before any part is requested (#679).
    #[tokio::test]
    async fn a_split_read_needs_the_voucher_high_water_before_any_part_is_read() {
        let rows = split_book();
        let plans = marked_plans_over(
            extent_with_marks(6_000, None),
            vec![generated_catalogue(&rows.iter().collect::<Vec<_>>())],
            None,
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "nothing is sent after the catalogue pair");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "parent_partition_voucher_witness_absent");
        assert!(error["remediation"]
            .as_str()
            .unwrap()
            .contains("fields=basic"));
    }

    const ODD: &str = "Odd\tParent";

    /// The split book plus `odd` ledgers under a parent name no filter can
    /// carry (a control character), and the whole sequence for reading it:
    /// the catalogue, the two named parts, then the complement part, then the
    /// groups.
    fn complement_book(
        odd: usize,
        reads: impl FnOnce(&[Generated]) -> [(String, String); 3],
    ) -> (Vec<Generated>, Vec<ScenarioPlan>) {
        let mut rows = split_book();
        for _ in 0..odd {
            let index = rows.len();
            rows.push(Generated {
                index,
                name: format!("Generated Ledger {index:05}"),
                parent: ODD,
            });
        }
        let [first, second, third] = reads(&rows);
        let mut plans = marked_compliance_plans(
            6_000,
            vec![
                generated_catalogue(&rows.iter().collect::<Vec<_>>()),
                first.0,
                first.1,
                second.0,
                second.1,
                third.0,
                third.1,
                groups(),
            ],
            None,
        );
        pair(&mut plans, xml(extent_with_master_mark(6_000)));
        (rows, plans)
    }

    fn complement_reads(rows: &[Generated]) -> [(String, String); 3] {
        [
            part_reads(&under(rows, &[BIG, NESTED])),
            part_reads(&under(rows, &[OTHER])),
            part_reads(&under(rows, &[ODD])),
        ]
    }

    /// Ledgers under a parent name with a control character cannot be named by
    /// any filter, so they are read as one extra part that excludes every
    /// named parent, and every ledger comes back once (#679).
    #[tokio::test]
    async fn ledgers_under_an_unnameable_parent_are_read_as_a_complement_part() {
        let (_, mut plans) = complement_book(3, complement_reads);
        plans.extend([xml(companies()), status(), xml(companies())]);
        let total = plans.len();
        let (response, requests) = call_with_max_bytes(
            plans,
            json!({"company_guid":GUID,"fields":"compliance"}),
            2_000_000,
        )
        .await;
        assert_eq!(requests, total);
        assert_ne!(response["isError"], true, "{response}");
        assert_eq!(response["structuredContent"]["result"]["total"], 4_303);
    }

    /// A named part that comes back short stops the read before the
    /// complement is requested: the complement's filter is built on the same
    /// assumption, so nothing more is sent to Tally (#679).
    #[tokio::test]
    async fn a_short_named_part_stops_the_read_before_the_complement_is_sent() {
        let (rows, _) = complement_book(3, complement_reads);
        let mut short = under(&rows, &[OTHER]);
        short.pop();
        let first = part_reads(&under(&rows, &[BIG, NESTED]));
        let second = part_reads(&short);
        let cause = stops_after(&rows, vec![first.0, first.1, second.0]).await;
        assert_eq!(cause, "parent_part_row_count_differs");
    }

    /// A complement part that returns a ledger of a named part, as when the
    /// exclusion was ignored, stops the read after its master.
    #[tokio::test]
    async fn a_complement_part_that_carries_a_named_ledger_stops_the_read() {
        let (rows, _) = complement_book(3, complement_reads);
        let mut wide = under(&rows, &[ODD]);
        wide.push(under(&rows, &[OTHER])[0]);
        let first = part_reads(&under(&rows, &[BIG, NESTED]));
        let second = part_reads(&under(&rows, &[OTHER]));
        let cause = stops_after(
            &rows,
            vec![first.0, first.1, second.0, second.1, part_reads(&wide).0],
        )
        .await;
        assert_eq!(cause, "parent_part_row_count_differs");
    }

    /// A complement part that swaps one unnameable ledger for a named one has
    /// the right count, so coverage refuses it as a repeat once the bracket is
    /// read.
    #[tokio::test]
    async fn a_complement_part_that_swaps_in_a_named_ledger_is_refused_as_a_repeat() {
        let (_, plans) = complement_book(3, |rows| {
            let mut swapped = under(rows, &[ODD]);
            swapped.pop();
            swapped.push(under(rows, &[OTHER])[0]);
            [
                part_reads(&under(rows, &[BIG, NESTED])),
                part_reads(&under(rows, &[OTHER])),
                part_reads(&swapped),
            ]
        });
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "the whole bracket is read before coverage");
        assert_eq!(refusal(&response)["cause"], "parent_part_row_repeated");
    }

    /// A complement part that omits one of the unnameable ledgers stops the
    /// read after its master.
    #[tokio::test]
    async fn a_complement_part_that_omits_a_ledger_stops_the_read() {
        let (rows, _) = complement_book(3, complement_reads);
        let mut short = under(&rows, &[ODD]);
        short.pop();
        let first = part_reads(&under(&rows, &[BIG, NESTED]));
        let second = part_reads(&under(&rows, &[OTHER]));
        let cause = stops_after(
            &rows,
            vec![first.0, first.1, second.0, second.1, part_reads(&short).0],
        )
        .await;
        assert_eq!(cause, "parent_part_row_count_differs");
    }

    /// More unnameable ledgers than one read holds are refused right after
    /// the catalogue, with the parent name not echoed (#679).
    #[tokio::test]
    async fn more_unnameable_ledgers_than_one_read_holds_are_refused_after_the_catalogue() {
        let rows = generated(&[(BIG, 1), (ODD, 4_267)]);
        let plans = marked_compliance_plans(
            6_000,
            vec![generated_catalogue(&rows.iter().collect::<Vec<_>>())],
            None,
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "nothing is sent after the catalogue pair");
        assert_eq!(refusal(&response)["cause"], "parent_over_budget");
        assert!(!response.to_string().contains("Odd"));
    }

    /// A book whose every ledger sits under an unnameable parent has no named
    /// part to exclude, so it is refused with the count of them, the cause
    /// that says so, and no name echoed (#679).
    #[tokio::test]
    async fn a_book_with_no_nameable_parent_is_refused_with_its_ledger_count() {
        let rows = generated(&[(ODD, 4_300)]);
        let plans = marked_compliance_plans(
            6_000,
            vec![generated_catalogue(&rows.iter().collect::<Vec<_>>())],
            None,
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "nothing is sent after the catalogue pair");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "parent_name_unsupported");
        assert_eq!(error["unsupported_parent_ledgers"], 4_300);
        assert!(!response.to_string().contains("Odd"), "{error}");
        assert!(error["remediation"]
            .as_str()
            .unwrap()
            .contains("fields=basic"));
    }

    /// One part needs no witness: a counted book that fits one read is read
    /// whole with or without a voucher high-water, as before.
    #[tokio::test]
    async fn a_counted_read_of_one_part_does_not_need_the_voucher_high_water() {
        let extent = extent_with_marks(5_000, None);
        let plans = marked_plans_over(
            extent.clone(),
            vec![catalogue(), masters(), balances(), groups()],
            Some(extent),
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total);
        assert_ne!(response["isError"], true, "{response}");
    }

    /// A part that returns one of its ledgers twice is refused on that read,
    /// before its balances are read: the master parser's own duplicate-identity
    /// refusal comes first, so the coverage check's repeat (`parent_part_row_repeated`,
    /// covered in the protocol crate) is a second line of defence, not a path
    /// the tool reaches.
    #[tokio::test]
    async fn a_part_that_repeats_a_ledger_is_refused_at_its_master_read() {
        let rows = split_book();
        let mut doubled = under(&rows, &[BIG, NESTED]);
        doubled.push(doubled[0]);
        let plans = marked_compliance_plans(
            6_000,
            vec![
                generated_catalogue(&rows.iter().collect::<Vec<_>>()),
                generated_masters(&doubled),
            ],
            None,
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(
            requests, total,
            "nothing is sent after the doubled master pair"
        );
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "duplicate_master_identity");
    }

    /// A catalogue that names another company does not size this one: it is
    /// refused on identity before any master request is sent.
    #[tokio::test]
    async fn a_count_from_another_company_is_refused_before_the_master_read() {
        let other = catalogue().replace(GUID, "00000000-0000-0000-0000-000000000000");
        let plans = marked_compliance_plans(5_000, vec![other], None);
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "nothing is sent after the catalogue pair");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "ledger_catalogue_identity_mismatch");
    }

    /// A catalogue pair whose second read disagrees with its first is refused
    /// on that drift before any master request is sent.
    #[tokio::test]
    async fn a_catalogue_that_changes_between_its_two_reads_is_refused_before_the_master_read() {
        let mut plans = marked_compliance_plans(5_000, Vec::new(), None);
        let changed = generated(&[(BIG, 10)]);
        plans.extend([
            xml(catalogue()),
            status(),
            xml(generated_catalogue(&under(&changed, &[BIG]))),
            status(),
        ]);
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "nothing is sent after the catalogue pair");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "party_ledger_catalogue_changed");
    }

    /// The catalogue pair that counted the ledgers is in the evidence of a
    /// read that succeeds, not only of one refused: its two bodies are the
    /// whole difference from the same book admitted by its mark.
    #[tokio::test]
    async fn a_counted_read_reports_the_catalogue_pair_in_its_evidence() {
        let bytes = |response: &Value| {
            response["structuredContent"]["evidence"]["bytes"]
                .as_u64()
                .unwrap()
        };
        let counted_plans = marked_compliance_plans(
            5_000,
            vec![catalogue(), masters(), balances(), groups()],
            Some(extent_with_master_mark(5_000)),
        );
        let (counted, _) = call(
            counted_plans,
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        let admitted_plans = marked_compliance_plans(
            4_266,
            vec![masters(), balances(), groups()],
            Some(extent_with_master_mark(4_266)),
        );
        let (admitted, _) = call(
            admitted_plans,
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        // UTF-16LE on the wire, with its two-byte byte-order mark.
        let catalogue_wire = 2 + 2 * catalogue().encode_utf16().count() as u64;
        assert_eq!(
            bytes(&counted) - bytes(&admitted),
            2 * catalogue_wire,
            "the paired catalogue bodies are counted"
        );
        assert_ne!(
            counted["structuredContent"]["evidence"]["request_sha256"],
            admitted["structuredContent"]["evidence"]["request_sha256"]
        );
    }

    // -- #679: the census counts a book whose mark is past its catalogue ------

    /// The synthetic lab company the census slices were captured from.
    const LAB_COMPANY_GUID: &str = "f8dab51e-a5d9-49d5-9232-54a16c8b95bb";

    /// The captured eight-ledger census slice.
    fn census_capture() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/ledger-census-slice-eight-rows.utf16le.xml"
        ))
    }

    /// The captured answer to a slice holding no ledger.
    fn census_empty() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/ledger-census-slice-empty.utf16le.xml"
        ))
    }

    /// A census slice answer holding one ledger per id in `ids`, each cut from
    /// the captured slice's first row, under `company_guid`.
    fn census_slice(company_guid: &str, ids: std::ops::Range<u32>) -> String {
        const MARK: &str = "</LEDGER>";
        assert!(
            !ids.is_empty(),
            "an empty slice is the captured empty answer"
        );
        let capture = census_capture().replace(LAB_COMPANY_GUID, company_guid);
        let first = capture.find("    <LEDGER ").unwrap();
        let first_end = first + capture[first..].find(MARK).unwrap() + MARK.len();
        let last_end = capture.rfind(MARK).unwrap() + MARK.len();
        let template = capture[first..first_end].trim_start().to_owned();
        // The name sits twice in a row: as the row's attribute and as its `NAME`.
        assert_eq!(template.matches("SZ Debtor 0284").count(), 2);
        assert_eq!(template.matches("-000001f3").count(), 1);
        let mut out = capture[..first].to_owned();
        for (position, id) in ids.enumerate() {
            if position > 0 {
                out.push_str("\n    ");
            }
            out.push_str(
                &template
                    .replace("SZ Debtor 0284", &format!("Census Ledger {id}"))
                    .replace("-000001f3", &format!("-{:08x}", 0x2000_0000 + id)),
            );
        }
        out.push_str(&capture[last_end..]);
        out
    }

    /// The slices of a census of a book whose master mark is `mark`, one answer
    /// per slice of 4,000, empty except where `filled` puts ledgers.
    fn census_bodies(
        mark: u64,
        company_guid: &str,
        filled: &[(usize, std::ops::Range<u32>)],
    ) -> Vec<String> {
        let slices = mark.div_ceil(4_000) as usize;
        (0..slices)
            .map(
                |index| match filled.iter().find(|(slice, _)| *slice == index) {
                    Some((_, ids)) => census_slice(company_guid, ids.clone()),
                    None => census_empty(),
                },
            )
            .collect()
    }

    /// `marked_compliance_plans` for a census: the slices are single reads, one
    /// request each with no health check, then `reads` are paired as usual.
    fn census_plans(
        mark: u64,
        slices: Vec<String>,
        reads: Vec<String>,
        closing: Option<String>,
    ) -> Vec<ScenarioPlan> {
        let mut plans = marked_plans_over(extent_with_master_mark(mark), Vec::new(), None);
        plans.extend(slices.into_iter().map(xml));
        for source in reads {
            pair(&mut plans, xml(source));
        }
        if let Some(closing) = closing {
            pair(&mut plans, xml(closing));
            plans.extend([xml(companies()), status(), xml(companies())]);
        }
        plans
    }

    /// The company-count answer for the test company: the captured company-extent
    /// answer reduced to this company's row holding `NUMLEDGERS` (`None`: the field
    /// absent). The row's shape (name attribute, NAME, GUID, NUMLEDGERS with a
    /// leading space) is the live capture's (`company-ledger-count`, bridge#938);
    /// only the company it names is the test company's.
    fn company_count_body(numledgers: Option<&str>) -> String {
        let capture = include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        );
        let at = capture.find(GUID).expect("the captured company");
        let start = capture[..at].rfind("<COMPANY ").unwrap();
        let end = at + capture[at..].find("</COMPANY>").unwrap() + "</COMPANY>".len();
        let row = &capture[start..end];
        let name = row[row.find("NAME=\"").unwrap() + 6..]
            .split('"')
            .next()
            .unwrap();
        let field = numledgers
            .map(|value| format!("     <NUMLEDGERS TYPE=\"Number\"> {value}</NUMLEDGERS>\n"))
            .unwrap_or_default();
        let new_row = format!(
            "<COMPANY NAME=\"{name}\" RESERVEDNAME=\"\">\n     <NAME TYPE=\"String\">{name}</NAME>\n     <GUID TYPE=\"String\">{GUID}</GUID>\n{field}    </COMPANY>"
        );
        let first = capture.find("<COMPANY ").unwrap();
        let last_end = capture.rfind("</COMPANY>").unwrap() + "</COMPANY>".len();
        format!("{}{}{}", &capture[..first], new_row, &capture[last_end..])
    }

    /// `census_plans` for a census that completes: the book's extent is read
    /// again after the last slice, before any read is admitted by the count
    /// (#679). `after` is that extent's text.
    fn census_plans_after(
        mark: u64,
        slices: Vec<String>,
        after: String,
        reads: Vec<String>,
        closing: Option<String>,
    ) -> Vec<ScenarioPlan> {
        // Tally's own count of the ledgers agrees with the census's: the distinct
        // GUIDs the census's own parser reads from the slices, so text that merely
        // looks like a ledger row cannot skew it.
        let counted = slices
            .iter()
            .flat_map(|body| {
                bridge_tally_protocol::parse_ledger_census_slice(body, GUID, u64::MAX)
                    .expect("a census slice given to the helper parses")
            })
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        census_plans_counted(
            mark,
            slices,
            company_count_body(Some(&counted.to_string())),
            after,
            reads,
            closing,
        )
    }

    /// [`census_plans_after`] with the company's own ledger-count answer given.
    fn census_plans_counted(
        mark: u64,
        slices: Vec<String>,
        company_count: String,
        after: String,
        reads: Vec<String>,
        closing: Option<String>,
    ) -> Vec<ScenarioPlan> {
        let mut plans = marked_plans_over(extent_with_master_mark(mark), Vec::new(), None);
        plans.extend(slices.into_iter().map(xml));
        // One read, not a pair, before the extent is read again.
        plans.push(xml(company_count));
        pair(&mut plans, xml(after));
        for source in reads {
            pair(&mut plans, xml(source));
        }
        if let Some(closing) = closing {
            pair(&mut plans, xml(closing));
            plans.extend([xml(companies()), status(), xml(companies())]);
        }
        plans
    }

    /// [`census_plans_after`] with the extent unchanged.
    fn census_plans_checked(
        mark: u64,
        slices: Vec<String>,
        reads: Vec<String>,
        closing: Option<String>,
    ) -> Vec<ScenarioPlan> {
        census_plans_after(mark, slices, extent_with_master_mark(mark), reads, closing)
    }

    /// A book whose mark (102,161: the shape of a real book of 864 ledgers) is
    /// past what its catalogue can be read for is counted by AlterID span, one
    /// single read per slice of 4,000, and the count admits the whole read: the
    /// same rows as the unsized read (#679).
    #[tokio::test]
    async fn a_book_whose_mark_is_past_its_catalogue_is_counted_by_span_and_read_whole() {
        let mark = 102_161_u64;
        let slices = census_bodies(mark, GUID, &[(24, 0..9)]);
        assert_eq!(slices.len(), 26);
        let plans = census_plans_checked(
            mark,
            slices,
            vec![masters(), balances(), groups()],
            Some(extent_with_master_mark(mark)),
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total);
        assert_ne!(response["isError"], true, "{response}");
        let (unsized_response, _) = call(
            compliance_plans(masters(), balances()),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        assert_eq!(items(&response), items(&unsized_response));
    }

    /// Each slice's answer is accounted in the evidence once (a slice is one
    /// read, not a pair). The same nine ledgers counted as one filled slice or
    /// as two differ in evidence bytes by exactly the difference of the slices'
    /// wire sizes, so a slice counted twice would double that difference.
    #[tokio::test]
    async fn a_census_slice_is_accounted_once_in_the_evidence() {
        let mark = 102_161_u64;
        let wire = |bodies: &[String]| -> u64 {
            bodies
                .iter()
                .map(|body| 2 + 2 * body.encode_utf16().count() as u64)
                .sum()
        };
        let one_slice = census_bodies(mark, GUID, &[(24, 0..9)]);
        let two_slices = census_bodies(mark, GUID, &[(24, 0..5), (25, 5..9)]);
        assert_ne!(wire(&one_slice), wire(&two_slices));
        let mut bytes = Vec::new();
        for slices in [&one_slice, &two_slices] {
            let plans = census_plans_checked(
                mark,
                slices.clone(),
                vec![masters(), balances(), groups()],
                Some(extent_with_master_mark(mark)),
            );
            let (response, _) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_ne!(response["isError"], true, "{response}");
            bytes.push(
                response["structuredContent"]["evidence"]["bytes"]
                    .as_u64()
                    .unwrap(),
            );
        }
        assert_eq!(
            bytes[0] as i64 - bytes[1] as i64,
            wire(&one_slice) as i64 - wire(&two_slices) as i64
        );
    }

    /// The census's count is compared with the ledgers the whole read returns:
    /// a count of eight against nine master rows is refused right after the
    /// master pair, and no balance or group request is sent.
    #[tokio::test]
    async fn a_census_count_that_differs_from_the_master_read_stops_the_read_after_its_master() {
        let mark = 102_161_u64;
        let plans = census_plans_checked(
            mark,
            census_bodies(mark, GUID, &[(24, 0..8)]),
            vec![masters()],
            None,
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "a request was sent past the master pair");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "ledger_count_differs");
        assert_eq!(
            error["remediation"],
            crate::agent::refusal_remediation("ledger_count_differs").unwrap()
        );
    }

    /// A census that found no ledger is refused, not counted as an empty book:
    /// an empty slice is the answer a closed or absent company gives too.
    #[tokio::test]
    async fn a_census_that_finds_no_ledger_is_refused_after_its_last_slice() {
        let mark = 102_161_u64;
        let plans = census_plans(mark, census_bodies(mark, GUID, &[]), Vec::new(), None);
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "nothing is sent after the census");
        assert_eq!(refusal(&response)["cause"], "ledger_span_census_empty");
    }

    /// Every refusal inside the census stops it at the slice that caused it:
    /// the plans end there, and a request past them fails the count.
    async fn census_stops_at(slices: Vec<String>) -> String {
        let plans = census_plans(102_161, slices, Vec::new(), None);
        let total = plans.len();
        let (response, requests) = call_with_max_bytes(
            plans,
            json!({"company_guid":GUID,"fields":"compliance"}),
            20_000_000,
        )
        .await;
        assert_eq!(requests, total, "a request was sent past the refusal");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        error["cause"].as_str().unwrap().to_owned()
    }

    #[tokio::test]
    async fn a_slice_holding_more_ledgers_than_its_span_stops_the_census_there() {
        // 4,001 ledgers in a slice of 4,000 AlterIDs: the filter was ignored.
        let cause = census_stops_at(vec![census_slice(GUID, 0..4_001)]).await;
        assert_eq!(cause, "ledger_span_slice_over_bound");
    }

    #[tokio::test]
    async fn a_ledger_seen_in_two_slices_stops_the_census_at_the_second() {
        let cause = census_stops_at(vec![census_slice(GUID, 0..5), census_slice(GUID, 3..8)]).await;
        assert_eq!(cause, "ledger_span_duplicate_identity");
    }

    /// The whole read after a catalogue count compares the count with the
    /// master rows too (#679): nine ledgers counted, eight returned, and the
    /// read ends right after the master pair, before any balance is requested.
    #[tokio::test]
    async fn a_catalogue_count_that_differs_from_the_master_read_stops_the_read_after_its_master() {
        let nine = generated(&[(BIG, 9)]);
        let eight = generated(&[(BIG, 8)]);
        let plans = marked_compliance_plans(
            5_000,
            vec![
                generated_catalogue(&nine.iter().collect::<Vec<_>>()),
                generated_masters(&eight.iter().collect::<Vec<_>>()),
            ],
            None,
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "a request was sent past the master pair");
        assert_eq!(refusal(&response)["cause"], "ledger_count_differs");
    }

    /// The plans of a census that stops at the company's own ledger count: the
    /// slices and that one answer, and nothing after it.
    fn census_plans_to_the_count(
        mark: u64,
        slices: Vec<String>,
        company_count: String,
    ) -> Vec<ScenarioPlan> {
        let mut plans = marked_plans_over(extent_with_master_mark(mark), Vec::new(), None);
        plans.extend(slices.into_iter().map(xml));
        plans.push(xml(company_count));
        plans
    }

    /// Tally's own count of the company's ledgers is higher than the census
    /// counted (as would follow if a company closed and reopened during the
    /// census with equal marks answered the slices after that with the empty
    /// body, which is reasoned, not reproduced; or a ledger added during the
    /// read): the call is refused
    /// right after that one read, before the extent is read again and before any
    /// master or catalogue read could be sized from the low count (#938).
    #[tokio::test]
    async fn a_census_below_the_companys_own_ledger_count_is_refused_before_anything_is_sized() {
        let mark = 102_161_u64;
        // The census holds nine ledgers: ten is the smallest higher count.
        for company in ["10", "20"] {
            let plans = census_plans_to_the_count(
                mark,
                census_bodies(mark, GUID, &[(24, 0..9)]),
                company_count_body(Some(company)),
            );
            let total = plans.len();
            let (response, requests) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(requests, total, "a request was sent after the count read");
            let error = refusal(&response);
            assert_eq!(error["code"], "party_ledger_master_read_failed");
            assert_eq!(error["cause"], "ledger_count_company_differs");
            assert_eq!(
                error["remediation"],
                crate::agent::refusal_remediation("ledger_count_company_differs").unwrap()
            );
        }
    }

    /// The company-count read is accounted in the evidence even when it refuses
    /// the call: two refusals that differ only in the size of that answer differ
    /// in evidence bytes by exactly that difference.
    #[tokio::test]
    async fn a_refused_company_count_read_is_accounted_in_the_evidence() {
        let mark = 102_161_u64;
        let wire = |body: &str| 2 + 2 * body.encode_utf16().count() as u64;
        let mut bytes = Vec::new();
        let mut sizes = Vec::new();
        for answer in ["20", "2000000"] {
            let body = company_count_body(Some(answer));
            sizes.push(wire(&body));
            let plans =
                census_plans_to_the_count(mark, census_bodies(mark, GUID, &[(24, 0..9)]), body);
            let (response, _) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(refusal(&response)["cause"], "ledger_count_company_differs");
            bytes.push(
                response["structuredContent"]["evidence"]["bytes"]
                    .as_u64()
                    .unwrap(),
            );
        }
        assert_ne!(sizes[0], sizes[1]);
        assert_eq!(bytes[1] - bytes[0], sizes[1] - sizes[0]);
    }

    /// The same in a read that goes on: the company-count answer is in the
    /// successful read's evidence, so two reads that differ only in its size
    /// differ in evidence bytes by exactly that difference.
    #[tokio::test]
    async fn the_company_count_read_is_accounted_in_a_successful_reads_evidence() {
        let mark = 102_161_u64;
        let wire = |body: &str| 2 + 2 * body.encode_utf16().count() as u64;
        let mut bytes = Vec::new();
        let mut sizes = Vec::new();
        for answer in ["8", "000000000008"] {
            let body = company_count_body(Some(answer));
            sizes.push(wire(&body));
            let plans = census_plans_counted(
                mark,
                census_bodies(mark, GUID, &[(24, 0..9)]),
                body,
                extent_with_master_mark(mark),
                vec![masters(), balances(), groups()],
                Some(extent_with_master_mark(mark)),
            );
            let (response, _) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_ne!(response["isError"], true, "{response}");
            bytes.push(
                response["structuredContent"]["evidence"]["bytes"]
                    .as_u64()
                    .unwrap(),
            );
        }
        assert_ne!(sizes[0], sizes[1]);
        assert_eq!(bytes[1] - bytes[0], sizes[1] - sizes[0]);
    }

    /// A company-count answer past the response cap is not a damaged answer: it
    /// is refused under the read's own code with its own `cause` and remediation
    /// (#1033), never as `ledger_count_company_invalid` or
    /// `ledger_count_company_differs`, and nothing is sent after it.
    #[tokio::test]
    async fn a_transport_failure_on_the_company_count_read_is_not_an_invalid_answer() {
        let mark = 102_161_u64;
        let mut plans = marked_plans_over(extent_with_master_mark(mark), Vec::new(), None);
        plans.extend(
            census_bodies(mark, GUID, &[(24, 0..9)])
                .into_iter()
                .map(xml),
        );
        plans.push(xml(company_count_body(Some("9"))).with_framing(
            ResponseFraming::DeclaredContentLength {
                bytes: bridge_tally_transport::XML_RESPONSE_MAX_BYTES + 1,
            },
        ));
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "a request was sent after the count read");
        assert_eq!(response["isError"], true, "{response}");
        let error = refusal(&response);
        assert_eq!(
            error["code"], "party_ledger_master_read_failed",
            "{response}"
        );
        assert_eq!(
            error["cause"], "ledger_count_company_response_too_large",
            "{response}"
        );
        assert_eq!(
            error["remediation"],
            crate::agent::refusal_remediation("ledger_count_company_response_too_large").unwrap()
        );
    }

    /// Any other failure of the company-count read keeps what it had (#1144
    /// review): a reset connection still carries no `cause`, and an HTTP error
    /// keeps the transport's own. Only an answer past the cap is renamed, so
    /// neither reads as `ledger_count_company_response_too_large`, and nothing
    /// is sent after either.
    #[tokio::test]
    async fn a_count_read_failure_short_of_the_cap_keeps_its_own_cause() {
        for (failed, cause) in [
            (
                xml(company_count_body(Some("9")))
                    .with_delivery(tally_protocol_simulator::Delivery::ResetBeforeBody),
                None,
            ),
            (
                xml(company_count_body(Some("9"))).with_http_status(500),
                Some("http_status_failure"),
            ),
        ] {
            let mark = 102_161_u64;
            let mut plans = marked_plans_over(extent_with_master_mark(mark), Vec::new(), None);
            plans.extend(
                census_bodies(mark, GUID, &[(24, 0..9)])
                    .into_iter()
                    .map(xml),
            );
            plans.push(failed);
            let total = plans.len();
            let (response, requests) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(requests, total, "a request was sent after the count read");
            let error = refusal(&response);
            assert_eq!(
                error["code"], "party_ledger_master_read_failed",
                "{response}"
            );
            assert_eq!(
                error.get("cause").and_then(Value::as_str),
                cause,
                "{response}"
            );
        }
    }

    /// The cross-check flag is added to a result whatever the frame was, and a
    /// read with no census leaves the frame as it was.
    #[test]
    fn the_cross_check_flag_joins_an_object_frame_and_replaces_any_other() {
        use crate::tally::connection::CountCrossCheck;
        let flag = json!({"status": "matched"});
        let joined = count_cross_check_frame(json!({"rows": 3}), Some(CountCrossCheck::Matched));
        assert_eq!(joined["rows"], 3);
        assert_eq!(joined["ledger_count_cross_check"], flag);
        for other in [json!(null), json!([1, 2]), json!("text")] {
            let framed = count_cross_check_frame(other, Some(CountCrossCheck::Matched));
            assert_eq!(framed, json!({"ledger_count_cross_check": flag}));
        }
        let untouched = json!({"rows": 3});
        assert_eq!(count_cross_check_frame(untouched.clone(), None), untouched);
    }

    /// A count the company does not give, or gives in a form that is not a plain
    /// integer, or gives for another company, is refused or unavailable, never
    /// read as agreement.
    #[tokio::test]
    async fn an_unusable_company_count_answer_refuses_the_call_after_that_read() {
        let mark = 102_161_u64;
        let other = company_count_body(Some("9")).replace(GUID, LAB_COMPANY_GUID);
        assert_ne!(other, company_count_body(Some("9")));
        for bad in [
            company_count_body(Some("9,000")),
            company_count_body(Some("-1")),
            company_count_body(Some("")),
            other,
            company_count_body(Some("9")).replace("<STATUS>1</STATUS>", "<STATUS>0</STATUS>"),
        ] {
            let plans =
                census_plans_to_the_count(mark, census_bodies(mark, GUID, &[(24, 0..9)]), bad);
            let total = plans.len();
            let (response, requests) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(requests, total, "a request was sent after the count read");
            assert_eq!(response["isError"], true, "{response}");
            assert_eq!(refusal(&response)["cause"], "ledger_count_company_invalid");
        }
    }

    /// Whether the check ran is in the result: `matched` when Tally's count equals
    /// the census's, `company_count_lower` when it is below (the read goes on),
    /// `unavailable` when the answer carries no count; and nothing at all when no
    /// census ran.
    #[tokio::test]
    async fn the_result_says_whether_the_census_count_was_cross_checked() {
        let mark = 102_161_u64;
        for (answer, status) in [
            (Some("9"), "matched"),
            (Some("8"), "company_count_lower"),
            (None, "unavailable"),
        ] {
            let plans = census_plans_counted(
                mark,
                census_bodies(mark, GUID, &[(24, 0..9)]),
                company_count_body(answer),
                extent_with_master_mark(mark),
                vec![masters(), balances(), groups()],
                Some(extent_with_master_mark(mark)),
            );
            let total = plans.len();
            let (response, requests) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(requests, total, "{status}");
            assert_ne!(response["isError"], true, "{status}: {response}");
            assert_eq!(
                response["structuredContent"]["result"]["ledger_count_cross_check"]["status"],
                status
            );
        }
        // No census (a whole read by the mark alone): no such field.
        let (unsized_response, _) = call(
            compliance_plans(masters(), balances()),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        assert!(
            unsized_response["structuredContent"]["result"]
                .get("ledger_count_cross_check")
                .is_none(),
            "{unsized_response}"
        );
    }

    /// A company closed, reopened or switched during the census answers the
    /// slices after that moment with the same empty body as a slice past every
    /// ledger, so the count can be low, and the count sizes the next read. The
    /// book's extent is read again after the census and before that read is
    /// admitted (#679): the company switching after the first, a middle or the
    /// last slice that held ledgers refuses the call, and nothing is sent after
    /// that extent (the plans end there; a request past them fails the count).
    /// The refusal comes from the extent alone whatever the slices held; the
    /// three positions keep a partial count (5, 10 and 15 ledgers against the
    /// nine master rows) from being read as a match. A company closed and
    /// reopened with equal marks is not caught by the extent. Tally's own count
    /// of the ledgers (#938) is meant to catch it (by reasoning, no live
    /// reproduction), before the extent is read again.
    #[tokio::test]
    async fn a_book_that_changes_during_the_census_is_refused_before_the_next_read_is_sized() {
        let mark = 102_161_u64;
        // Ledgers up to the slice of the switch, empty answers after it.
        for switched_after in [0_usize, 12, 24] {
            let filled: Vec<(usize, std::ops::Range<u32>)> = (0..=switched_after)
                .step_by(12)
                .enumerate()
                .map(|(position, slice)| {
                    let start = position as u32 * 5;
                    (slice, start..start + 5)
                })
                .collect();
            let slices = census_bodies(mark, GUID, &filled);
            // The master mark moved: the book was edited, or another was opened.
            let plans = census_plans_after(
                mark,
                slices.clone(),
                extent_with_master_mark(mark + 1),
                Vec::new(),
                None,
            );
            let total = plans.len();
            let (response, requests) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(
                requests, total,
                "a read was sent after the switch at slice {switched_after}"
            );
            assert_eq!(
                refusal(&response)["cause"],
                "party_ledger_extent_changed",
                "switch at slice {switched_after}"
            );
            // Another company's extent answers the same request: still nothing more.
            let other = extent_with_master_mark(mark).replace(GUID, LAB_COMPANY_GUID);
            assert_ne!(other, extent_with_master_mark(mark));
            let plans = census_plans_after(mark, slices, other, Vec::new(), None);
            let total = plans.len();
            let (response, requests) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(
                requests, total,
                "a read was sent after the company switched at slice {switched_after}"
            );
            assert_eq!(response["isError"], true, "{response}");
        }
    }

    /// A census that refuses still accounts for every slice it sent, the
    /// refused one included: two refusals that differ only in the size of the
    /// last slice differ in evidence bytes by exactly that difference.
    #[tokio::test]
    async fn a_refused_census_accounts_the_slices_it_sent() {
        let wire = |body: &str| 2 + 2 * body.encode_utf16().count() as u64;
        let mut bytes = Vec::new();
        let mut sizes = Vec::new();
        for last in [3..8_u32, 3..9] {
            let slices = vec![census_slice(GUID, 0..5), census_slice(GUID, last)];
            sizes.push(slices.iter().map(|body| wire(body)).sum::<u64>());
            let plans = census_plans(102_161, slices, Vec::new(), None);
            let (response, _) =
                call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
            assert_eq!(
                refusal(&response)["cause"],
                "ledger_span_duplicate_identity"
            );
            bytes.push(
                response["structuredContent"]["evidence"]["bytes"]
                    .as_u64()
                    .unwrap(),
            );
        }
        assert_ne!(sizes[0], sizes[1]);
        assert_eq!(bytes[1] - bytes[0], sizes[1] - sizes[0]);
        assert!(bytes[0] >= sizes[0], "both slices are accounted");
    }

    #[tokio::test]
    async fn a_ledger_seen_twice_within_one_slice_stops_the_census_there() {
        let twice = census_slice(GUID, 0..3);
        let second = format!("-{:08x}", 0x2000_0000 + 1);
        let first = format!("-{:08x}", 0x2000_0000);
        assert_eq!(twice.matches(&second).count(), 1);
        let cause = census_stops_at(vec![twice.replace(&second, &first)]).await;
        assert_eq!(cause, "ledger_span_duplicate_identity");
    }

    #[tokio::test]
    async fn a_failed_slice_is_refused_as_malformed_not_counted_as_empty() {
        let failed = census_empty().replace("<STATUS>1</STATUS>", "<STATUS>0</STATUS>");
        assert_ne!(failed, census_empty());
        let cause = census_stops_at(vec![failed]).await;
        assert_eq!(cause, "ledger_span_slice_malformed");
    }

    #[tokio::test]
    async fn a_slice_of_another_company_stops_the_census_there() {
        let other = "00000000-0000-0000-0000-000000000000";
        let cause = census_stops_at(vec![census_empty(), census_slice(other, 0..3)]).await;
        assert_eq!(cause, "ledger_span_identity_mismatch");
    }

    #[tokio::test]
    async fn a_slice_past_the_response_cap_is_refused_under_its_own_cause() {
        let mark = 102_161_u64;
        let mut plans = census_plans(mark, Vec::new(), Vec::new(), None);
        plans.push(
            xml(census_empty()).with_framing(ResponseFraming::DeclaredContentLength {
                bytes: bridge_tally_transport::XML_RESPONSE_MAX_BYTES + 1,
            }),
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(
            requests, total,
            "a request was sent past the oversized answer"
        );
        let error = refusal(&response);
        assert_eq!(error["cause"], "ledger_span_slice_response_too_large");
        assert_eq!(
            error["remediation"],
            crate::agent::refusal_remediation("ledger_span_slice_response_too_large").unwrap()
        );
    }

    /// A count past one read is read in parts, the parents named by the
    /// catalogue: the census's slices, then the catalogue pair, then the parts,
    /// and the two counts agree (#679).
    #[tokio::test]
    async fn a_census_count_past_one_read_is_read_in_parts_after_the_catalogue() {
        let mark = 30_000_u64;
        let rows = split_book();
        let mut plans = census_plans_checked(
            mark,
            census_bodies(mark, GUID, &[(0, 0..4_000), (1, 4_000..4_300)]),
            vec![generated_catalogue(&rows.iter().collect::<Vec<_>>())],
            None,
        );
        for source in [
            part_reads(&under(&rows, &[BIG, NESTED])).0,
            part_reads(&under(&rows, &[BIG, NESTED])).1,
            part_reads(&under(&rows, &[OTHER])).0,
            part_reads(&under(&rows, &[OTHER])).1,
            groups(),
            extent_with_master_mark(mark),
        ] {
            pair(&mut plans, xml(source));
        }
        plans.extend([xml(companies()), status(), xml(companies())]);
        let total = plans.len();
        let (response, requests) = call_with_max_bytes(
            plans,
            json!({"company_guid":GUID,"fields":"compliance"}),
            2_000_000,
        )
        .await;
        assert_eq!(requests, total);
        assert_ne!(response["isError"], true, "{response}");
        assert_eq!(response["structuredContent"]["result"]["total"], 4_300);
    }

    /// The census and the catalogue count one book twice; a difference stops
    /// the read right after the catalogue, before any master is requested.
    #[tokio::test]
    async fn a_census_and_a_catalogue_that_count_differently_stop_the_read_after_the_catalogue() {
        let mark = 30_000_u64;
        let rows = split_book();
        let plans = census_plans_checked(
            mark,
            census_bodies(mark, GUID, &[(0, 0..4_000), (1, 4_000..4_299)]),
            vec![generated_catalogue(&rows.iter().collect::<Vec<_>>())],
            None,
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total, "nothing is sent after the catalogue pair");
        assert_eq!(refusal(&response)["cause"], "ledger_count_differs");
    }

    /// A count whose catalogue would not fit the response limit is refused
    /// right after the census, before the catalogue is requested.
    #[tokio::test]
    async fn a_census_count_whose_catalogue_cannot_fit_is_refused_before_the_catalogue() {
        let mark = 30_000_u64;
        // 22,858 ledgers, one more than the catalogue limit admits.
        let filled = [
            (0, 0..4_000),
            (1, 4_000..8_000),
            (2, 8_000..12_000),
            (3, 12_000..16_000),
            (4, 16_000..20_000),
            (5, 20_000..22_858),
        ];
        let plans =
            census_plans_checked(mark, census_bodies(mark, GUID, &filled), Vec::new(), None);
        let total = plans.len();
        let (response, requests) = call_with_max_bytes(
            plans,
            json!({"company_guid":GUID,"fields":"compliance"}),
            20_000_000,
        )
        .await;
        assert_eq!(requests, total, "nothing is sent after the census");
        assert_eq!(
            refusal(&response)["cause"],
            "ledger_count_catalogue_too_large"
        );
    }

    /// A mark exactly at the bound is admitted and read as it was before #637:
    /// the same three reads in the same order, and the same rows.
    #[tokio::test]
    async fn a_book_whose_master_mark_is_at_the_bound_reads_as_before() {
        let mark = 16_000_000 / 3_750;
        let plans = marked_compliance_plans(
            mark,
            vec![masters(), balances(), groups()],
            Some(extent_with_master_mark(mark)),
        );
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        assert_eq!(requests, total);
        let (unsized_response, _) = call(
            compliance_plans(masters(), balances()),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        assert_eq!(items(&response), items(&unsized_response));
    }

    /// The whole successful `fields=basic` sequence: identity, then the runtime's boundary
    /// probe, the extent-bracketed BOOKSFROM-pinned ledger export and the closing checks.
    fn basic_plans() -> Vec<ScenarioPlan> {
        basic_plans_reading(period_opening(), None)
    }

    fn period_opening() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-period-opening.utf16le.xml"
        ))
    }

    /// The captured currency read of a book with one master (INR).
    fn single_currency() -> String {
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        ))
    }

    /// A basic read of a book with several Currency masters is refused after
    /// its currency read and before any ledger request: a bare opening names
    /// no currency, so a dollar ledger would read as rupees (#714).
    #[tokio::test]
    async fn a_basic_read_of_a_several_currency_book_is_refused_before_any_ledger() {
        let forex = "b14e9b2d-8a63-4779-804d-25d59eb787eb";
        let companies = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
        )));
        let extent = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
        )));
        let mut plans = Vec::new();
        pair(&mut plans, companies.clone());
        plans.extend([status(), companies.clone(), companies]);
        pair(&mut plans, extent);
        pair(
            &mut plans,
            xml(captured(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
            ))),
        );
        let total = plans.len();
        let (response, requests) = call(plans, json!({"company_guid":forex})).await;
        assert_eq!(requests, total, "no ledger request was sent");
        let error = refusal(&response);
        assert_eq!(error["code"], "ledger_export_invalid");
        assert_eq!(error["cause"], "company_several_currency_masters");
        let remediation = error["remediation"].as_str().unwrap();
        assert!(remediation.contains("#551"), "{error}");
    }

    /// A basic read whose currency collection holds no master is refused
    /// after it, before any ledger request: one master is not established.
    /// DERIVED from the captured single-master response with its one
    /// `CURRENCY` element removed (#714).
    #[tokio::test]
    async fn a_basic_read_with_no_currency_master_is_refused_before_any_ledger() {
        let captured_currency = single_currency();
        let start = captured_currency.find("<CURRENCY ").unwrap();
        let end =
            start + captured_currency[start..].find("</CURRENCY>").unwrap() + "</CURRENCY>".len();
        let mut none = captured_currency.clone();
        none.replace_range(start..end, "");
        assert!(!none.contains("<CURRENCY "), "no master left");
        let company = xml(companies());
        let extent = xml(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )
        .to_owned());
        let mut plans = identity_plans();
        plans.extend([
            status(),
            company.clone(),
            company,
            extent.clone(),
            status(),
            extent,
            status(),
        ]);
        pair(&mut plans, xml(none));
        let total = plans.len();
        let (response, requests) = call(plans, json!({"company_guid":GUID})).await;
        assert_eq!(requests, total, "no ledger request was sent");
        let error = refusal(&response);
        assert_eq!(error["code"], "ledger_export_invalid");
        assert_eq!(error["cause"], "company_currency_probe_failed");
    }

    /// A basic read whose one Currency master is not INR is refused after
    /// the currency read, before any ledger request: its bare openings would
    /// name no currency (#716). DERIVED from the captured single-master
    /// response with its `MAILINGNAME` changed from `INR`; no non-INR book
    /// has been captured.
    #[tokio::test]
    async fn a_basic_read_of_a_non_inr_book_is_refused_before_any_ledger() {
        let captured_currency = single_currency();
        let inr = "<MAILINGNAME TYPE=\"String\">INR</MAILINGNAME>";
        assert_eq!(captured_currency.matches(inr).count(), 1);
        let foreign =
            captured_currency.replace(inr, "<MAILINGNAME TYPE=\"String\">UAE Dirham</MAILINGNAME>");
        let company = xml(companies());
        let extent = xml(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )
        .to_owned());
        let mut plans = identity_plans();
        plans.extend([
            status(),
            company.clone(),
            company,
            extent.clone(),
            status(),
            extent,
            status(),
        ]);
        pair(&mut plans, xml(foreign));
        let total = plans.len();
        let (response, requests) = call(plans, json!({"company_guid":GUID})).await;
        assert_eq!(requests, total, "no ledger request was sent");
        let error = refusal(&response);
        assert_eq!(error["code"], "ledger_export_invalid");
        assert_eq!(error["cause"], "company_base_currency_not_inr");
    }

    /// A ledger export refused as a whole names why (#718): Tally's failure
    /// answer, no answer, a response cut off before its root closed, and one
    /// whose ledgers belong to another company each carry their own cause
    /// beside `ledger_export_invalid`. Each is the captured basic export with
    /// one change made here; before #718 all four carried no cause.
    #[tokio::test]
    async fn a_ledger_export_refused_as_a_whole_names_its_cause() {
        let captured = period_opening();
        let company_prefix = format!("{GUID}-");
        assert!(captured.contains(&company_prefix));
        for (body, cause) in [
            (
                captured.replacen("<STATUS>1</STATUS>", "<STATUS>0</STATUS>", 1),
                "native_collection_not_success",
            ),
            (
                captured.replacen("<STATUS>1</STATUS>", "<STATUS></STATUS>", 1),
                "native_collection_status_absent",
            ),
            (
                captured.replacen("</ENVELOPE>", "", 1),
                "native_collection_malformed_response",
            ),
            (
                captured.replace(&company_prefix, "00000000-0000-4000-8000-0000000000aa-"),
                "native_collection_identity_mismatch",
            ),
        ] {
            assert_ne!(body, captured, "{cause}");
            let (response, _) = call(
                basic_plans_reading(body, None),
                json!({"company_guid":GUID}),
            )
            .await;
            let error = refusal(&response);
            assert_eq!(error["code"], "ledger_export_invalid", "{response}");
            assert_eq!(error["cause"], cause, "{response}");
        }
    }

    /// A ledger export refused for one row names whose fault it was (#718,
    /// slice 2): a row repeating a field is `native_collection_row_unusable`, and
    /// a response that ends inside a row is `native_collection_malformed_response`.
    /// Each is the captured basic export with one change made here; before
    /// slice 2 both carried no cause.
    #[tokio::test]
    async fn a_ledger_export_refused_for_one_row_names_its_cause() {
        let captured = period_opening();
        let parent = r#"<PARENT TYPE="String">Bridge Nested Debtors WR4</PARENT>"#;
        assert_eq!(captured.matches(parent).count(), 1);
        let opening = captured.find("<OPENINGBALANCE").expect("a row's opening");
        for (body, cause) in [
            (
                captured.replacen(parent, &format!("{parent}{parent}"), 1),
                "native_collection_row_unusable",
            ),
            (
                captured[..opening].to_owned(),
                "native_collection_malformed_response",
            ),
        ] {
            assert_ne!(body, captured, "{cause}");
            let (response, _) = call(
                basic_plans_reading(body, None),
                json!({"company_guid":GUID}),
            )
            .await;
            let error = refusal(&response);
            assert_eq!(error["code"], "ledger_export_invalid", "{response}");
            assert_eq!(error["cause"], cause, "{response}");
        }
    }

    /// A `ledger_masters` catalogue export that outlives its deadline is sent
    /// once, as the movement catalogue is (#937): a 2 s deadline, a 5 s stall,
    /// and a wait past both. A whole second read is held behind the stalled
    /// export, so a retry would find plans to answer it and `received` would
    /// count them. Returns the response and how many requests were received.
    async fn ledger_masters_with_a_stalled_export(
        args: Value,
        groups: Option<String>,
    ) -> (Value, usize) {
        const EXPORT: usize = 15;
        let busy = std::time::Duration::from_millis(5_000);
        let captured = period_opening();
        let mut plans = basic_plans_reading(captured.clone(), groups.clone());
        plans[EXPORT] = plans[EXPORT]
            .clone()
            .with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(busy));
        let stalled = EXPORT + 1;
        plans.truncate(stalled);
        // Everything after the identity read: what a retried read sends next.
        plans.extend(basic_plans_reading(captured, groups).into_iter().skip(4));
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let mut server = Server::new(Settings {
            endpoint: TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            data_dir: directory.path().into(),
            max_rows: 500,
            max_bytes: 200_000,
            redaction: Redaction::None,
            import_enabled: false,
            posting: Posting::Off,
            master_writes_enabled: false,
            audit_enabled: false,
        });
        server.runtime = crate::tally::runtime::TallyRuntime::with_transport_policy(
            bridge_tally_transport::TransportPolicy {
                request_timeout: std::time::Duration::from_millis(2_000),
                ..Default::default()
            },
        );
        let response = server.call_tool("ledger_masters", args).await;
        // Past the stalled response and any retry that would have queued behind it.
        tokio::time::sleep(busy + std::time::Duration::from_millis(3_000)).await;
        let received = simulator.received();
        simulator.cancel();
        assert_eq!(
            received, stalled,
            "nothing was sent after the export that timed out: {response}"
        );
        (response, received)
    }

    #[tokio::test]
    async fn a_basic_ledger_masters_catalogue_that_times_out_is_sent_once() {
        let (response, _) =
            ledger_masters_with_a_stalled_export(json!({"company_guid":GUID}), None).await;
        let error = refusal(&response);
        assert_eq!(error["code"], "ledger_export_invalid", "{response}");
        assert_eq!(error["cause"], "request_deadline_exceeded", "{response}");
    }

    #[tokio::test]
    async fn a_group_filtered_ledger_masters_catalogue_that_times_out_is_sent_once() {
        let (response, _) = ledger_masters_with_a_stalled_export(
            json!({"company_guid":GUID, "group":"Sundry Debtors"}),
            Some(groups()),
        )
        .await;
        let error = refusal(&response);
        assert_eq!(error["code"], "ledger_export_invalid", "{response}");
        assert_eq!(error["cause"], "request_deadline_exceeded", "{response}");
    }

    /// As `basic_plans`, with the ledger export given and, when `groups` is
    /// supplied, the paired group collection a `group` filter adds inside the
    /// same extent and identity bracket.
    fn basic_plans_reading(ledgers: String, groups: Option<String>) -> Vec<ScenarioPlan> {
        let company = xml(companies());
        let extent = xml(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )
        .to_owned());
        let mut plans = identity_plans();
        plans.extend([
            status(),
            company.clone(),
            company.clone(),
            extent.clone(),
            status(),
            extent.clone(),
            status(),
        ]);
        // The basic read proves the book keeps one Currency master (#714).
        pair(&mut plans, xml(single_currency()));
        pair(&mut plans, xml(ledgers));
        if let Some(groups) = groups {
            pair(&mut plans, xml(groups));
        }
        plans.extend([
            extent.clone(),
            status(),
            extent,
            status(),
            company.clone(),
            status(),
            company,
        ]);
        plans
    }

    /// The BOOKSFROM the captured extent admits for this company; both ledger_masters requests
    /// pin SVFROMDATE to it (see `ledger_movement_opening_export_is_pinned_to_admitted_books_from`).
    const ADMITTED_BOOKS_FROM: &str = "20260401";

    #[tokio::test]
    async fn basic_ledger_masters_rows_carry_their_opening_balance_as_of() {
        // Without a `group` filter the group collection is never read, whatever
        // the scope: the whole replayed sequence is the ledger export's, and
        // the result carries no filter report.
        for args in [
            json!({"company_guid":GUID,"fields":"basic"}),
            json!({"company_guid":GUID,"fields":"basic","group_scope":"ancestry"}),
        ] {
            let plans = basic_plans();
            let total = plans.len();
            let (response, requests) = call(plans, args.clone()).await;
            let rows = items(&response);
            assert_eq!(requests, total, "{args}");
            assert!(!rows.is_empty());
            for row in rows {
                assert_eq!(row["opening_balance_as_of"], ADMITTED_BOOKS_FROM, "{row}");
            }
            assert!(
                response["structuredContent"]["result"]
                    .get("group_filter")
                    .is_none(),
                "{response}"
            );
        }
    }

    #[tokio::test]
    async fn compliance_ledger_masters_rows_carry_their_opening_balance_as_of() {
        let (response, _) = call(
            compliance_plans(masters(), balances()),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        let rows = items(&response);
        assert!(!rows.is_empty());
        for row in rows {
            assert_eq!(row["opening_balance_as_of"], ADMITTED_BOOKS_FROM, "{row}");
            // This capture predates the registration-history FETCH, so every
            // row falls back to the flat field, and says so (bridge#624).
            let expected = if row["party_gstin"].is_null() {
                "not_reported"
            } else {
                "flat_field"
            };
            assert_eq!(row["party_gstin_status"], expected, "{row}");
            let flat = row["party_gstin_flat"]
                .as_str()
                .filter(|flat| !flat.is_empty());
            assert_eq!(flat, row["party_gstin"].as_str(), "{row}");
            assert_eq!(row["gstin_sources_disagree"], false, "{row}");
            assert!(row["party_gstin_registration_type"].is_null(), "{row}");
            assert_eq!(row["party_gstin_as_of"], tally_host_today(), "{row}");
            assert_eq!(
                row["compliance"]["gst_registrations"]["observation"], "not_observed",
                "{row}"
            );
        }
    }

    /// bridge#1520: the `compliance` object of every row is the bytes it was
    /// before `gst_details` existed (#1518). The golden is the tool's own rows
    /// on this capture, produced by the tree at `90bcaa67^`; its provenance
    /// note gives the command.
    #[tokio::test]
    async fn compliance_rows_are_byte_for_byte_the_ones_read_before_gst_details() {
        let (response, _) = call(
            compliance_plans(masters(), balances()),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        let compliance = items(&response)
            .iter()
            .map(|row| &row["compliance"])
            .collect::<Vec<_>>();
        assert!(!compliance.is_empty());
        assert!(compliance.iter().all(|value| value.is_object()));
        let actual = serde_json::to_string(&compliance).unwrap();
        let golden = include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/ledger_masters_compliance_rows_before_gst_details.json"
        );
        let first_difference = actual
            .bytes()
            .zip(golden.bytes())
            .position(|(left, right)| left != right)
            .unwrap_or(actual.len().min(golden.len()));
        assert!(
            actual == golden,
            "{} rows differ from the golden ({} bytes, golden {}) from byte {first_difference}",
            compliance.len(),
            actual.len(),
            golden.len()
        );
    }

    /// bridge#1520: no `fields=compliance` row names `gst_details`, anywhere,
    /// until a caller asks for the GST details read.
    #[tokio::test]
    async fn compliance_rows_carry_no_gst_details_key() {
        let (response, _) = call(
            compliance_plans(masters(), balances()),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        let rows = items(&response);
        assert!(!rows.is_empty());
        for row in rows {
            assert!(
                !row["compliance"]
                    .as_object()
                    .unwrap()
                    .contains_key("gst_details"),
                "{row}"
            );
        }
        assert!(!response.to_string().contains("gst_details"));
    }

    /// bridge#1490, over the live answer to the request this tool sends when asked for them: each
    /// row's `compliance.mailing_details` carries the dated mailing entry the
    /// flat address fields leave empty. The probe party's entry is the one
    /// measured; a ledger with no address is an observed empty list.
    #[tokio::test]
    async fn compliance_rows_carry_the_dated_mailing_details() {
        let masters = mailing_masters();
        let balances = balances_matching(&masters);
        let (response, _) = call(
            compliance_plans(masters, balances),
            json!({"company_guid":GUID,"fields":"compliance","include_mailing_details":true}),
        )
        .await;
        let rows = items(&response);
        assert_eq!(rows.len(), 8);
        assert_eq!(
            row(rows, "Lab Party Duplicate Probe")["compliance"]["mailing_details"],
            json!({
                "observation": "entries",
                "entries": [{
                    "applicable_from": "20260401",
                    "mailing_name": "Lab Party Duplicate Probe",
                    "address_lines": ["1 Test Street", "Test Area"],
                    "state": "Karnataka",
                    "country": "India",
                    "pin_code": "560001",
                    "other_content_skipped": false
                }]
            })
        );
        for other in rows
            .iter()
            .filter(|r| r["name"] != "Lab Party Duplicate Probe")
        {
            assert_eq!(
                other["compliance"]["mailing_details"],
                json!({"observation": "entries", "entries": []}),
                "{other}"
            );
        }
    }

    /// bridge#1490: a mailing entry's `mailing_name` is the party's name, so
    /// `mask_parties` masks it as it masks the row's own `name`; nothing else of
    /// the entry is a party name.
    #[test]
    fn the_mailing_name_is_masked_like_the_party_name_it_is() {
        let name = "Lab Party Duplicate Probe";
        let fields = bridge_tally_protocol::PartyLedgerMasterFields {
            mailing_details:
                bridge_tally_protocol::mailing_details::MailingDetailsHistory::Entries {
                    entries: vec![
                        bridge_tally_protocol::mailing_details::MailingDetailsEntry {
                            applicable_from: bridge_tally_core::TallyDate::parse("20260401")
                                .unwrap(),
                            mailing_name: Some(name.to_string()),
                            address_lines: vec!["1 Test Street".to_string()],
                            state: Some("Karnataka".to_string()),
                            country: Some("India".to_string()),
                            pin_code: Some("560001".to_string()),
                            other_content_skipped: false,
                        },
                    ],
                },
            ..bridge_tally_protocol::PartyLedgerMasterFields::default()
        };
        let compliance = compliance_json(fields, false, true);
        let entry = |value: &Value| value["mailing_details"]["entries"][0].clone();
        assert_eq!(
            entry(&compliance)["mailing_name"],
            json!({crate::agent::PARTY_NAME_MARKER: name})
        );
        let masked =
            crate::agent::redact_value(compliance.clone(), crate::agent::Redaction::MaskParties);
        assert_eq!(
            entry(&masked)["mailing_name"],
            json!(crate::agent::mask(name))
        );
        assert_ne!(entry(&masked)["mailing_name"], json!(name));
        assert_eq!(entry(&masked)["pin_code"], json!("560001"));
        let unmasked = crate::agent::redact_value(compliance, crate::agent::Redaction::None);
        assert_eq!(entry(&unmasked)["mailing_name"], json!(name));
    }

    /// bridge#1490: every mailing address line and the flat address are masked
    /// under `mask_parties`, because a line can repeat the party's name. An empty
    /// address stays empty and an address not observed stays null; the PIN code,
    /// state and country stay readable; `none` returns every value as read.
    #[test]
    fn the_address_is_masked_line_by_line_and_empty_stays_empty() {
        let lines = ["1 Test Street", "Test Area"];
        let entry = |address_lines: Vec<String>| {
            bridge_tally_protocol::mailing_details::MailingDetailsEntry {
                applicable_from: bridge_tally_core::TallyDate::parse("20260401").unwrap(),
                mailing_name: None,
                address_lines,
                state: Some("Karnataka".to_string()),
                country: Some("India".to_string()),
                pin_code: Some("560001".to_string()),
                other_content_skipped: false,
            }
        };
        let fields = |address: bridge_tally_protocol::PartyLedgerMasterFieldObservation,
                      address_lines: Vec<String>| {
            bridge_tally_protocol::PartyLedgerMasterFields {
                address,
                mailing_details:
                    bridge_tally_protocol::mailing_details::MailingDetailsHistory::Entries {
                        entries: vec![entry(address_lines)],
                    },
                ..bridge_tally_protocol::PartyLedgerMasterFields::default()
            }
        };
        let returned = |text: &str| {
            bridge_tally_protocol::PartyLedgerMasterFieldObservation::Returned(text.to_string())
        };
        let masked = |fields| {
            crate::agent::redact_value(
                compliance_json(fields, false, true),
                crate::agent::Redaction::MaskParties,
            )
        };

        let row = masked(fields(
            returned("1 Test Street, Test Area"),
            lines.iter().map(|line| line.to_string()).collect(),
        ));
        assert_eq!(
            row["address"],
            json!(crate::agent::mask("1 Test Street, Test Area"))
        );
        let masked_lines = &row["mailing_details"]["entries"][0]["address_lines"];
        assert_eq!(
            masked_lines,
            &json!([crate::agent::mask(lines[0]), crate::agent::mask(lines[1])])
        );
        let entry_json = &row["mailing_details"]["entries"][0];
        assert_eq!(entry_json["pin_code"], json!("560001"));
        assert_eq!(entry_json["state"], json!("Karnataka"));
        assert_eq!(entry_json["country"], json!("India"));
        assert!(entry_json.get("mailing_name").is_none());

        let empty = masked(fields(returned(""), vec![String::new()]));
        assert_eq!(empty["address"], json!(""));
        assert_eq!(
            empty["mailing_details"]["entries"][0]["address_lines"],
            json!([""])
        );
        let absent = masked(fields(
            bridge_tally_protocol::PartyLedgerMasterFieldObservation::NotObserved,
            Vec::new(),
        ));
        assert_eq!(absent["address"], Value::Null);

        let unmasked = crate::agent::redact_value(
            compliance_json(
                fields(
                    returned("1 Test Street, Test Area"),
                    lines.iter().map(|line| line.to_string()).collect(),
                ),
                false,
                true,
            ),
            crate::agent::Redaction::None,
        );
        assert_eq!(unmasked["address"], json!("1 Test Street, Test Area"));
        assert_eq!(
            unmasked["mailing_details"]["entries"][0]["address_lines"],
            json!(lines)
        );
    }

    /// bridge#653: `as_of` sets the date every row's `party_gstin` is read as
    /// of, in either spelling. Which entry is in force on a date is pinned over
    /// the live registration-history capture by
    /// `a_gstin_held_only_in_the_dated_registration_history_is_reported_in_force`.
    #[tokio::test]
    async fn compliance_rows_read_their_gstin_as_of_the_date_given() {
        for as_of in ["20260331", "2026-03-31"] {
            let (response, _) = call(
                compliance_plans(masters(), balances()),
                json!({"company_guid":GUID,"fields":"compliance","as_of":as_of}),
            )
            .await;
            let rows = items(&response);
            assert!(!rows.is_empty());
            for row in rows {
                assert_eq!(row["party_gstin_as_of"], "20260331", "{row}");
                assert_eq!(row["opening_balance_as_of"], ADMITTED_BOOKS_FROM, "{row}");
            }
        }
    }

    /// `as_of` selects only the GSTIN, so a basic read refuses it before any
    /// request rather than returning rows a caller could take as dated by it.
    #[tokio::test]
    async fn as_of_without_compliance_fields_is_refused_before_any_request() {
        for args in [
            json!({"company_guid":GUID,"as_of":"20260331"}),
            json!({"company_guid":GUID,"fields":"basic","as_of":"20260331"}),
        ] {
            let (response, requests) = call(Vec::new(), args.clone()).await;
            assert_eq!(requests, 0, "{args}");
            let error = refusal(&response);
            assert_eq!(
                error["code"], "ledger_masters_as_of_requires_compliance",
                "{args}"
            );
            assert!(error["remediation"]
                .as_str()
                .is_some_and(|text| text.contains("fields=compliance")));
        }
    }

    #[tokio::test]
    async fn an_impossible_as_of_date_is_refused_before_any_request() {
        let (response, requests) = call(
            Vec::new(),
            json!({"company_guid":GUID,"fields":"compliance","as_of":"20260231"}),
        )
        .await;
        assert_eq!(requests, 0);
        assert_eq!(refusal(&response)["code"], "invalid_date", "{response}");
    }

    // -- #630: one read per logical listing ---------------------------------

    /// A continuation page's requests: the paired company identity read every
    /// call starts with, then the bracketed, paired extent read.
    fn continuation_plans(extent: String) -> Vec<ScenarioPlan> {
        let company = xml(companies());
        let mut plans = identity_plans();
        plans.push(company.clone());
        pair(&mut plans, xml(extent));
        plans.push(company);
        plans
    }

    /// A `fields=basic` first page on a book whose master mark is `mark`.
    fn basic_plans_marked(mark: u64) -> Vec<ScenarioPlan> {
        let captured_extent = include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        );
        let marked = extent_with_master_mark(mark);
        basic_plans()
            .into_iter()
            .map(|plan| {
                if plan.fixture.body() == captured_extent {
                    xml(marked.clone())
                } else {
                    plan
                }
            })
            .collect()
    }

    /// One server over one replayed sequence, so a later call can be served
    /// from what an earlier call held.
    struct OneServer {
        simulator: SequenceSimulator,
        server: Server,
        _directory: tempfile::TempDir,
    }

    impl OneServer {
        fn spawn(plans: Vec<ScenarioPlan>) -> Self {
            let simulator = SequenceSimulator::spawn(plans).unwrap();
            let directory = tempfile::tempdir().unwrap();
            let server = Server::new(Settings {
                endpoint: TallyEndpointConfig {
                    host: "127.0.0.1".into(),
                    port: simulator.address().port(),
                },
                data_dir: directory.path().into(),
                max_rows: 500,
                max_bytes: 200_000,
                redaction: Redaction::None,
                import_enabled: false,
                posting: Posting::Off,
                master_writes_enabled: false,
                audit_enabled: false,
            });
            Self {
                simulator,
                server,
                _directory: directory,
            }
        }

        async fn call(&self, args: Value) -> Value {
            self.server.call_tool("ledger_masters", args).await
        }

        fn requests(self) -> usize {
            self.simulator.finish().unwrap().len()
        }
    }

    fn snapshot_of(response: &Value) -> &Value {
        assert_ne!(response["isError"], true, "{response}");
        &response["structuredContent"]["result"]["snapshot"]
    }

    fn snapshot_id(response: &Value) -> String {
        snapshot_of(response)["id"].as_str().unwrap().to_string()
    }

    /// The rows a whole, unpaged basic listing returns, for comparing pages.
    async fn whole_listing() -> Vec<Value> {
        let (response, _) = call(basic_plans(), json!({"company_guid":GUID})).await;
        items(&response).clone()
    }

    /// Page 2 costs the identity read and one extent read, and returns the
    /// rows that follow page 1 in the same read.
    #[tokio::test]
    async fn a_continuation_page_is_served_from_its_first_pages_read() {
        let mut plans = basic_plans();
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let first = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let id = snapshot_id(&first);
        assert_eq!(snapshot_of(&first)["reused"], false);
        assert_eq!(snapshot_of(&first)["master_alter_id"], 219);
        let second = one
            .call(json!({"company_guid":GUID,"offset":4,"limit":4,"snapshot_id":id}))
            .await;
        assert_eq!(snapshot_of(&second)["reused"], true);
        assert_eq!(snapshot_of(&second)["id"], id);
        assert_eq!(one.requests(), total);
        let whole = whole_listing().await;
        assert_eq!(items(&first).as_slice(), &whole[..4]);
        assert_eq!(items(&second).as_slice(), &whole[4..8]);
    }

    /// A book that moved after page 1 is refused when the caller named the
    /// snapshot, and read fresh when it did not.
    #[tokio::test]
    async fn a_continuation_after_the_book_moved_is_refused_by_id_or_read_fresh() {
        let mut plans = basic_plans();
        plans.extend(continuation_plans(extent_with_master_mark(220)));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let id = snapshot_id(&one.call(json!({"company_guid":GUID,"limit":4})).await);
        let refused = one
            .call(json!({"company_guid":GUID,"offset":4,"limit":4,"snapshot_id":id}))
            .await;
        let error = refusal(&refused);
        assert_eq!(error["code"], "listing_snapshot_changed");
        assert_eq!(error["cause"], "book_changed_since_first_page");
        assert_eq!(
            one.requests(),
            total,
            "nothing is read after the extent check"
        );

        let mut plans = basic_plans();
        plans.extend(continuation_plans(extent_with_master_mark(220)));
        plans.extend(
            basic_plans_marked(220)
                .into_iter()
                .skip(identity_plans().len()),
        );
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let id = snapshot_id(&one.call(json!({"company_guid":GUID,"limit":4})).await);
        let fresh = one
            .call(json!({"company_guid":GUID,"offset":4,"limit":4}))
            .await;
        assert_eq!(snapshot_of(&fresh)["reused"], false);
        assert_ne!(snapshot_of(&fresh)["id"], id.as_str());
        assert_eq!(snapshot_of(&fresh)["master_alter_id"], 220);
        assert_eq!(one.requests(), total);
    }

    /// #653 with #630: a compliance listing's rows are rendered with
    /// `party_gstin` read as of one date, so its snapshot serves only a page
    /// asking for that date. Named, another date is refused; the same date is
    /// served.
    #[tokio::test]
    async fn a_compliance_continuation_for_another_as_of_is_not_served_from_the_snapshot() {
        let mut plans = compliance_plans(masters(), balances());
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let first = one
            .call(json!({"company_guid":GUID,"fields":"compliance","as_of":"20260331","limit":1}))
            .await;
        let id = snapshot_id(&first);
        let refused = one
            .call(
                json!({"company_guid":GUID,"fields":"compliance","as_of":"20250630",
                "offset":1,"limit":1,"snapshot_id":id}),
            )
            .await;
        let error = refusal(&refused);
        assert_eq!(error["code"], "listing_snapshot_changed");
        assert_eq!(error["cause"], "snapshot_not_held");
        assert_eq!(
            one.requests(),
            total,
            "nothing is read after the extent check"
        );

        let mut plans = compliance_plans(masters(), balances());
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let id = snapshot_id(
            &one.call(
                json!({"company_guid":GUID,"fields":"compliance","as_of":"20260331","limit":1}),
            )
            .await,
        );
        let served = one
            .call(
                json!({"company_guid":GUID,"fields":"compliance","as_of":"2026-03-31",
                "offset":1,"limit":1,"snapshot_id":id}),
            )
            .await;
        assert_eq!(
            snapshot_of(&served)["reused"],
            true,
            "the same date, spelled either way"
        );
        assert_eq!(one.requests(), total);
    }

    /// Unnamed, a page for another `as_of` reads fresh, and its rows carry the
    /// date it asked for.
    #[tokio::test]
    async fn a_compliance_continuation_for_another_as_of_reads_fresh_when_unnamed() {
        let mut plans = compliance_plans(masters(), balances());
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        plans.extend(
            compliance_plans(masters(), balances())
                .into_iter()
                .skip(identity_plans().len()),
        );
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let id = snapshot_id(
            &one.call(
                json!({"company_guid":GUID,"fields":"compliance","as_of":"20260331","limit":1}),
            )
            .await,
        );
        let fresh = one
            .call(
                json!({"company_guid":GUID,"fields":"compliance","as_of":"20250630",
                "offset":1,"limit":1}),
            )
            .await;
        assert_eq!(snapshot_of(&fresh)["reused"], false);
        assert_ne!(snapshot_of(&fresh)["id"], id.as_str());
        for row in items(&fresh) {
            assert_eq!(row["party_gstin_as_of"], "20250630", "{row}");
        }
        assert_eq!(one.requests(), total);
    }

    /// A listing read with the dated mailing details is served only to a page that
    /// asks for them too. Named, a page that drops `include_mailing_details` is
    /// refused (nothing more is read); unnamed, it reads fresh from the standard
    /// request and its rows carry no `mailing_details` key.
    #[tokio::test]
    async fn a_compliance_continuation_that_drops_the_mailing_details_is_not_served_from_the_snapshot(
    ) {
        let held = mailing_masters();
        let held_balances = balances_matching(&held);
        let mut plans = compliance_plans(held, held_balances);
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let first = one
            .call(json!({"company_guid":GUID,"fields":"compliance","include_mailing_details":true,"limit":1}))
            .await;
        let id = snapshot_id(&first);
        let refused = one
            .call(
                json!({"company_guid":GUID,"fields":"compliance","offset":1,"limit":1,
                "snapshot_id":id}),
            )
            .await;
        let error = refusal(&refused);
        assert_eq!(error["code"], "listing_snapshot_changed");
        assert_eq!(error["cause"], "snapshot_not_held");
        assert_eq!(
            one.requests(),
            total,
            "nothing is read after the extent check"
        );

        let held = mailing_masters();
        let held_balances = balances_matching(&held);
        let mut plans = compliance_plans(held, held_balances);
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        plans.extend(
            compliance_plans(masters(), balances())
                .into_iter()
                .skip(identity_plans().len()),
        );
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let first = one
            .call(json!({"company_guid":GUID,"fields":"compliance","include_mailing_details":true,"limit":1}))
            .await;
        assert!(items(&first)[0]["compliance"]["mailing_details"].is_object());
        let fresh = one
            .call(json!({"company_guid":GUID,"fields":"compliance","offset":1,"limit":1}))
            .await;
        assert_eq!(snapshot_of(&fresh)["reused"], false);
        for row in items(&fresh) {
            assert!(
                !row["compliance"]
                    .as_object()
                    .unwrap()
                    .contains_key("mailing_details"),
                "{row}"
            );
        }
        assert_eq!(one.requests(), total);
    }

    /// A continuation that names no snapshot is still served from the held
    /// read while the book is unchanged: the id only makes a change loud.
    #[tokio::test]
    async fn a_continuation_without_an_id_is_served_from_the_held_read_while_the_book_is_unchanged()
    {
        let mut plans = basic_plans();
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let first = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let second = one
            .call(json!({"company_guid":GUID,"offset":4,"limit":4}))
            .await;
        assert_eq!(snapshot_of(&second)["reused"], true);
        assert_eq!(snapshot_of(&second)["id"], snapshot_of(&first)["id"]);
        assert_eq!(one.requests(), total);
    }

    /// A second first page replaces the held snapshot, so a continuation
    /// naming the first page's id is refused rather than served from the
    /// newer read, even though the book did not change.
    #[tokio::test]
    async fn a_continuation_naming_a_replaced_snapshot_is_refused() {
        let mut plans = basic_plans();
        plans.extend(basic_plans());
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let replaced = snapshot_id(&one.call(json!({"company_guid":GUID,"limit":4})).await);
        let _newer = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let refused = one
            .call(json!({"company_guid":GUID,"offset":4,"limit":4,"snapshot_id":replaced}))
            .await;
        let error = refusal(&refused);
        assert_eq!(error["code"], "listing_snapshot_changed");
        assert_eq!(error["cause"], "snapshot_not_held");
        assert_eq!(one.requests(), total);
    }

    /// A first page is a new question: it always reads fresh, even when an
    /// unexpired snapshot of the same listing is held.
    #[tokio::test]
    async fn a_first_page_always_reads_fresh() {
        let mut plans = basic_plans();
        plans.extend(basic_plans());
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let first = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let again = one.call(json!({"company_guid":GUID,"limit":4})).await;
        assert_eq!(snapshot_of(&again)["reused"], false);
        assert_ne!(snapshot_of(&again)["id"], snapshot_of(&first)["id"]);
        assert_eq!(one.requests(), total);
    }

    /// A snapshot that is no longer held refuses a continuation that names
    /// it: after the TTL, after a write through this server drops it, and
    /// when it was larger than the byte cap.
    #[tokio::test]
    async fn a_snapshot_no_longer_held_refuses_a_continuation_that_names_it() {
        for case in ["expired", "written", "over_cap"] {
            let mut plans = basic_plans();
            plans.extend(continuation_plans(extent_with_master_mark(219)));
            let total = plans.len();
            let one = OneServer::spawn(plans);
            {
                let mut listings = one.server.listings.lock().unwrap();
                match case {
                    "expired" => listings.ttl = std::time::Duration::ZERO,
                    "over_cap" => listings.max_bytes = 1,
                    _ => {}
                }
            }
            let id = snapshot_id(&one.call(json!({"company_guid":GUID,"limit":4})).await);
            if case == "written" {
                one.server.drop_listing_snapshots(GUID);
            }
            let refused = one
                .call(json!({"company_guid":GUID,"offset":4,"limit":4,"snapshot_id":id}))
                .await;
            let error = refusal(&refused);
            assert_eq!(error["code"], "listing_snapshot_changed", "{case}");
            assert_eq!(error["cause"], "snapshot_not_held", "{case}");
            assert_eq!(one.requests(), total, "{case}");
        }
    }

    /// A page served from a snapshot records only the requests it sent: the
    /// identity read and the extent pair, never its first page's read again.
    #[tokio::test]
    async fn a_page_served_from_a_snapshot_records_only_the_reads_it_sent() {
        let bytes = |response: &Value| {
            response["structuredContent"]["evidence"]["bytes"]
                .as_u64()
                .unwrap()
        };
        let mut plans = basic_plans();
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        let one = OneServer::spawn(plans);
        let first = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let served = one
            .call(json!({"company_guid":GUID,"offset":4,"limit":4}))
            .await;
        assert_eq!(snapshot_of(&served)["reused"], true);
        assert!(bytes(&served) < bytes(&first), "{served}");

        // The extent pair is what it counts: an extent one character longer
        // (a four-digit mark, UTF-16) costs 2 bytes more per read of the pair.
        let mut plans = basic_plans_marked(2_200);
        plans.extend(continuation_plans(extent_with_master_mark(2_200)));
        let one = OneServer::spawn(plans);
        let _first = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let longer = one
            .call(json!({"company_guid":GUID,"offset":4,"limit":4}))
            .await;
        assert_eq!(snapshot_of(&longer)["reused"], true);
        assert_eq!(bytes(&longer), bytes(&served) + 4);
    }

    /// An extent read refused after both its requests were sent (the pair
    /// disagreed) records both: the refusal's evidence counts what was sent.
    #[tokio::test]
    async fn a_refused_extent_read_still_records_the_requests_it_sent() {
        let refused_under = |mark: u64| async move {
            let mut plans = basic_plans_marked(mark);
            plans.extend(identity_plans());
            plans.push(xml(companies()));
            plans.extend([
                xml(extent_with_master_mark(mark)),
                status(),
                xml(extent_with_master_mark(mark + 1)),
                status(),
            ]);
            let total = plans.len();
            let one = OneServer::spawn(plans);
            let first = one.call(json!({"company_guid":GUID,"limit":4})).await;
            assert_ne!(first["isError"], true, "{first}");
            let refused = one
                .call(json!({"company_guid":GUID,"offset":4,"limit":4}))
                .await;
            assert_eq!(
                refusal(&refused)["code"],
                "listing_extent_read_failed",
                "{refused}"
            );
            let bytes = refused["structuredContent"]["evidence"]["bytes"]
                .as_u64()
                .unwrap();
            assert_eq!(one.requests(), total);
            bytes
        };
        // Both extent responses are counted: each is one character longer
        // under a four-digit mark, 2 bytes each in UTF-16.
        assert_eq!(refused_under(2_200).await, refused_under(219).await + 4);
    }

    /// An extent pair that completed, followed by a closing identity bracket
    /// that no longer finds the company, still records both extent requests.
    #[tokio::test]
    async fn a_closing_bracket_refusal_still_records_the_extent_pair() {
        let refused_under = |mark: u64| async move {
            let gone = companies();
            assert_eq!(gone.matches(GUID).count(), 1, "one row names the company");
            let mut plans = basic_plans_marked(mark);
            plans.extend(identity_plans());
            plans.push(xml(companies()));
            pair(&mut plans, xml(extent_with_master_mark(mark)));
            plans.push(xml(
                gone.replace(GUID, "00000000-0000-0000-0000-000000000000")
            ));
            let total = plans.len();
            let one = OneServer::spawn(plans);
            let first = one.call(json!({"company_guid":GUID,"limit":4})).await;
            assert_ne!(first["isError"], true, "{first}");
            let refused = one
                .call(json!({"company_guid":GUID,"offset":4,"limit":4}))
                .await;
            assert_eq!(
                refusal(&refused)["code"],
                "listing_extent_read_failed",
                "{refused}"
            );
            let bytes = refused["structuredContent"]["evidence"]["bytes"]
                .as_u64()
                .unwrap();
            assert_eq!(one.requests(), total);
            bytes
        };
        // Both extent responses are counted: each is one character longer
        // under a four-digit mark, 2 bytes each in UTF-16.
        assert_eq!(refused_under(2_200).await, refused_under(219).await + 4);
    }

    /// An expired snapshot is not only skipped but dropped the next time the
    /// store is touched: by holding another listing, or by any write's drop.
    #[tokio::test]
    async fn an_expired_snapshot_is_no_longer_held_once_the_store_is_next_touched() {
        let mut plans = basic_plans();
        plans.extend(basic_plans_reading(period_opening(), Some(groups())));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        one.server.listings.lock().unwrap().ttl = std::time::Duration::ZERO;
        let _basic = one.call(json!({"company_guid":GUID,"limit":4})).await;
        assert_eq!(one.server.listings.lock().unwrap().held.len(), 1);
        let _grouped = one
            .call(json!({"company_guid":GUID,"limit":4,"group":"Sundry Debtors"}))
            .await;
        assert_eq!(
            one.server.listings.lock().unwrap().held.len(),
            1,
            "holding the grouped listing dropped the expired basic one"
        );
        one.server
            .drop_listing_snapshots("00000000-0000-0000-0000-000000000000");
        assert!(
            one.server.listings.lock().unwrap().held.is_empty(),
            "a drop for another company still drops what has expired"
        );
        assert_eq!(one.requests(), total);
    }

    /// A write's drop still happens after the store's lock was poisoned: a
    /// drop that did nothing would let a snapshot outlive the write.
    #[tokio::test]
    async fn a_write_drops_snapshots_even_from_a_poisoned_store() {
        let one = OneServer::spawn(basic_plans());
        let _first = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let listings = one.server.listings.clone();
        let _ = std::thread::spawn(move || {
            let _held = listings.lock().unwrap();
            panic!("poison the listing store");
        })
        .join();
        assert!(one.server.listings.is_poisoned());
        one.server.drop_listing_snapshots(GUID);
        let store = one
            .server
            .listings
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(store.held.is_empty());
    }

    /// A listing that holds the group collection counts it toward the byte
    /// cap: the same rows with groups weigh more than without.
    #[tokio::test]
    async fn a_grouped_listing_counts_its_groups_toward_the_cap() {
        let mut plans = basic_plans();
        plans.extend(basic_plans_reading(period_opening(), Some(groups())));
        let one = OneServer::spawn(plans);
        let _basic = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let _grouped = one
            .call(json!({"company_guid":GUID,"limit":4,"group":"Sundry Debtors"}))
            .await;
        let store = one.server.listings.lock().unwrap();
        let [basic, grouped] = store.held.as_slice() else {
            panic!("two listings held");
        };
        assert_eq!(basic.rows, grouped.rows);
        assert!(grouped.bytes > basic.bytes);
    }

    /// The byte cap drops the oldest snapshot to make room for a newer one.
    #[tokio::test]
    async fn the_byte_cap_evicts_the_oldest_listing_first() {
        // Each listing's size, measured on its own server first.
        let sizes = {
            let mut plans = basic_plans();
            plans.extend(basic_plans_reading(period_opening(), Some(groups())));
            let one = OneServer::spawn(plans);
            let _basic = one.call(json!({"company_guid":GUID,"limit":4})).await;
            let _grouped = one
                .call(json!({"company_guid":GUID,"limit":4,"group":"Sundry Debtors"}))
                .await;
            let store = one.server.listings.lock().unwrap();
            store.held.iter().map(|held| held.bytes).collect::<Vec<_>>()
        };
        let mut plans = basic_plans();
        plans.extend(basic_plans_reading(period_opening(), Some(groups())));
        plans.extend(continuation_plans(extent_with_master_mark(219)));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        // Room for either listing, not both.
        one.server.listings.lock().unwrap().max_bytes = sizes.iter().sum::<usize>() - 1;
        let basic = one.call(json!({"company_guid":GUID,"limit":4})).await;
        let _grouped = one
            .call(json!({"company_guid":GUID,"limit":4,"group":"Sundry Debtors"}))
            .await;
        let refused = one
            .call(
                json!({"company_guid":GUID,"offset":4,"limit":4,"snapshot_id":snapshot_id(&basic)}),
            )
            .await;
        assert_eq!(refusal(&refused)["cause"], "snapshot_not_held");
        assert_eq!(
            one.server.listings.lock().unwrap().held.len(),
            1,
            "the newer listing is held"
        );
        assert_eq!(one.requests(), total);
    }

    async fn call(plans: Vec<ScenarioPlan>, args: Value) -> (Value, usize) {
        call_with_max_bytes(plans, args, 200_000).await
    }

    async fn call_with_max_bytes(
        plans: Vec<ScenarioPlan>,
        args: Value,
        max_bytes: usize,
    ) -> (Value, usize) {
        // The simulator stops serving when its plans run out, so a request
        // past them would reach a closed port and never be counted. A last
        // plan that is served only if such a request is sent lets the count
        // go over `plans.len()`; the cancel's wake-up connection carries no
        // method and is not counted.
        let mut plans = plans;
        plans.push(status());
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(Settings {
            endpoint: TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            data_dir: directory.path().into(),
            max_rows: 500,
            max_bytes,
            redaction: Redaction::None,
            import_enabled: false,
            posting: Posting::Off,
            master_writes_enabled: false,
            audit_enabled: false,
        });
        let response = server.call_tool("ledger_masters", args).await;
        simulator.cancel();
        let requests = simulator
            .finish()
            .unwrap()
            .into_iter()
            .filter(|request| !request.method.is_empty())
            .count();
        (response, requests)
    }

    /// Identity, then the extent-bracketed currency read of a book with one
    /// Currency master whose mailing name is not INR: the captured modern INR
    /// response with only `MAILINGNAME` changed. Admission refuses it before
    /// any master read. A book with several masters is no longer a refusal
    /// case (bridge#551): the classified read goes on to identify its base.
    fn foreign_base_currency_plans() -> Vec<ScenarioPlan> {
        let company = xml(companies());
        let extent = xml(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )
        .to_owned());
        let inr = captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        ));
        let from = "<MAILINGNAME TYPE=\"String\">INR</MAILINGNAME>";
        assert_eq!(inr.matches(from).count(), 1);
        let currency = xml(inr.replace(from, "<MAILINGNAME TYPE=\"String\">USD</MAILINGNAME>"));
        let mut plans = identity_plans();
        plans.push(company.clone());
        pair(&mut plans, extent.clone());
        pair(&mut plans, currency);
        pair(&mut plans, extent);
        plans.push(company);
        plans
    }

    /// Identity, then a currency pair whose second read disagrees with its
    /// first: the paired-read stability check refuses before admission.
    fn drifting_currency_plans() -> Vec<ScenarioPlan> {
        let company = xml(companies());
        let extent = xml(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )
        .to_owned());
        let single = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        )));
        let multi = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
        )));
        let mut plans = identity_plans();
        plans.push(company);
        pair(&mut plans, extent);
        plans.extend([single, status(), multi, status()]);
        plans
    }

    fn refusal(response: &Value) -> &Value {
        assert_eq!(response["isError"], true, "{response}");
        &response["structuredContent"]["result"]["error"]
    }

    #[tokio::test]
    async fn currency_refusal_names_its_cause_beside_the_operation_code() {
        let (response, _) = call(
            foreign_base_currency_plans(),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "company_base_currency_not_inr");
    }

    #[tokio::test]
    async fn join_refusal_names_the_validation_variant_as_its_cause() {
        let balance_name = "NAME=\"Bridge Nested Debtor WR4\"";
        let source = balances();
        assert_eq!(source.matches(balance_name).count(), 1);
        let renamed = source.replace(balance_name, "NAME=\"Bridge Renamed Debtor WR4\"");
        // The join refuses before the closing re-bracket, so those three
        // reads are never sent.
        let mut plans = compliance_plans(masters(), renamed);
        plans.truncate(plans.len() - 3);
        let (response, _) = call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "balance_missing_master_ledger");
    }

    #[tokio::test]
    async fn paired_read_drift_names_the_changed_source_as_its_cause() {
        let (response, _) = call(
            drifting_currency_plans(),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert_eq!(error["cause"], "currency_master_changed");
    }

    #[tokio::test]
    async fn cause_is_omitted_below_the_guidance_budget_and_the_code_survives() {
        // Control: the same refusal carries a cause at the default budget, so
        // its absence below is the budget rule and not a missing cause.
        let (response, _) = call(
            foreign_base_currency_plans(),
            json!({"company_guid":GUID,"fields":"compliance"}),
        )
        .await;
        assert_eq!(refusal(&response)["cause"], "company_base_currency_not_inr");
        let (response, _) = call_with_max_bytes(
            foreign_base_currency_plans(),
            json!({"company_guid":GUID,"fields":"compliance"}),
            REMEDIATION_MIN_RESPONSE_BUDGET - 1,
        )
        .await;
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed");
        assert!(error.get("cause").is_none(), "{error}");
    }

    fn items(response: &Value) -> &Vec<Value> {
        assert_ne!(response["isError"], true, "{response}");
        response["structuredContent"]["result"]["items"]
            .as_array()
            .unwrap_or_else(|| panic!("no items: {response}"))
    }

    fn row<'a>(items: &'a [Value], name: &str) -> &'a Value {
        items
            .iter()
            .find(|item| item["name"] == name)
            .unwrap_or_else(|| panic!("{name} missing from {items:?}"))
    }

    fn names(items: &[Value]) -> std::collections::BTreeSet<String> {
        items
            .iter()
            .map(|item| item["name"].as_str().unwrap().to_string())
            .collect()
    }

    fn group_filter(response: &Value) -> &Value {
        assert_ne!(response["isError"], true, "{response}");
        &response["structuredContent"]["result"]["group_filter"]
    }

    /// #631: "the ledgers in Sundry Debtors" means the subtree, and the default
    /// scope answers a narrower question. The captured book files one ledger
    /// under a user group below Sundry Debtors; the immediate filter leaves it
    /// out, and the result must say so, under either field set.
    #[tokio::test]
    async fn an_immediate_group_filter_names_the_sub_group_ledgers_it_left_out() {
        for (fields, plans) in [
            (
                "basic",
                basic_plans_reading(period_opening(), Some(groups())),
            ),
            ("compliance", compliance_plans(masters(), balances())),
        ] {
            let total = plans.len();
            let (response, requests) = call(
                plans,
                json!({"company_guid":GUID,"fields":fields,"group":"Sundry Debtors"}),
            )
            .await;
            assert_eq!(requests, total, "{fields}");
            let found = names(items(&response));
            assert_eq!(found.len(), 5, "{fields}: {found:?}");
            assert!(!found.contains("Bridge Nested Debtor WR4"), "{fields}");
            assert_eq!(
                group_filter(&response),
                &json!({
                    "group": "Sundry Debtors",
                    "scope": "immediate",
                    "excluded_subgroup_ledgers": {
                        "count": 1,
                        "group_count": 1,
                        "groups": ["Bridge Nested Debtors WR4"],
                    },
                    "unresolved_ancestry_ledgers": 0,
                }),
                "{fields}"
            );
        }
    }

    /// A ledger directly under Tally's root shows the root as Tally sent it, not
    /// as the decoders' working marker, under either field set.
    #[tokio::test]
    async fn a_ledger_under_the_reserved_root_shows_the_root_as_tally_sent_it() {
        for (fields, plans) in [
            ("basic", basic_plans_reading(period_opening(), None)),
            ("compliance", compliance_plans(masters(), balances())),
        ] {
            let (response, _) = call(plans, json!({"company_guid":GUID,"fields":fields})).await;
            let rows = items(&response);
            let roots = rows
                .iter()
                .filter(|row| {
                    row["parent"]
                        .as_str()
                        .is_some_and(|p| p.contains("Primary"))
                })
                .collect::<Vec<_>>();
            assert!(!roots.is_empty(), "{fields}");
            for row in roots {
                assert_eq!(row["parent"], "&#4; Primary", "{fields}");
            }
            assert!(!response.to_string().contains('\u{fffd}'), "{fields}");
        }
    }

    /// Ancestry scope needs the group collection, not the compliance fields:
    /// `fields=basic` (also the default) reads it once, paired, inside the
    /// ledger export's bracket, and its rows stay basic.
    #[tokio::test]
    async fn basic_ancestry_scope_admits_the_sub_group_ledger_from_one_paired_group_read() {
        let plans = basic_plans_reading(period_opening(), Some(groups()));
        let total = plans.len();
        let (response, requests) = call(
            plans,
            json!({"company_guid":GUID,"group":"Sundry Debtors","group_scope":"ancestry"}),
        )
        .await;
        assert_eq!(requests, total);
        let found = items(&response);
        assert_eq!(found.len(), 6, "{found:?}");
        assert_eq!(
            row(found, "Bridge Nested Debtor WR4")["parent"],
            "Bridge Nested Debtors WR4"
        );
        for item in found {
            assert!(item.get("ancestry").is_none(), "{item}");
        }
        assert_eq!(
            group_filter(&response),
            &json!({
                "group": "Sundry Debtors",
                "scope": "ancestry",
                "excluded_subgroup_ledgers": {"count": 0, "group_count": 0, "groups": []},
                "unresolved_ancestry_ledgers": 0,
            })
        );
    }

    /// A ledger whose chain stops before reaching the group might sit under it
    /// or not; Bridge cannot say, so it is counted rather than silently
    /// treated as outside. Only PARENT changes: `WR2 Sales` is re-parented to
    /// a group the captured collection does not hold. The baseline tests above
    /// report 0 for the same book, including the root-parented ledger.
    #[tokio::test]
    async fn a_ledger_whose_chain_breaks_before_the_group_is_counted_unresolved() {
        let from = "<PARENT TYPE=\"String\">Sales Accounts</PARENT>";
        let ledgers = period_opening();
        assert_eq!(ledgers.matches(from).count(), 1);
        let ledgers = ledgers.replace(from, "<PARENT TYPE=\"String\">Group Absent WR631</PARENT>");
        for scope in ["immediate", "ancestry"] {
            let (response, _) = call(
                basic_plans_reading(ledgers.clone(), Some(groups())),
                json!({"company_guid":GUID,"group":"Sundry Debtors","group_scope":scope}),
            )
            .await;
            assert!(!names(items(&response)).contains("WR2 Sales"), "{scope}");
            assert_eq!(
                group_filter(&response)["unresolved_ancestry_ledgers"],
                1,
                "{scope}"
            );
        }
    }

    /// The added group read is paired like the ledger export beside it: a
    /// collection that changes between its two reads is refused, not resolved
    /// against. Only one group NAME differs in the second read; the closing
    /// extent and identity reads are never sent.
    #[tokio::test]
    async fn a_group_collection_that_changes_between_its_paired_reads_is_refused() {
        let from = "Bridge Nested Debtors WR4";
        let source = groups();
        assert!(source.contains(from));
        let mut plans = basic_plans_reading(period_opening(), Some(source.clone()));
        let closing = 7;
        let second_group_read = plans.len() - closing - 2;
        plans[second_group_read] = xml(source.replace(from, "Bridge Nested Debtors WR631"));
        plans.truncate(plans.len() - closing);
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"group":"Sundry Debtors"})).await;
        assert_eq!(requests, total);
        let error = refusal(&response);
        assert_eq!(error["code"], "ledger_export_invalid");
        assert_eq!(error["cause"], "native_ledger_group_changed");
    }

    /// bridge#551: the compliance source checks the master response as soon
    /// as it is read, so a master that repeats a ledger's identity is refused
    /// before the balance request is sent. The captured master with its second
    /// ledger block repeated, and nothing else changed.
    #[tokio::test]
    async fn a_repeated_master_identity_is_refused_before_the_balance_read() {
        let forex = "b14e9b2d-8a63-4779-804d-25d59eb787eb";
        let companies = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
        )));
        let extent = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
        )));
        let fixture = |bytes: &[u8]| xml(captured(bytes));
        let master = captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/compliance_master_forex_live.utf16le.xml"
        ));
        let start = master.match_indices("<LEDGER NAME=").nth(1).unwrap().0;
        let end = start + master[start..].find("</LEDGER>").unwrap() + "</LEDGER>".len();
        let repeated = format!(
            "{}{}{}",
            &master[..end],
            &master[start..end],
            &master[end..]
        );
        let mut plans = Vec::new();
        pair(&mut plans, companies.clone());
        plans.push(companies.clone());
        pair(&mut plans, extent.clone());
        for source in [
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
            )),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/currency_originalname_forex_live.utf16le.xml"
            )),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/company_currencyname_live.utf16le.xml"
            )),
        ] {
            pair(&mut plans, source);
        }
        pair(&mut plans, extent.clone());
        plans.push(companies.clone());
        plans.extend([status(), companies.clone(), companies.clone()]);
        pair(&mut plans, extent);
        pair(&mut plans, xml(repeated));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let response = one
            .call(json!({"company_guid":forex,"fields":"compliance"}))
            .await;
        assert_eq!(one.requests(), total, "{response}");
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed", "{error}");
        assert_eq!(error["cause"], "duplicate_master_identity", "{error}");
    }

    /// The plans of a compliance read of the several-currency book of the capture
    /// (FOREX_601D_CAPTURE_PROVENANCE), with `balance` as the answer to the balance read.
    fn several_currency_compliance_plans(balance: String) -> Vec<ScenarioPlan> {
        let companies = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
        )));
        let extent = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
        )));
        let fixture = |bytes: &[u8]| xml(captured(bytes));
        let mut plans = Vec::new();
        pair(&mut plans, companies.clone());
        plans.push(companies.clone());
        pair(&mut plans, extent.clone());
        for source in [
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
            )),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/currency_originalname_forex_live.utf16le.xml"
            )),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/company_currencyname_live.utf16le.xml"
            )),
        ] {
            pair(&mut plans, source);
        }
        pair(&mut plans, extent.clone());
        plans.push(companies.clone());
        plans.extend([status(), companies.clone(), companies.clone()]);
        pair(&mut plans, extent.clone());
        for source in [
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/compliance_master_forex_live.utf16le.xml"
            )),
            xml(balance.clone()),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/group_snapshot_forex_live.utf16le.xml"
            )),
        ] {
            pair(&mut plans, source);
        }
        pair(&mut plans, extent.clone());
        plans.extend([companies.clone(), status(), companies.clone()]);
        // A second page is served from the first page's snapshot: identity,
        // then the bracketed extent pair only.
        pair(&mut plans, companies.clone());
        plans.push(companies.clone());
        pair(&mut plans, extent);
        plans.push(companies);
        plans
    }

    /// The captured balance answer of that book, with the closing balance of one rupee ledger
    /// (`FX Party 01`) replaced by `composite`; nothing else is changed.
    fn forex_balances_with(composite: &str) -> String {
        let capture = captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/balance_snapshot_forex_live.utf16le.xml"
        ));
        let row = capture
            .find("<LEDGER NAME=\"FX Party 01\"")
            .expect("the ledger's row");
        let tail = &capture[row..];
        let open = tail.find("<CLOSINGBALANCE").expect("a closing balance");
        let start = row + open + tail[open..].find('>').unwrap() + 1;
        let end = start + capture[start..].find("</CLOSINGBALANCE>").unwrap();
        assert!(
            capture[start..end].contains(" @ "),
            "a composite to replace"
        );
        format!("{}{}{}", &capture[..start], composite, &capture[end..])
    }

    /// bridge#551, through the tool on the several-currency book's captures:
    /// the compliance read admits it through the classified base, returns its
    /// plain rupee ledgers only, and names the three dollar ledgers and the
    /// three rupee ledgers with a composite balance that it left out. The
    /// extent, master, balance and group responses are one moment of the book
    /// (FOREX_601D_CAPTURE_PROVENANCE); the currency and Company reads are the
    /// committed 22 Sep captures, from before the C1 voucher, which added no
    /// Currency master.
    #[tokio::test]
    async fn a_several_currency_book_returns_its_base_ledgers_and_names_the_rest() {
        let forex = "b14e9b2d-8a63-4779-804d-25d59eb787eb";
        let companies = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
        )));
        let extent = xml(captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
        )));
        let fixture = |bytes: &[u8]| xml(captured(bytes));
        let mut plans = Vec::new();
        pair(&mut plans, companies.clone());
        plans.push(companies.clone());
        pair(&mut plans, extent.clone());
        for source in [
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
            )),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/currency_originalname_forex_live.utf16le.xml"
            )),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/company_currencyname_live.utf16le.xml"
            )),
        ] {
            pair(&mut plans, source);
        }
        pair(&mut plans, extent.clone());
        plans.push(companies.clone());
        plans.extend([status(), companies.clone(), companies.clone()]);
        pair(&mut plans, extent.clone());
        for source in [
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/compliance_master_forex_live.utf16le.xml"
            )),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/balance_snapshot_forex_live.utf16le.xml"
            )),
            fixture(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/group_snapshot_forex_live.utf16le.xml"
            )),
        ] {
            pair(&mut plans, source);
        }
        pair(&mut plans, extent.clone());
        plans.extend([companies.clone(), status(), companies.clone()]);
        // A second page is served from the first page's snapshot: identity,
        // then the bracketed extent pair only.
        pair(&mut plans, companies.clone());
        plans.push(companies.clone());
        pair(&mut plans, extent);
        plans.push(companies);
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let page = |offset: usize| json!({"company_guid":forex,"fields":"compliance","limit":2,"offset":offset});
        let first = one.call(page(0)).await;
        let second = one.call(page(2)).await;
        assert_eq!(one.requests(), total);
        let dollar = ["BRIDGE FX DEBTOR A", "FX USD Debtor 01", "FX USD Debtor 02"];
        let mut names = Vec::new();
        for response in [&first, &second] {
            let result = &response["structuredContent"]["result"];
            names.extend(
                items(response)
                    .iter()
                    .map(|row| row["name"].as_str().unwrap().to_string()),
            );
            assert_eq!(result["total"], 4);
            assert_eq!(result["ledgers_scope"], "base_currency_ledgers_only");
            // Rupee ledgers a dollar entry touched carry composite balances:
            // set aside by name, never read.
            let mixed = &result["base_currency_ledgers_mixed_excluded"];
            assert_eq!(mixed["count"], 3);
            assert_eq!(mixed["reason"], "mixed_currency_movement");
            assert_eq!(
                mixed["ledgers"],
                json!(["FX Party 01", "FX Sales", "Profit & Loss A/c"])
            );
            let excluded = &result["foreign_currency_ledgers_excluded"];
            assert_eq!(excluded["count"], 3);
            let mut listed = excluded["ledgers"]
                .as_array()
                .unwrap()
                .iter()
                .map(|ledger| {
                    assert_eq!(ledger["currency"], "$");
                    ledger["ledger"].as_str().unwrap().to_string()
                })
                .collect::<Vec<_>>();
            listed.sort();
            assert_eq!(listed, dollar);
            // The composite opening on a dollar ledger is never read or shown.
            assert!(!response.to_string().contains(" @ "), "{response}");
        }
        assert_eq!(
            names,
            ["BRIDGE INR DEBTOR A", "Cash", "FX Party 02", "FX Party 03"]
        );
        assert_eq!(
            second["structuredContent"]["result"]["snapshot"]["reused"],
            true
        );
    }

    /// A basic read of a book holding a foreign-currency opening is refused as
    /// before, but names why and what to do (#675). The composite is the one in
    /// the captured several-currency ledgers, placed in the captured basic
    /// export; the closing extent and identity reads are never sent.
    #[tokio::test]
    async fn a_foreign_currency_opening_refuses_the_basic_read_with_its_cause() {
        let forex = captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/ledgers_currency_forex_live.utf16le.xml"
        ));
        let composite = forex
            .split("<OPENINGBALANCE")
            .skip(1)
            .filter_map(|tail| {
                let text = &tail[tail.find('>')? + 1..tail.find("</OPENINGBALANCE>")?];
                text.contains(" @ ").then(|| text.to_string())
            })
            .collect::<Vec<_>>();
        assert_eq!(composite.len(), 1, "one composite opening in the capture");
        let row = "<OPENINGBALANCE TYPE=\"Amount\">-50000.00</OPENINGBALANCE>";
        let source = period_opening();
        assert_eq!(source.matches(row).count(), 1);
        let foreign = source.replace(
            row,
            &format!(
                "<OPENINGBALANCE TYPE=\"Amount\">{}</OPENINGBALANCE>",
                composite[0]
            ),
        );
        let mut plans = basic_plans_reading(foreign, None);
        plans.truncate(plans.len() - 7);
        let total = plans.len();
        let (response, requests) = call(plans, json!({"company_guid":GUID})).await;
        assert_eq!(requests, total);
        let error = refusal(&response);
        assert_eq!(error["code"], "ledger_export_invalid");
        assert_eq!(error["cause"], "foreign_currency_ledger_balance");
        let remediation = error["remediation"].as_str().unwrap();
        assert!(remediation.contains("#683"), "{error}");
    }

    /// The whole next step of the refusal for a foreign-currency balance in a layout no capture
    /// shows (#2071, #683). It names no layout: the layouts the captures show are set aside, so
    /// what reaches this step is another shape.
    const FOREIGN_BALANCE_STEP: &str = "Tally wrote a ledger's balance in a foreign currency in a layout ComplyEaze Bridge does not recognise (#683), so this read is refused rather than guessed at, not because the response was damaged, and nothing was returned from it. Do not repeat it in a loop: it refuses again. Tell the user that this read cannot give this company's ledgers while Tally writes a balance that way, and that Tally itself shows them.";

    /// bridge#2071 and #683, through the tool: a rupee ledger whose closing balance is a composite
    /// is set aside by name, in the layout of the 7.1 captures (a space after the foreign symbol)
    /// and in the layout TallyPrime 7.2 wrote in the lab (`-$100.00 @ ₹ 87/$ = -₹ 8700.00`, as
    /// in the lab's Lab Bank row: no space after the foreign symbol, one before `=`). A layout no
    /// capture shows (here a rate with decimals) is not recognised, so the row stays a plain amount
    /// and the compliance read is refused, with the code and cause the lab measured and the step.
    /// The balance answer is the committed capture of the several-currency book with one closing
    /// balance replaced.
    #[tokio::test]
    async fn the_72_layout_is_set_aside_and_a_layout_no_capture_shows_refuses_with_its_step() {
        let forex = "b14e9b2d-8a63-4779-804d-25d59eb787eb";
        let args = json!({"company_guid":forex,"fields":"compliance"});
        let set_aside = |read: &Value| {
            assert_ne!(read["isError"], true, "{read}");
            let result = &read["structuredContent"]["result"];
            let named = result["base_currency_ledgers_mixed_excluded"]["ledgers"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|name| *name == "FX Party 01")
                .count();
            assert_eq!(named, 1, "{result}");
        };
        // The lab's 7.2 layout, and the captured 7.1 layout of the same balance.
        for composite in [
            "-$100.00 @ \u{20b9} 87/$ = -\u{20b9} 8700.00",
            "-$ 100.00 @ \u{20b9} 87/$  = -\u{20b9} 8700.00",
        ] {
            let one = OneServer::spawn(several_currency_compliance_plans(forex_balances_with(
                composite,
            )));
            set_aside(&one.call(args.clone()).await);
        }
        // A rate with decimals is in neither layout.
        let unrecognised = "-$100.00 @ \u{20b9} 87.5/$ = -\u{20b9} 8750.00";
        let one = OneServer::spawn(several_currency_compliance_plans(forex_balances_with(
            unrecognised,
        )));
        let response = one.call(args).await;
        let error = refusal(&response);
        assert_eq!(error["code"], "party_ledger_master_read_failed", "{error}");
        assert_eq!(
            error["cause"], "native_foreign_currency_ledger_balance",
            "{error}"
        );
        let remediation = error["remediation"].as_str().expect("a next step");
        assert_eq!(remediation, FOREIGN_BALANCE_STEP);
        assert!(!remediation.contains("-$100.00"), "{remediation}");
    }

    /// The typed pieces of that refusal on the committed 7.1 capture of the several-currency
    /// ledgers (#2071): the strict snapshot parse gives the variant `ForeignCurrencyLedgerBalance`
    /// as a type, not by text; the refusal built from it carries the code and cause the lab
    /// measured; and the step is chosen by that code and cause together, so the same cause under
    /// another code has none of its own. The path the tool takes with the lab's layout is the
    /// test above.
    #[test]
    fn a_foreign_currency_closing_balance_refuses_the_compliance_read_with_a_next_step() {
        use bridge_tally_protocol::native_outstandings::{
            parse_native_ledger_snapshot, NativeOutstandingsError,
        };
        let capture = captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/ledgers_currency_forex_live.utf16le.xml"
        ));
        let Err(variant) = parse_native_ledger_snapshot(&capture) else {
            panic!("the capture's composite closing balance is refused");
        };
        assert_eq!(
            variant,
            NativeOutstandingsError::ForeignCurrencyLedgerBalance {
                ledger_name: "BRIDGE FX DEBTOR A".to_string()
            }
        );
        let failure = crate::agent::ToolFailure::from_runtime(
            "party_ledger_master_read_failed",
            anyhow::Error::new(variant),
        );
        assert_eq!(failure.code, "party_ledger_master_read_failed");
        assert_eq!(
            failure.cause,
            Some("native_foreign_currency_ledger_balance")
        );
        let remediation =
            crate::agent::remediation_for(&failure.code, failure.cause).expect("a next step");
        assert_eq!(remediation, FOREIGN_BALANCE_STEP);
        // The ledger's name is not in the step.
        assert!(!remediation.contains("BRIDGE FX DEBTOR A"), "{remediation}");
        // The step belongs to this code: the same cause under another code is given none of
        // its own (the open-bills read says it in band, with a different step).
        assert_eq!(
            crate::agent::remediation_for("ledger_export_invalid", failure.cause),
            None
        );
        assert_eq!(
            crate::agent::remediation_for("native_outstandings_read_failed", failure.cause),
            None
        );
    }

    #[tokio::test]
    async fn compliance_rows_carry_the_chain_resolved_from_the_captured_groups() {
        let plans = compliance_plans(masters(), balances());
        let total = plans.len();
        let (response, requests) =
            call(plans, json!({"company_guid":GUID,"fields":"compliance"})).await;
        let items = items(&response);
        assert_eq!(requests, total);
        assert_eq!(items.len(), 9);
        // A user-created group (empty RESERVEDNAME) is a hop like any other,
        // and the walk continues through it to the reserved root.
        assert_eq!(
            row(items, "Bridge Nested Debtor WR4")["ancestry"],
            json!({
                "chain": [
                    {"name": "Bridge Nested Debtors WR4", "reserved_name": ""},
                    {"name": "Sundry Debtors", "reserved_name": "Sundry Debtors"},
                    {"name": "Current Assets", "reserved_name": "Current Assets"},
                ],
                "complete": true,
                "gap": null,
            })
        );
        assert_eq!(
            row(items, "Cash")["ancestry"],
            json!({
                "chain": [
                    {"name": "Cash-in-Hand", "reserved_name": "Cash-in-Hand"},
                    {"name": "Current Assets", "reserved_name": "Current Assets"},
                ],
                "complete": true,
                "gap": null,
            })
        );
        for item in items {
            assert!(item["ancestry"].is_object(), "{item}");
        }
    }

    #[tokio::test]
    async fn ancestry_scope_admits_a_ledger_whose_parent_is_below_the_group() {
        let sundry = |scope: &str| json!({"company_guid":GUID,"fields":"compliance","group":"Sundry Debtors","group_scope":scope});
        let (immediate, _) =
            call(compliance_plans(masters(), balances()), sundry("immediate")).await;
        let (ancestry, _) = call(compliance_plans(masters(), balances()), sundry("ancestry")).await;
        let immediate = names(items(&immediate));
        let ancestry = names(items(&ancestry));
        assert!(!immediate.is_empty());
        assert!(!immediate.contains("Bridge Nested Debtor WR4"));
        let mut expected = immediate.clone();
        expected.insert("Bridge Nested Debtor WR4".to_string());
        assert_eq!(ancestry, expected);
    }

    #[tokio::test]
    async fn ancestry_scope_reaches_loans_liability_through_bank_od() {
        // The capture has the `Bank OD A/c` -> `Loans (Liability)` groups but
        // no ledger under them, so one captured ledger is re-parented in both
        // the master and the balance response. Only PARENT changes; this is
        // not live evidence of such a ledger.
        let reparent = |source: String| {
            let from = "<PARENT TYPE=\"String\">Sales Accounts</PARENT>";
            assert_eq!(source.matches(from).count(), 1);
            source.replace(from, "<PARENT TYPE=\"String\">Bank OD A/c</PARENT>")
        };
        let loans = |scope: &str| json!({"company_guid":GUID,"fields":"compliance","group":"Loans (Liability)","group_scope":scope});
        let (response, _) = call(
            compliance_plans(reparent(masters()), reparent(balances())),
            loans("ancestry"),
        )
        .await;
        let found = items(&response);
        assert_eq!(names(found), ["WR2 Sales".to_string()].into());
        assert_eq!(found[0]["parent"], "Bank OD A/c");
        assert_eq!(
            found[0]["ancestry"],
            json!({
                "chain": [
                    {"name": "Bank OD A/c", "reserved_name": "Bank OD A/c"},
                    {"name": "Loans (Liability)", "reserved_name": "Loans (Liability)"},
                ],
                "complete": true,
                "gap": null,
            })
        );
        assert_eq!(response["structuredContent"]["result"]["total"], 1);

        // The default scope still matches only the immediate parent.
        let (response, _) = call(
            compliance_plans(reparent(masters()), reparent(balances())),
            loans("immediate"),
        )
        .await;
        assert!(items(&response).is_empty());
    }

    /// #554 through the stdio server: `ledger_masters` basic is two queued
    /// operations (the identity read, then the ledger read). The identity read's
    /// first leg is held; while it runs the client sends `second`. Returns the
    /// response to request 7 and how many requests reached the simulator.
    async fn serve_ledger_masters_then(second: &[&str]) -> (Value, usize) {
        let (responses, sent) = serve_ledger_masters_collecting(second, 1).await;
        let response = responses
            .into_iter()
            .find(|value| value["id"] == 7)
            .expect("a response to request 7");
        (response, sent)
    }

    /// As `serve_ledger_masters_then`, returning every response in the order
    /// written, once `expected` responses other than `initialize`'s arrived.
    async fn serve_ledger_masters_collecting(
        second: &[&str],
        expected: usize,
    ) -> (Vec<Value>, usize) {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        let mut plans = basic_plans();
        plans[0] = plans[0]
            .clone()
            .with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
                std::time::Duration::from_millis(900),
            ));
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(Settings {
            endpoint: TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            data_dir: directory.path().into(),
            max_rows: 500,
            max_bytes: 200_000,
            redaction: Redaction::None,
            import_enabled: false,
            posting: Posting::Off,
            master_writes_enabled: false,
            audit_enabled: false,
        });
        let (client, source) = tokio::io::duplex(1 << 20);
        let (client_read, mut client_write) = tokio::io::split(client);
        let (source_read, mut source_write) = tokio::io::split(source);
        let serve = async move {
            crate::agent::agent_protocol::serve_stdio(
                server,
                BufReader::new(source_read),
                &mut source_write,
            )
            .await
        };
        let client = async move {
            let call = json!({"jsonrpc":"2.0","id":7,"method":"tools/call","params":{
                "name":"ledger_masters","arguments":{"company_guid":GUID}}});
            let opening = format!(
                "{}\n{}\n{}\n",
                json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}),
                json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
                call
            );
            client_write.write_all(opening.as_bytes()).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            for frame in second {
                if *frame == CLOSE_INPUT {
                    client_write.shutdown().await.unwrap();
                    continue;
                }
                client_write.write_all(frame.as_bytes()).await.unwrap();
                client_write.write_all(b"\n").await.unwrap();
            }
            let mut lines = BufReader::new(client_read).lines();
            let mut responses = Vec::new();
            let mut answered = 0;
            while answered < expected {
                let line = lines
                    .next_line()
                    .await
                    .unwrap()
                    .expect("every expected response");
                let value: Value = serde_json::from_str(&line).unwrap();
                if value["id"] != 1 {
                    answered += 1;
                }
                responses.push(value);
            }
            drop(client_write);
            responses
        };
        let (served, responses) = tokio::time::timeout(std::time::Duration::from_secs(20), async {
            tokio::join!(serve, client)
        })
        .await
        .unwrap();
        served.unwrap();
        simulator.cancel();
        // `cancel` wakes the simulator with an empty connection; count only the
        // requests Bridge actually sent.
        let sent = simulator
            .finish()
            .unwrap()
            .iter()
            .filter(|request| !request.method.is_empty())
            .count();
        (responses, sent)
    }

    /// In `second`, closes the client's input instead of sending a frame.
    const CLOSE_INPUT: &str = "<close input>";

    #[tokio::test]
    async fn closing_the_input_during_a_read_does_not_withdraw_it() {
        // A client may write its requests, close its side and still read the
        // answers: a closed input is not a cancellation, so the read completes
        // in full and every request of it is sent.
        let (response, requests) = serve_ledger_masters_then(&[CLOSE_INPUT]).await;
        assert_eq!(response["result"]["isError"], false, "{response}");
        assert_eq!(requests, basic_plans().len());
    }

    #[tokio::test]
    async fn input_is_left_in_the_pipe_once_eight_requests_wait() {
        // The bound on what a read holds: once eight requests are queued, input
        // is not read until the call ends, so a cancellation behind them is not
        // seen (the read completes in full, as before #554) and all eight are
        // then answered. Without the bound the queue would grow without limit.
        let mut frames: Vec<String> = (100..108)
            .map(|id| json!({"jsonrpc":"2.0","id":id,"method":"tools/list"}).to_string())
            .collect();
        frames.push(
            json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":7}})
                .to_string(),
        );
        let frames: Vec<&str> = frames.iter().map(String::as_str).collect();
        let (responses, sent) = serve_ledger_masters_collecting(&frames, 9).await;
        let call = responses.iter().find(|value| value["id"] == 7).unwrap();
        assert_eq!(call["result"]["isError"], false, "{call}");
        assert_eq!(sent, basic_plans().len());
    }

    #[tokio::test]
    async fn requests_sent_during_a_read_are_all_served_after_it() {
        // Before #554 no input was read while a read ran, so a burst of requests
        // waited in the pipe and was served in order afterwards. Watching input
        // must not turn that into refusals: ten requests during one held read
        // are all answered, after the read, in the order sent.
        let burst: Vec<String> = (100..110)
            .map(|id| json!({"jsonrpc":"2.0","id":id,"method":"tools/list"}).to_string())
            .collect();
        let frames: Vec<&str> = burst.iter().map(String::as_str).collect();
        let (responses, _) = serve_ledger_masters_collecting(&frames, 11).await;
        let ids: Vec<Value> = responses
            .iter()
            .map(|value| value["id"].clone())
            .filter(|id| *id != 1)
            .collect();
        let expected: Vec<Value> = std::iter::once(json!(7))
            .chain((100..110).map(|id| json!(id)))
            .collect();
        assert_eq!(ids, expected, "{responses:?}");
        assert!(
            responses.iter().all(|value| value.get("error").is_none()),
            "{responses:?}"
        );
    }

    /// Input that yields `data`, then waits until `fail` fires and returns an
    /// I/O error, as a host whose pipe breaks mid-call would.
    struct FailingInput {
        data: std::io::Cursor<Vec<u8>>,
        fail: tokio::sync::oneshot::Receiver<()>,
    }

    impl tokio::io::AsyncRead for FailingInput {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            use std::io::Read;
            let remaining = buf.remaining();
            let mut chunk = vec![0; remaining];
            let read = self.data.read(&mut chunk).unwrap();
            if read > 0 {
                buf.put_slice(&chunk[..read]);
                return std::task::Poll::Ready(Ok(()));
            }
            match std::future::Future::poll(std::pin::Pin::new(&mut self.fail), cx) {
                std::task::Poll::Ready(_) => {
                    std::task::Poll::Ready(Err(std::io::Error::other("pipe broke")))
                }
                std::task::Poll::Pending => std::task::Poll::Pending,
            }
        }
    }

    #[tokio::test]
    async fn an_input_failure_still_answers_the_call_in_flight_then_ends() {
        // The input breaks while the identity read is held: the call stops
        // before its next operation, its answer is still written (the output
        // works), and only then does the session end with the input error.
        let mut plans = basic_plans();
        plans[0] = plans[0]
            .clone()
            .with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
                std::time::Duration::from_millis(900),
            ));
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(Settings {
            endpoint: TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            data_dir: directory.path().into(),
            max_rows: 500,
            max_bytes: 200_000,
            redaction: Redaction::None,
            import_enabled: false,
            posting: Posting::Off,
            master_writes_enabled: false,
            audit_enabled: false,
        });
        let frames = format!(
            "{}\n{}\n",
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}),
            json!({"jsonrpc":"2.0","id":7,"method":"tools/call","params":{
                "name":"ledger_masters","arguments":{"company_guid":GUID}}})
        );
        let (fail, failed) = tokio::sync::oneshot::channel();
        let input = FailingInput {
            data: std::io::Cursor::new(frames.into_bytes()),
            fail: failed,
        };
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            let _ = fail.send(());
        });
        let mut output = Vec::new();
        let served = tokio::time::timeout(
            std::time::Duration::from_secs(20),
            crate::agent::agent_protocol::serve_stdio(
                server,
                tokio::io::BufReader::new(input),
                &mut output,
            ),
        )
        .await
        .unwrap();
        assert!(served.is_err(), "the session ends with the input error");
        let responses: Vec<Value> = String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let call = responses
            .iter()
            .find(|value| value["id"] == 7)
            .expect("the call in flight is still answered");
        assert_eq!(
            call["result"]["structuredContent"]["result"]["error"]["code"],
            "request_cancelled"
        );
        simulator.cancel();
        let sent = simulator
            .finish()
            .unwrap()
            .iter()
            .filter(|request| !request.method.is_empty())
            .count();
        assert_eq!(sent, identity_plans().len());
    }

    #[tokio::test]
    async fn a_withdrawn_call_stops_before_its_next_operation() {
        let (response, requests) = serve_ledger_masters_then(&[
            r#"{"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":7}}"#,
        ])
        .await;
        // The identity read in flight completes (abandoning it would not stop
        // Tally); the ledger read is never sent, and the call is refused as
        // withdrawn with partial evidence, never answered with a partial read.
        assert_eq!(requests, identity_plans().len(), "{response}");
        let result = &response["result"];
        assert_eq!(result["isError"], true, "{response}");
        assert_eq!(
            result["structuredContent"]["result"]["error"]["code"],
            "request_cancelled"
        );
        assert_eq!(result["structuredContent"]["evidence"]["state"], "partial");
    }

    /// A tool call, served over MCP, fits the 2 MiB a test thread gets by
    /// default, held here whatever `RUST_MIN_STACK` says (#697). A debug build
    /// has no room to spare: measured, the call needed about 1.75 MiB, and a
    /// stack overflow aborts the whole test binary rather than failing one
    /// test. Any change that makes a tool call's future larger has to box it
    /// (see `with_operation_wire_budget`) before this passes again.
    #[test]
    fn a_tool_call_fits_a_two_mib_stack() {
        const STACK: usize = 2 * 1024 * 1024;
        std::thread::Builder::new()
            .stack_size(STACK)
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let (response, requests) = serve_ledger_masters_then(&[
                            r#"{"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":7}}"#,
                        ])
                        .await;
                        assert_eq!(requests, identity_plans().len(), "{response}");
                        assert_eq!(response["result"]["isError"], true, "{response}");
                    });
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[tokio::test]
    async fn an_unrelated_notification_does_not_stop_the_call() {
        // Control: a cancellation naming another request, and a plain
        // notification, arrive while the call runs; it completes in full.
        let (response, requests) = serve_ledger_masters_then(&[
            r#"{"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":99}}"#,
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        ])
        .await;
        assert_eq!(response["result"]["isError"], false, "{response}");
        assert_eq!(requests, basic_plans().len());
    }

    // -- include_gst_details: the opt-in dated GST rate details ---------------

    /// The GST details read, through the tool's own path. The request on the
    /// wire is what each test pins, by the hash the simulator keeps of every
    /// request body; the answers are the real capture or the harness's own
    /// generated books.
    mod gst_details {
        use super::*;
        use crate::tally::connection::{
            compliance_master_ledgers_per_read, parent_partition_limits, ComplianceRead,
        };
        use bridge_tally_protocol::native_outstandings::{
            render_party_ledger_master_request_for_parents,
            render_party_ledger_master_request_for_parents_with_gst_details,
            render_party_ledger_master_request_with_gst_details,
        };
        use bridge_tally_protocol::parent_partition::{
            ParentObservation, ParentPart, ParentPartition,
        };
        use tally_protocol_simulator::ObservedRequest;

        /// The company the shared harness answers for.
        const HARNESS_COMPANY: &str = "WR2 Unicode Lab";
        /// The capture's own company GUID, which the harness company's
        /// replaces so that the real bytes are served as its answer.
        const CAPTURE_COMPANY_GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

        fn capture() -> String {
            captured(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/ledger_gst_details_live.utf16le.xml"
            ))
        }

        /// The capture as the harness company's master answer: its company
        /// GUID, in every ledger's GUID and `BRIDGECOMPANYGUID`, is the only
        /// thing changed.
        fn capture_for_the_harness_company() -> String {
            let capture = capture();
            assert_eq!(capture.matches(CAPTURE_COMPANY_GUID).count(), 28);
            capture.replace(CAPTURE_COMPANY_GUID, GUID)
        }

        /// Each ledger of the capture: its (escaped) name and parent text.
        fn capture_ledgers(capture: &str) -> Vec<(String, String)> {
            capture
                .split("<LEDGER NAME=\"")
                .skip(1)
                .map(|block| {
                    let name = block[..block.find('"').unwrap()].to_owned();
                    let from = block.find("<PARENT TYPE=\"String\">").unwrap()
                        + "<PARENT TYPE=\"String\">".len();
                    let parent =
                        block[from..from + block[from..].find("</PARENT>").unwrap()].to_owned();
                    (name, parent)
                })
                .collect()
        }

        /// The ledger names whose `GSTDETAILS.LIST` holds a dated entry,
        /// counted from the bytes.
        fn ledgers_with_entries(capture: &str) -> Vec<String> {
            capture
                .split("<LEDGER NAME=\"")
                .skip(1)
                .filter(|block| {
                    block
                        .split("<GSTDETAILS.LIST>")
                        .skip(1)
                        .any(|list| list.trim_start().starts_with("<APPLICABLEFROM"))
                })
                .map(|block| block[..block.find('"').unwrap()].replace("&amp;", "&"))
                .collect()
        }

        /// A balance snapshot for the capture's ledgers: the harness's real
        /// balance row, once per ledger, at the capture's own zero opening.
        fn balances_for(ledgers: &[(String, String)]) -> String {
            const MARK: &str = "</LEDGER>";
            let source = balances();
            let first = source.find("    <LEDGER ").unwrap();
            let first_end = first + source[first..].find(MARK).unwrap() + MARK.len();
            let last_end = source.rfind(MARK).unwrap() + MARK.len();
            let template = source[first..first_end].trim_start();
            let mut out = source[..first].to_owned();
            for (position, (name, parent)) in ledgers.iter().enumerate() {
                if position > 0 {
                    out.push_str("\n    ");
                }
                out.push_str(
                    &template
                        .replace("Bridge Nested Debtors WR4", parent)
                        .replace("Bridge Nested Debtor WR4", name)
                        .replace("-50000.00", "0.00"),
                );
            }
            out.push_str(&source[last_end..]);
            out
        }

        fn capture_plans() -> Vec<ScenarioPlan> {
            let master = capture_for_the_harness_company();
            let balances = balances_for(&capture_ledgers(&master));
            compliance_plans(master, balances)
        }

        /// What the production code asks Tally for the harness company: the
        /// period is the captured extent's books-from and last voucher date.
        fn harness_period() -> bridge_tally_protocol::native_outstandings::NativeLedgerExportPeriod
        {
            NativeLedgerExportPeriod::new(
                DateBoundaryProfile::ModeAgnostic,
                TallyDate::parse("20260401").unwrap(),
                TallyDate::parse("20260907").unwrap(),
            )
            .unwrap()
        }

        fn wire_sha(request: &str) -> String {
            sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
                request,
            ))
        }

        /// How many of the requests sent were exactly `request`.
        fn sent(observed: &[ObservedRequest], request: &str) -> usize {
            let sha = wire_sha(request);
            observed
                .iter()
                .filter(|observed| observed.request_body_sha256 == sha)
                .count()
        }

        /// How many of the requests sent were a master read, of any variant:
        /// the production request or its details variant, whole or by parents.
        fn master_reads_sent(observed: &[ObservedRequest], parts: &[ParentPart]) -> usize {
            let period = harness_period();
            let mut requests = vec![
                render_party_ledger_master_request(HARNESS_COMPANY, &period),
                render_party_ledger_master_request_with_gst_details(HARNESS_COMPANY, &period),
            ];
            for part in parts {
                requests.push(render_party_ledger_master_request_for_parents(
                    HARNESS_COMPANY,
                    &period,
                    part,
                ));
                requests.push(
                    render_party_ledger_master_request_for_parents_with_gst_details(
                        HARNESS_COMPANY,
                        &period,
                        part,
                    ),
                );
            }
            requests.iter().map(|request| sent(observed, request)).sum()
        }

        async fn call_observed(
            plans: Vec<ScenarioPlan>,
            args: Value,
            redaction: Redaction,
            max_bytes: usize,
        ) -> (Value, Vec<ObservedRequest>) {
            // A last plan served only if a request is sent past the others.
            let mut plans = plans;
            plans.push(status());
            let simulator = SequenceSimulator::spawn(plans).unwrap();
            let directory = tempfile::tempdir().unwrap();
            let server = Server::new(Settings {
                endpoint: TallyEndpointConfig {
                    host: "127.0.0.1".into(),
                    port: simulator.address().port(),
                },
                data_dir: directory.path().into(),
                max_rows: 500,
                max_bytes,
                redaction,
                import_enabled: false,
                posting: crate::agent::Posting::Off,
                master_writes_enabled: false,
                audit_enabled: false,
            });
            let response = server.call_tool("ledger_masters", args).await;
            simulator.cancel();
            let observed = simulator
                .finish()
                .unwrap()
                .into_iter()
                .filter(|request| !request.method.is_empty())
                .collect();
            (response, observed)
        }

        fn compliance_args(flag: Option<bool>) -> Value {
            match flag {
                None => json!({"company_guid":GUID,"fields":"compliance"}),
                Some(flag) => {
                    json!({"company_guid":GUID,"fields":"compliance","include_gst_details":flag})
                }
            }
        }

        /// Without the flag (absent or false) the master read is the
        /// production request, byte for byte, and no row carries a
        /// `gst_details` key, not even a null one.
        #[tokio::test]
        async fn without_the_flag_the_request_is_the_production_one_and_no_row_carries_gst_details()
        {
            for flag in [None, Some(false)] {
                let plans = compliance_plans(masters(), balances());
                let total = plans.len();
                let (response, observed) =
                    call_observed(plans, compliance_args(flag), Redaction::None, 200_000).await;
                assert_eq!(observed.len(), total, "{flag:?}");
                let period = harness_period();
                assert_eq!(
                    sent(
                        &observed,
                        &render_party_ledger_master_request(HARNESS_COMPANY, &period)
                    ),
                    2,
                    "the production compliance request, read as a pair: {flag:?}"
                );
                assert_eq!(
                    sent(
                        &observed,
                        &render_party_ledger_master_request_with_gst_details(
                            HARNESS_COMPANY,
                            &period
                        )
                    ),
                    0,
                    "{flag:?}"
                );
                for row in items(&response) {
                    let compliance = row["compliance"].as_object().unwrap();
                    assert!(!compliance.contains_key("gst_details"), "{flag:?}: {row}");
                    assert!(row.get("gst_details").is_none(), "{flag:?}: {row}");
                    // The registration history is untouched beside it.
                    assert!(compliance.contains_key("gst_registrations"), "{row}");
                }
                assert!(
                    response["structuredContent"]["result"]
                        .get("limitations")
                        .is_none(),
                    "{flag:?}: {response}"
                );
            }
        }

        /// With the flag, the request sent is the production one with the
        /// pinned FETCH suffix, and the answer is the live capture: the five
        /// ledgers that carry entries show them, the nine that carry an empty
        /// placeholder show an empty list, and the Cess rows are `absent`.
        #[tokio::test]
        async fn with_the_flag_the_extended_request_is_sent_and_the_captured_rates_are_returned() {
            let capture = capture();
            let with_entries = ledgers_with_entries(&capture);
            assert_eq!(with_entries.len(), 5, "counted from the capture's bytes");
            let ledgers = capture_ledgers(&capture);
            assert_eq!(ledgers.len(), 14);

            let plans = capture_plans();
            let total = plans.len();
            let (response, observed) =
                call_observed(plans, compliance_args(Some(true)), Redaction::None, 200_000).await;
            assert_eq!(observed.len(), total);
            let period = harness_period();
            assert_eq!(
                sent(
                    &observed,
                    &render_party_ledger_master_request_with_gst_details(HARNESS_COMPANY, &period)
                ),
                2,
                "the extended compliance request, read as a pair"
            );
            assert_eq!(
                sent(
                    &observed,
                    &render_party_ledger_master_request(HARNESS_COMPANY, &period)
                ),
                0,
                "the production request is not also sent"
            );

            let rows = items(&response);
            assert_eq!(rows.len(), 14);
            let mut with_rows = Vec::new();
            for row in rows {
                let details = &row["compliance"]["gst_details"];
                assert_eq!(details["observation"], "entries", "{row}");
                let entries = details["entries"].as_array().unwrap();
                if !entries.is_empty() {
                    with_rows.push(row["name"].as_str().unwrap().to_owned());
                }
                for entry in entries {
                    assert_eq!(entry["applicable_from"], "20260401");
                    assert_eq!(entry["other_content_skipped"], false);
                    // Tally's GSTINELIGIBLEITC goes out under the name of its element (#1741),
                    // and never under the earlier eligibility key.
                    assert!(entry.get("itc_eligible").is_none(), "{entry}");
                    assert!(
                        entry.as_object().unwrap().contains_key("ineligible_itc"),
                        "{entry}"
                    );
                    for state in entry["states"].as_array().unwrap() {
                        assert_eq!(
                            state["state_name"],
                            format!("{} Any", bridge_tally_protocol::TALLY_SANITIZED_ROOT_MARKER),
                            "the reserved state value keeps its marker"
                        );
                        assert_eq!(state["slab_rates_present"], false);
                        let rates = state["rates"].as_array().unwrap();
                        assert_eq!(rates.len(), 5);
                        for rate in rates {
                            if rate["duty_head"] == "Cess" {
                                assert_eq!(rate["rate"], json!({"kind":"absent"}), "{rate}");
                            }
                        }
                    }
                }
            }
            with_rows.sort();
            let mut expected = with_entries;
            expected.sort();
            assert_eq!(with_rows, expected);
            // The captured values, as sent: Yes on the Goods ledger's entry, no element on the
            // Purchase ledger's (null, never defaulted).
            let flag = |name: &str| {
                row(rows, name)["compliance"]["gst_details"]["entries"][0]["ineligible_itc"].clone()
            };
            assert_eq!(flag("BRIDGE Goods 18%"), json!(true));
            assert_eq!(flag("BRIDGE Purchase Svc 5%"), Value::Null);

            let rates_of = |name: &str, head: &str| -> Value {
                let rates = &row(rows, name)["compliance"]["gst_details"]["entries"][0]["states"]
                    [0]["rates"];
                rates
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|rate| rate["duty_head"] == head)
                    .unwrap_or_else(|| panic!("{head} missing from {rates}"))["rate"]
                    .clone()
            };
            // Raw rates keep their leading spaces; a missing GSTRATE is absent.
            assert_eq!(
                rates_of("BRIDGE Purchase Svc 5%", "CGST"),
                json!({"kind":"value","raw":" 2.50","decimal":"2.50"})
            );
            assert_eq!(
                rates_of("BRIDGE Goods 18%", "IGST"),
                json!({"kind":"value","raw":" 18","decimal":"18"})
            );
            assert_eq!(
                rates_of("BRIDGE CGST 2.5%", "CGST"),
                json!({"kind":"absent"}),
                "this ledger's entry carries no rate at all"
            );
            // The limitations ship with the rows.
            let limitations = response["structuredContent"]["result"]["limitations"]
                .as_array()
                .unwrap_or_else(|| panic!("no limitations: {response}"))
                .iter()
                .map(|line| line.as_str().unwrap())
                .collect::<Vec<_>>()
                .join(" ");
            for words in [
                "never merged",
                "HSN/SAC and type of supply are not returned",
                "place of supply is in the registration history",
                "slab rates are only flagged",
                "Do not read true as eligible",
                "says nothing either way",
                "14-ledger lab book of TallyPrime 7.1 Silver",
                "not yet run through this tool on a live Tally",
                "admitted at 8,000 bytes a ledger instead of 3,750",
                "not a measured ratio",
            ] {
                assert!(limitations.contains(words), "{words}: {limitations}");
            }
        }

        /// A rate `absent` in the response is `absent` in the output, and a
        /// rate that is zero is `zero`: the output keeps the parser's four
        /// states apart.
        #[tokio::test]
        async fn an_absent_rate_is_not_rendered_as_zero() {
            let capture = capture_for_the_harness_company();
            // The real bytes with one GSTRATE edited: ` 9` on the Goods
            // ledger's CGST row becomes `0.00`.
            let from = "<GSTRATE TYPE=\"Number\"> 9</GSTRATE>";
            assert!(capture.matches(from).count() >= 1);
            let edited = capture.replacen(from, "<GSTRATE TYPE=\"Number\">0.00</GSTRATE>", 1);
            let balances = balances_for(&capture_ledgers(&edited));
            let (response, _) = call_observed(
                compliance_plans(edited, balances),
                compliance_args(Some(true)),
                Redaction::None,
                200_000,
            )
            .await;
            let rows = items(&response);
            let rate = |name: &str, head: &str| -> Value {
                row(rows, name)["compliance"]["gst_details"]["entries"][0]["states"][0]["rates"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|rate| rate["duty_head"] == head)
                    .unwrap()["rate"]
                    .clone()
            };
            assert_eq!(rate("BRIDGE Goods 18%", "CGST"), json!({"kind":"zero"}));
            assert_eq!(rate("BRIDGE Goods 18%", "Cess"), json!({"kind":"absent"}));
            assert_eq!(
                rate("BRIDGE Goods 18%", "SGST/UTGST"),
                json!({"kind":"value","raw":" 9","decimal":"9"})
            );
        }

        /// The flag is for fields=compliance only. Any other `fields` (the
        /// default included) refuses it before any request, with the code and
        /// a remediation that names the fix.
        #[tokio::test]
        async fn the_flag_with_basic_fields_is_refused_before_any_request() {
            for args in [
                json!({"company_guid":GUID,"include_gst_details":true}),
                json!({"company_guid":GUID,"fields":"basic","include_gst_details":true}),
            ] {
                let (response, requests) = call(Vec::new(), args.clone()).await;
                assert_eq!(requests, 0, "{args}");
                let error = refusal(&response);
                assert_eq!(
                    error["code"], "ledger_masters_gst_details_requires_compliance",
                    "{args}"
                );
                assert!(error["remediation"]
                    .as_str()
                    .is_some_and(|text| text.contains("fields=compliance")));
            }
            // False is not a request for the details, so basic accepts it.
            let (response, _) = call(
                basic_plans(),
                json!({"company_guid":GUID,"fields":"basic","include_gst_details":false}),
            )
            .await;
            assert!(!items(&response).is_empty());
        }

        /// Only a JSON boolean is a flag.
        #[tokio::test]
        async fn a_flag_that_is_not_a_boolean_is_refused_before_any_request() {
            for flag in [
                json!("true"),
                json!("false"),
                json!(1),
                json!(0),
                json!(null),
                json!([]),
                json!({}),
            ] {
                for fields in ["compliance", "basic"] {
                    let args =
                        json!({"company_guid":GUID,"fields":fields,"include_gst_details":flag});
                    let (response, requests) = call(Vec::new(), args.clone()).await;
                    assert_eq!(requests, 0, "{args}");
                    assert_eq!(
                        refusal(&response)["code"],
                        "argument_invalid:include_gst_details",
                        "{args}"
                    );
                }
            }
        }

        /// The flag changes what is asked, so it changes what the call's
        /// evidence commits to: the same book read with and without it
        /// commits to different request sets.
        #[tokio::test]
        async fn the_request_commitment_differs_between_the_two_modes() {
            let evidence = |flag: bool| {
                let args = compliance_args(Some(flag));
                async move {
                    let (response, _) = call_observed(
                        compliance_plans(masters(), balances()),
                        args,
                        Redaction::None,
                        200_000,
                    )
                    .await;
                    assert_ne!(response["isError"], true, "{response}");
                    response["structuredContent"]["evidence"]["request_sha256"]
                        .as_str()
                        .unwrap()
                        .to_owned()
                }
            };
            let without = evidence(false).await;
            let with = evidence(true).await;
            assert_eq!(without, evidence(false).await, "the commitment is stable");
            assert_eq!(with, evidence(true).await, "the commitment is stable");
            assert_ne!(without, with);
        }

        /// A listing held for one mode is not served to a page asking for the
        /// other: the rows differ in whether they carry the details.
        #[tokio::test]
        async fn a_snapshot_of_one_mode_is_not_served_to_a_page_of_the_other() {
            let mut plans = compliance_plans(masters(), balances());
            plans.extend(continuation_plans(extent_with_master_mark(219)));
            plans.extend(
                compliance_plans(masters(), balances())
                    .into_iter()
                    .skip(identity_plans().len()),
            );
            let total = plans.len();
            let one = OneServer::spawn(plans);
            let first = one
                .call(json!({"company_guid":GUID,"fields":"compliance","include_gst_details":true,"limit":1}))
                .await;
            assert!(items(&first)[0]["compliance"]["gst_details"].is_object());
            let id = snapshot_id(&first);
            let other = one
                .call(json!({"company_guid":GUID,"fields":"compliance","offset":1,"limit":1}))
                .await;
            assert_eq!(snapshot_of(&other)["reused"], false);
            assert_ne!(snapshot_of(&other)["id"], id.as_str());
            for row in items(&other) {
                assert!(row["compliance"].get("gst_details").is_none(), "{row}");
            }
            assert_eq!(one.requests(), total);
        }

        /// The other direction: a listing held for a read without the flag is
        /// not served to a later page that asks for the details, which reads
        /// afresh and carries `gst_details` and the limitations.
        #[tokio::test]
        async fn a_snapshot_without_the_details_is_not_served_to_a_page_that_asks_for_them() {
            let mut plans = compliance_plans(masters(), balances());
            plans.extend(continuation_plans(extent_with_master_mark(219)));
            plans.extend(
                compliance_plans(masters(), balances())
                    .into_iter()
                    .skip(identity_plans().len()),
            );
            let total = plans.len();
            let one = OneServer::spawn(plans);
            let first = one
                .call(json!({"company_guid":GUID,"fields":"compliance","limit":1}))
                .await;
            assert!(!items(&first).is_empty());
            for row in items(&first) {
                assert!(row["compliance"].get("gst_details").is_none(), "{row}");
            }
            let id = snapshot_id(&first);
            let other = one
                .call(json!({"company_guid":GUID,"fields":"compliance","include_gst_details":true,"offset":1,"limit":1}))
                .await;
            assert_eq!(snapshot_of(&other)["reused"], false);
            assert_ne!(snapshot_of(&other)["id"], id.as_str());
            assert!(!items(&other).is_empty());
            for row in items(&other) {
                assert!(row["compliance"]["gst_details"].is_object(), "{row}");
            }
            assert!(
                other["structuredContent"]["result"]["limitations"].is_array(),
                "{other}"
            );
            assert_eq!(one.requests(), total);
        }

        /// A second page of the same mode, naming its snapshot, is served from
        /// the held read, and its rows carry `gst_details` and the result
        /// carries the limitations, as the first page's do.
        #[tokio::test]
        async fn a_second_page_of_the_same_details_mode_is_served_from_the_held_listing() {
            let mut plans = compliance_plans(masters(), balances());
            plans.extend(continuation_plans(extent_with_master_mark(219)));
            let total = plans.len();
            let one = OneServer::spawn(plans);
            let args = |offset: usize, snapshot: Option<&str>| {
                let mut args = json!({"company_guid":GUID,"fields":"compliance","include_gst_details":true,"offset":offset,"limit":1});
                if let Some(snapshot) = snapshot {
                    args["snapshot_id"] = json!(snapshot);
                }
                args
            };
            let first = one.call(args(0, None)).await;
            let id = snapshot_id(&first);
            assert_eq!(snapshot_of(&first)["reused"], false);
            let second = one.call(args(1, Some(&id))).await;
            assert_eq!(snapshot_of(&second)["reused"], true);
            assert_eq!(snapshot_of(&second)["id"], id.as_str());
            for page in [&first, &second] {
                assert!(!items(page).is_empty());
                for row in items(page) {
                    assert!(row["compliance"]["gst_details"].is_object(), "{row}");
                }
                let limitations = page["structuredContent"]["result"]["limitations"]
                    .as_array()
                    .unwrap_or_else(|| panic!("no limitations: {page}"));
                assert_eq!(limitations.len(), 6, "{page}");
            }
            assert_ne!(items(&first)[0]["name"], items(&second)[0]["name"]);
            assert_eq!(one.requests(), total, "the second page reads no ledger");
        }

        /// A masking redaction marks the ledger names, and nothing else in the
        /// details changes: they hold rates, dates, heads and a state name, no
        /// party text, so `mask_parties` leaves them exactly as they are.
        #[tokio::test]
        async fn masking_parties_leaves_the_gst_details_as_they_are() {
            let read = |redaction| async move {
                call_observed(
                    capture_plans(),
                    compliance_args(Some(true)),
                    redaction,
                    200_000,
                )
                .await
                .0
            };
            let plain = read(Redaction::None).await;
            let masked = read(Redaction::MaskParties).await;
            let plain_rows = items(&plain);
            let masked_rows = items(&masked);
            assert_eq!(plain_rows.len(), masked_rows.len());
            let mut ledger_names_masked = 0;
            for (plain, masked) in plain_rows.iter().zip(masked_rows) {
                if plain["name"] != masked["name"] {
                    ledger_names_masked += 1;
                }
                assert!(plain["compliance"]["gst_details"].is_object(), "{plain}");
                assert_eq!(
                    plain["compliance"]["gst_details"], masked["compliance"]["gst_details"],
                    "{plain}"
                );
            }
            assert!(ledger_names_masked > 0, "the control: names are masked");
        }

        /// bridge#1490, through the handler: under `mask_parties` the probe party's
        /// mailing address lines come back masked beside its masked name, the PIN
        /// code, state and country stay readable, and under `none` the same call
        /// returns the lines as Tally sent them. The control is the row's own name.
        #[tokio::test]
        async fn masking_parties_masks_the_mailing_address_lines_through_the_tool() {
            // The captured answer gives the probe party no flat address (Tally 7.1 leaves it
            // empty), so one is written into its row here: a first line that repeats its name.
            let masters = {
                let capture = mailing_masters();
                let probe = capture
                    .find("<LEDGER NAME=\"Lab Party Duplicate Probe\"")
                    .expect("the probe row");
                let empty = "<LEDADDRESS.LIST>     </LEDADDRESS.LIST>";
                let at = probe + capture[probe..].find(empty).expect("its empty address");
                format!(
                    "{}<LEDADDRESS.LIST><LEDADDRESS>Lab Party Duplicate Probe</LEDADDRESS><LEDADDRESS>9 Test Lane</LEDADDRESS></LEDADDRESS.LIST>{}",
                    &capture[..at],
                    &capture[at + empty.len()..]
                )
            };
            let read = |redaction| {
                let masters = masters.clone();
                async move {
                    let balances = balances_matching(&masters);
                    call_observed(
                        compliance_plans(masters, balances),
                        json!({"company_guid":GUID,"fields":"compliance","include_mailing_details":true}),
                        redaction,
                        200_000,
                    )
                    .await
                    .0
                }
            };
            let probe = |response: &Value| {
                items(response)
                    .iter()
                    .find(|row| {
                        row["name"] == json!("Lab Party Duplicate Probe")
                            || row["name"] == json!(crate::agent::mask("Lab Party Duplicate Probe"))
                    })
                    .cloned()
                    .expect("the probe row")
            };
            let plain = probe(&read(Redaction::None).await);
            let masked = probe(&read(Redaction::MaskParties).await);
            assert_eq!(plain["name"], json!("Lab Party Duplicate Probe"));
            assert_eq!(
                masked["name"],
                json!(crate::agent::mask("Lab Party Duplicate Probe")),
                "the control: the name is masked"
            );
            let entry = |row: &Value| row["compliance"]["mailing_details"]["entries"][0].clone();
            assert_eq!(
                entry(&plain)["address_lines"],
                json!(["1 Test Street", "Test Area"])
            );
            assert_eq!(
                entry(&masked)["address_lines"],
                json!([
                    crate::agent::mask("1 Test Street"),
                    crate::agent::mask("Test Area")
                ])
            );
            for field in ["pin_code", "state", "country", "applicable_from"] {
                assert_eq!(entry(&plain)[field], entry(&masked)[field], "{field}");
            }
            // The mailing name and the flat address go through the tool the same way.
            assert_eq!(
                entry(&plain)["mailing_name"],
                json!("Lab Party Duplicate Probe")
            );
            assert_eq!(
                entry(&masked)["mailing_name"],
                json!(crate::agent::mask("Lab Party Duplicate Probe"))
            );
            let flat = "Lab Party Duplicate Probe\n9 Test Lane";
            assert_eq!(plain["compliance"]["address"], json!(flat));
            assert_eq!(
                masked["compliance"]["address"],
                json!(crate::agent::mask(flat))
            );
        }

        /// bridge#1490: the dated mailing details are read only when asked for. Without
        /// `include_mailing_details` (absent or false) the master read is the production
        /// request, byte for byte, and no row carries a `mailing_details` key; with it the
        /// read names the list and every row carries the key.
        #[tokio::test]
        async fn the_mailing_details_are_read_and_returned_only_when_asked_for() {
            let period = harness_period();
            for flag in [None, Some(false)] {
                let args = match flag {
                    None => json!({"company_guid":GUID,"fields":"compliance"}),
                    Some(flag) => {
                        json!({"company_guid":GUID,"fields":"compliance","include_mailing_details":flag})
                    }
                };
                let plans = compliance_plans(masters(), balances());
                let total = plans.len();
                let (response, observed) =
                    call_observed(plans, args, Redaction::None, 200_000).await;
                assert_eq!(observed.len(), total, "{flag:?}");
                assert_eq!(
                    sent(
                        &observed,
                        &render_party_ledger_master_request(HARNESS_COMPANY, &period)
                    ),
                    2,
                    "the production request, read as a pair: {flag:?}"
                );
                assert_eq!(
                    sent(
                        &observed,
                        &render_party_ledger_master_request_with_mailing_details(
                            HARNESS_COMPANY,
                            &period
                        )
                    ),
                    0,
                    "{flag:?}"
                );
                for row in items(&response) {
                    let compliance = row["compliance"].as_object().unwrap();
                    assert!(
                        !compliance.contains_key("mailing_details"),
                        "{flag:?}: {row}"
                    );
                    assert!(compliance.contains_key("gst_registrations"), "{row}");
                }
            }
            let masters = mailing_masters();
            let balances = balances_matching(&masters);
            let (response, observed) = call_observed(
                compliance_plans(masters, balances),
                json!({"company_guid":GUID,"fields":"compliance","include_mailing_details":true}),
                Redaction::None,
                200_000,
            )
            .await;
            assert_eq!(
                sent(
                    &observed,
                    &render_party_ledger_master_request_with_mailing_details(
                        HARNESS_COMPANY,
                        &period
                    )
                ),
                2,
                "the request that names the mailing details, read as a pair"
            );
            for row in items(&response) {
                assert!(row["compliance"].get("mailing_details").is_some(), "{row}");
            }
            let lead = &response["structuredContent"]["result"]["limitations"];
            assert!(
                lead.to_string()
                    .contains("admitted at 5,000 bytes a ledger"),
                "{lead}"
            );
        }

        /// bridge#1490: the flag is refused with any other `fields`, and together with
        /// `include_gst_details`, before any read.
        #[tokio::test]
        async fn the_mailing_details_flag_is_refused_where_it_cannot_be_served() {
            for (args, code) in [
                (
                    json!({"company_guid":GUID,"include_mailing_details":true}),
                    "ledger_masters_mailing_details_requires_compliance",
                ),
                (
                    json!({"company_guid":GUID,"fields":"compliance","include_mailing_details":true,"include_gst_details":true}),
                    "ledger_masters_mailing_details_with_gst_details_not_captured",
                ),
                (
                    json!({"company_guid":GUID,"fields":"compliance","include_mailing_details":"yes"}),
                    "argument_invalid:include_mailing_details",
                ),
            ] {
                let (response, observed) =
                    call_observed(Vec::new(), args, Redaction::None, 200_000).await;
                assert!(observed.is_empty(), "{code}: no read is sent");
                assert!(response.to_string().contains(code), "{code}: {response}");
            }
        }

        // -- admission: the details read is sized by its own estimate --------

        const BIG: &str = "Sundry Debtors";
        const OTHER: &str = "Sundry Creditors";

        /// A master mark that every count here is taken under: past what one
        /// standard read admits by the mark, within what the catalogue counts.
        const MARK: u64 = 6_000;

        /// `count` ledgers under two parents, `BIG` first. `BIG` holds the
        /// larger share, so a part packed largest-first never joins them when
        /// together they exceed a part.
        fn book(count: u64) -> Vec<Generated> {
            let big = count / 2 + 1;
            generated(&[(BIG, big as usize), (OTHER, (count - big) as usize)])
        }

        /// The parts the details read plans for `rows`, under the limits the
        /// production read uses for it.
        fn planned_parts(rows: &[Generated]) -> Vec<ParentPart> {
            let guids = rows
                .iter()
                .map(|row| format!("guid-{}", row.index))
                .collect::<Vec<_>>();
            ParentPartition::plan(
                rows.iter().zip(&guids).map(|(row, guid)| {
                    (
                        row.name.as_str(),
                        guid.as_str(),
                        ParentObservation::Named(row.parent),
                    )
                }),
                parent_partition_limits(ComplianceRead::WithGstDetails),
            )
            .unwrap()
            .parts()
            .to_vec()
        }

        fn whole_read_plans(rows: &[Generated]) -> Vec<ScenarioPlan> {
            let all = rows.iter().collect::<Vec<_>>();
            marked_compliance_plans(
                MARK,
                vec![
                    generated_catalogue(&all),
                    generated_masters(&all),
                    generated_balances(&all),
                    groups(),
                ],
                Some(extent_with_master_mark(MARK)),
            )
        }

        fn parts_read_plans(rows: &[Generated], parents_per_part: &[&[&str]]) -> Vec<ScenarioPlan> {
            let all = rows.iter().collect::<Vec<_>>();
            let mut reads = vec![generated_catalogue(&all)];
            for parents in parents_per_part {
                let part = under(rows, parents);
                reads.push(generated_masters(&part));
                reads.push(generated_balances(&part));
            }
            reads.push(groups());
            marked_compliance_plans(MARK, reads, Some(extent_with_master_mark(MARK)))
        }

        /// The books below sit between the two limits: they fit one standard
        /// read and not one details read, and their mark is past both so that
        /// either read counts the book first.
        #[test]
        fn the_books_chosen_for_the_boundary_tests_sit_between_the_two_limits() {
            let standard = compliance_master_ledgers_per_read(ComplianceRead::Standard);
            let with = compliance_master_ledgers_per_read(ComplianceRead::WithGstDetails);
            assert!(with < standard);
            assert!(std::hint::black_box(MARK) > standard);
        }

        /// A book one ledger over what a details read holds is one whole read
        /// without the flag and a read in parts with it, each part requested
        /// with the details; a book of exactly what it holds is one whole
        /// details read.
        #[tokio::test]
        async fn a_book_that_fits_a_standard_read_but_not_a_details_read_is_read_in_parts_with_the_flag(
        ) {
            let with = compliance_master_ledgers_per_read(ComplianceRead::WithGstDetails);
            let period = harness_period();

            // Without the flag: one whole standard read.
            let rows = book(with + 1);
            let plans = whole_read_plans(&rows);
            let total = plans.len();
            let (response, observed) =
                call_observed(plans, compliance_args(None), Redaction::None, 2_000_000).await;
            assert_ne!(response["isError"], true, "{response}");
            assert_eq!(observed.len(), total);
            assert_eq!(response["structuredContent"]["result"]["total"], with + 1);
            assert_eq!(
                sent(
                    &observed,
                    &render_party_ledger_master_request(HARNESS_COMPANY, &period)
                ),
                2
            );

            // With the flag: parts, each by its parents, each with the details.
            let parts = planned_parts(&rows);
            assert_eq!(parts.len(), 2, "the book does not fit one details read");
            let plans = parts_read_plans(&rows, &[&[BIG], &[OTHER]]);
            let total = plans.len();
            let (response, observed) = call_observed(
                plans,
                compliance_args(Some(true)),
                Redaction::None,
                2_000_000,
            )
            .await;
            assert_ne!(response["isError"], true, "{response}");
            assert_eq!(observed.len(), total, "{response}");
            assert_eq!(response["structuredContent"]["result"]["total"], with + 1);
            for part in &parts {
                assert_eq!(
                    sent(
                        &observed,
                        &render_party_ledger_master_request_for_parents_with_gst_details(
                            HARNESS_COMPANY,
                            &period,
                            part
                        )
                    ),
                    2,
                    "each part is requested with the details, as a pair"
                );
                assert_eq!(
                    sent(
                        &observed,
                        &render_party_ledger_master_request_for_parents(
                            HARNESS_COMPANY,
                            &period,
                            part
                        )
                    ),
                    0,
                    "no part is requested without them"
                );
            }
            assert_eq!(
                master_reads_sent(&observed, &parts),
                2 * parts.len(),
                "no other master read was sent"
            );

            // Exactly what a details read holds: one whole details read.
            let rows = book(with);
            let plans = whole_read_plans(&rows);
            let total = plans.len();
            let (response, observed) = call_observed(
                plans,
                compliance_args(Some(true)),
                Redaction::None,
                2_000_000,
            )
            .await;
            assert_ne!(response["isError"], true, "{response}");
            assert_eq!(observed.len(), total);
            assert_eq!(response["structuredContent"]["result"]["total"], with);
            assert_eq!(
                sent(
                    &observed,
                    &render_party_ledger_master_request_with_gst_details(HARNESS_COMPANY, &period)
                ),
                2
            );
            assert_eq!(master_reads_sent(&observed, &[]), 2);
        }

        /// A mark one standard read holds and a details read does not is
        /// counted first when the details are asked for (the catalogue pair is
        /// sent), and admitted by the mark alone when they are not.
        #[tokio::test]
        async fn a_mark_that_a_standard_read_admits_is_counted_first_when_the_details_are_asked_for(
        ) {
            let with = compliance_master_ledgers_per_read(ComplianceRead::WithGstDetails);
            let mark = with + 1;
            let rows = generated(&[(BIG, 10)]);
            let all = rows.iter().collect::<Vec<_>>();
            let period = harness_period();

            let plans = marked_compliance_plans(
                mark,
                vec![generated_masters(&all), generated_balances(&all), groups()],
                Some(extent_with_master_mark(mark)),
            );
            let total = plans.len();
            let (response, observed) =
                call_observed(plans, compliance_args(None), Redaction::None, 200_000).await;
            assert_ne!(response["isError"], true, "{response}");
            assert_eq!(observed.len(), total, "no catalogue is read");

            let plans = marked_compliance_plans(
                mark,
                vec![
                    generated_catalogue(&all),
                    generated_masters(&all),
                    generated_balances(&all),
                    groups(),
                ],
                Some(extent_with_master_mark(mark)),
            );
            let total = plans.len();
            let (response, observed) =
                call_observed(plans, compliance_args(Some(true)), Redaction::None, 200_000).await;
            assert_ne!(response["isError"], true, "{response}");
            assert_eq!(observed.len(), total, "the catalogue pair is read first");
            assert_eq!(
                sent(
                    &observed,
                    &render_party_ledger_master_request_with_gst_details(HARNESS_COMPANY, &period)
                ),
                2
            );
        }

        /// A count taken by AlterID span is sized by the read kind too: the
        /// same census count is one whole read without the flag and parts with
        /// it, after the catalogue that names the parents.
        #[tokio::test]
        async fn a_census_count_that_a_standard_read_holds_is_read_in_parts_when_the_details_are_asked_for(
        ) {
            let with = compliance_master_ledgers_per_read(ComplianceRead::WithGstDetails);
            let mark = 30_000_u64;
            let count = (with + 1) as u32;
            let rows = book(u64::from(count));
            let all = rows.iter().collect::<Vec<_>>();
            let period = harness_period();

            let plans = census_plans_checked(
                mark,
                census_bodies(mark, GUID, &[(0, 0..count)]),
                vec![generated_masters(&all), generated_balances(&all), groups()],
                Some(extent_with_master_mark(mark)),
            );
            let total = plans.len();
            let (response, observed) =
                call_observed(plans, compliance_args(None), Redaction::None, 2_000_000).await;
            assert_ne!(response["isError"], true, "{response}");
            assert_eq!(
                observed.len(),
                total,
                "one whole standard read, no catalogue"
            );

            let mut plans = census_plans_checked(
                mark,
                census_bodies(mark, GUID, &[(0, 0..count)]),
                vec![generated_catalogue(&all)],
                None,
            );
            for parents in [[BIG], [OTHER]] {
                let part = under(&rows, &parents);
                pair(&mut plans, xml(generated_masters(&part)));
                pair(&mut plans, xml(generated_balances(&part)));
            }
            pair(&mut plans, xml(groups()));
            pair(&mut plans, xml(extent_with_master_mark(mark)));
            plans.extend([xml(companies()), status(), xml(companies())]);
            let total = plans.len();
            let (response, observed) = call_observed(
                plans,
                compliance_args(Some(true)),
                Redaction::None,
                2_000_000,
            )
            .await;
            assert_ne!(response["isError"], true, "{response}");
            assert_eq!(observed.len(), total);
            let parts = planned_parts(&rows);
            assert_eq!(parts.len(), 2);
            for part in &parts {
                assert_eq!(
                    sent(
                        &observed,
                        &render_party_ledger_master_request_for_parents_with_gst_details(
                            HARNESS_COMPANY,
                            &period,
                            part
                        )
                    ),
                    2
                );
            }
        }

        /// A parent holding more ledgers than a details part holds cannot be
        /// split, so it is refused as the existing over-budget refusal, before
        /// any master request. The same parent is read whole without the flag.
        #[tokio::test]
        async fn a_parent_too_large_for_a_details_part_is_refused_not_truncated() {
            let with = compliance_master_ledgers_per_read(ComplianceRead::WithGstDetails);
            let rows = generated(&[(BIG, (with + 1) as usize)]);
            let all = rows.iter().collect::<Vec<_>>();
            let plans = marked_compliance_plans(MARK, vec![generated_catalogue(&all)], None);
            let total = plans.len();
            let (response, observed) = call_observed(
                plans,
                compliance_args(Some(true)),
                Redaction::None,
                2_000_000,
            )
            .await;
            assert_eq!(observed.len(), total, "nothing is sent after the catalogue");
            let error = refusal(&response);
            assert_eq!(error["code"], "party_ledger_master_read_failed");
            assert_eq!(error["cause"], "parent_over_budget");
            assert_eq!(master_reads_sent(&observed, &[]), 0);

            let plans = whole_read_plans(&rows);
            let total = plans.len();
            let (response, observed) =
                call_observed(plans, compliance_args(None), Redaction::None, 2_000_000).await;
            assert_ne!(response["isError"], true, "{response}");
            assert_eq!(observed.len(), total);
        }
    }
}
