//! Independent local approval. The model never supplies an approval boolean.
use super::agent_read_request::AgentReadRequest;
use super::ledger_create_item::{LedgerCreateItem, RenderedLedgerCreate};
use super::VerifiedCompanyIdentity;
use bridge_tally_core::master_name_index::{MasterKind, Via};
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::{
    outstandings_shared::DateBoundaryProfile, StandardLedgerCatalogBinding,
};
use std::{io::Read, num::NonZeroUsize, process::Stdio, time::Duration};
use tokio::io::AsyncWriteExt;

const MAX_PREVIEW_BYTES: usize = 8_000;

tokio::task_local! {
    /// Whether the post dialog of the task it is scoped to showed a box that did
    /// not fit the screen (#1832): set by [`note_box_did_not_fit`] when the
    /// dialog child exits with the status that says so, and read once by that
    /// task when the dialog ends. Scoped only by [`PendingPostApproval::ask`];
    /// anywhere else noting it does nothing.
    static BOX_DID_NOT_FIT: std::cell::Cell<bool>;
}

/// Record, for the dialog task this runs in, that its box did not fit.
fn note_box_did_not_fit() {
    let _ = BOX_DID_NOT_FIT.try_with(|noted| noted.set(true));
}
#[cfg(not(windows))]
const POST_LABEL: &str = "Post voucher";
/// The review dialog's positive button, as the person sees it. Windows shows
/// a Yes/No/Cancel message box, which has no custom labels.
#[cfg(not(windows))]
pub(crate) const REVIEW_BUTTON: &str = "I reviewed it";
#[cfg(windows)]
pub(crate) const REVIEW_BUTTON: &str = "Yes";
/// The invoice-stop release dialog's positive button on a Mac. Windows shows the
/// same Yes/No/Cancel box as the review.
#[cfg(not(windows))]
const RELEASE_BUTTON: &str = "Release the stop";
/// The settle dialog's positive button on a Mac (#1709). Windows shows the
/// same Yes/No/Cancel box as the review.
#[cfg(not(windows))]
const SETTLE_BUTTON: &str = "Close the batch";
/// What the review dialog's subprocess prints, followed by the nonce it was
/// given, when and only when the person chose the positive button.
const REVIEW_TOKEN_PREFIX: &str = "bridge-review-acknowledged:";
/// The same for the post dialog (#635). Distinct from the review's, so a
/// subprocess in one mode can never answer the other.
const POST_TOKEN_PREFIX: &str = "bridge-post-approved:";
/// The same for the one-ledger dialog. Distinct from both others, so a
/// subprocess in one mode can never answer another.
const MASTERS_TOKEN_PREFIX: &str = "bridge-create-ledger:";
/// The one-ledger dialog's title. The Windows box has no custom labels, so the
/// title asks the question there as well.
const MASTERS_TITLE: &str = "ComplyEaze Bridge — create one ledger?";
#[cfg(not(windows))]
const MASTERS_BUTTON: &str = "Create ledger";
/// What a one-ledger dialog is told as its count: always one item.
const ONE_ITEM: VoucherCount = VoucherCount(NonZeroUsize::MIN);
/// The same for the invoice-stop release dialog, distinct from both.
const RELEASE_TOKEN_PREFIX: &str = "bridge-stop-release-acknowledged:";
/// The same for the settle dialog (#1709), distinct from every other.
const SETTLE_TOKEN_PREFIX: &str = "bridge-batch-settle-acknowledged:";

/// How many vouchers a dialog asks about, so that its title and button say so
/// (#746). Never zero: a dialog about no voucher approves nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct VoucherCount(NonZeroUsize);

impl VoucherCount {
    pub(crate) fn new(count: usize) -> Option<Self> {
        NonZeroUsize::new(count).map(Self)
    }

    /// The count line the dialog child reads, in the one form the parent
    /// writes: decimal digits, with no sign and no leading zero.
    fn parse(line: &str) -> Option<Self> {
        let count = line.parse::<NonZeroUsize>().ok()?;
        (count.to_string() == line).then_some(Self(count))
    }

    /// `None` for one voucher, whose dialog keeps its single-voucher words,
    /// and the count for a batch.
    fn batch(self) -> Option<NonZeroUsize> {
        (self.0.get() > 1).then_some(self.0)
    }
}

/// What the person approved, and the all-company marks request every kind of
/// post is aimed with. The kind decides which reads the queue makes and which
/// recheck runs; it cannot be changed after construction.
#[derive(Clone)]
pub(crate) struct ApprovedImport {
    pub(super) kind: ApprovedKind,
    /// The all-company change marks, read last before the POST to confirm the
    /// aim and again right after it to see where the post went (#574).
    company_marks_request: AgentReadRequest,
}

/// The two requests one dispatch can carry. A voucher post and a master post
/// share the queue, the identity brackets, the aim and the single POST; they
/// differ only in what is read to admit them.
#[derive(Clone)]
pub(super) enum ApprovedKind {
    Vouchers(VoucherAdmission),
    /// Boxed: a master admission carries a rendered request and two reads, and
    /// every `ApprovedImport` moves by value through the approval path.
    Masters(Box<MasterAdmission>),
}

/// Which request an approval, a binding or a held request is for. Compared by
/// kind, never by a count that happens to differ between kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PostKind {
    Vouchers,
    Masters,
}

/// A voucher post's admission: every field the voucher path carried before
/// there was a second kind, moved here unchanged.
#[derive(Clone)]
pub(super) struct VoucherAdmission {
    xml: String,
    /// Every voucher's date, in batch order: the queue's Education recheck
    /// covers each of them.
    voucher_dates: Vec<TallyDate>,
    verification_request: AgentReadRequest,
    ledger_catalogue_request: AgentReadRequest,
    ledger_binding: StandardLedgerCatalogBinding,
    /// The group collection, for a Payment, Receipt or Contra: its legs'
    /// classification is re-derived from it inside the queue. A Journal has
    /// none, so it adds no group read to the queue.
    group_collection_request: Option<AgentReadRequest>,
    /// The company's Currency masters, re-read inside the queue: a post goes
    /// only into a book with exactly one (bridge#551).
    currency_request: AgentReadRequest,
    /// The reads an invoice's admission makes, for the queue to make again
    /// under its lock (#1337). `None` for every other voucher type. Boxed so
    /// that an approval an enum carries stays the size it was.
    invoice: Option<Box<InvoiceReadPlan>>,
}

/// The reads one Sales invoice's admission makes of its masters, fixed from the
/// saved invoice when the plan is made (#1337). The queue sends exactly these
/// inside the identity brackets of the POST and hands the answers back; what
/// they mean is the admission's own judgement, run on them there, so the rule
/// has one copy and a master added to the admission reaches the queue with it.
#[derive(Clone, Debug)]
pub(crate) struct InvoiceReadPlan {
    /// The company the reads are about, as the admission names it.
    pub(crate) company_name: String,
    /// The date the compliance listing is read as of.
    pub(crate) listing_as_of: TallyDate,
    /// What the queue may do with the marks and currencies it has read before
    /// it sends any of the reads below. The admission provides it, so the size
    /// limits and the currency are judged by the rule the build judges them by,
    /// ahead of the reads they spare.
    pub(crate) precheck: InvoicePrechecker,
    /// The company's own ledger count: sent only when the precheck asks for it,
    /// for a book over the master mark.
    pub(crate) ledger_count: AgentReadRequest,
    pub(crate) rates: AgentReadRequest,
    pub(crate) voucher_types: AgentReadRequest,
    pub(crate) number: AgentReadRequest,
    /// The same read for the invoice the journal held as the number's control.
    pub(crate) number_control: Option<AgentReadRequest>,
    pub(crate) registration: AgentReadRequest,
    /// The company object the HSN/SAC check reads once the classification
    /// admits, and the groups with their HSN/SAC rows. The build sends the
    /// groups only when a sales ledger's own row defers to them; the queue
    /// sends them whenever it sends the invoice's reads, and the judge parses
    /// an answer that is present whether or not it is needed, so a groups
    /// answer that cannot be read fails the post in the queue only.
    pub(crate) company_hsn: AgentReadRequest,
    pub(crate) group_hsn: AgentReadRequest,
}

/// What the queue has read when it asks whether an invoice's reads may go on.
pub(crate) struct InvoicePrecheckInput<'a> {
    /// The all-company marks, as the binding reads began.
    pub(crate) marks: &'a str,
    /// The company's Currency masters, as the queue read them.
    pub(crate) currencies: &'a str,
    /// The company's ledger count, once the precheck has asked for it.
    pub(crate) ledger_count: Option<&'a str>,
    pub(crate) company_name: &'a str,
    pub(crate) company_guid: &'a str,
}

/// The precheck's answer when it refuses nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InvoicePrecheck {
    /// The limits and the currency admit the book: send the rest of the plan.
    Admitted,
    /// The master mark is over the size gate's mark: the ledger count decides.
    NeedsLedgerCount,
}

/// A function of the invoice module that the queue runs before the plan's
/// reads, so that this layer does not call that one.
pub(crate) type InvoicePrechecker =
    fn(&InvoicePrecheckInput<'_>) -> Result<InvoicePrecheck, ApprovedImportAdmissionError>;

/// What the queue read for an [`InvoiceReadPlan`], or what the build has read
/// of one so far: each answer is `None` until it is read. The admission's judge
/// names the first answer it still needs.
#[derive(Debug, Default)]
pub(crate) struct InvoiceAnswers {
    /// The all-company marks, as the binding reads began.
    pub(crate) marks: Option<String>,
    pub(crate) ledger_count: Option<String>,
    pub(crate) listing: Option<super::runtime::PartyLedgerMasterListing>,
    pub(crate) rates: Option<String>,
    pub(crate) voucher_types: Option<String>,
    pub(crate) number: Option<String>,
    pub(crate) number_control: Option<String>,
    pub(crate) registration: Option<String>,
    pub(crate) company_hsn: Option<String>,
    pub(crate) group_hsn: Option<String>,
}

/// An invoice's plan and the answers the queue read for it.
pub(crate) struct QueuedInvoice<'a> {
    pub(crate) plan: &'a InvoiceReadPlan,
    pub(crate) answers: &'a InvoiceAnswers,
}

impl VoucherAdmission {
    /// The reads an invoice's admission makes, `None` for every other
    /// voucher type (#1337).
    pub(super) fn invoice_plan(&self) -> Option<&InvoiceReadPlan> {
        self.invoice.as_deref()
    }

    pub(super) fn verification_request(&self) -> AgentReadRequest {
        self.verification_request.clone()
    }

    pub(super) fn ledger_catalogue_request(&self) -> AgentReadRequest {
        self.ledger_catalogue_request.clone()
    }

    pub(super) fn ledger_binding(&self) -> &StandardLedgerCatalogBinding {
        &self.ledger_binding
    }

    pub(super) fn group_collection_request(&self) -> Option<AgentReadRequest> {
        self.group_collection_request.clone()
    }

    pub(super) fn currency_request(&self) -> AgentReadRequest {
        self.currency_request.clone()
    }
}

/// A master post's admission: one ledger `Create`, rendered once.
///
/// `rendered` is produced once, when the admission is built, and never again,
/// so the bytes the POST sends are the bytes whose digest the dispatch intent
/// records. The only production constructor is [`ApprovedImport::confirm_masters`],
/// which builds it before the dialog and holds it only if the person approves,
/// so no code holds an approved masters kind without that answer (P2).
#[derive(Clone)]
pub(super) struct MasterAdmission {
    item: LedgerCreateItem,
    rendered: RenderedLedgerCreate,
    /// The ledger name index read (the V3 catalogue request).
    ledger_index_request: AgentReadRequest,
    /// The group name index read (the group snapshot request).
    group_index_request: AgentReadRequest,
    company: VerifiedCompanyIdentity,
    /// The master journal batch this approval is for, once the tool binds it
    /// ([`ApprovedImport::bound_to_batch`]). A master request not bound to a
    /// batch is licensed by no intent.
    batch_id: Option<String>,
}

impl MasterAdmission {
    /// Renders the request once, with the two index reads for the company.
    fn new(
        item: LedgerCreateItem,
        company: &VerifiedCompanyIdentity,
    ) -> Result<Self, super::agent_read_request::AgentReadRequestError> {
        use super::agent_read_request::AgentReadRequestError;
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
            batch_id: None,
        })
    }

    pub(super) fn item(&self) -> &LedgerCreateItem {
        &self.item
    }

    pub(super) fn ledger_index_request(&self) -> AgentReadRequest {
        self.ledger_index_request.clone()
    }

    pub(super) fn group_index_request(&self) -> AgentReadRequest {
        self.group_index_request.clone()
    }

    pub(super) fn company(&self) -> &VerifiedCompanyIdentity {
        &self.company
    }
}

/// What the queue read for the last admission before the POST, by kind.
pub(crate) enum QueuedAdmission<'a> {
    Vouchers {
        first: &'a str,
        second: &'a str,
        catalogue: &'a str,
        groups: Option<&'a str>,
        currencies: &'a str,
        /// The all-company marks read as the binding reads began (#239).
        company_marks_at_binding: &'a str,
        company_marks: &'a str,
        ledger_binding: &'a StandardLedgerCatalogBinding,
        /// An invoice's masters, read again for the admission's judge (#1337).
        invoice: Option<QueuedInvoice<'a>>,
    },
    /// Each index is read twice, as the voucher absence reads are: the recheck
    /// requires the pairs byte-equal.
    Masters {
        /// The request being posted and the company it is for, taken from the
        /// approved request itself, so the recheck reads the name and the
        /// company from the bytes' own source and no caller supplies them.
        item: &'a LedgerCreateItem,
        company: &'a VerifiedCompanyIdentity,
        ledgers_first: &'a str,
        ledgers_second: &'a str,
        groups_first: &'a str,
        groups_second: &'a str,
        company_marks_at_binding: &'a str,
        company_marks: &'a str,
    },
}

/// Whether the profile accepts every voucher's date, and there is at least
/// one: an empty list approves nothing.
fn every_date_accepted(profile: DateBoundaryProfile, voucher_dates: &[TallyDate]) -> bool {
    !voucher_dates.is_empty()
        && voucher_dates
            .iter()
            .all(|voucher_date| profile.accepts_boundary(voucher_date))
}

impl ApprovedImport {
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn confirm_vouchers(
        xml: String,
        preview: &str,
        voucher_dates: Vec<TallyDate>,
        verification_request: AgentReadRequest,
        ledger_catalogue_request: AgentReadRequest,
        ledger_binding: StandardLedgerCatalogBinding,
        group_collection_request: Option<AgentReadRequest>,
        currency_request: AgentReadRequest,
        company_marks_request: AgentReadRequest,
    ) -> Result<Self, String> {
        let count = VoucherCount::new(voucher_dates.len())
            .ok_or_else(|| "voucher_date_invalid".to_string())?;
        approve(count, preview).await?;
        Ok(Self {
            kind: ApprovedKind::Vouchers(VoucherAdmission {
                xml,
                voucher_dates,
                verification_request,
                ledger_catalogue_request,
                ledger_binding,
                group_collection_request,
                currency_request,
                invoice: None,
            }),
            company_marks_request,
        })
    }

