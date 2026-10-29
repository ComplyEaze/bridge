# Results: 1780-D, why the approval box on #2107's build reads topmost 0, 29 Oct 2026. The control box (no Bridge code, `MB_TOPMOST`, from a hidden process) reads `topmost` 1 and stays above Tally after the click, so **D1 is ruled out**. The approval box reads `topmost` 0 in every trace block from 2.28 s after it appeared, with owner 0, BUILD-T's exe and one visible window: **none of D3, D4, D5 holds, and D2 is not ruled out for the first 1.1 s** (see Reading). Nothing was posted; the marks did not move. The box closed after one taskbar click without No, as in row 1780-C2

The sheet is `lab/1780-topmost-diagnosis` at 4cd8ac541df96355a874febb035c782416c3f58e. `SHA256.txt` matched 7 of 7, hash and size.

**Who acted.** On this synthetic company the lab did every step itself:
- every click into Tally and both Nos, on the owner's standing word of 27 Oct;
- posting on for part C and off after it, on the owner's word of 28 Oct.

**Deviations (each also in `notes.txt`):**
1. **The sheet's probe does not print the control box.** `zorder-probe.ps1` prints only titles holding "ComplyEaze Bridge" or "Tally", so it can never print "Lab topmost control". In part K the lab ran it unchanged and, straight after each run, `scripts/zorder-probe-k.ps1`. That copy only adds the third title to the filter (sha256 77783ff7e8b02ca3d769548ed955545f779545244cbfc4ae01119d1b15a304fd; diff in `scripts/zorder-probe-k.diff`).
2. **K1 ran twice.**
   - In attempt 1 the lab's runner read `-ProbeAt 8,11` as 811 (this PC's number format), so no probe or capture ran at K1-T0+8 or +11. The lab stopped the runner with the box left open.
   - The late click (at about K1-T0+206 s) landed on a new trace window, not on Tally.
   - Attempt 1 is kept as the box's state over three and a half minutes. **K1b** is the K1 the sheet describes, run again with the runner fixed. K2 was skipped: K1 showed the box.
3. **B5's first listing matched only itself.** The first try typed the flag in its own command line and matched its own shells. It was run again from `scripts/procs.ps1`: 0.
4. **Paths replaced.** Pushed copies have lab-PC paths replaced by placeholders; `substitutions.txt` lists each one.

## Build and setup
- **BUILD-T:**
  - #2107, head a66c31f2ab607b36701ea0474ca15d474b9e202c;
  - CI run 40709511734, artifact 12826362269 `unsigned-bundle-smoke-windows-latest`;
  - `bridge-tally.mcpb` 15,320,736 B, sha256 61e52d079544b2c7dd49cb47bf08a11120b0154957e830e8a76e598970ab99c2;
  - `bridge_mcp.exe` 31,804,416 B, sha256 3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974 (server 0.6.1).

  This is the same copy row 1780-C2 used; nothing was downloaded. Both hashes are equal to the sheet and to the artifact's `mcpb-smoke.json`, at B2 and again at C6.
- **System:**
  - TallyPrime 7.2 Silver;
  - Windows 10.0.19045 22H2;
  - desktop client 3.6556.2;
  - one monitor, 1536 x 864 logical, 1920 x 1080 physical (125 %). The screen tool's frame is 1456 x 819.
- **Company:** BRIDGE BATCH LAB, 100050, a30f23c3-4819-4334-96ab-ffa6fba70976, loaded alone at P0.
- **BK0:** `TDBK1800_100050.001`, 57,180 B, sha256 a7ad5f9a93396abd12b4f8bf3f3ca1e281999f99c2cd73d806f42e08c6d7db61.
- **Marks:** M 224 and V 87 at B2 and again at C6.

## Steps
Probe lines read position, topmost, foreground, pid, title. "K probe" is the copy from deviation 1.

