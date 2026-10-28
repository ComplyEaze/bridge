# Results: #1780 F approval focus, 28 Oct 2026, on the 0.6.1 cut build. PASS on the sheet's terms: the box opened above a maximised Tally with the expected title and first line, and a mouse No declined with nothing sent and the mark unchanged. One finding: the probe read the box's `topmost` as 0, not the expected 1. Two lab setup misses came first, each refused before any box and before anything reached Tally

The sheet is `lab/1780-approval-focus` at a4ab3da8471b3e0795c4b0b190e32cd8520364d1. `SHA256.txt` 4 of 4 equal (hash and bytes).

Every step was done by the lab on the owner's standing word of 27 Oct, the No click included. Posting was switched on for the launch by the lab on the owner's word of 28 Oct, and switched off again after the row.

## Setup
- **Company:** BRIDGE BATCH LAB, 100050, GUID a30f23c3-4819-4334-96ab-ffa6fba70976, `verified_tuple`.
  - It was in Tally's Select Company list (P0) and was loaded alone.
  - Current Period 1-Apr-26 to 31-Mar-27; current date Friday, 21-Aug-2026.
- **BK0:** `TDBK1800_100050.001`, 57,180 B, sha256 14e6888088b3a4fb678fbf7aaf06094e7f71217676fff9f5f9b331866b10c7d4.
- **Items, the same at start and end:**
  - CUT `bridge_mcp.exe` 6f520338...728d. The server reported `bridge-tally` 0.6.0 in its initialize answer.
  - `build-1591-s1.json` 5f8a5ff2...f13a.
  - `zorder-probe.ps1` 3f03768e...0fcb.
- **B2 mark** (`ledger_masters` basic, `reused` false): V 87, M 224. `tally_status`: TallyPrime 7.2, Silver, not education mode.
- **B3 probe known-answer:** `3,0,1,13992,TallyPrime:9001` / `visible_titled_windows,10`. Tally's pid with foreground 1: passed.
- **Host:**
  - Windows 10 build 19045.6466; one monitor, 1536 x 864 logical at 120 DPI (125 %).
  - Tally maximised (IsZoomed true), TallyPrime 7.2 Silver Rental.
  - desktop client 3.6556.2.

## Lab setup misses (recorded; nothing reached Tally, no box opened)
1. **First launch without posting:** `build_import_xml` (003) was refused with `import_unverified_on_live_tally`. The launch had not set `BRIDGE_AGENT_ENABLE_WRITES`, although the sheet says posting is on for the CUT.
   - The lab stopped that launch (exit 0) and set the variable to true. A second launch followed (exit 0, error stream empty).
   - F0 then ran as 004.
2. **post_import without the company GUID:** the first `post_import` (005, T0 13:48:27.852Z) carried only `batch_id`. It was refused at once with `company_guid_required`.
   - The T0+5 s and T0+15 s probes showed Tally alone at position 3 with foreground 1; the files are kept as `*-attempt1`.
   - F1 was then run again with `company_guid` and `batch_id` (006).

## Table
| step | stamps (UTC) | probe lines (position, topmost, foreground, pid, title) | read / result |
|---|---|---|---|
| F0 | 13:47:44Z (004) | none | `voucher_count` 1; `total_debit` 141, `total_credit` 141; `verification_preflight` state `current_window_readable`, `source_rows` 0 for 20260914; BATCH-F `bridge-b5e511c7-7899-4366-84ef-fdfd211fc647` |
| F1 | T0 13:50:05.505Z; Tally title clicked before T0 and again by 13:50:08.032Z (inside 5 s; no box yet) | T0+5 s (13:50:10.659Z): `3,0,1,13992,TallyPrime:9001`, no box line, 10 windows. T0+15 s (13:50:20.592Z): `3,0,1,7416,ComplyEaze Bridge — post this voucher?` above `4,0,0,13992,TallyPrime:9001`, 11 windows | Box above Tally: **yes**, by eye and by position (3 above 4). It opened between T0+5 s and T0+15 s. Its `topmost` is **0**, where the sheet expected 1 (FINDING) |
| F2 | from the T0+15 s capture (no box at T0+5 s) | none | **Title:** `ComplyEaze Bridge — post this voucher?` (exact). **First line**, with its breaks read as spaces: `Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.` (exact). **Voucher lines:** see the note below this table. **Buttons:** Yes / No / Cancel, No drawn with the default outline |
| F3 | T0+15 s probe | box foreground 1; Tally foreground 0 | The box's title bar is drawn active. Nothing typed |
| F4 | No clicked by mouse at about 13:50:43.1Z (the call's evidence closed 13:50:43.139Z; the stamp taken just after the click 13:50:43.686Z); the answer was saved 13:50:43Z (round trip 37.361 s from T0) | none | `import_approval_declined`, `attempt_recorded` false, `dispatch_response` null: nothing dispatched. Not MODE BLOCKED (the click landed while the call waited, before 40 s) |
| F5 | none | none | Review box not raised: no post in this row |
| F6 | 13:51:05.999Z | `3,0,1,13992,TallyPrime:9001`, 10 windows: no box left | Mark V 87, M 224 (007), equal to B2. No `bridge_mcp.exe` with `--confirm-journal`: the only `bridge_mcp.exe` processes are the desktop app's own connectors and this row's launch, which then exited 0. Three items re-hashed equal |

**F2 voucher lines.** The box read: Create ONE Receipt in "BRIDGE BATCH LAB"; Company GUID a30f23c3-...0976; Company number 100050; Books from 20260401; Tally http://127.0.0.1:9001; Date 20260914; Voucher number: Tally assigns it; Dr 141.00 "Synthetic Bank Ledger"; Cr 141.00 "Party A"; Total debit 141, Total credit 141; Batch bridge-b5e511c7-...; Reference (none); Narration "Lab topmost probe 1". These match the build file. The date shows as 20260914 rather than "14 Sep 2026", a format difference only, so the lab did not halt the row.

- **Box above Tally:** yes.
- **Foreground:** the box.
- **Result code:** `import_approval_declined`.
- **Voucher mark:** V 87, M 224 before and after.
- **End state:** BATCH LAB shut from the Gateway's Company menu; Select Company with none loaded. One tally.exe (13992) owns 9001; `GET /status` 200 (51 B).

**PASS / FAIL:** PASS on F1 (above Tally), F2 (title and first line exact) and F4 (declined, mark unchanged, no orphan). F3 is recorded: box foreground 1. The topmost 0 reading is a finding against the probe EXPECT; it does not fail the PASS terms.

**Not measured:**
- keys typed while the box is open (16 Oct, unchanged by design);
- a second monitor; macOS; Windows 11;
- the review acknowledgement box (needs a post);
- a Tally entry screen open under the box.
