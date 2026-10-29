//! Closed, read-only XML request profiles shared by the native app and portable tools.
//!
//! The public profile API accepts only validated company and date inputs. The
//! compatibility renderers are intentionally hidden from generated
//! documentation; they preserve the native app's existing string-based
//! function signatures while still exposing only fixed export profiles.

use std::fmt;

use bridge_tally_primitives::TallyDate;
use sha2::{Digest, Sha256};

use crate::encode_tally_xml_request_utf16le;
use crate::outstandings_shared::render_company_book_extent_v2;
use crate::xml_text::escape_text as xml_escape;

const TEMPLATE_COMPANY: &str = "BRIDGE TEMPLATE COMPANY";
const TEMPLATE_FROM: &str = "20000101";
const TEMPLATE_TO: &str = "20000102";

/// A template date as the typed date a date-only renderer takes. The
/// constants above are fixed valid dates, so this cannot fail; it is used only
/// to render a profile's fixed template, never on an input (#861).
fn template_date(value: &'static str) -> TallyDate {
    TallyDate::parse(value).expect("template dates are fixed valid YYYYMMDD constants")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadProfileValidationError {
    CompanyInvalid,
    DateInvalid,
    DateRangeInvalid,
}

impl fmt::Display for ReadProfileValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::CompanyInvalid => "read profile company input was invalid",
            Self::DateInvalid => "read profile date input was invalid",
            Self::DateRangeInvalid => "read profile date range was invalid",
        })
    }
}

impl std::error::Error for ReadProfileValidationError {}

#[derive(Clone, PartialEq, Eq)]
pub struct ValidatedCompanyName(String);

