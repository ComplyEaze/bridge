use super::*;

fn profiles(company: &ValidatedCompanyName) -> [ReadOnlyProfile<'_>; 7] {
    [
        ReadOnlyProfile::CompanyListV1,
        ReadOnlyProfile::CompanyListV2,
        ReadOnlyProfile::CompanyBookExtentV2 { company },
        ReadOnlyProfile::StandardLedgerIdentityV1 { company },
        ReadOnlyProfile::StandardLedgerCatalogV1 { company },
        ReadOnlyProfile::StandardLedgerCatalogV2 { company },
        ReadOnlyProfile::StandardLedgerCatalogV3 { company },
    ]
}

#[test]
fn validated_inputs_reject_arbitrary_or_invalid_values() {
    for company in ["", "  ", "line\nbreak", "\u{0}"] {
        assert_eq!(
            ValidatedCompanyName::new(company),
            Err(ReadProfileValidationError::CompanyInvalid)
        );
    }
    assert_eq!(
        ValidatedCompanyName::new("x".repeat(256)),
        Err(ReadProfileValidationError::CompanyInvalid)
    );
    for (from, to, expected) in [
        (
            "2026-01-01",
            "20260102",
            ReadProfileValidationError::DateInvalid,
        ),
        (
            "20260229",
            "20260301",
            ReadProfileValidationError::DateInvalid,
        ),
        (
            "20260402",
            "20260401",
            ReadProfileValidationError::DateRangeInvalid,
        ),
    ] {
        assert_eq!(ValidatedDateRange::new(from, to), Err(expected));
    }
    assert!(ValidatedDateRange::new("20240229", "20240229").is_ok());
}

#[test]
fn closed_profiles_emit_exports_only_and_escape_dynamic_values() {
    let company = ValidatedCompanyName::new("BRIDGE & <SYNTHETIC> \"BOOK\"").unwrap();
    for profile in profiles(&company) {
        let request = profile.render();
        let upper = request.to_ascii_uppercase();
        assert!(upper.contains("<TALLYREQUEST>EXPORT</TALLYREQUEST>"));
        for forbidden in ["IMPORT", "CREATE", "ALTER", "DELETE"] {
            assert!(!upper.contains(&format!("<TALLYREQUEST>{forbidden}")));
        }
    }
    let ledger = ReadOnlyProfile::StandardLedgerCatalogV2 { company: &company }.render();
    assert!(ledger.contains("BRIDGE &amp; &lt;SYNTHETIC&gt; &quot;BOOK&quot;"));
    assert!(!ledger.contains("BRIDGE & <SYNTHETIC>"));

    let injection =
        ValidatedCompanyName::new("X</SVCURRENTCOMPANY><TALLYREQUEST>IMPORT</TALLYREQUEST>")
            .unwrap();
    let escaped = ReadOnlyProfile::StandardLedgerCatalogV2 {
        company: &injection,
    }
    .render();
    assert_eq!(escaped.matches("<TALLYREQUEST>").count(), 1);
    assert!(!escaped.contains("<TALLYREQUEST>IMPORT"));
    assert!(escaped.contains("&lt;/SVCURRENTCOMPANY&gt;"));

    let bootstrap = ReadOnlyProfile::StandardLedgerIdentityV1 {
        company: &injection,
    }
    .render();
    assert_eq!(bootstrap.matches("<TALLYREQUEST>").count(), 1);
    assert!(!bootstrap.contains("<TALLYREQUEST>IMPORT"));
    assert!(bootstrap.contains("&lt;/SVCURRENTCOMPANY&gt;"));

    let extent_v2_request = ReadOnlyProfile::CompanyBookExtentV2 {
        company: &injection,
    }
    .render();
    assert_eq!(extent_v2_request.matches("<TALLYREQUEST>").count(), 1);
    assert!(!extent_v2_request.contains("<TALLYREQUEST>IMPORT"));
    assert!(extent_v2_request.contains("&lt;/SVCURRENTCOMPANY&gt;"));
}

