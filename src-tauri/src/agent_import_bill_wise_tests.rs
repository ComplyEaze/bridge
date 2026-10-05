use super::super::{ImportEntry, ImportVoucher};
use super::*;

fn entry(ledger: &str, side: EntrySide, amount: &str) -> ImportEntry {
    ImportEntry {
        ledger: ledger.into(),
        amount: amount.into(),
        side,
    }
}

fn voucher(txn: &str, voucher_type: VoucherType, entries: Vec<ImportEntry>) -> ImportVoucher {
    ImportVoucher {
        bridge_txn_id: txn.into(),
        date: "20260401".into(),
        voucher_type,
        narration: None,
        reference: None,
        voucher_number: None,
        entries,
    }
}

fn payload(vouchers: Vec<ImportVoucher>) -> ImportPayload {
    ImportPayload {
        company_guid: "synthetic-company-guid".into(),
        vouchers,
        amends_batch_id: None,
    }
}

fn flag(name: &str, bill_wise_on: bool) -> NativeLedgerBillWiseFlag {
    NativeLedgerBillWiseFlag {
        name: name.into(),
        parent: Some("Synthetic Group".into()),
        bill_wise_on,
    }
}

/// The names `bill_wise` marks Yes, every other name in `all` marked No.
fn observed(all: &[&str], bill_wise: &[&str]) -> ObservedBillWise {
    let rows = all
        .iter()
        .map(|name| flag(name, bill_wise.contains(name)))
        .collect::<Vec<_>>();
    ObservedBillWise::new(
        all.iter().copied(),
        &[BillWiseRead {
            scope: BillWiseScope::Whole {
                catalogue_rows: rows.len(),
            },
            rows: &rows,
        }],
    )
    .unwrap()
}

fn row(txn: &str, voucher_type: VoucherType, entries: &[(EntrySide, &str)]) -> PartyRow {
    PartyRow {
        bridge_txn_id: txn.into(),
        voucher_type,
        date: "20260401".into(),
        entries: entries
            .iter()
            .map(|(side, amount)| (side.clone(), (*amount).to_string()))
            .collect(),
    }
}

fn party(ledger: &str, rows: Vec<PartyRow>) -> BillWiseParty {
    BillWiseParty {
        ledger: ledger.into(),
        rows,
    }
}

const ALL: [&str; 4] = ["Party A", "Party B", "Bank", "Other Bank"];

#[test]
fn a_payment_counterparty_on_a_bill_wise_ledger_is_a_party_row() {
    let payload = payload(vec![voucher(
        "txn-1",
        VoucherType::Payment,
        vec![
            entry("Party A", EntrySide::Dr, "100.00"),
            entry("Bank", EntrySide::Cr, "100.00"),
        ],
    )]);
    assert_eq!(
        bill_wise_parties(&payload.vouchers, &observed(&ALL, &["Party A"])),
        [party(
            "Party A",
            vec![row(
                "txn-1",
                VoucherType::Payment,
                &[(EntrySide::Dr, "100.00")]
            )]
        )]
    );
}

#[test]
fn a_receipt_counterparty_on_a_bill_wise_ledger_is_a_party_row() {
    let payload = payload(vec![voucher(
        "txn-2",
        VoucherType::Receipt,
        vec![
            entry("Bank", EntrySide::Dr, "250.50"),
            entry("Party A", EntrySide::Cr, "250.50"),
        ],
    )]);
    assert_eq!(
        bill_wise_parties(&payload.vouchers, &observed(&ALL, &["Party A"])),
        [party(
            "Party A",
            vec![row(
                "txn-2",
                VoucherType::Receipt,
                &[(EntrySide::Cr, "250.50")]
            )]
        )]
    );
}

#[test]
fn a_payment_or_receipt_money_leg_on_a_bill_wise_ledger_is_a_party_row() {
    let payload = payload(vec![
        voucher(
            "txn-3",
            VoucherType::Payment,
            vec![
                entry("Party A", EntrySide::Dr, "10.00"),
                entry("Bank", EntrySide::Cr, "10.00"),
            ],
        ),
        voucher(
            "txn-4",
            VoucherType::Receipt,
            vec![
                entry("Bank", EntrySide::Dr, "20.00"),
                entry("Party A", EntrySide::Cr, "20.00"),
            ],
        ),
    ]);
    assert_eq!(
        bill_wise_parties(&payload.vouchers, &observed(&ALL, &["Bank"])),
        [party(
            "Bank",
            vec![
                row("txn-3", VoucherType::Payment, &[(EntrySide::Cr, "10.00")]),
                row("txn-4", VoucherType::Receipt, &[(EntrySide::Dr, "20.00")]),
            ]
        )]
    );
}

#[test]
fn both_legs_of_a_contra_on_bill_wise_ledgers_are_party_rows() {
    let payload = payload(vec![voucher(
        "txn-5",
        VoucherType::Contra,
        vec![
            entry("Bank", EntrySide::Dr, "5.00"),
            entry("Other Bank", EntrySide::Cr, "5.00"),
        ],
    )]);
    let parties = bill_wise_parties(&payload.vouchers, &observed(&ALL, &["Bank", "Other Bank"]));
    assert_eq!(
        parties,
        [
            party(
                "Bank",
                vec![row(
                    "txn-5",
                    VoucherType::Contra,
                    &[(EntrySide::Dr, "5.00")]
                )]
            ),
            party(
                "Other Bank",
                vec![row(
                    "txn-5",
                    VoucherType::Contra,
                    &[(EntrySide::Cr, "5.00")]
                )]
            ),
        ]
    );
}

#[test]
fn the_debit_and_the_credit_of_a_journal_on_bill_wise_ledgers_are_party_rows() {
    let payload = payload(vec![voucher(
        "txn-6",
        VoucherType::Journal,
        vec![
            entry("Party A", EntrySide::Dr, "7.00"),
            entry("Party B", EntrySide::Cr, "7.00"),
        ],
    )]);
    let parties = bill_wise_parties(&payload.vouchers, &observed(&ALL, &["Party A", "Party B"]));
    assert_eq!(
        parties,
        [
            party(
                "Party A",
                vec![row(
                    "txn-6",
                    VoucherType::Journal,
                    &[(EntrySide::Dr, "7.00")]
                )]
            ),
            party(
                "Party B",
                vec![row(
                    "txn-6",
                    VoucherType::Journal,
                    &[(EntrySide::Cr, "7.00")]
                )]
            ),
        ]
    );
}

