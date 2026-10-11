use super::*;
use crate::text_encoding::{
    decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
};

const SHAPE_LIVE: &[u8] =
    include_bytes!("../tests/fixtures/company_features_shape_lab_live.utf16le.xml");
const SHAPE_REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/company_features_shape_lab_request.utf16le.xml");
const FOREX_LIVE: &[u8] =
    include_bytes!("../tests/fixtures/company_features_corpus_forex_live.utf16le.xml");
const FOREX_REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/company_features_corpus_forex_request.utf16le.xml");

const SHAPE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const FOREX_GUID: &str = "b14e9b2d-8a63-4779-804d-25d59eb787eb";

/// A captured response, decoded as production decodes it.
fn response(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap()
    .text
}

fn shape() -> ExpectedCompany<'static> {
    ExpectedCompany {
        guid: SHAPE_GUID,
        name: "BRIDGE SHAPE LAB",
        number: "100021",
        books_from: "20250401",
    }
}

fn forex() -> ExpectedCompany<'static> {
    ExpectedCompany {
        guid: FOREX_GUID,
        name: "BRIDGE CORPUS FOREX",
        number: "100011",
        books_from: "20250401",
    }
}

/// The captured SHAPE answer with its first occurrence of `from` replaced by `to`.
fn mutate(from: &str, to: &str) -> String {
    let live = response(SHAPE_LIVE);
    assert!(live.contains(from), "mutation target not found: {from}");
    live.replacen(from, to, 1)
}

fn flag(name: &str, value: &str) -> String {
    format!("<{name} TYPE=\"Logical\">{value}</{name}>")
}

#[test]
fn the_request_is_byte_equal_to_each_committed_capture() {
    assert_eq!(
        render_company_features_request("BRIDGE SHAPE LAB", SHAPE_GUID).unwrap(),
        decode_xml_bytes(SHAPE_REQUEST).unwrap()
    );
    assert_eq!(
        render_company_features_request("BRIDGE CORPUS FOREX", FOREX_GUID).unwrap(),
        decode_xml_bytes(FOREX_REQUEST).unwrap()
    );
}

#[test]
fn a_guid_that_could_break_the_filter_is_refused_and_a_name_is_escaped() {
    for bad in ["", "a\" or 1=1 or \"", "3a6bd6e1 b835", "x;y"] {
        assert_eq!(
            render_company_features_request("X", bad),
            Err(NativeCompanyFeaturesError::GuidUnsupported),
            "{bad:?}"
        );
    }
    let request = render_company_features_request("A & B <C>", SHAPE_GUID).unwrap();
    assert!(request.contains("<SVCURRENTCOMPANY>A &amp; B &lt;C&gt;</SVCURRENTCOMPANY>"));
}

#[test]
fn the_two_captured_books_read_as_their_settings_and_symbol() {
    assert_eq!(
        parse_company_features(&response(SHAPE_LIVE), &shape()),
        Ok(NativeCompanyFeatures {
            cost_centres: NativeSetting::No,
            gst: NativeSetting::Yes,
            batch_wise: NativeSetting::Yes,
            base_currency: NativeCurrencySymbol::Reported("\u{20b9}".to_string()),
        })
    );
    assert_eq!(
        parse_company_features(&response(FOREX_LIVE), &forex()),
        Ok(NativeCompanyFeatures {
            cost_centres: NativeSetting::No,
            gst: NativeSetting::Yes,
            batch_wise: NativeSetting::No,
            base_currency: NativeCurrencySymbol::Reported("\u{20b9}".to_string()),
        })
    );
}

