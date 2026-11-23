# Row 2162-SALES-Q: the Sales qualification rows the 8c3fb9f2 build can run on one synthetic book

For #2162 (Sales qualification checklist): item 1, and the item 6 rows the build at master 8c3fb9f2 can exercise. Not for merge. Written 23 Nov 2026, revised the same day after a red-team read (Revisions, at the end). One sitting, one book, one backup, four parts. Confidence of everything here until the row runs: **unverified**.

**How an EXPECT is marked.** Each EXPECT line carries its source: **(code `file:line`)** at 8c3fb9f2, **(lab `<row>`)** for a result an earlier lab row measured, or **(not shown)** when neither shows it, so it is an estimate that this row measures.

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

Every part runs on TallyPrime 7.2 Silver, on the synthetic book below, with the 8c3fb9f2 build already on the lab disk. No part needs a person: the lab keys in Tally, launches the build with `client.py`, takes the backup and the restores, and runs the box helpers. **No hand read and no hand click answers an approval box**: only `boxcheck.py`'s answer file, clicked by `box.ps1` (The approval boxes).

## The question of each part, and its one variable
- **Part P (item 1).** Does the build's `gst_status` read a keyed post-dated Sales invoice as `post_dated`, with its three flags reading included? Variable: one keyed voucher, `BP/26-27/0073`, against the read before it (launch L0 against L1). The current date is moved for the keying and set back before L1.
- **Part R-B (inactive).** With the one registration inactive from 1-Sep-2026, does the build admit an invoice dated 1 Aug 2026 and refuse one dated 15 Sep 2026 as `invoice_company_registration_inactive`? Variable: the registration's status, against L1's builds of the same two calls.
- **Part R-C2 (two registrations).** With a second GST registration in force, does the build refuse as `invoice_company_registration_ambiguous` on both dates? Variable: the second registration, against L0's builds (the book restored to BK1 between).
- **Part G (the split, through the build).** Built and posted by the build, does `BP/26-27/0072` move the HSN summary by exactly what row 2162-HS's raw import of the same invoice moved it by? Variable: the sender (the build's own request and post, instead of a raw import of the build's earlier request), from the same book state as HS (BK1, marks 68/245). Beside it, the build's six block fields are compared with the keyed invoice's of Part P (step 14; more than one variable, so partial).

Parts P and R send no write through the build: `build_import_xml` writes a batch file in the launch's own data folder and sends nothing to Tally (step 3 checks it by the marks). Part G holds the row's one post. Its first box is the **smoke box**, a batch built and never posted, answered No by the helper (step 13.4); no Yes is clicked in a sitting where the helper did not see and close that box.

## Book
`BRIDGE PILOT LAB` (synthetic), GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, loaded alone. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2464-GN's and 2162-HS's) applies in full; where it and this sheet differ, the stricter one holds.

- **Pinned by GUID.** `count19.py`, `calls.py` and `boxcheck.py` hold the GUID; `list_companies` must name it. Any other GUID: STOP, whatever the company is called.
- **Its state.** Every row run on this book ends with a restore that leaves ALTVCHID 68 and ALTMSTID 245, current date 15-Sep-2026, period 1-Apr-26 to 31-Mar-27 (lab `lab/2162-hs-answers` d7a1adf8 s7). The lab's state file for 23 Nov lists other books and not this one. So step 1 is a precondition: this book in Select Company, its GUID, and 68/245. If any differs: STOP before the backup, with nothing keyed or launched.
- **The numbers.** `BP/26-27/0072` (Part G), `0073` (Part P) and `0074` (Part R and the smoke box: built only, never posted).
  - Row 2162-HS created `0072` by a raw import and restored it away (its s7: N-0072 0 rows). Reusing it keeps HS's compare to one variable.
  - A search of every `lab/` branch (750 refs) and of master for `26-27/0073` and `26-27/0074` found none; `0072` is found only in row 2162-HS's files and in two notes that cite HS (row 2162-RP's sheet and the lab queue).
  - Step 2's N reads stay a STOP, in case either check is wrong.
- **BK1 is the way back.** Taken at step 1, before anything is keyed, to `C:\bridge-lab\2162-sales-q\backup-1` (made new and empty at step 1); restored at steps 8, 11 and 16 by C6. After each restore the reads are compared with step 1's and 2's by what is stable (step 8).
- Write down the release shown on Tally's screen.

## Build
Only the NEW build of row 2569-R (`lab/2569-restore-sheet` 208932f7, its Builds section), already on the lab disk and granted:

NEW RC COMMIT: 8c3fb9f2277966fa98b31bdc775b65c5ea3c29f7  ZIP DIGEST: f8dc02e1f7bbb134e5ecc929c1f567aa265df2d3d1c866fea6636270e52e9472  MCPB archive_sha256: 445d45a6fe18035258f69f04c7538241f19c80a39ad9ad88165b8a7c9f15ac8f  EXE SHA256: 1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59

EXE: `C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`. Hash it (Get-FileHash) before every launch. Any other hash: STOP; the exe is never copied, moved or replaced by this row. The OLD build is not used. Nothing is downloaded, installed or built.

**The settings each launch carries: its committed launch file.** One file per launch shape, in this folder's `launch\`, committed with the sheet and checked by `SHA256.txt`: `L0-launch-env.json`, `L1-`, `LB-`, `LC-`, `LG-`. Each holds exactly these five keys, the shape of row 1392-E34's `launch-env.json` (`lab/1392-e34-sheet` 0e2fc769), and no other:
- `BRIDGE_AGENT_ENABLE_WRITES` `true` (posting on);
- `BRIDGE_TERMS_ACCEPTED` `true`;
- `BRIDGE_TALLY_HOST` `127.0.0.1` and `BRIDGE_TALLY_PORT` `9001`;
- `BRIDGE_AGENT_DATA_DIR`, the launch's own new empty folder: `C:\bridge-lab\2162-sales-q\data-0\` (L0), `data-1\` (L1), `data-b\` (LB), `data-c\` (LC), `data-g\` (LG).
- `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_ENABLE_MASTER_WRITES` are absent; `client.py` refuses to start with any of them.

Posting is on in all five, the four that post nothing included: the build refuses an invoice while posting is off (`invoice_post_not_enabled`, `agent_import.rs:4118`; its text at `agent.rs:2303`), and the setting is then never a variable between launches. Each file is copied byte for byte by `Copy-Item`, never retyped or edited, and its copy's sha256 is checked against `SHA256.txt` before the launch (on 19 Nov a shell rewrote `\b` in a typed path). **No script writes a launch file**, on anyone's word.

## Launches
Five launches, each over stdio under `client.py` (byte-equal to rows 2569-R's and 1392-E34's), started the same way for all five from the same shell: `python3 -I C:\bridge-lab\2162-sales-q\client.py <RUNDIR> <EXE> 1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`. Record the command, the shell, and the process list at the start of each.

| Launch | Step | RUNDIR | launch file | Calls, in order |
| --- | --- | --- | --- | --- |
| L0 | 3 | `C:\bridge-lab\2162-sales-q\run-0` | `launch\L0-launch-env.json` | 01 to 07 |
| L1 | 5 | `...\run-1` | `launch\L1-launch-env.json` | 01 to 07 |
| LB | 7 | `...\run-b` | `launch\LB-launch-env.json` | 01, 02, 03, 05, 06, 07 |
| LC | 10 | `...\run-c` | `launch\LC-launch-env.json` | 01, 02, 03, 05, 06, 07 |
| LG | 13 | `...\run-g` | `launch\LG-launch-env.json` | 01, 02, 03, 05 (the smoke batch), the smoke box, 07, g05, g05m, the post, the verify, g09, g10 |

- **Folders and the launch file**, for launch X with RUNDIR `run-x` and data folder `data-x`:
  - `Test-Path C:\bridge-lab\2162-sales-q\run-x, C:\bridge-lab\2162-sales-q\data-x`: EXPECT `False` twice. A `True`: STOP (a launch never reuses a folder).
  - `New-Item -ItemType Directory C:\bridge-lab\2162-sales-q\run-x\inbox, C:\bridge-lab\2162-sales-q\run-x\answers, C:\bridge-lab\2162-sales-q\data-x`.
  - `Copy-Item C:\bridge-lab\2162-sales-q\launch\X-launch-env.json C:\bridge-lab\2162-sales-q\run-x\launch-env.json`, then `Get-FileHash` of the copy: EXPECT the sha256 of `launch/X-launch-env.json` in `SHA256.txt`. Different: STOP, nothing launched.
  - After the launch, EXPECT `client-log.txt` to show the exe sha256 ok, the launch file's sha256 equal to the one just checked, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` with version 0.7.0, within 60 s. A `stop:` line, or no `initialized`: STOP.
