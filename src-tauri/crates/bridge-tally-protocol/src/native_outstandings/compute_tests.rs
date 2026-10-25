use crate::TALLY_SANITIZED_ROOT_MARKER;

use super::*;

#[test]
fn reserved_root_policy_matches_canonical_window_for_marker_carrying_parents() {
    // Only the marked form is the root (`TALLY_PROTOCOL_REFERENCE.md`
    // §1.1(d)); a bare `Primary` names a group.
    for root in [
        format!("{TALLY_SANITIZED_ROOT_MARKER} Primary"),
        format!(" {TALLY_SANITIZED_ROOT_MARKER}  primary "),
    ] {
        let groups = [TallyNamedMaster {
            name: "Sundry Debtors".to_string(),
            parent: crate::PartyLedgerMasterFieldObservation::Returned(root),
            reserved_name: Some("Sundry Debtors".to_string()),
        }];
        let ledgers = [LedgerSnapshotEntry {
            name: "Synthetic Ledger".to_string(),
            parent: Some("Sundry Debtors".to_string()),
            closing_balance: Some(ExactDecimal::zero()),
            opening_balance: ExactDecimal::zero(),
            opening_balance_observed: Some(ExactDecimal::zero()),
            bill_wise_on: false,
            currency_name: None,
        }];
        compute_residuals(&[], &[], &ledgers, NativeGroupSnapshot::Complete(&groups))
            .expect("shared reserved-root forms must terminate group ancestry");
    }

    let groups = [TallyNamedMaster {
        name: "Sundry Debtors".to_string(),
        parent: crate::PartyLedgerMasterFieldObservation::Returned(format!(
            "{TALLY_SANITIZED_ROOT_MARKER} Primary"
        )),
        reserved_name: Some("Sundry Debtors".to_string()),
    }];
    for parent in [
        format!("{TALLY_SANITIZED_ROOT_MARKER} Resave"),
        format!("{TALLY_SANITIZED_ROOT_MARKER}{TALLY_SANITIZED_ROOT_MARKER} Primary"),
    ] {
        let ledgers = [LedgerSnapshotEntry {
            name: "Synthetic Ledger".to_string(),
            parent: Some(parent),
            closing_balance: Some(ExactDecimal::zero()),
            opening_balance: ExactDecimal::zero(),
            opening_balance_observed: Some(ExactDecimal::zero()),
            bill_wise_on: false,
            currency_name: None,
        }];

        assert!(matches!(
            compute_residuals(&[], &[], &ledgers, NativeGroupSnapshot::Complete(&groups)),
            Err(NativeOutstandingsError::InvalidResponse(
                "ledger_group_parent_unresolved"
            ))
        ));
    }
}

/// The span a bill's age is now read from gives the same days as this
/// module's own day count, over three years of start dates and spans across
/// month ends, a leap day and whole years, and neither exists in reverse
/// (bridge#1097).
#[test]
fn a_date_span_agrees_with_the_bills_day_count() {
    let start = TallyDate::parse("20231201").unwrap();
    for offset in 0..(3 * 366) {
        let from = start.add_days(offset).unwrap();
        for days in [0, 1, 27, 28, 29, 30, 31, 59, 365, 366, 1_461] {
            let to = from.add_days(days).unwrap();
            let span = bridge_tally_primitives::DateSpan::new(&from, &to)
                .unwrap_or_else(|| panic!("{} to {}", from.as_str(), to.as_str()));
            assert_eq!(span.days(), days, "{} to {}", from.as_str(), to.as_str());
            assert_eq!(age_in_days(&from, &to), Ok(days), "{}", from.as_str());
            if days > 0 {
                assert_eq!(bridge_tally_primitives::DateSpan::new(&to, &from), None);
                assert_eq!(
                    age_in_days(&to, &from),
                    Err(NativeOutstandingsError::InvalidDate(
                        "native_date_after_as_of"
                    ))
                );
            }
        }
    }
}
