# Windows lab item 1832-CLIP: what does the native approval box do when its text is taller than the screen, and is the product DPI-aware? (#1832 / #1780; W1 and W2; 25 Oct 2026). NO Tally, NO display change, NO key.

Not for merge. Synthetic text only. Nothing is sent to Tally. The box is the real one (MessageBoxW) opened through the executable's own child mode with stdin the parent would write; the child prints a token only if the person presses Yes, and this item answers by MOUSE only (No, Cancel or the X), so nothing is ever approved.

## Why
The owner asked whether the box could size itself to the screen or scroll. The adversary-checked survivor measures the REAL box when it opens (rectangle against the monitor's work area) and cancels it if it does not fit. Before that is built, two facts decide its shape: (W1) is `bridge_mcp.exe` DPI-aware or not (the default Tauri manifest declares no DPI awareness; unverified for the built exe), and (W2) what Windows does with a message box whose text is taller than the screen: clip the text and keep the buttons, shrink the box, or let it run off the screen with the buttons out of reach. A key must never be used to find out (Y answers the box).

## Build
The exe of the scheduled Bridge CI run 40077056130 (master 2306d86df), artifact `unsigned-bundle-smoke-windows-latest` id 12553782422, zip digest sha256:74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e; `bridge_mcp.exe` sha256 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524 (as the 1832-BOX item recorded). Any newer bundle is fine if its `bridge_mcp.exe` hash is recorded; the box code must contain #1832 (first line of the text). Stage it; install nothing.

## Files (check them against SHA256.txt first; a different hash is a STOP)
`make-probe-inputs.py` (regenerates `inputs\` byte for byte, `python -I`, new folder); `inputs\probe-40|44|48|60|80|120.txt` (Windows wording; 40 to 120 displayed lines; a third of the lines at the full 100 characters); `dump-manifest.ps1` (W1); `run-probe.ps1` (W2). The scripts were written without a PowerShell to test them on: run each `-DryRun`/first; if one misbehaves use the manual fallbacks below and say so. There are no Tally request files in this item, so there is no request-ID preflight to run; the preflight is the hash check and the dry run.

## W1: DPI awareness (no box needed for W1a)
W1a. `powershell -NoProfile -ExecutionPolicy Bypass -File dump-manifest.ps1 -Exe <path>\bridge_mcp.exe`. Record the whole output: the exe hash, the number of manifests found, each manifest's text, and the RESULT line (does the file contain a dpiAware / dpiAwareness setting). Hand back the manifest text(s) in full (they hold no private data).
W1b. While a W2 box is open (any case), open Task Manager (by mouse), Details tab, right-click a column header, Select columns, tick "DPI awareness", find the row `bridge_mcp.exe` that belongs to the probe (the one started by the script; its command line has `--confirm-journal`: add the "Command line" column), and record its DPI awareness value (Unaware / System aware / Per-monitor aware). Screenshot it.

## W2: a box taller than the screen (the PC's own display setting: change nothing)
Run in this order, each with `-Case`: `probe-40`, `probe-44`, `probe-48`, `probe-60`, `probe-80`, `probe-120`. Example: `powershell -NoProfile -ExecutionPolicy Bypass -File run-probe.ps1 -Exe C:\bridge-lab\box\bridge_mcp.exe -InputDir C:\bridge-lab\box\inputs -Case probe-80`. Each run: the script checks the input's hash, prints the screen and work area, opens the box, finds its window by process id and prints its rectangle and four flags (top off screen, bottom below the work area, left off, right off); then take a WHOLE-SCREEN screenshot with the lab's own tool and answer by MOUSE.
For each case record, from the script line and the screenshot:
(a) box size (width x height) and rectangle; is the top (title bar) on screen; is the bottom edge on screen;
(b) is ALL the text visible (count the last visible line `P0NN`), or is it clipped at the bottom/top, and where does Windows cut it;
(c) are the buttons (Yes, No, Cancel) visible, and where (on screen, below the work area, hidden behind the taskbar), and are they reachable by the mouse;
(d) did Windows shrink the font or the box, or change the layout, compared with the 44-line box;
(e) how you closed it (No / Cancel / X by mouse) and the script's `cmd exit=` line; if NO button or X was reachable, the printed `taskkill /PID <cmd pid> /T /F` from another window (that kills only this probe); say which.
The expected boundary on this PC (1920x1080 at 125%, work area 1030 high) is near 44 to 48 lines: probe-44 should just fit and probe-48 should not. That is a prediction from two earlier boxes, not a fact; record what happens.
STOP rules: any box other than the probe's; a second box; a Tally window in front; typing anything while a box is open; a box that cannot be closed by mouse or by the printed taskkill. Counted writes: none. Display setting unchanged. Not covered: other scales, a second monitor, Tally in front, the real parent path.

## What the results decide (for the owner's five decisions)
- A box that CLIPS its text but keeps the buttons: Y could approve unseen lines; the show-time gate is needed. A box that RUNS OFF the screen with the buttons out of reach: nobody can approve (fails closed by accident, 120 s timeout), still a usability fault. A box that SHRINKS or re-lays itself out: the gate may be unnecessary on Windows. W1 decides whether the gate's numbers are physical or logical pixels.
