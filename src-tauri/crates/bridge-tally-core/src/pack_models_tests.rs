use super::*;
use crate::{CoreAccountingBatch, GroupRecord};

fn text(value: &str) -> CanonicalText {
    CanonicalText::parse(value).expect("valid canonical text")
}

#[test]
fn typed_ids_and_dates_reject_ambiguous_values_at_construction_and_deserialization() {
    for value in ["", " leading", "trailing ", "line\nbreak"] {
        assert!(SourceRecordId::parse(value).is_err(), "accepted {value:?}");
    }
    for value in ["20260229", "20261301", "20260001", "00000101", "2026-01-01"] {
        assert!(TallyDate::parse(value).is_err(), "accepted {value}");
    }
    assert!(TallyDate::parse("20240229").is_ok());
    assert!(serde_json::from_str::<TallyDate>(r#""20260229""#).is_err());
    assert_eq!(
        TallyDate::parse("20240228")
            .unwrap()
            .next_day()
            .unwrap()
            .as_str(),
        "20240229"
    );
    assert_eq!(
        TallyDate::parse("20241231")
            .unwrap()
            .next_day()
            .unwrap()
            .as_str(),
        "20250101"
    );
    assert!(TallyDate::parse("99991231").unwrap().next_day().is_err());
    for value in ["", " leading", "trailing ", "line\nbreak"] {
        assert!(CanonicalText::parse(value).is_err(), "accepted {value:?}");
    }
}

#[test]
fn source_counts_are_optional_explicit_evidence_never_derived_from_records() {
    let batch = CoreAccountingBatch {
        groups: vec![GroupRecord {
            source_id: "group:1".to_string(),
            name: "Synthetic Group".to_string(),
            parent_source_id: None,
        }],
        ..CoreAccountingBatch::default()
    };
    let absent = PackWindow::without_source_count_evidence(batch.clone());
    assert_eq!(absent.source_counts, None);
    absent
        .validate_source_count_evidence()
        .expect("absent source evidence is honest");

    let observed = PackWindow {
        batch,
        source_counts: Some(vec![SourceReportedCountEvidence {
            object_type: text("group"),
            query_profile: text("groups-v1"),
            source_scope_fingerprint: text("scope-sha256:synthetic"),
            source_count_scope: SourceCountScope::Complete,
            // Deliberately differs from the parsed Vec length. The model
            // stores the source claim without inventing reconciliation.
            source_reported_count: 37,
        }]),
        record_evidence: None,
    };
    observed
        .validate_source_count_evidence()
        .expect("valid explicit evidence");
    assert_eq!(
        observed.source_counts.as_ref().unwrap()[0].source_reported_count,
        37
    );
}

#[test]
fn source_count_fingerprint_binds_exact_versioned_scope() {
    let descriptor = SourceCountScopeDescriptor {
        source_identity: SourceIdentity {
            bridge_source_lineage: "lineage:synthetic".to_string(),
            company_guid: "company:synthetic".to_string(),
            observed_fingerprint: "fingerprint:synthetic".to_string(),
        },
        pack: CapabilityPackId::Inventory,
        pack_schema_version: PackSchemaVersion { major: 1, minor: 0 },
        object_type: text("stock_item"),
        query_profile: text("inventory-v1"),
        filters_sha256: text(&"a".repeat(64)),
        window: None,
    };
    let fingerprint = source_count_scope_fingerprint(&descriptor, SourceCountScope::Complete)
        .expect("valid complete scope fingerprint");
    assert_eq!(fingerprint.as_str().len(), 64);
    assert_eq!(
        fingerprint,
        source_count_scope_fingerprint(&descriptor, SourceCountScope::Complete).unwrap()
    );

    let evidence = SourceReportedCountEvidence {
        object_type: descriptor.object_type.clone(),
        query_profile: descriptor.query_profile.clone(),
        source_scope_fingerprint: fingerprint,
        source_count_scope: SourceCountScope::Complete,
        source_reported_count: 0,
    };
    assert!(evidence.matches_scope_descriptor(&descriptor).unwrap());

    let mut drifted = descriptor.clone();
    drifted.query_profile = text("inventory-v2");
    assert!(!evidence.matches_scope_descriptor(&drifted).unwrap());
    assert!(source_count_scope_fingerprint(&descriptor, SourceCountScope::Window).is_err());
}

#[test]
fn window_count_fingerprint_requires_exact_valid_window() {
    let mut descriptor = SourceCountScopeDescriptor {
        source_identity: SourceIdentity {
            bridge_source_lineage: "lineage:synthetic".to_string(),
            company_guid: "company:synthetic".to_string(),
            observed_fingerprint: "fingerprint:synthetic".to_string(),
        },
        pack: CapabilityPackId::CoreAccounting,
        pack_schema_version: PackSchemaVersion { major: 1, minor: 0 },
        object_type: text("voucher"),
        query_profile: text("voucher-v1"),
        filters_sha256: text(&"b".repeat(64)),
        window: Some(ReadWindow {
            from_yyyymmdd: "20260101".to_string(),
            to_yyyymmdd: "20260131".to_string(),
        }),
    };
    assert!(source_count_scope_fingerprint(&descriptor, SourceCountScope::Window).is_ok());
    assert!(source_count_scope_fingerprint(&descriptor, SourceCountScope::Complete).is_err());

    descriptor.window.as_mut().unwrap().from_yyyymmdd = "20260201".to_string();
    assert!(source_count_scope_fingerprint(&descriptor, SourceCountScope::Window).is_err());
}

#[test]
fn empty_or_duplicate_source_count_evidence_fails_closed() {
    let empty = PackWindow {
        batch: CoreAccountingBatch::default(),
        source_counts: Some(Vec::new()),
        record_evidence: None,
    };
    assert!(matches!(
        empty.validate_source_count_evidence(),
        Err(TallyError::InvalidData { code }) if code == "source_count_evidence_empty"
    ));

    let count = SourceReportedCountEvidence {
        object_type: text("voucher"),
        query_profile: text("voucher-v1"),
        source_scope_fingerprint: text("scope-sha256:synthetic"),
        source_count_scope: SourceCountScope::Complete,
        source_reported_count: 2,
    };
    let duplicate = PackWindow {
        batch: CoreAccountingBatch::default(),
        source_counts: Some(vec![count.clone(), count]),
        record_evidence: None,
    };
    assert!(matches!(
        duplicate.validate_source_count_evidence(),
        Err(TallyError::InvalidData { code }) if code == "source_count_evidence_duplicate_scope"
    ));
}
