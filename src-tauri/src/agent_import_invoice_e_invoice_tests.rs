use super::EInvoiceDetails;
use serde_json::{json, Value};

/// The details row 2162-IRN9 imported with invoice `3` on `BRIDGE EINVOICE LAB`, which Tally stored
/// as sent (`lab/2162-irn-import-sheet` at 463fbea9, `I-3.xml`; answers `lab/2162-irn-import-answers`
/// at 9cb3ebd0, `s3r`).
const IRN9_IRN: &str = "c3d4e5f6a7b8c9d0c3d4e5f6a7b8c9d0c3d4e5f6a7b8c9d0c3d4e5f6a7b8c9d0";

fn irn9() -> EInvoiceDetails {
    serde_json::from_value(json!({"irn": IRN9_IRN, "ack_no": "445566", "ack_date": "2025-04-03"}))
        .expect("IRN9's details parse")
}

fn parsed(value: Value) -> Result<EInvoiceDetails, serde_json::Error> {
    serde_json::from_value(value)
}

/// The three elements are written exactly as row 2162-IRN9's `I-3.xml` carried them after
/// `<ISINVOICE>Yes</ISINVOICE>`, the request Tally stored as sent.
#[test]
fn the_details_render_as_irn9_imported_them() {
    assert_eq!(
        irn9().render(),
        format!(
            "<IRN>{IRN9_IRN}</IRN><IRNACKNO>445566</IRNACKNO><IRNACKDATE>20250403</IRNACKDATE>"
        )
    );
}

/// A saved invoice keeps the details in the form they were given, so the post re-renders the
/// same bytes from the saved batch alone.
#[test]
fn the_details_round_trip_in_the_form_given() {
    let details = irn9();
    let saved = serde_json::to_value(&details).expect("serialises");
    assert_eq!(
        saved,
        json!({"irn": IRN9_IRN, "ack_no": "445566", "ack_date": "2025-04-03"})
    );
    assert_eq!(parsed(saved).expect("reads back"), details);
}

/// Each value is admitted only in its closed form, so nothing a caller types can end the element it
/// is written into; all three are required and nothing else is accepted.
#[test]
fn each_value_outside_its_form_is_refused() {
    let with = |key: &str, value: Value| {
        let mut details = json!({"irn": IRN9_IRN, "ack_no": "445566", "ack_date": "2025-04-03"});
        details[key] = value;
        parsed(details)
    };
    for irn in [
        IRN9_IRN.to_uppercase(),
        IRN9_IRN[..63].to_string(),
        format!("{IRN9_IRN}0"),
        format!("{}g", &IRN9_IRN[..63]),
        format!("{}<", &IRN9_IRN[..63]),
        String::new(),
    ] {
        assert!(with("irn", json!(irn)).is_err(), "{irn}");
    }
    for ack_no in ["", "12a", "123456789012345678901", " 445566", "4455-66"] {
        assert!(with("ack_no", json!(ack_no)).is_err(), "{ack_no}");
    }
    assert!(with("ack_no", json!("12345678901234567890")).is_ok());
    for ack_date in [
        "20250403",
        "2025-4-3",
        "2025-02-30",
        "2025-13-01",
        "2025/04/03",
        "2025-04-03 10:00",
    ] {
        assert!(with("ack_date", json!(ack_date)).is_err(), "{ack_date}");
    }
    assert!(with("ack_no", json!(445_566)).is_err());
    assert!(parsed(json!({"irn": IRN9_IRN, "ack_no": "445566"})).is_err());
    assert!(parsed(json!({
        "irn": IRN9_IRN, "ack_no": "445566", "ack_date": "2025-04-03", "qr_code": "x"
    }))
    .is_err());
}

/// The approval text shows every detail as it goes into the invoice, inside the dialog's width.
#[test]
fn the_review_lines_show_every_detail() {
    let lines = irn9().review_lines();
    assert_eq!(lines[0], format!("E-invoice IRN: {IRN9_IRN}"));
    assert_eq!(lines[1], "E-invoice ack: 445566 of 2025-04-03");
    assert_eq!(lines[0].chars().count(), 79);
    // The longest acknowledgement, a 20-digit number, is 49 characters: with the longer bill
    // allocation (46) and two spaces, the dialog's line is 97, inside its 100.
    let longest = parsed(json!({
        "irn": IRN9_IRN, "ack_no": "12345678901234567890", "ack_date": "2025-04-03"
    }))
    .expect("parses")
    .review_lines();
    assert_eq!(longest[1].chars().count(), 49);
}

