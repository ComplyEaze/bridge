//! The statement-period line at the top of a continuation page must not become part of the previous
//! page's last row, and a split row's tail must still reach it. Words are constructed, not captured: no
//! client text, two made-up pages at the geometry of the public sanitised capture.
//!
//! Found on a real HDFC print (6 Oct 2026): `parse_bank_statement` refused with `unparseable_date` at the
//! last row of page 1 (and of page 2), because that row's date cell read `dd/mm/yy` followed by a four-letter
//! word and a colon. Every page opens with `Statement of account` and, a little below it in a smaller font,
//! `From : <date> To : <date>`. The capture puts the two 2.4 pt apart, inside the 3 pt that `geometry::lines`
//! groups into one visual line, so the period line is skipped with the anchor line. A reader that puts the
//! second line 3 pt or more down makes it a line of its own below the anchor, which was read as a continuation
//! of the open row: `From :` in the date cell, both dates in the narration. That the real print's reader does
//! this is inferred from the shape of the refusal and the capture's geometry, not measured on its words.

mod common;

use bridge_bank_statement::bank::Bank;
use bridge_bank_statement::geometry::Page;
use bridge_bank_statement::parse::{parse_pages, Row};
use common::*;

const FIRST_NARRATION: &str = "0000000001-TPT-RENT-ZEPHYR";

/// Page 1: the column header, one row, the footer.
fn first_page() -> Page {
    let first: &[Cell] = &[
        (33.7, 62.1, "01/08/26"),
        (72.0, 200.0, FIRST_NARRATION),
        (282.0, 350.0, "0000000000000001"),
        (362.5, 391.0, "01/08/26"),
        (482.0, 540.0, "10,000.00"),
        (562.0, 620.0, "11,000.00"),
    ];
    page(&[
        (100.0, hdfc_header_row()),
        (120.0, first),
        (170.0, hdfc_footer()),
    ])
}

/// What follows the period line on page 2: an optional tail of page 1's last narration, then a row.
struct Below<'a> {
    /// a narration-only line, the wrap of the previous page's last row
    tail: Option<&'a str>,
    /// a left-margin word on the line after page 2's row
    margin_stray: Option<&'a str>,
}

/// `From : <date> To : <date>`, at the capture's x positions; the dates sit in the narration column.
const PERIOD: &[Cell] = &[
    (34.0, 51.4, "From"),
    (55.4, 57.6, ":"),
    (61.6, 98.0, "02/08/2026"),
    (154.0, 162.9, "To"),
    (166.9, 169.1, ":"),
    (173.1, 209.6, "03/08/2026"),
];

/// The same line with each label glued to its colon, so the `From` and `To` words the anchor needs are absent.
const PERIOD_GLUED: &[Cell] = &[
    (34.0, 57.6, "From:"),
    (61.6, 98.0, "02/08/2026"),
    (154.0, 169.1, "To:"),
    (173.1, 209.6, "03/08/2026"),
];

/// Page 2: `Statement of account`, then `period` `gap` points below it, then `below`.
fn second_page(gap: f64, period: &[Cell], below: &Below<'_>) -> Page {
    let anchor: &[Cell] = &[
        (340.2, 388.2, "Statement"),
        (391.2, 401.2, "of"),
        (404.2, 441.5, "account"),
    ];
    let row: &[Cell] = &[
        (33.7, 62.1, "02/08/26"),
        (72.0, 200.0, "NEFT DR-ZZZZ0000001-ACME"),
        (282.0, 350.0, "0000000000000002"),
        (362.5, 391.0, "02/08/26"),
        (402.0, 460.0, "2,500.00"),
        (562.0, 620.0, "8,500.00"),
    ];
    let tail_cells: Vec<Cell> = below
        .tail
        .map(|text| (74.0, 100.7, text))
        .into_iter()
        .collect();
    let stray_cells: Vec<Cell> = below
        .margin_stray
        .map(|text| (5.0, 30.0, text))
        .into_iter()
        .collect();
    let mut lines: Vec<ConstructedLine> = vec![(213.9, anchor), (213.9 + gap, period)];
    if below.tail.is_some() {
        lines.push((232.4, &tail_cells));
    }
    lines.push((249.6, row));
    if below.margin_stray.is_some() {
        lines.push((262.0, &stray_cells));
    }
    page(&lines)
}

