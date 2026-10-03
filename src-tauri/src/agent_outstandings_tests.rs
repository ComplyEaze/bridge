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
#[allow(clippy::too_many_arguments)]
async fn forex_outstandings(
    extent: Option<&[u8]>,
    receivable: &[u8],
    groups: &[u8],
    payable: &[u8],
    ledgers: &[u8],
    as_of: &str,
    redaction: Redaction,
    extra_arguments: Value,
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
        .call_tool("outstandings", {
            let mut arguments =
                json!({"company_guid":"b14e9b2d-8a63-4779-804d-25d59eb787eb","as_of":as_of});
            for (key, value) in extra_arguments.as_object().into_iter().flatten() {
                arguments[key] = value.clone();
            }
            arguments
        })
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
        json!({}),
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let content = &response["structuredContent"];
    assert_eq!(content["evidence"]["state"], "partial");
    let result = &content["result"];
    assert_eq!(result["state"], "partial");
    assert_eq!(result["as_of"], "20250930");
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
    // The counts sit with the figures they describe, under the same key.
    assert_eq!(base["open_bills_total"], 14);
    assert_eq!(base["open_bills_shown"], 14);
    assert!(result.get("open_bills_total").is_none(), "{result}");
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
        json!({}),
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
        json!({}),
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

#[test]
fn the_outstandings_description_says_what_decides_receivable_and_payable() {
    // Tally's Bills Receivable and Bills Payable reports scope by the sign of each
    // bill's balance and carry no bill type, so a customer's advance lands under
    // payable. A reader who takes "payable" as "owed to suppliers" is wrong by
    // the advances and credit notes; the description is what an agent reads.
    let definitions = tool_definitions(true, false);
    let description = definitions
        .as_array()
        .and_then(|tools| tools.iter().find(|tool| tool["name"] == "outstandings"))
        .expect("outstandings tool definition")["description"]
        .as_str()
        .expect("tool description");
    for needle in [
        "follow the sign of each bill's balance",
        "not the type of party",
        "a customer's advance or a credit note raised to a customer appears under payable",
        "a supplier's advance or a debit note raised to a supplier under receivable",
        "That holds for an advance or a note kept as its own bill",
        "an on-account advance goes to the unallocated figure instead",
        "a credit note set against an open invoice reduces that invoice",
        "Measured on one synthetic book (TallyPrime Silver 7.1)",
        "Read a bill's `kind` as a direction",
        "net into one figure",
        "the date used is always returned as `result.as_of`, whatever the state",
    ] {
        assert!(description.contains(needle), "missing: {needle}");
    }
}

/// What an agent must know before it relies on a party detail is in the
/// description it reads: what `tied` does and does not prove, what an absent
/// residual row and an empty read mean, and what the detail costs.
#[test]
fn the_outstandings_description_carries_the_party_details_caveats() {
    let definitions = tool_definitions(true, false);
    let description = definitions
        .as_array()
        .and_then(|tools| tools.iter().find(|tool| tool["name"] == "outstandings"))
        .expect("outstandings tool definition")["description"]
        .as_str()
        .expect("tool description");
    for needle in [
        "`tied` means the two figures are equal, not that the composition is proven",
        "two changes that compensate, or allocations that net to zero, can still read `tied`",
        "`no_residual_row_for_party` (with `residual` null)",
        "`no_named_bill_for_party`",
        "`window_returned_no_vouchers`",
        "from the whole company's vouchers",
        "its cost is that of a `vouchers` read over the same span, which is unmeasured on a large book",
        "any refusal of that read fails the whole `outstandings` call",
        "`unadjusted_detail_too_large` (nothing narrows it",
        "with the read's own `partial_reason`",
        "Passing `detail` is the request to read the company's vouchers from the start of the books",
        "neither cancelled nor optional",
        "`row_amounts: as_allocated`",
        "never net of what later allocations adjusted against its reference",
        "Tally's own `native_balance`",
        "`trail_window_too_large` (name a `reference`",
        "a window of more than 5,376 of the company's vouchers is always refused",
        "`named_bill_window_too_large` (a reference was named already",
        "told apart only by `native_rows`",
        "`unadjusted_window_too_large` (nothing narrows it",
        "`reads.needed_at_least` against `reads.allowed`",
        "The limit counts allocations, not bills",
        "`agent_response_too_large`",
        "One foreign-currency composite voucher anywhere in the window fails it",
        "can tie when named",
    ] {
        assert!(description.contains(needle), "missing: {needle}");
    }
}

/// The party detail's conflicting arguments are refused before any read: the
/// tool names the code before any request is attempted.
#[tokio::test]
async fn mcp_outstandings_refuse_a_conflicting_detail_request_before_any_read() {
    // Nothing listens here. A refusal that came after a read would carry a
    // connection error's code, so the expected refusal code below is also the
    // proof that no request was attempted.
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 1,
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let guid = "b14e9b2d-8a63-4779-804d-25d59eb787eb";
    for (extra, code) in [
        (json!({"party": "P"}), "party_requires_detail"),
        (json!({"detail": "bill_trail"}), "detail_requires_party"),
        // The schema's enum refuses an unknown kind before the handler runs;
        // `invalid_detail` is the handler's own guard behind it.
        (
            json!({"party": "P", "detail": "everything"}),
            "argument_invalid:detail",
        ),
        (
            json!({"party": "P", "detail": "unadjusted", "reference": "R"}),
            "reference_requires_bill_trail",
        ),
    ] {
        let mut arguments = json!({"company_guid": guid});
        for (key, value) in extra.as_object().unwrap() {
            arguments[key] = value.clone();
        }
        let response = server.call_tool("outstandings", arguments).await;
        assert_eq!(response["isError"], true, "{extra}: {response}");
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"], code,
            "{extra}"
        );
    }
}

