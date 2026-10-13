//! The voucher-numbering admission (bridge#724) without Tally: which readings
//! admit, which refuse and how each refusal is named, and the mark compare the
//! queue makes before the POST.
use super::location::LoadedCompanyMarks;
use super::numbering::{
    admit_numbering as admit_vouchers, NumberingAdmission, NumberingRefusal, OwnNumber,
    RefusalReason, BUILD_AMENDMENT_NEXT_STEP, BUILD_NEXT_STEP, NUMBERING_NOT_AUTOMATIC,
    NUMBERING_NOT_RETAIN, POST_NEXT_STEP,
};
use crate::tally::approved_import::ApprovedImportAdmissionError;
use crate::tally::runtime::VoucherNumberingRead;
use bridge_tally_protocol::native_voucher_type_numbering::{
    parse_voucher_type_numbering, DuplicateSetting, NumberingSubMethod, SeriesNumberingMethod,
    VoucherTypeNumbering,
};
use serde_json::json;

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const NAME: &str = "WR2 Unicode Lab";

/// The admission of vouchers that carry no number, as a post's are and as
/// every test below that does not say otherwise means.
fn admit_numbering<'a>(
    read: VoucherNumberingRead,
    voucher_types: impl IntoIterator<Item = &'a str>,
) -> Result<NumberingAdmission, NumberingRefusal> {
    admit_vouchers(
        read,
        voucher_types
            .into_iter()
            .map(|voucher_type| (voucher_type, OwnNumber::Absent)),
    )
}

fn observed(method: SeriesNumberingMethod, duplicates: DuplicateSetting) -> VoucherTypeNumbering {
    VoucherTypeNumbering::Observed { method, duplicates }
}

fn automatic() -> VoucherTypeNumbering {
    observed(
        SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRetain),
        DuplicateSetting::NotPrevented,
    )
}

/// A read of the four types a native voucher can be, Journal reading `journal`.
fn read_with_journal(journal: VoucherTypeNumbering) -> VoucherNumberingRead {
    VoucherNumberingRead::for_test(
        GUID,
        7,
        vec![
            ("Payment".to_string(), automatic()),
            ("Receipt".to_string(), automatic()),
            ("Journal".to_string(), journal),
            ("Contra".to_string(), automatic()),
        ],
    )
}

/// A refusal's rows: the voucher type, why it is refused and what was read.
fn rows(refusal: &NumberingRefusal) -> Vec<(&str, RefusalReason, &serde_json::Value)> {
    refusal
        .0
        .iter()
        .map(|row| (row.voucher_type.as_str(), row.reason, &row.observed))
        .collect()
}

fn refused(
    result: Result<super::numbering::NumberingAdmission, NumberingRefusal>,
) -> Vec<(String, serde_json::Value)> {
    match result {
        Err(NumberingRefusal(rows)) => rows
            .into_iter()
            .map(|row| (row.voucher_type, row.observed))
            .collect(),
        other => panic!("expected a numbering refusal, got {other:?}"),
    }
}

#[test]
fn automatic_admits_under_either_duplicate_setting() {
    for duplicates in [DuplicateSetting::Prevented, DuplicateSetting::NotPrevented] {
        let read = read_with_journal(observed(
            SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRetain),
            duplicates,
        ));
        assert!(admit_numbering(read, ["Journal"]).is_ok(), "{duplicates:?}");
    }
}

