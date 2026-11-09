# Row 2162-S9a: when a registered customer's GST party type is set on screen to SEZ, and then to Deemed Export, what does Tally store, where, and does the build's own compliance read return it?

Not for merge. Part of #2162 (ADR 0004, checklist item 15: the customer's party type; lab set S9). Written 9 Nov 2026. Lines below are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
ADR 0004's checklist item 15: the customer's party type is not read, so a customer in a special economic zone, or a deemed export, would be taken as an ordinary supply within the state. What is owed is a read of the party type and a refusal of anything but the ordinary type, on a captured ledger row (P1, P3).

A refusal cannot be written before three things are known, and none is captured:
- the element Tally stores the party type in;
- where it sits: inside the ledger's `LEDGSTREGDETAILS.LIST`, which the build fetches whole, or elsewhere;
- the text of each value.

**This row changes one field of one ledger: its party type.** On `BRIDGE Customer RJ`, a registered customer in the company's own state, the field is set on Tally's screen to SEZ, read, then set to Deemed Export, and read again. The book is backed up first and restored afterwards. No voucher is sent and no Bridge build runs.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-BC's and 2162-KD's) applies in full. Where it and this sheet differ, the stricter one holds.
- Its state, from the lab's last notes on it: each row since S13a2 restored it to ALTVCHID 68 and ALTMSTID 245, or only read it, and row 2162-KD's restore read 68 and 245.
- `BRIDGE Customer RJ` is not in the book. Row 2162-BC created it with its `L-seed.xml` and its restore removed it again (its catalogue read held 18 LEDGER rows before the seed, 19 after it and 18 after the restore), and this row's first sitting, on 9 Nov 2026, found 18 rows in the compliance read, twice and byte-equal, and no such ledger at step 2, before any write. This row seeds it again, with the same file.
- Row 2162-BC's `a-s3r-K-customer.xml` (`lab/2162-bc-answers` at 9f36c30f), read straight after that seed, read it under `Sundry Debtors`, not bill-wise, with one `LEDGSTREGDETAILS.LIST` holding `APPLICABLEFROM` 20260401, `GSTREGISTRATIONTYPE` Regular, `PLACEOFSUPPLY` Rajasthan and one GSTIN. Among its 435 leaves, the direct children `ISSEZPARTY` and `GSTNATUREOFSUPPLY` were present and empty, and no element's name contained DEEMED or PARTYTYPE. The build's compliance read fetches neither of the two (its fetch list, below).
- One BK before the first write and one restore after the last (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen. KD read TallyPrime 7.2 Silver.

**The ledger is setup**, seeded by direct XML under the owner's seed rule: row 2162-BC's `L-seed.xml`, byte-equal. It is never evidence of its own shape.

**No Bridge build.** Raw requests, one at a time, through `run.py` (row 2162-BC's, byte-equal). The writes are the setup ledger's import and the two alterations on Tally's screen, by the run lead. No approval window is involved.

**The sends** are row 2162-BC's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found":
- in `run.py`: a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes;
- in `count7.py`, which prints STOP and exits non-zero: a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, an answer that does not parse, or a `--leafdiff` side without exactly one row named `BRIDGE Customer RJ`.

## Files
Requests are UTF-16LE, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2162-KD's file of the same name** (`lab/2162-keyed-dup-sheet` at c5b8e06e, folder `lab/2162-keyed-dup/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml` and `template-full-1.utf16le.xml`. Row 2162-BC's ROW.md says where each first came from.
- **`L-seed.xml`:** byte-equal to row 2162-BC's (`lab/2162-bc-sheet` at e2b1ca82, folder `lab/2162-bc/`), sha256 `0090d9a8…`, itself S13a2's. One `LEDGER` create: `BRIDGE Customer RJ` under Sundry Debtors, not bill-wise, with one `LEDGSTREGDETAILS.LIST` from 20260401, Regular, Rajasthan, and a synthetic GSTIN. It sets no party type.
- **`C-compliance.xml`:** byte-equal to `r05-ledgers-compliance.xml` (`lab/1342-capture-requests`), sha256 `c283b511…`. It is the build's ledger compliance read for `BRIDGE PILOT LAB`: the committed answer `pilot-lab-ledgers-compliance.utf16le.xml` is its answer (`tests/fixtures/agent/pilot-lab/PROVENANCE.md`, the table's row for that file). Its fetch list ends in `LEDGSTREGDETAILS.LIST`, fetched whole (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/request.rs:382`). It reads every ledger of the book, about 40 KB.
- **`gen.py`** writes `K-rj.xml` from the template and asserts every change: the company `BRIDGE PILOT LAB`, the year 2026-27, `TYPE` Ledger, and the filter `$Name = "BRIDGE Customer RJ"`. It is every native field of that one ledger, built as row 2162-IRN10's K files. No build sends it.
- **`count7.py`:** row 2162-IRN10's `count.py` cut to the kinds M, K and an import kind (read under L, as row 2162-BC's `count5.py` reads its `L-seed` answer), with the book's GUID changed to `BRIDGE PILOT LAB`'s, its K kind widened to every leaf with a value, a C kind, and one check added:
  - **C** prints how many LEDGER rows the compliance read holds, then for `BRIDGE Customer RJ` every leaf of each `LEDGSTREGDETAILS.LIST`, empty ones included;
  - **`--leafdiff A B`** takes two K or two C answers, finds the one row named `BRIDGE Customer RJ` in each, and prints every leaf, by indexed path, whose text or `TYPE` differs or that is in one only, then how many differ.

  When the sheet was written:
  - on row 2162-BC's `a-s3r-K-customer.xml` it printed one LEDGER row with 435 leaves, 97 of them with a value;
  - `--leafdiff` of that answer with itself printed 0 leaves differ;
  - on a copy with one element added inside `LEDGSTREGDETAILS.LIST` (a test file, not kept) it printed that one leaf and 1 leaves differ;
  - against row 2162-BC's `a-s2-K-walkin.xml` it printed STOP;
  - on the committed `w18-ledgers-compliance.utf16le.xml` as a C answer it printed 18 LEDGER rows and 0 rows named `BRIDGE Customer RJ`. A count of `<LEDGER NAME=` in the same file also gives 18;
  - on row 2162-BC's `a-s3-L-seed.xml` it printed root RESPONSE, CREATED 1 and every other counter 0, as that row's RESULTS.md records; on a test answer with no counters (not kept) it printed STOP.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count7.py a-<label>-*`. A STOP from either script stops the row there.

**Before each write** (the setup import in step 3, the screen alterations in steps 4 and 5): `run.py URL s<n>m M-marks.xml`, then `python3 -I count7.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, make the write: send the import alone (`run.py URL s3 L-seed.xml`), or make the alteration.

1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row with the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder, before any write. Record the file's name, size and sha256. A BK taken earlier in this sitting, before any write, stands for it when `s1`'s COMPANY row equals the one read when it was taken (`count7.py --same-row`); record which BK is used. This row's first sitting took one such BK before any write, recorded in `lab/2162-s9a-answers` at 6bf8a03f (`RESULTS.md`, the BK line), with the COMPANY row it read then: the book's GUID, ALTVCHID 68, ALTMSTID 245. Use it when `s1` reads that row; otherwise take a new BK.
2. **The book as found.** `s2`: `C-compliance.xml K-rj.xml`.
   - EXPECT in C 18 LEDGER rows and 0 named `BRIDGE Customer RJ`; in K 0 LEDGER rows. Row 2162-BC's catalogue read held 18 before its seed and 18 after its restore, and this row's first sitting read 18 here, twice.
   - Anything else: record it and stop before any write.
3. **The setup ledger, then the ledger as seeded, read twice.** `s3m`, the check, then `s3`: `L-seed.xml`. Screenshot the window. Then `s3r`: `C-compliance.xml K-rj.xml M-marks.xml`. Then `s3b`: `C-compliance.xml K-rj.xml`.
   - EXPECT CREATED 1 and every other counter 0, with no `LINEERROR`, as row 2162-BC's `s3` read. In C 19 LEDGER rows, one named `BRIDGE Customer RJ`, with one `LEDGSTREGDETAILS.LIST` reading `GSTREGISTRATIONTYPE` Regular and `PLACEOFSUPPLY` Rajasthan, and one GSTIN held; in K one LEDGER row, the same. In M ALTVCHID as in `s1` and ALTMSTID above it.
   - `count7.py --leafdiff a-s3r-K-rj.xml a-s3b-K-rj.xml`, and the same for `C-compliance`. EXPECT 0 leaves differ in each. A difference between two reads with nothing changed is recorded and the row goes to step 6 with no alteration: a later difference could not then be put down to the screen.
   - Any other answer to the import: record it, make no alteration, and go to step 6.
4. **SEZ, on screen.** The row's first alteration. `s4m`, the check, as above.
   - From the Gateway: Alter, then Ledger, then `BRIDGE Customer RJ`. Write down the menu path and keys used.
   - Find the field that sets the party's GST party type. Its list is expected to offer at least Not Applicable, Deemed Export, Embassy/UN Body and SEZ. Write down its label as the screen shows it, its value as found, and every value its list offers.
   - If reaching it needs another field changed first (for example a Yes that opens a sub-screen of further GST details), write down that field, its value before and after, and go on. That field is then a second change, and the diff will show it.
   - If no such field is offered: change nothing, leave with Esc (any "Quit ?" answered Yes; C7), record it, and go to step 6. That is a result.
   - Set the party type to SEZ, and change nothing else. Screenshot every part of the screen before saving, and write down every field it shows with its value, as text.
   - Save with Ctrl+A. If Tally asks "Accept ?", answer Yes; that is the master's own accept (C7). Any other prompt: stop, press nothing, and ask. Screenshot the Gateway.
   - `s4r`: `C-compliance.xml K-rj.xml M-marks.xml`.
   - `count7.py --leafdiff a-s3r-K-rj.xml a-s4r-K-rj.xml`, and the same for `C-compliance`.
   - EXPECT (estimate) one leaf or more differ in K. In M, ALTVCHID as in `s1` and ALTMSTID above `s3r`'s.
5. **Deemed Export, on screen.** `s5m`, the check, as above. The same path as step 4, and change only the party type, from SEZ to Deemed Export. The same screenshots, record and save.
   - `s5r`: `C-compliance.xml K-rj.xml M-marks.xml`.
   - `count7.py --leafdiff a-s4r-K-rj.xml a-s5r-K-rj.xml`, and the same for `C-compliance`.
6. **The restore**, if anything was written, from the BK of step 1. C6, and row 2162-BC's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml C-compliance.xml K-rj.xml`, and `count7.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, C 18 LEDGER rows and 0 named `BRIDGE Customer RJ`, and K 0 LEDGER rows, as at `s2`.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write other than the prompts named above, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.

## Record
- Each answer's bytes and sha256, and `count7.py`'s lines, the `--leafdiff` lines above all.
- Steps 3 and 4's screens: the field's label, every value its list offers, every field and value shown, the screenshots by name, bytes and sha256, and every prompt word for word with the key that answered it.
- The BK file, and every prompt of the restore word for word with the key that answered it.
- The release on Tally's screen and the Gateway's date.

## Expected
Estimates, not predictions. Nothing about the party type has been captured, on this book or any other.
- **s2:** 18 rows, no `BRIDGE Customer RJ`, as the first sitting read.
- **s3r:** the ledger as row 2162-BC's `s3r` read it after the same seed, but for its own identity fields.
- **s4r, s5r:** one leaf differs at each step, with a value naming SEZ, then Deemed Export. Where it sits, and whether the C answer carries it, are this row's questions. If SEZ lands in the empty `ISSEZPARTY` row 2162-BC read, a direct child of the ledger, the C diff will show nothing at step 4.

## Decides
- **The leaf sits inside `LEDGSTREGDETAILS.LIST`, and the C diff shows it:** the build's existing compliance read already carries the party type. Item 15's refusal is then a parser change on these captures, with no new request. The rates read for an invoice's ledgers also fetches that list whole (`agent_import_invoice_wire.rs:882`), so the same leaf is expected there too (unverified until read).
- **The leaf sits elsewhere, and the C diff shows nothing:** the compliance read's fetch list does not reach it. Adding it is a new request shape: a candidate builder, reviewed and sent to a lab book before any release sends it (AGENTS.md P7), then item 15's refusal.
- **No leaf differs in K:** the screen's field is stored where a native read of the ledger does not reach, or not stored at all. Report it; a wider read is the next row's question.
- **No party-type field is offered for this customer:** report what the screen shows. The field may depend on another setting, and that setting is then the next row's variable.
- **A second field had to change to reach it:** the diff names both. Which leaf is the party type is then read from the step 5 diff, where only the party type changed.
- **A dialog after a save:** the row stops (see Stop).

## Code admits the trigger
Lines are at master ae38a75c.
- **The build reads the party's registration entries, three fields of each.** `read_gst_registration_entry` (`src-tauri/crates/bridge-tally-protocol/src/native_ledger_collection.rs:1048-1062`) keeps `APPLICABLEFROM`, `GSTIN` and `GSTREGISTRATIONTYPE` of each `LEDGSTREGDETAILS.LIST`. Its comment says other children are consumed and not kept. So no party type is read today, whatever Tally stores, and an SEZ customer in the company's own state passes as an ordinary one (ADR 0004, item 15).
- **What is sent.** C is the build's compliance read, byte-equal. K is a lab read of every native field, the kind rows 2162-BC and 2162-IRN10 sent as their K reads; no build sends it.
- **What is written.** The setup ledger's import and the two screen alterations, on a synthetic book, restored from the BK taken before the first of them. No voucher and no Bridge build.

## Not measured
- Embassy/UN Body, and the other values the list may offer.
- A party type set on an unregistered or composition customer.
- A party type dated by its own `APPLICABLEFROM`, with more than one registration entry.
- Whether Tally changes the tax of an invoice to such a customer.
- The rates read's answer for this ledger.
- Any other book or release.

Row: lab/2162-s9a/
