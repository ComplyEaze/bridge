//! bridge#583: the scripted answer to the native "approve this Journal" dialog
//! exists only in this crate's own unit tests. This gate holds the source,
//! build configuration and workflows to that; the shipped-binary scan
//! (`scripts/check-no-test-seam.mjs`) proves the result in every artefact.
//!
//! It runs as an integration test, which links the non-test library, so it
//! reads files rather than naming the seam: the seam does not exist here.
//! Each check is a function over text, driven below both on the real files
//! and on a broken copy it must reject, so a check that stops looking fails.
use std::fs;
use std::path::{Path, PathBuf};

const SEAM_ITEMS: [&str; 4] = [
    "test_seam",
    "SCRIPTED_APPROVAL",
    "ScriptedApproval",
    "SEAM_MARKER",
];

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri has a parent directory (the repo root)")
        .to_path_buf()
}

fn read(path: impl AsRef<Path>) -> String {
    let path = repo().join(path);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// Problems with how `approved_import.rs` gates the seam: the non-test arm
/// must be exactly the unchanged function, the test arm and module bare
/// `#[cfg(test)]`, and nothing in the file may widen a gate.
fn seam_gate_problems(source: &str) -> Vec<String> {
    let lines: Vec<&str> = source.lines().map(str::trim).collect();
    let mut problems = Vec::new();
    // The attribute governing an item: the nearest line above it that is not
    // a comment, so a doc comment between the two changes nothing.
    let attribute_before = |needle: &str| {
        let index = lines.iter().position(|line| *line == needle)?;
        lines[..index]
            .iter()
            .rev()
            .find(|line| !line.starts_with("//"))
            .copied()
    };
    for (item, gate) in [
        ("use confirm as approve;", "#[cfg(not(test))]"),
        ("use test_seam::approve;", "#[cfg(test)]"),
        // The review dialog for a doubted post (#239), gated the same way.
        ("use confirm_review as approve_review;", "#[cfg(not(test))]"),
        ("use test_seam::approve_review;", "#[cfg(test)]"),
        // The one-ledger dialog for a master approval, gated the same way.
        (
            "use confirm_masters_dialog as approve_masters;",
            "#[cfg(not(test))]",
        ),
        ("use test_seam::approve_masters;", "#[cfg(test)]"),
        // What a post dialog's own task runs under (#725), gated the same way.
        (
            "use carry_nothing as carry_approval_scope;",
            "#[cfg(not(test))]",
        ),
        ("use test_seam::carry_approval_scope;", "#[cfg(test)]"),
        ("pub(crate) mod test_seam {", "#[cfg(test)]"),
    ] {
        if lines.iter().filter(|line| **line == item).count() != 1 {
            problems.push(format!("expected exactly one `{item}`"));
        } else if attribute_before(item) != Some(gate) {
            problems.push(format!("`{item}` must sit directly under `{gate}`"));
        }
    }
    // Each dialog is asked from exactly one place, and each answer builds only
    // its own type: a post approval and a master approval are an
    // `ApprovedImport` (of their own kind), a review a `ReviewAcknowledged`,
    // and nothing converts one into the other.
    for call_site in [
        "approve(count, preview).await?;\n        Ok(Self {",
        "approve_review(kind, count, preview).await?;\n        Ok(Self(()))",
        "approve_masters(preview).await?;\n        Ok(Self {\n            kind: ApprovedKind::Masters(",
    ] {
        if source.matches(call_site).count() != 1 {
            problems.push(format!("expected exactly one call site `{call_site}`"));
        }
    }
    for call in [
        "approve(count, preview)",
        "approve_review(kind, count, preview)",
        "approve_masters(preview)",
    ] {
        if source.matches(call).count() != 1 {
            problems.push(format!("`{call}` must be called exactly once"));
        }
    }
    if source.contains("impl From<") || source.contains("impl Into<") {
        problems.push("no conversion may exist between the approval types".into());
    }
    // A dialog outliving its call (#725) runs on one task, spawned in one
    // place, under the gated scope; outside tests that scope carries nothing.
    if source.matches("tokio::spawn(").count() != 1
        || source
            .matches("task: tokio::spawn(carry_approval_scope(dialog)),")
            .count()
            != 1
    {
        problems
            .push("a post dialog's task must be spawned once, under carry_approval_scope".into());
    }
    if source
        .matches("#[cfg(not(test))]\nfn carry_nothing<F>(dialog: F) -> F {\n    dialog\n}")
        .count()
        != 1
    {
        problems.push("outside tests, carry_approval_scope must carry nothing".into());
    }
    // Every other way to run code off the calling task, counted (#725): none
    // but the one dialog task above. The one `.spawn(` is a process, the
    // dialog's child (#702 removed the unit tests' `sh` writer), and the bare
    // word is counted too, as the cfg census is, so a form nobody listed is
    // caught.
    for (form, expected) in [
        ("tokio::task::spawn(", 0),
        ("task::spawn(", 0),
        ("spawn_blocking(", 0),
        ("spawn_local(", 0),
        ("JoinSet", 0),
        ("Handle::current()", 0),
        (".spawn(", 1),
        ("spawn", 9),
    ] {
        if source.matches(form).count() != expected {
            problems.push(format!("expected {expected} of `{form}`"));
        }
    }
    problems.extend(widened_test_gates(source));
    problems
}

/// Any attribute that names `test` inside a cfg (`any(test, ..)`,
/// `cfg_attr(test, ..)`) other than a bare `#[cfg(test)]` or
/// `#[cfg(not(test))]` could widen or relocate a seam's gate, including one on
/// an enclosing block or wrapped over several lines, so each attribute is read
/// whole, from `#[` to its closing bracket. Unrelated cfgs are not a seam's
/// business.
fn widened_test_gates(source: &str) -> Vec<String> {
    attributes(source)
        .into_iter()
        .filter(|attribute| {
            attribute.contains("cfg")
                && (attribute.contains("(test") || attribute.contains(",test"))
                && !matches!(attribute.as_str(), "#[cfg(test)]" | "#[cfg(not(test))]")
        })
        .map(|attribute| format!("`{attribute}` could widen the seam's gate"))
        .collect()
}

/// The post's test-only REMOTEID seam (`SCRIPTED_REMOTE_ID`) lives in the post
/// path itself, so that file may carry only bare `cfg(test)` gates, and the
/// seam's two items must each sit under one.
fn remote_id_seam_problems(source: &str) -> Vec<String> {
    let mut problems = widened_test_gates(source);
    let lines: Vec<&str> = source.lines().map(str::trim).collect();
    for item in [
        "tokio::task_local! {",
        "if let Ok(scripted) = SCRIPTED_REMOTE_ID.try_with(|id| *id) {",
        "if let Ok(scripted) = SCRIPTED_REMOTE_IDS.try_with(Clone::clone) {",
    ] {
        let at = lines.iter().position(|line| *line == item);
        if at
            .and_then(|at| at.checked_sub(1))
            .map(|above| lines[above])
            != Some("#[cfg(test)]")
        {
            problems.push(format!("`{item}` must sit directly under `#[cfg(test)]`"));
        }
    }
    problems
}

#[test]
fn the_remote_id_seam_is_gated_by_bare_cfg_test() {
    let source = read("src-tauri/src/agent_import_post.rs");
    assert_eq!(remote_id_seam_problems(&source), Vec::<String>::new());
    for broken in [
        source.replacen(
            "#[cfg(test)]\ntokio::task_local! {",
            "#[cfg(any(test, feature = \"remote-id-seam\"))]\ntokio::task_local! {",
            1,
        ),
        source.replacen(
            "    #[cfg(test)]\n    if let Ok(scripted)",
            "    if let Ok(scripted)",
            1,
        ),
        source.replacen(
            "        #[cfg(test)]\n        if let Ok(scripted) = SCRIPTED_REMOTE_IDS",
            "        if let Ok(scripted) = SCRIPTED_REMOTE_IDS",
            1,
        ),
    ] {
        assert_ne!(
            broken, source,
            "the fabricated breakage must change the source"
        );
        assert!(!remote_id_seam_problems(&broken).is_empty());
    }
}

/// Every outer or inner attribute in `source`, read from `#[` or `#![` to the
/// bracket that closes it, however many lines it spans. Whitespace is removed
/// and string literals are kept only as `""`: a bracket inside a string (say
/// `feature = "x]y"`) neither ends the attribute nor hides what follows it,
/// and a word inside a string is not a cfg predicate.
fn attributes(source: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut current: Option<String> = None;
    let (mut depth, mut in_string, mut escaped) = (0_i32, false, false);
    for line in source.lines() {
        let trimmed = line.trim();
        if current.is_none() && (trimmed.starts_with("#[") || trimmed.starts_with("#![")) {
            current = Some(String::new());
            depth = 0;
        }
        let Some(text) = current.as_mut() else {
            continue;
        };
        for character in trimmed.chars() {
            if in_string {
                match (escaped, character) {
                    (true, _) => escaped = false,
                    (false, '\\') => escaped = true,
                    (false, '"') => {
                        in_string = false;
                        text.push('"');
                    }
                    _ => {}
                }
                continue;
            }
            match character {
                '"' => {
                    in_string = true;
                    text.push('"');
                }
                '[' => {
                    depth += 1;
                    text.push('[');
                }
                ']' => {
                    depth -= 1;
                    text.push(']');
                }
                c if c.is_whitespace() => {}
                c => text.push(c),
            }
            if depth == 0 && character == ']' {
                break;
            }
        }
        if depth == 0 && !in_string {
            found.push(current.take().unwrap());
        }
    }
    found
}

/// Files outside `approved_import.rs` and `*_tests.rs` that name the seam.
fn seam_users_outside_tests(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).expect("readable source directory") {
            let path = entry.expect("readable entry").path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if !name.ends_with(".rs") || name == "approved_import.rs" || name.ends_with("_tests.rs")
            {
                continue;
            }
            let text = fs::read_to_string(&path).expect("UTF-8 source");
            if SEAM_ITEMS.iter().any(|item| text.contains(item)) {
                found.push(path);
            }
        }
    }
    found
}

/// The tracked files a build takes its configuration from (bridge#836): every workflow and
/// composite action, every `.cargo/config` or `.cargo/config.toml` at any depth, every
/// `build.rs`, the root `package.json`, and the scripts and Dockerfile that start cargo.
fn build_configuration_files(tracked: &[String]) -> Vec<&str> {
    const STARTS_CARGO: [&str; 4] = [
        "package.json",
        "scripts/check-no-test-seam.mjs",
        "scripts/package-mcpb.mjs",
        "glama/Dockerfile",
    ];
    tracked
        .iter()
        .map(String::as_str)
        .filter(|path| {
            let (directory, name) = path.rsplit_once('/').unwrap_or(("", path));
            path.starts_with(".github/workflows/")
                || path.starts_with(".github/actions/")
                || name == "build.rs"
                || (matches!(name, "config" | "config.toml")
                    && (directory == ".cargo" || directory.ends_with("/.cargo")))
                || STARTS_CARGO.contains(path)
        })
        .collect()
}

/// The repository's tracked files: what a build can be given, unlike an untracked directory or
/// a second checkout under the root.
fn tracked_files() -> Vec<String> {
    #[expect(
        clippy::disallowed_methods,
        reason = "the gate lists the repository's tracked files with git; no network is involved"
    )]
    let output = std::process::Command::new("git")
        .args(["ls-files", "-z"])
        .current_dir(repo())
        .output()
        .expect("git ls-files runs");
    assert!(output.status.success(), "git ls-files must succeed");
    String::from_utf8(output.stdout)
        .expect("UTF-8 paths")
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_string)
        .collect()
}

/// A file's text with every run of separators made one space: whitespace, U+001F (the separator
/// of `CARGO_ENCODED_RUSTFLAGS`), quotes, brackets, commas and backslash escapes (`\t`, `\n`,
/// `\r`, `\x1f`, `\u001f`, `\u{1f}`, or a lone backslash). A flag split over lines, quoted or
/// escaped then reads as one phrase.
fn normalised(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut separated = false;
    let mut rest = text;
    while let Some(character) = rest.chars().next() {
        let mut taken = character.len_utf8();
        let separator = if character == '\\' {
            let escape = &rest[1..];
            for spelled in [
                "x1f", "x1F", "u001f", "u001F", "u{1f}", "u{1F}", "t", "n", "r",
            ] {
                if escape.starts_with(spelled) {
                    taken += spelled.len();
                    break;
                }
            }
            true
        } else {
            character.is_whitespace()
                || matches!(character, '\u{1f}' | '"' | '\'' | '`' | '[' | ']' | ',')
        };
        if separator {
            separated = true;
        } else {
            if separated && !out.is_empty() {
                out.push(' ');
            }
            separated = false;
            out.push(character);
        }
        rest = &rest[taken..];
    }
    out
}

/// Whether `text` begins with the whole word `word`: what follows is not part of an identifier.
fn starts_with_word(text: &str, word: &str) -> bool {
    text.strip_prefix(word).is_some_and(|after| {
        !after
            .chars()
            .next()
            .is_some_and(|next| next.is_alphanumeric() || next == '_')
    })
}

/// The same text as a shell joins it: quotes and backslashes removed rather than made
/// separators, so `te"st"`, `te\\st` and `\\test` read as `test`, and every run of whitespace
/// (or U+001F) made one space.
fn joined(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut separated = false;
    for character in text.chars() {
        if matches!(character, '"' | '\'' | '`' | '\\') {
            continue;
        }
        if character.is_whitespace() || character == '\u{1f}' {
            separated = true;
        } else {
            if separated && !out.is_empty() {
                out.push(' ');
            }
            separated = false;
            out.push(character);
        }
    }
    out
}

/// The `--cfg test` and `rustc-cfg=test` settings in one form of a text.
fn settings_in(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (flag, separators) in [("--cfg", " = "), ("rustc-cfg=", " ")] {
        for (at, _) in text.match_indices(flag) {
            let value = text[at + flag.len()..].trim_start_matches(|c| separators.contains(c));
            if starts_with_word(value, "test") {
                let end = (at + 60).min(text.len());
                let end = (end..=text.len())
                    .find(|&i| text.is_char_boundary(i))
                    .unwrap_or(end);
                found.push(text[at..end].to_string());
            }
        }
    }
    found
}

/// Settings that give a build `cfg(test)`: a `--cfg` flag whose value is the word `test`, or a
/// build script's `rustc-cfg=test`. They are read from the whole normalised text, where quotes,
/// brackets and escapes separate words; when that finds none, from the text as a shell joins it,
/// where a word written in pieces is one word. `--cfg testing` is not one.
fn cfg_test_settings(text: &str) -> Vec<String> {
    let found = settings_in(&normalised(text));
    if found.is_empty() {
        settings_in(&joined(text))
    } else {
        found
    }
}

