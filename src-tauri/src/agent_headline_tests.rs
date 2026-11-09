use super::*;

fn date(text: &str) -> bridge_tally_core::TallyDate {
    bridge_tally_core::TallyDate::parse(text.to_string()).unwrap()
}

fn company() -> CompanyName {
    CompanyName::new("Synthetic Traders")
}

fn basis(gaps: Vec<Gap>) -> TrialBalanceBasis {
    TrialBalanceBasis::new(
        date("20260401"),
        date("20260902"),
        Completeness::from_gaps(gaps),
    )
}

fn page(offset: usize, shown: usize, total: usize) -> Page {
    Page::new(Rows::Ledgers, offset, shown, total)
}

#[test]
fn a_read_with_no_gap_is_whole_and_says_the_company_and_the_exact_period() {
    let headline = basis(vec![]).headline(&company(), page(0, 14, 14));
    assert_eq!(
        headline.lead,
        "Trial balance for \u{201c}Synthetic Traders\u{201d}, 1 Apr 2026 to 2 Sep 2026: read for every ledger."
    );
    assert_eq!(headline.rows.as_deref(), Some("All 14 ledgers are listed."));
    assert!(!headline.lead.contains("Partial"));
}

/// The state is in the lead, in words: the first sentence says it is partial
/// and what that costs, and the counts of what was left out follow it.
#[test]
fn a_gap_makes_the_read_partial_and_the_lead_says_so_and_counts_it() {
    for (foreign, mixed, counts) in [
        (3, 1, "3 ledgers kept in another currency and 1 base-currency ledger with a value Tally shows in another currency are left out"),
        (1, 0, "1 ledger kept in another currency and 0 base-currency ledgers with a value Tally shows in another currency are left out"),
        (0, 5, "0 ledgers kept in another currency and 5 base-currency ledgers with a value Tally shows in another currency are left out"),
    ] {
        let headline = basis(vec![Gap::BaseCurrencyLedgersOnly { foreign, mixed }])
            .headline(&company(), page(0, 5, 5));
        let (first, rest) = headline.lead.split_once(". ").unwrap();
        assert!(first.starts_with("Partial trial balance for "), "{first}");
        assert!(
            first.ends_with("base-currency ledgers only, so debit and credit totals are not expected to match"),
            "{first}"
        );
        assert!(rest.contains(counts), "{rest}");
        assert!(first.split_whitespace().count() < 30, "a short first sentence: {first}");
    }
}

/// A several-currency book with nothing set aside is still read for its
/// base-currency ledgers only, and says so without claiming ledgers were left
/// out or that the totals will differ.
#[test]
fn a_several_currency_book_with_nothing_set_aside_says_so() {
    let headline = basis(vec![Gap::BaseCurrencyLedgersOnly {
        foreign: 0,
        mixed: 0,
    }])
    .headline(&company(), page(0, 5, 5));
    assert_eq!(
        headline.lead,
        "Partial trial balance for \u{201c}Synthetic Traders\u{201d}, 1 Apr 2026 to 2 Sep 2026: base-currency ledgers only, and no ledger was left out. The book has several currency masters, so only its base-currency ledgers were read."
    );
    assert!(!headline.lead.contains("0 ledgers"), "{}", headline.lead);
}

/// Every gap is named, however many there are: none is dropped for another.
#[test]
fn every_gap_is_named_in_the_lead() {
    let completeness = Completeness::from_gaps(vec![
        Gap::BaseCurrencyLedgersOnly {
            foreign: 2,
            mixed: 0,
        },
        Gap::BaseCurrencyLedgersOnly {
            foreign: 7,
            mixed: 4,
        },
    ]);
    let headline = TrialBalanceBasis::new(date("20260401"), date("20260902"), completeness)
        .headline(&company(), page(0, 1, 1));
    assert!(
        headline
            .lead
            .contains("2 ledgers kept in another currency and 0 base-currency"),
        "{}",
        headline.lead
    );
    assert!(
        headline
            .lead
            .contains("7 ledgers kept in another currency and 4 base-currency"),
        "{}",
        headline.lead
    );
}

#[test]
fn only_an_empty_list_of_gaps_is_whole() {
    assert!(Completeness::from_gaps(Vec::new()).is_whole());
    assert!(!Completeness::from_gaps(vec![Gap::BaseCurrencyLedgersOnly {
        foreign: 0,
        mixed: 0
    }])
    .is_whole());
}

