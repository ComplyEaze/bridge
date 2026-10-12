//! The voucher-numbering admission (bridge#724) without Tally: which readings
//! admit, which refuse and how each refusal is named, and the mark compare the
//! queue makes before the POST.
use super::location::LoadedCompanyMarks;
use super::numbering::{admit_numbering, NumberingRefusal, NUMBERING_NOT_AUTOMATIC};
use crate::tally::approved_import::ApprovedImportAdmissionError;
use crate::tally::runtime::VoucherNumberingRead;
use bridge_tally_protocol::native_voucher_type_numbering::{
    parse_voucher_type_numbering, DuplicateSetting, SeriesNumberingMethod, VoucherTypeNumbering,
};
use serde_json::json;

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const NAME: &str = "WR2 Unicode Lab";

fn observed(method: SeriesNumberingMethod, duplicates: DuplicateSetting) -> VoucherTypeNumbering {
    VoucherTypeNumbering::Observed { method, duplicates }
}

fn automatic() -> VoucherTypeNumbering {
    observed(
        SeriesNumberingMethod::Automatic,
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
        let read = read_with_journal(observed(SeriesNumberingMethod::Automatic, duplicates));
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
        let result = admit_numbering(read_with_journal(numbering.clone()), ["Journal"]);
        assert_eq!(
            refused(result),
            vec![("Journal".to_string(), expected)],
            "{numbering:?}"
        );
    }
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

/// The committed live capture (24 voucher types of a synthetic lab company):
/// Payment reads Manual with duplicates prevented and Receipt Manual with them
/// allowed; Journal and Contra read Automatic.
#[test]
fn the_captured_book_refuses_payment_and_receipt_and_admits_journal_and_contra() {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_type_numbering_series_live.utf16le.xml"
    );
    let text = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let rows = parse_voucher_type_numbering(text.trim_start_matches('\u{feff}')).unwrap();
    let read = || {
        VoucherNumberingRead::for_test(
            GUID,
            7,
            rows.iter()
                .map(|row| (row.name.clone(), row.numbering.clone()))
                .collect(),
        )
    };
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
