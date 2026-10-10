use super::*;

fn rate(text: Option<&str>) -> RawGstRateDetails {
    RawGstRateDetails {
        duty_head: Some("CGST".to_string()),
        valuation_type: Some("Based on Value".to_string()),
        rate: text.map(str::to_string),
        repeated_field: false,
    }
}

fn state(rates: Vec<RawGstRateDetails>) -> RawGstStateDetails {
    RawGstStateDetails {
        state_name: Some("\u{fffd}#4; Any".to_string()),
        rates,
        slab_rates_present: false,
        repeated_field: false,
    }
}

fn entry(date: Option<&str>, rates: Vec<RawGstRateDetails>) -> RawGstDetailsEntry {
    RawGstDetailsEntry {
        applicable_from: date.map(str::to_string),
        taxability: Some("Taxable".to_string()),
        source: Some("Specify Details Here".to_string()),
        itc_eligible: Some("Yes".to_string()),
        states: vec![state(rates)],
        repeated_field: false,
    }
}

fn defect(raw: Vec<RawGstDetailsEntry>) -> Option<GstDetailsDefect> {
    match GstDetailsObservation::from_raw(raw) {
        GstDetailsObservation::Unreadable { defect } => Some(defect),
        _ => None,
    }
}

fn only_rate(text: Option<&str>) -> Result<GstRate, GstDetailsDefect> {
    match GstDetailsObservation::from_raw(vec![entry(Some("20260401"), vec![rate(text)])]) {
        GstDetailsObservation::Entries { entries } => {
            Ok(entries[0].states[0].rates[0].rate.clone())
        }
        GstDetailsObservation::Unreadable { defect } => Err(defect),
        GstDetailsObservation::NotObserved => panic!("from_raw never returns NotObserved"),
    }
}

#[test]
fn absent_empty_and_zero_are_three_different_rates() {
    assert_eq!(only_rate(None), Ok(GstRate::Absent));
    assert_eq!(only_rate(Some("")), Ok(GstRate::Empty));
    assert_eq!(only_rate(Some("   ")), Ok(GstRate::Empty));
    assert_eq!(only_rate(Some(" 0")), Ok(GstRate::Zero));
    assert_eq!(only_rate(Some("0.00")), Ok(GstRate::Zero));
    assert_eq!(only_rate(Some("0")), Ok(GstRate::Zero));
}

#[test]
fn a_value_keeps_its_raw_text_with_leading_spaces_and_a_trimmed_decimal() {
    assert_eq!(
        only_rate(Some(" 2.50")),
        Ok(GstRate::Value {
            raw: " 2.50".to_string(),
            decimal: "2.50".to_string()
        })
    );
    assert_eq!(
        only_rate(Some(" 18")),
        Ok(GstRate::Value {
            raw: " 18".to_string(),
            decimal: "18".to_string()
        })
    );
    assert_eq!(
        only_rate(Some("0.05")),
        Ok(GstRate::Value {
            raw: "0.05".to_string(),
            decimal: "0.05".to_string()
        }),
        "a fraction that is not zero is a value"
    );
}

#[test]
fn a_rate_that_is_not_a_plain_decimal_is_unreadable_never_zero_or_absent() {
    for bad in [
        "-1", "+5", "1e2", "2,50", "1.2.3", ".5", "5.", "abc", "1 2", "-0",
    ] {
        assert_eq!(
            only_rate(Some(bad)),
            Err(GstDetailsDefect::RateNotDecimal),
            "{bad:?}"
        );
    }
}

#[test]
fn entries_are_sorted_by_date_whatever_the_document_order() {
    let observation = GstDetailsObservation::from_raw(vec![
        entry(Some("20260401"), vec![rate(Some(" 5"))]),
        entry(Some("20250401"), vec![rate(Some(" 18"))]),
    ]);
    let GstDetailsObservation::Entries { entries } = observation else {
        panic!("readable");
    };
    let dates: Vec<_> = entries
        .iter()
        .map(|e| e.applicable_from.as_deref().unwrap())
        .collect();
    assert_eq!(dates, ["20250401", "20260401"]);
}

#[test]
fn two_entries_on_one_date_are_one_if_identical_and_a_defect_if_not() {
    let same = GstDetailsObservation::from_raw(vec![
        entry(Some("20260401"), vec![rate(Some(" 5"))]),
        entry(Some("20260401"), vec![rate(Some(" 5"))]),
    ]);
    let GstDetailsObservation::Entries { entries } = same else {
        panic!("identical entries are readable");
    };
    assert_eq!(entries.len(), 1, "strictly increasing dates");
    assert_eq!(
        defect(vec![
            entry(Some("20260401"), vec![rate(Some(" 5"))]),
            entry(Some("20260401"), vec![rate(Some(" 18"))]),
        ]),
        Some(GstDetailsDefect::ConflictingEntriesOnOneDate)
    );
}

