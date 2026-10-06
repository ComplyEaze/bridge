//! Vouchers for the local MCP adapter.
use super::*;
use bridge_tally_core::book_presence::WindowRead;
use bridge_tally_core::TallyDate;
use std::collections::BTreeSet;

impl Server {
    pub(super) async fn vouchers(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        selected_voucher_operation(self, args).await
    }
}

/// The question a held `vouchers` window answers (#485): the same company, the
/// same dates and the same selectors, so the same rows. Paging (`offset`,
/// `limit`) and the name of the snapshot are not part of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct VoucherPageKey {
    company_guid: String,
    from: TallyDate,
    to: TallyDate,
    ledger: Option<String>,
    selector: Option<VoucherTypeSelector>,
    /// The search (#1230): a differently searched window is a different question.
    search: Option<VoucherSearch>,
    /// A summary (#1230) is its own question: a listing and a summary never replace each other's
    /// held window, so pages of one cannot be served from a read that belongs to the other.
    summary: Option<SummaryGroup>,
}

impl VoucherPageKey {
    pub(super) fn new(
        identity: &VerifiedCompanyIdentity,
        from: &TallyDate,
        to: &TallyDate,
        ledger: Option<&str>,
        selector: Option<&VoucherTypeSelector>,
        search: Option<&VoucherSearch>,
    ) -> Self {
        Self {
            company_guid: identity.company_guid().to_string(),
            from: from.clone(),
            to: to.clone(),
            ledger: ledger.map(str::to_string),
            selector: selector.cloned(),
            search: search.cloned(),
            summary: None,
        }
    }

    /// The same question asked as a summary by `group`.
    pub(super) fn with_summary(mut self, group: Option<SummaryGroup>) -> Self {
        self.summary = group;
        self
    }
}

/// One `vouchers` window read once and complete (#485): its rows after every
/// check and selector, unredacted and unmarked, held in process memory only,
/// never persisted. Redaction and party marking are applied to each page as it
/// is served. Valid while the company's two marks equal the ones the read
/// opened on: each screen action measured so far moved a mark (protocol
/// reference §11c.5), so a change of that kind makes a later page read afresh
/// or refuse; a change that moves neither mark is not seen (see the README).
pub(super) struct VoucherPageSnapshot {
    id: String,
    key: VoucherPageKey,
    marks: CompanyMarks,
    rows: Arc<Vec<Value>>,
    window: Value,
    voucher_types: Option<Value>,
    /// The ledger the rows were filtered to and how the request reached it,
    /// already redacted, so a served page names the ledger it read as the first
    /// page did (#1076).
    ledger_match: Option<Value>,
    /// The resolved name of that ledger, unredacted: a summary of a held window adds only
    /// that ledger's entries by month or type (#1230).
    selected_ledger: Option<String>,
    /// Why a complete window is complete when it is more than a counted read (an
    /// empty book), so a served page says it too.
    reason: Option<&'static str>,
    read_at: String,
    taken: std::time::Instant,
    bytes: usize,
}

impl VoucherPageSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        key: VoucherPageKey,
        marks: CompanyMarks,
        rows: Arc<Vec<Value>>,
        window: Value,
        voucher_types: Option<Value>,
        ledger_match: Option<Value>,
        selected_ledger: Option<String>,
        reason: Option<&'static str>,
    ) -> Self {
        let bytes = rows.iter().map(|row| row.to_string().len()).sum::<usize>()
            + window.to_string().len()
            + voucher_types
                .as_ref()
                .map_or(0, |types| types.to_string().len())
            + ledger_match
                .as_ref()
                .map_or(0, |ledger| ledger.to_string().len());
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            key,
            marks,
            rows,
            window,
            voucher_types,
            ledger_match,
            selected_ledger,
            reason,
            read_at: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            taken: std::time::Instant::now(),
            bytes,
        }
    }

    fn describe(&self, reused: bool) -> Value {
        json!({
            "id": self.id,
            "master_alter_id": self.marks.masters,
            "voucher_alter_id": self.marks.vouchers,
            "read_at": self.read_at,
            "reused": reused,
        })
    }
}

/// The held `vouchers` windows. Same lifetime and byte cap as the ledger
/// listings (#630): dropped after the TTL, when a newer read of the same
/// question replaces them, when a write through this server touches the
/// company, or when the byte cap evicts them.
pub(super) struct VoucherPages {
    held: Vec<Arc<VoucherPageSnapshot>>,
    ttl: std::time::Duration,
    max_bytes: usize,
}

