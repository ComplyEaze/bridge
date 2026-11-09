# Row 2162-S9b: does the GST Registration Types list's "Show More" offer a special economic zone type, and when a registered customer is given it, what does Tally store, and where?

Not for merge. Part of #2162 (ADR 0004, checklist item 15: the customer's party type; lab set S9). Written 9 Nov 2026. Lines below are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
ADR 0004's checklist item 15: a customer in a special economic zone, or a deemed export, must not be taken as an ordinary supply within the state. Row 2162-S9a (`lab/2162-s9a-answers` at e84d928e, folder `sitting-2/`) looked for a party-type field on `BRIDGE Customer RJ` and found none, either as seeded or after its Country and State were set (its save A). Its "Additional GST Details" held only Place of Supply and Is the Party a Transporter. Its "GST Registration Types" list read Unknown, Composition, Regular, Unregistered/Consumer and **Show More**, and Show More was not opened. That row names the next variable as a setting that shows the field.

**This row's one variable is that list's Show More.** On the same ledger, seeded and located as in S9a's second sitting, the run lead opens Show More and records the full list. If the list offers a type that names a special economic zone, the run lead gives the ledger that type, saves once and reads it. The book is backed up first and restored afterwards. No voucher is sent and no Bridge build runs.

Why this decides much of item 15: the build admits a customer with a GSTIN in force only when the registration type of the entry in force reads exactly `Regular`. Anything else is refused as `invoice_party_not_regular` (`src-tauri/src/agent_import_invoice.rs:856-860`). If the zone is a registration type, an SEZ customer is already refused, and item 15 needs only a test on this row's captured ledger. If it is stored elsewhere, the build does not read it.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-BC's, 2162-KD's and 2162-S9a's) applies in full. Where it and this sheet differ, the stricter one holds.
- **Its state.** Row 2162-S9a's second sitting restored it to ALTVCHID 68 and ALTMSTID 245, with 18 LEDGER rows in the compliance read and none named `BRIDGE Customer RJ`. This row seeds the ledger again, with the same file.
- **One BK, taken in this sitting before the first write, and one restore after the last** (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID must equal step 1's (the ruling of 4 Nov 2026 on unchanged marks). No earlier BK stands for this row's.
- **The release.** Write down the release shown on Tally's screen and the `tally.exe` file version. S9a read file version 1.1.7.2.

**Every master screen is left with Ctrl+Q** after a capture that shows its title, never with Esc (LAB-COMMON C7). S9a's second sitting left one screen with Esc, against C7; nothing was saved there, but this row allows no such exit.

**The ledger and its location are setup.** The ledger is seeded by direct XML under the owner's seed rule, with row 2162-BC's `L-seed.xml`, byte-equal. Its location is then set on screen exactly as S9a's save A did, because the list appeared there only after it. Neither is evidence of its own shape: S9a's second sitting already measured both.

**No Bridge build.** Raw requests go one at a time through `run.py` (row 2162-BC's, byte-equal). The writes are:
- the setup ledger's import;
- the location save (setup);
- at most one registration type save (the variable).

All three are made by the run lead. No approval window is involved.

**The sends** are row 2162-S9a's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found":
- **in `run.py`:** a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes;
- **in `count7.py`,** which prints STOP and exits non-zero:
  - a read answer with no `BODY/DATA/COLLECTION`;
  - an import answer with no counters;
  - an answer that does not parse;
  - a `--leafdiff` side without exactly one row named `BRIDGE Customer RJ`.

## Files
Requests are UTF-16LE, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2162-S9a's file of the same name** (`lab/2162-s9a-sheet` at e558c9f8, folder `lab/2162-s9a/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `template-full-1.utf16le.xml`, `L-seed.xml`, `C-compliance.xml`, `gen.py`, `K-rj.xml` and `count7.py`. S9a's ROW.md says where each came from and what `count7.py` printed when it was written.
  - `C-compliance.xml` is the build's ledger compliance read, byte-equal to `r05-ledgers-compliance.xml` (`lab/1342-capture-requests`). Its fetch list ends in `LEDGSTREGDETAILS.LIST`, fetched whole (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/request.rs:382`).
  - `K-rj.xml` is every native field of the one ledger. No build sends it.
  - Re-check `gen.py` in a separate folder: it must rebuild `K-rj.xml` byte for byte.
- **New:** this ROW.md and `SHA256.txt`. No script is changed.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count7.py a-<label>-*`. A STOP from either script stops the row there. On the lab PC, run `count7.py` with `python -X utf8` (S9a's second sitting: the cp1252 console could not print the rupee sign).

**Before each write** (the setup import in step 3, the screen saves in steps 4 and 5):
1. Send `run.py URL s<n>m M-marks.xml`.
2. Run `python3 -I count7.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`.
3. Only if that passes, and no key was pressed in Tally since, make the write: send the import alone (`run.py URL s3 L-seed.xml`), or open the screen.

**Labels are used once each.** A repeated walk takes a new label (`s5m2`, `s5q2` and so on), never an earlier one's.

1. **Load, check and BK.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row with the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder, before any write. Record the file's name, size and sha256.
2. **The book as found.** `s2`: `C-compliance.xml K-rj.xml`.
   - EXPECT in C 18 LEDGER rows and 0 named `BRIDGE Customer RJ`; in K 0 LEDGER rows, as both sittings of S9a read.
   - Anything else: record it and stop before any write.
3. **The setup ledger, then the ledger as seeded, read twice.** As S9a's step 3:
   - `s3m` and the check, then `s3`: `L-seed.xml`. Screenshot the window.
   - Then `s3r`: `C-compliance.xml K-rj.xml M-marks.xml`, and `s3b`: `C-compliance.xml K-rj.xml`.
   - EXPECT CREATED 1, every other counter 0 and no `LINEERROR`.
   - EXPECT in C 19 LEDGER rows, one named `BRIDGE Customer RJ`, with one `LEDGSTREGDETAILS.LIST` reading `APPLICABLEFROM` 20260401, `GSTREGISTRATIONTYPE` Regular and `PLACEOFSUPPLY` Rajasthan, with one GSTIN held. In K one LEDGER row, the same.
   - EXPECT in M ALTVCHID as in `s1` and ALTMSTID above it.
   - `count7.py --leafdiff a-s3r-K-rj.xml a-s3b-K-rj.xml`, and the same for `C-compliance`. EXPECT 0 leaves differ in each.
   - A difference, or any other answer to the import: record it, make no save, and go to step 6.
4. **The location (setup), as S9a's save A.** `s4m` and the check.
   - From the Gateway: Alter (`a`), Down to Ledger, Enter, type `Customer RJ` (one row listed), Enter: Ledger Alteration. Capture the screen.
   - Set Country to India and State to Rajasthan, and change nothing else. A Pincode field may appear: leave it empty.
   - Save with Ctrl+A. If Tally asks "Accept ?", answer Yes (C7). Any other prompt: stop, press nothing, and ask.
   - `s4r`: `C-compliance.xml K-rj.xml M-marks.xml`, then `count7.py --leafdiff a-s3r-K-rj.xml a-s4r-K-rj.xml`, and the same for `C-compliance`.
   - **EXPECT the same leaf paths as S9a's save A**, and no other: 19 in K and 2 in C. They are listed in `lab/2162-s9a-answers` at e84d928e, `sitting-2/evidence/leafdiff-s3r-s4ar-K-rj.txt` and `-C-compliance.txt`.
     - Compare paths, not values: `ALTERID`'s number and `REMOTEALTGUID` will differ.
     - A path in one list and not the other: record it, make no further save, and go to step 6. The variable's diff could not then be read against a known base.
5. **The variable: Show More.** `s5m` and the check. Then the same path to Ledger Alteration for `BRIDGE Customer RJ`.
   - **Before anything is opened:** capture the screen. Write down Tax Registration Details as shown: Registration type, GSTIN/UIN, and "Set/Alter additional GST details".
   - **Open the list.** Move to Registration type and open its list ("GST Registration Types"). Capture it, and write down every entry as shown.
   - **Show More.** Choose Show More, and key nothing else.
     - Capture every page of the list it shows. Write down every entry word for word, with its spacing and hyphens exactly as the screen shows them.
     - If the list scrolls, scroll with the arrow keys only, and capture each page.
   - **If an entry names a special economic zone** (for example "SEZ" or "Special Economic Zone"), choose it. Choose only one such entry. If there are several, take the first one listed, and write down the others.
     - After choosing it, write down every field the screen then shows, with its value.
     - **Key no other field.** If Tally asks for a value, or opens a sub-screen or a date before it will save, capture it, press nothing, and ask the run coordinator. A second change would be a second variable.
     - Save with Ctrl+A. If Tally asks "Accept ?", answer Yes (C7). Any other prompt: stop, press nothing, and ask. Screenshot the Gateway.
     - `s5r`: `C-compliance.xml K-rj.xml M-marks.xml`, then `count7.py --leafdiff a-s4r-K-rj.xml a-s5r-K-rj.xml`, and the same for `C-compliance`.
     - Then open Ledger Alteration for `BRIDGE Customer RJ` once more, key nothing, and capture the screen and its Tax Registration Details as shown. Leave with Ctrl+Q, and record any prompt word for word with the key that answered it.
     - `s5q`: `M-marks.xml`. EXPECT byte-equal to `s5r`'s marks: nothing saved by the look.
   - **If no entry names a special economic zone:** write down whether any entry names a deemed export, an export unit or an embassy, word for word. Choose nothing, and leave with Ctrl+Q, recording any prompt word for word with the key that answered it.
     - `s5q`: `M-marks.xml`. EXPECT byte-equal to `s4r`'s marks: nothing saved.
     - Go to step 6. That is a result.
   - **Not opened in this row:** F12 Configure, I More Details, "Set/Alter additional GST details", and any company GST feature. No second type is saved: a Deemed Export save would be a second variable and gets its own row.
6. **The restore**, from the BK of step 1, if anything was written. C6, and row 2162-S9a's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml C-compliance.xml K-rj.xml`, and `count7.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, with C 18 LEDGER rows and 0 named `BRIDGE Customer RJ`, and K 0 LEDGER rows, as at `s2`.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write other than the prompts named above, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.

## Record
- Each answer's bytes and sha256, and `count7.py`'s lines, above all the `--leafdiff` lines.
- **Step 5's screens:**
  - the list before and after Show More, every entry word for word;
  - the fields shown after the choice, with their values;
  - the screen on reopening;
  - the screenshots by name, bytes and sha256;
  - every prompt word for word, with the key that answered it.
- The BK file, and every prompt of the restore word for word with the key that answered it.
- The release on Tally's screen, the `tally.exe` file version and the Gateway's date.

## Expected
Estimates, not predictions. Nothing about a special economic zone has been captured, on this book or any other.
- **s2, s3r, s4r:** as S9a's second sitting read them.
- **Show More:** a longer list of registration types. Whether it names a special economic zone, and the exact text, are this row's first question.
- **s5r, if a zone type is chosen:** in K, `LEDGSTREGDETAILS.LIST[1]/GSTREGISTRATIONTYPE` changes from Regular to the chosen text, plus `ALTERID` and the alter marks. The C diff shows the same leaf. It is also possible that:
  - Tally adds a second dated `LEDGSTREGDETAILS.LIST` entry instead of changing the first (count7's C kind prints every entry, with its `APPLICABLEFROM`);
  - another leaf changes as well (`ISSEZPARTY`, read present and empty by rows 2162-BC and S9a, or `GSTNATUREOFSUPPLY`, which save A set to 'Not Applicable').

## Decides
- **Show More offers no zone type:** the party type is not a registration type on this release, and this row's answer is that alone. The next variable is F12 Configure on the ledger screen, in its own row.
- **The zone type is chosen, and only `GSTREGISTRATIONTYPE` changes (inside `LEDGSTREGDETAILS.LIST`, in both K and C):**
  - The build's compliance read already carries the party type, and its existing exact match refuses it as `invoice_party_not_regular`.
  - Item 15's remaining work is then a test on this row's captured ledger row, with no new request.
  - The test today covers `Composition` only (`src-tauri/src/agent_import_invoice_tests.rs:315-316`).
- **Tally adds a second dated entry:** the build reads the entry in force on the invoice date (`read_gst_registration_entry`, `src-tauri/crates/bridge-tally-protocol/src/native_ledger_collection.rs:1048`). The new entry's `APPLICABLEFROM` decides which invoices it reaches. Record it; the refusal's test then uses both entries.
- **A leaf outside `LEDGSTREGDETAILS.LIST` changes too** (in K, not in C): the compliance read's fetch list does not reach it. Whether it, rather than the registration type, is what marks the zone is then the next row's question. Adding it to a read is a new request shape: a candidate builder, reviewed and sent to a lab book before any release sends it (AGENTS.md P7).
- **A value or sub-screen is demanded before the save:** the row stops there, by step 5. What was asked for is the next row's variable.
- **A dialog after a save:** the row stops (see Stop).

## Code admits the trigger
Lines are at master ae38a75c.
- **The build reads the registration type of the entry in force.** `read_gst_registration_entry` (`native_ledger_collection.rs:1048`) keeps `APPLICABLEFROM`, `GSTIN` and `GSTREGISTRATIONTYPE` of each `LEDGSTREGDETAILS.LIST`, from the compliance read (`native_outstandings/request.rs:382`) and from the rates read (`src-tauri/src/agent_import_invoice_wire.rs:882`), both of which fetch that list whole.
- **The admission matches it exactly.** For a customer with a GSTIN in force, `Regular` admits. `None` is `invoice_party_registration_type_not_reported`, and any other text is `invoice_party_not_regular` (`agent_import_invoice.rs:830`, `:856-860`).
- **What is sent.** C is the build's compliance read, byte-equal. K is a lab read of every native field, the kind rows 2162-BC, 2162-IRN10 and 2162-S9a sent; no build sends it.
- **What is written.** The setup ledger's import, the location save and at most one registration type save, on a synthetic book, restored from the BK taken before the first of them. No voucher and no Bridge build.

## Not measured
- Deemed Export, an export unit, Embassy/UN Body and the other types the list may offer: each would be a save of its own.
- F12 Configure, More Details and the company's GST features.
- A zone type on an unregistered or composition customer, or set by import.
- Whether Tally changes the tax of an invoice to such a customer.
- Any other book or release.

Row: lab/2162-s9b/
