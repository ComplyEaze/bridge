# Row 2464-GN: in a book with two GST registrations, with BRIDGE Sales bound to the second, does an imported Sales invoice that names its registration keep it, and does a read-back that fetches the registration return it?

Shape discovery: the two imports and the candidate read-back are requests written for this row, not requests the build renders, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2464 (0.9.0), related to #2467 (gap 1) and #2162 (ADR 0004). Written 10 Nov 2026. Two master changes on Tally's screen (row 2464-GR's second sitting, repeated), two imports, then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
Row 2464-GR's second sitting (`lab/2464-gr-b-answers` at 01899d61, RESULTS.md) set up this book with two registrations and `BRIDGE Sales` bound to `Maharashtra Registration`. It then imported the build's invoice request, which names no registration. Tally created it and gave it none: `GSTREGISTRATION`, `CMPGSTIN`, `CMPGSTSTATE` and `CMPGSTREGISTRATIONTYPE` all empty. The build's read-back read it as uncertain. The read-back fetches neither registration field, so it could not say why. #2464 now needs two answers:
- **Write:** if the invoice names its registration, does Tally keep it, and fill the rest?
- **Read:** does a read-back that also fetches `GSTREGISTRATION` and `CMPGSTIN` return what Tally stored?

- **The one variable at each import.**
  - **Import A** is 46b's request (`I-0091.xml`) with one element added: `GSTREGISTRATION` naming `Maharashtra Registration`, the unit's name with no attributes.
  - **Import B** is import A with one more element added: `CMPGSTIN` `27ZZZZZ0000Z1ZQ`, that unit's GSTIN.
  - Each also takes a new number and REMOTEID, as every import does. Nothing else differs, byte for byte (Files).
- **Why two steps in one sitting, A before B.** Naming the unit alone, and naming the unit with its GSTIN, are separate questions:
  - A asks whether the unit's name is enough for Tally to fill the GSTIN itself, as it did on the one-registration book for `BP/26-27/0010`.
  - B asks what naming both stores. If A answers "enough", B shows whether the build's own GSTIN changes anything.
  - Run in one import, the two could not be told apart.
  - They are steps of one sitting rather than two sittings because the set-up costs the most: three named prompts and two master screens, each a place to stop. One set-up serves both imports.
  - What B sees beyond A's change is A's own invoice, under another number in the same type.
  - B is sent only if A changed no master that this row reads (step 6). If A changed one, the row stops before B, restores, and B goes to a sitting of its own.
- **The read-back.** `RC-` is the build's read-back with two names added at the end of its FETCH list: `GSTREGISTRATION` and `CMPGSTIN`. Nothing else is changed (Files).
  - It is sent beside the build's own `R-` each time, so the fetch change is the only difference between their answers.
  - It is also sent for `BP/26-27/0010` before any change, where the native read gives `Rajasthan Registration` and `08ZZZZZ0000Z1ZQ`. Its answer there shows whether this collection returns the two fields at all.
- **A known confound: the taxes do not fit the registration.** The invoice is `0010`'s intra-state Rajasthan sale: place of supply Rajasthan, with CGST and SGST legs.
  - Under `Maharashtra Registration`, a sale to Rajasthan is inter-state and would carry IGST.
  - So the GST status flags (`VCHGSTSTATUSIS*`) after either import cannot be put down to the registration: an uncertain status may come from the taxes.
  - The flags are recorded and not judged. This row decides what Tally stores and what the read-back returns, not whether naming the registration clears the uncertain status.
  - Changing the taxes or the place of supply here would be a second variable. The row that would answer the status question is under Later rows.
- **Not asked here:** a registration named with another unit's GSTIN (a mismatched pair), `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE` written by the request, GSTR-1, a type bound to the first registration, and the build's own rendered request (no builder writes these elements yet).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2464-GR's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count19.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is keyed unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. 46b started from those marks.
  - Step 4's compare with 46b's reads holds only from the same marks: the new unit's and the type's ALTERIDs follow from them.
  - So at s1 any other marks stop the row, before BK and before any key.
