//! Read profiles for the local MCP adapter.
use super::*;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::ledger_scope::LedgerScopeName;

/// The FETCH list for reads that RETURN bill allocations.
///
/// `ALLLEDGERENTRIES.*`, the shape `IMPLEMENTATION_GUIDE.md` §2.4a proves correct on
/// the instance where curated allocation paths misreport genuine `New Ref`/`Agst Ref`
/// rows as `On Account`. Measured on TallyPrime 7.1 Silver (protocol reference §8.2a),
/// `ALLLEDGERENTRIES.BILLALLOCATIONS.*` recovers the same allocations for 1.12x the
/// curated payload against 7.3x — but that is one instance, and §8.2a says plainly it
/// is **untested on the affected one**.
///
/// The asymmetry decides it. If the narrower shape is wrong there, a reader silently
/// receives incorrect bill types on compliance data. If the wider shape costs too much,
/// that is loud, measurable and fixable. An unverified narrowing is not worth a payload
/// saving when the failure mode is silently-wrong evidence, so the proven shape is used
/// until someone runs the A/B on that corpus.
///
/// The parser handles all 27 nested `*.LIST` types this returns per entry, including
/// `TAXBILLALLOCATIONS.LIST` — a different list from `BILLALLOCATIONS.LIST` — proven by
/// `entry_wildcard_response_parses_with_its_twenty_seven_nested_lists` against a live
/// capture rather than a constructed one.
///
/// `ISPOSTDATED` is requested here for the same reason `ISCANCELLED`/
/// `ISOPTIONAL` are: Tally will not include a field in COLLECTION XML that
/// this FETCH list does not name, regardless of what the parser is willing
/// to accept. Allow-listing the tag in `agent_voucher_scalars.rs` without
/// asking for it here would make `post_dated` permanently absent — a
/// wired-looking field that never fires.
///
/// `REFERENCE`, `ISINVOICE` and `PARTYGSTIN` are requested for the same
/// reason, added after `ALLLEDGERENTRIES.*` to match the order proven on the
/// wire in protocol reference §8.2c (TallyPrime 7.1 Silver, licensed,
/// `BRIDGE SHAPE LAB`, 2026-09-18): all three round-trip cleanly alongside
/// the existing fields, `ISINVOICE` is the one logical here Tally emits
/// without a `TYPE="Logical"` attribute (irrelevant to parsing, which
/// matches on tag name only), and `PARTYGSTIN` was structurally present but
/// empty on every voucher in that capture — its population is unverified,
/// not its presence.
///
/// `REFERENCEDATE` (#1257) is requested for the same reason, added last. A live read of the
/// synthetic book `BRIDGE SHAPE LAB` with this word added returned the element on every voucher
/// (`TYPE="Date"`, empty on 66 of 67 and `YYYYMMDD` on one), and the answers were byte-equal to the
/// same reads without it apart from the elements (protocol reference §8.2f).
pub(super) const AGENT_VOUCHER_FETCH: &str =
    "DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,\
GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLLEDGERENTRIES.*,\
REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE";

/// The window read's list without `REFERENCEDATE`: the list every ledger-scoped window read the lab
/// sent carried (the bill trail's, #1356, on two books), and the book-wide window reads it was
/// compared with. [`AGENT_VOUCHER_FETCH`] added `REFERENCEDATE` later (#1363) and has not been
/// sent ledger-scoped, so it is not what a ledger-scoped read sends until a lab run does.
const AGENT_LEDGER_WINDOW_FETCH: &str =
    "DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,\
GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLLEDGERENTRIES.*,\
REFERENCE,ISINVOICE,PARTYGSTIN";

/// The FETCH list for `ledger_movement`, which DISCARDS bill allocations.
///
/// `MovementEntry` carries no allocation field, so the movement read threw the
/// allocation payload away after paying for it — and, worse, inherited allocation
/// parse failures: one malformed allocation aborted a movement read that never wanted
/// allocations. With the entry wildcard above that cost becomes 7.3x for data the
/// caller cannot see, and dense windows that previously fit could exceed limits while
/// returning no movement at all.
///
/// A read should fetch what it returns. This list is the three entry fields movement
/// actually uses.
const AGENT_MOVEMENT_FETCH: &str = "DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,\
GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ALLLEDGERENTRIES.LEDGERNAME,ALLLEDGERENTRIES.AMOUNT,\
ALLLEDGERENTRIES.ISDEEMEDPOSITIVE";

