//! Tests the import outcome of a stock item Create, the read-back verdict and
//! `VerifiedItem`. The captures are row 1381-R1's; see
//! `bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/PROVENANCE.md`.
//! The loader below is a copy of the smallest part of `stock_item_create_tests.rs`,
//! whose helpers are private to that module.

use super::*;
use crate::tally::stock_item_create::{StockItemCatalogueError, StockItemDraft};
use crate::tally::VerifiedCompanyIdentity;
use bridge_tally_protocol::native_masters::NativeMastersError;
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_import_outcome, ExpectedTallyTextEncoding,
};

const COMPANY: &str = "BRIDGE STOCK CREATE LAB";
const COMPANY_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1";
// Another synthetic company identifier, already used by the import-post tests,
// for the one hand edit of the company.
const OTHER_COMPANY_GUID: &str = "00000000-0000-4000-8000-000000000002";
const SIBLING: &str = "SC Sibling Goods";
const MARKED_NAME: &str = "SC Copied Marked";
const PLAIN_NAME: &str = "SC Copied Goods";
const MARKED_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1-000000d2";
const ANSWER: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-create-answer.utf16le.xml"
);
const BEFORE: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-before.utf16le.xml"
);
const PLAIN: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-after-plain-create.utf16le.xml"
);
const MARKED: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-after-marked-create.utf16le.xml"
);
const ALIAS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-with-alias.utf16le.xml"
);
const GROUPS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-groups.utf16le.xml"
);
const UNITS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-units.utf16le.xml"
);

fn decode(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("the capture decodes")
    .text
}

fn catalogue_for(company_guid: &str, xml: &str) -> StockItemCatalogue {
    StockItemCatalogue::from_candidate_read(
        &VerifiedCompanyIdentity::test_fixture(COMPANY, company_guid),
        xml,
    )
    .expect("the read catalogues")
}

fn catalogue(xml: &str) -> StockItemCatalogue {
    catalogue_for(COMPANY_GUID, xml)
}

fn name(value: &str) -> MasterName {
    MasterName::new(value).expect("the test name is admitted")
}

/// The draft named `new_name`, copied from `SC Sibling Goods` in `before`.
fn shown<'c>(before: &'c StockItemCatalogue, new_name: &str) -> StockItemDraft<'c> {
    let absent = before
        .name_absent(&name(new_name))
        .expect("the new name is absent");
    let sibling = SiblingGst::named(before, &name(SIBLING)).expect("the sibling is complete");
    let sibling = GoodsSibling::from_sibling(sibling).expect("the sibling is the Goods case");
    StockItemDraft::new(absent, sibling, &decode(GROUPS), &decode(UNITS))
        .expect("the sibling drafts")
}

/// The GUIDs of a read, as the set taken before a Create.
fn guids(read: &StockItemCatalogue) -> BTreeSet<String> {
    read.items().iter().map(|item| item.guid.clone()).collect()
}

/// `ReadBackVerdict::of` for `shown`'s writes, with the company and GUIDs of
/// the catalogue its sibling was found in.
fn verdict(shown: &StockItemDraft<'_>, fresh: &StockItemCatalogue) -> ReadBackVerdict {
    let before = shown.sibling().sibling().catalogue();
    ReadBackVerdict::of(before.company(), shown.writes(), &guids(before), fresh)
}

/// `VerifiedItem::new` with the same arguments as `verdict`.
fn verified(
    outcome: StockItemImportOutcome,
    shown: &StockItemDraft<'_>,
    fresh: &StockItemCatalogue,
) -> Result<VerifiedItem, NotVerified> {
    let before = shown.sibling().sibling().catalogue();
    VerifiedItem::new(
        outcome,
        before.company(),
        shown.writes(),
        &guids(before),
        fresh,
    )
}

fn outcome(xml: &str) -> StockItemImportOutcome {
    StockItemImportOutcome::of(&parse_import_outcome(xml).expect("the answer parses"))
}

fn edited(xml: &str, from: &str, to: &str) -> String {
    assert_eq!(xml.matches(from).count(), 1, "the edit starts in one place");
    xml.replacen(from, to, 1)
}

