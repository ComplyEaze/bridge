//! The journal facts for the transfer pairing view. Every identifier, amount
//! and reference here is synthetic.
use super::*;
use crate::agent::transfer_pairing::{
    transfer_pairing_view, BatchStatus, Direction, PairRound, Side, SkipState,
};
use std::io::Cursor;

const COMPANY: &str = "00000000-0000-4000-8000-0000000000c1";
const OTHER_COMPANY: &str = "00000000-0000-4000-8000-0000000000c2";
const ENDPOINT: &str = "http://127.0.0.1:9001";
const OTHER_ENDPOINT: &str = "http://127.0.0.1:9002";
const B1: &str = "guid-bank-one";
const B2: &str = "guid-bank-two";
const B3: &str = "guid-bank-three";
const N1: &str = "Bank One";
const N2: &str = "Bank Two";
const N3: &str = "Bank Three";
const REF_A: &str = "600000000001";
const REF_B: &str = "600000000002";

fn st(n: u32) -> String {
    format!("st-20260901-{n:016x}")
}

fn batch_id(n: u32) -> String {
    format!("bridge-00000000-0000-4000-8000-{n:012}")
}

fn decimal(text: &str) -> ExactDecimal {
    ExactDecimal::parse(text).unwrap()
}

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

// ---- journal fixtures -------------------------------------------------------

fn voucher(txn: &str, kind: &str, entries: &[(&str, &str, &str)], day: &str) -> Value {
    json!({
        "bridge_txn_id": txn, "date": day, "voucher_type": kind,
        "entries": entries.iter()
            .map(|(ledger, amount, side)| json!({"ledger": ledger, "amount": amount, "side": side}))
            .collect::<Vec<_>>(),
    })
}

/// A transfer into `to` from `from`: debit `to`, credit `from`.
fn contra(txn: &str, to: &str, from: &str, amount: &str) -> Value {
    voucher(
        txn,
        "Contra",
        &[(to, amount, "Dr"), (from, amount, "Cr")],
        "20260901",
    )
}

fn payment(txn: &str, amount: &str) -> Value {
    voucher(
        txn,
        "Payment",
        &[("Expense", amount, "Dr"), (N1, amount, "Cr")],
        "20260901",
    )
}

struct Build {
    id: u32,
    amends: Option<u32>,
    company: &'static str,
    endpoint: Option<&'static str>,
    vouchers: Vec<Value>,
    identities: Option<Vec<(&'static str, &'static str)>>,
    statement: Option<StatementFacts>,
    status: &'static str,
}

fn build(id: u32) -> Build {
    Build {
        id,
        amends: None,
        company: COMPANY,
        endpoint: Some(ENDPOINT),
        vouchers: Vec::new(),
        identities: Some(vec![(N1, B1), (N2, B2), (N3, B3)]),
        statement: None,
        status: "built",
    }
}

impl Build {
    fn amends(mut self, id: u32) -> Self {
        self.amends = Some(id);
        self
    }
    fn company(mut self, company: &'static str) -> Self {
        self.company = company;
        self
    }
    fn endpoint(mut self, endpoint: Option<&'static str>) -> Self {
        self.endpoint = endpoint;
        self
    }
    fn voucher(mut self, voucher: Value) -> Self {
        self.vouchers.push(voucher);
        self
    }
    fn identities(mut self, identities: Option<Vec<(&'static str, &'static str)>>) -> Self {
        self.identities = identities;
        self
    }
    fn facts(mut self, facts: StatementFacts) -> Self {
        self.statement = Some(facts);
        self
    }
    fn status(mut self, status: &'static str) -> Self {
        self.status = status;
        self
    }
    fn line(&self) -> ImportLedgerLine {
        let txn_ids: Vec<String> = self
            .vouchers
            .iter()
            .map(|voucher| voucher["bridge_txn_id"].as_str().unwrap().to_string())
            .collect();
        let mut line: ImportLedgerLine = serde_json::from_value(json!({
            "batch_id": batch_id(self.id), "identity_scheme": "batch_v1",
            "company_guid": self.company,
            "txn_ids": txn_ids, "date_from": "20260901", "date_to": "20260901",
            "sha256": sha256_hex(batch_id(self.id).as_bytes()),
            // seconds follow the id, so a larger id is built later
            "built_at": format!("2026-09-01T10:00:{:02}Z", self.id % 60),
            "status": self.status,
            "pre_import_mark": {"kind": "company_high_water", "value": 1, "master_value": 1},
            "vouchers": self.vouchers,
        }))
        .unwrap();
        line.amends_batch_id = self.amends.map(batch_id);
        line.endpoint_origin = self.endpoint.map(str::to_string);
        line.ledger_identities = self.identities.as_ref().map(|identities| {
            identities
                .iter()
                .map(|(name, guid)| BoundLedger {
                    name: (*name).into(),
                    guid: (*guid).into(),
                })
                .collect()
        });
        line.statement = self.statement.as_ref().map(statement_record);
        line
    }
}

impl StatementFacts {
    fn window(mut self, from: &str, to: &str) -> Self {
        self.coverage = Some(StatementWindow {
            from: date(from),
            to: date(to),
        });
        self
    }
    fn contra(mut self, txn: &str, reference: Option<&str>) -> Self {
        self.contra_rows.push(ContraRow {
            bridge_txn_id: txn.into(),
            reference: reference.map(str::to_string),
            instrument: None,
            reversed_same_day: false,
        });
        self
    }
    fn skip(mut self, row: SkipRow) -> Self {
        self.skip_rows.push(row);
        self
    }
}

fn facts(bank: &str) -> StatementFacts {
    StatementFacts {
        bank_ledger_guid: bank.into(),
        coverage: None,
        contra_rows: Vec::new(),
        skip_rows: Vec::new(),
    }
}

/// An outflow of `amount` on 1 Sept, to the ledger `other`.
fn skip_row(txn: &str, amount: &str, other: &str, reference: Option<&str>) -> SkipRow {
    SkipRow {
        bridge_txn_id: txn.into(),
        date: date("20260901"),
        amount: amount.into(),
        direction: RowDirection::Outflow,
        other_ledger_guid: other.into(),
        reference: reference.map(str::to_string),
        instrument: None,
        reversed_same_day: false,
    }
}

#[derive(Default)]
struct Journal(String);

impl Journal {
    fn push(mut self, record: &impl Serialize) -> Self {
        self.0.push_str(&serde_json::to_string(record).unwrap());
        self.0.push('\n');
        self
    }
    fn batch(self, build: &Build) -> Self {
        self.push(&build.line())
    }
    /// A verification record that sets the batch's status.
    fn status(self, build: &Build, status: &str) -> Self {
        let mut line = build.line();
        line.status = status.into();
        self.push(&ledger::StatusRecord::from(&line))
    }
    /// A dispatch intent: the batch was sent.
    fn sent(self, build: &Build) -> Self {
        self.push(&ledger::StatusRecord::dispatch(&build.line()))
    }
    fn read(&self) -> Result<TransferFacts, TransferFactsError> {
        self.read_with("", COMPANY)
    }
    fn read_with(&self, skips: &str, company: &str) -> Result<TransferFacts, TransferFactsError> {
        self.read_candidate(skips, company, None)
    }
    fn read_candidate(
        &self,
        skips: &str,
        company: &str,
        candidate: Option<Candidate>,
    ) -> Result<TransferFacts, TransferFactsError> {
        ledger::transfer_facts(
            Cursor::new(self.0.as_bytes()),
            Cursor::new(skips.as_bytes()),
            company,
            candidate,
        )
    }
}

fn skip_record(
    n: u128,
    endpoint: &str,
    built_at: &str,
    statement: StatementFacts,
) -> SkipOnlyRecord {
    SkipOnlyRecord {
        record_id: format!("skips-{}", Uuid::from_u128(0x4000_8000_0000_0000_0000 + n)),
        company_guid: COMPANY.into(),
        endpoint_origin: endpoint.into(),
        built_at: built_at.into(),
        statement,
    }
}

fn skip_file(records: &[SkipOnlyRecord]) -> String {
    records
        .iter()
        .map(|record| format!("{}\n", serde_json::to_string(record).unwrap()))
        .collect()
}

fn key(bank: &str, row: u32) -> SkipKey {
    SkipKey {
        company: CompanyGuid::new(COMPANY),
        bank_ledger: LedgerGuid::new(bank),
        endpoint_origin: EndpointOrigin::new(ENDPOINT),
        row: StatementRowId::new(st(row)),
    }
}

fn contra_key(batch: u32, row: u32) -> ContraKey {
    ContraKey {
        batch: BatchId::new(batch_id(batch)),
        txn: TxnId::new(st(row)),
    }
}

// ---- the whole chain --------------------------------------------------------

/// A skipped row on Bank One and the Contra Bank Two's statement booked for it.
fn transfer_journal() -> Journal {
    let outflow = build(1)
        .facts(facts(B1).window("20260901", "20260930").skip(skip_row(
            &st(1),
            "500.00",
            B2,
            Some(REF_A),
        )))
        .status("posted_verified");
    let booked = build(2)
        .voucher(contra(&st(2), N2, N1, "500.00"))
        .facts(
            facts(B2)
                .window("20260901", "20260930")
                .contra(&st(2), Some(REF_A)),
        )
        .status("posted_verified");
    Journal::default().batch(&outflow).batch(&booked)
}

#[test]
fn facts_from_a_journal_give_a_pair_by_reference_in_the_view() {
    let facts = transfer_journal().read().unwrap();
    let view = transfer_pairing_view(&facts.skips, &facts.contras, &facts.coverage, 0).unwrap();
    assert_eq!(
        view.skips.get(&key(B1, 1)),
        Some(&SkipState::Paired {
            round: PairRound::Reference,
            contra: contra_key(2, 2),
        })
    );
    assert_eq!(facts.contras.len(), 1);
    let fact = &facts.contras[0];
    assert_eq!(fact.status, BatchStatus::PostedVerified);
    assert_eq!(fact.statement_bank_ledger, LedgerGuid::new(B2));
    assert_eq!(
        fact.reference.as_ref().map(ToString::to_string).as_deref(),
        Some(REF_A)
    );
    assert_eq!(fact.endpoint_origin, EndpointOrigin::new(ENDPOINT));
    assert_eq!(fact.company, CompanyGuid::new(COMPANY));
}

#[test]
fn a_skip_carries_its_row_into_the_view() {
    let mut inflow = skip_row(&st(3), "42.5", B2, None);
    inflow.direction = RowDirection::Inflow;
    inflow.date = date("20260903");
    let journal = Journal::default().batch(&build(3).facts(facts(B1).skip(inflow)));
    let found = journal.read().unwrap();
    let [skip] = &found.skips[..] else {
        panic!("one skip");
    };
    assert_eq!(skip.key, key(B1, 3));
    assert_eq!(skip.direction, Direction::Inflow);
    assert_eq!(skip.date, date("20260903"));
    assert!(skip.amount.numeric_eq(&decimal("42.5")));
    assert_eq!(skip.other_ledger, LedgerGuid::new(B2));
    assert_eq!(skip.reference, None);
    assert_eq!(skip.lineage, LineageId::new(batch_id(3)));
    assert!(!skip.reversed_same_day);
}

/// A refused build names the check that failed, one name for each.
#[test]
fn each_failed_check_has_a_name_of_its_own() {
    let txn = || st(1);
    let checks = [
        FactsError::EmptyLedgerGuid,
        FactsError::InvertedCoverage,
        FactsError::DuplicateTxnId { txn: txn() },
        FactsError::SkipIdNotStatementId { txn: txn() },
        FactsError::ReferenceInadmissible { txn: txn() },
        FactsError::AmountInadmissible { txn: txn() },
        FactsError::SkipLedgersEqual { txn: txn() },
        FactsError::ContraRowNotOneContraVoucher { txn: txn() },
        FactsError::ContraVoucherNotInFacts { txn: txn() },
        FactsError::SkipIsVoucher { txn: txn() },
        FactsError::BankLedgerUnbound,
        FactsError::BadRecordId,
        FactsError::NotSkipOnly,
    ];
    let names: BTreeSet<_> = checks.iter().map(FactsError::variant).collect();
    assert_eq!(names.len(), checks.len());
    assert!(names
        .iter()
        .all(|name| name.chars().all(|c| c.is_ascii_lowercase() || c == '_')));
}

