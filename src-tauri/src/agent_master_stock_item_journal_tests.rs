//! Stock-item journal tests, on row 1381-R1's captured reads; see
//! `bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/PROVENANCE.md`.
//! Every change to a stored value is marked as a hand edit.
use super::super::{
    append, append_intent, append_kind_intent, is_master_file, read_journal, read_kind_journal,
    write_pre_image, ExclusiveAdmission, JournalRecord, MasterBatch, MasterClass,
    MasterDispatchIntent, MasterJournal, MasterLedgerError, MasterMark, MasterRecord,
    StoredLedgerCreate, StoredLedgerKind,
};
use super::*;
use crate::agent::agent_import::tests::sentinel_batch_id;
use crate::tally::master_name::MasterName as Name;
use crate::tally::stock_item_create::{GoodsSibling, SiblingGst, StockItemDraft};
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, decode_xml_bytes, parse_import_outcome, sha256_hex,
    ExpectedTallyTextEncoding,
};
use std::io::Cursor;
use std::path::Path;

const COMPANY: &str = "BRIDGE STOCK CREATE LAB";
const COMPANY_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1";
const MARKED_NAME: &str = "SC Copied Marked";
const MARKED_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1-000000d2";
/// The master mark of the captured marks read, `stock-create-lab-marks`, as
/// its `PROVENANCE.md` records it.
const CAPTURED_MARK: u64 = 210;
const ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-create-answer.utf16le.xml");
const BEFORE: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-before.utf16le.xml");
const MARKED: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-after-marked-create.utf16le.xml");
const GROUPS: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-groups.utf16le.xml");
const UNITS: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-units.utf16le.xml");
const MARKED_REQUEST: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-marked-create.request.utf16le.xml");

/// What `StockItemBatch::new` writes for the captured marked draft, the batch id
/// put back in at run time.
const GOLDEN_STOCK_BATCH: &str = "{\"record_type\":\"master_batch\",\"batch_id\":\"@ID@\",\"batch_sha256\":\"2cf906404b718a0507d7901e38e28ee8e370cb159788ed49526b28288af77dd7\",\"class\":\"stock_item_create\",\"company\":{\"guid\":\"bac2b5e9-ada9-43c0-b8f8-a2812a813fa1\",\"name\":\"BRIDGE STOCK CREATE LAB\",\"number\":\"1\",\"books_from\":\"20260401\"},\"item\":{\"name\":\"SC Copied Marked\",\"sibling_guid\":\"bac2b5e9-ada9-43c0-b8f8-a2812a813fa1-000000d0\",\"sibling_source_name\":\"SC Sibling Goods\",\"parent\":\"SC Goods Group\",\"unit\":\"Nos\",\"gst_applicable_from\":\"20250401\",\"rates\":[{\"duty_head\":\"CGST\",\"rate\":\"9\"},{\"duty_head\":\"SGST/UTGST\",\"rate\":\"9\"},{\"duty_head\":\"IGST\",\"rate\":\"18\"},{\"duty_head\":\"Cess\",\"rate\":null},{\"duty_head\":\"State Cess\",\"rate\":null}],\"hsn_applicable_from\":\"20250401\",\"hsn_code\":\"39239090\",\"hsn_description\":\"Plastic Articles for Conveyance\"}}\n";

fn decode(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("the capture decodes")
    .text
}

fn identity(name: &str) -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture(name, COMPANY_GUID)
}

fn catalogue(bytes: &[u8]) -> StockItemCatalogue {
    StockItemCatalogue::from_candidate_read(&identity(COMPANY), &decode(bytes))
        .expect("the captured read catalogues")
}

