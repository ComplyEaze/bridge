use super::*;

#[tokio::test]
async fn independent_builds_reuse_labels_without_replacing_retained_batches() {
    let plans = qualified_import_cycle_plans()[..32].to_vec();
    let simulator = SequenceSimulator::spawn([plans.clone(), plans].concat()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().into(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let legacy: ImportLedgerLine = serde_json::from_value(legacy_record()).unwrap();
    server.append_import_ledger(&legacy).unwrap();
    let original_journal = fs::read(directory.path().join("agent-import-ledger.jsonl")).unwrap();
    let input = serde_json::to_value(captured_catalogue_payload()).unwrap();
    let mut built = Vec::new();
    for _ in 0..2 {
        let response = server
            .call_tool_response("build_import_xml", input.clone())
            .await;
        assert_eq!(response.value["isError"], false, "{}", response.value);
        let result = &response.value["structuredContent"]["result"];
        let batch_id = result["batch_id"].as_str().unwrap();
        let snapshot = server.latest_import_snapshot(batch_id).unwrap().unwrap();
        let xml = fs::read(result["path"].as_str().unwrap()).unwrap();
        assert_eq!(sha256_hex(&xml), result["sha256"]);
        assert_eq!(snapshot.batch.txn_ids, ["txn-001", "txn-002"]);
        assert_eq!(
            snapshot.batch.identity_scheme,
            Some(ImportIdentityScheme::BatchV1)
        );
        built.push((snapshot.batch, xml));
    }
    assert_ne!(built[0].0.batch_id, built[1].0.batch_id);
    assert_ne!(built[0].1, built[1].1);
    for voucher in &built[0].0.vouchers {
        assert_ne!(
            built[0].0.attribution_tag(voucher),
            built[1].0.attribution_tag(voucher)
        );
    }
    for (line, xml) in &built {
        assert_eq!(
            fs::read(
                directory
                    .path()
                    .join("imports")
                    .join(format!("{}.xml", line.batch_id))
            )
            .unwrap(),
            *xml
        );
    }
    let journal = fs::read(directory.path().join("agent-import-ledger.jsonl")).unwrap();
    assert!(journal.starts_with(&original_journal));
    assert_eq!(server.import_ledger().unwrap().len(), 3);
    assert_eq!(
        server
            .latest_import_snapshot(&legacy.batch_id)
            .unwrap()
            .unwrap()
            .batch
            .attribution_tag(&legacy.vouchers[0]),
        "txn-001"
    );
    let mut duplicate = captured_catalogue_payload();
    duplicate.vouchers[1].bridge_txn_id = duplicate.vouchers[0].bridge_txn_id.clone();
    let response = server
        .call_tool_response("build_import_xml", serde_json::to_value(duplicate).unwrap())
        .await;
    assert_eq!(
        response.value["structuredContent"]["result"]["error"]["code"],
        "bridge_txn_id_invalid_or_duplicate"
    );
    assert_eq!(
        fs::read(directory.path().join("agent-import-ledger.jsonl")).unwrap(),
        journal
    );
    assert_eq!(simulator.finish().unwrap().len(), 64);
}

fn legacy_record() -> Value {
    json!({
        "batch_id":"batch-render", "company_guid":GUID, "company":null,
        "txn_ids":["txn-001"], "date_from":"20260901", "date_to":"20260901",
        "sha256":"hash", "built_at":"2026-09-07T00:00:00Z", "status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water", "value":10, "master_value":10},
        "vouchers":[admitted_payload().vouchers.remove(0)]
    })
}

#[test]
fn batch_identity_is_deterministic_uuid_v8_and_separates_tuple_components() {
    let identity = import_identity("batch-render", "txn-001");
    assert_eq!(identity.to_string(), "e6982c7e-4bb4-8a1f-8364-da11832d928a");
    assert_eq!(identity, import_identity("batch-render", "txn-001"));
    assert_eq!(identity.get_version_num(), 8);
    assert_eq!(identity.get_variant(), uuid::Variant::RFC4122);
    assert_ne!(identity, import_identity("another-batch", "txn-001"));
    assert_ne!(identity, import_identity("batch-render", "txn-002"));
    assert_ne!(import_identity("a:b", "c"), import_identity("a", "b:c"));
}

#[test]
fn historical_journal_without_scheme_keeps_raw_attribution_and_serialization() {
    let original = legacy_record();
    assert!(original.get("identity_scheme").is_none());
    let snapshots = ledger::parse_snapshots(&format!("{original}\n")).unwrap();
    let line = &snapshots[0].batch;
    assert!(line.identity_scheme.is_none());
    assert_eq!(line.attribution_tag(&line.vouchers[0]), "txn-001");
    assert_eq!(serde_json::to_value(line).unwrap(), original);
}

#[test]
fn namespaced_journal_roundtrips_derived_attribution_and_rejects_unknown_scheme() {
    let mut value = legacy_record();
    value["identity_scheme"] = json!("batch_v1");
    let snapshots = ledger::parse_snapshots(&format!("{value}\n")).unwrap();
    let line = &snapshots[0].batch;
    assert!(matches!(
        line.identity_scheme,
        Some(ImportIdentityScheme::BatchV1)
    ));
    assert_eq!(
        line.attribution_tag(&line.vouchers[0]),
        import_identity(&line.batch_id, "txn-001").to_string()
    );
    assert_eq!(line.vouchers[0].bridge_txn_id, "txn-001");
    assert_eq!(serde_json::to_value(line).unwrap(), value);
    for unsupported in [json!("batch_v2"), json!("BatchV1"), json!(1)] {
        value["identity_scheme"] = unsupported;
        assert_eq!(
            ledger::parse_snapshots(&format!("{value}\n")).err(),
            Some("import_ledger_invalid".into())
        );
    }
}

#[test]
fn renderer_uses_one_derived_identity_for_remote_id_and_marker() {
    let mut input = admitted_payload();
    input.vouchers[0].voucher_number = Some("CLIENT-42".into());
    let mut renderings = Vec::new();
    for batch_id in ["batch-one", "batch-two"] {
        let rendered = render_import_xml("Book", &input.vouchers, batch_id);
        let mut reader = quick_xml::Reader::from_str(&rendered);
        let mut remote_ids = Vec::new();
        let mut narrations = Vec::new();
        loop {
            match reader.read_event().unwrap() {
                quick_xml::events::Event::Start(tag) if tag.name().as_ref() == b"VOUCHER" => {
                    for attribute in tag.attributes() {
                        let attribute = attribute.unwrap();
                        if attribute.key.as_ref() == b"REMOTEID" {
                            remote_ids.push(
                                attribute
                                    .decoded_and_normalized_value(
                                        quick_xml::XmlVersion::Implicit1_0,
                                        reader.decoder(),
                                    )
                                    .unwrap()
                                    .into_owned(),
                            );
                        }
                    }
                }
                quick_xml::events::Event::Start(tag) if tag.name().as_ref() == b"NARRATION" => {
                    narrations.push(
                        reader
                            .read_text(tag.name())
                            .unwrap()
                            .decode()
                            .unwrap()
                            .into_owned(),
                    );
                }
                quick_xml::events::Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(remote_ids.len(), input.vouchers.len());
        assert_eq!(narrations.len(), input.vouchers.len());
        for ((remote_id, narration), voucher) in
            remote_ids.iter().zip(narrations).zip(&input.vouchers)
        {
            let expected = import_identity(batch_id, &voucher.bridge_txn_id).to_string();
            assert_eq!(remote_id, &expected);
            assert!(narration.ends_with(&format!("[BRIDGE:{expected}]")));
            assert!(!rendered.contains(&voucher.bridge_txn_id));
        }
        assert_eq!(
            rendered,
            render_import_xml("Book", &input.vouchers, batch_id)
        );
        renderings.push(remote_ids);
    }
    assert_ne!(renderings[0], renderings[1]);
}

#[test]
fn captured_namespaced_journal_is_attributed_only_to_its_recorded_batch() {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-namespaced-journal.utf16le.xml"
    );
    assert_eq!(bytes.len(), 7990);
    assert_eq!(
        sha256_hex(bytes),
        "d4ee5c73377cbe7be6de9995bdf15a6bde58206c5abcb1c4c08751922685dedf"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    // Exact synthetic build record corresponding to the unchanged live capture.
    let mut line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-6c79872c-aab6-4be5-a181-18182c8148be",
        "identity_scheme":"batch_v1",
        "company_guid":CAPTURED_GUID,
        "company":{"name":"WR2 Unicode Lab", "guid":CAPTURED_GUID,
            "company_number":"100004", "books_from":"20260401"},
        "txn_ids":["BRIDGE_MCP_LIVE_20260906_A1"],
        "date_from":"20260907", "date_to":"20260907",
        "sha256":"e39eb3c0bfe53144bdd9c0f4afcb88c3d63a2050214233ee77465d42a54245ef",
        "built_at":"2026-09-06T21:40:26.641Z", "status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water", "value":8, "master_value":219},
        "vouchers":[{"bridge_txn_id":"BRIDGE_MCP_LIVE_20260906_A1", "date":"20260907",
            "voucher_type":"Journal", "narration":"Bridge MCP batch namespace qualification",
            "reference":null, "voucher_number":null,
            "entries":[{"ledger":"Bridge Nested Debtor WR4", "amount":"12.61", "side":"Dr"},
                {"ledger":"Cash", "amount":"12.61", "side":"Cr"}]}]
    }))
    .unwrap();
    assert_eq!(
        sha256_hex(render_import_xml("WR2 Unicode Lab", &line.vouchers, &line.batch_id).as_bytes()),
        line.sha256
    );
    let source = parse_import_vouchers(&xml, CAPTURED_GUID).unwrap();
    assert_eq!(source.rows.len(), 1);
    let result = verify_batch(&line, &source, Attribution::Tag).unwrap();
    assert_eq!(result["counts"]["posted_verified"], 1);
    assert_eq!(result["counts"]["matching_content_observed"], 0);
    assert_eq!(result["duplicates"], json!([]));
    assert_eq!(result["unrelated_duplicates_in_window"], json!([]));
    let row = &result["vouchers"][0];
    assert_eq!(row["bridge_txn_id"], "BRIDGE_MCP_LIVE_20260906_A1");
    assert_eq!(row["marker"], "narration_tag");
    assert_eq!(row["guid"], format!("{CAPTURED_GUID}-00000005"));
    assert_eq!(row["master_id"], "5");
    assert_eq!(row["voucher_number"], "2");
    assert_eq!(row["alter_id"], 10);

    // Keep the actual observed source and all expected accounting fields intact.
    // A different batch must not inherit this posting's attribution.
    line.batch_id = "bridge-00000000-0000-4000-8000-000000000002".into();
    let other = verify_batch(&line, &source, Attribution::Tag).unwrap();
    assert_eq!(other["counts"]["posted_verified"], 0);
    assert_eq!(other["counts"]["matching_content_observed"], 1);
    assert_eq!(other["vouchers"][0]["marker"], "accounting_fingerprint");
    assert_eq!(other["vouchers"][0]["attribution"], "not_established");
}

