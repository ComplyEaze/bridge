// SPDX-License-Identifier: Apache-2.0
//! Verify and launch a trusted, locally built MCPB archive without contacting Tally.
//!
//! The Rust port of `scripts/check-mcpb-bundle.py` (#1810 step 6). Every check is the same, in the same
//! order, with the same diagnostic codes, and the report has the same fields in the same order, so CI can
//! run both on one bundle and compare the two reports byte for byte before the Python is retired. It reads
//! the archive with the `zip` crate the `bridge` package already depends on.
//!
//! Usage: `check_mcpb_bundle <archive> [--output <file>]`. The report goes to stdout (and the file); a
//! failure prints `MCPB smoke failed: <code>` on stderr and exits 1.

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

pub const RESOURCES: [&str; 4] = [
    "LICENSE",
    "NOTICE",
    "THIRD_PARTY_LICENSES.txt",
    "THIRD_PARTY_LICENSES_RUST.txt",
];
// One name per line, sorted: two pull requests that add different tools then touch different lines.
pub const DEFAULT_TOOLS: &[&str] = &[
    "balance_sheet",
    "cash_flow",
    "company_features",
    "egress_log",
    "gst_by_counter_ledger",
    "gst_status",
    "gst_uncertain_vouchers",
    "ledger_masters",
    "ledger_movement",
    "list_companies",
    "local_data_report",
    "masters",
    "month_register",
    "negative_ledgers",
    "negative_stock",
    "outstandings",
    "profit_and_loss",
    "purchase_register",
    "read_evidence",
    "sales_register",
    "statement_tie_out",
    "stock_summary",
    "tally_status",
    "trial_balance",
    "validate_masters",
    "verify_import",
    "voucher_presence",
    "voucher_read_cost",
    "voucher_schema",
    "vouchers",
];
// The bundle always enables file preparation and bank-statement parsing; they write nothing to Tally.
// Posting, and recording a person's review of a doubted post (bridge#239), are the tools behind the
// user's switch, and it is off by default while the limits recorded on bridge#574 and bridge#579 remain.
pub const IMPORT_TOOLS: &[&str] = &[
    "build_import_xml",
    "parse_bank_statement",
    "parse_bank_statement_csv",
];
#[rustfmt::skip]
pub const POSTING_TOOLS: &[&str] = &[
    "acknowledge_post_review",
    "post_import",
];
// The tax-audit tool is behind its own switch, off by default.
pub const AUDIT_TOOLS: &[&str] = &["audit_test_run"];

// The bundled PDFium: beside the binary, where parse_bank_statement loads it, with the licence notice
// scripts/fetch-pdfium.mjs generated at the root. Both are checked against
// packaging/pdfium/pdfium.lock.json, not merely present.
pub const PDFIUM_NOTICE: &str = "THIRD_PARTY_LICENSES_PDFIUM.txt";
pub const ICON: &str = "icon.png";
pub const ICON_SOURCE: &str = "packaging/mcpb/icon.png";
pub const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
pub const ARCHIVE_MEMBERS: usize = RESOURCES.len() + 5; // manifest, icon, binary, PDFium library, PDFium notice
const STATEMENT_FIXTURE: &str =
    "src-tauri/crates/bridge-bank-statement/tests/fixtures/hdfc-synthetic.pdf";
const STATEMENT_PASSWORD: &str = "synthetic-user-4321";
pub const MAX_BUNDLE_BYTES: u64 = 128 * 1024 * 1024;
pub const MAX_OUTPUT_BYTES: usize = 512 * 1024;
pub const REMOVE_ATTEMPTS: u32 = 8;
pub const REMOVE_PAUSE: Duration = Duration::from_millis(500);

/// A failure: a fixed diagnostic code (never a tool payload), or the kind of error Python's checker
/// would have named, so both print comparable lines.
#[derive(Debug, PartialEq, Eq)]
pub struct Failure(pub String);

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub type Checked<T> = Result<T, Failure>;

pub fn require(condition: bool, code: &str) -> Checked<()> {
    if condition {
        Ok(())
    } else {
        Err(Failure(code.to_string()))
    }
}

fn os_error(error: std::io::Error) -> Failure {
    Failure(
        match error.kind() {
            std::io::ErrorKind::NotFound => "FileNotFoundError",
            std::io::ErrorKind::PermissionDenied => "PermissionError",
            std::io::ErrorKind::BrokenPipe => "BrokenPipeError",
            _ => "OSError",
        }
        .to_string(),
    )
}

fn zip_error(_: zip::result::ZipError) -> Failure {
    Failure("BadZipFile".to_string())
}

fn json_error(_: serde_json::Error) -> Failure {
    Failure("JSONDecodeError".to_string())
}

/// `value[key]`, a KeyError when absent or when `value` is not an object, as Python's indexing.
pub fn key<'a>(value: &'a Value, name: &str) -> Checked<&'a Value> {
    value
        .get(name)
        .ok_or_else(|| Failure("KeyError".to_string()))
}

/// `value.get(key)` on a dict: absent is `None`.
fn opt<'a>(value: &'a Value, name: &str) -> Option<&'a Value> {
    value.as_object().and_then(|object| object.get(name))
}

fn text(value: &Value) -> Checked<&str> {
    value
        .as_str()
        .ok_or_else(|| Failure("TypeError".to_string()))
}

/// Python's `==` between a JSON number and an int.
fn number_is(value: Option<&Value>, expected: i64) -> bool {
    match value {
        Some(Value::Number(number)) => number.as_f64() == Some(expected as f64),
        _ => false,
    }
}

/// `sys.platform` of the host this runs on.
pub fn host_platform() -> &'static str {
    match std::env::consts::OS {
        "macos" => "darwin",
        "windows" => "win32",
        other => other,
    }
}

