//! A ratchet on the texts that send a person to key or import vouchers in Tally by hand (#1709).
//!
//! ComplyEaze Bridge should settle its own work. Some refusals, next steps and tool descriptions
//! still tell the person to use Tally's voucher entry screen or to do something "by hand". This
//! test counts those phrases in the string literals of production sources and pins, per file, the
//! most it may hold. A count may only fall: a new text, or a new file, that carries one of the
//! phrases fails here. Lowering a pin when texts are removed is the work of the issue's later
//! slices; a pin that is higher than the count only leaves room, so lower it when you can.
//!
//! The scan reads string literals only. Comments are skipped, and so is everything from an inline
//! `#[cfg(test)] mod name {` to the end of a file, so documentation and test text never count.
//! A sentence that forbids the thing ("Do not create the ledger in Tally by hand.") is not an
//! instruction to do it, so a phrase in a sentence that carries a negation is not counted. It is
//! a count per file, not a list of lines, so an edit that swaps one hand-entry text for another
//! in the same file passes.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const PHRASES: [&str; 4] = ["entry screen", "by hand", "manually", "gateway of tally"];

/// Hits per production source file, relative to the crate directory. May only shrink.
const PINNED: &[(&str, usize)] = &[
    ("crates/bridge-tax-audit/src/financial_statements.rs", 1),
    ("src/agent_bank_statement.rs", 1),
    ("src/agent_catalog.rs", 20),
    ("src/agent_desktop_journal_review.rs", 4),
    ("src/agent_gst_status.rs", 1),
    ("src/agent_import.rs", 8),
    ("src/agent_import_numbering.rs", 2),
    ("src/agent_import_post.rs", 5),
    ("src/agent_masters.rs", 1),
];

/// The text of every string literal in `source`, in order. Comments are skipped; a backslash at
/// the end of a line joins the next line as Rust does; raw strings are read to their closing quote.
fn string_literals(source: &str) -> Vec<String> {
    let bytes = source.as_bytes();
    let mut literals = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &source[i..];
        if rest.starts_with("//") {
            i += rest.find('\n').unwrap_or(rest.len());
        } else if rest.starts_with("/*") {
            let mut depth = 0;
            while i < bytes.len() {
                if source[i..].starts_with("/*") {
                    depth += 1;
                    i += 2;
                } else if source[i..].starts_with("*/") {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        } else if let Some(raw) = raw_string_start(rest, i, bytes) {
            let (opening_len, hashes) = raw;
            let body_start = i + opening_len;
            let closing = format!("\"{}", "#".repeat(hashes));
            let end = source[body_start..]
                .find(&closing)
                .map_or(bytes.len(), |at| body_start + at);
            literals.push(source[body_start..end].to_string());
            i = (end + closing.len()).min(bytes.len());
        } else if bytes[i] == b'"' {
            let mut text = String::new();
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    if bytes[i + 1] == b'\n' || bytes[i + 1] == b'\r' {
                        i += 2;
                        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                            i += 1;
                        }
                    } else {
                        i += 2;
                    }
                    continue;
                }
                let ch = source[i..].chars().next().unwrap_or('\u{fffd}');
                text.push(ch);
                i += ch.len_utf8();
            }
            literals.push(text);
            i += 1;
        } else if bytes[i] == b'\'' {
            i += char_literal_len(rest);
        } else {
            i += source[i..].chars().next().map_or(1, char::len_utf8);
        }
    }
    literals
}

/// `r"`, `r#"`, `br"` and so on at `at`: the length of the opening and the number of hashes. Not an
/// identifier that merely ends in `r`.
fn raw_string_start(rest: &str, at: usize, bytes: &[u8]) -> Option<(usize, usize)> {
    if at > 0 && (bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_') {
        return None;
    }
    let after_prefix = rest.strip_prefix("br").or_else(|| rest.strip_prefix('r'))?;
    let prefix_len = rest.len() - after_prefix.len();
    let hashes = after_prefix
        .bytes()
        .take_while(|byte| *byte == b'#')
        .count();
    after_prefix[hashes..]
        .starts_with('"')
        .then_some((prefix_len + hashes + 1, hashes))
}

