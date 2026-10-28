//! The master post's queue recheck. The index and mark inputs to
//! `admit_master_create` are constructed values, not captures: it is a pure
//! decision over parsed indexes. The group answers and the ledger index answers
//! read through `check_name_free` are committed live captures.
use super::super::post::refusal_code;
use super::*;
use bridge_tally_core::master_name_index::Via;

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const NAME: &str = "Synthetic Masters Lab";

use crate::tally::ledger_create_item::{AdmittedLedgerKind, MasterName};
use std::sync::LazyLock;

fn item(name: &str) -> LedgerCreateItem {
    LedgerCreateItem::new(MasterName::new(name).unwrap(), AdmittedLedgerKind::Suspense)
}

static ITEM: LazyLock<LedgerCreateItem> = LazyLock::new(|| item("Brand New Party"));
static COMPANY: LazyLock<VerifiedCompanyIdentity> =
    LazyLock::new(|| VerifiedCompanyIdentity::test_fixture(NAME, GUID));
static OTHER_COMPANY: LazyLock<VerifiedCompanyIdentity> = LazyLock::new(|| {
    VerifiedCompanyIdentity::test_fixture("Some Other Company", crate::synthetic_guid(0x1111))
});

fn marks(masters: u64) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
         <COMPANY NAME=\"{NAME}\" RESERVEDNAME=\"\"><GUID>{GUID}</GUID><ALTVCHID>10</ALTVCHID><ALTMSTID>{masters}</ALTMSTID></COMPANY>\
         <COMPANY NAME=\"Synthetic Other Lab\" RESERVEDNAME=\"\"><GUID>{other}</GUID><ALTVCHID>5</ALTVCHID><ALTMSTID>3</ALTMSTID></COMPANY>\
         </COLLECTION></DATA></BODY></ENVELOPE>",
        other = crate::synthetic_guid(0x2222),
    )
}

fn master(kind: MasterKind, guid: &str, alter_id: u64, names: &[&str]) -> IndexedMaster {
    IndexedMaster {
        kind,
        guid: guid.to_string(),
        alter_id,
        names: names.iter().map(ToString::to_string).collect(),
    }
}

fn ledgers() -> Vec<IndexedMaster> {
    vec![
        master(MasterKind::Ledger, "L-1", 11, &["Cash", "Cash Alias"]),
        master(MasterKind::Ledger, "L-2", 12, &["Acme Traders"]),
    ]
}

fn groups() -> Vec<IndexedMaster> {
    vec![master(MasterKind::Group, "G-1", 3, &["Sundry Debtors"])]
}

fn admit(
    candidate: &str,
    at_binding: u64,
    at_aim: u64,
    ledgers: Vec<IndexedMaster>,
    groups: Vec<IndexedMaster>,
) -> anyhow::Result<BTreeMap<String, u64>> {
    admit_master_create(
        ledgers,
        groups,
        &item(candidate),
        &marks(at_binding),
        &marks(at_aim),
        &COMPANY,
    )
}

fn refusal(result: anyhow::Result<BTreeMap<String, u64>>) -> ApprovedImportAdmissionError {
    result
        .expect_err("the recheck must refuse")
        .downcast::<ApprovedImportAdmissionError>()
        .expect("the refusal is a typed admission error")
}

#[test]
fn a_name_that_is_a_ledgers_alias_is_taken_via_alias() {
    assert_eq!(
        refusal(admit("cash alias", 7, 7, ledgers(), groups())),
        ApprovedImportAdmissionError::MasterNameTaken {
            kind: MasterKind::Ledger,
            via: Via::Alias
        }
    );
}

#[test]
fn a_name_that_is_a_ledgers_stored_name_is_taken_via_name() {
    assert_eq!(
        refusal(admit("acme traders", 7, 7, ledgers(), groups())),
        ApprovedImportAdmissionError::MasterNameTaken {
            kind: MasterKind::Ledger,
            via: Via::Name
        }
    );
}

#[test]
fn a_name_that_is_a_groups_name_is_taken_as_a_group() {
    assert_eq!(
        refusal(admit("Sundry Debtors", 7, 7, ledgers(), groups())),
        ApprovedImportAdmissionError::MasterNameTaken {
            kind: MasterKind::Group,
            via: Via::Name
        }
    );
}