#[test]
fn each_setting_follows_its_own_element_and_no_other() {
    // One feature of the captured answer at a time. A third synthetic book read
    // cost centres Yes and GST No (recorded, not committed); these edits show
    // each value reaches only its own field.
    let cost = parse_company_features(
        &mutate(
            &flag("ISCOSTCENTRESON", "No"),
            &flag("ISCOSTCENTRESON", "Yes"),
        ),
        &shape(),
    )
    .unwrap();
    assert_eq!(
        (cost.cost_centres, cost.gst, cost.batch_wise),
        (NativeSetting::Yes, NativeSetting::Yes, NativeSetting::Yes)
    );
    let gst = parse_company_features(
        &mutate(&flag("ISGSTON", "Yes"), &flag("ISGSTON", "No")),
        &shape(),
    )
    .unwrap();
    assert_eq!(
        (gst.cost_centres, gst.gst, gst.batch_wise),
        (NativeSetting::No, NativeSetting::No, NativeSetting::Yes)
    );
    let batch = parse_company_features(
        &mutate(&flag("ISBATCHWISEON", "Yes"), &flag("ISBATCHWISEON", "No")),
        &shape(),
    )
    .unwrap();
    assert_eq!(
        (batch.cost_centres, batch.gst, batch.batch_wise),
        (NativeSetting::No, NativeSetting::Yes, NativeSetting::No)
    );
}

#[test]
fn a_flag_that_is_not_read_does_not_change_the_answer() {
    // ISEDITLOGON is fetched by the request and not read: even a value that is
    // not Yes or No does not refuse the read.
    let parsed = parse_company_features(
        &mutate(&flag("ISEDITLOGON", "No"), &flag("ISEDITLOGON", "Maybe")),
        &shape(),
    )
    .unwrap();
    assert_eq!(parsed.gst, NativeSetting::Yes);
}

#[test]
fn a_setting_tally_did_not_send_is_not_reported_and_never_no() {
    for (element, pick) in [
        (
            "ISCOSTCENTRESON",
            (|f: &NativeCompanyFeatures| f.cost_centres)
                as fn(&NativeCompanyFeatures) -> NativeSetting,
        ),
        ("ISGSTON", |f| f.gst),
        ("ISBATCHWISEON", |f| f.batch_wise),
    ] {
        let live = response(SHAPE_LIVE);
        let start = live.find(&format!("<{element} ")).unwrap();
        let end = start + live[start..].find("</").unwrap();
        let end = end + live[end..].find('>').unwrap() + 1;
        let edited = format!("{}{}", &live[..start], &live[end..]);
        let parsed = parse_company_features(&edited, &shape()).unwrap();
        assert_eq!(pick(&parsed), NativeSetting::NotReported, "{element}");
    }
}

#[test]
fn a_setting_that_is_empty_self_closed_or_not_yes_or_no_refuses_the_read_naming_it() {
    for (element, label) in [
        ("ISCOSTCENTRESON", "cost_centres"),
        ("ISGSTON", "gst"),
        ("ISBATCHWISEON", "batch_wise"),
    ] {
        let live = response(SHAPE_LIVE);
        let value = if element == "ISCOSTCENTRESON" {
            "No"
        } else {
            "Yes"
        };
        for bad in [
            format!("<{element} TYPE=\"Logical\"></{element}>"),
            format!("<{element} TYPE=\"Logical\"/>"),
            flag(element, "yes"),
            flag(element, "Maybe"),
            flag(element, " "),
            flag(element, "1"),
        ] {
            let edited = live.replacen(&flag(element, value), &bad, 1);
            assert_ne!(edited, live, "{element}");
            assert_eq!(
                parse_company_features(&edited, &shape()),
                Err(NativeCompanyFeaturesError::SettingInvalid(label)),
                "{element}: {bad}"
            );
        }
    }
}

#[test]
fn a_repeated_setting_refuses_the_read() {
    let repeated = mutate(
        &flag("ISGSTON", "Yes"),
        &format!("{}{}", flag("ISGSTON", "Yes"), flag("ISGSTON", "No")),
    );
    assert_eq!(
        parse_company_features(&repeated, &shape()),
        Err(NativeCompanyFeaturesError::Shape(
            "stock_row_field_repeated"
        ))
    );
}

