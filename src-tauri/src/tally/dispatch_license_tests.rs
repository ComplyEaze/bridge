//! The licence is sealed: exactly `()` and `RecordedIntent` implement it, and
//! only the master journal's `append_intent` issues a `RecordedIntent`. The
//! seal itself is the compiler's (`sealed::Sealed` cannot be named outside
//! `dispatch_license`); these tests read the sources to hold the count and the
//! one issuing call, so a change to either shows up here.
use std::path::{Path, PathBuf};

fn rust_sources(directory: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_sources(&path, found);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
}

fn sources() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut paths = Vec::new();
    rust_sources(&root, &mut paths);
    paths
        .into_iter()
        .map(|path| {
            let name = path
                .strip_prefix(&root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            (name, std::fs::read_to_string(&path).unwrap())
        })
        .collect()
}

/// Lines of `text` that are code, not comments.
fn code_lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines()
        .map(str::trim_start)
        .filter(|line| !line.starts_with("//"))
}

#[test]
fn exactly_the_unit_and_the_recorded_intent_implement_the_licence() {
    let mut implementors = Vec::new();
    let mut sealers = Vec::new();
    for (name, text) in sources() {
        if name.ends_with("_tests.rs") {
            continue;
        }
        for line in code_lines(&text) {
            if let Some(rest) = line.strip_prefix("impl DispatchLicense for ") {
                implementors.push((name.clone(), rest.trim_end_matches(" {").to_string()));
            }
            if line.contains("sealed::Sealed for") {
                sealers.push(name.clone());
            }
        }
    }
    implementors.sort();
    assert_eq!(
        implementors,
        [
            ("tally/dispatch_license.rs".to_string(), "()".to_string()),
            (
                "tally/dispatch_license.rs".to_string(),
                "RecordedIntent".to_string()
            ),
        ]
    );
    assert_eq!(sealers, ["tally/dispatch_license.rs"; 2]);
}

#[test]
fn only_the_journals_append_issues_a_recorded_intent() {
    let mut issuing = Vec::new();
    for (name, text) in sources() {
        if name.ends_with("_tests.rs") {
            continue;
        }
        let calls = code_lines(&text)
            .filter(|line| line.contains("RecordedIntent::issue("))
            .count();
        if calls > 0 {
            issuing.push((name, calls));
        }
    }
    issuing.sort();
    assert_eq!(issuing, [("agent_master_ledger.rs".to_string(), 1)]);
    // The test-only constructor is under bare `cfg(test)`.
    let license = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tally/dispatch_license.rs"),
    )
    .unwrap();
    assert!(license.contains("#[cfg(test)]\n    pub(crate) fn issued_for_test("));
}
