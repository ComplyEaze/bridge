# Row 2162-FY14: on a Sales voucher type with Manual numbering and Prevent duplicates Yes, does Tally decline an invoice number the book already holds in an earlier financial year?

Not for merge. Part of #2162 (ADR 0004, checklist item 14: a number used in an earlier financial year). Written 9 Nov 2026. Lines below are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
The build reads whether an invoice number is in use over **the invoice's own financial year** only: `financial_year_window` (`agent_import_invoice.rs:1892-1900`) gives 1 April to 31 March, and the number read (`render_invoice_number_request`, `agent_import_invoice_wire.rs:429`) filters on that window and the number. So a 2027-28 invoice numbered as one the book holds in 2026-27 finds no row and is admitted.

Tally's own guard is the series flag the build requires, `PREVENTDUPLICATES` Yes. Rows 2162-S4 and 2162-KD measured that it declines a number already held, in its answer, with no dialog, both times **within one financial year**. Neither says whether the flag looks across years. On `BRIDGE PILOT LAB`, `BRIDGE Sales` has one series, Default, Manual, `PREVENTDUPLICATES` Yes, and no restart entry (its `RESTARTFROMLIST.LIST` is present and empty in the committed answer `pilot-lab-voucher-types.utf16le.xml`).

**This row changes one variable: the number.** Two invoices dated 5 April 2027, in the next financial year, are sent to `BRIDGE Sales`:
- the control, `BP/26-27/0071`, a number the book does not hold in any year. It shows whether an invoice dated in the next year, after the Gateway's current date, is created at all;
- then `BP/26-27/0010`, the number the book holds on 1 August 2026, in 2026-27.

