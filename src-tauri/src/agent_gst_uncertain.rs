//! The uncertain vouchers of one voucher type (a Sales type is what was measured) in a date window, with the vouchers a
//! person accepted as they stand (Accept As Is), and the line a CA reads before filing.
//!
//! It is the status read of `gst_status` (its request, its parse and its refusals) with two
//! fields added: the party and the voucher's amount. Asking Tally for the amount makes it send
//! each voucher's ledger entries as well, about 7.5 KB a voucher on the lab's book, so the tool
//! first counts the window's vouchers with the status read and refuses a window above a bound
//! before it asks for them. The lab's TallyPrime 7.1 Silver book (13 Oct 2026, synthetic)
//! answered the request this tool sends, once before and once after one Accept As Is, and
//! nothing else changed: see the tests, which read both answers.
//!
//! A voucher's status is read from the second read's own flags. The count is a count, not a
//! source: nothing is classified from it.
use super::failure_code::GstUncertainCode;
use super::gst_status::{
    counts_json, gst_status_item_status, status_arguments, status_counts, GstStatus, Node,
    StatusRow,
};
use super::*;
use bridge_tally_core::ExactDecimal;

/// The most vouchers a window may hold for the wide read to be made. PROVISIONAL: at the lab's
/// largest voucher (about 9.7 KB of the 32 in the wide answer) it is about 9.7 MB, under a third of
/// the transport's cap; item invoices were not measured and are larger.
const UNCERTAIN_MAX_WINDOW_VOUCHERS: u64 = 1_000;

const AMOUNT_NOT_REPORTED: &str = "gst_uncertain_amount_not_reported";
const AMOUNT_UNREADABLE: &str = "gst_uncertain_amount_unreadable";
const PARTY_NOT_REPORTED: &str = "gst_uncertain_party_not_reported";
const PARTY_EMPTY: &str = "gst_uncertain_party_empty";
const PARTY_UNREADABLE: &str = "gst_uncertain_party_unreadable";
const NO_VOUCHERS_READ: &str = "gst_uncertain_no_vouchers_in_window";
const NONE_UNCERTAIN: &str = "gst_uncertain_none_marked_uncertain";

/// Which side of the party's entry an amount is on, from Tally's sign: a debit is written
/// negative. Only negative amounts were measured, on Sales invoices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum PartySide {
    Debit,
    Credit,
}

/// A voucher's total as Tally wrote it, without its sign, and the side the sign said.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct InvoiceAmount {
    magnitude: ExactDecimal,
    side: PartySide,
}

/// The party and the amount of one voucher, each read or the code of why it was not.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct WideFields {
    pub(super) party: Result<String, &'static str>,
    pub(super) amount: Result<InvoiceAmount, &'static str>,
}

/// The party ledger of a voucher: exactly one direct child `PARTYLEDGERNAME` holding a name.
/// An empty one (a cancelled voucher's) is not a name and is never read as one.
pub(super) fn party_of(found: Option<Option<String>>) -> Result<String, &'static str> {
    match found {
        None => Err(PARTY_UNREADABLE),
        Some(None) => Err(PARTY_NOT_REPORTED),
        Some(Some(name)) if name.is_empty() => Err(PARTY_EMPTY),
        Some(Some(name)) => Ok(name),
    }
}

/// The total of a voucher: exactly one direct child `AMOUNT` holding a plain decimal that is
/// not zero. An empty, repeated, zero or non-decimal one (a foreign-currency form) is unread,
/// never zero.
pub(super) fn amount_of(found: Option<Option<String>>) -> Result<InvoiceAmount, &'static str> {
    match found {
        None => Err(AMOUNT_UNREADABLE),
        Some(None) => Err(AMOUNT_NOT_REPORTED),
        Some(Some(text)) => {
            let value = ExactDecimal::parse(text).map_err(|_| AMOUNT_UNREADABLE)?;
            if value.is_zero() {
                return Err(AMOUNT_UNREADABLE);
            }
            Ok(InvoiceAmount {
                side: if value.is_negative() {
                    PartySide::Debit
                } else {
                    PartySide::Credit
                },
                magnitude: value.magnitude(),
            })
        }
    }
}