    /// The same approval, to be redeemed with an invoice's masters read again
    /// in the queue. A plain field of the approval and no part of what the
    /// person was shown: the queue's judge holds the plan to the invoice. Only
    /// a voucher approval carries a plan; a master approval is returned as it
    /// is, since the voucher post that calls this admits a voucher approval
    /// only.
    pub(crate) fn with_invoice(mut self, invoice: Option<InvoiceReadPlan>) -> Self {
        if let ApprovedKind::Vouchers(vouchers) = &mut self.kind {
            vouchers.invoice = invoice.map(Box::new);
        }
        self
    }

    /// The master post's approval: the one-ledger dialog, answered only by its
    /// own token. On the person's approval this holds `item` as the masters
    /// kind; any other answer is a typed refusal and holds nothing. The
    /// request and the two index reads are built before the dialog, so a
    /// fault there asks no one.
    ///
    /// `preview` is what the person is shown. It is built from `item` and
    /// `company` by the caller's review builder, and bound with them in the
    /// approval binding.
    pub(crate) async fn confirm_masters(
        item: LedgerCreateItem,
        company: &VerifiedCompanyIdentity,
        preview: &str,
        company_marks_request: AgentReadRequest,
    ) -> Result<Self, String> {
        let admission = MasterAdmission::new(item, company)
            .map_err(|_| "master_request_invalid".to_string())?;
        approve_masters(preview).await?;
        Ok(Self {
            kind: ApprovedKind::Masters(Box::new(admission)),
            company_marks_request,
        })
    }

    /// Which request this approval is for.
    pub(crate) fn post_kind(&self) -> PostKind {
        match &self.kind {
            ApprovedKind::Vouchers(_) => PostKind::Vouchers,
            ApprovedKind::Masters(_) => PostKind::Masters,
        }
    }

    /// The bytes the one POST sends, whatever the kind. For masters they are
    /// the bytes rendered once in [`MasterAdmission::new`], whose digest the
    /// dispatch intent records.
    pub(super) fn xml(&self) -> &str {
        match &self.kind {
            ApprovedKind::Vouchers(vouchers) => &vouchers.xml,
            ApprovedKind::Masters(masters) => masters.rendered.xml(),
        }
    }

    /// Whether the one POST would send exactly `xml`.
    pub(crate) fn sends_exactly(&self, xml: &str) -> bool {
        self.xml() == xml
    }

    /// Lowercase hex digest of the bytes the one POST sends, as a master
    /// dispatch intent records them.
    pub(crate) fn xml_sha256(&self) -> String {
        bridge_tally_protocol::sha256_hex(self.xml().as_bytes())
    }

    /// This approval bound to the master journal batch it was taken for, so the
    /// intent that licenses its send must be that batch's. A voucher approval is
    /// returned as it is.
    #[must_use]
    pub(crate) fn bound_to_batch(mut self, batch_id: &str) -> Self {
        if let ApprovedKind::Masters(masters) = &mut self.kind {
            masters.batch_id = Some(batch_id.to_string());
        }
        self
    }

    /// The master journal batch this approval is bound to, if it is a master
    /// approval that was bound.
    pub(crate) fn master_batch_id(&self) -> Option<&str> {
        match &self.kind {
            ApprovedKind::Masters(masters) => masters.batch_id.as_deref(),
            ApprovedKind::Vouchers(_) => None,
        }
    }

    pub(super) fn company_marks_request(&self) -> AgentReadRequest {
        self.company_marks_request.clone()
    }

    /// Recheck the operator-approved request against the observed product and
    /// mode after the endpoint queue admits it. The mode can change while
    /// native approval waits. Vouchers: every date is one the mode accepts.
    /// Masters: the mode is not Education, which this path has not measured a
    /// master write in.
    pub(super) fn require_boundary_profile(
        &self,
        profile: DateBoundaryProfile,
    ) -> Result<(), ApprovedImportAdmissionError> {
        match &self.kind {
            ApprovedKind::Vouchers(vouchers) => {
                if every_date_accepted(profile, &vouchers.voucher_dates) {
                    Ok(())
                } else {
                    Err(ApprovedImportAdmissionError::EducationVoucherDateUnsupported)
                }
            }
            ApprovedKind::Masters(_) => match profile {
                DateBoundaryProfile::ModeAgnostic => Ok(()),
                DateBoundaryProfile::EducationRestricted => {
                    Err(ApprovedImportAdmissionError::MasterWritesNeedLicensedMode)
                }
            },
        }
    }

    #[cfg(test)]
    pub(super) fn approved_for_test(
        xml: String,
        voucher_date: TallyDate,
        ledger_catalogue_request: AgentReadRequest,
        ledger_binding: StandardLedgerCatalogBinding,
        currency_request: AgentReadRequest,
        company_marks_request: AgentReadRequest,
    ) -> Self {
        // Carries the seam marker so the shipped-binary scan also covers this
        // bypass (bridge#583).
        std::hint::black_box(test_seam::SEAM_MARKER);
        Self {
            kind: ApprovedKind::Vouchers(VoucherAdmission {
                xml,
                voucher_dates: vec![voucher_date],
                verification_request: AgentReadRequest::parse(
                    bridge_tally_protocol::xml_read_profiles::ReadOnlyProfile::CompanyListV2
                        .render(),
                )
                .expect("static read profile is admitted"),
                ledger_catalogue_request,
                ledger_binding,
                group_collection_request: None,
                currency_request,
                invoice: None,
            }),
            company_marks_request,
        }
    }
}

/// How long a post dialog stays open before it approves nothing: the limit
/// [`nonce_bound_dialog`] applies. Used only to tell a caller how much of it
/// remains; the dialog enforces its own.
const POST_DIALOG_LIMIT: Duration = Duration::from_secs(120);

/// When a post dialog was answered, and whether the answer approved: stamped
/// by the dialog's own task as it ends, so an approval's age runs from the
/// click, not from whichever call later collects it (#725). Both clocks are
/// kept: the monotonic one does not advance while the machine sleeps.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Answered {
    pub(crate) approved: bool,
    pub(crate) at: std::time::Instant,
    pub(crate) at_wall: std::time::SystemTime,
}

impl Answered {
    fn now(approved: bool) -> Self {
        Self {
            approved,
            at: std::time::Instant::now(),
            at_wall: std::time::SystemTime::now(),
        }
    }
}

/// A post dialog left open after the MCP call that asked it returned (#725).
/// It is [`ApprovedImport::confirm_vouchers`] itself, unchanged, run on its own task so
/// that the dialog's time limit and its child's exit are observed while no call
/// is waiting. When its box did not fit the screen (#1832) the task also runs
/// the one note the asking call gave it, before the answer is stamped. Dropping this aborts that task, which drops the dialog child:
/// `kill_on_drop` closes the dialog, and a token it prints afterwards is never
/// read, so a late click approves nothing.
pub(crate) struct PendingPostApproval {
    task: tokio::task::JoinHandle<Result<ApprovedImport, String>>,
    /// The answer and, for a refusal, its code: one stamp, set once.
    answered: std::sync::Arc<std::sync::OnceLock<(Answered, Option<String>)>>,
    started: std::time::Instant,
}

impl PendingPostApproval {
    /// Ask the person in a dialog that may outlive the calling call:
    /// [`ApprovedImport::confirm_vouchers`] with exactly these arguments, on its own
    /// task. Nothing else can be run there but the masters twin below and the one
    /// note of a box that did not fit, which the caller gives and only the dialog
    /// task calls.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn ask(
        xml: String,
        preview: String,
        voucher_dates: Vec<TallyDate>,
        verification_request: AgentReadRequest,
        ledger_catalogue_request: AgentReadRequest,
        ledger_binding: StandardLedgerCatalogBinding,
        group_collection_request: Option<AgentReadRequest>,
        currency_request: AgentReadRequest,
        company_marks_request: AgentReadRequest,
        on_box_did_not_fit: Option<Box<dyn FnOnce() + Send>>,
    ) -> Self {
        Self::stamped(
            async move {
                ApprovedImport::confirm_vouchers(
                    xml,
                    &preview,
                    voucher_dates,
                    verification_request,
                    ledger_catalogue_request,
                    ledger_binding,
                    group_collection_request,
                    currency_request,
                    company_marks_request,
                )
                .await
            },
            on_box_did_not_fit,
        )
    }

    /// The same for the one-ledger dialog: [`ApprovedImport::confirm_masters`]
    /// with exactly these arguments, on its own task. Its box is measured as
    /// the post's is, but a box that does not fit is refused with nothing to
    /// note: the one-ledger text has no shorter form.
    pub(crate) fn ask_masters(
        item: LedgerCreateItem,
        company: VerifiedCompanyIdentity,
        preview: String,
        company_marks_request: AgentReadRequest,
    ) -> Self {
        Self::stamped(
            async move {
                ApprovedImport::confirm_masters(item, &company, &preview, company_marks_request)
                    .await
            },
            None,
        )
    }

    /// Run `confirm` on its own task, stamping its answer as it ends, and tell
    /// `on_box_did_not_fit` first if the box it showed did not fit the screen.
    fn stamped(
        confirm: impl std::future::Future<Output = Result<ApprovedImport, String>> + Send + 'static,
        on_box_did_not_fit: Option<Box<dyn FnOnce() + Send>>,
    ) -> Self {
        let answered = std::sync::Arc::new(std::sync::OnceLock::new());
        let stamp = std::sync::Arc::clone(&answered);
        let dialog = async move {
            let (answer, too_tall) = BOX_DID_NOT_FIT
                .scope(std::cell::Cell::new(false), async {
                    let answer = confirm.await;
                    (answer, BOX_DID_NOT_FIT.with(std::cell::Cell::get))
                })
                .await;
            // Told before the answer is stamped, so a call that reads the
            // refusal finds the fact already recorded.
            if let (true, Some(note)) = (too_tall, on_box_did_not_fit) {
                note();
            }
            // Stamped before the task ends, never after: a task seen finished
            // with no stamp is one that ended without an answer. A refusal's
            // code is stamped with it, so it can be read without the task.
            let _ = stamp.set((
                Answered::now(answer.is_ok()),
                answer.as_ref().err().cloned(),
            ));
            answer
        };
        Self {
            task: tokio::spawn(carry_approval_scope(dialog)),
            answered,
            started: std::time::Instant::now(),
        }
    }

    /// The person's answer and when it was given, if it arrives within
    /// `budget`, or the dialog back while it is still open. A task that ended
    /// without an answer (aborted or panicked) approves nothing.
    pub(crate) async fn answer_within(
        mut self,
        budget: Duration,
    ) -> Result<(Result<ApprovedImport, String>, Answered), Self> {
        match tokio::time::timeout(budget, &mut self.task).await {
            Err(_) => Err(self),
            Ok(Ok(answer)) => {
                let at = self
                    .answered()
                    .unwrap_or_else(|| Answered::now(answer.is_ok()));
                Ok((answer, at))
            }
            Ok(Err(_)) => Ok((
                Err("import_approval_unavailable".into()),
                Answered::now(false),
            )),
        }
    }

    /// Whether the dialog has been answered, and how, without waiting.
    pub(crate) fn answered(&self) -> Option<Answered> {
        self.answered.get().map(|(answered, _)| *answered)
    }

    /// Why the dialog approved nothing, once it has ended without an approval:
    /// the stamped refusal code, or `import_approval_unavailable` for a task
    /// that ended with no answer, as [`Self::answer_within`] reports it.
    /// `None` while the dialog is open, and for an approval.
    pub(crate) fn refusal(&self) -> Option<String> {
        // Finished is read before the stamp: the stamp is set before the task
        // ends, so a task seen finished with no stamp truly ended unanswered.
        // Read the other way, a dialog stamped and finished between the two
        // reads would drop a real approval as unavailable.
        let finished = self.task.is_finished();
        match self.answered.get() {
            Some((_, refusal)) => refusal.clone(),
            None if finished => Some("import_approval_unavailable".into()),
            None => None,
        }
    }

    /// How much of the dialog's time limit remains.
    pub(crate) fn remaining(&self) -> Duration {
        POST_DIALOG_LIMIT.saturating_sub(self.started.elapsed())
    }
}

