# Row 1392-E34: E3 and E4 of #1392 again, with the fixed box check, and what a repeat call after a kept No shows

Not for merge. Part of #1392. Written 23 Nov 2026. Lines below are at 8c3fb9f2, the head of the build this row runs, and this branch starts there. Confidence of everything here until the row runs: unverified.

## Why this row
Row 1392-E (sheet `lab/1392-e-statement-pairing` at a13ddc90, run 5 Nov 2026, answers on `lab/1392-e-answers`) settled E1 and E2. It did not settle E3 or E4:
- **E3 was not measured.** B3's Contra (one voucher) was never posted. The box check of that sitting knew only the box of a batch of several vouchers, so it declined the one-voucher box on the first call. On the second call it ended without an answer, after a read of the box log that was not guarded. The box helper then clicked No at its 70 s limit, after the call had already returned `pending` at about 40 s, so that No was kept. The third call got the kept No at once, with no box.
- **E4 was not run.** It runs only after E3 holds.

The sitting's own note on #1392 asks for two things in the next sitting. First, each post should start from its own new build of the batch (`build_import_xml`). Here that means one build for B3 and then one each for B4 and A4. The code does not need it, but it keeps each post's approval to its own call. Second, the sitting should record whether a repeat call after a kept No shows a new box.

This row does those two things and nothing else:
- **E3 and E4** as row 1392-E's steps 8 to 10. The box check is fixed: it reads the one-voucher box, and it reads the log only when the read is whole. Each post has its own new build.
- **K, the kept No.** On a batch of its own, after E4, a No is left to land after the call has returned. The row then records what the next call answers, and whether the call after that shows a new box.

Since row 1392-E ran, #2521 (8c6cc57a, in 8c3fb9f2) changed how a kept refusal is answered. It now carries `error.cause` `approval_answered_earlier`, and its `error.next_step` says that calling again with the same batch asks in a new box (Code, 5). K measures both on the build.

## What this row does not show
- **Not shown: E1, E2, F, B1 and the decoy.** They held on 5 Nov and are not run again. E3 pairs by reference, so it needs no statement to cover its day (Code, 3). E4 needs only B4's and A4's own cover.
- **Not shown: the parser's fit to real bank PDFs.** The statements come from the generator below, which writes only the layout the parser was written for.
- **Not shown: a box answered Yes after its call returned.** K answers No only, so nothing in K can post.

## Build
**NEW**, the build of rows 2569-R (Part B) and 2440-CP:
- the Windows bundle `unsigned-bundle-smoke-windows-latest` of a Bridge CI run on master at 8c3fb9f2, already on the lab disk at its granted folder;
- zip sha256 `f8dc02e1f7bbb134e5ecc929c1f567aa265df2d3d1c866fea6636270e52e9472`;
- mcpb sha256 `445d45a6fe18035258f69f04c7538241f19c80a39ad9ad88165b8a7c9f15ac8f`;
- exe `C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`, sha256 `1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`. It reports version 0.7.0.

Nothing is downloaded, installed or built. The OLD build is not used. `parse_bank_statement` reads PDFs with the PDFium beside the exe (`pdfium.dll` in the same folder; agent_bank_statement.rs:407).

One `bridge_mcp` for the whole row, under `client.py`, over stdio, directly on 9001, no relay. Its configuration is `launch-env.json` in this folder:
- `BRIDGE_AGENT_ENABLE_WRITES=true` and `BRIDGE_TERMS_ACCEPTED=true`;
- `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9001`;
- one new empty `BRIDGE_AGENT_DATA_DIR`, `C:\bridge-lab\1392e34\agent-data\`;
- `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` unset, and `BRIDGE_AGENT_ENABLE_BATCH_POST` absent. `client.py` refuses to start if any of these is in the file.

The file is copied byte for byte, never retyped, and its sha256 is checked against `SHA256.txt` before the launch. On 19 Nov a shell rewrote `\b` in a typed path.

## Book
`BRIDGE TRANSFER LAB`, GUID `d6e4c24f-8c7a-4797-93ec-e44bdff60bc9`, synthetic. TallyPrime 7.2 Silver, licensed.
- **Ledgers:** `TLab Bank A`, `TLab Bank B`, `Cash`, `TLab Clearing` (under Suspense A/c) and `Profit & Loss A/c`.
- **Last marks:** row 1392-E's m4 after its restore (5 Nov), **V 12, M 211**. It was left shut.
- **Fallback:** row 1392-E's backup BK-E, `C:\bridge-lab\1392e\backups\bke\TDBK1800_<company number>.001`, 32,428 B, sha256 `29c2de73fe7d383f5c388d92ff193e39ea9f65b59a57c8f812891dabf457cbb5`. It is not this row's backup. This row takes its own, BK-E2, at step 1.
- **Dates:** no day this row writes or reads (26 Oct to 3 Nov 2026) holds a voucher of the book's own.

Not on the PC, or another GUID: stop, the row has no book; never rebuild it. Any prompt when it opens: press no key, screenshot, stop and report its words. Education mode: stop.

## Rules
`LAB-COMMON.md` in this folder applies in full. It is v3, byte-equal to row 1392-E's, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`. THE COMPANY is BRIDGE TRANSFER LAB.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Writes.** Four posts through Bridge's `post_import`, at steps 3, 5a and 5c: B3 (1 voucher), B4 (3) and A4 (2). Also BK-E2 (step 1) and its restore (step 8). Nothing else is written: no raw import, no keying, and K posts nothing.
2. **Who answers the box.** No person. `box.ps1` clicks the button by mouse on the box's own control, and presses no key.
   - **In a post (P):** it clicks Yes only when `boxcheck.py` has written Yes. That needs the box's text to name this company, its GUID, this batch's id and this batch's voucher count, and, for a box of one voucher, the built voucher's type, date and every line. Anything else: No, and a stop.
   - **In K:** it clicks No, always.