impl Default for VoucherPages {
    fn default() -> Self {
        Self {
            held: Vec::new(),
            ttl: super::ledgers::LISTING_SNAPSHOT_TTL,
            max_bytes: super::ledgers::LISTING_SNAPSHOT_MAX_BYTES,
        }
    }
}

impl VoucherPages {
    fn purge_expired(&mut self) {
        let ttl = self.ttl;
        self.held.retain(|held| held.taken.elapsed() < ttl);
    }

    /// Whether the window was kept: one larger than the byte cap is not.
    pub(super) fn hold(&mut self, snapshot: Arc<VoucherPageSnapshot>) -> bool {
        self.drop_key(&snapshot.key);
        if snapshot.bytes > self.max_bytes {
            return false;
        }
        while self.held.iter().map(|held| held.bytes).sum::<usize>() + snapshot.bytes
            > self.max_bytes
        {
            let oldest = self
                .held
                .iter()
                .enumerate()
                .min_by_key(|(_, held)| held.taken)
                .map(|(index, _)| index)
                .expect("a held window while over the cap");
            self.held.remove(oldest);
        }
        self.held.push(snapshot);
        true
    }

    fn drop_key(&mut self, key: &VoucherPageKey) {
        self.purge_expired();
        self.held.retain(|held| held.key != *key);
    }

    pub(super) fn current(&mut self, key: &VoucherPageKey) -> Option<Arc<VoucherPageSnapshot>> {
        self.purge_expired();
        self.held.iter().find(|held| held.key == *key).cloned()
    }

    pub(super) fn drop_company(&mut self, company_guid: &str) {
        self.purge_expired();
        self.held
            .retain(|held| !held.key.company_guid.eq_ignore_ascii_case(company_guid));
    }

    #[cfg(test)]
    pub(super) fn limits_for_test(&mut self, ttl: std::time::Duration, max_bytes: usize) {
        self.ttl = ttl;
        self.max_bytes = max_bytes;
    }

    #[cfg(test)]
    pub(super) fn held_count(&self) -> usize {
        self.held.len()
    }
}

/// Whether a read may be held for later pages: its window was counted or
/// corroborated whole, and nothing was withheld from it.
pub(super) fn holdable(window_state: WindowRead, withheld_total: usize) -> bool {
    window_state == WindowRead::Complete && withheld_total == 0
}

/// What a later page of a held window came to: served from the held read, or to
/// be read afresh, with the held window it found moved on (when it did).
enum PageServe {
    Served(ToolOutcome),
    Fresh(Option<Value>),
}

/// The rows of one page, redacted and party-marked as `vouchers` always did.
pub(super) fn page_items(
    server: &Server,
    rows: &[Value],
    offset: usize,
    limit: usize,
) -> Vec<Value> {
    rows.iter()
        .skip(offset)
        .take(limit)
        .map(|row| {
            redact_value(
                mark_voucher_party_names(row.clone()),
                server.settings.redaction,
            )
        })
        .collect()
}

/// What a summary adds up, stated in the result so a reader does not infer more. The exclusions
/// are the same as `ledger_movement`'s; the voucher types are not told apart.
const SUMMARY_BASIS: &str = "every voucher the window, selectors and search selected that is not cancelled, optional or without accounting entries, as ledger_movement counts (a narrowed window is not a ledger's whole movement); post-dated vouchers are summed too: post_dated_included counts those Tally flagged Yes and post_dated_flag_absent those with no flag at all (Tally asserts the flag on every voucher the current read asks for, so that is expected to be 0; only when it is not is a zero in the first no proof that none are post-dated); a voucher type that does not post (a memorandum, a reversing journal, a sales or purchase order, a delivery or receipt note), if the book uses it and Tally exports it with ledger entries, is not told apart and is summed (none of the vouchers in the one window this was checked on were of those types; the book's voucher-type masters were not read)";

