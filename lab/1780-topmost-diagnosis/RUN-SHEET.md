# Windows item 1780-D v1: why the approval box on #2107's build reads topmost 0. A Windows-only control first, then row 1780-C2 again with a window trace by handle. Answered No by mouse (Part of #1780)

WRITES to the book: NONE. Both boxes are answered **No, by mouse**. Nothing is keyed in Tally. The one click into Tally in each part lands on a blank part of its screen.

## Why
- Row 1780-C2 (lab/1780-approval-box-topmost-rerun-answers @ 558a7ee0) found, on #2107's build:
  - the box reads `topmost` 0 at T0+8 and T0+13;
  - one click into a maximised Tally put Tally above the box.

  The build asks for topmost twice: `MB_TOPMOST` on `MessageBoxW`, and `SetWindowPos(HWND_TOPMOST)` once the box is shown.
- The probe control (lab/1780-topmost-probe-control-answers @ 11f56ca5) showed the probe reads the flag correctly. The C2 probe matched exactly one window with the box's title. So that window, the dialog itself, had the flag clear.
- **What this row decides, between five candidates (D1 to D5; part K and steps K1, K2 are the control):**

  | | candidate |
  |---|---|
  | D1 | Windows does not keep a box topmost when a background process asks for it, even with no Bridge code involved. |
  | D2 | The flag is set, then cleared after both requests. |
  | D3 | The box has an owner whose non-topmost state is carried to it. |
  | D4 | The box on screen was not shown by #2107's code: a stale child, or another executable. |
  | D5 | The build raised a different window of the same process (an addition to D2 or D3, not a cause on its own). |

  - T1. The control box (no Bridge code): its `topmost` once shown, and after one click into Tally.
  - T2. The approval box: `topmost` in every 50 ms sample from its first appearance, and when it changes.
  - T3. The approval box's owner, class and process, and every other window of that process.
  - T4. After the box is gone: the call's final answer, `egress_log`, and what the local journal says. Row C2 left this open.

