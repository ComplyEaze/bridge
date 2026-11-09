use super::*;

/// The synthetic company's GUID in the lab's answers.
const LAB_GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

const GROUPS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-groups.utf16le.xml"
);

const RATES_HSN: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-hsn.utf16le.xml"
);

const GROUPS_HSN: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-groups-hsn.utf16le.xml"
);

const COMPANY_OBJECT: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-company-object.utf16le.xml"
);

fn utf16(bytes: &[u8]) -> String {
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    String::from_utf16(&units)
        .unwrap()
        .trim_start_matches('\u{feff}')
        .to_string()
}

/// A dated row as the lab's ledgers and groups carried it.
fn row(from: &str, source: &str, code: &str) -> HsnRow {
    HsnRow {
        applicable_from: Some(from.to_string()),
        code: Some(code.to_string()),
        source: Some(source.to_string()),
        classification: Some(String::new()),
        unmeasured: false,
        empty: false,
    }
}

fn own(from: &str, code: &str) -> HsnRow {
    row(from, SPECIFIED, code)
}

fn defer(from: &str) -> HsnRow {
    row(from, INHERITED, "")
}

fn placeholder() -> HsnRow {
    HsnRow {
        applicable_from: None,
        code: None,
        source: None,
        classification: None,
        unmeasured: false,
        empty: true,
    }
}

fn ledger(rows: Vec<HsnRow>) -> Result<HsnRows, &'static str> {
    Ok(Some(rows))
}

/// The lab's chart, nearest first: `BRIDGE Sales Sub` under `Sales Accounts`.
fn sub_chain() -> Vec<String> {
    vec!["BRIDGE Sales Sub".to_string(), "Sales Accounts".to_string()]
}

fn top_chain() -> Vec<String> {
    vec!["Sales Accounts".to_string()]
}

fn chart(sub: Vec<HsnRow>, sales: Vec<HsnRow>) -> BTreeMap<String, HsnRows> {
    BTreeMap::from([
        ("BRIDGE Sales Sub".to_string(), Some(sub)),
        ("Sales Accounts".to_string(), Some(sales)),
    ])
}

/// The company as the lab's read it: priority Ledger, all sections, a
/// minimum length of 4, no GST classification.
fn company(rows: Vec<HsnRow>) -> CompanyHsn {
    CompanyHsn {
        rows: Some(rows),
        settings: vec![CompanyGstRow {
            applicable_from: Some("20260401".to_string()),
            priority: Some("Ledger".to_string()),
            sections: Some("All Sections".to_string()),
            code_length: Some("4".to_string()),
        }],
        classification_on: Some("No".to_string()),
    }
}

fn no_code_company() -> CompanyHsn {
    company(vec![placeholder()])
}

fn resolved(code: &str, level: HsnLevel) -> HsnStep {
    HsnStep::Resolved {
        code: code.to_string(),
        level,
    }
}

const DATE: &str = "20260807";

/// The captured company object answer, read at the edge: bound to the
/// company by GUID, with its one HSN/SAC row and its settings.
#[test]
fn the_company_object_answer_carries_the_company_code_and_its_settings() {
    let firm = parse_company_hsn(&utf16(COMPANY_OBJECT), LAB_GUID).unwrap();
    assert_eq!(firm.classification_on.as_deref(), Some("No"));
    assert_eq!(
        firm.settings,
        vec![CompanyGstRow {
            applicable_from: Some("20260401".to_string()),
            priority: Some("Ledger".to_string()),
            sections: Some("All Sections".to_string()),
            code_length: Some("4".to_string()),
        }]
    );
    let rows = firm.rows.as_ref().unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].applicable_from.as_deref(), Some("20260401"));
    assert_eq!(rows[0].source.as_deref(), Some(SPECIFIED));
    assert_eq!(rows[0].code.as_deref(), Some("998341"));
    assert!(!rows[0].unmeasured && !rows[0].empty);
    // A ledger that defers through a group with no code takes it.
    let step = resolve_hsn(
        &ledger(vec![defer("20260401")]),
        Some(&top_chain()),
        Some(&chart(vec![], vec![placeholder()])),
        &firm,
        DATE,
    );
    assert_eq!(step, resolved("998341", HsnLevel::Company));
    // Another company's answer is never read as this one's.
    assert_eq!(
        parse_company_hsn(
            &utf16(COMPANY_OBJECT),
            "00000000-0000-0000-0000-000000000000"
        ),
        Err("invoice_company_object_mismatch")
    );
}