- **The numbers, `BP/26-27/0095` (A) and `BP/26-27/0097` (B), are chosen from the captured answers and the branches.**
  - Row 2162-S5's year read (`lab/2162-s5-answers` at ab3d2e77, `answers/a-s2-Y-all.xml`) lists every voucher of 1 Apr 2026 to 31 Mar 2027 at the restored state. The `BP/26-27/` numbers held are 0001 to 0016, 0019, 0020, 0021 and 0041 to 0044.
  - Every blob on every `lab/` branch (653 refs, 23,523 blobs, read as UTF-16 and as UTF-8) and on master was searched. None holds `26-27/0095` or `26-27/0097`, or the REMOTEID tails `000000000995` or `000000000997`.
  - `0089`, `0091` and `0093` were passed over:
    - row 2360's sheet uses `0089`;
    - row 2464-GR created `0091` and restored it away;
    - row 1379-IT's queued sheet sends `0093`.
  - Step 2's N reads stay a STOP, in case this check is wrong.
- **BK is this row's way back.**
  - It is taken at step 1, before anything is keyed, and restored at step 8 by C6.
  - The restore runs after step 3 in every case, including a stop.
  - After it, every read is compared whole with the reads before the change. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Row 2464-GR's sittings read TallyPrime with `tally.exe` 1.1.7.2, from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. The two master changes are made on Tally's screen, by hand. The writes are `I-0095.xml` and `I-0097.xml`, each sent once.

**The sends** are row 2464-GR's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count19.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count19.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.
- Answers that hold a GSTIN stay on the lab PC under the lab's standing rules, listed by size and sha256, as 46b's did. That covers U, V and, if it returns `CMPGSTIN`, RC.

## Files
**Copied from earlier rows**
- **Byte-equal to row 2464-GR's file of the same name** (`lab/2464-gr-sheet` at 3508daa0, folder `lab/2464-gr/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `U-taxunits.xml`, `T-types.xml`, `R-0010.xml`, `V-0010.xml`, `template-import-0010.utf16le.xml`, `template-full-0057.utf16le.xml` and `template-n-0001.utf16le.xml`.
  - `template-import-0010.utf16le.xml` is the build's request that created `BP/26-27/0010` (sha256 `b333f0dc`, equal to `PILOT_REQUEST_SHA256` on master).
  - `gen.py` writes `V-0010.xml` again, and asserts it equal to row 2464-GR's.

**46b's reads, for step 3's and step 4's compares**
- **Committed here:** 46b's committed voucher-type answers, byte for byte, from `lab/2464-gr-b-answers` at 01899d61, `answers/`, renamed so that `count19.py` counts them as T:

  | Here | 46b's answer | Bytes | sha256 begins |
  | --- | --- | --- | --- |
  | `a-46bs2-T-types.xml` | `a-s2-T-types.xml` | 306,946 | `cfd36272` |
  | `a-46bs3-T-types.xml` | `a-s3-T-types.xml` | 306,946 | `3239e41d` |
  | `a-46bs4-T-types.xml` | `a-s4-T-types.xml` | 307,042 | `16bbd91c` |

- **On the lab PC:** 46b's tax-unit answers stayed there (they hold GSTINs). Before step 1, copy them beside this row's answers under these names, and check each sha256 first:

  | Name here | 46b's file | Bytes | sha256 begins |
  | --- | --- | --- | --- |
  | `a-46bs3-U-taxunits.xml` | `a-s3-U-taxunits.xml` | 33,304 | `2ec479c6` |
  | `a-46bs4-U-taxunits.xml` | `a-s4-U-taxunits.xml` | 33,304 | `508dd8de` |

  - The full sha256 values are in 46b's `kept-local.txt`.
  - If either is missing or its sha256 differs: STOP before step 1. Step 4's STOP cannot be checked without it.