#[test]
fn several_entries_of_one_ledger_in_one_voucher_fold_into_one_row_in_entry_order() {
    let payload = payload(vec![voucher(
        "txn-7",
        VoucherType::Journal,
        vec![
            entry("Party A", EntrySide::Dr, "3.00"),
            entry("Bank", EntrySide::Cr, "9.00"),
            entry("Party A", EntrySide::Dr, "6.00"),
        ],
    )]);
    assert_eq!(
        bill_wise_parties(&payload.vouchers, &observed(&ALL, &["Party A"])),
        [party(
            "Party A",
            vec![row(
                "txn-7",
                VoucherType::Journal,
                &[(EntrySide::Dr, "3.00"), (EntrySide::Dr, "6.00")]
            )]
        )]
    );
}

#[test]
fn a_party_in_two_vouchers_is_one_party_with_its_rows_in_batch_order() {
    let payload = payload(vec![
        voucher(
            "txn-9",
            VoucherType::Payment,
            vec![
                entry("Party A", EntrySide::Dr, "1.00"),
                entry("Bank", EntrySide::Cr, "1.00"),
            ],
        ),
        voucher(
            "txn-8",
            VoucherType::Receipt,
            vec![
                entry("Bank", EntrySide::Dr, "2.00"),
                entry("Party A", EntrySide::Cr, "2.00"),
            ],
        ),
    ]);
    let parties = bill_wise_parties(&payload.vouchers, &observed(&ALL, &["Party A"]));
    assert_eq!(parties.len(), 1);
    let ids = parties[0]
        .rows
        .iter()
        .map(|row| row.bridge_txn_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, ["txn-9", "txn-8"]);
}

#[test]
fn no_party_is_returned_when_no_ledger_is_bill_wise() {
    let payload = payload(vec![voucher(
        "txn-1",
        VoucherType::Payment,
        vec![
            entry("Party A", EntrySide::Dr, "1.00"),
            entry("Bank", EntrySide::Cr, "1.00"),
        ],
    )]);
    assert!(bill_wise_parties(&payload.vouchers, &observed(&ALL, &[])).is_empty());
}

#[test]
fn a_ledger_the_observation_does_not_hold_counts_as_bill_wise() {
    // `ObservedBillWise::new` refuses a requested ledger that is absent, so
    // only a name nobody requested reaches this answer, and refusing is the
    // safe error.
    let payload = payload(vec![voucher(
        "txn-1",
        VoucherType::Journal,
        vec![
            entry("Stranger", EntrySide::Dr, "1.00"),
            entry("Bank", EntrySide::Cr, "1.00"),
        ],
    )]);
    let observed = observed(&["Bank"], &[]);
    let parties = bill_wise_parties(&payload.vouchers, &observed);
    assert_eq!(parties.len(), 1);
    assert_eq!(parties[0].ledger, "Stranger");
}

#[test]
fn the_named_ledgers_are_every_entrys_ledger_once() {
    let payload = payload(vec![
        voucher(
            "txn-1",
            VoucherType::Payment,
            vec![
                entry("Party A", EntrySide::Dr, "1.00"),
                entry("Bank", EntrySide::Cr, "1.00"),
            ],
        ),
        voucher(
            "txn-2",
            VoucherType::Journal,
            vec![
                entry("Party A", EntrySide::Dr, "2.00"),
                entry("Party B", EntrySide::Cr, "2.00"),
            ],
        ),
    ]);
    assert_eq!(
        named_ledgers(&payload).into_iter().collect::<Vec<_>>(),
        ["Bank", "Party A", "Party B"]
    );
}

#[test]
fn a_requested_ledger_absent_from_the_rows_fails_closed() {
    let rows = vec![flag("Party A", false)];
    assert_eq!(
        ObservedBillWise::new(
            ["Party A", "Party B"],
            &[BillWiseRead {
                scope: BillWiseScope::Whole { catalogue_rows: 1 },
                rows: &rows,
            }]
        ),
        Err(BillWiseError::LedgerAbsent)
    );
}

#[test]
fn a_name_repeated_in_one_read_or_across_reads_fails_closed() {
    let twice = vec![flag("Party A", false), flag("Party A", true)];
    assert_eq!(
        ObservedBillWise::new(
            ["Party A"],
            &[BillWiseRead {
                scope: BillWiseScope::Whole { catalogue_rows: 2 },
                rows: &twice,
            }]
        ),
        Err(BillWiseError::LedgerRepeated)
    );
    let first = vec![flag("Party A", false)];
    let second = vec![flag("Party A", true)];
    assert_eq!(
        ObservedBillWise::new(
            ["Party A"],
            &[
                BillWiseRead {
                    scope: BillWiseScope::Whole { catalogue_rows: 1 },
                    rows: &first,
                },
                BillWiseRead {
                    scope: BillWiseScope::Whole { catalogue_rows: 1 },
                    rows: &second,
                },
            ]
        ),
        Err(BillWiseError::LedgerRepeated)
    );
}

fn unit_limits() -> PartitionLimits {
    PartitionLimits {
        max_ledgers_per_part: 4,
        max_parents_per_part: 1,
        max_parts: 12,
        max_complement_formula_bytes: 10_000,
    }
}

/// `(name, guid, parent)` for `ledgers` ledgers, under `parents` in turn.
type CatalogueRow = (String, String, Option<&'static str>);

fn catalogue(under: &[(&'static str, usize)]) -> Vec<CatalogueRow> {
    under
        .iter()
        .flat_map(|(parent, count)| std::iter::repeat_n(*parent, *count))
        .enumerate()
        .map(|(index, parent)| (format!("L{index}"), format!("guid-{index}"), Some(parent)))
        .collect()
}

fn observations(rows: &[CatalogueRow]) -> Vec<(&str, &str, ParentObservation<'_>)> {
    rows.iter()
        .map(|(name, guid, parent)| {
            (
                name.as_str(),
                guid.as_str(),
                ParentObservation::from(*parent),
            )
        })
        .collect()
}

fn names<'a>(names: &[&'a str]) -> BTreeSet<&'a str> {
    names.iter().copied().collect()
}

