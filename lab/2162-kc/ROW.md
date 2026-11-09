# Row 2162-KC: what does the build's number read show of its control, of a control deleted on Tally's screen, and of a number another Sales type holds?

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 12: the duplicate-number read's control). Written 9 Nov 2026. Lines below are at #2423's head 047288d7, where the build reads the number a calendar month at a time; master ae38a75c, where this branch starts, does not hold that read yet. Confidence of everything here until the row runs: unverified.

## Question
Before a Sales invoice is built, ComplyEaze Bridge reads every voucher of each month of the span and refuses a number a Sales-class voucher already holds. "No voucher holds it" is believed only beside a control: an invoice this company is known to hold, which the same month's read must show by its number and date (`control_row_found`, `agent_import_invoice_wire.rs:595`). If the read does not show it, the build refuses `invoice_number_control_missing` and records what the answer held instead (`control_miss`, `:538`: how many Sales rows, and whether the control's GUID was seen and under which number, date and type). Item 12 owes the control's cases one variable each. This row measures the three that are Tally's, on raw requests, in three parts, each changing one variable:
- **A. The control and a second invoice.** One invoice is sent. Does the month's read show it as the build looks for it? Then a second is sent beside it. Does the read still show the first, unchanged, and the second?
- **B. The control deleted on Tally's screen.** The first invoice is deleted by a person, from its voucher screen. What does the same month's read show afterwards: no row, a row marked deleted, or the GUID under something else?
- **C. A number another Sales type holds.** A number is stored on a second Sales type keyed for this row (as row 2162-KD keyed it). Does the month's read show it as Sales class, which is what makes the build refuse it? Then the same number is sent on `BRIDGE Sales`, a write the build would refuse: does that type's own duplicate guard decline a number another type holds?

Each read is the build's month read (W, for 2026-08), the build's read-back (R) or every native field of one number (V), and the marks (M).

**Not in this row.** The other cases item 12 names are not Tally's variables, or need what this row has not got:
- **The first post on a company with no invoice sent, and a post after a release.** These are states of ComplyEaze Bridge's journal (`invoice_number_control`, `agent_import_ledger.rs:1205`). With no control standing, the post stands on the type's own guard and its read-back (`absence_without_control`, `agent_import_invoice.rs:1975`). On Tally's side they are a read that finds no holder (step 2 here) and the guard (rows 2162-S4 and 2162-KD). Run through ComplyEaze Bridge, they need a build that admits Sales (#2434).
- **A second build controlled by the first, through ComplyEaze Bridge.** Part A measures what that build would read; the build itself needs the same.
- **A company with no voucher (mark 0).** That needs a company that has never held a voucher. The lab's notes name no such synthetic company, and making one leaves a company folder that a restore of this book does not remove. Left to the run coordinator.
- **The cost of the read on a larger book** (rows S8 and S10).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-RS's and 2162-KD's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count11.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M kind prints whether the COMPANY row's GUID is the book's, and `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before any write unless s1's COMPANY row is the book's.
- Its state, from the lab's last notes on it: rows 2162-KD and 2162-FY14R each left it restored, its marks equal to their `s1` (ALTVCHID 68, ALTMSTID 245), with no `BRIDGE Sales Keyed` type and none of this row's numbers. Row 2162-RS, queued before this one, also ends with a restore.
- One BK before the first write, and one restore after the last read (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen. Row 2162-KD read TallyPrime Silver, `tally.exe` file version 1.1.7.2.

**No Bridge build.** Raw requests, one at a time, through `run.py` (row 2162-RS's, byte-equal). The build cannot send these invoices, because Sales is not a qualified voucher type on master.

**The sends** are row 2162-RS's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count11.py`, which prints STOP and exits non-zero). A W answer of more than 300 rows also stops `count11.py`, which then prints only the count; the answer is kept. Check each file's sha256 against `SHA256.txt` before its send, send each write once, and screenshot the whole Tally window after it. Count every answer with `count11.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