/// What one page of a `vouchers` result holds: the vouchers, or with `summarise_by` the
/// buckets (#1230), and the fields only the second carries.
pub(super) struct PageBody {
    items_key: &'static str,
    profile: &'static str,
    pub(super) items: Vec<Value>,
    pub(super) total: usize,
    pub(super) truncated: bool,
    extra: Vec<(&'static str, Value)>,
}

/// The page of `rows` a request asks for, redacted and party-marked as `vouchers` always did,
/// or the page of a summary of them. A summary sums every row of the window, whatever the page.
pub(super) fn render_page_body(
    server: &Server,
    rows: &[Value],
    summary: Option<&SummaryRequest>,
    (offset, limit): (usize, usize),
) -> Result<PageBody, ToolFailure> {
    let Some(request) = summary else {
        let items = page_items(server, rows, offset, limit);
        return Ok(PageBody {
            items_key: "items",
            profile: "agent_vouchers_v1_filters",
            truncated: offset.saturating_add(items.len()) < rows.len(),
            items,
            total: rows.len(),
            extra: Vec::new(),
        });
    };
    let summary = voucher_summary::summarise(rows, request).map_err(ToolFailure::from)?;
    let (page, truncated) =
        voucher_summary::page_buckets(&summary, offset, limit, server.settings.max_bytes / 5);
    Ok(PageBody {
        items_key: "buckets",
        profile: "agent_vouchers_v1_summary",
        items: page
            .into_iter()
            .map(|bucket| redact_value(bucket, server.settings.redaction))
            .collect(),
        total: summary.buckets.len(),
        truncated,
        extra: vec![
            ("summarised_by", json!(request.group.name())),
            ("entries_counted", json!(summary.entries_counted)),
            ("vouchers_summarised", json!(summary.vouchers_summarised)),
            ("excluded_from_buckets", summary.excluded),
            ("post_dated_included", json!(summary.post_dated_included)),
            (
                "post_dated_flag_absent",
                json!(summary.post_dated_flag_absent),
            ),
            ("totals", summary.totals),
            ("basis", json!(SUMMARY_BASIS)),
        ],
    })
}

/// What every page of a `vouchers` read carries besides its selector-specific
/// fields, whether it was read now or served from a held window.
fn voucher_page_payload(
    company: &TallyCompany,
    state: &str,
    reason: Option<&str>,
    body: PageBody,
    offset: usize,
    window: Value,
) -> (Value, bool) {
    let PageBody {
        items_key,
        profile,
        items,
        total,
        truncated,
        extra,
    } = body;
    let mut payload = json!({"company": company_json(company, std::slice::from_ref(company)), "result": {"state": state, "reason": reason, items_key: items, "offset": offset, "total": total, "profile": profile, "window": window}});
    for (key, value) in extra {
        payload["result"][key] = value;
    }
    (payload, truncated)
}

/// The authoritative selected-voucher operation shared by the MCP and the
/// desktop presentation adapter. It owns source admission, catalogue
/// stability, validation, filtering, response shaping, and redaction; callers
/// only choose their adapter policy and dispatch this operation.
pub(crate) async fn selected_voucher_operation(
    server: &Server,
    args: &Value,
) -> Result<ToolOutcome, ToolFailure> {
    let guid = required_string(args, "company_guid")?;
    let from = normalized_date(required_string(args, "from")?)?;
    let to = normalized_date(required_string(args, "to")?)?;
    if from > to {
        return Err("invalid_date_range".to_string().into());
    }
    let (company, identity, accumulated) = server.verified_company(guid).await?;
    selected_voucher_operation_for_verified(
        server,
        args,
        VoucherOperationScope {
            guid: guid.to_string(),
            from,
            to,
            company,
            identity,
            initial_evidence: Some(accumulated),
            composites: VoucherComposites::Withhold,
            held_pages: true,
        },
    )
    .await
}

/// What a voucher whose amount Tally stored as a foreign-currency composite
/// does to the window (#674). Each caller chooses; there is no default.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum VoucherComposites {
    /// The MCP `vouchers` tool: the voucher is withheld and listed, and the
    /// rest of the window is returned.
    Withhold,
    /// A caller that cannot show a withheld voucher: the window is refused, as
    /// before #674.
    Refuse,
}

/// Runs the same operation after the desktop command has already admitted the
/// exact observed company tuple. This avoids degrading that tuple to a GUID or
/// issuing another company-list read before the shared source read.
pub(crate) struct VoucherOperationScope {
    /// Whether a later page may be served from the first page's read (#485).
    /// Only the MCP adapter holds windows: the desktop adapter never does.
    pub(crate) held_pages: bool,
    pub(crate) guid: String,
    pub(crate) from: TallyDate,
    pub(crate) to: TallyDate,
    pub(crate) company: TallyCompany,
    pub(crate) identity: VerifiedCompanyIdentity,
    pub(crate) initial_evidence: Option<Evidence>,
    pub(crate) composites: VoucherComposites,
}

impl VoucherOperationScope {
    /// The desktop screen's scope: the company is already admitted, a window
    /// that cannot show a withheld voucher is refused (#674), and nothing is
    /// held for later pages (#485): its result is shown whole and carries no
    /// `snapshot`.
    pub(crate) fn desktop(
        guid: String,
        from: TallyDate,
        to: TallyDate,
        company: TallyCompany,
        identity: VerifiedCompanyIdentity,
    ) -> Self {
        Self {
            held_pages: false,
            guid,
            from,
            to,
            company,
            identity,
            initial_evidence: None,
            composites: VoucherComposites::Refuse,
        }
    }
}