/// A party detail is tied against the whole book's bills, so a partial read
/// (here: ledgers kept in another currency) refuses it rather than tying
/// against a subset.
#[tokio::test]
async fn mcp_outstandings_refuse_a_party_detail_on_a_partial_read() {
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
        Redaction::None,
        json!({"party": "BRIDGE FX DEBTOR A", "detail": "bill_trail"}),
    )
    .await;
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "detail_requires_a_complete_read");
    // The read's own reason stays in band beside the refusal's code.
    assert_eq!(
        error["partial_reason"], "currency_ledgers_excluded",
        "{error}"
    );
    assert_eq!(
        error["partial_reasons"],
        json!(["foreign_currency_ledgers_excluded"]),
        "{error}"
    );
}

/// The ageing lab book's captured `outstandings` sequence (currency read, then
/// the native read), with `ledgers` as the native read's ledger source, called
/// through the MCP tool with `extra_arguments`. The sequence is the one
/// `currency_then_native_plans_with_ledgers` scripts for the runtime.
async fn ageing_outstandings(ledgers: String, extra_arguments: Value) -> Value {
    ageing_outstandings_with(None, ledgers, extra_arguments).await
}

/// [`ageing_outstandings`] with the Bills Receivable report replaced when `receivable` is given.
async fn ageing_outstandings_with(
    receivable: Option<String>,
    ledgers: String,
    extra_arguments: Value,
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
    let extent = xml(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    )
    .to_string());
    let mut plans = Vec::new();
    pair(&mut plans, companies.clone());
    // The one-master currency read.
    plans.push(companies.clone());
    pair(&mut plans, extent.clone());
    pair(
        &mut plans,
        xml(decode(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        ))),
    );
    pair(&mut plans, extent.clone());
    plans.push(companies.clone());
    // The native outstandings read.
    plans.extend([status(), companies.clone(), companies.clone()]);
    pair(&mut plans, extent.clone());
    let receivable = receivable.unwrap_or_else(|| {
        decode(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-receivable.utf16le.xml"
        ))
    });
    for body in [
        receivable,
        decode(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-groups.utf16le.xml"
        )),
        decode(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-payable.utf16le.xml"
        )),
    ] {
        pair(&mut plans, xml(body));
    }
    pair(&mut plans, xml(ledgers));
    pair(&mut plans, extent);
    plans.extend([companies.clone(), status(), companies]);
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
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let mut arguments =
        json!({"company_guid": "eebb9a9f-1679-4468-9e8f-814c729674cb", "as_of": "20260801"});
    for (key, value) in extra_arguments.as_object().into_iter().flatten() {
        // A null leaves the argument out, to exercise its default.
        if value.is_null() {
            arguments.as_object_mut().unwrap().remove(key);
        } else {
            arguments[key] = value.clone();
        }
    }
    let response = server.call_tool("outstandings", arguments).await;
    simulator.cancel();
    response
}