| step | stamps (UTC) | what was read |
|---|---|---|
| B3 | title clicked 04:56:05.800; probe 04:56:06.486 to 07.665 (P-time 1.18 s) | `3,0,1,13992,TallyPrime:9001`, 12 titled windows. Known answer met; Tally maximised. |
| B3b | forms shown 04:56:09.845; trace 04:56:12.815 to 22.915 | `FIRST` 04:56:12.906, `PROC` powershell.exe (pid 20968). W "Tally probe control TOPMOST" 0x312912 exstyle 0x00050108 `topmost=1`; W "Tally probe control PLAIN" 0xd2271a exstyle 0x00050100 `topmost=0`. 1 T line, 8 H lines, 2,506 B. **Known answer met.** |
| K1 (attempt 1) | T0 04:58:20.099; box asked 04:58:26.301 | trace `FIRST` 04:58:26.419: 0x852cbc `#32770` exstyle 0x00010109 `topmost=1`, owner 0, the foreground. No probe at +8 or +11 (deviation 2). Late probes 05:01:40 to 43: K probe box `1,1,0,27720`, Tally `5,0,0`. The late click went to the trace's window. No by mouse at 05:01:54.860; `answer=7`. |
| K1b | T0 05:02:29.559; box asked 05:02:35.906 | trace `FIRST` 05:02:36.022 |
| K1b-T0+8 | 05:02:37.565 to 39.739 | sheet probe: Tally `4,0,0,13992`. K probe: box `1,1,1,27476,Lab topmost control`, Tally `4,0,0`. |
| K1b-T0+11 | 05:02:40.556 to 42.886; capture 43.168 | the same lines |
| K1b click | 05:02:43.841 (T0+14.29 s) at logical (263,475) = screen tool (249,450), 337 px from the box | |
| K1b C+0 | 05:02:43.986 to 46.509 | sheet probe: Tally `4,0,1`. K probe: box `1,1,0`, Tally `4,0,1`. **The box stays above Tally.** |
| K1b C+6 | 05:02:49.846 to 51.950 | the same |
| K1b No | 05:02:52.357, by mouse at logical (890,484) | `control-k1b.txt`: `answer=7`, answered 05:02:52.371 |
| K2 | | skipped: K1 showed the box |
| B5 | 05:04:03.011 | processes with `--confirm-journal`: 0 |
| C0 | 05:04:09 | `voucher_count` 1, `total_debit` 141, `total_credit` 141; `current_window_readable`, `source_rows` 0 (20260914). BATCH-T `bridge-fc6fbd33-1003-49e0-bf5c-dadd0d2aa211`. |
| C1 | trace started 05:05:19.121; Tally's title clicked 05:05:21.199; T0 05:05:22.264 | one `post_import` (company_guid and batch_id). No window carried the title before the trace. |
| C1 trace | `FIRST` 05:05:29.170 | `PROC`: BUILD-T's path, `--confirm-journal`, parent 6120 (the launch), sha256 3ca00fab… |
| T0+8 | 05:05:30.297 to 32.410 | `3,0,1,19416,ComplyEaze Bridge - post this voucher?` and `4,0,0,13992,TallyPrime:9001`, 14 windows |
| T0+13 | 05:05:35.262 to 36.413; capture 36.673 | the same |
| C2 | 05:05:36.851 | the box's text: row 1780-C2's 26 lines, the batch line naming BATCH-T. None of the lines the halt rule names differs. Which button is the default was not read. |
| C3 | 05:05:38.264 (T0+16.02 s) at logical (263,475) = screen tool (249,450), 286 px from the box | the click did not answer the box |
| C+0 | 05:05:38.297 to 39.765 | `3,0,1,13992,TallyPrime:9001` and `4,0,0,19416,ComplyEaze Bridge - post this voucher?`: **Tally above the box** |
| C+6 | 05:05:44.264 to 45.194 | the same |
| C4 | 05:05:45 | the box was not above Tally (z 4, Tally 3), so nothing was clicked |
| C4 taskbar | 05:06:25.745 to 27.830 | box still open and visible (z 4). Taskbar button clicked once at 05:06:26.054 (logical 756,844 = screen tool 717,800). Probe 05:06:26.292 to 27.766: `3,0,0,13992,TallyPrime:9001`, 13 windows, no box line. At 05:06:27.830 the box is no longer a window. **Gone after the taskbar click without No;** one probe, one capture, nothing more clicked. Yes was never clicked. |
| C5 | answer 05:06:02 (sent 05:05:22, round trip 40.085 s); reads 05:06:55 | see T4 below |
| C6 | 05:07:18 to 05:07:23 | M 224, V 87 (equal to B2); `--confirm-journal` processes 0; items re-hashed equal; `trace-c.txt` ended `GONE`; launch exit 0; WRITES removed (env sha256 57ef555f…); company shut |

