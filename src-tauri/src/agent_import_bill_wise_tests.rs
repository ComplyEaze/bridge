use super::super::{ImportEntry, ImportVoucher};
use super::*;
use bridge_tally_protocol::StandardLedgerCatalogV2;

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

const BOOK: &str = "Synthetic Company";
const BOOK_GUID: &str = "11111111-2222-4333-8444-555555555555";

/// A V2 catalogue of `all`, the names in `bill_wise` marked Yes and the rest No.
///
/// A SYNTHETIC body (never evidence of what Tally answers; the live capture is
/// used below). It exercises this module's decisions on a typed catalogue.
fn catalogue_v2(all: &[&str], bill_wise: &[&str]) -> StandardLedgerCatalogV2 {
    let rows = all
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let flag = if bill_wise.contains(name) { "Yes" } else { "No" };
            format!(
                "<LEDGER NAME=\"{name}\" RESERVEDNAME=\"\"><GUID TYPE=\"String\">{BOOK_GUID}-{index:04}</GUID>\
                 <PARENT TYPE=\"String\">Synthetic Group</PARENT><ISBILLWISEON TYPE=\"Logical\">{flag}</ISBILLWISEON>\
                 <BRIDGECOMPANYGUID TYPE=\"String\">{BOOK_GUID}</BRIDGECOMPANYGUID>\
                 <BRIDGECOMPANYNAME TYPE=\"String\">{BOOK}</BRIDGECOMPANYNAME></LEDGER>"
            )
        })
        .collect::<String>();
    let xml = format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO>\
         <COMPANY>0</COMPANY></CMPINFO></DESC><DATA><COLLECTION>{rows}</COLLECTION></DATA></BODY></ENVELOPE>"
    );
    bridge_tally_protocol::parse_standard_ledger_catalog_v2_with_identities(&xml, BOOK, BOOK_GUID)
        .unwrap()
}

/// The names `bill_wise` marks Yes, every other name in `all` marked No.
fn observed(all: &[&str], bill_wise: &[&str]) -> ObservedBillWise {
    ObservedBillWise::from_catalogue(&catalogue_v2(all, bill_wise), all.iter().copied()).unwrap()
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
fn a_requested_ledger_the_catalogue_lacks_fails_closed() {
    let catalogue = catalogue_v2(&["Party A"], &[]);
    assert_eq!(
        ObservedBillWise::from_catalogue(&catalogue, ["Party A", "Party B"]),
        Err(BillWiseError::LedgerNotInCatalogue)
    );
}

#[test]
fn only_the_requested_ledgers_are_kept_and_the_rest_count_as_bill_wise() {
    let catalogue = catalogue_v2(&["Party A", "Party B", "Bank"], &["Party B"]);
    let observed = ObservedBillWise::from_catalogue(&catalogue, ["Party A", "Bank"]).unwrap();
    assert_eq!(observed.flags.len(), 2);
    assert!(!observed.is_bill_wise("Party A") && !observed.is_bill_wise("Bank"));
    // Party B was not requested: it was not read into the observation, so it is
    // answered in the refusing direction even though the catalogue says No for
    // Bank and Yes for Party B only.
    assert!(observed.is_bill_wise("Party B"));
}

fn approval(ledger: &str) -> OnAccountApproved {
    OnAccountApproved {
        ledger: ledger.into(),
        party_digest: "0".repeat(64),
    }
}

#[test]
fn a_ledger_not_bill_wise_at_the_build_must_still_not_be_bill_wise() {
    let named = names(&["Party A", "Bank"]);
    let all = ["Party A", "Party B", "Bank"];
    // Nothing changed.
    assert!(flags_still_as_approved(
        &catalogue_v2(&all, &[]),
        &named,
        &[]
    ));
    // A named, unapproved ledger switched to bill-wise: refused.
    assert!(!flags_still_as_approved(
        &catalogue_v2(&all, &["Party A"]),
        &named,
        &[]
    ));
    // A ledger the batch does not name switching is not the batch's concern.
    assert!(flags_still_as_approved(
        &catalogue_v2(&all, &["Party B"]),
        &named,
        &[]
    ));
    // An approval for one ledger does not cover another.
    assert!(!flags_still_as_approved(
        &catalogue_v2(&all, &["Party A", "Bank"]),
        &named,
        &[approval("Party A")]
    ));
}

#[test]
fn an_approved_ledger_passes_whether_it_stays_bill_wise_or_is_switched_off() {
    let named = names(&["Party A", "Bank"]);
    let all = ["Party A", "Bank"];
    let approved = [approval("Party A")];
    assert!(flags_still_as_approved(
        &catalogue_v2(&all, &["Party A"]),
        &named,
        &approved
    ));
    assert!(flags_still_as_approved(
        &catalogue_v2(&all, &[]),
        &named,
        &approved
    ));
}

#[test]
fn a_named_ledger_the_catalogue_no_longer_holds_fails_the_recheck() {
    let named = names(&["Party A", "Gone"]);
    assert!(!flags_still_as_approved(
        &catalogue_v2(&["Party A"], &[]),
        &named,
        &[]
    ));
}

fn names<'a>(names: &[&'a str]) -> BTreeSet<&'a str> {
    names.iter().copied().collect()
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
fn the_refusal_has_a_plain_sentence_that_names_no_code() {
    let sentence = BillWiseError::LedgerNotInCatalogue.plain();
    assert!(sentence.ends_with('.') && sentence.len() > 40, "{sentence}");
    // A person is told this sentence, so no snake_case code is in it.
    assert!(!sentence.contains('_'), "{sentence}");
    assert_eq!(
        BillWiseError::LedgerNotInCatalogue.cause(),
        "ledger_not_in_catalogue"
    );
    assert_eq!(
        BillWiseError::LedgerNotInCatalogue.reason(),
        "bill_wise_not_established"
    );
}

