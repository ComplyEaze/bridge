//! The one-ledger dialog's text: exact for each admitted kind, unsafe text
//! refused, and every limit a refusal, never a cut.
use super::*;
use crate::tally::ledger_create_item::{
    AdmittedDutyHead, AdmittedLedgerKind, MasterName, PartySide, VerifiedGstin,
};
use bridge_tally_protocol::GstDutyHead;

/// The preview of a batch of `item` alone: the one-ledger text, through the
/// batch's one builder.
fn master_review_text(
    item: &LedgerCreateItem,
    company: &VerifiedCompanyIdentity,
    origin: &str,
) -> Result<String, MasterReviewError> {
    super::master_review_text(&LedgerCreateBatch::one(item.clone()), company, origin)
}

/// The masked preview of a batch of `item` alone.
fn masked_review_text(
    item: &LedgerCreateItem,
    company: &VerifiedCompanyIdentity,
    origin: &str,
) -> Result<String, MasterReviewError> {
    super::masked_review_text(&LedgerCreateBatch::one(item.clone()), company, origin)
}

fn guid() -> &'static str {
    crate::synthetic_guid(1)
}
const ORIGIN: &str = "http://127.0.0.1:9000";
const GSTIN: &str = "08ZZZZZ0000Z2ZP";

fn company(name: &str) -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture(name, guid())
}

fn item(name: &str, kind: AdmittedLedgerKind) -> LedgerCreateItem {
    LedgerCreateItem::new(MasterName::new(name).unwrap(), kind)
}

fn party(side: PartySide, gstin: Option<&str>, bill_wise: bool) -> AdmittedLedgerKind {
    AdmittedLedgerKind::Party {
        side,
        gstin: gstin.map(|value| VerifiedGstin::new(value).unwrap()),
        bill_wise,
    }
}

fn duty() -> AdmittedLedgerKind {
    AdmittedLedgerKind::GstDuty {
        head: AdmittedDutyHead::new(GstDutyHead::SgstUtgst).unwrap(),
    }
}

fn destination() -> String {
    format!(
        "Create ONE ledger in \"BRIDGE MASTERS LAB\"\nCompany GUID: {}\nCompany number: 1  Books from: 1-Apr-2026\nTally: http://127.0.0.1:9000\n\n",
        guid()
    )
}
const WARNING: &str = "\n\nDo not create or rename ledgers in Tally until this is done.";

fn text(item: &LedgerCreateItem) -> Result<String, MasterReviewError> {
    master_review_text(item, &company("BRIDGE MASTERS LAB"), ORIGIN)
}

/// The text is exact for each of the three admitted kinds, one master on two
/// lines under the pinned company, ending with the warning line.
#[test]
fn the_preview_is_exact_for_each_admitted_kind() {
    let cases = [
        (
            "a party with a GSTIN",
            item(
                "MLab M1 Party",
                party(PartySide::SundryDebtors, Some(GSTIN), true),
            ),
            "Create ledger \"MLab M1 Party\" under Sundry Debtors\nGSTIN 08ZZZZZ0000Z2ZP, bill-by-bill Yes",
        ),
        (
            "a party without a GSTIN",
            item(
                "MLab M1 Creditor",
                party(PartySide::SundryCreditors, None, false),
            ),
            "Create ledger \"MLab M1 Creditor\" under Sundry Creditors\nno GST, bill-by-bill No",
        ),
        (
            "a suspense ledger",
            item("MLab M1 Suspense", AdmittedLedgerKind::Suspense),
            "Create ledger \"MLab M1 Suspense\" under Suspense A/c\nno GST, bill-by-bill No",
        ),
        (
            "a GST duty ledger",
            item("MLab M1 Duty SGST UTGST", duty()),
            "Create ledger \"MLab M1 Duty SGST UTGST\" under Duties & Taxes\nduty head SGST/UTGST, bill-by-bill No",
        ),
    ];
    for (case, item, master) in cases {
        assert_eq!(
            text(&item).unwrap(),
            format!("{}{master}{WARNING}", destination()),
            "{case}"
        );
    }
}

/// The text is within the dialog's limits as built, and names the company as
/// a quoted string so a name holding a quote cannot pose as another line.
#[test]
fn the_preview_quotes_every_name_it_shows() {
    let quoted = text(&item("Say \"hi\" \\ there", AdmittedLedgerKind::Suspense)).unwrap();
    assert!(quoted.contains("Create ledger \"Say \\\"hi\\\" \\\\ there\" under Suspense A/c"));
    let company = master_review_text(
        &item("Plain", AdmittedLedgerKind::Suspense),
        &company("Lab \"X\""),
        ORIGIN,
    )
    .unwrap();
    assert!(company.starts_with("Create ONE ledger in \"Lab \\\"X\\\"\"\n"));
}

