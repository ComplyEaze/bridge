//! The crates below the application stay clean: `bridge-tally-primitives`, `bridge-tally-protocol`
//! and `bridge-tally-transport` reach no application crate, no `tauri`, no database driver and no
//! storage or delivery crate through their normal and build dependencies, and each reaches exactly
//! the first-party crates listed in `EXPECTED_FIRST_PARTY` and no others (AGENTS.md P8: protocol
//! and canonical layers must not depend on transport, storage, or Tauri).
//!
//! `scripts/check-tally-live-read-boundary.mjs` (removed with the `tools/` workspace, #1745) made
//! this check through `cargo tree --edges normal` over the read tool's dependencies, which reached
//! these three crates; nothing else asked the question. This test asks it without running cargo,
//! and over a superset (the lock file is resolved once for the whole workspace, so it merges
//! features and targets, and build dependencies are followed too; a false alarm is possible, a
//! miss is not): it reads each crate's `Cargo.toml` (normal, build and target-specific dependency
//! tables, a renamed `package =`, and a dependency inherited from the workspace) and then follows the
//! resolved graph in `src-tauri/Cargo.lock`. A package that is not first-party is followed through
//! its lock entry, which carries its normal and build dependencies only; a first-party crate is
//! followed through its own manifest, so that a dev-dependency of one first-party crate (the
//! protocol crate's simulator, which depends on the core crate) does not count. Dev-dependencies
//! of the three crates themselves are not part of what ships and are not followed, as before.
//! It runs in `Tally portable core` on every pull request.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use toml::{Table, Value};

const LOWER_CRATES: [&str; 3] = [
    "bridge-tally-primitives",
    "bridge-tally-protocol",
    "bridge-tally-transport",
];

/// The first-party crates each lower crate may reach, itself excluded. Adding one is a deliberate
/// change to a pinned file.
const EXPECTED_FIRST_PARTY: [(&str, &[&str]); 3] = [
    ("bridge-tally-primitives", &[]),
    ("bridge-tally-protocol", &["bridge-tally-primitives"]),
    (
        "bridge-tally-transport",
        &["bridge-tally-primitives", "bridge-tally-protocol"],
    ),
];

const FORBIDDEN: [&str; 10] = [
    "bridge",
    "bridge-tally-canonical",
    "bridge-tally-core",
    "bridge-tally-incremental",
    "bridge-tally-observability",
    "libsqlite3-sys",
    "rusqlite",
    "sqlx",
    "tauri",
    "tauri-runtime",
];

fn table(value: &Value, key: &str) -> Table {
    value
        .get(key)
        .and_then(Value::as_table)
        .cloned()
        .unwrap_or_default()
}

/// Every table named `dependencies` or `build-dependencies` of a manifest, top level and under
/// `[target.<cfg>]`. Dev-dependencies are left out.
fn shipped_tables(manifest: &Value) -> Vec<Table> {
    let mut found = Vec::new();
    for kind in ["dependencies", "build-dependencies"] {
        found.push(table(manifest, kind));
    }
    for (_, target) in table(manifest, "target") {
        for kind in ["dependencies", "build-dependencies"] {
            found.push(table(&target, kind));
        }
    }
    found
}

/// The packages a manifest's normal and build dependencies resolve to: the key, or the `package`
/// it renames, taken from the workspace table when the entry is inherited.
fn shipped_packages(manifest: &Value, workspace_root: &Value) -> BTreeSet<String> {
    let inherited = table(
        &workspace_root
            .get("workspace")
            .cloned()
            .unwrap_or(Value::Table(Table::new())),
        "dependencies",
    );
    let mut packages = BTreeSet::new();
    for dependencies in shipped_tables(manifest) {
        for (key, spec) in dependencies {
            let mut rename = spec
                .get("package")
                .and_then(Value::as_str)
                .map(str::to_string);
            if spec.get("workspace").and_then(Value::as_bool) == Some(true) {
                if let Some(entry) = inherited.get(&key) {
                    rename = entry
                        .get("package")
                        .and_then(Value::as_str)
                        .map(str::to_string);
                }
            }
            packages.insert(rename.unwrap_or(key));
        }
    }
    packages
}

