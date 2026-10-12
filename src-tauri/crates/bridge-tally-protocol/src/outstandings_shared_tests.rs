use super::*;

#[test]
fn company_book_extent_template_has_only_the_verified_fetch_list() {
    let xml = render_company_book_extent("Synthetic & Company");
    assert!(xml.contains("<ID>BridgeCompanyBookExtentV1</ID>"));
    assert!(xml.contains("ALTVCHID"));
    assert!(xml.contains("ALTMSTID"));
    assert!(xml.contains("Synthetic &amp; Company"));
    assert!(!xml.contains("CompanyNumber"));
    assert!(!xml.contains("<COMPUTE>"));
    assert!(!xml.contains("$$NumItems"));

    let v2 = render_company_book_extent_v2("Synthetic & Company");
    assert!(v2.contains("<ID>BridgeCompanyBookExtentV2</ID>"));
    assert!(v2.contains(
        "<FETCH>Name, GUID, CompanyNumber, BooksFrom, LastVoucherDate, ALTVCHID, ALTMSTID</FETCH>"
    ));
    assert!(v2.contains("ISMODIFY=\"No\""));
}

fn synthetic_extent(
    master_alter_id_high_water: Option<MasterAlterIdHighWater>,
) -> CompanyBookExtent {
    let company = PinnedCompany::verified(
        ValidatedCompanyName::new("Synthetic Company".to_string())
            .expect("synthetic name validates"),
        "synthetic-guid".to_string(),
    )
    .expect("synthetic identity verifies");
    CompanyBookExtent::new(
        company,
        TallyDate::parse("20240101".to_string()).expect("synthetic BooksFrom"),
        TallyDate::parse("20260101".to_string()).expect("synthetic LastVoucherDate"),
        Some(VoucherAlterIdHighWater::parse("1").expect("synthetic ALTVCHID")),
        master_alter_id_high_water,
    )
}

/// The production bracket's strict check, isolated from any particular
/// caller: an extent whose `ALTMSTID` witness is absent must be refused
/// with the typed `MasterWitnessAbsent` error, while a witness-bearing
/// extent passes through unchanged. `connector.rs` and `connection.rs`
/// each call this at the point where their own bracket forms/compares a
/// production extent -- see their `..._fails_closed_when_altmstid_is_absent`
/// tests for that end-to-end wiring.
#[test]
fn require_master_witness_fails_closed_only_when_the_witness_is_absent() {
    assert_eq!(
        require_master_witness(&synthetic_extent(None)),
        Err(OutstandingsError::MasterWitnessAbsent)
    );
    assert_eq!(
        require_master_witness(&synthetic_extent(Some(
            MasterAlterIdHighWater::parse("1").expect("synthetic ALTMSTID")
        ))),
        Ok(())
    );
}

// `parse_credit_period` is shipped: the agent `vouchers` read calls it for
// every bill allocation's `BILLCREDITPERIOD`. These cases were first written
// against the legacy voucher-scan parser's wrapper of it; they assert the
// shared function itself, on its own typed error code, so no feature gate
// decides whether they run.

#[test]
fn credit_period_accepts_verified_units() {
    assert_eq!(
        parse_credit_period("45 Days").unwrap(),
        CreditPeriod::Days(45)
    );
    assert_eq!(parse_credit_period("1 Day").unwrap(), CreditPeriod::Days(1));
    assert_eq!(
        parse_credit_period("3 Weeks").unwrap(),
        CreditPeriod::Weeks(3)
    );
    assert_eq!(
        parse_credit_period("1 Week").unwrap(),
        CreditPeriod::Weeks(1)
    );
    assert_eq!(
        parse_credit_period("2 Months").unwrap(),
        CreditPeriod::Months(2)
    );
    assert_eq!(
        parse_credit_period("1 Month").unwrap(),
        CreditPeriod::Months(1)
    );
}

#[test]
fn blank_credit_period_is_zero_days() {
    assert_eq!(parse_credit_period(" ").unwrap(), CreditPeriod::Days(0));
    assert_eq!(parse_credit_period("").unwrap(), CreditPeriod::Days(0));
}

#[test]
fn credit_period_keeps_the_measured_ceiling_for_days_only() {
    // Licensed TallyPrime 7.1 read-back on 2026-08-23 retained all four
    // values. Only the day-unit ceiling is measured: 10000 Days comes
    // back empty from Tally rather than as an over-limit period.
    assert_eq!(
        parse_credit_period("9999 Days").unwrap(),
        CreditPeriod::Days(9999)
    );
    assert_eq!(
        parse_credit_period("3650 Days").unwrap(),
        CreditPeriod::Days(3650)
    );
    assert_eq!(
        parse_credit_period("100 Months").unwrap(),
        CreditPeriod::Months(100)
    );
    assert_eq!(
        parse_credit_period("1000 Months").unwrap(),
        CreditPeriod::Months(1000)
    );
    assert_eq!(
        parse_credit_period("10000 Days"),
        Err(BILL_CREDIT_PERIOD_INVALID)
    );
    assert_eq!(
        parse_credit_period("10000 Day"),
        Err(BILL_CREDIT_PERIOD_INVALID)
    );
}

#[test]
fn credit_period_rejects_an_unknown_unit_instead_of_defaulting() {
    for text in [
        "2 Fortnights",
        "30",
        "Days",
        " Days",
        "x Days",
        "-1 Days",
        "+1 Days",
        "1.5 Days",
    ] {
        assert_eq!(
            parse_credit_period(text),
            Err(BILL_CREDIT_PERIOD_INVALID),
            "{text:?} is not `<digits> Days|Weeks|Months`"
        );
    }
}

// `accepts_boundary` is shipped: every native report window, the agent read
// admission and the approved-import window ask it whether a TallyPrime
// Education endpoint honours a date. Day 31 and the day-3 gap are measured
// facts (guide: Education honours a boundary only on the 1st, 2nd or 31st).

#[test]
fn education_accepts_only_the_verified_boundary_days() {
    let education = DateBoundaryProfile::EducationRestricted;
    for date in ["20260401", "20260402", "20260331"] {
        assert!(
            education.accepts_boundary(&TallyDate::parse(date).unwrap()),
            "{date}: day 1, 2 and 31 are verified"
        );
    }
    for date in ["20260403", "20260415", "20260428", "20260429", "20260430"] {
        assert!(
            !education.accepts_boundary(&TallyDate::parse(date).unwrap()),
            "{date}: not a verified Education boundary"
        );
    }
}

#[test]
fn a_licensed_or_unknown_mode_accepts_every_boundary_day() {
    let licensed = DateBoundaryProfile::ModeAgnostic;
    for date in ["20260401", "20260403", "20260415", "20260430"] {
        assert!(licensed.accepts_boundary(&TallyDate::parse(date).unwrap()));
    }
}