/// The captured ageing ledgers predate `CURRENCYNAME`; this is the labelled
/// edit `ageing_ledgers_with_currency` makes for the runtime's tests: the
/// field in its captured position on every row, the book's single master
/// `I₹` on every row but `Ageing Customer A`, which gets `customer_a`.
fn ageing_ledgers_with_currency(customer_a: &str) -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-ledgers.utf16le.xml"
    );
    let captured = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|unit| u16::from_le_bytes([unit[0], unit[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let mut pieces = captured.split("<LEDGER ");
    let mut ledgers = pieces.next().unwrap().to_string();
    let mut rows = 0;
    for piece in pieces {
        let tag_end = piece.find('>').unwrap() + 1;
        let currency = if piece.starts_with("NAME=\"Ageing Customer A\"") {
            customer_a
        } else {
            "I\u{20b9}"
        };
        ledgers.push_str("<LEDGER ");
        ledgers.push_str(&piece[..tag_end]);
        ledgers.push_str(&format!(
            "\r\n     <CURRENCYNAME TYPE=\"String\">{currency}</CURRENCYNAME>"
        ));
        ledgers.push_str(&piece[tag_end..]);
        rows += 1;
    }
    assert_eq!(rows, 6);
    ledgers
}

/// A call without `as_of` reads as of the host's today, and says so: the date
/// is echoed in the result, so a figure is never left to be read as of
/// whatever date the caller assumed. Bracketed so a midnight crossing cannot
/// fail the test.
#[tokio::test]
async fn mcp_outstandings_echo_the_date_they_defaulted_to() {
    let before = tally_host_today();
    let response =
        ageing_outstandings(ageing_ledgers_with_currency("I₹"), json!({"as_of": null})).await;
    let after = tally_host_today();
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    // The captured book does not accept today's date (its read comes back
    // partial, `native_outstandings_as_of_refused`): the date is echoed in
    // that state too, and the state is not what this test is about.
    assert!(result["state"].is_string(), "{result}");
    assert!(
        result["as_of"] == before || result["as_of"] == after,
        "{result}"
    );
}

/// A hyphenated `as_of` is accepted and echoed in the compact form the read
/// used, so the echo is the date read and not the caller's spelling of it.
#[tokio::test]
async fn mcp_outstandings_echo_a_hyphenated_date_in_compact_form() {
    let response = ageing_outstandings(
        ageing_ledgers_with_currency("I₹"),
        json!({"as_of": "2026-08-01"}),
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    assert_eq!(response["structuredContent"]["result"]["as_of"], "20260801");
}

/// The `Partial` arm: a ledger kept in a currency the book's base does not
/// match makes the native read an in-band partial, and a party detail asked of
/// it is refused with the read's own reason beside the refusal's code.
#[tokio::test]
async fn mcp_outstandings_keep_the_partial_reason_when_refusing_a_party_detail() {
    // Without a detail the same read is the in-band partial the refusal names.
    let plain = ageing_outstandings(ageing_ledgers_with_currency("$"), json!({})).await;
    assert_eq!(plain["isError"], false, "{plain}");
    let result = &plain["structuredContent"]["result"];
    assert_eq!(result["state"], "partial", "{result}");
    assert_eq!(result["partial_reason"], "ledger_currency_base_unmatched");
    // The withheld arm states the date it was read at, too.
    assert_eq!(result["as_of"], "20260801", "{result}");

    let response = ageing_outstandings(
        ageing_ledgers_with_currency("$"),
        json!({"party": "Ageing Customer A", "detail": "unadjusted"}),
    )
    .await;
    assert_eq!(response["isError"], true, "{response}");
    let content = &response["structuredContent"];
    let error = &content["result"]["error"];
    assert_eq!(error["code"], "detail_requires_a_complete_read", "{error}");
    assert_eq!(
        error["partial_reason"], "ledger_currency_base_unmatched",
        "{error}"
    );
    // This arm has no derived list, so none is invented.
    assert!(error.get("partial_reasons").is_none(), "{error}");
    assert_eq!(
        content["evidence"]["reason_code"],
        "detail_requires_a_complete_read"
    );
    // The refusal keeps the evidence of every read before it: the company,
    // the currency and the outstandings read, the same reads the plain call
    // made.
    let plain_evidence = &plain["structuredContent"]["evidence"];
    assert_eq!(content["evidence"]["bytes"], plain_evidence["bytes"]);
    assert_eq!(
        content["evidence"]["request_sha256"],
        plain_evidence["request_sha256"]
    );
    assert_eq!(
        content["evidence"]["response_sha256"],
        plain_evidence["response_sha256"]
    );
    // The ledger the reason concerns is a party name, and is not carried.
    assert!(!error.to_string().contains("Ageing Customer A"), "{error}");
}

/// One recorded `outstandings` call with a party detail, replayed through the
/// MCP tool on the scripted transport, in the order the live gateway answered
/// it (#945, review P2.1). What the replay serves, exactly:
/// - each POST is answered with the bytes of the capture its sequence record
///   names, as captured (UTF-16LE, no byte-order mark). Three of those
///   captures, the company list, the book extent and the company marks, are
///   the live answers trimmed to the one company's row, as their own records
///   declare, so those three are not byte-identical to what the gateway sent;
/// - each status read is answered with the recorded status body;
/// - the HTTP head is the test double's own, not the gateway's.
///
/// Returns the tool's response, the requests the simulator observed, and the
/// record.
async fn replay_recorded_detail_call(
    record: &str,
) -> (Value, Vec<tally_protocol_simulator::ObservedRequest>, Value) {
    replay_recorded_call(record, |_| true, |arguments| arguments, Redaction::None).await
}

/// [`replay_recorded_detail_call`] serving only the recorded requests `keep`
/// admits, with the call's arguments passed through `arguments`, under
/// `redaction`.
async fn replay_recorded_call(
    record: &str,
    keep: impl Fn(&Value) -> bool,
    arguments: impl Fn(Value) -> Value,
    redaction: Redaction,
) -> (Value, Vec<tally_protocol_simulator::ObservedRequest>, Value) {
    use tally_protocol_simulator::{
        Fixture, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
    };
    let record: Value = serde_json::from_str(record).unwrap();
    let directory_of_fixtures = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("crates/bridge-tally-protocol/tests/fixtures/agent");
    let plans = record["requests"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|request| keep(request))
        .map(|request| match request["method"].as_str().unwrap() {
            "GET" => {
                let bytes = std::fs::read(
                    directory_of_fixtures.join(request["response_fixture"].as_str().unwrap()),
                )
                .unwrap();
                let plan = ScenarioPlan::new(Fixture::SyntheticXml(
                    String::from_utf8(bytes.clone()).unwrap(),
                ))
                .with_encoding(WireEncoding::Utf8)
                .with_framing(ResponseFraming::ContentLength);
                assert_eq!(
                    tally_protocol_simulator::encode(&plan.fixture.body(), plan.encoding),
                    bytes,
                    "the status read is served as recorded"
                );
                plan
            }
            _ => {
                let bytes = std::fs::read(
                    directory_of_fixtures.join(request["response_fixture"].as_str().unwrap()),
                )
                .unwrap();
                let body = String::from_utf16(
                    &bytes
                        .chunks_exact(2)
                        .map(|unit| u16::from_le_bytes([unit[0], unit[1]]))
                        .collect::<Vec<_>>(),
                )
                .unwrap();
                let plan = ScenarioPlan::new(Fixture::SyntheticXml(body))
                    .with_encoding(WireEncoding::Utf16LeNoBom)
                    .with_framing(ResponseFraming::ContentLength);
                assert_eq!(
                    tally_protocol_simulator::encode(&plan.fixture.body(), plan.encoding),
                    bytes,
                    "{} is served byte for byte",
                    request["response_fixture"]
                );
                plan
            }
        })
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes: 2_000_000,
        redaction,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let response = server
        .call_tool("outstandings", arguments(record["arguments"].clone()))
        .await;
    simulator.cancel();
    let observed = simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.cancelled)
        .collect::<Vec<_>>();
    (response, observed, record)
}

/// The replay sent exactly the recorded requests, each POST byte for byte as
/// the live call sent it, and its `detail` is the live call's.
fn assert_replay_matches_the_live_call(
    response: &Value,
    observed: &[tally_protocol_simulator::ObservedRequest],
    record: &Value,
) {
    assert_eq!(response["isError"], false, "{response}");
    let requests = record["requests"].as_array().unwrap();
    assert_eq!(
        observed.len(),
        requests.len(),
        "every recorded request, no more"
    );
    for (sent, recorded) in observed.iter().zip(requests) {
        assert_eq!(sent.method, recorded["method"], "seq {}", recorded["seq"]);
        if let Some(sha) = recorded["request_sha256"].as_str() {
            assert_eq!(sent.request_body_sha256, sha, "seq {}", recorded["seq"]);
        }
    }
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["as_of"], record["arguments"]["as_of"], "{result}");
    // The live answer predates `ledger_match` (#1076); every field it has
    // must still be the live call's, and the party was named exactly.
    let mut detail = result["detail"].clone();
    let ledger_match = detail
        .as_object_mut()
        .and_then(|fields| fields.remove("ledger_match"))
        .expect("the detail says which ledger it read");
    assert_eq!(ledger_match["matched"], "exact", "{ledger_match}");
    assert_eq!(detail, record["answer_detail"]);
}

/// Review P2.1: the `outstandings` tool with `detail: unadjusted`, replayed
/// from the live call's captured responses, runs to the live call's own
/// answer.
#[tokio::test]
async fn mcp_outstandings_answer_a_recorded_unadjusted_detail_as_the_live_call_did() {
    let (response, observed, record) = replay_recorded_detail_call(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-sequence-unadjusted.json"
    ))
    .await;
    assert_replay_matches_the_live_call(&response, &observed, &record);
    assert_detail_reads_are_in_the_evidence(&response, &record).await;
}

