//! The GST status tool (R7) against Tally's own bytes.
//!
//! The answers are the lab's, read on 10 and 12 Oct 2026 from the synthetic company
//! `BRIDGE PILOT LAB` (TallyPrime 7.1 Silver) and committed byte for byte under
//! `agent/gst-status/` (see its PROVENANCE.md). The 12 Oct answer is to the request the tool
//! sends; the 10 Oct answers are to earlier requests, which did not fetch whether a voucher is
//! cancelled, optional or post-dated (the wider one fetched the first two), and read every
//! voucher unread. Nothing here is evidence about a real book. The
//! expected rows below were taken from those bytes with a separate script, not from the code
//! under test; the mutations are string replacements on the decoded text of the committed
//! answer, each asserted to match exactly once, so none can pass by changing nothing.
//!
//! What each part proves:
//! - a voucher Tally marks deleted has no state, the cancelled, optional and post-dated flags
//!   are decided next, then the status and acceptance flags read exactly as measured, and every
//!   other spelling is refused with a code rather than read as a status;
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

/// The status rows of an answer: [`parse_rows_with`] with nothing extra read.
fn parse_status_rows(xml: &str) -> Result<Vec<StatusRow>, &'static str> {
    parse_rows_with(xml, |_| ()).map(|rows| rows.into_iter().map(|(row, ())| row).collect())
}

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
const AUGUST_SHA256: &str = "5cc5d249cc7817b9870a5aa5b0b82727ca56c0c8a59e900dde7808c7012beffa";
/// The request the tool sends, as the lab sent it on 12 Oct 2026.
const REQUEST_SHA256: &str = "68aaa0002f2e8cfa90dfa01016e2d6dcecad97ca8a4c33e4dc77f7c043b9ca59";

const LAB_COMPANY: &str = "BRIDGE PILOT LAB";
const LAB_TYPE: &str = "BRIDGE Sales";

const NOT_REPORTED: &str = "gst_status_not_reported";
const UNREADABLE: &str = "gst_status_unreadable";
const DELETED_VOUCHER: &str = "gst_status_voucher_deleted";

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
        (AUGUST, 111_748, AUGUST_SHA256),
        (AUGUST_REQUEST, 1_846, REQUEST_SHA256),
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
        ((Some("Yes"), Some("No"), Some("No")), FlagStatus::Included),
        ((Some("No"), Some("Yes"), Some("No")), FlagStatus::Uncertain),
        ((Some("No"), Some("No"), Some("Yes")), FlagStatus::Excluded),
        (
            (Some("No"), Some("No"), Some("No")),
            FlagStatus::NotInReturn,
        ),
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
fn cancelled_optional_and_post_dated_are_decided_before_the_three_flags_and_only_from_yes_and_no() {
    let mut kinds = std::collections::BTreeMap::<&str, usize>::new();
    let mut outcomes = std::collections::BTreeMap::<String, usize>::new();
    for is_cancelled in FLAG_VALUES {
        for is_optional in FLAG_VALUES {
            for is_post_dated in FLAG_VALUES {
                for included in FLAG_VALUES {
                    for uncertain in FLAG_VALUES {
                        for excluded in FLAG_VALUES {
                            // A cancelled, optional or post-dated voucher whose three flags state
                            // no measured state is unread, with the flags' own code.
                            let flags = status_of_flags(included, uncertain, excluded);
                            let (kind, expected) = match (is_cancelled, is_optional, is_post_dated)
                            {
                                (Some("Yes"), Some("No"), Some("No")) => {
                                    ("cancelled", flags.map(|_| GstStatus::Cancelled))
                                }
                                (Some("No"), Some("Yes"), Some("No")) => {
                                    ("optional", flags.map(|_| GstStatus::Optional))
                                }
                                (Some("No"), Some("No"), Some("Yes")) => {
                                    ("post_dated", flags.map(|_| GstStatus::PostDated))
                                }
                                (Some("No"), Some("No"), Some("No")) => {
                                    ("by_flags", flags.map(GstStatus::from))
                                }
                                (None, None, None) => ("not_reported", Err(NOT_REPORTED)),
                                _ => ("unreadable", Err(UNREADABLE)),
                            };
                            *kinds.entry(kind).or_default() += 1;
                            *outcomes.entry(format!("{expected:?}")).or_default() += 1;
                            assert_eq!(
                                status_of_voucher(
                                    is_cancelled,
                                    is_optional,
                                    is_post_dated,
                                    included,
                                    uncertain,
                                    excluded
                                ),
                                expected,
                                "{is_cancelled:?} {is_optional:?} {is_post_dated:?} {included:?} {uncertain:?} {excluded:?}"
                            );
                        }
                    }
                }
            }
        }
    }
    // 216 combinations of the three status flags for each of the 216 triples: one triple each is
    // cancelled, optional, post-dated, read by its status flags and not reported; the 211 others
    // are unread.
    assert_eq!(
        kinds.into_iter().collect::<Vec<_>>(),
        [
            ("by_flags", 216),
            ("cancelled", 216),
            ("not_reported", 216),
            ("optional", 216),
            ("post_dated", 216),
            ("unreadable", 211 * 216),
        ]
    );
    // Of the 46,656, a state is read only where the triple is one of four and the flags are one
    // of the four measured combinations: four cancelled, four optional, four post-dated, four by
    // the flags.
    let count = |key: &str| outcomes.get(key).copied().unwrap_or(0);
    assert_eq!(count("Ok(Cancelled)"), 4);
    assert_eq!(count("Ok(Optional)"), 4);
    assert_eq!(count("Ok(PostDated)"), 4);
    assert_eq!(
        count("Ok(Included)")
            + count("Ok(Uncertain)")
            + count("Ok(Excluded)")
            + count("Ok(NotInReturn)"),
        4
    );
    // Not reported: the 216 with all three absent, and the one all-absent triple of status flags
    // of each of the four.
    assert_eq!(count(&format!("Err({NOT_REPORTED:?})")), 220);
    assert_eq!(count(&format!("Err({UNREADABLE:?})")), 46_656 - 16 - 220);
}