/// `CompanyListV2` is a native Tally `Company` collection, not the
/// embedded-TDL custom report `CompanyListV1` builds: no `REPORT`/`FORM`/
/// `PART`/`LINE`/`FIELD` stack and no computed TDL function call (a
/// report stack, and no dynamic `$Field:Argument` invocation). Its three
/// fixed `$$LicenseInfo` arguments contain no spaces. It also carries no
/// `SVCURRENTCOMPANY`, so discovery is never scoped to one company -- the
/// request must be able to return every company Tally has loaded.
#[test]
fn company_list_v2_is_a_native_collection_scoped_to_no_single_company() {
    let request = ReadOnlyProfile::CompanyListV2.render();
    assert!(request.contains("<TYPE>Collection</TYPE>"));
    assert!(request.contains("<TYPE>Company</TYPE>"));
    for field in ["NAME", "GUID", "PRODUCTNAME"] {
        assert!(request.contains(&format!("<NATIVEMETHOD>{field}</NATIVEMETHOD>")));
    }
    for function in ["IsEducationalMode", "IsSilver", "IsGold"] {
        assert!(request.contains(&format!("$$LicenseInfo:{function}")));
    }
    for report_stack_tag in ["<REPORT", "<FORM ", "<PART ", "<LINE ", "<FIELD "] {
        assert!(
            !request.contains(report_stack_tag),
            "unexpected report stack tag {report_stack_tag}"
        );
    }
    assert!(!request.contains("<SVCURRENTCOMPANY"));
}

#[test]
fn profile_ids_and_template_hashes_are_stable() {
    let expected: Vec<(ReadOnlyProfileId, &str)> = vec![
        (
            ReadOnlyProfileId::CompanyListV1,
            "8905c041a75929f157be704ca9dd076a37995dc7936e0ad71c9361406a8fb694",
        ),
        (
            ReadOnlyProfileId::CompanyListV2,
            "9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8",
        ),
        (
            ReadOnlyProfileId::CompanyBookExtentV2,
            "28ffc66bfaee2172ac5de37ab358951ae2439ec2f2fc6b5aeb220624d8d87fdc",
        ),
        (
            ReadOnlyProfileId::StandardLedgerIdentityV1,
            "3f38d58a88c8bb99180290b2fb17a3057d6149f78d81e08ba3d9bfc2d595dd95",
        ),
        (
            ReadOnlyProfileId::StandardLedgerCatalogV1,
            "3f38d58a88c8bb99180290b2fb17a3057d6149f78d81e08ba3d9bfc2d595dd95",
        ),
        (
            ReadOnlyProfileId::StandardLedgerCatalogV2,
            "e93bbf843455db2a4c9f77991c52b16f10b16bbb792af5ecae4909a48216a62e",
        ),
        (
            ReadOnlyProfileId::StandardLedgerCatalogV3,
            "5bdc4b7c5c1baec328934aa5f090efd72ac821956defe63a58a4cc44909d9462",
        ),
        (
            ReadOnlyProfileId::AuditCompanyObjectV1,
            "ff023f06940b80c8e922d8b238772cd77e18f220b2fd7a4be7af667029948acf",
        ),
        (
            ReadOnlyProfileId::InvoiceCompanyHsnV1,
            "bb3efcb756ab15771e58e1daaa543af6bb73b0e32a43a723f0d1ed7d11219703",
        ),
        (
            ReadOnlyProfileId::AuditLedgersV1,
            "788333480a34bd0af0f5d724f2ac5cb8dd4d888c4a8db8e67039583fc295aa5f",
        ),
        (
            ReadOnlyProfileId::AuditVouchersV1,
            "e099da0dcb7426d833e1d2bf1eecc848ed5015847761d803cbb2a8cd6720f8a2",
        ),
        (
            ReadOnlyProfileId::AuditStockItemsV1,
            "18f8280794a476fd3b44f55c63a9f3060b7a523da82331744fe4fc0b0a151337",
        ),
    ];
    let observed = expected
        .iter()
        .map(|(profile, _)| (profile.as_str(), profile.template_sha256()))
        .collect::<Vec<_>>();
    let sealed = expected
        .iter()
        .map(|(profile, digest)| (profile.as_str(), (*digest).to_string()))
        .collect::<Vec<_>>();
    assert!(observed.iter().all(|(_, digest)| digest.len() == 64));
    assert_eq!(observed, sealed);
}

