//! A voucher's own text (its reference or narration) in a one-voucher dialog
//! (#1535): on one quoted line when it fits, as today, or else in numbered
//! parts, each quoted on its own line and split only at a space, so that the
//! parts joined in order are the value exactly. A space at a split stays
//! inside the earlier part's quotes, where the person can see it.

/// The most characters a whole value may take: no dialog shows more (the
/// one-voucher text's cap), so a longer one is refused before it is split.
const MOST_CHARS: usize = 1_600;
/// A run of this many dash-like characters reads like a dialog's dashed line.
const DASH_RUN: usize = 4;

/// Why a value cannot be shown in parts.
#[derive(Debug, PartialEq, Eq)]
pub(in super::super) enum VoucherTextRefusal {
    /// No space splits it into parts that each fit a line.
    DoesNotFit,
    /// It holds a run of dashes or a number in brackets, which a value shown
    /// in parts may not: the dialog's dashed line and its part labels.
    Layout,
}

/// A value as a dialog shows it: one part, or the parts that, joined in
/// order, are the value exactly. Only [`VoucherText::of`] builds one.
#[derive(Debug, PartialEq, Eq)]
pub(in super::super) struct VoucherText<'a> {
    parts: Vec<&'a str>,
}

impl<'a> VoucherText<'a> {
    /// `text` on one line when its quoted form takes at most `one_line`
    /// characters; else in parts, part `k` (from 1) taking at most
    /// `part_width(k)` characters quoted, each but the last ending in a space.
    pub(in super::super) fn of(
        text: &'a str,
        one_line: usize,
        part_width: impl Fn(usize) -> usize,
    ) -> Result<Self, VoucherTextRefusal> {
        if quoted_chars(text) <= one_line {
            return Ok(Self { parts: vec![text] });
        }
        if text.chars().count() > MOST_CHARS {
            return Err(VoucherTextRefusal::DoesNotFit);
        }
        if has_dash_run(text) || has_bracketed_number(text) {
            return Err(VoucherTextRefusal::Layout);
        }
        let mut parts = Vec::new();
        let mut rest = text;
        while !rest.is_empty() {
            let width = part_width(parts.len() + 1);
            if quoted_chars(rest) <= width {
                parts.push(rest);
                break;
            }
            let end = rest
                .char_indices()
                .filter(|(_, character)| *character == ' ')
                .map(|(at, _)| at + 1)
                .take_while(|&end| quoted_chars(&rest[..end]) <= width)
                .last()
                .ok_or(VoucherTextRefusal::DoesNotFit)?;
            parts.push(&rest[..end]);
            rest = &rest[end..];
        }
        Ok(Self { parts })
    }

    /// The parts, in order; one when the value fits one line.
    pub(in super::super) fn parts(&self) -> &[&'a str] {
        &self.parts
    }
}

/// The value as a dialog quotes it.
pub(super) fn quoted(text: &str) -> String {
    serde_json::to_string(text).expect("string serialization")
}

fn quoted_chars(text: &str) -> usize {
    quoted(text).chars().count()
}

/// Whether `text` holds `DASH_RUN` dash-like characters in a row: dash
/// punctuation (General_Category Pd, the hyphen-minus included), the minus
/// sign, or a box-drawing character.
fn has_dash_run(text: &str) -> bool {
    use icu_properties::{props::GeneralCategory, CodePointMapData};
    let category = CodePointMapData::<GeneralCategory>::new();
    let dash_like = |character: char| {
        category.get(character) == GeneralCategory::DashPunctuation
            || character == '\u{2212}'
            || ('\u{2500}'..='\u{257F}').contains(&character)
    };
    let mut run = 0;
    text.chars().any(|character| {
        run = if dash_like(character) { run + 1 } else { 0 };
        run >= DASH_RUN
    })
}

