# Results: #1780-F, which part of the spawn context makes the approval box lose its topmost bit, 29 Oct 2026. **Every lab parent kept the bit; only the server's own box lost it. Go on to D.**
- **Every lab cell read 1:** F0 to F4, G1 to G3 and T, each with the box's extended style 0x00010109 (`topmost=1`) in every trace block.
  - **All three parent kinds:** interactive, hidden and the lab's launcher.
  - **Both stdio shapes:** tokio's pipes and file redirects.
  - **Both boxes:** the dialog child and the control box.
- **C1 read 0 again (part R):** the box spawned by the running server read 0x00010101 (`topmost=0`) from its first block, as in row 1780-E.
- **The sheet's reading:** "F0 to F4 all read 1": the lab's parents do not reproduce C1. **Go on to D.**
- **Part D: not run: needs the owner at the PC.**
- **R1:** no field the reader reads separates C1's child from the cells that read 1.
- **Added cell F1d (forward commit): a lab parent loses the bit too, when Tally had the last input.** F1's exact command, run after an 8 s wait, with Tally's title clicked once after the parent's console window was up and no input until the box: **F1d-2 and F1d-3 both read 0** (0x00010101 from the first block), with the same spawn context as F1, which read 1. The first try, F1d, read 1, but its Tally click came before the console window opened, so the console held the foreground. By the requester's reading, a 0 here puts the cause on the foreground lock or input recency, not on the spawner. See "Cell F1d" below.

Nothing was posted. The marks were 224/87 before and after. Every box but one in parts F, G and T was answered No by mouse. Part R's box ended when its taskbar thumbnail was clicked, with neither No nor Yes pressed. F1d-2's and F1d-3's boxes were ended by spawn-parent's own 60 s bound (the sheet's carve-out), unanswered. No key was pressed while a box existed.

The sheet is `lab/1780-topmost-spawn` at ac1d9da957856332718a912512e903a4fb3cb502. `SHA256.txt` matched for every file before P0.

**Who acted.** The lab did every click into Tally and every No, on the owner's standing word of 27 Oct, on the synthetic company BRIDGE BATCH LAB (100050). Posting was on for part R only, on the owner's word of 28 Oct, and was turned off after it.

## Build and setup
- **BUILD-T:** #2107 at a66c31f2ab607b36701ea0474ca15d474b9e202c, CI run 40709511734, artifact 12826362269.
  - mcpb: 15,320,736 B, sha256 61e52d07…99c2.
  - `bridge_mcp.exe`: 31,804,416 B, sha256 3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974.
  - Equal at the start, after part R and at C10. The server reports bridge-tally 0.6.1.
- **The PC:**
  - Windows 10.0.19045.6466 22H2; TallyPrime 7.2 Silver; desktop client 3.6556.2.
  - One monitor: 1536 x 864 logical, 1920 x 1080 physical (125 %); the screen tool's frame is 1456 x 819.
- **BK0:** `TDBK1800_100050.001`, 57,180 B, sha256 40fcf481a75dcf9ab3fb020f4b30259cdfd9ce561522708a23a2bb657a802c9e.
- **The launcher's command line:** `python -I <lab folder>\launcher.py <cell> <spawn-parent arguments>`.
  - It keeps client.py's environment rule, working folder and pipes; only its command is replaced (`launcher.py`; each run's line is in `<cell>.launcher.txt`).

## The steps
"Probe" fields are z-position, topmost, foreground, pid and title, box first then Tally. In G cells the box is read from the K probe, because the sheet probe lists only "ComplyEaze Bridge" windows.