/// `platform.machine().lower()` as Python reports it on the hosts that bundle PDFium.
fn host_machine() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => "amd64",
        ("macos", "aarch64") => "arm64",
        (_, "aarch64") => "aarch64",
        (_, arch) => arch,
    }
}

pub fn pdfium_library(platform: &str) -> &'static str {
    match platform {
        "darwin" => "libpdfium.dylib",
        "win32" => "pdfium.dll",
        _ => "",
    }
}

fn pdfium_platform(platform: &str, machine: &str) -> Option<&'static str> {
    match (platform, machine) {
        ("darwin", "arm64") => Some("macos-arm64"),
        ("win32", "amd64") => Some("windows-x64"),
        _ => None,
    }
}

/// A member name that is safe to extract: relative, POSIX, no `..` part, no backslash, colon or NUL. The
/// parts are those of Python's `PurePosixPath`, which drops empty and `.` parts, and the name must be the
/// path's own spelling, so a doubled, leading or trailing slash or a `.` part is refused too.
pub fn member_path(name: &str) -> Checked<Vec<&str>> {
    let parts: Vec<&str> = name
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect();
    let spelling = if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    };
    require(
        !name.is_empty()
            && !name.starts_with('/')
            && spelling == name
            && !parts.contains(&"..")
            && !name.contains(['\\', ':', '\0']),
        "invalid_archive_member_path",
    )?;
    Ok(parts)
}

