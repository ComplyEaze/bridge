# Row 2162-SALES-Q: the Sales qualification rows the 8c3fb9f2 build can run on one synthetic book

For #2162 (Sales qualification checklist): item 1, and the item 6 rows the build at master 8c3fb9f2 can exercise. Not for merge. Written 23 Nov 2026. One sitting, one book, one backup, four parts. Confidence of everything here until the row runs: **unverified**.

## What each owed row must show, and what this sheet runs
The owed list is `docs/tally/TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md` ("Still owed before Sales is qualified", in its Sales invoice section) and the issue's newest comments. What is already measured is cited by branch and commit; this row does not repeat it.

| Owed row | What it must show | Already measured | Here |
| --- | --- | --- | --- |
| Item 1: a post-dated invoice reads post-dated Yes | A Sales invoice marked post-dated (Ctrl+T) and read back as post-dated | Raw only: `ISPOSTDATED` Yes on the read-back request and the native read (`lab/1342-w21c-answers` 19d50259; `lab/2162-s1-answers` 20ed166c; 7.2 Silver in S1). Never through a build. | **Part P**: one keyed post-dated invoice, read by the build's `gst_status`. |
| Item 6: GSTR-1 split for two sales ledgers on one tax pair | GSTR-1's HSN summary gives each sales line of a two-ledger invoice to its own code | Raw import of the build's own 0010 request, renumbered 0072 (`lab/2162-hs-answers` d7a1adf8, PARTIAL). Not through a build. | **Part G**: the same invoice, built and posted by the build; the HSN summary before and after. |
| Item 6: several registrations | What the build does on a book with two GST registrations in force | By code reading only (ADR 0004: refused as `invoice_company_registration_ambiguous`). Tally's own shape of a second registration: `lab/2162-ru-answers` c7b14084 (Part C2), `lab/2464-gn-sheet` 23b2e57b step 3. | **Part R, R-C2**: the second registration keyed, then the build asked to build. |
| Item 6: an inactive registration | What the build does when the one registration is inactive on the invoice date | Tally's shape: a new dated row with `ISINACTIVE` Yes from the date entered (`lab/2162-ru-answers` c7b14084, Part B). The refusal rests on the field names, and the tests edit captured bytes. | **Part R, R-B**: the registration made inactive from 1-Sep-2026, then the build asked to build on 1 Aug and on 15 Sep. |
| Item 6: buyer/consignee block compare | The reference's owed compare: an invoice with the block against one without, on a company with a valid registration, in GST status and GSTR-1; and the six added to the read-back or the gap accepted | Raw, with and without, registered and unregistered (`lab/2162-bc-answers` 9f36c30f); ADR 0004 item 9 closed with the gap accepted. | The with-and-without compare cannot run through the build: its renderer writes all six on every invoice (`agent_import_invoice.rs:3084`), so the build cannot send an invoice without them. **Step 14** grades what the build can show: the six as the build's post stored them (V-0072) against the six on the keyed invoice of step 4 (V-0073). |
| Item 6: composition, a changed GSTIN, a type bound to one unit | | Composition raw (RU Part A); the binding raw (`lab/2464-gr-b-answers` 01899d61, `lab/2464-gn-sheet`). | **Left out**: not in this row's task; each is its own keyed path, and the build reads no binding field (it refuses two registrations as ambiguous before any binding could matter). |
| An invoice filed under a second registration | | | **Left out**: the build refuses that book, so no build post can reach it; it needs a newer builder (#2464). |

Every part runs on TallyPrime 7.2 Silver, on the synthetic book below, with the 8c3fb9f2 build already on the lab disk. No part needs a person: the lab keys in Tally, launches the build with `client.py`, takes the backup and the restores, and in Part G answers the approval box by mouse.

## The question of each part, and its one variable
- **Part P (item 1).** Does the build's `gst_status` read a keyed post-dated Sales invoice as `post_dated`, with its three flags reading included? Variable: one keyed voucher, `BP/26-27/0073`, against the read before it (launch L0 against L1). The current date is moved for the keying and set back before L1.
- **Part R-B (inactive).** With the one registration inactive from 1-Sep-2026, does the build admit an invoice dated 1 Aug 2026 and refuse one dated 15 Sep 2026 as `invoice_company_registration_inactive`? Variable: the registration's status, against L1's builds of the same two calls.
- **Part R-C2 (two registrations).** With a second GST registration in force, does the build refuse as `invoice_company_registration_ambiguous` on both dates? Variable: the second registration, against L0's builds (the book restored to BK1 between).
- **Part G (the split, through the build).** Built and posted by the build, does `BP/26-27/0072` move the HSN summary by exactly what row 2162-HS's raw import of the same invoice moved it by? Variable: the sender (the build's own request and post, instead of a raw import of the build's earlier request), from the same book state as HS (BK1, marks 68/245). Beside it, the build's six block fields are compared with the keyed invoice's of Part P (step 14; more than one variable, so partial).

Parts P and R send no write through the build: `build_import_xml` writes a batch file in the launch's own data folder and sends nothing to Tally (step 3 checks it by the marks). Part G holds the row's one post and its one approval box, last, so a box that fails costs no other part.

## Book
`BRIDGE PILOT LAB` (synthetic), GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, loaded alone. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2464-GN's and 2162-HS's) applies in full; where it and this sheet differ, the stricter one holds.

- **Pinned by GUID.** `count19.py` and `calls.py` hold the GUID as `BOOK`; `list_companies` must name it. Any other GUID: STOP, whatever the company is called.
- **Its state.** Every row run on this book ends with a restore that leaves ALTVCHID 68 and ALTMSTID 245, current date 15-Sep-2026, period 1-Apr-26 to 31-Mar-27 (HS read these). The lab's state file for 23 Nov lists other books and not this one. So step 1 is a precondition: this book in Select Company, its GUID, and 68/245. If any differs: STOP before the backup, with nothing keyed or launched.
- **The numbers.** `BP/26-27/0072` (Part G), `0073` (Part P) and `0074` (Part R, built only, never posted).
  - Row 2162-HS created `0072` by a raw import and restored it away (its s7: N-0072 0 rows). Reusing it keeps HS's compare to one variable.
  - A search of every `lab/` branch (750 refs) and of master for `26-27/0073` and `26-27/0074` found none; `0072` is found only in row 2162-HS's files and in two notes that cite HS (row 2162-RP's sheet and the lab queue).
  - Step 2's N reads stay a STOP, in case either check is wrong.
