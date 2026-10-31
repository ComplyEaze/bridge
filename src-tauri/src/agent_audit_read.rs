//! The audit read: one `tally-read-v1` assembled in memory from live reads and
//! loaded by the tax-audit crate's public API, unchanged.
//!
//! Each part is one [`fetch_audit_part`] (single, identity-bracketed, kept byte
//! for byte), the vouchers come from the sealing window reader, and the company
//! marks are read before the first part and again by the window after the last.
//! Nothing is written to disk (a lab build may keep an admitted read: see
//! `lab::persist_audit_read`). The crate then does what it does for a directory:
//! `Read::open_from` (C1-C4, every blob re-hashed), `Read::check` (pin, period,
//! bracket, windows) and `book::load_book` (C5, C8, C9, C11). A read it refuses
//! is refused by its rule code; no detail, which can hold a ledger name, leaves
//! this module.
//!
//! [`fetch_audit_part`]: crate::tally::runtime::TallyRuntime::fetch_audit_part
use super::voucher_window::{
    company_marks, AuditWindowFailure, AuditWindowReader, CompanyMarks, RetainedWindowRead,
    VoucherReadShape, WindowReadKind, WindowReadLimits, WINDOW_CHANGED_DURING_READ,
    WINDOW_DIVIDED_BY_ALTERID,
};
use super::*;
use crate::tally::agent_read_request::AgentReadRequest;
use crate::tally::runtime::{AuditPart, AuditPartFailure, AuditPartShape, TallyRuntime};
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriod;
use bridge_tally_protocol::xml_read_profiles::{
    ReadOnlyProfile, ValidatedCompanyName, ValidatedDateRange,
};
use bridge_tax_audit::book::Book;
use bridge_tax_audit::read::{Read, ReadStore};
use bridge_tax_audit::{AuditError, Engagement};
use serde_json::json;
use std::io::Cursor;

/// The assessment year the crate's rules exist for (`rules_for` refuses any other).
const ASSESSMENT_YEAR: &str = "2026-27";

const MARKS_MISSING: &str = "audit_window_marks_missing";
const BOOK_UNUSABLE: &str = "audit_book_refused";
const PART_UNPARSABLE: &str = "audit_part_unparsable";
/// The window admitted a different number of vouchers than its parts hold.
const ROW_COUNT_DIFFERS: &str = "audit_window_row_count_differs";
/// The window's rows were not admitted against a census of it, so nothing
/// outside the data read says the window is whole (the completeness rule of
/// `window_read`, ADR 0017). A book that has never held a voucher has no
/// census and is refused here too.
const WINDOW_NOT_COUNTED: &str = "audit_window_not_counted";

/// The groups whose ledgers a cash test reads, supplied by the caller: there is
/// no default for what a firm calls its cash and bank groups.
#[derive(Clone, Debug)]
pub(super) struct AuditRoles {
    pub(super) cash_groups: Vec<String>,
    pub(super) bank_groups: Vec<String>,
}

/// A book the crate admitted, with the engagement it was admitted under and
/// the requests and responses it rests on.
pub(super) struct AuditBook {
    pub(super) engagement: Engagement,
    pub(super) book: Book,
    pub(super) evidence: Evidence,
    /// The admitted read's stored bytes, kept only in a lab build, which can
    /// write them out for a real-book parity record (`lab::persist_audit_read`).
    #[cfg(feature = "lab-writes")]
    pub(super) stored: StoredRead,
}

/// An admitted read as the crate opened it: its manifest, the manifest's
/// sha256 and each part's bytes by their manifest path (`parts/<name>`).
#[cfg(feature = "lab-writes")]
pub(super) struct StoredRead {
    pub(super) read_id: String,
    pub(super) company_guid: String,
    pub(super) manifest: Vec<u8>,
    pub(super) manifest_sha256: String,
    pub(super) blobs: std::collections::BTreeMap<String, Vec<u8>>,
}

