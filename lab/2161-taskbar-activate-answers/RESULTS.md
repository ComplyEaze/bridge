# Results: #2161, the system-modal approval box and the taskbar, 29 Oct 2026. **While any approval-style box is open, with or without `MB_SYSTEMMODAL`, the taskbar's button row is not repainted in a screen copy and shows no hover thumbnail, though the taskbar still draws every button when asked (`PrintWindow`). One left click on the product box's own task button did nothing to the box, whether the box was above Tally (C1) or in front (C2): it stayed open, not minimised, still topmost, and the fit check wrote no new line. A right-click opened no menu, so there was no "Close window" to try (C3). Each box was answered No by mouse: exit 1, nothing on stdout.**

## Findings
1. **The band is not the flag's doing.** In B-h-2 (`MB_SYSTEMMODAL`), B-c (without it) and B-p (the product's child), every screen copy taken while the box was open shows the task-button area as one undrawn band (the wallpaper's gradient), from the search box to the tray; every `PrintWindow` copy at the same moment draws all the buttons, and in B-p a new button for the box itself (a blue tick, at about physical x 948). Idle, and after B-h's box had been minimised, both routes draw the buttons.
2. **No hover thumbnail while a box is open.** Pointer moves (no click) over the new-button point and over the PowerShell button, made by the runner, showed no thumbnail in any screen copy in B-h-2, B-c or B-p. (In B-h, after the box had been minimised by the screen tool, a hover by the screen tool did show the PowerShell group's two thumbnails, "Windows PowerShell" and "Lab topmost control".)
3. **A taskbar click does not reach the product's box.** C1 (box above Tally, Tally in front) and C2 (box in front): one left click on the box's button at physical (948,1055), located in a `PrintWindow` copy taken in the same cell. Reads at once and 3 s after: the box open, `iconic=0`, `topmost=1`, rectangle 686,181,1254,926 unchanged, `box-fit:` lines still 1 (no activation). In C2 the click took the foreground off the box (box 1,1,1 before, 1,1,0 after), and did not minimise it. By the sheet's table: "the click did not activate the box (the hook was not reached)" (C1) and "a taskbar click on the box in front did nothing to it" (C2). Nothing is inferred about the fit check from these cells.
4. **C3:** a right-click on the box's button opened no menu that a screen copy shows: "no Close window item". One click on the blank Tally point, then No.
5. **The screen tool changes the cell (a finding about the lab's own tool).** The desktop client's screen tool hides the windows of apps not granted to it before each screenshot. In cell B-h its screenshot at the step-3 point minimised the control box (`iconic=1`, rectangle at -40000 at C+0, against `iconic=0` at T0+6); the runner's No then fell off-screen, and the box ended at the 60 s bound. From then on no screen-tool screenshot or zoom was taken while a box was open.
   - **Earlier cells with a screen-tool capture while a box was open, as far as their evidence shows:** row 1780-H cell H1 (a screen-tool zoom of the taskbar strip at about T0+25, which came back masked, before the band capture). Its box was still open and answerable after it (No by the fallback at T0+48 landed). Row 1780-H S1, S9 and L2 and cells H0 and H2 used no screen-tool capture while their box was open: their band (S9, S1) came from the runner's own screen copy. So the screen tool explains B-h's minimised box, not the band.
6. **B6:** in row 1780-H's whole-screen copies taken while a box was open, S9's shows the band over the whole button row and L2's shows the buttons drawn under a partial dark band: the band is not a fixed effect of an open box.

## Per cell
| cell | box | after the click into Tally (C+0) | taskbar (screen copy / PrintWindow) | answer |
|---|---|---|---|---|
| B-h | control h | minimised by the screen tool (see finding 5) | band at open / buttons drawn | unanswered: 60 s bound (`killed_after_timeout`) |
| B-h-2 | control h | above Tally (1,1,0), Tally in front | band / drawn (open and clicked) | No, `answer=7` |
| B-c | control c | under Tally (4 vs 3) | band / drawn | No after Tally was minimised and restored, `answer=7` |
| B-p | product child | above Tally (1,1,0) | band / drawn, box button at x 948 | No, exit 1, stdout 0 |
| C1 (run 2) | product child | above Tally | one click on the box's button: no change, no new fit line | No, exit 1, stdout 0 |
| C2 | product child | (no click into Tally; box in front at T0+12) | one click: foreground off the box, no other change, no new fit line | No, exit 1, stdout 0 |
| C3 | product child | above Tally | right-click: no menu | No, exit 1, stdout 0 |

Every product box's error output held two `box-fit: box=(686,181,1254,926) work=(0,0,1920,1030) fits=true` lines at its end: one when it opened, one with the No click (the click activates it). No `fits=false`, no exit 77. **Child output, read from the files** (run lead's correction: spawn-parent's files mode prints `stdout_bytes_read=0` whatever the child prints): every `<cell>.stdout` is 0 B (B-p, C1, C1 run 1, C2, C3; `evidence/stdout-sizes.txt`), so no token was printed.

## Checks
- **P0:** BRIDGE BATCH LAB loaded alone (filter, Enter); one tally.exe owns 9001; `/status` 200; MARKS one row, ALTVCHID 87, ALTMSTID 224. **C10:** MARKS again 87/224, read before the shut (LAB-COMMON's order); shut; no XML after the shut; one tally.exe owns 9001; `/status` 200.
- **B1:** 10 of 10 files equal to `SHA256.txt`; BUILD-H exe a9469795... with pdfium.dll beside it; equal at the end; the request files equal at the end.
- **B2:** `box-rect-probe.ps1`: `dpi_context=ok`, `boxes=0`, the foreground, four `K` lines (tray 0,1030,1920,1080). `taskbar-capture.ps1 -Name idle`: two images, `dpi=ok`, the tray's top at 1030. All three routes (screen copy, `PrintWindow`, the screen tool) drew the six task buttons. No route dropped.
- **B3:** Tally first among the titled windows and in front (3,0,1); P-time 2.5 s; R-time 2.77 s. **B4:** one monitor, 1536 x 864 logical, 1920 x 1080 physical (125 %), tool frame 1456 x 819. Windows 10.0.19045.6466; TallyPrime 7.2 Silver.
- **B5:** none before, after part B, and at the end.

## Deviations (each with the run lead's word where noted)
1. **No screen-tool screenshot or hover while a box is open** (after B-h; run lead's word): the runner's `hover-shot.ps1` takes a DPI-aware whole-screen copy and moves the pointer (no click) to the new-button point and the PowerShell button, with a copy after each. B-h is re-run as B-h-2.
2. **Part C's gate** (run lead's word): the sheet asked for the box's button found by the screen tool or a hover thumbnail; neither works while a box is open. The button was located in each part C cell from a `PrintWindow` copy taken while its box was open (`locate-button.ps1`, the rightmost button inside the task list).
3. **C1's first run is void:** the locator first sampled the background on the tray's help icon and returned x 1018, an empty spot; the click there did nothing (kept as `c1-run1-*`). Fixed (the background is the row's commonest colour, the scan bounded by the task list's edge) and C1 re-run.
4. **The click into Tally** came at T0+17.9 to 19.5 s in part B and C1-run1 (the hover helper's time), not 16 ± 2 s; at T0+16.0 in C1 and C3.
5. **A box under Tally** (B-c): Tally minimised and restored by `ShowWindow`, not by clicks on its minimise and taskbar buttons (as in rows 1780-G and H).
6. Each `.cmd` opened through `explorer.exe` (row 1780-H's way).

## Not measured
The server path; the review, invoice-stop and ledger-create boxes; a thumbnail click (none showed); any key; a second monitor; Windows 11; more than one sitting; what draws the band (a window over the taskbar, or the taskbar not repainting).

**Confidence:** one sitting; PARTIAL.

**Who acted.** Done by the lab on the owner's standing word of 27 Oct (the title clicks, the clicks into Tally, the taskbar clicks and every No). Posting was not switched on; no server was launched. The sheet is `lab/2161-taskbar-activate` at de2b1d15c50f8d1d51eb3b8617a5ef76a6d89fc4.