/// A build script's `rustc-cfg` emissions that are not a plain literal cfg name other than
/// `test`: a value built by `format!` (`{}`, `{name}`), split from its key, or `test` itself
/// cannot be judged from the text, so it is refused (bridge#836).
fn build_script_cfg_problems(script: &str) -> Vec<String> {
    let mut problems = Vec::new();
    for (at, _) in script.match_indices("rustc-cfg") {
        let after = &script[at + "rustc-cfg".len()..];
        let value = after.strip_prefix('=').map(|value| {
            let end = value.find('"').unwrap_or(value.len());
            &value[..end]
        });
        let literal = value.is_some_and(|value| {
            let mut characters = value.chars();
            characters
                .next()
                .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
                && characters.all(|c| c.is_ascii_alphanumeric() || c == '_')
                && value != "test"
        });
        if !literal {
            let shown: String = after.chars().take(40).collect();
            problems.push(format!("rustc-cfg{shown}"));
        }
    }
    problems
}

/// A manifest that names its build script: the gate reads only files named `build.rs`, so such
/// a manifest is refused rather than its script left unread. The key is found as TOML spells it
/// (bridge#1399 review): `build` under `[package]` (bare or quoted), the dotted `package.build`
/// at the top of the file (parts bare or quoted), or `build` inside an inline `package = { .. }`.
/// A string value names a script; `build = false` does not.
fn named_build_script(manifest: &str) -> bool {
    let key = |text: &str| -> String {
        text.chars()
            .filter(|c| !c.is_whitespace() && !matches!(c, '"' | '\''))
            .collect()
    };
    let names_a_script = |value: &str| value.trim_start().starts_with(['"', '\'']);
    let mut table = String::new();
    for line in manifest.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(header) = line.strip_prefix('[') {
            let header = header.trim_start_matches('[');
            table = key(header.split(']').next().unwrap_or(""));
            continue;
        }
        let Some((left, value)) = line.split_once('=') else {
            continue;
        };
        let name = if table.is_empty() {
            key(left)
        } else {
            format!("{table}.{}", key(left))
        };
        if name == "package.build" && names_a_script(value) {
            return true;
        }
        if name == "package" {
            let inner = value.trim().trim_start_matches('{').trim_end_matches('}');
            let names = inner
                .split(',')
                .filter_map(|pair| pair.split_once('='))
                .any(|(inner_key, inner_value)| {
                    key(inner_key) == "build" && names_a_script(inner_value)
                });
            if names {
                return true;
            }
        }
    }
    false
}

/// The gate's whole verdict over a tree, given its tracked paths and a reader: the files it
/// examined, and every problem found.
fn build_configuration_verdict(
    tracked: &[String],
    read: impl Fn(&str) -> String,
) -> (Vec<String>, Vec<String>) {
    let mut examined = Vec::new();
    let mut problems = Vec::new();
    for path in build_configuration_files(tracked) {
        let text = read(path);
        for setting in cfg_test_settings(&text) {
            problems.push(format!("{path} sets cfg(test): {setting}"));
        }
        if path.ends_with("build.rs") {
            for emission in build_script_cfg_problems(&text) {
                problems.push(format!(
                    "{path} emits a cfg not a plain literal: {emission}"
                ));
            }
        }
        examined.push(path.to_string());
    }
    for manifest in tracked
        .iter()
        .filter(|path| path.rsplit('/').next() == Some("Cargo.toml"))
    {
        if named_build_script(&read(manifest)) {
            problems.push(format!(
                "{manifest} names its build script; name it build.rs"
            ));
        }
    }
    (examined, problems)
}

#[test]
fn the_seam_is_gated_by_bare_cfg_test_and_its_non_test_arm_is_the_real_dialog() {
    let source = read("src-tauri/src/tally/approved_import.rs");
    assert_eq!(seam_gate_problems(&source), Vec::<String>::new());
    // The check itself: each way of widening or rewiring the gate is caught.
    for broken in [
        source.replace(
            "#[cfg(test)]\npub(crate) mod test_seam {",
            "#[cfg(any(test, feature = \"approval-seam\"))]\npub(crate) mod test_seam {",
        ),
        source.replace(
            "#[cfg(not(test))]\nuse confirm as approve;",
            "#[cfg(not(debug_assertions))]\nuse confirm as approve;",
        ),
        source.replace(
            "approve(count, preview).await?;",
            "confirm(count, preview).await?;",
        ),
        source.replace(
            "approve_review(kind, count, preview).await?;",
            "confirm_review(count, preview).await?;",
        ),
        source.replace(
            "#[cfg(not(test))]\nuse confirm_review as approve_review;",
            "#[cfg(not(debug_assertions))]\nuse confirm_review as approve_review;",
        ),
        source.replace(
            "#[cfg(test)]\nuse test_seam::approve_review;",
            "use test_seam::approve_review;",
        ),
        source.replace(
            "approve_masters(preview).await?;",
            "confirm_masters_dialog(preview).await?;",
        ),
        source.replace(
            "#[cfg(not(test))]\nuse confirm_masters_dialog as approve_masters;",
            "#[cfg(not(debug_assertions))]\nuse confirm_masters_dialog as approve_masters;",
        ),
        source.replace(
            "#[cfg(test)]\nuse test_seam::approve_masters;",
            "use test_seam::approve_masters;",
        ),
        // A master approval must never be answered by the post dialog, nor a
        // post by the one-ledger dialog.
        source.replace(
            "approve_masters(preview).await?;\n        Ok(Self {",
            "approve(ONE_ITEM, preview).await?;\n        Ok(Self {",
        ),
        source.replace(
            "approve(count, preview).await?;\n        Ok(Self {",
            "approve_masters(preview).await?;\n        Ok(Self {",
        ),
        source.replace(
            "approve_review(kind, count, preview).await?;\n        Ok(Self(()))",
            "approve_masters(preview).await?;\n        Ok(Self(()))",
        ),
        // A review answer must never build a post approval.
        source.replace(
            "approve_review(kind, count, preview).await?;\n        Ok(Self(()))",
            "approve(count, preview).await?;\n        Ok(Self(()))",
        ),
        format!(
            "{source}\nimpl From<ReviewAcknowledged> for ApprovedImport {{ fn from(_: ReviewAcknowledged) -> Self {{ unreachable!() }} }}\n"
        ),
        source.replace(
            "#[cfg(test)]\nuse test_seam::approve;",
            "use test_seam::approve;",
        ),
        source.replace(
            "#[cfg(not(test))]\nuse carry_nothing as carry_approval_scope;",
            "#[cfg(not(debug_assertions))]\nuse carry_nothing as carry_approval_scope;",
        ),
        source.replace(
            "#[cfg(test)]\nuse test_seam::carry_approval_scope;",
            "use test_seam::carry_approval_scope;",
        ),
        source.replace(
            "task: tokio::spawn(carry_approval_scope(dialog)),",
            "task: tokio::spawn(test_seam::carry_approval_scope(dialog)),",
        ),
        source.replace(
            "fn carry_nothing<F>(dialog: F) -> F {\n    dialog\n}",
            "fn carry_nothing<F>(dialog: F) -> F {\n    std::hint::black_box(dialog)\n}",
        ),
        format!("{source}\nfn elsewhere() {{ let _ = tokio::spawn(async {{}}); }}\n"),
        format!("{source}\nfn elsewhere() {{ let _ = tokio::task::spawn(async {{}}); }}\n"),
        format!("{source}\nfn elsewhere() {{ let _ = tokio::task::spawn_blocking(|| {{}}); }}\n"),
        format!("{source}\nfn elsewhere() {{ let _ = tokio::task::spawn_local(async {{}}); }}\n"),
        format!("{source}\nfn elsewhere() {{ let _ = tokio::task::JoinSet::<()>::new(); }}\n"),
        format!("{source}\nfn elsewhere() {{ let _ = tokio::runtime::Handle::current(); }}\n"),
        format!("{source}\nfn elsewhere() {{ let _ = std::process::Command::new(\"x\").spawn(); }}\n"),
        format!("{source}\nfn respawned() {{}}\n"),
        source.replace(
            "#[cfg(test)]\nuse test_seam::approve;",
            "#[cfg_attr(test, allow(unused))]\nuse test_seam::approve;",
        ),
        // A bracket inside a string literal must not end the attribute early
        // and hide the `test` that follows it (on an enclosing item, so the
        // seam's own bare `#[cfg(test)]` still sits directly above it).
        source.replace(
            "#[cfg(test)]\npub(crate) mod test_seam {",
            "#[cfg(any(feature = \"x]y\", test))]\nmod scoped {}\n#[cfg(test)]\npub(crate) mod test_seam {",
        ),
        // A widened gate on an enclosing block, wrapped over lines, with the
        // seam's own bare `#[cfg(test)]` left in place inside it.
        source.replace(
            "#[cfg(test)]\npub(crate) mod test_seam {",
            "#[cfg(any(\n    test,\n    feature = \"approval-seam\"\n))]\nmod scoped {}\n#[cfg(test)]\npub(crate) mod test_seam {",
        ),
    ] {
        assert_ne!(
            broken, source,
            "the fabricated breakage must change the source"
        );
        assert!(!seam_gate_problems(&broken).is_empty());
    }
    // Ordinary edits that leave the gate alone pass: a doc comment between an
    // attribute and its item, and an unrelated feature cfg elsewhere.
    for benign in [
        source.replace(
            "#[cfg(test)]\npub(crate) mod test_seam {",
            "#[cfg(test)]\n/// Scripted approvals.\npub(crate) mod test_seam {",
        ),
        format!("{source}\n#[cfg(feature = \"lab-writes\")]\nfn unrelated() {{}}\n"),
        // A word inside a string is not a cfg predicate.
        format!("{source}\n#[cfg(feature = \"test-fixtures\")]\nfn unrelated() {{}}\n"),
    ] {
        assert_ne!(benign, source);
        assert_eq!(seam_gate_problems(&benign), Vec::<String>::new());
    }
}

/// The post path accepts only a post approval: its signature names
/// `ApprovedImport`, so passing a review answer is a compile error (#239).
fn post_path_problems(runtime: &str) -> Vec<String> {
    let signature = "request: super::approved_import::ApprovedImport,";
    let name = "async fn post_approved_import<";
    let start = runtime.find(name).map(|start| start + name.len());
    match start.and_then(|start| {
        runtime[start..]
            .find(')')
            .map(|end| &runtime[start..start + end])
    }) {
        Some(parameters) if parameters.contains(signature) => Vec::new(),
        _ => vec![format!("post_approved_import must take `{signature}`")],
    }
}

#[test]
fn the_post_path_accepts_only_a_post_approval() {
    let runtime = read("src-tauri/src/tally/runtime.rs");
    assert_eq!(post_path_problems(&runtime), Vec::<String>::new());
    let broken = runtime.replace(
        "request: super::approved_import::ApprovedImport,",
        "request: super::approved_import::ReviewAcknowledged,",
    );
    assert_ne!(broken, runtime);
    assert!(!post_path_problems(&broken).is_empty());
}

/// Each subprocess mode runs its own dialog: swapping them would show the
/// post dialog for a review, or answer a review with a post approval (#239).
fn dialog_mode_problems(lib: &str) -> Vec<String> {
    let mut problems = Vec::new();
    for arm in [
        "Some(\"--confirm-journal\") => agent::run_confirmation as fn() -> bool,",
        "Some(\"--confirm-review\") => agent::run_review_confirmation,",
        "Some(\"--confirm-masters\") => agent::run_masters_confirmation,",
        "Some(\"--confirm-release\") => agent::run_release_confirmation,",
        "Some(\"--confirm-settle\") => agent::run_settle_confirmation,",
    ] {
        if lib.matches(arm).count() != 1 {
            problems.push(format!("expected exactly one `{arm}`"));
        }
    }
    problems
}

#[test]
fn each_dialog_mode_runs_its_own_dialog() {
    let lib = read("src-tauri/src/lib.rs");
    assert_eq!(dialog_mode_problems(&lib), Vec::<String>::new());
    let release_as_review = lib.replace(
        "agent::run_release_confirmation,",
        "agent::run_review_confirmation,",
    );
    assert_ne!(release_as_review, lib);
    assert!(!dialog_mode_problems(&release_as_review).is_empty());
    let settle_as_release = lib.replace(
        "agent::run_settle_confirmation,",
        "agent::run_release_confirmation,",
    );
    assert_ne!(settle_as_release, lib);
    assert!(!dialog_mode_problems(&settle_as_release).is_empty());
    let swapped = lib
        .replace("agent::run_confirmation as fn() -> bool,", "SWAP,")
        .replace(
            "agent::run_review_confirmation,",
            "agent::run_confirmation as fn() -> bool,",
        )
        .replace("SWAP,", "agent::run_review_confirmation,");
    assert_ne!(swapped, lib);
    assert!(!dialog_mode_problems(&swapped).is_empty());
    // The one-ledger mode runs only the one-ledger dialog: each other dialog
    // swapped into its arm, and its dialog swapped into another's, is caught.
    for broken in [
        lib.replace(
            "agent::run_masters_confirmation,",
            "agent::run_confirmation as fn() -> bool,",
        ),
        lib.replace(
            "agent::run_masters_confirmation,",
            "agent::run_review_confirmation,",
        ),
        lib.replace(
            "Some(\"--confirm-journal\") => agent::run_confirmation as fn() -> bool,",
            "Some(\"--confirm-journal\") => agent::run_masters_confirmation as fn() -> bool,",
        ),
        lib.replace(
            "Some(\"--confirm-review\") => agent::run_review_confirmation,",
            "Some(\"--confirm-review\") => agent::run_masters_confirmation,",
        ),
        lib.replace("\"--confirm-masters\"", "\"--confirm-ledger\""),
    ] {
        assert_ne!(broken, lib);
        assert!(!dialog_mode_problems(&broken).is_empty());
    }
}

/// The block of the queued post whose errors are marked as refused before the
/// intent (#656) must hold no intent and no POST: a refusal from inside it is
/// reported as "nothing was sent". So in `post_approved_import` the block
/// opens once, closes into `PreIntentQueueRefusal` once, holds neither
/// `before_dispatch()` nor `post_probe_xml(`, and both follow it in that order.
fn pre_intent_block_problems(runtime: &str) -> Vec<String> {
    let open = "let (admission_evidence, before_marks) = async {";
    let close = "PreIntentQueueRefusal { source }";
    let Some(function) = runtime.find("async fn post_approved_import<") else {
        return vec!["post_approved_import not found".into()];
    };
    let body = &runtime[function..];
    let body = &body[..body.find("\n    }\n").unwrap_or(body.len())];
    if body.matches(open).count() != 1 || body.matches(close).count() != 1 {
        return vec!["expected exactly one marked block in post_approved_import".into()];
    }
    let start = body.find(open).unwrap();
    let end = body.find(close).unwrap();
    if end < start {
        return vec!["the marked block closes before it opens".into()];
    }
    let mut problems = Vec::new();
    let block = &body[start..end];
    for forbidden in ["before_dispatch()", "post_probe_xml("] {
        if block.contains(forbidden) {
            problems.push(format!("`{forbidden}` inside the pre-intent block"));
        }
    }
    let after = &body[end..];
    match (
        after.find("before_dispatch()"),
        after.find("post_probe_xml("),
    ) {
        (Some(intent), Some(post)) if intent < post => {}
        _ => problems.push("the intent, then the POST, must follow the block".into()),
    }
    problems
}

