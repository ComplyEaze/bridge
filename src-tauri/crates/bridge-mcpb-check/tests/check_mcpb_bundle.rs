// SPDX-License-Identifier: Apache-2.0
//! The checks of `examples/check_mcpb_bundle.rs`, the MCPB bundle check (#1810). Every archive here is
//! built by the test itself with the `zip` crate's writer; a case the writer refuses to make (a duplicate
//! name, a local header naming another file) is made by patching the bytes it wrote. The checks inside the
//! server runs are tested against an in-process stand-in server.

#[allow(dead_code)] // the example's `main` and argument handling are not called from here
#[path = "../examples/check_mcpb_bundle.rs"]
mod checker;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;

use checker::{Failure, Limits, LIMITS};

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("src-tauri/crates/bridge-mcpb-check is three levels below the repository")
        .to_path_buf()
}

fn committed_manifest() -> Value {
    let text = fs::read_to_string(repository().join("packaging/mcpb/manifest.json"))
        .expect("the committed manifest");
    serde_json::from_str(&text).expect("the committed manifest is JSON")
}

fn failure(code: &str) -> Failure {
    Failure(code.to_string())
}

fn hex_sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Whether this host bundles PDFium, as the Python test's `sys.platform in PDFIUM_LIBRARIES`.
fn host_bundles_pdfium() -> bool {
    !checker::pdfium_library(checker::host_platform()).is_empty()
}

const ENTRY: &str = "bin/host/bridge_mcp";
const FIXTURE: &[u8] = b"packaging fixture";
const LIBRARY_BYTES: &[u8] = b"library fixture";
const NOTICE_BYTES: &[u8] = b"notice fixture";

/// One member the builder writes: its name, bytes and Unix mode (`None` leaves the writer's default).
struct Entry {
    name: String,
    bytes: Vec<u8>,
    mode: Option<u32>,
}

impl Entry {
    fn new(name: &str, bytes: &[u8]) -> Self {
        Self {
            name: name.to_string(),
            bytes: bytes.to_vec(),
            mode: None,
        }
    }
}

/// A bundle and the repository it is checked against. By default it passes every check of
/// `unpack_bundle`: the repository holds a lock that pins the fixture library and notice for both
/// bundled hosts, so the run reaches the extraction and the execute bit.
struct Bundle {
    manifest: Value,
    entries: Vec<Entry>,
    real_lock: bool,
}

impl Bundle {
    fn valid() -> Self {
        let platform = checker::host_platform();
        let library = format!("bin/host/{}", checker::pdfium_library(platform));
        let manifest = json!({
            "icon": checker::ICON,
            "server": {"type": "binary", "entry_point": ENTRY, "mcp_config": {"command": format!("${{__dirname}}/{ENTRY}")}},
            "compatibility": {"platforms": [platform]},
        });
        let mut entries = vec![Entry::new(checker::ICON, &icon())];
        entries.extend(
            checker::RESOURCES
                .iter()
                .map(|resource| Entry::new(resource, FIXTURE)),
        );
        entries.push(Entry {
            mode: Some(0o100_755),
            ..Entry::new(ENTRY, b"binary fixture")
        });
        entries.push(Entry::new(&library, LIBRARY_BYTES));
        entries.push(Entry::new(checker::PDFIUM_NOTICE, NOTICE_BYTES));
        Self {
            manifest,
            entries,
            real_lock: false,
        }
    }

    fn entry(&mut self, name: &str) -> &mut Entry {
        self.entries
            .iter_mut()
            .find(|entry| entry.name == name)
            .expect("a member of the fixture")
    }

    fn library(&self) -> String {
        format!(
            "bin/host/{}",
            checker::pdfium_library(checker::host_platform())
        )
    }

    /// Write the archive and the repository under `root`; returns their paths.
    fn write(&self, root: &Path) -> (PathBuf, PathBuf) {
        let repository = root.join("repository");
        fs::create_dir_all(repository.join("packaging/pdfium")).unwrap();
        fs::create_dir_all(repository.join("packaging/mcpb")).unwrap();
        let lock = if self.real_lock {
            fs::read(repository_lock()).unwrap()
        } else {
            let pin = json!({
                "library_bytes": LIBRARY_BYTES.len(), "library_sha256": hex_sha256(LIBRARY_BYTES),
                "notice_bytes": NOTICE_BYTES.len(), "notice_sha256": hex_sha256(NOTICE_BYTES),
            });
            serde_json::to_vec(&json!({"platforms": {"macos-arm64": pin, "windows-x64": pin}}))
                .unwrap()
        };
        fs::write(repository.join("packaging/pdfium/pdfium.lock.json"), lock).unwrap();
        for resource in checker::RESOURCES {
            fs::write(repository.join(resource), FIXTURE).unwrap();
        }
        fs::write(repository.join(checker::ICON_SOURCE), icon()).unwrap();
        let archive = root.join("bundle.mcpb");
        let mut writer = zip::ZipWriter::new(fs::File::create(&archive).unwrap());
        writer
            .start_file("manifest.json", SimpleFileOptions::default())
            .unwrap();
        writer
            .write_all(&serde_json::to_vec(&self.manifest).unwrap())
            .unwrap();
        for entry in &self.entries {
            let options = match entry.mode {
                Some(mode) => SimpleFileOptions::default().external_attributes(mode << 16),
                None => SimpleFileOptions::default(),
            };
            writer.start_file(entry.name.as_str(), options).unwrap();
            writer.write_all(&entry.bytes).unwrap();
        }
        writer.finish().unwrap();
        (archive, repository)
    }
}

fn repository_lock() -> PathBuf {
    repository().join("packaging/pdfium/pdfium.lock.json")
}

fn icon() -> Vec<u8> {
    [checker::PNG_SIGNATURE, b"icon fixture"].concat()
}

fn unpack(bundle: &Bundle, limits: Limits) -> (Result<PathBuf, Failure>, tempfile::TempDir) {
    let root = tempfile::tempdir().unwrap();
    let (archive, repository) = bundle.write(root.path());
    let result =
        checker::unpack_bundle(&archive, &root.path().join("unpacked"), &repository, limits)
            .map(|(_, binary)| binary);
    (result, root)
}

/// Replace every occurrence of `from` in the file with `to`, of the same length; returns how many.
#[derive(Clone, Copy)]
enum Copies {
    All,
    /// A name's first copy is its local header, its last the central directory's record.
    First,
    Last,
}

fn patch(path: &Path, from: &[u8], to: &[u8], copies: Copies) -> usize {
    assert_eq!(from.len(), to.len());
    let mut bytes = fs::read(path).unwrap();
    let mut found: Vec<usize> = bytes
        .windows(from.len())
        .enumerate()
        .filter(|(_, window)| *window == from)
        .map(|(at, _)| at)
        .collect();
    match copies {
        Copies::All => {}
        Copies::First => found.truncate(1),
        Copies::Last => {
            found.drain(..found.len().saturating_sub(1));
        }
    }
    for at in &found {
        bytes[*at..*at + to.len()].copy_from_slice(to);
    }
    fs::write(path, bytes).unwrap();
    found.len()
}

