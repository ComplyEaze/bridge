//! The reference a statement row carries when it is a network transfer, and
//! the printed instrument number. All values are synthetic.

mod common;

use bridge_bank_statement::bank::Bank;
use common::*;

/// `(narration, printed reference column, mode Bank::reference names, expected transfer reference)`.
type Case<'a> = (&'a str, &'a str, &'static str, Option<&'a str>);

fn check(bank: Bank, cases: &[Case<'_>]) {
    for (narration, printed, mode, expected) in cases {
        let row = row(&[("narr", narration), ("ref", printed), ("chq", printed)]);
        // the case really is the mode it claims, so a change in the parser cannot
        // leave a table row testing something else
        assert_eq!(bank.reference(&row).0, *mode, "{narration}");
        assert_eq!(
            bank.transfer_reference(&row).map(|found| found.to_string()),
            expected.map(str::to_string),
            "{bank:?} {narration} {printed}"
        );
    }
}

#[test]
fn sbi_admits_only_network_transfers() {
    check(
        Bank::Sbi,
        &[
            // UPI: twelve digits once spaces are gone, and the VPA tail dropped
            (
                &format!("UPI/DR/{}/ACME/ZZZZ/nw@okzz/Payment", p6(1, 12)),
                "",
                "UPI",
                Some(p6(1, 12).as_str()),
            ),
            (
                &format!("UPI/CR/{}/ACME/ZZZZ/nw@okzz/Payment", grp(&p6(2, 12))),
                "",
                "UPI",
                Some(p6(2, 12).as_str()),
            ),
            (
                &format!("UPI/DR/{}/ACME/ZZZZ/nw@okzz/P", p6(1, 11)),
                "",
                "UPI",
                None,
            ),
            (
                &format!("UPI/DR/{}/ACME/ZZZZ/nw@okzz/P", p6(11, 13)),
                "",
                "UPI",
                None,
            ),
            (
                &format!("UPI/DR/{}/ACME/ZZZZ/nw@okzz/P", padn(0, 12)),
                "",
                "UPI",
                None,
            ),
            // IMPS
            (
                &format!("IMPS/{}/ACME/NOTE", p6(3, 12)),
                "",
                "IMPS",
                Some(p6(3, 12).as_str()),
            ),
            (
                &format!("IMPS/{}/ACME/NOTE", grp(&p6(4, 12))),
                "",
                "IMPS",
                Some(p6(4, 12).as_str()),
            ),
            (&format!("IMPS/{}/ACME/NOTE", p6(3, 11)), "", "IMPS", None),
            (&format!("IMPS/{}/ACME/NOTE", padn(0, 12)), "", "IMPS", None),
            // NEFT and RTGS, by either printed shape
            (
                &format!("NEFT*ZZZZ0000001*ZZZZZ{}*ACME", padn(1, 11)),
                "",
                "NEFT",
                Some(format!("ZZZZZ{}", padn(1, 11)).as_str()),
            ),
            (
                &format!("UTR NO: ZZZZZ{}-ACME", padn(2, 11)),
                "",
                "NEFT",
                Some(format!("ZZZZZ{}", padn(2, 11)).as_str()),
            ),
            (
                &format!("RTGS UTR NO: ZZZZZ{}-ACME", padn(3, 11)),
                "",
                "RTGS",
                Some(format!("ZZZZZ{}", padn(3, 11)).as_str()),
            ),
            ("UTR NO: AB-ACME", "", "NEFT", None),
            ("UTR NO: INTERNATIONAL-ACME", "", "NEFT", None),
            // everything else
            (&format!("ATM CASH {} ZZ", p6(5, 12)), "", "ATM", None),
            ("TRANSFER FROM X", "CT0ZZZ0001 TRANSFER", "INB", None),
            ("CHEQUE PAID", "TRANSFER/ 000417", "CHQ", None),
            ("INT TRF TO 12345", "", "INT-TRF", None),
            ("SOMETHING ELSE", "", "TXN", None),
        ],
    );
}

#[test]
fn hdfc_admits_only_network_transfers() {
    check(
        Bank::Hdfc,
        &[
            (
                &format!("UPI-ACME-9@x-ABCD0001-{}-P", p6(1, 12)),
                "1",
                "UPI",
                Some(p6(1, 12).as_str()),
            ),
            (
                &format!("UPI-ACME-9@x-ABCD0001-{}-P", p6(11, 13)),
                "1",
                "UPI",
                None,
            ),
            (
                &format!("UPI-ACME-9@x-ABCD0001-{}-P", padn(0, 12)),
                "1",
                "UPI",
                None,
            ),
            (
                &format!("IMPS-{}-ACME-ABCD-NOTE", p6(3, 12)),
                "1",
                "IMPS",
                Some(p6(3, 12).as_str()),
            ),
            (
                &format!("IMPS-{}-ACME-ABCD-NOTE", p6(3, 11)),
                "1",
                "IMPS",
                None,
            ),
            (
                &format!("IMPS-{}-ACME-ABCD-NOTE", padn(0, 12)),
                "1",
                "IMPS",
                None,
            ),
            // HDFC's NEFT value is the narration's last field, which can be a
            // remark, or the reference column, which cannot hold a long UTR:
            // neither is admitted until a real statement shows the UTR's place.
            (
                &format!("NEFT DR-ABCD0001234-ACME INDUSTRIES-ZZZZZ{}", padn(3, 11)),
                "1",
                "NEFT",
                None,
            ),
            (
                "NEFT CR-ACME",
                &format!("0000ZZZZZ{}", padn(4, 11)),
                "NEFT",
                None,
            ),
            ("NEFT CR-ACME", "123", "NEFT", None),
            ("NEFT CR-ACME", &padn(0, 16), "NEFT", None),
            ("NEFT CR-ACME", "", "NEFT", None),
            (
                &format!("ACH D- ACME LTD-{}", padn(12345, 13)),
                &padn(12345, 13),
                "ACH",
                None,
            ),
            ("CHQ PAID-MICR CLG", "000417", "TXN", None),
            ("SOMETHING ELSE", "", "TXN", None),
        ],
    );
}

#[test]
fn ubi_admits_only_network_transfers() {
    check(
        Bank::Ubi,
        &[
            (
                &format!("UPIAB/{}/DR/ACME/ZZZZ", p6(1, 12)),
                "",
                "UPI",
                Some(p6(1, 12).as_str()),
            ),
            (
                &format!("UPIAB/{}/DR/ACME/ZZZZ", padn(0, 12)),
                "",
                "UPI",
                None,
            ),
            (
                &format!("IMPSAB/{}/ACME", p6(3, 12)),
                "",
                "IMPS",
                Some(p6(3, 12).as_str()),
            ),
            (
                &format!("IMPSAR/{}/ACME", p6(4, 12)),
                "",
                "IMPS",
                Some(p6(4, 12).as_str()),
            ),
            (&format!("IMPSAB/{}/ACME", padn(0, 12)), "", "IMPS", None),
            (
                &format!("NEFT:ACME INDUSTRIES ZZZZZ{}", padn(1, 11)),
                "",
                "NEFT",
                Some(format!("ZZZZZ{}", padn(1, 11)).as_str()),
            ),
            (&format!("MOBFT/ACME/{}", p6(5, 12)), "", "MOBFT", None),
            ("CLG/ACME", "000417", "CLG", None),
            ("BY CASH", "", "CASH", None),
            ("SOMETHING ELSE", "000417", "TXN", None),
        ],
    );
}

#[test]
fn bob_admits_only_network_transfers() {
    check(
        Bank::Bob,
        &[
            (
                &format!("UPI/{}/10:15:30/UPI/nw@okzz/ZZZZ BANK", p6(1, 12)),
                "",
                "UPI",
                Some(p6(1, 12).as_str()),
            ),
            (
                &format!("UPI/{}/10:15:30/UPI/nw@okzz/Z", padn(0, 12)),
                "",
                "UPI",
                None,
            ),
            (
                &format!("IMPS/P2A/{}/ZZZZ/NOTE", p6(3, 12)),
                "",
                "IMPS",
                Some(p6(3, 12).as_str()),
            ),
            (
                &format!("IMPS/P2A/{}/ZZZZ/NOTE", padn(0, 12)),
                "",
                "IMPS",
                None,
            ),
            (
                &format!("NEFT-ZZZZZ{}-ACME INDUSTRIES", padn(1, 11)),
                "",
                "NEFT",
                Some(format!("ZZZZZ{}", padn(1, 11)).as_str()),
            ),
            (
                &format!("RTGS-ZZZZZ{}-GREEN FIELD LTD", padn(2, 17)),
                "",
                "RTGS",
                Some(format!("ZZZZZ{}", padn(2, 17)).as_str()),
            ),
            ("NEFT-ab-ACME", "", "TXN", None),
            (
                &format!("MBK/{}/12:00:00/SOME NAME", p6(4, 12)),
                "",
                "MBK",
                None,
            ),
            ("BY CASH", "", "CASH", None),
            ("SYNTHETIC SUPPLIES-MICR INWARD CLG", "000417", "TXN", None),
        ],
    );
}

#[test]
fn a_transfer_reference_serialises_as_its_plain_string() {
    let row = row(&[("narr", &format!("IMPS/{}/ACME/NOTE", p6(3, 12)))]);
    let found = Bank::Sbi.transfer_reference(&row).unwrap();
    assert_eq!(found.to_string(), p6(3, 12));
    assert_eq!(
        serde_json::to_value(&found).unwrap(),
        serde_json::json!(p6(3, 12))
    );
}

#[test]
fn the_instrument_is_the_printed_cheque_or_number_column() {
    let printed = |column: &str, value: &str| row(&[(column, value), ("narr", "X")]);
    for bank in [Bank::Hdfc, Bank::Ubi] {
        assert_eq!(
            bank.instrument(&printed("ref", "  000417 ")),
            Some("000417".to_string()),
            "{bank:?}"
        );
        // Bank of Baroda's column is not read by the others
        assert_eq!(bank.instrument(&printed("chq", "000417")), None, "{bank:?}");
        assert_eq!(bank.instrument(&printed("ref", "")), None, "{bank:?}");
        assert_eq!(bank.instrument(&printed("ref", "  ")), None, "{bank:?}");
        assert_eq!(
            bank.instrument(&printed("ref", "0000000")),
            None,
            "{bank:?}"
        );
    }
    assert_eq!(
        Bank::Bob.instrument(&printed("chq", " 000417 ")),
        Some("000417".to_string())
    );
    assert_eq!(Bank::Bob.instrument(&printed("ref", "000417")), None);
    assert_eq!(Bank::Bob.instrument(&printed("chq", "")), None);
    assert_eq!(Bank::Bob.instrument(&printed("chq", "00000")), None);
    // State Bank of India's `ref` column is narrative (an account number and a
    // name); only a cheque number at its end is an instrument.
    let sbi = |reference: &str| row(&[("ref", reference), ("narr", "X")]);
    assert_eq!(
        Bank::Sbi.instrument(&sbi("CHQ TRANSFER / 000417")),
        Some("000417".to_string())
    );
    assert_eq!(Bank::Sbi.instrument(&sbi("CHQ TRANSFER / 000000")), None);
    assert_eq!(
        Bank::Sbi.instrument(&sbi("TRANSFER TO 12345678 ACME /")),
        None
    );
    assert_eq!(Bank::Sbi.instrument(&sbi("  000417 ")), None);
}
