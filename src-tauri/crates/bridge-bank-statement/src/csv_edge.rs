//! one private bank's CSV account statement export, read at the edge into
//! the statement `Row`.
//!
//! The one layout read so far, taken from the structure of one real export (a
//! file of 267 transactions for a financial year; no value of it is kept in
//! this repository, and every fixture is synthetic rows in this layout):
//! ASCII in the file measured (non-ASCII text in a description is kept as
//! written), LF line ends, no byte-order mark, every field double-quoted and
//! comma separated; a preamble of ragged rows (the customer's name and
//! address, with the account's key/value pairs beside them), one header of
//! exactly nine columns, the transactions, and a trailer that opens with the
//! `Closing Balance` line. Two columns share the name `Dr / Cr` (the direction
//! of the amount, then of the balance), so cells are read by position.
//!
//! Nothing downstream re-checks the shapes this reads. A file either matches
//! the layout exactly and becomes rows whose dates are `YYYY-MM-DD` and whose
//! amounts are plain two-place decimals (a debit and a credit unsigned, in
//! their own columns; a balance signed, a debit balance negative), or it is
//! refused, with the row it failed at where the refusal is about a row. The reader is closed: it parses
//! the one quoting this layout uses and refuses anything else (an unquoted
//! field, a doubled quote it cannot place, a quoted line break, a carriage
//! return, a byte-order mark, a UTF-16 file), so a file a spreadsheet has
//! re-saved is refused, not guessed at.
//!
//! - The header is found by its exact nine cells, once, wherever it sits (the
//!   address block's length varies); a file without it is `layout_unknown`.
//! - `Sl. No.` runs 1, 2, 3 and so on; the transactions end at the
//!   `Closing Balance` line, and nothing after it may look like a row.
//! - The row's date is the **value date**, which the bank's own note
//!   recommends for reconciliation (it differs from the transaction date when
//!   interest is posted after midnight). The transaction time is checked
//!   (shape and order, the file's order) and then dropped.
//! - The cheque or reference cell is kept apart from the description, under
//!   [`CHQ_REF`], with its internal runs of spaces as written.
//! - The opening balance is not printed: it is the first row's balance less
//!   that row's amount. The closing balance line must equal the last row's
//!   balance. Replaying the chain row by row is `reconcile`'s work, with the
//!   two balances this returns. Because the opening is derived from row 1,
//!   the replay cannot fault row 1: its amount and direction are proven by
//!   nothing here, and a file of one row proves only itself.
//! - Value dates are checked for shape and validity, not for order or
//!   against the transaction date; the preamble's `Period` is not read.
//!
//! Not measured: a balance marked `DR` (an overdrawn account) is read as a
//! negative balance by the layout's own indicator, but the closing balance
//! line of such a file has not been seen and is refused if it is not the
//! plain positive form; a statement of another year's layout, an amount
//! printed in another grouping, and every other bank's CSV.

use crate::bank::{BALANCE, CREDIT, DATE, DEBIT, NARRATION};
use crate::date::{parse_day_month_year_hyphenated, parse_day_month_year_slashed};
use crate::parse::Row;
use crate::refusal::Refusal;
use crate::text::strip;
use bridge_tally_primitives::ExactDecimal;
use regex::Regex;
use std::sync::LazyLock;

/// The cheque or reference cell, kept apart from the description.
pub const CHQ_REF: &str = "chq_ref";

/// The header, cell by cell, exactly as the bank prints it.
pub const HEADER_CELLS: [&str; 9] = [
    "Sl. No.",
    "Transaction Date",
    "Value Date",
    "Description",
    "Chq /Ref No.",
    "Amount",
    "Dr / Cr",
    "Balance",
    "Dr / Cr",
];

const CLOSING_BALANCE: &str = "Closing Balance";
/// The longest amount cell read, in bytes.
const MAX_AMOUNT_CELL: usize = 32;

/// What the file holds once it has been read whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvStatement {
    /// In file order, which is the bank's posting order.
    pub rows: Vec<Row>,
    /// The first row's balance before its own amount.
    pub opening: ExactDecimal,
    /// The `Closing Balance` line, equal to the last row's balance.
    pub closing: ExactDecimal,
}

fn refuse(category: &'static str, row: usize, what: &str) -> Refusal {
    Refusal::at_row(category, row, format!("row {row}: {what}"))
}

