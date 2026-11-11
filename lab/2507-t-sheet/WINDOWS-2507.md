# #2507 rows T0 to T3: a Stock Journal transfer sent by XML, four shapes. Not for merge.

**Question.** Before any builder for a stock transfer (design A on #2507), what does Tally do with the one recorded Stock Journal import shape?
- T0 sends it again as the baseline, now as evidence and not as set-up.
- T1 to T3 each change one thing against T0.

The answers decide:
- what a builder writes;
- whether the read-back can find the voucher by the id it sent;
- whether Bridge must send the outward value.

**Why direct XML.** No Bridge builder renders a stock voucher (`src-tauri/src/agent_import.rs:330-337` knows five accounting types; `src-tauri/src/agent_lab_import.rs:2287-2297` renders accounting and invoice vouchers only). So each import is a lab request, as in row 2016, sent by curl under the owner's word allowing lab curl requests for shape discovery, writes included. Every read after it goes through Bridge's own tools.

**Code admits the trigger (reads only).**
- `vouchers` with `stock_lines`: `src-tauri/src/agent_read_profiles.rs:71-74`, builder `:214`.
- The four reserved names whose lines are read: `src-tauri/src/agent_voucher_parse.rs:356-361`.
- The line fields: `src-tauri/src/agent_voucher_goods_lines.rs:87-106`.

**Book.** `BRIDGE SHAPE LAB`, the synthetic book of rows 2230 and 2016, loaded alone:
- batches on;
- godowns Main Location and Factory Floor;
- TallyPrime 7.2, licensed.

No client book, and no other company loaded. The book's GUID is the one row 2016 recorded; any other GUID is a STOP at step 0.

**Build.** The lab's current master build of Bridge. Record `bridge_mcp.exe`'s sha256 before and after.

**Files** (byte-exact, `.gitattributes` `-text`; hashes in `SHA256.txt`). Every `.xml` is BOM-prefixed UTF-16LE, as row 2016 sent its seeds.
- **`t0.xml`:** row 2016's `reqs/v-sj.xml` (`lab/2016-stock-lines-answers`, `186c39d6`), with three changes: `DATE` and `EFFECTIVEDATE` 2026-05-21 in place of 2026-05-20, and `NARRATION` `Row 2507-T`. Nothing else changes. It holds:
  - one Stock Journal, `Consumption Voucher View`;
  - an `INVENTORYENTRIESIN.LIST` and an `INVENTORYENTRIESOUT.LIST` of 2 Kgs of Hydrochloric Acid 33pc, batch H-2504-A, from Main Location to Factory Floor;
  - a rate and amount on the out side only;
  - no `REMOTEID`, no voucher number, no ledger entry.
- **`t1.xml`:** `t0.xml` with `REMOTEID="lab-2507-t1-0001"` on the `VOUCHER` element.
- **`t2.xml`:** `t0.xml` with both lists renamed `ALLINVENTORYENTRIES.LIST`, children unchanged.
- **`t3.xml`:** `t0.xml` with the out side's `RATE` and both of its `AMOUNT` elements removed, from the entry and from its batch row.
- **`marks.xml`:** row 2016's `reqs/m0.xml`, unchanged: the company's `GUID`, `ALTVCHID` and `ALTMSTID`.

**Rules.**
- **Who acts:** the lab session does every step on the owner's standing words of 27 and 28 Oct 2026, as row 1780-W records them, including the backup and each restore on Tally's screen. Any step that needs the owner in person is a STOP that asks first.
- **Every curl request carries `--max-time 60`.** A timeout is a STOP. Take a capture of Tally's screen after every import and every restore, and keep it.
- **One import per row, apart from T1b,** each sent once, never retried. Keep each request as sent and each answer whole, as a file, with its headers.
- **Restore between rows.** After each row, restore backup B0, read `marks.xml`, and read `vouchers` for 2026-05-21. EXPECT the marks of step 0, byte-equal apart from read time, and a complete answer with 0 rows, before the next row starts.
- **A dialog on a capture, an import that times out, or a mark that moves outside a row's import is a STOP at step N, id X.** The run lead decides; nothing more is sent.
- **An answer with `ERRORS` or `EXCEPTIONS` above 0, or a `LINEERROR`, is a finding for that row, not a STOP** (T2 may be refused). Record it, read as the row says, restore, go on.
- **No figure from the book goes in public text** beyond what row 2016 already published. Keep the answers on the answers branch.

0. **Baseline.**
   - Take backup B0 on Tally's screen; record its size and sha256.
   - `list_companies`. EXPECT `BRIDGE SHAPE LAB` alone, with row 2016's GUID.
   - Read `marks.xml` by curl and keep it.
   - `vouchers` for 2026-05-20, as a control. EXPECT a complete answer with the vouchers row 2016 left there (at least one). RECORD the count.
   - `vouchers` for 2026-05-21, and for 2026-05-22 to 2026-05-31. EXPECT each a complete answer with no refusal and a total of 0 rows, else STOP: a day is not empty, or the read did not answer.
   - Find the item's GUID in Bridge's stock-item read, and record it.
   - `stock_by_batch` for that GUID, as of 2026-05-31, which equals 21 May's balance once 22 to 31 May are empty. EXPECT at least 2 Kgs of H-2504-A at Main Location, else STOP: T0 would be an over-issue, which is row T10's variable, not this sheet's.
1. **T0, the baseline.** Send `t0.xml`.
   - EXPECT `CREATED 1`, every other counter 0, no `LINEERROR`, else STOP: the baseline did not hold, and T1 to T3 have nothing to compare against.
   - Read `marks.xml`. EXPECT the voucher mark up by 1 and the master mark unmoved.
   - `vouchers` for 2026-05-21 with `stock_lines: true`. Keep the answer. EXPECT one Stock Journal with one `in` and one `out` line, item, quantities, godowns and batch as sent.
   - RECORD the row's GUID, ALTERID and MASTERID, and the in side's rate and amount as read (row 2016 read them empty).
   - On Tally's screen, open the voucher. RECORD the values the screen shows on each side.
   - Restore B0 and check the marks.
2. **T1, the id sent.** Send `t1.xml`.
   - EXPECT `CREATED 1`.
   - Read `marks.xml`, then `vouchers` for the day with `stock_lines: true`, as T0.
   - RECORD the marks, and the row's GUID, ALTERID and MASTERID. The stock-line read does not fetch `REMOTEID` (`src-tauri/src/agent_read_profiles.rs:71-74`), so its absence there says nothing. The question is whether the row's GUID equals the `REMOTEID` sent; no EXPECT.
   - **T1b:** send `t1.xml` again, unchanged, with no restore between. RECORD every counter, then read `marks.xml` and the day's `vouchers` with `stock_lines` again. No EXPECT. Read the outcome this way:
     - **a second voucher:** `CREATED 1`, voucher mark up by 1, two rows;
     - **the first altered:** `ALTERED 1`, one row with T1's GUID and MASTERID and a higher ALTERID;
     - **ignored:** `IGNORED 1` and nothing moved;
     - **refused:** `EXCEPTIONS` or `ERRORS` above 0 and nothing moved.
     Any other mix is recorded as it is. A dialog is a STOP.
   - Restore B0 and check the marks.
3. **T2, one list name for both sides.** Send `t2.xml`.
   - RECORD every counter and any `LINEERROR`.
   - If `CREATED 1`: read as T0, with T0's marks EXPECT, and RECORD each line's direction, godown and batch against T0's. If refused: read `marks.xml`, and EXPECT nothing moved.
   - Restore B0 and check the marks.
4. **T3, no outward value.** Send `t3.xml`.
   - RECORD every counter.
   - If `CREATED 1`: read as T0, with T0's marks EXPECT, and RECORD the out and in sides' rate and amount as read and as the screen shows them. The question is whether Tally values the transfer at the source godown's cost.
   - Restore B0 and check the marks.
5. **End.**
   - `list_companies` and `marks.xml`. EXPECT step 0's marks.
   - `vouchers` for 2026-05-21. EXPECT zero rows.
   - Record `bridge_mcp.exe`'s sha256.

**Fails** if:
- a Bridge call errors;
- a figure Bridge reads differs from Tally's screen;
- a line's direction contradicts the screen;
- a restore does not bring back step 0's marks.

**Not asked here:**
- an unknown item, godown or batch (T6 to T8), no batch rows (T9), an over-issue (T10), decimals (T11);
- a book with batches off (T12), a month-end date (T13), a book without integration (T14);
- any Bridge build of the shape (TB).

**Keep.**
- Each request as sent, and each answer with its headers.
- Every `marks.xml` answer.
- The tool answers of each step.
- The screen values recorded in steps 1 and 4.
- B0's size and sha256.
- The `bridge_mcp.exe` hashes.

Answers branch: `lab/2507-t-answers`.
