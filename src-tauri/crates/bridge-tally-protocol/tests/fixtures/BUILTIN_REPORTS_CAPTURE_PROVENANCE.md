# `builtin_cash_flow_*`, `builtin_negative_ledgers_*`, `builtin_unknown_report_*` — provenance

Live captures of Tally's own Cash Flow and Negative Ledgers reports, and of its answer to a report name it does not have, requested by report name (protocol reference §12a.1's envelope). Every response file is a live capture, never hand-written.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-10-06, 18:37–18:53 IST, one request at a time, attended. A status probe after every request returned HTTP 200 with a 51-byte body. No dialog appeared at the Tally screen.
- **Book:** `BRIDGE PROBE B SANDBOX`, a synthetic lab company (single INR currency, 25 ledgers, books from 2025-04-01). Its trial balance for 2025-04-01 to 2026-03-31 had Cash-in-Hand `Cash` (debit 5,500.00) and Bank Accounts `W1 Bank` (debit 17,970,481.22), no credit on either, and no ledger with a balance on the side opposite its group.
- **Sender:** a lab capture script, not `bridge_mcp`. It sent the exact request bytes tabled below (UTF-16LE with a BOM), whose text is the envelope `render_built_in_report_request` produces; a test asserts that equality. The requests were pinned by SHA-256 before they were sent.
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