#[test]
fn every_reading_other_than_automatic_refuses_and_says_what_was_read() {
    let cases = [
        (
            observed(SeriesNumberingMethod::Manual, DuplicateSetting::Prevented),
            json!({"observation":"observed","method":"manual","duplicates":"prevented"}),
        ),
        (
            observed(
                SeriesNumberingMethod::Manual,
                DuplicateSetting::NotPrevented,
            ),
            json!({"observation":"observed","method":"manual","duplicates":"not_prevented"}),
        ),
        (
            observed(
                SeriesNumberingMethod::AutomaticManualOverride,
                DuplicateSetting::NotPrevented,
            ),
            json!({"observation":"observed","method":"automatic_manual_override","duplicates":"not_prevented"}),
        ),
        // Tally's own text is never repeated: only that it was not recognised.
        (
            observed(
                SeriesNumberingMethod::Unrecognised("Multi-user Auto <x>".to_string()),
                DuplicateSetting::NotPrevented,
            ),
            json!({"observation":"observed","method":"unrecognised","duplicates":"not_prevented"}),
        ),
        (
            VoucherTypeNumbering::SeriesCount(0),
            json!({"observation":"series_count","count":0}),
        ),
        (
            VoucherTypeNumbering::SeriesCount(2),
            json!({"observation":"series_count","count":2}),
        ),
        (
            VoucherTypeNumbering::FieldInvalid("numbering_method_absent"),
            json!({"observation":"field_invalid","code":"numbering_method_absent"}),
        ),
    ];
    for (numbering, expected) in cases {
        let refusal =
            admit_numbering(read_with_journal(numbering.clone()), ["Journal"]).unwrap_err();
        assert_eq!(refusal.code(), NUMBERING_NOT_AUTOMATIC, "{numbering:?}");
        assert_eq!(
            rows(&refusal),
            [("Journal", RefusalReason::NotAutomatic, &expected)],
            "{numbering:?}"
        );
    }
}

/// An Automatic type is admitted under `Auto Retain` only. `observed` names
/// the sub-method and never repeats Tally's own text.
#[test]
fn an_automatic_type_not_read_as_retain_refuses_and_says_what_was_read() {
    let cases = [
        (NumberingSubMethod::AutoRenumber, "auto_renumber"),
        (NumberingSubMethod::Absent, "absent"),
        (
            NumberingSubMethod::Unrecognised("x<y>".to_string()),
            "unrecognised",
        ),
        (
            NumberingSubMethod::Unrecognised(String::new()),
            "unrecognised",
        ),
        (
            NumberingSubMethod::Unrecognised("auto retain".to_string()),
            "unrecognised",
        ),
    ];
    for (sub_method, named) in cases {
        for (duplicates, setting) in [
            (DuplicateSetting::Prevented, "prevented"),
            (DuplicateSetting::NotPrevented, "not_prevented"),
        ] {
            let read = read_with_journal(observed(
                SeriesNumberingMethod::Automatic(sub_method.clone()),
                duplicates,
            ));
            let refusal = admit_numbering(read, ["Journal"]).unwrap_err();
            assert_eq!(refusal.code(), NUMBERING_NOT_RETAIN, "{sub_method:?}");
            assert_eq!(
                rows(&refusal),
                [(
                    "Journal",
                    RefusalReason::NotRetain,
                    &json!({"observation":"observed","method":"automatic","sub_method":named,"duplicates":setting})
                )],
                "{sub_method:?}"
            );
        }
    }
}

/// A batch refused for both reasons is named not-Automatic wherever that row
/// stands; only a batch refused for its sub-method alone is named not-retain.
#[test]
fn a_batch_refused_for_both_reasons_takes_the_not_automatic_code() {
    let renumber = || {
        observed(
            SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRenumber),
            DuplicateSetting::NotPrevented,
        )
    };
    let manual = || {
        observed(
            SeriesNumberingMethod::Manual,
            DuplicateSetting::NotPrevented,
        )
    };
    let read = |payment: VoucherTypeNumbering, receipt: VoucherTypeNumbering| {
        VoucherNumberingRead::for_test(
            GUID,
            7,
            vec![
                ("Payment".to_string(), payment),
                ("Receipt".to_string(), receipt),
                ("Journal".to_string(), automatic()),
            ],
        )
    };
    let reasons = |refusal: &NumberingRefusal| {
        refusal
            .0
            .iter()
            .map(|row| (row.voucher_type.clone(), row.reason))
            .collect::<Vec<_>>()
    };
    let both = [
        ("Payment".to_string(), RefusalReason::NotRetain),
        ("Receipt".to_string(), RefusalReason::NotAutomatic),
    ];
    let refusal = admit_numbering(
        read(renumber(), manual()),
        ["Payment", "Journal", "Receipt"],
    )
    .unwrap_err();
    assert_eq!(reasons(&refusal), both);
    assert_eq!(refusal.code(), NUMBERING_NOT_AUTOMATIC);
    // The same two readings with the not-Automatic row first.
    let refusal = admit_numbering(
        read(manual(), renumber()),
        ["Payment", "Journal", "Receipt"],
    )
    .unwrap_err();
    assert_eq!(
        reasons(&refusal),
        [
            ("Payment".to_string(), RefusalReason::NotAutomatic),
            ("Receipt".to_string(), RefusalReason::NotRetain),
        ]
    );
    assert_eq!(refusal.code(), NUMBERING_NOT_AUTOMATIC);
    // Two types refused for the sub-method alone.
    let refusal = admit_numbering(
        read(renumber(), renumber()),
        ["Payment", "Journal", "Receipt"],
    )
    .unwrap_err();
    assert_eq!(
        reasons(&refusal),
        [
            ("Payment".to_string(), RefusalReason::NotRetain),
            ("Receipt".to_string(), RefusalReason::NotRetain),
        ]
    );
    assert_eq!(refusal.code(), NUMBERING_NOT_RETAIN);
}

