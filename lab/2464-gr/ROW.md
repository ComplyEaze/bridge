# Row 2464-GR: in a book with two GST registrations, which registration does Tally give an imported Sales invoice that names none, when its voucher type is bound to one?

Not for merge. For #2464 (0.9.0: issue a Sales invoice under the registration the voucher type is bound to), related to #2162 (ADR 0004, slice 1, the "Not changed" bullet) and #2467. Written 10 Nov 2026; revised the same day after the first sitting stopped at step 2 on the number (now `0091`), with nothing written. Two master changes on Tally's screen and one import, then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
The build's invoice names no registration: no `GSTREGISTRATION`, no `CMPGSTIN` (`render_sales_invoice_xml`, `src-tauri/src/agent_import_invoice.rs:2664` at master ae38a75c). On this book, with one registration, Tally filled them itself. The native read of the book after the 9 Oct import (the `pilot-lab-vouchers-after-post` fixture) gives the imported `BP/26-27/0010` (REMOTEID tail `0777`) `GSTREGISTRATION` `Rajasthan Registration`, `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, `CMPGSTSTATE` `Rajasthan` and `CMPGSTREGISTRATIONTYPE` `Regular`, the same as the ten keyed invoices.

#2464 proposes that, with two registrations, the build reads the Sales type's binding (`DEFAULTGSTREGISTRATION`, row 2162-RU's part C3) and judges the bound registration alone. That design holds only if Tally then gives the imported invoice the bound registration. If it gives another, the build would judge one registration while Tally issues the invoice under the other. Nothing has measured which.