- **BK1 is the way back.** Taken at step 1, before anything is keyed; restored at steps 8, 11 and 16 by C6. After each restore the reads are compared whole with step 2's.
- Write down the release shown on Tally's screen.

## Build
Only the NEW build of row 2569-R (`lab/2569-restore-sheet` 208932f7, its Builds section), already on the lab disk and granted:

NEW RC COMMIT: 8c3fb9f2277966fa98b31bdc775b65c5ea3c29f7  ZIP DIGEST: f8dc02e1f7bbb134e5ecc929c1f567aa265df2d3d1c866fea6636270e52e9472  MCPB archive_sha256: 445d45a6fe18035258f69f04c7538241f19c80a39ad9ad88165b8a7c9f15ac8f  EXE SHA256: 1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59

EXE: `C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`. Hash it (Get-FileHash) before every launch. Any other hash: STOP; the exe is never copied, moved or replaced by this row. The OLD build is not used.

## Launches
Five launches, each over stdio under `client.py`, byte-equal to row 2569-R's (and so to row 2440-CP's), started the same way for all five: `python3 -I client.py <RUNDIR> <EXE> 1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`, from the same shell, the shell's window as row 2569-R's Part B had it. Record the command, the shell, and the process list at the start of each.

| Launch | Step | RUNDIR | DATADIR | Calls, in order |
| --- | --- | --- | --- | --- |
| L0 | 3 | `C:\bridge-lab\2162-sales-q\run-0` | `C:\bridge-lab\2162-sales-q\data-0` | 01 to 07 |
| L1 | 5 | `...\run-1` | `...\data-1` | 01 to 07 |
| LB | 7 | `...\run-b` | `...\data-b` | 01, 02, 03, 05, 06, 07 |
| LC | 10 | `...\run-c` | `...\data-c` | 01, 02, 03, 05, 06, 07 |
| LG | 13 | `...\run-g` | `...\data-g` | 01, 02, 03, g05, g05m, the posts, the verify, g09, g10 |

- **Folders.** Make each launch's folders with `python3 -I mkenv.py <RUNDIR> <DATADIR>`. It refuses a folder that exists, makes the empty `inbox` and `answers`, writes `launch-env.json` without a shell (row 2569-R's rule: a shell once rewrote `\b` in a path), reads it back and prints its sha256. Record that output. Every launch has the same five keys, the data folder aside: host 127.0.0.1, port 9001, the data folder, terms accepted, posting on. Posting is on for every launch, the ones that post nothing included, because the build makes an invoice only while posting is on (`invoice_post_not_enabled`) and so the setting is never a variable between launches. **`client.py`'s header says the owner adds the posting setting and the lab never writes it.** Here `mkenv.py` writes it. That needs the run coordinator's word, under the owner's standing authorization for this week, recorded before step 0; without it this row does not start.
- **Calls.** Copy the call files from this folder's `calls\` into the launch's `inbox\` **one at a time**, each only after the previous one's answer is in `answers\` and `client-log.txt` names it. Never edit a call file; never copy one twice into one launch. A call file names the book's GUID; `02` checks it.
- **Reading an answer.** `python3 -I calls.py show <RUNDIR>\answers\<name>.json` after each. Its STOP (exit 2) means the call failed: no answer, not JSON, a JSON-RPC error, no structured content. That is never read as "nothing found" or "refused". A refusal is an answer: `refused <code>`.
- **Closing a launch.** Create the empty file `<RUNDIR>\inbox\stop`; EXPECT `client-log.txt` to end `session ended; server pid <pid> exit code 0`, and no `bridge_mcp` process left from that pid. Never end a process (C2). A server that does not end: STOP and report.
- **No raw request while an exe runs, and no exe while a raw request or a key is in progress.** No other ComplyEaze Bridge makes a call to port 9001 during a launch: record the process list; the desktop app's connectors are left running, as rows 2162-S1 and 2569-R left them, and this is recorded.
- **Bounded logs.** `client.py` writes one line a call; `calls.py` prints a fixed set of fields a call (at most one line an item for `gst_status`, whose page is at most 500 items) and at most 200 lines a request compare; `run.py` keeps an answer above 2,000,000 bytes and stops. The server's error output goes to one file a launch; record its size, and do not read it unless a step says so.

## The approval box (Part G only)
Read with row 2569-R's two read-only helpers before it is answered (that sheet's "Reading a box"): `boxstate.ps1` (new in 2569-R, byte-equal) and `readwin.ps1` (byte-equal to row 2440-CP's).
- **The helper's pid.** The child of the server (pid in `<RUNDIR>\server.pid`) whose command line holds `--confirm-journal`: `Get-CimInstance Win32_Process -Filter "ParentProcessId=<server pid>"`, every 2 seconds from the post's send, up to 60 seconds. Record its command line whole; EXPECT the EXE path above. None in 60 seconds: record it, press nothing, and wait for the post's answer (step 13.8).
- **The read.** `powershell -NoProfile -File boxstate.ps1 -ProcId <helper pid> -Out <RUNDIR>\box-1.txt`, then `powershell -NoProfile -File readwin.ps1 -ProcId <helper pid> -Out <RUNDIR>\box-text.txt` with the same pid, then one screenshot of the whole screen. If `windows 0`, read again every 2 seconds to `box-2.txt` and on, up to 10 seconds.
- **Nothing else is done on the PC while the box is up.** No key is pressed anywhere: No is the box's default button, and in row 2569-R a box closed within 4 seconds of opening, likely to a key pressed elsewhere. No console is typed into. Only the mouse answers the box.
- **The Yes rule.** Click Yes **only if all of these hold** in the first read that shows the box:
  - one window titled `ComplyEaze Bridge — post this voucher?`, class `#32770`, `iconic=False`, `visible=True`, `topmost=True` (row 2569-R's W1 and W4);
  - the box's text (`box-text.txt`) holds the lines `Number: "BP/26-27/0072"` (after `Voucher type: "BRIDGE Sales"  Date: 1-Aug-2026` on the same line), `Customer: "BRIDGE Walk-in"` and `Company GUID: 6b43e498-430c-4d5c-bfef-d32e2ab93c85`, and names no other voucher number. The form is row 2440-CP's box on this build family (`lab/2440-composed-answers`, `evidence/s8-window-a.txt`). Copy the box text whole into the notes; the date's form, the legs and the totals are recorded, not graded;
  - the helper's command line is the EXE path above.
  Then one left click on the button whose text is `Yes` (UI Automation reads it as a Pane named `Yes`), located on the screenshot. The box also takes the keys Y and N: no key, only the click. Record the time. W2 (in front) and W3 (focus on No) are recorded beside it, not graded.