/// The captured marked Create's writes, drafted from `SC Sibling Goods`.
fn marked_writes() -> StockItemWrites {
    let before = catalogue(BEFORE);
    let absent = before
        .name_absent(&Name::new(MARKED_NAME).unwrap())
        .unwrap();
    let sibling = SiblingGst::named(&before, &Name::new("SC Sibling Goods").unwrap()).unwrap();
    let sibling = GoodsSibling::from_sibling(sibling).unwrap();
    StockItemDraft::new(absent, sibling, &decode(GROUPS), &decode(UNITS))
        .expect("the captured values draft")
        .writes()
        .clone()
}

fn stock_batch(serial: u16) -> StockItemBatch {
    StockItemBatch::new(
        sentinel_batch_id(serial),
        &identity(COMPANY),
        &marked_writes(),
    )
    .expect("the captured writes render")
}

fn lock() -> ExclusiveAdmission {
    let held = std::fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")).unwrap();
    ExclusiveAdmission::standing_in(held)
}

fn line<B: JournalBatch>(record: &JournalRecord<B>) -> String {
    format!("{}\n", serde_json::to_string(record).unwrap())
}

fn invalid(cause: Invalid) -> MasterLedgerError {
    MasterLedgerError::Invalid(cause)
}

/// A sidecar of each GUID of `read` with its ALTERID.
fn pre_image_of(read: &StockItemCatalogue) -> PreImage {
    PreImage::new(
        read.items()
            .iter()
            .map(|item| (item.guid().to_owned(), item.alter_id())),
    )
    .unwrap()
}

/// Writes `batch`'s sidecar from `read` in `directory` and returns its intent,
/// built with the generic `MasterDispatchIntent::new`.
fn intent_for<B: JournalBatch>(
    directory: &Path,
    batch: &B,
    read: &StockItemCatalogue,
) -> MasterDispatchIntent {
    intent_with(directory, batch, &pre_image_of(read))
}

fn intent_with<B: JournalBatch>(
    directory: &Path,
    batch: &B,
    pre_image: &PreImage,
) -> MasterDispatchIntent {
    let written = write_pre_image(directory, &lock(), batch, pre_image).unwrap();
    MasterDispatchIntent::new(
        batch,
        batch.batch_sha256().to_owned(),
        written,
        vec![MasterMark {
            company_guid: COMPANY_GUID.into(),
            altmstid: CAPTURED_MARK,
        }],
    )
    .unwrap()
}

/// The batch line and its intent line, its sidecar taken from the BEFORE read.
fn dispatched_text(dir: &Path, batch: &StockItemBatch) -> String {
    let intent = intent_for(dir, batch, &catalogue(BEFORE));
    let intent = JournalRecord::<StockItemBatch>::DispatchIntent(intent);
    line(&JournalRecord::Batch(batch.clone())) + &line(&intent)
}

/// Replaces the one `from` in `text`: a hand edit of a stored value.
fn hand_edit(text: &str, from: &str, to: &str) -> String {
    assert_eq!(
        text.matches(from).count(),
        1,
        "the edit starts in one place"
    );
    text.replacen(from, to, 1)
}

fn basis_of(
    dir: &Path,
    text: &str,
    batch_id: &str,
    company: &VerifiedCompanyIdentity,
) -> Result<StockItemReadBackBasis, BasisRefusal> {
    let journal = Journal::<StockItemBatch>::read(Cursor::new(text.as_bytes())).unwrap();
    StockItemReadBackBasis::from_journal(dir, &journal, batch_id, company)
}

#[test]
fn a_batch_digests_as_the_captured_request_and_reads_back_equal() {
    let batch = stock_batch(0x81);
    let captured = decode_xml_bytes(MARKED_REQUEST).expect("the marked request decodes");
    assert_eq!(batch.batch_sha256, sha256_hex(captured.as_bytes()));
    let directory = tempfile::tempdir().unwrap();
    let dir = directory.path();
    append(dir, &lock(), JournalRecord::Batch(batch.clone())).unwrap();
    assert!(dir.join("agent-master-stock-item.jsonl").exists());
    assert!(!dir.join("agent-master-ledger.jsonl").exists());
    let journal = read_kind_journal::<StockItemBatch>(dir).unwrap();
    let state = journal.state(&batch.batch_id).unwrap();
    assert_eq!(state.batch, batch);
    assert_eq!(state.batch.item.rebuild(), Ok(marked_writes()));
}