/// What a resolver needs: how to read a first-party crate's manifest, and the lock file's graph.
struct Graph {
    workspace_root: Value,
    lock: BTreeMap<String, (bool, BTreeSet<String>)>,
    manifests: BTreeMap<String, Value>,
}

impl Graph {
    fn new(
        workspace_root: &str,
        lock_text: &str,
        manifests: &[(&str, &str)],
    ) -> Result<Self, String> {
        let workspace_root: Value = toml::from_str(workspace_root)
            .map_err(|error| format!("workspace manifest: {error}"))?;
        let lock: Value =
            toml::from_str(lock_text).map_err(|error| format!("lock file: {error}"))?;
        let mut graph = BTreeMap::new();
        for package in lock
            .get("package")
            .and_then(Value::as_array)
            .ok_or("the lock file lists no packages")?
        {
            let name = package
                .get("name")
                .and_then(Value::as_str)
                .ok_or("a lock entry has no name")?
                .to_string();
            let first_party = package.get("source").is_none();
            let entry = graph.entry(name).or_insert((first_party, BTreeSet::new()));
            for dependency in package
                .get("dependencies")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
            {
                // "name", "name 1.2.3" or "name 1.2.3 (registry+...)".
                let dependency = dependency.split(' ').next().unwrap_or(dependency);
                entry.1.insert(dependency.to_string());
            }
        }
        let mut parsed = BTreeMap::new();
        for (name, text) in manifests {
            let manifest: Value =
                toml::from_str(text).map_err(|error| format!("manifest of {name}: {error}"))?;
            parsed.insert((*name).to_string(), manifest);
        }
        Ok(Self {
            workspace_root,
            lock: graph,
            manifests: parsed,
        })
    }

    /// The packages `start` (a first-party crate) reaches through normal and build dependencies,
    /// and which of them are first-party.
    fn reach(&self, start: &str) -> Result<(BTreeSet<String>, BTreeSet<String>), String> {
        let mut seen = BTreeSet::new();
        let mut first_party = BTreeSet::new();
        let mut queue = vec![start.to_string()];
        while let Some(name) = queue.pop() {
            if !seen.insert(name.clone()) {
                continue;
            }
            let (is_first_party, lock_dependencies) = self
                .lock
                .get(&name)
                .ok_or_else(|| format!("{name} is not in the lock file"))?;
            let next: BTreeSet<String> = if *is_first_party {
                first_party.insert(name.clone());
                let manifest = self
                    .manifests
                    .get(&name)
                    .ok_or_else(|| format!("first-party crate {name} has no manifest to read"))?;
                let declared = manifest
                    .get("package")
                    .and_then(|package| package.get("name"))
                    .and_then(Value::as_str);
                if declared.is_some_and(|declared| declared != name) {
                    return Err(format!(
                        "the manifest read for {name} declares {declared:?}"
                    ));
                }
                shipped_packages(manifest, &self.workspace_root)
            } else {
                lock_dependencies.clone()
            };
            queue.extend(next);
        }
        seen.remove(start);
        first_party.remove(start);
        Ok((seen, first_party))
    }
}

/// The problems with one lower crate: a forbidden package reached, the wrong first-party set, or
/// a reading that saw nothing.
fn problems(graph: &Graph, start: &str, expected: &[&str]) -> Vec<String> {
    let (reached, first_party) = match graph.reach(start) {
        Ok(found) => found,
        Err(error) => return vec![error],
    };
    let mut found = Vec::new();
    if !reached.contains("serde") {
        found.push(format!(
            "{start}: the graph shows no serde, so it was not read"
        ));
    }
    for name in FORBIDDEN.iter().filter(|name| reached.contains(**name)) {
        found.push(format!("{start} reaches forbidden package {name}"));
    }
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    if first_party != expected {
        found.push(format!(
            "{start} reaches first-party crates {first_party:?}, expected {expected:?}"
        ));
    }
    found
}