/// Why the read was not made or not admitted: a code and whether the same read
/// may be tried again later.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct AuditReadFailure {
    pub(super) code: String,
    pub(super) retryable: bool,
}

impl AuditReadFailure {
    fn new(code: impl Into<String>, retryable: bool) -> Self {
        Self {
            code: code.into(),
            retryable,
        }
    }
}

impl From<AuditPartFailure> for AuditReadFailure {
    fn from(failure: AuditPartFailure) -> Self {
        // One rule for a part wherever it was read: a deadline would send the
        // same request again and owe another drain.
        Self::from(AuditWindowFailure::Part(failure.kind))
    }
}

impl From<AuditWindowFailure> for AuditReadFailure {
    fn from(failure: AuditWindowFailure) -> Self {
        let retryable = failure.retryable();
        let code = match failure {
            AuditWindowFailure::Part(kind) => kind.code().to_string(),
            AuditWindowFailure::WindowChanged => WINDOW_CHANGED_DURING_READ.to_string(),
            AuditWindowFailure::Refused(code) => code,
        };
        Self::new(code, retryable)
    }
}

impl From<AuditError> for AuditReadFailure {
    fn from(error: AuditError) -> Self {
        let code = match &error {
            AuditError::Refused { code, .. } => code,
            AuditError::Parse { .. } => PART_UNPARSABLE,
            _ => BOOK_UNUSABLE,
        };
        // A bracket that moved is someone else writing; every other refusal
        // would repeat.
        Self::new(
            code,
            matches!(&error, AuditError::Refused { code, .. } if *code == "C6-moved"),
        )
    }
}

/// One part of the read, as it arrived.
struct Collected {
    id: String,
    kind: &'static str,
    name: String,
    window: Option<(TallyDate, TallyDate)>,
    /// A vouchers part's `(rows, alter_id_max)`, counted from its own bytes.
    voucher_counts: Option<(u64, Option<u64>)>,
    request_sha256: String,
    bytes: Vec<u8>,
}

/// The stored bytes of one read, held in memory.
struct MemoryStore {
    manifest: Vec<u8>,
    blobs: std::collections::BTreeMap<String, Vec<u8>>,
}

impl ReadStore for MemoryStore {
    fn manifest_bytes(&self) -> bridge_tax_audit::Result<Box<dyn std::io::Read + '_>> {
        Ok(Box::new(Cursor::new(self.manifest.as_slice())))
    }

    fn stored_blob(
        &self,
        path: &str,
    ) -> bridge_tax_audit::Result<Option<Box<dyn std::io::Read + '_>>> {
        Ok(self
            .blobs
            .get(path)
            .map(|bytes| Box::new(Cursor::new(bytes.as_slice())) as Box<dyn std::io::Read + '_>))
    }
}

fn iso(date: &TallyDate) -> String {
    let text = date.as_str();
    format!("{}-{}-{}", &text[0..4], &text[4..6], &text[6..8])
}

fn refused(code: &str) -> AuditReadFailure {
    AuditReadFailure::new(code, false)
}

async fn fetch(
    runtime: &TallyRuntime,
    config: &TallyConfig,
    identity: &VerifiedCompanyIdentity,
    request: AgentReadRequest,
) -> Result<AuditPart, AuditReadFailure> {
    Ok(runtime
        .fetch_audit_part(config.clone(), identity, request, AuditPartShape::Single)
        .await?)
}

async fn fetch_xml(
    runtime: &TallyRuntime,
    config: &TallyConfig,
    identity: &VerifiedCompanyIdentity,
    request: String,
) -> Result<AuditPart, AuditReadFailure> {
    let request = AgentReadRequest::parse(request).map_err(|error| refused(&error.to_string()))?;
    fetch(runtime, config, identity, request).await
}

