# Windows item 1780-F v2: which part of the spawn context makes the approval box lose its topmost bit. The dialog child and the control box started with tokio's stdio or with file redirects, from parents of different kinds; the context of the server's child read by pid; the taskbar click on a hand-run box with its stderr kept; and, last and only if needed, the post from Claude Desktop. Answered No by mouse (Part of #1780)

WRITES to the book: NONE. Every box is answered **No, by mouse**. Nothing is keyed in Tally. The one click into Tally in each run lands on a blank part of its screen. Parts F, G and T send no request to Tally and start no Bridge server. Parts R and D start one, with posting on, and end every box with No.

## Why
- Row 1780-E (on #2107's build, head a66c31f2) found:
  - the control box with the approval box's own flags (K1c), with the DPI call added (K1d), with the activation hook (K1e) and with the helper thread (K1f) all read `topmost` 1 (extended style 0x00010109), and so did the dialog child run by hand (H1);
  - the dialog child started by the server (C1) reads 0 (0x00010101), from the first block;
  - a taskbar click ended the box in 78 ms. Whether the child's activation hook fired is not known: its stderr, where its measurement line goes, was discarded by the parent.
- So the box's own code, flags, DPI call, hook and helper thread are ruled out as the cause. What still differs between H1 (reads 1) and C1 (reads 0) is **how and from what the child is started**. This row separates the two parts of that, one at a time, against a base cell F0 (child, file redirects, interactive parent: the nearest thing to H1 that this sitting's tool can make):

  | | candidate | cells |
  |---|---|---|
  | S1 | the stdio plumbing of `nonce_bound_dialog` (stdin and stdout anonymous pipes, stderr the NUL device, `CREATE_UNICODE_ENVIRONMENT` only, handles inherited) | X: F1 against F0 (only the stdio differs) |
  | S2 | the parent's own context: how the parent was started (interactive, hidden, the lab's launcher), its desktop, window station, job, integrity, console | Y: F2 against F0 (only the parent differs); F3 and F4; part R |
  | S3 | Claude Desktop as the parent of the server | part D, last |

- The control box is run the same way (part G) to tell whether the loss is about the **child** or about any box shown by a process started that way.
- The 1780-E trace hashed the 31.8 MB exe before its first window block, so its first block came 1.7 to 2.3 s after `FIRST`. This row uses `box-trace-early.ps1`, which writes the window block first and looks the process up after it.