/// The flag a statement set on a row follows it into the facts the view takes,
/// whether the row is a Contra, a skip of a batch or a skip-only record's, and
/// is written only when set.
#[test]
fn the_same_day_reversal_flag_follows_each_row_into_the_facts() {
    let mut flagged_skip = skip_row(&st(1), "500.00", B2, None);
    flagged_skip.reversed_same_day = true;
    let plain_skip = skip_row(&st(4), "500.00", B2, None);
    let mut flagged_contra = facts(B2).contra(&st(2), None);
    flagged_contra.contra_rows[0].reversed_same_day = true;
    let journal = Journal::default()
        .batch(&build(1).facts(facts(B1).skip(flagged_skip).skip(plain_skip)))
        .batch(
            &build(2)
                .voucher(contra(&st(2), N2, N1, "500.00"))
                .voucher(contra(&st(3), N2, N1, "500.00"))
                .facts(flagged_contra.contra(&st(3), None)),
        );
    let found = journal.read().unwrap();
    let skips: Vec<_> = found
        .skips
        .iter()
        .map(|skip| (skip.key.row.clone(), skip.reversed_same_day))
        .collect();
    assert_eq!(
        skips,
        [
            (StatementRowId::new(st(1)), true),
            (StatementRowId::new(st(4)), false)
        ]
    );
    let contras: Vec<_> = found
        .contras
        .iter()
        .map(|fact| (fact.key.txn.clone(), fact.reversed_same_day))
        .collect();
    assert_eq!(
        contras,
        [(TxnId::new(st(2)), true), (TxnId::new(st(3)), false)]
    );

    // a skip-only record's row carries it too
    let mut only = skip_row(&st(5), "500.00", B2, None);
    only.reversed_same_day = true;
    let record = skip_record(1, ENDPOINT, "2026-09-02T10:00:00Z", facts(B1).skip(only));
    let found = Journal::default()
        .read_with(&skip_file(std::slice::from_ref(&record)), COMPANY)
        .unwrap();
    assert!(found.skips[0].reversed_same_day);

    // written only when set
    let text = serde_json::to_string(&record).unwrap();
    assert!(text.contains("\"reversed_same_day\":true"), "{text}");
    let plain = serde_json::to_string(&skip_row(&st(6), "1.00", B2, None)).unwrap();
    assert!(!plain.contains("reversed_same_day"), "{plain}");
}

#[test]
fn legs_read_through_the_bound_names_and_follow_debit_and_credit() {
    let found = transfer_journal().read().unwrap();
    let legs = &found.contras[0].legs;
    assert_eq!(
        (legs[0].ledger.clone(), legs[0].side),
        (LedgerGuid::new(B2), Side::Debit)
    );
    assert_eq!(
        (legs[1].ledger.clone(), legs[1].side),
        (LedgerGuid::new(B1), Side::Credit)
    );
    assert!(found.contras[0].amount.numeric_eq(&decimal("500")));
    assert_eq!(found.contras[0].date, date("20260901"));
}

// ---- lineage -----------------------------------------------------------------

#[test]
fn an_amendment_naming_one_row_keeps_the_other_from_the_original() {
    let original = build(10)
        .voucher(contra(&st(11), N2, N1, "100.00"))
        .voucher(contra(&st(12), N2, N1, "200.00"))
        .facts(
            facts(B2)
                .contra(&st(11), Some(REF_A))
                .contra(&st(12), Some(REF_B)),
        )
        .status("posted_verified");
    let amendment = build(11)
        .amends(10)
        .voucher(contra(&st(11), N2, N1, "150.00"))
        .facts(facts(B2).contra(&st(11), Some("600000000003")))
        .status("posted_verified");
    let found = Journal::default()
        .batch(&original)
        .batch(&amendment)
        .read()
        .unwrap();
    assert_eq!(found.contras.len(), 2);
    let a = found
        .contras
        .iter()
        .find(|c| c.key == contra_key(11, 11))
        .unwrap();
    let b = found
        .contras
        .iter()
        .find(|c| c.key == contra_key(10, 12))
        .unwrap();
    assert!(a.amount.numeric_eq(&decimal("150")));
    assert_eq!(
        a.reference.as_ref().map(ToString::to_string).as_deref(),
        Some("600000000003")
    );
    assert!(b.amount.numeric_eq(&decimal("200")));
    assert_eq!(
        b.reference.as_ref().map(ToString::to_string).as_deref(),
        Some(REF_B)
    );
    // one lineage, whichever build decided the row
    assert_eq!(a.lineage, b.lineage);
    assert_eq!(a.lineage, LineageId::new(batch_id(10)));
}

#[test]
fn an_amendment_without_facts_changes_the_amount_and_the_fact_follows_it() {
    let original = build(20)
        .voucher(contra(&st(21), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(21), Some(REF_A)))
        .status("posted_verified");
    let amendment = build(21)
        .amends(20)
        .voucher(contra(&st(21), N2, N1, "175.00"))
        .status("posted_verified");
    let found = Journal::default()
        .batch(&original)
        .batch(&amendment)
        .read()
        .unwrap();
    let [fact] = &found.contras[..] else {
        panic!("one contra");
    };
    assert_eq!(fact.key, contra_key(21, 21));
    assert!(fact.amount.numeric_eq(&decimal("175")));
    // the reference is the newest facts entry for the row
    assert_eq!(
        fact.reference.as_ref().map(ToString::to_string).as_deref(),
        Some(REF_A)
    );
    assert_eq!(fact.statement_bank_ledger, LedgerGuid::new(B2));
}

#[test]
fn a_contra_with_no_facts_entry_in_its_lineage_is_not_a_statement_contra() {
    let hand_keyed = build(22).voucher(contra(&st(22), N2, N1, "100.00"));
    let with_facts = build(23)
        .voucher(contra(&st(23), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(23), None));
    let found = Journal::default()
        .batch(&hand_keyed)
        .batch(&with_facts)
        .read()
        .unwrap();
    assert_eq!(found.contras.len(), 1);
    assert_eq!(found.contras[0].key, contra_key(23, 23));
    // not counted as left out either: it was never a statement Contra
    assert_eq!(found.contras_not_two_leg, 0);
}

#[test]
fn a_contra_later_named_a_skip_in_its_own_lineage_is_an_error() {
    let original = build(30)
        .voucher(contra(&st(31), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(31), None));
    let amendment =
        build(31)
            .amends(30)
            .facts(facts(B2).skip(skip_row(&st(31), "100.00", B1, None)));
    let error = Journal::default()
        .batch(&original)
        .batch(&amendment)
        .read()
        .unwrap_err();
    assert_eq!(
        error,
        TransferFactsError::SkipIsVoucherInLineage {
            lineage: batch_id(30),
            txn: st(31),
        }
    );
}

#[test]
fn a_row_dropped_by_an_amendment_is_still_a_voucher_of_its_lineage() {
    // the amendment no longer carries the row, but an earlier build did
    let original = build(32).voucher(payment(&st(33), "10.00"));
    let amendment =
        build(33)
            .amends(32)
            .facts(facts(B1).skip(skip_row(&st(33), "10.00", B2, None)));
    let error = Journal::default()
        .batch(&original)
        .batch(&amendment)
        .read()
        .unwrap_err();
    assert!(matches!(
        error,
        TransferFactsError::SkipIsVoucherInLineage { .. }
    ));
}

#[test]
fn each_contra_takes_the_status_of_the_build_that_decided_it() {
    let first = build(40)
        .voucher(contra(&st(41), N2, N1, "100.00"))
        .voucher(contra(&st(42), N2, N1, "200.00"))
        .facts(facts(B2).contra(&st(41), None).contra(&st(42), None))
        .status("verification_incomplete");
    let amendment = build(41)
        .amends(40)
        .voucher(contra(&st(41), N2, N1, "110.00"))
        .facts(facts(B2).contra(&st(41), None))
        .status("posted_verified");
    let found = Journal::default()
        .batch(&first)
        .batch(&amendment)
        .read()
        .unwrap();
    let by_row = |row: u32| {
        found
            .contras
            .iter()
            .find(|c| c.key.txn == TxnId::new(st(row)))
            .unwrap()
    };
    assert_eq!(by_row(41).key, contra_key(41, 41));
    assert_eq!(by_row(41).status, BatchStatus::PostedVerified);
    assert_eq!(by_row(42).key, contra_key(40, 42));
    assert_eq!(
        by_row(42).status,
        BatchStatus::NotVerified {
            batch: BatchId::new(batch_id(40))
        }
    );
}

#[test]
fn the_latest_status_of_the_deciding_build_is_the_one_read() {
    let verified_then_reset = build(43)
        .voucher(contra(&st(43), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(43), None));
    let journal = Journal::default()
        .batch(&verified_then_reset)
        .status(&verified_then_reset, "posted_verified")
        .sent(&verified_then_reset);
    let found = journal.read().unwrap();
    assert_eq!(
        found.contras[0].status,
        BatchStatus::NotVerified {
            batch: BatchId::new(batch_id(43))
        }
    );
    let settled = journal
        .status(&verified_then_reset, "posted_verified")
        .read()
        .unwrap();
    assert_eq!(settled.contras[0].status, BatchStatus::PostedVerified);
}

#[test]
fn a_re_appended_record_keeps_the_age_of_its_first() {
    let original = build(44)
        .voucher(contra(&st(45), N2, N1, "100.00"))
        .voucher(contra(&st(46), N2, N1, "200.00"))
        .facts(facts(B2).contra(&st(45), None).contra(&st(46), None));
    let amendment = build(45)
        .amends(44)
        .voucher(contra(&st(45), N2, N1, "150.00"))
        .facts(facts(B2).contra(&st(45), None));
    // the original's full record is appended again after the amendment
    let found = Journal::default()
        .batch(&original)
        .batch(&amendment)
        .batch(&original)
        .read()
        .unwrap();
    let row = found
        .contras
        .iter()
        .find(|c| c.key.txn == TxnId::new(st(45)))
        .unwrap();
    assert_eq!(row.key, contra_key(45, 45));
    assert!(row.amount.numeric_eq(&decimal("150")));
}

#[test]
fn a_later_full_record_that_differs_from_the_first_is_an_error() {
    let original = build(46)
        .voucher(contra(&st(47), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(47), None));
    let other_voucher = build(46)
        .voucher(contra(&st(47), N2, N1, "101.00"))
        .facts(facts(B2).contra(&st(47), None));
    let other_facts = build(46)
        .voucher(contra(&st(47), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(47), Some(REF_A)));
    for changed in [other_voucher, other_facts] {
        let error = Journal::default()
            .batch(&original)
            .batch(&changed)
            .read()
            .unwrap_err();
        assert_eq!(
            error,
            TransferFactsError::BatchRecordChanged {
                batch_id: batch_id(46)
            }
        );
    }
}

// ---- rebuilds and retirement -------------------------------------------------

#[test]
fn two_lineages_skipping_one_row_give_the_newest_by_built_at() {
    let older = build(50).facts(facts(B1).skip(skip_row(&st(5), "500.00", B2, Some(REF_A))));
    let newer = build(51).facts(facts(B1).skip(skip_row(&st(5), "500.00", B2, None)));
    // the newer build comes FIRST in the journal: built_at decides, not position
    let found = Journal::default()
        .batch(&newer)
        .batch(&older)
        .read()
        .unwrap();
    let [skip] = &found.skips[..] else {
        panic!("one skip");
    };
    assert_eq!(skip.key, key(B1, 5));
    assert_eq!(skip.reference, None);
    assert_eq!(skip.lineage, LineageId::new(batch_id(51)));
}

