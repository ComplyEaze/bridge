# `builtin_cash_flow_*`, `builtin_negative_ledgers_*`, `builtin_unknown_report_*` — provenance

Live captures of Tally's own Cash Flow, Funds Flow, Sales Register, Ratio Analysis, Negative Stock and Negative Ledgers reports, and of its answer to a report name it does not have, requested by report name (protocol reference §12a.1's envelope). Every response file is a live capture, never hand-written. Only Cash Flow, Funds Flow, Negative Stock, the Negative Ledgers answers (empty, and with two ledgers) and the unknown-name refusal are read by a parser and its tests; the Sales Register and Ratio Analysis files are kept as the evidence for protocol reference §12a.16, and no code reads them yet.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-10-06, 18:37–18:53 IST, one request at a time, attended. Every answer was HTTP 200, and a status probe after every request returned HTTP 200 with a 51-byte body. No dialog appeared at the Tally screen. The capture script's per-request timings are not kept here.
- **Book:** `BRIDGE PROBE B SANDBOX`, a synthetic lab company (single INR currency, 25 ledgers, books from 2025-04-01), for every file except the Negative Stock pair, which is `BRIDGE SHAPE LAB` (a synthetic lab company with 11 stock items and two currency masters, the same company as the stock fixtures). Its trial balance for 2025-04-01 to 2026-03-31 had Cash-in-Hand `Cash` (debit 5,500.00) and Bank Accounts `W1 Bank` (debit 17,970,481.22), no credit on either, and no ledger with a balance on the side opposite its group.
- **Sender:** a lab capture script, not `bridge_mcp`. It sent the exact request bytes tabled below (UTF-16LE with a BOM), whose text is the envelope `render_built_in_report_request` produces; a test asserts that equality for the three Cash Flow requests, the Negative Stock request and the Funds Flow request (the other four request files are parked evidence for later slices). The requests were pinned by SHA-256 before they were sent.
- **Encoding:** responses are **BOM-less UTF-16LE**, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the captures show:**
  - Cash Flow: no `HEADER` or `STATUS`, and one `DSPPERIOD` (a month name, **no year**) followed by one `DSPACCINFO` (`DSPDRAMT/DSPDRAMTA`, `DSPCRAMT/DSPCRAMTA`, `DSPCLAMT/DSPCLAMTA`) per month: 12 for the year, 3 for April to June, 1 for June. A month with no activity is a present row with empty amounts. A debit is negative. The year's request sent twice gave byte-identical answers (not kept twice).
  - **The Cash Flow rows tie to the native trial balance on this book, inflows only:** for each window the sum of the months' `DSPDRAMTA` equalled the sum of the debits of the ledgers under Cash-in-Hand and Bank Accounts read by `trial_balance` for the same window (17,975,981.22 for the year, 7,263,013.22 for April to June, 5,472,348.20 for June); no credit existed to compare, so the credit column is untested. The trial balance responses themselves are not kept.
  - Negative Ledgers: an empty `<ENVELOPE>` on a book that holds no negative-signed ledger. This is **inconclusive**: an empty envelope cannot be told from a report that was not rendered (§12a.11). Its row shape was unseen on that date (seen on 2026-10-09: see the last section).
  - An unknown report name: `STATUS` 0 and a `LINEERROR` "Could not find Report '<name>'!", HTTP 200, no dialog. (§12a.1 measured a bare `RESPONSE` "Unknown Request" on another build.)
- **Not established (as of 2026-10-06; the 2026-10-07 captures below add credits, a month with both columns and a Bank OD ledger):** any outflow, a contra, Bank OD, a month with both an inflow and an outflow, a window that is not whole months, a multi-currency book, optional or post-dated vouchers, Education mode, and any book other than this one.

