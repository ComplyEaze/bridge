//! The company variable has one writer (#2397, IMPLEMENTATION_GUIDE §8, rule I2).
//!
//! `ValidatedCompanyName::xml_static_variables` in the protocol crate renders the
//! `STATICVARIABLES` block and its `SVCURRENTCOMPANY`. These scans hold that in place: no other
//! production file writes either, the unvalidated seam beside the writer is used by no more
//! builders than today, and each file calls the writer exactly as often as its pinned count, so
//! a new call of the writer fails until it is counted. A new entry point that reaches an
//! existing call through a shared core adds no call and is not caught here.
//!
//! Both scans read the production sources that `hand_entry_texts_tests` reads (`src` and each
//! `crates/*/src`), and also `build.rs`, `examples` and each `crates/*/examples` and
//! `crates/*/benches`. They leave out `*tests.rs` files, `tests` directories and the body of
//! every inline `#[cfg(test)] mod name { … }`. Comments are never read. The seam count reads code
//! with every comment and literal blanked; the writer scan reads string literals only.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::hand_entry_texts_tests::{lex, production_part, production_sources, scan};

/// The one file allowed to write the block.
const WRITER: &str = "crates/bridge-tally-protocol/src/xml_read_profiles.rs";

const SEAM: &str = "static_variables_for_unvalidated_company";

/// The most production uses of the seam there may be. It may only fall: each builder moved to
/// `&ValidatedCompanyName` lowers it. At zero the seam and this test are deleted together (#2397).
/// It rose once, from 27 to 28, when the invoice window read (#2423) added a builder that takes
/// `&str` and was moved onto the seam rather than left writing the block itself. It fell to 27
/// when the trial balance builders took `&ValidatedCompanyName`, to 26 when the built-in
/// report envelope did, to 18 when the outstandings builders did, and to 14 when the census,
/// inventory-flags and company-features builders did, to 11 when the masters, party-bills and
/// company-book-extent builders did, to 4 when the invoice reads did, and to 3 when the direct
/// company bootstrap rendered the standard ledger identity profile and the compatibility pair
/// that wrapped it was deleted.
const SEAM_CEILING: usize = 3;

/// The string literals of [`WRITER`] that open the block, exactly: the private writer's template
/// and the company-list template, which names no company.
const WRITER_LITERALS: usize = 2;

/// The request-byte tables. Every builder whose request reaches the writer is meant to have a row
/// in one of them, or to be listed in [`NOT_PINNED`]; review keeps that, not a test.
const TABLES: [&str; 2] = [
    "crates/bridge-tally-protocol/src/request_bytes_tests.rs",
    "src/agent_import_request_bytes_tests.rs",
];

/// Builders whose request reaches the writer, directly or through a shared envelope, and that
/// have no row in [`TABLES`], with their file and why. The test checks only that each one is
/// defined where listed and has no row.
const NOT_PINNED: &[(&str, &str, &str)] = &[(
    "render_voucher_batch_xml",
    "src/agent_lab_import.rs",
    "it mints a fresh UUID for each voucher, so its bytes differ on every call",
)];

/// The production calls of the writer each file holds, exactly: calls of
/// `xml_static_variables`, of the seam, and of the private `static_variables`. In [`WRITER`]
/// the last includes the two calls through which the typed method and the seam reach it. A
/// builder moved from the seam to `xml_static_variables` leaves its file's count unchanged.
const WRITER_CALLS: &[(&str, usize)] = &[
    ("crates/bridge-tally-protocol/src/import_envelope.rs", 1),
    ("crates/bridge-tally-protocol/src/ledger_census.rs", 2),
    (
        "crates/bridge-tally-protocol/src/native_company_features.rs",
        1,
    ),
    ("crates/bridge-tally-protocol/src/native_masters.rs", 2),
    (
        "crates/bridge-tally-protocol/src/native_outstandings/party_bills.rs",
        1,
    ),
    (
        "crates/bridge-tally-protocol/src/native_outstandings/request.rs",
        8,
    ),
    (
        "crates/bridge-tally-protocol/src/native_statement_reports.rs",
        1,
    ),
    (
        "crates/bridge-tally-protocol/src/native_stock_summary.rs",
        1,
    ),
    (
        "crates/bridge-tally-protocol/src/native_trial_balance/mod.rs",
        1,
    ),
    ("crates/bridge-tally-protocol/src/outstandings_shared.rs", 1),
    ("crates/bridge-tally-protocol/src/xml_read_profiles.rs", 7),
    ("src/agent_import.rs", 1),
    ("src/agent_import_invoice_wire.rs", 7),
    ("src/agent_lab.rs", 2),
    ("src/agent_lab_import.rs", 2),
    ("src/agent_read_profiles.rs", 4),
];

