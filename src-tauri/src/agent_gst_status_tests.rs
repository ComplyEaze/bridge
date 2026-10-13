//! The GST status tool (R7) against Tally's own bytes.
//!
//! The answers are the lab's, read on 10 and 12 Oct 2026 from the synthetic company
//! `BRIDGE PILOT LAB` (TallyPrime 7.1 Silver) and committed byte for byte under
//! `agent/gst-status/` (see its PROVENANCE.md). The 12 Oct answer is to the request the tool
//! sends; two of the 10 Oct answers are to the earlier request, which did not fetch whether a
//! voucher is cancelled or optional, and read every voucher unread. Nothing here is evidence about a real book. The
//! expected rows below were taken from those bytes with a separate script, not from the code
//! under test; the mutations are string replacements on the decoded text of the committed
//! answer, each asserted to match exactly once, so none can pass by changing nothing.
//!
//! What each part proves:
//! - the cancelled and optional flags are decided first, then the status and acceptance flags
//!   read exactly as measured, and every other spelling is refused with a code rather than read
//!   as a status;
//! - the request is the lab's request, character for character, and a type name that could break
//!   its string literal is refused before it is rendered;
//! - the parse of each answer, and the way a voucher or an answer that cannot be read fails closed;
//! - the tool through `call_tool`, against the simulator replaying the reads of a whole call: the
//!   company list (paired), the company marks and the status read, each inside an identity
//!   bracket and each read twice with a health check after each.
use super::*;
use bridge_tally_transport::TallyEndpointConfig;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const SALES_WINDOW: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-sales-window.utf16le.xml"
);
const FULL_WINDOW: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-window.utf16le.xml"
);
const THREE_VOUCHERS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-three-vouchers.utf16le.xml"
);
const EARLIER_REQUEST: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-sales-window.request.utf16le.xml"
);
const AUGUST: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-august.utf16le.xml"
);
const AUGUST_REQUEST: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-august.request.utf16le.xml"
);

const SALES_WINDOW_SHA256: &str =
    "504d9b06911e222b31d585a092e0192a0a263ede91b9fdc5173040438a1e1672";
const FULL_WINDOW_SHA256: &str = "bdde73bd794236fd76b4b3975f34366b6080322ec4a113cbffb871e016dcde57";
const THREE_VOUCHERS_SHA256: &str =
    "ebc84c460da6123c329ff36ce2b74e13f55f8024efa33d7b2ddec7c318996cb8";
const EARLIER_REQUEST_SHA256: &str =
    "6c94de0ce12aec70c33fda1a59b759bd16bb5e44179756411fa254136c56cd65";
const AUGUST_SHA256: &str = "5b536b15bf0cc57188c6c711e9071e995389295deaa4150c9c5c3a4299f4ae48";
/// The request the tool sends, as the lab sent it on 12 Oct 2026.
const REQUEST_SHA256: &str = "b58442823c22314087eb6941781ca92fe8627497817fbd7b25bff71cc751936c";

const LAB_COMPANY: &str = "BRIDGE PILOT LAB";
const LAB_TYPE: &str = "BRIDGE Sales";

const NOT_REPORTED: &str = "gst_status_not_reported";
const UNREADABLE: &str = "gst_status_unreadable";

fn decode_utf16(bytes: &[u8]) -> String {
    assert_eq!(bytes.len() % 2, 0, "whole UTF-16 code units");
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn tally_date(text: &str) -> bridge_tally_core::TallyDate {
    bridge_tally_core::TallyDate::parse(text).unwrap()
}

// ---------------------------------------------------------------------------------------------
// The committed bytes are the lab's.
// ---------------------------------------------------------------------------------------------

#[test]
fn the_committed_files_are_the_lab_bytes() {
    for (bytes, length, sha256) in [
        (SALES_WINDOW, 21_030, SALES_WINDOW_SHA256),
        (FULL_WINDOW, 35_928, FULL_WINDOW_SHA256),
        (THREE_VOUCHERS, 234_936, THREE_VOUCHERS_SHA256),
        (EARLIER_REQUEST, 1_776, EARLIER_REQUEST_SHA256),
        (AUGUST, 105_178, AUGUST_SHA256),
        (AUGUST_REQUEST, 1_822, REQUEST_SHA256),
    ] {
        assert_eq!(bytes.len(), length);
        assert_eq!(sha256_hex(bytes), sha256);
    }
    // Tally's answers carry no byte order mark; the request the lab sent begins with one.
    for answer in [SALES_WINDOW, FULL_WINDOW, THREE_VOUCHERS, AUGUST] {
        assert_eq!(&answer[..2], b"<\0");
    }
    for request in [EARLIER_REQUEST, AUGUST_REQUEST] {
        assert_eq!(&request[..2], &[0xFF, 0xFE]);
    }
}

// ---------------------------------------------------------------------------------------------
// The flags.
// ---------------------------------------------------------------------------------------------

/// Every value one flag can take in these tests: the two Tally sends, the absent flag, and three
/// spellings Tally was never seen to send.
const FLAG_VALUES: [Option<&str>; 6] = [
    Some("Yes"),
    Some("No"),
    None,
    Some(""),
    Some("yes"),
    Some("Maybe"),
];

#[test]
fn exactly_the_four_measured_flag_combinations_state_a_status() {
    let measured = [
        ((Some("Yes"), Some("No"), Some("No")), GstStatus::Included),
        ((Some("No"), Some("Yes"), Some("No")), GstStatus::Uncertain),
        ((Some("No"), Some("No"), Some("Yes")), GstStatus::Excluded),
        ((Some("No"), Some("No"), Some("No")), GstStatus::NotInReturn),
    ];
    let (mut states, mut not_reported, mut unreadable) = (0, 0, 0);
    for included in FLAG_VALUES {
        for uncertain in FLAG_VALUES {
            for excluded in FLAG_VALUES {
                let flags = (included, uncertain, excluded);
                let expected = match measured.iter().find(|(measured, _)| *measured == flags) {
                    Some((_, status)) => Ok(*status),
                    None if flags == (None, None, None) => Err(NOT_REPORTED),
                    None => Err(UNREADABLE),
                };
                let got = status_of_flags(included, uncertain, excluded);
                assert_eq!(got, expected, "{flags:?}");
                match got {
                    Ok(_) => states += 1,
                    Err(NOT_REPORTED) => not_reported += 1,
                    Err(_) => unreadable += 1,
                }
            }
        }
    }
    // 6 * 6 * 6 combinations: four states, the one all-absent combination, the rest unreadable.
    assert_eq!((states, not_reported, unreadable), (4, 1, 211));
}

#[test]
fn of_the_twenty_seven_yes_no_absent_combinations_four_are_states_and_one_is_not_reported() {
    let values = [Some("Yes"), Some("No"), None];
    let mut outcomes = Vec::new();
    for included in values {
        for uncertain in values {
            for excluded in values {
                outcomes.push(status_of_flags(included, uncertain, excluded));
            }
        }
    }
    assert_eq!(outcomes.len(), 27);
    assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 4);
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| **outcome == Err(NOT_REPORTED))
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| **outcome == Err(UNREADABLE))
            .count(),
        22
    );
}

#[test]
fn two_yes_flags_a_missing_flag_and_a_misspelt_flag_are_never_a_status() {
    for flags in [
        (Some("Yes"), Some("Yes"), Some("No")),
        (Some("Yes"), Some("Yes"), Some("Yes")),
        (Some("Yes"), Some("No"), None),
        (None, Some("No"), Some("No")),
        (Some("yes"), Some("No"), Some("No")),
        (Some(""), Some("No"), Some("No")),
        (Some("Yes"), Some("No"), Some("Maybe")),
    ] {
        assert_eq!(
            status_of_flags(flags.0, flags.1, flags.2),
            Err(UNREADABLE),
            "{flags:?}"
        );
    }
}

#[test]
fn the_acceptance_flag_is_yes_or_no_and_nothing_else() {
    assert_eq!(overridden_of(Some("Yes")), Ok(true));
    assert_eq!(overridden_of(Some("No")), Ok(false));
    assert_eq!(overridden_of(None), Err(NOT_REPORTED));
    for other in ["", "yes", "no", "YES", "true", "1", "Maybe", " Yes"] {
        assert_eq!(overridden_of(Some(other)), Err(UNREADABLE), "{other:?}");
    }
}