/// Replaces, inside the one row named `row`, the first `from` after `anchor`.
fn edited_in_row(xml: &str, row: &str, anchor: &str, from: &str, to: &str) -> String {
    let opening = format!("<STOCKITEM NAME=\"{row}\"");
    assert_eq!(xml.matches(&opening).count(), 1, "the row occurs once");
    let start = xml.find(&opening).expect("the row occurs");
    let end = start + xml[start..].find("</STOCKITEM>").expect("the row closes");
    let row_text = &xml[start..end];
    assert_eq!(
        row_text.matches(anchor).count(),
        1,
        "the anchor occurs once"
    );
    let at = start + row_text.find(anchor).expect("the anchor occurs");
    let offset = xml[at..end].find(from).expect("the edit starts in the row");
    let mut out = xml.to_string();
    out.replace_range(at + offset..at + offset + from.len(), to);
    out
}

fn differs(fields: &[SiblingField]) -> ReadBackVerdict {
    ReadBackVerdict::Differs {
        fields: fields.to_vec(),
    }
}

fn clean_create() -> StockItemImportOutcome {
    StockItemImportOutcome::CountersClean(CleanCreate(()))
}

fn clean_counters() -> ImportCounters {
    ImportCounters {
        created: 1,
        altered: 0,
        deleted: 0,
        ignored: 0,
        errors: 0,
        cancelled: 0,
        exceptions: 0,
    }
}

#[test]
fn the_captured_create_answer_is_counters_clean() {
    assert_eq!(outcome(&decode(ANSWER)), clean_create());
}

#[test]
fn an_answer_missing_a_counter_is_unreported() {
    // Hand edit: removes the CANCELLED counter, which the parser admits as not
    // reported. It shows the check fires on edited text, not how Tally behaves.
    let answer = edited(&decode(ANSWER), "<CANCELLED>0</CANCELLED>", "");
    assert_eq!(outcome(&answer), StockItemImportOutcome::Unreported);
}

#[test]
fn an_answer_that_altered_and_created_nothing_is_altered_existing() {
    // Hand edit: CREATED 1 to 0 and ALTERED 0 to 1. It shows the check fires on
    // edited text, not how Tally behaves.
    let answer = edited(
        &decode(ANSWER),
        "<CREATED>1</CREATED>",
        "<CREATED>0</CREATED>",
    );
    let answer = edited(&answer, "<ALTERED>0</ALTERED>", "<ALTERED>1</ALTERED>");
    assert_eq!(
        outcome(&answer),
        StockItemImportOutcome::AlteredExisting {
            counters: ImportCounters {
                created: 0,
                altered: 1,
                ..clean_counters()
            },
            line_errors: 0,
        }
    );
}

#[test]
fn an_answer_with_an_error_counter_is_not_clean() {
    // Hand edit: CREATED 1 to 0 and ERRORS 0 to 1. It shows the check fires on
    // edited text, not how Tally behaves.
    let answer = edited(
        &decode(ANSWER),
        "<CREATED>1</CREATED>",
        "<CREATED>0</CREATED>",
    );
    let answer = edited(&answer, "<ERRORS>0</ERRORS>", "<ERRORS>1</ERRORS>");
    assert_eq!(
        outcome(&answer),
        StockItemImportOutcome::NotClean {
            counters: ImportCounters {
                created: 0,
                errors: 1,
                ..clean_counters()
            },
            line_errors: 0,
        }
    );
}

#[test]
fn clean_counters_with_a_line_error_are_not_clean() {
    // Hand edit: adds one LINEERROR and leaves the counters clean. It shows the
    // check fires on edited text, not how Tally behaves.
    let answer = edited(
        &decode(ANSWER),
        "</RESPONSE>",
        "<LINEERROR>Hand-edited line error</LINEERROR></RESPONSE>",
    );
    assert_eq!(
        outcome(&answer),
        StockItemImportOutcome::NotClean {
            counters: clean_counters(),
            line_errors: 1,
        }
    );
}