#[test]
fn the_measured_cancelled_optional_and_post_dated_vouchers_are_neither_included_nor_not_in_the_return(
) {
    // `BP/26-27/0041`, cancelled, and `BP/26-27/0045`, post-dated, read the included flag Yes;
    // `BP/26-27/0042`, optional, reads all three No.
    let (yes, no) = (Some("Yes"), Some("No"));
    let state = |kind: [Option<&str>; 3], flags: [Option<&str>; 3]| {
        status_of_voucher(kind[0], kind[1], kind[2], flags[0], flags[1], flags[2])
    };
    assert_eq!(
        state([yes, no, no], [yes, no, no]),
        Ok(GstStatus::Cancelled)
    );
    assert_eq!(state([no, yes, no], [no, no, no]), Ok(GstStatus::Optional));
    assert_eq!(
        state([no, no, yes], [yes, no, no]),
        Ok(GstStatus::PostDated)
    );
    // The same flags on a voucher that is none of the three read as the flags say.
    assert_eq!(state([no, no, no], [yes, no, no]), Ok(GstStatus::Included));
    assert_eq!(
        state([no, no, no], [no, no, no]),
        Ok(GstStatus::NotInReturn)
    );
    // A cancelled or post-dated voucher whose flags state no measured state (two Yes) is unread.
    assert_eq!(state([yes, no, no], [yes, yes, no]), Err(UNREADABLE));
    assert_eq!(state([no, no, yes], [yes, yes, no]), Err(UNREADABLE));
    // More than one Yes was never seen, and one of the three absent is never No.
    for kind in [
        [yes, yes, no],
        [yes, no, yes],
        [no, yes, yes],
        [yes, yes, yes],
        [None, no, no],
        [no, None, no],
        [no, no, None],
        [None, None, no],
        [None, None, yes],
        [yes, no, None],
        [no, yes, None],
    ] {
        assert_eq!(state(kind, [yes, no, no]), Err(UNREADABLE), "{kind:?}");
    }
}