## The traces
**trace-c.txt (the approval box, T2 and T3)**
- `FIRST` 05:05:29.170Z, box_pid 19416.
- `PROC`:
  - `<build folder>\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`, parent_pid 6120;
  - command line ends `--confirm-journal`;
  - sha256 3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974, 31,804,416 B. **This is BUILD-T.**
- **The box's window:**
  - hwnd 0x1971534, class `#32770`, thread 18972, `owner=0`;
  - exstyle 0x00010101, `topmost=0`, in every block: 05:05:31.446, 05:05:38.339. **It never changed.**
- **Windows of the process (3, at the first block and at the last block with windows):**
  - the dialog (visible, `topmost=0`);
  - `MSCTFIME UI` (hidden, `topmost=1`);
  - `Default IME` (class `IME`, hidden, `topmost=1`; owner the dialog, `owner_topmost=0`).

  All three are on thread 18972. There is no other visible top-level window.
- **fg_pid:**
  - 19416, the box, at 05:05:31.446;
  - 13992, Tally, at 05:05:38.339, after the C3 click;
  - 11384, explorer, at 05:06:26.171, the taskbar click. In that block the process has 0 windows.
- `GONE` 05:06:29.284Z. 3 T, 55 H and 6 W lines; 4,055 B.

**trace-k1b.txt (the control box, T1)**
- `FIRST` 05:02:36.022Z, box_pid 27476.
- `PROC`: powershell.exe running `control-box.ps1` (sha256 9785001b…).
- **The box's window:**
  - hwnd 0x11f209a, class `#32770`, thread 12752, `owner=0`;
  - exstyle 0x00010109, `topmost=1`, in every block: 05:02:36.787 (0.77 s after `FIRST`), 05:02:43.904. **It never changed.**
- **Windows of the process: 4 at the first block:**
  - the dialog (visible, `topmost=1`);
  - the hidden console;
  - `MSCTFIME UI`;
  - `Default IME`.

  The last block with windows has only the hidden console.
- **fg_pid:**
  - 27476, the box, at 05:02:36.787;
  - 13992, Tally, at 05:02:43.904, after the click;
  - 0 at 05:02:52.419, after No;
  - 13992 at 05:02:52.653.
- `GONE` 05:02:55.680Z.

**trace-k1.txt and trace-k1-late.txt (attempt 1)**
- The same box state as K1b: 0x852cbc, exstyle 0x00010109, `topmost=1`, owner 0.
- It held from 04:58:26.419Z to the No at 05:01:54.860Z, with no change.

The two boxes differ by exactly the TOPMOST bit (0x8): 0x00010101 against 0x00010109.

## T4: after the box was gone (C5, read only)
- **The `post_import` call's own answer**, at 05:06:02 (T0+40 s, while the box was still open):
  - `approval.state` pending, `dialog_remaining_s` 86, `retry_after_s` 15;
  - `attempt_recorded` false;
  - `dispatch.state` not_dispatched, `resent` false;
  - `previous_approval` null;
  - no `next_step`.

  With 86 s left at 05:06:02, the box would have expired at about 05:07:28. It was gone at 05:06:27, so **it did not expire**.
