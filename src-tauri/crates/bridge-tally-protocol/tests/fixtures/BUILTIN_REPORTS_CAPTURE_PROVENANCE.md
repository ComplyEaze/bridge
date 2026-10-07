# `builtin_cash_flow_*`, `builtin_negative_ledgers_*`, `builtin_unknown_report_*` — provenance

Live captures of Tally's own Cash Flow, Funds Flow, Sales Register, Ratio Analysis, Negative Stock and Negative Ledgers reports, and of its answer to a report name it does not have, requested by report name (protocol reference §12a.1's envelope). Every response file is a live capture, never hand-written. Only Cash Flow, the empty Negative Ledgers answer and the unknown-name refusal are read by a parser and its tests; the Funds Flow, Sales Register, Ratio Analysis and Negative Stock files are kept as the evidence for protocol reference §12a.16, and no code reads them yet.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-10-06, 18:37–18:53 IST, one request at a time, attended. Every answer was HTTP 200, and a status probe after every request returned HTTP 200 with a 51-byte body. No dialog appeared at the Tally screen. The capture script's per-request timings are not kept here.
- **Book:** `BRIDGE PROBE B SANDBOX`, a synthetic lab company (single INR currency, 25 ledgers, books from 2025-04-01), for every file except the Negative Stock pair, which is `BRIDGE SHAPE LAB` (a synthetic lab company with 11 stock items and two currency masters, the same company as the stock fixtures). Its trial balance for 2025-04-01 to 2026-03-31 had Cash-in-Hand `Cash` (debit 5,500.00) and Bank Accounts `W1 Bank` (debit 17,970,481.22), no credit on either, and no ledger with a balance on the side opposite its group.
- **Sender:** a lab capture script, not `bridge_mcp`. It sent the exact request bytes tabled below (UTF-16LE with a BOM), whose text is the envelope `render_built_in_report_request` produces; a test asserts that equality for the three Cash Flow requests only (the other six request files are parked evidence for later slices). The requests were pinned by SHA-256 before they were sent.
- **Encoding:** responses are **BOM-less UTF-16LE**, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the captures show:**
  - Cash Flow: no `HEADER` or `STATUS`, and one `DSPPERIOD` (a month name, **no year**) followed by one `DSPACCINFO` (`DSPDRAMT/DSPDRAMTA`, `DSPCRAMT/DSPCRAMTA`, `DSPCLAMT/DSPCLAMTA`) per month: 12 for the year, 3 for April to June, 1 for June. A month with no activity is a present row with empty amounts. A debit is negative. The year's request sent twice gave byte-identical answers (not kept twice).
  - **The Cash Flow rows tie to the native trial balance on this book, inflows only:** for each window the sum of the months' `DSPDRAMTA` equalled the sum of the debits of the ledgers under Cash-in-Hand and Bank Accounts read by `trial_balance` for the same window (17,975,981.22 for the year, 7,263,013.22 for April to June, 5,472,348.20 for June); no credit existed to compare, so the credit column is untested. The trial balance responses themselves are not kept.
  - Negative Ledgers: an empty `<ENVELOPE>` on a book that holds no negative-signed ledger. This is **inconclusive**: an empty envelope cannot be told from a report that was not rendered (§12a.11). Its row shape is unseen.
  - An unknown report name: `STATUS` 0 and a `LINEERROR` "Could not find Report '<name>'!", HTTP 200, no dialog. (§12a.1 measured a bare `RESPONSE` "Unknown Request" on another build.)
- **Not established:** any outflow, a contra, Bank OD, a month with both an inflow and an outflow, a window that is not whole months, a multi-currency book, optional or post-dated vouchers, Education mode, and any book other than this one.

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
| `builtin_funds_flow_fy_request.utf16le.xml` | 764 | `e4aab2996d71aafdbb7df82ca8fb99dce8440f53d007cab61766d6c8d272c883` | request, Funds Flow, 2025-04-01 to 2026-03-31 |
| `builtin_funds_flow_probe_b_fy_live.utf16le.xml` | 6196 | `fd328be1820b65d9e9e3a501b88a7bce823f16c387a531bc8486bf5dd829ad4d` | Funds Flow, 12 month rows (not read by any parser) |
| `builtin_negative_stock_shape_lab_fy_live.utf16le.xml` | 2392 | `de7e67b5d1ef8d86a85179f1814333e91cba7bd07938d02bc3b0bdb22b4f7031` | Negative Stock, 5 items (read by `native_negative_stock`) |
| `builtin_negative_stock_shape_lab_fy_request.utf16le.xml` | 760 | `f054cc20c12679f65c6daaf29abb00a03ce665252bc60758fe95741fe99e5ecc` | request, Negative Stock, 2025-04-01 to 2026-03-31, on the stock company |
| `builtin_ratio_analysis_fy_request.utf16le.xml` | 772 | `c47d3a745d75821e39d918867c05a226cd19de062bd31fdb550ec9a096df3103` | request, Ratio Analysis, 2025-04-01 to 2026-03-31 |
| `builtin_ratio_analysis_probe_b_fy_live.utf16le.xml` | 3636 | `bd2b6621da370413ea602929e4e8bf760a243fc55cedb911c11739134a3dd635` | Ratio Analysis, 23 name and value pairs (not read by any parser) |
| `builtin_sales_register_fy_request.utf16le.xml` | 772 | `71ac0fb926d28e10423edd7f10df646bedb74fe63545b40500f3f9d6ac341ae1` | request, Sales Register, 2025-04-01 to 2026-03-31 |
| `builtin_sales_register_probe_b_fy_live.utf16le.xml` | 5888 | `61981e3281f26d1d9fab6199089dd78b5bc5ecbbc1ed6641b899413f8d22c80b` | Sales Register, 12 month rows (not read by any parser) |