/// The order Tally follows: the ledger's own row, the nearest group, the
/// company; the groups are asked for only when the ledger defers.
#[test]
fn the_ledger_wins_then_the_nearest_group_then_the_company() {
    let firm = company(vec![own("20260401", "998341")]);
    let chain = sub_chain();
    // The ledger's own code needs no group read.
    assert_eq!(
        resolve_hsn(
            &ledger(vec![own("20260401", "998344")]),
            Some(&chain),
            None,
            &firm,
            DATE
        ),
        resolved("998344", HsnLevel::Ledger)
    );
    // A deferring ledger asks for the groups.
    let deferring = ledger(vec![defer("20260401")]);
    assert_eq!(
        resolve_hsn(&deferring, Some(&chain), None, &firm, DATE),
        HsnStep::NeedsGroups
    );
    // The sub-group before its parent.
    let groups = chart(
        vec![own("20260401", "998343")],
        vec![own("20260401", "998342")],
    );
    assert_eq!(
        resolve_hsn(&deferring, Some(&chain), Some(&groups), &firm, DATE),
        resolved("998343", HsnLevel::Group("BRIDGE Sales Sub".to_string()))
    );
    // A sub-group that defers passes to its parent.
    let groups = chart(vec![defer("20260401")], vec![own("20260401", "998342")]);
    assert_eq!(
        resolve_hsn(&deferring, Some(&chain), Some(&groups), &firm, DATE),
        resolved("998342", HsnLevel::Group("Sales Accounts".to_string()))
    );
    // Nothing in the chain: the company.
    let groups = chart(vec![defer("20260401")], vec![placeholder()]);
    assert_eq!(
        resolve_hsn(&deferring, Some(&chain), Some(&groups), &firm, DATE),
        resolved("998341", HsnLevel::Company)
    );
    // And nothing at the company either: the refusal.
    assert_eq!(
        resolve_hsn(
            &deferring,
            Some(&chain),
            Some(&groups),
            &no_code_company(),
            DATE
        ),
        HsnStep::Refused(UNRESOLVED)
    );
}

/// The row in force is the latest one not after the invoice date; a row
/// dated later is ignored, and a date with no row in force is not measured.
#[test]
fn the_row_in_force_is_the_latest_not_after_the_date() {
    let firm = no_code_company();
    let step = |rows: Vec<HsnRow>, date: &str| {
        resolve_hsn(&ledger(rows), Some(&top_chain()), None, &firm, date)
    };
    let rows = vec![own("20260401", "998344"), own("20260901", "998345")];
    assert_eq!(
        step(rows.clone(), DATE),
        resolved("998344", HsnLevel::Ledger)
    );
    assert_eq!(step(rows, "20260901"), resolved("998345", HsnLevel::Ledger));
    // A code from 1 Sep does not cover a 7 Aug invoice. A deferring row in
    // force beside other rows was not measured (deferring was seen only on a
    // master with one row).
    assert_eq!(
        step(vec![defer("20260401"), own("20260901", "998345")], DATE),
        HsnStep::Refused(NOT_MEASURED)
    );
    // A row with no date beside a dated one.
    let mut undated = own("20260401", "998344");
    undated.applicable_from = None;
    assert_eq!(
        step(vec![own("20260401", "998344"), undated], DATE),
        HsnStep::Refused(NOT_MEASURED)
    );
    assert_eq!(
        step(vec![own("20260901", "998345")], DATE),
        HsnStep::Refused(NOT_MEASURED)
    );
    // Eight digits that name no calendar day are no date (#2009).
    assert_eq!(
        step(vec![own("20251340", "998344")], DATE),
        HsnStep::Refused(NOT_MEASURED)
    );
}