pub(crate) async fn selected_voucher_operation_for_verified(
    server: &Server,
    args: &Value,
    scope: VoucherOperationScope,
) -> Result<ToolOutcome, ToolFailure> {
    let VoucherOperationScope {
        guid,
        from,
        to,
        company,
        identity,
        initial_evidence,
        composites,
        held_pages,
    } = scope;
    let mut accumulated = initial_evidence;
    let outcome = async {
        // Parsed before the window is read, so a conflicting request costs only the identity read.
        let type_selector = VoucherTypeSelector::from_args(args)?;
        let search = VoucherSearch::from_args(args, server.settings.redaction)?;
        let summary_group = SummaryGroup::from_args(args)?;
        // A type GUID that is not this company's cannot name any of its
        // types: refused rather than answered with an empty selection.
        if let Some(VoucherTypeSelector::Guid(type_guid)) = &type_selector {
            if !bridge_tally_protocol::master_guid_belongs_to_company(
                type_guid,
                identity.company_guid(),
            ) {
                return Err(ToolFailure::from("voucher_type_guid_foreign".to_string()));
            }
        }
        let requested_ledger = optional_string(args, "ledger")?;
        let offset = arg_usize(args, "offset", 0)?;
        let limit =
            arg_positive_usize(args, "limit", server.settings.max_rows)?.min(server.settings.max_rows);
        // #485: a later page of a window this server already read is served from
        // that read while the company's marks are unchanged. A page that cannot
        // be served reads afresh and replaces the held window.
        let page_key = held_pages.then(|| {
            VoucherPageKey::new(
                &identity,
                &from,
                &to,
                requested_ledger.as_deref(),
                type_selector.as_ref(),
                search.as_ref(),
            )
            .with_summary(summary_group)
        });
        let mut earlier_snapshot = None;
        if let Some(key) = &page_key {
            if offset > 0 {
                let snapshot_id = optional_string(args, "snapshot_id")?;
                match server
                    .serve_voucher_page(
                        &identity,
                        &company,
                        key,
                        (offset, limit),
                        snapshot_id.as_deref(),
                        &mut accumulated,
                        &guid,
                        summary_group,
                    )
                    .await?
                {
                    PageServe::Served(outcome) => return Ok(outcome),
                    PageServe::Fresh(earlier) => earlier_snapshot = earlier,
                }
            }
            server.drop_voucher_page(key)?;
        }
        let selected_catalogue = if let Some(requested) = requested_ledger {
            let (ledgers, catalogue_evidence) =
                server.read_ledger_catalogue(&identity, &company.name).await?;
            accumulate_evidence(&mut accumulated, catalogue_evidence);
            let resolved = resolve_ledger_or_refuse(
                ledgers.iter().map(String::as_str),
                &requested,
                server.settings.redaction,
            )?;
            Some((resolved, ledgers))
        } else {
            None
        };
        // A type filter asks Tally to resolve each row's type in the same
        // response (bridge#625); every other call sends the request unchanged.
        let shape = if type_selector.is_some() {
            VoucherReadShape::ClassEntryWildcard
        } else {
            VoucherReadShape::EntryWildcard
        };
        let read = server
            .read_entry_window_rows(&identity, &company.name, &from, &to, shape, composites)
            .await?;
        accumulate_evidence(&mut accumulated, read.all_evidence());
        // What each request of the window read cost (#595); the empty-window
        // corroboration below is a read of its own and is not counted here.
        let window = serde_json::to_value(&read.timings).unwrap_or(Value::Null);
        let source_marks = read.witness.as_ref().map(|witness| witness.marks);
        let counted = read.counted();
        // A withheld voucher goes through every date, ledger and type check as
        // a row with no amounts, and is set aside only after them (#674).
        let rows = read.rows.into_iter().map(VoucherRow::into_filter_row).collect();
        let mut rows = validate_then_filter_voucher_rows(rows, from.as_str(), to.as_str(), None)?;
        let empty_window = if rows.is_empty() {
            let (read_evidence, partial, reason) = server
                .corroborate_empty_voucher_read(&identity, &company.name, &from, &to, None, source_marks)
                .await?;
            accumulate_evidence(&mut accumulated, read_evidence);
            Some((partial, reason))
        } else {
            None
        };
        // The window's label, decided before any selector: a selection is a
        // pure function of the window, so a zero from a counted window is a
        // checked zero and one from an uncounted window is not (#985).
        let (window_state, mut corroboration_reason) = window_read(counted, empty_window);
        let mut result_state = match window_state {
            WindowRead::Complete => "complete",
            WindowRead::Partial => "partial",
        };
        if let Some(evidence) = accumulated.as_mut() {
            if window_state == WindowRead::Partial {
                evidence.state = "partial";
            }
            if empty_window.is_some() || corroboration_reason.is_some() {
                evidence.reason_code = corroboration_reason.map(str::to_string);
            }
        }
        // A nonempty, validated source can legitimately have no selector match.
        // Corroborate actual source emptiness before any client-side selector.
        let mut ledger_match = None;
        let mut selected_ledger = None;
        if let Some((ledger, catalogue)) = selected_catalogue {
            let (corroboration, catalogue_evidence) =
                server.read_ledger_catalogue(&identity, &company.name).await?;
            accumulate_evidence(&mut accumulated, catalogue_evidence);
            let initial = catalogue.iter().map(String::as_str).collect::<BTreeSet<_>>();
            let repeated = corroboration.iter().map(String::as_str).collect::<BTreeSet<_>>();
            if initial.len() != catalogue.len()
                || repeated.len() != corroboration.len()
                || initial != repeated
                || rows.iter().flat_map(|row| row["amounts"].as_array().into_iter().flatten())
                    .any(|entry| !initial.contains(entry["ledger"].as_str().unwrap_or_default()))
            {
                return Err("ledger_snapshot_drifted".to_string().into());
            }
            rows = filter_voucher_rows_for_ledger(rows, ledger.name());
            ledger_match = Some(ledger.to_json(server.settings.redaction));
            selected_ledger = Some(ledger.name().to_string());
        }
        let mut voucher_types = None;
        if let Some(selector) = &type_selector {
            let selection = select_voucher_rows(rows, selector).map_err(|refusal| {
                let mut failure = ToolFailure::from(refusal.code.to_string());
                if !refusal.candidates.is_empty() {
                    failure.candidates = Some(Box::new(Candidates {
                        requested: None,
                        items: refusal.candidates.iter().map(WindowVoucherType::json).collect(),
                        miss: None,
                    }));
                }
                failure
            })?;
            // A name that selected nothing may name no type at all (a typo):
            // only then, one read of the book's voucher types tells a
            // confident zero from an unknown name (bridge#664).
            if let (VoucherTypeSelector::Name(name), true) = (selector, selection.rows.is_empty()) {
                let (book, catalogue_evidence) =
                    server.read_voucher_type_catalogue(&identity, &company.name).await?;
                accumulate_evidence(&mut accumulated, catalogue_evidence);
                if let Some(nearest) = unknown_voucher_type(name, &book) {
                    let mut failure = ToolFailure::from("unknown_voucher_type".to_string());
                    failure.candidates = Some(Box::new(Candidates {
                        requested: Some(name.clone()),
                        items: nearest.into_iter().map(BookVoucherType::json).collect(),
                        miss: None,
                    }));
                    return Err(failure);
                }
            }
            rows = selection.rows;
            voucher_types = Some(json!({
                "included": selection.included.iter().map(WindowVoucherType::json).collect::<Vec<_>>(),
                "in_scope": selection.window_types.iter().map(WindowVoucherType::json).collect::<Vec<_>>(),
            }));
        }
        // #1230: the search runs last, on the labelled window, so a zero is a checked zero.
        if let Some(search) = &search {
            rows = search.apply(rows);
        }
        let (withheld, rows): (Vec<Value>, Vec<Value>) =
            rows.into_iter().partition(|row| row.get(WITHHELD_MARKER).is_some());
        let withheld_total = withheld.len();
        if withheld_total > 0 {
            // `items` then does not cover the window: say so in the state an
            // agent reads first, not only in a field it might skip.
            result_state = "partial";
            corroboration_reason = Some("vouchers_withheld");
            if let Some(evidence) = accumulated.as_mut() {
                evidence.state = "partial";
                evidence.reason_code = Some("vouchers_withheld".to_string());
            }
        }
        let rows = Arc::new(rows);
        // #485: a complete window is held, for its later pages. A partial one is
        // not: a later page of it reads afresh, as before.
        let held = match (&page_key, source_marks) {
            (Some(key), Some(marks)) if holdable(window_state, withheld_total) => {
                server.hold_voucher_page(VoucherPageSnapshot::new(
                    key.clone(),
                    marks,
                    rows.clone(),
                    window.clone(),
                    voucher_types.clone(),
                    ledger_match.clone(),
                    selected_ledger.clone(),
                    corroboration_reason,
                ))?
            }
            _ => None,
        };
        let summary = summary_group.map(|group| SummaryRequest {
            group,
            selected_ledger: selected_ledger.clone(),
        });
        let body = render_page_body(server, &rows, summary.as_ref(), (offset, limit))?;
        let (mut payload, truncated) = voucher_page_payload(
            &company,
            result_state,
            corroboration_reason,
            body,
            offset,
            window,
        );
        if let Some(held) = &held {
            payload["result"]["snapshot"] = held.describe(false);
        }
        if let Some(earlier) = earlier_snapshot {
            payload["result"]["earlier_snapshot"] = earlier;
        }
        if let Some(voucher_types) = voucher_types {
            payload["result"]["voucher_types"] = voucher_types;
        }
        // The ledger the rows were filtered to, and how the request reached it.
        if let Some(ledger_match) = ledger_match {
            payload["result"]["ledger_match"] = ledger_match;
        }
        if withheld_total > 0 {
            payload["result"]["withheld_total"] = json!(withheld_total);
            payload["result"]["withheld_vouchers"] =
                Value::Array(listed_withheld(&withheld, server.settings.max_bytes));
            let amount_note = if search.as_ref().is_some_and(VoucherSearch::has_amount) {
                "; an amount search keeps every withheld voucher because its amounts cannot be compared, so some of them may not match"
            } else {
                ""
            };
            payload["result"]["coverage"] = json!(if summary_group.is_some() {
                format!("buckets exclude {withheld_total} voucher(s) whose amounts Tally stored in a foreign currency, so their totals are short by those vouchers; withheld_vouchers lists them up to its bound, withheld_total counts them all, and total counts buckets{amount_note}")
            } else {
                format!("items exclude {withheld_total} voucher(s) whose amounts Tally stored in a foreign currency; withheld_vouchers lists them up to its bound, withheld_total counts them all, and total counts items only{amount_note}")
            });
        }
        // What the cost means for the next call goes on this page only (a later page
        // is served from the held window, which keeps the plain timings), and only
        // when even the smallest page still carries it; a page that must be trimmed
        // loses rows, as for any field, and the window says when the block was left
        // out (#1239).
        let smallest_page = super::read_cost::smallest_page_len(&payload);
        super::read_cost::add_read_cost(
            &mut payload["result"]["window"],
            smallest_page,
            &read.timings,
            super::read_cost::Ended::Read,
            server.settings.max_bytes,
        );
        Ok(ToolOutcome {
            payload,
            evidence: accumulated.clone().expect("voucher source evidence is present after admitted read"),
            company_guid: Some(guid),
            truncated,
        })
    }
    .await;
    outcome.map_err(|failure: ToolFailure| match accumulated {
        Some(evidence) => failure.with_prior_evidence(evidence),
        None => failure,
    })
}