#[test]
fn the_symbol_is_read_as_sent_and_a_missing_or_blank_one_is_not_reported() {
    let symbol = "<CURRENCYNAME TYPE=\"String\">\u{20b9}</CURRENCYNAME>";
    for (replacement, want) in [
        (
            "<CURRENCYNAME TYPE=\"String\">US$</CURRENCYNAME>",
            NativeCurrencySymbol::Reported("US$".to_string()),
        ),
        (
            "<CURRENCYNAME TYPE=\"String\"></CURRENCYNAME>",
            NativeCurrencySymbol::NotReported,
        ),
        (
            "<CURRENCYNAME TYPE=\"String\">  </CURRENCYNAME>",
            NativeCurrencySymbol::NotReported,
        ),
        ("", NativeCurrencySymbol::NotReported),
    ] {
        let parsed = parse_company_features(&mutate(symbol, replacement), &shape()).unwrap();
        assert_eq!(parsed.base_currency, want, "{replacement}");
    }
}

#[test]
fn a_symbol_that_hides_text_or_is_over_the_bound_refuses_the_read() {
    let symbol = "<CURRENCYNAME TYPE=\"String\">\u{20b9}</CURRENCYNAME>";
    for bad in [
        "A\u{7}B",
        "ABCDEFGHIJKLMNOPQ",
        // bidirectional override and isolate, zero-width space and joiner, a
        // line separator and a tag character
        "A\u{202e}B",
        "A\u{2066}B",
        "A\u{200b}B",
        "A\u{200d}B",
        "A\u{2028}B",
        "A\u{e0041}B",
        // a format character that is not default-ignorable (Arabic number
        // sign), a default-ignorable one that is not a format character
        // (combining grapheme joiner), and a paragraph separator
        "A\u{600}B",
        "A\u{34f}B",
        "A\u{2029}B",
    ] {
        let edited = mutate(
            symbol,
            &format!("<CURRENCYNAME TYPE=\"String\">{bad}</CURRENCYNAME>"),
        );
        assert_eq!(
            parse_company_features(&edited, &shape()),
            Err(NativeCompanyFeaturesError::CurrencyInvalid),
            "{bad:?}"
        );
    }
    // Sixteen characters are within the bound.
    let edited = mutate(
        symbol,
        "<CURRENCYNAME TYPE=\"String\">ABCDEFGHIJKLMNOP</CURRENCYNAME>",
    );
    assert!(parse_company_features(&edited, &shape()).is_ok());
}

#[test]
fn a_row_for_another_company_or_with_another_book_refuses_naming_the_field() {
    let live = response(SHAPE_LIVE);
    let other = ExpectedCompany {
        guid: FOREX_GUID,
        ..shape()
    };
    assert_eq!(
        parse_company_features(&live, &other),
        Err(NativeCompanyFeaturesError::CompanyMismatch("guid"))
    );
    for (expected, field) in [
        (
            ExpectedCompany {
                name: "BRIDGE SHAPE LAB 2",
                ..shape()
            },
            "name",
        ),
        (
            ExpectedCompany {
                number: "100022",
                ..shape()
            },
            "company_number",
        ),
        (
            ExpectedCompany {
                books_from: "20240401",
                ..shape()
            },
            "books_from",
        ),
    ] {
        assert_eq!(
            parse_company_features(&live, &expected),
            Err(NativeCompanyFeaturesError::CompanyMismatch(field)),
            "{field}"
        );
    }
    // A GUID differing only in case is the same GUID.
    let upper_guid = SHAPE_GUID.to_ascii_uppercase();
    let upper = ExpectedCompany {
        guid: &upper_guid,
        ..shape()
    };
    assert!(parse_company_features(&live, &upper).is_ok());
}