## Who acts
- On this synthetic company the lab does every step itself, including the clicks into Tally and both Nos. This is the owner's standing word of 27 Oct.
- The lab records each such step as "done by the lab on the owner's standing word of 27 Oct". Posting is switched on for part C and off after it, on the owner's word of 28 Oct.
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first call and at the end; a different hash = STOP)
| item | source | bytes | sha256 |
|---|---|---|---|
| `bridge-tally.mcpb`, the bundle BUILD-T comes in | #2107 at head a66c31f2ab607b36701ea0474ca15d474b9e202c: CI run 40709511734, artifact 12826362269 `unsigned-bundle-smoke-windows-latest` (expires 4 Nov 2026 15:05 UTC), file `bridge-tally.mcpb` | 15,320,736 | 61e52d079544b2c7dd49cb47bf08a11120b0154957e830e8a76e598970ab99c2 |
| `bridge_mcp.exe`, BUILD-T | `bin/x86_64-pc-windows-msvc/bridge_mcp.exe` inside that `.mcpb`; its server reports version 0.6.1 | 31,804,416 | 3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974 |
| `build-1591-s1.json` (one Receipt, 14 Sep 2026, 141.00, "Lab topmost probe 1") | `requests/` in this folder, a byte copy of lab/1780-approval-box-topmost-rerun @ aa888c74751b25a9d81e673fad3cabdb0f18ada7 | 308 | 5f8a5ff24d416331ec2706774d178ba615759abafa87b7c408c279eca88bf13a |
| `zorder-probe.ps1` (read-only list of the top-level windows from the top down: position, topmost, foreground, pid, title) | `requests/` in this folder, a byte copy from the same commit | 2405 | 3f03768e83c4f567957c52b8fda3f6f67c0f1d12c7eceeeb9cbc11a0dd4f0fcb |
| `control-forms.ps1` (two small windows, one TopMost and one plain, for the trace's known answer) | `requests/` in this folder, a byte copy of lab/1780-topmost-probe-control-answers @ 11f56ca5c56523a587ceeabde26f23e053526643. Its `<lab folder>` placeholder is replaced by the working folder, as that row recorded in its `substitutions.txt` | see `SHA256.txt` | see `SHA256.txt` |
| `box-trace.ps1` (read-only: follows one box and every top-level window of its process, by handle, every 50 ms; written for this row, not yet run) | `requests/` in this folder | see `SHA256.txt` | see `SHA256.txt` |
| `control-box.ps1` (one plain `MessageBoxW`, flags `MB_YESNO \| MB_TOPMOST \| MB_SETFOREGROUND`, no owner, title "Lab topmost control"; written for this row, not yet run) | `requests/` in this folder | see `SHA256.txt` | see `SHA256.txt` |

- THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`. Step P0 checks it is there.
- Working folder: `C:\bridge-lab\1780d\`. Evidence goes in `C:\bridge-lab\1780d\evidence\`.
- `SHA256.txt` in this folder lists every file of the folder with its size. Check it before step P0. A different hash or size = STOP.
- The three `.ps1` files are run with `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...` from Windows PowerShell 5.1. Record each command line exactly as typed, with any substitution.

## Before the sitting
- P0. (preflight, before anything else) Open the list of companies on the lab PC: Tally's own Select Company list.
  - If BRIDGE BATCH LAB is not in the list: STOP and report. Do not restore it from a backup and do not use another book in its place.
  - If it is there but is not the one company loaded: load it alone, then go on.
- B1. Backup of THE COMPANY (BK0), named, with size and sha256.
- B3. **Probe known-answer check**, as in row 1780-C2's B3. Record P-time, how long one probe run takes from start to end.
- B3b. **Trace known-answer check.**
  - Run `control-forms.ps1 -Seconds 20`.
  - While its two windows show, run `box-trace.ps1 -Title "Tally probe control" -Out <evidence>\trace-b3b.txt -Seconds 10`.
  - EXPECT in `trace-b3b.txt`:
    - a `FIRST` line;
    - `PROC` lines naming `powershell.exe`;
    - `W` lines for both windows of that pid: "Tally probe control TOPMOST" with `topmost=1`, and "Tally probe control PLAIN" with `topmost=0`.

    Anything else = STOP: the trace does not read what this row needs.
  - Record the number of `T` and `H` lines and the file's size.
- B4. **The two frames**, as in row 1780-C2's B4: the screen tool's frame, and the screen's logical size and scale. Every click below is given in both.

## Part K: the control, no Bridge code (T1)
Bridge is not involved in part K, and no launch is running.
- K1. **Hidden process.** Tally MAXIMISED on the Gateway of Tally and in front.
  - Start the trace first: `box-trace.ps1 -Title "Lab topmost control" -Out <evidence>\trace-k1.txt -Seconds 60`, in its own PowerShell window. Then click Tally's title area so Tally is in front again.
  - From a second PowerShell, run `Start-Process powershell -WindowStyle Hidden -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File <folder>\requests\control-box.ps1 -DelaySeconds 5 -Out <evidence>\control-k1.txt'`. At once, click Tally's title area. Stamp K1-T0.
  - Probes at K1-T0+8 and K1-T0+11 with `zorder-probe.ps1`, start and end stamped, and a whole-screen screenshot at K1-T0+11.
  - **If no "Lab topmost control" line appears by K1-T0+15**: record "the box did not appear (hidden start)".
    - End the hidden process with `Stop-Process` on the pid of `powershell.exe` whose command line names `control-box.ps1` (record the pid and the command line).
    - Go to K2. A start hidden by `SW_HIDE` can hide the box itself; K2 avoids it.
  - **If the box is shown**:
    1. At K1-T0+14, pick one blank point in Tally at least 150 logical pixels from every edge of the box, record it in both frames, and click it ONCE.
    2. Probe and screenshot at once and 6 s later.
    3. Then answer **No** by mouse (via its taskbar button if Tally covers it; record that).
    4. EXPECT `answer=7` in `control-k1.txt`.
- K2. **Background window.** Only if K1's box did not appear. Otherwise skip it and write "skipped: K1 showed the box".
  - Start the trace as in K1, to `trace-k2.txt`.
  - In a normal PowerShell window run `control-box.ps1 -DelaySeconds 8 -Out <evidence>\control-k2.txt`. At once, click Tally's title area, so Tally covers that window and is in front when the box is asked for. Stamp K2-T0.
  - Then the same probes, click and No as in K1, at K2-T0+11, +14 and +20.

## Part C: the approval box, traced (T2, T3, T4)
- P1. **Posting on for this launch.** In the launch's environment file set `BRIDGE_AGENT_ENABLE_WRITES` to `true`, and point the launch at BUILD-T. Record the file's key names (not values) and its sha256. Then start the build.
- B2. Hash the build items above.
  - `tally_status` and `list_companies`: EXPECT BUILD-T and THE COMPANY as the one company.
  - `ledger_masters` (basic): record the voucher and master marks. Every later compare is against these.
  - Read and record the desktop client's version afresh.
- B5. **Before T0, the dialog children.** List every process whose command line contains `--confirm-journal`, with pid, parent pid, start time, image path and command line (`Get-CimInstance Win32_Process`).
  - EXPECT none.
  - If any is listed: record it, do not end it, and STOP. The coordinator decides; a stale child is candidate D4's evidence.
- C0. `build_import_xml` with the exact content of build-1591-s1.json.
  - EXPECT `result.voucher_count` 1, and `total_debit` and `total_credit` each 141.
  - EXPECT `verification_preflight.state` `current_window_readable`, with `source_rows` 0 for 20260914.
  - Keep `result.batch_id` as BATCH-T.
  - `source_rows` other than 0, or any refusal = STOP. Nothing has been sent to Tally.
- C1. Start the trace: `box-trace.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-c.txt -Seconds 120`, in its own PowerShell window.
  - Tally MAXIMISED on the Gateway of Tally and in front; click its title area only.
  - Then `post_import` with BOTH `company_guid` `a30f23c3-4819-4334-96ab-ffa6fba70976` and `batch_id` BATCH-T. Stamp T0 (UTC, ms).
  - Do NOT click into Tally after T0 until C3.
  - Probes with `zorder-probe.ps1` at T0+8 and T0+13, each with its start and end stamps. Whole-screen screenshot at T0+13, after its probe ends.
  - **If the trace's `FIRST` line names a process whose command line lacks `--confirm-journal`, or another title**: record it. Answer the real box No by mouse, then STOP.
- C2. From the T0+13 probe and screenshot: the box's title, its first line, and No as the default, exactly as row 1780-C2's C2 lists them.
  - The same **STOP rule** as row 1780-C2: a difference in any line that names the company, its GUID or number, the voucher type, the date, an entry, a total, the batch, the reference or the narration = No by mouse, then STOP. Type nothing.
- C3. **The click into Tally, at T0+16 s (±2 s)**, exactly as row 1780-C2's C3: one blank point at least 150 logical pixels from the box, recorded in both frames with its distance, and clicked ONCE. Probe and screenshot at once and 6 s later.
  - **If the click answered the box**: STOP. Make no further `post_import` call and report.
- C4. Answer **No by mouse** as soon as the C+6 probe ends, aiming for before T0+38 s.
  - If the box is hidden behind Tally, click its taskbar button once to bring it back, and record that you did, with a stamp and a probe straight after. Then click No.
  - Never click Yes.
  - **If the box is gone after the taskbar click without No being clicked** (as in row 1780-C2): record the stamp, take one probe and one screenshot, and click nothing more. C5 records the outcome.
- C5. **After the box is gone (T4).** Make no second `post_import` call: it would wait on, or ask for, the box again.
  - Record:
    - the `post_import` call's own answer, whole: `approval.state`, `dispatch`, `attempt_recorded`, `previous_approval` and any `next_step`;
    - and its stamp.
  - Then, read only:
    - `egress_log`, whole. Record any line for BATCH-T or THE COMPANY after T0.
    - `local_data_report`, whole. Record what it says of BATCH-T: its import journal lines, and any approval or decline.
  - EXPECT:
    - nothing dispatched;
    - no egress line that sends BATCH-T to Tally;
    - the journal holding the built batch and no dispatch.
- C6. End:
  - `ledger_masters` (basic): the marks are unchanged from B2.
  - No `bridge_mcp.exe` child with `--confirm-journal` remains (process list).
  - Let `trace-c.txt` end on its own (`GONE` or `END`).
  - Re-hash the build items.
  - Stop the launch, remove `BRIDGE_AGENT_ENABLE_WRITES` and record the environment file's new sha256. Shut THE COMPANY.

## Evidence (stays on the lab PC except the files listed)
- RESULTS.md: one table, one line per step (B3b, K1, K2, B5, C0 to C6), with every stamp and every probe line (position, topmost, foreground, pid, title).
- From each trace file (`trace-b3b.txt`, `trace-k1.txt` or `trace-k2.txt`, `trace-c.txt`), in RESULTS.md:
  - the `FIRST` stamp;
  - the `PROC` lines;
  - for the box's own window (by its title): its `hwnd`, `class`, `thread`, `owner` and owner fields, and **every stamp at which its `topmost` changed**, with the values either side;
  - the number of windows of the box's process, and each one's class, visibility and `topmost` at the first block and at the last;
  - any `fg_pid` change, with its stamp.
- Commit the trace files whole. They are bounded at the source: one block per change, and otherwise one heartbeat line a second.
- notes.txt: the same in time order.
- SCREENSHOTS.txt: name, size and sha256 of every capture (they stay local: whole-screen captures show other windows).
- SHA256.txt over the committed files, with sizes.
- Also record:
  - BUILD-T's pull request, head, CI run, artifact and exe sha256;
  - the screen size and scale, and whether there is one monitor;
  - the Windows build, the Tally release and the desktop client version, read afresh.

## Reading it (for the requester, not the lab)
| reading | candidate |
|---|---|
| The control box (K1 or K2) reads `topmost` 0 once shown, or Tally covers it after the click | **D1**: Windows does not keep the flag for a box a background process shows. The fix must change who shows the box, or how, not ask again. |
| The control box reads `topmost` 1 and stays above Tally after the click | The mechanism is in Bridge's process. Go on to the trace. |
| In `trace-c.txt` the box's window reads `topmost=1` in some block and `topmost=0` in a later one | **D2**: set, then cleared. A change within about 150 ms of `FIRST` points at the box's own start or the build's raise. A later change with no click and no `fg_pid` change points outside the process. |
| The box's window reads `topmost=0` from `FIRST` on, its `owner` is not 0, and the owner reads `owner_topmost=0` | **D3**: owned by a non-topmost window. The owner's class and pid say which. |
| `PROC sha256` is not BUILD-T's exe hash, the command line lacks `--confirm-journal`, or B5 listed a child before T0 | **D4**: the box was not #2107's. |
| The box's process has another visible top-level window at `FIRST`, on the same thread | **D5**: the build's raise may have taken it. This adds to whichever row above holds. |
| `topmost=0` from `FIRST`, owner 0, the hash right, one visible window, and the control reads 1 | **None of the five.** Report it as such. |
| C5 | T4: whether the closed box answered, declined or expired, and that nothing was sent. |

- **FAIL of the row:**
  - a click or a key answers a box;
  - any post;
  - a mark changes;
  - a box left open or an orphan child after C6;
  - B3b's known answer not met.

## Not measured (say so in the results)
- Keys, menu clicks and fields in Tally.
- A second monitor; macOS; Windows 11.
- The ledger-create, review-acknowledgement and invoice-stop boxes.
- Any change to the build: this row only reads.
