// SPDX-License-Identifier: Apache-2.0
//! The assessee's documents, as the reference implementation's adapters parse them: the TRACES
//! documents -- Form 26AS rows (`Form26ASRow`), AIS entries (`AisRow`) and TIS category totals
//! (`TisRow`) -- and a bank statement (`BankStatementDoc`). This crate reads no document itself:
//! the rows are caller data, in the JSON `parity/python_golden.py --emit-traces-documents` or
//! `--emit-bank-statement` writes from the reference's own adapters, so both sides of a parity run
//! see the same rows. A real client's rows are client data and never belong in this repository; CI
//! uses invented rows only.
//!
//! The bank statement reader also refuses, as the reference's reader does, a statement that would
//! read falsely: no declared opening or closing balance, a row outside its own period, or a date
//! that steps back in an order its running balance does not confirm ([`StatementRefusal`]).

use bridge_tally_primitives::TallyDate;
use serde_json::Value;

use crate::error::{AuditError, Result};

/// One Form 26AS row: Part I is TDS, Part VI is TCS; the reference keeps other parts too.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Form26asRow {
    pub doc: String,
    pub row: i64,
    pub part: String,
    pub deductor_tan: String,
    pub section: String,
    pub txn_date: TallyDate,
    pub amount_paise: i64,
    pub tax_paise: i64,
}

/// One AIS entry: `category` is "gst_turnover", "gst_purchases", "advance_tax" or "refund".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AisRow {
    pub doc: String,
    pub row: i64,
    pub category: String,
    pub source_name: String,
    pub source_id: String,
    pub txn_date: Option<TallyDate>,
    pub amount_paise: i64,
}

/// One TIS category total, under TIS's own category text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TisRow {
    pub doc: String,
    pub row: i64,
    pub category: String,
    pub reported_paise: i64,
    pub processed_paise: i64,
    pub accepted_paise: i64,
}

/// Every TRACES document a run was given, each `None` when its file was not loaded and its rows
/// (possibly none) when it was: the reference's pack lists a loaded document as examined, rows or
/// none (#1281).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TracesDocuments {
    pub form26as: Option<Vec<Form26asRow>>,
    pub ais: Option<Vec<AisRow>>,
    pub tis: Option<Vec<TisRow>>,
}

impl TracesDocuments {
    /// The Form 26AS rows; none when it was not loaded.
    pub fn form26as_rows(&self) -> &[Form26asRow] {
        self.form26as.as_deref().unwrap_or_default()
    }

    /// The AIS rows; none when it was not loaded.
    pub fn ais_rows(&self) -> &[AisRow] {
        self.ais.as_deref().unwrap_or_default()
    }

    /// The TIS rows; none when it was not loaded.
    pub fn tis_rows(&self) -> &[TisRow] {
        self.tis.as_deref().unwrap_or_default()
    }
}

/// A bank statement, as the reference's `adapters/bank_documents.py` reads one extraction: the
/// document-level facts it carries once, and its transaction rows in the source's own order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BankStatementDoc {
    pub doc_id: String,
    pub source_sha256: String,
    /// The masked account number, as the extraction masks it.
    pub account_ref: String,
    pub bank: String,
    /// The calendar window the statement covers, both ends inclusive.
    pub start: TallyDate,
    pub end: TallyDate,
    pub opening_balance_paise: i64,
    pub closing_balance_paise: i64,
    pub rows: Vec<BankStatementRow>,
}

/// One statement transaction. `row` is its 0-based position in the source's list; `balance_paise`
/// is `None` where the source carries no running balance for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BankStatementRow {
    pub doc: String,
    pub row: i64,
    pub account_ref: String,
    pub txn_date: TallyDate,
    pub narration: String,
    pub debit_paise: i64,
    pub credit_paise: i64,
    pub balance_paise: Option<i64>,
}

/// Which declared balance a statement lacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceField {
    Opening,
    Closing,
}