impl Drop for PendingPostApproval {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl ApprovedImport {
    /// How many vouchers the person was asked about: the dialog's count is
    /// taken from these dates (#746), so a redeemed approval can be held to the
    /// batch it is redeemed for.
    ///
    /// A master post has no vouchers and no count: `None`. Whether an approval
    /// is for the right kind is decided by [`Self::post_kind`], never by this.
    pub(crate) fn voucher_count(&self) -> Option<usize> {
        match &self.kind {
            ApprovedKind::Vouchers(vouchers) => Some(vouchers.voucher_dates.len()),
            ApprovedKind::Masters(_) => None,
        }
    }
}

/// A person's answer to the review dialog for a doubted post (#239): that
/// they checked the voucher in Tally. It changes nothing in Tally and
/// authorises no post: it is a different type from [`ApprovedImport`], built
/// only by [`ReviewAcknowledged::confirm`], and nothing converts one into the
/// other.
#[must_use]
pub(crate) struct ReviewAcknowledged(());

impl ReviewAcknowledged {
    pub(crate) async fn confirm(
        kind: ReviewKind,
        count: VoucherCount,
        preview: &str,
    ) -> Result<Self, String> {
        approve_review(kind, count, preview).await?;
        Ok(Self(()))
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub(crate) enum ApprovedImportAdmissionError {
    #[error("education_voucher_date_unsupported")]
    EducationVoucherDateUnsupported,
    #[error("import_preexisting_identity")]
    PreexistingIdentity,
    #[error("import_masters_changed")]
    LedgerIdentityChanged,
    /// A Payment, Receipt or Contra leg no longer classifies as it did when
    /// approved: a ledger or one of its groups moved (bridge#466 follow-up).
    #[error("import_bank_classification_changed")]
    BankClassificationChanged,
    /// A ledger a bank cash answer named as cash in hand no longer reaches
    /// Cash-in-Hand: it or one of its groups moved since the build (#815).
    /// Carries each refused ledger's row, as the build's
    /// `cash_ledger_not_cash_in_hand` refusal reports it, and how many rows the
    /// recheck's budget left out.
    #[error("cash_ledger_not_cash_in_hand")]
    CashLedgerNotCashInHand {
        refused: Vec<serde_json::Value>,
        omitted: usize,
    },
    /// The batch was recorded before Bridge stored the ledgers its bank cash
    /// answers named as cash in hand, so there is nothing to check (#815).
    #[error("import_batch_predates_cash_ledger_record")]
    CashLedgersNotRecorded,
    /// The batch was recorded before Bridge stored which bill-wise ledgers a
    /// person approved to receive entries On Account, so there is nothing to
    /// check (#1234).
    #[error("import_batch_predates_bill_wise_record")]
    BillWiseNotRecorded,
    /// A ledger the batch names is bill-wise now and was not approved: it was
    /// switched to bill-wise after the build, so an entry on it would land On
    /// Account unseen (#1234).
    #[error("import_bill_wise_changed")]
    BillWiseChanged,
    /// A named ledger now folds equal to another live ledger, which Tally's
    /// import lookup could take for it (bridge#626).
    #[error("ledger_has_folded_twin")]
    LedgerFoldedTwin,
    /// The group collection the queue re-read for a bank voucher's
    /// classification could not be parsed. Carries the snapshot parser's own
    /// data-free code, as the read before approval does (bridge#717).
    #[error("group_export_invalid")]
    GroupExportInvalid { cause: Option<&'static str> },
    /// What the queue reads does not fit what the batch calls for: a bank
    /// voucher without its group read or a Journal with one, an invoice without
    /// its reads or reads without an invoice, a plan other than the one the
    /// build carried, or a judge or precheck that still asks for an answer once
    /// its reads are in. A wiring fault, refused before the POST is sent; the
    /// queue's own reads may already have been sent.
    #[error("import_post_admission_inconsistent")]
    AdmissionInconsistent,
    /// The snapshot sent last before the POST no longer shows exactly one
    /// loaded company with the target's GUID and name, or shows another loaded
    /// company sharing its name (#574).
    #[error("post_company_scope_changed")]
    CompanyScopeChanged,
    /// That snapshot could not be read, so the aim cannot be confirmed.
    #[error("post_company_scope_unconfirmed")]
    CompanyScopeUnconfirmed,
    /// The company defines more than one Currency master. Bridge's amounts are
    /// plain base-currency figures, and the write path does not compare a
    /// leg's currency with an identified base yet, so no leg can be shown to
    /// be in it (bridge#551). Carries every master's NAME, for the refusal to name.
    #[error("import_multi_currency_unsupported")]
    MultiCurrencyBook { currencies: Vec<String> },
    /// The company's Currency masters read as none, or the response does not
    /// parse (a master without a NAME does not).
    #[error("import_base_currency_undetermined")]
    BaseCurrencyUndetermined,
    /// The target's master mark (ALTMSTID) moved between the snapshot taken as
    /// the queue's binding reads began and the aim snapshot read last before
    /// the POST: a master changed after the catalogue re-read (bridge#239). Or
    /// it moved between the mark the voucher numbering was read under, before
    /// the approval, and that first snapshot (bridge#724): a master, perhaps a
    /// voucher type, changed since the numbering was read.
    #[error("post_masters_moved")]
    MastersMoved,
    /// That comparison could not be made: the first snapshot could not be
    /// read, or either did not hold exactly one row for the target.
    #[error("post_masters_unconfirmed")]
    MastersUnconfirmed,
    /// The ledger catalogue the queue re-read could not be parsed, so the
    /// approved binding cannot be rechecked. Raised only by that recheck,
    /// before the intent or the POST (bridge#641). The source is the typed,
    /// data-free cause the refusal carries.
    #[error("post_catalogue_unreadable")]
    CatalogueUnreadable(#[source] bridge_tally_protocol::StandardLedgerCatalogError),
    /// A master post met an Education profile at the queue's opening or closing
    /// observation. Raised before any index read is trusted and before the
    /// intent; nothing was sent.
    #[error("master_writes_need_licensed_mode")]
    MasterWritesNeedLicensedMode,
    /// The proposed ledger name could resolve to an existing master, through
    /// its stored name or an alias. Carries only the colliding master's kind
    /// and how its name matched, never another client's name.
    #[error("master_name_taken")]
    MasterNameTaken { kind: MasterKind, via: Via },
    /// The ledger name index answer could not be read, so a master post
    /// cannot show a ledger name absent and is refused before the intent.
    /// `cause` is the parser's code, which carries no book data
    /// (`StandardLedgerCatalogError::safe_code`),
    /// so a foreign company's answer is told apart from a malformed one.
    #[error("master_ledger_index_unavailable")]
    MasterLedgerIndexUnavailable { cause: &'static str },
    /// What the queue read of an invoice's masters differs from what the build
    /// recorded (#1337). `field` names the first that differs.
    #[error("import_invoice_masters_changed")]
    InvoiceMastersChanged { field: &'static str },
    /// The admission's judge refused the invoice on the queue's reads, under
    /// the code the build would have refused it with: a number now in use, a
    /// series set back to Automatic, a registration lapsed (#1337).
    #[error("{code}")]
    InvoiceRefused { code: String },
    /// A read of an invoice's masters that could not be used (a response that
    /// does not parse, a request that is not admitted), under the code that
    /// names it. Not a master that moved, and not the admission refusing the
    /// invoice (#1337).
    #[error("{code}")]
    InvoiceReadUnusable { code: String },
}

/// The data-free code a group snapshot refusal carries as its `cause`, for
/// the read before approval and the queue's re-read alike (bridge#676, #717).
pub(crate) fn group_snapshot_cause(
    error: &bridge_tally_protocol::native_outstandings::NativeOutstandingsError,
) -> Option<&'static str> {
    use bridge_tally_protocol::native_outstandings::NativeOutstandingsError;
    match error {
        NativeOutstandingsError::InvalidResponse(code) => Some(code),
        NativeOutstandingsError::TallyReportedFailure => Some("group_status_not_success"),
        NativeOutstandingsError::StatusAbsent => Some("group_status_absent"),
        _ => None,
    }
}

/// A failure inside the endpoint queue before the dispatch intent is recorded
/// (#656): every queue read, and the admission recheck, run in one block whose
/// error this wraps; the intent, the POST and the readback run after it. So it
/// marks a refusal whose outcome is known — nothing was sent — by where it
/// happened, and a read added to that block later is covered without a list.
/// A named admission refusal inside it keeps its own code.
#[derive(Debug, thiserror::Error)]
#[error("{source}")]
pub(crate) struct PreIntentQueueRefusal {
    #[source]
    pub(crate) source: anyhow::Error,
}

/// A refusal under the exclusive admission lock, just before the dispatch
/// intent is appended (#711). Nothing was recorded and nothing was sent, so
/// each keeps its own code instead of the catch-all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub(crate) enum UnderLockRefusal {
    #[error("import_batch_not_found")]
    BatchNotFound,
    #[error("import_already_attempted")]
    AlreadyAttempted,
    #[error("import_remote_id_reused")]
    RemoteIdReused,
    #[error("import_batch_changed")]
    BatchChanged,
    #[error("import_txn_already_posted")]
    TxnAlreadyPosted,
    /// The aim snapshot did not yield the target's voucher mark to record with
    /// the intent: its target row omits the mark (#1945), or the company's GUID
    /// is on more than one loaded row. No dispatch intent is recorded and
    /// nothing is sent.
    #[error("post_mark_unrecorded")]
    MarkUnrecorded,
    /// The approval could not be spent: it was revoked after this call took
    /// it, or a voucher post reached the lock without one (#791).
    #[error("import_approval_revoked")]
    ApprovalRevoked,
    /// A master post reached the lock without the pre-image its queue recheck
    /// was to leave for the intent: no sidecar can be named, so no intent is
    /// recorded and nothing is sent.
    #[error("master_pre_image_unrecorded")]
    PreImageUnrecorded,
    /// The pre-image sidecar could not be written. Refused before the intent:
    /// nothing was recorded in the master journal and nothing was sent.
    #[error("master_pre_image_write_failed")]
    PreImageWriteFailed,
    /// The pre-image sidecar written for a dispatch the journal then refused
    /// could not be removed again: nothing was recorded or sent, but a file for
    /// the batch is left behind.
    #[error("master_pre_image_remove_failed")]
    PreImageRemoveFailed,
    /// Another dispatched master batch, whose outcome is still open, names a
    /// ledger this one names: a second send of it under a new batch id. Nothing
    /// was recorded and nothing was sent.
    #[error("master_ledger_open_dispatch_overlaps")]
    MasterOverlapsOpenDispatch,
    /// The master journal refused the record before writing it: nothing was
    /// recorded and nothing was sent.
    #[error("master_ledger_invalid")]
    MasterJournalRefused,
    /// The master journal could not be read under the lock, so the batch's
    /// state could not be checked: nothing was recorded and nothing was sent.
    #[error("master_ledger_unavailable")]
    MasterJournalUnavailable,
    /// Another invoice of this company was sent and is not verified posted, as
    /// an approval dialog stood open (ADR 0004, slice 4): nothing was recorded
    /// or sent.
    #[error("invoice_company_stopped")]
    CompanyStopped,
    /// The batch is no longer one the company's transfer records allow to be
    /// sent: the reason is the code. The records changed after the check made
    /// before the person was asked.
    #[error("{0}")]
    Transfers(&'static str),
}

impl UnderLockRefusal {
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::BatchNotFound => "import_batch_not_found",
            Self::AlreadyAttempted => "import_already_attempted",
            Self::RemoteIdReused => "import_remote_id_reused",
            Self::BatchChanged => "import_batch_changed",
            Self::TxnAlreadyPosted => "import_txn_already_posted",
            Self::MarkUnrecorded => "post_mark_unrecorded",
            Self::ApprovalRevoked => "import_approval_revoked",
            Self::PreImageUnrecorded => "master_pre_image_unrecorded",
            Self::PreImageWriteFailed => "master_pre_image_write_failed",
            Self::PreImageRemoveFailed => "master_pre_image_remove_failed",
            Self::MasterOverlapsOpenDispatch => "master_ledger_open_dispatch_overlaps",
            Self::MasterJournalRefused => "master_ledger_invalid",
            Self::MasterJournalUnavailable => "master_ledger_unavailable",
            Self::CompanyStopped => "invoice_company_stopped",
            Self::Transfers(reason) => reason,
        }
    }
}

// What `before_dispatch` hands the send: the licence to send this request. Sealed
// in its own module, where the only two licences are defined.
#[path = "dispatch_license.rs"]
mod dispatch_license;
pub(crate) use dispatch_license::{DispatchLicense, RecordedIntent};

/// Why `before_dispatch` refused. Only a named check made before the intent
/// append is `Refused`; the lock, the journal read and the append itself are
/// `Other`, which keeps the catch-all because the append may have recorded an
/// intent (#711).
/// Built explicitly at each site: this file holds no conversion (the approval
/// seam gate refuses any `impl From`).
#[derive(Debug)]
pub(crate) enum BeforeDispatchError {
    Refused(UnderLockRefusal),
    Other(String),
}

/// The native approval every real post goes through. Outside this crate's own
/// unit tests it is exactly [`confirm`]: nothing else exists to answer it.
#[cfg(not(test))]
use confirm as approve;

#[cfg(test)]
use test_seam::approve;

/// The native review dialog every acknowledgement goes through, gated exactly
/// as [`approve`] is.
#[cfg(not(test))]
use confirm_review as approve_review;

#[cfg(test)]
use test_seam::approve_review;

/// The native one-ledger dialog every master approval goes through, gated
/// exactly as [`approve`] is.
#[cfg(not(test))]
use confirm_masters_dialog as approve_masters;

#[cfg(test)]
use test_seam::approve_masters;

/// What a post dialog's own task runs under (#725). Outside this crate's unit
/// tests it is the dialog alone: nothing is carried into the task.
#[cfg(not(test))]
use carry_nothing as carry_approval_scope;

#[cfg(test)]
use test_seam::carry_approval_scope;

#[cfg(not(test))]
fn carry_nothing<F>(dialog: F) -> F {
    dialog
}

/// A scripted answer to the native approval, for this crate's unit tests only
/// (bridge#583). It is compiled only under bare `cfg(test)`, which Cargo sets
/// for no shipped build and no feature, variable or flag can set at runtime;
/// `tests/approval_seam_gate.rs` holds it to that, and
/// `scripts/check-no-test-seam.mjs` proves its marker is absent from every
/// shipped executable.
#[cfg(test)]
pub(crate) mod test_seam {
    use std::sync::{Arc, Mutex};

    /// Present in any binary this module is compiled into, and in no other.
    pub(crate) const SEAM_MARKER: &str = "bridge-test-approval-seam-5f1c9e7a";

    const ONE: super::VoucherCount = super::VoucherCount(std::num::NonZeroUsize::MIN);

    fn count(count: usize) -> super::VoucherCount {
        super::VoucherCount::new(count).unwrap()
    }

    /// Whether the review dialog for `kind` holds `preview` within the box's
    /// caps, as the real dialog measures it, for the caller's tests.
    pub(crate) fn review_fits(
        kind: super::ReviewKind,
        count: super::VoucherCount,
        preview: &str,
    ) -> bool {
        let shown = kind.shown();
        super::fits_the_box(shown, count, &super::displayed_text(shown, count, preview))
    }

    /// What a test decided, and every preview the post path asked it about.
    #[derive(Clone)]
    pub(crate) struct ScriptedApproval {
        approve: bool,
        previews: Arc<Mutex<Vec<String>>>,
        /// Every preview the review dialog was asked about, kept apart from
        /// the post dialog's so a test can tell which dialog a person saw.
        reviews: Arc<Mutex<Vec<String>>>,
        /// Every preview the one-ledger dialog was asked about, kept apart
        /// from the other two so a test can tell which dialog a person saw.
        masters: Arc<Mutex<Vec<String>>>,
        /// The voucher count each dialog was asked about, in the same order
        /// as `previews` and `reviews` (#746).
        counts: Arc<Mutex<Vec<usize>>>,
        review_counts: Arc<Mutex<Vec<usize>>>,
        review_kinds: Arc<Mutex<Vec<super::ReviewKind>>>,
        /// Run while the approval is pending, as something else changing the
        /// book or the journal while an operator reads the dialog would.
        while_pending: Option<Arc<dyn Fn() + Send + Sync>>,
        /// When set, the post dialog stays open until a test answers it
        /// through [`ScriptedApproval::answer`], as a person who has not yet
        /// clicked would (#725). `approve` is then ignored.
        held: Option<Arc<tokio::sync::watch::Sender<Option<bool>>>>,
        /// Signalled when a held post dialog opens, so a test waits for that
        /// event rather than polling for it against a clock.
        opened: Arc<tokio::sync::Notify>,
        /// How many of the next post dialogs show a box that does not fit the
        /// screen, as the real dialog child would report with its exit status
        /// (#1832): each is refused as `import_review_too_large` and noted.
        too_tall: Arc<std::sync::atomic::AtomicUsize>,
    }

    impl ScriptedApproval {
        pub(crate) fn approving() -> Self {
            Self::new(true)
        }

        pub(crate) fn declining() -> Self {
            Self::new(false)
        }

        /// Approves, after running `while_pending` as the dialog would wait.
        pub(crate) fn approving_after(while_pending: impl Fn() + Send + Sync + 'static) -> Self {
            Self {
                while_pending: Some(Arc::new(while_pending)),
                ..Self::new(true)
            }
        }

        /// Approves, except that the first `times` post dialogs show a box that
        /// does not fit the screen.
        pub(crate) fn too_tall_first(times: usize) -> Self {
            let scripted = Self::new(true);
            scripted
                .too_tall
                .store(times, std::sync::atomic::Ordering::SeqCst);
            scripted
        }

        /// A post dialog that stays open until [`ScriptedApproval::answer`].
        pub(crate) fn held() -> Self {
            Self {
                held: Some(Arc::new(tokio::sync::watch::channel(None).0)),
                ..Self::new(false)
            }
        }

        /// Whether a held post dialog is still open: its task is waiting for
        /// an answer. False once the task was aborted, which closes it.
        pub(crate) fn is_waiting(&self) -> bool {
            self.held
                .as_ref()
                .is_some_and(|held| held.receiver_count() > 0)
        }

        /// Resolves once a held post dialog has opened: its task is waiting
        /// for an answer. A permit is kept, so it also resolves when the
        /// dialog opened before this was awaited.
        pub(crate) async fn opened(&self) {
            self.opened.notified().await;
        }

        /// Resolves once an opened held post dialog has closed: its task took
        /// an answer or was aborted. Await it only after [`Self::opened`]; a
        /// dialog not yet open counts as closed.
        pub(crate) async fn closed(&self) {
            if let Some(held) = &self.held {
                held.closed().await;
            }
        }

        /// Answer a held post dialog. Answering one that was closed (its task
        /// aborted) reaches nothing.
        pub(crate) fn answer(&self, approve: bool) {
            if let Some(held) = &self.held {
                held.send_replace(Some(approve));
            }
        }

        fn new(approve: bool) -> Self {
            Self {
                approve,
                previews: Arc::default(),
                reviews: Arc::default(),
                masters: Arc::default(),
                counts: Arc::default(),
                review_counts: Arc::default(),
                review_kinds: Arc::default(),
                while_pending: None,
                held: None,
                opened: Arc::default(),
                too_tall: Arc::default(),
            }
        }

        pub(crate) fn previews(&self) -> Vec<String> {
            self.previews.lock().unwrap().clone()
        }

        pub(crate) fn reviews(&self) -> Vec<String> {
            self.reviews.lock().unwrap().clone()
        }

        pub(crate) fn masters(&self) -> Vec<String> {
            self.masters.lock().unwrap().clone()
        }

        pub(crate) fn counts(&self) -> Vec<usize> {
            self.counts.lock().unwrap().clone()
        }

        pub(crate) fn review_counts(&self) -> Vec<usize> {
            self.review_counts.lock().unwrap().clone()
        }

        /// Which use of the review dialog each review was asked for.
        pub(crate) fn review_kinds(&self) -> Vec<super::ReviewKind> {
            self.review_kinds.lock().unwrap().clone()
        }
    }

    tokio::task_local! {
        /// The decision for the one task a test scopes it to. A task-local
        /// does not cross `tokio::spawn`: an approval asked from a spawned
        /// task finds no decision and is declined, which fails safe.
        pub(crate) static SCRIPTED_APPROVAL: ScriptedApproval;
    }

    /// What the scripted decision is for a dialog that records itself with
    /// `record`: the decision, once a held dialog has been answered. Unscoped
    /// it declines at once and starts no process.
    async fn scripted_decision(record: impl FnOnce(&ScriptedApproval)) -> bool {
        let decision = SCRIPTED_APPROVAL
            .try_with(|scripted| {
                record(scripted);
                if let Some(while_pending) = &scripted.while_pending {
                    while_pending();
                }
                match &scripted.held {
                    Some(held) => {
                        let waiting = held.subscribe();
                        scripted.opened.notify_one();
                        Err(waiting)
                    }
                    None => Ok(scripted.approve),
                }
            })
            .unwrap_or(Ok(false));
        let decision = match decision {
            Ok(decision) => decision,
            Err(mut held) => held
                .wait_for(Option::is_some)
                .await
                .map(|answer| *answer == Some(true))
                .unwrap_or(false),
        };
        std::hint::black_box(SEAM_MARKER);
        decision
    }

    /// The test-build approval. Unscoped, it declines at once and starts no
    /// process, so no test can reach a real dialog or approve by default.
    pub(super) async fn approve(count: super::VoucherCount, preview: &str) -> Result<(), String> {
        let decision = scripted_decision(|scripted| {
            scripted.previews.lock().unwrap().push(preview.to_string());
            scripted.counts.lock().unwrap().push(count.0.get());
        })
        .await;
        let too_tall = SCRIPTED_APPROVAL
            .try_with(|scripted| {
                scripted
                    .too_tall
                    .fetch_update(
                        std::sync::atomic::Ordering::SeqCst,
                        std::sync::atomic::Ordering::SeqCst,
                        |left| left.checked_sub(1),
                    )
                    .is_ok()
            })
            .unwrap_or(false);
        if too_tall {
            super::note_box_did_not_fit();
            return Err("import_review_too_large".into());
        }
        if decision {
            Ok(())
        } else {
            Err("import_approval_declined".into())
        }
    }

    /// The test-build one-ledger dialog, scripted by the same decision
    /// (including a held dialog a test answers) and declining the same way
    /// when unscoped.
    pub(super) async fn approve_masters(preview: &str) -> Result<(), String> {
        let decision = scripted_decision(|scripted| {
            scripted.masters.lock().unwrap().push(preview.to_string());
        })
        .await;
        if decision {
            Ok(())
        } else {
            Err("master_approval_declined".into())
        }
    }

    /// Carries the test's scripted decision into a post dialog's own task
    /// (#725). A task-local does not cross `tokio::spawn`, so without this a
    /// dialog asked from its task would find no decision and decline; with it,
    /// the task sees exactly the decision the asking test scoped, and an
    /// unscoped test still declines.
    pub(crate) fn carry_approval_scope<F>(
        dialog: F,
    ) -> impl std::future::Future<Output = F::Output> + Send + 'static
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send,
    {
        let scripted = SCRIPTED_APPROVAL.try_with(Clone::clone).ok();
        async move {
            match scripted {
                Some(scripted) => SCRIPTED_APPROVAL.scope(scripted, dialog).await,
                None => dialog.await,
            }
        }
    }

    /// The test-build review dialog, scripted by the same decision and
    /// declining the same way when unscoped.
    pub(super) async fn approve_review(
        kind: super::ReviewKind,
        count: super::VoucherCount,
        preview: &str,
    ) -> Result<(), String> {
        let decision = SCRIPTED_APPROVAL
            .try_with(|scripted| {
                scripted.reviews.lock().unwrap().push(preview.to_string());
                scripted.review_counts.lock().unwrap().push(count.0.get());
                scripted.review_kinds.lock().unwrap().push(kind);
                if let Some(while_pending) = &scripted.while_pending {
                    while_pending();
                }
                scripted.approve
            })
            .unwrap_or(false);
        std::hint::black_box(SEAM_MARKER);
        if decision {
            Ok(())
        } else {
            Err("ack_review_declined".into())
        }
    }

    /// The real approval keeps its own preview limit; the scripted one does
    /// not repeat it, so the limit is held here, on the real path, where it
    /// refuses before any process is started.
    #[tokio::test]
    async fn the_real_approval_refuses_an_oversized_preview_before_starting_a_process() {
        let oversized = "x".repeat(super::MAX_PREVIEW_BYTES + 1);
        assert_eq!(
            super::confirm(ONE, &oversized).await,
            Err("import_review_too_large".to_string())
        );
    }

    /// The same for the review dialog (#239), which has its own limit code.
    #[tokio::test]
    async fn the_real_review_refuses_an_oversized_preview_before_starting_a_process() {
        let oversized = "x".repeat(super::MAX_PREVIEW_BYTES + 1);
        assert_eq!(
            super::confirm_review(super::ReviewKind::Review, ONE, &oversized).await,
            Err("ack_review_too_large".to_string())
        );
    }

    /// The same for the one-ledger dialog, which has its own limit code.
    #[tokio::test]
    async fn the_real_masters_approval_refuses_an_oversized_preview_before_starting_a_process() {
        let oversized = "x".repeat(super::MAX_PREVIEW_BYTES + 1);
        assert_eq!(
            super::confirm_masters_with(
                std::path::Path::new("/nonexistent/never-started"),
                &oversized
            )
            .await,
            Err("master_review_too_large".to_string())
        );
        assert_eq!(
            super::confirm_masters_dialog(&oversized).await,
            Err("master_review_too_large".to_string())
        );
    }

    /// The dialog child's stand-in, `crates/bridge-approval-standin/examples/approval_standin.rs` (#702),
    /// built beside this test executable: `cargo test --workspace` and `cargo
    /// nextest run --workspace` build it; a run of `-p bridge` alone does not,
    /// because the example belongs to the `bridge-approval-standin` package.
    /// Missing, it fails the test; it never skips it.
    fn standin_example() -> std::path::PathBuf {
        let test = std::env::current_exe().unwrap();
        let profile = test.parent().and_then(std::path::Path::parent).unwrap();
        let example = profile
            .join("examples")
            .join(format!("approval_standin{}", std::env::consts::EXE_SUFFIX));
        assert!(
            example.is_file(),
            "{} is missing: build it with cargo test or cargo nextest run --workspace, or cargo build -p bridge-approval-standin --example approval_standin",
            example.display()
        );
        example
    }

    /// A directory for stand-ins, beside the example on the same volume, so
    /// [`standin`] can link to it.
    fn standin_directory() -> tempfile::TempDir {
        tempfile::Builder::new()
            .prefix("approval-standin-")
            .tempdir_in(standin_example().parent().unwrap())
            .unwrap()
    }

    /// A stand-in for a dialog subprocess that writes `answer`, with
    /// `{NONCE}` replaced by the nonce it was sent, and exits with `exit_code`.
    /// Its first act is to create an empty `<stand-in>.ran`, and its last,
    /// just before it exits, is to save there the input it read. Each row
    /// checks that input with [`assert_ran_to_its_end`], so a refusal the
    /// stand-in produced is told apart from a spawn that failed, and from a
    /// stand-in that died after starting, whose non-zero exit would otherwise
    /// read as a person's decline.
    ///
    /// Each call is a hard link to the built example, never a copy. A link
    /// opens no descriptor, so the test process never holds a writable one to
    /// an executable. If it did, another test thread's fork would inherit that
    /// descriptor until its exec, and exec'ing the stand-in in that window
    /// fails on Linux with "text file busy" (ETXTBSY). That failure surfaces
    /// as `…_unavailable`, the very code some rows expect (#704 review).
    fn standin(directory: &std::path::Path, exit_code: i32, answer: &str) -> std::path::PathBuf {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = directory.join(format!(
            "row-{}{}",
            NEXT.fetch_add(1, Ordering::Relaxed),
            std::env::consts::EXE_SUFFIX
        ));
        std::fs::hard_link(standin_example(), &path).unwrap();
        std::fs::write(beside(&path, ".answer"), format!("{exit_code}\n{answer}")).unwrap();
        path
    }

    fn beside(path: &std::path::Path, extension: &str) -> std::path::PathBuf {
        let mut path = path.as_os_str().to_owned();
        path.push(extension);
        path.into()
    }

    /// What the stand-in at `standin` was last sent, or `None` if it never
    /// started: see [`standin`].
    fn standin_input(standin: &std::path::Path) -> Option<String> {
        std::fs::read_to_string(beside(standin, ".ran")).ok()
    }

    /// The stand-in at `standin` ran to its end: it saved the input it was
    /// sent, and the dialog child's own parser reads it as the parent's shape
    /// for one voucher and `preview`.
    fn assert_ran_to_its_end(standin: &std::path::Path, preview: &str, name: &str) {
        let input = standin_input(standin).unwrap_or_default();
        let (_, shown, text) = super::dialog_input(&input)
            .unwrap_or_else(|| panic!("{name}: the stand-in did not run to its end"));
        assert_eq!((shown, text), (ONE, preview), "{name}");
    }

    /// The control for [`standin_input`]. A stand-in that cannot start is
    /// refused as unavailable too, and leaves no input behind. So each row's
    /// input check is what tells a refusal the stand-in produced from a spawn
    /// that failed.
    #[tokio::test]
    async fn a_stand_in_that_cannot_start_is_unavailable_and_leaves_no_marker() {
        let directory = standin_directory();
        let path = directory
            .path()
            .join(format!("not-executable{}", std::env::consts::EXE_SUFFIX));
        std::fs::write(&path, "not an executable image\n").unwrap();
        assert_eq!(
            super::confirm_with(&path, ONE, "Post").await,
            Err("import_approval_unavailable".to_string())
        );
        assert_eq!(standin_input(&path), None);
    }

    /// Only the token echoing this call's nonce is an answer, and it is
    /// accepted whatever the exit status. A person's decline is no token and
    /// exit 1. A clean exit without the token is never that (#689): it is an
    /// executable that is not this dialog, so it is refused as unavailable:
    /// an older build that ignores `--confirm-review`, a process that echoes
    /// its input, a token for another nonce, the post dialog's token, or the
    /// token with stray output, with CRLF or without its newline.
    #[tokio::test]
    async fn the_review_is_answered_only_by_the_token_for_its_nonce() {
        let directory = standin_directory();
        let answers = [
            (
                "a person's decline: no token, exit 1",
                1,
                "",
                Err("ack_review_declined"),
            ),
            (
                "an older build exits 0",
                0,
                "",
                Err("ack_review_unavailable"),
            ),
            (
                "an echo of the input",
                0,
                "{NONCE}\n1\nReview",
                Err("ack_review_unavailable"),
            ),
            (
                "a token for another nonce",
                0,
                "bridge-review-acknowledged:00000000-0000-4000-8000-000000000000\n",
                Err("ack_review_unavailable"),
            ),
            (
                "the post dialog's token for this nonce",
                0,
                "bridge-post-approved:{NONCE}\n",
                Err("ack_review_unavailable"),
            ),
            (
                "the one-ledger dialog's token for this nonce",
                0,
                "bridge-create-ledger:{NONCE}\n",
                Err("ack_review_unavailable"),
            ),
            (
                "a log line, then the token",
                0,
                "starting\nbridge-review-acknowledged:{NONCE}\n",
                Err("ack_review_unavailable"),
            ),
            (
                "the token with CRLF",
                0,
                "bridge-review-acknowledged:{NONCE}\r\n",
                Err("ack_review_unavailable"),
            ),
            (
                "the token without its newline",
                0,
                "bridge-review-acknowledged:{NONCE}",
                Err("ack_review_unavailable"),
            ),
            (
                "the token, but a failing exit",
                1,
                "bridge-review-acknowledged:{NONCE}\n",
                Ok(()),
            ),
        ];
        for (name, exit_code, answer, expected) in answers {
            // The stand-in must have run: a spawn failure is also
            // `ack_review_unavailable`, and would pass a row without reaching
            // the clean-exit-without-token arm it is here to pin.
            let path = standin(directory.path(), exit_code, answer);
            let result =
                super::confirm_review_with(&path, super::ReviewKind::Review, ONE, "Review").await;
            assert_eq!(result, expected.map_err(str::to_string), "{name}");
            assert_ran_to_its_end(&path, "Review", name);
        }
        // The control: the token for this call's nonce is accepted.
        let echoes_token = standin(directory.path(), 0, "bridge-review-acknowledged:{NONCE}\n");
        assert_eq!(
            super::confirm_review_with(&echoes_token, super::ReviewKind::Review, ONE, "Review")
                .await,
            Ok(())
        );
        assert_ran_to_its_end(&echoes_token, "Review", "the control");
        // A stop release runs its own mode and answers with its own token: the
        // review's token does not release a stop, and the release's does.
        let review_token = standin(directory.path(), 0, "bridge-review-acknowledged:{NONCE}\n");
        assert_eq!(
            super::confirm_review_with(
                &review_token,
                super::ReviewKind::StopRelease,
                ONE,
                "Review"
            )
            .await,
            Err("ack_review_unavailable".to_string())
        );
        assert_ran_to_its_end(&review_token, "Review", "a review token for a release");
        let release_token = standin(
            directory.path(),
            0,
            "bridge-stop-release-acknowledged:{NONCE}\n",
        );
        assert_eq!(
            super::confirm_review_with(
                &release_token,
                super::ReviewKind::StopRelease,
                ONE,
                "Review"
            )
            .await,
            Ok(())
        );
        assert_ran_to_its_end(&release_token, "Review", "the release control");
    }

    /// The post dialog is answered only by the token echoing this call's
    /// nonce, and a clean exit (#635). Anything else is refused, never
    /// approved. A clean exit without the token cannot be a person's decline,
    /// which exits 1, so it is refused as the dialog being unavailable: an
    /// executable that ignores `--confirm-journal`, one that echoes its input,
    /// a token for another nonce, the review dialog's token, or stray output.
    /// A failing exit is a decline, even after the right token.
    #[tokio::test]
    async fn a_post_is_approved_only_by_the_token_for_its_nonce() {
        let directory = standin_directory();
        for (name, exit_code, answer) in [
            ("a person's decline: no token, exit 1", 1, ""),
            (
                "the token, but a failing exit",
                1,
                "bridge-post-approved:{NONCE}\n",
            ),
        ] {
            let path = standin(directory.path(), exit_code, answer);
            assert_eq!(
                super::confirm_with(&path, ONE, "Post").await,
                Err("import_approval_declined".to_string()),
                "{name}"
            );
            assert_ran_to_its_end(&path, "Post", name);
        }
        for (name, answer) in [
            ("an executable ignoring the flag exits 0", ""),
            ("an echo of the input", "{NONCE}\n1\nPost"),
            (
                "a token for another nonce",
                "bridge-post-approved:00000000-0000-4000-8000-000000000000\n",
            ),
            (
                "the review dialog's token for this nonce",
                "bridge-review-acknowledged:{NONCE}\n",
            ),
            (
                "the one-ledger dialog's token for this nonce",
                "bridge-create-ledger:{NONCE}\n",
            ),
            // The answer is matched byte for byte, so any stray output, such
            // as a log line, is refused. That is fail-closed on purpose: do
            // not trim or search the output to "fix" it.
            (
                "a log line, then the token",
                "starting\nbridge-post-approved:{NONCE}\n",
            ),
            (
                "the token, then more output",
                "bridge-post-approved:{NONCE}\nmore\n",
            ),
            // What a Windows `echo` writes (#702).
            ("the token with CRLF", "bridge-post-approved:{NONCE}\r\n"),
            (
                "the token without its newline",
                "bridge-post-approved:{NONCE}",
            ),
        ] {
            // The stand-in must have run: a spawn failure is also
            // `import_approval_unavailable`, and would pass this row without
            // reaching the clean-exit-without-token arm it is here to pin.
            let path = standin(directory.path(), 0, answer);
            assert_eq!(
                super::confirm_with(&path, ONE, "Post").await,
                Err("import_approval_unavailable".to_string()),
                "{name}"
            );
            assert_ran_to_its_end(&path, "Post", name);
        }
        // The control: the token for this call's nonce, then a clean exit.
        let approves = standin(directory.path(), 0, "bridge-post-approved:{NONCE}\n");
        assert_eq!(super::confirm_with(&approves, ONE, "Post").await, Ok(()));
        assert_ran_to_its_end(&approves, "Post", "the control");
    }

    /// A dialog whose box did not fit the screen exits with its own status
    /// (#1832). That is a refusal that says the text is too large, for the
    /// post, the review and the one-ledger box (#1780), with or without a
    /// token: no answer, a printed approval included, passes for a box the
    /// person could not see whole. It is not a decline, not unavailable and
    /// never approved.
    #[tokio::test]
    async fn a_box_that_did_not_fit_the_screen_is_refused_whatever_it_printed() {
        let directory = standin_directory();
        let status = crate::tally::box_fit::DOES_NOT_FIT_EXIT;
        for (name, answer) in [
            ("no token", ""),
            (
                "the post token for this nonce",
                "bridge-post-approved:{NONCE}\n",
            ),
            (
                "the review token for this nonce",
                "bridge-review-acknowledged:{NONCE}\n",
            ),
            (
                "the one-ledger token for this nonce",
                "bridge-create-ledger:{NONCE}\n",
            ),
        ] {
            let path = standin(directory.path(), status, answer);
            assert_eq!(
                super::confirm_with(&path, ONE, "Post").await,
                Err("import_review_too_large".to_string()),
                "post: {name}"
            );
            assert_ran_to_its_end(&path, "Post", name);
            let path = standin(directory.path(), status, answer);
            assert_eq!(
                super::confirm_review_with(&path, super::ReviewKind::Review, ONE, "Review").await,
                Err("ack_review_too_large".to_string()),
                "review: {name}"
            );
            assert_ran_to_its_end(&path, "Review", name);
            let path = standin(directory.path(), status, answer);
            assert_eq!(
                super::confirm_masters_with(&path, "Create").await,
                Err("master_review_too_large".to_string()),
                "one ledger: {name}"
            );
            assert_ran_to_its_end(&path, "Create", name);
        }
        // The statuses next to it keep their meaning: a decline is exit 1, a
        // clean exit with no token is unavailable.
        let decline = standin(directory.path(), 1, "");
        assert_eq!(
            super::confirm_with(&decline, ONE, "Post").await,
            Err("import_approval_declined".to_string())
        );
        let unavailable = standin(directory.path(), 0, "");
        assert_eq!(
            super::confirm_with(&unavailable, ONE, "Post").await,
            Err("import_approval_unavailable".to_string())
        );
    }

    /// Each call sends a nonce of its own: a stand-in that answers every call
    /// with the token for the nonce it read sees a different one each time.
    #[tokio::test]
    async fn each_post_dialog_gets_a_fresh_nonce() {
        let directory = standin_directory();
        let approves = standin(directory.path(), 0, "bridge-post-approved:{NONCE}\n");
        let mut nonces = Vec::new();
        for _ in 0..2 {
            assert_eq!(super::confirm_with(&approves, ONE, "Post").await, Ok(()));
            let input = standin_input(&approves).expect("the stand-in ran");
            nonces.push(input.lines().next().unwrap().to_string());
        }
        assert!(nonces
            .iter()
            .all(|nonce| uuid::Uuid::parse_str(nonce).is_ok()));
        assert_ne!(nonces[0], nonces[1]);
    }

    /// The one-ledger dialog is approved only by the token echoing this call's
    /// nonce and a clean exit, as the post dialog is. Anything else is refused,
    /// never approved: a clean exit without the token is an executable that is
    /// not this dialog (unavailable), and the post and review dialogs' tokens
    /// are not this one's.
    #[tokio::test]
    async fn a_ledger_create_is_approved_only_by_the_token_for_its_nonce() {
        let directory = standin_directory();
        for (name, exit_code, answer) in [
            ("a person's decline: no token, exit 1", 1, ""),
            (
                "the token, but a failing exit",
                1,
                "bridge-create-ledger:{NONCE}\n",
            ),
        ] {
            let path = standin(directory.path(), exit_code, answer);
            assert_eq!(
                super::confirm_masters_with(&path, "Create").await,
                Err("master_approval_declined".to_string()),
                "{name}"
            );
            assert_ran_to_its_end(&path, "Create", name);
        }
        let another_nonce = format!("bridge-create-ledger:{}\n", crate::synthetic_guid(0));
        for (name, answer) in [
            ("an executable ignoring the flag exits 0", ""),
            ("an echo of the input", "{NONCE}\n1\nCreate"),
            ("a token for another nonce", another_nonce.as_str()),
            (
                "the post dialog's token for this nonce",
                "bridge-post-approved:{NONCE}\n",
            ),
            (
                "the review dialog's token for this nonce",
                "bridge-review-acknowledged:{NONCE}\n",
            ),
            (
                "a log line, then the token",
                "starting\nbridge-create-ledger:{NONCE}\n",
            ),
            (
                "the token, then more output",
                "bridge-create-ledger:{NONCE}\nmore\n",
            ),
            ("the token with CRLF", "bridge-create-ledger:{NONCE}\r\n"),
            (
                "the token without its newline",
                "bridge-create-ledger:{NONCE}",
            ),
        ] {
            let path = standin(directory.path(), 0, answer);
            assert_eq!(
                super::confirm_masters_with(&path, "Create").await,
                Err("master_approval_unavailable".to_string()),
                "{name}"
            );
            assert_ran_to_its_end(&path, "Create", name);
        }
        let approves = standin(directory.path(), 0, "bridge-create-ledger:{NONCE}\n");
        assert_eq!(
            super::confirm_masters_with(&approves, "Create").await,
            Ok(())
        );
        assert_ran_to_its_end(&approves, "Create", "the control");
    }

    /// The one-ledger token does not approve the post or the review dialog.
    #[tokio::test]
    async fn the_ledger_create_token_approves_neither_the_post_nor_the_review() {
        let directory = standin_directory();
        let answers = standin(directory.path(), 0, "bridge-create-ledger:{NONCE}\n");
        assert_eq!(
            super::confirm_with(&answers, ONE, "Post").await,
            Err("import_approval_unavailable".to_string())
        );
        assert_eq!(
            super::confirm_review_with(&answers, super::ReviewKind::Review, ONE, "Review").await,
            Err("ack_review_unavailable".to_string())
        );
    }

    /// Each dialog is started in its own mode, so the one-ledger preview is
    /// never shown by the post dialog or the reverse.
    #[tokio::test]
    async fn each_dialog_is_started_in_its_own_mode() {
        let directory = standin_directory();
        let sent = |path: &std::path::Path| {
            std::fs::read_to_string(beside(path, ".args")).expect("the stand-in ran")
        };
        let masters = standin(directory.path(), 0, "bridge-create-ledger:{NONCE}\n");
        super::confirm_masters_with(&masters, "Create")
            .await
            .unwrap();
        assert_eq!(sent(&masters), "--confirm-masters");
        let post = standin(directory.path(), 0, "bridge-post-approved:{NONCE}\n");
        super::confirm_with(&post, ONE, "Post").await.unwrap();
        assert_eq!(sent(&post), "--confirm-journal");
        let review = standin(directory.path(), 0, "bridge-review-acknowledged:{NONCE}\n");
        super::confirm_review_with(&review, super::ReviewKind::Review, ONE, "Review")
            .await
            .unwrap();
        assert_eq!(sent(&review), "--confirm-review");
    }

    /// The one-ledger dialog is told it is about one item, on the count line,
    /// and each call sends a nonce of its own.
    #[tokio::test]
    async fn the_ledger_create_dialog_is_told_one_item_with_a_fresh_nonce() {
        let directory = standin_directory();
        let approves = standin(directory.path(), 0, "bridge-create-ledger:{NONCE}\n");
        let mut nonces = Vec::new();
        for _ in 0..2 {
            assert_eq!(
                super::confirm_masters_with(&approves, "Create").await,
                Ok(())
            );
            let input = standin_input(&approves).expect("the stand-in ran");
            let (nonce, shown, text) = super::dialog_input(&input).expect("the parent's shape");
            assert_eq!((shown, text), (ONE, "Create"));
            nonces.push(nonce.to_string());
        }
        assert_ne!(nonces[0], nonces[1]);
    }

    /// Each dialog child is told the voucher count its title and button
    /// name, on the line after the nonce, and then the preview (#746). The
    /// stand-in saves the bytes the parent sent, and the child's own parser
    /// reads them, so the writer and the parser are tested together.
    #[tokio::test]
    async fn each_dialog_child_is_told_the_voucher_count() {
        let directory = standin_directory();
        for (prefix, review, sent, preview) in [
            ("bridge-post-approved:", false, 200, "Post"),
            ("bridge-review-acknowledged:", true, 7, "Review"),
        ] {
            let answers = standin(directory.path(), 0, &format!("{prefix}{{NONCE}}\n"));
            let result = if review {
                super::confirm_review_with(
                    &answers,
                    super::ReviewKind::Review,
                    count(sent),
                    preview,
                )
                .await
            } else {
                super::confirm_with(&answers, count(sent), preview).await
            };
            assert_eq!(result, Ok(()), "{prefix}");
            let input = standin_input(&answers).expect("the stand-in ran");
            let (_, shown, text) = super::dialog_input(&input).expect("the parent's shape");
            assert_eq!((shown, text), (count(sent), preview), "{prefix}");
        }
    }

    /// One voucher keeps the single-voucher words. A batch names its count in
    /// each title and on the post button, which never reads "Cancel", the
    /// label that carries the decline (#746).
    #[cfg(not(windows))]
    #[test]
    fn each_dialog_names_a_batch_by_its_count() {
        assert_eq!(
            super::post_words(ONE),
            (
                "ComplyEaze Bridge — approve one voucher".to_string(),
                "Post voucher".to_string()
            )
        );
        assert_eq!(
            super::post_words(count(200)),
            (
                "ComplyEaze Bridge — approve 200 vouchers".to_string(),
                "Post 200 vouchers".to_string()
            )
        );
        assert_eq!(super::post_words(count(2)).1, "Post 2 vouchers");
        assert_eq!(
            super::review_title(ONE),
            "ComplyEaze Bridge — record that you reviewed one voucher"
        );
        assert_eq!(
            super::review_title(count(50)),
            "ComplyEaze Bridge — record that you reviewed 50 vouchers"
        );
        // A stop release is titled for what it does, and names no review (#1828).
        assert_eq!(
            super::release_title(),
            "ComplyEaze Bridge — release the invoice stop"
        );
        // What each window's answer text says, in its own buttons' words (#1780).
        assert_eq!(
            super::answer_text(super::Shown::Post, ONE),
            "Click \"Post voucher\" to post this voucher into Tally. Click Cancel to cancel. \
             Nothing is posted until you answer."
        );
        assert_eq!(
            super::answer_text(super::Shown::Post, count(200)),
            "Click \"Post 200 vouchers\" to post these 200 vouchers into Tally. Click Cancel to \
             cancel. Nothing is posted until you answer."
        );
        assert_eq!(
            super::answer_text(super::Shown::Review, count(50)),
            "Click \"I reviewed it\" to record that you reviewed these 50 vouchers. Click Cancel \
             to cancel. Nothing is recorded until you answer."
        );
        assert_eq!(
            super::answer_text(super::Shown::StopRelease, ONE),
            "Click \"Release the stop\" to release the invoice stop for this company, so that \
             ComplyEaze Bridge builds and posts invoices for it again. Click Cancel to cancel. \
             Nothing is released until you answer."
        );
        assert_eq!(
            super::answer_text(super::Shown::Masters, ONE),
            "Click \"Create ledger\" to create this ledger in Tally. Click Cancel to cancel. \
             Nothing is created until you answer."
        );
    }

    /// The same for the Windows titles, which carry the question.
    #[cfg(windows)]
    #[test]
    fn each_windows_dialog_names_a_batch_by_its_count() {
        assert_eq!(
            super::post_question(ONE),
            "ComplyEaze Bridge — post this voucher?"
        );
        assert_eq!(
            super::post_question(count(200)),
            "ComplyEaze Bridge — post 200 vouchers?"
        );
        assert_eq!(
            super::review_question(ONE),
            "ComplyEaze Bridge — record that you reviewed this voucher?"
        );
        assert_eq!(
            super::review_question(count(50)),
            "ComplyEaze Bridge — record that you reviewed these 50 vouchers?"
        );
        // What each window's answer text says: Y and N answer a message box
        // that has the keyboard (#1780). The text is wrapped at 100 characters,
        // so its words are compared with the breaks read as spaces.
        assert_eq!(
            super::answer_text(super::Shown::Post, ONE).replace('\n', " "),
            "Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to \
             cancel. Nothing is posted until you answer."
        );
        assert_eq!(
            super::answer_text(super::Shown::Post, count(200)).replace('\n', " "),
            "Press Y (or click Yes) to post these 200 vouchers into Tally. Press N (or click No) \
             to cancel. Nothing is posted until you answer."
        );
        assert_eq!(
            super::answer_text(super::Shown::Review, count(50)).replace('\n', " "),
            "Press Y (or click Yes) to record that you reviewed these 50 vouchers. Press N (or \
             click No) to cancel. Nothing is recorded until you answer."
        );
        assert_eq!(
            super::answer_text(super::Shown::StopRelease, ONE).replace('\n', " "),
            "Press Y (or click Yes) to release the invoice stop for this company, so that \
             ComplyEaze Bridge builds and posts invoices for it again. Press N (or click No) to \
             cancel. Nothing is released until you answer."
        );
        assert_eq!(
            super::answer_text(super::Shown::Masters, ONE).replace('\n', " "),
            "Press Y (or click Yes) to create this ledger in Tally. Press N (or click No) to \
             cancel. Nothing is created until you answer."
        );
    }

    /// The answer text is wrapped under the dialog's line width without losing
    /// a word where the box needs it (Windows), is one line on the Mac, where
    /// the alert wraps its own text, and adds no more than three lines and 150
    /// characters above a post or review text, the size the dialog's caps were
    /// raised by (#1780). A wording that outgrows it fails here, so the box
    /// does not grow unseen.
    #[test]
    fn the_answer_text_is_wrapped_and_stays_within_the_size_the_caps_allow() {
        for shown in [
            super::Shown::Post,
            super::Shown::Review,
            super::Shown::StopRelease,
            super::Shown::Masters,
        ] {
            for n in [1, 2, 9, 10, 50] {
                let text = super::answer_text(shown, count(n));
                let joined = text.replace('\n', " ");
                assert_eq!(super::wrapped(&joined, 100).replace('\n', " "), joined);
                // The box's own text: Windows lines are within the width; the Mac
                // text is a single line and stays one.
                assert!(
                    text.lines().count() == 1
                        || text.lines().all(|line| line.chars().count() <= 100),
                    "{text}"
                );
            }
        }
        let (lines, chars) = super::answer_reserve();
        assert!(
            lines <= 3 && chars <= 150,
            "{lines} lines, {chars} characters"
        );
    }

    /// A preview at exactly the dialog's cap is shown whole with its answer
    /// text above it, and is refused by one line more; the stop release, which
    /// keeps its statement first, is not measured (#1780).
    #[test]
    fn a_preview_at_the_cap_is_shown_whole_with_its_answer_text() {
        // `lines` lines of at most 100 characters that make `chars` characters.
        let preview = |lines: usize, chars: usize| -> String {
            let text = chars - (lines - 1);
            (0..lines)
                .map(|at| "x".repeat(text / lines + usize::from(at < text % lines)))
                .collect::<Vec<_>>()
                .join("\n")
        };
        for shown in [super::Shown::Post, super::Shown::Review] {
            for (n, lines, chars) in [(1, 24, 1_600), (2, 40, 3_200), (50, 40, 3_200)] {
                let at_cap = preview(lines, chars);
                assert_eq!(
                    (at_cap.lines().count(), at_cap.chars().count()),
                    (lines, chars)
                );
                let shown_text = super::displayed_text(shown, count(n), &at_cap);
                assert!(super::fits_the_box(shown, count(n), &shown_text), "{n}");
                assert!(shown_text.starts_with(&super::answer_text(shown, count(n))));
                assert!(shown_text.ends_with(&at_cap));
                // One line more, or characters past what the box has room for.
                let over = super::displayed_text(shown, count(n), &preview(lines + 1, chars));
                assert!(!super::fits_the_box(shown, count(n), &over), "{n}");
                let wide =
                    super::displayed_text(shown, count(n), &format!("{at_cap}{}", "x".repeat(200)));
                assert!(!super::fits_the_box(shown, count(n), &wide), "{n}");
            }
        }
    }

    /// Where the Mac alert's top is taken to sit: on the screen Part A measured,
    /// all three rules give its 168 points; elsewhere the lowest of them is
    /// taken (#1780).
    #[cfg(not(windows))]
    #[test]
    fn the_mac_alert_top_is_the_lowest_of_the_three_rules() {
        let screen = |height, visible_top, visible_bottom| super::ScreenSpan {
            height,
            visible_top,
            visible_bottom,
        };
        let measured = screen(982.0, 33.0, 898.0);
        assert!((super::alert_top(measured) - 168.0).abs() < 1.0e-9);
        // A 13.3-inch Mac at 1440 x 900, no notch, a bottom Dock: the fixed offset.
        assert!((super::alert_top(screen(900.0, 24.0, 816.0)) - 168.0).abs() < 1.0e-9);
        // A tall screen: its share of the height.
        assert!(
            (super::alert_top(screen(1440.0, 25.0, 1356.0)) - 1440.0 * 168.0 / 982.0).abs()
                < 1.0e-9
        );
        // A deep menu bar: the offset below the visible frame.
        assert!((super::alert_top(screen(982.0, 60.0, 898.0)) - 195.0).abs() < 1.0e-9);
    }

    /// The Mac alert is shown only if its top, its 150 points of title, icon and
    /// buttons, its text and one line of slack end inside the visible frame;
    /// one point more refuses it, and a height that is not a size fits nothing
    /// (#1780). The dialog child's exit, `exit_unless_the_alert_fits`, asks
    /// exactly this, and the seam gate pins that it does.
    #[cfg(not(windows))]
    #[test]
    fn the_mac_alert_fits_only_inside_the_visible_frame_with_its_slack() {
        let measured = super::ScreenSpan {
            height: 982.0,
            visible_top: 33.0,
            visible_bottom: 898.0,
        };
        // 898 - 168 - 150 - 16 = 564 points of text.
        assert!(super::alert_fits(564.0, measured));
        assert!(!super::alert_fits(564.5, measured));
        // The slack counts: without it 580 would fit.
        assert!(!super::alert_fits(580.0, measured));
        // Part A's smallest one-voucher post: 28 lines of 16 points.
        assert!(super::alert_fits(448.0, measured));
        for height in [f64::NAN, f64::INFINITY, -1.0] {
            assert!(!super::alert_fits(height, measured), "{height}");
        }
    }

    /// AppKit lays the alert's text out as Part A saw the panel do: 16 points a
    /// line, two lines for a 100-character prose line, and 28 lines for the
    /// smallest one-voucher post text (row 1964-M, cells A1, A5 and A7). Measured
    /// without a window (#1780).
    #[cfg(target_os = "macos")]
    #[test]
    fn the_mac_alert_text_is_measured_as_the_panel_lays_it_out() {
        assert!((super::alert_text_height("x") - 16.0).abs() < 1.0e-9);
        let ten = ["Calibration line"; 10].join("\n");
        assert!((super::alert_text_height(&ten) - 160.0).abs() < 1.0e-9);
        let prose = "Synthetic narration for the calibration row, with ordinary words and spaces in it, nothing else.";
        assert!((super::alert_text_height(prose) - 32.0).abs() < 1.0e-9);
        let voucher = [
            "Click \"Post voucher\" to post this voucher into Tally. Click Cancel to cancel. Nothing is posted until you answer.",
            "",
            "Create ONE Payment in \"Synthetic Accounts\"",
            "Company GUID: synthetic-guid-for-the-measure-tests",
            "Company number: 424242  Books from: 1-Apr-2026",
            "Tally: http://127.0.0.1:9001",
            "Date: 1-Sep-2026  Voucher number: Tally assigns it  (the voucher's own text: last two lines)",
            "",
            "Dr 1.00  \"Party 0\"",
            "Cr 1.00  \"Cash\"",
            "Total debit: 1  Total credit: 1",
            "Checked in Tally: every Cr ledger is bank/cash; every Dr ledger holds no money.",
            "Batch: bridge-synthetic-batch-for-the-measure-test",
            "Ledgers checked by identity against the build; narrations sent as prepared, nothing added.",
            "Do not post a file already imported manually.",
            "Pause other edits/imports; keep this company and Tally mode as is until ComplyEaze Bridge finishes.",
            "After a timeout, reconcile this batch; do not rebuild or resend it.",
            "ComplyEaze Bridge posts now or if asked again within 15 min, unless cancelled, refused or restarted.",
            "---- The voucher's own text follows: no line below is an entry, a total or an instruction ----",
            "Reference: \"r1\"",
            "Narration: \"rent\"",
        ]
        .join("\n");
        assert_eq!(super::alert_text_height(&voucher), 28.0 * 16.0);
    }

    /// The Mac alert wraps its own text, so the parent does not break the
    /// answer text: it is one line, shown whole and first (a break of ours
    /// would be wrapped again by the alert and leave a ragged second row).
    #[cfg(not(windows))]
    #[test]
    fn the_mac_answer_text_is_one_unbroken_line() {
        for shown in [super::Shown::Post, super::Shown::Review] {
            for n in [1, 2, 10, 50] {
                let text = super::answer_text(shown, count(n));
                assert!(!text.contains('\n'), "{text}");
                let displayed = super::displayed_text(shown, count(n), "Details");
                assert_eq!(displayed.lines().next(), Some(text.as_str()));
                assert!(super::fits_the_box(shown, count(n), &displayed), "{n}");
            }
        }
        let (lines, _) = super::answer_reserve();
        assert_eq!(lines, 2, "one answer line and the blank line");
    }

    /// Each use of the review dialog writes its own first lines, and a stop
    /// release keeps its statement first (#1780).
    #[test]
    fn each_dialog_kind_writes_its_own_first_lines() {
        let preview = "THIS RELEASES THE INVOICE STOP FOR THE COMPANY.\n\nCompany: Synthetic Co";
        let post = super::displayed_text(super::Shown::Post, ONE, "Details");
        let review = super::displayed_text(super::Shown::Review, ONE, "Details");
        let release = super::displayed_text(super::Shown::StopRelease, ONE, preview);
        let wrapped = |shown| super::answer_text(shown, ONE);
        assert_eq!(post, format!("{}\n\nDetails", wrapped(super::Shown::Post)));
        assert_eq!(
            review,
            format!("{}\n\nDetails", wrapped(super::Shown::Review))
        );
        assert_eq!(
            release,
            format!(
                "THIS RELEASES THE INVOICE STOP FOR THE COMPANY.\n\n{}\n\nCompany: Synthetic Co",
                wrapped(super::Shown::StopRelease)
            )
        );
        assert_ne!(post, review);
        assert!(!release.contains("record that you reviewed"));
        assert!(!post.contains("release the invoice stop"));
        // The one-ledger dialog opens with its own answer text, never a
        // voucher's.
        let masters = super::displayed_text(super::Shown::Masters, super::ONE_ITEM, "Details");
        assert_eq!(
            masters,
            format!("{}\n\nDetails", wrapped(super::Shown::Masters))
        );
        assert!(masters.contains("create this ledger in Tally") && !masters.contains("voucher"));
    }

    /// The dialog subprocesses show a dialog only for input of the shape the
    /// parent sends: a nonce line, a count line in the one form the parent
    /// writes, then a preview within the limit.
    #[test]
    fn a_dialog_subprocess_admits_only_the_parents_input_shape() {
        let nonce = "9c8d8de4-c06c-447b-8309-60ba702bf663";
        assert_eq!(
            super::dialog_input(&format!("{nonce}\n1\nReview")),
            Some((nonce, ONE, "Review"))
        );
        assert_eq!(
            super::dialog_input(&format!("{nonce}\n200\nPost\nmore")),
            Some((nonce, count(200), "Post\nmore"))
        );
        let oversized = "x".repeat(super::MAX_PREVIEW_BYTES + 1);
        for input in [
            "Review".to_string(),
            "not-a-nonce\n1\nReview".to_string(),
            format!("{nonce}\n1\n"),
            format!("{nonce}\n1\nRe\0view"),
            format!("{nonce}\n1\n{oversized}"),
            // The count line absent, zero, signed, padded, empty, too large,
            // or not a number.
            format!("{nonce}\nReview"),
            format!("{nonce}\n0\nReview"),
            format!("{nonce}\n+2\nReview"),
            format!("{nonce}\n02\nReview"),
            format!("{nonce}\n 2\nReview"),
            format!("{nonce}\n2 \nReview"),
            format!("{nonce}\n\nReview"),
            format!("{nonce}\n99999999999999999999999\nReview"),
            format!("{nonce}\ntwo\nReview"),
        ] {
            assert_eq!(super::dialog_input(&input), None, "{input:.60}");
        }
    }
}

/// Which dialog a text is shown in (#1780). Each says in its first lines what
/// its answers do, in the words of its own buttons, and the parent writes them,
/// so the caps that keep a native box reviewable measure what the box shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shown {
    Post,
    Review,
    StopRelease,
    Settle,
    /// The one-ledger dialog, always about one ledger.
    Masters,
}

/// The kinds of acknowledgement. [`ReviewAcknowledged::confirm`] takes one,
/// and each has its own subprocess mode, so the invoice-stop release or a
/// batch's settlement is never shown with the review's title or button
/// (#1828, #1709).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReviewKind {
    Review,
    StopRelease,
    /// A person's settlement of a batch Tally answered as creating none of
    /// its vouchers (#1709).
    Settle,
}

impl ReviewKind {
    fn shown(self) -> Shown {
        match self {
            Self::Review => Shown::Review,
            Self::StopRelease => Shown::StopRelease,
            Self::Settle => Shown::Settle,
        }
    }

