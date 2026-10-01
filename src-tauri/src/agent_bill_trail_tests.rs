//! The bill trail and the unadjusted detail: what ties, and what refuses to.
//!
//! The vouchers are a live capture (settle-then-reopen, TallyPrime Silver 7.1,
//! `vouchers_settle_then_reopen_live`): five bills each opened, settled to zero
//! and reopened for a smaller amount, and two controls settled and never
//! reopened. The native bills rows are written by hand from the amounts the
//! capture was built with (they are what Tally's own report lists at the end of
//! that window), not from this code's output:
//!
//! | party | bill | opened | settled | reopened | open now |
//! |---|---|---|---|---|---|
//! | RO Party 01 | RO-INV-003 | 2000 | 2000 | 750 | 750 (Dr) |
//! | RO Party 01 | RO-CTL-001, RO-CTL-002 | 1100, 1300 | same | none | 0 |
//! | RO Party 02 | RO-INV-001, RO-INV-004 | 1000, 2500 | same | 400, 900 | 400, 900 |
//! | RO Party 03 | RO-INV-002, RO-INV-005 | 1500, 3000 | same | 600, 1200 | 600, 1200 |
use super::*;
use crate::agent::voucher_parse::parse_agent_rows;
use crate::tally::UnallocatedComposition;

const REOPEN_COMPANY_GUID: &str = "ec4454ae-5c4c-4bfa-b3b0-68182a749689";
const BILL_DATE: &str = "20260810";

fn reopen_rows() -> Vec<Value> {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/vouchers_settle_then_reopen_live.utf16le.xml"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    parse_agent_rows(&xml, REOPEN_COMPANY_GUID).expect("the capture parses")
}

fn decimal(text: &str) -> ExactDecimal {
    ExactDecimal::parse(text).unwrap()
}

fn open_bill(
    party: &str,
    reference: &str,
    bill_date: &str,
    amount: &str,
    kind: ExposureDirection,
) -> OpenBillRow {
    OpenBillRow {
        party: party.into(),
        reference: reference.into(),
        bill_date: bill_date.into(),
        due_date: bill_date.into(),
        amount: decimal(amount),
        age_days: Some(0),
        kind,
    }
}

fn natives_for_party_01() -> Vec<OpenBillRow> {
    vec![open_bill(
        "RO Party 01",
        "RO-INV-003",
        BILL_DATE,
        "750",
        ExposureDirection::Receivable,
    )]
}

