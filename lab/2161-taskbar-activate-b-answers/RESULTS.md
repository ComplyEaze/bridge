# Results: #2161-B, what covers the taskbar while an approval box is open, and one taskbar activation of the box, 29 Oct 2026 (UTC). **Q1: the band is the lab's own trace console.** In N2 (no box, the helpers only) and in P (a box open, consoles as they open), the trace's Windows PowerShell console lay above the taskbar in the stacking order, uncloaked, its rectangle over the task list, and the screen copy showed the dark band exactly over that rectangle. In P-min (the trace and `.cmd` consoles minimised) no console met the task list, and the screen copy drew every task button, the box's included. **Q2 (C1):** with the box above Tally and Tally in front, one left click on the box's own task button (after the thumbnail gate) brought the box to the front, not minimised, and the fit check wrote one new `fits=true` line; No then gave exit 1 and a 0-byte stdout. **Q3 (C2) and Q4 (C3) were not measured:** the hover showed no thumbnail for the box, so by the gate nothing was clicked on the taskbar and each box was answered No (exit 1, 0-byte stdout).

## Reading (the sheet's table)
- **P with a recorded console over the task list and the band; P-min with no console there and the buttons drawn:** yes. "The band is the lab's own console window (the lab's hypothesis). A person's screen has no such console: the band is a lab artefact."
- **N2 shows the band, with the same console over the task list:** yes. The helpers alone cause it, with no box.
- **C1: the box still open, one new `fits=true` line:** "A taskbar activation of a box above Tally did no harm in this sitting (one run, partial)."
- **C2, C3:** "`point_ok=1` at the button point, but no thumbnail": Q3 and Q4 not measured (cite: the `P` lines `point_ok=1` at (948,1055); hover copies `c2-gate1-hover.png`, `c3-gate1-hover.png`, local).

## Q1: the windows over the taskbar (cover reads; z 1 is the top; the taskbar `Shell_TrayWnd` rect 0,1030,1920,1080)
| read | taskbar z | windows with `above_tray=1` | screen copy of the taskbar | PrintWindow copy |
|---|---|---|---|---|
| B2 / N1 idle | 4 | z1 `Chrome_WidgetWin_1` <desktop client> topmost=1 exstyle=0x08080028 cloaked=0 rect 0,0,1920,1080 | buttons drawn (a faint tint) | 6 buttons: x 513 576 638 700 762 824 |
| N2 at +12 s (no box) | 6 | z1 the desktop client's overlay (as above); **z5 `ConsoleWindowClass` powershell.exe (the trace console) exstyle=0x000c0110 cloaked=0 rect 96,96,1575,1126 title "Windows PowerShell"** | **band from about x 105 to 1545** (the console's rect) | 7 buttons: + 885 (the `.cmd` console) |
| P at T0+6 and C+3 | 7 | the overlay; **z6 the trace console, rect 160,160,1639,1190** | **band from about x 168 to 1608** | 8 buttons: + 947 (the box) |
| P-min at T0+6 and C+3 | 7 | the overlay only; both recorded consoles `iconic=1` | **buttons drawn**, the box's included | 8 buttons: 513 ... 885 948 |

- **The band's owner:** the trace's Windows PowerShell console (`ConsoleWindowClass`, powershell.exe, exstyle 0x000c0110, title "Windows PowerShell"), lying above the taskbar in the stacking order with its window over the task list. The `.cmd` console never met the taskbar's rectangle in these reads.
- **The desktop client's window** (`Chrome_WidgetWin_1`, topmost, exstyle 0x08080028: no-activate, layered, transparent) covers the whole screen above the taskbar in every read, the idle ones and P-min included, where the buttons are drawn: it is not the band. It is the lab operator's own client, present on a lab screen only.
- **The box's button:** in P-min's PrintWindow copy the one button in neither N1's nor N2's: x 947/948 (exactly one). Part C ran.

## Cells (times UTC; `evidence/<cell>-timeline.txt` has every line)
| cell | T0 | click into Tally | box at T0+6 | taskbar | box-fit lines | answer | exit / stdout |
|---|---|---|---|---|---|---|---|
| P | 19:00:21.400 | T0+20.71 s, Tally in front | open, topmost, above Tally (1,1,1) | none | 2 (the 2nd at the No click) | No at C+13 | 1 / 0 B |
| P-min | 19:02:17.596 | T0+16.40 s, Tally in front | open, above Tally | none | 2 | No at C+11 | 1 / 0 B |
| C1 | 19:03:39.051 | T0+18.70 s, Tally in front | open, above Tally | gate: `point_ok=1` at (948,1055); one thumbnail, title shown cut to "ComplyEa..." with the box as its preview: **one left click at T0+54.17 s: the box in front, iconic 0**, +3 s the same | 1 before, **2 after the click (one new `fits=true`)** | No | 1 / 0 B |
| C2 | 19:05:20.961 | none (C2) | open, in front at T0+12 (yes) | gate: `point_ok=1`; **no thumbnail**: no click | 1 | No | 1 / 0 B |
| C3 | 19:06:37.915 | T0+17.21 s, Tally in front | open, above Tally | gate: `point_ok=1`; **no thumbnail**: no right-click | 2 (the 2nd at the No click) | No | 1 / 0 B |