// ---- On a capture of a live Tally ---------------------------------------------------------------------------
//
// The answer below was read by Bridge's own V2 catalogue request (the profile this module reads through) from the
// synthetic company BRIDGE OUTSTANDINGS LAB (TallyPrime Silver 7.1) on 6 Oct 2026; its provenance file gives the
// request and response hashes. The expected flags are the 1 Oct snapshot of the same book (a different request,
// committed as `native-outstandings-detail-ledgers`), not this code's output.

const LIVE_COMPANY: &str = "BRIDGE OUTSTANDINGS LAB";
const LIVE_GUID: &str = "49f1fbda-ee59-4a4b-aacf-b45fe32402d7";
const LIVE_CATALOGUE_V2: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-ledger-catalogue-v2.utf16le.xml"
);
const LIVE_SNAPSHOT: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-ledgers.utf16le.xml"
);
const LIVE_PROVENANCE: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-ledger-catalogue-v2.json"
);
const LIVE_PARTY: &str = "OL P01 Named Bills Debtor";
const LIVE_BANK: &str = "OL Bank";

fn live_text(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .expect("captured UTF-16LE response")
}

fn live_catalogue() -> StandardLedgerCatalogV2 {
    bridge_tally_protocol::parse_standard_ledger_catalog_v2_with_identities(
        &live_text(LIVE_CATALOGUE_V2),
        LIVE_COMPANY,
        LIVE_GUID,
    )
    .expect("the captured V2 catalogue parses")
}

fn wire_sha256(xml: &str) -> String {
    super::super::sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        xml,
    ))
}

#[test]
fn live_the_request_the_build_sends_is_the_one_that_was_sent() {
    let provenance: Value = serde_json::from_str(LIVE_PROVENANCE).unwrap();
    let request =
        crate::tally::standard_ledger_catalog::render_import_ledger_catalog_request(LIVE_COMPANY)
            .unwrap();
    assert_eq!(
        provenance["source_request_sha256"].as_str().unwrap(),
        wire_sha256(&request)
    );
    assert_eq!(
        provenance["source_response_sha256"].as_str().unwrap(),
        super::super::sha256_hex(LIVE_CATALOGUE_V2)
    );
}

#[test]
fn live_every_ledger_of_the_book_carries_a_flag_and_the_flags_are_the_snapshots() {
    let catalogue = live_catalogue();
    let flags = catalogue
        .bill_wise_flags()
        .map(|(name, _, flag)| (name.to_string(), flag))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(flags.len(), 17);
    // The same book's outstandings snapshot (1 Oct): a different request, the
    // same flags by name.
    let snapshot = live_text(LIVE_SNAPSHOT);
    for (name, flag) in &flags {
        let marker = format!("<LEDGER NAME=\"{}\"", name.replace('&', "&amp;"));
        let at = snapshot
            .find(&marker)
            .expect("a catalogue ledger is in the snapshot");
        let row = &snapshot[at..at + snapshot[at..].find("</LEDGER>").unwrap()];
        let expected = if row.contains("<ISBILLWISEON TYPE=\"Logical\">Yes<") {
            bridge_tally_protocol::BillWiseFlag::On
        } else {
            bridge_tally_protocol::BillWiseFlag::Off
        };
        assert_eq!(*flag, expected, "{name}");
    }
    assert_eq!(
        flags
            .values()
            .filter(|flag| **flag == bridge_tally_protocol::BillWiseFlag::On)
            .count(),
        10
    );
    // Ledgers where Tally offers no bill-wise tracking answer No, not nothing.
    assert_eq!(flags[LIVE_BANK], bridge_tally_protocol::BillWiseFlag::Off);
    assert_eq!(flags["Cash"], bridge_tally_protocol::BillWiseFlag::Off);
}

#[test]
fn live_a_batch_naming_a_bill_wise_and_a_plain_ledger_has_one_party() {
    let payload = payload(vec![voucher(
        "txn-1",
        VoucherType::Payment,
        vec![
            entry(LIVE_PARTY, EntrySide::Dr, "10.00"),
            entry(LIVE_BANK, EntrySide::Cr, "10.00"),
        ],
    )]);
    let observed =
        ObservedBillWise::from_catalogue(&live_catalogue(), named_ledgers(&payload)).unwrap();
    let parties = bill_wise_parties(&payload.vouchers, &observed);
    assert_eq!(parties.len(), 1);
    assert_eq!(parties[0].ledger, LIVE_PARTY);
}