/// The captured namespaced Journal and the build record it came from, for the
/// post-span binding cases below.
fn captured_journal_and_line() -> (ImportReadSource, ImportLedgerLine) {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-namespaced-journal.utf16le.xml"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-6c79872c-aab6-4be5-a181-18182c8148be",
        "identity_scheme":"batch_v1",
        "company_guid":CAPTURED_GUID,
        "company":{"name":"WR2 Unicode Lab", "guid":CAPTURED_GUID,
            "company_number":"100004", "books_from":"20260401"},
        "txn_ids":["BRIDGE_MCP_LIVE_20260906_A1"],
        "date_from":"20260907", "date_to":"20260907",
        "sha256":"e39eb3c0bfe53144bdd9c0f4afcb88c3d63a2050214233ee77465d42a54245ef",
        "built_at":"2026-09-06T21:40:26.641Z", "status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water", "value":8, "master_value":219},
        "vouchers":[{"bridge_txn_id":"BRIDGE_MCP_LIVE_20260906_A1", "date":"20260907",
            "voucher_type":"Journal", "narration":"Bridge MCP batch namespace qualification",
            "reference":null, "voucher_number":null,
            "entries":[{"ledger":"Bridge Nested Debtor WR4", "amount":"12.61", "side":"Dr"},
                {"ledger":"Cash", "amount":"12.61", "side":"Cr"}]}]
    }))
    .unwrap();
    (parse_import_vouchers(&xml, CAPTURED_GUID).unwrap(), line)
}

