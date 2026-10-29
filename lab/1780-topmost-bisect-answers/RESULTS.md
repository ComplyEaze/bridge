# Results: 1780-E, the topmost bisect on #2107's build, 29 Oct 2026. Every control variant (K1c to K1f) and the dialog child run by hand (H1) read `topmost` 1 and stay above Tally after a click into it. Only the box spawned by the running server (C1) reads 0. **E1 to E5 do not hold: the reading is E6, the parent's spawn context.** With no taskbar click, C1's box stayed open to P-T+70. The single taskbar click then ended it within 78 ms, so **the taskbar click, not the parent's timeout, ends the box**. Nothing was posted; the marks did not move

The sheet is `lab/1780-topmost-bisect` at 1542ab88e9b6647f2fbb299966d1fff34556f531. `SHA256.txt` matched 9 of 9.

**Who acted.** The lab did every click into Tally, the taskbar click and every No, on the owner's standing word of 27 Oct. Posting was on for part C only, on the owner's word of 28 Oct.

**Deviations (each also in `notes.txt`):**
1. **The runners.** The K and C runners are row 1780-D's, edited as the sheet allows; each edit is listed in the runner's header.
2. **H1 stopped on the lab's own count.** It stopped at H3, counting the box text as 24 lines because the text ends in a line break. Compared line by line, the box text equals `confirm-input.txt`'s 23 lines, so no real stop condition was met.
   - The child stayed running with its box untouched.
   - `h-late.ps1` then attached to it by pid, keeping its handle so the exit code could be read, and started a new trace. It did H4 to H6.
   - So H4's click came at H-T0+121 s, not +16 s.
3. **Path placeholders.** Pushed copies have lab-PC paths replaced; `substitutions.txt` lists each one.

## Build and setup
- **BUILD-T:**
  - #2107, head a66c31f2ab607b36701ea0474ca15d474b9e202c;
  - CI run 40709511734, artifact 12826362269;
  - mcpb 15,320,736 B (61e52d07…), exe 31,804,416 B (3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974).

  The same copy as rows 1780-C2 and 1780-D; nothing was downloaded. Every item hashed equal at B2 and at C6.
- **System:** TallyPrime 7.2 Silver; Windows 10.0.19045 22H2; desktop client 3.6556.2; one monitor, 1536 x 864 logical, 1920 x 1080 physical (125 %). The screen tool's frame is 1456 x 819.
- **Company:** BRIDGE BATCH LAB, 100050, a30f23c3-4819-4334-96ab-ffa6fba70976, loaded alone.
- **BK0:** `TDBK1800_100050.001`, 57,180 B, sha256 55346ac8f8da4e0995bb82184b6a9766d5a52eaf8475a47451e6f7f70b5a98fc.
- **Marks:** M 224 and V 87 at B2 and at C6.

## Steps
"Box" is the box window's own line; "probe" lines read position, `topmost`, foreground. Every click point is logical (263,475), which is screen tool (249,450).

| step | stamps (UTC) | box window (trace) | after the click into Tally | ending |
|---|---|---|---|---|
| B3 | 05:43:22.695 | | probe `3,0,1,13992,TallyPrime:9001` | known answer met |
| B3b | 05:43:24.884 to 47 | TOPMOST 0x00050108 `topmost=1`; PLAIN 0x00050100 `topmost=0` | | **known answer met** |
| B5 | 05:43:48.658 | confirm-journal processes 0 | | |
| K1c (the flags) | T0 05:44:46.091; `FIRST` 05:44:54.171 | `#32770`, owner 0, 0x00010109, `topmost=1` in every block | click T0+14.75 s, 310 px: box `1,1,0` above Tally `4,0,1` at C+0 and C+6 | No 05:45:09.439, `answer=7` |
| K1d (+ DPI) | T0 05:45:23.287; `FIRST` 05:45:30.145 | 0x00010109, `topmost=1` throughout. `dpi_before_awareness=0`, `dpi_set=True`, `thread_is_per_monitor_v2=True` | T0+15.5 s, 294 px: box 1, Tally 4 | No 05:45:47.308, `answer=7` |
| K1e (+ hook) | `FIRST` 05:46:06.683 | 0x00010109, `topmost=1` throughout. `hook_installed=True`; `.fits.txt`: 2 activations, both `fits=True` | T0+14.3 s, 294 px: box 1, Tally 4 | No 05:46:22.687, `answer=7` |
| K1f (+ raise) | `FIRST` 05:46:45.123 | 0x00010109, `topmost=1` throughout. `front: raise done, already the foreground window`; 2 activations, `fits=True` | T0+16.26 s, 295 px: box 1, Tally 4 | No 05:47:05.289, `answer=7` |
| H1 (by hand) | H-T0 05:48:26.156; `FIRST` 05:48:27.276 | PROC: BUILD-T's exe, `--confirm-journal`. `#32770` 0x1322c1e, owner 0, 0x00010109, `topmost=1` in every block of both traces. H-T0+8 and +13: `1,1,1,16104` | click 05:50:27.312 (deviation 2), 286 px: box `1,1,0` above Tally `4,0,1` at C+0 and C+6 | No 05:50:45.009: exit code 1, stdout 0 B. stderr: two lines, `box-fit: box=(686,181,1254,926) work=(0,0,1920,1030) fits=true` |
| C0 | 05:51:33 | `voucher_count` 1, 141/141, `current_window_readable`, `source_rows` 0; BATCH-T `bridge-faf5d197-c8c9-4a12-8cc7-9c13808c2e5e` | | |
| C1 (the server) | T0 05:52:38.515; `FIRST` 05:52:45.408 | PROC: BUILD-T's exe, `--confirm-journal`, parent 12212 (the launch). `#32770` 0x1811bf6, owner 0, **0x00010101, `topmost=0` in every block** (first 1.70 s after `FIRST`). T0+8 and +13: `3,0,1,20648` | T0+16.03 s, 286 px: Tally `3,0,1` **above** the box `4,0,0` at C+0 and C+6 | |
| C2 | 05:52:53.237 | text as row 1780-D's except the batch line, which names BATCH-T | | |
| C4 | P-T 05:53:19.276 (T0+40.46 s) | the call answered pending: `dialog_remaining_s` 86, `retry_after_s` 15, `attempt_recorded` false, `not_dispatched`, `previous_approval` null | P-T+30, +50 and +70: Tally `3,0,1`, box `4,0,0`, **still a window each time** | taskbar click 05:54:30.157 (logical 756,844 = screen tool 717,800). The trace has the process at 0 windows at 05:54:30.235 (**78 ms**); probe: no box. Nothing more clicked |
| C5 | 05:54:4x | `egress_log`: after T0 only the call's own two lines. `local_data_report`: 1 batch, `no_dispatch_never_verified` 1, `sent_or_found_posted` 0. Journal: built plus `verification_status`, no dispatch, approval or decline | | |
| C6 | 05:54:53 to 05:55:12 | M 224, V 87; confirm-journal processes 0; re-hash equal; launch exit 0; WRITES removed (env 9fd0a03b…); company shut | | |

