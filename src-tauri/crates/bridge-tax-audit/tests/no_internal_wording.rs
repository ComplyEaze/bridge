//! No text a firm reads uses the developers' working vocabulary: the name of a working group
//! followed by a letter, the name of the coordinating role, or the word for the person who decides
//! (the reference's REND-12, whose pattern this file follows case for case). The prose a port
//! renders is what the committed goldens hold in their definition, title, limits, ask,
//! population-note and invariant-detail fields.
//!
//! What is scanned: those six keys, wherever a golden holds them, whole, and nothing else. What is
//! NOT: ledger and person names where they stand alone (ids, labels, subjects: data, not prose);
//! and text-valued figure values and evidence labels (the reference scans every cell it renders,
//! so it would read them where they are rendered; this golden-level test does not, and is not
//! widened to). An invariant `detail` is read whole, a ledger name inside it included: no
//! committed golden holds such a name there, and an edge book that would put one there should
//! name its ledger otherwise.
//!
//! Two narrow differences from the reference's pattern, both needing text no golden holds: a word
//! glued to a combining vowel sign is not seen as a word of its own here, and the separators
//! U+001C to U+001F are not read as a space.
//!
//! The scan proves itself first. It must flag the invented samples below (built at run time, so
//! this file does not hold them verbatim) and pass the ordinary uses; it must find a planted hit
//! under each of the six keys of a golden-shaped document, through the same walk that reads the
//! goldens; and it must have read a non-trivial number of texts under each key, so a scan that
//! found nothing because it read nothing (of one key, or of all) fails.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde_json::Value;

/// The keys whose string values (or lists of strings) are prose a CA reads.
const PROSE_KEYS: [&str; 6] = [
    "definition_text",
    "title_text",
    "limits_text",
    "ask_client_text",
    "population_note_text",
    "detail",
];

/// The fewest texts the committed goldens must hold under each key (about nine tenths of what
/// they hold: 6,221, 757, 1,770, 1,025, 169 and 593), and the fewest goldens.
const FLOORS: [(&str, usize); 6] = [
    ("definition_text", 5_500),
    ("title_text", 650),
    ("limits_text", 1_500),
    ("ask_client_text", 900),
    ("population_note_text", 150),
    ("detail", 500),
];
const MIN_FILES: usize = 150;