/// Production files whose string literals name the variable without writing a request, with
/// the most such literals each may hold and why.
const NAMED_NOT_WRITTEN: &[(&str, usize, &str)] = &[
    (
        "src/agent_import.rs",
        1,
        "the import tool's description names the variable in its warning",
    ),
    (
        "src/tally/runtime.rs",
        2,
        "request_scopes_company reads the variable a finished request names",
    ),
];

/// The identifier tokens of blanked `code`, with each `;` kept as a token so a `use` item ends.
fn tokens(code: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut start = None;
    for (at, ch) in code.char_indices() {
        let word = ch.is_ascii_alphanumeric() || ch == '_';
        match (word, start) {
            (true, None) => start = Some(at),
            (false, Some(from)) => {
                tokens.push(&code[from..at]);
                start = None;
            }
            _ => {}
        }
        if ch == ';' {
            tokens.push(";");
        }
    }
    if let Some(from) = start {
        tokens.push(&code[from..]);
    }
    tokens
}

/// The names the seam is imported or re-exported under (`SEAM as name`) in blanked `code`.
fn aliases(code: &str) -> BTreeSet<String> {
    tokens(code)
        .windows(3)
        .filter(|window| window[0] == SEAM && window[1] == "as")
        .map(|window| window[2].to_string())
        .collect()
}

/// `(uses, definitions)` of the seam in blanked `code`: a use is the seam's name, or a name in
/// `aliases`, anywhere outside a `use` item, so a call, a path call and a function value each
/// count once and an import counts nothing. `fn` followed by the name is the definition.
fn seam_uses(code: &str, aliases: &BTreeSet<String>) -> (usize, usize) {
    let tokens = tokens(code);
    let (mut uses, mut definitions, mut in_use) = (0, 0, false);
    for (at, token) in tokens.iter().enumerate() {
        match *token {
            "use" => in_use = true,
            ";" => in_use = false,
            name if !in_use && (name == SEAM || aliases.contains(name)) => {
                if at > 0 && tokens[at - 1] == "fn" {
                    definitions += 1;
                } else {
                    uses += 1;
                }
            }
            _ => {}
        }
    }
    (uses, definitions)
}

/// Calls of the writer in blanked `code`: the seam's [`seam_uses`], each `xml_static_variables`
/// not after `fn`, and each `static_variables` followed by `(` and not after `fn`. A local of
/// that name is not a call; the private writer passed as a function value is not counted
/// either, and only its own file could do that.
fn writer_calls(code: &str, aliases: &BTreeSet<String>) -> usize {
    let mut words: Vec<(&str, usize)> = Vec::new();
    let mut start = None;
    for (at, ch) in code.char_indices().chain([(code.len(), ' ')]) {
        match (ch.is_ascii_alphanumeric() || ch == '_', start) {
            (true, None) => start = Some(at),
            (false, Some(from)) => {
                words.push((&code[from..at], at));
                start = None;
            }
            _ => {}
        }
    }
    let typed_or_private = words
        .iter()
        .enumerate()
        .filter(|(at, (word, end))| {
            let after_fn = *at > 0 && words[at - 1].0 == "fn";
            let called = code[*end..].trim_start().starts_with('(');
            !after_fn
                && (*word == "xml_static_variables" || (*word == "static_variables" && called))
        })
        .count();
    seam_uses(code, aliases).0 + typed_or_private
}

