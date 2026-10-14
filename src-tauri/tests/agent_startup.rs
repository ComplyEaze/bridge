#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use std::process::{Command, Stdio};

fn startup_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_bridge_mcp"));
    command
        .env("BRIDGE_TALLY_HOST", "127.0.0.1")
        .env("BRIDGE_TALLY_PORT", "9")
        .env("BRIDGE_AGENT_REDACTION", "none")
        .env("BRIDGE_AGENT_MAX_ROWS", "500")
        .env("BRIDGE_AGENT_MAX_BYTES", "200000")
        .stdin(Stdio::null());
    command
}

#[cfg(any(unix, windows))]
fn non_unicode_component() -> std::ffi::OsString {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        std::ffi::OsString::from_vec(vec![b'a', 0xff])
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt;
        std::ffi::OsString::from_wide(&[b'a' as u16, 0xd800])
    }
}

#[cfg(any(unix, windows))]
#[test]
fn non_unicode_data_directory_fails_before_creating_state() {
    let directory = tempfile::tempdir().unwrap();
    let data = directory.path().join(non_unicode_component());
    assert!(data.to_str().is_none());
    assert!(serde_json::to_value(&data).is_err());
    let output = startup_command()
        .env("BRIDGE_AGENT_DATA_DIR", &data)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap().trim(),
        "bridge-mcp: agent_data_dir_encoding_invalid"
    );
    assert!(!data.exists());
}

#[cfg(any(target_os = "macos", windows))]
#[test]
fn non_unicode_default_data_directory_fails_before_creating_state() {
    let directory = tempfile::tempdir().unwrap();
    let base = directory.path().join(non_unicode_component());
    let mut command = startup_command();
    command.env_remove("BRIDGE_AGENT_DATA_DIR");
    #[cfg(target_os = "macos")]
    command.env("HOME", &base);
    #[cfg(windows)]
    command.env("LOCALAPPDATA", &base);
    let output = command.output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap().trim(),
        "bridge-mcp: agent_data_dir_encoding_invalid"
    );
    assert!(!base.exists());
}

#[test]
fn unicode_data_directory_is_preserved_and_admitted() {
    let directory = tempfile::tempdir().unwrap();
    let data = directory.path().join("पुस्तक-账簿-📚");
    let output = startup_command()
        .env("BRIDGE_AGENT_DATA_DIR", &data)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert!(data.is_dir());
    assert_eq!(serde_json::to_value(&data).unwrap().as_str(), data.to_str());
}

#[test]
fn invalid_endpoint_settings_fail_before_creating_state_or_advertising_tools() {
    for host in [
        "example.invalid",
        "192.0.2.1",
        "",
        "localhost/path",
        "localhost:9001",
        "[::1]",
    ] {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("not-created");
        let output = Command::new(env!("CARGO_BIN_EXE_bridge_mcp"))
            .env("BRIDGE_TALLY_HOST", host)
            .env("BRIDGE_TALLY_PORT", "9001")
            .env("BRIDGE_AGENT_DATA_DIR", &data)
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8(output.stderr).unwrap().trim(),
            "bridge-mcp: host_setting_invalid"
        );
        assert!(!data.exists());
    }
}

#[test]
fn an_invalid_master_writes_setting_stops_startup_whatever_posting_is() {
    for writes in [None, Some("false"), Some("true")] {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("data");
        let mut command = startup_command();
        command
            .env("BRIDGE_AGENT_DATA_DIR", &data)
            .env("BRIDGE_AGENT_ENABLE_MASTER_WRITES", "maybe");
        if let Some(writes) = writes {
            command.env("BRIDGE_AGENT_ENABLE_WRITES", writes);
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(1), "writes={writes:?}");
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8(output.stderr).unwrap().trim(),
            "bridge-mcp: boolean_setting_invalid",
            "writes={writes:?}"
        );
    }
}

#[test]
fn a_valid_master_writes_setting_starts_with_or_without_posting() {
    for (master, writes) in [
        ("true", "true"),
        ("true", "false"),
        ("false", "true"),
        ("0", "0"),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("data");
        let output = startup_command()
            .env("BRIDGE_AGENT_DATA_DIR", &data)
            .env("BRIDGE_AGENT_ENABLE_MASTER_WRITES", master)
            .env("BRIDGE_AGENT_ENABLE_WRITES", writes)
            .output()
            .unwrap();
        assert!(output.status.success(), "{master} {writes}: {output:?}");
    }
}

/// The names `tools/list` serves from a real `bridge_mcp` process started with
/// these settings: the only test that reads `Settings::from_env`'s own result.
fn served_tools(master: &str, writes: &str) -> Vec<String> {
    let text = session_output(
        master,
        writes,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    );
    let listing = text
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .find(|response| response["id"] == 2)
        .unwrap_or_else(|| panic!("no tools/list answer: {text}"));
    listing["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap().to_string())
        .collect()
}

/// Everything a real `bridge_mcp` process prints for an initialize and then
/// `request`, started with these master-writes and posting settings.
fn session_output(master: &str, writes: &str, request: &str) -> String {
    use std::io::Write;
    let directory = tempfile::tempdir().unwrap();
    let mut child = startup_command()
        .env("BRIDGE_AGENT_DATA_DIR", directory.path().join("data"))
        .env("BRIDGE_TERMS_ACCEPTED", "true")
        .env("BRIDGE_AGENT_ENABLE_MASTER_WRITES", master)
        .env("BRIDGE_AGENT_ENABLE_WRITES", writes)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin
        .write_all(
            format!(
                "{}\n{request}\n",
                r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}"#
            )
            .as_bytes(),
        )
        .unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn the_master_tools_are_served_only_when_the_setting_and_posting_are_both_on() {
    let has_master = |tools: &[String]| {
        ["post_ledger_creates", "prepare_ledger_creates"]
            .iter()
            .all(|name| tools.iter().any(|tool| tool == name))
    };
    let none_master = |tools: &[String]| {
        !tools
            .iter()
            .any(|tool| tool == "post_ledger_creates" || tool == "prepare_ledger_creates")
    };
    assert!(has_master(&served_tools("true", "true")));
    assert!(none_master(&served_tools("false", "true")));
    assert!(none_master(&served_tools("true", "false")));
    assert!(none_master(&served_tools("false", "false")));
}

/// The setting alone does not open the tools: with posting off, a call is
/// refused `master_writes_disabled` in a real process, listed or not.
#[test]
fn a_master_tool_call_is_refused_when_posting_is_off_even_with_the_setting_on() {
    let request = r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"prepare_ledger_creates","arguments":{"company_guid":"61c6de69-1748-461c-ad3f-162cb949df9f","ledger":{"name":"A","kind":"suspense"}}}}"#;
    for (master, writes, refused) in [
        ("true", "false", true),
        ("false", "true", true),
        ("false", "false", true),
        ("true", "true", false),
    ] {
        let text = session_output(master, writes, request);
        assert_eq!(
            text.contains("master_writes_disabled"),
            refused,
            "master={master} writes={writes}: {text}"
        );
    }
}