/// The length of a char literal such as `'"'` or `'\n'` at the start of `rest`, or 1 for a lifetime
/// tick. A quote inside a char literal must not open a string.
fn char_literal_len(rest: &str) -> usize {
    let mut chars = rest.char_indices().skip(1);
    match chars.next() {
        Some((_, '\\')) => rest[1..]
            .find('\'')
            .map_or(1, |end| end + 2)
            .min(rest.len()),
        Some((start, ch)) if rest[start + ch.len_utf8()..].starts_with('\'') => {
            start + ch.len_utf8() + 1
        }
        _ => 1,
    }
}

/// The part of `source` before an inline test module (`#[cfg(test)]`, then `mod name {`). A test
/// module that is only declared (`mod tests;`, often with a `#[path]`) holds no text here.
fn production_part(source: &str) -> &str {
    let mut offset = 0;
    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("#[cfg(test)]") {
            let mut ahead = lines.clone();
            let module = ahead.find(|next| !next.trim_start().starts_with("#["));
            if module.is_some_and(|next| {
                let next = next.trim_start();
                (next.starts_with("mod ") || next.starts_with("pub mod ")) && next.ends_with('{')
            }) {
                return &source[..offset];
            }
        }
        offset += line.len() + 1;
    }
    source
}

/// Words that make a clause a prohibition of the thing it names, or a statement that it is not done.
const NEGATIONS: [&str; 8] = [
    "do not ",
    "don't ",
    "never ",
    "must not ",
    "should not ",
    "cannot ",
    "without ",
    "nobody ",
];

/// Where a clause ends: a negation governs a phrase only within its own clause.
const CLAUSE_BREAKS: [char; 6] = ['.', '!', '?', ';', ':', ','];

