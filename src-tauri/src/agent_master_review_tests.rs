//! The one-ledger dialog's text: exact for each admitted kind, unsafe text
//! refused, and every limit a refusal, never a cut.
use super::super::approval::{ApprovalBinding, Begin, HeldRequest, Joined, PostApprovals};
use super::super::master_post::MasterPostRequest;
use super::*;
use crate::agent::failure_code::{stock_item_outcome_code, stock_item_read_back_code};
use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};
use crate::tally::approved_import::{Answered, ApprovedImport, PostKind, StockItemDialogRefusal};
use crate::tally::ledger_create_item::{
    AdmittedDutyHead, AdmittedLedgerKind, PartySide, VerifiedGstin,
};
use crate::tally::master_name::MasterName;
use crate::tally::stock_item_create::{
    render_stock_item_create, CreateRenderRefusal, DraftText, GoodsSibling, SiblingGst,
    StockItemCatalogue, StockItemDraft,
};
use crate::tally::stock_item_verified::{ReadBackVerdict, StockItemImportOutcome};
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_import_outcome, ExpectedTallyTextEncoding,
    GstDutyHead,
};

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
    // A company name with a control character (newline, carriage return,
    // tab, NUL) can no longer be held by an identity at all (#2397); the
    // separators below are not control characters, so the review still
    // refuses them itself.
    for (case, name) in [
        ("a line separator", "Lab\u{2028}X"),
        ("a paragraph separator", "Lab\u{2029}X"),
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

const STOCK_COMPANY: &str = "BRIDGE STOCK CREATE LAB";
const STOCK_COMPANY_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1";
const STOCK_BEFORE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-before.utf16le.xml"
);
const STOCK_CREATE_ANSWER: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-create-answer.utf16le.xml"
);
const STOCK_AFTER_MARKED: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-after-marked-create.utf16le.xml"
);
const STOCK_ALIAS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-with-alias.utf16le.xml"
);
const STOCK_GROUPS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-groups.utf16le.xml"
);
const STOCK_UNITS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-units.utf16le.xml"
);

const STOCK_MARKED_REQUEST: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-marked-create.request.utf16le.xml"
);

const STOCK_ITEM_PREVIEW: &str = concat!(
    "Create ONE stock item in \"BRIDGE STOCK CREATE LAB\"\n",
    "Company GUID: bac2b5e9-ada9-43c0-b8f8-a2812a813fa1\n",
    "Company number: 1  Books from: 1-Apr-2026\n",
    "Tally: http://127.0.0.1:9000\n",
    "\n",
    "Create stock item \"SC Copied Marked\"\n",
    "All six copied fields come from sibling stock item \"SC Sibling Goods\".\n",
    "Parent (stock group): \"SC Goods Group\" — source: \"SC Sibling Goods\"\n",
    "Base unit: \"Nos\" — source: \"SC Sibling Goods\"\n",
    "GST applicability: \"&#4; Applicable\" — source: \"SC Sibling Goods\"\n",
    "Type of supply: \"Goods\" — source: \"SC Sibling Goods\"\n",
    "GST details entry 1 — source: \"SC Sibling Goods\"\n",
    "  Applicable from: \"20250401\"\n",
    "  Taxability: \"Taxable\"\n",
    "  Source of GST details: \"Specify Details Here\"\n",
    "  State row 1: \"&#4; Any\"\n",
    "  Rate row 1: \"CGST\"; valuation: \"Based on Value\"; rate: \"9\"\n",
    "  Rate row 2: \"SGST/UTGST\"; valuation: \"Based on Value\"; rate: \"9\"\n",
    "  Rate row 3: \"IGST\"; valuation: \"Based on Value\"; rate: \"18\"\n",
    "  Rate row 4: \"Cess\"; valuation: \"&#4; Not Applicable\"; rate: absent\n",
    "  Rate row 5: \"State Cess\"; valuation: \"Based on Value\"; rate: absent\n",
    "  GST slab rates: empty\n",
    "HSN details entry 1 — source: \"SC Sibling Goods\"\n",
    "  Applicable from: \"20250401\"\n",
    "  HSN code: \"39239090\"\n",
    "  HSN description: \"Plastic Articles for Conveyance\"\n",
    "  Source of HSN details: \"Specify Details Here\"\n",
    "\n",
    "Do not create or rename stock items in Tally until this is done."
);

fn stock_decode(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("the capture decodes")
    .text
}

fn stock_catalogue(company_name: &str, xml: &str) -> StockItemCatalogue {
    StockItemCatalogue::from_candidate_read(
        &VerifiedCompanyIdentity::test_fixture(company_name, STOCK_COMPANY_GUID),
        xml,
    )
    .expect("the capture catalogues")
}

