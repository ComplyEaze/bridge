use super::{
    normalize_company_guid, normalize_company_name, validate_company_name, validate_date_range,
};

#[test]
fn validates_company_selection() {
    assert!(validate_company_name("Synthetic Company").is_ok());
    assert!(validate_company_name("  ").is_err());
    assert!(validate_company_name("Synthetic\nCompany").is_err());
    assert!(validate_company_name(&"x".repeat(256)).is_err());
    assert_eq!(
        normalize_company_name("  Synthetic Company  ").unwrap(),
        "Synthetic Company"
    );
    assert_eq!(normalize_company_guid("  guid-1  ").unwrap(), "guid-1");
    assert!(normalize_company_guid("guid\n1").is_err());
    assert!(normalize_company_guid(&"g".repeat(257)).is_err());
}

#[test]
fn validates_tally_date_ranges() {
    assert!(validate_date_range("20260101", "20260131").is_ok());
    assert!(validate_date_range("20260229", "20260301").is_err());
    assert!(validate_date_range("20260430", "20260401").is_err());
    assert!(validate_date_range("2026-04-01", "20260430").is_err());
}
