//! The transfer section and the checks around it, on invented facts: what it
//! lists and in what order, within what budget, and what it never says.
use super::*;
use crate::agent::transfer_pairing::transfer_pairing_view_with;
use crate::agent::transfer_pairing::{
    BatchId, BatchStatus, CompanyGuid, ContraFact, ContraKey, ContraLeg, Coverage, Direction,
    DoubleBooked, EndpointOrigin, LedgerGuid, LineageId, ParseWindow, Side, SkipFact, SkipKey,
    StatementRowId, TallyDays, TxnId,
};
use bridge_bank_statement::bank::BankReference;
use statement_facts::HeldSkip;

/// `n` behind zeros to `width` characters.
fn padn(n: u64, width: usize) -> String {
    format!("{n:0>width$}")
}

/// A 6, then `n` behind zeros, `width` characters in all.
fn p6(n: u64, width: usize) -> String {
    format!("6{n:0>w$}", w = width - 1)
}

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
        held: Vec::new(),
        restored: BTreeSet::new(),
        contras_not_two_leg: 0,
        contras_without_facts: 0,
        contras_superseded: 0,
        live_skips: BTreeSet::new(),
        skip_mentions: BTreeSet::new(),
        skip_built: BTreeMap::new(),
    }
}

fn summary_of(facts: &TransferFacts) -> TransferSummary {
    // every skip's own day counts as read whole, so that these tests keep asking
    // what the summary does with each state; production reads days only for a
    // build's tally_unchecked rows (#1392)
    let tally = TallyDays::clean_for(&facts.skips);
    let view = transfer_pairing_view_with(
        &facts.skips,
        &facts.contras,
        &facts.coverage,
        &tally,
        TRANSFER_K_DAYS,
    )
    .unwrap();
    TransferSummary::of(facts, &view)
}

/// The summary when no day of Tally has been read (the admission view, and
/// verify_import's section): a row no statement carries is `tally_unchecked`,
/// never `not_found`.
#[test]
fn without_a_read_of_tally_a_row_no_statement_carries_is_tally_unchecked() {
    let facts = facts_of(vec![skip("st-1", "20260107", "500.00")], Vec::new());
    let view = transfer_pairing_view(
        &facts.skips,
        &facts.contras,
        &facts.coverage,
        TRANSFER_K_DAYS,
    )
    .unwrap();
    let summary = TransferSummary::of(&facts, &view);
    assert_eq!(
        rows_of(&summary),
        vec![(
            "st-1".to_string(),
            "20260107".to_string(),
            "tally_unchecked",
            None
        )]
    );
    let section = summary.render(usize::MAX);
    assert!(section["next_steps"]["tally_unchecked"].is_string());
    assert!(section["next_steps"].get("not_found").is_none());
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
    skips[7].reference = printing(&p6(1, 12));
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
            other.reference = printing(&p6(3, 12));
            other
        },
        {
            let mut other = contra("batch-10", "t10", "20260111", "999.99");
            other.reference = printing(&p6(2, 12));
            other
        },
    ];
    let mut facts = facts_of(skips, contras);
    facts.held = vec![HeldSkip {
        fact: skip("st-held", "20260104", "100.00"),
        holder: BatchId::new("batch-9"),
        by: HeldBy::UnverifiedBatch,
        pairs: true,
    }];
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
    differs.reference = printing(&p6(1, 12));
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
            of_another_reference("t5", &p6(2, 12)),
            of_another_reference("t6", &p6(3, 12)),
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
        &p6(1, 12),
        &p6(2, 12),
        &p6(3, 12),
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

const EVERY_OPEN_STATE: [OpenState; 11] = [
    OpenState::NotFound,
    OpenState::Ambiguous,
    OpenState::OtherSideUnverified,
    OpenState::NotEstablished,
    OpenState::ReversedSameDay,
    OpenState::MatchedUnconfirmed,
    OpenState::ReferenceDiffers,
    OpenState::HeldElsewhere,
    OpenState::DoubleBooked,
    OpenState::PostedAgainstSkip,
    OpenState::TallyUnchecked,
];