**`gen.py`** writes each request below, with only the change it names, each asserted to occur exactly as often as expected. Each template's sha256 is checked first. As bytes against the build's `0010` request (`template-import-0010.utf16le.xml`):
- **`I-0095.xml` (A):**
  - `VOUCHERNUMBER` and `REFERENCE` `BP/26-27/0010` become `BP/26-27/0095`;
  - the REMOTEID's last group `000000000777` becomes `000000000995`;
  - one element is inserted right after `<PARTYNAME>BRIDGE Walk-in</PARTYNAME>`: `<GSTREGISTRATION>Maharashtra Registration</GSTREGISTRATION>`.
  - Nothing else. Against 46b's `I-0091.xml`, the number and the tail differ, and the one element is added.
- **`I-0097.xml` (B):**
  - the number `BP/26-27/0097` and the tail `000000000997`;
  - the same `GSTREGISTRATION`, then `<CMPGSTIN>27ZZZZZ0000Z1ZQ</CMPGSTIN>`, both right after `PARTYNAME`.
  - Against `I-0095.xml`: the number, the tail, and the `CMPGSTIN` element.
- **Where the elements go:** the place and form follow Tally's own native read of `BP/26-27/0010` (the `pilot-lab-vouchers-after-post` fixture on master). That read gives `PARTYNAME`, then `GSTREGISTRATION` (the unit's name, no attributes), then `CMPGSTIN` (the GSTIN), as three consecutive children.
  - `gen.py` parses each import and asserts that order.
  - It also asserts that each added element carries no attribute.
  - It asserts that neither import names `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE`.
- **`RC-0010.xml`, `RC-0095.xml`, `RC-0097.xml`:** `R-0010.xml` with `, ISPOSTDATED</FETCH>` changed to `, ISPOSTDATED, GSTREGISTRATION, CMPGSTIN</FETCH>`, then with the number `0095` and `0097`. Its filter, collection, type GUID and every other fetch name are the build's.
- **`N-0095.xml`, `N-0097.xml`, `R-0095.xml`, `R-0097.xml`, `V-0095.xml`, `V-0097.xml`:** the build's number read, the build's read-back and the native read of one voucher by number, each with the new number.

**`count19.py`** is row 1379-IT's `count18.py` (`lab/1379-it-sheet` at bab72dc8) with one change, marked GN.
- The R kind also prints `GSTREGISTRATION` (its text, and any attribute it carries) and `CMPGSTIN`, each counted as an element.
- An `RC-` answer is counted as R.
- On the build's read-back both print `<absent>`, since it does not fetch them.
- Tried once on a captured answer of every kind this row reads, each exit 0:
  - M: 46b's `a-s1` and `a-s6`;
  - U: row 2162-RU's `a-s2-U-taxunits.xml` and `a-sC2-U-taxunits.xml` (`lab/2162-ru-answers` at c7b14084), one registration and two;
  - T: 46b's s2 and s4;
  - N: 46b's s2 and s5r;
  - I: 46b's s5, and row 2162-BC's `a-s4a-I-0053.xml`;
  - R: 46b's s2, s5r and s6;
  - V: `0010`'s voucher cut from the `pilot-lab-vouchers-after-post` fixture and re-encoded as UTF-16LE, as row 2464-GR tried its count16.