impl ValidatedCompanyName {
    pub fn new(value: impl Into<String>) -> Result<Self, ReadProfileValidationError> {
        let value = value.into();
        if value.trim().is_empty() || value.len() > 255 || value.chars().any(char::is_control) {
            return Err(ReadProfileValidationError::CompanyInvalid);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ValidatedCompanyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ValidatedCompanyName([redacted])")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedDateRange {
    from: TallyDate,
    to: TallyDate,
}

impl ValidatedDateRange {
    pub fn new(
        from_yyyymmdd: impl Into<String>,
        to_yyyymmdd: impl Into<String>,
    ) -> Result<Self, ReadProfileValidationError> {
        let (Ok(from), Ok(to)) = (
            TallyDate::parse(from_yyyymmdd),
            TallyDate::parse(to_yyyymmdd),
        ) else {
            return Err(ReadProfileValidationError::DateInvalid);
        };
        if from > to {
            return Err(ReadProfileValidationError::DateRangeInvalid);
        }
        Ok(Self { from, to })
    }

    pub fn from_yyyymmdd(&self) -> &str {
        self.from.as_str()
    }

    pub fn to_yyyymmdd(&self) -> &str {
        self.to.as_str()
    }

    pub fn from_date(&self) -> &TallyDate {
        &self.from
    }

    pub fn to_date(&self) -> &TallyDate {
        &self.to
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadOnlyProfileId {
    CompanyListV1,
    CompanyListV2,
    CompanyBookExtentV2,
    StandardLedgerIdentityV1,
    StandardLedgerCatalogV1,
    StandardLedgerCatalogV2,
    StandardLedgerCatalogV3,
    AuditCompanyObjectV1,
    InvoiceCompanyHsnV1,
    AuditLedgersV1,
    AuditVouchersV1,
    AuditStockItemsV1,
}

impl ReadOnlyProfileId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompanyListV1 => "company_list_v1",
            Self::CompanyListV2 => "company_list_v2",
            Self::CompanyBookExtentV2 => "company_book_extent_v2",
            Self::StandardLedgerIdentityV1 => "standard_ledger_identity_v1",
            Self::StandardLedgerCatalogV1 => "standard_ledger_catalog_v1",
            Self::StandardLedgerCatalogV2 => "standard_ledger_catalog_v2",
            Self::StandardLedgerCatalogV3 => "standard_ledger_catalog_v3",
            Self::AuditCompanyObjectV1 => "audit_company_object_v1",
            Self::InvoiceCompanyHsnV1 => "invoice_company_hsn_v1",
            Self::AuditLedgersV1 => "audit_ledgers_v1",
            Self::AuditVouchersV1 => "audit_vouchers_v1",
            Self::AuditStockItemsV1 => "audit_stock_items_v1",
        }
    }

    /// SHA-256 of the exact request template rendered with fixed safe
    /// sentinels in every dynamic slot. This changes if any emitted byte in the
    /// fixed profile changes, but is independent of a live company or range.
    pub fn template_sha256(self) -> String {
        sha256_hex(&encode_tally_xml_request_utf16le(&self.template()))
    }

    /// Whether this profile must not be sent to an Education-mode Tally: a
    /// custom report (`TYPE=Data`) whose `$$` function takes a spaced
    /// identifier (IMPLEMENTATION_GUIDE I1). Education answered such a read
    /// with a blocking dialog on the Tally screen, which holds the XML gateway
    /// until someone dismisses it (bridge#45). No profile is one since
    /// `ledgers_v1` was deleted (#1570). A test renders every profile and
    /// checks this against [`first_spaced_function_argument`], so a new
    /// profile of that kind fails until it is listed here and a caller refuses
    /// it with [`EDUCATION_REPORT_FAMILY_UNSUPPORTED`].
    pub fn education_refuses_report_formula(self) -> bool {
        false
    }

    /// The exact request rendered with fixed safe sentinels in every dynamic
    /// slot.
    fn template(self) -> String {
        match self {
            Self::CompanyListV1 => render_company_list(),
            Self::CompanyListV2 => render_company_list_v2(),
            Self::CompanyBookExtentV2 => render_company_book_extent_v2(TEMPLATE_COMPANY),
            Self::StandardLedgerIdentityV1 => render_standard_ledger_identity(TEMPLATE_COMPANY),
            Self::StandardLedgerCatalogV1 => render_standard_ledger_identity(TEMPLATE_COMPANY),
            Self::StandardLedgerCatalogV2 => render_standard_ledger_catalog_v2(TEMPLATE_COMPANY),
            Self::StandardLedgerCatalogV3 => render_standard_ledger_catalog_v3(TEMPLATE_COMPANY),
            Self::AuditCompanyObjectV1 => {
                render_company_object(TEMPLATE_COMPANY, &AUDIT_COMPANY_FETCH)
            }
            Self::InvoiceCompanyHsnV1 => {
                render_company_object(TEMPLATE_COMPANY, &INVOICE_COMPANY_FETCH)
            }
            Self::AuditLedgersV1 => {
                render_audit_ledgers(TEMPLATE_COMPANY, TEMPLATE_FROM, TEMPLATE_TO)
            }
            Self::AuditVouchersV1 => render_audit_vouchers(
                TEMPLATE_COMPANY,
                &template_date(TEMPLATE_FROM),
                &template_date(TEMPLATE_TO),
            ),
            Self::AuditStockItemsV1 => {
                render_audit_stock_items(TEMPLATE_COMPANY, TEMPLATE_FROM, TEMPLATE_TO)
            }
        }
    }
}

/// The safe code for a read refused before dispatch because the endpoint is
/// in Education mode and the request's TDL is one Education cannot parse
/// ([`ReadOnlyProfileId::education_refuses_report_formula`]; bridge#45).
pub const EDUCATION_REPORT_FAMILY_UNSUPPORTED: &str = "education_report_family_unsupported";

/// The first `$$Function:argument` in `tdl` whose argument holds whitespace,
/// read the way `scripts/check-tally-request-builder-hazards.mjs` reads a
/// source literal: a quoted argument is only its quoted text (so
/// `$$Date:"20260401" AND ...` is not a hit), and an unquoted one, or one
/// whose quote never closes, runs to the next `<` or line break. Tests use it to keep every Education refusal in
/// step with the TDL a builder actually renders.
#[doc(hidden)]
pub fn first_spaced_function_argument(tdl: &str) -> Option<String> {
    let bytes = tdl.as_bytes();
    let mut at = 0;
    while let Some(offset) = tdl[at..].find("$$") {
        let start = at + offset;
        let mut name_end = start + 2;
        if name_end < bytes.len()
            && (bytes[name_end].is_ascii_alphabetic() || bytes[name_end] == b'_')
        {
            name_end += 1;
            while name_end < bytes.len()
                && (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'_')
            {
                name_end += 1;
            }
            if bytes.get(name_end) == Some(&b':') {
                let rest = &tdl[name_end + 1..];
                // A quote with no closing quote is scanned as unquoted, as
                // the script does, so a malformed literal is still checked.
                let closed = rest
                    .strip_prefix('"')
                    .and_then(|quoted| quoted.find('"').map(|end| &quoted[..end]));
                let argument = closed
                    .unwrap_or_else(|| &rest[..rest.find(['<', '\n', '\r']).unwrap_or(rest.len())]);
                if argument.chars().any(char::is_whitespace) {
                    return Some(tdl[start..name_end + 1].to_string() + argument);
                }
            }
        }
        at = start + 2;
    }
    None
}

#[derive(Debug, Clone, Copy)]
pub enum ReadOnlyProfile<'a> {
    CompanyListV1,
    /// Tally's documented `Company` collection (`TYPE=Collection`), fetching
    /// identity plus fixed gateway product/mode evidence. Unlike
    /// `CompanyListV1`'s custom TDL report,
    /// this returns the ordinary shaped `HEADER/STATUS=1` success envelope,
    /// so it can satisfy the trust check instead of being parsed as an
    /// unverified direct report.
    CompanyListV2,
    /// A versioned Company collection that includes the observed company
    /// number required to distinguish year-split sibling books.
    CompanyBookExtentV2 {
        company: &'a ValidatedCompanyName,
    },
    /// A narrow compatibility bootstrap for Tally responders that reject
    /// Bridge's custom report profile but accept the documented built-in
    /// `List of Ledgers` collection. Its output is parsed only for repeated
    /// computed company context; ledger data is discarded.
    StandardLedgerIdentityV1 {
        company: &'a ValidatedCompanyName,
    },
    /// A scoped compatibility catalog that returns only ledger names and
    /// parents after independently validating the repeated company context.
    /// It is not the custom Bridge ledger export and cannot qualify a sync.
    StandardLedgerCatalogV1 {
        company: &'a ValidatedCompanyName,
    },
    /// V1's request with each ledger's `ISBILLWISEON` fetched in the same row,
    /// so the import family learns which ledgers are maintained bill by bill from the
    /// catalogue read it already makes. Not for `source_draft`, presence or
    /// the voucher and bill reads, which keep V1.
    StandardLedgerCatalogV2 {
        company: &'a ValidatedCompanyName,
    },
    /// V2's request with each ledger's `ALTERID` fetched in the same row, so a
    /// later read can tell which ledgers changed since the catalogue was taken.
    /// Request only: no response parser exists for it yet, because no V3
    /// answer has been captured from a real Tally.
    StandardLedgerCatalogV3 {
        company: &'a ValidatedCompanyName,
    },
    /// The `company` part of a tally-read v1 read: a single-object export of
    /// the one named company. See [`AUDIT_COMPANY_FETCH`].
    AuditCompanyObjectV1 {
        company: &'a ValidatedCompanyName,
    },
    /// The company object an invoice build reads for the company-level
    /// HSN/SAC and its settings. See [`INVOICE_COMPANY_FETCH`].
    InvoiceCompanyHsnV1 {
        company: &'a ValidatedCompanyName,
    },
    /// The `ledgers` part of a tally-read v1 read, for the audit period.
    AuditLedgersV1 {
        company: &'a ValidatedCompanyName,
        period: &'a ValidatedDateRange,
    },
    /// One `vouchers` part of a tally-read v1 read: one date window.
    AuditVouchersV1 {
        company: &'a ValidatedCompanyName,
        window: &'a ValidatedDateRange,
    },
    /// The `stock_items` part of a tally-read v1 read, for the audit period.
    AuditStockItemsV1 {
        company: &'a ValidatedCompanyName,
        period: &'a ValidatedDateRange,
    },
}

