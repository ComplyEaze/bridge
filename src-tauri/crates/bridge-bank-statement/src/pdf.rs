//! Word boxes from a password-protected PDF, through a bundled PDFium.
//!
//! Replaces the `pdftotext -bbox-layout -upw/-opw` that the removed Python tool used. Two differences
//! matter:
//!
//! * **One password attempt covers both passwords.** PDFium's standard security
//!   handler checks a supplied password as the user password and then as the
//!   owner password, so an owner-password-only statement (which `pdftotext
//!   -upw` rejects) opens without a second attempt. The owner-password-only
//!   fixture is the test of that claim.
//! * **Words are assembled here from characters.** PDFium reports characters
//!   with boxes; `pdftotext` reported words. A word ends at whitespace (PDFium's
//!   own generated spaces included), at a change of baseline, or at a horizontal
//!   gap wider than a tenth of the font size — poppler's `minWordBreakSpace`.
//!   Character boxes are PDFium's *loose* boxes (font ascent to descent, advance
//!   width), the nearest equivalent of poppler's word boxes; tight glyph boxes
//!   would move a fragment's right edge by up to a glyph's side bearing, which
//!   is the quantity the wrap heuristic reads.
//!
//!   Measured on the synthetic fixtures: every horizontal edge agrees with
//!   `pdftotext` to within 0.01pt. Vertical edges are shifted by a constant per
//!   font — 1.43pt on 7pt Courier — because PDFium substitutes a system face for
//!   a non-embedded base-14 font and takes its ascent from that face, where
//!   poppler uses the standard AFM metrics. A shift that is uniform within a
//!   font changes neither line grouping nor anchors; a line mixing two
//!   non-embedded fonts could group differently from `pdftotext`. What would
//!   catch that depends on what the difference reaches. The balance replay
//!   proves amounts, and for SBI and HDFC so do the printed control totals
//!   (Union Bank prints none). The date parse refuses a date cell holding
//!   anything but one well-formed date, the account-number line has a refusal
//!   of its own, and so does Union Bank's page footer. Nothing proves narration
//!   or reference text: words a grouping difference moves into a narration
//!   change the row's transaction identity (which hashes the narration); words
//!   moved into a narration or a reference cell can change the party or
//!   reference read from the row; and no amount check sees either. In #1310
//!   the statement-period line was refused only because two of its words fell
//!   in the date column; the same line's dates entering the narration went
//!   unseen. Embedded fonts were not measured.
//!
//! Text in render mode 3 (`Invisible`), which a scanner app writes as its OCR
//! layer under the page image, is not read: such a character is left out, as a
//! character that draws nothing is. PDFium's own generated spaces and line
//! breaks are kept, and their render mode is never asked (they have no text
//! object). Mode 7 (`InvisibleClipping`) is read as before: content painted
//! afterwards can show through its glyph shapes. Only render mode 3 is left
//! out of the reading. Other ways a character can be kept from being seen are
//! counted per class (see [`ClassCounts`]) and change nothing that is read (#1656).
//!
//! Coordinates are converted to the top-left page space of `pdftotext`,
//! relative to the crop box (or the media box when a page has no crop box).
//! A rotated page is refused rather than guessed at: every column bound assumes
//! the page is upright.
//!
//! The password is borrowed, never stored, and never formatted into a message.
//! Residual, stated rather than assumed: `pdfium-render` copies it into a
//! `CString` for the FFI call and PDFium keeps its own copy inside the open
//! document; neither copy is zeroised.

use crate::geometry::{Page, Word};
use crate::refusal::Refusal;
use crate::text::is_space;
use bridge_tally_primitives::text::draws_nothing;
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// A statement larger than this is refused before PDFium parses it.
pub const MAX_PDF_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_PAGES: i32 = 500;
pub const MAX_CHARS_PER_PAGE: usize = 250_000;