- The modes `--type-diff`, `--unit-diff`, `--diff`, `--before-write` and `--same-row` were each tried on the same answers.
- 46b's s2 against s4 by `--type-diff` printed `ALTERID` 212 to 248 and `DEFAULTGSTREGISTRATION` `''` to `Maharashtra Registration`: the two leaves 46b's own count16 printed for s3 against s4.
- No captured answer holds a read-back with the two fields. The R kind's new lines were tried on `0010`'s native answer counted as R: `Rajasthan Registration`, `08ZZZZZ0000Z1ZQ`, no attributes. That checks the lines, not the request.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count19.py a-<label>-*`. A STOP from either script stops the row there.

**The prompt rule, at every step.** The only prompts answered are C5's, C6's and step 3's three named prompts, each with the key named.
- For any other prompt, message or dialog: copy it word for word and screenshot it first.
- Then answer No, or Esc if it offers no No. Press nothing more and send nothing.
- Then leave any screen still open by Ctrl+Q (C7), restore BK by C6, compare as step 8, and close (C10).
- That prompt is this row's result.

**The wait rule.** After each Ctrl+A, and after each import's send: if 5 minutes pass with no list, no Gateway, no answer and no message:
- send nothing, press nothing, screenshot the whole window, and report to the run coordinator;
- a read sent while Tally is busy can hold its gateway.
- With the Gateway showing and no dialog, run C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

**The path rule.** Every screen in steps 3 and 4 is as 46b recorded it (`lab/2464-gr-b-answers` at 01899d61, RESULTS.md and `evidence/notes.txt`, steps 3.1 to 4.4), which follow row 2162-RU's `evidence/sC-screens.txt` (`lab/2162-ru-answers` at c7b14084).
- If the line or screen shown is not exactly what the step names: stop, then Esc out of a list or prompt (never a creation or alteration screen, C7).
- Restore BK by C6, check as step 8, then Quit (C10).

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down. 46b read 1-Apr-26 to 31-Mar-27 and Tuesday 15-Sep-2026.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245.
     - Other marks: stop here, before BK, with nothing keyed (the Book's state bullet says why).
   - Check the two `a-46bs*-U-taxunits.xml` copies (Files).
   - **Take BK** by C5, to the new empty folder `C:\lab\2464-gn\backup-1`. 46b's step 1 is the path.
     - Tally may show the folder with its own capital, as `C:\lab\2464-Gn\backup-1`.
     - Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml T-types.xml N-0095.xml N-0097.xml R-0010.xml RC-0010.xml V-0010.xml M-marks.xml`.
   - **U:** EXPECT byte-equal to 46b's s2 answer (22,636 bytes, sha256 beginning `e40ef83c`): two units, `Rajasthan Registration` the one GST unit.
   - **T:** `count19.py --type-diff a-46bs2-T-types.xml a-s2-T-types.xml`: EXPECT every line printed reads 0 leaves differ, and no type in one only.
   - **N-0095 and N-0097:** EXPECT 0 rows each. One row or more in either: stop before step 3, with each row recorded whole.
   - **R-0010:** EXPECT one row, `BRIDGE Walk-in`, included Yes, every other status flag No, 6 legs, and `GSTREGISTRATION` and `CMPGSTIN` `<absent>`.
   - **RC-0010:** record its one row whole: the status flags, `GSTREGISTRATION` (and any attribute) and `CMPGSTIN`.
     - Then `count19.py --diff a-s2-R-0010.xml a-s2-RC-0010.xml`. EXPECT `GSTREGISTRATION` and `CMPGSTIN` the only fields that differ, each from `<absent>`. Record the whole output.
     - RC-0010's two fields are a result, whatever they read, and the row goes on. They are this row's first answer on whether the collection returns them.
   - **V-0010:** EXPECT one VOUCHER: `GSTREGISTRATION` `Rajasthan Registration` (no attributes), `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, `CMPGSTSTATE` `Rajasthan`, `CMPGSTREGISTRATIONTYPE` `Regular`, as 46b's s2 read.
     - Anything else: record it whole and stop before step 3.
3. **Change 1: a second GST registration** (46b's step 3, which is row 2162-RU's part C2).
   - Before it, `s3m`: `M-marks.xml`, then `count19.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass.
   - Then no key is pressed in Tally but those below.
   1. At the Gateway (shot): Create, Enter. `Master Creation`, the List of Masters.
      - Down 9 to Godown, once to the heading, once more: `GST Registration` highlighted (zoom and check). Enter (shot).
   2. **Named prompt 1.** EXPECT, word for word, "GST Registration already exists for the Company." then "Do you want to alter the existing GST Registration or create a new GST Registration?", with `C Create New` and `A Alter Existing` (shot, zoomed).
      - If the words are the same: press `c`. If they differ: the prompt rule.
   3. `GST Registration Creation` (shot). Write down every field as Tally filled it.
      - EXPECT 46b's 3.3: Registration status Active, State empty, Registration type Regular, Assessee of Other Territory No, GSTIN/UIN empty, Periodicity of GSTR-1 Monthly, the rest as 46b listed.
   4. Fill the form:
      - **State:** type `Maha`. EXPECT the List of States with `Maharashtra` the one state line; Enter.
      - **Registration type:** its list Regular, Composition, Regular - SEZ. Down once: `Regular` highlighted; zoom and check; Enter.
      - **Assessee of Other Territory:** No, Enter.
      - **GSTIN/UIN:** type `27ZZZZZ000`, then `0Z1ZQ`, zooming after each. Before any Enter, check it reads `27ZZZZZ0000Z1ZQ`, 15 characters, capitals (shot).
      - Then Enter on each later field, leaving it as Tally filled it, as 46b's 3.4 did.
   5. **Named prompt 2.** On the last field's Enter, EXPECT `Accept ?` (Yes or No) (shot). Press `y`.
   6. **Named prompt 3.** EXPECT these three paragraphs, word for word, as 46b recorded them on screen (shot, zoomed), with Yes or No:
      - "You can create a unique voucher numbering series to cater to multiple GST registrations."
      - "You can do this by going to the Chart of Accounts and selecting the relevant voucher types"
      - "Open Chart of Accounts and create now?"
      - The second paragraph has no full stop. If the words and punctuation are the same: press `n`. If they differ: the prompt rule.
   7. EXPECT a new, empty `GST Registration Creation` form (46b's 3.7). Type nothing in it.
      - Ctrl+Q, then Ctrl+Q again: `Master Creation`, then the Gateway (shot). Two Ctrl+Q in all.
   - Then `s3`: `U-taxunits.xml T-types.xml M-marks.xml`.
     - `count19.py --unit-diff a-46bs3-U-taxunits.xml a-s3-U-taxunits.xml`. EXPECT the names `Default Tax Unit`, `Maharashtra Registration` and `Rajasthan Registration` on both sides, and 0 leaves differ in each.
     - `count19.py --type-diff a-46bs3-T-types.xml a-s3-T-types.xml`. EXPECT every line printed reads 0 leaves differ, and no type in one only.
     - Record both outputs whole, and ALTMSTID (46b: 246).
     - **Any leaf that differs from 46b's stops the row here:** press nothing more, restore BK by C6, check as step 8, and close.
4. **Change 2: bind BRIDGE Sales to the second registration** (46b's step 4, which is row 2162-RU's part C3 with the other line).
   - Before it, `s4m`: `M-marks.xml`; record ALTMSTID against s3's.
   1. At the Gateway (shot), Go To (click its label).
      - Up 4 to `Alter Master`. 46b: the first Up lit no visible row; then Show More, Expand All, Alter Master, each zoomed. Enter.
      - `Master Alteration`; Down 3 to `Voucher Type`, Enter.
      - `List of Voucher Types`: Enter on `BRIDGE Sales` only (shot).
   2. `Voucher Type Alteration` (shot). Write down every field, as text. EXPECT 46b's 4.2, with `Default GST Registration` reading `Any`.
   3. Click the value of `Default GST Registration`.
      - EXPECT the list `List of GST Registrations`: Create; Any (highlighted); Maharashtra Registration; Rajasthan Registration.
      - **Down once:** `Maharashtra Registration` highlighted; zoom and check; Enter (shot).
      - EXPECT, as 46b: the cursor moves to `Use common voucher numbering series for all GST Registrations: No`, with that field's own Yes / No list open and No highlighted. **Press no key on that list, and touch no other field.**
   4. Ctrl+A. EXPECT no prompt, and the List of Voucher Types (46b's 4.4). Any prompt: the prompt rule.
      - Ctrl+Q once: the Gateway (shot).
   - Then `s4`: `T-types.xml U-taxunits.xml M-marks.xml`. Record ALTMSTID (46b: 248).
   - **STOP before the import if the type or unit reads differ from 46b's.** Both of these must hold, or the row stops here:
     - `count19.py --type "BRIDGE Sales" --type-diff a-46bs4-T-types.xml a-s4-T-types.xml`. EXPECT `type 'BRIDGE Sales': 0 leaves differ`, no other type line, and no type in one only. That is `DEFAULTGSTREGISTRATION` `Maharashtra Registration`, `LASTGSTTAXUNIT` and `LASTGSTTAXUNITBYSERIES` `Rajasthan Registration`, `TAXUNITNAME` `Any`, `ALTERID` 248, as 46b's s4 read.
     - `count19.py --unit-diff a-46bs4-U-taxunits.xml a-s4-U-taxunits.xml`. EXPECT the same three names on both sides and 0 leaves differ in each.
   - Record both outputs whole.
   - If either shows a difference: send nothing, press nothing more, restore BK by C6, check as step 8, and close. The differing lines are this row's result, and the coordinator decides.
5. **Import A: the registration's name alone.**
   - `s5m`: `M-marks.xml`, then `count19.py --before-write a-s1-M-marks.xml a-s5m-M-marks.xml`. It must pass.
   - Then no key is pressed in Tally before the send.
   - `s5`: `I-0095.xml`, sent once. Then the wait rule.
     - Record every counter and any LINEERROR, word for word.
     - The window after it (shot): the Gateway, no dialog. A dialog: the prompt rule.
   - `s5r`: `N-0095.xml R-0095.xml RC-0095.xml V-0095.xml V-0010.xml M-marks.xml`.
     - Record N-0095's rows, each with its GUID.
     - Record R-0095's rows: ledger, the three status flags and the legs.
     - **A's answers:**
       - V-0095's `GSTREGISTRATION` (and any attribute), `CMPGSTIN`, `CMPGSTSTATE`, `CMPGSTREGISTRATIONTYPE`, `PLACEOFSUPPLY` and `STATENAME`, as count19.py prints them;
       - RC-0095's `GSTREGISTRATION` (and any attribute) and `CMPGSTIN`.
       - Record them whole, beside V-0010's and 46b's V-0091.
     - `count19.py --diff a-s5r-R-0095.xml a-s5r-RC-0095.xml`. EXPECT only `GSTREGISTRATION` and `CMPGSTIN` differ. Record the whole output.
     - `count19.py --diff a-s2-V-0010.xml a-s5r-V-0010.xml`: record whether the existing invoice's fields moved.
     - Record ALTVCHID and ALTMSTID against s5m's.
   - Any answer at s5 is a result, created or not. Record it and go on to step 6. Never send `I-0095.xml` again (C3).
6. **Before import B: nothing A changed may reach B.** `s6m`: `T-types.xml U-taxunits.xml M-marks.xml`.
   - `count19.py --type-diff a-s4-T-types.xml a-s6m-T-types.xml`: EXPECT every line printed reads 0 leaves differ, and no type in one only.
   - `count19.py --unit-diff a-s4-U-taxunits.xml a-s6m-U-taxunits.xml`: EXPECT 0 leaves differ in each unit.
   - EXPECT ALTMSTID equal to s4's (46b's import left it unmoved).
   - Record all three whole.
   - **If any differs** (for example `LASTGSTTAXUNIT` on `BRIDGE Sales` moved to `Maharashtra Registration`), B would see a book A changed:
     - send nothing more, restore BK by C6, check as step 8, and close;
     - the difference is recorded as A's result;
     - B goes to a sitting of its own, which repeats steps 1 to 4 and sends B in place of A.
   - Otherwise `count19.py --before-write a-s1-M-marks.xml a-s6m-M-marks.xml`. It must pass. No key is pressed in Tally before the send.
7. **Import B: the registration's name and its GSTIN.**
   - `s7`: `I-0097.xml`, sent once. Then the wait rule. Record as step 5.
   - `s7r`: `N-0097.xml R-0097.xml RC-0097.xml V-0097.xml V-0095.xml V-0010.xml M-marks.xml`.
     - **B's answers:** V-0097's six fields and RC-0097's two, recorded as step 5's.
     - `count19.py --diff a-s7r-R-0097.xml a-s7r-RC-0097.xml`. EXPECT only `GSTREGISTRATION` and `CMPGSTIN` differ.
     - `count19.py --diff a-s5r-V-0095.xml a-s7r-V-0095.xml`: record whether A's invoice moved.
     - `count19.py --diff a-s2-V-0010.xml a-s7r-V-0010.xml`: record whether `0010`'s moved.
     - Record ALTVCHID and ALTMSTID against s6m's.
   - Any answer at s7 is a result. Never send `I-0097.xml` again (C3).
8. **Restore** BK by C6, with `restore-plan.txt` written first. 46b's step 6 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s8`: `M-marks.xml U-taxunits.xml T-types.xml N-0095.xml N-0097.xml R-0010.xml RC-0010.xml V-0010.xml`, and compare whole:
     - `count19.py --same-row a-s1-M-marks.xml a-s8-M-marks.xml`: EXPECT it passes.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml`: EXPECT the two units and 0 leaves differ in each.
     - `count19.py --type-diff a-s2-T-types.xml a-s8-T-types.xml`: EXPECT every line printed reads 0 leaves differ.
     - N-0095 and N-0097: 0 rows each.
     - `count19.py --diff a-s2-R-0010.xml a-s8-R-0010.xml` and `--diff a-s2-RC-0010.xml a-s8-RC-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
     - Record each answer's sha256 against s2's; EXPECT each equal.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule, step 3 and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds:
  - a stop between steps 3 and 8 leaves the second registration;
  - from step 4, also the binding;
  - from step 5 and step 7, also whatever each import did.

## Record
- Each answer's bytes and sha256, and `count19.py`'s lines.
- The `--unit-diff`, `--type-diff`, `--diff`, `--before-write` and `--same-row` outputs, whole.
- Steps 3, 4, 5 and 7's screenshots, listed by name, bytes and sha256.
- As text and word for word: the creation and alteration screens' fields, every key pressed, and every prompt, message and busy screen, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say.
- **RC-0010:** the two fields returned, `Rajasthan Registration` and `08ZZZZZ0000Z1ZQ`, with every other field equal to R-0010's. This is not established: no read-back has fetched them.
- **Steps 3 and 4:** 46b's: the three named prompts, a third unit, the binding, and every type and unit leaf equal to 46b's.
- **Step 5:** created. The import shape is 46b's, which Tally took, with one element added in the place Tally's own read gives it. What Tally stores in the four registration fields is not estimated: that is the row's question.
- **Step 6:** nothing moved. In 46b ALTMSTID was unmoved by the import, but that import named no registration.
- **Step 7:** created. What Tally stores is not estimated.
- **The status flags:** not estimated. Under a Maharashtra registration the taxes do not fit (the Question's confound).
- **s8:** equal to s2 and s1.

## Decides
Each is PARTIAL: one import of each shape, one sitting, one book, one release.

**For the write (A, then B)**
- **A stores `Maharashtra Registration` and Tally fills `CMPGSTIN` `27ZZZZZ0000Z1ZQ` (and the state and type):** on this release, naming the unit is enough. Tally fills the GSTIN from the unit, as it did unasked on the one-registration book.
  - A candidate builder could write `GSTREGISTRATION` alone.
  - B then shows whether also writing `CMPGSTIN` changes what is stored.
- **A stores `Maharashtra Registration` with `CMPGSTIN` empty:** naming the unit does not fill the GSTIN.
  - If B stores both as sent, a builder must write both.
  - That leaves open whether Tally checks one against the other (a mismatched pair is not sent here).
- **A stores no registration (`GSTREGISTRATION` empty or absent):** Tally ignores the name on an import in this place and form.
  - If B also stores none, neither element is taken as sent here.
  - The next candidate is another form, for example Tally's attributed form, measured first.
  - Until one is taken, a two-registration book stays refused (`invoice_company_registration_ambiguous`).
- **A or B stores a registration other than the one named, or fields that disagree** (for example the unit's name and another unit's GSTIN): record all six whole. Neither field can be relied on alone; the coordinator decides.
- **A or B is refused** (`CREATED` 0, an error or a LINEERROR): record the words. The element, its place or its form is not accepted on this release.
- **A changed a master this row reads (step 6):** record which. The build would face that change on its next invoice. B moves to its own sitting.
- **V-0010 or V-0095 moved at a later read:** a later import reached an earlier invoice. Record which field, and report before the restore.

**For the read-back (RC)**
- **RC returns `GSTREGISTRATION` and `CMPGSTIN` equal to V's, each time, with every other field equal to R's:** a read-back with these two fetch names reads what Tally stored.
  - The build's read-back can then compare them with what it wrote, as #2464 (the read-back's verdict) and #2467 (gap 1) need.
  - That change is to an existing builder, and its first answer is this one.
- **RC returns them `<absent>` or empty while V holds them:** this collection does not carry the two fields by these names. The read-back needs another shape, measured first.
- **RC's other fields differ from R's:** adding fetch names changed the answer. Record the whole `--diff`; the coordinator decides.

**Not decided here:** whether naming the registration clears `VCHGSTSTATUSISUNCERTAIN`. The taxes do not fit the bound registration, so the flags are recorded and not judged.

**What these answers are for.** They may inform #2464's design and a candidate builder. They are never a regression fixture for the build's output, because the build renders neither import.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build writes no registration.** `render_sales_invoice_xml` (`src-tauri/src/agent_import_invoice.rs:2664`, its template at `:2708`) writes `GSTREGISTRATIONTYPE` (the customer's) and no `GSTREGISTRATION`, `CMPGSTIN` or `CMPGSTSTATE`. Both imports here are therefore authored. This row does not change the builder.
- **The build's read-back fetches neither field.** `render_invoice_readback_request` (`src-tauri/src/agent_import_invoice_wire.rs:505`, its FETCH at `:514`) is `R-0010.xml`'s shape. `RC-` adds the two names and nothing else.
- **Today the build refuses this book.** With two GST units in force, `parse_company_registration` refuses as `invoice_company_registration_ambiguous` (`src-tauri/src/agent_import_invoice_wire.rs:804`). So no build reaches step 5's state. The row measures what a candidate builder would rely on, before it is built.

## Later rows
- **The status question.** A sitting with `BRIDGE Sales` bound to `Rajasthan Registration`, two registrations in force, and the invoice naming `Rajasthan Registration` in the form this row finds is taken. The taxes then fit the registration, and the flags can be judged.
- **A mismatched pair:** `GSTREGISTRATION` one unit and `CMPGSTIN` the other's GSTIN, which asks whether Tally checks one against the other.
- **The build's own rendered request,** re-run once a candidate builder writes the registration, and read back by the build's changed read-back. That run, not this row, is the first answer for the build's output.

## Not measured
- A type bound to the first registration, an unbound type with two registrations, and `Use common voucher numbering series for all GST Registrations` set to Yes.
- `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE` written by the request, and `GSTREGISTRATION` with attributes.
- An inter-state invoice with IGST under the bound registration, and GSTR-1 for either registration.
- An invoice built and posted through Bridge.
- Any other book or release.

Row: lab/2464-gn/