#[test]
fn the_page_sentence_says_which_rows_and_where_the_rest_begin() {
    let words = |offset, shown, total| page(offset, shown, total).sentence();
    assert_eq!(words(0, 14, 14), "All 14 ledgers are listed.");
    assert_eq!(
        words(0, 3, 14),
        "Ledgers 1 to 3 of 14; more follow, from offset 3."
    );
    assert_eq!(
        words(3, 3, 14),
        "Ledgers 4 to 6 of 14; more follow, from offset 6."
    );
    assert_eq!(words(12, 2, 14), "Ledgers 13 to 14 of 14: the last page.");
    assert_eq!(words(0, 0, 0), "There are no ledgers in this read.");
    assert!(words(20, 0, 14).starts_with("No ledgers on this page: there are 14 in all"));
}

/// A byte cap that trims the rows restates the row sentence from the rows that
/// are left, in the response itself; a whole page cut to part of it stops
/// saying "all".
#[test]
fn a_trimmed_page_restates_its_rows_and_never_claims_more_than_it_holds() {
    let mut response = json!({
        "headline": basis(vec![]).headline(&company(), page(0, 14, 14)),
        "result": {"ledgers": []},
    });
    assert_eq!(response["headline"]["rows"], "All 14 ledgers are listed.");
    restate_rows(&mut response, 9);
    assert_eq!(
        response["headline"]["rows"],
        "Ledgers 1 to 9 of 14; more follow, from offset 9."
    );
    // The lead is not touched, and a second cut restates again from what is left.
    assert!(response["headline"]["lead"]
        .as_str()
        .unwrap()
        .contains("read for every ledger"));
    restate_rows(&mut response, 4);
    assert_eq!(
        response["headline"]["rows"],
        "Ledgers 1 to 4 of 14; more follow, from offset 4."
    );
    // More rows than the page holds is not a claim.
    restate_rows(&mut response, 99);
    assert_eq!(
        response["headline"]["rows"],
        "Ledgers 1 to 4 of 14; more follow, from offset 4."
    );
}

#[test]
fn a_response_without_a_headline_is_left_alone() {
    let mut response = json!({"result": {"ledgers": [1, 2]}});
    restate_rows(&mut response, 1);
    assert_eq!(response, json!({"result": {"ledgers": [1, 2]}}));
}

/// Tally's company name is unrestricted text: control characters, bidirectional
/// overrides and zero-width marks are removed so that it cannot reorder or
/// hide the words around it, quotes inside it become apostrophes, it is always
/// quoted, and it is held to a bound.
#[test]
fn the_company_name_is_quoted_cleaned_and_bounded() {
    let name = CompanyName::new("  Odd\u{7}\nName \u{201c}Ltd\u{201d} \"x\" ");
    assert_eq!(name.quoted(), "\u{201c}Odd Name 'Ltd' 'x'\u{201d}");
    // A name written to close the quote and read as a sentence cannot: its
    // own quotes are apostrophes, so the state after it stays outside.
    let sentence =
        CompanyName::new("X\u{201d}, 1 Apr 2025 to 31 Mar 2026: read for every ledger. \u{201c}");
    assert_eq!(sentence.quoted().matches('\u{201d}').count(), 1);
    assert_eq!(sentence.quoted().matches('\u{201c}').count(), 1);
    let spoof = CompanyName::new("A\u{202e}B\u{200b}C\u{2066}D\u{feff}E");
    assert_eq!(spoof.quoted(), "\u{201c}ABCDE\u{201d}");
    // By category, not by list: the Arabic letter mark, the tag characters and
    // a soft hyphen are format characters too.
    let more = CompanyName::new("A\u{61c}B\u{e0041}C\u{ad}D\u{3164}E\u{fff9}F\u{600}G");
    assert_eq!(more.quoted(), "\u{201c}ABCDEFG\u{201d}");
    let long = CompanyName::new(&"n".repeat(5_000));
    assert_eq!(long.0.chars().count(), MAX_COMPANY_NAME_CHARS);
}

/// An offset at the top of the range is a sentence, not an overflow.
#[test]
fn an_offset_at_the_top_of_the_range_does_not_overflow() {
    let sentence = page(usize::MAX, 0, 14).sentence();
    assert!(
        sentence.starts_with("No ledgers on this page"),
        "{sentence}"
    );
    let sentence = page(usize::MAX - 1, 5, 14).sentence();
    assert!(sentence.contains("of 14"), "{sentence}");
}

/// A headline that cannot be restated fails closed: its rows sentence goes
/// rather than staying beside rows it no longer describes.
#[test]
fn a_headline_that_cannot_be_restated_loses_its_rows_sentence() {
    let mut response = json!({
        "headline": {"lead": "x", "rows": "All 14 ledgers are listed.", "page": {"rows": "vouchers", "offset": 0, "shown": 14, "total": 14}},
    });
    restate_rows(&mut response, 3);
    assert_eq!(response["headline"], json!({"lead": "x"}));
}

