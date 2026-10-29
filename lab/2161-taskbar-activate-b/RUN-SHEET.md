# Windows item 2161-B v1: what covers the taskbar while an approval box is open, and what one real taskbar activation does to the product's box. (Part of #2161)

WRITES to the book: NONE. **No ComplyEaze Bridge server is launched.**

The only requests to Tally are `LAB-COMMON.md`'s own read-only checks:
- at the start, GET /status and the MARKS read of C2 (c, d);
- just before the company is shut, the MARKS read again;
- after the shut, GET /status only (C10: no XML request after the shut).

Every box comes from the product's own dialog child, fed a fixed text, so with no server nothing can post. Each box is answered **No by mouse**, or ended by the one step this sheet names.

**No key is pressed in Tally from B2 until C10.** P0's load of the company and C10's shut use the inputs `LAB-COMMON.md` names, and are recorded. Otherwise Tally is used only as a window.

## Why
- **Row 2161-A** (`lab/2161-taskbar-activate-answers` @ 84e0caf7) found:
  - **The band is not `MB_SYSTEMMODAL`'s doing.** While any approval-style box was open, with or without the flag, a screen copy of the taskbar showed the button row as an undrawn band. At the same moment, `PrintWindow` of the taskbar drew every button, the box's own included. So something lay over the row, and nothing read what.
  - **Its taskbar cells (part C) are void.** The cell runner kept the click-into-Tally point and the located button in variables whose names differed only in letter case (`$Px` and `$px`). PowerShell treats those as one variable, so the click into Tally went to the button's point, and a "Windows PowerShell" window took the foreground, not Tally.
  - The taskbar clicks were aimed inside the task list, but each gave the foreground to the cell's own trace console (one left its title "Select Windows PowerShell", a console's title when it is clicked inside). The box was never activated. Whether they hit that console's button or its window over the task list was not recorded.
- **The lab's hypothesis:** the trace runs in a Windows PowerShell console, whose default background is dark navy. If that console, or the cell's `.cmd` console, lies over the task list, it may be the band. Part P-min tests it.
- **The code** (at #2107's merge 62123340):
  - On every activation of the post's box, `tally/box_fit.rs` `on_activate` writes one line `box-fit: box=(l,t,r,b) work=(l,t,r,b) fits=true|false` to the child's error output.
  - If it measures the box outside the work area, it sends Cancel itself, and the child exits 77, which the server reads as `import_review_too_large`.
  - A person's No or Cancel, or a close, gives exit 1, which the server reads as `import_approval_declined`.
  - Rows 1780-H and 2161-A counted one line per activation from outside the box.
  - The child under `spawn-parent.ps1 -Stdio files` keeps that output.
- **What this row decides:**
  - Q1. Which window lies over the taskbar's button row while a box is open: its class, program and title? Is it there with no box open, while only the lab's own helpers run? Does the band go when the lab's consoles are minimised?
  - Q2. With the box above Tally and Tally holding the foreground, does one click that reaches the box's own taskbar button leave the box open? What line does the fit check write?
  - Q3. With the box holding the foreground, does one such click minimise it, and what does the click that restores it do (exit code, `box-fit:` line)?
  - Q4. Expected from the code, not yet seen: does "Close window" from the box's taskbar menu end it with exit 1 and no new `box-fit:` line?

## Who acts
- The lab does every step on this synthetic company, on the owner's standing word of 27 Oct 2026: the title clicks, the clicks into Tally, the taskbar clicks, the menu click and every No. Record each as "done by the lab on the owner's standing word of 27 Oct".
- Posting is not switched on.
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first step and at the end; a different hash = STOP; `SHA256.txt` lists every file of this folder with its size)
- **BUILD-H**, already on the lab PC from row 1780-H. EXPECT `C:\bridge-lab\1780h\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe` with sha256 `a946979561b61f51e6dab3539d3de98fd48c8f103f61d2e3abe787acf3240c23`, and `pdfium.dll` beside it. Missing or different = STOP. The post box's code is the same on master.
- **Byte copies:**
  - `requests/box-trace-early.ps1`, `zorder-probe-k.ps1`, `spawn-parent.ps1`, `confirm-input.txt` from `lab/1780-system-modal-box` @ 693a04f5258cb14592b197b966de5aa1e60c0572;
  - `requests/box-rect-probe.ps1`, `taskbar-capture.ps1` from `lab/2161-taskbar-activate` @ de2b1d15c50f8d1d51eb3b8617a5ef76a6d89fc4;
  - all of them ran in rows 1780-H and 2161-A.
