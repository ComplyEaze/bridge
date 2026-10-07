use super::*;

fn glyph(character: char, left: f64, bottom: f64) -> Option<Glyph> {
    Some(Glyph {
        character,
        left,
        right: left + 4.2,
        top: bottom - 5.5,
        bottom,
        size: 7.0,
    })
}

#[test]
fn words_break_at_space_gap_and_baseline_but_not_between_adjacent_glyphs() {
    let words = assemble_words([
        glyph('A', 10.0, 20.0),
        glyph('B', 14.2, 20.0),
        glyph(' ', 18.4, 20.0),
        glyph('C', 22.6, 20.0),
        // a gap wider than a tenth of the font size
        glyph('D', 27.6, 20.0),
        // a new baseline directly below
        glyph('E', 31.8, 32.0),
    ]);
    let texts: Vec<&str> = words.iter().map(|word| word.text.as_str()).collect();
    assert_eq!(texts, ["AB", "C", "D", "E"]);
    assert!((words[0].x1 - 18.4).abs() < 1e-9);
    assert!((words[0].y0 - 14.5).abs() < 1e-9);
}

fn word(text: &str) -> Word {
    Word::new(10.0, 10.0, 30.0, 19.0, text)
}

#[test]
fn a_document_with_no_figures_in_its_words_has_no_readable_text() {
    for pages in [
        vec![],
        vec![Page::new(), Page::new()],
        vec![vec![word("SCANNED"), word("WITH"), word("APP")]],
    ] {
        let refusal = require_readable_text(&pages).unwrap_err();
        assert_eq!(refusal.category, "no_readable_text");
    }
}

#[test]
fn one_figure_on_any_page_is_readable_text() {
    // a blank first page does not decide it, and a Unicode digit or fraction counts
    for figure in ["4321", "AB1", "\u{0967}", "\u{00bd}"] {
        let pages = vec![Page::new(), vec![word("TERMS"), word(figure)]];
        assert!(require_readable_text(&pages).is_ok(), "{figure}");
    }
}
