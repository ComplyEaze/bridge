# Row 2162-FY14R: on the same Sales type with a yearly restart of its numbering, does Tally still decline an invoice number the book holds in an earlier financial year?

Not for merge. Part of #2162 (ADR 0004, checklist item 14: a number used in an earlier financial year). Written 9 Nov 2026. Lines below are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
Row 2162-FY14 (`lab/2162-fy14-answers` at 35bceb81, one run, PARTIAL) measured this on `BRIDGE Sales` of `BRIDGE PILOT LAB`, which has one series, Default, Manual, `PREVENTDUPLICATES` Yes, and an empty `RESTARTFROMLIST.LIST`. There, `BP/26-27/0010`, held dated 20260801, sent dated 20270405, was declined in the answer: CREATED 0, EXCEPTIONS 1, no `LINEERROR`, no dialog. A fresh `BP/26-27/0071` dated 20270405 was created. So on a series with no restart, Tally's duplicate guard spans financial years.

Item 14's change widens the build's number read to every year of the books for such a series. Many real books restart their numbering every year. On a restarting series, Tally may instead check a number only within its own period. The build would then refuse a number Tally takes. The planned rule for a series whose restart list is not empty keeps today's read, the invoice's own financial year, and is unmeasured. This row measures it.

**This row changes one variable from row 2162-FY14: a yearly restart entry on the series.** Everything else is FY14's: the book, the type, the two requests, the reads and the order. The only addition is the restart, keyed on screen in step 3.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-FY14's) applies in full. Where it and this sheet differ, the stricter one holds.
- **Pinned by GUID.** The book is the company whose GUID begins `6b43e498`, held whole as `BOOK` in `count9.py` (the GUID rows 2162-KD and 2162-FY14 read). Step 1's M must print `GUID is the book's: yes`. Any other GUID, or more than one COMPANY row, = STOP before any write, whatever name the company shows. `--before-write` and `--same-row` STOP on any other GUID too.
- Its state, from row 2162-FY14's answers: restored to ALTVCHID 68 and ALTMSTID 245; current period 1-Apr-26 to 31-Mar-27; the Gateway's current date 15-Sep-2026; `BP/26-27/0010` held once, dated 20260801. Later rows on this book may have moved the marks; another value is recorded, not a STOP.
- One BK before the first write and one restore after the last (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- **Do not change Tally's current date or period.**

**No Bridge build and no approval window.** Raw requests, one at a time, through `run.py` (row 2162-FY14's, byte-equal), and one alteration keyed on screen.

**The sends** are row 2162-FY14's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count9.py`, which prints STOP and exits non-zero).

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its sha256.
- **Byte-equal to row 2162-FY14's file of the same name** (`lab/2162-fy14-sheet` at e5d30d23, folder `lab/2162-fy14/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `gen.py`, `M-marks.xml`, `T-types.xml`, the three templates, and the seven files `gen.py` writes: `I-0071-y27.xml`, `I-0010-y27.xml`, `N-0010-y26.xml`, `N-0010-y27.xml`, `N-0071-y27.xml`, `V-0010.xml` and `V-0071.xml`. FY14's ROW.md says what each is. FY14's run rebuilt the seven by `gen.py` byte for byte.
- **`count9.py`:** row 2162-FY14's `count7.py` with two changes, each marked FY14R: the GUID pin above, and T printing every non-empty leaf of each `RESTARTFROMLIST.LIST` by tag. When the sheet was written:
  - on the committed answer `pilot-lab-voucher-types.utf16le.xml` as a T answer, `--type "BRIDGE Sales"`, it printed one row, one series Default, Manual, `PREVENTDUPLICATES` 'Yes', and one `RESTARTFROMLIST.LIST` with 0 children and no leaf;
  - on two test marks answers (not kept), it printed `GUID is the book's: yes` for one with `BOOK` and `no` for one with another GUID. `--before-write` passed on the first and printed STOP, exit 1, on the second.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count9.py a-<label>-*`. A STOP from either script stops the row there.

**Before each write** (steps 3, 4 and 5): `run.py URL s<n>m M-marks.xml`, then `python3 -I count9.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, make the write: the alteration (step 3), or `run.py URL s<n> <write>` alone (steps 4 and 5).
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Write down the current period and the Gateway's current date. If the date is on or after 5-Apr-2027, stop.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row and `GUID is the book's: yes` (else STOP, above). Record ALTVCHID and ALTMSTID.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256.
2. **The book as found.** `s2`: `T-types.xml N-0010-y26.xml N-0010-y27.xml N-0071-y27.xml V-0010.xml V-0071.xml`.
   - EXPECT as row 2162-FY14's `s2`: in T `BRIDGE Sales` 1 row, one series Default, Manual, `PREVENTDUPLICATES` Yes, its `RESTARTFROMLIST.LIST` with no children; N-0010-y26 one row, `BP/26-27/0010`, DATE 20260801; N-0010-y27 and N-0071-y27 0 rows; V-0010 one row, DATE 20260801; V-0071 0 rows.
   - Anything else: record it and stop before any write.
3. **The restart, keyed on screen.** `s3m` and the check. From the Gateway: Alter, then Voucher Type, then `BRIDGE Sales`. Write down the menu path and every key used.
   - Leave the name, the type, Manual and "Prevent creating duplicate Voucher Nos." Yes as they are.
   - Find where Tally sets a restart of numbering for this type. Write down the field's label and every option it offers.
   - Add one restart entry: applicable from 1-Apr-2026, starting number 1, periodicity Yearly. If Tally fills a line already, record it as shown, then set it to these values.
   - If Tally offers no restart for a Manual series, or offers it only after the method is changed: change nothing, leave the screen by Ctrl+Q without saving, answer any "Quit?" prompt to discard, and go to step 6 with no write sent. That is a result: a Manual series cannot carry a restart on this release.
   - Screenshot every part of the screen, the restart sub-screen included, before saving, and write down every field it shows with its value, as text. Save by Ctrl+A. If a screen is left open after the save, leave it by Ctrl+Q, each press after a capture of the screen's title, until the Gateway shows. Record every prompt word for word with the key that answered it.
   - `s3r`: `T-types.xml M-marks.xml`. EXPECT in T `BRIDGE Sales` one row, one series, Manual, `PREVENTDUPLICATES` Yes, and its `RESTARTFROMLIST.LIST` now holding the entry: record every leaf `count9.py` prints. In M, ALTVCHID as in `s1` and ALTMSTID above it (row 2162-KD read a voucher type's save move it by 2).
   - If T shows no restart leaf, or the series reads other than Manual and Yes: record it, send no invoice, and go to step 6. The screen's setting is then not what the read returns.
4. **The control: a fresh number in 2027-28.** As row 2162-FY14's step 4: `s4m`, the check, then `s4`: `I-0071-y27.xml`. Screenshot the window. Then `s4r`: `N-0071-y27.xml V-0071.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`; N one row, `BP/26-27/0071`, DATE 20270405; V one row with that number, DATE 20270405 and type `BRIDGE Sales`.
   - A decline: record its counters and every `LINEERROR`, send N and V once, send no other write, and go to step 6. With the restart, an invoice dated in the next year is then declined whatever its number.
5. **The number held in 2026-27, dated in 2027-28.** As row 2162-FY14's step 5: `s5m`, the check, then `s5`: `I-0010-y27.xml`. Screenshot the window. Then `s5r`: `N-0010-y26.xml N-0010-y27.xml V-0010.xml M-marks.xml`.
   - There is no EXPECT for the counters: this is the variable. Record every counter and every `LINEERROR`.
   - If declined: N-0010-y26 as at `s2`, N-0010-y27 0 rows, V-0010 one row as at `s2`, and `count9.py --same-row a-s4r-M-marks.xml a-s5r-M-marks.xml` passing.
   - If created: N-0010-y27 one row dated 20270405, V-0010 two rows, one in each year. Record both rows with every field `count9.py` prints.
6. **The restore**, whatever steps 3 to 5 did. C6, and row 2162-KD's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml T-types.xml N-0010-y27.xml N-0071-y27.xml`, and `count9.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, T as at `s2` (the restart list empty), and both N 0 rows. If step 3 left by Ctrl+Q with nothing saved and `s3r` was not read, the restore is still made, and the same EXPECT holds.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer.

## Record
- Each answer's bytes and sha256, and `count9.py`'s lines.
- Step 3's screens: every field and value as text, the restart sub-screen's label and options, the screenshots by name, bytes and sha256, and every prompt word for word with the key that answered it.
- The screenshot after each write, by name, bytes and sha256.
- The BK file, and every prompt of the restore word for word with the key that answered it.
- The release on Tally's screen, the current period, and the Gateway's date.

## Expected
Estimates, not predictions. No row has keyed a restart.
- **s3:** unknown whether this release offers a restart under Manual numbering.
- **s4:** created as sent, as at row 2162-FY14.
- **s5:** unknown. A guard checked per restart period would create it; a guard on the whole series, as at FY14, would decline it.

## Decides
- **No restart offered under Manual:** a series the build admits (Manual, one series) cannot restart on this release. The build's restart rule then covers a state Tally's screen does not make. It still fails closed, but it rests on no observation. Report the screen.
- **s5 declined in the answer, no dialog:** a yearly restart does not scope the guard on this release. Then a restarting series is no different, and the build should read every year for it too. The planned rule of reading only the invoice's own year for a restarting series admits a number Tally declines. That still ends typed at the post, but it should change.
- **s5 created:** the restart scopes the guard to its period. Reading only the invoice's own year for a restarting series matches Tally, and widening it would refuse a number Tally takes. Record both V rows, and note that the book then holds two vouchers with one number.
- **A dialog after any write:** the row stops (see Stop).

## Code admits the trigger
Lines are at master ae38a75c.
- **The build reads the series but not its restart.** `resolve_voucher_type` (`agent_import_invoice_wire.rs:329`) admits a Sales type whose one series reads Manual with `PREVENTDUPLICATES` Yes. It reads no restart entry. T is its request, byte-equal, and the answer returns the series' `RESTARTFROMLIST.LIST` (empty in the committed pilot answer).
- **The build's number read is per year today.** `financial_year_window` (`agent_import_invoice.rs:1892-1900`); `N-0010-y27` is that read for this invoice, as at row 2162-FY14.
- **What the build sends.** `I-0010-y27` is the build's request for `0010` with the date, number and REMOTEID changed, as at row 2162-FY14.

## Not measured
A restart with another periodicity or date; a series with a prefix or a suffix; a number held in a later period sent dated in an earlier one; another Sales type; an invoice built and posted through Bridge; and any other book or release.

Row: lab/2162-fy14r/
