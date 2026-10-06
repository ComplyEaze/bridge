//! The V2 catalogue parser on SYNTHETIC bodies that copy the layout of the
//! captured V1 answers and add `ISBILLWISEON` where the outstandings snapshot
//! has it. They exercise parsing only. They are not evidence that Tally
//! answers the V2 request this way; the live capture is (AGENTS.md P1).

use super::*;

const COMPANY: &str = "Synthetic V2 Book";
const COMPANY_GUID: &str = "11111111-2222-4333-8444-555555555555";

fn row(name: &str, guid_suffix: &str, parent: &str, flag: Option<&str>) -> String {
    let flag = flag
        .map(|text| format!("<ISBILLWISEON TYPE=\"Logical\">{text}</ISBILLWISEON>"))
        .unwrap_or_default();
    format!(
        "<LEDGER NAME=\"{name}\" RESERVEDNAME=\"\"><GUID TYPE=\"String\">{COMPANY_GUID}-{guid_suffix}</GUID>\
         <PARENT TYPE=\"String\">{parent}</PARENT>{flag}\
         <BRIDGECOMPANYGUID TYPE=\"String\">{COMPANY_GUID}</BRIDGECOMPANYGUID>\
         <BRIDGECOMPANYNAME TYPE=\"String\">{COMPANY}</BRIDGECOMPANYNAME>\
         <LANGUAGENAME.LIST><NAME.LIST TYPE=\"String\"><NAME>{name}</NAME></NAME.LIST>\
         <LANGUAGEID> 1033</LANGUAGEID></LANGUAGENAME.LIST></LEDGER>"
    )
}

fn body(rows: &[String]) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO>\
         <COMPANY>0</COMPANY></CMPINFO></DESC><DATA><COLLECTION ISMSTDEPTYPE=\"Yes\" MSTDEPTYPE=\"8\">{}\
         </COLLECTION></DATA></BODY></ENVELOPE>",
        rows.concat()
    )
}

fn parse_v2(xml: &str) -> Result<StandardLedgerCatalogV2, StandardLedgerCatalogError> {
    parse_standard_ledger_catalog_v2_with_identities(xml, COMPANY, COMPANY_GUID)
}

fn three_rows(second_flag: Option<&str>) -> String {
    body(&[
        row("Cash", "01", "Cash-in-Hand", Some("No")),
        row("Debtor A", "02", "Sundry Debtors", second_flag),
        row("Creditor B", "03", "Sundry Creditors", Some("Yes")),
    ])
}

#[test]
fn every_row_carries_its_flag_and_the_catalogue_is_v1s() {
    let v2 = parse_v2(&three_rows(Some("Yes"))).unwrap();
    assert_eq!(
        v2.bill_wise_flags()
            .map(|(name, guid, flag)| (name.to_string(), guid.to_string(), flag))
            .collect::<Vec<_>>(),
        vec![
            (
                "Cash".into(),
                format!("{COMPANY_GUID}-01"),
                BillWiseFlag::Off
            ),
            (
                "Debtor A".into(),
                format!("{COMPANY_GUID}-02"),
                BillWiseFlag::On
            ),
            (
                "Creditor B".into(),
                format!("{COMPANY_GUID}-03"),
                BillWiseFlag::On
            ),
        ]
    );
    // The same rows without the flag are a V1 answer, and V1's parse of them
    // is the catalogue inside V2's.
    let v1_body = body(&[
        row("Cash", "01", "Cash-in-Hand", None),
        row("Debtor A", "02", "Sundry Debtors", None),
        row("Creditor B", "03", "Sundry Creditors", None),
    ]);
    let v1 =
        parse_standard_ledger_catalog_with_identities(&v1_body, COMPANY, COMPANY_GUID).unwrap();
    assert_eq!(v2.catalog(), &v1);
}

#[test]
fn the_flag_is_exactly_yes_or_no_after_trimming_and_nothing_else() {
    // The text reader trims, as it does for every other value, so padding
    // around an exact spelling is read; whitespace alone is not.
    for (text, expected) in [
        ("Yes", BillWiseFlag::On),
        ("No", BillWiseFlag::Off),
        (" Yes ", BillWiseFlag::On),
        ("\nNo\t", BillWiseFlag::Off),
    ] {
        let v2 = parse_v2(&three_rows(Some(text))).unwrap();
        assert_eq!(v2.bill_wise_flags().nth(1).unwrap().2, expected, "{text}");
    }
    // Only the two spellings seen live are a flag. Tally's other logical
    // spellings, and other letter cases, are refused until a capture shows
    // them, not read as on or off.
    for text in [
        "", " ", "Maybe", "true", "false", "1", "0", "Yes Yes", "yes", "NO", "YES",
    ] {
        assert_eq!(
            parse_v2(&three_rows(Some(text))).unwrap_err(),
            StandardLedgerCatalogError::BillWiseFlagInvalid,
            "{text:?}"
        );
    }
    // The flag is a Logical: another type, or none, is not a flag.
    for typed in ["TYPE=\"String\"", "TYPE=\"logical\"", ""] {
        let retyped = three_rows(Some("Yes")).replacen("TYPE=\"Logical\"", typed, 1);
        assert_eq!(
            parse_v2(&retyped).unwrap_err(),
            StandardLedgerCatalogError::BillWiseFlagInvalid,
            "{typed}"
        );
    }
    let empty_element = body(&[row("Debtor A", "02", "Sundry Debtors", None).replace(
        "<BRIDGECOMPANYGUID",
        "<ISBILLWISEON TYPE=\"Logical\"/><BRIDGECOMPANYGUID",
    )]);
    assert_eq!(
        parse_v2(&empty_element).unwrap_err(),
        StandardLedgerCatalogError::BillWiseFlagInvalid
    );
}

