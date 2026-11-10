//! Lab only, never merged: renders row 2480-NM's `Ledger Outstandings` requests, one for each ledger of
//! `names.tsv`, by the template of master's own builder with only the name changed.
//!
//! Master's builder takes the party only as a `LedgerScopeName`, and that type refuses ten of the eleven
//! names (ledger_scope.rs:65 and :74). So the request is built here by a copy of the builder's template
//! (`TEMPLATE`, from `render_ledger_outstandings_request`, native_outstandings/party_bills.rs), with the
//! party escaped by the same `escape_text`. Before anything is written, the test checks the copy:
//! - the source file of the builder holds `TEMPLATE` as written here, character for character;
//! - for the one name the type admits (`pl`), the copy's request is byte-equal to the builder's own;
//! - for row 2346-PB's name, which the type refuses and which row 2346-PB sent to Tally, the copy's
//!   request is byte-equal to the file row 2346-PB sent (`pb/L-lo-bc-20260801.xml`);
//! - the snapshot builder at row 2346-PB's to-date is byte-equal to row 2346-PB's `S-snap-20260401-20260801.xml`,
//!   and the snapshot builder for the 29 Oct request is byte-equal to the request the build sent then
//!   (`snapshot-build-request-forex-20250401-20250430.bin`, row 2377-NB's control).
//! Then, for each name, it checks that `LedgerScopeName::new` answers as `names.tsv`'s ids say (the control
//! admitted, `op` refused as a formula word, every other refused as a character), that the name occurs in
//! the request once, and that the request is the control's with the name turned, and nothing else.
//!
//! Run (one build slot, a target directory of its own):
//! `LAB_2480_NM_OUT=<abs path of lab/2480-nm> cargo test -p bridge-tally-protocol --test lab_2480_nm_render
//! -- --ignored --exact lab_2480_nm_renders_the_row_requests --nocapture`.
//! Each request is written as the build sends it: UTF-16LE with a BOM, as `L-lo-<id>-20260801.xml`.

use bridge_tally_primitives::TallyDate;
use bridge_tally_protocol::ledger_scope::{LedgerScopeName, LedgerScopeNameError};
use bridge_tally_protocol::native_outstandings::{
    render_ledger_outstandings_request, render_native_ledger_snapshot_request,
    NativeLedgerSnapshotPeriod,
};
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;
use bridge_tally_protocol::xml_text::escape_text;

/// The book's name, as the build sends it (`identity.display_name()`).
const BOOK: &str = "BRIDGE PILOT LAB";
/// The book's first day (its BOOKSFROM, `pilot-lab-register-book-extent.utf16le.xml`).
const BOOKS_FROM: &str = "20260401";
/// The to-date of every read of this row: row 2346-PB's, the invoices' own date.
const TO: &str = "20260801";
/// Row 2346-PB's customer, which `LedgerScopeName` refuses and row 2346-PB sent.
const PB_PARTY: &str = "BRIDGE Walk-in BC";

/// The template of `render_ledger_outstandings_request`, as it stands at master 56f7be77.
const TEMPLATE: &str = r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>Ledger Outstandings</ID></HEADER><BODY><DESC><STATICVARIABLES><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><LedgerName>{party}</LedgerName><SVTODATE TYPE="Date">{to}</SVTODATE><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><EXPLODEFLAG>Yes</EXPLODEFLAG></STATICVARIABLES></DESC></BODY></ENVELOPE>"#;
const BUILDER_SOURCE: &str = include_str!("../src/native_outstandings/party_bills.rs");

