//! Binding a natively posted voucher to the Tally object its own POST created,
//! by its position inside that POST's AlterID span, so a voucher posted with a
//! print-ready, untagged narration can still be attributed.
//!
//! One import of N vouchers moved the company's voucher mark by exactly N, and
//! the span (before, after] then held exactly those N vouchers, AlterIDs and
//! MasterIDs in request order for every voucher that can be told apart, the
//! last MasterID equal to the response's `LASTVCHID`
//! (`fixtures/POST_SPAN_CAPTURE_PROVENANCE.md`: two raw runs on licensed Silver
//! 7.1, 10 Payment, Receipt and Contra vouchers and 5 Journals). Vouchers
//! identical in content cannot be told apart in a read, so they are bound by
//! position alone.
//!
//! The identity is the span's exclusivity plus the count; content only refuses.
//! Nothing here reads Tally: every input is already parsed, and every type that
//! states a fact can only be built by the function that checks it.

use super::*;

/// The target company's voucher mark immediately before the POST, as
/// journaled with the dispatch intent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PreMark(u64);

impl PreMark {
    /// The mark recorded with the dispatch intent. Its only source is that
    /// record, which is written before the POST.
    pub(super) fn recorded(mark: u64) -> Self {
        Self(mark)
    }

    pub(super) fn value(self) -> u64 {
        self.0
    }
}

/// A POST that created exactly `count` vouchers, during which the target's
/// voucher mark moved by exactly that many.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PostSpan {
    before: u64,
    through: u64,
    last_vch_id: u64,
    count: usize,
}

/// A voucher bound to the Tally object its own POST created.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PostedVoucherIdentity {
    pub(super) bridge_txn_id: String,
    pub(super) guid: String,
    pub(super) master_id: u64,
}

/// Why a binding was refused on what was read. Final for the automatic path:
/// only the person's review settles it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum SpanRefusal {
    EmptyBatch,
    CountersNotClean,
    CreatedNotCount {
        created: u64,
        count: usize,
    },
    LastVchIdAbsent,
    MarkWentBack,
    StepNotCreated {
        step: u64,
        created: u64,
    },
    SpanCount {
        expected: usize,
        observed: usize,
    },
    Position {
        position: usize,
    },
    Content {
        position: usize,
        fields: Vec<&'static str>,
    },
    NotEffective {
        position: usize,
    },
    MasterIdNotInSequence {
        position: usize,
    },
    GuidNotMasterId {
        position: usize,
    },
    /// Another batch already bound this GUID: a rolled-back or restored book
    /// can hand a voucher's GUID to a different voucher.
    IdentityReused {
        position: usize,
    },
}

impl SpanRefusal {
    pub(super) fn code(&self) -> &'static str {
        match self {
            Self::EmptyBatch => "span_empty_batch",
            Self::CountersNotClean => "span_counters_not_clean",
            Self::CreatedNotCount { .. } => "span_created_not_count",
            Self::LastVchIdAbsent => "span_lastvchid_absent",
            Self::MarkWentBack => "span_mark_went_back",
            Self::StepNotCreated { .. } => "span_step_not_created",
            Self::SpanCount { .. } => "span_count_mismatch",
            Self::Position { .. } => "span_position_mismatch",
            Self::Content { .. } => "span_content_mismatch",
            Self::NotEffective { .. } => "span_voucher_not_effective",
            Self::MasterIdNotInSequence { .. } => "span_master_id_not_in_sequence",
            Self::GuidNotMasterId { .. } => "span_guid_not_master_id",
            Self::IdentityReused { .. } => "span_identity_reused",
        }
    }
}

/// Why a binding could not be decided: nothing was refused, and the bind may
/// be attempted again. Kept apart from [`SpanRefusal`] so that what could not
/// be read, or was not observed, is never recorded as a refusal of what Tally
/// holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BindUnsettled {
    /// A Payment, Receipt or Contra row was read without `EFFECTIVEDATE`.
    /// Verification treats that absence as not observed, never as a
    /// difference, so it is not a reason to refuse the binding for good: the
    /// next verification decides again. Every other check still ran, and none
    /// refused.
    EffectiveDateNotObserved,
    /// The journal could not be read to check that no other batch bound the
    /// same GUIDs.
    JournalUnreadable,
}

