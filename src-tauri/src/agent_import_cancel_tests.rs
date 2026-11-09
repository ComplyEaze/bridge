use super::super::render_native_vouchers_xml;
use super::*;

/// For each voucher, its cancel rendering is its native Create rendering with
/// only the `ACTION` changed: a byte comparison, after checking the Create
/// rendering carries the attribute exactly once. Returns the last cancel.
pub(in crate::agent) fn assert_cancel_changes_only_the_action(
    company: &str,
    vouchers: &[(&ImportVoucher, Uuid)],
) -> String {
    assert!(!vouchers.is_empty());
    let mut last = String::new();
    for (voucher, id) in vouchers {
        let create = render_native_vouchers_xml(company, std::iter::once((*voucher, *id)));
        let cancel = render_native_cancel_xml(company, voucher, RemoteId(*id)).unwrap();
        assert_eq!(create.matches(" ACTION=\"Create\" ").count(), 1);
        assert!(!create.contains("ACTION=\"Cancel\""));
        let expected = create.replace(" ACTION=\"Create\" ", " ACTION=\"Cancel\" ");
        assert_eq!(cancel.as_bytes(), expected.as_bytes());
        last = cancel;
    }
    last
}

#[test]
fn a_remote_id_is_admitted_only_in_the_form_a_native_post_mints() {
    let minted = Uuid::new_v4();
    let text = minted.to_string();
    assert_eq!(RemoteId::parse(&text), Ok(RemoteId(minted)));
    let refused = Err("import_remote_id_not_native");
    for other in [
        String::new(),
        text.to_uppercase(),
        minted.simple().to_string(),
        minted.braced().to_string(),
        minted.urn().to_string(),
        format!(" {text}"),
        format!("{text}\""),
        format!("{text}\" ACTION=\"Delete"),
        Uuid::nil().to_string(),
        // Version 1 and version 7 UUIDs: well formed, but not what a post mints.
        "c232ab00-9414-11ec-b3c8-9f6bdeced846".to_string(),
        "017f22e2-79b0-7cc3-98c4-dc0c0c07398f".to_string(),
        // Version 4 with a variant a v4 generator never sets.
        format!("{}c{}", &text[..19], &text[20..]),
    ] {
        assert_eq!(RemoteId::parse(&other), refused, "{other}");
    }
}

/// Every REMOTEID of the live-captured native requests is admitted.
#[test]
fn every_captured_native_remote_id_is_admitted() {
    for fixture in [
        include_str!("../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-import.xml"),
        include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-journal-import.xml"
        ),
    ] {
        let ids = fixture
            .split("REMOTEID=\"")
            .skip(1)
            .map(|rest| &rest[..36])
            .collect::<Vec<_>>();
        assert!(!ids.is_empty());
        for id in ids {
            assert!(RemoteId::parse(id).is_ok(), "{id}");
        }
    }
}

/// `text` with the hex digit at `index` replaced by `digit`.
fn with_digit(text: &str, index: usize, digit: char) -> String {
    format!("{}{digit}{}", &text[..index], &text[index + 1..])
}

/// The version nibble is the first digit of the third group and the variant
/// nibble the first digit of the fourth. Only version 4 with the RFC variant
/// (8, 9, a or b) is what a native post mints; every other version and every
/// other variant is refused with the one code, each checked in place of the
/// minted id's own digit.
#[test]
fn a_remote_id_is_admitted_for_version_4_and_the_rfc_variant_alone() {
    const VERSION_AT: usize = 14;
    const VARIANT_AT: usize = 19;
    let refused = Err("import_remote_id_not_native");
    let text = Uuid::new_v4().to_string();
    assert_eq!(text.as_bytes()[VERSION_AT], b'4');
    assert!(matches!(
        text.as_bytes()[VARIANT_AT],
        b'8' | b'9' | b'a' | b'b'
    ));
    for digit in "0123456789abcdef".chars() {
        let versioned = with_digit(&text, VERSION_AT, digit);
        let expected = if digit == '4' {
            Ok(RemoteId(Uuid::try_parse(&versioned).unwrap()))
        } else {
            refused
        };
        assert_eq!(RemoteId::parse(&versioned), expected, "version {digit}");
        let varied = with_digit(&text, VARIANT_AT, digit);
        let expected = if matches!(digit, '8' | '9' | 'a' | 'b') {
            Ok(RemoteId(Uuid::try_parse(&varied).unwrap()))
        } else {
            refused
        };
        assert_eq!(RemoteId::parse(&varied), expected, "variant {digit}");
    }
}

/// The nil and max ids carry a version of their own and a variant no minted id
/// has; the id an import derives for a voucher is version 8 (`import_identity`),
/// which is the one a person's file carries and which this module must refuse.
#[test]
fn the_nil_the_max_and_an_imports_derived_id_are_not_remote_ids() {
    let refused = Err("import_remote_id_not_native");
    let derived = super::super::import_identity("bridge-synthetic-batch", "TXN-1");
    assert_eq!(derived.get_version(), Some(uuid::Version::Custom));
    for id in [Uuid::nil(), Uuid::max(), derived] {
        assert_eq!(
            RemoteId::parse(&id.hyphenated().to_string()),
            refused,
            "{id}"
        );
    }
}

/// One character short or long is not a UUID at all, and is refused with the
/// same code as a UUID of the wrong form.
#[test]
fn a_remote_id_one_character_short_or_long_is_refused() {
    let refused = Err("import_remote_id_not_native");
    let text = Uuid::new_v4().to_string();
    assert_eq!(text.len(), 36);
    for other in [
        text[..35].to_string(),
        text[1..].to_string(),
        format!("{text}0"),
        format!("0{text}"),
    ] {
        assert_eq!(RemoteId::parse(&other), refused, "{other}");
    }
}