#[test]
fn a_free_name_is_admitted_and_returns_the_pre_image() {
    let pre_image = admit("Brand New Party", 7, 7, ledgers(), groups()).unwrap();
    assert_eq!(
        pre_image,
        BTreeMap::from([
            ("l-1".to_string(), 11),
            ("l-2".to_string(), 12),
            ("g-1".to_string(), 3),
        ])
    );
}

#[test]
fn a_master_mark_that_moved_between_binding_and_aim_is_refused() {
    assert_eq!(
        refusal(admit("Brand New Party", 7, 8, ledgers(), groups())),
        ApprovedImportAdmissionError::MastersMoved
    );
}

#[test]
fn a_collision_is_reported_before_a_moved_mark() {
    assert_eq!(
        refusal(admit("Cash", 7, 8, ledgers(), groups())),
        ApprovedImportAdmissionError::MasterNameTaken {
            kind: MasterKind::Ledger,
            via: Via::Name
        }
    );
}

#[test]
fn unreadable_marks_are_unconfirmed_not_admitted() {
    let result = admit_master_create(
        ledgers(),
        groups(),
        &ITEM,
        "<ENVELOPE/>",
        &marks(7),
        &COMPANY,
    );
    assert_eq!(
        refusal(result),
        ApprovedImportAdmissionError::MastersUnconfirmed
    );
}

#[test]
fn an_index_with_a_guid_on_a_ledger_and_a_group_is_unreadable() {
    let clash = vec![master(MasterKind::Group, "l-1", 3, &["Some Group"])];
    assert_eq!(
        refusal(admit("Brand New Party", 7, 7, ledgers(), clash)),
        ApprovedImportAdmissionError::CatalogueUnreadable(
            StandardLedgerCatalogError::DuplicateIdentity
        )
    );
}

fn group_capture() -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-groups.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The group rows of the live capture become group entries of the one index,
/// each with its stored name first, its aliases after, its GUID and ALTERID.
#[test]
fn group_rows_become_group_masters_with_their_names_guid_and_alter_id() {
    let rows = parse_group_name_index(&group_capture(), GUID).unwrap();
    assert!(!rows.is_empty());
    let masters = group_masters(rows.clone());
    assert_eq!(masters.len(), rows.len());
    for (master, row) in masters.iter().zip(&rows) {
        assert_eq!(master.kind, MasterKind::Group);
        assert_eq!(master.guid, row.guid);
        assert_eq!(master.alter_id, row.alter_id);
        assert_eq!(master.names, row.names);
    }
    // A name in the capture is taken, as a group, by the decision over them.
    let taken = masters
        .iter()
        .map(|master| master.names[0].as_str())
        .find(|name| MasterName::new(name).is_ok())
        .expect("the capture holds a group name Bridge could send");
    assert_eq!(
        refusal(admit_master_create(
            Vec::new(),
            masters.clone(),
            &item(taken),
            &marks(7),
            &marks(7),
            &COMPANY,
        )),
        ApprovedImportAdmissionError::MasterNameTaken {
            kind: MasterKind::Group,
            via: Via::Name
        }
    );
}

fn masters_queued<'a>(
    ledgers: (&'a str, &'a str),
    groups: (&'a str, &'a str),
    at_binding: &'a str,
    at_aim: &'a str,
) -> QueuedAdmission<'a> {
    masters_queued_for(&COMPANY, ledgers, groups, at_binding, at_aim)
}

fn masters_queued_for<'a>(
    company: &'a VerifiedCompanyIdentity,
    ledgers: (&'a str, &'a str),
    groups: (&'a str, &'a str),
    at_binding: &'a str,
    at_aim: &'a str,
) -> QueuedAdmission<'a> {
    QueuedAdmission::Masters {
        item: &ITEM,
        company,
        ledgers_first: ledgers.0,
        ledgers_second: ledgers.1,
        groups_first: groups.0,
        groups_second: groups.1,
        company_marks_at_binding: at_binding,
        company_marks: at_aim,
    }
}

fn recheck(queued: QueuedAdmission<'_>) -> ApprovedImportAdmissionError {
    refusal(recheck_master_admission(queued))
}

#[test]
fn an_unparsable_ledger_answer_refuses_beside_a_valid_group_answer() {
    let groups = group_capture();
    let marks = marks(7);
    assert_eq!(
        recheck(masters_queued(
            ("<ENVELOPE/>", "<ENVELOPE/>"),
            (&groups, &groups),
            &marks,
            &marks
        )),
        ApprovedImportAdmissionError::MasterLedgerIndexUnavailable {
            cause: "ledger_catalogue_malformed_response"
        }
    );
}

