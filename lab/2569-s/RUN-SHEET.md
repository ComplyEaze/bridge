# Row 2569-S: the approval box on the launch path of 19 and 20 Nov, read before any screen act, and the start-up information its processes received

For #2569. On 19 and 20 Nov 2026, in row 2440-CP, the post approval box of the old build opened minimised twice and both posts timed out unanswered (answers `lab/2440-composed-answers` at 781598c0 and `lab/2440-composed-answers-2` at fd6d9c8b). Row 2569-R (answers `lab/2569-restore-answers` at 400575f3) did not see it again, but its Part K, the only launch of the old build on that path, was cut before its post call, so no box of the old build on the client path has been read since. This row opens that box again, reads how it shows before any screen act, reads what start-up information the server and its helper received, and posts nothing. One sitting, one synthetic book, one backup, a restore at the end. Written 24 Nov 2026 on a branch cut at master 8c3fb9f2. Until the row runs, every EXPECT is **unverified** unless its source says otherwise.

**How an EXPECT is marked.** **(code `file:line` at `<commit>`)** is a rule read in the tree of that commit; **(lab `row`)** is an earlier observation; **(not shown)** is what this sitting measures. A helper's exit 2 is a failed read, never "nothing found"; each helper below says how it tells the two apart.

## The questions and the one variable per launch

- **Q1.** On the launch path of 19 and 20 Nov, does the old build's box open minimised again? Each box is read before anyone or anything acts on the screen, and again after one screenshot by the screen tool.
- **Q2.** What show state and start-up flags did the server and its `--confirm-journal` helper receive, on that path and on row 2569-R's?
- **Q3.** If a box of the old build is minimised, is the same launch's box restored under the build holding the fix, and is the restore line in the helper's error output?

| launch | build and path | its one change from the launch before it | box | posts |
|---|---|---|---|---|
| L1 | OLD at row 2440-CP's own folder, the exe the minimised boxes came from; not on the screen tool's grant list | none: the first launch (the path of 19 and 20 Nov) | read, never answered | none |
| L0 | OLD at row 2569-R's `exe-old` path, on the grant list | the exe's path and so its grant (the same bytes) | read, never answered | none |
| L2 | NEW, at the grant state of the OLD launch whose box was minimised: `exe-new` (on the grant list) if that was L0; a NEW-hash path not on the grant list, named by step 0's reply, if it was L1 | the build (from that OLD launch) | read, never answered | none |

**What differs from row 2569-R's cut Part K, named from the records.** (1) Part K launched the old build from `exe-old`, granted on 22 Nov before it ran (lab 2569-R, "Grant runs"); on 19 and 20 Nov it ran from row 2440-CP's folder, never granted (lab 2440-CP, `launch-method.txt`; grants then were TallyPrime and Bridge only, lab 2569-R notes). L1 restores that path. (2) Part K never opened a box; this row does, in L1 and L0. (3) On 19 and 20 Nov every first sight of the box was a screenshot by the screen tool (lab 2440-CP: "found it on a screenshot"); in row 2569-R the box was first read by `boxstate.ps1` and the screenshots were the lab's own capture. The lab's record for #2161 shows that the screen tool minimised the window of an app outside its grant before a screenshot (answers `lab/2161-taskbar-activate-answers` at 84e0caf7, RESULTS finding 5; a later read in `lab/2161-taskbar-activate-b-answers` at 34b62f76, deviation 4, left another box not minimised, its grant state not recorded). So each box here is read first with no screen act (R1, R1b), then after exactly one screenshot by the screen tool (S1, R2). Each box gets both reads, so (2) and (3) are measured inside one launch and (1) between L1 and L0.

**Order.** L1 first: it is the launch of 19 and 20 Nov, unchanged. L0 second: it changes only the path. L2 last and only after a minimised OLD box (`M0`, `MB` or `M1`); if both were minimised, L2 pairs with L1 when its path exists, else with L0. No launch is answered, so the book is the same before each.

## Book

`BRIDGE EINVOICE LAB` (synthetic), its GUID pinned as `BOOK` in `count8.py`, `ans.py` and `postcall.py` and held in the call files; loaded alone on TallyPrime 7.2 Silver. `LAB-COMMON.md` v3, byte-equal to row 2569-R's, applies in full; the stricter text wins.