/// Review P2.1: `detail: bill_trail` with a `reference`, replayed the same
/// way: the named bill's window starts at its date, and the trail ties.
#[tokio::test]
async fn mcp_outstandings_answer_a_recorded_bill_trail_as_the_live_call_did() {
    let (response, observed, record) = replay_recorded_detail_call(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-sequence-bill-trail.json"
    ))
    .await;
    assert_replay_matches_the_live_call(&response, &observed, &record);
    assert_detail_reads_are_in_the_evidence(&response, &record).await;
}

/// The call's evidence is the plain call's (the same recorded requests
/// without the detail's) plus both bodies of each of the detail's own reads:
/// the ledger catalogue, the company marks and the voucher window.
async fn assert_detail_reads_are_in_the_evidence(response: &Value, record: &Value) {
    const DETAIL_READS: [&str; 3] = [
        "List of Ledgers",
        "Bridge Agent Company High Water",
        "Bridge Agent Vouchers",
    ];
    let is_detail_read = |request: &Value| {
        let seq = request["seq"].as_u64().unwrap();
        (50..=65).contains(&seq)
    };
    let text = record.to_string();
    let (plain, _, _) = replay_recorded_call(
        &text,
        |request| !is_detail_read(request),
        |mut arguments| {
            let arguments_map = arguments.as_object_mut().unwrap();
            for key in ["party", "detail", "reference"] {
                arguments_map.remove(key);
            }
            arguments
        },
        Redaction::None,
    )
    .await;
    assert_eq!(plain["isError"], false, "{plain}");
    let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("crates/bridge-tally-protocol/tests/fixtures/agent");
    let detail_bytes: u64 = record["requests"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|request| {
            is_detail_read(request)
                && request["method"] == "POST"
                && DETAIL_READS.contains(&request["request_id"].as_str().unwrap_or_default())
        })
        // Each as the simulator serves it: the capture's own bytes.
        .map(|request| {
            std::fs::metadata(directory.join(request["response_fixture"].as_str().unwrap()))
                .unwrap()
                .len()
        })
        .sum();
    let bytes = |value: &Value| {
        value["structuredContent"]["evidence"]["bytes"]
            .as_u64()
            .unwrap()
    };
    assert_eq!(bytes(response), bytes(&plain) + detail_bytes);
}