#[test]
fn tool_lists_are_one_name_per_line_sorted_without_duplicates() {
    // Two pull requests that add different tools then touch different lines.
    let source = include_str!("../examples/check_mcpb_bundle.rs");
    for (name, list) in [
        ("DEFAULT_TOOLS", checker::DEFAULT_TOOLS),
        ("IMPORT_TOOLS", checker::IMPORT_TOOLS),
        ("POSTING_TOOLS", checker::POSTING_TOOLS),
    ] {
        let mut sorted = list.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(list, sorted.as_slice(), "{name}");
        let start = source
            .find(&format!("pub const {name}: &[&str] = &[\n"))
            .expect(name);
        let body = &source[start..start + source[start..].find("\n];").expect(name)];
        let lines: Vec<&str> = body.lines().skip(1).map(str::trim).collect();
        let expected: Vec<String> = list.iter().map(|tool| format!("\"{tool}\",")).collect();
        assert_eq!(lines, expected, "{name}: one name per line");
    }
}

#[test]
fn schema_receipts_require_matching_write_completion() {
    let response = b"{\"jsonrpc\":\"2.0\",\"id\":3,\"result\":{}}\n";
    let digest = hex_sha256(response);
    let prepared = json!({"record_type": "response_prepared", "tool": "voucher_schema", "receipt_id": "test-receipt",
        "bytes_prepared": response.len(), "response_sha256": digest});
    let completed = json!({"record_type": "stdio_write_completed", "receipt_id": "test-receipt",
        "bytes_written": response.len(), "response_sha256": digest});
    let encoded = |records: &[&Value]| -> Vec<u8> {
        records
            .iter()
            .flat_map(|record| [serde_json::to_vec(record).unwrap(), b"\n".to_vec()].concat())
            .collect()
    };
    assert_eq!(
        checker::validate_schema_receipts(&encoded(&[&prepared, &completed]), response),
        Ok(2)
    );
    assert_eq!(
        checker::validate_schema_receipts(&encoded(&[&prepared]), response),
        Err(failure("schema_egress_completion_missing"))
    );
    for (key, value) in [
        ("receipt_id", json!("other")),
        ("response_sha256", json!("wrong")),
        ("bytes_written", json!(1)),
        ("record_type", json!("response_prepared")),
    ] {
        let mut changed = completed.clone();
        changed[key] = value;
        assert_eq!(
            checker::validate_schema_receipts(&encoded(&[&prepared, &changed]), response),
            Err(failure("schema_egress_completion_invalid")),
            "{key}"
        );
    }
}

#[test]
fn entry_point_cannot_escape_extraction_directory() {
    let mut bundle = Bundle::valid();
    bundle.manifest = json!({"server": {"entry_point": "bin/../../outside"}});
    let (result, root) = unpack(&bundle, LIMITS);
    assert_eq!(result, Err(failure("invalid_archive_member_path")));
    assert!(!root.path().join("outside").exists());
    assert!(!root.path().join("unpacked").exists());
}

#[test]
fn a_duplicate_member_name_is_refused() {
    // The writer refuses a duplicate name, so the notice is written as `LICENS@` and renamed in place, in
    // both its local header and the central directory.
    let mut bundle = Bundle::valid();
    bundle.entry(checker::PDFIUM_NOTICE).name = "LICENS@".to_string();
    let root = tempfile::tempdir().unwrap();
    let (archive, repository) = bundle.write(root.path());
    assert_eq!(patch(&archive, b"LICENS@", b"LICENSE", Copies::All), 2);
    // The crate opens it but keeps one entry per name, so its count alone would call this a missing
    // member; the checker counts the central directory's records instead.
    let reader =
        zip::ZipArchive::new(fs::File::open(&archive).unwrap()).expect("the crate opens it");
    assert_eq!(reader.len(), checker::ARCHIVE_MEMBERS - 1);
    assert_eq!(
        checker::unpack_bundle(&archive, &root.path().join("unpacked"), &repository, LIMITS)
            .map(|_| ()),
        Err(failure("duplicate_archive_member"))
    );
}

#[test]
fn a_local_header_that_names_another_file_is_refused() {
    // The local header names another file: Python's zipfile refuses it when it reads the member.
    for (copies, code) in [
        (Copies::First, "BadZipFile"),
        // The central directory's record names another file: the member set is wrong first, as in Python.
        (Copies::Last, "unexpected_archive_members"),
    ] {
        let root = tempfile::tempdir().unwrap();
        let (archive, repository) = Bundle::valid().write(root.path());
        assert_eq!(patch(&archive, b"NOTICE", b"NOTICX", copies), 1);
        assert_eq!(
            checker::unpack_bundle(&archive, &root.path().join("unpacked"), &repository, LIMITS)
                .map(|_| ()),
            Err(failure(code)),
            "{code}"
        );
    }
}

#[test]
fn a_bundle_over_either_size_cap_is_refused() {
    assert_eq!(
        (LIMITS.archive_bytes, LIMITS.unpacked_bytes),
        (128 * 1024 * 1024, 128 * 1024 * 1024)
    );
    let mut bundle = Bundle::valid();
    bundle.entry(ENTRY).bytes = vec![0; 64 * 1024];
    let (result, _root) = unpack(
        &bundle,
        Limits {
            archive_bytes: 100,
            unpacked_bytes: u64::MAX,
        },
    );
    assert_eq!(result, Err(failure("archive_too_large")));
    // Zeros deflate to a small archive: the archive is under its cap, and what it unpacks to is not.
    let (result, root) = unpack(
        &bundle,
        Limits {
            archive_bytes: 16 * 1024,
            unpacked_bytes: 32 * 1024,
        },
    );
    assert!(fs::metadata(root.path().join("bundle.mcpb")).unwrap().len() < 16 * 1024);
    assert_eq!(result, Err(failure("unpacked_archive_too_large")));
    assert!(!root.path().join("unpacked").exists());
}

#[test]
fn a_member_that_is_not_a_regular_file_is_refused() {
    for mode in [0o040_755, 0o120_777, 0o010_644] {
        let mut bundle = Bundle::valid();
        bundle.entry(checker::PDFIUM_NOTICE).mode = Some(mode);
        let (result, _root) = unpack(&bundle, LIMITS);
        assert_eq!(
            result,
            Err(failure("archive_member_is_not_regular_file")),
            "{mode:o}"
        );
    }
}