- **New, read-only: `requests/taskbar-cover.ps1`.** It is per-monitor DPI aware, so it works in physical pixels. Its C# compiled with Mono; the PowerShell has not run, so B2 checks it first. `taskbar-cover.ps1 -Out <file> [-X <px> -Y <py>]` writes:
  - the taskbar's window, rectangle and place in the stacking order;
  - every visible top-level window whose rectangle meets the taskbar's, from the top of the stacking order down, each with: `above_tray` (it lies above the taskbar), class, process id, program name (`image`), `topmost`, `exstyle`, `cloaked`, rectangle and title;
  - with `-X -Y`, what `WindowFromPoint` returns at that physical point, and its root window. It reads `point_ok=1` only when that window is `MSTaskListWClass` and its root is the taskbar (`Shell_TrayWnd`). `point_ok=1` says the point is on the task list, not which button: empty task-list space also reads 1. Only the thumbnail check below ties the point to the box's button.
- **Folders and company.**
  - Working folder `C:\bridge-lab\2161b\`; evidence in `C:\bridge-lab\2161b\evidence\`; captures in `C:\bridge-lab\2161b\shots\`.
  - THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, loaded alone, Tally maximised on the Gateway of Tally.
  - The `.ps1` files run from Windows PowerShell 5.1 as `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...`. A comma is a digit separator on this PC: no command takes a list.
- **Coordinates.** `PrintWindow` images are taskbar-relative. A point at (ix, iy) in such an image is at screen physical (ix + the taskbar's left, iy + the taskbar's top): add the `T` line's rectangle (row 2161-A: 0 and 1030).

### The cell runner
The cell runner is the lab's own. It is committed with the answers under `scripts/`, and its sha256 goes into `notes.txt` before the first cell.
- **Names.** Every point it clicks has its own variable, and no two variable names in the runner differ only in letter case. The names say what each point is: `$tallyBlankPoint`, `$boxButtonPoint`, `$noButtonPoint`. B1 checks this.
- **Each click records**, in the cell's timeline:
  - the point's name;
  - its logical and physical coordinates;
  - about 300 ms later, the foreground window (hwnd, program name, title).
- **Before a click into Tally:** `taskbar-cover.ps1 -X -Y` at `$tallyBlankPoint`. EXPECT `image=tally.exe`.
- **After a click into Tally,** Tally must be in front. If it is not:
  1. record "click into Tally missed";
  2. click nothing more except No by mouse;
  3. end the cell. It is void.
- **Reads.** The runner runs every read and capture inline in its own console, one after another.
  - **A read R** is:
    1. the line count of `<cell>.stderr`, and its last line;
    2. `box-rect-probe.ps1 -Out <evidence>\rect-<cell>-<label>.txt`;
    3. `zorder-probe-k.ps1`;
    4. `taskbar-cover.ps1 -Out <evidence>\cover-<cell>-<label>.txt`, with `-X -Y` when a click follows.
  - **A capture K** is `taskbar-capture.ps1 -Name <cell>-<label> -OutDir C:\bridge-lab\2161b\shots -Out <evidence>\captures.txt`, plus the runner's own whole-screen screen copy.
  - **Never the screen tool while a box is open.** Row 2161-A saw it minimise an open box.
- **Recorded pids.** The runner records the pids of its own console, the trace's console and the cell's `.cmd` console when it starts them. These are the "recorded consoles" below.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS.**
  - No key is pressed in Tally from B2 until C10.
  - No key is pressed anywhere from a cell's start until its box is answered or gone.
  - Keys are used only in the lab's own console, to start the runner before a cell starts.
  - Boxes are answered by mouse at the No button's centre.
  - Moving the pointer (hover) is not a click. Record it.
- **C2 carve-out (named).** `LAB-COMMON.md` C2 forbids ending any process. `spawn-parent.ps1` ends exactly one: the child it started itself, by the pid and handle its own `CreateProcess` returned, if it is still running after `-TimeoutSeconds` (120 in this row; the product's own limit is also 120 s). Nothing else is ended. A box or process left after the sitting is recorded and left.
- **Minimising a recorded console (named).** The runner may minimise a window with `ShowWindow(SW_MINIMIZE)`, by its own handle, only when all of these hold:
  - the window is a recorded console's top-level window;
  - it is not the foreground window at that moment;
  - it is minimised in P-min before T0, or in part C when the window is the one a `P` line names at the button point (its root being itself);
  - it is at most once per console per cell;
  - it is recorded.
  Nothing else is minimised, except Tally under "A box under Tally". If the row stays covered, the cell reads "not reachable", and nothing more is done.
- **A box under Tally.** If a box lies under Tally and must be answered, the runner:
  1. minimises Tally with `ShowWindow(SW_MINIMIZE)` by its handle;
  2. clicks No by mouse;
  3. restores Tally with `ShowWindow(SW_RESTORE)`.
  This is row 2161-A's way, so no click is made on Tally's buttons or the taskbar. Record each step.
- **Taskbar clicks** are made only in part C, and only under its gate. If a click's after-read shows anything other than the box or the taskbar affected, click nothing more on the taskbar in that cell and answer the box No.
- **A click into Tally** lands on a blank part of the Gateway screen, at least 150 logical pixels from every edge of the box. It never lands on a menu item, button, list line or field. If Tally's screen changes (another report or menu opens), record it, change nothing, and STOP.
- **One cell at a time.** A cell starts only after the previous cell's trace has written its `END` line.
- **Leftover list (B5)**, before the sitting, after part P-min, and at the end:
  - processes whose command line contains `--confirm-journal`, `spawn-parent.ps1` or `box-trace-early.ps1`;
  - visible windows whose title contains "ComplyEaze Bridge";
  - minimised recorded consoles still open.
  - EXPECT none. Any: record, end nothing, STOP.
- **Privacy.** Before committing evidence, search it for the user's name and for paths outside the lab folders. Replace each by `<user>` or `<path>`, and record the edit. That includes:
  - `cover-*.txt` titles;
  - the `F` lines;
  - the foreground titles, which name whatever window is there.

  Images stay local, with their name, size and sha256 in `SCREENSHOTS.txt`.

## Before the sitting
- **P0.** `LAB-COMMON.md` C1 and C2: BRIDGE BATCH LAB is in Select Company's list and loaded alone, else STOP (do not restore it, do not use another book). Record the inputs used to load it, and ALTVCHID and ALTMSTID from C2's MARKS read.
- **B1.** Check:
  - BUILD-H, as above;
  - every file of this folder, hashed against `SHA256.txt`;
  - the runner's sha256, recorded;
  - a case-insensitive scan of the runner for any variable name spelled two ways (`$Px` and `$px`, for example). EXPECT none; any = fix the runner first.
- **B2.** With no box open, `taskbar-cover.ps1` writes `dpi_context=ok`, a `T` line with a rectangle, and the `W` lines. Then two point checks:
  - `-X -Y` at the centre of Tally's own task button, from an idle `PrintWindow` copy converted as above: `point_ok=1`;
  - `-X -Y` at a point in the middle of Tally's Gateway: `point_ok=0`, with Tally's window.

  Anything else, or a throw: STOP, and record its output whole.
- **B3.** Probe check: Tally's title clicked; `zorder-probe-k.ps1` prints Tally first and in front. Record P-time and R-time.
- **B4.** The two frames: the screen's logical size and scale, and physical pixels. Every click below is recorded in both.
- **B5** before.

## Part N: the taskbar with no box (no click on the taskbar)
- **N1, idle.** Only Tally and the lab's own console are open. Make R and K, and record:
  - every `W` line;
  - whether the screen copy draws the task buttons;
  - each button's centre, in screen physical pixels, from the `PrintWindow` copy.
- **N2, the helpers with no box.**
  1. The runner pre-arms `box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-n2.txt -Seconds 40` in its own window, as in part P.
  2. It opens, through `explorer.exe`, a `.cmd` holding only `powershell -NoProfile -Command Start-Sleep -Seconds 30`.
  3. About 4 s later, it clicks Tally's title area once.
  4. At +12 s: R and K. Record the same as N1.
  5. Wait for both to end.

## Part P: the band's owner, with the product's box open
The cell's `.cmd` holds two lines and is opened through `explorer.exe`:
```
powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\2161b\requests\spawn-parent.ps1 -Target child -ChildExe C:\bridge-lab\1780h\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile C:\bridge-lab\2161b\requests\confirm-input.txt -Stdio files -Out C:\bridge-lab\2161b\evidence\<cell>.txt -TimeoutSeconds 120
```
- **P** (consoles as they open):
  1. Pre-arm the trace (`-Title "ComplyEaze Bridge" -Out <evidence>\trace-<cell>.txt -Seconds 160`, its own window). Tally in front.
  2. Open the `.cmd`. About 4 s later, click Tally's title area ONCE.
  3. `<cell>-T0` is the `SPAWN ... child_pid=` stamp. At T0+6: R and K. If there is no box by T0+15: record, end nothing, and wait for the end lines.
  4. Click into Tally at T0+16 (±2 s) at `$tallyBlankPoint`. Tally must then be in front. At C+3: R and K.
  5. At about C+10: No by mouse. EXPECT exit 1 and a 0-byte `<cell>.stdout`.
- **P-min** (the lab's consoles minimised): the same, except:
  - the runner starts the trace console minimised (`Start-Process ... -WindowStyle Minimized`, a launch option of its own helper);
  - during the `.cmd`'s 8 s sleep, before T0 and before the second title click, it minimises the `.cmd`'s own console (a recorded console), as the rules allow;
  - it records each console's state (`iconic`) at T0+6 and C+3.
- **Record**, for P and P-min at T0+6 and C+3:
  - every `W` line, especially those with `above_tray=1` and `cloaked=0` that meet the task list's rectangle (the `MSTaskListWClass` `K` line of `box-rect-probe`);
  - whether the screen copy shows the band;
  - each button's centre, from the `PrintWindow` copy;
  - each `box-fit:` line.
- **The band's owner:** the window or windows with `above_tray=1` and `cloaked=0` that meet the task list while the screen copy shows the band. Name each by class, program, `exstyle` and title, and compare with N1, N2 and P-min.
- **The box's button:** the button in P-min's `PrintWindow` copy that is in neither N1's nor N2's. If there is not exactly one such button, write "the box's button is not identified" and do not run part C.

Run B5 after part P-min.

## Part C: one taskbar activation of the product's box (the child, error output kept)
- Each cell runs **the P-min way**: the trace and `.cmd` consoles minimised before T0. `-Out` is set to `...\<cell>.txt`, and the pre-armed trace is `trace-<cell>.txt`.
- **Locate the button in each cell.** `$boxButtonPoint` is re-located from the cell's own T0+6 `PrintWindow` copy, against N2's buttons, because the task list's width changes between cells.

**The gate, before every taskbar click:**
1. R, with `taskbar-cover.ps1 -X -Y` at `$boxButtonPoint` (physical).
2. If `point_ok=0` and the window at the point is a recorded console, minimise it as the rules allow, and R again.
3. Click only if `point_ok=1`.
4. Then hover at the point and take the runner's whole-screen copy. Click only if it shows a thumbnail whose title **begins with** "ComplyEaze Bridge".
5. Otherwise, record which of these held, click nothing on the taskbar, and answer No:
   - "taskbar not reachable at the button point" (with the `P` line);
   - "no thumbnail for the box";
   - "a thumbnail with another title".

After each taskbar click, R at once and again 3 s later. For a box that ends, record the interval from the click to the trace's `GONE` (earlier rows saw about 78 ms).

- **C1: the box above Tally, Tally holding the foreground.**
  1. Steps 1 to 4 of part P.
  2. At C+6: the gate, then ONE left click at `$boxButtonPoint`.
  3. If several thumbnails open (a grouped button): ONE click at the centre of the preview image of the thumbnail whose title begins with "ComplyEaze Bridge", away from its close mark.
  4. If the box is still open: No by mouse. EXPECT exit 1 and a 0-byte `.stdout`.
  5. If the box is gone: click nothing more. Record the exit code, the `.stdout` size and `c1.stderr`, whole.
- **C2: the box holding the foreground.**
  1. Steps 1 to 3 of part P, with no click into Tally.
  2. At T0+12: R. EXPECT the box in front. If it is not, record "C2 not reached", answer No, and go to C3.
  3. At T0+16: the gate, then ONE left click, as in C1.
  4. If the box then reads `iconic=1`: at +6 s, the gate again (the point must still read `point_ok=1`), then ONE more left click. Then:
     - box open and visible: No by mouse;
     - still minimised: click nothing more, and let the 120 s bound end it (record it);
     - gone: record as in C1.
  5. If the box is open and not minimised: No by mouse.
- **C3: a close, expected from the code.**
  1. Steps 1 to 4 of part P.
  2. At C+6: the gate (the thumbnail check included), then ONE right-click at `$boxButtonPoint`.
  3. Take the runner's whole-screen copy of the menu.
  4. Read `taskbar-cover.ps1 -X -Y` at the item visibly labelled "Close window".
  5. ONE left click on that item only, and only if the window at that point belongs to none of `bridge_mcp.exe`, `tally.exe`, `powershell.exe`, `cmd.exe` or `conhost.exe`. The menu's other items open a new window of the program or change the taskbar.
  6. If there is no such item, the menu is unexpected, or the point's window fails that check:
     1. one click at `$tallyBlankPoint`, which closes the menu;
     2. No by mouse;
     3. record which.
  7. EXPECT:
     - the box gone within 1 s;
     - exit 1;
     - a 0-byte `.stdout`;
     - `c3.stderr` holding exactly the `box-fit:` lines it held at the R before the right-click.

**Close.** Run B5 at the end and re-hash the inputs. Then close by `LAB-COMMON.md` C10:
1. MARKS while THE COMPANY is loaded: EXPECT ALTVCHID and ALTMSTID as at P0.
2. Shut Company.
3. C2 a and b again (one tally.exe owns 9001), GET /status only, and the screenshot showing no company loaded.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| P: a recorded console with `above_tray=1`, `cloaked=0` meets the task list while the screen copy shows the band; P-min: no console there and the buttons drawn | The band is the lab's own console window (the lab's hypothesis). A person's screen has no such console: the band is a lab artefact. |
| N2 shows the band, with the same console over the task list | The helpers alone cause it, with no box. |
| P and P-min both show the band, and the window over the row belongs to the box's process, explorer, dwm or another system program | Not the lab's consoles: report the `W` lines whole, with `exstyle`. |
| P and P-min both show the band, with no uncloaked window over the task list | A drawing state, not a window: report it. |
| The band does not appear in P at all | Not reproduced here (row 2161-A's B6 showed it is not fixed). Q1 is open; part C may still run. |
| `point_ok=1` at the button point, but no thumbnail, or a thumbnail with another title | **Part C not run: Q2 to Q4 not measured.** Cite the `P` line and the copy. |
| A gate: `point_ok=0` after the one allowed minimise | The taskbar cannot be reached at that point: no click, and the cell reads "not reachable". |
| C1: the box still open, one new `fits=true` line | A taskbar activation of a box above Tally did no harm in this sitting (one run, partial). |
| C1 or C2: the box still open, no new line | The click reached the task list but did not activate the box. Infer nothing about the fit check from that cell. |
| C1 or C2: the box gone, exit 77, a new `fits=false` line far off-screen (for example near -32000 or -40000) | The fit check measured a minimised or hidden box as it was activated. The server would answer `import_review_too_large`. |
| C1 or C2: the box gone, exit 77, a new `fits=false` line with a normal rectangle | The fit check ended it on the rectangle it shows. Compare with `work=`. |
| The box gone, exit 77, `box-fit: not readable fits=false` | The fit check could not measure the box at that activation. |
| The box gone, exit 1, no new `box-fit:` line | Something closed the box without activating it. The server would answer `import_approval_declined`. |
| The box gone, exit 1, with a new `fits=true` line (C3 included) | Activated, then closed as a No by something other than the fit check. |
| The box gone, exit 0, a 0-byte `.stdout` | The child ended without a box answer (the fit check's "unavailable" outcome). Record `.stderr` whole. |
| C2: `iconic=1` after the first click | A taskbar click minimises a box that holds the foreground. |
| C2: still minimised at the 120 s bound | The restore click did not restore it. The server would answer `import_approval_timed_out` at its own 120 s bound. |
| C3: exit 1 and no new `box-fit:` line | The close behaves as the code predicts. |
| A click into Tally missed (Tally not in front after it) | That cell is void, as row 2161-A's were: record it. |
| Anything else | Record it whole and infer nothing. |

**FAIL of the row:** any of these.
- A key pressed in Tally during N, P or C, or anywhere while a box was open.
- A Yes.
- Any request to Tally other than C2's and C10's reads.
- Tally's screen changed beyond focus.
- A mark different at the closing MARKS read.
- A window minimised other than as the rules name.
- A box or process left at the end (B5).
- B1's, B2's or B3's check not met.

## Evidence (stays on the lab PC except the files listed)
- **RESULTS.md**, one line per cell and step, with:
  - every stamp, read and image reading;
  - every `W` and `P` line that decided something;
  - each click's point name, coordinates, and the foreground after it;
  - each exit code and `box-fit:` line.
- **Cell files**, each whole, after the privacy search: `<cell>.txt`, `<cell>.stderr`, `<cell>.stdout` and trace.
- **Reads:** every `rect-*.txt`, `cover-*.txt`, `captures.txt` and `zorder-probe-k` output.
- **The runner**, under `scripts/`.
- **`notes.txt`**, in UTC time order.
- **`SCREENSHOTS.txt`.**
- **`SHA256.txt`**, over the committed files, with sizes.
- **Also record**, read afresh: the Windows build, the Tally release, the screen's scale, and whether there is one monitor or more.

Evidence goes on a new branch `lab/2161-taskbar-activate-b-answers`. Send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- The server path (`post_import`). Its answer for each exit code is pinned by tests, not measured here.
- The review, invoice-stop release and ledger-create boxes.
- Any key; Alt+Tab; a second monitor; a scale change while a box is open; Windows 11; more than one sitting.
