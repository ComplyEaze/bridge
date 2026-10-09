// SPDX-License-Identifier: Apache-2.0
//! What `xml.rs` returns for CDATA, entities and bytes that are not valid UTF-8, pinned on
//! quick-xml 0.41 before the crate moved to 0.42 (#1198); the move had to leave every case here
//! unchanged.
//!
//! Each case compares the whole result: the full tree (name, attributes, `.text`, children) or
//! the whole error, its variant with its `part` and `detail`. Four details carry quick-xml's own
//! message (`parse_char_ref_nul`, `parse_char_ref_surrogate`, `bare_ampersand`,
//! `mismatched_end_tag`); a change there is the library's, and is reported, not re-pinned.

use bridge_tax_audit::error::{AuditError, Result};
use bridge_tax_audit::xml::{self, Element};

/// A string in quotes with every character outside printable ASCII, and `"` and `\`, written
/// as `\u{hex}`, so control characters, U+FEFF and U+FFFD stay visible in the expectations.
fn quoted(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        if matches!(c, ' '..='~') && c != '"' && c != '\\' {
            out.push(c);
        } else {
            out.push_str(&format!("\\u{{{:x}}}", u32::from(c)));
        }
    }
    out.push('"');
    out
}

/// The whole tree, one line: `NAME[key="value" ...]"text"(child child ...)`.
fn tree(element: &Element) -> String {
    let attrs: Vec<String> = element
        .attrs
        .iter()
        .map(|(key, value)| format!("{key}={}", quoted(value)))
        .collect();
    let children: Vec<String> = element.children.iter().map(tree).collect();
    format!(
        "{}[{}]{}({})",
        element.name,
        attrs.join(" "),
        quoted(&element.text),
        children.join(" ")
    )
}

fn render(result: Result<Element>) -> String {
    match result {
        Ok(root) => format!("Ok {}", tree(&root)),
        Err(AuditError::Parse { part, detail }) => {
            format!("Parse {} {}", quoted(&part), quoted(&detail))
        }
        Err(other) => panic!("not a parse error: {other:?}"),
    }
}

/// `xml::read` on bytes: decode (the encoding, the DTD check, the forbidden-reference rewrite),
/// then parse. This is what every loader calls.
fn read(part: &str, bytes: &[u8]) -> String {
    render(xml::read(bytes, part))
}

/// `xml::parse` on text, skipping decode, so a forbidden reference reaches the reader unmarked.
fn parse(part: &str, text: &str) -> String {
    render(xml::parse(text, part))
}