/// Each state, read from its place in `EVERY_OPEN_STATE`. A state added to
/// `OpenState` does not compile here until it has an arm, and an arm whose
/// index is past the end of the list does not compile. An arm given an index
/// already in use compiles and leaves the list short: nothing here stops that.
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
        OpenState::PostedAgainstSkip => EVERY_OPEN_STATE[9],
        OpenState::TallyUnchecked => EVERY_OPEN_STATE[10],
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
    assert!(step(OpenState::OtherSideUnverified)
        .contains("Run verify_import on every batch named for this row"));
    assert!(step(OpenState::OtherSideUnverified)
        .contains(" again only if every batch named for this row "));
    assert!(step(OpenState::NotFound).contains("withdraw the skip on exactly one side"));
    assert!(step(OpenState::NotFound).contains("Do not enter this transfer in Tally by hand"));
    assert!(step(OpenState::NotFound)
        .contains("read every voucher of this date in Tally and found no Contra"));
    // the unchecked row is kept until the person confirms, the check names what
    // `vouchers` returns, and the one-side and double-booking warnings stay
    let unchecked = step(OpenState::TallyUnchecked);
    assert!(unchecked.contains("did not read every voucher of this date in Tally"));
    assert!(unchecked
        .contains("look for a voucher of the Contra voucher class, or of any other type such as a Journal or Payment"));
    assert!(unchecked.contains(
        "a voucher dated another day, or booked against another ledger, is not looked for"
    ));
    assert!(unchecked.contains("a masked ledger name can read like another"));
    assert!(unchecked.contains("where it was not dropped"));
    assert!(!unchecked.contains("where it is not masked"));
    assert!(!unchecked.contains("banks often"));
    assert!(unchecked.contains("keep the skip"));
    assert!(unchecked.contains("withdraw the skip on one side only"));
    assert!(unchecked.contains("would book it twice"));
    assert!(unchecked.contains("withdraw_skips only when the person confirms"));
    assert!(!unchecked.contains("bank_allocations"));
    assert!(!unchecked.contains("by hand"));
    assert!(step(OpenState::NotFound).contains("and only when the person confirms"));
    assert!(step(OpenState::Ambiguous).contains("does not choose"));
    let reversed = step(OpenState::ReversedSameDay);
    assert!(reversed.contains("a reversal and a pair of real transfers look alike"));
    assert!(reversed.contains("Check in Tally which rows are transfers"));
    assert!(reversed.contains(
        "Name the row in withdraw_skips only when the person confirms the other account's statement does not carry it"
    ));
    // it never says the row is not a transfer
    assert!(!reversed.contains("not a transfer"), "{reversed}");
    let held = step(OpenState::HeldElsewhere);
    assert!(held.contains(
        "retired once every batch found posted that holds the row has a build in its lineage that recorded the withdrawal"
    ));
    assert!(held.contains(
        "was built after the newest record of the skip (a later build of the skipping statement records the skip again)"
    ));
    assert!(held.contains("posted_against_skip"));
    // the route is conditional on the batch named: one that does not name the
    // row is left unsent, one that does is the build to send and verify
    assert!(held.contains(
        "If the batch named does not name this row in withdraw_skips and has not been sent, build the statement that posts it again naming this row in withdraw_skips, and do not send the batch named"
    ));
    assert!(held.contains("If the batch named does name this row, send it and verify it"));
    assert!(!held.contains(". Do not send"), "{held}");
    // a send of the batch named is refused only once the skip is live; an
    // only-built skip lets it through, so both outcomes are named
    assert!(held.contains(
        "do not send the batch named or import its file: found posted, either leaves the row as posted_against_skip, and once a build that skips the row has been sent or found posted, or a skip-only record holds the skip, post_import refuses that send as skip_row_paired_elsewhere"
    ));
    assert!(!held.contains("sending it gives"), "{held}");
    // and what the route costs
    assert!(held.contains(
        "A batch left unsent blocks the days of its rows for the other account's coverage until it is verified, and no step inside ComplyEaze Bridge clears that"
    ));
    let posted = step(OpenState::PostedAgainstSkip);
    assert!(posted.contains(
        "no build in its lineage that was sent or found posted recorded its withdrawal after the newest record of the skip"
    ));
    assert!(posted.contains("booked twice"));
    // what clears it is what the facts do: a build of that batch's lineage that
    // names the row and is found posted after the newest record of the skip;
    // and a batch ComplyEaze Bridge posted itself cannot be amended
    assert!(posted.contains(
        "leaves this list only when a build in that batch's lineage that names it in withdraw_skips is found posted after the newest record of the skip"
    ));
    assert!(posted.contains("a batch ComplyEaze Bridge posted itself cannot be amended"));
    assert!(!posted.contains("hand"), "{posted}");
    // it describes an exit and does not forbid the build that is the exit
    assert!(!posted.contains("Do not build"), "{posted}");
    let double = step(OpenState::DoubleBooked);
    assert!(double.contains("If either batch named has not been sent to Tally, do not send it"));
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
                restored: false,
            })
            .collect(),
    }
}