/// What the wide read adds to a voucher. Only the voucher's direct children are read: an
/// `AMOUNT` also sits under each of its ledger entries and is not the voucher's own.
fn wide_fields(voucher: &Node) -> WideFields {
    WideFields {
        party: party_of(voucher.one("PARTYLEDGERNAME")),
        amount: amount_of(voucher.one("AMOUNT")),
    }
}

/// Whether a voucher is one the tool lists: uncertain, or accepted as it stands.
fn listed(row: &StatusRow) -> bool {
    row.status == Ok(GstStatus::Uncertain) || row.overridden == Ok(true)
}

fn item(list: &'static str, row: &StatusRow, fields: &WideFields) -> Value {
    let party = match &fields.party {
        Ok(name) => json!(party_name(name.clone())),
        Err(code) => json!({"unread": code}),
    };
    let (amount, side) = match &fields.amount {
        Ok(amount) => (json!(amount.magnitude.as_str()), json!(amount.side)),
        Err(code) => (json!({"unread": code}), json!({"unread": code})),
    };
    json!({
        "list": list,
        "voucher_number": row.voucher_number,
        "date": row.date,
        "party": party,
        "amount": amount,
        "party_side": side,
        "alter_id": row.alter_id,
        "master_id": row.master_id,
        "gst_status": gst_status_item_status(row),
    })
}

/// A Tally date (YYYYMMDD) as YYYY-MM-DD.
fn dashed(date: &bridge_tally_core::TallyDate) -> String {
    let text = date.as_str();
    format!("{}-{}-{}", &text[..4], &text[4..6], &text[6..])
}

/// The pre-filing line, worded to say only what is read. The owner's line says Tally "leaves them out
/// of its GSTR-1 screen"; the lab's accept was made on that screen, which listed the uncertain voucher
/// under its own heading (5 before, 4 after) apart from the included ones, so the screen does not leave
/// it out; and no export or filed return was captured, so "out of the return" is not measured either.
/// The line therefore says the flags: N is the number of the type's vouchers in the window whose
/// uncertain flag reads Yes, not a reading of the screen (the one comparison, one month with every type
/// on the screen, was 5 against 5, and the tool accepts any window up to 93 days). The voucher type is named in place of "Sales" (the type's class is not
/// read, so the line cannot say it is a Sales type, and another type in the book is not counted in
/// it). The owner left the choice of words to us (#1148, 19 Oct 2026); these are the ones the capture
/// supports, and the decision is noted on #1692.
fn pre_filing_line(uncertain: u64, from: &str, to: &str, type_name: &str) -> String {
    let vouchers = if uncertain == 1 {
        "voucher"
    } else {
        "vouchers"
    };
    format!(
        "Tally marks {uncertain} {vouchers} of the type {type_name} dated {from} to {to} as \
         Uncertain on its GSTR-1 status flags. Amounts are from ComplyEaze Bridge's voucher \
         read; Tally's JSON export and the portal are not checked. Resolve each in Tally before \
         exporting; do not add them on the portal by hand."
    )
}

const FILED_PERIOD_NOTE: &str =
    "If this period is already filed, this list may differ from your filed return; check against it.";