/// Every row shape the lab did not answer refuses under its own code.
#[test]
fn a_row_shape_not_measured_is_refused_as_such() {
    let firm = no_code_company();
    let refuse =
        |rows: Vec<HsnRow>| resolve_hsn(&ledger(rows), Some(&top_chain()), None, &firm, DATE);
    let not_measured = HsnStep::Refused(NOT_MEASURED);
    // Two rows on one date.
    assert_eq!(
        refuse(vec![own("20260401", "998344"), own("20260401", "998345")]),
        not_measured
    );
    // An element the lab's rows did not carry.
    let mut extra = own("20260401", "998344");
    extra.unmeasured = true;
    assert_eq!(refuse(vec![extra]), not_measured);
    // A classification.
    let mut classified = own("20260401", "998344");
    classified.classification = Some("Services".to_string());
    assert_eq!(refuse(vec![classified]), not_measured);
    // Its own code chosen but empty, a deferring row with a code, another source.
    assert_eq!(refuse(vec![own("20260401", "")]), not_measured);
    assert_eq!(
        refuse(vec![row("20260401", INHERITED, "998344")]),
        not_measured
    );
    assert_eq!(
        refuse(vec![row("20260401", "Use GST Classification", "")]),
        not_measured
    );
    // A date that is not eight digits, or none.
    assert_eq!(refuse(vec![own("2026-04-01", "998344")]), not_measured);
    let mut undated = own("20260401", "998344");
    undated.applicable_from = None;
    assert_eq!(refuse(vec![undated]), not_measured);
    // A placeholder on a ledger (measured on groups only), or beside a row.
    assert_eq!(refuse(vec![placeholder()]), not_measured);
    assert_eq!(
        refuse(vec![placeholder(), own("20260401", "998344")]),
        not_measured
    );
    // No HSN/SAC element at all, or rows that could not be read.
    assert_eq!(
        resolve_hsn(&Ok(None), Some(&top_chain()), None, &firm, DATE),
        not_measured
    );
    assert_eq!(
        resolve_hsn(
            &Err("invoice_read_field_repeated"),
            Some(&top_chain()),
            None,
            &firm,
            DATE
        ),
        not_measured
    );
}

/// A code shorter than the company's minimum length is not one Tally was
/// seen to accept, at any level; the measured four digits are.
#[test]
fn a_code_shorter_than_the_company_minimum_is_refused() {
    let mut firm = company(vec![own("20260401", "99")]);
    let chain = top_chain();
    let step = |rows: Vec<HsnRow>, firm: &CompanyHsn| {
        resolve_hsn(
            &ledger(rows),
            Some(&chain),
            Some(&chart(vec![], vec![placeholder()])),
            firm,
            DATE,
        )
    };
    assert_eq!(
        step(vec![own("20260401", "9983")], &firm),
        resolved("9983", HsnLevel::Ledger)
    );
    assert_eq!(
        step(vec![own("20260401", "998")], &firm),
        HsnStep::Refused(NOT_MEASURED)
    );
    assert_eq!(
        step(vec![own("20260401", "NA")], &firm),
        HsnStep::Refused(NOT_MEASURED)
    );
    assert_eq!(
        step(vec![defer("20260401")], &firm),
        HsnStep::Refused(NOT_MEASURED)
    );
    // At a group too.
    let short_group = chart(vec![], vec![own("20260401", "998")]);
    assert_eq!(
        resolve_hsn(
            &ledger(vec![defer("20260401")]),
            Some(&chain),
            Some(&short_group),
            &firm,
            DATE
        ),
        HsnStep::Refused(NOT_MEASURED)
    );
    firm.settings[0].code_length = Some("6".to_string());
    assert_eq!(
        step(vec![own("20260401", "9983")], &firm),
        HsnStep::Refused(NOT_MEASURED)
    );
    assert_eq!(
        step(vec![own("20260401", "998344")], &firm),
        resolved("998344", HsnLevel::Ledger)
    );
}

/// The chain followed is the measured one: up to two groups, every group
/// present in the listing, and a chain the compliance listing resolved.
#[test]
fn the_group_chain_is_followed_only_as_deep_as_measured() {
    let firm = company(vec![own("20260401", "998341")]);
    let deferring = ledger(vec![defer("20260401")]);
    let not_measured = HsnStep::Refused(NOT_MEASURED);
    let mut groups = chart(vec![defer("20260401")], vec![own("20260401", "998342")]);
    groups.insert("Deep".to_string(), Some(vec![defer("20260401")]));
    // Two deferring groups below `Sales Accounts`: its code is three groups
    // up, and a chain that deep was never measured.
    let deep = vec![
        "Deep".to_string(),
        "BRIDGE Sales Sub".to_string(),
        "Sales Accounts".to_string(),
    ];
    assert_eq!(
        resolve_hsn(&deferring, Some(&deep), Some(&groups), &firm, DATE),
        not_measured
    );
    // A group the listing does not hold, and a chain that did not resolve.
    let missing = vec!["Missing".to_string(), "Sales Accounts".to_string()];
    assert_eq!(
        resolve_hsn(&deferring, Some(&missing), Some(&groups), &firm, DATE),
        not_measured
    );
    assert_eq!(
        resolve_hsn(&deferring, None, Some(&groups), &firm, DATE),
        not_measured
    );
    // A group with no HSN/SAC element, or two placeholders.
    groups.insert("Sales Accounts".to_string(), None);
    assert_eq!(
        resolve_hsn(&deferring, Some(&top_chain()), Some(&groups), &firm, DATE),
        not_measured
    );
    groups.insert(
        "Sales Accounts".to_string(),
        Some(vec![placeholder(), placeholder()]),
    );
    assert_eq!(
        resolve_hsn(&deferring, Some(&top_chain()), Some(&groups), &firm, DATE),
        not_measured
    );
}