#[test]
fn cancelled_and_optional_are_decided_before_the_three_flags_and_only_from_yes_and_no() {
    let (mut cancelled, mut optional, mut by_flags, mut not_reported, mut unreadable) =
        (0, 0, 0, 0, 0);
    let mut outcomes = std::collections::BTreeMap::<String, usize>::new();
    for is_cancelled in FLAG_VALUES {
        for is_optional in FLAG_VALUES {
            for included in FLAG_VALUES {
                for uncertain in FLAG_VALUES {
                    for excluded in FLAG_VALUES {
                        // A cancelled or optional voucher whose three flags state no measured
                        // state is unread, with the flags' own code.
                        let flags = status_of_flags(included, uncertain, excluded);
                        let expected = match (is_cancelled, is_optional) {
                            (Some("Yes"), Some("No")) => {
                                cancelled += 1;
                                flags.map(|_| GstStatus::Cancelled)
                            }
                            (Some("No"), Some("Yes")) => {
                                optional += 1;
                                flags.map(|_| GstStatus::Optional)
                            }
                            (Some("No"), Some("No")) => {
                                by_flags += 1;
                                status_of_flags(included, uncertain, excluded)
                            }
                            (None, None) => {
                                not_reported += 1;
                                Err(NOT_REPORTED)
                            }
                            _ => {
                                unreadable += 1;
                                Err(UNREADABLE)
                            }
                        };
                        *outcomes.entry(format!("{expected:?}")).or_default() += 1;
                        let flags = (is_cancelled, is_optional, included, uncertain, excluded);
                        assert_eq!(
                            status_of_voucher(
                                is_cancelled,
                                is_optional,
                                included,
                                uncertain,
                                excluded
                            ),
                            expected,
                            "{flags:?}"
                        );
                    }
                }
            }
        }
    }
    // 216 combinations of the three status flags for each of the 36 pairs: one pair each is
    // cancelled, optional, read by its status flags and not reported; the 32 others are unread.
    assert_eq!(
        (cancelled, optional, by_flags, not_reported, unreadable),
        (216, 216, 216, 216, 32 * 216)
    );
    // Of the 7,776, a state is read only where the pair is one of three and the flags are one of
    // the four measured combinations: four cancelled, four optional, four by the flags.
    let count = |key: &str| outcomes.get(key).copied().unwrap_or(0);
    assert_eq!(count("Ok(Cancelled)"), 4);
    assert_eq!(count("Ok(Optional)"), 4);
    assert_eq!(
        count("Ok(Included)")
            + count("Ok(Uncertain)")
            + count("Ok(Excluded)")
            + count("Ok(NotInReturn)"),
        4
    );
    // Not reported: the 216 with both absent, and the one all-absent triple of each of the three.
    assert_eq!(count(&format!("Err({NOT_REPORTED:?})")), 219);
    assert_eq!(count(&format!("Err({UNREADABLE:?})")), 7_776 - 12 - 219);
}

