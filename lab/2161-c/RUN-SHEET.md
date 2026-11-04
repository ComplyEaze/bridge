# Windows item 2161-C v1: on master's Windows bundle, does the product's approval box get a taskbar button, and is it listed in Alt+Tab? Read three ways, with one Alt+Tab per cell, every box answered No by mouse. (Part of #2161)

WRITES to the book: NONE. **No ComplyEaze Bridge server is launched.**

The only requests to Tally are `LAB-COMMON.md`'s own read-only checks:
- at the start, GET /status and the MARKS read of C2 (c, d);
- just before the company is shut, the MARKS read again;
- after the shut, GET /status only (C10: no XML request after the shut).

Every box comes from the product's own dialog child, fed a fixed text, so with no server nothing can post. Each box is answered **No by mouse**.

**No taskbar button is clicked: the taskbar is only read.** The only keys pressed after B2 are Alt, Tab and Esc in step HA, under its carve-out. P0's load of the company and C10's shut use the inputs `LAB-COMMON.md` names, and are recorded.

## Why
- **Rows 2161-A and 2161-B** (`lab/2161-taskbar-activate-b-answers`) asked what a taskbar activation does to the box. The band over the task list was the lab's own console. One activation of a box above Tally did no harm (C1). The other two questions were not measured: the box in front (C2), and "Close window" from the box's taskbar menu (C3).
- **Since then, #2193** (merged as c1015563) owns each Windows approval box by a hidden topmost tool window: `TopmostOwner` in `src-tauri/src/tally/box_fit.rs` (`CreateWindowExW(WS_EX_TOPMOST | WS_EX_TOOLWINDOW, "Static", ...)`, never shown), passed as each box's owner by `guarded`. An owned window is expected to get no taskbar button. If so, C2 and C3 cannot arise on master. Row 1780-R6-bis (`lab/1780-r6bis-hidden-owner` @ 8fd27158) was written to read this and has not run.
- **The one keyboard route back** to a box with no button would be Alt+Tab. Windows may list an owned window whose owner is hidden, or may not. This is not measured.
- **What this row decides (the requester's ruling):**
  - **Q1.** With a box open, is a taskbar button added for it? Read three ways against the same cell with no box: UI Automation's button list, the windows over the taskbar (the cover read), and the taskbar drawn by `PrintWindow`.
  - **Q2.** Is the box listed in Alt+Tab's switcher? Read by eye, by UI Automation, and from a screen copy taken while the switcher is held.
  - If no button shows and the box is not in Alt+Tab, #2161 closes on #2193. If either shows, the taskbar cells of rows 2161-A and 2161-B come back on this build.

## Who acts
- The lab does every step on this synthetic company, on the owner's standing word of 27 Oct 2026: the title clicks, the keys of step HA and every No. Record each as "done by the lab on the owner's standing word of 27 Oct".
- **Downloading** the bundle needs the owner's OK in the lab chat.
- Posting is not switched on.
- The go-ahead and any STOP ruling come from the run coordinator. Step HA is run on the coordinator's yes; otherwise write "HA not run".

## Build and inputs (hash before the first step and at the end; a different hash = STOP; `SHA256.txt` lists every file of this folder with its size)
- **BUILD-M:** the `unsigned-bundle-smoke-windows-latest` artifact of CI run 41772223948, the merge-queue run of #2329 at 187e882b5176a3b577d57f257a6ec4319fc2aecd. That commit is master's head at this sheet's writing, and it holds #2193.
  - The run's conclusion is success, its Windows jobs included (both native shards, bundle smoke, seam control).
  - Artifact id 13295846440, 59,336,955 bytes, GitHub digest `sha256:1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a`; it expires on 11 Nov 2026. Record each again as fetched.
  - EXPECT:
    - the zip's sha256 equals the digest;
    - `bridge_mcp.exe` and `pdfium.dll` match `mcpb-smoke.json`'s hashes.
  - Extract the WHOLE `.mcpb` into `C:\bridge-lab\2161c\build\` (pdfium.dll beside the exe). A mismatch = STOP.
- **Byte copies:**
  - `requests/box-trace-early.ps1`, `zorder-probe-k.ps1`, `spawn-parent.ps1` and `confirm-input.txt` from `lab/1780-system-modal-box` @ 693a04f5258cb14592b197b966de5aa1e60c0572;
  - `requests/box-rect-probe.ps1`, `taskbar-capture.ps1` and `taskbar-cover.ps1` from `lab/2161-taskbar-activate-b` @ 996f869ffa4eec7c4a9be53a27b89ea50d1b57b0.

  All of them ran in rows 1780-H, 2161-A and 2161-B and are kept byte for byte, so their hashes hold. Some headers still say "NOT RUN"; that predates those rows. `confirm-input.txt` holds a nonce line, the count 1 and a 23-line box text with zero GUID and amounts. The child prints a token only on Yes; with no server, nothing can post.
- **New, read-only: `requests/taskbar-uia.ps1`.** `taskbar-uia.ps1 -Out <file>` lists every Button element under the taskbar (`Shell_TrayWnd`) as UI Automation finds them: a `B` line each, with its parent's class (`MSTaskListWClass` for a running program's button on Windows 10), class, rectangle and name, then a `Z` line with the count and the count in the task list. It is per-monitor DPI aware and invokes no pattern. Its C# compiled with Mono; the PowerShell has not run, so B2 checks it first.
- **New, read-only: `requests/switcher-watch.ps1`.** `switcher-watch.ps1 -Name <cell> -OutDir <shots> -Out <file> -Seconds <n>` is started before a cell. It polls every 50 ms for the switcher, a visible, uncloaked window of class `MultitaskingViewFrame` on Windows 10. When the switcher first shows, it writes:
  - an `A` line with the foreground;
  - about 300 ms later, one whole-screen copy, `<cell>-switcher.png` (a `C` line);
  - the switcher's tiles by UI Automation: `L` lines, one per ListItem, or `D` lines (every named element, at most 40) when there is none, or a `U` line if the read fails.

  When the switcher goes, it writes a `G` line with the foreground at that poll, and an `F` line 500 ms later. It also writes an `N` line, once, for each visible top-level window that was not there when it started (at most 100). It sends no key and changes no window. Its C# compiled with Mono; the PowerShell has not run, so cell N checks it first.
- **Folders and company.**
  - Working folder `C:\bridge-lab\2161c\`; evidence in `C:\bridge-lab\2161c\evidence\`; captures and copies in `C:\bridge-lab\2161c\shots\`.
  - THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, loaded alone, Tally maximised on the Gateway of Tally.
  - The `.ps1` files run from Windows PowerShell 5.1 as `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...`. A comma is a digit separator on this PC: no command takes a list.
- **Coordinates.** `PrintWindow` images are taskbar-relative: add the `T` line's rectangle, as in row 2161-B.

### The cell runner
The cell runner is the lab's own, as in row 2161-B. It is committed with the answers under `scripts/`, and its sha256 goes into `notes.txt` before the first cell.
- **Names.** Every point it clicks has its own variable, and no two variable names in the runner differ only in letter case (`$tallyTitlePoint`, `$noButtonPoint`). B1 checks this.
- **Each click records**, in the cell's timeline: the point's name, its logical and physical coordinates, and about 300 ms later the foreground window (hwnd, program name, title).
- **The read set X**, inline in the runner's own console, one after another:
  1. `box-rect-probe.ps1 -Out <evidence>\rect-<cell>.txt`;
  2. `taskbar-uia.ps1 -Out <evidence>\uia-<cell>.txt`;
  3. `taskbar-cover.ps1 -Out <evidence>\cover-<cell>.txt`;
  4. `taskbar-capture.ps1 -Name <cell> -OutDir C:\bridge-lab\2161c\shots -Out <evidence>\captures.txt`.
- **The gate probe G-P** is one `zorder-probe-k.ps1`, its output kept as `<evidence>\probe-<cell>-<label>.txt`.
- **Never the screen tool while a box is open.** Row 2161-A saw it minimise an open box. Copies while a box is open come only from the scripts.
- **Recorded consoles.** The runner records the pids of its own console, the trace's console, the watcher's console and the cell's `.cmd` console when it starts them. The trace and the watcher are started minimised (`Start-Process ... -WindowStyle Minimized`), and the `.cmd` console is minimised during its 8 s sleep, the P-min way of row 2161-B.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS.**
  - No key is pressed in Tally from B2 until C10, except step HA's.
  - No key is pressed anywhere from a cell's start until its box is answered or gone, except step HA's.
  - Keys are used only in the lab's own console, to start the runner before a cell starts.
  - Boxes are answered by mouse at the No button's centre.
- **ALT+TAB carve-out (named, step HA only, on the run coordinator's yes).**
  - The keys allowed are Alt, Tab and Esc, in the order step HA gives, by the lab's own input tool, and only while the gate probe just before shows Tally holding the foreground.
  - Never Enter, Space, Y, N or any other key, and never a second Tab.
  - None of these keys can answer the box Yes. An Esc that reached the box would be its Cancel, a decline.
  - **Why Esc and not a release.** One Tab moves the highlight off the window in front, to the next window in the switcher's order. Releasing Alt there would switch to that window, which could be the box. With Alt still held, Esc closes the switcher with no switch, so the original window keeps the foreground. That is how this row "releases on the original window".
- **C2 carve-out (named).** `LAB-COMMON.md` C2 forbids ending any process. `spawn-parent.ps1` ends exactly one: the child it started itself, by the pid and handle its own `CreateProcess` returned, if it is still running after `-TimeoutSeconds` (120 in this row; the product's own limit is also 120 s). Nothing else is ended. A box or process left after the sitting is recorded and left.
- **Minimising a recorded console (named).** The runner may minimise a window with `ShowWindow(SW_MINIMIZE)`, by its own handle, only when all of these hold:
  - the window is a recorded console's top-level window;
  - it is not the foreground window at that moment;
  - it is minimised before T0;
  - it is at most once per console per cell;
  - it is recorded.

  Nothing else is minimised, except Tally under "A box under Tally".
- **A box under Tally.** If a box lies under Tally and must be answered, the runner:
  1. minimises Tally with `ShowWindow(SW_MINIMIZE)` by its handle;
  2. clicks No by mouse;
  3. restores Tally with `ShowWindow(SW_RESTORE)`.

  This is row 2161-B's way, so no click is made on Tally's buttons or the taskbar. Record each step.
- **No taskbar click, no hover on the taskbar, and no click in the switcher.**
- **One cell at a time.** A cell starts only after the previous cell's trace has written its `END` line and its watcher its `Z` line.
- **Leftover list (B5)**, before the sitting, after C1, and at the end:
  - processes whose command line contains `--confirm-journal`, `spawn-parent.ps1`, `box-trace-early.ps1` or `switcher-watch.ps1`;
  - visible windows whose title contains "ComplyEaze Bridge";
  - minimised recorded consoles still open.
  - EXPECT none. Any: record, end nothing, STOP.
- **Privacy.** Before committing evidence, search it for the user's name and for paths outside the lab folders. Replace each by `<user>` or `<path>`, and record the edit in `substitutions.txt`. That includes:
  - the `name=` of `uia-*.txt` and the `L`, `D` and `N` lines, which name whatever windows are open;
  - `cover-*.txt` titles and every foreground title.

  A window of the owner's own (a browser, a document, the desktop client) is named by its program and `<other window n>`, the same n in every file. Images stay local, with their name, size and sha256 in `SCREENSHOTS.txt`.

## Before the sitting
- **P0.** `LAB-COMMON.md` C1 and C2: BRIDGE BATCH LAB is in Select Company's list and loaded alone, else STOP (do not restore it, do not use another book). Record the inputs used to load it, and ALTVCHID and ALTMSTID from C2's MARKS read.
- **B1.** Check:
  - BUILD-M, as above;
  - every file of this folder, hashed against `SHA256.txt`;
  - the runner's sha256, recorded;
  - a case-insensitive scan of the runner for any variable name spelled two ways. EXPECT none; any = fix the runner first.
- **B2. The idle read X0.** No box open, Tally maximised and in front, only Tally and the runner. Run the read set X with `<cell>` = `x0`. EXPECT:
  - `uia-x0.txt`: `dpi_context=ok`, a `B` line in the task list (`parent_class=MSTaskListWClass`) whose name contains "Tally", and `in_task_list` equal to the number of task buttons the `x0` `PrintWindow` copy shows;
  - `cover-x0.txt`: `dpi_context=ok`, the `T` line and the `W` lines.

  A throw, a missing line, or counts that differ: record the output whole and STOP.
- **B3.** Probe check: Tally's title clicked; `zorder-probe-k.ps1` prints Tally first and in front. Record P-time, and the time one read set X takes.
- **B4.** The two frames: the screen's logical size and scale, and physical pixels. Every click below is recorded in both.
- **B5** before.

## Step HA: one Alt+Tab (in each cell, on the run coordinator's yes; otherwise write "HA not run")
1. **The gate.** G-P, labelled `ha`. Go on only if it shows Tally in front (and, in C1 and C2, the box open and topmost). Otherwise write "HA skipped: Tally did not hold the foreground" and go to the cell's next step. If G-P has not ended by T0+36, write "HA skipped: late" and go on.
2. **Press and HOLD Alt, press Tab ONCE, and keep Alt held.** The switcher opens. Stamp each key.
3. **Hold for about 3 s**, Alt still held. In that time the watcher writes its `A` and `C` lines. **Read the switcher by eye:** write down the title of every tile, in order, which one is highlighted, and whether one reads "ComplyEaze Bridge".
   - **STOP rule: HA never selects the box's tile.** Press no further Tab, and never release Alt while the switcher is open.
4. **With Alt still held, press Esc ONCE.** The switcher closes with no switch. Then release Alt. Stamp each key.
   - If the switcher is still open after the Esc: keep Alt held, press nothing else, click nothing, and STOP. Alt is released only on the run coordinator's word, since releasing it would switch to the highlighted tile.
5. **The foreground after it** is the watcher's `G` and `F` lines. EXPECT Tally in both.
   - If the box is in front: record "the switch was not cancelled", and press nothing more.

## Part N: the cell with no box (the control for C1 and C2, and the known answer for HA)
The cell's `.cmd`, `n.cmd`, holds two lines and is opened through `explorer.exe`:
```
powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -Command Start-Sleep -Seconds 60
```
1. **Pre-arm**, each started minimised:
   - `box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-n.txt -Seconds 100`;
   - `switcher-watch.ps1 -Name n -OutDir C:\bridge-lab\2161c\shots -Out <evidence>\switch-n.txt -Seconds 100`.

   Tally maximised and in front. Before the cell, while no box exists, take one screen-tool copy of Tally's screen.
2. **Open `n.cmd`.** Minimise its console during the 8 s sleep. About 4 s after the open, click Tally's title area ONCE.
3. **T0** is the open's stamp plus 8 s (there is no `SPAWN` line). At T0+8: G-P labelled `t8`. At T0+10: the read set X with `<cell>` = `n`.
4. **Step HA.**
5. When the watcher's `Z` line is written, take one screen-tool copy of Tally's screen.
6. **EXPECT**, for HA's known answer:
   - in `switch-n.txt`: one `A` line, one `C` line, at least one `L` or `D` line naming Tally's tile, and `G` and `F` lines with Tally in front;
   - `n-switcher.png` showing the switcher, opened from its file after the cell;
   - Tally's screen the same in both screen-tool copies (the Gateway, no menu, report or message).

   If Tally's screen changed: record it, change nothing, and STOP. If the watcher wrote no `A` or `C` line, or the copy does not show the switcher: write "the watcher fails its known answer". C1 and C2 then run without step HA, and the coordinator is told. If only the tile lines are missing, HA still runs, and Q2 is read by eye and from the copy.

## Part C: the product's box (the child, error output kept)
The cell's `.cmd` holds two lines and is opened through `explorer.exe`:
```
powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\2161c\requests\spawn-parent.ps1 -Target child -ChildExe C:\bridge-lab\2161c\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile C:\bridge-lab\2161c\requests\confirm-input.txt -Stdio files -Out C:\bridge-lab\2161c\evidence\<cell>.txt -TimeoutSeconds 120
```
`-Stdio files` writes the child's error output to `<cell>.stderr`, so its `box-fit:` lines are kept.

Cells **C1** and **C2** follow the same recipe; C2 repeats C1.
1. **Pre-arm**, each started minimised: the trace (`-Out <evidence>\trace-<cell>.txt -Seconds 100`) and the watcher (`-Name <cell> -Out <evidence>\switch-<cell>.txt -Seconds 100`), as in part N. Tally maximised and in front.
2. **Open the `.cmd`.** Minimise its console during the 8 s sleep. About 4 s after the open, click Tally's title area ONCE. Then there is no input until step 4, except the runner's own reads.
3. **T0** is the stamp on the `SPAWN ... child_pid=` line of `<cell>.txt`. If no box titled "ComplyEaze Bridge" exists by T0+15: record "no box", end nothing, wait for the end lines, and go to the next cell.
   - At T0+8: G-P labelled `t8`. EXPECT the box open and topmost, and Tally in front.
   - At T0+10: the read set X.
4. **Step HA.**
5. **At T0+46:** G-P labelled `t46`.
6. **No** by mouse at the No button's centre, from `rect-<cell>.txt`, at about T0+50. EXPECT exit 1, a 0-byte `<cell>.stdout`, and every `box-fit:` line `fits=true` with the box's own rectangle.
7. **Record:**
   - from the trace: `FIRST`; the box's hwnd; `exstyle` and `topmost`; `owner`, `owner_class`, `owner_exstyle` and `owner_topmost`;
   - from X, against part N's X: each `uia-<cell>.txt` `B` line in the task list that is not in `uia-n.txt` (by name and parent class), and whether any is named "ComplyEaze Bridge"; each `W` line of `bridge_mcp.exe` or meeting the task list; each button of the `PrintWindow` copy that is not in N's, by its centre;
   - from the watcher: the `A`, `C`, `L` or `D`, `G` and `F` lines, and each `N` line;
   - the tiles read by eye, in order;
   - the probes' z and foreground;
   - from `<cell>.stderr`: each `box-fit:` line, whole.

Run B5 after C1, before C2.

**Close.** Run B5 at the end and re-hash the inputs. Then close by `LAB-COMMON.md` C10:
1. MARKS while THE COMPANY is loaded: EXPECT ALTVCHID and ALTMSTID as at P0.
2. Shut Company.
3. C2 a and b again (one tally.exe owns 9001), GET /status only, and the screenshot showing no company loaded.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| C1 and C2: no button added for the box in any of the three reads against part N, and no "ComplyEaze Bridge" tile in the switcher (by eye, in the `L` or `D` lines, or in the copy) | **The owned box has no taskbar button and is not in Alt+Tab on master: #2161 closes on #2193.** |
| A button added for the box in any one read, in either cell | **Shown. The taskbar cells of rows 2161-A and 2161-B come back on BUILD-M.** Name the read and cell that showed it. |
| A "ComplyEaze Bridge" tile in the switcher, in either cell | **Shown, as above.** Alt+Tab is a keyboard route back to the box. |
| The three reads disagree (a button in one read and not another) | Counted as shown, as above; report the three whole. |
| C1 and C2 disagree | Report both whole; nothing is concluded; the coordinator decides. |
| The `PrintWindow` copy shows fewer buttons than N's, or a recorded console with `above_tray=1` meets the task list | That cell's taskbar is covered or not drawn: record the cover read, and do not read Q1 from that copy. |
| The trace reads `owner` 0 for the box | Not BUILD-M, or the owner was refused: check B1, and report. Nothing is concluded. |
| HA: the foreground at `G` or `F` is the box, or the box answered anything but a decline | Record it whole, then STOP: the carve-out does not hold. |
| HA skipped or not run in a cell | Q2 is not measured in that cell. |
| The watcher fails its known answer (part N) | Q2 is not measured; Q1 still reads. |
| A `box-fit:` line reading `fits=false` or `0,0,0,0`, or exit 77 | The fit check measured a window that is not the box, or the box did not fit: report the lines whole. |
| No box at all, or exit 0 with no token | The box could not be shown or measured: report it. |
| Anything else | Record it whole and infer nothing. |

**FAIL of the row:** any of these.
- A Yes, by click or key.
- Any request to Tally other than C2's and C10's reads.
- A key pressed other than step HA's under its carve-out (and P0's and C10's inputs).
- A taskbar click, a hover on the taskbar, or a click in the switcher.
- Tally's screen changed beyond focus.
- A mark different at the closing MARKS read.
- A window minimised other than as the rules name.
- A box or process left at the end (B5).
- B1's, B2's or B3's check not met.

## Evidence (stays on the lab PC except the files listed)
- **RESULTS.md**, one line per cell and step, with every stamp, read, owner field, foreground, key, rectangle, exit code and `box-fit:` line, and the Q1 and Q2 table per cell.
- **Cell files**, each whole, after the privacy search: `<cell>.txt`, `<cell>.stderr`, `<cell>.stdout`, the trace and `switch-<cell>.txt`.
- **Reads:** every `rect-*.txt`, `uia-*.txt`, `cover-*.txt`, `captures.txt` and `probe-*.txt`.
- **The runner**, under `scripts/`.
- **`notes.txt`**, in UTC time order.
- **`substitutions.txt`.**
- **`SCREENSHOTS.txt`**, the switcher copies and taskbar captures included.
- **`SHA256.txt`**, over the committed files, with sizes.
- **Also record:** BUILD-M's branch, head, CI run, artifact and exe sha256; and, read afresh, the Windows build, the Tally release, the screen's scale, and whether there is one monitor or more.

Evidence goes on a new branch `lab/2161-c-answers`. Send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- The server path (`post_import`). Its answer for each exit code is pinned by tests, not measured here.
- The review, invoice-stop release, settlement and ledger-create boxes, though they share the owner code.
- Whether a person notices a box that has no button and is refused the foreground, beyond its place on top.
- Alt+Tab with more than one Tab, or Win+Tab.
- A second monitor, Windows 11 (the watcher's class for it is not checked), and more than one sitting.
