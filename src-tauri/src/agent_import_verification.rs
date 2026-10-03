//! Read-back verification of an imported batch.
//!
//! Parses the verification collection, corroborates its window, attributes each
//! observed voucher to the batch by narration marker or accounting fingerprint,
//! classifies every expected voucher, finds duplicates, and renders the proof.
//! Moved out of `agent_import.rs` with only visibility and `super::` paths changed;
//! nothing here dispatches to Tally.
use super::*;

pub(super) fn parse_import_vouchers(
    xml: &str,
    company_guid: &str,
) -> Result<ImportReadSource, String> {
    ImportReadSource::admit(parse_import_voucher_rows(xml, company_guid)?)
}

/// Parse one verification response into rows **without admitting them.**
///
/// `ImportReadSource::admit` enforces identity uniqueness across the whole row
/// set, so a window read in parts must be admitted **once over the union** — not
/// per part. Admitting each part separately would check uniqueness only within
/// each part and let a voucher duplicated across two sub-windows through, which
/// is exactly the kind of thing this read exists to catch.
pub(super) fn parse_import_voucher_rows(
    xml: &str,
    company_guid: &str,
) -> Result<Vec<ReadVoucher>, String> {
    let parsed =
        super::super::parse_import_verification_rows(xml, company_guid).map_err(|code| {
            match code.as_str() {
                // Preserve the import error contract while sharing scalar admission.
                "agent_read_protocol_invalid"
                    if super::super::validate_agent_envelope(xml).is_err() =>
                {
                    "import_verification_protocol_invalid"
                }
                "agent_read_protocol_invalid"
                | "change_row_core_field_invalid"
                | "voucher_date_invalid"
                | "voucher_effective_date_invalid"
                | "voucher_accounting_state_not_observed" => "import_verification_export_invalid",
                "change_row_identity_invalid"
                | "voucher_source_identity_invalid"
                | "voucher_company_identity_invalid" => "import_verification_identity_invalid",
                "voucher_amount_invalid" => "import_verification_amount_invalid",
                "voucher_master_id_invalid" => "import_verification_master_id_invalid",
                _ => return code,
            }
            .to_string()
        })?;
    let mut rows: Vec<ReadVoucher> = parsed
        .into_iter()
        .map(|row| {
            serde_json::from_value(row)
                .map_err(|_| "import_verification_export_invalid".to_string())
        })
        .collect::<Result<_, _>>()?;
    // The shared parser has validated these lexemes; comparisons use canonical
    // Yes/No while preserving original amount strings for proof output.
    for entry in rows.iter_mut().flat_map(|row| &mut row.entries) {
        entry.is_deemed_positive = entry.is_deemed_positive.trim().to_string();
    }
    Ok(rows)
}

// File preflight and later verification require the same window and row identities.
pub(super) fn verification_window_identities(
    observed: &ImportReadSource,
    from: &str,
    to: &str,
) -> Result<BTreeSet<(String, u64)>, String> {
    if observed.rows.iter().any(|voucher| {
        voucher
            .date
            .as_deref()
            .is_none_or(|date| date < from || date > to)
    }) {
        return Err("window_not_honoured".to_string());
    }
    observed
        .rows
        .iter()
        .map(|voucher| {
            Ok((
                voucher
                    .guid
                    .clone()
                    .ok_or_else(|| "verification_incomplete:window_not_corroborated".to_string())?,
                voucher
                    .alter_id
                    .ok_or_else(|| "verification_incomplete:window_not_corroborated".to_string())?,
            ))
        })
        .collect()
}

pub(super) fn corroborate_verification_window(
    observed: &ImportReadSource,
    corroboration: &ImportReadSource,
    from: &str,
    to: &str,
) -> Result<(), String> {
    // This collection request has no row limit. The MCP output-page setting
    // cannot establish source truncation; corroborate the observed identity set
    // independently of that presentation cap.
    if verification_window_identities(observed, from, to)?
        != verification_window_identities(corroboration, from, to)?
    {
        return Err("verification_incomplete:window_not_corroborated".to_string());
    }
    Ok(())
}

pub(super) fn canonical_verification_amount(value: &str) -> Result<String, String> {
    ExactDecimal::parse(value.to_string())
        .and_then(|amount| amount.checked_add(&ExactDecimal::zero()))
        .map(|amount| amount.as_str().to_string())
        .map_err(|_| "import_verification_amount_invalid".to_string())
}

type VerificationFingerprint = (Option<String>, Option<String>, Vec<String>);

#[derive(Default)]
struct VerificationCandidates {
    remaining: BTreeSet<usize>,
    after_mark: BTreeSet<usize>,
    consumed: bool,
}

impl VerificationCandidates {
    fn insert(&mut self, index: usize, after_mark: bool) {
        self.remaining.insert(index);
        if after_mark {
            self.after_mark.insert(index);
        }
    }
    fn consume(&mut self, index: usize) {
        self.consumed |= self.remaining.remove(&index);
        self.after_mark.remove(&index);
    }
}

pub(super) fn observed_fingerprint(voucher: &ReadVoucher) -> VerificationFingerprint {
    (
        voucher.date.clone(),
        voucher.voucher_type.clone(),
        actual_entry_fingerprint(voucher),
    )
}

