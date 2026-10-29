# Windows item 1780-E v1: where the approval box on #2107's build loses its topmost bit, and what ends it at a taskbar click. A control with the box's own flags and then its process's features one at a time (K1c to K1f), the dialog child run by hand (H1), and the post with no taskbar click (C1). Answered No by mouse (Part of #1780)

WRITES to the book: NONE. Every box is answered **No, by mouse**. Nothing is keyed in Tally. The one click into Tally in each run lands on a blank part of its screen. Part H sends no request to Tally and starts no Bridge server.

## Why
- Row 1780-D (lab/1780-topmost-diagnosis-answers @ 5c6dbf2ad8ab3b0c3ae5df29b9cf3c73ba32b759; the probe was checked in lab/1780-topmost-probe-control-answers @ 11f56ca5) found, on #2107's build (head a66c31f2):
  - a plain `MessageBoxW` with `MB_YESNO | MB_SETFOREGROUND | MB_TOPMOST` from a hidden process reads `topmost` 1 (extended style 0x00010109) and stays above a maximised Tally after a click into Tally;
  - the approval box (class `#32770`, owner 0, the build's own exe) reads `topmost` 0 (0x00010101) in every trace block from 2.28 s after it appeared, and the probe read 0 at 1.13 s. The two styles differ by exactly `WS_EX_TOPMOST` (0x8);
  - the source asks for topmost twice (`MB_TOPMOST` at approved_import.rs `show_review`, and `SetWindowPos(HWND_TOPMOST)` in box_fit.rs `raise`) and has nothing that clears it;
  - one taskbar click closed the box within 117 ms, with no No, and the journal holds no approval or decline line.
- **What differs between the control and the approval box** (the sheet changes one thing at a time, in the order of `box_fit::guarded`):

  | | candidate | the step that tests it |
  |---|---|---|
  | E1 | the flags: the approval box adds `MB_YESNOCANCEL` for `MB_YESNO`, `MB_DEFBUTTON2` and `MB_ICONWARNING` | K1c |
  | E2 | the process calls `SetProcessDpiAwarenessContext(PER_MONITOR_AWARE_V2)` before the box | K1d |
  | E3 | a `WH_CBT` hook on the thread (box_fit.rs `on_activate`) | K1e |
  | E4 | the helper thread (box_fit.rs `Front`, `raise`) | K1f |
  | E5 | something else in the dialog child's own code or exe | H1 reads 0 while K1f reads 1 |
  | E6 | the parent's spawn context (`nonce_bound_dialog`: stdin and stdout piped, stderr null, `kill_on_drop`) | H1 reads 1 while C1 reads 0 |

- **What ends the box at a taskbar click** (T4): box_fit.rs `on_activate` runs at every `HCBT_ACTIVATE` of the box's thread, not only the first. It measures the box against the monitor's work area and, for a box not wholly inside it, posts `IDCANCEL`; the child then exits 77, which the parent turns into `import_review_too_large`. A taskbar click activates the box again. Its measurement line goes to the child's stderr, which the parent discards (`.stderr(Stdio::null())`), so the evidence of 1780-D cannot show it. The other candidate is the parent's own end of the child: the dialog's whole life is bounded at 120 s from the spawn, and `kill_on_drop` kills the child when the parent's future is dropped. Both rows so far clicked the taskbar 20 to 24 s after the call answered `pending`.

## Who acts
- On this synthetic company the lab does every step itself, including the clicks into Tally and both Nos, on the owner's standing word of 27 Oct 2026.
- The lab records each such step as "done by the lab on the owner's standing word of 27 Oct".
- Posting is switched on for part C only, on the owner's word of 28 Oct, and off after it.
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first step and at the end; a different hash = STOP)
| item | source | bytes | sha256 |
|---|---|---|---|
| `bridge-tally.mcpb` and `bridge_mcp.exe`, BUILD-T | #2107 at head a66c31f2ab607b36701ea0474ca15d474b9e202c: CI run 40709511734, artifact 12826362269 `unsigned-bundle-smoke-windows-latest`; the copy rows 1780-C2 and 1780-D used, nothing is downloaded | mcpb 15,320,736; exe 31,804,416 | mcpb 61e52d079544b2c7dd49cb47bf08a11120b0154957e830e8a76e598970ab99c2; exe 3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974 |
| `requests/box-trace.ps1`, `zorder-probe.ps1`, `control-forms.ps1`, `build-1591-s1.json` | byte copies from lab/1780-topmost-diagnosis @ 4cd8ac541df96355a874febb035c782416c3f58e. `control-forms.ps1` keeps its `<lab folder>` placeholder: replace it by the working folder in the copy you run, as row 1780-D did (and record it) | see `SHA256.txt` | see `SHA256.txt` |
| `requests/zorder-probe-k.ps1` | a byte copy of lab/1780-topmost-diagnosis-answers @ 5c6dbf2a, `scripts/zorder-probe-k.ps1` (the probe with the title "Lab topmost control" added to its filter) | see `SHA256.txt` | see `SHA256.txt` |
| `requests/control-box-product.ps1` | written for this row, not yet run (its C# part compiled on a Mac with Mono's `mcs`, no errors) | see `SHA256.txt` | see `SHA256.txt` |
| `requests/confirm-input.txt` | written for this row: what the parent writes to the dialog child's stdin (a nonce line, the count line `1`, then the text of the box), with placeholders for the company number, the GUID and the amounts | see `SHA256.txt` | see `SHA256.txt` |

- THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, loaded alone. Step P0 checks it.
- Working folder: `C:\bridge-lab\1780e\`. Evidence goes in `C:\bridge-lab\1780e\evidence\`.
- `SHA256.txt` lists every other file of this folder with its size. Check it before step P0. A different hash or size = STOP.
- The `.ps1` files are run with `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...` from Windows PowerShell 5.1. Record each command line exactly as typed, with any substitution.
- **The number format of this PC (row 1780-D, deviation 2).** A comma is a digit separator there: `-ProbeAt 8,11` was read as 811. No command in this sheet takes a list, and none may be written with a comma between numbers. A list of probe times is one quoted string with spaces (`'8 11'`).
- The lab's 1780-D helpers (`k-run.ps1`, `c-run.ps1`, `c4-taskbar.ps1`, `b3.ps1`, `procs.ps1`; kept on the lab PC, pushed with the working folder replaced in lab/1780-topmost-diagnosis-answers @ 5c6dbf2a, `scripts/`) may be reused to keep the timings. Each edit made to them is a deviation, written in `notes.txt`. They need at least: the control script and variant as parameters (K), a stop after C3 and the later probes (C), and no hard-coded handle of Tally's window (it changes with each Tally process).

## Before the sitting
- P0. Tally's own Select Company list. If BRIDGE BATCH LAB is not in it: STOP and report. Do not restore it from a backup and do not use another book. If it is there but is not the one company loaded: load it alone.
- B1. Backup of THE COMPANY (BK0), named, with size and sha256.
- B3. **Probe known-answer check**, as in row 1780-D: with Tally's title clicked, `zorder-probe.ps1` prints Tally first and in front. Record P-time.
- B3b. **Trace known-answer check**, as in row 1780-D: `control-forms.ps1 -Seconds 20` while `box-trace.ps1 -Title "Tally probe control" -Out <evidence>\trace-b3b.txt -Seconds 10` runs. EXPECT a `FIRST` line, `PROC` lines naming `powershell.exe`, "Tally probe control TOPMOST" with `topmost=1` and "Tally probe control PLAIN" with `topmost=0`. Anything else = STOP.
- B4. **The two frames**: the screen tool's frame, and the screen's logical size and scale. Every click below is given in both.
- B5. List every process whose command line contains `--confirm-journal` (pid, parent, start, image, command line). EXPECT none. If any: record, do not end it, STOP.

## Part K: the control ladder (T1)
Bridge is not involved and no launch is running. Run **K1c, then K1d, then K1e, then K1f**, each with the same recipe, and **stop at the first one that reads `topmost` 0** (write "skipped: K1x read 0" for the rest). Then go on to part H whatever K read.

**The recipe for K1x** (x is c, d, e or f; `-Variant x`). Tally MAXIMISED on the Gateway of Tally and in front.
1. Start the trace in its own window: `box-trace.ps1 -Title "Lab topmost control" -Out <evidence>\trace-k1x.txt -Seconds 80`. Click Tally's title area so Tally is in front again.
2. From a second PowerShell: `Start-Process powershell -WindowStyle Hidden -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File <folder>\requests\control-box-product.ps1 -Variant x -DelaySeconds 5 -Out <evidence>\control-k1x.txt'`. At once click Tally's title area. Stamp K1x-T0.
3. Probes at K1x-T0+8 and K1x-T0+11, each as the pair `zorder-probe.ps1` then `zorder-probe-k.ps1`, start and end stamped; a whole-screen screenshot after the second.
4. If no "Lab topmost control" line appears by K1x-T0+15: record "the box did not appear", end the hidden process (record its pid and command line), and STOP the ladder: report.
5. At K1x-T0+14 click ONE blank point in Tally at least 150 logical pixels from every edge of the box (record it in both frames and its distance). Probes (the pair) and a screenshot at once and 6 s later.
6. **If the box is below Tally** after that click: click its taskbar button ONCE (record the stamp and both frames' coordinates). Probe at once and again 1.5 s later; note whether the box is still a window. For K1e and K1f read `control-k1x.txt.fits.txt` at once (the `HCBT_ACTIVATE` and `box-fit` lines carry UTC stamps) and record every line.
7. **If the box is still there**, answer **No** by mouse (through its taskbar button if Tally covers it; record that). EXPECT `answer=7` in `control-k1x.txt`. If the box vanished after the taskbar click, `control-k1x.txt` should hold `answer=2` (Cancel): record it and the `.fits.txt` lines whole.
8. Record from the trace: `FIRST`, the box's `hwnd`, `class`, `exstyle`, `topmost` in the first block and in every later block, and the extended style of the box in each (a change of `topmost` with its stamp).
9. Each variant's `control-k1x.txt` line also records the DPI and hook results (`dpi_set`, `thread_is_per_monitor_v2`, `hook_installed`, `front_started`): a variant whose feature did not take effect (`dpi_set=False`, `hook_installed=False`) is recorded and read as "not tested", and the ladder goes on.

## Part H: the dialog child by hand (T4 ii, and E5 or E6)
No Bridge server and no request to Tally. The child only shows a box; it prints its token to stdout only after a Yes, which is never clicked, and no parent exists to read one.
- H0. Tally MAXIMISED on the Gateway of Tally and in front (title area clicked). B5 shows no `--confirm-journal` process.
- H1. Start the trace in its own window: `box-trace.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-h1.txt -Seconds 100`. Click Tally's title area.
- H2. From a second PowerShell, one command: `$p = Start-Process -FilePath '<build folder>\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe' -ArgumentList '--confirm-journal' -NoNewWindow -PassThru -RedirectStandardInput '<folder>\requests\confirm-input.txt' -RedirectStandardOutput '<evidence>\h1-stdout.txt' -RedirectStandardError '<evidence>\h1-stderr.txt'; $null = $p.Handle`. Stamp H-T0 and record the pid. (`$null = $p.Handle` is needed: without it PowerShell's process object returns no exit code, as the curl helper of row 2119 showed.) `h1-stderr.txt` is open for writing while the child runs: read it through a shared handle (`Get-Content`, or `[IO.File]::Open` with `FileShare.ReadWrite`). (The exe's own path is the one the trace's `PROC` line gave in row 1780-D; its sha256 is BUILD-T's.) At once click Tally's title area.
- H3. Probes (`zorder-probe.ps1`) at H-T0+8 and H-T0+13, start and end stamped; a whole-screen screenshot after the second. Read `h1-stderr.txt` at H-T0+13 and record every line (EXPECT one `box-fit: box=(...) work=(...) fits=True` line per activation so far).
  - **If no box with "ComplyEaze Bridge" in its title appears by H-T0+15, or the box's text differs in its line count from `confirm-input.txt`'s 23 lines**: record it, end nothing, and STOP.
- H4. At H-T0+16 click ONE blank point in Tally at least 150 logical pixels from every edge of the box (as in C3 of row 1780-D). Probes and a screenshot at once and 6 s later. Record `h1-stderr.txt`'s line count at C+6.
- H5. If the box is below Tally: click its taskbar button ONCE; stamp; probe at once and 1.5 s later; read `h1-stderr.txt` again and `$p.HasExited` and `$p.ExitCode`. Record every stderr line with the order it was written.
- H6. If the box is still there, answer **No** by mouse (through its taskbar button if needed). EXPECT `$p.ExitCode` 1 and an empty `h1-stdout.txt`. Exit code **77** means the box did not fit at an activation: record it with the stderr lines.
- H7. The trace gives, for the hand-run box, the same window facts as in part K: `topmost` in every block, `owner`, `exstyle`.

## Part C: the post with no taskbar click (T4 i)
- P1. **Posting on for this launch**: in the launch's environment file set `BRIDGE_AGENT_ENABLE_WRITES` to `true`, point the launch at BUILD-T, record the file's key names (not values) and its sha256, then start the build.
- B2. Hash the build items. `tally_status` and `list_companies`: EXPECT BUILD-T and THE COMPANY as the one company. `ledger_masters` (basic): record the marks. Read the desktop client's version afresh.
- C0. `build_import_xml` with the exact content of `build-1591-s1.json`. EXPECT `voucher_count` 1 and `current_window_readable` with `source_rows` 0 for 20260914. Keep `batch_id` as BATCH-T. Any other = STOP (nothing has been sent to Tally).
- C1. As in row 1780-D: the trace `box-trace.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-c.txt -Seconds 150` first; Tally maximised and in front; `post_import` with `company_guid` and BATCH-T; stamp T0; probes with `zorder-probe.ps1` at T0+8 and T0+13; a screenshot after the second.
- C2. The box's text, as in row 1780-D: the same STOP rule for a difference in a line that names the company, GUID, number, voucher type, date, an entry, a total, the batch, the reference or the narration.
- C3. At T0+16 s (plus or minus 2 s) ONE click into Tally on a blank point at least 150 logical pixels from the box; probes and screenshots at once and 6 s later.
- C4. **No taskbar click and nothing else is clicked.** The `post_import` call answers `pending` at about T0+40 s; stamp that answer as P-T (EXPECT `dialog_remaining_s` near 86).
  - Probes (`zorder-probe.ps1`) at P-T+30, P-T+50 and P-T+70, each start and end stamped, and a screenshot after the last. The trace keeps running.
  - **Why only 70 s, not the 90 s the request named:** the dialog's whole life is bounded at 120 s from its spawn, and the call's `dialog_remaining_s` was 86 at `pending`. At about P-T+86 s the parent ends the child itself, which would hide what this step measures.
  - **If the box has gone before P-T+70** (the trace shows a block with `windows=0`, or a probe without it): record the stamp of the last block with the box and of the first without it, take one probe and one screenshot, click nothing. C5 records the outcome.
  - **If the box is still open at P-T+70**: click its taskbar button ONCE (record it), probe at once, then answer **No** by mouse. Never click Yes.
- C5. After the box is gone, make no second `post_import`. Record the call's own answer whole, `egress_log` whole (any line for BATCH-T after T0), and `local_data_report` for BATCH-T. EXPECT nothing dispatched and no line that sends BATCH-T to Tally.
- C6. End: `ledger_masters` (basic): the marks equal B2's. No `--confirm-journal` child remains. Re-hash the build items. Stop the launch, remove `BRIDGE_AGENT_ENABLE_WRITES`, record the environment file's new sha256. Shut THE COMPANY.

## Reading it (for the requester, not the lab)
| reading | meaning |
|---|---|
| K1c reads 0 (or Tally covers the box after the click) | **E1**: the flag set the approval box asks for does not give a topmost box. Bisect the flags next (YESNOCANCEL, DEFBUTTON2, ICONWARNING one at a time). |
| K1c reads 1, K1d reads 0 | **E2**: the DPI call. |
| K1d reads 1, K1e reads 0 | **E3**: the hook (read its `.fits.txt` for what it measured). |
| K1e reads 1, K1f reads 0 | **E4**: the helper thread's `raise`. |
| K1c to K1f all read 1, H1 reads 0 | **E5**: the dialog child's own code or exe, outside the features above. |
| K1c to K1f all read 1, H1 reads 1, C1 reads 0 | **E6**: the parent's spawn context. |
| K1c to K1f and H1 and C1 all read 1 | The bit holds in this sitting: report it, with the Z-order lines. |
| After the taskbar click the K1e or K1f box (or the H1 box) goes with `answer=2` / exit 77, and the `.fits.txt` / stderr line before it reads `fits=False` | **T4 is `on_activate`**: the box is cancelled at a re-activation because it was measured as not fitting. The measurement at that moment (the numbers) says why. |
| The box goes at the taskbar click but `fits=True` at that activation | The close is not the fit check: look at the parent (C1). |
| C1: the box vanishes at about P-T+20 to 25 s with no click | The parent ends the child at that age (T4 is the parent, not the click). |
| C1: the box is still open at P-T+70 | The taskbar click in rows 1780-C2 and 1780-D was the cause. |

- **FAIL of the row:** a click or a key answers a box; any post; a mark changes; a box or a hand-run child left after the sitting; B3b's known answer not met.

## Evidence (stays on the lab PC except the files listed)
- RESULTS.md: one table, one line per step (B3b, K1c to K1f, H1, B5, C0 to C6), every stamp and every probe line (position, topmost, foreground, pid, title).
- From each trace, in RESULTS.md: the `FIRST` stamp, the `PROC` lines, for the box's own window its `hwnd`, `class`, `thread`, `owner` and `exstyle`, every stamp at which its `topmost` changed with the values either side, the windows of its process at the first and last block, and any `fg_pid` change.
- `control-k1x.txt`, its `.fits.txt`, `h1-stderr.txt`, `h1-stdout.txt` and the exit code, whole.
- Commit the trace files whole. SHA256.txt over the committed files, with sizes. notes.txt in time order (UTC). SCREENSHOTS.txt: name, size and sha256 of every capture (they stay local).
- Also record: BUILD-T's pull request, head, CI run, artifact and exe sha256; the screen size and scale; the Windows build, the Tally release and the desktop client version, read afresh.
- Evidence on a new branch, `lab/1780-topmost-bisect-answers`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- Keys, menu clicks and fields in Tally.
- A second monitor; macOS; Windows 11.
- The ledger-create, review-acknowledgement and invoice-stop boxes.
- Any change to the build: this row only reads.
- The control boxes have no 23-line text: a hook measured on them shows their own size, not the approval box's.
