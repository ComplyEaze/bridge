//! The uncertain-voucher tool against Tally's own bytes.
//!
//! The answers are the lab's, read on 13 Oct 2026 from the synthetic company `BRIDGE PILOT LAB`
//! (TallyPrime 7.1 Silver) and committed byte for byte under `agent/gst-status/` (see its
//! PROVENANCE.md): the company's marks, the narrow status read that counts the window, the wide
//! read before one Accept As Is and the wide read after it, and the marks after it. The lab made
//! them in this order: marks, count, wide before, the accept, wide after, marks (PROVENANCE.md has
//! the log). A call on the before answer is served the first marks; a call on the after answer is
//! served the marks after the accept, with the count read made before it, because none was made
//! after (nothing in the tool compares a mark with a voucher, or takes a status from the count); the company list the call reads first
//! is the lab's answer of 9 Oct 2026 (`pilot-lab/`). The health checks after each read are the
//! simulator's own page, not a capture. The expected rows below were taken from those
//! bytes with a separate script (an XML tree, one direct child at a time), not from the code
//! under test. Every failure-path control is derived in the test from those bytes, by
//! deleting, repeating or emptying one element of one voucher, each edit asserted to match
//! exactly once so none can pass by changing nothing; none is a capture of Tally. Nothing
//! here is evidence about a real book.
use super::*;
use bridge_tally_transport::TallyEndpointConfig;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const BEFORE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-uncertain-august-before.utf16le.xml"
);
const AFTER: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-uncertain-august-after.utf16le.xml"
);
const MARKS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-uncertain-august-marks.utf16le.xml"
);
const MARKS_AFTER: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-uncertain-august-marks-after.utf16le.xml"
);
const COUNT: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-uncertain-august-count.utf16le.xml"
);
const WIDE_REQUEST: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-uncertain-august.request.utf16le.xml"
);
const NARROW_REQUEST: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-august.request.utf16le.xml"
);

const BEFORE_SHA256: &str = "015ddda734957af2a50e0e6222c19ce9df5e1fa1607ac3567a573153362afc2a";
const AFTER_SHA256: &str = "e73610f1e946673240dbe3c86b531addf84db6b4092199bd7cac54b620513dce";
const MARKS_SHA256: &str = "8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6";
const MARKS_AFTER_SHA256: &str = "4464435c71e3e9e4d358317ad730f1bd536df3f85b5a3b85a68074059d69e5d5";
/// The lab's marks request (`r02-marks.xml`): the read the call's gate sends, as the lab was sent it.
const MARKS_REQUEST_SHA256: &str =
    "81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c";
const COUNT_SHA256: &str = "6b6b5e0af19d2bc6df7cfaf03a954aedc7b515b9870106e967d95e6243ee3a34";
const WIDE_REQUEST_SHA256: &str =
    "cb037151f19053ae5810877f029b52d6a37a96e902245e424b4037bd66b9ab20";
const NARROW_REQUEST_SHA256: &str =
    "68aaa0002f2e8cfa90dfa01016e2d6dcecad97ca8a4c33e4dc77f7c043b9ca59";

const LAB_COMPANY: &str = "BRIDGE PILOT LAB";
const LAB_TYPE: &str = "BRIDGE Sales";
const LAB_PARTY: &str = "BRIDGE Walk-in";

/// The pre-filing line for five vouchers of the type `BRIDGE Sales` dated the whole of August 2026:
/// the wording that says what is read (the type's uncertain flags), the product's full name, and the
/// voucher type named in place of "Sales" (the type's class is not read).
const LINE_FOR_FIVE: &str = "Tally marks 5 vouchers of the type BRIDGE Sales dated 2026-08-01 to 2026-08-31 as Uncertain on its GSTR-1 status flags. Amounts are from ComplyEaze Bridge's voucher read; Tally's JSON export and the portal are not checked. Resolve each in Tally before exporting; do not add them on the portal by hand.";

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

/// One voucher as the lab's script read it: date, number, master id, alter id, party, amount
/// without its sign (empty where the answer's was), the state the flags state, whether the
/// acceptance flag reads Yes.
type Expected = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    bool,
);

/// The lab script's rows, one voucher a line, the fields of `Expected` separated by `|`.
const BEFORE_TABLE: &str = "\
20260801|BP/26-27/0001|1|2|BRIDGE Walk-in|1050.00|included|false
20260801|BP/26-27/0002|2|11|BRIDGE Walk-in|1051.05|uncertain|false
20260801|BP/26-27/0003|3|50|BRIDGE Walk-in|1051.04|uncertain|false
20260801|BP/26-27/0004|4|13|BRIDGE Walk-in|1051.05|included|false
20260801|BP/26-27/0005|5|54|BRIDGE Walk-in|1051.05|included|true
20260801|BP/26-27/0006|6|48|BRIDGE Walk-in|1050.02|included|true
20260801|BP/26-27/0007|7|8|BRIDGE Walk-in|1050.00|included|false
20260801|BP/26-27/0008|8|9|BRIDGE Walk-in|1051.05|uncertain|false
20260801|BP/26-27/0009|9|10|BRIDGE Walk-in|1456.00|included|false
20260801|BP/26-27/0011|10|16|BRIDGE Walk-in|1051.06|included|false
20260801|BP/26-27/0010|11|17|BRIDGE Walk-in|1051.00|included|false
20260802|BP/26-27/0012|16|23|BRIDGE Walk-in|210.20|included|false
20260802|BP/26-27/0013|17|24|BRIDGE Walk-in|210.44|included|false
20260802|BP/26-27/0014|18|25|BRIDGE Walk-in|211.28|included|false
20260802|BP/26-27/0015|19|26|BRIDGE Walk-in|210.44|included|false
20260802|BP/26-27/0016|20|28|BRIDGE Walk-in|210.42|included|true
20260802|BP/26-27/0019|22|30|BRIDGE Walk-in|105.00|included|false
20260805|BP/26-27/0021|26|35|BRIDGE Walk-in|315.30|included|false
20260807|5|27|36|BRIDGE Walk-in|105.00|uncertain|false
20260807|6|28|37|BRIDGE Walk-in|105.00|uncertain|false
20260807|7|29|38|BRIDGE Walk-in|105.00|included|false
20260807|8|30|39|BRIDGE Walk-in|105.00|included|false
20260807|9|31|40|BRIDGE Walk-in|105.00|included|false
20260807|10|32|41|BRIDGE Walk-in|105.00|included|false
20260807|11|33|42|BRIDGE Walk-in|105.00|included|false
20260807|12|34|43|BRIDGE Walk-in|105.00|included|false
20260807|13|35|44|BRIDGE Walk-in|105.00|included|false
20260807|14|36|45|BRIDGE Walk-in|105.00|included|false
20260820|BP/26-27/0041|40|63|||cancelled|false
20260820|BP/26-27/0042|41|62|BRIDGE Walk-in|1050.00|optional|false
20260822|BP/26-27/0043|43|67|BRIDGE Walk-in|1051.05|included|true
20260822|BP/26-27/0044|44|68|BRIDGE Walk-in|1051.05|included|true
";

fn before_rows() -> [Expected; 32] {
    let rows: Vec<Expected> = BEFORE_TABLE
        .lines()
        .map(|line| {
            let field: Vec<&'static str> = line.split('|').collect();
            assert_eq!(field.len(), 8, "{line}");
            let accepted = match field[7] {
                "true" => true,
                "false" => false,
                other => panic!("{other}"),
            };
            (
                field[0], field[1], field[2], field[3], field[4], field[5], field[6], accepted,
            )
        })
        .collect();
    rows.try_into().expect("32 rows")
}

/// The answer after one Accept As Is: the lab's script found one voucher different, the third.
fn after_rows() -> [Expected; 32] {
    let mut rows = before_rows();
    // The accept gave the third voucher a new alter id, a state of included and the flag Yes.
    rows[2].3 = "69";
    rows[2].6 = "included";
    rows[2].7 = true;
    rows
}

fn wide_rows(xml: &str) -> Vec<(StatusRow, WideFields)> {
    super::super::gst_status::parse_rows_with(xml, wide_fields).unwrap()
}

fn before() -> String {
    decode_utf16(BEFORE)
}

fn after() -> String {
    decode_utf16(AFTER)
}

// ---------------------------------------------------------------------------------------------
// The committed bytes are the lab's, and the request is the lab's.
// ---------------------------------------------------------------------------------------------