/// How a batch's vouchers can be attributed to rows of the book.
#[derive(Clone, Copy, Debug)]
pub(super) enum Attribution<'a> {
    /// By the `[BRIDGE:...]` narration tag: the hand-import file, a native
    /// post made before the pre-POST mark was recorded, and the check made
    /// before a native POST.
    Tag,
    /// A native post that sent no tag, attributed only by the vouchers its own
    /// POST was bound to by its AlterID span (`agent_import_span_identity.rs`),
    /// if it was bound. A row carrying the batch's tag cannot be one of the
    /// vouchers this POST created, so the tag attributes nothing: such a row
    /// (a hand import of the batch's file) is matched by content only.
    Span(Option<&'a [super::span_identity::PostedVoucherIdentity]>),
}

/// Verifies a batch against a window read. Bound vouchers rank first, so a
/// bound voucher is found by its GUID, not by a narration tag or its content.
/// A bound voucher the window does not hold is `bound_not_in_window`, never
/// `not_found`: it was posted, so its absence here is not evidence that it is
/// absent.
pub(super) fn verify_batch(
    line: &ImportLedgerLine,
    observed: &ImportReadSource,
    attribution: Attribution<'_>,
) -> Result<Value, String> {
    let (bindings, tags_attribute) = match attribution {
        Attribution::Tag => (None, true),
        Attribution::Span(bindings) => (bindings, false),
    };
    // Normalize only the comparison copies. Persisted batches and generated XML
    // retain their original amount lexemes and remain backward compatible.
    let mut comparison_line = line.clone();
    for entry in comparison_line
        .vouchers
        .iter_mut()
        .flat_map(|v| &mut v.entries)
    {
        entry.amount = canonical_verification_amount(&entry.amount)?;
    }
    let mut comparison_observed = observed.rows.clone();
    for entry in comparison_observed.iter_mut().flat_map(|v| &mut v.entries) {
        entry.amount = canonical_verification_amount(&entry.amount)?;
    }
    let line = &comparison_line;
    let observed = comparison_observed.as_slice();
    let observed_identities = observed
        .iter()
        .map(observed_voucher_identity)
        .collect::<Result<Vec<_>, _>>()?;
    let mut fully_verified_identities = BTreeSet::new();
    let mut rows = Vec::new();
    let expected_fingerprints = line
        .vouchers
        .iter()
        .map(|voucher| {
            (
                normalized_date(&voucher.date).ok(),
                Some(voucher.voucher_type.as_str().to_string()),
                expected_entry_fingerprint(voucher),
            )
        })
        .collect::<Vec<VerificationFingerprint>>();
    let observed_fingerprints = observed
        .iter()
        .map(observed_fingerprint)
        .collect::<Vec<_>>();
    let mut expected_fingerprint_counts = BTreeMap::new();
    for fingerprint in &expected_fingerprints {
        *expected_fingerprint_counts
            .entry(fingerprint)
            .or_insert(0_usize) += 1;
    }
    let expected_markers = line
        .vouchers
        .iter()
        .map(|voucher| line.attribution_tag(voucher))
        .collect::<Vec<_>>();
    let expected_tags = expected_markers
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    // Source admission permits at most one well-formed reserved marker. Parse it
    // once, then reserve expected tags before any fingerprint fallback is used.
    let observed_tags = observed
        .iter()
        .map(|voucher| {
            narration_markers(voucher.narration.as_deref()?)
                .next()
                .flatten()
        })
        .collect::<Vec<_>>();
    // Each bound voucher's row, found by the GUID its POST created. A bound row
    // takes part in no tag or fingerprint match: it is already attributed.
    let bound_rows = line
        .vouchers
        .iter()
        .map(|voucher| {
            let identity = bindings?
                .iter()
                .find(|identity| identity.bridge_txn_id == voucher.bridge_txn_id)?;
            Some((
                identity,
                observed
                    .iter()
                    .position(|row| row.guid.as_deref() == Some(identity.guid.as_str())),
            ))
        })
        .collect::<Vec<_>>();
    let bound_indexes = bound_rows
        .iter()
        .filter_map(|bound| bound.and_then(|(_, index)| index))
        .collect::<BTreeSet<_>>();
    let mut tagged = BTreeMap::<&str, VerificationCandidates>::new();
    let mut fallback = BTreeMap::<&VerificationFingerprint, VerificationCandidates>::new();
    for (index, voucher) in observed.iter().enumerate() {
        if bound_indexes.contains(&index) {
            continue;
        }
        let after_mark = line
            .pre_import_mark
            .value
            .is_some_and(|mark| voucher.alter_id.is_some_and(|id| id > mark));
        if let Some(tag) =
            observed_tags[index].filter(|tag| tags_attribute && expected_tags.contains(tag))
        {
            tagged.entry(tag).or_default().insert(index, after_mark);
        } else {
            fallback
                .entry(&observed_fingerprints[index])
                .or_default()
                .insert(index, after_mark);
        }
    }
    let mut ambiguous_within_batch = Vec::new();
    let mut counts = BTreeMap::from([
        ("bound_not_in_window", 0_u64),
        ("posted_verified", 0_u64),
        ("matching_content_observed", 0),
        ("posted_not_effective", 0),
        ("posted_divergent", 0),
        ("not_found", 0),
        ("not_attributable", 0),
        ("duplicate_fingerprint", 0),
    ]);
    for (((expected, expected_key), marker_identity), bound) in line
        .vouchers
        .iter()
        .zip(&expected_fingerprints)
        .zip(&expected_markers)
        .zip(&bound_rows)
    {
        if let Some((identity, index)) = bound {
            let value = match index {
                None => {
                    counts
                        .entry("bound_not_in_window")
                        .and_modify(|count| *count += 1);
                    json!({"bridge_txn_id":expected.bridge_txn_id,"status":"bound_not_in_window","marker":"post_span_binding","guid":identity.guid,"master_id":identity.master_id.to_string(),"next_step":plain_next_step("bound_not_in_window")})
                }
                Some(index) => {
                    let matched = &observed[*index];
                    if matched.master_id.as_deref() != Some(identity.master_id.to_string().as_str())
                    {
                        // Refuse-only: the GUID found the row, but its MasterID
                        // is not the one bound. Nothing is attributed to it.
                        counts
                            .entry("not_attributable")
                            .and_modify(|count| *count += 1);
                        json!({"bridge_txn_id":expected.bridge_txn_id,"status":"not_attributable","marker":"post_span_binding","reason":"bound_master_id_changed"})
                    } else {
                        let mut value = attributed_status(
                            expected,
                            matched,
                            "post_span_binding",
                            expected_key.2 == observed_fingerprints[*index].2,
                            &mut counts,
                        )?;
                        if value["status"] == "posted_verified" {
                            fully_verified_identities.insert(observed_identities[*index].clone());
                        }
                        if effective_date_not_observed(expected, matched) {
                            value["not_observed"] = json!(["effective_date"]);
                        }
                        value
                    }
                }
            };
            rows.push(value);
            continue;
        }
        let fingerprint_ambiguous_within_batch = expected_fingerprint_counts[expected_key] > 1;
        let tagged_group = tagged.get(marker_identity.as_str());
        let fingerprint_fallback = tagged_group.is_none();
        let group = tagged_group.or_else(|| fallback.get(expected_key));
        let marker = if fingerprint_fallback {
            "accounting_fingerprint"
        } else {
            "narration_tag"
        };
        let not_attributable = line.pre_import_mark.value.is_some()
            && group.is_some_and(|group| !fingerprint_fallback || !group.remaining.is_empty());
        let match_count = group.map_or(0, |group| group.after_mark.len());
        let matched_index = group.and_then(|group| group.after_mark.first().copied());
        let already_consumed = tagged_group.is_some_and(|group| group.consumed);
        let mut value = if not_attributable && match_count == 0 {
            counts
                .entry("not_attributable")
                .and_modify(|count| *count += 1);
            let reason = if already_consumed {
                "observed_voucher_already_attributed"
            } else if marker == "narration_tag" {
                "tag_precedes_pre_import_voucher_mark"
            } else {
                "fingerprint_precedes_pre_import_voucher_mark"
            };
            json!({"bridge_txn_id":expected.bridge_txn_id,"status":"not_attributable","marker":marker,"reason":reason})
        } else if match_count == 0 {
            counts.entry("not_found").and_modify(|count| *count += 1);
            json!({"bridge_txn_id":expected.bridge_txn_id,"status":"not_found"})
        } else if match_count > 1 {
            counts
                .entry("duplicate_fingerprint")
                .and_modify(|count| *count += 1);
            json!({"bridge_txn_id":expected.bridge_txn_id,"status":"duplicate_fingerprint","marker":marker,"matches":match_count})
        } else {
            let matched_index = matched_index.expect("one indexed candidate");
            if let Some(tag) = observed_tags[matched_index] {
                if let Some(group) = tagged.get_mut(tag) {
                    group.consume(matched_index);
                }
            }
            if let Some(group) = fallback.get_mut(&observed_fingerprints[matched_index]) {
                group.consume(matched_index);
            }
            let matched = &observed[matched_index];
            let effective_date_unobserved = effective_date_not_observed(expected, matched);
            let diffs = voucher_diffs(
                expected,
                matched,
                expected_key.2 == observed_fingerprints[matched_index].2,
            );
            let mut matched_value = if !fingerprint_fallback {
                let value = attributed_status(
                    expected,
                    matched,
                    marker,
                    expected_key.2 == observed_fingerprints[matched_index].2,
                    &mut counts,
                )?;
                if value["status"] == "posted_verified" {
                    fully_verified_identities.insert(observed_identities[matched_index].clone());
                }
                value
            } else {
                counts
                    .entry("matching_content_observed")
                    .and_modify(|count| *count += 1);
                json!({"bridge_txn_id":expected.bridge_txn_id,"status":"matching_content_observed","marker":marker,"attribution":"not_established","accounting_effective":voucher_is_accounting_effective(matched)?,"diffs":diffs,"voucher_number":matched.voucher_number,"guid":matched.guid,"master_id":matched.master_id,"alter_id":matched.alter_id})
            };
            if effective_date_unobserved {
                matched_value["not_observed"] = json!(["effective_date"]);
            }
            matched_value
        };
        if fingerprint_fallback && fingerprint_ambiguous_within_batch {
            value["ambiguous_within_batch"] = Value::Bool(true);
            ambiguous_within_batch.push(expected.bridge_txn_id.clone());
        }
        rows.push(value);
    }
    let (batch_duplicates, unrelated_duplicates_in_window) = batch_duplicate_sets(
        observed,
        &observed_identities,
        &observed_fingerprints,
        &expected_fingerprint_counts,
        &observed_tags,
        &expected_tags,
        &fully_verified_identities,
    );
    Ok(
        json!({"counts":counts,"vouchers":rows,"duplicates":batch_duplicates,"unrelated_duplicates_in_window":unrelated_duplicates_in_window,"ambiguous_within_batch":ambiguous_within_batch}),
    )
}

