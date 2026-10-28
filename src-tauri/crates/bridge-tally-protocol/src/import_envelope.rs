//! The Tally import envelope, rendered once for every writer.
//!
//! Vouchers (`Vouchers`) and masters (`All Masters`) travel in the same
//! envelope and differ only in the report name, so they share this one
//! function. The company is escaped here, once; `messages` is inserted as
//! given, so the caller must already have escaped the values inside it.
use crate::xml_text::escape_text;

/// The report an import envelope names. A closed set, so no caller can put
/// arbitrary text in `REPORTNAME`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportReport {
    Vouchers,
    AllMasters,
}

impl ImportReport {
    const fn tally_name(self) -> &'static str {
        match self {
            Self::Vouchers => "Vouchers",
            Self::AllMasters => "All Masters",
        }
    }
}

/// The `Import Data` envelope around `messages`, for `company` and `report`.
pub fn render_import_envelope(company: &str, report: ImportReport, messages: &str) -> String {
    let report_name = report.tally_name();
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>{report_name}</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{messages}</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>",
        escape_text(company)
    )
}

#[cfg(test)]
#[path = "import_envelope_tests.rs"]
mod tests;