fn single(
    id: &str,
    kind: &'static str,
    name: &str,
    window: Option<(TallyDate, TallyDate)>,
    part: AuditPart,
) -> Collected {
    Collected {
        id: id.to_string(),
        kind,
        name: name.to_string(),
        window,
        voucher_counts: None,
        request_sha256: part.evidence.request_sha256,
        bytes: part.encoded_body,
    }
}

/// `(rows, alter_id_max)` as the crate's loader counts them (C9): every
/// `VOUCHER` that is not the template row, and the largest `ALTERID`.
fn count_vouchers(bytes: &[u8], id: &str) -> Result<(u64, Option<u64>), AuditReadFailure> {
    let root = bridge_tax_audit::xml::read(bytes, id)?;
    let (mut rows, mut alter_id_max) = (0_u64, None::<u64>);
    for voucher in root.descendants_named("VOUCHER") {
        if voucher.attr("REMOTEID").is_none_or(str::is_empty) && voucher.child("GUID").is_none() {
            continue;
        }
        rows += 1;
        let alter_id = voucher.child_text("ALTERID");
        if !alter_id.is_empty() {
            let alter_id = alter_id
                .parse::<u64>()
                .map_err(|_| refused(PART_UNPARSABLE))?;
            alter_id_max = Some(alter_id_max.map_or(alter_id, |max| max.max(alter_id)));
        }
    }
    Ok((rows, alter_id_max))
}

/// A string as a TOML basic string. JSON's escapes are a subset of TOML's.
fn toml_string(text: &str) -> String {
    serde_json::to_string(text).expect("a string always serializes")
}

fn toml_strings(items: &[String]) -> String {
    let items: Vec<String> = items.iter().map(|item| toml_string(item)).collect();
    format!("[{}]", items.join(", "))
}

/// The engagement of a read Bridge made itself. The pin is the verified
/// identity, so the crate proves the read's company is the one that was named.
fn engagement_toml(
    identity: &VerifiedCompanyIdentity,
    (from, to): (&TallyDate, &TallyDate),
    roles: &AuditRoles,
) -> Result<String, AuditReadFailure> {
    let books_from = TallyDate::parse(identity.books_from_yyyymmdd())
        .map_err(|_| refused("company_books_from_invalid"))?;
    Ok(format!(
        "[client]\nlabel = {label}\nassessment_year = {ASSESSMENT_YEAR:?}\n\
         [client.tally]\ncompany_guid = {guid}\nbooks_from = {books_from:?}\n\
         [period]\nstart = {from:?}\nend = {to:?}\n\
         [roles]\ncash_groups = {cash}\nbank_groups = {bank}\n",
        label = toml_string(identity.display_name()),
        guid = toml_string(identity.company_guid()),
        books_from = iso(&books_from),
        from = iso(from),
        to = iso(to),
        cash = toml_strings(&roles.cash_groups),
        bank = toml_strings(&roles.bank_groups),
    ))
}

fn part_json(part: &Collected) -> serde_json::Value {
    let encoding = if part.bytes.starts_with(&[0xFF, 0xFE]) || part.bytes.get(1) == Some(&0) {
        "utf-16le"
    } else {
        "utf-8"
    };
    let sha256 = sha256_hex(&part.bytes);
    let mut value = json!({
        "id": part.id,
        "kind": part.kind,
        "name": part.name,
        "request": { "stored": false, "sha256": part.request_sha256 },
        "response": {
            "path": format!("parts/{}", part.name),
            "storage": "identity",
            "stored_sha256": sha256,
            "stored_bytes": part.bytes.len(),
            "sha256": sha256,
            "bytes": part.bytes.len(),
            "media_type": "application/xml",
            "text_encoding": encoding,
            "wire_exact": true,
            "transformation": "none",
        },
    });
    if let Some((from, to)) = &part.window {
        value["window"] = json!({ "from": iso(from), "to": iso(to) });
    }
    if let Some((rows, alter_id_max)) = part.voucher_counts {
        value["rows"] = json!(rows);
        value["alter_id_max"] = json!(alter_id_max);
    }
    value
}