#[test]
fn compatibility_renderers_preserve_validated_profile_bytes() {
    let company = ValidatedCompanyName::new("BRIDGE SYNTHETIC BOOK").unwrap();
    assert_eq!(
        compatibility::company_list_request(),
        ReadOnlyProfile::CompanyListV1.render()
    );
    assert_eq!(
        compatibility::standard_ledger_identity_request(company.as_str()),
        ReadOnlyProfile::StandardLedgerIdentityV1 { company: &company }.render()
    );
    assert_eq!(
        compatibility::standard_ledger_catalog_request(company.as_str()),
        ReadOnlyProfile::StandardLedgerCatalogV1 { company: &company }.render()
    );
}

/// Every element of `xml` as its slash-joined path and trimmed text, in
/// document order. Attributes are recorded as `path/@NAME`.
fn element_paths(xml: &str) -> Vec<(String, String)> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(xml);
    let mut path = Vec::<String>::new();
    let mut out = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(event) => {
                path.push(event.name().as_ref().to_owned());
                for attribute in event.attributes() {
                    let attribute = attribute.unwrap();
                    out.push((
                        format!("{}/@{}", path.join("/"), attribute.key.as_ref()),
                        attribute
                            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                            .unwrap()
                            .into_owned(),
                    ));
                }
                out.push((path.join("/"), String::new()));
            }
            Event::Text(text) => {
                if let Some(last) = out.last_mut() {
                    last.1.push_str(&text);
                }
            }
            Event::GeneralRef(reference) => {
                let decoded = quick_xml::escape::resolve_predefined_entity(&reference).unwrap();
                out.last_mut().unwrap().1.push_str(decoded);
            }
            Event::End(_) => {
                path.pop();
            }
            Event::Eof => break,
            other => panic!("unexpected event in a rendered profile: {other:?}"),
        }
    }
    out.into_iter()
        .map(|(path, text)| (path, text.trim().to_string()))
        .collect()
}

/// The invoice build's company read is, character for character, the
/// request the lab answered on 12 Oct 2026 (the file starts with a byte order
/// mark), and differs from the audit part's only in its field list.
#[test]
fn invoice_company_hsn_is_the_company_object_request_the_lab_answered() {
    let bytes = include_bytes!(
        "../tests/fixtures/agent/pilot-lab/pilot-lab-company-object.request.utf16le.xml"
    );
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    let sent = String::from_utf16(&units).unwrap();
    let company = ValidatedCompanyName::new("BRIDGE PILOT LAB").unwrap();
    let invoice = ReadOnlyProfile::InvoiceCompanyHsnV1 { company: &company }.render();
    assert_eq!(sent.trim_start_matches('\u{feff}'), invoice);
    let audit = ReadOnlyProfile::AuditCompanyObjectV1 { company: &company }.render();
    let fields = AUDIT_COMPANY_FETCH
        .iter()
        .map(|field| format!("<FETCH>{field}</FETCH>"))
        .collect::<String>();
    assert_eq!(audit.replace(&fields, "<FETCH>*</FETCH>"), invoice);
}