#[test]
fn nothing_is_sent_inside_the_pre_intent_block() {
    let runtime = read("src-tauri/src/tally/runtime.rs");
    assert_eq!(pre_intent_block_problems(&runtime), Vec::<String>::new());
    // The check itself: the intent moved into the block, or the block's error
    // left unmarked, is caught.
    let intent_inside = runtime.replacen(
        "let (admission_evidence, before_marks) = async {",
        "let (admission_evidence, before_marks) = async {\n                        before_dispatch().ok();",
        1,
    );
    let unmarked = runtime.replacen("PreIntentQueueRefusal { source }", "source", 1);
    for broken in [intent_inside, unmarked] {
        assert_ne!(broken, runtime);
        assert!(!pre_intent_block_problems(&broken).is_empty());
    }
}

/// The post child's whole entry point: its token, its dialog, and the real
/// stdin and stdout. A test calls `answer_with_token` with its own input,
/// output and dialog, so only this pin covers what the entry point passes.
const POST_ENTRY: &str = "pub fn run_confirmation() -> bool {
    answer_with_token(
        POST_TOKEN_PREFIX,
        show_review,
        std::io::stdin(),
        std::io::stdout(),
    )
}";

/// The review child's whole entry point, as [`POST_ENTRY`].
const REVIEW_ENTRY: &str = "pub fn run_review_confirmation() -> bool {
    answer_with_token(
        REVIEW_TOKEN_PREFIX,
        show_review_acknowledgement,
        std::io::stdin(),
        std::io::stdout(),
    )
}";

/// The one-ledger child's whole entry point, as [`POST_ENTRY`].
const MASTERS_ENTRY: &str = "pub fn run_masters_confirmation() -> bool {
    answer_with_token(
        MASTERS_TOKEN_PREFIX,
        show_create_ledger,
        std::io::stdin(),
        std::io::stdout(),
    )
}";

/// The settle child's whole entry point, as [`POST_ENTRY`] (#1709).
const SETTLE_ENTRY: &str = "pub fn run_settle_confirmation() -> bool {
    answer_with_token(
        SETTLE_TOKEN_PREFIX,
        show_settle_acknowledgement,
        std::io::stdin(),
        std::io::stdout(),
    )
}";

/// The release child's whole entry point, as [`POST_ENTRY`] (#1828).
const RELEASE_ENTRY: &str = "pub fn run_release_confirmation() -> bool {
    answer_with_token(
        RELEASE_TOKEN_PREFIX,
        show_release_acknowledgement,
        std::io::stdin(),
        std::io::stdout(),
    )
}";

/// Each dialog subprocess entry point answers with its own token, after its
/// own dialog, from the real stdin to the real stdout (#635, #687). The
/// parent approves a post only on the post token, so pairing that token with
/// the review dialog would let "I reviewed it" approve a post, and passing a
/// dialog other than the real one would approve with nobody asked. No
/// stand-in test could see either: a stand-in is not this code. Each entry
/// point's whole body is pinned, so a swapped body is caught too, and no
/// other `answer_with_token(` call may appear in the file. A text gate cannot
/// see a call that does not spell that out, such as one through a function
/// pointer, a parenthesised path `(answer_with_token)(…)`, or a `use … as`
/// rename; the lib.rs mode arms, pinned above, are still the only way in.
fn dialog_token_problems(source: &str) -> Vec<String> {
    let mut problems = Vec::new();
    for entry in [
        POST_ENTRY,
        REVIEW_ENTRY,
        RELEASE_ENTRY,
        SETTLE_ENTRY,
        MASTERS_ENTRY,
    ] {
        if source.matches(entry).count() != 1 {
            problems.push(format!("expected exactly one `{entry}`"));
        }
    }
    if source.matches("answer_with_token(").count() != 6 {
        problems.push("expected the five entry points and the definition only".into());
    }
    problems
}

#[test]
fn each_dialog_answers_with_its_own_token() {
    let source = read("src-tauri/src/tally/approved_import.rs");
    assert_eq!(dialog_token_problems(&source), Vec::<String>::new());
    let post_as_review = POST_ENTRY.replace("POST_TOKEN_PREFIX", "REVIEW_TOKEN_PREFIX");
    let review_as_post = REVIEW_ENTRY.replace("REVIEW_TOKEN_PREFIX", "POST_TOKEN_PREFIX");
    let post_body = &POST_ENTRY[POST_ENTRY.find('{').unwrap()..];
    let review_body = &REVIEW_ENTRY[REVIEW_ENTRY.find('{').unwrap()..];
    let swapped = source
        .replace(POST_ENTRY, "SWAP_POST")
        .replace(REVIEW_ENTRY, "SWAP_REVIEW")
        .replace(
            "SWAP_POST",
            &format!("pub fn run_confirmation() -> bool {review_body}"),
        )
        .replace(
            "SWAP_REVIEW",
            &format!("pub fn run_review_confirmation() -> bool {post_body}"),
        );
    for broken in [
        source.replace(POST_ENTRY, &POST_ENTRY.replace("show_review,", "show_review_acknowledgement,")),
        source.replace(POST_ENTRY, &post_as_review),
        source.replace(REVIEW_ENTRY, &review_as_post),
        source.replace(
            RELEASE_ENTRY,
            &RELEASE_ENTRY.replace("RELEASE_TOKEN_PREFIX", "REVIEW_TOKEN_PREFIX"),
        ),
        source.replace(
            RELEASE_ENTRY,
            &RELEASE_ENTRY.replace(
                "show_release_acknowledgement,",
                "show_review_acknowledgement,",
            ),
        ),
        source.replace(
            SETTLE_ENTRY,
            &SETTLE_ENTRY.replace("SETTLE_TOKEN_PREFIX", "REVIEW_TOKEN_PREFIX"),
        ),
        source.replace(
            SETTLE_ENTRY,
            &SETTLE_ENTRY.replace(
                "show_settle_acknowledgement,",
                "show_release_acknowledgement,",
            ),
        ),
        source.replace(POST_ENTRY, &POST_ENTRY.replace("show_review,", "|_, _| true,")),
        source.replace(POST_ENTRY, &POST_ENTRY.replace("std::io::stdin()", "&b\"\"[..]")),
        swapped,
        // The one-ledger entry answers with another dialog's token, or shows
        // another dialog, or no dialog; and another entry answers with the
        // one-ledger token or dialog.
        source.replace(MASTERS_ENTRY, &MASTERS_ENTRY.replace("MASTERS_TOKEN_PREFIX", "POST_TOKEN_PREFIX")),
        source.replace(MASTERS_ENTRY, &MASTERS_ENTRY.replace("MASTERS_TOKEN_PREFIX", "REVIEW_TOKEN_PREFIX")),
        source.replace(MASTERS_ENTRY, &MASTERS_ENTRY.replace("show_create_ledger,", "show_review,")),
        source.replace(MASTERS_ENTRY, &MASTERS_ENTRY.replace("show_create_ledger,", "|_, _| true,")),
        source.replace(MASTERS_ENTRY, &MASTERS_ENTRY.replace("std::io::stdin()", "&b\"\"[..]")),
        source.replace(POST_ENTRY, &POST_ENTRY.replace("POST_TOKEN_PREFIX", "MASTERS_TOKEN_PREFIX")),
        source.replace(REVIEW_ENTRY, &REVIEW_ENTRY.replace("show_review_acknowledgement,", "show_create_ledger,")),
        format!(
            "{source}\nfn extra() -> bool {{ answer_with_token(MASTERS_TOKEN_PREFIX, |_, _| true, std::io::stdin(), std::io::stdout()) }}\n"
        ),
        format!(
            "{source}\nfn extra() -> bool {{ answer_with_token(POST_TOKEN_PREFIX, |_, _| true, std::io::stdin(), std::io::stdout()) }}\n"
        ),
    ] {
        assert_ne!(broken, source);
        assert!(!dialog_token_problems(&broken).is_empty());
    }
}

/// Where a click becomes the answer (#687). No test can open a real window,
/// and the parent's tests replace the child with a stand-in (#702). So the
/// six native dialog functions (the post, review and release dialog on each
/// platform), `confirm` and `confirm_review`, and the two functions that decide
/// from the child's answer (`confirm_with`, `confirm_review_with`) are pinned
/// here verbatim, with the button labels and the functions that word each
/// title and the post button (#746). On Windows the post dialog's title and its
/// first line (#1780) are the only text that says what Yes does.
/// The file's `cfg` attributes are counted as well: a platform or test split
/// anywhere in it, such as a `#[cfg(windows)]` twin of a pinned function,
/// must change this gate. This pins text, not the platform's behaviour.
const REVIEW_ACK_DIALOG: &str = r#"#[cfg(not(windows))]
fn show_review_acknowledgement(count: VoucherCount, preview: &str) -> bool {
    exit_unless_the_alert_fits(preview);
    rfd::MessageDialog::new()
        .set_title(review_title(count))
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            REVIEW_BUTTON.into(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(REVIEW_BUTTON.into())
}"#;

const REVIEW_ACK_DIALOG_WINDOWS: &str = r#"#[cfg(windows)]
fn show_review_acknowledgement(count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = review_question(count)
        .encode_utf16()
        .chain(Some(0))
        .collect();
    // As for `show_review`: measured when it opens, and never returning if it
    // is not wholly on the screen (#1832).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: as for `show_review`: both buffers are NUL-terminated and
        // live for the synchronous dialog, and no parent HWND is borrowed.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // As for the post's box (`show_review`): topmost with
                // MB_SYSTEMMODAL, taking the foreground when Windows lets it,
                // with No as the default button (#1589, #1780).
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}"#;

/// The one-ledger dialog (the masters twin of [`POST_DIALOG`]): shown for one
/// item only, with its own title, its own positive button, and the same
/// Escape/default-No behaviour.
const MASTERS_DIALOG: &str = r#"#[cfg(not(windows))]
fn show_create_ledger(count: VoucherCount, preview: &str) -> bool {
    if count != ONE_ITEM {
        return false;
    }
    exit_unless_the_alert_fits(preview);
    rfd::MessageDialog::new()
        .set_title(MASTERS_TITLE)
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        // The Cancel label supplies the native Escape action. Creating
        // requires the explicitly matched positive button.
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            MASTERS_BUTTON.into(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(MASTERS_BUTTON.into())
}"#;

const MASTERS_DIALOG_WINDOWS: &str = r#"#[cfg(windows)]
fn show_create_ledger(count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    // As for the post dialog: No is the default, Escape and close are Cancel.
    if count != ONE_ITEM {
        return false;
    }
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = MASTERS_TITLE.encode_utf16().chain(Some(0)).collect();
    // As for `show_review`: measured when it opens, and never returning if it
    // is not wholly on the screen (#1832). Through `guarded` it is also
    // brought to the front once it appears, as the post's box is: its first
    // line tells the person to press Y or N, and in one lab run on Windows 10
    // MB_SETFOREGROUND alone did not give it the keyboard when it opened
    // seconds after the last input went to Tally (#1780).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: as for `show_review`: both buffers are NUL-terminated and
        // live for the synchronous dialog, and no parent HWND is borrowed.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // As for the post's box (`show_review`): topmost with
                // MB_SYSTEMMODAL, taking the foreground when Windows lets it,
                // with No as the default button (#1589, #1780).
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}"#;

const POST_DIALOG: &str = r#"#[cfg(not(windows))]
fn show_review(count: VoucherCount, preview: &str) -> bool {
    exit_unless_the_alert_fits(preview);
    let (title, button) = post_words(count);
    rfd::MessageDialog::new()
        .set_title(title)
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        // The Cancel label supplies the native Escape action. Posting requires
        // the explicitly matched positive button; Return may leave this dialog open.
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            button.clone(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(button)
}"#;

const POST_DIALOG_WINDOWS: &str = r#"#[cfg(windows)]
fn show_review(count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    // rfd without common-controls-v6 discards custom labels. Use the existing
    // Win32 dependency so No is the default and Escape/close remain Cancel.
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = post_question(count).encode_utf16().chain(Some(0)).collect();
    // The box cannot scroll: `guarded` measures it when it opens and, if it is
    // not wholly on the screen, never returns (#1832).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: Both buffers are NUL-terminated and live for the synchronous
        // dialog; no parent HWND is borrowed. No application state is exposed
        // to callbacks.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // Topmost: a window the connector opens while Tally fills the
                // screen is otherwise drawn behind it, since a background
                // process may not take the foreground (#1589). MB_TOPMOST alone
                // was seen not to hold when the box opened seconds after the
                // last input went to Tally. With MB_SYSTEMMODAL added the box
                // held it in one lab run on Windows 10, where Tally still took a
                // click and a key while the box was open (#1780). That run did
                // not test what else a system-modal box changes: the taskbar
                // showed no task buttons while it was open, cause not tested
                // (#2161). When Windows lets it,
                // MB_SETFOREGROUND also gives the box the foreground, so a key
                // typed for Tally can answer it; the parent puts a first line on
                // its text that says what Y and N do (#1780). No stays the
                // default button.
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}"#;

/// The post button a batch sees (#746): what `show_review` shows and
/// compares against, so it is pinned with the dialog.
const POST_WORDS: &str = r#"#[cfg(not(windows))]
fn post_words(count: VoucherCount) -> (String, String) {
    match count.batch() {
        None => (
            "ComplyEaze Bridge — approve one voucher".into(),
            POST_LABEL.into(),
        ),
        Some(count) => (
            format!("ComplyEaze Bridge — approve {count} vouchers"),
            format!("Post {count} vouchers"),
        ),
    }
}"#;

/// Each dialog's title for one voucher and for a batch (#746): the review's,
/// then the Windows review's and the Windows post's questions.
const REVIEW_TITLE: &str = r#"#[cfg(not(windows))]
fn review_title(count: VoucherCount) -> String {
    match count.batch() {
        None => "ComplyEaze Bridge — record that you reviewed one voucher".into(),
        Some(count) => format!("ComplyEaze Bridge — record that you reviewed {count} vouchers"),
    }
}"#;

const REVIEW_QUESTION: &str = r#"#[cfg(windows)]
fn review_question(count: VoucherCount) -> String {
    match count.batch() {
        None => "ComplyEaze Bridge — record that you reviewed this voucher?".into(),
        Some(count) => {
            format!("ComplyEaze Bridge — record that you reviewed these {count} vouchers?")
        }
    }
}"#;

/// The invoice-stop release window's own title and Windows question (#1828):
/// neither may name a review or a post.
const RELEASE_TITLE: &str = r#"#[cfg(not(windows))]
fn release_title() -> String {
    "ComplyEaze Bridge — release the invoice stop".into()
}"#;

const RELEASE_QUESTION: &str = r#"#[cfg(windows)]
fn release_question() -> String {
    "ComplyEaze Bridge — release the invoice stop for this company?".into()
}"#;

/// The invoice-stop release dialogs (#1828), pinned as the review's are.
const RELEASE_ACK_DIALOG: &str = r#"#[cfg(not(windows))]
fn show_release_acknowledgement(_count: VoucherCount, preview: &str) -> bool {
    exit_unless_the_alert_fits(preview);
    rfd::MessageDialog::new()
        .set_title(release_title())
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            RELEASE_BUTTON.into(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(RELEASE_BUTTON.into())
}"#;

const RELEASE_ACK_DIALOG_WINDOWS: &str = r#"#[cfg(windows)]
fn show_release_acknowledgement(_count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = release_question().encode_utf16().chain(Some(0)).collect();
    // As for `show_review`: measured when it opens, and never returning if it
    // is not wholly on the screen (#1832).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: as for `show_review`: both buffers are NUL-terminated and
        // live for the synchronous dialog, and no parent HWND is borrowed.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // As for the review's box: topmost with MB_SYSTEMMODAL, taking
                // the foreground when Windows lets it, with No as the default
                // button (#1589, #1780).
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}"#;

/// The settle window's own title and Windows question (#1709): neither may
/// name a review or a post.
const SETTLE_TITLE: &str = r#"#[cfg(not(windows))]
fn settle_title() -> String {
    "ComplyEaze Bridge — close this batch".into()
}"#;

const SETTLE_QUESTION: &str = r#"#[cfg(windows)]
fn settle_question() -> String {
    "ComplyEaze Bridge — close this batch for good?".into()
}"#;

/// The settle dialogs (#1709), pinned as the release's are.
const SETTLE_ACK_DIALOG: &str = r#"#[cfg(not(windows))]
fn show_settle_acknowledgement(_count: VoucherCount, preview: &str) -> bool {
    exit_unless_the_alert_fits(preview);
    rfd::MessageDialog::new()
        .set_title(settle_title())
        .set_description(preview)
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "Cancel".into(),
            SETTLE_BUTTON.into(),
        ))
        .show()
        == rfd::MessageDialogResult::Custom(SETTLE_BUTTON.into())
}"#;

const SETTLE_ACK_DIALOG_WINDOWS: &str = r#"#[cfg(windows)]
fn show_settle_acknowledgement(_count: VoucherCount, preview: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_DEFBUTTON2, MB_ICONWARNING, MB_SETFOREGROUND, MB_SYSTEMMODAL,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    let text: Vec<u16> = preview.encode_utf16().chain(Some(0)).collect();
    let title: Vec<u16> = settle_question().encode_utf16().chain(Some(0)).collect();
    // As for `show_review`: measured when it opens, and never returning if it
    // is not wholly on the screen (#1832).
    let answer = super::box_fit::guarded(|| {
        // SAFETY: as for `show_review`: both buffers are NUL-terminated and
        // live for the synchronous dialog, and no parent HWND is borrowed.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                // As for the review's box: topmost with MB_SYSTEMMODAL, taking
                // the foreground when Windows lets it, with No as the default
                // button (#1589, #1780).
                MB_YESNOCANCEL
                    | MB_DEFBUTTON2
                    | MB_ICONWARNING
                    | MB_SETFOREGROUND
                    | MB_TOPMOST
                    | MB_SYSTEMMODAL,
            )
        }
    });
    answer == IDYES
}"#;

