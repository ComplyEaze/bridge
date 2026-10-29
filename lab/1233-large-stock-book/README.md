# Lab run sheet: BRIDGE STOCK LAB past the single-read mark, and past 874 items, through #2180's build (#1233). SYNTHETIC COMPANY ONLY. WRITES, THEN RESTORED. Not for merge.

About 90 minutes. `LAB-COMMON.md` in this folder (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`) applies in full; where this sheet and it differ, the stricter holds. THE COMPANY is backed up first (BK0), stock items are imported into it, and BK0 is restored at the end. Nothing is keyed in Tally beyond the Backup and Restore screens and loading the company.

**THIS SHEET RUNS ONLY WITH #2180's BUILD, AND ONLY AFTER THE RUN COORDINATOR'S YES TO DOWNLOAD IT.** #2180 stays a draft until this sheet passes, and a draft pull request runs no bundle job. So its bundle comes from a run of `ci.yml` started by hand on #2180's branch (which runs every job on both systems), and a pull request's bundle is downloaded only on the run coordinator's yes. THE BUILD below names that run; nothing in this sheet is sent before the run coordinator's yes to download it.

**WHAT IT DECIDES.** `stock_summary` reads a company's stock items in one request and, until #2180, admitted it only when the master mark (`ALTMSTID`) was at most 874. #2180 counts the stock items and stock groups by AlterID span past that mark, and admits the same read when at most 874 items are counted. It refuses with the counted figures above that. Neither path has run against a live book: the only books in the lab with stock are under the mark. This sheet makes one past it, with items imported in a shape this book has taken before, and records three things:
1. **On the counted path** (mark past 874, at most 874 items), the call returns the book's items with `value_total_matched`, as many rows as Tally's own count. It sends one census slice per collection before the item read.
2. **Past 874 items**, the call refuses as `stock_summary_too_large`, with `size.counted_items` and `size.limit_items`. It refuses after the item census and before any group census, item or report request.
3. **How long the counts take.** The time of each call is set against the host's cut of about 60 seconds. Separately, the time of 200 census slice reads sent one after another is measured. That is the most a census of a 400,000 mark sends: 100 item slices and 100 group slices of 4,000 AlterIDs. On this small book it gives an estimate, not the time on a large one.

**THE COMPANY.** `BRIDGE STOCK LAB`, company number 100027, GUID `1edb9b05-8d35-4c0c-9959-dce655730463` (as listed in `scripts/merge-gate-synthetic-uuids.json`).
- **At the census sitting of 29 October 2026** (`lab/1233-stock-census-slice-answers`): `ALTMSTID` 292, `ALTVCHID` 4, `NUMSTOCKITEMS` 55, three stock groups, books from 1 April 2025.
- **The stock group and unit the imports use:** `Lab Raw Materials` and `U.`. Both exist on this book: 40 of its 55 items were imported under them on 2 October 2026, by "All Masters" imports with the same item fields.
- **If step 1 reads other marks or another count: STOP.** The book is not the one this sheet was written for.

**THE BUILD** (from the run started by hand on #2180's branch; it is downloaded only after the run coordinator's yes):
- Pull request #2180 at head `b8110b1f46fce66b4fa0474137250ca58a9b156d`. Workflow run `40942539219` (started by hand on the branch), artifact `unsigned-bundle-smoke-windows-latest` id `12929180854`, 57,348,809 bytes, zip digest `sha256:338f65948ad65804858779676f090ad6df0e210bbfeaf63772fe0d2e35590e4c` (it expires 5 Nov 2026 19:58 UTC). Check the downloaded zip's sha256 against that digest; another = STOP. `bridge_mcp.exe` sha256: recorded at step 0 from the unpacked zip.
- Hash the exe before the first call and after the last; another hash = STOP.
- Over MCP stdio, one launch, with a new empty `BRIDGE_AGENT_DATA_DIR` and `BRIDGE_TERMS_ACCEPTED=true`. Leave `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_IMPORT`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_REDACTION` unset: the tool writes nothing.
- Record the version the tool list reports, and that `list_companies`, `stock_summary` and `egress_log` are in it. Every call's answer is saved whole, with its sha256, and with the wall time from the call's send to its answer.
- **The egress log** (`egress_log`) keeps, for each of the last 32 requests, only its `seq`, its `kind` (`post` or `status`), its request and response bytes and `held_ms`; it names no collection or span, and its `omitted` count says how many earlier requests it no longer holds. A census slice is therefore told by its `request_bytes`: the stock-item and stock-group census requests differ in size from each other and from the stock-item read, the report and the extent reads. Record every entry of each call as given, and its `omitted`.
- **No tool call or request is made while a Backup, Restore or any other screen or dialog is open in Tally.**

**THE REQUESTS** (in `requests/`; check each against `SHA256.txt` before its first send; a mismatch = STOP).
- **How they were made.** Each is the UTF-16LE text, with a byte-order mark, that Bridge's own builders render at #2180's commit `b8110b1f`. A throwaway test called them with THE COMPANY's name and wrote their output; no byte was typed by hand.
- **`m-marks-S.utf16le.xml` (1,640 bytes):** THE COMPANY's flags, counts and marks (`render_company_inventory_flags_request`). It is byte-identical to the request of that name in `lab/1233-stock-census-slice`, which this book answered on 29 October. READ BOUND 8,192 bytes.
- **The imports, `i1a`, `i1b`, `i1c` (200 items each, 0001 to 0600, 118,666 bytes each) and `i2a`, `i2b` (150 each, 0601 to 0900, 89,166 bytes each):**
  - Each is one "All Masters" import of stock items named `Lab Census Item 0001` to `Lab Census Item 0900`. Each item is under `Lab Raw Materials`, base unit `U.`, with an opening of `1 U.` at `10.00/U.`, value `-10.00`.
  - They are rendered by the lab writer's own stock-item builder (`render_master_batch_xml` with `render_stock_item_xml`, `agent_lab_import.rs`). Like every request Bridge sends, the text is encoded as UTF-16LE with a byte-order mark; its XML declaration says UTF-8, as Bridge's import envelope always does.
  - This book took items with these fields on 2 October, in two imports of 20 under the same group and unit.
  - Each WRITES, and is sent ONCE in this item's life (C3). READ BOUND 65,536 bytes for each answer.
- **The census slices, `near-limit/s001` to `s200`:**
  - `s001` to `s100` are the stock-item census over (0, 4000], (4000, 8000] and so on to (396000, 400000]. `s101` to `s200` are the stock-group census over the same spans (`render_stock_item_census_slice_request` and `render_stock_group_census_slice_request`).
  - They are the plan the census makes for a mark of 400,000.
  - The two shapes were sent to this book on 29 October; these are other spans. READ BOUND 8,000,000 bytes each: a slice holds at most 4,000 rows, at most 1,992 bytes a row by the captured row's size.
- **Request-ID check before the first send**, in this folder: `python -I check-request-ids.py requests`. EXPECT `census: ok 201, skipped 5, FAIL 0`; the five skipped are the imports, which name no `TYPE` in their header (check by eye that each is `Import Data`, report `All Masters`). Anything else = STOP, nothing sent.
- **How they are sent:** by curl under C3, one at a time; the 200 slices by the lab's send script, in order, stopping at the first that fails C3. The tool calls go through the build above.

## Steps
0. **Start, and the build.**
   - C2 a, b and c with NO company loaded (a company already loaded = STOP).
   - Hash the exe. Launch the build, list the tools, and record the version.
   - Load THE COMPANY alone by Select Company. C2 d: the MARKS read is `m-marks-S`.
   - Record Tally's current date, the period and the last entry the Gateway shows, and this PC's own date: `as_of` is checked against the PC's date, not Tally's.
1. **The book as found.** Send, one at a time:
   - `m-marks-S` (name `m-a`). One COMPANY row with THE COMPANY's GUID (C4, else STOP). Record `ALTMSTID`, `ALTVCHID`, `NUMSTOCKITEMS`, `ISINVENTORYON`, `BOOKSFROM`. EXPECT 292, 4, 55, `Yes`, 20250401; else STOP (above).
   - `s001` (name `base-items`). EXPECT 55 `STOCKITEM` rows.
   - `s101` (name `base-groups`). EXPECT 3 `STOCKGROUP` rows.
   - These three answers are what step 8 compares the restored book with.
2. **The call as it is today** (T0). `list_companies`, then `stock_summary` with `company_guid` and `as_of` `20260331`, then `egress_log`.
   - EXPECT `result.state` `value_total_matched`, and `item_count_cross_check` `rows` 55 and `tally_count` 55. The mark is 292, so no census request is sent.
   - Record the call's wall time, `truncated` and the rows in the page, and every egress-log entry for the call.
3. **BK0**, after the run coordinator's yes (C5).
   - Use a NEW empty local folder `lab-1233-bk0`. Backup, `Specify Path`, THE COMPANY selected; move the list's highlight to `End of List` and press Enter (not Ctrl+A); `Accept?` Yes.
   - The folder then holds one file `TDBK1800_100027.001`. Record its name, bytes, time and sha256.
   - **IF THE FILE, ITS BYTES OR ITS SHA256 IS MISSING: STOP before import 1, and send nothing further.**
   - Then MARKS (`m-bk0`): EXPECT it byte-identical to `m-a`, or differing only inside `CMPINFO`. **Any other difference: STOP before import 1** (the backup would not be of the book step 8 compares with).
4. **Import 1.** After the run coordinator's yes, send `i1a`, then MARKS, then `i1b`, then MARKS, then `i1c`, then MARKS (`m-1a`, `m-1b`, `m-1c`).
   - Take a screenshot of the whole Tally window after each import, before the next request (C3).
   - Each import's answer: EXPECT `CREATED` 200, and `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` 0, with no `LINEERROR` (C4); else STOP.
   - After `i1a`: EXPECT `NUMSTOCKITEMS` 255 and `ALTMSTID` at least 492 (one or more per item created). Under 492 = STOP before `i1b`: the mark grows by less than one per item, and the sheet's sizes no longer hold.
   - After `i1c`: EXPECT `NUMSTOCKITEMS` 655, `ALTVCHID` 4, and `ALTMSTID` at least 875 (292 + 600 = 892 at one per item). Record it as M1.
   - An `ALTMSTID` under 875 = STOP: the book does not reach the counted path, and import 2 is not sent. The book then holds the imports already sent; it is restored (step 8) only on the run coordinator's word (C8).
5. **The counted path** (T1). `stock_summary` with `as_of` `20260331`, then `egress_log`.
   - EXPECT `result.state` `value_total_matched`, and `item_count_cross_check` `rows` 655 and `tally_count` 655. One page holds at most 500 items: record `truncated` and the rows in the page; the item count is judged on `item_count_cross_check`, not on the rows in the page.
   - EXPECT the egress log to show, after the flags reads and before the stock-item read, exactly two single `post` entries with no `status` between them: the stock-item census slice (0, M1] and the stock-group census slice (0, M1], told apart by `request_bytes`.
   - Record the call's wall time, the two census posts' `held_ms`, and the call's total against 60 seconds.
   - Then a screenshot of the whole Tally window.
   - A refusal, or a result that differs from these EXPECT lines, is recorded with its `code`, `cause` and `size` and goes to the run coordinator (C11) before step 6.
6. **Import 2.** After the run coordinator's yes, MARKS (`m-5`; EXPECT the marks of `m-1c`, else STOP), then send `i2a`, then MARKS, then `i2b`, then MARKS (`m-2a`, `m-2b`), each with a screenshot as in step 4.
   - Each answer: EXPECT `CREATED` 150 and the other counters 0; else STOP.
   - After `i2b`: EXPECT `NUMSTOCKITEMS` 955 and `ALTVCHID` 4. Record `ALTMSTID` as M2.
7. **Past 874 items, and the near-limit count.**
   - **T2:** `stock_summary` with `as_of` `20260331`, then `egress_log`.
     - EXPECT a refusal, `error.code` `stock_summary_too_large`, with `size.counted_items` 955 and `size.limit_items` 874.
     - EXPECT the egress log to show, after the flags reads, one single `post` (the stock-item census slice, by its `request_bytes`), then the extent read again (`post`, `status`, `post`, `status`: a refusal from the count is held until it), and nothing after that: no stock-group census slice, stock-item read or report.
     - Record the wall time, then a screenshot of the whole Tally window.
   - **The near-limit count:** send `s001` to `s200` in order by the lab's send script, each saved as received with headers, size, sha256 and its time from send to answer. The script sends `GET /status` after each slice and stops at the first that fails it or C3. Take a screenshot of the whole Tally window after `s001` (the first large slice), `s002`, `s100`, `s101`, `s102` and `s200`, before the next request.
     - EXPECT `s001` to hold 955 `STOCKITEM` rows and `s101` 3 `STOCKGROUP` rows, every other slice an empty collection, and every row's GUID beginning with THE COMPANY's GUID.
     - Record the sum of the 200 times, the slowest and the median. Each time includes the script's own start of curl, so the sum overstates Tally's part; set beside it the sum of T1's two census posts' `held_ms` (Bridge's own measure of the same request on this book).
     - The ESTIMATE of one call on a book of mark 400,000 on this PC is: T1's wall time, minus its two census posts' `held_ms`, plus 200 times the median slice time measured here. Record it against 60 seconds, as an estimate from a small book: each slice's time on a book with hundreds of thousands of masters is not measured.
   - Then MARKS (`m-7`): EXPECT the marks of `m-2b` (reads move no mark).
8. **Restore BK0**, after the run coordinator's yes (C6).
   - Write the restore plan with the backup file's folder, name, bytes and sha256 as recorded at step 3, and the two prompts expected. Shut THE COMPANY, restore, and load it alone.
   - Then send `m-marks-S` (`m-r`), `s001` (`r-items`) and `s101` (`r-groups`).
   - EXPECT `m-r` byte-identical to `m-a`, `r-items` byte-identical to `base-items`, and `r-groups` byte-identical to `base-groups`. A difference only inside `CMPINFO` is recorded, not a failure.
   - **IF ANY OTHER DIFFERENCE: STOP**, report "the restore check differs" with both sizes and the first differing offset, and send nothing further. The book is then not as found.
   - Then **T3:** `stock_summary` with `as_of` `20260331`: EXPECT `rows` 55 and `result.totals` equal to T0's (the GUID and marks reads cannot see a changed opening value).
9. C10 close. Company > Shut; Select Company with none loaded; one tally.exe; 9001 LISTENING; GET /status 200. Hash the exe.

**PASS (the sheet ran)** means all of these:
- BK0 is recorded;
- every write's counters are as expected;
- every read answered whole and was recorded as above;
- T0, T1 and T2 were recorded with their times;
- the restore check is byte-identical.

**FAIL** means any of these:
- a transport failure;
- a count by substring;
- a write sent twice;
- a mark that moves during the reads of one step;
- the restore check differs.

**THE FINDING it records**, on one synthetic book past the single-read mark:
- whether #2180's counted path returns the book's items as the single read does, with as many rows as Tally's count;
- whether a book past 874 items refuses with the counted figures, before any item request;
- how long one call takes with its census;
- an estimate, from this small book, of one call on a book of mark 400,000.

**NOT MEASURED:**
- the census on a book whose mark comes from many ledgers or alterations rather than items;
- a mark above about 1,300;
- the time of one census slice on a book with hundreds of thousands of masters;
- a report with long item names;
- stock items with no unit or value;
- any other book.

**EVIDENCE** on a NEW branch `lab/1233-large-stock-book-answers` (C11), after the pre-push scan and the run coordinator's yes. It holds:
- every raw answer with headers, sizes and sha256;
- every tool answer;
- the send log with times, the parser's tables and the timing table;
- the restore plan and the restore check;
- a notes file with UTC times.

Screenshots and the backup file stay on the lab PC, listed by name, bytes and sha256. No item name or amount from the book goes into any text line of the evidence other than the raw answers and the parser's tables.

**The pre-push scan and the span bounds.** The census slices' AlterID span bounds (multiples of 4,000, in the slice requests' names and text) coincide with figures the pre-push scan holds, so this sheet's own push named `SHA256.txt` and 28 of the slice request files as places where such a figure is accepted. The evidence branch's push names the same kind of places: its copies or listings of those requests, its send log and timing table where they repeat a bound, and any answer whose text carries one. A figure anywhere else is not accepted that way: it is reported.