/// Windowed voucher read for `ledger_movement`, which does not return allocations.
pub(super) fn render_agent_movement_vouchers(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
) -> Result<String, String> {
    render_agent_movement_vouchers_in_span(company, from, to, None)
}

/// [`render_agent_movement_vouchers`], optionally narrowed to an AlterID span.
/// `None` renders the unnarrowed request byte for byte; `Some` is one part of
/// a day too heavy for one read (protocol reference §11c).
pub(super) fn render_agent_movement_vouchers_in_span(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    span: Option<AlterIdSpan>,
) -> Result<String, String> {
    render_windowed_vouchers(
        company,
        from,
        to,
        &span.map(AlterIdSpan::filter).unwrap_or_default(),
        AGENT_MOVEMENT_FETCH,
        "",
    )
}

pub(super) fn render_agent_vouchers(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    alter_id: Option<u64>,
) -> Result<String, String> {
    let alter_filter = alter_id
        .map(|value| format!(" AND $AlterID > {value}"))
        .unwrap_or_default();
    render_windowed_vouchers(company, from, to, &alter_filter, AGENT_VOUCHER_FETCH, "")
}

/// [`render_agent_vouchers`], optionally narrowed to an AlterID span. `None`
/// renders the unnarrowed request byte for byte; `Some` is one part of a day
/// too heavy for one read (protocol reference §11c).
pub(super) fn render_agent_vouchers_in_span(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    span: Option<AlterIdSpan>,
) -> Result<String, String> {
    render_windowed_vouchers(
        company,
        from,
        to,
        &span.map(AlterIdSpan::filter).unwrap_or_default(),
        AGENT_VOUCHER_FETCH,
        "",
    )
}

/// [`render_agent_vouchers_in_span`] with each row's voucher type resolved by
/// Tally in the same response: its GUID, its own reserved name and one
/// `$$Is<Class>` answer per measured class (bridge#625). Sent only by a
/// `vouchers` call that filters by type, so every other read is unchanged.
pub(super) fn render_agent_class_vouchers_in_span(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    span: Option<AlterIdSpan>,
) -> Result<String, String> {
    render_windowed_vouchers(
        company,
        from,
        to,
        &span.map(AlterIdSpan::filter).unwrap_or_default(),
        AGENT_VOUCHER_FETCH,
        &voucher_type_class_computes(),
    )
}

/// The pre-flight census of protocol reference §11c: one light row per voucher
/// in the window, optionally only those whose AlterID falls in `span`.
///
/// The `FETCH` is the one §12.7 qualified for the empty-partition witness, and
/// the AlterID range is the segment filter the outstandings scanner sends. The
/// date window keeps a census cheap; the span bounds the response by
/// construction — with distinct AlterIDs it cannot return more than
/// `span.through - span.after` rows — and is used whenever the book's mark is
/// larger than one census (protocol reference §11c.3).
pub(super) fn render_agent_voucher_census(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    span: Option<AlterIdSpan>,
) -> Result<String, String> {
    render_voucher_census(company, VoucherScope::Book, from, to, span)
}

/// The census of [`render_agent_voucher_census`] for the vouchers of one ledger (#485): the same
/// light row per voucher, in the same window, read as a `Vouchers : Ledger` collection `CHILDOF`
/// the ledger, always in an AlterID span (every answer observed was spanned; an unspanned ledger
/// census has not been observed). Read on `BRIDGE CORPUS DENSE` (TallyPrime 7.2 Silver, 28 Oct
/// 2026): a day of one ledger as 82 vouchers, equal to the book-wide read of that day; a year of
/// another ledger as 248 vouchers over four spans, equal to its unspanned ledger-scoped read; an
/// empty span an empty collection (protocol reference section 11c.6, lab
/// `lab/485-ledger-census-spans-answers`). Sent by `ledger_movement` to plan its window (slice 2 of
/// #485). Not observed: a name that matches no ledger, and whether a voucher
/// with two lines on the ledger is listed twice.
pub(super) fn render_agent_ledger_census(
    company: &str,
    ledger: &LedgerScopeName,
    from: &TallyDate,
    to: &TallyDate,
    span: AlterIdSpan,
) -> Result<String, String> {
    render_voucher_census(company, VoucherScope::Ledger(ledger), from, to, Some(span))
}

