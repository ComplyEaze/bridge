# Results: #1780-G, three candidate fixes for the lost topmost bit, on the control box under the F1d timing, 29 Oct 2026. **The control box loses the bit too; MB_SYSTEMMODAL and a hidden topmost owner each keep it (twice each); re-asserting does not.**

| cell | variant | reads | first block: exstyle, topmost, fg | above Tally at T0+8, +11, +20 | answer |
|---|---|---|---|---|---|
| G1d-1 | c (the approval box's flags) | **0** | 0x00010101, 0, Tally | no (box z 4, Tally z 3, Tally foreground) | not answered: the 60 s bound |
| G1d-2 | c | **0** | 0x00010101, 0, Tally | no | No, after Tally was minimised |
| G-g | g (re-assert HWND_TOPMOST 5 times) | **0** | 0x00010101, 0, Tally | no | No, after Tally was minimised |
| G-h | h (MB_SYSTEMMODAL) | **1** | 0x00010108, 1, Tally | **yes** (box z 1, Tally z 4, Tally keeps the foreground) | No, directly |
| G-h-2 | h, repeat | **1** | 0x00010108, 1, Tally | **yes** | No, directly |
| G-i | i (hidden topmost owner) | **1** | 0x00010109, 1, Tally; owner 0x502018 | **yes** | No, directly |
| G-i-2 | i, repeat | **1** | 0x00010109, 1, Tally; owner 0x1fa1b66 | **yes** | No, directly |

**The sheet's readings:**
- **G1d-1 and G1d-2 read 0.** The control box reproduces the loss without any Bridge code, so the candidates are meaningful.
- **G-g reads 0, with `SetWindowPos=True` and the style never gaining 0x8.** The system accepts the call and ignores it: re-asserting alone is not enough.
  - The first line's exstyle (0x10101, at 08:29:33.890Z, before the trace's first block) shows the bit was already missing when the helper first saw the box.
  - Its `reassert.txt` is whole in `evidence/g-g.control.txt.reassert.txt`.
- **G-h reads 1 (twice): candidate 2 works.**
  - The box has the bit from its first block and stays above Tally while Tally keeps the foreground.
  - Its exstyle is 0x00010108 (WS_EX_DLGMODALFRAME 0x1 absent, unlike the other boxes' 0x...09).
- **G-i reads 1 (twice): candidate 3 works.**
  - The box itself has the bit (0x00010109) from its first block, owned by the hidden topmost window, and stays above Tally.
- **In none of the five 1 cells did the box take the foreground:** Tally kept it. The bit, not activation, kept the box above.

No Yes; nothing could post (no server, nothing sent to Tally). No key was pressed while a box existed. Every box but G1d-1's was answered No by mouse (`answer=7`). B5 was clean before and after.

The sheet is `lab/1780-topmost-fixes` at c0805b145adcfbc3997f35aff1fd93e4b8f1f8f6. `SHA256.txt` matched 8 of 8 before P0, and the request files were equal again at the end.

**Who acted.** The lab, on the owner's standing word of 27 Oct, with BRIDGE BATCH LAB (synthetic) loaded alone and Tally maximised on the Gateway.

## Setup and checks
- **The PC:**
  - Windows 10.0.19045.6466 22H2; TallyPrime 7.2 Silver.
  - One monitor: 1536 x 864 logical, 1920 x 1080 physical; the screen tool's frame is 1456 x 819.
- **B3:** Tally first and in front (3,0,1), maximised.
- **B3b:** FIRST, then a first block 103 ms later; PROC after the block; TOPMOST 0x00050108 `topmost=1`, PLAIN 0x00050100 `topmost=0`.
- **B5 before and at the end:** 0 processes, 0 windows.
- **C10:**
  - BRIDGE BATCH LAB shut; none loaded.
  - One tally.exe owns 9001; `/status` 200.

## Each cell
The parent tree is the same in every cell: control box ← spawn-parent's `powershell.exe` ← `cmd.exe` (the cell's `.cmd`) ← explorer. The title click came 4.09 to 4.25 s after the start, and there was no other input until No.

