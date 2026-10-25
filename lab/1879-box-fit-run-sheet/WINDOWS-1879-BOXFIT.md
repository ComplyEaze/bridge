# Windows lab item 1879-BOXFIT: does the show-time box-fit gate behave on a real Windows box? (#1879 / #1780 / #1832; B1 to B7; 25 Oct 2026). NO Tally, NO display change, NO key.

Not for merge. Synthetic text only. Nothing is sent to Tally: the box is the real one (MessageBoxW) opened through the executable's own child mode with the stdin the parent would write. The child only shows a box and prints a token to its stdout if Yes is clicked; nothing connects to Tally and nothing is posted. This item answers by MOUSE only (Yes where the case says so, otherwise No, Cancel or the X). Type nothing while a box is open: Y answers it.

## Why
The new gate installs a WH_CBT hook on the dialog's thread, measures the box's real rectangle against the work area of its monitor when Windows activates it, and, if the box is not wholly inside, cancels it and ends the dialog with exit status 77 (the parent then refuses with "too large"). It has compiled in Windows CI but has never run on a real Windows. Four things are unproven, and the first two decide whether any Windows post can be approved at all:
1. The hook fires, so a box that fits is answered normally (a Yes prints the token). If the hook never fires, every box ends "not measured" (exit 0, no token) and nothing can be approved.
2. The hook sees the box's FINAL size and position (activation after layout). If it fires early, a too-tall box is measured as fitting (today's behaviour).
3. A box taller than the screen is cancelled at once, with exit 77 and no token, and is not left open.
4. The units agree: the product is DPI-unaware, so its window rectangle and the monitor's work area are both read in virtualised units; a box that fits by the lab's physical numbers must still count as fitting, at the edge (48 lines: bottom 1023 of 1030) as well as well inside.

## Build
The `bridge_mcp.exe` from the Bridge CI pull_request run for PR #1879 at the head named below, artifact `unsigned-bundle-smoke-windows-latest` (the PR run builds the Windows bundle; if the artifact is missing for that head the item does not start). Filled in by whoever relays this sheet, before it goes to the lab:

- head sha: ______  run id: ______  artifact id: ______  zip digest: ______  `bridge_mcp.exe` sha256: ______

The lab records the hash of the file it stages and compares it with the line above; a different hash, or a head that is not the PR's current head, is a STOP. Stage it; install nothing.

## Files (check them against SHA256.txt first; a different hash is a STOP)
`make-probe-inputs.py` (regenerates `inputs\` byte for byte, `python -I`, new folder); `inputs\probe-40|44|48|60|80|120.txt` (the six boxes of the earlier clipping item: Windows wording, 40 to 120 displayed lines, a third of the lines at the full 100 characters); `run-fit.ps1` (opens the box, prints the window rectangle against the work area, then how the child ended: exit status, milliseconds, what it printed to stdout). It was written without a PowerShell to test it on: run `-DryRun` first (prints, opens nothing); if it misbehaves use the manual fallback (open `cmd`, run `"<exe>" --confirm-journal < <input> > <out>`, then `echo %ERRORLEVEL%`) and say so. There are no Tally request files in this item, so there is no request-ID preflight; the preflight is the hash check and the dry run.

Each run: `powershell -NoProfile -ExecutionPolicy Bypass -File run-fit.ps1 -Exe <path>\bridge_mcp.exe -InputDir <path>\inputs -Case probe-44 -OutDir <NEW folder per case>`. Take a WHOLE-SCREEN screenshot with the lab tool when the script says so (not the keyboard).

## The rows, in this order (the PC's own display setting; change nothing)
Expected on this PC (1920x1080 at 125 percent, work area 1030 high): probe-44 and probe-48 fit (the earlier item measured 48 lines with its bottom at 1023); probe-60 and above do not. Those are predictions from that item; record what happens.

- B1 `probe-44`, answer **Yes by mouse**. Expect the box to appear whole (top and bottom on screen, all three buttons visible), `cmd exit=0`, stdout `bridge-post-approved:` followed by the nonce on the first line of the input file, then a line end. Record: the script's window line; whether the box came to the front of the other windows without a click (the child is started from a background shell); exit; stdout bytes and text. If exit is 0 with NO token printed after a Yes: the hook did not fire (thing 1), STOP the item, say so.
- B2 `probe-44`, answer **No by mouse**. Expect exit 1, stdout empty (0 bytes).
- B3 `probe-48`, answer **Yes by mouse**. The edge case: expect the same as B1. If exit is 77 with a box that your screenshot shows wholly on screen: record the rectangle, the work area and the exit, and say the units disagree (thing 4).
- B4 `probe-60`: click NOTHING. Expect the box to be closed by the product within a second: `cmd exit=77`, elapsed well under 10 s, stdout 0 bytes. Record: elapsed ms; whether you SAW the box on screen at all (flash) and for how long (a number of frames if the lab tool records video; otherwise "seen / not seen / cannot say"); the script's window line (it may say no visible window, which is a result). If the box stays open for 10 s: that is thing 3 failing; close it by MOUSE (No, Cancel or the X) or the printed `taskkill /PID <cmd pid> /T /F`, record the rectangle and say the hook did not cancel it.
- B5 `probe-80` and B6 `probe-120`: as B4.
- B7 `probe-40`, answer **Yes by mouse**: as B1 (a small box, well inside).

STOP rules: any box other than the probe's; a second box; a Tally window in front; typing anything while a box is open; a box that cannot be closed by mouse or by the printed taskkill; an exe hash that differs from the line above. Counted writes: none. Display setting unchanged. Not counted as evidence: any run after a STOP.

## Optional rows (only if the owner has said the lab PC's display scale may be changed and put back; otherwise write "not run: display change not allowed")
- B8 the same B1 and B4 at 100 percent and at 150 percent scale (sign out and in as Windows asks), and B9 the same on a second monitor with a different scale if one is attached. Put the scale back and say what it is. These are the other-scaling rows the gate's authors could not run.

## What to send back
One results file: the build line, the SHA256.txt check, and for every row the script's output (screen, workarea, window line, `cmd exit=`, stdout line), the screenshot name, and the answers above in one line each. Say plainly what you did not run.

## What the results decide
- B1, B2, B3, B7 pass and B4, B5, B6 give exit 77 with no visible box (or a flash the person could not read): the gate works as built; it is evidence for the PR's lab row.
- B1/B3 give exit 0 with no token after Yes: the hook never fires on this setup; the gate makes every Windows approval impossible and cannot ship as is.
- B3 gives 77 for a box that is wholly on screen: the units or the work area differ from what the code assumes; a slack or a different rectangle is needed.
- B4 to B6 leave the box open: activation is not the right moment, or the Cancel is not delivered; the gate does not protect and the PR does not merge.