/// The status of a voucher attributed to `matched` by an identity (its
/// narration tag, or the GUID its own POST was bound to), counted in `counts`.
fn attributed_status(
    expected: &ImportVoucher,
    matched: &ReadVoucher,
    marker: &str,
    entries_match: bool,
    counts: &mut BTreeMap<&'static str, u64>,
) -> Result<Value, String> {
    let diffs = voucher_diffs(expected, matched, entries_match);
    let mut count = |key: &'static str| {
        counts.entry(key).and_modify(|count| *count += 1);
    };
    Ok(if matched.cancelled == Some(true) {
        // Tally drops a cancelled voucher's entries from this read
        // (measured for Journals only: protocol reference §9.14, PARTIAL,
        // for a gateway cancel; a screen cancel is captured in
        // fixtures/D3_CANCELLED_CAPTURE_PROVENANCE.md), so its entries
        // need not match: it is cancelled, not changed.
        // Only the header is compared, so a re-date before the cancel
        // still shows, if it stays inside the read window: a voucher
        // re-dated out of it is not read, so is not found unless another
        // voucher in the window has its content. The fingerprint branch
        // cannot see a cancelled row whose entries were dropped:
        // with no entries, no build's fingerprint matches.
        count("posted_not_effective");
        json!({"bridge_txn_id":expected.bridge_txn_id,"status":"posted_not_effective","marker":marker,"reason":"voucher_cancelled","diffs":voucher_diffs(expected, matched, true),"voucher_number":matched.voucher_number,"guid":matched.guid,"master_id":matched.master_id,"alter_id":matched.alter_id})
    } else if diffs.is_empty() && voucher_is_accounting_effective(matched)? {
        count("posted_verified");
        json!({"bridge_txn_id":expected.bridge_txn_id,"status":"posted_verified","marker":marker,"voucher_number":matched.voucher_number,"guid":matched.guid,"master_id":matched.master_id,"alter_id":matched.alter_id})
    } else if diffs.is_empty() {
        count("posted_not_effective");
        json!({"bridge_txn_id":expected.bridge_txn_id,"status":"posted_not_effective","marker":marker,"reason":"voucher_optional","voucher_number":matched.voucher_number,"guid":matched.guid,"master_id":matched.master_id,"alter_id":matched.alter_id})
    } else {
        count("posted_divergent");
        json!({"bridge_txn_id":expected.bridge_txn_id,"status":"posted_divergent","marker":marker,"diffs":diffs,"voucher_number":matched.voucher_number,"guid":matched.guid,"master_id":matched.master_id,"alter_id":matched.alter_id})
    })
}

