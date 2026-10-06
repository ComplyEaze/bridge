//! Voucher summaries over captured rows (#1230). The base rows are the captured three-voucher
//! window (`native-three-vouchers`) parsed by the production parser: three Sales vouchers, each a
//! party debit against one `WR2 Sales` credit. Variants change dates, flags or amounts of those
//! rows and are derived rows, not live captures.
use super::*;

const CAPTURED_COMPANY_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";

fn captured_rows() -> Vec<Value> {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    parse_agent_rows(&xml, CAPTURED_COMPANY_GUID).unwrap()
}

fn request(group: SummaryGroup, selected_ledger: Option<&str>) -> SummaryRequest {
    SummaryRequest {
        group,
        selected_ledger: selected_ledger.map(str::to_string),
        placements: None,
    }
}

fn summed(rows: &[Value], group: SummaryGroup, ledger: Option<&str>) -> Summary {
    summarise(rows, &request(group, ledger)).expect("the rows summarise")
}

fn cents(text: &str) -> i64 {
    let negative = text.starts_with('-');
    let digits = text.trim_start_matches('-');
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    let fraction = format!("{fraction:0<2}");
    let value = whole.parse::<i64>().unwrap() * 100 + fraction[..2].parse::<i64>().unwrap();
    if negative {
        -value
    } else {
        value
    }
}

#[test]
fn a_month_summary_adds_every_entry_of_the_vouchers_in_it() {
    let summary = summed(&captured_rows(), SummaryGroup::Month, None);
    assert_eq!(summary.buckets.len(), 1);
    let bucket = &summary.buckets[0];
    assert_eq!(bucket["group"], "2026-08");
    assert_eq!(bucket["vouchers"], 3);
    assert_eq!(bucket["debit"], "-306.06");
    assert_eq!(bucket["credit"], "306.06");
    assert_eq!(bucket["net"], "0"); // a balanced set nets to zero
    assert_eq!(bucket["voucher_refs_complete"], true);
    assert_eq!(bucket["voucher_refs"].as_array().unwrap().len(), 3);
    assert_eq!(summary.vouchers_summarised, 3);
    assert_eq!(summary.entries_counted, "all_entries");
    assert_eq!(
        summary.totals,
        json!({"debit": "-306.06", "credit": "306.06"})
    );
}

