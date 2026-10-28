# Results: #1780 C click into Tally, 28 Oct 2026, on the 0.6.1 cut build. Outcome B: after one click on a blank part of a maximised Tally, Tally covered the approval box at C+0 and C+5. Tally took the foreground and the box fell below it; the box's `topmost` read 0 throughout. The box was brought back through its taskbar button and answered No by mouse. Nothing was sent: the marks were unchanged and no confirm child remained. The box opened between T0+5 s and T0+10 s

The sheet is `lab/1780-approval-box-click-into-tally` at f16bbddbc79de97ff32b1ff1288e64c45d62b433. `SHA256.txt` 4 of 4 equal (hash and bytes).

Every step was done by the lab on the owner's standing word of 27 Oct, including the click into Tally, the taskbar click and the No. Posting was switched on for the launch and off after it on the owner's word of 28 Oct.

## Setup
- **P0:** BRIDGE BATCH LAB is in Tally's Select Company list. It was loaded alone: Current Period 1-Apr-26 to 31-Mar-27, current date Friday, 21-Aug-2026.
- **P1 launch file, posting on:** keys BRIDGE_AGENT_DATA_DIR, BRIDGE_AGENT_ENABLE_WRITES, BRIDGE_TALLY_HOST, BRIDGE_TALLY_PORT, BRIDGE_TERMS_ACCEPTED; sha256 74aafdfdac76faa505ad78ef9a0fe83b40fa334183b27138eaa510bbe780815e.
- **P1 launch file, after C5:** BRIDGE_AGENT_ENABLE_WRITES removed; sha256 c1a33ea7bcd7967a619bedd0408546eef9eb38f7ec6283f3753a26ece988b170.
- **BK0:** `TDBK1800_100050.001`, 57,180 B, sha256 1a9840ee469f87e0319e41922fc0a095cadf71094631a4d26423ba9eb1b19936.
- **Items, equal at start and end:**
  - CUT `bridge_mcp.exe` 6f520338...728d; the server reports 0.6.0;
  - `build-1591-s1.json` 5f8a5ff2...f13a;
  - `zorder-probe.ps1` 3f03768e...0fcb.
- **B2:** `tally_status` TallyPrime 7.2, education mode false, BRIDGE BATCH LAB the one company; `list_companies` the same, `verified_tuple`; `ledger_masters` basic V 87, M 224, `reused` false.
- **B3:** probe `3,0,1,13992,TallyPrime:9001` / `visible_titled_windows,10`, Tally foreground 1 (met). P-time 1.823 s.
- **Host:**
  - Tally maximised (IsZoomed true); one monitor, 1536 x 864 logical at 120 DPI (125 %);
  - Windows 10 build 19045.6466; TallyPrime 7.2 Silver Rental;
  - desktop client 3.6556.2 (as recorded in row 1780-F this sitting).