/// How many string literals of `source` name the variable or open the block: `<SVCURRENTCOMPANY`
/// or `<STATICVARIABLES` in any case, or `SVCURRENTCOMPANY` as spelt. A TDL formula's
/// `##SVCurrentCompany` reads the variable and is not counted.
fn writer_hits(source: &str) -> usize {
    lex(&production_part(source))
        .1
        .iter()
        .filter(|literal| {
            let lower = literal.to_ascii_lowercase();
            lower.contains("<svcurrentcompany")
                || lower.contains("<staticvariables")
                || literal.contains("SVCURRENTCOMPANY")
        })
        .count()
}

fn blanked(source: &str) -> String {
    lex(&production_part(source)).0
}

/// The files both scans read: [`production_sources`], then `build.rs`, `examples`, and each
/// `crates/*/examples` and `crates/*/benches` that exists, under the same exclusions.
fn sources() -> BTreeMap<String, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = production_sources();
    let build = fs::read_to_string(root.join("build.rs")).expect("build.rs is readable");
    sources.insert("build.rs".to_string(), build);
    scan(&root.join("examples"), "examples", &mut sources);
    let mut crates: Vec<_> = fs::read_dir(root.join("crates"))
        .expect("the crates directory is readable")
        .flatten()
        .collect();
    crates.sort_by_key(|entry| entry.file_name());
    for krate in crates {
        let name = krate.file_name().to_string_lossy().into_owned();
        for directory in ["examples", "benches"] {
            scan(
                &krate.path().join(directory),
                &format!("crates/{name}/{directory}"),
                &mut sources,
            );
        }
    }
    sources
}

/// What a scan over `sources` must have read before "nothing found" means anything: at least 100
/// files, the writer, `build.rs` and one example among them.
fn unread(sources: &BTreeMap<String, String>) -> Option<String> {
    let examples = sources
        .keys()
        .filter(|path| path.contains("examples/"))
        .count();
    (sources.len() < 100
        || !sources.contains_key(WRITER)
        || !sources.contains_key("build.rs")
        || examples == 0)
        .then(|| {
            format!(
                "the scan read {} files and {examples} example(s); it must read at least 100, the \
                 writer, build.rs and the examples among them",
                sources.len()
            )
        })
}