/// The vouchers a windowed request reads: the book's, or one ledger's (#485). A ledger enters a
/// request only as a [`LedgerScopeName`], admitted once and closed.
#[derive(Clone, Copy)]
enum VoucherScope<'a> {
    Book,
    Ledger(&'a LedgerScopeName),
}

impl VoucherScope<'_> {
    /// The collection's name (the request header's `ID` is the same) and its `TYPE` element, with
    /// the `CHILDOF` element after it for a ledger. `book` and `ledger` are the two names.
    fn collection(self, book: &'static str, ledger: &'static str) -> (&'static str, String) {
        match self {
            Self::Book => (book, "<TYPE>Voucher</TYPE>".to_string()),
            Self::Ledger(name) => (
                ledger,
                format!(
                    "<TYPE>Vouchers : Ledger</TYPE><CHILDOF>{}</CHILDOF>",
                    xml_escape(name.as_str())
                ),
            ),
        }
    }
}

fn render_voucher_census(
    company: &str,
    scope: VoucherScope<'_>,
    from: &TallyDate,
    to: &TallyDate,
    span: Option<AlterIdSpan>,
) -> Result<String, String> {
    let (collection, type_element) =
        scope.collection("Bridge Agent Voucher Census", "Bridge Agent Ledger Census");
    let company = ValidatedCompanyName::new(company.to_string())
        .map_err(|_| "company_name_invalid".to_string())?;
    let span_filter = span.map(AlterIdSpan::filter).unwrap_or_default();
    // A quoted `$$Date:"…"` literal takes only a date: XML escaping cannot
    // protect it, since Tally decodes `&quot;` before evaluating (#861).
    let (from, to) = (from.as_str(), to.as_str());
    Ok(format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>{collection}</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{from}</SVFROMDATE><SVTODATE TYPE=\"Date\">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeAgentCensus\">$Date &gt;= $$Date:\"{from}\" AND $Date &lt;= $$Date:\"{to}\"{span_filter}</SYSTEM><COLLECTION NAME=\"{collection}\" ISMODIFY=\"No\">{type_element}<FETCH>GUID,ALTERID,DATE</FETCH><FILTERS>BridgeAgentCensus</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        xml_escape(company.as_str())
    ))
}

fn render_windowed_vouchers(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    alter_filter: &str,
    fetch: &str,
    computes: &str,
) -> Result<String, String> {
    render_scoped_vouchers(
        company,
        VoucherScope::Book,
        from,
        to,
        alter_filter,
        fetch,
        computes,
    )
}

/// The field list of a ledger-scoped voucher read: a closed set, never text from a caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LedgerVoucherFields {
    /// The movement read's: the three entry fields movement uses ([`AGENT_MOVEMENT_FETCH`]).
    Movement,
    /// The window read's list as the lab sent it ledger-scoped, before `REFERENCEDATE` was added to
    /// the window read (#1363): every entry field and the invoice fields, ending `PARTYGSTIN`
    /// ([`AGENT_LEDGER_WINDOW_FETCH`]).
    Window,
}

impl LedgerVoucherFields {
    fn fetch(self) -> &'static str {
        match self {
            Self::Movement => AGENT_MOVEMENT_FETCH,
            Self::Window => AGENT_LEDGER_WINDOW_FETCH,
        }
    }
}

/// A voucher read (`Bridge Agent Ledger Vouchers`) for the vouchers of one ledger (#485): the
/// book-wide read's window formula, read as a `Vouchers : Ledger` collection `CHILDOF` the ledger,
/// with the field list `fields` names and nothing else different. Observed without an AlterID span
/// only: with [`LedgerVoucherFields::Movement`] a month of one ledger as 21 vouchers, equal to the
/// census rows of that month, whose lines sum to the committed one-month Trial Balance's debit for
/// the ledger; [`LedgerVoucherFields::Window`] differs from it in the `FETCH` element alone and
/// was read by the lab for the bill trail (#1356). A spanned form has not been observed and is not
/// offered. `ledger_movement` sends the movement fields (#485), and the `outstandings` party detail
/// the window fields (#1356).
pub(super) fn render_agent_ledger_vouchers(
    company: &str,
    ledger: &LedgerScopeName,
    fields: LedgerVoucherFields,
    from: &TallyDate,
    to: &TallyDate,
) -> Result<String, String> {
    render_scoped_vouchers(
        company,
        VoucherScope::Ledger(ledger),
        from,
        to,
        "",
        fields.fetch(),
        "",
    )
}