#[test]
fn a_skip_found_posted_in_another_batch_is_retired() {
    let skipped = build(60).facts(facts(B1).skip(skip_row(&st(6), "500.00", B2, None)));
    let posted = build(61)
        .voucher(payment(&st(6), "500.00"))
        .status("posted_verified");
    let found = Journal::default()
        .batch(&skipped)
        .batch(&posted)
        .read()
        .unwrap();
    assert!(found.skips.is_empty());
    assert_eq!(found.retired_skips, 1);
    assert!(found.held_elsewhere.is_empty());
}

#[test]
fn a_skip_found_posted_at_some_time_is_retired_whatever_the_status_now() {
    let skipped = build(62).facts(facts(B1).skip(skip_row(&st(6), "500.00", B2, None)));
    let posted = build(63).voucher(payment(&st(6), "500.00"));
    let journal = Journal::default()
        .batch(&skipped)
        .batch(&posted)
        .status(&posted, "posted_verified")
        .status(&posted, "verification_incomplete");
    assert_eq!(journal.read().unwrap().retired_skips, 1);
}

#[test]
fn a_skip_held_by_a_batch_only_built_or_only_sent_is_held_elsewhere() {
    let skipped = build(64).facts(facts(B1).skip(skip_row(&st(6), "500.00", B2, None)));
    let held = build(65).voucher(payment(&st(6), "500.00"));
    for journal in [
        Journal::default().batch(&skipped).batch(&held),
        Journal::default().batch(&skipped).batch(&held).sent(&held),
    ] {
        let found = journal.read().unwrap();
        assert!(found.skips.is_empty());
        assert_eq!(found.retired_skips, 0);
        assert_eq!(
            found.held_elsewhere,
            vec![(key(B1, 6), BatchId::new(batch_id(65)), date("20260901"))]
        );
    }
}

#[test]
fn a_batch_of_another_company_holds_nothing() {
    let skipped = build(66).facts(facts(B1).skip(skip_row(&st(6), "500.00", B2, None)));
    let elsewhere = build(67)
        .company(OTHER_COMPANY)
        .voucher(payment(&st(6), "500.00"))
        .status("posted_verified");
    let found = Journal::default()
        .batch(&skipped)
        .batch(&elsewhere)
        .read()
        .unwrap();
    assert_eq!(found.skips.len(), 1);
    assert_eq!(found.retired_skips, 0);
}

#[test]
fn an_unsent_unfound_contra_is_dropped_once_another_lineage_holds_its_row() {
    let rebuilt = build(70)
        .voucher(contra(&st(7), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(7), None));
    let holder = build(71).voucher(contra(&st(7), N2, N1, "100.00"));
    // built only: both stand
    let both = Journal::default()
        .batch(&rebuilt)
        .batch(&holder)
        .read()
        .unwrap();
    assert_eq!((both.contras.len(), both.contras_superseded), (1, 0));
    // the other lineage was sent: the never-sent rebuild no longer counts
    let sent = Journal::default()
        .batch(&rebuilt)
        .batch(&holder)
        .sent(&holder)
        .read()
        .unwrap();
    assert_eq!((sent.contras.len(), sent.contras_superseded), (0, 1));
    // or found posted
    let posted = Journal::default()
        .batch(&rebuilt)
        .batch(&holder)
        .status(&holder, "posted_verified")
        .read()
        .unwrap();
    assert_eq!((posted.contras.len(), posted.contras_superseded), (0, 1));
    // a rebuild that was itself sent stays: it may be in the book
    let itself_sent = Journal::default()
        .batch(&rebuilt)
        .sent(&rebuilt)
        .batch(&holder)
        .sent(&holder)
        .read()
        .unwrap();
    assert_eq!(
        (itself_sent.contras.len(), itself_sent.contras_superseded),
        (1, 0)
    );
    // and a verified one stays
    let verified = Journal::default()
        .batch(&rebuilt)
        .status(&rebuilt, "posted_verified")
        .batch(&holder)
        .sent(&holder)
        .read()
        .unwrap();
    assert_eq!(
        (verified.contras.len(), verified.contras_superseded),
        (1, 0)
    );
    // a sent batch of the same lineage holding the row does not supersede it
    let amendment = build(72)
        .amends(70)
        .voucher(contra(&st(7), N2, N1, "120.00"));
    let own = Journal::default()
        .batch(&rebuilt)
        .sent(&rebuilt)
        .batch(&amendment)
        .read()
        .unwrap();
    assert_eq!((own.contras.len(), own.contras_superseded), (1, 0));
    assert_eq!(own.contras[0].key, contra_key(72, 7));
}

// ---- legs and left-out Contras -----------------------------------------------

#[test]
fn a_ledger_name_the_build_did_not_bind_is_an_error() {
    let unbound = build(80)
        .voucher(contra(&st(81), N2, "Unlisted", "100.00"))
        .facts(facts(B2).contra(&st(81), None));
    let error = Journal::default().batch(&unbound).read().unwrap_err();
    assert_eq!(
        error,
        TransferFactsError::LegLedgerUnbound {
            batch_id: batch_id(80),
            ledger: "Unlisted".into()
        }
    );
}

#[test]
fn a_build_without_a_binding_cannot_hold_facts() {
    let none = build(82)
        .identities(None)
        .voucher(contra(&st(83), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(83), None));
    assert_eq!(
        Journal::default().batch(&none).read().unwrap_err(),
        TransferFactsError::Facts {
            source_id: batch_id(82),
            error: FactsError::BankLedgerUnbound
        }
    );
}

#[test]
fn a_contra_that_is_not_two_ledgers_either_side_is_left_out_and_counted() {
    let three_legs = voucher(
        &st(90),
        "Contra",
        &[
            (N2, "100.00", "Dr"),
            (N1, "60.00", "Cr"),
            (N3, "40.00", "Cr"),
        ],
        "20260901",
    );
    let both_debit = voucher(
        &st(91),
        "Contra",
        &[(N2, "100.00", "Dr"), (N1, "100.00", "Dr")],
        "20260901",
    );
    let unequal = voucher(
        &st(92),
        "Contra",
        &[(N2, "100.00", "Dr"), (N1, "99.00", "Cr")],
        "20260901",
    );
    let one_ledger = voucher(
        &st(93),
        "Contra",
        &[(N2, "100.00", "Dr"), (N2, "100.00", "Cr")],
        "20260901",
    );
    let zero = voucher(
        &st(94),
        "Contra",
        &[(N2, "0.00", "Dr"), (N1, "0", "Cr")],
        "20260901",
    );
    let negative = voucher(
        &st(95),
        "Contra",
        &[(N2, "-5", "Dr"), (N1, "-5", "Cr")],
        "20260901",
    );
    let unreadable = voucher(
        &st(96),
        "Contra",
        &[(N2, "x", "Dr"), (N1, "x", "Cr")],
        "20260901",
    );
    // two bound ledgers that are one ledger
    let same_guid = voucher(
        &st(97),
        "Contra",
        &[(N2, "100.00", "Dr"), ("Bank Two (old)", "100.00", "Cr")],
        "20260901",
    );
    // neither leg is the statement's own bank
    let third_bank = voucher(
        &st(98),
        "Contra",
        &[(N1, "100.00", "Dr"), (N3, "100.00", "Cr")],
        "20260901",
    );
    let all = [
        three_legs, both_debit, unequal, one_ledger, zero, negative, unreadable, same_guid,
        third_bank,
    ];
    for (index, rejected) in all.iter().enumerate() {
        let txn = rejected["bridge_txn_id"].as_str().unwrap().to_string();
        let ok = build(100 + u32::try_from(index).unwrap())
            .voucher(contra(&st(99), N2, N1, "5.00"))
            .voucher(rejected.clone())
            .identities(Some(vec![
                (N1, B1),
                (N2, B2),
                (N3, B3),
                ("Bank Two (old)", B2),
            ]))
            .facts(facts(B2).contra(&st(99), None).contra(&txn, None))
            .status("posted_verified");
        let found = Journal::default().batch(&ok).read().unwrap();
        assert_eq!(found.contras.len(), 1, "{txn}");
        assert_eq!(found.contras[0].key.txn, TxnId::new(st(99)), "{txn}");
        assert_eq!(found.contras_not_two_leg, 1, "{txn}");
    }
}

#[test]
fn equal_amounts_may_differ_in_scale() {
    let scaled = build(110)
        .voucher(voucher(
            &st(111),
            "Contra",
            &[(N2, "100.0", "Dr"), (N1, "100.00", "Cr")],
            "20260901",
        ))
        .facts(facts(B2).contra(&st(111), None));
    let found = Journal::default().batch(&scaled).read().unwrap();
    assert_eq!((found.contras.len(), found.contras_not_two_leg), (1, 0));
}

// ---- coverage ----------------------------------------------------------------

/// A skip of 500 on Bank One to Bank Two with no reference, dated 1 Sept, and
/// what the view says of it given `rest` of the journal and no Contra to find.
fn state_given(rest: Journal, skips_file: &str) -> SkipState {
    let skipped = build(120).facts(facts(B1).window("20260901", "20260930").skip(skip_row(
        &st(1),
        "500.00",
        B2,
        None,
    )));
    let journal = Journal(format!(
        "{}{}",
        Journal::default().batch(&skipped).0,
        rest.0
    ));
    let found = journal.read_with(skips_file, COMPANY).unwrap();
    let view = transfer_pairing_view(&found.skips, &found.contras, &found.coverage, 0).unwrap();
    view.skips[&key(B1, 1)].clone()
}

fn bank_two_statement(id: u32, status: &'static str) -> Build {
    build(id)
        .voucher(contra(&st(200 + id), N3, N2, "7.00"))
        .facts(
            facts(B2)
                .window("20260901", "20260930")
                .contra(&st(200 + id), None),
        )
        .status(status)
}

#[test]
fn a_verified_statement_of_the_other_ledger_proves_a_skip_has_no_other_side() {
    let verified = bank_two_statement(121, "posted_verified");
    assert_eq!(
        state_given(Journal::default().batch(&verified), ""),
        SkipState::NotFound
    );
}

#[test]
fn nothing_proves_absence_without_a_statement_of_the_other_ledger() {
    assert_eq!(
        state_given(Journal::default(), ""),
        SkipState::NotEstablished
    );
}

#[test]
fn a_statement_not_verified_adds_no_coverage() {
    let unverified = bank_two_statement(122, "verification_incomplete");
    assert_eq!(
        state_given(Journal::default().batch(&unverified), ""),
        SkipState::NotEstablished
    );
}

#[test]
fn a_lineage_partly_unverified_adds_no_coverage() {
    let first = build(123)
        .voucher(contra(&st(223), N3, N2, "7.00"))
        .voucher(contra(&st(224), N3, N2, "8.00"))
        .facts(
            facts(B2)
                .window("20260901", "20260930")
                .contra(&st(223), None)
                .contra(&st(224), None),
        )
        .status("verification_incomplete");
    let amendment = build(124)
        .amends(123)
        .voucher(contra(&st(223), N3, N2, "7.00"))
        .facts(
            facts(B2)
                .window("20260901", "20260930")
                .contra(&st(223), None),
        )
        .status("posted_verified");
    // row 224 is still decided by the build that was not verified
    let journal = Journal::default().batch(&first).batch(&amendment);
    assert_eq!(state_given(journal, ""), SkipState::NotEstablished);
    // verifying the first build as well completes the lineage
    let journal = Journal::default()
        .batch(&first)
        .status(&first, "posted_verified")
        .batch(&amendment);
    assert_eq!(state_given(journal, ""), SkipState::NotFound);
}

#[test]
fn a_lineage_that_left_a_contra_out_adds_no_coverage() {
    let three_legs = voucher(
        &st(230),
        "Contra",
        &[
            (N3, "100.00", "Dr"),
            (N2, "60.00", "Cr"),
            (N1, "40.00", "Cr"),
        ],
        "20260901",
    );
    let leftover = build(125)
        .voucher(three_legs)
        .facts(
            facts(B2)
                .window("20260901", "20260930")
                .contra(&st(230), None),
        )
        .status("posted_verified");
    assert_eq!(
        state_given(Journal::default().batch(&leftover), ""),
        SkipState::NotEstablished
    );
}

