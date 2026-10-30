//! The stock summary's refusals under this read's codes, and each code as its display.
use super::*;

#[test]
fn the_stock_summary_size_refusal_keeps_its_size_under_this_reads_code() {
    let error = own(admit_stock_summary_size(875).unwrap_err());
    match &error {
        StockByGodownReadError::TooLarge {
            master_alter_id: 875,
            limit_master_alter_id: 874,
            ..
        } => {}
        other => panic!("{other:?}"),
    }
    assert_eq!(error.safe_code(), "stock_by_godown_too_large");
}

#[test]
fn a_premise_the_answers_break_is_refused_under_this_reads_code() {
    let error = own(check_stock_premise(5, 4, 10, 1_000).unwrap_err());
    assert!(matches!(
        error,
        StockByGodownReadError::PremiseViolated("stock_rows_exceed_master_mark")
    ));
}

#[test]
fn each_refusal_has_its_own_code_and_the_code_is_its_display() {
    for error in [
        StockByGodownReadError::EducationUnqualified,
        StockByGodownReadError::AsOfBeforeBooks,
        StockByGodownReadError::AsOfInFuture,
        StockByGodownReadError::PeriodNotHonoured,
        StockByGodownReadError::TooLarge {
            master_alter_id: 1,
            estimated_bytes: 1,
            limit_bytes: 1,
            limit_master_alter_id: 1,
        },
        StockByGodownReadError::PremiseViolated("x"),
    ] {
        assert_eq!(error.to_string(), error.safe_code());
    }
}
