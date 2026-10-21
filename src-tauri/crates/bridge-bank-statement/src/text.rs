//! String folds, ported with Python's semantics rather than Rust's defaults.
//!
//! The rules were first written against Python's `str.strip`,
//! `str.isspace`, `str.upper` and `re.sub(r"\s+", ...)`. Rust's
//! `char::is_whitespace` is the Unicode `White_Space` property, which omits
//! the four information separators U+001C..U+001F that Python's `isspace`
//! accepts, so every whitespace test here goes through [`is_space`].

/// Python's `str.isspace` for one character.
pub fn is_space(character: char) -> bool {
    character.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&character)
}

/// Python's `str.strip()`.
pub fn strip(text: &str) -> &str {
    text.trim_matches(is_space)
}

/// `_squash`: every whitespace run becomes one space, then strip.
pub fn squash(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;
    for character in text.chars() {
        if is_space(character) {
            pending_space = true;
        } else {
            if pending_space && !out.is_empty() {
                out.push(' ');
            }
            pending_space = false;
            out.push(character);
        }
    }
    out
}

/// `_strip`: remove every whitespace character.
pub fn remove_space(text: &str) -> String {
    text.chars()
        .filter(|character| !is_space(*character))
        .collect()
}

/// `_key`: the mapping key. Whitespace-insensitive, everything else
/// significant, case folded with Unicode upper-casing (`straße` → `STRASSE`,
/// as Python's `str.upper` does).
///
/// Whitespace is folded because a PDF cell wrap splits one counterparty's name
/// two ways in one statement (`NORTHWINDT RADERS` / `NORTHWINDTRADERS`).
/// Punctuation is kept: dropping it collapsed `A & B` with `AB` and posted two
/// payees to one ledger.
///
/// Dropping punctuation also collapsed `S.K. ACME` with `SK ACME`, `M/S ACME` with
/// `MS ACME` and `ACME-RAM TRADERS` with `ACME RAM TRADERS`: different names, silently
/// posted to one ledger. The mapping loader refuses two mapping rows that collide, but
/// nothing refuses a statement party colliding with a row written for somebody else:
/// there is exactly one candidate, so no ambiguity to reject, and the write goes to a
/// ledger the person never chose for it. A sole candidate under a loose fold is not a
/// resolution (protocol reference section 9.4b). Removing the punctuation fold cost
/// nothing measurable: party fields in the measured statements are upper-case
/// alphanumeric, so none carried punctuation the old key dropped, and the one real merge,
/// the wrap case above, survives.
///
/// The fold is done on characters, not by a category filter, so a name written in
/// Devanagari, Tamil or Bengali keeps every character instead of reducing toward the
/// empty string, and combining marks stay with the letters they modify (a filter on
/// alphanumerics would drop some of them).
///
/// There is no "empty key" refusal, deliberately: removing only whitespace makes an empty
/// key unreachable, because [`squash`] has already dropped a name that is entirely
/// whitespace and every other name keeps at least one character. An unreachable guard
/// reads as protection and is not.
pub fn mapping_key(text: &str) -> String {
    text.to_uppercase()
        .chars()
        .filter(|character| !is_space(*character))
        .collect()
}

/// `_ledger_key`: a fold deliberately **looser** than Tally's measured
/// master-name identity (protocol reference §9.4b).
///
/// Used only where looseness fails safe (the uses are listed under "Why a loose fold is
/// safe here" below). It agrees with Tally's measured identity on ASCII case folding, one
/// trailing space and a space supplied where the master stores a hyphen. **Never use it
/// to bind a name to a master** — that is `validate_masters`' job, and a sole
/// candidate under a loose fold is not a resolution.
///
/// It is looser than the measurement in several ways, and each is a transformation Tally
/// has not been shown to make: a hyphen supplied for a stored space, an internal
/// whitespace run collapsed, leading whitespace ignored, two or more trailing spaces
/// ignored, a tab or no-break space read as a space, and non-ASCII case folding
/// (`to_uppercase` applies full Unicode case mapping, so a German sharp s (U+00DF) and
/// `SS` collide, and the length changes, which no rule in section 9.4b contemplates).
/// Kept apart: `&`
/// against `AND`, a deleted `&`, an en dash, an underscore and `/`.
///
/// The measurement is also directional (a space was supplied where the master carried a
/// hyphen; the reverse was never sent) and a fold is symmetric by construction, so it
/// cannot express that. When the question is "will Tally match these?", use the
/// asymmetric predicate section 9.4b gives, not this.
///
/// Why a loose fold is safe here: nothing in this crate resolves a name against Tally's
/// masters. The ledger name comes from the person's own mapping and is carried into the
/// proposal as given (whitespace squashed), and Tally does its own matching at import.
/// The fold feeds only internal comparisons, and being loose fails safe in each: refusing
/// a voucher whose legs collapse together (looser refuses more), refusing a cash answer
/// that names the suspense ledger (looser refuses more), flagging a row as unidentified
/// for a person to look at (looser flags more), and matching the bank leg when the
/// proposals are re-read, where both sides came from the same bank-ledger value so the
/// fold changes nothing.
pub fn ledger_key(name: &str) -> String {
    squash(&name.replace('-', " ")).to_uppercase()
}

/// Python's `str.isupper()`: at least one cased character, and no lower-case
/// one.
pub fn is_upper(text: &str) -> bool {
    let mut cased = false;
    for character in text.chars() {
        if character.is_lowercase() {
            return false;
        }
        if character.is_uppercase() {
            cased = true;
        }
    }
    cased
}

/// Python's `str.isalnum()` on a nonempty string.
pub fn is_alnum(text: &str) -> bool {
    !text.is_empty() && text.chars().all(char::is_alphanumeric)
}