#[test]
fn a_budget_cuts_the_open_rows_and_counts_what_it_left_out() {
    // enough rows that they, not the frame's steps, are most of the section
    const ROWS: usize = 120;
    let summary = many_open(ROWS);
    let everything = summary.render(usize::MAX);
    assert_eq!(everything["open"].as_array().unwrap().len(), ROWS);
    assert_eq!(everything["open_omitted"], 0);
    assert!(everything["next_steps"].get("open_omitted").is_none());
    let full = everything.to_string().len();
    // a section never exceeds a budget that holds its frame
    for available in [full / 2, full / 3, full / 4] {
        let section = summary.render(available);
        let listed = section["open"].as_array().unwrap().len();
        assert!(listed > 0 && listed < ROWS, "{available}: {listed}");
        assert_eq!(section["open_omitted"], ROWS - listed);
        assert!(section.to_string().len() <= available, "{available}");
        // the rows kept are the first in order, not any that happened to fit
        assert_eq!(
            section["open"][listed - 1]["bridge_txn_id"],
            format!("st-20260105-{:016x}", listed - 1)
        );
        // the count of open skips is not cut, only the list
        assert_eq!(section["skips_open"], ROWS);
        let step = section["next_steps"]["open_omitted"].as_str().unwrap();
        assert!(step.contains("BRIDGE_AGENT_MAX_BYTES"), "{step}");
    }
    let none = summary.render(0);
    assert_eq!(none["open"], json!([]));
    assert_eq!(none["open_omitted"], ROWS);
    assert!(none["next_steps"]["open_omitted"].is_string());
}