/// Rewrites a verification of a post into a book that was rolled back after
/// it: a voucher the window does not hold is `book_rolled_back`, never
/// `not_found`, because it was posted and the book was then restored or
/// replaced. A rolled-back post is not evidence that its rows are absent.
pub(super) fn mark_book_rolled_back(result: &mut Value) {
    mark_not_found_as(result, "book_rolled_back");
    result["book_rolled_back"] = json!(true);
}

/// Rewrites a verification of a native post that carries no tag and was not
/// bound to its span: a voucher not found by its content is `sent_not_attributed`,
/// never `not_found`. It was sent, so an edit in Tally (which a tag used to
/// survive) is as likely as absence, and absence is never reported for it.
pub(super) fn mark_sent_not_attributed(result: &mut Value) {
    mark_not_found_as(result, "sent_not_attributed");
}

/// Why an untagged native post's vouchers that its content cannot find are not
/// found, as far as the post's own answer from Tally can say (bridge#1108).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UnmatchedCause {
    /// Tally's answer reported every counter, with `CREATED 0`, `EXCEPTIONS`
    /// equal to the vouchers sent and every other counter zero, and none of
    /// them is found. For a batch of two or more, the company's voucher mark
    /// was also read on both sides of the post and did not move. Only these
    /// captured shapes are read so (protocol reference §9.2; bridge#1108). A
    /// partly created batch is never read so: a count does not say which
    /// voucher Tally rejected, and twins in one batch defeat matching by
    /// content.
    ReportedNotCreated,
    /// Anything else, including no recorded answer: an edit in Tally is as
    /// likely as absence, so the voucher is `sent_not_attributed`.
    NotEstablished,
}

/// The cause for `unmatched` vouchers of a post of `sent`, from the post's own
/// answer. Every counter must have been present in the answer: an omitted
/// counter is not an observed zero (§9.2). `voucher_step` is how far the
/// company's voucher mark moved across the post, `None` unless it was read on
/// both sides; a batch of two or more needs it measured at 0, which catches a
/// Tally that created something while answering `CREATED 0`.
pub(super) fn unmatched_cause(
    counters: Option<&bridge_tally_protocol::TallyImportResult>,
    sent: usize,
    unmatched: u64,
    voucher_step: Option<u64>,
) -> UnmatchedCause {
    let Some(counters) = counters else {
        return UnmatchedCause::NotEstablished;
    };
    let Ok(sent) = u64::try_from(sent) else {
        return UnmatchedCause::NotEstablished;
    };
    let step_confirms = sent == 1 || voucher_step == Some(0);
    if sent >= 1
        && step_confirms
        && counters.counter_presence.all_reported()
        && counters.created == 0
        && counters.altered == 0
        && counters.deleted == 0
        && counters.ignored == 0
        && counters.errors == 0
        && counters.cancelled == 0
        && counters.exceptions == sent
        && unmatched == sent
    {
        UnmatchedCause::ReportedNotCreated
    } else {
        UnmatchedCause::NotEstablished
    }
}

