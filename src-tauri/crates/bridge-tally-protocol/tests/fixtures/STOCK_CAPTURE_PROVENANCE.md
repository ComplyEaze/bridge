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

## Stock groups nested three deep (captured 2026-10-27)

- **What:** the company's inventory flags and stock item count, the stock items collection and the plain Stock Summary of one synthetic lab company, `BRIDGE NESTED STOCK LAB`, keyed by hand on Tally's screens for this capture: two units, three stock groups nested three deep, eight stock items (one directly under the root, the rest at depths one to three) and two Stock Journals. Protocol reference §12a.13 records what they show against Tally's own screen.
- **Host / gateway:** TallyPrime **Silver**, 7.2, a lab instance on port 9001, one request at a time.
- **Sender:** curl, sending the bytes of each request file below once each. Those bytes are the production builders' output for this company and the period 20250401 to 20260331 (`the_nested_books_requests_are_the_builders_bytes_for_that_company` in `src/native_stock_summary_tests.rs` asserts it). The marks read before and after the reads were equal, and a second read of the stock items was byte-identical to the first.
- **Source:** lab branch `lab/1233-nested-groups-capture-answers` at `c1a82ddd16cb`: `raw/n1-a.xml`, `raw/n2-a.xml`, `raw/n3-b.xml` and the filled request `n3-company-flags.utf16le.xml`, with `requests/n1-*` and `requests/n2-*` from its request folder. Each sha256 below equals the lab's send-log line for that file.
- **Encoding:** responses are BOM-less UTF-16LE exactly as received, with the HTTP header removed; requests are the bytes sent, UTF-16LE with a BOM. Nothing is edited.

| file | bytes | sha256 | content |
|---|---|---|---|
| `company_inventory_flags_nested_lab_request.utf16le.xml` | 1654 | `bf5197c0ddac03788a1de135b1b99315b2fe612f6bc858e7e88307ae6b1b2640` | request, flags |
| `company_inventory_flags_nested_lab_live.utf16le.xml` | 4920 | `425bfe9724052a00034755b5463db70a446f8c0586db41e9b4c8e58370865b5c` | flags, 1 row, item count 8 |
| `stock_items_nested_lab_fy_request.utf16le.xml` | 1272 | `7c6770b2e18f146fd69c71d4f381bde63833366da99d69a0d54ef8e304d01b07` | request, stock items |
| `stock_items_nested_lab_fy_live.utf16le.xml` | 13954 | `ba77d8a0dc343b8eaf767803bbc4ee2cf5c4de2e22c4b0cee5a3527c3758fd41` | stock items, 8 rows at depths 0 to 3 |
| `stock_summary_report_nested_lab_fy_request.utf16le.xml` | 772 | `d7d0b8c17f3c93f2f656baf3ec51cc1d37aabd601d4677632465aa4e7810c3f0` | request, Stock Summary |
| `stock_summary_report_nested_lab_fy_live.utf16le.xml` | 948 | `338b20a3d4dbf9db10a9e6955a5d4f78b04440e10d1e3c1c2f1d36e0bd3b1937` | Stock Summary, 2 lines (the top group, the item under the root) |

## Period ends inside the year (captured 2026-10-26 and 2026-10-29)

- **What:** the stock items collection, the plain Stock Summary and the company's flags of two synthetic lab companies, read to a month end inside the financial year:
  - `BRIDGE TIEOUT D` (one item under the root), to 30 September and 31 May 2025;
  - `BRIDGE NESTED STOCK LAB` (the nested book above), to 31 August and 30 September 2025, either side of its Stock Journal of 15 September.
  Protocol reference §12a.13 records what they show against Tally's own screen.
- **Host / gateway:** TallyPrime **Silver**, 7.2, a lab instance on port 9001, one request at a time.
- **Sender:** curl, sending each request file below once. The requests are the production builders' output for each company and period, as `the_month_end_requests_are_the_builders_bytes_for_each_book_and_date` (`src/native_stock_summary_tests.rs`) asserts. The nested book's flags request is the committed `company_inventory_flags_nested_lab_request.utf16le.xml`. In each sitting, the marks read before and after the reads were byte-identical.
- **Source:**
  - `BRIDGE TIEOUT D`: lab branch `lab/1233-stock-mid-year-answers` at `ba3b55cf1bc5` (answers `01`, `02` to `05`; requests from `lab/1233-stock-mid-year-requests`).
  - The nested book: answers from `lab/1233-nested-as-of-answers` at `a2ff6c015524` (`evidence/a1`, `a2`, `b1`, `b2`, `n3-a`), requests from `lab/1233-nested-as-of` at `c42cfeff5497`.
  Each sha256 below equals the lab's send-log line for that file.