**Before each write**, sent as a request or keyed on screen: `run.py URL s<n>m M-marks.xml`, then `python3 -I count11.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, make the write.

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2162-RS's file of the same name** (`lab/2162-rs-sheet` at 073144e5, folder `lab/2162-rs/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml`, `template-full-1.utf16le.xml` and `template-import-0010.utf16le.xml`. Each but `R-0010.xml` is also byte-equal to row 2162-KD's. RS's ROW.md says where each first came from.
- **`T-types.xml`:** row 2162-KD's (`lab/2162-keyed-dup-sheet` at c5b8e06e), byte-equal: the build's voucher-type read.
- **`template-y-pilot.utf16le.xml`:** row 2162-S7's (`lab/2162-s7-sheet` at b63accd9), byte-equal: the build's window read of this book's year 2026-27.
- **`gen9.py`** writes, from those files, and asserts every change:
  - `I-0062.xml` and `I-0063.xml`: `template-import-0010` (the build's request that created `BP/26-27/0010`) with the number `BP/26-27/0062` or `BP/26-27/0063` (twice each) and the REMOTEID tails `0811` and `0812`. With that number and tail set back, `I-0062.xml` is row RS's `I-0057.xml` byte for byte (checked when the sheet was written).
  - `I-0064K.xml`: the same with `BP/26-27/0064`, the type `BRIDGE Sales Keyed` (VCHTYPE and VOUCHERTYPENAME) and the tail `0813`. With the number and tail set back it is row KD's `I-0061.xml` byte for byte (checked).
  - `I-0064.xml`: the same number on `BRIDGE Sales`, with the tail `0814`.
  - `R-0062.xml` and `R-0063.xml`: `R-0010.xml` with the number changed; `V-0062.xml`, `V-0063.xml` and `V-0064.xml`: `template-full-1` for this book over 2026-27, filtered to the number, as RS's `gen8.py` made `V-0057.xml`.
  - `W-2026-08.xml`: `template-y-pilot` with its dates set to 1 Aug 2026 and 31 Aug 2026. `gen9.py` asserts it equal to `render_invoice_window_numbers_request` (`agent_import_invoice_wire.rs:455`) for this book and that month, and when the sheet was written it was also checked against that function's text in the source.
  - No REMOTEID tail here is used by any earlier row's generator (checked against the request files of every `lab/` branch when the sheet was written).
- **`count11.py`:** row RS's `count10.py` with two changes, each marked KC:
  - a W kind: how many VOUCHER rows the month read holds, then each, sorted by number, with its DATE, VOUCHERNUMBER, VOUCHERTYPENAME, BRIDGEVCHISSALES, ISCANCELLED, ISDELETED, whether IRN holds text, and GUID;
  - `--guid GUID W...`: for each W answer, how many rows carry that GUID, and each such row.

  When the sheet was written, on the captured answers `pilot-lab-numbers-2026-09` and `pilot-lab-numbers-empty-month` (the protocol crate's pilot-lab fixtures), W printed one row (`BP/26-27/0020`, `BRIDGE Sales`, Yes, ISCANCELLED No, ISDELETED No) and 0 rows. `--guid` with that row's GUID printed 1 row and 0 rows; with a GUID one character short, 0 rows. On copies made for the check (not committed), 300 rows printed in full and 301 printed STOP and exited 1; an answer with no COLLECTION printed STOP and exited 1.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count11.py a-<label>-*`. A STOP from either script stops the row there.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date. If it is before 1-Aug-2026, stop: the invoices are dated 1 Aug 2026, and a post-dated invoice is another row's variable.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256. Then `s1bk`: `M-marks.xml`.
2. **The book as found.** `s2`: `T-types.xml W-2026-08.xml V-0062.xml V-0063.xml V-0064.xml`.
   - EXPECT in T `BRIDGE Sales` 1 row (Default, Manual, Yes) and `BRIDGE Sales Keyed` 0; 0 rows in each V; no W row numbered `BP/26-27/0062`, `0063` or `0064`. Record every W row.
   - Anything else: record it and stop before any write.

**Part A: the control and a second invoice.**

3. **The control.** `s3m`, the check, then `s3`: `I-0062.xml`. Screenshot the window. Then `s3r`: `W-2026-08.xml R-0062.xml V-0062.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`; R one row, V one row; ALTVCHID one more than s1's.
   - EXPECT W to hold s2's rows and one more: `BP/26-27/0062`, DATE 20260801, `BRIDGE Sales`, BRIDGEVCHISSALES Yes, ISCANCELLED No, ISDELETED No. Then `count11.py --guid <R's GUID> a-s3r-W-2026-08.xml`: EXPECT 1 row, that one. This row is the control as the build looks for it.
   - A decline, or any other answer: record its counters and every `LINEERROR`, and go to step 9.
4. **The second invoice.** s3r's W holds no `BP/26-27/0063`: the build's "not in use", beside the control found. `s4m`, the check, then `s4`: `I-0063.xml`. Screenshot the window. Then `s4r`: `W-2026-08.xml R-0063.xml V-0063.xml M-marks.xml`.
   - EXPECT CREATED 1 as at step 3; R and V one row each; W s3r's rows and one more, `BP/26-27/0063`, as `0062`'s fields.
   - `count11.py --guid <0062's GUID from s3r> a-s3r-W-2026-08.xml a-s4r-W-2026-08.xml`: EXPECT 1 row in each, with the same fields.
   - A decline, or any other answer: record it and go to step 9.

