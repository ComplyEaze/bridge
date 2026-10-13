//! What `Bank::reference` returns, one row per branch, for the four supported banks.
//!
//! Each table is one bank. A case is a named branch of that bank's reader, with the
//! rows that reach it: the narration, the cell the reader falls back to (the
//! reference column, or the cheque column for Bank of Baroda), and the `(mode, value)`
//! it returns today. The values pin what the code does now; they are not a claim that
//! every one is a bank reference. A table asserts its own length, in cases and in rows,
//! so a case or a row deleted from it cannot go unseen. A case may carry a second row
//! with a wrapped cell (a space inside a value) or an outer space, which the readers
//! remove or strip.
//!
//! Every narration and number below is synthetic.

mod common;

use bridge_bank_statement::bank::Bank;
use common::*;

/// `(narration, fallback cell, mode, value)`.
type Example = (&'static str, &'static str, &'static str, &'static str);

struct Case {
    name: &'static str,
    rows: &'static [Example],
}

/// Run every case of one bank, and fail if the table has not the cases and rows it is meant to have.
fn check(
    bank: Bank,
    cell_column: &str,
    cases: &[Case],
    expected_cases: usize,
    expected_rows: usize,
) {
    assert_eq!(cases.len(), expected_cases, "a case was added or removed");
    let rows: usize = cases.iter().map(|case| case.rows.len()).sum();
    assert_eq!(rows, expected_rows, "a row was added or removed");
    for case in cases {
        assert!(!case.rows.is_empty(), "{}: no rows", case.name);
        for (narration, cell, mode, value) in case.rows {
            let found = bank.reference(&row(&[("narr", narration), (cell_column, cell)]));
            assert_eq!(
                found,
                (mode.to_string(), value.to_string()),
                "{}: narration {narration:?}, cell {cell:?}",
                case.name
            );
        }
    }
}

#[test]
fn sbi_reference_branches() {
    const CASES: &[Case] = &[
        Case {
            name: "utr_rtgs",
            rows: &[(
                "RTGS UTR NO: SYNTH0000000001-ACME INDUSTRIES",
                "",
                "RTGS",
                "SYNTH0000000001",
            )],
        },
        // a wrapped cell leaves a space inside the reference
        Case {
            name: "utr_neft",
            rows: &[(
                "NEFT UTR NO: SYNTH00000 00003-ACME INDUSTRIES",
                "",
                "NEFT",
                "SYNTH0000000003",
            )],
        },
        Case {
            name: "neft_star",
            rows: &[
                (
                    "NEFT*SYNTHBANK*SYNTH0000000004*ACME INDUSTRIES*",
                    "",
                    "NEFT",
                    "SYNTH0000000004",
                ),
                (
                    "NEFT*SYNTHBANK*SYNTH00000 00009*ACME INDUSTRIES*",
                    "",
                    "NEFT",
                    "SYNTH0000000009",
                ),
            ],
        },
        // the second row wraps both the reference and the VPA
        Case {
            name: "upi_with_vpa",
            rows: &[
                (
                    "UPI/DR/600000000005/ACME INDUSTRIES/SYNTHBANK/acme@synthbank/Payment",
                    "",
                    "UPI",
                    "600000000005 (acme@synthbank)",
                ),
                (
                    "UPI/DR/6000 00000010/ACME INDUSTRIES/SYNTHBANK/acme @synthbank/Payment",
                    "",
                    "UPI",
                    "600000000010 (acme@synthbank)",
                ),
            ],
        },
        Case {
            name: "upi_without_vpa",
            rows: &[(
                "UPI/CR/600000000006/ACME INDUSTRIES",
                "",
                "UPI",
                "600000000006",
            )],
        },
        Case {
            name: "imps",
            rows: &[
                (
                    "IMPS/600000000007/ACME INDUSTRIES",
                    "",
                    "IMPS",
                    "600000000007",
                ),
                (
                    "IMPS/6000 00000011/ACME INDUSTRIES",
                    "",
                    "IMPS",
                    "600000000011",
                ),
            ],
        },
        Case {
            name: "atm",
            rows: &[
                ("ATM CASH 0000123456 SYNTHBANK", "", "ATM", "0000123456"),
                ("ATM CASH 0000 123457 SYNTHBANK", "", "ATM", "0000123457"),
            ],
        },
        // the internet-banking reference is in the reference column, not the narration
        Case {
            name: "inb",
            rows: &[(
                "SYNTHETIC TRANSFER",
                "CT0SYNTH1 2TRANSFER",
                "INB",
                "CT0SYNTH12",
            )],
        },
        Case {
            name: "cheque",
            rows: &[(
                "SYNTHETIC CLEARING",
                "ACME INDUSTRIES/000417",
                "CHQ",
                "000417",
            )],
        },
        Case {
            name: "internal_transfer_with_account",
            rows: &[("INT TRF TO 4000000000008", "", "INT-TRF", "4000000000008")],
        },
        Case {
            name: "internal_transfer_without_account",
            rows: &[("INT TRF CREDIT", "", "INT-TRF", "")],
        },
        Case {
            name: "txn_fallback",
            rows: &[("SYNTHETIC SUPPLIES PAYMENT", "SYNTH 12", "TXN", "")],
        },
    ];
    check(Bank::Sbi, "ref", CASES, 12, 16);
}

