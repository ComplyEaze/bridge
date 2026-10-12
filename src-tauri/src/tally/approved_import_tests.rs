use super::*;

// The only constructors of the masters kind. They live in this test module, not
// in the shipped file, so no production path can build one (AGENTS.md P2): the
// dialog slice adds the real constructor beside the masters dialog.

impl MasterAdmission {
    /// Renders the request once, with the two index reads for the company.
    fn new(
        item: LedgerCreateItem,
        company: &VerifiedCompanyIdentity,
    ) -> Result<Self, super::super::agent_read_request::AgentReadRequestError> {
        use super::super::agent_read_request::AgentReadRequestError;
        use bridge_tally_protocol::xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName};
        let validated =
            ValidatedCompanyName::new(company.display_name()).map_err(|_| AgentReadRequestError)?;
        let rendered = item.render_import(company);
        Ok(Self {
            item,
            rendered,
            ledger_index_request: AgentReadRequest::parse(
                ReadOnlyProfile::StandardLedgerCatalogV3 {
                    company: &validated,
                }
                .render(),
            )?,
            group_index_request: AgentReadRequest::parse(
                bridge_tally_protocol::native_outstandings::render_native_group_snapshot_request(
                    company.display_name(),
                ),
            )?,
            company: company.clone(),
        })
    }
}

impl ApprovedImport {
    /// The masters twin of `approved_for_test`: an approval without a dialog,
    /// carrying the same seam marker. `pub(crate)` because the agent module's
    /// queue tests build the approval too.
    pub(crate) fn approved_masters_for_test(
        item: LedgerCreateItem,
        company: &VerifiedCompanyIdentity,
        company_marks_request: AgentReadRequest,
    ) -> Self {
        std::hint::black_box(test_seam::SEAM_MARKER);
        Self {
            kind: ApprovedKind::Masters(Box::new(
                MasterAdmission::new(item, company).expect("static read profiles are admitted"),
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
        original.guid.clone().unwrap(),
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
const MASTERS_GUID: &str = "00000000-0000-4000-8000-000000000001";

fn masters_identity() -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture(MASTERS_LAB, MASTERS_GUID)
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
    let rendered = masters_of(&approved).rendered();
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

/// A master approval redeemed against a voucher binding can never match its
/// count: it is refused as a changed binding.
#[test]
fn a_master_post_counts_no_vouchers() {
    assert_eq!(approved_masters("Queue Test Suspense").voucher_count(), 0);
}

#[test]
fn the_masters_approval_does_not_exist_yet() {
    assert_eq!(
        ApprovedImport::confirm_masters().err(),
        Some(MasterApprovalUnavailable)
    );
    assert_eq!(
        MasterApprovalUnavailable.to_string(),
        "master_approval_unavailable"
    );
}