fn source_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn read(path: PathBuf) -> String {
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

#[test]
fn the_lower_crates_reach_nothing_forbidden_and_only_their_expected_first_party_crates() {
    let root = source_root();
    let workspace = read(root.join("Cargo.toml"));
    let lock = read(root.join("Cargo.lock"));
    // Every first-party package in the lock file has a manifest here: a crate directory of the
    // same name, or the application's own.
    let lock_value: Value = toml::from_str(&lock).expect("the lock file parses");
    let mut manifests: Vec<(String, String)> = Vec::new();
    for package in lock_value["package"].as_array().expect("packages") {
        if package.get("source").is_none() {
            let name = package["name"].as_str().expect("name").to_string();
            let path = if name == "bridge" {
                root.join("Cargo.toml")
            } else {
                root.join("crates").join(&name).join("Cargo.toml")
            };
            if path.exists() {
                manifests.push((name, read(path)));
            }
        }
    }
    let manifests: Vec<(&str, &str)> = manifests
        .iter()
        .map(|(name, text)| (name.as_str(), text.as_str()))
        .collect();
    let graph = Graph::new(&workspace, &lock, &manifests).expect("the graph is built");
    for (crate_name, expected) in EXPECTED_FIRST_PARTY {
        assert!(LOWER_CRATES.contains(&crate_name));
        assert_eq!(problems(&graph, crate_name, expected), Vec::<String>::new());
    }
}

// The cases below use a small invented graph: a lower crate `low`, a first-party crate `sim`,
// third-party crates `serde`, `thirdy` and `driver`.
const ROOT: &str = "[workspace]\nmembers = [\"low\"]\n[workspace.dependencies]\nrenamed = { package = \"rusqlite\", version = \"1\" }\n";
const LOCK: &str = r#"
[[package]]
name = "low"
version = "0.1.0"
dependencies = ["serde"]

[[package]]
name = "sim"
version = "0.1.0"
dependencies = ["serde"]

[[package]]
name = "core-ish"
version = "0.1.0"

[[package]]
name = "serde"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"

[[package]]
name = "thirdy"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
dependencies = ["serde", "libsqlite3-sys 0.30.0"]

[[package]]
name = "libsqlite3-sys"
version = "0.30.0"
source = "registry+https://github.com/rust-lang/crates.io-index"

[[package]]
name = "rusqlite"
version = "0.32.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
dependencies = ["libsqlite3-sys 0.30.0"]

[[package]]
name = "tauri"
version = "2.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
dependencies = ["serde"]

[[package]]
name = "bridge-tally-core"
version = "0.1.0"
dependencies = ["serde"]
"#;

fn graph_of(low: &str) -> Graph {
    let sim = "[dependencies]\nserde = \"1\"\n[dev-dependencies]\nbridge-tally-core = { path = \"../core\" }\n";
    let core = "[dependencies]\nserde = \"1\"\n";
    Graph::new(
        ROOT,
        LOCK,
        &[("low", low), ("sim", sim), ("bridge-tally-core", core)],
    )
    .expect("the invented graph is built")
}

fn report(low: &str, expected: &[&str]) -> Vec<String> {
    problems(&graph_of(low), "low", expected)
}

#[test]
fn a_clean_crate_and_a_dev_only_edge_to_a_forbidden_crate_pass() {
    assert_eq!(
        report("[dependencies]\nserde = \"1\"\n", &[]),
        Vec::<String>::new()
    );
    // `sim` has a dev-dependency on a forbidden crate; dev edges are not followed.
    let with_sim = "[dependencies]\nserde = \"1\"\nsim = { path = \"../sim\" }\n";
    assert_eq!(report(with_sim, &["sim"]), Vec::<String>::new());
    // The crate's own dev-dependency is not followed either.
    let own_test_edges =
        "[dependencies]\nserde = \"1\"\n[dev-dependencies]\nrusqlite = \"0.32\"\ntauri = \"2\"\n";
    assert_eq!(report(own_test_edges, &[]), Vec::<String>::new());
    // A name that is only a feature, or the crate's own name, is not a dependency.
    let feature =
        "[package]\nname = \"low\"\n[features]\ntauri = []\n[dependencies]\nserde = \"1\"\n";
    assert_eq!(report(feature, &[]), Vec::<String>::new());
}

#[test]
fn a_forbidden_package_is_found_however_it_is_named_or_reached() {
    let cases = [
        ("direct", "[dependencies]\nserde = \"1\"\nrusqlite = \"0.32\"\n", "rusqlite"),
        ("build table", "[dependencies]\nserde = \"1\"\n[build-dependencies]\ntauri = \"2\"\n", "tauri"),
        (
            "target table",
            "[dependencies]\nserde = \"1\"\n[target.'cfg(windows)'.dependencies]\ntauri = \"2\"\n",
            "tauri",
        ),
        (
            "rename in an inline table",
            "[dependencies]\nserde = \"1\"\ndb = { package = \"rusqlite\", version = \"0.32\" }\n",
            "rusqlite",
        ),
        (
            "rename in a named table",
            "[dependencies]\nserde = \"1\"\n[dependencies.db]\nversion = \"0.32\"\npackage = \"rusqlite\"\n",
            "rusqlite",
        ),
        (
            "rename by a dotted key",
            "[dependencies]\nserde = \"1\"\ndb.package = \"rusqlite\"\ndb.version = \"0.32\"\n",
            "rusqlite",
        ),
        (
            "rename inherited from the workspace",
            "[dependencies]\nserde = \"1\"\nrenamed.workspace = true\n",
            "rusqlite",
        ),
        (
            "header with a trailing comment",
            "[lints]\nworkspace = true\n[dependencies] # shipped\nserde = \"1\"\nrusqlite = \"0.32\"\n",
            "rusqlite",
        ),
        (
            "through a third-party crate",
            "[dependencies]\nserde = \"1\"\nthirdy = \"1\"\n",
            "libsqlite3-sys",
        ),
        (
            "through a first-party crate's own shipped edge",
            "[dependencies]\nserde = \"1\"\nbridge-tally-core = { path = \"../core\" }\n",
            "bridge-tally-core",
        ),
    ];
    for (label, low, forbidden) in cases {
        let found = report(low, &["bridge-tally-core"]);
        assert!(
            found
                .iter()
                .any(|problem| problem.contains(&format!("forbidden package {forbidden}"))),
            "{label}: {found:?}"
        );
    }
}

#[test]
fn a_first_party_crate_outside_the_expected_set_is_found() {
    let low = "[dependencies]\nserde = \"1\"\nsim = { path = \"../sim\" }\n";
    let found = report(low, &[]);
    assert!(
        found
            .iter()
            .any(|problem| problem.contains("first-party crates")),
        "{found:?}"
    );
}

#[test]
fn a_reading_that_saw_nothing_or_an_unknown_crate_is_an_error() {
    // No dependency at all: nothing was read, so "nothing forbidden" would mean nothing.
    let found = report("[package]\nname = \"low\"\n", &[]);
    assert!(
        found.iter().any(|problem| problem.contains("no serde")),
        "{found:?}"
    );
    // A first-party crate with no manifest to read.
    let graph = Graph::new(
        ROOT,
        LOCK,
        &[("low", "[dependencies]\nsim = { path = \"../sim\" }\n")],
    )
    .unwrap();
    let found = problems(&graph, "low", &["sim"]);
    assert!(
        found.iter().any(|problem| problem.contains("no manifest")),
        "{found:?}"
    );
    // A dependency the lock file does not know.
    let found = report("[dependencies]\nserde = \"1\"\nghost = \"1\"\n", &[]);
    assert!(
        found
            .iter()
            .any(|problem| problem.contains("not in the lock file")),
        "{found:?}"
    );
}