The two requests differ only in the number and the REMOTEID.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-KD's) applies in full. Where it and this sheet differ, the stricter one holds.
- Its state, from row 2162-KD's answers (`lab/2162-keyed-dup-answers`, 9 Nov 2026): restored to ALTVCHID 68 and ALTMSTID 245; 28 voucher types; current period 1-Apr-26 to 31-Mar-27; the Gateway's current date 15-Sep-2026. `BP/26-27/0010` is held once, dated 20260801 (row 2162-BC's `s2` read it). Neither `BP/26-27/0071` nor any voucher dated in 2027-28 is expected.
- One BK before the first write and one restore after the last (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen. KD read TallyPrime 7.2 Silver.
- **Do not change Tally's current date or period.** The invoices are dated after both, which is the case being measured.

**No Bridge build.** Raw requests, one at a time, through `run.py` (row 2162-KD's, byte-equal). The build cannot send this invoice, because Sales is not a qualified voucher type.

**The sends** are row 2162-KD's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count7.py`, which prints STOP and exits non-zero).

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2162-KD's file of the same name** (`lab/2162-keyed-dup-sheet` at c5b8e06e, folder `lab/2162-keyed-dup/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `T-types.xml`, `template-import-0010.utf16le.xml`, `template-n-0001.utf16le.xml` and `template-full-1.utf16le.xml`. KD's ROW.md says where each first came from; `T-types.xml` is the build's voucher-type read, `render_voucher_types_request` (`agent_import_invoice_wire.rs:267`).
- **`gen.py`** writes, from those files, and asserts every change occurs as often as expected:
  - `I-0071-y27.xml`: the build's request that created `BP/26-27/0010`, with the date `20270405` (three times), the number `BP/26-27/0071` (in `VOUCHERNUMBER` and `REFERENCE`) and the REMOTEID tail `0794`. Party (`BRIDGE Walk-in`), type (`BRIDGE Sales`) and figures unchanged;
  - `I-0010-y27.xml`: `I-0071-y27.xml` with the number `BP/26-27/0010` (twice) and the REMOTEID tail `0795`, nothing else;
  - `N-0010-y26.xml`: the number read for `BP/26-27/0010` over 20260401 to 20270331;
  - `N-0010-y27.xml`: the same over 20270401 to 20280331, the window the build reads for an invoice dated 20270405;
  - `N-0071-y27.xml`: `N-0010-y27.xml` for `BP/26-27/0071`;
  - `V-0010.xml`, `V-0071.xml`: every native field of the vouchers of that number from 20260401 to 20280331, built as KD's V file with the window widened to both years.
- **`count7.py`:** row 2162-KD's `count6.py` with three changes, each marked FY14: N and V print each row's `DATE`, and T prints each series' `DUPLICATECONTROL`, `BEGINNINGNUMBER` and how many children its `RESTARTFROMLIST.LIST` holds.

  When the sheet was written, on the committed answer `pilot-lab-voucher-types.utf16le.xml` as a T answer, it printed 25 rows; `BRIDGE Sales` 1, one series 'Default', 'Manual', `PREVENTDUPLICATES` 'Yes', `DUPLICATECONTROL` '', `BEGINNINGNUMBER` '1', and one `RESTARTFROMLIST.LIST` with 0 children.

**Checked when the sheet was written:** `I-0071-y27.xml` and `I-0010-y27.xml` are the same length and differ in 5 characters (the number's last two digits, twice, and the REMOTEID's last digit). Neither holds `20260801`. Each N file names its number once and its window's two dates twice each.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count7.py a-<label>-*`. A STOP from either script stops the row there.

**Before each write** (steps 4 and 5): `run.py URL s<n>m M-marks.xml`, then `python3 -I count7.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, send the write alone: `run.py URL s<n> <write>`.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Write down the current period and the Gateway's current date. If the date is on or after 5-Apr-2027, stop: the invoices would then not be post-dated, and that is another row's variable.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row with the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256.
2. **The book as found.** `s2`: `T-types.xml N-0010-y26.xml N-0010-y27.xml N-0071-y27.xml V-0010.xml V-0071.xml`.
   - EXPECT in T `BRIDGE Sales` 1 row, one series Default, Manual, `PREVENTDUPLICATES` Yes, and its `RESTARTFROMLIST.LIST` with no children; N-0010-y26 one row, `BP/26-27/0010`, DATE 20260801, `BRIDGEVCHISSALES` Yes; N-0010-y27 and N-0071-y27 0 rows; V-0010 one row, DATE 20260801, type `BRIDGE Sales`; V-0071 0 rows.
   - Anything else: record it and stop before any write. A restart entry in T, above all, changes the question.
3. **No step 3.** The row keys nothing on screen; the numbering keeps KD's step numbers.
4. **The control: a fresh number in 2027-28.** `s4m`, the check, then `s4`: `I-0071-y27.xml`. Screenshot the window. Then `s4r`: `N-0071-y27.xml V-0071.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`; N one row, `BP/26-27/0071`, DATE 20270405; V one row with that number, DATE 20270405 and type `BRIDGE Sales`.
   - A decline is the result "an invoice dated in the next year is declined": record its counters and every `LINEERROR`, send N and V once, send no other write, and go to step 6. The number question then stays open.
   - A V row with another date, number or type: record it and go to step 6.
5. **The number held in 2026-27, dated in 2027-28.** `s5m`, the check, then `s5`: `I-0010-y27.xml`. Screenshot the window. Then `s5r`: `N-0010-y26.xml N-0010-y27.xml V-0010.xml M-marks.xml`.
   - There is no EXPECT for the counters: this is the variable. Record every counter and every `LINEERROR`.
   - If declined: N-0010-y26 as at `s2`, N-0010-y27 0 rows, V-0010 one row as at `s2`, and M equal to `s4r`'s COMPANY row (`count7.py --same-row a-s4r-M-marks.xml a-s5r-M-marks.xml`).
   - If created: N-0010-y27 one row dated 20270405, V-0010 two rows, one in each year. Record both rows with every field `count7.py` prints.
6. **The restore**, if anything was created. C6, and row 2162-KD's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml N-0010-y27.xml N-0071-y27.xml`, and `count7.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, and both N 0 rows.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A dialog after step 5 would make the guard a failure inside Tally's screen rather than one in its answer (P7), and that decides the design before any count does. Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer.

## Record
- Each answer's bytes and sha256, and `count7.py`'s lines.
- The screenshot after each write, by name, bytes and sha256.
- The BK file, and every prompt of the restore word for word with the key that answered it.
- The release on Tally's screen, the current period, and the Gateway's date.

## Expected
Estimates, not predictions. No row has sent a number held in another year.
- **s4:** created as sent. Tally takes a voucher dated after its current period and current date by import; no row has measured this on `BRIDGE PILOT LAB`.
- **s5:** unknown. With no restart entry, the series is one run of numbers for every year, so a decline as at S4 and KD is the guess; a series checked per year would create it.

## Decides
- **s4 declined:** an invoice dated in the next year is not created on this book, whatever its number. Item 14's question then waits on a row whose control is created; report the decline.
- **s5 declined in the answer, no dialog:** Tally's flag spans years on a series with no restart, and the build's per-year read admits a number Tally declines. The failure is in-band (P7), but it comes after the approval, with nothing created. Item 14 becomes a build change: the number read covers every year the series runs (or the build refuses a number held in any year) before the approval is shown. Report the counters.
- **s5 created:** Tally keeps one number in two years on this type, as the build's per-year read already assumes. Item 14 needs no refusal for this case; record both V rows, and note that the book then holds two vouchers with one number.
- **A dialog after any write:** the row stops (see Stop).

## Code admits the trigger
Lines are at master ae38a75c.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends this invoice, and this row sends it raw.
- **The build's number read is per year.** `financial_year_window` (`agent_import_invoice.rs:1892-1900`) maps the invoice's date to 1 April to 31 March. The build's control and its post-time check both take that window (`:1114-1116`, `:2141`), and `render_invoice_number_request` (`agent_import_invoice_wire.rs:429`) filters `$Date` on it. `N-0010-y27` is that request for this invoice, byte for byte, but for the number and window gen.py names.
- **The flag the build reads.** `resolve_voucher_type` (`agent_import_invoice_wire.rs:329`) refuses a Sales type unless its one series reads Manual with `PREVENTDUPLICATES` Yes. T is its request, byte-equal. It does not read the restart list.
- **What the build sends.** `I-0010-y27` is the build's request for `0010` with the date, number and REMOTEID changed: what the build would send for a 2027-28 invoice numbered `BP/26-27/0010`.
- **The reads.** V is a lab read of every native field over both years; no build sends it.

## Not measured
A series with a restart entry; a number held in a later year sent dated in an earlier one; a number held by a cancelled voucher; another Sales type; an invoice built and posted through Bridge; an invoice dated before the current date in the next year; and any other book or release.

Row: lab/2162-fy14/