impl BindUnsettled {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::EffectiveDateNotObserved => "binding_effective_date_not_observed",
            Self::JournalUnreadable => "binding_journal_unreadable",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum BindError {
    Unsettled(BindUnsettled),
    Refused(SpanRefusal),
}

impl From<SpanRefusal> for BindError {
    fn from(refusal: SpanRefusal) -> Self {
        Self::Refused(refusal)
    }
}

/// The counters of a clean create of `count` vouchers, and its `LASTVCHID`.
fn clean_create(
    outcome: &bridge_tally_protocol::TallyImportOutcome,
    count: usize,
) -> Result<u64, SpanRefusal> {
    if count == 0 {
        return Err(SpanRefusal::EmptyBatch);
    }
    let counters = outcome.counters();
    if !counters.counter_presence.all_reported()
        || counters.altered != 0
        || counters.deleted != 0
        || counters.ignored != 0
        || counters.errors != 0
        || counters.cancelled != 0
        || counters.exceptions != 0
        || counters.line_error_count != 0
    {
        return Err(SpanRefusal::CountersNotClean);
    }
    if usize::try_from(counters.created).ok() != Some(count) {
        return Err(SpanRefusal::CreatedNotCount {
            created: counters.created,
            count,
        });
    }
    outcome.last_vch_id().ok_or(SpanRefusal::LastVchIdAbsent)
}

impl PostSpan {
    /// The span of a clean POST of `count` vouchers, for a bind made later
    /// (owner decision, 2026-10-02): `(before, before + count]`. It is used
    /// when the mark after the POST was not read (a transport failure, a
    /// timeout or a crash), and by every later verification, including one
    /// after a bind left unsettled (as by a read without `EFFECTIVEDATE`). No
    /// step is checked here, because none was journaled; a later voucher change
    /// always takes an AlterID above the mark, so the slots can only empty, and
    /// `bind`'s count, positions and `LASTVCHID` decide. The caller must first have read
    /// the company's mark at or above `before + count`; below it, the book was
    /// rolled back and nothing may bind.
    pub(super) fn after_clean_response(
        before: PreMark,
        outcome: &bridge_tally_protocol::TallyImportOutcome,
        count: usize,
    ) -> Result<Self, SpanRefusal> {
        let last_vch_id = clean_create(outcome, count)?;
        Ok(Self {
            before: before.0,
            through: before
                .0
                .checked_add(count as u64)
                .ok_or(SpanRefusal::MarkWentBack)?,
            last_vch_id,
            count,
        })
    }

    /// The span of a clean POST of `count` vouchers. Refused unless every one
    /// of the seven counters was reported, `CREATED == count`, every other
    /// counter is zero with no `LINEERROR`, `LASTVCHID` was reported, and the
    /// target's voucher mark moved by exactly `CREATED`.
    pub(super) fn after_clean_post(
        before: PreMark,
        after_mark: u64,
        outcome: &bridge_tally_protocol::TallyImportOutcome,
        count: usize,
    ) -> Result<Self, BindError> {
        let last_vch_id = clean_create(outcome, count)?;
        let created = count as u64;
        let step = after_mark
            .checked_sub(before.0)
            .ok_or(SpanRefusal::MarkWentBack)?;
        if step != created {
            return Err(SpanRefusal::StepNotCreated { step, created }.into());
        }
        Ok(Self {
            before: before.0,
            through: after_mark,
            last_vch_id,
            count,
        })
    }

