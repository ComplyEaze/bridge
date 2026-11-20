# Row 2569-R: the approval window on row 2440-CP's launch path, with the build that opened it minimised and with the build holding #2569's fix; then row 2440-CP's posts

Not for merge. Part of #2569; it also runs the steps of row 2440-CP (Part of #2162, #2440) that did not run on 19 Nov. Written 20 Nov 2026 at master 8c3fb9f2, where this branch starts. One row, on the synthetic book BRIDGE EINVOICE LAB. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2440-CP's) applies in full; where this sheet and it differ, the stricter holds. Record the TallyPrime version, the licence mode and the date. Confidence of everything here until the row runs: unverified.

**WHY.** On 19 Nov, in row 2440-CP, the approval window for `post_import` opened minimised twice, a title bar alone above the taskbar, and both posts timed out unanswered (#2569; answers `lab/2440-composed-answers` at 781598c0, section "The approval window opened minimised"). #2577 changed the box's hook in `src-tauri/src/tally/box_fit.rs`: when Windows is about to activate the box minimised, the hook restores it before it measures it, and writes `box-fit: restored from minimised` to its error output. A box Windows shows normally is left as it was. On the launch path of 19 Nov that error output goes nowhere (the server starts the approval helper with its error output discarded), so this row reads the window itself.

The issue names a cause it did not test: the helper may take its first show state from the start-up information it inherits. Part H tests that cause directly, by starting the helper by hand with a chosen show state, with each build. Part K repeats 19 Nov's failure on the same launch path with the same build, as the control. Part B changes only the exe and posts row 2440-CP's two invoices. The row checks:
- **H:** the old build started minimised opens its box minimised; the new build started minimised opens it restored, in front, with keyboard focus on No, and its error output holds the restore line; the new build started normally opens it as before, without the line.
- **K:** under the same client, launch file and book, the old build's post window opens minimised, as on 19 Nov. Nothing is answered and nothing is posted.
- **B:** under the same client, the new build's two post windows open restored, in front, with focus on No, read before anyone answers; [OWNER] answers both in time; and row 2440-CP's steps 8.4 to 11 run as written there.

**WRITES.** As row 2440-CP: one ledger seeded by a raw import (`I-seed.xml`), under the owner's seed rule; two duty ledgers and one Sales type keyed BY HAND; two invoices posted by the build's own `post_import`, one at a time, each after [OWNER]'s approval; the book restored to BK1 at the end. Part H writes nothing to Tally. Part K posts nothing.

**[OWNER] steps** are marked [OWNER]: the owner at the lab PC. Everything else is the operator's. The run coordinator's word, where the row asks for it (go-ahead, RESTORE, the evidence push), comes from the run coordinator. As in row 2440-CP: the go-ahead is the run coordinator's, under the owner's standing word for lab-book sheets; the answers to the approval windows and the computer-use grants may be the operator's where the owner's standing words to the lab cover them, and the operator records which words it relied on.

## The book as last read
Row 2440-CP's step 12 restored BRIDGE EINVOICE LAB to its BK1 and read it back equal to its step 1: one COMPANY row with the book's GUID, ALTVCHID 3, ALTMSTID 215; vouchers Sales 1 (with an IRN), Sales 2 and Payment 1, all dated 2025-04-03; `BRIDGE IRN Sales` IGST 18, HSN 998313; no `BRIDGE CP` customer, duty ledger or Sales type. Part A below builds those masters again, exactly as row 2440-CP did. Row 2440-CP's table "What the build requires, and where the row meets it" holds unchanged.

## Builds
Two builds, each in its own new empty folder, never mixed. Check every hash before the first use of each, and hash the exe again before each launch and each hand run; any other hash = STOP.

**OLD: the build that opened the window minimised.** The one row 2440-CP ran, from its build line:

OLD RC COMMIT: e0467fbc344969ba8d8c8f119e7726e3bdca6cca  CI RUN: 44463467192  ARTIFACT: unsigned-bundle-smoke-windows-latest, 60,609,752 bytes  ZIP DIGEST: 092e41de16d96986ee9b59971beba7f0a893d2cf8aaabf19482ab5f4cf26139c  MCPB archive_sha256: 212f5880b8c1daf3e8b48b54002e7c1aded1e771524f613a7b0de354d8fa98a9  EXE SHA256: c437138a08dc10a8609e42d28104920401b4615daa69f3cdf404f6e37d6323dd

Use the lab's own copy from row 2440-CP if its exe hash equals the line; otherwise download that run's artifact and check its three hashes against the line. GitHub keeps that artifact until 26 Nov 2026; after that, without the lab's copy, Parts H1 and K cannot run: STOP and tell the run coordinator. Folder `C:\bridge-lab\2569-r\exe-old\`. It reports version 0.7.0.

**NEW: the build holding #2569's fix.** The Windows bundle `unsigned-bundle-smoke-windows-latest` of a Bridge CI run started by hand on master at 8c3fb9f2. That commit holds #2577 (6fc389f6, the fix) and #2440 (d559303e, the e-invoice details row 2440-CP's steps need). Folder `C:\bridge-lab\2569-r\exe-new\`. The lab downloads it and checks three hashes, as row 2440-CP's sheet says:
- the zip's sha256 against the artifact's digest;
- the `.mcpb` against `archive_sha256` in its `mcpb-smoke.json`;
- `bridge_mcp.exe` against `binary_sha256` in the same file.

The artifact's size and digest below were read by the run coordinator from GitHub, without a download; the CI log does not print the other two. The lab sends all three hashes to the run coordinator, who fills the two left by a new commit on this branch, after checking the zip's against the line. GitHub keeps this artifact until 27 Nov 2026. While any field reads `<to fill>`, Parts H2, H3 and B do not run: STOP. Record the version the tool reports.

NEW RC COMMIT: 8c3fb9f2277966fa98b31bdc775b65c5ea3c29f7  CI RUN: 44740211021 (started by hand on 20 Nov 2026)  ARTIFACT: unsigned-bundle-smoke-windows-latest, 60,653,959 bytes  ZIP DIGEST: f8dc02e1f7bbb134e5ecc929c1f567aa265df2d3d1c866fea6636270e52e9472  MCPB archive_sha256: <to fill>  EXE SHA256: <to fill>

**Read in 8c3fb9f2, for the steps of row 2440-CP:** `e_invoice` under `invoice`, requiring `irn`, `ack_no` and `ack_date`; `vouchers` with `include_e_invoice`; the window lines `E-invoice IRN: ` and `E-invoice ack: <number> of <date>`; the registration read-back of #2475 (`invoice_registration_missing`, `invoice_registration_differs`); and `invoice_company_stopped`. Row 2440-CP's sheet note on the registration read holds: in steps 8 and 9 an answer reading `posted_divergent` whose `invoice_fields` name `invoice_registration_missing` or `invoice_registration_differs` is a FINDING: record it whole and go to step 10.

**Read in both builds, for the approval box:** the post box is `MessageBoxW` with Yes, No and Cancel, No the default button (`MB_DEFBUTTON2`), topmost and system-modal, titled `ComplyEaze Bridge — post this voucher?`. The helper is the exe run with the one argument `--confirm-journal` and no other (with any further argument the exe starts as a server instead). It reads from its standard input a nonce line (a UUID), a count line and the text to show; it writes a token to its standard output only on Yes. A declined box (No or Cancel) writes nothing and exits 1; a box that does not fit the screen exits with another status, and a box that could not be shown exits 0 with no token. Only the new build restores a minimised box and writes the restore line.

## The launches (Parts K and B)
Each launch runs a staged exe over stdio under `client.py` in this folder: byte-equal to the client row 2440-CP ran (`lab/2440-composed-answers` at 781598c0, `scripts/client.py`, sha256 `8d3729ea3ab3c5c272ddad1d1c7eb034b9c6fbdb6293b8b9620c190481b969fa`). The client starts the server with piped standard input and output, its error output to a file, and no start-up information of its own; calls go through its inbox, one at a time, as in row 2440-CP. Do not change the client, how it is started, or the console or account it is started from between the launches. The exe is the only setting the row changes from K to B; the client folder and data folder are new for each launch by design. The two builds differ by more than #2577 (OLD is #2440's head, NEW is master after it merged), but #2577 is the only change between them to `box_fit.rs`, the code that shows the box. Record how the client was started for each launch (the command, the shell, and whether that shell's window was minimised), the same both times.

Each launch has its own new empty client folder, the client's first argument: `C:\bridge-lab\2569-r\run-k\` for K and `C:\bridge-lab\2569-r\run-b\` for B, each holding, before the launch, only that launch's `launch-env.json` and two empty folders `inbox\` and `answers\` (the client reads both at its start). Nothing is copied from one to the other. A launch that fails before `tools/list` answers may be started again once, on the run coordinator's word, with a new empty client folder and data folder; the failed one is recorded whole. Each `launch-env.json` is as row 2440-CP's:
- `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9001`;
- ONE new empty `BRIDGE_AGENT_DATA_DIR`: `C:\bridge-lab\2569-r\data-k\` for K and `C:\bridge-lab\2569-r\data-b\` for B, each made before its launch, with its trailing backslash;
- `BRIDGE_TERMS_ACCEPTED=true`, the person responsible for the lab accepting the Terms of Use;
- [OWNER] posting ON: `BRIDGE_AGENT_ENABLE_WRITES=true`, set before each launch (the operator records only that it is present);
- `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` unset, and `BRIDGE_AGENT_ENABLE_BATCH_POST` absent.

Write each environment file with a tool that writes the bytes given, not through a shell that rewrites escapes: on 19 Nov a shell turned `\b` and `\24` in the path into other bytes and the first launch failed. Read the file's bytes back before the launch and record its sha256.

No other ComplyEaze Bridge may make a call against port 9001 during a launch, and no raw request is sent while an exe runs. Record the process list at the start of each launch. Close each launch before the next part starts.

## Reading a box (Parts H, K and B)
A box is read with two read-only helpers in this folder, before anyone answers it:
- **`boxstate.ps1`** (new): for each top-level window of a process, its handle, title, class, `iconic` (Windows holds it minimised), `visible`, `topmost`, its rectangle, its UI Automation `visual_state`, and whether it is the foreground window; then the foreground window's process and the element holding keyboard focus. Usage: `powershell -NoProfile -File boxstate.ps1 -ProcId <pid> -Out <new file>`.
- **`readwin.ps1`**: byte-equal to row 2440-CP's (`scripts/readwin.ps1` at 781598c0, sha256 `c138b645933c8e8d2a1f87e2767983ea911aced0de4453b1d28abf08f640873f`): every text of the process's windows, by UI Automation.

Neither clicks, presses a key or moves a window. Running them while a box is open is allowed; nothing else is done on the PC then, and nobody types into any console window, which could take the focus from the box. Commands go from the operator's own tool. The helper's pid: in Parts K and B, the child of the server whose command line contains `--confirm-journal` (`Get-CimInstance Win32_Process` with the server's pid as `ParentProcessId`; record its command line whole); in Part H, the pid `starth.ps1` prints.

Run `boxstate.ps1` first, then `readwin.ps1`, then take one screenshot of the whole screen. If `boxstate.ps1` prints `windows 0`, run it again every 2 seconds, each to a new file, up to 10 seconds; the first read that shows the box is the one graded. No box within 10 seconds is recorded as such. Record the time the box was first read, the time it was answered or timed out, and the answer.

**The box checks.** For the box's window (title `ComplyEaze Bridge — post this voucher?`, class `#32770`) in that first read:
- **W1, not minimised:** `iconic=False`. `visual_state` is recorded beside it; `iconic=False` with `visual_state=Minimized` is a FINDING, and `unreadable` is recorded as such, neither changing W1.
- **W2, in front:** `foreground=True`, and the FOREGROUND pid is the helper's.
- **W3, focus on No:** the FOCUS line names `No`, with the helper's pid. Its type is recorded, not graded: on 19 Nov UI Automation read this box's buttons as `ControlType.Pane`.
- **W4, topmost and visible:** `topmost=True` and `visible=True`.

Each is PASS or FAIL as read, recorded whole. A FAIL is recorded, not a STOP, except where a step says otherwise.

## Files
Every request file is byte-equal to row 2440-CP's file of the same name (`lab/2440-composed-sheet` at 3d8275c0), as are `gen.py`, its three templates, `run.py`, `count8.py` and `LAB-COMMON.md`; `SHA256.txt` holds every file's hash, and row 2440-CP's sheet describes each. New here:
- **`client.py`, `readwin.ps1`:** row 2440-CP's lab helpers, byte-equal, as above.
- **`boxstate.ps1`:** as above.
- **`starth.ps1`:** starts `<exe> --confirm-journal` with an explicit first show state (`-Show 1` normal, `-Show 7` minimised without activation, through `STARTF_USESHOWWINDOW`) and no console window, its standard input from a file and its standard output and error to new files. It prints the child's pid at once, then waits up to 600 seconds for it to exit and prints its exit code. Start it in the background with its own output to a new file, for example `Start-Process powershell -ArgumentList '-NoProfile','-File','starth.ps1',...` with `-RedirectStandardOutput h<n>-starth.txt -WindowStyle Hidden`, and read the child's pid from that file's STARTED line.
- **`H-input.txt`:** 145 bytes, with two LF line ends and none at the end: the nil UUID as the nonce, the count `1`, and the text `Lab row 2569-R, a hand run of the approval helper. This box posts nothing, whatever the answer. Answer No.` A hand-run box posts nothing whatever its answer: no server runs, and a Yes only writes a token to a file.

`boxstate.ps1` and `starth.ps1` were written for this row and have not been run; Part H's first run is their first. Before the sitting, check each parses (`[System.Management.Automation.Language.Parser]::ParseFile`) and run `boxstate.ps1` once against the running tally.exe: EXPECT one WINDOW line for Tally's window and a FOREGROUND and FOCUS line. Either failing = STOP before Part H: report it to the run coordinator, who fixes the helper by a new commit.

## Before the sitting
1. [OWNER] Tally is LICENSED on the lab PC. Educational mode anywhere on screen = STOP (C1).
2. [OWNER] The computer-use grant covers both exe paths, `exe-old\` and `exe-new\`. Without it this row does not run.
3. Check every file against `SHA256.txt` (Get-FileHash).
4. The helpers' check above.
5. Both builds staged and checked as **Builds** says; the NEW line filled.
6. [OWNER] Go-ahead for this row.

## Part H: the helper by hand (no Tally request, no server running)
No ComplyEaze Bridge server runs during Part H, and Tally is not called. For each run: hash the exe; run `starth.ps1` in the background with the exe, its `-Show`, `-In H-input.txt` and two new files `h<n>-out.txt` and `h<n>-err.txt`; read the box (**Reading a box**); then [OWNER] answers it **No** by mouse (a minimised box first restored by clicking its title bar, which is recorded); then wait for `starth.ps1`'s EXITED line. Record each run's `starth.ps1` output, both files whole with their sizes, the reads and the screenshot.

### H1: OLD, started minimised (`-Show 7`)
EXPECT `iconic=True` (the cause the issue names: a minimised first show state gives a minimised box). Then EXITED code 1, `h1-out.txt` 0 bytes. Record `h1-err.txt` whole. `iconic=False` is recorded as a FINDING: a minimised start does not reproduce the box of 19 Nov, so the issue's named cause is not shown. It is not a STOP.

### H2: NEW, started minimised (`-Show 7`)
EXPECT W1, W2, W3 and W4 PASS. EXPECT `h2-err.txt` to hold `box-fit: restored from minimised` exactly once, before the first line beginning `box-fit: box=`, which ends `fits=true`. Further `box=` lines (the box activated again) are recorded, not graded. Then EXITED code 1 and `h2-out.txt` 0 bytes.

### H3: NEW, started normally (`-Show 1`)
EXPECT W1, W3 and W4 PASS, and W2 recorded. EXPECT `h3-err.txt` to hold the line beginning `box-fit: box=` and ending `fits=true`, and no line `box-fit: restored from minimised`. Then EXITED code 1 and `h3-out.txt` 0 bytes. Compare the box's rectangle with H2's and record both.

Any EXITED code other than 1, a non-empty `-out` file, or a box that did not appear = record it and STOP Part H; go on to Part A on the run coordinator's word.

## Part A: the book, by raw requests (no ComplyEaze Bridge running)
Row 2440-CP's steps 1 to 5, exactly as written in its sheet (`lab/2440-composed-sheet` at 3d8275c0, `lab/2440-composed/ROW.md`), with its EXPECTs and its checks before each write, using this folder's byte-equal files. Its backup goes into a NEW empty folder `C:\bridge-lab\2569-r\backups\BK1\`. One change, as row 2440-CP recorded it: this release's Tax type list offers IGST, CGST, SGST/UTGST and Cess, so in step 4 key Tax type `CGST` for `BRIDGE CP CGST` and `SGST/UTGST` for `BRIDGE CP SGST`, and step 4's EXPECT reads `GSTDUTYHEAD` CGST and SGST/UTGST. Another list = stop before saving and ask, as step 4 says. EXPECT step 1 to read the book as **The book as last read** says. The book at the end of Part A: the masters of steps 3 to 5 added; no voucher.

## Part K: the control (OLD, ONE launch, nothing posted)
1. Step 0 as row 2440-CP's step 6.1, with the OLD exe and `data-k\`: the process list, `BRIDGE_AGENT_ENABLE_WRITES` present, the exe hash equal to the OLD line.
2. `list_companies`: EXPECT exactly BRIDGE EINVOICE LAB, with the GUID `count8.py` holds as `BOOK`. Another GUID = STOP.
3. MARKS: MK and VK. EXPECT them equal to Part A step 5's `s5r` ALTMSTID and ALTVCHID.
4. `build_import_xml` with row 2440-CP's step 7 arguments, unchanged. EXPECT `voucher_count` 1 and a `batch_id`, KA. A refusal: record it whole, close the exe, and go to Part B; Part K is then NOT RUN.
5. Tell [OWNER] a window will open, and that it is NOT to be answered or touched: nobody uses the mouse or keyboard until step 6. A Yes here would post invoice A outside steps 8 and 9, and Part B's own invoice A would then be refused as a duplicate number. `post_import` with `company_guid` and `batch_id` KA. When an answer reads `pending`, read the box (**Reading a box**). EXPECT W1 FAIL: `iconic=True` (the window of 19 Nov). Then, as the tool's flow asks, wait `approval.retry_after_s` and call `post_import` again with KA, until an answer is final. Nobody answers the box. EXPECT `import_approval_timed_out` with `attempt_recorded` false.
6. MARKS: EXPECT MK and VK unchanged. Close the exe. That ends Part K.

**If the box in step 5 is not minimised** (`iconic=False`): record its read whole. If it is still open, [OWNER] answers it **No**; record that answer and the final `post_import` answer. The launch path no longer reproduces the failure, so Part B cannot show the fix there, and #2569's grade is at best INCONCLUSIVE (**Grading #2569**). Part B still runs, for row 2440-CP's posts and to record W1 to W4: a minimised new-build box there is still a FAIL.

## Part B: the build holding the fix (NEW, ONE launch)
Row 2440-CP's steps 6 to 10, exactly as written in its sheet, with these changes:
- **The launch:** the NEW exe and `data-b\`, under the same client, started the same way as in Part K. In step 6.1 the exe hash is compared with the NEW line.
- **Step 6.4:** EXPECT M6 and V6 equal to Part K step 6's (and so to `s5r`'s; Part K posted nothing).
- **Steps 8.2 and 9.2, the box:** when the first answer reads `pending`, read the box (**Reading a box**) before [OWNER] is told to answer. EXPECT W1, W2, W3 and W4 PASS. Only then [OWNER] answers it **Yes** by mouse, within its time, and the step goes on as written. If the box is minimised (W1 FAIL), record it and STOP the post: nobody answers it, it is left to time out, and, in step 8 or step 9 alike, the step goes on as row 2440-CP's 8.3 says: record the answer whole, read MARKS once, and go to step 10. A minimised box under the new build is #2569's FAIL.
- **Steps 8.4 to 10:** as written there. The read-back's EXPECTs are row 2440-CP's.

Then close the exe. That ends Part B.

## Part C: raw reads, then the restore (no ComplyEaze Bridge running)
Row 2440-CP's steps 11 and 12, exactly as written in its sheet. The restore is BK1 of Part A, on the run coordinator's explicit RESTORE.

## Grading #2569
Graded on the new build's boxes; every other read is reported beside the grade.
- **FAIL:** any box of the new build read with W1 FAIL (`iconic=True`): in H2, H3, or Part B.
- **PASS** needs all of, with no FAIL:
  - H2: W1 PASS, and its error output holds the restore line (the hook restored a box that started minimised);
  - Part K step 5: the old build's box read with `iconic=True` (the launch path still gives a minimised box);
  - Part B step 8's box read, W1 PASS, and answered by [OWNER] inside its time; and step 9's box, if step 9 ran, the same.
- **INCONCLUSIVE:** no FAIL, and any PASS condition not met: for example H2 not minimised at the start (no restore line), Part K not run or its box not minimised, or no Part B box read (step 7 refused, or a STOP before step 8).

W2, W3 and W4 are graded for each box and reported with the result; a FAIL of any of them is a FINDING beside it. H1 is reported on the issue's named cause, and H3 on a box shown normally being left as it was (no restore line). Part K is one sample; 19 Nov's record holds two more.

## STOP RULES
C7 and C8; a post in Part K (any answer reading `posted_` or a MARK that moves): record it, close the exe, and go to step 12 on the run coordinator's word; any dialog before a call or a request; an approval window other than Part H's, K's step 5 and B's steps 8 and 9; a mark that moves outside the step that changes it (Part K moves none); any answer outside the stated shape; an exe whose hash differs from its line. On a STOP during a launch, close the exe first, then go to step 12 on the run coordinator's word.

## Not covered by this row
- Other MCP clients, a server started by the desktop app, and any other Windows version or account: the issue measured one client and this row uses the same one.
- The cause of the minimised start on the client path: Part H tests one cause by hand; the row does not establish what start-up information the client path gives the helper.
- The other boxes the hook guards (review, one-ledger create, releases, settle): they share the hook, but only the post box is opened here.
- A box that does not fit the screen, and a person who answers late: the fit check and the timeout are unchanged by #2577.
- Everything row 2440-CP's sheet lists under "Not covered by this row".

## Close and evidence
- **C10:** Shut Company; leave no company loaded. The book stays on the lab PC: it is synthetic.
- **C11:** evidence to a new branch `lab/2569-restore-answers`, with `SHA256.txt`, after the pre-push scan and the run coordinator's yes. It holds what row 2440-CP's C11 lists, and also:
  - for Part H, each run's `starth.ps1` output, its `-out` and `-err` files, its reads and screenshot;
  - for each box in Parts K and B, its `boxstate.ps1` and `readwin.ps1` reads, its screenshot, its open and answer times and its answer;
  - each launch's environment file sha256, how the client was started, and the process list;
  - both builds' hashes as checked at the lab.

  A RESULTS.md gives #2569's grade first, then H1 to H3, K, and each check of row 2440-CP's steps as PASS or FAIL, with the builds, the book, the date and the record.