/// Which subprocess mode and token each kind of acknowledgement uses (#1828):
/// a release shown in the review's mode would be titled a review again, and one
/// answered with the review's token could be answered by the review dialog.
const REVIEW_KIND_CHILD: &str = r#"    fn child(self) -> (&'static str, &'static str) {
        match self {
            Self::Review => ("--confirm-review", REVIEW_TOKEN_PREFIX),
            Self::StopRelease => ("--confirm-release", RELEASE_TOKEN_PREFIX),
            Self::Settle => ("--confirm-settle", SETTLE_TOKEN_PREFIX),
        }
    }"#;

const POST_QUESTION: &str = r#"#[cfg(windows)]
fn post_question(count: VoucherCount) -> String {
    match count.batch() {
        None => "ComplyEaze Bridge — post this voucher?".into(),
        Some(count) => format!("ComplyEaze Bridge — post {count} vouchers?"),
    }
}"#;

/// What the parent writes above a dialog's text (#1780): the answer text, in
/// the words of that platform's buttons, and the check that what the box will
/// show is within its caps. The child shows what it is sent, so these decide
/// what a person reads before the details.
const ANSWER_EFFECT: &str = r#"fn answer_effect(shown: Shown, count: VoucherCount) -> (String, &'static str) {
    match shown {
        Shown::Post => (
            format!("post {} into Tally", these_vouchers(count)),
            "Nothing is posted until you answer.",
        ),
        Shown::Review => (
            format!("record that you reviewed {}", these_vouchers(count)),
            "Nothing is recorded until you answer.",
        ),
        Shown::StopRelease => (
            "release the invoice stop for this company, so that ComplyEaze Bridge builds and \
             posts invoices for it again"
                .into(),
            "Nothing is released until you answer.",
        ),
        Shown::Settle => (
            format!(
                "close this batch in ComplyEaze Bridge for good, so that it never sends {} again",
                these_vouchers(count)
            ),
            "Nothing is closed until you answer.",
        ),
        Shown::Masters => (
            "create this ledger in Tally".into(),
            "Nothing is created until you answer.",
        ),
    }
}"#;

const ANSWER_TEXT: &str = r#"#[cfg(not(windows))]
fn answer_text(shown: Shown, count: VoucherCount) -> String {
    let (yes, nothing) = answer_effect(shown, count);
    let button = match shown {
        Shown::Post => post_words(count).1,
        Shown::Review => REVIEW_BUTTON.to_string(),
        Shown::StopRelease => RELEASE_BUTTON.to_string(),
        Shown::Settle => SETTLE_BUTTON.to_string(),
        Shown::Masters => MASTERS_BUTTON.to_string(),
    };
    wrapped(
        &format!("Click \"{button}\" to {yes}. Click Cancel to cancel. {nothing}"),
        usize::MAX,
    )
}"#;

const ANSWER_TEXT_WINDOWS: &str = r#"#[cfg(windows)]
fn answer_text(shown: Shown, count: VoucherCount) -> String {
    let (yes, nothing) = answer_effect(shown, count);
    wrapped(
        &format!("Press Y (or click Yes) to {yes}. Press N (or click No) to cancel. {nothing}"),
        ANSWER_WIDTH,
    )
}"#;

const DISPLAYED_TEXT: &str = r#"fn displayed_text(shown: Shown, count: VoucherCount, preview: &str) -> String {
    let answer = answer_text(shown, count);
    match (shown, preview.split_once("\n\n")) {
        (Shown::StopRelease, Some((statement, rest))) => {
            format!("{statement}\n\n{answer}\n\n{rest}")
        }
        _ => format!("{answer}\n\n{preview}"),
    }
}"#;

const MAC_ALERT_CHECK: &str = r#"#[cfg(not(windows))]
fn alert_top(screen: ScreenSpan) -> f64 {
    ALERT_TOP_FROM_SCREEN_TOP
        .max(screen.visible_top + ALERT_TOP_BELOW_VISIBLE_TOP)
        .max(screen.height * ALERT_TOP_SHARE_OF_HEIGHT)
}"#;

const MAC_ALERT_FITS: &str = r#"#[cfg(not(windows))]
fn alert_fits(text_height: f64, screen: ScreenSpan) -> bool {
    text_height.is_finite()
        && text_height >= 0.0
        && alert_top(screen) + ALERT_CHROME + text_height + ALERT_SLACK <= screen.visible_bottom
}"#;

const MAC_ALERT_EXIT: &str = r#"#[cfg(not(windows))]
fn exit_unless_the_alert_fits(text: &str) {
    match measured_alert(text) {
        Some((height, screen)) if alert_fits(height, screen) => {}
        Some(_) => std::process::exit(super::box_fit::DOES_NOT_FIT_EXIT),
        None => std::process::exit(0),
    }
}"#;

const FITS_THE_BOX: &str = r#"fn fits_the_box(shown: Shown, count: VoucherCount, displayed: &str) -> bool {
    if shown == Shown::StopRelease {
        return true;
    }
    let (lines, chars) = match count.batch() {
        None => (ONE_VOUCHER_LINES, ONE_VOUCHER_CHARS),
        Some(_) => (BATCH_LINES, BATCH_CHARS),
    };
    let (extra_lines, extra_chars) = answer_reserve();
    let answer_lines = answer_text(shown, count).lines().count();
    displayed.lines().count() <= lines + extra_lines
        && displayed.chars().count() <= chars + extra_chars
        && displayed
            .lines()
            .skip(answer_lines)
            .all(|line| line.chars().count() <= ANSWER_WIDTH)
}"#;

const CONFIRM_WITH: &str = r#"async fn confirm_with(
    executable: &std::path::Path,
    count: VoucherCount,
    preview: &str,
) -> Result<(), String> {
    if preview.len() > MAX_PREVIEW_BYTES {
        return Err("import_review_too_large".into());
    }
    match nonce_bound_dialog(
        executable,
        "--confirm-journal",
        POST_TOKEN_PREFIX,
        count,
        preview,
    )
    .await
    {
        // A box that did not fit the screen is a refusal whatever else the
        // child printed: the text is too large for this person's screen.
        Ok(answer) if answer.box_did_not_fit => {
            note_box_did_not_fit();
            Err("import_review_too_large".into())
        }
        Ok(answer) if answer.token_matched && answer.exited_cleanly => Ok(()),
        // A person's decline is no token and exit 1: `run_confirmation`
        // returns false. A clean exit without the token is never that; it is
        // an executable that does not answer with this token, such as one
        // ignoring the flag, or a build from before #635 whose dialog ran.
        Ok(answer) if answer.exited_cleanly => Err("import_approval_unavailable".into()),
        Ok(_) => Err("import_approval_declined".into()),
        Err(DialogFailure::Unavailable) => Err("import_approval_unavailable".into()),
        Err(DialogFailure::TimedOut) => Err("import_approval_timed_out".into()),
    }
}"#;

const CONFIRM_REVIEW_WITH: &str = r#"async fn confirm_review_with(
    executable: &std::path::Path,
    kind: ReviewKind,
    count: VoucherCount,
    preview: &str,
) -> Result<(), String> {
    if preview.len() > MAX_PREVIEW_BYTES {
        return Err("ack_review_too_large".into());
    }
    let (mode, prefix) = kind.child();
    match nonce_bound_dialog(executable, mode, prefix, count, preview).await {
        Ok(answer) if answer.box_did_not_fit => Err("ack_review_too_large".into()),
        Ok(answer) if answer.token_matched => Ok(()),
        // A person's decline is no token and exit 1: `run_review_confirmation`
        // and `run_release_confirmation` return false. A clean exit without
        // the token is never that; it is an executable that is not this
        // dialog, such as one ignoring the flag (#689).
        Ok(answer) if answer.exited_cleanly => Err("ack_review_unavailable".into()),
        Ok(_) => Err("ack_review_declined".into()),
        Err(DialogFailure::Unavailable) => Err("ack_review_unavailable".into()),
        Err(DialogFailure::TimedOut) => Err("ack_review_timed_out".into()),
    }
}"#;

/// `box_fit::guarded`, the Windows dialog's fit check, verbatim (#1832). Both
/// Windows dialogs answer through it, so an edit that lets `Unavailable` or a
/// box that did not fit return a result would approve with no box or a
/// clipped one.
const BOX_FIT_GUARDED: &str = r#"#[cfg(windows)]
pub(crate) fn guarded(show: impl FnOnce() -> i32) -> i32 {
    if !windows_hook::make_dpi_aware() {
        // Rectangles could not be read in one unit, so no box is shown. Said on
        // the error output, so a run by hand on an older Windows names the cause.
        use std::io::Write;
        let _ = writeln!(
            std::io::stderr(),
            "box-fit: not per-monitor DPI aware (Windows 10 version 1703 or later is needed): no box shown"
        );
        std::process::exit(0);
    }
    let Some(hook) = windows_hook::install() else {
        // No way to measure the box, so none is shown.
        std::process::exit(0);
    };
    let front = windows_hook::Front::start();
    let result = show();
    let seen = windows_hook::seen();
    drop(front);
    drop(hook);
    match outcome(result, seen) {
        Outcome::Answered(result) => result,
        Outcome::DoesNotFit => std::process::exit(DOES_NOT_FIT_EXIT),
        Outcome::Unavailable => std::process::exit(0),
    }
}"#;