#[test]
fn a_voucher_type_the_read_does_not_hold_refuses() {
    let read = VoucherNumberingRead::for_test(GUID, 7, vec![("Payment".to_string(), automatic())]);
    assert_eq!(
        refused(admit_numbering(read, ["Payment", "Contra"])),
        vec![("Contra".to_string(), json!({"observation":"absent"}))]
    );
}

#[test]
fn only_the_refused_types_are_named_once_each_in_batch_order() {
    let read = VoucherNumberingRead::for_test(
        GUID,
        7,
        vec![
            ("Payment".to_string(), automatic()),
            (
                "Receipt".to_string(),
                observed(
                    SeriesNumberingMethod::Manual,
                    DuplicateSetting::NotPrevented,
                ),
            ),
            ("Journal".to_string(), automatic()),
            ("Contra".to_string(), VoucherTypeNumbering::SeriesCount(2)),
        ],
    );
    let named = refused(admit_numbering(
        read,
        [
            "Payment", "Contra", "Receipt", "Journal", "Contra", "Receipt",
        ],
    ))
    .into_iter()
    .map(|(voucher_type, _)| voucher_type)
    .collect::<Vec<_>>();
    assert_eq!(named, ["Contra", "Receipt"]);
}

#[test]
fn a_refusal_on_a_later_voucher_is_not_missed() {
    let read = read_with_journal(observed(
        SeriesNumberingMethod::Manual,
        DuplicateSetting::Prevented,
    ));
    let result = admit_numbering(read, ["Payment", "Payment", "Journal"]);
    assert_eq!(refused(result).len(), 1);
}

#[test]
fn voucher_type_names_match_by_ascii_case_fold() {
    let read = VoucherNumberingRead::for_test(GUID, 7, vec![("JOURNAL".to_string(), automatic())]);
    assert!(admit_numbering(read, ["Journal"]).is_ok());
}

#[test]
fn the_refusal_code_and_its_listed_types() {
    let read = read_with_journal(observed(
        SeriesNumberingMethod::Manual,
        DuplicateSetting::Prevented,
    ));
    let refusal = admit_numbering(read, ["Journal"]).unwrap_err();
    assert_eq!(refusal.code(), NUMBERING_NOT_AUTOMATIC);
    assert_eq!(
        refusal.refused_types(),
        json!([{"voucher_type":"Journal","observed":{"observation":"observed","method":"manual","duplicates":"prevented"}}])
    );
}

/// The voucher types of a committed capture, as the series parser reads them.
fn captured_types(bytes: &[u8]) -> Vec<(String, VoucherTypeNumbering)> {
    let text = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    parse_voucher_type_numbering(text.trim_start_matches('\u{feff}'))
        .unwrap()
        .into_iter()
        .map(|row| (row.name, row.numbering))
        .collect()
}