#[test]
fn the_ledger_parse_refuses_what_is_not_a_ledger_index() {
    assert_eq!(
        ledger_masters("<ENVELOPE/>", GUID),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
    // The group answer of the company `GUID` names: right company, wrong rows.
    assert_eq!(
        ledger_masters(&group_capture(), GUID),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
    let masters = ledger_masters(&masters_lab(MASTERS_LAB_LEDGERS), MASTERS_LAB_GUID).unwrap();
    assert_eq!(masters.len(), 8);
    assert!(masters
        .iter()
        .all(|master| master.kind == MasterKind::Ledger));
}

const MASTERS_LAB_GUID: &str = "f107e523-3d6b-4516-bd24-5b1a14498fa6";
const MASTERS_LAB_LEDGERS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/masters-lab-ledger-index-baseline.utf16le.xml"
);
const MASTERS_LAB_LEDGERS_AFTER: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/masters-lab-ledger-index-after-name-cases.utf16le.xml"
);
const MASTERS_LAB_LEDGERS_FOLDED: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/masters-lab-ledger-index-after-fold-cases.utf16le.xml"
);
const MASTERS_LAB_GROUPS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/masters-lab-groups-baseline.utf16le.xml"
);

fn masters_lab(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The name check over one book's captured ledger and group answers.
fn name_check(ledgers: &[u8], candidate: &str) -> Option<ApprovedImportAdmissionError> {
    check_name_free(
        &masters_lab(ledgers),
        &masters_lab(MASTERS_LAB_GROUPS),
        &item(candidate),
        MASTERS_LAB_GUID,
    )
    .err()
    .map(|error| {
        error
            .downcast::<ApprovedImportAdmissionError>()
            .expect("a typed admission error")
    })
}

/// Every Create name recorded in `masters-lab.PROVENANCE.md`, checked against
/// the captured book before those writes: each one Tally altered, refused or
/// (for a group's name) created beside the group is refused here, and only the
/// new name is admitted. The doubled-space name is not in the list: `MasterName`
/// refuses it before any check, as the last assertion shows.
#[test]
fn every_create_the_lab_sent_is_refused_against_the_captured_book_but_the_new_name() {
    let taken = |kind, via| Some(ApprovedImportAdmissionError::MasterNameTaken { kind, via });
    for (candidate, expected) in [
        ("MLab Exact Target", taken(MasterKind::Ledger, Via::Name)),
        ("MLab Alias Name", taken(MasterKind::Ledger, Via::Alias)),
        ("MLAB CASE TARGET", taken(MasterKind::Ledger, Via::Name)),
        ("MLab Dash Target", taken(MasterKind::Ledger, Via::Name)),
        ("MLab Slash Target", taken(MasterKind::Ledger, Via::Name)),
        ("MLab Group Target", taken(MasterKind::Group, Via::Name)),
        ("MLab Fresh Party", None),
    ] {
        assert_eq!(
            name_check(MASTERS_LAB_LEDGERS, candidate),
            expected,
            "{candidate}"
        );
    }
    assert!(MasterName::new("MLab  Space Target").is_err());
}

/// After those writes the book holds ledgers whose stored name changed: the
/// old spelling is still refused, and so is a spelling that differs only by a
/// removed space.
#[test]
fn an_old_spelling_and_a_removed_space_are_refused_after_the_writes() {
    assert_eq!(
        name_check(MASTERS_LAB_LEDGERS_AFTER, "MLab Slash/Target"),
        Some(ApprovedImportAdmissionError::MasterNameTaken {
            kind: MasterKind::Ledger,
            via: Via::Name,
        })
    );
    let row_spelling_kept =
        ledger_masters(&masters_lab(MASTERS_LAB_LEDGERS_AFTER), MASTERS_LAB_GUID)
            .unwrap()
            .into_iter()
            .find(|master| master.names[0] == "MLab Slash Target")
            .unwrap();
    assert_eq!(
        row_spelling_kept.names,
        ["MLab Slash Target", "MLab Slash/Target"]
    );
    assert_eq!(
        name_check(MASTERS_LAB_LEDGERS_AFTER, "MLab FreshParty"),
        Some(ApprovedImportAdmissionError::MasterNameTaken {
            kind: MasterKind::Ledger,
            via: Via::Name,
        })
    );
    assert_eq!(
        name_check(MASTERS_LAB_LEDGERS_AFTER, "MLab Probe Pvt Ltd"),
        None
    );
}

/// A removed space and removed full stops, against the captured book after
/// those Creates: both the old and the new spelling of each ledger are refused.
#[test]
fn a_removed_space_and_removed_full_stops_are_refused_against_the_folded_book() {
    for candidate in [
        "MLab Fresh Party",
        "MLab FreshParty",
        "MLab Probe Pvt. Ltd.",
        "MLab Probe Pvt Ltd",
        "MLab Probe PvtLtd",
    ] {
        assert!(
            matches!(
                name_check(MASTERS_LAB_LEDGERS_FOLDED, candidate),
                Some(ApprovedImportAdmissionError::MasterNameTaken {
                    kind: MasterKind::Ledger,
                    ..
                })
            ),
            "{candidate}"
        );
    }
}

#[tokio::test]
async fn the_readback_parse_refuses_alike_unless_a_seam_scopes_it() {
    let groups = group_capture();
    let unavailable = |result: anyhow::Result<(Vec<IndexedMaster>, Vec<IndexedMaster>)>| {
        refusal_code(&result.unwrap_err()).to_string()
    };
    // An answer that does not parse: the same typed refusal as the queue's,
    // so the read-back says it read nothing.
    assert_eq!(
        unavailable(parse_readback_indexes("<ENVELOPE/>", &groups, GUID)),
        "master_ledger_index_unavailable"
    );
    // The queue's seam serves the read-back when no later one is scoped.
    let before = vec![master(
        MasterKind::Ledger,
        crate::synthetic_guid(0xa001),
        1,
        &["Cash"],
    )];
    let (ledgers, grouped) = TEST_LEDGER_INDEX
        .scope(before.clone(), async {
            parse_readback_indexes("<ENVELOPE/>", &groups, GUID).unwrap()
        })
        .await;
    assert_eq!(ledgers, before);
    assert!(!grouped.is_empty());
    // The read-back's own seam wins, for the read-back alone.
    let after = vec![master(
        MasterKind::Ledger,
        crate::synthetic_guid(0xa002),
        2,
        &["Zeta"],
    )];
    let (ledgers, _) = TEST_LEDGER_INDEX
        .scope(
            before.clone(),
            TEST_LEDGER_INDEX_AFTER_POST.scope(after.clone(), async {
                parse_readback_indexes("<ENVELOPE/>", &groups, GUID).unwrap()
            }),
        )
        .await;
    assert_eq!(ledgers, after);
    let queue = TEST_LEDGER_INDEX
        .scope(
            before.clone(),
            TEST_LEDGER_INDEX_AFTER_POST.scope(after, async {
                ledger_masters("<ENVELOPE/>", GUID).unwrap()
            }),
        )
        .await;
    assert_eq!(queue, before, "the queue's parse never sees the later seam");
    // A group answer that does not parse is still the group refusal.
    let bad = unavailable(parse_readback_indexes("<ENVELOPE/>", "<ENVELOPE/>", GUID));
    assert_ne!(bad, "master_ledger_index_unavailable");
}

#[test]
fn ledger_reads_that_differ_are_an_unreadable_catalogue() {
    let groups = group_capture();
    let marks = marks(7);
    assert_eq!(
        recheck(masters_queued(
            ("<A/>", "<B/>"),
            (&groups, &groups),
            &marks,
            &marks
        )),
        ApprovedImportAdmissionError::CatalogueUnreadable(
            StandardLedgerCatalogError::MalformedResponse
        )
    );
}

#[test]
fn group_reads_that_differ_are_an_unreadable_catalogue() {
    let groups = group_capture();
    let changed = groups.replacen("Sundry", "Sundri", 1);
    assert_ne!(changed, groups);
    let marks = marks(7);
    assert_eq!(
        recheck(masters_queued(
            ("<ENVELOPE/>", "<ENVELOPE/>"),
            (&groups, &changed),
            &marks,
            &marks
        )),
        ApprovedImportAdmissionError::CatalogueUnreadable(
            StandardLedgerCatalogError::MalformedResponse
        )
    );
}

#[test]
fn an_unparsable_group_answer_is_a_group_export_refusal_before_the_ledger_index() {
    let marks = marks(7);
    let refused = recheck(masters_queued(
        ("<ENVELOPE/>", "<ENVELOPE/>"),
        ("<ENVELOPE/>", "<ENVELOPE/>"),
        &marks,
        &marks,
    ));
    assert!(
        matches!(
            refused,
            ApprovedImportAdmissionError::GroupExportInvalid { .. }
        ),
        "{refused:?}"
    );
}

#[test]
fn a_group_answer_for_another_company_is_refused() {
    let groups = group_capture();
    let marks = marks(7);
    let refused = refusal(recheck_master_admission(masters_queued_for(
        &OTHER_COMPANY,
        ("<ENVELOPE/>", "<ENVELOPE/>"),
        (&groups, &groups),
        &marks,
        &marks,
    )));
    assert!(
        matches!(
            refused,
            ApprovedImportAdmissionError::GroupExportInvalid { .. }
        ),
        "{refused:?}"
    );
}

// --- The real recheck, driven through the endpoint queue --------------------
//
// The simulator speaks the captured Company and Group answers; the ledger
// index answer is a stand-in body, since no V3 answer has been captured and the
// parser refuses every input.

mod queue {
    use super::*;
    use crate::tally::approved_import::{ApprovedImport, BeforeDispatchError, UnderLockRefusal};
    use crate::tally::ledger_create_item::{AdmittedLedgerKind, LedgerCreateItem, MasterName};
    use crate::tally::{TallyConfig, TallyRuntime, VerifiedCompanyIdentity};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use tally_protocol_simulator::{
        Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
    };

    const ITEM: &str = "Queue Test Suspense";

    fn decode(bytes: &[u8]) -> String {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }

    fn companies() -> String {
        decode(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
        ))
    }

    fn identity(xml: &str) -> VerifiedCompanyIdentity {
        let observed = bridge_tally_protocol::parse_companies_from_collection(xml).unwrap();
        let company = observed
            .iter()
            .find(|company| company.guid.as_deref() == Some(GUID))
            .unwrap();
        VerifiedCompanyIdentity::from_observed_companies(
            company.name.clone(),
            crate::tally::test_company_guid(GUID),
            company.company_number.clone().unwrap(),
            company.books_from.clone().unwrap(),
            &observed,
        )
        .unwrap()
    }

    fn company_plan(xml: String) -> ScenarioPlan {
        ScenarioPlan::new(Fixture::SyntheticXml(xml))
            .with_encoding(WireEncoding::Utf16Le)
            .with_framing(ResponseFraming::ContentLength)
    }

    fn status_plan() -> ScenarioPlan {
        ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
            .with_framing(ResponseFraming::ContentLength)
    }

    /// 32 queue requests: opening admission and binding marks (4), four paired
    /// index reads (6 each), the closing admission (3), the aim marks.
    fn plans(opening: String, ledgers: String, groups: String) -> Vec<ScenarioPlan> {
        let paired = |body: &String, companies: &String| {
            vec![
                company_plan(companies.clone()),
                company_plan(body.clone()),
                status_plan(),
                company_plan(body.clone()),
                status_plan(),
                company_plan(companies.clone()),
            ]
        };
        let mut plans = vec![
            status_plan(),
            company_plan(opening.clone()),
            company_plan(opening.clone()),
            company_plan(opening.clone()),
        ];
        for body in [&ledgers, &ledgers, &groups, &groups] {
            plans.extend(paired(body, &opening));
        }
        plans.extend([
            status_plan(),
            company_plan(opening.clone()),
            company_plan(opening.clone()),
            company_plan(opening),
        ]);
        plans
    }

    fn approved(companies: &str) -> ApprovedImport {
        let identity = identity(companies);
        let marks = crate::tally::agent_read_request::AgentReadRequest::parse(format!(
            "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC><STATICVARIABLES><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME=\"Bridge Agent Company High Water\" ISMODIFY=\"No\"><TYPE>Company</TYPE><FETCH>GUID,ALTVCHID,ALTMSTID</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
            identity.display_name()
        ))
        .unwrap();
        ApprovedImport::approved_masters_for_test(
            LedgerCreateItem::new(MasterName::new(ITEM).unwrap(), AdmittedLedgerKind::Suspense),
            &identity,
            marks,
        )
    }

    struct Run {
        error: anyhow::Error,
        observed: usize,
        intent_recorded: bool,
    }

    /// Runs one master post whose recheck is `recheck_master_admission` and
    /// whose intent closure refuses (`import_batch_not_found`), against `plans`
    /// cut to `requests`. The closure's refusal stands in for any refusal under
    /// the lock; the real intent is proved in the tool's own tests.
    async fn run(
        companies: String,
        mut plans: Vec<ScenarioPlan>,
        requests: usize,
        permissive_recheck: bool,
    ) -> Run {
        plans.truncate(requests);
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let recorded = Arc::new(AtomicBool::new(false));
        let flag = recorded.clone();
        let error = TallyRuntime::default()
            .post_approved_import(
                TallyConfig {
                    host: "127.0.0.1".into(),
                    port: simulator.address().port(),
                },
                &identity(&companies),
                approved(&companies),
                |queued: QueuedAdmission<'_>| {
                    if permissive_recheck {
                        Ok(())
                    } else {
                        recheck_master_admission(queued).map(|_| ())
                    }
                },
                move || {
                    // Reaching the intent at all is what the test watches for.
                    flag.store(true, Ordering::Release);
                    Err::<(), _>(BeforeDispatchError::Refused(
                        UnderLockRefusal::BatchNotFound,
                    ))
                },
            )
            .await
            .expect_err("a master post is refused in this slice");
        Run {
            error,
            observed: simulator.finish().unwrap().len(),
            intent_recorded: recorded.load(Ordering::Acquire),
        }
    }

    /// A ledger index answer that does not parse refuses a master post after
    /// every read and before the intent, whatever the books hold. The POST (a
    /// 33rd request) never arrives.
    #[tokio::test]
    async fn an_unreadable_ledger_index_refuses_a_master_post_before_intent_and_post() {
        let companies = companies();
        let groups = decode(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-groups.utf16le.xml"
        ));
        let run = run(
            companies.clone(),
            plans(companies, "<ENVELOPE/>".to_string(), groups),
            32,
            false,
        )
        .await;
        assert_eq!(refusal_code(&run.error), "master_ledger_index_unavailable");
        assert_eq!(run.observed, 32, "every queue read, and no POST");
        assert!(!run.intent_recorded, "refused before the intent");
    }

    /// Education is refused first: no index is read, so no index parser (or its
    /// absence) is reached.
    #[tokio::test]
    async fn an_education_book_is_refused_before_the_ledger_index_is_reached() {
        let companies = companies();
        let education = companies.replace(
            "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
            "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
        );
        assert_ne!(education, companies);
        let run = run(
            companies.clone(),
            plans(
                education,
                "<ENVELOPE/>".to_string(),
                "<ENVELOPE/>".to_string(),
            ),
            3,
            false,
        )
        .await;
        assert_eq!(refusal_code(&run.error), "master_writes_need_licensed_mode");
        assert_eq!(run.observed, 3);
        assert!(!run.intent_recorded);
    }

    /// With the admission passed, the intent closure is the last stop: a
    /// refusal there keeps its own code, and nothing is sent after the 32nd
    /// request.
    #[tokio::test]
    async fn a_refusing_intent_closure_is_the_last_stop_and_nothing_is_sent() {
        let companies = companies();
        let run = run(
            companies.clone(),
            plans(
                companies,
                "<ENVELOPE/>".to_string(),
                "<ENVELOPE/>".to_string(),
            ),
            32,
            true,
        )
        .await;
        assert_eq!(refusal_code(&run.error), "import_batch_not_found");
        assert_eq!(run.observed, 32, "no POST follows the refused intent");
        assert!(run.intent_recorded, "the intent closure was reached");
    }
}

/// A renamed reserved ledger's built-in name, carried in `RESERVEDNAME`, is
/// one more name the check refuses (one attribute changed in the capture).
#[test]
fn a_reserved_ledgers_built_in_name_is_refused() {
    let renamed = masters_lab(MASTERS_LAB_LEDGERS).replacen(
        "<LEDGER NAME=\"MLab Alias Target\" RESERVEDNAME=\"\">",
        "<LEDGER NAME=\"MLab Alias Target\" RESERVEDNAME=\"Synthetic Reserved\">",
        1,
    );
    let masters = ledger_masters(&renamed, MASTERS_LAB_GUID).unwrap();
    let alias = masters
        .iter()
        .find(|master| master.names[0] == "MLab Alias Target")
        .unwrap();
    assert_eq!(
        alias.names.last().map(String::as_str),
        Some("Synthetic Reserved")
    );
}