fn render_scoped_vouchers(
    company: &str,
    scope: VoucherScope<'_>,
    from: &TallyDate,
    to: &TallyDate,
    alter_filter: &str,
    fetch: &str,
    computes: &str,
) -> Result<String, String> {
    let (collection, type_element) =
        scope.collection("Bridge Agent Vouchers", "Bridge Agent Ledger Vouchers");
    let company = ValidatedCompanyName::new(company.to_string())
        .map_err(|_| "company_name_invalid".to_string())?;
    // A quoted `$$Date:"…"` literal takes only a date: XML escaping cannot
    // protect it, since Tally decodes `&quot;` before evaluating (#861).
    let (from, to) = (from.as_str(), to.as_str());
    Ok(format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>{collection}</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{from}</SVFROMDATE><SVTODATE TYPE=\"Date\">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeAgentWindow\">$Date &gt;= $$Date:\"{from}\" AND $Date &lt;= $$Date:\"{to}\"{alter_filter}</SYSTEM><COLLECTION NAME=\"{collection}\" ISMODIFY=\"No\">{type_element}<FETCH>{fetch}</FETCH>{computes}<FILTERS>BridgeAgentWindow</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        xml_escape(company.as_str())
    ))
}

/// The FETCH list for LAB-ONLY inventory-entry reads (`lab_read_inventory`,
/// feature `lab-writes`). Unlike [`AGENT_VOUCHER_FETCH`], this asks for
/// `ALLINVENTORYENTRIES.*` instead of `ALLLEDGERENTRIES.*` -- no shipped tool
/// reads voucher inventory entries today, so this shape is
/// exploratory pending a live capture, not a qualified/compatibility-evidenced
/// read.
#[cfg(feature = "lab-writes")]
const AGENT_LAB_INVENTORY_VOUCHER_FETCH: &str = "DATE,VOUCHERNUMBER,VOUCHERTYPENAME,\
PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ALLINVENTORYENTRIES.*";

/// Windowed voucher read for the LAB-ONLY `lab_read_inventory` tool. Reuses
/// the same [`render_windowed_vouchers`] windowing machinery (and therefore
/// the same `window_honoured` corroboration path) as `vouchers`/
/// `ledger_movement` -- only the FETCH list differs.
#[cfg(feature = "lab-writes")]
pub(super) fn render_agent_lab_inventory_vouchers(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
) -> Result<String, String> {
    render_windowed_vouchers(company, from, to, "", AGENT_LAB_INVENTORY_VOUCHER_FETCH, "")
}

/// The candidate FETCH for a check that a supplier invoice is not already in the books (#1904):
/// [`AGENT_VOUCHER_FETCH`] with its entry wildcard narrowed to the entry fields such a check reads
/// and the bill allocations, nothing else changed. LAB-ONLY (feature `lab-writes`) until a lab
/// sitting has compared it with the agent read on the same window: protocol reference §8.2a
/// measured `ALLLEDGERENTRIES.BILLALLOCATIONS.*` on one 7.1 book only, and this exact list has never
/// been sent to Tally.
#[cfg(feature = "lab-writes")]
const AGENT_LAB_PURCHASE_IDENTITY_FETCH: &str =
    "DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,\
NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLLEDGERENTRIES.LEDGERNAME,\
ALLLEDGERENTRIES.AMOUNT,ALLLEDGERENTRIES.ISDEEMEDPOSITIVE,ALLLEDGERENTRIES.BILLALLOCATIONS.*,\
REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE";

/// The candidate narrow read of [`AGENT_LAB_PURCHASE_IDENTITY_FETCH`], in the window shape and with
/// the voucher-type class computes of [`render_agent_class_vouchers_in_span`], so the FETCH list is
/// the only difference from the agent read it is compared with.
#[cfg(feature = "lab-writes")]
pub(super) fn render_agent_lab_purchase_identity_vouchers(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
) -> Result<String, String> {
    render_windowed_vouchers(
        company,
        from,
        to,
        "",
        AGENT_LAB_PURCHASE_IDENTITY_FETCH,
        &voucher_type_class_computes(),
    )
}