#[test]
fn dates_are_written_for_a_person() {
    assert_eq!(plain_date(&date("20260401")), "1 Apr 2026");
    assert_eq!(plain_date(&date("20251231")), "31 Dec 2025");
}

fn statement(
    kind: StatementKind,
    parts: Vec<(StatementPart, PartOutcome)>,
    read_tally_profit_and_loss: bool,
) -> Headline {
    StatementBasis::new(
        kind,
        date("20260401"),
        date("20260902"),
        parts,
        read_tally_profit_and_loss,
    )
    .headline(&company())
}

fn not_established(reason: NotEstablishedReason, lines: usize) -> PartOutcome {
    PartOutcome::NotEstablished {
        reason,
        differing_lines: lines,
    }
}

const EVERY_REASON: [NotEstablishedReason; 6] = [
    NotEstablishedReason::UnclassifiedLedgerCarriesAnAmount,
    NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance,
    NotEstablishedReason::ProfitAndLossLedgerNotReturned,
    NotEstablishedReason::TallyBalanceSheetDiffers,
    NotEstablishedReason::TallyProfitAndLossDiffers,
    NotEstablishedReason::TallyBalanceSheetDoesNotBalance,
];

/// Every reason a statement can be not established is in its own words, and
/// the lead never says "established" over a part that is not: whichever part
/// fails, however many fail, the lead starts with "Not established".
#[test]
fn a_statement_part_that_is_not_established_is_always_in_the_lead() {
    for reason in EVERY_REASON {
        for (gross, net) in [
            (PartOutcome::Established, not_established(reason, 0)),
            (not_established(reason, 2), PartOutcome::Established),
            (not_established(reason, 0), not_established(reason, 1)),
        ] {
            let headline = statement(
                StatementKind::ProfitAndLoss,
                vec![
                    (StatementPart::GrossResult, gross),
                    (StatementPart::NetResult, net),
                ],
                true,
            );
            assert!(
                headline
                    .lead
                    .starts_with("Not established: the profit and loss for "),
                "{}",
                headline.lead
            );
            assert!(
                headline.lead.contains(reason_words(reason)),
                "{}",
                headline.lead
            );
            assert!(
                !headline.lead.contains("passed the comparison"),
                "never says nothing differs over a part that is not established: {}",
                headline.lead
            );
        }
    }
}

/// The two parts are each named with their own state: an established gross
/// beside a net that is not says both.
#[test]
fn each_part_is_named_with_its_own_state() {
    let headline = statement(
        StatementKind::ProfitAndLoss,
        vec![
            (StatementPart::GrossResult, PartOutcome::Established),
            (
                StatementPart::NetResult,
                not_established(NotEstablishedReason::TallyProfitAndLossDiffers, 3),
            ),
        ],
        true,
    );
    assert!(headline.lead.contains("The gross result is established. The net result is not established because Tally's own Profit and Loss differs from the derived lines (on 3 lines)"), "{}", headline.lead);
    // The lines are withheld whenever the net result is not established.
    assert!(
        headline.lead.contains("The derived lines are withheld"),
        "{}",
        headline.lead
    );
}

#[test]
fn an_established_statement_says_what_it_ties_to_and_no_more() {
    let both = statement(
        StatementKind::ProfitAndLoss,
        vec![
            (StatementPart::GrossResult, PartOutcome::Established),
            (StatementPart::NetResult, PartOutcome::Established),
        ],
        true,
    );
    assert_eq!(
        both.lead,
        "Profit and loss for \u{201c}Synthetic Traders\u{201d}, 1 Apr 2026 to 2 Sep 2026: the gross result and the net result are established, after the derived lines passed the comparison with Tally's own Balance Sheet and Profit and Loss."
    );
    let sheet = statement(
        StatementKind::BalanceSheet,
        vec![(
            StatementPart::BalanceSheetProfitAndLoss,
            PartOutcome::Established,
        )],
        false,
    );
    // Pinned whole: one part takes "is", which a lead checked only at its end
    // would not show.
    assert_eq!(
        sheet.lead,
        "Balance sheet for \u{201c}Synthetic Traders\u{201d}, 1 Apr 2026 to 2 Sep 2026: the profit and loss line of the balance sheet is established, after the derived lines passed the comparison with Tally's own Balance Sheet."
    );
    assert!(sheet.rows.is_none() && sheet.page.is_none());
}

