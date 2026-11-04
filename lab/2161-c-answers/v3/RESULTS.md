# Windows item 2161-C v3: results

**Q1 (taskbar button): none added for the box, in C1 or in C2, in any of the three reads.**
- The UI Automation read lists the same six task-list buttons as part N's, with no "ComplyEaze Bridge" B line.
- The cover read has no `bridge_mcp.exe` window meeting the task list.
- The `PrintWindow` copy shows the same 7 task buttons as N's, none added.

**Q2 (Alt+Tab): not measured in C1 or C2.** In both cells the box opened topmost and held the foreground, so step HA's gate (Tally in front) was not met and HA was skipped: no key was pressed. In part N, HA ran and the switcher listed no "ComplyEaze Bridge" tile, but N has no box. (verified)

**Reading (the sheet's table):** Q1 reads as "the owned box has no taskbar button" in both cells. Q2 reads as "HA skipped or not run in a cell: Q2 is not measured in that cell". The sheet's closing row, "#2161 closes on #2193", needs both halves, so it is **not reached**: Q1 only. (verified; the reading is the sheet's)

The sheet is `lab/2161-c-sheet` at 6f918e8b; SHA256.txt is 13 of 13 equal, and the files were re-hashed equal at the end. Every stamp is in `notes.txt` (UTC). All writes to the book: none. Requests to Tally: C2's MARKS and GET /status at P0, MARKS before the shut, and GET /status after it.

## Before the sitting

| step | result |
|---|---|
| P0 | BRIDGE BATCH LAB loaded alone; Gateway period 1-Apr-26 to 31-Mar-27, Current Date Friday 21-Aug-2026. One tally.exe (pid 1200) owns 9001; GET /status 200. MARKS: ALTVCHID 87, ALTMSTID 224, 3,394 B, sha256 566be20d…, equal to v1's and v2's P0. (verified) |
| B1 | **BUILD-M:** zip 1940bd97… = the digest. `bridge_mcp.exe` 9bda7144… and `pdfium.dll` 79d4676b… are both in `mcpb-smoke.json`.<br>**Runner:** `cell.ps1` ba0003a967f6f5b8aa35483196b63ec9906f0b9eff27526187bcc9c4ead2f627 (v1's, unchanged). 0 parser errors; 97 distinct names, none spelled two ways (`scripts/b1names.ps1`). (verified) |
| B2 (x0) | **UIA read:** `dpi_context=ok`, `client=com8`; Z `buttons=20 in_task_list=6`. The task list holds File Explorer, Brave, Windows PowerShell, Snipping Tool, TallyPrime and the desktop client.<br>**`PrintWindow` copy:** 6 task buttons, the same six.<br>**MET:** the known answer for the read. The cover read has the T line and 4 W lines. (verified) |
| B3 | Tally's title clicked; G-P shows Tally first and in front. P-time 1.43 s; X-time 9.28 s. (verified) |
| B4 | Logical 1536×864, physical 1920×1080, scale 125 %, one monitor. Windows 10 Home Single Language 10.0.19045. Tally maximised. (verified) |
| B5 | Before, after N1, after N2, after N3, after C1 and at the end: processes 0, windows 0, minimised recorded consoles 0. (verified) |

## Part N (the cell with no box)

| run | what happened |
|---|---|
| N1 | **The run:** HA ran: gate met, Alt down, Tab once, held 12.05 s, Esc, Alt up.<br>**The switcher:** watcher A and C lines written. Tiles: TallyPrime:9001, cmd.exe, Windows PowerShell ×2, a browser tab, the desktop client. No "ComplyEaze Bridge" tile.<br>**The control shot:** one desktop-tool screenshot taken during the hold, on the run lead's word.<br>**Miss 1:** **G and F showed the desktop client's window in front, not Tally.** The sheet expects Tally.<br>**The stop:** stopped before C1 and reported. (verified) |
| N2 | **A lab slip, not evidence.** Its precondition, Tally in front at the cell's start, was not set: before N1 the lab had set it by SwitchToThisWindow, and before N2 it did not. The desktop client's window held the foreground through the runner's title clicks, so HA was skipped. No key was pressed and the switcher never opened. (verified) |
| N3 | **The setup, on the run lead's word:** N1's start restored by `front.ps1` (SwitchToThisWindow on Tally's own window only). No desktop-tool screenshot at any point; the copies before and after were `PrintWindow` captures of Tally's window, and they are byte-equal.<br>**The run:** HA ran: gate met, Alt, Tab, held 12.03 s, Esc, Alt up.<br>**The switcher:** watcher A, C and L lines written; the copy shows the switcher. Tiles: TallyPrime:9001, cmd.exe, Windows PowerShell ×2, the desktop client, a browser tab. No "ComplyEaze Bridge" tile.<br>**The result:** **G 09:04:54.841Z and F 09:04:55.367Z: TallyPrime:9001 in front (EXPECT met).** The watcher meets its known answer. (verified) |

**Miss 1, by the run lead's rule:** it is put down to N1's control shot, at confidence **partial** (one repeat, one variable changed). (verified; the attribution is the run lead's rule)

**Miss 2, observed in N1, N2 and N3:** the UIA read gives `in_task_list=6` with no Windows PowerShell line. The task list holds File Explorer, Brave, Snipping Tool, TallyPrime, the desktop client and "Command Prompt - 1 running window". The `PrintWindow` copy of the same cell shows 7 task buttons, Windows PowerShell among them. No cause is given. C1 and C2 show the same six against the same seven, so Q1 is read by comparing cell with cell. (verified)

## Part C (the product's box)

| | C1 | C2 |
|---|---|---|
| T0 (SPAWN child_pid) | 09:08:11.941Z | 09:10:36.018Z |
| trace FIRST | 09:08:12.250Z | 09:10:36.168Z |
| the box | hwnd 0x11c27aa, class #32770, exstyle 0x00010108, topmost=1, rect 686,181,1254,926 | hwnd 0x14b29ca, the same class, exstyle, topmost and rect |
| owner | 0x287277a, owner_class `Static`, owner_pid the child, owner_exstyle 0x00000088 (WS_EX_TOPMOST and WS_EX_TOOLWINDOW), owner_topmost=1; the owner is not visible | 0x3f51bf6, the same class, pid, exstyle and topmost |
| G-P t8 | the box first and in front, topmost; Tally z 2, not in front | the same |
| UIA (X) | `in_task_list=6`, the same six names as N3's; no "ComplyEaze Bridge" B line; no B line not in N3's | the same |
| cover (X) | the T line; no W line of `bridge_mcp.exe`; none meeting the task list | the same |
| `PrintWindow` copy | 7 task buttons, the same as N3's; none added | the same |
| step HA | skipped: Tally did not hold the foreground (the box did); no key pressed | the same |
| watcher | N lines for the cmd console and the box; no switcher (appearances 0) | the same |
| G-P t46 | the box first and in front | the same |
| No by mouse | 09:09:02.009Z (T0+50.01 s), at the No button's centre (physical 1063,886, logical 850,709) | 09:11:26.106Z (T0+50.02 s), the same point |
| child | exit 1, stdout 0 B, 50.1 s | exit 1, stdout 0 B, 50.1 s |
| `box-fit:` (stderr, whole) | `box-fit: box=(686,181,1254,926) work=(0,0,1920,1030) fits=true` | the same |

## Close

- B5 at the end: 0 / 0 / 0.
- Inputs re-hashed equal.
- MARKS before the shut: byte-equal to P0's (ALTVCHID 87, ALTMSTID 224).
- Shut Company (Alt+K, Down ×4 to "SHut", Enter, Enter on BRIDGE BATCH LAB, the only row): Select Company, none loaded.
- After the shut: one tally.exe owns 9001, and GET /status 200. No XML request after the shut. (verified)

Every key and the No click were done by the lab on the owner's standing word of 27 Oct. No Yes was given and no taskbar click was made. Nothing was minimised except recorded consoles before T0.

## Not measured

- The server path (`post_import`).
- The other boxes that share the owner code.
- Alt+Tab with a box open (Q2 here).
- More than one Tab, or Win+Tab.
- A second monitor, Windows 11, or more than one sitting.
- Whether a person notices a box with no button.

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