3. **No key is pressed and no window is touched while a box may be up.** That is from the start of any `post_import` call until its box log shows `CLICKED` or `NO BOX`. A key aimed at Tally can answer the box. Nobody keys in Tally from step 2 to step 7: the backup of step 1 is taken before Bridge starts.
4. **The lab's operator runs every command** from `C:\bridge-lab\1392e34\` in PowerShell, with its own tool. It never types into a console window that is open on the screen.

## Kit
| file | what | origin |
|---|---|---|
| `LAB-COMMON.md` | v3 | byte-equal to `lab/1392-e/LAB-COMMON.md` (a13ddc90) |
| `generate-bank-statement-fixtures.py`, `make_statements.py`, `statements.sha256`, `.gitattributes` | the statement generator, the seven statements' rows and arguments, their sha256 | byte-equal to row 1392-E's (a13ddc90) |
| `marks.utf16le.xml` | the MARKS request (GUID, ALTVCHID, ALTMSTID) | byte-equal to row 1392-E's |
| `batches/k.json` | K's `vouchers`: one numberless Receipt, 26 Oct 2026, `Cash` Dr / `TLab Clearing` Cr 104.00, narration `e1392 k 001` | new |
| `launch-env.json` | Bridge's launch configuration | new |
| `helpers/client.py` | the stdio client of rows 2440-CP and 2569-R | byte-equal to `lab/2569-restore/client.py` (sha256 `8d3729ea...`) |
| `helpers/box.ps1`, `call.py`, `show.py`, `capwin2.ps1`, `front.ps1` | box watcher and clicker, one tool call, answer summary, window capture | byte-equal to row 1392-E's lab copies (`lab/1392-e-answers`, `scripts/`) |
| `helpers/boxcheck.py`, `p.py`, `cap.py`, `c2.py`, `marks.py` | the box check (its third version, the one 1392-E self-tested), the post P, capture, C2, MARKS | row 1392-E's lab copies with only the row folder changed to `C:\bridge-lab\1392e34` |
| `helpers/args.py` | writes every call's arguments | row 1392-E's, changed (its header says how) |
| `helpers/selftest34.py`, `helpers/sumcheck.py` | the box check's self-test on 5 Nov's saved logs; a sha256 list check | new |
| `placeholder-helpers/bk.py`, `restore-prep.py` | BK-E2's record; C6's checks | row 1392-E's, changed (headers) |

`SHA256.txt` lists every other file here with its sha256.