/// A statement's outcome comes from the derived result by an exhaustive match.
#[test]
fn a_part_outcome_is_read_from_the_derived_result() {
    use crate::reports::statements::Established;
    let value = bridge_tally_core::ExactDecimal::parse("1.00").unwrap();
    assert_eq!(
        PartOutcome::of(&Established::Established { value }),
        PartOutcome::Established
    );
    assert_eq!(
        PartOutcome::of(&Established::NotEstablished {
            reason: NotEstablishedReason::TallyBalanceSheetDiffers,
            lines: vec!["a".into(), "b".into()],
        }),
        not_established(NotEstablishedReason::TallyBalanceSheetDiffers, 2)
    );
}

/// Each reason is worded as itself, and no two are worded alike.
#[test]
fn each_reason_has_its_own_words() {
    let words = EVERY_REASON.map(reason_words);
    assert_eq!(
        words,
        [
            "a ledger the derivation cannot classify carries an amount",
            "the book has a Stock-in-Hand balance and closing stock is not derived",
            "Tally did not return its Profit & Loss A/c ledger",
            "Tally's own Balance Sheet differs from the derived lines",
            "Tally's own Profit and Loss differs from the derived lines",
            "Tally's own Balance Sheet does not balance",
        ]
    );
}

/// A reason that carries no differing lines prints no count: a count of zero
/// never reads as "0 lines differ".
#[test]
fn a_reason_without_differing_lines_prints_no_count() {
    let headline = statement(
        StatementKind::BalanceSheet,
        vec![(
            StatementPart::BalanceSheetProfitAndLoss,
            not_established(
                NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance,
                0,
            ),
        )],
        false,
    );
    assert!(!headline.lead.contains("(on "), "{}", headline.lead);
    assert!(
        headline
            .lead
            .contains("closing stock is not derived. The derived lines"),
        "{}",
        headline.lead
    );
}

/// Tally's own Profit and Loss counts only for a profit and loss: a balance
/// sheet that happened to carry one claims no comparison with it, and a profit
/// and loss without one claims only the Balance Sheet.
#[test]
fn the_profit_and_loss_comparison_is_claimed_only_for_a_profit_and_loss_that_read_it() {
    let parts = |part| vec![(part, PartOutcome::Established)];
    let sheet = statement(
        StatementKind::BalanceSheet,
        parts(StatementPart::BalanceSheetProfitAndLoss),
        true,
    );
    assert!(
        sheet.lead.ends_with("with Tally's own Balance Sheet."),
        "{}",
        sheet.lead
    );
    let without = statement(
        StatementKind::ProfitAndLoss,
        parts(StatementPart::NetResult),
        false,
    );
    assert!(
        without.lead.ends_with("with Tally's own Balance Sheet."),
        "{}",
        without.lead
    );
}

/// Whether the lines are withheld follows the parts: it cannot be set apart
/// from them, so "Not established" with lines shown cannot be built.
#[test]
fn the_withheld_note_follows_the_parts() {
    let gross_only = statement(
        StatementKind::ProfitAndLoss,
        vec![
            (
                StatementPart::GrossResult,
                not_established(NotEstablishedReason::TallyBalanceSheetDiffers, 1),
            ),
            (StatementPart::NetResult, PartOutcome::Established),
        ],
        true,
    );
    assert!(!gross_only.lead.contains("withheld"), "{}", gross_only.lead);
    let net = statement(
        StatementKind::ProfitAndLoss,
        vec![(
            StatementPart::NetResult,
            not_established(NotEstablishedReason::TallyBalanceSheetDiffers, 1),
        )],
        true,
    );
    assert!(
        net.lead.contains("The derived lines are withheld"),
        "{}",
        net.lead
    );
}

/// A lead that is not established reads as sentences with one colon, says
/// what is withheld, and gives a next step for each reason, once each.
#[test]
fn a_not_established_lead_is_plain_and_has_a_next_step_for_each_reason() {
    for reason in EVERY_REASON {
        let headline = statement(
            StatementKind::ProfitAndLoss,
            vec![
                (StatementPart::GrossResult, not_established(reason, 0)),
                (StatementPart::NetResult, not_established(reason, 0)),
            ],
            true,
        );
        assert_eq!(headline.lead.matches(':').count(), 1, "{}", headline.lead);
        assert_eq!(
            headline.lead.matches(reason_next_step(reason)).count(),
            1,
            "once, not once per part: {}",
            headline.lead
        );
    }
    let two = statement(
        StatementKind::ProfitAndLoss,
        vec![
            (
                StatementPart::GrossResult,
                not_established(NotEstablishedReason::TallyBalanceSheetDiffers, 1),
            ),
            (
                StatementPart::NetResult,
                not_established(NotEstablishedReason::TallyProfitAndLossDiffers, 2),
            ),
        ],
        true,
    );
    assert!(
        two.lead.contains("See balance_sheet_gate in the result"),
        "{}",
        two.lead
    );
    assert!(
        two.lead.contains("See tie_out in the result"),
        "{}",
        two.lead
    );
}

