//! A bank's own CSV export, read at the edge into the statement `Row`.
//!
//! Nothing downstream re-checks the shapes this reads: a file either matches a
//! [`CsvLayout`] exactly and becomes rows whose dates are `YYYY-MM-DD` and whose
//! amounts are plain two-place decimals, or it is refused with the row it failed
//! at. Which amount cells a row fills is left to the later stages, as on the PDF
//! path: `reconcile` refuses a row with both filled (`two_sided_row`) and the
//! proposals refuse one with neither inside the run's date window
//! (`row_without_amount`). Whether a date
//! falls inside the statement's period is not checked here, and no range of
//! plausible dates is applied. The running-balance chain sees amounts and not
//! dates, so the checks the PDF path gets from geometry are made here by shape:
//! an exact header, one date format with two-digit day and month, one declared
//! thousands grouping, and dates in the declared order.
//!
//! A layout is meant to be shipped data, not supplied by a caller, but the type
//! does not enforce that yet: its fields are public so the tests can build the
//! invented layout they read, which is why `layout_invalid` exists. This module
//! ships none (`LAYOUTS` is empty): a layout's header and column order come from
//! a real export, and no export was read to write this engine. Narrowing
//! [`read_rows`] to the shipped list waits for the first real layout.
//!
//! Row numbers in a refusal count the file's data records from 1, in file order
//! (a blank line is skipped by the reader and a quoted narration spanning lines
//! is one record), including for a newest-first layout, whose rows are returned
//! oldest first: a refusal from a later stage numbers them in that order, so
//! the two numberings differ for such a layout. Date, amount and narration
//! cells are trimmed of surrounding whitespace before they are read; a
//! narration is otherwise kept as written, including a leading `=`, `+`, `-` or
//! `@`, a line break inside quotes, and any other control character.

use crate::bank::{BALANCE, CREDIT, DATE, DEBIT, NARRATION};
use crate::date::{parse_day_month_year_hyphenated, parse_day_month_year_slashed, Date};
use crate::parse::Row;
use crate::refusal::Refusal;
use crate::text::strip;
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateFormat {
    /// `DD-MM-YYYY`
    DayMonthYearHyphen,
    /// `DD/MM/YYYY`
    DayMonthYearSlash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowOrder {
    OldestFirst,
    NewestFirst,
}

/// How an amount cell groups its digits. A cell that does not follow the
/// declared grouping is refused, so a file edited in a spreadsheet (which
/// regroups or ungroups cells) does not pass as the bank's export.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grouping {
    /// `1234567.50`
    None,
    /// `1,234,567.50`
    Western,
    /// `12,34,567.50`
    Indian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CsvLayout {
    pub id: &'static str,
    /// The header line, cell by cell, exactly as the bank prints it.
    pub header: &'static [&'static str],
    pub date: usize,
    pub date_format: DateFormat,
    pub order: RowOrder,
    pub narration: usize,
    pub debit: usize,
    pub credit: usize,
    pub balance: usize,
    pub grouping: Grouping,
}

/// The layouts this build reads. Each comes from a real export; none yet.
pub const LAYOUTS: &[&CsvLayout] = &[];

fn refuse(category: &'static str, row: usize, what: &str) -> Refusal {
    Refusal::at_row(category, row, format!("row {row}: {what}"))
}

fn grouped(grouping: Grouping) -> &'static Regex {
    static NONE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^-?(0|[1-9][0-9]*)(\.[0-9]{1,2})?$").unwrap());
    static WESTERN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^-?(0|[1-9][0-9]{0,2}(,[0-9]{3})*)(\.[0-9]{1,2})?$").unwrap()
    });
    static INDIAN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^-?(0|[1-9][0-9]?(,[0-9]{2})*,[0-9]{3}|[1-9][0-9]{0,2})(\.[0-9]{1,2})?$")
            .unwrap()
    });
    match grouping {
        Grouping::None => &NONE,
        Grouping::Western => &WESTERN,
        Grouping::Indian => &INDIAN,
    }
}

