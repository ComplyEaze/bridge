# Windows item 1780-G v1: three candidate fixes for the lost topmost bit, tried on the control box under the F1d timing. G1d first (does the control box lose the bit?), then, only if it does, re-assert with a check, MB_SYSTEMMODAL and a hidden topmost owner. Answered No by mouse (Part of #1780)

WRITES to the book: NONE. No Bridge server and no `bridge_mcp.exe` is started, nothing is sent to Tally, nothing is keyed. Every box is answered **No, by mouse**; nothing can post. The one click into Tally in each cell lands on a blank part of its title area.

## Why
Row 1780-F found that the dialog child started by a C# parent loses the bit too (cells F1d-2 and F1d-3 read 0x00010101) when the last input before the spawn went to Tally about 8 s earlier, and keeps it (0x00010109) when the spawn follows the click closely. So the loss comes from input recency, not from the spawner. This row asks which cheap change keeps the bit under that timing. It needs the control box (the approval box's flags, no Bridge code) to reproduce the loss first: **G1d**. If it does not, the candidates cannot be tried here (the loss then needs the child's own code and a build) and the row stops after G1d.

## Who acts
The lab does every step, including the clicks into Tally and every No, on the owner's standing word of 27 Oct 2026, and records each as "done by the lab on the owner's standing word of 27 Oct". No owner act and no posting setting is needed. The go-ahead and any STOP ruling come from the run coordinator.

