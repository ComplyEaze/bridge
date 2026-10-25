// Goldens are the decoded text of the three lab masters requests (synthetic
// names and a synthetic GSTIN, company `BRIDGE MASTERS LAB`). Their digests
// were taken with `shasum -a 256` over the text without its trailing newline,
// not with this module's hasher.
use super::*;

const LAB_COMPANY: &str = "BRIDGE MASTERS LAB";
const LAB_GSTIN: &str = "08ZZZZZ0000Z2ZP";

const PARTY_GOLDEN: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>BRIDGE MASTERS LAB</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE><LEDGER NAME=\"MLab M1 Party\" ACTION=\"Create\"><NAME>MLab M1 Party</NAME><PARENT>Sundry Debtors</PARENT><ISBILLWISEON>Yes</ISBILLWISEON><PARTYGSTIN>08ZZZZZ0000Z2ZP</PARTYGSTIN></LEDGER></TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>";
const PARTY_SHA: &str = "fecd63a0d9ee5c27fe0b0732c7e3915d5900e13e39c04b0499af7322eb5f581c";

const SUSPENSE_GOLDEN: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>BRIDGE MASTERS LAB</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE><LEDGER NAME=\"MLab M1 Suspense\" ACTION=\"Create\"><NAME>MLab M1 Suspense</NAME><PARENT>Suspense A/c</PARENT><ISBILLWISEON>No</ISBILLWISEON></LEDGER></TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>";
const SUSPENSE_SHA: &str = "e9cc3e00113412a8181f02b98ca5b236974d6de799f4ab4f4c474f9501a41e25";

const DUTY_GOLDEN: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>BRIDGE MASTERS LAB</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE><LEDGER NAME=\"MLab M1 Duty SGST UTGST\" ACTION=\"Create\"><NAME>MLab M1 Duty SGST UTGST</NAME><PARENT>Duties &amp; Taxes</PARENT><ISBILLWISEON>No</ISBILLWISEON><TAXTYPE>GST</TAXTYPE><GSTDUTYHEAD>SGST/UTGST</GSTDUTYHEAD></LEDGER></TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>";
const DUTY_SHA: &str = "4f34aa92404c463a93dd1aa269a837b3f2da632b6545ecc04f600e242bb9cef7";

/// The verified identity the dispatch takes, built with the crate's test
/// constructor (`VerifiedCompanyIdentity::test_fixture`).
fn lab() -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture(LAB_COMPANY, crate::synthetic_guid(1))
}

fn name(value: &str) -> MasterName {
    MasterName::new(value).expect("a valid name")
}

fn party(side: PartySide, gstin: Option<&str>, bill_wise: bool) -> AdmittedLedgerKind {
    AdmittedLedgerKind::Party {
        side,
        gstin: gstin.map(|g| VerifiedGstin::new(g).expect("a valid GSTIN")),
        bill_wise,
    }
}

fn duty() -> AdmittedLedgerKind {
    AdmittedLedgerKind::GstDuty {
        head: AdmittedDutyHead::new(GstDutyHead::SgstUtgst).expect("admitted"),
    }
}

/// The element names of `xml` in document order, read by walking `<...>`
/// tokens rather than by substring.
fn element_names(xml: &str) -> Vec<String> {
    xml.split('<')
        .skip(1)
        .filter_map(|token| {
            let tag = token.split('>').next()?;
            if tag.starts_with('/') || tag.starts_with('?') {
                return None;
            }
            Some(tag.split_whitespace().next()?.to_string())
        })
        .collect()
}

// --- MasterName -----------------------------------------------------------

#[test]
fn a_plain_ascii_name_is_accepted_and_kept_verbatim() {
    assert_eq!(
        name("Acme Traders - Delhi (No. 1)/A").as_str(),
        "Acme Traders - Delhi (No. 1)/A"
    );
}

#[test]
fn an_empty_name_is_refused() {
    assert_eq!(MasterName::new(""), Err(MasterNameError::Empty));
}