/// Python's `ZipInfo.filename`: the stored name cut at a NUL, with the host's separator made `/`.
fn stored_filename(raw: &str) -> String {
    let name = raw.split('\0').next().unwrap_or_default();
    if cfg!(windows) {
        name.replace('\\', "/")
    } else {
        name.to_string()
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The archive's size limits; tests lower them to build small over-cap archives.
#[derive(Clone, Copy)]
pub struct Limits {
    pub archive_bytes: u64,
    pub unpacked_bytes: u64,
}

pub const LIMITS: Limits = Limits {
    archive_bytes: MAX_BUNDLE_BYTES,
    unpacked_bytes: MAX_BUNDLE_BYTES,
};

struct Member {
    original: String,
    name: String,
    size: u64,
    mode: u32,
    is_dir: bool,
}

fn read_member(bundle: &mut zip::ZipArchive<fs::File>, name: &str) -> Checked<Vec<u8>> {
    let mut file = bundle
        .by_name(name)
        .map_err(|_| Failure("KeyError".to_string()))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|_| Failure("BadZipFile".to_string()))?;
    Ok(bytes)
}

pub fn pdfium_pin(repository: &Path, host: Option<(&str, &str)>) -> Checked<Value> {
    let (platform, machine) = host.unwrap_or((host_platform(), host_machine()));
    let raw = fs::read(repository.join("packaging/pdfium/pdfium.lock.json")).map_err(os_error)?;
    let lock: Value = serde_json::from_slice(&raw).map_err(json_error)?;
    let platforms = key(&lock, "platforms")?;
    let name = pdfium_platform(platform, machine);
    require(
        name.is_some_and(|name| platforms.get(name).is_some()),
        "pdfium_platform_not_pinned",
    )?;
    Ok(platforms[name.unwrap_or_default()].clone())
}

/// The raw names of the central directory's file headers, in order, read from `start` to the first
/// record that is not one.
fn central_directory_names(archive: &Path, start: u64) -> Checked<Vec<Vec<u8>>> {
    let bytes = fs::read(archive).map_err(os_error)?;
    let truncated = || Failure("BadZipFile".to_string());
    let mut at = usize::try_from(start).map_err(|_| truncated())?;
    let mut names = Vec::new();
    while bytes.get(at..at + 4) == Some(b"PK\x01\x02".as_slice()) {
        let header = bytes.get(at..at + 46).ok_or_else(truncated)?;
        let field =
            |offset: usize| usize::from(u16::from_le_bytes([header[offset], header[offset + 1]]));
        let (name, extra, comment) = (field(28), field(30), field(32));
        names.push(
            bytes
                .get(at + 46..at + 46 + name)
                .ok_or_else(truncated)?
                .to_vec(),
        );
        at += 46 + name + extra + comment;
    }
    Ok(names)
}

/// Check the archive and extract it into `destination`; returns the manifest and the binary's path.
pub fn unpack_bundle(
    archive: &Path,
    destination: &Path,
    repository: &Path,
    limits: Limits,
) -> Checked<(Value, PathBuf)> {
    let size = fs::metadata(archive).map_err(os_error)?.len();
    require(size <= limits.archive_bytes, "archive_too_large")?;
    let mut bundle =
        zip::ZipArchive::new(fs::File::open(archive).map_err(os_error)?).map_err(zip_error)?;
    // The crate keeps one entry per name, so a duplicate would vanish from its count: the members are
    // counted, and their names read, from the central directory's own records, as Python's infolist().
    let recorded = central_directory_names(archive, bundle.central_directory_start())?;
    require(
        recorded.len() == ARCHIVE_MEMBERS,
        "unexpected_archive_members",
    )?;
    let recorded: Vec<String> = recorded
        .iter()
        .map(|raw| stored_filename(&String::from_utf8_lossy(raw)))
        .collect();
    require(
        recorded.iter().collect::<HashSet<_>>().len() == recorded.len(),
        "duplicate_archive_member",
    )?;
    require(bundle.len() == recorded.len(), "BadZipFile")?;
    let mut members = Vec::with_capacity(bundle.len());
    for index in 0..bundle.len() {
        let file = bundle.by_index_raw(index).map_err(zip_error)?;
        let original = file.name().map_err(zip_error)?.into_owned();
        let name = stored_filename(&original);
        members.push(Member {
            is_dir: name.ends_with('/'),
            original,
            name,
            size: file.size(),
            mode: file.external_attributes() >> 16,
        });
    }
    let names: Vec<&str> = members.iter().map(|member| member.name.as_str()).collect();
    require(
        members.iter().map(|member| member.size).sum::<u64>() <= limits.unpacked_bytes,
        "unpacked_archive_too_large",
    )?;
    for member in &members {
        member_path(&member.original)?;
        let kind = member.mode & 0o170000;
        require(
            !member.is_dir && (kind == 0 || kind == 0o100000),
            "archive_member_is_not_regular_file",
        )?;
    }
    let manifest: Value =
        serde_json::from_slice(&read_member(&mut bundle, "manifest.json")?).map_err(json_error)?;
    let server = key(&manifest, "server")?;
    let entry_value = key(server, "entry_point")?;
    require(entry_value.is_string(), "invalid_entry_point")?;
    let entry = text(entry_value)?.to_string();
    member_path(&entry)?;
    require(entry.starts_with("bin/"), "invalid_entry_point")?;
    let parent = entry.rsplit_once('/').map_or("", |(parent, _)| parent);
    let library = match pdfium_library(host_platform()) {
        "" => parent.to_string(),
        file => format!("{parent}/{file}"),
    };
    let mut wanted: BTreeSet<String> = RESOURCES.iter().map(|name| name.to_string()).collect();
    wanted.extend([
        "manifest.json".to_string(),
        ICON.to_string(),
        entry.clone(),
        library.clone(),
        PDFIUM_NOTICE.to_string(),
    ]);
    let present: BTreeSet<String> = names.iter().map(|name| name.to_string()).collect();
    require(present == wanted, "unexpected_archive_members")?;
    // Python's zipfile refuses a member whose local header names another file when it reads it; the
    // crate checks every local header against the central directory, and refuses overlapping members.
    // Checked here, once every member is the expected one, so an unexpected name keeps Python's code.
    bundle.validate_header_consistency().map_err(zip_error)?;
    // The icon is the file committed in packaging/mcpb, unmodified, and the manifest names it.
    require(
        opt(&manifest, "icon") == Some(&json!(ICON)),
        "icon_declaration_mismatch",
    )?;
    let icon = read_member(&mut bundle, ICON)?;
    require(icon.starts_with(PNG_SIGNATURE), "icon_is_not_png")?;
    require(
        icon == fs::read(repository.join(ICON_SOURCE)).map_err(os_error)?,
        "icon_bytes_differ",
    )?;
    let config = key(server, "mcp_config")?;
    let args_empty = opt(config, "args").is_none_or(falsy);
    let overrides_empty = opt(config, "platform_overrides").is_none_or(falsy);
    require(
        key(server, "type")? == "binary"
            && key(config, "command")? == &json!(format!("${{__dirname}}/{entry}"))
            && args_empty
            && overrides_empty,
        "launch_command_does_not_match_binary",
    )?;
    require(
        key(key(&manifest, "compatibility")?, "platforms")? == &json!([host_platform()]),
        "archive_platform_mismatch",
    )?;
    for resource in RESOURCES {
        require(
            read_member(&mut bundle, resource)?
                == fs::read(repository.join(resource)).map_err(os_error)?,
            "legal_resource_bytes_differ",
        )?;
    }
    let pin = pdfium_pin(repository, None)?;
    for (member, prefix) in [(library.as_str(), "library"), (PDFIUM_NOTICE, "notice")] {
        let data = read_member(&mut bundle, member)?;
        let pinned = number_is(
            Some(key(&pin, &format!("{prefix}_bytes"))?),
            data.len() as i64,
        ) && key(&pin, &format!("{prefix}_sha256"))? == &json!(sha256_hex(&data));
        require(pinned, &format!("pdfium_{prefix}_not_pinned"))?;
    }
    fs::create_dir_all(destination).map_err(os_error)?;
    let root = destination.canonicalize().map_err(os_error)?;
    for member in &members {
        let parts = member_path(&member.name)?;
        let target: PathBuf = parts
            .iter()
            .fold(destination.to_path_buf(), |path, part| path.join(part));
        let parent = target.parent().unwrap_or(destination);
        fs::create_dir_all(parent).map_err(os_error)?;
        let resolved = parent.canonicalize().map_err(os_error)?;
        require(
            resolved.starts_with(&root)
                && !target.components().any(|part| part == Component::ParentDir),
            "archive_member_escapes_destination",
        )?;
        let bytes = read_member(&mut bundle, &member.name)?;
        fs::write(&target, bytes).map_err(os_error)?;
    }
    let binary: PathBuf = member_path(&entry)?
        .iter()
        .fold(destination.to_path_buf(), |path, part| path.join(part));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = members
            .iter()
            .find(|member| member.name == entry)
            .map_or(0, |member| member.mode);
        require(mode & 0o100 != 0, "binary_is_not_executable")?;
        fs::set_permissions(&binary, fs::Permissions::from_mode(mode & 0o777)).map_err(os_error)?;
    }
    Ok((manifest, binary))
}

/// Python's truthiness for a JSON value read with `.get()`.
fn falsy(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Bool(flag) => !flag,
        Value::Number(number) => number.as_f64() == Some(0.0),
        Value::String(text) => text.is_empty(),
        Value::Array(items) => items.is_empty(),
        Value::Object(object) => object.is_empty(),
    }
}

/// The tools/list a bundle launched with `environment` must advertise.
pub fn expected_tools(environment: &Map<String, Value>) -> BTreeSet<String> {
    let flag = |name: &str| environment.get(name).and_then(Value::as_str) == Some("true");
    let mut tools: BTreeSet<String> = DEFAULT_TOOLS.iter().map(|name| name.to_string()).collect();
    if flag("BRIDGE_AGENT_ENABLE_IMPORT") || flag("BRIDGE_AGENT_ENABLE_WRITES") {
        tools.extend(IMPORT_TOOLS.iter().map(|name| name.to_string()));
    }
    if flag("BRIDGE_AGENT_ENABLE_WRITES") {
        tools.extend(POSTING_TOOLS.iter().map(|name| name.to_string()));
    }
    if flag("BRIDGE_AGENT_ENABLE_TAX_AUDIT") {
        tools.extend(AUDIT_TOOLS.iter().map(|name| name.to_string()));
    }
    tools
}

