use super::*;

#[test]
fn exact_decimals_round_trip_without_float_conversion() {
    for value in ["0", "0.00", "12345678901234567890.0001", "-1180.00"] {
        let parsed = ExactDecimal::parse(value).expect("valid exact decimal");
        assert_eq!(parsed.as_str(), value);
    }
}

#[test]
fn exact_decimals_reject_ambiguous_or_non_numeric_values() {
    for value in ["", "-", ".1", "1.", "+1", "1,000", "NaN", "1.2.3"] {
        assert!(
            ExactDecimal::parse(value).is_err(),
            "unexpectedly accepted {value}"
        );
    }
    assert!(ExactDecimal::parse("9".repeat(MAX_EXACT_DECIMAL_BYTES + 1)).is_err());
}

#[test]
fn exact_decimal_arithmetic_is_canonical_checked_and_ordered_by_magnitude() {
    let left = ExactDecimal::parse("100.00").unwrap();
    let right = ExactDecimal::parse("-40.0").unwrap();
    assert_eq!(left.checked_add(&right).unwrap().as_str(), "60");
    assert_eq!(left.checked_subtract(&right).unwrap().as_str(), "140");
    assert!(right.is_negative());
    assert!(!right.is_zero());
    assert_eq!(right.abs().unwrap().as_str(), "40.0");
    assert_eq!(
        ExactDecimal::parse("-100.001")
            .unwrap()
            .cmp_magnitude(&left),
        std::cmp::Ordering::Greater
    );
}

#[test]
fn legacy_capability_profiles_deserialize_without_inventing_feature_evidence() {
    let profile: CapabilityProfile = serde_json::from_str(
        r#"{
                "profile_version": 1,
                "product": "Unknown",
                "release": null,
                "mode": null,
                "transports": {},
                "packs": {}
            }"#,
    )
    .expect("deserialize legacy profile");

    assert_eq!(profile.profile_version, 1);
    assert!(profile.features.is_empty());
}

fn record_evidence(object_type: &str, source_id: &str) -> SourceRecordEvidence {
    let source_id = SourceRecordId::parse(source_id).unwrap();
    SourceRecordEvidence {
        object_type: CanonicalText::parse(object_type).unwrap(),
        source_id: source_id.clone(),
        identity_kind: SourceIdentityKind::Guid,
        observed_identities: ObservedSourceIdentities {
            guid: Some(source_id),
            ..Default::default()
        },
        raw_source_sha256: RawSourceSha256::parse("a".repeat(64)).unwrap(),
        alter_id: Some(SourceAlterId::parse("alter:42").unwrap()),
    }
}

#[test]
fn record_provenance_must_bind_one_to_one_to_canonical_records() {
    let batch = PackBatch::Inventory(InventoryBatch {
        stock_items: vec![StockItemRecord {
            source_id: SourceRecordId::parse("stock:1").unwrap(),
            name: CanonicalText::parse("Synthetic Item").unwrap(),
            base_unit: CanonicalText::parse("nos").unwrap(),
        }],
        godowns: Vec::new(),
        inventory_entries: Vec::new(),
    });
    let valid = CanonicalPackWindow {
        batch: batch.clone(),
        source_counts: None,
        record_evidence: Some(vec![record_evidence("stock_item", "stock:1")]),
    };
    valid.validate_record_evidence_binding().unwrap();

    let missing = CanonicalPackWindow {
        batch: batch.clone(),
        source_counts: None,
        record_evidence: Some(vec![record_evidence("godown", "stock:1")]),
    };
    assert!(matches!(
        missing.validate_record_evidence_binding(),
        Err(TallyError::InvalidData { code })
            if code == "source_record_evidence_binding_mismatch"
    ));

    let duplicate = CanonicalPackWindow {
        batch,
        source_counts: None,
        record_evidence: Some(vec![
            record_evidence("stock_item", "stock:1"),
            record_evidence("stock_item", "stock:1"),
        ]),
    };
    assert!(matches!(
        duplicate.validate_record_evidence_binding(),
        Err(TallyError::InvalidData { code })
            if code == "source_record_evidence_duplicate_record"
    ));
}

#[test]
fn record_provenance_rejects_noncanonical_hashes_and_alter_ids() {
    assert!(RawSourceSha256::parse("A".repeat(64)).is_err());
    assert!(RawSourceSha256::parse("a".repeat(63)).is_err());
    assert!(SourceAlterId::parse("contains whitespace").is_err());
    assert!(SourceAlterId::parse("alter:42").is_ok());
}

#[test]
fn a_diagnostic_never_changes_a_batch_s_serialized_bytes() {
    let plain = CoreAccountingBatch::default();
    let mut diagnosed = CoreAccountingBatch::default();
    diagnosed
        .foreign_master_text_diagnostics
        .push(ForeignMasterTextDiagnostic {
            object_type: "ledger".to_string(),
            source_id: "src-1".to_string(),
            stored_name: "bad\u{7}name".to_string(),
            likely_intended_spelling: None,
        });
    assert!(diagnosed.has_foreign_master_text_diagnostics());

    assert_eq!(
        serde_json::to_vec(&diagnosed).expect("serialize diagnosed batch"),
        serde_json::to_vec(&plain).expect("serialize plain batch"),
        "a diagnostic must never change the versioned batch payload"
    );
    assert_eq!(
        serde_json::to_vec(&PackBatch::CoreAccounting(diagnosed)).expect("serialize wrapped batch"),
        serde_json::to_vec(&PackBatch::CoreAccounting(plain)).expect("serialize wrapped plain"),
    );
}
