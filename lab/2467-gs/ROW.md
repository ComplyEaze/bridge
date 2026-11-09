# Row 2467-GS: when the company's GSTIN changes to another GSTIN of the same state, what does the build's tax-unit read show?

Not for merge. Settles the measurement #2467 asks for (related to #2162, ADR 0004 item 16). Written 10 Nov 2026. One master alteration on Tally's screen, read before and after, then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
Row 2162-SC changed the GSTIN of BRIDGE PILOT LAB's one GST registration to another state's code, with the state kept (`lab/2162-sc-answers` at 87963e77). Tally asked nothing, `GSTREGNUMBER` and `TAXREGISTRATION` took the new number, `GSTOLDREGNUMBER` stayed empty, and the dated row did not change. The build refuses that answer through the GSTIN's state code alone. #2467 records that a change to another GSTIN of the **same** state would pass the build's consistency term unless the answer marks it some other way, and that no row has measured one.

- **The one variable:** the GSTIN of `Rajasthan Registration`, from `08ZZZZZ0000Z1ZQ` to `08ZZZZZ0000Z3ZO`. The state code (08, Rajasthan) and the PAN part are unchanged; only the entity character and the check character differ. Both are of the synthetic family the gate admits, and both pass the public check digit (the build's `gstin_valid`; master's admission test `a_customer_holding_the_companys_own_gstin_is_refused_and_another_registration_is_admitted` admits `08ZZZZZ0000Z3ZO` as a customer's valid GSTIN). `08ZZZZZ0000Z2ZP` is not used, because a ledger on this book holds it. The state stays Rajasthan, and nothing else on the screen is changed.
- **What is read:** the build's own tax-unit request (U) and the marks (M), before the change, after it, and after the restore.
- **Not asked here:** GSTR-1, a posted invoice's status, what the build does with the answer (that is a test once the answer is captured), and a dated change.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-KC's, 2162-SC's, 2162-RU's, 2162-GF's and 2162-GY's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count15.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's, and its U kind whether each tax unit's GUID begins with it.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is keyed unless s1's COMPANY row is the book's.
- **Its state,** from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **BK is this row's way back.** It is taken at step 1, before anything is keyed, and restored at step 5 by C6. The restore runs after step 3 in every case, including a stop. After it, every read is compared whole with the reads before the change; any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2162-RU, 2162-SC and 2162-GY read TallyPrime Silver with tally.exe 1.1.7.2.

**No Bridge build.** Raw reads, one at a time, through `run.py`. The change is made on Tally's screen, by hand; no request writes.

**The sends** are row 2162-GY's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, or an answer that does not parse (`count15.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count15.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.

## Files
Byte-equal to row 2162-GY's file of the same name (`lab/2162-gy-sheet` at 0b8f09d8, folder `lab/2162-gy/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count15.py`, `M-marks.xml` and `U-taxunits.xml`.

- `U-taxunits.xml` is the build's company registration read (`render_company_registration_request`, `src-tauri/src/agent_import_invoice_wire.rs:694` at master ae38a75c), 934 bytes, sha256 beginning `c763f19d`: the request rows 2162-RU and 2162-SC sent. It is not a new shape.
- `count15.py`'s U kind prints each tax unit's name, type, GSTIN attribute, GUID (and whether it is the book's), `GSTREGNUMBER`, `GSTOLDREGNUMBER`, `USEDFOR` and its dated rows. Its `--unit-diff A B` prints, unit by unit, every attribute and leaf that differs, at most 200 lines. Tried on row 2162-SC's s2 and s5 answers, it prints 0 leaves for `Default Tax Unit` and three for `Rajasthan Registration`: `ALTERID`, `GSTREGNUMBER` and `@TAXREGISTRATION`.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count15.py a-<label>-*`. A STOP from either script stops the row there.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date, and write it down. Rows 2162-RU and 2162-SC read 15 Sep 2026.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2467-gs\backup-1`. Row 2162-GY's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml M-marks.xml`.
   - EXPECT U byte-equal to row 2162-RU's s2 answer (`lab/2162-ru-answers` at c7b14084, `answers/a-s2-U-taxunits.xml`: 22,636 bytes, sha256 beginning `e40ef83c`), which is byte-equal to row 2162-SC's s2 and s8 answers. So: two TAXUNIT rows, each the book's; `Rajasthan Registration` with TAXTYPE `GST`, TAXREGISTRATION and GSTREGNUMBER `08ZZZZZ0000Z1ZQ`, GSTOLDREGNUMBER empty, and one dated row `20260401`, Rajasthan, Rajasthan, Regular, not inactive.
   - Not equal: record `count15.py --unit-diff` against RU's s2 answer, and stop before step 3.
3. **The change, on Tally's screen.** Before it, `s3m`: `M-marks.xml`, then `count15.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass. After that, no key is pressed in Tally but those below.
   - **The path rule.** Every screen below is row 2162-SC's step 4 path (`lab/2162-sc-answers` at 87963e77, `evidence/s4-screens.txt`). If the line or screen shown is not exactly what the step names: stop, then Esc out (a list or prompt, never the alteration screen itself, C7). Restore BK by C6, check as step 5, then Quit (C10).
   - **The prompt rule.** SC's change showed no prompt and no message, not even `Accept ?`, so this row answers none. For any prompt or message, at any step: copy it word for word and screenshot it first, then answer No, or Esc if it offers no No. Press nothing more. Then leave any screen still open by Ctrl+Q (C7), restore and compare by step 5, and close. That prompt is this row's result. A request for a date from which the new number applies is such a prompt: a dated change is a second variable.
   1. At the Gateway (shot), press Alt+G (Go To) and type `Alter Master`. (SC typed `GST Registration` first and got a Spelling Error box; do not.) Enter on the line `Alter Master`. EXPECT `Master Alteration`, with `GST Registration` under Statutory Masters (shot).
   2. Move to `GST Registration` with Down only (SC took 11), and Enter. EXPECT `List of GST Registrations` with one line, `Rajasthan Registration` (s2's U name). Enter on it (shot). More than one line, or another name: the path rule.
   3. `GST Registration Alteration` (shot). Write down every field it shows with its value, as text. EXPECT the fields SC recorded at 4.3, with `GSTIN/UIN` holding `08ZZZZZ0000Z1ZQ`. No such field, or more than one: press nothing more, and stop.
   4. Enter, then Enter: the cursor passes Registration type (kept Regular) and Assessee of Other Territory (kept No) to `GSTIN/UIN`, its value highlighted. **No Backspace** (SC found one walks back to the field before). Paste `08ZZZZZ0000Z3ZO` over the highlighted value. Zoom the field and check, before any Enter, that it reads `08ZZZZZ0000Z3ZO`: 15 characters, capitals (shot). A wrong value: Enter back onto the field so it is highlighted, and paste again.
   5. **Touch no other field.** Write down what State, Registration type, Assessee of Other Territory and Periodicity of GSTR-1 read. If Tally opens any field or screen not in step 3.3's list: the prompt rule.
   6. Zoom the field once more, then Ctrl+A (shot after). EXPECT, as SC: no prompt, and `List of GST Registrations` again. Any prompt or message: the prompt rule.
      - **The wait rule.** If 5 minutes pass after Ctrl+A with no list, no Gateway and no message: send nothing, press nothing, screenshot the whole window, and report to the run coordinator. A read sent while Tally is busy can hold its gateway.
   7. Ctrl+Q on the list: the Gateway (shot). Write down every screen passed, then C2's status check (`GET /status` 200, one tally.exe owning 9001) before any read.
4. **The book after the change.** `s4`: `M-marks.xml U-taxunits.xml`. Then, each recorded whole:
   - `count15.py --unit-diff a-s2-U-taxunits.xml a-s4-U-taxunits.xml`. Record every leaf that moved. In particular: `GSTOLDREGNUMBER` (empty, or a number, and which); whether `Rajasthan Registration` holds a second dated row, with its `FROMDATE`, state, type and inactive flag; and whether `Default Tax Unit` or any other unit changed, or a new unit appeared.
   - Record ALTMSTID and ALTVCHID against s3m's.
   - Any read that now fails is a result: record it as the request failed, with its words, and go on to step 5.
5. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2162-GY's step 5 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). Then `s5`: `M-marks.xml U-taxunits.xml`, and compare whole:
   - `count15.py --same-row a-s1-M-marks.xml a-s5-M-marks.xml`: EXPECT it passes.
   - `count15.py --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml`: EXPECT 0 leaves differ in each unit.
   - Record each answer's sha256 against s2's. **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
6. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those that step 3's prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A stop between step 3.6's Ctrl+A and step 5 leaves the book with the changed GSTIN: say so in the report.

## Record
- Each answer's bytes and sha256, and `count15.py`'s lines.
- The `--unit-diff`, `--before-write` and `--same-row` outputs, whole.
- Step 3's screenshots, listed by name, bytes and sha256.
- As text and word for word: the alteration screen's fields before the change, every key pressed, and every prompt, message and busy screen, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date.

## Expected
Estimates, not predictions.
- **s2:** U byte-equal to RU's s2 answer; the marks at 68 and 245.
- **Step 3:** as SC's step 4: no date asked, no prompt, the list again after Ctrl+A.
- **s4:** as SC's s5, with the new number: `GSTREGNUMBER` and `@TAXREGISTRATION` `08ZZZZZ0000Z3ZO`, `ALTERID` moved, `GSTOLDREGNUMBER` empty, the dated row unchanged. That estimate is the row's question, since SC changed the state code and this row does not.
- **s5:** equal to s2 and s1.

## Decides
What each answer shape at s4 means for the build's consistency term (`parse_company_registration`, `src-tauri/src/agent_import_invoice_wire.rs:812`-`:818` at master ae38a75c: no `GSTOLDREGNUMBER`, `TAXREGISTRATION` equal to `GSTREGNUMBER`, a valid GSTIN, and its state code the dated row's state). Each is PARTIAL: one change, one sitting, one release.
- **`GSTOLDREGNUMBER` filled** (with `08ZZZZZ0000Z1ZQ`, or with anything): the term already refuses the changed unit as `invoice_company_registration_inconsistent`, at every invoice date. Both of #2467's gaps are then closed by the existing check, with no code: an approved invoice whose re-read finds the change is refused, and no invoice is issued under the new number for a date before it. The cost is the build's reach: every invoice of a company that ever changed its GSTIN is refused, after the change as well as before. That goes to the coordinator as a finding.
- **A dated row added** (a second `GSTREGISTRATIONDETAILS.LIST` row with a later `FROMDATE`, the same state, type and flag): the term does not read a GSTIN from the dated rows, so it still passes, and #2467's gaps stand as written. The row's date is then evidence of when the change took effect, which a later decision could use; record which date it holds (Tally's current date, as row 2162-RU's Composition took, or another). If that row names a different state, type or flag from the first, record it: the term then judges the row in force on the invoice date, and SC's and RU's answers are the reference.
- **Nothing visible but the number** (`GSTREGNUMBER` and `@TAXREGISTRATION` changed, `ALTERID` moved, nothing else): the term passes the changed unit, and nothing in the build's read tells the old number from the new. #2467's two gaps are then measured, not only reasoned: the re-read before a post cannot see the change, and an invoice dated before it would be issued under the new number. What the build does about it is decided after this row, in #2467.
- **A new unit appears beside the first, or the first goes inactive:** Tally records a same-state change as a new registration. Record both units whole. The term then counts the units in force on the invoice date (refused as ambiguous if two are), and row 2162-RU's second registration is the reference.
- **Tally asks for a date, or shows any message, at step 3:** the prompt rule; record the words. A same-state change is then dated or warned on this release, and whether to answer it is a second row.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build reads the registration this row changes.** The invoice judge reads the company's tax units (`render_company_registration_request`, `src-tauri/src/agent_import_invoice_wire.rs:694`) and parses them with `parse_company_registration` (`:731`, called at `src-tauri/src/agent_import_invoice.rs:2338`).
- **It keeps the GSTIN only for one check.** The parse returns the unit's GSTIN and state (`CompanyRegistration`, `agent_import_invoice_wire.rs:704`). The judge uses the GSTIN only to refuse a customer holding the company's own (`agent_import_invoice.rs:2354`) and records only the state in the observation (`:2382`; `InvoiceObserved::company_state`, `:78`), which the re-reads compare (`first_difference`, `:1299`). So whether this change is caught depends only on what the answer shows the term at `agent_import_invoice_wire.rs:812`.

## Not measured
- A dated GSTIN change, or one answered through a prompt.
- A change of the state, or of the PAN part.
- GSTR-1, and the status a posted voucher holds after the change.
- A company with more than one registration.
- Any other book or release.
- An invoice built and posted through Bridge while the book is altered.

Row: lab/2467-gs/