- **Starting state.** Row 2569-R restored it to its BK1 and read it back: one COMPANY row with the book's GUID, ALTVCHID 3, ALTMSTID 215, no `BRIDGE CP` customer, duty ledger or Sales type (lab 2569-R, Part C step 12). EXPECT exactly that at step 1. Any difference: STOP before the backup.
- **Writes.** As row 2569-R's Part A: one ledger by a raw import (`I-seed.xml`), two duty ledgers and one Sales type keyed on screen; nothing else. No post: every box is left unanswered and times out.
- **BK1 is the way back.** Taken at step 2 into the new empty `C:\bridge-lab\2569-s\backups\BK1\` by C5; restored at step 9 by C6 on the run coordinator's RESTORE.
- Record the release shown on Tally's screen and Tally's current date.

## Builds

Only the two builds already on the lab disk. No copy, move, download, install or build, at any path.

OLD RC COMMIT: e0467fbc344969ba8d8c8f119e7726e3bdca6cca  ZIP DIGEST: 092e41de16d96986ee9b59971beba7f0a893d2cf8aaabf19482ab5f4cf26139c  MCPB archive_sha256: 212f5880b8c1daf3e8b48b54002e7c1aded1e771524f613a7b0de354d8fa98a9  EXE SHA256: c437138a08dc10a8609e42d28104920401b4615daa69f3cdf404f6e37d6323dd

NEW RC COMMIT: 8c3fb9f2277966fa98b31bdc775b65c5ea3c29f7  ZIP DIGEST: f8dc02e1f7bbb134e5ecc929c1f567aa265df2d3d1c866fea6636270e52e9472  MCPB archive_sha256: 445d45a6fe18035258f69f04c7538241f19c80a39ad9ad88165b8a7c9f15ac8f  EXE SHA256: 1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59

- **L1 exe:** `C:\bridge-lab\2440-cp\build\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe` (lab 2440-CP, `kept-local.txt`: the bundle under the row folder), only if step 0's inventory lists it with the OLD hash and step 0's reply allows it.
- **L0 exe:** `C:\bridge-lab\2569-r\exe-old\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe` (lab 2569-R), on the grant list.
- **L2 exe:** paired with L0: `C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`, on the grant list, the NEW path 2569-R and 2162-RX used. Paired with L1: a NEW-hash path from step 0's inventory that is not on the grant list, only if step 0's reply names it; none is known, so this pairing will likely not run.

Hash the exe before each launch. A different hash: STOP.

**What the two builds share on this path, read in both trees.** The server starts the helper in `nonce_bound_dialog` (code `src-tauri/src/tally/approved_import.rs:2784-2801` at e0467fbc and at 8c3fb9f2): `tokio::process::Command::new(executable)` with the one argument `--confirm-journal`, standard input and output piped, **standard error to the null device** (`:2800`), `kill_on_drop(true)` (`:2801`), and no creation flag or show state of its own; the call waits 120 s (`:2804`). The post call reaches it through `confirm` (`:2638`) and `confirm_with` (`:2655`); the server's tool dispatch sends `post_import` at `src-tauri/src/agent.rs:3569` (both trees). The helper runs the box by `src-tauri/src/lib.rs:113`. `src-tauri/src/bin/bridge_mcp.rs` has no `windows_subsystem` attribute in either tree, so the exe is a console program (step 0 reads its PE subsystem). `git diff e0467fbc 8c3fb9f2` is empty for `approved_import.rs`, `lib.rs` and `bin/`.

**What differs.** Only NEW's box hook checks for a minimised box: at `HCBT_ACTIVATE` it calls `IsIconic`, and if the box is minimised it calls `ShowWindow(SW_RESTORE)` and writes `box-fit: restored from minimised` to standard error (code `src-tauri/src/tally/box_fit.rs:557-570` at 8c3fb9f2; OLD's hook at `box_fit.rs:547-558` at e0467fbc only measures). So NEW restores a box that is minimised **when Windows activates it**, not one minimised later, until it is activated again; and on this path its restore line goes to the null device (`approved_import.rs:2800`). **Q3's second half cannot be read on this path: the helper's error output is discarded by the server, in both builds.** This row does not read it another way.

## Launches

Each launch runs `client.py`, byte-equal to the client of row 2440-CP and row 2569-R (sha256 `8d3729ea3ab3c5c272ddad1d1c7eb034b9c6fbdb6293b8b9620c190481b969fa`). It starts the server with `subprocess.Popen([EXE], stdin=PIPE, stdout=PIPE, stderr=<file>, env=...)`, no creation flags and no start-up information of its own (`client.py:47`), sends `tools/list` at `client.py:99` and every tool call at `client.py:101`, one at a time, from its inbox. Start it exactly as rows 2440-CP and 2569-R did: a background command of the operator's own shell tool (Git Bash), no window, standard input from the null device:

`cd /c/bridge-lab/2569-s && python -I -X utf8 client.py C:/bridge-lab/2569-s/run-<X> <EXE with forward slashes> <EXE SHA256> < /dev/null`

for example, for L0, `<EXE with forward slashes>` is `C:/bridge-lab/2569-r/exe-old/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe` (row 2569-R's notes record forward-slash paths and standard input from `/dev/null` for its launches). Change nothing in how the client is started between launches; record the command and the shell each time.

**Shell.** Every command in this sheet runs from the operator's own shell tool, Git Bash, as rows 2440-CP and 2569-R ran theirs. Bash strips an unquoted backslash, so every path this sheet writes with backslashes is typed there with forward slashes (`C:/bridge-lab/2569-s/...`). A PowerShell cmdlet named here (`Test-Path`, `New-Item`, `Copy-Item`, `Get-FileHash`, `Get-CimInstance`, `Get-Process`) runs as `powershell -NoProfile -Command "<cmdlet line>"` from the same shell; a helper `.ps1` as `powershell -NoProfile -File C:/bridge-lab/2569-s/<helper>.ps1 ...`. Write no file through a shell `echo` or heredoc.

The three files under `launch\` each hold row 2162-RX's five settings: writes enabled (the build refuses a post otherwise), terms accepted, host `127.0.0.1`, port `9001`, and the launch's own new empty data folder `C:\bridge-lab\2569-s\data-<X>\`. The four forbidden keys are absent; `client.py` refuses them. Copy the launch file byte for byte (`Copy-Item`), never write one through a shell, and check its hash against `SHA256.txt`.

**Inbox rule.** Every call goes into `run-<X>\inbox\` under a name ending `.part` and is then renamed to its own name, after its SHA-256 is checked: the client opens a name ending in .json at once (lab 2162-SALES-Q, deviation 4). Static calls are checked against `SHA256.txt`. The three post calls are written by `postcall.py` straight to `<name>.json.part` and checked against the hash it prints. Each answer is read with `python -I ans.py run-<X>\answers\<name>.json`; its exit 2 is a failed read. No call file other than these is used; no verify call is sent.

No other ComplyEaze Bridge may call port 9001 during a launch, and no raw request is sent while an exe runs. Record the process list before each launch.

## Reading a box (the box frame)

Every box is read by read-only helpers, in this order, and **nobody answers it**: not Yes, not No, not Cancel, not by mouse, key or a scripted click. Each box times out by the build's own rule; the server then ends its helper (`kill_on_drop`, `approved_import.rs:2801`). A minimised box's buttons are off the screen, so no scripted click is safe on it.

- **W. The watch.** `boxwatch.ps1` starts before the post call and waits for the server's child whose command line holds `--confirm-journal`. Its log `reads-<X>\<X>-watch.txt` tells the cases apart: `HELPER pid` (found), `NO_HELPER within 90 s` (exit 1, a result: the process queries succeeded), `CIM_FAILED`/`WATCH_FAILED`, `SERVER_GONE`, `HELPER_GONE` (exit 2, failed reads).
- **R1.** `boxstate.ps1` on the helper at once, again every 2 s up to 10 s while it prints `windows 0` and the helper still runs. The first read that shows a window is the one graded. `R1 NO_WINDOW` means the helper ran 10 s with no top-level window; it is recorded, never read as "not minimised".
- **Q2h.** `startinfo.ps1 -Mode process` on the helper: its start-up block and every top-level window, hidden ones included.
- **R1b.** `boxstate.ps1` again: the same box with no screen act since R1 (a control for elapsed time and for the reads themselves).
- **S1.** When the watch log ends `FIRST READS DONE`, exactly ONE `screenshot` by the screen tool, the whole screen, no zoom, no click, no key. Record the UTC time just before and just after.
- **R2.** `boxstate.ps1` at once after S1.
- **R3.** `boxstate.ps1` 10 s after R2, with no screen act between.
- **R4.** `boxstate.ps1` after the post's final answer, and `Get-Process -Id <helper pid>`: EXPECT `windows 0` and the helper gone.

**From the post call until R3 is saved, nothing else acts on the screen:** no screenshot, zoom, click, key, `request_access` or `list_granted_applications`, and nobody at the PC. Only shell commands that read files or processes. The run coordinator asks the owner, if present, to keep hands off until the launch ends.

**The outcome word for a box**, from the box's line (title `ComplyEaze Bridge — post this voucher?`, class `#32770`) in each read:

- `M0`: R1 `iconic=True`. It opened minimised, with no screen act before it.
- `MB`: R1 `iconic=False`, R1b `iconic=True`. It became minimised with no screen act.
- `M1`: R1 and R1b `iconic=False`, R2 `iconic=True`. It became minimised across the one screenshot.
- `M3`: R1, R1b and R2 `iconic=False`, R3 `iconic=True`.
- `N`: `iconic=False` in R1, R1b, R2 and R3.
- `none`: no box read (`NO_HELPER` or `R1 NO_WINDOW`).
- `failed`: a read in the frame failed (exit 2). It is not graded; it is not `N`.

Record beside the word, from each read: `visible`, `topmost`, the rectangle, `visual_state`, `foreground` and the FOCUS line. A box that goes back from minimised to not minimised between reads is written down with the reads it happened between.

## Reading start-up information (Q2)

`startinfo.ps1` loads `startinfo.cs` with `Add-Type` (Windows PowerShell 5.1, nothing installed) and reads a running process's own parameters block (`RTL_USER_PROCESS_PARAMETERS`, through the PEB), where Windows keeps what the process was started with:

- `window_flags`: the STARTUPINFO `dwFlags`, named bit by bit (`USESHOWWINDOW`, `USESTDHANDLES`, ...);
- `show_window_flags`: `wShowWindow`, with its `SW_` name, and whether it is used for the first show (only when `USESHOWWINDOW` is set);
- the starting position, size and title fields; `console_handle`, `console_flags` and the three standard handles; the image path and command line;
- the image's PE subsystem (2 GUI, 3 console) and the parent pid, checked against pid reuse by creation time.