/// One line as fields. Every field is quoted; `""` inside one is a quote. A
/// blank line has no fields. Anything else is not this layout's quoting.
fn fields(line: &str) -> Option<Vec<String>> {
    if line.is_empty() {
        return Some(Vec::new());
    }
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut at = 0;
    loop {
        if bytes.get(at) != Some(&b'"') {
            return None;
        }
        at += 1;
        let mut field = String::new();
        loop {
            let rest = &line[at..];
            let quote = rest.find('"')?;
            field.push_str(&rest[..quote]);
            at += quote + 1;
            if bytes.get(at) == Some(&b'"') {
                field.push('"');
                at += 1;
            } else {
                break;
            }
        }
        out.push(field);
        match bytes.get(at) {
            None => return Some(out),
            Some(b',') => at += 1,
            Some(_) => return None,
        }
    }
}

/// An amount in Indian grouping with exactly two decimals, as the bank prints
/// it, as a plain decimal. Anything else (another grouping, a sign, one or
/// three decimals, a leading zero) is not the export's spelling.
fn amount(text: &str) -> Option<String> {
    // No real amount is this long; the bound keeps every decimal built from a
    // cell far inside the exact-decimal limit.
    if text.len() > MAX_AMOUNT_CELL {
        return None;
    }
    static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(0|[1-9][0-9]{0,2}|[1-9][0-9]?(,[0-9]{2})*,[0-9]{3})\.[0-9]{2}$").unwrap()
    });
    PATTERN.is_match(text).then(|| text.replace(',', ""))
}

/// `DD-MM-YYYY HH:MM:SS` with a real date and a real time, as a key that sorts
/// in time order.
fn posting_time(text: &str) -> Option<String> {
    static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^([0-9]{2}-[0-9]{2}-[0-9]{4}) ([0-9]{2}):([0-9]{2}):([0-9]{2})$").unwrap()
    });
    let found = PATTERN.captures(text)?;
    let date = parse_day_month_year_hyphenated(&found[1])?;
    let (hour, minute, second): (u8, u8, u8) = (
        found[2].parse().ok()?,
        found[3].parse().ok()?,
        found[4].parse().ok()?,
    );
    (hour < 24 && minute < 60 && second < 60)
        .then(|| format!("{} {hour:02}:{minute:02}:{second:02}", date.iso()))
}

fn decimal(text: &str) -> ExactDecimal {
    ExactDecimal::parse(text.to_string()).expect("a validated plain decimal")
}

/// The balance with the sign its own indicator gives it: `CR` is positive, `DR`
/// a debit balance, negative.
fn signed_balance(plain: &str, indicator: &str, row: usize) -> Result<String, Refusal> {
    match indicator {
        "CR" => Ok(plain.to_string()),
        "DR" if decimal(plain).is_zero() => Ok(plain.to_string()),
        "DR" => Ok(format!("-{plain}")),
        _ => Err(refuse(
            "direction_unknown",
            row,
            "the balance's Dr / Cr cell is neither CR nor DR",
        )),
    }
}