#[test]
fn audit_company_object_is_exactly_one_named_object_with_the_closed_fetchlist() {
    let company = ValidatedCompanyName::new("BRIDGE & <SYNTHETIC> \"BOOK\"").unwrap();
    let request = ReadOnlyProfile::AuditCompanyObjectV1 { company: &company }.render();
    let mut expected = vec![
        ("ENVELOPE", ""),
        ("ENVELOPE/HEADER", ""),
        ("ENVELOPE/HEADER/VERSION", "1"),
        ("ENVELOPE/HEADER/TALLYREQUEST", "Export"),
        ("ENVELOPE/HEADER/TYPE", "Object"),
        ("ENVELOPE/HEADER/SUBTYPE", "Company"),
        ("ENVELOPE/HEADER/ID/@TYPE", "Name"),
        ("ENVELOPE/HEADER/ID", "BRIDGE & <SYNTHETIC> \"BOOK\""),
        ("ENVELOPE/BODY", ""),
        ("ENVELOPE/BODY/DESC", ""),
        ("ENVELOPE/BODY/DESC/STATICVARIABLES", ""),
        (
            "ENVELOPE/BODY/DESC/STATICVARIABLES/SVEXPORTFORMAT",
            "$$SysName:XML",
        ),
        (
            "ENVELOPE/BODY/DESC/STATICVARIABLES/SVCURRENTCOMPANY",
            "BRIDGE & <SYNTHETIC> \"BOOK\"",
        ),
        ("ENVELOPE/BODY/DESC/FETCHLIST", ""),
    ];
    for field in AUDIT_COMPANY_FETCH {
        expected.push(("ENVELOPE/BODY/DESC/FETCHLIST/FETCH", field));
    }
    let observed = element_paths(&request);
    assert_eq!(
        observed,
        expected
            .into_iter()
            .map(|(path, text)| (path.to_string(), text.to_string()))
            .collect::<Vec<_>>(),
        "the company part is a closed shape: no TDL, filter, compute or extra field"
    );
    assert_eq!(
        AUDIT_COMPANY_FETCH,
        ["GUID", "NAME", "BOOKSFROM", "ISINTEGRATED"]
    );
}

#[test]
fn audit_collections_are_exports_of_one_type_with_their_fetch_and_period() {
    let company = ValidatedCompanyName::new("BRIDGE & <SYNTHETIC> \"BOOK\"").unwrap();
    let period = ValidatedDateRange::new("20250401", "20260331").unwrap();
    let day = ValidatedDateRange::new("20260330", "20260330").unwrap();
    for (request, collection, object_type, fetch, from, to) in [
        (
            ReadOnlyProfile::AuditLedgersV1 {
                company: &company,
                period: &period,
            }
            .render(),
            "Bridge Audit Ledgers",
            "Ledger",
            AUDIT_LEDGER_FETCH,
            "20250401",
            "20260331",
        ),
        (
            ReadOnlyProfile::AuditStockItemsV1 {
                company: &company,
                period: &period,
            }
            .render(),
            "Bridge Audit Stock Items",
            "StockItem",
            AUDIT_STOCK_ITEM_FETCH,
            "20250401",
            "20260331",
        ),
        (
            ReadOnlyProfile::AuditVouchersV1 {
                company: &company,
                window: &day,
            }
            .render(),
            "Bridge Agent Vouchers",
            "Voucher",
            AUDIT_VOUCHER_FETCH,
            "20260330",
            "20260330",
        ),
    ] {
        let observed = element_paths(&request);
        let text = |path: &str| {
            let values = observed
                .iter()
                .filter(|(candidate, _)| candidate == path)
                .map(|(_, value)| value.as_str())
                .collect::<Vec<_>>();
            assert_eq!(values.len(), 1, "{path} occurs once in {collection}");
            values[0].to_string()
        };
        assert_eq!(text("ENVELOPE/HEADER/TALLYREQUEST"), "Export");
        assert_eq!(text("ENVELOPE/HEADER/TYPE"), "Collection");
        assert_eq!(text("ENVELOPE/HEADER/ID"), collection);
        assert_eq!(
            text("ENVELOPE/BODY/DESC/STATICVARIABLES/SVCURRENTCOMPANY"),
            "BRIDGE & <SYNTHETIC> \"BOOK\""
        );
        assert_eq!(text("ENVELOPE/BODY/DESC/STATICVARIABLES/SVFROMDATE"), from);
        assert_eq!(text("ENVELOPE/BODY/DESC/STATICVARIABLES/SVTODATE"), to);
        let collection_path = "ENVELOPE/BODY/DESC/TDL/TDLMESSAGE/COLLECTION";
        assert_eq!(text(&format!("{collection_path}/@NAME")), collection);
        assert_eq!(text(&format!("{collection_path}/@ISMODIFY")), "No");
        assert_eq!(text(&format!("{collection_path}/TYPE")), object_type);
        assert_eq!(text(&format!("{collection_path}/FETCH")), fetch);
        assert!(
            !observed.iter().any(|(path, _)| path.ends_with("/COMPUTE")),
            "{collection} computes nothing"
        );
    }
}

