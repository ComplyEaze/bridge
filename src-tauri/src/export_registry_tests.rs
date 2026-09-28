use super::*;

fn registry() -> (tempfile::TempDir, PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(FILE_NAME);
    (directory, path)
}

/// The registry's hash is the one the uploader computes for a file with the
/// same bytes: SHA-256 in upper-case hex.
#[test]
fn a_recorded_export_is_known_by_its_content_hash() {
    let (_directory, path) = registry();
    assert!(recorded_in(&path).unwrap().is_empty(), "no registry yet");
    let sha256 = content_sha256(b"abc");
    assert_eq!(
        sha256,
        "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
    );
    record_in(&path, &sha256).unwrap();
    record_in(&path, &content_sha256(b"abd")).unwrap();
    let known = recorded_in(&path).unwrap();
    assert!(known.contains(&sha256));
    assert!(known.contains(&content_sha256(b"abd")));
    assert!(!known.contains(&content_sha256(b"abe")));
}

/// At the bound, the oldest tenth is dropped as the next is recorded, every
/// line stays whole, and the next exports append again.
#[test]
fn the_registry_keeps_only_the_newest_entries() {
    let (_directory, path) = registry();
    let lines = (0..MAX_ENTRIES)
        .map(|index| content_sha256(index.to_string().as_bytes()))
        .collect::<Vec<_>>();
    fs::write(&path, lines.join("\n") + "\n").unwrap();
    let newest = content_sha256(b"newest");
    record_in(&path, &newest).unwrap();
    let kept = read_lines(&path).unwrap();
    let tenth = MAX_ENTRIES / 10;
    assert_eq!(kept.len(), MAX_ENTRIES - tenth);
    assert_eq!(
        kept.first(),
        lines.get(tenth + 1),
        "the oldest tenth was dropped"
    );
    assert_eq!(kept.last(), Some(&newest));
    let next = content_sha256(b"next");
    record_in(&path, &next).unwrap();
    assert_eq!(read_lines(&path).unwrap().len(), MAX_ENTRIES - tenth + 1);
    let leftovers = fs::read_dir(path.parent().unwrap())
        .unwrap()
        .filter(|entry| entry.as_ref().unwrap().path() != path)
        .count();
    assert_eq!(leftovers, 0, "no staging file is left behind");
}

/// A torn or foreign line hides nothing else; an unreadable registry is an
/// error, not an empty one.
#[test]
fn a_torn_line_is_left_out_and_an_unreadable_registry_is_an_error() {
    let (directory, path) = registry();
    let whole = content_sha256(b"whole");
    let mut torn = format!("{whole}\nnot a hash \u{fffd}\n").into_bytes();
    torn.extend_from_slice(&[0xff, b'\n']);
    torn.extend_from_slice(&whole.as_bytes()[..20]);
    fs::write(&path, torn).unwrap();
    assert_eq!(read_lines(&path).unwrap(), vec![whole.clone()]);
    let next = content_sha256(b"next");
    record_in(&path, &next).unwrap();
    assert_eq!(read_lines(&path).unwrap(), vec![whole, next]);
    let unreadable = directory.path().join("a-directory");
    fs::create_dir(&unreadable).unwrap();
    assert_eq!(
        recorded_in(&unreadable),
        Err("export_registry_unreadable".to_string())
    );
}

/// A last line a power cut left zero-filled has the length of a whole line
/// but no newline: the next hash still gets a line of its own.
#[test]
fn a_zero_filled_last_line_does_not_swallow_the_next_hash() {
    let (_directory, path) = registry();
    let whole = content_sha256(b"whole");
    let mut bytes = format!("{whole}\n").into_bytes();
    bytes.extend_from_slice(&[0_u8; 65]);
    fs::write(&path, bytes).unwrap();
    let next = content_sha256(b"next");
    record_in(&path, &next).unwrap();
    assert_eq!(read_lines(&path).unwrap(), vec![whole, next]);
}