Every read prints one RESULT line: `OK`, or `OPEN_FAILED` (87 the pid is not running, 5 access denied), `QUERY_FAILED`, `READ_FAILED`, `PARTIAL_READ`, `WOW64_TARGET`, `HOST_NOT_64BIT`, `LAYOUT_CHECK_FAILED` (the block's image path differs from the one Windows reports for the process, so the offsets are wrong on this Windows), or `PARENT_UNREADABLE`. Its self-test (step 0) proves the offsets on the lab's own Windows: its own process against `GetStartupInfoW` and `GetCommandLineW`, then two `ping.exe` children it starts with known values, each read and left to exit by itself.

**What cannot be read this way, and is not claimed.** The creation flags given to `CreateProcess` (for example `CREATE_NO_WINDOW`, `DETACHED_PROCESS`, `CREATE_NEW_CONSOLE`) are not stored where a reader can reach them after the start; only their effects are, which this row reads: the console handle in the block and, by `-Mode console`, whether the process has a console and whether that console has a window. The `console_handle` value read after the start is the console connection the process holds, not the marker its parent passed. The helper's error output is not readable on this path (above). What Windows did with `wShowWindow` at the first show is inferred from the box reads, not read.

Chain reads (`-Mode chain`) print the server and its parent in full; for higher ancestors (the shells and the operator's tools) they print only the image's file name and the lengths of the command line, folder and title, because those texts may hold user paths or session values.

## Bounded output

`boxstate.ps1` prints one line per window of one process. `startinfo.ps1` caps the command line at 300 characters, paths at 260, titles at 120 (a window title at 60), lists at most 20 windows per process with the count omitted, and at most 6 ancestors. `boxwatch.ps1` writes one log line per event and per read, never per poll. `exes.ps1` prints at most 40 files and 5 errors, each with the count omitted. `ans.py` prints at most 10 companies. `count8.py`, `run.py` and `client.py` keep their previous bounds. No output is piped through a truncating reader.

## What stays on the lab PC

As in row 2569-R, keep these local and list each in `kept-local.txt` by name, bytes and SHA-256: BK1 and every backup or restore listing (no company number quoted in the notes); `restore-plan.txt`; every saved Bridge answer and server error log until scanned (evidence copies replace `company_number` with `redacted`); every screenshot (S1 included); every `startinfo.ps1` output until scanned (evidence copies replace a path outside `C:\bridge-lab` with `<path>`); any file the pre-evidence scan finds holding a company number, a user name, a home path or data outside the synthetic lab family.

## Files

Byte-equal copies from row 2569-R's sheet (`lab/2569-restore-sheet` at 208932f7, folder `lab/2569-restore/`): `.gitattributes`, `LAB-COMMON.md`, `client.py`, `boxstate.ps1`, `run.py`, `count8.py`, `M-marks.xml`, `O-company.xml`, `I-seed.xml`, `K-customer.xml`, `K-cgst.xml`, `K-sgst.xml`, `K-sales.xml`, `F-0101.xml`.

Byte-equal copies of row 2569-R's Part K calls (`lab/2569-restore-answers` at 400575f3, `run-k/inbox/`), renamed: `calls\01-list.json` (its `000`), `calls\02-list_companies.json` (`001`), `calls\03-marks.json` and `calls\08-marks.json` (both `002`), `calls\04-build-a.json` (`003`: row 2440-CP's invoice A, `CP/25-26/0101`).

New for this row:

- `startinfo.cs`, `startinfo.ps1`: the start-up reader above.
- `boxwatch.ps1`: the watch, R1, Q2h and R1b.
- `exes.ps1`: every `bridge_mcp.exe` under a folder, by hash.
- `postcall.py`: one post call from a build answer, in the bytes row 2569-R's post calls had (checked on macOS: its output for row 2569-R's build A equals that row's `006-s8-post-a-1.json` byte for byte).
- `ans.py`: the graded lines of one answer.
- `launch\L1-launch-env.json`, `L0-`, `L2-`: row 2162-RX's settings, data folders under this row's folder.
- `check_sheet.py`, `RUN-SHEET.md`, `SHA256.txt` (sha256, size, path).

At the sitting `run.py` writes `a-*` answers and `log-*.txt`; each launch writes `run-<X>`, `data-<X>` and `reads-<X>`; the post calls are written by `postcall.py`; C5 writes BK1. These are outputs, not missing inputs.

## Self-checks before the sitting

Run from `C:\bridge-lab\2569-s`. The interpreter is `python -I` (3.10).

- `python -I check_sheet.py`: EXPECT the three OK lines, exit 0.
- `python -I -c "import ast,sys;[ast.parse(open(f,encoding='ascii').read()) for f in sys.argv[1:]]" ans.py postcall.py check_sheet.py client.py run.py count8.py`: EXPECT no output, exit 0.
- `[System.Management.Automation.Language.Parser]::ParseFile` on `boxstate.ps1`, `boxwatch.ps1`, `startinfo.ps1`, `exes.ps1`: EXPECT 0 errors each.
- `powershell -NoProfile -File startinfo.ps1 -Mode selftest -Out evidence\selftest-startinfo.txt`: EXPECT `parse cases 10, wrong 0`, `case self: right`, `case child-a ...: right`, `case child-b ...: right`, each child `exited code 0`, `selftest wrong 0`, exit 0. A child `NOT EXITED within 30 s` is left running, recorded and a STOP. Any WRONG, a `RESULT` other than OK, or `HOST_NOT_64BIT`: STOP before step 1 and report the file; the reader is the run coordinator's to fix by a new commit.
- No Tally request, launch or restore is part of these checks.

## Steps

Raw reads use `python -I run.py http://127.0.0.1:9001 <label> <files>` (it sends by `curl.exe` at `run.py:22`), then `python -I count8.py` on each answer. Record each whole output. Keys and prompts follow C7; any dialog not named: screenshot, copy its text, press no key, STOP. **Path rule:** the screen after each key must be the one named, otherwise screenshot and STOP with no further key. Never Escape at Select Company or the Gateway. Never end a process.

0. **Before the sitting.**
   - Copy the folder byte for byte to `C:\bridge-lab\2569-s`; make new `evidence\` (`Test-Path` first, EXPECT `False`); run every self-check above. `run-*`, `data-*`, `reads-*` and `backups` must be absent until their step makes them.
   - C2 step 0 with no company loaded: EXPECT the screenshot to show Select Company with no company loaded and no dialog, one tally.exe owning 9001, GET /status 200. A company loaded or any dialog: STOP.
   - `powershell -NoProfile -File boxstate.ps1 -ProcId <tally pid> -Out evidence\s0-boxstate-tally.txt`: EXPECT one WINDOW line for Tally's window, `iconic=False`, and a FOREGROUND and a FOCUS line (lab 2569-R).
   - `powershell -NoProfile -File exes.ps1 -Root C:\bridge-lab -Out evidence\s0-exes.txt`: record every line. EXPECT exit 0, an `EXE OLD` line for L0's path, and whether L1's path is listed `EXE OLD`. Exit 3 (some folder unreadable): a path not listed is unknown, not absent.
   - `list_granted_applications`, with no Bridge box open: record every entry and path. EXPECT row 2569-R's `exe-old` and `exe-new` paths on the list, and row 2440-CP's folder not on it (lab 2569-R: grants named those two paths; a restart of the desktop app drops grants). `exe-old` missing: L0 is then not the granted control; say so in the step 0 reply and L0 runs only on its word. 2440-CP's path on the list: L1 is then not the ungranted launch; say so and L1 runs only on the reply's word.
   - `powershell -NoProfile -File startinfo.ps1 -Mode process -ProcId <tally pid> -Out evidence\s0-startinfo-tally.txt`: a live read of a process this row does not start. EXPECT `RESULT OK`. Any other RESULT: STOP.
   - Process list: `Get-CimInstance Win32_Process` for `bridge_mcp.exe`, `python.exe`; EXPECT no `bridge_mcp.exe`.
   - **Step 0 reply.** Send the run coordinator `evidence\s0-exes.txt`, the grant list, and two questions: (a) may L1 run the OLD exe from a path not on the grant list (row 2440-CP's folder, or another `EXE OLD` path the inventory lists), and from which (nobody clicks its box; this sheet's build rule otherwise names granted paths only); (b) if the inventory lists a NEW-hash exe at a path not on the grant list, may L2 use it when paired with L1, and which. Wait for the reply. The reply is a new commit on this sheet's branch that fills the two lines below (fetch it, check the folder against its `SHA256.txt` again and run the self-checks again), or a message that answers no to both; a ruling that arrives any other way is not used. No reply: do not start.

     L1 RULING: `<to fill>`
     L2 PATH: `<to fill>`

     Each line holds `no` or the exact exe path allowed. While L1 RULING reads `<to fill>` or `no`, L1 does not run; while L2 PATH reads `<to fill>` or `no`, L2 does not run when paired with L1. L0, and L2 paired with L0 at `exe-new`, run either way once a reply has come.
1. **Load and read** (row 2569-R's Part A step 1, keys from its notes). Load BRIDGE EINVOICE LAB alone from Select Company: click the filter; type `BRIDGE`, space, `EINVOICE`, space, `LAB`, a zoom after each part, until it is the one highlighted line; Enter. EXPECT the Gateway, period 1-Apr-25 to 31-Mar-26, current date 3-Apr-2025, no dialog. Send `s1`: `M-marks.xml O-company.xml K-sales.xml K-customer.xml K-cgst.xml K-sgst.xml`. EXPECT, as row 2569-R's `s1` (lab 2569-R): M one COMPANY row, `GUID is the book's: yes`, ALTVCHID 3, ALTMSTID 215; O the book's GUID, `ISGSTCLASSIFON` No, one `CMPGSTDETAILS.LIST` from 20250401 with `HSNSOURCEPRIORITY` Ledger and `HSNAPPLICABLESECTION` All Sections; K-sales `BRIDGE IRN Sales`, IGST 18, HSN 998313; K-customer, K-cgst, K-sgst 0 LEDGER rows each. Otherwise STOP before any write.
2. **Back up BK1** (C5, with the keys row 2162-RX's sheet writes out, and the company filter of row 2569-R's notes). `Test-Path C:\bridge-lab\2569-s\backups\BK1`: EXPECT `False`; make it with `New-Item -ItemType Directory`. Click the Gateway's left panel; Alt+Y: EXPECT the Data menu; Enter (Backup & Restore); Enter (Backup): EXPECT the Backup screen. In Backup Data Path press Backspace: EXPECT the List of Folders; Up three times, one at a time with a zoom after each, to `Specify Path` highlighted; Enter; type `C:\bridge-lab\2569-s\backups\BK1` (zoom: exact); Enter: EXPECT the list with that folder highlighted; Enter: EXPECT the path to read that folder. In the company filter type `BRIDGE`, space, `EINVOICE`: EXPECT one line, BRIDGE EINVOICE LAB, highlighted; Enter; click `End of List` (zoom: highlighted); Enter. EXPECT `Accept ?` showing that path and the one company: screenshot, click inside the box, `y`. EXPECT the Gateway and no message. A line, list or prompt other than these: path rule. EXPECT exactly one file `TDBK1800_<company number>.001` in the folder: record its bytes, SHA-256 and the Restore-list time. Send `s1b`: `M-marks.xml`; `python -I count8.py --same-row a-s1-M-marks.xml a-s1b-M-marks.xml`: EXPECT pass.
3. **The customer, seeded.** Send `s3m`: `M-marks.xml`; `count8.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`: EXPECT pass. Send `s3`: `I-seed.xml`, once. EXPECT CREATED 1, every other counter 0, no LINEERROR, no dialog on screen. Send `s3r`: `K-customer.xml M-marks.xml`. EXPECT one LEDGER row `BRIDGE CP Customer MH`, PARENT Sundry Debtors, `ISBILLWISEON` No, one GSTIN, one `LEDGSTREGDETAILS.LIST` from 20250401, Regular, Maharashtra; ALTVCHID 3, ALTMSTID 216 (lab 2569-R). Otherwise STOP; the book then holds the seed only.
4. **The two duty ledgers, keyed.** Send `s4m`: `M-marks.xml`; `--before-write` against `a-s1-M-marks.xml`: EXPECT pass. At the Gateway: Create (Enter), Down to Ledger, Enter: EXPECT Ledger Creation with Under already Duties & Taxes. Name `BRIDGE CP CGST` typed in parts; alias Enter; Under Enter; Type of Duty/Tax list Up to GST, Enter; Tax Type list Down to `CGST`, Enter; rate `9`, Enter; Rounding list `Not Applicable`, Enter. Screenshot and write down every field before the save. Ctrl+A: EXPECT no prompt and a new blank Ledger Creation screen. Then `BRIDGE CP SGST` the same, with the Tax Type list Down twice to `SGST/UTGST`. Then Ctrl+Q (Master Creation), Ctrl+Q (the Gateway), each after a capture of the screen's title. A Tax Type list other than IGST, CGST, SGST/UTGST, Cess, or a field not named here: no save, STOP. Send `s4r`: `K-cgst.xml K-sgst.xml M-marks.xml`. EXPECT each one LEDGER row, PARENT Duties & Taxes, `TAXTYPE` GST, `GSTDUTYHEAD` CGST and SGST/UTGST, `RATEOFTAXCALCULATION` 9, `ROUNDINGMETHOD` Not Applicable, `ROUNDINGLIMIT` 0; ALTVCHID 3, ALTMSTID 218 (lab 2569-R).
5. **The Sales type, keyed.** Send `s5m`: `M-marks.xml`; `--before-write` against `a-s1-M-marks.xml`: EXPECT pass. At the Gateway: Create, Down three times to Voucher Type, Enter. Name `BRIDGE CP Sales` in parts; alias Enter; type `Sales` into the type list's filter (EXPECT Sales and Sales Order), Enter on Sales; Abbreviation as filled, Enter; Activate Yes, Enter; Method list Down twice to `Manual`, Enter; `Prevent creating duplicate Voucher Nos.` showing No: `y`, Enter. Screenshot and write down every field. Ctrl+A: EXPECT no prompt and a new blank Voucher Type Creation screen; Ctrl+Q, Ctrl+Q to the Gateway, each after a capture. Send `s5r`: `M-marks.xml`: EXPECT the book's GUID, ALTVCHID 3, ALTMSTID 220 (lab 2569-R). The book stays loaded at the Gateway. The book now: the masters of steps 3 to 5; no voucher.

**The launch frame**, for launch `<X>` (L1, L0 or L2) with its exe path and line:

- a. `Test-Path` `run-<X>`, `data-<X>`, `reads-<X>` under `C:\bridge-lab\2569-s`: EXPECT `False` each; make `run-<X>\inbox`, `run-<X>\answers`, `data-<X>`, `reads-<X>`. Copy `launch\<X>-launch-env.json` to `run-<X>\launch-env.json`; EXPECT its SHA-256 equal to `SHA256.txt`. Hash the exe: EXPECT its line.
- b. C2 step 0 with the book loaded (its MARKS read is `pre-<X>`: `M-marks.xml`; `count8.py --same-row a-s5r-M-marks.xml a-pre-<X>-M-marks.xml` must pass). Process list: EXPECT no `bridge_mcp.exe`.
- c. Start the client (**Launches**). EXPECT `client-log.txt` within 60 s to show the exe hash ok, the launch file's hash, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` with version 0.7.0. A `stop:` line or no initialization: STOP.
- d. `calls\01-list.json`, `02-list_companies.json`, `03-marks.json`, one at a time. EXPECT `01` the five tools present; `02` one company, `guid_is_book True`; `03` `master_alter_id 220 voucher_alter_id 3` (lab 2569-R, Part K step 3).
- e. **Chain read**, no box open: `powershell -NoProfile -File startinfo.ps1 -Mode chain -ProcId <server pid from run-<X>\server.pid> -Out reads-<X>\<X>-chain.txt`. EXPECT exit 0, `RESULT OK` for the server and for its parent `python.exe`; an ancestor's failed read is recorded, not a STOP. EXPECT (unverified; Python's `subprocess` sets `STARTF_USESTDHANDLES` when it redirects and sets no show state unless given one) the server's `window_flags` to hold `USESTDHANDLES` and not `USESHOWWINDOW`, and `pe_subsystem=3 (WINDOWS_CUI)`.
- f. `calls\04-build-a.json`. EXPECT `voucher_count 1`, a `batch_id`, `hsn_warnings 0` (lab 2569-R, Part K step 4). A refusal: record it whole, close (h), and the launch has no box.
- g. **The post, never answered.**
  - Start the watch as a background command of the operator's shell tool: `powershell -NoProfile -File C:/bridge-lab/2569-s/boxwatch.ps1 -ServerPid <server pid> -Dir C:/bridge-lab/2569-s/reads-<X> -Tag <X> -WaitS 90 < /dev/null`. EXPECT `reads-<X>\<X>-watch.txt` to show `watch start`.
  - `python -I postcall.py run-<X>\answers\04-build-a.json run-<X>\inbox\05-post-1.json.part`; check the hash it printed; rename to `05-post-1.json`. From here the box frame holds: no screen act until R3 is saved.
  - When the watch log ends `FIRST READS DONE`: S1, R2, R3 (**Reading a box**). If the watch ended with exit 1 or 2: no S1; record its log.
  - When `05` is answered: EXPECT `approval state pending`, `retry_after_s 15`, `attempt_recorded False`, `dispatch state not_dispatched` (lab 2440-CP; lab 2569-R). Wait `retry_after_s` seconds after the answer's log line, then `06-post-2.json` by `postcall.py` in the same way. While an answer reads `pending`, the next after `retry_after_s`: `07-post-3.json`. EXPECT the last `isError True`, `error_code import_approval_timed_out`, `attempt_recorded False` (lab 2440-CP, `008-s8-post-a-3`). No further post call is made. An answer reading `posted_` or a `dispatch state` other than `not_dispatched`: STOP (a post happened); close and go to step 9 on the run coordinator's word.
  - R4.
