//! Tally's composite amount: a foreign-currency amount, its rate and the base
//! amount, written into one field where an amount is carried in a foreign
//! currency: on a voucher's entries and bill allocations, and in a Trial Balance
//! row of a ledger whose own currency is the foreign one (`$`).
//!
//! Captured forms (licensed TallyPrime 7.1, a synthetic several-currency book):
//! `-$ 100.00 @ I₹ 86/$  = -I₹ 8600.00` on a voucher's entries and bill
//! allocation (`fixtures/agent/vouchers-forex-composite-20260915`), and with an
//! empty rate `$ 0.00 @ I₹ /$  = I₹ 0.00` in a Trial Balance
//! (`fixtures/trial_balance_currency_forex_live`).
//!
//! [`is_currency_composite`] classifies the shape and reads no value, so
//! nothing downstream can mistake a composite for an amount; the voucher,
//! trial-balance and outstandings readers use it, and set the row aside or
//! refuse it (the ledger-opening reader uses the looser classifier named below). [`parse_currency_composite`]
//! reads a composite into its parts, and is not called by any reader yet (#683):
//! a later slice adopts it everywhere at once, and deletes then the sign check
//! in `agent_voucher_parse.rs` (`composite_signs_agree`) and the looser
//! classifier in `native_outstandings/wire.rs` (`is_foreign_currency_balance`).
//!
//! [`parse_currency_composite`] refuses a different layout, not every value no
//! capture shows. Among the shapes admitted without a capture are these six,
//! and the tests pin each so a change is seen: a foreign symbol other than `$`
//! (the symbol is not checked against a list), a rate with one to three
//! decimals (every captured rate has none or four), a rate below one (every
//! captured rate is 84 or more), a foreign amount below one other than zero
//! (the smallest captured is 40.00), a base symbol other than the rupee symbol
//! (the book's own base is accepted whatever it is), and a zero composite that
//! writes a zero rate (the captured zero has an empty rate). The base amount is
//! also not tied to the foreign amount times the rate (see [`CurrencyComposite`]):
//! any base value in the right symbol and sign is admitted.
//!
//! Strict by design: a composite cut short, doubled, or with a non-ASCII digit
//! in an amount is not one, and falls through to the caller's own amount
//! parse, which refuses it. A currency symbol is any run without whitespace,
//! an ASCII digit or `-@=/`, so a symbol is not checked against a list.

use crate::native_outstandings::BaseCurrencyName;
use bridge_tally_primitives::ExactDecimal;

/// Whether `text` is exactly one composite: `<amount> @ <rate> = <amount>`,
/// each amount an optional `-`, a currency symbol, one space and an ASCII
/// decimal, and the rate `<base symbol> <decimal>/<foreign symbol>` or, empty,
/// `<base symbol> /<foreign symbol>`.
///
/// The symbols must agree with each other, as every captured composite does:
/// the rate is quoted in the base amount's symbol per the foreign amount's,
/// and the two are different currencies. A string of the right shape whose
/// symbols disagree is not a composite, so its caller refuses it.
///
/// Signs are not compared. A ledger balance may hold a foreign amount and a
/// base amount of opposite signs (bought at one rate, sold at another), but no
/// committed capture shows it. A caller for which the signs must agree checks
/// that itself; [`parse_currency_composite`] refuses disagreeing signs until a
/// capture shows them.
pub fn is_currency_composite(text: &str) -> bool {
    let Some((foreign, rest)) = split_once_exact(text, " @ ") else {
        return false;
    };
    let Some((rate, base)) = split_once_exact(rest, " = ") else {
        return false;
    };
    let (Some(foreign), Some(rate), Some(base)) = (
        symbol_amount(foreign),
        rate_symbols(rate.trim_end_matches(' ')),
        symbol_amount(base),
    ) else {
        return false;
    };
    rate.base == base.symbol && rate.per == foreign.symbol && foreign.symbol != base.symbol
}

fn split_once_exact<'a>(text: &'a str, separator: &str) -> Option<(&'a str, &'a str)> {
    let (left, right) = text.split_once(separator)?;
    (!right.contains(separator)).then_some((left, right))
}

struct SymbolAmount<'a> {
    symbol: &'a str,
}

fn symbol_amount(text: &str) -> Option<SymbolAmount<'_>> {
    let text = text.strip_prefix('-').unwrap_or(text);
    let (symbol, amount) = text.split_once(' ')?;
    (is_symbol(symbol) && is_ascii_decimal(amount)).then_some(SymbolAmount { symbol })
}

/// The rate's two symbols: the base it is quoted in, and the foreign unit.
struct RateSymbols<'a> {
    base: &'a str,
    per: &'a str,
}