/// The manifest and blobs of one read, in the order the parts were read.
fn memory_read(
    identity: &VerifiedCompanyIdentity,
    period: (&TallyDate, &TallyDate),
    marks: (CompanyMarks, CompanyMarks),
    parts: &[Collected],
) -> Result<(MemoryStore, String, String), AuditReadFailure> {
    let (before, after) = marks;
    let books_from = TallyDate::parse(identity.books_from_yyyymmdd())
        .map_err(|_| refused("company_books_from_invalid"))?;
    let read_id = format!(
        "bridge-audit-{}-{}-{}-{}",
        period.0.as_str(),
        period.1.as_str(),
        before.vouchers(),
        before.masters()
    );
    let read_at = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let side = |marks: CompanyMarks, part: &str| {
        json!({
            "alter_voucher_id": marks.reported_vouchers(),
            "alter_master_id": marks.masters(),
            "part": part,
        })
    };
    let manifest = json!({
        "format": "tally-read",
        "format_version": "1.1",
        "read_id": read_id,
        "created_at": read_at,
        "read_at": read_at,
        "producer": {
            "name": "ComplyEaze Bridge",
            "version": env!("CARGO_PKG_VERSION"),
            "kind": "bridge",
        },
        "company": {
            "guid": identity.company_guid(),
            "name": identity.display_name(),
            "books_from": iso(&books_from),
        },
        "tally": { "basis": "not_recorded", "license_tier": null, "education_mode": null },
        "period": { "from": iso(period.0), "to": iso(period.1) },
        "consistency": {
            "status": if before == after { "unchanged" } else { "moved" },
            "before": side(before, "high-water-before"),
            "after": side(after, "high-water-after"),
        },
        "parts": parts.iter().map(part_json).collect::<Vec<_>>(),
    });
    let manifest = serde_json::to_vec(&manifest).expect("a manifest always serializes");
    let manifest_sha256 = sha256_hex(&manifest);
    let blobs = parts
        .iter()
        .map(|part| (format!("parts/{}", part.name), part.bytes.clone()))
        .collect();
    Ok((MemoryStore { manifest, blobs }, read_id, manifest_sha256))
}

/// Read one company's books for `period` and have the crate admit them.
///
/// Requests go one at a time, each between two identity brackets, and none is
/// anything but an Export. The marks are read first; the window reads them
/// again after the last voucher part; if they differ, or any part could not be
/// read, nothing is returned. The window is always counted by a census, a book
/// that fits one request included, and a window the census did not count is
/// refused (`audit_window_not_counted`).
pub(super) async fn read_audit_book(
    runtime: &TallyRuntime,
    config: &TallyConfig,
    identity: &VerifiedCompanyIdentity,
    period: (&TallyDate, &TallyDate),
    roles: &AuditRoles,
    limits: WindowReadLimits,
) -> Result<AuditBook, AuditReadFailure> {
    let engagement = Engagement::from_toml_for_read(&engagement_toml(identity, period, roles)?)?;
    // C5 refuses a company whose books begin after the period ends. The verified
    // identity already holds that date, and the company part must match it, so
    // the same refusal comes before any request rather than after the whole read.
    if identity.books_from_yyyymmdd() > period.1.as_str() {
        return Err(refused("C5-books-from"));
    }
    let (parts, marks) = collect_parts(runtime, config, identity, period, limits).await?;
    let (store, read_id, manifest_sha256) = memory_read(identity, period, marks, &parts)?;
    let admitted = Read::open_from(&store, &read_id, &manifest_sha256)?;
    admitted.check(
        &engagement.period,
        engagement.allow_unbracketed_read,
        engagement.company_pin.as_ref(),
    )?;
    let book = bridge_tax_audit::book::load_book(&admitted, identity.display_name())?;
    Ok(AuditBook {
        engagement,
        book,
        evidence: evidence_of(&parts),
        #[cfg(feature = "lab-writes")]
        stored: StoredRead {
            read_id,
            company_guid: identity.company_guid().to_string(),
            manifest: store.manifest,
            manifest_sha256,
            blobs: store.blobs,
        },
    })
}