#[cfg(unix)]
#[test]
fn the_execute_bit_must_be_on_the_binary_not_another_member() {
    use std::os::unix::fs::PermissionsExt;
    if !host_bundles_pdfium() {
        return;
    }
    let (binary, _root) = unpack(&Bundle::valid(), LIMITS);
    let binary = binary.expect("the valid fixture unpacks");
    assert_eq!(
        fs::metadata(&binary).unwrap().permissions().mode() & 0o777,
        0o755
    );
    let mut bundle = Bundle::valid();
    let library = bundle.library();
    bundle.entry(ENTRY).mode = Some(0o100_644);
    bundle.entry(&library).mode = Some(0o100_755);
    let (result, _root) = unpack(&bundle, LIMITS);
    assert_eq!(result, Err(failure("binary_is_not_executable")));
    // A mode with no file type, as Python's zipfile writes one, is a regular file without the bit.
    let mut bundle = Bundle::valid();
    bundle.entry(ENTRY).mode = Some(0o600);
    let (result, _root) = unpack(&bundle, LIMITS);
    assert_eq!(result, Err(failure("binary_is_not_executable")));
}

#[test]
fn bundled_pdfium_must_match_the_lock() {
    if !host_bundles_pdfium() || checker::pdfium_pin(&repository(), None).is_err() {
        return;
    }
    let mut bundle = Bundle::valid();
    bundle.real_lock = true;
    let (result, _root) = unpack(&bundle, LIMITS);
    assert_eq!(result, Err(failure("pdfium_library_not_pinned")));
    let mut bundle = Bundle::valid();
    bundle.entry(checker::PDFIUM_NOTICE).bytes = b"another notice".to_vec();
    let (result, _root) = unpack(&bundle, LIMITS);
    assert_eq!(result, Err(failure("pdfium_notice_not_pinned")));
}

#[test]
fn bundle_without_the_library_beside_the_binary_is_refused() {
    if !host_bundles_pdfium() {
        return;
    }
    let mut bundle = Bundle::valid();
    let library = bundle.library();
    bundle.entry(&library).name = "bin/host/other".to_string();
    let (result, _root) = unpack(&bundle, LIMITS);
    assert_eq!(result, Err(failure("unexpected_archive_members")));
}

type Change = dyn Fn(&mut Bundle);

#[test]
fn the_icon_must_be_present_named_by_the_manifest_and_the_committed_file() {
    if !host_bundles_pdfium() {
        return;
    }
    let absent = |bundle: &mut Bundle| bundle.entries.retain(|entry| entry.name != checker::ICON);
    let renamed = |bundle: &mut Bundle| bundle.manifest["icon"] = json!("logo.png");
    let not_png = |bundle: &mut Bundle| bundle.entry(checker::ICON).bytes = b"not a png".to_vec();
    let other = |bundle: &mut Bundle| {
        bundle.entry(checker::ICON).bytes = [checker::PNG_SIGNATURE, b"another image"].concat()
    };
    let cases: [(&Change, &str); 4] = [
        (&absent, "unexpected_archive_members"),
        (&renamed, "icon_declaration_mismatch"),
        (&not_png, "icon_is_not_png"),
        (&other, "icon_bytes_differ"),
    ];
    for (change, code) in cases {
        let mut bundle = Bundle::valid();
        change(&mut bundle);
        let (result, _root) = unpack(&bundle, LIMITS);
        assert_eq!(result, Err(failure(code)), "{code}");
    }
}

#[test]
fn the_committed_manifest_names_the_committed_icon() {
    assert_eq!(committed_manifest()["icon"], json!(checker::ICON));
    assert!(fs::read(repository().join(checker::ICON_SOURCE))
        .unwrap()
        .starts_with(checker::PNG_SIGNATURE));
}

#[test]
fn an_unpinned_host_is_refused() {
    let repository = repository();
    assert_eq!(
        checker::pdfium_pin(&repository, Some(("linux", "x86_64"))),
        Err(failure("pdfium_platform_not_pinned"))
    );
    for host in [("darwin", "arm64"), ("win32", "amd64")] {
        let pin = checker::pdfium_pin(&repository, Some(host)).unwrap();
        assert_eq!(
            pin["library_sha256"].as_str().map(str::len),
            Some(64),
            "{host:?}"
        );
    }
}

#[test]
fn unsafe_member_names_are_rejected() {
    for name in [
        "../outside",
        "/outside",
        "bin/../../outside",
        "bin\\outside",
        "bin/file:stream",
        "bin/\0file",
        "",
        "bin//file",
        "bin/",
        "./bin",
    ] {
        assert_eq!(
            checker::member_path(name),
            Err(failure("invalid_archive_member_path")),
            "{name:?}"
        );
    }
    assert_eq!(
        checker::member_path("bin/bridge_mcp"),
        Ok(vec!["bin", "bridge_mcp"])
    );
}

#[test]
fn endpoint_settings_are_resolved_through_manifest_mappings() {
    let resolved = checker::resolve_environment(&committed_manifest()).unwrap();
    assert_eq!(resolved["BRIDGE_TALLY_HOST"], "127.0.0.1");
    assert_eq!(resolved["BRIDGE_TALLY_PORT"], "9");
    assert_eq!(resolved["BRIDGE_AGENT_ENABLE_WRITES"], "false");
    assert_eq!(resolved["BRIDGE_AGENT_ENABLE_IMPORT"], "true");
    for key in ["BRIDGE_TALLY_HOST", "BRIDGE_TALLY_PORT"] {
        let mut broken = committed_manifest();
        broken["server"]["mcp_config"]["env"][key] = json!("${user_config.redaction}");
        assert_eq!(
            checker::resolve_environment(&broken),
            Err(failure("endpoint_environment_mapping_mismatch")),
            "{key}"
        );
    }
}

#[test]
fn initialize_server_version_must_match_the_archived_manifest() {
    let manifest = json!({"version": "0.2.0"});
    let response = json!({"result": {"serverInfo": {"version": "0.2.0"}}});
    assert_eq!(
        checker::validate_server_version(&response, &manifest),
        Ok("0.2.0".to_string())
    );
    let other = json!({"result": {"serverInfo": {"version": "0.2.1"}}});
    assert_eq!(
        checker::validate_server_version(&other, &manifest),
        Err(failure("server_version_mismatch"))
    );
}

#[test]
fn posting_defaults_off_and_a_default_on_manifest_is_refused() {
    let mut manifest = committed_manifest();
    assert_eq!(
        manifest["user_config"]["enable_posting_2026_10"]["default"],
        json!(false)
    );
    manifest["user_config"]["enable_posting_2026_10"]["default"] = json!(true);
    assert_eq!(
        checker::resolve_environment(&manifest),
        Err(failure("posting_must_default_off"))
    );
}

fn with(environment: &Map<String, Value>, name: &str, value: &str) -> Map<String, Value> {
    let mut copy = environment.clone();
    copy.insert(name.to_string(), json!(value));
    copy
}

fn set(names: &[&str]) -> std::collections::BTreeSet<String> {
    names.iter().map(ToString::to_string).collect()
}

#[test]
fn the_default_bundle_prepares_and_parses_but_does_not_post() {
    let default = checker::resolve_environment(&committed_manifest()).unwrap();
    let tools = checker::expected_tools(&default);
    assert!(set(&["build_import_xml", "parse_bank_statement", "verify_import"]).is_subset(&tools));
    assert!(!tools.contains("post_import") && !tools.contains("acknowledge_post_review"));
    let enabled = checker::expected_tools(&with(&default, "BRIDGE_AGENT_ENABLE_WRITES", "true"));
    assert_eq!(
        enabled
            .difference(&tools)
            .cloned()
            .collect::<std::collections::BTreeSet<_>>(),
        set(&["post_import", "acknowledge_post_review"])
    );
}