fn rate_symbols(text: &str) -> Option<RateSymbols<'_>> {
    let (base, rest) = text.split_once(' ')?;
    let (number, per) = rest.split_once('/')?;
    (is_symbol(base) && (number.is_empty() || is_ascii_decimal(number)) && is_symbol(per))
        .then_some(RateSymbols { base, per })
}

fn is_symbol(text: &str) -> bool {
    !text.is_empty()
        && text.chars().all(|c| {
            !c.is_whitespace() && !c.is_ascii_digit() && !matches!(c, '-' | '@' | '=' | '/')
        })
}

fn is_ascii_decimal(text: &str) -> bool {
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    !whole.is_empty()
        && whole.bytes().all(|b| b.is_ascii_digit())
        && fraction.bytes().all(|b| b.is_ascii_digit())
        && (!fraction.is_empty() || !text.ends_with('.'))
}

/// A composite read into its parts, never recomputed: the base amount is the
/// text after `=` exactly as Tally wrote it, the foreign amount and the quoted
/// rate are kept apart, and the base symbol was checked against the book's own
/// base currency before this value existed.
///
/// UNUSED until a later slice adopts it in the readers (#683). Measured on one
/// currency pair (`$` against `I₹`) on release 7.1, the rate is Tally's display
/// of `base / foreign` rounded to four places (one capture, a rate of
/// 279.6667, is not an exact integer: the evidence for the rounding is that one
/// datum), so the base cannot be rebuilt from the foreign amount and the rate,
/// and the rate carries no arithmetic here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrencyComposite {
    /// Zero foreign and zero base, with an empty rate or a zero one: carries
    /// no amount.
    Zero {
        foreign_symbol: String,
    },
    Valued(ValuedComposite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValuedComposite {
    side: Side,
    foreign_symbol: String,
    foreign: ForeignAmount,
    rate: QuotedRate,
    base: BaseAmount,
}

impl ValuedComposite {
    /// The one sign the foreign and the base amount share.
    pub fn side(&self) -> Side {
        self.side
    }

    /// The foreign currency's symbol as written. It is not checked against
    /// any currency master here: the slice that adopts this parse must.
    pub fn foreign_symbol(&self) -> &str {
        &self.foreign_symbol
    }

    pub fn foreign(&self) -> &ForeignAmount {
        &self.foreign
    }

    pub fn rate(&self) -> &QuotedRate {
        &self.rate
    }

    pub fn base(&self) -> &BaseAmount {
        &self.base
    }
}

/// A foreign-currency magnitude: unsigned, exactly two decimal places, not
/// zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignAmount(ExactDecimal);

impl ForeignAmount {
    pub fn magnitude(&self) -> &ExactDecimal {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Positive,
    Negative,
}

/// A base-currency magnitude: unsigned, exactly two decimal places, not zero.
/// It can be built only by this module's parse, after the symbol equalled the
/// book's base currency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseAmount(ExactDecimal);

impl BaseAmount {
    pub fn magnitude(&self) -> &ExactDecimal {
        &self.0
    }
}

/// The rate a composite quotes, as written: positive, at most four decimal
/// places. No arithmetic is offered. On a ledger's total (a Trial Balance row)
/// Tally derives it from the two amounts, so it is not a voucher's own rate
/// there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedRate(ExactDecimal);

impl QuotedRate {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Why a string is not read as a composite. Every shape a capture does not
/// show is refused under its own name, so a variant that later appears in a
/// capture is added deliberately rather than read by accident.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeRefusal {
    /// Not one composite at all (the classifier refuses it).
    NotComposite,
    /// The base amount's symbol is not the book's base currency.
    BaseNotBookBase,
    /// The rate is not followed by exactly two spaces before ` = ` (the captured
    /// form). Other spacing mistakes (a missing ` @ ` or ` = `) are not a
    /// composite at all and come out as `NotComposite`.
    Spacing,
    /// An amount without exactly two decimal places.
    AmountScale,
    /// A rate with more than four decimal places.
    RatePrecision,
    /// An amount or rate with a leading zero before a digit (`0086`).
    LeadingZero,
    /// A rate that is not greater than zero.
    RateNotPositive,
    /// The foreign and base amounts carry different signs.
    SignsDisagree,
    /// A zero amount written with a sign.
    SignedZero,
    /// A zero foreign amount with a non-zero base amount or rate.
    ZeroForeignWithValue,
    /// A non-zero foreign amount with a zero base amount.
    ZeroBaseWithForeign,
    /// An empty rate with a non-zero foreign amount.
    EmptyRateWithValue,
}

/// Read `text` as one composite in the book whose base currency is `base`.
pub fn parse_currency_composite(
    text: &str,
    base: &BaseCurrencyName,
) -> Result<CurrencyComposite, CompositeRefusal> {
    if !is_currency_composite(text) {
        return Err(CompositeRefusal::NotComposite);
    }
    // The classifier has checked the shape; what follows reads its pieces.
    let (foreign_text, rest) = text
        .split_once(" @ ")
        .ok_or(CompositeRefusal::NotComposite)?;
    let (rate_text, base_text) = rest
        .split_once(" = ")
        .ok_or(CompositeRefusal::NotComposite)?;
    // The captured spacing: the rate ends with exactly one extra space.
    let Some(rate_text) = rate_text.strip_suffix(' ') else {
        return Err(CompositeRefusal::Spacing);
    };
    if rate_text.ends_with(' ') {
        return Err(CompositeRefusal::Spacing);
    }
    let foreign = signed_amount(foreign_text)?;
    let base_amount = signed_amount(base_text)?;
    if base_amount.symbol != base.name() {
        return Err(CompositeRefusal::BaseNotBookBase);
    }
    let rate = quoted_rate(rate_text)?;
    // Zero amounts are unsigned.
    if (foreign.zero && foreign.negative) || (base_amount.zero && base_amount.negative) {
        return Err(CompositeRefusal::SignedZero);
    }
    if foreign.zero {
        let rate_is_zero = rate.as_ref().is_none_or(|rate| rate.0.is_zero());
        return if base_amount.zero && rate_is_zero {
            Ok(CurrencyComposite::Zero {
                foreign_symbol: foreign.symbol.to_string(),
            })
        } else {
            Err(CompositeRefusal::ZeroForeignWithValue)
        };
    }
    if base_amount.zero {
        return Err(CompositeRefusal::ZeroBaseWithForeign);
    }
    if foreign.negative != base_amount.negative {
        return Err(CompositeRefusal::SignsDisagree);
    }
    let Some(rate) = rate else {
        return Err(CompositeRefusal::EmptyRateWithValue);
    };
    if rate.0.is_zero() {
        return Err(CompositeRefusal::RateNotPositive);
    }
    Ok(CurrencyComposite::Valued(ValuedComposite {
        side: if foreign.negative {
            Side::Negative
        } else {
            Side::Positive
        },
        foreign_symbol: foreign.symbol.to_string(),
        foreign: ForeignAmount(foreign.magnitude),
        rate,
        base: BaseAmount(base_amount.magnitude),
    }))
}

struct ReadAmount<'a> {
    symbol: &'a str,
    negative: bool,
    zero: bool,
    magnitude: ExactDecimal,
}

/// `[-]<symbol> <amount>`, the amount with exactly two decimal places.
fn signed_amount(text: &str) -> Result<ReadAmount<'_>, CompositeRefusal> {
    let (negative, unsigned) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let (symbol, amount) = unsigned
        .split_once(' ')
        .ok_or(CompositeRefusal::NotComposite)?;
    match amount.split_once('.') {
        Some((_, fraction)) if fraction.len() == 2 => {}
        _ => return Err(CompositeRefusal::AmountScale),
    }
    if has_leading_zero(amount) {
        return Err(CompositeRefusal::LeadingZero);
    }
    let magnitude = ExactDecimal::parse(amount).map_err(|_| CompositeRefusal::NotComposite)?;
    Ok(ReadAmount {
        symbol,
        negative,
        zero: magnitude.is_zero(),
        magnitude,
    })
}

