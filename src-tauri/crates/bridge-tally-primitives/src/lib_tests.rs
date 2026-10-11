use super::{DateSpan, ExactDecimal, TallyDate, MAX_EXACT_DECIMAL_BYTES};

#[test]
fn adding_calendar_months_clamps_to_the_target_month_end() {
    assert_eq!(
        TallyDate::parse("20260131")
            .unwrap()
            .add_months_clamped(1)
            .unwrap()
            .as_str(),
        "20260228"
    );
    assert_eq!(
        TallyDate::parse("20240131")
            .unwrap()
            .add_months_clamped(1)
            .unwrap()
            .as_str(),
        "20240229"
    );
}

#[test]
fn adding_days_is_constant_time_calendar_arithmetic() {
    assert_eq!(
        TallyDate::parse("20260228")
            .unwrap()
            .add_days(1)
            .unwrap()
            .as_str(),
        "20260301"
    );
    assert!(TallyDate::parse("20260101")
        .unwrap()
        .add_days(u32::MAX)
        .is_err());
}

fn decimal(text: &str) -> ExactDecimal {
    ExactDecimal::parse(text).unwrap()
}

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

/// A magnitude never fails and drops only a negative non-zero value's sign,
/// exactly as `abs` always did (bridge#1097).
#[test]
fn a_magnitude_is_the_value_without_its_sign() {
    let long = format!(
        "-{}.{}",
        "9".repeat(200),
        "5".repeat(MAX_EXACT_DECIMAL_BYTES - 202)
    );
    assert_eq!(long.len(), MAX_EXACT_DECIMAL_BYTES);
    for (value, expected) in [
        ("-12.50", "12.50"),
        ("12.50", "12.50"),
        ("0", "0"),
        ("0.00", "0.00"),
        // Zero is not negative, so its written sign stays, as with `abs`.
        ("-0.00", "-0.00"),
        (long.as_str(), &long[1..]),
    ] {
        let magnitude = decimal(value).magnitude();
        assert_eq!(magnitude.as_str(), expected, "{value}");
        assert_eq!(decimal(value).abs().unwrap(), magnitude, "{value}");
    }
}

/// A span exists only for an ordered pair, and counts whole days between
/// them across month, year and leap-day boundaries (bridge#1097).
#[test]
fn a_date_span_counts_the_days_of_an_ordered_pair_only() {
    for (from, to, days) in [
        ("20260415", "20260415", 0),
        ("20260131", "20260201", 1),
        ("20251231", "20260101", 1),
        ("20240228", "20240301", 2),
        ("20240229", "20240301", 1),
        ("20230228", "20230301", 1),
        ("20000228", "20000301", 2),
        ("19000228", "19000301", 1),
        ("00010101", "99991231", 3_652_058),
    ] {
        assert_eq!(
            DateSpan::new(&date(from), &date(to)).map(|span| span.days()),
            Some(days),
            "{from} to {to}"
        );
    }
    assert_eq!(DateSpan::new(&date("20260202"), &date("20260201")), None);
    assert_eq!(DateSpan::new(&date("99991231"), &date("00010101")), None);
}