#[test]
fn the_stock_batch_line_keeps_its_bytes() {
    let batch = stock_batch(0x82);
    let golden = GOLDEN_STOCK_BATCH.replace("@ID@", &batch.batch_id);
    assert_eq!(line(&JournalRecord::Batch(batch)), golden);
}

#[test]
fn a_stored_line_that_does_not_rebuild_or_parse_is_refused() {
    let text = line(&JournalRecord::Batch(stock_batch(0x83)));
    let read = |text: &str| Journal::<StockItemBatch>::read(Cursor::new(text.as_bytes()));
    // Hand edit: the stored IGST rate `18` given a leading space, which is not
    // the plain decimal a draft stores.
    let edited = hand_edit(&text, "\"rate\":\"18\"", "\"rate\":\" 18\"");
    assert_eq!(read(&edited).unwrap_err(), invalid(Invalid::ItemNotRebuilt));
    // Hand edit: a field the line does not allow.
    let edited = hand_edit(&text, "\"class\":", "\"kind\":null,\"class\":");
    assert_eq!(read(&edited).unwrap_err(), invalid(Invalid::Parse));
    // Hand edit: the ledger class.
    let edited = hand_edit(&text, "\"stock_item_create\"", "\"ledger_create\"");
    assert_eq!(read(&edited).unwrap_err(), invalid(Invalid::Parse));
    // A stock line in the ledger file, and a ledger line in the stock file.
    let in_ledger = MasterJournal::read(Cursor::new(text.as_bytes()));
    assert_eq!(in_ledger.unwrap_err(), invalid(Invalid::Parse));
    let ledger = line(&MasterRecord::Batch(ledger_batch(0x84)));
    assert_eq!(read(&ledger).unwrap_err(), invalid(Invalid::Parse));
}