fn accumulate_evidence(target: &mut Option<Evidence>, next: Evidence) {
    *target = Some(match target.take() {
        Some(current) => combine_evidence(current, next),
        None => next,
    });
}

impl Server {
    /// Drops every held `vouchers` window of a company, as a write through this
    /// server does before it returns (see `drop_listing_snapshots`).
    pub(super) fn drop_voucher_pages(&self, company_guid: &str) {
        self.voucher_pages
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .drop_company(company_guid);
    }

    fn drop_voucher_page(&self, key: &VoucherPageKey) -> Result<(), ToolFailure> {
        self.voucher_pages
            .lock()
            .map_err(|_| "listing_snapshot_store_unavailable".to_string())?
            .drop_key(key);
        Ok(())
    }

    /// The held window, or `None` when the byte cap kept it out: a page then
    /// advertises no snapshot it could not be served from.
    fn hold_voucher_page(
        &self,
        snapshot: VoucherPageSnapshot,
    ) -> Result<Option<Arc<VoucherPageSnapshot>>, ToolFailure> {
        let snapshot = Arc::new(snapshot);
        let kept = self
            .voucher_pages
            .lock()
            .map_err(|_| "listing_snapshot_store_unavailable".to_string())?
            .hold(snapshot.clone());
        Ok(kept.then_some(snapshot))
    }