/// A voucher type's name as the status read quotes it into a TDL string
/// literal, where XML escaping cannot protect it (Tally decodes `&quot;` before
/// evaluating, #861). Parsed once, from the tool's argument: a name is admitted
/// only when it holds nothing a literal could be closed or broken by. `&`, `<`,
/// `>` and `'` are refused as well: Tally's handling of them inside the literal
/// has not been measured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GstStatusTypeName(String);

impl GstStatusTypeName {
    pub(super) fn parse(name: &str) -> Option<Self> {
        let admitted = !name.is_empty()
            && name.chars().count() <= 100
            && name == name.trim()
            && !name
                .chars()
                .any(|c| matches!(c, '"' | '\\' | '&' | '<' | '>' | '\'') || c.is_control());
        admitted.then(|| Self(name.to_string()))
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

/// The GST status of the vouchers of one type in a date window (R7, 10 Oct
/// 2026): the narrow field list and the by-name voucher-type filter the lab
/// answered on a synthetic TallyPrime 7.1 Silver book, over two days holding
/// Sales vouchers, Receipts and a second Sales-class type. Since 12 Oct 2026 it
/// fetches `ISCANCELLED` and `ISOPTIONAL` (#1655) and `ISPOSTDATED` (#1664) as
/// well, and the request is, character for character, the lab's file
/// `w21-status-window-aug-postdated.xml` (the identifiers are the ones it
/// carried). The type is named, never classed: a type of the same class under
/// another name is not selected.
pub(super) fn render_gst_status_window(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    type_name: &GstStatusTypeName,
) -> Result<String, String> {
    render_status_window(company, from, to, type_name, StatusFetch::Flags)
}

/// The uncertain-voucher read: the status read with the party and the
/// voucher's amount added to its field list. Since 13 Oct 2026 it is, character
/// for character, the lab's file `w21d-status-window-aug-wide-postdated.xml`,
/// which a synthetic TallyPrime 7.1 Silver book answered. Asking for `AMOUNT`
/// makes Tally return each voucher's ledger entries as well (about 7.5 KB a
/// voucher, UTF-16, on the lab's book).
pub(super) fn render_gst_uncertain_window(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    type_name: &GstStatusTypeName,
) -> Result<String, String> {
    render_status_window(company, from, to, type_name, StatusFetch::FlagsPartyAmount)
}

/// The two field lists a status window read is measured with, each named by
/// what it adds to the fields every status read fetches.
#[derive(Clone, Copy)]
enum StatusFetch {
    /// The cancelled, optional and post-dated flags (W21, `gst_status`).
    Flags,
    /// The party, those flags and the voucher's amount (W21d).
    FlagsPartyAmount,
}

impl StatusFetch {
    fn tail(self) -> &'static str {
        match self {
            StatusFetch::Flags => ",ISCANCELLED,ISOPTIONAL,ISPOSTDATED",
            StatusFetch::FlagsPartyAmount => {
                ",PARTYLEDGERNAME,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,AMOUNT"
            }
        }
    }
}

/// The status window read of one voucher type, with the fields of `fetch`
/// appended to the fields every status read fetches.
fn render_status_window(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    type_name: &GstStatusTypeName,
    fetch: StatusFetch,
) -> Result<String, String> {
    let fetch_tail = fetch.tail();
    let company = ValidatedCompanyName::new(company.to_string())
        .map_err(|_| "company_name_invalid".to_string())?;
    // A quoted `$$Date:"…"` literal takes only a date (#861).
    let (from, to) = (from.as_str(), to.as_str());
    Ok(format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Agent Import Verification</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{from}</SVFROMDATE><SVTODATE TYPE=\"Date\">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeImportWindow\">$Date &gt;= $$Date:\"{from}\" AND $Date &lt;= $$Date:\"{to}\" AND $VoucherTypeName = \"{}\"</SYSTEM><COLLECTION NAME=\"Bridge Agent Import Verification\" ISMODIFY=\"No\"><TYPE>Voucher</TYPE><FETCH>DATE,VOUCHERNUMBER,VOUCHERTYPENAME,MASTERID,ALTERID,VCHGSTSTATUSISINCLUDED,VCHGSTSTATUSISUNCERTAIN,VCHGSTSTATUSISEXCLUDED,ISGSTOVERRIDDEN{fetch_tail}</FETCH><FILTERS>BridgeImportWindow</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        xml_escape(company.as_str()),
        xml_escape(type_name.as_str())
    ))
}