/// The hook that measures the box and cancels one that does not fit, verbatim
/// (#1832). An edit that measured nothing, or always found a fit, would leave
/// the unit-tested outcome logic intact and measure nothing.
const BOX_FIT_HOOK: &str = r#"#[cfg(windows)]
mod windows_hook {
    use super::{Rect, Seen};
    use std::ffi::c_void;
    use std::io::Write;
    use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use windows_sys::core::BOOL;
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows_sys::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        BringWindowToTop, CallNextHookEx, EnumThreadWindows, FlashWindowEx, GetForegroundWindow,
        GetWindowRect, GetWindowThreadProcessId, IsWindowVisible, PostMessageW,
        SetForegroundWindow, SetWindowsHookExW, UnhookWindowsHookEx, FLASHWINFO, FLASHW_ALL,
        FLASHW_TIMERNOFG, HCBT_ACTIVATE, HHOOK, IDCANCEL, WH_CBT, WM_COMMAND,
    };

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleW(name: *const u16) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *const c_void;
    }

    /// `DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2`, a pseudo handle.
    const PER_MONITOR_AWARE_V2: isize = -4;

    /// Make this process per-monitor DPI aware (version 2), before any window
    /// exists, so that a window's rectangle and its monitor's work area are
    /// both physical pixels on every monitor. `false` if the process is not
    /// in that mode afterwards: the three functions are looked up when they
    /// are called, so that a Windows that lacks them (before Windows 10
    /// version 1703) still starts the program and simply shows no box.
    pub(super) fn make_dpi_aware() -> bool {
        let user32: Vec<u16> = "user32.dll\0".encode_utf16().collect();
        // SAFETY: `user32` is a NUL-terminated wide string that outlives the
        // call; the module is already loaded by the program's own imports.
        let module = unsafe { GetModuleHandleW(user32.as_ptr()) };
        if module.is_null() {
            return false;
        }
        let find = |name: &'static [u8]| -> *const c_void {
            debug_assert_eq!(name.last(), Some(&0));
            // SAFETY: `module` is a loaded module and `name` is a
            // NUL-terminated ASCII name.
            unsafe { GetProcAddress(module, name.as_ptr()) }
        };
        let set = find(b"SetProcessDpiAwarenessContext\0");
        let get = find(b"GetThreadDpiAwarenessContext\0");
        let same = find(b"AreDpiAwarenessContextsEqual\0");
        if set.is_null() || get.is_null() || same.is_null() {
            return false;
        }
        // SAFETY: each pointer is the address of the exported function of
        // that name, whose signature is the one given here (user32.dll).
        unsafe {
            let set = std::mem::transmute::<
                *const c_void,
                unsafe extern "system" fn(*mut c_void) -> i32,
            >(set);
            let get = std::mem::transmute::<
                *const c_void,
                unsafe extern "system" fn() -> *mut c_void,
            >(get);
            let same = std::mem::transmute::<
                *const c_void,
                unsafe extern "system" fn(*mut c_void, *mut c_void) -> i32,
            >(same);
            set(PER_MONITOR_AWARE_V2 as *mut c_void);
            same(get(), PER_MONITOR_AWARE_V2 as *mut c_void) != 0
        }
    }

    /// A helper thread that, once, when the box first appears, tries to bring
    /// it to the front, and flashes its taskbar button if Windows refuses. A
    /// background process is often refused the foreground, and a box nobody
    /// sees is a box nobody answers. It never takes the foreground back after
    /// that single attempt, so a person who moves to another window is not
    /// pulled away from it. It stops when dropped.
    pub(super) struct Front {
        stop: Arc<AtomicBool>,
        worker: Option<std::thread::JoinHandle<()>>,
    }

    impl Front {
        /// Start the helper for the box this thread is about to show.
        pub(super) fn start() -> Front {
            let stop = Arc::new(AtomicBool::new(false));
            // SAFETY: no arguments; reads this thread's id.
            let owner = unsafe { GetCurrentThreadId() };
            let flag = Arc::clone(&stop);
            let worker = std::thread::Builder::new()
                .spawn(move || raise_once_shown(owner, &flag))
                .ok();
            Front { stop, worker }
        }
    }

    impl Drop for Front {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
            if let Some(worker) = self.worker.take() {
                let _ = worker.join();
            }
        }
    }

    unsafe extern "system" fn first_visible(window: HWND, found: LPARAM) -> BOOL {
        // SAFETY: `found` is the address of the `HWND` slot `box_of` passed.
        let slot = unsafe { &mut *(found as *mut HWND) };
        // SAFETY: `window` is a handle Windows is enumerating.
        if unsafe { IsWindowVisible(window) } != 0 {
            *slot = window;
            0
        } else {
            1
        }
    }

    /// The first visible top-level window of `thread`, or null.
    fn box_of(thread: u32) -> HWND {
        let mut found: HWND = std::ptr::null_mut();
        // SAFETY: the callback only writes the slot whose address is passed,
        // which outlives the call.
        unsafe {
            EnumThreadWindows(
                thread,
                Some(first_visible),
                &mut found as *mut HWND as LPARAM,
            );
        }
        found
    }

    /// Try to make `window` the foreground window; if Windows refuses, flash
    /// its taskbar button until it is.
    fn raise(window: HWND) {
        // SAFETY: every handle is one just read from Windows; the input
        // attachment is undone before returning.
        unsafe {
            let foreground = GetForegroundWindow();
            if foreground == window {
                return;
            }
            let me = GetCurrentThreadId();
            let other = if foreground.is_null() {
                0
            } else {
                GetWindowThreadProcessId(foreground, std::ptr::null_mut())
            };
            let attached = other != 0 && other != me && AttachThreadInput(me, other, 1) != 0;
            SetForegroundWindow(window);
            BringWindowToTop(window);
            if attached {
                AttachThreadInput(me, other, 0);
            }
            if GetForegroundWindow() != window {
                let flash = FLASHWINFO {
                    cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
                    hwnd: window,
                    dwFlags: FLASHW_ALL | FLASHW_TIMERNOFG,
                    uCount: 0,
                    dwTimeout: 0,
                };
                FlashWindowEx(&flash);
            }
        }
    }

    fn raise_once_shown(owner: u32, stop: &AtomicBool) {
        // About ten seconds to appear: the box has long appeared by then.
        for _ in 0..100 {
            if stop.load(Ordering::SeqCst) {
                return;
            }
            let window = box_of(owner);
            if !window.is_null() {
                raise(window);
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// What the hook has seen, for this process's one box (0 nothing, 1 fits,
    /// 2 does not fit).
    static SEEN: AtomicU8 = AtomicU8::new(0);

    pub(super) fn seen() -> Seen {
        match SEEN.load(Ordering::SeqCst) {
            1 => Seen::Fits,
            2 => Seen::DoesNotFit,
            _ => Seen::Nothing,
        }
    }

    fn remember(seen: Seen) {
        SEEN.store(
            match seen {
                Seen::Nothing => 0,
                Seen::Fits => 1,
                Seen::DoesNotFit => 2,
            },
            Ordering::SeqCst,
        );
    }

    /// The hook, removed when it is dropped.
    pub(super) struct Hook(HHOOK);

    impl Drop for Hook {
        fn drop(&mut self) {
            // SAFETY: the handle came from `SetWindowsHookExW` and is removed
            // once, here.
            unsafe {
                UnhookWindowsHookEx(self.0);
            }
        }
    }

    /// Install a computer-based-training hook on this thread, the one that is
    /// about to show the box. `None` if Windows refuses.
    pub(super) fn install() -> Option<Hook> {
        remember(Seen::Nothing);
        // SAFETY: `on_activate` is an `extern "system"` function with the
        // hook procedure's signature; a null module handle with this thread's
        // id hooks this thread only.
        let handle = unsafe {
            SetWindowsHookExW(
                WH_CBT,
                Some(on_activate),
                std::ptr::null_mut(),
                GetCurrentThreadId(),
            )
        };
        (!handle.is_null()).then_some(Hook(handle))
    }

    /// The rectangle of `window` and the work area of the monitor nearest to
    /// it, in the same unit. `None` if either cannot be read.
    fn measure(window: HWND) -> Option<(Rect, Rect)> {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        // SAFETY: `window` is the window being activated and `rect` is a
        // live out-parameter of the right type.
        if unsafe { GetWindowRect(window, &mut rect) } == 0 {
            return None;
        }
        // SAFETY: as above.
        let monitor = unsafe { MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST) };
        if monitor.is_null() {
            return None;
        }
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            rcWork: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            dwFlags: 0,
        };
        // SAFETY: `monitor` is a handle just returned for this window and
        // `info` has its `cbSize` set, as the call requires.
        if unsafe { GetMonitorInfoW(monitor, &mut info) } == 0 {
            return None;
        }
        let rect = Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        };
        let work = Rect {
            left: info.rcWork.left,
            top: info.rcWork.top,
            right: info.rcWork.right,
            bottom: info.rcWork.bottom,
        };
        Some((rect, work))
    }

    /// Whether `window` lies wholly inside the work area of the monitor
    /// nearest to it. Any failure to read either is "does not fit". One line
    /// of the numbers goes to the process's own error output (the parent
    /// discards it; a person running the dialog by hand can read it).
    fn fits(window: HWND) -> bool {
        let measured = measure(window);
        let fitted = measured.is_some_and(|(rect, work)| rect.is_wholly_inside(work));
        let _ = match measured {
            Some((r, w)) => writeln!(
                std::io::stderr(),
                "box-fit: box=({},{},{},{}) work=({},{},{},{}) fits={fitted}",
                r.left,
                r.top,
                r.right,
                r.bottom,
                w.left,
                w.top,
                w.right,
                w.bottom
            ),
            None => writeln!(std::io::stderr(), "box-fit: not readable fits=false"),
        };
        fitted
    }

    /// The hook procedure: when the box is about to be activated, measure it;
    /// if it does not fit, post it a Cancel and remember that it did not.
    unsafe extern "system" fn on_activate(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code == HCBT_ACTIVATE as i32 {
            let window = wparam as HWND;
            let fitted = fits(window);
            remember(seen().after(fitted));
            if !fitted {
                // SAFETY: `window` is the box being activated; Cancel is a
                // button every box this module guards has.
                unsafe {
                    PostMessageW(window, WM_COMMAND, IDCANCEL as WPARAM, 0);
                }
            }
        }
        // SAFETY: passing the hook's own arguments on, with no hook handle,
        // is what the documentation prescribes.
        unsafe { CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam) }
    }
}"#;

/// Every `cfg` in box_fit.rs: the inner attribute, the two Windows items and
/// the test module. The last entry counts the bare text `cfg`.
const BOX_FIT_CFG_CENSUS: [(&str, usize); 4] = [
    (
        "#![cfg_attr(all(not(windows), not(test)), allow(dead_code))]",
        1,
    ),
    ("#[cfg(windows)]", 2),
    ("#[cfg(test)]", 1),
    ("cfg", 4),
];

fn box_fit_problems(source: &str) -> Vec<String> {
    let mut problems = Vec::new();
    for (pin, expected) in [(BOX_FIT_GUARDED, 1), (BOX_FIT_HOOK, 1)]
        .iter()
        .chain(BOX_FIT_CFG_CENSUS.iter())
    {
        if source.matches(pin).count() != *expected {
            problems.push(format!("expected {expected} of `{pin}`"));
        }
    }
    problems
}

fn replace_last(source: &str, from: &str, to: &str) -> String {
    let at = source.rfind(from).expect("the text to replace is there");
    format!("{}{to}{}", &source[..at], &source[at + from.len()..])
}

#[test]
fn the_fit_check_that_guards_both_windows_dialogs_is_pinned() {
    let source = read("src-tauri/src/tally/box_fit.rs");
    assert_eq!(box_fit_problems(&source), Vec::<String>::new());
    for broken in [
        // The process is not made DPI-aware, or the check passes anyway, so
        // the rectangle and the work area may be read in different units.
        source.replacen("if !windows_hook::make_dpi_aware() {", "if false {", 1),
        source.replacen("same(get(), PER_MONITOR_AWARE_V2 as *mut c_void) != 0", "true", 1),
        source.replacen("set(PER_MONITOR_AWARE_V2 as *mut c_void);", "", 1),
        source.replacen("if set.is_null() || get.is_null() || same.is_null() {\n            return false;", "if set.is_null() || get.is_null() || same.is_null() {\n            return true;", 1),
        // The helper takes the foreground back again and again, so a person
        // who moves to another window is pulled away from it.
        source.replacen("                raise(window);\n                return;", "                raise(window);", 1),
        // The helper that brings the box to the front is not started.
        source.replacen("let front = windows_hook::Front::start();", "", 1),
        // A box never measured, or a box that did not fit, returns a result
        // instead of ending the process.
        source.replacen("Outcome::Unavailable => std::process::exit(0),", "Outcome::Unavailable => result,", 1),
        source.replacen("Outcome::DoesNotFit => std::process::exit(DOES_NOT_FIT_EXIT),", "Outcome::DoesNotFit => result,", 1),
        source.replacen("Outcome::Answered(result) => result,", "Outcome::Answered(_) => 6,", 1),
        // No hook: the box is shown unmeasured, or the answer is made up.
        source.replacen("        std::process::exit(0);\n    };\n    let front", "        return show();\n    };\n    let front", 1),
        source.replacen("let result = show();", "let result = { show(); 6 };", 1),
        source.replacen("let seen = windows_hook::seen();", "let seen = Seen::Fits;", 1),
        // The measurement always finds a fit, or never cancels.
        source.replacen("rect.is_wholly_inside(work));", "true);", 1),
        source.replacen("if !fitted {", "if false {", 1),
        source.replacen("remember(seen().after(fitted));", "remember(Seen::Fits);", 1),
        source.replacen("let fitted = measured.is_some_and(", "let fitted = measured.is_none() || measured.is_some_and(", 1),
        source.replacen("== 0 {\n            return None;", "== 0 {\n            return Some((Rect { left: 0, top: 0, right: 1, bottom: 1 }, Rect { left: 0, top: 0, right: 2, bottom: 2 }));", 1),
        // The guard made non-Windows beside a Windows twin that approves.
        source.replacen(
            "#[cfg(windows)]\npub(crate) fn guarded",
            "#[cfg(not(windows))]\npub(crate) fn guarded",
            1,
        ),
        format!("{source}\n#[cfg(windows)]\npub(crate) fn guarded_twin(show: impl FnOnce() -> i32) -> i32 {{\n    show()\n}}\n"),
        format!("{source}\n#[cfg_attr(windows, allow(unused))]\nfn extra() {{}}\n"),
    ] {
        assert_ne!(broken, source);
        assert!(!box_fit_problems(&broken).is_empty());
    }
}

const CONFIRM_MASTERS_WITH: &str = r#"async fn confirm_masters_with(executable: &std::path::Path, preview: &str) -> Result<(), String> {
    if preview.len() > MAX_PREVIEW_BYTES {
        return Err("master_review_too_large".into());
    }
    match nonce_bound_dialog(
        executable,
        "--confirm-masters",
        MASTERS_TOKEN_PREFIX,
        ONE_ITEM,
        preview,
    )
    .await
    {
        // A box that did not fit the screen is a refusal whatever else the
        // child printed, as for the post's box (#1832).
        Ok(answer) if answer.box_did_not_fit => Err("master_review_too_large".into()),
        Ok(answer) if answer.token_matched && answer.exited_cleanly => Ok(()),
        // A person's decline is no token and exit 1. A clean exit without the
        // token is an executable that is not this dialog.
        Ok(answer) if answer.exited_cleanly => Err("master_approval_unavailable".into()),
        Ok(_) => Err("master_approval_declined".into()),
        Err(DialogFailure::Unavailable) => Err("master_approval_unavailable".into()),
        Err(DialogFailure::TimedOut) => Err("master_approval_timed_out".into()),
    }
}"#;

