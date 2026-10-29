# Lab run sheet: how many purchase groups Stock Query returns, where an item's batches come from, and what an empty answer means, on BRIDGE SHAPE LAB (#2018). SYNTHETIC COMPANY ONLY. READS ONLY. Not for merge.

About 35 minutes. `LAB-COMMON.md` in this folder (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`) applies in full; where this sheet and it differ, the stricter holds. Nothing is keyed, imported, backed up or restored, and no setting on Tally's screen is changed. Every request is either one a released Bridge tool sends or a byte copy of a request this book answered on 28 or 29 October 2026; no request shape is sent here for the first time.

**WHAT IT DECIDES.** A tool that reads stock by batch (#2018) will send Tally's `Stock Query` once per item. Its request and parser are in a protocol change for #2018 that has no pull request number yet. Before each send the tool must bound the answer's size, which is made of two parts:
- **The last-purchase and last-sale groups.** One item, Soda Ash Light, answered five purchase groups dated 11 to 15 April 2025. It is not recorded whether that item had more purchases, so it is not known whether Tally caps the groups (at five, or at all).
- **One line per godown and batch holding stock.** The book's marks and counts measured so far say nothing about how many batches an item has.

This sheet records:
1. **L1:** how many invoice-mode Purchase vouchers name Soda Ash Light in the year, against the purchase groups Stock Query returns for it.
2. **L2:** for each batch-kept item, the godowns and batches its invoice-mode vouchers' goods lines name in the year, and the opening-day and year-end lines Stock Query returns for it. This says whether the opening and the vouchers can bound an item's batch lines.
3. **L3:** whether Stock Query's empty answer stands for zero stock. Tally sends the same empty answer for an item it does not know, so a tool may read it as zero only where the stock-item collection's closing for the same item and date agrees. On 28 October Zero Stock Item answered empty at 1 April 2025 and Cleaning Kit B at the year end; the collection read on 29 October gave Zero Stock Item a closing of zero at 1 April and Cleaning Kit B no closing at all. This sheet reads both in one sitting.

**WHAT THE VOUCHERS READ CANNOT SEE.** `goods_lines` covers invoice vouchers only. A Stock Journal showed none in the one read of it; a Delivery Note, a Receipt Note and a purchase in accounting mode are not read. Stock moved by any of these is invisible to L1 and L2, so a line found in Stock Query and in no voucher is recorded as "from the opening or a source this sheet does not read", never as "from the opening".

**THE COMPANY.** `BRIDGE SHAPE LAB`, GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346` (as listed in `scripts/merge-gate-synthetic-uuids.json`), books from 1 April 2025. Other sittings have changed and restored this book since 28 October, so its marks are recorded, not expected. They must not move during this sheet.

**THE REQUESTS** (in `requests/`; check each against `SHA256.txt` before its first send; a mismatch = STOP). The raw sends below use the names in `requests/` without the `.utf16le.xml` suffix; on the lab branches they came from, `q-r-09` is `r-09`, and so on for each `q-` name.
- `m-marks-shape-lab.utf16le.xml` (1,640 bytes): THE COMPANY's flags, counts and marks, from `tests/fixtures/company_inventory_flags_request.utf16le.xml` (`render_company_inventory_flags_request`). READ BOUND 8,192 bytes. Sent twice, as `m-a` before the reads and `m-b` after them; the same file each time.
- Stock Query from 1 April 2025 to 31 March 2026, the requests sent on 28 October: `q-r-09` (Soda Ash Light), `q-r-03` (Cleaning Kit A) and `q-r-02` (Caustic Soda Flakes) from `lab/2018-stock-query-items-periods-answers` at `a5b369bf`, and `q-r-4` (Hydrochloric Acid 33pc) from `lab/2018-stock-query-with-item-answers` at `263cc72c`.
- Stock Query from 1 April 2025 to 1 April 2025, from `a5b369bf`: `q-r-12-stock-query-hcl-to-20250401` (Hydrochloric Acid 33pc) and `q-r-14-stock-query-cleaning-kit-a-to-20250401` (Cleaning Kit A).
- Stock Query from 1 April 2025, from `a5b369bf`: `q-r-13-stock-query-zero-stock-item-to-20250401` (Zero Stock Item, to 1 April 2025) and `q-r-04-stock-query-cleaning-kit-b` (Cleaning Kit B, to 31 March 2026).
- The stock-item collection (`render_audit_stock_items`, collection `Bridge Audit Stock Items`) from `lab/2018-stock-query-names-units-dates-answers` at `86ea5211`: `c-a-01-items-to-20250401` (to 1 April 2025) and `c-a-03-items-to-20260331` (to 31 March 2026), 1,258 bytes each. READ BOUND 262,144 bytes each.
- Each Stock Query request is a report request (TYPE Data) that defines no collection. READ BOUND 65,536 bytes each.
- Request-ID check before the first send, in this folder: `python -I check-request-ids.py requests`. EXPECT `census: ok 3, skipped 8, FAIL 0`; the three passed are `m-marks-shape-lab` and the two `c-` requests, and the eight skipped are the Stock Query requests (check by eye: ID `Stock Query`, no `COLLECTION`). Anything else = STOP, nothing sent.

**THE BUILD.** The build the lab already holds, at or after #2038 (which added `goods_lines` to `vouchers`). Record its version as the tool list reports it, and the exe's sha256 before the first call and after the last; another hash = STOP. MCP stdio, one launch, with a new empty `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_AGENT_REDACTION` unset (no names masked), and writes and imports unset. Every tool answer is saved whole with its sha256.

**TOOL-CALL SAFETY.**
- A tool call with no answer after 10 minutes is recorded and never sent again; the sheet stops there (C-rules close still runs).
- A tool answer above 2,000,000 bytes = STOP: record its size, keep the saved file, send nothing more.
- A refusal is recorded whole. A call is repeated once only when its refusal says repeating it is safe; a second refusal = STOP.

## Steps
0. C2 a, b and c with NO company loaded (a company already loaded = STOP). Load THE COMPANY alone by Select Company. C2 d: the MARKS read is `m-marks-shape-lab`. Record Tally's current date, the period and the last entry the Gateway shows.
1. Send `m-marks-shape-lab` (name `m-a`). One COMPANY row with THE COMPANY's GUID (C4, else STOP). Record `ALTMSTID`, `ALTVCHID`, `NUMSTOCKITEMS`, `ISBATCHWISEON`. EXPECT `ISBATCHWISEON` `Yes`; else STOP (the batch questions do not apply).
2. **Stock Query again.** Send `q-r-09`, `q-r-03`, `q-r-02`, `q-r-4`, `q-r-12-…`, `q-r-14-…`, `q-r-13-…` and `q-r-04-…`, one at a time, each saved as received with headers, size and sha256. For each:
   - record whether it is byte-identical to that request's answer on 28 October (`raw/<name>` on the lab branch named above);
   - record by parser the number of `STQPURCDATE` and `STQSALESDATE` elements, each group's date, and each `STQGODOWNNAME`, `STQBATCHNAME`, `STQGODOWNQTY` triple as sent.
   - A difference from the committed answer is recorded, not a STOP: the book has been changed and restored since.
   - Then send `c-a-01-items-to-20250401` and `c-a-03-items-to-20260331`, one at a time, each saved as received with headers, size and sha256. For Zero Stock Item, Cleaning Kit B and each item the Stock Query reads above name, record by parser its `CLOSINGBALANCE` element exactly as sent: absent, empty, or its text.
3. **The year's vouchers.**
   - Pre-check: in the `tools/list` answer, `vouchers`' input schema has a `goods_lines` property. Absent = STOP (the build is older than #2038).
   - `list_companies`, then `vouchers` with `company_guid`, `from` `2025-04-01`, `to` `2026-03-31` and `goods_lines` `true`.
   - Paging: while `truncated` is true, call again with the same arguments, `snapshot_id` set to the first page's `result.snapshot.id`, and `offset` = the previous page's `offset` + the number of vouchers it returned. After the last page, the vouchers returned over all pages must sum to `result.total`; else record it and STOP.
   - Record for each page: `result.state`, `result.reason` and `result.withheld_vouchers` (any withheld voucher is listed, not a STOP).
   - Record for each voucher: its date, voucher type, number, class, `cancelled` and `optional`.
   - Goods lines sit under each ledger entry's `goods_lines`. For each line record `stock_item`, `actual_quantity` as sent, and each row of its `batches` with the row's `godown` and `batch` values. A line with no `batches` array, or an empty one, is recorded as "no batches" (Tally sent no godown or batch split for it). Quantities are recorded as sent; Tally sends them unsigned, and their direction comes from the voucher's class (Purchase in, Sales out).
   - Record any `goods_lines_unreadable` with its reason.
   - Take a screenshot of Tally's screen after the last page.
4. MARKS again (`m-b`): `ALTMSTID` and `ALTVCHID` equal to `m-a`'s, else STOP.
5. Call `egress_log` and save its answer whole with its sha256.
6. C10 close: Company > Shut; Select Company with none loaded; one tally.exe; 9001 LISTENING; GET /status 200. Hash the exe.

**Record, from the tables (no new read):**
- **L1:** from the vouchers that are not cancelled and not optional and are of class Purchase, the number of goods lines for Soda Ash Light and their dates, against the number and dates of `q-r-09`'s purchase groups. The expected outcome, from the book's history, is five against five.
  - More lines than groups, with the groups being the latest: Stock Query returned the latest N, and N is recorded as measured on one item.
  - Equal: no cap is shown, and a cap stays unmeasured.
  - Fewer lines than groups: Stock Query's groups come from stock movements this read does not see; recorded as such.
- **L2:** for Soda Ash Light, Cleaning Kit A, Caustic Soda Flakes and Hydrochloric Acid 33pc:
  - each distinct (godown, batch) pair the year's goods lines name, from vouchers that are not cancelled and not optional, and the count of pairs;
  - for each pair, the signed sum of its quantities (Purchase in as plus, Sales out as minus), in the item's unit;
  - each (godown, batch) line Stock Query returned at the year end, and, for Cleaning Kit A and Hydrochloric Acid 33pc, on the opening day (`q-r-14-…`, `q-r-12-…`);
  - for those two items, whether opening line + the pair's signed sum equals the year-end line, pair by pair;
  - which year-end lines appear in no voucher and in no opening line (from a source this sheet does not read).

- **L3:** a table of item, date, Stock Query's answer (empty or its lines) and the collection's `CLOSINGBALANCE` as sent, for Zero Stock Item at 1 April 2025, Cleaning Kit B at 31 March 2026, and each item and date above that Stock Query answered with lines.
  - An empty answer beside a closing of zero (`0.000` and its unit) is the zero-stock case.
  - An empty answer beside an empty or absent closing is recorded as such: an item with no stock movement at all.
  - An empty answer beside a closing that is not zero, or lines beside a closing that is zero, is recorded as a disagreement.
  - For each item that answered with lines, whether its lines' sum equals the closing's quantity.

**PASS (the sheet ran)** = every read answered whole and was recorded as above, the paging sum held, and the marks did not move. **FAIL** = a transport failure, a count by substring, or a mark that moves during the reads.

**THE FINDING it records:** on one synthetic book, whether Stock Query's purchase groups are capped and at what count, and whether an item's batch lines can be bounded from its opening and its invoice vouchers' goods lines, and whether Stock Query's empty answer and the collection's zero closing agree for the same item and date.

**NOT MEASURED:** zero stock reached by sales or transfers within the period (Zero Stock Item is zero only at the opening), a book with many purchases of one item, an item with more than three batches, stock moved by Stock Journals, Delivery or Receipt Notes or accounting-mode purchases, a godown or batch count that would need a census, any new request shape (a batch collection, or an item's batch allocations; neither has a builder), another release.

**EVIDENCE** on a NEW branch `lab/2018-stock-query-groups-batches-answers` (C11), after the pre-push scan and the run coordinator's yes. It holds every raw answer with headers, sizes and sha256, every tool answer, the `egress_log` answer, the send log, the parser's tables, the L1 and L2 comparisons and a notes file with UTC times. Screenshots stay on the lab PC, listed by name, bytes and sha256. No party name, amount or item name from the book goes into any text line of the evidence other than the raw answers, the tool answers and the parser's tables.
