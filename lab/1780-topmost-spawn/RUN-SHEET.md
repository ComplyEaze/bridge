# Windows item 1780-F v1: which part of the spawn context makes the approval box lose its topmost bit. The dialog child and the control box started with tokio's stdio or with file redirects, from parents of different kinds; the context of the server's child read by pid; the taskbar click on a hand-run box with its stderr kept; and, last and only if needed, the post from Claude Desktop. Answered No by mouse (Part of #1780)

WRITES to the book: NONE. Every box is answered **No, by mouse**. Nothing is keyed in Tally. The one click into Tally in each run lands on a blank part of its screen. Parts F, G, R-hand and T send no request to Tally and start no Bridge server. Part C starts one, with posting on, and ends every box with No.

## Why
- Row 1780-E (on #2107's build, head a66c31f2) found:
  - the control box with the approval box's own flags (K1c), with the DPI call added (K1d), with the activation hook (K1e) and with the helper thread (K1f) all read `topmost` 1 (extended style 0x00010109), and so did the dialog child run by hand (H1);
  - the dialog child started by the server (C1) reads 0 (0x00010101), from the first block;
  - a taskbar click ended the box in 78 ms. Whether the child's activation hook fired is not known: its stderr, where its measurement line goes, was discarded by the parent.
- So the box's own code, flags, DPI call, hook and helper thread are ruled out as the cause. What still differs between H1 (reads 1) and C1 (reads 0) is **how and from what the child is started**. This row separates the two parts of that, one at a time:

  | | candidate | cells |
  |---|---|---|
  | S1 | the stdio plumbing of `nonce_bound_dialog` (stdin and stdout anonymous pipes, stderr the NUL device, `CREATE_UNICODE_ENVIRONMENT` only, handles inherited) | X: F1 against the files cell F2 |
  | S2 | the parent's own context: how the parent was started (interactive, hidden, the lab's launcher), its desktop, window station, job, integrity | Y: F2 and the other launcher cells; part R |
  | S3 | Claude Desktop as the parent of the server | part D, last |

- The control box is run the same way (part G) to tell whether the loss is about the **child** or about any box shown by a process started that way.
- The 1780-E trace hashed the 31.8 MB exe before its first window block, so its first block came 1.7 to 2.3 s after `FIRST`. This row uses `box-trace-early.ps1`, which writes the window block first and looks the process up after it.