pub(super) fn gst_uncertain_read(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    type_name: &GstStatusTypeName,
) -> Result<ReadRequest, String> {
    render_gst_uncertain_window(company, from, to, type_name).map(ReadRequest)
}

pub(super) fn gst_status_read(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    type_name: &GstStatusTypeName,
) -> Result<ReadRequest, String> {
    render_gst_status_window(company, from, to, type_name).map(ReadRequest)
}

pub(super) fn render_agent_company_high_water(company: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME=\"Bridge Agent Company High Water\" ISMODIFY=\"No\"><TYPE>Company</TYPE><FETCH>GUID,ALTVCHID,ALTMSTID</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        xml_escape(company)
    )
}

/// A read request rendered by one of the profiles below.
///
/// The field is private to this module, so a `ReadRequest` can only come from
/// a function here, and each of those renders a fixed Bridge profile.
/// `Server::post_read` accepts nothing else. `AgentReadRequest::parse` still
/// checks the envelope before dispatch, but it is no longer the only barrier
/// between caller-built XML and the gateway.
#[derive(Clone, Debug)]
pub(super) struct ReadRequest(String);

impl ReadRequest {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }

    pub(super) fn into_xml(self) -> String {
        self.0
    }

    /// Test-only: lets a test hand `post_read` a request no profile renders,
    /// to prove the envelope gate behind the seal still refuses it.
    #[cfg(test)]
    pub(super) fn unrendered_for_test(xml: String) -> Self {
        Self(xml)
    }
}

/// The book's voucher-type masters, as the sync connector reads them: names
/// and GUIDs only, no period (bridge#664).
pub(super) fn voucher_type_catalogue_read(company: &str) -> ReadRequest {
    ReadRequest(
        bridge_tally_protocol::native_outstandings::render_native_voucher_type_export_request(
            company,
        ),
    )
}

pub(super) fn company_high_water_read(company: &str) -> ReadRequest {
    ReadRequest(render_agent_company_high_water(company))
}

pub(super) fn standard_ledger_catalog_read(company: &str) -> anyhow::Result<ReadRequest> {
    crate::tally::standard_ledger_catalog::render_standard_ledger_catalog_request(company)
        .map(ReadRequest)
}

/// The import family's ledger catalogue: V1's request with each ledger's
/// bill-wise flag in the same row (#1234).
pub(super) fn import_ledger_catalogue_read(company: &str) -> anyhow::Result<ReadRequest> {
    crate::tally::standard_ledger_catalog::render_import_ledger_catalog_request(company)
        .map(ReadRequest)
}

/// The ledger name index a master post reads: the V3 ledger catalogue, the
/// request `MasterAdmission` re-reads inside the queue, parsed by
/// `agent_master_post::ledger_masters`.
pub(super) fn ledger_name_index_read(company: &str) -> anyhow::Result<ReadRequest> {
    use bridge_tally_protocol::xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName};
    let validated = ValidatedCompanyName::new(company)
        .map_err(|_| anyhow::anyhow!("master_request_invalid"))?;
    Ok(ReadRequest(
        ReadOnlyProfile::StandardLedgerCatalogV3 {
            company: &validated,
        }
        .render(),
    ))
}

/// The party/ledger master read restricted to the ledgers under one parent
/// group: the request the compliance reader sends per parent part, used to read
/// back the fields of one ledger a master post created. Its answer carries
/// sensitive master values; the one caller projects each row onto the few
/// fields it reads (`agent_master_verdict::PartyView`).
pub(super) fn party_ledger_master_for_parent_read(
    company: &str,
    period: &bridge_tally_protocol::native_outstandings::NativeLedgerExportPeriod,
    part: &bridge_tally_protocol::parent_partition::ParentPart,
) -> ReadRequest {
    ReadRequest(
        bridge_tally_protocol::native_outstandings::render_party_ledger_master_request_for_parents(
            company, period, part,
        ),
    )
}

pub(super) fn native_group_snapshot_read(company: &str) -> ReadRequest {
    ReadRequest(
        bridge_tally_protocol::native_outstandings::render_native_group_snapshot_request(company),
    )
}

/// The voucher types of the book, every field, for an invoice build: the named
/// type, its parent chain to a reserved class and its series-level numbering.
pub(super) fn invoice_voucher_types_read(company: &str) -> ReadRequest {
    ReadRequest(agent_import::invoice::render_voucher_types_request(company))
}