#[test]
fn the_committed_files_are_the_lab_bytes() {
    for (bytes, length, sha256) in [
        (BEFORE, 242_632, BEFORE_SHA256),
        (AFTER, 242_636, AFTER_SHA256),
        (MARKS, 3_394, MARKS_SHA256),
        (MARKS_AFTER, 3_396, MARKS_AFTER_SHA256),
        (COUNT, 108_442, COUNT_SHA256),
        (WIDE_REQUEST, 1_892, WIDE_REQUEST_SHA256),
        (NARROW_REQUEST, 1_846, NARROW_REQUEST_SHA256),
    ] {
        assert_eq!(bytes.len(), length);
        assert_eq!(sha256_hex(bytes), sha256);
    }
    for answer in [BEFORE, AFTER, COUNT, MARKS, MARKS_AFTER] {
        assert_eq!(
            &answer[..2],
            b"<\0",
            "Tally's answers carry no byte order mark"
        );
    }
    for request in [WIDE_REQUEST, NARROW_REQUEST] {
        assert_eq!(
            &request[..2],
            &[0xFF, 0xFE],
            "the lab's requests begin with one"
        );
    }
}

#[test]
fn the_renderer_reproduces_the_wide_request_the_lab_sent_character_for_character() {
    let sent = decode_utf16(WIDE_REQUEST);
    let sent = sent.strip_prefix('\u{feff}').expect("a byte order mark");
    let (from, to) = (tally_date("20260801"), tally_date("20260831"));
    let lab_type = read_profiles::GstStatusTypeName::parse(LAB_TYPE).unwrap();
    let rendered =
        read_profiles::render_gst_uncertain_window(LAB_COMPANY, &from, &to, &lab_type).unwrap();
    assert_eq!(rendered, sent);
    // The read built for the tool is the same text, and it goes out as the lab's bytes.
    let request = read_profiles::gst_uncertain_read(LAB_COMPANY, &from, &to, &lab_type).unwrap();
    assert_eq!(request.into_xml(), sent);
    assert_eq!(
        sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
            &rendered
        )),
        WIDE_REQUEST_SHA256
    );
}

#[test]
fn the_wide_request_is_the_narrow_one_with_the_party_and_the_amount_added() {
    let narrow = decode_utf16(NARROW_REQUEST);
    let wide = decode_utf16(WIDE_REQUEST);
    let tail = ",ISCANCELLED,ISOPTIONAL,ISPOSTDATED</FETCH>";
    assert_eq!(narrow.matches(tail).count(), 1);
    assert_eq!(
        narrow.replace(
            tail,
            ",PARTYLEDGERNAME,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,AMOUNT</FETCH>"
        ),
        wide
    );
}

