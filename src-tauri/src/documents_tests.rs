#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::{
    authorize_selected_paths, clean_etag, hash_file, prepare_upload_snapshot,
    resolve_scanned_files, scan_documents, validate_presign_mapping, validate_upload_url,
    validate_upload_url_with_allowed_origins, PresignResponse, ScanDocumentsRequest,
};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

#[test]
fn parses_multipart_presign_response() {
    let json = r#"
{
  "batchId": "batch-1",
  "documents": [
    {
      "relativePath": "invoices/a.pdf",
      "fileKey": "workspaces/ws/documents/a.pdf",
      "isMultipart": true,
      "uploadId": "upload-1",
      "parts": [
        { "partNumber": 1, "url": "https://upload.test/1", "expectedSize": 5242880 },
        { "partNumber": 2, "url": "https://upload.test/2", "expectedSize": 128 }
      ]
    }
  ],
  "duplicates": []
}
"#;

    let response =
        serde_json::from_str::<PresignResponse>(json).expect("presign response should parse");
    let document = &response.documents[0];

    assert_eq!(response.batch_id, "batch-1");
    assert!(document.is_multipart);
    assert_eq!(document.upload_id.as_deref(), Some("upload-1"));
    assert_eq!(document.parts.len(), 2);
    assert_eq!(document.parts[0].part_number, 1);
    assert_eq!(document.parts[0].expected_size, 5_242_880);
}

#[test]
fn cleans_quoted_etags() {
    assert_eq!(clean_etag("\"abc123\""), "abc123");
    assert_eq!(clean_etag("'abc123'"), "abc123");
}

#[tokio::test]
async fn file_hash_remains_sha256_compatible() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let path = directory.path().join("hash-input.bin");
    std::fs::write(&path, b"abc").expect("write hash input");

    assert_eq!(
        hash_file(&path).await.expect("hash file"),
        "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
    );
}

#[test]
fn upload_urls_require_https_without_embedded_credentials() {
    assert!(validate_upload_url_with_allowed_origins(
        "https://storage.example/file?signature=ok",
        Some("https://storage.example")
    )
    .is_ok());
    assert!(validate_upload_url("http://storage.example/file").is_err());
    assert!(validate_upload_url("https://user:pass@storage.example/file").is_err());
    assert!(validate_upload_url("https://storage.example/file").is_err());
}

#[tokio::test]
async fn native_selection_and_opaque_scan_ids_authorize_sync() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let selected = directory.path().join("selected.txt");
    std::fs::write(&selected, b"authorized content").expect("write selected file");
    let selections =
        authorize_selected_paths(vec![selected.clone()]).expect("authorize native selection");
    let selection_json = serde_json::to_string(&selections).expect("serialize selections");
    assert!(!selection_json.contains(&directory.path().display().to_string()));
    let response = scan_documents(ScanDocumentsRequest {
        selection_ids: vec![
            selections[0].selection_id.clone(),
            uuid::Uuid::new_v4().to_string(),
        ],
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: false,
        exclude_zero_byte_files: false,
    })
    .await
    .expect("scan selected file");

    assert_eq!(response.files.len(), 1);
    let response_json = serde_json::to_string(&response).expect("serialize scan response");
    assert!(!response_json.contains(&directory.path().display().to_string()));
    assert!(response
        .skipped
        .iter()
        .any(|skipped| skipped.reason.contains("invalid, expired")));
    let serialized = serde_json::to_string(&response.files[0]).expect("serialize file");
    assert!(serialized.contains("scanId"));
    assert!(!serialized.contains("fullPath"));
    assert!(!serialized.contains(&selected.display().to_string()));

    let unknown: PresignResponse = serde_json::from_value(serde_json::json!({
        "batchId": "batch",
        "documents": [{
            "relativePath": "unknown.txt",
            "fileKey": "key",
            "url": "https://storage.example/file"
        }],
        "duplicates": []
    }))
    .expect("parse unknown presign");
    assert!(validate_presign_mapping(&response.files, &unknown).is_err());

    let duplicate: PresignResponse = serde_json::from_value(serde_json::json!({
        "batchId": "batch",
        "documents": [
            {
                "relativePath": response.files[0].relative_path.clone(),
                "fileKey": "key-1",
                "url": "https://storage.example/1"
            },
            {
                "relativePath": response.files[0].relative_path.clone(),
                "fileKey": "key-2",
                "url": "https://storage.example/2"
            }
        ],
        "duplicates": []
    }))
    .expect("parse duplicate presign");
    assert!(validate_presign_mapping(&response.files, &duplicate).is_err());

    let resolved = resolve_scanned_files(&response.scan_session_id, &response.files)
        .expect("resolve authorized scan ID");
    assert_eq!(
        resolved[0].native_path,
        std::fs::canonicalize(&selected).unwrap()
    );
    std::fs::write(&selected, vec![b'X'; "authorized content".len()])
        .expect("replace with same-size content");
    assert!(prepare_upload_snapshot(&resolved[0]).await.is_err());
    assert!(resolve_scanned_files(&response.scan_session_id, &response.files).is_err());

    let mut forged = response.files[0].clone();
    forged.scan_id = uuid::Uuid::new_v4().to_string();
    assert!(resolve_scanned_files(&response.scan_session_id, &[forged]).is_err());
    assert!(resolve_scanned_files("expired", &response.files).is_err());
}

