//! `statement_tie_out`: does the bank ledger in Tally stand where the bank
//! statement says it stood at the start and at the end of the statement's own
//! dates?
//!
//! Read-only. It writes nothing to Tally, never posts and never blocks a post.
//! It reads the proposals file `parse_bank_statement` wrote and the book's
//! ledger catalogue at three dates, and returns three gaps and the dates.
//!
//! **Whole statements only.** A statement narrowed with from/to would need the
//! running balance at its edges, which the result of `parse_bank_statement`
//! deliberately never carries. On a whole statement the two edge figures are
//! the opening and closing balances the caller supplied, so a gap reveals
//! nothing the caller did not already hold. A narrowed file is
//! `not_established`.
//!
//! **No figure beyond the gaps and the dates is returned**: not the book's
//! opening or closing, not the statement's.
//!
//! **The sign is a type.** Tally holds a debit (asset) balance as a negative
//! number; a statement prints money in the account as positive. [`BankSide`]
//! is the statement's sign, and [`BankSide::from_book`] is the one place a
//! Tally figure is converted into it.
//!
//! Reading text is fixed, built in one place ([`reading`]) and pinned by a test
//! that compares the exact words. It names every gap that is not zero and every
//! figure that is not established, and prescribes no remedy.
use super::*;
use bridge_bank_statement::date::Date;
use bridge_tally_core::{ExactDecimal, TallyDate};

/// Whether this file's own vouchers are counted into the closing figure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    /// Before the file is built and imported: the closing figure is the book
    /// plus what this file's vouchers would add to the bank ledger.
    BeforeBuild,
    /// After an import, by anyone: the closing figure is the book as it stands.
    AfterPost,
}

impl Stage {
    fn parse(text: &str) -> Result<Self, String> {
        match text {
            "before_build" => Ok(Self::BeforeBuild),
            "after_post" => Ok(Self::AfterPost),
            _ => Err("argument_invalid:stage".to_string()),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::BeforeBuild => "before_build",
            Self::AfterPost => "after_post",
        }
    }
}

/// Why a figure is not established. Each is a typed result, not an error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NotEstablished {
    WindowNotRecorded,
    WindowNarrowerThanStatement,
    WindowPrecedesBooksFrom,
    BankLedgerNotInBook,
    BookChangedDuringRead,
    OpeningBalanceNotObserved,
}

impl NotEstablished {
    fn code(self) -> &'static str {
        match self {
            Self::WindowNotRecorded => "window_not_recorded",
            Self::WindowNarrowerThanStatement => "window_narrower_than_statement",
            Self::WindowPrecedesBooksFrom => "window_precedes_books_from",
            Self::BankLedgerNotInBook => "bank_ledger_not_in_book",
            Self::BookChangedDuringRead => "book_changed_during_read",
            Self::OpeningBalanceNotObserved => "opening_balance_not_observed",
        }
    }

    fn explanation(self) -> &'static str {
        match self {
            Self::WindowNotRecorded => "this proposals file was written before it recorded the statement's dates; parse the statement again",
            Self::WindowNarrowerThanStatement => "this file was parsed with a from or to date that leaves out part of the statement, and this check ties whole statements only; parse the statement again without them",
            Self::WindowPrecedesBooksFrom => "the statement starts before the company's books begin",
            Self::BankLedgerNotInBook => "the file's bank ledger is not in the book under that exact name",
            Self::BookChangedDuringRead => "the bank ledger changed while it was being read; read again",
            Self::OpeningBalanceNotObserved => "Tally returned no opening balance for the bank ledger at one of the dates",
        }
    }
}

/// An amount in the statement's sign: positive is money in the bank account.
#[derive(Clone, Debug, PartialEq, Eq)]
struct BankSide(ExactDecimal);

impl BankSide {
    /// A figure as the statement prints it, or an entry of the file read the
    /// same way (a debit to the bank ledger is money in).
    fn from_printed(text: &str) -> Result<Self, String> {
        ExactDecimal::parse(text)
            .map(Self)
            .map_err(|_| "proposals_file_invalid".to_string())
    }