const DIALOG_ANSWER_PINS: [(&str, usize); 47] = [
    ("const POST_LABEL: &str = \"Post voucher\";", 1),
    (
        "pub(crate) const REVIEW_BUTTON: &str = \"I reviewed it\";",
        1,
    ),
    (POST_DIALOG, 1),
    (POST_WORDS, 1),
    (POST_QUESTION, 1),
    (REVIEW_TITLE, 1),
    (REVIEW_QUESTION, 1),
    (ANSWER_EFFECT, 1),
    (ANSWER_TEXT, 1),
    (ANSWER_TEXT_WINDOWS, 1),
    (DISPLAYED_TEXT, 1),
    // The Mac alert's measured check, with which each Mac dialog ends its child
    // unshown when its text would run past the screen (#1780, option (c2)): where
    // its top is taken to sit, the comparison, and the exit. Its figures are the
    // lab rows', so they are not pinned here; the five calls are in the dialogs'
    // own pins.
    (MAC_ALERT_CHECK, 1),
    (MAC_ALERT_FITS, 1),
    (MAC_ALERT_EXIT, 1),
    (FITS_THE_BOX, 1),
    (POST_DIALOG_WINDOWS, 1),
    (REVIEW_ACK_DIALOG, 1),
    (REVIEW_ACK_DIALOG_WINDOWS, 1),
    (
        "async fn confirm(count: VoucherCount, preview: &str) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|_| \"import_approval_unavailable\")?;
    let shown = displayed_text(Shown::Post, count, preview);
    if !fits_the_box(Shown::Post, count, &shown) {
        return Err(\"import_review_too_large\".into());
    }
    confirm_with(&executable, count, &shown).await
}",
        1,
    ),
    (
        "async fn confirm_review(
    kind: ReviewKind,
    count: VoucherCount,
    preview: &str,
) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|_| \"ack_review_unavailable\")?;
    let shown = displayed_text(kind.shown(), count, preview);
    if !fits_the_box(kind.shown(), count, &shown) {
        return Err(\"ack_review_too_large\".into());
    }
    confirm_review_with(&executable, kind, count, &shown).await
}",
        1,
    ),
    (CONFIRM_WITH, 1),
    (CONFIRM_REVIEW_WITH, 1),
    // The child's "my box did not fit the screen" exit status is read as
    // that and nothing else (#1832).
    (
        "box_did_not_fit: status.code() == Some(super::box_fit::DOES_NOT_FIT_EXIT),",
        1,
    ),
    ("pub(crate) const REVIEW_BUTTON: &str = \"Yes\";", 1),
    // The one-ledger dialog: its words, its count, both native dialogs, and
    // the two functions that ask it and decide from the child's answer.
    (
        "const MASTERS_TITLE: &str = \"ComplyEaze Bridge — create one ledger?\";",
        1,
    ),
    (
        "#[cfg(not(windows))]\nconst MASTERS_BUTTON: &str = \"Create ledger\";",
        1,
    ),
    (
        "const ONE_ITEM: VoucherCount = VoucherCount(NonZeroUsize::MIN);",
        1,
    ),
    (MASTERS_DIALOG, 1),
    (MASTERS_DIALOG_WINDOWS, 1),
    (
        "async fn confirm_masters_dialog(preview: &str) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|_| \"master_approval_unavailable\")?;
    let shown = displayed_text(Shown::Masters, ONE_ITEM, preview);
    confirm_masters_with(&executable, &shown).await
}",
        1,
    ),
    (CONFIRM_MASTERS_WITH, 1),
    ("const RELEASE_BUTTON: &str = \"Release the stop\";", 1),
    ("const SETTLE_BUTTON: &str = \"Close the batch\";", 1),
    // Each token's value, so no two modes can share one (#1828).
    (
        "const REVIEW_TOKEN_PREFIX: &str = \"bridge-review-acknowledged:\";",
        1,
    ),
    (
        "const POST_TOKEN_PREFIX: &str = \"bridge-post-approved:\";",
        1,
    ),
    (
        "const RELEASE_TOKEN_PREFIX: &str = \"bridge-stop-release-acknowledged:\";",
        1,
    ),
    (
        "const MASTERS_TOKEN_PREFIX: &str = \"bridge-create-ledger:\";",
        1,
    ),
    (
        "const SETTLE_TOKEN_PREFIX: &str = \"bridge-batch-settle-acknowledged:\";",
        1,
    ),
    (RELEASE_TITLE, 1),
    (RELEASE_QUESTION, 1),
    (RELEASE_ACK_DIALOG, 1),
    (RELEASE_ACK_DIALOG_WINDOWS, 1),
    (SETTLE_TITLE, 1),
    (SETTLE_QUESTION, 1),
    (SETTLE_ACK_DIALOG, 1),
    (SETTLE_ACK_DIALOG_WINDOWS, 1),
    (REVIEW_KIND_CHILD, 1),
];

/// Every `cfg` in approved_import.rs, by form. The last entry counts the
/// bare text `cfg`, so a `cfg_attr`, a `cfg!`, or a combined predicate such as
/// `cfg(any(…))` is caught too.
const CFG_CENSUS: [(&str, usize); 6] = [
    // #725 added one of each seam arm for the dialog task's scope, and the
    // non-test arm's identity function. The one-ledger dialog added one of
    // each arm of its own.
    // #1780 added one more, the test-only accessor for what a review shows.
    ("#[cfg(test)]", 8),
    ("#[cfg(not(test))]", 5),
    // #702 ran the parent's seven unix-only test items on every platform.
    ("#[cfg(unix)]", 0),
    // The one-ledger dialog added one Windows dialog, one non-Windows dialog
    // and the non-Windows button; #1780's follow-up, the Mac one-line answer
    // text test. #1828: the release window's question and box on Windows, and
    // its title, dialog and button on a Mac. #1709: the same for the settle
    // window.
    ("#[cfg(windows)]", 12),
    // #1780's Mac check (c2): the seven measured figures, the screen span,
    // where the top sits, the comparison, the child's exit and their two tests;
    // and four `cfg(target_os = "macos")` items (the text's height, the screen,
    // the two together, and the height's test) with one other-platform arm.
    ("#[cfg(not(windows))]", 30),
    ("cfg", 61),
];

fn dialog_answer_problems(source: &str) -> Vec<String> {
    let mut problems = Vec::new();
    for (pin, expected) in DIALOG_ANSWER_PINS.iter().chain(CFG_CENSUS.iter()) {
        if source.matches(pin).count() != *expected {
            problems.push(format!("expected {expected} of `{pin}`"));
        }
    }
    problems
}

/// A dialog function rewritten to compute its comparison and then return
/// `true` whatever the person chose.
fn discard_answer(dialog: &str) -> String {
    let discarded = if dialog.contains("rfd::MessageDialog::new()") {
        dialog.replacen(
            "    rfd::MessageDialog::new()",
            "    let _ = rfd::MessageDialog::new()",
            1,
        )
    } else {
        dialog.replacen(
            "    answer == IDYES\n}",
            "    let _ = answer == IDYES;\n    true\n}",
            1,
        )
    };
    let discarded = if discarded.ends_with(")\n}") || discarded.ends_with("))\n}") {
        format!("{};\n    true\n}}", discarded.trim_end_matches("\n}"))
    } else {
        discarded.replacen(
            ") == IDYES\n    }\n}",
            ") == IDYES;\n        true\n    }\n}",
            1,
        )
    };
    assert_ne!(discarded, dialog);
    discarded
}

