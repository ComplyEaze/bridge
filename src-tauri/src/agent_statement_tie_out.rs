//! `statement_tie_out`: does the bank ledger in Tally stand where a parsed bank
//! statement says it stood at the start and at the end of the statement's own
//! dates? Read-only: it writes nothing to Tally and never posts. Its journal
//! check takes the shared import-admission lock for a moment, so a build or post
//! at the same time can be refused as lock-busy, and it can be refused by theirs;
//! nothing waits, so nothing deadlocks. Whole statements only. See
//! `docs/agent/README.md` ("Statement tie-out").
use super::*;
use bridge_bank_statement::date::Date;
use bridge_bank_statement::proposals::format_amount;
use bridge_tally_core::{ExactDecimal, TallyDate};

/// Whether this file's own vouchers are counted into the closing figure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum Stage {
    /// Before the file is built and imported: the closing figure is the book
    /// plus what this file's vouchers would add to the bank ledger.
    BeforeBuild,
    /// After an import, by anyone: the closing figure is the book as it stands.
    AfterPost,
}

/// Why a figure is not established. Each is a typed result, not an error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NotEstablished {
    WindowNotRecorded,
    WindowNarrowerThanStatement,
    WindowPrecedesBooksFrom,
    BankLedgerNotInBook,
    BookChangedDuringRead,
    /// `before_build` only: a row of the statement has no voucher in the file,
    /// so the file cannot say what the book would hold once it is imported.
    RowsWithoutVoucher,
    /// `before_build` only: the import journal already holds a row of this
    /// file, so its vouchers may already be in the book.
    FileAlreadyPartlyPosted,
}

impl NotEstablished {
    fn code(self) -> &'static str {
        match self {
            Self::WindowNotRecorded => "window_not_recorded",
            Self::WindowNarrowerThanStatement => "window_narrower_than_statement",
            Self::WindowPrecedesBooksFrom => "window_precedes_books_from",
            Self::BankLedgerNotInBook => "bank_ledger_not_in_book",
            Self::BookChangedDuringRead => "book_changed_during_read",
            Self::RowsWithoutVoucher => "rows_without_voucher",
            Self::FileAlreadyPartlyPosted => "file_already_partly_posted",
        }
    }

    fn explanation(self) -> &'static str {
        match self {
            Self::WindowNotRecorded => "this proposals file was written before it recorded the statement's dates; parse the statement again",
            Self::WindowNarrowerThanStatement => "this file was parsed with a from or to date that leaves out part of the statement, and this check ties whole statements only; parse the statement again without them",
            Self::WindowPrecedesBooksFrom => "the statement starts before the company's books begin",
            Self::BankLedgerNotInBook => "the file's bank ledger is not in the book under that exact name",
            Self::BookChangedDuringRead => "the bank ledger changed while it was being read; read again",
            Self::RowsWithoutVoucher => "some rows of the statement have no voucher in this file, so what the book would hold once it is imported is not known; use stage after_post once what is wanted is in the book",
            Self::FileAlreadyPartlyPosted => "a row of this file has already been sent to Tally or found posted, so counting the file's vouchers again would count them twice; use stage after_post",
        }
    }
}

/// An amount in the statement's sign: positive is money in the bank account.
/// Held in canonical form, so equal amounts are equal values.
#[derive(Clone, Debug, PartialEq, Eq)]
struct BankSide(ExactDecimal);

impl BankSide {
    fn canonical(value: ExactDecimal) -> Result<Self, String> {
        ExactDecimal::zero()
            .checked_add(&value)
            .map(Self)
            .map_err(|_| "tie_out_arithmetic_out_of_range".to_string())
    }

    /// A figure as the statement prints it, or an entry of the file read the
    /// same way (a debit to the bank ledger is money in).
    fn from_printed(text: &str) -> Result<Self, String> {
        Self::canonical(
            ExactDecimal::parse(text).map_err(|_| "proposals_file_invalid".to_string())?,
        )
    }

    /// The one conversion from Tally's sign, where a debit balance is negative.
    fn from_book(native: &str) -> Result<Self, String> {
        let native =
            ExactDecimal::parse(native).map_err(|_| "ledger_opening_invalid".to_string())?;
        Self::canonical(
            ExactDecimal::zero()
                .checked_subtract(&native)
                .map_err(|_| "tie_out_arithmetic_out_of_range".to_string())?,
        )
    }