- **`egress_log`**: 10 records, all from this launch. After T0 there are only two lines, both answering the call itself: the `post_import` `response_prepared` (05:06:02.448) and `stdio_write_completed` (05:06:02.508, 1,403 B). **No line sends BATCH-T to Tally.**
- **`local_data_report`**:
  - journal `batches` 1, `no_dispatch_never_verified` 1, `sent_or_found_posted` 0, `not_settled` 0;
  - `import_files` 1, `proofs` 2, `approval_notes` 0.
- **The import journal** (`evidence/import-journal.jsonl`): the built batch and one `verification_status` line (`verification_incomplete`). There is no dispatch, approval or decline line.
- **Not known:**
  - what closed the box at the taskbar click, and whether the child returned any answer to its parent;
  - the journal records no decline. The call was not asked again (the sheet forbids a second `post_import`).

## Reading
- **Control (T1).** A plain `MessageBoxW` with `MB_TOPMOST`, shown by a process started hidden and in the background, has the TOPMOST bit from its first block. It stays above a maximised Tally after a click into Tally. **D1 does not hold.**
- **The approval box (T2).** It has the bit clear in every trace block, the first one 2.28 s after `FIRST`. The trace's `PROC` step hashes the 31.8 MB exe before its first block, so the trace cannot show the first 2.28 s. The earliest read is the T0+8 probe, which started 1.13 s after `FIRST` and read 0. **So a set-then-cleared change in the first 1.1 s (D2, "within about 150 ms of `FIRST`") is not ruled out.** No change was seen from then on, through the click and to the end.
- **D3:** owner 0, so it does not hold.
- **D4:** BUILD-T's hash and `--confirm-journal` are right, and B5 listed no child, so it does not hold.
- **D5:** only one visible top-level window, so it does not hold.
- **Against the sheet's table:**
  - From 2.28 s on, the row "`topmost=0` from `FIRST`, owner 0, the hash right, one visible window, and the control reads 1" fits: **none of the five**.
  - Within the first 1.1 s, D2 stays open.
- **T4:** nothing was dispatched or posted. The box closed after a taskbar click without No being clicked, and did not expire. Its answer to the parent is not recorded anywhere this row could read.
- **Row FAIL conditions, none met:**
  - no click or key answered the approval box;
  - no post;
  - the marks are unchanged;
  - no box or child is left after C6;
  - B3b's known answer was met.

**Not measured:**
- keys, menu clicks and fields in Tally;
- a second monitor; macOS; Windows 11;
- the ledger-create, review-acknowledgement and invoice-stop boxes;
- any change to the build;
- the approval box's first 2.28 s in the trace;
- which button is the default.

## Files
- `answers/`: every call of the launch, whole.
- `client-log.txt`.
- `evidence/`:
  - the timelines `b3-`, `k1-`, `k1b-` and `c-timeline.txt`;
  - the traces `trace-b3b`, `trace-k1`, `trace-k1-late`, `trace-k1b` and `trace-c.txt`, whole except for path placeholders;
  - every probe's output, with `-k` for the K probe;
  - `forms.txt`, `control-k1.txt` and `control-k1b.txt`;
  - `c2-box-text.txt`;
  - `b5-processes.txt`, `c6-processes.txt` and `c6-rehash.txt`;
  - `import-journal.jsonl`.
- `scripts/`: the lab's runners `b3.ps1`, `k-run.ps1`, `c-run.ps1`, `c4-taskbar.ps1` and `procs.ps1`; `zorder-probe-k.ps1` and its diff; and `control-forms.ps1` as run, with the working folder in place of its placeholder.
- `substitutions.txt`: each pushed copy that differs from the lab's file.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256. They stay on the lab PC.