#[test]
fn the_tax_audit_tool_is_off_by_default_and_its_switch_adds_only_it() {
    let default = checker::resolve_environment(&committed_manifest()).unwrap();
    assert_eq!(default["BRIDGE_AGENT_ENABLE_TAX_AUDIT"], "false");
    let tools = checker::expected_tools(&default);
    assert!(!tools.contains("audit_test_run"));
    let enabled = checker::expected_tools(&with(&default, "BRIDGE_AGENT_ENABLE_TAX_AUDIT", "true"));
    assert_eq!(
        enabled
            .difference(&tools)
            .cloned()
            .collect::<std::collections::BTreeSet<_>>(),
        set(&["audit_test_run"])
    );
}

#[test]
fn import_mapping_must_be_the_constant_true() {
    for mapping in ["false", "${user_config.enable_posting_2026_10}", "1"] {
        let mut manifest = committed_manifest();
        manifest["server"]["mcp_config"]["env"]["BRIDGE_AGENT_ENABLE_IMPORT"] = json!(mapping);
        assert_eq!(
            checker::resolve_environment(&manifest),
            Err(failure("import_environment_mapping_mismatch")),
            "{mapping}"
        );
    }
    let mut manifest = committed_manifest();
    manifest["server"]["mcp_config"]["env"]
        .as_object_mut()
        .unwrap()
        .remove("BRIDGE_AGENT_ENABLE_IMPORT");
    assert_eq!(
        checker::resolve_environment(&manifest),
        Err(failure("unexpected_environment_mapping"))
    );
}

#[test]
fn write_setting_requires_a_real_boolean_default() {
    for default in [
        json!("true"),
        json!("false"),
        json!(0),
        json!(1),
        Value::Null,
    ] {
        let mut manifest = committed_manifest();
        manifest["user_config"]["enable_posting_2026_10"]["default"] = default.clone();
        assert_eq!(
            checker::resolve_environment(&manifest),
            Err(failure("writes_default_must_be_boolean")),
            "{default}"
        );
    }
}

#[test]
fn write_mapping_cannot_bypass_user_setting() {
    for mapping in [
        "true",
        "false",
        "${user_config.redaction}",
        "${user_config.enable_writes}",
    ] {
        let mut manifest = committed_manifest();
        manifest["server"]["mcp_config"]["env"]["BRIDGE_AGENT_ENABLE_WRITES"] = json!(mapping);
        assert_eq!(
            checker::resolve_environment(&manifest),
            Err(failure("writes_environment_mapping_mismatch")),
            "{mapping}"
        );
    }
}

#[test]
fn terms_setting_defaults_to_not_accepted_and_is_mapped() {
    assert_eq!(
        checker::resolve_environment(&committed_manifest()).unwrap()["BRIDGE_TERMS_ACCEPTED"],
        "false"
    );
    for (field, value) in [
        ("default", json!(true)),
        ("default", Value::Null),
        ("type", json!("string")),
    ] {
        let mut broken = committed_manifest();
        broken["user_config"]["accept_terms_2026_10_1"][field] = value.clone();
        assert_eq!(
            checker::resolve_environment(&broken),
            Err(failure("terms_setting_must_be_boolean_and_off")),
            "{field} {value}"
        );
    }
    let mut broken = committed_manifest();
    broken["user_config"]
        .as_object_mut()
        .unwrap()
        .remove("accept_terms_2026_10_1");
    assert_eq!(
        checker::resolve_environment(&broken),
        Err(failure("terms_setting_must_be_boolean_and_off"))
    );
    // bridge#1413: a required setting with no stored value stops Claude Desktop from starting the
    // server, so no setting may be required, the Terms setting included.
    for name in ["accept_terms_2026_10_1", "port"] {
        let mut broken = committed_manifest();
        broken["user_config"][name]["required"] = json!(true);
        assert_eq!(
            checker::resolve_environment(&broken),
            Err(failure("required_setting_blocks_launch")),
            "{name}"
        );
    }
    let mut unrequired = committed_manifest();
    unrequired["user_config"]["accept_terms_2026_10_1"]["required"] = json!(false);
    assert_eq!(
        checker::resolve_environment(&unrequired).unwrap()["BRIDGE_TERMS_ACCEPTED"],
        "false"
    );
    for mapping in ["true", "${user_config.enable_posting_2026_10}"] {
        let mut broken = committed_manifest();
        broken["server"]["mcp_config"]["env"]["BRIDGE_TERMS_ACCEPTED"] = json!(mapping);
        assert_eq!(
            checker::resolve_environment(&broken),
            Err(failure("terms_environment_mapping_mismatch")),
            "{mapping}"
        );
    }
    let mut broken = committed_manifest();
    broken["server"]["mcp_config"]["env"]
        .as_object_mut()
        .unwrap()
        .remove("BRIDGE_TERMS_ACCEPTED");
    assert_eq!(
        checker::resolve_environment(&broken),
        Err(failure("unexpected_environment_mapping"))
    );
}

fn host_environment() -> Map<String, Value> {
    std::env::vars()
        .map(|(name, value)| (name, json!(value)))
        .collect()
}

fn shell(script_unix: &str, script_windows: &str) -> Vec<String> {
    if cfg!(windows) {
        ["powershell", "-NoProfile", "-Command", script_windows]
            .map(String::from)
            .to_vec()
    } else {
        ["sh", "-c", script_unix].map(String::from).to_vec()
    }
}

#[test]
fn server_output_is_bounded() {
    let command = shell(
        "head -c 1048576 /dev/zero",
        "[Console]::Out.Write('x' * 1048576)",
    );
    let result = checker::run_bounded(&command, b"", &host_environment(), Duration::from_secs(60));
    assert_eq!(result.map(|_| ()), Err(failure("server_output_limit")));
}

#[test]
fn server_timeout_is_enforced() {
    let command = shell("sleep 10", "Start-Sleep -Seconds 10");
    let started = std::time::Instant::now();
    let result = checker::run_bounded(
        &command,
        b"",
        &host_environment(),
        Duration::from_millis(100),
    );
    assert_eq!(result.map(|_| ()), Err(failure("server_timeout")));
    assert!(started.elapsed() < Duration::from_secs(9));
}

#[test]
fn a_failed_server_reports_its_exit_code() {
    let command = shell("exit 3", "exit 3");
    let result = checker::run_bounded(&command, b"", &host_environment(), Duration::from_secs(60));
    assert_eq!(result.map(|_| ()), Err(failure("server_exit_failed:3")));
}