impl ReadOnlyProfile<'_> {
    pub fn id(self) -> ReadOnlyProfileId {
        match self {
            Self::CompanyListV1 => ReadOnlyProfileId::CompanyListV1,
            Self::CompanyListV2 => ReadOnlyProfileId::CompanyListV2,
            Self::CompanyBookExtentV2 { .. } => ReadOnlyProfileId::CompanyBookExtentV2,
            Self::StandardLedgerIdentityV1 { .. } => ReadOnlyProfileId::StandardLedgerIdentityV1,
            Self::StandardLedgerCatalogV1 { .. } => ReadOnlyProfileId::StandardLedgerCatalogV1,
            Self::StandardLedgerCatalogV2 { .. } => ReadOnlyProfileId::StandardLedgerCatalogV2,
            Self::StandardLedgerCatalogV3 { .. } => ReadOnlyProfileId::StandardLedgerCatalogV3,
            Self::AuditCompanyObjectV1 { .. } => ReadOnlyProfileId::AuditCompanyObjectV1,
            Self::InvoiceCompanyHsnV1 { .. } => ReadOnlyProfileId::InvoiceCompanyHsnV1,
            Self::AuditLedgersV1 { .. } => ReadOnlyProfileId::AuditLedgersV1,
            Self::AuditVouchersV1 { .. } => ReadOnlyProfileId::AuditVouchersV1,
            Self::AuditStockItemsV1 { .. } => ReadOnlyProfileId::AuditStockItemsV1,
        }
    }

    pub fn template_sha256(self) -> String {
        self.id().template_sha256()
    }

    pub fn render(self) -> String {
        match self {
            Self::CompanyListV1 => render_company_list(),
            Self::CompanyListV2 => render_company_list_v2(),
            Self::CompanyBookExtentV2 { company } => {
                render_company_book_extent_v2(company.as_str())
            }
            Self::StandardLedgerIdentityV1 { company } => {
                render_standard_ledger_identity(company.as_str())
            }
            Self::StandardLedgerCatalogV1 { company } => {
                render_standard_ledger_identity(company.as_str())
            }
            Self::StandardLedgerCatalogV2 { company } => {
                render_standard_ledger_catalog_v2(company.as_str())
            }
            Self::StandardLedgerCatalogV3 { company } => {
                render_standard_ledger_catalog_v3(company.as_str())
            }
            Self::AuditCompanyObjectV1 { company } => {
                render_company_object(company.as_str(), &AUDIT_COMPANY_FETCH)
            }
            Self::InvoiceCompanyHsnV1 { company } => {
                render_company_object(company.as_str(), &INVOICE_COMPANY_FETCH)
            }
            Self::AuditLedgersV1 { company, period } => render_audit_ledgers(
                company.as_str(),
                period.from_yyyymmdd(),
                period.to_yyyymmdd(),
            ),
            Self::AuditVouchersV1 { company, window } => {
                render_audit_vouchers(company.as_str(), window.from_date(), window.to_date())
            }
            Self::AuditStockItemsV1 { company, period } => render_audit_stock_items(
                company.as_str(),
                period.from_yyyymmdd(),
                period.to_yyyymmdd(),
            ),
        }
    }
}