#[test]
fn the_basis_refuses_each_loose_pairing() {
    let directory = tempfile::tempdir().unwrap();
    let dir = directory.path();
    let batch = stock_batch(0x85);
    let id = batch.batch_id.clone();
    let own = identity(COMPANY);
    let refused = |text: &str, company| basis_of(dir, text, &id, company).unwrap_err();
    let batch_only = line(&JournalRecord::Batch(batch.clone()));
    assert_eq!(refused(&batch_only, &own), BasisRefusal::NoIntent);
    let text = dispatched_text(dir, &batch);
    let unknown = sentinel_batch_id(0x8a);
    let no_batch = basis_of(dir, &text, &unknown, &own).unwrap_err();
    assert_eq!(no_batch, BasisRefusal::NoBatch);
    // Hand edit: another canonical company GUID stored, in the batch and in
    // the intent's mark, so the journal still admits both.
    let stored = format!("\"{COMPANY_GUID}\"");
    assert_eq!(
        text.matches(&stored).count(),
        2,
        "the edit starts in two places"
    );
    let edited = text.replace(&stored, "\"00000000-0000-4000-8000-000000000002\"");
    assert_eq!(refused(&edited, &own), BasisRefusal::OtherCompanyGuid);
    // Hand edit: another company number stored.
    let edited = hand_edit(&text, "\"number\":\"1\"", "\"number\":\"2\"");
    assert_eq!(refused(&edited, &own), BasisRefusal::OtherCompanyNumber);
    // Hand edit: another books-from date stored.
    let edited = hand_edit(
        &text,
        "\"books_from\":\"20260401\"",
        "\"books_from\":\"20250401\"",
    );
    assert_eq!(refused(&edited, &own), BasisRefusal::OtherBooksFrom);
    // The company as verified now has another display name, which its Create
    // would also render, and digest, with.
    let renamed = identity("BRIDGE STOCK CREATE LAB TWO");
    assert_eq!(refused(&text, &renamed), BasisRefusal::OtherCompanyName);
    // Hand edit: the stored parent changed to other text `rebuild` admits; the
    // company matches, the item rebuilds, and the digest is not the batch's.
    let edited = hand_edit(
        &text,
        "\"parent\":\"SC Goods Group\"",
        "\"parent\":\"SC Other\"",
    );
    assert_eq!(refused(&edited, &own), BasisRefusal::DigestMismatch);
    // Hand edit: the stored IGST rate `18` made `18.0`, a plain decimal that
    // `rebuild` keeps as stored, so the line is admitted and the digest refuses.
    let edited = hand_edit(&text, "\"rate\":\"18\"", "\"rate\":\"18.0\"");
    assert_eq!(refused(&edited, &own), BasisRefusal::DigestMismatch);
    // Hand edit: the intent's request digest made another digest.
    let edited = hand_edit(
        &text,
        &format!("\"native_request_sha256\":\"{}\"", batch.batch_sha256),
        &format!("\"native_request_sha256\":\"{}\"", "f".repeat(64)),
    );
    assert_eq!(refused(&edited, &own), BasisRefusal::RequestDigestMismatch);
    // Hand edit: the intent's sidecar count raised by one.
    let count = pre_image_of(&catalogue(BEFORE)).len();
    let edited = hand_edit(
        &text,
        &format!("\"pre_image_count\":{count}"),
        &format!("\"pre_image_count\":{}", count + 1),
    );
    let mismatch = BasisRefusal::PreImage(PreImageError::CountMismatch);
    assert_eq!(refused(&edited, &own), mismatch);
    // A sidecar whose bytes are not the ones the intent's digest was taken from.
    let other = serde_json::to_vec(&pre_image_of(&catalogue(MARKED))).unwrap();
    std::fs::write(dir.join(format!("master-pre-image-{id}.json")), other).unwrap();
    assert_eq!(
        refused(&text, &own),
        BasisRefusal::PreImage(PreImageError::ShaMismatch)
    );
    // The sidecar deleted.
    std::fs::remove_file(dir.join(format!("master-pre-image-{id}.json"))).unwrap();
    let missing = BasisRefusal::PreImage(PreImageError::Missing);
    assert_eq!(refused(&text, &own), missing);
}

#[test]
fn a_sidecar_of_another_company_is_refused() {
    let directory = tempfile::tempdir().unwrap();
    let dir = directory.path();
    let batch = stock_batch(0x8b);
    // Hand edit: one GUID of the BEFORE map given another company's part, the
    // sidecar written so its digest and count are those of the intent.
    let mut entries = pre_image_of(&catalogue(BEFORE)).entries().clone();
    let (guid, alter_id) = entries.pop_first().unwrap();
    let master = guid.strip_prefix(COMPANY_GUID).unwrap();
    entries.insert(
        format!("00000000-0000-4000-8000-000000000002{master}"),
        alter_id,
    );
    let pre_image = PreImage::new(entries).unwrap();
    let intent =
        JournalRecord::<StockItemBatch>::DispatchIntent(intent_with(dir, &batch, &pre_image));
    let text = line(&JournalRecord::Batch(batch.clone())) + &line(&intent);
    let refused = basis_of(dir, &text, &batch.batch_id, &identity(COMPANY)).unwrap_err();
    assert_eq!(refused, BasisRefusal::BeforeOfOtherCompany);
}

/// A data folder holding `batch` and its intent, its sidecar taken from `read`.
fn dispatched_folder(batch: &StockItemBatch, read: &StockItemCatalogue) -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    let (dir, lock) = (directory.path(), lock());
    append(dir, &lock, JournalRecord::Batch(batch.clone())).unwrap();
    let intent = intent_for(dir, batch, read);
    let _recorded = append_kind_intent::<StockItemBatch>(dir, &lock, intent).unwrap();
    directory
}

