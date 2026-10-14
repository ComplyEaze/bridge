use super::*;
use crate::bbox::read_pages;
use crate::geometry::Word;

fn refusal() -> Refusal {
    Refusal::new(
        "account_not_in_statement",
        "the label's digits are not on the statement",
    )
}

/// What a scanner app draws over its hidden OCR layer: a stamp, no row.
fn stamp_only() -> Vec<Page> {
    vec![vec![
        Word::new(28.0, 20.0, 70.0, 27.0, "SCANNED"),
        Word::new(75.0, 20.0, 117.0, 27.0, "12/08/2026"),
    ]]
}

fn hdfc_statement() -> Vec<Page> {
    let xml = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/hdfc-synthetic.pdftotext.xml"),
    )
    .unwrap();
    read_pages(&xml)
}

#[test]
fn a_mostly_hidden_pdf_whose_drawn_text_has_no_row_is_refused_as_no_readable_text() {
    // SBI and HDFC read no row from a stamp; Union and Bank of Baroda refuse it
    for bank in [Bank::Sbi, Bank::Hdfc, Bank::Ubi, Bank::Bob] {
        let named = name_hidden_text(refusal(), &stamp_only(), true, bank);
        assert_eq!(
            (named.category, named.row),
            ("no_readable_text", None),
            "{bank:?}"
        );
    }
}

#[test]
fn a_refusal_keeps_its_name_when_the_drawn_text_has_rows_or_little_is_hidden() {
    let statement = hdfc_statement();
    assert!(!parse_statement(&statement, Bank::Hdfc).unwrap().is_empty());
    // a statement with a scanned page appended: a request fault is named as itself
    assert_eq!(
        name_hidden_text(refusal(), &statement, true, Bank::Hdfc),
        refusal()
    );
    assert_eq!(
        name_hidden_text(refusal(), &stamp_only(), false, Bank::Hdfc),
        refusal()
    );
}