    /// A later page, served from the held window of the same question while the
    /// company's two marks equal the ones that window was read under. It sends
    /// the identity read and one paired marks read, not the window again. When
    /// the caller names its snapshot, anything else refuses: the book moved
    /// (`book_changed_since_first_page`) or the window is not held
    /// (`snapshot_not_held`). Without a name, a page that cannot be served is
    /// read afresh.
    #[allow(clippy::too_many_arguments)]
    async fn serve_voucher_page(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &TallyCompany,
        key: &VoucherPageKey,
        (offset, limit): (usize, usize),
        snapshot_id: Option<&str>,
        accumulated: &mut Option<Evidence>,
        guid: &str,
        summary_group: Option<SummaryGroup>,
    ) -> Result<PageServe, ToolFailure> {
        let held = self
            .voucher_pages
            .lock()
            .map_err(|_| "listing_snapshot_store_unavailable".to_string())?
            .current(key);
        // Nothing held for this question: no marks are read for nothing.
        let Some(held) = held else {
            return match snapshot_id {
                Some(_) => Err(super::ledgers::snapshot_refusal("snapshot_not_held")),
                None => Ok(PageServe::Fresh(None)),
            };
        };
        if snapshot_id.is_some_and(|id| held.id != id) {
            return Err(super::ledgers::snapshot_refusal("snapshot_not_held"));
        }
        let (marks, read) = self
            .read_company_marks_once(identity, &company.name)
            .await?;
        accumulate_evidence(accumulated, read);
        if held.marks != marks {
            return match snapshot_id {
                Some(_) => Err(super::ledgers::snapshot_refusal(
                    "book_changed_since_first_page",
                )),
                // Read afresh, and say so: this page's offsets do not continue
                // the earlier pages of a book that has since changed.
                None => Ok(PageServe::Fresh(Some(json!({
                    "id": held.id,
                    "cause": "book_changed_since_first_page",
                    "offsets_do_not_continue": true,
                })))),
            };
        }
        let snapshot = held;
        let summary = summary_group.map(|group| SummaryRequest {
            group,
            selected_ledger: snapshot.selected_ledger.clone(),
        });
        let body = render_page_body(self, &snapshot.rows, summary.as_ref(), (offset, limit))?;
        if let (Some(reason), Some(evidence)) = (snapshot.reason, accumulated.as_mut()) {
            evidence.reason_code = Some(reason.to_string());
        }
        let (mut payload, truncated) = voucher_page_payload(
            company,
            "complete",
            snapshot.reason,
            body,
            offset,
            snapshot.window.clone(),
        );
        if let Some(voucher_types) = &snapshot.voucher_types {
            payload["result"]["voucher_types"] = voucher_types.clone();
        }
        if let Some(ledger_match) = &snapshot.ledger_match {
            payload["result"]["ledger_match"] = ledger_match.clone();
        }
        payload["result"]["snapshot"] = snapshot.describe(true);
        Ok(PageServe::Served(ToolOutcome {
            payload,
            evidence: accumulated
                .clone()
                .expect("a served page carries its identity and marks evidence"),
            company_guid: Some(guid.to_string()),
            truncated,
        }))
    }