/// Compatibility seam for the native app's existing function signatures.
/// These functions still expose only the three fixed read-only profiles and
/// XML-escape every dynamic value; no caller-provided XML can be dispatched.
#[doc(hidden)]
pub mod compatibility {
    pub fn company_list_request() -> String {
        super::render_company_list()
    }

    pub fn standard_ledger_identity_request(company: &str) -> String {
        super::render_standard_ledger_identity(company)
    }

    pub fn standard_ledger_catalog_request(company: &str) -> String {
        super::render_standard_ledger_identity(company)
    }
}

fn render_company_list() -> String {
    r#"
<ENVELOPE>
    <HEADER>
        <VERSION>1</VERSION>
        <TALLYREQUEST>Export</TALLYREQUEST>
        <TYPE>Data</TYPE>
        <ID>Company Report</ID>
    </HEADER>
    <BODY>
        <DESC>
            <STATICVARIABLES>
                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
            </STATICVARIABLES>
            <TDL>
                <TDLMESSAGE>
                    <REPORT NAME="Company Report">
                        <FORMS>Company Form</FORMS>
                        <TITLE>"Company Details"</TITLE>
                    </REPORT>
                    <FORM NAME="Company Form">
                        <TOPPARTS>Company Part</TOPPARTS>
                        <HEIGHT>100% Page</HEIGHT>
                        <WIDTH>100% Page</WIDTH>
                    </FORM>
                    <PART NAME="Company Part">
                        <TOPLINES>Company Header, Company Details</TOPLINES>
                        <REPEAT>Company Details : CompanyCollection</REPEAT>
                        <SCROLLED>Vertical</SCROLLED>
                        <COMMONBORDERS>Yes</COMMONBORDERS>
                    </PART>
                    <LINE NAME="Company Header">
                        <LEFTFIELDS>
                            Company Name Header, Company GUID Header
                        </LEFTFIELDS>
                    </LINE>
                    <FIELD NAME="Company Name Header"><SET>"Company Name"</SET></FIELD>
                    <FIELD NAME="Company GUID Header"><SET>"Company GUID"</SET></FIELD>
                    <LINE NAME="Company Details">
                        <LEFTFIELDS>
                            Company Name Field, Company GUID Field
                        </LEFTFIELDS>
                        <XMLTAG>"CompanyInfo"</XMLTAG>
                    </LINE>
                    <FIELD NAME="Company Name Field"><SET>$Name</SET></FIELD>
                    <FIELD NAME="Company GUID Field"><SET>$GUID</SET></FIELD>
                    <COLLECTION NAME="CompanyCollection">
                        <TYPE>Company</TYPE>
                        <FETCH>Name, GUID</FETCH>
                    </COLLECTION>
                </TDLMESSAGE>
            </TDL>
        </DESC>
    </BODY>
</ENVELOPE>
"#
    .trim()
    .to_string()
}