fn binding(guid_suffix: &str, master_id: u64) -> Vec<span_identity::PostedVoucherIdentity> {
    vec![span_identity::PostedVoucherIdentity {
        bridge_txn_id: "BRIDGE_MCP_LIVE_20260906_A1".into(),
        guid: format!("{CAPTURED_GUID}-{guid_suffix}"),
        master_id,
    }]
}

/// A bound voucher is found by its GUID first, ahead of the narration tag the
/// capture also carries.
#[test]
fn a_bound_voucher_is_attributed_by_its_guid_first() {
    let (source, line) = captured_journal_and_line();
    let result = verify_batch(
        &line,
        &source,
        Attribution::Span(Some(&binding("00000005", 5))),
    )
    .unwrap();
    assert_eq!(result["counts"]["posted_verified"], 1);
    assert_eq!(result["vouchers"][0]["marker"], "post_span_binding");
    assert_eq!(result["duplicates"], json!([]));
    // A binding whose GUID the window does not hold is `bound_not_in_window`,
    // even beside a matching tag: the binding ranks first and a tag never
    // overrides it, so the disagreement stays unverified (loud).
    let elsewhere = verify_batch(
        &line,
        &source,
        Attribution::Span(Some(&binding("00000099", 153))),
    )
    .unwrap();
    assert_eq!(elsewhere["vouchers"][0]["status"], "bound_not_in_window");
}