#[test]
fn each_dialog_answers_only_on_its_positive_button() {
    let source = read("src-tauri/src/tally/approved_import.rs");
    assert_eq!(dialog_answer_problems(&source), Vec::<String>::new());
    for broken in [
        source.replacen(
            "== rfd::MessageDialogResult::Custom(button)",
            "!= rfd::MessageDialogResult::Custom(button)",
            1,
        ),
        source.replacen(
            "== rfd::MessageDialogResult::Custom(REVIEW_BUTTON.into())",
            "!= rfd::MessageDialogResult::Custom(REVIEW_BUTTON.into())",
            1,
        ),
        source.replacen("answer == IDYES", "answer != IDNO", 1),
        source.replacen(
            REVIEW_ACK_DIALOG_WINDOWS,
            &REVIEW_ACK_DIALOG_WINDOWS.replace("MB_DEFBUTTON2", "MB_DEFBUTTON1"),
            1,
        ),
        // The parent shows the preview without the answer text, or shows the
        // release with the review's words (#1780).
        source.replacen(
            "displayed_text(Shown::Post, count, preview)",
            "preview.to_string()",
            1,
        ),
        source.replacen(
            "let shown = displayed_text(kind.shown(), count, preview);",
            "let shown = displayed_text(Shown::Review, count, preview);",
            1,
        ),
        source.replacen("let (extra_lines, extra_chars) = answer_reserve();", "let (extra_lines, extra_chars) = (0, 0);", 1),
        // A Mac dialog stops measuring its alert, or the measure drops the
        // lowest top, the slack or the visible bottom, shows an alert it could
        // not measure, or ends the child with a status the parent does not read
        // as a box that did not fit (#1780, option (c2)).
        source.replacen("    exit_unless_the_alert_fits(preview);\n", "", 1),
        replace_last(&source, "    exit_unless_the_alert_fits(preview);\n", ""),
        source.replacen(
            "fn show_review(count: VoucherCount, preview: &str) -> bool {\n    exit_unless_the_alert_fits(preview);\n",
            "fn show_review(count: VoucherCount, preview: &str) -> bool {\n",
            1,
        ),
        source.replacen(
            "fn show_release_acknowledgement(_count: VoucherCount, preview: &str) -> bool {\n    exit_unless_the_alert_fits(preview);\n",
            "fn show_release_acknowledgement(_count: VoucherCount, preview: &str) -> bool {\n",
            1,
        ),
        source.replacen(
            "fn show_settle_acknowledgement(_count: VoucherCount, preview: &str) -> bool {\n    exit_unless_the_alert_fits(preview);\n",
            "fn show_settle_acknowledgement(_count: VoucherCount, preview: &str) -> bool {\n",
            1,
        ),
        source.replacen("        .max(screen.height * ALERT_TOP_SHARE_OF_HEIGHT)\n", "", 1),
        source.replacen(" + ALERT_SLACK <= screen.visible_bottom", " <= screen.visible_bottom", 1),
        source.replacen("<= screen.visible_bottom", "<= screen.height", 1),
        source.replacen("        None => std::process::exit(0),", "        None => {}", 1),
        source.replacen(
            "        Some(_) => std::process::exit(super::box_fit::DOES_NOT_FIT_EXIT),",
            "        Some(_) => std::process::exit(0),",
            1,
        ),
        // The parent stops reading a box that did not fit as a refusal, or
        // the dialog stops measuring its box (#1832).
        source.replacen(
            "        Ok(answer) if answer.box_did_not_fit => {\n            note_box_did_not_fit();\n            Err(\"import_review_too_large\".into())\n        }\n",
            "",
            1,
        ),
        // The box that did not fit is refused but no longer noted for the
        // batch's next ask (#1832).
        source.replacen("            note_box_did_not_fit();\n", "", 1),
        source.replacen(
            "        Ok(answer) if answer.box_did_not_fit => Err(\"ack_review_too_large\".into()),\n",
            "",
            1,
        ),
        source.replacen(
            "        Ok(answer) if answer.box_did_not_fit => Err(\"master_review_too_large\".into()),\n",
            "",
            1,
        ),
        source.replacen("super::box_fit::guarded(|| {", "(|| {", 1),
        replace_last(&source, "super::box_fit::guarded(|| {", "(|| {"),
        source.replacen(
            "box_did_not_fit: status.code() == Some(super::box_fit::DOES_NOT_FIT_EXIT),",
            "box_did_not_fit: false,",
            1,
        ),
        source.replacen("\"Post voucher\"", "\"Cancel\"", 1),
        source.replacen("\"I reviewed it\"", "\"Cancel\"", 1),
        source.replacen(
            "confirm_with(&executable, count, &shown).await\n}",
            "let _ = (executable, count, shown);\n    Ok(())\n}",
            1,
        ),
        source.replacen(
            "confirm_review_with(&executable, kind, count, &shown).await\n}",
            "let _ = (executable, count, shown);\n    Ok(())\n}",
            1,
        ),
        source.replacen(
            "Ok(answer) if answer.token_matched && answer.exited_cleanly => Ok(()),",
            "Ok(answer) if answer.exited_cleanly => Ok(()),",
            1,
        ),
        source.replacen(
            "Ok(answer) if answer.token_matched => Ok(()),",
            "Ok(_) => Ok(()),",
            1,
        ),
        // A Windows-only twin that approves, beside the real one made
        // non-Windows: every pinned body is still present.
        source.replacen(
            CONFIRM_WITH,
            &format!(
                "#[cfg(not(windows))]\n{CONFIRM_WITH}\n#[cfg(windows)]\nasync fn confirm_with(_: &std::path::Path, _: VoucherCount, _: &str) -> Result<(), String> {{\n    Ok(())\n}}"
            ),
            1,
        ),
        format!(
            "{source}\n#[cfg(windows)]\nfn dialog_token(prefix: &str, _: &str) -> String {{\n    prefix.to_string()\n}}\n"
        ),
        format!("{source}\n#[cfg_attr(windows, allow(unused))]\nfn extra() {{}}\n"),
        source.replacen(
            CONFIRM_WITH,
            &CONFIRM_WITH.replacen(
                "        Ok(answer) if answer.token_matched && answer.exited_cleanly => Ok(()),",
                "        Ok(answer) if answer.exited_cleanly => Ok(()),\n        Ok(answer) if answer.token_matched && answer.exited_cleanly => Ok(()),",
                1,
            ),
            1,
        ),
        // The review dialog's clean-exit arm (#689) turned into an approval.
        source.replacen(
            "Ok(answer) if answer.exited_cleanly => Err(\"ack_review_unavailable\".into()),",
            "Ok(answer) if answer.exited_cleanly => Ok(()),",
            1,
        ),
        // A Windows-only review twin that acknowledges, beside the real one
        // made non-Windows.
        source.replacen(
            CONFIRM_REVIEW_WITH,
            &format!(
                "#[cfg(not(windows))]\n{CONFIRM_REVIEW_WITH}\n#[cfg(windows)]\nasync fn confirm_review_with(_: &std::path::Path, _: &str) -> Result<(), String> {{\n    Ok(())\n}}"
            ),
            1,
        ),
        source.replacen("\"Yes\";", "\"No\";", 1),
        // The one-ledger dialog approves on anything but its own button,
        // reads as the decline, or defaults to Yes (Windows MB_DEFBUTTON1).
        source.replacen(
            "== rfd::MessageDialogResult::Custom(MASTERS_BUTTON.into())",
            "!= rfd::MessageDialogResult::Custom(MASTERS_BUTTON.into())",
            1,
        ),
        source.replacen("\"Create ledger\"", "\"Cancel\"", 1),
        source.replacen(
            MASTERS_DIALOG_WINDOWS,
            &MASTERS_DIALOG_WINDOWS.replace("MB_DEFBUTTON2", "MB_DEFBUTTON1"),
            1,
        ),
        source.replacen(
            MASTERS_DIALOG_WINDOWS,
            &MASTERS_DIALOG_WINDOWS.replacen("answer == IDYES", "answer != IDNO", 1),
            1,
        ),
        // The one-ledger dialog's title reads as the post's, or its count
        // guard is gone, so a batch's count would show it.
        source.replacen(
            "const MASTERS_TITLE: &str = \"ComplyEaze Bridge — create one ledger?\";",
            "const MASTERS_TITLE: &str = \"ComplyEaze Bridge — approve one voucher\";",
            1,
        ),
        source.replacen(
            MASTERS_DIALOG,
            &MASTERS_DIALOG.replacen("count != ONE_ITEM", "false", 1),
            1,
        ),
        source.replacen(
            MASTERS_DIALOG_WINDOWS,
            &MASTERS_DIALOG_WINDOWS.replacen("count != ONE_ITEM", "false", 1),
            1,
        ),
        source.replacen(
            "confirm_masters_with(&executable, &shown).await\n}",
            "let _ = (executable, shown);\n    Ok(())\n}",
            1,
        ),
        // The one-ledger decision approves a clean exit with no token, or on
        // the token alone, or is asked in another dialog's mode or with
        // another's token or count.
        source.replacen(
            CONFIRM_MASTERS_WITH,
            &CONFIRM_MASTERS_WITH.replacen(
                "Ok(answer) if answer.token_matched && answer.exited_cleanly => Ok(()),",
                "Ok(answer) if answer.exited_cleanly => Ok(()),",
                1,
            ),
            1,
        ),
        source.replacen(
            CONFIRM_MASTERS_WITH,
            &CONFIRM_MASTERS_WITH.replacen(
                "Ok(answer) if answer.exited_cleanly => Err(\"master_approval_unavailable\".into()),",
                "Ok(answer) if answer.exited_cleanly => Ok(()),",
                1,
            ),
            1,
        ),
        source.replacen(
            CONFIRM_MASTERS_WITH,
            &CONFIRM_MASTERS_WITH.replacen("\"--confirm-masters\"", "\"--confirm-journal\"", 1),
            1,
        ),
        source.replacen(
            CONFIRM_MASTERS_WITH,
            &CONFIRM_MASTERS_WITH.replacen("MASTERS_TOKEN_PREFIX", "POST_TOKEN_PREFIX", 1),
            1,
        ),
        source.replacen(
            CONFIRM_MASTERS_WITH,
            &CONFIRM_MASTERS_WITH.replacen("ONE_ITEM", "VoucherCount(NonZeroUsize::MAX)", 1),
            1,
        ),
        source.replacen(
            CONFIRM_MASTERS_WITH,
            &CONFIRM_MASTERS_WITH.replacen("MAX_PREVIEW_BYTES", "usize::MAX", 1),
            1,
        ),
        // A Windows-only twin that approves, beside the real one made
        // non-Windows.
        source.replacen(
            CONFIRM_MASTERS_WITH,
            &format!(
                "#[cfg(not(windows))]\n{CONFIRM_MASTERS_WITH}\n#[cfg(windows)]\nasync fn confirm_masters_with(_: &std::path::Path, _: &str) -> Result<(), String> {{\n    Ok(())\n}}"
            ),
            1,
        ),
        // The post and review dialogs asked in the one-ledger mode.
        source.replacen(
            CONFIRM_WITH,
            &CONFIRM_WITH.replacen("\"--confirm-journal\"", "\"--confirm-masters\"", 1),
            1,
        ),
        // The review's mode now comes from the kind's table, not from the
        // function, so the review is sent to the one-ledger mode there.
        source.replacen(
            "Self::Review => (\"--confirm-review\", REVIEW_TOKEN_PREFIX),",
            "Self::Review => (\"--confirm-masters\", REVIEW_TOKEN_PREFIX),",
            1,
        ),
        // The one-ledger dialogs discard their answer: they still compute the
        // comparison, then return true.
        source.replacen(MASTERS_DIALOG, &discard_answer(MASTERS_DIALOG), 1),
        source.replacen(
            MASTERS_DIALOG_WINDOWS,
            &discard_answer(MASTERS_DIALOG_WINDOWS),
            1,
        ),
        // A batch's post button reads as the decline, or the dialog shows one
        // button and compares against another (#746).
        source.replacen("format!(\"Post {count} vouchers\")", "\"Cancel\".into()", 1),
        source.replacen("            button.clone(),\n", "            \"Post\".into(),\n", 1),
        // The Windows post dialog asks the review's question, so Yes reads as
        // recording a review while it posts (#746).
        source.replacen(
            "None => \"ComplyEaze Bridge — post this voucher?\".into(),",
            "None => \"ComplyEaze Bridge — record that you reviewed this voucher?\".into(),",
            1,
        ),
        source.replacen(
            "format!(\"ComplyEaze Bridge — post {count} vouchers?\")",
            "format!(\"ComplyEaze Bridge — record that you reviewed these {count} vouchers?\")",
            1,
        ),
        // A review dialog's title reads as approving a post (#746).
        source.replacen(
            "format!(\"ComplyEaze Bridge — record that you reviewed {count} vouchers\")",
            "format!(\"ComplyEaze Bridge — approve {count} vouchers\")",
            1,
        ),
        // The Windows review dialog asks the post's question (#746).
        source.replacen(
            "format!(\"ComplyEaze Bridge — record that you reviewed these {count} vouchers?\")",
            "format!(\"ComplyEaze Bridge — post {count} vouchers?\")",
            1,
        ),
        // rfd post dialog discards its answer: it still computes the comparison, then returns true.
        source.replacen(POST_DIALOG, &discard_answer(POST_DIALOG), 1),
        // Windows post dialog discards its answer: it still computes the comparison, then returns true.
        source.replacen(POST_DIALOG_WINDOWS, &discard_answer(POST_DIALOG_WINDOWS), 1),
        // rfd review dialog discards its answer: it still computes the comparison, then returns true.
        source.replacen(REVIEW_ACK_DIALOG, &discard_answer(REVIEW_ACK_DIALOG), 1),
        // Windows review dialog discards its answer: it still computes the comparison, then returns true.
        source.replacen(
            REVIEW_ACK_DIALOG_WINDOWS,
            &discard_answer(REVIEW_ACK_DIALOG_WINDOWS),
            1,
        ),
        // The release dialogs discard their answer (#1828).
        source.replacen(RELEASE_ACK_DIALOG, &discard_answer(RELEASE_ACK_DIALOG), 1),
        source.replacen(
            RELEASE_ACK_DIALOG_WINDOWS,
            &discard_answer(RELEASE_ACK_DIALOG_WINDOWS),
            1,
        ),
        // The settle dialogs discard their answer, read as the decline, are
        // titled a review, or are shown in the review's mode (#1709).
        source.replacen(SETTLE_ACK_DIALOG, &discard_answer(SETTLE_ACK_DIALOG), 1),
        source.replacen(
            SETTLE_ACK_DIALOG_WINDOWS,
            &discard_answer(SETTLE_ACK_DIALOG_WINDOWS),
            1,
        ),
        source.replacen("\"Close the batch\"", "\"Cancel\"", 1),
        source.replacen(
            "\"ComplyEaze Bridge — close this batch\".into()",
            "\"ComplyEaze Bridge — record that you reviewed one voucher\".into()",
            1,
        ),
        source.replacen(
            "Self::Settle => (\"--confirm-settle\", SETTLE_TOKEN_PREFIX),",
            "Self::Settle => (\"--confirm-review\", REVIEW_TOKEN_PREFIX),",
            1,
        ),
        source.replacen(
            SETTLE_ACK_DIALOG_WINDOWS,
            &SETTLE_ACK_DIALOG_WINDOWS.replace("MB_DEFBUTTON2", "MB_DEFBUTTON1"),
            1,
        ),
        // A release reads as the decline, is titled a review, or is shown in the
        // review's mode or answered with its token (#1828).
        source.replacen("\"Release the stop\"", "\"Cancel\"", 1),
        source.replacen(
            "\"ComplyEaze Bridge — release the invoice stop\".into()",
            "\"ComplyEaze Bridge — record that you reviewed one voucher\".into()",
            1,
        ),
        source.replacen(
            "Self::StopRelease => (\"--confirm-release\", RELEASE_TOKEN_PREFIX),",
            "Self::StopRelease => (\"--confirm-review\", REVIEW_TOKEN_PREFIX),",
            1,
        ),
        // Two modes share a token, the Windows release box asks the review's
        // question, or loses its default No (#1828).
        source.replacen(
            "\"bridge-review-acknowledged:\"",
            "\"bridge-stop-release-acknowledged:\"",
            1,
        ),
        source.replacen(
            "\"ComplyEaze Bridge — release the invoice stop for this company?\".into()",
            "\"ComplyEaze Bridge — record that you reviewed this voucher?\".into()",
            1,
        ),
        source.replacen(
            RELEASE_ACK_DIALOG_WINDOWS,
            &RELEASE_ACK_DIALOG_WINDOWS.replace("MB_DEFBUTTON2", "MB_DEFBUTTON1"),
            1,
        ),
    ] {
        assert_ne!(broken, source);
        assert!(!dialog_answer_problems(&broken).is_empty());
    }
}

#[test]
fn only_test_files_name_the_seam() {
    let source = repo().join("src-tauri").join("src");
    assert_eq!(seam_users_outside_tests(&source), Vec::<PathBuf>::new());
    let fabricated = tempfile::tempdir().unwrap();
    fs::write(
        fabricated.path().join("agent_import_post_tests.rs"),
        "SCRIPTED_APPROVAL",
    )
    .unwrap();
    assert!(seam_users_outside_tests(fabricated.path()).is_empty());
    fs::write(
        fabricated.path().join("agent_import_post.rs"),
        "use test_seam::ScriptedApproval;",
    )
    .unwrap();
    assert_eq!(seam_users_outside_tests(fabricated.path()).len(), 1);
}

#[test]
fn nothing_sets_cfg_test_for_a_build() {
    let tracked = tracked_files();
    let (examined, problems) = build_configuration_verdict(&tracked, |path| read(path));
    assert_eq!(problems, Vec::<String>::new());
    // A gate that stops finding its files passes vacuously: these are read today.
    for path in [
        ".github/workflows/ci.yml",
        ".github/actions/setup-windows-native/action.yml",
        "src-tauri/build.rs",
        "package.json",
        "scripts/check-no-test-seam.mjs",
        "scripts/package-mcpb.mjs",
        "glama/Dockerfile",
    ] {
        assert!(
            examined.iter().any(|seen| seen == path),
            "the gate must read {path}"
        );
    }
    // ...and every tracked build script and Cargo config, whatever else the selection keeps.
    for path in &tracked {
        let (directory, name) = path.rsplit_once('/').unwrap_or(("", path));
        let cargo_config = matches!(name, "config" | "config.toml")
            && (directory == ".cargo" || directory.ends_with("/.cargo"));
        if name == "build.rs" || cargo_config {
            assert!(examined.contains(path), "the gate must read {path}");
        }
    }
}

/// bridge#836: each way of setting `cfg(test)`, in each place a build reads, on a synthetic tree;
/// the whole verdict is asserted, so a rule that stops matching, or starts over-matching, fails.
#[test]
fn every_spelling_and_place_that_sets_cfg_test_is_refused() {
    let verdict = |files: &[(&str, &str)]| {
        let tracked: Vec<String> = files.iter().map(|(path, _)| path.to_string()).collect();
        build_configuration_verdict(&tracked, |path| {
            files
                .iter()
                .find(|(listed, _)| *listed == path)
                .map(|(_, text)| text.to_string())
                .unwrap_or_default()
        })
        .1
    };
    let clean = "name: x\non: push\n";
    for (path, text, setting) in [
        (
            ".github/workflows/x.yml",
            "env:\n  RUSTFLAGS: --cfg test\n",
            "--cfg test",
        ),
        (
            ".github/workflows/x.yml",
            "env:\n  RUSTFLAGS: \"--cfg 'test'\"\n",
            "--cfg test",
        ),
        (
            ".github/workflows/x.yml",
            "env:\n  RUSTFLAGS: --cfg=\"test\"\n",
            "--cfg= test",
        ),
        (
            ".github/workflows/x.yml",
            "env:\n  CARGO_ENCODED_RUSTFLAGS: \"--cfg\\x1ftest\"\n",
            "--cfg test",
        ),
        (
            ".github/workflows/x.yml",
            "env:\n  CARGO_ENCODED_RUSTFLAGS: \"--cfg\u{1f}test\"\n",
            "--cfg test",
        ),
        (
            ".github/workflows/x.yml",
            "run: cargo build --config \"build.rustflags=[\\\"--cfg\\\",\\\"test\\\"]\"\n",
            "--cfg test",
        ),
        (
            ".github/actions/other/action.yml",
            "runs:\n  env:\n    RUSTFLAGS: --cfg test\n",
            "--cfg test",
        ),
        (
            ".cargo/config.toml",
            "[build]\nrustflags = [\n  \"--cfg\",\n  \"test\",\n]\n",
            "--cfg test",
        ),
        (
            "tools/.cargo/config",
            "[build]\nrustflags = [\"--cfg\", \"test\"]\n",
            "--cfg test",
        ),
        (
            "package.json",
            "{\"scripts\": {\"b\": \"RUSTFLAGS='--cfg test' cargo build\"}}\n",
            "--cfg test",
        ),
        (
            "glama/Dockerfile",
            "ENV RUSTFLAGS=\"--cfg test\"\n",
            "--cfg test",
        ),
        (
            "scripts/package-mcpb.mjs",
            "spawn(\"cargo\", [\"build\"], { env: { RUSTFLAGS: \"--cfg test\" } });\n",
            "--cfg test",
        ),
    ] {
        let problems = verdict(&[(path, text), (".github/workflows/ci.yml", clean)]);
        assert_eq!(problems.len(), 1, "{path}: {text:?} -> {problems:?}");
        assert!(
            problems[0].starts_with(&format!("{path} sets cfg(test): {setting}")),
            "{path}: {text:?} -> {problems:?}"
        );
    }
    // A literal `rustc-cfg=test` in a build script is refused twice: as a cfg(test) setting and
    // as a cfg value other than a plain non-test name.
    assert_eq!(
        verdict(&[(
            "tools/x/build.rs",
            "fn main() { println!(\"cargo::rustc-cfg=test\"); }\n"
        )]),
        [
            "tools/x/build.rs sets cfg(test): rustc-cfg=test ); }",
            "tools/x/build.rs emits a cfg not a plain literal: rustc-cfg=test\"); }\n",
        ]
    );
    // A build script's cfg judged by value, and a named build script refused.
    for script in [
        "fn main() { println!(\"cargo:rustc-cfg={}\", \"test\"); }\n",
        "fn main() { let c = \"test\"; println!(\"cargo:rustc-cfg={c}\"); }\n",
        "fn main() { println!(concat!(\"cargo:rustc-cfg\", \"=test\")); }\n",
    ] {
        let problems = verdict(&[("src-tauri/crates/x/build.rs", script)]);
        assert_eq!(problems.len(), 1, "{script:?} -> {problems:?}");
        assert!(problems[0]
            .starts_with("src-tauri/crates/x/build.rs emits a cfg not a plain literal: "));
    }
    // Every TOML spelling of the build-script key (bridge#1399 review, P2 a).
    for manifest in [
        "[package]\nname = \"x\"\nbuild = \"setup.rs\"\n",
        "[package]\nname = \"x\"\n\"build\" = \"setup.rs\"\n",
        "[package]\nname = \"x\"\n'build' = 'setup.rs'\n",
        "package.name = \"x\"\npackage.build = \"setup.rs\"\n",
        "\"package\" . \"build\" = \"setup.rs\"\n",
        "package = { name = \"x\", build = \"setup.rs\" }\n",
        "[ \"package\" ]\nbuild=\"setup.rs\"\n",
    ] {
        assert_eq!(
            verdict(&[("tools/x/Cargo.toml", manifest)]),
            ["tools/x/Cargo.toml names its build script; name it build.rs"],
            "{manifest:?}"
        );
    }
    // A shell joins a word written in pieces (bridge#1399 review, P2 b): each is one setting.
    for text in [
        "run: RUSTFLAGS=\"--cfg te\"st cargo build\n",
        "run: RUSTFLAGS='--cfg te'\"st\" cargo build\n",
        "run: RUSTFLAGS=--cfg\\ te\\st cargo build\n",
        "run: RUSTFLAGS=\"--cfg \\test\" cargo build\n",
        "run: RUSTFLAGS=\"--c\"fg\" test\" cargo build\n",
    ] {
        let problems = verdict(&[(".github/workflows/x.yml", text)]);
        assert_eq!(problems.len(), 1, "{text:?} -> {problems:?}");
        assert!(
            problems[0].starts_with(".github/workflows/x.yml sets cfg(test): --cfg test"),
            "{text:?} -> {problems:?}"
        );
    }
    // What is not a cfg(test) setting, or not a place a build reads, passes.
    assert_eq!(
        verdict(&[
            (".github/workflows/x.yml", "env:\n  RUSTFLAGS: --cfg testing --cfg tests_x --cfg feature=\"test\"\n"),
            ("src-tauri/build.rs", "fn main() { println!(\"cargo:rustc-cfg=shipping\"); println!(\"cargo:rustc-check-cfg=cfg(shipping)\"); }\n"),
            ("tools/x/Cargo.toml", "[package]\nname = \"x\"\nbuild = false\n"),
            ("tools/y/Cargo.toml", "[package]\nname = \"y\"\n[dependencies]\nbuild = \"1.0\"\n"),
            ("tools/z/Cargo.toml", "[package.metadata.x]\nbuild = \"notes\"\n"),
            ("docs/notes.md", "RUSTFLAGS=--cfg test\n"),
            ("scripts/other.sh", "RUSTFLAGS=--cfg test cargo build\n"),
        ]),
        Vec::<String>::new()
    );
}