fn parents_of(parts: &[ParentPart]) -> Vec<Vec<&str>> {
    parts
        .iter()
        .map(|part| {
            part.parents()
                .iter()
                .map(ParentName::as_catalogue_text)
                .collect()
        })
        .collect()
}

#[test]
fn a_catalogue_that_fits_one_part_is_read_whole() {
    let rows = catalogue(&[("Debtors", 2), ("Creditors", 2)]);
    assert_eq!(
        plan_reads(observations(&rows), unit_limits(), &names(&["L0"])),
        Ok(BillWiseReadPlan::Whole { catalogue_rows: 4 })
    );
}

#[test]
fn a_ledger_the_catalogue_lacks_is_refused_for_a_whole_read_too() {
    let rows = catalogue(&[("Debtors", 2)]);
    assert_eq!(
        plan_reads(observations(&rows), unit_limits(), &names(&["Missing"])),
        Err(BillWiseError::LedgerNotInCatalogue)
    );
}

#[test]
fn a_big_book_is_read_only_under_the_named_ledgers_parents_with_every_ledger_there() {
    // 9 ledgers: Debtors 2, Creditors 3, Banks 2, Stock 2. Only Debtors and
    // Banks are named, so only they are read, and all four of their ledgers.
    let rows = catalogue(&[("Debtors", 2), ("Creditors", 3), ("Banks", 2), ("Stock", 2)]);
    let plan = plan_reads(observations(&rows), unit_limits(), &names(&["L0", "L5"])).unwrap();
    let BillWiseReadPlan::Parts(parts) = plan else {
        panic!("a 9-ledger book against a 4-ledger part is not read whole");
    };
    let mut parents = parents_of(&parts).concat();
    parents.sort_unstable();
    assert_eq!(parents, ["Banks", "Debtors"]);
    assert_eq!(parts.iter().map(ParentPart::ledger_count).sum::<u64>(), 4);
    assert!(parts.iter().all(|part| !part.is_complement()));
}

#[test]
fn ledgers_sharing_a_parent_choose_its_part_once() {
    let rows = catalogue(&[("Debtors", 3), ("Creditors", 3), ("Stock", 3)]);
    let plan = plan_reads(
        observations(&rows),
        unit_limits(),
        &names(&["L0", "L1", "L2"]),
    )
    .unwrap();
    let BillWiseReadPlan::Parts(parts) = plan else {
        panic!("not read whole");
    };
    assert_eq!(parents_of(&parts), [vec!["Debtors"]]);
}

#[test]
fn a_named_ledger_whose_parent_cannot_be_named_in_a_big_book_is_refused() {
    let mut rows = catalogue(&[("Debtors", 3), ("Creditors", 3), ("Stock", 3)]);
    // A quote cannot be placed in a formula literal.
    rows[0].2 = Some("Bad \"Group\"");
    assert_eq!(
        plan_reads(observations(&rows), unit_limits(), &names(&["L0"])),
        Err(BillWiseError::ParentNotNameable)
    );
}

#[test]
fn a_named_ledger_with_no_parent_in_a_big_book_is_refused_as_not_nameable() {
    let mut rows = catalogue(&[("Debtors", 3), ("Creditors", 3), ("Stock", 3)]);
    rows[0].2 = None;
    assert_eq!(
        plan_reads(observations(&rows), unit_limits(), &names(&["L0"])),
        Err(BillWiseError::ParentNotNameable)
    );
}

#[test]
fn a_parent_holding_more_ledgers_than_a_part_is_a_too_large_refusal() {
    let rows = catalogue(&[("Debtors", 5), ("Creditors", 2)]);
    let error = plan_reads(observations(&rows), unit_limits(), &names(&["L0"])).unwrap_err();
    assert_eq!(error, BillWiseError::ParentOverBudget);
    assert_eq!(error.reason(), "bill_wise_read_too_large");
}

#[test]
fn parents_needing_more_parts_than_the_plan_allows_are_a_too_large_refusal() {
    let rows = catalogue(&[("A", 3), ("B", 3), ("C", 3)]);
    let limits = PartitionLimits {
        max_parts: 1,
        ..unit_limits()
    };
    let error = plan_reads(observations(&rows), limits, &names(&["L0", "L3", "L6"])).unwrap_err();
    assert_eq!(error, BillWiseError::TooManyParts);
    assert_eq!(error.reason(), "bill_wise_read_too_large");
}

#[test]
fn more_than_four_parts_is_a_too_large_refusal_even_inside_the_plans_own_limit() {
    let rows = catalogue(&[("A", 3), ("B", 3), ("C", 3), ("D", 3), ("E", 3)]);
    let error = plan_reads(
        observations(&rows),
        unit_limits(),
        &names(&["L0", "L3", "L6", "L9", "L12"]),
    )
    .unwrap_err();
    assert_eq!(error, BillWiseError::TooManyParts);
}

#[test]
fn a_repeated_ledger_guid_is_refused_with_the_protocols_own_code() {
    let mut rows = catalogue(&[("Debtors", 3), ("Creditors", 3), ("Stock", 3)]);
    rows[1].1 = rows[0].1.clone();
    let error = plan_reads(observations(&rows), unit_limits(), &names(&["L0"])).unwrap_err();
    assert_eq!(
        error,
        BillWiseError::Unplannable("parent_partition_duplicate_ledger_identity")
    );
    assert_eq!(error.reason(), "bill_wise_not_established");
}

#[test]
fn every_error_has_a_distinct_cause_and_only_two_are_too_large() {
    let errors = [
        BillWiseError::BooksFromAbsent,
        BillWiseError::PeriodUnsupported,
        BillWiseError::LedgerNotInCatalogue,
        BillWiseError::ParentNotNameable,
        BillWiseError::ParentOverBudget,
        BillWiseError::TooManyParts,
        BillWiseError::Unplannable("x"),
        BillWiseError::RowCountDiffers,
        BillWiseError::LedgerAbsent,
        BillWiseError::LedgerRepeated,
    ];
    let causes = errors
        .iter()
        .map(|error| error.cause())
        .collect::<BTreeSet<_>>();
    assert_eq!(causes.len(), errors.len());
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.reason() == "bill_wise_read_too_large")
            .count(),
        2
    );
}