/// Each reason's next step is its own.
#[test]
fn each_reason_has_its_own_next_step() {
    let steps = EVERY_REASON.map(reason_next_step);
    for (index, step) in steps.iter().enumerate() {
        assert!(!step.is_empty());
        assert!(
            steps.iter().skip(index + 1).all(|other| other != step),
            "{step}"
        );
    }
}

// ---- cash flow ----

fn cash_flow_basis(outcome: CashFlowOutcome) -> CashFlowBasis {
    CashFlowBasis::new(date("20250401"), date("20260331"), outcome, false)
}

#[test]
fn a_tied_cash_flow_says_what_was_checked_and_what_was_not() {
    let lead = cash_flow_basis(CashFlowOutcome::Tied)
        .headline(&CompanyName::new("Lab Co"))
        .lead;
    assert!(
        lead.starts_with("Cash flow for \u{201c}Lab Co\u{201d}, 1 Apr 2025 to 31 Mar 2026:"),
        "{lead}"
    );
    assert!(lead.contains("equals the cash and bank ledgers"), "{lead}");
    assert!(lead.contains("has not been checked"), "{lead}");
    assert!(
        lead.contains("not a cash flow statement under AS 3"),
        "{lead}"
    );
}

#[test]
fn every_cash_flow_state_that_is_not_established_leads_with_it_and_withholds_the_months() {
    for (outcome, words) in [
        (
            CashFlowOutcome::Differs,
            "differs from the cash and bank ledgers",
        ),
        (
            CashFlowOutcome::MoneyGroupUnmeasured,
            "Bank OD A/c or Bank OCC A/c",
        ),
        (CashFlowOutcome::NothingToCompare, "Nothing could be tied"),
    ] {
        let basis = cash_flow_basis(outcome);
        let lead = basis.headline(&company()).lead;
        assert!(
            lead.starts_with("Not established: the cash flow for"),
            "{lead}"
        );
        assert!(lead.contains(words), "{lead}");
        assert!(lead.contains("months are withheld"), "{lead}");
        assert!(basis.months_withheld());
    }
    assert!(!cash_flow_basis(CashFlowOutcome::Tied).months_withheld());
}

#[test]
fn a_tied_cash_flow_with_an_unmeasured_shape_says_so_in_its_lead() {
    let measured = CashFlowBasis::new(
        date("20250401"),
        date("20260331"),
        CashFlowOutcome::Tied,
        false,
    )
    .headline(&company())
    .lead;
    assert!(
        measured.contains("No month's figure has been checked against"),
        "{measured}"
    );
    let unmeasured = CashFlowBasis::new(
        date("20250401"),
        date("20260331"),
        CashFlowOutcome::Tied,
        true,
    )
    .headline(&company())
    .lead;
    assert!(
        unmeasured.contains("treat each month's figure as unverified"),
        "{unmeasured}"
    );
    // What was and was not run is stated, not left to the reader.
    assert!(
        unmeasured.contains("a window from March into April was not run"),
        "{unmeasured}"
    );
    assert!(
        unmeasured.contains("no month was compared with Tally's own Cash Flow screen"),
        "{unmeasured}"
    );
    assert!(
        !unmeasured.contains("No month's figure has been checked against"),
        "{unmeasured}"
    );
}

fn features(
    cost_centres: bridge_tally_protocol::native_company_features::NativeSetting,
    gst: bridge_tally_protocol::native_company_features::NativeSetting,
    currency: bridge_tally_protocol::native_company_features::NativeCurrencySymbol,
) -> CompanyFeaturesBasis {
    use bridge_tally_protocol::native_company_features::{NativeCompanyFeatures, NativeSetting};
    CompanyFeaturesBasis::new(&NativeCompanyFeatures {
        cost_centres,
        gst,
        batch_wise: NativeSetting::Yes,
        base_currency: currency,
    })
}

#[test]
fn a_company_features_headline_says_today_not_the_books_and_what_was_compared() {
    use bridge_tally_protocol::native_company_features::{NativeCurrencySymbol, NativeSetting};
    let basis = features(
        NativeSetting::No,
        NativeSetting::Yes,
        NativeCurrencySymbol::Reported("\u{20b9}".to_string()),
    );
    assert!(basis.all_reported());
    assert_eq!(
        basis.headline(&company()).lead,
        "The settings Tally's company record holds today for \u{201c}Synthetic Traders\u{201d}: cost centres off, GST on, batch-wise stock on. They are not what the books contain or what they were during a year, and each of the three has been compared with Tally's own screen, on synthetic books. The company's currency symbol is \u{20b9}: a symbol, not a currency code."
    );
}