const RECORDED_UNADJUSTED: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-sequence-unadjusted.json"
);
const RECORDED_BILL_TRAIL: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-detail-sequence-bill-trail.json"
);

/// Under `mask_parties` the recorded calls send the same requests, and the
/// party is masked wherever the detail names it: the detail's own `party`
/// and every bill's. The name appears nowhere in the tool's response.
#[tokio::test]
async fn mcp_outstandings_mask_the_party_throughout_a_recorded_detail() {
    for record in [RECORDED_UNADJUSTED, RECORDED_BILL_TRAIL] {
        let (response, observed, record) = replay_recorded_call(
            record,
            |_| true,
            |arguments| arguments,
            Redaction::MaskParties,
        )
        .await;
        assert_eq!(response["isError"], false, "{response}");
        assert_eq!(observed.len(), record["requests"].as_array().unwrap().len());
        let party = record["arguments"]["party"].as_str().unwrap();
        let detail = &response["structuredContent"]["result"]["detail"];
        let masked = json!(mask(party));
        assert_eq!(detail["party"], masked, "{detail}");
        for bill in detail["bills"].as_array().into_iter().flatten() {
            assert_eq!(bill["party"], masked, "{bill}");
        }
        assert!(!response.to_string().contains(party), "{party} unmasked");
        // Masking changes names only: the figures are the live call's.
        let live = &record["answer_detail"];
        for key in ["state", "window", "rows", "residual", "on_account_sum"] {
            assert_eq!(detail.get(key), live.get(key), "{key}");
        }
    }
}

