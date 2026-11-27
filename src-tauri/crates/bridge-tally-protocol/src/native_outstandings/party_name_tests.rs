//! Each admitted mark is read here against its own capture: the request the
//! lab sent for a name holding it, Tally's answer, and the snapshot of the
//! same book (`tests/fixtures/agent/party-bills/PROVENANCE.md`, rows
//! 2480-NS, 2480-NX, 2480-NC and 2480-MX). The answer names no party, so a
//! test ties it to that name's own row of the snapshot and checks that its
//! one bill is the bill that name's invoice made.

use super::*;
use crate::native_outstandings::{
    parse_ledger_outstandings_answer, parse_native_ledger_snapshot_for_company,
    render_ledger_outstandings_request, PartyBillReference,
};
use bridge_tally_primitives::{ExactDecimal, TallyDate};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/agent/party-bills/"
);

/// The synthetic book every row read (in the synthetic-UUID registry).
const BOOK: &str = "BRIDGE PILOT LAB";
const BOOK_GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

/// A capture as text. The lab's request files carry the byte-order mark they
/// were sent with; Tally's answers carry none.
fn capture(name: &str) -> String {
    let bytes = std::fs::read(format!("{FIXTURES}{name}")).expect("capture is present");
    let bytes = bytes.strip_prefix(&[0xff, 0xfe]).unwrap_or(&bytes);
    assert_eq!(bytes.len() % 2, 0, "{name} holds whole UTF-16 units");
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    String::from_utf16(&units).expect("capture decodes")
}

fn date(value: &str) -> TallyDate {
    TallyDate::parse(value).expect("date")
}

/// One row's name read through `Ledger Outstandings`: `row` and `id` name the
/// captures, `number` the bill its invoice made.
fn reads_as_its_own_ledger(row: &str, id: &str, name: &str, number: &str) {
    let party = LedgerOutstandingsParty::new(name).expect("the name is admitted");
    assert_eq!(party.as_str(), name);
    let to = date("20260801");
    assert_eq!(
        render_ledger_outstandings_request(BOOK, &party, &to),
        capture(&format!(
            "ledger-outstandings-2480-{row}-{id}-20260801.request.utf16le.xml"
        )),
        "Bridge sends the request the lab sent for {name:?}"
    );
    let answer = parse_ledger_outstandings_answer(
        &capture(&format!(
            "ledger-outstandings-2480-{row}-{id}-20260801.utf16le.xml"
        )),
        &date("20260401"),
        &to,
    )
    .expect("the answer parses");
    let snapshot = parse_native_ledger_snapshot_for_company(
        &capture(&format!("ledger-snapshot-2480-{row}-20260801.utf16le.xml")),
        BOOK_GUID,
    )
    .expect("the snapshot parses");
    let ledger = snapshot
        .iter()
        .filter(|entry| entry.name == name)
        .collect::<Vec<_>>();
    assert_eq!(ledger.len(), 1, "one ledger is named {name:?}");
    assert!(ledger[0].bill_wise_on);
    let read = answer
        .tie(ledger[0].closing_balance.as_ref())
        .expect("the bills tie to that ledger's closing");
    assert!(read.on_account().is_empty());
    let [bill] = read.bills() else {
        panic!("one bill for {name:?}: {:?}", read.bills());
    };
    assert_eq!(
        bill.reference,
        PartyBillReference::Bill(format!("BP/26-27/{number}"))
    );
    assert!(bill
        .closing
        .numeric_eq(&ExactDecimal::parse("-1051.00").unwrap()));
    assert_eq!(bill.bill_date, date("20260801"));
}

/// One test per admitted mark, each on the one-mark name the lab read for it.
macro_rules! a_mark_is_read {
    ($($test:ident: $mark:expr, $row:literal, $id:literal, $name:literal, $number:literal;)*) => {$(
        #[test]
        fn $test() {
            let mark: char = $mark;
            assert!($name.contains(mark), "{:?} holds {mark:?}", $name);
            assert!(MEASURED_MARKS.contains(&mark), "{mark:?} is admitted");
            reads_as_its_own_ledger($row, $id, $name, $number);
        }
    )*};
}