/// Whether `text` holds an opening bracket, one or more decimal digits and a
/// closing bracket, in a row (General_Category Ps, Nd and Pe), the shape of a
/// part's label.
fn has_bracketed_number(text: &str) -> bool {
    use icu_properties::{props::GeneralCategory, CodePointMapData};
    let category = CodePointMapData::<GeneralCategory>::new();
    let chars: Vec<GeneralCategory> = text.chars().map(|c| category.get(c)).collect();
    chars.iter().enumerate().any(|(at, kind)| {
        *kind == GeneralCategory::OpenPunctuation && {
            let digits = chars[at + 1..]
                .iter()
                .take_while(|kind| **kind == GeneralCategory::DecimalNumber)
                .count();
            digits > 0 && chars.get(at + 1 + digits) == Some(&GeneralCategory::ClosePunctuation)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn split(text: &str, width: usize) -> Result<Vec<&str>, VoucherTextRefusal> {
        VoucherText::of(text, width, |_| width).map(|shown| shown.parts().to_vec())
    }

    #[test]
    fn a_value_that_fits_one_line_is_one_part() {
        assert_eq!(split("Rent for June", 15), Ok(vec!["Rent for June"]));
        // Fits only as one line: shape checks apply to parts alone.
        assert_eq!(split("(1) ----", 10), Ok(vec!["(1) ----"]));
    }

    #[test]
    fn parts_split_only_after_a_space_and_join_to_the_value() {
        let text = "Cash deposit by self at branch 000123 on 25 Jul";
        let parts = split(text, 20).unwrap();
        assert_eq!(parts.concat(), text);
        assert!(parts.len() > 1);
        for part in &parts[..parts.len() - 1] {
            assert!(part.ends_with(' '), "{part:?}");
        }
        for part in &parts {
            assert!(quoted_chars(part) <= 20, "{part:?}");
        }
        // The longest fitting prefix ending in a space is taken.
        assert_eq!(parts[0], "Cash deposit by ");
    }

    #[test]
    fn widths_are_measured_on_the_quoted_form() {
        // Each `"` quotes as two characters: the first part is 6 characters
        // raw and 10 quoted.
        let text = "a\"b\"c d";
        assert_eq!(split(text, 10), Ok(vec!["a\"b\"c ", "d"]));
        assert_eq!(split(text, 9), Err(VoucherTextRefusal::DoesNotFit));
    }

    #[test]
    fn a_value_with_no_space_to_split_at_is_refused() {
        // A run of twelve digits, built from short pieces.
        let digits = ["000", "123", "456", "789"].concat();
        assert_eq!(
            split(&format!("UPI/{digits}/PAYMENT/SYNTHETIC"), 20),
            Err(VoucherTextRefusal::DoesNotFit)
        );
        // A space too late to keep the first part in its width.
        assert_eq!(
            split(&format!("{digits}{digits} x"), 20),
            Err(VoucherTextRefusal::DoesNotFit)
        );
    }

    #[test]
    fn a_split_keeps_every_character_of_multibyte_and_combining_text() {
        // A combining acute accent right after a space opens a part: no
        // character is lost or cut in two, whatever the rendering does.
        let text = "caf\u{e9} cr\u{e8}me \u{301}x br\u{fb}l\u{e9}e \u{4e2d}\u{6587} done";
        let parts = split(text, 14).unwrap();
        assert!(parts.len() > 1, "{parts:?}");
        assert_eq!(parts.concat(), text);
        assert!(
            parts.iter().any(|part| part.starts_with('\u{301}')),
            "{parts:?}"
        );
    }

    #[test]
    fn a_value_over_the_dialog_cap_is_refused_before_splitting() {
        let text = "word ".repeat(MOST_CHARS / 5 + 1);
        assert_eq!(split(&text, 50), Err(VoucherTextRefusal::DoesNotFit));
    }

    #[test]
    fn a_value_in_parts_may_not_hold_dashes_or_a_bracketed_number() {
        let long = |inner: &str| format!("Synthetic narration {inner} that does not fit one line");
        for inner in [
            "----",
            "\u{2014}\u{2014}\u{2014}\u{2014}",
            "\u{2500}\u{2500}\u{2500}\u{2500}",
            "\u{257F}\u{257F}\u{257F}\u{257F}",
            "-\u{2212}-\u{2013}",
            "(1)",
            "(12)",
            "[3]",
            "\u{FF08}\u{FF11}\u{FF09}",
        ] {
            assert_eq!(
                split(&long(inner), 30),
                Err(VoucherTextRefusal::Layout),
                "{inner:?}"
            );
        }
        // Empty brackets, and an opening quotation mark (Initial_Punctuation,
        // not an opening bracket) before a number.
        for inner in ["---", "(a1)", "( 1)", "(1", "1)", "-- --", "()", "\u{ab}1)"] {
            assert!(split(&long(inner), 30).is_ok(), "{inner:?}");
        }
    }

    #[test]
    fn the_cap_counts_the_value_itself_and_refuses_from_one_character_over() {
        let at_cap = "word ".repeat(MOST_CHARS / 5);
        assert_eq!(at_cap.chars().count(), MOST_CHARS);
        assert_eq!(split(&at_cap, 50).unwrap().concat(), at_cap);
        let over = format!("{at_cap}x");
        assert_eq!(split(&over, 50), Err(VoucherTextRefusal::DoesNotFit));
        // Under the cap as typed and over it quoted: each `"` quotes as two
        // characters, and the value is still shown, in parts.
        let quotes = "a\" ".repeat(500);
        assert!(quotes.chars().count() < MOST_CHARS && quoted_chars(&quotes) > MOST_CHARS);
        assert_eq!(split(&quotes, 50).unwrap().concat(), quotes);
    }

    #[test]
    fn a_value_over_the_cap_is_refused_as_not_fitting_before_its_layout_is_read() {
        let text = format!("{}----", "word ".repeat(MOST_CHARS / 5));
        assert_eq!(split(&text, 50), Err(VoucherTextRefusal::DoesNotFit));
    }

    #[test]
    fn only_a_space_splits_a_value() {
        // A tab would fit the first part's width; no space does.
        let text = "ab\tcdefgh ij";
        assert_eq!(
            VoucherText::of(text, 0, |k| if k == 1 { 6 } else { 20 }),
            Err(VoucherTextRefusal::DoesNotFit)
        );
    }

    #[test]
    fn the_width_of_each_part_is_asked_by_its_number() {
        let text = "aa bb cc dd";
        let shown = VoucherText::of(text, 0, |k| if k == 1 { 6 } else { 20 }).unwrap();
        assert_eq!(shown.parts(), ["aa ", "bb cc dd"]);
    }
}