/// The company's settings decide whether the measured order holds at all,
/// so they gate the ledger's own code as well as the company's.
#[test]
fn the_company_settings_gate_every_level() {
    let base = company(vec![own("20260401", "998341")]);
    let not_measured = HsnStep::Refused(NOT_MEASURED);
    let ledger_code = |firm: &CompanyHsn| {
        resolve_hsn(
            &ledger(vec![own("20260401", "998344")]),
            Some(&top_chain()),
            None,
            firm,
            DATE,
        )
    };
    let company_code = |firm: &CompanyHsn| {
        resolve_hsn(
            &ledger(vec![defer("20260401")]),
            Some(&top_chain()),
            Some(&chart(vec![], vec![placeholder()])),
            firm,
            DATE,
        )
    };
    assert_eq!(ledger_code(&base), resolved("998344", HsnLevel::Ledger));
    assert_eq!(company_code(&base), resolved("998341", HsnLevel::Company));
    let mut variants = Vec::new();
    // GST classification on, or not reported.
    for value in [Some("Yes"), None] {
        let mut firm = base.clone();
        firm.classification_on = value.map(str::to_string);
        variants.push(firm);
    }
    // A priority or a section setting that is not the measured one, or absent;
    // a length that is no number.
    for (priority, sections, length) in [
        (Some("Group"), Some("All Sections"), Some("4")),
        (None, Some("All Sections"), Some("4")),
        (Some("Ledger"), Some("B2B Only"), Some("4")),
        (Some("Ledger"), None, Some("4")),
        (Some("Ledger"), Some("All Sections"), Some("four")),
        (Some("Ledger"), Some("All Sections"), None),
    ] {
        let mut firm = base.clone();
        firm.settings[0].priority = priority.map(str::to_string);
        firm.settings[0].sections = sections.map(str::to_string);
        firm.settings[0].code_length = length.map(str::to_string);
        variants.push(firm);
    }
    // No setting in force, none at all, two on one date.
    let mut later = base.clone();
    later.settings[0].applicable_from = Some("20260901".to_string());
    variants.push(later);
    let mut none = base.clone();
    none.settings.clear();
    variants.push(none);
    let mut twice = base.clone();
    twice.settings.push(twice.settings[0].clone());
    variants.push(twice);
    for firm in &variants {
        assert_eq!(ledger_code(firm), not_measured, "{firm:?}");
        assert_eq!(company_code(firm), not_measured, "{firm:?}");
    }
    // A company row cannot defer to anything; no element at all is not measured.
    assert_eq!(
        company_code(&company(vec![defer("20260401")])),
        not_measured
    );
    let mut absent = base;
    absent.rows = None;
    assert_eq!(company_code(&absent), not_measured);
}

/// The group parser on the captured group listing (the answered request
/// without `HSNDETAILS.LIST`): every group by name, none with HSN/SAC rows,
/// bound to the company; edits of that answer (marked as edits) cover the
/// listing with no group and a name twice.
#[test]
fn the_group_parser_binds_names_and_refuses_a_listing_it_cannot_trust() {
    let answer = utf16(GROUPS);
    let groups = parse_group_hsn(&answer, LAB_GUID).unwrap();
    assert_eq!(groups.len(), 28);
    assert_eq!(groups.get("Sales Accounts"), Some(&None));
    assert!(groups.values().all(Option::is_none));
    assert_eq!(
        parse_group_hsn(&answer, "00000000-0000-0000-0000-000000000000"),
        Err("invoice_group_hsn_company_mismatch")
    );
    // Edit: the listing with every group removed.
    let first = answer.find("<GROUP NAME=").unwrap();
    let last = answer.rfind("</GROUP>").unwrap() + "</GROUP>".len();
    let empty = format!("{}{}", &answer[..first], &answer[last..]);
    assert_eq!(
        parse_group_hsn(&empty, LAB_GUID),
        Err("invoice_group_hsn_listing_empty")
    );
    // Edit: the first group twice.
    let end = first + answer[first..].find("</GROUP>").unwrap() + "</GROUP>".len();
    let twice = format!("{}{}", &answer[..end], &answer[first..]);
    assert_eq!(
        parse_group_hsn(&twice, LAB_GUID),
        Err("invoice_group_hsn_name_repeated")
    );
}