#[tokio::test]
async fn upload_snapshot_is_immutable_after_source_changes() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let selected = directory.path().join("snapshot.txt");
    std::fs::write(&selected, b"original").expect("write source");
    let selections = authorize_selected_paths(vec![selected.clone()]).expect("authorize selection");
    let response = scan_documents(ScanDocumentsRequest {
        selection_ids: vec![selections[0].selection_id.clone()],
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: true,
        exclude_zero_byte_files: true,
    })
    .await
    .expect("scan source");
    let files =
        resolve_scanned_files(&response.scan_session_id, &response.files).expect("resolve source");
    let mut snapshot = prepare_upload_snapshot(&files[0]).await.expect("snapshot");
    std::fs::write(&selected, b"tampered").expect("replace source");
    snapshot.rewind().await.expect("rewind snapshot");
    let mut content = Vec::new();
    snapshot
        .read_to_end(&mut content)
        .await
        .expect("read snapshot");
    assert_eq!(content, b"original");
}

#[tokio::test]
async fn independent_scan_sessions_do_not_invalidate_each_other() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let first = directory.path().join("first.txt");
    let second = directory.path().join("second.txt");
    std::fs::write(&first, b"first").expect("write first");
    std::fs::write(&second, b"second").expect("write second");
    let selections = authorize_selected_paths(vec![first, second]).expect("authorize files");
    let request = |selection_id: String| ScanDocumentsRequest {
        selection_ids: vec![selection_id],
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: true,
        exclude_zero_byte_files: true,
    };
    let first_scan = scan_documents(request(selections[0].selection_id.clone()))
        .await
        .expect("first scan");
    let second_scan = scan_documents(request(selections[1].selection_id.clone()))
        .await
        .expect("second scan");

    assert!(resolve_scanned_files(&first_scan.scan_session_id, &first_scan.files).is_ok());
    assert!(resolve_scanned_files(&second_scan.scan_session_id, &second_scan.files).is_ok());
}

#[cfg(unix)]
#[tokio::test]
async fn directory_scan_rejects_symlink_escape() {
    use std::os::unix::fs::symlink;

    let selected = tempfile::tempdir().expect("selected directory");
    let outside = tempfile::tempdir().expect("outside directory");
    std::fs::write(outside.path().join("private.txt"), b"private").expect("outside file");
    symlink(outside.path(), selected.path().join("escape")).expect("create symlink");
    let selections =
        authorize_selected_paths(vec![selected.path().to_path_buf()]).expect("authorize directory");
    let response = scan_documents(ScanDocumentsRequest {
        selection_ids: vec![selections[0].selection_id.clone()],
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: true,
        exclude_zero_byte_files: true,
    })
    .await
    .expect("scan directory");
    assert!(response.files.is_empty());
    assert!(response.skipped.iter().any(|skipped| skipped
        .reason
        .contains("Symbolic links and filesystem reparse points")));
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn macos_hidden_flag_is_excluded() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let hidden = directory.path().join("hidden-without-dot.txt");
    std::fs::write(&hidden, b"hidden").expect("write hidden file");
    assert!(std::process::Command::new("chflags")
        .arg("hidden")
        .arg(&hidden)
        .status()
        .expect("run chflags")
        .success());
    let selections = authorize_selected_paths(vec![directory.path().to_path_buf()])
        .expect("authorize directory");
    let response = scan_documents(ScanDocumentsRequest {
        selection_ids: vec![selections[0].selection_id.clone()],
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: true,
        exclude_zero_byte_files: true,
    })
    .await
    .expect("scan directory");
    assert!(response.files.is_empty());
}