    /// The read narrowing that selects exactly this span.
    pub(super) fn alter_id_span(&self) -> super::super::AlterIdSpan {
        super::super::AlterIdSpan {
            after: self.before,
            through: self.through,
        }
    }
}

/// Binds each voucher `sent` (in request order) to the row its POST created.
///
/// `read` is the rows of the batch's corroborated verification window that lie
/// inside [`PostSpan::alter_id_span`]. They must hold exactly the batch's count,
/// and row k (by AlterID) must sit at `before + k`, with the content sent as
/// voucher k: type, date, `EFFECTIVEDATE` where written, the signed entries and
/// the narration byte for byte. Every row must be effective. MasterIDs must run
/// `LASTVCHID - N + 1 ..= LASTVCHID` in request order, and each GUID must be the
/// company GUID followed by its MasterID in eight hex digits, as observed: a
/// refuse-only cross-check. No GUID may be one `bound_elsewhere` (another
/// batch's binding).
pub(super) fn bind(
    span: &PostSpan,
    company_guid: &str,
    sent: &[ImportVoucher],
    read: &ImportReadSource,
    bound_elsewhere: &BTreeSet<String>,
) -> Result<Vec<PostedVoucherIdentity>, BindError> {
    if sent.len() != span.count || read.rows.len() != span.count {
        return Err(SpanRefusal::SpanCount {
            expected: span.count,
            observed: read.rows.len(),
        }
        .into());
    }
    let mut rows = read.rows.iter().collect::<Vec<_>>();
    rows.sort_by_key(|row| row.alter_id);
    let company_guid = company_guid.trim().to_ascii_lowercase();
    let first_master_id = span
        .last_vch_id
        .checked_add(1)
        .and_then(|next| next.checked_sub(span.count as u64))
        .ok_or(SpanRefusal::MasterIdNotInSequence { position: 0 })?;
    let mut identities = Vec::with_capacity(span.count);
    let mut effective_date_unobserved = false;
    for (position, (row, voucher)) in rows.iter().zip(sent).enumerate() {
        if row.alter_id != span.before.checked_add(1 + position as u64) {
            return Err(SpanRefusal::Position { position }.into());
        }
        if voucher.voucher_type.bank_shape().is_some() && row.effective_date.is_none() {
            effective_date_unobserved = true;
        }
        let fields = content_differences(voucher, row);
        if !fields.is_empty() {
            return Err(SpanRefusal::Content { position, fields }.into());
        }
        if !matches!(voucher_is_accounting_effective(row), Ok(true)) {
            return Err(SpanRefusal::NotEffective { position }.into());
        }
        let master_id = row
            .master_id
            .as_deref()
            .and_then(|id| id.parse::<u64>().ok())
            .filter(|id| Some(*id) == first_master_id.checked_add(position as u64))
            .ok_or(SpanRefusal::MasterIdNotInSequence { position })?;
        let guid = row
            .guid
            .clone()
            .filter(|guid| *guid == format!("{company_guid}-{master_id:08x}"))
            .ok_or(SpanRefusal::GuidNotMasterId { position })?;
        if bound_elsewhere.contains(&guid) {
            return Err(SpanRefusal::IdentityReused { position }.into());
        }
        identities.push(PostedVoucherIdentity {
            bridge_txn_id: voucher.bridge_txn_id.clone(),
            guid,
            master_id,
        });
    }
    if effective_date_unobserved {
        return Err(BindError::Unsettled(
            BindUnsettled::EffectiveDateNotObserved,
        ));
    }
    Ok(identities)
}

/// The fields in which `row` differs from what was sent as `voucher`. An
/// amount that does not parse is a difference, never a match.
fn content_differences(voucher: &ImportVoucher, row: &ReadVoucher) -> Vec<&'static str> {
    let mut fields = Vec::new();
    let date = normalized_date(&voucher.date).ok();
    if date.is_none() || row.date != date {
        fields.push("date");
    }
    // The bank shapes write EFFECTIVEDATE equal to DATE; a Journal writes none.
    // An absent one is not observed, never a difference (`bind` leaves it
    // unsettled), as in verification.
    if voucher.voucher_type.bank_shape().is_some()
        && row.effective_date.is_some()
        && row.effective_date != date
    {
        fields.push("effective_date");
    }
    if row.voucher_type.as_deref() != Some(voucher.voucher_type.as_str()) {
        fields.push("voucher_type");
    }
    let sent = voucher
        .entries
        .iter()
        .map(|entry| {
            let signed = match entry.side {
                EntrySide::Dr => format!("-{}", entry.amount),
                EntrySide::Cr => entry.amount.clone(),
            };
            canonical_verification_amount(&signed).map(|amount| {
                (
                    entry.ledger.clone(),
                    amount,
                    entry.side.tally_positive().to_string(),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>();
    let read = row
        .entries
        .iter()
        .map(|entry| {
            canonical_verification_amount(&entry.amount).map(|amount| {
                (
                    entry.ledger.clone(),
                    amount,
                    entry.is_deemed_positive.clone(),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>();
    // A multiset: a voucher may carry the same ledger and amount twice, and a
    // dropped repeat must still differ.
    match (sent, read) {
        (Ok(mut sent), Ok(mut read)) => {
            sent.sort();
            read.sort();
            if sent != read {
                fields.push("entries");
            }
        }
        _ => fields.push("entries"),
    }
    let sent_narration = voucher.narration.as_deref().unwrap_or("").trim();
    if row.narration.as_deref().unwrap_or("") != sent_narration {
        fields.push("narration");
    }
    fields
}

#[cfg(test)]
#[path = "agent_import_span_identity_tests.rs"]
mod tests;