/// The row flags, on edits of the captured company object (marked as edits):
/// an element the lab's rows did not carry marks the row unmeasured, and a
/// row emptied to the placeholder reads as the company having no code.
#[test]
fn a_row_with_an_unknown_element_or_no_content_is_flagged_so() {
    let answer = utf16(COMPANY_OBJECT);
    let start = answer.find("<HSNDETAILS.LIST>").unwrap() + "<HSNDETAILS.LIST>".len();
    let end = answer.find("</HSNDETAILS.LIST>").unwrap();
    // Edit: one more element in the row.
    let extra = format!(
        "{}<HSNEXTRA>1</HSNEXTRA>{}",
        &answer[..start],
        &answer[start..]
    );
    let firm = parse_company_hsn(&extra, LAB_GUID).unwrap();
    let rows = firm.rows.as_ref().unwrap();
    assert!(rows[0].unmeasured && !rows[0].empty);
    // Edit: the row emptied.
    let emptied = format!("{} {}", &answer[..start], &answer[end..]);
    let firm = parse_company_hsn(&emptied, LAB_GUID).unwrap();
    let rows = firm.rows.as_ref().unwrap();
    assert!(rows[0].empty && !rows[0].unmeasured);
    let deferring = resolve_hsn(
        &ledger(vec![defer("20260401")]),
        Some(&top_chain()),
        Some(&chart(vec![], vec![placeholder()])),
        &firm,
        DATE,
    );
    assert_eq!(deferring, HsnStep::Refused(UNRESOLVED));
}

/// The three answers of the lab request set W16 (one book state, 12 Oct
/// 2026), read by the code's own parsers, resolve each sales ledger to the
/// code and the level Tally's own GST rate setup screens showed in that run;
/// the ledger whose row in force defers beside a later row, and a ledger
/// carrying only the empty element (a tax ledger here), are refused as not
/// measured.
#[test]
fn on_the_labs_answers_each_sales_ledger_resolves_as_tallys_screens_showed() {
    let names = [
        "BRIDGE Svc 998313 5%",
        "BRIDGE Svc 998314 5%",
        "BRIDGE Svc 998315 5%",
        "BRIDGE Svc 4D 5%",
        "BRIDGE Goods 18%",
        "BRIDGE Svc INH 5%",
        "BRIDGE Svc INH2 5%",
        "BRIDGE SGST 2.5%",
    ];
    let rates = super::super::parse_ledger_rates(&utf16(RATES_HSN), LAB_GUID, &names).unwrap();
    assert_eq!(rates.len(), names.len());
    let groups = parse_group_hsn(&utf16(GROUPS_HSN), LAB_GUID).unwrap();
    assert_eq!(groups.len(), 29);
    let placeholders = groups
        .values()
        .filter(|rows| matches!(rows.as_deref(), Some([row]) if row.empty))
        .count();
    assert_eq!(placeholders, 27);
    let firm = parse_company_hsn(&utf16(COMPANY_OBJECT), LAB_GUID).unwrap();
    let step = |name: &str, chain: &[String], date: &str| {
        resolve_hsn(&rates[name].hsn, Some(chain), Some(&groups), &firm, date)
    };
    let (top, sub) = (top_chain(), sub_chain());
    for (name, code) in [
        ("BRIDGE Svc 998313 5%", "998313"),
        ("BRIDGE Svc 998314 5%", "998314"),
        ("BRIDGE Svc 4D 5%", "9983"),
        ("BRIDGE Goods 18%", "847130"),
        ("BRIDGE Svc INH 5%", "998344"),
    ] {
        assert_eq!(
            step(name, &top, DATE),
            resolved(code, HsnLevel::Ledger),
            "{name}"
        );
    }
    // A later row is used from its own date.
    assert_eq!(
        step("BRIDGE Svc INH 5%", &top, "20260901"),
        resolved("998345", HsnLevel::Ledger)
    );
    assert_eq!(
        step("BRIDGE Svc 998315 5%", &top, DATE),
        resolved("998315", HsnLevel::Ledger)
    );
    assert_eq!(
        step("BRIDGE Svc 998315 5%", &top, "20260801"),
        HsnStep::Refused(NOT_MEASURED)
    );
    // Deferring to its sub-group, as the screen showed `(998343)` for it.
    assert_eq!(
        step("BRIDGE Svc INH2 5%", &sub, DATE),
        resolved("998343", HsnLevel::Group("BRIDGE Sales Sub".to_string()))
    );
    assert_eq!(
        step("BRIDGE SGST 2.5%", &top, DATE),
        HsnStep::Refused(NOT_MEASURED)
    );
}

