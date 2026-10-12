//! The reference a statement row carries when it is a network transfer, and
//! the printed instrument number. All values are synthetic.

mod common;

use bridge_bank_statement::bank::Bank;
use common::*;

/// `(narration, printed reference column, mode Bank::reference names, expected transfer reference)`.
type Case = (
    &'static str,
    &'static str,
    &'static str,
    Option<&'static str>,
);

fn check(bank: Bank, cases: &[Case]) {
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
                "UPI/DR/600000000001/ACME/ZZZZ/nw@okzz/Payment",
                "",
                "UPI",
                Some("600000000001"),
            ),
            (
                "UPI/CR/6000 0000 0002/ACME/ZZZZ/nw@okzz/Payment",
                "",
                "UPI",
                Some("600000000002"),
            ),
            ("UPI/DR/60000000001/ACME/ZZZZ/nw@okzz/P", "", "UPI", None),
            ("UPI/DR/6000000000011/ACME/ZZZZ/nw@okzz/P", "", "UPI", None),
            ("UPI/DR/000000000000/ACME/ZZZZ/nw@okzz/P", "", "UPI", None),
            // IMPS
            (
                "IMPS/600000000003/ACME/NOTE",
                "",
                "IMPS",
                Some("600000000003"),
            ),
            (
                "IMPS/6000 0000 0004/ACME/NOTE",
                "",
                "IMPS",
                Some("600000000004"),
            ),
            ("IMPS/60000000003/ACME/NOTE", "", "IMPS", None),
            ("IMPS/000000000000/ACME/NOTE", "", "IMPS", None),
            // NEFT and RTGS, by either printed shape
            (
                "NEFT*ZZZZ0000001*ZZZZZ00000000001*ACME",
                "",
                "NEFT",
                Some("ZZZZZ00000000001"),
            ),
            (
                "UTR NO: ZZZZZ00000000002-ACME",
                "",
                "NEFT",
                Some("ZZZZZ00000000002"),
            ),
            (
                "RTGS UTR NO: ZZZZZ00000000003-ACME",
                "",
                "RTGS",
                Some("ZZZZZ00000000003"),
            ),
            ("UTR NO: AB-ACME", "", "NEFT", None),
            ("UTR NO: INTERNATIONAL-ACME", "", "NEFT", None),
            // everything else
            ("ATM CASH 600000000005 ZZ", "", "ATM", None),
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
                "UPI-ACME-9@x-ABCD0001-600000000001-P",
                "1",
                "UPI",
                Some("600000000001"),
            ),
            ("UPI-ACME-9@x-ABCD0001-6000000000011-P", "1", "UPI", None),
            ("UPI-ACME-9@x-ABCD0001-000000000000-P", "1", "UPI", None),
            (
                "IMPS-600000000003-ACME-ABCD-NOTE",
                "1",
                "IMPS",
                Some("600000000003"),
            ),
            ("IMPS-60000000003-ACME-ABCD-NOTE", "1", "IMPS", None),
            ("IMPS-000000000000-ACME-ABCD-NOTE", "1", "IMPS", None),
            (
                "NEFT DR-ABCD0001234-ACME INDUSTRIES-ZZZZZ00000000003",
                "1",
                "NEFT",
                Some("ZZZZZ00000000003"),
            ),
            // a NEFT line that falls back to the printed reference column
            (
                "NEFT CR-ACME",
                "0000ZZZZZ00000000004",
                "NEFT",
                Some("ZZZZZ00000000004"),
            ),
            ("NEFT CR-ACME", "123", "NEFT", None),
            ("NEFT CR-ACME", "0000000000000000", "NEFT", None),
            ("NEFT CR-ACME", "", "NEFT", None),
            (
                "ACH D- ACME LTD-0000000012345",
                "0000000012345",
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
                "UPIAB/600000000001/DR/ACME/ZZZZ",
                "",
                "UPI",
                Some("600000000001"),
            ),
            ("UPIAB/000000000000/DR/ACME/ZZZZ", "", "UPI", None),
            ("IMPSAB/600000000003/ACME", "", "IMPS", Some("600000000003")),
            ("IMPSAR/600000000004/ACME", "", "IMPS", Some("600000000004")),
            ("IMPSAB/000000000000/ACME", "", "IMPS", None),
            (
                "NEFT:ACME INDUSTRIES ZZZZZ00000000001",
                "",
                "NEFT",
                Some("ZZZZZ00000000001"),
            ),
            ("MOBFT/ACME/600000000005", "", "MOBFT", None),
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
                "UPI/600000000001/10:15:30/UPI/nw@okzz/ZZZZ BANK",
                "",
                "UPI",
                Some("600000000001"),
            ),
            ("UPI/000000000000/10:15:30/UPI/nw@okzz/Z", "", "UPI", None),
            (
                "IMPS/P2A/600000000003/ZZZZ/NOTE",
                "",
                "IMPS",
                Some("600000000003"),
            ),
            ("IMPS/P2A/000000000000/ZZZZ/NOTE", "", "IMPS", None),
            (
                "NEFT-ZZZZZ00000000001-ACME INDUSTRIES",
                "",
                "NEFT",
                Some("ZZZZZ00000000001"),
            ),
            (
                "RTGS-ZZZZZ00000000000000002-GREEN FIELD LTD",
                "",
                "RTGS",
                Some("ZZZZZ00000000000000002"),
            ),
            ("NEFT-ab-ACME", "", "TXN", None),
            ("MBK/600000000004/12:00:00/SOME NAME", "", "MBK", None),
            ("BY CASH", "", "CASH", None),
            ("SYNTHETIC SUPPLIES-MICR INWARD CLG", "000417", "TXN", None),
        ],
    );
}

#[test]
fn a_transfer_reference_serialises_as_its_plain_string() {
    let row = row(&[("narr", "IMPS/600000000003/ACME/NOTE")]);
    let found = Bank::Sbi.transfer_reference(&row).unwrap();
    assert_eq!(found.to_string(), "600000000003");
    assert_eq!(
        serde_json::to_value(&found).unwrap(),
        serde_json::json!("600000000003")
    );
}

#[test]
fn the_instrument_is_the_printed_cheque_or_number_column() {
    let printed = |column: &str, value: &str| row(&[(column, value), ("narr", "X")]);
    for bank in [Bank::Sbi, Bank::Hdfc, Bank::Ubi] {
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
}