/// How many of a verification's vouchers its content found nowhere. Read only
/// where nothing is bound, so no voucher is `bound_not_in_window`.
pub(super) fn unmatched_count(result: &Value) -> u64 {
    result["counts"]["not_found"].as_u64().unwrap_or(0)
}

/// Rewrites the post's own readback when Tally's answer to that post
/// reported its voucher as not created (`UnmatchedCause::ReportedNotCreated`).
pub(super) fn mark_reported_not_created(result: &mut Value) {
    mark_not_found_as(result, "tally_reported_not_created");
}

#[cfg(test)]
#[path = "agent_import_unmatched_cause_tests.rs"]
mod unmatched_cause_tests;

/// The one plain line a person reads for each status that is never absence.
pub(super) fn plain_next_step(status: &str) -> Option<&'static str> {
    match status {
        "bound_not_in_window" => Some("This voucher was posted, but it is not in the book for these dates now: it may have been deleted or re-dated in Tally, or the company restored from a backup. Check in Tally before posting it again."),
        "book_rolled_back" => Some("This voucher was posted, but the company's books are now older than that post: they were probably restored from a backup or replaced by another copy. Check in Tally before posting it again."),
        "sent_not_attributed" => Some("This voucher was sent to Tally, but ComplyEaze Bridge cannot match it in the book now, for example because it was edited in Tally. Check in Tally before posting it again."),
        "tally_reported_not_created" => Some("Tally reported this voucher as not created. Check that it is not in Tally, then enter this one voucher in Tally's voucher entry screen; do not import it again through Tally's Import menu. ComplyEaze Bridge will not send this saved voucher again."),
        _ => None,
    }
}

fn mark_not_found_as(result: &mut Value, status: &str) {
    let mut moved = 0_u64;
    if let Some(vouchers) = result["vouchers"].as_array_mut() {
        for voucher in vouchers {
            if voucher["status"] == "not_found" || voucher["status"] == "bound_not_in_window" {
                voucher["status"] = json!(status);
                voucher["next_step"] = json!(plain_next_step(status));
                moved += 1;
            }
        }
    }
    let counts = &mut result["counts"];
    let already = counts[status].as_u64().unwrap_or(0);
    counts["not_found"] = json!(0);
    counts["bound_not_in_window"] = json!(0);
    counts[status] = json!(already + moved);
}

pub(super) fn voucher_is_accounting_effective(voucher: &ReadVoucher) -> Result<bool, String> {
    match (voucher.cancelled, voucher.optional) {
        (Some(false), Some(false)) => Ok(true),
        (Some(true), _) | (_, Some(true)) => Ok(false),
        _ => Err("voucher_accounting_state_not_observed".to_string()),
    }
}

/// Every place a verification result names a ledger, as a path from the
/// result (`*` for each element of an array). The one list that marking reads,
/// and that its test holds against the names the verifier marks.
pub(super) const VERIFICATION_NAME_FIELDS: [&[&str]; 3] = [
    &[
        "vouchers", "*", "diffs", "*", "entries", "expected", "*", "ledger",
    ],
    &[
        "vouchers", "*", "diffs", "*", "entries", "observed", "*", "ledger",
    ],
    &["masters_after_post", "ledgers", "*"],
];

/// Marks every ledger name a verification result carries as a party name, so
/// the response's configured redaction applies to it. Names already marked are
/// left as they are; a proof saved with plain names is marked the same way.
/// A posted_under_changed_masters message is given its current text, which
/// names no ledger.
pub(super) fn mark_verification_names(result: &mut Value) {
    fn mark_at(value: &mut Value, path: &[&str]) {
        match path.split_first() {
            None => {
                if let Value::String(name) = value {
                    *value =
                        serde_json::to_value(party_name(std::mem::take(name))).unwrap_or_default();
                }
            }
            Some((&"*", rest)) => {
                if let Some(items) = value.as_array_mut() {
                    for item in items {
                        mark_at(item, rest);
                    }
                }
            }
            Some((key, rest)) => {
                if let Some(next) = value.get_mut(*key) {
                    mark_at(next, rest);
                }
            }
        }
    }
    for path in VERIFICATION_NAME_FIELDS {
        mark_at(result, path);
    }
    if result.pointer("/error/code") == Some(&json!("posted_under_changed_masters")) {
        if let Some(message) = result.pointer_mut("/error/message") {
            *message = json!(super::post::CHANGED_MASTERS_MESSAGE);
        }
    }
}

/// The `verify_import` response for one page of a verification (bridge#627).
///
/// Every row that is not `posted_verified`, the duplicate lists, the counts and
/// the status are always returned in full: they are what a caller must act on,
/// and the byte cap never cuts them. Only the `posted_verified` rows are paged,
/// as `items` from `offset`, which the response byte cap may shorten further
/// (it then sets `next_offset`). `proof` names the persisted proof a later
/// page is served from, so every page describes the same verification.
pub(super) fn verification_response_page(
    proof: &Value,
    proof_sha256: &str,
    offset: usize,
) -> Value {
    let (verified, unverified): (Vec<Value>, Vec<Value>) = proof["vouchers"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .partition(|row| row["status"] == "posted_verified");
    let mut page = proof.clone();
    if let Some(fields) = page.as_object_mut() {
        fields.remove("vouchers");
    }
    page["unverified_vouchers"] = json!(unverified);
    page["verified_total"] = json!(verified.len());
    page["offset"] = json!(offset);
    page["items"] = json!(verified.into_iter().skip(offset).collect::<Vec<_>>());
    page["proof"] = json!({"batch_id": proof["batch_id"], "sha256": proof_sha256});
    page
}

/// A verdict of `verify_import`: the only statuses a proof may carry. The
/// ledger line keeps its status as a string, which also holds non-verdicts
/// such as `built`; taking this type at the persist boundary keeps those out
/// of a proof (bridge#814).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VerificationStatus {
    PostedVerified,
    VerificationIncomplete,
}