/// The type of supply is read from the lab's rate listing as Tally sent it:
/// Services on the services ledgers, Goods on the goods ledger, and nothing
/// on the round-off ledger, which no rule then judges. A repeated element
/// (an edit of the answer) is not judged either, and fails nothing.
#[test]
fn the_type_of_supply_is_read_from_the_labs_listing_and_never_fails_it() {
    let names = [
        "BRIDGE Svc 998313 5%",
        "BRIDGE Goods 18%",
        "BRIDGE Round Off",
    ];
    let rates = super::super::parse_ledger_rates(&utf16(RATES_HSN), LAB_GUID, &names).unwrap();
    let supply = names.map(|name| rates[name].supply);
    assert_eq!(
        supply,
        [
            SupplyType::Services,
            SupplyType::Goods,
            SupplyType::NotJudged
        ]
    );
    // Edited: the services ledger's element written twice.
    let answer = utf16(RATES_HSN);
    let one = "<GSTTYPEOFSUPPLY TYPE=\"String\">Services</GSTTYPEOFSUPPLY>";
    let at = answer[answer.find("NAME=\"BRIDGE Svc 998313 5%\"").unwrap()..]
        .find(one)
        .unwrap()
        + answer.find("NAME=\"BRIDGE Svc 998313 5%\"").unwrap();
    let twice = format!("{}{one}{}", &answer[..at], &answer[at..]);
    let rates =
        super::super::parse_ledger_rates(&twice, LAB_GUID, &["BRIDGE Svc 998313 5%"]).unwrap();
    assert_eq!(rates["BRIDGE Svc 998313 5%"].supply, SupplyType::NotJudged);
}

/// Each rule on the codes the lab's ledgers resolved to (998313 and 847130
/// clean, the four-digit 9983 not six digits), and on edited codes and types
/// for the rules no lab ledger trips. A code that is not digits only is
/// judged on that alone.
#[test]
fn the_warnings_follow_the_code_and_the_type_of_supply() {
    use HsnWarningCode::*;
    use SupplyType::{Goods, NotJudged, Services};
    let codes = |code: &str, supply| {
        hsn_warnings("L", code, &HsnLevel::Ledger, supply)
            .into_iter()
            .map(|warning| warning.code)
            .collect::<Vec<_>>()
    };
    let cases: [(&str, SupplyType, &[HsnWarningCode]); 11] = [
        // The lab's.
        ("998313", Services, &[]),
        ("998314", Services, &[]),
        ("9983", Services, &[NotSixDigits]),
        ("847130", Goods, &[]),
        // Edited.
        ("84713010", Goods, &[NotSixDigits]),
        ("847130", Services, &[NotASacForServices]),
        ("998313", Goods, &[SacForGoods]),
        ("8471", Services, &[NotSixDigits, NotASacForServices]),
        ("998313", NotJudged, &[TypeOfSupplyNotJudged]),
        ("9983 13", Services, &[NotDigits]),
        ("99831A", Goods, &[NotDigits]),
    ];
    for (code, supply, expected) in cases {
        assert_eq!(codes(code, supply), expected, "{code} {supply:?}");
    }
    // The ledger, the level and the code's length travel with each warning.
    assert_eq!(
        hsn_warnings(
            "BRIDGE Svc INH2 5%",
            "9983",
            &HsnLevel::Group("BRIDGE Sales Sub".to_string()),
            Services
        ),
        [HsnWarning {
            code: NotSixDigits,
            ledger: "BRIDGE Svc INH2 5%".to_string(),
            level: HsnLevel::Group("BRIDGE Sales Sub".to_string()),
            characters: 4,
        }]
    );
}

/// A warning's code is written in a saved batch as the response names it.
#[test]
fn a_warning_code_is_saved_under_the_name_the_response_gives_it() {
    use HsnWarningCode::*;
    for code in [
        NotDigits,
        NotSixDigits,
        NotASacForServices,
        SacForGoods,
        TypeOfSupplyNotJudged,
    ] {
        assert_eq!(serde_json::to_value(code).unwrap(), code.as_str());
    }
}