#[test]
fn a_deleted_flag_yes_refuses_a_state_and_only_both_absent_or_both_no_pass() {
    for deleted in FLAG_VALUES {
        for retained in FLAG_VALUES {
            // Both absent, or both No, passes; Yes on either of two that are Yes or No refuses
            // the state; one absent beside the other, or any other spelling, is unreadable.
            let yes_or_no = |flag: Option<&str>| matches!(flag, Some("No" | "Yes"));
            let expected = match (deleted, retained) {
                (None, None) | (Some("No"), Some("No")) => Ok(()),
                _ if yes_or_no(deleted) && yes_or_no(retained) => Err(DELETED_VOUCHER),
                _ => Err(UNREADABLE),
            };
            assert_eq!(
                not_deleted(deleted, retained),
                expected,
                "{deleted:?} {retained:?}"
            );
        }
    }
    // Three pass or refuse by a rule; the rest by spelling.
    assert_eq!(not_deleted(None, None), Ok(()));
    assert_eq!(not_deleted(Some("No"), None), Err(UNREADABLE));
    assert_eq!(not_deleted(None, Some("Yes")), Err(UNREADABLE));
    assert_eq!(not_deleted(Some("No"), Some("Yes")), Err(DELETED_VOUCHER));
}

// ---------------------------------------------------------------------------------------------
// The request.
// ---------------------------------------------------------------------------------------------

fn type_name(name: &str) -> read_profiles::GstStatusTypeName {
    read_profiles::GstStatusTypeName::parse(name).expect("an admitted type name")
}

#[test]
fn the_renderer_reproduces_the_request_the_lab_sent_character_for_character() {
    let sent = decode_utf16(AUGUST_REQUEST);
    let sent = sent.strip_prefix('\u{feff}').expect("a byte order mark");
    let rendered = read_profiles::render_gst_status_window(
        LAB_COMPANY,
        &tally_date("20260801"),
        &tally_date("20260831"),
        &type_name(LAB_TYPE),
    )
    .unwrap();
    assert_eq!(rendered, sent);
    // The read built for the tool is the same text.
    let request = read_profiles::gst_status_read(
        LAB_COMPANY,
        &tally_date("20260801"),
        &tally_date("20260831"),
        &type_name(LAB_TYPE),
    )
    .unwrap();
    assert_eq!(request.into_xml(), sent);
}

#[test]
fn the_earlier_request_differs_only_by_the_cancelled_optional_and_post_dated_field_names() {
    let earlier = decode_utf16(EARLIER_REQUEST);
    let earlier = earlier.strip_prefix('\u{feff}').expect("a byte order mark");
    let rendered = read_profiles::render_gst_status_window(
        LAB_COMPANY,
        &tally_date("20260802"),
        &tally_date("20260803"),
        &type_name(LAB_TYPE),
    )
    .unwrap();
    assert_ne!(rendered, earlier);
    assert_eq!(
        rendered,
        once(
            earlier,
            "ISGSTOVERRIDDEN</FETCH>",
            "ISGSTOVERRIDDEN,ISCANCELLED,ISOPTIONAL,ISPOSTDATED</FETCH>"
        )
    );
}