#[test]
fn a_window_outside_the_skips_date_is_no_cover() {
    let narrow = build(126)
        .facts(facts(B2).window("20260915", "20260930"))
        .status("posted_verified");
    assert_eq!(
        state_given(Journal::default().batch(&narrow), ""),
        SkipState::NotEstablished
    );
}

/// Bank Two's verified statement, which on its own proves the skip has no other
/// side, beside `inline`, a batch of a lineage that holds no facts.
fn beside_bank_two(inline: Build) -> SkipState {
    let verified = bank_two_statement(140, "posted_verified");
    state_given(Journal::default().batch(&verified).batch(&inline), "")
}

#[test]
fn a_transfer_booked_by_a_batch_without_facts_blocks_the_day_it_names() {
    // a withdrawn skip posted inline as a Contra, or as a Receipt naming both
    // banks: the facts cannot see it, sent, found or only built
    for kind in ["Contra", "Receipt"] {
        for status in ["built", "posted_verified"] {
            let inline = build(141)
                .voucher(voucher(
                    &st(9),
                    kind,
                    &[(N2, "500.00", "Dr"), (N1, "500.00", "Cr")],
                    "20260901",
                ))
                .status(status);
            assert_eq!(
                beside_bank_two(inline),
                SkipState::NotEstablished,
                "{kind} {status}"
            );
        }
    }
}

#[test]
fn a_statement_row_blocks_the_day_its_id_carries_as_well_as_its_date() {
    // dated after the window, but the row the id names was printed on the 1st
    let inline = build(142).voucher(voucher(
        &st(9),
        "Contra",
        &[(N2, "500.00", "Dr"), (N1, "500.00", "Cr")],
        "20261001",
    ));
    assert_eq!(beside_bank_two(inline), SkipState::NotEstablished);
}

#[test]
fn a_ledger_or_endpoint_a_batch_did_not_bind_blocks_every_ledger() {
    // a Contra that names neither bank of the skip by a bound GUID
    let unbound = build(143)
        .voucher(contra("txn-1", N3, N1, "500.00"))
        .identities(None);
    assert_eq!(beside_bank_two(unbound), SkipState::NotEstablished);
    // Bank Two, by a batch that names no endpoint
    let no_endpoint = build(144)
        .voucher(contra("txn-1", N2, N1, "500.00"))
        .endpoint(None);
    assert_eq!(beside_bank_two(no_endpoint), SkipState::NotEstablished);
    let neither = build(148)
        .voucher(contra("txn-1", N3, N1, "500.00"))
        .identities(None)
        .endpoint(None);
    assert_eq!(beside_bank_two(neither), SkipState::NotEstablished);
}

#[test]
fn a_batch_without_facts_blocks_only_the_ledgers_and_endpoint_it_names() {
    let other_ledgers = build(145).voucher(contra(&st(9), N3, N1, "500.00"));
    assert_eq!(beside_bank_two(other_ledgers), SkipState::NotFound);
    let other_endpoint = build(146)
        .voucher(contra(&st(9), N2, N1, "500.00"))
        .endpoint(Some(OTHER_ENDPOINT));
    assert_eq!(beside_bank_two(other_endpoint), SkipState::NotFound);
    // a blocked day takes the whole window it falls in: this one is outside it
    let other_day = build(147).voucher(voucher(
        "txn-1",
        "Contra",
        &[(N2, "500.00", "Dr"), (N1, "500.00", "Cr")],
        "20261001",
    ));
    assert_eq!(beside_bank_two(other_day), SkipState::NotFound);
}

#[test]
fn a_statement_without_a_window_adds_no_coverage() {
    let windowless = build(127)
        .voucher(contra(&st(231), N3, N2, "7.00"))
        .facts(facts(B2).contra(&st(231), None))
        .status("posted_verified");
    assert_eq!(
        state_given(Journal::default().batch(&windowless), ""),
        SkipState::NotEstablished
    );
}

#[test]
fn coverage_is_per_endpoint_and_per_company() {
    let other_endpoint = bank_two_statement(128, "posted_verified").endpoint(Some(OTHER_ENDPOINT));
    assert_eq!(
        state_given(Journal::default().batch(&other_endpoint), ""),
        SkipState::NotEstablished
    );
    let other_company = bank_two_statement(129, "posted_verified").company(OTHER_COMPANY);
    assert_eq!(
        state_given(Journal::default().batch(&other_company), ""),
        SkipState::NotEstablished
    );
}

#[test]
fn a_skip_only_record_adds_its_window() {
    let record = skip_record(
        1,
        ENDPOINT,
        "2026-09-01T11:00:00Z",
        facts(B2)
            .window("20260901", "20260930")
            .skip(skip_row(&st(9), "1.00", B3, None)),
    );
    assert_eq!(
        state_given(Journal::default(), &skip_file(&[record])),
        SkipState::NotFound
    );
}

#[test]
fn a_statement_with_no_vouchers_covers_its_window_whatever_its_status() {
    // no voucher row is decided by any build, so there is nothing to verify
    for status in ["built", "posted_verified"] {
        let all_skipped = build(130)
            .facts(facts(B2).window("20260901", "20260930").skip(skip_row(
                &st(9),
                "1.00",
                B3,
                None,
            )))
            .status(status);
        assert_eq!(
            state_given(Journal::default().batch(&all_skipped), ""),
            SkipState::NotFound,
            "{status}"
        );
    }
}

// ---- skip-only records ---------------------------------------------------------

#[test]
fn skip_only_records_join_the_merge_by_built_at() {
    let batch = build(140).facts(facts(B1).skip(skip_row(&st(14), "500.00", B2, Some(REF_A))));
    let newer = skip_record(
        2,
        ENDPOINT,
        "2026-09-02T10:00:00Z",
        facts(B1).skip(skip_row(&st(14), "500.00", B2, Some(REF_B))),
    );
    let older = skip_record(
        3,
        ENDPOINT,
        "2026-08-01T10:00:00Z",
        facts(B1).skip(skip_row(&st(14), "500.00", B2, None)),
    );
    let found = Journal::default()
        .batch(&batch)
        .read_with(&skip_file(&[older, newer.clone()]), COMPANY)
        .unwrap();
    let [skip] = &found.skips[..] else {
        panic!("one skip");
    };
    assert_eq!(
        skip.reference.as_ref().map(ToString::to_string).as_deref(),
        Some(REF_B)
    );
    assert_eq!(skip.lineage, LineageId::new(newer.record_id));
}

#[test]
fn a_skip_only_record_is_retired_like_a_batch_skip() {
    let record = skip_record(
        4,
        ENDPOINT,
        "2026-09-02T10:00:00Z",
        facts(B1).skip(skip_row(&st(15), "500.00", B2, None)),
    );
    let posted = build(141)
        .voucher(payment(&st(15), "500.00"))
        .status("posted_verified");
    let found = Journal::default()
        .batch(&posted)
        .read_with(&skip_file(&[record]), COMPANY)
        .unwrap();
    assert_eq!((found.skips.len(), found.retired_skips), (0, 1));
}

#[test]
fn skip_only_records_of_another_company_are_not_read() {
    let mut record = skip_record(
        5,
        ENDPOINT,
        "2026-09-02T10:00:00Z",
        facts(B1).skip(skip_row(&st(15), "500.00", B2, None)),
    );
    record.company_guid = OTHER_COMPANY.into();
    // not even checked: this one would be refused for its record id
    record.record_id = "not-a-record-id".into();
    let found = Journal::default()
        .read_with(&skip_file(&[record]), COMPANY)
        .unwrap();
    assert!(found.skips.is_empty());
}

#[test]
fn a_skip_only_record_is_read_whatever_the_case_of_its_company_guid() {
    let mut record = skip_record(
        6,
        ENDPOINT,
        "2026-09-02T10:00:00Z",
        facts(B1).skip(skip_row(&st(15), "500.00", B2, None)),
    );
    record.company_guid = COMPANY.to_ascii_uppercase();
    let found = Journal::default()
        .read_with(&skip_file(&[record]), &COMPANY.to_ascii_uppercase())
        .unwrap();
    assert_eq!(found.skips.len(), 1);
    assert_eq!(found.skips[0].key.company, CompanyGuid::new(COMPANY));
}

#[test]
fn the_skip_file_round_trips_through_the_journal_framing() {
    let records = [
        skip_record(
            7,
            ENDPOINT,
            "2026-09-02T10:00:00Z",
            facts(B1).window("20260901", "20260930").skip(skip_row(
                &st(16),
                "500.00",
                B2,
                Some(REF_A),
            )),
        ),
        skip_record(
            8,
            OTHER_ENDPOINT,
            "2026-09-03T10:00:00Z",
            facts(B3).skip(skip_row(&st(17), "20", B1, None)),
        ),
    ];
    let text = skip_file(&records);
    assert_eq!(
        read_skip_records(Cursor::new(text.as_bytes())).unwrap(),
        records
    );
    assert!(read_skip_records(Cursor::new(b"".as_slice()))
        .unwrap()
        .is_empty());
}

#[test]
fn the_skip_file_refuses_what_the_journal_framing_refuses() {
    let record = skip_record(
        9,
        ENDPOINT,
        "2026-09-02T10:00:00Z",
        facts(B1).skip(skip_row(&st(16), "500.00", B2, None)),
    );
    let line = serde_json::to_string(&record).unwrap();
    let read = |text: &str| read_skip_records(Cursor::new(text.as_bytes()));
    // the record delimiter is required, or the next append would run on
    assert_eq!(read(&line), Err("import_ledger_invalid".to_string()));
    assert_eq!(
        read("{not json}\n"),
        Err("import_ledger_invalid".to_string())
    );
    assert_eq!(
        read_skip_records(Cursor::new(&b"\xff\n"[..])),
        Err("import_ledger_unavailable".to_string())
    );
    // an unknown field is not skipped over
    let mut value = serde_json::to_value(&record).unwrap();
    value["extra"] = json!(1);
    assert_eq!(
        read(&format!("{value}\n")),
        Err("import_ledger_invalid".to_string())
    );
    // a date that is not a date
    let mut value = serde_json::to_value(&record).unwrap();
    value["statement"]["skip_rows"][0]["date"] = json!("20261301");
    assert_eq!(
        read(&format!("{value}\n")),
        Err("import_ledger_invalid".to_string())
    );
    // a direction that is neither
    let mut value = serde_json::to_value(&record).unwrap();
    value["statement"]["skip_rows"][0]["direction"] = json!("sideways");
    assert_eq!(
        read(&format!("{value}\n")),
        Err("import_ledger_invalid".to_string())
    );
}

#[test]
fn an_unreadable_journal_or_skip_file_is_a_typed_error() {
    let journal = Journal("{broken\n".into());
    assert_eq!(
        journal.read().unwrap_err(),
        TransferFactsError::JournalUnreadable {
            code: "import_ledger_invalid".into()
        }
    );
    assert_eq!(
        Journal::default()
            .read_with("{broken\n", COMPANY)
            .unwrap_err(),
        TransferFactsError::SkipFileUnreadable {
            code: "import_ledger_invalid".into()
        }
    );
}