**`fg_pid` changes:**
- each K box: the box at first, Tally after the click, 0 at the No, then Tally;
- H1: the box, then Tally (late trace), 0 at the No, then Tally;
- C1: the box (20648), Tally from 05:52:54.541, explorer (11384) at the taskbar click.

**Windows of each box's process:** the dialog (visible) plus the hidden IME windows, all on the dialog's thread. K boxes also have the hidden console. No process had a second visible top-level window.

## Reading
- **E1 to E4 do not hold.** K1c to K1f each read `topmost` 1 in every block and stayed above Tally after the click:
  - E1, the flags (`MB_YESNOCANCEL | MB_DEFBUTTON2 | MB_ICONWARNING | MB_SETFOREGROUND | MB_TOPMOST`);
  - E2, the DPI call, which took effect here (`dpi_before_awareness` 0, `dpi_set=True`);
  - E3, the hook;
  - E4, the raise thread.
- **E5 does not hold.** The real dialog child, run by hand from BUILD-T's exe with the box's 23-line text, reads 0x00010109 (`topmost=1`) and stays above Tally.
- **E6 fits.** The same exe and text, spawned by the running server, reads 0x00010101 from its first block, and Tally covers it after the click. The bit is lost only when the server's parent context spawns the child.
- **T4.** With nothing clicked, C1's box was still a window at P-T+70 (about 111 s after `FIRST`). The single taskbar click ended its process 78 ms later. The parent's 120 s bound was not reached, so **the taskbar click caused the end in rows 1780-C2 and 1780-D too**.
  - **Is it the fit check (`on_activate`)? Not established.** The parent discards the child's stderr, and H1 never needed a taskbar click (its box stayed on top). Every activation seen on stderr (H1) and in the controls' `.fits.txt` measured `fits=true`/`True`.
- **FAIL of the row: none.**
  - No click or key answered a box with Yes or by accident; every No was by mouse, and the C1 box ended at the taskbar click.
  - No post; marks unchanged.
  - No box or hand-run child left.
  - B3b's known answer met.

**Not measured:**
- keys, menu clicks and fields in Tally;
- a second monitor; macOS; Windows 11;
- the ledger-create, review-acknowledgement and invoice-stop boxes;
- any change to the build;
- the C1 child's own stderr and exit code (discarded by the parent);
- the control boxes measure their own size, not the approval box's.

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `answers/`: every call of the launch, whole.
- `client-log.txt`.
- `evidence/`:
  - every runner's timeline and console;
  - every trace, whole except for path placeholders (`trace-b3b`, `trace-k1c` to `trace-k1f`, `trace-h1`, `trace-h1-late`, `trace-c`);
  - every probe (`-k` for the K probe);
  - `control-k1x.txt` and the `.fits.txt` files;
  - `h1-stdout.txt` and `h1-stderr.txt`, whole;
  - `h1-box-text.txt` and `c2-box-text.txt`;
  - `forms.txt`;
  - the process lists and hashes;
  - `import-journal.jsonl`.
- `scripts/`:
  - the runners `b3.ps1`, `k-run.ps1`, `h-run.ps1`, `h-late.ps1` (and `make-h-late.py`, which made it), `c-run.ps1` and `procs.ps1`;
  - `control-forms.ps1` as run.
- `substitutions.txt`, `notes.txt` (UTC), and `SCREENSHOTS.txt` (the captures by sha256; they stay on the lab PC).