#[test]
fn a_type_name_is_admitted_only_when_it_cannot_break_the_string_literal_it_is_quoted_into() {
    let safe = |name: &str| read_profiles::GstStatusTypeName::parse(name).is_some();
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

/// The renderer takes a parsed name, so an unsafe one never reaches it: the
/// tool's argument parse refuses it first, as `gst_status_type_name_invalid`.
#[test]
fn the_renderer_quotes_the_type_name_and_an_unsafe_one_is_refused_at_the_argument() {
    let (from, to) = (tally_date("20260802"), tally_date("20260803"));
    let rendered = read_profiles::render_gst_status_window(
        LAB_COMPANY,
        &from,
        &to,
        &type_name("BRIDGE Sales"),
    )
    .unwrap();
    assert!(
        rendered.contains("AND $VoucherTypeName = \"BRIDGE Sales\"</SYSTEM>"),
        "{rendered}"
    );
    let code = |name: &str| {
        let args = serde_json::json!({
            "company_guid": "g",
            "from": "2026-08-02",
            "to": "2026-08-03",
            "voucher_type_name": name,
        });
        super::status_arguments(&args)
            .err()
            .map(|failure| failure.code)
    };
    // An empty name is refused earlier, as a missing argument.
    assert_eq!(code(""), Some("voucher_type_name_required".to_string()));
    for refused in [
        "a\"b",
        "a\\b",
        "a\u{7}b",
        " a",
        "a ",
        "R&D <Sales>",
        &"a".repeat(101),
    ] {
        assert_eq!(
            code(refused),
            Some("gst_status_type_name_invalid".to_string()),
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

/// The rows of an answer to an earlier request: each voucher identified and its acceptance read,
/// and no state, since whether it is cancelled, optional or post-dated was not fetched (`code`
/// says whether none of the three was, or only some).
fn earlier_rows(expected: &[Expected], code: &'static str) -> Vec<StatusRow> {
    expected_rows(expected)
        .into_iter()
        .map(|row| StatusRow {
            status: Err(code),
            status_flags: None,
            ..row
        })
        .collect()
}

/// What the three status flags read on the one cancelled, optional and post-dated voucher of the
/// committed answers (`BP/26-27/0041` and `0045` the included flag Yes, `0042` all three No).
fn flags_beside(status: GstStatus) -> Option<FlagStatus> {
    match status {
        GstStatus::Cancelled | GstStatus::PostDated => Some(FlagStatus::Included),
        GstStatus::Optional => Some(FlagStatus::NotInReturn),
        _ => None,
    }
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
                status_flags: flags_beside(*status),
                overridden: Ok(*accepted),
            },
        )
        .collect()
}

/// The vouchers of the 12 Oct answer, in the answer's order (taken from its bytes by a separate
/// script): 25 included (five of them accepted as they stand), five uncertain, `BP/26-27/0041`
/// cancelled, `BP/26-27/0042` optional and `BP/26-27/0045` post-dated.
#[rustfmt::skip]
const AUGUST_ROWS: [Expected; 33] = [
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
    ("BP/26-27/0045", "20260825", "BRIDGE Sales", "45", "69", GstStatus::PostDated, false),
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
fn the_august_answer_reads_the_cancelled_optional_and_post_dated_vouchers_apart_from_the_others() {
    let rows = parse_status_rows(&august()).unwrap();
    assert_eq!(rows, expected_rows(&AUGUST_ROWS));
    let state = |number: &str| {
        let row = rows
            .iter()
            .find(|row| row.voucher_number == number)
            .unwrap();
        (row.status, row.overridden)
    };
    // The cancelled and the post-dated voucher's included flag is Yes and the optional one's three
    // flags are No.
    assert_eq!(
        state("BP/26-27/0041"),
        (Ok(GstStatus::Cancelled), Ok(false))
    );
    assert_eq!(state("BP/26-27/0042"), (Ok(GstStatus::Optional), Ok(false)));
    assert_eq!(
        state("BP/26-27/0045"),
        (Ok(GstStatus::PostDated), Ok(false))
    );
    let count = |status: GstStatus| rows.iter().filter(|row| row.status == Ok(status)).count();
    assert_eq!(
        [
            count(GstStatus::Included),
            count(GstStatus::Uncertain),
            count(GstStatus::Excluded),
            count(GstStatus::NotInReturn),
            count(GstStatus::Cancelled),
            count(GstStatus::Optional),
            count(GstStatus::PostDated),
        ],
        [25, 5, 0, 0, 1, 1, 1]
    );
}

#[test]
fn an_answer_to_the_earlier_request_reads_every_voucher_unread_never_included() {
    // The six vouchers' status flags are all there, but not whether each is cancelled, optional
    // or post-dated: none has a state, and each keeps its identity and its acceptance flag.
    let rows = parse_status_rows(&decode_utf16(SALES_WINDOW)).unwrap();
    assert_eq!(rows, earlier_rows(&SALES_WINDOW_ROWS, NOT_REPORTED));
}

#[test]
fn the_unfiltered_answer_to_the_earlier_request_reads_every_voucher_unread_with_its_type() {
    let rows = parse_status_rows(&decode_utf16(FULL_WINDOW)).unwrap();
    assert_eq!(rows, earlier_rows(&FULL_WINDOW_ROWS, NOT_REPORTED));
    // The four Receipts read all three flags No, which is no longer read as not in the return.
    assert_eq!(
        rows.iter()
            .filter(|row| row.voucher_type == "Receipt" && row.status == Err(NOT_REPORTED))
            .count(),
        4
    );
}

#[test]
fn the_wider_field_list_answer_has_no_post_dated_flag_and_reads_every_voucher_unread() {
    // It carries `&#4;` references, which the parse marks and cleans, and some forty fields a
    // voucher, the cancelled and optional flags (No) among them and the post-dated flag not: a
    // flag absent beside two that are present is never No, so no voucher has a state. Each keeps
    // its identity and its acceptance flag.
    let text = decode_utf16(THREE_VOUCHERS);
    assert!(text.contains("&#4;"));
    assert!(text.contains("<ISCANCELLED") && !text.contains("<ISPOSTDATED"));
    let rows = parse_status_rows(&text).unwrap();
    assert_eq!(rows, earlier_rows(&THREE_VOUCHER_ROWS, UNREADABLE));
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

/// The voucher every mutation below is made on (master id 17, the thirteenth row), and its seven
/// status elements and two deleted flags as Tally wrote them.
const TARGET: &str = "BP/26-27/0013";
const INCLUDED: &str = r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Yes</VCHGSTSTATUSISINCLUDED>"#;
const UNCERTAIN: &str = r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">No</VCHGSTSTATUSISUNCERTAIN>"#;
const EXCLUDED: &str = r#"<VCHGSTSTATUSISEXCLUDED TYPE="Logical">No</VCHGSTSTATUSISEXCLUDED>"#;
const OVERRIDDEN: &str = r#"<ISGSTOVERRIDDEN TYPE="Logical">No</ISGSTOVERRIDDEN>"#;
const CANCELLED: &str = r#"<ISCANCELLED TYPE="Logical">No</ISCANCELLED>"#;
const OPTIONAL: &str = r#"<ISOPTIONAL TYPE="Logical">No</ISOPTIONAL>"#;
const POST_DATED: &str = r#"<ISPOSTDATED TYPE="Logical">No</ISPOSTDATED>"#;
const DELETED: &str = "<ISDELETED>No</ISDELETED>";
const RETAINED: &str = "<ISDELETEDVCHRETAINED>No</ISDELETEDVCHRETAINED>";
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

/// The target voucher's row after `change`, with the other 32 rows asserted to read exactly as
/// in the committed answer.
fn target_after(change: impl FnOnce(String) -> String) -> StatusRow {
    let rows = parse_status_rows(&edit(&august(), TARGET, change)).unwrap();
    assert_eq!(rows.len(), 33);
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
fn a_cancelled_optional_or_post_dated_flag_decides_the_state_and_an_unreadable_one_leaves_none() {
    let yes = |element: &str| element.replace(">No<", ">Yes<");
    let kinds = [
        (CANCELLED, GstStatus::Cancelled),
        (OPTIONAL, GstStatus::Optional),
        (POST_DATED, GstStatus::PostDated),
    ];
    for (kind, state) in kinds {
        // Yes on one flag decides the state over the measured flags beside it; the acceptance is
        // still read.
        let target = target_after(|block| once(&block, kind, &yes(kind)));
        assert_eq!((target.status, target.overridden), (Ok(state), Ok(false)));
        // What the three flags read is kept beside the state: here included, and uncertain when
        // they read so.
        assert_eq!(target.status_flags, Some(FlagStatus::Included), "{kind}");
        let target = target_after(|block| {
            let block = once(&block, kind, &yes(kind));
            let block = once(&block, INCLUDED, &INCLUDED.replace(">Yes<", ">No<"));
            once(&block, UNCERTAIN, &yes(UNCERTAIN))
        });
        assert_eq!(
            (target.status, target.status_flags),
            (Ok(state), Some(FlagStatus::Uncertain)),
            "{kind}"
        );
        // Such a voucher whose three status flags are absent is not reported, and one spelt
        // another way or repeated leaves it unread: the answer around the flag is not read.
        let target = target_after(|block| {
            [INCLUDED, UNCERTAIN, EXCLUDED]
                .iter()
                .fold(once(&block, kind, &yes(kind)), |block, element| {
                    once(&block, element, "")
                })
        });
        assert_eq!(
            (target.status, target.overridden),
            (Err(NOT_REPORTED), Ok(false)),
            "{kind}"
        );
        assert_eq!(target.status_flags, None, "{kind}");
        let target = target_after(|block| {
            once(
                &once(&block, kind, &yes(kind)),
                INCLUDED,
                &INCLUDED.replace(">Yes<", ">yes<"),
            )
        });
        assert_eq!(
            (target.status, target.overridden),
            (Err(UNREADABLE), Ok(false)),
            "{kind}"
        );
        let target = target_after(|block| {
            once(
                &once(&block, kind, &yes(kind)),
                INCLUDED,
                &format!("{INCLUDED}{INCLUDED}"),
            )
        });
        assert_eq!(
            (target.status, target.overridden),
            (Err(UNREADABLE), Err(UNREADABLE)),
            "{kind}"
        );
        // Removed while the other two are present; spelt another way, empty or closed empty: no
        // state, and the acceptance is still read.
        let name = kind[1..kind.find(' ').unwrap()].to_string();
        for changed in [
            String::new(),
            kind.replace(">No<", ">no<"),
            kind.replace(">No<", "><"),
            format!(r#"<{name} TYPE="Logical"/>"#),
        ] {
            let target = target_after(|block| once(&block, kind, &changed));
            assert_eq!(
                (target.status, target.overridden),
                (Err(UNREADABLE), Ok(false)),
                "{changed:?}"
            );
        }
        // Repeated, or inside another element: neither the state nor the acceptance is read.
        for changed in [
            format!("{kind}{kind}"),
            format!("<WRAPPER>{kind}</WRAPPER>"),
            format!("{kind}<WRAPPER>{}</WRAPPER>", yes(kind)),
        ] {
            let target = target_after(|block| once(&block, kind, &changed));
            assert_eq!(
                (target.status, target.overridden),
                (Err(UNREADABLE), Err(UNREADABLE)),
                "{changed}"
            );
        }
    }
    // Two Yes, or all three: no state, and the acceptance is still read.
    for set in [
        &[CANCELLED, OPTIONAL][..],
        &[CANCELLED, POST_DATED],
        &[OPTIONAL, POST_DATED],
        &[CANCELLED, OPTIONAL, POST_DATED],
    ] {
        let several = target_after(|block| {
            set.iter()
                .fold(block, |block, kind| once(&block, kind, &yes(kind)))
        });
        assert_eq!(
            (several.status, several.overridden),
            (Err(UNREADABLE), Ok(false)),
            "{set:?}"
        );
    }
    // All three removed: not reported. Two removed beside the third: unreadable.
    let none = target_after(|block| {
        [CANCELLED, OPTIONAL, POST_DATED]
            .iter()
            .fold(block, |block, kind| once(&block, kind, ""))
    });
    assert_eq!(
        (none.status, none.overridden),
        (Err(NOT_REPORTED), Ok(false))
    );
    let two = target_after(|block| once(&once(&block, CANCELLED, ""), OPTIONAL, ""));
    assert_eq!((two.status, two.overridden), (Err(UNREADABLE), Ok(false)));
}

#[test]
fn a_voucher_tally_marks_deleted_has_no_state_and_the_two_flags_absent_are_allowed() {
    let yes = |element: &str| element.replace(">No<", ">Yes<");
    for flag in [DELETED, RETAINED] {
        // Yes was never measured: the voucher keeps its row, with no state and no acceptance.
        let target = target_after(|block| once(&block, flag, &yes(flag)));
        assert_eq!(
            (target.status, target.overridden),
            (Err(DELETED_VOUCHER), Err(DELETED_VOUCHER)),
            "{flag}"
        );
        // One of the two absent beside the other is never No.
        let target = target_after(|block| once(&block, flag, ""));
        assert_eq!(
            (target.status, target.overridden),
            (Err(UNREADABLE), Ok(false)),
            "{flag}"
        );
        // Spelt another way, empty or closed empty: no state, the acceptance still read.
        let name = &flag[1..flag.find('>').unwrap()];
        for changed in [
            flag.replace(">No<", ">no<"),
            flag.replace(">No<", "><"),
            format!("<{name}/>"),
        ] {
            let target = target_after(|block| once(&block, flag, &changed));
            assert_eq!(
                (target.status, target.overridden),
                (Err(UNREADABLE), Ok(false)),
                "{changed}"
            );
        }
        // Repeated, or inside another element: neither the state nor the acceptance is read.
        for changed in [
            format!("{flag}{flag}"),
            format!("<WRAPPER>{flag}</WRAPPER>"),
            format!("{flag}<WRAPPER>{}</WRAPPER>", yes(flag)),
        ] {
            let target = target_after(|block| once(&block, flag, &changed));
            assert_eq!(
                (target.status, target.overridden),
                (Err(UNREADABLE), Err(UNREADABLE)),
                "{changed}"
            );
        }
    }
    // The acceptance flag of a deleted voucher is not read: it is not counted as accepted.
    let accepted = target_after(|block| {
        once(
            &once(&block, DELETED, &yes(DELETED)),
            OVERRIDDEN,
            &yes(OVERRIDDEN),
        )
    });
    assert_eq!(
        (accepted.status, accepted.overridden),
        (Err(DELETED_VOUCHER), Err(DELETED_VOUCHER))
    );
    // Both removed, as when Tally does not volunteer them: read as before. Both Yes: no state.
    let neither = target_after(|block| once(&once(&block, DELETED, ""), RETAINED, ""));
    assert_eq!(neither.status, Ok(GstStatus::Included));
    let both = target_after(|block| {
        once(
            &once(&block, DELETED, &yes(DELETED)),
            RETAINED,
            &yes(RETAINED),
        )
    });
    assert_eq!(both.status, Err(DELETED_VOUCHER));
    // A deleted voucher that also reads cancelled, optional or post-dated is not given that state,
    // nor the flags beside it; and the deleted code is the one returned when another flag of the
    // voucher is unreadable too.
    for kind in [CANCELLED, OPTIONAL, POST_DATED] {
        let target =
            target_after(|block| once(&once(&block, DELETED, &yes(DELETED)), kind, &yes(kind)));
        assert_eq!(
            (target.status, target.status_flags),
            (Err(DELETED_VOUCHER), None),
            "{kind}"
        );
        let target = target_after(|block| {
            once(
                &once(&block, DELETED, &yes(DELETED)),
                kind,
                &kind.replace(">No<", ">no<"),
            )
        });
        assert_eq!(target.status, Err(DELETED_VOUCHER), "{kind}");
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
        posting: Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
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
        GstStatus::PostDated => "post_dated",
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
    let mut item = json!({
        "voucher_number": expected.0,
        "date": expected.1,
        "voucher_type": expected.2,
        "master_id": expected.3,
        "alter_id": expected.4,
        "gst_status": status_name(expected.5),
        "accepted_as_it_stands": expected.6,
    });
    if let Some(flags) = flags_beside(expected.5) {
        item["status_flags"] = json!(status_name(flags.into()));
    }
    item
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
    assert_eq!(result["total"], 33);
    assert_eq!(result["offset"], 0);
    // The cancelled, the optional and the post-dated voucher are counted under their own keys only.
    assert_eq!(
        result["counts"],
        json!({
            "included": 25,
            "uncertain": 5,
            "excluded": 0,
            "not_in_return": 0,
            "cancelled": 1,
            "optional": 1,
            "post_dated": 1,
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
            "BP/26-27/0044",
            "BP/26-27/0045"
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
        "never counted as included, uncertain, excluded or not_in_return",
        "it is not counted: it does not say that the return lists the voucher",
        "where no Edit Log was seen to be on",
        "post_dated means Tally's ISPOSTDATED reads Yes",
        "is before the voucher's date or once it has reached it, is not measured",
        "gst_status_voucher_deleted",
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
        (30, 30..33, None),
    ] {
        let mut args = lab_arguments();
        args["limit"] = json!(10);
        args["offset"] = json!(offset);
        let call = run(plans_with(marks(), august()), args).await;
        assert_eq!(call.observed.len(), call.expected);
        let result = payload(&call.response);
        assert_eq!(result["state"], "complete");
        assert_eq!(result["total"], 33, "offset {offset}");
        assert_eq!(result["offset"], offset);
        assert_eq!(result["counts"]["included"], 25);
        assert_eq!(result["counts"]["cancelled"], 1);
        assert_eq!(result["counts"]["post_dated"], 1);
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

/// The count keys of a result in which every voucher is unread.
fn nothing_counted(unread: u64, accepted: u64) -> Value {
    json!({
        "included": 0,
        "uncertain": 0,
        "excluded": 0,
        "not_in_return": 0,
        "cancelled": 0,
        "optional": 0,
        "post_dated": 0,
        "unread": unread,
        "accepted_as_it_stands": accepted,
    })
}

#[tokio::test]
async fn an_answer_to_an_earlier_request_through_the_tool_is_partial_and_counts_nothing_included() {
    // The narrow answer fetched none of the cancelled, optional and post-dated flags; the wider
    // one fetched the first two.
    for (answer, from, to, total, code) in [
        (SALES_WINDOW, "20260802", "20260803", 6, NOT_REPORTED),
        (THREE_VOUCHERS, "20260801", "20260802", 3, UNREADABLE),
    ] {
        let call = run(
            plans_with(marks(), decode_utf16(answer)),
            arguments(from, to, LAB_TYPE),
        )
        .await;
        assert_eq!(call.observed.len(), call.expected);
        let result = payload(&call.response);
        assert_eq!(result["state"], "partial", "{code}");
        assert_eq!(result["total"], total);
        assert_eq!(result["counts"], nothing_counted(total, 1), "{code}");
        for item in result["items"].as_array().unwrap() {
            assert_eq!(item["gst_status"], json!({"unread": code}), "{item}");
        }
        let receipt = receipt(&call.response);
        assert_eq!(receipt["state"], "partial");
        assert_eq!(receipt["reason_code"], "gst_status_voucher_unread");
    }
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
        // The post-dated flag removed beside the cancelled and optional flags.
        (
            edit(&window, TARGET, |block| once(&block, POST_DATED, "")),
            UNREADABLE,
        ),
        // A deleted flag Yes.
        (
            edit(&window, TARGET, |block| {
                once(&block, DELETED, &DELETED.replace(">No<", ">Yes<"))
            }),
            DELETED_VOUCHER,
        ),
    ];
    for (mutated, unread) in cases {
        let call = run(plans_with(marks(), mutated), lab_arguments()).await;
        assert_eq!(call.observed.len(), call.expected);
        let result = payload(&call.response);
        // The voucher is returned, with the code that says why it has no status.
        assert_eq!(result["state"], "partial", "{unread}");
        assert_eq!(result["total"], 33);
        assert_eq!(
            result["counts"],
            json!({
                "included": 24,
                "uncertain": 5,
                "excluded": 0,
                "not_in_return": 0,
                "cancelled": 1,
                "optional": 1,
                "post_dated": 1,
                "unread": 1,
                "accepted_as_it_stands": 5,
            }),
            "{unread}"
        );
        let items = result["items"].as_array().unwrap();
        assert_eq!(items.len(), 33);
        assert_eq!(items[TARGET_AT]["voucher_number"], TARGET);
        assert_eq!(items[TARGET_AT]["master_id"], "17");
        assert_eq!(
            items[TARGET_AT]["gst_status"],
            json!({"unread": unread}),
            "{unread}"
        );
        // The 32 others are read as in the committed answer.
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
    assert_eq!(result["counts"]["post_dated"], 1);
}

/// The acceptance flag is read on a post-dated voucher as on any other, and its item says what the
/// three status flags read without counting it. A second post-dated voucher makes the count two,
/// apart from the one optional and the one cancelled voucher.
#[tokio::test]
async fn a_post_dated_voucher_accepted_as_it_stands_is_counted_as_accepted_and_not_as_included() {
    let mutated = edit(&august(), "BP/26-27/0045", |block| {
        once(&block, OVERRIDDEN, &OVERRIDDEN.replace(">No<", ">Yes<"))
    });
    let mutated = edit(&mutated, TARGET, |block| {
        once(&block, POST_DATED, &POST_DATED.replace(">No<", ">Yes<"))
    });
    let call = run(plans_with(marks(), mutated), lab_arguments()).await;
    assert_eq!(call.observed.len(), call.expected);
    let result = payload(&call.response);
    assert_eq!(result["state"], "complete");
    assert_eq!(result["counts"]["included"], 24);
    assert_eq!(result["counts"]["post_dated"], 2);
    assert_eq!(result["counts"]["optional"], 1);
    assert_eq!(result["counts"]["cancelled"], 1);
    assert_eq!(result["counts"]["accepted_as_it_stands"], 6);
    assert_eq!(result["items"][TARGET_AT]["gst_status"], "post_dated");
    assert_eq!(result["items"][TARGET_AT]["status_flags"], "included");
    assert_eq!(
        result["items"][32],
        json!({
            "voucher_number": "BP/26-27/0045",
            "date": "20260825",
            "voucher_type": "BRIDGE Sales",
            "master_id": "45",
            "alter_id": "69",
            "gst_status": "post_dated",
            "status_flags": "included",
            "accepted_as_it_stands": true,
        })
    );
    // A voucher that is none of the three carries no such key.
    assert!(result["items"][0].get("status_flags").is_none());
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
