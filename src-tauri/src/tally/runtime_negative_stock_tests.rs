//! The size admission and the premise check of the Negative Stock read: the stock
//! summary's own, with this read's refusals.
use super::*;

#[test]
fn a_mark_of_874_is_admitted_and_875_is_refused_with_its_size() {
    assert_eq!(admit_size(874).unwrap(), 874 * 18_296);
    match admit_size(875) {
        Err(NegativeStockReadError::TooLarge {
            master_alter_id: 875,
            estimated_bytes: 16_009_000,
            limit_bytes: 16_000_000,
            limit_master_alter_id: 874,
        }) => {}
        other => panic!("{other:?}"),
    }
}

#[test]
fn rows_over_the_mark_or_bytes_over_the_estimate_break_the_premise_and_the_edge_does_not() {
    assert!(check_premise(4, 4, 1_000, 1_000).is_ok());
    assert!(matches!(
        check_premise(5, 4, 10, 1_000),
        Err(NegativeStockReadError::PremiseViolated(
            "stock_rows_exceed_master_mark"
        ))
    ));
    assert!(matches!(
        check_premise(1, 4, 1_001, 1_000),
        Err(NegativeStockReadError::PremiseViolated(
            "stock_response_over_admitted_bytes"
        ))
    ));
}

#[test]
fn each_refusal_has_its_own_code_and_the_code_is_its_display() {
    for error in [
        NegativeStockReadError::EducationUnqualified,
        NegativeStockReadError::BeforeBooks,
        NegativeStockReadError::TooLarge {
            master_alter_id: 1,
            estimated_bytes: 1,
            limit_bytes: 1,
            limit_master_alter_id: 1,
        },
        NegativeStockReadError::PremiseViolated("x"),
    ] {
        assert_eq!(error.to_string(), error.safe_code());
    }
}