#[test]
fn audit_voucher_window_filters_on_literal_dates_only() {
    let company = ValidatedCompanyName::new("BRIDGE SYNTHETIC BOOK").unwrap();
    let day = ValidatedDateRange::new("20260330", "20260331").unwrap();
    let request = ReadOnlyProfile::AuditVouchersV1 {
        company: &company,
        window: &day,
    }
    .render();
    let observed = element_paths(&request);
    let formulae = observed
        .iter()
        .filter(|(path, _)| path == "ENVELOPE/BODY/DESC/TDL/TDLMESSAGE/SYSTEM")
        .map(|(_, value)| value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        formulae,
        ["$Date >= $$Date:\"20260330\" AND $Date <= $$Date:\"20260331\""]
    );
    assert!(observed.contains(&(
        "ENVELOPE/BODY/DESC/TDL/TDLMESSAGE/COLLECTION/FILTERS".to_string(),
        "BridgeAgentWindow".to_string()
    )));
}

#[test]
fn audit_fetch_lists_carry_what_a_silent_default_would_hide() {
    let fields = |fetch: &str| {
        fetch
            .split(',')
            .map(|field| field.trim().to_string())
            .collect::<Vec<_>>()
    };
    let vouchers = fields(AUDIT_VOUCHER_FETCH);
    // A voucher missing any status flag is UNKNOWN and the engine refuses
    // the book; Tally emits no field the FETCH does not name.
    for required in [
        "GUID",
        "ALTERID",
        "DATE",
        "ISOPTIONAL",
        "ISCANCELLED",
        "ISPOSTDATED",
    ] {
        assert!(vouchers.iter().any(|field| field == required), "{required}");
    }
    // An absent quantity, rate or amount on a goods line is read as none,
    // and the stock movement is silently dropped.
    for prefix in [
        "ALLLEDGERENTRIES.INVENTORYALLOCATIONS",
        "ALLINVENTORYENTRIES",
    ] {
        for leaf in ["STOCKITEMNAME", "BILLEDQTY", "ACTUALQTY", "RATE", "AMOUNT"] {
            let field = format!("{prefix}.{leaf}");
            assert!(vouchers.contains(&field), "{field}");
        }
    }
    for prefix in ["INVENTORYENTRIESIN", "INVENTORYENTRIESOUT"] {
        for leaf in ["STOCKITEMNAME", "ACTUALQTY", "AMOUNT"] {
            let field = format!("{prefix}.{leaf}");
            assert!(vouchers.contains(&field), "{field}");
        }
    }
    assert!(
        !vouchers
            .iter()
            .any(|field| field.contains("BILLALLOCATIONS")),
        "no audit consumer reads bill allocations"
    );
    for fetch in [
        AUDIT_VOUCHER_FETCH,
        AUDIT_LEDGER_FETCH,
        AUDIT_STOCK_ITEM_FETCH,
    ] {
        let listed = fields(fetch);
        let mut unique = listed.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), listed.len(), "no field is listed twice");
        assert!(
            !listed.iter().any(|field| field.contains('*')),
            "no wildcard"
        );
    }
}

#[test]
fn audit_ledgers_fetch_no_contact_or_bank_details() {
    let upper = AUDIT_LEDGER_FETCH.to_ascii_uppercase();
    for sensitive in [
        "BANK",
        "IFSC",
        "EMAIL",
        "PHONE",
        "MOBILE",
        "ADDRESS",
        "PINCODE",
        "MSME",
        "UDYAM",
        "NAMEONPAN",
    ] {
        assert!(!upper.contains(sensitive), "{sensitive}");
    }
    assert!(AUDIT_LEDGER_FETCH.contains("LEDGSTREGDETAILS.LIST"));
    assert!(AUDIT_LEDGER_FETCH.contains("ISBILLWISEON"));
}