#[test]
fn a_non_ascii_name_is_refused() {
    for value in [
        "Caf\u{e9}",
        "\u{928}\u{92e}",
        "A\u{a0}B",
        "A\u{200b}B",
        "\u{feff}A",
    ] {
        assert_eq!(
            MasterName::new(value),
            Err(MasterNameError::NonAscii),
            "{value:?}"
        );
    }
}

#[test]
fn every_ascii_control_character_is_refused() {
    for code in (0x00u8..=0x1F).chain([0x7F]) {
        let value = format!("A{}B", char::from(code));
        assert_eq!(
            MasterName::new(&value),
            Err(MasterNameError::ControlCharacter),
            "0x{code:02X}"
        );
    }
}

#[test]
fn leading_or_trailing_whitespace_is_refused() {
    for value in [" A", "A ", " ", "  "] {
        assert_eq!(
            MasterName::new(value),
            Err(MasterNameError::SurroundingWhitespace),
            "{value:?}"
        );
    }
}

#[test]
fn a_run_of_two_or_more_spaces_is_refused() {
    for value in ["A  B", "A   B"] {
        assert_eq!(
            MasterName::new(value),
            Err(MasterNameError::RepeatedSpaces),
            "{value:?}"
        );
    }
}

#[test]
fn the_length_cap_is_the_master_binding_cap_and_is_inclusive() {
    assert_eq!(MAX_NAME_CHARS, 16_384);
    assert!(MasterName::new(&"A".repeat(MAX_NAME_CHARS)).is_ok());
    assert_eq!(
        MasterName::new(&"A".repeat(MAX_NAME_CHARS + 1)),
        Err(MasterNameError::TooLong)
    );
}

// --- VerifiedGstin --------------------------------------------------------

#[test]
fn a_gstin_with_a_correct_check_character_is_accepted() {
    // Synthetic-family values whose check characters were computed outside
    // this module (a separate script), one per state code and entity digit.
    assert_eq!(VerifiedGstin::new(LAB_GSTIN).unwrap().as_str(), LAB_GSTIN);
    for value in ["08ZZZZZ0000Z1ZQ", "27ZZZZZ0000Z1ZQ", "29ZZZZZ0000Z3ZK"] {
        assert!(VerifiedGstin::new(value).is_ok(), "{value}");
    }
}

#[test]
fn a_gstin_with_a_wrong_check_character_is_refused() {
    assert_eq!(
        VerifiedGstin::new("08ZZZZZ0000Z2ZQ"),
        Err(GstinError::CheckCharacter)
    );
    assert_eq!(
        VerifiedGstin::new("08ZZZZZ0000Z2Z0"),
        Err(GstinError::CheckCharacter)
    );
}

// The check character, computed on a base-36 string that is not GSTIN-shaped
// and has a non-zero character at every position, so a wrong weight, a missing
// carry or a mis-ordered alphabet each change the answer.
//
// Value v of each character (0-9 then A-Z), times weight 1 on even positions
// and 2 on odd ones; each product adds its quotient and remainder by 36:
//   9=9, Z=35*2=70 (1+34=35), 8=8, Y=34*2=68 (1+32=33), 7=7, X=33*2=66 (31),
//   6=6, W=32*2=64 (29), 5=5, V=31*2=62 (27), 4=4, U=30*2=60 (25), 3=3,
//   T=29*2=58 (23).
// Even positions: 9+8+7+6+5+4+3 = 42. Odd positions: 35+33+31+29+27+25+23 = 203.
// Sum 245; 245 mod 36 = 29; check = (36 - 29) mod 36 = 7.
// Without the carry the sum would be 238 and the check 'E', so the carry is
// exercised. Worked by hand and by a separate script, not by this module.
#[test]
fn the_check_character_function_matches_a_hand_computed_value() {
    assert_eq!(gstin_check_character(b"9Z8Y7X6W5V4U3T"), b'7');
}

#[test]
fn swapping_two_adjacent_characters_changes_the_check_character() {
    // 08 -> 80 moves 8 from weight 2 to weight 1 (16 -> 8), so the sum moves by 8.
    assert!(VerifiedGstin::new(LAB_GSTIN).is_ok());
    assert_eq!(
        VerifiedGstin::new("80ZZZZZ0000Z2ZP"),
        Err(GstinError::CheckCharacter)
    );
}