#[test]
fn the_marked_create_reads_back_equal_and_is_verified() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, MARKED_NAME);
    let verdict = verdict(&shown, &catalogue(&decode(MARKED)));
    assert_eq!(
        verdict,
        ReadBackVerdict::Equal(EqualRead {
            guid: MARKED_GUID.to_string(),
        })
    );
    let verified = verified(
        outcome(&decode(ANSWER)),
        &shown,
        &catalogue(&decode(MARKED)),
    )
    .expect("a clean answer with an equal read is verified");
    assert_eq!(verified.guid(), MARKED_GUID);
    assert_eq!(verified.name(), &name(MARKED_NAME));
}

#[test]
fn the_plain_create_reads_back_without_applicability_and_gst_details_and_is_not_verified() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, PLAIN_NAME);
    let verdict = verdict(&shown, &catalogue(&decode(PLAIN)));
    let expected = differs(&[SiblingField::Applicability, SiblingField::GstDetails]);
    assert_eq!(verdict, expected);
    let outcome = outcome(&decode(ANSWER));
    assert_eq!(outcome, clean_create());
    let refused =
        verified(outcome, &shown, &catalogue(&decode(PLAIN))).expect_err("the read differs");
    assert_eq!(refused.outcome, clean_create());
    assert_eq!(refused.verdict, expected);
}

const CGST: &str = "<GSTRATEDUTYHEAD TYPE=\"String\">CGST</GSTRATEDUTYHEAD>";
const RATE_9: &str = "<GSTRATE TYPE=\"Number\"> 9</GSTRATE>";

#[test]
fn a_changed_rate_differs_in_the_gst_details_only() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, MARKED_NAME);
    // Hand edit: the new row's CGST rate 9 to 8. It shows the check fires on
    // edited text, not how Tally behaves.
    let read = edited_in_row(
        &decode(MARKED),
        MARKED_NAME,
        CGST,
        RATE_9,
        "<GSTRATE TYPE=\"Number\"> 8</GSTRATE>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&read)),
        differs(&[SiblingField::GstDetails])
    );
}

#[test]
fn a_rate_equal_in_value_but_spelt_otherwise_differs() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, MARKED_NAME);
    // Hand edit: the new row's CGST rate `9` to `9.00`, pinning the exact-text
    // rule (row 1381-O, o4). It shows the check fires on edited text, not how
    // Tally behaves.
    let read = edited_in_row(
        &decode(MARKED),
        MARKED_NAME,
        CGST,
        RATE_9,
        "<GSTRATE TYPE=\"Number\"> 9.00</GSTRATE>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&read)),
        differs(&[SiblingField::GstDetails])
    );
}

#[test]
fn a_changed_parent_or_unit_differs_in_that_field_only() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, MARKED_NAME);
    let parent = "<PARENT TYPE=\"String\">SC Goods Group</PARENT>";
    // Hand edit: the new row's PARENT to another name. It shows the check fires
    // on edited text, not how Tally behaves.
    let read = edited_in_row(
        &decode(MARKED),
        MARKED_NAME,
        parent,
        parent,
        "<PARENT TYPE=\"String\">SC Other Group</PARENT>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&read)),
        differs(&[SiblingField::Parent])
    );
    let unit = "<BASEUNITS TYPE=\"String\">Nos</BASEUNITS>";
    // Hand edit: the new row's BASEUNITS to another name. It shows the check
    // fires on edited text, not how Tally behaves.
    let read = edited_in_row(
        &decode(MARKED),
        MARKED_NAME,
        unit,
        unit,
        "<BASEUNITS TYPE=\"String\">Box</BASEUNITS>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&read)),
        differs(&[SiblingField::Unit])
    );
}

#[test]
fn a_changed_hsn_code_differs_in_the_hsn_details_only() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, MARKED_NAME);
    let code = "<HSNCODE TYPE=\"String\">39239090</HSNCODE>";
    // Hand edit: the new row's HSN code to another code. It shows the check
    // fires on edited text, not how Tally behaves.
    let read = edited_in_row(
        &decode(MARKED),
        MARKED_NAME,
        code,
        code,
        "<HSNCODE TYPE=\"String\">39239091</HSNCODE>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&read)),
        differs(&[SiblingField::HsnDetails])
    );
}