/// How many hand-entry phrases the string literals of `source` carry as an instruction, case and
/// spacing aside. A phrase counts unless its clause (from the last `.`, `!`, `?`, `;`, `:` or `,`
/// before it) holds a negation, so "Do not create it by hand." is not counted and "It does not get
/// past this check: tell the user and enter it by hand." is.
fn hand_entry_hits(source: &str) -> usize {
    string_literals(production_part(source))
        .iter()
        .map(|literal| {
            let text = literal
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase();
            PHRASES
                .iter()
                .map(|phrase| {
                    text.match_indices(phrase)
                        .filter(|(at, _)| {
                            let before = &text[..*at];
                            let clause = before
                                .rfind(CLAUSE_BREAKS)
                                .map_or(before, |end| &before[end + 1..]);
                            let clause = format!(" {clause}");
                            !NEGATIONS.iter().any(|negation| clause.contains(negation))
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

fn scan(
    directory: &Path,
    relative: &str,
    found: &mut BTreeMap<String, usize>,
    visited: &mut BTreeSet<String>,
) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let relative_path = format!("{relative}/{name}");
        if path.is_dir() {
            if name != "tests" {
                scan(&path, &relative_path, found, visited);
            }
        } else if name.ends_with(".rs") && !name.ends_with("tests.rs") {
            let source = fs::read_to_string(&path).expect("a production source file is readable");
            let relative_path = relative_path.trim_start_matches('/').to_string();
            visited.insert(relative_path.clone());
            let hits = hand_entry_hits(&source);
            if hits > 0 {
                found.insert(relative_path, hits);
            }
        }
    }
}

/// The hits per file, and every production file the walk read (so an unreadable or moved tree is
/// told from a tree with no hand-entry texts).
fn production_hits() -> (BTreeMap<String, usize>, BTreeSet<String>) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut found = BTreeMap::new();
    let mut visited = BTreeSet::new();
    scan(&root.join("src"), "src", &mut found, &mut visited);
    let mut crates: Vec<_> = fs::read_dir(root.join("crates"))
        .expect("the crates directory is readable")
        .flatten()
        .collect();
    crates.sort_by_key(|entry| entry.file_name());
    for krate in crates {
        let name = krate.file_name().to_string_lossy().into_owned();
        scan(
            &krate.path().join("src"),
            &format!("crates/{name}/src"),
            &mut found,
            &mut visited,
        );
    }
    (found, visited)
}

#[test]
fn the_scan_reads_string_literals_and_nothing_else() {
    // Controls: each shape the scan must see, and each it must skip, so that "no hits" can never
    // mean "the scan read nothing".
    assert_eq!(hand_entry_hits(r#"const A: &str = "enter it by hand";"#), 1);
    assert_eq!(
        hand_entry_hits("const A: &str = \"Use the Entry\n   Screen\";"),
        1,
        "spacing and case are set aside"
    );
    assert_eq!(
        hand_entry_hits("const A: &str = \"use the entry \\\n     screen\";"),
        1,
        "a line-continued literal is one text"
    );
    assert_eq!(
        hand_entry_hits(r##"const A: &str = r#"Gateway of Tally "by hand""#;"##),
        2
    );
    assert_eq!(hand_entry_hits("const A: &str = \"Type it MANUALLY\";"), 1);
    assert_eq!(
        hand_entry_hits("// enter it by hand\n/* manually */\nconst A: u8 = 1;"),
        0
    );
    assert_eq!(hand_entry_hits("/// by hand\nfn f() {}"), 0);
    assert_eq!(
        hand_entry_hits("const A: &str = \"Do not create the ledger in Tally by hand.\";"),
        0,
        "a sentence that forbids it is not an instruction"
    );
    assert_eq!(
        hand_entry_hits("const A: &str = \"Do not retry. Enter it by hand.\";"),
        1,
        "the next sentence is judged on its own"
    );
    assert_eq!(
        hand_entry_hits("const A: &str = \"Nobody has to type it manually; type it by hand.\";"),
        1,
        "a semicolon ends the sentence"
    );
    assert_eq!(
        hand_entry_hits("const A: &str = \"Do not post, then key it by hand or manually.\";"),
        2,
        "a comma ends the clause: the negation does not reach past it"
    );
    assert_eq!(
        hand_entry_hits(
            "const A: &str = \"A new number does not get past this check: tell the user and enter it in Tally by hand.\";"
        ),
        1,
        "a conditional instruction after a colon is an instruction"
    );
    assert_eq!(
        hand_entry_hits(
            "const A: &str = \"It cannot post it while that batch is unconfirmed: enter it in Tally by hand.\";"
        ),
        1
    );
    assert_eq!(
        hand_entry_hits("const A: &str = \"Never enter it by hand, and do not do it manually.\";"),
        0,
        "each clause holds its own negation"
    );
    assert_eq!(
        hand_entry_hits("const Q: char = '\"';\nconst A: &str = \"by hand\";"),
        1,
        "a quote inside a char literal does not open a string"
    );
    assert_eq!(
        hand_entry_hits("fn f<'a>(x: &'a str) -> &'a str { \"by hand\" }"),
        1,
        "a lifetime tick does not hide the next string"
    );
    assert_eq!(
        hand_entry_hits(
            "const A: &str = \"fine\";\n#[cfg(test)]\nmod tests {\n    const B: &str = \"by hand\";\n}"
        ),
        0,
        "text inside an inline cfg(test) module never counts"
    );
    assert_eq!(
        hand_entry_hits(
            "#[cfg(test)]\n#[path = \"x_tests.rs\"]\nmod tests;\nconst A: &str = \"by hand\";"
        ),
        1,
        "a declared test module does not end the production text"
    );
    assert_eq!(hand_entry_hits("fn driver() { let _ = \"by hand\"; }"), 1);
}

#[test]
fn texts_that_send_a_person_to_key_vouchers_by_hand_only_shrink() {
    let (found, visited) = production_hits();
    let pinned: BTreeMap<&str, usize> = PINNED.iter().copied().collect();
    let mut problems = Vec::new();
    // The floor: a scan that read nothing would find nothing. Every pinned file must have been read
    // (it may have fewer hits than its pin, or none), and the walk must have read the tree.
    for file in pinned.keys() {
        if !visited.contains(*file) {
            problems.push(format!(
                "{file}: pinned but not read by the scan (moved, or the tree is unreadable)"
            ));
        }
    }
    if visited.len() < 100 {
        problems.push(format!(
            "the scan read only {} production files",
            visited.len()
        ));
    }
    for (file, hits) in &found {
        match pinned.get(file.as_str()) {
            None => problems.push(format!("{file}: {hits} hand-entry text(s), none allowed")),
            Some(pin) if hits > pin => problems.push(format!("{file}: {hits} > at most {pin}")),
            Some(_) => {}
        }
    }
    let table: String = found
        .iter()
        .map(|(file, hits)| format!("    (\"{file}\", {hits}),\n"))
        .collect();
    assert!(
        problems.is_empty(),
        "A refusal, next step or description must not send a person to key or import vouchers \
         in Tally by hand (#1709). The pins are upper bounds and may only be lowered.\n{}\nPins as \
         they stand now:\n{table}",
        problems.join("\n")
    );
}