#[test]
fn a_part_whose_row_count_differs_from_the_catalogues_fails_closed() {
    let rows = catalogue(&[("Debtors", 3), ("Creditors", 3), ("Stock", 3)]);
    let plan = plan_reads(observations(&rows), unit_limits(), &names(&["L0"])).unwrap();
    let BillWiseReadPlan::Parts(parts) = plan else {
        panic!("not read whole");
    };
    let too_few = vec![flag("L0", false), flag("L1", false)];
    assert_eq!(
        ObservedBillWise::new(
            ["L0"],
            &[BillWiseRead {
                scope: BillWiseScope::Part(&parts[0]),
                rows: &too_few,
            }]
        ),
        Err(BillWiseError::RowCountDiffers)
    );
    let exact = vec![flag("L0", false), flag("L1", false), flag("L2", true)];
    assert!(ObservedBillWise::new(
        ["L0"],
        &[BillWiseRead {
            scope: BillWiseScope::Part(&parts[0]),
            rows: &exact,
        }]
    )
    .is_ok());
}

#[test]
fn a_whole_read_whose_row_count_differs_from_the_catalogues_fails_closed() {
    let rows = vec![flag("Party A", false), flag("Party B", false)];
    assert_eq!(
        ObservedBillWise::new(
            ["Party A"],
            &[BillWiseRead {
                scope: BillWiseScope::Whole { catalogue_rows: 3 },
                rows: &rows,
            }]
        ),
        Err(BillWiseError::RowCountDiffers)
    );
}

fn date(text: &str) -> bridge_tally_core::TallyDate {
    bridge_tally_core::TallyDate::parse(text.to_string()).unwrap()
}

#[test]
fn the_period_is_the_one_day_books_from() {
    let period = bill_wise_period(Some("20260401"), DateBoundaryProfile::ModeAgnostic).unwrap();
    assert_eq!(period.from(), &date("20260401"));
    assert_eq!(period.to(), &date("20260401"));
    let dashed = bill_wise_period(Some("2026-04-01"), DateBoundaryProfile::ModeAgnostic).unwrap();
    assert_eq!(dashed.from(), &date("20260401"));
}

#[test]
fn an_absent_or_unreadable_books_from_has_no_period() {
    assert_eq!(
        bill_wise_period(None, DateBoundaryProfile::ModeAgnostic),
        Err(BillWiseError::BooksFromAbsent)
    );
    assert_eq!(
        bill_wise_period(Some("not-a-date"), DateBoundaryProfile::ModeAgnostic),
        Err(BillWiseError::PeriodUnsupported)
    );
}

#[test]
fn an_education_licence_refuses_a_books_from_that_is_not_a_boundary_day() {
    assert_eq!(
        bill_wise_period(Some("20260415"), DateBoundaryProfile::EducationRestricted),
        Err(BillWiseError::PeriodUnsupported)
    );
    assert!(bill_wise_period(Some("20260401"), DateBoundaryProfile::EducationRestricted).is_ok());
}

// ---- digests

fn company() -> ImportCompanyTuple {
    ImportCompanyTuple {
        name: "Synthetic Company".into(),
        guid: "synthetic-guid".into(),
        company_number: "1".into(),
        books_from: "20260401".into(),
    }
}

fn batch() -> Vec<ImportVoucher> {
    vec![
        voucher(
            "txn-1",
            VoucherType::Payment,
            vec![
                entry("Party A", EntrySide::Dr, "100.00"),
                entry("Bank", EntrySide::Cr, "100.00"),
            ],
        ),
        voucher(
            "txn-2",
            VoucherType::Receipt,
            vec![
                entry("Bank", EntrySide::Dr, "40.00"),
                entry("Party A", EntrySide::Cr, "40.00"),
            ],
        ),
    ]
}

fn context<'a>(company: &'a ImportCompanyTuple, vouchers: &[ImportVoucher]) -> DigestContext<'a> {
    DigestContext {
        company,
        endpoint_origin: "http://127.0.0.1:9001",
        amends_batch_id: None,
        batch_content: batch_content_digest(vouchers),
    }
}

fn party_a(vouchers: &[ImportVoucher]) -> BillWiseParty {
    bill_wise_parties(vouchers, &observed(&ALL, &["Party A"])).remove(0)
}

/// Pinned: a change to the encoding, to a field it binds, or to the order it
/// binds them in moves this value, so a reader of the diff must look.
#[test]
fn the_party_digest_of_a_known_batch_is_pinned() {
    let vouchers = batch();
    let company = company();
    assert_eq!(
        party_digest(&context(&company, &vouchers), &party_a(&vouchers)),
        "6e2ced507f370b1dbb787bca0267b14bdd82b965277c1466964d09da6ee2022a"
    );
}