| step | stamp (UTC) | what happened | reading |
|---|---|---|---|
| B3 | 07:27:28.910Z | Tally's title clicked; probe 3,0,1,13992 (Tally in front), 12 titled windows; maximised | known answer met |
| B3b | 07:27:36.501Z FIRST | first block 81 ms after FIRST; `PROC powershell.exe` after it; TOPMOST 0x00050108 `topmost=1`, PLAIN 0x00050100 `topmost=0` | known answer met |
| B3c | 07:28:08Z | `reader check ok`, every sub-check True | known answer met |
| B5 before | 07:28:12.428Z | processes 0, windows 0 (a first run at 07:27:25Z listed the lab's own shell: deviation 2) | clean |
| F0 | T0 07:34:43.925Z | child, files, interactive. T0+8 and +11: box 1,1,1; Tally 5,0,0. Click at T0+17.23 s. C+0 and C+6: box 1,1,0 above Tally 5,0,0. No at T0+27.63 s; exit 1, stdout 0 B | **1** |
| F1 | T0 07:35:45.648Z | child, tokio, interactive. Same probes as F0. Click at T0+17.36 s; C+0 and C+6: box 1,1,0 above Tally. No at T0+27.24 s; exit 1, stdout 0 B | **1** |
| F2 | T0 07:36:40.992Z | child, files, launcher. T0+8 and +11: box 1,1,1; Tally 4,0,0. Click at T0+15.67 s; C+0 and C+6: box 1,1,0 above Tally 4,0,1. No at T0+25.29 s; exit 1, stdout 0 B | **1** |
| F3 | T0 07:37:27.744Z | child, tokio, launcher. Same as F2. Click at T0+15.49 s. No at T0+25.04 s; exit 1, stdout 0 B | **1** |
| F4 | T0 07:38:04.408Z | child, tokio, hidden. Same as F2. Click at T0+18.54 s. No at T0+27.99 s; exit 1, stdout 0 B | **1** |
| G1 | T0 07:38:54.905Z | control, tokio, interactive. K probe T0+8 and +11: 1,1,1. Click at T0+16.05 s; C+0 and C+6: 1,1,0 above Tally 5,0,0. No at T0+25.4 s; answer=7, exit 0 | **1** |
| G2 | T0 07:39:31.936Z | control, tokio, hidden. Click at T0+34.02 s (deviation 4); C+0 and C+6: 1,1,0 above Tally 4,0,1. No at T0+44.02 s; answer=7, exit 0 | **1** |
| G3 | T0 07:40:29.111Z | control, tokio, launcher. Click at T0+17.33 s; C+0 and C+6: 1,1,0 above Tally 4,0,1. No at T0+32.39 s; answer=7, exit 0 | **1** |
| B5 after F, G | 07:41:25.715Z | processes 0, windows 0 | clean |
| T1 to T3 | T0 07:41:36.526Z | F0's command, `-Out t.txt`. T0+8 and +11: box 1,1,1; Tally 5,0,0. Click at T0+15.67 s; C+0 and C+6: box 1,1,0 above Tally 5,0,0 | **1** |
| T4 | — | not reached: the box stayed above Tally, so its taskbar button was not clicked | — |
| T5 | 07:42:02.127Z | No by mouse at T0+25.62 s; exit 1; `t.stdout` 0 B; `t.stderr` two lines, both `box-fit: box=(686,181,1254,926) work=(0,0,1920,1030) fits=true` | — |
| P1 | 07:43:04Z | launch-env.json with `BRIDGE_AGENT_ENABLE_WRITES` (5 keys), sha256 5cff5236…2d8; BUILD-T launched 07:43:15Z, server pid 14164 | — |
| B2 | 07:43:20Z | `tally_status` 7.2 silver; `list_companies` BRIDGE BATCH LAB alone; marks 224/87 | as expected |
| C0 | 07:43:30Z | `build_import_xml`: voucher_count 1, totals 141/141; BATCH-T `bridge-f56f25ed-eefe-4f7d-97cc-aad43e6a192e` | as expected |
| C1 | T0 07:43:46.057Z | ONE `post_import`. T0+8 (07:43:54.007Z) and T0+13 (07:43:59.007Z): box 3,0,1; Tally 4,0,0 | **0** |
| C2 | 07:44:00.640Z | box text saved; it differs from row 1780-E's only in the batch line | as expected |
| C3 | 07:44:02.022Z | ONE click into Tally at T0+16.03 s; C+0 and C+6: Tally 3,0,1 above the box 4,0,0 | box covered |
| C3 at T0+30 | 07:44:16.108Z | box below Tally; ONE taskbar click at logical (756,844). It opened the window thumbnails and did not raise the box: Tally 4,0,0, box 5,0,0, still a window | — |
| C4 | 07:44:26Z | `post_import` answered at T0+40 s with the box still open: approval pending, dialog_remaining_s 86, not_dispatched. No second call | — |
| C4 | 07:45:05.435Z | ONE click on the box's thumbnail at logical (756,747), to raise it for the No. The box ended at that click (gone by 07:45:05.572Z); no No, no Yes (deviation 6) | — |
| C4 | 07:45:22Z | `egress_log`: after T0, only `post_import` response_prepared and stdio_write_completed. `local_data_report`: no_dispatch_never_verified 1, sent_or_found_posted 0 | nothing dispatched |
| C5 | 07:45:25Z | marks 224/87, equal to B2; B5 0/0; BUILD-T equal; launch stopped (exit 0). `BRIDGE_AGENT_ENABLE_WRITES` removed: sha256 910b8fe9…4d99 | as expected |
| C10 | 07:48:16Z | BRIDGE BATCH LAB shut; none loaded; one tally.exe owns 9001; `/status` 200 | as expected |
| B5 end | 07:48:28.708Z | processes 0, windows 0 | clean |
| D | — | **not run: needs the owner at the PC** | — |

## Each cell's parent (image and pid)
The pids come from `<cell>.txt` and the runner's lines. The ancestry is read with the reader's name shift undone (see "The reader's CHAIN lines" below).

| cell | the box's process | spawn-parent.ps1's PowerShell | its parent | above that |
|---|---|---|---|---|
| F0 | bridge_mcp.exe 15064 | powershell.exe 20976 | cmd.exe 28264 (f0.cmd) | explorer.exe 18784 |
| F1 | bridge_mcp.exe 25584 | powershell.exe 2860 | cmd.exe 22688 (f1.cmd) | explorer.exe 5056 |
| F2 | bridge_mcp.exe 30536 | powershell.exe 2236 | python.exe 27952 (launcher.py) | powershell.exe 2732 (cell runner) |
| F3 | bridge_mcp.exe 8328 | powershell.exe 10188 | python.exe 23204 (launcher.py) | powershell.exe 14428 (cell runner) |
| F4 | bridge_mcp.exe 11532 | powershell.exe 26776 (hidden) | powershell.exe 10992 (cell runner) | bash.exe 22940 |
| G1 | powershell.exe 3664 (control box) | powershell.exe 15332 | cmd.exe 4148 (g1.cmd) | explorer.exe 15804 |
| G2 | powershell.exe 29604 | powershell.exe 29240 (hidden) | powershell.exe 4864 (cell runner) | bash.exe 11136 |
| G3 | powershell.exe 2240 | powershell.exe 8868 | python.exe 18092 (launcher.py) | powershell.exe 18148 (cell runner) |
| T | bridge_mcp.exe 12456 | powershell.exe 3020 | cmd.exe 9716 (t.cmd) | explorer.exe 17724 |
| C1 | bridge_mcp.exe 18856 | — (the server, bridge_mcp.exe 14164, is its parent) | python.exe 12268 (client.py) | bash.exe 13824 |

## The traces
From `trace-<cell>.txt` (summarised in `evidence/trace-summary.txt` by `scripts/trace-sum.py`):

| trace | FIRST | first block | box hwnd, class, thread, owner | exstyle | topmost changes | fg_pid at the first block, then |
|---|---|---|---|---|---|---|
| b3b | 07:27:36.501Z | 07:27:36.582Z | 0x18e298a WindowsForms10 (TOPMOST), 14896, 0 | 0x00050108 | none | 25660 |
| f0 | 07:34:44.229Z | 07:34:44.372Z | 0x16b27ce #32770, 29392, 0 | 0x00010109 | none | box 15064; 28264 at 07:35:01.171Z (the click); 22180 at the end |
| f1 | 07:35:46.043Z | 07:35:46.225Z | 0x3e2040 #32770, 9736, 0 | 0x00010109 | none | box 25584; 22688 at the click; 13992 at the end |
| f2 | 07:36:41.496Z | 07:36:41.606Z | 0x1f502de #32770, 25096, 0 | 0x00010109 | none | box 30536; Tally 13992 at the click |
| f3 | 07:37:28.218Z | 07:37:28.418Z | 0x19264e #32770, 29216, 0 | 0x00010109 | none | box 8328; Tally at the click |
| f4 | 07:38:04.805Z | 07:38:05.071Z | 0x1771db6 #32770, 20208, 0 | 0x00010109 | none | box 11532; Tally at the click |
| g1 | 07:38:58.206Z | 07:38:58.322Z | 0x1f802de #32770, 12428, 0 | 0x00010109 | none | box 3664; 4148 at the click; Tally at the end |
| g2 | 07:39:36.863Z | 07:39:37.366Z | 0x3d2c30 #32770, 17604, 0 | 0x00010109 | none | box 29604; Tally at the click |
| g3 | 07:40:31.969Z | 07:40:32.134Z | 0x2612a3c #32770, 22092, 0 | 0x00010109 | none | box 2240; Tally at the click |
| t | 07:41:36.791Z | 07:41:36.961Z | 0x3f2c30 #32770, 12628, 0 | 0x00010109 | none | box 12456; 9716 at the click; 22180 at the end |
| c1 | 07:43:53.289Z | 07:43:53.374Z | 0x9a2042 #32770, 12512, 0 | **0x00010101** | none (0 throughout) | **Tally 13992**; box 18856 at 07:43:55.987Z; Tally at the click; 5344 (the trace's own window) at the taskbar click; Tally at 07:45:05.495Z |

- **Every box's own process:** at the first block, three windows: the box (visible), MSCTFIME UI and Default IME (both hidden, `topmost=1`). At the last block it had none.
- **Every trace's PROC lines:** BUILD-T's exe (sha256 3ca00fab…) or Windows PowerShell (9785001b…), started with the cell's command line, the parent pid as in the table above. In every trace the first window block comes 80 to 510 ms after FIRST and before the PROC lines.

## R1: C1's context beside the cells'
The box's process: C1's is the server's child; the F and T cells' is the dialog child; the G cells' is the control box. Each value is from that process's CTX line (`evidence/r1-table.txt`, made by `scripts/r1-table.py`):

| field | C1 | F0 | F1 | F2 | F3 | F4 | G1 | G2 | G3 | T |
|---|---|---|---|---|---|---|---|---|---|---|
| reads | **0** | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| pid | 18856 | 15064 | 25584 | 30536 | 8328 | 11532 | 3664 | 29604 | 2240 | 12456 |
| in_job | True | False | False | True | True | True | False | True | True | False |
| integrity_rid | 0x2000 | 0x2000 | 0x2000 | 0x2000 | 0x2000 | 0x2000 | 0x2000 | 0x2000 | 0x2000 | 0x2000 |
| appcontainer | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| session | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 |
| wow64 | False | False | False | False | False | False | False | False | False | False |
| console_handle | 0x48 | 0x48 | 0x48 | 0x54 | 0x54 | 0x4c | 0x48 | 0x48 | 0x48 | 0x48 |
| std_in | 0x2a4 | 0x7b0 | 0x7f4 | 0x7d8 | 0x7f8 | 0x7f8 | 0x7d0 | 0x7c0 | 0x7d4 | 0x82c |
| std_out | 0x29c | 0x7e0 | 0x7a8 | 0x7dc | 0x7f0 | 0x790 | 0x7a4 | 0x780 | 0x818 | 0x830 |
| std_err | 0x26c | 0x7b8 | 0x7c4 | 0x7fc | 0x6b0 | 0x7cc | 0x790 | 0x7cc | 0x878 | 0x68c |
| startf | 0x100 | 0x100 | 0x100 | 0x100 | 0x100 | 0x100 | 0x100 | 0x100 | 0x100 | 0x100 |
| show_window | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| desktop | WinSta0\Default | same | same | same | same | same | same | same | same | same |

**Its parent's lines.** C1's parent is the server, 14164. The other cells' parent is spawn-parent's PowerShell:

| field | C1 | F0 | F1 | F2 | F3 | F4 | G1 | G2 | G3 | T |
|---|---|---|---|---|---|---|---|---|---|---|
| in_job | True | False | False | True | True | True | False | True | True | False |
| startf | 0x100 | 0x0 | 0x0 | 0x100 | 0x100 | 0x1 | 0x0 | 0x1 | 0x100 | 0x0 |
| show_window | 0 | 1 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 1 |
| console_handle | 0x48 | 0x48 | 0x60 | 0x54 | 0x54 | 0x44 | 0x48 | 0x50 | 0x48 | 0x60 |

Their integrity, appcontainer, session, wow64 and desktop are the same in all ten.

**What R1 shows.** A field that differs is a candidate, not a cause. Here none differs in that way.
- **F2 and F3 match C1:** each child's every field read (handle values aside) equals C1's child's, and each parent's equals C1's server's: in_job True, startf 0x100, show_window 0.
- **Yet F2 and F3 read 1:** the launcher-started cells keep the bit with the very context in which C1 loses it.
- **in_job is not the cause:** the interactive cells (in_job False) and the hidden and launcher cells (in_job True) all read 1.

**What still differs between C1 and every cell** (candidates for part D and for the requester, not causes):
1. **The process that calls `CreateProcess`.**
   - In C1 it is the server itself (`bridge_mcp.exe`, through `nonce_bound_dialog`, with tokio's runtime and its threads).
   - In every cell it is `spawn-parent.ps1`'s C# stand-in inside `powershell.exe`.
   - The spawn's own flags and stdio, as far as the reader sees them, are the same.
2. **When the box appeared.**
   - C1's box appeared 7.2 s after the `post_import` call, while the server was handling it.
   - The cells' boxes appeared 0.3 to 0.5 s after the spawn (FIRST minus `child_pid`), and 2.9 to 4.9 s after the spawn for the control box.
3. **Not a candidate:** in this sitting, Tally held the foreground at C1's first block, and the box took it 2.6 s later. In every cell the box was foreground at its first block. But row 1780-E's C1 had the box foreground at its first block (fg_pid 20648) and it still read 0, so foreground at creation does not decide the bit.

**The reader's CHAIN lines.** Each line writes each ancestor's pid beside the image name of the process one step below it. In the reader's function `Chain`, the line is `$p.ParentProcessId + ":" + $p.Name` while `$p` is the child.
- **Read as written,** C1's chain looks like "14164 ← 12268:bridge_mcp.exe". Read correctly, the server 14164's parent is python.exe 12268 (client.py), whose ancestors are bash.exe and the desktop client.
- **The ctx files are committed as the reader wrote them** (the desktop client's image name and one install path aside: `substitutions.txt`), and the tables above undo the shift.
- **The pids are right; only the names are one step off.**
- **A correction to the lab's own first reading:** read as written, the chain suggested a second bridge_mcp.exe as the server's parent. That was this shift; the server's parent is python.exe.

## Cell F1d: F1 with Tally holding the last input (added 29 Oct, after the first push)
The recipe:
- F1's exact spawn-parent line (child, tokio, interactive), run from a `.cmd` whose first line is `powershell -NoProfile -Command Start-Sleep -Seconds 8`;
- Tally's title clicked once after the start;
- no other input until the box, then the usual T0+14 click, probes, and No or the taskbar.

| cell | title click | spawn-parent, its parent | box | first block: exstyle, topmost, fg_pid | T0+8 and +11 (box; Tally) | after the T0+14 click | end | reads |
|---|---|---|---|---|---|---|---|---|
| F1d | 43 ms after the start, before the console window opened | powershell.exe 7536, cmd.exe 4988 | 11496 | 0x00010109, 1, the box | 1,1,1; 5,0,0 | box 1,1,0 above Tally | No by mouse, T0+29.32 s | 1 |
| F1d-2 | 4.06 s after the start, the console window up | powershell.exe 15700, cmd.exe 21160 | 26788 | **0x00010101, 0**, the box | **3,0,1**; 4,0,0 | Tally 3,0,1 above the box 4,0,0 | not answered: the 60 s bound, `killed_after_timeout` | **0** |
| F1d-3 | 4.14 s after the start | powershell.exe 25388, cmd.exe 18036 | 13948 | **0x00010101, 0**, the box | **3,0,1**; 4,0,0 | Tally above the box; one taskbar click at (756,844) raised the reader's own PowerShell window, not the box | not answered: the 60 s bound | **0** |

- **The context is the same as F1's.** F1d-2's and F1d-3's ctx lines equal F1's for the child (in_job False, integrity 0x2000, startf 0x100, show_window 0, WinSta0\Default) and for its parent (startf 0x0, show_window 1). F1 read 1, and F1d-2 and F1d-3 read 0.
- **What changed between F1 and F1d-2:**
  - the last input before the box: in F1d-2 it was a click on Tally, 8 s before the spawn;
  - the wait between the parent's start and the spawn.
- **The box still took the foreground at its first block** in both 0 cells (fg_pid = the box), yet without the topmost bit, at z 3 under two other windows.
- **F1d did not test the point.** Its title click came before the parent's console window opened, and that console then held the foreground until the box. It is kept as run, reading 1.
- **The trace cannot see** which window had the foreground before the box's first block.

**A correction this cell brought to light.** The traces show where the T0+14 click landed:
- **The interactive cells (F0, F1, G1, T, F1d):** the click at logical (263,475) landed on the parent's own console window, which lay over Tally there. fg_pid moved to the cell's cmd.exe, not to Tally.
- **The hidden and launcher cells (F2, F3, F4, G2, G3):** the click landed on Tally (fg_pid 13992).
- **The readings stand:** they come from the box's exstyle, not from the click.

## Deviations
1. **Working folder:** a new folder on the lab PC, not the sheet's `1780f` folder, which already holds an earlier row's files. Pushed copies say `<lab folder>`.
2. **B5's first run listed 3 processes:** the lab's own shell, whose command line held the markers, because it ran B5, B3 and B3c in one line. Run alone, it listed 0.
3. **The parent kinds, as run:**
   - *Interactive:* a `.cmd` holding the cell's command, opened by `explorer.exe`. Its tree is cmd.exe ← explorer.exe, outside the lab shell's job, in its own console window. The sheet asks for a PowerShell opened from the Start menu with the command typed in it.
   - *Hidden:* started by the cell runner (a PowerShell under the lab's shell), not by an interactive PowerShell.
   - *Launcher:* `launcher.py`, client.py's launch with its command replaced.
4. **G2's click came at T0+34.02 s**, not T0+14, because its T0+8 sheet probe took 14.7 s. Its No came at T0+44.02 s. The bit held throughout.
5. **T1's tail window was not opened.** `t.stderr` was read from the file after the box ended; T4 was not reached, so there was no taskbar-click line to see.
6. **Part R's box was not answered No.**
   - At T0+30 it lay below Tally. The runner clicked its taskbar button ONCE at (756,844); that showed the window thumbnails and raised nothing.
   - The lab then clicked the box's thumbnail ONCE at (756,747) to bring it up for the No, and the box ended at that click.
   - No Yes was pressed by click or key, and nothing was dispatched (egress, local report, marks).
   - The box ended when activated from the taskbar without being answered, which matches row 1780-E's taskbar click. The child's stderr was discarded by the server, so its fit line is not known.
7. **F1d's 8 s wait is the `.cmd`'s first line,** not typed in the console before F1's line. The lab types nothing into a console. The spawn-parent line is F1's, byte for byte.
8. **F1d-2's and F1d-3's boxes were not answered No.** Each lay below Tally after the T0+14 click.
   - In F1d-2 the runner's 30 s wait for a taskbar position ran out, and nothing was clicked.
   - In F1d-3 one taskbar click at (756,844) raised the reader's PowerShell window, not the box.
   - Both children were ended by spawn-parent's 60 s bound (`killed_after_timeout`), the sheet's named carve-out. No Yes was pressed; in part F a Yes posts nothing.

## Reading
- **F0 to F4 all read 1:** the lab's parents do not reproduce C1, and the sheet says go on to D.
- **G cells read the same as F cells** (1): the loss is not shown here by either box under a lab parent.
- **C1 read 0 again:** this sitting agrees with 1780-E.
- **T4 was not reached:** the box stayed above Tally. Part T therefore says nothing about the taskbar close. Part R's box ended at a taskbar activation again.
- **Part D: not run: needs the owner at the PC.**
- **F1d-2 and F1d-3 read 0:** F1's context loses the bit when Tally had the last input, 8 s before the spawn. By the requester's reading, the cause is the foreground lock or input recency, not the spawner. Two runs, one sitting.

**FAIL checks:**
- No Yes; no post; marks unchanged.
- B5 clean at every run after the first (deviation 2).
- B3b and B3c met.
- No key while a box had the keyboard.
- D not run, so there is no configuration file to restore.

## Not measured
- Keys, menu clicks and fields in Tally.
- A second monitor; macOS; Windows 11.
- The ledger-create, review-acknowledgement and invoice-stop boxes.
- Any change to the build.
- A parent that is not one of the kinds above (a service, a scheduled task, a remote session), or one whose console was created windowless the way a Node host starts a server.
  - Every F and G parent is `powershell.exe`, never `bridge_mcp.exe`.
- The limits of an unnamed job object.
- The real preview text: the box text is `confirm-input.txt`'s synthetic 23 lines.
- The standard library's overlapped pipe: the pipe is an anonymous `CreatePipe` pair.
- The 120 s bound: the bound is 60 s.
- Part D (the desktop client as the parent).
- Part T's taskbar click (T4).

**Confidence:** one sitting.

## Files
- `answers/` holds part R's eight answers whole, and `client-log.txt` holds part R's launch log.
- `evidence/` holds:
  - every `<cell>.txt`, `<cell>.stdout` and `<cell>.stderr` (files stdio only), each `<cell>.control.txt`, and each `<cell>.launcher.txt` with its error file;
  - every `ctx-<cell>.txt` and `trace-<cell>.txt`, whole (the one edit is in `substitutions.txt`);
  - every probe file and `<cell>-timeline.txt`;
  - `c-timeline.txt`, `r-run-console.txt`, `c2-box-text.txt`, `b3-timeline.txt`, `forms.txt` and `ctx-check.txt`;
  - the B5 runs, `c10.txt`, `trace-summary.txt` and `r1-table.txt`.
- `scripts/` holds the runners and helpers named above; `cmd/` holds the `.cmd` files of the interactive cells and part R's taskbar position.
- Also: `substitutions.txt`, `notes.txt` (UTC), and `SCREENSHOTS.txt` (the captures, kept on the lab PC).
