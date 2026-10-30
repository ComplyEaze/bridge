//! A Sales invoice with an HSN/SAC warning, built and then posted-and-declined
//! or verified, on the pilot lab's own answers from the W18, W18b and W18c sittings
//! (fixtures/agent/pilot-lab-w18/PROVENANCE.md). The ledger `BRIDGE Svc 4D 5%`
//! carries the four-digit code 9983, which `hsn_warning_not_six_digits` names.
//!
//! Every answer is Tally's own bytes, taken from one book state (the marks of
//! W18 and of the books W18b and W18c restored are the same), except three,
//! which are DERIVED and say so where they are built: the rates answer `Q` (the
//! code's request names this invoice's six ledgers by GUID and was never sent to
//! the lab; the answer is the lab's 21 Oct 2026 answer to the request restricted
//! to their parents, cut to the six: `w18_scoped_rates_hsn`) and the two voucher-numbering
//! answers `X` and `Y` (the lab was not asked them on this book: the 16 Oct
//! answers with the rows above this book's mark removed). The order below is the
//! order the build, the post's reads before its dialog and a verification each
//! asked in, as observed against a replay of those answers; it is not an order
//! read off the code. Each position is checked against the SHA-256 of the
//! request the lab answered. Legend as `PILOT_ORDER` in the parent module,
//! except that `N` is the number read for `BP/26-27/0040` and `w` is W18b's
//! verification window of 1 Aug 2026, which holds the eleven vouchers of the
//! book.
use super::*;

/// The build: 122 requests, no write.
const W18_BUILD: &str = concat!(
    "seesesekskseemsmseebsbscscsbsbseseebsbsLsLsPsPsgsgsbsbseseeQsQseeT",
    "sTseeNsNseeCsCseeOsOseemsmseseeXsXsYsYseseekskseewswsese",
);

/// The post's reads before its dialog: the verification readback, then the
/// admission's reads again. 140 requests.
const W18_POST_BEFORE_DIALOG: &str = concat!(
    "seesesemsmseewswseewswseseesesekskseemsmseebsbscscsbsbseseebsbsLsLsPsPsgsgsbsbsese",
    "eQsQseeTsTseeNsNseeCsCseeOsOseecscseseemsmseseeXsXsYsYsese",
);

/// A verification of the saved batch, no post: 26 requests.
const W18_VERIFY: &str = "seesesemsmseewswseewswsese";

const W18_GUID: &str = PILOT_GUID;

/// The SHA-256 of the request the build sends for each kind of read. Each is
/// a request the lab answered, as in the 9 Oct replay, except: the number
/// read, which is `w18-number-0040.xml` (`../r09-number-new-invoice.xml` with
/// the number changed only); the two listings named below; `Q`, the rates
/// request by GUID, which is the code's and was never sent with these GUIDs (a
/// test derives its pin from the lab's 31 Oct 2026 request naming five other
/// ledgers; see `w18_scoped_rates_hsn`); and `X` and `Y`, which are the code's requests the
/// lab answered on 16 Oct 2026 on a later state of the book (see
/// `w18_types_as_of_the_book`).
fn w18_request_sha256(letter: char) -> &'static str {
    match letter {
        'N' => "84f6b7228f3e8c94d028c302c289ed7f99d9076eb45b4b0676adcbc67dc497d8",
        // Not `r05` and `r06` as W18 sent them (SVTODATE 20260801): on W18's
        // book the last voucher date is 20260915, so the build asks the
        // compliance listing with SVTODATE 20260915 and the paired listing
        // with 20261001. W18c sent those two requests to Tally and these are
        // its answers (the paired one differs from W18's: see
        // fixtures/agent/pilot-lab-w18/PROVENANCE.md).
        'L' => "b0c02fc724502e10dc23a9d6b430c9cbb20c191df948082f06fab9d7e4c8f9bc",
        'P' => "41b2de3ed4488e5943d25fe36b2c2727f397ca01bae4082d7af36f787bb4b4b5",
        // The rates read names the invoice's ledgers by GUID (#1331), and this
        // invoice names `BRIDGE Svc 4D 5%` where `BP/26-27/0010` names
        // `BRIDGE Svc 998313 5%`: the request the lab answered on 31 Oct 2026
        // (`pilot-lab-ledger-rates-guid.request`) with only its GUIDs changed to
        // these six ledgers' (DERIVED: not sent with these GUIDs).
        'Q' => super::super::super::tests::W18_INVOICE_GUID_RATES_SHA256,
        other => pilot_request_sha256(other),
    }
}