#[test]
fn a_setting_tally_did_not_send_leads_as_not_sent_and_not_as_off() {
    use bridge_tally_protocol::native_company_features::{NativeCurrencySymbol, NativeSetting};
    let basis = features(
        NativeSetting::NotReported,
        NativeSetting::Yes,
        NativeCurrencySymbol::NotReported,
    );
    assert!(!basis.all_reported());
    let lead = basis.headline(&company()).lead;
    assert!(lead.contains("cost centres not sent by Tally"), "{lead}");
    assert!(
        lead.contains("A setting Tally did not send is not reported; that is not the same as off."),
        "{lead}"
    );
    assert!(lead.ends_with("Tally sent no currency symbol."), "{lead}");
    assert!(!lead.contains("cost centres off"), "{lead}");
}

fn outcome(statuses: &[&str], verdict: BatchVerdict) -> BatchOutcome {
    BatchOutcome::from_statuses(statuses.iter().copied(), verdict).expect("a batch with rows")
}

/// A batch of several vouchers that all read back, and read back as a whole,
/// is confirmed in one sentence; one voucher has its own.
#[test]
fn a_batch_that_is_confirmed_whole_says_so_in_one_sentence() {
    let verified = ["posted_verified"; 3];
    assert_eq!(
        outcome(&verified, BatchVerdict::PostedVerified)
            .headline()
            .lead,
        "All 3 vouchers of this batch are confirmed in Tally."
    );
    assert_eq!(
        outcome(&verified[..1], BatchVerdict::PostedVerified)
            .headline()
            .lead,
        "The voucher of this batch is confirmed in Tally."
    );
}

/// The count is by status and the sentence names how many are not confirmed;
/// a single voucher left over is singular.
#[test]
fn a_batch_with_vouchers_left_says_how_many_and_counts_each_status() {
    let partly = outcome(
        &[
            "posted_verified",
            "posted_divergent",
            "not_found",
            "mystery",
        ],
        BatchVerdict::PostedVerified,
    );
    assert_eq!(
        partly.headline().lead,
        "1 of 4 vouchers of this batch reads back in Tally, but the batch is not confirmed; the result says why. Check the batch in Tally before doing anything else with it, starting with each of the other 3; each row says what was found, and gives a next step where there is one. ComplyEaze Bridge will not send this saved batch again."
    );
    assert_eq!(
        serde_json::to_value(&partly).unwrap(),
        json!({
            "vouchers": 4, "confirmed": 1, "verdict": "verification_incomplete",
            "not_confirmed": {
                "matching_content_observed": 0, "sent_not_attributed": 0,
                "tally_reported_not_created": 0, "posted_divergent": 1,
                "posted_not_effective": 0, "not_found": 1, "not_attributable": 0,
                "duplicate_fingerprint": 0, "bound_not_in_window": 0,
                "book_rolled_back": 0, "unrecognised": 1,
            },
        })
    );
    let one = outcome(
        &["posted_verified", "posted_verified", "book_rolled_back"],
        BatchVerdict::VerificationIncomplete,
    );
    assert_eq!(
        one.headline().lead,
        "2 of 3 vouchers of this batch read back in Tally, but the batch is not confirmed; the result says why. Check the batch in Tally before doing anything else with it, starting with the other one; its row says what was found, and gives a next step where there is one. ComplyEaze Bridge will not send this saved batch again."
    );
}