## Table
| step | stamps (UTC) | probe lines (position, topmost, foreground, pid, title) | reading |
|---|---|---|---|
| C0 | 14:27Z (003) | none | `voucher_count` 1, debit and credit 141; preflight `current_window_readable`, `source_rows` 0 for 20260914; BATCH-C `bridge-193013cd-9e92-4c55-841c-b10d058a5ccd` |
| C1 | T0 14:27:41.800Z (call with `company_guid` and `batch_id`); Tally's title area clicked just before T0; no click into Tally after T0 until C3 | T0+1 (42.802 to 44.194): Tally 3,0,1, no box. T0+2 (44.239 to 45.601): Tally 3,0,1, no box. T0+5 (47.048 to 48.527): Tally 3,0,1, no box. T0+10 (51.765 to 53.273): box `3,0,1,25404` above Tally `4,0,0,13992`. T0+15 (57.199 to 58.826): box 3,0,1; Tally 4,0,0 | **Open delay: between T0+5 s (probe end T0+6.7 s) and T0+10 s (probe start T0+10.0 s).** T0+2 was not skipped (it started 0.04 s after T0+1 ended). Captures at T0+5 (46.792) and T0+15 (56.788) |
| C2 | T0+15 probe and capture | box 3,0,1; Tally 4,0,0 | Box above Tally, foreground on the box, box `topmost` 0. **Title:** `ComplyEaze Bridge — post this voucher?` (exact). **First line**, breaks read as spaces: `Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.` (exact). No drawn as the default. **Lines under it:** see the note below this table |
| C3 | click at (250, 450) in the screen tool's 1456 x 819 frame, about 276 px left of the box's left edge, on the blank left panel of the Gateway; the click completed by 14:28:08.239Z (about T0+26 s, later than the planned T0+17 s: the lab's screenshot before the click took longer) | C+0 (14:28:08.449 to 09.801): Tally `3,0,1,13992`, box `4,0,0,25404`. C+5 (14:28:13.313 to 14.406): the same | **Outcome B.** At C+0 and C+5 Tally is above the box (3 over 4). Foreground: Tally 1, box 0. Box `topmost` 0. Tally's title bar is drawn active. The click did not answer the box: it stayed open, and the call had not answered. Captures C+0 (09.820) and C+5 (14.426) |
| C4 | the call answered `pending` at 14:28:22.214Z (T0+40.4 s, round trip 40.085 s): approval state pending, `dialog_remaining_s` 86, `retry_after_s` 15, `dispatch.state` not_dispatched, `attempt_recorded` false. Then the box's taskbar button was clicked once (the box came back above Tally), and No was clicked by mouse at about 14:28:34.9Z (T0+53 s; the stamp just after it 14:28:35.217Z), later than the sheet's "by T0+35 s" | none | **MODE BLOCKED:** the call had answered `pending` before No, so no decline code is returned to this call. The marks decide: unchanged |
| C5 | 14:28:55Z | `3,0,0,13992,TallyPrime:9001`, 10 windows: no box | `ledger_masters` basic (005) V 87, M 224, `reused` false: equal to B2. No `bridge_mcp.exe` with `--confirm-journal`: only the desktop client's four connectors and this launch, which then exited 0. Three items re-hashed equal. Launch stopped 14:29:11Z (exit 0, error stream empty); WRITES removed; BRIDGE BATCH LAB shut; Select Company with none loaded; one tally.exe 13992 owns 9001; `GET /status` 200 (51 B) |

**C2, the lines under the first line**, read from the T0+15 s capture and the box as shown, compared line by line with the sheet's list:
- **Equal**, with `<BATCH-C>` = bridge-193013cd-...:
  - Create ONE Receipt in "BRIDGE BATCH LAB";
  - Company GUID a30f23c3-...0976;
  - Company number 100050, Books from 20260401;
  - Tally http://127.0.0.1:9001;
  - Date 20260914, Voucher number: Tally assigns it (the voucher's own text: last two lines);
  - Dr 141.00 "Synthetic Bank Ledger";
  - Cr 141.00 "Party A";
  - Total debit 141, Total credit 141;
  - Checked in Tally ...;
  - Batch;
  - Ledgers checked by identity ...;
  - Do not post a file already imported manually;
  - Pause other edits/imports ...;
  - After a timeout, reconcile this batch ...;
  - the dashed line;
  - Reference: (none);
  - Narration: "Lab topmost probe 1".
- **One extra line of the build's own wording**, between "After a timeout ..." and the dashed line: "ComplyEaze Bridge posts now or if asked again within 15 min, unless cancelled, refused or restarted." Recorded word for word; it does not halt the row.

## Outcome
- **Outcome B:** after one click into Tally, Tally covers the box until it is brought back through its taskbar button.
- **Not a FAIL of the row:**
  - the click did not answer the box;
  - nothing was posted;
  - the marks did not change;
  - no box was left open and no orphan child remained.
- **Timing:** the click came at about T0+26 s, not T0+17 s, and the No at about T0+53 s, not by T0+35 s. The call's own `pending` answer at T0+40 s came before the No, hence MODE BLOCKED; the box was still open (`dialog_remaining_s` 86 at T0+40 s) and closed on the No.

**Not measured:**
- keys typed while the box is open;
- a click on a Tally menu item, button or field;
- a Tally entry screen under the box;
- a second monitor; macOS; Windows 11;
- the review acknowledgement box;
- why `topmost` reads 0.