impl VerificationStatus {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::PostedVerified => "posted_verified",
            Self::VerificationIncomplete => "verification_incomplete",
        }
    }
}

/// The verdict `verify_import` records and the proof renders. A dispatch that
/// needs reconciliation is never verified, whatever the readback shows; with no
/// dispatch record, the readback alone decides (bridge#804).
pub(super) fn final_verification_status(
    dispatch: Option<&Value>,
    result: &Value,
    expected_voucher_count: usize,
) -> VerificationStatus {
    if dispatch.is_some_and(|dispatch| dispatch["state"] == "reconciliation_required") {
        VerificationStatus::VerificationIncomplete
    } else {
        readback_verdict(result, expected_voucher_count)
    }
}

/// The readback's verdict, as the string tool results and the ledger carry.
pub(super) fn verification_status(result: &Value, expected_voucher_count: usize) -> &'static str {
    readback_verdict(result, expected_voucher_count).as_str()
}

fn readback_verdict(result: &Value, expected_voucher_count: usize) -> VerificationStatus {
    if result["counts"]["posted_verified"].as_u64() == Some(expected_voucher_count as u64)
        && result["duplicates"].as_array().is_some_and(Vec::is_empty)
    {
        VerificationStatus::PostedVerified
    } else {
        VerificationStatus::VerificationIncomplete
    }
}

pub(super) fn batch_duplicate_sets(
    observed: &[ReadVoucher],
    identities: &[String],
    fingerprints: &[VerificationFingerprint],
    expected_fingerprints: &BTreeMap<&VerificationFingerprint, usize>,
    tags: &[Option<&str>],
    expected_tags: &BTreeSet<&str>,
    fully_verified_identities: &BTreeSet<String>,
) -> (Vec<Value>, Vec<Value>) {
    // Serialize the structured vector before hashing: ledger names may contain
    // the delimiters used inside an entry, so joining entries is ambiguous.
    let duplicate_keys = fingerprints.iter().map(sha256_json).collect::<Vec<_>>();
    let all_duplicates = duplicates(observed, identities, &duplicate_keys)
        .into_iter()
        .filter(|duplicate| {
            duplicate["kind"] != "accounting_fingerprint"
                || !duplicate["voucher_ids"].as_array().is_some_and(|ids| {
                    ids.iter().all(|id| {
                        id.as_str()
                            .is_some_and(|id| fully_verified_identities.contains(id))
                    })
                })
        });
    let batch_indexes = (0..observed.len())
        .filter(|&index| {
            tags[index].is_some_and(|tag| expected_tags.contains(tag))
                || expected_fingerprints.contains_key(&fingerprints[index])
        })
        .collect::<Vec<_>>();
    let batch_remote_ids = batch_indexes
        .iter()
        .filter_map(|&index| observed[index].remote_id.as_deref())
        .collect::<BTreeSet<_>>();
    let batch_fingerprints = batch_indexes
        .iter()
        .map(|&index| duplicate_keys[index].as_str())
        .collect::<BTreeSet<_>>();
    let (batch, unrelated): (Vec<_>, Vec<_>) =
        all_duplicates.partition(|duplicate| match duplicate["kind"].as_str() {
            Some("remote_id") => duplicate["remote_id"]
                .as_str()
                .is_some_and(|id| batch_remote_ids.contains(id)),
            Some("accounting_fingerprint") => duplicate["fingerprint"]
                .as_str()
                .is_some_and(|key| batch_fingerprints.contains(key)),
            _ => false,
        });
    let safe_duplicate = |mut duplicate: Value| {
        if let Some(fields) = duplicate.as_object_mut() {
            if let Some(Value::String(fingerprint)) = fields.remove("fingerprint") {
                fields.insert("fingerprint_sha256".into(), json!(fingerprint));
            }
        }
        duplicate
    };
    (
        batch.into_iter().map(safe_duplicate).collect(),
        unrelated.into_iter().map(safe_duplicate).collect(),
    )
}

/// A bank voucher whose readback carried no `EFFECTIVEDATE`: its effective
/// date was written but could not be compared, which a clean status must not hide.
fn effective_date_not_observed(expected: &ImportVoucher, actual: &ReadVoucher) -> bool {
    expected.voucher_type != VoucherType::Journal && actual.effective_date.is_none()
}

