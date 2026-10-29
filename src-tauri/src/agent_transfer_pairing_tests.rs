//! The pairing view against synthetic facts. Skips are read from bank `b2`'s
//! statement (a transfer to `b1`); Contras come from bank `b1`'s statement.
//! Every value is invented.
use super::*;
use bridge_bank_statement::bank::Bank;
use bridge_bank_statement::parse::Row;

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

/// A synthetic twelve-digit reference, built where it is used: a 6, zeros, then `n`.
fn p6(n: u64, width: usize) -> String {
    format!("6{n:0>w$}", w = width - 1)
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
    // Every skip's own day counts as read whole, so that these tests keep asking
    // what the statements and the pairing say; what a read of Tally adds is
    // asked below, with `view_without_tally`.
    transfer_pairing_view_with(
        skips,
        contras,
        coverage,
        &TallyDays::clean_for(skips),
        k_days,
    )
    .expect("no duplicate keys")
}

/// The view as production builds it: no day of Tally has been read.
fn view_without_tally(
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
        reversed_same_day: false,
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
        reversed_same_day: false,
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
        coverage
            .add(
                CompanyGuid::new("co"),
                EndpointOrigin::new("ep"),
                ledger(bank),
                window("20260101", "20260131"),
            )
            .unwrap();
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

fn matched_unconfirmed(batch: &str, txn: &str) -> SkipState {
    SkipState::MatchedUnconfirmed {
        contra: contra_key(batch, txn),
    }
}

fn reference_differs(candidates: &[(&str, &str)]) -> SkipState {
    SkipState::ReferenceDiffers {
        candidates: candidates
            .iter()
            .map(|(batch, txn)| contra_key(batch, txn))
            .collect(),
    }
}

fn u1() -> String {
    p6(1, 12)
}
fn u2() -> String {
    p6(2, 12)
}
fn u3() -> String {
    p6(3, 12)
}

/// The skipped row, printing the network reference `digits`.
fn skip_printing(row: &str, digits: &str) -> SkipFact {
    let mut fact = skip(row);
    fact.reference = Some(reference(digits));
    fact
}

/// b1's Contra, printing the network reference `digits`.
fn contra_printing(batch: &str, txn: &str, digits: &str) -> ContraFact {
    let mut fact = contra(batch, txn);
    fact.reference = Some(reference(digits));
    fact
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
    s.reference = Some(reference(&p6(1, 12)));
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference(&p6(1, 12)));
    c.date = date("20260109");
    let view = view_of(&[s], &[c], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::Reference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_references_that_differ_never_pair() {
    // equal amount, equal date, a reference on each side, and not the same one:
    // no pair, and the Contra is not said to be absent
    let view = view_of(
        &[skip_printing("st-1", &u1())],
        &[contra_printing("batch-1", "t1", &u2())],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        reference_differs(&[("batch-1", "t1")])
    );
}

#[test]
fn transfer_pairing_a_contra_of_another_reference_is_listed_whatever_is_covered_or_verified() {
    for coverage in [january_covered(), Coverage::default()] {
        for other in [
            contra_printing("batch-1", "t1", &u2()),
            unverified(contra_printing("batch-1", "t1", &u2())),
        ] {
            let view = view_of(&[skip_printing("st-1", &u1())], &[other], &coverage, 0);
            assert_eq!(
                state_of(&view, "st-1"),
                reference_differs(&[("batch-1", "t1")])
            );
        }
    }
    // every such Contra is listed, in key order whatever the input order
    let view = view_of(
        &[skip_printing("st-1", &u1())],
        &[
            contra_printing("batch-2", "t2", &u3()),
            contra_printing("batch-1", "t1", &u2()),
        ],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        reference_differs(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
    // a row flagged as reversed the same day is listed the same way: the
    // flag decides only among candidates found by date
    let mut flagged = skip_printing("st-1", &u1());
    flagged.reversed_same_day = true;
    let view = view_of(
        &[flagged],
        &[contra_printing("batch-1", "t1", &u2())],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        reference_differs(&[("batch-1", "t1")])
    );
}

#[test]
fn transfer_pairing_only_a_contra_the_row_could_match_is_one_of_another_reference() {
    let skipped = skip_printing("st-1", &u1());
    let mut next_day = contra_printing("batch-1", "t1", &u2());
    next_day.date = date("20260106");
    let mut own_statement = contra_printing("batch-1", "t1", &u2());
    own_statement.statement_bank_ledger = ledger("b2");
    let mut own_lineage = contra_printing("batch-1", "t1", &u2());
    own_lineage.lineage = skipped.lineage.clone();
    let mut other_amount = contra_printing("batch-1", "t1", &u2());
    other_amount.amount = ExactDecimal::parse("500.01").unwrap();
    for other in [next_day.clone(), own_statement, own_lineage, other_amount] {
        let view = view_of(
            std::slice::from_ref(&skipped),
            &[other],
            &january_covered(),
            0,
        );
        assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    }
    // the next day is within reach once k is 1
    let view = view_of(&[skipped], &[next_day], &january_covered(), 1);
    assert_eq!(
        state_of(&view, "st-1"),
        reference_differs(&[("batch-1", "t1")])
    );
}

#[test]
fn transfer_pairing_a_contra_paired_with_another_row_is_not_one_of_another_reference() {
    // paired by reference with its own row
    let view = view_of(
        &[skip_printing("st-1", &u1()), skip_printing("st-2", &u2())],
        &[contra_printing("batch-1", "t1", &u1())],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::Reference, "batch-1", "t1")
    );
    assert_eq!(state_of(&view, "st-2"), SkipState::NotFound);
    // paired by date with a row that prints no reference: that row still pairs
    let rows = [skip("st-0"), skip_printing("st-1", &u1())];
    let other = [contra_printing("batch-1", "t1", &u2())];
    let view = view_of(&rows, &other, &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-0"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    // matched by date on a day no window holds, it is still that row's Contra
    let view = view_of(&rows, &other, &Coverage::default(), 0);
    assert_eq!(
        state_of(&view, "st-0"),
        matched_unconfirmed("batch-1", "t1")
    );
    assert_eq!(state_of(&view, "st-1"), SkipState::NotEstablished);
}

#[test]
fn transfer_pairing_two_rows_and_two_contras_each_with_its_own_reference_pair_by_reference() {
    let view = view_of(
        &[skip_printing("st-1", &u1()), skip_printing("st-2", &u2())],
        &[
            contra_printing("batch-1", "t1", &u1()),
            contra_printing("batch-1", "t2", &u2()),
        ],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::Reference, "batch-1", "t1")
    );
    assert_eq!(
        state_of(&view, "st-2"),
        paired(PairRound::Reference, "batch-1", "t2")
    );
}

#[test]
fn transfer_pairing_a_row_with_a_reference_or_date_candidate_gains_none_of_another_reference() {
    // a candidate by reference, not yet verified, beside a verified Contra of
    // another reference: the row is open for the candidate it has
    let view = view_of(
        &[skip_printing("st-1", &u1())],
        &[
            unverified(contra_printing("batch-1", "t1", &u1())),
            contra_printing("batch-2", "t2", &u2()),
        ],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        SkipState::OtherSideUnverified {
            batch: BatchId::new("batch-1")
        }
    );
    // two candidates by date beside a Contra of another reference
    let view = view_of(
        &[skip_printing("st-1", &u1())],
        &[
            contra("batch-1", "t1"),
            contra_printing("batch-2", "t2", &u2()),
            contra("batch-3", "t3"),
        ],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-3", "t3")])
    );
}

#[test]
fn transfer_pairing_a_reference_on_one_side_only_is_a_round_two_edge() {
    let mut s = skip("st-1");
    s.reference = Some(reference(&p6(1, 12)));
    let view = view_of(&[s], &[contra("batch-1", "t1")], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_round_one_needs_one_edge_at_each_end() {
    let mut first = skip("st-1");
    first.reference = Some(reference(&p6(1, 12)));
    let mut second = skip("st-2");
    second.reference = Some(reference(&p6(1, 12)));
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference(&p6(1, 12)));
    let view = view_of(&[first, second], &[c], &january_covered(), 0);
    for row in ["st-1", "st-2"] {
        assert_eq!(state_of(&view, row), ambiguous(&[("batch-1", "t1")]));
    }
    // one skip, two Contras that both carry its reference
    let mut s = skip("st-1");
    s.reference = Some(reference(&p6(1, 12)));
    let mut c1 = contra("batch-1", "t1");
    c1.reference = Some(reference(&p6(1, 12)));
    let mut c2 = contra("batch-2", "t2");
    c2.reference = Some(reference(&p6(1, 12)));
    let view = view_of(&[s], &[c1, c2], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
}

#[test]
fn transfer_pairing_a_row_contested_in_round_one_does_not_pair_in_round_two() {
    // st-1's reference matches two Contras, and a third carries none; the
    // referenceless one is not a way out of the reference ambiguity, but it
    // is a candidate and is listed with the others
    let mut s = skip("st-1");
    s.reference = Some(reference(&p6(1, 12)));
    let mut c1 = contra("batch-1", "t1");
    c1.reference = Some(reference(&p6(1, 12)));
    let mut c2 = contra("batch-2", "t2");
    c2.reference = Some(reference(&p6(1, 12)));
    let c3 = contra("batch-3", "t3");
    let view = view_of(&[s], &[c1, c2, c3], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2"), ("batch-3", "t3")])
    );
}

#[test]
fn transfer_pairing_a_verified_loose_candidate_is_not_hidden_behind_an_unverified_reference() {
    // st-1's reference matches a Contra in a batch not yet verified, and a
    // verified Contra carries no reference at all: both are candidates
    let mut s = skip("st-1");
    s.reference = Some(reference(&p6(1, 12)));
    let mut by_reference = unverified(contra("batch-1", "t1"));
    by_reference.reference = Some(reference(&p6(1, 12)));
    let by_date = contra("batch-2", "t2");
    let view = view_of(&[s], &[by_reference, by_date], &january_covered(), 0);
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
    s.reference = Some(reference(&p6(1, 12)));
    let mut own_statement = contra("batch-1", "t1");
    own_statement.reference = Some(reference(&p6(1, 12)));
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
    // a pair by date claims that nothing else exists, which needs coverage:
    // without it the skip is open, and still names the Contra that matches it
    let view = view_of(&[skip("st-1")], &[contra("batch-1", "t1")], &none, 0);
    assert_eq!(
        state_of(&view, "st-1"),
        matched_unconfirmed("batch-1", "t1")
    );
    // a pair by reference stands on the reference alone
    let mut referenced = skip("st-1");
    referenced.reference = Some(reference(&p6(1, 12)));
    let mut carried = contra("batch-1", "t1");
    carried.reference = Some(reference(&p6(1, 12)));
    let view = view_of(&[referenced], &[carried], &none, 0);
    assert_eq!(
        state_of(&view, "st-1"),
        paired(PairRound::Reference, "batch-1", "t1")
    );
}

fn covering(windows: &[(&str, &str, &str)]) -> Coverage {
    let mut coverage = Coverage::default();
    for (bank, from, to) in windows {
        coverage
            .add(
                CompanyGuid::new("co"),
                EndpointOrigin::new("ep"),
                ledger(bank),
                window(from, to),
            )
            .unwrap();
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
        matched_unconfirmed("batch-1", "t1")
    );
    // b1 holds the skip's 5th-7th, but b2 starts on the 6th and misses the
    // Contra's 4th
    assert_eq!(
        state(covering(&[
            ("b1", "20260101", "20260131"),
            ("b2", "20260106", "20260131")
        ])),
        matched_unconfirmed("batch-1", "t1")
    );
    // b2 holds the Contra's 4th-6th, but b1 stops on the 5th and misses the
    // skip's 7th
    assert_eq!(
        state(covering(&[
            ("b1", "20260101", "20260105"),
            ("b2", "20260101", "20260131")
        ])),
        matched_unconfirmed("batch-1", "t1")
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
    let skipped = skip("st-1");
    // b2's own Contra comes from another lineage than the skipped row's
    let b2_contra = |batch: &str, txn: &str| {
        let mut fact = contra_from_b2(batch, txn);
        fact.lineage = LineageId::new("lineage-b2-amended");
        fact
    };
    // no reference: listed, and the only candidate no longer pairs
    let view = view_of(
        std::slice::from_ref(&skipped),
        &[contra("batch-1", "t1"), b2_contra("batch-2", "t2")],
        &january_covered(),
        0,
    );
    assert_eq!(view.double_booked.len(), 1);
    assert_eq!(state_of(&view, "st-1"), ambiguous(&[("batch-1", "t1")]));
    // equal references: the pair by reference is refused for the same reason
    let mut referenced = skipped;
    referenced.reference = Some(reference(&p6(1, 12)));
    let mut first = contra("batch-1", "t1");
    first.reference = Some(reference(&p6(1, 12)));
    let mut second = b2_contra("batch-2", "t2");
    second.reference = Some(reference(&p6(1, 12)));
    let view = view_of(&[referenced], &[first, second], &january_covered(), 0);
    assert_eq!(view.double_booked.len(), 1);
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
    first.reference = Some(reference(&p6(1, 12)));
    let mut second = skip("st-2");
    second.reference = Some(reference(&p6(1, 12)));
    let third = skip("st-3");
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference(&p6(1, 12)));
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
    referenced.reference = Some(reference(&p6(1, 12)));
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference(&p6(1, 12)));
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
            coverage
                .add(
                    CompanyGuid::new("co"),
                    EndpointOrigin::new("ep"),
                    ledger("b1"),
                    window(from, to),
                )
                .unwrap();
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
    coverage
        .add(
            CompanyGuid::new("co"),
            EndpointOrigin::new("ep"),
            ledger("b2"),
            window.clone(),
        )
        .unwrap();
    coverage
        .add(
            CompanyGuid::new("co-2"),
            EndpointOrigin::new("ep"),
            ledger("b1"),
            window.clone(),
        )
        .unwrap();
    coverage
        .add(
            CompanyGuid::new("co"),
            EndpointOrigin::new("ep-2"),
            ledger("b1"),
            window,
        )
        .unwrap();
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
    s.reference = Some(reference(&p6(1, 12)));
    let mut c = unverified(contra("batch-1", "t1"));
    c.reference = Some(reference(&p6(1, 12)));
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
                SkipState::Paired { contra, .. } | SkipState::MatchedUnconfirmed { contra } => {
                    assert_ne!(contra, &gone);
                }
                SkipState::Ambiguous { candidates }
                | SkipState::ReversedSameDay { candidates }
                | SkipState::ReferenceDiffers { candidates } => {
                    assert!(!candidates.contains(&gone));
                }
                SkipState::OtherSideUnverified { batch } => assert_ne!(batch, &gone.batch),
                SkipState::NotFound | SkipState::TallyUnchecked | SkipState::NotEstablished => {}
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
    first.reference = Some(reference(&p6(1, 12)));
    let mut second = contra_from_b2("batch-2", "t2");
    second.reference = Some(reference(&p6(1, 12)));
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
    one_reference.reference = Some(reference(&p6(1, 12)));
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
    a.reference = Some(reference(&p6(1, 12)));
    let mut b = contra_from_b2("batch-2", "t2");
    b.reference = Some(reference(&p6(2, 12)));
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
    s.reference = Some(reference(&p6(1, 12)));
    s.amount = ExactDecimal::parse("100.00").unwrap();
    let mut c = contra("batch-1", "t1");
    c.reference = Some(reference(&p6(1, 12)));
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
    // a reversed skip, and a reversed Contra beside a plain one
    let mut s = skip("st-8");
    s.amount = ExactDecimal::parse("900.00").unwrap();
    s.reversed_same_day = true;
    skips.push(s);
    let mut s = skip("st-9");
    s.amount = ExactDecimal::parse("950.00").unwrap();
    skips.push(s);
    for (txn, reversed) in [("t7", true), ("t8", false)] {
        let mut c = contra("batch-6", txn);
        c.amount = ExactDecimal::parse("950.00").unwrap();
        c.reversed_same_day = reversed;
        contras.push(c);
    }
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
            coverage
                .add(
                    CompanyGuid::new("co"),
                    EndpointOrigin::new("ep"),
                    ledger("b1"),
                    half.clone(),
                )
                .unwrap();
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
    assert!(has(|state| matches!(
        state,
        SkipState::ReversedSameDay { candidates } if candidates.is_empty()
    )));
    assert!(has(|state| matches!(
        state,
        SkipState::ReversedSameDay { candidates } if candidates.len() == 2
    )));
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

fn refused(skips: &[SkipFact], contras: &[ContraFact]) -> PairingInputError {
    transfer_pairing_view(skips, contras, &january_covered(), 0).expect_err("a bad fact")
}

#[test]
fn transfer_pairing_a_skip_amount_that_is_not_positive_is_refused() {
    for amount in ["0", "0.00", "-0", "-500.00"] {
        let mut bad = skip("st-1");
        bad.amount = ExactDecimal::parse(amount).unwrap();
        assert_eq!(
            refused(&[bad], &[]),
            PairingInputError::NonPositiveSkipAmount {
                key: skip_key("st-1")
            },
            "{amount}"
        );
    }
}

#[test]
fn transfer_pairing_a_contra_amount_that_is_not_positive_is_refused() {
    for amount in ["0", "0.00", "-0", "-500.00"] {
        let mut bad = contra("batch-1", "t1");
        bad.amount = ExactDecimal::parse(amount).unwrap();
        assert_eq!(
            refused(&[skip("st-1")], &[bad]),
            PairingInputError::NonPositiveContraAmount {
                key: contra_key("batch-1", "t1")
            },
            "{amount}"
        );
    }
}

#[test]
fn transfer_pairing_a_skip_whose_two_ledgers_are_one_is_refused() {
    let mut bad = skip("st-1");
    bad.other_ledger = ledger("b2");
    assert_eq!(
        refused(&[bad], &[]),
        PairingInputError::SkipLedgersEqual {
            key: skip_key("st-1")
        }
    );
}

#[test]
fn transfer_pairing_a_window_that_ends_before_it_starts_is_refused() {
    let mut coverage = Coverage::default();
    assert_eq!(
        coverage.add(
            CompanyGuid::new("co"),
            EndpointOrigin::new("ep"),
            ledger("b1"),
            window("20260110", "20260109")
        ),
        Err(PairingInputError::InvertedWindow {
            bank_ledger: ledger("b1")
        })
    );
    // a one-day window is fine
    assert_eq!(
        coverage.add(
            CompanyGuid::new("co"),
            EndpointOrigin::new("ep"),
            ledger("b1"),
            window("20260110", "20260110")
        ),
        Ok(())
    );
}

#[test]
fn transfer_pairing_bad_facts_are_refused_the_same_way_in_any_input_order() {
    let (mut skips, mut contras, coverage) = busy_book();
    // two bad skips (the smaller key is named: amount, then ledgers) and a bad Contra
    skips[4].other_ledger = ledger("b2");
    skips[2].amount = ExactDecimal::parse("0").unwrap();
    contras[3].amount = ExactDecimal::parse("-1").unwrap();
    let named = [
        (
            skips.clone(),
            contras.clone(),
            PairingInputError::NonPositiveSkipAmount {
                key: skips[2].key.clone(),
            },
        ),
        (
            {
                skips[2].amount = ExactDecimal::parse("300.00").unwrap();
                skips.clone()
            },
            contras.clone(),
            PairingInputError::SkipLedgersEqual {
                key: skips[4].key.clone(),
            },
        ),
        (
            {
                skips[4].other_ledger = ledger("b1");
                skips.clone()
            },
            contras.clone(),
            PairingInputError::NonPositiveContraAmount {
                key: contras[3].key.clone(),
            },
        ),
    ];
    for (skips, contras, expected) in named {
        for seed in 0..100 {
            assert_eq!(
                transfer_pairing_view(
                    &shuffled(&skips, seed),
                    &shuffled(&contras, seed + 500),
                    &coverage,
                    1
                ),
                Err(expected.clone()),
                "seed {seed}"
            );
        }
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
fn transfer_pairing_equal_amounts_match_whatever_their_spelling() {
    for spelling in ["500", "500.0", "0500.00", "500.000"] {
        let mut spelled = contra("batch-1", "t1");
        spelled.amount = ExactDecimal::parse(spelling).unwrap();
        let view = view_of(&[skip("st-1")], &[spelled], &january_covered(), 0);
        assert_eq!(
            state_of(&view, "st-1"),
            paired(PairRound::NoReference, "batch-1", "t1"),
            "{spelling}"
        );
    }
    for other in ["50", "5000", "500.5", "0.5"] {
        let mut spelled = contra("batch-1", "t1");
        spelled.amount = ExactDecimal::parse(other).unwrap();
        let view = view_of(&[skip("st-1")], &[spelled], &january_covered(), 0);
        assert_eq!(state_of(&view, "st-1"), SkipState::NotFound, "{other}");
    }
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
        coverage
            .add(
                CompanyGuid::new("co"),
                EndpointOrigin::new("ep-2"),
                ledger(bank),
                window("20260101", "20260131"),
            )
            .unwrap();
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

// ---- a line with an equal amount the other way the same day --------------------

fn reversed_skip(row: &str, direction: Direction) -> SkipFact {
    let mut fact = skip(row);
    fact.direction = direction;
    fact.reversed_same_day = true;
    fact
}

fn reversed_state(candidates: &[(&str, &str)]) -> SkipState {
    SkipState::ReversedSameDay {
        candidates: candidates
            .iter()
            .map(|(batch, txn)| contra_key(batch, txn))
            .collect(),
    }
}

/// A payment from b2 to b1 failed and came straight back on b2's statement
/// (out and in, the same day, the same amount), and the transfer was made again
/// from b1: b1's statement shows one withdrawal. By amount and date alone b2's
/// returned line matches b1's withdrawal. Nothing but a reference may pair it.
#[test]
fn transfer_pairing_a_payment_failed_and_retried_from_the_other_account_pairs_nothing() {
    // b1's one withdrawal debits b2 and credits b1
    let mut withdrawal = contra("batch-1", "t1");
    withdrawal.legs = [
        ContraLeg {
            ledger: ledger("b1"),
            side: Side::Credit,
        },
        ContraLeg {
            ledger: ledger("b2"),
            side: Side::Debit,
        },
    ];
    let out = reversed_skip("st-out", Direction::Outflow);
    let back = reversed_skip("st-back", Direction::Inflow);
    let view = view_of(
        &[out, back.clone()],
        &[withdrawal.clone()],
        &january_covered(),
        0,
    );
    assert_eq!(state_of(&view, "st-out"), reversed_state(&[]));
    assert_eq!(
        state_of(&view, "st-back"),
        reversed_state(&[("batch-1", "t1")])
    );
    assert!(!view
        .skips
        .values()
        .any(|state| matches!(state, SkipState::Paired { .. })));
    // without the flag the returned line pairs with b1's withdrawal, which is
    // the false pair the flag exists to stop
    let mut plain = back;
    plain.reversed_same_day = false;
    let view = view_of(&[plain], &[withdrawal], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-back"),
        paired(PairRound::NoReference, "batch-1", "t1")
    );
}

#[test]
fn transfer_pairing_a_reversed_contra_beside_a_plain_one_pairs_with_neither() {
    let mut flagged = contra("batch-1", "t1");
    flagged.reversed_same_day = true;
    let plain = contra("batch-2", "t2");
    let view = view_of(
        &[skip("st-1")],
        &[flagged.clone(), plain],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        reversed_state(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
    // alone, the reversed Contra is still no pair
    let view = view_of(&[skip("st-1")], &[flagged], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        reversed_state(&[("batch-1", "t1")])
    );
}

#[test]
fn transfer_pairing_a_flag_does_not_veto_an_equal_network_reference() {
    // the flag also holds for a real round trip, so a reference that both
    // statements print still pairs the row
    let mut flagged = contra("batch-1", "t1");
    flagged.reversed_same_day = true;
    flagged.reference = Some(reference(&p6(1, 12)));
    let mut referenced = skip("st-1");
    referenced.reference = Some(reference(&p6(1, 12)));
    for (skip_flag, contra_flag) in [(false, true), (true, false), (true, true)] {
        let mut skip = referenced.clone();
        skip.reversed_same_day = skip_flag;
        let mut contra = flagged.clone();
        contra.reversed_same_day = contra_flag;
        let view = view_of(&[skip], &[contra], &january_covered(), 0);
        assert_eq!(
            state_of(&view, "st-1"),
            paired(PairRound::Reference, "batch-1", "t1")
        );
    }
    // a reference that differs is still no pair, flag or none: the Contra is
    // listed as one of another reference
    let mut other = flagged.clone();
    other.reference = Some(reference(&p6(2, 12)));
    let view = view_of(&[referenced], &[other], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        reference_differs(&[("batch-1", "t1")])
    );
}

/// A transfer from b2 to b1 in the morning and one back in the evening, the
/// same amount: both statements flag all four rows. b2 skipped both of its
/// rows, as carried by b1's two Contras.
fn round_trip(references: bool) -> (Vec<SkipFact>, Vec<ContraFact>) {
    let tag = |fact: &mut Option<BankReference>, digits: &str| {
        *fact = references.then(|| reference(digits));
    };
    let mut out = reversed_skip("st-out", Direction::Outflow);
    tag(&mut out.reference, &p6(1, 12));
    let mut back = reversed_skip("st-back", Direction::Inflow);
    tag(&mut back.reference, &p6(2, 12));
    // b1 received the first and sent the second
    let mut received = contra("batch-1", "t1");
    received.reversed_same_day = true;
    tag(&mut received.reference, &p6(1, 12));
    let mut sent = contra("batch-1", "t2");
    sent.legs = [
        ContraLeg {
            ledger: ledger("b1"),
            side: Side::Credit,
        },
        ContraLeg {
            ledger: ledger("b2"),
            side: Side::Debit,
        },
    ];
    sent.reversed_same_day = true;
    tag(&mut sent.reference, &p6(2, 12));
    (vec![out, back], vec![received, sent])
}

#[test]
fn transfer_pairing_a_round_trip_with_equal_references_pairs_both_legs() {
    let (skips, contras) = round_trip(true);
    let view = view_of(&skips, &contras, &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-out"),
        paired(PairRound::Reference, "batch-1", "t1")
    );
    assert_eq!(
        state_of(&view, "st-back"),
        paired(PairRound::Reference, "batch-1", "t2")
    );
}

#[test]
fn transfer_pairing_a_round_trip_without_references_pairs_neither_leg_and_is_listed() {
    let (skips, contras) = round_trip(false);
    let view = view_of(&skips, &contras, &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-out"),
        reversed_state(&[("batch-1", "t1")])
    );
    assert_eq!(
        state_of(&view, "st-back"),
        reversed_state(&[("batch-1", "t2")])
    );
}

/// A reversed row still counts against the rows that could pair, so a plain
/// skip is not paired to a Contra a reversed skip also matches.
#[test]
fn transfer_pairing_a_reversed_skip_still_counts_against_the_others() {
    let mut flagged = skip("st-1");
    flagged.reversed_same_day = true;
    let view = view_of(
        &[flagged, skip("st-2")],
        &[contra("batch-1", "t1")],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        reversed_state(&[("batch-1", "t1")])
    );
    assert_eq!(state_of(&view, "st-2"), ambiguous(&[("batch-1", "t1")]));
}

#[test]
fn transfer_pairing_a_reversed_row_outranks_an_unverified_candidate() {
    let mut flagged = unverified(contra("batch-1", "t1"));
    flagged.reversed_same_day = true;
    let view = view_of(&[skip("st-1")], &[flagged], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        reversed_state(&[("batch-1", "t1")])
    );
}

/// A flagged row with a reference edge is open for the reason an unflagged one
/// would be: its candidate waits in an unverified batch, or there are several.
/// The flag decides only among candidates found by date.
#[test]
fn transfer_pairing_a_flag_does_not_hide_why_a_referenced_skip_is_open() {
    let referenced = |flagged: bool| {
        let mut fact = skip("st-1");
        fact.reference = Some(reference(&p6(1, 12)));
        fact.reversed_same_day = flagged;
        fact
    };
    let carrying = |batch: &str, txn: &str, flagged: bool| {
        let mut fact = unverified(contra(batch, txn));
        fact.reference = Some(reference(&p6(1, 12)));
        fact.reversed_same_day = flagged;
        fact
    };
    for (skip_flag, contra_flag) in [(true, false), (false, true), (true, true)] {
        let view = view_of(
            &[referenced(skip_flag)],
            &[carrying("batch-1", "t1", contra_flag)],
            &january_covered(),
            0,
        );
        assert_eq!(
            state_of(&view, "st-1"),
            SkipState::OtherSideUnverified {
                batch: BatchId::new("batch-1")
            },
            "{skip_flag} {contra_flag}"
        );
    }
    // two Contras print the reference: ambiguous, flag or none
    let view = view_of(
        &[referenced(true)],
        &[
            carrying("batch-1", "t1", true),
            carrying("batch-2", "t2", false),
        ],
        &january_covered(),
        0,
    );
    assert_eq!(
        state_of(&view, "st-1"),
        ambiguous(&[("batch-1", "t1"), ("batch-2", "t2")])
    );
    // with no reference edge the flag still decides, as before
    let mut plain = carrying("batch-1", "t1", true);
    plain.reference = None;
    let view = view_of(&[skip("st-1")], &[plain], &january_covered(), 0);
    assert_eq!(
        state_of(&view, "st-1"),
        reversed_state(&[("batch-1", "t1")])
    );
}

/// A skip with no candidate whose other ledger's statements cover its date is
/// `NotFound` only when a read of Tally covered the same day with no Contra
/// between the two ledgers; without one it is `TallyUnchecked`; and where the
/// statements do not cover the date it is `NotEstablished` whatever Tally read.
#[test]
fn not_found_needs_a_whole_day_read_of_tally_besides_the_statements() {
    let coverage = january_covered();
    let one = [skip("st-1")];
    // no read of Tally: the production call
    let view = view_without_tally(&one, &[], &coverage, 0);
    assert_eq!(state_of(&view, "st-1"), SkipState::TallyUnchecked);
    // the day read whole between these two ledgers
    let view =
        transfer_pairing_view_with(&one, &[], &coverage, &TallyDays::clean_for(&one), 0).unwrap();
    assert_eq!(state_of(&view, "st-1"), SkipState::NotFound);
    // a read of another day, other ledgers or another company does not count
    for other in [
        {
            let mut s = skip("st-1");
            s.date = date("20260106");
            s
        },
        {
            let mut s = skip("st-1");
            s.other_ledger = ledger("b9");
            s
        },
    ] {
        let view =
            transfer_pairing_view_with(&one, &[], &coverage, &TallyDays::clean_for(&[other]), 0)
                .unwrap();
        assert_eq!(state_of(&view, "st-1"), SkipState::TallyUnchecked);
    }
    // statements that do not cover the date: not established, read or not
    let view = transfer_pairing_view_with(
        &one,
        &[],
        &Coverage::default(),
        &TallyDays::clean_for(&one),
        0,
    )
    .unwrap();
    assert_eq!(state_of(&view, "st-1"), SkipState::NotEstablished);
}

/// A read of Tally changes nothing about a row that has a candidate: only the
/// absence of any candidate is said as `NotFound` or `TallyUnchecked`.
#[test]
fn a_read_of_tally_does_not_change_a_row_with_a_candidate() {
    let coverage = january_covered();
    let one = [skip("st-1")];
    let contras = [contra("batch-1", "t1")];
    let without = view_without_tally(&one, &contras, &coverage, 0);
    let with =
        transfer_pairing_view_with(&one, &contras, &coverage, &TallyDays::clean_for(&one), 0)
            .unwrap();
    assert_eq!(state_of(&without, "st-1"), state_of(&with, "st-1"));
}

/// The whole-day read (#1392), replayed on the answers the lab captured on
/// BRIDGE TRANSFER LAB for 3 to 6 April 2026 (`fixtures/agent/
/// whole-day-read-transfer-lab/PROVENANCE.md`). Every request is checked against
/// the one the lab sent, byte for byte, and every answer is what Tally returned
/// to it; no answer here is written by hand.
mod whole_day {
    use super::*;
    use crate::agent::read_profiles::ReadRequest;
    use crate::agent::voucher_parse::parse_agent_rows_withholding;
    use crate::agent::voucher_window::{parse_voucher_census, WindowReadKind};
    use crate::agent::{Evidence, ToolFailure};
    use crate::tally::VerifiedCompanyIdentity;
    use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;
    use std::sync::Mutex;

    const LAB: &str = "BRIDGE TRANSFER LAB";
    const LAB_GUID: &str = "d6e4c24f-8c7a-4797-93ec-e44bdff60bc9";

    macro_rules! lab {
        ($($name:expr),+) => {
            include_bytes!(concat!(
                "../crates/bridge-tally-protocol/tests/fixtures/agent/whole-day-read-transfer-lab/",
                $($name),+
            ))
            .as_slice()
        };
    }

    type Exchange = (&'static [u8], &'static [u8]);

    fn utf16(bytes: &[u8]) -> String {
        assert_eq!(bytes.len() % 2, 0, "a UTF-16 capture has an even length");
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
        .trim_start_matches('\u{feff}')
        .to_string()
    }

    /// Serves the lab's answers in order, each only to the request the lab sent
    /// for it. A request past the last, or a refusal scripted in place of an
    /// answer (`None`), is how a read fails.
    struct Captured(Mutex<Vec<(&'static [u8], Option<&'static [u8]>)>>);

    impl Captured {
        fn of(exchanges: Vec<Exchange>) -> Self {
            Self(Mutex::new(
                exchanges
                    .into_iter()
                    .map(|(sent, answer)| (sent, Some(answer)))
                    .collect(),
            ))
        }

        fn left(&self) -> usize {
            self.0.lock().unwrap().len()
        }
    }

    impl WindowReader for Captured {
        fn evidence_copies(&self) -> u64 {
            1
        }

        async fn read(
            &self,
            _identity: &VerifiedCompanyIdentity,
            request: ReadRequest,
            _kind: WindowReadKind,
        ) -> Result<(String, Evidence, DateBoundaryProfile), ToolFailure> {
            let (sent, answer) = {
                let mut exchanges = self.0.lock().unwrap();
                assert!(
                    !exchanges.is_empty(),
                    "an extra request: {}",
                    request.as_str()
                );
                exchanges.remove(0)
            };
            assert_eq!(request.as_str(), utf16(sent), "the request the lab sent");
            let answer = answer.ok_or_else(|| ToolFailure::from("scripted_refusal".to_string()))?;
            Ok((
                utf16(answer),
                crate::agent::agent_import::local_evidence("captured"),
                DateBoundaryProfile::ModeAgnostic,
            ))
        }
    }

    /// The reads one day needs, in the order the code sends them: the marks, the
    /// day's census, the day's vouchers.
    macro_rules! lab_day {
        ($day:literal) => {
            vec![
                (lab!("m.request.utf16le.xml"), lab!("m1.utf16le.xml")),
                (
                    lab!($day, "-c1.request.utf16le.xml"),
                    lab!($day, "-c1.utf16le.xml"),
                ),
                (
                    lab!($day, "-v.request.utf16le.xml"),
                    lab!($day, "-v.utf16le.xml"),
                ),
            ]
        };
    }

    fn lab_days() -> [(&'static str, Vec<Exchange>); 4] {
        [
            ("20260403", lab_day!("d03")),
            ("20260404", lab_day!("d04")),
            ("20260405", lab_day!("d05")),
            ("20260406", lab_day!("d06")),
        ]
    }

    fn identity() -> VerifiedCompanyIdentity {
        VerifiedCompanyIdentity::test_fixture(LAB, LAB_GUID)
    }

    fn endpoint() -> EndpointOrigin {
        EndpointOrigin::new("ep")
    }

    /// A plan of exactly these days: skipped rows of the lab company on each.
    fn plan_of(days: &[&str]) -> DayPlan {
        let skips: Vec<SkipFact> = days.iter().map(|day| lab_skip(day, day)).collect();
        DayPlan::of(
            &skips,
            &view_without_tally(&skips, &[], &lab_covered(), 0),
            &CompanyGuid::new(LAB_GUID),
            &endpoint(),
        )
    }

    fn lab_skip(row: &str, day: &str) -> SkipFact {
        let mut skip = skip(row);
        skip.key.company = CompanyGuid::new(LAB_GUID);
        skip.date = date(day);
        skip.other_ledger = ledger("tlab-b");
        skip.key.bank_ledger = ledger("tlab-a");
        skip
    }

    fn lab_covered() -> Coverage {
        covered_between("20260401", "20260430")
    }

    fn covered_between(from: &str, to: &str) -> Coverage {
        let mut coverage = Coverage::default();
        for bank in ["tlab-a", "tlab-b", "tlab-c", "tlab-z"] {
            coverage
                .add(
                    CompanyGuid::new(LAB_GUID),
                    endpoint(),
                    ledger(bank),
                    window(from, to),
                )
                .unwrap();
        }
        coverage
    }

    async fn read_lab(days: &[&str], exchanges: Vec<Exchange>) -> (DayReads, usize) {
        let reader = Captured::of(exchanges);
        let reads = read_days(&reader, &identity(), LAB, &endpoint(), &plan_of(days)).await;
        (reads, reader.left())
    }

    #[test]
    fn each_days_two_censuses_and_both_marks_reads_are_byte_identical() {
        assert_eq!(lab!("d03-c1.utf16le.xml"), lab!("d03-c2.utf16le.xml"));
        assert_eq!(lab!("d04-c1.utf16le.xml"), lab!("d04-c2.utf16le.xml"));
        assert_eq!(lab!("d05-c1.utf16le.xml"), lab!("d05-c2.utf16le.xml"));
        assert_eq!(lab!("d06-c1.utf16le.xml"), lab!("d06-c2.utf16le.xml"));
        assert_eq!(lab!("m1.utf16le.xml"), lab!("m2.utf16le.xml"));
        // the days are not one answer repeated
        assert_ne!(lab!("d03-c1.utf16le.xml"), lab!("d04-c1.utf16le.xml"));
        assert_ne!(lab!("d04-c1.utf16le.xml"), lab!("d05-c1.utf16le.xml"));
    }

    #[tokio::test]
    async fn the_three_days_with_vouchers_are_read_whole_and_agree_with_their_census() {
        let census = |name: &[u8], day: &str| {
            let day = chrono::NaiveDate::parse_from_str(day, "%Y%m%d").unwrap();
            parse_voucher_census(&utf16(name), (day, day), None).unwrap()
        };
        let censuses = [
            (
                "20260403",
                census(lab!("d03-c1.utf16le.xml"), "20260403"),
                3,
            ),
            (
                "20260404",
                census(lab!("d04-c1.utf16le.xml"), "20260404"),
                1,
            ),
            (
                "20260405",
                census(lab!("d05-c1.utf16le.xml"), "20260405"),
                1,
            ),
        ];
        for (day, counted, vouchers) in censuses {
            let exchanges = lab_days()
                .into_iter()
                .find(|(lab_day, _)| *lab_day == day)
                .unwrap()
                .1;
            let (reads, left) = read_lab(&[day], exchanges).await;
            assert_eq!(left, 0, "every captured request was sent, and no other");
            assert!(reads.kept.is_empty(), "{day}: {:?}", reads.kept);
            let [whole] = reads.whole.as_slice() else {
                panic!("{day}: one whole day expected");
            };
            assert_eq!(whole.date(), &date(day));
            assert_eq!(whole.guids().len(), vouchers);
            // census and vouchers name the same vouchers
            assert_eq!(
                whole.guids(),
                counted.iter().map(|row| row.guid.as_str()).collect(),
                "{day}"
            );
            assert_eq!(counted.len(), vouchers);
            // every voucher of the day is a Contra between the two banks
            assert!(whole.has_entries_on("TLab Bank A", "TLab Bank B"));
            assert!(whole.has_entries_on("tlab bank b ", "TLAB BANK A"));
            assert!(!whole.has_entries_on("TLab Bank A", "TLab Bank C"));
        }
    }

    #[tokio::test]
    async fn the_empty_day_is_read_in_full_but_is_not_said_to_be_whole() {
        assert_eq!(lab!("d06-c1.utf16le.xml"), lab!("d06-v.utf16le.xml"));
        let (reads, left) = read_lab(&["20260406"], lab_day!("d06")).await;
        assert_eq!(left, 0);
        assert!(reads.whole.is_empty());
        assert_eq!(
            reads.kept,
            [(date("20260406"), DayKept::NotWhole(NotWhole::Empty))]
        );
    }

    #[tokio::test]
    async fn four_days_are_read_newest_first_and_only_three_are_whole() {
        let mut exchanges = Vec::new();
        for (_, day) in lab_days().into_iter().rev() {
            exchanges.extend(day);
        }
        let (reads, left) =
            read_lab(&["20260403", "20260404", "20260405", "20260406"], exchanges).await;
        assert_eq!(left, 0);
        assert_eq!(
            reads
                .whole
                .iter()
                .map(|day| day.date().as_str())
                .collect::<Vec<_>>(),
            ["20260405", "20260404", "20260403"]
        );
        assert_eq!(reads.kept.len(), 1);
        assert_eq!(reads.kept[0].0, date("20260406"));
    }

    #[tokio::test]
    async fn a_failed_read_keeps_its_day_and_the_days_after_it_are_not_tried() {
        let mut exchanges = lab_day!("d06");
        // the 6 April read gets as far as the census; the vouchers are refused
        exchanges.extend(lab_day!("d05"));
        let reader = Captured::of(exchanges);
        reader.0.lock().unwrap()[2].1 = None;
        let plan = plan_of(&["20260405", "20260406"]);
        let reads = read_days(&reader, &identity(), LAB, &endpoint(), &plan).await;
        assert!(reads.whole.is_empty());
        assert_eq!(
            reads.kept,
            [
                (
                    date("20260406"),
                    DayKept::ReadFailed("scripted_refusal".to_string())
                ),
                (date("20260405"), DayKept::NotAttempted),
            ]
        );
        assert_eq!(reader.left(), 3, "the days after the failure were not sent");
    }

    /// The two answers that make up a captured read of 3 April, as the parts of
    /// a read and its rows: the same inputs `admit` takes from a real outcome.
    fn lab_rows(day_file: &[u8], guid: &str) -> Vec<VoucherRow> {
        parse_agent_rows_withholding(&utf16(day_file), guid).unwrap()
    }

    fn part(from: &str, to: &str) -> WindowPart {
        WindowPart {
            from: date(from),
            to: date(to),
            span: None,
        }
    }

    fn admit(
        day: &str,
        parts: &[WindowPart],
        counted: bool,
        rows: &[VoucherRow],
    ) -> Result<WholeDay, NotWhole> {
        WholeDay::admit(
            CompanyGuid::new(LAB_GUID),
            endpoint(),
            &date(day),
            parts,
            counted,
            rows,
        )
    }

    /// A read that is partial in any one way builds no whole day, whatever else
    /// about it is right. The rows are the 3 April answer's, parsed as the code
    /// parses them.
    #[test]
    fn a_partial_read_can_never_build_the_whole_day() {
        let rows = lab_rows(lab!("d03-v.utf16le.xml"), LAB_GUID);
        assert_eq!(rows.len(), 3);
        // the control: everything holds
        assert!(admit("20260403", &[part("20260403", "20260403")], true, &rows).is_ok());
        // no census counted the day
        assert_eq!(
            admit("20260403", &[part("20260403", "20260403")], false, &rows).unwrap_err(),
            NotWhole::Uncounted
        );
        // a window of more than the day, or of another day, or of no part at all
        for parts in [
            vec![part("20260403", "20260404")],
            vec![part("20260402", "20260403")],
            vec![part("20260404", "20260404")],
            vec![part("20260403", "20260403"), part("20260403", "20260404")],
            vec![],
        ] {
            assert_eq!(
                admit("20260403", &parts, true, &rows).unwrap_err(),
                NotWhole::NotTheDay,
                "{parts:?}"
            );
        }
        // rows that are not the day's
        assert_eq!(
            admit("20260404", &[part("20260404", "20260404")], true, &rows).unwrap_err(),
            NotWhole::NotTheDay
        );
        // no voucher at all
        assert_eq!(
            admit("20260403", &[part("20260403", "20260403")], true, &[]).unwrap_err(),
            NotWhole::Empty
        );
    }

    /// A voucher Tally stored as a foreign-currency composite is withheld by
    /// the read; one withheld voucher keeps the whole day from being whole, even
    /// beside vouchers read in full. The row is the committed capture of one.
    #[test]
    fn a_withheld_voucher_keeps_its_day_from_being_whole() {
        let forex = parse_agent_rows_withholding(
            &utf16(include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-forex-composite-20260915.utf16le.xml"
            )),
            "b14e9b2d-8a63-4779-804d-25d59eb787eb",
        )
        .unwrap();
        assert!(matches!(forex[0], VoucherRow::Withheld(_)));
        let mut rows = lab_rows(lab!("d03-v.utf16le.xml"), LAB_GUID);
        rows.extend(forex);
        assert_eq!(
            admit("20260403", &[part("20260403", "20260403")], true, &rows).unwrap_err(),
            NotWhole::Withheld { vouchers: 1 }
        );
    }

    /// The same refusal on a real outcome: the 3 April read with the census
    /// that admitted it taken away is a read nothing but its own response
    /// counts, and builds nothing.
    #[tokio::test]
    async fn a_real_read_whose_census_is_gone_builds_no_whole_day() {
        let reader = Captured::of(lab_day!("d03"));
        let day = date("20260403");
        let mut read = read_entry_window_rows_with(
            &reader,
            &identity(),
            LAB,
            &day,
            &day,
            VoucherReadShape::EntryWildcard,
            VoucherComposites::Withhold,
        )
        .await
        .unwrap();
        let build = |read: &WindowReadOutcome<VoucherRow>| {
            WholeDay::from_read(CompanyGuid::new(LAB_GUID), endpoint(), &day, read)
        };
        assert!(read.counted());
        assert_eq!(build(&read).unwrap().guids().len(), 3);
        read.witness = None;
        assert_eq!(build(&read).unwrap_err(), NotWhole::Uncounted);
    }

    /// Days of skipped rows that stayed `TallyUnchecked` only, newest first, at
    /// most four and within 31 days of the newest.
    #[test]
    fn the_plan_holds_only_the_unchecked_days_newest_first_within_the_caps() {
        let plan_for = |skips: &[SkipFact], coverage: &Coverage| {
            let view = view_without_tally(skips, &[], coverage, 0);
            DayPlan::of(skips, &view, &CompanyGuid::new(LAB_GUID), &endpoint())
        };
        let texts = |plan: &DayPlan| {
            plan.days()
                .iter()
                .map(|day| day.as_str().to_string())
                .collect::<Vec<_>>()
        };
        // six days: the four newest
        let skips: Vec<SkipFact> = [
            "20260402", "20260410", "20260411", "20260420", "20260421", "20260422",
        ]
        .iter()
        .map(|day| lab_skip(&format!("st-{day}"), day))
        .collect();
        assert_eq!(
            texts(&plan_for(&skips, &lab_covered())),
            ["20260422", "20260421", "20260420", "20260411"]
        );
        assert_eq!(MAX_DAY_READS, 4);
        // two skips on one day are one day
        let mut twice = vec![lab_skip("st-1", "20260410"), lab_skip("st-2", "20260410")];
        twice[1].amount = ExactDecimal::parse("700.00").unwrap();
        assert_eq!(texts(&plan_for(&twice, &lab_covered())), ["20260410"]);
        // 30 days before the newest is inside the 31, 31 days before is not
        let spread = [
            lab_skip("st-1", "20260331"),
            lab_skip("st-2", "20260301"),
            lab_skip("st-3", "20260228"),
        ];
        assert_eq!(
            texts(&plan_for(&spread, &covered_between("20260201", "20260430"))),
            ["20260331", "20260301"]
        );
        assert_eq!(MAX_DAY_SPAN, 31);
        // a row with a candidate, one the statements do not cover and another
        // company's are not asked about
        let mut other_company = lab_skip("st-4", "20260415");
        other_company.key.company = CompanyGuid::new("other");
        let with_candidate = {
            let mut skip = lab_skip("st-5", "20260416");
            skip.reference = None;
            skip
        };
        let mut carried = contra("batch-1", "t1");
        carried.company = CompanyGuid::new(LAB_GUID);
        carried.date = date("20260416");
        carried.legs = [
            ContraLeg {
                ledger: ledger("tlab-b"),
                side: Side::Debit,
            },
            ContraLeg {
                ledger: ledger("tlab-a"),
                side: Side::Credit,
            },
        ];
        carried.statement_bank_ledger = ledger("tlab-b");
        let skips = [
            lab_skip("st-1", "20260410"),
            lab_skip("st-2", "20260511"),
            other_company,
            with_candidate,
        ];
        let view = transfer_pairing_view(&skips, &[carried], &lab_covered(), 0).unwrap();
        let plan = DayPlan::of(&skips, &view, &CompanyGuid::new(LAB_GUID), &endpoint());
        assert_eq!(texts(&plan), ["20260410"]);
        assert!(
            DayPlan::of(&[], &view, &CompanyGuid::new(LAB_GUID), &endpoint())
                .days()
                .is_empty()
        );
    }

    /// What a whole day does to the view: a skip is `NotFound` only when its
    /// own day was read whole and holds no voucher with entries on both of its
    /// ledgers. The day is the captured 4 April, a Contra between the two banks.
    #[tokio::test]
    async fn a_whole_day_makes_a_row_not_found_only_where_it_holds_no_entry_on_both_ledgers() {
        let (reads, _) = read_lab(&["20260404"], lab_day!("d04")).await;
        let name_of = |guid: &LedgerGuid| match guid.as_str() {
            "tlab-a" => Some("TLab Bank A".to_string()),
            "tlab-b" => Some("TLab Bank B".to_string()),
            "tlab-c" => Some("TLab Bank C".to_string()),
            _ => None,
        };
        let between_the_banks = lab_skip("st-ab", "20260404");
        let mut to_c = lab_skip("st-ac", "20260404");
        to_c.other_ledger = ledger("tlab-c");
        let mut unread_day = lab_skip("st-day", "20260405");
        unread_day.other_ledger = ledger("tlab-c");
        let mut unnamed = lab_skip("st-z", "20260404");
        unnamed.other_ledger = ledger("tlab-z");
        let mut other_company = lab_skip("st-co", "20260404");
        other_company.key.company = CompanyGuid::new("other");
        other_company.other_ledger = ledger("tlab-c");
        let skips = [between_the_banks, to_c, unread_day, unnamed, other_company];
        let tally = TallyDays::from_whole_days(&reads.whole, &skips, name_of);
        let coverage = lab_covered();
        let view = transfer_pairing_view_with(&skips, &[], &coverage, &tally, 0).unwrap();
        let state = |row: &str| {
            view.skips
                .iter()
                .find(|(key, _)| key.row.as_str() == row)
                .map(|(_, state)| state.clone())
                .unwrap()
        };
        assert_eq!(state("st-ab"), SkipState::TallyUnchecked);
        assert_eq!(state("st-ac"), SkipState::NotFound);
        assert_eq!(state("st-day"), SkipState::TallyUnchecked);
        assert_eq!(state("st-z"), SkipState::TallyUnchecked);
        // another company's skip is not covered by this company's day
        assert_ne!(state("st-co"), SkipState::NotFound);
        // no day read at all is the production call, and it keeps every row
        let none = TallyDays::from_whole_days(&[], &skips, name_of);
        let view = transfer_pairing_view_with(&skips, &[], &coverage, &none, 0).unwrap();
        assert!(view
            .skips
            .values()
            .all(|state| *state != SkipState::NotFound));
    }
}