    /// The subprocess mode that shows this dialog, and the token prefix it answers
    /// with: each kind its own, so its window is titled for what it does.
    fn child(self) -> (&'static str, &'static str) {
        match self {
            Self::Review => ("--confirm-review", REVIEW_TOKEN_PREFIX),
            Self::StopRelease => ("--confirm-release", RELEASE_TOKEN_PREFIX),
            Self::Settle => ("--confirm-settle", SETTLE_TOKEN_PREFIX),
        }
    }
}

/// What a dialog is about, as the titles word it: one voucher, or a batch by
/// its count (#746).
fn these_vouchers(count: VoucherCount) -> String {
    match count.batch() {
        None => "this voucher".into(),
        Some(count) => format!("these {count} vouchers"),
    }
}

/// What the positive answer does, and what stays undone until it is given.
fn answer_effect(shown: Shown, count: VoucherCount) -> (String, &'static str) {
    match shown {
        Shown::Post => (
            format!("post {} into Tally", these_vouchers(count)),
            "Nothing is posted until you answer.",
        ),
        Shown::Review => (
            format!("record that you reviewed {}", these_vouchers(count)),
            "Nothing is recorded until you answer.",
        ),
        Shown::StopRelease => (
            "release the invoice stop for this company, so that ComplyEaze Bridge builds and \
             posts invoices for it again"
                .into(),
            "Nothing is released until you answer.",
        ),
        Shown::Settle => (
            format!(
                "close this batch in ComplyEaze Bridge for good, so that it never sends {} again",
                these_vouchers(count)
            ),
            "Nothing is closed until you answer.",
        ),
        Shown::Masters => (
            "create this ledger in Tally".into(),
            "Nothing is created until you answer.",
        ),
    }
}

/// The answer text of a Windows box, whose Yes and No answer to Y and N (#1780).
#[cfg(windows)]
fn answer_text(shown: Shown, count: VoucherCount) -> String {
    let (yes, nothing) = answer_effect(shown, count);
    wrapped(
        &format!("Press Y (or click Yes) to {yes}. Press N (or click No) to cancel. {nothing}"),
        ANSWER_WIDTH,
    )
}

/// The answer text of a Mac alert, which names its buttons and claims no key.
#[cfg(not(windows))]
fn answer_text(shown: Shown, count: VoucherCount) -> String {
    let (yes, nothing) = answer_effect(shown, count);
    let button = match shown {
        Shown::Post => post_words(count).1,
        Shown::Review => REVIEW_BUTTON.to_string(),
        Shown::StopRelease => RELEASE_BUTTON.to_string(),
        Shown::Settle => SETTLE_BUTTON.to_string(),
        Shown::Masters => MASTERS_BUTTON.to_string(),
    };
    wrapped(
        &format!("Click \"{button}\" to {yes}. Click Cancel to cancel. {nothing}"),
        usize::MAX,
    )
}

/// The most characters a line of a dialog's text takes (the post dialog's cap).
const ANSWER_WIDTH: usize = 100;

/// `text` broken at spaces into lines of at most `limit` characters. The
/// Windows box needs it, to keep the box's width bounded; the Mac passes no
/// limit, because the alert wraps its own text and a break of ours would be
/// wrapped a second time.
pub(crate) fn wrapped(text: &str, limit: usize) -> String {
    let (mut lines, mut width) = (String::new(), 0);
    for word in text.split(' ') {
        let length = word.chars().count();
        if width == 0 {
            lines.push_str(word);
            width = length;
        } else if width + 1 + length <= limit {
            lines.push(' ');
            lines.push_str(word);
            width += 1 + length;
        } else {
            lines.push('\n');
            lines.push_str(word);
            width = length;
        }
    }
    lines
}

/// What the native box holds of a post or review text (#1780): the lines and
/// characters of one voucher's text and of a batch's. The builders refuse a
/// preview over them, never cut it. The box also shows the answer text above
/// the preview, which has room of its own, [`answer_reserve`].
pub(crate) const ONE_VOUCHER_LINES: usize = 24;
pub(crate) const ONE_VOUCHER_CHARS: usize = 1_600;
pub(crate) const BATCH_LINES: usize = 40;
pub(crate) const BATCH_CHARS: usize = 3_200;
/// The most vouchers a dialog is asked about: a batch posts at most this many.
const MOST_VOUCHERS: usize = 50;

/// The lines and characters the answer text adds above a post or review text,
/// at most, over every text the code writes (both dialogs, one voucher to
/// [`MOST_VOUCHERS`]): its wrapped lines and a blank line, and the characters
/// of both. Derived from the text itself, so a new wording moves the box's
/// caps with it; a test holds it to the size the maintainers were told.
fn answer_reserve() -> (usize, usize) {
    let mut reserve = (0, 0);
    for shown in [Shown::Post, Shown::Review] {
        for count in (1..=MOST_VOUCHERS).filter_map(VoucherCount::new) {
            let answer = answer_text(shown, count);
            let added = (answer.lines().count() + 1, answer.chars().count() + 2);
            reserve = (reserve.0.max(added.0), reserve.1.max(added.1));
        }
    }
    reserve
}

/// Whether the text the box will show is within the box's caps (#1780): the
/// preview's own caps and room for the answer text, with no preview line over
/// 100 characters (the answer text's own lines are the Windows box's, wrapped
/// at that width, or the Mac alert's one line, which the alert wraps itself).
/// The parent measures what is shown, so a text that does not fit
/// is refused before a window opens, never cut. A stop release has no caps
/// beyond the byte limit. A Mac's dialog child also counts the alert's own
/// lines before it shows it, [`exit_unless_the_alert_fits`].
fn fits_the_box(shown: Shown, count: VoucherCount, displayed: &str) -> bool {
    if shown == Shown::StopRelease {
        return true;
    }
    let (lines, chars) = match count.batch() {
        None => (ONE_VOUCHER_LINES, ONE_VOUCHER_CHARS),
        Some(_) => (BATCH_LINES, BATCH_CHARS),
    };
    let (extra_lines, extra_chars) = answer_reserve();
    let answer_lines = answer_text(shown, count).lines().count();
    displayed.lines().count() <= lines + extra_lines
        && displayed.chars().count() <= chars + extra_chars
        && displayed
            .lines()
            .skip(answer_lines)
            .all(|line| line.chars().count() <= ANSWER_WIDTH)
}

/// What the review dialog shows for `kind` (#1780), for the caller's tests.
#[cfg(test)]
pub(crate) fn review_text(kind: ReviewKind, count: VoucherCount, preview: &str) -> String {
    displayed_text(kind.shown(), count, preview)
}

/// What the dialog shows: the answer text, then the preview. A stop release
/// keeps its own statement first, so its first line says what it releases, and
/// puts the answer text under it.
fn displayed_text(shown: Shown, count: VoucherCount, preview: &str) -> String {
    let answer = answer_text(shown, count);
    match (shown, preview.split_once("\n\n")) {
        (Shown::StopRelease, Some((statement, rest))) => {
            format!("{statement}\n\n{answer}\n\n{rest}")
        }
        _ => format!("{answer}\n\n{preview}"),
    }
}

/// The post dialog. An exit status alone is not an answer (#635): an
/// executable that does not know `--confirm-journal`, such as a build of
/// `bridge_mcp` older than the flag left at `current_exe()`, starts the MCP
/// server instead, reads the preview as input and exits 0 at its end. So the
/// parent sends a fresh nonce and requires the token that echoes it, which
/// only this dialog's positive button prints, and a clean exit as well.
async fn confirm(count: VoucherCount, preview: &str) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|_| "import_approval_unavailable")?;
    let shown = displayed_text(Shown::Post, count, preview);
    if !fits_the_box(Shown::Post, count, &shown) {
        return Err("import_review_too_large".into());
    }
    confirm_with(&executable, count, &shown).await
}