#[test]
fn an_empty_placeholder_is_an_observed_empty_list_not_an_unobserved_one() {
    assert_eq!(
        GstDetailsObservation::from_raw(vec![RawGstDetailsEntry::default()]),
        GstDetailsObservation::Entries { entries: vec![] }
    );
    assert_ne!(
        GstDetailsObservation::from_raw(vec![]),
        GstDetailsObservation::NotObserved
    );
    assert_eq!(
        GstDetailsObservation::default(),
        GstDetailsObservation::NotObserved
    );
}

#[test]
fn each_defect_has_its_own_reason() {
    assert_eq!(
        defect(vec![entry(None, vec![])]),
        Some(GstDetailsDefect::EntryWithoutDate)
    );
    for bad in ["20261301", "20260231", "2026-04-01", ""] {
        assert_eq!(
            defect(vec![entry(Some(bad), vec![])]),
            Some(GstDetailsDefect::DateInvalid),
            "{bad:?}"
        );
    }
    let mut repeated = entry(Some("20260401"), vec![]);
    repeated.repeated_field = true;
    assert_eq!(
        defect(vec![repeated]),
        Some(GstDetailsDefect::EntryRepeatsAField)
    );
    let mut repeated_rate = rate(Some(" 5"));
    repeated_rate.repeated_field = true;
    assert_eq!(
        defect(vec![entry(Some("20260401"), vec![repeated_rate])]),
        Some(GstDetailsDefect::EntryRepeatsAField),
        "a repeat inside a rate row fails the ledger too"
    );
    let mut repeated_state = state(vec![]);
    repeated_state.repeated_field = true;
    let mut with_state = entry(Some("20260401"), vec![]);
    with_state.states = vec![repeated_state];
    assert_eq!(
        defect(vec![with_state]),
        Some(GstDetailsDefect::EntryRepeatsAField)
    );
}

#[test]
fn input_tax_credit_is_yes_or_no_and_nothing_else() {
    let itc = |text: Option<&str>| {
        let mut e = entry(Some("20260401"), vec![]);
        e.itc_eligible = text.map(str::to_string);
        match GstDetailsObservation::from_raw(vec![e]) {
            GstDetailsObservation::Entries { entries } => Ok(entries[0].itc_eligible),
            GstDetailsObservation::Unreadable { defect } => Err(defect),
            GstDetailsObservation::NotObserved => unreachable!(),
        }
    };
    assert_eq!(itc(Some("Yes")), Ok(Some(true)));
    assert_eq!(itc(Some("No")), Ok(Some(false)));
    assert_eq!(itc(None), Ok(None));
    for bad in ["", "yes", "Maybe", "1", "True"] {
        assert_eq!(
            itc(Some(bad)),
            Err(GstDetailsDefect::ItcNotYesNo),
            "{bad:?}"
        );
    }
}

#[test]
fn text_fields_are_kept_verbatim_with_their_spaces_and_markers() {
    let mut e = entry(Some("20260401"), vec![rate(Some(" 5"))]);
    e.taxability = Some(" Taxable ".to_string());
    e.source = Some(" As per Company/Group".to_string());
    e.states[0].state_name = Some("\u{fffd}#4; Any".to_string());
    e.states[0].rates[0].duty_head = Some(" SGST/UTGST ".to_string());
    e.states[0].rates[0].valuation_type = Some("\u{fffd}#4; Not Applicable".to_string());
    let GstDetailsObservation::Entries { entries } = GstDetailsObservation::from_raw(vec![e])
    else {
        panic!("readable");
    };
    let got = &entries[0];
    assert_eq!(got.taxability.as_deref(), Some(" Taxable "));
    assert_eq!(got.source.as_deref(), Some(" As per Company/Group"));
    assert_eq!(
        got.states[0].state_name.as_deref(),
        Some("\u{fffd}#4; Any"),
        "leading marker and the space after it"
    );
    assert_eq!(
        got.states[0].rates[0].duty_head.as_deref(),
        Some(" SGST/UTGST ")
    );
    assert_eq!(
        got.states[0].rates[0].valuation_type.as_deref(),
        Some("\u{fffd}#4; Not Applicable")
    );
}

#[test]
fn the_observation_serialises_with_its_tag() {
    let json = serde_json::to_value(GstDetailsObservation::NotObserved).unwrap();
    assert_eq!(json, serde_json::json!({"observation": "not_observed"}));
    let json = serde_json::to_value(GstDetailsObservation::Unreadable {
        defect: GstDetailsDefect::RateNotDecimal,
    })
    .unwrap();
    assert_eq!(
        json,
        serde_json::json!({"observation": "unreadable", "defect": "rate_not_decimal"})
    );
}