/// The wide renderer takes the same parsed name, so a name that could break
/// the string literal cannot reach it either.
#[test]
fn a_type_name_that_could_break_the_string_literal_cannot_reach_the_wide_renderer() {
    for name in ["BRIDGE \"Sales", "BRIDGE\\Sales", "a&b", " Sales", ""] {
        assert_eq!(
            read_profiles::GstStatusTypeName::parse(name),
            None,
            "{name:?}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// The parse: the party and the amount of each voucher, read from direct children only.
// ---------------------------------------------------------------------------------------------

fn state_name(status: GstStatus) -> &'static str {
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

fn check_rows(xml: &str, expected: &[Expected; 32]) {
    let rows = wide_rows(xml);
    assert_eq!(rows.len(), 32);
    for ((row, fields), expected) in rows.iter().zip(expected) {
        let (date, number, master, alter, party, amount, state, accepted) = *expected;
        assert_eq!(
            (
                row.date.as_str(),
                row.voucher_number.as_str(),
                row.master_id.as_str(),
                row.alter_id.as_str()
            ),
            (date, number, master, alter)
        );
        assert_eq!(row.voucher_type, LAB_TYPE);
        assert_eq!(row.status.map(state_name), Ok(state), "{number}");
        assert_eq!(row.overridden, Ok(accepted), "{number}");
        if party.is_empty() {
            assert_eq!(fields.party, Err(PARTY_EMPTY), "{number}");
        } else {
            assert_eq!(fields.party.as_deref(), Ok(party), "{number}");
        }
        if amount.is_empty() {
            assert_eq!(fields.amount, Err(AMOUNT_UNREADABLE), "{number}");
        } else {
            // Every amount of the book is negative: the party is debited, and the sign is not
            // part of the amount.
            let read = fields
                .amount
                .as_ref()
                .unwrap_or_else(|code| panic!("{number}: {code}"));
            assert_eq!(read.magnitude.as_str(), amount, "{number}");
            assert_eq!(read.side, PartySide::Debit, "{number}");
        }
    }
}

#[test]
fn the_answer_before_the_accept_reads_every_voucher_with_its_party_and_amount() {
    check_rows(&before(), &before_rows());
}

#[test]
fn the_answer_after_the_accept_reads_the_same_but_for_the_one_voucher() {
    check_rows(&after(), &after_rows());
    let (earlier, later) = (wide_rows(&before()), wide_rows(&after()));
    let changed: Vec<_> = earlier
        .iter()
        .zip(&later)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| {
            (
                a.0.voucher_number.clone(),
                a.0.alter_id.clone(),
                b.0.alter_id.clone(),
            )
        })
        .collect();
    assert_eq!(
        changed,
        [(
            "BP/26-27/0003".to_string(),
            "50".to_string(),
            "69".to_string()
        )]
    );
    // Its amount did not change, and its party did not.
    assert_eq!(earlier[2].1, later[2].1);
}

#[test]
fn the_cancelled_voucher_reads_no_party_and_no_amount_and_the_optional_one_reads_both() {
    let rows = wide_rows(&before());
    let cancelled = &rows[28];
    assert_eq!(cancelled.0.voucher_number, "BP/26-27/0041");
    assert_eq!(cancelled.0.status, Ok(GstStatus::Cancelled));
    assert_eq!(cancelled.1.party, Err(PARTY_EMPTY));
    assert_eq!(cancelled.1.amount, Err(AMOUNT_UNREADABLE));
    let optional = &rows[29];
    assert_eq!(optional.0.voucher_number, "BP/26-27/0042");
    assert_eq!(optional.0.status, Ok(GstStatus::Optional));
    assert_eq!(optional.1.party.as_deref(), Ok(LAB_PARTY));
    assert_eq!(
        optional.1.amount.as_ref().unwrap().magnitude.as_str(),
        "1050.00"
    );
}

#[test]
fn an_amount_is_a_nonzero_plain_decimal_and_nothing_else() {
    for (text, amount, side) in [
        ("-1051.04", "1051.04", PartySide::Debit),
        ("1051.04", "1051.04", PartySide::Credit),
        ("-1050.00", "1050.00", PartySide::Debit),
        ("-0.01", "0.01", PartySide::Debit),
        ("-210", "210", PartySide::Debit),
    ] {
        let read = amount_of(Some(Some(text.to_string()))).unwrap();
        assert_eq!(
            (read.magnitude.as_str(), read.side),
            (amount, side),
            "{text}"
        );
    }
    // Empty, zero, thousands separators, a unit, a foreign-currency form: unread, never zero.
    for text in [
        "",
        "0",
        "0.00",
        "-0.00",
        "1,051.04",
        "-1051.04 Dr",
        "Rs. 1051.04",
        "-$ 12.50 @ Rs. 83 = Rs. -1037.50",
        "+1051.04",
        "1051.",
        ".5",
        "1e3",
        "NaN",
    ] {
        assert_eq!(
            amount_of(Some(Some(text.to_string()))),
            Err(AMOUNT_UNREADABLE),
            "{text:?}"
        );
    }
    // Repeated is unread, and absent is its own code.
    assert_eq!(amount_of(None), Err(AMOUNT_UNREADABLE));
    assert_eq!(amount_of(Some(None)), Err(AMOUNT_NOT_REPORTED));
}

#[test]
fn a_party_is_one_nonempty_name_and_an_empty_one_is_not_a_name() {
    assert_eq!(
        party_of(Some(Some(LAB_PARTY.to_string()))),
        Ok(LAB_PARTY.to_string())
    );
    assert_eq!(party_of(Some(Some(String::new()))), Err(PARTY_EMPTY));
    assert_eq!(party_of(Some(None)), Err(PARTY_NOT_REPORTED));
    assert_eq!(party_of(None), Err(PARTY_UNREADABLE));
}

const TARGET: &str = "BP/26-27/0003";
const TARGET_AT: usize = 2;
const DIRECT_AMOUNT: &str = r#"<AMOUNT TYPE="Amount">-1051.04</AMOUNT>"#;
const PARTY_LINE: &str = r#"<PARTYLEDGERNAME TYPE="String">BRIDGE Walk-in</PARTYLEDGERNAME>"#;

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

/// A voucher's block with its first occurrence of `from` (asserted to come before the ledger
/// entries, so it is the voucher's own child and not an entry's) replaced.
fn replace_direct(block: &str, from: &str, to: &str) -> String {
    let first = block.find(from).expect("the element is there");
    let entries = block.find("<LEDGERENTRIES.LIST>").expect("ledger entries");
    assert!(first < entries, "{from} comes before the ledger entries");
    format!("{}{}{}", &block[..first], to, &block[first + from.len()..])
}

/// The target voucher's wide row after `change`, with the 31 others asserted unchanged.
fn target_after(change: impl FnOnce(String) -> String) -> (StatusRow, WideFields) {
    let rows = wide_rows(&edit(&before(), TARGET, change));
    let control = wide_rows(&before());
    assert_eq!(rows.len(), 32);
    let mut target = None;
    for (index, (row, control)) in rows.into_iter().zip(control).enumerate() {
        if index == TARGET_AT {
            assert_eq!(row.0.voucher_number, TARGET);
            target = Some(row);
        } else {
            assert_eq!(row, control, "voucher {index}");
        }
    }
    target.expect("the target keeps its row")
}

#[test]
fn the_unmutated_target_reads_its_party_and_amount_so_each_edit_below_is_the_cause() {
    let (row, fields) = target_after(|block| block);
    assert_eq!(row.status, Ok(GstStatus::Uncertain));
    assert_eq!(fields.party.as_deref(), Ok(LAB_PARTY));
    assert_eq!(fields.amount.unwrap().magnitude.as_str(), "1051.04");
}

#[test]
fn two_direct_amounts_an_empty_one_or_one_that_is_not_a_decimal_are_unread() {
    // The second direct AMOUNT, put after the first, names another figure.
    let (row, fields) = target_after(|block| {
        replace_direct(
            &block,
            DIRECT_AMOUNT,
            &format!("{DIRECT_AMOUNT}<AMOUNT TYPE=\"Amount\">-7.00</AMOUNT>"),
        )
    });
    assert_eq!(fields.amount, Err(AMOUNT_UNREADABLE));
    // The status of the voucher is not touched by an amount that cannot be read.
    assert_eq!(row.status, Ok(GstStatus::Uncertain));
    for (text, code) in [
        ("", AMOUNT_UNREADABLE),
        ("0.00", AMOUNT_UNREADABLE),
        ("-$ 12.50 @ Rs. 83 = Rs. -1037.50", AMOUNT_UNREADABLE),
        ("1,051.04", AMOUNT_UNREADABLE),
    ] {
        let (_, fields) = target_after(|block| {
            replace_direct(
                &block,
                DIRECT_AMOUNT,
                &format!("<AMOUNT TYPE=\"Amount\">{text}</AMOUNT>"),
            )
        });
        assert_eq!(fields.amount, Err(code), "{text:?}");
    }
}

#[test]
fn an_amount_that_sits_only_under_the_ledger_entries_is_not_the_vouchers_own() {
    // The voucher's own AMOUNT removed: its entries still hold the same figure twice or more.
    let (row, fields) = target_after(|block| replace_direct(&block, DIRECT_AMOUNT, ""));
    assert_eq!(fields.amount, Err(AMOUNT_NOT_REPORTED));
    assert_eq!(row.status, Ok(GstStatus::Uncertain));
    // The same, with the voucher's own element closed empty.
    let (_, fields) =
        target_after(|block| replace_direct(&block, DIRECT_AMOUNT, r#"<AMOUNT TYPE="Amount"/>"#));
    assert_eq!(fields.amount, Err(AMOUNT_UNREADABLE));
}

#[test]
fn the_party_is_unread_when_it_is_empty_absent_or_repeated() {
    let (_, fields) = target_after(|block| {
        replace_direct(
            &block,
            PARTY_LINE,
            r#"<PARTYLEDGERNAME TYPE="String"></PARTYLEDGERNAME>"#,
        )
    });
    assert_eq!(fields.party, Err(PARTY_EMPTY));
    let (_, fields) = target_after(|block| replace_direct(&block, PARTY_LINE, ""));
    assert_eq!(fields.party, Err(PARTY_NOT_REPORTED));
    let (_, fields) = target_after(|block| {
        replace_direct(
            &block,
            PARTY_LINE,
            &format!("{PARTY_LINE}<PARTYLEDGERNAME TYPE=\"String\">Another</PARTYLEDGERNAME>"),
        )
    });
    assert_eq!(fields.party, Err(PARTY_UNREADABLE));
}

// ---------------------------------------------------------------------------------------------
// The tool, against the simulator replaying the reads of a whole call.
// ---------------------------------------------------------------------------------------------

/// The lab company's GUID, as its marks answer and its company list hold it.
const GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

/// The lab's company-list request (`r01-company-extent.xml`), the first read of every call.
const COMPANY_LIST_REQUEST_SHA256: &str =
    "9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8";

/// The company list the call reads first: the lab's own answer of 9 Oct 2026 (the pilot-lab
/// fixtures) to that request, one company, `BRIDGE PILOT LAB`. It is four days older than the
/// answers of 13 Oct; the lab did not read the list again in the 13 Oct run.
fn companies() -> String {
    decode_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-company-extent.utf16le.xml"
    ))
}

/// The lab's marks answer (voucher mark 68), the bytes as received.
fn marks() -> String {
    let text = decode_utf16(MARKS);
    assert_eq!(text.matches(GUID).count(), 1);
    text
}

fn marks_with_voucher_mark(vouchers: u64) -> String {
    once(
        &marks(),
        r#"<ALTVCHID TYPE="Number"> 68</ALTVCHID>"#,
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

/// The reads up to and including the company marks: ten requests.
fn plans_to_the_marks(marks: String) -> Vec<ScenarioPlan> {
    let mut plans = Vec::new();
    pair(&mut plans, &plan(companies()));
    bracketed(&mut plans, &plan(marks));
    plans
}

/// Those ten and the count read inside its bracket: sixteen requests.
fn plans_to_the_count(marks: String, count: String) -> Vec<ScenarioPlan> {
    let mut plans = plans_to_the_marks(marks);
    bracketed(&mut plans, &plan(count));
    plans
}

/// A whole call: those sixteen, then the wide read inside its bracket: twenty-two requests.
fn plans_with(count: String, wide: String) -> Vec<ScenarioPlan> {
    plans_with_marks(marks(), count, wide)
}

fn plans_with_marks(marks: String, count: String, wide: String) -> Vec<ScenarioPlan> {
    let mut plans = plans_to_the_count(marks, count);
    bracketed(&mut plans, &plan(wide));
    plans
}

/// The marks the lab read after the accept (voucher mark 69).
fn marks_after() -> String {
    let text = decode_utf16(MARKS_AFTER);
    assert_eq!(text.matches(GUID).count(), 1);
    text
}

fn count() -> String {
    decode_utf16(COUNT)
}

/// The count answer, made to hold `vouchers` vouchers: its first voucher repeated under a
/// master id of its own each time. DERIVED, not Tally's bytes: the lab did not read such a window.
fn count_of(vouchers: usize) -> String {
    repeated(count(), vouchers)
}

/// The wide answer made to hold `vouchers` vouchers in the same way. DERIVED, not Tally's bytes: the
/// lab never read a window of 1,000 vouchers; this repeats its first voucher under new master ids.
fn wide_of(vouchers: usize) -> String {
    repeated(before(), vouchers)
}

fn repeated(xml: String, vouchers: usize) -> String {
    let start = xml.find("<VOUCHER REMOTEID=").expect("a voucher");
    let end = xml.rfind("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let first_end = start + xml[start..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let block = &xml[start..first_end];
    let master = r#"<MASTERID TYPE="Number"> 1</MASTERID>"#;
    assert_eq!(block.matches(master).count(), 1);
    let mut all = String::new();
    for index in 0..vouchers {
        all.push_str(&block.replace(
            master,
            &format!(
                r#"<MASTERID TYPE="Number"> {}</MASTERID>"#,
                1_000_000 + index
            ),
        ));
        all.push_str("\r\n    ");
    }
    format!("{}{}{}", &xml[..start], all, &xml[end..])
}

fn tool_server(
    port: u16,
    directory: &std::path::Path,
    redaction: Redaction,
    max_rows: usize,
) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port,
        },
        data_dir: directory.to_path_buf(),
        max_rows,
        max_bytes: 200_000,
        redaction,
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
async fn run_under(
    plans: Vec<ScenarioPlan>,
    args: Value,
    redaction: Redaction,
    max_rows: usize,
) -> Call {
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = tool_server(
        simulator.address().port(),
        directory.path(),
        redaction,
        max_rows,
    );
    let response = server.call_tool("gst_uncertain_vouchers", args).await;
    let observed = simulator.finish().unwrap();
    Call {
        response,
        observed,
        expected,
    }
}

async fn run(plans: Vec<ScenarioPlan>, args: Value) -> Call {
    run_under(plans, args, Redaction::None, 500).await
}

/// A whole call on the lab's reads of `wide`.
async fn call_on(wide: String) -> Call {
    let call = run(plans_with(count(), wide), lab_arguments()).await;
    assert_eq!(
        call.observed.len(),
        call.expected,
        "every planned request was served"
    );
    call
}

/// A whole call on the lab's reads after the accept: the marks it read after it and `wide`.
async fn call_on_after(wide: String) -> Call {
    let call = run(
        plans_with_marks(marks_after(), count(), wide),
        lab_arguments(),
    )
    .await;
    assert_eq!(
        call.observed.len(),
        call.expected,
        "every planned request was served"
    );
    call
}

/// A call that is refused before any read: the response, and how many requests reached the
/// simulator.
async fn run_expecting_no_read(args: Value) -> (Value, usize) {
    let simulator = SequenceSimulator::spawn(vec![health()]).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = tool_server(
        simulator.address().port(),
        directory.path(),
        Redaction::None,
        500,
    );
    let response = server.call_tool("gst_uncertain_vouchers", args).await;
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

fn items(response: &Value) -> &Vec<Value> {
    payload(response)["items"].as_array().expect("items")
}

/// The items a result should hold: for each voucher, in the tool's order (date, number as text,
/// master id), its row under `uncertain` where its state is uncertain and under `overridden`
/// where its acceptance flag reads Yes.
fn expected_items(rows: &[Expected]) -> Vec<Value> {
    let mut rows = rows.to_vec();
    rows.sort_by(|a, b| (a.0, a.1, a.2).cmp(&(b.0, b.1, b.2)));
    let item = |list: &str, row: &Expected| {
        json!({
            "list": list,
            "voucher_number": row.1,
            "date": row.0,
            "party": row.4,
            "amount": row.5,
            "party_side": "debit",
            "alter_id": row.3,
            "master_id": row.2,
            "gst_status": row.6,
        })
    };
    let mut out = Vec::new();
    for row in &rows {
        if row.6 == "uncertain" {
            out.push(item("uncertain", row));
        }
        if row.7 {
            out.push(item("overridden", row));
        }
    }
    out
}

/// (list, voucher number) of each item.
fn listing(items: &[Value]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|item| {
            (
                item["list"].as_str().unwrap().to_string(),
                item["voucher_number"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

fn pairs(list: &[(&str, &str)]) -> Vec<(String, String)> {
    list.iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

#[tokio::test]
async fn the_tool_lists_the_five_uncertain_vouchers_and_the_five_accepted_ones_before_the_accept() {
    let call = call_on(before()).await;
    assert_eq!(call.expected, 22);
    let result = payload(&call.response);
    assert_eq!(result["state"], "complete");
    assert_eq!(
        result["basis"],
        "tally_voucher_gst_status_party_amount_one_voucher_type_one_window"
    );
    assert_eq!(
        result["window"],
        json!({"from": "20260801", "to": "20260831"})
    );
    assert_eq!(result["voucher_type_name"], LAB_TYPE);
    assert_eq!(result["total"], 32);
    // The cancelled and the optional voucher are counted under their own keys, and the five
    // vouchers whose acceptance flag reads Yes under accepted_as_it_stands.
    assert_eq!(
        result["counts"],
        json!({
            "included": 25,
            "uncertain": 5,
            "excluded": 0,
            "not_in_return": 0,
            "cancelled": 1,
            "optional": 1,
            "post_dated": 0,
            "unread": 0,
            "accepted_as_it_stands": 5,
        })
    );
    // Exactly these five are uncertain, and these five are accepted, in the tool's order.
    assert_eq!(
        listing(items(&call.response)),
        pairs(&[
            ("uncertain", "BP/26-27/0002"),
            ("uncertain", "BP/26-27/0003"),
            ("overridden", "BP/26-27/0005"),
            ("overridden", "BP/26-27/0006"),
            ("uncertain", "BP/26-27/0008"),
            ("overridden", "BP/26-27/0016"),
            ("uncertain", "5"),
            ("uncertain", "6"),
            ("overridden", "BP/26-27/0043"),
            ("overridden", "BP/26-27/0044"),
        ])
    );
    assert_eq!(*items(&call.response), expected_items(&before_rows()));
    // The first row, in full.
    assert_eq!(
        items(&call.response)[0],
        json!({
            "list": "uncertain",
            "voucher_number": "BP/26-27/0002",
            "date": "20260801",
            "party": "BRIDGE Walk-in",
            "amount": "1051.05",
            "party_side": "debit",
            "alter_id": "11",
            "master_id": "2",
            "gst_status": "uncertain",
        })
    );
    // The line as measured (the owner's own wording is not used), with the product's full name; N is
    // the five uncertain.
    assert_eq!(result["pre_filing_line"], LINE_FOR_FIVE);
    assert!(
        result["filed_period_note"]
            .as_str()
            .unwrap()
            .contains("may differ from your filed return; check against it."),
        "{result}"
    );
    // The company block.
    let company = &call.response["structuredContent"]["company"];
    assert_eq!(company["name"], LAB_COMPANY);
    assert_eq!(company["identity_state"], "verified_tuple");
}

#[tokio::test]
async fn the_tool_sends_the_count_read_and_then_the_wide_read_the_lab_sent() {
    let call = call_on(before()).await;
    // The sixth and eighth requests of the call are the marks; the status read inside its
    // bracket is the twelfth, read again as the fourteenth, and the wide read the eighteenth
    // and the twentieth.
    // The company list and the marks requests are the ones the lab was sent for its answers.
    assert_eq!(
        call.observed[0].request_body_sha256,
        COMPANY_LIST_REQUEST_SHA256
    );
    assert_eq!(call.observed[5].request_body_sha256, MARKS_REQUEST_SHA256);
    assert_eq!(call.observed[7].request_body_sha256, MARKS_REQUEST_SHA256);
    // Every company-list send, the two plain ones and the six that open and close the three
    // brackets, is the request the lab was sent for its company list.
    for at in [0, 2, 4, 9, 10, 15, 16, 21] {
        assert_eq!(
            call.observed[at].request_body_sha256, COMPANY_LIST_REQUEST_SHA256,
            "{at}"
        );
    }
    assert_eq!(call.observed[11].request_body_sha256, NARROW_REQUEST_SHA256);
    assert_eq!(call.observed[13].request_body_sha256, NARROW_REQUEST_SHA256);
    assert_eq!(call.observed[17].request_body_sha256, WIDE_REQUEST_SHA256);
    assert_eq!(call.observed[19].request_body_sha256, WIDE_REQUEST_SHA256);
    let narrow_sends = call
        .observed
        .iter()
        .filter(|request| request.request_body_sha256 == NARROW_REQUEST_SHA256)
        .count();
    let wide_sends = call
        .observed
        .iter()
        .filter(|request| request.request_body_sha256 == WIDE_REQUEST_SHA256)
        .count();
    assert_eq!((narrow_sends, wide_sends), (2, 2));
    // The receipt commits to the four reads: the company list, the marks, the count and the
    // wide read, each with its answer.
    let receipt = receipt(&call.response);
    assert_eq!(receipt["state"], "complete");
    assert!(receipt["reason_code"].is_null(), "{receipt}");
    let join = |left: &str, right: &str| sha256_hex(format!("{left}:{right}").as_bytes());
    assert_eq!(
        receipt["request_sha256"],
        join(
            &join(
                &join(
                    &call.observed[0].request_body_sha256,
                    &call.observed[5].request_body_sha256
                ),
                &call.observed[11].request_body_sha256,
            ),
            &call.observed[17].request_body_sha256,
        )
    );
    let answers = [
        plan(companies()).response_bytes(),
        plan(marks()).response_bytes(),
        plan(count()).response_bytes(),
        plan(before()).response_bytes(),
    ];
    assert_eq!(
        answers[1], MARKS,
        "the replayed marks are the committed bytes"
    );
    assert_eq!(
        answers[2], COUNT,
        "the replayed count is the committed bytes"
    );
    assert_eq!(
        answers[3], BEFORE,
        "the replayed answer is the committed bytes"
    );
    assert_eq!(
        receipt["response_sha256"],
        join(
            &join(
                &join(&sha256_hex(&answers[0]), &sha256_hex(&answers[1])),
                &sha256_hex(&answers[2])
            ),
            &sha256_hex(&answers[3])
        )
    );
}

#[tokio::test]
async fn after_the_accept_the_third_voucher_is_overridden_and_included_with_the_same_amount() {
    let earlier = call_on(before()).await;
    let later = call_on_after(after()).await;
    let result = payload(&later.response);
    assert_eq!(result["state"], "complete");
    assert_eq!(result["counts"]["uncertain"], 4);
    assert_eq!(result["counts"]["included"], 26);
    assert_eq!(result["counts"]["accepted_as_it_stands"], 6);
    assert_eq!(*items(&later.response), expected_items(&after_rows()));
    assert!(result["pre_filing_line"]
        .as_str()
        .unwrap()
        .starts_with("Tally marks 4 vouchers of the type BRIDGE Sales dated 2026-08-01 to 2026-08-31 as Uncertain"));
    // Nothing but that voucher changed lists: the vouchers the earlier run listed, but the
    // third, are listed as they were.
    let find = |response: &Value, list: &str, number: &str| {
        items(response)
            .iter()
            .find(|item| item["list"] == list && item["voucher_number"] == number)
            .cloned()
    };
    let before_item = find(&earlier.response, "uncertain", TARGET).expect("uncertain before");
    assert!(find(&later.response, "uncertain", TARGET).is_none());
    let after_item = find(&later.response, "overridden", TARGET).expect("overridden after");
    assert_eq!(before_item["master_id"], after_item["master_id"]);
    assert_eq!(before_item["amount"], after_item["amount"]);
    assert_eq!(before_item["party_side"], after_item["party_side"]);
    assert_eq!(after_item["gst_status"], "included");
    assert_eq!(before_item["alter_id"], "50");
    assert_eq!(after_item["alter_id"], "69");
    for item in items(&earlier.response) {
        if item["voucher_number"] != TARGET {
            assert!(items(&later.response).contains(item), "{item}");
        }
    }
    assert_eq!(items(&later.response).len(), items(&earlier.response).len());
}

/// What the assistant is told to do with two runs: the acceptance of a voucher is confirmed
/// only if the earlier run listed it uncertain under a master id and this run lists it
/// overridden and included under that master id with the same amount.
fn confirmed(earlier: &Value, later: &Value) -> Vec<String> {
    let mut out = Vec::new();
    for before in items(earlier)
        .iter()
        .filter(|item| item["list"] == "uncertain")
    {
        let accepted = items(later).iter().any(|item| {
            item["list"] == "overridden"
                && item["gst_status"] == "included"
                && item["master_id"] == before["master_id"]
                && item["amount"] == before["amount"]
        });
        if accepted {
            out.push(before["voucher_number"].as_str().unwrap().to_string());
        }
    }
    out
}

#[tokio::test]
async fn two_runs_hold_what_a_confirmation_compares_and_confirm_only_the_accepted_voucher() {
    let earlier = call_on(before()).await;
    let later = call_on_after(after()).await;
    assert_eq!(confirmed(&earlier.response, &later.response), [TARGET]);
    // Run again with nothing accepted: nothing is confirmed.
    let same = call_on(before()).await;
    assert!(confirmed(&earlier.response, &same.response).is_empty());
    // An accepted voucher whose amount changed is not confirmed: edit the amount in both of the
    // wide reads' entries and in the voucher itself.
    let changed = edit(&after(), TARGET, |block| {
        block.replace("-1051.04", "-1051.03")
    });
    let edited = call_on_after(changed).await;
    assert!(confirmed(&earlier.response, &edited.response).is_empty());
    // A voucher accepted under another master id is not confirmed.
    let other = edit(&after(), TARGET, |block| {
        once(
            &block,
            r#"<MASTERID TYPE="Number"> 3</MASTERID>"#,
            r#"<MASTERID TYPE="Number"> 99</MASTERID>"#,
        )
    });
    let other_master = call_on_after(other).await;
    assert!(confirmed(&earlier.response, &other_master.response).is_empty());
}

#[tokio::test]
async fn the_cancelled_and_the_optional_voucher_are_not_listed_and_the_line_does_not_count_them() {
    let call = call_on(before()).await;
    let listed = listing(items(&call.response));
    for number in ["BP/26-27/0041", "BP/26-27/0042"] {
        assert!(
            listed.iter().all(|(_, listed)| listed != number),
            "{number}"
        );
    }
    assert_eq!(payload(&call.response)["pre_filing_line"], LINE_FOR_FIVE);
    // A cancelled voucher whose status flags read uncertain is still cancelled, and is not
    // counted: its party and amount read empty, as the lab's did.
    let uncertain_cancelled = edit(&before(), "BP/26-27/0041", |block| {
        let block = once(
            &block,
            r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Yes</VCHGSTSTATUSISINCLUDED>"#,
            r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">No</VCHGSTSTATUSISINCLUDED>"#,
        );
        once(
            &block,
            r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">No</VCHGSTSTATUSISUNCERTAIN>"#,
            r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">Yes</VCHGSTSTATUSISUNCERTAIN>"#,
        )
    });
    let call = call_on(uncertain_cancelled).await;
    let result = payload(&call.response);
    assert_eq!(result["counts"]["uncertain"], 5);
    assert_eq!(result["counts"]["cancelled"], 1);
    assert_eq!(result["pre_filing_line"], LINE_FOR_FIVE);
    assert_eq!(listing(items(&call.response)).len(), 10);
}

#[tokio::test]
async fn a_cancelled_voucher_whose_acceptance_flag_reads_yes_is_listed_and_makes_the_result_partial(
) {
    let call = call_on(edit(&before(), "BP/26-27/0041", |block| {
        once(
            &block,
            r#"<ISGSTOVERRIDDEN TYPE="Logical">No</ISGSTOVERRIDDEN>"#,
            r#"<ISGSTOVERRIDDEN TYPE="Logical">Yes</ISGSTOVERRIDDEN>"#,
        )
    }))
    .await;
    let result = payload(&call.response);
    assert_eq!(result["state"], "partial");
    assert_eq!(result["counts"]["cancelled"], 1);
    assert_eq!(result["counts"]["unread"], 1);
    assert_eq!(
        result["pre_filing_line"],
        json!({"withheld": "gst_uncertain_voucher_unread"})
    );
    let listed = items(&call.response)
        .iter()
        .find(|item| item["voucher_number"] == "BP/26-27/0041")
        .expect("listed under overridden");
    assert_eq!(listed["list"], "overridden");
    assert_eq!(listed["party"], json!({"unread": PARTY_EMPTY}));
}

/// The lab's five uncertain vouchers, each given the flags of an included one.
fn nothing_uncertain() -> String {
    let mut xml = before();
    for number in ["BP/26-27/0002", "BP/26-27/0003", "BP/26-27/0008", "5", "6"] {
        xml = edit(&xml, number, |block| {
            let block = once(
                &block,
                r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">No</VCHGSTSTATUSISINCLUDED>"#,
                r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Yes</VCHGSTSTATUSISINCLUDED>"#,
            );
            once(
                &block,
                r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">Yes</VCHGSTSTATUSISUNCERTAIN>"#,
                r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">No</VCHGSTSTATUSISUNCERTAIN>"#,
            )
        });
    }
    xml
}

#[tokio::test]
async fn a_window_with_vouchers_and_none_uncertain_withholds_the_line_and_keeps_the_counts() {
    let call = call_on(nothing_uncertain()).await;
    let result = payload(&call.response);
    // Every voucher was read: the result is complete, with the counts and the five accepted
    // vouchers. A line saying "0 vouchers" would read as clearance, so it is withheld with its
    // own reason, not the unread one.
    assert_eq!(result["state"], "complete");
    assert_eq!(result["total"], 32);
    assert_eq!(result["counts"]["uncertain"], 0);
    assert_eq!(result["counts"]["included"], 30);
    assert_eq!(result["counts"]["unread"], 0);
    assert_eq!(result["counts"]["accepted_as_it_stands"], 5);
    assert_eq!(
        result["pre_filing_line"],
        json!({"withheld": "gst_uncertain_none_marked_uncertain"})
    );
    assert_eq!(items(&call.response).len(), 5);
    assert!(items(&call.response)
        .iter()
        .all(|item| item["list"] == "overridden"));
    assert!(result["filed_period_note"].is_string());
    assert_eq!(receipt(&call.response)["state"], "complete");
}

#[tokio::test]
async fn the_line_names_the_voucher_type_and_one_uncertain_voucher_is_enough_to_state_it() {
    // The five become one: the line is stated for N = 1 and ends with the type it covers.
    let mut xml = nothing_uncertain();
    xml = edit(&xml, "BP/26-27/0002", |block| {
        let block = once(
            &block,
            r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Yes</VCHGSTSTATUSISINCLUDED>"#,
            r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">No</VCHGSTSTATUSISINCLUDED>"#,
        );
        once(
            &block,
            r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">No</VCHGSTSTATUSISUNCERTAIN>"#,
            r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">Yes</VCHGSTSTATUSISUNCERTAIN>"#,
        )
    });
    let call = call_on(xml).await;
    let result = payload(&call.response);
    assert_eq!(result["counts"]["uncertain"], 1);
    assert_eq!(
        result["pre_filing_line"],
        LINE_FOR_FIVE.replace("marks 5 vouchers", "marks 1 voucher")
    );
    // The line names the type read and never says the type is a Sales type.
    assert!(result["pre_filing_line"]
        .as_str()
        .unwrap()
        .contains(&format!("voucher of the type {LAB_TYPE} dated")));
}

#[tokio::test]
async fn a_post_dated_voucher_is_decided_before_its_flags_and_is_not_counted_uncertain() {
    // The second voucher (uncertain) marked post-dated: derived, the lab's one post-dated
    // voucher was in another book.
    let call = call_on(edit(&before(), "BP/26-27/0002", |block| {
        once(
            &block,
            r#"<ISPOSTDATED TYPE="Logical">No</ISPOSTDATED>"#,
            r#"<ISPOSTDATED TYPE="Logical">Yes</ISPOSTDATED>"#,
        )
    }))
    .await;
    let result = payload(&call.response);
    assert_eq!(result["counts"]["uncertain"], 4);
    assert_eq!(result["counts"]["post_dated"], 1);
    assert!(result["pre_filing_line"]
        .as_str()
        .unwrap()
        .starts_with("Tally marks 4 vouchers of the type BRIDGE Sales"));
    assert!(listing(items(&call.response))
        .iter()
        .all(|(_, number)| number != "BP/26-27/0002"));
}

#[tokio::test]
async fn an_accepted_voucher_that_reads_uncertain_or_excluded_now_is_listed_with_its_status() {
    // The third voucher after the accept, its flags put back to uncertain, then to excluded
    // (derived): the acceptance flag still reads Yes.
    let flags = |included: &str, uncertain: &str, excluded: &str| {
        edit(&after(), TARGET, move |block| {
            let block = replace_direct(
                &block,
                r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Yes</VCHGSTSTATUSISINCLUDED>"#,
                &format!(
                    r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">{included}</VCHGSTSTATUSISINCLUDED>"#
                ),
            );
            let block = replace_direct(
                &block,
                r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">No</VCHGSTSTATUSISUNCERTAIN>"#,
                &format!(
                    r#"<VCHGSTSTATUSISUNCERTAIN TYPE="Logical">{uncertain}</VCHGSTSTATUSISUNCERTAIN>"#
                ),
            );
            replace_direct(
                &block,
                r#"<VCHGSTSTATUSISEXCLUDED TYPE="Logical">No</VCHGSTSTATUSISEXCLUDED>"#,
                &format!(
                    r#"<VCHGSTSTATUSISEXCLUDED TYPE="Logical">{excluded}</VCHGSTSTATUSISEXCLUDED>"#
                ),
            )
        })
    };
    let call = call_on_after(flags("No", "Yes", "No")).await;
    let result = payload(&call.response);
    assert_eq!(result["counts"]["uncertain"], 5);
    assert_eq!(result["counts"]["accepted_as_it_stands"], 6);
    // On both lists, once under each, with the status it reads now.
    let third: Vec<_> = items(&call.response)
        .iter()
        .filter(|item| item["voucher_number"] == TARGET)
        .collect();
    assert_eq!(third.len(), 2);
    assert_eq!(third[0]["list"], "uncertain");
    assert_eq!(third[1]["list"], "overridden");
    for item in &third {
        assert_eq!(item["gst_status"], "uncertain");
    }
    let call = call_on_after(flags("No", "No", "Yes")).await;
    let result = payload(&call.response);
    assert_eq!(result["counts"]["uncertain"], 4);
    assert_eq!(result["counts"]["excluded"], 1);
    let third: Vec<_> = items(&call.response)
        .iter()
        .filter(|item| item["voucher_number"] == TARGET)
        .collect();
    assert_eq!(third.len(), 1);
    assert_eq!(third[0]["list"], "overridden");
    assert_eq!(third[0]["gst_status"], "excluded");
}

#[tokio::test]
async fn a_listed_voucher_whose_amount_or_party_is_unread_makes_the_result_partial_and_withholds_the_line(
) {
    let unread = "gst_uncertain_voucher_unread";
    type Change = Box<dyn Fn(String) -> String>;
    let cases: [(&str, Change, Value, Value); 4] = [
        (
            "empty amount",
            Box::new(|block| {
                replace_direct(&block, DIRECT_AMOUNT, r#"<AMOUNT TYPE="Amount"></AMOUNT>"#)
            }),
            json!({"unread": AMOUNT_UNREADABLE}),
            json!(LAB_PARTY),
        ),
        (
            "two direct amounts",
            Box::new(|block| {
                replace_direct(
                    &block,
                    DIRECT_AMOUNT,
                    &format!("{DIRECT_AMOUNT}{DIRECT_AMOUNT}"),
                )
            }),
            json!({"unread": AMOUNT_UNREADABLE}),
            json!(LAB_PARTY),
        ),
        (
            "no direct amount",
            Box::new(|block| replace_direct(&block, DIRECT_AMOUNT, "")),
            json!({"unread": AMOUNT_NOT_REPORTED}),
            json!(LAB_PARTY),
        ),
        (
            "foreign-currency amount",
            Box::new(|block| {
                replace_direct(
                    &block,
                    DIRECT_AMOUNT,
                    r#"<AMOUNT TYPE="Amount">-$ 12.50 @ Rs. 83 = Rs. -1037.50</AMOUNT>"#,
                )
            }),
            json!({"unread": AMOUNT_UNREADABLE}),
            json!(LAB_PARTY),
        ),
    ];
    for (label, change, amount, party) in cases {
        let call = call_on(edit(&before(), TARGET, change)).await;
        let result = payload(&call.response);
        assert_eq!(result["state"], "partial", "{label}");
        assert_eq!(result["counts"]["unread"], 1, "{label}");
        // The five are still counted; only the line that would state them is withheld.
        assert_eq!(result["counts"]["uncertain"], 5, "{label}");
        assert_eq!(
            result["pre_filing_line"],
            json!({"withheld": unread}),
            "{label}"
        );
        assert!(result.get("filed_period_note").is_none(), "{label}");
        let item = items(&call.response)
            .iter()
            .find(|item| item["voucher_number"] == TARGET)
            .unwrap();
        assert_eq!(item["amount"], amount, "{label}");
        assert_eq!(item["party_side"], amount, "{label}");
        assert_eq!(item["party"], party, "{label}");
        assert_eq!(item["master_id"], "3");
        assert_eq!(receipt(&call.response)["state"], "partial", "{label}");
        assert_eq!(receipt(&call.response)["reason_code"], unread, "{label}");
    }
    // An empty party is never a name: unread, in the shape of an unread value, whatever the
    // masking setting.
    for redaction in [Redaction::None, Redaction::MaskParties] {
        let changed = edit(&before(), TARGET, |block| {
            replace_direct(
                &block,
                PARTY_LINE,
                r#"<PARTYLEDGERNAME TYPE="String"></PARTYLEDGERNAME>"#,
            )
        });
        let call = run_under(
            plans_with(count(), changed),
            lab_arguments(),
            redaction,
            500,
        )
        .await;
        let result = payload(&call.response);
        assert_eq!(result["state"], "partial");
        assert_eq!(result["pre_filing_line"], json!({"withheld": unread}));
        let item = items(&call.response)
            .iter()
            .find(|item| item["voucher_number"] == TARGET)
            .unwrap();
        assert_eq!(
            item["party"],
            json!({"unread": PARTY_EMPTY}),
            "{redaction:?}"
        );
    }
}

#[tokio::test]
async fn an_unread_status_makes_the_result_partial_and_a_voucher_not_listed_does_not() {
    // A voucher whose status flags cannot be read may be an uncertain one: the line is
    // withheld.
    let changed = edit(&before(), "BP/26-27/0004", |block| {
        replace_direct(
            &block,
            r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Yes</VCHGSTSTATUSISINCLUDED>"#,
            r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Maybe</VCHGSTSTATUSISINCLUDED>"#,
        )
    });
    let call = call_on(changed).await;
    let result = payload(&call.response);
    assert_eq!(result["state"], "partial");
    assert_eq!(result["counts"]["unread"], 1);
    assert_eq!(
        result["pre_filing_line"],
        json!({"withheld": "gst_uncertain_voucher_unread"})
    );
    // A voucher that is neither uncertain nor accepted, with no amount and no party, is not
    // listed and does not matter: the result stays complete, as the cancelled voucher's does.
    let changed = edit(&before(), "BP/26-27/0001", |block| {
        let block = replace_direct(&block, DIRECT_AMOUNT_1050, "");
        replace_direct(&block, PARTY_LINE, "")
    });
    let call = call_on(changed).await;
    let result = payload(&call.response);
    assert_eq!(result["state"], "complete");
    assert_eq!(result["counts"]["unread"], 0);
    assert_eq!(result["pre_filing_line"], LINE_FOR_FIVE);
    // A deleted voucher has no state.
    let changed = edit(&before(), "BP/26-27/0004", |block| {
        once(
            &block,
            "<ISDELETED>No</ISDELETED>",
            "<ISDELETED>Yes</ISDELETED>",
        )
    });
    let call = call_on(changed).await;
    assert_eq!(payload(&call.response)["state"], "partial");
}

#[tokio::test]
async fn a_listed_voucher_whose_flags_and_party_are_both_unread_is_counted_once() {
    // BP/26-27/0005 reads accepted as it stands (so it is listed), with its included flag spoiled
    // and its party removed (derived): its flags are unread, which counts it, and its party is
    // unread as well, which must not count it again.
    let changed = edit(&before(), "BP/26-27/0005", |block| {
        let block = replace_direct(
            &block,
            r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Yes</VCHGSTSTATUSISINCLUDED>"#,
            r#"<VCHGSTSTATUSISINCLUDED TYPE="Logical">Maybe</VCHGSTSTATUSISINCLUDED>"#,
        );
        replace_direct(&block, PARTY_LINE, "")
    });
    let call = call_on(changed).await;
    let result = payload(&call.response);
    assert_eq!(result["state"], "partial");
    assert_eq!(result["counts"]["unread"], 1);
    assert_eq!(
        result["pre_filing_line"],
        json!({"withheld": "gst_uncertain_voucher_unread"})
    );
}

#[tokio::test]
async fn the_result_carries_no_offset_so_the_byte_cap_cannot_trim_it_with_a_cursor() {
    let call = call_on(before()).await;
    let result = payload(&call.response);
    for key in ["offset", "next_offset", "limit"] {
        assert!(result.get(key).is_none(), "{key}");
    }
    assert!(result["items"].is_array());
}

const DIRECT_AMOUNT_1050: &str = r#"<AMOUNT TYPE="Amount">-1050.00</AMOUNT>"#;

#[tokio::test]
async fn a_window_of_a_thousand_vouchers_is_read_and_a_thousand_and_one_is_refused_before_the_wide_read(
) {
    // 1,000: both reads are made, on a count answer and a wide answer that are both DERIVED (the
    // lab's first voucher repeated), so this proves the bound and not any read of the lab.
    let call = run(plans_with(count_of(1_000), wide_of(1_000)), lab_arguments()).await;
    assert_eq!(call.observed.len(), 22);
    assert_eq!(payload(&call.response)["state"], "complete");
    assert_eq!(
        call.observed
            .iter()
            .filter(|request| request.request_body_sha256 == WIDE_REQUEST_SHA256)
            .count(),
        2
    );
    // 1,001: the count read is made, and no wide request is sent.
    let call = run(
        plans_to_the_count(marks(), count_of(1_001)),
        lab_arguments(),
    )
    .await;
    assert_eq!(call.observed.len(), 16);
    assert_eq!(
        refusal_code(&call.response),
        "gst_uncertain_window_too_large"
    );
    assert!(call
        .observed
        .iter()
        .all(|request| request.request_body_sha256 != WIDE_REQUEST_SHA256));
    assert_eq!(
        call.response["structuredContent"]["result"]["error"]["counts"],
        json!({"returned": 0, "counted": 1001})
    );
    assert!(call.response["structuredContent"]["result"]
        .get("items")
        .is_none());
    let receipt = receipt(&call.response);
    assert_eq!(receipt["state"], "partial");
    assert_eq!(receipt["reason_code"], "gst_uncertain_window_too_large");
    assert!(receipt["bytes"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn a_read_that_fails_is_a_refusal_and_never_a_window_with_no_vouchers() {
    // The wide answer cut off: a protocol error, not an empty list.
    let wide = before();
    let cut = wide[..wide.len() / 2].to_string();
    let call = run(plans_with(count(), cut), lab_arguments()).await;
    assert_eq!(call.observed.len(), call.expected);
    let code = refusal_code(&call.response).to_string();
    assert!(code.starts_with("gst_status_read_"), "{code}");
    assert!(call.response["structuredContent"]["result"]
        .get("items")
        .is_none());
    assert!(call.response["structuredContent"]["result"]
        .get("total")
        .is_none());
    // The count answer cut off: refused before any wide request.
    let count = count();
    let cut = count[..count.len() / 2].to_string();
    let call = run(plans_to_the_count(marks(), cut), lab_arguments()).await;
    assert_eq!(call.observed.len(), 16);
    assert!(refusal_code(&call.response).starts_with("gst_status_read_"));
    assert!(call
        .observed
        .iter()
        .all(|request| request.request_body_sha256 != WIDE_REQUEST_SHA256));
}

/// `xml` with its vouchers from the `keep`th on removed (derived: the lab's answers always held
/// all of the window's vouchers).
fn keeping(xml: &str, keep: usize) -> String {
    let opens: Vec<usize> = xml
        .match_indices("<VOUCHER REMOTEID=")
        .map(|(at, _)| at)
        .collect();
    assert!(keep <= opens.len());
    let end = xml.rfind("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let cut = if keep < opens.len() { opens[keep] } else { end };
    format!("{}{}", &xml[..cut], &xml[end..])
}

#[tokio::test]
async fn a_wide_read_with_fewer_vouchers_than_the_count_read_found_is_refused_not_an_empty_window()
{
    // The count read found 32 vouchers: a wide read of none (the collection and no voucher), or of
    // fewer than 32, contradicts it and is refused with both numbers, never reported as a window
    // with none (derived: the lab's two reads always agreed).
    for kept in [0, 1, 31] {
        let call = call_on(keeping(&before(), kept)).await;
        assert_eq!(
            refusal_code(&call.response),
            "gst_uncertain_wide_read_short",
            "{kept}"
        );
        assert_eq!(
            call.response["structuredContent"]["result"]["error"]["counts"],
            json!({"returned": kept, "counted": 32}),
            "{kept}"
        );
        let result = &call.response["structuredContent"]["result"];
        assert!(result.get("items").is_none() && result.get("total").is_none());
        assert_eq!(receipt(&call.response)["state"], "partial");
        assert_eq!(
            receipt(&call.response)["reason_code"],
            "gst_uncertain_wide_read_short"
        );
    }
    // All 32 are read: no refusal.
    assert_eq!(keeping(&before(), 32), before());
    let call = call_on(keeping(&before(), 32)).await;
    assert_eq!(payload(&call.response)["total"], 32);
}

#[tokio::test]
async fn a_window_the_count_read_found_empty_is_complete_with_the_line_withheld() {
    // The count read and the wide read both hold no voucher (derived by removing the vouchers from
    // the lab's answers): a window with none, said as such, which a mistyped type name also gives.
    let call = run(
        plans_with(keeping(&count(), 0), keeping(&before(), 0)),
        lab_arguments(),
    )
    .await;
    assert_eq!(call.observed.len(), call.expected);
    let result = payload(&call.response);
    assert_eq!(result["total"], 0);
    assert_eq!(result["state"], "complete");
    assert_eq!(items(&call.response).len(), 0);
    // The counts are all read, and zero; a line saying "0 vouchers" would read as clearance
    // (a mistyped type name reads no voucher), so the line is withheld with its own reason.
    assert_eq!(result["counts"]["uncertain"], 0);
    assert_eq!(result["counts"]["unread"], 0);
    assert_eq!(
        result["pre_filing_line"],
        json!({"withheld": "gst_uncertain_no_vouchers_in_window"})
    );
}

#[tokio::test]
async fn the_wide_read_is_refused_when_it_holds_a_voucher_outside_the_window_or_of_another_type() {
    let call = call_on(edit(&before(), TARGET, |block| {
        once(
            &block,
            r#"<DATE TYPE="Date">20260801</DATE>"#,
            r#"<DATE TYPE="Date">20260901</DATE>"#,
        )
    }))
    .await;
    assert_eq!(refusal_code(&call.response), "window_not_honoured");
    assert_eq!(
        receipt(&call.response)["reason_code"],
        "window_not_honoured"
    );
    let call = call_on(edit(&before(), TARGET, |block| {
        once(
            &block,
            "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>",
            "<VOUCHERTYPENAME>BRIDGE Sales Z1</VOUCHERTYPENAME>",
        )
    }))
    .await;
    assert_eq!(refusal_code(&call.response), "gst_status_type_not_honoured");
    assert_eq!(
        receipt(&call.response)["reason_code"],
        "gst_status_type_not_honoured"
    );
    // A voucher repeated in the wide read refuses the whole answer.
    let repeated = {
        let wide = before();
        let block_start = wide.find("<VOUCHER REMOTEID=").unwrap();
        let block_end =
            block_start + wide[block_start..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
        let block = wide[block_start..block_end].to_string();
        format!("{}{}{}", &wide[..block_end], block, &wide[block_end..])
    };
    let call = call_on(repeated).await;
    assert_eq!(
        refusal_code(&call.response),
        "gst_status_read_voucher_repeated"
    );
}

#[tokio::test]
async fn more_rows_than_the_servers_row_limit_are_refused_with_their_counts() {
    // Ten rows: at a limit of ten they are returned, at nine they are refused.
    let call = run_under(
        plans_with(count(), before()),
        lab_arguments(),
        Redaction::None,
        10,
    )
    .await;
    assert_eq!(
        payload(&call.response)["items"].as_array().unwrap().len(),
        10
    );
    let call = run_under(
        plans_with(count(), before()),
        lab_arguments(),
        Redaction::None,
        9,
    )
    .await;
    assert_eq!(call.observed.len(), 22);
    assert_eq!(
        refusal_code(&call.response),
        "gst_uncertain_rows_over_limit"
    );
    assert_eq!(
        call.response["structuredContent"]["result"]["error"]["counts"],
        json!({"returned": 0, "counted": 10})
    );
    assert!(call.response["structuredContent"]["result"]
        .get("items")
        .is_none());
}

#[tokio::test]
async fn party_names_are_masked_under_mask_parties_and_amounts_are_not() {
    let call = run_under(
        plans_with(count(), before()),
        lab_arguments(),
        Redaction::MaskParties,
        500,
    )
    .await;
    let whole = call.response.to_string();
    assert!(!whole.contains(LAB_PARTY), "{whole}");
    // The names are masked to the form the other tools mask to, and the amounts are not.
    let item = &items(&call.response)[0];
    assert_eq!(item["party"], mask(LAB_PARTY));
    assert_ne!(item["party"], LAB_PARTY);
    assert_eq!(item["amount"], "1051.05");
    // Unmasked, the same call returns the name.
    let call = call_on(before()).await;
    assert_eq!(items(&call.response)[0]["party"], LAB_PARTY);
}

#[tokio::test]
async fn arguments_that_cannot_be_read_are_refused_before_any_request_is_sent() {
    for (args, expected) in [
        (
            arguments("20260802", "20260803", "BRIDGE \"Sales"),
            "gst_status_type_name_invalid",
        ),
        (
            arguments("20260802", "20260803", "BRIDGE Sales "),
            "gst_status_type_name_invalid",
        ),
        (
            json!({"company_guid": GUID, "from": "20260802", "to": "20260803"}),
            "voucher_type_name_required",
        ),
        (
            arguments("20260401", "20260703", LAB_TYPE),
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
    }
    // The tool pages nothing: a page argument is not one of its arguments.
    for extra in ["limit", "offset"] {
        let mut args = lab_arguments();
        args[extra] = json!(1);
        let (response, received) = run_expecting_no_read(args).await;
        assert_eq!(response["isError"], true, "{extra}");
        assert_eq!(received, 0, "{extra}");
    }
}

#[tokio::test]
async fn a_book_with_a_voucher_mark_above_the_limit_is_refused_before_any_status_read() {
    let call = run(
        plans_to_the_marks(marks_with_voucher_mark(25_001)),
        lab_arguments(),
    )
    .await;
    assert_eq!(call.expected, 10);
    assert_eq!(call.observed.len(), 10);
    assert_eq!(refusal_code(&call.response), "gst_status_book_too_large");
    // At the limit both reads are made.
    let call = run(
        {
            let mut plans = plans_to_the_count(marks_with_voucher_mark(25_000), count());
            bracketed(&mut plans, &plan(before()));
            plans
        },
        lab_arguments(),
    )
    .await;
    assert_eq!(call.observed.len(), 22);
    assert_eq!(payload(&call.response)["state"], "complete");
}

#[tokio::test]
async fn the_limitations_say_what_was_not_read_and_what_was_not_measured() {
    let call = call_on(before()).await;
    let limitations = payload(&call.response)["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|line| line.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    for stated in [
        "The reason for uncertainty is not read",
        "Bills missing an HSN/SAC are not split out",
        "they need re-saving with the code, not Accept As Is",
        "Amounts are not masked by the party masking setting",
        "gst_status carries neither",
        "The acceptance is confirmed only if an earlier run listed the voucher uncertain under the same master_id",
        "keeps no state",
        "the invoice total on the party's entry, tax and round-off included",
        "not the taxable value of the GSTR-1 tables",
        "a cash sale shows the cash ledger",
        "no GSTIN is fetched",
        "an unregistered buyer only",
        "item invoices, a registered buyer, credit notes or other Sales-class types, foreign currency, the amount of a post-dated voucher, an Accept As Is of several vouchers at once, Gold and other releases",
        "the pre-filing line is withheld",
        "not that the voucher is correct or complete",
        "the origin of the other three is not recorded",
        "gst_uncertain_wide_read_short",
        "agent_response_too_large",
        "listed under overridden",
        "above 1,000 vouchers",
        "provisional",
    ] {
        assert!(limitations.contains(stated), "{stated}");
    }
    // Public text names the product in full and uses none of the words kept out of it.
    let text = format!("{limitations} {LINE_FOR_FIVE}");
    // The words are written in two pieces so that this file does not hold them itself.
    for pieces in [
        ["mov", "ed"],
        [" fr", "ee "],
        ["off", "line"],
        ["pre", "view"],
    ] {
        let word = pieces.concat();
        assert!(!text.contains(&word), "{word}");
    }
    assert!(
        !text.replace("ComplyEaze Bridge", "").contains("Bridge"),
        "the product is named ComplyEaze Bridge"
    );
}

#[test]
fn the_tool_is_registered_as_a_read_with_the_arguments_of_the_status_read_and_no_paging() {
    let definitions = catalog::registered_tool_definitions(true, true);
    let tool = definitions
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "gst_uncertain_vouchers")
        .expect("registered");
    assert_eq!(tool["annotations"]["readOnlyHint"], true);
    assert_eq!(
        tool["inputSchema"]["required"],
        json!(["company_guid", "from", "to", "voucher_type_name"])
    );
    assert!(tool["inputSchema"]["properties"].get("limit").is_none());
    assert!(tool["inputSchema"]["properties"].get("offset").is_none());
}