/// The GUID found the row, but its MasterID is not the one bound: refused,
/// never attributed (the GUID is the company GUID plus the MasterID).
#[test]
fn a_bound_guid_on_another_master_id_is_not_attributed() {
    let (source, line) = captured_journal_and_line();
    let result = verify_batch(
        &line,
        &source,
        Attribution::Span(Some(&binding("00000005", 6))),
    )
    .unwrap();
    assert_eq!(result["counts"]["not_attributable"], 1);
    assert_eq!(result["counts"]["posted_verified"], 0);
    assert_eq!(result["vouchers"][0]["reason"], "bound_master_id_changed");
}

/// A bound voucher the window does not hold is never `not_found`; after a
/// rollback it is `book_rolled_back`, and so is any voucher not found.
#[test]
fn a_bound_voucher_absent_from_the_window_is_never_not_found() {
    let (_, line) = captured_journal_and_line();
    let empty = ImportReadSource::admit(Vec::new()).unwrap();
    let mut result = verify_batch(
        &line,
        &empty,
        Attribution::Span(Some(&binding("00000005", 5))),
    )
    .unwrap();
    assert_eq!(result["counts"]["bound_not_in_window"], 1);
    assert_eq!(result["counts"]["not_found"], 0);
    verification::mark_book_rolled_back(&mut result);
    assert_eq!(result["counts"]["bound_not_in_window"], 0);
    assert_eq!(result["counts"]["book_rolled_back"], 1);
    assert_eq!(result["vouchers"][0]["status"], "book_rolled_back");
    // An unbound voucher not found is rewritten the same way.
    let mut unbound = verify_batch(&line, &empty, Attribution::Span(None)).unwrap();
    assert_eq!(unbound["counts"]["not_found"], 1);
    verification::mark_book_rolled_back(&mut unbound);
    assert_eq!(unbound["counts"]["not_found"], 0);
    assert_eq!(unbound["vouchers"][0]["status"], "book_rolled_back");
}