#[cfg(windows)]
#[tokio::test]
async fn windows_junction_escape_is_rejected() {
    let selected = tempfile::tempdir().expect("selected directory");
    let outside = tempfile::tempdir().expect("outside directory");
    std::fs::write(outside.path().join("private.txt"), b"private").expect("outside file");
    let junction = selected.path().join("escape");
    let status = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(&junction)
        .arg(outside.path())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .expect("create junction");
    assert!(status.success(), "Windows junction creation failed");

    let selections =
        authorize_selected_paths(vec![selected.path().to_path_buf()]).expect("authorize directory");
    let response = scan_documents(ScanDocumentsRequest {
        selection_ids: vec![selections[0].selection_id.clone()],
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: true,
        exclude_zero_byte_files: true,
    })
    .await
    .expect("scan directory");
    assert!(response.files.is_empty());
    assert!(response.skipped.iter().any(|skipped| skipped
        .reason
        .contains("Symbolic links and filesystem reparse points")));
    std::fs::remove_dir(&junction).expect("remove junction");
}

/// bridge#833: a file Bridge exported, found in a folder the user chose, is
/// listed as skipped with its reason and never uploaded, while the user's own
/// file beside it, and an edited copy of the export, are scanned as usual.
#[tokio::test]
async fn a_bridge_export_in_a_chosen_folder_is_skipped_but_a_user_file_is_not() {
    crate::export_registry::init_for_tests();
    let export = format!("statement {}", uuid::Uuid::new_v4()).into_bytes();
    crate::export_registry::record(&export).expect("record the export");
    let selected = tempfile::tempdir().expect("selected directory");
    std::fs::write(
        selected.path().join("statement-party-20260928.xlsx"),
        &export,
    )
    .unwrap();
    std::fs::write(selected.path().join("renamed.xlsx"), &export).unwrap();
    std::fs::write(selected.path().join("invoice.pdf"), b"the user's own file").unwrap();
    let mut edited = export.clone();
    edited.extend_from_slice(b" edited");
    std::fs::write(selected.path().join("statement-edited.xlsx"), edited).unwrap();
    let selections =
        authorize_selected_paths(vec![selected.path().to_path_buf()]).expect("authorize directory");
    let response = scan_documents(ScanDocumentsRequest {
        selection_ids: vec![selections[0].selection_id.clone()],
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: true,
        exclude_zero_byte_files: true,
    })
    .await
    .expect("scan directory");
    let mut scanned = response
        .files
        .iter()
        .map(|file| file.relative_path.as_str())
        .collect::<Vec<_>>();
    scanned.sort_unstable();
    assert_eq!(scanned, ["invoice.pdf", "statement-edited.xlsx"]);
    let mut skipped = response
        .skipped
        .iter()
        .map(|file| (file.path.as_str(), file.reason.as_str()))
        .collect::<Vec<_>>();
    skipped.sort_unstable();
    assert_eq!(
        skipped,
        [
            ("renamed.xlsx", super::BRIDGE_EXPORT_SKIPPED),
            (
                "statement-party-20260928.xlsx",
                super::BRIDGE_EXPORT_SKIPPED
            ),
        ]
    );
}

/// bridge#833: an export takes no relative path from the user's own file of
/// the same name in another selected folder.
#[tokio::test]
async fn an_export_does_not_take_a_user_files_name_in_another_folder() {
    crate::export_registry::init_for_tests();
    let export = format!("statement {}", uuid::Uuid::new_v4()).into_bytes();
    crate::export_registry::record(&export).expect("record the export");
    let first = tempfile::tempdir().expect("first directory");
    let second = tempfile::tempdir().expect("second directory");
    std::fs::write(first.path().join("x.xlsx"), &export).unwrap();
    std::fs::write(second.path().join("x.xlsx"), b"the user's own workbook").unwrap();
    let selections = authorize_selected_paths(vec![
        first.path().to_path_buf(),
        second.path().to_path_buf(),
    ])
    .expect("authorize directories");
    let response = scan_documents(ScanDocumentsRequest {
        selection_ids: selections
            .iter()
            .map(|selection| selection.selection_id.clone())
            .collect(),
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: true,
        exclude_zero_byte_files: true,
    })
    .await
    .expect("scan directories");
    assert_eq!(response.files.len(), 1, "{:?}", response.skipped.len());
    assert_eq!(response.files[0].relative_path, "x.xlsx");
    assert_eq!(response.files[0].size, 23);
    let reasons = response
        .skipped
        .iter()
        .map(|file| file.reason.as_str())
        .collect::<Vec<_>>();
    assert_eq!(reasons, [super::BRIDGE_EXPORT_SKIPPED]);
}

