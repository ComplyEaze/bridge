use super::{DateSpan, ExactDecimal, TallyDate, MAX_EXACT_DECIMAL_BYTES};

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

#[test]
fn a_date_is_shown_as_tally_shows_it() {
    for (date, shown) in [
        ("20260914", "14-Sep-2026"),
        ("20260401", "1-Apr-2026"),
        ("20241231", "31-Dec-2024"),
        ("20240229", "29-Feb-2024"),
    ] {
        assert_eq!(TallyDate::parse(date).unwrap().shown(), shown);
    }
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