impl Server {
    pub(super) async fn gst_uncertain_vouchers(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let arguments = status_arguments(args)?;
        let (from, to, type_name) = (&arguments.from, &arguments.to, &arguments.type_name);
        let gate = self.status_gate(arguments.guid).await?;

        // The first read is the status read of `gst_status`, which counts the window's
        // vouchers (about 3.4 KB a voucher on the lab's book) before the wide read is made.
        // Nothing is classified from it.
        let lean = gst_status_read(&gate.company.name, from, to, type_name).map_err(|code| {
            ToolFailure::from(code.to_string()).with_prior_evidence(gate.evidence.clone())
        })?;
        let (counted, evidence) = self
            .read_status_window(&gate, &arguments, lean, gate.evidence.clone(), |_| ())
            .await?;
        let counted = counted.len() as u64;
        if counted > UNCERTAIN_MAX_WINDOW_VOUCHERS {
            let mut failure =
                ToolFailure::from(GstUncertainCode::WindowTooLarge).with_prior_evidence(evidence);
            failure.counts = Some(Box::new(RowCounts {
                returned: 0,
                counted,
            }));
            return Err(failure);
        }

        let wide = gst_uncertain_read(&gate.company.name, from, to, type_name).map_err(|code| {
            ToolFailure::from(code.to_string()).with_prior_evidence(evidence.clone())
        })?;
        let (rows, mut evidence) = self
            .read_status_window(&gate, &arguments, wide, evidence, wide_fields)
            .await?;
        // A wide read that returns fewer vouchers than the count read found contradicts the
        // tool's own first read: it is refused with both numbers, never reported as a smaller
        // window (an empty one would read as "none found"). More than counted is not refused: a
        // voucher saved between the two reads is allowed for in the limitations.
        if (rows.len() as u64) < counted {
            let mut failure =
                ToolFailure::from(GstUncertainCode::WideReadShort).with_prior_evidence(evidence);
            failure.counts = Some(Box::new(RowCounts {
                returned: rows.len() as u64,
                counted,
            }));
            return Err(failure);
        }

        let (counts, flags_unread) = status_counts(rows.iter().map(|(row, _)| row));
        // A listed voucher whose party or amount was not read is unread as well.
        let fields_unread = rows
            .iter()
            .filter(|(row, fields)| {
                row.status.is_ok()
                    && row.overridden.is_ok()
                    && listed(row)
                    && (fields.party.is_err() || fields.amount.is_err())
            })
            .count() as u64;
        let unread = flags_unread + fields_unread;
        let uncertain = counts.get("uncertain").copied().unwrap_or(0);

        let mut items = Vec::new();
        for (row, fields) in &rows {
            if row.status == Ok(GstStatus::Uncertain) {
                items.push(item("uncertain", row, fields));
            }
            if row.overridden == Ok(true) {
                items.push(item("overridden", row, fields));
            }
        }
        if items.len() > self.settings.max_rows {
            let mut failure =
                ToolFailure::from(GstUncertainCode::RowsOverLimit).with_prior_evidence(evidence);
            failure.counts = Some(Box::new(RowCounts {
                returned: 0,
                counted: items.len() as u64,
            }));
            return Err(failure);
        }

        // A line stating "0 vouchers" reads as clearance, so it is stated only when at least one
        // voucher is marked Uncertain. A mistyped type name, or a type with none, reads no
        // voucher or none uncertain: the counts are complete and the line is withheld.
        let total = rows.len() as u64;
        let (state, line) = if unread > 0 {
            evidence.state = "partial";
            evidence.reason_code = Some("gst_uncertain_voucher_unread".to_string());
            (
                "partial",
                json!({"withheld": "gst_uncertain_voucher_unread"}),
            )
        } else if total == 0 {
            ("complete", json!({"withheld": NO_VOUCHERS_READ}))
        } else if uncertain == 0 {
            ("complete", json!({"withheld": NONE_UNCERTAIN}))
        } else {
            (
                "complete",
                json!(pre_filing_line(
                    uncertain,
                    &dashed(from),
                    &dashed(to),
                    type_name.as_str()
                )),
            )
        };
        let mut result = json!({
            "state": state,
            "basis": "tally_voucher_gst_status_party_amount_one_voucher_type_one_window",
            "window": {"from": from.as_str(), "to": to.as_str()},
            "voucher_type_name": type_name.as_str(),
            "total": total,
            "counts": counts_json(&counts, unread),
            "items": items,
            "pre_filing_line": line,
            "limitations": LIMITATIONS,
        });
        if unread == 0 {
            result["filed_period_note"] = json!(FILED_PERIOD_NOTE);
        }
        let company = gate.company;
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            }),
            evidence,
            company_guid: Some(arguments.guid.to_string()),
            truncated: false,
        })
    }
}

