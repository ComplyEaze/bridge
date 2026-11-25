//! The master post's queue recheck. The index and mark inputs to
//! `admit_master_create` are constructed values, not captures: it is a pure
//! decision over parsed indexes. The group answers and the ledger index answers
//! read through `check_name_free` are committed live captures.
use super::super::post::refusal_code;
use super::*;
use bridge_tally_core::master_name_index::Via;
use bridge_tally_protocol::{decode_tally_xml_response_bytes_limited, ExpectedTallyTextEncoding};

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const NAME: &str = "Synthetic Masters Lab";

use crate::tally::ledger_create_item::{AdmittedLedgerKind, LedgerCreateBatch};
use crate::tally::master_name::MasterName;
use std::sync::LazyLock;

/// A batch of one Suspense ledger named `name`.
fn item(name: &str) -> LedgerCreateBatch {
    LedgerCreateBatch::one(LedgerCreateItem::new(
        MasterName::new(name).unwrap(),
        AdmittedLedgerKind::Suspense,
    ))
}

static ITEM: LazyLock<LedgerCreateBatch> = LazyLock::new(|| item("Brand New Party"));
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
        batch: &ITEM,
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
    use crate::tally::ledger_create_item::{AdmittedLedgerKind, LedgerCreateItem};
    use crate::tally::master_name::MasterName;
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

const ALIAS_LAB_GUID: &str = "021bda0c-b1b1-44c8-8279-5212ce87c9c7";
/// Tally's own name index of a book whose ledgers carry aliases keyed on its screen and by import
/// (`alias-lab-1688/PROVENANCE.md` beside the fixture).
const ALIAS_LAB_LEDGERS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/alias-lab-1688/i1-idx.utf16le.xml"
);

/// On the captured index, a new ledger named as any ledger's alias is taken via that alias, one named
/// as a ledger's own name is taken via its name, and a new name is admitted. The non-ASCII aliases are not
/// tried here: `MasterName` refuses a name outside its alphabet before any check.
#[test]
fn a_name_that_is_a_captured_alias_is_taken_via_alias() {
    let ledgers = ledger_masters(&masters_lab(ALIAS_LAB_LEDGERS), ALIAS_LAB_GUID)
        .expect("the captured name index parses");
    for (candidate, via) in [
        ("Lab Plain Alias", Some(Via::Alias)),
        ("Lab Other Second", Some(Via::Alias)),
        ("Lab Shape Alias", Some(Via::Alias)),
        ("Lab Plain Ledger", Some(Via::Name)),
        ("Lab New Party", None),
    ] {
        let taken = admit_free_names(ledgers.clone(), groups(), &item(candidate))
            .err()
            .map(|error| {
                error
                    .downcast::<ApprovedImportAdmissionError>()
                    .expect("a typed admission error")
            });
        assert_eq!(
            taken,
            via.map(|via| ApprovedImportAdmissionError::MasterNameTaken {
                kind: MasterKind::Ledger,
                via,
            }),
            "{candidate}"
        );
    }
}

const STOCK_BEFORE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-before.utf16le.xml"
);
const STOCK_AFTER_MARKED: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-after-marked-create.utf16le.xml"
);
const STOCK_ALIAS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-with-alias.utf16le.xml"
);
const STOCK_GROUPS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-groups.utf16le.xml"
);
const STOCK_UNITS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-units.utf16le.xml"
);
const STOCK_MARKS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-marks.utf16le.xml"
);
const STOCK_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1";
const STOCK_SOURCE: &str = "SC Sibling Goods";
const STOCK_PROPOSED: &str = "SC Copied Marked";

fn stock_capture(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("the stock capture decodes through the limited decoder")
    .text
}

fn stock_identity_from_capture() -> VerifiedCompanyIdentity {
    let marks = stock_capture(STOCK_MARKS);
    let rows = parse_all_company_marks(&marks).expect("the captured marks parse");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].masters, 210);
    VerifiedCompanyIdentity::test_fixture(rows[0].name.clone(), rows[0].guid.clone())
}

