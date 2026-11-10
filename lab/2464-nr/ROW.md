# Row 2464-NR: on the book with one GST registration, is an imported Sales invoice that names that registration created, and does it keep it?

Shape discovery: the two imports and the candidate read-back are requests written for this row, not requests the build renders, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2464 (0.9.0), related to #2467 (gap 1). Written 10 Nov 2026. No master change, one import (a second only if the first is created), then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
Row 2464-GN (`lab/2464-gn-answers` at aa2ff2e6, RESULTS.md) sent the build's invoice request with one element added after `PARTYNAME`: `<GSTREGISTRATION>Maharashtra Registration</GSTREGISTRATION>`. Its book held two registrations, with `BRIDGE Sales` bound to that one. Tally answered `CREATED` 0 and `EXCEPTIONS` 1, with no words, and stored nothing. Without the element, the same request was created on that book. Four causes fit that answer, and GN's reads cannot tell them apart:
1. the element, its place or its form is not taken on an import, whatever unit it names;
2. the named unit disagrees with the invoice, which is intra-state Rajasthan (place of supply Rajasthan, CGST and SGST legs);
3. the named unit disagrees with something else in the book;
4. a combination.

This row asks cause 1 alone.
- **The one variable.** Import A' is the request that created `BP/26-27/0010` on this book, renumbered, with GN's one element added in GN's place and form. The element names `Rajasthan Registration`: the book's one GST registration, and the unit Tally itself gave `BP/26-27/0010` (GN's s2, the native read).
  - Against GN's import A, A' differs in three places: the unit's name, the number and the REMOTEID tail (Files).
  - With the element taken out, A' is the template renumbered, byte for byte.
- **Nothing in the invoice or the book disagrees with the unit.** The book is the restored state: one registration, `BRIDGE Sales` unbound (`Default GST Registration` `Any`). No master is changed and no named prompt is answered.
- **Import B', only if A' is created.** B' is A' with one more element after it: `<CMPGSTIN>08ZZZZZ0000Z1ZQ</CMPGSTIN>`, that unit's GSTIN, as GN's B was to A. A builder would write both fields. B' asks whether writing the GSTIN as well changes what is stored. If A' is not created, B' adds nothing and is not sent.
- **The read-back.** `RC-` is the build's read-back with `GSTREGISTRATION` and `CMPGSTIN` added at the end of its FETCH list, byte-equal to GN's. On `BP/26-27/0010` it returned the two fields equal to the native read. If A' is created, `RC-0085` is its first answer on an invoice written with a registration.
- **Not asked here:** a book with two registrations, a type bound to a registration, a mismatched pair, `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE` written by the request, `GSTREGISTRATION` with an attribute or in another place, GSTR-1, and the build's own rendered request (no builder writes these elements yet).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2464-GN's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count19.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is keyed unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. GN's s1, s2 and s8 read those marks.
  - Step 2 compares every read with GN's s2 answer of the same kind, byte for byte. That compare holds only from the same marks.
  - So at s1 any other marks stop the row, before BK and before any key.
- **A sitting of its own, or a shared one.** The row needs no set-up. It may follow another row in the same sitting only when that row's restore compare passed, and s1 here then reads the marks above. The run coordinator decides.
- **The numbers, `BP/26-27/0085` (A') and `BP/26-27/0087` (B'), are chosen from the captured answers and the branches.**
  - Row 2162-S5's year read (`lab/2162-s5-answers` at ab3d2e77, `answers/a-s2-Y-all.xml`) lists every voucher of 1 Apr 2026 to 31 Mar 2027 at the restored state. The `BP/26-27/` numbers held are 0001 to 0016, 0019, 0020, 0021 and 0041 to 0044.
  - Every blob on every `lab/` branch (659 refs, 23,702 blobs, read as UTF-16 and as UTF-8) and on master was searched on 10 Nov 2026.
    - None holds `26-27/0085` or `26-27/0087`.
    - None holds a REMOTEID ending in the tail `000000000985` or `000000000987`. The digits `000000000987` occur only inside the bank-statement fixtures and their generator, never in a REMOTEID.
  - `0081` and `0083` were passed over: row 2360's declined-import sheet uses them. `0089`, `0091`, `0093`, `0095`, `0097`, `0098` and `0099` are used by other rows.
  - Step 2's N reads stay a STOP, in case this check is wrong.
- **BK is this row's way back, and the restore always runs.**
  - BK is taken at step 1, before anything is sent, and restored at step 5 by C6.
  - Every path from step 3 on that is not a STOP reaches step 5. That includes A' declined, A' created with B' not sent, B' sent, and step 4's compare failing.
    - GN's refused imports moved Tally's own count lines (`DESC/CMPINFO`) until its restore. So the restore runs even when nothing is created.
  - After a STOP (C7, C8, C9), C8 holds: nothing more is sent or keyed, and BK is restored only on the run coordinator's word.
  - After the restore, every read is compared with step 2's. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. GN read TallyPrime with `tally.exe` 1.1.7.2, from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. The writes are `I-0085.xml` and `I-0087.xml`, each sent at most once.

**The sends** are row 2464-GN's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count19.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count19.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.
- Answers that hold a GSTIN stay on the lab PC under the lab's standing rules, listed by size and sha256, as GN's did. That covers U, V and RC.

## Files
**Copied from row 2464-GN**
- **Byte-equal to GN's file of the same name** (`lab/2464-gn-sheet` at 23b2e57b, folder `lab/2464-gn/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count19.py`, `M-marks.xml`, `U-taxunits.xml`, `T-types.xml`, `R-0010.xml`, `template-import-0010.utf16le.xml`, `template-full-0057.utf16le.xml` and `template-n-0001.utf16le.xml`.
  - `template-import-0010.utf16le.xml` is the build's request that created `BP/26-27/0010` (sha256 `b333f0dc`, equal to `PILOT_REQUEST_SHA256` on master).
  - `gen.py` writes `RC-0010.xml` and `V-0010.xml` again, and asserts each equal to GN's.
- **`count19.py` is GN's, unchanged.** It counts every kind this row reads (M, U, T, N, I, R, and V), with an `RC-` answer counted as R. GN ran each of its modes on its own answers.

**GN's s2 answers: what step 2 compares with.** GN read the restored book at its s2, before any change, and again at its s8, after its restore. Each pair below was byte-equal. The answers with no GSTIN are committed on `lab/2464-gn-answers` at aa2ff2e6, `lab/2464-gn-answers/answers/`. The rest stayed on the lab PC, listed in GN's `kept-local.txt`.

| Read here | GN's s2 answer | Bytes | sha256 | Where |
| --- | --- | --- | --- | --- |
| `M-marks.xml` | `a-s2-M-marks.xml` | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` | committed |
| `T-types.xml` | `a-s2-T-types.xml` | 306,946 | `cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed` | committed |
| `N-0085.xml`, `N-0087.xml` | `a-s2-N-0095.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | committed |
| `R-0010.xml` | `a-s2-R-0010.xml` | 94,474 | `b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5` | committed |
| `U-taxunits.xml` | `a-s2-U-taxunits.xml` | 22,636 | `e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989` | lab PC |
| `RC-0010.xml` | `a-s2-RC-0010.xml` | 94,744 | `5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399` | lab PC |
| `V-0010.xml` | `a-s2-V-0010.xml` | 52,784 | `4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456` | lab PC |

- An N answer with no rows does not carry the number: GN's `a-s2-N-0095.xml` and `a-s2-N-0097.xml` are byte-equal. So both N reads here are compared with the one answer.
- Before step 1, copy GN's three lab-PC answers beside this row's answers as `a-gns2-U-taxunits.xml`, `a-gns2-RC-0010.xml` and `a-gns2-V-0010.xml`, checking each sha256 first. Also copy GN's committed `a-s2-T-types.xml` and `a-s2-R-0010.xml` as `a-gns2-T-types.xml` and `a-gns2-R-0010.xml`.
  - If any is missing or its sha256 differs: STOP before step 1. Step 2's compares cannot be checked without them.
  - They are needed only to show what differs, if anything does.

**`gen.py`** writes each request below, with only the change it names, each asserted to occur exactly as often as expected. Each template's sha256 is checked first. As bytes against the build's `0010` request (`template-import-0010.utf16le.xml`):
- **`I-0085.xml` (A'):**
  - `VOUCHERNUMBER` and `REFERENCE` `BP/26-27/0010` become `BP/26-27/0085`;
  - the REMOTEID's last group `000000000777` becomes `000000000985`;
  - one element is inserted right after `<PARTYNAME>BRIDGE Walk-in</PARTYNAME>`: `<GSTREGISTRATION>Rajasthan Registration</GSTREGISTRATION>`.
  - Nothing else. `gen.py` asserts that taking the element out again leaves the template renumbered, byte for byte.
  - **Against GN's `I-0095.xml`:** the same function, given GN's number, tail and unit, writes GN's `I-0095.xml` byte for byte (sha256 `d955b163`, asserted). So A' differs from GN's A in the unit's name, the number and the tail, and in nothing else.
- **`I-0087.xml` (B'):**
  - the number `BP/26-27/0087` and the tail `000000000987`;
  - the same `GSTREGISTRATION`, then `<CMPGSTIN>08ZZZZZ0000Z1ZQ</CMPGSTIN>`, both right after `PARTYNAME`.
  - Against `I-0085.xml`: the number, the tail, and the `CMPGSTIN` element.
- **Where the elements go:** GN's place and form, which follow Tally's own native read of `BP/26-27/0010` (the `pilot-lab-vouchers-after-post` fixture on master). That read gives `PARTYNAME`, then `GSTREGISTRATION` (the unit's name, no attributes), then `CMPGSTIN`, as three consecutive children.
  - `gen.py` parses each import and asserts that order.
  - It also asserts that each added element carries no attribute.
  - It asserts that neither import names `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE`.
- **`RC-0010.xml`, `RC-0085.xml`, `RC-0087.xml`:** `R-0010.xml` with `, ISPOSTDATED</FETCH>` changed to `, ISPOSTDATED, GSTREGISTRATION, CMPGSTIN</FETCH>`, then with the number `0085` and `0087`. `RC-0010.xml` is asserted byte-equal to GN's.
- **`N-0085.xml`, `N-0087.xml`, `R-0085.xml`, `R-0087.xml`, `V-0085.xml`, `V-0087.xml`:** the build's number read, the build's read-back and the native read of one voucher by number, each with the new number. `V-0010.xml` is asserted byte-equal to GN's.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count19.py a-<label>-*`. A STOP from either script stops the row there.

**No screen is opened in this row.** Nothing is keyed in Tally but C1's load, C5's backup, C6's restore and C10's close. The only prompts answered are those C5 and C6 quote. Any other prompt, message or dialog, at any step, is C7's: screenshot it, copy its text word for word, press no key, send nothing, and report. That is a STOP (C8).

**The wait rule.** After each import's send: if 5 minutes pass with no answer and no message:
- send nothing, press nothing, screenshot the whole window, and report to the run coordinator;
- a read sent while Tally is busy can hold its gateway.
- With the Gateway showing and no dialog, run C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down. GN read 1-Apr-26 to 31-Mar-27 and Tuesday 15-Sep-2026.
   - Check the five `a-gns2-*` copies (Files).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245.
     - Other marks: stop here, before BK, with nothing keyed (the Book's state bullet says why).
   - **Take BK** by C5, to the new empty folder `C:\lab\2464-nr\backup-1`. GN's step 1 is the path.
     - Tally may show the folder with its own capital, as `C:\lab\2464-Nr\backup-1`.
     - Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml T-types.xml N-0085.xml N-0087.xml R-0010.xml RC-0010.xml V-0010.xml M-marks.xml`.
   - EXPECT each answer byte-equal to GN's s2 answer of the same kind (Files): the sha256 and the bytes in the table.
   - So: two units, `Rajasthan Registration` the one GST unit; `BRIDGE Sales` unbound; 0 rows for each number; `BP/26-27/0010` with `Rajasthan Registration` and `08ZZZZZ0000Z1ZQ` in V and RC.
   - **If any answer differs:** compare it and record the whole output. For U, T, R, RC and V, run `--unit-diff`, `--type-diff` or `--diff` against the `a-gns2-` copy. For M, run `--same-row a-s1-M-marks.xml a-s2-M-marks.xml`. For N, `count19.py`'s own lines are the compare.
     - Then stop before step 3: nothing has been sent but reads, so nothing is restored. Close by C10.
     - One row or more in N-0085 or N-0087 is such a difference. Record each row whole.
3. **Import A': the registration's name alone.**
   - `s3m`: `M-marks.xml`, then `count19.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass.
   - `s3`: `I-0085.xml`, sent once. Then the wait rule.
     - Record every counter and any LINEERROR, word for word.
     - The window after it (shot): the Gateway, no dialog.
     - GN's import A was answered with 538 bytes, sha256 `09a713e2674c7016c2cb10f194f6e0fc7f6bf9dee3b523f4f6bbf1b07ea0f874`. Record whether this answer is byte-equal to it.
   - `s3r`: `N-0085.xml R-0085.xml RC-0085.xml V-0085.xml V-0010.xml M-marks.xml`.
     - Record N-0085's rows, each with its GUID.
     - Record R-0085's rows: ledger, the three status flags and the legs.
     - **What A' stored:**
       - V-0085's `GSTREGISTRATION` (and any attribute), `CMPGSTIN`, `CMPGSTSTATE`, `CMPGSTREGISTRATIONTYPE`, `PLACEOFSUPPLY` and `STATENAME`, as count19.py prints them;
       - RC-0085's `GSTREGISTRATION` (and any attribute) and `CMPGSTIN`.
       - Record them whole, beside V-0010's.
     - `count19.py --diff a-s3r-R-0085.xml a-s3r-RC-0085.xml`. EXPECT only `GSTREGISTRATION` and `CMPGSTIN` differ. Record the whole output.
     - `count19.py --diff a-s2-V-0010.xml a-s3r-V-0010.xml`: record whether the existing invoice's fields moved.
     - Record ALTVCHID and ALTMSTID against s3m's.
   - **A' is created** when the answer reads `CREATED` 1, `ERRORS` 0 and `EXCEPTIONS` 0, and N-0085 holds one row. Then go on to step 4.
   - **Any other answer** is a result: record it and go to step 5. B' is not sent.
   - Never send `I-0085.xml` again (C3).
4. **Import B', only if A' was created.**
   - **Before it, nothing A' changed may reach B'.** `s4m`: `T-types.xml U-taxunits.xml M-marks.xml`.
     - `count19.py --type-diff a-s2-T-types.xml a-s4m-T-types.xml`: EXPECT every line printed reads 0 leaves differ, and no type in one only.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s4m-U-taxunits.xml`: EXPECT 0 leaves differ in each unit.
     - EXPECT ALTMSTID equal to s3m's.
     - Record all three whole.
     - **If any differs** (for example `LASTGSTTAXUNIT` on `BRIDGE Sales` moved), B' would see a book A' changed. Send nothing more. The difference is recorded as a result of A', and B' goes to a sitting of its own. Go to step 5.
     - Otherwise `count19.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml`. It must pass.
   - `s4`: `I-0087.xml`, sent once. Then the wait rule. Record as step 3.
   - `s4r`: `N-0087.xml R-0087.xml RC-0087.xml V-0087.xml V-0085.xml V-0010.xml M-marks.xml`.
     - **What B' stored:** V-0087's six fields and RC-0087's two, recorded as step 3's.
     - `count19.py --diff a-s4r-R-0087.xml a-s4r-RC-0087.xml`. EXPECT only `GSTREGISTRATION` and `CMPGSTIN` differ.
     - `count19.py --diff a-s3r-V-0085.xml a-s4r-V-0085.xml`: record whether the invoice A' created moved.
     - `count19.py --diff a-s2-V-0010.xml a-s4r-V-0010.xml`: record whether `0010`'s moved.
     - Record ALTVCHID and ALTMSTID against s4m's.
   - Any answer at s4 is a result. Never send `I-0087.xml` again (C3).
5. **Restore** BK by C6, with `restore-plan.txt` written first. GN's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s5`: `M-marks.xml U-taxunits.xml T-types.xml N-0085.xml N-0087.xml R-0010.xml RC-0010.xml V-0010.xml`, and compare whole:
     - `count19.py --same-row a-s1-M-marks.xml a-s5-M-marks.xml`: EXPECT it passes.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml`: EXPECT the two units and 0 leaves differ in each.
     - `count19.py --type-diff a-s2-T-types.xml a-s5-T-types.xml`: EXPECT every line printed reads 0 leaves differ.
     - N-0085 and N-0087: 0 rows each.
     - `count19.py --diff a-s2-R-0010.xml a-s5-R-0010.xml` and `--diff a-s2-RC-0010.xml a-s5-RC-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
     - Record each answer's sha256 against s2's; EXPECT each equal.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
6. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those C5 and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds: a stop after step 3's send leaves whatever A' did, and after step 4's send, also whatever B' did.

## Record
- Each answer's bytes and sha256, and `count19.py`'s lines.
- The `--unit-diff`, `--type-diff`, `--diff`, `--before-write` and `--same-row` outputs, whole.
- The screenshots after each import's send, listed by name, bytes and sha256.
- As text and word for word: every key pressed, and every prompt, message and busy screen, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** byte-equal to GN's s2, as the Book and step 2 say.
- **Step 3:** not estimated. Both answers are plausible, and each decides something (Decides). GN's import, with the same element in the same place and form, was refused. The request it adds the element to was created on this book.
- **Step 4:** not estimated.
- **The status flags:** if A' is created, those of `BP/26-27/0010` (included Yes, every other flag No), since the unit fits the taxes. Not established.
- **s5:** equal to s2 and s1.

## Decides
Each is PARTIAL: one import of each shape, one sitting, one book, one release.

**For the write (A', then B')**
- **A' is created and stores `Rajasthan Registration`, with `CMPGSTIN`, the state and the type as `BP/26-27/0010` holds them:** the element, in this place and form, is taken on an import where it agrees with the invoice and the book.
  - GN's refusal then came from the unit disagreeing with the invoice or the book (causes 2 and 3), not from the element.
  - The next row is GN's Later rows' status question: two registrations, `BRIDGE Sales` bound to `Rajasthan Registration`, the invoice naming it.
  - B' then shows whether also writing `CMPGSTIN` changes what is stored.
- **A' is created and stores the registration fields some other way** (empty, or other values than `0010`'s): the element is accepted but not stored as named. Record all six whole; the coordinator decides.
- **A' is answered as GN's A was** (`CREATED` 0, `EXCEPTIONS` 1, no words): the element in this place or form is not taken, even where nothing disagrees with it (cause 1).
  - The next row is the form or the place, on this same book. One candidate is `TYPE="String"`, as the fetch printed it. Another is the place where Tally's own export puts the element. Each is one variable, and neither needs set-up.
  - Until a form is taken, no builder writes a registration, and a two-registration book stays refused (`invoice_company_registration_ambiguous`).
- **A' is refused with words** (an error or a LINEERROR): record them word for word. The element is not accepted, and the words are the first answer on why.
- **B' is created** while A' was: compare V-0087's six fields with V-0085's. Equal: writing the GSTIN changes nothing stored, and a builder may write the name alone or both. Different: record whole; the coordinator decides.
- **B' is refused** while A' was created: writing `CMPGSTIN` as well is not taken in this place or form, and a builder writes the name alone.
- **A' changed a master this row reads (step 4):** record which. The build would face that change on its next invoice. B' moves to its own sitting.
- **V-0010 or V-0085 moved at a later read:** a later import reached an earlier invoice. Record which field, and report before the restore.

**For the read-back (RC), only if A' or B' is created**
- **RC returns `GSTREGISTRATION` and `CMPGSTIN` equal to V's, with every other field equal to R's:** the candidate read-back reads a registration an import wrote, as it read the one Tally gave `BP/26-27/0010`.
- **RC returns them `<absent>` or empty while V holds them:** the collection does not return them for an imported registration. Record whole; the coordinator decides.

**Not decided here:** anything about a book with two registrations or a bound type. Here the named unit is the book's only one, so the row cannot show whether Tally checks it against the binding, the invoice's state, or both.

**What these answers are for.** They may inform #2464's design and a candidate builder. They are never a regression fixture for the build's output, because the build renders neither import.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build writes no registration.** `render_sales_invoice_xml` (`src-tauri/src/agent_import_invoice.rs:2664`, its template at `:2708`) writes `GSTREGISTRATIONTYPE` (the customer's) and no `GSTREGISTRATION`, `CMPGSTIN` or `CMPGSTSTATE`. Both imports here are therefore authored. This row does not change the builder.
- **The build's read-back fetches neither field.** `render_invoice_readback_request` (`src-tauri/src/agent_import_invoice_wire.rs:505`, its FETCH at `:514`) is `R-0010.xml`'s shape. `RC-` adds the two names and nothing else.
- **The build does not refuse this book as ambiguous.** With one GST unit in force, `parse_company_registration` takes its one-unit arm (`src-tauri/src/agent_import_invoice_wire.rs:800`), not the ambiguous one (`:804`). It then checks that unit's type and GSTIN. A builder that wrote the element would face the same checks: this row measures what that builder would rely on, before it is built.

## Later rows
- **If A' is created: the status question.** A sitting with two registrations, `BRIDGE Sales` bound to `Rajasthan Registration`, and the invoice naming it in this form. The taxes then fit the registration, and the flags can be judged.
- **If A' is refused: the form or the place**, one variable at a time, on this book.
- **The build's own rendered request,** re-run once a candidate builder writes the registration, and read back by the build's changed read-back. That run, not this row, is the first answer for the build's output.

## Not measured
- A book with two registrations, and a type bound to a registration.
- `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE` written by the request, and `GSTREGISTRATION` with attributes or in another place.
- A mismatched unit and GSTIN.
- An inter-state invoice, and GSTR-1.
- An invoice built and posted through Bridge.
- Any other book or release.

Row: lab/2464-nr/