#[test]
fn the_scans_count_what_they_say() {
    let none = BTreeSet::new();
    let use_of = |code: &str| seam_uses(&blanked(code), &aliases(&blanked(code)));
    assert_eq!(
        use_of("fn f() { static_variables_for_unvalidated_company(a, b, c); }"),
        (1, 0)
    );
    assert_eq!(
        use_of("fn f() { x::compatibility::static_variables_for_unvalidated_company(a, b, c) }"),
        (1, 0),
        "a path call is a use"
    );
    assert_eq!(
        use_of("use x::compatibility::static_variables_for_unvalidated_company;\n"),
        (0, 0),
        "an import alone is not a use"
    );
    assert_eq!(
        use_of("use x::{static_variables_for_unvalidated_company as sv};\nfn f() { sv(a, b, c); }"),
        (1, 0),
        "a call through an alias is a use"
    );
    assert_eq!(
        use_of("fn f() { let g = static_variables_for_unvalidated_company; }"),
        (1, 0),
        "a function value is a use"
    );
    assert_eq!(
        use_of("pub fn static_variables_for_unvalidated_company(a: &str) -> String { a.into() }"),
        (0, 1)
    );
    assert_eq!(
        use_of(
            "// static_variables_for_unvalidated_company(a)\n/// static_variables_for_unvalidated_company\nconst A: &str = \"static_variables_for_unvalidated_company(\";"
        ),
        (0, 0),
        "comments and literals are not code"
    );
    assert_eq!(
        seam_uses(
            &blanked("fn f() {}\n#[cfg(test)]\nmod tests {\n    fn g() { static_variables_for_unvalidated_company(a) }\n}"),
            &none
        ),
        (0, 0),
        "an inline test module is not production"
    );

    let calls = |code: &str| writer_calls(&blanked(code), &aliases(&blanked(code)));
    assert_eq!(
        calls("fn f(c: &C) -> String { c.xml_static_variables(\"\", \"\") }"),
        1
    );
    assert_eq!(
        calls(
            "fn f() { let static_variables = c.xml_static_variables(a, b); g(&static_variables) }"
        ),
        1,
        "a local named like the private writer is not a call"
    );
    assert_eq!(
        calls(
            "fn static_variables(a: &str) -> String { a.into() }\nfn f() { static_variables(a) }"
        ),
        1,
        "a definition is not a call"
    );
    assert_eq!(
        calls("pub fn xml_static_variables(&self) -> String { super::static_variables(a, b, c) }"),
        1
    );
    assert_eq!(
        calls("use x::{static_variables_for_unvalidated_company as sv};\nfn f() { sv(a, b, c); }"),
        1,
        "a seam call through an alias is a call"
    );
    assert_eq!(
        calls("// c.xml_static_variables(a, b)\nconst A: &str = \"static_variables(\";"),
        0,
        "comments and literals are not code"
    );

    assert_eq!(
        writer_hits("const A: &str = \"<STATICVARIABLES><SVCURRENTCOMPANY>\";"),
        1
    );
    assert_eq!(
        writer_hits("const A: &str = r#\"<svCurrentCompany>{}\"#;"),
        1
    );
    assert_eq!(writer_hits("const A: &[u8] = b\"SVCURRENTCOMPANY\";"), 1);
    assert_eq!(
        writer_hits("const A: &str = \"<COMPUTE>X:$GUID:Company:##SVCurrentCompany</COMPUTE>\";"),
        0,
        "a TDL formula reading the variable writes nothing"
    );
    assert_eq!(
        writer_hits("/// <STATICVARIABLES> SVCURRENTCOMPANY\n// SVCURRENTCOMPANY\nfn f() {}"),
        0,
        "comments are not read"
    );
}

#[test]
fn the_unvalidated_company_seam_is_used_no_more_than_its_ceiling() {
    let sources = sources();
    let blanked: BTreeMap<&str, String> = sources
        .iter()
        .map(|(path, source)| (path.as_str(), blanked(source)))
        .collect();
    let aliases: BTreeSet<String> = blanked.values().flat_map(|code| aliases(code)).collect();
    let mut per_file = BTreeMap::new();
    let mut definitions = Vec::new();
    for (path, code) in &blanked {
        let (uses, defined) = seam_uses(code, &aliases);
        if uses > 0 {
            per_file.insert(*path, uses);
        }
        if defined > 0 {
            definitions.push((*path, defined));
        }
    }
    let total: usize = per_file.values().sum();
    if let Some(problem) = unread(&sources) {
        panic!("{problem}");
    }
    assert_eq!(
        definitions,
        vec![(WRITER, 1)],
        "the scan must find the seam's one definition, in the writer's file"
    );
    let table: String = per_file
        .iter()
        .map(|(path, uses)| format!("    {path}: {uses}\n"))
        .collect();
    assert!(
        total > 0,
        "the seam has no use left: delete it and this test together (#2397)"
    );
    assert!(
        total <= SEAM_CEILING,
        "{total} production uses of `{SEAM}`, at most {SEAM_CEILING}. A builder takes \
         `&ValidatedCompanyName` and calls `xml_static_variables`; the seam is only for the \
         builders that still take `&str` (#2397). Uses now:\n{table}"
    );
}