#[test]
fn each_field_the_digest_binds_moves_it() {
    let vouchers = batch();
    let company = company();
    let baseline = party_digest(&context(&company, &vouchers), &party_a(&vouchers));
    let mut seen = BTreeSet::from([baseline.clone()]);
    let mut moved = |digest: String| {
        assert_ne!(digest, baseline);
        assert!(seen.insert(digest), "two changes gave one digest");
    };

    for change in 0..4 {
        let mut other = company.clone();
        match change {
            0 => other.name.push('!'),
            1 => other.guid.push('!'),
            2 => other.company_number.push('!'),
            _ => other.books_from = "20250401".into(),
        }
        moved(party_digest(
            &context(&other, &vouchers),
            &party_a(&vouchers),
        ));
    }

    let mut other_endpoint = context(&company, &vouchers);
    other_endpoint.endpoint_origin = "http://127.0.0.1:9002";
    moved(party_digest(&other_endpoint, &party_a(&vouchers)));

    let mut amendment = context(&company, &vouchers);
    amendment.amends_batch_id = Some("bridge-original");
    moved(party_digest(&amendment, &party_a(&vouchers)));

    let mut other_ledger = party_a(&vouchers);
    other_ledger.ledger = "Party B".into();
    moved(party_digest(&context(&company, &vouchers), &other_ledger));

    let mut fewer_rows = party_a(&vouchers);
    fewer_rows.rows.pop();
    moved(party_digest(&context(&company, &vouchers), &fewer_rows));

    let mut other_amount = party_a(&vouchers);
    other_amount.rows[0].entries[0].1 = "100.01".into();
    moved(party_digest(&context(&company, &vouchers), &other_amount));

    let mut other_side = party_a(&vouchers);
    other_side.rows[0].entries[0].0 = EntrySide::Cr;
    moved(party_digest(&context(&company, &vouchers), &other_side));

    let mut other_date = party_a(&vouchers);
    other_date.rows[0].date = "20260402".into();
    moved(party_digest(&context(&company, &vouchers), &other_date));

    let mut other_type = party_a(&vouchers);
    other_type.rows[0].voucher_type = VoucherType::Journal;
    moved(party_digest(&context(&company, &vouchers), &other_type));

    let mut other_txn = party_a(&vouchers);
    other_txn.rows[0].bridge_txn_id = "txn-9".into();
    moved(party_digest(&context(&company, &vouchers), &other_txn));
}

#[test]
fn a_change_to_any_voucher_field_moves_the_batch_digest_and_so_every_partys_digest() {
    let baseline = batch_content_digest(&batch());
    type Edit = Box<dyn Fn(&mut ImportVoucher)>;
    let edits: Vec<Edit> = vec![
        Box::new(|v| v.narration = Some("note".into())),
        Box::new(|v| v.reference = Some("ref".into())),
        Box::new(|v| v.voucher_number = Some("7".into())),
        Box::new(|v| v.date = "20260402".into()),
        Box::new(|v| v.bridge_txn_id = "txn-x".into()),
        Box::new(|v| v.voucher_type = VoucherType::Journal),
        Box::new(|v| v.entries[1].ledger = "Other Bank".into()),
        Box::new(|v| v.entries[1].amount = "100.01".into()),
        Box::new(|v| v.entries[1].side = EntrySide::Dr),
        Box::new(|v| v.entries.pop().map(drop).unwrap_or(())),
    ];
    let mut digests = BTreeSet::from([baseline]);
    for edit in edits {
        let mut vouchers = batch();
        edit(&mut vouchers[0]);
        assert!(
            digests.insert(batch_content_digest(&vouchers)),
            "an edit left the digest as it was"
        );
    }
    // Another party's rows are untouched, yet its digest moves with the batch.
    let company = company();
    let original = batch();
    let mut changed = batch();
    changed[1].narration = Some("note".into());
    let bank_party = |vouchers: &[ImportVoucher]| {
        bill_wise_parties(vouchers, &observed(&ALL, &["Bank"])).remove(0)
    };
    assert_ne!(
        party_digest(&context(&company, &original), &bank_party(&original)),
        party_digest(&context(&company, &changed), &bank_party(&changed))
    );
}

#[test]
fn an_absent_optional_field_and_an_empty_one_are_different_batches() {
    let mut empty = batch();
    empty[0].narration = Some(String::new());
    assert_ne!(batch_content_digest(&batch()), batch_content_digest(&empty));
}

// ---- approvals

fn approvals_args(items: Value) -> Value {
    json!({"company_guid": "g", "vouchers": [], APPROVALS_KEY: items})
}

fn digest_of(ledger: &str, vouchers: &[ImportVoucher], bill_wise: &[&str]) -> String {
    let company = company();
    let parties = bill_wise_parties(vouchers, &observed(&ALL, bill_wise));
    let party = parties.iter().find(|party| party.ledger == ledger).unwrap();
    party_digest(&context(&company, vouchers), party)
}

#[test]
fn the_approvals_key_is_removed_from_the_arguments_whatever_it_holds() {
    let mut args = approvals_args(json!([]));
    assert_eq!(take_approvals(&mut args), Ok(Vec::new()));
    assert!(args.get(APPROVALS_KEY).is_none());
    let mut absent = json!({"company_guid": "g", "vouchers": []});
    assert_eq!(take_approvals(&mut absent), Ok(Vec::new()));
    let mut malformed = approvals_args(json!("nope"));
    assert_eq!(
        take_approvals(&mut malformed),
        Err(ApprovalError::Malformed)
    );
    assert!(malformed.get(APPROVALS_KEY).is_none());
}

#[test]
fn a_well_formed_approval_is_parsed() {
    let digest = "a".repeat(64);
    let mut args = approvals_args(json!([{"ledger": "Party A", "party_digest": digest}]));
    assert_eq!(
        take_approvals(&mut args),
        Ok(vec![OnAccountApproved {
            ledger: "Party A".into(),
            party_digest: digest
        }])
    );
}

#[test]
fn a_malformed_approval_is_refused_not_dropped() {
    let good = "a".repeat(64);
    let cases = [
        json!({"ledger": "Party A", "party_digest": good}),
        json!([["Party A", good]]),
        json!([{"ledger": "Party A"}]),
        json!([{"party_digest": good}]),
        json!([{"ledger": "Party A", "party_digest": good, "extra": 1}]),
        json!([{"ledger": "", "party_digest": good}]),
        json!([{"ledger": "Party A", "party_digest": "a".repeat(63)}]),
        json!([{"ledger": "Party A", "party_digest": "A".repeat(64)}]),
        json!([{"ledger": "Party A", "party_digest": "g".repeat(64)}]),
        json!([{"ledger": 7, "party_digest": good}]),
    ];
    for case in cases {
        let mut args = approvals_args(case.clone());
        assert_eq!(
            take_approvals(&mut args),
            Err(ApprovalError::Malformed),
            "{case}"
        );
    }
}

#[test]
fn one_ledger_approved_twice_is_refused() {
    let digest = "a".repeat(64);
    let mut args = approvals_args(json!([
        {"ledger": "Party A", "party_digest": digest},
        {"ledger": "Party A", "party_digest": "b".repeat(64)},
    ]));
    assert_eq!(take_approvals(&mut args), Err(ApprovalError::Duplicate));
}