#[test]
fn a_row_the_goods_rule_or_the_measured_shape_refuses_names_that_field() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, MARKED_NAME);
    let applicable = "<GSTAPPLICABLE TYPE=\"String\">&#4; Applicable</GSTAPPLICABLE>";
    // Hand edit: the new row's applicability without its marked form, which
    // `GoodsSibling::from_sibling` refuses. It shows the check fires on edited
    // text, not how Tally behaves.
    let read = edited_in_row(
        &decode(MARKED),
        MARKED_NAME,
        applicable,
        applicable,
        "<GSTAPPLICABLE TYPE=\"String\">Applicable</GSTAPPLICABLE>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&read)),
        differs(&[SiblingField::Applicability])
    );
    let state = "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>";
    // Hand edit: the new row's state row without its marked form, which
    // `measured_shape` refuses. It shows the check fires on edited text, not
    // how Tally behaves.
    let read = edited_in_row(
        &decode(MARKED),
        MARKED_NAME,
        state,
        state,
        "<STATENAME TYPE=\"String\">Any</STATENAME>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&read)),
        differs(&[SiblingField::GstDetails])
    );
}

#[test]
fn a_name_not_held_once_as_a_row_name_is_not_found_once() {
    let before = catalogue(&decode(BEFORE));
    // The draft's name in the read before the Create: no item holds it.
    assert_eq!(
        verdict(&shown(&before, MARKED_NAME), &before),
        ReadBackVerdict::NotFoundOnce { rows: 0 }
    );
    // A case variant of the created name: no item holds it exactly.
    assert_eq!(
        verdict(
            &shown(&before, "sc copied marked"),
            &catalogue(&decode(MARKED))
        ),
        ReadBackVerdict::NotFoundOnce { rows: 0 }
    );
    // The alias keyed on the sibling: one item holds it, as a name entry only.
    assert_eq!(
        verdict(
            &shown(&before, "SC Sibling Alias"),
            &catalogue(&decode(ALIAS))
        ),
        ReadBackVerdict::NotFoundOnce { rows: 1 }
    );
}

#[test]
fn a_row_whose_guid_was_in_the_before_read_is_not_new() {
    let before_xml = decode(BEFORE);
    let before = catalogue(&before_xml);
    let shown = shown(&before, MARKED_NAME);
    // Hand edit: the before read with the sibling's row name and name entry
    // changed to the draft's name, its GUID kept. A draft named as the sibling
    // cannot be built, because `name_absent` refuses that name. It shows the
    // check fires on edited text, not how Tally behaves.
    let renamed = edited(
        &before_xml,
        "<STOCKITEM NAME=\"SC Sibling Goods\"",
        "<STOCKITEM NAME=\"SC Copied Marked\"",
    );
    let renamed = edited(
        &renamed,
        "<NAME>SC Sibling Goods</NAME>",
        "<NAME>SC Copied Marked</NAME>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&renamed)),
        ReadBackVerdict::NotNew
    );
}

#[test]
fn a_read_for_another_company_is_refused() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, MARKED_NAME);
    let marked = decode(MARKED);
    // The parser ties each row's GUID to the company identifier as well as its
    // `BRIDGECOMPANYGUID`, so the identifier occurs twice per row.
    assert_eq!(marked.matches(COMPANY_GUID).count(), 6, "two per row");
    // Hand edit: every occurrence of the company identifier, and nothing else.
    // It shows the check fires on edited text, not how Tally behaves.
    let other = marked.replace(COMPANY_GUID, OTHER_COMPANY_GUID);
    assert_eq!(
        verdict(&shown, &catalogue_for(OTHER_COMPANY_GUID, &other)),
        ReadBackVerdict::OtherCompany
    );
    // The captured read, unedited, catalogued for an identity with the same
    // identifier and another name: the whole identity is compared.
    let renamed_company = StockItemCatalogue::from_candidate_read(
        &VerifiedCompanyIdentity::test_fixture("BRIDGE STOCK CREATE LAB 2", COMPANY_GUID),
        &marked,
    )
    .expect("the read catalogues");
    assert_eq!(
        verdict(&shown, &renamed_company),
        ReadBackVerdict::OtherCompany
    );
}