pub(super) fn voucher_diffs(
    expected: &ImportVoucher,
    actual: &ReadVoucher,
    entries_match: bool,
) -> Vec<Value> {
    let mut diffs = Vec::new();
    let expected_date = normalized_date(&expected.date).ok();
    if actual.date.as_deref() != expected_date.as_deref() {
        diffs.push(json!("date"));
    }
    // A bank voucher is written with EFFECTIVEDATE equal to DATE (§9.13), and
    // the verification read returns it (§9.8 scoped correction). Only a value
    // that came back and differs is a diff: a response without the element
    // is reported as not observed, never refused.
    if expected.voucher_type != VoucherType::Journal
        && actual
            .effective_date
            .as_deref()
            .is_some_and(|observed| Some(observed) != expected_date.as_deref())
    {
        diffs.push(json!("effective_date"));
    }
    if actual.voucher_type.as_deref() != Some(expected.voucher_type.as_str()) {
        diffs.push(json!("voucher_type"));
    }
    if expected.voucher_number.is_some()
        && actual.voucher_number.as_deref() != expected.voucher_number.as_deref()
    {
        diffs.push(json!("voucher_number"));
    }
    if !entries_match {
        let expected_entries = expected
            .entries
            .iter()
            .map(|entry| {
                json!({"ledger":party_name(&entry.ledger), "amount":entry.amount, "side":entry.side,
                "is_deemed_positive":entry.side.tally_positive()})
            })
            .collect::<Vec<_>>();
        let actual_entries = actual
            .entries
            .iter()
            .map(|entry| {
                json!({"ledger":party_name(&entry.ledger), "amount":entry.amount,
                "is_deemed_positive":entry.is_deemed_positive})
            })
            .collect::<Vec<_>>();
        diffs.push(json!({"entries":{"expected":expected_entries,"observed":actual_entries}}));
    }
    diffs
}

pub(super) fn expected_entry_fingerprint(voucher: &ImportVoucher) -> Vec<String> {
    let mut result = voucher
        .entries
        .iter()
        .map(|entry| {
            format!(
                "{}|{}|{}",
                entry.ledger,
                match entry.side {
                    EntrySide::Dr => format!("-{}", entry.amount),
                    EntrySide::Cr => entry.amount.clone(),
                },
                entry.side.tally_positive()
            )
        })
        .collect::<Vec<_>>();
    result.sort();
    result
}
pub(super) fn actual_entry_fingerprint(voucher: &ReadVoucher) -> Vec<String> {
    let mut result = voucher
        .entries
        .iter()
        .map(|entry| {
            format!(
                "{}|{}|{}",
                entry.ledger, entry.amount, entry.is_deemed_positive
            )
        })
        .collect::<Vec<_>>();
    result.sort();
    result
}

pub(super) fn observed_voucher_identity(voucher: &ReadVoucher) -> Result<String, String> {
    voucher
        .guid
        .as_deref()
        .filter(|id| !id.trim().is_empty())
        .map(|id| format!("guid:{}", id.to_ascii_lowercase()))
        .or_else(|| {
            voucher
                .master_id
                .as_deref()
                .filter(|id| !id.trim().is_empty())
                .map(|id| format!("master_id:{id}"))
        })
        .ok_or_else(|| "import_verification_identity_invalid".to_string())
}

pub(super) fn duplicates(
    observed: &[ReadVoucher],
    identities: &[String],
    cached_fingerprints: &[String],
) -> Vec<Value> {
    let mut remote = BTreeMap::<String, BTreeSet<String>>::new();
    let mut fingerprints = BTreeMap::<String, BTreeMap<String, Option<String>>>::new();
    for ((voucher, identity), fingerprint) in
        observed.iter().zip(identities).zip(cached_fingerprints)
    {
        if let Some(id) = voucher
            .remote_id
            .as_ref()
            .filter(|id| !id.trim().is_empty())
        {
            remote
                .entry(id.clone())
                .or_default()
                .insert(identity.clone());
        }
        // A cancel whose entries this read dropped (measured for Journals only;
        // see `verify_batch`) fingerprints as its date and type alone, which
        // says nothing of what it recorded, so every such cancel of one date
        // and type would pair (bridge#767). It is left out; a cancelled row
        // that came back with its entries keeps its fingerprint.
        if !(voucher.cancelled == Some(true) && voucher.entries.is_empty()) {
            fingerprints
                .entry(fingerprint.clone())
                .or_default()
                .insert(identity.clone(), voucher.remote_id.clone());
        }
    }
    let mut result = remote.into_iter().filter(|(_, identities)| identities.len() > 1)
        .map(|(remote_id, identities)| json!({"kind":"remote_id","remote_id":remote_id,"count":identities.len()}))
        .collect::<Vec<_>>();
    result.extend(fingerprints.into_iter().filter(|(_, identities)| identities.len() > 1)
        .map(|(fingerprint, identities)| {
            let remote_ids = identities.values().flatten().collect::<BTreeSet<_>>();
            json!({"kind":"accounting_fingerprint","fingerprint":fingerprint,"voucher_ids":identities.keys().collect::<Vec<_>>(),"remote_ids":remote_ids})
        }));
    result
}

pub(super) fn company_high_water_mark(high_water: &Value) -> Result<PreImportMark, String> {
    let voucher = high_water["altvchid"]
        .as_u64()
        .ok_or_else(|| "pre_import_mark_unobserved".to_string())?;
    let master = high_water["altmstid"]
        .as_u64()
        .ok_or_else(|| "pre_import_mark_unobserved".to_string())?;
    Ok(PreImportMark {
        kind: "company_high_water".to_string(),
        value: Some(voucher),
        master_value: Some(master),
    })
}

pub(super) fn alter_id_delta(mark: &PreImportMark, observed: &[ReadVoucher]) -> Value {
    let latest = observed.iter().filter_map(|voucher| voucher.alter_id).max();
    match (mark.value, latest) {
        (Some(before), Some(after)) if after >= before => {
            json!({"before":before,"after_seen":after,"delta":after-before})
        }
        _ => json!({"before":mark.value,"after_seen":latest,"delta":"not_observed"}),
    }
}

/// `text` as a code span inside a Markdown table cell. A REMOTEID comes from
/// Tally and may hold `|`, a backtick or a line break: the pipe is escaped, so
/// the row keeps its cells, and the rest is [`markdown_code`]'s.
fn markdown_table_code(text: &str) -> String {
    markdown_code(&text.replace('|', "\\|"))
}