- h. `calls\08-marks.json`: EXPECT `master_alter_id 220 voucher_alter_id 3`. Then `powershell -NoProfile -File startinfo.ps1 -Mode console -ProcId <server pid> -Out reads-<X>\<X>-console.txt`: record the line (`attached window=...`, or `NO_CONSOLE`). Then close: create `run-<X>\inbox\stop`. EXPECT the log to end `session ended; server pid <pid> exit code 0`, that pid gone, no `bridge_mcp.exe` left. A server that does not end: STOP and report; never end it.
- i. Send `after-<X>`: `M-marks.xml F-0101.xml`. `count8.py --same-row a-s5r-M-marks.xml a-after-<X>-M-marks.xml`: EXPECT pass; F-0101: EXPECT 0 VOUCHER rows. A moved mark or a voucher row: STOP.
- j. Record the box's outcome word, and from `<X>-Q2h.txt` the helper's `RESULT`, `window_flags`, `show_window_flags`, `console_handle`, the three standard handles, `pe_subsystem`, its parent pid (EXPECT the server's) and its windows; from `<X>-chain.txt` the same for the server and its parent.

6. **L1: OLD from row 2440-CP's folder.** Only if L1 RULING names a path and step 0 listed that path `EXE OLD`; otherwise write `L1 not run` and why, and go to step 7. Run the launch frame with L1's exe. EXPECT (not shown): the outcome word. Lab 2440-CP saw the box minimised at its first screenshot, twice.
7. **L0: OLD from `exe-old`.** Run the launch frame with L0's exe. EXPECT (not shown): the outcome word. Under the #2161 record's finding 5 a box on the grant list is not hidden by the screenshot, so `M1` here would contradict it.
8. **L2: NEW at its pair's grant state.** Pair it with the OLD launch whose word was `M0`, `MB` or `M1` (L1 first, when L2 PATH names a path step 0 listed `EXE NEW` and the grant list does not hold; else L0, with `exe-new`, which step 0 found on the grant list). No minimised OLD box, or no path for its pair: write `L2 not run` and why. Run the launch frame with L2's exe and record which launch it is paired with. EXPECT (code `box_fit.rs:557-570` at 8c3fb9f2; not shown live): if the pair was `M0`, R1 `iconic=False` (the box restored when it was activated); if the pair was `M1` or `MB`, the read after the act that minimised the OLD box `iconic=True` again, the hook not restoring a box minimised after its activation. The restore line is not readable (`approved_import.rs:2800`).
9. **Restore and close.**
   - On the run coordinator's explicit RESTORE, restore BK1 by C6, as row 2569-R's step 12 did: re-hash BK1 (one file, the recorded bytes and SHA-256); write `restore-plan.txt` and the data-folder listing; Shut Company (the list shows BRIDGE EINVOICE LAB alone; Enter); Data, Backup & Restore, Restore; Restore Data Path by Specify Path, the BK1 folder typed in parts; EXPECT the list to show BRIDGE EINVOICE LAB at the time recorded at step 2; Enter; End of List; `Accept ?` captured, `y`; the "already exists ... Replace" prompt, `r`; EXPECT Select Company and no other prompt. A prompt not named here: STOP.
   - Load the book alone as at step 1. Send `s12`: `M-marks.xml K-customer.xml F-0101.xml`. `python -I count8.py --same-row a-s1-M-marks.xml a-s12-M-marks.xml`: EXPECT pass (ALTVCHID 3, ALTMSTID 215); K-customer 0 LEDGER rows; F-0101 0 VOUCHER rows. Otherwise STOP and report what the book holds.
   - C10: Shut Company, one tally.exe owning 9001, GET /status 200, no company loaded, current date as at step 0, no `bridge_mcp.exe` running. Send no XML after the shut.

## Record

- Every raw answer whole with bytes and SHA-256, every `run.py` log, every `count8.py` and `ans.py` output whole.
- For each launch: folder checks, exe path and hash, launch-file hash, the start command and shell, the process list, the whole `client-log.txt`, every answer under the kept-local rule, the server's error log size, and every file under `reads-<X>`.
- For each box: the outcome word, the UTC times of the helper's creation, R1, R1b, S1 (before and after), R2, R3, the final answer and R4; and every field of the box's line in each read.
- A one-line table: launch; exe path and whether it was on the grant list; outcome word; R1, R1b, R2, R3 `iconic`; the helper's `window_flags`, `show_window_flags`, `console_handle`, `pe_subsystem`; the server's `window_flags`; the console line; final post answer; marks before and after; or why it was not run.

## Decides

One book, one release, one Windows account, one sitting: every reading is **partial** at best, and one box per launch is one sample.

- **L1 `M0`:** the old build's box opened minimised on the 19/20 Nov path with no screen act before it: the failure is the program's or its launch's, seen without the screen tool. Q2's reads then show what show state the helper had (`show_window_flags` with `USESHOWWINDOW`, or none). **L1 `M1`:** the box opened normally and was minimised across one screenshot by the screen tool: the boxes of 19 and 20 Nov are explained by the act of looking, not by the build's launch (partial: one sample; consistent with the #2161 record's finding 5). **L1 `MB` or `M3`:** minimised without the screenshot as its last act before; recorded as it stands, the cause not shown. **L1 `N`:** the failure was not reproduced on its own path; its cause stays not shown, and the grant and the screenshot are not shown to cause it in this sitting.
- **L0 `N` beside L1 `M1`:** the grant decides whether the screenshot minimises the box (partial). **L0 `N` beside L1 `M0`:** the path or its grant changes how the box first shows; the build's bytes are the same, so this points to the screen tool's handling of an app outside its grant, not to the exe. **L0 equal to L1:** the grant is not the variable here. **L0 `M1`:** the screenshot minimises even a granted box; that contradicts the #2161 record's reading and is reported.
- **Q2, on every launch read:** the `window_flags` and `show_window_flags` the helper and server hold are **verified** for this sitting once the self-test passed. `USESHOWWINDOW` clear on the helper means the helper was given no show state; a minimised box then did not come from its STARTUPINFO. `USESHOWWINDOW` set with a minimising `SW_` value names the show state it was given. L1 and L0 equal in these fields means the path change did not change what the processes received. The console line says whether the server has a console and whether that console has a window; creation flags themselves are not read.
- **L2, after its pair's `M0`:** `iconic=False` at R1 means the fix restored a box that opened minimised on this path (partial; the restore line cannot be read here); `iconic=True` at R1 means it did not (#2569's FAIL on this path). **L2, after its pair's `M1` or `MB`:** `iconic=True` at R2 means the fix does not cover a box minimised after its activation, as its code reads; `iconic=False` at R2 means something activated the box again or the screenshot did not minimise it this time; the two are not told apart. **L2 not run:** Q3 is answered from the code only (unverified live).
- **A `none` or `failed` word:** that launch decides nothing about the box; its Q2 reads still stand if their RESULT is OK.
- **Marks moved, a voucher row, or a post answer `posted_`:** the row touched more than its design permits; STOP and report; no reading above is made from that launch.

## Not covered by this row

- The helper's error output on the client path (discarded by the server in both builds), so the restore line.
- Creation flags, read directly (only their effects are read).
- Other MCP clients, a server started by the desktop app, other Windows versions or accounts, other boxes (review, one-ledger create, releases, settle).
- A box the person answers late, or one that does not fit the screen.
- The cause of a box minimised by the screen tool inside that tool: this row reads only that it happens or not.

## Close and evidence

C10 at step 9 leaves one Tally with no company loaded. The synthetic book stays on the lab PC. C11 applies: evidence goes to a new branch `lab/2569-s-answers` only after the pre-push scan and the run coordinator's yes. Its RESULTS gives Q1, Q2 and Q3 first, each with its confidence, then L1, L0 and L2 in order with every read, the restore, and each launch not run with its reason. Quote only grading lines from kept-local files.