fn stock_draft<'c>(
    catalogue: &'c StockItemCatalogue,
    new_name: &str,
    source_name: &str,
) -> StockItemDraft<'c> {
    let absent = catalogue
        .name_absent(&MasterName::new(new_name).unwrap())
        .expect("the proposed name is absent");
    let sibling = SiblingGst::named(
        catalogue,
        &MasterName::new(source_name).expect("the source name is admitted"),
    )
    .expect("the sibling is complete");
    let sibling = GoodsSibling::from_sibling(sibling).expect("the sibling is the Goods case");
    StockItemDraft::new(
        absent,
        sibling,
        &stock_decode(STOCK_GROUPS),
        &stock_decode(STOCK_UNITS),
    )
    .expect("the sibling drafts")
}

fn stock_preview_with(
    company_name: &str,
    items: &[u8],
    new_name: &str,
    source_name: &str,
    origin: &str,
) -> Result<String, StockItemReviewError> {
    let xml = stock_decode(items);
    let catalogue = stock_catalogue(company_name, &xml);
    let draft = stock_draft(&catalogue, new_name, source_name);
    stock_item_review_text(&draft, origin)
}

fn stock_preview(items: &[u8], new_name: &str, source_name: &str) -> String {
    stock_preview_with(STOCK_COMPANY, items, new_name, source_name, ORIGIN)
        .expect("the captured draft has a review")
}

#[test]
fn the_captured_stock_item_preview_is_exact() {
    assert_eq!(
        stock_preview(STOCK_BEFORE, "SC Copied Marked", "SC Sibling Goods"),
        STOCK_ITEM_PREVIEW
    );
}

#[test]
fn absent_cess_rates_are_shown_as_absent_not_zero() {
    let preview = stock_preview(STOCK_BEFORE, "SC Copied Marked", "SC Sibling Goods");
    let cess_lines = preview
        .lines()
        .filter(|line| line.contains("\"Cess\"") || line.contains("\"State Cess\""))
        .collect::<Vec<_>>();
    assert_eq!(
        cess_lines,
        [
            "  Rate row 4: \"Cess\"; valuation: \"&#4; Not Applicable\"; rate: absent",
            "  Rate row 5: \"State Cess\"; valuation: \"Based on Value\"; rate: absent",
        ]
    );
    for line in cess_lines {
        assert!(line.ends_with("rate: absent"));
        assert!(!line.contains("rate: \"0\""));
        assert!(!line.contains("rate: \"0.0\""));
    }
}

#[test]
fn marked_values_are_shown_as_request_text_not_parser_markers() {
    let preview = stock_preview(STOCK_BEFORE, "SC Copied Marked", "SC Sibling Goods");
    assert_eq!(preview.matches("&#4;").count(), 3);
    assert!(!preview.contains('\u{fffd}'));
    for value in [
        "\"&#4; Applicable\"",
        "\"&#4; Any\"",
        "\"&#4; Not Applicable\"",
    ] {
        assert!(preview.contains(value));
    }
    for plain in ["\" Applicable\"", "\" Any\"", "\" Not Applicable\""] {
        assert!(!preview.contains(plain));
    }
}

#[test]
fn stock_item_review_refuses_unsafe_company_and_origin_text() {
    for (company_name, refusal) in [
        ("BRIDGE\u{2028}STOCK LAB", MasterReviewError::LayoutText),
        ("BRIDGE\u{200b}STOCK LAB", MasterReviewError::FormatText),
    ] {
        assert_eq!(
            stock_preview_with(
                company_name,
                STOCK_BEFORE,
                "SC Copied Marked",
                "SC Sibling Goods",
                ORIGIN,
            ),
            Err(StockItemReviewError::Review(refusal))
        );
    }
    assert_eq!(
        stock_preview_with(
            STOCK_COMPANY,
            STOCK_BEFORE,
            "SC Copied Marked",
            "SC Sibling Goods",
            "http://127.0.0.1:9000\nTally: other",
        ),
        Err(StockItemReviewError::Review(MasterReviewError::LayoutText))
    );
}

#[test]
fn stock_item_review_refuses_an_overlong_field_instead_of_cutting_it() {
    // Hand edit of the captured read: only the HSN description changes.
    let xml = stock_decode(STOCK_BEFORE);
    let description = "Plastic Articles for Conveyance";
    assert_eq!(xml.matches(description).count(), 1);
    let xml = xml.replace(description, &"x".repeat(101));
    let catalogue = stock_catalogue(STOCK_COMPANY, &xml);
    let draft = stock_draft(&catalogue, "SC Copied Marked", "SC Sibling Goods");
    assert_eq!(
        stock_item_review_text(&draft, ORIGIN),
        Err(StockItemReviewError::Review(MasterReviewError::TooLarge))
    );
}