#[test]
fn no_production_file_but_the_writer_writes_the_company_variable() {
    let sources = sources();
    let allowed: BTreeMap<&str, usize> = NAMED_NOT_WRITTEN
        .iter()
        .map(|(path, most, _)| (*path, *most))
        .collect();
    let mut problems: Vec<String> = unread(&sources).into_iter().collect();
    match sources.get(WRITER) {
        Some(writer) if writer_hits(writer) == WRITER_LITERALS => {}
        Some(writer) => problems.push(format!(
            "{WRITER}: {} literal(s) open the block, exactly {WRITER_LITERALS} expected: a new \
             template there needs a row in the request-byte tables {TABLES:?}",
            writer_hits(writer)
        )),
        None => problems.push(format!("{WRITER}: not read, so the scan proves nothing")),
    }
    for path in allowed.keys() {
        if !sources.contains_key(*path) {
            problems.push(format!(
                "{path}: allowed but not read (moved, or the tree is unreadable)"
            ));
        }
    }
    for (path, source) in &sources {
        if path == WRITER {
            continue;
        }
        let hits = writer_hits(source);
        match allowed.get(path.as_str()) {
            _ if hits == 0 => {}
            Some(most) if hits <= *most => {}
            Some(most) => problems.push(format!("{path}: {hits} literal(s), at most {most}")),
            None => problems.push(format!("{path}: {hits} literal(s), none allowed")),
        }
    }
    assert!(
        problems.is_empty(),
        "Only {WRITER} writes STATICVARIABLES and SVCURRENTCOMPANY: render the block with \
         `ValidatedCompanyName::xml_static_variables` (#2397).\n{}",
        problems.join("\n")
    );
}

#[test]
fn each_file_calls_the_writer_as_often_as_pinned() {
    let sources = sources();
    if let Some(problem) = unread(&sources) {
        panic!("{problem}");
    }
    let blanked: BTreeMap<&str, String> = sources
        .iter()
        .map(|(path, source)| (path.as_str(), blanked(source)))
        .collect();
    let aliases: BTreeSet<String> = blanked.values().flat_map(|code| aliases(code)).collect();
    let found: BTreeMap<&str, usize> = blanked
        .iter()
        .map(|(path, code)| (*path, writer_calls(code, &aliases)))
        .filter(|(_, calls)| *calls > 0)
        .collect();
    let pinned: BTreeMap<&str, usize> = WRITER_CALLS.iter().copied().collect();
    assert_eq!(
        pinned.len(),
        WRITER_CALLS.len(),
        "WRITER_CALLS names a file twice"
    );
    assert!(
        found.values().sum::<usize>() > 0,
        "no call of the writer found, so the scan proves nothing"
    );
    if found != pinned {
        let table: String = found
            .iter()
            .map(|(path, calls)| format!("    ({path:?}, {calls}),\n"))
            .collect();
        panic!(
            "the production calls of the company-variable writer changed (#2397). Give each new \
             builder a row in one of {TABLES:?}, or list it in NOT_PINNED with its reason, then \
             set WRITER_CALLS to the calls found now:\n{table}"
        );
    }
}

#[test]
fn each_unpinned_builder_is_defined_where_listed_and_has_no_row() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sources = sources();
    let tables: Vec<(&str, String)> = TABLES
        .iter()
        .map(|path| {
            let table = fs::read_to_string(root.join(path))
                .unwrap_or_else(|error| panic!("{path} is unreadable: {error}"));
            assert!(
                table.contains("const TABLE: &[(&str, &str)]"),
                "{path}: no table found"
            );
            (*path, table)
        })
        .collect();
    for (builder, path, _) in NOT_PINNED {
        let code = blanked(
            sources
                .get(*path)
                .unwrap_or_else(|| panic!("{path} is not read")),
        );
        assert!(
            tokens(&code)
                .windows(2)
                .any(|window| window == ["fn", *builder]),
            "{path} defines no `{builder}`: update NOT_PINNED"
        );
        for (table_path, table) in &tables {
            assert!(
                !table.contains(&format!("\"{builder}")),
                "`{builder}` has a row in {table_path}: remove it from NOT_PINNED"
            );
        }
    }
}