#[test]
fn a_ledger_that_differs_only_by_a_trailing_crlf_is_a_different_ledger() {
    let digest = "a".repeat(64);
    let mut args = approvals_args(json!([
        {"ledger": "Party A", "party_digest": digest},
        {"ledger": "Party A\r\n", "party_digest": digest},
    ]));
    assert_eq!(take_approvals(&mut args).unwrap().len(), 2);
}

#[test]
fn an_approval_for_a_ledger_that_is_not_a_party_of_the_batch_is_refused() {
    let vouchers = batch();
    let company = company();
    let parties = bill_wise_parties(&vouchers, &observed(&ALL, &["Party A"]));
    let approval = OnAccountApproved {
        ledger: "Party B".into(),
        party_digest: "a".repeat(64),
    };
    assert_eq!(
        judge_approvals(&[approval], &parties, &context(&company, &vouchers)).err(),
        Some(ApprovalError::UnknownLedger)
    );
}

#[test]
fn an_approval_that_names_a_party_whose_ledger_is_not_bill_wise_is_refused() {
    let vouchers = batch();
    let company = company();
    // Bank is a ledger of the batch, but not bill-wise, so not a party.
    let parties = bill_wise_parties(&vouchers, &observed(&ALL, &["Party A"]));
    let approval = OnAccountApproved {
        ledger: "Bank".into(),
        party_digest: "a".repeat(64),
    };
    assert_eq!(
        judge_approvals(&[approval], &parties, &context(&company, &vouchers)).err(),
        Some(ApprovalError::UnknownLedger)
    );
}

#[test]
fn an_approval_with_another_digest_is_refused() {
    let vouchers = batch();
    let company = company();
    let parties = bill_wise_parties(&vouchers, &observed(&ALL, &["Party A"]));
    let approval = OnAccountApproved {
        ledger: "Party A".into(),
        party_digest: "a".repeat(64),
    };
    assert_eq!(
        judge_approvals(&[approval], &parties, &context(&company, &vouchers)).err(),
        Some(ApprovalError::DigestDiffers)
    );
}

#[test]
fn an_approval_made_for_another_company_or_an_edited_batch_does_not_carry() {
    let vouchers = batch();
    let company = company();
    let parties = bill_wise_parties(&vouchers, &observed(&ALL, &["Party A"]));
    let approval = OnAccountApproved {
        ledger: "Party A".into(),
        party_digest: digest_of("Party A", &vouchers, &["Party A"]),
    };
    // A year-end split gives the child its parent's GUID but another number.
    let mut split = company.clone();
    split.company_number = "2".into();
    assert_eq!(
        judge_approvals(
            std::slice::from_ref(&approval),
            &parties,
            &context(&split, &vouchers)
        )
        .err(),
        Some(ApprovalError::DigestDiffers)
    );
    let mut edited = batch();
    edited[1].narration = Some("changed".into());
    let edited_parties = bill_wise_parties(&edited, &observed(&ALL, &["Party A"]));
    assert_eq!(
        judge_approvals(
            std::slice::from_ref(&approval),
            &edited_parties,
            &context(&company, &edited)
        )
        .err(),
        Some(ApprovalError::DigestDiffers)
    );
}

#[test]
fn a_party_without_an_approval_is_unapproved_and_the_rest_are_recorded_sorted() {
    let company = company();
    let vouchers = vec![voucher(
        "txn-1",
        VoucherType::Journal,
        vec![
            entry("Party B", EntrySide::Dr, "5.00"),
            entry("Party A", EntrySide::Cr, "5.00"),
        ],
    )];
    let both = ["Party A", "Party B"];
    let parties = bill_wise_parties(&vouchers, &observed(&ALL, &both));
    let approval_b = OnAccountApproved {
        ledger: "Party B".into(),
        party_digest: digest_of("Party B", &vouchers, &both),
    };
    let verdict = judge_approvals(
        std::slice::from_ref(&approval_b),
        &parties,
        &context(&company, &vouchers),
    )
    .unwrap();
    assert_eq!(verdict.approved, std::slice::from_ref(&approval_b));
    assert_eq!(verdict.unapproved.len(), 1);
    assert_eq!(verdict.unapproved[0].0.ledger, "Party A");
    assert_eq!(
        verdict.unapproved[0].1,
        digest_of("Party A", &vouchers, &both)
    );

    let approval_a = OnAccountApproved {
        ledger: "Party A".into(),
        party_digest: digest_of("Party A", &vouchers, &both),
    };
    let all = judge_approvals(
        &[approval_b.clone(), approval_a.clone()],
        &parties,
        &context(&company, &vouchers),
    )
    .unwrap();
    assert!(all.unapproved.is_empty());
    assert_eq!(all.approved, [approval_a, approval_b]);
}

#[test]
fn a_batch_with_no_bill_wise_party_needs_and_records_no_approval() {
    let vouchers = batch();
    let company = company();
    let verdict = judge_approvals(&[], &[], &context(&company, &vouchers)).unwrap();
    assert!(verdict.approved.is_empty());
    assert!(verdict.unapproved.is_empty());
}

// ---- the refusal's party list

fn big_party(ledger: &str, rows: usize) -> BillWiseParty {
    party(
        ledger,
        (0..rows)
            .map(|index| {
                row(
                    &format!("txn-{index:03}"),
                    VoucherType::Journal,
                    &[(EntrySide::Dr, "1.25")],
                )
            })
            .collect(),
    )
}

#[test]
fn a_party_list_that_fits_shows_every_row_and_the_exact_totals() {
    let mixed = party(
        "Party A",
        vec![
            row("txn-1", VoucherType::Payment, &[(EntrySide::Dr, "100.10")]),
            row("txn-2", VoucherType::Receipt, &[(EntrySide::Cr, "40.05")]),
            row("txn-3", VoucherType::Journal, &[(EntrySide::Dr, "0.20")]),
        ],
    );
    let unapproved = vec![(&mixed, "d".repeat(64))];
    let (shown, omitted) = refused_parties_json(&unapproved, 1 << 20);
    assert_eq!(omitted, 0);
    assert_eq!(shown.len(), 1);
    assert_eq!(shown[0]["row_count"], 3);
    assert_eq!(shown[0]["rows_omitted"], 0);
    assert_eq!(shown[0]["rows"].as_array().unwrap().len(), 3);
    assert_eq!(shown[0]["debit_total"], "100.3");
    assert_eq!(shown[0]["credit_total"], "40.05");
    assert_eq!(shown[0]["party_digest"], "d".repeat(64));
    assert_eq!(shown[0]["rows"][0]["date"], "20260401");
}