#[test]
fn a_row_without_the_flag_fails_the_whole_answer_and_a_v1_body_fails_closed() {
    assert_eq!(
        parse_v2(&three_rows(None)).unwrap_err(),
        StandardLedgerCatalogError::BillWiseFlagMissing
    );
    let v1_body = body(&[
        row("Cash", "01", "Cash-in-Hand", None),
        row("Debtor A", "02", "Sundry Debtors", None),
    ]);
    assert_eq!(
        parse_v2(&v1_body).unwrap_err(),
        StandardLedgerCatalogError::BillWiseFlagMissing
    );
}

#[test]
fn a_repeated_flag_is_refused() {
    let doubled = three_rows(Some("Yes")).replacen(
        "<ISBILLWISEON TYPE=\"Logical\">Yes</ISBILLWISEON>",
        "<ISBILLWISEON TYPE=\"Logical\">Yes</ISBILLWISEON><ISBILLWISEON TYPE=\"Logical\">No</ISBILLWISEON>",
        1,
    );
    assert_eq!(
        parse_v2(&doubled).unwrap_err(),
        StandardLedgerCatalogError::BillWiseFlagRepeated
    );
}

#[test]
fn v2_keeps_every_v1_check() {
    // Another company's row.
    assert_eq!(
        parse_standard_ledger_catalog_v2_with_identities(
            &three_rows(Some("Yes")),
            COMPANY,
            "99999999-2222-4333-8444-555555555555",
        )
        .unwrap_err(),
        StandardLedgerCatalogError::CompanyIdentityMismatch
    );
    // A repeated ledger GUID, and a repeated name.
    let same_guid = body(&[
        row("A", "01", "Sundry Debtors", Some("Yes")),
        row("B", "01", "Sundry Debtors", Some("Yes")),
    ]);
    assert_eq!(
        parse_v2(&same_guid).unwrap_err(),
        StandardLedgerCatalogError::DuplicateIdentity
    );
    let same_name = body(&[
        row("A", "01", "Sundry Debtors", Some("Yes")),
        row("a", "02", "Sundry Debtors", Some("Yes")),
    ]);
    assert_eq!(
        parse_v2(&same_name).unwrap_err(),
        StandardLedgerCatalogError::DuplicateIdentity
    );
    // No rows.
    assert_eq!(
        parse_v2(&body(&[])).unwrap_err(),
        StandardLedgerCatalogError::MalformedResponse
    );
    // An attribute the flag element does not carry live.
    let odd_attribute = three_rows(Some("Yes")).replacen("TYPE=\"Logical\"", "FOO=\"x\"", 1);
    assert_eq!(
        parse_v2(&odd_attribute).unwrap_err(),
        StandardLedgerCatalogError::MalformedResponse
    );
}

#[test]
fn the_flag_element_belongs_to_v2_alone() {
    // Every V1 parser refuses a row that carries it, so a V2 answer can never
    // be read as a V1 one by a caller that holds the wrong parser.
    let v2_body = three_rows(Some("Yes"));
    assert_eq!(
        parse_standard_ledger_catalog_with_identities(&v2_body, COMPANY, COMPANY_GUID).unwrap_err(),
        StandardLedgerCatalogError::MalformedResponse
    );
    assert_eq!(
        parse_standard_ledger_catalog(&v2_body, COMPANY, COMPANY_GUID).unwrap_err(),
        StandardLedgerCatalogError::MalformedResponse
    );
    assert!(parse_standard_ledger_identity_observation(&v2_body, COMPANY).is_err());
    // Control: the same rows without the flag pass all three, so the refusals
    // above are the flag's and not some other defect of the body.
    let v1_body = body(&[
        row("Cash", "01", "Cash-in-Hand", None),
        row("Debtor A", "02", "Sundry Debtors", None),
        row("Creditor B", "03", "Sundry Creditors", None),
    ]);
    assert!(parse_standard_ledger_catalog_with_identities(&v1_body, COMPANY, COMPANY_GUID).is_ok());
    assert!(parse_standard_ledger_catalog(&v1_body, COMPANY, COMPANY_GUID).is_ok());
    assert!(parse_standard_ledger_identity_observation(&v1_body, COMPANY).is_ok());
}

/// The flag codes are new strings: the older outstandings parser already emits
/// `ledger_bill_wise_flag_missing` and `_invalid` for its own read, so a failure
/// log must say which read refused.
#[test]
fn the_flag_codes_are_not_the_outstandings_parsers_codes() {
    let codes = [
        StandardLedgerCatalogError::BillWiseFlagMissing.safe_code(),
        StandardLedgerCatalogError::BillWiseFlagInvalid.safe_code(),
        StandardLedgerCatalogError::BillWiseFlagRepeated.safe_code(),
    ];
    for code in codes {
        assert!(
            code.starts_with("ledger_catalogue_bill_wise_flag_"),
            "{code}"
        );
    }
    for outstandings in [
        "ledger_bill_wise_flag_missing",
        "ledger_bill_wise_flag_invalid",
    ] {
        assert!(!codes.contains(&outstandings));
    }
}
