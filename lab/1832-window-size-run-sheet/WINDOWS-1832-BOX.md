# Windows lab item 1832-BOX: does the approval box hold its largest text on a small screen? (#1832, Part of #1780; 23 Oct 2026). NO Tally, NO licence, NO posting.

Not for merge. Synthetic text only. The box is the real native one (MessageBoxW) opened through the executable's own child mode with the exact stdin the parent writes. Nothing is sent to Tally; the child prints a token only if the person presses Yes, and this item answers NO BY MOUSE.

## Why
#1832 added a 3-line answer text above the preview and let the box hold 27 lines (one voucher) and 43 lines (a batch of at most 10). On a Mac the 27-line box fits a 982 pt screen and the 43-line box does not (the owner could not scroll). A MessageBoxW cannot scroll. If it runs past the work area, Y still answers while buttons or lines are out of sight: a person could approve lines they cannot see. This item measures that. FAIL = fix forward before the 0.6.1 tag.

## Build (the lab downloads it itself and records the digest)
The master CI dispatch run 39905765174 (master c55cc1cbc, which contains bc5241b21bfbbf54fc43f8706fb96bf994ca8a6b = #1832): artifact `unsigned-bundle-smoke-windows-latest` id 12481365328, 55,074,580 bytes, digest sha256:6757c5174d1e6e444a87c21cebcaf581db36399168f0aff321c14d59e1a4f9f9 (the run may still show in progress; the artifact is complete). Download into an EMPTY folder, hash the zip (must equal the digest), unpack, unpack the `.mcpb` as a zip, hash `bridge_mcp.exe`: it must equal `binary_sha256` in the artifact's `mcpb-smoke.json`. Record both hashes. A different digest or hash is a STOP. If the artifact has expired, ask for a new dispatch.

## Files (check them against SHA256.txt first; a different hash is a STOP)
`inputs\` holds ten stdin files. Cases used here: `win-real-1` (a REAL one-voucher preview of 24 lines, shown 27), `win-real-10-40` (a REAL batch of 10 vouchers, preview 40 lines, shown 43), `win-real-10-39` (real, 39, shown 42), and the worst-case synthetic `win-1` (27 lines) and `win-10` (43 lines, one third of the lines at the full 100 characters). The `mac-*` files are for the Mac. `previews\` holds the three real previews as the product builds them (synthetic data from the repository's own test helpers); `make-real-inputs.py` and `make-synthetic-inputs.py` regenerate the inputs byte for byte (`python -I`, new folder).
`run-box.ps1` checks the input's hash, prints the screen and work-area sizes, opens the box with `cmd.exe /s /c "<exe> --confirm-journal < <input>"`, and when the box closes prints the exit code, the box's window rectangle (found by its title, DPI-aware), and whether the box runs past the work area. It was written without a PowerShell to test it on: run `-DryRun` first (prints, opens nothing), then one real case, and tell us if it misbehaves. If the script fails, the manual way is the same command in cmd.exe and a screenshot.

## Rules
- One box at a time. TYPE NOTHING while a box is open: it takes the keyboard, and Y answers it. Answer by MOUSE, No. Take the screenshot with the lab's own tool, not the keyboard.
- Tally may be closed (the item does not need it). If it is open it must be at the Gateway with no dialog; nothing is typed in it.
- Put the display back to what it was at the end and record the original setting.
- A screen setting the PC cannot offer is "not available", not improvised.

## Settings (Settings > System > Display), in this order; record the original first
S1 1366x768, scale 100%. S2 1366x768, scale 125%. S3 1920x1080, scale 125%. S4 1920x1080, scale 150%. (If the PC cannot do 1366x768, report which resolutions it offers and use the smallest.) Sign out/in is not needed for 125/150; if Windows asks for it, do it and say so.

## Cases, per setting
For each setting run, in this order, each with `-Setting "<setting>"`:
1. `win-real-1` (27 lines) 2. `win-real-10-40` (43 lines) 3. `win-10` (43 lines worst case) 4. `win-1` (27 lines worst case) 5. (S1 and S2 only) `win-real-10-39`.
Example: `powershell -NoProfile -ExecutionPolicy Bypass -File run-box.ps1 -Exe C:\bridge-lab\box\bridge_mcp.exe -InputDir C:\bridge-lab\box -Case win-real-10-40 -Setting "1366x768 100%"` (the folder holds `inputs\` contents flat or adjust `-InputDir`).
While each box is open, record by looking at the whole-screen screenshot: (a) the title text; (b) the first line of the text; (c) whether the LAST preview line is visible (for real-10-40 it is `After a timeout, reconcile this batch; do not rebuild or resend it.`; for real-1 and win-1 the last line is the one before the buttons); (d) whether all three buttons (Yes, No, Cancel) are visible and clickable; (e) whether any part of the box is outside the work area or under the taskbar; (f) can the box be moved to show the rest (drag the title) and does text still stop? When the box closes the script prints its rectangle: copy that line.
PASS (per case): every line visible, all three buttons visible and clickable, no part off the work area. FAIL: any line or button out of view. The coordinator rules whether a fail at 150% only blocks.
ADDED, only if a case FAILS and the buttons are out of view: do NOT press any key to test whether Y still answers (it could answer a real post later; this item has no post). Report "buttons out of view" and stop that setting.

## Hand-back
One table: setting x case: lines visible of 27/42/43 (count them), buttons visible y/n, off-work-area y/n, rectangle line, exe sha256, zip digest. Screenshots stay on the PC (list by sha256). Counted writes: none. The display setting restored. Not covered: a second monitor, a non-English Windows, Tally in front, the real parent path (needs Tally).