#[test]
fn split_lines_matches_python_bytes_splitlines() {
    let text = b"a\nb\r\nc\rd";
    assert_eq!(
        checker::split_lines(text, false),
        vec![&b"a"[..], b"b", b"c", b"d"]
    );
    assert_eq!(
        checker::split_lines(text, true),
        vec![&b"a\n"[..], b"b\r\n", b"c\r", b"d"]
    );
    assert_eq!(checker::split_lines(b"a\n", false), vec![&b"a"[..]]);
    assert!(checker::split_lines(b"", false).is_empty());
}

#[test]
fn the_report_has_the_python_fields_in_the_python_order_and_layout() {
    let report = checker::Report {
        archive_sha256: "a".repeat(64),
        binary_sha256: "b".repeat(64),
        platform: "darwin".to_string(),
        archive_entries: 9,
        legal_resources: 5,
        pdfium_library_sha256: json!("c"),
        statement_vouchers: json!(6),
        server_version: "0.6.1".to_string(),
        response_ids: vec![json!(1), json!(2), json!(3)],
        response_bytes: 10,
        default_tool_count: 33,
        opt_in_tool_count: 35,
        opt_in_stderr_bytes: 0,
        egress_receipts: 2,
        stderr_bytes: 0,
        stderr_sha256: "d".to_string(),
    };
    // What Python's `json.dumps(result, indent=2)` prints for the same values.
    let python = format!(
        "{{\n  \"archive_sha256\": \"{a}\",\n  \"binary_sha256\": \"{b}\",\n  \"platform\": \"darwin\",\n  \"archive_entries\": 9,\n  \"legal_resources\": 5,\n  \"pdfium_library_sha256\": \"c\",\n  \"statement_vouchers\": 6,\n  \"server_version\": \"0.6.1\",\n  \"response_ids\": [\n    1,\n    2,\n    3\n  ],\n  \"response_bytes\": 10,\n  \"default_tool_count\": 33,\n  \"opt_in_tool_count\": 35,\n  \"opt_in_stderr_bytes\": 0,\n  \"egress_receipts\": 2,\n  \"stderr_bytes\": 0,\n  \"stderr_sha256\": \"d\"\n}}",
        a = "a".repeat(64),
        b = "b".repeat(64),
    );
    assert_eq!(checker::report_text(&report).unwrap(), python + "\n");
    assert_eq!(checker::ARCHIVE_MEMBERS, 9);
    // json.dumps escapes every character outside ASCII, a pair of escapes above the first plane.
    let report = checker::Report {
        server_version: "0.6.1-\u{e9}\u{1f600}".to_string(),
        ..report
    };
    assert!(checker::report_text(&report)
        .unwrap()
        .contains("\"server_version\": \"0.6.1-\\u00e9\\ud83d\\ude00\","));
}

#[test]
fn the_entry_point_must_be_a_string_under_bin() {
    for entry in [json!(7), json!("lib/bridge_mcp")] {
        let mut bundle = Bundle::valid();
        bundle.manifest["server"]["entry_point"] = entry.clone();
        let (result, _root) = unpack(&bundle, LIMITS);
        assert_eq!(result, Err(failure("invalid_entry_point")), "{entry}");
    }
}

#[test]
fn the_launch_command_platform_and_legal_files_must_match() {
    if !host_bundles_pdfium() {
        return;
    }
    let other_command = |bundle: &mut Bundle| {
        bundle.manifest["server"]["mcp_config"]["command"] = json!("${__dirname}/bin/other");
    };
    let arguments =
        |bundle: &mut Bundle| bundle.manifest["server"]["mcp_config"]["args"] = json!(["--x"]);
    let overrides = |bundle: &mut Bundle| {
        bundle.manifest["server"]["mcp_config"]["platform_overrides"] = json!({"win32": {}});
    };
    let not_binary = |bundle: &mut Bundle| bundle.manifest["server"]["type"] = json!("node");
    let platform =
        |bundle: &mut Bundle| bundle.manifest["compatibility"]["platforms"] = json!(["linux"]);
    let licence = |bundle: &mut Bundle| bundle.entry("LICENSE").bytes = b"another licence".to_vec();
    let cases: [(&Change, &str); 6] = [
        (&other_command, "launch_command_does_not_match_binary"),
        (&arguments, "launch_command_does_not_match_binary"),
        (&overrides, "launch_command_does_not_match_binary"),
        (&not_binary, "launch_command_does_not_match_binary"),
        (&platform, "archive_platform_mismatch"),
        (&licence, "legal_resource_bytes_differ"),
    ];
    for (change, code) in cases {
        let mut bundle = Bundle::valid();
        change(&mut bundle);
        let (result, _root) = unpack(&bundle, LIMITS);
        assert_eq!(result, Err(failure(code)), "{code}");
    }
    // Empty arguments and overrides are allowed, as Python's `not config.get(...)`.
    let mut bundle = Bundle::valid();
    bundle.manifest["server"]["mcp_config"]["args"] = json!([]);
    bundle.manifest["server"]["mcp_config"]["platform_overrides"] = json!({});
    let (result, _root) = unpack(&bundle, LIMITS);
    assert!(
        !matches!(result, Err(Failure(ref code)) if code == "launch_command_does_not_match_binary")
    );
}

#[test]
fn the_tax_audit_setting_and_every_mapping_must_resolve() {
    for (field, value) in [("default", json!(true)), ("type", json!("string"))] {
        let mut broken = committed_manifest();
        broken["user_config"]["enable_tax_audit_2026_10"][field] = value.clone();
        assert_eq!(
            checker::resolve_environment(&broken),
            Err(failure("tax_audit_setting_must_be_boolean_and_off")),
            "{field} {value}"
        );
    }
    let mut broken = committed_manifest();
    broken["server"]["mcp_config"]["env"]["BRIDGE_AGENT_ENABLE_TAX_AUDIT"] = json!("true");
    assert_eq!(
        checker::resolve_environment(&broken),
        Err(failure("tax_audit_environment_mapping_mismatch"))
    );
    let mut broken = committed_manifest();
    broken["server"]["mcp_config"]["env"]["BRIDGE_AGENT_REDACTION"] =
        json!("${user_config.missing}");
    assert_eq!(
        checker::resolve_environment(&broken),
        Err(failure("unresolved_environment_mapping"))
    );
}

#[test]
fn a_schema_receipt_must_be_prepared_for_this_response() {
    let response = b"{}\n";
    let digest = hex_sha256(response);
    let prepared = json!({"record_type": "response_prepared", "tool": "voucher_schema", "receipt_id": "r",
        "bytes_prepared": response.len(), "response_sha256": digest});
    let completed = json!({"record_type": "stdio_write_completed", "receipt_id": "r",
        "bytes_written": response.len(), "response_sha256": digest});
    for (key, value) in [
        ("tool", json!("vouchers")),
        ("receipt_id", json!("")),
        ("bytes_prepared", json!(1)),
    ] {
        let mut changed = prepared.clone();
        changed[key] = value;
        let raw = [
            serde_json::to_vec(&changed).unwrap(),
            b"\n".to_vec(),
            serde_json::to_vec(&completed).unwrap(),
        ]
        .concat();
        assert_eq!(
            checker::validate_schema_receipts(&raw, response),
            Err(failure("schema_egress_preparation_invalid")),
            "{key}"
        );
    }
}