#[test]
fn an_alias_source_is_named_exactly_as_approved() {
    let preview = stock_preview(STOCK_ALIAS, "SC Copied From Alias", "SC Sibling Alias");
    let expected = STOCK_ITEM_PREVIEW
        .replace("SC Copied Marked", "SC Copied From Alias")
        .replace("SC Sibling Goods", "SC Sibling Alias");
    assert_eq!(preview, expected);
    assert!(!preview.contains("SC Sibling Goods"));
}

/// The HSN description of the captured sibling, replaced in the decoded read.
fn stock_preview_with_description(description_xml: &str) -> Result<String, StockItemReviewError> {
    let xml = stock_decode(STOCK_BEFORE);
    let original = "Plastic Articles for Conveyance";
    assert_eq!(xml.matches(original).count(), 1);
    let xml = xml.replace(original, description_xml);
    let catalogue = stock_catalogue(STOCK_COMPANY, &xml);
    let draft = stock_draft(&catalogue, "SC Copied Marked", "SC Sibling Goods");
    stock_item_review_text(&draft, ORIGIN)
}

#[test]
fn unsafe_text_in_a_copied_field_is_refused_by_code() {
    // Hand edit of the captured read: only the HSN description changes.
    assert_eq!(
        stock_preview_with_description("Plastic\u{2028}Articles"),
        Err(StockItemReviewError::Review(MasterReviewError::LayoutText))
    );
    assert_eq!(
        stock_preview_with_description("Plastic\u{200b}Articles"),
        Err(StockItemReviewError::Review(MasterReviewError::FormatText))
    );
}

#[test]
fn a_draft_the_renderer_refuses_is_not_previewed() {
    // Hand edit of the captured read: `&amp;` is read as `&`, which the
    // renderer refuses to write; two spaces together fail the plain-text rule.
    assert_eq!(
        stock_preview_with_description("Tools &amp; Dies"),
        Err(StockItemReviewError::NotRenderable(
            CreateRenderRefusal::TextNeedsEscaping {
                field: DraftText::HsnDescription
            }
        ))
    );
    assert_eq!(
        stock_preview_with_description("Tools  Dies"),
        Err(StockItemReviewError::NotRenderable(
            CreateRenderRefusal::TextNotPlain {
                field: DraftText::HsnDescription
            }
        ))
    );
}

#[test]
fn a_line_at_the_limit_fits_and_one_character_more_is_refused() {
    let prefix = "  HSN description: \"\"";
    let fitting = BATCH_REVIEW_MAX_LINE_CHARS - prefix.chars().count();
    let preview = stock_preview_with_description(&"x".repeat(fitting))
        .expect("a line of exactly the limit fits");
    let longest = preview.lines().map(|line| line.chars().count()).max();
    assert_eq!(longest, Some(BATCH_REVIEW_MAX_LINE_CHARS));
    assert_eq!(
        stock_preview_with_description(&"x".repeat(fitting + 1)),
        Err(StockItemReviewError::Review(MasterReviewError::TooLarge))
    );
}

/// Every text between `open` and `close` in `xml`, in order.
fn tag_values<'x>(xml: &'x str, tag: &str) -> Vec<&'x str> {
    let (open, close) = (format!("<{tag}>"), format!("</{tag}>"));
    xml.split(open.as_str())
        .skip(1)
        .map(|rest| rest.split(close.as_str()).next().expect("closed tag"))
        .collect()
}

/// The first JSON-quoted value after `label` on each line that starts with it.
fn shown_values(preview: &str, label: &str) -> Vec<String> {
    preview
        .lines()
        .filter_map(|line| line.strip_prefix(label))
        .map(|rest| {
            serde_json::Deserializer::from_str(rest)
                .into_iter::<String>()
                .next()
                .expect("a quoted value")
                .expect("a JSON string")
        })
        .collect()
}