Every `box-fit:` line read `box=(686,181,1254,926) work=(0,0,1920,1030) fits=true`. The box's No button was clicked at its centre, physical (1063,886) = logical (850,709); the box was above Tally each time, so Tally was never minimised. No exit 77, no `fits=false`.

## Checks
- **Sheet:** `lab/2161-taskbar-activate-b` at 996f869ffa4eec7c4a9be53a27b89ea50d1b57b0; `SHA256.txt` 10 of 10 equal (sha256 and size) before and at the end; the `requests/` copies equal to the sheet's. **BUILD-H** `bridge_mcp.exe` sha256 a946979561b61f51e6dab3539d3de98fd48c8f103f61d2e3abe787acf3240c23, `pdfium.dll` beside it, before and at the end.
- **B1:** the runner `scripts/cell.ps1`; case-insensitive scan for a variable spelled two ways: none (both versions, see Deviations).
- **B2:** `taskbar-cover.ps1` idle: `dpi_context=ok`, the `T` line, four `W` lines. At Tally's task button (762,1055), from the idle PrintWindow copy: `point_ok=1` (`MSTaskListWClass`, root `Shell_TrayWnd`). At the Gateway's middle (960,540): `point_ok=0`, Tally's window (`Ganeshji`, tally.exe).
- **B3:** Tally's title clicked; `zorder-probe-k` printed Tally first and in front (3,0,1); P-time 2.8 s, R-time 1.69 s.
- **B4:** one monitor; logical 1536x864, scale 125 % (120 dpi), physical 1920x1080. Windows 10 22H2, build 19045.6466. TallyPrime 7.2 Silver (title band).
- **B5** before, after P-min and at the end: no process, no window, no recorded console left.
- **P0 / close:** BRIDGE BATCH LAB (100050, guid a30f23c3-4819-4334-96ab-ffa6fba70976) loaded alone by its name filter; MARKS one row, ALTVCHID 87, ALTMSTID 224; at the close (read before the shut) 87 / 224, byte-identical answers. C10: shut from the Company menu; none loaded; one tally.exe owns 9001; `/status` 200; no XML after the shut.
- **Keys:** none in Tally from B2 to C10, none while a box was open; every box answered No by mouse; no Yes.

## Deviations
1. **The click into Tally in P came at T0+20.71 s**, outside 16 +- 2 s: the first runner version made a whole read R before it. The runner was changed to the cover check alone, started at T0+13 (the version committed). After that: P-min T0+16.40, C3 T0+17.21, **C1 T0+18.70 (0.7 s late: the T0+6 reads took 6.4 s)**.
2. **The C+3 read and capture took 9 to 10 s,** so P's and P-min's No came at about C+13 and C+11, and the C1 and C3 gates began at about C+10 and C+11, not C+6. The gate's wait for the lab's thumbnail word added 18 to 36 s (C1's click at T0+54 s).
3. **P-min and part C:** at 2.5 s the `.cmd` console was the foreground window, so by the rule it was not minimised then; it was minimised 0.7 s after the second title click (still in the 8 s sleep, before T0). The order "before the second title click" was not met; the foreground rule was.
4. **The screen tool once while a box was open (a lab error):** at about 19:07:25Z, while C3's runner waited for the gate word with the box open, the lab took one screen-tool zoom (a 1x1 px region) by mistake, in place of opening the saved hover copy. A probe at 19:07:28-30Z read the box open, `iconic=0`, its rectangle unchanged, Tally in front (`evidence/rect-c3-after-screen-tool.txt`). The gate's hover copy (19:07:13Z) was taken before it, and C3 clicked nothing on the taskbar.
5. **The thumbnail check** was made by the lab reading the runner's hover copy (the runner paused for the word). C1's thumbnail title was drawn cut to "ComplyEa..."; it was read as the box's (one thumbnail, its preview the box).
6. **N1 is not "only Tally and the lab's console":** the owner's other windows (the desktop client, a browser, a cloaked Media Player) stayed as they were. The runner itself had no window (started hidden by the lab's tool), so "the runner's own console" is a pid only.
7. **Two runner versions:** P ran on sha256 99957e591462b633c2a8f026909ebbba71babc5fa1fd7ed2cbe56d99638f9a94; the change in deviation 1 gave 815729f153ed0ea185507a2e97940b9e2020347d457e0383d1d2219576453410, which ran P-min and part C and is the one committed.

**Who acted.** Every title click, click into Tally, taskbar hover and click and every No was done by the lab on the owner's standing word of 27 Oct. Posting was not switched on.

**Confidence:** one sitting, one monitor, Windows 10: PARTIAL.

## Not measured
The server path (`post_import`); Q3 (C2: no thumbnail); Q4 (C3: no thumbnail); the review, invoice-stop release and ledger-create boxes; any key; Alt+Tab; a second monitor; a scale change while a box is open; Windows 11; more than one sitting.