fn rows(gap: f64, below: &Below<'_>) -> Vec<Row> {
    pair(gap, PERIOD, below)
}

fn pair(gap: f64, period: &[Cell], below: &Below<'_>) -> Vec<Row> {
    let parsed = parse_pages(&[first_page(), second_page(gap, period, below)], Bank::Hdfc);
    assert_eq!(parsed.len(), 2, "one row per page");
    parsed
}

const NOTHING_BELOW: Below<'static> = Below {
    tail: None,
    margin_stray: None,
};

#[test]
fn a_period_line_below_the_anchor_line_is_not_part_of_the_last_row() {
    // 3.266 pt is what PDFium measured on the real print (From at y 214.679, Statement at 211.413): its own
    // visual line under the anchor, 0.27 pt past the grouping. 5 pt is the same shape with room to spare.
    for gap in [3.266, 5.0] {
        assert_period_line_skipped(gap);
    }
}

fn assert_period_line_skipped(gap: f64) {
    let parsed = rows(gap, &NOTHING_BELOW);
    assert_eq!(
        parsed[0].get("date"),
        "01/08/26",
        "page 1's last row keeps its own date cell, gap {gap}"
    );
    assert_eq!(
        parsed[0].get("narr"),
        FIRST_NARRATION,
        "and its own narration, no period dates in it, gap {gap}"
    );
    assert_eq!(parsed[0].get("narr_spaced"), FIRST_NARRATION);
    assert_eq!(parsed[1].get("date"), "02/08/26");
}

#[test]
fn the_same_period_line_inside_the_anchor_line_is_skipped() {
    // Control: identical words, 2.4 pt below the anchor line as the capture has them, so it shares that
    // line. This is the geometry poppler gave on the real print, and it parses.
    let parsed = rows(2.4, &NOTHING_BELOW);
    assert_eq!(parsed[0].get("date"), "01/08/26");
    assert_eq!(parsed[0].get("narr"), FIRST_NARRATION);
    assert_eq!(parsed[1].get("date"), "02/08/26");
}

#[test]
fn a_split_rows_tail_after_the_period_line_still_reaches_its_row() {
    // The public capture's page 2 has a narration-only line between the period line and the first row: the
    // wrap of the previous page's last narration. It must be read whichever way the lines group.
    let below = Below {
        tail: Some("ZETA"),
        margin_stray: None,
    };
    // -5.0 puts the period line above the anchor line: the anchor's own words then sit below the top, in
    // columns the row never reads from a continuation line.
    for gap in [-5.0, 2.4, 3.266, 5.0] {
        let parsed = rows(gap, &below);
        assert_eq!(parsed[0].get("date"), "01/08/26", "gap {gap}");
        assert_eq!(
            parsed[0].get("narr_spaced"),
            "0000000001-TPT-RENT-ZEPHYR ZETA",
            "gap {gap}"
        );
        assert_eq!(Bank::Hdfc.party(&parsed[0]), "ZEPHYR ZETA", "gap {gap}");
        assert_eq!(
            parsed[1].get("narr"),
            "NEFT DR-ZZZZ0000001-ACME",
            "gap {gap}"
        );
    }
}

#[test]
fn a_date_column_word_below_a_row_still_reaches_its_date_cell_and_the_date_refuses() {
    // Guards against fixing this by row-scoping the date column: the fix is the page's top anchor, so a stray
    // word under a row stays in that row's date cell, where the date parse refuses it loudly. It passes on
    // master too; it is a guard, not the regression check.
    let below = Below {
        tail: None,
        margin_stray: Some("Zzzz"),
    };
    let parsed = rows(2.4, &below);
    assert_eq!(parsed[1].get("date"), "02/08/26Zzzz");
    assert_eq!(Bank::Hdfc.parse_date(parsed[1].get("date")), None);
}

#[test]
fn a_period_line_the_anchor_cannot_read_still_refuses_loudly() {
    // Residual of reading the period line by its `From` and `To` words: glued to their colons they do not
    // match, the anchor falls back to `Statement of account`, and the line below it is a continuation again.
    // That must fail as it did before, in the date cell (and so as a refusal), never as a silent merge.
    let parsed = pair(5.0, PERIOD_GLUED, &NOTHING_BELOW);
    assert_eq!(parsed[0].get("date"), "01/08/26From:");
    assert_eq!(Bank::Hdfc.parse_date(parsed[0].get("date")), None);
}