/// The master mark of the W18 book (`ALTMSTID` of `w18-marks`).
const W18_MASTER_MARK: u64 = 243;

/// DERIVED, not Tally's bytes: a numbering answer of the pilot lab (read on
/// 16 Oct 2026, at master mark 245) as of the W18 book (mark 243). The code
/// refuses a numbering read with a voucher type newer than the mark it is
/// bounded by, so the answer is served with the rows above this book's mark
/// removed: every removed row must carry an `ALTERID` above the mark and the
/// removed names must be exactly `BRIDGE Sales Two`, and the rows left must
/// equal the W18 voucher-types capture (name, AlterID, GUID). The files are
/// the answers whole, as received.
fn w18_types_as_of_the_book(answer: &str) -> String {
    const ROW: &str = "<VOUCHERTYPE NAME=\"";
    let mut kept = String::new();
    let mut removed = Vec::new();
    let mut rest = answer;
    while let Some(start) = rest.find(ROW) {
        let end = start + rest[start..].find("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
        let (name, alter, _) = pilot_type_rows(&rest[start..end]).remove(0);
        if alter > W18_MASTER_MARK {
            removed.push(name);
            kept.push_str(&rest[..start]);
        } else {
            kept.push_str(&rest[..end]);
        }
        rest = &rest[end..];
    }
    kept.push_str(rest);
    assert_eq!(removed, ["BRIDGE Sales Two"], "{removed:?}");
    let mut left = pilot_type_rows(&kept);
    let mut book = pilot_type_rows(&captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-voucher-types.utf16le.xml"
    )));
    left.sort();
    book.sort();
    assert_eq!(left, book, "the rows left are not the W18 voucher types");
    kept
}

/// Tally's answer to one kind of read, from the W18 sittings, except the
/// three DERIVED ones named below (`Q`, `X`, `Y`). The company list and the
/// currencies are byte-equal to the 9 Oct answers the earlier replay already
/// holds, and are read from there; so is the unrestricted rates listing (12 Oct
/// 2026) that `Q` is cut from.
fn w18_plan(letter: char) -> ScenarioPlan {
    let bytes: &[u8] = match letter {
        's' => return status(),
        'e' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-company-extent.utf16le.xml"),
        'c' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-currencies.utf16le.xml"),
        // DERIVED: see `w18_scoped_rates_hsn`.
        'Q' => return pilot_answer(super::super::super::tests::w18_scoped_rates_hsn()),
        'm' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-marks.utf16le.xml"),
        'b' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-book-extent.utf16le.xml"),
        'L' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-ledgers-compliance.utf16le.xml"),
        'P' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-ledgers-paired.utf16le.xml"),
        'g' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-groups.utf16le.xml"),
        'T' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-voucher-types.utf16le.xml"),
        'N' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-number-0040.utf16le.xml"),
        'C' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-tax-units.utf16le.xml"),
        'O' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-company-object.utf16le.xml"),
        'k' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-ledger-catalogue-v2.utf16le.xml"),
        // DERIVED: the 16 Oct 2026 numbering answers (pilot-lab-numbering-*)
        // with the rows above this book's master mark removed.
        'X' | 'Y' => {
            let bytes: &[u8] = if letter == 'X' {
                include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-numbering-masters.utf16le.xml")
            } else {
                include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-numbering-series.utf16le.xml")
            };
            return pilot_answer(w18_types_as_of_the_book(&captured(bytes)));
        }
        'w' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18b-verification-window.utf16le.xml"),
        other => panic!("unknown kind {other}"),
    };
    ScenarioPlan::new(Fixture::SyntheticXml(captured(bytes)))
        .with_encoding(WireEncoding::Utf16LeNoBom)
        .with_framing(ResponseFraming::ContentLength)
}