fn stock_test_draft<'a>(
    catalogue: &'a crate::tally::stock_item_create::StockItemCatalogue,
    proposed: &str,
    source: &str,
    groups: &str,
    units: &str,
) -> StockItemDraft<'a> {
    let name = catalogue
        .name_absent(&MasterName::new(proposed).unwrap())
        .expect("the shown name is absent from its before read");
    let sibling = SiblingGst::named(catalogue, &MasterName::new(source).unwrap())
        .expect("the captured sibling is complete");
    let sibling = GoodsSibling::from_sibling(sibling).expect("the captured sibling is Goods");
    StockItemDraft::new(name, sibling, groups, units).expect("the measured draft shape admits")
}

struct StockRecheckCase {
    shown_candidate: String,
    fresh_candidate: String,
    proposed: String,
    source: String,
    groups: String,
    units: String,
    binding: String,
    aim: String,
    candidate_mark: u64,
    groups_mark: u64,
    units_mark: u64,
    shown_identity: VerifiedCompanyIdentity,
    fresh_identity: VerifiedCompanyIdentity,
    groups_identity: Option<VerifiedCompanyIdentity>,
    units_identity: Option<VerifiedCompanyIdentity>,
}

fn stock_recheck_case() -> StockRecheckCase {
    let identity = stock_identity_from_capture();
    StockRecheckCase {
        shown_candidate: stock_capture(STOCK_BEFORE),
        fresh_candidate: stock_capture(STOCK_BEFORE),
        proposed: STOCK_PROPOSED.to_string(),
        source: STOCK_SOURCE.to_string(),
        groups: stock_capture(STOCK_GROUPS),
        units: stock_capture(STOCK_UNITS),
        binding: stock_capture(STOCK_MARKS),
        aim: stock_capture(STOCK_MARKS),
        candidate_mark: 210,
        groups_mark: 210,
        units_mark: 210,
        shown_identity: identity.clone(),
        fresh_identity: identity,
        groups_identity: None,
        units_identity: None,
    }
}

fn run_stock_recheck(case: StockRecheckCase) -> anyhow::Result<()> {
    let shown_catalogue = crate::tally::stock_item_create::StockItemCatalogue::from_candidate_read(
        &case.shown_identity,
        &case.shown_candidate,
    )
    .expect("the shown candidate catalogues");
    let shown = ShownDraft::from_draft(stock_test_draft(
        &shown_catalogue,
        &case.proposed,
        &case.source,
        &case.groups,
        &case.units,
    ));
    let fresh_catalogue = crate::tally::stock_item_create::StockItemCatalogue::from_candidate_read(
        &case.fresh_identity,
        &case.fresh_candidate,
    )
    .expect("the fresh candidate catalogues");
    let candidate =
        CompleteStockItemCatalogue::from_catalogue_for_test(fresh_catalogue, case.candidate_mark);
    let groups = AdmittedStockGroups::from_capture_for_test(
        case.groups_identity
            .as_ref()
            .unwrap_or(&case.fresh_identity),
        &case.groups,
        case.groups_mark,
    )
    .expect("the captured groups parse for the fresh identity");
    let units = AdmittedStockUnits::from_capture_for_test(
        case.units_identity.as_ref().unwrap_or(&case.fresh_identity),
        &case.units,
        case.units_mark,
    )
    .expect("the captured units parse for the fresh identity");
    recheck_stock_item_admission(StockItemRecheckInput {
        shown: &shown,
        candidate: &candidate,
        groups: &groups,
        units: &units,
        company_marks_at_binding: &case.binding,
        company_marks: &case.aim,
    })
}

fn assert_stock_recheck_refusal(
    result: anyhow::Result<()>,
    expected: impl FnOnce(&StockItemRecheckError) -> bool,
) {
    let error = result.expect_err("the stock recheck refuses");
    let typed = error
        .downcast_ref::<StockItemRecheckError>()
        .expect("the stock recheck keeps its typed error");
    assert!(expected(typed), "{typed:?}");
}

#[test]
fn stock_recheck_admits_the_same_shown_draft_and_unchanged_marks() {
    let identity = stock_identity_from_capture();
    assert_eq!(identity.display_name(), "BRIDGE STOCK CREATE LAB");
    assert_eq!(identity.company_guid(), STOCK_GUID);
    let catalogue = crate::tally::stock_item_create::StockItemCatalogue::from_candidate_read(
        &identity,
        &stock_capture(STOCK_BEFORE),
    )
    .unwrap();
    let sibling = SiblingGst::named(&catalogue, &MasterName::new(STOCK_SOURCE).unwrap()).unwrap();
    assert!(sibling.guid().ends_with("-000000d0"));
    run_stock_recheck(stock_recheck_case()).expect("the unchanged R1 recheck admits");
}