/// Why the reference's bank-statement reader (`adapters/bank_documents.py`) refuses a statement:
/// the statement is well formed, but reading it would put a false sentence in front of the CA.
/// `row` is a 0-based position in the statement's list; [`StatementRefusal::reason`] is the
/// reader's plain-words reason, which `bank_reconciliation::refused` reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementRefusal {
    /// The statement declares no opening or closing balance: an absent balance is not Rs 0.
    BalanceMissing(BalanceField),
    /// A row dated outside the statement's own period, which may not meet its books entry.
    OutsidePeriod {
        row: usize,
        date: TallyDate,
        start: TallyDate,
        end: TallyDate,
    },
    /// The first row dated before the row listed above it, in an order the running balance does
    /// not confirm: `first_break` is the date of the first row whose balance does not follow the
    /// row before, or `None` when some row carries no balance to show it.
    OutOfOrder {
        row: usize,
        date: TallyDate,
        previous: TallyDate,
        first_break: Option<TallyDate>,
    },
}

impl StatementRefusal {
    /// The reader's reason, word for word: what the statement does, completing "the bank
    /// statement ...".
    pub fn reason(&self) -> String {
        match self {
            Self::BalanceMissing(BalanceField::Opening) => {
                "declares no opening balance".to_string()
            }
            Self::BalanceMissing(BalanceField::Closing) => {
                "declares no closing balance".to_string()
            }
            Self::OutsidePeriod {
                date, start, end, ..
            } => format!(
                "lists a transaction dated {} outside its own period, {} to {}",
                day_mon_year(date),
                day_mon_year(start),
                day_mon_year(end)
            ),
            Self::OutOfOrder {
                date,
                previous,
                first_break,
                ..
            } => {
                let how = match first_break {
                    None => "a row carries no running balance to show that order is the bank's"
                        .to_string(),
                    Some(at) => format!(
                        "its running balance does not hold in the listed order (it first breaks \
                         at the transaction dated {})",
                        day_mon_year(at)
                    ),
                };
                format!(
                    "lists a transaction dated {} after one dated {}, and {how}",
                    day_mon_year(date),
                    day_mon_year(previous)
                )
            }
        }
    }
}

impl std::fmt::Display for StatementRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BalanceMissing(_) => write!(
                f,
                "the bank statement {}; supply a statement extraction that carries it (an absent \
                 balance is not read as zero)",
                self.reason()
            ),
            Self::OutsidePeriod { row, .. } | Self::OutOfOrder { row, .. } => write!(
                f,
                "the bank statement {} (transaction {row}, counted from 0)",
                self.reason()
            ),
        }
    }
}

/// A date as Python's `strftime('%d-%b-%Y')` writes it in the C locale: `05-Jun-2025`.
fn day_mon_year(date: &TallyDate) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let s = date.as_str();
    let month: usize = s[4..6].parse().expect("a TallyDate's month is 01..12");
    format!("{}-{}-{}", &s[6..8], MONTHS[month - 1], &s[0..4])
}

/// The running-balance tolerance the order check allows (Re 1), as the reference's reader keeps
/// its own copy of `bank_reconciliation`'s.
const BALANCE_TOL_PAISE: i128 = 100;

/// The reference's `_check_order`: a date that steps back is refused unless every row carries a
/// balance and each follows the one listed before it within Re 1.
fn check_order(rows: &[BankStatementRow]) -> Result<()> {
    let Some(back) = (1..rows.len()).find(|&i| rows[i].txn_date < rows[i - 1].txn_date) else {
        return Ok(());
    };
    let refused = |first_break: Option<&TallyDate>| {
        AuditError::StatementRefused(StatementRefusal::OutOfOrder {
            row: back,
            date: rows[back].txn_date.clone(),
            previous: rows[back - 1].txn_date.clone(),
            first_break: first_break.cloned(),
        })
    };
    let mut balances = Vec::with_capacity(rows.len());
    for r in rows {
        match r.balance_paise {
            Some(b) => balances.push(i128::from(b)),
            None => return Err(refused(None)),
        }
    }
    // The reason names where the chain first breaks, which need not be at the back-step.
    for i in 1..rows.len() {
        let moved = i128::from(rows[i].credit_paise) - i128::from(rows[i].debit_paise);
        if (balances[i - 1] + moved - balances[i]).abs() > BALANCE_TOL_PAISE {
            return Err(refused(Some(&rows[i].txn_date)));
        }
    }
    Ok(())
}

/// A declared balance: absent or null is refused as missing, never read as zero.
fn declared_balance(v: &Value, key: &str, field: BalanceField) -> Result<i64> {
    match v.get(key) {
        None | Some(Value::Null) => Err(AuditError::StatementRefused(
            StatementRefusal::BalanceMissing(field),
        )),
        Some(_) => int(v, key, "statement"),
    }
}

