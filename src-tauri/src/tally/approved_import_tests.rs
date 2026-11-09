use super::*;
use crate::tally::ledger_create_item::LedgerCreateItem;

// The production constructor of the masters kind is `confirm_masters`, which
// asks the person first. This module also builds one without a dialog, from the
// same admission, for the queue's tests.

impl ApprovedImport {
    /// The masters twin of `approved_for_test`: an approval without a dialog,
    /// carrying the same seam marker. `pub(crate)` because the agent module's
    /// queue tests build the approval too.
    pub(crate) fn approved_masters_for_test(
        batch: impl Into<LedgerCreateBatch>,
        company: &VerifiedCompanyIdentity,
        company_marks_request: AgentReadRequest,
    ) -> Self {
        std::hint::black_box(test_seam::SEAM_MARKER);
        Self {
            kind: ApprovedKind::Masters(Box::new(
                MasterAdmission::new(batch.into(), company)
                    .expect("static read profiles are admitted"),
            )),
            company_marks_request,
        }
    }
}

#[test]
fn import_name_scope_must_select_one_observed_company() {
    // Local identity-admission test. These are not claimed Tally responses.
    let original = bridge_tally_protocol::TallyCompany {
        name: "Synthetic Book".into(),
        guid: Some("00000000-0000-4000-8000-000000000001".into()),
        company_number: Some("1".into()),
        books_from: Some("20260401".into()),
    };
    let identity = super::super::VerifiedCompanyIdentity::from_observed_companies(
        original.name.clone(),
        crate::tally::test_company_guid(&original.guid.clone().unwrap()),
        "1".into(),
        "20260401".into(),
        std::slice::from_ref(&original),
    )
    .unwrap();
    assert!(require_unique_company_scope(std::slice::from_ref(&original), &identity).is_ok());
    for name in ["Synthetic Book", " synthetic book "] {
        let mut other = original.clone();
        other.name = name.into();
        other.guid = Some("00000000-0000-4000-8000-000000000002".into());
        assert_eq!(
            require_unique_company_scope(&[original.clone(), other], &identity),
            Err(AmbiguousImportCompany)
        );
    }
}

/// The queue's Education recheck covers every voucher's date, not only the
/// first, and an empty list approves nothing.
#[test]
fn every_voucher_date_must_pass_the_education_boundary() {
    let date = |value: &str| TallyDate::parse(value.to_string()).unwrap();
    let education = DateBoundaryProfile::EducationRestricted;
    assert!(every_date_accepted(education, &[date("20250401")]));
    assert!(every_date_accepted(
        education,
        &[date("20250401"), date("20250502"), date("20250531")]
    ));
    // The second voucher's date fails, so the batch fails.
    assert!(!every_date_accepted(
        education,
        &[date("20250401"), date("20250415")]
    ));
    assert!(!every_date_accepted(
        education,
        &[date("20250415"), date("20250401")]
    ));
    assert!(every_date_accepted(
        DateBoundaryProfile::ModeAgnostic,
        &[date("20250401"), date("20250415")]
    ));
    assert!(!every_date_accepted(DateBoundaryProfile::ModeAgnostic, &[]));
}

// #687: the child's own answer, which the parent's tests replace with a script.

const NONCE: &str = "00000000-0000-4000-8000-000000000687";

/// What the post child wrote, and what it returned, for `input` and `dialog`.
fn child_answer(dialog: fn(VoucherCount, &str) -> bool, input: &str) -> (bool, Vec<u8>) {
    answer_as(POST_TOKEN_PREFIX, dialog, input)
}

fn answer_as(prefix: &str, dialog: fn(VoucherCount, &str) -> bool, input: &str) -> (bool, Vec<u8>) {
    let mut output = Vec::new();
    let approved = answer_with_token(prefix, dialog, input.as_bytes(), &mut output);
    (approved, output)
}

#[test]
fn a_declined_dialog_writes_no_token() {
    // The dialog records that it was shown, so the test cannot pass by the
    // input being refused before the decline it is about.
    static SHOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    let (approved, output) = child_answer(
        |_, _| {
            SHOWN.store(true, std::sync::atomic::Ordering::SeqCst);
            false
        },
        &format!("{NONCE}\n1\nPost one voucher"),
    );
    assert!(
        SHOWN.load(std::sync::atomic::Ordering::SeqCst),
        "the dialog was shown"
    );
    assert!(!approved);
    assert!(output.is_empty(), "{output:?}");
}

#[test]
fn the_core_answers_with_the_prefix_it_is_given() {
    let (approved, output) = answer_as(
        REVIEW_TOKEN_PREFIX,
        |_, _| true,
        &format!("{NONCE}\n1\nReview one voucher"),
    );
    assert!(approved);
    assert_eq!(
        output,
        format!("bridge-review-acknowledged:{NONCE}\n").into_bytes()
    );
}