/// The words that may precede the person word without it being the developers' vocabulary. The
/// reference's look-behind is exact: "Truck " with a capital is not among them.
const ORDINARY_PRECEDING_WORDS: [&str; 4] = ["truck ", "vehicle ", "property ", "building "];

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Whether the characters on either side of the span `at..at + len` are not word characters (the
/// pattern's `\b` at both ends).
fn word_at(hay: &str, at: usize, len: usize) -> bool {
    let before = hay[..at].chars().next_back();
    let after = hay[at + len..].chars().next();
    !before.is_some_and(is_word_char) && !after.is_some_and(is_word_char)
}

/// The working-group word (capital first letter only) + whitespace + a capital ASCII letter and
/// an optional ASCII digit, on word boundaries.
fn group_hit(text: &str) -> Option<(usize, String)> {
    for (i, _) in text.match_indices("Lane") {
        let rest = &text[i + 4..];
        let spaced = rest.trim_start_matches(char::is_whitespace);
        if spaced.len() < rest.len() && !text[..i].chars().next_back().is_some_and(is_word_char) {
            let mut cs = spaced.chars();
            if cs.next().is_some_and(|c| c.is_ascii_uppercase()) {
                let tail = cs.as_str();
                let after = match tail.chars().next() {
                    Some(d) if d.is_ascii_digit() => &tail[d.len_utf8()..],
                    _ => tail,
                };
                if !after.chars().next().is_some_and(is_word_char) {
                    return Some((i, text[i..text.len() - after.len()].to_string()));
                }
            }
        }
    }
    None
}

/// Whether `c` matches the lower-case ASCII letter `p` as Python's `(?i)` matches a literal: in
/// either case, and the long s for s.
fn ci_matches(c: char, p: char) -> bool {
    c == p || c.to_lowercase().eq(std::iter::once(p)) || (p == 's' && c == '\u{17f}')
}

/// The coordinating role's name in any case, as a whole word, found without lower-casing the text
/// (which can change its byte length).
fn coordinator_hit(text: &str) -> Option<(usize, String)> {
    // In two pieces, so this file does not hold the word it refuses.
    const WORD: &str = concat!("orche", "strator");
    for (i, _) in text.char_indices() {
        let mut cs = text[i..].char_indices();
        let mut end = i;
        let all = WORD.chars().all(|p| match cs.next() {
            Some((off, c)) if ci_matches(c, p) => {
                end = i + off + c.len_utf8();
                true
            }
            _ => false,
        });
        if all && word_at(text, i, end - i) {
            return Some((i, text[i..end].to_string()));
        }
    }
    None
}

/// The decider's word in its lower-case or capitalised form (not the all-capitals one) as a whole
/// word, not after "truck ", "vehicle ", "property " or "building " and not followed by
/// whitespace and `of`.
fn person_hit(text: &str) -> Option<(usize, String)> {
    let mut found: Option<(usize, String)> = None;
    for needle in [concat!("ow", "ner"), concat!("Ow", "ner")] {
        for (i, _) in text.match_indices(needle) {
            if !word_at(text, i, needle.len()) {
                continue;
            }
            if ORDINARY_PRECEDING_WORDS
                .iter()
                .any(|w| text[..i].ends_with(w))
            {
                continue;
            }
            let after = &text[i + needle.len()..];
            let spaced = after.trim_start_matches(char::is_whitespace);
            if spaced.len() < after.len()
                && spaced.starts_with("of")
                && !spaced[2..].chars().next().is_some_and(is_word_char)
            {
                continue;
            }
            if found.as_ref().is_none_or(|(at, _)| i < *at) {
                found = Some((i, needle.to_string()));
            }
            break;
        }
    }
    found
}

/// The first hit of the rule in `text` (the earliest of the three alternatives), as the
/// reference's `_R12_RE` finds it.
fn internal_wording(text: &str) -> Option<String> {
    [group_hit(text), coordinator_hit(text), person_hit(text)]
        .into_iter()
        .flatten()
        .min_by_key(|(at, _)| *at)
        .map(|(_, hit)| hit)
}

/// Every prose string of a document with the key it sits under (a list keeps its key).
fn collect_prose(v: &Value, key: &str, out: &mut Vec<(&'static str, String)>) {
    match v {
        Value::String(s) => {
            if let Some(k) = PROSE_KEYS.iter().copied().find(|k| *k == key) {
                out.push((k, s.clone()));
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_prose(item, key, out);
            }
        }
        Value::Object(map) => {
            for (k, item) in map {
                collect_prose(item, k, out);
            }
        }
        _ => {}
    }
}

/// What one golden-shaped document gives: how many texts it holds under each key, and each hit
/// with its key.
fn scan_document(doc: &Value) -> (BTreeMap<&'static str, usize>, Vec<(&'static str, String)>) {
    let mut prose = Vec::new();
    collect_prose(doc, "", &mut prose);
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut hits = Vec::new();
    for (key, text) in prose {
        *counts.entry(key).or_insert(0) += 1;
        if let Some(hit) = internal_wording(&text) {
            hits.push((key, hit));
        }
    }
    (counts, hits)
}

/// The vocabulary the rule refuses, in pieces: joined at run time, so that this file does not hold
/// a sample verbatim.
fn join(parts: &[&str]) -> String {
    parts.concat()
}

fn group(tail: &str) -> String {
    join(&["La", "ne ", tail])
}

fn coordinator(a: &str, b: &str) -> String {
    join(&[a, "rche", "strator", b])
}

fn person(a: &str, b: &str) -> String {
    join(&[a, "ow", "ner", b])
}

#[test]
fn the_scan_flags_what_it_must_and_passes_ordinary_prose() {
    for flagged in [
        join(&["the rule (", "ow", "ner, 25-Sep) lowers nothing"]),
        group("R's check"),
        group("D waits"),
        group("C3 waits"),
        format!("ask the {}", coordinator("o", "")),
        format!("The {} decides", coordinator("O", "")),
        // The pattern is case-insensitive for this word, and for its letter s in any form.
        format!("ask the {}", coordinator("O", "").to_uppercase()),
        join(&["ask the orche", "\u{17f}trator"]),
        person("waits on the ", ""),
        join(&["Ow", "ner decision"]),
        // The look-behind is exact: a capital T is not "truck ".
        person("Truck ", ""),
        // A text whose lower-case form is longer than itself is still read, never skipped.
        format!("\u{130} {}", person("", "")),
        format!("{} \u{130}", coordinator("o", "")),
    ] {
        assert!(internal_wording(&flagged).is_some(), "{flagged}");
    }
    for ordinary in [
        person("the truck ", ""),
        person("a vehicle ", ""),
        person("property ", ""),
        person("building ", ""),
        person("the ", " of the shop"),
        person("the ", "\tof the shop"),
        join(&["La", "ne markings"]),
        join(&["a la", "ne"]),
        join(&["La", "nes and roads"]),
        join(&["co", "ow", "ner"]),
        join(&["ow", "nership"]),
        // The all-capitals word is no hit for the reference's pattern, which reads the lower-case
        // and capitalised forms only.
        person("the ", "").to_uppercase(),
        join(&["La", "ne r"]),
        join(&["La", "ne R12"]),
        join(&["La", "ne"]),
    ] {
        assert_eq!(internal_wording(&ordinary), None, "{ordinary}");
    }
}

/// A document shaped as a golden is, with one planted hit under each prose key, found through the
/// same walk that reads the goldens.
#[test]
fn the_walk_finds_a_planted_hit_under_each_prose_key() {
    let hit = person("waits on the ", "");
    let doc = serde_json::json!({
        "population_note_text": hit,
        "figures": [{"id": "f", "definition_text": hit, "evidence": []}],
        "findings": [{
            "id": "x",
            "title_text": hit,
            "limits_text": [hit, "plain"],
            "ask_client_text": [hit],
            "evidence": []
        }],
        "module_invariant_violations": [{"invariant": "I", "subject": "s", "detail": hit}]
    });
    let (counts, hits) = scan_document(&doc);
    let found: BTreeSet<&str> = hits.iter().map(|(k, _)| *k).collect();
    for key in PROSE_KEYS {
        assert!(found.contains(key), "no hit found under {key}: {hits:?}");
    }
    assert_eq!(hits.len(), 6, "{hits:?}");
    assert_eq!(counts["limits_text"], 2, "{counts:?}");
}

#[test]
fn no_golden_prose_uses_the_working_vocabulary() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/golden");
    let mut files = 0usize;
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut hits = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("the golden directory reads") {
        let path = entry.expect("a directory entry reads").path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let doc: Value = serde_json::from_slice(&std::fs::read(&path).expect("a golden reads"))
            .expect("a golden is JSON");
        let (c, h) = scan_document(&doc);
        files += 1;
        for (key, n) in c {
            *counts.entry(key).or_insert(0) += n;
        }
        for (key, hit) in h {
            hits.push(format!(
                "{} {key}: {hit:?}",
                path.file_name().and_then(|n| n.to_str()).unwrap_or("?")
            ));
        }
    }
    assert!(files >= MIN_FILES, "scanned {files} goldens");
    for (key, floor) in FLOORS {
        let n = counts.get(key).copied().unwrap_or(0);
        assert!(n >= floor, "scanned {n} texts under {key}, floor {floor}");
    }
    assert!(hits.is_empty(), "{hits:#?}");
}