#[cfg(unix)]
#[test]
fn a_server_that_closes_its_input_early_is_a_broken_pipe() {
    // More than a pipe holds, so the write is still waiting when the server closes its input.
    let command = shell("exec 0<&-; sleep 1", "");
    let result = checker::run_bounded(
        &command,
        &vec![b'x'; 1024 * 1024],
        &host_environment(),
        Duration::from_secs(60),
    );
    assert_eq!(result.map(|_| ()), Err(failure("BrokenPipeError")));
}

/// Scratch-folder removal: the folder holds the bundle's server binary, which Windows can keep locked for
/// a moment after the process has exited.
fn locked(
    failures: Option<usize>,
    kind: std::io::ErrorKind,
) -> (Vec<usize>, Option<String>, usize) {
    let mut calls = Vec::new();
    let mut pauses = 0;
    let message = checker::remove_with_retries(
        Path::new("scratch"),
        |_| {
            calls.push(calls.len());
            if failures.is_none_or(|count| calls.len() <= count) {
                Err(std::io::Error::from(kind))
            } else {
                Ok(())
            }
        },
        |pause| {
            assert_eq!(pause, checker::REMOVE_PAUSE);
            pauses += 1;
        },
    );
    (calls, message, pauses)
}

#[test]
fn a_folder_that_cannot_be_removed_at_first_is_removed_once_the_lock_clears() {
    let (calls, message, pauses) = locked(Some(2), std::io::ErrorKind::PermissionDenied);
    assert_eq!((calls.len(), message, pauses), (3, None, 2));
    let (calls, message, _) = locked(Some(1), std::io::ErrorKind::NotADirectory);
    assert_eq!((calls.len(), message), (2, None));
}

#[test]
fn a_folder_that_stays_locked_is_left_with_a_bounded_wait_and_a_message() {
    let (calls, message, pauses) = locked(None, std::io::ErrorKind::PermissionDenied);
    assert_eq!(calls.len(), checker::REMOVE_ATTEMPTS as usize);
    assert!(
        (3..=12).contains(&calls.len()),
        "retries, but not without end"
    );
    assert!(checker::REMOVE_PAUSE * checker::REMOVE_ATTEMPTS <= Duration::from_secs(10));
    assert_eq!(
        pauses,
        calls.len() - 1,
        "a pause between attempts, none after the last"
    );
    let message = message.expect("a message");
    assert!(
        message
            .starts_with("MCPB smoke could not remove its scratch folder scratch after 8 attempts"),
        "{message}"
    );
    assert!(message.contains("PermissionDenied"), "{message}");
}

#[test]
fn a_folder_that_is_already_gone_is_neither_retried_nor_reported() {
    let (calls, message, pauses) = locked(None, std::io::ErrorKind::NotFound);
    assert_eq!((calls.len(), message, pauses), (1, None, 0));
}

#[test]
fn the_scratch_folder_is_removed_when_the_run_ends() {
    let path = {
        let scratch = checker::ScratchDirectory::new("bridge-mcpb-test-").unwrap();
        fs::write(scratch.0.join("server.exe"), b"x").unwrap();
        scratch.0.clone()
    };
    assert!(!path.exists());
}

/// What the stand-in server gets wrong: one fault a test, each a check inside the smoke's server runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Fault {
    None,
    RefusedRunServesTheCall,
    RefusedRunHidesTheTools,
    RefusedRunRecordsAcceptance,
    OldProtocol,
    /// The last reply of the start with this index (0 refused, 1 accepted, 2 posting opt-in, 3 tax-audit
    /// opt-in, 4 statement) is never written.
    DropsLastReply(usize),
    NotJsonRpc2,
    RefusalNotMarkedAnError,
    AcceptanceFromAnotherSource,
    SchemaReplyIsAnError,
    SchemaIsNotAnObject,
    PasswordInTheReply,
    PostingListedByDefault,
    OptInMissesAPostingTool,
    AuditSwitchAddsAnotherTool,
    SchemaTextDiffers,
    NoCompletionReceipt,
    AcceptanceRecordedEachRun,
    StatementVouchersShort,
    StatementRefused,
    PasswordEchoed,
}

const STAND_IN_VERSION: &str = "9.9.9-stand-in";

/// One start of the server as the smoke made it.
struct Start {
    command: Vec<String>,
    environment: Map<String, Value>,
    timeout: Duration,
}

/// An in-process stand-in for the bundle's server: it answers each request line as Bridge's server does,
/// from the environment it is started with, and writes the egress receipts and the Terms record into the
/// data folder that environment names.
struct StandIn {
    fault: Fault,
    starts: Vec<Start>,
}