/// Every part's request and response, folded in the order they were read.
fn evidence_of(parts: &[Collected]) -> Evidence {
    parts
        .iter()
        .map(|part| Evidence {
            request_sha256: part.request_sha256.clone(),
            response_sha256: sha256_hex(&part.bytes),
            bytes: part.bytes.len(),
            state: "complete",
            read_at: None,
            duration_ms: None,
            reason_code: None,
        })
        .reduce(combine_evidence)
        .expect("a read has parts")
}

/// The limits the audit's window is read under. A book small enough for one
/// request is counted too: the census is the only count of the window from
/// outside its own data (ADR 0017). The period cannot be narrowed, so its read
/// count is refused only when no measurement could fit it (#2173).
pub(super) const fn audit_window_limits(limits: WindowReadLimits) -> WindowReadLimits {
    limits.counting_small_books().refusing_at_the_floor()
}

/// Every part of the read, in order, and the marks it opened and closed on.
async fn collect_parts(
    runtime: &TallyRuntime,
    config: &TallyConfig,
    identity: &VerifiedCompanyIdentity,
    period: (&TallyDate, &TallyDate),
    limits: WindowReadLimits,
) -> Result<(Vec<Collected>, (CompanyMarks, CompanyMarks)), AuditReadFailure> {
    let name = identity.display_name();
    let company = ValidatedCompanyName::new(name).map_err(|_| refused("company_name_invalid"))?;
    let range = ValidatedDateRange::new(period.0.as_str(), period.1.as_str())
        .map_err(|_| refused("invalid_date_range"))?;
    let guid = identity.company_guid();
    let mut parts = Vec::new();

    let opening = fetch_xml(
        runtime,
        config,
        identity,
        company_high_water_read(name).into_xml(),
    )
    .await?;
    let before = company_marks(&opening.body, guid).map_err(|code| refused(&code))?;
    parts.push(single(
        "high-water-before",
        "company_high_water",
        "high_water_before.xml",
        None,
        opening,
    ));

    let company_object =
        AgentReadRequest::company_object(identity).map_err(|error| refused(&error.to_string()))?;
    let company_part = fetch(runtime, config, identity, company_object).await?;
    bridge_tally_protocol::audit_company_part::admit_audit_company_part(
        &company_part.body,
        guid,
        identity.books_from_yyyymmdd(),
    )
    .map_err(|error| refused(error.code()))?;
    let profile = company_part.boundary_profile;
    parts.push(single(
        "company",
        "company",
        "company_object.xml",
        None,
        company_part,
    ));

    let groups = fetch_xml(
        runtime,
        config,
        identity,
        native_group_snapshot_read(name).into_xml(),
    )
    .await?;
    parts.push(single("groups", "groups", "groups.xml", None, groups));

    let ledgers = ReadOnlyProfile::AuditLedgersV1 {
        company: &company,
        period: &range,
    }
    .render();
    let ledgers = fetch_xml(runtime, config, identity, ledgers).await?;
    parts.push(single("ledgers", "ledgers", "ledgers.xml", None, ledgers));

    let snapshot = NativeLedgerSnapshotPeriod::new(profile, period.0.clone(), period.1.clone())
        .map_err(|_| refused("audit_part_window_unsupported_in_education"))?;
    let trial_balance =
        bridge_tally_protocol::native_trial_balance::render_native_trial_balance_request(
            name, &snapshot,
        );
    let trial_balance = fetch_xml(runtime, config, identity, trial_balance).await?;
    let window = Some((period.0.clone(), period.1.clone()));
    parts.push(single(
        "trial-balance",
        "trial_balance",
        "trial_balance.xml",
        window,
        trial_balance,
    ));

    let voucher_types = fetch_xml(
        runtime,
        config,
        identity,
        voucher_type_catalogue_read(name).into_xml(),
    )
    .await?;
    parts.push(single(
        "voucher-types",
        "voucher_types",
        "voucher_types.xml",
        None,
        voucher_types,
    ));

    let read = AuditWindowReader::new(runtime, config.clone())
        .read_window(
            identity,
            name,
            period.0,
            period.1,
            VoucherReadShape::EntryWildcard,
            audit_window_limits(limits),
            |xml| parse_agent_rows(xml, guid),
        )
        .await;
    if let Some(failure) = read.failure {
        return Err(failure.into());
    }
    let outcome = read.outcome.map_err(|failure| refused(&failure.code))?;
    if !outcome.counted() {
        return Err(refused(WINDOW_NOT_COUNTED));
    }
    let admitted_rows = outcome.rows.len();
    let retained = read.retained.ok_or_else(|| refused(MARKS_MISSING))?;
    let first_voucher_part = parts.len();
    let closing = collect_window(&retained, guid, &mut parts)?;
    check_row_count(admitted_rows, &parts[first_voucher_part..])?;
    Ok((parts, (before, closing)))
}