/// Python's `str()` of a setting's default, and `json.dumps()` of a bool.
fn setting_text(value: &Value) -> String {
    match value {
        Value::Bool(flag) => flag.to_string(),
        Value::String(text) => text.clone(),
        Value::Null => "None".to_string(),
        other => other.to_string(),
    }
}

pub fn resolve_environment(manifest: &Value) -> Checked<Map<String, Value>> {
    let mappings = key(key(key(manifest, "server")?, "mcp_config")?, "env")?;
    let mappings = mappings
        .as_object()
        .ok_or_else(|| Failure("TypeError".to_string()))?;
    let names: BTreeSet<&str> = mappings.keys().map(String::as_str).collect();
    let wanted: BTreeSet<&str> = [
        "BRIDGE_TERMS_ACCEPTED",
        "BRIDGE_TALLY_HOST",
        "BRIDGE_TALLY_PORT",
        "BRIDGE_AGENT_REDACTION",
        "BRIDGE_AGENT_ENABLE_IMPORT",
        "BRIDGE_AGENT_ENABLE_WRITES",
        "BRIDGE_AGENT_ENABLE_TAX_AUDIT",
    ]
    .into_iter()
    .collect();
    require(names == wanted, "unexpected_environment_mapping")?;
    let user_config = key(manifest, "user_config")?
        .as_object()
        .ok_or_else(|| Failure("TypeError".to_string()))?;
    // No setting may be required. Claude Desktop builds no launch command while a required setting has no
    // stored value, and an update that renames the Terms setting leaves it with none, so the server would
    // never start to refuse (bridge#1413).
    require(
        user_config
            .values()
            .all(|option| opt(option, "required") != Some(&Value::Bool(true))),
        "required_setting_blocks_launch",
    )?;
    // The Terms of Use are accepted by the user, never by default: the setting is off, and the server
    // refuses every tool until it is on (see agent_terms.rs).
    let empty = json!({});
    let terms = user_config.get("accept_terms_2026_10_1").unwrap_or(&empty);
    require(
        opt(terms, "type") == Some(&json!("boolean"))
            && opt(terms, "default") == Some(&Value::Bool(false)),
        "terms_setting_must_be_boolean_and_off",
    )?;
    require(
        mappings["BRIDGE_TERMS_ACCEPTED"] == "${user_config.accept_terms_2026_10_1}",
        "terms_environment_mapping_mismatch",
    )?;
    require(
        mappings["BRIDGE_AGENT_ENABLE_IMPORT"] == "true",
        "import_environment_mapping_mismatch",
    )?;
    let writes = user_config.get("enable_posting_2026_10").unwrap_or(&empty);
    require(
        opt(writes, "type") == Some(&json!("boolean"))
            && opt(writes, "default").is_some_and(Value::is_boolean),
        "writes_default_must_be_boolean",
    )?;
    // Posting stays off by default while the limits recorded on bridge#574 and bridge#579 remain.
    require(
        key(writes, "default")? == &Value::Bool(false),
        "posting_must_default_off",
    )?;
    // The key is versioned (bridge#1090): a value stored under the released enable_writes, whose text
    // offered one voucher at a time, must not turn on batch posting, so it is mapped to nothing.
    require(
        mappings["BRIDGE_AGENT_ENABLE_WRITES"] == "${user_config.enable_posting_2026_10}",
        "writes_environment_mapping_mismatch",
    )?;
    // The tax-audit tool ships off by default behind its own versioned switch.
    let audit = user_config
        .get("enable_tax_audit_2026_10")
        .unwrap_or(&empty);
    require(
        opt(audit, "type") == Some(&json!("boolean"))
            && opt(audit, "default") == Some(&Value::Bool(false)),
        "tax_audit_setting_must_be_boolean_and_off",
    )?;
    require(
        mappings["BRIDGE_AGENT_ENABLE_TAX_AUDIT"] == "${user_config.enable_tax_audit_2026_10}",
        "tax_audit_environment_mapping_mismatch",
    )?;
    // Supply isolated client settings through the manifest itself. Overwriting the resulting environment
    // would hide broken host/port substitutions.
    let mut values: Vec<(String, Value)> = Vec::new();
    for (name, option) in user_config {
        values.push((name.clone(), key(option, "default")?.clone()));
    }
    for (name, value) in [("host", json!("127.0.0.1")), ("port", json!(9))] {
        match values.iter_mut().find(|(existing, _)| existing == name) {
            Some(slot) => slot.1 = value,
            None => values.push((name.to_string(), value)),
        }
    }
    let mut resolved = Map::new();
    for (name, mapping) in mappings {
        let mut value = text(mapping)?.to_string();
        for (setting, default) in &values {
            value = value.replace(
                &format!("${{user_config.{setting}}}"),
                &setting_text(default),
            );
        }
        require(!value.contains("${"), "unresolved_environment_mapping")?;
        resolved.insert(name.clone(), Value::String(value));
    }
    require(
        resolved["BRIDGE_TALLY_HOST"] == "127.0.0.1" && resolved["BRIDGE_TALLY_PORT"] == "9",
        "endpoint_environment_mapping_mismatch",
    )?;
    Ok(resolved)
}

/// Python's `bytes.splitlines(keepends)`: lines end at `\n`, `\r` or `\r\n`.
pub fn split_lines(bytes: &[u8], keep_ends: bool) -> Vec<&[u8]> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < bytes.len() {
        let end_len = match bytes[index] {
            b'\r' if bytes.get(index + 1) == Some(&b'\n') => 2,
            b'\r' | b'\n' => 1,
            _ => 0,
        };
        if end_len > 0 {
            lines.push(&bytes[start..if keep_ends { index + end_len } else { index }]);
            index += end_len;
            start = index;
        } else {
            index += 1;
        }
    }
    if start < bytes.len() {
        lines.push(&bytes[start..]);
    }
    lines
}