#[test]
fn audit_fetch_lists_are_pinned_exactly() {
    // A field dropped from any list silently changes a figure in at least one
    // engine (a line without AMOUNT is skipped; an absent OPENINGBALANCE reads
    // as 0), and a template-hash reseal would not say which. Each list is
    // therefore pinned field by field, and a change must edit this test.
    let fields = |fetch: &str| {
        fetch
            .split(',')
            .map(|field| field.trim().to_string())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        fields(AUDIT_LEDGER_FETCH),
        [
            "NAME",
            "GUID",
            "MASTERID",
            "PARENT",
            "OPENINGBALANCE",
            "ISBILLWISEON",
            "PARTYGSTIN",
            "INCOMETAXNUMBER",
            "LEDGSTREGDETAILS.LIST",
        ]
    );
    assert_eq!(
        fields(AUDIT_STOCK_ITEM_FETCH),
        [
            "NAME",
            "GUID",
            "PARENT",
            "BASEUNITS",
            "OPENINGBALANCE",
            "OPENINGVALUE",
            "CLOSINGBALANCE",
            "CLOSINGVALUE",
        ]
    );
    assert_eq!(
        fields(AUDIT_VOUCHER_FETCH),
        [
            "GUID",
            "MASTERID",
            "ALTERID",
            "DATE",
            "VOUCHERTYPENAME",
            "VOUCHERNUMBER",
            "REFERENCE",
            "PARTYLEDGERNAME",
            "PARTYGSTIN",
            "NARRATION",
            "ISOPTIONAL",
            "ISCANCELLED",
            "ISPOSTDATED",
            "ALLLEDGERENTRIES.LEDGERNAME",
            "ALLLEDGERENTRIES.AMOUNT",
            "ALLLEDGERENTRIES.ISDEEMEDPOSITIVE",
            "ALLLEDGERENTRIES.INVENTORYALLOCATIONS.STOCKITEMNAME",
            "ALLLEDGERENTRIES.INVENTORYALLOCATIONS.BILLEDQTY",
            "ALLLEDGERENTRIES.INVENTORYALLOCATIONS.ACTUALQTY",
            "ALLLEDGERENTRIES.INVENTORYALLOCATIONS.RATE",
            "ALLLEDGERENTRIES.INVENTORYALLOCATIONS.AMOUNT",
            "ALLINVENTORYENTRIES.STOCKITEMNAME",
            "ALLINVENTORYENTRIES.BILLEDQTY",
            "ALLINVENTORYENTRIES.ACTUALQTY",
            "ALLINVENTORYENTRIES.RATE",
            "ALLINVENTORYENTRIES.AMOUNT",
            "INVENTORYENTRIESIN.STOCKITEMNAME",
            "INVENTORYENTRIESIN.ACTUALQTY",
            "INVENTORYENTRIESIN.AMOUNT",
            "INVENTORYENTRIESOUT.STOCKITEMNAME",
            "INVENTORYENTRIESOUT.ACTUALQTY",
            "INVENTORYENTRIESOUT.AMOUNT",
        ]
    );
}

