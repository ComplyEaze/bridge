# Windows item 2161-A v1: the approval box opened system-modal (#2107) and the taskbar. Is the taskbar drawn while a box is open, and what does one taskbar activation do to the product's box? (Part of #2161)

WRITES to the book: NONE. **No ComplyEaze Bridge server is launched.** The only requests to Tally are `LAB-COMMON.md`'s own read-only checks: GET /status and the MARKS read of C2 (c, d), and GET /status at C10. Every box comes from one of two sources, and each is answered **No by mouse** or ended by the one step this sheet names:
- the product's own dialog child, fed a fixed text (no server, so nothing can post);
- row 1780-G's control box, which has no Bridge code.

**No key is pressed in Tally at all, and none anywhere while a box is open.** Tally is used only as a window, with at most one click on its title area and one click on a blank part of its Gateway screen per cell.

## Why
- Row 1780-H (`lab/1780-system-modal-box-answers` @ 968810d4) ran #2107's build. Each system-modal box kept the topmost bit and stayed above Tally after a click into Tally. The taskbar step (T4) was not made: while a box was open, a screen copy of the taskbar showed the whole task-button area as one undrawn dark band. Idle, the same capture drew every button.
- Rows 1780-E, 1780-F (part R), 1780-C2 and 1780-D saw a taskbar or thumbnail activation end an unanswered box about 78 ms after the click, before #2107. The child's error output was discarded, so the cause is not known.
- **Why the child alone answers it** (code at #2107's head 2bc31a31):
  - The post box is shown by the child `bridge_mcp.exe --confirm-journal` (`tally/approved_import.rs`, `nonce_bound_dialog`). The server discards its error output and reads only its exit code and token.
  - In the child, `tally/box_fit.rs` `guarded` hooks the box's own thread. On **every** activation of the box, `on_activate` measures the box against the work area of its monitor and writes one line `box-fit: box=(l,t,r,b) work=(l,t,r,b) fits=true|false` to the error output.
  - A box measured outside the work area, or not measurable (`box-fit: not readable fits=false`), is sent Cancel, and the child exits **77**, which the server reads as `import_review_too_large`.
  - No, Cancel or a close gives exit **1**, read as `import_approval_declined`. Exit **0** without the token is read as `import_approval_unavailable`. None of these reaches Tally.
  - Row 1780-H counted one line per activation from outside the box: H0 1, H1 2, H2 2.
  - So the child under `spawn-parent.ps1 -Stdio files`, which keeps the error output, shows what a taskbar activation did. The exit code's mapping to the server's answer is pinned by the tests in `approved_import.rs` (`a_box_that_did_not_fit_the_screen_is_refused_whatever_it_printed`).
- **What this row decides:**
  - Q1. While a box is open, is the taskbar's button area undrawn on the screen, or only in one capture route? Also: can a person still find the box's button (a hover shows its thumbnail)?
  - Q2. Is the undrawn area the system-modal flag's doing? The control box with it (variant h) against the same box without it (variant c), all else equal.
  - Q3. With the product's box above Tally and Tally holding the foreground, does one click on the box's taskbar button leave the box open? What line does the fit check write?
  - Q4. With the product's box holding the foreground, does one click on its taskbar button minimise it? If it does, does the click that restores it end the box, with which exit code and `box-fit:` line?
  - Q5. Expected from the code, not yet seen in the lab: "Close window" from the box's taskbar menu ends it with exit 1 and no new `box-fit:` line.

## Who acts
- The lab does every step on this synthetic company, on the owner's standing word of 27 Oct 2026: the title clicks, the clicks into Tally, the taskbar clicks and the menu click, and every No. Record each as "done by the lab on the owner's standing word of 27 Oct". Posting is not switched on.
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first step and at the end; a different hash = STOP; `SHA256.txt` lists every file of this folder with its size)
- **BUILD-H**, already on the lab PC from row 1780-H: #2107 at 48898314. The later head 2bc31a31 changes only comments, so the binary's behaviour is the same.
  - EXPECT `C:\bridge-lab\1780h\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe` to have sha256 `a946979561b61f51e6dab3539d3de98fd48c8f103f61d2e3abe787acf3240c23`, with `pdfium.dll` beside it.
  - Missing or different = STOP. Ask the run coordinator; do not download a build.