**Part B: the control deleted on Tally's screen.**

5. **The delete.** No request is sent during this step. Screenshot the whole window at each point marked (shot).
   - **Never press Escape on a voucher screen** (C7). The only prompts answered are the delete prompt named below. Any other prompt or dialog: press no key, screenshot it, copy its text exactly, and stop (C7).
   1. At the Gateway (shot), press Alt+G (Go To), type `Day Book`, and press Enter on the line that reads Day Book (shot).
   2. Set the period with Alt+F2 (Period): From `1-8-2026`, To `1-8-2026`, Enter. Do not use F2, which changes Tally's current date. (shot) Write down every line the Day Book lists, by its Vch No.
   3. Move with the Down arrow, one line at a time, until the highlighted line's Vch No. reads `BP/26-27/0062` (shot, showing that line highlighted). If no line reads it, press nothing more and stop.
   4. Press Enter once. EXPECT the voucher's alteration screen: its title names Alteration and Sales, and its number reads `BP/26-27/0062` (shot). Any other screen: press no key and stop.
   5. Press Alt+D once. EXPECT a prompt asking whether to delete (shot). Copy its text word for word. If it asks whether to delete this voucher, answer `y`. Any other prompt: press no key and stop.
   6. (shot) Write down which screen Tally shows, and every line the Day Book now lists, by its Vch No. If Tally shows a voucher screen instead, leave it with Ctrl+Q after a screenshot that shows its title (C7).
   7. On the Day Book, a report screen, press Esc once to return to the Gateway (shot). If Esc does not return to the Gateway, press nothing more and stop.
6. **The reads after the delete.** `s6`: `W-2026-08.xml R-0062.xml V-0062.xml V-0063.xml M-marks.xml`. Then wait 60 seconds by the clock, pressing nothing, and send `s6b`: `W-2026-08.xml M-marks.xml`.
   - `count11.py --guid <0062's GUID from s3r> a-s3r-W-2026-08.xml a-s6-W-2026-08.xml a-s6b-W-2026-08.xml`, and every W row of s6 and s6b as printed.
   - Nothing is expected here; these answers are Part B's result. The build takes a person's delete to give no row with that number and date read as Sales (`invoice_number_control_missing`).
   - V-0063 is read to show the other invoice untouched: EXPECT its one row as at s4r.

**Part C: a number another Sales type holds.**

7. **Key the second Sales type on screen.** This is the row's one master write. Row 2162-KD's step 3, word for word, with its findings: `s7m`, the check, as before a request.
   - From the Gateway: Create, then Voucher Type. Write down the menu path and keys used.
   - Set only these four fields, and leave every other field as Tally fills it: Name `BRIDGE Sales Keyed`; Select type of voucher `Sales`; Method of Voucher Numbering `Manual`; Prevent creating duplicate Voucher Nos. `Yes` (row KD found the field under this label, under Method of Voucher Numbering once Manual is chosen). If Tally does not offer it, stop before saving and ask.
   - Screenshot every part of the screen before saving, and write down every field it shows with its value, as text.
   - Save with Ctrl+A. If Tally asks "Accept ?", answer Yes; that is the master's own accept (C7). Any other prompt: stop, press nothing, and ask. Row KD's save showed no prompt.
   - If Tally then opens a new blank Voucher Type Creation screen, screenshot its title and leave it with Ctrl+Q (C7), as often as it takes to reach the Gateway, each after a screenshot of the screen's title. Screenshot the Gateway.
   - `s7r`: `T-types.xml M-marks.xml`. EXPECT in T one row named `BRIDGE Sales Keyed`, PARENT `Sales`, one series, Manual, `PREVENTDUPLICATES` Yes; `BRIDGE Sales` as at s2. In M ALTVCHID as at s6b and ALTMSTID above it.
   - Any other answer: record it, send no invoice, and go to step 9.
8. **The number on the keyed type, then on `BRIDGE Sales`.**
   1. `s8m`, the check, then `s8`: `I-0064K.xml`. Screenshot the window. Then `s8r`: `W-2026-08.xml V-0064.xml M-marks.xml`.
      - EXPECT CREATED 1 as at step 3; V one row, `BP/26-27/0064` on `BRIDGE Sales Keyed`; W s6b's rows and one more, `BP/26-27/0064`, `BRIDGE Sales Keyed`, BRIDGEVCHISSALES Yes. That row is what makes the build refuse `BP/26-27/0064` on `BRIDGE Sales` (`invoice_number_already_used`): any Sales-class holder counts.
      - A decline, or any other answer: record it and go to step 9.
   2. `s8bm`, the check, then `s8b`: `I-0064.xml`, the same number on `BRIDGE Sales`. The build would refuse to build this; it is sent only to measure the guard. Screenshot the window. Then `s8br`: `W-2026-08.xml V-0064.xml M-marks.xml`.
      - Nothing is expected. A decline in the answer (as rows S4 and KD saw: CREATED 0, EXCEPTIONS 1, no `LINEERROR`) means `BRIDGE Sales`' guard counts a number another type holds. CREATED 1, with two V rows, means it does not. Record the counters, every `LINEERROR`, and every V row.