/// Tally's documented `Company` collection. It returns company identity plus
/// endpoint-wide product/mode facts from fixed, no-space `$$LicenseInfo`
/// functions. Those facts are parsed separately so a missing capability field
/// cannot make ordinary company discovery unavailable.
/// Unlike `render_company_list`'s custom TDL report — which Tally answers
/// with a bare `<ENVELOPE><COMPANYINFO>...` document carrying no
/// `HEADER`/`STATUS` at all — a `TYPE=Collection` export returns the
/// ordinary shaped success envelope, so its response can satisfy the same
/// `HEADER/STATUS=1` trust check every other export profile requires.
// Product, mode, tier and release are observed endpoint facts; see
// TALLY_PROTOCOL_REFERENCE.md §3.1 for the captured qualification scope.
fn render_company_list_v2() -> String {
    r#"
<ENVELOPE>
    <HEADER>
        <VERSION>1</VERSION>
        <TALLYREQUEST>Export</TALLYREQUEST>
        <TYPE>Collection</TYPE>
        <ID>BridgeCompanyExtent</ID>
    </HEADER>
    <BODY>
        <DESC>
            <STATICVARIABLES>
                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
            </STATICVARIABLES>
            <TDL>
                <TDLMESSAGE>
                    <COLLECTION NAME="BridgeCompanyExtent" ISMODIFY="No" ISINITIALIZE="Yes">
                        <TYPE>Company</TYPE>
                        <NATIVEMETHOD>NAME</NATIVEMETHOD>
                        <NATIVEMETHOD>GUID</NATIVEMETHOD>
                        <NATIVEMETHOD>COMPANYNUMBER</NATIVEMETHOD>
                        <NATIVEMETHOD>BOOKSFROM</NATIVEMETHOD>
                        <NATIVEMETHOD>PRODUCTNAME</NATIVEMETHOD>
                        <COMPUTE>EduMode : $$LicenseInfo:IsEducationalMode</COMPUTE>
                        <COMPUTE>Silver : $$LicenseInfo:IsSilver</COMPUTE>
                        <COMPUTE>Gold : $$LicenseInfo:IsGold</COMPUTE>
                        <COMPUTE>BridgeRelease : @@VersionReleaseString</COMPUTE>
                    </COLLECTION>
                </TDLMESSAGE>
            </TDL>
        </DESC>
    </BODY>
</ENVELOPE>
"#
    .trim()
    .to_string()
}