#[test]
fn a_type_summary_groups_by_the_voucher_type_name() {
    let mut rows = captured_rows();
    rows[1]["voucher_type"] = json!("Credit Note");
    let summary = summed(&rows, SummaryGroup::VoucherType, None);
    let groups = summary
        .buckets
        .iter()
        .map(|bucket| {
            (
                bucket["group"].as_str().unwrap(),
                bucket["vouchers"].as_u64().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    // The larger movement first: two Sales vouchers (204.04 of credit) before one Credit Note.
    assert_eq!(groups, [("Sales", 2), ("Credit Note", 1)]);
}

#[test]
fn a_ledger_summary_has_one_bucket_per_ledger_with_the_larger_movement_first() {
    let summary = summed(&captured_rows(), SummaryGroup::Ledger, None);
    let names = summary
        .buckets
        .iter()
        .map(|bucket| bucket["group"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            party_name_value("WR2 Sales".to_string()),
            party_name_value("WR2 XML Café Naïve Ledger 01A01A2F".to_string()),
            party_name_value("Café Naïve Traders".to_string()),
            party_name_value("नमस्ते ट्रेडर्स".to_string()),
        ]
    );
    let sales = &summary.buckets[0];
    assert_eq!(sales["vouchers"], 3);
    assert_eq!(sales["credit"], "306.06");
    assert_eq!(sales["debit"], "0");
    let first_party = &summary.buckets[3];
    assert_eq!(first_party["debit"], "-101.01");
    assert_eq!(first_party["net"], "-101.01");
    // The ledger window total is the sum of its buckets, on both sides.
    assert_eq!(
        summary.totals,
        json!({"debit": "-306.06", "credit": "306.06"})
    );
}

/// Buckets of equal movement are ordered by where they first appeared in the window, never by
/// name (V14's review of #1250, P3 3): under `mask_parties` an order by name would show the
/// alphabetical order of the real names in the order of tied rows.
#[test]
fn buckets_of_equal_movement_keep_the_windows_order_not_the_alphabetical_one() {
    let entry = |ledger: &str, amount: &str, deemed_positive: &str| json!({"ledger": ledger, "amount": amount, "is_deemed_positive": deemed_positive});
    let voucher = |number: &str, debit_ledger: &str, credit_ledger: &str| {
        json!({
            "cancelled": false, "optional": false, "post_dated": false,
            "date": "20260801", "voucher_type": "Journal", "voucher_number": number,
            "guid": format!("guid-{number}"),
            "amounts": [entry(debit_ledger, "-10.00", "Yes"), entry(credit_ledger, "10.00", "No")],
        })
    };
    // Four ledgers, every one moving 10.00. They appear Zulu, Mike, Alpha, Bravo: not alphabetical.
    let rows = vec![voucher("1", "Zulu", "Mike"), voucher("2", "Alpha", "Bravo")];
    let summary = summed(&rows, SummaryGroup::Ledger, None);
    let names = summary
        .buckets
        .iter()
        .map(|bucket| bucket["group"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            party_name_value("Zulu".to_string()),
            party_name_value("Mike".to_string()),
            party_name_value("Alpha".to_string()),
            party_name_value("Bravo".to_string()),
        ]
    );
    let positions = summary
        .buckets
        .iter()
        .map(|bucket| bucket["position"].as_u64().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(positions, [1, 2, 3, 4]);
}

#[test]
fn a_ledger_name_is_a_party_marked_value_so_masking_reaches_it() {
    let summary = summed(&captured_rows(), SummaryGroup::Ledger, None);
    let masked = redact_value(summary.buckets[3].clone(), Redaction::MaskParties);
    assert_ne!(masked["group"], "नमस्ते ट्रेडर्स");
    let plain = redact_value(summary.buckets[3].clone(), Redaction::None);
    assert_eq!(plain["group"], "नमस्ते ट्रेडर्स");
}

#[test]
fn months_come_in_calendar_order_and_a_boundary_day_stays_in_its_own_month() {
    let mut rows = captured_rows();
    rows[0]["date"] = json!("20260731");
    rows[1]["date"] = json!("20260801");
    rows[2]["date"] = json!("20260901");
    let summary = summed(&rows, SummaryGroup::Month, None);
    let months = summary
        .buckets
        .iter()
        .map(|bucket| bucket["group"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(months, ["2026-07", "2026-08", "2026-09"]);
    assert_eq!(summary.buckets[0]["credit"], "101.01");
    assert_eq!(summary.buckets[1]["credit"], "102.02");
    assert_eq!(summary.buckets[2]["credit"], "103.03");
}

#[test]
fn a_narrowed_window_adds_only_the_selected_ledgers_entries_by_month_and_type() {
    let rows = captured_rows();
    for group in [SummaryGroup::Month, SummaryGroup::VoucherType] {
        let summary = summed(&rows, group, Some("WR2 Sales"));
        assert_eq!(summary.entries_counted, "selected_ledger");
        let bucket = &summary.buckets[0];
        assert_eq!(bucket["vouchers"], 3);
        assert_eq!(bucket["debit"], "0");
        assert_eq!(bucket["credit"], "306.06");
        assert_eq!(bucket["net"], "306.06");
    }
    // By ledger, every ledger of the narrowed vouchers keeps its bucket.
    let by_ledger = summed(&rows, SummaryGroup::Ledger, Some("WR2 Sales"));
    assert_eq!(by_ledger.entries_counted, "all_entries");
    assert_eq!(by_ledger.buckets.len(), 4);
}

#[test]
fn a_narrowing_ledger_the_voucher_lacks_leaves_it_in_no_bucket() {
    let summary = summed(
        &captured_rows(),
        SummaryGroup::Month,
        Some("No Such Ledger"),
    );
    assert!(summary.buckets.is_empty());
    assert_eq!(summary.totals, json!({"debit": "0", "credit": "0"}));
}

#[test]
fn cancelled_optional_and_entryless_vouchers_are_left_out_and_counted() {
    let mut rows = captured_rows();
    rows[0]["cancelled"] = json!(true);
    rows[1]["optional"] = json!(true);
    let mut stock_journal = rows[2].clone();
    stock_journal["amounts"] = json!([]);
    rows.push(stock_journal);
    let summary = summed(&rows, SummaryGroup::Month, None);
    assert_eq!(summary.vouchers_summarised, 1);
    assert_eq!(
        summary.excluded,
        json!({"cancelled": 1, "optional": 1, "no_accounting_entries": 1})
    );
    assert_eq!(summary.buckets[0]["vouchers"], 1);
    assert_eq!(summary.buckets[0]["credit"], "103.03");
}

#[test]
fn an_unbalanced_voucher_refuses_the_whole_summary_under_every_grouping() {
    let mut rows = captured_rows();
    rows[1]["amounts"][1]["amount"] = json!("102.03");
    for group in [
        SummaryGroup::Ledger,
        SummaryGroup::Month,
        SummaryGroup::VoucherType,
    ] {
        assert_eq!(
            summarise(&rows, &request(group, None)).err().as_deref(),
            Some("voucher_entries_unbalanced")
        );
    }
}

#[test]
fn an_unbalanced_voucher_that_is_cancelled_is_left_out_not_refused() {
    let mut rows = captured_rows();
    rows[1]["amounts"][1]["amount"] = json!("102.03");
    rows[1]["cancelled"] = json!(true);
    let summary = summed(&rows, SummaryGroup::Month, None);
    assert_eq!(summary.vouchers_summarised, 2);
}

#[test]
fn a_voucher_counts_once_in_a_bucket_however_many_of_its_entries_land_there() {
    let mut rows = captured_rows();
    let entries = rows[0]["amounts"].as_array().unwrap().clone();
    rows[0]["amounts"].as_array_mut().unwrap().extend(entries);
    let summary = summed(&rows, SummaryGroup::Ledger, None);
    let sales = summary
        .buckets
        .iter()
        .find(|bucket| bucket["group"] == party_name_value("WR2 Sales".to_string()))
        .unwrap();
    assert_eq!(sales["vouchers"], 3);
    assert_eq!(sales["credit"], "407.07");
}

#[test]
fn a_bucket_names_a_bounded_sample_and_says_whether_it_is_complete() {
    let template = captured_rows().remove(0);
    let rows = (0..MAX_VOUCHER_REFS_PER_BUCKET + 2)
        .map(|_| template.clone())
        .collect::<Vec<_>>();
    let bucket = &summed(&rows, SummaryGroup::Month, None).buckets[0];
    assert_eq!(bucket["vouchers"], MAX_VOUCHER_REFS_PER_BUCKET + 2);
    assert_eq!(
        bucket["voucher_refs"].as_array().unwrap().len(),
        MAX_VOUCHER_REFS_PER_BUCKET
    );
    assert_eq!(bucket["voucher_refs_complete"], false);
}

#[test]
fn a_debit_is_read_from_the_amounts_sign_not_from_the_flag() {
    let mut rows = captured_rows();
    // The flag disagrees with the sign on the second voucher's party entry.
    rows[1]["amounts"][0]["is_deemed_positive"] = json!("No");
    let summary = summed(&rows, SummaryGroup::Ledger, None);
    let party = summary
        .buckets
        .iter()
        .find(|bucket| bucket["group"] == party_name_value("Café Naïve Traders".to_string()))
        .unwrap();
    assert_eq!(party["debit"], "-102.02");
    assert_eq!(party["credit"], "0");
}

#[test]
fn bucket_totals_equal_an_independent_sum_of_the_listed_entries() {
    // The same sums rebuilt in integer cents from the rows' own entries, not through the
    // summary's decimal arithmetic.
    let mut rows = captured_rows();
    rows[0]["date"] = json!("20260715");
    rows[2]["voucher_type"] = json!("Credit Note");
    for group in [
        SummaryGroup::Ledger,
        SummaryGroup::Month,
        SummaryGroup::VoucherType,
    ] {
        let summary = summed(&rows, group, None);
        let (mut debit, mut credit) = (0i64, 0i64);
        for bucket in &summary.buckets {
            debit += cents(bucket["debit"].as_str().unwrap());
            credit += cents(bucket["credit"].as_str().unwrap());
        }
        let (mut want_debit, mut want_credit) = (0i64, 0i64);
        for row in &rows {
            for entry in row["amounts"].as_array().unwrap() {
                let amount = cents(entry["amount"].as_str().unwrap());
                if amount < 0 {
                    want_debit += amount;
                } else {
                    want_credit += amount;
                }
            }
        }
        assert_eq!((debit, credit), (want_debit, want_credit), "{group:?}");
        assert_eq!(
            summary.totals,
            json!({"debit": format!("{:.2}", want_debit as f64 / 100.0), "credit": format!("{:.2}", want_credit as f64 / 100.0)}),
            "{group:?}"
        );
    }
}

#[test]
fn a_page_of_buckets_respects_offset_limit_and_the_byte_budget() {
    let summary = summed(&captured_rows(), SummaryGroup::Ledger, None);
    let (page, truncated) = page_buckets(&summary, 0, 2, usize::MAX);
    assert_eq!(page.len(), 2);
    assert!(truncated);
    let (page, truncated) = page_buckets(&summary, 2, 10, usize::MAX);
    assert_eq!(page.len(), 2);
    assert!(!truncated);
    // A budget smaller than one bucket still advances by one, so paging cannot stall.
    let (page, truncated) = page_buckets(&summary, 0, 10, 1);
    assert_eq!(page.len(), 1);
    assert!(truncated);
    let (page, truncated) = page_buckets(&summary, 4, 10, usize::MAX);
    assert!(page.is_empty());
    assert!(!truncated);
}

#[test]
fn only_the_five_named_groupings_are_accepted() {
    assert_eq!(SummaryGroup::from_args(&json!({})).unwrap(), None);
    for (name, group) in [
        ("ledger", SummaryGroup::Ledger),
        ("month", SummaryGroup::Month),
        ("voucher_type", SummaryGroup::VoucherType),
        ("group", SummaryGroup::Group),
        ("primary_group", SummaryGroup::PrimaryGroup),
    ] {
        assert_eq!(
            SummaryGroup::from_args(&json!({"summarise_by": name})).unwrap(),
            Some(group)
        );
    }
    for bad in ["groups", "Group", "Month", "", "type", "primary"] {
        assert_eq!(
            SummaryGroup::from_args(&json!({"summarise_by": bad}))
                .expect_err("refused")
                .code,
            "summarise_by_invalid"
        );
    }
}

#[test]
fn a_negative_zero_amount_adds_nothing_and_does_not_refuse_the_summary() {
    let mut rows = captured_rows();
    // `-0.00` parses as an amount; its magnitude keeps the sign, which once built `--0.00`.
    rows[0]["amounts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"ledger": "WR2 Sales", "amount": "-0.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    let summary = summed(&rows, SummaryGroup::Month, None);
    assert_eq!(
        summary.totals,
        json!({"debit": "-306.06", "credit": "306.06"})
    );
    assert_eq!(summary.buckets[0]["debit"], "-306.06");
}

#[test]
fn post_dated_vouchers_are_summed_and_counted() {
    let mut rows = captured_rows();
    rows[0]["post_dated"] = json!(true);
    rows[2]["post_dated"] = json!(true);
    rows[1]["post_dated"] = json!(false);
    let summary = summed(&rows, SummaryGroup::Month, None);
    assert_eq!(summary.vouchers_summarised, 3);
    assert_eq!(summary.post_dated_included, 2);
    assert_eq!(summary.post_dated_flag_absent, 0);
    assert_eq!(summary.buckets[0]["credit"], "306.06");
    // The captured window predates the fetch list that asks for ISPOSTDATED, so it carries none.
    // Such vouchers are counted apart, never folded into a zero that reads as "none".
    let plain = summed(&captured_rows(), SummaryGroup::Month, None);
    assert_eq!(plain.post_dated_included, 0);
    assert_eq!(plain.post_dated_flag_absent, 3);
}

#[test]
fn every_bucket_carries_its_position_in_the_whole_ordering() {
    let summary = summed(&captured_rows(), SummaryGroup::Ledger, None);
    let positions = summary
        .buckets
        .iter()
        .map(|bucket| bucket["position"].as_u64().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(positions, [1, 2, 3, 4]);
    // A page keeps the whole-ordering position, not its own count.
    let (page, _) = page_buckets(&summary, 2, 10, usize::MAX);
    assert_eq!(page[0]["position"], 3);
}

// ---- The live rows (#1230). `vouchers-shape-lab-fy.rows.json` holds the 67 vouchers of a synthetic
// book as the tool returned them on 6 Oct 2026 (a debug build of master at 4c30f3f9f), and
// `vouchers-shape-lab-fy.live-answers.json` what the same build answered for each summary and search over them and what Tally's own
// `trial_balance` reported for the same year (see the PROVENANCE note beside them). These tests run
// the production summary and search code over those rows and require the answers the live run gave.
const LIVE_ROWS: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.rows.json"
);
const LIVE_ANSWERS: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.live-answers.json"
);

fn live_rows() -> Vec<Value> {
    serde_json::from_str(LIVE_ROWS).expect("the live rows parse")
}

fn live_answers() -> Value {
    serde_json::from_str(LIVE_ANSWERS).expect("the live answers parse")
}

/// The buckets as the tool presents them: `vouchers` passes each through `redact_value` (default settings
/// here), which turns the in-memory party-name marker of a ledger bucket into the name.
fn presented(summary: &Summary) -> Vec<Value> {
    summary
        .buckets
        .iter()
        .map(|bucket| redact_value(bucket.clone(), Redaction::None))
        .collect()
}

fn assert_equals_live(summary: &Summary, golden: &Value, what: &str) {
    assert_eq!(
        Value::Array(presented(summary)),
        golden["buckets"],
        "{what}: buckets"
    );
    assert_eq!(
        json!(summary.buckets.len()),
        golden["total"],
        "{what}: bucket count"
    );
    assert_eq!(summary.totals, golden["totals"], "{what}: totals");
    assert_eq!(
        json!(summary.vouchers_summarised),
        golden["vouchers_summarised"],
        "{what}: vouchers summarised"
    );
    assert_eq!(
        summary.excluded, golden["excluded_from_buckets"],
        "{what}: excluded"
    );
    assert_eq!(
        json!(summary.post_dated_included),
        golden["post_dated_included"],
        "{what}: post-dated"
    );
    assert_eq!(
        json!(summary.post_dated_flag_absent),
        golden["post_dated_flag_absent"],
        "{what}: flag absent"
    );
    assert_eq!(
        json!(summary.entries_counted),
        golden["entries_counted"],
        "{what}: entries counted"
    );
}

fn decimal(text: &str) -> bridge_tally_core::ExactDecimal {
    bridge_tally_core::ExactDecimal::parse(text.to_string()).expect("a plain decimal")
}

#[test]
fn the_live_rows_summarise_to_the_answers_the_tool_gave_live() {
    let (rows, answers) = (live_rows(), live_answers());
    assert_eq!(rows.len(), 67);
    for (group, name) in [
        (SummaryGroup::Ledger, "ledger"),
        (SummaryGroup::Month, "month"),
        (SummaryGroup::VoucherType, "voucher_type"),
    ] {
        assert_eq!(answers["summaries"][name]["summarised_by"], name);
        assert_equals_live(
            &summed(&rows, group, None),
            &answers["summaries"][name],
            name,
        );
    }
}

#[test]
fn the_live_ledger_selected_and_searched_summaries_are_reproduced() {
    let answers = live_answers();
    let selected = &answers["summary_with_ledger_selected"];
    let ledger = selected["ledger"].as_str().unwrap();
    let kept = filter_voucher_rows_for_ledger(live_rows(), ledger);
    assert_equals_live(
        &summed(&kept, SummaryGroup::Month, Some(ledger)),
        selected,
        "ledger selected",
    );
    let searched = &answers["summary_of_amount_search"];
    let found = VoucherSearch::from_args(&searched["args"], Redaction::None)
        .expect("the live search arguments are accepted")
        .expect("a criterion was given")
        .apply(live_rows());
    assert_equals_live(
        &summed(&found, SummaryGroup::Ledger, None),
        searched,
        "amount search, by ledger",
    );
}

#[test]
fn the_live_ledger_buckets_equal_the_trial_balance_of_the_same_year() {
    let answers = live_answers();
    let summary = summed(&live_rows(), SummaryGroup::Ledger, None);
    let amount = |value: &Value| match value["state"].as_str() {
        Some("present") => value["value"].as_str().unwrap().to_string(),
        Some("present_empty") => "0".to_string(),
        other => panic!("a trial balance amount in state {other:?}"),
    };
    let rows = answers["trial_balance_ledgers"].as_array().unwrap();
    let mut tied = 0;
    let buckets = presented(&summary);
    for bucket in &buckets {
        let name = bucket["group"].as_str().unwrap();
        let row = rows
            .iter()
            .find(|row| row["ledger"] == name)
            .unwrap_or_else(|| panic!("the trial balance has no row for {name}"));
        assert!(
            decimal(bucket["debit"].as_str().unwrap()).numeric_eq(&decimal(&amount(&row["debit"]))),
            "{name} debit"
        );
        assert!(
            decimal(bucket["credit"].as_str().unwrap())
                .numeric_eq(&decimal(&amount(&row["credit"]))),
            "{name} credit"
        );
        tied += 1;
    }
    assert_eq!(tied, 30);
    // A ledger the trial balance shows movement for, and the summary has no bucket for, would be a miss.
    let bucketed: BTreeSet<&str> = buckets
        .iter()
        .map(|b| b["group"].as_str().unwrap())
        .collect();
    for row in rows
        .iter()
        .filter(|row| !bucketed.contains(row["ledger"].as_str().unwrap()))
    {
        assert!(
            decimal(&amount(&row["debit"])).is_zero() && decimal(&amount(&row["credit"])).is_zero(),
            "{}",
            row["ledger"]
        );
    }
}

/// The buckets of a grouping summed straight from the rows: each posting voucher's entries by `key`,
/// a negative amount a debit and a positive one a credit, a voucher counted once per bucket. It uses
/// `ExactDecimal` only, not the summary code's own helpers, so it does not share the code under test.
fn independent_buckets(
    rows: &[Value],
    key: impl Fn(&Value, &Value) -> String,
) -> BTreeMap<
    String,
    (
        bridge_tally_core::ExactDecimal,
        bridge_tally_core::ExactDecimal,
        usize,
    ),
> {
    let zero = bridge_tally_core::ExactDecimal::zero;
    let mut out: BTreeMap<
        String,
        (
            bridge_tally_core::ExactDecimal,
            bridge_tally_core::ExactDecimal,
            usize,
        ),
    > = BTreeMap::new();
    for row in rows.iter().filter(|row| {
        row["cancelled"] != true
            && row["optional"] != true
            && !row["amounts"].as_array().unwrap().is_empty()
    }) {
        let mut touched = BTreeSet::new();
        for entry in row["amounts"].as_array().unwrap() {
            let name = key(row, entry);
            let amount = decimal(entry["amount"].as_str().unwrap());
            let bucket = out
                .entry(name.clone())
                .or_insert_with(|| (zero(), zero(), 0));
            if entry["amount"].as_str().unwrap().starts_with('-') {
                bucket.0 = bucket.0.checked_add(&amount).unwrap();
            } else {
                bucket.1 = bucket.1.checked_add(&amount).unwrap();
            }
            if touched.insert(name) {
                bucket.2 += 1;
            }
        }
    }
    out
}

#[test]
fn the_live_buckets_of_every_grouping_are_the_sums_of_the_listed_vouchers_and_trace_to_them() {
    let rows = live_rows();
    type Key = fn(&Value, &Value) -> String;
    let by_ledger: Key = |_, entry| entry["ledger"].as_str().unwrap().to_string();
    let by_month: Key = |row, _| {
        let date = row["date"].as_str().unwrap();
        format!("{}-{}", &date[..4], &date[4..6])
    };
    let by_type: Key = |row, _| row["voucher_type"].as_str().unwrap().to_string();
    for (group, key) in [
        (SummaryGroup::Ledger, by_ledger),
        (SummaryGroup::Month, by_month),
        (SummaryGroup::VoucherType, by_type),
    ] {
        let summary = summed(&rows, group, None);
        let buckets = presented(&summary);
        let want = independent_buckets(&rows, key);
        assert_eq!(buckets.len(), want.len(), "{group:?}: bucket count");
        let (mut total_debit, mut total_credit) = (
            bridge_tally_core::ExactDecimal::zero(),
            bridge_tally_core::ExactDecimal::zero(),
        );
        for bucket in &buckets {
            let name = bucket["group"].as_str().unwrap();
            let (debit, credit, vouchers) = want
                .get(name)
                .unwrap_or_else(|| panic!("{group:?}: no sum for {name}"));
            assert!(
                decimal(bucket["debit"].as_str().unwrap()).numeric_eq(debit),
                "{group:?} {name} debit"
            );
            assert!(
                decimal(bucket["credit"].as_str().unwrap()).numeric_eq(credit),
                "{group:?} {name} credit"
            );
            assert_eq!(bucket["vouchers"], *vouchers, "{group:?} {name} vouchers");
            total_debit = total_debit.checked_add(debit).unwrap();
            total_credit = total_credit.checked_add(credit).unwrap();
            let refs = bucket["voucher_refs"].as_array().unwrap();
            assert_eq!(
                refs.len(),
                (*vouchers).min(MAX_VOUCHER_REFS_PER_BUCKET),
                "{group:?} {name} refs"
            );
            assert_eq!(bucket["voucher_refs_complete"], refs.len() == *vouchers);
            for reference in refs {
                let row = rows
                    .iter()
                    .find(|row| row["guid"] == reference["guid"])
                    .expect("a ref names a listed voucher");
                assert!(
                    row["cancelled"] != true
                        && row["optional"] != true
                        && row["amounts"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .any(|entry| key(row, entry) == name),
                    "{group:?} {name}: {}",
                    reference["guid"]
                );
            }
        }
        assert!(
            decimal(summary.totals["debit"].as_str().unwrap()).numeric_eq(&total_debit),
            "{group:?} totals debit"
        );
        assert!(
            decimal(summary.totals["credit"].as_str().unwrap()).numeric_eq(&total_credit),
            "{group:?} totals credit"
        );
    }
}

#[test]
fn the_live_buckets_page_without_a_gap_or_a_repeat() {
    let summary = summed(&live_rows(), SummaryGroup::Ledger, None);
    assert_eq!(summary.buckets.len(), 30);
    let (mut seen, mut offset) = (Vec::new(), 0);
    loop {
        let (page, truncated) = page_buckets(&summary, offset, 7, usize::MAX);
        offset += page.len();
        seen.extend(page);
        if !truncated {
            break;
        }
    }
    assert_eq!(seen, summary.buckets);
    // A byte budget below one bucket still returns one, so a page always advances.
    let (page, truncated) = page_buckets(&summary, 0, 30, 1);
    assert_eq!((page.len(), truncated), (1, true));
}

#[test]
fn each_live_voucher_with_a_disagreeing_flag_balances_by_the_sign_of_the_amount_and_the_summary_accepts_it(
) {
    // Two Round Off entries carry a deemed-positive flag that disagrees with the sign of their amount.
    // The summary counts by sign (as `ledger_movement` does) and every live voucher balances that way;
    // counting by the flag would leave those two vouchers unbalanced.
    let rows = live_rows();
    let disagreeing: Vec<&Value> = rows
        .iter()
        .filter(|row| {
            row["amounts"]
                .as_array()
                .unwrap()
                .iter()
                .any(|e| e["polarity_disagrees_with_amount"] == true)
        })
        .collect();
    assert_eq!(disagreeing.len(), 2);
    for row in disagreeing {
        let entries = row["amounts"].as_array().unwrap();
        let (mut by_sign, mut by_flag) = ("0".to_string(), "0".to_string());
        for entry in entries {
            let amount = entry["amount"].as_str().unwrap();
            by_sign = add_decimal(&by_sign, amount).unwrap();
            let magnitude = amount.trim_start_matches('-');
            let flagged = if entry["is_deemed_positive"] == "Yes" {
                format!("-{magnitude}")
            } else {
                magnitude.to_string()
            };
            by_flag = add_decimal(&by_flag, &flagged).unwrap();
        }
        assert!(decimal(&by_sign).is_zero(), "balances by sign");
        assert!(
            !decimal(&by_flag).is_zero(),
            "would not balance by the flag"
        );
    }
    assert!(summarise(&rows, &request(SummaryGroup::Ledger, None)).is_ok());
}

#[test]
fn a_cancelled_voucher_that_keeps_its_entries_is_left_out_and_counted() {
    // Derived: the live rows with the one cancelled voucher (exported with no entries) given the entries of
    // another voucher, as a book whose export keeps them would show.
    let rows = live_rows();
    let baseline = summed(&rows, SummaryGroup::Ledger, None);
    let mut derived = rows.clone();
    let donor = rows
        .iter()
        .find(|row| {
            row["cancelled"] != true
                && row["optional"] != true
                && !row["amounts"].as_array().unwrap().is_empty()
        })
        .unwrap()["amounts"]
        .clone();
    let cancelled = derived
        .iter_mut()
        .find(|row| row["cancelled"] == true)
        .unwrap();
    cancelled["amounts"] = donor;
    let after = summed(&derived, SummaryGroup::Ledger, None);
    assert_eq!(presented(&after), presented(&baseline));
    assert_eq!(after.excluded, baseline.excluded);
    assert_eq!(after.excluded["cancelled"], 1);
    assert_eq!(after.vouchers_summarised, baseline.vouchers_summarised);
}

#[test]
fn the_live_ledger_selected_month_buckets_count_only_that_ledgers_entries_by_an_independent_sum() {
    let rows = live_rows();
    let ledger = "Shape Buyer 1";
    let kept = filter_voucher_rows_for_ledger(rows.clone(), ledger);
    let summary = summed(&kept, SummaryGroup::Month, Some(ledger));
    // The same rows with every other entry removed: a plain month sum of what is left.
    let only_that_ledger: Vec<Value> = kept
        .iter()
        .map(|row| {
            let mut row = row.clone();
            row["amounts"] = Value::Array(
                row["amounts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|e| e["ledger"] == ledger)
                    .cloned()
                    .collect(),
            );
            row
        })
        .collect();
    let want = independent_buckets(&only_that_ledger, |row, _| {
        let date = row["date"].as_str().unwrap();
        format!("{}-{}", &date[..4], &date[4..6])
    });
    let buckets = presented(&summary);
    assert_eq!(buckets.len(), want.len());
    for bucket in &buckets {
        let (debit, credit, vouchers) = &want[bucket["group"].as_str().unwrap()];
        assert!(decimal(bucket["debit"].as_str().unwrap()).numeric_eq(debit));
        assert!(decimal(bucket["credit"].as_str().unwrap()).numeric_eq(credit));
        assert_eq!(bucket["vouchers"], *vouchers);
    }
    assert_eq!(summary.entries_counted, "selected_ledger");
}

// ---- Group summaries over the live rows (#1230). The ledger catalogue and the group snapshot are the
// live book's (see `shape-lab-group-chain.PROVENANCE.md`); the expected buckets are written separately
// from the code under test (the trial balance's `parent` column and the catalogue's `PARENT` come from the same
// ledger collection, so this checks the placement logic, not Tally's grouping), from the trial balance's own `parent` column and from a hand-written table of
// which primary group each immediate group sits under (read from the group snapshot's structure).
const LIVE_TRIAL_BALANCE: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/shape-lab-fy.trial-balance-ledgers.json"
);

fn live_request(group: SummaryGroup, placements: Placements) -> SummaryRequest {
    SummaryRequest {
        group,
        selected_ledger: None,
        placements: Some(Arc::new(placements)),
    }
}

fn live_group_summary(group: SummaryGroup) -> Summary {
    let request = live_request(
        group,
        crate::agent::voucher_groups::tests::live_placements(),
    );
    summarise(&live_rows(), &request).expect("the live rows summarise by group")
}

/// The trial balance's own parent of each ledger, with the reserved root written as `Primary`.
fn trial_balance_parents() -> BTreeMap<String, String> {
    let rows: Vec<Value> = serde_json::from_str(LIVE_TRIAL_BALANCE).unwrap();
    rows.iter()
        .map(|row| {
            let parent = row["parent"].as_str().unwrap();
            let parent = parent
                .strip_prefix(bridge_tally_protocol::TALLY_SANITIZED_ROOT_MARKER)
                .map_or(parent, str::trim);
            (
                row["ledger"].as_str().unwrap().to_string(),
                parent.to_string(),
            )
        })
        .collect()
}

/// The group directly under the root that each immediate group of the live book sits under.
const PRIMARY_OF: &[(&str, &str)] = &[
    ("Bank Accounts", "Current Assets"),
    ("Bank OD A/c", "Loans (Liability)"),
    ("Capital Account", "Capital Account"),
    ("Cash-in-Hand", "Current Assets"),
    ("Chemical Suppliers", "Current Liabilities"),
    ("Direct Expenses", "Direct Expenses"),
    ("Duties & Taxes", "Current Liabilities"),
    ("Indirect Expenses", "Indirect Expenses"),
    ("Indirect Incomes", "Indirect Incomes"),
    ("Power and Fuel", "Indirect Expenses"),
    ("Primary", "Primary"),
    ("Purchase Accounts", "Purchase Accounts"),
    ("Sales Accounts", "Sales Accounts"),
    ("Secured Loans", "Loans (Liability)"),
    ("Trade Debtors - Local", "Current Assets"),
    ("Trade Debtors - Outstation", "Current Assets"),
];

fn primary_of(group: &str) -> &'static str {
    PRIMARY_OF
        .iter()
        .find(|(name, _)| *name == group)
        .unwrap_or_else(|| panic!("{group}"))
        .1
}

#[test]
fn the_live_group_buckets_equal_independent_sums_by_the_trial_balances_own_parent() {
    let parents = trial_balance_parents();
    let rows = live_rows();
    let summary = live_group_summary(SummaryGroup::Group);
    let by_parent =
        |_row: &Value, entry: &Value| parents[entry["ledger"].as_str().unwrap()].clone();
    let want = independent_buckets(&rows, by_parent);
    let buckets = presented(&summary);
    assert_eq!(buckets.len(), want.len());
    for bucket in &buckets {
        let name = bucket["group"].as_str().unwrap();
        let (debit, credit, vouchers) = want
            .get(name)
            .unwrap_or_else(|| panic!("no sum for {name}"));
        assert!(
            decimal(bucket["debit"].as_str().unwrap()).numeric_eq(debit),
            "{name} debit"
        );
        assert!(
            decimal(bucket["credit"].as_str().unwrap()).numeric_eq(credit),
            "{name} credit"
        );
        assert_eq!(bucket["vouchers"], *vouchers, "{name} vouchers");
    }
    assert_eq!(summary.vouchers_summarised, 64);
    assert_eq!(
        summary.excluded,
        json!({"cancelled": 1, "optional": 1, "no_accounting_entries": 1})
    );
}

#[test]
fn the_live_group_buckets_equal_the_trial_balance_rolled_up_by_its_own_parent() {
    // Tally's own period figures per ledger, added up by the parent Tally reports for each ledger.
    let tb: Vec<Value> = serde_json::from_str(LIVE_TRIAL_BALANCE).unwrap();
    let amount = |value: &Value| match value["state"].as_str() {
        Some("present") => decimal(value["value"].as_str().unwrap()),
        _ => bridge_tally_core::ExactDecimal::zero(),
    };
    let mut rolled: BTreeMap<
        String,
        (
            bridge_tally_core::ExactDecimal,
            bridge_tally_core::ExactDecimal,
        ),
    > = BTreeMap::new();
    for row in &tb {
        let parent = row["parent"].as_str().unwrap();
        let parent = parent
            .strip_prefix(bridge_tally_protocol::TALLY_SANITIZED_ROOT_MARKER)
            .map_or(parent, str::trim);
        let entry = rolled.entry(parent.to_string()).or_insert_with(|| {
            (
                bridge_tally_core::ExactDecimal::zero(),
                bridge_tally_core::ExactDecimal::zero(),
            )
        });
        entry.0 = entry.0.checked_add(&amount(&row["debit"])).unwrap();
        entry.1 = entry.1.checked_add(&amount(&row["credit"])).unwrap();
    }
    let buckets = presented(&live_group_summary(SummaryGroup::Group));
    for bucket in &buckets {
        let name = bucket["group"].as_str().unwrap();
        let (debit, credit) = &rolled[name];
        assert!(
            decimal(bucket["debit"].as_str().unwrap()).numeric_eq(debit),
            "{name} debit"
        );
        assert!(
            decimal(bucket["credit"].as_str().unwrap()).numeric_eq(credit),
            "{name} credit"
        );
    }
    // A group with movement in the trial balance and no bucket would be a miss.
    let bucketed: BTreeSet<&str> = buckets
        .iter()
        .map(|b| b["group"].as_str().unwrap())
        .collect();
    for (name, (debit, credit)) in &rolled {
        assert!(
            bucketed.contains(name.as_str()) || (debit.is_zero() && credit.is_zero()),
            "{name}"
        );
    }
}

#[test]
fn the_live_primary_group_buckets_add_the_group_buckets_under_each_primary_group() {
    let by_group = presented(&live_group_summary(SummaryGroup::Group));
    let primary = presented(&live_group_summary(SummaryGroup::PrimaryGroup));
    let mut want: BTreeMap<
        &str,
        (
            bridge_tally_core::ExactDecimal,
            bridge_tally_core::ExactDecimal,
        ),
    > = BTreeMap::new();
    for bucket in &by_group {
        let entry = want
            .entry(primary_of(bucket["group"].as_str().unwrap()))
            .or_insert_with(|| {
                (
                    bridge_tally_core::ExactDecimal::zero(),
                    bridge_tally_core::ExactDecimal::zero(),
                )
            });
        entry.0 = entry
            .0
            .checked_add(&decimal(bucket["debit"].as_str().unwrap()))
            .unwrap();
        entry.1 = entry
            .1
            .checked_add(&decimal(bucket["credit"].as_str().unwrap()))
            .unwrap();
    }
    assert_eq!(primary.len(), want.len());
    for bucket in &primary {
        let name = bucket["group"].as_str().unwrap();
        let (debit, credit) = &want[name];
        assert!(
            decimal(bucket["debit"].as_str().unwrap()).numeric_eq(debit),
            "{name} debit"
        );
        assert!(
            decimal(bucket["credit"].as_str().unwrap()).numeric_eq(credit),
            "{name} credit"
        );
        assert_eq!(
            bucket["reserved_name"], bucket["group"],
            "{name}: a predefined primary group keeps its name"
        );
    }
    // A user group's ledgers land in the primary group above it, and the sum of everything is unchanged.
    assert!(primary
        .iter()
        .any(|b| b["group"] == "Current Assets" && b["vouchers"].as_u64().unwrap() > 0));
    let total = live_group_summary(SummaryGroup::PrimaryGroup).totals;
    assert_eq!(
        total,
        summed(&live_rows(), SummaryGroup::Ledger, None).totals
    );
}

#[test]
fn a_group_bucket_names_its_chain_and_members_that_add_up_to_it() {
    let buckets = presented(&live_group_summary(SummaryGroup::Group));
    let debtors = buckets
        .iter()
        .find(|b| b["group"] == "Trade Debtors - Local")
        .expect("a user group has a bucket");
    assert_eq!(debtors["reserved_name"], "");
    assert_eq!(
        debtors["chain"],
        json!([
            {"name": "Trade Debtors - Local", "reserved_name": ""},
            {"name": "Sundry Debtors", "reserved_name": "Sundry Debtors"},
            {"name": "Current Assets", "reserved_name": "Current Assets"},
        ])
    );
    assert_eq!(
        debtors["primary_group"],
        json!({"name": "Current Assets", "reserved_name": "Current Assets"})
    );
    for bucket in &buckets {
        let members = bucket["members"].as_array().unwrap();
        let total = bucket["members_total"].as_u64().unwrap() as usize;
        assert_eq!(bucket["members_complete"], total <= MAX_MEMBERS_PER_BUCKET);
        assert_eq!(members.len(), total.min(MAX_MEMBERS_PER_BUCKET));
        if bucket["members_complete"] == true {
            let (mut debit, mut credit) = (
                bridge_tally_core::ExactDecimal::zero(),
                bridge_tally_core::ExactDecimal::zero(),
            );
            for member in members {
                debit = debit
                    .checked_add(&decimal(member["debit"].as_str().unwrap()))
                    .unwrap();
                credit = credit
                    .checked_add(&decimal(member["credit"].as_str().unwrap()))
                    .unwrap();
            }
            assert!(
                debit.numeric_eq(&decimal(bucket["debit"].as_str().unwrap())),
                "{}",
                bucket["group"]
            );
            assert!(
                credit.numeric_eq(&decimal(bucket["credit"].as_str().unwrap())),
                "{}",
                bucket["group"]
            );
        }
    }
}

#[test]
fn group_names_are_shown_and_member_ledgers_are_masked_under_mask_parties() {
    let summary = live_group_summary(SummaryGroup::Group);
    let shown = |redaction: Redaction| -> Vec<Value> {
        summary
            .buckets
            .iter()
            .map(|b| redact_value(b.clone(), redaction))
            .collect()
    };
    let (plain, masked) = (shown(Redaction::None), shown(Redaction::MaskParties));
    for (open, hidden) in plain.iter().zip(&masked) {
        assert_eq!(
            open["group"], hidden["group"],
            "a group name is not a party name"
        );
        assert_eq!(open["chain"], hidden["chain"]);
        for (open_member, hidden_member) in open["members"]
            .as_array()
            .unwrap()
            .iter()
            .zip(hidden["members"].as_array().unwrap())
        {
            assert_ne!(
                open_member["ledger"], hidden_member["ledger"],
                "{}",
                open_member["ledger"]
            );
            assert_eq!(open_member["debit"], hidden_member["debit"]);
        }
    }
}

#[test]
fn a_ledger_directly_under_the_root_has_a_bucket_of_its_own_in_both_group_summaries() {
    // Derived: one live voucher given an entry on `Profit & Loss A/c`, the live ledger under the reserved root.
    let mut rows = live_rows();
    let voucher = rows
        .iter_mut()
        .find(|row| {
            row["cancelled"] != true
                && row["optional"] != true
                && !row["amounts"].as_array().unwrap().is_empty()
        })
        .unwrap();
    let entries = voucher["amounts"].as_array_mut().unwrap();
    entries.push(json!({"ledger": "Profit & Loss A/c", "amount": "5.00", "is_deemed_positive": "No", "bill_allocations": []}));
    entries.push(json!({"ledger": "Cash", "amount": "-5.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    for group in [SummaryGroup::Group, SummaryGroup::PrimaryGroup] {
        let request = live_request(
            group,
            crate::agent::voucher_groups::tests::live_placements(),
        );
        let buckets = presented(&summarise(&rows, &request).unwrap());
        let root = buckets
            .iter()
            .find(|b| b["group"] == "Primary")
            .unwrap_or_else(|| panic!("{group:?}"));
        assert_eq!(root["vouchers"], 1);
        assert_eq!(root["credit"], "5");
        assert_eq!(
            root["reserved_name"],
            Value::Null,
            "the root has no reserved name: {group:?}"
        );
    }
}

#[test]
fn a_ledger_whose_chain_cannot_be_walked_refuses_the_whole_group_summary_with_the_gap() {
    let rows = live_rows();
    // Derived: a ledger the live vouchers touch, given no parent in the catalogue.
    let catalogue_parents = |changed: &str| -> Vec<(String, Option<String>)> {
        crate::agent::voucher_groups::tests::live_parents()
            .into_iter()
            .map(|(ledger, parent)| {
                (
                    ledger.clone(),
                    if ledger == changed { None } else { parent },
                )
            })
            .collect()
    };
    let build = |parents: &Vec<(String, Option<String>)>| {
        Placements::build(
            parents.iter().map(|(l, p)| (l.as_str(), p.as_deref())),
            crate::agent::voucher_groups::tests::live_groups(),
        )
    };
    let touched = catalogue_parents("Cash");
    for group in [SummaryGroup::Group, SummaryGroup::PrimaryGroup] {
        let err = summarise(&rows, &live_request(group, build(&touched)))
            .err()
            .expect("refused");
        assert_eq!(err, "summary_group_unresolved:no_parent:Cash", "{group:?}");
    }
    // A ledger the window never touches may have a gap: nothing is placed under it.
    let untouched = catalogue_parents("Drawings");
    assert!(rows.iter().all(|row| row["amounts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|e| e["ledger"] != "Drawings")));
    assert!(summarise(&rows, &live_request(SummaryGroup::Group, build(&untouched))).is_ok());
    // A ledger the vouchers name and the catalogue does not list is drift, not a bucket.
    let mut missing = live_rows();
    missing[0]["amounts"].as_array_mut().unwrap().push(json!({"ledger": "Not In The Catalogue", "amount": "0.00", "is_deemed_positive": "No", "bill_allocations": []}));
    let err = summarise(
        &missing,
        &live_request(SummaryGroup::Group, build(&touched)),
    )
    .err()
    .expect("refused");
    assert_eq!(
        err,
        "summary_group_unresolved:ledger_not_in_catalogue:Not In The Catalogue"
    );
    // And a request that asks for a group grouping with no placements at all cannot summarise.
    let none = SummaryRequest {
        group: SummaryGroup::Group,
        selected_ledger: None,
        placements: None,
    };
    assert_eq!(
        summarise(&rows, &none).err().unwrap(),
        "summary_group_unresolved:no_placements"
    );
}

#[test]
fn a_group_bucket_lists_at_most_the_bound_of_members_largest_movement_first_and_counts_them_all() {
    // Derived: one voucher with a debit on `Cash` and credits of 1 to 12 on twelve ledgers that all sit
    // under one group, so the group has more members than the bound.
    let mut parents = vec![("Cash".to_string(), Some("Cash-in-Hand".to_string()))];
    let mut entries = vec![
        json!({"ledger": "Cash", "amount": "-78.00", "is_deemed_positive": "Yes", "bill_allocations": []}),
    ];
    for n in 1..=12 {
        let ledger = format!("Member {n:02}");
        parents.push((ledger.clone(), Some("Cash-in-Hand".to_string())));
        entries.push(json!({"ledger": ledger, "amount": format!("{n}.00"), "is_deemed_positive": "No", "bill_allocations": []}));
    }
    let row = json!({"date": "20250401", "voucher_type": "Journal", "voucher_number": "1", "guid": "g-1", "cancelled": false, "optional": false, "post_dated": false, "amounts": entries});
    let placements = Placements::build(
        parents.iter().map(|(l, p)| (l.as_str(), p.as_deref())),
        crate::agent::voucher_groups::tests::live_groups(),
    );
    let summary = summarise(&[row], &live_request(SummaryGroup::Group, placements)).unwrap();
    let buckets = presented(&summary);
    assert_eq!(buckets.len(), 1);
    let bucket = &buckets[0];
    assert_eq!(bucket["group"], "Cash-in-Hand");
    assert_eq!(bucket["members_total"], 13);
    assert_eq!(bucket["members_complete"], false);
    let members = bucket["members"].as_array().unwrap();
    assert_eq!(members.len(), 10);
    // Largest movement first: Cash (78), then Member 12, 11, 10 ... down to the tenth.
    let order: Vec<&str> = members
        .iter()
        .map(|m| m["ledger"].as_str().unwrap())
        .collect();
    assert_eq!(order[..4], ["Cash", "Member 12", "Member 11", "Member 10"]);
    assert_eq!(order[9], "Member 04");
}

/// The groups at and under each of these, written from the live group snapshot's tree.
const SUBTREES: &[(&str, &[&str])] = &[
    (
        "Indirect Expenses",
        &["Indirect Expenses", "Factory Overheads", "Power and Fuel"],
    ),
    (
        "Sundry Debtors",
        &[
            "Sundry Debtors",
            "Trade Debtors - Local",
            "Trade Debtors - Outstation",
        ],
    ),
    (
        "Current Assets",
        &[
            "Current Assets",
            "Bank Accounts",
            "Cash-in-Hand",
            "Sundry Debtors",
            "Trade Debtors - Local",
            "Trade Debtors - Outstation",
        ],
    ),
    (
        "Current Liabilities",
        &[
            "Current Liabilities",
            "Duties & Taxes",
            "Sundry Creditors",
            "Chemical Suppliers",
        ],
    ),
    (
        "Loans (Liability)",
        &["Loans (Liability)", "Bank OD A/c", "Secured Loans"],
    ),
];

#[test]
fn the_live_subtree_totals_give_a_predefined_group_its_whole_figure_descendants_included() {
    // Oracle: Tally's own trial balance, ledger by ledger, added up by the trial balance's own parent over
    // the hand-written subtree sets; vouchers counted from the rows by the same parents.
    let tb: Vec<Value> = serde_json::from_str(LIVE_TRIAL_BALANCE).unwrap();
    let amount = |value: &Value| match value["state"].as_str() {
        Some("present") => decimal(value["value"].as_str().unwrap()),
        _ => bridge_tally_core::ExactDecimal::zero(),
    };
    let parents = trial_balance_parents();
    let summary = live_group_summary(SummaryGroup::Group);
    assert_eq!(
        summary.subtree_total_count,
        summary.subtree_totals.len(),
        "the live book has fewer groups than the bound"
    );
    let rows = live_rows();
    for (group, members) in SUBTREES {
        let (mut debit, mut credit) = (
            bridge_tally_core::ExactDecimal::zero(),
            bridge_tally_core::ExactDecimal::zero(),
        );
        for row in tb.iter().filter(|row| {
            let parent = row["parent"].as_str().unwrap();
            let parent = parent
                .strip_prefix(bridge_tally_protocol::TALLY_SANITIZED_ROOT_MARKER)
                .map_or(parent, str::trim);
            members.contains(&parent)
        }) {
            debit = debit.checked_add(&amount(&row["debit"])).unwrap();
            credit = credit.checked_add(&amount(&row["credit"])).unwrap();
        }
        let vouchers = rows
            .iter()
            .filter(|row| row["cancelled"] != true && row["optional"] != true)
            .filter(|row| {
                row["amounts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|e| members.contains(&parents[e["ledger"].as_str().unwrap()].as_str()))
            })
            .count();
        let total = summary
            .subtree_totals
            .iter()
            .find(|t| t["group"] == *group)
            .unwrap_or_else(|| {
                panic!("no subtree total for {group}: {:?}", summary.subtree_totals)
            });
        assert!(
            decimal(total["debit"].as_str().unwrap()).numeric_eq(&debit),
            "{group} debit"
        );
        assert!(
            decimal(total["credit"].as_str().unwrap()).numeric_eq(&credit),
            "{group} credit"
        );
        assert_eq!(total["vouchers"], vouchers, "{group} vouchers");
    }
    // A predefined group with no ledger of its own is still there, one level above its sub-groups.
    let by = |name: &str| {
        summary
            .subtree_totals
            .iter()
            .find(|t| t["group"] == name)
            .unwrap()
    };
    assert!(
        summary
            .buckets
            .iter()
            .all(|b| presented_name(b) != "Sundry Debtors"),
        "Sundry Debtors has no ledger of its own"
    );
    assert_eq!(by("Current Assets")["depth"], 1);
    assert_eq!(by("Sundry Debtors")["depth"], 2);
    assert_eq!(by("Trade Debtors - Local")["depth"], 3);
    assert_eq!(by("Sundry Debtors")["reserved_name"], "Sundry Debtors");
    assert_eq!(by("Trade Debtors - Local")["reserved_name"], "");
}

fn presented_name(bucket: &Value) -> String {
    redact_value(bucket.clone(), Redaction::None)["group"]
        .as_str()
        .unwrap()
        .to_string()
}

#[test]
fn the_subtree_totals_at_the_top_equal_the_primary_group_buckets() {
    let group = live_group_summary(SummaryGroup::Group);
    let primary = presented(&live_group_summary(SummaryGroup::PrimaryGroup));
    let tops: Vec<&Value> = group
        .subtree_totals
        .iter()
        .filter(|t| t["depth"] == 1)
        .collect();
    assert_eq!(tops.len(), primary.len());
    for top in tops {
        let bucket = primary
            .iter()
            .find(|b| b["group"] == top["group"])
            .unwrap_or_else(|| panic!("{top}"));
        assert!(
            decimal(bucket["debit"].as_str().unwrap())
                .numeric_eq(&decimal(top["debit"].as_str().unwrap())),
            "{top}"
        );
        assert!(
            decimal(bucket["credit"].as_str().unwrap())
                .numeric_eq(&decimal(top["credit"].as_str().unwrap())),
            "{top}"
        );
        assert_eq!(bucket["vouchers"], top["vouchers"], "{top}");
    }
    // The other groupings carry no subtree totals.
    assert!(live_group_summary(SummaryGroup::PrimaryGroup)
        .subtree_totals
        .is_empty());
    assert!(summed(&live_rows(), SummaryGroup::Ledger, None)
        .subtree_totals
        .is_empty());
}

#[test]
fn subtree_totals_are_bounded_largest_first_and_counted_exactly() {
    // Derived: one voucher over 70 ledgers each in its own user group directly under `Current Assets`.
    let mut groups = crate::agent::voucher_groups::tests::live_groups();
    let under_assets = bridge_tally_protocol::PartyLedgerMasterFieldObservation::Returned(
        "Current Assets".to_string(),
    );
    let mut parents = vec![("Cash".to_string(), Some("Cash-in-Hand".to_string()))];
    let mut entries = vec![
        json!({"ledger": "Cash", "amount": "-2485.00", "is_deemed_positive": "Yes", "bill_allocations": []}),
    ];
    for n in 1..=70 {
        let name = format!("Group {n:02}");
        groups.push(bridge_tally_protocol::TallyNamedMaster {
            name: name.clone(),
            parent: under_assets.clone(),
            reserved_name: Some(String::new()),
        });
        parents.push((format!("Ledger {n:02}"), Some(name)));
        entries.push(json!({"ledger": format!("Ledger {n:02}"), "amount": format!("{n}.00"), "is_deemed_positive": "No", "bill_allocations": []}));
    }
    let row = json!({"date": "20250401", "voucher_type": "Journal", "voucher_number": "1", "guid": "g-1", "cancelled": false, "optional": false, "post_dated": false, "amounts": entries});
    let placements = Placements::build(
        parents.iter().map(|(l, p)| (l.as_str(), p.as_deref())),
        groups,
    );
    let summary = summarise(&[row], &live_request(SummaryGroup::Group, placements)).unwrap();
    assert_eq!(
        summary.subtree_total_count, 72,
        "70 groups, Cash-in-Hand and Current Assets"
    );
    assert_eq!(summary.subtree_totals.len(), MAX_SUBTREE_TOTALS);
    // Largest first: Current Assets holds everything, then Cash-in-Hand (2485.00), then Group 70, 69, ...
    let order: Vec<&str> = summary
        .subtree_totals
        .iter()
        .map(|t| t["group"].as_str().unwrap())
        .collect();
    assert_eq!(
        order[..4],
        ["Current Assets", "Cash-in-Hand", "Group 70", "Group 69"]
    );
    assert_eq!(summary.subtree_totals[0]["vouchers"], 1);
}

#[test]
fn a_user_group_named_like_the_reserved_root_is_not_the_roots_bucket() {
    use super::super::voucher_groups::GroupHop;
    let under_root = Placement {
        chain: Vec::new(),
        root_label: Some("Primary".to_string()),
    };
    let in_user_group = Placement {
        chain: vec![GroupHop {
            name: "Primary".to_string(),
            reserved_name: String::new(),
        }],
        root_label: None,
    };
    assert_ne!(
        group_key(SummaryGroup::Group, &under_root),
        group_key(SummaryGroup::Group, &in_user_group)
    );
    assert_ne!(
        group_key(SummaryGroup::PrimaryGroup, &under_root),
        group_key(SummaryGroup::PrimaryGroup, &in_user_group)
    );
    // Nor is one named like the key the root itself is filed under.
    let in_group_named_root = Placement {
        chain: vec![GroupHop {
            name: "root".to_string(),
            reserved_name: String::new(),
        }],
        root_label: None,
    };
    assert_ne!(
        group_key(SummaryGroup::Group, &under_root),
        group_key(SummaryGroup::Group, &in_group_named_root)
    );
    assert_ne!(
        group_key(SummaryGroup::PrimaryGroup, &under_root),
        group_key(SummaryGroup::PrimaryGroup, &in_group_named_root)
    );
}

#[test]
fn subtree_totals_are_complete_up_to_the_bound_and_not_beyond() {
    assert!(subtree_totals_complete(MAX_SUBTREE_TOTALS));
    assert!(!subtree_totals_complete(MAX_SUBTREE_TOTALS + 1));
}

#[test]
fn members_of_equal_movement_keep_the_order_they_first_appear_in_never_by_name() {
    // Derived: three ledgers under one group, each credited 5.00, first seen as Zed, Alpha, Mid.
    let mut parents = vec![("Cash".to_string(), Some("Cash-in-Hand".to_string()))];
    let mut entries = vec![
        json!({"ledger": "Cash", "amount": "-15.00", "is_deemed_positive": "Yes", "bill_allocations": []}),
    ];
    for ledger in ["Zed", "Alpha", "Mid"] {
        parents.push((ledger.to_string(), Some("Cash-in-Hand".to_string())));
        entries.push(json!({"ledger": ledger, "amount": "5.00", "is_deemed_positive": "No", "bill_allocations": []}));
    }
    let row = json!({"date": "20250401", "voucher_type": "Journal", "voucher_number": "1", "guid": "g-1", "cancelled": false, "optional": false, "post_dated": false, "amounts": entries});
    let placements = Placements::build(
        parents.iter().map(|(l, p)| (l.as_str(), p.as_deref())),
        crate::agent::voucher_groups::tests::live_groups(),
    );
    let summary = summarise(&[row], &live_request(SummaryGroup::Group, placements)).unwrap();
    let buckets = presented(&summary);
    let order: Vec<&str> = buckets[0]["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["ledger"].as_str().unwrap())
        .collect();
    assert_eq!(order, ["Cash", "Zed", "Alpha", "Mid"]);
}
