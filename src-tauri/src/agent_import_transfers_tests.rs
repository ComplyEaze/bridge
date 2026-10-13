//! The transfer section and the checks around it, on invented facts: what it
//! lists and in what order, within what budget, and what it never says.
use super::*;
use crate::agent::transfer_pairing::{
    BatchId, BatchStatus, CompanyGuid, ContraFact, ContraKey, ContraLeg, Coverage, Direction,
    DoubleBooked, EndpointOrigin, LedgerGuid, LineageId, ParseWindow, Side, SkipFact, SkipKey,
    StatementRowId, TxnId,
};
use bridge_bank_statement::bank::BankReference;

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

fn skip_key(row: &str) -> SkipKey {
    SkipKey {
        company: CompanyGuid::new("co"),
        bank_ledger: LedgerGuid::new("b2"),
        endpoint_origin: EndpointOrigin::new("ep"),
        row: StatementRowId::new(row),
    }
}

/// An outflow of `amount` from b2 on `day`, skipped as carried by b1's Contra.
fn skip(row: &str, day: &str, amount: &str) -> SkipFact {
    SkipFact {
        key: skip_key(row),
        date: date(day),
        amount: ExactDecimal::parse(amount).unwrap(),
        direction: Direction::Outflow,
        other_ledger: LedgerGuid::new("b1"),
        reference: None,
        lineage: LineageId::new("lineage-b2"),
        reversed_same_day: false,
    }
}

/// The Contra b1's statement booked for such a transfer: debits b1, credits b2.
fn contra(batch: &str, txn: &str, day: &str, amount: &str) -> ContraFact {
    ContraFact {
        key: ContraKey {
            batch: BatchId::new(batch),
            txn: TxnId::new(txn),
        },
        company: CompanyGuid::new("co"),
        endpoint_origin: EndpointOrigin::new("ep"),
        legs: [
            ContraLeg {
                ledger: LedgerGuid::new("b1"),
                side: Side::Debit,
            },
            ContraLeg {
                ledger: LedgerGuid::new("b2"),
                side: Side::Credit,
            },
        ],
        amount: ExactDecimal::parse(amount).unwrap(),
        date: date(day),
        reference: None,
        statement_bank_ledger: LedgerGuid::new("b1"),
        lineage: LineageId::new(format!("lineage-{batch}")),
        status: BatchStatus::PostedVerified,
        reversed_same_day: false,
    }
}

fn coverage() -> Coverage {
    let mut coverage = Coverage::default();
    for bank in ["b1", "b2"] {
        coverage
            .add(
                CompanyGuid::new("co"),
                EndpointOrigin::new("ep"),
                LedgerGuid::new(bank),
                ParseWindow {
                    from: date("20260101"),
                    to: date("20260131"),
                },
            )
            .unwrap();
    }
    coverage
}

fn facts_of(skips: Vec<SkipFact>, contras: Vec<ContraFact>) -> TransferFacts {
    TransferFacts {
        skips,
        contras,
        coverage: coverage(),
        retired_skips: 3,
        held_elsewhere: Vec::new(),
        contras_not_two_leg: 0,
        contras_without_facts: 0,
        contras_superseded: 0,
        live_skips: BTreeSet::new(),
    }
}

fn summary_of(facts: &TransferFacts) -> TransferSummary {
    let view = transfer_pairing_view(
        &facts.skips,
        &facts.contras,
        &facts.coverage,
        TRANSFER_K_DAYS,
    )
    .unwrap();
    TransferSummary::of(facts, &view)
}