async fn confirm_with(
    executable: &std::path::Path,
    count: VoucherCount,
    preview: &str,
) -> Result<(), String> {
    if preview.len() > MAX_PREVIEW_BYTES {
        return Err("import_review_too_large".into());
    }
    match nonce_bound_dialog(
        executable,
        "--confirm-journal",
        POST_TOKEN_PREFIX,
        count,
        preview,
    )
    .await
    {
        // A box that did not fit the screen is a refusal whatever else the
        // child printed: the text is too large for this person's screen.
        Ok(answer) if answer.box_did_not_fit => {
            note_box_did_not_fit();
            Err("import_review_too_large".into())
        }
        Ok(answer) if answer.token_matched && answer.exited_cleanly => Ok(()),
        // A person's decline is no token and exit 1: `run_confirmation`
        // returns false. A clean exit without the token is never that; it is
        // an executable that does not answer with this token, such as one
        // ignoring the flag, or a build from before #635 whose dialog ran.
        Ok(answer) if answer.exited_cleanly => Err("import_approval_unavailable".into()),
        Ok(_) => Err("import_approval_declined".into()),
        Err(DialogFailure::Unavailable) => Err("import_approval_unavailable".into()),
        Err(DialogFailure::TimedOut) => Err("import_approval_timed_out".into()),
    }
}

