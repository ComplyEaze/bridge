//! Safety-property tests for `outstandings_shared`: the company-identity and
//! book-extent read the native outstandings path pins against.

use bridge_tally_protocol::outstandings_shared::{
    parse_company_book_extent_v2, CompanyBookExtentExpectation, OutstandingsError,
};

const AARAV_NAME: &str = "Aarav Trading Company Demo";
const AARAV_GUID: &str = "bb8ad19e-6aef-4239-a917-87fec0c6215e";
const AARAV_NUMBER: &str = "100000";
const AARAV_BOOKS_FROM: &str = "20240401";
const AARAV_ALTMSTID: &str = "328";
const COMPANY_EXTENT_V2: &str =
    include_str!("fixtures/agent/native-company-book-extents-with-number.utf8.xml");
const SPLIT_COMPANY_NAME: &str = "BRIDGE PROBE B SANDBOX";
const SPLIT_COMPANY_GUID: &str = "ec4454ae-5c4c-4bfa-b3b0-68182a749689";
const SPLIT_COMPANY_NUMBER: &str = "100005";
const SPLIT_BOOKS_FROM: &str = "20250401";

fn split_expectation() -> CompanyBookExtentExpectation {
    CompanyBookExtentExpectation::new(
        SPLIT_COMPANY_NAME.to_string(),
        SPLIT_COMPANY_GUID.to_string(),
        SPLIT_COMPANY_NUMBER.to_string(),
        SPLIT_BOOKS_FROM.to_string(),
    )
    .expect("captured full tuple is a valid expectation")
}