/// `BP/26-27/0040` on 1 Aug 2026: two sales lines, the first on the ledger
/// whose code is four digits (so one warning), the second on a six-digit code
/// (none). 5 percent in all: 25.02 a head on 1,000.80, and 0.16 of rounding.
fn w18_args() -> Value {
    json!({"company_guid":W18_GUID, "vouchers":[{
    "bridge_txn_id":"t1", "date":"2026-08-01", "voucher_type":"Sales", "voucher_number":"BP/26-27/0040",
    "invoice":{"voucher_type_name":"BRIDGE Sales","place_of_supply":"Rajasthan","round_off_ledger":"BRIDGE Round Off"},
    "entries":[
        {"ledger":"BRIDGE Walk-in","amount":"1051.00","side":"Dr"},
        {"ledger":"BRIDGE Svc 4D 5%","amount":"600.40","side":"Cr"},
        {"ledger":"BRIDGE Svc 998314 5%","amount":"400.40","side":"Cr"},
        {"ledger":"BRIDGE CGST 2.5%","amount":"25.02","side":"Cr"},
        {"ledger":"BRIDGE SGST 2.5%","amount":"25.02","side":"Cr"},
        {"ledger":"BRIDGE Round Off","amount":"0.16","side":"Cr"}
    ]}]})
}

