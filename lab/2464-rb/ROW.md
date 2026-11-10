# Row 2464-RB: on the book with two GST registrations and `BRIDGE Sales` bound to `Rajasthan Registration`, is an imported Sales invoice that names that registration created, does it keep it, and is it included in GSTR-1?

Shape discovery: the two imports and the candidate read-back are requests written for a row, not requests the build renders, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2464 (0.9.0), related to #2467 (gap 1). Written 10 Nov 2026. Two master changes made on Tally's screen, one import (a second only if the first is created and changed nothing it may not), then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
Three rows have met this invoice, each changing one thing:
- **Row 2464-GR, second sitting** (`lab/2464-gr-b-answers` at 01899d61): two registrations, `BRIDGE Sales` bound to `Maharashtra Registration`, and the build's request, which names no registration. Created, with every registration field empty, and the GSTR-1 status uncertain.
- **Row 2464-GN** (`lab/2464-gn-answers` at aa2ff2e6): the same book, the same request with `<GSTREGISTRATION>Maharashtra Registration</GSTREGISTRATION>` added after `PARTYNAME`. Refused: `CREATED` 0, `EXCEPTIONS` 1, no words. That invoice is intra-state Rajasthan, so under Maharashtra its taxes do not fit.
- **Row 2464-NR** (`lab/2464-nr-answers` at 77d3f33b): the restored book (one registration, `BRIDGE Sales` unbound), the same element naming `Rajasthan Registration`. Created, the registration stored as `BP/26-27/0010` holds it, included in GSTR-1, and the candidate read-back returned both fields.

So the element, in that place and form, is taken where nothing disagrees with it (NR). This row asks the status question GN's Later rows named: on the two-registration book, with the type bound to the unit whose state the invoice fits, and the invoice naming that unit, is it created, stored, and included?