#[test]
fn the_preview_shows_what_the_create_renderer_sends_for_the_same_draft() {
    for (items, new_name, source_name) in [
        (STOCK_BEFORE, "SC Copied Marked", "SC Sibling Goods"),
        (STOCK_ALIAS, "SC Copied From Alias", "SC Sibling Alias"),
    ] {
        let xml = stock_decode(items);
        let catalogue = stock_catalogue(STOCK_COMPANY, &xml);
        let draft = stock_draft(&catalogue, new_name, source_name);
        let preview = stock_item_review_text(&draft, ORIGIN).expect("the draft has a review");
        let request = render_stock_item_create(
            &PreparedStockItem::from_draft(draft).expect("the draft renders"),
        );
        // The captured company name needs no XML escape, so the envelope holds
        // it as shown.
        assert_eq!(
            shown_values(&preview, "Create ONE stock item in "),
            tag_values(&request, "SVCURRENTCOMPANY")
        );
        for (label, tag) in [
            ("Create stock item ", "NAME"),
            ("Parent (stock group): ", "PARENT"),
            ("Base unit: ", "BASEUNITS"),
            ("GST applicability: ", "GSTAPPLICABLE"),
            ("Type of supply: ", "GSTTYPEOFSUPPLY"),
            ("  Applicable from: ", "APPLICABLEFROM"),
            ("  Taxability: ", "TAXABILITY"),
            ("  Source of GST details: ", "SRCOFGSTDETAILS"),
            ("  State row 1: ", "STATENAME"),
            ("  HSN code: ", "HSNCODE"),
            ("  HSN description: ", "HSN"),
            ("  Source of HSN details: ", "SRCOFHSNDETAILS"),
        ] {
            assert_eq!(
                shown_values(&preview, label),
                tag_values(&request, tag),
                "{label}"
            );
        }
        let rates: Vec<(String, String, Option<String>)> = preview
            .lines()
            .filter(|line| line.starts_with("  Rate row "))
            .map(|line| {
                let mut quoted = serde_json::Deserializer::from_str(
                    line.split_once(": ").expect("a rate row").1,
                )
                .into_iter::<String>();
                let head = quoted.next().expect("duty head").expect("a JSON string");
                let rest = line.split_once("; valuation: ").expect("valuation").1;
                let valuation = serde_json::Deserializer::from_str(rest)
                    .into_iter::<String>()
                    .next()
                    .expect("valuation")
                    .expect("a JSON string");
                let rate = rest.split_once("; rate: ").expect("rate").1;
                let rate = (rate != "absent")
                    .then(|| serde_json::from_str::<String>(rate).expect("a JSON string"));
                (head, valuation, rate)
            })
            .collect();
        let sent: Vec<(String, String, Option<String>)> = request
            .split("<RATEDETAILS.LIST>")
            .skip(1)
            .map(|row| {
                let one = |tag: &str| tag_values(row, tag).first().map(|v| (*v).to_string());
                (
                    one("GSTRATEDUTYHEAD").expect("duty head"),
                    one("GSTRATEVALUATIONTYPE").expect("valuation"),
                    one("GSTRATE"),
                )
            })
            .collect();
        assert_eq!(rates, sent);
        assert_eq!(rates.len(), 5);
    }
}

#[test]
fn a_quote_or_backslash_in_the_company_name_is_shown_escaped() {
    // Hand-made company name; the capture is unchanged.
    let preview = stock_preview_with(
        "Bolt \"A\" \\ Lab",
        STOCK_BEFORE,
        "SC Copied Marked",
        "SC Sibling Goods",
        ORIGIN,
    )
    .expect("the draft has a review");
    assert_eq!(
        preview.lines().next(),
        Some(r#"Create ONE stock item in "Bolt \"A\" \\ Lab""#)
    );
}

#[test]
fn stock_item_review_refusals_use_addendum_codes() {
    for (review, code) in [
        (
            MasterReviewError::LayoutText,
            StockItemCreateCode::ReviewLayoutText,
        ),
        (
            MasterReviewError::FormatText,
            StockItemCreateCode::ReviewFormatText,
        ),
        (
            MasterReviewError::TooLarge,
            StockItemCreateCode::ReviewTooLarge,
        ),
    ] {
        assert_eq!(StockItemReviewError::Review(review).code(), code);
        assert_eq!(code.as_str(), review.code());
    }
    assert_eq!(
        StockItemReviewError::NotRenderable(CreateRenderRefusal::TextNotPlain {
            field: DraftText::HsnDescription,
        })
        .code(),
        StockItemCreateCode::TextNotPlain
    );
    assert_eq!(
        StockItemReviewError::NotRenderable(CreateRenderRefusal::TextNeedsEscaping {
            field: DraftText::HsnDescription,
        })
        .code(),
        StockItemCreateCode::TextNeedsEscaping
    );
}

/// The read-back verdict of `shown`'s writes, with the company and GUIDs of
/// the catalogue its sibling was found in.
fn shown_read_back(shown: &StockItemDraft<'_>, fresh: &StockItemCatalogue) -> ReadBackVerdict {
    let before = shown.sibling().sibling().catalogue();
    let guids = before
        .items()
        .iter()
        .map(|item| item.guid.clone())
        .collect();
    ReadBackVerdict::of(before.company(), shown.writes(), &guids, fresh)
}

#[test]
fn successful_outcome_and_equal_readback_have_no_refusal_code() {
    let parsed = parse_import_outcome(&stock_decode(STOCK_CREATE_ANSWER))
        .expect("the captured Create answer parses");
    let outcome = StockItemImportOutcome::of(&parsed);
    assert!(matches!(outcome, StockItemImportOutcome::CountersClean(_)));

    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    let shown = stock_draft(&before, "SC Copied Marked", "SC Sibling Goods");
    let after = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_AFTER_MARKED));
    let verdict = shown_read_back(&shown, &after);
    assert!(matches!(verdict, ReadBackVerdict::Equal(_)));

    assert_eq!(stock_item_outcome_code(&outcome), None);
    assert_eq!(stock_item_read_back_code(&verdict), None);
}