fn json_lines(bytes: &[u8]) -> Checked<Vec<Value>> {
    split_lines(bytes, false)
        .into_iter()
        .map(|line| serde_json::from_slice(line).map_err(json_error))
        .collect()
}

fn ids(replies: &[Value]) -> Vec<Option<&Value>> {
    replies.iter().map(|reply| opt(reply, "id")).collect()
}

fn ids_are(replies: &[Value], expected: &[i64]) -> bool {
    let found = ids(replies);
    found.len() == expected.len()
        && found
            .iter()
            .zip(expected)
            .all(|(id, want)| number_is(*id, *want))
}

pub fn validate_schema_receipts(raw: &[u8], response: &[u8]) -> Checked<usize> {
    let records = json_lines(raw)?;
    require(records.len() == 2, "schema_egress_completion_missing")?;
    let (prepared, completed) = (&records[0], &records[1]);
    let digest = json!(sha256_hex(response));
    let receipt = opt(prepared, "receipt_id");
    require(
        opt(prepared, "record_type") == Some(&json!("response_prepared"))
            && opt(prepared, "tool") == Some(&json!("voucher_schema"))
            && receipt
                .and_then(Value::as_str)
                .is_some_and(|id| !id.is_empty())
            && number_is(opt(prepared, "bytes_prepared"), response.len() as i64)
            && opt(prepared, "response_sha256") == Some(&digest),
        "schema_egress_preparation_invalid",
    )?;
    require(
        opt(completed, "record_type") == Some(&json!("stdio_write_completed"))
            && opt(completed, "receipt_id") == receipt
            && number_is(opt(completed, "bytes_written"), response.len() as i64)
            && opt(completed, "response_sha256") == Some(&digest),
        "schema_egress_completion_invalid",
    )?;
    Ok(records.len())
}

pub fn validate_server_version(reply: &Value, manifest: &Value) -> Checked<String> {
    let version = opt(manifest, "version");
    let reported = opt(reply, "result")
        .and_then(|result| opt(result, "serverInfo"))
        .and_then(|info| opt(info, "version"));
    require(
        version.is_some_and(Value::is_string) && reported == version,
        "server_version_mismatch",
    )?;
    Ok(version
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string())
}

/// Run `command` with `payload` on stdin; stdout and stderr each bounded, the whole run timed out.
#[expect(
    clippy::disallowed_methods,
    reason = "a CI check that starts the unpacked bundle's own server, with no Tally reachable (port 9)"
)]
pub fn run_bounded(
    command: &[String],
    payload: &[u8],
    environment: &Map<String, Value>,
    timeout: Duration,
) -> Checked<(Vec<u8>, Vec<u8>)> {
    let mut process = Command::new(&command[0])
        .args(&command[1..])
        .env_clear()
        .envs(
            environment
                .iter()
                .map(|(name, value)| (name, value.as_str().unwrap_or_default())),
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(os_error)?;
    let exceeded = Arc::new(AtomicBool::new(false));
    let reader = |stream: Box<dyn Read + Send>| {
        let exceeded = Arc::clone(&exceeded);
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut collected = Vec::new();
            let _ = stream
                .take(MAX_OUTPUT_BYTES as u64 + 1)
                .read_to_end(&mut collected);
            if collected.len() > MAX_OUTPUT_BYTES {
                exceeded.store(true, Ordering::SeqCst);
            }
            let _ = sender.send(collected);
        });
        receiver
    };
    let stdout = reader(Box::new(process.stdout.take().expect("piped stdout")));
    let stderr = reader(Box::new(process.stderr.take().expect("piped stderr")));
    // Dropped at the end of the statement, which closes the server's stdin. Python raises a write error
    // (a server that closed its stdin early gives BrokenPipeError) once the process has ended.
    let written = process
        .stdin
        .take()
        .map_or(Ok(()), |mut stdin| stdin.write_all(payload));
    let deadline = Instant::now() + timeout;
    let status = loop {
        if exceeded.load(Ordering::SeqCst) {
            let _ = process.kill();
        }
        match process.try_wait().map_err(os_error)? {
            Some(status) => break status,
            None if Instant::now() >= deadline => {
                let _ = process.kill();
                let _ = process.wait();
                return Err(Failure("server_timeout".to_string()));
            }
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    };
    written.map_err(os_error)?;
    // Each reader is waited for one second, as Python's join(timeout=1): a stream still held open (by a
    // process the server started) leaves its output unread, which Python reports as a KeyError.
    let output = stdout.recv_timeout(Duration::from_secs(1)).ok();
    let diagnostics = stderr.recv_timeout(Duration::from_secs(1)).ok();
    require(!exceeded.load(Ordering::SeqCst), "server_output_limit")?;
    require(
        status.success(),
        &format!("server_exit_failed:{}", exit_code(status)),
    )?;
    match (output, diagnostics) {
        (Some(output), Some(diagnostics)) => Ok((output, diagnostics)),
        _ => Err(Failure("KeyError".to_string())),
    }
}

/// Python's `returncode`: the exit code, or minus the signal that ended the process.
fn exit_code(status: std::process::ExitStatus) -> i32 {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return -signal;
        }
    }
    status.code().unwrap_or(-1)
}

fn requests_payload(requests: &[Value]) -> Vec<u8> {
    requests
        .iter()
        .flat_map(|request| {
            let mut line = serde_json::to_vec(request).unwrap_or_default();
            line.push(b'\n');
            line
        })
        .collect()
}