#[test]
fn a_malformed_gstin_is_refused_by_shape() {
    let sixteen = format!("{LAB_GSTIN}P");
    let lowercase = LAB_GSTIN.to_lowercase();
    for value in [
        "",
        "08ZZZZZ0000Z2Z",
        lowercase.as_str(),
        sixteen.as_str(),
        "08ZZZZZ0000Z2AP",
    ] {
        assert_eq!(
            VerifiedGstin::new(value),
            Err(GstinError::Shape),
            "{value:?}"
        );
    }
}

// --- AdmittedDutyHead -----------------------------------------------------

#[test]
fn only_sgst_utgst_is_admitted_today() {
    assert_eq!(ADMITTED_DUTY_HEADS, &[GstDutyHead::SgstUtgst]);
    assert_eq!(
        AdmittedDutyHead::new(GstDutyHead::SgstUtgst)
            .unwrap()
            .head(),
        GstDutyHead::SgstUtgst
    );
}

#[test]
fn every_other_head_is_refused_with_a_typed_error() {
    for head in GstDutyHead::ALL {
        if head == GstDutyHead::SgstUtgst {
            continue;
        }
        assert_eq!(
            AdmittedDutyHead::new(head),
            Err(AdmittedDutyHeadError::DutyHeadNotAdmitted(head))
        );
    }
}

// --- Rendering ------------------------------------------------------------

#[test]
fn the_party_request_equals_the_lab_file() {
    let item = LedgerCreateItem::new(
        name("MLab M1 Party"),
        party(PartySide::SundryDebtors, Some(LAB_GSTIN), true),
    );
    let rendered = item.render_import(&lab());
    assert_eq!(rendered.xml(), PARTY_GOLDEN);
    assert_eq!(rendered.sha256(), PARTY_SHA);
}

#[test]
fn the_suspense_request_equals_the_lab_file() {
    let item = LedgerCreateItem::new(name("MLab M1 Suspense"), AdmittedLedgerKind::Suspense);
    let rendered = item.render_import(&lab());
    assert_eq!(rendered.xml(), SUSPENSE_GOLDEN);
    assert_eq!(rendered.sha256(), SUSPENSE_SHA);
}

#[test]
fn the_duty_request_equals_the_lab_file_and_escapes_the_ampersand_once() {
    let item = LedgerCreateItem::new(name("MLab M1 Duty SGST UTGST"), duty());
    let rendered = item.render_import(&lab());
    assert_eq!(rendered.xml(), DUTY_GOLDEN);
    assert_eq!(rendered.sha256(), DUTY_SHA);
    assert!(!rendered.xml().contains("&amp;amp;"));
}

#[test]
fn a_creditor_without_a_gstin_and_not_bill_wise_renders_its_own_parent() {
    let item = LedgerCreateItem::new(
        name("MLab Creditor"),
        party(PartySide::SundryCreditors, None, false),
    );
    assert_eq!(item.parent(), "Sundry Creditors");
    assert_eq!(
        item.render_import(&lab()).xml(),
        PARTY_GOLDEN
            .replace("MLab M1 Party", "MLab Creditor")
            .replace("Sundry Debtors", "Sundry Creditors")
            .replace("<ISBILLWISEON>Yes", "<ISBILLWISEON>No")
            .replace("<PARTYGSTIN>08ZZZZZ0000Z2ZP</PARTYGSTIN>", "")
    );
}

#[test]
fn a_bill_wise_debtor_without_a_gstin_carries_no_tax_elements() {
    let item = LedgerCreateItem::new(
        name("MLab Debtor"),
        party(PartySide::SundryDebtors, None, true),
    );
    assert_eq!(
        element_names(item.render_import(&lab()).xml()),
        [
            "ENVELOPE",
            "HEADER",
            "TALLYREQUEST",
            "BODY",
            "IMPORTDATA",
            "REQUESTDESC",
            "REPORTNAME",
            "STATICVARIABLES",
            "SVCURRENTCOMPANY",
            "REQUESTDATA",
            "TALLYMESSAGE",
            "LEDGER",
            "NAME",
            "PARENT",
            "ISBILLWISEON",
        ]
    );
}