/// A malformed field; the public reader it came through adds which document it was.
fn bad(what: &str) -> AuditError {
    AuditError::Config(what.to_string())
}

fn document(name: &str) -> impl Fn(AuditError) -> AuditError + '_ {
    move |e| match e {
        AuditError::Config(m) => AuditError::Config(format!("{name}: {m}")),
        other => other,
    }
}

fn text(v: &Value, key: &str, what: &str) -> Result<String> {
    v[key]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| bad(&format!("{what}.{key} is not a string")))
}

fn int(v: &Value, key: &str, what: &str) -> Result<i64> {
    v[key]
        .as_i64()
        .ok_or_else(|| bad(&format!("{what}.{key} is not an integer")))
}

/// An ISO date, `YYYY-MM-DD`, as the emitter writes `date.isoformat()`.
fn iso_date(v: &Value, key: &str, what: &str) -> Result<TallyDate> {
    let s = text(v, key, what)?;
    let b = s.as_bytes();
    let shaped = b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b.iter()
            .enumerate()
            .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit());
    shaped
        .then(|| TallyDate::parse(s.replace('-', "")).ok())
        .flatten()
        .ok_or_else(|| bad(&format!("{what}.{key} {s:?} is not YYYY-MM-DD")))
}

/// A document's rows, each read by `row`: `None` when `key` is absent or null (the file was not
/// loaded), its rows (possibly none) when it is a list.
fn rows<T>(v: &Value, key: &str, row: impl Fn(&Value) -> Result<T>) -> Result<Option<Vec<T>>> {
    match &v[key] {
        Value::Null => Ok(Some(Vec::new())),
        Value::Array(a) => a.iter().map(row).collect::<Result<_>>().map(Some),
        _ => Err(bad(&format!("{key} is not a list"))),
    }
}

/// The JSON `parity/python_golden.py --emit-traces-documents` writes: `form26as`, `ais` and `tis`
/// lists, each absent or null when its file was not loaded.
pub fn traces_documents_from_json(v: &Value) -> Result<TracesDocuments> {
    traces_documents(v).map_err(document("traces documents"))
}

fn traces_documents(v: &Value) -> Result<TracesDocuments> {
    if !v.is_object() {
        return Err(bad("not an object"));
    }
    let form26as = rows(v, "form26as", |r| {
        Ok(Form26asRow {
            doc: text(r, "doc", "form26as")?,
            row: int(r, "row", "form26as")?,
            part: text(r, "part", "form26as")?,
            deductor_tan: text(r, "deductor_tan", "form26as")?,
            section: text(r, "section", "form26as")?,
            txn_date: iso_date(r, "txn_date", "form26as")?,
            amount_paise: int(r, "amount_paise", "form26as")?,
            tax_paise: int(r, "tax_paise", "form26as")?,
        })
    })?;
    let ais = rows(v, "ais", |r| {
        Ok(AisRow {
            doc: text(r, "doc", "ais")?,
            row: int(r, "row", "ais")?,
            category: text(r, "category", "ais")?,
            source_name: text(r, "source_name", "ais")?,
            source_id: text(r, "source_id", "ais")?,
            txn_date: match &r["txn_date"] {
                Value::Null => None,
                _ => Some(iso_date(r, "txn_date", "ais")?),
            },
            amount_paise: int(r, "amount_paise", "ais")?,
        })
    })?;
    let tis = rows(v, "tis", |r| {
        Ok(TisRow {
            doc: text(r, "doc", "tis")?,
            row: int(r, "row", "tis")?,
            category: text(r, "category", "tis")?,
            reported_paise: int(r, "reported_paise", "tis")?,
            processed_paise: int(r, "processed_paise", "tis")?,
            accepted_paise: int(r, "accepted_paise", "tis")?,
        })
    })?;
    Ok(TracesDocuments { form26as, ais, tis })
}

/// The JSON `parity/python_golden.py --emit-bank-statement` writes: the document's own facts, a
/// `period` of `start`/`end`, and its `rows` (each `balance_paise` an integer or null).
///
/// A well-formed statement the reference's reader refuses is refused with
/// [`AuditError::StatementRefused`], checked in the reader's own order: a row outside the period,
/// then a date that steps back ([`StatementRefusal::OutOfOrder`]), then an absent or null opening
/// and then closing balance. Rows are read in the listed order, which the statement's own checks in
/// `bank_reconciliation` rely on.
pub fn bank_statement_from_json(v: &Value) -> Result<BankStatementDoc> {
    bank_statement(v).map_err(document("bank statement"))
}