#[test]
fn only_a_clean_answer_with_an_equal_read_is_verified() {
    let before_xml = decode(BEFORE);
    let before = catalogue(&before_xml);
    let shown = shown(&before, MARKED_NAME);
    let marked_xml = decode(MARKED);
    let marked = catalogue(&marked_xml);
    let equal = verdict(&shown, &marked);
    assert!(matches!(equal, ReadBackVerdict::Equal(_)));
    let clean = outcome(&decode(ANSWER));
    assert_eq!(clean, clean_create());

    let outcomes = [
        StockItemImportOutcome::AlteredExisting {
            counters: ImportCounters {
                created: 0,
                altered: 1,
                ..clean_counters()
            },
            line_errors: 0,
        },
        StockItemImportOutcome::NotClean {
            counters: clean_counters(),
            line_errors: 1,
        },
        StockItemImportOutcome::Unreported,
        StockItemImportOutcome::ApplicationFailure,
    ];
    for other in outcomes {
        let refused =
            verified(other.clone(), &shown, &marked).expect_err("only a clean answer is verified");
        assert_eq!(refused.outcome, other);
        assert_eq!(refused.verdict, equal);
    }

    // Each read below is a hand edit or a read taken before the Create, as in
    // the verdict tests above; they show the check fires, not how Tally
    // behaves.
    let changed_rate = edited_in_row(
        &marked_xml,
        MARKED_NAME,
        CGST,
        RATE_9,
        "<GSTRATE TYPE=\"Number\"> 8</GSTRATE>",
    );
    let renamed_sibling = edited(
        &edited(
            &before_xml,
            "<STOCKITEM NAME=\"SC Sibling Goods\"",
            "<STOCKITEM NAME=\"SC Copied Marked\"",
        ),
        "<NAME>SC Sibling Goods</NAME>",
        "<NAME>SC Copied Marked</NAME>",
    );
    let reads = [
        (
            catalogue(&changed_rate),
            differs(&[SiblingField::GstDetails]),
        ),
        (
            catalogue(&before_xml),
            ReadBackVerdict::NotFoundOnce { rows: 0 },
        ),
        (catalogue(&renamed_sibling), ReadBackVerdict::NotNew),
        (
            catalogue_for(
                OTHER_COMPANY_GUID,
                &marked_xml.replace(COMPANY_GUID, OTHER_COMPANY_GUID),
            ),
            ReadBackVerdict::OtherCompany,
        ),
    ];
    for (read, expected) in reads {
        let refused =
            verified(clean.clone(), &shown, &read).expect_err("only an equal read is verified");
        assert_eq!(refused.outcome, clean);
        assert_eq!(refused.verdict, expected);
    }

    let verified = verified(clean, &shown, &marked).expect("the captured pair verifies");
    assert_eq!(verified.guid(), MARKED_GUID);
}

/// The captured answer's counters inside an import `ENVELOPE` whose
/// `HEADER/STATUS` is `status`. A hand edit: it shows the check fires on edited
/// text, not how Tally behaves.
fn in_envelope(status: &str) -> String {
    let answer = edited(
        &decode(ANSWER),
        "<RESPONSE>",
        &format!(
            "<ENVELOPE><HEADER><STATUS>{status}</STATUS></HEADER>\
             <BODY><DATA><IMPORTRESULT>"
        ),
    );
    edited(
        &answer,
        "</RESPONSE>",
        "</IMPORTRESULT></DATA></BODY></ENVELOPE>",
    )
}

#[test]
fn a_failure_status_with_clean_counters_is_an_application_failure() {
    assert_eq!(
        outcome(&in_envelope("0")),
        StockItemImportOutcome::ApplicationFailure
    );
    // The same counters under a success status are clean, so the status alone
    // decides.
    assert_eq!(outcome(&in_envelope("1")), clean_create());
}