fn initialize() -> Value {
    json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
        "protocolVersion": "2025-06-18", "capabilities": {},
        "clientInfo": {"name": "bridge-mcpb-smoke", "version": "1.0.0"}}})
}

fn with(environment: &Map<String, Value>, name: &str, value: &str) -> Map<String, Value> {
    let mut copy = environment.clone();
    copy.insert(name.to_string(), Value::String(value.to_string()));
    copy
}

fn tool_names(reply: &Value) -> Checked<Vec<String>> {
    let tools = key(key(reply, "result")?, "tools")?
        .as_array()
        .ok_or_else(|| Failure("TypeError".to_string()))?;
    tools
        .iter()
        .map(|tool| Ok(text(key(tool, "name")?)?.to_string()))
        .collect()
}

fn as_set(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| name.to_string()).collect()
}

/// Parse the synthetic statement through the unpacked bundle, so the bundled PDFium is proven to load
/// from beside the binary.
fn statement_smoke(
    command: &str,
    base_environment: &Map<String, Value>,
    temporary: &Path,
    repository: &Path,
) -> Checked<Value> {
    let work = temporary.join("statement");
    fs::create_dir(&work).map_err(os_error)?;
    let statement = work.join("statement.pdf");
    fs::write(
        &statement,
        fs::read(repository.join(STATEMENT_FIXTURE)).map_err(os_error)?,
    )
    .map_err(os_error)?;
    let password_file = work.join("statement.password");
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options
        .open(&password_file)
        .and_then(|mut handle| handle.write_all(format!("{STATEMENT_PASSWORD}\n").as_bytes()))
        .map_err(os_error)?;
    // The default bundle environment, posting off: parsing must not need it.
    let environment = with(
        base_environment,
        "BRIDGE_AGENT_DATA_DIR",
        &work.join("data").to_string_lossy(),
    );
    let arguments = json!({
        "statement_path": statement.to_string_lossy(), "password_file": password_file.to_string_lossy(), "bank": "hdfc",
        "account_label": "Synthetic CA xx4321", "opening_balance": "1,000.00",
        "closing_balance": "1,02,200.00", "total_debits": "8,800.00",
        "total_credits": "1,10,000.00", "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense",
    });
    let requests = [
        initialize(),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        json!({"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "parse_bank_statement", "arguments": arguments}}),
    ];
    let (output, diagnostics) = run_bounded(
        &[command.to_string()],
        &requests_payload(&requests),
        &environment,
        Duration::from_secs(60),
    )?;
    let echoed = [output.as_slice(), diagnostics.as_slice()].concat();
    require(
        !echoed
            .windows(STATEMENT_PASSWORD.len())
            .any(|window| window == STATEMENT_PASSWORD.as_bytes()),
        "statement_password_echoed",
    )?;
    let replies = json_lines(&output)?;
    require(ids_are(&replies, &[1, 2]), "statement_response_ids")?;
    let empty = json!({});
    let call = opt(&replies[1], "result").unwrap_or(&empty);
    let result = opt(call, "structuredContent")
        .and_then(|content| opt(content, "result"))
        .unwrap_or(&empty);
    let code = opt(result, "error")
        .and_then(|error| opt(error, "code"))
        .map_or("no_result".to_string(), setting_text);
    require(
        opt(call, "isError") == Some(&Value::Bool(false))
            && number_is(opt(result, "statement_rows"), 6)
            && number_is(opt(result, "vouchers"), 6)
            && opt(result, "proposals_id").is_some_and(Value::is_string),
        &format!("statement_parse_failed:{code}"),
    )?;
    Ok(key(result, "vouchers")?.clone())
}

/// A folder removed on drop, waiting a bounded time for the server binary inside it: Windows can keep the
/// binary of a process that has just exited locked for a moment. If it stays locked the run still reports
/// what it verified and says on stderr which folder was left behind and the last error.
pub struct ScratchDirectory(pub PathBuf);

impl ScratchDirectory {
    pub fn new(prefix: &str) -> Checked<Self> {
        let base = std::env::temp_dir();
        for attempt in 0..100u32 {
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_nanos());
            let path = base.join(format!("{prefix}{}-{stamp}-{attempt}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(os_error(error)),
            }
        }
        Err(Failure("FileExistsError".to_string()))
    }
}

impl Drop for ScratchDirectory {
    fn drop(&mut self) {
        if let Some(message) =
            remove_with_retries(&self.0, |path| fs::remove_dir_all(path), std::thread::sleep)
        {
            eprintln!("{message}");
        }
    }
}

/// Remove `path`, pausing between attempts and not after the last; the stderr line to print if it was left.
pub fn remove_with_retries(
    path: &Path,
    mut remove: impl FnMut(&Path) -> std::io::Result<()>,
    mut pause: impl FnMut(Duration),
) -> Option<String> {
    for attempt in 0..REMOVE_ATTEMPTS {
        match remove(path) {
            Ok(()) => return None,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return None,
            Err(error) if attempt + 1 == REMOVE_ATTEMPTS => {
                return Some(format!(
                    "MCPB smoke could not remove its scratch folder {} after {REMOVE_ATTEMPTS} attempts (last error: {error:?}); it was left in place",
                    path.display()
                ));
            }
            Err(_) => pause(REMOVE_PAUSE),
        }
    }
    None
}

/// The report, field for field and in the order of `scripts/check-mcpb-bundle.py`.
#[derive(Serialize)]
pub struct Report {
    pub archive_sha256: String,
    pub binary_sha256: String,
    pub platform: String,
    pub archive_entries: usize,
    pub legal_resources: usize,
    pub pdfium_library_sha256: Value,
    pub statement_vouchers: Value,
    pub server_version: String,
    pub response_ids: Vec<Value>,
    pub response_bytes: usize,
    pub default_tool_count: usize,
    pub opt_in_tool_count: usize,
    pub opt_in_stderr_bytes: usize,
    pub egress_receipts: usize,
    pub stderr_bytes: usize,
    pub stderr_sha256: String,
}

pub fn smoke(archive: &Path, repository: &Path) -> Checked<Report> {
    let scratch = ScratchDirectory::new("bridge-mcpb-smoke-")?;
    let temporary = scratch.0.as_path();
    let destination = temporary.join("bundle");
    let (manifest, binary) = unpack_bundle(archive, &destination, repository, LIMITS)?;
    // Python's os.environ: names upper-cased on Windows, where they are case-insensitive, and a value
    // that is not Unicode kept as near as a string allows.
    let mut environment: Map<String, Value> = std::env::vars_os()
        .map(|(name, value)| {
            let name = name.to_string_lossy();
            let name = if cfg!(windows) {
                name.to_uppercase()
            } else {
                name.into_owned()
            };
            (name, value.to_string_lossy().into_owned())
        })
        .filter(|(name, _)| {
            ![
                "BRIDGE_AGENT_",
                "BRIDGE_TALLY_",
                "BRIDGE_PDFIUM_",
                "BRIDGE_TERMS_",
            ]
            .iter()
            .any(|prefix| name.starts_with(prefix))
        })
        .map(|(name, value)| (name, Value::String(value)))
        .collect();
    environment.extend(resolve_environment(&manifest)?);
    environment.insert(
        "BRIDGE_AGENT_DATA_DIR".to_string(),
        Value::String(temporary.join("data").to_string_lossy().into_owned()),
    );
    // With the manifest's defaults the Terms of Use are not accepted: the server must still answer
    // initialize and tools/list, and refuse a tool call in band. Own data folder, so this run's receipts
    // do not mix with the accepted run's below.
    require(
        environment["BRIDGE_TERMS_ACCEPTED"] == "false",
        "terms_must_default_to_not_accepted",
    )?;
    let requests = [
        initialize(),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}),
        json!({"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "voucher_schema", "arguments": {}}}),
    ];
    let command = text(key(
        key(key(&manifest, "server")?, "mcp_config")?,
        "command",
    )?)?
    .replace("${__dirname}", &destination.to_string_lossy());
    let refused_data = temporary.join("data-refused");
    let (refused_output, _) = run_bounded(
        std::slice::from_ref(&command),
        &requests_payload(&requests),
        &with(
            &environment,
            "BRIDGE_AGENT_DATA_DIR",
            &refused_data.to_string_lossy(),
        ),
        Duration::from_secs(15),
    )?;
    let refused = json_lines(&refused_output)?;
    require(ids_are(&refused, &[1, 2, 3]), "terms_refusal_response_ids")?;
    require(
        !falsy(key(key(&refused[1], "result")?, "tools")?),
        "terms_refusal_hid_the_tool_list",
    )?;
    let refusal = key(&refused[2], "result")?;
    let refusal_code = opt(refusal, "structuredContent")
        .and_then(|content| opt(content, "result"))
        .and_then(|result| opt(result, "error"))
        .and_then(|error| opt(error, "code"));
    require(
        opt(refusal, "isError") == Some(&Value::Bool(true))
            && refusal_code == Some(&json!("terms_not_accepted")),
        "terms_refusal_missing",
    )?;
    require(
        !refused_data.join("terms-acceptance.jsonl").exists(),
        "terms_recorded_without_acceptance",
    )?;
    environment.insert(
        "BRIDGE_TERMS_ACCEPTED".to_string(),
        Value::String("true".to_string()),
    );
    let (output, diagnostics) = run_bounded(
        std::slice::from_ref(&command),
        &requests_payload(&requests),
        &environment,
        Duration::from_secs(15),
    )?;
    let replies = json_lines(&output)?;
    require(ids_are(&replies, &[1, 2, 3]), "unexpected_response_ids")?;
    require(
        replies.iter().all(|reply| {
            opt(reply, "jsonrpc") == Some(&json!("2.0")) && opt(reply, "result").is_some()
        }),
        "invalid_jsonrpc_response",
    )?;
    require(
        key(key(&replies[0], "result")?, "protocolVersion")? == "2025-06-18",
        "protocol_mismatch",
    )?;
    let server_version = validate_server_version(&replies[0], &manifest)?;
    let names = tool_names(&replies[1])?;
    let name_set: BTreeSet<String> = names.iter().cloned().collect();
    let expected = expected_tools(&environment);
    require(
        names.len() == expected.len() && name_set == expected,
        "default_tools_mismatch",
    )?;
    require(
        as_set(POSTING_TOOLS).is_disjoint(&name_set) && as_set(IMPORT_TOOLS).is_subset(&name_set),
        "default_bundle_must_prepare_but_not_post",
    )?;
    require(
        as_set(AUDIT_TOOLS).is_disjoint(&name_set),
        "default_bundle_must_not_list_the_tax_audit_tool",
    )?;
    // The user's opt-in adds the posting tools and nothing else; verify_import stays available either way
    // for safe recovery of saved batches.
    let enabled_environment = with(&environment, "BRIDGE_AGENT_ENABLE_WRITES", "true");
    let catalogue_payload = requests_payload(&requests[..3]);
    let (enabled_output, enabled_diagnostics) = run_bounded(
        std::slice::from_ref(&command),
        &catalogue_payload,
        &enabled_environment,
        Duration::from_secs(15),
    )?;
    let enabled_replies = json_lines(&enabled_output)?;
    require(ids_are(&enabled_replies, &[1, 2]), "opt_in_response_ids")?;
    let enabled_names = tool_names(&enabled_replies[1])?;
    let enabled_set: BTreeSet<String> = enabled_names.iter().cloned().collect();
    let enabled_expected = expected_tools(&enabled_environment);
    require(
        enabled_names.len() == enabled_expected.len()
            && enabled_set == enabled_expected
            && enabled_set
                .difference(&name_set)
                .cloned()
                .collect::<BTreeSet<_>>()
                == as_set(POSTING_TOOLS),
        "opt_in_tools_mismatch",
    )?;
    // The tax-audit switch, run on the packaged binary: it adds that one tool and nothing else.
    let audit_environment = with(&environment, "BRIDGE_AGENT_ENABLE_TAX_AUDIT", "true");
    let (audit_output, _) = run_bounded(
        std::slice::from_ref(&command),
        &catalogue_payload,
        &audit_environment,
        Duration::from_secs(15),
    )?;
    let audit_replies = json_lines(&audit_output)?;
    require(
        ids_are(&audit_replies, &[1, 2]),
        "audit_opt_in_response_ids",
    )?;
    let audit_names = tool_names(&audit_replies[1])?;
    let audit_set: BTreeSet<String> = audit_names.iter().cloned().collect();
    let audit_expected = expected_tools(&audit_environment);
    require(
        audit_names.len() == audit_expected.len()
            && audit_set == audit_expected
            && audit_set
                .difference(&name_set)
                .cloned()
                .collect::<BTreeSet<_>>()
                == as_set(AUDIT_TOOLS),
        "audit_opt_in_tools_mismatch",
    )?;
    let schema = key(&replies[2], "result")?;
    // Each lookup after the first runs only when the one before it held, as Python's `and` chain.
    let schema_valid = opt(schema, "isError") == Some(&Value::Bool(false)) && {
        let structured = key(schema, "structuredContent")?;
        key(key(key(structured, "result")?, "schema")?, "type")? == "object" && {
            let first = key(schema, "content")?
                .get(0)
                .ok_or_else(|| Failure("IndexError".to_string()))?;
            serde_json::from_str::<Value>(text(key(first, "text")?)?).map_err(json_error)?
                == *structured
        }
    };
    require(schema_valid, "voucher_schema_response_invalid")?;
    let third_line = split_lines(&output, true)
        .get(2)
        .copied()
        .ok_or_else(|| Failure("IndexError".to_string()))?;
    let receipts = validate_schema_receipts(
        &fs::read(temporary.join("data").join("agent-egress.jsonl")).map_err(os_error)?,
        third_line,
    )?;
    // Accepting the Terms is recorded once, next to the receipts, in the data folder.
    let acceptance_raw =
        fs::read(temporary.join("data").join("terms-acceptance.jsonl")).map_err(os_error)?;
    let acceptance = split_lines(&acceptance_raw, false);
    let recorded = acceptance.len() == 1 && {
        let record: Value = serde_json::from_slice(acceptance[0]).map_err(json_error)?;
        opt(&record, "source") == Some(&json!("setting"))
    };
    require(recorded, "terms_acceptance_not_recorded")?;
    let statement_vouchers = statement_smoke(&command, &environment, temporary, repository)?;
    Ok(Report {
        archive_sha256: sha256_hex(&fs::read(archive).map_err(os_error)?),
        binary_sha256: sha256_hex(&fs::read(&binary).map_err(os_error)?),
        platform: host_platform().to_string(),
        archive_entries: ARCHIVE_MEMBERS,
        legal_resources: RESOURCES.len() + 1,
        pdfium_library_sha256: key(&pdfium_pin(repository, None)?, "library_sha256")?.clone(),
        statement_vouchers,
        server_version,
        response_ids: replies.iter().map(|reply| reply["id"].clone()).collect(),
        response_bytes: output.len(),
        default_tool_count: names.len(),
        opt_in_tool_count: enabled_names.len(),
        opt_in_stderr_bytes: enabled_diagnostics.len(),
        egress_receipts: receipts,
        stderr_bytes: diagnostics.len(),
        stderr_sha256: sha256_hex(&diagnostics),
    })
}

/// The report as Python's `json.dumps(result, indent=2) + "\n"` prints it: every character outside ASCII
/// escaped as `\uxxxx` (a UTF-16 pair above the first plane), which in JSON can only stand in a string.
pub fn report_text(report: &Report) -> Checked<String> {
    let pretty = serde_json::to_string_pretty(report).map_err(json_error)?;
    let mut text = String::with_capacity(pretty.len() + 1);
    for character in pretty.chars() {
        if character.is_ascii() {
            text.push(character);
        } else {
            for unit in character.encode_utf16(&mut [0; 2]) {
                text.push_str(&format!("\\u{unit:04x}"));
            }
        }
    }
    text.push('\n');
    Ok(text)
}

/// The repository this example was built from: `src-tauri/..`.
fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

fn run(arguments: &[String]) -> i32 {
    let (mut archive, mut output) = (None, None);
    let mut items = arguments.iter();
    while let Some(argument) = items.next() {
        match argument.as_str() {
            "--output" => match items.next() {
                Some(path) => output = Some(PathBuf::from(path)),
                None => {
                    eprintln!("check_mcpb_bundle: --output needs a file");
                    return 2;
                }
            },
            _ if archive.is_none() && !argument.starts_with("--") => {
                archive = Some(PathBuf::from(argument))
            }
            _ => {
                eprintln!("usage: check_mcpb_bundle <archive> [--output <file>]");
                return 2;
            }
        }
    }
    let Some(archive) = archive else {
        eprintln!("usage: check_mcpb_bundle <archive> [--output <file>]");
        return 2;
    };
    let result = smoke(&archive, &repository()).and_then(|report| {
        let mut text = report_text(&report)?;
        // Python writes the report in text mode, which ends each line with the host's line ending.
        if cfg!(windows) {
            text = text.replace('\n', "\r\n");
        }
        if let Some(path) = &output {
            fs::write(path, &text).map_err(os_error)?;
        }
        print!("{text}");
        Ok(())
    });
    match result {
        Ok(()) => 0,
        Err(failure) => {
            eprintln!("MCPB smoke failed: {failure}");
            1
        }
    }
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(run(&arguments));
}