/// The names of `types` other than those in `without`.
fn names_without<'a>(
    types: &'a [(String, VoucherTypeNumbering)],
    without: &[&str],
) -> Vec<&'a str> {
    types
        .iter()
        .map(|(name, _)| name.as_str())
        .filter(|name| !without.contains(name))
        .collect()
}

/// The second synthetic book (13 October 2026), read with its Receipt type set
/// to "Renumber Vouchers": Receipt is refused for its sub-method, Payment
/// (Manual in that book) as not Automatic, and the other 22 types are admitted.
#[test]
fn the_book_with_receipt_set_to_renumber_refuses_receipt_and_admits_the_22_on_retain() {
    let types = captured_types(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_type_numbering_m8r_x4_series.utf16le.xml"
    ));
    assert_eq!(types.len(), 24);
    let read = || VoucherNumberingRead::for_test(GUID, 7, types.clone());
    let on_retain = names_without(&types, &["Receipt", "Payment"]);
    assert_eq!(on_retain.len(), 22);
    assert!(admit_numbering(read(), on_retain).is_ok());

    let renumber = json!({"observation":"observed","method":"automatic","sub_method":"auto_renumber","duplicates":"not_prevented"});
    let receipt = ("Receipt", RefusalReason::NotRetain, &renumber);
    let refusal = admit_numbering(read(), ["Receipt"]).unwrap_err();
    assert_eq!(rows(&refusal), [receipt]);
    assert_eq!(refusal.code(), NUMBERING_NOT_RETAIN);
    assert_eq!(
        refusal.refused_types(),
        json!([{"voucher_type":"Receipt","observed":{"observation":"observed","method":"automatic","sub_method":"auto_renumber","duplicates":"not_prevented"}}])
    );

    // With the Manual type beside it the batch is named not-Automatic, and
    // the Manual row says nothing of a sub-method.
    let refusal = admit_numbering(read(), ["Receipt", "Journal", "Payment"]).unwrap_err();
    assert_eq!(
        rows(&refusal),
        [
            receipt,
            (
                "Payment",
                RefusalReason::NotAutomatic,
                &json!({"observation":"observed","method":"manual","duplicates":"not_prevented"})
            ),
        ]
    );
    assert_eq!(refusal.code(), NUMBERING_NOT_AUTOMATIC);
}

/// The same book before the Receipt type was changed, on "Retain Original
/// Voucher No.": Receipt is admitted.
#[test]
fn the_same_book_with_receipt_on_retain_admits_receipt() {
    let types = captured_types(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_type_numbering_m8r_x0_series.utf16le.xml"
    ));
    let read = || VoucherNumberingRead::for_test(GUID, 7, types.clone());
    assert!(admit_numbering(read(), ["Receipt"]).is_ok());
    let automatic = names_without(&types, &["Payment"]);
    assert_eq!(automatic.len(), 23);
    assert!(admit_numbering(read(), automatic).is_ok());
    let refusal = admit_numbering(read(), ["Receipt", "Payment"]).unwrap_err();
    assert_eq!(
        rows(&refusal),
        [(
            "Payment",
            RefusalReason::NotAutomatic,
            &json!({"observation":"observed","method":"manual","duplicates":"not_prevented"})
        )]
    );
    assert_eq!(refusal.code(), NUMBERING_NOT_AUTOMATIC);
}

/// The committed live capture (24 voucher types of a synthetic lab company):
/// Payment reads Manual with duplicates prevented and Receipt Manual with them
/// allowed; Journal and Contra read Automatic.
#[test]
fn the_captured_book_refuses_payment_and_receipt_and_admits_journal_and_contra() {
    let rows = captured_types(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_type_numbering_series_live.utf16le.xml"
    ));
    let read = || VoucherNumberingRead::for_test(GUID, 7, rows.clone());
    assert!(admit_numbering(read(), ["Journal", "Contra"]).is_ok());
    assert_eq!(
        refused(admit_numbering(
            read(),
            ["Payment", "Receipt", "Journal", "Contra"]
        )),
        vec![
            (
                "Payment".to_string(),
                json!({"observation":"observed","method":"manual","duplicates":"prevented"})
            ),
            (
                "Receipt".to_string(),
                json!({"observation":"observed","method":"manual","duplicates":"not_prevented"})
            ),
        ]
    );
}