fn render_standard_ledger_identity(company: &str) -> String {
    render_standard_ledger_collection(company, "")
}

/// V1's request with `ISBILLWISEON` as one more native method.
fn render_standard_ledger_catalog_v2(company: &str) -> String {
    render_standard_ledger_collection(
        company,
        "\n                        <NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>",
    )
}

/// V2's request with `ALTERID` as one more native method.
fn render_standard_ledger_catalog_v3(company: &str) -> String {
    render_standard_ledger_collection(
        company,
        "\n                        <NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>\n                        <NATIVEMETHOD>AlterID</NATIVEMETHOD>",
    )
}

fn render_standard_ledger_collection(company: &str, extra_methods: &str) -> String {
    format!(
        r#"
<ENVELOPE>
    <HEADER>
        <VERSION>1</VERSION>
        <TALLYREQUEST>EXPORT</TALLYREQUEST>
        <TYPE>COLLECTION</TYPE>
        <ID>List of Ledgers</ID>
    </HEADER>
    <BODY>
        <DESC>
            <STATICVARIABLES>
                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
                <SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY>
            </STATICVARIABLES>
            <TDL>
                <TDLMESSAGE>
                    <COLLECTION NAME="List of Ledgers" ISMODIFY="Yes">
                        <NATIVEMETHOD>Name</NATIVEMETHOD>
                        <NATIVEMETHOD>GUID</NATIVEMETHOD>
                        <NATIVEMETHOD>Parent</NATIVEMETHOD>{}
                        <COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>
                        <COMPUTE>BRIDGECOMPANYNAME:##SVCurrentCompany</COMPUTE>
                    </COLLECTION>
                </TDLMESSAGE>
            </TDL>
        </DESC>
    </BODY>
</ENVELOPE>
"#,
        xml_escape(company),
        extra_methods
    )
    .trim()
    .to_string()
}

/// FETCHLIST of the tally-read v1 `company` part (`AuditCompanyObjectV1`).
///
/// A `Company` *collection* returns every loaded company whatever
/// `SVCURRENTCOMPANY` says (protocol reference §12a.7), and both consumers
/// take the first `COMPANY` carrying a GUID, so the part is a single-object
/// export instead (protocol reference §9.11a). That shape was first measured
/// with `FETCH *` (565 tags). This explicit list is narrower on purpose: the
/// part is stored, and nothing else in the company definition is read:
/// `GUID` and `BOOKSFROM` (both engines), `ISINTEGRATED` (the Python stock
/// test), and `NAME`, which no engine reads but which evidences which company
/// Tally resolved the `ID` to. Each was present in the `FETCH *` captures of
/// the audit books. On the one Company object export measured with this list,
/// the answer was far narrower than a `FETCH *` answer from another book, but
/// Tally also sent three children the list does not name (protocol reference
/// §9.11e, measured 2026-10-12), so admission checks the fields the consumers
/// need, not the absence of the rest.
///
/// Never add `ORIGINALNAME` or any field outside this list without a new
/// profile id.
pub const AUDIT_COMPANY_FETCH: [&str; 4] = ["GUID", "NAME", "BOOKSFROM", "ISINTEGRATED"];

/// FETCHLIST of the invoice build's company read (`InvoiceCompanyHsnV1`):
/// every field, the shape that was measured (§9.11a). The company-level
/// HSN/SAC (`HSNDETAILS.LIST`) and its settings (`CMPGSTDETAILS.LIST`) came
/// back in it and not in the company collection (protocol reference §9.16,
/// the HSN/SAC check before a post, 12 Oct 2026); whether an explicit list
/// carries them is not measured. The answer is parsed for those lists and
/// the company's GUID, then dropped: it is never stored.
pub const INVOICE_COMPANY_FETCH: [&str; 1] = ["*"];