fn day(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

/// As the build sends a request: a BOM, then UTF-16LE.
fn as_sent(xml: &str) -> Vec<u8> {
    let mut bytes = vec![0xff, 0xfe];
    for unit in xml.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

fn from_sent(bytes: &[u8]) -> String {
    assert_eq!(&bytes[..2], &[0xff, 0xfe]);
    String::from_utf16(
        &bytes[2..]
            .chunks(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The builder's template with any party. The only difference from the builder is that the party is a
/// `&str`, not a `LedgerScopeName`.
fn request_for(party: &str) -> String {
    TEMPLATE
        .replace("{company}", &escape_text(BOOK))
        .replace("{party}", &escape_text(party))
        .replace("{to}", day(TO).as_str())
}

/// The snapshot `fetch_party_bills` sends (`tally/runtime_party_bills.rs:80-85`): the books' first day
/// to `to`, on a licensed gateway, which is `ModeAgnostic`.
fn snapshot(book: &str, from: &str, to: &str) -> Vec<u8> {
    let period =
        NativeLedgerSnapshotPeriod::new(DateBoundaryProfile::ModeAgnostic, day(from), day(to))
            .unwrap();
    as_sent(&render_native_ledger_snapshot_request(book, &period))
}

fn code(error: LedgerScopeNameError) -> &'static str {
    error.code()
}

#[test]
#[ignore = "lab only: writes row 2480-NM's requests"]
fn lab_2480_nm_renders_the_row_requests() {
    let out = std::path::PathBuf::from(std::env::var("LAB_2480_NM_OUT").unwrap());
    let read = |name: &str| std::fs::read(out.join(name)).unwrap();

    // The copy of the template is the builder's.
    assert_eq!(
        BUILDER_SOURCE.matches(TEMPLATE).count(),
        1,
        "the builder's template is not the one copied here"
    );
    // The builders against the files the lab was sent.
    assert_eq!(
        snapshot("BRIDGE FOREXGROUP LAB", "20250401", "20250430"),
        read("snapshot-build-request-forex-20250401-20250430.bin"),
        "the snapshot builder's render differs from the build's request of 29 Oct"
    );
    assert_eq!(
        snapshot(BOOK, BOOKS_FROM, TO),
        read("S-snap-20260401-20260801.xml"),
        "the snapshot builder's render differs from row 2346-PB's request"
    );
    assert_eq!(
        LedgerScopeName::new(PB_PARTY),
        Err(LedgerScopeNameError::Character)
    );
    assert_eq!(
        as_sent(&request_for(PB_PARTY)),
        read("pb/L-lo-bc-20260801.xml"),
        "the copy of the template differs from the request row 2346-PB sent"
    );

    let names: Vec<(String, String, String)> =
        String::from_utf8(std::fs::read(out.join("names.tsv")).unwrap())
            .unwrap()
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                assert_eq!(parts.len(), 3, "{line}");
                (parts[0].into(), parts[1].into(), parts[2].into())
            })
            .collect();
    assert_eq!(names.len(), 11);

    // The control: the one name the type admits. The copy is the builder's own render for it.
    let (control_id, _, control_name) = &names[0];
    assert_eq!(control_id, "pl");
    let admitted = LedgerScopeName::new(control_name).unwrap();
    assert_eq!(admitted.as_str(), control_name);
    let control = render_ledger_outstandings_request(BOOK, &admitted, &day(TO));
    assert_eq!(
        request_for(control_name),
        control,
        "the copy of the template differs from the builder's render for the admitted name"
    );
    assert_eq!(control.matches(control_name.as_str()).count(), 1);

    let mut written = Vec::new();
    for (id, number, name) in &names {
        let verdict = match LedgerScopeName::new(name) {
            Ok(_) => "admitted",
            Err(error) => code(error),
        };
        let expected = match id.as_str() {
            "pl" => "admitted",
            "op" => "ledger_scope_name_formula_word",
            _ => "ledger_scope_name_character",
        };
        assert_eq!(verdict, expected, "{id} {name}");
        let request = request_for(name);
        assert_eq!(request.matches(&escape_text(name)).count(), 1, "{id}");
        assert_eq!(
            request.replace(&escape_text(name), &escape_text(control_name)),
            control,
            "{id}: the request differs from the control's beyond the name"
        );
        let bytes = as_sent(&request);
        assert_eq!(from_sent(&bytes), request);
        let file = format!("L-lo-{id}-{TO}.xml");
        std::fs::write(out.join(&file), &bytes).unwrap();
        written.push(file.clone());
        println!("{id} {number} {name:?}: LedgerScopeName {verdict}; {file} {} bytes", bytes.len());
    }
    assert_eq!(written.len(), 11);
}