| file | bytes | sha256 | content |
|---|---|---|---|
| `builtin_cash_flow_apr_jun_request.utf16le.xml` | 762 | `1a8e6ba391bd612ce88a924d6c19d512bd9b4d4e81899a486fca11a079e91689` | request, Cash Flow, 2025-04-01 to 2025-06-30 |
| `builtin_cash_flow_fy_request.utf16le.xml` | 762 | `a387a28928f851708f34e2ea4b1ec88a23ed0bf92a17d8b38ac0a250d1707278` | request, Cash Flow, 2025-04-01 to 2026-03-31 |
| `builtin_cash_flow_june_request.utf16le.xml` | 762 | `da616fec98f0f953a356c7afd52b49daf147b2e8a1258435d4baadc5a38d6885` | request, Cash Flow, 2025-06-01 to 2025-06-30 |
| `builtin_cash_flow_probe_b_apr_jun_live.utf16le.xml` | 1520 | `4a5530cd001287d645b3231244078826b9f0e1ce5a7ff9e51379acea5573e8bc` | Cash Flow, 3 month rows |
| `builtin_cash_flow_probe_b_fy_live.utf16le.xml` | 5710 | `be7c76364e524f769b2de12dc73bd6644c9caecbc3d864a4b08103135cafd619` | Cash Flow, 12 month rows, April to August with amounts |
| `builtin_cash_flow_probe_b_june_live.utf16le.xml` | 544 | `109e63deda80daff3ebb04924352ea67272869365477c62286932c09f7978ac8` | Cash Flow, 1 month row |
| `builtin_negative_ledgers_fy_request.utf16le.xml` | 776 | `441226581421515195c92e06810a3b6556d60de004209bab2026dcad5153cb60` | request, Negative Ledgers, 2025-04-01 to 2026-03-31 |
| `builtin_negative_ledgers_probe_b_fy_empty_live.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | Negative Ledgers, empty envelope (inconclusive) |
| `builtin_unknown_report_refusal_live.utf16le.xml` | 448 | `010d3144c2483d9147bb576a3b88314ff0d6434645bcbea5822bfcaa8ef43dc6` | Tally's refusal of that name |
| `builtin_unknown_report_request.utf16le.xml` | 798 | `232ba9c3a753133eb24912a803d63d79ad1fd22aa6b45cd54717a1e1a73ca7ae` | request, a report name Tally does not have |
| `builtin_negative_ledgers_bridge_lab_win_fy_live.utf16le.xml` | 1018 | `b838c49fe1d4a80876ba63f74e6c975e781deb201e7b455b4098545e05d16d5d` | Negative Ledgers, 2026-04-01 to 2027-03-31, two ledgers listed (see the section at the end) |
| `builtin_negative_ledgers_bridge_lab_win_fy_request.utf16le.xml` | 772 | `301671dafe3802202b24515a62c90001658b86089c41cba48ce4098f9287ce68` | request, Negative Ledgers, 2026-04-01 to 2027-03-31, company `Bridge Lab Win 0.5.1` |
| `builtin_funds_flow_fy_request.utf16le.xml` | 764 | `e4aab2996d71aafdbb7df82ca8fb99dce8440f53d007cab61766d6c8d272c883` | request, Funds Flow, 2025-04-01 to 2026-03-31 |
| `builtin_funds_flow_probe_b_fy_live.utf16le.xml` | 6196 | `fd328be1820b65d9e9e3a501b88a7bce823f16c387a531bc8486bf5dd829ad4d` | Funds Flow, 12 month rows, the only Funds Flow window captured (read by `native_funds_flow`) |
| `builtin_negative_stock_shape_lab_fy_live.utf16le.xml` | 2392 | `de7e67b5d1ef8d86a85179f1814333e91cba7bd07938d02bc3b0bdb22b4f7031` | Negative Stock, 5 items (read by `native_negative_stock`) |
| `builtin_negative_stock_shape_lab_fy_request.utf16le.xml` | 760 | `f054cc20c12679f65c6daaf29abb00a03ce665252bc60758fe95741fe99e5ecc` | request, Negative Stock, 2025-04-01 to 2026-03-31, on the stock company |
| `builtin_ratio_analysis_fy_request.utf16le.xml` | 772 | `c47d3a745d75821e39d918867c05a226cd19de062bd31fdb550ec9a096df3103` | request, Ratio Analysis, 2025-04-01 to 2026-03-31 |
| `builtin_ratio_analysis_probe_b_fy_live.utf16le.xml` | 3636 | `bd2b6621da370413ea602929e4e8bf760a243fc55cedb911c11739134a3dd635` | Ratio Analysis, 23 name and value pairs (not read by any parser) |
| `builtin_sales_register_fy_request.utf16le.xml` | 772 | `71ac0fb926d28e10423edd7f10df646bedb74fe63545b40500f3f9d6ac341ae1` | request, Sales Register, 2025-04-01 to 2026-03-31 |
| `builtin_sales_register_probe_b_fy_live.utf16le.xml` | 5888 | `61981e3281f26d1d9fab6199089dd78b5bc5ecbbc1ed6641b899413f8d22c80b` | Sales Register, 12 month rows (the grammar `native_month_register` reads; not read by a test) |

## Added 2026-10-07: Cash Flow on three more synthetic books

- **Date and instance:** 2026-10-07, attended, one request at a time, licensed TallyPrime 7.1 Silver (`education_mode=false`). Every answer was HTTP 200 with `/status` healthy after it.
- **Sender:** a lab script (`curl`, not `bridge_mcp`) sending a plain envelope (`TALLYREQUEST` Export, `TYPE` Data, `ID` Cash Flow, `SVCURRENTCOMPANY`, `SVFROMDATE`, `SVTODATE`), UTF-16LE with a BOM. The request files are not committed: the sent bytes were pinned by SHA-256 before sending (the table), and the three Cash Flow request files above carry the product's own rendering. Responses are **BOM-less UTF-16LE**, exactly as received.
- **What the captures show:**
  - `BRIDGE AMEND LAB`, April to September 2026 (6 rows): the debit, credit and closing columns, with two months of credit only (debit empty), one month with both, and three months with every amount empty. On every row with an amount the closing equals the debit plus the credit (a debit negative).
  - `BRIDGE SHAPE LAB`, 2025-04-01 to 2026-03-31 (12 rows): two months with both columns (one closing positive), a month of credit only and nine with every amount empty. The same equation holds on every row with an amount.
  - `BRIDGE CORPUS DENSE` (about 30,000 vouchers), the same year: 12 rows, all 36 amounts empty, 5,502 bytes in 0.1 s. The lab's earlier reading of that book is that it has no cash or bank activity; this answer alone cannot tell that from a report that did not compute, and its timing is a no-activity lower bound only.
  - The net total (the sum of every closing) tied to the debit plus credit totals of the ledgers under Cash-in-Hand and Bank Accounts for `BRIDGE AMEND LAB` (Cash and one bank ledger). For `BRIDGE SHAPE LAB` it tied only when a ledger of the Bank OD A/c group was counted as well (one ledger, a debit only); the ledgers under Cash-in-Hand and Bank Accounts alone differ from Tally's net by exactly that ledger's net. On both books the Cash Flow's debit column and credit column each differed from the debit and credit totals of the ledgers Tally counts as cash and bank (on SHAPE LAB including the Bank OD ledger; without it the two differences are not equal) by one common amount in size (the ledgers' columns larger in both) (a transfer between two of the money ledgers is the leading but untested reading), while the net tied. The trial balance figures are recorded readings of `trial_balance` for the same window (the responses are kept privately, not committed); the tests type the rows from them.
- **Not established:** a Bank OD A/c credit, a Bank OCC A/c ledger, what the two columns each include, a financial-year crossing, optional or post-dated vouchers, foreign-currency vouchers (`BRIDGE SHAPE LAB` holds two currency masters, and the tool's several-currency refusal was not exercised by this capture), a large book with cash activity. The timings of 2026-10-07 and the status probes are the capture script's log, not committed.

| file | bytes | sha256 | content | sent-bytes sha256 |
| --- | --- | --- | --- | --- |
| `builtin_cash_flow_amend_lab_apr_sep_live.utf16le.xml` | 2876 | `9c0e4946165e784d00987713a0b1d81651191eb78621c305e064c13600bd08e0` | Cash Flow, 6 month rows, credits present | `eb8c282b7f35bdfe204e37f46db8aae9ccd521bd77220593d20bdffd3018d266` |
| `builtin_cash_flow_shape_lab_fy_live.utf16le.xml` | 5622 | `f384c5e7b2bfaa4affc8e8941d2e11f8d724a0563cc0f2626812ef42f17ebf3c` | Cash Flow, 12 month rows, both columns and a positive closing | `3e462f1cfa31b3ca68ee9efbf4c663cb9331918bfec2a156633f82122454664a` |
| `builtin_cash_flow_corpus_dense_fy_empty_live.utf16le.xml` | 5502 | `54667ddd7f613d50c11c3a97b06a57c78eec7b9105878463769c97851c75801a` | Cash Flow, 12 month rows, every amount empty | `d518f28d13549a353b4f9d4de57d844801b1a95b31f1647cd498266bb40ab43d` |

## Added 2026-10-13: the month-wise Sales and Purchase Registers of BRIDGE PILOT LAB

- **Date and instance:** 2026-10-13, 08:17 to 08:18 UTC, one request at a time, TallyPrime 7.1 Silver on Windows, one synthetic company loaded (`BRIDGE PILOT LAB`). Every answer was HTTP 200 with `CONTENT-LENGTH` equal to the bytes on disk, `RESPSTATUS` 1 in the HTTP headers and `/status` healthy before and after; a marks read before the first and after the last request answered the same bytes both times. No dialog appeared at the Tally screen.
- **Sender:** a lab script (`curl`, not `bridge_mcp`) sending the request files tabled below, UTF-16LE with a BOM, whose text is the envelope `render_built_in_report_request` produces (a test asserts that equality for both). The answers and their send log are on the branch `lab/issue-reads-2026-10-13` (commit `9c75e2e2`, folder `1232/pilot`); each file here is byte-equal to the file there, and its SHA-256 equals the one the send log recorded.
- **What the captures show:** each answer has the Cash Flow's month-row grammar: 12 rows for 2026-04-01 to 2027-03-31, every month present. The Sales Register prints the month's sales in `DSPCRAMTA` (August 17638.50, September 210.20) and a running total in `DSPCLAMTA` for August and September only (17638.50, then 17848.70), the other months' `DSPCLAMTA` empty; the Purchase Register prints the month's purchases as a negative `DSPDRAMTA` (August -2100.00) and `DSPCLAMTA` for August and, unchanged, for September, which has no amount of its own; its other months are empty. The lab read those figures against the book's vouchers: August's sales equalled the invoice totals, tax included, of the month's Sales-class vouchers that are neither cancelled nor optional, September's the one Sales voucher of 15 Sep; August's purchases the two Purchases' total.
- **Not established:** equality with Tally's own register screens; credit and debit notes; a book with stock items; a window shorter than the year or crossing years; Receipt, Payment and other registers; other releases.

| file | bytes | sha256 | content |
| --- | ---: | --- | --- |
| `builtin_sales_register_pilot_lab_fy_request.utf16le.xml` | 760 | `6b07243bf22fd02207c11e5949eb0f5e670f3b438d58ee3325cf8ceed38d99f6` | request, Sales Register, 2026-04-01 to 2027-03-31, on the pilot lab |
| `builtin_sales_register_pilot_lab_fy_live.utf16le.xml` | 5562 | `03208b50260350059a55ac7db39acfa4662da89dc35fb357f06443196ad15861` | Sales Register, 12 month rows, two with amounts (read by `native_month_register`) |
| `builtin_purchase_register_pilot_lab_fy_request.utf16le.xml` | 766 | `84998b3748069d070f6d900427554e7131bfebe48592e7d1bb00418016364fb1` | request, Purchase Register, 2026-04-01 to 2027-03-31, on the pilot lab |
| `builtin_purchase_register_pilot_lab_fy_live.utf16le.xml` | 5550 | `dc3d8d424786913b6560e1bffa2217b3b03f307afb30e625b36c77de9bde45c5` | Purchase Register, 12 month rows, one with a month amount (read by `native_month_register`) |

## `builtin_negative_ledgers_bridge_lab_win_fy_*`: the first non-empty Negative Ledgers answer

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, Windows 10 Home Single Language 22H2, a lab instance; one request at a time, attended, `GET /status` after it answered HTTP 200 (51 bytes).
- **Date and book:** 2026-10-09, the synthetic company `Bridge Lab Win 0.5.1` (the only company loaded; voucher mark 13 before and after), window 2026-04-01 to 2027-03-31.
- **Sender:** a script run by an outside contributor, not `bridge_mcp`: the committed `builtin_negative_ledgers_fy_request` envelope with exactly three values changed (company and the two dates), UTF-16LE with a BOM, `Content-Type: text/xml; charset=utf-16`, one request, no retry. A test asserts that `render_native_negative_ledgers_request` produces these request bytes.
- **Answer:** 1,018 bytes, BOM-less UTF-16LE, `RESPSTATUS: 1`, the bytes as received. Two entries, each a `DSPACCNAME`/`DSPDISPNAME` then a `DSPACCINFO` with `DSPCLDRAMT/DSPCLDRAMTA` and `DSPCLCRAMT/DSPCLCRAMTA`: a creditor listed with a **debit** closing, carried as a negative `DSPCLDRAMTA` (`-2500.50`, the credit column empty), and a debtor with a **credit** closing, a positive `DSPCLCRAMTA` (`10000.00`, the debit column empty). The display fields are empty. A second request returned the same bytes (sha256 `b838c49f...` as above), reported by the contributor and not kept.
- **Checked against Tally's own screen and the trial balance of the same window, by the contributor, on this one book:** both entries matched the Negative Ledgers screen and the trial balance. A ledger under Suspense A/c with a credit balance (93,700.50) is **not** listed, on the screen and in the answer alike: it is not a general rule, the book is one book.
- **Not established:** why Tally lists a ledger (the rule behind the list), a window that is not a whole financial year, a book with GST, stock or several currencies, a large book (cost and size), optional or post-dated vouchers, any other release.

## `builtin_negative_ledgers_shape_lab_fy_*`: a Negative Ledgers answer of eleven ledgers (2026-10-21)

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, Windows, a lab instance, `education_mode=false`; one request at a time, curl, no retry, HTTP 200.
- **Book:** the synthetic company `BRIDGE SHAPE LAB` (loaded alone for the reads of this group), window 2025-04-01 to 2026-03-31. Its request is the committed Negative Ledgers envelope with only the company and the dates changed (UTF-16LE with a BOM); a test asserts that `render_native_negative_ledgers_request` produces it.
- **Answer:** 5,236 bytes in 0.12 s, BOM-less UTF-16LE as received (the lab's send log records sha256 `2945267f...`, equal to this file's). Eleven entries in the row shape of the first capture: `Bank of Baroda CA` with a credit closing; `Capital A/c`, `HDFC CC`, `Working Capital Loan` and the four Duties & Taxes ledgers `Cess`, `CGST`, `IGST` and `State Tax` with debit closings (negative `DSPCLDRAMTA`); and three `Shape Buyer` debtors with credit closings. Every entry has exactly one column filled, a debit negative and a credit positive, as in §12a.16's sign rule.
- **Not checked against:** Tally's screen or this book's trial balance (neither was read in this sitting), so whether each listed ledger sits on the side opposite its group, and whether a ledger Tally did not list sits on its usual side, is not established by this capture.
- **Not established:** why Tally lists a ledger, a window that is not a whole financial year, the cost on a large book, another release.

| file | bytes | sha256 | content |
| --- | ---: | --- | --- |
| `builtin_negative_ledgers_shape_lab_fy_request.utf16le.xml` | 764 | `8b54a2bfa6e5c330bc86264124f4aaf316f01046e54bfaa98aff86ffaa1ede28` | request, Negative Ledgers, 2025-04-01 to 2026-03-31, on the shape lab |
| `builtin_negative_ledgers_shape_lab_fy_live.utf16le.xml` | 5236 | `2945267f32e168ceb55a32bc569d5d6c862644870b7ccb3cb008858ed64b4f4c` | Negative Ledgers, eleven ledgers on both sides |

## Added 2026-10-21: Negative Stock on a book with a short item, to a mid-year date, and on a book with none

- **Date and instance:** 2026-10-21, TallyPrime 7.1 Silver on Windows, attended, one request at a time, HTTP 200 on every answer. Responses are BOM-less UTF-16LE exactly as received; the requests are UTF-16LE with a BOM.
- **Sender:** a lab script (not `bridge_mcp`) sending the request texts tabled below, which are the committed Negative Stock request with only the company and, for one, the end date changed (a test asserts equality with `render_native_negative_stock_request`).
- **Books:** `BRIDGE NEGSTOCK LAB 2` (synthetic; books from 2025-04-01, inventory on and integrated with accounts, unit Nos, two items with no opening stock; a sale of 5 Nos of `Lab Short Item` on 2025-05-10, a purchase of 10 Nos of `Lab Plain Item` on 2025-05-15 and a purchase of 2 Nos of `Lab Short Item` on 2025-06-20, all raw imports) and `BRIDGE TIEOUT C` (synthetic, read only). A company of a similar name, `BRIDGE NEGSTOCK LAB`, also exists on that instance: a read must not be bound by name alone.
- **What the captures show:**
  - 2025-04-01 to 2026-03-31 on `BRIDGE NEGSTOCK LAB 2`: one item row, `Lab Short Item`, closing quantity `-3 Nos`, rate `120.00`, amount `360.00` (positive). Tally's plain Stock Summary for the same window (answer kept by the lab, not committed) printed the same item as `-3 Nos`, `120.00`, `360.00`, and `Lab Plain Item` as `10 Nos`, `100.00`, `-1000.00`; the latter was not listed here.
  - 2025-04-01 to 2025-05-31 on the same book: one row, the same item, `-5 Nos`, with the rate and the amount present and empty. The quantity is as at the window's end, not the book's.
  - 2025-04-01 to 2026-03-31 on `BRIDGE TIEOUT C` (which holds stock items and no short one): exactly `<ENVELOPE></ENVELOPE>`, 46 bytes, the same bytes as the empty Negative Ledgers answer. An empty envelope cannot be told from a report that was not rendered (§12a.11), so it is read as not established, never as "no item is negative".
- **Not established:** that an empty answer means no item is below zero, what Tally lists beyond these books (§12a.16), a batch-wise or multi-godown book, a window crossing a financial year, the cost of the report on a large book, any other release.

| file | bytes | sha256 | content |
| --- | ---: | --- | --- |
| `builtin_negative_stock_negstock_lab_2_fy_request.utf16le.xml` | 770 | `4e29535d9d740e7ed89fbecad799c47f3ce0d35a59ad62b5830ae6366a12623e` | request, Negative Stock, 2025-04-01 to 2026-03-31 |
| `builtin_negative_stock_negstock_lab_2_fy_live.utf16le.xml` | 514 | `0b6fdf155016e065f669dff7e221108789c52d1900528c443f66ad608b1fc20e` | Negative Stock, one item at `-3 Nos` (read by `native_negative_stock`) |
| `builtin_negative_stock_negstock_lab_2_to_20250531_request.utf16le.xml` | 770 | `5e6811cef1ce108f736c17b420e6ae6ac42362f230161a54705d71640371b0d4` | request, Negative Stock, 2025-04-01 to 2025-05-31 |
| `builtin_negative_stock_negstock_lab_2_to_20250531_live.utf16le.xml` | 490 | `720cd2f5ecb09138b675d399df3dcca90fafb416a0709738d391ad44161a65c2` | Negative Stock to 31 May, one item at `-5 Nos`, rate and amount empty |
| `builtin_negative_stock_tieout_c_fy_request.utf16le.xml` | 758 | `6f9ed321654680017caaf3f49fbaa88f3491f9d283ba768579474c95d7017495` | request, Negative Stock, 2025-04-01 to 2026-03-31, on a book with no short item |
| `builtin_negative_stock_tieout_c_fy_empty_live.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | Negative Stock, an empty envelope |