/// The review dialog for a doubted post (#239), or the invoice-stop release
/// dialog (#1828): each its own subprocess mode, so neither title nor button
/// reads as approving a post, nor one as the other. It is answered by
/// the token alone, as the post dialog is by the token and a clean exit; a
/// clean exit without the token is unavailable, as the post dialog's is.
async fn confirm_review(
    kind: ReviewKind,
    count: VoucherCount,
    preview: &str,
) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|_| "ack_review_unavailable")?;
    let shown = displayed_text(kind.shown(), count, preview);
    if !fits_the_box(kind.shown(), count, &shown) {
        return Err("ack_review_too_large".into());
    }
    confirm_review_with(&executable, kind, count, &shown).await
}

async fn confirm_review_with(
    executable: &std::path::Path,
    kind: ReviewKind,
    count: VoucherCount,
    preview: &str,
) -> Result<(), String> {
    if preview.len() > MAX_PREVIEW_BYTES {
        return Err("ack_review_too_large".into());
    }
    let (mode, prefix) = kind.child();
    match nonce_bound_dialog(executable, mode, prefix, count, preview).await {
        Ok(answer) if answer.box_did_not_fit => Err("ack_review_too_large".into()),
        Ok(answer) if answer.token_matched => Ok(()),
        // A person's decline is no token and exit 1: `run_review_confirmation`
        // and `run_release_confirmation` return false. A clean exit without
        // the token is never that; it is an executable that is not this
        // dialog, such as one ignoring the flag (#689).
        Ok(answer) if answer.exited_cleanly => Err("ack_review_unavailable".into()),
        Ok(_) => Err("ack_review_declined".into()),
        Err(DialogFailure::Unavailable) => Err("ack_review_unavailable".into()),
        Err(DialogFailure::TimedOut) => Err("ack_review_timed_out".into()),
    }
}

