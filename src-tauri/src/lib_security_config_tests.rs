// The webview's security configuration is pinned by exact equality. A substring check on the CSP
// still passes with a widened source list, and checking one capability file misses a second file,
// an inline capability or a platform override config. Change a value here only together with the
// config, deliberately.

use std::path::Path;

fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

// Entries Tauri could read as configuration: subdirectories (capabilities are discovered
// recursively) and files with a JSON, JSON5 or TOML extension. Extensions are matched in any
// letter case, a superset of the lower-case ones Tauri loads.
fn config_entries(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(directory)
        .expect("readable directory")
        .map(|entry| entry.expect("readable entry"))
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            entry.path().is_dir()
                || [".json", ".json5", ".toml"]
                    .iter()
                    .any(|extension| name.ends_with(extension))
        })
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn renderer_receives_only_native_lifecycle_event_permissions() {
    // Tauri grants every capability file in this directory, so default.json must be the only one.
    assert_eq!(
        config_entries(&manifest_dir().join("capabilities")),
        ["default.json"]
    );
    let capability: serde_json::Value =
        serde_json::from_str(include_str!("../capabilities/default.json"))
            .expect("valid capability JSON");
    assert_eq!(
        capability,
        serde_json::json!({
            "$schema": "../gen/schemas/desktop-schema.json",
            "identifier": "default",
            "description": "Default desktop permissions for Bridge",
            "windows": ["main"],
            "permissions": ["core:event:allow-listen", "core:event:allow-unlisten"]
        })
    );
}

#[test]
fn production_csp_has_no_remote_browser_egress_or_inline_code() {
    let config: serde_json::Value =
        serde_json::from_str(include_str!("../tauri.conf.json")).expect("valid Tauri config");
    assert_eq!(
        config["app"],
        serde_json::json!({
            "windows": [{
                "title": "Bridge",
                "width": 1180,
                "height": 760,
                "minWidth": 520,
                "minHeight": 620
            }],
            "security": {
                "csp": "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self'; font-src 'self'; connect-src ipc: http://ipc.localhost; object-src 'none'; base-uri 'none'; form-action 'none'; frame-src 'none'; frame-ancestors 'none'",
                "devCsp": "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self'; font-src 'self'; connect-src ipc: http://ipc.localhost http://127.0.0.1:5174 ws://127.0.0.1:5174; object-src 'none'; base-uri 'none'; form-action 'none'; frame-src 'none'; frame-ancestors 'none'"
            }
        })
    );
    assert_eq!(
        config["build"],
        serde_json::json!({
            "beforeDevCommand": "corepack pnpm run dev",
            "devUrl": "http://127.0.0.1:5174",
            "beforeBuildCommand": "corepack pnpm run build",
            "beforeBundleCommand": "node scripts/check-no-test-seam.mjs --tauri-bundle-hook",
            "frontendDist": "../dist",
        })
    );
}

#[test]
fn no_file_overrides_the_tauri_config() {
    // Tauri merges a platform file (tauri.macos.conf.json and the like) over tauri.conf.json, so any
    // other tauri.* config beside it could replace these values. JSON5 and TOML forms are refused
    // too, although this build does not enable Tauri's features that read them.
    let overrides: Vec<String> = config_entries(manifest_dir())
        .into_iter()
        .filter(|name| {
            let lower = name.to_ascii_lowercase();
            lower.starts_with("tauri.") && lower != "tauri.conf.json"
        })
        .collect();
    assert!(overrides.is_empty(), "override configs: {overrides:?}");

    // CI's bundle builds run this script, and a config passed on its command line would override
    // the file as well.
    let package: serde_json::Value =
        serde_json::from_str(include_str!("../../package.json")).expect("valid package.json");
    assert_eq!(package["scripts"]["tauri:build"], "tauri build");
}