/// A voucher that carries its own number: admitted into a type read as Manual
/// (Tally keeps the number) or as Automatic with `Auto Retain` (Tally discards
/// it and assigns the next), under either duplicate setting, and refused for
/// every other reading exactly as a voucher without one is, row for row.
#[test]
fn a_voucher_that_carries_a_number_is_admitted_into_manual_and_auto_retain_only() {
    let automatic = |sub_method| SeriesNumberingMethod::Automatic(sub_method);
    for duplicates in [DuplicateSetting::Prevented, DuplicateSetting::NotPrevented] {
        for method in [
            SeriesNumberingMethod::Manual,
            automatic(NumberingSubMethod::AutoRetain),
        ] {
            let read = read_with_journal(observed(method.clone(), duplicates));
            assert!(
                admit_vouchers(read, [("Journal", OwnNumber::Carried)]).is_ok(),
                "{method:?} {duplicates:?}"
            );
        }
    }
    let refusing = [
        (
            Some(observed(
                automatic(NumberingSubMethod::AutoRenumber),
                DuplicateSetting::NotPrevented,
            )),
            RefusalReason::NotRetain,
        ),
        (
            Some(observed(
                automatic(NumberingSubMethod::Absent),
                DuplicateSetting::Prevented,
            )),
            RefusalReason::NotRetain,
        ),
        (
            Some(observed(
                automatic(NumberingSubMethod::Unrecognised("x".to_string())),
                DuplicateSetting::NotPrevented,
            )),
            RefusalReason::NotRetain,
        ),
        (
            Some(observed(
                SeriesNumberingMethod::AutomaticManualOverride,
                DuplicateSetting::NotPrevented,
            )),
            RefusalReason::NotAutomatic,
        ),
        (
            Some(observed(
                SeriesNumberingMethod::Unrecognised("Serial".to_string()),
                DuplicateSetting::NotPrevented,
            )),
            RefusalReason::NotAutomatic,
        ),
        (
            Some(VoucherTypeNumbering::SeriesCount(0)),
            RefusalReason::NotAutomatic,
        ),
        (
            Some(VoucherTypeNumbering::SeriesCount(2)),
            RefusalReason::NotAutomatic,
        ),
        (
            Some(VoucherTypeNumbering::FieldInvalid("series_method")),
            RefusalReason::NotAutomatic,
        ),
        (None, RefusalReason::NotAutomatic),
    ];
    for (numbering, reason) in refusing {
        let read = || {
            VoucherNumberingRead::for_test(
                GUID,
                7,
                numbering
                    .clone()
                    .map(|numbering| ("Journal".to_string(), numbering))
                    .into_iter()
                    .collect(),
            )
        };
        let numbered = admit_vouchers(read(), [("Journal", OwnNumber::Carried)]).unwrap_err();
        assert_eq!(numbered.0.len(), 1, "{numbering:?}");
        assert_eq!(numbered.0[0].reason, reason, "{numbering:?}");
        // The row is the one a voucher without a number gets: the same
        // reading, said the same way.
        let numberless = admit_numbering(read(), ["Journal"]).unwrap_err();
        assert_eq!(numbered, numberless, "{numbering:?}");
    }
}

/// The number is read from the voucher: a blank one is none.
#[test]
fn a_blank_voucher_number_is_no_number() {
    for (given, own) in [
        (None, OwnNumber::Absent),
        (Some(""), OwnNumber::Absent),
        (Some("  "), OwnNumber::Absent),
        (Some("7"), OwnNumber::Carried),
        (Some(" JV/1 "), OwnNumber::Carried),
    ] {
        assert_eq!(OwnNumber::of(given), own, "{given:?}");
    }
}