#[test]
fn rows_past_the_budget_are_counted_not_cut_and_the_party_keeps_its_digest() {
    let big = big_party("Party A", 50);
    let unapproved = vec![(&big, "d".repeat(64))];
    let (shown, omitted) = refused_parties_json(&unapproved, 700);
    assert_eq!(omitted, 0);
    let kept = shown[0]["rows"].as_array().unwrap().len();
    assert!(kept > 0 && kept < 50, "{kept}");
    assert_eq!(shown[0]["rows_omitted"], 50 - kept);
    assert_eq!(shown[0]["row_count"], 50);
    assert_eq!(shown[0]["debit_total"], "62.5");
    assert_eq!(shown[0]["party_digest"], "d".repeat(64));
}

#[test]
fn a_party_whose_header_does_not_fit_is_counted_as_omitted() {
    let first = big_party("Party A", 2);
    let second = big_party("Party B", 2);
    let unapproved = vec![(&first, "d".repeat(64)), (&second, "e".repeat(64))];
    let (shown, omitted) = refused_parties_json(&unapproved, 10);
    assert!(shown.is_empty());
    assert_eq!(omitted, 2);
}

#[test]
fn a_total_that_cannot_be_added_is_absent_rather_than_guessed() {
    let broken = party(
        "Party A",
        vec![row(
            "txn-1",
            VoucherType::Journal,
            &[(EntrySide::Dr, "not-money")],
        )],
    );
    let (debit, credit) = side_totals(&broken);
    assert_eq!(debit, None);
    assert_eq!(credit.as_deref(), Some("0"));
}

#[test]
fn every_refusal_has_its_own_plain_sentence_that_names_no_code() {
    let errors = [
        BillWiseError::BooksFromAbsent,
        BillWiseError::PeriodUnsupported,
        BillWiseError::LedgerNotInCatalogue,
        BillWiseError::ParentNotNameable,
        BillWiseError::ParentOverBudget,
        BillWiseError::TooManyParts,
        BillWiseError::Unplannable("x"),
        BillWiseError::RowCountDiffers,
        BillWiseError::LedgerAbsent,
        BillWiseError::LedgerRepeated,
    ];
    let sentences = errors
        .iter()
        .map(|error| error.plain())
        .collect::<BTreeSet<_>>();
    assert_eq!(sentences.len(), errors.len());
    for error in errors {
        let sentence = error.plain();
        assert!(sentence.ends_with('.') && sentence.len() > 40, "{sentence}");
        // A person is told this sentence, so no snake_case code is in it.
        assert!(!sentence.contains('_'), "{sentence}");
    }
}

#[test]
fn two_observations_differ_only_in_the_ledgers_asked_about() {
    let all = ["Party A", "Party B", "Bank"];
    let before = observed(&all, &[]);
    let party_b_flips = observed(&all, &["Party B"]);
    let asked = names(&["Party A", "Bank"]);
    // Party B is not asked about, so its flip is not a difference.
    assert_eq!(before.flags_of(&asked), party_b_flips.flags_of(&asked));
    // A named ledger's flip is.
    assert_ne!(
        before.flags_of(&names(&["Party B"])),
        party_b_flips.flags_of(&names(&["Party B"]))
    );
    // A named ledger absent from an observation is not read as "not bill-wise".
    assert_eq!(
        before.flags_of(&names(&["Stranger"])).get("Stranger"),
        Some(&None)
    );
}

// ---- Slice 1 on captures of a live Tally ------------------------------------------------------------------
//
// The answers below were read from the synthetic company BRIDGE OUTSTANDINGS LAB (TallyPrime Silver 7.1) by the
// requests this module chooses: the ledger catalogue (1 Oct 2026; the book is unchanged since, which the 6 Oct
// sitting confirmed name by name), the one-day snapshot of the whole book, and the same snapshot filtered to the
// ledgers under two parents (both 6 Oct 2026). Their provenance files give the request and response hashes. The
// build tests elsewhere still run on regression doubles; these run this module's own decisions on what Tally
// answered. The expected flags are the seeding of the book, not this code's output.

const LIVE_COMPANY: &str = "BRIDGE OUTSTANDINGS LAB";
const LIVE_GUID: &str = "49f1fbda-ee59-4a4b-aacf-b45fe32402d7";
const LIVE_CATALOGUE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-ledger-catalogue.utf16le.xml"
);
const LIVE_WHOLE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/bill-wise-snapshot-oneday-outstandings-lab.utf16le.xml"
);
const LIVE_PARTS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/bill-wise-snapshot-parents-oneday-outstandings-lab.utf16le.xml"
);
const LIVE_CATALOGUE_PROVENANCE: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-ledger-catalogue.json"
);
const LIVE_WHOLE_PROVENANCE: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/bill-wise-snapshot-oneday-outstandings-lab.json"
);
const LIVE_PARTS_PROVENANCE: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/bill-wise-snapshot-parents-oneday-outstandings-lab.json"
);
const LIVE_PARTY: &str = "OL P01 Named Bills Debtor";
const LIVE_CREDITOR: &str = "OL P06 Debit Note Creditor";
const LIVE_BANK: &str = "OL Bank";

fn live_text(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .expect("a UTF-16LE capture")
}

fn live_catalogue() -> bridge_tally_protocol::StandardLedgerCatalog {
    crate::tally::standard_ledger_catalog::parse_standard_ledger_catalog_response(
        &live_text(LIVE_CATALOGUE),
        LIVE_COMPANY,
        LIVE_GUID,
    )
    .expect("the live catalogue parses")
}

