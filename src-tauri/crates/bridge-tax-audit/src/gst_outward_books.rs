// SPDX-License-Identifier: Apache-2.0
//! The books side of GST outward supplies: one row per books-population Sales, Credit Note or
//! Debit Note voucher, a port of the reference Python implementation's
//! `gst_outward_gstr1._books_outward_rows` (`gst_outward_gstr1.py:147-210`).
//!
//! A row's taxable value is a residual, never "lines under a Sales group": every nonzero line
//! once the tax ledgers' and the round-off ledgers' lines are set aside, less the voucher's own
//! PARTYLEDGERNAME line, negated once whatever the document kind (a Sales voucher and a sales-side
//! Debit Note come out positive, a Credit Note negative). A voucher left with no line but tax and
//! round-off is excluded, not a row. The population is the three base types, with no group check
//! on a note: a purchase-side Debit Note is a row too, as in the reference.
//!
//! `gst_ledgers_3b` reads the rows' months and taxable values; the GSTR-1 comparison will read
//! the rest. The reference's GSTIN and document number, and its two optional client tables that
//! override them (`party_gstin_by_ledger`, `doc_no_by_voucher_number`), come with that port.

use std::collections::BTreeSet;

use bridge_tally_primitives::TallyDate;

use crate::book::{Book, Voucher};
use crate::error::Result;
use crate::support;

/// Invoice or note, from the voucher's base type, never its type name (`:135`, `:195`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentKind {
    Invoice,
    Note,
}

/// One books outward row (the reference's `BookOutwardRow`, without its GSTIN and document
/// number). Amounts in paise, in the natural sign of the module docs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookOutwardRow {
    pub guid: String,
    pub label: String,
    pub kind: DocumentKind,
    /// `YYYY-MM` of the voucher date.
    pub month: String,
    pub taxable_paise: i128,
    pub tax_paise: i128,
    pub round_off_paise: i128,
}

/// `YYYY-MM`, as the reference's `date.strftime("%Y-%m")`.
pub(crate) fn year_month(date: &TallyDate) -> String {
    let s = date.as_str();
    format!("{}-{}", &s[0..4], &s[4..6])
}

/// The rows in scope, and the vouchers excluded because nothing but tax and round-off was left
/// (`:165-209`). `sales_tax_ledgers` and `round_off_ledgers` are client configuration.
pub fn books_outward_rows<'a>(
    book: &'a Book,
    sales_tax_ledgers: &BTreeSet<String>,
    round_off_ledgers: &BTreeSet<String>,
) -> Result<(Vec<BookOutwardRow>, Vec<&'a Voucher>)> {
    let mut rows = Vec::new();
    let mut excluded = Vec::new();
    for v in book.population()? {
        if !matches!(v.base_type.as_str(), "Sales" | "Credit Note" | "Debit Note") {
            continue;
        }
        let (mut tax, mut round_off, mut other) = (0i128, 0i128, Vec::new());
        for l in v.lines.iter().filter(|l| l.amount_paise != 0) {
            if sales_tax_ledgers.contains(&l.ledger) {
                tax += i128::from(l.amount_paise);
            } else if round_off_ledgers.contains(&l.ledger) {
                round_off += i128::from(l.amount_paise);
            } else {
                other.push(l);
            }
        }
        if other.is_empty() {
            excluded.push(v);
            continue;
        }
        // :185-190: the party's own line is left out only when the voucher names a party.
        let revenue: i128 = other
            .iter()
            .filter(|l| v.party_field.is_empty() || l.ledger != v.party_field)
            .map(|l| i128::from(l.amount_paise))
            .sum();
        rows.push(BookOutwardRow {
            guid: v.guid.clone(),
            label: support::voucher_label(v),
            kind: if v.base_type == "Sales" {
                DocumentKind::Invoice
            } else {
                DocumentKind::Note
            },
            month: year_month(&v.date),
            taxable_paise: -revenue,
            tax_paise: -tax,
            round_off_paise: -round_off,
        });
    }
    Ok((rows, excluded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{LedgerLine, VoucherStatus};

    fn voucher(guid: &str, base_type: &str, party: &str, lines: &[(&str, i64)]) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse("20250415").unwrap(),
            vtype: base_type.to_string(),
            base_type: base_type.to_string(),
            number: guid.to_string(),
            status: VoucherStatus::Regular,
            party_field: party.to_string(),
            lines: lines
                .iter()
                .map(|(l, a)| LedgerLine {
                    ledger: (*l).to_string(),
                    amount_paise: *a,
                })
                .collect(),
            ..Voucher::default()
        }
    }

    /// Each line lands where `_books_outward_rows` puts it: tax, round-off, the party's line, and
    /// the residual; a voucher of only tax and round-off is excluded; a purchase is not read.
    #[test]
    fn rows_split_tax_round_off_party_and_residual() {
        let book = Book {
            vouchers: vec![
                voucher(
                    "s1",
                    "Sales",
                    "Party",
                    &[
                        ("Party", 11_801),
                        ("Sales", -10_000),
                        ("Tax", -1_800),
                        ("Round", -1),
                    ],
                ),
                voucher(
                    "c1",
                    "Credit Note",
                    "",
                    &[("Party", -590), ("Sales", 500), ("Tax", 90)],
                ),
                voucher(
                    "x1",
                    "Debit Note",
                    "Party",
                    &[("Tax", 0), ("Round", 5), ("Tax", -5)],
                ),
                voucher(
                    "p1",
                    "Purchase",
                    "Party",
                    &[("Party", -100), ("Stock", 100)],
                ),
            ],
            ..Book::default()
        };
        let set = |n: &str| BTreeSet::from([n.to_string()]);
        let (rows, excluded) = books_outward_rows(&book, &set("Tax"), &set("Round")).unwrap();
        let got: Vec<_> = rows
            .iter()
            .map(|r| {
                (
                    r.guid.as_str(),
                    r.kind,
                    r.month.as_str(),
                    r.taxable_paise,
                    r.tax_paise,
                    r.round_off_paise,
                )
            })
            .collect();
        assert_eq!(
            got,
            [
                ("s1", DocumentKind::Invoice, "2025-04", 10_000, 1_800, 1),
                ("c1", DocumentKind::Note, "2025-04", 90, -90, 0),
            ]
        );
        assert_eq!(rows[0].label, "Sales s1 on 2025-04-15");
        assert_eq!(
            excluded.iter().map(|v| v.guid.as_str()).collect::<Vec<_>>(),
            ["x1"]
        );
    }
}
