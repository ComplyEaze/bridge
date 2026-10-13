use super::*;
use bridge_tally_core::{
    CanonicalPackWindow, CanonicalText, CapabilityPackId, CompanyRef, LedgerEntryPolarity,
    ObservedSourceIdentities, PackBatch, PackSchemaVersion, ReadWindow, RequestContext,
    SourceIdentity, SourceIdentityKind, TallyError,
};
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_native_group_source_records_with_evidence,
    parse_native_ledger_source_records_with_evidence,
    parse_native_voucher_source_records_with_evidence,
    parse_native_voucher_type_source_records_with_evidence, CompanyContextEvidence,
    ExpectedTallyTextEncoding, ExportEvidence, ParsedExport, ParsedSourceIdentities,
    ParsedSourceIdentityKind, ParsedSourceRecord, PartyLedgerMasterFieldObservation, TallyLedger,
    TallyLedgerEntry, TallyNamedMaster, TallyVoucher, TALLY_SANITIZED_ROOT_MARKER,
};

fn context() -> RequestContext {
    RequestContext {
        run_id: "synthetic-run".to_string(),
        company: CompanyRef {
            identity: SourceIdentity {
                bridge_source_lineage: "synthetic-lineage".to_string(),
                company_guid: "synthetic-company-guid".to_string(),
                observed_fingerprint: "synthetic-observation".to_string(),
            },
            display_name: "BRIDGE SYNTHETIC BOOK".to_string(),
        },
        pack: CapabilityPackId::CoreAccounting,
        schema_version: PackSchemaVersion { major: 1, minor: 0 },
        window: ReadWindow {
            from_yyyymmdd: "20260701".to_string(),
            to_yyyymmdd: "20260731".to_string(),
        },
        query_profile: CanonicalText::parse("core_accounting_v1").unwrap(),
        filters_sha256: CanonicalText::parse("0".repeat(64)).unwrap(),
    }
}