/// Some vouchers read back and the rest do not. The counts hold only the rows,
/// so a doubt about the whole batch (an answer from Tally that was lost or not
/// clean, a doubt about the ledgers or the voucher mark, a duplicate) cannot
/// be told from a doubt about one row: the vouchers that read back are never
/// called confirmed, and the sentence always says the batch is not, whatever
/// verdict the caller passed.
#[test]
fn a_batch_that_reads_back_in_part_is_never_called_confirmed_and_always_says_the_batch_is_not() {
    let others = [
        "matching_content_observed",
        "sent_not_attributed",
        "tally_reported_not_created",
        "posted_divergent",
        "posted_not_effective",
        "not_found",
        "not_attributable",
        "duplicate_fingerprint",
        "bound_not_in_window",
        "book_rolled_back",
        "mystery",
    ];
    for other in others {
        for read_back in 1..=3 {
            for rest in 1..=2 {
                let mut statuses = vec!["posted_verified"; read_back];
                statuses.extend(std::iter::repeat_n(other, rest));
                let vouchers = read_back + rest;
                // The caller's verdict is replaced by the rows; a batch built
                // with the other verdict directly must read the same.
                let mut batches = vec![
                    outcome(&statuses, BatchVerdict::PostedVerified),
                    outcome(&statuses, BatchVerdict::VerificationIncomplete),
                ];
                let mut forced = outcome(&statuses, BatchVerdict::VerificationIncomplete);
                forced.verdict = BatchVerdict::PostedVerified;
                batches.push(forced);
                for batch in batches {
                    let lead = batch.headline().lead;
                    let case = format!("{other} {read_back}+{rest}: {lead}");
                    let reads = if read_back == 1 { "reads" } else { "read" };
                    assert!(
                        lead.starts_with(&format!(
                            "{read_back} of {vouchers} vouchers of this batch {reads} back in Tally, but the batch is not confirmed; the result says why. Check the batch in Tally before doing anything else with it, starting with "
                        )),
                        "{case}"
                    );
                    assert!(!lead.contains("confirmed in Tally"), "{case}");
                }
            }
        }
    }
    // Nothing read back and several vouchers: none is confirmed, and the
    // sentence says so.
    assert!(
        outcome(&["not_found", "not_found"], BatchVerdict::PostedVerified)
            .headline()
            .lead
            .starts_with("0 of 2 vouchers of this batch are confirmed in Tally. 2 are not:")
    );
}

/// Every voucher reads back but the batch is not confirmed as a whole (the
/// readback found a duplicate, or a doubt about the ledgers or the voucher
/// mark): the headline never says all are confirmed.
#[test]
fn all_vouchers_read_back_beside_an_unconfirmed_batch_is_not_called_confirmed() {
    let lead = outcome(
        &["posted_verified"; 2],
        BatchVerdict::VerificationIncomplete,
    )
    .headline()
    .lead;
    assert_eq!(
        lead,
        "All 2 vouchers of this batch read back in Tally, but the batch is not confirmed: check it in Tally before doing anything else with it; the result says why. ComplyEaze Bridge will not send this saved batch again."
    );
    assert!(!lead.contains("are confirmed"), "{lead}");
}

/// One voucher that reads back, in a batch that is not confirmed as a whole,
/// has its own sentence.
#[test]
fn one_voucher_that_reads_back_beside_an_unconfirmed_batch_is_not_called_confirmed() {
    let lead = outcome(&["posted_verified"], BatchVerdict::VerificationIncomplete)
        .headline()
        .lead;
    assert_eq!(
        lead,
        "The voucher of this batch reads back in Tally, but the batch is not confirmed: check it in Tally before doing anything else with it; the result says why. ComplyEaze Bridge will not send this saved batch again."
    );
}

/// A batch of one voucher that is not confirmed has its own sentence, whether
/// or not its row carries a next step, and its typed counts say the same.
#[test]
fn one_voucher_that_is_not_confirmed_has_its_own_sentence_and_counts() {
    // One status that carries a next step and one that does not.
    for status in ["sent_not_attributed", "posted_divergent"] {
        let batch = outcome(&[status], BatchVerdict::PostedVerified);
        assert_eq!(
            batch.headline().lead,
            "The voucher of this batch is not confirmed in Tally: check it in Tally before doing anything else with it; its row says what was found, and gives a next step where there is one. ComplyEaze Bridge will not send this saved batch again.",
            "{status}"
        );
        let value = serde_json::to_value(&batch).unwrap();
        assert_eq!(value["vouchers"], 1, "{status}");
        assert_eq!(value["confirmed"], 0, "{status}");
        assert_eq!(value["verdict"], "verification_incomplete", "{status}");
        assert_eq!(value["not_confirmed"][status], 1, "{status}");
        let total = value["not_confirmed"]
            .as_object()
            .unwrap()
            .values()
            .map(|count| count.as_u64().unwrap())
            .sum::<u64>();
        assert_eq!(total, 1, "{status}");
    }
}

/// Each status other than `posted_verified` is counted in its own field, and
/// no field takes another's voucher.
#[test]
fn every_status_is_counted_in_its_own_field() {
    // The fields of the counter itself, so a status added to it is walked
    // here; one with no arm in `count` lands in `unrecognised` and fails.
    let fields = serde_json::to_value(NotConfirmed::default()).unwrap();
    let statuses = fields
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    assert!(statuses.contains(&"unrecognised"));
    assert!(statuses.len() > 1);
    for (index, &status) in statuses.iter().enumerate() {
        // Each status is counted a different number of times, so two fields
        // swapped by mistake cannot read the same.
        let times = index + 1;
        let batch = outcome(&vec![status; times], BatchVerdict::PostedVerified);
        let value = serde_json::to_value(&batch).unwrap();
        for &other in &statuses {
            let wanted = if other == status { times } else { 0 };
            assert_eq!(value["not_confirmed"][other], wanted, "{status} / {other}");
        }
        assert_eq!(value["vouchers"], times, "{status}");
        assert_eq!(value["confirmed"], 0, "{status}");
    }
}