/// One voucher type that a numbered and a numberless voucher both go into.
/// Under Manual only the numberless one refuses, in either order, and the
/// type is named once; where both refuse, it is still named once.
#[test]
fn a_type_holding_numbered_and_numberless_vouchers_is_named_once() {
    let manual = || {
        read_with_journal(observed(
            SeriesNumberingMethod::Manual,
            DuplicateSetting::Prevented,
        ))
    };
    let expected = json!({"observation":"observed","method":"manual","duplicates":"prevented"});
    for vouchers in [
        [
            ("Journal", OwnNumber::Carried),
            ("Journal", OwnNumber::Absent),
        ],
        [
            ("Journal", OwnNumber::Absent),
            ("Journal", OwnNumber::Carried),
        ],
    ] {
        let refusal = admit_vouchers(manual(), vouchers).unwrap_err();
        assert_eq!(
            rows(&refusal),
            [("Journal", RefusalReason::NotAutomatic, &expected)]
        );
        assert_eq!(refusal.code(), NUMBERING_NOT_AUTOMATIC);
    }
    let renumber = read_with_journal(observed(
        SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRenumber),
        DuplicateSetting::NotPrevented,
    ));
    let refusal = admit_vouchers(
        renumber,
        [
            ("Journal", OwnNumber::Carried),
            ("Journal", OwnNumber::Absent),
            ("JOURNAL", OwnNumber::Carried),
        ],
    )
    .unwrap_err();
    assert_eq!(refusal.0.len(), 1);
    assert_eq!(refusal.code(), NUMBERING_NOT_RETAIN);
}

/// What a person is shown where only the message is shown: each refused
/// voucher type by name with what was read of it, every reading in its own
/// words, and never Tally's own text for a reading that was not recognised;
/// then what the person does. Where a refused type is set to renumber it also
/// says what a hand entry was seen to do there, and not otherwise.
#[test]
fn the_message_names_each_refused_voucher_type_and_what_was_read_of_it() {
    let automatic = |sub_method| {
        observed(
            SeriesNumberingMethod::Automatic(sub_method),
            DuplicateSetting::NotPrevented,
        )
    };
    let readings = [
        (
            "Payment",
            Some(observed(
                SeriesNumberingMethod::Manual,
                DuplicateSetting::Prevented,
            )),
            "Payment is numbered Manual",
        ),
        (
            "Receipt",
            Some(automatic(NumberingSubMethod::AutoRenumber)),
            "Receipt is numbered Automatic with Renumber Vouchers",
        ),
        (
            "Contra",
            Some(automatic(NumberingSubMethod::Absent)),
            "Contra is numbered Automatic, and Tally did not say",
        ),
        (
            "Journal",
            Some(automatic(NumberingSubMethod::Unrecognised(
                "x<y>".to_string(),
            ))),
            "Journal is numbered Automatic, with a setting",
        ),
        (
            "Purchase",
            Some(observed(
                SeriesNumberingMethod::AutomaticManualOverride,
                DuplicateSetting::NotPrevented,
            )),
            "Purchase is numbered Automatic (Manual Override)",
        ),
        (
            "Memorandum",
            Some(observed(
                SeriesNumberingMethod::Unrecognised("Multi <z>".to_string()),
                DuplicateSetting::NotPrevented,
            )),
            "Memorandum has a numbering method",
        ),
        (
            "Credit Note",
            Some(VoucherTypeNumbering::SeriesCount(2)),
            "Credit Note does not have exactly one number series",
        ),
        (
            "Debit Note",
            Some(VoucherTypeNumbering::FieldInvalid("series_method")),
            "Debit Note has numbering settings",
        ),
        ("Sales", None, "Sales was not among the voucher types"),
    ];
    let read = VoucherNumberingRead::for_test(
        GUID,
        7,
        readings
            .iter()
            .filter_map(|(name, numbering, _)| Some(((*name).to_string(), numbering.clone()?)))
            .collect(),
    );
    let refusal = admit_numbering(read, readings.iter().map(|(name, _, _)| *name)).unwrap_err();
    assert_eq!(refusal.0.len(), readings.len());
    let message = refusal.message();
    for (_, _, words) in &readings {
        assert!(message.contains(words), "{words}: {message}");
    }
    assert!(!message.contains("x<y>"), "{message}");
    assert!(!message.contains("<z>"), "{message}");

    const DOES_NEXT: [&str; 2] = [
        "enter these vouchers in Tally's voucher entry screen yourself",
        "Do not change a voucher type's voucher numbering only to post them",
    ];
    const SEEN_BY_HAND: &str = "a voucher entered by hand with an earlier date was seen to change";
    let manual_only = admit_numbering(
        read_with_journal(observed(
            SeriesNumberingMethod::Manual,
            DuplicateSetting::Prevented,
        )),
        ["Journal"],
    )
    .unwrap_err()
    .message();
    for closing in DOES_NEXT {
        assert!(message.contains(closing), "{closing}: {message}");
        assert!(manual_only.contains(closing), "{closing}: {manual_only}");
    }
    assert!(message.contains(SEEN_BY_HAND), "{message}");
    assert!(!manual_only.contains(SEEN_BY_HAND), "{manual_only}");

    // Where the type's setting for existing numbers was not read, missing or
    // not recognised, the message says a hand entry may change them; it says
    // so of no other reading, and what was seen under renumber stays its own
    // sentence.
    const NOT_KNOWN: &str = "a hand entry may change existing voucher numbers";
    assert!(message.contains(NOT_KNOWN), "{message}");
    assert!(!manual_only.contains(NOT_KNOWN), "{manual_only}");
    let only = |sub_method| {
        admit_numbering(read_with_journal(automatic(sub_method)), ["Journal"])
            .unwrap_err()
            .message()
    };
    for not_read in [
        NumberingSubMethod::Absent,
        NumberingSubMethod::Unrecognised("x".to_string()),
    ] {
        let said = only(not_read.clone());
        assert!(said.contains(NOT_KNOWN), "{not_read:?}: {said}");
        assert!(!said.contains(SEEN_BY_HAND), "{not_read:?}: {said}");
    }
    let renumber = only(NumberingSubMethod::AutoRenumber);
    assert!(renumber.contains(SEEN_BY_HAND), "{renumber}");
    assert!(!renumber.contains(NOT_KNOWN), "{renumber}");
}

