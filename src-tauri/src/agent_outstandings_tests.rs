use super::*;

#[test]
fn opposing_unallocated_direction_is_preserved_and_only_gross_exposure_is_ranked() {
    let bills = vec![OpenBillRow {
        party: "Synthetic Party".into(),
        reference: "INV-1".into(),
        bill_date: "20260901".into(),
        due_date: "20260901".into(),
        amount: bridge_tally_core::ExactDecimal::parse("100").unwrap(),
        age_days: Some(5),
        kind: ExposureDirection::Receivable,
    }];
    let unallocated = vec![UnallocatedParty {
        party: "Synthetic Party".into(),
        amount: bridge_tally_core::ExactDecimal::parse("30").unwrap(),
        direction: ExposureDirection::Payable,
        opening_balance: None,
        composition: None,
    }];
    let ranked = ranked_parties_from_exposure(&bills, &unallocated, 1).unwrap();
    let party = &ranked[0];
    assert_eq!(party["gross_exposure"], "130");
    assert_eq!(party["gross_billed"], "100");
    assert_eq!(party["billed_receivable"], "100");
    assert_eq!(party["billed_payable"], "0");
    assert_eq!(party["unallocated_receivable"], "0");
    assert_eq!(party["unallocated_payable"], "30");
    assert_eq!(party["gross_unallocated"], "30");
    for absent in ["outstanding_total", "net_due", "billed", "unallocated"] {
        assert!(party.get(absent).is_none(), "ambiguous total {absent}");
    }
    let residuals = unallocated_totals_from_parties(&unallocated).unwrap();
    assert_eq!(
        residuals,
        json!({"receivable":"0", "payable":"30", "gross_unallocated":"30", "by_composition": {"not_bill_wise_ledger":{"receivable":"0","payable":"0"}, "bill_wise_ledger_components_not_separated":{"receivable":"0","payable":"0"}, "composition_not_observed":{"receivable":"0","payable":"30"}}})
    );
    let billed = outstanding_totals_from_open_bills(&bills).unwrap();
    assert_eq!(billed["scope"], "open_bills_only");
    assert_eq!(billed["receivable"], "100");
    assert_eq!(billed["payable"], "0");
}

/// bridge#551: a currency refusal names its ledger on the MCP result, masked
/// like any party name; a reason without a ledger carries none.
#[test]
fn a_withheld_result_names_its_ledger_under_redaction() {
    let mut reason =
        crate::tally::OutstandingsPartialReason::code("ledger_currency_base_unmatched");
    reason.foreign_currency_ledger_name = Some("Synthetic FX Debtor".to_string());
    let plain = partial_payload(&reason, Redaction::None);
    assert_eq!(plain["partial_reason"], "ledger_currency_base_unmatched");
    assert_eq!(plain["ledger"], "Synthetic FX Debtor");
    let masked = partial_payload(&reason, Redaction::MaskParties);
    assert_eq!(masked["ledger"], json!(mask("Synthetic FX Debtor")));
    assert_ne!(masked["ledger"], "Synthetic FX Debtor");
    let unnamed = partial_payload(
        &crate::tally::OutstandingsPartialReason::code("native_bills_report_drifted"),
        Redaction::MaskParties,
    );
    assert_eq!(
        unnamed,
        json!({"state":"partial","partial_reason":"native_bills_report_drifted"})
    );
}

/// bridge#551: the foreign-balance refusal, which predates the currency
/// classification, names its ledger on the MCP result through the same
/// party-name redaction.
#[test]
fn the_foreign_balance_refusal_names_its_ledger_under_redaction() {
    let reason = crate::tally::OutstandingsPartialReason::foreign_currency_ledger_balance(
        "Synthetic FX Debtor".to_string(),
    );
    let plain = partial_payload(&reason, Redaction::None);
    assert_eq!(
        plain,
        json!({
            "state": "partial",
            "partial_reason": "company_foreign_currency_ledger_balance",
            "ledger": "Synthetic FX Debtor",
        })
    );
    let masked = partial_payload(&reason, Redaction::MaskParties);
    assert_eq!(masked["ledger"], json!(mask("Synthetic FX Debtor")));
    assert_ne!(masked["ledger"], "Synthetic FX Debtor");
}