a_mark_is_read! {
    a_hyphen_is_read: '-', "ns", "hy", "BRIDGE Ns Hk-Hm", "0222";
    a_full_stop_is_read: '.', "ns", "dt", "BRIDGE Ns Dk.Dm", "0223";
    an_ampersand_is_read: '&', "ns", "am", "BRIDGE Ns Ak&Am", "0224";
    a_slash_is_read: '/', "ns", "sl", "BRIDGE Ns Sk/Sm", "0225";
    a_left_bracket_is_read: '(', "ns", "lp", "BRIDGE Ns Lk(Lm", "0226";
    a_right_bracket_is_read: ')', "ns", "rp", "BRIDGE Ns Rk)Rm", "0227";
    an_apostrophe_is_read: '\'', "ns", "ap", "BRIDGE Ns Pk'Pm", "0228";
    digits_with_a_dash_are_read: '-', "ns", "dd", "BRIDGE Ns 2023-24", "0230";
    a_comma_is_read: ',', "nx", "cm", "BRIDGE Nx Ak, Am", "0262";
    an_underscore_is_read: '_', "nx", "us", "BRIDGE Nx Bk_Bm", "0263";
    a_pair_of_double_quotes_is_read: '"', "nx", "dq", "BRIDGE Nx \"Ck\" Cm", "0264";
    a_plus_is_read: '+', "nx", "ps", "BRIDGE Nx Dk + Dm", "0265";
    an_at_sign_is_read: '@', "nx", "at", "BRIDGE Nx Ek @ Em", "0266";
    a_hash_is_read: '#', "nx", "hs", "BRIDGE Nx Fk #12", "0267";
    a_colon_is_read: ':', "nx", "cl", "BRIDGE Nx Gk: Gm", "0268";
    a_percent_sign_is_read: '%', "nx", "pc", "BRIDGE Nx Hk 15% Hm", "0269";
    an_asterisk_is_read: '*', "nx", "st", "BRIDGE Nx Jk * Jm", "0270";
    an_exclamation_mark_is_read: '!', "nx", "ex", "BRIDGE Nx Kk!", "0271";
}

/// `and`, which [`crate::ledger_scope::LedgerScopeName`] refuses as a formula
/// word, is admitted here as a word.
#[test]
fn the_word_and_is_read() {
    assert!(MEASURED_FORMULA_WORDS.contains(&"and"));
    reads_as_its_own_ledger("ns", "op", "BRIDGE Ns Ok and Om", "0229");
}

/// The one Devanagari word read holds each of the six characters admitted, and
/// no other.
#[test]
fn the_devanagari_word_is_read() {
    let name = "BRIDGE Ns \u{0909}\u{0926}\u{093E}\u{0939}\u{0930}\u{0923}";
    let devanagari = name.chars().filter(|c| !c.is_ascii()).collect::<Vec<_>>();
    assert_eq!(devanagari, MEASURED_DEVANAGARI);
    reads_as_its_own_ledger("ns", "sc", name, "0231");
}

/// Three of the mixed names the lab read: the Devanagari word beside two
/// marks, every first-row mark with `and`, and all seventeen marks in one name.
#[test]
fn names_that_mix_measured_marks_are_read() {
    reads_as_its_own_ledger(
        "nc",
        "sm",
        "BRIDGE Nc \u{0909}\u{0926}\u{093E}\u{0939}\u{0930}\u{0923} & Gk.",
        "0248",
    );
    reads_as_its_own_ledger(
        "nc",
        "al",
        "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24",
        "0250",
    );
    let all = "BRIDGE Mx Lk, Lm_Ln \"Lp\" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly's";
    for mark in MEASURED_MARKS {
        assert!(all.contains(mark), "{mark:?}");
    }
    reads_as_its_own_ledger("mx", "al", all, "0291");
}

/// Every name the narrower `CHILDOF` type admits is admitted here unchanged.
#[test]
fn a_plain_name_is_admitted_as_before() {
    for name in ["Lab Party P1", "BRIDGE Supplier RJ", "Cash", "7", "a b c"] {
        assert!(crate::ledger_scope::LedgerScopeName::new(name).is_ok());
        assert_eq!(LedgerOutstandingsParty::new(name).unwrap().as_str(), name);
    }
}