## Who acts
- On this synthetic company the lab does every step itself, including the clicks into Tally and every No, on the owner's standing word of 27 Oct 2026.
- The lab records each such step as "done by the lab on the owner's standing word of 27 Oct".
- Posting is switched on for part C only, on the owner's word of 28 Oct, and off after it.
- **Part D needs the owner at the PC** (a change to Claude Desktop's own configuration and a chat in it). The lab cannot drive Claude Desktop. If the owner is not there, write "not run: needs the owner at the PC" and finish the sheet.
- The go-ahead and any STOP ruling come from the run coordinator.

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
- **The number format of this PC (row 1780-D, deviation 2).** A comma is a digit separator: no command in this sheet takes a list. `ctx-reader.ps1 -Pids` takes ONE quoted string with spaces (`'1234 5678'`).
- **Three kinds of parent** are named below and each is recorded with its pid:
  - *interactive*: a Windows PowerShell opened from the Start menu, the command typed in it;
  - *hidden*: `Start-Process powershell -WindowStyle Hidden -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File ...'` from the interactive one;
  - *launcher*: the lab's own launcher, the script or shortcut that started the server in rows 1780-C2, D and E, run with the same options, working folder and environment file, with its command replaced by the one given here. Record the launcher's command line. If the launcher cannot start a `.ps1`, say so, skip the launcher cells and say what the launcher is.
- The command of a cell is always `spawn-parent.ps1 -Target <child|control> -Stdio <tokio|files> -Out <evidence>\<cell>.txt -TimeoutSeconds 60` plus, for `child`, `-ChildExe <build folder>\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile <folder>\requests\confirm-input.txt`, and for `control`, `-ControlScript <folder>\requests\control-box-product.ps1`. The 60-second bound is the parent's own `kill_on_drop` stand-in: record if it fires (`exit_code=killed_after_timeout`).

## Before the sitting
- P0. Tally's own Select Company list. If BRIDGE BATCH LAB is not in it: STOP and report. Do not restore it from a backup and do not use another book. If it is there but is not the one company loaded: load it alone.
- B1. Backup of THE COMPANY (BK0), named, with size and sha256.
- B3. **Probe known-answer check**, as in rows 1780-D and E: with Tally's title clicked, `zorder-probe.ps1` prints Tally first and in front.
- B3b. **Trace known-answer check**: `control-forms.ps1 -Seconds 20` while `box-trace-early.ps1 -Title "Tally probe control" -Out <evidence>\trace-b3b.txt -Seconds 10` runs. EXPECT a `FIRST` line, a first window block within 1 s of it, `PROC` lines naming `powershell.exe` after that block, "Tally probe control TOPMOST" with `topmost=1` and "Tally probe control PLAIN" with `topmost=0`. Anything else = STOP.
- B3c. **Reader known-answer check**: `ctx-reader.ps1 -Check -Out <evidence>\ctx-check.txt`. EXPECT the last line `reader check ok`. `reader check FAILED` = STOP (the PEB offsets are wrong for this Windows; report both CTX lines).
- B4. The screen tool's frame, and the screen's logical size and scale. Every click below is given in both.
- B5. List every process whose command line contains `--confirm-journal` (pid, parent, start, image, command line). EXPECT none. If any: record, do not end it, STOP.

## Parts F and G: the cells (X and Y first)
**Order: F1, F2, F3, F4, G1, G2, G3.** Run all seven unless a cell's box does not appear (step 5). Every cell is the same recipe. Tally MAXIMISED on the Gateway of Tally and in front.

| cell | target | stdio | parent kind | note |
|---|---|---|---|---|
| F1 (X) | child | tokio | interactive | tokio's stdio from an interactive parent |
| F2 (Y) | child | files | launcher | file redirects under the lab's launcher |
| F3 | child | tokio | launcher | the stdio of the product, the launcher of the lab |
| F4 | child | tokio | hidden | |
| G1 | control | tokio | interactive | the 1780-E control box (variant c) |
| G2 | control | tokio | hidden | |
| G3 | control | tokio | launcher | |

**The recipe for a cell** (the box's title is "ComplyEaze Bridge" for F, "Lab topmost control" for G):
1. Start the trace in its own window, BEFORE the parent and without hashing the exe first: `box-trace-early.ps1 -Title "<title>" -Out <evidence>\trace-<cell>.txt -Seconds 100`. Click Tally's title area so Tally is in front.
2. Start the parent of the cell (kind and command above). Stamp `<cell>-T0` at the command. Click Tally's title area at once. The parent writes `<evidence>\<cell>.txt` (its pid, the child's pid, the command line, the stdio kind, the creation flags).
3. At `<cell>-T0+6`: `ctx-reader.ps1 -Pids '<parent pid> <child pid>' -Out <evidence>\ctx-<cell>.txt` (the child's pid is in `<cell>.txt`; if the box's process is not that pid, add the trace's `box_pid`). Record every CTX, CHAIN and CONHOST line.
4. Probes at `<cell>-T0+8` and `<cell>-T0+11`, each as the pair `zorder-probe.ps1` then `zorder-probe-k.ps1`, start and end stamped; a whole-screen screenshot after the second.
5. If no box with the title appears by `<cell>-T0+15`: record "the box did not appear", do not end anything, read `<cell>.txt` and `<cell>.stderr` if present, and go on to the next cell.
6. At `<cell>-T0+14` click ONE blank point in Tally at least 150 logical pixels from every edge of the box (record it in both frames and its distance). Probes and a screenshot at once and 6 s later. **No taskbar click in parts F and G.**
7. Answer **No** by mouse (through the box's taskbar button if Tally covers it; record that). EXPECT, for F cells, `exit_code=1` and `stdout_bytes_read=0` in `<cell>.txt`; for G cells the parent's `<cell>.control.txt` holds `answer=7`. Exit code 77 means the box did not fit at an activation: record it. `exit_code=killed_after_timeout` means nothing answered it in 60 s: record it.
8. Record from the trace: `FIRST` and the first block's stamp, the box's `hwnd`, `class`, `exstyle`, `owner`, `topmost` in the first block and every later block, with the stamp of each change. A cell reads **0** if any block shows `topmost=0` for the box, else **1**.

## Part R: the server's child, read by pid (C1 again)
- P1. **Posting on for this launch**: in the launch's environment file set `BRIDGE_AGENT_ENABLE_WRITES` to `true`, point the launch at BUILD-T, record the file's key names (not values) and its sha256, then start the build with the lab's launcher.
- B2. Hash the build items. `tally_status` and `list_companies`: EXPECT BUILD-T and THE COMPANY as the one company. `ledger_masters` (basic): record the marks. Read the desktop client's version afresh.
- C0. `build_import_xml` with the exact content of `build-1591-s1.json`. EXPECT `voucher_count` 1 and `current_window_readable` with `source_rows` 0 for 20260914. Keep `batch_id` as BATCH-T. Any other = STOP.
- C1. Trace first (`box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-c1.txt -Seconds 150`); Tally maximised and in front; `post_import` with `company_guid` and BATCH-T; stamp T0; click Tally's title area at once.
- R1. At T0+6 read, with one `ctx-reader.ps1 -Pids '<server pid> <child pid> <the server's parent pid>' -Out <evidence>\ctx-c1.txt`, the three processes. The child is the process whose command line holds `--confirm-journal`; the server is its parent. Record every line, and the child's `in_job`, `integrity_rid`, `appcontainer`, `session`, `console_handle`, `std_in`, `std_out`, `std_err`, `startf` and `show_window`, `desktop`. Then read the same lines for the F1 child and the hand-run children, taken from `ctx-f1.txt` to `ctx-f4.txt`, side by side in RESULTS.md. A field that differs is a candidate, not a cause.
- C2. The box's text, as in row 1780-D: the same STOP rule for a difference in a line that names the company, GUID, number, voucher type, date, an entry, a total, the batch, the reference or the narration.
- C3. Probes (`zorder-probe.ps1`) at T0+8 and T0+13; a screenshot after the second. At T0+16 ONE click into Tally on a blank point at least 150 logical pixels from the box; probes and a screenshot at once and 6 s later. **No taskbar click.** Answer **No** by mouse at about T0+30 (through its taskbar button if needed; record it). Never click Yes.
- C4. After the box is gone, make no second `post_import`. Record the call's own answer whole, `egress_log` whole (any line for BATCH-T after T0), `local_data_report` for BATCH-T. EXPECT nothing dispatched.
- C5. End: `ledger_masters` (basic): the marks equal B2's. No `--confirm-journal` child remains. Re-hash the build items. Stop the launch, remove `BRIDGE_AGENT_ENABLE_WRITES`, record the environment file's new sha256.

## Part T: the taskbar click on a hand-run box, with its stderr kept
Which of the two the click does (the hook's re-measurement posting IDCANCEL when the box is measured as not fitting, or something else) is read from the child's own stderr, which part C discards but a file redirect keeps. No server, no request to Tally.
- T0. Tally MAXIMISED and in front. B5 shows no `--confirm-journal` process.
- T1. Trace as in the recipe (`trace-t.txt`). The cell is a child cell (`-Target child`) with `-Stdio files`, so that the child's stderr is kept: with the parent kind of F2 if F2 read 1, else the interactive kind. Name it T and record which. Its stderr is `<evidence>\t.stderr` (the files stdio writes it). Read it through a shared handle (`Get-Content`, or `[IO.File]::Open` with `FileShare.ReadWrite`).
- T2. Probes at T-T0+8 and T-T0+11; read `t.stderr` at T-T0+11 and record every line (EXPECT one `box-fit: box=(...) work=(...) fits=True` line per activation so far).
- T3. At T-T0+14 click ONE blank point in Tally at least 150 logical pixels from the box; probes and a screenshot at once and 6 s later.
- T4. If the box is below Tally: click its taskbar button ONCE; stamp; probe at once and 1.5 s later; read `t.stderr` at once and again 2 s later. Record every line with its order. Record the exit code and whether the box is still a window.
- T5. If the box is still there, answer **No** by mouse. EXPECT exit code 1 and an empty stdout.

## Part D: Claude Desktop as the parent (last, and only if parts F, G, R and T do not settle it) NEEDS THE OWNER AT THE PC
Run it only when the reading table below says "go on to D". It changes Claude Desktop's MCP configuration, which the owner approves and does.
- D0. The owner backs up Claude Desktop's MCP configuration file as it stands (name, size, sha256). Claude Desktop rewrites its own configuration: record the file again after it is closed.
- D1. With Claude Desktop closed, the owner adds a server entry whose command is BUILD-T's `bridge_mcp.exe` and whose environment holds the lab's usual settings and `BRIDGE_AGENT_ENABLE_WRITES=true` (key names only in the notes), then starts Claude Desktop.
- D2. Trace first; Tally maximised and in front. In a Claude Desktop chat the owner asks for `build_import_xml` with the exact content of `build-1591-s1.json`, then `post_import` for it. Stamp D-T0 at the second request. Click Tally's title area at once.
- D3. At D-T0+6 run `ctx-reader.ps1` on the server, its child and the server's parent (Claude Desktop's process). At D-T0+8 and D-T0+11 the probe pair; a screenshot. At D-T0+14 ONE blank click into Tally; probes at once and 6 s later.
- D4. **One taskbar click** on the box, once, after those probes; stamp; probe at once and 1.5 s later. Then answer **No** by mouse if the box is still there. Never Yes.
- D5. Afterwards the owner restores the configuration file from D0 with Claude Desktop closed and records its sha256 and that no `--confirm-journal` child remains. Remove `BRIDGE_AGENT_ENABLE_WRITES` and stop the launch.

## Reading it (for the requester, not the lab)
| reading | meaning |
|---|---|
| F1 reads 0 | **S1**: tokio's stdio shape (pipes and the NUL stderr with those creation flags) is enough to lose the bit, from an interactive parent. F2 reading 1 confirms it. |
| F1 reads 1, F2 reads 0 | **S2 (launcher)**: the lab launcher's context loses the bit whatever the stdio is. R1's fields for the launcher against the interactive parent name the difference. |
| F1 reads 1, F2 reads 1, F3 reads 0 | **S1 and S2 together**: pipes under the launcher, neither alone. |
| F4 reads 0 and F1 reads 1 | A hidden parent loses the bit (its window station, desktop or show-window state). R1's `desktop`, `session`, `show_window` and `startf` for it. |
| F1 to F4 all read 1 | The lab's parents do not reproduce C1. R1 against `ctx-f1` to `ctx-f4` names what the server's child has that they do not. **Go on to D**. |
| G cells read the same as F cells with the same kind and stdio | The loss belongs to the parent context, not to the dialog child. |
| G cells read 1 where the F cell of the same kind and stdio reads 0 | The loss is the child's: something it does after the spawn, with that context, not the context alone. |
| C1 (part R) reads 1 this time | The bit held in this sitting. The 1780-E reading and this one disagree: report both, with the Z-order lines, and do not go on to D. |
| T4: the stderr line before the box ends reads `fits=False`, the exit code 77 | The close at a taskbar click is `on_activate` measuring the box as not fitting at re-activation. The numbers on the line say why. |
| T4: the box ends and the line reads `fits=True`, the exit code not 77 | The close is not the fit check. |
| T4: the box stays | The taskbar click is not what ended it in 1780-E; read the parent's own cell (F) for its exit code. |
| D: the box reads 0 and the lab cells read 1 | **S3**: Claude Desktop as the parent. |
| D: the box reads 1 | The bit held under Claude Desktop in this sitting: report it. |

- **FAIL of the row:** a click or a key answers a box with Yes; any post; a mark changes; a box or a child left after the sitting; B3b's or B3c's known answer not met; the configuration file of part D not restored.

## Evidence (stays on the lab PC except the files listed)
- RESULTS.md: one table, one line per step (B3b, B3c, F1 to F4, G1 to G3, B5, C0 to C5, T1 to T5, D if run), every stamp and every probe line (position, topmost, foreground, pid, title), each cell's reading (0 or 1), and the side-by-side table of R1's lines.
- From each trace, in RESULTS.md: `FIRST` and the first block's stamp, the `PROC` lines, for the box's own window its `hwnd`, `class`, `thread`, `owner` and `exstyle`, every stamp at which its `topmost` changed with the values either side, the windows of its process at the first and last block, and any `fg_pid` change.
- Each `<cell>.txt`, `ctx-<cell>.txt`, `t.stderr` and the exit codes, whole.
- Commit the trace and context files whole. SHA256.txt over the committed files, with sizes. notes.txt in time order (UTC). SCREENSHOTS.txt: name, size and sha256 of every capture (they stay local).
- Also record: BUILD-T's pull request, head, CI run, artifact and exe sha256; the screen size and scale; the Windows build, the Tally release and the desktop client version, read afresh; the launcher's command line.
- Evidence on a new branch, `lab/1780-topmost-spawn-answers`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- Keys, menu clicks and fields in Tally.
- A second monitor; macOS; Windows 11.
- The ledger-create, review-acknowledgement and invoice-stop boxes.
- Any change to the build: this row only reads.
- A parent that is not one of the kinds above (a service, a scheduled task, a remote session).
- The job object's own UI limits: `ctx-reader.ps1` reports whether a process is in any job and which named jobs exist, not the limits of an unnamed job.