#[test]
fn hdfc_reference_branches() {
    const CASES: &[Case] = &[
        Case {
            name: "imps",
            rows: &[(
                "IMPS-600000000101-ACME INDUSTRIES-SYNTHBANK-XXXX0001-NOTE",
                "0000000000",
                "IMPS",
                "600000000101",
            )],
        },
        Case {
            name: "upi",
            rows: &[(
                "UPI-ACME INDUSTRIES-acme@synthbank-SYNT0000001-600000000102-NOTE",
                "0000000000",
                "UPI",
                "600000000102",
            )],
        },
        Case {
            name: "neft_pattern",
            rows: &[(
                "NEFT CR-SYNT0000001-ACME INDUSTRIES-SYNTHN0000000106",
                "0000000000",
                "NEFT",
                "SYNTHN0000000106",
            )],
        },
        // the NEFT prefix without the pattern: the reference column, zeros trimmed
        Case {
            name: "neft_prefix_without_pattern",
            rows: &[("NEFT RETURN SYNTHETIC", "0000000412", "NEFT", "412")],
        },
        Case {
            name: "ach",
            rows: &[(
                "ACH D- TP ACH ACME INDUSTRIES-0001234567",
                "0000000077",
                "ACH",
                "77",
            )],
        },
        Case {
            name: "txn_leading_zeros_trimmed",
            rows: &[("SYNTHETIC SUPPLIES PAYMENT", "0000000456", "TXN", "456")],
        },
        Case {
            name: "txn_all_zero_kept_as_printed",
            rows: &[(
                "SYNTHETIC SUPPLIES PAYMENT",
                "0000000000",
                "TXN",
                "0000000000",
            )],
        },
    ];
    check(Bank::Hdfc, "ref", CASES, 7, 7);
}