#[test]
fn a_reopened_bill_ties_to_its_native_balance_and_a_settled_control_ties_to_zero() {
    let rows = reopen_rows();
    let entries = entries_for_party(&rows, "RO Party 01").unwrap();
    let trails = bill_trails("RO Party 01", None, &entries, &natives_for_party_01()).unwrap();
    let summary = trails
        .iter()
        .map(|outcome| match outcome {
            BillOutcome::Tied(trail) => (
                trail.reference.as_str(),
                trail.balance.as_str().to_string(),
                trail.listed_open,
                trail.entries.len(),
            ),
            other => panic!("expected every bill to tie, got {}", other.state()),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        summary,
        vec![
            ("RO-CTL-001", "0".to_string(), false, 2),
            ("RO-CTL-002", "0".to_string(), false, 2),
            ("RO-INV-003", "-750".to_string(), true, 3),
        ]
    );
}

#[test]
fn the_trail_lists_its_allocations_oldest_first_with_the_original_bill_date() {
    let rows = reopen_rows();
    let entries = entries_for_party(&rows, "RO Party 01").unwrap();
    let trails = bill_trails(
        "RO Party 01",
        Some("RO-INV-003"),
        &entries,
        &natives_for_party_01(),
    )
    .unwrap();
    assert_eq!(trails.len(), 1, "the reference filter keeps one bill");
    let BillOutcome::Tied(trail) = &trails[0] else {
        panic!("expected a tied bill")
    };
    let dates = trail
        .entries
        .iter()
        .map(|e| e.date.as_str())
        .collect::<Vec<_>>();
    assert_eq!(dates, ["20260810", "20260812", "20260814"]);
    let kinds = trail.entries.iter().map(|e| e.kind).collect::<Vec<_>>();
    assert_eq!(
        kinds,
        [
            AllocationKind::NewRef,
            AllocationKind::AgstRef,
            AllocationKind::AgstRef
        ]
    );
    // Every allocation, including the reopening on a later voucher, carries the
    // original bill's date: that is what makes (party, reference, bill_date) a key.
    assert!(trail
        .entries
        .iter()
        .all(|e| e.bill_date.as_deref() == Some(BILL_DATE)));
    let amounts = trail
        .entries
        .iter()
        .map(|e| e.amount.as_str())
        .collect::<Vec<_>>();
    assert_eq!(amounts, ["-2000.00", "2000.00", "-750.00"]);
}

#[test]
fn a_native_balance_that_the_allocations_do_not_sum_to_is_reported_with_both_numbers() {
    let rows = reopen_rows();
    let entries = entries_for_party(&rows, "RO Party 01").unwrap();
    let wrong = vec![open_bill(
        "RO Party 01",
        "RO-INV-003",
        BILL_DATE,
        "700",
        ExposureDirection::Receivable,
    )];
    let trails = bill_trails("RO Party 01", Some("RO-INV-003"), &entries, &wrong).unwrap();
    let BillOutcome::DoesNotTie {
        trail_sum, native, ..
    } = &trails[0]
    else {
        panic!("expected trail_does_not_tie, got {}", trails[0].state())
    };
    assert_eq!(trail_sum.as_str(), "-750");
    assert_eq!(native.as_ref().unwrap().as_str(), "-700");
}

#[test]
fn a_dropped_allocation_makes_the_trail_not_tie() {
    let rows = reopen_rows();
    let mut entries = entries_for_party(&rows, "RO Party 01").unwrap();
    // Lose the reopening: the window or a filter dropped a voucher.
    entries.retain(|entry| {
        !(entry.reference.as_deref() == Some("RO-INV-003") && entry.date == "20260814")
    });
    let trails = bill_trails(
        "RO Party 01",
        Some("RO-INV-003"),
        &entries,
        &natives_for_party_01(),
    )
    .unwrap();
    assert_eq!(trails[0].state(), "trail_does_not_tie");
}

#[test]
fn a_settled_bill_whose_allocations_do_not_reach_zero_does_not_tie() {
    let rows = reopen_rows();
    let mut entries = entries_for_party(&rows, "RO Party 01").unwrap();
    entries.retain(|entry| {
        !(entry.reference.as_deref() == Some("RO-CTL-001") && entry.kind == AllocationKind::AgstRef)
    });
    let trails = bill_trails("RO Party 01", Some("RO-CTL-001"), &entries, &[]).unwrap();
    let BillOutcome::DoesNotTie {
        native, trail_sum, ..
    } = &trails[0]
    else {
        panic!("{}", trails[0].state())
    };
    assert!(native.is_none(), "the report does not list it");
    assert_eq!(trail_sum.as_str(), "-1100");
}

#[test]
fn a_bill_matched_by_more_than_one_native_row_or_a_different_date_is_ambiguous_and_never_merged() {
    let rows = reopen_rows();
    let entries = entries_for_party(&rows, "RO Party 01").unwrap();
    let twice = vec![
        open_bill(
            "RO Party 01",
            "RO-INV-003",
            BILL_DATE,
            "750",
            ExposureDirection::Receivable,
        ),
        open_bill(
            "RO Party 01",
            "RO-INV-003",
            BILL_DATE,
            "750",
            ExposureDirection::Receivable,
        ),
    ];
    let trails = bill_trails("RO Party 01", Some("RO-INV-003"), &entries, &twice).unwrap();
    let BillOutcome::Ambiguous { native_rows, .. } = &trails[0] else {
        panic!("{}", trails[0].state())
    };
    assert_eq!(*native_rows, 2);
    // One native row, but dated differently from the allocations' own bill date.
    let other_date = vec![open_bill(
        "RO Party 01",
        "RO-INV-003",
        "20260811",
        "750",
        ExposureDirection::Receivable,
    )];
    let trails = bill_trails("RO Party 01", Some("RO-INV-003"), &entries, &other_date).unwrap();
    assert_eq!(trails[0].state(), "bill_identity_ambiguous");
}

#[test]
fn allocations_carrying_two_bill_dates_under_one_reference_are_ambiguous() {
    let rows = reopen_rows();
    let mut entries = entries_for_party(&rows, "RO Party 01").unwrap();
    for entry in entries.iter_mut().filter(|e| {
        e.reference.as_deref() == Some("RO-INV-003") && e.kind == AllocationKind::AgstRef
    }) {
        entry.bill_date = Some("20260901".into());
    }
    let trails = bill_trails(
        "RO Party 01",
        Some("RO-INV-003"),
        &entries,
        &natives_for_party_01(),
    )
    .unwrap();
    let BillOutcome::Ambiguous {
        bill_dates_in_vouchers,
        ..
    } = &trails[0]
    else {
        panic!("{}", trails[0].state())
    };
    assert_eq!(bill_dates_in_vouchers.len(), 2);
}

#[test]
fn another_partys_bill_with_the_same_reference_is_not_this_partys() {
    let rows = reopen_rows();
    let entries = entries_for_party(&rows, "RO Party 01").unwrap();
    // Party 02 also has a bill named RO-INV-003 natively: it must not be matched.
    let natives = vec![
        open_bill(
            "RO Party 01",
            "RO-INV-003",
            BILL_DATE,
            "750",
            ExposureDirection::Receivable,
        ),
        open_bill(
            "RO Party 02",
            "RO-INV-003",
            BILL_DATE,
            "9999",
            ExposureDirection::Receivable,
        ),
    ];
    let trails = bill_trails("RO Party 01", Some("RO-INV-003"), &entries, &natives).unwrap();
    assert_eq!(trails[0].state(), "tied");
}

#[test]
fn a_bill_the_report_lists_but_no_voucher_allocates_to_still_appears_and_cannot_tie() {
    let rows = reopen_rows();
    let entries = entries_for_party(&rows, "RO Party 01").unwrap();
    let mut natives = natives_for_party_01();
    natives.push(open_bill(
        "RO Party 01",
        "OPEN-1",
        BILL_DATE,
        "5000",
        ExposureDirection::Receivable,
    ));
    let trails = bill_trails("RO Party 01", None, &entries, &natives).unwrap();
    let opening = trails
        .iter()
        .find(|t| matches!(t, BillOutcome::DoesNotTie { reference, .. } if reference == "OPEN-1"))
        .expect("the opening bill is listed");
    let BillOutcome::DoesNotTie {
        entries, native, ..
    } = opening
    else {
        unreachable!()
    };
    assert!(entries.is_empty());
    assert_eq!(native.as_ref().unwrap().as_str(), "-5000");
}

#[test]
fn a_payable_row_is_positive_and_a_receivable_row_negative() {
    let entry = |amount: &str, kind| TrailEntry {
        date: "20260810".into(),
        voucher_type: "Purchase".into(),
        voucher_number: Some("1".into()),
        guid: "g".into(),
        kind,
        reference: Some("B".into()),
        bill_date: Some("20260810".into()),
        amount: decimal(amount),
    };
    let entries = vec![entry("900", AllocationKind::NewRef)];
    let payable = vec![open_bill(
        "P",
        "B",
        "20260810",
        "900",
        ExposureDirection::Payable,
    )];
    assert_eq!(
        bill_trails("P", None, &entries, &payable).unwrap()[0].state(),
        "tied"
    );
    // The same magnitude in the receivable report is a different sign and does not tie.
    let receivable = vec![open_bill(
        "P",
        "B",
        "20260810",
        "900",
        ExposureDirection::Receivable,
    )];
    assert_eq!(
        bill_trails("P", None, &entries, &receivable).unwrap()[0].state(),
        "trail_does_not_tie"
    );
}

fn voucher(date: &str, kind: &str, party: &str, entries: Vec<Value>) -> Value {
    json!({"date": date, "voucher_type": kind, "voucher_number": "1", "guid": format!("g-{date}-{kind}"), "party": party, "cancelled": false, "optional": false, "amounts": entries})
}

fn entry(ledger: &str, amount: &str, allocations: Vec<Value>) -> Value {
    json!({"ledger": ledger, "amount": amount, "is_deemed_positive": "Yes", "bill_allocations": allocations})
}

fn allocation(kind: &str, reference: Option<&str>, amount: &str, bill_date: Option<&str>) -> Value {
    let mut value = json!({
        "reference": reference.map_or(json!({"kind": "on_account"}), |name| json!({"kind": "named", "name": name})),
        "bill_type": kind, "amount": amount,
    });
    if let Some(date) = bill_date {
        value["bill_date"] = json!(date);
    }
    value
}

#[test]
fn an_allocation_on_a_second_partys_ledger_in_the_same_voucher_is_that_partys() {
    // A journal moving a bill from party A to party B: one voucher, one party
    // field, allocations on two ledgers. Only the entry's own ledger counts.
    let rows = vec![voucher(
        "20260701",
        "Journal",
        "B",
        vec![
            entry(
                "B",
                "-2000.00",
                vec![allocation(
                    "New Ref",
                    Some("JV-2"),
                    "-2000.00",
                    Some("20260701"),
                )],
            ),
            entry(
                "A",
                "2000.00",
                vec![allocation(
                    "Agst Ref",
                    Some("INV-1"),
                    "2000.00",
                    Some("20260401"),
                )],
            ),
        ],
    )];
    let a = entries_for_party(&rows, "A").unwrap();
    assert_eq!(a.len(), 1);
    assert_eq!(a[0].reference.as_deref(), Some("INV-1"));
    let b = entries_for_party(&rows, "B").unwrap();
    assert_eq!(b.len(), 1);
    assert_eq!(b[0].reference.as_deref(), Some("JV-2"));
}

#[test]
fn cancelled_and_optional_vouchers_are_left_out_as_tallys_reports_leave_them_out() {
    let mut cancelled = voucher(
        "20260701",
        "Sales",
        "A",
        vec![entry(
            "A",
            "-5.00",
            vec![allocation("New Ref", Some("X"), "-5.00", Some("20260701"))],
        )],
    );
    cancelled["cancelled"] = json!(true);
    let mut optional = voucher(
        "20260702",
        "Sales",
        "A",
        vec![entry(
            "A",
            "-6.00",
            vec![allocation("New Ref", Some("Y"), "-6.00", Some("20260702"))],
        )],
    );
    optional["optional"] = json!(true);
    let live = voucher(
        "20260703",
        "Sales",
        "A",
        vec![entry(
            "A",
            "-7.00",
            vec![allocation("New Ref", Some("Z"), "-7.00", Some("20260703"))],
        )],
    );
    let entries = entries_for_party(&[cancelled, optional, live], "A").unwrap();
    assert_eq!(
        entries
            .iter()
            .map(|e| e.reference.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["Z"]
    );
}

#[test]
fn rows_that_are_not_shaped_as_a_trail_needs_are_refused_not_guessed() {
    let bad_type = vec![voucher(
        "20260701",
        "Sales",
        "A",
        vec![entry(
            "A",
            "-5.00",
            vec![allocation("Sideways", Some("X"), "-5.00", Some("20260701"))],
        )],
    )];
    assert_eq!(
        entries_for_party(&bad_type, "A"),
        Err(TrailRefusal("trail_allocation_type_unknown"))
    );
    let no_date = vec![voucher(
        "20260701",
        "Sales",
        "A",
        vec![entry(
            "A",
            "-5.00",
            vec![allocation("New Ref", Some("X"), "-5.00", None)],
        )],
    )];
    assert_eq!(
        entries_for_party(&no_date, "A"),
        Err(TrailRefusal("trail_bill_date_missing"))
    );
    let no_reference = vec![voucher(
        "20260701",
        "Sales",
        "A",
        vec![entry(
            "A",
            "-5.00",
            vec![
                json!({"reference": {"kind": "on_account"}, "bill_type": "New Ref", "amount": "-5.00", "bill_date": "20260701"}),
            ],
        )],
    )];
    assert_eq!(
        entries_for_party(&no_reference, "A"),
        Err(TrailRefusal("trail_reference_missing"))
    );
    let bad_amount = vec![voucher(
        "20260701",
        "Sales",
        "A",
        vec![entry(
            "A",
            "-5.00",
            vec![allocation("New Ref", Some("X"), "five", Some("20260701"))],
        )],
    )];
    assert_eq!(
        entries_for_party(&bad_amount, "A"),
        Err(TrailRefusal("trail_amount_invalid"))
    );
}

/// Every field a trail reads from a voucher row is required: a row missing
/// one is refused as `trail_voucher_malformed`, never skipped or defaulted.
#[test]
fn a_voucher_row_missing_a_field_the_trail_reads_is_refused_as_malformed() {
    let good = || {
        voucher(
            "20260701",
            "Sales",
            "A",
            vec![entry(
                "A",
                "-5.00",
                vec![allocation("New Ref", Some("X"), "-5.00", Some("20260701"))],
            )],
        )
    };
    assert_eq!(entries_for_party(&[good()], "A").unwrap().len(), 1);
    for pointer in [
        "/date",
        "/voucher_type",
        "/guid",
        "/amounts",
        "/amounts/0/bill_allocations",
        "/amounts/0/bill_allocations/0/bill_type",
        "/amounts/0/bill_allocations/0/amount",
    ] {
        let mut row = good();
        let (parent, key) = pointer.rsplit_once('/').unwrap();
        let removed = if parent.is_empty() {
            row.as_object_mut().unwrap().remove(key)
        } else {
            row.pointer_mut(parent)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(key)
        };
        assert!(removed.is_some(), "{pointer}");
        assert_eq!(
            entries_for_party(&[row], "A"),
            Err(TrailRefusal("trail_voucher_malformed")),
            "{pointer}"
        );
    }
    // A field of the wrong type is as missing as an absent one.
    let mut row = good();
    row["amounts"][0]["bill_allocations"] = json!("none");
    assert_eq!(
        entries_for_party(&[row], "A"),
        Err(TrailRefusal("trail_voucher_malformed"))
    );
}

/// A named allocation without its bill date is refused by the trail itself,
/// not given an empty date, whether or not Tally lists the bill.
#[test]
fn a_named_allocation_without_its_bill_date_is_refused_not_given_an_empty_date() {
    let mut undated = trail_entry(1);
    undated.bill_date = None;
    assert_eq!(
        bill_trails("P", None, std::slice::from_ref(&undated), &[]),
        Err(TrailRefusal("trail_bill_date_missing"))
    );
    // A settled pair with no native row would otherwise have tied, dated "".
    let mut opening = trail_entry(2);
    opening.bill_date = None;
    let mut settling = trail_entry(3);
    settling.bill_date = None;
    settling.amount = decimal("1");
    assert_eq!(
        bill_trails("P", None, &[opening, settling], &[]),
        Err(TrailRefusal("trail_bill_date_missing"))
    );
    // One dated and one undated allocation of one bill is refused too.
    let dated = trail_entry(4);
    assert_eq!(
        bill_trails("P", None, &[dated, undated], &[]),
        Err(TrailRefusal("trail_bill_date_missing"))
    );
}

fn unallocated(
    amount: &str,
    direction: ExposureDirection,
    opening: Option<&str>,
    composition: UnallocatedComposition,
) -> UnallocatedParty {
    UnallocatedParty {
        party: "P".into(),
        amount: decimal(amount),
        direction,
        opening_balance: opening.map(decimal),
        composition: Some(composition),
    }
}

fn on_account_rows() -> Vec<Value> {
    vec![
        voucher(
            "20260512",
            "Receipt",
            "P",
            vec![entry(
                "P",
                "4000.00",
                vec![allocation("On Account", None, "4000.00", None)],
            )],
        ),
        voucher(
            "20260520",
            "Payment",
            "P",
            vec![entry(
                "P",
                "-1000.00",
                vec![allocation("On Account", None, "-1000.00", None)],
            )],
        ),
    ]
}

#[test]
fn the_on_account_entries_of_a_party_sum_to_its_unallocated_residual() {
    // The OL book's P02: on-account receipt 4,000 and payment 1,000 net to +3,000
    // (payable direction on a debtor), which is its unallocated residual.
    let entries = entries_for_party(&on_account_rows(), "P").unwrap();
    let residual = unallocated(
        "3000",
        ExposureDirection::Payable,
        Some("0.00"),
        UnallocatedComposition::BillWiseLedgerComponentsNotSeparated,
    );
    let detail = unadjusted_detail(&entries, &[], "P", Some(&residual)).unwrap();
    assert_eq!(detail.state(), "tied");
    assert!(detail.on_account_sum.numeric_eq(&decimal("3000")));
    assert_eq!(detail.rows.len(), 2);
    assert!(detail.rows.iter().all(|(class, _)| *class == "on_account"));
    assert_eq!(
        detail.tie,
        UnadjustedTie::Tied {
            residual: decimal("3000")
        }
    );
}

#[test]
fn a_residual_the_vouchers_do_not_explain_is_reported_with_the_difference_and_the_opening_fact() {
    // No voucher allocates anything: the whole -20,000 residual is unexplained,
    // and it equals the ledger's own opening balance. That is stated as a fact.
    let residual = unallocated(
        "20000",
        ExposureDirection::Receivable,
        Some("-20000.00"),
        UnallocatedComposition::BillWiseLedgerComponentsNotSeparated,
    );
    let detail = unadjusted_detail(&[], &[], "P", Some(&residual)).unwrap();
    assert_eq!(
        detail.tie,
        UnadjustedTie::ResidualNotExplainedByVouchers {
            residual: decimal("-20000"),
            difference: decimal("-20000"),
            equals_opening_balance: Some(true),
        }
    );
    let opening_fact = |opening: Option<&str>| match unadjusted_detail(
        &[],
        &[],
        "P",
        Some(&unallocated(
            "20000",
            ExposureDirection::Receivable,
            opening,
            UnallocatedComposition::BillWiseLedgerComponentsNotSeparated,
        )),
    )
    .unwrap()
    .tie
    {
        UnadjustedTie::ResidualNotExplainedByVouchers {
            equals_opening_balance,
            ..
        } => equals_opening_balance,
        other => panic!("{other:?}"),
    };
    // A different opening does not match.
    assert_eq!(opening_fact(Some("-100.00")), Some(false));
    // An opening Tally did not send is neither equal nor unequal.
    assert_eq!(opening_fact(None), None);
    // The JSON states the comparison as a fact and nothing else: these exact
    // fields, so no label for the difference can creep in beside them.
    assert_eq!(
        detail.json(),
        json!({
            "state": "residual_not_explained_by_vouchers",
            "residual": "-20000",
            "on_account_sum": "0",
            "rows": [],
            "difference": "-20000",
            "difference_equals_opening_balance": true,
        })
    );
}

/// Absent is not zero: a ledger the snapshot lists no residual for (a bank,
/// sales or capital ledger, a name that differs from the native row's, or a
/// party whose residual is zero) is never `tied` against a residual of 0.
#[test]
fn a_party_without_an_unallocated_row_has_no_residual_and_ties_nothing() {
    let detail = unadjusted_detail(&[], &[], "P", None).unwrap();
    assert_eq!(detail.tie, UnadjustedTie::NoResidualRowForParty);
    assert_eq!(detail.state(), "no_residual_row_for_party");
    assert!(detail.rows.is_empty());
    let json = detail.json();
    assert_eq!(json["state"], "no_residual_row_for_party");
    assert!(json["residual"].is_null(), "{json}");
    assert!(json.get("difference").is_none(), "{json}");
    // On-account entries are still listed as read, and still tie nothing.
    let entries = entries_for_party(&on_account_rows(), "P").unwrap();
    let detail = unadjusted_detail(&entries, &[], "P", None).unwrap();
    assert_eq!(detail.tie, UnadjustedTie::NoResidualRowForParty);
    assert_eq!(detail.rows.len(), 2);
    assert!(detail.on_account_sum.numeric_eq(&decimal("3000")));
    // A row found for another party is not this party's.
    let other = unallocated(
        "3000",
        ExposureDirection::Payable,
        Some("0.00"),
        UnallocatedComposition::BillWiseLedgerComponentsNotSeparated,
    );
    let detail = party_detail(
        DetailKind::Unadjusted,
        "Q",
        &json!("Q"),
        None,
        &entries,
        &[],
        &[other],
        2,
    )
    .unwrap();
    assert_eq!(detail["state"], "no_residual_row_for_party");
    assert!(detail["residual"].is_null(), "{detail}");
}

/// A window read that returned no voucher at all is not corroborated, so it
/// ties nothing and lists nothing: never `bills: []`, never a native bill that
/// "does not tie" for want of vouchers, never a residual the vouchers "do not
/// explain".
#[test]
fn a_window_that_returned_no_vouchers_is_its_own_state_and_ties_nothing() {
    let natives = [open_bill(
        "P",
        "R",
        "20260701",
        "5",
        ExposureDirection::Receivable,
    )];
    let residual = [unallocated(
        "20000",
        ExposureDirection::Receivable,
        Some("-20000.00"),
        UnallocatedComposition::BillWiseLedgerComponentsNotSeparated,
    )];
    for (kind, label) in [
        (DetailKind::BillTrail, "bill_trail"),
        (DetailKind::Unadjusted, "unadjusted"),
    ] {
        let detail =
            party_detail(kind, "P", &json!("P"), None, &[], &natives, &residual, 0).unwrap();
        assert_eq!(
            detail,
            json!({"kind": label, "state": "window_returned_no_vouchers"}),
            "{label}"
        );
    }
    // A reference asked for is not refused as unknown on an empty read either:
    // nothing was read to know it by.
    assert_eq!(
        party_detail(
            DetailKind::BillTrail,
            "P",
            &json!("P"),
            Some("UNKNOWN"),
            &[],
            &[],
            &[],
            0
        )
        .unwrap()["state"],
        "window_returned_no_vouchers"
    );
    // The same inputs with vouchers read do tie, or fail to.
    let read = party_detail(
        DetailKind::Unadjusted,
        "P",
        &json!("P"),
        None,
        &[],
        &natives,
        &residual,
        3,
    )
    .unwrap();
    assert_eq!(read["state"], "residual_not_explained_by_vouchers");
}

/// An empty bill list says in band why it is empty, and is never a bare `[]`.
#[test]
fn an_empty_bill_trail_says_why_it_is_empty() {
    let trail = |entries: &[TrailEntry], unallocated: &[UnallocatedParty]| {
        party_detail(
            DetailKind::BillTrail,
            "P",
            &json!("P"),
            None,
            entries,
            &[],
            unallocated,
            2,
        )
        .unwrap()
    };
    // Only on-account allocations, and no bill in Tally's report.
    let entries = entries_for_party(&on_account_rows(), "P").unwrap();
    let empty = trail(&entries, &[]);
    assert_eq!(empty["bills"], json!([]));
    assert_eq!(empty["state"], "no_named_bill_for_party");
    // No allocation at all on this ledger.
    assert_eq!(trail(&[], &[])["state"], "no_named_bill_for_party");
    // The ledger snapshot says the ledger keeps no bills.
    let not_bill_wise = unallocated(
        "7500",
        ExposureDirection::Receivable,
        Some("0.00"),
        UnallocatedComposition::NotBillWiseLedger,
    );
    assert_eq!(
        trail(&[], &[not_bill_wise])["state"],
        "not_bill_wise_ledger"
    );
    // A listed bill makes the answer a list.
    let listed = party_detail(
        DetailKind::BillTrail,
        "P",
        &json!("P"),
        None,
        &[],
        &[open_bill(
            "P",
            "R",
            "20260701",
            "5",
            ExposureDirection::Receivable,
        )],
        &[],
        1,
    )
    .unwrap();
    assert_eq!(listed["state"], "bills_listed");
    assert_eq!(listed["bills"].as_array().unwrap().len(), 1);
}

#[test]
fn a_party_on_a_ledger_that_keeps_no_bills_returns_no_rows() {
    let residual = unallocated(
        "7500",
        ExposureDirection::Receivable,
        Some("0.00"),
        UnallocatedComposition::NotBillWiseLedger,
    );
    let detail = unadjusted_detail(&[], &[], "P", Some(&residual)).unwrap();
    assert_eq!(detail.state(), "not_bill_wise_ledger");
    assert!(detail.rows.is_empty());
}

#[test]
fn advances_and_open_pending_notes_are_listed_and_settled_notes_are_not() {
    let rows = vec![
        voucher(
            "20260515",
            "Receipt",
            "P",
            vec![entry(
                "P",
                "6000.00",
                vec![allocation(
                    "Advance",
                    Some("ADV-1"),
                    "6000.00",
                    Some("20260515"),
                )],
            )],
        ),
        voucher(
            "20260520",
            "Credit Note",
            "P",
            vec![entry(
                "P",
                "3000.00",
                vec![allocation(
                    "New Ref",
                    Some("CN-1"),
                    "3000.00",
                    Some("20260520"),
                )],
            )],
        ),
        voucher(
            "20260521",
            "Credit Note",
            "P",
            vec![entry(
                "P",
                "100.00",
                vec![allocation(
                    "New Ref",
                    Some("CN-SETTLED"),
                    "100.00",
                    Some("20260521"),
                )],
            )],
        ),
        voucher(
            "20260522",
            "Sales",
            "P",
            vec![entry(
                "P",
                "-50.00",
                vec![allocation(
                    "New Ref",
                    Some("INV-1"),
                    "-50.00",
                    Some("20260522"),
                )],
            )],
        ),
    ];
    let entries = entries_for_party(&rows, "P").unwrap();
    // CN-1 is still open natively; CN-SETTLED is not listed; INV-1 is an ordinary bill.
    let natives = vec![open_bill(
        "P",
        "CN-1",
        "20260520",
        "3000",
        ExposureDirection::Payable,
    )];
    let detail = unadjusted_detail(&entries, &natives, "P", None).unwrap();
    let classes = detail
        .rows
        .iter()
        .map(|(class, e)| (*class, e.reference.clone().unwrap()))
        .collect::<Vec<_>>();
    assert_eq!(
        classes,
        vec![
            ("advance", "ADV-1".to_string()),
            ("pending_note_with_reference", "CN-1".to_string())
        ]
    );
}

#[test]
fn the_json_of_each_state_names_it_and_carries_both_numbers_when_it_does_not_tie() {
    let rows = reopen_rows();
    let entries = entries_for_party(&rows, "RO Party 01").unwrap();
    let wrong = vec![open_bill(
        "RO Party 01",
        "RO-INV-003",
        BILL_DATE,
        "700",
        ExposureDirection::Receivable,
    )];
    let outcome = &bill_trails("RO Party 01", Some("RO-INV-003"), &entries, &wrong).unwrap()[0];
    let json = outcome.json(json!("RO Party 01"));
    assert_eq!(json["state"], "trail_does_not_tie");
    assert_eq!(json["trail_sum"], "-750");
    assert_eq!(json["native_balance"], "-700");
    assert_eq!(json["allocations"].as_array().unwrap().len(), 3);
    let tied = &bill_trails(
        "RO Party 01",
        Some("RO-INV-003"),
        &entries,
        &natives_for_party_01(),
    )
    .unwrap()[0];
    let json = tied.json(json!("RO Party 01"));
    assert_eq!(json["state"], "tied");
    assert_eq!(json["open"], true);
    assert_eq!(json["bill_date"], BILL_DATE);
}

// ---- the seeded book, on live captures -----------------------------------------
//
// BRIDGE OUTSTANDINGS LAB (TallyPrime Silver 7.1, synthetic, seeded by this
// project; captured 1 Oct 2026 after all five seed writes). The vouchers are
// the raw answer of the `vouchers` data request for 20250401..20260630 (20
// vouchers); the native rows are the raw answers of Bills Receivable and Bills
// Payable at 20260630. The expectations below are worked out by hand from the
// vouchers the book was seeded with, not from this code:
//
// | party | bill | allocations (Dr negative) | open now |
// |---|---|---|---|
// | P01 | OL-INV-001 | -10,000, +4,000 | -6,000 |
// | P01 | OL-INV-002 | -6,000, then +2,000 by the W5 journal (whose party field is P11) | -4,000 |
// | P04 | OL-ADV-001 | +6,000 (an advance) | +6,000 |
// | P05 | OL-INV-301 | -12,000 | -12,000 |
// | P05 | OL-CN-001 | +3,000 (a credit note with a reference) | +3,000 |
// | P06 | OL-BILL-401 | +9,000 | +9,000 |
// | P06 | OL-DN-001 | -2,000 (a debit note with a reference) | -2,000 |
// | P09 | OL-REUSE-1 | -5,000, +5,000, then -4,200 a year later | -4,200 |
// | P10 | OL-REUSE-1 | -6,500 | -6,500 |
// | P11 | OL-JV-001, OL-JV-002 | -5,000 (journal), -2,000 (the W5 journal) | -5,000, -2,000 |
// | P07 | OL-OPEN-001 | none: an opening on the ledger | -33,333 |
const LAB_COMPANY_GUID: &str = "49f1fbda-ee59-4a4b-aacf-b45fe32402d7";

fn lab_fixture(name: &str) -> String {
    let bytes = std::fs::read(format!(
        "{}/crates/bridge-tally-protocol/tests/fixtures/agent/{name}.utf16le.xml",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap_or_else(|error| panic!("fixture {name}: {error}"));
    assert_eq!(bytes.len() % 2, 0, "UTF-16LE has an even length");
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn lab_rows() -> Vec<Value> {
    let rows = parse_agent_rows(
        &lab_fixture("native-outstandings-lab-vouchers-window"),
        LAB_COMPANY_GUID,
    )
    .expect("the capture parses");
    assert_eq!(rows.len(), 20);
    rows
}

fn lab_natives() -> Vec<OpenBillRow> {
    use bridge_tally_protocol::native_outstandings::parse_native_bill_rows;
    let from = bridge_tally_core::TallyDate::parse("20250401").unwrap();
    let to = bridge_tally_core::TallyDate::parse("20260630").unwrap();
    let mut out = Vec::new();
    for (name, kind) in [
        (
            "native-outstandings-lab-bills-receivable-after-journal",
            ExposureDirection::Receivable,
        ),
        (
            "native-outstandings-lab-bills-payable-after-journal",
            ExposureDirection::Payable,
        ),
    ] {
        for row in parse_native_bill_rows(&lab_fixture(name), &from, &to).unwrap() {
            out.push(OpenBillRow {
                party: row.party.clone(),
                reference: row.reference.clone(),
                bill_date: row.bill_date.as_str().to_string(),
                due_date: row.due_date.as_str().to_string(),
                amount: row.closing_balance.abs().unwrap(),
                age_days: None,
                kind,
            });
        }
    }
    assert_eq!(
        out.len(),
        13,
        "the 10 receivable and 3 payable rows after the journal"
    );
    out
}

/// (reference, bill date, balance, state, allocations)
type TrailSummary = (String, String, String, &'static str, usize);

fn lab_trails(party: &str) -> Vec<TrailSummary> {
    let entries = entries_for_party(&lab_rows(), party).unwrap();
    bill_trails(party, None, &entries, &lab_natives())
        .unwrap()
        .iter()
        .map(|outcome| match outcome {
            BillOutcome::Tied(trail) => (
                trail.reference.clone(),
                trail.bill_date.clone(),
                trail.balance.as_str().to_string(),
                "tied",
                trail.entries.len(),
            ),
            other => (
                other.reference().to_string(),
                String::new(),
                String::new(),
                other.state(),
                0,
            ),
        })
        .collect()
}

#[test]
fn every_named_bill_of_the_seeded_book_ties_to_the_native_report() {
    let s = |value: &str| value.to_string();
    let expected: Vec<(&str, Vec<TrailSummary>)> = vec![
        (
            "OL P01 Named Bills Debtor",
            vec![
                (s("OL-INV-001"), s("20250415"), s("-6000"), "tied", 2),
                (s("OL-INV-002"), s("20250420"), s("-4000"), "tied", 2),
            ],
        ),
        (
            "OL P04 Advance Debtor",
            vec![(s("OL-ADV-001"), s("20250515"), s("6000"), "tied", 1)],
        ),
        (
            "OL P05 Credit Note Debtor",
            vec![
                (s("OL-CN-001"), s("20250520"), s("3000"), "tied", 1),
                (s("OL-INV-301"), s("20250418"), s("-12000"), "tied", 1),
            ],
        ),
        (
            "OL P06 Debit Note Creditor",
            vec![
                (s("OL-BILL-401"), s("20250422"), s("9000"), "tied", 1),
                (s("OL-DN-001"), s("20250528"), s("-2000"), "tied", 1),
            ],
        ),
        (
            "OL P09 Reuse Debtor A",
            vec![(s("OL-REUSE-1"), s("20250610"), s("-4200"), "tied", 3)],
        ),
        (
            "OL P10 Reuse Debtor B",
            vec![(s("OL-REUSE-1"), s("20250612"), s("-6500"), "tied", 1)],
        ),
        (
            "OL P11 Journal Debtor",
            vec![
                (s("OL-JV-001"), s("20250701"), s("-5000"), "tied", 1),
                (s("OL-JV-002"), s("20250715"), s("-2000"), "tied", 1),
            ],
        ),
    ];
    for (party, bills) in expected {
        assert_eq!(lab_trails(party), bills, "{party}");
    }
}

#[test]
fn the_same_reference_on_two_parties_is_two_separate_bills() {
    // P09's OL-REUSE-1 (reused a year later on the same party, stored as one continuing
    // bill) and P10's are different bills with different keys and balances.
    let a = lab_trails("OL P09 Reuse Debtor A");
    let b = lab_trails("OL P10 Reuse Debtor B");
    assert_eq!((a[0].1.as_str(), a[0].2.as_str()), ("20250610", "-4200"));
    assert_eq!((b[0].1.as_str(), b[0].2.as_str()), ("20250612", "-6500"));
}

#[test]
fn a_journal_whose_party_field_names_another_party_still_belongs_to_the_party_on_whose_ledger_it_allocates(
) {
    let rows = lab_rows();
    // The W5 journal's own party field is P11 (the debit ledger), but its Agst Ref
    // against OL-INV-002 is on P01's ledger. A filter on the voucher's party field
    // would have lost it and P01's bill would not tie.
    let journal = rows
        .iter()
        .find(|row| {
            row["narration"]
                .as_str()
                .is_some_and(|text| text.starts_with("OL V20"))
        })
        .expect("the W5 journal");
    assert_eq!(journal["party"], "OL P11 Journal Debtor");
    let entries = entries_for_party(&rows, "OL P01 Named Bills Debtor").unwrap();
    assert!(entries
        .iter()
        .any(|e| e.reference.as_deref() == Some("OL-INV-002") && e.date == "20250715"));
    let by_voucher_party = entries_for_party(&rows, "OL P11 Journal Debtor").unwrap();
    assert!(by_voucher_party
        .iter()
        .all(|e| e.reference.as_deref() != Some("OL-INV-002")));
}

#[test]
fn an_opening_bill_with_no_voucher_is_listed_and_cannot_tie() {
    let entries = entries_for_party(&lab_rows(), "OL P07 Referenced Opening Debtor").unwrap();
    assert!(entries.is_empty());
    let trails = bill_trails(
        "OL P07 Referenced Opening Debtor",
        None,
        &entries,
        &lab_natives(),
    )
    .unwrap();
    assert_eq!(trails.len(), 1);
    assert_eq!(trails[0].state(), "trail_does_not_tie");
    let BillOutcome::DoesNotTie {
        native, entries, ..
    } = &trails[0]
    else {
        unreachable!()
    };
    assert!(entries.is_empty());
    assert_eq!(native.as_ref().unwrap().as_str(), "-33333");
}

#[test]
fn dropping_one_allocation_of_the_seeded_book_breaks_the_tie() {
    let mut entries = entries_for_party(&lab_rows(), "OL P09 Reuse Debtor A").unwrap();
    entries.retain(|e| e.date != "20260610");
    let trails = bill_trails("OL P09 Reuse Debtor A", None, &entries, &lab_natives()).unwrap();
    assert_eq!(trails[0].state(), "trail_does_not_tie");
}

fn lab_unallocated(
    party: &str,
    amount: &str,
    direction: ExposureDirection,
    opening: &str,
    composition: UnallocatedComposition,
) -> UnallocatedParty {
    UnallocatedParty {
        party: party.into(),
        amount: decimal(amount),
        direction,
        opening_balance: Some(decimal(opening)),
        composition: Some(composition),
    }
}

#[test]
fn the_unadjusted_detail_of_the_seeded_book_ties_where_the_vouchers_explain_the_residual() {
    use UnallocatedComposition::BillWiseLedgerComponentsNotSeparated as Mixed;
    let rows = lab_rows();
    let natives = lab_natives();
    // (party, residual amount, direction, expected state, on-account rows, other rows)
    let cases = [
        (
            "OL P02 On Account Debtor",
            "3000",
            ExposureDirection::Payable,
            "tied",
            2usize,
            0usize,
        ),
        (
            "OL P05 Credit Note Debtor",
            "1500",
            ExposureDirection::Payable,
            "tied",
            1,
            1,
        ),
        (
            "OL P06 Debit Note Creditor",
            "800",
            ExposureDirection::Receivable,
            "tied",
            1,
            1,
        ),
    ];
    for (party, amount, direction, state, on_account, other) in cases {
        let entries = entries_for_party(&rows, party).unwrap();
        let residual = lab_unallocated(party, amount, direction, "0.00", Mixed);
        let detail = unadjusted_detail(&entries, &natives, party, Some(&residual)).unwrap();
        assert_eq!(detail.state(), state, "{party}");
        assert_eq!(
            detail
                .rows
                .iter()
                .filter(|(c, _)| *c == "on_account")
                .count(),
            on_account,
            "{party}"
        );
        assert_eq!(
            detail
                .rows
                .iter()
                .filter(|(c, _)| *c != "on_account")
                .count(),
            other,
            "{party}"
        );
    }
    // P04 has an advance and no residual row: listed, and nothing tied against a residual.
    let entries = entries_for_party(&rows, "OL P04 Advance Debtor").unwrap();
    let detail = unadjusted_detail(&entries, &natives, "OL P04 Advance Debtor", None).unwrap();
    assert_eq!(detail.state(), "no_residual_row_for_party");
    assert_eq!(
        detail.rows.iter().map(|(c, _)| *c).collect::<Vec<_>>(),
        ["advance"]
    );
    // P08: an opening with no reference, and no voucher at all.
    let party = "OL P08 Unreferenced Opening Debtor";
    let residual = lab_unallocated(
        party,
        "20000",
        ExposureDirection::Receivable,
        "-20000.00",
        Mixed,
    );
    let detail = unadjusted_detail(
        &entries_for_party(&rows, party).unwrap(),
        &natives,
        party,
        Some(&residual),
    )
    .unwrap();
    assert!(matches!(
        detail.tie,
        UnadjustedTie::ResidualNotExplainedByVouchers {
            equals_opening_balance: Some(true),
            ..
        }
    ));
    // P03: a ledger that keeps no bills.
    let party = "OL P03 Not Billwise Debtor";
    let residual = lab_unallocated(
        party,
        "7500",
        ExposureDirection::Receivable,
        "0.00",
        UnallocatedComposition::NotBillWiseLedger,
    );
    let detail = unadjusted_detail(
        &entries_for_party(&rows, party).unwrap(),
        &natives,
        party,
        Some(&residual),
    )
    .unwrap();
    assert_eq!(detail.state(), "not_bill_wise_ledger");
}

// ---- review fixes: ordering, native-only duplicates, unknown reference, arguments, cap ----

#[test]
fn allocations_come_back_oldest_first_whatever_order_the_vouchers_arrive_in() {
    let later = voucher(
        "20260901",
        "Receipt",
        "P",
        vec![entry(
            "P",
            "400",
            vec![allocation("Agst Ref", Some("R1"), "400", Some("20260801"))],
        )],
    );
    let earlier = voucher(
        "20260801",
        "Sales",
        "P",
        vec![entry(
            "P",
            "-1000",
            vec![allocation("New Ref", Some("R1"), "-1000", Some("20260801"))],
        )],
    );
    // The later voucher is listed first.
    let entries = entries_for_party(&[later, earlier], "P").unwrap();
    assert_eq!(
        entries.iter().map(|e| e.date.as_str()).collect::<Vec<_>>(),
        ["20260801", "20260901"]
    );
    let natives = vec![open_bill(
        "P",
        "R1",
        "20260801",
        "600",
        ExposureDirection::Receivable,
    )];
    let trails = bill_trails("P", None, &entries, &natives).unwrap();
    let BillOutcome::Tied(trail) = &trails[0] else {
        panic!("{}", trails[0].state())
    };
    assert_eq!(trail.entries[0].date, "20260801");
    assert_eq!(trail.entries[1].date, "20260901");
}

#[test]
fn two_native_rows_for_one_reference_that_no_voucher_allocates_to_are_ambiguous_not_dropped() {
    let natives = vec![
        open_bill("P", "DUP", "20260101", "100", ExposureDirection::Receivable),
        open_bill("P", "DUP", "20260201", "200", ExposureDirection::Receivable),
        open_bill("P", "ONE", "20260101", "50", ExposureDirection::Receivable),
    ];
    // Named or not, the answer is the same shape.
    for reference in [None, Some("DUP")] {
        let trails = bill_trails("P", reference, &[], &natives).unwrap();
        let dup = trails
            .iter()
            .find(|t| t.reference() == "DUP")
            .expect("the reference is listed");
        let BillOutcome::Ambiguous {
            native_rows,
            entries,
            ..
        } = dup
        else {
            panic!("{}", dup.state())
        };
        assert_eq!(*native_rows, 2);
        assert!(entries.is_empty());
    }
    let all = bill_trails("P", None, &[], &natives).unwrap();
    assert_eq!(all.len(), 2, "DUP once, ONE once");
    assert_eq!(
        all.iter().find(|t| t.reference() == "ONE").unwrap().state(),
        "trail_does_not_tie"
    );
}

#[test]
fn the_json_of_an_ambiguous_bill_names_the_state_the_dates_and_each_allocations_bill_date() {
    let rows = vec![
        voucher(
            "20260101",
            "Sales",
            "P",
            vec![entry(
                "P",
                "-100",
                vec![allocation("New Ref", Some("R"), "-100", Some("20260101"))],
            )],
        ),
        voucher(
            "20260301",
            "Sales",
            "P",
            vec![entry(
                "P",
                "-200",
                vec![allocation("New Ref", Some("R"), "-200", Some("20260301"))],
            )],
        ),
    ];
    let entries = entries_for_party(&rows, "P").unwrap();
    let outcome = &bill_trails("P", None, &entries, &[]).unwrap()[0];
    let json = outcome.json(json!("P"));
    assert_eq!(json["state"], "bill_identity_ambiguous");
    assert_eq!(
        json["bill_dates_in_vouchers"],
        json!(["20260101", "20260301"])
    );
    assert_eq!(json["native_rows"], 0);
    assert_eq!(json["native_bill_dates"], json!([]));
    let allocations = json["allocations"].as_array().unwrap();
    assert_eq!(allocations.len(), 2);
    // The two bills' allocations are told apart by their own bill date.
    assert_eq!(allocations[0]["bill_date"], "20260101");
    assert_eq!(allocations[1]["bill_date"], "20260301");
}

#[test]
fn an_unknown_reference_is_refused_not_returned_as_an_empty_list() {
    let rows = reopen_rows();
    let entries = entries_for_party(&rows, "RO Party 01").unwrap();
    assert_eq!(
        bill_trails(
            "RO Party 01",
            Some("NO-SUCH-BILL"),
            &entries,
            &natives_for_party_01()
        ),
        Err(TrailRefusal("bill_reference_not_found"))
    );
    // A party with no bills at all, asked about no reference, is an empty list.
    assert_eq!(
        bill_trails("Nobody", None, &[], &natives_for_party_01()).unwrap(),
        Vec::new()
    );
}

#[test]
fn a_detail_asks_for_a_party_and_a_kind_and_a_reference_only_for_a_trail() {
    use super::bill_trail::{parse_detail_request, DetailKind};
    assert_eq!(parse_detail_request(None, None, None), Ok(None));
    assert_eq!(
        parse_detail_request(Some("P"), Some("bill_trail"), None),
        Ok(Some(DetailKind::BillTrail))
    );
    assert_eq!(
        parse_detail_request(Some("P"), Some("bill_trail"), Some("R")),
        Ok(Some(DetailKind::BillTrail))
    );
    assert_eq!(
        parse_detail_request(Some("P"), Some("unadjusted"), None),
        Ok(Some(DetailKind::Unadjusted))
    );
    assert_eq!(
        parse_detail_request(Some("P"), None, None),
        Err("party_requires_detail")
    );
    assert_eq!(
        parse_detail_request(None, Some("bill_trail"), None),
        Err("detail_requires_party")
    );
    assert_eq!(
        parse_detail_request(Some("P"), Some("everything"), None),
        Err("invalid_detail")
    );
    assert_eq!(
        parse_detail_request(Some("P"), Some("unadjusted"), Some("R")),
        Err("reference_requires_bill_trail")
    );
    // A reference alone is a detail request with no party first.
    assert_eq!(
        parse_detail_request(None, None, Some("R")),
        Err("reference_requires_bill_trail")
    );
}

#[test]
fn an_answer_of_more_than_five_hundred_allocations_is_refused_never_cut() {
    use super::bill_trail::within_detail_cap;
    for (kind, code) in [
        (DetailKind::BillTrail, "trail_too_large"),
        (DetailKind::Unadjusted, "unadjusted_detail_too_large"),
    ] {
        assert_eq!(within_detail_cap(kind, 0), Ok(()));
        assert_eq!(within_detail_cap(kind, 500), Ok(()));
        assert_eq!(within_detail_cap(kind, 501), Err(TrailRefusal(code)));
    }
}

/// The advice a refusal over the row limit carries can be followed: a bill
/// trail is narrowed by `reference`, while `reference` with `unadjusted` is
/// itself refused, so the unadjusted refusal says plainly that nothing narrows
/// it instead of sending the caller to `reference`.
#[test]
fn each_over_the_limit_refusal_carries_advice_that_can_be_followed() {
    let trail = crate::agent::refusal_remediation("trail_too_large").unwrap();
    assert!(trail.contains("`reference`"), "{trail}");
    let unadjusted = crate::agent::refusal_remediation("unadjusted_detail_too_large").unwrap();
    assert!(
        unadjusted.contains("`reference` applies only to `bill_trail`"),
        "{unadjusted}"
    );
    assert!(unadjusted.contains("not available"), "{unadjusted}");
    // Which is why: the call its advice would otherwise name is refused.
    assert_eq!(
        super::bill_trail::parse_detail_request(Some("P"), Some("unadjusted"), Some("R")),
        Err("reference_requires_bill_trail")
    );
}

#[test]
fn a_named_bills_window_starts_at_the_earliest_date_tally_lists_for_that_reference() {
    use super::bill_trail::{trail_window_start, DetailKind};
    let natives = vec![
        open_bill("P", "R", "20260301", "10", ExposureDirection::Receivable),
        open_bill("P", "R", "20260101", "10", ExposureDirection::Receivable),
        open_bill("Q", "R", "20250101", "10", ExposureDirection::Receivable),
        open_bill(
            "P",
            "OTHER",
            "20240101",
            "10",
            ExposureDirection::Receivable,
        ),
    ];
    let start = |kind, reference| trail_window_start(kind, reference, "P", &natives, "20250401");
    // The earliest of this party's rows of that reference: not the first listed,
    // not another party's, not another reference's.
    assert_eq!(start(DetailKind::BillTrail, Some("R")), "20260101");
    // A reference Tally no longer lists is a settled bill: from the books' start.
    assert_eq!(start(DetailKind::BillTrail, Some("SETTLED")), "20250401");
    assert_eq!(start(DetailKind::BillTrail, None), "20250401");
    // An opening bill dated before the books begin never starts a window there.
    let opening = vec![open_bill(
        "P",
        "OPEN",
        "20240101",
        "10",
        ExposureDirection::Receivable,
    )];
    assert_eq!(
        trail_window_start(
            DetailKind::BillTrail,
            Some("OPEN"),
            "P",
            &opening,
            "20250401"
        ),
        "20250401"
    );
    assert_eq!(start(DetailKind::Unadjusted, None), "20250401");
}

#[test]
fn a_listed_bill_with_no_allocations_never_ties_even_at_a_zero_balance() {
    // Tally lists open bills only, so a zero native balance is not expected; the
    // rule still holds: an empty trail proves nothing, so it is never `tied`.
    let natives = vec![open_bill(
        "P",
        "Z",
        "20260101",
        "0",
        ExposureDirection::Receivable,
    )];
    let trails = bill_trails("P", Some("Z"), &[], &natives).unwrap();
    assert_eq!(trails.len(), 1);
    assert_eq!(trails[0].state(), "trail_does_not_tie");
}

#[test]
fn one_native_row_dated_differently_from_the_allocations_shows_both_dates() {
    let rows = vec![voucher(
        "20260101",
        "Sales",
        "P",
        vec![entry(
            "P",
            "-100",
            vec![allocation("New Ref", Some("R"), "-100", Some("20260101"))],
        )],
    )];
    let entries = entries_for_party(&rows, "P").unwrap();
    let natives = vec![open_bill(
        "P",
        "R",
        "20260215",
        "100",
        ExposureDirection::Receivable,
    )];
    let json = bill_trails("P", None, &entries, &natives).unwrap()[0].json(json!("P"));
    assert_eq!(json["state"], "bill_identity_ambiguous");
    assert_eq!(json["native_rows"], 1);
    assert_eq!(json["bill_dates_in_vouchers"], json!(["20260101"]));
    assert_eq!(json["native_bill_dates"], json!(["20260215"]));
}

fn trail_entry(n: usize) -> TrailEntry {
    TrailEntry {
        date: "20260101".into(),
        voucher_type: "Sales".into(),
        voucher_number: Some(n.to_string()),
        guid: format!("g{n}"),
        kind: AllocationKind::NewRef,
        reference: Some("R".into()),
        bill_date: Some("20260101".into()),
        amount: decimal("-1"),
    }
}

#[test]
fn the_size_of_a_trail_answer_counts_every_bills_allocations_in_every_state() {
    use super::bill_trail::{trail_row_count, within_detail_cap};
    let outcomes = vec![
        BillOutcome::DoesNotTie {
            reference: "A".into(),
            bill_date: "20260101".into(),
            entries: (0..300).map(trail_entry).collect(),
            trail_sum: decimal("-300"),
            native: None,
        },
        BillOutcome::Ambiguous {
            reference: "B".into(),
            bill_dates_in_vouchers: vec![],
            native_bill_dates: vec![],
            native_rows: 2,
            entries: (0..200).map(trail_entry).collect(),
        },
    ];
    assert_eq!(trail_row_count(&outcomes), 500);
    assert_eq!(
        within_detail_cap(DetailKind::BillTrail, trail_row_count(&outcomes)),
        Ok(())
    );
    let mut more = outcomes;
    more.push(BillOutcome::DoesNotTie {
        reference: "C".into(),
        bill_date: "20260101".into(),
        entries: vec![trail_entry(9_999)],
        trail_sum: decimal("-1"),
        native: None,
    });
    assert_eq!(trail_row_count(&more), 501);
    assert_eq!(
        within_detail_cap(DetailKind::BillTrail, trail_row_count(&more)),
        Err(TrailRefusal("trail_too_large"))
    );
}

#[test]
fn a_refusal_after_the_voucher_read_keeps_that_reads_evidence() {
    use super::bill_trail::with_evidence;
    let read = |hash: &str, bytes: usize| Evidence {
        request_sha256: format!("req-{hash}"),
        response_sha256: format!("res-{hash}"),
        bytes,
        state: "complete",
        read_at: None,
        duration_ms: None,
        reason_code: None,
    };
    let bare = with_evidence(
        ToolFailure::from("trail_too_large".to_string()),
        &read("a", 10),
    );
    assert_eq!(bare.evidence.as_ref().unwrap().bytes, 10);
    let mut carrying = ToolFailure::from("trail_too_large".to_string());
    carrying.evidence = Some(Box::new(read("b", 5)));
    let both = with_evidence(carrying, &read("a", 10));
    assert_eq!(both.evidence.as_ref().unwrap().bytes, 15);
    assert_eq!(both.code, "trail_too_large");
}
