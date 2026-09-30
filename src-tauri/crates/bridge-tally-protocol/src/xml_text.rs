//! One shared XML text-escaping routine for every Tally request renderer.
//!
//! [`escape_text`] replaces the roughly ten near-identical private
//! `xml_escape` copies this crate and the app crate each carried (bridge#832):
//! most escaped only `&`, `<`, `>`, `"`, `'`; the shipped voucher-import write
//! path also escaped CR and LF, and nothing else did. [`escape_text`] is the
//! superset of both sets, so every call site gets the stronger guarantee.
//!
//! Escaping CR and LF matters even though the XML 1.0 grammar would pass a
//! literal CR or LF through unescaped: the spec's end-of-line handling rule
//! requires a conforming parser to fold a literal CR LF pair (and a lone CR)
//! to a single LF before the application ever sees the text. Left raw, a
//! value that ends in CR LF silently becomes a different value once Tally's
//! own file reader parses it back -- for the voucher-import write path this
//! showed up as a ledger name ending in a line break naming a ledger the book
//! does not hold (bridge#626). Writing CR and LF as `&#13;`/`&#10;` character
//! references survives that fold intact.
//!
//! This is ordinary XML character-data escaping: it is correct for a value
//! placed in an element's text content or in a quoted XML attribute. It does
//! **not** protect a value placed inside a quoted Tally *formula* literal
//! (`<SET>"..."</SET>`, or a quoted `$$Date:"..."` function argument): Tally
//! decodes `&quot;` back to a literal `"` before evaluating the formula text,
//! so an escaped quote can still terminate the literal early and let
//! attacker-controlled text run as TDL. A value that must go there needs a
//! closed input alphabet instead of escaping -- see
//! `bridge_tally_primitives::TallyDate` and the doc comment on
//! `native_outstandings::render_native_voucher_export_request`.

/// Escapes `value` for insertion as XML character data or an XML attribute
/// value: `&`, `<`, `>`, `"`, `'`, CR (`\r`) and LF (`\n`) each become their
/// predefined entity or a numeric character reference (`&#13;`, `&#10;`).
///
/// Escaping is **not** idempotent, by design: running it twice double-escapes
/// (`&` becomes `&amp;amp;`, which then decodes to the literal text `&amp;`
/// instead of `&`). Every call site must escape a raw, Tally-derived value
/// exactly once, at the point it is interpolated into a request, and never
/// re-escape a string that has already been through this function.
pub fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('\r', "&#13;")
        .replace('\n', "&#10;")
}

#[cfg(test)]
#[path = "xml_text_tests.rs"]
mod tests;
