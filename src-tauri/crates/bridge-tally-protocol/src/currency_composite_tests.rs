//! Every composite here is read from a committed capture, never typed.
use super::*;
use crate::native_outstandings::{parse_company_currency_name, parse_currency_master_list};
use std::collections::BTreeMap;

/// A native Trial Balance of the synthetic several-currency lab book
/// (licensed 7.1), whose amount fields hold composites.
const FOREX_TRIAL_BALANCE: &[u8] =
    include_bytes!("../tests/fixtures/trial_balance_currency_forex_live.utf16le.xml");

fn forex_trial_balance() -> String {
    String::from_utf16(
        &FOREX_TRIAL_BALANCE
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// Every amount-field value in the capture that holds an ` @ `.
fn captured_composites() -> Vec<String> {
    let capture = forex_trial_balance();
    ["TBALOPENING", "DEBITTOTALS", "CREDITTOTALS", "TBALCLOSING"]
        .iter()
        .flat_map(|tag| {
            capture
                .split(&format!("<{tag} "))
                .skip(1)
                .filter_map(|tail| {
                    let text = &tail[tail.find('>')? + 1..tail.find(&format!("</{tag}>"))?];
                    text.contains(" @ ").then(|| text.to_string())
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn every_captured_composite_is_classified() {
    let composites = captured_composites();
    assert_eq!(composites.len(), 11, "{composites:?}");
    for composite in &composites {
        assert!(is_currency_composite(composite), "{composite}");
    }
}

#[test]
fn the_captured_empty_rate_composite_is_classified() {
    let empty_rate = captured_composites()
        .into_iter()
        .find(|value| value.contains(" /$"))
        .expect("the captured empty-rate composite");
    assert!(is_currency_composite(&empty_rate), "{empty_rate}");
}

#[test]
fn a_damaged_composite_is_not_one() {
    let empty_rate = captured_composites()
        .into_iter()
        .find(|value| value.contains(" /$"))
        .unwrap();
    let rated = captured_composites()
        .into_iter()
        .find(|value| !value.contains(" /$"))
        .expect("a captured composite with a rate");
    let cut = &empty_rate[..empty_rate.find(" = ").unwrap()];
    for damaged in [
        cut.to_string(),
        format!("{empty_rate} @ $ 1/$"),
        format!("{rated} = I\u{20b9} 1.00"),
        empty_rate.replace("0.00", "\u{0660}.00"),
        empty_rate.replacen("$ ", "", 1),
        rated.replacen(".00", ".", 1),
        rated.replacen(" @ ", " @", 1),
    ] {
        assert!(!is_currency_composite(&damaged), "{damaged}");
    }
}

#[test]
fn a_plain_amount_is_not_a_composite() {
    for plain in [
        "-4250.00",
        "4250.00",
        "0.00",
        "",
        "$ 100.00",
        "-I\u{20b9} 8600.00",
    ] {
        assert!(!is_currency_composite(plain), "{plain:?}");
    }
}

/// A `vouchers` read of the same book (#674): the party entry, its bill
/// allocation and the sales entry each hold a composite.
#[test]
fn every_composite_in_the_captured_voucher_is_classified() {
    let bytes: &[u8] =
        include_bytes!("../tests/fixtures/agent/vouchers-forex-composite-20260915.utf16le.xml");
    let capture = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let amounts: Vec<&str> = capture
        .split("<AMOUNT")
        .skip(1)
        .filter_map(|tail| Some(&tail[tail.find('>')? + 1..tail.find("</AMOUNT>")?]))
        .collect();
    assert_eq!(amounts.len(), 3, "{amounts:?}");
    for amount in amounts {
        assert!(is_currency_composite(amount), "{amount}");
    }
}

/// The second capture of that book's day (#674): the same Sales voucher and a
/// Receipt whose party entry, its bill allocation and cash entry each hold a
/// composite at another rate
/// (fixtures/agent/vouchers-forex-bill-allocation-20260915.PROVENANCE.md).
#[test]
fn every_composite_in_the_bill_allocation_capture_is_classified() {
    let bytes: &[u8] = include_bytes!(
        "../tests/fixtures/agent/vouchers-forex-bill-allocation-20260915.utf16le.xml"
    );
    let capture = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let amounts: Vec<&str> = capture
        .split("<AMOUNT")
        .skip(1)
        .filter_map(|tail| Some(&tail[tail.find('>')? + 1..tail.find("</AMOUNT>")?]))
        .collect();
    assert_eq!(amounts.len(), 6, "{amounts:?}");
    for amount in &amounts {
        assert!(is_currency_composite(amount), "{amount}");
    }
    assert_eq!(
        amounts
            .iter()
            .filter(|amount| amount.contains("@ I₹ 88/$"))
            .count(),
        3,
        "{amounts:?}"
    );
}

/// A captured negative composite with a rate, on which each rule that ties a
/// composite's parts together is broken alone below. Every captured composite
/// passes all of them (above).
fn captured_rated_negative() -> String {
    let captured = captured_composites()
        .into_iter()
        .find(|value| value.starts_with("-$ ") && !value.contains(" /$"))
        .expect("a captured negative composite with a rate");
    assert!(is_currency_composite(&captured), "{captured}");
    captured
}

#[test]
fn a_rate_not_quoted_in_the_base_symbol_is_not_a_composite() {
    let captured = captured_rated_negative();
    let rate_start = captured.find(" @ ").unwrap() + " @ ".len();
    let other_base = format!(
        "{}\u{20ac}{}",
        &captured[..rate_start],
        &captured[rate_start + "I\u{20b9}".len()..]
    );
    assert!(other_base.contains("@ \u{20ac} "), "{other_base}");
    assert!(!is_currency_composite(&other_base), "{other_base}");
}

#[test]
fn a_rate_not_per_the_foreign_symbol_is_not_a_composite() {
    let captured = captured_rated_negative();
    let other_per = captured.replacen("/$", "/\u{20ac}", 1);
    assert_ne!(other_per, captured);
    assert!(!is_currency_composite(&other_per), "{other_per}");
}

/// Signs are the caller's rule (a voucher entry's), not the shape's: a ledger
/// balance can pair a foreign and a base amount of opposite signs.
#[test]
fn amounts_of_different_signs_are_still_a_composite_shape() {
    let captured = captured_rated_negative();
    let base_start = captured.find(" = ").unwrap() + " = ".len();
    assert!(captured[base_start..].starts_with('-'));
    let unsigned_base = format!("{}{}", &captured[..base_start], &captured[base_start + 1..]);
    assert!(is_currency_composite(&unsigned_base), "{unsigned_base}");
}

#[test]
fn a_composite_in_one_currency_is_not_one() {
    let captured = captured_rated_negative();
    // The foreign amount and the rate's unit become the base symbol.
    let one_currency = captured
        .replacen("-$ ", "-I\u{20b9} ", 1)
        .replacen("/$", "/I\u{20b9}", 1);
    assert_ne!(one_currency, captured);
    assert!(one_currency.starts_with("-I\u{20b9} "), "{one_currency}");
    assert!(!is_currency_composite(&one_currency), "{one_currency}");
}

// ---- parse_currency_composite (#683), on the committed captures only --------

/// Every capture that holds a composite, by file name, as committed.
const CAPTURES: [(&str, &[u8]); 9] = [
    (
        "balance_snapshot_forex_live",
        include_bytes!("../tests/fixtures/balance_snapshot_forex_live.utf16le.xml"),
    ),
    (
        "balance_snapshot_forex_post_receipt_live",
        include_bytes!("../tests/fixtures/balance_snapshot_forex_post_receipt_live.utf16le.xml"),
    ),
    (
        "compliance_master_forex_live",
        include_bytes!("../tests/fixtures/compliance_master_forex_live.utf16le.xml"),
    ),
    (
        "ledgers_currency_forex_live",
        include_bytes!("../tests/fixtures/ledgers_currency_forex_live.utf16le.xml"),
    ),
    (
        "ledgers_forex_composite_live",
        include_bytes!("../tests/fixtures/ledgers_forex_composite_live.utf16le.xml"),
    ),
    (
        "trial_balance_currency_forex_live",
        include_bytes!("../tests/fixtures/trial_balance_currency_forex_live.utf16le.xml"),
    ),
    (
        "trial_balance_forex_live",
        include_bytes!("../tests/fixtures/trial_balance_forex_live.utf16le.xml"),
    ),
    (
        "vouchers-forex-bill-allocation-20260915",
        include_bytes!(
            "../tests/fixtures/agent/vouchers-forex-bill-allocation-20260915.utf16le.xml"
        ),
    ),
    (
        "vouchers-forex-composite-20260915",
        include_bytes!("../tests/fixtures/agent/vouchers-forex-composite-20260915.utf16le.xml"),
    ),
];

const COMPANY_CURRENCY_LIVE: &[u8] =
    include_bytes!("../tests/fixtures/company_currencyname_live.utf16le.xml");
const CURRENCY_ORIGINALNAME_FOREX: &[u8] =
    include_bytes!("../tests/fixtures/currency_originalname_forex_live.utf16le.xml");
const FOREX_GUID: &str = "b14e9b2d-8a63-4779-804d-25d59eb787eb";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The book's base currency as production identifies it: from the captured
/// currency masters and the company's own currency name.
fn forex_base() -> BaseCurrencyName {
    let masters = parse_currency_master_list(&decode(CURRENCY_ORIGINALNAME_FOREX)).unwrap();
    let name = parse_company_currency_name(&decode(COMPANY_CURRENCY_LIVE), FOREX_GUID).unwrap();
    masters.identify_base(Some(&name)).unwrap().base().clone()
}

/// Every text node of a capture that holds a composite, with how often.
fn composites_by_text_node() -> BTreeMap<String, usize> {
    let mut found = BTreeMap::new();
    for (_, bytes) in CAPTURES {
        let capture = decode(bytes);
        for segment in capture.split('<') {
            let Some((_, text)) = segment.split_once('>') else {
                continue;
            };
            let text = text.trim();
            if text.contains(" @ ") && text.contains(" = ") {
                *found.entry(text.to_string()).or_default() += 1;
            }
        }
    }
    found
}

const CAPTURED: &str = "-$ 100.00 @ I\u{20b9} 86/$  = -I\u{20b9} 8600.00";

fn parse(text: &str) -> Result<CurrencyComposite, CompositeRefusal> {
    parse_currency_composite(text, &forex_base())
}

#[test]
fn the_captures_hold_fifteen_distinct_composites_and_all_are_read() {
    let found = composites_by_text_node();
    assert_eq!(found.len(), 15, "{found:?}");
    assert_eq!(found.values().sum::<usize>(), 53);
    let mut zero = 0;
    for composite in found.keys() {
        assert!(is_currency_composite(composite), "{composite}");
        match parse(composite).unwrap_or_else(|refusal| panic!("{composite}: {refusal:?}")) {
            CurrencyComposite::Zero { foreign_symbol } => {
                assert_eq!(foreign_symbol, "$");
                zero += 1;
            }
            CurrencyComposite::Valued(valued) => {
                assert_eq!(valued.foreign_symbol(), "$");
                // the side is the sign the text carries
                let negative = composite.starts_with('-');
                assert_eq!(valued.side() == Side::Negative, negative, "{composite}");
            }
        }
    }
    // the one composite with no value is the empty-rate zero in the trial balance
    assert_eq!(zero, 1);
}

#[test]
fn a_captured_composite_is_read_into_its_parts_and_the_base_is_the_text() {
    let CurrencyComposite::Valued(valued) = parse(CAPTURED).unwrap() else {
        panic!("a valued composite");
    };
    assert_eq!(valued.side(), Side::Negative);
    assert_eq!(valued.foreign().magnitude().as_str(), "100.00");
    assert_eq!(valued.rate().as_str(), "86");
    assert_eq!(valued.base().magnitude().as_str(), "8600.00");
    // and the positive side of the same capture
    let CurrencyComposite::Valued(positive) =
        parse("$ 100.00 @ I\u{20b9} 86/$  = I\u{20b9} 8600.00").unwrap()
    else {
        panic!("a valued composite");
    };
    assert_eq!(positive.side(), Side::Positive);
    assert_eq!(positive.foreign().magnitude().as_str(), "100.00");
    assert_eq!(positive.rate().as_str(), "86");
    assert_eq!(positive.base().magnitude().as_str(), "8600.00");
    // the captured fractional rate: the base is Tally's own figure, kept as written
    let CurrencyComposite::Valued(fractional) =
        parse("-$ 60.00 @ I\u{20b9} 279.6667/$  = -I\u{20b9} 16780.00").unwrap()
    else {
        panic!("a valued composite");
    };
    assert_eq!(fractional.rate().as_str(), "279.6667");
    assert_eq!(fractional.base().magnitude().as_str(), "16780.00");
}

/// A measurement, not a rule: on every captured composite (one currency pair,
/// `$` against the rupee, release 7.1) the quoted rate is the base over the
/// foreign amount, rounded to four decimal places (13 of the 14 are exact
/// integers, so the rounding itself rests on the one fractional rate, 279.6667:
/// it rules out truncation, not every other rounding). So the rate is derived
/// from the base and the base is never rebuilt from the rate: for a larger
/// amount the product can differ from the base by the foreign amount times
/// 0.00005.
#[test]
fn measured_the_rate_is_the_base_over_the_foreign_amount_to_four_places() {
    fn scaled(text: &str, places: usize) -> i128 {
        let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
        let padded = format!("{fraction:0<places$}");
        format!("{whole}{padded}").parse().unwrap()
    }
    let mut checked = 0;
    for composite in composites_by_text_node().keys() {
        let CurrencyComposite::Valued(valued) = parse(composite).unwrap() else {
            continue;
        };
        let foreign = scaled(valued.foreign().magnitude().as_str(), 2);
        let base = scaled(valued.base().magnitude().as_str(), 2);
        // base/foreign in units of 0.0001, rounded to the nearest
        let derived = (base * 10_000 * 2 + foreign) / (2 * foreign);
        assert_eq!(derived, scaled(valued.rate().as_str(), 4), "{composite}");
        checked += 1;
    }
    assert_eq!(checked, 14);
}

#[test]
fn a_composite_in_another_base_currency_is_refused() {
    let other = BaseCurrencyName::among_several_for_tests("\u{20ac}");
    assert_eq!(
        parse_currency_composite(CAPTURED, &other),
        Err(CompositeRefusal::BaseNotBookBase)
    );
}

/// One feature of a captured string changed at a time, each refused under its
/// own name.
#[test]
fn every_shape_no_capture_shows_is_refused_under_its_own_name() {
    let change = |from: &str, to: &str| {
        assert_eq!(CAPTURED.matches(from).count(), 1, "{from}");
        parse(&CAPTURED.replacen(from, to, 1))
    };
    assert_eq!(change(" @ ", " @"), Err(CompositeRefusal::NotComposite));
    // the rate quoted in another symbol, or per another foreign symbol
    assert_eq!(
        change("I\u{20b9} 86/", "\u{20ac} 86/"),
        Err(CompositeRefusal::NotComposite)
    );
    assert_eq!(
        change("86/$", "86/\u{20ac}"),
        Err(CompositeRefusal::NotComposite)
    );
    assert_eq!(change("100.00", "100"), Err(CompositeRefusal::AmountScale));
    assert_eq!(change("/$  =", "/$ ="), Err(CompositeRefusal::Spacing));
    assert_eq!(change("/$  =", "/$   ="), Err(CompositeRefusal::Spacing));
    assert_eq!(
        change("100.00", "100.0"),
        Err(CompositeRefusal::AmountScale)
    );
    assert_eq!(
        change("8600.00", "8600.000"),
        Err(CompositeRefusal::AmountScale)
    );
    assert_eq!(
        change(" 86/", " 86.12345/"),
        Err(CompositeRefusal::RatePrecision)
    );
    assert_eq!(
        change(" 86/", " 0/"),
        Err(CompositeRefusal::RateNotPositive)
    );
    // a leading zero in the rate or an amount
    assert_eq!(change(" 86/", " 0086/"), Err(CompositeRefusal::LeadingZero));
    assert_eq!(
        change("100.00", "0100.00"),
        Err(CompositeRefusal::LeadingZero)
    );
    assert_eq!(
        change("8600.00", "08600.00"),
        Err(CompositeRefusal::LeadingZero)
    );
    // a lone zero before the point is fine
    assert!(parse("$ 0.50 @ I\u{20b9} 86/$  = I\u{20b9} 43.00").is_ok());
    assert_eq!(change("= -I", "= I"), Err(CompositeRefusal::SignsDisagree));
    assert_eq!(
        change(" 86/", " /"),
        Err(CompositeRefusal::EmptyRateWithValue)
    );
    assert_eq!(
        parse("$ 100.00 @ I\u{20b9} 86/$  = I\u{20b9} 0.00"),
        Err(CompositeRefusal::ZeroBaseWithForeign)
    );
    // a zero foreign amount with a value elsewhere, and a signed zero
    assert_eq!(
        parse("$ 0.00 @ I\u{20b9} 86/$  = I\u{20b9} 8600.00"),
        Err(CompositeRefusal::ZeroForeignWithValue)
    );
    assert_eq!(
        parse("$ 0.00 @ I\u{20b9} 86/$  = I\u{20b9} 0.00"),
        Err(CompositeRefusal::ZeroForeignWithValue)
    );
    for signed_zero in [
        "-$ 0.00 @ I\u{20b9} /$  = I\u{20b9} 0.00",
        "$ 0.00 @ I\u{20b9} /$  = -I\u{20b9} 0.00",
        "$ 100.00 @ I\u{20b9} 86/$  = -I\u{20b9} 0.00",
    ] {
        assert_eq!(
            parse(signed_zero),
            Err(CompositeRefusal::SignedZero),
            "{signed_zero}"
        );
    }
    // a zero composite may quote a zero rate as well as none
    assert_eq!(
        parse("$ 0.00 @ I\u{20b9} 0/$  = I\u{20b9} 0.00"),
        Ok(CurrencyComposite::Zero {
            foreign_symbol: "$".to_string()
        })
    );
}