/// The company's own count of its ledgers, to size an invoice build on a book
/// whose master mark is high.
pub(super) fn invoice_ledger_count_read(company: &str) -> ReadRequest {
    ReadRequest(
        bridge_tally_protocol::outstandings_shared::render_company_ledger_count_request(company),
    )
}

/// The vouchers carrying one invoice number in one financial year, with the
/// measured per-row Sales-class compute, for the duplicate-number check.
pub(super) fn invoice_number_read(
    company: &str,
    number: &agent_import::invoice::InvoiceNumber,
    window: (&TallyDate, &TallyDate),
) -> ReadRequest {
    ReadRequest(agent_import::invoice::render_invoice_number_request(
        company, number, window,
    ))
}

/// A posted invoice read back by type and number, every field the build wrote.
pub(super) fn invoice_readback_read(
    company: &str,
    type_guid: &agent_import::invoice::VoucherTypeGuid,
    number: &agent_import::invoice::InvoiceNumber,
    window: (&TallyDate, &TallyDate),
) -> ReadRequest {
    ReadRequest(agent_import::invoice::render_invoice_readback_request(
        company, type_guid, number, window,
    ))
}

/// The ledger listing with each ledger's GST rate and rounding, for the tax
/// check of an invoice, restricted to the ledgers under one scope's parents
/// (#1331). Its window is the invoice's financial year start and the invoice
/// date.
pub(super) fn invoice_ledger_rates_scoped_read(
    company: &str,
    window: (&TallyDate, &TallyDate),
    part: &bridge_tally_protocol::parent_partition::ParentPart,
) -> ReadRequest {
    ReadRequest(
        agent_import::invoice::render_ledger_rates_request_for_parents(company, window, part),
    )
}

/// The group listing with each group's HSN/SAC rows, for the HSN/SAC check
/// of an invoice (rows bound to the company by GUID).
pub(super) fn invoice_group_hsn_read(company: &str) -> ReadRequest {
    ReadRequest(agent_import::invoice::render_group_hsn_request(company))
}

/// The company's GST registrations, for an invoice build (rows bound to the
/// company by GUID).
pub(super) fn invoice_company_registration_read(company: &str) -> ReadRequest {
    ReadRequest(agent_import::invoice::render_company_registration_request(
        company,
    ))
}

/// The company's Currency masters: the request the outstandings paths send,
/// verbatim. Its response carries no company GUID, so it is bound only by the
/// identity brackets it is read inside.
pub(super) fn company_currency_read(company: &str) -> ReadRequest {
    ReadRequest(
        bridge_tally_protocol::native_outstandings::render_company_currency_request(company),
    )
}

pub(super) fn voucher_window_part_read(
    shape: super::voucher_window::VoucherReadShape,
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    span: Option<AlterIdSpan>,
) -> Result<ReadRequest, String> {
    shape.render(company, from, to, span).map(ReadRequest)
}

pub(super) fn voucher_census_read(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
    span: Option<AlterIdSpan>,
) -> Result<ReadRequest, String> {
    render_agent_voucher_census(company, from, to, span).map(ReadRequest)
}

/// The census request for the vouchers of `ledger` (#485): always spanned, the one form the lab
/// observed. The caller supplies the span even for a book that fits one census request.
pub(super) fn ledger_census_read(
    company: &str,
    ledger: &LedgerScopeName,
    from: &TallyDate,
    to: &TallyDate,
    span: AlterIdSpan,
) -> Result<ReadRequest, String> {
    render_agent_ledger_census(company, ledger, from, to, span).map(ReadRequest)
}

/// One part of a read of the vouchers of `ledger`: the movement read (#485), or the window read's
/// fields for the `outstandings` party detail (#1356). Only these two were observed ledger-scoped,
/// and only without an AlterID span, so any other shape or a part carrying a span is refused here,
/// before it is rendered, with [`LEDGER_SCOPE_NEEDS_DAY_SPLIT`].
pub(super) fn ledger_part_read(
    shape: super::voucher_window::VoucherReadShape,
    company: &str,
    ledger: &LedgerScopeName,
    from: &TallyDate,
    to: &TallyDate,
    span: Option<AlterIdSpan>,
) -> Result<ReadRequest, String> {
    use super::voucher_window::VoucherReadShape;
    let fields = match (shape, span) {
        (VoucherReadShape::Movement, None) => LedgerVoucherFields::Movement,
        (VoucherReadShape::EntryWildcard, None) => LedgerVoucherFields::Window,
        _ => return Err(super::voucher_window::LEDGER_SCOPE_NEEDS_DAY_SPLIT.to_string()),
    };
    render_agent_ledger_vouchers(company, ledger, fields, from, to).map(ReadRequest)
}