#[tokio::test]
async fn a_confirmed_stock_item_draft_is_approved_for_the_create_it_was_shown() {
    use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};

    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    let scripted = ScriptedApproval::approving();
    let recording = scripted.clone();
    let asked = SCRIPTED_APPROVAL
        .scope(scripted, async {
            show_stock_item_draft(
                stock_draft(&before, "SC Copied Marked", "SC Sibling Goods"),
                ORIGIN,
            )
            .expect("the captured draft is shown")
            .ask(stock_marks(), stock_batch())
            .into_parts()
            .1
        })
        .await;
    let shown = stock_answer(asked)
        .await
        .0
        .expect("the scripted dialog approves");
    assert_eq!(recording.stock_items(), [STOCK_ITEM_PREVIEW]);
    assert!(recording.previews().is_empty());
    assert!(recording.reviews().is_empty());
    assert!(recording.masters().is_empty());

    // The one prepared value the dialog was shown is the one sent: the preview
    // above, and the captured marked-create request here.
    let marked = bridge_tally_protocol::decode_xml_bytes(STOCK_MARKED_REQUEST)
        .expect("the captured request decodes");
    assert!(shown.sends_exactly(&marked));
    assert_eq!(
        shown.post_kind(),
        crate::tally::approved_import::PostKind::Masters
    );
}

/// The answer of a stock dialog asked in a scripted scope, and when it was
/// given.
async fn stock_answer(asked: PendingPostApproval) -> (Result<ApprovedImport, String>, Answered) {
    asked
        .answer_within(std::time::Duration::from_secs(120))
        .await
        .ok()
        .expect("the scripted dialog answers")
}

/// The journal batch a stock approval is bound to at birth.
fn stock_batch() -> String {
    crate::synthetic_batch_id(0xa1).to_string()
}

fn stock_marks() -> AgentReadRequest {
    AgentReadRequest::parse(
        crate::agent::read_profiles::render_agent_company_high_water(STOCK_COMPANY),
    )
    .expect("the marks request is admitted")
}

#[tokio::test]
async fn a_declined_or_unscoped_stock_item_dialog_makes_no_approval() {
    use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};

    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    let scripted = ScriptedApproval::declining();
    let recording = scripted.clone();
    let shown = || {
        show_stock_item_draft(
            stock_draft(&before, "SC Copied Marked", "SC Sibling Goods"),
            ORIGIN,
        )
        .expect("the captured draft is shown")
    };
    let asked = SCRIPTED_APPROVAL
        .scope(scripted, async {
            shown().ask(stock_marks(), stock_batch()).into_parts().1
        })
        .await;
    let declined = stock_answer(asked).await.0.err();
    assert_eq!(declined.as_deref(), Some("master_approval_declined"));
    assert_eq!(recording.stock_items(), [STOCK_ITEM_PREVIEW]);

    let unscoped = stock_answer(shown().ask(stock_marks(), stock_batch()).into_parts().1)
        .await
        .0
        .err();
    assert_eq!(unscoped.as_deref(), Some("master_approval_declined"));
}

#[tokio::test]
async fn a_refused_stock_item_preview_asks_no_one() {
    use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};

    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    let scripted = ScriptedApproval::approving();
    let recording = scripted.clone();
    let refused = SCRIPTED_APPROVAL
        .scope(scripted, async {
            show_stock_item_draft(
                stock_draft(&before, "SC Copied Marked", "SC Sibling Goods"),
                "http://127.0.0.1:9000\nTally: other",
            )
            .map(|shown| shown.ask(stock_marks(), stock_batch()))
        })
        .await
        .err();
    assert_eq!(
        refused,
        Some(StockItemReviewError::Review(MasterReviewError::LayoutText))
    );
    assert!(recording.stock_items().is_empty());
}

#[cfg(not(windows))]
const STOCK_ITEM_ANSWER: &str = "Click \"Create stock item\" to create this stock item in Tally. Click Cancel to cancel. Nothing is created until you answer.";
#[cfg(windows)]
const STOCK_ITEM_ANSWER: &str = "Press Y (or click Yes) to create this stock item in Tally. Press N (or click No) to cancel. Nothing\nis created until you answer.";

#[test]
fn the_stock_item_dialog_shows_its_answer_then_the_whole_preview() {
    let shown = crate::tally::approved_import::stock_item_dialog_text(STOCK_ITEM_PREVIEW);
    assert_eq!(
        shown,
        format!("{STOCK_ITEM_ANSWER}\n\n{STOCK_ITEM_PREVIEW}")
    );
    #[cfg(not(windows))]
    {
        assert_eq!(shown.lines().count(), 31);
        assert_eq!(shown.len(), 1_449);
    }
    #[cfg(windows)]
    {
        assert_eq!(shown.lines().count(), 32);
        assert_eq!(shown.len(), 1_455);
    }
}

