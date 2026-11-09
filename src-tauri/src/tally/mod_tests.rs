use super::*;

const CAPTURED_EXTENT_V2: &str = include_str!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
);
const CAPTURED_NAME: &str = "BRIDGE PROBE B SANDBOX";
const CAPTURED_GUID: &str = "ec4454ae-5c4c-4bfa-b3b0-68182a749689";
const CAPTURED_NUMBER: &str = "100005";
const CAPTURED_BOOKS_FROM: &str = "20250401";

fn captured_identity(xml: &str, guid: &str) -> VerifiedCompanyIdentity {
    let companies = bridge_tally_protocol::parse_companies_from_collection(xml)
        .expect("captured CompanyBookExtentV2 response parses as a Company collection");
    VerifiedCompanyIdentity::from_observed_companies(
        CAPTURED_NAME.to_string(),
        test_company_guid(guid),
        CAPTURED_NUMBER.to_string(),
        CAPTURED_BOOKS_FROM.to_string(),
        &companies,
    )
    .expect("captured full tuple remains uniquely observed")
}

#[test]
fn verified_identity_canonicalizes_captured_guid_case_for_closing_comparison() {
    let target_start = CAPTURED_EXTENT_V2
        .find(&format!(r#"<COMPANY NAME="{CAPTURED_NAME}""#))
        .expect("captured target company row exists");
    let target_end = target_start
        + CAPTURED_EXTENT_V2[target_start..]
            .find("</COMPANY>")
            .expect("captured target company row closes")
        + "</COMPANY>".len();
    let target = &CAPTURED_EXTENT_V2[target_start..target_end];
    let upper_guid = CAPTURED_GUID.to_ascii_uppercase();
    let changed_target = target.replacen(CAPTURED_GUID, &upper_guid, 1);
    assert_ne!(
        changed_target, target,
        "captured GUID case mutation must apply"
    );
    let closing = CAPTURED_EXTENT_V2.replacen(target, &changed_target, 1);
    assert_ne!(
        closing, CAPTURED_EXTENT_V2,
        "captured response mutation must apply"
    );

    assert_eq!(
        captured_identity(CAPTURED_EXTENT_V2, CAPTURED_GUID),
        captured_identity(&closing, &upper_guid),
        "GUID casing alone must not make a verified closing identity drift"
    );
}

fn companies_with_guid(guid: &str) -> Vec<TallyCompany> {
    vec![TallyCompany {
        name: CAPTURED_NAME.to_string(),
        guid: Some(guid.to_string()),
        company_number: Some(CAPTURED_NUMBER.to_string()),
        books_from: Some(CAPTURED_BOOKS_FROM.to_string()),
    }]
}

#[test]
fn identity_accepts_a_strict_company_guid_in_either_case_and_stores_lowercase() {
    let upper = CAPTURED_GUID.to_ascii_uppercase();
    let companies = companies_with_guid(&upper);
    let identity = VerifiedCompanyIdentity::from_observed_companies(
        CAPTURED_NAME.to_string(),
        test_company_guid(&upper),
        CAPTURED_NUMBER.to_string(),
        CAPTURED_BOOKS_FROM.to_string(),
        &companies,
    )
    .expect("a strict-shaped GUID in a complete tuple verifies");
    assert_eq!(identity.company_guid(), CAPTURED_GUID);
}

fn observed_row(name: &str, number: &str) -> TallyCompany {
    TallyCompany {
        name: name.to_string(),
        guid: Some(CAPTURED_GUID.to_string()),
        company_number: Some(number.to_string()),
        books_from: Some(CAPTURED_BOOKS_FROM.to_string()),
    }
}

/// Builds the identity for `name` against a list holding exactly that tuple,
/// so only the name itself can refuse it.
fn identity_named(name: &str) -> Result<VerifiedCompanyIdentity, VerifiedCompanyIdentityError> {
    VerifiedCompanyIdentity::from_observed_companies(
        name.to_string(),
        test_company_guid(CAPTURED_GUID),
        CAPTURED_NUMBER.to_string(),
        CAPTURED_BOOKS_FROM.to_string(),
        &[observed_row(name, CAPTURED_NUMBER)],
    )
}

#[test]
fn identity_refuses_a_name_no_request_can_carry() {
    for name in [
        "A".repeat(256),
        "line\nbreak".to_string(),
        "tab\tinside".to_string(),
        "nul\u{0}inside".to_string(),
        "delete\u{7f}inside".to_string(),
        "Client Book\r".to_string(),
        String::new(),
        " ".to_string(),
        "   ".to_string(),
        "\u{3000}".to_string(),
    ] {
        assert_eq!(
            identity_named(&name).err(),
            Some(VerifiedCompanyIdentityError::InvalidCompanyName),
            "{name:?}"
        );
    }
}

/// The limit is the existing 255-byte rule, counted in bytes, not characters.
#[test]
fn identity_admits_a_255_byte_name_and_refuses_one_of_256() {
    let ascii = "A".repeat(255);
    let devanagari = "क".repeat(85);
    assert_eq!(devanagari.len(), 255);
    for name in [&ascii, &devanagari] {
        let identity = identity_named(name).expect("a 255-byte name is admitted");
        assert_eq!(identity.display_name(), name.as_str());
    }
    for name in [format!("{ascii}A"), format!("{devanagari}A")] {
        assert_eq!(name.len(), 256);
        assert_eq!(
            identity_named(&name).err(),
            Some(VerifiedCompanyIdentityError::InvalidCompanyName)
        );
    }
}

/// Markup characters, quotes, Devanagari and edge spaces are all names Tally
/// can report; the identity holds them byte-exact, untrimmed and unescaped.
#[test]
fn identity_holds_a_markup_and_devanagari_name_byte_exact() {
    let name = "  R&D <Co> \"1\" 'Q' मुंबई शाखा  ";
    let identity = identity_named(name).expect("a sendable name is admitted");
    assert_eq!(identity.display_name().as_bytes(), name.as_bytes());
    assert_eq!(identity.company_name().as_str().as_bytes(), name.as_bytes());
    assert!(identity.matches_observed_company(&observed_row(name, CAPTURED_NUMBER)));
}

/// The name is checked first: a bad name in a list that would also be
/// ambiguous, duplicated or missing, or with a bad number, is refused as a name.
#[test]
fn a_bad_name_is_refused_before_the_number_and_ambiguity_checks() {
    let name = "A".repeat(256);
    let sibling = observed_row(&format!(" {} ", name.to_ascii_lowercase()), "2");
    let target = observed_row(&name, CAPTURED_NUMBER);
    for (number, companies) in [
        (CAPTURED_NUMBER, vec![target.clone(), sibling]),
        (CAPTURED_NUMBER, vec![target.clone(), target.clone()]),
        (CAPTURED_NUMBER, Vec::new()),
        ("not-a-number", vec![target]),
    ] {
        assert_eq!(
            VerifiedCompanyIdentity::from_observed_companies(
                name.clone(),
                test_company_guid(CAPTURED_GUID),
                number.to_string(),
                CAPTURED_BOOKS_FROM.to_string(),
                &companies,
            )
            .err(),
            Some(VerifiedCompanyIdentityError::InvalidCompanyName),
            "{number} with {} rows",
            companies.len()
        );
    }
    // The same ambiguous shape with a sendable name reaches the ambiguity check.
    let name = "Client Book";
    assert_eq!(
        VerifiedCompanyIdentity::from_observed_companies(
            name.to_string(),
            test_company_guid(CAPTURED_GUID),
            CAPTURED_NUMBER.to_string(),
            CAPTURED_BOOKS_FROM.to_string(),
            &[
                observed_row(name, CAPTURED_NUMBER),
                observed_row(" client book ", "2"),
            ],
        )
        .err(),
        Some(VerifiedCompanyIdentityError::DisplayScopeAmbiguous)
    );
}

#[test]
fn the_name_refusal_prints_no_name() {
    let name = "SYNTHETIC CLIENT NAME\n";
    let error = identity_named(name).expect_err("a control character is refused");
    assert_eq!(error, VerifiedCompanyIdentityError::InvalidCompanyName);
    for printed in [format!("{error:?}"), format!("{error}")] {
        assert!(!printed.contains("SYNTHETIC"), "{printed}");
    }
    let identity = identity_named("SYNTHETIC CLIENT NAME").unwrap();
    assert!(!format!("{identity:?}").contains("SYNTHETIC"));
}