| cell | T0 (child_pid) | spawn-parent, its parent | box pid | FIRST, first block | probes T0+8 and +11 (box; Tally) | No | exit |
|---|---|---|---|---|---|---|---|
| G1d-1 | 08:24:29.847Z | 13240, cmd.exe 12764 | 7216 | 08:24:31.763Z, 31.851Z | 4,0,0; 3,0,1 | — | `killed_after_timeout` 08:25:29.897Z |
| G1d-2 | 08:27:42.988Z | 16508, cmd.exe 14460 | 14352 | 08:27:44.686Z, 44.756Z | 4,0,0; 3,0,1 | 08:28:33Z | 0, answer=7 |
| G-g | 08:29:32.137Z | 27472, cmd.exe 6372 | 27332 | 08:29:33.908Z, 33.972Z | 4,0,0; 3,0,1 | 08:29:52.868Z (T0+20.73 s) | 0, answer=7 |
| G-h | 08:30:30.061Z | 22716, cmd.exe 9596 | 9860 | 08:30:31.916Z, 32.000Z | 1,1,0; 4,0,1 | 08:30:50.151Z (T0+20.1 s) | 0, answer=7 |
| G-h-2 | 08:31:20.588Z | 14900, cmd.exe 21716 | 8460 | 08:31:22.229Z, 22.278Z | 1,1,0; 4,0,1 | 08:31:40.655Z (T0+20.09 s) | 0, answer=7 |
| G-i | 08:31:57.917Z | 25028, cmd.exe 26764 | 25440 | 08:31:59.518Z, 59.574Z | 1,1,0; 4,0,1 | 08:32:18.090Z (T0+20.18 s) | 0, answer=7 |
| G-i-2 | 08:32:49.517Z | 11188, cmd.exe 13028 | 20308 | 08:32:51.445Z, 51.498Z | 1,1,0; 4,0,1 | 08:33:09.582Z (T0+20.08 s) | 0, answer=7 |

- **The traces** (`evidence/trace-summary.txt`):
  - every box's `topmost` stayed as in its first block until it ended;
  - no topmost change in any trace;
  - `fg_pid` was Tally (13992) at every first block.
- **G-g's `reassert.txt`, whole:**
  - box seen 08:29:33.890Z, exstyle 0x10101;
  - reassert 1 to 5 (08:29:33.899Z to 34.764Z): `SetWindowPos=True` each time; `GetLastError` 0, then 183 four times (a stale value: the call succeeded); exstyle 0x10101 `topmost=0` each time.

## Deviations
1. **B3b's known box is a copy of row 1780-F's `control-forms.ps1`,** with the folder changed. It is not one of this sheet's files.
2. **Each `.cmd` is opened through `explorer.exe`, not typed in an interactive PowerShell.** The lab types nothing into a console. The `.cmd` holds the sheet's two lines exactly.
3. **The 0 boxes lay under Tally at T0+20, and their taskbar button is not usable here.**
   - The control box's button is grouped with the PowerShell windows, and a click opens thumbnails (rows 1780-F part R and F1d-3). UI Automation does not expose the task buttons on this PC.
   - **G1d-1:** the runner waited 30 s for a taskbar position, got none, and the 60 s bound ended the box unanswered.
   - **G1d-2:** a z-order lift (`SetWindowPos(HWND_TOP)`, no activation) returned True and was ignored. Then, by a one-off command, Tally was minimised (`ShowWindow SW_MINIMIZE`), No was clicked by mouse, and Tally was restored (`SW_RESTORE`, back to maximised).
   - **G-g:** the runner did the same.
   - **In every case this came after the readings,** and no taskbar click was made.
4. **The #899 drive search ran in the background through the sitting.** It reads the disks only.

## Not measured
- Candidates 4 (AttachThreadInput) and 5 (the child's own dialog).
- The child's own box, and a build of the product with either fix.
- Windows 11; a second monitor; more than one sitting.
- Whether MB_SYSTEMMODAL's other effects suit the product.
- Whether a hidden owner changes the box's taskbar entry.

**Confidence:** one sitting, two runs of each candidate that read 1.

## Files
- `evidence/` holds:
  - every `<cell>.txt`, `<cell>.control.txt`, `g-g.control.txt.reassert.txt` and `trace-<cell>.txt`, whole;
  - every probe file and `<cell>-timeline.txt`;
  - `b3-timeline.txt`, `forms.txt`, `trace-b3b.txt` and `trace-summary.txt`;
  - the B5 runs, `c10.txt` and `requests-end-hashes.txt`.
- `scripts/` holds `g-run.ps1` (the version used from G-g on; G1d-1 and G1d-2 ran earlier versions, described in deviation 3), `b3.ps1`, `leftover.ps1`, `taskbar-find.ps1` (a read-only lookup that found no task buttons), `trace-sum.py`, `control-forms.ps1` and `cmd/` (each cell's `.cmd`).
- Also: `substitutions.txt`, `notes.txt` (UTC) and `SCREENSHOTS.txt` (the captures, kept on the lab PC).