#[test]
#[ignore = "lab tool: writes the stock-item box input for row 1381-B; not a check"]
fn write_the_stock_item_box_input() {
    let path = std::env::var("BRIDGE_STOCK_BOX_INPUT")
        .expect("BRIDGE_STOCK_BOX_INPUT must name the lab output path");
    let input = format!(
        "{}\n1\n{}",
        crate::synthetic_guid(0),
        crate::tally::approved_import::stock_item_dialog_text(STOCK_ITEM_PREVIEW)
    );
    std::fs::write(path, input).expect("the lab box input is writable");
}

/// The stock item `new_name` copied from the captured Goods sibling, as shown.
fn shown_stock_item(new_name: &str) -> ShownStockItem {
    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    show_stock_item_draft(stock_draft(&before, new_name, "SC Sibling Goods"), ORIGIN)
        .expect("the captured draft is shown")
}

/// The digest a stock binding names: of the bytes its item renders to.
fn stock_digest(prepared: &PreparedStockItem) -> String {
    bridge_tally_protocol::sha256_hex(render_stock_item_create(prepared).as_bytes())
}

/// A ledger `Create` in the stock lab's company, for the cross-kind cases.
fn stock_company_ledger() -> LedgerCreateBatch {
    LedgerCreateBatch::one(item(
        "SC Ledger Party",
        party(PartySide::SundryDebtors, None, false),
    ))
}

/// `PostApprovals` over a data directory with its imports directory, so a
/// lapse leaves its note.
fn stock_approvals(directory: &std::path::Path) -> PostApprovals {
    std::fs::create_dir_all(directory.join("imports")).unwrap();
    PostApprovals::new(directory)
}

/// The stock item `new_name` asked under an approving scripted dialog and
/// answered: the binding and held request its one ask made, the approval and
/// its answer.
async fn approved_stock_item(
    new_name: &str,
) -> (ApprovalBinding, HeldRequest, ApprovedImport, Answered) {
    let shown = shown_stock_item(new_name);
    let asked = SCRIPTED_APPROVAL
        .scope(ScriptedApproval::approving(), async {
            shown.ask(stock_marks(), stock_batch())
        })
        .await;
    let (binding, dialog, held) = asked.into_parts();
    let (approved, answered) = stock_answer(dialog).await;
    (
        binding,
        held,
        approved.expect("the scripted dialog approves"),
        answered,
    )
}

/// The binding, the approval and the held request one ask made are held, then
/// taken with the fresh binding of an equal draft: the redemption gives back
/// the stock approval, of the masters kind, with the item asked about, which
/// renders to the bytes it sends, bound to its batch. The fresh binding of
/// another draft is refused.
#[tokio::test]
async fn one_asks_binding_approval_and_held_request_are_taken_with_a_fresh_binding() {
    let directory = tempfile::tempdir().unwrap();
    let approvals = stock_approvals(directory.path());
    let batch = stock_batch();
    let prepared = shown_stock_item("SC Copied Marked").prepared;
    let (binding, held, approved, answered) = approved_stock_item("SC Copied Marked").await;
    assert_eq!(binding.kind(), PostKind::Masters);
    approvals
        .hold_approved(&batch, binding.clone(), approved, held, answered)
        .unwrap();
    assert!(matches!(approvals.begin(&batch), Begin::Redeem(_)));
    let fresh = shown_stock_item("SC Copied Marked").binding();
    assert_eq!(fresh, binding);
    let (taken, redeemed, held) = approvals.take_for_dispatch(&batch, &fresh).unwrap();
    assert_eq!(redeemed.post_kind(), PostKind::Masters);
    assert_eq!(redeemed.master_batch_id(), Some(batch.as_str()));
    let HeldRequest::StockItem(held) = held else {
        panic!("a stock approval comes back with its stock item");
    };
    assert_eq!(*held.prepared(), prepared);
    let marked = bridge_tally_protocol::decode_xml_bytes(STOCK_MARKED_REQUEST)
        .expect("the captured request decodes");
    assert_eq!(render_stock_item_create(held.prepared()), marked);
    assert!(redeemed.sends_exactly(&marked));
    drop(taken);

    let (binding, held, approved, answered) = approved_stock_item("SC Copied Marked").await;
    approvals
        .hold_approved(&batch, binding, approved, held, answered)
        .unwrap();
    let other = shown_stock_item("SC Copied Other").binding();
    assert_eq!(
        approvals.take_for_dispatch(&batch, &other).err().as_deref(),
        Some("import_approval_binding_changed")
    );
    assert!(matches!(approvals.begin(&batch), Begin::Ask(_)));
}