#[test]
fn every_shipped_build_runs_the_seam_scan() {
    let marker = "bridge-test-approval-seam-5f1c9e7a";
    assert!(read("src-tauri/src/tally/approved_import.rs").contains(&format!("\"{marker}\"")));
    assert!(read("scripts/check-no-test-seam.mjs").contains(&format!("\"{marker}\"")));

    let tauri: serde_json::Value =
        serde_json::from_str(&read("src-tauri/tauri.conf.json")).unwrap();
    assert_eq!(
        tauri["build"]["beforeBundleCommand"],
        "node scripts/check-no-test-seam.mjs --tauri-bundle-hook"
    );

    let package = read("scripts/package-mcpb.mjs");
    let scan = package
        .find("assertNoTestSeam([sourceBinary]);")
        .expect("package-mcpb scans the binary");
    let stage = package
        .find("await cp(sourceBinary, destination);")
        .expect("package-mcpb stages the binary");
    assert!(
        scan < stage,
        "package-mcpb must scan the binary before staging it"
    );

    let ci = read(".github/workflows/ci.yml");
    for invocation in [
        "node scripts/check-no-test-seam.mjs --test-harness\n",
        "node scripts/check-no-test-seam.mjs --test-harness --release",
        "node scripts/check-no-test-seam.mjs \"src-tauri/target/release/bridge$ext\" \"src-tauri/target/release/bridge_mcp$ext\"",
        "node scripts/check-no-test-seam.mjs src-tauri/target/release/bundle/macos",
    ] {
        assert!(ci.contains(invocation), "ci.yml must run `{}`", invocation.trim());
    }
    assert!(read(".github/workflows/release-mcpb-preview.yml").contains(
        "node scripts/check-no-test-seam.mjs src-tauri/target/release/${{ matrix.binary }}"
    ));
}

/// Problems with who may make the totals form of a post's preview (#1832): a
/// batch is shown it only after its box with the voucher lines did not fit the
/// screen, and only the approvals holder remembers that, so no other module
/// names it. `others` is every other source file of the crate that is not a
/// test file, as (name, text).
fn totals_form_problems(post: &str, approval: &str, others: &[(String, String)]) -> Vec<String> {
    let mut problems = Vec::new();
    for (name, text) in others {
        if text.contains("TotalsOnly") {
            problems.push(format!("{name} must not name PreviewForm::TotalsOnly"));
        }
    }
    let made =
        "            PreviewForm::TotalsOnly\n        } else {\n            PreviewForm::Listed";
    if approval.matches(made).count() != 1 {
        problems.push("expected the one place that makes PreviewForm::TotalsOnly".into());
    }
    // Its variant, its doc comment and its one maker: no `Self::TotalsOnly`,
    // no glob import, no second maker.
    if approval.matches("TotalsOnly").count() != 3 || approval.contains("PreviewForm::*") {
        problems
            .push("TotalsOnly is named in its definition, its doc and its one maker only".into());
    }
    // The note that remembers a batch is given only to a dialog asking the
    // listed form of a batch, so the totals form can never note itself.
    let given = "(form == PreviewForm::Listed && preview_lists_vouchers)\n                                .then(|| self.post_approvals.note_listed_did_not_fit(batch_id)),";
    if post.matches(given).count() != 1 {
        problems.push("the note must be given only for a batch's listed form".into());
    }
    problems
}

/// Every non-test source file of the crate but the approvals holder and the
/// post path, as (path, text).
fn other_sources() -> Vec<(String, String)> {
    let mut found = Vec::new();
    let mut directories = vec![repo().join("src-tauri/src")];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory).expect("src-tauri/src is readable") {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                directories.push(path);
                continue;
            }
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let is_rust = name.ends_with(".rs");
            let is_test =
                name.ends_with("_tests.rs") || name.contains("_tests_") || name == "tests.rs";
            if is_rust && !is_test && name != "agent_import_approval.rs" {
                let text = fs::read_to_string(&path).expect("a readable source file");
                found.push((path.display().to_string(), text));
            }
        }
    }
    found
}

#[test]
fn only_the_approvals_holder_makes_the_totals_form() {
    let post = read("src-tauri/src/agent_import_post.rs");
    let approval = read("src-tauri/src/agent_import_approval.rs");
    let others = other_sources();
    assert!(
        others.len() > 50,
        "the scan reads the crate: {}",
        others.len()
    );
    assert_eq!(
        totals_form_problems(&post, &approval, &others),
        Vec::<String>::new()
    );
    // The check itself: each way of widening it is caught.
    let mut named_elsewhere = others.clone();
    named_elsewhere[0]
        .1
        .push_str("\nconst X: u8 = 0; // PreviewForm::TotalsOnly\n");
    for (broken_post, broken_approval, broken_others) in [
        (post.clone(), approval.clone(), named_elsewhere),
        (
            post.clone(),
            format!("{approval}\nconst Y: PreviewForm = PreviewForm::TotalsOnly;\n"),
            others.clone(),
        ),
        (
            post.clone(),
            format!("{approval}\nuse PreviewForm::*;\n"),
            others.clone(),
        ),
        (
            post.replace(
                "(form == PreviewForm::Listed && preview_lists_vouchers)",
                "(preview_lists_vouchers)",
            ),
            approval.clone(),
            others.clone(),
        ),
        (
            post.replace(
                "(form == PreviewForm::Listed && preview_lists_vouchers)",
                "(form == PreviewForm::Listed)",
            ),
            approval.clone(),
            others.clone(),
        ),
        (
            post.replace(
                ".then(|| self.post_approvals.note_listed_did_not_fit(batch_id)),",
                ".then(|| Box::new(|| {}) as Box<dyn FnOnce() + Send>),",
            ),
            approval.clone(),
            others.clone(),
        ),
    ] {
        assert!(!totals_form_problems(&broken_post, &broken_approval, &broken_others).is_empty());
    }
}

/// `source` with each comment, string literal and character literal replaced
/// by one space, so that a word or a parenthesis inside one is not read as
/// code. Raw and byte strings are literals too; a lifetime such as `'a` is code.
fn code_only(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let identifier = |at: usize| chars[at].is_alphanumeric() || chars[at] == '_';
    let mut out = String::with_capacity(source.len());
    let mut at = 0;
    while at < chars.len() {
        let character = chars[at];
        let next = chars.get(at + 1).copied();
        if character == '/' && next == Some('/') {
            while at < chars.len() && chars[at] != '\n' {
                at += 1;
            }
            out.push(' ');
            continue;
        }
        if character == '/' && next == Some('*') {
            let mut depth = 0_usize;
            while at < chars.len() {
                if chars[at] == '/' && chars.get(at + 1) == Some(&'*') {
                    depth += 1;
                    at += 2;
                } else if chars[at] == '*' && chars.get(at + 1) == Some(&'/') {
                    depth -= 1;
                    at += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    at += 1;
                }
            }
            out.push(' ');
            continue;
        }
        let raw_prefix = character == 'r'
            && (at == 0
                || !identifier(at - 1)
                || (chars[at - 1] == 'b' && (at == 1 || !identifier(at - 2))));
        if raw_prefix {
            let mut quote = at + 1;
            while chars.get(quote) == Some(&'#') {
                quote += 1;
            }
            if chars.get(quote) == Some(&'"') {
                let hashes = quote - at - 1;
                let mut end = quote + 1;
                while end < chars.len()
                    && !(chars[end] == '"'
                        && (1..=hashes).all(|offset| chars.get(end + offset) == Some(&'#')))
                {
                    end += 1;
                }
                at = end + 1 + hashes;
                out.push(' ');
                continue;
            }
        }
        if character == '"' {
            at += 1;
            while at < chars.len() && chars[at] != '"' {
                at += if chars[at] == '\\' { 2 } else { 1 };
            }
            at += 1;
            out.push(' ');
            continue;
        }
        if character == '\'' {
            if next == Some('\\') {
                let mut end = at + 3;
                while end < chars.len() && chars[end] != '\'' {
                    end += 1;
                }
                at = end + 1;
                out.push(' ');
                continue;
            }
            if chars.get(at + 2) == Some(&'\'') {
                at += 3;
                out.push(' ');
                continue;
            }
        }
        out.push(character);
        at += 1;
    }
    out
}

/// Whether each `MessageBoxW` call in `source` names `MB_SYSTEMMODAL` among its
/// arguments, in source order (#1780): the code from the opening parenthesis
/// to the one that closes it. Comments and literals are not code, so a flag
/// named only in a comment does not count.
fn message_box_calls_are_system_modal(source: &str) -> Vec<bool> {
    let code = code_only(source);
    let mut calls = Vec::new();
    let mut rest = code.as_str();
    while let Some(at) = rest.find("MessageBoxW(") {
        let after = &rest[at + "MessageBoxW(".len()..];
        let mut depth = 1_usize;
        let mut end = after.len();
        for (index, character) in after.char_indices() {
            match character {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = index;
                        break;
                    }
                }
                _ => {}
            }
        }
        calls.push(after[..end].contains("MB_SYSTEMMODAL"));
        rest = &after[end..];
    }
    calls
}

/// A Windows box opened without `MB_SYSTEMMODAL` is drawn behind Tally when the
/// last input went to Tally seconds before it opened (#1780, #2107). Every
/// `MessageBoxW` call of `approved_import.rs` must carry it, so a box added or
/// changed later fails here rather than in review. The five boxes are the
/// post's, the review's, the stop release's, the ledger creation's and the
/// settlement's.
#[test]
fn every_windows_message_box_is_system_modal() {
    let calls = message_box_calls_are_system_modal(&read("src-tauri/src/tally/approved_import.rs"));
    assert!(
        calls.len() >= 5,
        "fewer MessageBoxW calls than the five known boxes: {calls:?}"
    );
    assert!(
        calls.iter().all(|system_modal| *system_modal),
        "a MessageBoxW call lacks MB_SYSTEMMODAL: {calls:?}"
    );
    // The check must see a missing flag.
    assert_eq!(
        message_box_calls_are_system_modal(
            "MessageBoxW(null, text, title, MB_YESNOCANCEL | MB_TOPMOST)"
        ),
        [false]
    );
    assert_eq!(
        message_box_calls_are_system_modal(
            "MessageBoxW(null, text, title, MB_YESNOCANCEL | MB_SYSTEMMODAL) MessageBoxW(a, (b), c)"
        ),
        [true, false]
    );
    // A flag named only in a comment or a literal is not the flag: each of
    // the five boxes names it in a comment as well.
    assert_eq!(
        message_box_calls_are_system_modal(
            "MessageBoxW(\n    null,\n    text,\n    // MB_SYSTEMMODAL keeps it above Tally\n    MB_YESNOCANCEL | MB_TOPMOST,\n)"
        ),
        [false]
    );
    assert_eq!(
        message_box_calls_are_system_modal(
            "MessageBoxW(null, \"MB_SYSTEMMODAL\", r#\"MB_SYSTEMMODAL\"#, /* MB_SYSTEMMODAL */ MB_TOPMOST)"
        ),
        [false]
    );
    // A parenthesis inside a literal does not end the call, and a call named
    // only in a comment is not a call.
    assert_eq!(
        message_box_calls_are_system_modal("MessageBoxW(null, \")\", ')', MB_SYSTEMMODAL)"),
        [true]
    );
    assert_eq!(
        message_box_calls_are_system_modal(
            "// MessageBoxW(null, t, c, MB_TOPMOST)\nlet a: &'a str = x;"
        ),
        Vec::<bool>::new()
    );
}