fn every_profile_id() -> Vec<ReadOnlyProfileId> {
    // The match makes a new profile fail to compile here until it is listed.
    fn listed(id: ReadOnlyProfileId) {
        match id {
            ReadOnlyProfileId::CompanyListV1
            | ReadOnlyProfileId::CompanyListV2
            | ReadOnlyProfileId::CompanyBookExtentV2
            | ReadOnlyProfileId::StandardLedgerIdentityV1
            | ReadOnlyProfileId::StandardLedgerCatalogV1
            | ReadOnlyProfileId::StandardLedgerCatalogV2
            | ReadOnlyProfileId::StandardLedgerCatalogV3
            | ReadOnlyProfileId::AuditCompanyObjectV1
            | ReadOnlyProfileId::InvoiceCompanyHsnV1
            | ReadOnlyProfileId::AuditLedgersV1
            | ReadOnlyProfileId::AuditVouchersV1
            | ReadOnlyProfileId::AuditStockItemsV1 => {}
        }
    }
    let ids = vec![
        ReadOnlyProfileId::CompanyListV1,
        ReadOnlyProfileId::CompanyListV2,
        ReadOnlyProfileId::CompanyBookExtentV2,
        ReadOnlyProfileId::StandardLedgerIdentityV1,
        ReadOnlyProfileId::StandardLedgerCatalogV1,
        ReadOnlyProfileId::StandardLedgerCatalogV2,
        ReadOnlyProfileId::StandardLedgerCatalogV3,
        ReadOnlyProfileId::AuditCompanyObjectV1,
        ReadOnlyProfileId::InvoiceCompanyHsnV1,
        ReadOnlyProfileId::AuditLedgersV1,
        ReadOnlyProfileId::AuditVouchersV1,
        ReadOnlyProfileId::AuditStockItemsV1,
    ];
    ids.iter().copied().for_each(listed);
    ids
}

#[test]
fn education_refuses_exactly_the_profiles_that_render_a_spaced_function_argument() {
    let mut refused = Vec::new();
    for id in every_profile_id() {
        let hazard = first_spaced_function_argument(&id.template());
        assert_eq!(
            id.education_refuses_report_formula(),
            hazard.is_some(),
            "{}: {hazard:?}",
            id.as_str()
        );
        refused.extend(hazard.map(|h| (id.as_str(), h)));
    }
    assert!(refused.is_empty(), "{refused:?}");
}

/// `$$name:argument`, assembled at run time: a literal would itself be a hit
/// for `scripts/check-tally-request-builder-hazards.mjs`.
fn spaced(name: &str, argument: &str) -> String {
    format!("{}{name}:{argument}", "$".repeat(2))
}

#[test]
fn a_spaced_function_argument_is_read_as_the_hazard_script_reads_it() {
    let numitems = spaced("NumItems", "BRIDGE Ledger Collection V1");
    let quoted = spaced("Fn", "\"quoted with space\"");
    let tabbed = spaced("Fn", "tab\there");
    let second = format!("$$9x:a{}{}<", "\u{20}", spaced("_ok", "a b"));
    for (tdl, hit) in [
        (format!("<SET>{numitems}</SET>"), Some(numitems.clone())),
        ("<SET>$$NumItems:AllLedgerEntries</SET>".to_string(), None),
        (
            "$Date >= $$Date:\"20260401\" AND $Date <= $$Date:\"20260430\"".to_string(),
            None,
        ),
        ("$$String:##SVFromDate:\"YYYYMMDD\"".to_string(), None),
        ("$GUID:Company:##SVCurrentCompany".to_string(), None),
        (quoted, Some(spaced("Fn", "quoted with space"))),
        (tabbed.clone(), Some(tabbed)),
        (second, Some(spaced("_ok", "a b"))),
        ("$$".to_string(), None),
        // An unclosed quote is scanned as unquoted, up to the next `<`.
        (
            format!("{}<x y>", spaced("Fn", "\"open quote")),
            Some(spaced("Fn", "\"open quote")),
        ),
        (format!("{}<x y>", "$$Fn:\"unclosed"), None),
    ] {
        assert_eq!(first_spaced_function_argument(&tdl), hit, "{tdl}");
    }
}