#[test]
fn no_kind_ever_renders_an_opening_balance_element() {
    let items = [
        LedgerCreateItem::new(
            name("P"),
            party(PartySide::SundryDebtors, Some(LAB_GSTIN), true),
        ),
        LedgerCreateItem::new(name("C"), party(PartySide::SundryCreditors, None, false)),
        LedgerCreateItem::new(name("S"), AdmittedLedgerKind::Suspense),
        LedgerCreateItem::new(name("D"), duty()),
    ];
    for item in items {
        let names = element_names(item.render_import(&lab()).xml());
        assert!(!names.iter().any(|n| n == "OPENINGBALANCE"), "{names:?}");
    }
}

#[test]
fn tax_type_and_duty_head_appear_on_a_duty_ledger_only() {
    let party_names = element_names(
        LedgerCreateItem::new(
            name("P"),
            party(PartySide::SundryDebtors, Some(LAB_GSTIN), true),
        )
        .render_import(&lab())
        .xml(),
    );
    assert!(party_names.iter().any(|n| n == "PARTYGSTIN"));
    assert!(!party_names
        .iter()
        .any(|n| n == "TAXTYPE" || n == "GSTDUTYHEAD"));

    let suspense_names = element_names(
        LedgerCreateItem::new(name("S"), AdmittedLedgerKind::Suspense)
            .render_import(&lab())
            .xml(),
    );
    assert!(!suspense_names
        .iter()
        .any(|n| n == "PARTYGSTIN" || n == "TAXTYPE" || n == "GSTDUTYHEAD"));

    let duty_names = element_names(
        LedgerCreateItem::new(name("D"), duty())
            .render_import(&lab())
            .xml(),
    );
    assert!(duty_names.iter().any(|n| n == "TAXTYPE"));
    assert!(duty_names.iter().any(|n| n == "GSTDUTYHEAD"));
    assert!(!duty_names.iter().any(|n| n == "PARTYGSTIN"));
}

#[test]
fn a_company_and_a_name_with_reserved_characters_are_escaped_once() {
    let company = VerifiedCompanyIdentity::test_fixture(
        "R&D <Co> \"1\"",
        "00000000-0000-4000-8000-000000000002",
    );
    let rendered = LedgerCreateItem::new(name("A & B <C>"), AdmittedLedgerKind::Suspense)
        .render_import(&company);
    assert!(rendered
        .xml()
        .contains("<SVCURRENTCOMPANY>R&amp;D &lt;Co&gt; &quot;1&quot;</SVCURRENTCOMPANY>"));
    assert!(rendered.xml().contains("<NAME>A &amp; B &lt;C&gt;</NAME>"));
}

#[test]
fn the_digest_is_over_the_utf8_bytes_of_the_rendered_xml() {
    let rendered =
        LedgerCreateItem::new(name("S"), AdmittedLedgerKind::Suspense).render_import(&lab());
    assert_eq!(rendered.sha256(), sha256_hex(rendered.xml().as_bytes()));
    assert_eq!(rendered.sha256().len(), 64);
    assert!(rendered
        .sha256()
        .bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    // Different input, different digest.
    let other =
        LedgerCreateItem::new(name("T"), AdmittedLedgerKind::Suspense).render_import(&lab());
    assert_ne!(rendered.sha256(), other.sha256());
}

#[test]
fn the_item_exposes_the_name_and_kind_it_was_built_from() {
    let item = LedgerCreateItem::new(name("MLab M1 Suspense"), AdmittedLedgerKind::Suspense);
    assert_eq!(item.name().as_str(), "MLab M1 Suspense");
    assert_eq!(item.kind(), &AdmittedLedgerKind::Suspense);
    assert_eq!(item.parent(), "Suspense A/c");
    assert_eq!(
        LedgerCreateItem::new(name("D"), duty()).parent(),
        "Duties & Taxes"
    );
}