/// FETCH of the tally-read v1 `ledgers` part (`AuditLedgersV1`).
///
/// The fields the audit consumers were recorded reading (the recorded
/// Python reads and the crate's traced reads of the same parts, 2026-09-21).
/// It is deliberately narrower
/// than the party-master workbook's list: no bank details, IFSC, e-mail,
/// phone or address, because no audit consumer reads them and the read is
/// stored (plan D-C). Adding a field needs a new profile id.
/// `LEDGSTREGDETAILS.LIST` fetches the whole registration sub-list (the
/// consumers read `APPLICABLEFROM` and `GSTIN` from it); it carries the
/// dated GST registration history; the flat `PARTYGSTIN` was blank for
/// ledgers whose GSTIN lives only there (measured 2026-09-21 on licensed
/// Silver 7.1 with this exact FETCH token in a `Ledger` collection).
pub const AUDIT_LEDGER_FETCH: &str = "NAME, GUID, MASTERID, PARENT, OPENINGBALANCE, \
ISBILLWISEON, PARTYGSTIN, INCOMETAXNUMBER, LEDGSTREGDETAILS.LIST";

/// FETCH of one tally-read v1 `vouchers` part (`AuditVouchersV1`): the
/// fields both engines were recorded reading (2026-09-21). `@REMOTEID` and
/// `@VCHTYPE` are attributes Tally emits on every voucher unasked.
///
/// - `ISOPTIONAL`, `ISCANCELLED` and `ISPOSTDATED` are load-bearing: Tally
///   emits no field the FETCH does not name, and a voucher missing any of the
///   three is UNKNOWN to the engine, which then refuses the book.
/// - Quantities, rates and amounts on goods lines are fetched at every level
///   the Python engine falls back through, including
///   `ALLLEDGERENTRIES.INVENTORYALLOCATIONS`: an absent quantity there reads
///   as no quantity and the stock movement is silently dropped.
/// - No bill allocations. That holds only while no audit test reads bill
///   types; the agent's `ALLLEDGERENTRIES.*` shape exists for readers that do.
/// - **Unmeasured:** Bridge ships one-level dotted paths
///   (`ALLLEDGERENTRIES.LEDGERNAME`); the two-level
///   `ALLLEDGERENTRIES.INVENTORYALLOCATIONS.*` and the `ALLINVENTORYENTRIES.*`
///   and `INVENTORYENTRIES{IN,OUT}.*` paths have not been sent to Tally by
///   Bridge. Live qualification must show they return the nested values on an
///   inventory book with batches before this profile reads a client book.
pub const AUDIT_VOUCHER_FETCH: &str = "GUID,MASTERID,ALTERID,DATE,VOUCHERTYPENAME,\
VOUCHERNUMBER,REFERENCE,PARTYLEDGERNAME,PARTYGSTIN,NARRATION,ISOPTIONAL,ISCANCELLED,ISPOSTDATED,\
ALLLEDGERENTRIES.LEDGERNAME,ALLLEDGERENTRIES.AMOUNT,ALLLEDGERENTRIES.ISDEEMEDPOSITIVE,\
ALLLEDGERENTRIES.INVENTORYALLOCATIONS.STOCKITEMNAME,ALLLEDGERENTRIES.INVENTORYALLOCATIONS.BILLEDQTY,\
ALLLEDGERENTRIES.INVENTORYALLOCATIONS.ACTUALQTY,ALLLEDGERENTRIES.INVENTORYALLOCATIONS.RATE,\
ALLLEDGERENTRIES.INVENTORYALLOCATIONS.AMOUNT,\
ALLINVENTORYENTRIES.STOCKITEMNAME,ALLINVENTORYENTRIES.BILLEDQTY,ALLINVENTORYENTRIES.ACTUALQTY,\
ALLINVENTORYENTRIES.RATE,ALLINVENTORYENTRIES.AMOUNT,\
INVENTORYENTRIESIN.STOCKITEMNAME,INVENTORYENTRIESIN.ACTUALQTY,INVENTORYENTRIESIN.AMOUNT,\
INVENTORYENTRIESOUT.STOCKITEMNAME,INVENTORYENTRIESOUT.ACTUALQTY,INVENTORYENTRIESOUT.AMOUNT";