fn basis_in(dir: &Path, batch: &StockItemBatch) -> StockItemReadBackBasis {
    let journal = read_kind_journal::<StockItemBatch>(dir).unwrap();
    StockItemReadBackBasis::from_journal(dir, &journal, &batch.batch_id, &identity(COMPANY))
        .unwrap()
}

#[test]
fn a_basis_from_the_journal_reads_the_captured_create_back() {
    let (before, marked) = (catalogue(BEFORE), catalogue(MARKED));
    let batch = stock_batch(0x86);
    let directory = dispatched_folder(&batch, &before);
    let basis = basis_in(directory.path(), &batch);
    assert!(matches!(
        basis.read_back(&marked),
        ReadBackVerdict::Equal(_)
    ));
    let not_found = ReadBackVerdict::NotFoundOnce { rows: 0 };
    assert_eq!(basis.read_back(&before), not_found);
    let outcome = StockItemImportOutcome::of(&parse_import_outcome(&decode(ANSWER)).unwrap());
    assert_eq!(basis.verify(outcome, &marked).unwrap().guid(), MARKED_GUID);
    // The sidecar taken from the read after the Create holds the new row.
    let after = dispatched_folder(&batch, &marked);
    let not_new = basis_in(after.path(), &batch).read_back(&marked);
    assert_eq!(not_new, ReadBackVerdict::NotNew);
}

fn ledger_batch(serial: u16) -> MasterBatch {
    MasterBatch {
        batch_id: sentinel_batch_id(serial),
        batch_sha256: "a".repeat(64),
        class: MasterClass::LedgerCreate,
        company: MasterCompany {
            guid: COMPANY_GUID.into(),
            name: COMPANY.into(),
            number: "1".into(),
            books_from: "20260401".into(),
        },
        items: vec![StoredLedgerCreate {
            name: MARKED_NAME.into(),
            kind: StoredLedgerKind::Suspense,
            bill_wise: false,
            party_gstin: None,
            gst_duty_head: None,
        }],
    }
}

#[test]
fn an_open_stock_dispatch_blocks_its_name_and_a_ledger_of_that_name_does_not() {
    let before = catalogue(BEFORE);
    let directory = tempfile::tempdir().unwrap();
    let (dir, lock) = (directory.path(), lock());
    // A ledger of the same name, dispatched and open, in the ledger journal.
    let ledger = ledger_batch(0x87);
    append(dir, &lock, MasterRecord::Batch(ledger.clone())).unwrap();
    let _recorded = append_intent(dir, &lock, intent_for(dir, &ledger, &before)).unwrap();
    assert!(read_journal(dir).unwrap().dispatched(&ledger.batch_id));
    // It does not block the stock item; the stock item's open dispatch blocks
    // a second stock batch of that name.
    let (first, second) = (stock_batch(0x88), stock_batch(0x89));
    for batch in [&first, &second] {
        append(dir, &lock, JournalRecord::Batch(batch.clone())).unwrap();
    }
    let intent = intent_for(dir, &first, &before);
    let _recorded = append_kind_intent::<StockItemBatch>(dir, &lock, intent).unwrap();
    let intent = intent_for(dir, &second, &before);
    assert_eq!(
        append_kind_intent::<StockItemBatch>(dir, &lock, intent).unwrap_err(),
        MasterLedgerError::OpenDispatchOverlaps
    );
}

#[test]
fn the_stock_journal_is_a_master_file() {
    assert!(is_master_file("agent-master-stock-item.jsonl"));
    assert!(!is_master_file("agent-master-stock-items.jsonl"));
}