fn utf16le(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

fn cases() -> Vec<(&'static str, String, &'static str)> {
    let mut lone_surrogate = utf16le("<A>x");
    lone_surrogate.extend_from_slice(&0xD800u16.to_le_bytes());
    lone_surrogate.extend(utf16le("</A>"));
    let mut odd_length = utf16le("<A>x</A>");
    odd_length.push(b' ');
    vec![
        // CDATA
        (
            "cdata_markup",
            read("cdata_markup", b"<A><![CDATA[x & y < z]]></A>"),
            r#"Ok A[]"x & y < z"()"#,
        ),
        (
            "cdata_references",
            read("cdata_references", b"<A><![CDATA[&amp; &#4; Primary]]></A>"),
            r#"Ok A[]"&amp; \u{fffd}#4; Primary"()"#,
        ),
        (
            "cdata_line_ends",
            read("cdata_line_ends", b"<A><![CDATA[a\r\nb\rc]]></A>"),
            r#"Ok A[]"a\u{a}b\u{a}c"()"#,
        ),
        (
            "cdata_between_text",
            read("cdata_between_text", b"<A>a<![CDATA[b]]>c</A>"),
            r#"Ok A[]"abc"()"#,
        ),
        (
            "cdata_after_child",
            read("cdata_after_child", b"<A><B/>t<![CDATA[x]]></A>"),
            r#"Ok A[]""(B[]""())"#,
        ),
        // Entities and references
        (
            "predefined_in_text",
            read("predefined_in_text", b"<A>&lt;&gt;&amp;&quot;&apos;</A>"),
            r#"Ok A[]"<>&\u{22}'"()"#,
        ),
        (
            "predefined_in_attribute",
            read(
                "predefined_in_attribute",
                b"<A k=\"&lt;&gt;&amp;&quot;&apos;\"/>",
            ),
            r#"Ok A[k="<>&\u{22}'"]""()"#,
        ),
        (
            "attribute_normalisation",
            read(
                "attribute_normalisation",
                b"<A k=\"a\tb\nc\r\nd&#10;e&#9;f\"/>",
            ),
            r#"Ok A[k="a b c d\u{a}e\u{9}f"]""()"#,
        ),
        (
            "legal_references",
            read("legal_references", b"<A>&#65;&#x42;&#x1F600;&#xfffd;</A>"),
            r#"Ok A[]"AB\u{1f600}\u{fffd}"()"#,
        ),
        (
            "forbidden_references_marked",
            read(
                "forbidden_references_marked",
                b"<A k=\"&#4; Primary\">&#4;|&#x4;|&#0;|&#xD800;|&#xFFFE;</A>",
            ),
            r#"Ok A[k="\u{fffd}#4; Primary"]"\u{fffd}#4;|\u{fffd}#4;|\u{fffd}#0;|\u{fffd}#55296;|\u{fffd}#65534;"()"#,
        ),
        (
            "forbidden_reference_unmarked",
            read("forbidden_reference_unmarked", b"<A>&#00000000004;</A>"),
            r#"Parse "forbidden_reference_unmarked" "reference to U+0004, which XML does not allow""#,
        ),
        (
            "undefined_entity",
            read("undefined_entity", b"<A>&nbsp;</A>"),
            r#"Parse "undefined_entity" "undefined entity &nbsp;""#,
        ),
        (
            "bare_ampersand",
            read("bare_ampersand", b"<A>a & b</A>"),
            r#"Parse "bare_ampersand" "malformed XML at byte 5: ill-formed document: entity or character reference not closed: `;` not found before end of input""#,
        ),
        (
            "mismatched_end_tag",
            read("mismatched_end_tag", b"<A><B></A>"),
            r#"Parse "mismatched_end_tag" "malformed XML at byte 6: ill-formed document: expected `</B>`, but `</A>` was found""#,
        ),
        (
            "parse_char_ref_nul",
            parse("parse_char_ref_nul", "<A>&#0;</A>"),
            r#"Parse "parse_char_ref_nul" "character reference: invalid character reference: 0x0 character is not permitted in XML""#,
        ),
        (
            "parse_char_ref_surrogate",
            parse("parse_char_ref_surrogate", "<A>&#xD800;</A>"),
            r#"Parse "parse_char_ref_surrogate" "character reference: invalid character reference: `55296` is not a valid codepoint""#,
        ),
        (
            "parse_char_ref_fffe",
            parse("parse_char_ref_fffe", "<A>&#xFFFE;</A>"),
            r#"Parse "parse_char_ref_fffe" "reference to U+FFFE, which XML does not allow""#,
        ),
        // Encodings, and bytes that are not valid in theirs
        (
            "utf8_non_ascii",
            read("utf8_non_ascii", "<A k=\"é\">₹ 1</A>".as_bytes()),
            r#"Ok A[k="\u{e9}"]"\u{20b9} 1"()"#,
        ),
        (
            "utf8_invalid_byte",
            read("utf8_invalid_byte", b"<A>a\xffb</A>"),
            r#"Parse "utf8_invalid_byte" "undecodable content: InvalidUtf8""#,
        ),
        (
            "utf8_bom_invalid_byte",
            read("utf8_bom_invalid_byte", b"\xef\xbb\xbf<A>a\xffb</A>"),
            r#"Parse "utf8_bom_invalid_byte" "undecodable content: InvalidUtf8""#,
        ),
        (
            "utf8_truncated_sequence",
            read("utf8_truncated_sequence", b"<A>a</A>\xe2\x82"),
            r#"Parse "utf8_truncated_sequence" "undecodable content: InvalidUtf8""#,
        ),
        (
            "utf8_second_bom",
            read("utf8_second_bom", b"\xef\xbb\xbf\xef\xbb\xbf<A>x</A>"),
            r#"Ok A[]"x"()"#,
        ),
        (
            "utf16le_non_bmp",
            read("utf16le_non_bmp", &utf16le("<A>\u{1F600}</A>")),
            r#"Ok A[]"\u{1f600}"()"#,
        ),
        (
            "utf16le_lone_surrogate",
            read("utf16le_lone_surrogate", &lone_surrogate),
            r#"Parse "utf16le_lone_surrogate" "invalid UTF-16LE content""#,
        ),
        (
            "utf16le_odd_length",
            read("utf16le_odd_length", &odd_length),
            r#"Parse "utf16le_odd_length" "truncated UTF-16LE content""#,
        ),
    ]
}

#[test]
fn xml_rs_returns_the_pinned_result_for_cdata_entities_and_invalid_bytes() {
    let mut differ = Vec::new();
    for (name, actual, expected) in cases() {
        if actual != expected {
            differ.push(format!(
                "{name}\n  expected: {expected}\n  actual:   {actual}"
            ));
        }
    }
    assert!(differ.is_empty(), "{}", differ.join("\n"));
}
