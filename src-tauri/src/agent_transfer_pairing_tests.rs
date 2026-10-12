//! The pairing view against synthetic facts. Skips are read from bank `b2`'s
//! statement (a transfer to `b1`); Contras come from bank `b1`'s statement.
//! Every value is invented.
use super::*;
use bridge_bank_statement::bank::Bank;
use bridge_bank_statement::parse::Row;

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

fn reference(digits: &str) -> BankReference {
    let row = Row::from_pairs([
        (
            "narr",
            format!("UPI/DR/{digits}/ACME/ZZZZ/nw@okzz/Payment").as_str(),
        ),
        ("ref", ""),
    ]);
    Bank::Sbi
        .transfer_reference(&row)
        .expect("a twelve-digit UPI reference")
}

fn ledger(name: &str) -> LedgerGuid {
    LedgerGuid::new(name)
}

fn view_of(
    skips: &[SkipFact],
    contras: &[ContraFact],
    coverage: &Coverage,
    k_days: u32,
) -> TransferView {
    transfer_pairing_view(skips, contras, coverage, k_days).expect("no duplicate keys")
}

fn skip_key(row: &str) -> SkipKey {
    SkipKey {
        company: CompanyGuid::new("co"),
        bank_ledger: ledger("b2"),
        endpoint_origin: EndpointOrigin::new("ep"),
        row: StatementRowId::new(row),
    }
}

fn contra_key(batch: &str, txn: &str) -> ContraKey {
    ContraKey {
        batch: BatchId::new(batch),
        txn: TxnId::new(txn),
    }
}

/// A 500.00 withdrawal from b2 on 20260105, skipped because b1 received it.
fn skip(row: &str) -> SkipFact {
    SkipFact {
        key: skip_key(row),
        date: date("20260105"),
        amount: ExactDecimal::parse("500.00").unwrap(),
        direction: Direction::Outflow,
        other_ledger: ledger("b1"),
        reference: None,
        lineage: LineageId::new("lineage-b2"),
    }
}

/// The Contra b1's statement posts for that transfer: it credits b2, debits b1.
fn contra(batch: &str, txn: &str) -> ContraFact {
    ContraFact {
        key: contra_key(batch, txn),
        company: CompanyGuid::new("co"),
        endpoint_origin: EndpointOrigin::new("ep"),
        legs: [
            ContraLeg {
                ledger: ledger("b1"),
                side: Side::Debit,
            },
            ContraLeg {
                ledger: ledger("b2"),
                side: Side::Credit,
            },
        ],
        amount: ExactDecimal::parse("500.00").unwrap(),
        date: date("20260105"),
        reference: None,
        statement_bank_ledger: ledger("b1"),
        lineage: LineageId::new(format!("lineage-{batch}")),
        status: BatchStatus::PostedVerified,
    }
}

fn unverified(mut fact: ContraFact) -> ContraFact {
    fact.status = BatchStatus::NotVerified {
        batch: fact.key.batch.clone(),
    };
    fact
}

fn window(from: &str, to: &str) -> ParseWindow {
    ParseWindow {
        from: date(from),
        to: date(to),
    }
}

/// Both banks' statements for January are verified.
fn january_covered() -> Coverage {
    let mut coverage = Coverage::default();
    for bank in ["b1", "b2"] {
        coverage.add(
            CompanyGuid::new("co"),
            EndpointOrigin::new("ep"),
            ledger(bank),
            window("20260101", "20260131"),
        );
    }
    coverage
}

fn state_of(view: &TransferView, row: &str) -> SkipState {
    view.skips
        .get(&skip_key(row))
        .unwrap_or_else(|| panic!("no state for {row}"))
        .clone()
}

fn paired(round: PairRound, batch: &str, txn: &str) -> SkipState {
    SkipState::Paired {
        round,
        contra: contra_key(batch, txn),
    }
}

fn ambiguous(candidates: &[(&str, &str)]) -> SkipState {
    SkipState::Ambiguous {
        candidates: candidates
            .iter()
            .map(|(batch, txn)| contra_key(batch, txn))
            .collect(),
    }
}

