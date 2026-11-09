# Row 2162-SC: does a company GSTIN whose state code is not its registration's state make Tally's GSTR-1 count the month as uncertain?

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 19: why the rehearsal's company read its registration as invalid). Written 9 Nov 2026. One master alteration on Tally's screen, read before and after, then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
Section 9.16 (the bullet on where GSTR-1 places a posted invoice) records that on the rehearsal's company, Tally's GSTR-1 counted every voucher of the month as uncertain. It gave the company's GST registration details as invalid or not specified. That company is not a book we hold. One candidate cause is a company GSTIN whose first two digits name another state than the registration's own state. That can be isolated on a book we hold as one variable.
- **The one variable:** the GSTIN of BRIDGE PILOT LAB's one GST registration, `Rajasthan Registration`, from `08ZZZZZ0000Z1ZQ` (state code 08, Rajasthan) to `27ZZZZZ0000Z1ZQ` (state code 27, Maharashtra). Both are of the synthetic family, and both pass the public check digit: the build's own `gstin_valid` gives Q for each, and its unit tests hold `27ZZZZZ0000Z1ZQ` as valid. The registration's state stays Rajasthan, and nothing else on the screen is changed.
- **What is read:** GSTR-1 for 2026-08 before and after, as a screen read. Also the build's company registration read (U), the read-back of `BP/26-27/0010` (R), the month read (W) and the marks (M), before and after the change, after the GSTR-1 look, and after the restore.
- **Not asked here:** what the build does with that registration. The code already refuses it (below); this row is about Tally.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-KC's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count13.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M kind prints whether the COMPANY row's GUID is the book's, its U kind whether each tax unit's GUID begins with it, and `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before anything is keyed unless s1's COMPANY row is the book's.
- Its state, from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **BK is this row's way back,** taken at step 1 before anything is keyed, and restored at step 8 by C6.
- Write down the release shown on Tally's screen. Row 2162-S13a2 read TallyPrime 7.2 Silver.

**No Bridge build.** Raw reads, one at a time, through `run.py`. The change is made on Tally's screen, by hand; no request writes.

**The sends** are row 2162-KC's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, or an answer that does not parse (`count13.py`, which prints STOP and exits non-zero). A W answer of more than 300 rows also stops `count13.py`, which then prints only the count; the answer is kept. Check each file's sha256 against `SHA256.txt` before its send. Count every answer with `count13.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files
- **Byte-equal to row 2162-KC's file of the same name** (`lab/2162-kc-sheet` at 604bd4e0, folder `lab/2162-kc/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` (the build's read-back for `BP/26-27/0010`) and `W-2026-08.xml` (the build's month read for 2026-08).
- **`U-taxunits.xml`:** the build's company registration read, `render_company_registration_request` (`agent_import_invoice_wire.rs:694` at master ae38a75c), for `BRIDGE PILOT LAB`, in UTF-16LE with a BOM. It is 934 bytes, and its sha256 begins `c763f19d`, equal to the request `r16-tax-units.xml` that answered the `pilot-lab-tax-units` fixture on this book (that fixture's PROVENANCE line). It is not a new shape.
- **`count13.py`:** row 2162-KC's `count11.py` with two changes, each marked SC:
  - a kind U, which prints each tax unit's name, type, GSTIN attribute, GUID (and whether it is the book's), `GSTREGNUMBER`, `GSTOLDREGNUMBER`, `USEDFOR` and its dated rows;
  - `--unit-diff A B`, which prints, unit by unit, every attribute and leaf that differs between two U answers, at most 200 lines.
  - Tried on the `pilot-lab-tax-units` fixture, it prints the two units with the Rajasthan row. On a copy with the GSTIN replaced, it prints two differing leaves, `GSTREGNUMBER` and `@TAXREGISTRATION`; on the answer against itself, none.
- Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count13.py a-<label>-*`. A STOP from either script stops the row there.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date, and write it down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2162-sc\backup-1`. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml R-0010.xml W-2026-08.xml M-marks.xml`.
   - EXPECT U two TAXUNIT rows, each the book's: `Default Tax Unit`, and `Rajasthan Registration` with TAXTYPE `GST`, TAXREGISTRATION and GSTREGNUMBER `08ZZZZZ0000Z1ZQ`, GSTOLDREGNUMBER empty, and one dated row `20260401`, Rajasthan, Rajasthan, Regular, not inactive. Any other: record it and stop before step 4.
   - EXPECT R-0010 one row: `BRIDGE Walk-in`, included Yes and every other status flag No.
   - Record every W row, and count the rows with BRIDGEVCHISSALES Yes.
3. **GSTR-1 before, read only.** Follow the screen rules below, then:
   1. At the Gateway (shot), press Alt+G (Go To) and type `GSTR-1`. Earlier rows found the List of Reports with no line highlighted: one Down put the highlight on `GSTR-1`, then Enter. Press Enter only on the line that reads `GSTR-1`. If Tally first shows a list of GST registrations, it should hold one line, `08ZZZZZ0000Z1ZQ` (Rajasthan): screenshot it and press Enter on that line. A list of more than one line is a stop. (shot)
   2. The period. Earlier rows found the report opening on 1-Aug-26 to 31-Aug-26, with a button bar offering `F2: Period`. If it opens on that period, press no period key. If it opens on any other, write down what it shows and the period keys its button bar offers, and stop.
   3. (shot) Write down, as text, the whole summary: Total Vouchers, Included in Return with each of its lines, Not Relevant for This Return, Uncertain Transactions, and every line of the Return View with its count and amounts.
   4. If Uncertain Transactions shows a count above 0, move to that line and press Enter once (shot). Write down the screen's title and every line it lists, as text, word for word. Press nothing else on that screen but Esc.
   5. Press Esc one screen at a time back to the Gateway (shot). Write down each screen passed.
4. **The change, on Tally's screen.** Before it, `s4m`: `M-marks.xml`, then `count13.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml`. It must pass; then no key is pressed in Tally but those below.
   1. At the Gateway (shot), press Alt+G and type `GST Registration`. Press Enter only on a line that alters a GST registration (its words name Alter and GST Registration). Write down every line the list offered. If no line reads so, or a line reads Create only, stop and ask.
   2. If a list of GST registrations is shown, it must hold one line, `Rajasthan Registration` (s2's U name). Press Enter on it (shot). More than one line, or another name: stop.
   3. The alteration screen (shot). Write down every field it shows with its value, as text. Find the field that holds `08ZZZZZ0000Z1ZQ`, with its label. No such field, or more than one: press nothing more, and stop (C7: leave by Ctrl+Q only on the coordinator's word, after a screenshot of the title).
   4. Move to that field with Enter or the arrow keys only (never Esc on this screen, C7). Clear it with exactly 15 Backspaces and type `27ZZZZZ0000Z1ZQ`. Zoom the field and check, before any Enter, that it reads `27ZZZZZ0000Z1ZQ`: 15 characters, capitals. A wrong character: Backspace and type again before any Enter.
   5. **Touch no other field.** The state must still read Rajasthan; write down what it reads. If Tally asks for a date from which the new number applies, or opens any field or screen not in step 3's list, press nothing more: screenshot, and stop (a dated change is a second variable).
   6. Accept with Ctrl+A (shot after). An `Accept ?` prompt is answered Yes. **Any other prompt or message** (for example about the GSTIN and the state, or about vouchers already entered): copy it word for word, screenshot, press no key, and stop (C7). That message is this row's result if it comes, and the coordinator decides what is pressed next.
   7. If Tally returns to a list, screenshot its title, then Ctrl+Q to the Gateway (shot). Write down every screen passed.
5. **The book after the change.** `s5`: `M-marks.xml U-taxunits.xml R-0010.xml W-2026-08.xml`.
   - `count13.py --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml`. Record the whole output. EXPECT, as an estimate: `GSTREGNUMBER` and `@TAXREGISTRATION` now `27ZZZZZ0000Z1ZQ`, the dated row unchanged, and `Default Tax Unit` unchanged. Record whether `GSTOLDREGNUMBER` now holds `08ZZZZZ0000Z1ZQ`, and every other leaf that moved.
   - Record ALTMSTID (one more than s4m's expected) and ALTVCHID (unchanged expected).
   - `count13.py --diff a-s2-R-0010.xml a-s5-R-0010.xml`. Record the whole output, and the three status flags as they now read.
   - Record W's rows against s2's.
6. **GSTR-1 after, read only.** Step 3 again, in full, with the same keys. At 3.1, write down whether a list of registrations shows, and which GSTIN it shows. Write down the whole summary. If Uncertain Transactions shows a count above 0, take 3.4 and write down every line word for word.
7. **The reads after the look.** `s7`: `M-marks.xml R-0010.xml`.
   - `count13.py --same-row a-s5-M-marks.xml a-s7-M-marks.xml`. EXPECT it passes: the look changed neither mark.
   - `count13.py --diff a-s5-R-0010.xml a-s7-R-0010.xml`. Record the whole output. This says whether opening GSTR-1 changed the status the voucher holds.
8. **Restore** BK by C6, with `restore-plan.txt` written first. Then `s8`: `M-marks.xml U-taxunits.xml R-0010.xml W-2026-08.xml`.
   - `count13.py --same-row a-s1-M-marks.xml a-s8-M-marks.xml`: EXPECT it passes.
   - `count13.py --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml`: EXPECT 0 leaves differ in each unit.
   - `count13.py --diff a-s2-R-0010.xml a-s8-R-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
   - EXPECT W's rows as at s2. Record each answer's sha256 against s2's.
   - Any check failing: send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