/// A whole part of two or more digits that starts with `0` (`0086`, `00.50`).
fn has_leading_zero(decimal: &str) -> bool {
    let whole = decimal.split_once('.').map_or(decimal, |(whole, _)| whole);
    whole.len() > 1 && whole.starts_with('0')
}

/// `<base symbol> <rate>/<foreign symbol>` or, empty, `<base symbol> /<foreign
/// symbol>`: `None` for the empty rate.
fn quoted_rate(text: &str) -> Result<Option<QuotedRate>, CompositeRefusal> {
    let (_, rest) = text.split_once(' ').ok_or(CompositeRefusal::NotComposite)?;
    let (number, _) = rest.split_once('/').ok_or(CompositeRefusal::NotComposite)?;
    if number.is_empty() {
        return Ok(None);
    }
    if number
        .split_once('.')
        .is_some_and(|(_, fraction)| fraction.len() > 4)
    {
        return Err(CompositeRefusal::RatePrecision);
    }
    if has_leading_zero(number) {
        return Err(CompositeRefusal::LeadingZero);
    }
    let rate = ExactDecimal::parse(number).map_err(|_| CompositeRefusal::NotComposite)?;
    // A zero rate is refused by the caller where the amounts are not zero; a
    // negative rate is not a composite (the classifier refuses its `-`).
    Ok(Some(QuotedRate(rate)))
}

#[cfg(test)]
#[path = "currency_composite_tests.rs"]
mod tests;