fn bank_statement(v: &Value) -> Result<BankStatementDoc> {
    if !v.is_object() {
        return Err(bad("not an object"));
    }
    let period = &v["period"];
    if !period.is_object() {
        return Err(bad("period is not an object"));
    }
    let doc_id = text(v, "doc_id", "statement")?;
    let account_ref = text(v, "account_ref", "statement")?;
    let start = iso_date(period, "start", "period")?;
    let end = iso_date(period, "end", "period")?;
    if start > end {
        return Err(bad("period.start is after period.end"));
    }
    // Unlike the traces lists, a statement's rows and each row's balance are always written:
    // an absent key is a malformed document, never an empty statement or a missing balance.
    let Some(Value::Array(listed)) = v.get("rows") else {
        return Err(bad("rows is not a list"));
    };
    let rows = listed
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let row = BankStatementRow {
                doc: text(r, "doc", "rows")?,
                row: int(r, "row", "rows")?,
                account_ref: text(r, "account_ref", "rows")?,
                txn_date: iso_date(r, "txn_date", "rows")?,
                narration: text(r, "narration", "rows")?,
                debit_paise: int(r, "debit_paise", "rows")?,
                credit_paise: int(r, "credit_paise", "rows")?,
                balance_paise: match r.get("balance_paise") {
                    None => return Err(bad(&format!("rows[{i}].balance_paise is missing"))),
                    Some(Value::Null) => None,
                    Some(_) => Some(int(r, "balance_paise", "rows")?),
                },
            };
            let at = format!("rows[{i}]");
            if usize::try_from(row.row).ok() != Some(i) {
                return Err(bad(&format!("{at}.row is not its position")));
            }
            if row.doc != doc_id || row.account_ref != account_ref {
                return Err(bad(&format!("{at} names another document or account")));
            }
            if row.debit_paise < 0 || row.credit_paise < 0 {
                return Err(bad(&format!("{at} has a negative amount")));
            }
            if row.debit_paise != 0 && row.credit_paise != 0 {
                return Err(bad(&format!("{at} is both a debit and a credit")));
            }
            Ok(row)
        })
        .collect::<Result<Vec<_>>>()?;
    let source_sha256 = text(v, "source_sha256", "statement")?;
    let bank = text(v, "bank", "statement")?;
    // The reference reader's refusals, in its order.
    if let Some(i) = rows
        .iter()
        .position(|r| !(start <= r.txn_date && r.txn_date <= end))
    {
        return Err(AuditError::StatementRefused(
            StatementRefusal::OutsidePeriod {
                row: i,
                date: rows[i].txn_date.clone(),
                start,
                end,
            },
        ));
    }
    check_order(&rows)?;
    let opening_balance_paise =
        declared_balance(v, "opening_balance_paise", BalanceField::Opening)?;
    let closing_balance_paise =
        declared_balance(v, "closing_balance_paise", BalanceField::Closing)?;
    Ok(BankStatementDoc {
        doc_id,
        source_sha256,
        account_ref,
        bank,
        start,
        end,
        opening_balance_paise,
        closing_balance_paise,
        rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn documents_are_read_as_the_emitter_writes_them_and_bad_shapes_refuse() {
        let d = traces_documents_from_json(&json!({
            "form26as": [{"doc": "form26as:x", "row": 1, "part": "I", "deductor_tan": "TAN-A",
                          "section": "194C", "txn_date": "2025-06-30", "amount_paise": 100,
                          "tax_paise": 1}],
            "ais": [{"doc": "ais:x", "row": 2, "category": "refund", "source_name": "s",
                     "source_id": "i", "txn_date": null, "amount_paise": 5}],
            "tis": [{"doc": "tis:x", "row": 3, "category": "GST turnover", "reported_paise": 1,
                     "processed_paise": 2, "accepted_paise": 3}]
        }))
        .unwrap();
        assert_eq!(d.form26as_rows()[0].txn_date.as_str(), "20250630");
        assert_eq!(d.ais_rows()[0].txn_date, None);
        assert_eq!(d.tis_rows()[0].accepted_paise, 3);
        assert_eq!(
            traces_documents_from_json(&json!({})).unwrap(),
            TracesDocuments::default()
        );
        for bad in [
            json!([]),
            json!({"form26as": {}}),
            json!({"form26as": [{"doc": "d", "row": 1, "part": "I", "deductor_tan": "T",
                                 "section": "194C", "txn_date": "30-06-2025",
                                 "amount_paise": 1, "tax_paise": 1}]}),
            json!({"tis": [{"doc": "d", "row": "1", "category": "c", "reported_paise": 1,
                            "processed_paise": 1, "accepted_paise": 1}]}),
        ] {
            assert!(traces_documents_from_json(&bad).is_err(), "{bad}");
        }
        let err = traces_documents_from_json(&json!([])).unwrap_err();
        assert_eq!(
            format!("{err}"),
            format!(
                "{}",
                AuditError::Config("traces documents: not an object".into())
            )
        );
    }

    #[test]
    fn a_traces_document_not_loaded_is_none_and_one_loaded_with_no_rows_is_empty() {
        let none = TracesDocuments {
            form26as: None,
            ais: None,
            tis: None,
        };
        assert_eq!(traces_documents_from_json(&json!({})).unwrap(), none);
        assert_eq!(
            traces_documents_from_json(&json!({"form26as": null, "ais": null, "tis": null}))
                .unwrap(),
            none
        );
        let d = traces_documents_from_json(&json!({"form26as": [], "tis": []})).unwrap();
        assert_eq!(
            d,
            TracesDocuments {
                form26as: Some(Vec::new()),
                ais: None,
                tis: Some(Vec::new()),
            }
        );
        assert!(d.ais_rows().is_empty());
    }
    #[test]
    fn a_bank_statement_is_read_as_the_emitter_writes_it_and_bad_shapes_refuse() {
        let row = json!({"doc": "bank:x:statement", "row": 0, "account_ref": "XX12", "txn_date":
                         "2026-03-02", "narration": "NEFT", "debit_paise": 0, "credit_paise": 500,
                         "balance_paise": 1500});
        let doc = json!({"doc_id": "bank:x:statement", "source_sha256": "ab", "account_ref": "XX12",
                         "bank": "B", "period": {"start": "2026-03-01", "end": "2026-03-31"},
                         "opening_balance_paise": 1000, "closing_balance_paise": 1500,
                         "rows": [row, {"doc": "bank:x:statement", "row": 1, "account_ref": "XX12",
                                        "txn_date": "2026-03-03", "narration": "", "debit_paise": 1,
                                        "credit_paise": 0, "balance_paise": null}]});
        let d = bank_statement_from_json(&doc).unwrap();
        assert_eq!((d.start.as_str(), d.end.as_str()), ("20260301", "20260331"));
        assert_eq!(
            (
                d.rows.len(),
                d.rows[0].balance_paise,
                d.rows[1].balance_paise
            ),
            (2, Some(1500), None)
        );
        assert_eq!(d.opening_balance_paise, 1000);
        for (key, value) in [
            ("period", json!("2026-03")),
            ("opening_balance_paise", json!(1.5)),
            ("rows", json!({})),
            ("bank", json!(null)),
        ] {
            let mut broken = doc.clone();
            broken[key] = value;
            let err = bank_statement_from_json(&broken).unwrap_err();
            assert!(
                format!("{err}").contains("bank statement: "),
                "{key}: {err}"
            );
        }
        let mut broken = doc.clone();
        broken["rows"][0]["debit_paise"] = json!("0");
        assert!(bank_statement_from_json(&broken).is_err());
        for (field, value) in [
            ("row", json!(1)),
            ("doc", json!("bank:y:statement")),
            ("account_ref", json!("XX13")),
            ("credit_paise", json!(-1)),
            ("debit_paise", json!(1)),
            ("txn_date", json!("2026-04-01")),
            ("txn_date", json!("2026-02-28")),
        ] {
            let mut broken = doc.clone();
            broken["rows"][0][field] = value;
            assert!(bank_statement_from_json(&broken).is_err(), "{field}");
        }
        let mut broken = doc.clone();
        broken["rows"][1]["debit_paise"] = json!(-1);
        assert!(
            bank_statement_from_json(&broken).is_err(),
            "a negative debit"
        );
        let mut one_day = doc.clone();
        one_day["period"] = json!({"start": "2026-03-02", "end": "2026-03-02"});
        one_day["rows"] = json!([row]);
        assert!(
            bank_statement_from_json(&one_day).is_ok(),
            "a one-day statement"
        );
        let mut broken = doc.clone();
        broken["rows"][0]
            .as_object_mut()
            .unwrap()
            .remove("balance_paise");
        assert!(bank_statement_from_json(&broken).is_err(), "no balance key");
        let mut broken = doc.clone();
        broken.as_object_mut().unwrap().remove("rows");
        assert!(bank_statement_from_json(&broken).is_err(), "no rows key");
        let mut broken = doc.clone();
        broken["period"]["start"] = json!("2026-04-01");
        assert!(
            bank_statement_from_json(&broken).is_err(),
            "start after end"
        );
    }

    /// A June 2025 statement in the emitter's shape: rows of (date, debit paise, balance).
    fn june(opening: Value, closing: Value, rows: &[(&str, i64, Option<i64>)]) -> Value {
        let rows: Vec<Value> = rows
            .iter()
            .enumerate()
            .map(|(i, (on, debit, balance))| {
                json!({"doc": "bank:t", "row": i, "account_ref": "XX34", "txn_date": on,
                       "narration": "NEFT payment", "debit_paise": debit, "credit_paise": 0,
                       "balance_paise": balance})
            })
            .collect();
        json!({"doc_id": "bank:t", "source_sha256": "ab", "account_ref": "XX34", "bank": "B",
               "period": {"start": "2025-06-01", "end": "2025-06-30"},
               "opening_balance_paise": opening, "closing_balance_paise": closing, "rows": rows})
    }

    fn refusal(v: &Value) -> StatementRefusal {
        match bank_statement_from_json(v) {
            Err(AuditError::StatementRefused(r)) => r,
            other => panic!("not refused: {other:?}"),
        }
    }

    /// The reference reader's own selftest cases (`selftest/test_bank_documents.py` at reference
    /// `da9e2d3d`), on the emitter's shape: each refusal by its variant, with the reader's words.
    #[test]
    fn a_statement_the_reference_reader_refuses_is_refused_with_its_reason() {
        let one = [("2025-06-05", 100_000, Some(400_000))];
        // An absent or null balance is refused, opening first; a declared zero is a balance.
        for (key, value) in [
            ("opening_balance_paise", None),
            ("opening_balance_paise", Some(Value::Null)),
            ("closing_balance_paise", None),
            ("closing_balance_paise", Some(Value::Null)),
        ] {
            let mut v = june(json!(500_000), json!(400_000), &one);
            match value {
                Some(x) => v[key] = x,
                None => {
                    v.as_object_mut().unwrap().remove(key);
                }
            }
            let field = if key.starts_with("opening") {
                BalanceField::Opening
            } else {
                BalanceField::Closing
            };
            assert_eq!(
                refusal(&v),
                StatementRefusal::BalanceMissing(field),
                "{key}"
            );
        }
        assert_eq!(
            refusal(&june(Value::Null, Value::Null, &one)),
            StatementRefusal::BalanceMissing(BalanceField::Opening)
        );
        assert_eq!(
            refusal(&june(json!(500_000), Value::Null, &one)).reason(),
            "declares no closing balance"
        );
        let zero = bank_statement_from_json(&june(json!(0), json!(0), &[])).unwrap();
        assert_eq!(
            (zero.opening_balance_paise, zero.closing_balance_paise),
            (0, 0)
        );

        // A back-valued row with an intact running balance is read, in the listed order.
        let back = [
            ("2025-06-10", 50_000, Some(450_000)),
            ("2025-06-12", 40_000, Some(410_000)),
            ("2025-06-11", 60_000, Some(350_000)),
        ];
        let d = bank_statement_from_json(&june(json!(500_000), json!(350_000), &back)).unwrap();
        let days: Vec<&str> = d.rows.iter().map(|r| &r.txn_date.as_str()[6..]).collect();
        assert_eq!(days, ["10", "12", "11"]);
        // Newest first: the balances do not run in that order.
        let newest = [
            ("2025-06-20", 50_000, Some(400_000)),
            ("2025-06-05", 50_000, Some(450_000)),
        ];
        let r = refusal(&june(json!(500_000), json!(400_000), &newest));
        assert!(
            matches!(r, StatementRefusal::OutOfOrder { row: 1, .. }),
            "{r:?}"
        );
        assert_eq!(
            r.reason(),
            "lists a transaction dated 05-Jun-2025 after one dated 20-Jun-2025, and its running \
             balance does not hold in the listed order (it first breaks at the transaction dated \
             05-Jun-2025)"
        );
        // Rows with no balance cannot show the order is the bank's.
        let bare = [("2025-06-20", 50_000, None), ("2025-06-05", 50_000, None)];
        assert_eq!(
            refusal(&june(json!(500_000), json!(400_000), &bare)).reason(),
            "lists a transaction dated 05-Jun-2025 after one dated 20-Jun-2025, and a row carries \
             no running balance to show that order is the bank's"
        );
        // A break away from the back-step is named where it is.
        let mut later = vec![
            ("2025-06-02", 1_000, Some(9_000)),
            ("2025-06-01", 1_000, Some(8_000)),
        ];
        for (on, balance) in [
            ("2025-06-03", 7_000),
            ("2025-06-04", 6_000),
            ("2025-06-05", 5_000),
            ("2025-06-06", 4_000),
            ("2025-06-07", 500),
        ] {
            later.push((on, 1_000, Some(balance)));
        }
        let r = refusal(&june(json!(10_000), json!(500), &later));
        assert!(
            matches!(&r, StatementRefusal::OutOfOrder { row: 1, first_break: Some(at), .. } if at.as_str() == "20250607"),
            "{r:?}"
        );
        // The order chain allows Re 1 and not a paisa more.
        for (last, read) in [(349_900, true), (349_899, false)] {
            let rows = [
                ("2025-06-10", 50_000, Some(450_000)),
                ("2025-06-09", 100_000, Some(last)),
            ];
            let got = bank_statement_from_json(&june(json!(500_000), json!(last), &rows));
            assert_eq!(got.is_ok(), read, "{last}: {got:?}");
        }
        // Rows on one date, and in date order, are read.
        let same = [
            ("2025-06-05", 50_000, None),
            ("2025-06-05", 40_000, None),
            ("2025-06-20", 10_000, None),
        ];
        assert!(bank_statement_from_json(&june(json!(500_000), json!(400_000), &same)).is_ok());

        // A row outside the period, either side, is refused naming its date and the period; rows
        // on its first and last days are read.
        for (on, shown) in [("2025-05-31", "31-May-2025"), ("2025-07-01", "01-Jul-2025")] {
            let rows = [
                ("2025-06-05", 50_000, Some(450_000)),
                (on, 50_000, Some(400_000)),
            ];
            let r = refusal(&june(json!(500_000), json!(400_000), &rows));
            assert!(
                matches!(r, StatementRefusal::OutsidePeriod { row: 1, .. }),
                "{r:?}"
            );
            assert_eq!(
                r.reason(),
                format!("lists a transaction dated {shown} outside its own period, 01-Jun-2025 to 30-Jun-2025")
            );
        }
        let ends = [
            ("2025-06-01", 50_000, Some(450_000)),
            ("2025-06-30", 50_000, Some(400_000)),
        ];
        assert!(bank_statement_from_json(&june(json!(500_000), json!(400_000), &ends)).is_ok());
        // The reader's order: the period before the order, the order before the balances.
        let both = [
            ("2025-06-20", 50_000, None),
            ("2025-06-05", 50_000, None),
            ("2025-07-01", 50_000, None),
        ];
        let r = refusal(&june(Value::Null, Value::Null, &both));
        assert!(
            matches!(r, StatementRefusal::OutsidePeriod { row: 2, .. }),
            "{r:?}"
        );
        let r = refusal(&june(Value::Null, Value::Null, &both[..2]));
        assert!(
            matches!(r, StatementRefusal::OutOfOrder { row: 1, .. }),
            "{r:?}"
        );
        // The message adds the position, as the reader's does, and every month is named.
        assert_eq!(
            format!("{}", AuditError::StatementRefused(r)),
            "the bank statement lists a transaction dated 05-Jun-2025 after one dated 20-Jun-2025, \
             and a row carries no running balance to show that order is the bank's (transaction \
             1, counted from 0)"
        );
        let months: Vec<String> = (1..=12)
            .map(|m| day_mon_year(&TallyDate::parse(format!("2025{m:02}09")).unwrap()))
            .collect();
        assert_eq!(
            months.join(" "),
            "09-Jan-2025 09-Feb-2025 09-Mar-2025 09-Apr-2025 09-May-2025 09-Jun-2025 09-Jul-2025 \
             09-Aug-2025 09-Sep-2025 09-Oct-2025 09-Nov-2025 09-Dec-2025"
        );
    }
}