#[test]
fn stock_recheck_refuses_a_name_taken_since_approval() {
    let mut case = stock_recheck_case();
    // Unchanged marks isolate name admission; a real Create moves the marks.
    case.fresh_candidate = stock_capture(STOCK_AFTER_MARKED);
    assert_stock_recheck_refusal(run_stock_recheck(case), |error| {
        matches!(
            error,
            StockItemRecheckError::NameNotAbsent(ItemNameNotAbsent::Taken(_))
        )
    });
}

#[test]
fn stock_recheck_does_not_treat_a_stock_group_or_unit_name_as_an_item_collision() {
    for proposed in ["SC Goods Group", "Nos"] {
        let mut case = stock_recheck_case();
        case.proposed = proposed.to_string();
        run_stock_recheck(case).unwrap_or_else(|error| {
            panic!("{proposed} is absent from the stock-item read: {error:?}")
        });
    }
}

#[test]
fn stock_recheck_refuses_a_changed_written_field() {
    let original = stock_capture(STOCK_BEFORE);
    let gst = original.replacen(
        "<GSTRATE TYPE=\"Number\"> 9</GSTRATE>",
        "<GSTRATE TYPE=\"Number\"> 10</GSTRATE>",
        1,
    );
    assert_ne!(gst, original);
    let hsn = original.replacen(
        "<HSNCODE TYPE=\"String\">39239090</HSNCODE>",
        "<HSNCODE TYPE=\"String\">39239099</HSNCODE>",
        1,
    );
    assert_ne!(hsn, original);
    // Both edits are constructed fault vectors, not measured Tally changes.
    for fresh_candidate in [gst, hsn] {
        let mut case = stock_recheck_case();
        case.fresh_candidate = fresh_candidate;
        assert_stock_recheck_refusal(run_stock_recheck(case), |error| {
            matches!(error, StockItemRecheckError::DraftChanged)
        });
    }
}

#[test]
fn stock_recheck_refuses_a_different_source_item_or_shown_draft() {
    let original = stock_capture(STOCK_BEFORE);
    let other_source = original.replacen("000000d0", "000000d9", 1);
    assert_ne!(other_source, original);
    let mut fresh_source = stock_recheck_case();
    fresh_source.fresh_candidate = other_source.clone();
    assert_stock_recheck_refusal(run_stock_recheck(fresh_source), |error| {
        matches!(error, StockItemRecheckError::DraftChanged)
    });

    let mut changed_shown = stock_recheck_case();
    changed_shown.shown_candidate = other_source;
    assert_stock_recheck_refusal(run_stock_recheck(changed_shown), |error| {
        matches!(error, StockItemRecheckError::DraftChanged)
    });
}

#[test]
fn stock_recheck_refuses_another_company_even_with_a_matching_guid() {
    let other = VerifiedCompanyIdentity::test_fixture("BRIDGE STOCK CREATE LAB OTHER", STOCK_GUID);
    let own = stock_identity_from_capture();
    // All three reads, then each read alone: the candidate (the support reads
    // keep the shown identity), the stock groups, the units.
    let mut all = stock_recheck_case();
    all.fresh_identity = other.clone();
    let mut candidate = stock_recheck_case();
    candidate.fresh_identity = other.clone();
    candidate.groups_identity = Some(own.clone());
    candidate.units_identity = Some(own);
    let mut groups = stock_recheck_case();
    groups.groups_identity = Some(other.clone());
    let mut units = stock_recheck_case();
    units.units_identity = Some(other);
    for case in [all, candidate, groups, units] {
        assert_stock_recheck_refusal(run_stock_recheck(case), |error| {
            matches!(error, StockItemRecheckError::OtherCompany)
        });
    }
}

