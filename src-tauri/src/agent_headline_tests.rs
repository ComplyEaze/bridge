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

const EVERY_REASON: [NotEstablishedReason; 5] = [
    NotEstablishedReason::UnclassifiedLedgerCarriesAnAmount,
    NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance,
    NotEstablishedReason::ProfitAndLossLedgerNotReturned,
    NotEstablishedReason::TallyBalanceSheetDiffers,
    NotEstablishedReason::TallyProfitAndLossDiffers,
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