- `requests/box-trace-early.ps1`, `zorder-probe-k.ps1`, `spawn-parent.ps1`, `confirm-input.txt`: byte copies from `lab/1780-system-modal-box` @ 693a04f5258cb14592b197b966de5aa1e60c0572, all run in row 1780-H. `zorder-probe-k.ps1` also prints windows titled "Lab topmost control".
- `requests/control-box-fixes.ps1`: a byte copy from `lab/1780-topmost-fixes` @ c0805b145adcfbc3997f35aff1fd93e4b8f1f8f6, run in row 1780-G. Variant c is the approval box's flags without `MB_SYSTEMMODAL`; variant h adds it. Its box is titled "Lab topmost control".
- `requests/box-rect-probe.ps1` and `requests/taskbar-capture.ps1` are new and read-only. Both are per-monitor DPI aware, so they measure in physical pixels, the fit check's unit. Their C# parts compiled with Mono; the PowerShell has not run, so B2 runs each once before any cell.
  - `box-rect-probe.ps1 -Out <file>` writes, to a new file each time:
    - each box's hwnd, pid, `iconic`, `topmost` and rectangle;
    - the foreground window;
    - the taskbar's windows (`Shell_TrayWnd`, `ReBarWindow32`, `MSTaskSwWClass`, `MSTaskListWClass`) with their rectangles;
    - whether the DPI context was set.
  - `taskbar-capture.ps1 -Name <n> -OutDir <dir> -Out <file>` saves the taskbar twice at one moment: `<n>-screen.png`, a screen copy (row 1780-H's route), and `<n>-print.png`, the taskbar window asked to draw itself (`PrintWindow`). It appends a line with `dpi=`, the taskbar's rectangle and `print_window=`.
- **The cell runner** is the lab's own, like row 1780-H's `h-cell.ps1`, adapted to this sheet and committed with the answers under `scripts/`. It:
  - pre-arms the trace;
  - opens the `.cmd`;
  - makes the title click, the click into Tally and the taskbar clicks at the stamps named, at points the lab gives it in command files, so nothing is typed during a cell;
  - runs every read **inline in its own console, one after another**, so no new window takes the foreground.
  - **A read R** is, in this order:
    1. the line count of `<cell>.stderr` and its last line (the file is readable while the child runs);
    2. `box-rect-probe.ps1 -Out <evidence>\rect-<cell>-<stamp-label>.txt`;
    3. `zorder-probe-k.ps1`.
  - **A capture K** is `taskbar-capture.ps1 -Name <cell>-<label> -OutDir C:\bridge-lab\2161\shots -Out <evidence>\captures.txt`.
  - Each read and capture records its start and end stamps.
- Working folder `C:\bridge-lab\2161\`; evidence in `C:\bridge-lab\2161\evidence\`; captures in `C:\bridge-lab\2161\shots\`.
- THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, loaded alone, Tally maximised on the Gateway of Tally.
- The `.ps1` files run from Windows PowerShell 5.1 as `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...`. A comma is a digit separator on this PC: no command takes a list.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS.** No key is pressed in Tally at any point. None is pressed anywhere from a cell's start until its box is answered or gone. Keys are used only in the lab's own console, to start the runner before a cell starts. Boxes are answered by mouse at the button's centre. Moving the pointer over the taskbar (hover) is not a click and is allowed; record it.
- **C2 carve-out (named).** `LAB-COMMON.md` C2 forbids ending any process. `spawn-parent.ps1` ends exactly one: the child it started itself, by the pid and handle its own `CreateProcess` returned, if still running after `-TimeoutSeconds` (60). Nothing else is ended. A box or process left after the sitting is recorded and left.
- **A box under Tally.** If a box lies under Tally and must be answered, the clicks are, in order (record each):
  1. one click on Tally's minimise button (never its close button beside it);
  2. No by mouse;
  3. one click on Tally's own taskbar button to restore it.
- **Taskbar clicks.**
  - They are made only in part C: on the box's own button, found by hovering until its thumbnail shows the box's title (a control cell's console has its own button), or on the preview image of the box's thumbnail (never a thumbnail's close mark), and on the one menu item C3 names.
  - A click that lands on another window's button: record it, click nothing more on the taskbar in that cell, and answer the box No.
  - If that click minimised Tally, restore Tally by one click on its own taskbar button after the No.
- **A click into Tally** lands on a blank part of the Gateway screen. It is at least 150 logical pixels from every edge of the box, and never on a menu item, button, list line or field. If Tally's screen changes (another report or menu opens), record it, change nothing, and STOP.
- **Leftover list (B5)** before the sitting, after part B, and at the end:
  - processes whose command line contains `--confirm-journal`, `spawn-parent.ps1`, `control-box-fixes.ps1` or `box-trace-early.ps1`;
  - visible windows whose title contains "ComplyEaze Bridge" or "Lab topmost control".
  - EXPECT none. Any: record, end nothing, STOP.
- **Privacy.** Before committing evidence, search it for the user's name and for paths outside the lab folders. Replace each by `<user>` or `<path>` and record the edit. This includes each probe's `F` line, which names whatever window is in front. Screenshots and captures stay local, with name, size and sha256 in `SCREENSHOTS.txt`; the results say in words what each shows.

## Before the sitting
- P0. `LAB-COMMON.md` C1 and C2: BRIDGE BATCH LAB is in Select Company's list and loaded alone, else STOP (do not restore it, do not use another book). Its GET /status and MARKS read are the row's only reads of Tally before C10. Record ALTVCHID and ALTMSTID.
- B1. BUILD-H checked as above; hash every file of this folder against `SHA256.txt`.
- B2. Helper checks, with no box open. A helper that throws or prints otherwise = STOP; record its output whole.
  - `box-rect-probe.ps1` writes `dpi_context=ok`, `boxes=0`, the foreground window, and four `K` lines with a rectangle each.
  - `taskbar-capture.ps1 -Name idle` writes two images and `dpi=ok`. EXPECT the taskbar's rectangle along the screen's bottom edge, with its top at the work area's bottom (1030 in row 1780-H's `box-fit:` lines).
  - Open both images, and the screen tool's own whole-screen screenshot taken at the same time. Record, for each: the task buttons drawn (and how many) or not.
  - A `-print.png` that is black or blank while the other two draw the buttons is recorded as "print route dropped": that route is not read in later cells. This is not a STOP.
- B3. Probe check: Tally's title clicked, `zorder-probe-k.ps1` prints Tally first and in front. Record P-time (one probe's run time) and R-time (one whole read R).
- B4. The two frames: the screen tool's frame, and the screen's logical size and scale. Every click below is recorded in both.
- B5 before.
- B6. From row 1780-H, on the lab PC: look at the bottom strip of `s9-c0-screen.png` and `l2-c0-screen.png`. These are whole-screen screen copies (the same route as the band capture) taken while a box was open, at C+0. Record for each: task buttons drawn, or an undrawn band. Nothing is clicked.
  - Drawn there = the band is not a fixed effect of an open box: record it and say so.

## Part B: is the taskbar drawn while a box is open? (one variable at a time; no taskbar click)
A cell is a two-line `<cell>.cmd` in the working folder, opened by the runner through `explorer.exe` (row 1780-H's way):
```
powershell -NoProfile -Command Start-Sleep -Seconds 8
<the cell's spawn-parent line>
```
- **B-h** (control, with `MB_SYSTEMMODAL`): `powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\2161\requests\spawn-parent.ps1 -Target control -ControlScript C:\bridge-lab\2161\requests\control-box-fixes.ps1 -ControlVariant h -Stdio tokio -Out C:\bridge-lab\2161\evidence\b-h.txt -TimeoutSeconds 60`. `-Stdio tokio`, as row 1780-G ran the control: the control box writes no error output, and `files` would need an input file.
- **B-c** (control, without it): the same with `-ControlVariant c` and `-Out ...\b-c.txt`.
- **B-p** (the product's child): `powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\2161\requests\spawn-parent.ps1 -Target child -ChildExe C:\bridge-lab\1780h\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile C:\bridge-lab\2161\requests\confirm-input.txt -Stdio files -Out C:\bridge-lab\2161\evidence\b-p.txt -TimeoutSeconds 60`

Each cell, in the order B-h, B-c, B-p:
1. **Pre-arm.** The runner starts the trace in its own window and waits: `box-trace-early.ps1 -Title "<box title>" -Out <evidence>\trace-<cell>.txt -Seconds 100`. The box title is "Lab topmost control" for B-h and B-c, and "ComplyEaze Bridge" for B-p. Tally maximised and in front.
2. **Start.** The runner opens `<cell>.cmd`, and **about 4 s after the start clicks Tally's title area ONCE.** No other input until step 4.
3. **First reads.** `<cell>-T0` is the stamp on the `SPAWN ... child_pid=` line of `<cell>.txt`. At T0+6 the runner makes R, then K labelled `open`, then the lab takes a screen-tool whole-screen screenshot. Then the lab hovers over the place where a new task button would be (the end of the buttons drawn idle in B2), and records whether a thumbnail shows and its title.
   - No box by T0+15: record "no box", end nothing, wait for `<cell>.txt`'s end line (at most the 60 s bound), then the next cell.
4. **The click into Tally at T0+16 (±2 s)**, made by the runner on a blank part of the Gateway (see Rules); record the point in both frames. At once: R. Then K labelled `clicked`, a screen-tool screenshot, and the same hover as step 3.
5. **No.** At about C+12: **No** by mouse on the box's No button. If the box lies under Tally, use "A box under Tally". EXPECT:
   - B-h and B-c: `.control.txt` reads `answer=7`;
   - B-p: exit 1 and `stdout_bytes=0`.
6. **Record**, for steps 3 and 4:
   - for each image (screen copy, `PrintWindow` unless dropped, screen tool): the task buttons drawn (and how many) or an undrawn band, and the hover result;
   - the `K` lines;
   - the box's rectangle, `iconic`, `topmost` and the foreground;
   - for B-p, each `box-fit:` line of `b-p.stderr`.

Run B5 after B-p.

**Part C is run only if** B-p's step 4 found the box's task button: drawn in the screen-tool screenshot, or found by hover with its thumbnail titled "ComplyEaze Bridge — post this voucher?". Record where the button is (the hover point). Otherwise write "part C not run: the box's task button could not be found while it was open" and go to the end.

## Part C: one taskbar activation of the product's box (the child, error output kept)
Each cell uses B-p's `.cmd` with `-Out` set to `...\<cell>.txt` and the pre-armed trace `trace-<cell>.txt`. Every click is made by the runner at the point the lab found by hover, and recorded in both frames with its stamp. **R is made right before each click, at once after it, and 3 s after it.** For a box that ends, record the click-to-end interval, which is the click's stamp to the trace's `GONE` stamp (rows 1780-E/F/C2/D saw about 78 ms).
- **C1 (box above Tally, Tally holding the foreground).**
  1. Steps 1 to 4 of part B (click into Tally at T0+16).
  2. At C+6, ONE left click on the box's taskbar button.
  3. If that opens several thumbnails (a grouped button), ONE click on the preview image of the thumbnail titled "ComplyEaze Bridge" (never its close mark).
  4. Then:
     - box still open: **No** by mouse. EXPECT exit 1, `stdout_bytes=0`;
     - box gone: click nothing more. Record the exit code, `stdout_bytes` and `c1.stderr` whole.
- **C2 (box holding the foreground).**
  1. Steps 1 to 3 of part B: the title click at about 4 s, and **no click into Tally**.
  2. At T0+12, R. EXPECT the box in front. If it is not, record "C2 not reached", answer No and go to C3.
  3. At T0+16, ONE left click on the box's taskbar button (as in C1, thumbnail rule included).
  4. Then:
     - The box reads `iconic=1` (minimised): at +6 s, ONE more left click on its taskbar button to restore it, with R around it as above. Then:
       - the box is open and visible: **No** by mouse;
       - it is still minimised: click nothing more and let the 60 s bound end it (spawn-parent records `exit_code=killed`);
       - it is gone: record as below.
     - The box is open and not minimised: **No** by mouse.
     - The box is gone: click nothing more. Record the exit code, `stdout_bytes` and `c2.stderr` whole.
- **C3 (a close, expected from the code).**
  1. Steps 1 to 4 of part B.
  2. At C+6, ONE right-click on the box's taskbar button.
  3. Take a screen-tool zoom screenshot of the menu that opens.
  4. ONE left click on the item visibly labelled "Close window", and on nothing else. Its other items open a new window of the program or change the taskbar, so they are never clicked.
  5. If no item is labelled so, or the menu looks otherwise unexpected:
     1. click once on the same blank point of Tally's Gateway used for the click into Tally (that closes the menu);
     2. **No** by mouse;
     3. record "no Close window item" with the zoom screenshot, and do not run C3 again.
  6. EXPECT:
     - the box gone within 1 s;
     - exit 1 and `stdout_bytes=0`;
     - `c3.stderr` holding exactly the `box-fit:` lines it held at the R before the right-click (a right-click and a close do not activate the box).

Run B5 at the end. Re-hash the inputs. Close as `LAB-COMMON.md` C10, with GET /status and the MARKS read again: EXPECT ALTVCHID and ALTMSTID as at P0.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| B6: the task buttons drawn in a 1780-H whole-screen copy taken while a box was open | The band is not a fixed effect of an open box: something else at that moment. |
| B-p step 4: the screen tool shows the buttons, or a hover finds the box's button | A person can still reach the box from the taskbar while it is open; part C runs. |
| B-h and B-p show the band in every route and no hover finds a button; B-c draws the buttons | `MB_SYSTEMMODAL` stops the taskbar's buttons drawing while the box is open. A finding against #2107's flag, reported as such; part C not run. |
| B-c shows the band too | The band is not the flag's doing (something common to both boxes). |
| B-h draws the buttons and B-p does not | The band comes from the product's child, not the flag: report what differs. |
| Only the screen copy shows the band; `PrintWindow` and the screen tool draw the buttons | The band is an artefact of the screen copy route. |
| C1: the box still open, one new `fits=true` line | A taskbar activation of a box above Tally is harmless. |
| C1 or C2: the box still open, no new line | The click did not activate the box (the hook was not reached); say so, and infer nothing about the fit check from that cell. |
| C1 or C2: box gone, exit 77, a new `fits=false` line with coordinates near -32000 | The fit check measured a minimised box as it was restored; the server would answer `import_review_too_large`. |
| C1 or C2: box gone, exit 77, a new `fits=false` line with a normal rectangle | The fit check ended it on the rectangle it shows; compare with `work=`. |
| Box gone, exit 77, a new `box-fit: not readable fits=false` | The fit check could not measure the box at that activation. |
| C1 or C2: box gone, exit 1, no new `box-fit:` line | Something closed the box without activating it; the server would answer `import_approval_declined`. |
| Box gone, exit 1, with a new `fits=true` line | The box was activated, then closed as a No by something other than the fit check. |
| Box gone, exit 0, `stdout_bytes=0` | The child ended without a box answer; the server would answer `import_approval_unavailable`. |
| C2: `iconic=1` after the first click | A taskbar click minimises a box that holds the foreground. |
| C2: not minimised, still open, no new line | A taskbar click on the box in front did nothing to it. |
| C2: `exit_code=killed` at the 60 s bound | The box stayed minimised; the server would answer `import_approval_timed_out` at its own bound. |
| C3: exit 1 and no new `box-fit:` line | The close behaves as the code predicts. |
| Anything else | Record it whole and infer nothing. |

**FAIL of the row:**
- a key pressed in Tally, or anywhere while a box was open;
- a Yes;
- any request to Tally other than C2's and C10's reads;
- Tally's screen changed beyond focus;
- a mark different at C10;
- a box or process left at the end (B5);
- B2's or B3's check not met.

## Evidence (stays on the lab PC except the files listed)
- RESULTS.md: one line per cell and step, with every stamp, read, image reading, hover result, exit code and `box-fit:` line.
- Each `<cell>.txt`, `<cell>.stderr`, `<cell>.stdout`, `.control.txt` and trace, whole, after the privacy search.
- Every `rect-*.txt`, `captures.txt` and `zorder-probe-k` output.
- The runner under `scripts/`.
- `notes.txt` in UTC time order.
- `SCREENSHOTS.txt`: names, sizes and sha256 of the images, which stay local.
- `SHA256.txt` over the committed files, with sizes.
- Also record the Windows build, the Tally release and the screen's scale, read afresh, and whether there is one monitor or more.

Evidence goes on a new branch `lab/2161-taskbar-activate-answers`. Send its head to the run coordinator before pushing; push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- The server path (`post_import`): its answer for each exit code is pinned by tests, not measured here.
- The review, invoice-stop release and ledger-create boxes. The fit check does not measure the ledger box, so a taskbar activation cannot end it as too large.
- Any key; Alt+Tab; a second monitor; a scale change while a box is open; Windows 11; more than one sitting.