/// An open row as the summary lists it: id, date, state label and batch.
type OpenListed = (String, String, &'static str, Option<String>);

fn rows_of(summary: &TransferSummary) -> Vec<OpenListed> {
    let TransferSummary::Computed { open, .. } = summary else {
        panic!("a computed summary");
    };
    open.iter()
        .map(|row| {
            (
                row.id.clone(),
                row.date.as_str().to_string(),
                row.state.label(),
                row.batch.clone(),
            )
        })
        .collect()
}

/// One transfer in every state, with the amounts that must never be said.
fn every_state() -> TransferFacts {
    let mut skips = vec![
        skip("st-paired", "20260105", "111.11"),
        skip("st-unverified", "20260106", "222.22"),
        skip("st-notfound", "20260107", "333.33"),
        skip("st-ambiguous", "20260108", "444.44"),
        skip("st-reversed", "20260109", "555.55"),
        skip("st-outside", "20260301", "666.66"),
        skip("st-matched", "20260302", "888.88"),
        skip("st-differs", "20260111", "999.99"),
    ];
    skips[4].reversed_same_day = true;
    let printing = |digits: &str| BankReference::from_recorded(digits);
    skips[7].reference = printing("600000000001");
    assert!(skips[7].reference.is_some());
    let mut unverified = contra("batch-2", "t2", "20260106", "222.22");
    unverified.status = BatchStatus::NotVerified {
        batch: BatchId::new("batch-2"),
    };
    let contras = vec![
        contra("batch-1", "t1", "20260105", "111.11"),
        unverified,
        contra("batch-3", "t3", "20260108", "444.44"),
        contra("batch-4", "t4", "20260108", "444.44"),
        // one transfer booked from both statements
        contra("batch-5", "t5", "20260110", "777.77"),
        {
            let mut other = contra("batch-6", "t6", "20260110", "777.77");
            other.statement_bank_ledger = LedgerGuid::new("b2");
            other
        },
        // the one Contra of its day and amount, on a day no window holds
        contra("batch-7", "t7", "20260302", "888.88"),
        // two Contras of a row's day and amount, each of another reference
        {
            let mut other = contra("batch-11", "t11", "20260111", "999.99");
            other.reference = printing("600000000003");
            other
        },
        {
            let mut other = contra("batch-10", "t10", "20260111", "999.99");
            other.reference = printing("600000000002");
            other
        },
    ];
    let mut facts = facts_of(skips, contras);
    facts.held_elsewhere = vec![(
        skip_key("st-held"),
        BatchId::new("batch-9"),
        date("20260104"),
    )];
    facts
}

#[test]
fn the_summary_lists_every_open_state_by_date_then_id_with_the_batch_it_names() {
    let summary = summary_of(&every_state());
    let TransferSummary::Computed {
        paired,
        skips_open,
        retired,
        ..
    } = &summary
    else {
        panic!("computed");
    };
    // one pair; seven skips and the held one are open, each counted once however
    // many rows list it; the three retired are counted
    assert_eq!((*paired, *skips_open, *retired), (1, 8, 3));
    let row = |id: &str, day: &str, state, batch: Option<&str>| {
        (
            id.to_string(),
            day.to_string(),
            state,
            batch.map(str::to_string),
        )
    };
    assert_eq!(
        rows_of(&summary),
        vec![
            row("st-held", "20260104", "held_elsewhere", Some("batch-9")),
            row(
                "st-unverified",
                "20260106",
                "other_side_unverified",
                Some("batch-2")
            ),
            row("st-notfound", "20260107", "not_found", None),
            // one row for each batch that holds a Contra that could carry it
            row("st-ambiguous", "20260108", "ambiguous", Some("batch-3")),
            row("st-ambiguous", "20260108", "ambiguous", Some("batch-4")),
            // nothing matches this one, so it names no batch
            row("st-reversed", "20260109", "reversed_same_day", None),
            // both Contras of the double booking, each naming the other batch
            row("t5", "20260110", "double_booked", Some("batch-6")),
            row("t6", "20260110", "double_booked", Some("batch-5")),
            // one row for each batch that holds a Contra of another reference
            row(
                "st-differs",
                "20260111",
                "reference_differs",
                Some("batch-10")
            ),
            row(
                "st-differs",
                "20260111",
                "reference_differs",
                Some("batch-11")
            ),
            row("st-outside", "20260301", "not_established", None),
            row(
                "st-matched",
                "20260302",
                "matched_unconfirmed",
                Some("batch-7")
            ),
        ]
    );
}

fn open_count(summary: &TransferSummary) -> usize {
    let TransferSummary::Computed { skips_open, .. } = summary else {
        panic!("a computed summary");
    };
    *skips_open
}

fn listed(id: &str, day: &str, state: &'static str, batch: Option<&str>) -> OpenListed {
    (
        id.to_string(),
        day.to_string(),
        state,
        batch.map(str::to_string),
    )
}

#[test]
fn a_row_more_than_one_contra_could_carry_is_listed_once_for_each_batch_holding_one() {
    // three Contras of the row's day and amount, two of them in one batch
    let facts = facts_of(
        vec![skip("st-several", "20260108", "444.44")],
        vec![
            contra("batch-4", "t4", "20260108", "444.44"),
            contra("batch-3", "t3", "20260108", "444.44"),
            contra("batch-4", "t5", "20260108", "444.44"),
        ],
    );
    let summary = summary_of(&facts);
    assert_eq!(
        rows_of(&summary),
        vec![
            listed("st-several", "20260108", "ambiguous", Some("batch-3")),
            listed("st-several", "20260108", "ambiguous", Some("batch-4")),
        ]
    );
    // the row is one open skip, however many rows list it
    assert_eq!(open_count(&summary), 1);
}

#[test]
fn a_row_whose_amount_reverses_the_same_day_is_listed_once_for_each_batch_holding_a_candidate() {
    let mut with_two = skip("st-reversed-two", "20260109", "555.55");
    with_two.reversed_same_day = true;
    let mut with_none = skip("st-reversed-none", "20260110", "556.55");
    with_none.reversed_same_day = true;
    let facts = facts_of(
        vec![with_two, with_none],
        vec![
            contra("batch-6", "t6", "20260109", "555.55"),
            contra("batch-5", "t5", "20260109", "555.55"),
        ],
    );
    let summary = summary_of(&facts);
    assert_eq!(
        rows_of(&summary),
        vec![
            listed(
                "st-reversed-two",
                "20260109",
                "reversed_same_day",
                Some("batch-5")
            ),
            listed(
                "st-reversed-two",
                "20260109",
                "reversed_same_day",
                Some("batch-6")
            ),
            // nothing matches this one: one row, and no batch
            listed("st-reversed-none", "20260110", "reversed_same_day", None),
        ]
    );
    assert_eq!(open_count(&summary), 2);
}

#[test]
fn two_contras_of_one_batch_list_the_row_once_for_that_batch() {
    let printing = |digits: &str| BankReference::from_recorded(digits);
    let mut reversed = skip("st-reversed", "20260109", "555.55");
    reversed.reversed_same_day = true;
    let mut differs = skip("st-differs", "20260111", "999.99");
    differs.reference = printing("600000000001");
    assert!(differs.reference.is_some());
    let of_another_reference = |txn: &str, digits: &str| {
        let mut other = contra("batch-3", txn, "20260111", "999.99");
        other.reference = printing(digits);
        other
    };
    let facts = facts_of(
        vec![skip("st-several", "20260108", "444.44"), reversed, differs],
        vec![
            contra("batch-1", "t1", "20260108", "444.44"),
            contra("batch-1", "t2", "20260108", "444.44"),
            contra("batch-2", "t3", "20260109", "555.55"),
            contra("batch-2", "t4", "20260109", "555.55"),
            of_another_reference("t5", "600000000002"),
            of_another_reference("t6", "600000000003"),
        ],
    );
    let summary = summary_of(&facts);
    assert_eq!(
        rows_of(&summary),
        vec![
            listed("st-several", "20260108", "ambiguous", Some("batch-1")),
            listed(
                "st-reversed",
                "20260109",
                "reversed_same_day",
                Some("batch-2")
            ),
            listed(
                "st-differs",
                "20260111",
                "reference_differs",
                Some("batch-3")
            ),
        ]
    );
    assert_eq!(open_count(&summary), 3);
}

#[test]
fn a_row_of_such_a_state_with_no_candidate_keeps_one_row_without_a_batch() {
    let facts = facts_of(
        vec![
            skip("st-a", "20260108", "444.44"),
            skip("st-b", "20260109", "555.55"),
        ],
        Vec::new(),
    );
    let view = TransferView {
        skips: BTreeMap::from([
            (
                skip_key("st-a"),
                SkipState::Ambiguous {
                    candidates: Vec::new(),
                },
            ),
            (
                skip_key("st-b"),
                SkipState::ReversedSameDay {
                    candidates: Vec::new(),
                },
            ),
        ]),
        double_booked: Vec::new(),
    };
    let summary = TransferSummary::of(&facts, &view);
    assert_eq!(
        rows_of(&summary),
        vec![
            listed("st-a", "20260108", "ambiguous", None),
            listed("st-b", "20260109", "reversed_same_day", None),
        ]
    );
    assert_eq!(open_count(&summary), 2);
}

#[test]
fn a_rendered_section_names_rows_by_id_and_date_and_says_no_amount_or_ledger() {
    let section = summary_of(&every_state()).render(usize::MAX);
    assert_eq!(section["state"], "computed");
    assert_eq!(section["skips_paired"], 1);
    assert_eq!(section["skips_open"], 8);
    assert_eq!(section["retired"], 3);
    assert_eq!(section["open_omitted"], 0);
    let text = section.to_string();
    for secret in [
        "111.11",
        "222.22",
        "333.33",
        "444.44",
        "555.55",
        "666.66",
        "777.77",
        "888.88",
        "999.99",
        "600000000001",
        "600000000002",
        "600000000003",
        "b1",
        "b2",
    ] {
        assert!(!text.contains(secret), "{secret} in {text}");
    }
    // each listed row carries exactly the id, date, state and a batch where it has one
    for row in section["open"].as_array().unwrap() {
        let keys: BTreeSet<&str> = row
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let expected: BTreeSet<&str> = if row["batch_id"].is_null() {
            ["bridge_txn_id", "date", "state"].into()
        } else {
            ["bridge_txn_id", "date", "state", "batch_id"].into()
        };
        assert_eq!(keys, expected, "{row}");
    }
    // a step for each state present, and for none that is not
    let steps: BTreeSet<&str> = section["next_steps"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        steps,
        BTreeSet::from([
            "held_elsewhere",
            "other_side_unverified",
            "not_found",
            "ambiguous",
            "reversed_same_day",
            "double_booked",
            "not_established",
            "matched_unconfirmed",
            "reference_differs",
        ])
    );
}

const EVERY_OPEN_STATE: [OpenState; 9] = [
    OpenState::NotFound,
    OpenState::Ambiguous,
    OpenState::OtherSideUnverified,
    OpenState::NotEstablished,
    OpenState::ReversedSameDay,
    OpenState::MatchedUnconfirmed,
    OpenState::ReferenceDiffers,
    OpenState::HeldElsewhere,
    OpenState::DoubleBooked,
];

/// Each state, read from its place in `EVERY_OPEN_STATE`. A state added to
/// `OpenState` does not compile here until it has an arm, and its arm does not
/// compile until the list is one longer: an index past the end is an error.
fn from_the_list(state: OpenState) -> OpenState {
    match state {
        OpenState::NotFound => EVERY_OPEN_STATE[0],
        OpenState::Ambiguous => EVERY_OPEN_STATE[1],
        OpenState::OtherSideUnverified => EVERY_OPEN_STATE[2],
        OpenState::NotEstablished => EVERY_OPEN_STATE[3],
        OpenState::ReversedSameDay => EVERY_OPEN_STATE[4],
        OpenState::MatchedUnconfirmed => EVERY_OPEN_STATE[5],
        OpenState::ReferenceDiffers => EVERY_OPEN_STATE[6],
        OpenState::HeldElsewhere => EVERY_OPEN_STATE[7],
        OpenState::DoubleBooked => EVERY_OPEN_STATE[8],
    }
}

/// With the labels being distinct (below), a list as long as the arms above
/// holds every state.
#[test]
fn the_list_of_every_open_state_holds_each_state_at_its_own_place() {
    for state in EVERY_OPEN_STATE {
        assert_eq!(from_the_list(state), state);
    }
}

#[test]
fn every_open_state_has_its_own_label_and_its_own_step() {
    let labels: BTreeSet<&str> = EVERY_OPEN_STATE.iter().map(|state| state.label()).collect();
    let steps: BTreeSet<&str> = EVERY_OPEN_STATE.iter().map(|state| state.step()).collect();
    assert_eq!(labels.len(), EVERY_OPEN_STATE.len());
    assert_eq!(steps.len(), EVERY_OPEN_STATE.len());
    // a refused withdrawal uses the section's own labels, though not always the
    // one the section gives the row: a row it reads ambiguous, reversed_same_day
    // or reference_differs is listed in paired_rows as other_side_unverified
    // when one of its candidates is not verified
    assert_eq!(Carrier::Paired.label(), "paired");
    assert_eq!(
        Carrier::OtherSideUnverified.label(),
        OpenState::OtherSideUnverified.label()
    );
    assert_eq!(
        Carrier::MatchedUnconfirmed.label(),
        OpenState::MatchedUnconfirmed.label()
    );
}

#[test]
fn each_next_step_says_what_was_decided() {
    let step = |state: OpenState| state.step();
    assert!(step(OpenState::OtherSideUnverified).contains("Run verify_import on that batch"));
    assert!(step(OpenState::OtherSideUnverified).contains("Do not build or post either statement"));
    assert!(step(OpenState::NotFound).contains("withdraw the skip on exactly one side"));
    assert!(step(OpenState::NotFound).contains("Do not enter this transfer in Tally by hand"));
    assert!(step(OpenState::Ambiguous).contains("does not choose"));
    let reversed = step(OpenState::ReversedSameDay);
    assert!(reversed.contains("a reversal and a pair of real transfers look alike"));
    assert!(reversed.contains("Check in Tally which rows are transfers"));
    assert!(reversed.contains(
        "Name the row in withdraw_skips only when the person confirms the other account's statement does not carry it"
    ));
    // it never says the row is not a transfer
    assert!(!reversed.contains("not a transfer"), "{reversed}");
    assert!(step(OpenState::HeldElsewhere).contains("once it is found posted the skip is retired"));
    assert!(step(OpenState::DoubleBooked).contains("keep one"));
    let open = step(OpenState::NotEstablished);
    assert!(open.contains("Verify any batch of the other account's statement for this date first"));
    assert!(open.contains("build that statement only if no batch of it exists"));
    for state in EVERY_OPEN_STATE {
        // the product is never called by its short name
        let text = step(state);
        assert!(
            !text.contains("Bridge") || text.contains("ComplyEaze Bridge"),
            "{text}"
        );
    }
}

#[test]
fn a_section_that_cannot_be_computed_says_so_and_claims_nothing() {
    let section = TransferSummary::NotEstablished.render(usize::MAX);
    assert_eq!(section["state"], "not_established");
    for claim in ["skips_paired", "skips_open", "open", "retired"] {
        assert!(section.get(claim).is_none(), "{claim} in {section}");
    }
    assert_eq!(
        section["next_steps"],
        json!({"check_not_run": "Not run, or cut to fit: verify again; if it repeats, raise BRIDGE_AGENT_MAX_BYTES."})
    );
}

fn many_open(count: usize) -> TransferSummary {
    TransferSummary::Computed {
        paired: 0,
        skips_open: count,
        retired: 0,
        open: (0..count)
            .map(|n| OpenRow {
                id: format!("st-20260105-{n:016x}"),
                date: date("20260105"),
                state: OpenState::NotFound,
                batch: None,
            })
            .collect(),
    }
}

#[test]
fn a_budget_cuts_the_open_rows_and_counts_what_it_left_out() {
    let summary = many_open(30);
    let everything = summary.render(usize::MAX);
    assert_eq!(everything["open"].as_array().unwrap().len(), 30);
    assert_eq!(everything["open_omitted"], 0);
    assert!(everything["next_steps"].get("open_omitted").is_none());
    let full = everything.to_string().len();
    // a section never exceeds a budget that holds its frame
    for available in [full / 2, full / 3, full / 4] {
        let section = summary.render(available);
        let listed = section["open"].as_array().unwrap().len();
        assert!(listed > 0 && listed < 30, "{available}: {listed}");
        assert_eq!(section["open_omitted"], 30 - listed);
        assert!(section.to_string().len() <= available, "{available}");
        // the rows kept are the first in order, not any that happened to fit
        assert_eq!(
            section["open"][listed - 1]["bridge_txn_id"],
            format!("st-20260105-{:016x}", listed - 1)
        );
        // the count of open skips is not cut, only the list
        assert_eq!(section["skips_open"], 30);
        let step = section["next_steps"]["open_omitted"].as_str().unwrap();
        assert!(step.contains("BRIDGE_AGENT_MAX_BYTES"), "{step}");
    }
    let none = summary.render(0);
    assert_eq!(none["open"], json!([]));
    assert_eq!(none["open_omitted"], 30);
    assert!(none["next_steps"]["open_omitted"].is_string());
}

#[test]
fn the_budget_is_what_the_cap_leaves_counting_the_text_copy_and_at_most_a_quarter() {
    assert_eq!(transfers_budget(200_000, 1_000), 50_000);
    assert_eq!(transfers_budget(200_000, 95_000), 5_000);
    assert_eq!(transfers_budget(200_000, 100_000), 0);
    assert_eq!(transfers_budget(256, 10_000), 0);
}

#[test]
fn withdraw_skips_are_distinct_statement_row_ids() {
    let row = |n: u32| format!("st-20260901-{n:016x}");
    let mut args = json!({"company_guid": "g", "withdraw_skips": [row(2), row(1)]});
    let rows = take_withdraw_skips(&mut args).unwrap();
    assert_eq!(rows, BTreeSet::from([row(1), row(2)]));
    // taken out, so the payload parser never sees it
    assert!(args.get(WITHDRAW_SKIPS_KEY).is_none());
    let mut none = json!({"company_guid": "g"});
    assert!(take_withdraw_skips(&mut none).unwrap().is_empty());
    let mut empty = json!({"withdraw_skips": []});
    assert!(take_withdraw_skips(&mut empty).unwrap().is_empty());
    for bad in [
        json!({"withdraw_skips": [row(1), row(1)]}),
        json!({"withdraw_skips": ["bridge-1"]}),
        json!({"withdraw_skips": ["st-20260901-ABCDEF0123456789"]}),
        json!({"withdraw_skips": [7]}),
        json!({"withdraw_skips": "st-20260901-0000000000000001"}),
        json!({"withdraw_skips": null}),
        json!({"withdraw_skips": (0..=MAX_VOUCHERS as u32).map(row).collect::<Vec<_>>()}),
    ] {
        let mut args = bad.clone();
        assert_eq!(
            take_withdraw_skips(&mut args),
            Err("argument_schema_invalid".to_string()),
            "{bad}"
        );
    }
    // the cap itself is allowed
    let mut cap = json!({"withdraw_skips": (0..MAX_VOUCHERS as u32).map(row).collect::<Vec<_>>()});
    assert_eq!(take_withdraw_skips(&mut cap).unwrap().len(), MAX_VOUCHERS);
}

#[test]
fn a_failure_is_named_by_its_kind_and_record_and_never_by_a_value() {
    use crate::agent::transfer_pairing::PairingInputError;
    use statement_facts::FactsError;
    let facts = TransferFactsError::Facts {
        source_id: "skips-record".into(),
        error: FactsError::SkipLedgersEqual {
            txn: "st-secret-row".into(),
        },
    };
    assert_eq!(facts.variant(), "facts");
    assert_eq!(facts.check(), Some("skip_ledgers_equal"));
    assert_eq!(facts.record_id(), Some("skips-record"));
    let unbound = TransferFactsError::LegLedgerUnbound {
        batch_id: "bridge-x".into(),
        ledger: "A Client Ledger".into(),
    };
    assert_eq!(unbound.record_id(), Some("bridge-x"));
    let in_lineage = TransferFactsError::SkipIsVoucherInLineage {
        lineage: "bridge-y".into(),
        txn: "st-secret-row".into(),
    };
    assert_eq!(in_lineage.record_id(), Some("bridge-y"));
    let unreadable = TransferFactsError::JournalUnreadable {
        code: "import_ledger_invalid".into(),
    };
    assert_eq!(unreadable.record_id(), None);
    let pairing = TransferFactsError::Pairing(PairingInputError::InvertedWindow {
        bank_ledger: LedgerGuid::new("secret-guid"),
    });
    assert_eq!(pairing.variant(), "pairing_inverted_window");
    assert_eq!(pairing.record_id(), None);
    for error in [&facts, &unbound, &in_lineage, &unreadable, &pairing] {
        let named = json!([error.variant(), error.check(), error.record_id()]).to_string();
        for value in [
            "st-secret-row",
            "A Client Ledger",
            "secret-guid",
            "import_ledger_invalid",
        ] {
            assert!(!named.contains(value), "{value} in {named}");
        }
    }
}

fn booked(first: &str, second: &str, class: DoubleBookedClass) -> DoubleBooked {
    let key = |batch: &str| ContraKey {
        batch: BatchId::new(batch),
        txn: TxnId::new("t"),
    };
    DoubleBooked {
        first: key(first),
        second: key(second),
        class,
    }
}

fn view_of(double_booked: Vec<DoubleBooked>) -> TransferView {
    TransferView {
        skips: BTreeMap::new(),
        double_booked,
    }
}

/// The other batch is named whichever of the pair the build's Contra is, and
/// only for a pair that refuses a build.
#[test]
fn a_double_booking_names_the_other_batch_whichever_side_the_build_is() {
    let view = view_of(vec![booked(
        "batch-a",
        "batch-b",
        DoubleBookedClass::RefuseAtBuild,
    )]);
    assert_eq!(double_booked_with(&view, "batch-a"), Some("batch-b"));
    assert_eq!(double_booked_with(&view, "batch-b"), Some("batch-a"));
    assert_eq!(double_booked_with(&view, "batch-c"), None);
    // a pair that is only listed refuses nothing
    let listed = view_of(vec![booked("batch-a", "batch-b", DoubleBookedClass::List)]);
    assert_eq!(double_booked_with(&listed, "batch-a"), None);
    assert_eq!(double_booked_with(&listed, "batch-b"), None);
    // a pair that does not involve the build is not its refusal, but a later one may be
    let two = view_of(vec![
        booked("batch-x", "batch-y", DoubleBookedClass::RefuseAtBuild),
        booked("batch-a", "batch-b", DoubleBookedClass::RefuseAtBuild),
    ]);
    assert_eq!(double_booked_with(&two, "batch-b"), Some("batch-a"));
    assert_eq!(double_booked_with(&view_of(Vec::new()), "batch-a"), None);
}

#[test]
fn trimming_a_section_adds_to_what_was_already_left_out() {
    let TransferSummary::Computed { .. } = many_open(10) else {
        panic!("computed");
    };
    let mut section = many_open(10).render(usize::MAX);
    section["open_omitted"] = json!(5);
    let trimmed = trim_section(&section, 3);
    assert_eq!(trimmed["open"].as_array().unwrap().len(), 3);
    assert_eq!(trimmed["open_omitted"], 12);
    assert!(trimmed["next_steps"]["open_omitted"].is_string());
    // keeping every row of a section that left nothing out changes nothing
    let whole = many_open(10).render(usize::MAX);
    assert_eq!(trim_section(&whole, 10), whole);
    assert_eq!(trim_section(&whole, 99), whole);
    // counts only: no step texts, and no step for rows not cut
    let counts = counts_only_section(&whole);
    assert_eq!(counts["open_omitted"], 10);
    assert_eq!(
        counts["next_steps"]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        ["open_omitted"]
    );
    let empty = many_open(0).render(usize::MAX);
    assert_eq!(counts_only_section(&empty)["next_steps"], json!({}));
}

#[test]
fn a_withdrawal_of_carried_rows_is_told_what_each_state_needs() {
    let step = |kinds: &[Carrier]| Carrier::refusal_step(&kinds.iter().copied().collect());
    let paired = step(&[Carrier::Paired]);
    let waiting = step(&[Carrier::OtherSideUnverified]);
    let matched = step(&[Carrier::MatchedUnconfirmed]);
    let both = step(&[Carrier::OtherSideUnverified, Carrier::Paired]);
    let all = step(&[
        Carrier::Paired,
        Carrier::OtherSideUnverified,
        Carrier::MatchedUnconfirmed,
    ]);
    assert!(paired.contains("already carries this row as a Contra"));
    assert!(!paired.contains("not verified yet"));
    assert!(waiting.contains("which is not verified yet. Run verify_import on that batch first"));
    assert!(!waiting.contains("already carries"));
    assert_eq!(
        both,
        "No file was written and nothing was recorded. For a row with state paired, the other account's statement already carries it as a Contra in the batch named in paired_rows: if that Contra is wrong, correct that statement's batch first. For a row with state other_side_unverified, the other account's statement may carry it as a Contra in the batch named, which is not verified yet: run verify_import on that batch first, or, if it was never imported and will not be, build that statement again, import it and verify it. Do not post these rows."
    );
    // a mix says what each state among the rows needs, and one state alone is
    // told in its own words
    assert_eq!(
        all,
        "No file was written and nothing was recorded. For a row with state paired, the other account's statement already carries it as a Contra in the batch named in paired_rows: if that Contra is wrong, correct that statement's batch first. For a row with state other_side_unverified, the other account's statement may carry it as a Contra in the batch named, which is not verified yet: run verify_import on that batch first, or, if it was never imported and will not be, build that statement again, import it and verify it. For a row with state matched_unconfirmed, a verified Contra of the same day and amount, booked from the other account's statement, is in the batch named, and whether it is the only one is not established: keep the skip. The row can be decided once the statement records of both accounts cover its date, which needs for each account a statement whose rows begin before that date and end after it (a statement that begins or ends on that date does not cover it). Do not post these rows."
    );
    let texts = [&paired, &waiting, &matched, &both, &all];
    assert_eq!(texts.iter().collect::<BTreeSet<_>>().len(), texts.len());
    for step in texts {
        assert!(step.contains("do not post this row") || step.contains("Do not post these rows"));
        assert!(step.starts_with("No file was written and nothing was recorded. "));
    }
}

#[test]
fn a_section_that_did_not_run_names_the_call_that_made_it() {
    let batch = check_not_run_section(false);
    assert_eq!(batch["state"], "not_established");
    assert_eq!(
        batch["next_steps"]["check_not_run"],
        "Not run, or cut to fit: verify again; if it repeats, raise BRIDGE_AGENT_MAX_BYTES."
    );
    assert_eq!(TransferSummary::NotEstablished.render(0), batch);
    let recorded = check_not_run_section(true);
    assert_eq!(
        recorded["next_steps"]["check_not_run"],
        "The skipped rows are recorded; the transfer check will show on the next build or verification for this company."
    );
}