/// A bound voucher edited in Tally after its post (an amount changed) is found
/// by its GUID and reads `posted_divergent`, never `posted_verified`: the
/// binding names the object, and the content is still compared.
#[test]
fn a_bound_voucher_whose_amount_changed_reads_divergent() {
    let (source, line) = captured_journal_and_line();
    let mut rows = source.rows.clone();
    for entry in &mut rows[0].entries {
        entry.amount = if entry.amount.starts_with('-') {
            "-13.61".into()
        } else {
            "13.61".into()
        };
    }
    let edited = ImportReadSource::admit(rows).unwrap();
    let result = verify_batch(
        &line,
        &edited,
        Attribution::Span(Some(&binding("00000005", 5))),
    )
    .unwrap();
    assert_eq!(
        result["vouchers"][0]["marker"], "post_span_binding",
        "{result}"
    );
    assert_eq!(
        result["vouchers"][0]["status"], "posted_divergent",
        "{result}"
    );
    assert_eq!(result["counts"]["posted_verified"], 0, "{result}");
    assert_eq!(result["counts"]["posted_divergent"], 1, "{result}");
}

/// An untagged native post that was not bound: a row carrying the batch's tag
/// is a hand import of the batch's file, not a voucher this POST created, so
/// the tag attributes nothing and the row is matched by content only. The same
/// row on the tag basis reads verified.
#[test]
fn a_tag_never_attributes_an_untagged_native_post() {
    let (source, line) = captured_journal_and_line();
    let tagged = verify_batch(&line, &source, Attribution::Tag).unwrap();
    assert_eq!(tagged["counts"]["posted_verified"], 1, "{tagged}");
    let native = verify_batch(&line, &source, Attribution::Span(None)).unwrap();
    assert_eq!(native["counts"]["posted_verified"], 0, "{native}");
    assert_eq!(native["counts"]["matching_content_observed"], 1, "{native}");
    assert_eq!(native["vouchers"][0]["marker"], "accounting_fingerprint");
}

/// A hand-imported copy beside a bound voucher: the bound row (untagged, as
/// the native post wrote it) verifies by its GUID, and the tagged copy is
/// reported as a duplicate, never ignored and never counted.
#[test]
fn a_tagged_copy_beside_a_bound_voucher_is_a_duplicate() {
    let (source, line) = captured_journal_and_line();
    let mut native = source.rows[0].clone();
    native.narration = line.vouchers[0].narration.clone();
    let mut copy = source.rows[0].clone();
    copy.guid = Some(format!("{CAPTURED_GUID}-00000099"));
    copy.master_id = Some("153".into());
    copy.alter_id = Some(11);
    copy.remote_id = copy.guid.clone();
    let both = ImportReadSource::admit(vec![native, copy]).unwrap();
    let result = verify_batch(
        &line,
        &both,
        Attribution::Span(Some(&binding("00000005", 5))),
    )
    .unwrap();
    assert_eq!(
        result["vouchers"][0]["marker"], "post_span_binding",
        "{result}"
    );
    assert_eq!(result["counts"]["posted_verified"], 1, "{result}");
    assert_eq!(result["counts"]["matching_content_observed"], 0, "{result}");
    assert_eq!(
        result["duplicates"].as_array().map(Vec::len),
        Some(1),
        "{result}"
    );
}