#[test]
fn union_bank_reference_branches() {
    const CASES: &[Case] = &[
        // the second row has spaces outside the narration, which every reader strips first
        Case {
            name: "upi",
            rows: &[
                (
                    "UPIAB/600000000201/CR/ACME INDUSTRIES/SYNT/acme@synthbank",
                    "SYNTHID001",
                    "UPI",
                    "600000000201",
                ),
                (
                    "  UPIAB/600000000206/CR/ACME INDUSTRIES/SYNT/acme@synthbank ",
                    "SYNTHID009",
                    "UPI",
                    "600000000206",
                ),
            ],
        },
        // one pattern, `IMPSA[BR]`: both letters are pinned
        Case {
            name: "imps_ab_and_ar",
            rows: &[
                (
                    "IMPSAB/600000000202/ACME INDUSTRIES/9000000001",
                    "SYNTHID002",
                    "IMPS",
                    "600000000202",
                ),
                (
                    "IMPSAR/600000000203/ACME INDUSTRIES/90000000000001",
                    "SYNTHID003",
                    "IMPS",
                    "600000000203",
                ),
            ],
        },
        Case {
            name: "mobft",
            rows: &[(
                "MOBFT/ACME INDUSTRIES/600000000204",
                "SYNTHID004",
                "MOBFT",
                "600000000204",
            )],
        },
        Case {
            name: "neft_utr_shaped_tail",
            rows: &[(
                "NEFT:ACME INDUSTRIES SYNTHN000000205",
                "SYNTHID005",
                "NEFT",
                "SYNTHN000000205",
            )],
        },
        // a tail that is not a bank reference falls to TXN with the reference column
        Case {
            name: "neft_tail_not_utr_shaped",
            rows: &[(
                "NEFT:ACME INDUSTRIES ref",
                "SYNTHID006",
                "TXN",
                "SYNTHID006",
            )],
        },
        Case {
            name: "clearing",
            rows: &[("CLG/ACME INDUSTRIES", "000417", "CLG", "000417")],
        },
        Case {
            name: "by_cash",
            rows: &[("BY CASH", "SYNTHID007", "CASH", "SYNTHID007")],
        },
        Case {
            name: "txn",
            rows: &[(
                "SYNTHETIC SUPPLIES PAYMENT",
                "  SYNTHID008 ",
                "TXN",
                "SYNTHID008",
            )],
        },
    ];
    check(Bank::Ubi, "ref", CASES, 8, 10);
}

#[test]
fn bank_of_baroda_reference_branches() {
    const CASES: &[Case] = &[
        // the second row has spaces outside the narration, which every reader strips first
        Case {
            name: "upi",
            rows: &[
                (
                    "UPI/600000000301/10:15:30/UPI/nw@okzz/ZZZZ BANK",
                    "",
                    "UPI",
                    "600000000301",
                ),
                (
                    "  UPI/600000000306/10:15:30/UPI/nw@okzz/ZZZZ BANK ",
                    "",
                    "UPI",
                    "600000000306",
                ),
            ],
        },
        Case {
            name: "imps",
            rows: &[(
                "IMPS/P2A/600000000302/ZZZZZZZZZZ1 234/NOTE",
                "",
                "IMPS",
                "600000000302",
            )],
        },
        Case {
            name: "mbk",
            rows: &[(
                "MBK/600000000303/12:00:00/SOME NAME",
                "",
                "MBK",
                "600000000303",
            )],
        },
        Case {
            name: "neft",
            rows: &[(
                "NEFT-SYNTHN00000000304-ACME INDUSTRIES",
                "",
                "NEFT",
                "SYNTHN00000000304",
            )],
        },
        Case {
            name: "rtgs",
            rows: &[(
                "RTGS-SYNTHR00000000000305-GREEN FIELD LTD",
                "",
                "RTGS",
                "SYNTHR00000000000305",
            )],
        },
        // a second field that is not a bank reference falls to TXN with the cheque column
        Case {
            name: "transfer_second_field_not_utr_shaped",
            rows: &[("NEFT-ab-ACME INDUSTRIES", "000417", "TXN", "000417")],
        },
        Case {
            name: "by_cash",
            rows: &[("BY CASH", "000418", "CASH", "000418")],
        },
        Case {
            name: "txn",
            rows: &[(
                "SYNTHETIC SUPPLIES-MICR INWARD CLG (CTS)",
                "  000419 ",
                "TXN",
                "000419",
            )],
        },
    ];
    check(Bank::Bob, "chq", CASES, 8, 9);
}