- **The set-up is GN's, with the binding on the other unit.** Change 1 is GN's step 3 (a second registration, `Maharashtra Registration`), key for key. Change 2 is GN's step 4 with one key different: `BRIDGE Sales`' Default GST Registration is set to `Rajasthan Registration`, not `Maharashtra Registration`.
- **Import A''** is NR's created A', renumbered `BP/26-27/0105`, and nothing else (Files). So against NR, the invoice is the same and the book differs by the two changes. Against GN, the book differs in the binding, and the invoice in the unit it names: both now the unit the invoice fits.
- **Import B'', its own step, only if A'' is created** and changed nothing step 6 forbids. It is NR's B' renumbered `BP/26-27/0107`: A'' with `<CMPGSTIN>08ZZZZZ0000Z1ZQ</CMPGSTIN>` (that unit's GSTIN) added after the element. NR did not send B': its step 4 rule stopped on the type's own `LASTNUMBER` moving after A' was created. Step 6 here allows that one move and nothing else.
- **The read-back.** `RC-` is the build's read-back with `GSTREGISTRATION` and `CMPGSTIN` added at the end of its FETCH list, byte-equal to GN's and NR's. On a created invoice here, it is that shape's first answer on a two-registration invoice.
- **Not asked here:** a type bound to one unit with the invoice naming another, an unbound type on the two-registration book, `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE` written by the request, `GSTREGISTRATION` with an attribute or in another place, an inter-state invoice, GSTR-1 itself on screen, and the build's own rendered request (no builder writes these elements yet).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2464-GN's and 2464-NR's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count19.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is keyed unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. GN's s1 and NR's s1 read those marks.
  - Step 2 compares every read with GN's s2 answer of the same kind, and step 3 with GN's s3, byte for byte or leaf for leaf. Those compares hold only from the same marks: the new unit's and the type's ALTERIDs follow from them.
  - So at s1 any other marks stop the row, before BK and before any key.
- **The numbers, `BP/26-27/0105` (A'') and `BP/26-27/0107` (B''), are chosen from the captured answers and the branches.**
  - Row 2162-S5's year read (`lab/2162-s5-answers` at ab3d2e77, `answers/a-s2-Y-all.xml`) lists every voucher of 1 Apr 2026 to 31 Mar 2027 at the restored state. The `BP/26-27/` numbers held are 0001 to 0016, 0019, 0020, 0021 and 0041 to 0044.
  - Every blob on every `lab/` branch (668 refs, 23,974 blobs, read as UTF-16 and as UTF-8) and on master was searched on 10 Nov 2026. None holds `26-27/0105`, `26-27/0107`, `26-27/0109` or `26-27/0111`, or the REMOTEID tails `000000001105` or `000000001107`.
  - The numbers below `0100` were passed over: rows 2360, 2464-GR, 2464-GN, 2464-NR and 1379-IT, and other queued rows, use several of them.
  - Step 2's N reads stay a STOP, in case this check is wrong.
- **BK is this row's way back, and the restore always runs.**
  - BK is taken at step 1, before anything is keyed, and restored at step 8 by C6.
  - Every path from step 3 on that is not a STOP reaches step 8. That includes a set-up compare failing, A'' declined, A'' created with B'' not sent, and B'' sent.
  - After a STOP (C7, C8, C9), C8 holds: nothing more is sent or keyed, and BK is restored only on the run coordinator's word.
  - After the restore, every read is compared with step 2's. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. GN and NR read TallyPrime with `tally.exe` 1.1.7.2, from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. The two master changes are made on Tally's screen, by hand. The writes are `I-0105.xml` and `I-0107.xml`, each sent at most once.

**The sends** are row 2464-GN's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count19.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count19.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.
- Answers that hold a GSTIN stay on the lab PC under the lab's standing rules, listed by size and sha256, as GN's and NR's did. That covers U, V and RC.

## Files
**Copied from row 2464-NR**
- **Byte-equal to NR's file of the same name** (`lab/2464-nr-sheet` at de14c060, folder `lab/2464-nr/`), which are themselves byte-equal to GN's, whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count19.py`, `M-marks.xml`, `U-taxunits.xml`, `T-types.xml`, `R-0010.xml`, `RC-0010.xml`, `V-0010.xml`, `template-import-0010.utf16le.xml`, `template-full-0057.utf16le.xml` and `template-n-0001.utf16le.xml`.
  - `template-import-0010.utf16le.xml` is the build's request that created `BP/26-27/0010` (sha256 `b333f0dc`, equal to `PILOT_REQUEST_SHA256` on master).
- **`count19.py` is GN's and NR's, unchanged.** It counts every kind this row reads (M, U, T, N, I, R, and V), with an `RC-` answer counted as R. GN ran each of its modes on its own answers, `--type-diff` and `--unit-diff` across the same two changes.
- **Byte-equal to GN's file of the same name** (`lab/2464-gn-sheet` at 23b2e57b): `a-46bs2-T-types.xml` and `a-46bs3-T-types.xml`, row 2464-GR second sitting's type answers before any change and after the second registration. GN's own s3 type answer (`lab/2464-gn-answers`, `answers/a-s3-T-types.xml`) is byte-equal to `a-46bs3-T-types.xml` (sha256 `3239e41d`).

**What steps 2 and 3 compare with.**
- **Step 2:** GN's s2 answers, as NR's step 2 did. NR's s2 answers were byte-equal to them, kind for kind. The table is NR's.

| Read here | GN's s2 answer | Bytes | sha256 | Where |
| --- | --- | --- | --- | --- |
| `M-marks.xml` | `a-s2-M-marks.xml` | 3,394 | `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6` | committed |
| `T-types.xml` | `a-s2-T-types.xml` | 306,946 | `cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed` | committed |
| `N-0105.xml`, `N-0107.xml` | `a-s2-N-0095.xml` | 3,022 | `947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974` | committed |
| `R-0010.xml` | `a-s2-R-0010.xml` | 94,474 | `b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5` | committed |
| `U-taxunits.xml` | `a-s2-U-taxunits.xml` | 22,636 | `e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989` | lab PC |
| `RC-0010.xml` | `a-s2-RC-0010.xml` | 94,744 | `5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399` | lab PC |
| `V-0010.xml` | `a-s2-V-0010.xml` | 52,784 | `4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456` | lab PC |

- An N answer with no rows does not carry the number, so both N reads are compared with the one answer.
- **Step 3:** GN's s3 answers after the same change: `a-46bs3-T-types.xml` (here), and GN's `a-s3-U-taxunits.xml` (33,304 bytes, sha256 `2ec479c6d234d5d5d130a362268c042adf35a05a6993d6c09ad71071519196f1`, lab PC, equal to row 2464-GR second sitting's).
- **Step 4 has no earlier answer to compare with.** No row has bound `BRIDGE Sales` to `Rajasthan Registration`. Step 4 compares its reads with step 3's and names the two leaves it expects to move. Row 2464-GR's second sitting, binding to the other unit, moved exactly two on `BRIDGE Sales`: `ALTERID` `212` to `248`, and `DEFAULTGSTREGISTRATION` `''` to the unit's name (its `evidence/typediff-s3-s4.txt`).
- **Before step 1,** copy the four lab-PC answers above beside this row's answers, as `a-gns2-U-taxunits.xml`, `a-gns2-RC-0010.xml`, `a-gns2-V-0010.xml` and `a-gns3-U-taxunits.xml`, checking each sha256 first. Also copy GN's committed `a-s2-T-types.xml` and `a-s2-R-0010.xml` as `a-gns2-T-types.xml` and `a-gns2-R-0010.xml`.
  - If any is missing or its sha256 differs: STOP before step 1. Steps 2 and 3 cannot be checked without them.

**`gen.py`** is NR's with the two numbers and their tails changed, and nothing else. It asserts first that the same functions, given NR's numbers and tails, write NR's `I-0085.xml` (sha256 `d8100a06`) and `I-0087.xml` (`76bc9be0`) byte for byte, and GN's `I-0095.xml` (`d955b163`). Each template's sha256 is checked first.
- **`I-0105.xml` (A''):** the template with `VOUCHERNUMBER` and `REFERENCE` `BP/26-27/0105`, the REMOTEID's last group `000000001105`, and `<GSTREGISTRATION>Rajasthan Registration</GSTREGISTRATION>` right after `<PARTYNAME>BRIDGE Walk-in</PARTYNAME>`. Against NR's created `I-0085.xml`: the number and the tail.
- **`I-0107.xml` (B''):** the number `BP/26-27/0107`, the tail `000000001107`, the same `GSTREGISTRATION`, then `<CMPGSTIN>08ZZZZZ0000Z1ZQ</CMPGSTIN>`. Against NR's `I-0087.xml`: the number and the tail.
- `gen.py` parses each import and asserts the elements' order (`PARTYNAME`, the added ones, `BASICBASEPARTYNAME`), that no added element carries an attribute, that neither import names `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE`, and that taking the added elements out leaves the template renumbered.
- **`RC-0105.xml`, `RC-0107.xml`:** `RC-0010.xml` (asserted byte-equal to GN's) with the new number.
- **`N-0105.xml`, `N-0107.xml`, `R-0105.xml`, `R-0107.xml`, `V-0105.xml`, `V-0107.xml`:** the build's number read, the build's read-back and the native read of one voucher by number, each with the new number. `V-0010.xml` is asserted byte-equal to GN's.

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

**The path rule.** Every screen in steps 3 and 4 is as GN recorded it (`lab/2464-gn-answers` at aa2ff2e6, RESULTS.md and `evidence/notes.txt`), which follow row 2464-GR second sitting's steps 3.1 to 4.4.
- If the line or screen shown is not exactly what the step names: stop, then Esc out of a list or prompt (never a creation or alteration screen, C7).
- Restore BK by C6, check as step 8, then Quit (C10).

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down. GN and NR read 1-Apr-26 to 31-Mar-27 and Tuesday 15-Sep-2026.
   - Check the six `a-gns*` copies (Files).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245.
     - Other marks: stop here, before BK, with nothing keyed (the Book's state bullet says why).
   - **Take BK** by C5, to the new empty folder `C:\lab\2464-rb\backup-1`. GN's step 1 is the path.
     - Tally may show the folder with its own capital, as `C:\lab\2464-Rb\backup-1`.
     - Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml T-types.xml N-0105.xml N-0107.xml R-0010.xml RC-0010.xml V-0010.xml M-marks.xml`.
   - EXPECT each answer byte-equal to GN's s2 answer of the same kind (Files).
   - So: two units, `Rajasthan Registration` the one GST unit; `BRIDGE Sales` unbound; 0 rows for each number; `BP/26-27/0010` with `Rajasthan Registration` and `08ZZZZZ0000Z1ZQ` in V and RC.
   - **If any answer differs:** compare it and record the whole output. For U, T, R, RC and V, run `--unit-diff`, `--type-diff` or `--diff` against the `a-gns2-` copy. For M, run `--same-row a-s1-M-marks.xml a-s2-M-marks.xml`. For N, `count19.py`'s own lines are the compare.
     - Then stop before step 3: nothing has been sent but reads and nothing keyed but BK, so nothing is restored. Close by C10.
     - One row or more in N-0105 or N-0107 is such a difference. Record each row whole.
3. **Change 1: a second GST registration.** GN's step 3, items 1 to 7, key for key and prompt for prompt, with the same three named prompts answered `c`, `y` and `n`, and the same GSTIN typed (`27ZZZZZ0000Z1ZQ`, in GN's two parts, zoomed after each).
   - Before it, `s3m`: `M-marks.xml`, then `count19.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass. Then no key is pressed in Tally but GN's step 3's.
   - Then `s3`: `U-taxunits.xml T-types.xml M-marks.xml`.
     - `count19.py --unit-diff a-gns3-U-taxunits.xml a-s3-U-taxunits.xml`. EXPECT the names `Default Tax Unit`, `Maharashtra Registration` and `Rajasthan Registration` on both sides, and 0 leaves differ in each.
     - `count19.py --type-diff a-46bs3-T-types.xml a-s3-T-types.xml`. EXPECT every line printed reads 0 leaves differ, and no type in one only.
     - Record both outputs whole, and ALTMSTID (GN: 246).
     - **Any leaf that differs from GN's stops the row here:** press nothing more, restore BK by C6, check as step 8, and close.
4. **Change 2: bind BRIDGE Sales to `Rajasthan Registration`.** GN's step 4, items 1 to 4, with one key different.
   - Before it, `s4m`: `M-marks.xml`; record ALTMSTID against s3's.
   1. GN's 4.1: at the Gateway, Go To, `Alter Master`, `Voucher Type`, Enter on `BRIDGE Sales` only (shot).
   2. GN's 4.2: `Voucher Type Alteration` (shot). Write down every field, as text. EXPECT GN's 4.2, with `Default GST Registration` reading `Any`.
   3. Click the value of `Default GST Registration`.
      - EXPECT the list `List of GST Registrations`: Create; Any (highlighted); Maharashtra Registration; Rajasthan Registration.
      - **Down twice:** `Rajasthan Registration` highlighted; zoom and check that it reads `Rajasthan Registration` and that `Maharashtra Registration` is not highlighted; Enter (shot).
      - EXPECT, as GN: the cursor moves to `Use common voucher numbering series for all GST Registrations: No`, with that field's own Yes / No list open and No highlighted. **Press no key on that list, and touch no other field.**
      - If the cursor goes anywhere else, or a prompt shows: the path rule.
   4. GN's 4.4: Ctrl+A. EXPECT no prompt, and the List of Voucher Types. Any prompt: the prompt rule.
      - Ctrl+Q once: the Gateway (shot).
   - Then `s4`: `T-types.xml U-taxunits.xml M-marks.xml`. Record ALTMSTID (GN, binding to the other unit: 248).
   - **STOP before the import unless both of these hold:**
     - `count19.py --type-diff a-s3-T-types.xml a-s4-T-types.xml`. EXPECT one type line, `type 'BRIDGE Sales': 2 leaves differ`, its two leaves `/ALTERID[1]` (from `'212'` to s4's ALTMSTID) and `/DEFAULTGSTREGISTRATION[1]: '' -> 'Rajasthan Registration'`, and no type in one only.
     - `count19.py --unit-diff a-s3-U-taxunits.xml a-s4-U-taxunits.xml`. EXPECT the same three names on both sides and 0 leaves differ in each.
   - Record both outputs whole.
   - If either shows anything else: send nothing, press nothing more, restore BK by C6, check as step 8, and close. The differing lines are this row's result, and the coordinator decides.
5. **Import A'': the bound unit's name.**
   - `s5m`: `M-marks.xml`, then `count19.py --before-write a-s1-M-marks.xml a-s5m-M-marks.xml`. It must pass. Then no key is pressed in Tally before the send.
   - `s5`: `I-0105.xml`, sent once. Then the wait rule.
     - Record every counter and any LINEERROR, word for word.
     - The window after it (shot): the Gateway, no dialog. A dialog: the prompt rule.
     - Record whether this answer is byte-equal to NR's created A' answer (540 bytes, sha256 `fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea`) or to GN's refused A answer (538 bytes, sha256 `09a713e2674c7016c2cb10f194f6e0fc7f6bf9dee3b523f4f6bbf1b07ea0f874`).
   - `s5r`: `N-0105.xml R-0105.xml RC-0105.xml V-0105.xml V-0010.xml M-marks.xml`.
     - Record N-0105's rows, each with its GUID.
     - **The status question:** record R-0105's rows: ledger, the three status flags (`VCHGSTSTATUSISINCLUDED`, `VCHGSTSTATUSISUNCERTAIN`, `VCHGSTSTATUSISEXCLUDED`), `ISGSTOVERRIDDEN` and the legs. Then `count19.py --diff a-s2-R-0010.xml a-s5r-R-0105.xml`, and record the whole output: it shows whether any status flag differs from `BP/26-27/0010`'s.
     - **What A'' stored:**
       - V-0105's `GSTREGISTRATION` (and any attribute), `CMPGSTIN`, `CMPGSTSTATE`, `CMPGSTREGISTRATIONTYPE`, `PLACEOFSUPPLY` and `STATENAME`, as count19.py prints them;
       - RC-0105's `GSTREGISTRATION` (and any attribute) and `CMPGSTIN`.
       - Record them whole, beside V-0010's.
     - `count19.py --diff a-s5r-R-0105.xml a-s5r-RC-0105.xml`. EXPECT only `GSTREGISTRATION` and `CMPGSTIN` differ. Record the whole output.
     - `count19.py --diff a-s2-V-0010.xml a-s5r-V-0010.xml`: record whether the existing invoice's fields moved.
     - Record ALTVCHID and ALTMSTID against s5m's.
   - **A'' is created** when the answer reads `CREATED` 1, `ERRORS` 0 and `EXCEPTIONS` 0, and N-0105 holds one row. Then go on to step 6.
   - **Any other answer** is a result: record it and go to step 8. B'' is not sent.
   - Never send `I-0105.xml` again (C3).
6. **Before B'': nothing A'' changed may reach B'' but its own number.** `s6m`: `T-types.xml U-taxunits.xml M-marks.xml`.
   - `count19.py --type-diff a-s4-T-types.xml a-s6m-T-types.xml`. EXPECT either no leaf that differs, or exactly one type line, `type 'BRIDGE Sales': 1 leaves differ`, whose one leaf is `/LASTNUMBER[1]: 'BP/26-27/0044' -> 'BP/26-27/0105'`. That is the type's own last number moving to A'''s, as NR's s4m read after its created A'. No type in one only.
   - `count19.py --unit-diff a-s4-U-taxunits.xml a-s6m-U-taxunits.xml`: EXPECT 0 leaves differ in each unit.
   - EXPECT ALTMSTID equal to s4's.
   - Record all three whole.
   - **If anything else differs** (another leaf on `BRIDGE Sales`, such as `LASTGSTTAXUNIT`; another type; a unit leaf; ALTMSTID moved), B'' would see a book A'' changed:
     - send nothing more, and go to step 8;
     - the difference is recorded as A'''s result, and B'' goes to a sitting of its own.
   - Otherwise `count19.py --before-write a-s1-M-marks.xml a-s6m-M-marks.xml`. It must pass. No key is pressed in Tally before the send.
7. **Import B'': the bound unit's name and its GSTIN.**
   - `s7`: `I-0107.xml`, sent once. Then the wait rule. Record as step 5, and whether the answer is byte-equal to A'''s.
   - `s7r`: `N-0107.xml R-0107.xml RC-0107.xml V-0107.xml V-0105.xml V-0010.xml M-marks.xml`.
     - **B'''s answers:** R-0107's status flags, V-0107's six fields and RC-0107's two, recorded as step 5's.
     - `count19.py --diff a-s5r-R-0105.xml a-s7r-R-0107.xml`: record whether B'''s status or fields differ from A'''s.
     - `count19.py --diff a-s7r-R-0107.xml a-s7r-RC-0107.xml`. EXPECT only `GSTREGISTRATION` and `CMPGSTIN` differ.
     - `count19.py --diff a-s5r-V-0105.xml a-s7r-V-0105.xml`: record whether A'''s invoice moved.
     - `count19.py --diff a-s2-V-0010.xml a-s7r-V-0010.xml`: record whether `0010`'s moved.
     - Record ALTVCHID and ALTMSTID against s6m's.
   - Any answer at s7 is a result. Never send `I-0107.xml` again (C3).
8. **Restore** BK by C6, with `restore-plan.txt` written first. GN's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s8`: `M-marks.xml U-taxunits.xml T-types.xml N-0105.xml N-0107.xml R-0010.xml RC-0010.xml V-0010.xml`, and compare whole:
     - `count19.py --same-row a-s1-M-marks.xml a-s8-M-marks.xml`: EXPECT it passes.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml`: EXPECT the two units and 0 leaves differ in each.
     - `count19.py --type-diff a-s2-T-types.xml a-s8-T-types.xml`: EXPECT every line printed reads 0 leaves differ.
     - N-0105 and N-0107: 0 rows each.
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
- The screenshots of steps 3 and 4, and after each import's send, listed by name, bytes and sha256.
- As text and word for word: every key pressed, and every prompt, message and busy screen, with the key that answered it and its time (UTC).
- Every field of `GST Registration Creation` as Tally filled it, and of `Voucher Type Alteration`, as text.
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** byte-equal to GN's s2, as the Book and step 2 say.
- **Step 3:** as GN's s3: the third unit, 0 leaves differ, ALTMSTID 246.
- **Step 4:** the two leaves step 4 names, with `ALTERID` 248 and ALTMSTID 248, as row 2464-GR second sitting's binding moved them. Not measured for this unit.
- **Step 5:** not estimated. Created is plausible: the named unit is the one the type is bound to and the one the invoice's taxes fit, and NR created the same invoice naming it. Refused is also plausible: GN's refusal came on this two-registration book, and the book is the one thing changed from NR.
- **The status flags:** not estimated. GR's second sitting read uncertain on this book with no registration named and the type bound to the other unit. That cannot be told apart here from a book-level cause.
- **s8:** equal to s2 and s1.

## Decides
Each is PARTIAL: one import of each shape, one sitting, one book, one release.

**For the write and the status (A'')**
- **A'' is created, stores `Rajasthan Registration` with the GSTIN, state and type as `BP/26-27/0010` holds them, and R-0105 reads included `Yes`, uncertain `No`, excluded `No`:** on a two-registration book, an invoice that names the unit its type is bound to, which its taxes fit, is taken, stored and included in GSTR-1.
  - A candidate builder for a two-registration book can then admit the invoice when its type is bound to a unit the invoice fits, write that unit's name in this place and form, and judge the read-back's `GSTREGISTRATION` against it.
  - Until that builder is built and reviewed, a two-registration book stays refused (`invoice_company_registration_ambiguous`).
- **A'' is created and stored so, but R-0105 reads uncertain:** naming the fitting, bound unit does not clear the status on this book. The cause is not in the answer. Record the flags and the whole `--diff` against `0010`'s read-back; the coordinator decides the next variable.
- **A'' is created but stores the registration fields some other way** (empty, `Maharashtra Registration`, or values other than `0010`'s): Tally does not keep the element as named on this book. Record all six whole; the coordinator decides.
- **A'' is answered `CREATED` 0, `EXCEPTIONS` 1, no words:** on this book Tally refuses the element even when it names the bound unit the invoice fits. GN's refusal then cannot be put down to the taxes alone; something of the two-registration book refuses it (GN's cause 3). The next row is chosen by the coordinator. A two-registration book stays refused.
- **A'' is refused with words** (an error or a LINEERROR): record them word for word. They are the first answer on why.
- **V-0010 moved at a later read:** a later import reached an earlier invoice. Record which field, and report before the restore.

**For B''**
- **B'' is created** while A'' was: compare V-0107's six fields and R-0107's flags with A'''s. Equal: writing the GSTIN as well changes nothing stored or reported on this book, and a builder may write the name alone or both. Different: record whole; the coordinator decides.
- **B'' is refused** while A'' was created: writing `CMPGSTIN` as well is not taken in this place or form on this book, and a builder writes the name alone.
- **A'' changed a master this row reads beyond its own `LASTNUMBER` (step 6):** record which. The build would face that change on its next invoice. B'' moves to its own sitting.

**For the read-back (RC), only if A'' or B'' is created**
- **RC returns `GSTREGISTRATION` and `CMPGSTIN` equal to V's, with every other field equal to R's:** the candidate read-back reads the registration of a two-registration invoice, as NR's read it on the one-registration book.
- **RC returns them `<absent>` or empty while V holds them:** the collection does not return them here. Record whole; the coordinator decides.

**Not decided here:** an invoice naming a unit other than the one its type is bound to, an unbound type on this book, and an inter-state invoice.

**What these answers are for.** They may inform #2464's design and a candidate builder. They are never a regression fixture for the build's output, because the build renders neither import.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build refuses this book before any invoice request.** With two GST units in force, `parse_company_registration` (`src-tauri/src/agent_import_invoice_wire.rs:800`) takes its ambiguous arm (`:804`) and refuses as `invoice_company_registration_ambiguous`, bound or not. So the build sends neither import here. This row measures what a candidate builder that picks the bound unit (#2464's Expected 2) would rely on, before it is built.
- **The build writes no registration.** `render_sales_invoice_xml` (`src-tauri/src/agent_import_invoice.rs:2664`, its template at `:2708`) writes `GSTREGISTRATIONTYPE` (the customer's) and no `GSTREGISTRATION`, `CMPGSTIN` or `CMPGSTSTATE`. Both imports here are therefore authored.
- **The build's read-back fetches neither field.** `render_invoice_readback_request` (`src-tauri/src/agent_import_invoice_wire.rs:505`, its FETCH at `:514`) is `R-0010.xml`'s shape. `RC-` adds the two names and nothing else.

## Later rows
- **If A'' is created and included:** the build's own rendered request, once a candidate builder writes the registration and the admission picks the bound unit, re-run here and read back by the build's changed read-back. That run, not this row, is the first answer for the build's output.
- **Whatever A'' answers:** an invoice naming the unit its type is not bound to, and an unbound type on this book, each one variable from this row.

## Not measured
- An invoice naming a unit other than its type's binding, and an unbound type on a two-registration book.
- `CMPGSTSTATE` or `CMPGSTREGISTRATIONTYPE` written by the request, and `GSTREGISTRATION` with attributes or in another place.
- A mismatched unit and GSTIN.
- An inter-state invoice, and GSTR-1 on screen.
- An invoice built and posted through Bridge.
- Any other book or release.