/// A held stock approval taken with a binding for anything else is refused as
/// a changed binding, and lapses with a note that is not an approval: other
/// writes under the same preview, the same writes under another preview or at
/// another endpoint, and a ledger binding with the same digest, endpoint,
/// company and preview.
#[tokio::test]
async fn a_held_stock_item_approval_is_refused_for_another_binding() {
    let directory = tempfile::tempdir().unwrap();
    let approvals = stock_approvals(directory.path());
    let batch = stock_batch();
    let base = shown_stock_item("SC Copied Marked");
    let other = shown_stock_item("SC Copied Other");
    assert_ne!(other.prepared.writes(), base.prepared.writes());
    let company = base.prepared.company().clone();
    // The binding names the endpoint the preview was shown for.
    assert_eq!(
        base.binding(),
        ApprovalBinding::stock_item(ORIGIN, &base.prepared, &base.preview)
    );
    let fresh_bindings = [
        (
            "writes",
            ApprovalBinding::stock_item(ORIGIN, &other.prepared, &base.preview),
        ),
        (
            "preview",
            ApprovalBinding::stock_item(ORIGIN, &base.prepared, &other.preview),
        ),
        (
            "endpoint",
            ApprovalBinding::stock_item("http://127.0.0.1:1", &base.prepared, &base.preview),
        ),
        (
            "ledger",
            ApprovalBinding::masters(
                &stock_digest(&base.prepared),
                ORIGIN,
                &company,
                &stock_company_ledger(),
                &base.preview,
            ),
        ),
    ];
    for (case, fresh) in fresh_bindings {
        let (binding, held, approved, answered) = approved_stock_item("SC Copied Marked").await;
        assert_eq!(binding, base.binding(), "{case}");
        assert_ne!(fresh, binding, "{case} must differ from the base");
        approvals
            .hold_approved(&batch, binding, approved, held, answered)
            .unwrap();
        assert_eq!(
            approvals.take_for_dispatch(&batch, &fresh).err().as_deref(),
            Some("import_approval_binding_changed"),
            "{case}"
        );
        let note = approvals.lapse_note(&batch).unwrap();
        assert_eq!(note["reason"], "approval_binding_changed", "{case}");
        assert_eq!(note["redeemable"], false, "{case}");
        assert!(matches!(approvals.begin(&batch), Begin::Ask(_)), "{case}");
    }
}

fn answered_now() -> Answered {
    Answered {
        approved: true,
        at: std::time::Instant::now(),
        at_wall: std::time::SystemTime::now(),
    }
}

/// The request held beside an approval must be the one it sends: a stock
/// approval held with another stock item or a ledger request, and a ledger
/// approval held with a stock item, are refused with an equal binding. A held
/// ledger approval is refused for a stock binding, after its own takes it, and
/// the refusal lapses it as a changed binding and asks again.
#[tokio::test]
async fn held_requests_of_other_bytes_and_a_held_ledger_approval_for_a_stock_binding_are_refused() {
    let directory = tempfile::tempdir().unwrap();
    let approvals = stock_approvals(directory.path());
    let batch = stock_batch();
    let other = shown_stock_item("SC Copied Other").prepared;
    let prepared = shown_stock_item("SC Copied Marked").prepared;
    assert_ne!(
        render_stock_item_create(&other),
        render_stock_item_create(&prepared)
    );
    let company = prepared.company().clone();
    let ledger = stock_company_ledger();
    let other_held: [(&str, HeldRequest); 2] = [
        (
            "another stock item",
            approved_stock_item("SC Copied Other").await.1,
        ),
        (
            "a ledger request",
            MasterPostRequest::render(ledger.clone(), &company).into(),
        ),
    ];
    for (case, held) in other_held {
        let (binding, _, approved, answered) = approved_stock_item("SC Copied Marked").await;
        approvals
            .hold_approved(&batch, binding.clone(), approved, held, answered)
            .unwrap();
        assert_eq!(
            approvals
                .take_for_dispatch(&batch, &binding)
                .err()
                .as_deref(),
            Some("import_approval_binding_changed"),
            "{case}"
        );
        assert!(matches!(approvals.begin(&batch), Begin::Ask(_)), "{case}");
    }

    let ledger_binding = ApprovalBinding::masters(
        &stock_digest(&prepared),
        ORIGIN,
        &company,
        &ledger,
        "Ledger preview",
    );
    approvals
        .hold_approved(
            &batch,
            ledger_binding.clone(),
            ApprovedImport::approved_masters_for_test(ledger.clone(), &company, stock_marks()),
            approved_stock_item("SC Copied Marked").await.1,
            answered_now(),
        )
        .unwrap();
    assert_eq!(
        approvals
            .take_for_dispatch(&batch, &ledger_binding)
            .err()
            .as_deref(),
        Some("import_approval_binding_changed"),
        "a ledger approval held with a stock item"
    );
    let hold_ledger = |batch: &str| {
        let approved =
            ApprovedImport::approved_masters_for_test(ledger.clone(), &company, stock_marks());
        let held = MasterPostRequest::render(ledger.clone(), &company);
        (approvals.hold_approved(
            batch,
            ledger_binding.clone(),
            approved,
            held,
            answered_now(),
        ))
        .unwrap();
    };
    hold_ledger(&batch);
    drop(
        approvals
            .take_for_dispatch(&batch, &ledger_binding)
            .unwrap(),
    );
    // A batch of its own, with no note yet: the note read below is this
    // refusal's.
    let ledger_batch = crate::synthetic_batch_id(0xa2).to_string();
    hold_ledger(&ledger_batch);
    assert!(approvals.lapse_note(&ledger_batch).is_none());
    let stock = shown_stock_item("SC Copied Marked").binding();
    assert_eq!(stock.kind(), ledger_binding.kind());
    assert_eq!(
        approvals
            .take_for_dispatch(&ledger_batch, &stock)
            .err()
            .as_deref(),
        Some("import_approval_binding_changed"),
        "a held ledger approval, a stock binding"
    );
    assert_eq!(
        approvals.lapse_note(&ledger_batch).unwrap()["reason"],
        "approval_binding_changed"
    );
    assert!(matches!(approvals.begin(&ledger_batch), Begin::Ask(_)));
}