#[test]
fn skip_only_records_are_checked_whole() {
    let good = || {
        skip_record(
            10,
            ENDPOINT,
            "2026-09-02T10:00:00Z",
            facts(B1).skip(skip_row(&st(16), "500.00", B2, None)),
        )
    };
    let check = |record: SkipOnlyRecord| {
        Journal::default()
            .read_with(&skip_file(&[record]), COMPANY)
            .map(drop)
    };
    assert_eq!(check(good()), Ok(()));
    let mut bad_id = good();
    bad_id.record_id = "skips-not-a-uuid".into();
    let id = bad_id.record_id.clone();
    assert_eq!(
        check(bad_id),
        Err(TransferFactsError::Facts {
            source_id: id,
            error: FactsError::BadRecordId
        })
    );
    // a version 1 UUID, or an upper-case one, is not what the writer draws
    for text in [
        "skips-6ba7b810-9dad-11d1-80b4-00c04fd430c8",
        "skips-6BA7B810-9DAD-41D1-80B4-00C04FD430C8",
    ] {
        let mut record = good();
        record.record_id = text.into();
        assert!(
            matches!(
                check(record),
                Err(TransferFactsError::Facts {
                    error: FactsError::BadRecordId,
                    ..
                })
            ),
            "{text}"
        );
    }
    let mut with_contra = good();
    with_contra.statement.contra_rows.push(ContraRow {
        bridge_txn_id: st(1),
        reference: None,
        instrument: None,
        reversed_same_day: false,
    });
    assert!(matches!(
        check(with_contra),
        Err(TransferFactsError::Facts {
            error: FactsError::NotSkipOnly,
            ..
        })
    ));
    let mut no_skips = good();
    no_skips.statement.skip_rows.clear();
    assert!(matches!(
        check(no_skips),
        Err(TransferFactsError::Facts {
            error: FactsError::NotSkipOnly,
            ..
        })
    ));
    let mut bad_facts = good();
    bad_facts.statement.skip_rows[0].amount = "0".into();
    assert!(matches!(
        check(bad_facts),
        Err(TransferFactsError::Facts {
            error: FactsError::AmountInadmissible { .. },
            ..
        })
    ));
    let mut no_endpoint = good();
    no_endpoint.endpoint_origin = String::new();
    let id = no_endpoint.record_id.clone();
    assert_eq!(
        check(no_endpoint),
        Err(TransferFactsError::MissingEndpoint { source_id: id })
    );
    let mut bad_time = good();
    bad_time.built_at = "yesterday".into();
    let id = bad_time.record_id.clone();
    assert_eq!(
        check(bad_time),
        Err(TransferFactsError::BuiltAtUnparsable { source_id: id })
    );
    let twice = skip_file(&[good(), good()]);
    let id = good().record_id;
    assert_eq!(
        Journal::default().read_with(&twice, COMPANY).map(drop),
        Err(TransferFactsError::DuplicateSkipRecordId { record_id: id })
    );
}

// ---- company, endpoint and the whole-view failure -------------------------------

#[test]
fn only_the_asked_companys_records_are_read_or_checked() {
    // fails exactly one check: it holds facts and names no endpoint
    let broken = build(150)
        .company(OTHER_COMPANY)
        .endpoint(None)
        .facts(facts(B1).skip(skip_row(&st(5), "500.00", B2, None)));
    let found = transfer_journal().batch(&broken).read().unwrap();
    assert_eq!(found.skips.len(), 1);
    // and read for the company they name
    assert_eq!(
        Journal::default()
            .batch(&broken)
            .read_with("", OTHER_COMPANY)
            .unwrap_err(),
        TransferFactsError::MissingEndpoint {
            source_id: batch_id(150)
        }
    );
}

#[test]
fn the_company_guid_matches_whatever_its_case() {
    let upper = transfer_journal()
        .read_with("", &COMPANY.to_ascii_uppercase())
        .unwrap();
    assert_eq!(upper.skips.len(), 1);
    assert_eq!(upper.skips[0].key.company, CompanyGuid::new(COMPANY));
}

#[test]
fn a_skip_is_per_endpoint() {
    let here = build(151).facts(facts(B1).skip(skip_row(&st(5), "500.00", B2, None)));
    let there = build(152)
        .endpoint(Some(OTHER_ENDPOINT))
        .facts(facts(B1).skip(skip_row(&st(5), "500.00", B2, None)));
    let found = Journal::default()
        .batch(&here)
        .batch(&there)
        .read()
        .unwrap();
    assert_eq!(found.skips.len(), 2);
}

#[test]
fn facts_on_a_build_with_no_endpoint_are_an_error() {
    let blind =
        build(153)
            .endpoint(None)
            .facts(facts(B1).skip(skip_row(&st(5), "500.00", B2, None)));
    assert_eq!(
        Journal::default().batch(&blind).read().unwrap_err(),
        TransferFactsError::MissingEndpoint {
            source_id: batch_id(153)
        }
    );
    // an amendment deciding a Contra needs one as well
    let original = build(154)
        .voucher(contra(&st(54), N2, N1, "1.00"))
        .facts(facts(B2).contra(&st(54), None));
    let blind_amendment =
        build(155)
            .amends(154)
            .endpoint(None)
            .voucher(contra(&st(54), N2, N1, "2.00"));
    assert_eq!(
        Journal::default()
            .batch(&original)
            .batch(&blind_amendment)
            .read()
            .unwrap_err(),
        TransferFactsError::MissingEndpoint {
            source_id: batch_id(155)
        }
    );
}

#[test]
fn a_build_time_that_is_not_a_time_is_an_error_where_it_is_compared() {
    let mut line = build(156)
        .facts(facts(B1).skip(skip_row(&st(5), "500.00", B2, None)))
        .line();
    line.built_at = "2026-09-01 10:00".into();
    let journal = Journal::default().push(&line);
    assert_eq!(
        journal.read().unwrap_err(),
        TransferFactsError::BuiltAtUnparsable {
            source_id: batch_id(156)
        }
    );
    // an older record with no facts is never compared
    let mut plain = build(157).line();
    plain.built_at = "garbled".into();
    assert!(Journal::default().push(&plain).read().is_ok());
}

#[test]
fn a_facts_error_names_its_batch_and_fails_the_whole_view() {
    let bad = build(160).facts(facts(B1).skip(skip_row("not-a-statement-id", "500.00", B2, None)));
    assert_eq!(
        transfer_journal().batch(&bad).read().unwrap_err(),
        TransferFactsError::Facts {
            source_id: batch_id(160),
            error: FactsError::SkipIdNotStatementId {
                txn: "not-a-statement-id".into()
            },
        }
    );
}

#[test]
fn a_journal_with_a_dispatch_in_it_reads_for_facts_too() {
    let sent = build(161)
        .voucher(payment(&st(61), "5.00"))
        .facts(facts(B1));
    let found = Journal::default()
        .batch(&sent)
        .sent(&sent)
        .status(&sent, "posted_verified")
        .read()
        .unwrap();
    assert!(found.contras.is_empty() && found.skips.is_empty());
}

// ---- the semantic checks ---------------------------------------------------------

fn shape<'a>(vouchers: &'a [BuildVoucher], identities: Option<&'a [BoundLedger]>) -> LineShape<'a> {
    LineShape {
        vouchers,
        ledger_identities: identities,
    }
}

fn slim(value: Value) -> BuildVoucher {
    let line = build(1).voucher(value).line();
    BuildLine::of(&line).vouchers.remove(0)
}

fn bound(pairs: &[(&str, &str)]) -> Vec<BoundLedger> {
    pairs
        .iter()
        .map(|(name, guid)| BoundLedger {
            name: (*name).into(),
            guid: (*guid).into(),
        })
        .collect()
}

fn valid() -> StatementFacts {
    facts(B1)
        .window("20260901", "20260930")
        .contra(&st(1), Some(REF_A))
        .skip(skip_row(&st(2), "500.00", B2, Some("SBIN526000000001")))
}

#[test]
fn well_formed_facts_are_admitted_and_typed() {
    let checked = validate_statement_facts(&valid(), None).unwrap();
    assert_eq!(checked.bank_ledger, LedgerGuid::new(B1));
    assert_eq!(
        checked.window,
        Some(ParseWindow {
            from: date("20260901"),
            to: date("20260930")
        })
    );
    assert_eq!(checked.contras.len(), 1);
    assert_eq!(checked.skips.len(), 1);
    assert_eq!(checked.skips[0].other_ledger, LedgerGuid::new(B2));
    // one place, two decimal places, or none
    for amount in ["1", "1.5", "1.50", "0.01", "100000000"] {
        let mut facts = valid();
        facts.skip_rows[0].amount = amount.into();
        assert!(validate_statement_facts(&facts, None).is_ok(), "{amount}");
    }
}

#[test]
fn every_semantic_check_is_a_typed_error() {
    let check = |mutate: &dyn Fn(&mut StatementFacts)| {
        let mut facts = valid();
        mutate(&mut facts);
        validate_statement_facts(&facts, None).unwrap_err()
    };
    assert_eq!(
        check(&|f| f.bank_ledger_guid.clear()),
        FactsError::EmptyLedgerGuid
    );
    assert_eq!(
        check(&|f| f.skip_rows[0].other_ledger_guid.clear()),
        FactsError::EmptyLedgerGuid
    );
    assert_eq!(
        check(&|f| f.skip_rows[0].other_ledger_guid = B1.into()),
        FactsError::SkipLedgersEqual { txn: st(2) }
    );
    assert_eq!(
        check(&|f| f.coverage = Some(StatementWindow {
            from: date("20260902"),
            to: date("20260901")
        })),
        FactsError::InvertedCoverage
    );
    assert_eq!(
        check(&|f| f.skip_rows[0].bridge_txn_id = st(1)),
        FactsError::DuplicateTxnId { txn: st(1) }
    );
    assert_eq!(
        check(&|f| f.contra_rows.push(f.contra_rows[0].clone())),
        FactsError::DuplicateTxnId { txn: st(1) }
    );
    assert_eq!(
        check(&|f| f.skip_rows.push(f.skip_rows[0].clone())),
        FactsError::DuplicateTxnId { txn: st(2) }
    );
    for bad in [
        "",
        "st-2026",
        "ST-20260901-0000000000000001",
        "st-20260901-000000000000000G",
        "txn-001",
    ] {
        assert_eq!(
            check(&|f| f.skip_rows[0].bridge_txn_id = bad.into()),
            FactsError::SkipIdNotStatementId { txn: bad.into() },
            "{bad:?}"
        );
    }
    for bad in [
        "",
        " ",
        "12345",
        "abc",
        "000000000000",
        "600000000001 ",
        "sbin526000000001",
    ] {
        assert_eq!(
            check(&|f| f.contra_rows[0].reference = Some(bad.into())),
            FactsError::ReferenceInadmissible { txn: st(1) },
            "{bad:?}"
        );
        assert_eq!(
            check(&|f| f.skip_rows[0].reference = Some(bad.into())),
            FactsError::ReferenceInadmissible { txn: st(2) },
            "{bad:?}"
        );
    }
    for bad in [
        "", "0", "0.00", "-5", "+5", "abc", "1.234", "1.", ".5", "1e3", "5 ",
    ] {
        assert_eq!(
            check(&|f| f.skip_rows[0].amount = bad.into()),
            FactsError::AmountInadmissible { txn: st(2) },
            "{bad:?}"
        );
    }
}

#[test]
fn a_reference_that_is_absent_is_not_an_empty_one() {
    let mut facts = valid();
    facts.contra_rows[0].reference = None;
    facts.skip_rows[0].reference = None;
    let checked = validate_statement_facts(&facts, None).unwrap();
    assert_eq!(checked.contras[&st(1)], None);
    assert_eq!(checked.skips[0].reference, None);
}