9. **The restore**, if anything was created or keyed. C6, as rows 2162-RS and 2162-KD did, word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml T-types.xml W-2026-08.xml`, and `count11.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, T `BRIDGE Sales Keyed` 0, and W's rows as at s2.
10. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write or at any point of steps 5 and 7 that the step does not name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A dialog after step 8.2 would make the guard a failure inside Tally's screen rather than one in its answer (P7). Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer.

## Record
- Each answer's bytes and sha256, and `count11.py`'s lines, the `--guid` outputs whole.
- Steps 5 and 7's screenshots, listed by name, bytes and sha256; the Day Book's lines before and after the delete, the alteration screen and the voucher type screen written down as text; every prompt word for word with the key that answered it.
- The BK file, and every prompt of the backup and the restore word for word with the key that answered it.
- The release on Tally's screen, the Gateway's date, and the screenshot after each write, listed by name, bytes and sha256.

## Expected
Estimates from earlier rows, not predictions.
- **s3r and s4r:** stored as sent, as row RS's `0057` is expected to be and row BC's `0055` was; each W row as the captured `pilot-lab-numbers-2026-09` row reads.
- **s6:** not predicted. No earlier row has read the month after a delete.
- **s7r and s8r:** as row KD's s3r and s4r.
- **s8br:** not predicted. Row KD's "Not measured" names it.

## Decides
Each outcome is stated against #2423's head 047288d7.
- **s3r's W shows 0062 with its R GUID, and s4r's shows it unchanged beside 0063:** the build's control is found as it looks for it, and a second invoice beside it reads "not in use" until sent. Item 12's "a second build controlled by it" then stands on Tally's side, PARTIAL (one book, one release).
- **s3r's W does not show 0062 as the build looks for it** (another number, date or class, or no row): the control could never be found on this book, and every build after a first post would refuse `invoice_number_control_missing`. Report it before anything is written up.
- **s6 shows no row carrying 0062's GUID:** a person's delete removes the row from the read, and the build's refusal as missing fires as designed.
- **s6 shows a row with 0062's number and date, read as Sales, with ISDELETED Yes:** `control_row_found` reads no ISDELETED, so the build would take a deleted control as found. Report it before anything is written up.
- **s6 shows 0062's GUID under another number, date or type:** `control_miss` records it so; write down which.
- **s6 against s6b:** if they differ, Tally changed the read without a key pressed. The time between is the variable of a later row.
- **s8r's W shows 0064 on `BRIDGE Sales Keyed` as Sales:** the build refuses that number on `BRIDGE Sales`, whatever the guard does.
- **s8b declined:** a number another type holds is caught twice, by the build's read and by the type's guard. **s8b created:** only the build's read stands between them, and a build whose read matched nothing would let such a number through. Either is recorded with the guard's measured scope: one number, two types, one book.

## Code admits the trigger
Lines are at #2423's head 047288d7, except where named.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends these invoices, and this row sends them raw.
- **What the build renders.** `I-0062`, `I-0063` and `I-0064` are `template-import-0010` with the number and REMOTEID changed, as row RS's `I-0057` is; RS's sheet names the build function that wrote it.
- **The month read.** `render_invoice_window_numbers_request` (`agent_import_invoice_wire.rs:455`) is `W-2026-08`. The build reads each month of the span (`month_holding`, `agent_import_invoice.rs:1936`) and refuses a number any Sales-class row holds (`number_holding`, wire `:575`, and `agent_import_invoice.rs:2381`). It looks for the control in the control's own month's answer (`agent_import_invoice.rs:2407`); `control_row_found` (wire `:595`) checks the class, the number and the date, and no other field; `control_miss` (wire `:538`) records what the answer held instead.
- **The control.** `invoice_number_control` (`agent_import_ledger.rs:1205`) takes it from the journal: the latest verified invoice of the company, unless a person released it.

## Not measured
A delete by request, a cancelled invoice, a control in another month than the invoice built, a number held by a type of another class, a type with more than one series, the cases listed under "Not in this row", a build or post run through ComplyEaze Bridge, and any other book or release.

Row: lab/2162-kc/