/// A voucher that is not confirmed cannot sit in a batch whose verdict is
/// confirmed, whatever the caller passed; and a batch with no rows has none.
#[test]
fn the_verdict_cannot_be_confirmed_beside_a_voucher_that_is_not() {
    let batch = outcome(
        &["posted_verified", "matching_content_observed"],
        BatchVerdict::PostedVerified,
    );
    assert_eq!(batch.verdict, BatchVerdict::VerificationIncomplete);
    assert!(BatchOutcome::from_statuses([], BatchVerdict::PostedVerified).is_none());
    assert_eq!(
        BatchVerdict::from_status(None),
        BatchVerdict::VerificationIncomplete
    );
}

/// A headline with no page (a batch's) comes through `restate_rows` as it
/// went in, counts included. It does not show that a trim keeps the counts of
/// a headline that has a page: a batch headline has none.
#[test]
fn restating_the_rows_of_a_response_leaves_a_batch_headline_as_it_was() {
    let headline = outcome(
        &["posted_verified", "not_found"],
        BatchVerdict::PostedVerified,
    )
    .headline();
    let before = serde_json::to_value(&headline).unwrap();
    let mut response = json!({"headline": headline, "result": {"items": [1]}});
    restate_rows(&mut response, 0);
    assert_eq!(response["headline"], before);
}

/// Tally's own Balance Sheet export leaves out its Difference in opening
/// balances line (#2063): when its sides differ the lead says both totals,
/// whatever the reason a part is not established, and gives the next step once.
#[test]
fn a_balance_sheet_whose_sides_differ_says_both_totals_in_the_lead() {
    for reason in [
        NotEstablishedReason::TallyBalanceSheetDiffers,
        NotEstablishedReason::TallyBalanceSheetDoesNotBalance,
    ] {
        let headline = StatementBasis::new(
            StatementKind::BalanceSheet,
            date("20260401"),
            date("20260902"),
            vec![(
                StatementPart::BalanceSheetProfitAndLoss,
                not_established(reason, 0),
            )],
            false,
        )
        .with_tally_balance_sheet_sides("4846.67", "4946.67")
        .headline(&company());
        assert!(
            headline.lead.contains(
                "Tally's own Balance Sheet export does not balance (its credit lines add to 4846.67 and its debit lines to 4946.67)"
            ),
            "{}",
            headline.lead
        );
        assert_eq!(
            headline
                .lead
                .matches("Difference in opening balances")
                .count(),
            2,
            "the sentence and the next step, once each: {}",
            headline.lead
        );
    }
    let balanced = statement(
        StatementKind::BalanceSheet,
        vec![(
            StatementPart::BalanceSheetProfitAndLoss,
            not_established(NotEstablishedReason::TallyBalanceSheetDiffers, 1),
        )],
        false,
    );
    assert!(
        !balanced.lead.contains("export does not balance"),
        "{}",
        balanced.lead
    );
}

/// Ledgers that do not add up (#2068) are counted in the lead of every page of the read, whole
/// or partial, and a read with none says nothing about it.
#[test]
fn ledgers_that_do_not_add_up_are_counted_on_every_page_and_only_then() {
    let none = basis(vec![]).headline(&company(), page(0, 14, 14));
    assert!(!none.lead.contains("not add up"), "{}", none.lead);
    for (count, words) in [
        (1, "1 ledger does not add up"),
        (3, "3 ledgers do not add up"),
    ] {
        for (offset, shown) in [(0, 5), (5, 5), (10, 4)] {
            let headline = basis(vec![])
                .with_movements_differ(count)
                .headline(&company(), page(offset, shown, 14));
            assert!(headline.lead.contains(words), "{}", headline.lead);
            assert!(
                headline.lead.contains("movements_differ_by"),
                "{}",
                headline.lead
            );
        }
    }
    let partial = basis(vec![Gap::BaseCurrencyLedgersOnly {
        foreign: 1,
        mixed: 0,
    }])
    .with_movements_differ(2)
    .headline(&company(), page(0, 5, 5));
    assert!(
        partial.lead.starts_with("Partial trial balance"),
        "{}",
        partial.lead
    );
    assert!(
        partial.lead.contains("2 ledgers do not add up"),
        "{}",
        partial.lead
    );
}