#[test]
fn the_checks_against_the_batch_line() {
    let line_vouchers = vec![
        slim(contra(&st(1), N2, N1, "100.00")),
        slim(payment(&st(5), "9.00")),
    ];
    let identities = bound(&[(N1, B1), (N2, B2)]);
    let mut facts = facts(B1)
        .contra(&st(1), None)
        .skip(skip_row(&st(2), "5.00", B2, None));
    assert!(
        validate_statement_facts(&facts, Some(shape(&line_vouchers, Some(&identities)))).is_ok()
    );
    let against =
        |facts: &StatementFacts, vouchers: &[BuildVoucher], identities: Option<&[BoundLedger]>| {
            validate_statement_facts(facts, Some(shape(vouchers, identities))).unwrap_err()
        };
    // a facts row naming no voucher, or a voucher that is not a Contra
    assert_eq!(
        against(&facts, &[slim(payment(&st(5), "9.00"))], Some(&identities)),
        FactsError::ContraRowNotOneContraVoucher { txn: st(1) }
    );
    assert_eq!(
        against(&facts, &[slim(payment(&st(1), "9.00"))], Some(&identities)),
        FactsError::ContraRowNotOneContraVoucher { txn: st(1) }
    );
    // or two of them
    assert_eq!(
        against(
            &facts,
            &[
                slim(contra(&st(1), N2, N1, "1.00")),
                slim(contra(&st(1), N2, N1, "2.00"))
            ],
            Some(&identities)
        ),
        FactsError::ContraRowNotOneContraVoucher { txn: st(1) }
    );
    // a Contra of the line the facts leave out
    assert_eq!(
        against(
            &facts,
            &[
                slim(contra(&st(1), N2, N1, "1.00")),
                slim(contra(&st(3), N2, N1, "1.00"))
            ],
            Some(&identities)
        ),
        FactsError::ContraVoucherNotInFacts { txn: st(3) }
    );
    // a skip that the line also carries as a voucher
    assert_eq!(
        against(
            &facts,
            &[
                slim(contra(&st(1), N2, N1, "1.00")),
                slim(payment(&st(2), "5.00"))
            ],
            Some(&identities)
        ),
        FactsError::SkipIsVoucher { txn: st(2) }
    );
    // a bank ledger the line never bound
    assert_eq!(
        against(&facts, &line_vouchers, None),
        FactsError::BankLedgerUnbound
    );
    assert_eq!(
        against(&facts, &line_vouchers, Some(&bound(&[(N2, B2)]))),
        FactsError::BankLedgerUnbound
    );
    // only the checks that need no line run without one
    facts.contra_rows.clear();
    assert!(validate_statement_facts(&facts, None).is_ok());
}

// ---- the journal record ------------------------------------------------------------

#[test]
fn the_binding_is_written_as_ledger_identities_3_only() {
    let line = build(1).voucher(payment(&st(1), "1.00")).line();
    let written = serde_json::to_value(&line).unwrap();
    assert!(written.get("ledger_identities_3").is_some(), "{written}");
    assert!(written.get("ledger_identities_2").is_none(), "{written}");
    assert!(written.get("ledger_identities").is_none(), "{written}");
    assert!(written.get("statement").is_none(), "{written}");
    let reread: ImportLedgerLine = serde_json::from_value(written).unwrap();
    assert_eq!(reread.ledger_identities, line.ledger_identities);
}

#[test]
fn a_binding_written_under_an_earlier_name_is_still_read() {
    let line = build(1).voucher(payment(&st(1), "1.00")).line();
    for earlier in ["ledger_identities_2", "ledger_identities"] {
        let mut value = serde_json::to_value(&line).unwrap();
        let binding = value
            .as_object_mut()
            .unwrap()
            .remove("ledger_identities_3")
            .unwrap();
        value[earlier] = binding;
        let reread: ImportLedgerLine = serde_json::from_value(value).unwrap();
        assert_eq!(
            reread.ledger_identities, line.ledger_identities,
            "{earlier}"
        );
    }
}

#[test]
fn a_record_carrying_two_names_for_the_binding_does_not_parse() {
    let line = build(1).voucher(payment(&st(1), "1.00")).line();
    let value = serde_json::to_value(&line).unwrap();
    let binding = value["ledger_identities_3"].clone();
    for (first, second) in [
        ("ledger_identities_3", "ledger_identities_2"),
        ("ledger_identities_3", "ledger_identities"),
        ("ledger_identities_2", "ledger_identities"),
    ] {
        let mut both = value.clone();
        both.as_object_mut().unwrap().remove("ledger_identities_3");
        both[first] = binding.clone();
        both[second] = binding.clone();
        assert!(
            serde_json::from_value::<ImportLedgerLine>(both.clone()).is_err(),
            "{first} {second}"
        );
        // and the journal reader, which refuses a record it cannot parse,
        // refuses the whole history with it
        let text = format!("{both}\n");
        assert_eq!(
            ledger::read_snapshot(Cursor::new(text.as_bytes()), None).map(drop),
            Err("import_ledger_invalid".to_string()),
            "{first} {second}"
        );
        assert_eq!(
            ledger::settlement(Cursor::new(text.as_bytes())).map(drop),
            Err("import_ledger_invalid".to_string()),
            "{first} {second}"
        );
    }
}

#[test]
fn a_release_that_reads_the_earlier_name_finds_no_binding_in_a_new_record() {
    // what a release that reads only `ledger_identities_2` sees of this record
    let line = build(1)
        .voucher(payment(&st(1), "1.00"))
        .facts(facts(B1))
        .line();
    let written = serde_json::to_value(&line).unwrap();
    assert!(written.get("ledger_identities_2").is_none());
    assert!(written.get("ledger_identities").is_none());
    assert!(written["statement"].is_object());
}

#[test]
fn facts_survive_the_journal_and_change_nothing_the_other_readers_see() {
    let with = build(1).voucher(contra(&st(1), N2, N1, "100.00")).facts(
        facts(B2)
            .window("20260901", "20260930")
            .contra(&st(1), Some(REF_A))
            .skip(skip_row(&st(2), "5.00", B1, None)),
    );
    let mut stripped = with.line();
    stripped.statement = None;
    let render = |build: &Build, line: &ImportLedgerLine| {
        Journal::default()
            .push(line)
            .sent(build)
            .status(build, "posted_verified")
            .0
    };
    let with_text = render(&with, &with.line());
    let without_text = render(&with, &stripped);
    let read = |text: &str| {
        let snapshot = ledger::read_snapshot(Cursor::new(text.as_bytes()), Some(&batch_id(1)))
            .unwrap()
            .unwrap();
        (
            snapshot.batch.statement.clone(),
            snapshot.batch.status.clone(),
            snapshot.dispatched,
            ledger::settlement(Cursor::new(text.as_bytes())).unwrap(),
        )
    };
    let (facts_read, status, dispatched, settlement) = read(&with_text);
    let (no_facts, status_without, dispatched_without, settlement_without) = read(&without_text);
    assert_eq!(facts_read, with.line().statement);
    assert_eq!(no_facts, None);
    assert_eq!(
        (status, dispatched, settlement),
        (status_without, dispatched_without, settlement_without)
    );
    // the duplicate-row check sees the same batch whichever journal it reads
    let rows = vec![serde_json::from_value(contra(&st(1), N2, N1, "100.00")).unwrap()];
    let posted = |text: &str| {
        ledger::vouchers_already_posted(
            Cursor::new(text.as_bytes()),
            COMPANY,
            Some("another"),
            &rows,
        )
        .unwrap()
    };
    assert_eq!(posted(&with_text), Some(batch_id(1)));
    assert_eq!(posted(&with_text), posted(&without_text));
    // and the history reads it back whole
    let history = ledger::parse_snapshots(&with_text).unwrap();
    assert_eq!(history[0].batch.statement, with.line().statement);
}

// ---- further orderings and statuses ---------------------------------------------

#[test]
fn a_batch_found_posted_stays_found_when_its_record_is_appended_again() {
    let skipped = build(170).facts(facts(B1).skip(skip_row(&st(6), "500.00", B2, None)));
    let posted = build(171)
        .voucher(payment(&st(6), "500.00"))
        .status("posted_verified");
    let again = build(171).voucher(payment(&st(6), "500.00"));
    let found = Journal::default()
        .batch(&skipped)
        .batch(&posted)
        .batch(&again)
        .read()
        .unwrap();
    assert_eq!(found.retired_skips, 1);
    // and its status is the later record's, as `read_snapshot` takes it
    let contra_built = build(172)
        .voucher(contra(&st(72), N2, N1, "1.00"))
        .facts(facts(B2).contra(&st(72), None))
        .status("posted_verified");
    let contra_again = build(172)
        .voucher(contra(&st(72), N2, N1, "1.00"))
        .facts(facts(B2).contra(&st(72), None));
    let found = Journal::default()
        .batch(&contra_built)
        .batch(&contra_again)
        .read()
        .unwrap();
    assert_eq!(
        found.contras[0].status,
        BatchStatus::NotVerified {
            batch: BatchId::new(batch_id(172))
        }
    );
}

#[test]
fn the_newest_mention_of_a_skip_in_a_lineage_gives_its_facts() {
    let first = build(173).facts(facts(B1).skip(skip_row(&st(8), "500.00", B2, Some(REF_A))));
    let second =
        build(174)
            .amends(173)
            .facts(facts(B1).skip(skip_row(&st(8), "500.00", B2, Some(REF_B))));
    let found = Journal::default()
        .batch(&first)
        .batch(&second)
        .read()
        .unwrap();
    let [skip] = &found.skips[..] else {
        panic!("one skip");
    };
    assert_eq!(
        skip.reference.as_ref().map(ToString::to_string).as_deref(),
        Some(REF_B)
    );
    assert_eq!(skip.lineage, LineageId::new(batch_id(173)));
}

#[test]
fn skips_built_at_one_instant_are_ordered_by_their_record_id() {
    let at = "2026-09-02T10:00:00Z";
    let small = skip_record(
        11,
        ENDPOINT,
        at,
        facts(B1).skip(skip_row(&st(18), "500.00", B2, Some(REF_A))),
    );
    let large = skip_record(
        12,
        ENDPOINT,
        at,
        facts(B1).skip(skip_row(&st(18), "500.00", B2, Some(REF_B))),
    );
    for order in [
        [small.clone(), large.clone()],
        [large.clone(), small.clone()],
    ] {
        let found = Journal::default()
            .read_with(&skip_file(&order), COMPANY)
            .unwrap();
        assert_eq!(
            found.skips[0].lineage,
            LineageId::new(large.record_id.clone())
        );
    }
    // an instant is not its text: 15:30 at +05:30 is 10:00 UTC, a tie
    let same_instant = skip_record(
        0,
        ENDPOINT,
        "2026-09-02T15:30:00+05:30",
        facts(B1).skip(skip_row(&st(18), "500.00", B2, None)),
    );
    let found = Journal::default()
        .read_with(&skip_file(&[same_instant.clone(), small.clone()]), COMPANY)
        .unwrap();
    assert_eq!(
        found.skips[0].lineage,
        LineageId::new(small.record_id.clone())
    );
    // one minute earlier loses to the other whatever the text says
    let earlier = skip_record(
        99,
        ENDPOINT,
        "2026-09-02T15:29:00+05:30",
        facts(B1).skip(skip_row(&st(18), "500.00", B2, None)),
    );
    let found = Journal::default()
        .read_with(&skip_file(&[earlier, small.clone()]), COMPANY)
        .unwrap();
    assert_eq!(found.skips[0].lineage, LineageId::new(small.record_id));
}

#[test]
fn a_skip_held_by_two_batches_names_the_latest_in_the_journal() {
    let skipped = build(175).facts(facts(B1).skip(skip_row(&st(6), "500.00", B2, None)));
    let early = build(176).voucher(payment(&st(6), "500.00"));
    let late = build(177).voucher(payment(&st(6), "500.00"));
    // 177 is written first: the latest in the journal is 176, whatever the ids
    let found = Journal::default()
        .batch(&skipped)
        .batch(&late)
        .batch(&early)
        .read()
        .unwrap();
    assert_eq!(
        found.held_elsewhere,
        vec![(key(B1, 6), BatchId::new(batch_id(176)), date("20260901"))]
    );
}

#[test]
fn a_rebuild_once_found_posted_is_not_superseded_whatever_its_status_now() {
    let rebuilt = build(178)
        .voucher(contra(&st(7), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(7), None));
    let holder = build(179).voucher(contra(&st(7), N2, N1, "100.00"));
    let found = Journal::default()
        .batch(&rebuilt)
        .status(&rebuilt, "posted_verified")
        .status(&rebuilt, "verification_incomplete")
        .batch(&holder)
        .sent(&holder)
        .read()
        .unwrap();
    assert_eq!((found.contras.len(), found.contras_superseded), (1, 0));
}

#[test]
fn a_contra_later_given_another_type_is_left_out() {
    let original = build(180)
        .voucher(contra(&st(80), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(80), None));
    let retyped = build(181).amends(180).voucher(voucher(
        &st(80),
        "Payment",
        &[(N2, "100.00", "Dr"), (N1, "100.00", "Cr")],
        "20260901",
    ));
    let found = Journal::default()
        .batch(&original)
        .batch(&retyped)
        .read()
        .unwrap();
    assert!(found.contras.is_empty());
    assert_eq!(found.contras_not_two_leg, 1);
}