## Inputs (hash before the first step; a different hash = STOP; `SHA256.txt` lists every file with its size)
- `requests/box-trace-early.ps1`, `zorder-probe.ps1`, `zorder-probe-k.ps1`: byte copies from lab/1780-topmost-spawn @ ac1d9da957856332718a912512e903a4fb3cb502.
- `requests/spawn-parent.ps1`: a copy of that file with ONE change, a `-ControlVariant` parameter (default `c`) passed as the control script's `-Variant`. Its 60-second watchdog is the only process this row ends, under the named carve-out below.
- `requests/control-box-fixes.ps1`: new, not yet run (its C# compiled on a Mac with Mono's `mcs`). Variants: `c` (the flags only), `g` (c plus a helper that, once the box exists, calls `SetWindowPos(HWND_TOPMOST, NOMOVE|NOSIZE|NOACTIVATE)` up to 5 times 200 ms apart and writes to `<Out>.reassert.txt` the return value, `GetLastError` and the box's extended style, stopping at the first time the style has 0x8), `h` (c plus `MB_SYSTEMMODAL` 0x1000), `i` (c, owned by a hidden `WS_EX_TOPMOST|WS_EX_TOOLWINDOW` window the process creates and never shows).
- Working folder `C:\bridge-lab\1780g\`; evidence in `C:\bridge-lab\1780g\evidence\`. THE COMPANY is BRIDGE BATCH LAB, loaded alone, Tally maximised on the Gateway of Tally. The `.ps1` files run from Windows PowerShell 5.1 as `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...`. A comma is a digit separator on this PC: no command takes a list.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS.** From the moment a cell is started until its box is answered, no key is pressed (a stray Y or an Up-arrow recall then Enter would press Yes; harmless here, but the habit holds), and nothing is typed. Everything that reads is started before the cell (the trace, the lab's timing helper for the probe pairs). The box is answered by mouse.
- **C2 carve-out (named).** `LAB-COMMON.md` C2 forbids ending any process. `spawn-parent.ps1` ends exactly one: the child it started itself, by the pid and handle its own `CreateProcess` returned, if still running after `-TimeoutSeconds` (60). Nothing else is ended; a box or process left after the sitting is recorded and left.
- **Leftover list (B5)** before the sitting and after the last cell: processes whose command line contains `control-box-fixes.ps1`, `spawn-parent.ps1` or `box-trace-early.ps1`, and visible windows titled "Lab topmost control". EXPECT none. Any: record, end nothing, STOP.
- **Privacy.** Before committing any evidence, search it for the user's name and for paths outside the lab folders; replace by `<user>` or `<path>` and record the edit.

## Before the sitting
P0. BRIDGE BATCH LAB is in Select Company's list and loaded alone, else STOP. B3. Probe known answer (Tally's title clicked: `zorder-probe.ps1` prints Tally first and in front). B3b. Trace known answer as in row 1780-F (`box-trace-early.ps1` while a known box is up: a `FIRST` line, a first window block within 1 s). B5 before.

## The cells
A cell is a two-line `<cell>.cmd` in the working folder:
```
powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\1780g\requests\spawn-parent.ps1 -Target control -ControlScript C:\bridge-lab\1780g\requests\control-box-fixes.ps1 -ControlVariant <V> -Stdio tokio -Out C:\bridge-lab\1780g\evidence\<cell>.txt -TimeoutSeconds 60
```
(`<V>` is the cell's variant). **Recipe**, the same for every cell (the timing of F1d-2/-3):
1. Pre-arm, each in its own window, started and waiting: `box-trace-early.ps1 -Title "Lab topmost control" -Out <evidence>\trace-<cell>.txt -Seconds 100`, and the lab's timing helper for `zorder-probe.ps1` then `zorder-probe-k.ps1` pairs at `<cell>-T0+8` and `+11`. Tally maximised and in front.
2. Start `<cell>.cmd` from the lab's interactive PowerShell. **About 4 s after the start, once its console window is up, click Tally's title area ONCE. Then no input at all** until the box is read.
3. `<cell>-T0` is the stamp on the `SPAWN ... child_pid=` line of `<cell>.txt`. Probes at T0+8 and +11 (by the helper); a screenshot after the second. If no box titled "Lab topmost control" by T0+15: record "no box", end nothing, next cell.
4. Answer **No** with the mouse on the box's No button, at about T0+20 (never Yes or Cancel). If Tally covers the box, use its taskbar button ONCE and record it. EXPECT `answer=7` in `<cell>.control.txt`.
5. Record from the trace: `FIRST`, the box's hwnd, `exstyle` and `topmost` in the first block and every later block, with the stamp of any change, and `fg_pid` at the first block. A cell reads **0** if any block shows `topmost=0` for the box, else **1**. For `g` also keep `<cell>.control.txt.reassert.txt` whole.

**Order.** G1d-1 and G1d-2 (variant `c`, run twice). **Stop here if either reads 1:** the control box does not reproduce the loss; run a third G1d to settle two different readings, and if the control box still reads 1, the row ends and the child's own code is the next question. If both read 0: G-g (variant `g`), G-h (variant `h`), G-i (variant `i`), one cell each; a candidate that reads 1 is run once more. Run B5 after the last cell.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| G1d-1 and G1d-2 read 0 | The control box reproduces the loss without the child: the candidates below are meaningful. |
| G1d reads 1 | The control box keeps the bit under this timing: the loss needs something in the child. Candidates not run. |
| G-g reads 1, `reassert.txt` shows `SetWindowPos=True` and the style gaining 0x8 | Candidate 1 works; the first line's exstyle says whether the bit was already missing when the box was first seen. |
| G-g reads 0, `SetWindowPos=True`, style never gains 0x8 | The system accepts the call and ignores it: re-asserting alone is not enough. |
| G-g reads 0, `SetWindowPos=False` | `GetLastError` names the refusal. |
| G-h reads 1 | Candidate 2 works. G-i reads 1 (or the box stays above Tally with its own bit 0) | Candidate 3 works; record which. |
| All three read 0 | None of the cheap changes helps; candidates 4 (AttachThreadInput around the call) and 5 (the child's own dialog) are next. |

**FAIL of the row:** a Yes by click or key; a box or process left after the sitting (B5); a key pressed while a box existed; B3 or B3b's known answer not met.

## Evidence (stays on the lab PC except the files listed)
RESULTS.md (one line per cell with its reading and the `reassert` lines), each `<cell>.txt`, `<cell>.control.txt`, `reassert.txt` and trace, whole after the privacy search; `SHA256.txt` over the committed files; `notes.txt` in UTC time order; `SCREENSHOTS.txt` (name, size, sha256; the captures stay local). Evidence on a new branch `lab/1780-topmost-fixes-answers`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured
Candidates 4 and 5; the child's own box (this row uses the control box); a build of the product with any fix; Windows 11; a second monitor; more than one sitting.