    /// The book's voucher types (`voucher_type_catalogue_read`), bound to the
    /// verified company by their GUIDs.
    async fn read_voucher_type_catalogue(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
    ) -> Result<(Vec<BookVoucherType>, Evidence), ToolFailure> {
        let (xml, evidence) = self
            .post_read(identity, voucher_type_catalogue_read(company))
            .await?;
        let parsed = bridge_tally_protocol::parse_native_voucher_type_source_records_with_evidence(
            &xml,
            identity.company_guid(),
        )
        .map_err(|error| {
            // The code names what failed and the cause why (bridge#676).
            let mut failure = ToolFailure::from("voucher_type_export_invalid".to_string())
                .with_prior_evidence(evidence.clone());
            failure.cause = Some(error.safe_code());
            failure
        })?;
        let book = parsed
            .records
            .into_iter()
            .map(|record| BookVoucherType {
                name: record.record.name,
                guid: record.identities.guid,
            })
            .collect();
        Ok((book, evidence))
    }

    pub(super) async fn corroborate_empty_voucher_read(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        from: &TallyDate,
        to: &TallyDate,
        ledger: Option<&str>,
        known_marks: Option<CompanyMarks>,
    ) -> Result<(Evidence, bool, Option<&'static str>), ToolFailure> {
        let (wider_from, wider_to) = widened_window(from, to)?;
        // The window itself was empty, but the day either side of it need not
        // be, and this read uses the entry wildcard: it is bounded like any
        // other windowed read rather than trusted to be small.
        let wider = self
            .read_entry_wildcard_window(
                identity,
                company,
                &wider_from,
                &wider_to,
                known_marks,
                SmallBooks::Skip,
            )
            .await?;
        let mut evidence = wider.all_evidence();
        let wider_rows = wider.rows;
        let outcome = async {
            let wider_rows = validate_then_filter_voucher_rows(
                wider_rows,
                wider_from.as_str(),
                wider_to.as_str(),
                ledger,
            )?;
            let high_water = if wider_rows.is_empty() {
                let (high_water_xml, high_water_evidence) = self
                    .post_read(identity, company_high_water_read(company))
                    .await?;
                evidence = combine_evidence(evidence.clone(), high_water_evidence);
                Some(company_voucher_high_water(
                    &high_water_xml,
                    identity.company_guid(),
                )?)
            } else {
                None
            };
            let (partial, reason) = corroborate_empty_voucher_window(
                &wider_rows,
                from.as_str(),
                to.as_str(),
                high_water,
            )?;
            Ok((evidence.clone(), partial, reason))
        }
        .await;
        outcome.map_err(|failure: ToolFailure| failure.with_prior_evidence(evidence))
    }
}