- **If any of them fails:** one left click on `No`, record why, and go on as step 13.8 says for a declined box. If the box is minimised or cannot be clicked: press nothing; it times out on its own, and step 13.8 says what follows.

## Files
**Byte-equal to earlier rows' files of the same name** (check: `SHA256.txt`; each was also compared with its source branch's own `SHA256.txt`):
- From row 2464-GN (`lab/2464-gn-sheet` 23b2e57b, folder `lab/2464-gn/`), whose ROW.md gives each one's origin: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count19.py`, `M-marks.xml`, `U-taxunits.xml` (the build's tax-unit request), `R-0010.xml` (the build's invoice read-back, with `ISPOSTDATED`), `template-n-0001.utf16le.xml`, `template-full-0057.utf16le.xml`, `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`; **never sent**: step 13 compares the build's new request with it).
- From row 2162-HS (`lab/2162-hs-sheet` b6767b87): `N-0072.xml`, `R-0072.xml`. `gen.py` writes both again and asserts each byte-equal to HS's.
- From row 2162-HS's answers (`lab/2162-hs-answers` d7a1adf8, `answers/a-s4r-R-0072.xml`), renamed `a-hss4r-R-0072.xml` so that `count19.py` counts it as R: HS's read-back of its raw `0072`, 94,478 bytes, sha256 `8908800f…`, for step 14's compare.
- From row 2569-R (`lab/2569-restore-sheet` 208932f7): `client.py` (sha256 `8d3729ea…`), `boxstate.ps1` (`fd0ab80f…`), `readwin.ps1` (`c138b645…`).