/// No row has read these in a `LedgerName`, so each is refused by the mark it
/// holds: the ASCII marks outside the two tables, the control characters,
/// Devanagari signs, letters and digits outside the word read, other scripts
/// and accented letters, and the look-alikes of admitted marks.
#[test]
fn a_name_with_an_unmeasured_mark_is_refused_by_that_mark() {
    let unmeasured = [
        // ASCII marks no row read.
        '<', '>', ';', '=', '$', '?', '[', ']', '{', '}', '|', '\\', '`', '~', '^',
        // Control characters.
        '\n', '\r', '\t', '\u{0}', '\u{4}', '\u{7f}',
        // Devanagari outside the word read: a virama, a nukta, an anusvara, a
        // visarga, a candrabindu, another consonant, another vowel sign, a
        // digit and the danda; then other scripts and accented letters.
        '\u{094D}', '\u{093C}', '\u{0902}', '\u{0903}', '\u{0901}', '\u{0915}', '\u{093F}',
        '\u{0966}', '\u{0964}', '\u{e9}', '\u{430}', '\u{0B95}', '\u{4e2d}', '\u{ff21}',
        // Look-alikes of admitted marks and of the space.
        '\u{2019}', '\u{201c}', '\u{2013}', '\u{2010}', '\u{ff06}', '\u{a0}', '\u{200b}',
        '\u{fffd}',
    ];
    for mark in unmeasured {
        let name = format!("Ak{mark}Am");
        assert_eq!(
            LedgerOutstandingsParty::new(&name),
            Err(LedgerOutstandingsPartyError::UnmeasuredMark(mark)),
            "{name:?}"
        );
    }
    // The first unmeasured mark is the one named, whatever follows it.
    assert_eq!(
        LedgerOutstandingsParty::new("Ak & Am; Co <x>"),
        Err(LedgerOutstandingsPartyError::UnmeasuredMark(';'))
    );
    // A name the lab's tables would admit but for one unmeasured mark.
    assert_eq!(
        LedgerOutstandingsParty::new("Gupta, Verma & Co.\u{2019}s"),
        Err(LedgerOutstandingsPartyError::UnmeasuredMark('\u{2019}'))
    );
}

/// Only `and` was read: every other formula word is refused by name, as a
/// word, whatever its case, and whatever mark parts it from the next.
#[test]
fn a_name_with_an_unmeasured_formula_word_is_refused_by_that_word() {
    for word in FORMULA_WORDS {
        if MEASURED_FORMULA_WORDS.contains(&word) {
            continue;
        }
        for name in [
            format!("Ak {word} Am"),
            format!("Ak {} Am", word.to_ascii_uppercase()),
            format!("Ak/{word}-Am"),
            word.to_string(),
        ] {
            assert_eq!(
                LedgerOutstandingsParty::new(&name),
                Err(LedgerOutstandingsPartyError::UnmeasuredFormulaWord(word)),
                "{name:?}"
            );
        }
    }
    // A measured word in a case no row read.
    for name in ["Ak AND Am", "Ak And Am", "Ak/aND-Am"] {
        assert_eq!(
            LedgerOutstandingsParty::new(name),
            Err(LedgerOutstandingsPartyError::UnmeasuredFormulaWord("and")),
            "{name:?}"
        );
    }
    // The same letters inside a longer word are a plain name.
    for name in ["Island Order Book", "Android Notes", "Within Stores"] {
        assert!(LedgerOutstandingsParty::new(name).is_ok(), "{name}");
    }
}

#[test]
fn empty_edge_spaced_doubled_and_overlong_names_are_refused_by_their_own_reason() {
    assert_eq!(
        LedgerOutstandingsParty::new(""),
        Err(LedgerOutstandingsPartyError::Empty)
    );
    for name in [" Ak & Am", "Ak & Am ", "Ak  & Am", " "] {
        assert_eq!(
            LedgerOutstandingsParty::new(name),
            Err(LedgerOutstandingsPartyError::Spacing),
            "{name:?}"
        );
    }
    let longest = "a".repeat(LEDGER_SCOPE_NAME_MAX_CHARS);
    assert!(LedgerOutstandingsParty::new(&longest).is_ok());
    assert_eq!(
        LedgerOutstandingsParty::new(&format!("{longest}a")),
        Err(LedgerOutstandingsPartyError::TooLong)
    );
}

#[test]
fn each_refusal_has_its_own_code() {
    let codes = [
        LedgerOutstandingsPartyError::Empty,
        LedgerOutstandingsPartyError::TooLong,
        LedgerOutstandingsPartyError::Spacing,
        LedgerOutstandingsPartyError::UnmeasuredMark('<'),
        LedgerOutstandingsPartyError::UnmeasuredFormulaWord("or"),
    ]
    .map(LedgerOutstandingsPartyError::code);
    let mut sorted = codes.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), 5);
}
