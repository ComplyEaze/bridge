use super::*;
use std::io;

fn stripped(text: &str) -> PathBuf {
    strip_verbatim_prefix(PathBuf::from(text))
}

#[test]
fn a_verbatim_drive_path_loses_its_prefix() {
    assert_eq!(stripped(r"\\?\C:\x"), PathBuf::from(r"C:\x"));
    assert_eq!(
        stripped(r"\\?\D:\Profiles\a\AppData\Local\Bridge\agent"),
        PathBuf::from(r"D:\Profiles\a\AppData\Local\Bridge\agent")
    );
    assert_eq!(stripped(r"\\?\C:"), PathBuf::from(r"C:"));
}

#[test]
fn a_verbatim_unc_path_becomes_a_plain_share_path() {
    assert_eq!(
        stripped(r"\\?\UNC\server\share\x"),
        PathBuf::from(r"\\server\share\x")
    );
    assert_eq!(
        stripped(r"\\?\unc\server\share"),
        PathBuf::from(r"\\server\share")
    );
}

#[test]
fn a_path_with_no_plain_form_is_left_as_it_is() {
    for text in [
        r"C:\x",
        r"\\server\share\x",
        r"\\?\Volume{0123}\x",
        r"\\?\UNC\",
        r"\\?\UNC",
        r"\\?\",
        r"\\?\1:\x",
        r"\\?\C:x",
        r"\\.\C:\x",
        "relative/path",
    ] {
        assert_eq!(stripped(text), PathBuf::from(text), "{text}");
    }
}

#[test]
fn a_path_of_248_characters_or_more_keeps_its_verbatim_form() {
    let prefix = r"C:\";
    let short = format!(r"{prefix}{}", "a".repeat(247 - prefix.len()));
    assert_eq!(short.len(), 247);
    assert_eq!(stripped(&format!(r"\\?\{short}")), PathBuf::from(&short));
    let long = format!(r"{prefix}{}", "a".repeat(248 - prefix.len()));
    assert_eq!(long.len(), 248);
    let verbatim = format!(r"\\?\{long}");
    assert_eq!(stripped(&verbatim), PathBuf::from(&verbatim));
    let unc = format!(r"\\?\UNC\server\share\{}", "a".repeat(250));
    assert_eq!(stripped(&unc), PathBuf::from(&unc));
}

#[test]
fn a_component_ending_in_a_dot_or_space_keeps_the_verbatim_form() {
    for text in [
        r"\\?\C:\a\name.",
        r"\\?\C:\a\name \b",
        r"\\?\C:\a\name.txt.",
        r"\\?\UNC\server\share.\x",
    ] {
        assert_eq!(stripped(text), PathBuf::from(text), "{text}");
    }
    // A dot or space inside a name is fine.
    assert_eq!(
        stripped(r"\\?\C:\a b\n.txt"),
        PathBuf::from(r"C:\a b\n.txt")
    );
}

#[test]
fn a_reserved_device_name_keeps_the_verbatim_form() {
    for name in [
        "CON",
        "con",
        "Prn",
        "AUX",
        "nul",
        "COM1",
        "com9",
        "LPT1",
        "lpt9",
        "NUL.txt",
        "con.tar.gz",
    ] {
        let text = format!(r"\\?\C:\a\{name}\b");
        assert_eq!(stripped(&text), PathBuf::from(&text), "{name}");
        let last = format!(r"\\?\C:\a\{name}");
        assert_eq!(stripped(&last), PathBuf::from(&last), "{name}");
    }
    // Names that only resemble a device name are plain.
    for name in [
        "CONSOLE",
        "COM0",
        "COM10",
        "LPT",
        "COMX",
        "AUXILIARY",
        "NULL",
    ] {
        assert_eq!(
            stripped(&format!(r"\\?\C:\a\{name}")),
            PathBuf::from(format!(r"C:\a\{name}")),
            "{name}"
        );
    }
}

#[test]
fn a_resolved_path_is_marked_resolved_and_loses_its_prefix() {
    let shown = from_resolution(
        Path::new(r"C:\logical\x"),
        Ok(PathBuf::from(r"\\?\C:\real\x")),
    );
    assert_eq!(shown, VisiblePath::Resolved(PathBuf::from(r"C:\real\x")));
    assert_eq!(shown.state(), "resolved");
}

#[test]
fn a_failed_resolution_is_never_shown_as_resolved() {
    for kind in [
        io::ErrorKind::NotFound,
        io::ErrorKind::PermissionDenied,
        io::ErrorKind::Other,
    ] {
        let logical = Path::new(r"C:\logical\x");
        let shown = from_resolution(logical, Err(io::Error::from(kind)));
        assert_eq!(shown, VisiblePath::Unresolved(logical.to_path_buf()));
        assert_eq!(shown.state(), "unresolved");
        assert_eq!(shown.path(), logical);
    }
}

#[test]
fn put_into_writes_the_path_and_how_it_was_obtained() {
    let mut value = serde_json::json!({});
    VisiblePath::Unresolved(PathBuf::from("/nowhere")).put_into(&mut value, "folder_path");
    assert_eq!(value["folder_path"], "/nowhere");
    assert_eq!(value["folder_path_state"], "unresolved");
}

#[cfg(not(windows))]
#[test]
fn off_windows_an_existing_path_is_returned_unchanged_and_not_resolved() {
    let directory = tempfile::tempdir().unwrap();
    let real = directory.path().join("real");
    std::fs::create_dir(&real).unwrap();
    // A link: a canonicalizing implementation would return `real`.
    let link = directory.path().join("link");
    std::os::unix::fs::symlink(&real, &link).unwrap();
    for path in [&real, &link] {
        let shown = user_visible_path(path);
        assert_eq!(shown, VisiblePath::AsIs(path.clone()));
        assert_eq!(shown.state(), "as_is");
    }
}

#[cfg(not(windows))]
#[test]
fn off_windows_a_missing_path_is_unresolved_and_unchanged() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("not-yet").join("agent");
    let shown = user_visible_path(&missing);
    assert_eq!(shown, VisiblePath::Unresolved(missing));
}

#[cfg(windows)]
#[test]
fn on_windows_an_existing_file_is_resolved_without_the_verbatim_prefix() {
    let directory = tempfile::tempdir().unwrap();
    let file = directory.path().join("bridge-x.xml");
    std::fs::write(&file, b"<x/>").unwrap();
    let shown = user_visible_path(&file);
    assert_eq!(shown.state(), "resolved");
    let text = shown.display_string();
    assert!(!text.starts_with(r"\\?\"), "{text}");
    // The same file: both canonical forms agree, and the shown path opens.
    assert_eq!(
        std::fs::canonicalize(shown.path()).unwrap(),
        std::fs::canonicalize(&file).unwrap()
    );
    assert_eq!(std::fs::read(shown.path()).unwrap(), b"<x/>");
}

#[cfg(windows)]
#[test]
fn on_windows_a_missing_path_is_the_logical_path_marked_unresolved() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("not-yet").join("agent");
    let shown = user_visible_path(&missing);
    assert_eq!(shown, VisiblePath::Unresolved(missing));
}