    fn plus(&self, other: &Self) -> Result<Self, String> {
        Self::canonical(
            self.0
                .checked_add(&other.0)
                .map_err(|_| "tie_out_arithmetic_out_of_range".to_string())?,
        )
    }

    fn minus(&self, other: &Self) -> Result<Self, String> {
        Self::canonical(
            self.0
                .checked_subtract(&other.0)
                .map_err(|_| "tie_out_arithmetic_out_of_range".to_string())?,
        )
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Exact, at least two decimal places.
    fn text(&self) -> String {
        format_amount(&self.0)
    }

    /// The same without its sign.
    fn magnitude_text(&self) -> String {
        format_amount(&self.0.magnitude())
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

/// The three figures. Built only by [`gaps`] and [`all_not_established`], so
/// the figures that are not established share one reason.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Gaps {
    opening: Figure,
    closing: Figure,
    change: Figure,
}

fn all_not_established(reason: NotEstablished) -> Gaps {
    let figure = Figure::NotEstablished(reason);
    Gaps {
        opening: figure.clone(),
        closing: figure.clone(),
        change: figure,
    }
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
    /// Rows of the statement with no voucher in the file: a cash line not yet
    /// answered, a row skipped because another account's Contra carries it, or
    /// any disposition this check does not know (a row is counted as carried
    /// only when the file says it is a voucher).
    rows_without_voucher: usize,
}

impl StatementFile {
    /// Every field is parsed here, once, and a file that cannot be read is
    /// `proposals_file_invalid`: an empty or unreadable figure is not zero.
    fn read(document: &Value) -> Result<Self, String> {
        fn text(value: &Value) -> Result<&str, String> {
            value
                .as_str()
                .ok_or_else(|| "proposals_file_invalid".to_string())
        }
        let invalid = || "proposals_file_invalid".to_string();
        let bank_ledger = text(&document["bank_ledger"])?.to_string();
        let proposals_net = proposals_net(document, &bank_ledger)?;
        let rows_without_voucher = document["records"]
            .as_array()
            .ok_or_else(invalid)?
            .iter()
            .filter(|record| record["disposition"].get("voucher").is_none())
            .count();
        Ok(Self {
            opening: BankSide::from_printed(text(&document["controls"]["opening_balance"])?)?,
            closing: BankSide::from_printed(text(&document["controls"]["closing_balance"])?)?,
            window: recorded_window(document)?,
            bank_ledger,
            proposals_net,
            rows_without_voucher,
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
            net = match entry["side"].as_str() {
                Some("Dr") => net.plus(&amount)?,
                Some("Cr") => net.minus(&amount)?,
                _ => return Err(invalid()),
            };
        }
    }
    Ok(net)
}

/// The bank ledger as one catalogue read showed it, in the statement's sign.
#[derive(Clone, Debug, PartialEq, Eq)]
enum LedgerRead {
    Absent,
    Present(BankSide),
}

impl LedgerRead {
    /// By exact name. A name that appears twice in one catalogue is not a book
    /// this check can read, and a row without an opening is not read as zero
    /// (`ledger_opening_missing` is the typed error for a malformed read; the
    /// native reader already refuses such a row).
    fn of(ledgers: &[TallyLedger], name: &str) -> Result<Self, String> {
        let mut found = ledgers.iter().filter(|ledger| ledger.name == name);
        match (found.next(), found.next()) {
            (None, _) => Ok(Self::Absent),
            (Some(ledger), None) => ledger
                .opening_balance
                .as_deref()
                .ok_or_else(|| "ledger_opening_missing".to_string())
                .and_then(BankSide::from_book)
                .map(Self::Present),
            _ => Err("ledger_name_duplicated_in_catalogue".to_string()),
        }
    }
}

/// The catalogue dates a tie-out reads: the first row's date, the day after
/// the last (the book's position at the end of the last day), and the first
/// again. The third read catches a change dated before the first date made
/// between the first and the third read; a voucher dated inside the window and
/// posted in between moves only the end figure, and is not caught.
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

/// The three gaps from the three reads. `partly_posted` is whether the import
/// journal already holds a row of this file.
fn gaps(
    stage: Stage,
    file: &StatementFile,
    reads: &[LedgerRead; 3],
    partly_posted: bool,
) -> Result<Gaps, String> {
    let (start, end) = match reads {
        [LedgerRead::Absent, LedgerRead::Absent, LedgerRead::Absent] => {
            return Ok(all_not_established(NotEstablished::BankLedgerNotInBook))
        }
        [LedgerRead::Present(start), LedgerRead::Present(end), LedgerRead::Present(again)]
            if start == again =>
        {
            (start, end)
        }
        // Present in one read and absent in another, or moved between the two
        // reads at the first date.
        _ => return Ok(all_not_established(NotEstablished::BookChangedDuringRead)),
    };
    let opening = start.minus(&file.opening)?;
    // The file's own vouchers are counted into the closing figure only when
    // the file says what the book would hold, and only once.
    let blocked = match stage {
        Stage::AfterPost => None,
        Stage::BeforeBuild if file.rows_without_voucher > 0 => {
            Some(NotEstablished::RowsWithoutVoucher)
        }
        Stage::BeforeBuild if partly_posted => Some(NotEstablished::FileAlreadyPartlyPosted),
        Stage::BeforeBuild => None,
    };
    let (closing, change) = match blocked {
        Some(reason) => (
            Figure::NotEstablished(reason),
            Figure::NotEstablished(reason),
        ),
        None => {
            let book = match stage {
                Stage::AfterPost => end.clone(),
                Stage::BeforeBuild => end.plus(&file.proposals_net)?,
            };
            let closing = book.minus(&file.closing)?;
            let change = closing.minus(&opening)?;
            (Figure::Gap(closing), Figure::Gap(change))
        }
    };
    Ok(Gaps {
        opening: Figure::Gap(opening),
        closing,
        change,
    })
}

const SIGN_SENTENCE: &str = "A positive amount means the book shows more money in the bank than the statement does; a negative amount, less.";
const POSSIBLE_CAUSES: &str = "Uncleared cheques and deposits in transit explain differences like these. So can a missing entry, a repeated entry or the wrong bank ledger. This check cannot tell them apart. To see which vouchers are involved, read the bank ledger's vouchers for these dates.";
const SCOPE: &str =
    "This checks the bank ledger only, and takes the ledger named in the file to be a bank account: a ledger of another kind opens at zero for the period, so its gaps would mean nothing. A wrong party or expense ledger is not caught here.";

/// The fixed reading text. The headline comes first; it names every figure
/// that is not established, then every gap that is not zero, and prescribes
/// nothing.
fn reading(stage: Stage, window: Option<(Date, Date)>, gaps: &Gaps) -> Value {
    let subject = match window {
        Some((first, last)) => format!(
            "Bank ledger against the statement, {} to {}",
            first.iso(),
            last.iso()
        ),
        None => "Bank ledger against the statement".to_string(),
    };
    let figures = [
        ("opening gap", &gaps.opening),
        ("closing gap", &gaps.closing),
        ("change in window", &gaps.change),
    ];
    let mut parts = Vec::new();
    // The figures that are not established share one reason.
    let missing = figures
        .iter()
        .filter(|(_, figure)| figure.reason().is_some())
        .collect::<Vec<_>>();
    if let Some(reason) = missing.first().and_then(|(_, figure)| figure.reason()) {
        let names = missing.iter().map(|(label, _)| *label).collect::<Vec<_>>();
        let listed = match names.split_last() {
            Some((last, rest)) if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
            Some((only, _)) => (*only).to_string(),
            None => String::new(),
        };
        parts.push(format!(
            "{listed} not established ({})",
            reason.explanation()
        ));
    }
    let mut differs = false;
    for (label, figure) in &figures[..2] {
        if let Figure::Gap(gap) = figure {
            if !gap.is_zero() {
                differs = true;
                parts.push(format!("{label} {}", gap.text()));
            }
        }
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
    // By direction, so it reads right whether the money moved in or out: the
    // book's movement against the statement's.
    let moved = match &gaps.change {
        Figure::Gap(change) if !change.is_zero() => Some(format!(
            " Within these dates the book moved {} further {} than the statement did.",
            change.magnitude_text(),
            if change.0.is_negative() { "down" } else { "up" },
        )),
        _ => None,
    };
    headline.push_str(moved.as_deref().unwrap_or_default());
    let mut text = json!({
        "headline": headline,
        "stage": match stage {
            Stage::AfterPost => "The closing figure is the book as it stands now.",
            Stage::BeforeBuild => "The closing figure counts this file's vouchers as if they were already in the book. Once any of them has been imported, use stage after_post.",
        },
        "scope": SCOPE,
    });
    if differs || moved.is_some() {
        text["possible_causes"] = json!(POSSIBLE_CAUSES);
    }
    text
}

fn result_json(
    stage: Stage,
    window: Option<(Date, Date)>,
    rows_without_voucher: usize,
    gaps: &Gaps,
) -> Value {
    let figure = |figure: &Figure| {
        json!({
            "state": figure.state(),
            "amount": figure.amount(),
            "reason": figure.reason().map(NotEstablished::code),
        })
    };
    json!({
        "stage": stage,
        "window": window.map(|(first, last)| json!({
            "first_row_date": first.iso(),
            "last_row_date": last.iso(),
        })),
        "gap_basis": "book_minus_statement_in_the_statements_sign",
        "rows_without_voucher": rows_without_voucher,
        "opening_gap": figure(&gaps.opening),
        "closing_gap": figure(&gaps.closing),
        "change_in_window": figure(&gaps.change),
        "reading": reading(stage, window, gaps),
    })
}

impl Server {
    pub(super) async fn statement_tie_out(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let stage = serde_json::from_value(args["stage"].clone())
            .map_err(|_| "argument_invalid:stage".to_string())?;
        let document = bank_statement::load_proposals(
            &self.settings.data_dir,
            args.get("proposals_id")
                .ok_or_else(|| "proposals_id_required".to_string())?,
            args.get("proposals_sha256"),
        )?;
        let file = StatementFile::read(&document)?;
        let (company, identity, mut evidence) = self.verified_company(guid).await?;
        let result: Result<ToolOutcome, ToolFailure> = async {
            let (window, gaps) = match file.window {
                // The file alone answers: nothing is read to compare against.
                Err(reason) => (None, all_not_established(reason)),
                Ok((first, last)) => {
                    let books_from = normalized_date(
                        company
                            .books_from
                            .as_deref()
                            .ok_or_else(|| "company_identity_incomplete".to_string())?,
                    )?;
                    let first_day = normalized_date(&first.iso())?;
                    let gaps = if ensure_movement_window_within_books(&first_day, &books_from)
                        .is_err()
                    {
                        all_not_established(NotEstablished::WindowPrecedesBooksFrom)
                    } else {
                        let partly_posted = stage == Stage::BeforeBuild
                            && file.rows_without_voucher == 0
                            && self.import_journal_holds_rows_of(guid, &document["vouchers"])?;
                        let mut reads =
                            [LedgerRead::Absent, LedgerRead::Absent, LedgerRead::Absent];
                        for (read, date) in reads.iter_mut().zip(read_dates(first, last)?) {
                            let (ledgers, ledger_evidence) =
                                self.read_movement_ledgers(&identity, date).await?;
                            evidence = combine_evidence(evidence.clone(), ledger_evidence);
                            *read = LedgerRead::of(&ledgers, &file.bank_ledger)?;
                        }
                        gaps(stage, &file, &reads, partly_posted)?
                    };
                    (Some((first, last)), gaps)
                }
            };
            // Complete only when every figure was established.
            let evidence = match [&gaps.opening, &gaps.closing, &gaps.change]
                .into_iter()
                .find_map(Figure::reason)
            {
                None => evidence.clone(),
                Some(reason) => Evidence {
                    state: "partial",
                    reason_code: Some(reason.code().to_string()),
                    ..evidence.clone()
                },
            };
            Ok(ToolOutcome {
                payload: json!({
                    "company": company_json(&company, std::slice::from_ref(&company)),
                    "result": result_json(stage, window, file.rows_without_voucher, &gaps),
                }),
                evidence,
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