**The screen rules for steps 3 and 6:** no request is sent while a report screen is open, and nothing is keyed but what the step lists. Screenshot the whole window at each point marked (shot).
- **Stop rule:** a prompt, a screen not described here, or a button bar that would have to be used other than as named. Stop, press nothing more, screenshot, and ask.
- **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts or marks anything; on a list of uncertain transactions, Esc is the only key.
- **If any Enter opens a voucher or a master screen, press nothing at all, not even Esc** (C7), and stop. Esc in these steps is for report screens only.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but the two step 4.6 and C6 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A stop between steps 4 and 8 leaves the book altered: say so in the report.

## Record
- Each answer's bytes and sha256, and `count13.py`'s lines.
- The `--unit-diff`, `--diff`, `--before-write` and `--same-row` outputs, whole.
- Steps 3, 4 and 6's screenshots, listed by name, bytes and sha256. Also record, as text and word for word: the summaries, every list, the alteration screen's fields before the change, every key pressed, and every prompt or message with the key that answered it.
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date.

## Expected
Estimates, not predictions.
- **s2 and step 3:** U as the `pilot-lab-tax-units` fixture reads; R-0010 included; the summary giving no registration message, since row 2162-S13a2 opened GSTR-1 on this book and found its registration valid.
- **Step 4:** the screen takes the number. Whether Tally warns about the state code on the field or on the accept is not known; either is a result.
- **Step 6:** not estimated. This is the row's question.
- **s8:** equal to s2 and s1.

## Decides
- **Step 6 counts the month's vouchers as uncertain, giving the company's GST registration details as invalid or not specified:** a GSTIN whose state code is not its registration's state is enough to give the rehearsal's message, on this book and release (PARTIAL: one change, one sitting). Section 9.16's bullet can then name it as a cause that reproduces the message. Whether it was the rehearsal company's cause stays not established, since that book is not ours to read.
- **Step 6 as step 3 (no registration message):** the mismatch alone does not give that message here. The rehearsal's cause stays open, with this one candidate ruled out on this release.
- **Tally refuses the number at the field or on the accept:** a mismatched GSTIN cannot be saved this way on this release. Record the words. The rehearsal's company then reached its state some other way, or on another release.
- **s5's U shows `GSTOLDREGNUMBER` filled:** Tally keeps the earlier number on a change. The build already refuses any unit with that field filled (below), so every invoice of a company that ever changed its GSTIN is refused. That goes to the coordinator as a finding about the build's reach, not this row's question.
- **s5 or s7's R-0010 status flags differ from s2's:** a company registration change reaches the status a posted voucher holds, or the GSTR-1 look does. Record which, and report before the restore.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build reads the registration this row changes.** The invoice judge reads the company's tax units (`render_company_registration_request`, `agent_import_invoice_wire.rs:694`) and parses them with `parse_company_registration` (`:731`, called at `agent_import_invoice.rs:2338`).
- **It refuses the changed state of the book.** The unit in force must hold a valid GSTIN whose state code is its dated row's state, equal to its `TAXREGISTRATION`, with no `GSTOLDREGNUMBER` (`agent_import_invoice_wire.rs:812`). Otherwise it is `invoice_company_registration_inconsistent` (`:820`). So no invoice is built on the book while it is altered. This row measures Tally's report, not a guard of the build. If step 6 reproduces the message, the build already refuses the cause before any write.

## Not measured
- The rehearsal company itself.
- A GSTIN of the right state but a wrong check digit, or a blank GSTIN.
- A state changed and the GSTIN kept.
- A dated GSTIN change.
- A company with more than one registration.
- Any month but 2026-08, and any other book or release.
- An invoice built and posted through Bridge while the book is altered: the build refuses it (above).

Row: lab/2162-sc/
