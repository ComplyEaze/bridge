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
- **Sender:** a lab capture script, not `bridge_mcp`. It sent the exact request bytes tabled below, one request at a time. Those bytes are the production renderers' own output: `every_request_is_byte_equal_to_its_committed_fixture` (`src/native_stock_summary_tests.rs`) asserts that each renderer's output equals its committed request, so the responses answer the request Bridge sends.
- **Encoding:** responses are **BOM-less UTF-16LE**, exactly as received. Requests are the exact bytes sent: **UTF-16LE with a BOM**. `.gitattributes` marks this tree `-text`.
- **Request shape:**
  - Stock items: the production `render_audit_stock_items` text (`AuditStockItemsV1`), period 2025-04-01 to 2026-03-31.
  - Stock Summary: the production `render_native_statement_request` envelope (§12a.1) with `<ID>Stock Summary</ID>`, over the same period.
- **What the captures show:**
  - Stock items:
    - 11 rows, every GUID carries the company's prefix;
    - closing quantity as `<number> <unit>`, some empty;
    - closing value as a plain signed decimal, some empty. On the wire a negative value is stock held and a positive one is what Tally's Stock Summary screen shows as negative (protocol reference §12a.13, measured on another synthetic company on 2026-10-01). Most of this book's values are positive on the wire, so this capture is mostly of values Tally's screen shows as negative; how they came to be entered that way is not recorded.
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
| `company_inventory_flags_request.utf16le.xml` | 1640 | `9af2b68b2f5373ccd685cf6bc4c9483d33bd959114012930498b1c442179a624` | request, company inventory flags (re-taken 2026-10-01, below) |
| `company_inventory_flags_shape_lab_live.utf16le.xml` | 4902 | `3bdc6c55ee4c20df7d8f6739926f20ba9469ca9c1fef776d3c17ee57b4c1dc10` | company inventory flags, 1 row |

## Captures taken through Bridge (2026-10-01)

- **Why:** the company inventory-flags request named its `$GUID` filter formula after an internal lab capture. It is now `BridgeCompanyGuidFilter`, which changes the request's bytes, so the request was captured again instead of being edited (bridge#979). The same run captured a company with no stock item.
- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-10-01, 14:50:24–14:50:51 IST.
- **Sender:** `bridge_mcp` itself (a debug build of the commit that renamed the formula), making one `stock_summary` call per company at `as_of` 20260331, through a recording relay that forwards each request unchanged and keeps the exact request and response bytes. Each call sent 30 requests (16 `POST`, 14 `GET /status`); every data request was sent twice (Bridge's paired read) and the two answers were equal.
  - `tally_status` was read before and after, and the loaded companies were the same.
  - The requests are therefore the bytes Bridge sends, not a script's copy of them.
- **Committed from this run:** only responses that name no other company. The company list and book-extent answers list every loaded company and are not committed.

**`BRIDGE SHAPE LAB` (the company of the captures above).** The tool answered `value_total_matched`: 11 rows, total `3000.01`, item count 11.
- The flags request as sent replaces the committed one (new formula name; 1640 bytes).
- Its response is byte-identical to the committed `company_inventory_flags_shape_lab_live.utf16le.xml`.
- The stock items and Stock Summary requests and responses were byte-identical to the four committed files above. So every stock capture in this file has now been reproduced through Bridge.

**`BRIDGE EMPTY BOOK`, a synthetic company with inventory on and no stock item.** The tool answered `no_stock_items`.
- Flags: one row; `ISINVENTORYON`, `ISINTEGRATED` and `ISBATCHWISEON` `Yes`; `NUMSTOCKITEMS` written `0`, with no leading space (a non-zero count has one).
- Stock items: `STATUS` 1 and a present, empty `COLLECTION` (three spaces between its tags).
- Stock Summary: `<ENVELOPE></ENVELOPE>` and a line end, nothing else.

| file | bytes | sha256 | content |
|---|---|---|---|
| `company_inventory_flags_request.utf16le.xml` | 1640 | `9af2b68b2f5373ccd685cf6bc4c9483d33bd959114012930498b1c442179a624` | request, flags, `BRIDGE SHAPE LAB` (before the rename: 1624 bytes, `e30ddb83ffbb0604046e1d13cc3d771983c6892cbb7bf2908350ebc2db59e869`) |
| `company_inventory_flags_empty_book_request.utf16le.xml` | 1642 | `9dea99a4a6a961d074cd485126869a3cde932e71f028c6bffd5c28a1d58fd993` | request, flags, `BRIDGE EMPTY BOOK` |
| `company_inventory_flags_empty_book_live.utf16le.xml` | 4804 | `ff2515cceac8b120f4283aff0f888f4f6bb87f848d47627142054343c36c79b1` | flags, 1 row, item count `0` |
| `stock_items_empty_book_fy_request.utf16le.xml` | 1260 | `ddba74f9c330b3b045a690690c70308a930748f4dae6bc90be585b6e9ee8d0e1` | request, stock items |
| `stock_items_empty_book_fy_live.utf16le.xml` | 2998 | `cf34fa0a514109d2691ae8ccd750fd83338927b5153541d55f810b0530658a1a` | stock items, 0 rows |
| `stock_summary_report_empty_book_fy_request.utf16le.xml` | 760 | `0b1dfbc8f1b7256f0dfac6d0e9e114dc856c7d69457f79cf2027b982651e16b2` | request, Stock Summary |
| `stock_summary_report_empty_book_fy_live.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | Stock Summary, empty envelope |

## A sale of an item that held no stock (captured 2026-10-07)

- **What:** the stock items collection and the plain Stock Summary read from one synthetic lab company after one sales item invoice (5 Nos at 25.00, dated 8 July 2025, no batch, no godown) was imported for an item whose closing balance read empty before it. The same two reads made before the sale showed that item with an empty balance and no report line; those two answers are not committed.
- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, a lab instance, one request at a time through a built `bridge_mcp` (the tool's own requests: `AuditStockItemsV1` over 20250401 to 20260331, and `Stock Summary`).
- **Edits (the only ones):** the company GUID prefix of every row GUID is replaced by `7f3c9a10-5b2d-4e6a-9c41-0d2e8b6a1f37` (same length), and the two item names' lab prefix is replaced by `Lab`. Everything else is the bytes Tally sent, BOM-less UTF-16LE, with the HTTP header removed. The sizes below are of the edited files.
- **What the captures show:** the item has `CLOSINGBALANCE` `-5 Nos` and an empty `CLOSINGVALUE`, and the report has a line for it with `DSPCLQTY` `-5 Nos` and an empty `DSPCLAMTA`; the other item reads ` 18 Nos` with `-186.00`, in both.

| file | bytes | sha256 | content |
|---|---|---|---|
| `stock_items_negative_sale_lab_live.utf16le.xml` | 5464 | `5ab16aff408fc8744211bebd6351ac09aec3ed3b8a5a28b624a8ba3ac4fc5646` | stock items, 2 rows |
| `stock_summary_report_negative_sale_lab_live.utf16le.xml` | 946 | `5d3a01b4a3a833b85283432c0cfed5aa88458a8bbec31ded395a67231717a2be` | Stock Summary, 2 lines |