/// Text the dialog cannot show faithfully is refused before a dialog exists:
/// a line break or control character, and a character that hides a difference
/// between exact names.
#[test]
fn unsafe_text_is_refused_with_its_typed_error() {
    let plain = item("Plain", AdmittedLedgerKind::Suspense);
    for (case, name) in [
        ("a newline in the company", "Lab\nX"),
        ("a carriage return", "Lab\rX"),
        ("a tab", "Lab\tX"),
        ("a line separator", "Lab\u{2028}X"),
        ("a paragraph separator", "Lab\u{2029}X"),
        ("a NUL", "Lab\0X"),
    ] {
        assert_eq!(
            master_review_text(&plain, &company(name), ORIGIN),
            Err(MasterReviewError::LayoutText),
            "{case}"
        );
    }
    for (case, name) in [
        ("a zero-width space", "Lab\u{200b}X"),
        ("a right-to-left override", "Lab\u{202e}X"),
        ("a soft hyphen", "Lab\u{ad}X"),
        ("a byte order mark", "Lab\u{feff}X"),
    ] {
        assert_eq!(
            master_review_text(&plain, &company(name), ORIGIN),
            Err(MasterReviewError::FormatText),
            "{case}"
        );
    }
    // The origin is checked as well.
    assert_eq!(
        master_review_text(
            &plain,
            &company("Lab"),
            "http://127.0.0.1:9000\nTally: other"
        ),
        Err(MasterReviewError::LayoutText)
    );
}

/// A line over the per-line limit is refused, not cut: a long item name, and a
/// long company name.
#[test]
fn a_line_over_the_limit_is_refused_never_cut() {
    let fixed = "Create ledger \"\" under Suspense A/c".chars().count();
    let fits = "n".repeat(BATCH_REVIEW_MAX_LINE_CHARS - fixed);
    let over = "n".repeat(BATCH_REVIEW_MAX_LINE_CHARS - fixed + 1);
    let shown = text(&item(&fits, AdmittedLedgerKind::Suspense)).unwrap();
    assert!(shown.contains(&format!("Create ledger \"{fits}\" under Suspense A/c\n")));
    assert_eq!(
        text(&item(&over, AdmittedLedgerKind::Suspense)),
        Err(MasterReviewError::TooLarge)
    );
    let fixed = "Create ONE ledger in \"\"".chars().count();
    let long_company = "c".repeat(BATCH_REVIEW_MAX_LINE_CHARS - fixed);
    assert!(master_review_text(
        &item("Plain", AdmittedLedgerKind::Suspense),
        &company(&long_company),
        ORIGIN
    )
    .is_ok());
    let longer_company = "c".repeat(BATCH_REVIEW_MAX_LINE_CHARS - fixed + 1);
    assert_eq!(
        master_review_text(
            &item("Plain", AdmittedLedgerKind::Suspense),
            &company(&longer_company),
            ORIGIN
        ),
        Err(MasterReviewError::TooLarge)
    );
}

/// Each of the four limits refuses on its own, at its boundary: the builder's
/// own text reaches only the per-line limit, so the others are held on the
/// shared check.
#[test]
fn each_limit_refuses_at_its_boundary() {
    let lines = |count: usize, width: usize| vec!["x".repeat(width); count];
    assert!(fits(&lines(BATCH_REVIEW_MAX_LINES, 1)));
    assert!(!fits(&lines(BATCH_REVIEW_MAX_LINES + 1, 1)));
    assert!(fits(&lines(1, BATCH_REVIEW_MAX_LINE_CHARS)));
    assert!(!fits(&lines(1, BATCH_REVIEW_MAX_LINE_CHARS + 1)));
    // Total characters: 32 lines of 99 and 31 separators is 3,199; 33 is 3,299.
    let wide = lines(32, 99);
    assert_eq!(wide.join("\n").chars().count(), 32 * 99 + 31);
    assert!(fits(&wide));
    assert!(!fits(&lines(33, 99)));
    // Bytes: multibyte characters cost more bytes than characters.
    let multibyte = vec!["\u{20b9}".repeat(BATCH_REVIEW_MAX_LINE_CHARS); 25];
    let joined = multibyte.join("\n");
    assert!(joined.chars().count() <= BATCH_REVIEW_MAX_CHARS);
    assert!(joined.len() > BATCH_REVIEW_MAX_BYTES);
    assert!(!fits(&multibyte));
    // The limits are the voucher preview's own.
    assert_eq!(BATCH_REVIEW_MAX_LINES, 40);
    assert_eq!(BATCH_REVIEW_MAX_CHARS, 3_200);
    assert_eq!(BATCH_REVIEW_MAX_LINE_CHARS, 100);
    assert_eq!(BATCH_REVIEW_MAX_BYTES, 7_000);
}