/// The rows the window admitted by its own parser are the rows the crate will
/// count in the parts (C9 compares the crate with itself, so this is the
/// comparison with the window's admission).
fn check_row_count(admitted: usize, parts: &[Collected]) -> Result<(), AuditReadFailure> {
    let held: u64 = parts
        .iter()
        .filter_map(|part| part.voucher_counts)
        .map(|(rows, _)| rows)
        .sum();
    if u64::try_from(admitted) == Ok(held) {
        Ok(())
    } else {
        Err(refused(ROW_COUNT_DIFFERS))
    }
}

/// File a sealed window's reads as parts: its voucher parts in order, and the
/// marks it closed on, which it returns. The census requests are not kept.
fn collect_window(
    retained: &[RetainedWindowRead],
    guid: &str,
    parts: &mut Vec<Collected>,
) -> Result<CompanyMarks, AuditReadFailure> {
    let mut closing = None;
    for read in retained {
        match &read.kind {
            WindowReadKind::Census => {}
            WindowReadKind::Marks => {
                closing = Some((
                    company_marks(&read.part.body, guid).map_err(|code| refused(&code))?,
                    read,
                ));
            }
            WindowReadKind::Part(window) => {
                if window.span.is_some() {
                    return Err(refused(WINDOW_DIVIDED_BY_ALTERID));
                }
                let id = format!("vouchers-{}", iso(&window.from));
                parts.push(Collected {
                    voucher_counts: Some(count_vouchers(&read.part.encoded_body, &id)?),
                    name: format!("{id}.xml"),
                    id,
                    kind: "vouchers",
                    window: Some((window.from.clone(), window.to.clone())),
                    request_sha256: read.request_sha256.clone(),
                    bytes: read.part.encoded_body.clone(),
                });
            }
        }
    }
    // The marks that close the read are the last request it made.
    if !matches!(
        retained.last().map(|read| &read.kind),
        Some(WindowReadKind::Marks)
    ) {
        return Err(refused(MARKS_MISSING));
    }
    let (marks, read) = closing.ok_or_else(|| refused(MARKS_MISSING))?;
    parts.push(Collected {
        id: "high-water-after".to_string(),
        kind: "company_high_water",
        name: "high_water_after.xml".to_string(),
        window: None,
        voucher_counts: None,
        request_sha256: read.request_sha256.clone(),
        bytes: read.part.encoded_body.clone(),
    });
    Ok(marks)
}

#[cfg(test)]
#[path = "agent_audit_read_tests.rs"]
pub(in crate::agent) mod tests;
