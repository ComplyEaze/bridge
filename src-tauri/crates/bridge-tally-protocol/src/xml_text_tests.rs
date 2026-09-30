use super::escape_text;

#[test]
fn escapes_every_reserved_character_and_cr_lf() {
    assert_eq!(escape_text("&"), "&amp;");
    assert_eq!(escape_text("<"), "&lt;");
    assert_eq!(escape_text(">"), "&gt;");
    assert_eq!(escape_text("\""), "&quot;");
    assert_eq!(escape_text("'"), "&apos;");
    assert_eq!(escape_text("\r"), "&#13;");
    assert_eq!(escape_text("\n"), "&#10;");
}

#[test]
fn escapes_a_mix_of_every_reserved_character_in_one_pass() {
    let input = "BRIDGE & <LAB> \"Q\" 'A'\r\n";
    let expected = "BRIDGE &amp; &lt;LAB&gt; &quot;Q&quot; &apos;A&apos;&#13;&#10;";
    assert_eq!(escape_text(input), expected);
}

#[test]
fn leaves_non_reserved_text_unchanged() {
    // Devanagari and the rupee sign carry no XML-reserved bytes and must
    // survive byte-for-byte.
    let input = "श्री गणेश ट्रेडर्स ₹12,345.00";
    assert_eq!(escape_text(input), input);
}

#[test]
fn leaves_plain_ascii_text_unchanged() {
    let input = "BRIDGE SYNTHETIC BOOK";
    assert_eq!(escape_text(input), input);
}

#[test]
fn escaping_twice_double_escapes_so_call_sites_must_escape_exactly_once() {
    // Escaping is deliberately not idempotent: a second pass turns the
    // entity written by the first pass into text for a *different* raw
    // character. A call site that escapes a value twice (or escapes an
    // already-escaped value it read back from somewhere) corrupts it instead
    // of leaving it alone.
    let once = escape_text("&");
    assert_eq!(once, "&amp;");
    let twice = escape_text(&once);
    assert_eq!(twice, "&amp;amp;");
    assert_ne!(twice, once);
}
