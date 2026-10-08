//! The company's base currency symbol and three of its F11 settings, read from
//! the GUID-filtered Company collection (#1231, protocol reference §12a.18).
//!
//! The request is the one captured on two synthetic books: it fetches 17 flags
//! and the company's identity fields, and a test pins it byte-for-byte to the
//! committed captures. Only three flags are read from the answer (cost
//! centres, GST, batch-wise): they are the only ones that ever differed across
//! the three books read, and only the cost-centre setting has been compared
//! with Tally's own screen (on those synthetic books). The other flags stay in the request so that it is
//! the captured one, and are skipped.
//!
//! A setting is what Tally's Company collection holds today. It says nothing
//! about what the books contain or what the setting was during a period (a
//! cost-centre allocation is stored although the setting reads No, §12a.17).
use crate::native_stock_summary::{read_collection, read_row_fields, NativeStockError};
use crate::xml_text::escape_text as xml_escape;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

/// The longest currency symbol read. Symbols seen are one to three characters;
/// the bound is this module's own choice, so a long text is refused, not shown.
const MAX_SYMBOL_CHARS: usize = 16;

const FETCH: &str = "Name, GUID, CompanyNumber, BooksFrom, LastVoucherDate, ALTVCHID, ALTMSTID, CURRENCYNAME, ISACCOUNTINGON, ISINVENTORYON, ISINTEGRATED, ISBILLWISEON, ISALLBILLWISEON, ISCOSTCENTRESON, ISBATCHWISEON, ISGSTON, ISGSTCLASSIFON, ISTDSON, ISTCSON, ISPAYROLLON, ISEDITLOGON, ISJOBCOSTINGON, ISCOSTTRACKINGON, ISTRACKVOUCHERSON, ISISOCURRENCYAPPLICABLE";

/// The row's children this module reads; every other child is skipped.
const FIELDS: [&str; 8] = [
    "GUID",
    "NAME",
    "COMPANYNUMBER",
    "BOOKSFROM",
    "CURRENCYNAME",
    "ISCOSTCENTRESON",
    "ISGSTON",
    "ISBATCHWISEON",
];

/// The Company collection of the captured request, filtered to one GUID: a
/// `Company` collection ignores `SVCURRENTCOMPANY` and returns every loaded
/// company (§12a.7), so the filter makes the answer this company's alone. A
/// GUID that could break the formula's string is refused, not escaped.
pub fn render_company_features_request(
    company: &str,
    company_guid: &str,
) -> Result<String, NativeCompanyFeaturesError> {
    if company_guid.is_empty()
        || !company_guid
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        return Err(NativeCompanyFeaturesError::GuidUnsupported);
    }
    Ok(format!(
        r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>BridgeCompanyFeaturesV1</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE="Formulae" NAME="BridgeCompanyGuidFilter">$GUID = "{company_guid}"</SYSTEM><COLLECTION NAME="BridgeCompanyFeaturesV1" ISMODIFY="No"><TYPE>Company</TYPE><FETCH>{FETCH}</FETCH><FILTERS>BridgeCompanyGuidFilter</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"#,
        company = xml_escape(company),
    ))
}

/// A setting as the Company collection holds it. An element Tally did not
/// send is `NotReported`, never `No`; an element that is empty or says
/// anything but `Yes` or `No` refuses the read instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeSetting {
    Yes,
    No,
    NotReported,
}

/// The company's currency symbol exactly as `CURRENCYNAME` carries it (a
/// symbol such as the rupee sign, not an ISO code). Absent or blank is
/// `NotReported`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeCurrencySymbol {
    Reported(String),
    NotReported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeCompanyFeatures {
    pub cost_centres: NativeSetting,
    pub gst: NativeSetting,
    pub batch_wise: NativeSetting,
    pub base_currency: NativeCurrencySymbol,
}