fn company_row<'a>(xml: &'a str, name: &str) -> &'a str {
    let start = xml
        .find(&format!(r#"<COMPANY NAME="{name}""#))
        .expect("captured company row exists");
    let end = start
        + xml[start..]
            .find("</COMPANY>")
            .expect("captured company row closes")
        + "</COMPANY>".len();
    &xml[start..end]
}

#[test]
fn v2_captured_extent_selects_the_exact_tuple_among_same_guid_siblings() {
    let extent = parse_company_book_extent_v2(COMPANY_EXTENT_V2, &split_expectation())
        .expect("captured V2 full tuple selects the intended split book");
    assert_eq!(extent.company().name(), SPLIT_COMPANY_NAME);
    assert_eq!(extent.company().guid(), SPLIT_COMPANY_GUID);
    assert_eq!(extent.books_from().as_str(), SPLIT_BOOKS_FROM);
    assert_eq!(COMPANY_EXTENT_V2.matches("<COMPANYNUMBER").count(), 16);
    assert_eq!(COMPANY_EXTENT_V2.matches(SPLIT_COMPANY_GUID).count(), 2);
}

#[test]
fn v2_canonicalizes_a_captured_guid_case_variant_for_extent_brackets() {
    let target = company_row(COMPANY_EXTENT_V2, SPLIT_COMPANY_NAME);
    let upper_guid = SPLIT_COMPANY_GUID.to_ascii_uppercase();
    let changed_target = target.replacen(SPLIT_COMPANY_GUID, &upper_guid, 1);
    assert_ne!(
        changed_target, target,
        "captured GUID case mutation must apply"
    );
    let changed = COMPANY_EXTENT_V2.replacen(target, &changed_target, 1);
    assert_ne!(
        changed, COMPANY_EXTENT_V2,
        "captured response mutation must apply"
    );

    let extent = parse_company_book_extent_v2(&changed, &split_expectation())
        .expect("GUID casing alone must not alter the selected extent");
    assert_eq!(extent.company().guid(), SPLIT_COMPANY_GUID);
}

#[test]
fn v2_full_tuple_expectation_refuses_noncanonical_caller_numbers() {
    for company_number in [
        "",
        " 100005",
        "100005 ",
        "100005\n",
        "1.0",
        "१२३",
        "12345678901234567",
    ] {
        assert!(
            CompanyBookExtentExpectation::new(
                SPLIT_COMPANY_NAME.to_string(),
                SPLIT_COMPANY_GUID.to_string(),
                company_number.to_string(),
                SPLIT_BOOKS_FROM.to_string(),
            )
            .is_err(),
            "{company_number:?}"
        );
    }
}

#[test]
fn v2_trims_the_wire_number_without_integer_coercion() {
    let target = company_row(COMPANY_EXTENT_V2, SPLIT_COMPANY_NAME);
    let changed_target = target.replacen(" 100005</COMPANYNUMBER>", " 000001 </COMPANYNUMBER>", 1);
    assert_ne!(changed_target, target);
    let changed = COMPANY_EXTENT_V2.replacen(target, &changed_target, 1);
    let expectation = CompanyBookExtentExpectation::new(
        SPLIT_COMPANY_NAME.to_string(),
        SPLIT_COMPANY_GUID.to_string(),
        "000001".to_string(),
        SPLIT_BOOKS_FROM.to_string(),
    )
    .expect("canonical leading-zero number remains a string");

    assert!(parse_company_book_extent_v2(&changed, &expectation).is_ok());
}

#[test]
fn v2_refuses_missing_or_malformed_observed_company_number_without_fallback() {
    let target = company_row(COMPANY_EXTENT_V2, SPLIT_COMPANY_NAME);
    for replacement in ["", r#"<COMPANYNUMBER TYPE="Number"> abc</COMPANYNUMBER>"#] {
        let changed_target = target.replacen(
            r#"<COMPANYNUMBER TYPE="Number"> 100005</COMPANYNUMBER>"#,
            replacement,
            1,
        );
        assert_ne!(changed_target, target);
        let changed = COMPANY_EXTENT_V2.replacen(target, &changed_target, 1);
        assert_eq!(
            parse_company_book_extent_v2(&changed, &split_expectation()),
            Err(OutstandingsError::InvalidResponse(
                if replacement.is_empty() {
                    "company_number_missing"
                } else {
                    "company_number_invalid"
                }
            ))
        );
    }
}

#[test]
fn v2_refuses_missing_or_malformed_observed_books_from_without_fallback() {
    let target = company_row(COMPANY_EXTENT_V2, SPLIT_COMPANY_NAME);
    for replacement in ["", r#"<BOOKSFROM TYPE="Date">not-a-date</BOOKSFROM>"#] {
        let changed_target = target.replacen(
            r#"<BOOKSFROM TYPE="Date">20250401</BOOKSFROM>"#,
            replacement,
            1,
        );
        assert_ne!(changed_target, target);
        let changed = COMPANY_EXTENT_V2.replacen(target, &changed_target, 1);
        assert!(
            parse_company_book_extent_v2(&changed, &split_expectation()).is_err(),
            "{replacement:?} must not select another same-GUID row"
        );
    }
}

#[test]
fn v2_refuses_repeated_observed_company_number_field() {
    let target = company_row(COMPANY_EXTENT_V2, SPLIT_COMPANY_NAME);
    let repeated = target.replacen(
        r#"<COMPANYNUMBER TYPE="Number"> 100005</COMPANYNUMBER>"#,
        r#"<COMPANYNUMBER TYPE="Number"> 100005</COMPANYNUMBER><COMPANYNUMBER TYPE="Number"> 100005</COMPANYNUMBER>"#,
        1,
    );
    assert_ne!(repeated, target);
    let repeated = COMPANY_EXTENT_V2.replacen(target, &repeated, 1);
    assert!(parse_company_book_extent_v2(&repeated, &split_expectation()).is_err());
}

#[test]
fn v2_refuses_duplicate_exact_tuple_and_presentation_collision() {
    let target = company_row(COMPANY_EXTENT_V2, SPLIT_COMPANY_NAME);
    let duplicate =
        COMPANY_EXTENT_V2.replacen("</COLLECTION>", &format!("{target}</COLLECTION>"), 1);
    assert_eq!(
        parse_company_book_extent_v2(&duplicate, &split_expectation()),
        Err(OutstandingsError::InvalidResponse(
            "company_identity_ambiguous"
        ))
    );

    let collision = target
        .replace(SPLIT_COMPANY_NAME, "bridge probe b sandbox")
        .replacen(" 100005</COMPANYNUMBER>", " 100099</COMPANYNUMBER>", 1);
    assert_ne!(collision, target);
    let collision =
        COMPANY_EXTENT_V2.replacen("</COLLECTION>", &format!("{collision}</COLLECTION>"), 1);
    assert_eq!(
        parse_company_book_extent_v2(&collision, &split_expectation()),
        Err(OutstandingsError::InvalidResponse(
            "company_identity_presentation_collision"
        ))
    );
}

#[test]
fn v2_refuses_same_guid_selector_attribute_with_a_different_nested_name() {
    let target = company_row(COMPANY_EXTENT_V2, SPLIT_COMPANY_NAME);
    let sibling = target.replacen(
        &format!(r#"<NAME TYPE="String">{SPLIT_COMPANY_NAME}</NAME>"#),
        r#"<NAME TYPE="String">BRIDGE PROBE B OTHER BOOK</NAME>"#,
        1,
    );
    assert_ne!(sibling, target, "captured nested-name mutation must apply");
    let collision =
        COMPANY_EXTENT_V2.replacen("</COLLECTION>", &format!("{sibling}</COLLECTION>"), 1);
    assert_ne!(
        collision, COMPANY_EXTENT_V2,
        "captured sibling insertion must apply"
    );

    assert_eq!(
        parse_company_book_extent_v2(&collision, &split_expectation()),
        Err(OutstandingsError::InvalidResponse(
            "company_identity_presentation_collision"
        ))
    );
}

fn aarav_expectation(guid: &str) -> CompanyBookExtentExpectation {
    CompanyBookExtentExpectation::new(
        AARAV_NAME.to_string(),
        guid.to_string(),
        AARAV_NUMBER.to_string(),
        AARAV_BOOKS_FROM.to_string(),
    )
    .expect("captured full tuple is a valid expectation")
}

#[test]
fn v2_company_pin_is_created_only_after_live_identity_matches() {
    let extent = parse_company_book_extent_v2(COMPANY_EXTENT_V2, &aarav_expectation(AARAV_GUID))
        .expect("captured V2 extent selects the Aarav book");
    assert_eq!(extent.company().name(), AARAV_NAME);
    assert_eq!(extent.company().guid(), AARAV_GUID);
    assert!(matches!(
        parse_company_book_extent_v2(
            COMPANY_EXTENT_V2,
            &aarav_expectation("00000000-0000-4000-8000-000000000001")
        ),
        Err(OutstandingsError::CompanyIdentityMismatch)
    ));
}

/// The core-window bracket in `connector.rs` compares two `CompanyBookExtent`
/// reads by `PartialEq` and rejects the window when they differ. `ALTMSTID` --
/// Tally's master alteration high-water mark -- has to move that comparison
/// when a GROUP or LEDGER master is edited mid-window, because nothing else in
/// the struct does.
#[test]
fn v2_extents_differing_only_in_master_high_water_are_not_equal() {
    let expectation = aarav_expectation(AARAV_GUID);
    let opening = parse_company_book_extent_v2(COMPANY_EXTENT_V2, &expectation)
        .expect("captured V2 extent parses");
    assert_eq!(
        opening.master_alter_id_high_water().map(|w| w.get()),
        Some(
            AARAV_ALTMSTID
                .parse::<u64>()
                .expect("test constant is a valid u64")
        )
    );
    let edited_master_response = COMPANY_EXTENT_V2.replacen(
        &format!(r#"<ALTMSTID TYPE="Number"> {AARAV_ALTMSTID}</ALTMSTID>"#),
        r#"<ALTMSTID TYPE="Number"> 329</ALTMSTID>"#,
        1,
    );
    assert_ne!(
        edited_master_response, COMPANY_EXTENT_V2,
        "the replacement must actually change the response for this test to prove anything"
    );
    let closing = parse_company_book_extent_v2(&edited_master_response, &expectation)
        .expect("edited capture still parses");

    assert_eq!(
        opening.voucher_alter_id_high_water(),
        closing.voucher_alter_id_high_water()
    );
    assert_eq!(opening.books_from(), closing.books_from());
    assert_eq!(opening.last_voucher_date(), closing.last_voucher_date());
    assert_ne!(
        opening.master_alter_id_high_water(),
        closing.master_alter_id_high_water()
    );
    assert_ne!(
        opening, closing,
        "a master-only edit must be visible to the whole-struct PartialEq the bracket compares"
    );
}