/// The MCP `outstandings` call on the FOREX book's captured sequence, with
/// the native read's four sources given, and FOREX's own extent capture when
/// `extent` is given (else a generic lab extent): every scripted response is
/// served.
async fn forex_outstandings(
    extent: Option<&[u8]>,
    receivable: &[u8],
    groups: &[u8],
    payable: &[u8],
    ledgers: &[u8],
    as_of: &str,
    redaction: Redaction,
) -> Value {
    use tally_protocol_simulator::{
        Fixture, ProductStatus, ScenarioPlan, SequenceSimulator, WireEncoding,
    };
    fn decode(bytes: &[u8]) -> String {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|unit| u16::from_le_bytes([unit[0], unit[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
    let xml = |body: String| {
        ScenarioPlan::new(Fixture::SyntheticXml(body)).with_encoding(WireEncoding::Utf16Le)
    };
    let status = || ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    let pair = |plans: &mut Vec<ScenarioPlan>, source: ScenarioPlan| {
        plans.extend([source.clone(), status(), source, status()]);
    };
    let companies = xml(decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    )));
    let captured = |bytes: &[u8]| xml(decode(bytes));
    let extent = match extent {
        Some(bytes) => captured(bytes),
        None => xml(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )
        .to_string()),
    };

    let mut plans = Vec::new();
    pair(&mut plans, companies.clone());
    // The classified currency read: plain, then with ORIGINALNAME, then the
    // Company collection.
    plans.push(companies.clone());
    pair(&mut plans, extent.clone());
    for source in [
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
        )),
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_originalname_forex_live.utf16le.xml"
        )),
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/company_currencyname_live.utf16le.xml"
        )),
    ] {
        pair(&mut plans, source);
    }
    pair(&mut plans, extent.clone());
    plans.push(companies.clone());
    // The native outstandings read.
    plans.extend([status(), companies.clone(), companies.clone()]);
    pair(&mut plans, extent.clone());
    for source in [
        captured(receivable),
        captured(groups),
        captured(payable),
        captured(ledgers),
    ] {
        pair(&mut plans, source);
    }
    pair(&mut plans, extent);
    plans.extend([companies.clone(), status(), companies]);
    let plan_count = plans.len();

    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let response = server
        .call_tool(
            "outstandings",
            json!({"company_guid":"b14e9b2d-8a63-4779-804d-25d59eb787eb","as_of":as_of}),
        )
        .await;
    assert_eq!(simulator.finish().unwrap().len(), plan_count);
    response
}

/// bridge#551, through the MCP tool itself on FOREX's captures, with party
/// names masked: a book with an `I₹` and a `$` master comes back partial,
/// with the rupee ledgers' figures under `base_currency_ledgers`, no figure
/// for the whole book, and the three `$` ledgers listed with their currency,
/// their names masked like any party's.
#[tokio::test]
async fn mcp_outstandings_report_base_currency_ledgers_only_on_forex() {
    let response = forex_outstandings(
        None,
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/bills_receivable_forex_live.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/groups_forex_live.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/bills_payable_forex_live.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/ledgers_currency_forex_live.utf16le.xml"
        ),
        "20250930",
        Redaction::MaskParties,
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let content = &response["structuredContent"];
    assert_eq!(content["evidence"]["state"], "partial");
    let result = &content["result"];
    assert_eq!(result["state"], "partial");
    // One scalar for any mix; the derived set names only the foreign list,
    // as this capture predates the dollar invoice to a rupee party (#642).
    assert_eq!(result["partial_reason"], "currency_ledgers_excluded");
    assert_eq!(
        result["partial_reasons"],
        json!(["foreign_currency_ledgers_excluded"])
    );
    assert_eq!(
        content["evidence"]["reason_code"],
        "foreign_currency_ledgers_excluded"
    );
    assert_eq!(result["base_currency_ledgers_mixed_excluded"]["count"], 0);
    for book_level in [
        "totals",
        "ageing_buckets",
        "top_parties",
        "open_bills",
        "unallocated",
    ] {
        assert!(
            result.get(book_level).is_none(),
            "{book_level} at book level"
        );
    }
    let base = &result["base_currency_ledgers"];
    assert_eq!(base["totals"]["receivable"], "34500");
    assert_eq!(base["open_bills"].as_array().unwrap().len(), 14);
    let excluded = &result["foreign_currency_ledgers_excluded"];
    assert_eq!(excluded["count"], 3);
    let ledgers = excluded["ledgers"].as_array().unwrap();
    assert_eq!(ledgers.len(), 3);
    assert!(ledgers.iter().all(|ledger| ledger["currency"] == "$"));
    let text = response.to_string();
    for name in [
        "BRIDGE FX DEBTOR A",
        "FX USD Debtor 01",
        "FX USD Debtor 02",
        "FX Party",
    ] {
        assert!(!text.contains(name), "{name} unmasked");
    }
}