#[test]
fn an_approved_dialog_writes_exactly_the_token_for_its_nonce() {
    let (approved, output) = child_answer(|_, _| true, &format!("{NONCE}\n1\nPost one voucher"));
    assert!(approved);
    assert_eq!(
        output,
        format!("bridge-post-approved:{NONCE}\n").into_bytes()
    );
}

#[test]
fn the_dialog_shows_the_preview_after_the_count_line() {
    let (approved, _) = child_answer(
        |_, preview| preview == "Post one voucher\nsecond line",
        &format!("{NONCE}\n1\nPost one voucher\nsecond line"),
    );
    assert!(approved);
}

/// The dialog is shown the count from the parent's count line (#746), which
/// its title and button name; the control shows another count is not taken
/// for it.
#[test]
fn the_dialog_is_shown_the_count_the_parent_sent() {
    fn three(count: VoucherCount, _: &str) -> bool {
        count == VoucherCount::new(3).unwrap()
    }
    assert!(child_answer(three, &format!("{NONCE}\n3\nPost 3 vouchers")).0);
    assert!(!child_answer(three, &format!("{NONCE}\n2\nPost 2 vouchers")).0);
}

#[test]
fn an_input_not_in_the_parents_shape_shows_no_dialog_and_writes_nothing() {
    let oversized = "x".repeat(MAX_PREVIEW_BYTES + 1);
    for input in [
        String::new(),
        NONCE.to_string(),
        format!("{NONCE}\n"),
        format!("{NONCE}\n1\n"),
        format!("{NONCE}\nPost one voucher"),
        format!("{NONCE}\n0\nPost one voucher"),
        "not-a-uuid\n1\nPost one voucher".to_string(),
        format!("{NONCE}\n1\nPost\0one voucher"),
        format!("{NONCE}\n1\n{oversized}"),
    ] {
        let (approved, output) = child_answer(
            |_, _| panic!("no dialog is shown for input the parent never sends"),
            &input,
        );
        assert!(!approved, "{input:?}");
        assert!(output.is_empty(), "{input:?}");
    }
}

// A master post is a second kind of `ApprovedImport`. These tests build it
// through the test-only constructor, since no production path can.

const MASTERS_LAB: &str = "BRIDGE MASTERS LAB";
fn masters_guid() -> &'static str {
    crate::synthetic_guid(1)
}

fn masters_identity() -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture(MASTERS_LAB, masters_guid())
}

fn masters_item(name: &str) -> LedgerCreateItem {
    LedgerCreateItem::new(
        super::super::ledger_create_item::MasterName::new(name).unwrap(),
        super::super::ledger_create_item::AdmittedLedgerKind::Suspense,
    )
}

fn marks_request() -> AgentReadRequest {
    AgentReadRequest::parse(
        bridge_tally_protocol::xml_read_profiles::ReadOnlyProfile::CompanyListV2.render(),
    )
    .unwrap()
}

fn approved_masters(name: &str) -> ApprovedImport {
    ApprovedImport::approved_masters_for_test(
        masters_item(name),
        &masters_identity(),
        marks_request(),
    )
}

fn masters_of(approved: &ApprovedImport) -> &MasterAdmission {
    match &approved.kind {
        ApprovedKind::Masters(masters) => masters,
        ApprovedKind::Vouchers(_) => panic!("expected the masters kind"),
    }
}

/// What the POST sends is what was rendered once, and its digest is the one the
/// intent will record.
#[test]
fn a_master_post_sends_the_bytes_rendered_once_whose_digest_it_records() {
    let approved = approved_masters("Queue Test Suspense");
    let rendered = &masters_of(&approved).rendered;
    assert_eq!(approved.xml(), rendered.xml());
    assert_eq!(
        bridge_tally_protocol::sha256_hex(approved.xml().as_bytes()),
        rendered.sha256()
    );
    assert_eq!(
        rendered,
        &masters_item("Queue Test Suspense").render_import(&masters_identity()),
        "the same item and company render the same request"
    );
    assert!(approved.xml().contains("Queue Test Suspense"));
}

#[test]
fn a_master_posts_index_reads_are_the_ledger_catalogue_and_group_snapshot_for_its_company() {
    let approved = approved_masters("Queue Test Suspense");
    let masters = masters_of(&approved);
    let company =
        bridge_tally_protocol::xml_read_profiles::ValidatedCompanyName::new(MASTERS_LAB).unwrap();
    assert_eq!(
        masters.ledger_index_request().into_xml(),
        bridge_tally_protocol::xml_read_profiles::ReadOnlyProfile::StandardLedgerCatalogV3 {
            company: &company
        }
        .render()
    );
    assert_eq!(
        masters.group_index_request().into_xml(),
        bridge_tally_protocol::native_outstandings::render_native_group_snapshot_request(
            MASTERS_LAB
        )
    );
    assert_ne!(
        masters.ledger_index_request().into_xml(),
        masters.group_index_request().into_xml()
    );
    assert_eq!(masters.company(), &masters_identity());
}