/// The one-ledger dialog: its own subprocess mode, title, button and token,
/// answered exactly as the post dialog is, by the token for this call's nonce
/// and a clean exit together. Always asked about one item. Like the other
/// dialogs, its first lines say what each answer does (#1780).
async fn confirm_masters_dialog(preview: &str) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|_| "master_approval_unavailable")?;
    let shown = displayed_text(Shown::Masters, ONE_ITEM, preview);
    confirm_masters_with(&executable, &shown).await
}

async fn confirm_masters_with(executable: &std::path::Path, preview: &str) -> Result<(), String> {
    if preview.len() > MAX_PREVIEW_BYTES {
        return Err("master_review_too_large".into());
    }
    match nonce_bound_dialog(
        executable,
        "--confirm-masters",
        MASTERS_TOKEN_PREFIX,
        ONE_ITEM,
        preview,
    )
    .await
    {
        // A box that did not fit the screen is a refusal whatever else the
        // child printed, as for the post's box (#1832).
        Ok(answer) if answer.box_did_not_fit => Err("master_review_too_large".into()),
        Ok(answer) if answer.token_matched && answer.exited_cleanly => Ok(()),
        // A person's decline is no token and exit 1. A clean exit without the
        // token is an executable that is not this dialog.
        Ok(answer) if answer.exited_cleanly => Err("master_approval_unavailable".into()),
        Ok(_) => Err("master_approval_declined".into()),
        Err(DialogFailure::Unavailable) => Err("master_approval_unavailable".into()),
        Err(DialogFailure::TimedOut) => Err("master_approval_timed_out".into()),
    }
}

/// What a dialog subprocess answered: whether it printed exactly the token
/// for this call's nonce, whether it exited cleanly, and whether it exited
/// with the status that says its box did not fit the screen (#1832).
struct DialogAnswer {
    token_matched: bool,
    exited_cleanly: bool,
    box_did_not_fit: bool,
}

enum DialogFailure {
    Unavailable,
    TimedOut,
}

/// Show `preview` in the native dialog `mode` selects, in a subprocess of
/// `executable`, and read its answer. The parent sends a fresh nonce line, a
/// line with the voucher count the dialog's title and button name (#746),
/// and then the preview; the child prints `prefix` and that nonce only when
/// the person chose the positive button.
async fn nonce_bound_dialog(
    executable: &std::path::Path,
    mode: &str,
    prefix: &str,
    count: VoucherCount,
    preview: &str,
) -> Result<DialogAnswer, DialogFailure> {
    let nonce = uuid::Uuid::new_v4().to_string();
    #[expect(
        clippy::disallowed_methods,
        reason = "the native approval dialog helper, a local executable reached over stdin and stdout, not the network"
    )]
    let mut child = tokio::process::Command::new(executable)
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .map_err(|_| DialogFailure::Unavailable)?;
    let (answer, status) = tokio::time::timeout(Duration::from_secs(120), async {
        let mut input = child.stdin.take().ok_or(DialogFailure::Unavailable)?;
        input
            .write_all(format!("{nonce}\n{}\n{preview}", count.0).as_bytes())
            .await
            .map_err(|_| DialogFailure::Unavailable)?;
        drop(input);
        // The answer is one short line, so at most 128 bytes are read. A
        // child that writes more without exiting is waited on until the
        // timeout, then killed on drop: bounded on purpose.
        let mut output = child.stdout.take().ok_or(DialogFailure::Unavailable)?;
        let mut answer = Vec::new();
        tokio::io::AsyncReadExt::read_to_end(
            &mut tokio::io::AsyncReadExt::take(&mut output, 128),
            &mut answer,
        )
        .await
        .map_err(|_| DialogFailure::Unavailable)?;
        let status = child.wait().await.map_err(|_| DialogFailure::Unavailable)?;
        Ok::<_, DialogFailure>((answer, status))
    })
    .await
    .map_err(|_| DialogFailure::TimedOut)??;
    Ok(DialogAnswer {
        token_matched: answer == dialog_token(prefix, &nonce).as_bytes(),
        exited_cleanly: status.success(),
        box_did_not_fit: status.code() == Some(super::box_fit::DOES_NOT_FIT_EXIT),
    })
}

fn dialog_token(prefix: &str, nonce: &str) -> String {
    format!("{prefix}{nonce}\n")
}

/// Entry point for the same executable's private native-dialog subprocess.
/// Runs before Tokio starts, because macOS dialogs require the main thread.
/// It prints the token for the nonce it was given only when the person chose
/// to post (#635); the parent trusts nothing else.
pub fn run_confirmation() -> bool {
    answer_with_token(
        POST_TOKEN_PREFIX,
        show_review,
        std::io::stdin(),
        std::io::stdout(),
    )
}

/// Entry point for the review dialog's subprocess (#239), under the same rules.
pub fn run_review_confirmation() -> bool {
    answer_with_token(
        REVIEW_TOKEN_PREFIX,
        show_review_acknowledgement,
        std::io::stdin(),
        std::io::stdout(),
    )
}

/// Entry point for the one-ledger dialog's subprocess, under the same rules.
pub fn run_masters_confirmation() -> bool {
    answer_with_token(
        MASTERS_TOKEN_PREFIX,
        show_create_ledger,
        std::io::stdin(),
        std::io::stdout(),
    )
}

/// Entry point for the invoice-stop release dialog's subprocess, under the same
/// rules: its own window title and token, so a release is never titled a review.
pub fn run_release_confirmation() -> bool {
    answer_with_token(
        RELEASE_TOKEN_PREFIX,
        show_release_acknowledgement,
        std::io::stdin(),
        std::io::stdout(),
    )
}

/// Entry point for the settle dialog's subprocess (#1709), under the same
/// rules: its own window title and token, so a settlement is never titled a
/// review.
pub fn run_settle_confirmation() -> bool {
    answer_with_token(
        SETTLE_TOKEN_PREFIX,
        show_settle_acknowledgement,
        std::io::stdin(),
        std::io::stdout(),
    )
}

/// Read the parent's nonce line, count line and preview from `input`, show
/// `dialog` with that count and preview, and
/// write the token for that nonce to `output` only when it returns true. The
/// entry points pass stdin, stdout and their own dialog. The parent's tests
/// stand a script in for the child, so taking these as parameters is the only
/// way a test reaches the one line that turns a click into an approval: a
/// declined dialog writes nothing (#687).
fn answer_with_token(
    prefix: &str,
    dialog: fn(VoucherCount, &str) -> bool,
    input: impl Read,
    mut output: impl std::io::Write,
) -> bool {
    let mut text = String::new();
    // The nonce line is 37 bytes and the count line at most 21, so 64 covers
    // both: a preview at the limit is still read whole.
    if input
        .take(MAX_PREVIEW_BYTES as u64 + 64)
        .read_to_string(&mut text)
        .is_err()
    {
        return false;
    }
    let Some((nonce, count, preview)) = dialog_input(&text) else {
        return false;
    };
    if !dialog(count, preview) {
        return false;
    }
    output
        .write_all(dialog_token(prefix, nonce).as_bytes())
        .is_ok()
        && output.flush().is_ok()
}