/// `direction`, `top`, `offset` and `limit` page and filter the book-wide
/// figures only: with all four set, the recorded call sends the same requests
/// and its `detail` is still the live call's whole answer.
#[tokio::test]
async fn mcp_outstandings_give_the_whole_detail_whatever_the_paging_arguments() {
    let (response, observed, record) = replay_recorded_call(
        RECORDED_UNADJUSTED,
        |_| true,
        |mut arguments| {
            for (key, value) in [
                ("direction", json!("payable")),
                ("top", json!(1)),
                ("offset", json!(1)),
                ("limit", json!(1)),
            ] {
                arguments[key] = value;
            }
            arguments
        },
        Redaction::None,
    )
    .await;
    assert_replay_matches_the_live_call(&response, &observed, &record);
    let result = &response["structuredContent"]["result"];
    // The book-wide figures were paged as asked.
    assert_eq!(result["offset"], 1);
    assert_eq!(result["limit"], 1);
    assert!(
        result["open_bills"].as_array().unwrap().len() <= 1,
        "{result}"
    );
    assert!(
        result["top_parties"].as_array().unwrap().len() <= 1,
        "{result}"
    );
}

/// The two sequence records hold the expected answers and request hashes the
/// replays assert against. The fixture-provenance gate reads a JSON record
/// carrying `source` as documentation and never hashes the record itself, so
/// their bytes are pinned here: an edited expected answer fails this test
/// rather than passing the replay it was edited to match. These are the bytes
/// committed with the capture in 2d415cd.
#[test]
fn the_recorded_sequences_are_the_ones_committed_with_the_capture() {
    for (record, bytes, sha256) in [
        (
            RECORDED_UNADJUSTED,
            27_538,
            "9695b8107905bb4483ef8c82ad0ba9927ac52fb3ddc830d1b3897de806a16817",
        ),
        (
            RECORDED_BILL_TRAIL,
            27_780,
            "07a1512d5af5ca7457a0d6664906ef2fedf20d1501f764c5912e635f3e91c610",
        ),
    ] {
        assert_eq!(record.len(), bytes);
        assert_eq!(sha256_hex(record.as_bytes()), sha256);
    }
}