/// The e-invoice read is byte for byte the window request the `vouchers` planner renders for a
/// day with `include_e_invoice` and no voucher type (§8.2j), over the invoice's one date: no new
/// request shape.
#[test]
fn the_e_invoice_read_is_the_measured_vouchers_read_over_one_date() {
    let date = bridge_tally_core::TallyDate::parse("20250403".to_string()).expect("date");
    let request =
        crate::agent::invoice_e_invoice_read("BRIDGE EINVOICE LAB", &date).expect("renders");
    let asked = crate::agent::NamedVoucherFields {
        e_invoice: crate::agent::EInvoiceFields::Asked,
        gst: crate::agent::VoucherGstFields::NotAsked,
    };
    let shape = crate::agent::VoucherReadShape::EntryWildcard.asking(asked);
    assert_eq!(shape.named_fields(), asked);
    let vouchers_call = shape
        .render("BRIDGE EINVOICE LAB", &date, &date, None)
        .expect("renders");
    assert_eq!(request.as_str(), vouchers_call);
    assert!(vouchers_call.contains(",IRN,IRNACKNO,IRNACKDATE</FETCH>"));
    assert!(vouchers_call.contains("<SVFROMDATE TYPE=\"Date\">20250403</SVFROMDATE>"));
}

/// The rows of a captured answer on `BRIDGE EINVOICE LAB` (`einvoice-lab-dispatch`, row 2027-D2):
/// Sales `1` keyed with details, Sales `2` without, and a Payment, read as the e-invoice read's
/// own parse reads them. DERIVED: this answer is to the class-shape read that also names the GST
/// words and a voucher type. The read the post sends was answered in full on the same book
/// (§8.2j), but that answer is not committed, so these tests rest on the three elements, direct
/// children of `VOUCHER`, coming back the same in both.
fn captured_rows() -> Vec<Value> {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/einvoice-lab-dispatch/window-read-class-e-invoice-dispatch.utf16le.xml"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .expect("UTF-16LE");
    super::read_rows(&xml, "eb2f5190-292e-4ed8-bfd3-eace839fe699")
        .expect("the captured answer parses")
}

/// Sales `1`'s details as that answer carries them, typed on Tally's screen.
fn keyed() -> EInvoiceDetails {
    parsed(json!({
        "irn": "a1b2c3d4e5f6a7b8a1b2c3d4e5f6a7b8a1b2c3d4e5f6a7b8a1b2c3d4e5f6a7b8",
        "ack_no": "112233",
        "ack_date": "2025-04-03"
    }))
    .expect("the keyed details parse")
}

const SALES_1: &str = "eb2f5190-292e-4ed8-bfd3-eace839fe699-00000001";
const SALES_2: &str = "eb2f5190-292e-4ed8-bfd3-eace839fe699-00000002";

/// The invoice whose details the answer holds, found by its GUID in any case, has no difference.
#[test]
fn details_read_back_as_sent_have_no_difference() {
    let rows = captured_rows();
    assert!(keyed().readback_differences(&rows, SALES_1).is_empty());
    assert!(keyed()
        .readback_differences(&rows, &SALES_1.to_uppercase())
        .is_empty());
}

/// Each element whose text is not the one sent is named, and only that one.
#[test]
fn each_element_read_back_otherwise_is_a_difference() {
    let rows = captured_rows();
    let changed = |key: &str, value: &str| {
        let mut details = serde_json::to_value(keyed()).expect("serialises");
        details[key] = json!(value);
        parsed(details).expect("parses")
    };
    assert_eq!(
        changed("irn", IRN9_IRN).readback_differences(&rows, SALES_1),
        ["IRN"]
    );
    assert_eq!(
        changed("ack_no", "445566").readback_differences(&rows, SALES_1),
        ["IRNACKNO"]
    );
    assert_eq!(
        changed("ack_date", "2025-04-04").readback_differences(&rows, SALES_1),
        ["IRNACKDATE"]
    );
}

/// An element Tally sent empty, as on the invoice without details, is never equal to a value sent.
#[test]
fn an_empty_element_is_a_difference() {
    assert_eq!(
        keyed().readback_differences(&captured_rows(), SALES_2),
        ["IRN", "IRNACKNO", "IRNACKDATE"]
    );
}

/// A row that says nothing of the three (a read that did not name them) is never equal either.
#[test]
fn a_row_without_the_details_is_a_difference() {
    let rows = captured_rows()
        .into_iter()
        .map(|mut row| {
            row.as_object_mut().expect("object").remove("e_invoice");
            row
        })
        .collect::<Vec<_>>();
    assert_eq!(
        keyed().readback_differences(&rows, SALES_1),
        ["IRN", "IRNACKNO", "IRNACKDATE"]
    );
}

/// No row with the GUID, or more than one, is a difference of its own, never a pass.
#[test]
fn a_missing_or_repeated_voucher_is_a_difference() {
    let rows = captured_rows();
    assert_eq!(
        keyed().readback_differences(&rows, "eb2f5190-292e-4ed8-bfd3-eace839fe699-00000009"),
        ["e_invoice_not_found"]
    );
    assert_eq!(
        keyed().readback_differences(&[], SALES_1),
        ["e_invoice_not_found"]
    );
    let twice = rows.iter().chain(rows.iter()).cloned().collect::<Vec<_>>();
    assert_eq!(
        keyed().readback_differences(&twice, SALES_1),
        ["e_invoice_not_unique"]
    );
}