**Written for this row:**
- `gen.py`: from the templates above, `N-0072.xml`, `N-0073.xml`, `N-0074.xml`, `R-0072.xml`, `R-0073.xml`, `V-0072.xml`, `V-0073.xml`, each with only the voucher number changed, asserted once each; each template's sha256 is checked first. No import is written: this row sends no raw write.
- `calls.py`: reads `client.py`'s answers by parsing JSON (show, status-diff, batch, reqdiff) and writes the two calls that carry a batch id (post, verify). Its exit 2 is "the call failed"; its `batch` FINDING (exit 1) is a result.
- `mkenv.py`: one launch's folders and `launch-env.json`.
- `calls\*.json`: the fixed calls. `01` tools/list; `02` list_companies; `03`, `07`, `g05m`, `g10` ledger_masters (the marks: `snapshot.master_alter_id` M, `snapshot.voucher_alter_id` V, `snapshot.reused` false, as row 2440-CP defines MARKS); `04`, `g09` gst_status of `BRIDGE Sales` for 2026-08-01 to 2026-08-31; `05`, `06` build_import_xml of `BP/26-27/0074` dated 2026-08-01 and 2026-09-15, with their own `bridge_txn_id` (`sq-r1`, `sq-r2`; a build is refused for a row only once that row was sent, `agent_import.rs:1666` to `:1690`, and these are never sent); `g05` build_import_xml of `BP/26-27/0072` dated 2026-08-01. The invoice is `pilot_invoice` of `agent_import_invoice_admission_tests.rs` at 8c3fb9f2 (the build's own test of this book), with the number and date named: `BRIDGE Walk-in` Dr 1,051.00; `BRIDGE Svc 998313 5%` 600.40; `BRIDGE Svc 998314 5%` 400.40; `BRIDGE CGST 2.5%` 25.02; `BRIDGE SGST 2.5%` 25.02; `BRIDGE Round Off` 0.16; type `BRIDGE Sales`, place of supply Rajasthan. It is row 2162-HS's `0072` invoice, leg for leg.

## Self-checks run on macOS before commit (23 Nov 2026)
- Every reused file's sha256 equals its source's `SHA256.txt` line.
- `python3 -I gen.py`: seven files written; `N-0072.xml` and `R-0072.xml` byte-equal to HS's (asserted). `python3 -m py_compile` on `calls.py`, `gen.py`, `mkenv.py`, `client.py`, `run.py`, `count19.py`: no error.
- `count19.py` on captured answers, each exit 0: M (HS s1), N (HS s2 with 0 rows, s4r with 1), R (HS s4r R-0072), U (RU s2, sB, sC2). `--same-row` HS s1 against s7: equal; against s4r: STOP (exit 1), as it must. `--before-write` HS s1 against s4r: passes. `--diff` HS s2 R-0010 against s7: `lists compared: 1`, `legs: 6 -> 6`. `--unit-diff` RU s2 against sB: `Rajasthan Registration` 9 leaves differ (ALTERID 206 to 246, and the eight leaves of a new dated row: FROMDATE 20260901, ISINACTIVE Yes, STATE and PLACEOFSUPPLY Rajasthan, REGISTRATIONTYPE Regular, GSTR1PERIODICITY Monthly, ISSTATECESSON No, ISOTHTERRITORYASSESSEE empty); RU s2 against sBr: 0.
- `calls.py show` on row 2569-R's 15 Part B answers (the same build, `lab/2569-restore-answers` 400575f3): every field printed, including `pending` with `dialog_remaining_s` 96 and `retry_after_s` 15 on the first post call, and `posted_verified` on the second; on row 2440-CP's timed-out post: `refused import_approval_timed_out`; on a missing file: STOP, exit 2. `status-diff` on two `gst_status` answers of `lab/1633-export-ready-answers`: runs, items matched by master id. `post` writes a call and refuses to overwrite one, and refuses a refused build (exit 2); `batch` on a missing file and on a wrong-hash file: FINDING, exit 1. `reqdiff` of a file with itself: 0 lines.
- `mkenv.py` on a new folder: the five keys, none that `client.py` refuses; on an existing folder: STOP.
- A model of `parse_company_registration` (8c3fb9f2, written in Python for this check only, not shipped) on RU's captured tax-unit answers: s2 one unit in force on both dates; sB (inactive from 20260901) one in force on 20260801 and inactive on 20260915; sC2 (two units) ambiguous on both. These are the EXPECT lines of steps 7 and 10. They predict; they are not evidence.
- Not run on macOS: `client.py` against a server, `boxstate.ps1`, `readwin.ps1` (Windows only), any Tally request.

## Steps
Raw reads: `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, then `python3 -I count19.py a-<label>-*`. A STOP from either stops the row. Check each request file's sha256 against `SHA256.txt` before its send. Copy each script's whole output into the notes.

**The prompt rule, at every step.** The only prompts answered are C5's, C6's, the ones steps 4, 6 and 9 name with their keys, and the approval box by step 13's Yes rule. Any other prompt, message or dialog: screenshot it, copy its text word for word, answer No (or Esc if it offers no No, never on a master or voucher screen, C7), press nothing more, send nothing. Then leave any open screen by Ctrl+Q (C7), restore BK1 by C6, compare as step 16, and close (C10). That prompt is this row's result.

**The wait rule.** After a save, a Ctrl+A or a send: if 5 minutes pass with no screen change, no answer and no message, press nothing, send nothing, screenshot, report. With the Gateway showing and no dialog, run C2's status check before the next read.

**The path rule.** Every keyed screen follows the recorded path cited at its step. If a line or screen is not exactly as named: stop, Esc out of a list or prompt (never out of a creation or alteration screen, C7), restore BK1 by C6, check as step 16, close (C10).

0. **Before the sitting.**
   - The run coordinator's word that the lab may write `BRIDGE_AGENT_ENABLE_WRITES` into each `launch-env.json` (Launches). Without it: do not start.
   - Check every file against `SHA256.txt` (Get-FileHash).
   - `python3 -I gen.py`: EXPECT the seven lines and each sha256 equal to `SHA256.txt`'s.
   - `[System.Management.Automation.Language.Parser]::ParseFile` on `boxstate.ps1` and `readwin.ps1`: 0 errors each. Run `boxstate.ps1` once against the running `tally.exe`: EXPECT one WINDOW line and a FOREGROUND and a FOCUS line (row 2569-R's check). Either failing: STOP before step 1.
   - Hash the EXE against the build line.
   - C2 step 0, with no company loaded.
1. **Load, check and back up.**
   - Load BRIDGE PILOT LAB alone from Select Company (C1). Not in the list, or any prompt: STOP, press nothing.
   - Write down the Gateway's current date and period. EXPECT 15-Sep-2026 and 1-Apr-26 to 31-Mar-27.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245. Anything else: STOP here, before BK1, with nothing keyed or launched.
   - **BK1** by C5, to the new empty folder `C:\lab\2162-sales-q\backup-1` (Tally may show it with its own capitals). Record the file's name, size, sha256 and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml N-0072.xml N-0073.xml N-0074.xml R-0010.xml M-marks.xml`.
   - U: EXPECT two TAXUNIT rows, `Default Tax Unit` and `Rajasthan Registration`; the latter with GSTREGNUMBER `08ZZZZZ0000Z1ZQ` and one dated row: FROMDATE 20260401, STATE Rajasthan, REGISTRATIONTYPE Regular, ISINACTIVE No (row 2162-RU's s2).
   - N-0072, N-0073, N-0074: EXPECT 0 rows each. One row or more: STOP before step 3, with each row recorded whole.
   - R-0010: EXPECT one row, `BRIDGE Walk-in`, included Yes, every other status flag No, `ISPOSTDATED` No, 6 legs.
   - M: equal to s1 (`count19.py --same-row a-s1-M-marks.xml a-s2-M-marks.xml`).
   - Record each answer's sha256: step 16 compares against them.
3. **L0: the build on the book before** (Parts P's and R's control).
   - `python3 -I mkenv.py C:\bridge-lab\2162-sales-q\run-0 C:\bridge-lab\2162-sales-q\data-0`; hash the EXE; start `client.py` (Launches).
   - Calls `01` to `07`, each read with `calls.py show`:
     - `01`: EXPECT none of this row's six tools missing (`gst_status` is in the build's registered list, `agent_catalog.rs:457`, and is not filtered by any setting). `gst_status` alone missing: Part P's reads (`04` here and at step 5) are skipped and recorded; steps 4 to 17 run without them. Any other missing: close the launch, STOP (posting off, or not this build).
     - `02`: EXPECT one company, `BRIDGE PILOT LAB`, `is the book's: yes`. Otherwise close the launch, STOP before any other call.
     - `03`: EXPECT M 245, V 68, reused False.
     - `04`: EXPECT not refused; state `complete`; no `next_offset`; `post_dated` 0 in the counts; `BP/26-27/0010` among the items, date 20260801, `included`. Record every item line whole: this is Part P's before. `next_offset` present: record, and Part P's compare in step 5 is then not run (one page only).
     - `05` (dated 2026-08-01) and `06` (2026-09-15): EXPECT each not refused, `voucher_count 1`. Record the batch id, sha256 and the warnings whole. Then `python3 -I calls.py batch <answer> C:\bridge-lab\2162-sales-q\data-0` for each; record its line (a FINDING is recorded, the row goes on). **A refusal here is a result** (the build refuses this book as it stands): record it whole; then steps 7 and 10 cannot show a change for that date, and they run only for a date admitted here.
     - `07`: EXPECT M 245, V 68: nothing was written to Tally.
   - Close the launch (Launches). Then `s3`: `M-marks.xml`; `count19.py --same-row a-s1-M-marks.xml a-s3-M-marks.xml`: EXPECT it passes. A difference: STOP, and report that a build launch moved a mark.
4. **Part P: key one post-dated invoice** (row 2162-S1's path, `lab/2162-s1-answers` 20ed166c, which followed `lab/1342-w21c-answers` 19d50259).
   - `s4m`: `M-marks.xml`; `count19.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml`: it must pass. Then no key is pressed in Tally but those below.
   1. At the Gateway (shot), F2: set the current date to `10-Aug-2026`. EXPECT the Gateway to show 10-Aug-2026 (shot).
   2. Open Vouchers, choose the type `BRIDGE Sales`, and key one invoice dated `25-Aug-2026`:
      - mark it post-dated with **Ctrl+T** before saving; EXPECT the screen's status to read Post-Dated (shot);
      - the number field: Tally gives none on this book (S1); type `BP/26-27/0073`;
      - party `BRIDGE Walk-in`; one line on `BRIDGE Svc 998313 5%` at 1,000.00; then `BRIDGE CGST 2.5%` 25.00 and `BRIDGE SGST 2.5%` 25.00, as Tally computes them; total 1,050.00. Choose each tax ledger by typing until one row is left; `BRIDGE CGST` alone matches two ledgers, and Enter is never pressed while two match (W21c's pause);
      - the Dispatch Details and Party Details sub-screens: accept as offered, with nothing typed (S1);
      - shot of the whole voucher before the save; save with its own accept. Any other prompt: the prompt rule.
   3. EXPECT the Gateway after the save; W21c found its current date then read 25-Aug-2026. Write down what it shows. F2: set the current date to `15-Sep-2026`. EXPECT 15-Sep-2026 (shot). If it cannot be set back: STOP; the restore at step 8 sets it back (S1).
   - `s4`: `N-0073.xml R-0073.xml V-0073.xml R-0010.xml M-marks.xml`.
     - N-0073: EXPECT one row, with a VOUCHERNUMBER. Zero rows: the number was not saved as typed; STOP, restore, report.
     - R-0073: EXPECT one row, DATE 20260825, `BRIDGE Walk-in`, `ISPOSTDATED` Yes, included Yes, uncertain No, excluded No, `ISCANCELLED` and `ISOPTIONAL` No (S1's R-0045). Record it whole.
     - V-0073: record it whole (the keyed invoice's native fields).
     - R-0010: `count19.py --diff a-s2-R-0010.xml a-s4-R-0010.xml`: EXPECT `lists compared: 1` and `legs: 6 -> 6` only.
     - M: EXPECT V 69, M 245.
     - R-0073 not reading `ISPOSTDATED` Yes: record it whole; Part P's result is then raw already, and step 5 still runs and records what the build reads.
5. **L1: the build after the keyed invoice.**
   - `mkenv.py ...\run-1 ...\data-1`; hash; launch. Calls `01` to `07`, as step 3.
     - `03` and `07`: EXPECT M 245, V 69.
     - `04`: `python3 -I calls.py status-diff C:\bridge-lab\2162-sales-q\run-0\answers\04-status-aug.json C:\bridge-lab\2162-sales-q\run-1\answers\04-status-aug.json`. EXPECT exactly one line `in B only`, for `BP/26-27/0073`: date `20260825`, `gst_status` `post_dated`, `status_flags` `included`, `accepted_as_it_stands` false; no other item line; the counts equal but `post_dated` 0 to 1 and the total one more. Record the whole output.
     - `05` and `06`: EXPECT as at L0 (not refused, `voucher_count 1`). These are the control for step 7.
   - Close the launch. `s5`: `M-marks.xml`; EXPECT V 69, M 245 (`count19.py --same-row a-s4-M-marks.xml a-s5-M-marks.xml`).
6. **Part R-B: the registration made inactive from 1-Sep-2026** (row 2162-RU's Part B, `lab/2162-ru-answers` c7b14084, `evidence/sB-screens.txt` 4.1 to 4.7, keyed there from 68/245; here the book also holds `0073`, which no registration field names).
   - `s6m`: `M-marks.xml`; `count19.py --before-write a-s1-M-marks.xml a-s6m-M-marks.xml`: it must pass.
   1. At the Gateway (shot): Down once, `Alter` highlighted; Enter. `Master Alteration`; Down 11 to `GST Registration`; Enter. `List of GST Registrations`: Create, Back, one line `Rajasthan Registration`; Enter on it.
   2. `GST Registration Alteration` (shot). EXPECT RU's 4.1: Registration status Active; State Rajasthan; Registration type Regular; Assessee of Other Territory No; GSTIN/UIN 08ZZZZZ0000Z1ZQ; Periodicity of GSTR-1 Monthly; the rest as RU recorded. Write down every field.
   3. Shift+Tab twice: `Registration status: Active` with its list (Active highlighted, Inactive). Down once: `Inactive` highlighted (zoom); Enter.
   4. **Named prompt 1, the "Effective Date" box**, one field "Effective Date for revised GST details" with a "List of Effective Dates". Write down every line of the list (RU's held 15-Sep-26 twice and two computer dates; this book's "Date of Last Entry" may differ). Type `1-9-2026`. EXPECT the field to read 1-9-2026 and the list one line, `1-9-2026  New Effective Date`. Enter.
   5. **Named prompt 2, the "New Effective Date" box**, one field reading `1-Sep-26` (zoom). If exactly so: Enter once. Any other date or field: the prompt rule.
   6. **Named prompt 3**, the form's own `Accept ?`, with Registration status Inactive and every other field as at 6.2 (shot). Press `y`.
   7. EXPECT `List of GST Registrations` again; Ctrl+Q once: the Gateway, no question (shot).
   - `s6`: `U-taxunits.xml R-0010.xml M-marks.xml`.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s6-U-taxunits.xml`. EXPECT the same two names on both sides; `Default Tax Unit` 0 leaves differ; `Rajasthan Registration` 9 leaves differ, exactly RU's s2-to-sB nine (Self-checks): ALTERID to 246, and a new dated row FROMDATE 20260901, ISINACTIVE Yes, STATE Rajasthan, PLACEOFSUPPLY Rajasthan, REGISTRATIONTYPE Regular, GSTR1PERIODICITY Monthly, ISSTATECESSON No, ISOTHTERRITORYASSESSEE empty. Record the whole output.
     - M: EXPECT V 69, M 246.
     - A different unit diff: record it whole; step 7 still runs (its result is then read against the shape recorded here, and RU's shape is not reproduced; say so).
7. **LB: the build with the registration inactive.**
   - `mkenv.py ...\run-b ...\data-b`; hash; launch. Calls `01`, `02`, `03`, `05`, `06`, `07`.
     - `03`, `07`: EXPECT M 246, V 69.
     - `05` (1 Aug): EXPECT not refused, `voucher_count 1`: the dated row before 1 Sep is in force, Active.
     - `06` (15 Sep): EXPECT `refused invoice_company_registration_inactive`.
     - Each other answer is a result: record it whole. A refusal at `05`, or `06` admitted, is this row's finding for R-B.
   - Close the launch.
8. **Restore BK1 (R1)** by C6, `restore-plan.txt` written first. Then `s8`: `M-marks.xml U-taxunits.xml N-0073.xml R-0010.xml`.
   - `count19.py --same-row a-s1-M-marks.xml a-s8-M-marks.xml`: EXPECT it passes (68/245).
   - `count19.py --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml`: EXPECT 0 leaves differ in each unit.
   - N-0073: EXPECT 0 rows. R-0010: `--diff` against s2: EXPECT `lists compared` and `legs: 6 -> 6` only.
   - EXPECT each answer's sha256 equal to s1's or s2's of the same file. Write down the Gateway's current date: EXPECT 15-Sep-2026.
   - Any of these not equal: send nothing more, report before anything else; the book is not as it was and the coordinator decides.
9. **Part R-C2: a second GST registration** (row 2464-GN's step 3, `lab/2464-gn-sheet` 23b2e57b, which is row 2162-RU's Part C2; `lab/2464-gr-b-answers` 01899d61 steps 3.1 to 3.7 recorded each screen).
   - `s9m`: `M-marks.xml`; `count19.py --before-write a-s1-M-marks.xml a-s9m-M-marks.xml`: it must pass.
   1. At the Gateway (shot): Create, Enter. `Master Creation`; Down 9 to Godown, once to the heading, once more: `GST Registration` highlighted (zoom). Enter.
   2. **Named prompt 1.** EXPECT, word for word, "GST Registration already exists for the Company." then "Do you want to alter the existing GST Registration or create a new GST Registration?", with `C Create New` and `A Alter Existing` (zoom). If the words are the same: `c`.
   3. `GST Registration Creation` (shot). Write down every field. EXPECT Registration status Active, State empty, Registration type Regular, Assessee of Other Territory No, GSTIN/UIN empty, Periodicity of GSTR-1 Monthly.
   4. State: type `Maha`; EXPECT `Maharashtra` the one line; Enter. Registration type: Down once, `Regular` highlighted (zoom); Enter. Assessee of Other Territory: No, Enter. GSTIN/UIN: type `27ZZZZZ000`, then `0Z1ZQ`, zooming after each; before any Enter it must read `27ZZZZZ0000Z1ZQ`, 15 capitals (shot). Then Enter on each later field, leaving it as Tally filled it.
   5. **Named prompt 2.** `Accept ?` (shot): `y`.
   6. **Named prompt 3.** EXPECT the three paragraphs 2464-GN quotes, word for word ("You can create a unique voucher numbering series to cater to multiple GST registrations." / "You can do this by going to the Chart of Accounts and selecting the relevant voucher types" (no full stop) / "Open Chart of Accounts and create now?"), with Yes or No (zoom). If the same: `n`.
   7. EXPECT a new, empty `GST Registration Creation` form. Type nothing. Ctrl+Q, then Ctrl+Q again: `Master Creation`, then the Gateway (shot).
   - `s9`: `U-taxunits.xml M-marks.xml`.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s9-U-taxunits.xml`. EXPECT the names `Default Tax Unit`, `Maharashtra Registration` and `Rajasthan Registration` after; `Maharashtra Registration` in the second only, with GSTREGNUMBER `27ZZZZZ0000Z1ZQ` and one dated row FROMDATE 20260401, STATE Maharashtra, REGISTRATIONTYPE Regular, ISINACTIVE No (RU's sC2); the two others 0 leaves differ. Record the whole output.
     - M: EXPECT V 68, M 246 (2464-GR-b's 246).
10. **LC: the build with two registrations in force.**
    - `mkenv.py ...\run-c ...\data-c`; hash; launch. Calls `01`, `02`, `03`, `05`, `06`, `07`.
      - `03`, `07`: EXPECT M 246, V 68.
      - `05` and `06`: EXPECT each `refused invoice_company_registration_ambiguous`.
      - Any other answer is the finding: record it whole.
    - Close the launch.
11. **Restore BK1 (R2)** by C6. Then `s11`: as step 8, with `N-0073.xml` replaced by `N-0074.xml`; the same EXPECTs (0 rows for N-0074).
12. **Part G: the HSN summary before** (row 2162-HS's step 3, on screen only, nothing saved; its keys are recorded in `lab/2162-hs-answers` d7a1adf8, `evidence/s5-screens.txt`).
    1. At the Gateway (shot): click the `G: Go To` label; type `GSTR-1`; the List of Reports, no line highlighted; Down once: `GSTR-1` highlighted; Enter. A list of GST registrations: it holds one line, `08ZZZZZ0000Z1ZQ`; Enter on it (HS saw none). More than one line: the path rule.
    2. EXPECT the period 1-Aug-26 to 31-Aug-26, with no period key pressed. Any other: write it down and STOP (the restore at step 16 follows).
    3. Write down the counts. EXPECT HS's before: Total Vouchers 39, Included in Return 27, Not Relevant 7, Uncertain 5; B2C (Small) 27 (2).
    4. Down 10, 12, 1, 1 (24 in all): `HSN Summary - 12 (B2B - B2C Supplies)` highlighted (zoom); Enter. One list, headed B2C Supplies. Write down every line: HSN/SAC, UQC, total, rate, taxable value, each tax and the tax amount, then Sub Total and Total. EXPECT HS's nine lines before, among them 998313 at 11,137.40 / 10,607.00 / 265.21 / 265.19 / 530.40, 998314 at 1,366.70 / 1,301.60 / 32.55 / 32.55 / 65.10, and Total 14,275.20 / 13,508.70 / 383.26 / 383.24 / 766.50.
    5. Down 2 from 847130 to `998313` (zoom); Enter once: "GSTR-1 - Voucher Register", "Vouchers of 998313". Write down every line. EXPECT HS's 19 lines, `BP/26-27/0010` at 630.42 / 600.40 / 15.01 / 15.01. Esc once. The same on `998314`: EXPECT HS's 7 lines. Esc once.
    6. Esc one screen at a time to the Gateway (shot), writing down each screen passed.
    - A count or line that differs from HS's: record it whole and go on. Step 15's compare is against this step's reading, not HS's.
13. **LG: the build builds and posts `BP/26-27/0072`.**
    1. `mkenv.py ...\run-g ...\data-g`; hash; launch. Calls `01`, `02`, `03` as step 3 (EXPECT M 245, V 68).
    2. `g05`: EXPECT not refused, `voucher_count 1`. Record the answer whole. A refusal: record it whole, close the launch, go to step 16.
    3. `python3 -I calls.py batch <run-g>\answers\g05-build-0072.json C:\bridge-lab\2162-sales-q\data-g`; record its line. If found: `python3 -I calls.py reqdiff template-import-0010.utf16le.xml <the file it names>`; record the whole output (the build's request of 0010 at an earlier master against its request of 0072 at 8c3fb9f2; recorded, not graded).
    4. `g05m`: EXPECT M 245, V 68, immediately before the post.
    5. `python3 -I calls.py post <run-g>\answers\g05-build-0072.json <run-g>\inbox\g06-post-1.json`. That copy is the send. From that moment no key is pressed anywhere.
    6. **The box**: find the helper, read the box, then the Yes rule (The approval box). Record the times of the box's first read and of the click, and the button clicked.
    7. Wait for `g06-post-1`'s answer; `calls.py show` it.
       - `approval` `pending`: wait `retry_after_s` seconds, then `calls.py post ... <run-g>\inbox\g06-post-2.json`, and so on (`-3`, `-4`).
       - `approval` `approved`: the next call at once. This is the tool's flow, not a retry.
       - At most four post calls; a fifth would be needed: STOP, close the launch, go to step 14.
    8. **The final post answer.** EXPECT `dispatch state posted_verified`, counters `created` 1 and `altered`, `deleted`, `errors`, `ignored`, `cancelled`, `exceptions` 0, `verification_status posted_verified`. Then, before any other call, screenshot the whole Tally window: a dialog or a half-entered screen = C7, STOP.
       - `refused import_approval_timed_out`, or a declined box (No clicked): record it whole; `g10`; close the launch. Then `s14x`: `N-0072.xml M-marks.xml`: EXPECT N-0072 0 rows and V 68, M 245 (nothing posted). Skip steps 14 and 15; go to step 16. This is Part G's result: the lab's own answer did not post, and the split is not measured.
       - Any other final state: record it whole; `g10`; close; then step 14, whose EXPECTs are then recorded against what N-0072 shows; skip step 15 unless N-0072 holds one row and R-0072 its six legs.
    9. `python3 -I calls.py verify <run-g>\answers\g05-build-0072.json <run-g>\inbox\g08-verify.json`: EXPECT `verification_status posted_verified`.
    10. `g09`: `calls.py status-diff <run-0>\answers\04-status-aug.json <run-g>\answers\g09-status-aug.json`. EXPECT exactly one line `in B only`: `BP/26-27/0072`, date `20260801`, `included`, no `status_flags`. Record the whole output.
    11. `g10`: EXPECT M 245, V 69.
    - Close the launch.
14. **What Tally stored.** `s14`: `N-0072.xml R-0072.xml V-0072.xml R-0010.xml M-marks.xml`.
    - N-0072: EXPECT one row. R-0072: EXPECT one row, DATE 20260801, `BRIDGE Walk-in`, the six legs as sent (-1,051.00; 600.40; 400.40; 25.02; 25.02; 0.16), included Yes, uncertain and excluded No, `ISPOSTDATED` No.
    - `count19.py --diff a-hss4r-R-0072.xml a-s14-R-0072.xml`: HS's raw `0072` against the build's. Record the whole output. EXPECT the legs `6 -> 6`; every field that differs is recorded, not judged (identity fields will differ: GUID, MASTERID, ALTERID, and the keys Tally makes).
    - **The block, built against keyed.** `count19.py --diff a-s4-V-0073.xml a-s14-V-0072.xml`, and the six block fields of each as `count19.py` prints them (`BASICBUYERNAME`, `PARTYMAILINGNAME`, `CONSIGNEEMAILINGNAME`, `CONSIGNEESTATENAME`, `CONSIGNEECOUNTRYNAME`, `VATDEALERTYPE`). Record both whole.
      - EXPECT, on V-0072, what the build writes and row 2162-BC found Tally stores as sent: the three names `BRIDGE Walk-in`, `CONSIGNEESTATENAME` Rajasthan, `CONSIGNEECOUNTRYNAME` India, `VATDEALERTYPE` Regular.
      - EXPECT V-0073's three names and two places to equal V-0072's. `VATDEALERTYPE` on the keyed invoice is recorded, not graded: nine of ten keyed invoices of an earlier book read `Unknown` and one `Regular` (the reference's PARTIAL bullet), and the cause is not known.
      - The two invoices differ in more than the sender (date, post-dated flag, lines, amounts). So this compare says whether the build's block matches what a keyed invoice gets, partial, and nothing about why a field differs.
    - V-0072: record also the registration fields as `count19.py` prints them.
    - R-0010: `--diff` against s2: EXPECT only `lists compared` and `legs: 6 -> 6`. M: EXPECT V 69, M 245.
15. **The HSN summary after.** Step 12 again in full, with the same keys. EXPECT:
    - the counts Total 40, Included 28, Not Relevant 7, Uncertain 5 (one more Total and Included than step 12);
    - 998313 up by exactly 630.42 / 600.40 / 15.01 / 15.01 / 30.02 and 998314 by exactly 420.42 / 400.40 / 10.01 / 10.01 / 20.02 against step 12's lines; every other line unchanged; the Total up by 1,050.84 / 1,000.80 / 25.02 / 25.02 / 50.04 (the 0.16 round off on no line), as HS;
    - under 998313 and 998314, step 12's lines unchanged plus one line `BP/26-27/0072` at 630.42 / 600.40 / 15.01 / 15.01 and 420.42 / 400.40 / 10.01 / 10.01.
    - Then `s15`: `M-marks.xml`; `count19.py --same-row a-s14-M-marks.xml a-s15-M-marks.xml`: EXPECT it passes (the look changed nothing).
16. **Restore BK1 (R3)** by C6 (it runs after any STOP from step 4 on, on the coordinator's word as C8 says). Then `s16`: `M-marks.xml U-taxunits.xml N-0072.xml N-0073.xml N-0074.xml R-0010.xml`.
    - `--same-row` against s1: EXPECT it passes. `--unit-diff` against s2: EXPECT 0 leaves differ. N-0072, N-0073, N-0074: 0 rows each. R-0010 `--diff` against s2: `lists compared` and `legs: 6 -> 6` only.
    - EXPECT every answer's sha256 equal to s2's of the same file (s1's for M). The Gateway's current date 15-Sep-2026.
    - Any not equal: report before the close; the coordinator decides.
17. **Close** by C10. One tally.exe owns 9001; no company loaded; no `bridge_mcp` process this row started.

**Stop**, with no key pressed, if Tally shows a dialog at any point but those the prompt rule, steps 4, 6, 9 and C5/C6 name, or if the gateway stops answering. Screenshot, send the screenshot and the answers to the run coordinator, and ask. Say in the report what the book holds:
- from step 4 to 8: `0073` (and the current date if it was not set back); from step 6, also the inactive row;
- from step 9 to 11: the second registration;
- from step 13: whatever the post did (`calls.py show` of the last post answer, and N-0072 once the exe is closed).

## Record
- Every raw answer's bytes and sha256 and `count19.py`'s lines; every `--diff`, `--unit-diff`, `--same-row`, `--before-write` output whole.
- For each launch: `mkenv.py`'s output, the start command and shell, the process list, `client-log.txt` whole, every answer file in `answers\` whole with its sha256, every `calls.py` output whole, the size of the server's error output, the EXE hash.
- For the box: the helper's command line, `box-*.txt`, `box-text.txt`, the screenshot (by name, size and sha256; screenshots stay on the lab PC), the first-read and click times, the button.
- As text, word for word: every screen keyed in steps 4, 6, 9 and every line read in steps 12 and 15, every key pressed, every prompt and its key, with UTC times.
- BK1 and the three restores as C5 and C6 ask. The release on screen, the Gateway's date and period at steps 1, 4, 8, 11, 16.
- Answers that hold a GSTIN (U, V) stay on the lab PC under the lab's standing rules, listed by size and sha256, as row 2464-GN's did. The synthetic family's GSTINs in this sheet are `08ZZZZZ0000Z1ZQ` and `27ZZZZZ0000Z1ZQ` only.

## Expected
Estimates, not predictions, except where a step says the self-check predicts.
- **Part P:** `post_dated`, with `status_flags` `included`. The build decides `ISPOSTDATED` before the three flags (`status_of_voucher`), and the raw reads gave Yes and included Yes with the current date before, at and after the voucher's date (W21c). Not estimated: whether the build's own status request carries the flag as the read-back did; that is the row's question.
- **R-B:** 1 Aug admitted, 15 Sep `invoice_company_registration_inactive` (predicted by the model in Self-checks, on RU's captured bytes).
- **R-C2:** `invoice_company_registration_ambiguous` on both dates (predicted likewise).
- **Part G:** `posted_verified` and HS's exact deltas: the build's request differs from HS's raw one by the build's own identity fields, not by the legs. The box: as in row 2569-R's Part B on the same build, in front and not minimised. Not estimated: that the lab's own click is taken; it has not answered a post box before.

## Decides
Each is **partial** at best: one voucher, one sitting, one book, one release.
- **Part P as expected:** the build reads a keyed post-dated Sales invoice as `post_dated` through `gst_status`; with S1 (raw), item 1's read is met on 7.2 Silver for one voucher. Not established: a post-dated voucher in a later return period, an uncertain one, another type.
- **Part P reading `included` (or anything but `post_dated`):** the build's status request does not carry what the read-back carried; record which field, and item 1 stays open with a code defect named.
- **R-B as expected:** the build's inactive refusal holds on Tally's own shape of an inactive registration, and the dated rule (the row in force on the invoice date) admits the earlier date. Item 6's "inactive" is then measured through the build.
- **R-B otherwise** (`06` admitted, `05` refused, or another code): the refusal rests on a shape Tally does not write this way; the coordinator decides on a fix row.
- **R-C2 as expected:** the code reading in ADR 0004 is confirmed on Tally's own shape: a two-registration book is refused. Several registrations stay refused, not supported (#2464).
- **R-C2 admitted:** a defect: the build would issue an invoice on a book with two registrations in force. Report at once.
- **Part G as expected:** the GSTR-1 split HS measured raw also holds for the build's own request and post. Item 6's split is then measured through the build, partial.
- **Part G with other deltas:** the build's request differs from the raw one in a way Tally's return sees; the reqdiff and the R-0072 diff say where.
- **The block, as expected:** the build's post stores the six as the keyed invoice holds them (VATDEALERTYPE aside); with row 2162-BC's raw with-and-without compare, the block is measured for both the build and a keyed invoice on this book, partial. A name or place that differs: record which; the coordinator decides whether ADR 0004 item 9 is reopened.
- **Part G box declined or timed out:** the split stays at HS's raw evidence; the lab's own answering of a post box is the finding, for the coordinator.

## Code admits the trigger
Lines at 8c3fb9f2. Every change named is an ancestor of it (`git merge-base --is-ancestor`): #2434 20f6e294, #2475 d80cf1d6, #2577 6fc389f6, #2325 2d25d3d0, #1646 9263657b, #1683 4ebb9ea9, #2307 21b45277.
- Sales is posted by the build: `LIVE_QUALIFIED_VOUCHER_TYPES` holds `Sales` (`src-tauri/src/agent_import.rs:443`).
- An invoice is built only with posting on: `invoice_post_not_enabled` (`agent_import.rs:4118`; its text at `agent.rs:2303`).
- The registration rule: `parse_company_registration` (`agent_import_invoice_wire.rs:1097`): per GST unit, the latest dated row on or before the invoice date; one in force is admitted; none with an inactive row in force is `..._inactive`; more than one is `..._ambiguous` (`:1165` to `:1170`). Its request is `render_company_registration_request` (`:1056`), which `U-taxunits.xml` is.
- `gst_status` decides `ISPOSTDATED` before the three flags (`agent_gst_status.rs:115`, `status_of_voucher`) and reports the flags' own state as `status_flags` (`:410` to `:433`); window at most 93 days, the voucher mark at most 25,000 (`:26`, `:29`).

## Not measured, and left out
- The block with against without through the build: the build always writes the six, so it cannot be varied by the build (row 2162-BC measured it raw). Step 14 compares the build's six with a keyed invoice's only.
- Composition, a changed GSTIN, a type bound to one tax unit, an invoice filed under a second registration: not in this row; see the table at the top.
- A post-dated invoice built by the build (it takes no such argument), a post-dated invoice in a later return period, GSTR-1 for September.
- The OLD build, any other book or release, any other client.

## Close and evidence
- **C10** at step 17. The book stays on the lab PC: it is synthetic.
- **C11:** evidence to a new branch `lab/2162-sales-q-answers`, with `SHA256.txt`, after the pre-push scan and the run coordinator's yes. A RESULTS.md gives each part's result first (P, R-B, R-C2, G), each with its confidence, then every step's EXPECT as met or not, with the build, the book, the date and the record.

Row: lab/2162-sales-q/
