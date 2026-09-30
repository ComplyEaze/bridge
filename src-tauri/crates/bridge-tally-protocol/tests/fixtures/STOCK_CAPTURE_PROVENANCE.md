# `stock_items_*`, `stock_summary_report_*` — provenance

Two captures for the stock summary read:
- the stock items read with their closing quantity and value;
- Tally's built-in Stock Summary report for the same company and period.

Every file is a live capture, never hand-written.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-09-30, 19:22:14–19:22:17 IST, one request at a time.
  - `tally_status` was healthy before the first request and after every request.
  - The company was confirmed loaded before anything was sent.
- **Book:** `BRIDGE SHAPE LAB`, a synthetic lab company.
  - Its book extent (`ALTMSTID` 289, `ALTVCHID` 111) was read before and after the two requests, and was equal.
- **Encoding:** responses are **BOM-less UTF-16LE**, exactly as received. Requests are the exact bytes sent: **UTF-16LE with a BOM**. `.gitattributes` marks this tree `-text`.
- **Request shape:**
  - Stock items: the production `render_audit_stock_items` text (`AuditStockItemsV1`), period 2025-04-01 to 2026-03-31.
  - Stock Summary: the production `render_native_statement_request` envelope (§12a.1) with `<ID>Stock Summary</ID>`, over the same period.
- **What the captures show:**
  - Stock items:
    - 11 rows, every GUID carries the company's prefix;
    - closing quantity as `<number> <unit>`, some empty;
    - closing value as a plain signed decimal, some empty.
  - Stock Summary: no `HEADER`/`STATUS`, and 3 `DSPACCNAME`/`DSPSTKINFO` pairs (the top-level stock groups).
  - **The report's closing-amount total equals the sum of the stock items' `CLOSINGVALUE` exactly (3000.01).** The same tie held on a larger real book at the same period end (recorded privately, by role only).

**Company inventory flags (2026-09-30 19:51:37 IST, the same book, extent read before and after and equal).**
- **Request:** the production book-extent Company collection, with its `FETCH` extended by `ISINTEGRATED, ISINVENTORYON, ISBATCHWISEON` and seven `NUM*` counts, and one single-term filter `$GUID = "<the company's GUID>"`.
- **Why the filter:** a `Company` collection ignores `SVCURRENTCOMPANY` and returns every loaded company (§12a.7). The filter returned exactly this company's row. That is what makes the capture safe to keep.
- **Values:** `ISINTEGRATED` `Yes`, `ISINVENTORYON` `Yes`, `ISBATCHWISEON` `Yes`, and `NUMSTOCKITEMS` 11.
  - `NUMSTOCKITEMS`, `NUMGODOWNS` and `NUMUNITS` equal the rows in the captures above.
  - `NUMVOUCHERTYPES` (35) does not equal the voucher-type rows (26).

| file | bytes | sha256 | content |
|---|---|---|---|
| `stock_items_fy_request.utf16le.xml` | 1258 | `dacd47b0b96772f220b97abf3714650c760a98f4e54197b681ba35cd69c9edab` | request, stock items |
| `stock_items_shape_lab_fy_live.utf16le.xml` | 16904 | `7a76a52474b6b56357d687474d3f916f7a43618e3d57d5454cf11de0f9eea467` | stock items, 11 rows |
| `stock_summary_report_fy_request.utf16le.xml` | 758 | `17c4ff82b6ae49e6543f7be3745fbe023879ed0d2f47cc8c3bc3686e713afac8` | request, Stock Summary |
| `stock_summary_report_shape_lab_fy_live.utf16le.xml` | 1370 | `862c3206e5f22922cea932380da07d2170591085da7ef8227f4c911452bfe9a4` | Stock Summary, 3 rows |
| `company_inventory_flags_request.utf16le.xml` | 1624 | `e30ddb83ffbb0604046e1d13cc3d771983c6892cbb7bf2908350ebc2db59e869` | request, company inventory flags |
| `company_inventory_flags_shape_lab_live.utf16le.xml` | 4902 | `3bdc6c55ee4c20df7d8f6739926f20ba9469ca9c1fef776d3c17ee57b4c1dc10` | company inventory flags, 1 row |
