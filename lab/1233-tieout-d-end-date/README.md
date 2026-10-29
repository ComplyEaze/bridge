# Lab run sheet: stock items and the Stock Summary to a voucher's own date on BRIDGE TIEOUT D (#1233). SYNTHETIC COMPANY ONLY. READS ONLY. Not for merge.

About 20 minutes. `LAB-COMMON.md` in this folder (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`) applies in full; where this sheet and it differ, the stricter holds. Nothing is keyed, imported, backed up or restored. Only the Stock Summary's period is changed on Tally's screen, and it is set back.

**WHAT IT DECIDES.** `stock_summary` will admit an `as_of` at the end of any month (protocol reference, measurements part, §12a.13). Month ends are where closing journals are dated, so the end date's inclusiveness matters. Does the stock item collection, and the plain Stock Summary, include a voucher dated on `SVTODATE` itself? No end date read so far had a voucher on it. This book has a sale dated 15 Oct 2025 (its last entry), so a read to 15 Oct 2025 answers it. 15 Oct is not a month end; only the shape of the end date is measured here. If both sources include the sale, the month-end change states that the end date is inclusive. If either leaves it out, or the two disagree, the change keeps "a voucher dated on `as_of` is unmeasured" as a limitation, and the finding goes to the run coordinator.

**THE COMPANY.** `BRIDGE TIEOUT D`, GUID `6a9518b2-a88c-417c-a700-40243f9d9119`, read on 26 Oct 2026 by `lab/1233-stock-mid-year-requests` (answers `lab/1233-stock-mid-year-answers` at `ba3b55cf1bc5`). That sitting recorded its marks as ALTVCHID 4 and ALTMSTID 213, and one item, `Lab Gadget`. Its quantities on Tally's screen were 22 Nos for 1 Apr to 30 Sep 2025 and 17 Nos for 1 Apr to 15 Oct 2025. If step 1's marks are not 213 and 4: STOP and report (the book changed since then, and the expectation below no longer holds).

**THE REQUESTS** (in `requests/`; check each against `SHA256.txt` before its first send; a mismatch = STOP):
- `m-marks-D.utf16le.xml` (1,638 bytes): the company's flags, count and marks (`render_company_inventory_flags_request` for THE COMPANY). It is a byte copy of the request the 26 Oct sitting sent. READ BOUND 8,192 bytes.
- `t1-stock-items-to-20251015.utf16le.xml` (1,256 bytes): the item collection (`AuditStockItemsV1`) from 1 April 2025 to 15 October 2025.
- `t2-stock-summary-report-to-20251015.utf16le.xml` (756 bytes): the plain Stock Summary over the same period. A report request (TYPE Data): it defines no collection.
- Each `t` file is the 26 Oct sitting's 30 September request for THE COMPANY (`k1-stock-items-apr-sep` and `j-r4-stock-summary-apr-sep`). Only its end date, `20250930`, is replaced by `20251015` (one occurrence each; UTF-16LE with a byte-order mark kept). The pull request that admits month ends asserts the shape equal to the builder's own rendering. READ BOUND 65,536 bytes each.
- Both shapes were sent to this book on 26 Oct at other dates. Each first send here at this date is still treated as a NEW SHAPE under C3: take a screenshot of the whole Tally window before the next request.
- Request-ID check before the first send, in this folder: `python -I check-request-ids.py requests`. EXPECT `census: ok 2, skipped 1, FAIL 0`. The skip is `t2`: check by eye that its ID is `Stock Summary` and that it defines no `COLLECTION`. Anything else = STOP, nothing sent.
- Sent by curl under C3. No `bridge_mcp` call.

## Steps
0. C2 a, b and c with NO company loaded (a company already loaded = STOP). Load THE COMPANY alone by Select Company. C2 d: the MARKS read is `m-marks-D`. Record Tally's current date, the period and the last entry the Gateway shows.
1. Send `m-marks-D` (name `m-a`): one COMPANY row with THE COMPANY's GUID (C4, else STOP). EXPECT ALTMSTID 213, ALTVCHID 4, NUMSTOCKITEMS 1 (else STOP, above).
2. Send, one at a time, each saved as received with headers, size and sha256: `t1`, `t2`, then `t1` again (`t1r`), then `m-marks-D` again (`m-b`). Record by parser:
   - `t1`: the number of `STOCKITEM` rows (EXPECT 1) and that row's `NAME`, `CLOSINGBALANCE` and `CLOSINGVALUE` exactly as sent.
   - `t2`: every line, `DSPDISPNAME` (or `DSPACCNAME`'s name), `DSPCLQTY`, `DSPCLAMTA`. Does its `DSPCLAMTA` equal `t1`'s `CLOSINGVALUE` (exact decimals)? Record yes or no.
   - `t1r`: EXPECT byte-identical to `t1`.
   - `m-b`: ALTMSTID and ALTVCHID equal to `m-a`'s, else STOP.
3. **Screen truth**, after the last request (nothing is sent during this step): Gateway, Stock Summary; record the period it opens with. Alt+F2 to 1-4-2025 to 15-10-2025. Transcribe the item's line (quantity, unit, rate, value) twice, a minute apart, each from its own screenshot; the two must agree. Then Display, Day Book (or the voucher list the screen offers) for 15-10-2025: record each voucher of that day, its type and the item's quantity on it. Set the Stock Summary's period back to the one it opened with and return to the Gateway; record that the Gateway shows what it showed at step 0.
4. **Compare**: `t1`'s `CLOSINGBALANCE`, `t2`'s `DSPCLQTY` and both transcriptions, as number and unit. EXPECT 17 Nos on all four, which includes the 15 October sale. 22 Nos on either source means that source left out the voucher dated on its end date.
5. C10 close: Company > Shut; Select Company with none loaded; one tally.exe; 9001 LISTENING; GET /status 200.

**PASS (the sheet ran)** = every read answered whole and recorded as above, marks unchanged, the screen transcribed twice in agreement, the period set back. **FAIL** = a transport failure, a count by substring, a mark that moves during a read, or a setting not put back.

**THE FINDING it records**: whether the item collection and the plain Stock Summary each include a voucher dated on `SVTODATE`. It is a yes, a no, or a disagreement between them, read against Tally's own screen for the same period.

**NOT MEASURED:** a month-end date with a voucher on it, more than one voucher on the end date, a Stock Journal or a purchase on the end date, another book, TallyPrime 7.1.

**EVIDENCE** on a NEW branch `lab/1233-tieout-d-end-date-answers` (C11), after the pre-push scan and the run coordinator's yes. It holds every raw answer with headers, sizes and sha256, a send log, the parser's table, both transcriptions, the day's voucher list, the comparison and a notes file with UTC times. Screenshots stay on the lab PC, listed by name, bytes and sha256. No amount from the book goes into any text line of the evidence other than the raw answers and the parser's table.