const LIMITATIONS: [&str; 14] = [
    "uncertain means Tally's uncertain flag reads Yes on the voucher once cancelled, optional and post-dated are decided first; those three are counted under their own keys and never as uncertain. The flags are the ones Tally holds when the vouchers are read: Tally keeps the status a voucher was given when it was saved",
    "The reason for uncertainty is not read: Tally shows it on its screen only. Bills missing an HSN/SAC are not split out from the others; they need re-saving with the code, not Accept As Is",
    "list=overridden holds every voucher whose acceptance flag (ISGSTOVERRIDDEN) reads Yes, with its gst_status as Tally reads it now. The flag is read as Accept As Is having been used; that nothing else sets it to Yes is not measured, and the before-capture already held five vouchers with the flag Yes: an Accept As Is on Tally's screen is recorded for two of them (BP/26-27/0005 and BP/26-27/0016) and the origin of the other three is not recorded. One that reads uncertain or excluded now, after it was accepted, is the row to look at. A voucher can appear under both lists",
    "The acceptance is confirmed only if an earlier run listed the voucher uncertain under the same master_id and this run lists it overridden, with gst_status included and the same amount. That shows only that the acceptance flag was set and the voucher reads included with the same amount, not that the voucher is correct or complete. This tool keeps no state: compare the runs by master_id, alter_id and amount. A voucher that is no longer listed has not had its acceptance confirmed",
    "party is the party ledger of the voucher: a cash sale shows the cash ledger, and no GSTIN is fetched. It was measured on an unregistered buyer only; a registered buyer is not measured",
    "amount is the invoice total on the party's entry, tax and round-off included, without its sign; party_side is the side Tally's sign gives it (a negative amount is a debit). It is not the taxable value of the GSTR-1 tables. Only negative amounts, on Sales invoices, were measured",
    "Amounts are not masked by the party masking setting: that setting masks party names only. This tool's answer carries both party names and amounts to the assistant; gst_status carries neither",
    "A voucher is unread when its flags, or its acceptance flag, were not read, or, if it is listed, when its party or its amount was not: absent, empty, repeated, zero, or not a plain decimal (a foreign-currency amount is not read). The state is then partial, the pre-filing line is withheld and the counts include the voucher under unread, so the number in the line is never short",
    "A cancelled voucher reads an empty party and amount on the lab's book and is not listed as uncertain; one whose acceptance flag reads Yes is listed under overridden, and its empty party and amount then make the result partial and withhold the line. An optional voucher reads its party and amount and is not counted as uncertain. A post-dated voucher is not counted as uncertain either: the amount of a post-dated voucher was not measured",
    "The window's vouchers are counted with the status read first and the amounts are read after it, so a voucher saved between the two reads can make the second larger than counted; a second read that returns fewer vouchers than the count read found is refused (gst_uncertain_wide_read_short, with both numbers in counts), never reported as a smaller window. A window the count finds above 1,000 vouchers is refused before the second read (gst_uncertain_window_too_large; provisional: sized on the lab's vouchers, and item invoices, which are larger, were not measured). Rows above this server's row limit are refused, with the counts (gst_uncertain_rows_over_limit). The answer is not paged, so one over this server's response byte cap is refused whole (agent_response_too_large, no counts; the cap is met by about half of BRIDGE_AGENT_MAX_BYTES because the result is sent twice): ask for a shorter window",
    "Measured on one synthetic book of TallyPrime 7.1 Silver, once before and once after one Accept As Is, which changed that voucher alone. Not measured: item invoices, a registered buyer, credit notes or other Sales-class types, foreign currency, the amount of a post-dated voucher, an Accept As Is of several vouchers at once, Gold and other releases",
    "excluded has not been observed on the lab's book. included counts a voucher a person accepted as it stands as well, so included is not 'clean'. A status is Tally's own flag and not a judgement of ComplyEaze Bridge",
    "A voucher whose ISDELETED or ISDELETEDVCHRETAINED flag reads Yes is unread (gst_status_voucher_deleted); a Yes was never measured",
    "At most 93 days a window and one voucher type named by its exact name (a type of the same class under another name is not read and not in the line, which names the type it covers and does not say it is a Sales type, because the type's class is not read). The name is not checked against the book's voucher types, so take it from masters. A total of 0 may mean the type name matched nothing: the line is withheld, with the counts kept, when no voucher was read and when none is uncertain, because a line of 0 would read as clearance. A book whose voucher mark is above 25,000 is refused (gst_status_book_too_large), because the selection is evaluated on every voucher of the book",
];

#[cfg(test)]
#[path = "agent_gst_uncertain_tests.rs"]
mod tests;