/// Each production file under `src` whose code lines (not `//` lines) hold
/// `needle`, with the count: the pattern of `tally/dispatch_license_tests.rs`.
fn callers(needle: &str) -> Vec<(String, usize)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let (mut pending, mut calling) = (vec![root.clone()], Vec::new());
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            let name = path
                .strip_prefix(&root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            if path.is_dir() {
                pending.push(path);
            } else if name.ends_with(".rs") && !name.ends_with("_tests.rs") {
                let text = std::fs::read_to_string(&path).unwrap();
                let calls = text
                    .lines()
                    .map(str::trim_start)
                    .filter(|line| !line.starts_with("//") && line.contains(needle))
                    .count();
                if calls > 0 {
                    calling.push((name, calls));
                }
            }
        }
    }
    calling.sort();
    calling
}

#[test]
fn only_the_basis_judges_a_stock_item_read_back() {
    let basis = ("agent_master_stock_item_journal.rs".to_string(), 1);
    // The second is `VerifiedItem::new`'s own judgement.
    let verified = ("tally/stock_item_verified.rs".to_string(), 1);
    assert_eq!(callers("ReadBackVerdict::of("), [basis.clone(), verified]);
    assert_eq!(callers("VerifiedItem::new("), [basis]);
}

/// The stock intent writer on tokens the stock recheck minted in the queue, on
/// the constructed answers of `coordinator::passing_answers`, spending stock
/// approvals asked, held and taken as the approvals holder takes them.
mod writer {
    use super::super::super::super::approval::{PostApprovals, Redemption};
    use super::super::super::super::master_post::recheck_stock_item_for_dispatch;
    use super::super::super::super::master_post::tests::coordinator::{
        approved_stock, import_answer, imports, minted, passing_answers, post_stock_through,
        rebound_prepared, shape_identity,
    };
    use super::super::super::super::master_review::tests::{stock_approvals, taken_stock_approval};
    use super::super::super::TEST_FAULTS;
    use super::*;
    use crate::tally::approved_import::{BeforeDispatchError, DispatchLicense, PostKind};

    const A: u16 = 0xa1;
    const B: u16 = 0xb2;