#[test]
fn a_row_that_lacks_an_identity_field_refuses_because_nothing_is_equal_to_it() {
    for element in ["GUID", "NAME", "COMPANYNUMBER", "BOOKSFROM"] {
        let live = response(SHAPE_LIVE);
        let start = live.find(&format!("<{element} ")).unwrap();
        let end = start + live[start..].find("</").unwrap();
        let end = end + live[end..].find('>').unwrap() + 1;
        let edited = format!("{}{}", &live[..start], &live[end..]);
        assert!(
            matches!(
                parse_company_features(&edited, &shape()),
                Err(NativeCompanyFeaturesError::CompanyMismatch(_))
            ),
            "{element}"
        );
    }
}

#[test]
fn zero_or_two_rows_refuse_and_the_counter_block_is_not_a_row() {
    let live = response(SHAPE_LIVE);
    // The capture's CMPINFO block holds a bare <COMPANY>0</COMPANY> counter
    // outside the collection: it is not read as a row.
    assert!(live.contains("<COMPANY>0</COMPANY>"));
    let start = live.find("<COMPANY NAME=").unwrap();
    let end = live.rfind("</COMPANY>").unwrap() + "</COMPANY>".len();
    let row = &live[start..end];
    assert_eq!(
        parse_company_features(&live.replacen(row, "", 1), &shape()),
        Err(NativeCompanyFeaturesError::NotOneRow)
    );
    assert_eq!(
        parse_company_features(&live.replacen(row, &format!("{row}{row}"), 1), &shape()),
        Err(NativeCompanyFeaturesError::NotOneRow)
    );
}

#[test]
fn a_failure_signal_or_a_shape_that_is_not_the_collection_refuses_with_its_own_code() {
    for tail in ["<LINEERROR>x</LINEERROR>", "<ERROR/>"] {
        let edited = mutate("</ENVELOPE>", &format!("{tail}</ENVELOPE>"));
        assert_eq!(
            parse_company_features(&edited, &shape()),
            Err(NativeCompanyFeaturesError::TallyReportedFailure),
            "{tail}"
        );
    }
    let status = mutate("<STATUS>1</STATUS>", "<STATUS>0</STATUS>");
    assert_eq!(
        parse_company_features(&status, &shape()),
        Err(NativeCompanyFeaturesError::TallyReportedFailure)
    );
    assert_eq!(
        parse_company_features("<REPORT/>", &shape()),
        Err(NativeCompanyFeaturesError::Shape("stock_root_not_envelope"))
    );
}

#[test]
fn every_error_has_its_own_stable_code_and_the_cause_names_where() {
    assert_eq!(
        [
            NativeCompanyFeaturesError::TallyReportedFailure.code(),
            NativeCompanyFeaturesError::Shape("x").code(),
            NativeCompanyFeaturesError::NotOneRow.code(),
            NativeCompanyFeaturesError::CompanyMismatch("guid").code(),
            NativeCompanyFeaturesError::SettingInvalid("cost_centres").code(),
            NativeCompanyFeaturesError::SettingInvalid("gst").code(),
            NativeCompanyFeaturesError::SettingInvalid("batch_wise").code(),
            NativeCompanyFeaturesError::CurrencyInvalid.code(),
            NativeCompanyFeaturesError::GuidUnsupported.code(),
        ],
        [
            "company_features_tally_reported_failure",
            "company_features_response_invalid",
            "company_features_not_one_row",
            "company_features_company_mismatch",
            "company_features_setting_invalid:cost_centres",
            "company_features_setting_invalid:gst",
            "company_features_setting_invalid:batch_wise",
            "company_features_currency_invalid",
            "company_features_guid_unsupported",
        ]
    );
    assert_eq!(
        NativeCompanyFeaturesError::Shape("stock_foreign_child").cause(),
        Some("stock_foreign_child")
    );
    assert_eq!(
        NativeCompanyFeaturesError::CompanyMismatch("books_from").cause(),
        Some("books_from")
    );
    assert_eq!(NativeCompanyFeaturesError::NotOneRow.cause(), None);
}