fn live_rows(bytes: &[u8]) -> Vec<NativeLedgerBillWiseFlag> {
    bridge_tally_protocol::native_outstandings::parse_native_ledger_bill_wise_flags_for_company(
        &live_text(bytes),
        LIVE_GUID,
    )
    .expect("the live snapshot parses")
}

fn live_period() -> NativeLedgerSnapshotPeriod {
    // The company's books_from, as the build derives it.
    bill_wise_period(Some("20250401"), DateBoundaryProfile::ModeAgnostic).unwrap()
}

fn recorded_request_sha256(provenance: &str) -> String {
    serde_json::from_str::<Value>(provenance).unwrap()["source_request_sha256"]
        .as_str()
        .unwrap()
        .to_string()
}

fn wire_sha256(xml: &str) -> String {
    super::super::sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        xml,
    ))
}

fn live_limits_of_one_part_of_eleven() -> PartitionLimits {
    PartitionLimits {
        max_ledgers_per_part: 11,
        max_parents_per_part: 200,
        max_parts: 12,
        max_complement_formula_bytes: 262_144,
    }
}

#[test]
fn live_a_book_that_fits_one_part_is_read_whole_and_the_catalogue_names_match_the_snapshot() {
    let catalogue = live_catalogue();
    let rows = live_rows(LIVE_WHOLE);
    assert_eq!(catalogue.names().count(), 17);
    assert_eq!(
        catalogue.names().collect::<BTreeSet<_>>(),
        rows.iter().map(|row| row.name.as_str()).collect()
    );
    // The catalogue and the snapshot agree on every ledger's parent, not only on its name.
    for (name, parent) in catalogue.parents() {
        let row = rows.iter().find(|row| row.name == name).unwrap();
        assert_eq!(row.parent.as_deref(), parent, "{name}");
    }
    let named = names(&[LIVE_PARTY, LIVE_BANK]);
    assert_eq!(
        plan_reads(
            catalogue.identified_parents(),
            crate::tally::connection::parent_partition_limits(),
            &named
        ),
        Ok(BillWiseReadPlan::Whole { catalogue_rows: 17 })
    );
    let observed = ObservedBillWise::new(
        named.iter().copied(),
        &[BillWiseRead {
            scope: BillWiseScope::Whole { catalogue_rows: 17 },
            rows: &rows,
        }],
    )
    .expect("the live snapshot is the catalogue's ledgers");
    let flags = observed.flags_of(&named);
    assert_eq!(flags[LIVE_PARTY], Some(true));
    assert_eq!(flags[LIVE_BANK], Some(false));
    // A catalogue that disagrees by one ledger is a different book: the check fails closed.
    assert_eq!(
        ObservedBillWise::new(
            named.iter().copied(),
            &[BillWiseRead {
                scope: BillWiseScope::Whole { catalogue_rows: 16 },
                rows: &rows,
            }],
        ),
        Err(BillWiseError::RowCountDiffers)
    );
    // A short answer is the dangerous case: 11 rows of a 17-ledger book.
    assert_eq!(
        ObservedBillWise::new(
            named.iter().copied(),
            &[BillWiseRead {
                scope: BillWiseScope::Whole { catalogue_rows: 17 },
                rows: &live_rows(LIVE_PARTS),
            }],
        ),
        Err(BillWiseError::RowCountDiffers)
    );
}

#[test]
fn live_the_requests_the_build_dispatches_are_the_ones_that_were_sent() {
    // Through the wrappers the build sends with, not the protocol renderers they call.
    let whole = super::super::ledger_bill_wise_whole_read(LIVE_COMPANY, &live_period());
    assert_eq!(
        wire_sha256(whole.as_str()),
        recorded_request_sha256(LIVE_WHOLE_PROVENANCE)
    );
    let catalogue = super::super::standard_ledger_catalog_read(LIVE_COMPANY).unwrap();
    assert_eq!(
        wire_sha256(catalogue.as_str()),
        recorded_request_sha256(LIVE_CATALOGUE_PROVENANCE)
    );
}

#[test]
fn live_a_book_above_one_part_is_read_by_the_part_under_the_named_parents_and_that_part_answers_exactly(
) {
    let catalogue = live_catalogue();
    // Eleven ledgers a part: the book (17) is above one part, as a large book is.
    let named = names(&[LIVE_PARTY, LIVE_CREDITOR]);
    let Ok(BillWiseReadPlan::Parts(parts)) = plan_reads(
        catalogue.identified_parents(),
        live_limits_of_one_part_of_eleven(),
        &named,
    ) else {
        panic!("a book above one part is read by parts");
    };
    assert_eq!(parts.len(), 1, "both parents fit one part");
    let request = super::super::ledger_bill_wise_read(LIVE_COMPANY, &live_period(), &parts[0]);
    assert_eq!(
        wire_sha256(request.as_str()),
        recorded_request_sha256(LIVE_PARTS_PROVENANCE),
        "the part's request is the one that was sent"
    );
    let rows = live_rows(LIVE_PARTS);
    assert_eq!(rows.len(), 11);
    let observed = ObservedBillWise::new(
        named.iter().copied(),
        &[BillWiseRead {
            scope: BillWiseScope::Part(&parts[0]),
            rows: &rows,
        }],
    )
    .expect("the part's answer is exactly its ledgers");
    let flags = observed.flags_of(&named);
    assert_eq!(
        (flags[LIVE_PARTY], flags[LIVE_CREDITOR]),
        (Some(true), Some(true))
    );
    // A short answer (the first ten of the part's eleven rows) is not the part's answer either.
    assert_eq!(
        ObservedBillWise::new(
            named.iter().copied(),
            &[BillWiseRead {
                scope: BillWiseScope::Part(&parts[0]),
                rows: &rows[..10],
            }],
        ),
        Err(BillWiseError::RowCountDiffers)
    );
    // The whole book's 17 rows are not this part's answer.
    assert_eq!(
        ObservedBillWise::new(
            named.iter().copied(),
            &[BillWiseRead {
                scope: BillWiseScope::Part(&parts[0]),
                rows: &live_rows(LIVE_WHOLE),
            }],
        ),
        Err(BillWiseError::RowCountDiffers)
    );
}