/// bridge#1091: a Bills report row Bridge cannot read refuses the read with its cause, the report
/// and the row, and a next step, and never names the bill or its party.
#[tokio::test]
async fn an_unreadable_bills_row_refuses_with_its_cause_report_and_row() {
    fn decode(bytes: &[u8]) -> String {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|unit| u16::from_le_bytes([unit[0], unit[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
    let captured = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-receivable.utf16le.xml"
    ));
    // The fourth bill's due date, written with a four-digit year below the one form Tally was seen to
    // print in full.
    let damaged = captured.replacen(
        "<BILLDUE>2-Jul-26</BILLDUE>",
        "<BILLDUE>2-Jul-2026</BILLDUE>",
        1,
    );
    assert_ne!(damaged, captured);
    let response = ageing_outstandings_with(
        Some(damaged),
        ageing_ledgers_with_currency("Ageing Customer A"),
        json!({}),
    )
    .await;
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "native_outstandings_read_failed");
    assert_eq!(error["cause"], "native_date_year_invalid");
    assert_eq!(error["bill_row"], json!({"report": "receivable", "row": 4}));
    let remediation = error["remediation"].as_str().expect("a next step");
    assert!(
        remediation.contains("Do not retry") && remediation.contains("bill_row"),
        "{remediation}"
    );
    // Nothing of the bill: not its party, its reference or its dates, in the structured result
    // or in the text copy of it.
    let text = response.to_string();
    for secret in [
        "Ageing Customer A",
        "MP-A",
        "CANARY",
        "CREDIT-30",
        "BD-DIFF",
        "2-Jul",
    ] {
        assert!(
            !text.contains(secret),
            "the refusal carries {secret:?}: {text}"
        );
    }
}

/// The next step for a date the Bills report could not read opens with words that fit every such
/// cause, including the two about the book's date window, which name no row (#1096).
#[test]
fn the_date_remediation_opens_with_words_that_fit_a_book_window_cause() {
    for cause in [
        "native_date_book_window_invalid",
        "native_date_year_ambiguous_book_window",
        "native_date_year_invalid",
    ] {
        let text = outstandings_cause_remediation(cause).expect("a date cause has a next step");
        assert_eq!(
            text.split(". ").next(),
            Some(
                "A date in Tally's Bills Receivable or Payable report is not one ComplyEaze Bridge \
                 can read, so no figures were returned"
            ),
            "{cause}"
        );
    }
}

/// An unreadable bill amount can now name its row (#1096), so its next step says
/// what a `bill_row` means, as a sentence of its own (#1128 review).
#[test]
fn the_amount_remediation_says_what_a_bill_row_names() {
    let text = outstandings_cause_remediation("native_amount_invalid")
        .expect("an amount cause has a next step");
    assert!(
        text.split(". ").any(|sentence| sentence
            == "When the refusal carries a `bill_row`, it names the report and the row in the \
                order Tally sent them, which may not be the order on screen"),
        "{text}"
    );
}

/// The Bills-report next steps belong to the outstandings tool only (bridge#1091): the same cause
/// codes reach other tools through reads that never open that report.
#[test]
fn the_bills_remediation_is_chosen_only_under_the_outstandings_code() {
    for cause in [
        "native_date_year_invalid",
        "bills_xml_malformed",
        "native_amount_invalid",
        "native_arithmetic_overflow",
        "native_tally_reported_failure",
    ] {
        assert!(outstandings_cause_remediation(cause).is_some(), "{cause}");
        // Neither the cause nor an operation code of another tool picks it up.
        assert_eq!(refusal_remediation(cause), None, "{cause}");
    }
    assert_eq!(refusal_remediation("trial_balance_read_failed"), None);
    // A cause that cannot come from the Bills report has no Bills advice.
    assert_eq!(outstandings_cause_remediation("native_status_absent"), None);
    // The choice is made on the code: the same cause under another tool's code gets none.
    assert!(remediation_for(
        "native_outstandings_read_failed",
        Some("native_date_year_invalid")
    )
    .is_some());
    assert_eq!(
        remediation_for("trial_balance_read_failed", Some("native_status_absent")),
        None
    );
    assert_eq!(
        remediation_for(
            "party_ledger_master_read_failed",
            Some("native_amount_invalid")
        ),
        None
    );
}

/// RD1 (2 Oct): no field said how many open bills there were, so a page cut
/// at `limit` read like the whole list while its totals covered every bill.
/// `open_bills_total` counts the bills in the requested direction before any
/// paging; `open_bills_shown` counts the page actually returned.
#[tokio::test]
async fn mcp_outstandings_count_every_open_bill_and_the_bills_shown() {
    let call = |arguments: Value| async move {
        let response = ageing_outstandings(ageing_ledgers_with_currency("I₹"), arguments).await;
        assert_eq!(response["isError"], false, "{response}");
        let result = response["structuredContent"]["result"].clone();
        assert_eq!(result["state"], "complete", "{result}");
        result
    };
    let length = |result: &Value| result["open_bills"].as_array().unwrap().len() as u64;

    // Every bill on one page: shown equals total, and nothing follows.
    let whole = call(json!({})).await;
    let total = whole["open_bills_total"].as_u64().unwrap();
    assert_eq!(total, length(&whole));
    assert_eq!(whole["open_bills_shown"], total);
    assert!(whole["next_offset"].is_null(), "{whole}");
    assert!(total > 2, "the captured book must page: {total}");

    // A cut page keeps the total and counts what it shows.
    let first = call(json!({"limit": 2})).await;
    assert_eq!(first["open_bills_total"], total);
    assert_eq!(first["open_bills_shown"], 2);
    assert_eq!(length(&first), 2);
    assert_eq!(first["next_offset"], 2);

    // The last page is shorter than `limit`: shown is the page, not `limit`.
    let last = call(json!({"offset": total - 1, "limit": 2})).await;
    assert_eq!(last["open_bills_total"], total);
    assert_eq!(last["open_bills_shown"], 1);
    assert_eq!(last["limit"], 2);

    // An offset past the end shows nothing and keeps the total.
    let beyond = call(json!({"offset": total, "limit": 2})).await;
    assert_eq!(beyond["open_bills_total"], total);
    assert_eq!(beyond["open_bills_shown"], 0);
    assert!(beyond["next_offset"].is_null(), "{beyond}");

    // The total is counted after the direction filter, as the totals are.
    let receivable = call(json!({"direction": "receivable", "limit": 1})).await;
    let receivable_bills = whole["open_bills"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|bill| bill["kind"] == "receivable")
        .count() as u64;
    assert!(receivable_bills > 1 && receivable_bills < total, "{whole}");
    assert_eq!(receivable["open_bills_total"], receivable_bills);
    assert_eq!(receivable["open_bills_shown"], 1);
}

#[test]
fn the_outstandings_description_says_how_to_read_a_shortened_bill_list() {
    let definitions = tool_definitions(true, false);
    let description = definitions
        .as_array()
        .and_then(|tools| tools.iter().find(|tool| tool["name"] == "outstandings"))
        .expect("outstandings tool definition")["description"]
        .as_str()
        .expect("tool description");
    for needle in [
        "`open_bills_total` counts every open bill in the requested direction",
        "`open_bills_shown` counts the bills on this page",
        "`limit` is not lowered when the response size shortens the page",
        "\"showing 500 of 1,240 open bills; the totals and the ageing cover all 1,240\"",
        "(on a later page, the bills from offset + 1;",
        "on a partial read, both counts and the sentence cover the base-currency ledgers only, so say so in it)",
        "`offset` set to `next_offset`",
    ] {
        assert!(description.contains(needle), "{needle}");
    }
}

/// On a partial read the counts sit under `base_currency_ledgers` and page
/// as the complete read's do: an offset past the end shows no bill and keeps
/// the base-currency total.
#[tokio::test]
async fn a_partial_read_past_the_end_shows_no_bill_and_keeps_the_total() {
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
        json!({"offset": 14, "limit": 5}),
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "partial", "{result}");
    let base = &result["base_currency_ledgers"];
    assert_eq!(base["open_bills_total"], 14);
    assert_eq!(base["open_bills_shown"], 0);
    assert!(base["open_bills"].as_array().unwrap().is_empty(), "{base}");
    assert!(base["next_offset"].is_null(), "{base}");
}