impl StandIn {
    fn serve(
        &mut self,
        command: &[String],
        payload: &[u8],
        environment: &Map<String, Value>,
        timeout: Duration,
    ) -> Result<(Vec<u8>, Vec<u8>), Failure> {
        let start = self.starts.len();
        self.starts.push(Start {
            command: command.to_vec(),
            environment: environment.clone(),
            timeout,
        });
        let fault = self.fault;
        let flag = |name: &str| environment.get(name).and_then(Value::as_str) == Some("true");
        let accepted = flag("BRIDGE_TERMS_ACCEPTED");
        let writes = flag("BRIDGE_AGENT_ENABLE_WRITES");
        let audit = flag("BRIDGE_AGENT_ENABLE_TAX_AUDIT");
        let data = PathBuf::from(environment["BRIDGE_AGENT_DATA_DIR"].as_str().unwrap());
        fs::create_dir_all(&data).unwrap();
        let acceptance = data.join("terms-acceptance.jsonl");
        let record_acceptance = (accepted
            && (!acceptance.exists() || fault == Fault::AcceptanceRecordedEachRun))
            || (!accepted && fault == Fault::RefusedRunRecordsAcceptance);
        if record_acceptance {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&acceptance)
                .unwrap();
            let source = if fault == Fault::AcceptanceFromAnotherSource {
                "prompt"
            } else {
                "setting"
            };
            file.write_all(format!("{{\"source\":\"{source}\"}}\n").as_bytes())
                .unwrap();
        }
        let (mut output, mut diagnostics) = (Vec::new(), Vec::new());
        let mut last_reply = 0;
        for line in payload
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            let request: Value = serde_json::from_slice(line).unwrap();
            let Some(id) = request.get("id").cloned() else {
                continue;
            };
            let method = request["method"].as_str().unwrap();
            let tool = request["params"]["name"].as_str().unwrap_or_default();
            let refused =
                method == "tools/call" && !accepted && fault != Fault::RefusedRunServesTheCall;
            let result = match method {
                "initialize" => json!({
                    "protocolVersion": if fault == Fault::OldProtocol { "2024-11-05" } else { "2025-06-18" },
                    "serverInfo": {"name": "bridge-mcp", "version": STAND_IN_VERSION},
                }),
                "tools/list" => {
                    let mut names = checker::expected_tools(environment);
                    let extra = checker::POSTING_TOOLS[0].to_string();
                    match fault {
                        Fault::RefusedRunHidesTheTools if !accepted => names.clear(),
                        Fault::PostingListedByDefault if !writes => {
                            names.insert(extra);
                        }
                        Fault::OptInMissesAPostingTool if writes => {
                            names.remove(&extra);
                        }
                        Fault::AuditSwitchAddsAnotherTool if audit => {
                            names.insert(extra);
                        }
                        _ => {}
                    }
                    let tools: Vec<Value> =
                        names.iter().map(|name| json!({"name": name})).collect();
                    json!({ "tools": tools })
                }
                "tools/call" if refused => json!({
                    "isError": fault != Fault::RefusalNotMarkedAnError,
                    "structuredContent": {"result": {"error": {"code": "terms_not_accepted"}}},
                    "content": [{"type": "text", "text": "the Terms of Use are not accepted"}],
                }),
                "tools/call" if tool == "voucher_schema" => {
                    let kind = if fault == Fault::SchemaIsNotAnObject {
                        "array"
                    } else {
                        "object"
                    };
                    let structured = json!({"result": {"schema": {"type": kind}}});
                    let shown = if fault == Fault::SchemaTextDiffers {
                        json!({"result": {"schema": {"type": "array"}}})
                    } else {
                        structured.clone()
                    };
                    json!({
                        "isError": fault == Fault::SchemaReplyIsAnError,
                        "structuredContent": structured,
                        "content": [{"type": "text", "text": shown.to_string()}],
                    })
                }
                "tools/call" if tool == "parse_bank_statement" => {
                    let password_file = request["params"]["arguments"]["password_file"]
                        .as_str()
                        .unwrap();
                    assert_eq!(
                        fs::read_to_string(password_file).unwrap(),
                        format!("{}\n", checker::STATEMENT_PASSWORD)
                    );
                    assert!(Path::new(
                        request["params"]["arguments"]["statement_path"]
                            .as_str()
                            .unwrap()
                    )
                    .is_file());
                    if fault == Fault::PasswordEchoed {
                        diagnostics.extend_from_slice(checker::STATEMENT_PASSWORD.as_bytes());
                    }
                    if fault == Fault::StatementRefused {
                        json!({"isError": true, "structuredContent": {"result": {"error": {"code": "pdfium_unavailable"}}}})
                    } else {
                        let vouchers = if fault == Fault::StatementVouchersShort {
                            5
                        } else {
                            6
                        };
                        let mut parsed = json!({
                            "isError": false,
                            "structuredContent": {"result": {"statement_rows": 6, "vouchers": vouchers, "proposals_id": "stand-in"}},
                        });
                        if fault == Fault::PasswordInTheReply {
                            parsed["content"] =
                                json!([{"type": "text", "text": checker::STATEMENT_PASSWORD}]);
                        }
                        parsed
                    }
                }
                other => {
                    panic!("the smoke sent a request the stand-in does not answer: {other} {tool}")
                }
            };
            let version = if fault == Fault::NotJsonRpc2 && start == 1 {
                "1.0"
            } else {
                "2.0"
            };
            let mut reply =
                serde_json::to_vec(&json!({"jsonrpc": version, "id": id, "result": result}))
                    .unwrap();
            reply.push(b'\n');
            if tool == "voucher_schema" && !refused {
                let digest = hex_sha256(&reply);
                let mut receipts = vec![json!({
                    "record_type": "response_prepared", "tool": "voucher_schema", "receipt_id": "r-1",
                    "bytes_prepared": reply.len(), "response_sha256": digest,
                })];
                if fault != Fault::NoCompletionReceipt {
                    receipts.push(json!({
                        "record_type": "stdio_write_completed", "receipt_id": "r-1",
                        "bytes_written": reply.len(), "response_sha256": digest,
                    }));
                }
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(data.join("agent-egress.jsonl"))
                    .unwrap();
                for receipt in receipts {
                    file.write_all(format!("{receipt}\n").as_bytes()).unwrap();
                }
            }
            last_reply = output.len();
            output.extend(reply);
        }
        if fault == Fault::DropsLastReply(start) {
            output.truncate(last_reply);
        }
        Ok((output, diagnostics))
    }
}

/// A bundle that passes `unpack_bundle` and carries the committed manifest's settings, mappings and a
/// version, so the smoke reaches its server runs; its repository holds the synthetic statement.
fn serving_bundle(root: &Path) -> (PathBuf, PathBuf) {
    let mut bundle = Bundle::valid();
    let committed = committed_manifest();
    bundle.manifest["version"] = json!(STAND_IN_VERSION);
    bundle.manifest["user_config"] = committed["user_config"].clone();
    bundle.manifest["server"]["mcp_config"]["env"] =
        committed["server"]["mcp_config"]["env"].clone();
    let (archive, repository) = bundle.write(root);
    let statement = repository.join(checker::STATEMENT_FIXTURE);
    fs::create_dir_all(statement.parent().unwrap()).unwrap();
    fs::copy(
        self::repository().join(checker::STATEMENT_FIXTURE),
        statement,
    )
    .unwrap();
    (archive, repository)
}

fn smoke_against(fault: Fault) -> (Result<checker::Report, Failure>, StandIn) {
    let root = tempfile::tempdir().unwrap();
    let (archive, repository) = serving_bundle(root.path());
    let mut stand_in = StandIn {
        fault,
        starts: Vec::new(),
    };
    let result = checker::smoke_with(
        &archive,
        &repository,
        &mut |command, payload, environment, timeout| {
            stand_in.serve(command, payload, environment, timeout)
        },
    );
    (result, stand_in)
}

fn smoke_failure(fault: Fault) -> Failure {
    match smoke_against(fault).0 {
        Ok(_) => panic!("{fault:?}: the smoke passed"),
        Err(failure) => failure,
    }
}

