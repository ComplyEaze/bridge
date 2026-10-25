use super::*;

const BOTH: [PathRule; 2] = [PathRule::Windows, PathRule::Unix];

fn refusal(text: &str, rule: PathRule) -> LocalDiskPathRefusal {
    LocalDiskPath::parse_for(text, rule).expect_err(text)
}

#[test]
fn a_text_beginning_with_two_separators_is_refused_under_both_rules() {
    for text in [
        r"\\host\share\statement.pdf",
        r"\\host@80\share\statement.pdf",
        r"\\host@SSL@443\DavWWWRoot\statement.pdf",
        r"\\?\UNC\host\share\statement.pdf",
        r"\\?\C:\data\statement.pdf",
        r"\\.\C:\statement.pdf",
        r"\\.\pipe\name",
        "//host/share/statement.pdf",
        "//?/UNC/host/share/statement.pdf",
        "//./C:/statement.pdf",
        r"\/host\share\statement.pdf",
        r"/\host/share/statement.pdf",
        r"\\",
        "//",
    ] {
        for rule in BOTH {
            assert_eq!(
                refusal(text, rule),
                LocalDiskPathRefusal::DoubleSeparatorPrefix,
                "{text} under {rule:?}"
            );
        }
    }
}

#[test]
fn the_windows_rule_admits_only_a_drive_letter_root() {
    for text in [
        r"C:\data\in\statement.pdf",
        "C:/data/in/statement.pdf",
        r"d:\statement.pdf",
        r"Z:\",
    ] {
        let path = LocalDiskPath::parse_for(text, PathRule::Windows).expect(text);
        assert_eq!(path.as_path(), Path::new(text));
    }
    for text in [
        "",
        "statement.pdf",
        r"Downloads\statement.pdf",
        // drive-relative: resolved against that drive's current directory
        "C:statement.pdf",
        "C:",
        // rooted on the current drive, whatever that is
        r"\data\statement.pdf",
        "/data/statement.pdf",
        // the NT object-manager spelling of a device path
        r"\??\C:\statement.pdf",
        // not a letter before the colon
        // a letter, but not an ASCII one: no drive is named by it
        r"é:\statement.pdf",
        r"Ж:\statement.pdf",
        r"1:\statement.pdf",
        r"::\statement.pdf",
        // a letter and a separator with no colon between them: a relative path
        r"ab\statement.pdf",
        "ab/statement.pdf",
        // more than one letter before the colon
        r"CD:\statement.pdf",
    ] {
        assert_eq!(
            refusal(text, PathRule::Windows),
            LocalDiskPathRefusal::NotALocalRoot,
            "{text}"
        );
    }
}

#[test]
fn the_unix_rule_admits_only_a_single_leading_slash() {
    for text in ["/data/in/statement.pdf", "/tmp/a b.pdf", "/"] {
        let path = LocalDiskPath::parse_for(text, PathRule::Unix).expect(text);
        assert_eq!(path.as_path(), Path::new(text));
    }
    for text in [
        "",
        "statement.pdf",
        "Downloads/statement.pdf",
        "~/statement.pdf",
        r"C:\data\statement.pdf",
        "C:/data/statement.pdf",
        r"\data\statement.pdf",
    ] {
        assert_eq!(
            refusal(text, PathRule::Unix),
            LocalDiskPathRefusal::NotALocalRoot,
            "{text}"
        );
    }
}

#[test]
fn the_host_rule_is_the_one_this_build_runs_on() {
    assert_eq!(
        PathRule::host(),
        if cfg!(windows) {
            PathRule::Windows
        } else {
            PathRule::Unix
        }
    );
    let local = if cfg!(windows) {
        r"C:\statement.pdf"
    } else {
        "/statement.pdf"
    };
    assert_eq!(
        LocalDiskPath::parse(local),
        LocalDiskPath::parse_for(local, PathRule::host())
    );
    assert!(LocalDiskPath::parse(local).is_ok());
    assert_eq!(
        LocalDiskPath::parse(r"\\host\share\statement.pdf"),
        Err(LocalDiskPathRefusal::DoubleSeparatorPrefix)
    );
}