    fn id(serial: u16) -> &'static str {
        crate::synthetic_batch_id(serial)
    }

    /// The batch of the rebound draft, as a stock tool would record it.
    fn shape_batch(serial: u16) -> StockItemBatch {
        let prepared = rebound_prepared(&shape_identity());
        StockItemBatch::new(id(serial).into(), prepared.company(), prepared.writes()).unwrap()
    }

    /// A data folder holding `batches`, none dispatched.
    fn folder(batches: &[&StockItemBatch]) -> tempfile::TempDir {
        let directory = tempfile::tempdir().unwrap();
        let dir = directory.path();
        for batch in batches {
            append(dir, &lock(), JournalRecord::Batch((*batch).clone())).unwrap();
        }
        directory
    }

    /// An approvals holder over a folder of its own, so the note a lapse
    /// leaves is not in the journal's folder. It holds one approval at a time.
    fn holder() -> (tempfile::TempDir, PostApprovals) {
        let directory = tempfile::tempdir().unwrap();
        let approvals = stock_approvals(directory.path());
        (directory, approvals)
    }

    /// Whether the journal in `dir` shows `batch` dispatched, and whether its
    /// sidecar is there.
    fn written(dir: &Path, batch: &StockItemBatch) -> (bool, bool) {
        let journal = read_kind_journal::<StockItemBatch>(dir).unwrap();
        let sidecar = dir.join(format!("master-pre-image-{}.json", batch.batch_id));
        (journal.dispatched(&batch.batch_id), sidecar.exists())
    }

    /// Records `batch` on `passed`, spending `taken`.
    fn record(
        dir: &Path,
        batch: &StockItemBatch,
        passed: StockItemRecheckPassed,
        taken: &Redemption<'_>,
    ) -> Result<RecordedIntent, StockIntentRefusal> {
        record_stock_item_intent(dir, &lock(), batch, passed, taken)
    }

    /// The approval `taken` was spent: its slot is empty, so spending it again
    /// is refused.
    fn spent(taken: &Redemption<'_>) -> bool {
        taken.spend() == Err(UnderLockRefusal::ApprovalRevoked)
    }

    /// The approval `taken` was not spent: spending it now succeeds. This
    /// spends it.
    fn unspent(taken: &Redemption<'_>) -> bool {
        taken.spend() == Ok(())
    }

    #[tokio::test]
    async fn the_writer_records_one_intent_that_licenses_its_own_batch_only() {
        let batch = shape_batch(A);
        let directory = folder(&[&batch]);
        let dir = directory.path();
        let (_held, approvals) = holder();
        let taken = taken_stock_approval(&approvals, id(A)).await;
        let recorded = record(dir, &batch, minted(id(A)).await, &taken).unwrap();
        assert!(spent(&taken), "the approval is spent once");
        assert_eq!(written(dir, &batch), (true, true));
        assert_eq!(recorded.batch_id(), id(A));
        assert!(recorded.licenses(&approved_stock(id(A)).await));
        let journal = read_kind_journal::<StockItemBatch>(dir).unwrap();
        StockItemReadBackBasis::from_journal(dir, &journal, id(A), &shape_identity())
            .expect("the basis builds for the recorded batch");
        // A second token and approval for the same batch: the journal already
        // holds its intent, and nothing is spent.
        let again_taken = taken_stock_approval(&approvals, id(A)).await;
        let again = record(dir, &batch, minted(id(A)).await, &again_taken);
        assert_eq!(
            again.unwrap_err(),
            StockIntentRefusal::Refused(UnderLockRefusal::AlreadyAttempted)
        );
        assert!(unspent(&again_taken));
        // An intent of batch A licenses no approval of batch B.
        let (a, b) = (shape_batch(A), shape_batch(B));
        let other = folder(&[&a, &b]);
        let (_held, approvals) = holder();
        let taken = taken_stock_approval(&approvals, id(A)).await;
        let recorded = record(other.path(), &a, minted(id(A)).await, &taken).unwrap();
        assert!(!recorded.licenses(&approved_stock(id(B)).await));
    }

    #[tokio::test]
    async fn a_token_for_another_batch_or_digest_writes_nothing() {
        let b = shape_batch(B);
        let directory = folder(&[&b]);
        let (_held, approvals) = holder();
        // The approval is batch B's own: only the token's batch differs.
        let taken = taken_stock_approval(&approvals, id(B)).await;
        let refused = record(directory.path(), &b, minted(id(A)).await, &taken);
        assert_eq!(refused.unwrap_err(), StockIntentRefusal::OtherBatch);
        assert_eq!(written(directory.path(), &b), (false, false));
        assert!(unspent(&taken));
        // A hand edit: batch A's stored digest made another, so only the
        // digest differs from the token's.
        let mut r1 = shape_batch(A);
        r1.batch_sha256 = "f".repeat(64);
        let directory = folder(&[&r1]);
        let taken = taken_stock_approval(&approvals, id(A)).await;
        let refused = record(directory.path(), &r1, minted(id(A)).await, &taken);
        assert_eq!(refused.unwrap_err(), StockIntentRefusal::OtherDigest);
        assert_eq!(written(directory.path(), &r1), (false, false));
        assert!(unspent(&taken));
    }

    /// The token and the batch are A's, and the approval is batch B's: refused
    /// as another batch before it is spent, and nothing is written.
    #[tokio::test]
    async fn an_approval_of_another_batch_writes_nothing_and_is_not_spent() {
        let a = shape_batch(A);
        let directory = folder(&[&a]);
        let (_held, approvals) = holder();
        let taken = taken_stock_approval(&approvals, id(B)).await;
        let refused = record(directory.path(), &a, minted(id(A)).await, &taken);
        assert_eq!(refused.unwrap_err(), StockIntentRefusal::OtherBatch);
        assert_eq!(written(directory.path(), &a), (false, false));
        assert!(unspent(&taken));
    }

    #[tokio::test]
    async fn an_open_dispatch_of_the_name_or_a_refused_spend_writes_nothing() {
        let (a, b) = (shape_batch(A), shape_batch(B));
        let directory = folder(&[&a, &b]);
        let dir = directory.path();
        let (_held_a, approvals_a) = holder();
        let (_held_b, approvals_b) = holder();
        let taken_a = taken_stock_approval(&approvals_a, id(A)).await;
        let taken_b = taken_stock_approval(&approvals_b, id(B)).await;
        let _recorded = record(dir, &a, minted(id(A)).await, &taken_a).unwrap();
        let overlap = record(dir, &b, minted(id(B)).await, &taken_b);
        assert_eq!(
            overlap.unwrap_err(),
            StockIntentRefusal::Refused(UnderLockRefusal::MasterOverlapsOpenDispatch)
        );
        assert!(spent(&taken_a));
        assert!(unspent(&taken_b));
        assert_eq!(written(dir, &b), (false, false));
        // A spend that refuses writes no sidecar and no intent: the approval
        // was withdrawn after it was taken, as a cancelled call withdraws it.
        let fresh = folder(&[&a]);
        let (_held, approvals) = holder();
        let taken = taken_stock_approval(&approvals, id(A)).await;
        approvals.revoke_of(PostKind::Masters, id(A), "request_cancelled");
        let revoked = record(fresh.path(), &a, minted(id(A)).await, &taken);
        assert_eq!(
            revoked.unwrap_err(),
            StockIntentRefusal::Refused(UnderLockRefusal::ApprovalRevoked)
        );
        assert_eq!(written(fresh.path(), &a), (false, false));
    }

    #[tokio::test]
    async fn a_sidecar_written_before_a_refused_append_is_removed() {
        let a = shape_batch(A);
        let directory = folder(&[&a]);
        let passed = minted(id(A)).await;
        let (_held, approvals) = holder();
        let taken = taken_stock_approval(&approvals, id(A)).await;
        let refused = TEST_FAULTS.sync_scope(
            std::cell::RefCell::new(vec!["master_dispatch_intent"]),
            || record(directory.path(), &a, passed, &taken),
        );
        assert_eq!(
            refused.unwrap_err(),
            StockIntentRefusal::Refused(UnderLockRefusal::MasterJournalRefused)
        );
        assert!(spent(&taken));
        assert_eq!(written(directory.path(), &a), (false, false));
    }

    /// The queue's stock recheck and the stock intent writer, end to end: the
    /// intent is recorded before the POST, and the one import the double
    /// receives is the approved bytes.
    #[tokio::test]
    async fn the_queue_records_the_stock_intent_before_it_posts_the_approved_bytes() {
        let a = shape_batch(A);
        let directory = folder(&[&a]);
        let dir = directory.path();
        let (_held, approvals) = holder();
        let taken = taken_stock_approval(&approvals, id(A)).await;
        let mut plans = passing_answers();
        let reads = plans.len();
        plans.push(import_answer());
        let (result, observed, at_dispatch) =
            post_stock_through(id(A), plans, recheck_stock_item_for_dispatch, |passed| {
                assert_eq!(
                    written(dir, &a),
                    (false, false),
                    "nothing before the writer"
                );
                record_stock_item_intent(dir, &lock(), &a, passed, &taken)
                    .map_err(|refusal| BeforeDispatchError::Other(format!("{refusal:?}")))
            })
            .await;
        result.expect("the post is sent");
        assert_eq!(at_dispatch, Some(reads), "the writer ran before the POST");
        assert_eq!(written(dir, &a), (true, true));
        assert!(spent(&taken));
        let approved = approved_stock(id(A)).await;
        let sent = imports(&observed);
        assert_eq!(sent.len(), 1, "exactly one import");
        assert!(approved.sends_exactly(&sent[0]), "the approved bytes");
        assert_eq!(observed.len(), reads + 1);
    }
}