/// Opaque stand-in for the digest of the exact source fragment a row was read
/// from. `build_core_window` only requires it to be a well-formed SHA-256 and
/// keeps it as provenance; deriving it from a label keeps two different rows
/// from sharing one.
fn fixture_digest(label: &str) -> String {
    Sha256::digest(label.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// A parsed export with the count evidence the shipped native parsers give:
/// the count of rows read (`observed_record_count`), and no count Tally
/// reported (`source_record_count`). `build_core_window` takes the observed
/// count first (and the source count only when none was observed), and builds
/// `source_counts` from the source-reported counts alone.
fn export_of<T>(records: Vec<ParsedSourceRecord<T>>) -> ParsedExport<ParsedSourceRecord<T>> {
    let count = records.len() as u64;
    ParsedExport {
        records,
        evidence: ExportEvidence {
            company_context: Some(CompanyContextEvidence {
                name: Some("BRIDGE SYNTHETIC BOOK".to_string()),
                guid: Some("synthetic-company-guid".to_string()),
                ..CompanyContextEvidence::default()
            }),
            observed_record_count: Some(count),
            identified_record_count: count,
            ..ExportEvidence::default()
        },
    }
}

fn source_record<T: std::fmt::Debug>(
    record: T,
    guid: &str,
    remote_id: Option<&str>,
    master_id: &str,
    alter_id: &str,
) -> ParsedSourceRecord<T> {
    let raw_source_sha256 = fixture_digest(&format!("{guid}|{master_id}|{alter_id}|{record:?}"));
    ParsedSourceRecord {
        record,
        source_id: Some(guid.to_string()),
        identity_kind: Some(ParsedSourceIdentityKind::Guid),
        identities: ParsedSourceIdentities {
            guid: Some(guid.to_string()),
            remote_id: remote_id.map(str::to_string),
            master_id: Some(master_id.to_string()),
        },
        alter_id: Some(alter_id.to_string()),
        raw_source_sha256,
    }
}

/// The same export as the Bridge-defined export shapes carried it: the row
/// count also set as a count the source reported. Only these shapes reach the
/// `source_counts` branch of `build_core_window`; issue #1570 deletes them.
fn reported_by_the_source<T>(
    mut export: ParsedExport<ParsedSourceRecord<T>>,
) -> ParsedExport<ParsedSourceRecord<T>> {
    export.evidence.source_record_count = export.evidence.observed_record_count;
    export
}

fn groups() -> ParsedExport<ParsedSourceRecord<TallyNamedMaster>> {
    export_of(vec![source_record(
        TallyNamedMaster {
            name: "Assets".to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned(format!(
                "{TALLY_SANITIZED_ROOT_MARKER} Primary"
            )),
            reserved_name: None,
        },
        "group-guid",
        None,
        "1",
        "5",
    )])
}

fn ledger(
    name: &str,
    guid: &str,
    remote_id: Option<&str>,
    master_id: &str,
    alter_id: &str,
) -> ParsedSourceRecord<TallyLedger> {
    source_record(
        TallyLedger {
            name: name.to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned("Assets".to_string()),
            party_gstin: PartyLedgerMasterFieldObservation::NotObserved,
            opening_balance: Some("0".to_string()),
        },
        guid,
        remote_id,
        master_id,
        alter_id,
    )
}

fn ledgers_and_vouchers(
    cash_name: &str,
    entry_ledger_name: &str,
) -> (
    ParsedExport<ParsedSourceRecord<TallyLedger>>,
    ParsedExport<ParsedSourceRecord<TallyVoucher>>,
) {
    let ledgers = export_of(vec![
        ledger(cash_name, "ledger-cash", Some("cash-remote"), "2", "6"),
        ledger("Sales", "ledger-sales", None, "3", "7"),
    ]);
    (
        ledgers,
        voucher_with_entries(entry_ledger_name, "Sales", true, false),
    )
}

fn voucher_types() -> ParsedExport<ParsedSourceRecord<TallyNamedMaster>> {
    export_of(vec![source_record(
        TallyNamedMaster {
            name: "Receipt".to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned("Receipt".to_string()),
            reserved_name: None,
        },
        "voucher-type-guid",
        None,
        "8",
        "9",
    )])
}

fn valid_window() -> CanonicalPackWindow {
    let (ledgers, vouchers) = ledgers_and_vouchers("Cash", "Cash");
    build_core_window(&context(), groups(), ledgers, voucher_types(), vouchers).unwrap()
}

/// [`valid_window`] from exports that also carry source-reported counts.
fn valid_window_with_source_reported_counts() -> CanonicalPackWindow {
    let (ledgers, vouchers) = ledgers_and_vouchers("Cash", "Cash");
    build_core_window(
        &context(),
        reported_by_the_source(groups()),
        reported_by_the_source(ledgers),
        reported_by_the_source(voucher_types()),
        reported_by_the_source(vouchers),
    )
    .unwrap()
}

/// A balanced two-entry Receipt whose entry flags are given by the caller,
/// so the two halves of the bridge#392 contract are built from one fixture
/// and can be read as a pair. `Cash -100.00` wants `Yes` and `Sales 100.00`
/// wants `No`; passing anything else inverts that leg.
use bridge_tally_core::reconciliation;

fn voucher_with_entry_flags(
    cash_flag: &str,
    sales_flag: &str,
) -> ParsedExport<ParsedSourceRecord<TallyVoucher>> {
    let deemed_positive = |flag: &str| match flag {
        "Yes" => true,
        "No" => false,
        other => panic!("not a Tally logical: {other}"),
    };
    voucher_with_entries(
        "Cash",
        "Sales",
        deemed_positive(cash_flag),
        deemed_positive(sales_flag),
    )
}

/// The balanced two-entry Receipt `SYN-1`: the first entry debits
/// `first_ledger` for 100.00, the second credits `second_ledger`.
fn voucher_with_entries(
    first_ledger: &str,
    second_ledger: &str,
    first_deemed_positive: bool,
    second_deemed_positive: bool,
) -> ParsedExport<ParsedSourceRecord<TallyVoucher>> {
    let entry = |entry_index: u64, ledger_name: &str, amount: &str, is_deemed_positive: bool| {
        TallyLedgerEntry {
            entry_index,
            ledger_name: ledger_name.to_string(),
            amount: amount.to_string(),
            is_deemed_positive,
            raw_source_sha256: fixture_digest(&format!(
                "voucher-guid|{entry_index}|{ledger_name}|{amount}|{is_deemed_positive}"
            )),
        }
    };
    export_of(vec![source_record(
        TallyVoucher {
            id: Some("voucher-guid".to_string()),
            date: Some("20260714".to_string()),
            voucher_type: Some("Receipt".to_string()),
            voucher_number: Some("SYN-1".to_string()),
            party_ledger_name: None,
            cancelled: Some(false),
            optional: Some(false),
            ledger_entry_count: Some(2),
            ledger_entries: vec![
                entry(1, first_ledger, "-100.00", first_deemed_positive),
                entry(2, second_ledger, "100.00", second_deemed_positive),
            ],
        },
        "voucher-guid",
        Some("voucher-remote"),
        "9",
        "10",
    )])
}

fn assess_flags(
    cash_flag: &str,
    sales_flag: &str,
) -> bridge_tally_core::reconciliation::CoreAccountingAssessment {
    let (ledgers, _) = ledgers_and_vouchers("Cash", "Cash");
    let window = build_core_window(
        &context(),
        groups(),
        ledgers,
        voucher_types(),
        voucher_with_entry_flags(cash_flag, sales_flag),
    )
    .expect("a disagreeing flag is observed data, not a build failure");
    let PackBatch::CoreAccounting(batch) = &window.batch else {
        panic!("expected a core accounting pack")
    };
    // The persisted polarity is still Tally's flag, not a value derived
    // from the amount. bridge#392 turned on keeping it that way.
    assert_eq!(batch.ledger_entries[1].amount.as_str(), "100.00");
    bridge_tally_core::reconciliation::assess_core_accounting(batch)
}

#[test]
fn a_lone_disagreeing_leg_reaches_reconciliation_and_is_not_a_mismatch() {
    // The rounding shape, driven the whole way: parsed records -> canonical window ->
    // reconciliation. `Sales 100.00` carries `Yes`, so its flag disagrees
    // with its sign while `Cash` agrees. One leg of two.
    let assessment = assess_flags("Yes", "Yes");
    assert_eq!(
        assessment.checks.voucher_entry_balance,
        reconciliation::CheckState::Passed,
        "the voucher still sums to zero"
    );
    assert_eq!(
        assessment.checks.voucher_entry_polarity,
        reconciliation::CheckState::Passed
    );
    assert!(assessment
        .issues
        .iter()
        .all(|issue| issue.safe_reason_code != "voucher_entry_polarity_mismatch"));
}

#[test]
fn a_wholly_inverted_voucher_reaches_reconciliation_and_is_reported() {
    // Same builder, both legs inverted. Asserted beside the case above so
    // the pair is visibly a pair: without this, the test above would keep
    // passing if inversion detection broke generally.
    let assessment = assess_flags("No", "Yes");
    assert_eq!(
        assessment.checks.voucher_entry_balance,
        reconciliation::CheckState::Passed,
        "a wholly inverted voucher still balances, which is why the balance check cannot see it"
    );
    assert_eq!(
        assessment.checks.voucher_entry_polarity,
        reconciliation::CheckState::Mismatch
    );
    assert!(assessment
        .issues
        .iter()
        .any(|issue| issue.safe_reason_code == "voucher_entry_polarity_mismatch"));
}

#[test]
fn marker_carrying_parent_policy_fails_closed_for_unobserved_non_root_references() {
    let group_ids_by_name = BTreeMap::from([("Assets".to_string(), "group-guid".to_string())]);

    for value in [
        format!("{TALLY_SANITIZED_ROOT_MARKER} Primary"),
        format!("  {TALLY_SANITIZED_ROOT_MARKER}  primary "),
    ] {
        assert_eq!(
            resolve_group_parent(Some(&value), &group_ids_by_name, "group_parent_missing").unwrap(),
            None
        );
        assert_eq!(
            resolve_optional_reference(
                Some(&value),
                &group_ids_by_name,
                "ledger_parent_group_missing"
            )
            .unwrap(),
            None
        );
    }

    // This pins Bridge's policy for an input Tally has not been observed to emit; it is not
    // evidence about Tally behaviour. Measured: the marker occurs on 30 group parents and
    // one ledger parent across two companies, always followed by `Primary`; Bridge's
    // `TALLY_PROTOCOL_REFERENCE.md:66-76` records it on `OBJECTUPDATEACTION` as
    // `&#4; Resave`. The policy fails closed: the old starts-with rule silently turned an
    // unrecognised marker-prefixed value into a `None` parent, whereas this rule surfaces it
    // as a missing reference.
    // A bare `Primary` names a group a user called that
    // (`TALLY_PROTOCOL_REFERENCE.md` §1.1(d), §8.2b): with no such group
    // observed it is a missing reference, never the root.
    for value in [
        "Primary".to_string(),
        format!("{TALLY_SANITIZED_ROOT_MARKER} Resave"),
        format!("{TALLY_SANITIZED_ROOT_MARKER} Anything"),
        format!("{TALLY_SANITIZED_ROOT_MARKER}{TALLY_SANITIZED_ROOT_MARKER} Primary"),
    ] {
        assert!(matches!(
            resolve_group_parent(Some(&value), &group_ids_by_name, "group_parent_missing"),
            Err(TallyError::InvalidData { code }) if code == "group_parent_missing"
        ));
        assert!(matches!(
            resolve_optional_reference(
                Some(&value),
                &group_ids_by_name,
                "ledger_parent_group_missing"
            ),
            Err(TallyError::InvalidData { code }) if code == "ledger_parent_group_missing"
        ));
    }

    // And when such a group is observed, a bare `Primary` resolves to it.
    let with_primary_group =
        BTreeMap::from([("Primary".to_string(), "user-primary-guid".to_string())]);
    assert_eq!(
        resolve_group_parent(Some("Primary"), &with_primary_group, "group_parent_missing").unwrap(),
        Some("user-primary-guid".to_string())
    );
    assert_eq!(
        resolve_optional_reference(
            Some("Primary"),
            &with_primary_group,
            "ledger_parent_group_missing"
        )
        .unwrap(),
        Some("user-primary-guid".to_string())
    );
}

#[test]
fn marker_prefixed_real_master_name_resolves_by_its_raw_name() {
    let marked_name = format!("{TALLY_SANITIZED_ROOT_MARKER} Resave");
    let groups = padded_native_two_group_export(
        &marked_name,
        "&#4; Primary",
        "Synthetic Child Group",
        &marked_name,
    );
    let ledgers = padded_native_ledger_export("Synthetic Ledger", "Synthetic Child Group");
    let voucher_types = padded_native_voucher_type_export("Synthetic Receipt");
    let vouchers = padded_native_voucher_export("Synthetic Receipt", "Synthetic Ledger");

    let window = build_core_window(&context(), groups, ledgers, voucher_types, vouchers)
        .expect("a real marker-prefixed master name must resolve by its exact raw text");
    let PackBatch::CoreAccounting(batch) = window.batch else {
        panic!("wrong pack");
    };
    let child = batch
        .groups
        .iter()
        .find(|group| group.name == "Synthetic Child Group")
        .expect("child group is present");

    assert_eq!(
        child.parent_source_id.as_deref(),
        Some(format!("{PADDED_COMPANY_GUID}-00000001").as_str())
    );
}

#[test]
fn canonicalizes_all_core_records_with_exact_reference_and_provenance_binding() {
    // Source-reported counts are set explicitly: that branch is reached only by the
    // Bridge-defined export shapes, which issue #1570 deletes.
    let window = valid_window_with_source_reported_counts();
    window.validate_record_evidence_binding().unwrap();
    let PackBatch::CoreAccounting(batch) = &window.batch else {
        panic!("wrong pack")
    };
    assert_eq!(
        (
            batch.groups.len(),
            batch.ledgers.len(),
            batch.voucher_types.len()
        ),
        (1, 2, 1)
    );
    assert_eq!((batch.vouchers.len(), batch.ledger_entries.len()), (1, 2));
    assert_eq!(
        batch.ledgers[0].parent_source_id.as_deref(),
        Some("group-guid")
    );
    assert_eq!(batch.groups[0].parent_source_id, None);
    assert_eq!(
        batch.vouchers[0].voucher_type_source_id,
        "voucher-type-guid"
    );
    assert_eq!(batch.ledger_entries[0].ledger_source_id, "ledger-cash");
    assert_eq!(batch.ledger_entries[0].voucher_source_id, "voucher-guid");
    assert_eq!(batch.ledger_entries[0].polarity, LedgerEntryPolarity::Debit);
    assert_eq!(
        batch.ledger_entries[1].polarity,
        LedgerEntryPolarity::Credit
    );
    assert!(batch.ledger_entries[0]
        .source_id
        .starts_with("bridge-derived:ledger-entry:v1:"));
    assert_eq!(window.source_counts.as_ref().unwrap().len(), 4);
    assert!(window
        .source_counts
        .as_ref()
        .unwrap()
        .iter()
        .all(|evidence| evidence.object_type.as_str() != "ledger_entry"));
    assert_eq!(window.record_evidence.as_ref().unwrap().len(), 7);

    let voucher_evidence = window
        .record_evidence
        .as_ref()
        .unwrap()
        .iter()
        .find(|evidence| evidence.object_type.as_str() == "voucher")
        .unwrap();
    assert_eq!(voucher_evidence.identity_kind, SourceIdentityKind::Guid);
    assert_eq!(
        voucher_evidence
            .observed_identities
            .remote_id
            .as_ref()
            .unwrap()
            .as_str(),
        "voucher-remote"
    );
    assert_eq!(
        voucher_evidence
            .observed_identities
            .master_id
            .as_ref()
            .unwrap()
            .as_str(),
        "9"
    );
}

#[test]
fn nested_entry_totals_remain_local_and_are_never_claimed_as_source_reported() {
    // Source-reported counts are set explicitly: that branch is reached only by the
    // Bridge-defined export shapes, which issue #1570 deletes.
    let window = valid_window_with_source_reported_counts();
    let PackBatch::CoreAccounting(batch) = &window.batch else {
        panic!("wrong pack")
    };

    assert_eq!(batch.ledger_entries.len(), 2);
    assert_eq!(
        window
            .record_evidence
            .as_ref()
            .unwrap()
            .iter()
            .filter(|evidence| evidence.object_type.as_str() == "ledger_entry")
            .count(),
        2
    );
    assert!(window
        .source_counts
        .as_ref()
        .unwrap()
        .iter()
        .all(|evidence| evidence.object_type.as_str() != "ledger_entry"));
}

#[test]
fn native_shaped_exports_give_a_window_with_no_source_reported_counts() {
    // The shipped native parsers observe their row counts and never see a count
    // Tally reported, so the window carries per-record evidence and no
    // `source_counts`.
    let window = valid_window();
    assert!(window.source_counts.is_none());
    assert_eq!(window.record_evidence.as_ref().unwrap().len(), 7);
    window.validate_record_evidence_binding().unwrap();
}

#[test]
fn derived_entry_ids_are_deterministic_but_never_claim_native_identity() {
    fn entry_ids(window: &CanonicalPackWindow) -> Vec<String> {
        let PackBatch::CoreAccounting(batch) = &window.batch else {
            panic!("wrong pack")
        };
        batch
            .ledger_entries
            .iter()
            .map(|entry| entry.source_id.clone())
            .collect()
    }
    let first = valid_window();
    let second = valid_window();
    assert_eq!(entry_ids(&first), entry_ids(&second));
    let entry_evidence = first
        .record_evidence
        .as_ref()
        .unwrap()
        .iter()
        .filter(|evidence| evidence.object_type.as_str() == "ledger_entry")
        .collect::<Vec<_>>();
    assert_eq!(entry_evidence.len(), 2);
    assert!(entry_evidence.iter().all(|evidence| {
        evidence.identity_kind == SourceIdentityKind::Fallback
            && evidence.observed_identities == ObservedSourceIdentities::default()
    }));
}

#[test]
fn unresolved_mutable_name_reference_fails_closed() {
    let (ledgers, vouchers) = ledgers_and_vouchers("Cash", "Missing Ledger");
    let error =
        build_core_window(&context(), groups(), ledgers, voucher_types(), vouchers).unwrap_err();
    assert!(matches!(
        error,
        TallyError::InvalidData { code }
            if code == "voucher_ledger_reference_missing"
    ));
}

#[test]
fn duplicate_mutable_names_fail_closed_even_when_native_ids_differ() {
    let (ledgers, vouchers) = ledgers_and_vouchers("Sales", "Sales");
    let error =
        build_core_window(&context(), groups(), ledgers, voucher_types(), vouchers).unwrap_err();
    assert!(matches!(
        error,
        TallyError::InvalidData { code } if code == "ledger_name_duplicate"
    ));
}

#[test]
fn invalid_or_out_of_window_voucher_dates_fail_before_canonical_state_exists() {
    for (date, expected_code) in [
        ("20260230", "voucher_date_invalid"),
        ("20260630", "voucher_date_outside_requested_window"),
        ("20260801", "voucher_date_outside_requested_window"),
    ] {
        let (ledgers, mut vouchers) = ledgers_and_vouchers("Cash", "Cash");
        vouchers.records[0].record.date = Some(date.to_string());
        let error = build_core_window(&context(), groups(), ledgers, voucher_types(), vouchers)
            .unwrap_err();
        assert!(matches!(
            error,
            TallyError::InvalidData { code } if code == expected_code
        ));
    }
}

#[test]
fn captured_svtodate_bound_drop_fails_closed_before_canonicalisation() {
    let bytes = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/response-illegal-svtodate-bound-dropped-wr2.utf16le.xml"
    );
    let xml = decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("captured UTF-16LE response decodes")
    .text;
    let vouchers = parse_native_voucher_source_records_with_evidence(
        &xml,
        "61c6de69-1748-461c-ad3f-162cb949df9f",
    )
    .expect("captured response has structurally valid native vouchers");

    let error = validate_selected_voucher_window("20260401", "20260430", &vouchers)
        .expect_err("out-of-window response rows must fail closed");
    assert!(matches!(
        error,
        TallyError::InvalidData { code } if code == "voucher_date_outside_requested_window"
    ));
}

#[test]
fn foreign_master_name_with_c1_control_is_retained_and_diagnosed_verbatim() {
    let mojibake = "ZZ Curly âQuotedâ Ledger";
    let (mut ledgers, mut vouchers) = ledgers_and_vouchers(mojibake, mojibake);
    ledgers.records[0].record.name = mojibake.to_string();
    vouchers.records[0].record.ledger_entries[0].ledger_name = mojibake.to_string();

    let window = build_core_window(&context(), groups(), ledgers, voucher_types(), vouchers)
        .expect("foreign text must not reject an otherwise valid company window");
    let PackBatch::CoreAccounting(batch) = window.batch else {
        panic!("wrong pack");
    };
    assert_eq!(batch.ledgers[0].name, mojibake);
    assert_eq!(batch.foreign_master_text_diagnostics.len(), 1);
    assert_eq!(
        batch.foreign_master_text_diagnostics[0]
            .likely_intended_spelling
            .as_deref(),
        Some("ZZ Curly “Quoted” Ledger")
    );
}

// Regression coverage for the master-name/reference trim asymmetry: master NAME
// attributes are stored verbatim (`attr_value` in bridge-tally-protocol/src/lib.rs
// does not trim), so the canonical lookup maps below are keyed on the untrimmed
// name. A LEDGERNAME/PARENT/VOUCHERTYPENAME reference that trimmed its own text
// before the fix could never match a padded master name and would fail the whole
// company read with `..._reference_missing`. No real capture exhibits this padding
// (see the safety-check note in the PR description), so these three tests use
// synthetic XML shaped exactly like the real native captures in
// `bridge-tally-protocol/tests/fixtures/native/*` (ENVELOPE/HEADER/STATUS,
// BODY/DATA/COLLECTION, GUID/MASTERID/ALTERID identity, company-guid-prefixed
// GUIDs) and are read by the native parsers, whereas the helpers at the top of
// this module build the parsed records directly, with no XML involved.
const PADDED_COMPANY_GUID: &str = "synthetic-company-guid";

fn padded_native_group_export(
    name: &str,
    parent_ref: &str,
) -> ParsedExport<ParsedSourceRecord<TallyNamedMaster>> {
    let xml = format!(
        r#"<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><GROUP NAME="{name}"><GUID>{PADDED_COMPANY_GUID}-00000001</GUID><MASTERID>1</MASTERID><ALTERID>1</ALTERID><PARENT>{parent_ref}</PARENT></GROUP></COLLECTION></DATA></BODY></ENVELOPE>"#
    );
    parse_native_group_source_records_with_evidence(&xml, PADDED_COMPANY_GUID)
        .expect("synthetic native group row parses")
}

fn padded_native_two_group_export(
    parent_name: &str,
    parent_parent_ref: &str,
    child_name: &str,
    child_parent_ref: &str,
) -> ParsedExport<ParsedSourceRecord<TallyNamedMaster>> {
    let xml = format!(
        r#"<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><GROUP NAME="{parent_name}"><GUID>{PADDED_COMPANY_GUID}-00000001</GUID><MASTERID>1</MASTERID><ALTERID>1</ALTERID><PARENT>{parent_parent_ref}</PARENT></GROUP><GROUP NAME="{child_name}"><GUID>{PADDED_COMPANY_GUID}-00000002</GUID><MASTERID>2</MASTERID><ALTERID>2</ALTERID><PARENT>{child_parent_ref}</PARENT></GROUP></COLLECTION></DATA></BODY></ENVELOPE>"#
    );
    parse_native_group_source_records_with_evidence(&xml, PADDED_COMPANY_GUID)
        .expect("synthetic native two-group collection parses")
}

fn padded_native_ledger_export(
    name: &str,
    parent_ref: &str,
) -> ParsedExport<ParsedSourceRecord<TallyLedger>> {
    let xml = format!(
        r#"<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><LEDGER NAME="{name}"><GUID>{PADDED_COMPANY_GUID}-00000002</GUID><MASTERID>2</MASTERID><ALTERID>2</ALTERID><PARENT>{parent_ref}</PARENT><OPENINGBALANCE>0.00</OPENINGBALANCE></LEDGER></COLLECTION></DATA></BODY></ENVELOPE>"#
    );
    parse_native_ledger_source_records_with_evidence(&xml, PADDED_COMPANY_GUID)
        .expect("synthetic native ledger row parses")
}

fn padded_native_voucher_type_export(
    name: &str,
) -> ParsedExport<ParsedSourceRecord<TallyNamedMaster>> {
    let xml = format!(
        r#"<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><VOUCHERTYPE NAME="{name}"><GUID>{PADDED_COMPANY_GUID}-00000003</GUID><MASTERID>3</MASTERID><ALTERID>3</ALTERID><PARENT>Primary</PARENT></VOUCHERTYPE></COLLECTION></DATA></BODY></ENVELOPE>"#
    );
    parse_native_voucher_type_source_records_with_evidence(&xml, PADDED_COMPANY_GUID)
        .expect("synthetic native voucher type row parses")
}

fn padded_native_voucher_export(
    voucher_type_ref: &str,
    entry_ledger_name_ref: &str,
) -> ParsedExport<ParsedSourceRecord<TallyVoucher>> {
    let xml = format!(
        r#"<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><VOUCHER REMOTEID="{PADDED_COMPANY_GUID}-00000004"><DATE>20260714</DATE><GUID>{PADDED_COMPANY_GUID}-00000004</GUID><MASTERID>4</MASTERID><ALTERID>4</ALTERID><VOUCHERTYPENAME>{voucher_type_ref}</VOUCHERTYPENAME><VOUCHERNUMBER>SYN-1</VOUCHERNUMBER><ISCANCELLED>No</ISCANCELLED><ISOPTIONAL>No</ISOPTIONAL><ALLLEDGERENTRIES.LIST><LEDGERNAME>{entry_ledger_name_ref}</LEDGERNAME><AMOUNT>-100.00</AMOUNT><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE></ALLLEDGERENTRIES.LIST></VOUCHER></COLLECTION></DATA></BODY></ENVELOPE>"#
    );
    parse_native_voucher_source_records_with_evidence(&xml, PADDED_COMPANY_GUID)
        .expect("synthetic native voucher row parses")
}

#[test]
fn padded_ledger_name_and_its_ledgername_reference_resolve_together() {
    let padded_ledger_name = " Padded Cash Ledger ";
    let groups = padded_native_group_export("Assets", "&#4; Primary");
    let ledgers = padded_native_ledger_export(padded_ledger_name, "Assets");
    let voucher_types = padded_native_voucher_type_export("Receipt");
    let vouchers = padded_native_voucher_export("Receipt", padded_ledger_name);

    let window = build_core_window(&context(), groups, ledgers, voucher_types, vouchers).expect(
        "a ledger name and its LEDGERNAME reference carry identical padding and must resolve",
    );
    let PackBatch::CoreAccounting(batch) = window.batch else {
        panic!("wrong pack");
    };
    // The resolved name must retain its exact original bytes, not a trimmed copy.
    assert_eq!(batch.ledgers[0].name, padded_ledger_name);
    assert_eq!(batch.ledger_entries.len(), 1);
    assert_eq!(
        batch.ledger_entries[0].ledger_source_id,
        format!("{PADDED_COMPANY_GUID}-00000002")
    );
}

#[test]
fn padded_group_name_and_a_sibling_groups_parent_reference_resolve_together() {
    let padded_group_name = " Padded Parent Group ";
    let groups = padded_native_two_group_export(
        padded_group_name,
        "&#4; Primary",
        "Child Group",
        padded_group_name,
    );
    let ledgers = padded_native_ledger_export("Cash", "Child Group");
    let voucher_types = padded_native_voucher_type_export("Receipt");
    let vouchers = padded_native_voucher_export("Receipt", "Cash");

    let window = build_core_window(&context(), groups, ledgers, voucher_types, vouchers).expect(
        "a group name and a sibling group's PARENT reference carry identical padding and must resolve",
    );
    let PackBatch::CoreAccounting(batch) = window.batch else {
        panic!("wrong pack");
    };
    let parent_group = batch
        .groups
        .iter()
        .find(|group| group.source_id == format!("{PADDED_COMPANY_GUID}-00000001"))
        .expect("padded parent group is present");
    // The resolved name must retain its exact original bytes, not a trimmed copy.
    assert_eq!(parent_group.name, padded_group_name);
    let child_group = batch
        .groups
        .iter()
        .find(|group| group.source_id == format!("{PADDED_COMPANY_GUID}-00000002"))
        .expect("child group is present");
    assert_eq!(
        child_group.parent_source_id.as_deref(),
        Some(format!("{PADDED_COMPANY_GUID}-00000001").as_str())
    );
}

#[test]
fn padded_voucher_type_name_and_its_vouchertypename_reference_resolve_together() {
    let padded_voucher_type_name = " Padded Receipt Type ";
    let groups = padded_native_group_export("Assets", "&#4; Primary");
    let ledgers = padded_native_ledger_export("Cash", "Assets");
    let voucher_types = padded_native_voucher_type_export(padded_voucher_type_name);
    let vouchers = padded_native_voucher_export(padded_voucher_type_name, "Cash");

    let window = build_core_window(&context(), groups, ledgers, voucher_types, vouchers).expect(
        "a voucher type name and its VOUCHERTYPENAME reference carry identical padding and must resolve",
    );
    let PackBatch::CoreAccounting(batch) = window.batch else {
        panic!("wrong pack");
    };
    // The resolved name must retain its exact original bytes, not a trimmed copy.
    assert_eq!(batch.voucher_types[0].name, padded_voucher_type_name);
    assert_eq!(
        batch.vouchers[0].voucher_type_source_id,
        format!("{PADDED_COMPANY_GUID}-00000003")
    );
}

#[test]
fn invalid_requested_window_fails_before_source_rows_are_canonicalized() {
    for (from, to) in [("20260230", "20260731"), ("20260801", "20260731")] {
        let mut request = context();
        request.window.from_yyyymmdd = from.to_string();
        request.window.to_yyyymmdd = to.to_string();
        let (ledgers, vouchers) = ledgers_and_vouchers("Cash", "Cash");
        let error =
            build_core_window(&request, groups(), ledgers, voucher_types(), vouchers).unwrap_err();
        assert!(matches!(
            error,
            TallyError::InvalidData { code } if code == "requested_window_invalid"
        ));
    }
}

#[test]
fn selected_voucher_qualification_rejects_noncanonical_records_and_entries() {
    let (_, vouchers) = ledgers_and_vouchers("Cash", "Cash");
    validate_selected_voucher_window("20260701", "20260731", &vouchers).unwrap();

    let mut invalid_amount = vouchers.clone();
    invalid_amount.records[0].record.ledger_entries[0].amount = "not-an-amount".to_string();
    assert!(validate_selected_voucher_window("20260701", "20260731", &invalid_amount).is_err());

    let mut foreign_name = vouchers.clone();
    foreign_name.records[0].record.ledger_entries[0].ledger_name = " x ".to_string();
    assert!(
        validate_selected_voucher_window("20260701", "20260731", &foreign_name).is_ok(),
        "Tally-originated master text is not a Bridge canonical token"
    );

    let mut invalid_alter_id = vouchers;
    invalid_alter_id.records[0].alter_id = Some("contains whitespace".to_string());
    assert!(validate_selected_voucher_window("20260701", "20260731", &invalid_alter_id).is_err());
}