**The statements** (row 1392-E's Kit, unchanged). Every transfer goes from A (`TLab A xx7701`, `TLab Bank A`) to B (`TLab B xx7702`, `TLab Bank B`). The transfer line's party reads `TLAB SELF XFER`: on A's statements it is mapped `skip` to `TLab Bank B`, and on B's `contra` to `TLab Bank A`. Filler lines read `TLAB FILLER`, are unmapped, and post to `TLab Clearing` as Receipts.

| file | rows | reference on the transfer line | for |
|---|---|---|---|
| `e1392-b3.pdf` | 29 Oct transfer 7,003.00 | IMPS R3 | E3's Contra |
| `e1392-a3.pdf` | 29 Oct transfer 7,003.00 | IMPS R3 | E3's skip |
| `e1392-b4.pdf` | 1 Nov filler 1,204.00; 2 Nov transfer 9,004.00; 3 Nov filler 1,205.00 | IMPS R4 | E4's Contra, covering 2 Nov |
| `e1392-a4.pdf` | 1 Nov filler 1,206.00; 2 Nov transfer 9,004.00; 3 Nov filler 1,207.00 | none (a TPT line) | E4's skip, its statement covering 2 Nov |

`make_statements.py` also writes `e1392-a1`, `a2` and `b1`. This row does not parse them.

**The password file.** It is one file, `%USERPROFILE%\lab-1392e34-pw\pw.txt`, holding `lab-1392e-user` and a line feed (15 B), in a new folder under the user profile. On 5 Nov a file under the lab folder was refused as not private, and a copy under the user profile was accepted. `args.py` resolves the path itself, and neither it nor this sheet prints it.

## Lab folder
`C:\bridge-lab\1392e34\`, new: it must not exist before step 0.
- `run\`: this folder, as fetched;
- the helpers, copied from `run\helpers\` and `run\placeholder-helpers\` to the root, because `p.py` runs them from there;
- `calls\`, `box\`, `evidence\`, `reads\`, `shots\`, `pdf\`, and `backups\bke2\`;
- `agent-run\`: the client folder, holding before the launch only `launch-env.json` and the empty `inbox\`, `answers\` and `evidence\`;
- `agent-data\`: Bridge's data folder, new and empty.

## Steps
One at a time. Every command's output goes to a file, and the notes file `evidence\notes.txt` gets one line per step in UTC.

A file with no answer is a failed step, not an empty one. `call.py` prints `isError` and `rpc` for each answer: an answer that is an error, a JSON-RPC error, or no answer is "the request failed", never "nothing found".

Every log this row writes is bounded where it is written:
- a box log is one watch;
- a tool answer is one call, and `egress_log` is limited to 256 lines;
- a MARKS answer must be under 2,000,000 B (the printed size), else stop;
- the server's error output is the product's own: record its size at step 7, and only its first 200 lines leave the PC.

### 0. Load and checks
0.1 **Folders.** `Test-Path C:\bridge-lab\1392e34` must print False, else STOP. Make the folders: `New-Item -ItemType Directory C:\bridge-lab\1392e34\run, C:\bridge-lab\1392e34\calls, C:\bridge-lab\1392e34\box, C:\bridge-lab\1392e34\evidence, C:\bridge-lab\1392e34\reads, C:\bridge-lab\1392e34\shots, C:\bridge-lab\1392e34\pdf, C:\bridge-lab\1392e34\backups\bke2, C:\bridge-lab\1392e34\agent-run\inbox, C:\bridge-lab\1392e34\agent-run\answers, C:\bridge-lab\1392e34\agent-run\evidence, C:\bridge-lab\1392e34\agent-data`, then `Set-Location C:\bridge-lab\1392e34`. Fetch this branch at its commit and copy its `lab/1392-e34/` into `run\` (`git archive -o run.tar <commit> lab/1392-e34`, then `tar -x -f run.tar --strip-components=2 -C run`, then `Remove-Item run.tar`).
- Check the sheet: `python -I -X utf8 run\helpers\sumcheck.py run\SHA256.txt run evidence\s0-sha256.txt`. EXPECT the count line `N of N equal` and exit 0. Any other: STOP.
- Copy the helpers: `Copy-Item run\helpers\* .` and `Copy-Item run\placeholder-helpers\* .`.
- Fill in the data folder: in the root's `bk.py` and `restore-prep.py` only, replace the quoted `'<Tally data folder>'` (once in each; the header comment keeps its own copy) with Tally's data folder, as the Data Path on Tally's own Select Company screen shows it, written with forward slashes. With `<data path>` so written:
  `python -I -X utf8 -c "import sys; q = chr(39); a = q + '<Tally data folder>' + q; b = q + sys.argv[1] + q; [open(f, 'w', encoding='utf-8', newline='\n').write(t.replace(a, b)) if t.count(a) == 1 else sys.exit('HALT: ' + f) for f in ('bk.py', 'restore-prep.py') for t in [open(f, encoding='utf-8').read()]]" "<data path>"`.
  A `HALT`: STOP. Then `Get-FileHash bk.py, restore-prep.py | Format-List > evidence\s0-placeholders.txt`.
- Copy the configuration: `Copy-Item run\launch-env.json agent-run\launch-env.json`, then `Get-FileHash agent-run\launch-env.json | Format-List > evidence\s0-launch-env.txt`. EXPECT the hash on `SHA256.txt`'s line for `launch-env.json`. Different: STOP.

0.2 **C2.** With THE COMPANY loaded alone (C1), take the screenshot first: `python -I cap.py s0-start` (output in `shots\`, its hash printed). A dialog or a half-entered screen: STOP. Then `python -I -X utf8 c2.py s0`, which writes `evidence\s0-c2-*.txt`. EXPECT:
- one `tally.exe`;
- 9001 LISTENING by that pid;
- `/status` 200.
Record Tally's release and date and the PC's date. The PC's date must be on or after 3 Nov 2026, else STOP.

0.3 **m0.** `python -I -X utf8 marks.py m0` saves `reads\m0.xml` and writes a line to `evidence\marks.txt`. EXPECT curl exit 0, `200`, the answer under 2,000,000 B, parsed, and one COMPANY row: `GUID d6e4c24f-8c7a-4797-93ec-e44bdff60bc9 V 12 M 211`.
- Another GUID, or more than one row: STOP.
- Other V or M: STOP and report. `p.py` checks M 211, so this row needs those marks.

0.4 **The build.** `Get-FileHash C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe, C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\pdfium.dll | Format-List > evidence\s0-exe.txt`. EXPECT the exe's hash `1e3dd540...6b59`, the whole hash under Build, and a hash for `pdfium.dll` (it is present). Different, or either missing: STOP.
- `Get-Process bridge_mcp -ErrorAction SilentlyContinue | Measure-Object | Select-Object -ExpandProperty Count > evidence\s0-bridge-procs.txt` must write `0`. Any other: STOP, kill nothing.

0.5 **The box check's self-test.** `python -I -X utf8 selftest34.py C:/bridge-lab/1392e C:/bridge-lab/1392e34`, its output saved to `evidence\s0-selftest34.txt`. It runs this row's `boxcheck.py` on copies of row 1392-E's saved box logs of 5 Nov (`b3-p1`, `b3-p2`, `f-p1` and `b1-p1`, kept in `C:\bridge-lab\1392e\box\`). It reads B3's build answer of that day from `C:\bridge-lab\1392e\agent-run\answers\` and writes only to `box\selftest\`. It takes about 35 s.
- EXPECT eight lines ending `OK`, then `ALL CASES RIGHT`, exit 0:
  - t11 and t12: Yes on the one-voucher box;
  - t13: No on it when counted as two;
  - t14 and t16: Yes on the three- and two-voucher boxes;
  - t15 and t16b: No on them at the wrong count;
  - t19: No, exit 2, on a log cut inside its WAITING line.
- A `HALT` (a saved log missing) or any `WRONG`: STOP. No post is made with a box check that has not passed here.

0.6 **The fallback.** `Get-ChildItem C:\bridge-lab\1392e\backups\bke | Get-FileHash | Format-List > evidence\s0-bke.txt`, and record its size with `(Get-ChildItem C:\bridge-lab\1392e\backups\bke).Length`. EXPECT one file, 32,428 B and `29c2de73...7cbb5`. Absent or different: STOP, because the row would have no way back but its own backup.

### 1. BK-E2 (C5), before Bridge starts
`Get-ChildItem backups\bke2 | Measure-Object | Select-Object -ExpandProperty Count > evidence\s1-bke2-before.txt` must write `0`.
- Take the backup by Tally's Backup screen, to `C:\bridge-lab\1392e34\backups\bke2`, THE COMPANY only, as C5 says. Record the local time to the minute.
- Then `python -I -X utf8 bk.py`, which writes `evidence\s1-bk.txt`. EXPECT one folder holding the book's name and one file, `TDBK1800_<company number>.001`, above 0 bytes. Record its bytes and sha256 in the notes: step 8 passes them to `restore-prep.py`.
- Then `python -I -X utf8 marks.py m0b`. EXPECT as m0.

### 2. Kit, Bridge, the four parses
2.1 **Statements.** `python -I run\make_statements.py C:\bridge-lab\1392e34\pdf`, then `python -I -X utf8 sumcheck.py run\statements.sha256 pdf evidence\s2-statements.txt`. EXPECT `7 of 7 equal`. Any other: STOP.
- `cmd /c "python -I -X utf8 run\make_statements.py --calls > calls\calls-printed.txt"`. EXPECT a JSON object with the keys `b1 a1 a2 b3 a3 b4 a4`.

2.2 **Password file.** `python -I -X utf8 -c "import os; d = os.path.join(os.environ['USERPROFILE'], 'lab-1392e34-pw'); os.mkdir(d); open(os.path.join(d, 'pw.txt'), 'wb').write(b'lab-1392e-user\n'); print(os.path.getsize(os.path.join(d, 'pw.txt')))"`. EXPECT `15`. An error, because the folder exists: STOP.
- Record the file's ACL: `python -I -X utf8 -c "import os, subprocess; f = os.path.join(os.environ['USERPROFILE'], 'lab-1392e34-pw', 'pw.txt'); o = subprocess.run(['icacls', f], capture_output=True, text=True).stdout; open('evidence/s2-pw-acl.txt', 'w', encoding='utf-8', newline='\n').write(o.replace(f, '<pw file>').replace(os.environ['USERNAME'], '<user>'))"`. EXPECT in the file: SYSTEM, Administrators and `<user>` only, each `(I)(F)`, inherited.

2.3 **Arguments.** `python -I -X utf8 args.py fixed`. EXPECT nine lines: `args-empty`, `-guid`, `-vt`, `-egress`, `args-parse-b3`, `-a3`, `-b4`, `-a4` and `args-build-k`, each with a hash. A `HALT`: STOP.

2.4 **Launch.**
- `Get-Process bridge_mcp, python -ErrorAction SilentlyContinue | Select-Object Id, ProcessName | Format-Table > evidence\s2-procs.txt`. EXPECT no `bridge_mcp`.
- Start the client in the background with no window: `Start-Process python -ArgumentList '-I','-X','utf8','C:\bridge-lab\1392e34\client.py','C:\bridge-lab\1392e34\agent-run','C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe','1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59' -WindowStyle Hidden -RedirectStandardOutput evidence\client-out.txt -RedirectStandardError evidence\client-err.txt -PassThru`. Record the pid.
- EXPECT, within 60 s, `agent-run\client-log.txt` to show:
  - the exe sha256 ok and the launch file's sha256 (equal to 0.1's);
  - `BRIDGE_AGENT_ENABLE_WRITES present: True`;
  - `initialized` with version 0.7.0.
- A `stop:` line, or no `initialized`: STOP.

2.5 **Company and types.** `python -I -X utf8 call.py agent-run s2-companies list_companies calls\args-empty.json`. EXPECT this GUID, one company. Record `books_from`, which must be on or before 26 Oct 2026.
- `call.py agent-run s2-status tally_status calls\args-empty.json`: record release, licence and mode.
- `call.py agent-run s2-vt masters calls\args-vt.json`. Read the Receipt and Contra rows' `series_numbering` with `show.py`. EXPECT `method` automatic and `sub_method` auto_retain for both. Any other: STOP. This row alters no type.

2.6 **The four parses, before any write.** Run `call.py agent-run <k>-parse parse_bank_statement calls\args-parse-<k>.json` for k = b3, a3, b4, a4, one at a time, then `show.py` on each answer. A parse sends nothing to Tally and writes only its proposals file to the data folder. EXPECT `reconciled` with the four controls equal, and:

| k | EXPECT |
|---|---|
| b3 | 1 row, `TLAB SELF XFER`, contra, `skipped` 0 |
| a3 | 1 row, `TLAB SELF XFER`, `skipped` 1 |
| b4 | 3 rows: 2 `TLAB FILLER` to suspense and the transfer as contra; `skipped` 0 |
| a4 | 3 rows: 2 `TLAB FILLER` to suspense and the transfer skipped, with no reference; `skipped` 1 |

- A refusal `statement_password_file_permissions`: record it whole and STOP. On 5 Nov the folder under the user profile was accepted, so a refusal is a finding, not a step.
- Any other refusal, or a figure that differs: STOP before step 3.

2.7 **m2.** `python -I -X utf8 marks.py m2`. EXPECT as m0.

### 3. B3, the Contra of E3
3.1 `python -I -X utf8 args.py build-from <the b3-parse answer file> b3`, then `call.py agent-run b3-build build_import_xml calls\args-build-b3.json`. EXPECT `voucher_count` 1, a Contra of 29 Oct 2026, `TLab Bank B` Dr and `TLab Bank A` Cr, 7,003.00, and a `batch_id`, B3. A refusal: STOP.

3.2 **P:** `python -I -X utf8 p.py b3 <B3> 1 12`, its output saved to `evidence\s3-p.txt`. EXPECT:
- the box log `box\b3-p1-box.txt` with `BOX SEEN`, title `post this voucher?`;
- `box\b3-p1-check.out` `ALL PASS`, then `CLICKED Yes`;
- dispatch `posted_verified` with `created` 1;
- `verify_import` `posted_verified` 1 of 1;
- `VERIFY posted_verified n of n | MARKS V 13 M 211`, exit 0.

Any other: STOP. The box check writing No is a stop, not a retry.

### 4. E3
`args.py build-from <the a3-parse answer file> a3`, then `call.py agent-run a3-build build_import_xml calls\args-build-a3.json`, then `show.py` on it, saved to `evidence\s4-e3.txt`. EXPECT:
- `result.state` `skips_recorded` and `skipped_rows` 1;
- in `result.transfers`: `state` computed, **`skips_paired` 1** and `skips_open` 0, with A3's row not among `open`.

Keep the answer whole. Then `call.py agent-run s4-egress egress_log calls\args-egress.json`, whole. Then `marks.py s4-marks`. EXPECT V 13, M 211.

**E4 runs only if step 4 gave `skips_paired` 1.** Otherwise go to step 6.

### 5. E4
5a **B4.** `args.py build-from <the b4-parse answer> b4`, then `call.py agent-run b4-build build_import_xml calls\args-build-b4.json`. EXPECT `voucher_count` 3: two Receipts (1 and 3 Nov) and a Contra (2 Nov, 9,004.00). Then P: `p.py b4 <B4> 3 13`. EXPECT as 3.2, with the title `post 3 vouchers?`, `created` 3 and `V 16 M 211`.

5b **A4's build.** `args.py build-from <the a4-parse answer> a4`, then `call.py agent-run a4-build build_import_xml calls\args-build-a4.json`, then `show.py`, saved to `evidence\s5-a4-build.txt`. EXPECT:
- `voucher_count` 2, the two Receipts;
- in `result.transfers`, A4's row **`matched_unconfirmed`**. A's statement is not yet posted and verified, so it covers no day.

5c **A4's post.** P: `p.py a4 <A4> 2 16`. EXPECT as 3.2, with `created` 2 and `V 18 M 211`. Then `show.py` on the `a4-verify` answer, saved to `evidence\s5-a4-verify.txt`. EXPECT in its `transfers` **`skips_paired` 2** and `skips_open` 0.

### 6. K: what a repeat call after a kept No shows
A batch of its own, built once, never posted. Each sub-step is one call. `p.py` is not used here, because it calls again on `pending` and would join the open box. No key is pressed and nothing is sent to Bridge or Tally from 6.1's start to 6.3's end, except the calls named.

6.0 **Build.** `call.py agent-run k-build build_import_xml calls\args-build-k.json`. EXPECT `voucher_count` 1, a Receipt of 26 Oct 2026, and a `batch_id`, K. Then `python -I -X utf8 args.py batch k <K>`.

6.1 **K1: the No lands after the call has returned.**
- `$b = Start-Process powershell -ArgumentList '-NoProfile','-File','C:\bridge-lab\1392e34\box.ps1','-RowDir','C:\bridge-lab\1392e34','-Tag','k-p1','-SeeWithin','120','-AnswerBy','70' -WindowStyle Hidden -RedirectStandardOutput box\k-p1-ps.out -PassThru`.
- `Start-Sleep 2`, then `python -I -X utf8 call.py agent-run k-p1 post_import calls\args-post-k.json`.
- No answer file is written, so `box.ps1` clicks No 70 s after it saw the box.
- EXPECT the call to return after about 40 s with `approval.state` `pending` and `attempt_recorded` false.
- Then `$b.WaitForExit()`. EXPECT `box\k-p1-box.txt`:
  - `BOX SEEN`, title `post this voucher?`, naming K's batch;
  - `no valid answer file by 70 s`, then `CLICKED No`, then `box visible False`;
  - exit 0.
- **The grade of K1:** the `CLICKED No` time must be later than the call's end (`agent-run\evidence\tools-log.txt`: its sent time plus its seconds). Record both times.
- If the call returned after the click, or answered anything but `pending`: K is not set up. Record it whole and go to step 7.

6.2 **K2: the call that reads the kept No.** Within 5 minutes of the click:
- `$b = Start-Process powershell -ArgumentList '-NoProfile','-File','C:\bridge-lab\1392e34\box.ps1','-RowDir','C:\bridge-lab\1392e34','-Tag','k-p2','-SeeWithin','20','-AnswerBy','20' -WindowStyle Hidden -RedirectStandardOutput box\k-p2-ps.out -PassThru`.
- `Start-Sleep 2`, then `call.py agent-run k-p2 post_import calls\args-post-k.json`.
- Within 3 s of its answer: `Get-CimInstance Win32_Process | Where-Object Name -eq bridge_mcp.exe | Select-Object ProcessId,ParentProcessId,CommandLine | Format-List > evidence\s6-k2-procs.txt`.
- Then `$b.WaitForExit()`.
- EXPECT:
  - an answer at once, under 2 s, with `error.code` **`import_approval_declined`**, `error.cause` **`approval_answered_earlier`**, and an `error.next_step` beginning `This call showed no approval box.`;
  - `attempt_recorded` false or absent;
  - the box log `box\k-p2-box.txt` with its `watch start` line, then `NO BOX seen within 20 s`, exit 3;
  - the process list with one `bridge_mcp.exe` and no process whose command line holds `--confirm-journal`.
- Keep the answer whole.

6.3 **K3: the call after that, with the same batch.** At once after 6.2:
- `$b = Start-Process powershell -ArgumentList '-NoProfile','-File','C:\bridge-lab\1392e34\box.ps1','-RowDir','C:\bridge-lab\1392e34','-Tag','k-p3','-SeeWithin','120','-AnswerBy','20' -WindowStyle Hidden -RedirectStandardOutput box\k-p3-ps.out -PassThru`.
- The process read while the call waits, started now in the background because `call.py` blocks: `$c = Start-Process powershell -ArgumentList '-NoProfile','-Command','Start-Sleep 10; Get-CimInstance Win32_Process | Where-Object Name -eq bridge_mcp.exe | Select-Object ProcessId,ParentProcessId,CommandLine | Format-List | Out-File -Encoding utf8 C:\bridge-lab\1392e34\evidence\s6-k3-procs.txt' -WindowStyle Hidden -PassThru`. It reads about 8 s after the call starts, inside the 20 s before the No.
- `Start-Sleep 2`, then `call.py agent-run k-p3 post_import calls\args-post-k.json`.
- Then `$b.WaitForExit(); $c.WaitForExit()`.
- **THE RECORD:** whether a new box shows. Yes means `BOX SEEN` in `k-p3-box.txt`, its handle and pid, its title and its `Batch:` line naming K, and a `--confirm-journal` child in the process list.
- **EXPECT:**
  - a new box;
  - `box.ps1` `CLICKED No` at 20 s, while the call waits;
  - the call answers `import_approval_declined` with **no** `error.cause`, as a No given to a waiting call (Code, 5), and `attempt_recorded` false.
- If no box shows: the call's answer recorded whole, `NO BOX seen within 120 s`, exit 3. Then no key is pressed for 130 s after the call started, because Bridge's box limit is 120 s. Nothing more is sent to the batch in either case.

6.4 `marks.py s6-marks`. EXPECT V unchanged from step 5's (18 after E4, 13 without it), M 211. K posts nothing.

### 7. End of Bridge
- `call.py agent-run s7-egress egress_log calls\args-egress.json`, whole.
- `python -I -X utf8 marks.py m3`. EXPECT V = m0b's V + 6 after E4, + 1 without it, and M 211.
- Stop the client: `New-Item agent-run\inbox\stop`. EXPECT `session ended` in `client-log.txt` within 60 s and `Get-Process bridge_mcp` empty. If it is still running, record it and STOP; kill nothing.
- Record the size of the server's error output, `agent-run\server-stderr-*.log`.

### 8. Restore BK-E2 (C6)
`python -I -X utf8 restore-prep.py before <bytes> <sha256> "<the Restore list time>"`, using step 1's record. EXPECT `name, bytes and sha256 equal to the record: True` and the count line in `evidence\company-folder.txt`.
- Then the restore, as C6 says, of BK-E2 only.
- Then `restore-prep.py after`, then C6's screenshots, then `marks.py m4`. EXPECT the COMPANY row's GUID, V and M equal to m0's, the first read, and to m0b's (V 12, M 211).
- Anything else: STOP. The way back is then BK-E, restored only on the run coordinator's word.

### 9. Close (C10)
Shut THE COMPANY. Run `c2.py s9`. EXPECT one `tally.exe` on 9001 and `/status` 200. Take the screenshot `cap.py s9-close`: no company loaded, and Tally's date as at step 0. No XML request is sent after the shut.
- Remove the password folder (`Remove-Item -Recurse $env:USERPROFILE\lab-1392e34-pw`) and record that it is gone.

## What the result decides
| step | result | reading on #1392 | confidence when the evidence is whole |
|---|---|---|---|
| 3 | B3 `posted_verified`, V 13 | the fixed box check passes a one-voucher box: 5 Nov's helper defect is closed on the build | verified |
| 4 | `skips_paired` 1, A3 not open | **E3 holds, by reference.** A3's statement has one row, so it covers no day (proposals.rs:290). Round 1 pairs equal references with no date bound (Code, 3) | verified |
| 4 | A3's row open (`reference_differs`, `not_established`, `tally_unchecked` or another state) | **E3 does not hold.** The reference was not used. E4 is moot and is not run | verified, as a negative |
| 5b | A4 `matched_unconfirmed` at build | round 2 found the date pair but could not confirm A's cover yet, as designed | verified |
| 5c | `skips_paired` 2 in A4's `verify_import` | **E4 holds:** with both statements verified around 2 Nov, a row with no reference pairs by date | verified |
| 5c | A4's row still `matched_unconfirmed` after verify | a window fell short. Recorded with both statements' cover, not a stop | partial |
| 6.1 | the No clicked after K1 returned `pending` | the set-up of 5 Nov's call 2, reproduced on purpose | verified |
| 6.2 | `import_approval_declined`, cause `approval_answered_earlier`, no box, no `--confirm-journal` child | a kept No is returned once, at once, and says so (#2521) | verified |
| 6.2 | no cause, or a box shows | #2521's kept-refusal answer is not on the build: a finding | verified, as a finding |
| 6.3 | `BOX SEEN` naming K, a `--confirm-journal` child in `s6-k3-procs.txt`, `CLICKED No`, then a decline with no cause | **a repeat call after a kept No shows a new box**, as #2521's `next_step` says | verified |
| 6.3 | `NO BOX`, no `--confirm-journal` child, and an answer that is not a decline given to a waiting call | **a repeat call after a kept No shows no new box**: the `next_step` is wrong, a finding | verified, as a finding |
| 6.3 | any other mix (for example `NO BOX` with a child, or `NO BOX` and a plain decline, or `pending` with no box seen) | the three records disagree; not graded | unverified |
| any | a refusal before a write, or a stop rule | recorded whole; nothing after it runs | n/a |

## Stop rules
- Any dialog on Tally's screen, before or after a call.
- Any refusal before an attempt is recorded, any parse refusal, and any refusal of a build.
- Any call with no answer in 10 minutes.
- A box the check does not pass. `boxcheck.py` exits other than 0, or `p.py` exits other than 0.
- The self-test of 0.5 not `ALL CASES RIGHT`.
- A Receipt or Contra type that is not automatic with retain.
- A statement or sheet file whose sha256 differs from its list.
- V not as expected after a step, or M other than 211: report before the next step.
- A key pressed, or a window touched, while a box may be up (Rules, 3): record it and stop.
- Education mode.

After a stop, nothing more is sent or keyed (C8). The book then holds whatever was posted before it:
- B3's Contra from step 3;
- B4's three vouchers from step 5a;
- A4's two Receipts from step 5c.

K posts nothing. The way back is BK-E2, then (on the run coordinator's word) BK-E.

## Limits
- One generated statement layout (HDFC), one book, one sitting, and k = 0 (`TRANSFER_K_DAYS`, agent_transfer_pairing.rs:276). Only the same printed day pairs by date.
- K is one batch of one voucher. It times one No against a 40 s call budget and a 70 s click, both well inside the box's 120 s limit. It does not try a box left to time out, a No given after the 15 minutes an approval lasts, or a second kept No.
- E1, E2 and the decoy are not repeated (What this row does not show).
- K runs last, so that its batch cannot touch E3's and E4's posts. A stop in steps 3 to 5 therefore ends the row before K, and the kept-No record then waits for another sitting.

## Code admits the trigger (8c3fb9f2)
1. **A skip comes only from a statement PDF.** `parse_bank_statement` (agent.rs:3562) reads an hdfc PDF, and `skip` is a mapping treatment. The password file must be private to the user (local_files/access.rs:338).
2. **Records and views.** A statement of only skipped rows is recorded, not built, as `skips_recorded` (agent_import.rs:2336) with its `transfers` (:2341). A build carries the same section (:2218), and so does `verify_import` (:2393).
3. **Pairs.**
   - A candidate is a Contra of another lineage, built from the other account's statement (`is_edge`, agent_transfer_pairing.rs:1470), and posted and verified (`is_verified`, :1474).
   - Round 1 pairs equal references with no date bound (:1271).
   - Round 2 takes only pairs where one side has no reference (:1298). Such a pair is `Paired` only when both statements cover the dates, else `MatchedUnconfirmed` (:1359 to :1363).
   - Cover is the statement's rows less its first and last day (`coverage`, proposals.rs:290). A3 has one row and covers nothing; B4 and A4 each cover 2 Nov.
   - The open states are decided in `open_state` (:1485).
4. **Posting.**
   - Receipt and Contra are live-qualified (`LIVE_QUALIFIED_VOUCHER_TYPES`, agent_import.rs:443).
   - A numberless voucher needs automatic numbering with retain (`admit_numbering_for_post`, agent_import_numbering.rs:493).
   - Dates must lie between books-from and the PC's today (`validate_dates`, agent_import.rs:5139).
   - The box text of a one-voucher batch (`Create ONE <type> in "<company>"`, agent_import_post.rs:3482) and its title (`post this voucher?`, tally/approved_import.rs:3413) are what `boxcheck.py` reads. They are unchanged since row 1392-E's build.
5. **The kept No.**
   - Times: a call waits at most `CALL_BUDGET`, 40 s (agent_import_approval.rs:46), and the box is shown for at most 120 s (`POST_DIALOG_LIMIT`, tally/approved_import.rs:578).
   - Keeping: a refusal that ends after its call returned is kept by `settle` (agent_import_approval.rs:706) for the batch's next call.
   - Return: that call's `begin` takes it first (:683; `take_refusal`, :586, returns it once), and it is answered as `Begin::Refused` (agent_import_post.rs:522) with the cause `approval_answered_earlier` (agent.rs:1434). The `next_step` (agent.rs:1450) says the next call "asks them again in a new approval box".
   - A call made while a box is open joins it, and shows no second box (`Begin::Join`).

## Who sends each EXPECT's request (8c3fb9f2)
| step | request | sent by | the count |
|---|---|---|---|
| 0, 1, 2, P, 4, 6, 7, 8 | marks, raw | `marks.py`'s curl on 9001 | one COMPANY row; V is its ALTVCHID, so +n is n vouchers created; M its ALTMSTID |
| 1 | company list, status, voucher types | `list_companies`, `tally_status`, `masters` | the rows named |
| 1 | none to Tally | `parse_bank_statement` (agent.rs:3562) | rows, skipped, counterparties |
| 3 to 6 | the build's reads; for a skip, the day reads | `build_import_xml` (agent.rs:3531) | `voucher_count`, `skipped_rows`, `transfers` |
| P, 6 | reads, the one import, the read-back; or a refusal with nothing sent | `post_import` (agent.rs:3569) | `CREATED`, `posted_verified`; `error.code` and `error.cause` |
| P | the verification reads | `verify_import` (agent.rs:3595) | `posted_verified`; `transfers` |
| 4, 7 | none to Tally | `egress_log` (agent.rs:3537) | the log's lines |
| 6 | none | `box.ps1`, `Get-CimInstance` | the box seen or not; the `--confirm-journal` child |

## Keep
On `lab/1392-e34-answers`, with its own `SHA256.txt`:
- every tool answer whole, with its size and sha256;
- every box log, check output and `-ps.out`;
- the self-test's results;
- the marks answers;
- BK-E2's name, size and sha256, and the masked `restore-plan.txt`;
- the process lists of 6.2 and 6.3, with any user name written as `<user>`;
- a `RESULTS.md` with each case's state first, then the table of What the result decides filled in with its confidence;
- `evidence\notes.txt`, with one line per step in UTC.

These stay on the lab PC, listed by name, size and sha256:
- the statements and the password file;
- screenshots and box captures;
- the data folder;
- the server's error output beyond its first 200 lines.

Nothing is pushed before the pre-push scan and the run coordinator's yes (C11).