- **Encoding:** responses are BOM-less UTF-16LE exactly as received, with the HTTP header removed; requests are the bytes sent, UTF-16LE with a BOM. Nothing is edited. `stock_summary_report_nested_lab_to_20250930_live` is byte-identical to `stock_summary_report_nested_lab_fy_live`, because nothing in that book moves between 30 September and 31 March.

| file | bytes | sha256 | content |
|---|---|---|---|
| `company_inventory_flags_tieout_d_request.utf16le.xml` | 1638 | `19a29941331fec29e44557eb964bd50623d0b7307d9951d987defc03dd5e1905` | request |
| `company_inventory_flags_tieout_d_live.utf16le.xml` | 4886 | `84e0a5be2487af5d65029b3eb736c5fca7381de186d10bc3f0d7a9e29bf34104` | flags, 1 row |
| `stock_items_tieout_d_to_20250930_request.utf16le.xml` | 1256 | `45445f60dfd7f10c668b8a0748bf90256d004572778e3fcceaeaaa525a90603a` | request |
| `stock_items_tieout_d_to_20250930_live.utf16le.xml` | 4356 | `2a85a2a3487cad003696f3ca893c024a53bfc8a7ae6f53b10c194053d3cd2c4a` | stock items |
| `stock_summary_report_tieout_d_to_20250930_request.utf16le.xml` | 756 | `d621a687e49f41574c7d137f9a8d586d3a8027ceefa086cbf593a77ab654fccc` | request |
| `stock_summary_report_tieout_d_to_20250930_live.utf16le.xml` | 510 | `6ea369d19adb9d08bf3883d186744d6d705e625e21516d876df131f0881516a7` | Stock Summary |
| `stock_items_tieout_d_to_20250531_request.utf16le.xml` | 1256 | `47072bc7f8c1484953bde1e71d395d3611867c4800980a7f47e3836a4f02e384` | request |
| `stock_items_tieout_d_to_20250531_live.utf16le.xml` | 4356 | `c6414fcc1f628395d0a82cd474a548504655c77776799f9f707576cda996c3fc` | stock items |
| `stock_summary_report_tieout_d_to_20250531_request.utf16le.xml` | 756 | `db9f85b3a8aff078fba801f0fc8e1a6a37b68dd3841dee29c45536445e275c3e` | request |
| `stock_summary_report_tieout_d_to_20250531_live.utf16le.xml` | 510 | `64f67059e3a921e4c96cd7bf059b7397e18ddea71cd79b4b4f715da7b68efe71` | Stock Summary |
| `company_inventory_flags_nested_lab_as_of_live.utf16le.xml` | 4918 | `847d99be63bf40a84dc9378f38b02d57ddcb1fc74937dab0d7991be26166c083` | flags, 1 row |
| `stock_items_nested_lab_to_20250831_request.utf16le.xml` | 1272 | `8a527cab50a05dbbe5e12cd7c65ce42a3baa2fea9a7066558aa0cfe7963d3f63` | request |
| `stock_items_nested_lab_to_20250831_live.utf16le.xml` | 13952 | `1f012bcb94bb4fe3271590da85b42c6af0309daa6b83a1685201de94fb8928e4` | stock items |
| `stock_summary_report_nested_lab_to_20250831_request.utf16le.xml` | 772 | `b56c6bdc058f3c066116d11ace14b52381a9d31e22705b040c1c3fe9bf87353b` | request |
| `stock_summary_report_nested_lab_to_20250831_live.utf16le.xml` | 948 | `0ab20c1536d48bd75faacaf3ab45cea38ee6ba7e463e7fc443f7ca3eb553a365` | Stock Summary |
| `stock_items_nested_lab_to_20250930_request.utf16le.xml` | 1272 | `ac2818c36086ade4d9cfb28720f4d2b96b0b6f607278c5466bf076cc43bd70c6` | request |
| `stock_items_nested_lab_to_20250930_live.utf16le.xml` | 13952 | `53aa65c8e27e84aa9af0cffe29f2fd3679c7c44518ea89e8c90fbb59ed86e74e` | stock items |
| `stock_summary_report_nested_lab_to_20250930_request.utf16le.xml` | 772 | `cbdd7658148f83352f33d54560eab3c99fdab604b9c1460774686f5cbca01909` | request |
| `stock_summary_report_nested_lab_to_20250930_live.utf16le.xml` | 948 | `338b20a3d4dbf9db10a9e6955a5d4f78b04440e10d1e3c1c2f1d36e0bd3b1937` | Stock Summary |