#[test]
fn a_name_held_by_two_items_is_not_found_once() {
    let before = catalogue(&decode(BEFORE));
    let shown = shown(&before, MARKED_NAME);
    // Hand edit: the sibling's row also takes the created name as a name
    // entry, so the new row holds it as its row name and the sibling as an
    // entry. It shows the check fires on edited text, not how Tally behaves.
    let read = edited(
        &decode(MARKED),
        "<NAME>SC Sibling Goods</NAME>",
        "<NAME>SC Sibling Goods</NAME><NAME>SC Copied Marked</NAME>",
    );
    assert_eq!(
        verdict(&shown, &catalogue(&read)),
        ReadBackVerdict::NotFoundOnce { rows: 2 }
    );
    // Hand edit: the sibling's row name and name entry both changed to the
    // created name, so two rows would hold it as their row name. The parser
    // refuses such a read, so no verdict is reached. It shows the check fires
    // on edited text, not how Tally behaves.
    let read = edited(
        &edited(
            &decode(MARKED),
            "<STOCKITEM NAME=\"SC Sibling Goods\"",
            "<STOCKITEM NAME=\"SC Copied Marked\"",
        ),
        "<NAME>SC Sibling Goods</NAME>",
        "<NAME>SC Copied Marked</NAME>",
    );
    assert_eq!(
        StockItemCatalogue::from_candidate_read(
            &VerifiedCompanyIdentity::test_fixture(COMPANY, COMPANY_GUID),
            &read,
        )
        .expect_err("two rows of one name are refused"),
        StockItemCatalogueError::Unreadable(NativeMastersError::DuplicateName)
    );
}

#[test]
fn a_draft_is_judged_against_the_read_it_is_given() {
    // The read after both Creates, given with the plain draft: `new` judges
    // the plain draft's own row in it, which differs, and not the marked row
    // that equals the marked draft.
    let before = catalogue(&decode(BEFORE));
    let plain = shown(&before, PLAIN_NAME);
    let refused = verified(clean_create(), &plain, &catalogue(&decode(MARKED)))
        .expect_err("the plain draft's row differs");
    assert_eq!(
        refused.verdict,
        differs(&[SiblingField::Applicability, SiblingField::GstDetails])
    );
}

/// The marked draft's writes, stored as plain values and rebuilt, with the
/// draft and its catalogue gone.
fn rebuilt_marked_writes() -> StockItemWrites {
    let before = catalogue(&decode(BEFORE));
    let plain = shown(&before, MARKED_NAME).writes().to_plain();
    StockItemWrites::rebuild(&plain).expect("the stored values admit")
}

#[test]
fn rebuilt_writes_read_back_equal_to_the_marked_create_and_verify() {
    let writes = rebuilt_marked_writes();
    let company = VerifiedCompanyIdentity::test_fixture(COMPANY, COMPANY_GUID);
    let before = guids(&catalogue(&decode(BEFORE)));
    let marked = catalogue(&decode(MARKED));
    assert_eq!(
        ReadBackVerdict::of(&company, &writes, &before, &marked),
        ReadBackVerdict::Equal(EqualRead {
            guid: MARKED_GUID.to_string(),
        })
    );
    let verified = VerifiedItem::new(
        outcome(&decode(ANSWER)),
        &company,
        &writes,
        &before,
        &marked,
    )
    .expect("a clean answer with an equal read is verified");
    assert_eq!(verified.guid(), MARKED_GUID);
    assert_eq!(verified.name(), &name(MARKED_NAME));
}

#[test]
fn rebuilt_writes_whose_row_guid_is_in_the_before_set_are_not_new() {
    let writes = rebuilt_marked_writes();
    let company = VerifiedCompanyIdentity::test_fixture(COMPANY, COMPANY_GUID);
    let marked = catalogue(&decode(MARKED));
    // The before set taken from the read after the Create holds the new row's
    // own GUID, so the row is not new.
    let after = guids(&marked);
    assert!(after.contains(MARKED_GUID));
    assert_eq!(
        ReadBackVerdict::of(&company, &writes, &after, &marked),
        ReadBackVerdict::NotNew
    );
    let refused = VerifiedItem::new(clean_create(), &company, &writes, &after, &marked)
        .expect_err("a row that is not new is not verified");
    assert_eq!(refused.verdict, ReadBackVerdict::NotNew);
}