    /// The one conversion from Tally's sign, where a debit balance is negative.
    fn from_book(native: &str) -> Result<Self, String> {
        let native =
            ExactDecimal::parse(native).map_err(|_| "ledger_opening_invalid".to_string())?;
        ExactDecimal::zero()
            .checked_subtract(&native)
            .map(Self)
            .map_err(|_| "ledger_opening_invalid".to_string())
    }

    fn plus(&self, other: &Self) -> Result<Self, String> {
        self.0
            .checked_add(&other.0)
            .map(Self)
            .map_err(|_| "voucher_amount_invalid".to_string())
    }

    fn minus(&self, other: &Self) -> Result<Self, String> {
        self.0
            .checked_subtract(&other.0)
            .map(Self)
            .map_err(|_| "voucher_amount_invalid".to_string())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Exact, at least two decimal places: no digit is rounded away, and a
    /// zero never carries a sign.
    fn text(&self) -> String {
        if self.is_zero() {
            return "0.00".to_string();
        }
        let raw = self.0.as_str();
        let (whole, fraction) = raw.split_once('.').unwrap_or((raw, ""));
        let mut fraction = fraction.trim_end_matches('0').to_string();
        while fraction.len() < 2 {
            fraction.push('0');
        }
        format!("{whole}.{fraction}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Figure {
    Gap(BankSide),
    NotEstablished(NotEstablished),
}

impl Figure {
    fn state(&self) -> &'static str {
        match self {
            Self::Gap(gap) if gap.is_zero() => "tied",
            Self::Gap(_) => "differs",
            Self::NotEstablished(_) => "not_established",
        }
    }

    fn amount(&self) -> Option<String> {
        match self {
            Self::Gap(gap) => Some(gap.text()),
            Self::NotEstablished(_) => None,
        }
    }

    fn reason(&self) -> Option<NotEstablished> {
        match self {
            Self::Gap(_) => None,
            Self::NotEstablished(reason) => Some(*reason),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Gaps {
    opening: Figure,
    closing: Figure,
    change: Figure,
}

/// What the file says about the statement, read once.
#[derive(Debug)]
struct StatementFile {
    bank_ledger: String,
    opening: BankSide,
    closing: BankSide,
    /// The statement's own first and last row dates, when the file recorded
    /// that its window was the whole statement.
    window: Result<(Date, Date), NotEstablished>,
    /// What this file's vouchers add to the bank ledger.
    proposals_net: BankSide,
}

impl StatementFile {
    /// Every field is parsed here, once, and a file that cannot be read is
    /// `proposals_file_invalid`: an empty or unreadable figure is not zero.
    fn read(document: &Value) -> Result<Self, String> {
        let invalid = || "proposals_file_invalid".to_string();
        let text = |value: &Value| value.as_str().map(str::to_string).ok_or_else(invalid);
        let bank_ledger = text(&document["bank_ledger"])?;
        let opening = BankSide::from_printed(&text(&document["controls"]["opening_balance"])?)?;
        let closing = BankSide::from_printed(&text(&document["controls"]["closing_balance"])?)?;
        let window = recorded_window(document)?;
        let proposals_net = proposals_net(document, &bank_ledger)?;
        Ok(Self {
            bank_ledger,
            opening,
            closing,
            window,
            proposals_net,
        })
    }
}

/// The window the parse recorded, when it recorded the whole statement. A file
/// that records none, or a narrower one, is a typed result; one whose window is
/// malformed is refused.
fn recorded_window(document: &Value) -> Result<Result<(Date, Date), NotEstablished>, String> {
    let invalid = || "proposals_file_invalid".to_string();
    let Some(window) = document.get("window") else {
        return Ok(Err(NotEstablished::WindowNotRecorded));
    };
    let window = window.as_object().ok_or_else(invalid)?;
    let day = |key: &str| {
        window
            .get(key)
            .and_then(Value::as_str)
            .and_then(Date::parse_iso)
            .ok_or_else(invalid)
    };
    let (first, last) = (day("first_row_date")?, day("last_row_date")?);
    let whole = window
        .get("whole_statement")
        .and_then(Value::as_bool)
        .ok_or_else(invalid)?;
    if first > last {
        return Err(invalid());
    }
    Ok(if whole {
        Ok((first, last))
    } else {
        Err(NotEstablished::WindowNarrowerThanStatement)
    })
}

/// What the file's vouchers add to the bank ledger: a debit to it is money in,
/// a credit is money out. A leg on any other ledger, or on a name that only
/// resembles the bank ledger, counts for nothing.
fn proposals_net(document: &Value, bank_ledger: &str) -> Result<BankSide, String> {
    let invalid = || "proposals_file_invalid".to_string();
    let mut net = BankSide::from_printed("0")?;
    for voucher in document["vouchers"].as_array().ok_or_else(invalid)? {
        for entry in voucher["entries"].as_array().ok_or_else(invalid)? {
            if entry["ledger"].as_str().ok_or_else(invalid)? != bank_ledger {
                continue;
            }
            let amount = BankSide::from_printed(entry["amount"].as_str().ok_or_else(invalid)?)?;
            if amount.0.is_negative() {
                return Err(invalid());
            }
            net = match entry["side"].as_str() {
                Some("Dr") => net.plus(&amount)?,
                Some("Cr") => net.minus(&amount)?,
                _ => return Err(invalid()),
            };
        }
    }
    Ok(net)
}

/// The bank ledger as one catalogue read showed it.
#[derive(Clone, Debug, PartialEq, Eq)]
enum LedgerRead {
    Absent,
    Present { opening: Option<String> },
}

impl LedgerRead {
    /// By exact name. A name that appears twice in one catalogue is not a
    /// book this check can read.
    fn of(ledgers: &[TallyLedger], name: &str) -> Result<Self, String> {
        let mut found = ledgers.iter().filter(|ledger| ledger.name == name);
        match (found.next(), found.next()) {
            (None, _) => Ok(Self::Absent),
            (Some(ledger), None) => Ok(Self::Present {
                opening: ledger.opening_balance.clone(),
            }),
            _ => Err("ledger_snapshot_drifted".to_string()),
        }
    }
}

struct Observed {
    at_first: LedgerRead,
    after_last: LedgerRead,
    at_first_again: LedgerRead,
}

/// The catalogue dates a tie-out reads: the first row's date, the day after
/// the last (the book's position at the end of the last day), and the first
/// again to catch a change made during the read.
fn read_dates(first: Date, last: Date) -> Result<[TallyDate; 3], String> {
    let tally = |date: Date| {
        TallyDate::parse(date.iso().replace('-', "")).map_err(|_| "invalid_date".to_string())
    };
    let first = tally(first)?;
    let after_last = tally(last)?
        .next_day()
        .map_err(|_| "invalid_date".to_string())?;
    Ok([first.clone(), after_last, first])
}

/// The three gaps from the three reads, or the reason none can be given.
fn gaps(stage: Stage, file: &StatementFile, observed: &Observed) -> Result<Gaps, String> {
    let (start, end, start_again) = match (
        &observed.at_first,
        &observed.after_last,
        &observed.at_first_again,
    ) {
        (LedgerRead::Absent, LedgerRead::Absent, LedgerRead::Absent) => {
            return Ok(all_not_established(NotEstablished::BankLedgerNotInBook))
        }
        (
            LedgerRead::Present { opening: start },
            LedgerRead::Present { opening: end },
            LedgerRead::Present {
                opening: start_again,
            },
        ) => (start, end, start_again),
        // Present in one read and absent in another.
        _ => return Ok(all_not_established(NotEstablished::BookChangedDuringRead)),
    };
    if !same_opening(start.as_deref(), start_again.as_deref())? {
        return Ok(all_not_established(NotEstablished::BookChangedDuringRead));
    }
    let not_observed = Figure::NotEstablished(NotEstablished::OpeningBalanceNotObserved);
    let opening = match start {
        None => not_observed.clone(),
        Some(native) => Figure::Gap(BankSide::from_book(native)?.minus(&file.opening)?),
    };
    let closing = match end {
        None => not_observed.clone(),
        Some(native) => {
            let book = BankSide::from_book(native)?;
            let book = match stage {
                Stage::AfterPost => book,
                Stage::BeforeBuild => book.plus(&file.proposals_net)?,
            };
            Figure::Gap(book.minus(&file.closing)?)
        }
    };
    let change = match (&opening, &closing) {
        (Figure::Gap(opening), Figure::Gap(closing)) => Figure::Gap(closing.minus(opening)?),
        _ => not_observed,
    };
    Ok(Gaps {
        opening,
        closing,
        change,
    })
}

fn all_not_established(reason: NotEstablished) -> Gaps {
    Gaps {
        opening: Figure::NotEstablished(reason),
        closing: Figure::NotEstablished(reason),
        change: Figure::NotEstablished(reason),
    }
}

/// Whether two reads of one opening agree, by value: `-1500` and `-1500.00`
/// are one figure. One read with a figure and one without do not agree.
fn same_opening(left: Option<&str>, right: Option<&str>) -> Result<bool, String> {
    let parse =
        |text: &str| ExactDecimal::parse(text).map_err(|_| "ledger_opening_invalid".to_string());
    match (left, right) {
        (None, None) => Ok(true),
        (Some(left), Some(right)) => Ok(parse(left)?.numeric_eq(&parse(right)?)),
        _ => Ok(false),
    }
}

const SIGN_SENTENCE: &str = "A positive amount means the book shows more money in the bank than the statement does; a negative amount, less.";
const POSSIBLE_CAUSES: &str = "Uncleared cheques and deposits in transit explain differences like these. So can a missing entry, a repeated entry or the wrong bank ledger. This check cannot tell them apart. To see which vouchers are involved, read the bank ledger's vouchers for these dates.";
const SCOPE: &str =
    "This checks the bank ledger only. A wrong party or expense ledger is not caught here.";

/// The fixed reading text. The headline comes first and names every gap that is
/// not zero and every figure that is not established; nothing is prescribed.
fn reading(stage: Stage, window: Option<(Date, Date)>, gaps: &Gaps) -> Value {
    let figures = [
        ("opening gap", &gaps.opening),
        ("closing gap", &gaps.closing),
        ("change in window", &gaps.change),
    ];
    let subject = match window {
        Some((first, last)) => format!(
            "Bank ledger against the statement, {} to {}",
            first.iso(),
            last.iso()
        ),
        None => "Bank ledger against the statement".to_string(),
    };
    let mut parts = Vec::new();
    let mut differs = false;
    for (label, figure) in figures {
        if let Figure::Gap(gap) = figure {
            if !gap.is_zero() {
                differs = true;
                parts.push(format!("{label} {}", gap.text()));
            }
        }
    }
    // One clause per reason, in the order the figures first show it.
    let mut unestablished: Vec<(NotEstablished, Vec<&str>)> = Vec::new();
    for (label, figure) in figures {
        if let Some(reason) = figure.reason() {
            match unestablished.iter_mut().find(|(known, _)| *known == reason) {
                Some((_, labels)) => labels.push(label),
                None => unestablished.push((reason, vec![label])),
            }
        }
    }
    for (reason, labels) in &unestablished {
        parts.push(format!(
            "{} not established ({})",
            join_and(labels),
            reason.explanation()
        ));
    }
    let mut headline = if parts.is_empty() {
        format!("{subject}: tied at the start and at the end.")
    } else {
        format!("{subject}: {}.", parts.join("; "))
    };
    if differs {
        headline.push(' ');
        headline.push_str(SIGN_SENTENCE);
    }
    let mut text = json!({
        "headline": headline,
        "stage": match stage {
            Stage::AfterPost => "The closing figure is the book as it stands now.",
            Stage::BeforeBuild => "The closing figure counts this file's vouchers as if they were already in the book. Once any of them has been imported, use stage after_post.",
        },
        "scope": SCOPE,
    });
    if differs {
        text["possible_causes"] = json!(POSSIBLE_CAUSES);
    }
    text
}

/// "a", "a and b", "a, b and c".
fn join_and(labels: &[&str]) -> String {
    match labels {
        [] => String::new(),
        [only] => (*only).to_string(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

fn result_json(stage: Stage, window: Option<(Date, Date)>, gaps: &Gaps) -> Value {
    let figure = |figure: &Figure| {
        json!({
            "state": figure.state(),
            "amount": figure.amount(),
            "reason": figure.reason().map(NotEstablished::code),
        })
    };
    json!({
        "stage": stage.as_str(),
        "window": window.map(|(first, last)| json!({
            "first_row_date": first.iso(),
            "last_row_date": last.iso(),
        })),
        "gap_basis": "book_minus_statement_in_the_statements_sign",
        "opening_gap": figure(&gaps.opening),
        "closing_gap": figure(&gaps.closing),
        "change_in_window": figure(&gaps.change),
        "reading": reading(stage, window, gaps),
    })
}

/// The evidence of a result: complete only when every figure was established,
/// otherwise partial with the first reason that was not.
fn judged(evidence: Evidence, gaps: &Gaps) -> Evidence {
    let reason = [&gaps.opening, &gaps.closing, &gaps.change]
        .into_iter()
        .find_map(Figure::reason);
    match reason {
        None => evidence,
        Some(reason) => Evidence {
            state: "partial",
            reason_code: Some(reason.code().to_string()),
            ..evidence
        },
    }
}

impl Server {
    pub(super) async fn statement_tie_out(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let stage = Stage::parse(required_string(args, "stage")?)?;
        let document = bank_statement::load_proposals(
            &self.settings.data_dir,
            args.get("proposals_id")
                .ok_or_else(|| "proposals_id_required".to_string())?,
            args.get("proposals_sha256"),
        )?;
        let file = StatementFile::read(&document)?;
        // A file whose window is not the whole statement is answered from the
        // file alone: nothing is read from Tally to compare against.
        let (first, last) = match file.window {
            Ok(window) => window,
            Err(reason) => {
                let gaps = all_not_established(reason);
                let payload = json!({"result": result_json(stage, None, &gaps)});
                let evidence = Evidence {
                    request_sha256: sha256_hex(b"statement_tie_out"),
                    response_sha256: sha256_json(&payload),
                    bytes: serde_json::to_vec(&payload).map_or(0, |bytes| bytes.len()),
                    state: "complete",
                    read_at: None,
                    duration_ms: None,
                    reason_code: None,
                };
                return Ok(ToolOutcome {
                    payload,
                    evidence: judged(evidence, &gaps),
                    company_guid: None,
                    truncated: false,
                });
            }
        };
        let (company, identity, mut evidence) = self.verified_company(guid).await?;
        let result: Result<ToolOutcome, ToolFailure> = async {
            let books_from = normalized_date(
                company
                    .books_from
                    .as_deref()
                    .ok_or_else(|| "company_identity_incomplete".to_string())?,
            )?;
            let gaps =
                if ensure_movement_window_within_books(&first.iso().replace('-', ""), &books_from)
                    .is_err()
                {
                    all_not_established(NotEstablished::WindowPrecedesBooksFrom)
                } else {
                    let mut reads = Vec::with_capacity(3);
                    for date in read_dates(first, last)? {
                        let (ledgers, ledger_evidence) =
                            self.read_movement_ledgers(&identity, date).await?;
                        evidence = combine_evidence(evidence.clone(), ledger_evidence);
                        reads.push(LedgerRead::of(&ledgers, &file.bank_ledger)?);
                    }
                    let [at_first, after_last, at_first_again]: [LedgerRead; 3] = reads
                        .try_into()
                        .map_err(|_| "ledger_snapshot_drifted".to_string())?;
                    gaps(
                        stage,
                        &file,
                        &Observed {
                            at_first,
                            after_last,
                            at_first_again,
                        },
                    )?
                };
            Ok(ToolOutcome {
                payload: json!({
                    "company": company_json(&company, std::slice::from_ref(&company)),
                    "result": result_json(stage, Some((first, last)), &gaps),
                }),
                evidence: judged(evidence.clone(), &gaps),
                company_guid: Some(guid.to_string()),
                truncated: false,
            })
        }
        .await;
        result.map_err(|failure| failure.with_prior_evidence(evidence))
    }
}

#[cfg(test)]
#[path = "agent_statement_tie_out_tests.rs"]
mod tests;