#[test]
fn a_master_post_refuses_education_and_accepts_a_mode_that_is_not() {
    let approved = approved_masters("Queue Test Suspense");
    assert_eq!(
        approved.require_boundary_profile(DateBoundaryProfile::EducationRestricted),
        Err(ApprovedImportAdmissionError::MasterWritesNeedLicensedMode)
    );
    assert_eq!(
        approved.require_boundary_profile(DateBoundaryProfile::ModeAgnostic),
        Ok(())
    );
}

/// A master post has no voucher count at all: whether an approval is for the
/// right kind is `post_kind`, which differs between the kinds (the voucher
/// side is held in the approval slot's tests).
#[test]
fn a_master_post_has_no_voucher_count_and_its_own_kind() {
    let masters = approved_masters("Queue Test Suspense");
    assert_eq!(masters.voucher_count(), None);
    assert_eq!(masters.post_kind(), PostKind::Masters);
}

use test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};

const LEDGER_PREVIEW: &str = "Create ledger \"Queue Test Suspense\" under Suspense A/c";

async fn confirm_masters_under(
    scripted: &ScriptedApproval,
    name: &str,
) -> Result<ApprovedImport, String> {
    SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            ApprovedImport::confirm_masters(
                masters_item(name).into(),
                &masters_identity(),
                LEDGER_PREVIEW,
                marks_request(),
            ),
        )
        .await
}

/// The person's approval makes the masters kind, from the same admission the
/// queue tests build, and the one-ledger dialog is asked with the preview as
/// given. The post and review dialogs are not asked.
#[tokio::test]
async fn an_approved_ledger_create_holds_the_masters_kind() {
    let scripted = ScriptedApproval::approving();
    let approved = confirm_masters_under(&scripted, "Queue Test Suspense")
        .await
        .unwrap();
    assert_eq!(approved.post_kind(), PostKind::Masters);
    assert_eq!(approved.voucher_count(), None);
    assert_eq!(
        masters_of(&approved).batch(),
        &masters_item("Queue Test Suspense").into()
    );
    assert_eq!(masters_of(&approved).company(), &masters_identity());
    assert_eq!(
        approved.xml(),
        approved_masters("Queue Test Suspense").xml(),
        "the same item and company render the same bytes"
    );
    assert_eq!(scripted.masters(), vec![LEDGER_PREVIEW.to_string()]);
    assert!(scripted.previews().is_empty());
    assert!(scripted.reviews().is_empty());
}

/// A decline, and an unscoped call, approve nothing and hold no kind.
#[tokio::test]
async fn a_declined_or_unscoped_ledger_create_approves_nothing() {
    let declining = ScriptedApproval::declining();
    assert_eq!(
        confirm_masters_under(&declining, "Queue Test Suspense")
            .await
            .err(),
        Some("master_approval_declined".to_string())
    );
    assert_eq!(declining.masters(), vec![LEDGER_PREVIEW.to_string()]);
    assert_eq!(
        ApprovedImport::confirm_masters(
            masters_item("Queue Test Suspense").into(),
            &masters_identity(),
            LEDGER_PREVIEW,
            marks_request(),
        )
        .await
        .err(),
        Some("master_approval_declined".to_string())
    );
}

/// A voucher dialog's scripted answer is not a master approval: the post and
/// review dialogs record to their own lists, and a ledger create records to
/// its own.
#[tokio::test]
async fn the_scripted_dialogs_record_apart() {
    let scripted = ScriptedApproval::approving();
    SCRIPTED_APPROVAL
        .scope(scripted.clone(), async {
            super::approve(VoucherCount::new(1).unwrap(), "Post")
                .await
                .unwrap();
            super::approve_review(
                super::ReviewKind::Review,
                VoucherCount::new(1).unwrap(),
                "Review",
            )
            .await
            .unwrap();
            super::approve_masters(MastersPrompt {
                text: "Create",
                count: VoucherCount::new(1).unwrap(),
            })
            .await
            .unwrap();
        })
        .await;
    assert_eq!(scripted.previews(), vec!["Post".to_string()]);
    assert_eq!(scripted.reviews(), vec!["Review".to_string()]);
    assert_eq!(scripted.masters(), vec!["Create".to_string()]);
}