/// bridge#642: a book with several Currency masters but no ledger kept in
/// another currency, where only rupee ledgers with a composite value are set
/// aside, is still partial: never Complete with a party silently missing. It is
/// the captured post-invoice book with its three `$` ledgers' CURRENCYNAME
/// rewritten to the base `I₹`, and nothing else changed: two of them then hold a
/// composite and join the three mixed rupee ledgers; the third closes 0.00.
#[tokio::test]
async fn a_book_with_only_mixed_ledgers_set_aside_is_still_partial() {
    let snapshot: &[u8] = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/balance_snapshot_forex_live.utf16le.xml"
    );
    let text = String::from_utf16(
        &snapshot
            .chunks_exact(2)
            .map(|unit| u16::from_le_bytes([unit[0], unit[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let dollar = r#"<CURRENCYNAME TYPE="String">$</CURRENCYNAME>"#;
    assert_eq!(text.matches(dollar).count(), 3);
    let rupees_only = text
        .replace(dollar, r#"<CURRENCYNAME TYPE="String">I₹</CURRENCYNAME>"#)
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<u8>>();
    let response = forex_outstandings(
        Some(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
        )),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/bills_receivable_forex_post_c1_live.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/group_snapshot_forex_live.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/bills_payable_forex_post_c1_live.utf16le.xml"
        ),
        &rupees_only,
        "20260915",
        Redaction::None,
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let content = &response["structuredContent"];
    let result = &content["result"];
    assert_eq!(result["state"], "partial", "{result}");
    assert_eq!(content["evidence"]["state"], "partial");
    assert_eq!(result["partial_reason"], "currency_ledgers_excluded");
    assert_eq!(
        result["partial_reasons"],
        json!(["mixed_currency_ledgers_excluded"])
    );
    assert_eq!(result["foreign_currency_ledgers_excluded"]["count"], 0);
    let mixed = &result["base_currency_ledgers_mixed_excluded"];
    assert_eq!(mixed["count"], 5);
    assert_eq!(
        mixed["ledgers"],
        json!([
            "BRIDGE FX DEBTOR A",
            "FX Party 01",
            "FX Sales",
            "FX USD Debtor 02",
            "Profit & Loss A/c"
        ])
    );
    for book_level in ["totals", "ageing_buckets", "top_parties", "open_bills"] {
        assert!(
            result.get(book_level).is_none(),
            "{book_level} at book level"
        );
    }
    let bills = result["base_currency_ledgers"]["open_bills"]
        .as_array()
        .unwrap();
    for bill in bills {
        let party = bill["party"].as_str().unwrap();
        assert!(
            !mixed["ledgers"]
                .as_array()
                .unwrap()
                .iter()
                .any(|ledger| ledger == party),
            "a set-aside party's bill is listed: {party}"
        );
    }
    assert!(
        !response.to_string().contains(" @ "),
        "a composite reached the response"
    );
}

/// bridge#642, through the MCP tool on the book after a dollar invoice to the
/// rupee party `FX Party 01`, every source from one moment of the book
/// (FOREX_601D_CAPTURE_PROVENANCE, PARTIAL). That party's balance is a
/// composite, so it is set aside by name with its five bills (20,100), as are
/// the two other rupee ledgers with a composite value. The result is partial
/// with the one scalar reason and both derived reasons; its receivable is the
/// plain rupee parties' 23,000; no composite reaches the response. The
/// balance snapshot's period ends after the book's last voucher date, so its
/// closings are those at this as-of date.
#[tokio::test]
async fn mcp_outstandings_set_a_mixed_party_aside_with_its_bills() {
    let response = forex_outstandings(
        Some(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
        )),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/bills_receivable_forex_post_c1_live.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/group_snapshot_forex_live.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/bills_payable_forex_post_c1_live.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/balance_snapshot_forex_live.utf16le.xml"
        ),
        "20260915",
        Redaction::None,
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let content = &response["structuredContent"];
    let result = &content["result"];
    assert_eq!(result["state"], "partial");
    assert_eq!(result["partial_reason"], "currency_ledgers_excluded");
    assert_eq!(
        result["partial_reasons"],
        json!([
            "foreign_currency_ledgers_excluded",
            "mixed_currency_ledgers_excluded"
        ])
    );
    assert_eq!(
        content["evidence"]["reason_code"],
        "foreign_currency_ledgers_excluded+mixed_currency_ledgers_excluded"
    );
    let mixed = &result["base_currency_ledgers_mixed_excluded"];
    assert_eq!(mixed["count"], 3);
    assert_eq!(mixed["reason"], "mixed_currency_movement");
    assert_eq!(
        mixed["ledgers"],
        json!(["FX Party 01", "FX Sales", "Profit & Loss A/c"])
    );
    assert_eq!(result["foreign_currency_ledgers_excluded"]["count"], 3);
    let base = &result["base_currency_ledgers"];
    assert_eq!(base["totals"]["receivable"], "23000");
    let bills = base["open_bills"].as_array().unwrap();
    // BRIDGE INR DEBTOR A 2, FX Party 02 4, FX Party 03 4.
    assert_eq!(bills.len(), 10);
    let parties = bills
        .iter()
        .map(|bill| bill["party"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        parties,
        ["BRIDGE INR DEBTOR A", "FX Party 02", "FX Party 03"].into()
    );
    let text = response.to_string();
    assert!(!text.contains(" @ "), "a composite reached the response");
    assert!(
        !text.contains("FX-USD-ON-INR-1"),
        "the mixed party's bill is listed"
    );
}

#[test]
fn unallocated_totals_split_by_composition_and_the_parts_add_up() {
    use crate::tally::UnallocatedComposition::{
        BillWiseLedgerComponentsNotSeparated, NotBillWiseLedger,
    };
    let party =
        |name: &str,
         amount: &str,
         direction: ExposureDirection,
         composition: Option<crate::tally::UnallocatedComposition>| UnallocatedParty {
            party: name.into(),
            amount: bridge_tally_core::ExactDecimal::parse(amount).unwrap(),
            direction,
            opening_balance: None,
            composition,
        };
    let parties = [
        party(
            "A",
            "7500",
            ExposureDirection::Receivable,
            Some(NotBillWiseLedger),
        ),
        party(
            "B",
            "20000",
            ExposureDirection::Receivable,
            Some(BillWiseLedgerComponentsNotSeparated),
        ),
        party(
            "C",
            "3000",
            ExposureDirection::Payable,
            Some(BillWiseLedgerComponentsNotSeparated),
        ),
        party("D", "100", ExposureDirection::Receivable, None),
    ];
    let totals = unallocated_totals_from_parties(&parties).unwrap();
    assert_eq!(totals["receivable"], "27600");
    assert_eq!(totals["payable"], "3000");
    let split = &totals["by_composition"];
    assert_eq!(
        split["not_bill_wise_ledger"],
        json!({"receivable": "7500", "payable": "0"})
    );
    assert_eq!(
        split["bill_wise_ledger_components_not_separated"],
        json!({"receivable": "20000", "payable": "3000"})
    );
    // A row with no composition is neither hidden nor folded into another one.
    assert_eq!(
        split["composition_not_observed"],
        json!({"receivable": "100", "payable": "0"})
    );
    let text = totals.to_string().to_ascii_lowercase();
    assert!(!text.contains("on_account") && !text.contains("on account"));
    // With no row lacking a composition, that key is absent.
    let without = unallocated_totals_from_parties(&parties[..3]).unwrap();
    assert!(without["by_composition"]
        .get("composition_not_observed")
        .is_none());
}

#[test]
fn an_unallocated_row_carries_its_composition_and_opening_through_the_json_and_redaction() {
    use crate::tally::UnallocatedComposition::NotBillWiseLedger;
    let row = UnallocatedParty {
        party: "Synthetic Debtor".into(),
        amount: bridge_tally_core::ExactDecimal::parse("7500").unwrap(),
        direction: ExposureDirection::Receivable,
        opening_balance: Some(bridge_tally_core::ExactDecimal::parse("-100.00").unwrap()),
        composition: Some(NotBillWiseLedger),
    };
    let json = redact_value(unallocated_party_json(&row), Redaction::MaskParties);
    assert_eq!(json["ledger_bill_wise"], false);
    assert_eq!(json["opening_balance"], "-100.00");
    assert_eq!(json["composition"], "not_bill_wise_ledger");
    assert_eq!(json["amount"], "7500");
    assert_eq!(json["direction"], "receivable");
    // The party name is what redaction masks; the composition fields stay.
    assert_ne!(json["party"], "Synthetic Debtor");
}