/// The masked preview a tool result carries is refused exactly when the
/// dialog's own text is: a name one character too long for its line is
/// refused although its mask would fit, and the longest name that fits is
/// returned masked.
#[test]
fn the_masked_preview_is_refused_whenever_the_dialogs_own_text_is() {
    let fixed = "Create ledger \"\" under Suspense A/c".chars().count();
    let lab = company("BRIDGE MASTERS LAB");
    let fits = item(
        &"n".repeat(BATCH_REVIEW_MAX_LINE_CHARS - fixed),
        AdmittedLedgerKind::Suspense,
    );
    let masked = masked_review_text(&fits, &lab, ORIGIN).unwrap();
    assert!(masked.contains("Create ledger \"nn…nn\" under Suspense A/c\n"));
    let over = item(
        &"n".repeat(BATCH_REVIEW_MAX_LINE_CHARS - fixed + 1),
        AdmittedLedgerKind::Suspense,
    );
    assert_eq!(text(&over), Err(MasterReviewError::TooLarge));
    assert_eq!(
        masked_review_text(&over, &lab, ORIGIN),
        Err(MasterReviewError::TooLarge)
    );
}

/// The ladder: each refusal's code. `master_review_too_large` is the new rung;
/// the other two are the voucher preview's own codes.
#[test]
fn each_refusal_has_its_code() {
    assert_eq!(
        MasterReviewError::TooLarge.code(),
        "master_review_too_large"
    );
    assert_eq!(
        MasterReviewError::LayoutText.code(),
        "import_review_layout_text"
    );
    assert_eq!(
        MasterReviewError::FormatText.code(),
        "import_review_format_text"
    );
}

/// The built text is also within the real dialog's byte limit, so a text this
/// builder returns is never refused by the dialog.
#[test]
fn every_built_text_fits_the_dialog() {
    let fixed = "Create ledger \"\" under Suspense A/c".chars().count();
    let longest = item(
        &"n".repeat(BATCH_REVIEW_MAX_LINE_CHARS - fixed),
        AdmittedLedgerKind::Suspense,
    );
    let shown = text(&longest).unwrap();
    assert!(shown.len() <= BATCH_REVIEW_MAX_BYTES);
    assert!(shown.lines().count() <= BATCH_REVIEW_MAX_LINES);
}

/// A batch names its count on the first line and lists each ledger, two lines
/// each, in the order it is sent; the masked text masks every name and is
/// refused exactly when the whole text is.
#[test]
fn a_batch_preview_names_its_count_and_lists_each_ledger_in_order() {
    let batch = LedgerCreateBatch::new(vec![
        item(
            "MLab Batch One",
            party(PartySide::SundryCreditors, None, false),
        ),
        item(
            "MLab Batch Two",
            party(PartySide::SundryCreditors, None, true),
        ),
    ])
    .unwrap();
    let lab = company("BRIDGE MASTERS LAB");
    let text = super::master_review_text(&batch, &lab, ORIGIN).unwrap();
    let one = destination();
    let expected = format!(
        "{}Create ledger \"MLab Batch One\" under Sundry Creditors\nno GST, bill-by-bill No\nCreate ledger \"MLab Batch Two\" under Sundry Creditors\nno GST, bill-by-bill Yes{WARNING}",
        one.replacen("Create ONE ledger in", "Create 2 ledgers in", 1)
    );
    assert_eq!(text, expected);
    let masked = super::masked_review_text(&batch, &lab, ORIGIN).unwrap();
    assert!(!masked.contains("MLab Batch One") && !masked.contains("MLab Batch Two"));
    assert_eq!(masked.lines().count(), text.lines().count());
}

/// A batch at the limit fits the box with names of a working length.
#[test]
fn a_batch_at_the_limit_fits_the_box() {
    let batch = LedgerCreateBatch::new(
        (0..crate::tally::ledger_create_item::MAX_LEDGERS_PER_BATCH)
            .map(|index| {
                item(
                    &format!("MLab Batch Creditor With A Longer Name {index:02}"),
                    party(PartySide::SundryCreditors, Some(GSTIN), true),
                )
            })
            .collect(),
    )
    .unwrap();
    assert!(super::master_review_text(&batch, &company("BRIDGE MASTERS LAB"), ORIGIN).is_ok());
}