/// The whole file as a statement, or the first refusal.
pub fn read_statement(bytes: &[u8]) -> Result<CsvStatement, Refusal> {
    let text = match std::str::from_utf8(bytes) {
        Ok(text) if !bytes.contains(&0) && !text.starts_with('\u{feff}') => text,
        _ => {
            return Err(Refusal::new(
                "encoding_unsupported",
                "the file is not UTF-8 text without a byte-order mark (a UTF-16 file, or one with NUL bytes)",
            ))
        }
    };
    if text.contains('\r') {
        return Err(Refusal::new(
            "encoding_unsupported",
            "the file has carriage returns: the export's line ends are LF, so this is a re-saved file",
        ));
    }
    let lines: Vec<&str> = text.split('\n').collect();
    let parsed: Vec<Option<Vec<String>>> = lines.iter().map(|line| fields(line)).collect();

    let headers: Vec<usize> = parsed
        .iter()
        .enumerate()
        .filter(|(_, line)| {
            line.as_ref()
                .is_some_and(|cells| cells.iter().map(String::as_str).eq(HEADER_CELLS))
        })
        .map(|(index, _)| index)
        .collect();
    let [header] = headers[..] else {
        return Err(Refusal::new(
            "layout_unknown",
            if headers.is_empty() {
                "the file has no header line of the nine columns this layout reads"
            } else {
                "the file has the header line more than once"
            },
        ));
    };

    // The preamble is quoted fields like the rest: a line that is not is not
    // this export.
    if parsed[..header].iter().any(Option::is_none) {
        return Err(Refusal::new(
            "layout_unknown",
            "a line before the header is not quoted comma-separated fields",
        ));
    }
    let mut rows = Vec::new();
    let mut before: Option<String> = None;
    let mut balances: Vec<ExactDecimal> = Vec::new();
    let mut opening = None;
    let mut trailer = None;
    for (offset, cells) in parsed.iter().enumerate().skip(header + 1) {
        let number = rows.len() + 1;
        let Some(cells) = cells else {
            return Err(refuse(
                "malformed_row",
                number,
                "the line is not quoted comma-separated fields",
            ));
        };
        // The file's last line ends in a line break: that is not a blank row.
        if cells.is_empty() && offset + 1 == parsed.len() {
            break;
        }
        if cells.len() == 2 && cells[0] == CLOSING_BALANCE {
            trailer = Some((offset, cells[1].clone()));
            break;
        }
        if cells.len() != HEADER_CELLS.len() {
            return Err(refuse(
                "malformed_row",
                number,
                "the line does not have the header's nine columns",
            ));
        }
        if cells[0] != number.to_string() {
            return Err(refuse(
                "serial_broken",
                number,
                "the Sl. No. is not the next whole number",
            ));
        }
        let Some(posted) = posting_time(&cells[1]) else {
            return Err(refuse(
                "unparseable_date",
                number,
                "the transaction date is not DD-MM-YYYY HH:MM:SS",
            ));
        };
        if before.as_ref().is_some_and(|earlier| *earlier > posted) {
            return Err(refuse(
                "date_order_broken",
                number,
                "the transaction dates are not in the file's posting order",
            ));
        }
        before = Some(posted);
        let Some(value_date) = parse_day_month_year_hyphenated(&cells[2]) else {
            return Err(refuse(
                "unparseable_date",
                number,
                "the value date is not DD-MM-YYYY",
            ));
        };
        let Some(moved) = amount(&cells[5]) else {
            return Err(refuse(
                "malformed_amount",
                number,
                "the amount is not in Indian grouping with two decimals",
            ));
        };
        let Some(plain_balance) = amount(&cells[7]) else {
            return Err(refuse(
                "malformed_balance",
                number,
                "the balance is not in Indian grouping with two decimals",
            ));
        };
        let credited = match cells[6].as_str() {
            "CR" => true,
            "DR" => false,
            _ => {
                return Err(refuse(
                    "direction_unknown",
                    number,
                    "the amount's Dr / Cr cell is neither CR nor DR",
                ))
            }
        };
        let balance = signed_balance(&plain_balance, &cells[8], number)?;
        let signed_amount = decimal(&moved);
        let this_balance = decimal(&balance);
        if opening.is_none() {
            // The balance before this row: after, less a credit or plus a debit.
            let before_row = if credited {
                this_balance.checked_subtract(&signed_amount)
            } else {
                this_balance.checked_add(&signed_amount)
            }
            .map_err(|_| refuse("malformed_balance", number, "the balance does not fit"))?;
            opening = Some(before_row);
        }
        balances.push(this_balance);
        let (debit, credit) = if credited {
            ("", moved.as_str())
        } else {
            (moved.as_str(), "")
        };
        rows.push(Row::from_pairs([
            (DATE, value_date.iso().as_str()),
            (NARRATION, strip(&cells[3])),
            (CHQ_REF, cells[4].as_str()),
            (DEBIT, debit),
            (CREDIT, credit),
            (BALANCE, balance.as_str()),
        ]));
    }

    let Some((trailer_at, closing_cell)) = trailer else {
        return Err(Refusal::new(
            "trailer_missing",
            "the file ends without the Closing Balance line",
        ));
    };
    // Nothing after the closing line may look like a row.
    for (offset, cells) in parsed.iter().enumerate().skip(trailer_at + 1) {
        match cells {
            Some(cells) if cells.len() <= 1 => {}
            _ => {
                return Err(refuse(
                    "trailer_unexpected",
                    rows.len(),
                    &format!(
                        "line {} after the closing balance is not a note",
                        offset + 1
                    ),
                ))
            }
        }
    }
    let (Some(opening), Some(last)) = (opening, balances.last()) else {
        return Err(Refusal::new(
            "empty_statement",
            "the file has a header and a closing balance but no transaction",
        ));
    };
    static CLOSING: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^as on ([0-9]{2}/[0-9]{2}/[0-9]{4}) INR ([0-9][0-9,]*\.[0-9]{2})$").unwrap()
    });
    let closing = CLOSING
        .captures(&closing_cell)
        .filter(|found| parse_day_month_year_slashed(&found[1]).is_some())
        .and_then(|found| amount(&found[2]))
        .ok_or_else(|| {
            Refusal::new(
                "closing_balance_unreadable",
                "the Closing Balance line is not 'as on DD/MM/YYYY INR <amount>' with an Indian-grouped positive amount",
            )
        })?;
    let closing = decimal(&closing);
    if !closing.numeric_eq(last) {
        return Err(Refusal::new(
            "closing_balance_differs",
            "the Closing Balance line is not the last row's balance",
        ));
    }
    Ok(CsvStatement {
        rows,
        opening,
        closing,
    })
}