/// FETCH of the tally-read v1 `stock_items` part (`AuditStockItemsV1`).
///
/// The fields the Python stock tests were recorded reading from a
/// `stock_items` part (2026-09-21). `CLOSINGRATE` is read only from
/// `stock_summary` parts and is
/// not fetched here. The request carries the audit period on the inference
/// that closing figures are period-dependent; that is unmeasured.
pub const AUDIT_STOCK_ITEM_FETCH: &str = "NAME, GUID, PARENT, BASEUNITS, \
OPENINGBALANCE, OPENINGVALUE, CLOSINGBALANCE, CLOSINGVALUE";

fn render_company_object(company: &str, fetch: &[&str]) -> String {
    let company = xml_escape(company);
    let fetch = fetch
        .iter()
        .map(|field| format!("<FETCH>{field}</FETCH>"))
        .collect::<String>();
    format!(
        r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Object</TYPE><SUBTYPE>Company</SUBTYPE><ID TYPE="Name">{company}</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES><FETCHLIST>{fetch}</FETCHLIST></DESC></BODY></ENVELOPE>"#
    )
}

fn render_audit_ledgers(company: &str, from: &str, to: &str) -> String {
    format!(
        r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Audit Ledgers</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><SVFROMDATE TYPE="Date">{from}</SVFROMDATE><SVTODATE TYPE="Date">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME="Bridge Audit Ledgers" ISMODIFY="No"><TYPE>Ledger</TYPE><FETCH>{AUDIT_LEDGER_FETCH}</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"#,
        company = xml_escape(company),
        from = xml_escape(from),
        to = xml_escape(to),
    )
}

/// The agent's windowed voucher envelope (literal `$Date` bounds, protocol
/// reference §5.3) with [`AUDIT_VOUCHER_FETCH`]. The app crate holds a test
/// that this is byte-identical to its own windowed renderer apart from the
/// FETCH, so the request is the qualified window shape. The response-side
/// checks keyed to that shape (`window_not_honoured`, part admission) are not
/// inherited by this renderer; the caller that sends it must apply them.
///
/// The dates go into a quoted `$$Date:"…"` literal, which XML escaping cannot
/// protect (Tally decodes `&quot;` before evaluating), so only a date is
/// accepted (#861).
fn render_audit_vouchers(company: &str, from: &TallyDate, to: &TallyDate) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Agent Vouchers</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{from}</SVFROMDATE><SVTODATE TYPE=\"Date\">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeAgentWindow\">$Date &gt;= $$Date:\"{from}\" AND $Date &lt;= $$Date:\"{to}\"</SYSTEM><COLLECTION NAME=\"Bridge Agent Vouchers\" ISMODIFY=\"No\"><TYPE>Voucher</TYPE><FETCH>{AUDIT_VOUCHER_FETCH}</FETCH><FILTERS>BridgeAgentWindow</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        company = xml_escape(company),
        from = from.as_str(),
        to = to.as_str(),
    )
}

fn render_audit_stock_items(company: &str, from: &str, to: &str) -> String {
    format!(
        r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Audit Stock Items</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><SVFROMDATE TYPE="Date">{from}</SVFROMDATE><SVTODATE TYPE="Date">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME="Bridge Audit Stock Items" ISMODIFY="No"><TYPE>StockItem</TYPE><FETCH>{AUDIT_STOCK_ITEM_FETCH}</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"#,
        company = xml_escape(company),
        from = xml_escape(from),
        to = xml_escape(to),
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        use fmt::Write as _;
        write!(&mut output, "{byte:02x}").expect("writing to a String cannot fail");
    }
    output
}

#[cfg(test)]
#[path = "xml_read_profiles_tests.rs"]
mod tests;