/// A row that names a batch a recorded rollback covers is marked restored, and
/// the note that says what that means is in the frame whenever such a row is
/// open, listed or not, so the budget pays for it (#1050). Of a pair booked
/// twice, the row naming the restored batch is marked. Nothing is marked,
/// and there is no note, when no batch is restored.
#[test]
fn a_row_naming_a_restored_batch_is_marked_and_the_note_is_paid_for() {
    let plain = summary_of(&every_state()).render(usize::MAX);
    assert!(plain["next_steps"].get("restored").is_none());
    assert!(plain["open"]
        .as_array()
        .unwrap()
        .iter()
        .all(|row| row.get("restored").is_none()));
    let mut facts = every_state();
    facts.restored = ["batch-2", "batch-5", "batch-9"].map(str::to_string).into();
    let summary = summary_of(&facts);
    let everything = summary.render(usize::MAX);
    let marked: BTreeSet<(&str, &str)> = everything["open"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row.get("restored").is_some())
        .map(|row| {
            assert_eq!(row["restored"], true);
            (
                row["bridge_txn_id"].as_str().unwrap(),
                row["batch_id"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        marked,
        [
            ("st-unverified", "batch-2"),
            ("st-held", "batch-9"),
            ("t6", "batch-5"),
        ]
        .into()
    );
    assert_eq!(everything["next_steps"]["restored"], RESTORED_NOTE);
    // with no row listed the note is still there, and a budget that holds the
    // frame (the note in it) is never exceeded
    let frame = summary.render(0);
    assert_eq!(frame["open"], json!([]));
    assert_eq!(frame["next_steps"]["restored"], RESTORED_NOTE);
    let (low, high) = (frame.to_string().len() + 64, everything.to_string().len());
    for available in [low, (low + high) / 2, high - 1] {
        let section = summary.render(available);
        assert_eq!(
            section["next_steps"]["restored"], RESTORED_NOTE,
            "{available}"
        );
        assert!(section.to_string().len() <= available, "{available}");
    }
}

/// A withdrawal refused for a row that names a restored batch says what the
/// marker means after its own step, alone or among other kinds.
#[test]
fn a_refusal_naming_a_restored_batch_ends_with_the_note() {
    for kinds in [
        BTreeSet::from([Carrier::OtherSideUnverified]),
        BTreeSet::from([Carrier::OtherSideUnverified, Carrier::Paired]),
    ] {
        let plain = Carrier::refusal_step(&kinds);
        assert_eq!(Carrier::refusal_step_restored(&kinds, false), plain);
        assert_eq!(
            Carrier::refusal_step_restored(&kinds, true),
            format!("{plain} {RESTORED_NOTE}")
        );
    }
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
        json!({"withdraw_skips": &format!("st-20260901-{}", padn(1, 16))}),
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
    assert!(waiting.contains(
        "which is not verified yet. Run verify_import on every batch named for this row first"
    ));
    assert!(!waiting.contains("already carries"));
    assert_eq!(
        both,
        "No file was written and nothing was recorded. For a row with state paired, the other account's statement already carries it as a Contra in the batch named in paired_rows (for a skip that a batch holds, it may carry it: that skip is judged alone, and another skipped row could share that Contra): if that Contra is wrong, correct that statement's batch first. For a row with state other_side_unverified, the other account's statement may carry it as a Contra in a batch named, which is not verified yet: run verify_import on every batch named for this row first; build the other account's statement for this row's date again only if every batch named for this row was never sent to Tally, verify_import listed every one of their vouchers as not_found, and the person confirms none of their files was ever imported into Tally; then post the new build, verify it, and never import an old batch's file; if verify_import found any voucher of a batch named for this row, or refused to read one, or a file of one was imported, do not build the other account's statement again: check in Tally which of their vouchers are there, and no step inside ComplyEaze Bridge frees this row yet; if Tally rejected the send of a batch named for this row, building the other account's statement again is refused as import_txn_already_posted, and no step inside ComplyEaze Bridge frees this row yet. Do not post these rows."
    );
    // a mix says what each state among the rows needs, and one state alone is
    // told in its own words
    assert_eq!(
        all,
        "No file was written and nothing was recorded. For a row with state paired, the other account's statement already carries it as a Contra in the batch named in paired_rows (for a skip that a batch holds, it may carry it: that skip is judged alone, and another skipped row could share that Contra): if that Contra is wrong, correct that statement's batch first. For a row with state other_side_unverified, the other account's statement may carry it as a Contra in a batch named, which is not verified yet: run verify_import on every batch named for this row first; build the other account's statement for this row's date again only if every batch named for this row was never sent to Tally, verify_import listed every one of their vouchers as not_found, and the person confirms none of their files was ever imported into Tally; then post the new build, verify it, and never import an old batch's file; if verify_import found any voucher of a batch named for this row, or refused to read one, or a file of one was imported, do not build the other account's statement again: check in Tally which of their vouchers are there, and no step inside ComplyEaze Bridge frees this row yet; if Tally rejected the send of a batch named for this row, building the other account's statement again is refused as import_txn_already_posted, and no step inside ComplyEaze Bridge frees this row yet. For a row with state matched_unconfirmed, a verified Contra of the same day and amount, booked from the other account's statement, is in the batch named, and whether it is the only one is not established: keep the skip. The row can be decided only once statement records of both accounts cover its date: for each account that needs a statement whose rows begin before that date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on that date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). A statement build abandoned unsent, never sent to Tally and never verified, keeps its days uncovered, and no step inside ComplyEaze Bridge clears one yet. Until then the row stays as it is. Do not post these rows."
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

// ---- a next step names something ComplyEaze Bridge does, or says none does ------

/// Every text a person or an assistant is told to follow about a skipped row.
fn every_step_text() -> Vec<String> {
    let mut texts: Vec<String> = EVERY_OPEN_STATE
        .iter()
        .map(|state| state.step().to_string())
        .collect();
    let kinds = [
        Carrier::Paired,
        Carrier::OtherSideUnverified,
        Carrier::MatchedUnconfirmed,
    ];
    for kind in kinds {
        texts.push(Carrier::refusal_step(&BTreeSet::from([kind])));
    }
    texts.push(Carrier::refusal_step(&kinds.into_iter().collect()));
    texts
}

/// Every sentence a person or an assistant is told to follow about a skipped
/// row, with the five refusals' next steps and the notes of the section.
fn every_sentence_to_follow() -> Vec<String> {
    let mut texts = every_step_text();
    texts.extend(
        [
            SKIP_ROW_ALREADY_POSTED_NEXT_STEP,
            SKIP_ROW_PAIRED_ELSEWHERE_NEXT_STEP,
            WITHDRAW_SKIP_INVALID_NEXT_STEP,
            TRANSFER_DOUBLE_BOOKED_NEXT_STEP,
            TRANSFER_VIEW_NOT_ESTABLISHED_NEXT_STEP,
            NOT_ESTABLISHED_STEP,
            CHECK_NOT_RUN_STEP,
            CHECK_NOT_RUN_RECORDED_STEP,
            OPEN_OMITTED_STEP,
            RESTORED_NOTE,
        ]
        .map(str::to_string),
    );
    texts
        .iter()
        .flat_map(|text| {
            text.replace("; ", ". ")
                .split(". ")
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Every sentence that names Tally is one of the named reads or one of the
/// named prohibitions and disclosures below, word for word. Anything else is a
/// new sentence about Tally and has to be looked at: a step may tell a person
/// to look in Tally, never to enter, edit, delete or import there. A negation
/// somewhere in a sentence is not enough.
#[test]
fn every_sentence_that_names_tally_is_a_prohibition_a_disclosure_or_a_named_read() {
    const READS: [&str; 8] = [
        "Check in Tally whether that batch's voucher for the row is there, and tell the person",
        "Check in Tally which rows are transfers",
        "Name the row in withdraw_skips only when the person confirms that no entry in Tally carries this transfer",
        "If verify_import found any voucher of a batch named for this row, or refused to read one, or a file of one was imported, do not build the other account's statement again: check in Tally which of their vouchers are there, and no step inside ComplyEaze Bridge frees this row yet",
        "Then check that Contra in Tally: keep the skip if it carries this transfer, and name the row in withdraw_skips only when the person confirms it does not",
        "This build skips a row that is already a voucher of a batch of this company: blocking_batch_id names it, and it was sent to Tally, was found posted, or is an earlier build of the batch being amended",
        "check in Tally which Contra carries this transfer",
        "if verify_import found any voucher of a batch named for this row, or refused to read one, or a file of one was imported, do not build the other account's statement again: check in Tally which of their vouchers are there, and no step inside ComplyEaze Bridge frees this row yet",
    ];
    const NEGATIONS_AND_DISCLOSURES: [&str; 16] = [
        "A Contra in a batch not yet verified is not in Tally until that batch is imported and verified",
        "Do not book this transfer in Tally another way, and withdraw the skip on one side only: a later post of the other statement would book it twice",
        "The other account's statement for this date went through ComplyEaze Bridge and holds no Contra for this row dated the same day, and ComplyEaze Bridge read every voucher of this date in Tally and found no Contra between the two accounts that could carry it",
        "The other account's statement for this date went through ComplyEaze Bridge and holds no Contra for this row dated the same day, but whether Tally holds one is not established: ComplyEaze Bridge did not read every voucher of this date in Tally, or could not find one of the two ledgers in the ledger list it read, or read them all and found a voucher of this date with an entry on each of the two ledgers, which it does not compare with this row, so a Contra entered in Tally directly is not ruled out",
        "A row marked restored names a batch whose vouchers a restore that ComplyEaze Bridge recorded may have removed from Tally",
        "A skip cannot take a voucher out of Tally, and it would say that another account's Contra carries a transfer this company already booked",
        "A statement build abandoned unsent, never sent to Tally and never verified, keeps its days uncovered, and no step inside ComplyEaze Bridge clears one yet",
        "Build the other account's statement for this row's date again only if every batch named for this row was never sent to Tally, verify_import listed every one of their vouchers as not_found, and the person confirms none of their files was ever imported into Tally",
        "ComplyEaze Bridge does not compare the two, and no step inside ComplyEaze Bridge removes or changes a voucher already in Tally",
        "Do not enter this transfer in Tally by hand, and never import that batch's file: a later re-post of its rows would book it twice",
        "Do not enter this transfer in Tally by hand: a later post of the other statement would book it twice",
        "If Tally rejected the send of a batch named for this row, building the other account's statement again is refused as import_txn_already_posted, and no step inside ComplyEaze Bridge frees this row yet",
        "If either batch named has not been sent to Tally, do not send it, and do not build or post anything further for this transfer",
        "No step inside ComplyEaze Bridge removes or changes a voucher already in Tally yet",
        "build the other account's statement for this row's date again only if every batch named for this row was never sent to Tally, verify_import listed every one of their vouchers as not_found, and the person confirms none of their files was ever imported into Tally",
        "if Tally rejected the send of a batch named for this row, building the other account's statement again is refused as import_txn_already_posted, and no step inside ComplyEaze Bridge frees this row yet",
    ];
    let mut used = BTreeSet::new();
    for sentence in every_sentence_to_follow() {
        if !sentence.contains("Tally") {
            continue;
        }
        let bare = sentence.trim_end_matches('.');
        let allowed = READS
            .iter()
            .chain(NEGATIONS_AND_DISCLOSURES.iter())
            .find(|allowed| bare == **allowed)
            .unwrap_or_else(|| panic!("a new sentence about Tally: {sentence:?}"));
        used.insert(*allowed);
    }
    // every allowed sentence is still in a text, so the lists do not drift
    assert_eq!(used.len(), READS.len() + NEGATIONS_AND_DISCLOSURES.len());
}

/// The same rule over the tool texts that describe skipped rows: a sentence
/// that names a skip and gives an instruction to enter, edit, delete or import
/// is a prohibition.
#[test]
fn the_tool_texts_about_skipped_rows_give_no_instruction_to_change_tally() {
    let definitions = crate::agent::catalog::registered_tool_definitions(true, true);
    let tools = definitions.as_array().unwrap();
    let text_of = |name: &str| {
        tools
            .iter()
            .find(|tool| tool["name"] == name)
            .unwrap()
            .clone()
    };
    let build = text_of("build_import_xml");
    let texts = [
        text_of("parse_bank_statement")["description"].clone(),
        build["description"].clone(),
        build
            .pointer("/inputSchema/properties/withdraw_skips/description")
            .unwrap()
            .clone(),
    ];
    let mut checked = 0;
    for text in texts {
        let text = text.as_str().unwrap().replace("; ", ". ");
        for sentence in text
            .split(". ")
            .filter(|sentence| sentence.contains("skip"))
        {
            checked += 1;
            let padded = format!(" {sentence} ");
            let instructs = [" enter ", " edit ", " delete ", " import it "]
                .iter()
                .any(|verb| padded.contains(verb));
            let prohibits = [" not ", " no step ", " never "]
                .iter()
                .any(|negation| padded.contains(negation));
            assert!(!instructs || prohibits, "{sentence:?}");
        }
    }
    assert!(checked >= 10, "{checked}");
}

#[test]
fn a_state_with_no_way_out_inside_the_product_says_so() {
    let none = "no step inside complyeaze bridge";
    let lower = |text: &str| text.to_lowercase();
    // two Contras already booked, and a skip a posted voucher contradicts
    assert!(lower(OpenState::DoubleBooked.step()).contains(none));
    assert!(lower(OpenState::PostedAgainstSkip.step()).contains(none));
    // the other statement's send was rejected: a rebuild is refused
    assert!(lower(OpenState::OtherSideUnverified.step()).contains(none));
    assert!(OpenState::OtherSideUnverified.step().contains("rejected"));
    let waiting = Carrier::refusal_step(&BTreeSet::from([Carrier::OtherSideUnverified]));
    assert!(lower(&waiting).contains(none), "{waiting}");
    assert!(waiting.contains("import_txn_already_posted"), "{waiting}");
    // a statement build abandoned unsent keeps its days uncovered for good
    let matched = OpenState::MatchedUnconfirmed.step();
    assert!(lower(matched).contains(none) && matched.contains("abandoned"));
    let carried = Carrier::refusal_step(&BTreeSet::from([Carrier::MatchedUnconfirmed]));
    assert!(lower(&carried).contains(none) && carried.contains("abandoned"));
}

#[test]
fn a_skip_a_posted_voucher_contradicts_is_listed_open_with_the_batch_that_posts_it() {
    let mut facts = facts_of(vec![], vec![]);
    facts.held = vec![HeldSkip {
        fact: skip("st-posted", "20260105", "100.00"),
        holder: BatchId::new("batch-4"),
        by: HeldBy::PostedBatch,
        pairs: true,
    }];
    let summary = summary_of(&facts);
    let TransferSummary::Computed {
        paired,
        skips_open,
        retired,
        ..
    } = &summary
    else {
        panic!("computed");
    };
    assert_eq!((*paired, *skips_open, *retired), (0, 1, 3));
    assert_eq!(
        rows_of(&summary),
        vec![(
            "st-posted".to_string(),
            "20260105".to_string(),
            "posted_against_skip",
            Some("batch-4".to_string())
        )]
    );
    let section = summary.render(usize::MAX);
    assert_eq!(
        section["next_steps"]["posted_against_skip"],
        OpenState::PostedAgainstSkip.step()
    );
}

/// Each failure kind a caller sees as `error` is pinned by its value, so a
/// rename cannot pass unseen.
#[test]
fn every_failure_kind_is_named_by_its_pinned_value() {
    use crate::agent::transfer_pairing::PairingInputError as Pairing;
    use statement_facts::FactsError;
    let skip = skip_key("st-row");
    let contra_key = ContraKey {
        batch: BatchId::new("batch"),
        txn: TxnId::new("txn"),
    };
    let all: Vec<(TransferFactsError, &str)> = vec![
        (
            TransferFactsError::JournalUnreadable { code: "c".into() },
            "journal_unreadable",
        ),
        (
            TransferFactsError::SkipFileUnreadable { code: "c".into() },
            "skip_file_unreadable",
        ),
        (
            TransferFactsError::Facts {
                source_id: "s".into(),
                error: FactsError::Unparsable,
            },
            "facts",
        ),
        (
            TransferFactsError::MissingEndpoint {
                source_id: "s".into(),
            },
            "missing_endpoint",
        ),
        (
            TransferFactsError::BuiltAtUnparsable {
                source_id: "s".into(),
            },
            "built_at_unparsable",
        ),
        (
            TransferFactsError::BatchRecordChanged {
                batch_id: "b".into(),
            },
            "batch_record_changed",
        ),
        (
            TransferFactsError::DuplicateSkipRecordId {
                record_id: "r".into(),
            },
            "duplicate_skip_record_id",
        ),
        (
            TransferFactsError::SkipIsVoucherInLineage {
                lineage: "l".into(),
                txn: "t".into(),
            },
            "skip_is_voucher_in_lineage",
        ),
        (
            TransferFactsError::LegLedgerUnbound {
                batch_id: "b".into(),
                ledger: "l".into(),
            },
            "leg_ledger_unbound",
        ),
        (
            Pairing::DuplicateSkip { key: skip.clone() }.into(),
            "pairing_duplicate_skip",
        ),
        (
            Pairing::DuplicateContra {
                key: contra_key.clone(),
            }
            .into(),
            "pairing_duplicate_contra",
        ),
        (
            Pairing::NonPositiveSkipAmount { key: skip.clone() }.into(),
            "pairing_non_positive_skip",
        ),
        (
            Pairing::NonPositiveContraAmount { key: contra_key }.into(),
            "pairing_non_positive_contra",
        ),
        (
            Pairing::SkipLedgersEqual { key: skip }.into(),
            "pairing_skip_ledgers_equal",
        ),
        (
            Pairing::InvertedWindow {
                bank_ledger: LedgerGuid::new("g"),
            }
            .into(),
            "pairing_inverted_window",
        ),
    ];
    assert_eq!(all.len(), 15);
    for (error, name) in &all {
        assert_eq!(error.variant(), *name);
    }
    let names: BTreeSet<&str> = all.iter().map(|(_, name)| *name).collect();
    assert_eq!(names.len(), all.len());
}

/// A held skip is judged beside the Contras alone, not beside the other skips:
/// two skips of one day and amount compete for one Contra, which would read
/// ambiguous and could be withdrawn.
#[test]
fn a_held_skip_is_judged_without_the_skips_in_the_main_view() {
    let mut facts = facts_of(
        vec![skip("st-kept", "20260105", "100.00")],
        vec![contra("batch-1", "t1", "20260105", "100.00")],
    );
    facts.held = vec![HeldSkip {
        fact: skip("st-held", "20260105", "100.00"),
        holder: BatchId::new("batch-9"),
        by: HeldBy::UnverifiedBatch,
        pairs: true,
    }];
    let alone = held_carrier_view(&facts).unwrap();
    assert!(
        matches!(
            alone.skips.get(&skip_key("st-held")),
            Some(SkipState::Paired { .. })
        ),
        "{:?}",
        alone.skips
    );
    // chained into one view, the two skips share the Contra
    let all: Vec<SkipFact> = facts
        .skips
        .iter()
        .cloned()
        .chain(facts.held.iter().map(|held| held.fact.clone()))
        .collect();
    let combined =
        transfer_pairing_view(&all, &facts.contras, &facts.coverage, TRANSFER_K_DAYS).unwrap();
    assert!(
        !matches!(
            combined.skips.get(&skip_key("st-held")),
            Some(SkipState::Paired { .. })
        ),
        "{:?}",
        combined.skips
    );
}

// ---- what a refusal at send says -------------------------------------------------------

/// A batch about to be sent is a saved batch, not a build in progress: its
/// refusals say nothing was sent, never that a file was not written or that a
/// build posts or skips a row, and end by saying the statement is built again.
#[test]
fn a_refusal_at_send_speaks_of_the_batch_and_not_of_a_build() {
    let build_wording = [
        "This build",
        "this build",
        "in this build",
        "No file was written",
        "nothing was recorded",
    ];
    let mut steps = Vec::new();
    for reason in [
        "skip_row_already_posted",
        "skip_row_paired_elsewhere",
        "withdraw_skip_invalid",
        "transfer_double_booked",
    ] {
        // whatever the build said, the send says its own
        let refusal = TransferRefusal {
            reason,
            detail: json!({}),
            next_step: format!("{NOTHING_WRITTEN} This build posts a row."),
        };
        steps.push((reason, refusal.for_send().into_parts().2));
    }
    for kinds in [
        BTreeSet::from([Carrier::Paired]),
        BTreeSet::from([Carrier::OtherSideUnverified]),
        BTreeSet::from([Carrier::MatchedUnconfirmed]),
        BTreeSet::from([Carrier::Paired, Carrier::OtherSideUnverified]),
    ] {
        let refusal = TransferRefusal {
            reason: "withdraw_skip_paired",
            detail: json!({}),
            next_step: Carrier::refusal_step(&kinds),
        };
        steps.push(("withdraw_skip_paired", refusal.for_send().into_parts().2));
    }
    for (reason, step) in steps {
        assert!(step.starts_with(NOTHING_SENT), "{reason}: {step}");
        // a refused withdrawal names the build it means, since its carrier
        // steps may speak of building the other account's statement
        let ending = if reason == "withdraw_skip_paired" {
            "This batch cannot be sent as built: build this statement again with the rows in paired_rows still skipped, not named in withdraw_skips."
        } else {
            SEND_ENDING
        };
        assert!(step.ends_with(ending), "{reason}: {step}");
        for wording in build_wording {
            assert!(!step.contains(wording), "{reason}: {wording}: {step}");
        }
    }
}

/// The one send refusal that does not end by saying the statement is built
/// again: building it again meets the same unreadable record. It says what the
/// build's step says about a later release, and that nothing was sent.
#[test]
fn the_unreadable_view_refusal_at_send_has_no_build_again_ending() {
    let build = TransferRefusal {
        reason: "transfer_view_not_established",
        detail: json!({}),
        next_step: TRANSFER_VIEW_NOT_ESTABLISHED_NEXT_STEP.into(),
    };
    let step = build.for_send().into_parts().2;
    assert!(step.starts_with(NOTHING_SENT), "{step}");
    assert!(!step.contains(SEND_ENDING), "{step}");
    assert!(!step.contains("build it again"), "{step}");
    assert!(
        step.contains("a later release of ComplyEaze Bridge wrote in this folder"),
        "{step}"
    );
    assert!(step.contains("send with that release"), "{step}");
    assert!(
        step.contains("report it with the error named here"),
        "{step}"
    );
    // and none of the build's own wording
    for wording in [
        "This build",
        "this build",
        "No file was written",
        "nothing was recorded",
    ] {
        assert!(!step.contains(wording), "{wording}: {step}");
    }
}

/// A ledger catalogue of the lab book, as `(name, GUID)` rows under the group
/// Bank Accounts.
///
/// A SYNTHETIC body (never evidence of what Tally answers): the names are the
/// ones the captured voucher rows of 3 to 6 April 2026 spell, and the GUIDs are
/// the ones the lab skips carry.
fn lab_catalogue(ledgers: &[(&str, &str)]) -> bridge_tally_protocol::StandardLedgerCatalogV2 {
    use crate::agent::transfer_pairing::tests::whole_day::{LAB, LAB_GUID};
    let rows = ledgers
        .iter()
        .map(|(name, guid)| {
            format!(
                "<LEDGER NAME=\"{name}\" RESERVEDNAME=\"\"><GUID TYPE=\"String\">{guid}</GUID>\
                 <PARENT TYPE=\"String\">Bank Accounts</PARENT><ISBILLWISEON TYPE=\"Logical\">No</ISBILLWISEON>\
                 <BRIDGECOMPANYGUID TYPE=\"String\">{LAB_GUID}</BRIDGECOMPANYGUID>\
                 <BRIDGECOMPANYNAME TYPE=\"String\">{LAB}</BRIDGECOMPANYNAME></LEDGER>"
            )
        })
        .collect::<String>();
    let xml = format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO>\
         <COMPANY>0</COMPANY></CMPINFO></DESC><DATA><COLLECTION>{rows}</COLLECTION></DATA></BODY></ENVELOPE>"
    );
    bridge_tally_protocol::parse_standard_ledger_catalog_v2_with_identities(&xml, LAB, LAB_GUID)
        .unwrap()
}

/// The section a build returns once its rows' days are read (#1392), with the
/// rows' ledgers named through the catalogue the build read. The day is the
/// lab's captured 4 April, a Contra between TLab Bank A and TLab Bank B. The
/// row to TLab Bank C, which no voucher of the day names, is `not_found`; the
/// row between the two banks stays `tally_unchecked`; and with TLab Bank C
/// missing from the catalogue, both stay `tally_unchecked`.
#[tokio::test]
async fn a_build_names_the_rows_ledgers_from_its_catalogue_for_the_days_it_reads() {
    use crate::agent::transfer_pairing::tests::whole_day::{
        day_d04, identity, lab_covered, lab_skip, lab_skip_to_c, Captured, LAB,
    };
    let admitted = || {
        let mut facts = facts_of(
            vec![
                lab_skip_to_c("st-ac", "20260404"),
                lab_skip("st-ab", "20260404"),
            ],
            Vec::new(),
        );
        facts.coverage = lab_covered();
        Admitted(Some(facts))
    };
    let row = |id: &str, state| (id.to_string(), "20260404".to_string(), state, None);
    let bank_a = ("TLab Bank A", "tlab-a");
    let bank_b = ("TLab Bank B", "tlab-b");
    let bank_c = ("TLab Bank C", "tlab-c");

    let reader = Captured::of(day_d04());
    let (summary, evidence) = admitted()
        .summary_with_days_read(
            &reader,
            &identity(),
            LAB,
            lab_catalogue(&[bank_a, bank_b, bank_c]).catalog(),
        )
        .await;
    assert_eq!(reader.left(), 0, "the day was read, and nothing else");
    assert!(evidence.is_some(), "the day read is evidenced");
    assert_eq!(
        rows_of(&summary),
        vec![row("st-ab", "tally_unchecked"), row("st-ac", "not_found")]
    );

    // the day is read whole, but the row's other ledger has no name
    let reader = Captured::of(day_d04());
    let (summary, _) = admitted()
        .summary_with_days_read(
            &reader,
            &identity(),
            LAB,
            lab_catalogue(&[bank_a, bank_b]).catalog(),
        )
        .await;
    assert_eq!(reader.left(), 0);
    assert_eq!(
        rows_of(&summary),
        vec![
            row("st-ab", "tally_unchecked"),
            row("st-ac", "tally_unchecked")
        ]
    );
}