#[test]
fn a_server_that_answers_as_bridge_passes_the_smoke_with_each_run_set_up_as_released() {
    if !host_bundles_pdfium() {
        return;
    }
    let (result, stand_in) = smoke_against(Fault::None);
    let report = match result {
        Ok(report) => report,
        Err(failure) => panic!("the stand-in failed the smoke: {failure:?}"),
    };
    assert_eq!(report.server_version, STAND_IN_VERSION);
    assert_eq!(report.response_ids, [json!(1), json!(2), json!(3)]);
    assert_eq!(report.statement_vouchers, json!(6));
    assert_eq!(report.egress_receipts, 2);
    let starts = &stand_in.starts;
    assert_eq!(
        starts.len(),
        5,
        "refused, accepted, posting opt-in, tax-audit opt-in, statement"
    );
    let defaults = checker::expected_tools(&starts[1].environment);
    assert_eq!(report.default_tool_count, defaults.len());
    assert_eq!(
        report.opt_in_tool_count,
        defaults.len() + checker::POSTING_TOOLS.len()
    );
    let binary = Path::new(&starts[0].command[0]);
    assert!(
        binary.ends_with(Path::new("bundle").join(ENTRY)),
        "{}",
        binary.display()
    );
    assert!(
        !binary.exists(),
        "the scratch folder is removed once the run ends"
    );
    let value = |start: &Start, name: &str| {
        start
            .environment
            .get(name)
            .and_then(Value::as_str)
            .map(str::to_string)
    };
    let data = |start: &Start| PathBuf::from(value(start, "BRIDGE_AGENT_DATA_DIR").unwrap());
    for (index, start) in starts.iter().enumerate() {
        assert_eq!(start.command, starts[0].command, "start {index}");
        assert_eq!(
            value(start, "BRIDGE_TALLY_HOST").as_deref(),
            Some("127.0.0.1"),
            "start {index}"
        );
        assert_eq!(
            value(start, "BRIDGE_TALLY_PORT").as_deref(),
            Some("9"),
            "start {index}"
        );
        assert_eq!(
            value(start, "BRIDGE_AGENT_ENABLE_IMPORT").as_deref(),
            Some("true"),
            "start {index}"
        );
        let terms = if index == 0 { "false" } else { "true" };
        assert_eq!(
            value(start, "BRIDGE_TERMS_ACCEPTED").as_deref(),
            Some(terms),
            "start {index}"
        );
        let writes = if index == 2 { "true" } else { "false" };
        assert_eq!(
            value(start, "BRIDGE_AGENT_ENABLE_WRITES").as_deref(),
            Some(writes),
            "start {index}"
        );
        let audit = if index == 3 { "true" } else { "false" };
        assert_eq!(
            value(start, "BRIDGE_AGENT_ENABLE_TAX_AUDIT").as_deref(),
            Some(audit),
            "start {index}"
        );
        let timeout = if index == 4 { 60 } else { 15 };
        assert_eq!(start.timeout, Duration::from_secs(timeout), "start {index}");
    }
    assert!(data(&starts[0]).ends_with("data-refused"));
    assert!(data(&starts[1]).ends_with("data"));
    assert_eq!(data(&starts[2]), data(&starts[1]));
    assert_eq!(data(&starts[3]), data(&starts[1]));
    assert!(data(&starts[4]).ends_with(Path::new("statement").join("data")));
}

#[test]
fn the_terms_refusal_is_checked_on_the_run_with_the_default_settings() {
    if !host_bundles_pdfium() {
        return;
    }
    for (fault, code) in [
        (Fault::DropsLastReply(0), "terms_refusal_response_ids"),
        (Fault::RefusedRunServesTheCall, "terms_refusal_missing"),
        (Fault::RefusalNotMarkedAnError, "terms_refusal_missing"),
        (
            Fault::RefusedRunHidesTheTools,
            "terms_refusal_hid_the_tool_list",
        ),
        (
            Fault::RefusedRunRecordsAcceptance,
            "terms_recorded_without_acceptance",
        ),
        (
            Fault::AcceptanceRecordedEachRun,
            "terms_acceptance_not_recorded",
        ),
        (
            Fault::AcceptanceFromAnotherSource,
            "terms_acceptance_not_recorded",
        ),
    ] {
        assert_eq!(smoke_failure(fault), failure(code), "{fault:?}");
    }
}

#[test]
fn the_tool_lists_must_match_each_setting() {
    if !host_bundles_pdfium() {
        return;
    }
    for (fault, code) in [
        (Fault::OldProtocol, "protocol_mismatch"),
        (Fault::DropsLastReply(1), "unexpected_response_ids"),
        (Fault::NotJsonRpc2, "invalid_jsonrpc_response"),
        (Fault::DropsLastReply(2), "opt_in_response_ids"),
        (Fault::DropsLastReply(3), "audit_opt_in_response_ids"),
        (Fault::PostingListedByDefault, "default_tools_mismatch"),
        (Fault::OptInMissesAPostingTool, "opt_in_tools_mismatch"),
        (
            Fault::AuditSwitchAddsAnotherTool,
            "audit_opt_in_tools_mismatch",
        ),
    ] {
        assert_eq!(smoke_failure(fault), failure(code), "{fault:?}");
    }
}

#[test]
fn the_schema_reply_must_agree_with_itself_and_its_receipts() {
    if !host_bundles_pdfium() {
        return;
    }
    for (fault, code) in [
        (Fault::SchemaTextDiffers, "voucher_schema_response_invalid"),
        (
            Fault::SchemaReplyIsAnError,
            "voucher_schema_response_invalid",
        ),
        (
            Fault::SchemaIsNotAnObject,
            "voucher_schema_response_invalid",
        ),
        (
            Fault::NoCompletionReceipt,
            "schema_egress_completion_missing",
        ),
    ] {
        assert_eq!(smoke_failure(fault), failure(code), "{fault:?}");
    }
}

#[test]
fn the_statement_parse_must_build_its_vouchers_without_echoing_the_password() {
    if !host_bundles_pdfium() {
        return;
    }
    for (fault, code) in [
        (
            Fault::StatementVouchersShort,
            "statement_parse_failed:no_result",
        ),
        (
            Fault::StatementRefused,
            "statement_parse_failed:pdfium_unavailable",
        ),
        (Fault::PasswordEchoed, "statement_password_echoed"),
        (Fault::PasswordInTheReply, "statement_password_echoed"),
        (Fault::DropsLastReply(4), "statement_response_ids"),
    ] {
        assert_eq!(smoke_failure(fault), failure(code), "{fault:?}");
    }
}

/// NOTICE is copied into every archive, and the smoke compares the archive's copy to it, so this is what a
/// user of the package reads about the icon: not Apache-2.0, all rights reserved.
#[test]
fn the_shipped_notice_reserves_the_rights_in_the_icon() {
    let notice = fs::read_to_string(repository().join("NOTICE")).unwrap();
    let text = notice.split_whitespace().collect::<Vec<_>>().join(" ");
    for phrase in [
        "packaging/mcpb/icon.png",
        "are not licensed under Apache-2.0: all rights in them are reserved",
        "A fork or modified build must replace them",
    ] {
        assert!(text.contains(phrase), "{phrase}");
    }
}

/// A file held open without delete sharing locks its folder's removal on Windows: the removal waits, once,
/// for the lock to clear, against the real operating system.
#[cfg(windows)]
#[test]
fn a_file_held_open_is_waited_for_against_the_real_operating_system() {
    use std::os::windows::fs::OpenOptionsExt;
    let root = tempfile::tempdir().unwrap();
    let folder = root.path().join("scratch");
    fs::create_dir(&folder).unwrap();
    let mut held = Some(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .share_mode(0)
            .open(folder.join("server.exe"))
            .unwrap(),
    );
    let mut pauses = 0;
    let message = checker::remove_with_retries(
        &folder,
        |path| fs::remove_dir_all(path),
        |_| {
            pauses += 1;
            drop(held.take());
        },
    );
    assert_eq!(message, None);
    assert_eq!(pauses, 1);
    assert!(!folder.exists());
}