## Who acts
- On this synthetic company the lab does every step itself, including the clicks into Tally and every No, on the owner's standing word of 27 Oct 2026.
- The lab records each such step as "done by the lab on the owner's standing word of 27 Oct".
- Posting is switched on for parts R and D only, on the owner's word of 28 Oct, and off after each of them and at once by any STOP in them (the rule below).
- **Part D needs the owner at the PC** (a change to Claude Desktop's own configuration and a chat in it). The lab cannot drive Claude Desktop. If the owner is not there, write "not run: needs the owner at the PC" and finish the sheet.
- The go-ahead and any STOP ruling come from the run coordinator.

## Rules of this sheet (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS: no key while a box has the keyboard.** A box takes the foreground and the keyboard when it appears (`MB_SETFOREGROUND`). A stray `Y` presses Yes, and an Up-arrow recall moves the focus to Yes before Enter does. In parts F, G and T a Yes only makes the child print a token to a file nobody reads; **in parts R and D a Yes is a post.** Therefore, in every part:
  1. Everything that can be started before the box exists is: `ctx-reader.ps1` waits by itself (`-SpawnFile` or `-WaitForConfirmChild`, below), and the lab's timing helper for the probe pairs (the 1780-D helpers `k-run.ps1` and `c-run.ps1` may be reused) is started before the parent. After the box appears the lab uses the mouse and the screenshot tool only.
  2. If a command must be typed while a box exists (a probe the helper does not cover, a read of a file): **first click the lab's own PowerShell window** (a mouse click on its title bar), **confirm by a screenshot that it is the active window**, then type the whole command. No history recall (Up, F7, Ctrl+R), no Tab completion, Enter only after the line has been read back on the screen. In parts R and D this is the only way a key may be pressed while a box exists.
  3. No key in the box's window, ever. A box is answered by mouse.
- **C2 carve-out (named).** `LAB-COMMON.md` C2 forbids ending any process. This sheet ends exactly one kind: `spawn-parent.ps1` ends **the one child it started itself, by the pid and handle its own `CreateProcess` returned**, if that child is still running `-TimeoutSeconds` (60) after its start (the stand-in for the product's own 120 s bound). Nothing else is ended, by name or by number, by any step, script or person of this sheet; `ctx-reader.ps1 -Check` ends nothing (its helper ends itself after 20 s). A box or process still there after the sitting is recorded and left (C2), not ended.
- **A STOP in part R or part D turns posting off.** First answer any box **No** by mouse (never Yes). Then, for R: remove `BRIDGE_AGENT_ENABLE_WRITES` from the launch's environment file, stop the launch, record the file's sha256. For D: the restore of D5 (which removes the whole lab entry, and with it the setting). Only then report.
- **Privacy of the context files.** `ctx-reader.ps1` and `spawn-parent.ps1` replace the signed-in user's name by `<user>` and any `X:\Users\<name>` by `X:\Users\<user>` before writing (a name of fewer than 3 characters is only replaced inside such a path). Before any context, spawn or trace file is committed the lab searches it for the user's name and for a path outside the lab folders; any hit is replaced by `<user>` or `<path>`, the edit recorded in `notes.txt`, and the unedited file is not pushed. A file name recorded for the owner's configuration (D0) is the bare file name, never a path.

## Build and inputs (hash before the first step and at the end; a different hash = STOP)
| item | source | bytes | sha256 |
|---|---|---|---|
| `bridge-tally.mcpb` and `bridge_mcp.exe`, BUILD-T | #2107 at head a66c31f2ab607b36701ea0474ca15d474b9e202c: CI run 40709511734, artifact 12826362269 `unsigned-bundle-smoke-windows-latest`; the copy rows 1780-C2, D and E used, nothing is downloaded | mcpb 15,320,736; exe 31,804,416 | mcpb 61e52d079544b2c7dd49cb47bf08a11120b0154957e830e8a76e598970ab99c2; exe 3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974 |
| `requests/box-trace.ps1`, `zorder-probe.ps1`, `control-forms.ps1`, `build-1591-s1.json` | byte copies from lab/1780-topmost-diagnosis @ 4cd8ac541df96355a874febb035c782416c3f58e (as in row 1780-E) | see `SHA256.txt` | see `SHA256.txt` |
| `requests/zorder-probe-k.ps1`, `control-box-product.ps1`, `confirm-input.txt` | byte copies from row 1780-E (lab/1780-topmost-bisect @ 1542ab88, the probe from lab/1780-topmost-diagnosis-answers @ 5c6dbf2a) | see `SHA256.txt` | see `SHA256.txt` |
| `requests/box-trace-early.ps1` | written for this row: `box-trace.ps1` with the first window block written before the process lookup, nothing else changed | see `SHA256.txt` | see `SHA256.txt` |
| `requests/spawn-parent.ps1` | written for this row, not yet run (its C# part compiled on a Mac with Mono's `mcs`, no errors) | see `SHA256.txt` | see `SHA256.txt` |
| `requests/ctx-reader.ps1` | written for this row, not yet run (its C# part compiled the same way); it reads only | see `SHA256.txt` | see `SHA256.txt` |

- THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, loaded alone. Step P0 checks it.
- Working folder: `C:\bridge-lab\1780f\`. Evidence goes in `C:\bridge-lab\1780f\evidence\`. `control-forms.ps1` keeps its `<lab folder>` placeholder: replace it in the copy you run and record it.
- `SHA256.txt` lists every other file of this folder with its size. Check it before step P0. A different hash or size = STOP.
- The `.ps1` files are run with `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...` from Windows PowerShell 5.1. Record each command line exactly as typed, with any substitution.
- **The number format of this PC (row 1780-D, deviation 2).** A comma is a digit separator: no command in this sheet takes a list. `ctx-reader.ps1 -Pids` takes ONE quoted string with spaces (`'1234 5678'`); a token that is not a pid is written as an `ERROR` line and the script exits 2.
- **Three kinds of parent** are named below and each is recorded with its pid:
  - *interactive*: a Windows PowerShell opened from the Start menu, the command typed in it;
  - *hidden*: `Start-Process powershell -WindowStyle Hidden -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File ...'` from the interactive one;
  - *launcher*: the lab's own launcher, the script or shortcut that started the server in rows 1780-C2, D and E, run with the same options, working folder and environment file, with its command replaced by the one given here. Record the launcher's command line. If the launcher cannot start a `.ps1`, say so, skip the launcher cells and say what the launcher is.
- The command of a cell is always `spawn-parent.ps1 -Target <child|control> -Stdio <tokio|files> -Out <evidence>\<cell>.txt -TimeoutSeconds 60` plus, for `child`, `-ChildExe <build folder>\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile <folder>\requests\confirm-input.txt`, and for `control`, `-ControlScript <folder>\requests\control-box-product.ps1`. The files it writes are `<cell>.txt` (its own record), `<cell>.stdout` and `<cell>.stderr` (files stdio only; opened for read and write sharing, so they can be read while the child runs) and `<cell>.control.txt` (the control box's answer). The 60-second bound is the parent's own `kill_on_drop` stand-in: the carve-out above; record if it fires (`exit_code=killed_after_timeout`).

## Before the sitting
- P0. Tally's own Select Company list. If BRIDGE BATCH LAB is not in it: STOP and report. Do not restore it from a backup and do not use another book. If it is there but is not the one company loaded: load it alone.
- B1. Backup of THE COMPANY (BK0), named, with size and sha256.
- B3. **Probe known-answer check**, as in rows 1780-D and E: with Tally's title clicked, `zorder-probe.ps1` prints Tally first and in front.
- B3b. **Trace known-answer check**: `control-forms.ps1 -Seconds 20` while `box-trace-early.ps1 -Title "Tally probe control" -Out <evidence>\trace-b3b.txt -Seconds 10` runs. EXPECT a `FIRST` line, a first window block within 1 s of it, `PROC` lines naming `powershell.exe` after that block, "Tally probe control TOPMOST" with `topmost=1` and "Tally probe control PLAIN" with `topmost=0`. Anything else = STOP.
- B3c. **Reader known-answer check**: `ctx-reader.ps1 -Check -Out <evidence>\ctx-check.txt`. EXPECT the last line `reader check ok`. It proves, on this very PowerShell: that CIM returned a command line and the one read from the PEB equals it (offset 0x70); that the three standard handles read from the PEB equal `GetStdHandle`'s (0x20, 0x28, 0x30); that the desktop string reads `WinSta0\Default` (0xC0); that the console handle (0x10) is non-zero and equals none of the standard handles; that the reader is 64-bit and not wow64; and, on a hidden PowerShell it starts, that `STARTF_USESHOWWINDOW` is set with show-window 0 (0xA4, 0xA8). What it does not prove: that offset 0x10 is the console handle and not another non-zero field, beyond those inequalities. `reader check FAILED` = STOP (the PEB offsets are wrong for this Windows; report both CTX lines and the result list).
- B4. The screen tool's frame, and the screen's logical size and scale. Every click below is given in both.
- B5. **Leftover list** (run before the sitting, after parts F, G and T, and at the very end): every process whose command line contains `--confirm-journal`, `control-box-product.ps1`, `spawn-parent.ps1`, `box-trace-early.ps1` or `ctx-reader.ps1` (pid, parent, start, image, command line), and every visible window titled "ComplyEaze Bridge" or "Lab topmost control". EXPECT none (the lab's own current shell excepted). If any: record it whole, end nothing, STOP and report.

## Parts F and G: the cells
**Order: F0, F1, F2, F3, F4, G1, G2, G3.** Run all eight unless a cell's box does not appear (step 5). Every cell is the same recipe. Tally MAXIMISED on the Gateway of Tally and in front.

| cell | target | stdio | parent kind | note |
|---|---|---|---|---|
| F0 | child | files | interactive | the base: file redirects from an interactive parent |
| F1 (X) | child | tokio | interactive | F0 with only the stdio changed |
| F2 (Y) | child | files | launcher | F0 with only the parent changed |
| F3 | child | tokio | launcher | the stdio of the product, the launcher of the lab |
| F4 | child | tokio | hidden | |
| G1 | control | tokio | interactive | the 1780-E control box (variant c) |
| G2 | control | tokio | hidden | |
| G3 | control | tokio | launcher | |

**The recipe for a cell** (the box's title is "ComplyEaze Bridge" for F, "Lab topmost control" for G). Read the KEYS rule first:
1. **Pre-arm, before the parent is started** (each in its own window, started and left waiting): the trace, without hashing the exe first, `box-trace-early.ps1 -Title "<title>" -Out <evidence>\trace-<cell>.txt -Seconds 100`; the reader, `ctx-reader.ps1 -SpawnFile <evidence>\<cell>.txt -DelaySeconds 6 -Out <evidence>\ctx-<cell>.txt` (it waits for the `child_pid` line the parent writes, then reads the parent and the child; record every CTX, CHAIN and CONHOST line); and the lab's timing helper for the probe pairs at `<cell>-T0+8` and `<cell>-T0+11`. Click Tally's title area so Tally is in front.
2. Start the parent of the cell (kind and command above) and click Tally's title area at once. **`<cell>-T0` is the stamp on the `SPAWN ... child_pid=` line of `<cell>.txt`**, not the time the command was typed (`Add-Type` compiles the C# for a second or more first, longer under a hidden or launcher parent). The click schedule below counts from it.
3. Probes at `<cell>-T0+8` and `<cell>-T0+11`, each as the pair `zorder-probe.ps1` then `zorder-probe-k.ps1`, start and end stamped; a whole-screen screenshot after the second.
4. If no box with the title appears by `<cell>-T0+15`: record "the box did not appear", do not end anything, read `<cell>.txt` and `<cell>.stderr` if present, and go on to the next cell.
5. At `<cell>-T0+14` click ONE blank point in Tally at least 150 logical pixels from every edge of the box (record it in both frames and its distance). Probes and a screenshot at once and 6 s later. **No taskbar click in parts F and G.**
6. Answer **No** by mouse (through the box's taskbar button if Tally covers it; record that). EXPECT, for F cells, `exit_code=1` and `stdout_bytes_read=0` in `<cell>.txt` (files stdio: an empty `<cell>.stdout`); for G cells `<cell>.control.txt` holds `answer=7`. Exit code 77 means the box did not fit at an activation: record it. `exit_code=killed_after_timeout` means nothing answered it in 60 s: record it.
7. Record from the trace: `FIRST` and the first block's stamp, the box's `hwnd`, `class`, `exstyle`, `owner`, `topmost` in the first block and every later block, with the stamp of each change. A cell reads **0** if any block shows `topmost=0` for the box, else **1**.
8. After the last of these cells (G3) run the leftover list (B5).

## Part T: the taskbar click on a hand-run box, with its stderr kept
Which of the two the click does (the hook's re-measurement posting `IDCANCEL` when the box is measured as not fitting, or something else) is read from the child's own stderr, which part R discards but a file redirect keeps. No server, no request to Tally.
- T0. Tally MAXIMISED and in front. The leftover list (B5) shows nothing.
- T1. **Pre-arm**: create an empty `<evidence>\t.stderr` (`New-Item`); open a second PowerShell window and start in it `Get-Content <evidence>\t.stderr -Wait` (it shows each stderr line as it is written); start the trace (`box-trace-early.ps1`, `trace-t.txt`), the reader (`-SpawnFile <evidence>\t.txt -DelaySeconds 6`) and the timing helper. Then run **F0's command** with `-Out <evidence>\t.txt` (child, files, interactive). The stderr is `<evidence>\t.stderr`. The tail window is read from screenshots at the stamps below; no key is typed while the box exists.
- T2. `T-T0` is the `child_pid` line's stamp. Probes at T-T0+8 and T-T0+11; a screenshot of the tail window at T-T0+11 (EXPECT one `box-fit: box=(...) work=(...) fits=True` line per activation so far).
- T3. At T-T0+14 click ONE blank point in Tally at least 150 logical pixels from the box; probes and a screenshot at once and 6 s later.
- T4. If the box is below Tally: click its taskbar button ONCE; stamp; probe at once and 1.5 s later; screenshot the tail window at once and again 2 s later. Record every line with its order. Record the exit code (from `t.txt`) and whether the box is still a window.
- T5. If the box is still there, answer **No** by mouse. EXPECT exit code 1 and an empty `t.stdout`. Close the tail window by its mouse close button. Run the leftover list (B5).

## Part R: the server's child, read by pid (C1 again)
**A STOP anywhere in this part: the box is answered No first, then posting is turned off (rules above).** The pre-armed reader and helper are started before `post_import`; nothing is typed while the box exists (KEYS).
- P1. **Posting on for this launch**: in the launch's environment file set `BRIDGE_AGENT_ENABLE_WRITES` to `true`, point the launch at BUILD-T, record the file's key names (not values) and its sha256, then start the build with the lab's launcher.
- B2. Hash the build items. `tally_status` and `list_companies`: EXPECT BUILD-T and THE COMPANY as the one company. `ledger_masters` (basic): record the marks. Read the desktop client's version afresh.
- C0. `build_import_xml` with the exact content of `build-1591-s1.json`. EXPECT `voucher_count` 1 and `current_window_readable` with `source_rows` 0 for 20260914. Keep `batch_id` as BATCH-T. Any other = STOP.
- C1. **Pre-arm**: the trace (`box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-c1.txt -Seconds 150`), the reader (`ctx-reader.ps1 -WaitForConfirmChild -DelaySeconds 6 -Out <evidence>\ctx-c1.txt`; it finds the `bridge_mcp.exe` whose command line holds `--confirm-journal`, then reads it, the server that started it and the server's parent) and the timing helper. Tally maximised and in front. Then `post_import` with `company_guid` and BATCH-T; stamp T0 at the call; click Tally's title area at once.
- R1. The reader's lines (`ctx-c1.txt`): record every line, and the child's `in_job`, `integrity_rid`, `appcontainer`, `session`, `wow64`, `console_handle`, `std_in`, `std_out`, `std_err`, `startf`, `show_window` and `desktop`. Then set the same lines of the F and G cells (`ctx-f0.txt` to `ctx-g3.txt`) beside them in RESULTS.md. A field that differs is a candidate, not a cause.
- C2. The box's text, as in row 1780-D: the same STOP rule for a difference in a line that names the company, GUID, number, voucher type, date, an entry, a total, the batch, the reference or the narration.
- C3. Probes (`zorder-probe.ps1`) at T0+8 and T0+13, by the helper; a screenshot after the second. At T0+16 ONE click into Tally on a blank point at least 150 logical pixels from the box; probes and a screenshot at once and 6 s later. **No taskbar click.** Answer **No** by mouse at about T0+30 (through its taskbar button if needed; record it). Never click Yes.
- C4. After the box is gone, make no second `post_import`. Record the call's own answer whole, `egress_log` whole (any line for BATCH-T after T0), `local_data_report` for BATCH-T. EXPECT nothing dispatched.
- C5. End: `ledger_masters` (basic): the marks equal B2's. The leftover list (B5) shows nothing. Re-hash the build items. Stop the launch, remove `BRIDGE_AGENT_ENABLE_WRITES`, record the environment file's new sha256.

## Part D: Claude Desktop as the parent (last, and only if parts F, G, R and T do not settle it) NEEDS THE OWNER AT THE PC
Run it only when the reading table says "go on to D". It changes Claude Desktop's MCP configuration, which the owner approves and does. **Posting is switched on in this part (D1) and off again by D5.** Read the KEYS rule: the box takes the keyboard of the same PC the owner is typing on.
- D0. **Reference and other servers.** With Claude Desktop closed, the owner copies its MCP configuration file as it stands into the lab folder and the lab records, for the original and the copy, the bare file name, size and sha256. That sha256 is **REF-D**. The owner lists, in Claude Desktop's settings, every Bridge server or extension that is enabled, and disables each one for this sitting (an installed Bridge extension would run beside the lab entry and could answer or double the call); the lab records which. BUILD-T's `bridge_mcp.exe` is hashed (EXPECT the table's sha256).
- D1. With Claude Desktop still closed, the owner adds one server entry whose command is BUILD-T's `bridge_mcp.exe` and whose environment holds the lab's usual settings and `BRIDGE_AGENT_ENABLE_WRITES=true` (key names only in the notes), then starts Claude Desktop. The lab records the file's new sha256 (not committed).
- D2. **Checks first, in a new chat, "Allow once" on every tool prompt and never "Always allow".** In order, one tool per message: `tally_status` (EXPECT the server version of BUILD-T; any other build = STOP), `list_companies` (EXPECT THE COMPANY as the one company), `ledger_masters` basic (the marks, MD0), `egress_log` (whole; the baseline). Then `build_import_xml` with the exact content of `build-1591-s1.json` (EXPECT `voucher_count` 1; keep `batch_id` as BATCH-D). Any other result = STOP.
- D3. **Pre-arm**: the trace (`box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-d.txt -Seconds 150`), the reader (`ctx-reader.ps1 -WaitForConfirmChild -DelaySeconds 6 -Out <evidence>\ctx-d.txt`; its third process is the server's parent, Claude Desktop or the host it uses) and the timing helper. Tally maximised and in front. The owner sends exactly this message and nothing else: "Call post_import once for this company and BATCH-D. Do not call it again, whatever it answers, and say what it answered." and gives "Allow once". Stamp D-T0 at the allow. The owner and the lab type nothing while the box exists (KEYS).
- D4. Probes at D-T0+8 and D-T0+11 by the helper; a screenshot. At D-T0+14 ONE blank click into Tally; probes at once and 6 s later. Then **one taskbar click** on the box, once; stamp; probe at once and 1.5 s later. Then answer **No** by mouse if the box is still there. Never Yes.
- D4b. **One call only.** If the chat calls `post_import` again (a second box), or calls any tool beyond D2's list: answer **No** by mouse on every further box, ask for nothing more, stop the chat's answer with its stop button, STOP and go to D5. A retry after the box ended at the taskbar click is the likely case.
- D5. After the box is gone and the chat has stopped:
  1. In the same chat, one message each, "Allow once": `ledger_masters` basic (the marks MD1; EXPECT equal to MD0, else FAIL), `egress_log` whole (EXPECT no line that sends BATCH-D to Tally), `local_data_report` for BATCH-D (EXPECT nothing dispatched). Record each whole.
  2. The leftover list (B5) shows nothing; BUILD-T's exe re-hashed (EXPECT the table's).
  3. Close Claude Desktop. The owner restores the configuration file **byte for byte from the D0 copy** (the restore removes the lab entry with its `BRIDGE_AGENT_ENABLE_WRITES`), and the lab hashes it: **EXPECT REF-D**; a different sha256 is a FAIL of the row (record both). The owner re-enables the servers disabled in D0.
  4. The owner confirms in Claude Desktop's settings that **no "Always allow" grant for the lab entry's tools remains** (the entry is gone with the restore; if a grant is still listed, the owner removes it); record what the screen showed.
  5. What stays in the owner's setup and is **not** deleted by the lab: the lab chat in Claude Desktop's history (record its title) and Claude Desktop's own MCP log files for the lab entry (record their names and sizes). Claude Desktop may rewrite its configuration at its next start; record it, it is not a FAIL if the hash matched right after the restore.

## Reading it (for the requester, not the lab)
| reading | meaning |
|---|---|
| F1 reads 0, F0 reads 1 | **S1**: tokio's stdio shape (pipes and the NUL stderr with those creation flags) is enough to lose the bit, from an interactive parent. Only the stdio differs between the two. |
| F0 reads 0 | The bit is lost already with file redirects from an interactive parent: the spawn tool itself (a `CreateProcess` child of PowerShell) differs from 1780-E's H1 (`Start-Process`). Report it before reading the rest. |
| F0 reads 1, F1 reads 1, F2 reads 0 | **S2 (launcher)**: the lab launcher's context loses the bit whatever the stdio is. Only the parent differs between F0 and F2. R1's fields for the launcher against the interactive parent name the difference. |
| F0 reads 1, F1 reads 1, F2 reads 1, F3 reads 0 | **S1 and S2 together**: pipes under the launcher, neither alone. |
| F4 reads 0 and F1 reads 1 | A hidden parent loses the bit (its window station, desktop or show-window state). R1's `desktop`, `session`, `show_window` and `startf` for it. |
| F0 to F4 all read 1 | The lab's parents do not reproduce C1. R1 against `ctx-f0` to `ctx-f4` names what the server's child has that they do not. **Go on to D.** |
| G cells read the same as F cells with the same kind and stdio | The loss belongs to the parent context, not to the dialog child. |
| G cells read 1 where the F cell of the same kind and stdio reads 0 | The loss is the child's: something it does after the spawn, with that context, not the context alone. |
| C1 (part R) reads 1 this time | The bit held in this sitting. The 1780-E reading and this one disagree: report both, with the Z-order lines, and do not go on to D. |
| T4: the stderr line before the box ends reads `fits=False`, the exit code 77 | The close at a taskbar click is `on_activate` measuring the box as not fitting at re-activation. The numbers on the line say why. |
| T4: the box ends and the line reads `fits=True`, the exit code not 77 | The close is not the fit check. |
| T4: the box stays | The taskbar click is not what ended it in 1780-E; read the parent's own cell (F0) for its exit code. |
| D: the box reads 0 and the lab cells read 1 | **S3**: Claude Desktop as the parent. |
| D: the box reads 1 | The bit held under Claude Desktop in this sitting: report it. |

- **FAIL of the row:** a Yes by click or key on any box; any post; a mark that changes; a box or a child left after the sitting (B5); B3b's or B3c's known answer not met; the configuration file of part D not restored to REF-D; an "Always allow" grant left; a key pressed while a box had the keyboard against the KEYS rule.

## Evidence (stays on the lab PC except the files listed)
- RESULTS.md: one table, one line per step (B3b, B3c, F0 to F4, G1 to G3, T1 to T5, B5 at each of its runs, R's B2 and C0 to C5, D if run), every stamp and every probe line (position, topmost, foreground, pid, title), each cell's reading (0 or 1), and the side-by-side table of R1's lines.
- From each trace, in RESULTS.md: `FIRST` and the first block's stamp, the `PROC` lines, for the box's own window its `hwnd`, `class`, `thread`, `owner` and `exstyle`, every stamp at which its `topmost` changed with the values either side, the windows of its process at the first and last block, and any `fg_pid` change.
- Each `<cell>.txt`, `<cell>.stdout`, `<cell>.stderr`, `ctx-<cell>.txt` and the exit codes, whole, after the privacy search above.
- Commit the trace and context files whole after that search. SHA256.txt over the committed files, with sizes. notes.txt in time order (UTC). SCREENSHOTS.txt: name, size and sha256 of every capture (they stay local). The owner's configuration file, its copy and Claude Desktop's logs are never committed; only their bare names, sizes and sha256.
- Also record: BUILD-T's pull request, head, CI run, artifact and exe sha256; the screen size and scale; the Windows build, the Tally release and the desktop client version, read afresh; the launcher's command line.
- Evidence on a new branch, `lab/1780-topmost-spawn-answers`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- Keys, menu clicks and fields in Tally.
- A second monitor; macOS; Windows 11.
- The ledger-create, review-acknowledgement and invoice-stop boxes.
- Any change to the build: this row only reads.
- A parent that is not one of the kinds above (a service, a scheduled task, a remote session), and a parent whose console was created windowless the way a Node host (`windowsHide`) starts a server: only the hidden and launcher cells cover that, and only if they happen to; the parent image in every F and G cell is `powershell.exe`, not `bridge_mcp.exe`.
- The job object's own UI limits: `ctx-reader.ps1` reports whether a process is in any job and which named jobs exist, not the limits of an unnamed job.
- The box text is `confirm-input.txt`'s synthetic 23 lines, not C1's real preview; the pipe is an anonymous `CreatePipe` pair, not the standard library's overlapped one; the bound is 60 s, not 120 s.