/// `text` as a Markdown code span. Text from Tally, such as a company name
/// (bridge#807), may hold a backtick or a line break: control characters
/// become spaces, so the line stays whole; the span's fence is one backtick
/// longer than any run inside; and a text that starts and ends with a space is
/// padded, since a code span drops one from each end. An empty text is padded
/// too: a bare pair of backticks is no span.
fn markdown_code(text: &str) -> String {
    let text = text
        .chars()
        .map(|character| {
            if character.is_control() {
                ' '
            } else {
                character
            }
        })
        .collect::<String>();
    let longest_run = text
        .split(|character| character != '`')
        .map(str::len)
        .max()
        .unwrap_or(0);
    let padded = text.is_empty()
        || longest_run > 0
        || (text.starts_with(' ')
            && text.ends_with(' ')
            && !text.chars().all(|character| character == ' '));
    let fence = "`".repeat(longest_run + 1);
    if padded {
        format!("{fence} {text} {fence}")
    } else {
        format!("{fence}{text}{fence}")
    }
}

pub(super) fn render_proof_markdown(proof: &Value) -> String {
    let mut output = format!(
        "# Voucher import verification — {}\n\n",
        proof["batch_id"].as_str().unwrap_or("unknown")
    );
    let dispatch_state = proof["dispatch"]["state"].as_str();
    // The banner follows the verdict itself, so a batch that is not verified
    // never reads clean; an absent status is not a verified one (bridge#804).
    let verification_status = proof["verification_status"].as_str();
    let not_verified = verification_status != Some("posted_verified");
    let dispatched = proof.get("dispatch").is_some();
    let banner = !proof["error"].is_null()
        || not_verified
        || (dispatched
            && !matches!(
                dispatch_state,
                Some("posted_verified" | "previous_attempt_reconciled")
            ));
    // Whether to forbid a resend depends only on whether Bridge sent the batch.
    // With no dispatch record it did not, and the batch may not be in Tally at
    // all (the readback before a post), so there is nothing to reconcile.
    if banner && dispatched {
        output.push_str("**Reconciliation required — this report does not confirm posting.**\n\nA matching voucher readback alone is insufficient. Reconcile the original saved batch; do not rebuild or resend it.\n\n");
    } else if banner {
        output.push_str("**Not verified — this report does not confirm posting.**\n\nThe verification status, the counts and any duplicates below say what the readback found.\n\n");
    }
    output.push_str(&format!(
        "- Verification status: `{}`\n",
        verification_status.unwrap_or("unknown")
    ));
    if let Some(state) = dispatch_state {
        output.push_str(&format!(
            "- Dispatch verdict: `{state}`\n- Response state: `{}`\n",
            proof["dispatch"]["response_state"]
                .as_str()
                .unwrap_or("unknown")
        ));
    }
    if let Some(code) = proof["error"]["code"].as_str() {
        output.push_str(&format!("- Error: `{code}`\n"));
    }
    output.push_str(&format!("\n- Company: {}\n- Batch SHA-256: `{}`\n- Readback checked: `{}`\n- Readback counts: matching {}, divergent {}, not effective {}, not found {}\n- AlterID delta: `{}`\n- Duplicates in this batch: {}\n- Unrelated duplicates in window: {}\n\n| Transaction | Readback status |\n| --- | --- |\n", markdown_code(proof["company"]["name"].as_str().unwrap_or("unknown")), proof["batch_sha256"].as_str().unwrap_or("unknown"), proof["verified_at"].as_str().unwrap_or("unknown"), proof["counts"]["posted_verified"], proof["counts"]["posted_divergent"], proof["counts"]["posted_not_effective"], proof["counts"]["not_found"], proof["alter_id_delta"], proof["duplicates"].as_array().map_or(0, Vec::len), proof["unrelated_duplicates_in_window"].as_array().map_or(0, Vec::len)));
    for row in proof["vouchers"].as_array().into_iter().flatten() {
        output.push_str(&format!(
            "| {} | {} |\n",
            row["bridge_txn_id"].as_str().unwrap_or("unknown"),
            row["status"].as_str().unwrap_or("unknown")
        ));
    }
    let batch_duplicates = proof["duplicates"].as_array().cloned().unwrap_or_default();
    if !batch_duplicates.is_empty() {
        output.push_str("\n| Duplicate in this batch | Key | Vouchers |\n| --- | --- | --- |\n");
        for duplicate in &batch_duplicates {
            let (key, vouchers) = if duplicate["kind"] == "remote_id" {
                (
                    &duplicate["remote_id"],
                    duplicate["count"].as_u64().unwrap_or(0),
                )
            } else {
                (
                    &duplicate["fingerprint_sha256"],
                    duplicate["voucher_ids"].as_array().map_or(0, Vec::len) as u64,
                )
            };
            output.push_str(&format!(
                "| {} | {} | {vouchers} |\n",
                duplicate["kind"].as_str().unwrap_or("unknown"),
                markdown_table_code(key.as_str().unwrap_or("unknown"))
            ));
        }
    }
    output.push_str(&format!(
        "\nEvidence hashes: company `{}`, voucher read `{}`.\n",
        proof["evidence"]["company"]["response_sha256"]
            .as_str()
            .unwrap_or("unknown"),
        proof["evidence"]["voucher_read_sha256"]
            .as_str()
            .unwrap_or("unknown")
    ));
    output
}
