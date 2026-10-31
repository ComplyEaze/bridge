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

/// A character with no advance, where a format character usually sits.
fn unspaced(character: char, at: f64, bottom: f64) -> Option<Glyph> {
    Some(Glyph {
        character,
        left: at,
        right: at,
        top: bottom - 5.5,
        bottom,
        size: 7.0,
    })
}

#[test]
fn a_character_that_draws_nothing_is_not_read_and_the_word_goes_on() {
    let words = assemble_words([
        glyph('A', 10.0, 20.0),
        unspaced('\u{202E}', 14.2, 20.0),
        glyph('B', 14.2, 20.0),
        glyph(' ', 18.4, 20.0),
        glyph('C', 22.6, 20.0),
        unspaced('\u{200D}', 26.8, 20.0),
        glyph('D', 26.8, 20.0),
    ]);
    let texts: Vec<&str> = words.iter().map(|word| word.text.as_str()).collect();
    // The override is dropped; the joiner, which Indic spellings need, is kept.
    assert_eq!(texts, ["AB", "C\u{200D}D"]);
}

#[test]
fn a_soft_hyphen_reads_as_the_hyphen_it_prints() {
    assert_eq!(printed_character(Some('\u{ad}')), '-');
    assert_eq!(printed_character(Some('\u{2}')), '-');
    assert_eq!(printed_character(Some('\u{202E}')), '\u{202E}');
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

fn mode(mode: PdfPageTextRenderMode) -> impl FnOnce() -> Result<PdfPageTextRenderMode, ()> {
    move || Ok(mode)
}

#[test]
fn only_render_mode_3_is_hidden_and_a_generated_character_is_never_asked_its_mode() {
    let never =
        || -> Result<PdfPageTextRenderMode, ()> { panic!("mode asked of a generated character") };
    assert_eq!(classify(Ok(true), never), Ok(Origin::Generated));
    assert_eq!(
        classify(Ok(false), mode(PdfPageTextRenderMode::Invisible)),
        Ok(Origin::Hidden)
    );
    // mode 7 only adds to the clip: what is painted after it can show through
    for drawn in [
        PdfPageTextRenderMode::FilledUnstroked,
        PdfPageTextRenderMode::StrokedUnfilled,
        PdfPageTextRenderMode::FilledThenStroked,
        PdfPageTextRenderMode::FilledUnstrokedClipping,
        PdfPageTextRenderMode::StrokedUnfilledClipping,
        PdfPageTextRenderMode::FilledThenStrokedClipping,
        PdfPageTextRenderMode::InvisibleClipping,
    ] {
        assert_eq!(
            classify(Ok(false), mode(drawn)),
            Ok(Origin::Drawn),
            "{drawn:?}"
        );
    }
}

#[test]
fn a_character_that_cannot_be_classified_refuses_the_pdf() {
    let never =
        || -> Result<PdfPageTextRenderMode, ()> { panic!("mode asked after a failed origin") };
    for refused in [
        classify(Err(()), never),
        classify(Ok(false), || Err(())),
        classify(Ok(false), mode(PdfPageTextRenderMode::Unknown)),
    ] {
        assert_eq!(refused.unwrap_err().category, "unreadable_pdf");
    }
}

fn counts(origins: &[(Origin, usize)]) -> TextCounts {
    let mut counts = TextCounts::default();
    for &(origin, times) in origins {
        (0..times).for_each(|_| counts.add(origin, ExtraClasses::default()));
    }
    counts
}

#[test]
fn hidden_text_must_outnumber_drawn_text_and_generated_characters_do_not_count() {
    assert!(counts(&[(Origin::Hidden, 11), (Origin::Drawn, 10)]).mostly_hidden());
    assert!(!counts(&[(Origin::Hidden, 10), (Origin::Drawn, 10)]).mostly_hidden());
    assert!(!counts(&[]).mostly_hidden());
    // PDFium's own spaces and line breaks are neither
    assert!(counts(&[
        (Origin::Hidden, 2),
        (Origin::Drawn, 1),
        (Origin::Generated, 5)
    ])
    .mostly_hidden());
    assert!(!counts(&[
        (Origin::Hidden, 1),
        (Origin::Drawn, 1),
        (Origin::Generated, 5)
    ])
    .mostly_hidden());
}

const PAGE: (f64, f64) = (612.0, 792.0);
const INSIDE: Option<(f64, f64, f64, f64)> = Some((50.0, 56.0, 100.0, 110.0));

fn classes(
    mode: Option<PdfPageTextRenderMode>,
    size: f64,
    bounds: Option<(f64, f64, f64, f64)>,
    fill: Option<(u8, u8, u8, u8)>,
) -> ExtraClasses {
    extra_classes(mode, size, bounds, PAGE, fill)
}

#[test]
fn an_ordinary_black_character_inside_the_page_has_no_extra_class() {
    let drawn = Some(PdfPageTextRenderMode::FilledUnstroked);
    assert_eq!(
        classes(drawn, 7.0, INSIDE, Some((0, 0, 0, 255))),
        ExtraClasses::default()
    );
    // what PDFium did not report counts nothing
    assert_eq!(classes(None, 7.0, None, None), ExtraClasses::default());
}

#[test]
fn only_the_clip_only_mode_is_a_clip_only_character() {
    for (mode, expected) in [
        (PdfPageTextRenderMode::InvisibleClipping, true),
        (PdfPageTextRenderMode::Invisible, false),
        (PdfPageTextRenderMode::FilledUnstrokedClipping, false),
        (PdfPageTextRenderMode::FilledUnstroked, false),
        (PdfPageTextRenderMode::StrokedUnfilled, false),
    ] {
        assert_eq!(
            classes(Some(mode), 7.0, INSIDE, None).clip_only_mode,
            expected,
            "{mode:?}"
        );
    }
}

#[test]
fn a_font_size_under_the_threshold_is_tiny_and_the_threshold_itself_is_not() {
    let tiny = |size| classes(None, size, INSIDE, None).tiny;
    assert!(tiny(1.0) && tiny(TINY_POINTS - 0.001) && tiny(0.0));
    assert!(!tiny(TINY_POINTS) && !tiny(7.0));
    // a size PDFium reports as not a number is not counted
    assert!(!tiny(f64::NAN));
}

#[test]
fn the_effective_size_ignores_sign_and_rotation_and_an_unread_matrix_is_not_a_size() {
    // upright, flipped, turned a quarter, turned the other way: all 10 pt
    for row in [(0.0, 1.0), (0.0, -1.0), (-1.0, 0.0), (1.0, 0.0)] {
        assert_eq!(effective_size(10.0, Some(row)), 10.0, "{row:?}");
    }
    // a negative font size is its magnitude; a matrix scaled by 2 doubles it
    assert_eq!(effective_size(-10.0, Some((0.0, 1.0))), 10.0);
    assert_eq!(effective_size(1.0, Some((0.0, 2.0))), 2.0);
    // a zero font size is a size of zero (a character PDFium reports at 0 is tiny)
    assert_eq!(effective_size(0.0, Some((0.0, 1.0))), 0.0);
    assert!(effective_size(10.0, None).is_nan());
}

#[test]
fn a_box_wholly_outside_the_crop_box_is_off_the_page_and_one_touching_it_is_not() {
    let off = |bounds| classes(None, 7.0, Some(bounds), None).off_page;
    // right of, left of, above and below the page
    assert!(off((612.0, 618.0, 100.0, 110.0)));
    assert!(off((700.0, 706.0, 100.0, 110.0)));
    assert!(off((-20.0, 0.0, 100.0, 110.0)));
    assert!(off((50.0, 56.0, -20.0, 0.0)));
    assert!(off((50.0, 56.0, 792.0, 800.0)));
    // partly inside is on the page
    assert!(!off((-3.0, 3.0, 100.0, 110.0)));
    assert!(!off((608.0, 614.0, 100.0, 110.0)));
    assert!(!off((50.0, 56.0, 785.0, 795.0)));
}

#[test]
fn white_means_opaque_white_and_transparent_means_alpha_zero() {
    let fill = |color| classes(None, 7.0, INSIDE, Some(color));
    assert!(fill((255, 255, 255, 255)).white_fill);
    assert!(fill((255, 255, 255, 1)).white_fill);
    assert!(!fill((255, 255, 254, 255)).white_fill);
    assert!(!fill((0, 0, 0, 255)).white_fill);
    // white with no alpha is the transparent class, not the white one
    let clear = fill((255, 255, 255, 0));
    assert!(clear.transparent_fill && !clear.white_fill);
    assert!(fill((0, 0, 0, 0)).transparent_fill);
    assert!(!fill((0, 0, 0, 255)).transparent_fill);
}

#[test]
fn the_counts_add_each_class_apart_and_leave_generated_characters_out() {
    let mut counts = TextCounts::default();
    counts.add(Origin::Drawn, ExtraClasses::default());
    counts.add(
        Origin::Drawn,
        ExtraClasses {
            tiny: true,
            white_fill: true,
            ..ExtraClasses::default()
        },
    );
    counts.add(
        Origin::Hidden,
        ExtraClasses {
            off_page: true,
            ..ExtraClasses::default()
        },
    );
    counts.add(
        Origin::Generated,
        ExtraClasses {
            tiny: true,
            ..ExtraClasses::default()
        },
    );
    assert_eq!(
        counts.class_counts(),
        ClassCounts {
            drawn: 2,
            invisible_mode: 1,
            clip_only_mode: 0,
            tiny: 1,
            off_page: 1,
            white_fill: 1,
            transparent_fill: 0,
        }
    );
}