/// The three next steps share what they say was read and seen, and differ in
/// what the user is asked to do. Both refusal codes give the post's.
#[test]
fn every_next_step_names_the_admitted_reading_and_the_import_that_renumbered() {
    for step in [POST_NEXT_STEP, BUILD_NEXT_STEP, BUILD_AMENDMENT_NEXT_STEP] {
        for stated in [
            "Auto Retain",
            "sent by import without a number",
            "the import's answer reported nothing altered",
            "one book, TallyPrime 7.1 Silver: by import on a Receipt and a Journal, by hand on a Receipt",
            "do not add a voucher number to it",
            "do not change a voucher type's voucher numbering to get past this",
        ] {
            assert!(step.contains(stated), "{stated}: {step}");
        }
    }
    let post = crate::agent::refusal_remediation(NUMBERING_NOT_RETAIN);
    assert_eq!(post, Some(POST_NEXT_STEP));
    assert_eq!(
        post,
        crate::agent::refusal_remediation(NUMBERING_NOT_AUTOMATIC)
    );
    for asks_for_a_hand_entry in [POST_NEXT_STEP, BUILD_NEXT_STEP] {
        assert!(asks_for_a_hand_entry.contains("Ask the user to enter the vouchers"));
    }
    // An amendment is refused before its book check: its step claims nothing
    // of the book and sends the caller to verify_import.
    // A build's vouchers may carry a number: neither build step says they
    // have none, and both say where one that does is written.
    for build in [BUILD_NEXT_STEP, BUILD_AMENDMENT_NEXT_STEP] {
        assert!(!build.contains("without a voucher number"), "{build}");
        assert!(build.contains("is also written into one read as Manual"));
    }
    // A written file of a refused post is not to be imported by hand either;
    // only the post's step can speak of one.
    const NOT_BY_THE_MENU: &str = "imported through Tally's Import menu either";
    assert!(POST_NEXT_STEP.contains(NOT_BY_THE_MENU));
    assert!(!BUILD_NEXT_STEP.contains(NOT_BY_THE_MENU));
    // An amendment alters vouchers in place: its step says they are written
    // without a number where they carry none, and nothing of how Tally
    // numbers them. The build of new vouchers says what was measured.
    assert!(!BUILD_AMENDMENT_NEXT_STEP.contains("Tally numbers"));
    assert!(BUILD_NEXT_STEP.contains("Tally numbers a voucher that carries no number"));
    assert!(!BUILD_AMENDMENT_NEXT_STEP.contains("Ask the user to enter"));
    for stated in [
        "was not checked before this refusal",
        "Call verify_import",
        "is never entered again",
        "that carry no number are written without one",
    ] {
        assert!(BUILD_AMENDMENT_NEXT_STEP.contains(stated), "{stated}");
    }
}