/// The identity the answer must carry: what the company list and the identity
/// check established for this read. All four must equal the row's.
#[derive(Debug, Clone, Copy)]
pub struct ExpectedCompany<'a> {
    pub guid: &'a str,
    pub name: &'a str,
    pub number: &'a str,
    pub books_from: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeCompanyFeaturesError {
    /// A `STATUS` other than 1, or an `ERROR` or `LINEERROR` element.
    TallyReportedFailure,
    /// The envelope or the row is not the closed shape; the code is the shared
    /// collection reader's, which names where.
    Shape(&'static str),
    /// The filtered collection did not return exactly one `COMPANY` row.
    NotOneRow,
    /// The row's GUID, name, company number or books-from date is not the one
    /// this read was made for (or is absent); the field is named.
    CompanyMismatch(&'static str),
    /// A setting element that is empty or says anything but `Yes` or `No`.
    SettingInvalid(&'static str),
    /// A currency symbol with a control character or over the bound.
    CurrencyInvalid,
    /// A GUID that could break the filter formula's string.
    GuidUnsupported,
}

impl NativeCompanyFeaturesError {
    /// The refusal's stable code, which holds no data.
    pub fn code(&self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "company_features_tally_reported_failure",
            Self::Shape(_) => "company_features_response_invalid",
            Self::NotOneRow => "company_features_not_one_row",
            Self::CompanyMismatch(_) => "company_features_company_mismatch",
            Self::SettingInvalid("cost_centres") => "company_features_setting_invalid:cost_centres",
            Self::SettingInvalid("gst") => "company_features_setting_invalid:gst",
            Self::SettingInvalid("batch_wise") => "company_features_setting_invalid:batch_wise",
            Self::SettingInvalid(_) => "company_features_setting_invalid",
            Self::CurrencyInvalid => "company_features_currency_invalid",
            Self::GuidUnsupported => "company_features_guid_unsupported",
        }
    }

    /// What sits under the code: the shared reader's own code for a shape
    /// refusal, or the field a mismatch names.
    pub fn cause(&self) -> Option<&'static str> {
        match self {
            Self::Shape(code) => Some(code),
            Self::CompanyMismatch(field) => Some(field),
            _ => None,
        }
    }
}

impl fmt::Display for NativeCompanyFeaturesError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "company features response refused ({})",
            self.code()
        )
    }
}

impl std::error::Error for NativeCompanyFeaturesError {}

impl From<NativeStockError> for NativeCompanyFeaturesError {
    fn from(error: NativeStockError) -> Self {
        match error {
            NativeStockError::TallyReportedFailure => Self::TallyReportedFailure,
            other => Self::Shape(other.code()),
        }
    }
}

/// Parses the answer to [`render_company_features_request`]. Exactly one
/// `COMPANY` row under `ENVELOPE/BODY/DATA/COLLECTION` is admitted (the
/// `CMPINFO` block carries a bare `COMPANY` counter, which is not a row), and
/// its GUID, name, company number and books-from date must equal `expected`'s:
/// year-split siblings share a GUID, so two rows refuse as `NotOneRow`.
pub fn parse_company_features(
    response: &str,
    expected: &ExpectedCompany<'_>,
) -> Result<NativeCompanyFeatures, NativeCompanyFeaturesError> {
    let mut rows = read_collection(response, b"COMPANY", None, |reader, _element| {
        read_row_fields(reader, b"COMPANY", &FIELDS)
    })?;
    if rows.len() != 1 {
        return Err(NativeCompanyFeaturesError::NotOneRow);
    }
    let mut fields = rows.remove(0);
    let same = |fields: &mut HashMap<&'static str, String>,
                key: &'static str,
                label: &'static str,
                want: &str,
                ignore_case: bool| {
        let text = fields.remove(key);
        let text = text.as_deref().map(str::trim);
        let equal = text.is_some_and(|text| {
            if ignore_case {
                text.eq_ignore_ascii_case(want.trim())
            } else {
                text == want.trim()
            }
        });
        if equal {
            Ok(())
        } else {
            Err(NativeCompanyFeaturesError::CompanyMismatch(label))
        }
    };
    same(&mut fields, "GUID", "guid", expected.guid, true)?;
    same(&mut fields, "NAME", "name", expected.name, false)?;
    same(
        &mut fields,
        "COMPANYNUMBER",
        "company_number",
        expected.number,
        false,
    )?;
    same(
        &mut fields,
        "BOOKSFROM",
        "books_from",
        expected.books_from,
        false,
    )?;
    Ok(NativeCompanyFeatures {
        cost_centres: setting(&mut fields, "ISCOSTCENTRESON", "cost_centres")?,
        gst: setting(&mut fields, "ISGSTON", "gst")?,
        batch_wise: setting(&mut fields, "ISBATCHWISEON", "batch_wise")?,
        base_currency: currency(fields.remove("CURRENCYNAME"))?,
    })
}

fn setting(
    fields: &mut HashMap<&'static str, String>,
    key: &'static str,
    label: &'static str,
) -> Result<NativeSetting, NativeCompanyFeaturesError> {
    match fields.remove(key).as_deref().map(str::trim) {
        None => Ok(NativeSetting::NotReported),
        Some("Yes") => Ok(NativeSetting::Yes),
        Some("No") => Ok(NativeSetting::No),
        Some(_) => Err(NativeCompanyFeaturesError::SettingInvalid(label)),
    }
}

fn currency(text: Option<String>) -> Result<NativeCurrencySymbol, NativeCompanyFeaturesError> {
    match text.as_deref().map(str::trim) {
        None | Some("") => Ok(NativeCurrencySymbol::NotReported),
        Some(symbol)
            if symbol.chars().count() > MAX_SYMBOL_CHARS
                || symbol.chars().any(char::is_control) =>
        {
            Err(NativeCompanyFeaturesError::CurrencyInvalid)
        }
        Some(symbol) => Ok(NativeCurrencySymbol::Reported(symbol.to_string())),
    }
}

#[cfg(test)]
#[path = "native_company_features_tests.rs"]
mod tests;