- **The one variable:** one import of the build's invoice request, renumbered, into a book that holds two GST registrations, with `BRIDGE Sales` bound to the one that is **not** the book's first: `Maharashtra Registration`. Binding to the second makes the answer tell the binding apart from the first unit and from `LASTGSTTAXUNIT`, which reads `Rajasthan Registration` (row 2162-RU).
- **The set-up,** one change per save, each read before the next: row 2162-RU's part C2 (a second registration, `27ZZZZZ0000Z1ZQ`, Maharashtra) and part C3 with the other line chosen (the binding).
- **What is read:** the tax units (U), the voucher types (T), the marks (M), the number read (N), the build's read-back (R) and a native read of one voucher by number (V), whose answer holds the registration fields.
- **Not asked here:** GSTR-1, the place of supply under the bound registration (the invoice is `0010`'s intra-state Rajasthan sale, so under a Maharashtra registration its taxes are not the ones a person would key; the status flags are recorded, not judged), an unbound type, and a type bound to the first registration.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-KC's, 2162-SC's, 2162-RU's, 2162-GF's, 2162-GY's, 2162-RP's and 2467-GS's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count16.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's, and its U kind whether each tax unit's GUID begins with it.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is keyed unless s1's COMPANY row is the book's.
- **Its state,** from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **The number, `BP/26-27/0091`, is chosen from the captured answers, cancelled vouchers included.**
  - Row 2162-S5's year read (`lab/2162-s5-answers` at ab3d2e77, `answers/a-s2-Y-all.xml`, 117,992 bytes, sha256 beginning `f8ccb3ca`) returns every voucher of 1 Apr 2026 to 31 Mar 2027, of every type, with `ISCANCELLED`: 41 rows, at the book's restored state (marks 68 and 245).
    - The `BP/26-27/` numbers held are 0001 to 0016, 0019, 0020, 0021 and 0041 to 0044.
    - `0041` is the one cancelled (20260820, `BRIDGE Sales`). This row's first sitting stopped at step 2 on it, as the step's STOP is meant to.
  - Row 2162-KC's August read at s2 (`lab/2162-kc-answers` at e3c2bd1f, RESULTS.md, `a-s2-W-2026-08.xml`, 40 rows) lists the same numbers for that month, `0041` among them.
  - Every voucher number in every committed answer on the `lab/*-answers` branches (322 branches, UTF-16 decoded) was counted: no answer holds `BP/26-27/0091`. That includes numbers that rows created and then restored away, up to `0072`.
  - No file on any `lab/` branch names `BP/26-27/0091`, or the REMOTEID tail `000000000991`, and nor does master.
  - `0081` was passed over: row 2360's queued sheet sends it in its run A (`lab/2360-declined-sheet` at 5e73c1db).
  - Step 2's N read stays a STOP, in case this check is wrong.
- **BK is this row's way back.** It is taken at step 1, before anything is keyed, and restored at step 6 by C6. The restore runs after step 3 in every case, including a stop. After it, every read is compared whole with the reads before the change; any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2162-RU, 2162-SC, 2162-GY and 2467-GS read TallyPrime with tally.exe 1.1.7.2.

**No Bridge build.** Raw requests, one at a time, through `run.py`. The two changes are made on Tally's screen, by hand. The one write is `I-0091.xml`, sent once.

**The sends** are row 2162-RP's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count16.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count16.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.

## Files
- **Byte-equal to row 2162-RP's file of the same name** (`lab/2162-rp-sheet` at 86c0029f, folder `lab/2162-rp/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` (the build's read-back for `BP/26-27/0010`), `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`, sha256 `b333f0dc`, equal to `PILOT_REQUEST_SHA256` on master) and `template-n-0001.utf16le.xml` (the build's number read).
- **Byte-equal to row 2162-GY's file of the same name** (`lab/2162-gy-sheet` at 0b8f09d8): `U-taxunits.xml` (the build's company registration read, sha256 `c763f19d`) and `T-types.xml` (the build's voucher-type read, sha256 `c351d6c6`), as rows 2162-RU and 2162-GY sent them.
- **`template-full-0057.utf16le.xml`:** row 2162-RS's `V-0057.xml` (`lab/2162-rs-sheet`, folder `lab/2162-rs/`), byte for byte: every native field of the one voucher holding a number, over 1 Apr 2026 to 31 Mar 2027. Its ROW.md records that with the number set back it is row 2162-BC's `V-0055.xml`. It is not a new shape.
- **`gen.py`** writes, each with only the change it names asserted, and each template's sha256 checked first:
  - `I-0091.xml`: the template import with the number `BP/26-27/0091` (twice) and the REMOTEID tail `0991`. It names no registration, as the build's request never does (`gen.py` asserts it). An edit of a captured request, as row 2162-HS's import was.
  - `N-0091.xml`, `R-0091.xml`: the number read and the read-back for `BP/26-27/0091`.
  - `V-0010.xml`, `V-0091.xml`: the native read for `BP/26-27/0010` and for `BP/26-27/0091`.
- **`count16.py`:** row 2162-GY's `count15.py` with one change, marked GR: the V kind also prints `GSTREGISTRATION` (its text, and any attribute it carries), `CMPGSTIN`, `CMPGSTSTATE`, `CMPGSTREGISTRATIONTYPE`, `PLACEOFSUPPLY` and `STATENAME`, each counted as an element (`<absent>`, or `<N times>` if repeated). Tried on `0010`'s voucher cut from the `pilot-lab-vouchers-after-post` fixture and re-encoded as UTF-16LE, it prints `Rajasthan Registration`, `08ZZZZZ0000Z1ZQ`, `Rajasthan`, `Regular`, `Rajasthan`, `Rajasthan`, and no attributes.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count16.py a-<label>-*`. A STOP from either script stops the row there.

**The prompt rule, at every step.** The only prompts answered are C5's, C6's and the three step 3 names, each with the key named. For any other prompt, message or dialog, copy it word for word and screenshot it first. Then answer No, or Esc if it offers no No. Press nothing more and send nothing. Then leave any screen still open by Ctrl+Q (C7), restore BK by C6, compare as step 6, and close (C10). That prompt is this row's result.

**The wait rule.** After each Ctrl+A, and after the send at step 5: if 5 minutes pass with no list, no Gateway, no answer and no message, send nothing, press nothing, screenshot the whole window, and report to the run coordinator. A read sent while Tally is busy can hold its gateway. With the Gateway showing and no dialog, run C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

**The path rule.** Every screen in steps 3 and 4 is row 2162-RU's (`lab/2162-ru-answers` at c7b14084, `evidence/sC-screens.txt`, steps 5.1 to 5.13). If the line or screen shown is not exactly what the step names: stop, then Esc out of a list or prompt (never a creation or alteration screen, C7). Restore BK by C6, check as step 6, then Quit (C10).

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2464-gr\backup-2` (the first sitting took `backup-1`; leave it as it is). Row 2162-GY's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml T-types.xml N-0091.xml R-0010.xml V-0010.xml M-marks.xml`.
   - EXPECT U byte-equal to row 2162-RU's s2 answer (`answers/a-s2-U-taxunits.xml` at c7b14084: 22,636 bytes, sha256 beginning `e40ef83c`): two units, `Rajasthan Registration` the one GST unit.
   - EXPECT T: `BRIDGE Sales` with `DEFAULTGSTREGISTRATION` empty (count16 prints no unit leaf for it), `LASTGSTTAXUNIT` `Rajasthan Registration` and `TAXUNITNAME` `Any`. These are as row 2162-RU's s2 and sC2 answers read, and as this row's first sitting read on 10 Nov 2026.
   - EXPECT N-0091 0 rows: no voucher holds the number, cancelled or not (the Book's number bullet says why). One row or more: stop before step 3, with each row recorded whole.
   - EXPECT R-0010 one row: `BRIDGE Walk-in`, included Yes, every other status flag No, 6 legs.
   - EXPECT V-0010 one VOUCHER: `GSTREGISTRATION` `Rajasthan Registration`, `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, `CMPGSTSTATE` `Rajasthan`, `CMPGSTREGISTRATIONTYPE` `Regular`, as the fixture reads. Anything else: record it whole and stop before step 3. It would mean the book does not hold what the 9 Oct read recorded.
3. **Change 1: a second GST registration (row 2162-RU's part C2).** Before it, `s3m`: `M-marks.xml`, then `count16.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass. Then no key is pressed in Tally but those below.
   1. At the Gateway (shot), Create, Enter. `Master Creation`, the List of Masters. Down to `GST Registration` (RU: Down 9 to Godown, once to the heading, once more), Enter (shot).
   2. **Named prompt 1.** EXPECT, word for word, "GST Registration already exists for the Company. Do you want to alter the existing GST Registration or create a new GST Registration?", with `C Create New` and `A Alter Existing` (shot, zoomed). If the words are the same: press `c`. If they differ: the prompt rule.
   3. `GST Registration Creation` (shot). Write down every field as Tally filled it. EXPECT RU's 5.2: State empty, Registration type Regular, Assessee of Other Territory No, GSTIN/UIN empty, Periodicity of GSTR-1 Monthly, the rest as RU listed.
   4. State: type `Maha`; EXPECT the List of States with `Maharashtra` the one state line; Enter. Registration type: its list Regular, Composition, Regular - SEZ; `Regular` highlighted (RU: Down once), zoom and check, Enter. Assessee of Other Territory: No, Enter. GSTIN/UIN: type `27ZZZZZ0000Z1ZQ`; zoom and check before any Enter that it reads `27ZZZZZ0000Z1ZQ`, 15 characters, capitals (shot). Then Enter on each later field, leaving it as Tally filled it, as RU's 5.4 did.
   5. **Named prompt 2.** On the last field's Enter, EXPECT `Accept ?` (Yes or No) (shot). Press `y`.
   6. **Named prompt 3.** EXPECT, word for word, "You can create a unique voucher numbering series to cater to multiple GST registrations. You can do this by going to the Chart of Accounts and selecting the relevant voucher types. Open Chart of Accounts and create now?", Yes or No (shot, zoomed). If the words are the same: press `n`. If they differ: the prompt rule.
   7. EXPECT a new, empty `GST Registration Creation` form (RU's 5.7). Type nothing in it. Ctrl+Q, then Ctrl+Q again: `Master Creation`, then the Gateway (shot). Two Ctrl+Q in all, as RU's 5.8.
   - Then `s3`: `U-taxunits.xml T-types.xml M-marks.xml`.
     - `count16.py --unit-diff a-s2-U-taxunits.xml a-s3-U-taxunits.xml`. Record the whole output. EXPECT a third unit, `Maharashtra Registration`, `27ZZZZZ0000Z1ZQ`, its GUID the book's with a suffix, one dated row from 20260401, Maharashtra, Regular, not inactive; the other two units unchanged.
     - `count16.py --type "BRIDGE Sales" --type-diff a-s2-T-types.xml a-s3-T-types.xml`. Record the whole output. EXPECT `DEFAULTGSTREGISTRATION` still empty.
4. **Change 2: bind BRIDGE Sales to the second registration (row 2162-RU's part C3, with the other line).** Before it, `s4m`: `M-marks.xml`; record ALTMSTID against s3's.
   1. At the Gateway (shot), Go To (its label), Up to `Alter Master` (RU: Up 4), Enter. `Master Alteration`; Down to `Voucher Type` (RU: Down 3), Enter. `List of Voucher Types`: Enter on `BRIDGE Sales` only (shot).
   2. `Voucher Type Alteration` (shot). Write down every field, as text. EXPECT RU's 5.10, with `Default GST Registration` reading `Any`.
   3. A click on the value of `Default GST Registration`. EXPECT the list `List of GST Registrations`: Create; Any (highlighted); Maharashtra Registration; Rajasthan Registration. **Down once:** `Maharashtra Registration` highlighted; zoom and check; Enter (shot). The cursor moves to `Use common voucher numbering series for all GST Registrations: No`. **Touch no other field.**
   4. Ctrl+A. EXPECT no prompt, and the List of Voucher Types (RU's 5.12). Any prompt: the prompt rule. Ctrl+Q once: the Gateway (shot).
   - Then `s4`: `T-types.xml U-taxunits.xml M-marks.xml`.
     - `count16.py --type "BRIDGE Sales" --type-diff a-s3-T-types.xml a-s4-T-types.xml`. Record the whole output. EXPECT `DEFAULTGSTREGISTRATION` `Maharashtra Registration` on `BRIDGE Sales`, and record `LASTGSTTAXUNIT`, `LASTGSTTAXUNITBYSERIES` and `TAXUNITNAME` as they now read.
     - `count16.py --unit-diff a-s3-U-taxunits.xml a-s4-U-taxunits.xml`: EXPECT 0 leaves differ in every unit.
5. **The import.** `s5m`: `M-marks.xml`, then `count16.py --before-write a-s1-M-marks.xml a-s5m-M-marks.xml`. It must pass. Then no key is pressed in Tally before the send.
   - `s5`: `I-0091.xml`, sent once. Then the wait rule. Record every counter and any LINEERROR, word for word. The window after it (shot): the Gateway, no dialog. A dialog: the prompt rule.
   - `s5r`: `N-0091.xml R-0091.xml V-0091.xml V-0010.xml M-marks.xml`.
     - Record N-0091's rows, each with its GUID.
     - Record R-0091's rows: ledger, the three status flags and the legs.
     - **The row's answer: V-0091's `GSTREGISTRATION` (and any attribute), `CMPGSTIN`, `CMPGSTSTATE`, `CMPGSTREGISTRATIONTYPE`, `PLACEOFSUPPLY` and `STATENAME`, as count16.py prints them.** Record them whole, beside V-0010's.
     - `count16.py --diff a-s2-V-0010.xml a-s5r-V-0010.xml`. Record the whole output: whether the existing invoice's registration fields moved.
     - Record ALTVCHID and ALTMSTID against s5m's.
   - Any answer at s5 is a result, created or not. Record it and go on to step 6. Never send `I-0091.xml` again (C3).
6. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2162-GY's step 5 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). Then `s6`: `M-marks.xml U-taxunits.xml T-types.xml N-0091.xml R-0010.xml V-0010.xml`, and compare whole:
   - `count16.py --same-row a-s1-M-marks.xml a-s6-M-marks.xml`: EXPECT it passes.
   - `count16.py --unit-diff a-s2-U-taxunits.xml a-s6-U-taxunits.xml`: EXPECT the two units and 0 leaves differ in each.
   - `count16.py --type "BRIDGE Sales" --type-diff a-s2-T-types.xml a-s6-T-types.xml`: EXPECT no type differs.
   - N-0091 0 rows. `count16.py --diff a-s2-R-0010.xml a-s6-R-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
   - Record each answer's sha256 against s2's; EXPECT each equal. **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule, step 3 and C6 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Say in the report what the book then holds: a stop between steps 3 and 6 leaves the second registration, and from step 4 the binding, and from step 5 whatever the import did.

## Record
- Each answer's bytes and sha256, and `count16.py`'s lines.
- The `--unit-diff`, `--type-diff`, `--diff`, `--before-write` and `--same-row` outputs, whole.
- Steps 3, 4 and 5's screenshots, listed by name, bytes and sha256.
- As text and word for word: the creation and alteration screens' fields, every key pressed, and every prompt, message and busy screen, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say.
- **Step 3:** as row 2162-RU's part C2: the three named prompts, then a third unit in force from 20260401.
- **Step 4:** as row 2162-RU's part C3 with the other line: `DEFAULTGSTREGISTRATION` `Maharashtra Registration`, `TAXUNITNAME` `Any`, the units unchanged.
- **Step 5:** created (no captured answer shows `0091` in use, and `I-0091.xml` is row 2162-HS's shape with another number). Which registration it gets is not estimated: that is the row's question.
- **s6:** equal to s2 and s1.

## Decides
Each is PARTIAL: one import, one sitting, one book, one release.
- **V-0091 reads `Maharashtra Registration`, `27ZZZZZ0000Z1ZQ`, Maharashtra:** Tally gives an imported invoice that names no registration its type's bound registration. #2464's design holds as written: judging the bound unit is judging the registration Tally issues under, with no registration field added to the request. The slice 1 "Not changed" bullet of ADR 0004 can then say so for a bound type.
- **V-0091 reads `Rajasthan Registration`, `08ZZZZZ0000Z1ZQ`:** Tally ignores the binding on an import and uses the first unit or the type's last one (`LASTGSTTAXUNIT`). #2464 as written would judge Maharashtra while Tally issues under Rajasthan. Before #2464 is built, the build must either name the registration in the request (a new request shape, measured first) or refuse a book with two registrations as it does today. Record which unit `LASTGSTTAXUNIT` named.
- **V-0091 holds no registration (`GSTREGISTRATION` and `CMPGSTIN` absent or empty):** with two registrations, Tally gives such an import none. The build must name the registration itself before any two-registration book is invoiced.
- **V-0091's fields disagree** (for example `GSTREGISTRATION` one unit and `CMPGSTIN` the other's GSTIN): record all six whole. Neither field can be relied on alone; the coordinator decides.
- **The import is refused** (`CREATED` 0, an error or a LINEERROR): record the words. A bound type, or two registrations with no common numbering series, then changes what an import needs on this release; that is its own question.
- **V-0010's registration fields moved at s5r:** the second registration or the binding reached an existing invoice. Record which field, and report before the restore.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build writes no registration.** `render_sales_invoice_xml` (`src-tauri/src/agent_import_invoice.rs:2664`, its template at `:2708`) writes `GSTREGISTRATIONTYPE` (the customer's) and no `GSTREGISTRATION`, `CMPGSTIN` or `CMPGSTSTATE`. `I-0091.xml` is that shape, and `gen.py` asserts it names none of the three.
- **Today the build refuses this book.** With two GST units in force, `parse_company_registration` refuses as `invoice_company_registration_ambiguous` (`src-tauri/src/agent_import_invoice_wire.rs:804`), and `parse_voucher_types` (`:274`) does not read `DEFAULTGSTREGISTRATION`. So no build reaches step 5's state; the row measures what #2464 would rely on, before it is built.

## Not measured
- A type bound to the first registration, and an unbound type with two registrations.
- `Use common voucher numbering series for all GST Registrations` set to Yes.
- An inter-state invoice under the bound registration, and GSTR-1 for either registration.
- A request that names a registration (`GSTREGISTRATION` or `CMPGSTIN` written by the build).
- An invoice built and posted through Bridge.
- Any other book or release.

Row: lab/2464-gr/