#[test]
fn stock_recheck_reuses_the_typed_aim_refusals() {
    let moved = stock_capture(STOCK_MARKS).replacen(
        "<ALTMSTID TYPE=\"Number\"> 210</ALTMSTID>",
        "<ALTMSTID TYPE=\"Number\"> 211</ALTMSTID>",
        1,
    );
    let mut moved_case = stock_recheck_case();
    moved_case.aim = moved;
    let error = run_stock_recheck(moved_case).expect_err("a moved aim refuses");
    assert!(matches!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::MastersMoved)
    ));

    let mut malformed_aim = stock_recheck_case();
    malformed_aim.aim = "<ENVELOPE/>".to_string();
    let error = run_stock_recheck(malformed_aim).expect_err("malformed aim marks refuse");
    assert!(matches!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::CompanyScopeUnconfirmed)
    ));

    let mut malformed_binding = stock_recheck_case();
    malformed_binding.binding = "<ENVELOPE/>".to_string();
    let error = run_stock_recheck(malformed_binding).expect_err("malformed binding marks refuse");
    assert!(matches!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::MastersUnconfirmed)
    ));

    let mut without_target = stock_recheck_case();
    without_target.aim = stock_capture(STOCK_MARKS).replacen(STOCK_GUID, GUID, 1);
    let error = run_stock_recheck(without_target).expect_err("an aim without the target refuses");
    assert!(matches!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::CompanyScopeChanged)
    ));
}