fn scan_everything(selection_id: String) -> ScanDocumentsRequest {
    ScanDocumentsRequest {
        selection_ids: vec![selection_id],
        use_hash: true,
        max_file_size: None,
        excluded_extensions: None,
        exclude_hidden_files: false,
        exclude_zero_byte_files: false,
    }
}

/// bridge#833: a statement the writer had not finished, a hidden staging file
/// or the empty file reserving its name, is never listed, even with the
/// hidden-file and zero-byte settings off: the scan skips both kinds always.
#[tokio::test]
async fn a_partial_or_reserved_statement_is_never_listed() {
    let selected = tempfile::tempdir().expect("selected directory");
    std::fs::write(
        selected
            .path()
            .join(crate::reports::bulk_party_statement::staging_name(
                "statement-party-20260928.xlsx",
            )),
        b"half a statement",
    )
    .unwrap();
    std::fs::write(selected.path().join("statement-party-20260928.xlsx"), b"").unwrap();
    std::fs::write(selected.path().join("invoice.pdf"), b"the user's own file").unwrap();
    let selections =
        authorize_selected_paths(vec![selected.path().to_path_buf()]).expect("authorize directory");
    let response =
        super::scan_documents_with(scan_everything(selections[0].selection_id.clone()), || {
            Ok(std::collections::HashSet::new())
        })
        .await
        .expect("scan directory");
    let scanned = response
        .files
        .iter()
        .map(|file| file.relative_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(scanned, ["invoice.pdf"]);
}

/// bridge#833: a registry that cannot be read stops the scan, rather than
/// letting an export through unchecked.
#[tokio::test]
async fn an_unreadable_export_registry_stops_the_scan() {
    let selected = tempfile::tempdir().expect("selected directory");
    std::fs::write(selected.path().join("invoice.pdf"), b"the user's own file").unwrap();
    let selections =
        authorize_selected_paths(vec![selected.path().to_path_buf()]).expect("authorize directory");
    let error =
        super::scan_documents_with(scan_everything(selections[0].selection_id.clone()), || {
            Err("export_registry_unreadable".to_string())
        })
        .await
        .err()
        .expect("the scan refuses");
    assert!(
        error.to_string().contains("export_registry_unreadable"),
        "{error}"
    );
    // The same when only the read after the walk fails.
    let selections =
        authorize_selected_paths(vec![selected.path().to_path_buf()]).expect("authorize directory");
    let reads = std::cell::Cell::new(0);
    let error =
        super::scan_documents_with(scan_everything(selections[0].selection_id.clone()), || {
            reads.set(reads.get() + 1);
            if reads.get() == 1 {
                Ok(std::collections::HashSet::new())
            } else {
                Err("export_registry_unreadable".to_string())
            }
        })
        .await
        .err()
        .expect("the scan refuses after its walk");
    assert_eq!(reads.get(), 2);
    assert!(
        error.to_string().contains("export_registry_unreadable"),
        "{error}"
    );
}

/// bridge#833: an export recorded while the scan walks the folder (Bridge
/// records it before writing it) is caught by the registry's second read.
#[tokio::test]
async fn an_export_recorded_during_the_walk_is_still_skipped() {
    let export = b"a statement written during the scan".to_vec();
    let selected = tempfile::tempdir().expect("selected directory");
    std::fs::write(selected.path().join("statement-late.xlsx"), &export).unwrap();
    let selections =
        authorize_selected_paths(vec![selected.path().to_path_buf()]).expect("authorize directory");
    let reads = std::cell::Cell::new(0);
    let response =
        super::scan_documents_with(scan_everything(selections[0].selection_id.clone()), || {
            reads.set(reads.get() + 1);
            Ok(if reads.get() == 1 {
                std::collections::HashSet::new()
            } else {
                std::collections::HashSet::from([crate::export_registry::content_sha256(&export)])
            })
        })
        .await
        .expect("scan directory");
    assert_eq!(reads.get(), 2);
    assert!(response.files.is_empty(), "{}", response.files.len());
    assert_eq!(
        response
            .skipped
            .iter()
            .map(|file| (file.path.as_str(), file.reason.as_str()))
            .collect::<Vec<_>>(),
        [("statement-late.xlsx", super::BRIDGE_EXPORT_SKIPPED)]
    );
}
