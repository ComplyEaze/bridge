//! A tripwire over the committed statement captures, `scripts/fixtures/*-bbox-capture.xml`.
//!
//! Every customer value in them was fabricated, from a fixed alphabet, when the capture was
//! sanitised (the bank's own wording is kept); the real statements are not in the repository. What this test can say is only that
//! nothing outside ASCII is there, once the XML's own entities are decoded: a name, an address or
//! a narration in an Indian-language script, or an accented letter, would show up here. It is a
//! tripwire and not a gate. It passes for the raw text layer of any statement printed in English,
//! so it does not stop a raw statement being committed: `scripts/fixtures/README.md` states the
//! rule that no new real capture is committed, and a capture is reviewed by a person.
//!
//! The glob is deliberate: a capture added later is checked for ASCII without editing the check.
//! The list asserted at the end is the set that exists; adding a capture deliberately means editing
//! it, which is a prompt to read the rule above.

mod common;

use std::fs;

use common::repository_root;

/// The document without its comments. The banner at the top of each capture is a comment: it
/// is prose about the capture (it even quotes a numeric character reference as an example of
/// what is not preserved), and it is not statement text.
fn without_comments(text: &str) -> Result<String, String> {
    let mut kept = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("<!--") {
        kept.push_str(&rest[..start]);
        let after = &rest[start + 4..];
        let end = after.find("-->").ok_or("a comment is never closed")?;
        rest = &after[end + 3..];
    }
    kept.push_str(rest);
    Ok(kept)
}

/// The ASCII-only text of an XML document, comments removed and entities decoded, or the first
/// thing that cannot be: a character reference naming something outside ASCII, an entity this
/// reader does not know, or a character above 0x7f in the file itself.
fn decoded_ascii(file: &str, bytes: &[u8]) -> Result<String, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("{file}: not UTF-8: {error}"))?;
    let text = without_comments(text).map_err(|problem| format!("{file}: {problem}"))?;
    if let Some(at) = text.bytes().position(|byte| !byte.is_ascii()) {
        return Err(format!(
            "{file}: byte {at} outside the comments is not ASCII"
        ));
    }
    let mut decoded = String::with_capacity(text.len());
    let mut rest = text.as_str();
    while let Some(start) = rest.find('&') {
        decoded.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find(';') else {
            return Err(format!("{file}: an & with no ; after it"));
        };
        let entity = &after[..end];
        let character = match entity {
            "amp" => '&',
            "lt" => '<',
            "gt" => '>',
            "quot" => '"',
            "apos" => '\'',
            _ => {
                let number = if let Some(hex) = entity.strip_prefix("#x") {
                    u32::from_str_radix(hex, 16).ok()
                } else if let Some(decimal) = entity.strip_prefix('#') {
                    decimal.parse().ok()
                } else {
                    None
                };
                number
                    .and_then(char::from_u32)
                    .ok_or_else(|| format!("{file}: unknown entity &{entity};"))?
            }
        };
        if !character.is_ascii() {
            return Err(format!(
                "{file}: &{entity}; is U+{:04X}, which is not ASCII",
                u32::from(character)
            ));
        }
        decoded.push(character);
        rest = &after[end + 1..];
    }
    decoded.push_str(rest);
    Ok(decoded)
}

#[test]
fn the_committed_captures_hold_only_ascii_once_their_entities_are_decoded() {
    let directory = repository_root().join("scripts").join("fixtures");
    let mut seen = Vec::new();
    for entry in fs::read_dir(&directory).expect("scripts/fixtures is readable") {
        let path = entry.expect("a directory entry").path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !name.ends_with("-bbox-capture.xml") {
            continue;
        }
        let bytes = fs::read(&path).expect("a capture is readable");
        decoded_ascii(&name, &bytes).unwrap_or_else(|problem| panic!("{problem}"));
        seen.push(name);
    }
    seen.sort();
    // Not vacuous: the two captures the parser tests read are there.
    assert_eq!(seen, ["hdfc-bbox-capture.xml", "sbi-bbox-capture.xml"]);
}

#[test]
fn the_reader_refuses_what_it_exists_to_catch() {
    assert_eq!(
        decoded_ascii("f", b"<w>A &amp; B &#65; &#x42;</w>").unwrap(),
        "<w>A & B A B</w>"
    );
    // A banner may quote what it is not: prose in a comment is not statement text.
    assert_eq!(
        decoded_ascii(
            "f",
            "<!-- an em dash \u{2014} and &#2358; -->\n<w>A</w>".as_bytes()
        )
        .unwrap(),
        "\n<w>A</w>"
    );
    for (label, document) in [
        (
            "a non-ASCII byte in the file",
            "<w>caf\u{e9}</w>".as_bytes(),
        ),
        (
            "a decimal reference outside ASCII",
            b"<w>&#233;</w>".as_slice(),
        ),
        ("a hex reference outside ASCII", b"<w>&#xe9;</w>".as_slice()),
        (
            "an Indic letter by reference",
            b"<w>&#x0915;</w>".as_slice(),
        ),
        ("an unknown named entity", b"<w>&eacute;</w>".as_slice()),
        ("an unterminated entity", b"<w>&amp</w>".as_slice()),
    ] {
        assert!(decoded_ascii("f", document).is_err(), "{label}");
    }
}