- **Calls.** Copy the call files from this folder's `calls\` into the launch's `inbox\` **one at a time**, each only after the previous one's answer is in `answers\` and `client-log.txt` names it. Never edit a call file; never copy one twice into one launch. A call file names the book's GUID; `02` checks it. The posts are not copied by hand: `pg.py` writes them (The approval boxes).
- **Reading an answer.** `python3 -I calls.py show <RUNDIR>\answers\<name>.json` after each. Its STOP (exit 2) means the call failed: no answer, not JSON, a JSON-RPC error, no structured content. That is never read as "nothing found" or "refused". A refusal is an answer: `refused <code>`.
- **Closing a launch.** Create the empty file `<RUNDIR>\inbox\stop`; EXPECT `client-log.txt` to end `session ended; server pid <pid> exit code 0`, and no `bridge_mcp` process left from that pid (lab `lab/1392-e34-answers` c2c3d292, step 7). Never end a process (C2). A server that does not end: STOP and report.
- **No raw request while an exe runs, and no exe while a raw request or a key is in progress.** No other ComplyEaze Bridge makes a call to port 9001 during a launch: record the process list; the desktop app's connectors are left running, as rows 2162-S1 and 2569-R left them, and this is recorded.
- **Bounded logs.** `client.py` writes one line a call; `calls.py` prints a fixed set of fields a call (at most one line an item for `gst_status`, whose page is at most 500 items) and at most 200 lines a request compare; `run.py` keeps an answer above 2,000,000 bytes and stops; `pg.py` prints the box log's stamped lines only, at most 220 characters each. The server's error output goes to one file a launch; record its size, and do not read it unless a step says so.

## The approval boxes (step 13 only)
Four helpers, run as row 1392-E34 ran its own (`python -I -X utf8`; the box's title holds an em dash):
- **`box.ps1`**, byte-equal to row 1392-E34's (`lab/1392-e34-sheet` 0e2fc769, `helpers/box.ps1`). It watches for a visible top-level window titled `ComplyEaze Bridge…` owned by a `bridge_mcp.exe`, logs it (time, rect, topmost bit, z-order against Tally, the foreground window, every child's class and text) and one screen capture, then waits for `box\<tag>.answer`. It clicks the answer's button by mouse at its centre; with no valid answer by `-AnswerBy` seconds it clicks **No**. No key is pressed. Exit 3: no box seen; 4: the box went before a click; 5: no such button. It saw and clicked this build's box five times in row 1392-E34 (lab `lab/1392-e34-answers` c2c3d292: K1 and K3 No, B3, B4 and A4 Yes).
- **`boxcheck.py`**, row 1392-E34's check with an invoice case. It writes **Yes only if every one of these holds** in the box log, and otherwise No and exit 1; any error of its own writes No (exit 2):
  - `BOX SEEN`, the title `ComplyEaze Bridge — post this voucher?` (code `tally/approved_import.rs:3413`);
  - exactly one `Create` line, `Create ONE Sales invoice in "BRIDGE PILOT LAB"`;
  - the line `Company GUID: 6b43e498-430c-4d5c-bfef-d32e2ab93c85`;
  - exactly one `Batch:` line, naming the batch id given (g05's, read from its answer);
  - a voucher count of one, and exactly one build answer in `run-g\answers` naming that batch, `voucher_count` 1, whose saved XML (`path`) has the answer's `sha256`;
  - from that XML: the line `Voucher type: "<VOUCHERTYPENAME>"  Date: <DATE as 1-Aug-2026>  Number: "<VOUCHERNUMBER>"`, with exactly one `Number: "` in the log; `Customer: "<PARTYLEDGERNAME>"`; the box's `Dr`/`Cr` lines equal to the XML's legs, one for one; `Total debit` and `Total credit` equal to the legs' sums and to the build answer's `total_debit` and `total_credit`.
  The text is the format of `agent_import_post.rs:3655` (code), and row 2440-CP's box on this build reads it line for line (lab `lab/2440-composed-answers`, `evidence/s8-window-a.txt`; Self-checks). Its output, `box\<tag>-check.out`, lists each check and never holds the box's company-number line.
