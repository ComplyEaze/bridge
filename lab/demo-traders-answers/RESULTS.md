# BRIDGE DEMO TRADERS: read-back against the expected answers (12 Oct 2026)

Book: BRIDGE DEMO TRADERS (company number 100062, GUID d8421da9-5ccc-4c56-8fc5-dcd3190e3568, books from 1 Apr 2025), loaded alone,
TallyPrime 7.1 Silver, port 9001. Imported from lab/demo-traders-book @ 874dc0991c052ceaaf2aa12acef2606bf31be207 through Tally's own
Import menu: masters Created 40, then 13 voucher files, Created 292 in all, Errors 0 everywhere (send-log.txt, import-log.txt).
Every read below is ComplyEaze Bridge's own tool (build 411f7cca), state complete unless stated. Seconds = tool start to evidence end.

| # | Read | Expected | Read from the book | Result | Seconds |
|---|---|---|---|---|---|
| 1 | outstandings, receivable, as at 31 Mar 2026, over 90 days | 7 parties, 539,127.91 | days_90_plus 539127.91; bills over 90 days belong to 7 parties | MATCH | 1.9 |
| 1a | top 3 over 90 days | Vardhman 218,878.10; Patel & Sons 102,816.00; Gopal Krishna 68,311.80 | 218878.10 (DT/25-26/002, 057, 064, 065, 071); 102816.00 (012, 062); 68311.80 (017, 074); next New Bharat 52309.04 | MATCH (summed from open_bills with age_days > 90) | (same read) |
| 2 | profit_and_loss FY 2025-26, sales excl GST | 3,698,434.00 | Tally's own P&L Sales Accounts 3698434.00 (tie_out matched) | MATCH | 1.5 |
| 2a | gross profit (expected only) | 913,319.00 | tool state not_established (tally_balance_sheet_differs on Profit & Loss A/c and Current Assets; an inventory book is expected to refuse); Tally's own lines: Cost of Sales 2786315.00 = Purchases 3983535.00 - Closing Stock 1197220.00, so GP = 912,119.00 | FINDING: differs by 1,200.00 | (same read) |
| 2b | net profit (expected only) | 324,819.00 (323,619.00 if the negative-stock line is valued at its rate) | Tally's own Balance Sheet Profit & Loss A/c 323619.00 (= 912,119.00 - Indirect Expenses 588,500.00) | FINDING: Tally values the negative stock at its rate; NP = the alternative 323,619.00 | (same read) |
| 3 | sales_register FY, top 5 customers | Vardhman, Sunrise, Lakshmi, Meghdoot, Gopal Krishna | taxable (Sales - Goods) per party over 91 rows: Vardhman 615850.00, Sunrise 562868.00, Lakshmi 541162.00, Meghdoot 499112.00, Gopal Krishna 467620.00 (then Patel 357760.00, Saraswati 325890.00, New Bharat 208722.00, Om Sai 119450.00; total 3698434.00) | MATCH (ranking computed from the rows; the tool does not rank) | 15.1 + 18.1 (2 pages) |
| 4 | stock_summary 31 Mar 2026: Packing Tape Roll | -40 Nos | quantity -40 Nos (agreed with Tally's Stock Summary), value 1200.00 (sign as sent: Tally shows (-)1,200, i.e. valued at its rate) | MATCH on quantity; value is the FINDING behind 2a/2b | 0.9 |
| 4a | LED Bulb 9W, Cotton Floor Mop | 0 | quantity state report_has_no_line, value null (Tally's Stock Summary has no line for them); not returned as an explicit 0 | MATCH in substance (no stock), FINDING on form | (same read) |
| 4b | closing stock total | (not given) | tie_out matched -1197220.00; 8 items = Tally count 8; batchwise no, integrated yes | - | (same read) |
| 5 | vouchers 14 Nov 2025 | two cash Payments of 12,000 to Ganga Metal Works | 2 Payments, Ganga Metal Works Dr 12000.00 / Cash Cr 12000.00, Agst Ref GMW/1109 and GMW/1120, Tally Nos 19 and 20 | MATCH | 2.2 |
| 6 | Saraswati Paper Depot open balance | 54,504.20 | outstandings: one open bill DT/25-26/086 54504.20 (10 Mar 2026, 21 days); ledger_movement FY: opening 0.00, debit 380964.60, credit 326460.40, closing 54504.20 Dr, 14 vouchers | MATCH | 9.1 (ledger read) |

Other observations:
- The README's company GSTIN fails Tally's GSTIN check: Tally warned "The Company GSTIN/UIN provided is invalid/incomplete." at GST
  Registration (kept as given, as instructed). No other screen or report visited in this sitting flagged it.
- Tally ignored the voucher numbers in the files and numbered each type from 1 (as the README allows).
- stock_summary returns each stock line's parent as U+FFFD followed by "#4; Primary" (the reserved root's name with a control character).
- Deviation: one re-import of 10-sales-1-first-voucher-only.xml (Altered 1 by REMOTEID, no duplicate; see send-log.txt). Hence the
  voucher alter mark reads 293 for 292 vouchers.
- Not done (by instruction): the Rs 25,000 receipt from Saraswati Paper Depot was not posted; fallback-receipt-c was not imported.