/// An amount cell as a plain decimal with two places: `1,000.5` is `1000.50`.
/// Blank is `None`. A leading zero (`0100.00`) is refused, so one value has one
/// spelling and the transaction id, which hashes the cell text, is stable. The
/// layout's grouping decides what a comma means: a Western `1,234` is 1234.
fn amount(
    text: &str,
    grouping: Grouping,
    signed: bool,
    row: usize,
    category: &'static str,
) -> Result<Option<String>, Refusal> {
    let text = strip(text);
    if text.is_empty() {
        return Ok(None);
    }
    if !grouped(grouping).is_match(text) || (!signed && text.starts_with('-')) {
        return Err(refuse(
            category,
            row,
            "the cell is not an amount in the layout's grouping with at most two decimal places",
        ));
    }
    let plain = text.replace(',', "");
    Ok(Some(match plain.split_once('.') {
        None => format!("{plain}.00"),
        Some((whole, fraction)) => format!("{whole}.{fraction:0<2}"),
    }))
}

fn date_of(text: &str, format: DateFormat) -> Option<Date> {
    match format {
        DateFormat::DayMonthYearHyphen => parse_day_month_year_hyphenated(text),
        DateFormat::DayMonthYearSlash => parse_day_month_year_slashed(text),
    }
}

/// The file's rows, oldest first, or the first refusal.
pub fn read_rows(bytes: &[u8], layout: &CsvLayout) -> Result<Vec<Row>, Refusal> {
    // A UTF-16 file, or any file with a NUL, is not read as if it were UTF-8.
    // The reader drops a UTF-8 byte-order mark itself (a test pins that).
    let text = match std::str::from_utf8(bytes) {
        Ok(text) if !bytes.contains(&0) => text,
        _ => {
            return Err(Refusal::new(
                "encoding_unsupported",
                "the file is not UTF-8 text (a UTF-16 file, or one with NUL bytes)",
            ))
        }
    };
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(false)
        .from_reader(text.as_bytes());
    let columns = [
        layout.date,
        layout.narration,
        layout.debit,
        layout.credit,
        layout.balance,
    ];
    let distinct = columns
        .iter()
        .all(|column| columns.iter().filter(|other| *other == column).count() == 1);
    if !distinct || columns.iter().any(|column| *column >= layout.header.len()) {
        return Err(Refusal::new(
            "layout_invalid",
            "the layout's columns are not five distinct columns of its header",
        ));
    }
    let mut records = reader.records();
    let header_matches = match records.next() {
        Some(Ok(header)) => header.iter().eq(layout.header.iter().copied()),
        _ => false,
    };
    if !header_matches {
        return Err(Refusal::new(
            "layout_unknown",
            "the header line is not the one this layout reads",
        ));
    }
    let mut rows = Vec::new();
    let mut previous: Option<Date> = None;
    for (index, record) in records.enumerate() {
        let number = index + 1;
        let record = record.map_err(|_| {
            refuse(
                "malformed_row",
                number,
                "the line does not have the header's columns",
            )
        })?;
        let cell = |column: usize| record.get(column).unwrap_or("");
        let Some(date) = date_of(strip(cell(layout.date)), layout.date_format) else {
            return Err(refuse(
                "unparseable_date",
                number,
                "the date is not in the layout's format",
            ));
        };
        let in_order = previous.is_none_or(|before| match layout.order {
            RowOrder::OldestFirst => before <= date,
            RowOrder::NewestFirst => before >= date,
        });
        if !in_order {
            return Err(refuse(
                "date_order_broken",
                number,
                "the dates are not in the order the layout declares",
            ));
        }
        previous = Some(date);
        let debit = amount(
            cell(layout.debit),
            layout.grouping,
            false,
            number,
            "malformed_amount",
        )?;
        let credit = amount(
            cell(layout.credit),
            layout.grouping,
            false,
            number,
            "malformed_amount",
        )?;
        let balance = amount(
            cell(layout.balance),
            layout.grouping,
            true,
            number,
            "malformed_balance",
        )?
        .ok_or_else(|| refuse("malformed_balance", number, "the balance is blank"))?;
        rows.push(Row::from_pairs([
            (DATE, date.iso().as_str()),
            (NARRATION, strip(cell(layout.narration))),
            (DEBIT, debit.as_deref().unwrap_or("")),
            (CREDIT, credit.as_deref().unwrap_or("")),
            (BALANCE, balance.as_str()),
        ]));
    }
    if layout.order == RowOrder::NewestFirst {
        rows.reverse();
    }
    Ok(rows)
}