#[test]
fn transfer_pairing_direction_must_match_on_the_skips_own_bank() {
    // b2 -> b1 pairs with the Contra that credits b2
    let view = view_of(
        &[skip("st-1")],
        &[contra("batch-1", "t1")],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
    // the same-amount return transfer b1 -> b2 debits b2 and credits b1
    let mut returned = contra("batch-1", "t1");
    returned.legs = [
        ContraLeg {
            ledger: ledger("b1"),
            side: Side::Credit,
        },
        ContraLeg {
            ledger: ledger("b2"),
            side: Side::Debit,
        },
    ];
    let view = view_of(&[skip("st-1")], &[returned.clone()], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    // an inflow skip is the mirror: it pairs only with the Contra that debits its bank
    let mut inflow = skip("st-1");
    inflow.direction = Direction::Inflow;
    let view = view_of(&[inflow.clone()], &[returned], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
    let view = view_of(&[inflow], &[contra("batch-1", "t1")], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
}

#[test]
fn transfer_pairing_a_contra_needs_exactly_the_two_ledgers_on_opposite_sides() {
    let covered = january_covered();
    let mut third_ledger = contra("batch-1", "t1");
    third_ledger.legs[0].ledger = ledger("b3");
    let mut one_side = contra("batch-1", "t1");
    one_side.legs[0].side = Side::Credit;
    let mut one_ledger_twice = contra("batch-1", "t1");
    one_ledger_twice.legs[0].ledger = ledger("b2");
    one_ledger_twice.legs[0].side = Side::Debit;
    for wrong in [third_ledger, one_side, one_ledger_twice] {
        let view = view_of(&[skip("st-1")], &[wrong], &covered, 0);
        assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    }
    // a different amount, even by a cent, is no edge; a scale difference is
    let mut other_amount = contra("batch-1", "t1");
    other_amount.amount = ExactDecimal::parse("500.01").unwrap();
    let view = view_of(&[skip("st-1")], &[other_amount], &covered, 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    let mut other_scale = contra("batch-1", "t1");
    other_scale.amount = ExactDecimal::parse("500").unwrap();
    let view = view_of(&[skip("st-1")], &[other_scale], &covered, 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_a_contra_in_the_skips_own_lineage_is_no_edge() {
    let mut same_lineage = contra("batch-1", "t1");
    same_lineage.lineage = skip("st-1").lineage;
    let view = view_of(&[skip("st-1")], &[same_lineage], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
}

#[test]
fn transfer_pairing_equal_references_pair_in_round_one_whatever_the_date() {
    let mut s = skip("st-1");
    s.reference = Some(reference("600000000001"));
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference("600000000001"));
    c.date = date("20260109");
    let view = view_of(&[s], &[c], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::Reference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_references_that_differ_never_pair() {
    // equal amount, equal date, a reference on each side, and not the same one
    let mut s = skip("st-1");
    s.reference = Some(reference("600000000001"));
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference("600000000002"));
    let view = view_of(&[s], &[c], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
}

#[test]
fn transfer_pairing_a_reference_on_one_side_only_is_a_round_two_edge() {
    let mut s = skip("st-1");
    s.reference = Some(reference("600000000001"));
    let view = view_of(&[s], &[contra("batch-1", "t1")], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_round_one_needs_one_edge_at_each_end() {
    let mut first = skip("st-1");
    first.reference = Some(reference("600000000001"));
    let mut second = skip("st-2");
    second.reference = Some(reference("600000000001"));
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference("600000000001"));
    let view = view_of(&[first, second], &[c], &january_covered(), 0);
    for row in ["st-1", "st-2"] {
        assert_eq!(state_of(&view, row), ambiguous(&[("batch-1", "t1")]));
    }
    // one skip, two Contras that both carry its reference
    let mut s = skip("st-1");
    s.reference = Some(reference("600000000001"));
    let mut c1 = contra("batch-1", "t1");
    c1.reference = Some(reference("600000000001"));
    let mut c2 = contra("batch-2", "t2");
    c2.reference = Some(reference("600000000001"));
    let view = view_of(&[s], &[c1, c2], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
}

#[test]
fn transfer_pairing_a_row_contested_in_round_one_does_not_pair_in_round_two() {
    // st-1's reference matches two Contras, and a third carries none; the
    // referenceless one is not a way out of the reference ambiguity
    let mut s = skip("st-1");
    s.reference = Some(reference("600000000001"));
    let mut c1 = contra("batch-1", "t1");
    c1.reference = Some(reference("600000000001"));
    let mut c2 = contra("batch-2", "t2");
    c2.reference = Some(reference("600000000001"));
    let c3 = contra("batch-3", "t3");
    let view = view_of(&[s], &[c1, c2, c3], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
}

#[test]
fn transfer_pairing_without_a_reference_one_candidate_each_way_pairs() {
    let view = view_of(
        &[skip("st-1")],
        &[contra("batch-1", "t1")],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_a_second_skip_makes_both_ambiguous() {
    let view = view_of(
        &[skip("st-1"), skip("st-2")],
        &[contra("batch-1", "t1")],
        &january_covered(),
        0,
    );
    for row in ["st-1", "st-2"] {
        assert_eq!(state_of(&view, row), ambiguous(&[("batch-1", "t1")]));
    }
}

#[test]
fn transfer_pairing_a_second_contra_makes_the_skip_ambiguous_with_both_listed() {
    // listed in key order whatever the input order
    let view = view_of(
        &[skip("st-1")],
        &[contra("batch-2", "t2"), contra("batch-1", "t1")],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
}

#[test]
fn transfer_pairing_round_one_takes_only_the_other_banks_statement() {
    // a Contra from b2's own statement carrying the same reference is that
    // statement's own row, never the other side of a b2 skip
    let mut s = skip("st-1");
    s.reference = Some(reference("600000000001"));
    let mut own_statement = contra("batch-1", "t1");
    own_statement.reference = Some(reference("600000000001"));
    own_statement.statement_bank_ledger = ledger("b2");
    let view = view_of(&[s], &[own_statement], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
}

#[test]
fn transfer_pairing_round_two_takes_only_the_other_banks_statement() {
    // a Contra built from b2's own statement is not the other side of a b2 skip
    let mut own_statement = contra("batch-1", "t1");
    own_statement.statement_bank_ledger = ledger("b2");
    let view = view_of(&[skip("st-1")], &[own_statement], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
}

#[test]
fn transfer_pairing_round_two_dates_must_be_within_k_days() {
    let mut next_day = contra("batch-1", "t1");
    next_day.date = date("20260106");
    let mut two_days = contra("batch-2", "t2");
    two_days.date = date("20260107");
    let paired_next = paired(PairRound::NoReference, "batch-1", "t1");
    // k = 0: the next day is no candidate
    let view = view_of(&[skip("st-1")], &[next_day.clone()], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    // k = 1: it is, and two days away still is not
    let view = view_of(
        &[skip("st-1")],
        &[next_day.clone(), two_days.clone()],
        &january_covered(),
        1,
    );
    assert_eq!(state_of(&view, "st-1"), paired_next);
    // k = 2: both are candidates
    let view = view_of(
        &[skip("st-1")],
        &[next_day, two_days],
        &january_covered(),
        2,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
    // the day before counts too
    let mut before = contra("batch-1", "t1");
    before.date = date("20260104");
    let view = view_of(&[skip("st-1")], &[before], &january_covered(), 1);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_a_candidate_in_another_company_or_endpoint_never_pairs() {
    let mut other_company = contra("batch-1", "t1");
    other_company.company = CompanyGuid::new("co-2");
    let mut other_endpoint = contra("batch-2", "t2");
    other_endpoint.endpoint_origin = EndpointOrigin::new("ep-2");
    let view = view_of(
        &[skip("st-1")],
        &[other_company, other_endpoint],
        &january_covered(),
        0,
    );
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    // a skip of another endpoint does not see this endpoint's Contra either
    let mut elsewhere = skip("st-1");
    elsewhere.key.endpoint_origin = EndpointOrigin::new("ep-2");
    let view = view_of(
        &[elsewhere.clone()],
        &[contra("batch-1", "t1")],
        &Coverage::default(),
        0,
    );
    assert_eq!(view.skips[&elsewhere.key], SkipState::NotEstablished);
}

#[test]
fn transfer_pairing_without_coverage_an_absent_contra_is_not_established() {
    let none = Coverage::default();
    let view = view_of(&[skip("st-1")], &[], &none, 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotEstablished);
    let view = view_of(&[skip("st-1")], &[], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    // a pair by date claims that nothing else exists, which needs coverage
    let view = view_of(&[skip("st-1")], &[contra("batch-1", "t1")], &none, 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotEstablished);
    // a pair by reference stands on the reference alone
    let mut referenced = skip("st-1");
    referenced.reference = Some(reference("600000000001"));
    let mut carried = contra("batch-1", "t1");
    carried.reference = Some(reference("600000000001"));
    let view = view_of(&[referenced], &[carried], &none, 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::Reference, "batch-1", "t1")
    );
}

fn covering(windows: &[(&str, &str, &str)]) -> Coverage {
    let mut coverage = Coverage::default();
    for (bank, from, to) in windows {
        coverage.add(
            CompanyGuid::new("co"),
            EndpointOrigin::new("ep"),
            ledger(bank),
            window(from, to),
        );
    }
    coverage
}

#[test]
fn transfer_pairing_a_date_pair_needs_both_ledgers_windows_around_its_dates() {
    // the Contra is on the 5th, the skip on the 6th, k = 1
    let mut c0 = contra("batch-1", "t1");
    c0.date = date("20260105");
    let mut s1 = skip("st-1");
    s1.date = date("20260106");
    let state =
        |coverage: Coverage| state_of(&view_of(&[s1.clone()], &[c0.clone()], &coverage, 1), "st-1");
    // b1 stops on the 5th and b2 starts on the 6th: neither side holds its half
    assert_eq!(
        state(covering(&[
            ("b1", "20260101", "20260105"),
            ("b2", "20260106", "20260131")
        ])),
        SkipState::NotEstablished
    );
    // b1 holds the skip's 5th-7th, but b2 starts on the 6th and misses the
    // Contra's 4th
    assert_eq!(
        state(covering(&[
            ("b1", "20260101", "20260131"),
            ("b2", "20260106", "20260131")
        ])),
        SkipState::NotEstablished
    );
    // b2 holds the Contra's 4th-6th, but b1 stops on the 5th and misses the
    // skip's 7th
    assert_eq!(
        state(covering(&[
            ("b1", "20260101", "20260105"),
            ("b2", "20260101", "20260131")
        ])),
        SkipState::NotEstablished
    );
    // both hold theirs
    assert_eq!(
        state(covering(&[
            ("b1", "20260104", "20260107"),
            ("b2", "20260104", "20260106")
        ])),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_a_double_booked_contra_never_pairs() {
    // b1's statement posted the Contra, and b2's statement posted one too
    let mut other_lineage = skip("st-1");
    other_lineage.lineage = LineageId::new("lineage-b2");
    let b2_contra = |batch: &str, txn: &str| {
        let mut fact = contra_from_b2(batch, txn);
        fact.lineage = LineageId::new("lineage-b2-amended");
        fact
    };
    // no reference: listed, and the only candidate no longer pairs
    let view = view_of(
        &[other_lineage.clone()],
        &[contra("batch-1", "t1"), b2_contra("batch-2", "t2")],
        &january_covered(),
        0,
    );
    assert_eq!(view.double_booked.len(), 1);
    assert_eq!(state_of(&view, "st-1"), ambiguous(&[("batch-1", "t1")]));
    // equal references: the pair by reference is refused for the same reason
    let mut referenced = other_lineage;
    referenced.reference = Some(reference("600000000001"));
    let mut first = contra("batch-1", "t1");
    first.reference = Some(reference("600000000001"));
    let mut second = b2_contra("batch-2", "t2");
    second.reference = Some(reference("600000000001"));
    let view = view_of(&[referenced], &[first, second], &january_covered(), 0);
    assert_eq!(
        view.double_booked[0].class,
        DoubleBookedClass::RefuseAtBuild
    );
    assert_eq!(state_of(&view, "st-1"), ambiguous(&[("batch-1", "t1")]));
}

#[test]
fn transfer_pairing_a_reference_ambiguity_is_not_a_way_in_for_a_referenceless_skip() {
    // two skips and a Contra share one reference; a third skip, with none,
    // matches the Contra by amount and day
    let mut first = skip("st-1");
    first.reference = Some(reference("600000000001"));
    let mut second = skip("st-2");
    second.reference = Some(reference("600000000001"));
    let third = skip("st-3");
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference("600000000001"));
    let view = view_of(&[first, second, third], &[c], &january_covered(), 0);
    for row in ["st-1", "st-2", "st-3"] {
        assert_eq!(
            state_of(&view, row),
            ambiguous(&[("batch-1", "t1")]),
            "{row}"
        );
    }
}

#[test]
fn transfer_pairing_a_contra_paired_by_reference_is_not_a_candidate_by_date() {
    let mut referenced = skip("st-1");
    referenced.reference = Some(reference("600000000001"));
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference("600000000001"));
    let view = view_of(&[referenced, skip("st-2")], &[c], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::Reference, "batch-1", "t1")
    );
    // st-2 matches the same amount and day, but that Contra is spoken for
    assert_eq!(state_of(&view, "st-2"), SkipState::NotFound);
}

#[test]
fn transfer_pairing_a_window_inside_another_does_not_shorten_it() {
    // January 1st-10th with a window nested in it: the 8th is still covered
    let mut eighth = skip("st-1");
    eighth.date = date("20260108");
    for windows in [
        [
            ("b1", "20260101", "20260110"),
            ("b1", "20260102", "20260105"),
        ],
        [
            ("b1", "20260102", "20260105"),
            ("b1", "20260101", "20260110"),
        ],
    ] {
        let view = view_of(&[eighth.clone()], &[], &covering(&windows), 0);
        assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    }
}

#[test]
fn transfer_pairing_coverage_must_hold_the_whole_interval() {
    let skip_on_the_fifth = [skip("st-1")];
    let covered_by = |windows: &[(&str, &str)], k: u32| {
        let mut coverage = Coverage::default();
        for (from, to) in windows {
            coverage.add(
                CompanyGuid::new("co"),
                EndpointOrigin::new("ep"),
                ledger("b1"),
                window(from, to),
            );
        }
        state_of(&view_of(&skip_on_the_fifth, &[], &coverage, k), "st-1")
    };
    // exactly the interval [4th, 6th] for k = 1
    assert_eq!(
        covered_by(&[("20260104", "20260106")], 1),
        SkipState::NotFound
    );
    // one day short at either end
    assert_eq!(
        covered_by(&[("20260105", "20260106")], 1),
        SkipState::NotEstablished
    );
    assert_eq!(
        covered_by(&[("20260104", "20260105")], 1),
        SkipState::NotEstablished
    );
    // two windows that meet cover it between them; a gap does not
    assert_eq!(
        covered_by(&[("20260101", "20260105"), ("20260106", "20260110")], 1),
        SkipState::NotFound
    );
    assert_eq!(
        covered_by(&[("20260101", "20260104"), ("20260106", "20260110")], 1),
        SkipState::NotEstablished
    );
    // overlapping windows, given in either order
    assert_eq!(
        covered_by(&[("20260105", "20260110"), ("20260101", "20260106")], 1),
        SkipState::NotFound
    );
    // k = 0 needs only the day itself
    assert_eq!(
        covered_by(&[("20260105", "20260105")], 0),
        SkipState::NotFound
    );
}

#[test]
fn transfer_pairing_coverage_of_another_ledger_company_or_endpoint_does_not_count() {
    let mut coverage = Coverage::default();
    let window = window("20260101", "20260131");
    // the skip's own bank ledger is not the ledger that must hold the Contra
    coverage.add(
        CompanyGuid::new("co"),
        EndpointOrigin::new("ep"),
        ledger("b2"),
        window.clone(),
    );
    coverage.add(
        CompanyGuid::new("co-2"),
        EndpointOrigin::new("ep"),
        ledger("b1"),
        window.clone(),
    );
    coverage.add(
        CompanyGuid::new("co"),
        EndpointOrigin::new("ep-2"),
        ledger("b1"),
        window,
    );
    let view = view_of(&[skip("st-1")], &[], &coverage, 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotEstablished);
}

#[test]
fn transfer_pairing_an_unverified_contra_never_pairs() {
    let view = view_of(
        &[skip("st-1")],
        &[unverified(contra("batch-1", "t1"))],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        SkipState::OtherSideUnverified {
            batch: BatchId::new("batch-1")
        }
    );
    // not even with equal references, and whether or not anything is covered
    let mut s = skip("st-1");
    s.reference = Some(reference("600000000001"));
    let mut c = unverified(contra("batch-1", "t1"));
    c.reference = Some(reference("600000000001"));
    let view = view_of(&[s], &[c], &Coverage::default(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        SkipState::OtherSideUnverified {
            batch: BatchId::new("batch-1")
        }
    );
}

#[test]
fn transfer_pairing_an_unverified_batch_names_itself_and_not_the_view_batch() {
    let mut c = contra("batch-1", "t1");
    c.status = BatchStatus::NotVerified {
        batch: BatchId::new("batch-9"),
    };
    let view = view_of(&[skip("st-1")], &[c], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        SkipState::OtherSideUnverified {
            batch: BatchId::new("batch-9")
        }
    );
}

#[test]
fn transfer_pairing_a_verified_and_an_unverified_candidate_are_ambiguous() {
    let view = view_of(
        &[skip("st-1")],
        &[contra("batch-1", "t1"), unverified(contra("batch-2", "t2"))],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
}

#[test]
fn transfer_pairing_candidates_all_in_unverified_batches_of_two_are_ambiguous() {
    let view = view_of(
        &[skip("st-1")],
        &[
            unverified(contra("batch-1", "t1")),
            unverified(contra("batch-2", "t2")),
        ],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
}

#[test]
fn transfer_pairing_an_unverified_candidate_outranks_missing_coverage() {
    // the other bank's batch exists but is not verified, so no verified window
    // covers it; the open state must still say where the Contra is
    let view = view_of(
        &[skip("st-1")],
        &[unverified(contra("batch-1", "t1"))],
        &Coverage::default(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        SkipState::OtherSideUnverified {
            batch: BatchId::new("batch-1")
        }
    );
}

#[test]
fn transfer_pairing_removing_the_verified_batch_reopens_the_skip() {
    let skips = [skip("st-1")];
    let covered = january_covered();
    let with = view_of(&skips, &[contra("batch-1", "t1")], &covered, 0);
    assert_eq!(
        state_of(&with, "st-1"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
    // rolled back or amended away: its Contra is not among the facts
    let without = view_of(&skips, &[], &covered, 0);
    assert_eq!(state_of(&without, "st-1"), SkipState::NotFound);
    // failing verification: the same Contra, no longer verified
    let failed = view_of(&skips, &[unverified(contra("batch-1", "t1"))], &covered, 0);
    assert_eq!(
        state_of(&failed, "st-1"),
        SkipState::OtherSideUnverified {
            batch: BatchId::new("batch-1")
        }
    );
}

#[test]
fn transfer_pairing_no_state_names_a_removed_contra() {
    let (skips, contras, coverage) = busy_book();
    for removed in 0..contras.len() {
        let mut kept = contras.clone();
        let gone = kept.remove(removed).key;
        let view = view_of(&skips, &kept, &coverage, 1);
        for state in view.skips.values() {
            match state {
                SkipState::Paired { contra, .. } => assert_ne!(contra, &gone),
                SkipState::Ambiguous { candidates } => assert!(!candidates.contains(&gone)),
                SkipState::OtherSideUnverified { batch } => assert_ne!(batch, &gone.batch),
                SkipState::NotFound | SkipState::NotEstablished => {}
            }
        }
        for found in &view.double_booked {
            assert!(found.first != gone && found.second != gone);
        }
    }
}

fn double_booked_pair(first: &ContraFact, second: &ContraFact) -> Vec<DoubleBooked> {
    view_of(
        &[],
        &[first.clone(), second.clone()],
        &Coverage::default(),
        1,
    )
    .double_booked
}

/// The Contra b2's statement posts for the same transfer: it credits b2 and
/// debits b1 too, but it comes from b2's statement.
fn contra_from_b2(batch: &str, txn: &str) -> ContraFact {
    let mut fact = contra(batch, txn);
    fact.statement_bank_ledger = ledger("b2");
    fact
}

#[test]
fn transfer_pairing_double_booked_equal_references_refuse_the_build() {
    let mut first = contra("batch-1", "t1");
    first.reference = Some(reference("600000000001"));
    let mut second = contra_from_b2("batch-2", "t2");
    second.reference = Some(reference("600000000001"));
    // equal references refuse whatever the dates say
    second.date = date("20260120");
    assert_eq!(
        double_booked_pair(&second, &first),
        vec![DoubleBooked {
            first: contra_key("batch-1", "t1"),
            second: contra_key("batch-2", "t2"),
            class: DoubleBookedClass::RefuseAtBuild,
        }]
    );
}

#[test]
fn transfer_pairing_double_booked_without_a_reference_is_listed_within_k() {
    let first = contra("batch-1", "t1");
    let mut second = contra_from_b2("batch-2", "t2");
    second.date = date("20260106");
    assert_eq!(
        double_booked_pair(&first, &second),
        vec![DoubleBooked {
            first: contra_key("batch-1", "t1"),
            second: contra_key("batch-2", "t2"),
            class: DoubleBookedClass::List,
        }]
    );
    // two days apart with k = 1
    second.date = date("20260107");
    assert!(double_booked_pair(&first, &second).is_empty());
    // a reference on one side only is still no reference to compare
    let mut one_reference = contra_from_b2("batch-2", "t2");
    one_reference.reference = Some(reference("600000000001"));
    assert_eq!(
        double_booked_pair(&first, &one_reference)[0].class,
        DoubleBookedClass::List
    );
}

#[test]
fn transfer_pairing_double_booked_needs_two_statements_and_one_transfer() {
    let first = contra("batch-1", "t1");
    // references that differ are two transfers
    let mut a = contra("batch-1", "t1");
    a.reference = Some(reference("600000000001"));
    let mut b = contra_from_b2("batch-2", "t2");
    b.reference = Some(reference("600000000002"));
    assert!(double_booked_pair(&a, &b).is_empty());
    // the same bank's statement twice is not two statements
    assert!(double_booked_pair(&first, &contra("batch-2", "t2")).is_empty());
    // one lineage is one statement
    let mut same_lineage = contra_from_b2("batch-2", "t2");
    same_lineage.lineage = first.lineage.clone();
    assert!(double_booked_pair(&first, &same_lineage).is_empty());
    // the other way round (b1 -> b2) is a different transfer
    let mut reverse = contra_from_b2("batch-2", "t2");
    reverse.legs[0].side = Side::Credit;
    reverse.legs[1].side = Side::Debit;
    assert!(double_booked_pair(&first, &reverse).is_empty());
    // another amount, company or endpoint
    let mut amount = contra_from_b2("batch-2", "t2");
    amount.amount = ExactDecimal::parse("500.01").unwrap();
    let mut company = contra_from_b2("batch-2", "t2");
    company.company = CompanyGuid::new("co-2");
    let mut endpoint = contra_from_b2("batch-2", "t2");
    endpoint.endpoint_origin = EndpointOrigin::new("ep-2");
    for other in [amount, company, endpoint] {
        assert!(double_booked_pair(&first, &other).is_empty());
    }
}

#[test]
fn transfer_pairing_double_booked_counts_unverified_batches_too() {
    let first = contra("batch-1", "t1");
    let second = unverified(contra_from_b2("batch-2", "t2"));
    assert_eq!(double_booked_pair(&first, &second).len(), 1);
}

/// A book with a pair of every kind, for the properties.
fn busy_book() -> (Vec<SkipFact>, Vec<ContraFact>, Coverage) {
    let mut skips = Vec::new();
    let mut contras = Vec::new();
    // a reference pair
    let mut s = skip("st-1");
    s.reference = Some(reference("600000000001"));
    s.amount = ExactDecimal::parse("100.00").unwrap();
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference("600000000001"));
    c.amount = ExactDecimal::parse("100.00").unwrap();
    skips.push(s);
    contras.push(c);
    // a referenceless pair a day apart
    let mut s = skip("st-2");
    s.amount = ExactDecimal::parse("200.00").unwrap();
    let mut c = contra("batch-1", "t2");
    c.amount = ExactDecimal::parse("200.00").unwrap();
    c.date = date("20260106");
    skips.push(s);
    contras.push(c);
    // two skips and one Contra: both ambiguous
    for row in ["st-3", "st-4"] {
        let mut s = skip(row);
        s.amount = ExactDecimal::parse("300.00").unwrap();
        skips.push(s);
    }
    let mut c = contra("batch-2", "t3");
    c.amount = ExactDecimal::parse("300.00").unwrap();
    contras.push(c);
    // a Contra in a batch not verified
    let mut s = skip("st-5");
    s.amount = ExactDecimal::parse("400.00").unwrap();
    let mut c = unverified(contra("batch-3", "t4"));
    c.amount = ExactDecimal::parse("400.00").unwrap();
    skips.push(s);
    contras.push(c);
    // nothing for this one, and nothing covers March
    let mut s = skip("st-6");
    s.amount = ExactDecimal::parse("600.00").unwrap();
    skips.push(s);
    let mut s = skip("st-7");
    s.amount = ExactDecimal::parse("700.00").unwrap();
    s.date = date("20260310");
    skips.push(s);
    // the same transfer booked from both statements
    let mut c = contra("batch-4", "t5");
    c.amount = ExactDecimal::parse("800.00").unwrap();
    contras.push(c);
    let mut c = contra_from_b2("batch-5", "t6");
    c.amount = ExactDecimal::parse("800.00").unwrap();
    contras.push(c);
    (skips, contras, january_covered())
}

/// A fixed-seed Fisher-Yates, so a failure replays.
fn shuffled<T: Clone>(items: &[T], seed: u64) -> Vec<T> {
    let mut state = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    let mut out = items.to_vec();
    for index in (1..out.len()).rev() {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let pick = usize::try_from((state >> 33) % (index as u64 + 1)).unwrap();
        out.swap(index, pick);
    }
    out
}

#[test]
fn transfer_pairing_the_result_does_not_depend_on_input_order() {
    let (skips, contras, _) = busy_book();
    // January in two windows, so the order they are added in is exercised too
    let halves = [
        window("20260101", "20260115"),
        window("20260116", "20260131"),
    ];
    let covered = |halves: &[ParseWindow]| {
        let mut coverage = covering(&[("b2", "20260101", "20260131")]);
        for half in halves {
            coverage.add(
                CompanyGuid::new("co"),
                EndpointOrigin::new("ep"),
                ledger("b1"),
                half.clone(),
            );
        }
        coverage
    };
    let baseline = view_of(&skips, &contras, &covered(&halves), 1);
    // the book really holds every state, so the equality below is not vacuous
    let has = |wanted: fn(&SkipState) -> bool| baseline.skips.values().any(wanted);
    assert!(has(|state| matches!(
        state,
        SkipState::Paired {
            round: PairRound::Reference,
            ..
        }
    )));
    assert!(has(|state| matches!(
        state,
        SkipState::Paired {
            round: PairRound::NoReference,
            ..
        }
    )));
    assert!(has(|state| matches!(state, SkipState::Ambiguous { .. })));
    assert!(has(|state| matches!(
        state,
        SkipState::OtherSideUnverified { .. }
    )));
    assert!(has(|state| *state == SkipState::NotFound));
    assert!(has(|state| *state == SkipState::NotEstablished));
    assert_eq!(baseline.double_booked.len(), 1);
    for seed in 0..300 {
        let windows = if seed % 2 == 0 {
            halves.clone()
        } else {
            [halves[1].clone(), halves[0].clone()]
        };
        let view = view_of(
            &shuffled(&skips, seed),
            &shuffled(&contras, seed + 1_000),
            &covered(&windows),
            1,
        );
        assert_eq!(view, baseline, "seed {seed}");
    }
}

/// 2,000 transfers of one amount between one pair of ledgers: one group.
const MANY: usize = 2_000;

fn many_covered() -> Coverage {
    covering(&[
        ("b1", "20191201", "20300101"),
        ("b2", "20191201", "20300101"),
    ])
}

#[test]
fn transfer_pairing_two_thousand_references_in_one_group_each_pair_with_their_own() {
    let reference_of = |index: usize| reference(&format!("6000{index:08}"));
    let skips: Vec<SkipFact> = (0..MANY)
        .map(|index| {
            let mut fact = skip(&format!("st-{index:04}"));
            fact.reference = Some(reference_of(index));
            fact
        })
        .collect();
    let contras: Vec<ContraFact> = (0..MANY)
        .map(|index| {
            let mut fact = contra(&format!("batch-{index:04}"), &format!("t{index:04}"));
            fact.reference = Some(reference_of(index));
            fact
        })
        .collect();
    let view = view_of(
        &shuffled(&skips, 7),
        &shuffled(&contras, 8),
        &many_covered(),
        0,
    );
    assert_eq!(view.skips.len(), MANY);
    for index in 0..MANY {
        assert_eq!(
            state_of(&view, &format!("st-{index:04}")),
            paired(
                PairRound::Reference,
                &format!("batch-{index:04}"),
                &format!("t{index:04}")
            )
        );
    }
    assert!(view.double_booked.is_empty());
}

#[test]
fn transfer_pairing_two_thousand_dated_transfers_in_one_group_each_pair_with_their_own() {
    let day_of = |index: usize| {
        date("20200101")
            .add_days(u32::try_from(index).unwrap())
            .unwrap()
    };
    let skips: Vec<SkipFact> = (0..MANY)
        .map(|index| {
            let mut fact = skip(&format!("st-{index:04}"));
            fact.date = day_of(index);
            fact
        })
        .collect();
    let contras: Vec<ContraFact> = (0..MANY)
        .map(|index| {
            let mut fact = contra(&format!("batch-{index:04}"), &format!("t{index:04}"));
            fact.date = day_of(index);
            fact
        })
        .collect();
    let view = view_of(
        &shuffled(&skips, 9),
        &shuffled(&contras, 10),
        &many_covered(),
        0,
    );
    for index in 0..MANY {
        assert_eq!(
            state_of(&view, &format!("st-{index:04}")),
            paired(
                PairRound::NoReference,
                &format!("batch-{index:04}"),
                &format!("t{index:04}")
            )
        );
    }
    // with k = 1 each day has three neighbours' worth of candidates: the middle
    // ones are ambiguous, so the window really is a range
    let view = view_of(&skips, &contras, &many_covered(), 1);
    assert!(matches!(
        state_of(&view, "st-0100"),
        SkipState::Ambiguous { ref candidates } if candidates.len() == 3
    ));
    assert!(matches!(
        state_of(&view, "st-0000"),
        SkipState::Ambiguous { ref candidates } if candidates.len() == 2
    ));
}

#[test]
fn transfer_pairing_two_thousand_double_booked_contras_in_one_group_are_found() {
    let reference_of = |index: usize| reference(&format!("6000{index:08}"));
    let mut contras = Vec::new();
    for index in 0..MANY {
        let mut first = contra(&format!("batch-a{index:04}"), &format!("t{index:04}"));
        first.reference = Some(reference_of(index));
        let mut second = contra_from_b2(&format!("batch-b{index:04}"), &format!("u{index:04}"));
        second.reference = Some(reference_of(index));
        contras.push(first);
        contras.push(second);
    }
    let view = view_of(&[], &shuffled(&contras, 11), &Coverage::default(), 0);
    assert_eq!(view.double_booked.len(), MANY);
    assert!(view
        .double_booked
        .iter()
        .all(|found| found.class == DoubleBookedClass::RefuseAtBuild));
}

#[test]
fn transfer_pairing_a_contra_from_a_third_banks_statement_is_no_edge() {
    // only the other ledger's own statement carries the other side of the skip
    let mut third = contra("batch-1", "t1");
    third.statement_bank_ledger = ledger("b3");
    let view = view_of(&[skip("st-1")], &[third], &january_covered(), 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
}

#[test]
fn transfer_pairing_two_skips_with_one_key_fail_loudly() {
    let mut again = skip("st-1");
    again.date = date("20260106");
    let duplicate = Err(PairingInputError::DuplicateSkip {
        key: skip_key("st-1"),
    });
    assert_eq!(
        transfer_pairing_view(&[skip("st-1"), again], &[], &january_covered(), 0),
        duplicate
    );
    // an identical fact twice is still two facts for one key
    assert_eq!(
        transfer_pairing_view(&[skip("st-1"), skip("st-1")], &[], &january_covered(), 0),
        duplicate
    );
}

#[test]
fn transfer_pairing_two_contras_with_one_key_fail_loudly() {
    assert_eq!(
        transfer_pairing_view(
            &[skip("st-1")],
            &[contra("batch-1", "t1"), contra("batch-1", "t1")],
            &january_covered(),
            0
        ),
        Err(PairingInputError::DuplicateContra {
            key: contra_key("batch-1", "t1")
        })
    );
}

#[test]
fn transfer_pairing_one_statement_on_two_endpoints_is_two_skips() {
    let first = skip("st-1");
    let mut second = skip("st-1");
    second.key.endpoint_origin = EndpointOrigin::new("ep-2");
    let mut second_contra = contra("batch-2", "t2");
    second_contra.endpoint_origin = EndpointOrigin::new("ep-2");
    let mut coverage = january_covered();
    for bank in ["b1", "b2"] {
        coverage.add(
            CompanyGuid::new("co"),
            EndpointOrigin::new("ep-2"),
            ledger(bank),
            window("20260101", "20260131"),
        );
    }
    let view = view_of(
        &[first.clone(), second.clone()],
        &[contra("batch-1", "t1"), second_contra],
        &coverage,
        0,
    );
    assert_eq!(view.skips.len(), 2);
    assert_eq!(
        view.skips[&first.key],
        paired(PairRound::NoReference, "batch-1", "t1")
    );
    assert_eq!(
        view.skips[&second.key],
        paired(PairRound::NoReference, "batch-2", "t2")
    );
}

#[test]
fn transfer_pairing_duplicates_fail_the_same_way_in_any_input_order() {
    let (mut skips, mut contras, coverage) = busy_book();
    // two duplicated keys: the smaller is the one named, whatever the order
    skips.push(skips[3].clone());
    skips.push(skips[1].clone());
    let named = skips[1].key.clone();
    for seed in 0..100 {
        assert_eq!(
            transfer_pairing_view(
                &shuffled(&skips, seed),
                &shuffled(&contras, seed + 500),
                &coverage,
                1
            ),
            Err(PairingInputError::DuplicateSkip { key: named.clone() }),
            "seed {seed}"
        );
    }
    // and with only a duplicated Contra
    skips.truncate(7);
    contras.push(contras[2].clone());
    contras.push(contras[0].clone());
    let named = contras[0].key.clone();
    for seed in 0..100 {
        assert_eq!(
            transfer_pairing_view(
                &shuffled(&skips, seed),
                &shuffled(&contras, seed + 500),
                &coverage,
                1
            ),
            Err(PairingInputError::DuplicateContra { key: named.clone() }),
            "seed {seed}"
        );
    }
}
