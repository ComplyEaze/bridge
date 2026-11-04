# Row 2161-C v1: results (Windows 10, TallyPrime 7.2 Silver, BRIDGE BATCH LAB, BUILD-M at 187e882b)

**First line: STOPPED at B2, the idle read X0. `taskbar-uia.ps1` read `buttons=0 in_task_list=0`, while the taskbar's `PrintWindow` copy shows six running-program buttons. The sheet's EXPECT (a `B` line in the task list naming Tally, and `in_task_list` equal to the copy's count) is not met: a missing line, and counts that differ. No cell was run, no box was opened, and Q1 and Q2 are not measured.** (verified)

Sheet `lab/2161-c/RUN-SHEET.md` at cc441415 (SHA256.txt 12 of 12, by sha256 and size). Every step and stamp is in `notes.txt` (UTC).

## Before the sitting

| check | result |
|---|---|
| B1, BUILD-M | CI run <CI run id> (merge queue of #2329, head 187e882b5176a3b577d57f257a6ec4319fc2aecd, success), artifact `unsigned-bundle-smoke-windows-latest` id <CI artifact id>, 59,336,955 B: zip sha256 1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a = the digest. `bridge_mcp.exe` 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0, `pdfium.dll` and the `.mcpb` equal `mcpb-smoke.json`. Equal again at the end. (verified) |
| B1, the runner | `scripts/cell.ps1`, sha256 ba0003a967f6f5b8aa35483196b63ec9906f0b9eff27526187bcc9c4ead2f627. PowerShell parser errors 0. Case-insensitive scan: 99 variable names, none spelled two ways. (verified) |
| P0 | BRIDGE BATCH LAB loaded alone (Select Company, its name typed as a filter, one row, Enter), GUID a30f23c3-4819-4334-96ab-ffa6fba70976. ALTVCHID 87, ALTMSTID 224. One tally.exe on 9001, `GET /status` 200. (verified) |
| B4 | 1536x864 logical, 1920x1080 physical, 125 %. One monitor. Windows 10 build 19045. Tally maximised (physical 0,0,1920,1030). (verified) |
| B5 before | No process, window or minimised recorded console. (verified) |

## B2, the idle read X0 (the stop)

Tally's title was clicked by the runner, and Tally was in front. One read set X took 9.6 s.
- **`uia-x0.txt`, whole:** `E … dpi_context=ok tray=0x20138` and `Z … buttons=0 in_task_list=0`. There is no `B` line at all. (verified)
- **`x0-print.png`** (local, by hash in `SCREENSHOTS.txt`): six running-program buttons in the task list, for File Explorer, the owner's browser, a PowerShell-type icon, one more application, TallyPrime and the desktop client. (verified)
- **`cover-x0.txt`:** `dpi_context=ok`, the `T` line (tray z=13, 0,1030,1920,1080) and the `W` lines, as expected. Its z=1 `W` line is the desktop client's own window: full screen, topmost, `above_tray=1`, exstyle 0x08080028. (verified)
- **`rect-x0.txt`:** Tally's `F` line, the `K` lines with `MSTaskListWClass` at 480,1030,1261,1080, and `boxes=0`. (verified)

## A read-only check of the cause, after the stop (not a step of the sheet)

- **The same script, run from the lab's PowerShell tool** (an STA thread), read `buttons=0 in_task_list=0` again (`uia-x0-diag-pwsh.txt`). (verified)
- **A managed `System.Windows.Automation` walk of `Shell_TrayWnd`** (all descendants, read-only; `uia-walk-diag.txt`) lists 19 elements, all `ControlType.Pane` window elements: Start, the search box, Task View, `ReBarWindow32`, `MSTaskSwWClass` and `MSTaskListWClass` ("Running applications"), and the notification area. `MSTaskListWClass` has 0 children. (verified)
- **Hypothesis, unverified:** .NET's managed UI Automation client sees the Windows 10 taskbar only through its Win32 window proxy, and does not reach the task list's own UI Automation buttons. If so, `taskbar-uia.ps1` reads 0 on this PC whatever is open, and a COM `IUIAutomation` read would be needed. (hypothesis)

- **Bidi marks removed (forward commit):** the clock's name in `uia-walk-diag.txt` (line 19, `TrayClockWClass`) held three U+200E left-to-right marks that Windows puts in the date. They were removed, and nothing else in the file changed. The file's sha256 in `SHA256.txt` is refreshed. (verified)

## Close (C10)

- **MARKS while loaded:** ALTVCHID 87, ALTMSTID 224, the answer byte-equal to P0's. (verified)
- **B5 at the end:** nothing left over. Every input re-hashed equal. (verified)
- **Before the shut:** the owner's browser held the foreground. Tally was brought forward by `SwitchToThisWindow` on its own window, and nothing was done to the browser. (verified)
- **The shut:** Company > Shut gave Select Company with none loaded. One tally.exe, 9001 LISTENING, `GET /status` 200. No XML request after the shut. (verified)

## Not measured

Q1 (a taskbar button for the box) and Q2 (the box in Alt+Tab): no cell ran. Step HA did not run. Nothing else on the sheet's own "Not measured" list was measured either.