#[cfg(feature = "lab-writes")]
pub(super) fn lab_inventory_vouchers_read(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
) -> Result<ReadRequest, String> {
    render_agent_lab_inventory_vouchers(company, from, to).map(ReadRequest)
}

#[cfg(feature = "lab-writes")]
pub(super) fn lab_purchase_identity_vouchers_read(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
) -> Result<ReadRequest, String> {
    render_agent_lab_purchase_identity_vouchers(company, from, to).map(ReadRequest)
}

/// The agent's class-resolved window read, unnarrowed: what `vouchers` sends when it filters by
/// type, here the read the candidate narrow read is compared with.
#[cfg(feature = "lab-writes")]
pub(super) fn lab_agent_class_vouchers_read(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
) -> Result<ReadRequest, String> {
    render_agent_class_vouchers_in_span(company, from, to, None).map(ReadRequest)
}

#[cfg(feature = "lab-writes")]
pub(super) fn lab_voucher_key_probe_read(
    company: &str,
    day: &TallyDate,
    term: super::lab::ProbeTerm,
    key: &super::lab::ProbeKey,
) -> Result<ReadRequest, String> {
    super::lab::render_lab_voucher_key_probe(company, day, term, key).map(ReadRequest)
}

#[cfg(feature = "lab-writes")]
pub(super) fn lab_master_collection_read(
    company: &str,
    kind: super::lab::LabMasterKind,
    period: &bridge_tally_protocol::native_outstandings::NativeLedgerExportPeriod,
) -> Result<ReadRequest, String> {
    super::lab::render_lab_master_collection(company, kind, period).map(ReadRequest)
}

#[cfg(feature = "lab-writes")]
pub(super) fn lab_write_master_collection_read(
    company: &str,
    kind: super::lab::import::MasterKind,
    period: &bridge_tally_protocol::native_outstandings::NativeLedgerExportPeriod,
) -> Result<ReadRequest, String> {
    super::lab::import::render_master_collection_request(company, kind, period).map(ReadRequest)
}

#[cfg(feature = "lab-writes")]
pub(super) fn lab_voucher_window_read(
    company: &str,
    from: &TallyDate,
    to: &TallyDate,
) -> Result<ReadRequest, String> {
    super::lab::import::render_voucher_window_request(company, from, to).map(ReadRequest)
}

#[cfg(test)]
mod sealed_read_tests {
    use super::*;
    use crate::tally::agent_read_request::AgentReadRequest;

    /// Every sealed profile renders a read envelope the dispatch gate admits.
    /// A profile added here that rendered anything else would fail this test
    /// before it could reach `post_read`.
    #[test]
    fn every_sealed_profile_renders_an_admitted_read_envelope() {
        let company = "Bridge Sealed Read Co";
        let span = Some(AlterIdSpan {
            after: 10,
            through: 20,
        });
        let (from, to) = (tally_date("20260401"), tally_date("20260430"));
        let mut reads = vec![
            company_high_water_read(company),
            standard_ledger_catalog_read(company).unwrap(),
            native_group_snapshot_read(company),
            voucher_census_read(company, &from, &to, span).unwrap(),
            gst_status_read(
                company,
                &from,
                &to,
                &GstStatusTypeName::parse("Sales").unwrap(),
            )
            .unwrap(),
        ];
        for shape in [
            super::super::voucher_window::VoucherReadShape::ImportVerification,
            super::super::voucher_window::VoucherReadShape::Movement,
            super::super::voucher_window::VoucherReadShape::EntryWildcard,
            super::super::voucher_window::VoucherReadShape::ClassEntryWildcard,
        ] {
            for part_span in [None, span] {
                reads
                    .push(voucher_window_part_read(shape, company, &from, &to, part_span).unwrap());
            }
        }
        for read in reads {
            assert!(
                AgentReadRequest::parse(read.as_str().to_string()).is_ok(),
                "{}",
                read.as_str()
            );
        }
    }
}