/// PDFium, bound once per process.
///
/// `pdfium-render` keeps the library bindings in a process-wide cell, so a
/// second library path cannot be bound after the first; asking for one is
/// refused instead of silently using the first.
pub struct PdfEngine {
    pdfium: Pdfium,
}

impl std::fmt::Debug for PdfEngine {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("PdfEngine")
    }
}

static ENGINE: OnceLock<(PathBuf, Result<PdfEngine, String>)> = OnceLock::new();

/// The PDFium library's platform file name (`libpdfium.dylib`, `pdfium.dll`,
/// `libpdfium.so`) inside `directory`.
pub fn platform_library_path(directory: &Path) -> PathBuf {
    Pdfium::pdfium_platform_library_name_at_path(directory)
}

/// The shared engine, loading PDFium from `library` on first use.
pub fn engine(library: &Path) -> Result<&'static PdfEngine, Refusal> {
    let (bound_path, bound) = ENGINE.get_or_init(|| {
        let loaded = Pdfium::bind_to_library(library)
            .map(|bindings| PdfEngine {
                pdfium: Pdfium::new(bindings),
            })
            .map_err(|error| format!("{error:?}"));
        (library.to_path_buf(), loaded)
    });
    if bound_path != library {
        return Err(Refusal::new(
            "pdf_engine_unavailable",
            "PDFium is already bound from a different library path in this process",
        ));
    }
    bound.as_ref().map_err(|error| {
        Refusal::new(
            "pdf_engine_unavailable",
            format!("the bundled PDFium library could not be loaded ({error})"),
        )
    })
}

struct Glyph {
    character: char,
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    size: f64,
}

fn close_word(glyphs: &mut Vec<Glyph>, words: &mut Page) {
    if glyphs.is_empty() {
        return;
    }
    let mut word = Word::new(
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
        String::new(),
    );
    for glyph in glyphs.iter() {
        word.x0 = word.x0.min(glyph.left);
        word.x1 = word.x1.max(glyph.right);
        word.y0 = word.y0.min(glyph.top);
        word.y1 = word.y1.max(glyph.bottom);
        word.text.push(glyph.character);
    }
    words.push(word);
    glyphs.clear();
}

/// Split a page's characters, in PDFium's text order, into words.
fn assemble_words(glyphs: impl IntoIterator<Item = Option<Glyph>>) -> Page {
    let mut words = Page::new();
    let mut current: Vec<Glyph> = Vec::new();
    for glyph in glyphs {
        let Some(glyph) = glyph else {
            close_word(&mut current, &mut words);
            continue;
        };
        if is_space(glyph.character) || glyph.character.is_control() {
            close_word(&mut current, &mut words);
            continue;
        }
        // It draws nothing a reader sees, so it is not read, and the word around
        // it goes on (the gap rule below still splits it if the font gave the
        // character an advance).
        if draws_nothing(glyph.character) {
            continue;
        }
        if let Some(previous) = current.last() {
            let size = previous.size.max(glyph.size).max(1.0);
            let gap = glyph.left - previous.right;
            let new_baseline = (glyph.bottom - previous.bottom).abs() > 0.5 * size;
            if new_baseline || gap > 0.1 * size || gap < -0.5 * size {
                close_word(&mut current, &mut words);
            }
        }
        current.push(glyph);
    }
    close_word(&mut current, &mut words);
    words
}

/// The character a glyph printed.
///
/// PDFium reports a hyphen that ends a line as U+0002, its hyphenation marker,
/// with the hyphen's own geometry. Statement narrations wrap at hyphens
/// constantly (`NEFT DR-ZZZZ0000001-` / `ACME…`), and every HDFC party boundary
/// is a hyphen, so the marker is read back as the hyphen that was printed.
/// Residual: a line-ending soft hyphen (U+00AD) arrives the same way and is
/// also read as `-`. A soft hyphen reported as itself is read as `-` too: it is
/// what a WinAnsi font's byte 0xAD prints, a hyphen, though it is a character
/// that otherwise draws nothing ([`draws_nothing`]).
fn printed_character(reported: Option<char>) -> char {
    match reported {
        Some('\u{2}' | '\u{ad}') => '-',
        Some(character) => character,
        None => '\u{fffd}',
    }
}