/// A master dialog on its own task, as a held approval runs it, answers as a
/// voucher dialog does.
#[tokio::test]
async fn a_ledger_create_dialog_can_be_held_on_its_own_task() {
    let scripted = ScriptedApproval::approving();
    let pending = SCRIPTED_APPROVAL
        .scope(scripted.clone(), async {
            PendingPostApproval::ask_masters(
                masters_item("Queue Test Suspense").into(),
                masters_identity(),
                LEDGER_PREVIEW.to_string(),
                marks_request(),
            )
        })
        .await;
    let (answer, answered) = pending
        .answer_within(Duration::from_secs(5))
        .await
        .ok()
        .expect("answered");
    assert!(answered.approved);
    assert_eq!(answer.unwrap().post_kind(), PostKind::Masters);
    assert_eq!(scripted.masters(), vec![LEDGER_PREVIEW.to_string()]);

    let declined = ScriptedApproval::declining();
    let pending = SCRIPTED_APPROVAL
        .scope(declined, async {
            PendingPostApproval::ask_masters(
                masters_item("Queue Test Suspense").into(),
                masters_identity(),
                LEDGER_PREVIEW.to_string(),
                marks_request(),
            )
        })
        .await;
    let (answer, answered) = pending
        .answer_within(Duration::from_secs(5))
        .await
        .ok()
        .expect("answered");
    assert!(!answered.approved);
    assert_eq!(answer.err(), Some("master_approval_declined".to_string()));
}

// The one-ledger child's own answer.

#[test]
fn an_approved_ledger_dialog_writes_exactly_its_own_token() {
    let (approved, output) = answer_as(
        MASTERS_TOKEN_PREFIX,
        |_, _| true,
        &format!("{NONCE}\n1\nCreate one ledger"),
    );
    assert!(approved);
    assert_eq!(
        output,
        format!("bridge-create-ledger:{NONCE}\n").into_bytes()
    );
    assert_ne!(MASTERS_TOKEN_PREFIX, POST_TOKEN_PREFIX);
    assert_ne!(MASTERS_TOKEN_PREFIX, REVIEW_TOKEN_PREFIX);
    assert!(!MASTERS_TOKEN_PREFIX.starts_with(POST_TOKEN_PREFIX));
    assert!(!MASTERS_TOKEN_PREFIX.starts_with(REVIEW_TOKEN_PREFIX));
}

#[test]
fn a_declined_ledger_dialog_writes_no_token() {
    static SHOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    let (approved, output) = answer_as(
        MASTERS_TOKEN_PREFIX,
        |_, _| {
            SHOWN.store(true, std::sync::atomic::Ordering::SeqCst);
            false
        },
        &format!("{NONCE}\n1\nCreate one ledger"),
    );
    assert!(SHOWN.load(std::sync::atomic::Ordering::SeqCst));
    assert!(!approved);
    assert!(output.is_empty(), "{output:?}");
}

/// The ledger dialog is about at most the batch limit: a larger count is not
/// the parent's shape for it, so it shows nothing and does not approve. (It
/// returns before any window opens.)
#[test]
fn the_ledger_dialog_is_not_shown_above_the_batch_limit() {
    // Counts the box would show are not called here: they open a real box.
    for count in [MAX_LEDGERS_PER_BATCH + 1, 200] {
        assert!(!show_create_ledger(
            VoucherCount::new(count).unwrap(),
            "Create"
        ));
    }
}

/// The words the person reads: the title asks the question, and the button
/// names what it does and is not the decline's label.
#[test]
fn the_ledger_dialog_words_name_what_the_button_does() {
    assert_eq!(MASTERS_TITLE, "ComplyEaze Bridge — create one ledger?");
    let (_, button) = masters_words(ONE_ITEM).unwrap();
    assert_eq!(button, "Create ledger");
    assert_ne!(button, "Cancel");
}

/// One ledger keeps the words its box was measured with; a batch is named by
/// its count; above the batch limit no box is shown.
#[test]
fn the_ledger_dialog_is_worded_by_its_count() {
    let words = |count: usize| masters_words(VoucherCount::new(count).unwrap());
    assert_eq!(
        words(1),
        Some((
            "ComplyEaze Bridge — create one ledger?".to_string(),
            "Create ledger".to_string()
        ))
    );
    assert_eq!(
        words(3),
        Some((
            "ComplyEaze Bridge — create 3 ledgers?".to_string(),
            "Create 3 ledgers".to_string()
        ))
    );
    assert!(words(MAX_LEDGERS_PER_BATCH).is_some());
    assert_eq!(words(MAX_LEDGERS_PER_BATCH + 1), None);
    assert_eq!(
        answer_effect(Shown::Masters, VoucherCount::new(3).unwrap()).0,
        "create these 3 ledgers in Tally"
    );
    assert_eq!(
        answer_effect(Shown::Masters, VoucherCount::new(1).unwrap()).0,
        "create this ledger in Tally"
    );
}