impl Server {
    /// A bounded read of the entry-wildcard voucher window (`render_agent_vouchers`)
    /// shared by `voucher_presence` and the empty-window corroboration. Rows
    /// are parsed per part and returned in date order; validating them is the
    /// caller's job, over the union. `small_books` is whether a book that fits
    /// one request is counted first (#1029).
    pub(super) async fn read_entry_wildcard_window(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        from: &TallyDate,
        to: &TallyDate,
        known_marks: Option<CompanyMarks>,
        small_books: SmallBooks,
    ) -> Result<WindowReadOutcome<Value>, ToolFailure> {
        self.read_entry_window_shaped(
            identity,
            company,
            from,
            to,
            known_marks,
            VoucherReadShape::EntryWildcard,
            small_books,
        )
        .await
    }

    /// The `vouchers` window in either entry-wildcard shape, parsed under
    /// `composites` (#674).
    pub(super) async fn read_entry_window_rows(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        from: &TallyDate,
        to: &TallyDate,
        shape: VoucherReadShape,
        composites: VoucherComposites,
    ) -> Result<WindowReadOutcome<VoucherRow>, ToolFailure> {
        self.read_voucher_window(
            identity,
            company,
            from,
            to,
            shape,
            WindowPlanSource::Estimate { known_marks: None },
            WindowReadLimits::for_shape(shape).counting_small_books(),
            |xml| match composites {
                VoucherComposites::Withhold => {
                    parse_agent_rows_withholding(xml, identity.company_guid())
                }
                VoucherComposites::Refuse => parse_agent_rows(xml, identity.company_guid())
                    .map(|rows| rows.into_iter().map(VoucherRow::Read).collect()),
            },
        )
        .await
    }

    /// [`Self::read_entry_wildcard_window`] in either entry-wildcard shape:
    /// plain, or with each row's voucher type resolved (bridge#625).
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn read_entry_window_shaped(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        from: &TallyDate,
        to: &TallyDate,
        known_marks: Option<CompanyMarks>,
        shape: VoucherReadShape,
        small_books: SmallBooks,
    ) -> Result<WindowReadOutcome<Value>, ToolFailure> {
        let limits = WindowReadLimits::for_shape(shape);
        let limits = match small_books {
            SmallBooks::Skip => limits,
            SmallBooks::Count => limits.counting_small_books(),
        };
        self.read_voucher_window(
            identity,
            company,
            from,
            to,
            shape,
            WindowPlanSource::Estimate { known_marks },
            limits,
            |xml| parse_agent_rows(xml, identity.company_guid()),
        )
        .await
    }
}

/// At most this many withheld vouchers are listed; `withheld_total` is exact.
const MAX_WITHHELD_LISTED: usize = 100;

/// The first withheld vouchers, in window order: at most
/// [`MAX_WITHHELD_LISTED`], and no more than a quarter of the response budget,
/// as `candidates` are bounded. Like theirs, the quarter counts each entry's
/// serialized bytes, not the MCP text copy of the payload, which repeats them
/// escaped. `withheld_total` stays exact.
fn listed_withheld(withheld: &[Value], response_budget_bytes: usize) -> Vec<Value> {
    let budget = response_budget_bytes / 4;
    let mut used = 0usize;
    let mut listed = Vec::new();
    for summary in withheld
        .iter()
        .take(MAX_WITHHELD_LISTED)
        .map(withheld_summary)
    {
        used = used.saturating_add(summary.to_string().len());
        if used > budget {
            break;
        }
        listed.push(summary);
    }
    listed
}

/// A withheld voucher as `withheld_vouchers` lists it: identity and cause,
/// with no amount, party or ledger.
fn withheld_summary(row: &Value) -> Value {
    json!({
        "guid": row["guid"], "date": row["date"], "voucher_type": row["voucher_type"],
        "voucher_number": row["voucher_number"], "cause": row[WITHHELD_MARKER],
    })
}

#[cfg(test)]
#[path = "agent_vouchers_tests.rs"]
mod tests;