fn unreadable(message: &str) -> Refusal {
    Refusal::new("unreadable_pdf", message)
}

/// What a character PDFium reports is, for reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Origin {
    /// A space or line break PDFium inserted; it splits words.
    Generated,
    /// Text in any render mode but 3.
    Drawn,
    /// Text in render mode 3, which draws nothing.
    Hidden,
}

/// Classify one character. The render mode is asked only of a character that
/// is not generated: a generated one has no text object, so asking fails.
/// Either PDFium answer failing, or an `Unknown` mode, refuses: an unclassified
/// character could be hidden, and dropping it could lose a drawn figure.
fn classify<E>(
    generated: Result<bool, E>,
    mode: impl FnOnce() -> Result<PdfPageTextRenderMode, E>,
) -> Result<Origin, Refusal> {
    match generated {
        Err(_) => Err(unreadable("a character's origin could not be read")),
        Ok(true) => Ok(Origin::Generated),
        Ok(false) => match mode() {
            Ok(PdfPageTextRenderMode::Invisible) => Ok(Origin::Hidden),
            Ok(PdfPageTextRenderMode::Unknown) | Err(_) => {
                Err(unreadable("a character's render mode could not be read"))
            }
            Ok(_) => Ok(Origin::Drawn),
        },
    }
}

/// A character whose effective font size (see [`effective_size`]) is below this
/// many points is counted as tiny. A count threshold only: nothing is refused on it (#1656).
pub const TINY_POINTS: f64 = 2.0;

/// The classes a document's characters fall into besides drawn and
/// render-mode-3, counted and never read differently (#1656). A character can
/// fall into several, and each count is its own. Generated characters are not
/// counted. A class whose value PDFium could not report counts nothing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ClassCounts {
    /// Characters drawn (any render mode but 3), the denominator.
    pub drawn: usize,
    /// Characters in render mode 3, which are left out of the reading.
    pub invisible_mode: usize,
    /// Characters in the mode that only adds the glyph to the clip.
    pub clip_only_mode: usize,
    /// Characters whose effective font size is under [`TINY_POINTS`].
    pub tiny: usize,
    /// Characters whose box lies wholly outside the page's crop box.
    pub off_page: usize,
    /// Characters filled white (all three channels 255) with an alpha above 0.
    pub white_fill: usize,
    /// Characters filled with alpha 0.
    pub transparent_fill: usize,
}

/// What one character is, besides drawn or invisible, from what PDFium reports.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct ExtraClasses {
    clip_only_mode: bool,
    tiny: bool,
    off_page: bool,
    white_fill: bool,
    transparent_fill: bool,
}

/// The extra classes of one character that is not generated. `page` is the
/// crop box's (width, height) and `bounds` the character's (left, right, top,
/// bottom) in the same top-left space; either missing, like an unreadable fill,
/// leaves its class out.
fn extra_classes(
    mode: Option<PdfPageTextRenderMode>,
    size: f64,
    bounds: Option<(f64, f64, f64, f64)>,
    page: (f64, f64),
    fill: Option<(u8, u8, u8, u8)>,
) -> ExtraClasses {
    ExtraClasses {
        clip_only_mode: mode == Some(PdfPageTextRenderMode::InvisibleClipping),
        tiny: size.is_finite() && size < TINY_POINTS,
        off_page: bounds.is_some_and(|(left, right, top, bottom)| {
            right <= 0.0 || left >= page.0 || bottom <= 0.0 || top >= page.1
        }),
        white_fill: fill.is_some_and(|(red, green, blue, alpha)| {
            red == 255 && green == 255 && blue == 255 && alpha > 0
        }),
        transparent_fill: fill.is_some_and(|(_, _, _, alpha)| alpha == 0),
    }
}