- **`selftest-box.py`**: `cases` at step 0, before any box: 18 cases on logs written in `box.ps1`'s form for one fixture invoice (the build's own request of `0010`); the right box gives Yes and each field changed alone gives No, and a log cut inside its WAITING line gives No. These show the check's logic only. `replay` at step 13.4: the check once more on the smoke box's real log, against its real build answer; EXPECT Yes. That is the evidence that the check reads a real invoice box of this build. Both write only under `box\selftest\`, where no `box.ps1` reads an answer.
- **`pg.py`**: one post through LG, adapted from row 1392-E34's `p.py`. It starts `box.ps1`, then (real post only) `boxcheck.py`, writes the `post_import` call into `run-g\inbox` whole, waits for the answer, and on `approval` `pending` calls again after `retry_after_s` while `dialog_remaining_s` lasts, at most four calls (1392-E34's rule). It ends after both helpers end. `smoke` mode: no `boxcheck.py`; `box.ps1` clicks No at 20 s while the call waits; one call.

**The rules of a box.**
1. **Only the helper answers.** The lab presses no key, clicks nothing and types into no window (Tally, the box, a shell) from `pg.py`'s start until it exits. `pg.py` exits only after `box.ps1` has logged `CLICKED` or `NO BOX` and the call has answered.
2. **A Yes needs `boxcheck.py`'s Yes.** Anything else is a No (by `boxcheck.py`, or by `box.ps1` at `-AnswerBy`) and a STOP of Part G: steps 14 and 15 are not run; step 13.9's no-post reads, then step 16 on the coordinator's word (C8).
3. **The smoke box comes first (step 13.4).** STOP before the real post if `box.ps1` did not see the smoke box, did not log `CLICKED No` followed by `box visible False`, the marks after it differ from before it, or the replay is not ALL PASS. No Yes is clicked in a sitting where the helper could not see and close a box that posts nothing (row 1392-E34's step 6.5).
4. **A box left open.** Press no key and touch no window. Wait out Bridge's 120 s box limit from the call's send (in `pg.py`'s output) plus 10 s. Then take the state reads once, as C8 allows, and report.

**What the smoke box leaves, from the code** (partial: these paths were read, not every branch): the approval state is per process and per batch (`agent_import_approval.rs:411`); a kept refusal answers only a call with its own batch (`take_refusal`, `:586` to `:588`); the one shared slot is freed when the box ends (`settle`, `:706`), and a box still open would make the next post refused `post_approval_busy` (`:690`), a loud stop; an invoice batch whose approval was declined was never sent and stops no later Sales post (`agent_import_ledger.rs:1308` to `:1309`). Row 1392-E34 posted B3 in the same launch after its own smoke box (lab `lab/1392-e34-answers` c2c3d292).

## What stays on the lab PC
Rows 1392-E34 and 2569-R kept these local; this row does the same (`lab/1392-e34-answers` c2c3d292, RESULTS deviation 5 and `kept-local.txt`). Each is listed in a `kept-local.txt` by sha256, bytes and name, with why; RESULTS and the notes quote only the lines that grade a step:
- every box log (`box\*-box.txt`, the self-test's included) and every box capture (`box\*.png`): the box's text holds a `Company number:` line;
- `pg.py`'s printouts are on the branch only if they hold no company-number line (it prints none; check before the scan);
- the BK1 file (Tally names it `TDBK1800_<company number>.001`): the notes write its name so, never the number;
- `restore-plan.txt` and the company-folder listings of C6: the branch holds a copy of `restore-plan.txt` with the number replaced by `<company number>`, the original kept local;
- every Bridge answer as saved: each carries `company_number`. The branch holds a copy with that value replaced by `redacted`, as 2569-R and 1392-E34 did; the originals are kept local;
- raw answers that hold a GSTIN (U, V), under the lab's standing rules, as row 2464-GN's did. The synthetic family's GSTINs in this sheet are `08ZZZZZ0000Z1ZQ` and `27ZZZZZ0000Z1ZQ` only;
- screenshots (C11).
On the branch: every `*-check.out`, `*.answer` and `*-ps.out`, `box\selftest\selftest-box-results.txt`, and the replay's check output.

## Files
**Byte-equal to earlier rows' files of the same name** (check: `SHA256.txt`; each was also compared with its source branch's own `SHA256.txt`):
- From row 2464-GN (`lab/2464-gn-sheet` 23b2e57b, folder `lab/2464-gn/`), whose ROW.md gives each one's origin: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count19.py`, `M-marks.xml`, `U-taxunits.xml` (the build's tax-unit request), `R-0010.xml` (the build's invoice read-back, with `ISPOSTDATED`), `template-n-0001.utf16le.xml`, `template-full-0057.utf16le.xml`, `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`; **never sent**: step 13 compares the build's new request with it, and `selftest-box.py` uses it as its fixture invoice).
- From row 2162-HS (`lab/2162-hs-sheet` b6767b87): `N-0072.xml`, `R-0072.xml`. `gen.py` writes both again and asserts each byte-equal to HS's.
- From row 2162-HS's answers (`lab/2162-hs-answers` d7a1adf8, `answers/a-s4r-R-0072.xml`), renamed `a-hss4r-R-0072.xml` so that `count19.py` counts it as R: HS's read-back of its raw `0072`, 94,478 bytes, sha256 `8908800f…`, for step 14's compare.
- From row 2569-R (`lab/2569-restore-sheet` 208932f7): `client.py` (sha256 `8d3729ea…`, also row 1392-E34's).
- From row 1392-E34 (`lab/1392-e34-sheet` 0e2fc769): `box.ps1` (`550fc11f…`).

**Written for this row:**
- `gen.py`: from the templates above, `N-0072.xml`, `N-0073.xml`, `N-0074.xml`, `R-0072.xml`, `R-0073.xml`, `V-0072.xml`, `V-0073.xml`, each with only the voucher number changed, asserted once each; each template's sha256 is checked first. No import is written: this row sends no raw write.
- `calls.py`: reads `client.py`'s answers by parsing JSON (show, status-diff, batch, reqdiff) and writes the verify call (verify), whole and then renamed in. Its exit 2 is "the call failed"; its `batch` FINDING (exit 1) is a result.
- `boxcheck.py` (from row 1392-E34's, changed as The approval boxes says), `selftest-box.py`, `pg.py` (from row 1392-E34's `p.py`).
- `launch\*-launch-env.json`: the five launch files (Build).
- `calls\*.json`: the fixed calls. `01` tools/list; `02` list_companies; `03`, `07`, `g05m`, `g10` ledger_masters (the marks: `snapshot.master_alter_id` M, `snapshot.voucher_alter_id` V, `snapshot.reused` false, as row 2440-CP defines MARKS); `04`, `g09` gst_status of `BRIDGE Sales` for 2026-08-01 to 2026-08-31; `05`, `06` build_import_xml of `BP/26-27/0074` dated 2026-08-01 and 2026-09-15, with their own `bridge_txn_id` (`sq-r1`, `sq-r2`; a build is refused for a row only once that row was sent, `agent_import.rs:1666` to `:1690`, and these are never sent; `05` is also LG's smoke batch); `g05` build_import_xml of `BP/26-27/0072` dated 2026-08-01. The invoice is `pilot_invoice` of `agent_import_invoice_admission_tests.rs` at 8c3fb9f2 (the build's own test of this book), with the number and date named: `BRIDGE Walk-in` Dr 1,051.00; `BRIDGE Svc 998313 5%` 600.40; `BRIDGE Svc 998314 5%` 400.40; `BRIDGE CGST 2.5%` 25.02; `BRIDGE SGST 2.5%` 25.02; `BRIDGE Round Off` 0.16; type `BRIDGE Sales`, place of supply Rajasthan. It is row 2162-HS's `0072` invoice, leg for leg.

## Self-checks run on macOS before commit (23 Nov 2026)
- Every reused file's sha256 equals its source's `SHA256.txt` line (`box.ps1` against 1392-E34's).
- `python3 -I gen.py`: seven files written; `N-0072.xml` and `R-0072.xml` byte-equal to HS's (asserted). `python3 -m py_compile` on `calls.py`, `gen.py`, `client.py`, `run.py`, `count19.py`, `boxcheck.py`, `selftest-box.py`, `pg.py`: no error.
- `count19.py` on captured answers, each exit 0: M (HS s1), N (HS s2 with 0 rows, s4r with 1), R (HS s4r R-0072), U (RU s2, sB, sC2). `--same-row` HS s1 against s7: equal; against s4r: STOP (exit 1), as it must. `--before-write` HS s1 against s4r: passes. `--diff` HS s2 R-0010 against s7: `lists compared: 1`, `legs: 6 -> 6`. `--unit-diff` RU s2 against sB: `Rajasthan Registration` 9 leaves differ (ALTERID 206 to 246, and the eight leaves of a new dated row: FROMDATE 20260901, ISINACTIVE Yes, STATE and PLACEOFSUPPLY Rajasthan, REGISTRATIONTYPE Regular, GSTR1PERIODICITY Monthly, ISSTATECESSON No, ISOTHTERRITORYASSESSEE empty); RU s2 against sBr: 0.
- `calls.py show` on row 2569-R's 15 Part B answers (the same build, `lab/2569-restore-answers` 400575f3): every field printed, including `pending` with `dialog_remaining_s` 96 and `retry_after_s` 15 on the first post call, and `posted_verified` on the second; on row 2440-CP's timed-out post: `refused import_approval_timed_out`; on a missing file: STOP, exit 2. `status-diff` on two `gst_status` answers of `lab/1633-export-ready-answers`: runs, items matched by master id. `verify` writes a call (through a temporary file, renamed in) and refuses to overwrite one, and refuses a refused build (exit 2); `batch` on a missing file and on a wrong-hash file: FINDING, exit 1. `reqdiff` of a file with itself: 0 lines.
- `selftest-box.py cases` on a scratch copy of this folder: the 18 cases right, `ALL CASES RIGHT`, exit 0 (t19 takes 30 s). `t01-check.out` holds no `Company number` line.
- **The fixture's text against a real box.** Row 2440-CP's captured invoice box (`lab/2440-composed-answers`, `evidence/s8-window-a.txt`) and the fixture's text, from `Create ONE` to `Narration`, with quoted names, numbers and GUIDs masked and runs of entry lines folded: the same lines in the same order, except two whose values differ by design (the date's month; the customer's registration line, which the check does not grade).
- `launch\*-launch-env.json`: each parses as JSON, holds the five keys and no other, its data folder its own; none holds a key `client.py` refuses.
- A model of `parse_company_registration` (8c3fb9f2, written in Python for this check only, not shipped) on RU's captured tax-unit answers: s2 one unit in force on both dates; sB (inactive from 20260901) one in force on 20260801 and inactive on 20260915; sC2 (two units) ambiguous on both. These are the EXPECT lines of steps 7 and 10. They predict; they are not evidence.
- Not run on macOS: `client.py` against a server, `box.ps1` and `pg.py` (Windows only), any Tally request.

## Steps
Raw reads: `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, then `python3 -I count19.py a-<label>-*`. A STOP from either stops the row. Check each request file's sha256 against `SHA256.txt` before its send. Copy each script's whole output into the notes (the kept-local rule aside).

**The prompt rule, at every step (C7, C8).** The only prompts answered are C5's, C6's and the ones steps 4, 6 and 9 name with their keys. Any other prompt, message or dialog in Tally, at any step: screenshot it, copy its text word for word, **press no key**, send nothing, STOP and report at once. It stays on the screen until the run coordinator answers. **Nothing is restored without the run coordinator's word** (C8). Before BK1 exists (steps 0 and 1) the same: STOP.

**The wait rule.** After a save, a Ctrl+A or a send: if 5 minutes pass with no screen change, no answer and no message, press nothing, send nothing, screenshot, report. With the Gateway showing and no dialog, run C2's status check before the next read.

**The path rule.** Every keyed screen follows the path written at its step. If a screen, a line or a field is not exactly as named: screenshot, press nothing more, STOP and report (C8). A list that the step names but that does not show is handled as the step says; no step is guessed.

**The screen rules.** Never Escape on a master or voucher screen (C7). **Never Escape at Select Company or at the Gateway**: one Escape at Select Company raised Tally's `Quit ?` prompt (lab `lab/1392-e34-answers` c2c3d292, deviation 1). A screen is left by the keys its step names, Ctrl+Q only after a screenshot that shows the screen's title. If `Quit ?` shows at any point: the prompt rule (no key, STOP).

0. **Before the sitting.**
   - This folder's files copied from the branch to `C:\bridge-lab\2162-sales-q\` byte for byte; check every file against `SHA256.txt` (Get-FileHash). Any differs: STOP.
   - `python3 -I gen.py`: EXPECT the seven lines and each sha256 equal to `SHA256.txt`'s.
   - `[System.Management.Automation.Language.Parser]::ParseFile` on `box.ps1`: 0 errors.
   - `python -I -X utf8 selftest-box.py cases C:/bridge-lab/2162-sales-q`, its output to `evidence\s0-selftest-box.txt`. EXPECT 18 lines ending `OK`, then `ALL CASES RIGHT`, exit 0 (as on macOS, Self-checks). Anything else: STOP before step 1.
   - Hash the EXE against the build line.
   - C2 step 0, with no company loaded.
1. **Load, check and back up.**
   - Load BRIDGE PILOT LAB alone from Select Company (C1): type `BRIDGE PILOT` until it is the one line highlighted (zoom), Enter (not shown: C1 names no keys). Not in the list, or any prompt: STOP, press nothing (no Escape here).
   - Write down the Gateway's current date and period. EXPECT 15-Sep-2026 and 1-Apr-26 to 31-Mar-27 (lab 2162-HS).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245 (lab 2162-HS s7). Anything else: STOP here, before BK1, with nothing keyed or launched.
   - `Test-Path C:\bridge-lab\2162-sales-q\backup-1`: EXPECT `False`; then `New-Item -ItemType Directory C:\bridge-lab\2162-sales-q\backup-1` and a listing of it (empty), as C5 asks. A `True`: STOP.
   - **BK1** by C5, to that folder (Tally may show it with its own capitals). Record the file's name (as `TDBK1800_<company number>.001`), size, sha256 and the time the Restore list will show.
2. **The book before.** `s2`: `U-taxunits.xml N-0072.xml N-0073.xml N-0074.xml R-0010.xml M-marks.xml`.
   - U: EXPECT two TAXUNIT rows, `Default Tax Unit` and `Rajasthan Registration`; the latter with GSTREGNUMBER `08ZZZZZ0000Z1ZQ` and one dated row: FROMDATE 20260401, STATE Rajasthan, REGISTRATIONTYPE Regular, ISINACTIVE No (lab 2162-RU s2).
   - N-0072, N-0073, N-0074: EXPECT 0 rows each (lab 2162-HS s7 for 0072; not shown for 0073 and 0074, a branch search only). One row or more: STOP before step 3, with each row recorded whole.
   - R-0010: EXPECT one row, `BRIDGE Walk-in`, included Yes, every other status flag No, `ISPOSTDATED` No, 6 legs (lab 2162-S1 R-0010 control; 2162-HS s7).
   - M: equal to s1 (`count19.py --same-row a-s1-M-marks.xml a-s2-M-marks.xml`).
   - Record each answer's sha256 (recorded only; no later step grades a sha256).
3. **L0: the build on the book before** (Parts P's and R's control).
   - Folders and launch file `L0` (Launches); hash the EXE; start `client.py` with RUNDIR `run-0`.
   - Calls `01` to `07`, each read with `calls.py show`:
     - `01`: EXPECT none of this row's six tools missing (code `agent_catalog.rs:457`: `gst_status` is registered and not filtered by any setting; `build_import_xml`, `post_import` and `verify_import` are listed with posting on, lab 2569-R). `gst_status` alone missing: Part P's reads (`04` here and at step 5) are skipped and recorded; steps 4 to 17 run without them. Any other missing: close the launch, STOP (posting off, or not this build).
     - `02`: EXPECT one company, `BRIDGE PILOT LAB`, `is the book's: yes` (lab 2162-HS: the book loaded alone). Otherwise close the launch, STOP before any other call.
     - `03`: EXPECT M 245, V 68, reused False (lab 2162-HS s1, as raw marks).
     - `04`: EXPECT not refused; state `complete`; no `next_offset`; `BP/26-27/0010` among the items, date `20260801`, `included` (not shown through `gst_status` on this book; R-0010 reads included, lab 2162-S1). `post_dated` 0 in the counts (not shown; W21c's post-dated `0045` was restored away, lab 1342-W21c). Record every item line whole: this is Part P's before. `next_offset` present: record, and Part P's compare in step 5 is then not run (one page only).
     - `05` (dated 2026-08-01) and `06` (2026-09-15): EXPECT each not refused, `voucher_count 1` (not shown; the model of Self-checks predicts one registration in force on both dates). Record the batch id, sha256 and the warnings whole. Then `python3 -I calls.py batch <answer> C:\bridge-lab\2162-sales-q\data-0` for each; record its line (a FINDING is recorded, the row goes on). **A refusal here is a result** (the build refuses this book as it stands): record it whole; then steps 7 and 10 cannot show a change for that date, and they run only for a date admitted here. A refusal of `05` also means step 13 cannot build its smoke batch: Part G then STOPs at step 13.3.
     - `07`: EXPECT M 245, V 68: nothing was written to Tally (code: a build sends no write, `build_import_xml` writes only the batch file).
   - Close the launch (Launches). Then `s3`: `M-marks.xml`; `count19.py --same-row a-s1-M-marks.xml a-s3-M-marks.xml`: EXPECT it passes. A difference: STOP, and report that a build launch moved a mark.
4. **Part P: key one post-dated invoice.** No lab row recorded this voucher's keys one by one. What is recorded: row 2162-S1 (`lab/2162-s1-answers` 20ed166c, step 3) and row 1342-W21c (`lab/1342-w21c-answers` 19d50259, NOTES) keyed this same shape on this book and say what each screen did; row 1633-ER (`lab/1633-export-ready-answers` 62c29955, notes steps 4 and 7) keyed a Sales invoice on another book through Vouchers, F8, the two sub-screens by Enter, Ctrl+A, and one Ctrl+Q. Each key below names its source; a key marked **(not shown)** is gated by a zoom before the next key, and a screen that differs is the path rule (STOP, no restore).
   - `s4m`: `M-marks.xml`; `count19.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml`: it must pass. Then no key is pressed in Tally but those below.
   1. **The current date.** At the Gateway (shot): F2 (S1, W21c). EXPECT a date field over the Gateway (zoom; not shown how it looks). Type `10-8-2026`, Enter. EXPECT the Gateway to show 10-Aug-2026 (shot) (S1). Anything else: STOP.
   2. **The voucher screen.** At the Gateway: Down until `Vouchers` is highlighted (zoom after each Down; the Gateway's line order is not shown by a row: RU's path shows `Create` first and `Alter` one Down below it). Enter. EXPECT a voucher creation screen (shot; 1633-ER: Vouchers opened the type last used). Then F8 (1633-ER: F8 opened Sales). EXPECT the title `Accounting Voucher Creation` and the type `BRIDGE Sales` (zoom).
      - If a list of voucher types shows instead (not shown on this book, which holds `BRIDGE Sales`, `BRIDGE Sales Two` and `BRIDGE Sales Z1`, lab 2162-HS s3): type `BRIDGE Sales` until it is the one line highlighted (zoom), Enter; EXPECT the title and type above.
      - Neither: screenshot, press nothing more, STOP. Nothing is keyed yet.
   3. **The voucher's date.** F2 on the voucher screen (not shown: S1 and W21c say only that the voucher was dated 25-Aug-2026). EXPECT the voucher's date field open (zoom); type `25-8-2026`, Enter. EXPECT the voucher's date 25-Aug-26 (zoom).
   4. **Post-dated.** Ctrl+T (S1: "marked Post-Dated with Ctrl+T before saving. Status read Post-Dated"). EXPECT the screen to read Post-Dated (zoom).
   5. **The fields, in the order Tally offers them.** For each field, the key by the field's name on screen; zoom on the field before its key. A field not in this list: the path rule.
      - `No.`: type `BP/26-27/0073`, Enter (S1: the number field stays empty and does not take Enter; this type is numbered by hand on this book). EXPECT it to read `BP/26-27/0073` (zoom).
      - `Reference No.` or its date, if offered: Enter, nothing typed (not shown).
      - `Party A/c name`: type `BRIDGE Walk-in` (S1, 1633-ER: the party chosen from its list). EXPECT the highlighted line to read `BRIDGE Walk-in` in capitals (zoom), then Enter. The book also holds a party shown as `Bridge Walk-in` (lab 2162-HS s3): if both lines show, Down or Up until the capitals line is highlighted (zoom), then Enter (not shown). A "Too many choices" box: the prompt rule. R-0073 (below) shows which party was taken.
      - `Dispatch Details`, when it opens: Enter on each field with nothing typed until it closes (S1: accepted as offered; 1633-ER: closed with Enter only).
      - `Party Details` (the buyer box), when it opens: Enter on each field with nothing typed until it closes (S1; 1633-ER: passed by Enter only with every value as offered). Write down every value it shows.
      - Particulars, first line: type `BRIDGE Svc 998313` until `BRIDGE Svc 998313 5%` is the one line, Enter; a rate field: Enter, nothing typed (W21c: the rate left blank); the amount: type `1000`, Enter (S1, W21c).
      - Particulars, next line: type `BRIDGE CGST 2.5` until `BRIDGE CGST 2.5%` is the one line, Enter. **Never Enter while two lines match** (W21c: `BRIDGE CGST` matches two and raised "Too many choices"; that box is the prompt rule). EXPECT its amount 25.00, as Tally computed it (S1); Enter on each of its fields, nothing typed.
      - Particulars, next line: `BRIDGE SGST 2.5` the same way; EXPECT 25.00 (S1).
      - Particulars, the empty line after: Enter, nothing typed (not shown).
      - `Narration`: nothing typed (not shown).
   6. **The save.** EXPECT the total 1,050.00 (S1) and Post-Dated still shown; a shot of the whole voucher. Ctrl+A (1633-ER; S1: saved with no prompt). EXPECT a new blank voucher screen of the same type (1633-ER) (shot of its title), then Ctrl+Q once. EXPECT the Gateway with no prompt (1633-ER). If the Gateway shows at once after Ctrl+A instead (W21c names the Gateway after the save): no Ctrl+Q (never at the Gateway, C7); go on at 4.7. Any prompt: the prompt rule.
   7. **The current date back.** EXPECT the Gateway; W21c found its current date then read 25-Aug-2026. Write down what it shows. F2: type `15-9-2026`, Enter. EXPECT 15-Sep-2026 (shot). If it cannot be set back: STOP and report (C8); the restore at step 8 sets it back (S1), but only on the coordinator's word.
   - `s4`: `N-0073.xml R-0073.xml V-0073.xml R-0010.xml M-marks.xml`.
     - N-0073: EXPECT one row, with a VOUCHERNUMBER (lab 2162-S1: the number as typed). Zero rows: the number was not saved as typed; STOP and report (C8).
     - R-0073: EXPECT one row, DATE 20260825, PARTYLEDGERNAME `BRIDGE Walk-in` (14 characters, no line break), `ISPOSTDATED` Yes, included Yes, uncertain No, excluded No, `ISCANCELLED` and `ISOPTIONAL` No (lab 2162-S1, R-0045). Record it whole.
     - V-0073: record it whole (the keyed invoice's native fields).
     - R-0010: `count19.py --diff a-s2-R-0010.xml a-s4-R-0010.xml`: EXPECT `lists compared: 1` and `legs: 6 -> 6` only (lab 2162-HS s4r against s2).
     - M: EXPECT V 69, M 245 (lab 2162-S1 and 1342-W21c: 69/245 after this voucher).
     - R-0073 not reading `ISPOSTDATED` Yes: record it whole; Part P's result is then raw already, and step 5 still runs and records what the build reads.
5. **L1: the build after the keyed invoice.**
   - Folders and launch file `L1`; hash; launch with RUNDIR `run-1`. Calls `01` to `07`, as step 3.
     - `03` and `07`: EXPECT M 245, V 69 (step 4's s4).
     - `04`: `python3 -I calls.py status-diff C:\bridge-lab\2162-sales-q\run-0\answers\04-status-aug.json C:\bridge-lab\2162-sales-q\run-1\answers\04-status-aug.json`. EXPECT:
       - exactly one item line `in B only`, for `BP/26-27/0073`: date `20260825`, `gst_status` `post_dated`, `status_flags` `included`, `accepted_as_it_stands` false (code `agent_gst_status.rs:115` to `:127`: `ISPOSTDATED` Yes with the other two No gives `post_dated`; `:432` to `:433`: the three flags' own state as `status_flags`; not shown through the build);
       - no item in A only, and no item whose fields differ;
       - in the counts: `post_dated` one more than L0's (0 to 1) and the total one more; every other count equal to L0's (not shown).
       Record the whole output.
     - `05` and `06`: EXPECT as at L0 (not refused, `voucher_count 1`) (lab: this row's step 3). These are the control for step 7.
   - Close the launch. `s5`: `M-marks.xml`; EXPECT V 69, M 245 (`count19.py --same-row a-s4-M-marks.xml a-s5-M-marks.xml`).
6. **Part R-B: the registration made inactive from 1-Sep-2026** (row 2162-RU's Part B, `lab/2162-ru-answers` c7b14084, `evidence/sB-screens.txt` 4.1 to 4.7, keyed there from 68/245; here the book also holds `0073`, which no registration field names).
   - `s6m`: `M-marks.xml`; `count19.py --before-write a-s1-M-marks.xml a-s6m-M-marks.xml`: it must pass.
   1. At the Gateway (shot): Down once, `Alter` highlighted (zoom); Enter. `Master Alteration` (shot); Down 11 to `GST Registration` (zoom); Enter (RU 4.1).
      - EXPECT `List of GST Registrations`: Create, Back, one line `Rajasthan Registration`, highlighted (zoom) (RU 4.1). Enter on it.
      - **If no list shows** and the screen is `GST Registration Alteration` naming `Rajasthan Registration` (zoom): press nothing for the list; go on at 6.2.
      - Any other screen: the path rule.
   2. `GST Registration Alteration` (shot). EXPECT RU's 4.1: Registration status Active; State Rajasthan; Registration type Regular; Assessee of Other Territory No; GSTIN/UIN 08ZZZZZ0000Z1ZQ; Periodicity of GSTR-1 Monthly; the rest as RU recorded. Write down every field.
   3. Shift+Tab twice: `Registration status: Active` with its list (Active highlighted, Inactive). Down once: `Inactive` highlighted (zoom); Enter (RU 4.2, 4.3).
   4. **Named prompt 1, the "Effective Date" box**, one field "Effective Date for revised GST details" with a "List of Effective Dates". Write down every line of the list (RU's held 15-Sep-26 twice and two computer dates; this book's "Date of Last Entry" may differ). Type `1-9-2026`. EXPECT the field to read 1-9-2026 and the list one line, `1-9-2026  New Effective Date`. Enter (RU 4.3, 4.4).
   5. **Named prompt 2, the "New Effective Date" box**, one field reading `1-Sep-26` (zoom). If exactly so: Enter once. Any other date or field: the prompt rule (RU 4.4, 4.5).
   6. **Named prompt 3**, the form's own `Accept ?`, with Registration status Inactive and every other field as at 6.2 (shot). Press `y` (RU 4.5, 4.6).
   7. EXPECT `List of GST Registrations` again (shot showing its title); Ctrl+Q once. EXPECT the Gateway, no question (shot) (RU 4.6, 4.7). If 6.1 showed no list and the form closed to `Master Alteration` instead: shot showing its title, Ctrl+Q once, EXPECT the Gateway.
   - `s6`: `U-taxunits.xml R-0010.xml M-marks.xml`.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s6-U-taxunits.xml`. EXPECT the same two names on both sides; `Default Tax Unit` 0 leaves differ; `Rajasthan Registration` 9 leaves differ, exactly RU's s2-to-sB nine (lab 2162-RU; Self-checks): ALTERID to 246, and a new dated row FROMDATE 20260901, ISINACTIVE Yes, STATE Rajasthan, PLACEOFSUPPLY Rajasthan, REGISTRATIONTYPE Regular, GSTR1PERIODICITY Monthly, ISSTATECESSON No, ISOTHTERRITORYASSESSEE empty. Record the whole output.
     - R-0010: `--diff` against s2: EXPECT `lists compared: 1` and `legs: 6 -> 6` only (lab 2162-HS).
     - M: EXPECT V 69, M 246 (lab 2162-RU: the unit's ALTERID 246 from 245).
     - A different unit diff: record it whole; step 7 still runs (its result is then read against the shape recorded here, and RU's shape is not reproduced; say so).
7. **LB: the build with the registration inactive.**
   - Folders and launch file `LB`; hash; launch with RUNDIR `run-b`. Calls `01`, `02`, `03`, `05`, `06`, `07`.
     - `03`, `07`: EXPECT M 246, V 69 (step 6's s6).
     - `05` (1 Aug): EXPECT not refused, `voucher_count 1`: the dated row before 1 Sep is in force, Active (code `agent_import_invoice_wire.rs:1097` to `:1170`, the latest dated row on or before the invoice date; the Self-checks model on RU's sB bytes predicts it; not shown through the build).
     - `06` (15 Sep): EXPECT `refused invoice_company_registration_inactive` (code `agent_import_invoice_wire.rs:1168`; partial: it holds if no earlier refusal of the build fires first, which is not shown).
     - Each other answer is a result: record it whole. A refusal at `05`, or `06` admitted, is this row's finding for R-B.
   - Close the launch.
8. **Restore BK1 (R1)** by C6, `restore-plan.txt` written first (kept local; the branch's copy names `<company number>`). Then `s8`: `M-marks.xml U-taxunits.xml N-0073.xml R-0010.xml`. **The restore is graded by what is stable**, never by an answer's sha256 (the CMPINFO counters differ per load, `count19.py`'s `--same-row` note):
   - the marks row: `count19.py --same-row a-s1-M-marks.xml a-s8-M-marks.xml`: EXPECT it passes, GUID, ALTVCHID 68 and ALTMSTID 245 (lab 2162-HS s7 and 1392-E34 m4: the marks row equal after a C6 restore);
   - the registrations: `count19.py --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml`: EXPECT 0 leaves differ in each unit (lab 2162-RU: s2 against sBr, 0);
   - the vouchers the parts read: N-0073 EXPECT 0 rows; R-0010 `--diff` against s2: EXPECT `lists compared: 1` and `legs: 6 -> 6` only (lab 2162-HS s7);
   - the Gateway's current date: EXPECT 15-Sep-2026 (lab 2162-S1: after the reload the Gateway read 15-Sep-2026).
   - Record each answer's sha256, not graded.
   - Any of these not as EXPECTed: STOP; send nothing more and report before anything else. The book is not as it was and the coordinator decides (C6, C8).
9. **Part R-C2: a second GST registration** (row 2464-GN's step 3, `lab/2464-gn-sheet` 23b2e57b, which is row 2162-RU's Part C2; `lab/2464-gr-b-answers` 01899d61, RESULTS "Step 3", ran these keys and recorded each screen and prompt).
   - `s9m`: `M-marks.xml`; `count19.py --before-write a-s1-M-marks.xml a-s9m-M-marks.xml`: it must pass.
   1. At the Gateway (shot): `Create` highlighted (zoom), Enter. `Master Creation`; Down 9 to Godown, once to the heading, once more: `GST Registration` highlighted (zoom). Enter.
   2. **Named prompt 1.** EXPECT, word for word, "GST Registration already exists for the Company." then "Do you want to alter the existing GST Registration or create a new GST Registration?", with `C Create New` and `A Alter Existing` (zoom). If the words are the same: `c`. **If no such prompt shows** and the screen is `GST Registration Creation` (zoom): press nothing for it; go on at 9.3. Any other screen: the path rule.
   3. `GST Registration Creation` (shot). Write down every field. EXPECT Registration status Active, State empty, Registration type Regular, Assessee of Other Territory No, GSTIN/UIN empty, Periodicity of GSTR-1 Monthly.
   4. State: type `Maha`; EXPECT `Maharashtra` the one line; Enter. Registration type: Down once, `Regular` highlighted (zoom); Enter. Assessee of Other Territory: No, Enter. GSTIN/UIN: type `27ZZZZZ000`, then `0Z1ZQ`, zooming after each; before any Enter it must read `27ZZZZZ0000Z1ZQ`, 15 capitals (shot). Then Enter on each later field, leaving it as Tally filled it.
   5. **Named prompt 2.** `Accept ?` (shot): `y`.
   6. **Named prompt 3.** EXPECT the three paragraphs 2464-GN quotes, word for word ("You can create a unique voucher numbering series to cater to multiple GST registrations." / "You can do this by going to the Chart of Accounts and selecting the relevant voucher types" (no full stop) / "Open Chart of Accounts and create now?"), with Yes or No (zoom). If the same: `n`. If it does not show: press nothing for it; go on at 9.7.
   7. EXPECT a new, empty `GST Registration Creation` form (shot showing its title). Type nothing. Ctrl+Q. EXPECT `Master Creation` (shot showing its title); Ctrl+Q. EXPECT the Gateway (shot).
   - `s9`: `U-taxunits.xml M-marks.xml`.
     - `count19.py --unit-diff a-s2-U-taxunits.xml a-s9-U-taxunits.xml`. EXPECT the names `Default Tax Unit`, `Maharashtra Registration` and `Rajasthan Registration` after; `Maharashtra Registration` in the second only, with GSTREGNUMBER `27ZZZZZ0000Z1ZQ` and one dated row FROMDATE 20260401, STATE Maharashtra, REGISTRATIONTYPE Regular, ISINACTIVE No (lab 2162-RU sC2); the two others 0 leaves differ. Record the whole output.
     - M: EXPECT V 68, M 246 (lab 2464-GR-b: 246).
10. **LC: the build with two registrations in force.**
    - Folders and launch file `LC`; hash; launch with RUNDIR `run-c`. Calls `01`, `02`, `03`, `05`, `06`, `07`.
      - `03`, `07`: EXPECT M 246, V 68 (step 9's s9).
      - `05` and `06`: EXPECT each `refused invoice_company_registration_ambiguous` (code `agent_import_invoice_wire.rs:1170`; the Self-checks model on RU's sC2 bytes predicts it; partial: it holds if no earlier refusal fires first, which is not shown).
      - Any other answer is the finding: record it whole.
    - Close the launch.
11. **Restore BK1 (R2)** by C6. Then `s11`: `M-marks.xml U-taxunits.xml N-0074.xml R-0010.xml`, graded as step 8 (N-0074: 0 rows). Any not as EXPECTed: STOP and report (C8).
12. **Part G: the HSN summary before** (row 2162-HS's step 3, on screen only, nothing saved; its keys are recorded in `lab/2162-hs-answers` d7a1adf8, `evidence/s3-screens.txt` 3.1 to 3.6). Fourteen keys in all, HS's: the label click; `GSTR-1` typed; Down; Enter; Down 24; Enter; Down 2; Enter; Esc; Down; Enter; Esc; Esc; Esc.
    1. At the Gateway (shot): click the `G: Go To` label; type `GSTR-1`; EXPECT the List of Reports, no line highlighted; Down once: `GSTR-1` highlighted (zoom); Enter (HS 3.1).
       - **If no list of GST registrations shows** (HS: none came): press nothing for it; go to 12.2.
       - If a list of registrations shows (not shown): it must hold one line, `08ZZZZZ0000Z1ZQ` (zoom); Enter on it. More than one line: the path rule.
    2. EXPECT `GSTR-1`, the period 1-Aug-26 to 31-Aug-26, with no period key pressed (lab HS 3.2). Any other: write it down; the path rule (STOP; the restore at step 16 waits for the coordinator's word).
    3. Write down the counts. EXPECT HS's before: Total Vouchers 39, Included in Return 27, Not Relevant 7, Uncertain 5; B2C (Small) 27 (2) (lab HS 3.3).
    4. From Total Vouchers, Down 10, 12, 1, 1 (24 in all): `HSN Summary - 12 (B2B - B2C Supplies)` highlighted (zoom); Enter (HS 3.4). One list, headed B2C Supplies. Write down every line: HSN/SAC, UQC, total, rate, taxable value, each tax and the tax amount, then Sub Total and Total. EXPECT HS's nine lines before, among them 998313 at 11,137.40 / 10,607.00 / 265.21 / 265.19 / 530.40, 998314 at 1,366.70 / 1,301.60 / 32.55 / 32.55 / 65.10, and Total 14,275.20 / 13,508.70 / 383.26 / 383.24 / 766.50 (lab HS 3.4).
    5. Down 2 from 847130 to `998313` (zoom); Enter once: "GSTR-1 - Voucher Register", "Vouchers of 998313". Write down every line. EXPECT HS's 19 lines, `BP/26-27/0010` at 630.42 / 600.40 / 15.01 / 15.01 (lab HS 3.5). No key but Esc on it: Esc once. EXPECT the HSN/SAC Summary, highlight on 998313 (HS). Down once: `998314` highlighted (zoom); Enter once. EXPECT HS's 7 lines (lab HS 3.5). Esc once. EXPECT the HSN/SAC Summary, highlight on 998314 (HS 3.6).
    6. **Exactly two Escapes more**, a screenshot after each (HS 3.6): Esc once, EXPECT `GSTR-1` with its counts as at 12.3; Esc once, EXPECT the Gateway of Tally, BRIDGE PILOT LAB (shot). No further Escape: the Gateway is never escaped (the screen rules). A screen other than the one named after an Esc: the path rule.
    - A count or line that differs from HS's: record it whole and go on. Step 15's compare is against this step's reading, not HS's.
13. **LG: the smoke box, then the build builds and posts `BP/26-27/0072`.** Nothing is keyed in Tally in this step.
    1. Folders and launch file `LG`; hash; launch with RUNDIR `run-g`. Calls `01`, `02`, `03` as step 3 (EXPECT M 245, V 68: step 11's s11).
    2. **The smoke batch.** Copy `05` (0074, 1 Aug, never posted) into `run-g\inbox`. EXPECT not refused, `voucher_count 1` (lab: this row's steps 3 and 11). `python3 -I calls.py batch C:\bridge-lab\2162-sales-q\run-g\answers\05-build-0074-0801.json C:\bridge-lab\2162-sales-q\data-g`; record its line.
    3. A refusal at 13.2: record it whole, close the launch, STOP before any box (no smoke box, so no Yes this sitting).
    4. **The smoke box.** `python -I -X utf8 pg.py smoke k 05-build-0074-0801.json`, its output to `evidence\s13-smoke.txt`. No key, click or typing until it exits (The approval boxes, rule 1). EXPECT (lab `lab/1392-e34-answers` c2c3d292, K3, for a Receipt; not shown for an invoice):
       - `BOX SEEN`, title `post this voucher?`; `CLICKED No` about 20 s later, then `after click: box visible False`; `box.ps1` exit 0;
       - the call answering `refused import_approval_declined`, `attempt_recorded` false;
       - `SMOKE declined, CLICKED No, box gone`, exit 0.
       Then `python -I -X utf8 selftest-box.py replay C:/bridge-lab/2162-sales-q k <the smoke batch id>`, its output to `evidence\s13-replay.txt`. EXPECT every check PASS on the real box (the `Batch:` line naming the smoke batch, `Number: "BP/26-27/0074"`, the six lines and totals), then `REPLAY ALL PASS`, exit 0 (not shown: no invoice box was checked by a script before).
       Then copy `07`: EXPECT M 245, V 68, unchanged (code: a declined approval sends nothing, `agent_import_ledger.rs:1308`).
       **STOP before 13.5** if any of these holds (The approval boxes, rule 3): no `BOX SEEN`; no `CLICKED No` followed by `box visible False`; the replay not ALL PASS; `07` not M 245, V 68. A box left unclicked: rule 4.
       Wait until at least 130 s have passed since the smoke call's send (`pg.py`'s `sent` line), so that its box is surely over (Bridge's box limit is 120 s; row 1392-E34's rule before its B3).
    5. `g05`: EXPECT not refused, `voucher_count 1` (lab: this row's step 3, `05`, the same legs). Record the answer whole. A refusal: record it whole, close the launch, go to step 16 on the coordinator's word.
    6. `python3 -I calls.py batch C:\bridge-lab\2162-sales-q\run-g\answers\g05-build-0072.json C:\bridge-lab\2162-sales-q\data-g`; record its line. If found: `python3 -I calls.py reqdiff template-import-0010.utf16le.xml <the file it names>`; record the whole output (the build's request of 0010 at an earlier master against its request of 0072 at 8c3fb9f2; recorded, not graded).
    7. `g05m`: EXPECT M 245, V 68, immediately before the post.
    8. **The post.** `python -I -X utf8 pg.py post g06 g05-build-0072.json`, its output to `evidence\s13-post.txt`. No key, click or typing until it exits. EXPECT:
       - `BOX SEEN`, title `post this voucher?`; `boxcheck.py` ALL PASS, `answer file written: Yes`; `CLICKED Yes`; `box visible False` (lab 1392-E34 B3: CLICKED Yes 0.8 s after the box was seen);
       - either `posted_verified` on the first call (lab 1392-E34 B3, a Contra), or `pending` (`dialog_remaining_s` about 96, `retry_after_s` 15) and then `posted_verified` on the next (lab 2569-R Part B, an invoice, with a person's click);
       - dispatch counters `created` 1 and `altered`, `deleted`, `errors`, `ignored`, `cancelled`, `exceptions` 0, `verification_status posted_verified` (lab 2569-R Part B; 1392-E34 B3);
       - `POST posted_verified, created 1`, exit 0.
       Then, before any other call, screenshot the whole Tally window: a dialog or a half-entered screen = C7, STOP.
    9. **Any other end of 13.8** (No written or clicked, a timed-out box, a refusal, any other state): record it whole; this is a STOP of Part G (The approval boxes, rule 2). Then, if no dialog is up, the state reads once (C8): `g10`; close the launch; `s14x`: `N-0072.xml M-marks.xml`. Say what the book holds: EXPECT N-0072 0 rows and V 68, M 245 when nothing was posted (code: a declined approval sends nothing). Report; steps 14 and 15 are not run, and step 16 waits for the coordinator's word.
    10. `python3 -I calls.py verify C:\bridge-lab\2162-sales-q\run-g\answers\g05-build-0072.json C:\bridge-lab\2162-sales-q\run-g\inbox\g08-verify.json`: EXPECT `verification_status posted_verified` (lab 2569-R, 1392-E34).
    11. `g09`: `calls.py status-diff <run-0>\answers\04-status-aug.json <run-g>\answers\g09-status-aug.json`. EXPECT exactly one line `in B only`: `BP/26-27/0072`, date `20260801`, `included`, no `status_flags` (code `agent_gst_status.rs:432`: `status_flags` only when a row carries them; not shown through the build). Record the whole output.
    12. `g10`: EXPECT M 245, V 69 (lab 2162-HS: 69 after one invoice).
    - Close the launch.
14. **What Tally stored.** `s14`: `N-0072.xml R-0072.xml V-0072.xml R-0010.xml M-marks.xml`.
    - N-0072: EXPECT one row. R-0072: EXPECT one row, DATE 20260801, `BRIDGE Walk-in`, the six legs as sent (-1,051.00; 600.40; 400.40; 25.02; 25.02; 0.16), included Yes, uncertain and excluded No, `ISPOSTDATED` No (lab 2162-HS s4r, by a raw import of the same legs).
    - `count19.py --diff a-hss4r-R-0072.xml a-s14-R-0072.xml`: HS's raw `0072` against the build's. Record the whole output. EXPECT the legs `6 -> 6` (lab 2162-HS); every field that differs is recorded, not judged (identity fields will differ: GUID, MASTERID, ALTERID, and the keys Tally makes).
    - **The block, built against keyed.** `count19.py --diff a-s4-V-0073.xml a-s14-V-0072.xml`, and the six block fields of each as `count19.py` prints them (`BASICBUYERNAME`, `PARTYMAILINGNAME`, `CONSIGNEEMAILINGNAME`, `CONSIGNEESTATENAME`, `CONSIGNEECOUNTRYNAME`, `VATDEALERTYPE`). Record both whole.
      - EXPECT, on V-0072, what the build writes (code `agent_import_invoice.rs:3084`) and what row 2162-BC found Tally stores as sent (lab `lab/2162-bc-answers` 9f36c30f, raw): the three names `BRIDGE Walk-in`, `CONSIGNEESTATENAME` Rajasthan, `CONSIGNEECOUNTRYNAME` India, `VATDEALERTYPE` Regular.
      - EXPECT V-0073's three names and two places to equal V-0072's (not shown for a keyed invoice on this book). `VATDEALERTYPE` on the keyed invoice is recorded, not graded: nine of ten keyed invoices of an earlier book read `Unknown` and one `Regular` (the reference's PARTIAL bullet), and the cause is not known.
      - The two invoices differ in more than the sender (date, post-dated flag, lines, amounts). So this compare says whether the build's block matches what a keyed invoice gets, partial, and nothing about why a field differs.
    - V-0072: record also the registration fields as `count19.py` prints them.
    - R-0010: `--diff` against s2: EXPECT only `lists compared` and `legs: 6 -> 6` (lab 2162-HS). M: EXPECT V 69, M 245 (lab 2162-HS).
15. **The HSN summary after.** Step 12 again in full, with the same keys (fourteen, HS's 5.1 to 5.6). EXPECT (lab 2162-HS step 5, for its raw import of the same invoice):
    - the counts Total 40, Included 28, Not Relevant 7, Uncertain 5 (one more Total and Included than step 12);
    - 998313 up by exactly 630.42 / 600.40 / 15.01 / 15.01 / 30.02 and 998314 by exactly 420.42 / 400.40 / 10.01 / 10.01 / 20.02 against step 12's lines; every other line unchanged; the Total up by 1,050.84 / 1,000.80 / 25.02 / 25.02 / 50.04 (the 0.16 round off on no line);
    - under 998313 and 998314, step 12's lines unchanged plus one line `BP/26-27/0072` at 630.42 / 600.40 / 15.01 / 15.01 and 420.42 / 400.40 / 10.01 / 10.01.
    - Then `s15`: `M-marks.xml`; `count19.py --same-row a-s14-M-marks.xml a-s15-M-marks.xml`: EXPECT it passes (the look changed nothing; lab 2162-HS s6).
16. **Restore BK1 (R3)** by C6. After any STOP from step 4 on it runs only on the coordinator's word (C8). Then `s16`: `M-marks.xml U-taxunits.xml N-0072.xml N-0073.xml N-0074.xml R-0010.xml`, graded as step 8: `--same-row` against s1 (68/245); `--unit-diff` against s2, 0 leaves differ; N-0072, N-0073, N-0074 0 rows each; R-0010 `--diff` against s2, `lists compared` and `legs: 6 -> 6` only; the Gateway's current date 15-Sep-2026. Each sha256 recorded, not graded. Any not as EXPECTed: report before the close; the coordinator decides.
17. **Close** by C10. One tally.exe owns 9001; no company loaded; no `bridge_mcp` process this row started.

**Stop**, with no key pressed, if Tally shows a dialog at any point but those the prompt rule names, or if the gateway stops answering. Screenshot, send the screenshot (by name and hash; it stays local) and the answers to the run coordinator, and ask. Nothing is restored until the coordinator says so. Say in the report what the book holds:
- from step 4 to 8: `0073` (and the current date if it was not set back); from step 6, also the inactive row;
- from step 9 to 11: the second registration;
- from step 13.8: whatever the post did (`calls.py show` of the last post answer, and N-0072 once the exe is closed).

## Record
- Every raw answer's bytes and sha256 and `count19.py`'s lines; every `--diff`, `--unit-diff`, `--same-row`, `--before-write` output whole.
- For each launch: the folder checks, the launch file's copy and its sha256, the start command and shell, the process list, `client-log.txt` whole, every answer file in `answers\` whole with its sha256 (the kept-local rule), every `calls.py` output whole, the size of the server's error output, the EXE hash.
- For each box: `pg.py`'s output, `box\<tag>-ps.out`, `box\<tag>.answer`, `box\<tag>-check.out` (the real post's), the replay's output; the box log and its capture by name, bytes and sha256 only (kept local); the `BOX SEEN`, `CLICKED` and `box visible` lines quoted in the notes.
- `evidence\s0-selftest-box.txt` and `box\selftest\selftest-box-results.txt`.
- As text, word for word: every screen keyed in steps 4, 6, 9 and every line read in steps 12 and 15, every key pressed, every prompt and its key, with UTC times.
- BK1 and the three restores as C5 and C6 ask, under the kept-local rule. The release on screen, the Gateway's date and period at steps 1, 4, 8, 11, 16.
- The kept-local list (What stays on the lab PC).

## Expected
Estimates, not predictions, except where a step says the self-check predicts.
- **Part P:** `post_dated`, with `status_flags` `included`. The build decides `ISPOSTDATED` before the three flags (`status_of_voucher`), and the raw reads gave Yes and included Yes with the current date before, at and after the voucher's date (W21c). Not estimated: whether the build's own status request carries the flag as the read-back did; that is the row's question.
- **R-B:** 1 Aug admitted, 15 Sep `invoice_company_registration_inactive` (predicted by the model in Self-checks, on RU's captured bytes).
- **R-C2:** `invoice_company_registration_ambiguous` on both dates (predicted likewise).
- **Part G:** the smoke box declined and closed by the helper, and the replay ALL PASS; then `posted_verified` and HS's exact deltas: the build's request differs from HS's raw one by the build's own identity fields, not by the legs. Not shown: that `boxcheck.py`'s invoice case passes a real invoice box (the replay is its first test on one).

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
- **The smoke box failing, or the real box answered No:** Part G stops; the split stays at HS's raw evidence. If the replay is not ALL PASS on a real invoice box, `boxcheck.py`'s invoice case is the defect to fix before another sitting.

## Code admits the trigger
Lines at 8c3fb9f2. Every change named is an ancestor of it (`git merge-base --is-ancestor`): #2434 20f6e294, #2475 d80cf1d6, #2577 6fc389f6, #2325 2d25d3d0, #1646 9263657b, #1683 4ebb9ea9, #2307 21b45277.
- Sales is posted by the build: `LIVE_QUALIFIED_VOUCHER_TYPES` holds `Sales` (`src-tauri/src/agent_import.rs:443`).
- An invoice is built only with posting on: `invoice_post_not_enabled` (`agent_import.rs:4118`; its text at `agent.rs:2303`).
- The registration rule: `parse_company_registration` (`agent_import_invoice_wire.rs:1097`): per GST unit, the latest dated row on or before the invoice date; one in force is admitted; none with an inactive row in force is `..._inactive`; more than one is `..._ambiguous` (`:1165` to `:1170`). Its request is `render_company_registration_request` (`:1056`), which `U-taxunits.xml` is.
- `gst_status` decides `ISPOSTDATED` before the three flags (`agent_gst_status.rs:115`, `status_of_voucher`) and reports the flags' own state as `status_flags` (`:410` to `:433`); window at most 93 days, the voucher mark at most 25,000 (`:26`, `:29`).
- The invoice's approval text: `agent_import_post.rs:3655` (its test `agent_import_post_tests.rs:4709` to `:4731`); the box's title `tally/approved_import.rs:3413`. A declined invoice approval stops no later Sales post (`agent_import_ledger.rs:1308`).

## Not measured, and left out
- The block with against without through the build: the build always writes the six, so it cannot be varied by the build (row 2162-BC measured it raw). Step 14 compares the build's six with a keyed invoice's only.
- Composition, a changed GSTIN, a type bound to one tax unit, an invoice filed under a second registration: not in this row; see the table at the top.
- A post-dated invoice built by the build (it takes no such argument), a post-dated invoice in a later return period, GSTR-1 for September.
- What a repeat call after the smoke box's No shows (row 1392-E34's K measured it for a Receipt).
- The OLD build, any other book or release, any other client.

## Close and evidence
- **C10** at step 17. The book stays on the lab PC: it is synthetic.
- **C11:** evidence to a new branch `lab/2162-sales-q-answers`, with `SHA256.txt` and `kept-local.txt`, after the pre-push scan and the run coordinator's yes. A RESULTS.md gives each part's result first (P, R-B, R-C2, G, the smoke box), each with its confidence, then every step's EXPECT as met or not, with the build, the book, the date and the record. RESULTS quotes only grading lines from a kept-local file.

## Revisions
23 Nov 2026, after a red-team read of the first draft (d03e415f), on the run coordinator's rulings:
1. The Yes is `boxcheck.py`'s only, after a check of the company, GUID, batch, count and the built invoice's type, date, number, customer, lines and totals; the hand read and the hand click are gone, and so are `boxstate.ps1` and `readwin.ps1`.
2. The first box is the smoke box (13.4), answered No by the helper, with the marks equal after it and a STOP before any Yes otherwise.
3. The prompt rule and the path rule STOP with no key; nothing is restored without the coordinator's word, and before BK1 a prompt is a STOP.
4. `mkenv.py` is gone: five committed launch files, copied and hash-checked; Build says what each carries.
5. What stays on the lab PC is listed; RESULTS quotes only grading lines.
6. The restores are graded by the marks row, the registrations and the vouchers the parts read; sha256s are recorded only.
7. Step 4 writes its keys with their sources and gates each key no row recorded; steps 6.1, 9.2, 9.6 and 12.1 say what to do when a list or prompt does not show; step 12.6 has its Escape count; Ctrl+Q follows a shot of the title; no Escape at Select Company or the Gateway.
8. Every EXPECT names its source. Step 5's counts EXPECT is corrected: one item more, `post_dated` 0 to 1 and the total one more, every other count equal.
9. One folder, `C:\bridge-lab\2162-sales-q\`, for the files, the runs, the boxes and BK1 (`backup-1`, made new at step 1).

Row: lab/2162-sales-q/