/// One prepared item has one digest: its binding's, its approval's and the
/// one its writes give for its company are the same.
#[tokio::test]
async fn a_stock_items_binding_digest_is_its_approvals_and_its_writes_digest() {
    let prepared = shown_stock_item("SC Copied Marked").prepared;
    let (binding, _, approved, _) = approved_stock_item("SC Copied Marked").await;
    let digest = binding.digest_for_test().to_string();
    assert_eq!(digest, approved.xml_sha256());
    let writes = prepared.writes().create_sha256(prepared.company());
    assert_eq!(digest, writes.expect("the captured item renders"));
}

/// Asked, then held while its dialog is open: on the scripted approval the
/// held dialog is joined, approved and taken; on the scripted decline its
/// refusal is kept and returned once, as the ledger dialog's code.
#[tokio::test]
async fn a_held_stock_item_dialog_is_approved_or_refused_by_its_answer() {
    for approve in [true, false] {
        let directory = tempfile::tempdir().unwrap();
        let approvals = stock_approvals(directory.path());
        let batch = stock_batch();
        let shown = shown_stock_item("SC Copied Marked");
        let scripted = if approve {
            ScriptedApproval::approving()
        } else {
            ScriptedApproval::declining()
        };
        let asked = SCRIPTED_APPROVAL
            .scope(scripted.clone(), async {
                shown.ask(stock_marks(), stock_batch())
            })
            .await;
        let (binding, dialog, held) = asked.into_parts();
        approvals
            .hold_pending(&batch, binding.clone(), dialog, held)
            .unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(120), async {
            while !approvals.answered_for_test(&batch) {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("the held dialog's answer was stamped");
        assert_eq!(scripted.stock_items(), [STOCK_ITEM_PREVIEW]);
        if approve {
            let Begin::Join(dialog) = approvals.begin(&batch) else {
                panic!("an answered dialog is joined");
            };
            let answer = stock_answer(dialog).await;
            assert!(matches!(
                approvals.settle_join(&batch, Ok(answer)),
                Joined::Approved
            ));
            let (taken, redeemed, held) = approvals.take_for_dispatch(&batch, &binding).unwrap();
            assert!(matches!(held, HeldRequest::StockItem(_)));
            assert_eq!(redeemed.post_kind(), PostKind::Masters);
            drop(taken);
        } else {
            let Begin::Refused(kept) = approvals.begin(&batch) else {
                panic!("a declined dialog's refusal is kept");
            };
            assert_eq!(kept.code(), "master_approval_declined");
            assert!(matches!(approvals.begin(&batch), Begin::Ask(_)));
        }
    }
}

/// Each refusal of the stock-item dialog is stamped as the ledger dialog's
/// code for the same answer.
#[test]
fn each_stock_item_dialog_refusal_has_the_ledger_dialogs_code() {
    for (refusal, code) in [
        (StockItemDialogRefusal::Declined, "master_approval_declined"),
        (
            StockItemDialogRefusal::Unavailable,
            "master_approval_unavailable",
        ),
        (
            StockItemDialogRefusal::TimedOut,
            "master_approval_timed_out",
        ),
        (StockItemDialogRefusal::TooLarge, "master_review_too_large"),
    ] {
        assert_eq!(refusal.code(), code);
    }
}