fn captured_outcome() -> bridge_tally_protocol::TallyImportOutcome {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-import-response.utf16le.xml"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    bridge_tally_protocol::parse_import_outcome(&xml).unwrap()
}

#[test]
fn a_post_span_verdict_sets_no_status_so_a_verified_contra_stays_verified() {
    let sent = build(182)
        .voucher(contra(&st(82), N2, N1, "100.00"))
        .facts(facts(B2).contra(&st(82), None));
    let line = sent.line();
    let mut intent = serde_json::to_value(ledger::StatusRecord::dispatch_native(
        &line,
        "c".repeat(64),
        Uuid::new_v4(),
    ))
    .unwrap();
    intent["pre_post_voucher_mark"] = json!(1795);
    let response = ledger::StatusRecord::response(
        &line,
        ledger::DispatchResponse {
            request_sha256: "c".repeat(64),
            response_sha256: "d".repeat(64),
            bytes: 1,
            outcome: Some(captured_outcome()),
        },
    );
    let verdict = ledger::StatusRecord::post_span_verdict(
        &line,
        &ledger::PostSpanVerdict::Refused("span_step_not_created".into()),
    );
    let journal = Journal::default()
        .batch(&sent)
        .push(&intent)
        .push(&response)
        .status(&sent, "posted_verified")
        .push(&verdict);
    let found = journal.read().unwrap();
    assert_eq!(found.contras[0].status, BatchStatus::PostedVerified);
}

#[test]
fn a_contra_keyed_by_something_other_than_a_statement_id_is_still_a_statement_contra() {
    // only skipped rows carry statement ids; a booked Contra's id is not bound to one
    let keyed = build(190)
        .voucher(contra("txn-contra-1", N2, N1, "100.00"))
        .facts(facts(B2).contra("txn-contra-1", None))
        .status("posted_verified");
    let found = Journal::default().batch(&keyed).read().unwrap();
    assert_eq!(found.contras.len(), 1);
    assert_eq!(found.contras[0].key.txn, TxnId::new("txn-contra-1"));
}

#[test]
fn a_batch_first_written_unverified_and_written_again_verified_is_found() {
    let skipped = build(191).facts(facts(B1).skip(skip_row(&st(6), "500.00", B2, None)));
    let first = build(192).voucher(payment(&st(6), "500.00"));
    let again = build(192)
        .voucher(payment(&st(6), "500.00"))
        .status("posted_verified");
    let found = Journal::default()
        .batch(&skipped)
        .batch(&first)
        .batch(&again)
        .read()
        .unwrap();
    assert_eq!(found.retired_skips, 1);
}

#[test]
fn facts_on_a_build_with_no_endpoint_are_an_error_even_when_nothing_uses_it() {
    // its only row is decided by a later build that has an endpoint, and it
    // carries no skipped row or window that would need one
    let blind = build(193)
        .endpoint(None)
        .voucher(contra(&st(93), N2, N1, "1.00"))
        .facts(facts(B2).contra(&st(93), None));
    let amendment = build(194)
        .amends(193)
        .voucher(contra(&st(93), N2, N1, "2.00"));
    assert_eq!(
        Journal::default()
            .batch(&blind)
            .batch(&amendment)
            .read()
            .unwrap_err(),
        TransferFactsError::MissingEndpoint {
            source_id: batch_id(193)
        }
    );
}

// ---- coverage blocked by rows the facts cannot see --------------------------------

fn contra_on(txn: &str, to: &str, from: &str, amount: &str, day: &str) -> Value {
    voucher(
        txn,
        "Contra",
        &[(to, amount, "Dr"), (from, amount, "Cr")],
        day,
    )
}

fn three_leg(txn: &str, day: &str) -> Value {
    voucher(
        txn,
        "Contra",
        &[
            (N2, "100.00", "Dr"),
            (N1, "60.00", "Cr"),
            (N3, "40.00", "Cr"),
        ],
        day,
    )
}

/// A verified statement of Bank Two over all of September, in a lineage of its
/// own, that skips one row.
fn september_of_bank_two(id: u32) -> Build {
    build(id)
        .facts(facts(B2).window("20260901", "20260930").skip(skip_row(
            &st(300 + id),
            "1.00",
            B3,
            None,
        )))
        .status("posted_verified")
}

#[test]
fn a_window_over_a_row_an_unverified_build_decides_is_no_cover() {
    // B0 booked Y at 500 and was verified; B1 changed it to 510 and was only
    // built; Bank Two's September, verified, skips Y (it is retired)
    let b0 = build(200)
        .voucher(contra(&st(50), N2, N1, "500.00"))
        .facts(facts(B2).contra(&st(50), None))
        .status("posted_verified");
    let b1 = build(201)
        .amends(200)
        .voucher(contra(&st(50), N2, N1, "510.00"));
    let september = build(202)
        .facts(
            facts(B2)
                .window("20260901", "20260930")
                .skip(skip_row(&st(50), "500.00", B1, None)),
        )
        .status("posted_verified");
    let journal = Journal::default().batch(&b0).batch(&b1).batch(&september);
    assert_eq!(journal.read().unwrap().retired_skips, 1);
    // the 500 Contra B0 posted is in Tally, so nothing proves a 500 skip has no other side
    assert_eq!(state_given(journal, ""), SkipState::NotEstablished);
}

#[test]
fn a_window_over_a_contra_left_out_is_no_cover() {
    let left_out = build(203)
        .voucher(three_leg(&st(51), "20260901"))
        .facts(facts(B2).contra(&st(51), None))
        .status("posted_verified");
    let journal = Journal::default()
        .batch(&left_out)
        .batch(&september_of_bank_two(204));
    assert_eq!(state_given(journal, ""), SkipState::NotEstablished);
}

#[test]
fn a_skip_only_window_over_an_unverified_row_is_no_cover() {
    let unverified = build(205)
        .voucher(contra(&st(52), N2, N1, "7.00"))
        .facts(facts(B2).contra(&st(52), None));
    let record = skip_record(
        20,
        ENDPOINT,
        "2026-09-02T10:00:00Z",
        facts(B2)
            .window("20260901", "20260930")
            .skip(skip_row(&st(9), "1.00", B3, None)),
    );
    assert_eq!(
        state_given(Journal::default().batch(&unverified), &skip_file(&[record])),
        SkipState::NotEstablished
    );
}

#[test]
fn a_blocked_date_blocks_only_its_own_endpoint_ledger_company_and_date() {
    // each of these rows is unverified, but none is Bank Two's, on this
    // endpoint, in this company, on a day the window holds
    let elsewhere = [
        build(206)
            .voucher(contra(&st(53), N3, N1, "7.00"))
            .facts(facts(B3).contra(&st(53), None)),
        build(207)
            .endpoint(Some(OTHER_ENDPOINT))
            .voucher(contra(&st(53), N2, N1, "7.00"))
            .facts(facts(B2).contra(&st(53), None)),
        build(208)
            .company(OTHER_COMPANY)
            .voucher(contra(&st(53), N2, N1, "7.00"))
            .facts(facts(B2).contra(&st(53), None)),
        build(209)
            .voucher(contra_on(&st(53), N2, N1, "7.00", "20261015"))
            .facts(facts(B2).contra(&st(53), None)),
        // the window's own last day is inside it
    ];
    for unverified in &elsewhere {
        let journal = Journal::default()
            .batch(unverified)
            .batch(&september_of_bank_two(210));
        assert_eq!(
            state_given(journal, ""),
            SkipState::NotFound,
            "{}",
            unverified.id
        );
    }
    let on_the_last_day = build(211)
        .voucher(contra_on(&st(53), N2, N1, "7.00", "20260930"))
        .facts(facts(B2).contra(&st(53), None));
    let journal = Journal::default()
        .batch(&on_the_last_day)
        .batch(&september_of_bank_two(212));
    assert_eq!(state_given(journal, ""), SkipState::NotEstablished);
    let on_the_first_day = build(213)
        .voucher(contra_on(&st(53), N2, N1, "7.00", "20260901"))
        .facts(facts(B2).contra(&st(53), None));
    let journal = Journal::default()
        .batch(&on_the_first_day)
        .batch(&september_of_bank_two(214));
    assert_eq!(state_given(journal, ""), SkipState::NotEstablished);
}

#[test]
fn every_date_a_row_has_had_is_blocked_while_its_latest_build_is_unverified() {
    // verified on 1 Sept, then moved to 25 Sept by a build that was only built
    let verified = build(215)
        .voucher(contra(&st(54), N2, N1, "500.00"))
        .facts(facts(B2).contra(&st(54), None))
        .status("posted_verified");
    let moved = build(216)
        .amends(215)
        .voucher(contra_on(&st(54), N2, N1, "500.00", "20260925"));
    let early_window = build(217)
        .facts(
            facts(B2)
                .window("20260901", "20260910")
                .skip(skip_row(&st(9), "1.00", B3, None)),
        )
        .status("posted_verified");
    let journal = Journal::default()
        .batch(&verified)
        .batch(&moved)
        .batch(&early_window);
    assert_eq!(state_given(journal, ""), SkipState::NotEstablished);
}

#[test]
fn a_lineage_with_a_row_outside_its_window_still_adds_no_cover() {
    // the row that makes the lineage unsure is on 25 Sept; its window is the
    // first ten days, which the blocked date does not touch
    let window = |f: StatementFacts| f.window("20260901", "20260910");
    let unverified = build(220)
        .voucher(contra_on(&st(55), N2, N1, "7.00", "20260925"))
        .facts(window(facts(B2)).contra(&st(55), None));
    let left_out = build(221)
        .voucher(three_leg(&st(55), "20260925"))
        .facts(window(facts(B2)).contra(&st(55), None))
        .status("posted_verified");
    let payment_then_contra = [
        build(222)
            .voucher(voucher(
                &st(56),
                "Payment",
                &[("Expense", "9.00", "Dr"), (N1, "9.00", "Cr")],
                "20260925",
            ))
            .facts(window(facts(B2)))
            .status("posted_verified"),
        build(223)
            .amends(222)
            .voucher(contra_on(&st(56), N2, N1, "9.00", "20260925"))
            .status("posted_verified"),
    ];
    assert_eq!(
        state_given(Journal::default().batch(&unverified), ""),
        SkipState::NotEstablished
    );
    assert_eq!(
        state_given(Journal::default().batch(&left_out), ""),
        SkipState::NotEstablished
    );
    let journal = Journal::default()
        .batch(&payment_then_contra[0])
        .batch(&payment_then_contra[1]);
    assert_eq!(state_given(journal, ""), SkipState::NotEstablished);
}

#[test]
fn a_contra_with_no_facts_entry_in_a_lineage_with_facts_is_counted_and_blocks_cover() {
    // B0 booked Y as a Payment; B1 retyped it as a Contra without facts
    let b0 = build(224)
        .voucher(payment(&st(57), "9.00"))
        .facts(facts(B2).window("20260901", "20260930"))
        .status("posted_verified");
    let b1 = build(225)
        .amends(224)
        .voucher(contra(&st(57), N2, N1, "9.00"))
        .status("posted_verified");
    let found = Journal::default().batch(&b0).batch(&b1).read().unwrap();
    assert_eq!(found.contras_without_facts, 1);
    assert!(found.contras.is_empty());
    assert_eq!(found.contras_not_two_leg, 0);
    // neither its own window nor another lineage's over its day is cover
    let journal = Journal::default()
        .batch(&b0)
        .batch(&b1)
        .batch(&september_of_bank_two(226));
    assert_eq!(state_given(journal, ""), SkipState::NotEstablished);
}

#[test]
fn a_contra_with_no_facts_in_a_lineage_without_facts_is_not_counted() {
    let hand_keyed = build(227).voucher(contra(&st(58), N2, N1, "9.00"));
    let found = Journal::default().batch(&hand_keyed).read().unwrap();
    assert_eq!(found.contras_without_facts, 0);
}

// ---- smaller points ----------------------------------------------------------------