/// The nonce line, the voucher count and the preview, when the input has the
/// shape the parent sends; `None` shows no dialog.
fn dialog_input(input: &str) -> Option<(&str, VoucherCount, &str)> {
    let (nonce, rest) = input.split_once('\n')?;
    let (count, preview) = rest.split_once('\n')?;
    let count = VoucherCount::parse(count)?;
    (uuid::Uuid::parse_str(nonce).is_ok()
        && !preview.contains('\0')
        && !preview.is_empty()
        && preview.len() <= MAX_PREVIEW_BYTES)
        .then_some((nonce, count, preview))
}

/// The acknowledgement dialog's title: the single-voucher words for one, and
/// the count for a batch (#746).
#[cfg(not(windows))]
fn review_title(count: VoucherCount) -> String {
    match count.batch() {
        None => "ComplyEaze Bridge — record that you reviewed one voucher".into(),
        Some(count) => format!("ComplyEaze Bridge — record that you reviewed {count} vouchers"),
    }
}

// The macOS alert as row 1964-M Part A measured it (`lab/1964-mac-alert-caps-answers`,
// macOS 26.6.2, one display, one run of each cell, so every figure here is
// partial). A real approval text takes its wide layout: 13-point text in a
// column about 406 points wide, under 150 points of title, icon and buttons. The
// alert opens with its top at a fixed place and is never moved up, so a tall one
// runs off the bottom of the screen. Its top was 168 points from the top of a
// 982-point screen whose visible frame starts 33 points down; whether that is a
// fixed offset, an offset below the visible frame or a share of the height is
// row 1964-C2's cell C2-2, so the alert's top is taken as the lowest of the
// three. One line of text, 16 points, is kept for a hyphenated word or a second
// title line.
#[cfg(not(windows))]
const ALERT_FONT_POINTS: f64 = 13.0;
#[cfg(not(windows))]
const ALERT_TEXT_WIDTH: f64 = 406.0;
#[cfg(not(windows))]
const ALERT_CHROME: f64 = 150.0;
#[cfg(not(windows))]
const ALERT_SLACK: f64 = 16.0;
#[cfg(not(windows))]
const ALERT_TOP_FROM_SCREEN_TOP: f64 = 168.0;
#[cfg(not(windows))]
const ALERT_TOP_BELOW_VISIBLE_TOP: f64 = 135.0;
#[cfg(not(windows))]
const ALERT_TOP_SHARE_OF_HEIGHT: f64 = 168.0 / 982.0;

/// A screen in points, measured down from its top edge, as the alert is placed
/// on it: its height, and the top and bottom of its visible frame.
#[cfg(not(windows))]
#[derive(Clone, Copy, Debug, PartialEq)]
struct ScreenSpan {
    height: f64,
    visible_top: f64,
    visible_bottom: f64,
}

/// Where the alert's top edge is taken to sit on `screen` (#1780): the lowest of
/// the three places Part A could not tell apart.
#[cfg(not(windows))]
fn alert_top(screen: ScreenSpan) -> f64 {
    ALERT_TOP_FROM_SCREEN_TOP
        .max(screen.visible_top + ALERT_TOP_BELOW_VISIBLE_TOP)
        .max(screen.height * ALERT_TOP_SHARE_OF_HEIGHT)
}

/// Whether an alert whose text is `text_height` points tall ends inside the
/// visible frame of `screen`, with its slack (#1780). A height that is not a
/// finite size fits nothing.
#[cfg(not(windows))]
fn alert_fits(text_height: f64, screen: ScreenSpan) -> bool {
    text_height.is_finite()
        && text_height >= 0.0
        && alert_top(screen) + ALERT_CHROME + text_height + ALERT_SLACK <= screen.visible_bottom
}

/// The height AppKit lays `text` out at in the alert's column (#1780), measured
/// without a window.
#[cfg(target_os = "macos")]
fn alert_text_height(text: &str) -> f64 {
    use objc2::runtime::AnyObject;
    use objc2_app_kit::{
        NSFont, NSFontAttributeName, NSStringDrawingOptions, NSStringNSExtendedStringDrawing,
    };
    use objc2_foundation::{NSDictionary, NSSize, NSString};
    let font = NSFont::systemFontOfSize(ALERT_FONT_POINTS);
    // SAFETY: an AppKit constant, read once.
    let key = unsafe { NSFontAttributeName };
    let attributes = NSDictionary::<_, AnyObject>::from_slices(&[key], &[font.as_ref()]);
    // SAFETY: the attributes map the font key to an `NSFont`, the type AppKit
    // reads for it.
    let rect = unsafe {
        NSString::from_str(text).boundingRectWithSize_options_attributes_context(
            NSSize::new(ALERT_TEXT_WIDTH, 1.0e7),
            NSStringDrawingOptions::UsesLineFragmentOrigin
                | NSStringDrawingOptions::UsesFontLeading,
            Some(&attributes),
            None,
        )
    };
    rect.size.height.ceil()
}

/// The menu-bar screen, which the alert is taken to open on; `None` off the
/// main thread or with no screen.
#[cfg(target_os = "macos")]
fn alert_screen() -> Option<ScreenSpan> {
    let main_thread = objc2::MainThreadMarker::new()?;
    let screen = objc2_app_kit::NSScreen::screens(main_thread).firstObject()?;
    let (frame, visible) = (screen.frame(), screen.visibleFrame());
    let top = frame.origin.y + frame.size.height;
    Some(ScreenSpan {
        height: frame.size.height,
        visible_top: top - (visible.origin.y + visible.size.height),
        visible_bottom: top - visible.origin.y,
    })
}

/// The alert's text height and its screen, measured in the dialog child before
/// anything is shown.
#[cfg(target_os = "macos")]
fn measured_alert(text: &str) -> Option<(f64, ScreenSpan)> {
    Some((alert_text_height(text), alert_screen()?))
}

/// No other platform's alert has been measured.
#[cfg(all(not(windows), not(target_os = "macos")))]
fn measured_alert(_text: &str) -> Option<(f64, ScreenSpan)> {
    None
}

/// Ends the dialog child before anything is shown when the Mac alert would run
/// past the bottom of its screen (#1780), with the status the parent reads as a
/// box that did not fit the screen, as a Windows box that does not fit ends
/// (`box_fit`). The parent then refuses the text as too large, and a batch's
/// next ask shows its totals only (#1832). A text that could not be measured is
/// not shown either: the child ends with no token, which the parent reads as
/// unavailable.
#[cfg(not(windows))]
fn exit_unless_the_alert_fits(text: &str) {
    match measured_alert(text) {
        Some((height, screen)) if alert_fits(height, screen) => {}
        Some(_) => std::process::exit(super::box_fit::DOES_NOT_FIT_EXIT),
        None => std::process::exit(0),
    }
}

/// The acknowledgement dialog. It posts nothing, so neither its title nor its
/// button may read as approving a post.
#[cfg(not(windows))]
fn show_review_acknowledgement(count: VoucherCount, preview: &str) -> bool {
    exit_unless_the_alert_fits(preview);
    rfd::MessageDialog::new()
        .set_title(review_title(count))
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            REVIEW_BUTTON.into(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(REVIEW_BUTTON.into())
}

/// The invoice-stop release window's title: it releases a stop and reviews no
/// voucher, so it names neither a review nor a post (#1828).
#[cfg(not(windows))]
fn release_title() -> String {
    "ComplyEaze Bridge — release the invoice stop".into()
}

/// The invoice-stop release dialog: the acknowledgement dialog with its own
/// title and button. It posts nothing either.
#[cfg(not(windows))]
fn show_release_acknowledgement(_count: VoucherCount, preview: &str) -> bool {
    exit_unless_the_alert_fits(preview);
    rfd::MessageDialog::new()
        .set_title(release_title())
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            RELEASE_BUTTON.into(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(RELEASE_BUTTON.into())
}

/// The settle window's title: it closes a batch and reviews no voucher, so it
/// names neither a review nor a post (#1709).
#[cfg(not(windows))]
fn settle_title() -> String {
    "ComplyEaze Bridge — close this batch".into()
}

/// The settle dialog: the acknowledgement dialog with its own title and
/// button. It posts nothing either.
#[cfg(not(windows))]
fn show_settle_acknowledgement(_count: VoucherCount, preview: &str) -> bool {
    exit_unless_the_alert_fits(preview);
    rfd::MessageDialog::new()
        .set_title(settle_title())
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            SETTLE_BUTTON.into(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(SETTLE_BUTTON.into())
}

/// The Windows acknowledgement dialog's title. Its Yes/No/Cancel box has no
/// custom labels, so the title asks the question, with the count for a batch
/// (#746).
#[cfg(windows)]
fn review_question(count: VoucherCount) -> String {
    match count.batch() {
        None => "ComplyEaze Bridge — record that you reviewed this voucher?".into(),
        Some(count) => {
            format!("ComplyEaze Bridge — record that you reviewed these {count} vouchers?")
        }
    }
}

#[cfg(windows)]
fn show_review_acknowledgement(count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = review_question(count)
        .encode_utf16()
        .chain(Some(0))
        .collect();
    // As for `show_review`: measured when it opens, and never returning if it
    // is not wholly on the screen (#1832).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: as for `show_review`: both buffers are NUL-terminated and
        // live for the synchronous dialog, and no parent HWND is borrowed.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // As for the post's box (`show_review`): topmost with
                // MB_SYSTEMMODAL, taking the foreground when Windows lets it,
                // with No as the default button (#1589, #1780).
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}

/// The Windows invoice-stop release box's title, a question as the review's is
/// (#1828).
#[cfg(windows)]
fn release_question() -> String {
    "ComplyEaze Bridge — release the invoice stop for this company?".into()
}

/// The Windows invoice-stop release box: the review's box with its own title.
#[cfg(windows)]
fn show_release_acknowledgement(_count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = release_question().encode_utf16().chain(Some(0)).collect();
    // As for `show_review`: measured when it opens, and never returning if it
    // is not wholly on the screen (#1832).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: as for `show_review`: both buffers are NUL-terminated and
        // live for the synchronous dialog, and no parent HWND is borrowed.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // As for the review's box: topmost with MB_SYSTEMMODAL, taking
                // the foreground when Windows lets it, with No as the default
                // button (#1589, #1780).
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}

/// The Windows settle box's title, a question as the review's is (#1709).
#[cfg(windows)]
fn settle_question() -> String {
    "ComplyEaze Bridge — close this batch for good?".into()
}

/// The Windows settle box: the review's box with its own title.
#[cfg(windows)]
fn show_settle_acknowledgement(_count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = settle_question().encode_utf16().chain(Some(0)).collect();
    // As for `show_review`: measured when it opens, and never returning if it
    // is not wholly on the screen (#1832).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: as for `show_review`: both buffers are NUL-terminated and
        // live for the synchronous dialog, and no parent HWND is borrowed.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // As for the review's box: topmost with MB_SYSTEMMODAL, taking
                // the foreground when Windows lets it, with No as the default
                // button (#1589, #1780).
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}

/// The post dialog's title and positive button. One voucher keeps the
/// single-voucher words; a batch names its count in both, so neither calls a
/// batch one voucher (#746).
#[cfg(not(windows))]
fn post_words(count: VoucherCount) -> (String, String) {
    match count.batch() {
        None => (
            "ComplyEaze Bridge — approve one voucher".into(),
            POST_LABEL.into(),
        ),
        Some(count) => (
            format!("ComplyEaze Bridge — approve {count} vouchers"),
            format!("Post {count} vouchers"),
        ),
    }
}

#[cfg(not(windows))]
fn show_review(count: VoucherCount, preview: &str) -> bool {
    exit_unless_the_alert_fits(preview);
    let (title, button) = post_words(count);
    rfd::MessageDialog::new()
        .set_title(title)
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        // The Cancel label supplies the native Escape action. Posting requires
        // the explicitly matched positive button; Return may leave this dialog open.
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            button.clone(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(button)
}

/// The Windows post dialog's title. Its Yes/No/Cancel box has no custom
/// labels, so the title asks the question, with the count for a batch (#746).
#[cfg(windows)]
fn post_question(count: VoucherCount) -> String {
    match count.batch() {
        None => "ComplyEaze Bridge — post this voucher?".into(),
        Some(count) => format!("ComplyEaze Bridge — post {count} vouchers?"),
    }
}

#[cfg(windows)]
fn show_review(count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    // rfd without common-controls-v6 discards custom labels. Use the existing
    // Win32 dependency so No is the default and Escape/close remain Cancel.
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = post_question(count).encode_utf16().chain(Some(0)).collect();
    // The box cannot scroll: `guarded` measures it when it opens and, if it is
    // not wholly on the screen, never returns (#1832).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: Both buffers are NUL-terminated and live for the synchronous
        // dialog; no parent HWND is borrowed. No application state is exposed
        // to callbacks.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // Topmost: a window the connector opens while Tally fills the
                // screen is otherwise drawn behind it, since a background
                // process may not take the foreground (#1589). MB_TOPMOST alone
                // was seen not to hold when the box opened seconds after the
                // last input went to Tally. With MB_SYSTEMMODAL added the box
                // held it in one lab run on Windows 10, where Tally still took a
                // click and a key while the box was open (#1780). That run did
                // not test what else a system-modal box changes: the taskbar
                // showed no task buttons while it was open, cause not tested
                // (#2161). When Windows lets it,
                // MB_SETFOREGROUND also gives the box the foreground, so a key
                // typed for Tally can answer it; the parent puts a first line on
                // its text that says what Y and N do (#1780). No stays the
                // default button.
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}

/// The one-ledger dialog. It is shown for one item only: a count other than
/// one is not the parent's shape and shows nothing.
#[cfg(not(windows))]
fn show_create_ledger(count: VoucherCount, preview: &str) -> bool {
    if count != ONE_ITEM {
        return false;
    }
    exit_unless_the_alert_fits(preview);
    rfd::MessageDialog::new()
        .set_title(MASTERS_TITLE)
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        // The Cancel label supplies the native Escape action. Creating
        // requires the explicitly matched positive button.
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            MASTERS_BUTTON.into(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(MASTERS_BUTTON.into())
}

#[cfg(windows)]
fn show_create_ledger(count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    // As for the post dialog: No is the default, Escape and close are Cancel.
    if count != ONE_ITEM {
        return false;
    }
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = MASTERS_TITLE.encode_utf16().chain(Some(0)).collect();
    // As for `show_review`: measured when it opens, and never returning if it
    // is not wholly on the screen (#1832). Through `guarded` it is also
    // brought to the front once it appears, as the post's box is: its first
    // line tells the person to press Y or N, and in one lab run on Windows 10
    // MB_SETFOREGROUND alone did not give it the keyboard when it opened
    // seconds after the last input went to Tally (#1780).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: as for `show_review`: both buffers are NUL-terminated and
        // live for the synchronous dialog, and no parent HWND is borrowed.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // As for the post's box (`show_review`): topmost with
                // MB_SYSTEMMODAL, taking the foreground when Windows lets it,
                // with No as the default button (#1589, #1780).
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("import_company_scope_ambiguous")]
pub(crate) struct AmbiguousImportCompany;

pub(super) fn require_unique_company_scope(
    companies: &[bridge_tally_protocol::TallyCompany],
    identity: &super::VerifiedCompanyIdentity,
) -> Result<(), AmbiguousImportCompany> {
    let count = companies
        .iter()
        .filter(|company| {
            company
                .name
                .trim()
                .eq_ignore_ascii_case(identity.display_name().trim())
        })
        .count();
    if count == 1 {
        Ok(())
    } else {
        Err(AmbiguousImportCompany)
    }
}

#[cfg(test)]
#[path = "approved_import_tests.rs"]
mod tests;
