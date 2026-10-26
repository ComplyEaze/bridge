//! The classes a statement PDF's characters fall into, counted and never read
//! differently (#1656), on PDFs built here from a content stream: one page,
//! non-embedded Courier, no encryption, invented text.
//!
//! These need the PDFium shared library, so they are ignored by default. Run
//! them with `BRIDGE_PDFIUM_LIBRARY=/path/to/libpdfium.dylib cargo test -p
//! bridge-bank-statement -- --ignored`; without the variable they fail rather
//! than pass vacuously.

use bridge_bank_statement::pdf::{engine, extract_pages, ClassCounts, PdfEngine};
use std::path::PathBuf;

fn pdfium() -> &'static PdfEngine {
    let path = std::env::var_os("BRIDGE_PDFIUM_LIBRARY")
        .map(PathBuf::from)
        .expect("BRIDGE_PDFIUM_LIBRARY must name the PDFium shared library");
    engine(&path).unwrap()
}

/// A one-page PDF 612 x 792 points, whose page content is `content`, with the
/// font /F1 (Courier) and the graphics state /GS0 (fill alpha 0). Offsets are
/// computed, so it opens without PDFium repairing it.
fn pdf_with(content: &str) -> Vec<u8> {
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R \
         /Resources << /Font << /F1 5 0 R >> /ExtGState << /GS0 6 0 R >> >> >>"
            .to_string(),
        format!(
            "<< /Length {} >>\nstream\n{content}\nendstream",
            content.len()
        ),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>".to_string(),
        "<< /Type /ExtGState /ca 0 >>".to_string(),
    ];
    let mut out = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::new();
    for (index, body) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{body}\nendobj\n", index + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for offset in &offsets {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    out
}

/// The drawn line every fixture has, so a run under test is the difference. It
/// carries a digit because a document with none is refused as unreadable.
const BASE: &str = "BT /F1 10 Tf 50 700 Td (DRAWN1) Tj ET";

fn counts(content: &str) -> ClassCounts {
    extract_pages(pdfium(), &pdf_with(content), "")
        .unwrap()
        .class_counts()
}

fn only_base() -> ClassCounts {
    counts(BASE)
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn a_plain_page_has_drawn_characters_and_no_class() {
    let base = only_base();
    assert_eq!(base.drawn, 6, "DRAWN1");
    assert_eq!(
        ClassCounts { drawn: 0, ..base },
        ClassCounts::default(),
        "{base:?}"
    );
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn render_mode_3_text_counts_as_invisible_and_not_as_drawn() {
    let run = counts(&format!(
        "{BASE}\nBT /F1 10 Tf 3 Tr 50 680 Td (HIDDEN) Tj ET"
    ));
    assert_eq!(run.invisible_mode, 6);
    assert_eq!(run.drawn, 6);
    assert_eq!(
        ClassCounts {
            invisible_mode: 0,
            ..run
        },
        only_base()
    );
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn clip_only_text_is_counted_and_is_still_drawn_as_before() {
    let run = counts(&format!(
        "{BASE}\nq BT /F1 10 Tf 7 Tr 50 660 Td (CLIPPED) Tj ET Q"
    ));
    assert_eq!(run.clip_only_mode, 7);
    assert_eq!(run.drawn, 6 + 7, "the reading is unchanged: still drawn");
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn tiny_text_is_counted() {
    let run = counts(&format!("{BASE}\nBT /F1 1 Tf 50 640 Td (TINY) Tj ET"));
    assert_eq!(run.tiny, 4);
    assert_eq!(run.drawn, 6 + 4);
    assert_eq!(
        ClassCounts {
            tiny: 0,
            drawn: 6,
            ..run
        },
        only_base()
    );
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn ten_point_text_is_not_tiny_however_it_is_flipped_turned_or_signed() {
    for (name, extra, length) in [
        (
            "flipped",
            "BT /F1 10 Tf 1 0 0 -1 50 560 Tm (FLIP9) Tj ET",
            5,
        ),
        (
            "turned",
            "BT /F1 10 Tf 0 1 -1 0 300 300 Tm (TURN8) Tj ET",
            5,
        ),
        (
            "negative size",
            "BT /F1 -10 Tf 50 520 Td (NEGSIZE6) Tj ET",
            8,
        ),
    ] {
        let run = counts(&format!("{BASE}\n{extra}"));
        assert_eq!(run.tiny, 0, "{name}: {run:?}");
        assert_eq!(run.drawn, 6 + length, "{name}: {run:?}");
    }
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn text_outside_the_page_is_counted() {
    let run = counts(&format!("{BASE}\nBT /F1 10 Tf 700 700 Td (OFF) Tj ET"));
    assert_eq!(run.off_page, 3);
    assert_eq!(
        ClassCounts {
            off_page: 0,
            drawn: 6,
            ..run
        },
        only_base()
    );
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn white_text_is_counted_and_black_text_after_it_is_not() {
    let run = counts(&format!(
        "{BASE}\n1 1 1 rg BT /F1 10 Tf 50 620 Td (WHITE) Tj ET 0 0 0 rg \
         BT /F1 10 Tf 50 600 Td (BLACK) Tj ET"
    ));
    assert_eq!(run.white_fill, 5);
    assert_eq!(run.drawn, 6 + 5 + 5);
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn text_with_a_fill_alpha_of_zero_is_counted_as_transparent() {
    let run = counts(&format!(
        "{BASE}\nq /GS0 gs BT /F1 10 Tf 50 580 Td (CLEAR) Tj ET Q"
    ));
    assert_eq!(run.transparent_fill, 5);
    assert_eq!(run.white_fill, 0);
}

#[test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
fn the_words_read_do_not_change_for_any_class_but_render_mode_3() {
    let words = |content: &str| -> Vec<String> {
        extract_pages(pdfium(), &pdf_with(content), "")
            .unwrap()
            .pages()[0]
            .iter()
            .map(|word| word.text.clone())
            .collect()
    };
    let base = words(BASE);
    assert_eq!(base, ["DRAWN1"]);
    for (name, extra, expected) in [
        (
            "clip only",
            "q BT /F1 10 Tf 7 Tr 50 660 Td (CLIPPED) Tj ET Q",
            vec!["DRAWN1", "CLIPPED"],
        ),
        (
            "tiny",
            "BT /F1 1 Tf 50 640 Td (TINY) Tj ET",
            vec!["DRAWN1", "TINY"],
        ),
        (
            "off page",
            "BT /F1 10 Tf 700 700 Td (OFF) Tj ET",
            vec!["DRAWN1", "OFF"],
        ),
        (
            "white",
            "1 1 1 rg BT /F1 10 Tf 50 620 Td (WHITE) Tj ET",
            vec!["DRAWN1", "WHITE"],
        ),
        (
            "transparent",
            "q /GS0 gs BT /F1 10 Tf 50 580 Td (CLEAR) Tj ET Q",
            vec!["DRAWN1", "CLEAR"],
        ),
        (
            "render mode 3",
            "BT /F1 10 Tf 3 Tr 50 680 Td (HIDDEN) Tj ET",
            vec!["DRAWN1"],
        ),
    ] {
        let read = words(&format!("{BASE}\n{extra}"));
        // every class but mode 3 is still read exactly as before (#1656 changes no reading)
        assert_eq!(read, expected, "{name}");
    }
}