#[test]
fn a_contra_is_superseded_only_by_a_batch_holding_a_statement_id() {
    // two unrelated Contras that share a hand-typed id are not one row
    let rebuilt = build(230)
        .voucher(contra("txn-shared", N2, N1, "100.00"))
        .facts(facts(B2).contra("txn-shared", None));
    let other = build(231)
        .voucher(contra("txn-shared", N2, N1, "100.00"))
        .facts(facts(B2).contra("txn-shared", None));
    let found = Journal::default()
        .batch(&rebuilt)
        .batch(&other)
        .sent(&other)
        .read()
        .unwrap();
    assert_eq!((found.contras.len(), found.contras_superseded), (2, 0));
}

#[test]
fn a_re_appended_record_gives_its_latest_binding_and_endpoint() {
    let first = build(240)
        .voucher(contra(&st(80), N2, N1, "1.00"))
        .facts(facts(B2).contra(&st(80), None))
        .status("posted_verified");
    let again = build(240)
        .endpoint(Some(OTHER_ENDPOINT))
        .identities(Some(vec![(N1, "guid-bank-one-b"), (N2, B2), (N3, B3)]))
        .voucher(contra(&st(80), N2, N1, "1.00"))
        .facts(facts(B2).contra(&st(80), None))
        .status("posted_verified");
    let found = Journal::default()
        .batch(&first)
        .batch(&again)
        .read()
        .unwrap();
    let fact = &found.contras[0];
    assert_eq!(fact.endpoint_origin, EndpointOrigin::new(OTHER_ENDPOINT));
    assert_eq!(fact.legs[1].ledger, LedgerGuid::new("guid-bank-one-b"));
    // and the facts are checked against it
    let unbound = build(240)
        .identities(Some(vec![(N1, B1), (N3, B3)]))
        .voucher(contra(&st(80), N2, N1, "1.00"))
        .facts(facts(B2).contra(&st(80), None));
    assert_eq!(
        Journal::default()
            .batch(&first)
            .batch(&unbound)
            .read()
            .unwrap_err(),
        TransferFactsError::Facts {
            source_id: batch_id(240),
            error: FactsError::BankLedgerUnbound
        }
    );
}

#[test]
fn a_statement_this_release_cannot_parse_does_not_make_the_journal_unreadable() {
    let mut line = build(250)
        .voucher(contra(&st(81), N2, N1, "1.00"))
        .status("posted_verified")
        .line();
    line.statement = Some(json!({"bank_ledger_guid": B2, "added_by_a_later_release": [1]}));
    let journal = Journal::default().push(&line);
    let text = journal.0.as_bytes();
    // every other reader goes on reading it
    let snapshot = ledger::read_snapshot(Cursor::new(text), Some(&batch_id(250)))
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.batch.statement, line.statement);
    assert_eq!(ledger::settlement(Cursor::new(text)).unwrap().batches, 1);
    assert_eq!(ledger::parse_snapshots(&journal.0).unwrap().len(), 1);
    // the fact builder is the one place it fails, as a typed error
    assert_eq!(
        journal.read().unwrap_err(),
        TransferFactsError::Facts {
            source_id: batch_id(250),
            error: FactsError::Unparsable
        }
    );
    // and only for the company it names
    line.company_guid = OTHER_COMPANY.into();
    assert!(Journal::default().push(&line).read().is_ok());
}

#[test]
fn a_statement_record_reads_back_as_the_facts_it_was_made_from() {
    let facts = valid();
    let recorded = statement_record(&facts);
    assert!(recorded.is_object());
    assert_eq!(
        serde_json::from_value::<StatementFacts>(recorded).unwrap(),
        facts
    );
}

// ---- live skips and a candidate record -------------------------------------------

fn skipping(id: u32, row: u32) -> Build {
    build(id).facts(facts(B1).skip(skip_row(&st(row), "500.00", B2, None)))
}

#[test]
fn a_skip_is_live_once_its_batch_is_sent_or_found_and_not_before() {
    let built = skipping(90, 9);
    let live = |journal: Journal| journal.read().unwrap().live_skips;
    assert!(live(Journal::default().batch(&built)).is_empty());
    // an incomplete verification is neither a dispatch nor a find
    assert!(live(
        Journal::default()
            .batch(&built)
            .status(&built, "verification_incomplete")
    )
    .is_empty());
    let wanted = BTreeSet::from([key(B1, 9)]);
    assert_eq!(live(Journal::default().batch(&built).sent(&built)), wanted);
    assert_eq!(
        live(
            Journal::default()
                .batch(&built)
                .status(&built, "posted_verified")
        ),
        wanted
    );
}

#[test]
fn a_skip_in_an_earlier_sent_build_stays_live_when_a_later_build_was_only_built() {
    let first = skipping(91, 9);
    let rebuilt =
        build(91 + 1)
            .amends(91)
            .facts(facts(B1).skip(skip_row(&st(9), "500.00", B2, Some(REF_A))));
    let found = Journal::default()
        .batch(&first)
        .sent(&first)
        .batch(&rebuilt)
        .read()
        .unwrap();
    assert_eq!(found.live_skips, BTreeSet::from([key(B1, 9)]));
    // the newest mention still gives the facts
    assert!(found.skips[0].reference.is_some());
}

#[test]
fn liveness_follows_the_row_not_the_mention_the_merge_kept() {
    // the newer lineage was only built, the older one was sent
    let older = skipping(92, 9);
    let newer = skipping(93, 9);
    let found = Journal::default()
        .batch(&older)
        .sent(&older)
        .batch(&newer)
        .read()
        .unwrap();
    assert_eq!(found.skips.len(), 1);
    assert_eq!(found.skips[0].lineage, LineageId::new(batch_id(93)));
    assert_eq!(found.live_skips, BTreeSet::from([key(B1, 9)]));
    // neither sent: neither live
    let found = Journal::default()
        .batch(&older)
        .batch(&newer)
        .read()
        .unwrap();
    assert!(found.live_skips.is_empty());
}

#[test]
fn a_skip_only_record_is_live() {
    let record = skip_record(
        7,
        ENDPOINT,
        "2026-09-02T10:00:00Z",
        facts(B1).skip(skip_row(&st(9), "500.00", B2, None)),
    );
    let found = Journal::default()
        .read_with(&skip_file(&[record]), COMPANY)
        .unwrap();
    assert_eq!(found.live_skips, BTreeSet::from([key(B1, 9)]));
}

#[test]
fn a_held_elsewhere_skip_is_live_by_its_own_source_and_a_retired_skip_is_not() {
    let sent = skipping(94, 9);
    let held = build(95).voucher(payment(&st(9), "500.00"));
    let found = Journal::default()
        .batch(&sent)
        .sent(&sent)
        .batch(&held)
        .read()
        .unwrap();
    assert_eq!(found.held_elsewhere.len(), 1);
    assert_eq!(found.live_skips, BTreeSet::from([key(B1, 9)]));
    // not sent: held elsewhere but not live
    let built = skipping(96, 9);
    let found = Journal::default()
        .batch(&built)
        .batch(&held)
        .read()
        .unwrap();
    assert_eq!(found.held_elsewhere.len(), 1);
    assert!(found.live_skips.is_empty());
    // the holder found posted retires the skip, which is then no skip at all
    let found = Journal::default()
        .batch(&sent)
        .sent(&sent)
        .batch(&held)
        .status(&held, "posted_verified")
        .read()
        .unwrap();
    assert_eq!(found.retired_skips, 1);
    assert!(found.live_skips.is_empty());
}

#[test]
fn a_candidate_build_is_read_as_the_newest_record_and_a_candidate_record_as_a_skip_file_line() {
    let held = build(97).voucher(payment(&st(9), "500.00"));
    let skipped = skipping(98, 9);
    let journal = Journal::default().batch(&skipped).sent(&skipped);
    let without = journal.read().unwrap();
    assert_eq!(without.skips.len(), 1);
    assert!(without.held_elsewhere.is_empty());
    // with a candidate that posts the row, the skip is held by the candidate
    let with = journal
        .read_candidate(
            "",
            COMPANY,
            Some(Candidate::Build(Box::new(BuildLine::of(&held.line())))),
        )
        .unwrap();
    assert!(with.skips.is_empty());
    assert_eq!(
        with.held_elsewhere,
        vec![(key(B1, 9), BatchId::new(batch_id(97)), date("20260901"))]
    );
    // a candidate skip-only record is a skip of its own
    let record = skip_record(
        8,
        ENDPOINT,
        "2026-09-03T10:00:00Z",
        facts(B1).skip(skip_row(&st(10), "500.00", B2, None)),
    );
    let with = journal
        .read_candidate("", COMPANY, Some(Candidate::SkipOnly(Box::new(record))))
        .unwrap();
    assert_eq!(with.skips.len(), 2);
    assert!(with.live_skips.contains(&key(B1, 10)));
}

// ---- rows a batch holds as vouchers, read from the journal alone -----------------

fn held(
    journal: &Journal,
    own_lineage: Option<u32>,
    rows: &[u32],
) -> Option<(String, Vec<String>)> {
    let rows: BTreeSet<String> = rows.iter().map(|row| st(*row)).collect();
    let own = own_lineage.map(batch_id);
    ledger::rows_held_as_vouchers(
        Cursor::new(journal.0.as_bytes()),
        COMPANY,
        own.as_deref(),
        &rows,
    )
    .unwrap()
    .map(|(batch, rows)| (batch, rows.into_iter().collect()))
}

#[test]
fn a_row_is_held_by_a_sent_or_found_batch_of_the_company_and_by_no_other() {
    let posting = build(100).voucher(payment(&st(1), "500.00"));
    // only built: held by no one, unless it is the asker's own lineage
    let built = Journal::default().batch(&posting);
    assert_eq!(held(&built, None, &[1]), None);
    // sent, or found posted
    let wanted = Some((batch_id(100), vec![st(1)]));
    assert_eq!(held(&built.sent(&posting), None, &[1]), wanted);
    let found = Journal::default()
        .batch(&posting)
        .status(&posting, "posted_verified");
    assert_eq!(held(&found, None, &[1]), wanted);
    // a batch found posted by its own record
    let recorded = build(100)
        .voucher(payment(&st(1), "500.00"))
        .status("posted_verified");
    assert_eq!(
        held(&Journal::default().batch(&recorded), None, &[1]),
        wanted
    );
    // a row it does not hold
    assert_eq!(held(&found, None, &[2]), None);
    // another company's batch holds nothing of this one's
    let elsewhere = build(101)
        .company(OTHER_COMPANY)
        .voucher(payment(&st(1), "500.00"))
        .status("posted_verified");
    assert_eq!(
        held(&Journal::default().batch(&elsewhere), None, &[1]),
        None
    );
}

#[test]
fn a_row_is_held_by_the_askers_own_lineage_at_any_status() {
    let original = build(110)
        .voucher(payment(&st(1), "500.00"))
        .voucher(payment(&st(2), "600.00"));
    let journal = Journal::default().batch(&original);
    let both = Some((batch_id(110), vec![st(1), st(2)]));
    assert_eq!(held(&journal, Some(110), &[1, 2]), both);
    // only the rows asked for are named
    assert_eq!(
        held(&journal, Some(110), &[2]),
        Some((batch_id(110), vec![st(2)]))
    );
    // an amendment is of the lineage its original names
    let amendment = build(111).amends(110).voucher(payment(&st(1), "500.00"));
    let journal = journal.batch(&amendment);
    assert_eq!(held(&journal, Some(110), &[1, 2]), both);
    // another lineage's built batch is not held against the asker
    assert_eq!(held(&journal, Some(120), &[1, 2]), None);
    assert_eq!(held(&journal, None, &[1, 2]), None);
}

#[test]
fn the_first_holding_batch_in_id_order_is_named() {
    let low = build(130).voucher(payment(&st(1), "500.00"));
    let high = build(131).voucher(payment(&st(1), "500.00"));
    let journal = Journal::default()
        .batch(&high)
        .sent(&high)
        .batch(&low)
        .sent(&low);
    assert_eq!(
        held(&journal, None, &[1]),
        Some((batch_id(130), vec![st(1)]))
    );
}
