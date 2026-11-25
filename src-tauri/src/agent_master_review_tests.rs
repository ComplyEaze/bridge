//! The one-ledger dialog's text: exact for each admitted kind, unsafe text
//! refused, and every limit a refusal, never a cut.
use super::*;
use crate::agent::failure_code::{stock_item_outcome_code, stock_item_read_back_code};
use crate::tally::ledger_create_item::{
    AdmittedDutyHead, AdmittedLedgerKind, PartySide, VerifiedGstin,
};
use crate::tally::master_name::MasterName;
use crate::tally::stock_item_create::{
    render_stock_item_create, CreateRenderRefusal, DraftText, GoodsSibling, ShownDraft, SiblingGst,
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
        let request =
            render_stock_item_create(&ShownDraft::from_draft(draft)).expect("the draft renders");
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

#[test]
fn successful_outcome_and_equal_readback_have_no_refusal_code() {
    let parsed = parse_import_outcome(&stock_decode(STOCK_CREATE_ANSWER))
        .expect("the captured Create answer parses");
    let outcome = StockItemImportOutcome::of(&parsed);
    assert!(matches!(outcome, StockItemImportOutcome::CountersClean(_)));

    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    let shown =
        ShownDraft::from_draft(stock_draft(&before, "SC Copied Marked", "SC Sibling Goods"));
    let after = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_AFTER_MARKED));
    let verdict = ReadBackVerdict::of(&shown, &after);
    assert!(matches!(verdict, ReadBackVerdict::Equal(_)));

    assert_eq!(stock_item_outcome_code(&outcome), None);
    assert_eq!(stock_item_read_back_code(&verdict), None);
}

#[tokio::test]
async fn a_confirmed_stock_item_draft_holds_the_preview_it_was_shown() {
    use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};

    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    let scripted = ScriptedApproval::approving();
    let recording = scripted.clone();
    let shown = SCRIPTED_APPROVAL
        .scope(scripted, async {
            confirm_stock_item_draft(
                stock_draft(&before, "SC Copied Marked", "SC Sibling Goods"),
                ORIGIN,
            )
            .await
        })
        .await
        .expect("the scripted dialog approves");
    assert_eq!(shown.shown_text(), STOCK_ITEM_PREVIEW);
    assert_eq!(recording.stock_items(), [STOCK_ITEM_PREVIEW]);
    assert!(recording.previews().is_empty());
    assert!(recording.reviews().is_empty());
    assert!(recording.masters().is_empty());

    let rendered = render_stock_item_create(&shown).expect("the confirmed draft renders");
    let expected = render_stock_item_create(&ShownDraft::from_draft(stock_draft(
        &before,
        "SC Copied Marked",
        "SC Sibling Goods",
    )))
    .expect("the same draft renders");
    assert_eq!(rendered, expected);

    let after = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_AFTER_MARKED));
    assert!(matches!(
        ReadBackVerdict::of(&shown, &after),
        ReadBackVerdict::Equal(_)
    ));
}

#[tokio::test]
async fn a_declined_or_unscoped_stock_item_dialog_makes_no_shown_draft() {
    use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};

    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    let scripted = ScriptedApproval::declining();
    let recording = scripted.clone();
    let declined = SCRIPTED_APPROVAL
        .scope(scripted, async {
            confirm_stock_item_draft(
                stock_draft(&before, "SC Copied Marked", "SC Sibling Goods"),
                ORIGIN,
            )
            .await
        })
        .await
        .err();
    assert_eq!(
        declined,
        Some(StockItemConfirmError::Dialog(
            StockItemDialogRefusal::Declined,
        ))
    );
    assert_eq!(recording.stock_items(), [STOCK_ITEM_PREVIEW]);

    let unscoped = confirm_stock_item_draft(
        stock_draft(&before, "SC Copied Marked", "SC Sibling Goods"),
        ORIGIN,
    )
    .await
    .err();
    assert_eq!(
        unscoped,
        Some(StockItemConfirmError::Dialog(
            StockItemDialogRefusal::Declined,
        ))
    );
}

#[tokio::test]
async fn a_refused_stock_item_preview_asks_no_one() {
    use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};

    let before = stock_catalogue(STOCK_COMPANY, &stock_decode(STOCK_BEFORE));
    let scripted = ScriptedApproval::approving();
    let recording = scripted.clone();
    let refused = SCRIPTED_APPROVAL
        .scope(scripted, async {
            confirm_stock_item_draft(
                stock_draft(&before, "SC Copied Marked", "SC Sibling Goods"),
                "http://127.0.0.1:9000\nTally: other",
            )
            .await
        })
        .await
        .err();
    assert_eq!(
        refused,
        Some(StockItemConfirmError::Review(StockItemReviewError::Review(
            MasterReviewError::LayoutText
        ),))
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