#[test]
fn the_measured_cancelled_and_optional_vouchers_are_neither_included_nor_not_in_the_return() {
    // `BP/26-27/0041`, cancelled, reads the included flag Yes; `BP/26-27/0042`, optional, reads
    // all three No.
    let yes_no_no = (Some("Yes"), Some("No"), Some("No"));
    let no_no_no = (Some("No"), Some("No"), Some("No"));
    assert_eq!(
        status_of_voucher(
            Some("Yes"),
            Some("No"),
            yes_no_no.0,
            yes_no_no.1,
            yes_no_no.2
        ),
        Ok(GstStatus::Cancelled)
    );
    assert_eq!(
        status_of_voucher(Some("No"), Some("Yes"), no_no_no.0, no_no_no.1, no_no_no.2),
        Ok(GstStatus::Optional)
    );
    // The same flags on a voucher that is neither read as the flags say.
    assert_eq!(
        status_of_voucher(
            Some("No"),
            Some("No"),
            yes_no_no.0,
            yes_no_no.1,
            yes_no_no.2
        ),
        Ok(GstStatus::Included)
    );
    assert_eq!(
        status_of_voucher(Some("No"), Some("No"), no_no_no.0, no_no_no.1, no_no_no.2),
        Ok(GstStatus::NotInReturn)
    );
    // A cancelled voucher whose flags state no measured state (two Yes) is unread.
    assert_eq!(
        status_of_voucher(
            Some("Yes"),
            Some("No"),
            Some("Yes"),
            Some("Yes"),
            Some("No")
        ),
        Err(UNREADABLE)
    );
    // Both Yes was never seen, and one of the two absent is never No.
    for (is_cancelled, is_optional) in [
        (Some("Yes"), Some("Yes")),
        (None, Some("No")),
        (Some("No"), None),
        (None, Some("Yes")),
        (Some("Yes"), None),
    ] {
        assert_eq!(
            status_of_voucher(
                is_cancelled,
                is_optional,
                yes_no_no.0,
                yes_no_no.1,
                yes_no_no.2
            ),
            Err(UNREADABLE),
            "{is_cancelled:?} {is_optional:?}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// The request.
// ---------------------------------------------------------------------------------------------

#[test]
fn the_renderer_reproduces_the_request_the_lab_sent_character_for_character() {
    let sent = decode_utf16(AUGUST_REQUEST);
    let sent = sent.strip_prefix('\u{feff}').expect("a byte order mark");
    let rendered = read_profiles::render_gst_status_window(
        LAB_COMPANY,
        &tally_date("20260801"),
        &tally_date("20260831"),
        LAB_TYPE,
    )
    .unwrap();
    assert_eq!(rendered, sent);
    // The read built for the tool is the same text.
    let request = read_profiles::gst_status_read(
        LAB_COMPANY,
        &tally_date("20260801"),
        &tally_date("20260831"),
        LAB_TYPE,
    )
    .unwrap();
    assert_eq!(request.into_xml(), sent);
}

#[test]
fn the_earlier_request_differs_only_by_the_cancelled_and_optional_field_names() {
    let earlier = decode_utf16(EARLIER_REQUEST);
    let earlier = earlier.strip_prefix('\u{feff}').expect("a byte order mark");
    let rendered = read_profiles::render_gst_status_window(
        LAB_COMPANY,
        &tally_date("20260802"),
        &tally_date("20260803"),
        LAB_TYPE,
    )
    .unwrap();
    assert_ne!(rendered, earlier);
    assert_eq!(
        rendered,
        once(
            earlier,
            "ISGSTOVERRIDDEN</FETCH>",
            "ISGSTOVERRIDDEN,ISCANCELLED,ISOPTIONAL</FETCH>"
        )
    );
}

#[test]
fn a_type_name_is_admitted_only_when_it_cannot_break_the_string_literal_it_is_quoted_into() {
    let safe = read_profiles::gst_status_type_name_literal_safe;
    for admitted in [
        "BRIDGE Sales",
        "BRIDGE Sales Z1",
        "a",
        "Part and Labour Sale",
    ] {
        assert!(safe(admitted), "{admitted:?}");
    }
    // One hundred characters, counted as characters and not bytes.
    assert!(safe(&"a".repeat(100)));
    assert!(safe(&"\u{e9}".repeat(100)));
    for refused in [
        String::new(),
        "a".repeat(101),
        "\u{e9}".repeat(101),
        "BRIDGE \"Sales".to_string(),
        "\"".to_string(),
        "BRIDGE\\Sales".to_string(),
        "BRIDGE\u{7}Sales".to_string(),
        "BRIDGE\nSales".to_string(),
        "BRIDGE\tSales".to_string(),
        "BRIDGE\0Sales".to_string(),
        "BRIDGE\u{85}Sales".to_string(),
        " BRIDGE Sales".to_string(),
        "BRIDGE Sales ".to_string(),
        " ".to_string(),
        "R&D Sales".to_string(),
        "Sales <Export>".to_string(),
        "Sales >".to_string(),
        "Bob's Sales".to_string(),
        "x&quot; OR $VoucherTypeName <> &quot;y".to_string(),
    ] {
        assert!(!safe(&refused), "{refused:?}");
    }
}

#[test]
fn the_renderer_quotes_the_type_name_and_refuses_an_unsafe_one() {
    let (from, to) = (tally_date("20260802"), tally_date("20260803"));
    let rendered =
        read_profiles::render_gst_status_window(LAB_COMPANY, &from, &to, "BRIDGE Sales").unwrap();
    assert!(
        rendered.contains("AND $VoucherTypeName = \"BRIDGE Sales\"</SYSTEM>"),
        "{rendered}"
    );
    for refused in [
        "",
        "a\"b",
        "a\\b",
        "a\u{7}b",
        " a",
        "a ",
        "R&D <Sales>",
        &"a".repeat(101),
    ] {
        assert_eq!(
            read_profiles::render_gst_status_window(LAB_COMPANY, &from, &to, refused),
            Err("gst_status_type_name_invalid".to_string()),
            "{refused:?}"
        );
        assert_eq!(
            read_profiles::gst_status_read(LAB_COMPANY, &from, &to, refused).map(|_| ()),
            Err("gst_status_type_name_invalid".to_string()),
            "{refused:?}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// The parse of each committed answer.
// ---------------------------------------------------------------------------------------------

/// Voucher number, date, type, master id, alter id, status, accepted as it stands.
type Expected = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    GstStatus,
    bool,
);

/// The rows of an answer to the earlier request: each voucher identified and its acceptance read,
/// and no state, since whether it is cancelled or optional was not fetched.
fn earlier_rows(expected: &[Expected]) -> Vec<StatusRow> {
    expected_rows(expected)
        .into_iter()
        .map(|row| StatusRow {
            status: Err(NOT_REPORTED),
            ..row
        })
        .collect()
}

fn expected_rows(expected: &[Expected]) -> Vec<StatusRow> {
    expected
        .iter()
        .map(
            |(number, day, voucher_type, master, alter, status, accepted)| StatusRow {
                voucher_number: number.to_string(),
                date: day.to_string(),
                voucher_type: voucher_type.to_string(),
                master_id: master.to_string(),
                alter_id: alter.to_string(),
                status: Ok(*status),
                overridden: Ok(*accepted),
            },
        )
        .collect()
}

/// The vouchers of the 12 Oct answer, in the answer's order (taken from its bytes by a separate
/// script): 25 included (five of them accepted as they stand), five uncertain, `BP/26-27/0041`
/// cancelled and `BP/26-27/0042` optional.
#[rustfmt::skip]
const AUGUST_ROWS: [Expected; 32] = [
    ("BP/26-27/0001", "20260801", "BRIDGE Sales", "1", "2", GstStatus::Included, false),
    ("BP/26-27/0002", "20260801", "BRIDGE Sales", "2", "11", GstStatus::Uncertain, false),
    ("BP/26-27/0003", "20260801", "BRIDGE Sales", "3", "50", GstStatus::Uncertain, false),
    ("BP/26-27/0004", "20260801", "BRIDGE Sales", "4", "13", GstStatus::Included, false),
    ("BP/26-27/0005", "20260801", "BRIDGE Sales", "5", "54", GstStatus::Included, true),
    ("BP/26-27/0006", "20260801", "BRIDGE Sales", "6", "48", GstStatus::Included, true),
    ("BP/26-27/0007", "20260801", "BRIDGE Sales", "7", "8", GstStatus::Included, false),
    ("BP/26-27/0008", "20260801", "BRIDGE Sales", "8", "9", GstStatus::Uncertain, false),
    ("BP/26-27/0009", "20260801", "BRIDGE Sales", "9", "10", GstStatus::Included, false),
    ("BP/26-27/0011", "20260801", "BRIDGE Sales", "10", "16", GstStatus::Included, false),
    ("BP/26-27/0010", "20260801", "BRIDGE Sales", "11", "17", GstStatus::Included, false),
    ("BP/26-27/0012", "20260802", "BRIDGE Sales", "16", "23", GstStatus::Included, false),
    ("BP/26-27/0013", "20260802", "BRIDGE Sales", "17", "24", GstStatus::Included, false),
    ("BP/26-27/0014", "20260802", "BRIDGE Sales", "18", "25", GstStatus::Included, false),
    ("BP/26-27/0015", "20260802", "BRIDGE Sales", "19", "26", GstStatus::Included, false),
    ("BP/26-27/0016", "20260802", "BRIDGE Sales", "20", "28", GstStatus::Included, true),
    ("BP/26-27/0019", "20260802", "BRIDGE Sales", "22", "30", GstStatus::Included, false),
    ("BP/26-27/0021", "20260805", "BRIDGE Sales", "26", "35", GstStatus::Included, false),
    ("5", "20260807", "BRIDGE Sales", "27", "36", GstStatus::Uncertain, false),
    ("6", "20260807", "BRIDGE Sales", "28", "37", GstStatus::Uncertain, false),
    ("7", "20260807", "BRIDGE Sales", "29", "38", GstStatus::Included, false),
    ("8", "20260807", "BRIDGE Sales", "30", "39", GstStatus::Included, false),
    ("9", "20260807", "BRIDGE Sales", "31", "40", GstStatus::Included, false),
    ("10", "20260807", "BRIDGE Sales", "32", "41", GstStatus::Included, false),
    ("11", "20260807", "BRIDGE Sales", "33", "42", GstStatus::Included, false),
    ("12", "20260807", "BRIDGE Sales", "34", "43", GstStatus::Included, false),
    ("13", "20260807", "BRIDGE Sales", "35", "44", GstStatus::Included, false),
    ("14", "20260807", "BRIDGE Sales", "36", "45", GstStatus::Included, false),
    ("BP/26-27/0041", "20260820", "BRIDGE Sales", "40", "63", GstStatus::Cancelled, false),
    ("BP/26-27/0042", "20260820", "BRIDGE Sales", "41", "62", GstStatus::Optional, false),
    ("BP/26-27/0043", "20260822", "BRIDGE Sales", "43", "67", GstStatus::Included, true),
    ("BP/26-27/0044", "20260822", "BRIDGE Sales", "44", "68", GstStatus::Included, true),
];

#[rustfmt::skip]
const SALES_WINDOW_ROWS: [Expected; 6] = [
    ("BP/26-27/0012", "20260802", "BRIDGE Sales", "16", "23", GstStatus::Included, false),
    ("BP/26-27/0013", "20260802", "BRIDGE Sales", "17", "24", GstStatus::Included, false),
    ("BP/26-27/0014", "20260802", "BRIDGE Sales", "18", "25", GstStatus::Included, false),
    ("BP/26-27/0015", "20260802", "BRIDGE Sales", "19", "26", GstStatus::Included, false),
    ("BP/26-27/0016", "20260802", "BRIDGE Sales", "20", "28", GstStatus::Included, true),
    ("BP/26-27/0019", "20260802", "BRIDGE Sales", "22", "30", GstStatus::Included, false),
];

#[rustfmt::skip]
const FULL_WINDOW_ROWS: [Expected; 11] = [
    ("BP/26-27/0012", "20260802", "BRIDGE Sales", "16", "23", GstStatus::Included, false),
    ("BP/26-27/0013", "20260802", "BRIDGE Sales", "17", "24", GstStatus::Included, false),
    ("BP/26-27/0014", "20260802", "BRIDGE Sales", "18", "25", GstStatus::Included, false),
    ("BP/26-27/0015", "20260802", "BRIDGE Sales", "19", "26", GstStatus::Included, false),
    ("BP/26-27/0016", "20260802", "BRIDGE Sales", "20", "28", GstStatus::Included, true),
    ("Z1/0001", "20260802", "BRIDGE Sales Z1", "21", "29", GstStatus::Included, false),
    ("BP/26-27/0019", "20260802", "BRIDGE Sales", "22", "30", GstStatus::Included, false),
    ("1", "20260803", "Receipt", "12", "19", GstStatus::NotInReturn, false),
    ("2", "20260803", "Receipt", "13", "20", GstStatus::NotInReturn, false),
    ("3", "20260803", "Receipt", "14", "21", GstStatus::NotInReturn, false),
    ("4", "20260803", "Receipt", "15", "22", GstStatus::NotInReturn, false),
];

#[rustfmt::skip]
const THREE_VOUCHER_ROWS: [Expected; 3] = [
    ("BP/26-27/0002", "20260801", "BRIDGE Sales", "2", "11", GstStatus::Uncertain, false),
    ("BP/26-27/0010", "20260801", "BRIDGE Sales", "11", "17", GstStatus::Included, false),
    ("BP/26-27/0016", "20260802", "BRIDGE Sales", "20", "28", GstStatus::Included, true),
];

#[test]
fn the_august_answer_reads_the_cancelled_and_the_optional_voucher_apart_from_the_others() {
    let rows = parse_status_rows(&august()).unwrap();
    assert_eq!(rows, expected_rows(&AUGUST_ROWS));
    let state = |number: &str| {
        let row = rows
            .iter()
            .find(|row| row.voucher_number == number)
            .unwrap();
        (row.status, row.overridden)
    };
    // The cancelled voucher's included flag is Yes and the optional one's three flags are No.
    assert_eq!(
        state("BP/26-27/0041"),
        (Ok(GstStatus::Cancelled), Ok(false))
    );
    assert_eq!(state("BP/26-27/0042"), (Ok(GstStatus::Optional), Ok(false)));
    let count = |status: GstStatus| rows.iter().filter(|row| row.status == Ok(status)).count();
    assert_eq!(
        [
            count(GstStatus::Included),
            count(GstStatus::Uncertain),
            count(GstStatus::Excluded),
            count(GstStatus::NotInReturn),
            count(GstStatus::Cancelled),
            count(GstStatus::Optional),
        ],
        [25, 5, 0, 0, 1, 1]
    );
}

#[test]
fn an_answer_to_the_earlier_request_reads_every_voucher_unread_never_included() {
    // The six vouchers' status flags are all there, but not whether each is cancelled or
    // optional: none has a state, and each keeps its identity and its acceptance flag.
    let rows = parse_status_rows(&decode_utf16(SALES_WINDOW)).unwrap();
    assert_eq!(rows, earlier_rows(&SALES_WINDOW_ROWS));
}

#[test]
fn the_unfiltered_answer_to_the_earlier_request_reads_every_voucher_unread_with_its_type() {
    let rows = parse_status_rows(&decode_utf16(FULL_WINDOW)).unwrap();
    assert_eq!(rows, earlier_rows(&FULL_WINDOW_ROWS));
    // The four Receipts read all three flags No, which is no longer read as not in the return.
    assert_eq!(
        rows.iter()
            .filter(|row| row.voucher_type == "Receipt" && row.status == Err(NOT_REPORTED))
            .count(),
        4
    );
}

#[test]
fn the_wider_field_list_answer_reads_uncertain_included_and_included_accepted() {
    // It carries `&#4;` references, which the parse marks and cleans, and some forty fields a
    // voucher beyond the six it reads, the cancelled and optional flags (No) among them.
    let text = decode_utf16(THREE_VOUCHERS);
    assert!(text.contains("&#4;"));
    let rows = parse_status_rows(&text).unwrap();
    assert_eq!(rows, expected_rows(&THREE_VOUCHER_ROWS));
}

#[test]
fn an_answer_with_a_collection_and_no_voucher_is_an_empty_list() {
    let text = august();
    let open = text.find("<COLLECTION").unwrap();
    let close = text.find("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    let empty = format!(
        "{}<COLLECTION></COLLECTION>{}",
        &text[..open],
        &text[close..]
    );
    assert_eq!(parse_status_rows(&empty).unwrap(), Vec::<StatusRow>::new());
}

// ---------------------------------------------------------------------------------------------
// Mutations of the August answer. Each is an exact replacement, asserted to match once.
// ---------------------------------------------------------------------------------------------

/// The voucher every mutation below is made on (master id 17, the thirteenth row), and its six
/// status elements as Tally wrote them.
const TARGET: &str = "BP/26-27/0013";
const INCLUDED: &str = r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Yes</VCHGSTSTATUSISINCLUDED>"#;
const UNCERTAIN: &str = r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">No</VCHGSTSTATUSISUNCERTAIN>"#;
const EXCLUDED: &str = r#"<VCHGSTSTATUSISEXCLUDED TYPE="Logical">No</VCHGSTSTATUSISEXCLUDED>"#;
const OVERRIDDEN: &str = r#"<ISGSTOVERRIDDEN TYPE="Logical">No</ISGSTOVERRIDDEN>"#;
const CANCELLED: &str = r#"<ISCANCELLED TYPE="Logical">No</ISCANCELLED>"#;
const OPTIONAL: &str = r#"<ISOPTIONAL TYPE="Logical">No</ISOPTIONAL>"#;
/// The target's position in the answer, and in the tool's items (by date, then number).
const TARGET_AT: usize = 12;

fn august() -> String {
    decode_utf16(AUGUST)
}

/// `haystack` with the one occurrence of `from` replaced; any other count fails the test.
fn once(haystack: &str, from: &str, to: &str) -> String {
    assert_eq!(haystack.matches(from).count(), 1, "{from}");
    haystack.replacen(from, to, 1)
}

/// `xml` with the voucher numbered `number` changed by `change`.
fn edit(xml: &str, number: &str, change: impl FnOnce(String) -> String) -> String {
    let marker = format!("<VOUCHERNUMBER>{number}</VOUCHERNUMBER>");
    assert_eq!(xml.matches(&marker).count(), 1, "{marker}");
    let at = xml.find(&marker).unwrap();
    let start = xml[..at].rfind("<VOUCHER ").unwrap();
    let end = at + xml[at..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let changed = change(xml[start..end].to_string());
    format!("{}{}{}", &xml[..start], changed, &xml[end..])
}

/// The target voucher's row after `change`, with the other 31 rows asserted to read exactly as
/// in the committed answer.
fn target_after(change: impl FnOnce(String) -> String) -> StatusRow {
    let rows = parse_status_rows(&edit(&august(), TARGET, change)).unwrap();
    assert_eq!(rows.len(), 32);
    assert_eq!(rows[TARGET_AT].voucher_number, TARGET);
    let mut target = None;
    for row in rows {
        if row.voucher_number == TARGET {
            assert_eq!(row.master_id, "17");
            target = Some(row);
        } else {
            let expected = AUGUST_ROWS
                .iter()
                .find(|expected| expected.0 == row.voucher_number)
                .unwrap();
            assert_eq!(row, expected_rows(&[*expected]).remove(0));
        }
    }
    target.expect("the target voucher keeps its row")
}

#[test]
fn the_unmutated_target_reads_included_so_each_mutation_below_is_the_cause() {
    let target = target_after(|block| block);
    assert_eq!(target.status, Ok(GstStatus::Included));
    assert_eq!(target.overridden, Ok(false));
}

#[test]
fn a_cancelled_or_optional_flag_decides_the_state_and_an_unreadable_one_leaves_none() {
    let yes = |element: &str| element.replace(">No<", ">Yes<");
    // Yes on either flag decides the state over the measured flags beside it; the acceptance is
    // still read.
    let target = target_after(|block| once(&block, CANCELLED, &yes(CANCELLED)));
    assert_eq!(
        (target.status, target.overridden),
        (Ok(GstStatus::Cancelled), Ok(false))
    );
    let target = target_after(|block| once(&block, OPTIONAL, &yes(OPTIONAL)));
    assert_eq!(
        (target.status, target.overridden),
        (Ok(GstStatus::Optional), Ok(false))
    );
    // A cancelled voucher whose three status flags are absent is not reported, and one spelt
    // another way or repeated leaves it unread: the answer around the cancelled flag is not read.
    let target = target_after(|block| {
        [INCLUDED, UNCERTAIN, EXCLUDED].iter().fold(
            once(&block, CANCELLED, &yes(CANCELLED)),
            |block, element| once(&block, element, ""),
        )
    });
    assert_eq!(
        (target.status, target.overridden),
        (Err(NOT_REPORTED), Ok(false))
    );
    let target = target_after(|block| {
        once(
            &once(&block, CANCELLED, &yes(CANCELLED)),
            INCLUDED,
            &INCLUDED.replace(">Yes<", ">yes<"),
        )
    });
    assert_eq!(
        (target.status, target.overridden),
        (Err(UNREADABLE), Ok(false))
    );
    let target = target_after(|block| {
        once(
            &once(&block, CANCELLED, &yes(CANCELLED)),
            INCLUDED,
            &format!("{INCLUDED}{INCLUDED}"),
        )
    });
    assert_eq!(
        (target.status, target.overridden),
        (Err(UNREADABLE), Err(UNREADABLE))
    );
    // Both Yes; one removed; both removed; spelt another way, empty or closed empty: no state,
    // and the acceptance is still read.
    let both = target_after(|block| {
        once(
            &once(&block, CANCELLED, &yes(CANCELLED)),
            OPTIONAL,
            &yes(OPTIONAL),
        )
    });
    assert_eq!((both.status, both.overridden), (Err(UNREADABLE), Ok(false)));
    for element in [CANCELLED, OPTIONAL] {
        let name = element[1..element.find(' ').unwrap()].to_string();
        for changed in [
            String::new(),
            element.replace(">No<", ">no<"),
            element.replace(">No<", "><"),
            format!(r#"<{name} TYPE="Logical"/>"#),
        ] {
            let target = target_after(|block| once(&block, element, &changed));
            assert_eq!(
                (target.status, target.overridden),
                (Err(UNREADABLE), Ok(false)),
                "{changed:?}"
            );
        }
    }
    let neither = target_after(|block| once(&once(&block, CANCELLED, ""), OPTIONAL, ""));
    assert_eq!(
        (neither.status, neither.overridden),
        (Err(NOT_REPORTED), Ok(false))
    );
    // Repeated, or inside another element: neither the state nor the acceptance is read.
    for element in [CANCELLED, OPTIONAL] {
        for changed in [
            format!("{element}{element}"),
            format!("<WRAPPER>{element}</WRAPPER>"),
            format!("{element}<WRAPPER>{}</WRAPPER>", yes(element)),
        ] {
            let target = target_after(|block| once(&block, element, &changed));
            assert_eq!(
                (target.status, target.overridden),
                (Err(UNREADABLE), Err(UNREADABLE)),
                "{changed}"
            );
        }
    }
}

#[test]
fn a_status_element_repeated_in_one_voucher_keeps_the_voucher_and_reads_no_status() {
    for element in [INCLUDED, UNCERTAIN, EXCLUDED, OVERRIDDEN] {
        let target = target_after(|block| once(&block, element, &format!("{element}{element}")));
        assert_eq!(target.status, Err(UNREADABLE), "{element}");
        assert_eq!(target.overridden, Err(UNREADABLE), "{element}");
    }
    // A repeat that disagrees with the first is no more readable.
    let target = target_after(|block| {
        once(
            &block,
            INCLUDED,
            &format!("{INCLUDED}{}", INCLUDED.replace(">Yes<", ">No<")),
        )
    });
    assert_eq!(target.status, Err(UNREADABLE));
    assert_eq!(target.overridden, Err(UNREADABLE));
}

#[test]
fn a_status_element_inside_another_element_of_the_voucher_is_not_the_vouchers_own() {
    for element in [INCLUDED, UNCERTAIN, EXCLUDED, OVERRIDDEN] {
        let target =
            target_after(|block| once(&block, element, &format!("<WRAPPER>{element}</WRAPPER>")));
        assert_eq!(target.status, Err(UNREADABLE), "{element}");
        assert_eq!(target.overridden, Err(UNREADABLE), "{element}");
    }
    // A wrapper that sits beside a direct copy: the nested copy still makes the voucher unread.
    let target = target_after(|block| {
        once(
            &block,
            INCLUDED,
            &format!("{INCLUDED}<WRAPPER>{INCLUDED}</WRAPPER>"),
        )
    });
    assert_eq!(target.status, Err(UNREADABLE));
    assert_eq!(target.overridden, Err(UNREADABLE));
}

#[test]
fn status_elements_two_wrappers_down_with_no_direct_copy_are_never_read_as_a_status() {
    // A status element nested at any depth is not the voucher's own: two wrappers down it is
    // refused as the one wrapper down is, as unreadable and not as absent.
    let target = target_after(|block| {
        let mut block = block;
        for element in [INCLUDED, UNCERTAIN, EXCLUDED, OVERRIDDEN] {
            block = once(&block, element, "");
        }
        once(
            &block,
            "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>",
            &format!(
                "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME><OUTER><INNER>{INCLUDED}{UNCERTAIN}{EXCLUDED}{OVERRIDDEN}</INNER></OUTER>"
            ),
        )
    });
    assert_eq!(target.status, Err(UNREADABLE));
    assert_eq!(target.overridden, Err(UNREADABLE));
}

#[test]
fn removed_status_elements_are_not_reported_and_a_partly_removed_set_is_unreadable() {
    // All four absent: the answer says nothing, for the status and for the acceptance.
    let target = target_after(|block| {
        [INCLUDED, UNCERTAIN, EXCLUDED, OVERRIDDEN]
            .iter()
            .fold(block, |block, element| once(&block, element, ""))
    });
    assert_eq!(target.status, Err(NOT_REPORTED));
    assert_eq!(target.overridden, Err(NOT_REPORTED));
    // The three status flags absent and the acceptance present: no status, the acceptance read.
    let target = target_after(|block| {
        [INCLUDED, UNCERTAIN, EXCLUDED]
            .iter()
            .fold(block, |block, element| once(&block, element, ""))
    });
    assert_eq!(target.status, Err(NOT_REPORTED));
    assert_eq!(target.overridden, Ok(false));
    // One of the three absent while the others are present: unreadable, never No.
    for element in [INCLUDED, UNCERTAIN, EXCLUDED] {
        let target = target_after(|block| once(&block, element, ""));
        assert_eq!(target.status, Err(UNREADABLE), "{element}");
        assert_eq!(target.overridden, Ok(false), "{element}");
    }
    // The acceptance flag absent: the status is still read, and the acceptance is not reported.
    let target = target_after(|block| once(&block, OVERRIDDEN, ""));
    assert_eq!(target.status, Ok(GstStatus::Included));
    assert_eq!(target.overridden, Err(NOT_REPORTED));
}

#[test]
fn a_flag_spelt_in_lower_case_left_empty_or_closed_empty_is_unreadable() {
    let empty = |element: &str| element.replace(">Yes<", "><").replace(">No<", "><");
    let lower = |element: &str| element.replace(">Yes<", ">yes<").replace(">No<", ">no<");
    let self_closing = |name: &str| format!(r#"<{name} TYPE="Logical"/>"#);
    for element in [INCLUDED, UNCERTAIN, EXCLUDED] {
        let name = element[1..element.find(' ').unwrap()].to_string();
        for changed in [lower(element), empty(element), self_closing(&name)] {
            let target = target_after(|block| once(&block, element, &changed));
            assert_eq!(target.status, Err(UNREADABLE), "{changed}");
            assert_eq!(target.overridden, Ok(false), "{changed}");
        }
    }
    for changed in [
        lower(OVERRIDDEN),
        empty(OVERRIDDEN),
        self_closing("ISGSTOVERRIDDEN"),
    ] {
        let target = target_after(|block| once(&block, OVERRIDDEN, &changed));
        assert_eq!(target.status, Ok(GstStatus::Included), "{changed}");
        assert_eq!(target.overridden, Err(UNREADABLE), "{changed}");
    }
    // Two flags Yes at once.
    let target = target_after(|block| once(&block, UNCERTAIN, &UNCERTAIN.replace(">No<", ">Yes<")));
    assert_eq!(target.status, Err(UNREADABLE));
}

#[test]
fn a_voucher_row_repeated_with_the_same_master_id_refuses_the_whole_answer() {
    let repeated = edit(&august(), TARGET, |block| format!("{block}{block}"));
    assert_eq!(
        parse_status_rows(&repeated).unwrap_err(),
        "gst_status_read_voucher_repeated"
    );
    // The same master id under another voucher number is the same voucher row.
    let renumbered = edit(&august(), TARGET, |block| {
        let second = once(&block, "BP/26-27/0013", "BP/26-27/9999");
        format!("{block}{second}")
    });
    assert_eq!(
        parse_status_rows(&renumbered).unwrap_err(),
        "gst_status_read_voucher_repeated"
    );
}

#[test]
fn a_voucher_that_cannot_be_told_apart_refuses_the_whole_answer() {
    let master = r#"<MASTERID TYPE="Number"> 17</MASTERID>"#;
    let alter = r#"<ALTERID TYPE="Number"> 24</ALTERID>"#;
    let day = r#"<DATE TYPE="Date">20260802</DATE>"#;
    let kind = "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>";
    let number = "<VOUCHERNUMBER>BP/26-27/0013</VOUCHERNUMBER>";
    let window = august();
    let cases = [
        (
            "no master id",
            edit(&window, TARGET, |b| once(&b, master, "")),
        ),
        (
            "an empty master id",
            edit(&window, TARGET, |b| {
                once(&b, master, r#"<MASTERID TYPE="Number"></MASTERID>"#)
            }),
        ),
        (
            "a master id twice",
            edit(&window, TARGET, |b| {
                once(&b, master, &format!("{master}{master}"))
            }),
        ),
        (
            "a master id twice, disagreeing",
            edit(&window, TARGET, |b| {
                once(&b, master, &format!("{master}<MASTERID> 99</MASTERID>"))
            }),
        ),
        (
            "no voucher number",
            edit(&window, TARGET, |b| once(&b, number, "")),
        ),
        (
            "a voucher number twice",
            edit(&window, TARGET, |b| {
                once(
                    &b,
                    number,
                    &format!("{number}<VOUCHERNUMBER>X</VOUCHERNUMBER>"),
                )
            }),
        ),
        (
            "an empty voucher number",
            edit(&window, TARGET, |b| {
                once(&b, number, "<VOUCHERNUMBER></VOUCHERNUMBER>")
            }),
        ),
        (
            "no alter id",
            edit(&window, TARGET, |b| once(&b, alter, "")),
        ),
        ("no date", edit(&window, TARGET, |b| once(&b, day, ""))),
        (
            "no voucher type",
            edit(&window, TARGET, |b| once(&b, kind, "")),
        ),
        (
            "a voucher type twice",
            edit(&window, TARGET, |b| {
                once(&b, kind, &format!("{kind}{kind}"))
            }),
        ),
    ];
    for (name, mutated) in cases {
        assert_eq!(
            parse_status_rows(&mutated).unwrap_err(),
            "gst_status_read_voucher_unidentified",
            "{name}"
        );
    }
}

#[test]
fn an_answer_whose_status_is_not_one_is_refused_before_any_row_is_read() {
    // The shared envelope admission refuses a failed status first, so the parse's own
    // `gst_status_read_status_not_success` is not the code a STATUS of 0 reaches.
    for status in ["0", "2", ""] {
        let mutated = once(
            &august(),
            "<STATUS>1</STATUS>",
            &format!("<STATUS>{status}</STATUS>"),
        );
        assert_eq!(
            parse_status_rows(&mutated).unwrap_err(),
            "gst_status_read_protocol_invalid",
            "{status:?}"
        );
    }
    let without = once(&august(), "<STATUS>1</STATUS>", "");
    assert_eq!(
        parse_status_rows(&without).unwrap_err(),
        "gst_status_read_protocol_invalid"
    );
}

#[test]
fn a_truncated_answer_is_a_protocol_error_and_yields_no_row() {
    let text = august();
    assert!(text.is_ascii(), "byte offsets are character offsets");
    let before_close = text.rfind("</ENVELOPE>").unwrap();
    for end in [
        text.len() / 2,
        text.len() * 3 / 4,
        before_close,
        before_close + 5,
    ] {
        assert_eq!(
            parse_status_rows(&text[..end]).unwrap_err(),
            "gst_status_read_protocol_invalid",
            "cut at {end} of {}",
            text.len()
        );
    }
    assert_eq!(
        parse_status_rows("").unwrap_err(),
        "gst_status_read_protocol_invalid"
    );
}

#[test]
fn an_answer_with_no_collection_or_no_data_names_the_missing_collection() {
    let text = august();
    let open = r#"<COLLECTION ISCMPDEPTYPE="Yes" CMPLOCUS="4" CMPDEPTYPE="64">"#;
    let renamed = once(&text, open, &open.replace("<COLLECTION", "<COLLECTIONS"));
    let no_collection = once(&renamed, "</COLLECTION>", "</COLLECTIONS>");
    assert_eq!(
        parse_status_rows(&no_collection).unwrap_err(),
        "gst_status_read_collection_absent"
    );
    let renamed = once(&text, "<DATA>", "<DATAX>");
    let no_data = once(&renamed, "</DATA>", "</DATAX>");
    assert_eq!(
        parse_status_rows(&no_data).unwrap_err(),
        "gst_status_read_collection_absent"
    );
}

#[test]
fn a_second_collection_or_a_voucher_outside_the_collection_refuses_the_answer() {
    let text = august();
    let close = "</COLLECTION>";
    let second = once(
        &text,
        close,
        &format!("{close}<COLLECTION><VOUCHER><MASTERID>9</MASTERID></VOUCHER></COLLECTION>"),
    );
    let wrapped = once(
        &text,
        close,
        "<WRAP><VOUCHER><MASTERID>9</MASTERID></VOUCHER></WRAP></COLLECTION>",
    );
    let beside = once(
        &text,
        close,
        &format!("{close}<VOUCHER><MASTERID>9</MASTERID></VOUCHER>"),
    );
    for changed in [second, wrapped, beside] {
        assert_eq!(
            parse_status_rows(&changed).unwrap_err(),
            "gst_status_read_collection_unexpected"
        );
    }
}

#[test]
fn a_line_error_or_an_error_element_anywhere_in_the_answer_refuses_it() {
    let text = august();
    let in_collection = once(
        &text,
        "</COLLECTION>",
        "<LINEERROR>Could not find the voucher type</LINEERROR></COLLECTION>",
    );
    let in_voucher = edit(&text, TARGET, |block| {
        once(
            &block,
            INCLUDED,
            &format!("{INCLUDED}<LINEERROR>x</LINEERROR>"),
        )
    });
    let lower_case = once(&text, "</COLLECTION>", "<error>x</error></COLLECTION>");
    let response = once(
        &text,
        "</COLLECTION>",
        "<RESPONSE>x</RESPONSE></COLLECTION>",
    );
    for mutated in [in_collection, in_voucher, lower_case, response] {
        assert_eq!(
            parse_status_rows(&mutated).unwrap_err(),
            "gst_status_read_protocol_invalid"
        );
    }
}

#[test]
fn a_value_written_with_a_character_reference_or_a_cdata_section_reads_as_its_text() {
    let kind = "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>";
    // `&#83;` is an ordinary character reference (an `S`): only references to control characters
    // are marked.
    let referenced = edit(&august(), TARGET, |block| {
        once(
            &block,
            kind,
            "<VOUCHERTYPENAME>BRIDGE &#83;ales</VOUCHERTYPENAME>",
        )
    });
    assert_eq!(
        parse_status_rows(&referenced).unwrap()[TARGET_AT].voucher_type,
        "BRIDGE Sales"
    );
    let cdata = edit(&august(), TARGET, |block| {
        once(
            &block,
            kind,
            "<VOUCHERTYPENAME><![CDATA[BRIDGE Sales]]></VOUCHERTYPENAME>",
        )
    });
    assert_eq!(
        parse_status_rows(&cdata).unwrap()[TARGET_AT].voucher_type,
        "BRIDGE Sales"
    );
}

// ---------------------------------------------------------------------------------------------
// The tool through `call_tool`.
// ---------------------------------------------------------------------------------------------

const GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";
const MARKS_GUID: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";

/// The captured company list, with the test company's name made the lab's, so the request the
/// tool renders is the lab's request.
fn companies() -> String {
    let text = decode_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ));
    assert_eq!(text.matches("Bridge Ageing Lab").count(), 2);
    text.replace("Bridge Ageing Lab", LAB_COMPANY)
}

/// The captured company marks (voucher mark 59), with its GUID made the test company's.
fn marks() -> String {
    let text = decode_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-marks.utf16le.xml"
    ));
    assert_eq!(text.matches(MARKS_GUID).count(), 1);
    text.replace(MARKS_GUID, GUID)
}

/// The same marks with the voucher mark moved to `vouchers`.
fn marks_with_voucher_mark(vouchers: u64) -> String {
    once(
        &marks(),
        r#"<ALTVCHID TYPE="Number"> 59</ALTVCHID>"#,
        &format!(r#"<ALTVCHID TYPE="Number"> {vouchers}</ALTVCHID>"#),
    )
}

fn plan(text: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(text))
        .with_encoding(WireEncoding::Utf16LeNoBom)
        .with_framing(ResponseFraming::ContentLength)
}

fn health() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}

/// A read made twice with a health check after each.
fn pair(plans: &mut Vec<ScenarioPlan>, response: &ScenarioPlan) {
    plans.extend([response.clone(), health(), response.clone(), health()]);
}

/// A read inside an identity bracket: the company list, the paired read, the company list.
fn bracketed(plans: &mut Vec<ScenarioPlan>, response: &ScenarioPlan) {
    let companies = plan(companies());
    plans.push(companies.clone());
    pair(plans, response);
    plans.push(companies);
}

/// The reads up to and including the company marks: the company list (paired), then the marks
/// inside their bracket. Ten requests.
fn plans_to_the_marks(marks: String) -> Vec<ScenarioPlan> {
    let mut plans = Vec::new();
    pair(&mut plans, &plan(companies()));
    bracketed(&mut plans, &plan(marks));
    plans
}

/// A whole call: those ten, then the status read inside its bracket. Sixteen requests.
fn plans_with(marks: String, answer: String) -> Vec<ScenarioPlan> {
    let mut plans = plans_to_the_marks(marks);
    bracketed(&mut plans, &plan(answer));
    plans
}

fn tool_server(port: u16, directory: &std::path::Path) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port,
        },
        data_dir: directory.to_path_buf(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

fn arguments(from: &str, to: &str, type_name: &str) -> Value {
    json!({"company_guid": GUID, "from": from, "to": to, "voucher_type_name": type_name})
}

/// The window and type of the lab's request.
fn lab_arguments() -> Value {
    arguments("20260801", "20260831", LAB_TYPE)
}

struct Call {
    response: Value,
    observed: Vec<tally_protocol_simulator::ObservedRequest>,
    expected: usize,
}

/// A call whose plans are all served: a plan the tool does not send would stall `finish`, and a
/// request it sends beyond them fails its response, so the count is exact.
async fn run(plans: Vec<ScenarioPlan>, args: Value) -> Call {
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = tool_server(simulator.address().port(), directory.path());
    let response = server.call_tool("gst_status", args).await;
    let observed = simulator.finish().unwrap();
    Call {
        response,
        observed,
        expected,
    }
}

/// A call that is refused before any read: the response, and how many requests reached the
/// simulator.
async fn run_expecting_no_read(args: Value) -> (Value, usize) {
    let simulator = SequenceSimulator::spawn(vec![health()]).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = tool_server(simulator.address().port(), directory.path());
    let response = server.call_tool("gst_status", args).await;
    let received = simulator.received();
    simulator.cancel();
    (response, received)
}

fn refusal_code(response: &Value) -> &str {
    assert_eq!(response["isError"], true, "{response}");
    response["structuredContent"]["result"]["error"]["code"]
        .as_str()
        .expect("a refusal carries a code")
}

fn payload(response: &Value) -> &Value {
    assert_eq!(response["isError"], false, "{response}");
    &response["structuredContent"]["result"]
}

fn receipt(response: &Value) -> &Value {
    &response["structuredContent"]["evidence"]
}

fn join(left: &str, right: &str) -> String {
    sha256_hex(format!("{left}:{right}").as_bytes())
}

fn status_name(status: GstStatus) -> &'static str {
    match status {
        GstStatus::Included => "included",
        GstStatus::Uncertain => "uncertain",
        GstStatus::Excluded => "excluded",
        GstStatus::NotInReturn => "not_in_return",
        GstStatus::Cancelled => "cancelled",
        GstStatus::Optional => "optional",
    }
}

/// The August rows as the tool orders its items: by date, then voucher number (as text), then
/// master id.
fn august_items() -> Vec<Value> {
    let mut rows = AUGUST_ROWS.to_vec();
    rows.sort_by(|a, b| (a.1, a.0, a.3).cmp(&(b.1, b.0, b.3)));
    rows.iter().map(item).collect()
}

fn item(expected: &Expected) -> Value {
    json!({
        "voucher_number": expected.0,
        "date": expected.1,
        "voucher_type": expected.2,
        "master_id": expected.3,
        "alter_id": expected.4,
        "gst_status": status_name(expected.5),
        "accepted_as_it_stands": expected.6,
    })
}

#[tokio::test]
async fn a_read_returns_each_vouchers_status_and_sends_the_request_the_lab_sent() {
    let call = run(plans_with(marks(), august()), lab_arguments()).await;
    assert_eq!(
        call.observed.len(),
        call.expected,
        "every planned request was served"
    );
    assert_eq!(call.expected, 16);
    let result = payload(&call.response);
    assert_eq!(result["state"], "complete");
    assert_eq!(
        result["basis"],
        "tally_voucher_gst_status_fields_one_voucher_type_one_window"
    );
    assert_eq!(
        result["window"],
        json!({"from": "20260801", "to": "20260831"})
    );
    assert_eq!(result["voucher_type_name"], LAB_TYPE);
    assert_eq!(result["total"], 32);
    assert_eq!(result["offset"], 0);
    // The cancelled and the optional voucher are counted under their own keys only.
    assert_eq!(
        result["counts"],
        json!({
            "included": 25,
            "uncertain": 5,
            "excluded": 0,
            "not_in_return": 0,
            "cancelled": 1,
            "optional": 1,
            "unread": 0,
            "accepted_as_it_stands": 5,
        })
    );
    assert_eq!(result["items"], Value::Array(august_items()));
    // Ordered by date, then voucher number as text: `10` sorts before `5`.
    let numbers: Vec<&str> = result["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["voucher_number"].as_str().unwrap())
        .collect();
    assert_eq!(
        &numbers[9..12],
        ["BP/26-27/0010", "BP/26-27/0011", "BP/26-27/0012"]
    );
    assert_eq!(&numbers[18..21], ["10", "11", "12"]);
    assert_eq!(
        &numbers[28..],
        [
            "BP/26-27/0041",
            "BP/26-27/0042",
            "BP/26-27/0043",
            "BP/26-27/0044"
        ]
    );
    assert!(result.get("next_offset").is_none(), "{result}");
    let limitations = result["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|line| line.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    for stated in [
        "included is not 'clean'",
        "Receipts only",
        "never counted as included or not_in_return",
        "Post-dated vouchers are not marked",
        "A total of 0 may mean",
        "a voucher saved between two pages",
        "excluded has not been observed",
    ] {
        assert!(limitations.contains(stated), "{stated}");
    }
    assert!(
        !limitations.contains("Receipts and Purchases"),
        "{limitations}"
    );
    // The first-line company block.
    let company = &call.response["structuredContent"]["company"];
    assert_eq!(company["name"], LAB_COMPANY);
    assert_eq!(company["guid"], GUID);
    assert_eq!(company["identity_state"], "verified_tuple");

    // The status read on the wire (the twelfth request, read again as the fourteenth) is the
    // committed request of the lab, byte for byte, byte order mark included.
    assert_eq!(call.observed[11].request_body_sha256, REQUEST_SHA256);
    assert_eq!(call.observed[13].request_body_sha256, REQUEST_SHA256);
    // The receipt commits to the three reads: the company list, the marks and the status read.
    let receipt = receipt(&call.response);
    assert_eq!(receipt["state"], "complete");
    assert!(receipt["reason_code"].is_null(), "{receipt}");
    assert_eq!(
        receipt["request_sha256"],
        join(
            &join(
                &call.observed[0].request_body_sha256,
                &call.observed[5].request_body_sha256
            ),
            &call.observed[11].request_body_sha256,
        )
    );
    let answers = [
        plan(companies()).response_bytes(),
        plan(marks()).response_bytes(),
        plan(august()).response_bytes(),
    ];
    assert_eq!(
        answers[2], AUGUST,
        "the replayed answer is the committed bytes"
    );
    assert_eq!(
        receipt["response_sha256"],
        join(
            &join(&sha256_hex(&answers[0]), &sha256_hex(&answers[1])),
            &sha256_hex(&answers[2])
        )
    );
}

#[tokio::test]
async fn dates_written_with_hyphens_send_the_same_request() {
    let call = run(
        plans_with(marks(), august()),
        arguments("2026-08-01", "2026-08-31", LAB_TYPE),
    )
    .await;
    assert_eq!(call.observed.len(), call.expected);
    assert_eq!(
        payload(&call.response)["window"],
        json!({"from": "20260801", "to": "20260831"})
    );
    assert_eq!(call.observed[11].request_body_sha256, REQUEST_SHA256);
}

#[tokio::test]
async fn a_page_of_ten_names_the_next_offset_and_the_counts_cover_the_whole_window() {
    for (offset, rows, next) in [
        (0_usize, 0..10, Some(10)),
        (10, 10..20, Some(20)),
        (20, 20..30, Some(30)),
        (30, 30..32, None),
    ] {
        let mut args = lab_arguments();
        args["limit"] = json!(10);
        args["offset"] = json!(offset);
        let call = run(plans_with(marks(), august()), args).await;
        assert_eq!(call.observed.len(), call.expected);
        let result = payload(&call.response);
        assert_eq!(result["state"], "complete");
        assert_eq!(result["total"], 32, "offset {offset}");
        assert_eq!(result["offset"], offset);
        assert_eq!(result["counts"]["included"], 25);
        assert_eq!(result["counts"]["cancelled"], 1);
        assert_eq!(result["counts"]["accepted_as_it_stands"], 5);
        assert_eq!(
            result["items"],
            Value::Array(august_items()[rows].to_vec()),
            "offset {offset}"
        );
        match next {
            Some(next) => assert_eq!(result["next_offset"], next),
            None => assert!(result.get("next_offset").is_none(), "{result}"),
        }
    }
}

#[tokio::test]
async fn the_wider_answer_through_the_tool_counts_one_uncertain_and_two_included() {
    let call = run(
        plans_with(marks(), decode_utf16(THREE_VOUCHERS)),
        arguments("20260801", "20260802", LAB_TYPE),
    )
    .await;
    assert_eq!(call.observed.len(), call.expected);
    let result = payload(&call.response);
    assert_eq!(result["state"], "complete");
    assert_eq!(result["total"], 3);
    assert_eq!(
        result["counts"],
        json!({
            "included": 2,
            "uncertain": 1,
            "excluded": 0,
            "not_in_return": 0,
            "cancelled": 0,
            "optional": 0,
            "unread": 0,
            "accepted_as_it_stands": 1,
        })
    );
    // Ordered by date, then voucher number.
    assert_eq!(
        result["items"],
        Value::Array(THREE_VOUCHER_ROWS.iter().map(item).collect())
    );
}

#[tokio::test]
async fn an_answer_to_the_earlier_request_through_the_tool_is_partial_and_counts_nothing_included()
{
    let call = run(
        plans_with(marks(), decode_utf16(SALES_WINDOW)),
        arguments("20260802", "20260803", LAB_TYPE),
    )
    .await;
    assert_eq!(call.observed.len(), call.expected);
    let result = payload(&call.response);
    assert_eq!(result["state"], "partial");
    assert_eq!(result["total"], 6);
    assert_eq!(
        result["counts"],
        json!({
            "included": 0,
            "uncertain": 0,
            "excluded": 0,
            "not_in_return": 0,
            "cancelled": 0,
            "optional": 0,
            "unread": 6,
            "accepted_as_it_stands": 1,
        })
    );
    for item in result["items"].as_array().unwrap() {
        assert_eq!(
            item["gst_status"],
            json!({"unread": NOT_REPORTED}),
            "{item}"
        );
    }
    let receipt = receipt(&call.response);
    assert_eq!(receipt["state"], "partial");
    assert_eq!(receipt["reason_code"], "gst_status_voucher_unread");
}

#[tokio::test]
async fn arguments_that_cannot_be_read_are_refused_before_any_request_is_sent() {
    let long_name = "a".repeat(101);
    for (args, expected) in [
        (
            arguments("20260802", "20260803", "BRIDGE \"Sales"),
            "gst_status_type_name_invalid",
        ),
        (
            arguments("20260802", "20260803", "BRIDGE\\Sales"),
            "gst_status_type_name_invalid",
        ),
        (
            arguments("20260802", "20260803", "BRIDGE\u{7}Sales"),
            "gst_status_type_name_invalid",
        ),
        (
            arguments("20260802", "20260803", " BRIDGE Sales"),
            "gst_status_type_name_invalid",
        ),
        (
            arguments("20260802", "20260803", "BRIDGE Sales "),
            "gst_status_type_name_invalid",
        ),
        // The published schema bounds the name's length before the tool sees it.
        (
            arguments("20260802", "20260803", &long_name),
            "argument_invalid:voucher_type_name",
        ),
        (
            arguments("20260802", "20260803", ""),
            "argument_invalid:voucher_type_name",
        ),
        (
            json!({"company_guid": GUID, "from": "20260802", "to": "20260803"}),
            "voucher_type_name_required",
        ),
        // 94 days: April 1 to July 3.
        (
            arguments("20260401", "20260703", LAB_TYPE),
            "gst_status_window_too_long",
        ),
        (
            arguments("2026-04-01", "2026-07-03", LAB_TYPE),
            "gst_status_window_too_long",
        ),
        (
            arguments("20260803", "20260802", LAB_TYPE),
            "invalid_date_range",
        ),
    ] {
        let (response, received) = run_expecting_no_read(args.clone()).await;
        assert_eq!(refusal_code(&response), expected, "{args}");
        assert_eq!(received, 0, "{args}");
        assert_eq!(receipt(&response)["bytes"], 0, "{args}");
    }
}

#[tokio::test]
async fn a_window_of_ninety_three_days_is_read_and_the_vouchers_outside_it_are_refused() {
    // April 1 to July 2 is 93 days: admitted, so every read is made; the answer holds vouchers
    // dated 2 August, which the window does not cover.
    let call = run(
        plans_with(marks(), august()),
        arguments("20260401", "20260702", LAB_TYPE),
    )
    .await;
    assert_eq!(call.observed.len(), call.expected);
    assert_eq!(refusal_code(&call.response), "window_not_honoured");
    let receipt = receipt(&call.response);
    assert_eq!(receipt["state"], "partial");
    assert_eq!(receipt["reason_code"], "window_not_honoured");
    assert!(receipt["bytes"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn a_voucher_dated_after_the_window_is_refused_not_reported() {
    let call = run(
        plans_with(marks(), august()),
        arguments("20260801", "20260801", LAB_TYPE),
    )
    .await;
    assert_eq!(call.observed.len(), call.expected);
    assert_eq!(refusal_code(&call.response), "window_not_honoured");
    assert!(call.response["structuredContent"]["result"]
        .get("items")
        .is_none());
}

#[tokio::test]
async fn an_answer_holding_a_voucher_of_another_type_is_refused() {
    // The unfiltered answer, as if Tally had not honoured the type in the request: it holds
    // `Z1/0001` of another Sales-class type and four Receipts.
    let call = run(
        plans_with(marks(), decode_utf16(FULL_WINDOW)),
        lab_arguments(),
    )
    .await;
    assert_eq!(call.observed.len(), call.expected);
    assert_eq!(refusal_code(&call.response), "gst_status_type_not_honoured");
    let receipt = receipt(&call.response);
    assert_eq!(receipt["state"], "partial");
    assert_eq!(receipt["reason_code"], "gst_status_type_not_honoured");
    assert!(call.response["structuredContent"]["result"]
        .get("items")
        .is_none());
}

#[tokio::test]
async fn a_book_with_a_voucher_mark_above_the_limit_is_refused_before_the_status_read() {
    let call = run(
        plans_to_the_marks(marks_with_voucher_mark(25_001)),
        lab_arguments(),
    )
    .await;
    // The company list (four requests) and the marks inside their bracket (six): no status read.
    assert_eq!(call.expected, 10);
    assert_eq!(call.observed.len(), 10);
    assert_eq!(refusal_code(&call.response), "gst_status_book_too_large");
    let receipt = receipt(&call.response);
    assert_eq!(receipt["state"], "partial");
    assert_eq!(receipt["reason_code"], "gst_status_book_too_large");
    assert!(receipt["bytes"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn a_book_at_the_limit_is_read() {
    let call = run(
        plans_with(marks_with_voucher_mark(25_000), august()),
        lab_arguments(),
    )
    .await;
    assert_eq!(call.observed.len(), 16);
    assert_eq!(payload(&call.response)["state"], "complete");
}

#[tokio::test]
async fn an_answer_that_repeats_a_voucher_is_refused_whole() {
    let repeated = edit(&august(), TARGET, |block| format!("{block}{block}"));
    let call = run(plans_with(marks(), repeated), lab_arguments()).await;
    assert_eq!(call.observed.len(), call.expected);
    assert_eq!(
        refusal_code(&call.response),
        "gst_status_read_voucher_repeated"
    );
    let receipt = receipt(&call.response);
    assert_eq!(receipt["state"], "partial");
    assert!(receipt["bytes"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn a_voucher_with_a_date_that_is_not_a_date_is_refused_as_unidentified() {
    let mutated = edit(&august(), TARGET, |block| {
        once(
            &block,
            r#"<DATE TYPE="Date">20260802</DATE>"#,
            r#"<DATE TYPE="Date">not a date</DATE>"#,
        )
    });
    let call = run(plans_with(marks(), mutated), lab_arguments()).await;
    assert_eq!(call.observed.len(), call.expected);
    assert_eq!(
        refusal_code(&call.response),
        "gst_status_read_voucher_unidentified"
    );
}

#[tokio::test]
async fn a_voucher_whose_flags_cannot_be_read_makes_the_result_partial_and_is_never_included() {
    let window = august();
    let cases = [
        // One of the three status flags removed.
        (
            edit(&window, TARGET, |block| once(&block, UNCERTAIN, "")),
            UNREADABLE,
        ),
        // A flag repeated.
        (
            edit(&window, TARGET, |block| {
                once(&block, INCLUDED, &format!("{INCLUDED}{INCLUDED}"))
            }),
            UNREADABLE,
        ),
        // A flag spelt another way.
        (
            edit(&window, TARGET, |block| {
                once(&block, INCLUDED, &INCLUDED.replace(">Yes<", ">yes<"))
            }),
            UNREADABLE,
        ),
        // A flag inside another element.
        (
            edit(&window, TARGET, |block| {
                once(&block, EXCLUDED, &format!("<WRAPPER>{EXCLUDED}</WRAPPER>"))
            }),
            UNREADABLE,
        ),
        // All four removed.
        (
            edit(&window, TARGET, |block| {
                [INCLUDED, UNCERTAIN, EXCLUDED, OVERRIDDEN]
                    .iter()
                    .fold(block, |block, element| once(&block, element, ""))
            }),
            NOT_REPORTED,
        ),
    ];
    for (mutated, unread) in cases {
        let call = run(plans_with(marks(), mutated), lab_arguments()).await;
        assert_eq!(call.observed.len(), call.expected);
        let result = payload(&call.response);
        // The voucher is returned, with the code that says why it has no status.
        assert_eq!(result["state"], "partial", "{unread}");
        assert_eq!(result["total"], 32);
        assert_eq!(
            result["counts"],
            json!({
                "included": 24,
                "uncertain": 5,
                "excluded": 0,
                "not_in_return": 0,
                "cancelled": 1,
                "optional": 1,
                "unread": 1,
                "accepted_as_it_stands": 5,
            }),
            "{unread}"
        );
        let items = result["items"].as_array().unwrap();
        assert_eq!(items.len(), 32);
        assert_eq!(items[TARGET_AT]["voucher_number"], TARGET);
        assert_eq!(items[TARGET_AT]["master_id"], "17");
        assert_eq!(
            items[TARGET_AT]["gst_status"],
            json!({"unread": unread}),
            "{unread}"
        );
        // The 31 others are read as in the committed answer.
        for (position, expected) in august_items().iter().enumerate() {
            if position != TARGET_AT {
                assert_eq!(&items[position], expected, "{position}");
            }
        }
        let receipt = receipt(&call.response);
        assert_eq!(receipt["state"], "partial", "{unread}");
        assert_eq!(
            receipt["reason_code"], "gst_status_voucher_unread",
            "{unread}"
        );
    }
}

/// The counts of the two states the lab's answers never held, through the tool: a voucher with
/// only the excluded flag Yes and one with all three No.
#[tokio::test]
async fn excluded_and_not_in_return_vouchers_are_counted_under_their_own_keys() {
    let mutated = edit(&august(), TARGET, |block| {
        let block = once(&block, INCLUDED, &INCLUDED.replace(">Yes<", ">No<"));
        once(&block, EXCLUDED, &EXCLUDED.replace(">No<", ">Yes<"))
    });
    let mutated = edit(&mutated, "BP/26-27/0012", |block| {
        once(&block, INCLUDED, &INCLUDED.replace(">Yes<", ">No<"))
    });
    let call = run(plans_with(marks(), mutated), lab_arguments()).await;
    assert_eq!(call.observed.len(), call.expected);
    let result = payload(&call.response);
    assert_eq!(result["state"], "complete");
    assert_eq!(result["counts"]["included"], 23);
    assert_eq!(result["counts"]["excluded"], 1);
    assert_eq!(result["counts"]["not_in_return"], 1);
    assert_eq!(result["counts"]["uncertain"], 5);
    assert_eq!(result["counts"]["cancelled"], 1);
    assert_eq!(result["counts"]["optional"], 1);
}

/// A voucher whose status reads but whose acceptance flag does not is not a clean read: its
/// status is kept, and the answer is partial with the reason that a voucher was unread.
#[tokio::test]
async fn a_voucher_whose_acceptance_flag_is_missing_keeps_its_status_and_makes_the_result_partial()
{
    let mutated = edit(&august(), TARGET, |block| once(&block, OVERRIDDEN, ""));
    let call = run(plans_with(marks(), mutated), lab_arguments()).await;
    assert_eq!(call.observed.len(), call.expected);
    let result = payload(&call.response);
    assert_eq!(result["state"], "partial");
    assert_eq!(result["counts"]["included"], 25, "the status was read");
    assert_eq!(result["counts"]["unread"], 1);
    assert_eq!(receipt(&call.response)["state"], "partial");
    assert_eq!(
        receipt(&call.response)["reason_code"],
        "gst_status_voucher_unread"
    );
    let rows = result["items"].as_array().unwrap();
    let missing = rows
        .iter()
        .find(|row| row["accepted_as_it_stands"] == json!({"unread": "gst_status_not_reported"}))
        .expect("the voucher with no acceptance flag");
    assert_eq!(missing["gst_status"], "included");
}