/// A character's size in points whatever the sign of the font size or of the
/// text matrix and whatever its rotation: the font size's magnitude times the
/// length of the matrix's second row `(c, d)`. NaN when the matrix could not be
/// read, which is never tiny: a flipped, turned or negatively sized 10 pt
/// character is 10 pt.
fn effective_size(unscaled: f32, row: Option<(f32, f32)>) -> f64 {
    row.map_or(f64::NAN, |(c, d)| {
        f64::from(unscaled).abs() * f64::from(c).hypot(f64::from(d))
    })
}

/// The document's own characters, hidden and drawn; generated ones are not
/// counted.
#[derive(Debug, Default)]
struct TextCounts {
    hidden: usize,
    drawn: usize,
    classes: ClassCounts,
}

impl TextCounts {
    fn add(&mut self, origin: Origin, extra: ExtraClasses) {
        match origin {
            Origin::Hidden => self.hidden += 1,
            Origin::Drawn => self.drawn += 1,
            Origin::Generated => return,
        }
        self.classes.clip_only_mode += usize::from(extra.clip_only_mode);
        self.classes.tiny += usize::from(extra.tiny);
        self.classes.off_page += usize::from(extra.off_page);
        self.classes.white_fill += usize::from(extra.white_fill);
        self.classes.transparent_fill += usize::from(extra.transparent_fill);
    }

    fn class_counts(&self) -> ClassCounts {
        ClassCounts {
            drawn: self.drawn,
            invisible_mode: self.hidden,
            ..self.classes
        }
    }

    /// More are hidden than drawn: an OCR layer under page images, whatever a
    /// scanner app stamps on top.
    fn mostly_hidden(&self) -> bool {
        self.hidden > self.drawn
    }
}

/// The words of a statement PDF, and whether most of its text was hidden.
#[derive(Debug)]
pub struct StatementText {
    pages: Vec<Page>,
    mostly_hidden: bool,
    class_counts: ClassCounts,
}

impl StatementText {
    /// Every page's words. A statement is prepared from them with
    /// [`crate::pipeline::prepare_text`], which names a hidden text layer.
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    /// More of the document's own characters were hidden than drawn.
    pub fn mostly_hidden(&self) -> bool {
        self.mostly_hidden
    }

    /// The document's characters counted by class. Counts only: nothing read
    /// depends on them.
    pub fn class_counts(&self) -> ClassCounts {
        self.class_counts
    }
}