/// The two tools that refuse on the numbering say so: each description names
/// both codes, what was seen on an import and its scope, and what was not
/// measured.
#[test]
fn the_build_and_post_descriptions_name_both_refusals_and_what_was_not_measured() {
    let definitions = crate::agent::catalog::registered_tool_definitions(true, true);
    for name in ["build_import_xml", "post_import"] {
        let description = definitions
            .as_array()
            .and_then(|tools| tools.iter().find(|tool| tool["name"] == name))
            .and_then(|tool| tool["description"].as_str())
            .expect(name);
        for stated in [
            NUMBERING_NOT_AUTOMATIC,
            NUMBERING_NOT_RETAIN,
            "one book, TallyPrime 7.1 Silver: by import on a Receipt and a Journal, by hand on a Receipt",
            "sent by import without a number",
            "the import's answer reported nothing altered",
            "the same through ComplyEaze Bridge's own post are not measured",
        ] {
            assert!(description.contains(stated), "{name}: {stated}");
        }
        // The statement the sub-method read replaced.
        assert!(
            !description.contains("is admitted whatever its setting"),
            "{name}"
        );
    }
}

fn marks(rows: &[(&str, &str, u64)]) -> Vec<LoadedCompanyMarks> {
    rows.iter()
        .map(|(name, guid, masters)| LoadedCompanyMarks {
            name: (*name).to_string(),
            guid: (*guid).to_string(),
            vouchers: 10,
            masters: *masters,
        })
        .collect()
}

fn admission_at(mark: u64) -> super::numbering::NumberingAdmission {
    admit_numbering(
        VoucherNumberingRead::for_test(GUID, mark, vec![("Journal".to_string(), automatic())]),
        ["Journal"],
    )
    .unwrap()
}

#[test]
fn the_binding_snapshot_must_hold_the_mark_the_numbering_was_read_under() {
    let admission = admission_at(7);
    let other = ("Other Lab", "another-guid", 99);
    assert_eq!(
        admission.admit_at_binding(&marks(&[(NAME, GUID, 7), other]), GUID, NAME),
        Ok(())
    );
    for moved in [6, 8] {
        assert_eq!(
            admission.admit_at_binding(&marks(&[(NAME, GUID, moved), other]), GUID, NAME),
            Err(ApprovedImportAdmissionError::MastersMoved),
            "{moved}"
        );
    }
}

#[test]
fn a_binding_snapshot_without_one_target_row_cannot_be_compared() {
    let admission = admission_at(7);
    for rows in [
        marks(&[("Other Lab", "another-guid", 7)]),
        marks(&[(NAME, GUID, 7), (NAME, GUID, 7)]),
        // The GUID matches, the name does not: not the target.
        marks(&[("Renamed Lab", GUID, 7)]),
    ] {
        assert_eq!(
            admission.admit_at_binding(&rows, GUID, NAME),
            Err(ApprovedImportAdmissionError::MastersUnconfirmed)
        );
    }
}

#[test]
fn an_admission_of_another_company_cannot_be_compared() {
    let admission = admission_at(7);
    let rows = marks(&[("Other Lab", "another-guid", 7)]);
    assert_eq!(
        admission.admit_at_binding(&rows, "another-guid", "Other Lab"),
        Err(ApprovedImportAdmissionError::MastersUnconfirmed)
    );
}