/// A server over a simulator that answers `order` (and one more, so a request
/// too many is counted).
fn w18_lab(order: &str) -> (SequenceSimulator, Server, tempfile::TempDir) {
    let simulator =
        SequenceSimulator::spawn(with_sentinel(order.chars().map(w18_plan).collect())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    (simulator, server, directory)
}

/// The kind of each request of the run, as in `order`: `s` for a status
/// probe, the letter of the read whose request it is by its SHA-256, and `?`
/// for a request that is none of them.
fn w18_kinds(observed: &[tally_protocol_simulator::ObservedRequest]) -> String {
    observed
        .iter()
        .map(|request| {
            if (request.method.as_str(), request.path.as_str()) == ("GET", "/status") {
                's'
            } else {
                "ebcLQOPgTNkwCmXY"
                    .chars()
                    .find(|letter| w18_request_sha256(*letter) == request.request_body_sha256)
                    .unwrap_or('?')
            }
        })
        .collect()
}

/// Every request of the run was, at its position, the request named in
/// `w18_request_sha256` (the lab's own for every kind but `Q`, whose request
/// is the code's, and `X` and `Y`, whose answers the lab gave on a later state
/// of the book), and no more were made.
fn assert_asked_as_the_lab_was(
    order: &str,
    observed: &[tally_protocol_simulator::ObservedRequest],
) {
    assert_eq!(w18_kinds(observed), order);
}

/// The ledger name as the build's payload and the saved proof spell it.
fn marked_ledger() -> Value {
    json!({"$bridge_agent_party_name": "BRIDGE Svc 4D 5%"})
}

/// The one warning the invoice draws: the four-digit code on the ledger, said
/// to be six-digit-preferred, named by where the code came from.
///
/// `ledger` is how the document under test spells the ledger name: marked as
/// a party name in the build's payload and in the saved proof (where the
/// configured redaction applies), plain in a tool response served under no
/// redaction.
fn assert_the_four_digit_warning(warnings: &Value, ledger: &Value, whole: &Value) {
    let listed = warnings.as_array().expect("hsn_warnings is a list");
    assert_eq!(listed.len(), 1, "{whole}");
    assert_eq!(listed[0]["code"], "hsn_warning_not_six_digits", "{whole}");
    assert_eq!(&listed[0]["ledger"], ledger, "{whole}");
    assert_eq!(listed[0]["characters"], 4, "{whole}");
    assert!(
        !listed[0].to_string().contains("9983"),
        "the code itself is not listed: {whole}"
    );
}

/// The build, on W18's answers: the invoice is admitted with the four-digit
/// code on `BRIDGE Svc 4D 5%` warned about, never refused.
async fn w18_build(server: &Server) -> (Value, String) {
    let built = built_or_panic(server.build_import_xml(&w18_args()).await);
    let result = &built.payload["result"];
    assert!(result["batch_id"].is_string(), "{}", built.payload);
    assert_ne!(result["state"], "refused", "{}", built.payload);
    let batch_id = result["batch_id"].as_str().unwrap().to_string();
    (built.payload, batch_id)
}

/// #691: the build reports the warning on the four-digit ledger, and the
/// dialog the post then opens says one sales ledger has a code to check. The
/// person declines; the proof the post's readback saved names the same
/// warning. The saved invoice observation holds the warning, so the build
/// response, the dialog and the proof each lose it if the build does not pass
/// the warnings into what it saves.
#[tokio::test]
async fn a_four_digit_code_is_warned_in_the_build_the_dialog_and_the_proof_on_the_pilot_labs_w18_answers(
) {
    let order = format!("{W18_BUILD}{W18_POST_BEFORE_DIALOG}");
    let (simulator, server, _directory) = w18_lab(&order);
    let scripted = ScriptedApproval::declining();
    let (built, declined, batch_id) = TEST_QUALIFIED_VOUCHER_TYPES
        .scope(PILOT_QUALIFIED, async {
            let (built, batch_id) = w18_build(&server).await;
            let declined = SCRIPTED_APPROVAL
                .scope(
                    scripted.clone(),
                    server.call_tool(
                        "post_import",
                        json!({"company_guid":W18_GUID,"batch_id":batch_id}),
                    ),
                )
                .await;
            (built, declined, batch_id)
        })
        .await;
    let observed = sent(simulator);
    assert_asked_as_the_lab_was(&order, &observed);

    // The build's response.
    assert_the_four_digit_warning(&built["result"]["hsn_warnings"], &marked_ledger(), &built);
    assert!(
        built["result"]["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|warning| warning
                .as_str()
                .is_some_and(|text| text.starts_with("HSN/SAC to check."))),
        "the plain warning: {built}"
    );

    // The dialog: one sales ledger to check, the usual checked line gone.
    let previews = scripted.previews();
    assert_eq!(previews.len(), 1, "{declined}");
    assert!(
        previews[0].contains(&invoice_checked_hsn_line(1)),
        "{}",
        previews[0]
    );
    assert!(
        !previews[0].contains("CGST and state tax per line at the ledger's rate"),
        "{}",
        previews[0]
    );

    // Declined, nothing was sent past the reads before the dialog; the
    // readback it made saved a proof, which carries the warning.
    let result = &declined["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_approval_declined",
        "{declined}"
    );
    assert_ne!(result["attempt_recorded"], json!(true), "{declined}");
    let proof: Value =
        serde_json::from_slice(&fs::read(&server.current_proof_paths(&batch_id)[0]).unwrap())
            .unwrap();
    assert_the_four_digit_warning(&proof["hsn_warnings"], &marked_ledger(), &proof);
    assert_eq!(proof["verification_status"], "verification_incomplete");
}

/// #691: a verification of the built batch, with no post, carries the
/// warning the batch was approved with, whatever the verdict. `hsn_warnings`
/// is added to every verification result from the saved batch; here the batch
/// was never posted, so the verdict is that nothing is verified.
#[tokio::test]
async fn a_verification_of_the_four_digit_invoice_carries_its_warning_on_the_pilot_labs_w18_answers(
) {
    let order = format!("{W18_BUILD}{W18_VERIFY}");
    let (simulator, server, _directory) = w18_lab(&order);
    let (built, verified) = TEST_QUALIFIED_VOUCHER_TYPES
        .scope(PILOT_QUALIFIED, async {
            let (built, batch_id) = w18_build(&server).await;
            let verified = server
                .call_tool(
                    "verify_import",
                    json!({"company_guid":W18_GUID,"batch_id":batch_id}),
                )
                .await;
            (built, verified)
        })
        .await;
    let observed = sent(simulator);
    assert_asked_as_the_lab_was(&order, &observed);
    assert_the_four_digit_warning(&built["result"]["hsn_warnings"], &marked_ledger(), &built);
    let result = &verified["structuredContent"]["result"];
    assert_the_four_digit_warning(
        &result["hsn_warnings"],
        &json!("BRIDGE Svc 4D 5%"),
        &verified,
    );
    assert_ne!(
        result["verification_status"], "posted_verified",
        "{verified}"
    );
}