/// Every page's words, top-left coordinates.
pub fn extract_pages(
    engine: &PdfEngine,
    pdf: &[u8],
    password: &str,
) -> Result<StatementText, Refusal> {
    if pdf.len() > MAX_PDF_BYTES {
        return Err(unreadable(
            "the statement is larger than the accepted PDF size",
        ));
    }
    if password.contains('\0') {
        // pdfium-render panics converting such a password to a C string
        return Err(Refusal::new(
            "unusable_password",
            "the password contains a NUL character, which a PDF password cannot carry",
        ));
    }
    let document = engine
        .pdfium
        .load_pdf_from_byte_slice(pdf, Some(password))
        .map_err(|error| match error {
            PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::PasswordError) => {
                unreadable(
                    "cannot open the statement: the password is not its user or owner password",
                )
            }
            _ => unreadable("cannot open the statement: it is not a PDF PDFium can read"),
        })?;
    let pages = document.pages();
    if pages.len() > MAX_PAGES {
        return Err(unreadable(
            "the statement has more pages than the accepted limit",
        ));
    }
    let mut out = Vec::new();
    let mut counts = TextCounts::default();
    for page in pages.iter() {
        let rotation = page
            .rotation()
            .map_err(|_| unreadable("a page's rotation could not be read"))?;
        if rotation != PdfPageRenderRotation::None {
            return Err(Refusal::new(
                "unsupported_page_rotation",
                "a page is rotated; the column geometry assumes an upright page",
            ));
        }
        let boundary = page
            .boundaries()
            .crop()
            .or_else(|_| page.boundaries().media())
            .map_err(|_| unreadable("a page has no readable crop or media box"))?;
        let origin_left = f64::from(boundary.bounds.left().value);
        let origin_top = f64::from(boundary.bounds.top().value);
        let page_size = (
            f64::from(boundary.bounds.right().value) - origin_left,
            origin_top - f64::from(boundary.bounds.bottom().value),
        );
        let text = page
            .text()
            .map_err(|_| unreadable("a page's text could not be extracted"))?;
        let chars = text.chars();
        if chars.len() > MAX_CHARS_PER_PAGE {
            return Err(unreadable(
                "a page carries more characters than the accepted limit",
            ));
        }
        let mut glyphs = Vec::with_capacity(chars.len());
        for character in chars.iter() {
            let origin = classify(character.is_generated(), || character.render_mode())?;
            let bounds = character.loose_bounds().ok().map(|bounds| {
                (
                    f64::from(bounds.left().value) - origin_left,
                    f64::from(bounds.right().value) - origin_left,
                    origin_top - f64::from(bounds.top().value),
                    origin_top - f64::from(bounds.bottom().value),
                )
            });
            let scaled = f64::from(character.scaled_font_size().value);
            let extra = if origin == Origin::Generated {
                ExtraClasses::default()
            } else {
                extra_classes(
                    character.render_mode().ok(),
                    effective_size(
                        character.unscaled_font_size().value,
                        character
                            .matrix()
                            .ok()
                            .map(|matrix| (matrix.c(), matrix.d())),
                    ),
                    bounds,
                    page_size,
                    character
                        .fill_color()
                        .ok()
                        .map(|color| (color.red(), color.green(), color.blue(), color.alpha())),
                )
            };
            counts.add(origin, extra);
            if origin == Origin::Hidden {
                // left out, so the word around it goes on
                continue;
            }
            glyphs.push(bounds.map(|(left, right, top, bottom)| Glyph {
                character: printed_character(character.unicode_char()),
                left,
                right,
                top,
                bottom,
                size: scaled,
            }));
        }
        out.push(assemble_words(glyphs));
    }
    require_readable_text(&out)?;
    Ok(StatementText {
        pages: out,
        mostly_hidden: counts.mostly_hidden(),
        class_counts: counts.class_counts(),
    })
}

/// Refuse a document whose words carry no numeric character at all: a scan, a
/// printout with glyphs drawn as paths, a scan with only a scanner app's stamp,
/// or a scan whose only text is a hidden OCR layer (not read). Such a document
/// would otherwise be refused further on under a name that points at the wrong
/// fix (`no_account_number_line` or `page_sequence_unproven`).
///
/// Judged on the whole document, so an image-only terms page inside a bank PDF
/// does not trigger it. A numeric character (Unicode Nd, Nl or No, a superset of
/// what the account check's `\d` reads) is the test because every statement that
/// gets past `parse::require_account_match` has one (its account-number line), so
/// this renames a refusal that was certain for the words read. Since hidden text
/// is not read, a PDF whose figures exist only as hidden text is refused here
/// although it was read before; no held bank statement has a hidden character.
/// A scan that also draws a digit (a stamp with a date, page numbers) passes
/// here; when its parse then refuses, [`crate::pipeline::prepare_text`] names it.
fn require_readable_text(pages: &[Page]) -> Result<(), Refusal> {
    let has_figures = pages
        .iter()
        .flatten()
        .any(|word| word.text.chars().any(char::is_numeric));
    if has_figures {
        return Ok(());
    }
    Err(no_readable_text())
}

pub(crate) fn no_readable_text() -> Refusal {
    Refusal::new(
        "no_readable_text",
        "no figures could be read from the PDF: a scan, a printed copy or an image has none, \
         and a scan's OCR layer in render mode 3 is not read",
    )
}

#[cfg(test)]
#[path = "pdf_tests.rs"]
mod tests;
