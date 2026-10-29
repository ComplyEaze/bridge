# Results: 1780-C2, 29 Oct 2026, on #2107's build at a66c31f2 (exe 3ca00fab…): the fix does not hold. The box's `topmost` still reads 0 once it is shown. After one click into a maximised Tally, Tally covered the box at once and 6 s later (Outcome B again). The one Down key went to Tally, not the box. The box did not take the keyboard. **No was not clicked:** the box closed when its taskbar button was clicked to bring it back, at about T0+60 s, after the call had already answered `pending`. Its answer is not established, so no further `post_import` was made and the launch was stopped. Nothing was posted: the marks are unchanged and no child is left

The sheet is `lab/1780-approval-box-topmost-rerun` at aa888c74751b25a9d81e673fad3cabdb0f18ada7 (`SHA256.txt` 4 of 4, hash and bytes).

**Who acted.** Every step was done by the lab on the owner's standing word of 27 Oct: the click into Tally, the Down key, the taskbar click. The owner was at the PC for the post. Posting was on for this launch and off after it, on the owner's word of 28 Oct.

**Build.** BUILD-T is #2107 at a66c31f2ab607b36701ea0474ca15d474b9e202c: CI run 40709511734, artifact 12826362269.
- `bridge-tally.mcpb` 15,320,736 B, 61e52d079544b2c7dd49cb47bf08a11120b0154957e830e8a76e598970ab99c2.
- `bridge_mcp.exe` 31,804,416 B, 3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974.
- `build-1591-s1.json` 5f8a5ff2… and `zorder-probe.ps1` 3f03768e….
- All equal before and at the end. The server reports 0.6.1.

**Host.**
- Windows 10.0.19045 (22H2).
- One monitor, 1536 x 864 logical at 125 %; the screen tool's frame is 1456 x 819.
- TallyPrime 7.2 Silver, maximised.
- Desktop client 3.6556.2, read afresh.

**Company.** BRIDGE BATCH LAB, 100050, a30f23c3-4819-4334-96ab-ffa6fba70976, loaded alone. Current date Friday 21-Aug-2026.

**Setup steps.**
- **P1:** the launch file keys were BRIDGE_AGENT_DATA_DIR, BRIDGE_AGENT_ENABLE_WRITES, BRIDGE_TALLY_HOST, BRIDGE_TALLY_PORT and BRIDGE_TERMS_ACCEPTED (sha256 65f2e23a…). After removing the write setting it is 1587513b….
- **B1, BK0:** TDBK1800_100050.001, 57,180 B, sha256 524f4300a472fa70fd6986fcaa16c43e4c01aa759b499e7c25d2a080e237b008.
- **B2:** `tally_status` and `list_companies` show the one company, `verified_tuple`. `ledger_masters` basic: V 87, M 224.
- **B3:** with Tally's title clicked, the probe ran from 03:04:01.851 to 03:04:03.478Z (P-time 1.63 s) and gave `3,0,1,13992,TallyPrime:9001` with 13 titled windows. Known answer met.
- **C0:** `build_import_xml` gave 1 voucher, total_debit 141 and total_credit 141, preflight `current_window_readable` with `source_rows` 0 for 20260914. BATCH-T is bridge-2c93891a-043c-4bfe-a04b-bc239e190e97.

Probe lines read: position from the top, `topmost`, `foreground`, pid, title.

| step | stamp (UTC) | observed |
|---|---|---|
| C1 | T0 04:18:27.745 | Tally's title clicked; `post_import` (company_guid, BATCH-T) queued |
| C1 | T0+3 (04:18:30.739 to 31.479) | `3,0,1,13992,TallyPrime:9001` only: no box yet |
| C1 | T0+8 (35.744 to 36.604) | box `3,0,1,25028,ComplyEaze Bridge — post this voucher?` above Tally `4,0,0,13992` |
| C1/C2 | T0+13 (40.751 to 41.476), capture 41.503 | the same: box position 3, **`topmost` 0**, foreground 1; Tally 4, 0, 0. Box rect (logical) 549,145 to 1003,741; buttons Yes, No, Cancel. **Open delay: between T0+3 and T0+8 s** |
| C2 | the box's text | title as the sheet; first line "Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer." Every line naming the company, GUID, number, date (`Books from: 20260401`, `Date: 20260914`), the entries, totals, batch, reference and narration is as row 1780-C's sheet lists it. Two differences outside those: a blank line between the Cr entry and the totals, and one added line "ComplyEaze Bridge posts now or if asked again within 15 min, unless cancelled, refused or restarted." |
| C3 | 04:18:43.772 (T0+16.05 s) | ONE left click at logical (263, 475) = screen tool (249, 450), the blank left panel of the Gateway, **286 logical px** from the box's nearest edge |
| C3 | C+0 (43.781 to 44.556), capture | Tally `3,0,1` **above** the box `4,0,0`: the box covered, Tally in the foreground |
| C3 | C+6 (49.776 to 50.507), capture | the same: Tally `3,0,1`, box `4,0,0`; box `topmost` 0 |
| C3b | 04:18:50.640 | the box open and unanswered, Tally in the foreground: **Down pressed once**. The Gateway highlight moved Create → Alter (capture): **the key went to Tally**, not the box |
| C4 | 04:19:08 | the call answered **`pending`** (40.07 s; `retry_after_s` 15, `dialog_remaining_s` 86, `attempt_recorded` false, `not_dispatched`): **MODE BLOCKED**; the box was behind Tally |
| C4 | by 04:19:28.141 | the box's taskbar button clicked once to bring it back (screen tool (767, 798)) |
| C4 | 04:19:28.303 to 30.155 | probe: `3,0,0,13992,TallyPrime:9001`, 13 windows: **no box**. The box's process had ended. **No was not clicked** |
| C5 | 04:19Z to 04:20Z | `ledger_masters` basic: V 87, M 224 (equal to B2). No `bridge_mcp.exe` with `--confirm-journal`. Import journal: the built batch and one verification line, no dispatch. No further `post_import` (the box's answer is unknown, and a call would act on it). Three items re-hashed equal. Launch stopped 04:20:37Z (exit 0, error stream 0 B); WRITES removed. Company shut; one tally.exe owns 9001; `/status` 200 |

## Reading
- **T1:** `topmost` is still **0** once the box is shown. The fix is not in effect on this build.
- **T2:** after one click into Tally, Tally is above the box at C+0 and C+6, with the foreground on Tally. This is Outcome B, as in row 1780-C.
- **T3:** the Down key reached Tally. The box did not take the keyboard.
- **T4 not shown.** The box closed after the taskbar click without a No. How it closed, and what it answered, is **not established**: nothing on screen or in the journal records it.

**Row checks.**
- Nothing was posted and the marks did not change.
- No box is open and no child is left.
- Neither the click nor the key answered the box: it was still open after both.

**Confidence:** one sitting, PARTIAL until repeated.

**Not measured:** as the sheet lists:
- any key other than one Down arrow;
- a click on a Tally menu item, button or field;
- an entry screen;
- the ledger-create box;
- a second monitor; macOS; Windows 11;
- the review and invoice-stop boxes.

Also the No path, here.

## Files
- `answers/`: every tool answer whole.
- `client-log.txt`.
- `notes.txt` (UTC).
- `evidence/`:
  - the timeline;
  - every probe's output;
  - the box's text as read from the window;
  - B3, B4, P1, C5's process list and checks.
- `substitutions.txt`: the lab folder in one pushed copy.
- `SCREENSHOTS.txt`: name, size and sha256 of every capture. They stay on the lab PC (whole-screen captures show other windows).