#[test]
fn stock_read_and_recheck_have_no_shipped_caller_or_registered_tool() {
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn scan(
        directory: &Path,
        root: &Path,
        owners: &BTreeSet<PathBuf>,
        visited: &mut usize,
        problems: &mut Vec<String>,
    ) {
        for entry in fs::read_dir(directory).expect("the source directory is readable") {
            let path = entry.expect("a source directory entry").path();
            if path.is_dir() {
                scan(&path, root, owners, visited, problems);
                continue;
            }
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if !name.ends_with(".rs")
                || name.ends_with("_tests.rs")
                || name.contains("_tests_")
                || name == "tests.rs"
            {
                continue;
            }
            let relative = path.strip_prefix(root).unwrap().to_path_buf();
            *visited += 1;
            if owners.contains(&relative) {
                continue;
            }
            let source = fs::read_to_string(&path).expect("a production source is readable");
            for identifier in [
                "admit_candidate_stock_item_read",
                "admit_stock_groups",
                "admit_stock_units",
                "StockItemReadSize",
                "recheck_stock_item_admission",
                "CompleteStockItemCatalogue",
                "new_admitted",
            ] {
                if source.contains(identifier) {
                    problems.push(format!("{} names {identifier}", relative.display()));
                }
            }
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let owners = [
        "tally/runtime_masters.rs",
        "tally/runtime.rs",
        "tally/stock_item_create.rs",
        "agent_master_post.rs",
    ]
    .into_iter()
    .map(PathBuf::from)
    .collect::<BTreeSet<_>>();
    let mut visited = 0;
    let mut problems = Vec::new();
    scan(&root, &root, &owners, &mut visited, &mut problems);
    assert!(visited >= 100, "the scan read only {visited} source files");
    // This lexical guard is not macro-proof and does not replace the old seam gate.
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn stock_recheck_refuses_a_read_sized_at_another_mark() {
    for (expected, field) in [
        (StockItemReadKind::Candidate, 0),
        (StockItemReadKind::StockGroups, 1),
        (StockItemReadKind::Units, 2),
    ] {
        let mut case = stock_recheck_case();
        match field {
            0 => case.candidate_mark = 209,
            1 => case.groups_mark = 209,
            2 => case.units_mark = 209,
            _ => unreachable!(),
        }
        assert_stock_recheck_refusal(run_stock_recheck(case), |error| {
            matches!(
                error,
                StockItemRecheckError::SizedAtAnotherMark {
                    read,
                    sized: 209,
                    binding: 210,
                } if *read == expected
            )
        });
    }
}

#[test]
fn same_create_compares_every_written_field() {
    let original = stock_capture(STOCK_BEFORE);
    let groups = stock_capture(STOCK_GROUPS);
    let units = stock_capture(STOCK_UNITS);
    let identity = stock_identity_from_capture();
    let same = |second_candidate: String,
                second_proposed: &str,
                second_source: &str,
                second_groups: String,
                second_units: String| {
        let first_catalogue =
            crate::tally::stock_item_create::StockItemCatalogue::from_candidate_read(
                &identity, &original,
            )
            .unwrap();
        let second_catalogue =
            crate::tally::stock_item_create::StockItemCatalogue::from_candidate_read(
                &identity,
                &second_candidate,
            )
            .unwrap();
        let first = stock_test_draft(
            &first_catalogue,
            "SC R10 Fresh",
            STOCK_SOURCE,
            &groups,
            &units,
        );
        let second = stock_test_draft(
            &second_catalogue,
            second_proposed,
            second_source,
            &second_groups,
            &second_units,
        );
        first.same_create(&second)
    };
    assert!(same(
        original.clone(),
        "SC R10 Fresh",
        STOCK_SOURCE,
        groups.clone(),
        units.clone()
    ));

    let changed_parent = original.replace("SC Goods Group", "SC Other Group");
    let changed_unit = original.replace("Nos", "Each");
    let changed_guid = original.replacen("000000d0", "000000d9", 1);
    let changed_rate = original.replacen(
        "<GSTRATE TYPE=\"Number\"> 9</GSTRATE>",
        "<GSTRATE TYPE=\"Number\"> 10</GSTRATE>",
        1,
    );
    let changed_hsn = original.replacen("39239090", "39239099", 1);
    // Each edit is a separate in-memory fault vector, not a captured Tally state.
    for (label, candidate, proposed, source, changed_groups, changed_units) in [
        (
            "parent",
            changed_parent,
            "SC R10 Fresh",
            STOCK_SOURCE,
            groups.replace("SC Goods Group", "SC Other Group"),
            units.clone(),
        ),
        (
            "unit",
            changed_unit,
            "SC R10 Fresh",
            STOCK_SOURCE,
            groups.clone(),
            units.replace("Nos", "Each"),
        ),
        (
            "proposed name",
            original.clone(),
            "SC R10 Other",
            STOCK_SOURCE,
            groups.clone(),
            units.clone(),
        ),
        (
            "source name",
            stock_capture(STOCK_ALIAS),
            "SC R10 Fresh",
            "SC Sibling Alias",
            groups.clone(),
            units.clone(),
        ),
        (
            "source GUID",
            changed_guid,
            "SC R10 Fresh",
            STOCK_SOURCE,
            groups.clone(),
            units.clone(),
        ),
        (
            "GST rate",
            changed_rate,
            "SC R10 Fresh",
            STOCK_SOURCE,
            groups.clone(),
            units.clone(),
        ),
        (
            "HSN code",
            changed_hsn,
            "SC R10 Fresh",
            STOCK_SOURCE,
            groups.clone(),
            units.clone(),
        ),
    ] {
        assert!(
            !same(candidate, proposed, source, changed_groups, changed_units),
            "{label} differs"
        );
    }
}

#[test]
fn stock_recheck_takes_the_binding_mark_from_the_named_target_not_a_same_guid_sibling() {
    // A constructed snapshot, not a capture: a second loaded company sharing the
    // target's GUID under another name (as a year split can), listed first, at
    // another master mark. Reads sized at that sibling's mark must refuse.
    let sibling = "<COMPANY NAME=\"BRIDGE STOCK CREATE LAB SPLIT\" RESERVEDNAME=\"\">\
                   <GUID TYPE=\"String\">bac2b5e9-ada9-43c0-b8f8-a2812a813fa1</GUID>\
                   <ALTMSTID TYPE=\"Number\"> 205</ALTMSTID></COMPANY>";
    let with_sibling =
        stock_capture(STOCK_MARKS).replacen("<COLLECTION>", &format!("<COLLECTION>{sibling}"), 1);
    assert_ne!(with_sibling, stock_capture(STOCK_MARKS));
    let mut case = stock_recheck_case();
    case.binding = with_sibling.clone();
    case.aim = with_sibling;
    case.candidate_mark = 205;
    case.groups_mark = 205;
    case.units_mark = 205;
    assert_stock_recheck_refusal(run_stock_recheck(case), |error| {
        matches!(
            error,
            StockItemRecheckError::SizedAtAnotherMark {
                sized: 205,
                binding: 210,
                ..
            }
        )
    });
}