/// V2 is V1 and one more native method, so the same ledger rows come back with
/// one more field, and V1's own bytes are not touched.
#[test]
fn the_v2_catalogue_request_is_v1_plus_the_bill_wise_method() {
    let company = ValidatedCompanyName::new("BRIDGE SYNTHETIC BOOK").unwrap();
    let v1 = ReadOnlyProfile::StandardLedgerCatalogV1 { company: &company }.render();
    let v2 = ReadOnlyProfile::StandardLedgerCatalogV2 { company: &company }.render();
    let method = "<NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>";
    assert_eq!(v2.matches(method).count(), 1);
    assert!(!v1.contains(method));
    assert_eq!(
        v2.lines()
            .filter(|line| !line.contains(method))
            .collect::<Vec<_>>(),
        v1.lines().collect::<Vec<_>>()
    );
    // The method sits with the other native methods, not elsewhere in the request.
    let lines = v2.lines().collect::<Vec<_>>();
    let parent = lines
        .iter()
        .position(|line| line.trim() == "<NATIVEMETHOD>Parent</NATIVEMETHOD>")
        .unwrap();
    assert_eq!(lines[parent + 1].trim(), method);
    assert_eq!(
        ReadOnlyProfileId::StandardLedgerCatalogV2.as_str(),
        "standard_ledger_catalog_v2"
    );
    assert_ne!(
        ReadOnlyProfileId::StandardLedgerCatalogV1.template_sha256(),
        ReadOnlyProfileId::StandardLedgerCatalogV2.template_sha256()
    );
}

/// V3 is V2 and one more native method, `AlterID`, placed straight after
/// `IsBillWiseOn` and formatted like it; V2's own bytes are not touched.
#[test]
fn the_v3_catalogue_request_is_v2_plus_the_alter_id_method() {
    let company = ValidatedCompanyName::new("BRIDGE SYNTHETIC BOOK").unwrap();
    let v2 = ReadOnlyProfile::StandardLedgerCatalogV2 { company: &company }.render();
    let v3 = ReadOnlyProfile::StandardLedgerCatalogV3 { company: &company }.render();
    let method = "<NATIVEMETHOD>AlterID</NATIVEMETHOD>";
    assert_eq!(v3.matches(method).count(), 1);
    assert!(!v2.contains(method));
    assert_eq!(
        v3.lines()
            .filter(|line| !line.contains(method))
            .collect::<Vec<_>>(),
        v2.lines().collect::<Vec<_>>()
    );
    let lines = v3.lines().collect::<Vec<_>>();
    let bill_wise = lines
        .iter()
        .position(|line| line.trim() == "<NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>")
        .unwrap();
    assert_eq!(
        lines[bill_wise + 1],
        format!("                        {method}")
    );
    assert_eq!(
        ReadOnlyProfileId::StandardLedgerCatalogV3.as_str(),
        "standard_ledger_catalog_v3"
    );
    assert_ne!(
        ReadOnlyProfileId::StandardLedgerCatalogV2.template_sha256(),
        ReadOnlyProfileId::StandardLedgerCatalogV3.template_sha256()
    );
}

/// The exact bytes V3 sends for a sample company.
#[test]
fn the_v3_catalogue_request_renders_these_exact_bytes() {
    let company = ValidatedCompanyName::new("BRIDGE SYNTHETIC BOOK").unwrap();
    let expected = r#"<ENVELOPE>
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
                <SVCURRENTCOMPANY>BRIDGE SYNTHETIC BOOK</SVCURRENTCOMPANY>
            </STATICVARIABLES>
            <TDL>
                <TDLMESSAGE>
                    <COLLECTION NAME="List of Ledgers" ISMODIFY="Yes">
                        <NATIVEMETHOD>Name</NATIVEMETHOD>
                        <NATIVEMETHOD>GUID</NATIVEMETHOD>
                        <NATIVEMETHOD>Parent</NATIVEMETHOD>
                        <NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>
                        <NATIVEMETHOD>AlterID</NATIVEMETHOD>
                        <COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>
                        <COMPUTE>BRIDGECOMPANYNAME:##SVCurrentCompany</COMPUTE>
                    </COLLECTION>
                </TDLMESSAGE>
            </TDL>
        </DESC>
    </BODY>
</ENVELOPE>"#;
    assert_eq!(
        ReadOnlyProfile::StandardLedgerCatalogV3 { company: &company }.render(),
        expected
    );
}
