use chrono::NaiveDate;

use bridge_tally_protocol::xml_read_profiles::{
    CompanyNameRefusal, ValidatedCompanyName, MAX_COMPANY_NAME_BYTES,
};

/// Whether a request may carry this company name, exactly as given. The rule
/// is `ValidatedCompanyName`'s; this only words its refusal for the app (#2433).
pub fn validate_company_name(value: &str) -> Result<(), String> {
    match ValidatedCompanyName::refusal(value) {
        None => Ok(()),
        Some(CompanyNameRefusal::Blank) => {
            Err("Select a Tally company before requesting company data".to_string())
        }
        Some(CompanyNameRefusal::OverLimit | CompanyNameRefusal::ControlCharacter) => Err(format!(
            "Tally company name must be at most {MAX_COMPANY_NAME_BYTES} bytes and contain no control characters"
        )),
    }
}

pub(crate) fn is_valid_company_number(value: &str) -> bool {
    (1..=16).contains(&value.len()) && value.bytes().all(|byte| byte.is_ascii_digit())
}

pub fn normalize_company_guid(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.len() > 256
        || !trimmed.is_ascii()
        || trimmed.chars().any(char::is_control)
    {
        return Err(
            "Tally company GUID must be printable ASCII, at most 256 bytes, and contain no control characters"
                .to_string(),
        );
    }
    Ok(trimmed.to_string())
}

pub fn validate_date_range(from: &str, to: &str) -> Result<(), String> {
    fn parse(label: &str, value: &str) -> Result<NaiveDate, String> {
        if value.len() != 8 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(format!("{label} must use the YYYYMMDD format"));
        }
        NaiveDate::parse_from_str(value, "%Y%m%d")
            .map_err(|_| format!("{label} must be a valid calendar date"))
    }

    let from_date = parse("From date", from)?;
    let to_date = parse("To date", to)?;
    if from_date > to_date {
        return Err("From date must be on or before To date".to_string());
    }
    Ok(())
}

#[cfg(test)]
#[path = "validators_tests.rs"]
mod tests;
