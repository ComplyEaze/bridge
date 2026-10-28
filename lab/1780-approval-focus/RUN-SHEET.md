# Windows item 1780-F v1: the approval box above a full-screen Tally on the 0.6.1 build, and what its first line now says (Part of #1780)

WRITES to the book: NONE. Every box is answered **No, by mouse**. No key is typed while a box is open, and nothing is keyed in Tally.

## Why
- #1780: on 0.6.0 the Windows approval box opened above a full-screen Tally and took the keyboard, so `y` approved and `n` declined (issue comments of 16 Oct).
- The maintainer's decision of 21 Oct: **keep the keyboard, fix the words**. The box keeps `MB_SETFOREGROUND | MB_TOPMOST`. Its first line says what Y and N do (#1832, bc5241b21). The posting documentation says not to type in Tally while it is open (#1784).
- Since then, a box-fit guard wraps every box (#1879, #1961, #1962, #1963). Its Windows rows ran from `cmd` with no Tally in front (lab/1879-box-fit-answers, lab/1832-window-size-answers, lab/1975-release-box-row-answers). Those rows noted that activation came late once (RESULTS.md point 3), and that an earlier build kept the box behind other windows (RESULTS-62bf7126.md).
- **What this row decides**, on the 0.6.1 build, through the real `post_import` path with a full-screen Tally in front:
  - F1. Does the box still open ABOVE Tally?
  - F2. Does its FIRST LINE say what Y and N do, in the exact words?
  - F3. Does it hold the foreground? By design it now does. The first line is the mitigation.
  - F4. Does a mouse No still decline, with nothing sent?
- **Not re-measured:** where typed keys go. That was measured on 16 Oct and is unchanged by design (same flags: the Windows `show_review` in `src-tauri/src/tally/approved_import.rs` at 67f647e1a, line 2284). A key would answer the box, so none is typed.

## Who acts
- On this synthetic company, the lab does every step itself, including clicking No in the box. This is the owner's standing word of 27 Oct.
- The lab records each such step as "done by the lab on the owner's standing word of 27 Oct".
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first call and at the end; a different hash = STOP)
| item | source | bytes | sha256 |
|---|---|---|---|
| `bridge_mcp.exe`, the CUT (0.6.1) | 67f647e1a292e8d7815053b5c08289cba6402795, the 0.6.1 cut build already staged on the lab PC | as staged | 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d |
| `build-1591-s1.json` (one Receipt, 14 Sep 2026, 141.00, "Lab topmost probe 1") | `requests/` in this folder, a byte copy of lab/060s-run-sheet @ 8dceb8b72db528ba433d8e911e945fc98db184a7 | 308 | 5f8a5ff24d416331ec2706774d178ba615759abafa87b7c408c279eca88bf13a |
| `zorder-probe.ps1` (read-only list of the top-level windows from the top down: position, topmost, foreground, pid, title; only titles containing "ComplyEaze Bridge" or "Tally") | `requests/` in this folder, a byte copy from the same commit | 2405 | 3f03768e83c4f567957c52b8fda3f6f67c0f1d12c7eceeeb9cbc11a0dd4f0fcb |

- The CUT contains #1832 bc5241b21 (`git merge-base --is-ancestor` true), #1784, #1879, #1961, #1962 and #1963.
- THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976` (the build file's own guid). Step P0 checks it is there.
- Posting is ON for the CUT build.
- Working folder: `C:\bridge-lab\1780f\`. Evidence goes in `C:\bridge-lab\1780f\evidence\`.
- `SHA256.txt` in this folder lists every file of the folder with its size; check it before step P0. A different hash or size = STOP.

## Before the sitting
- P0. (preflight, before anything else) Open the list of companies on the lab PC (Tally's own Select Company list, or `list_companies` once the build is running).
  - If BRIDGE BATCH LAB is not in the list: STOP and report. Do not restore it from a backup and do not use another book in its place.
  - If it is there but is not the one company loaded: load it alone, then go on.
- B1. Backup of THE COMPANY (BK0), named, with size and sha256.
- B2. Hash the three items above. Run `tally_status` and `list_companies`: EXPECT the CUT build and THE COMPANY. Record the voucher mark. Every later compare is against this mark.
- B3. Probe known-answer check: the same as step S4 of `WINDOWS-060S.md` on lab/060s-run-sheet, from the lab's own shell as a child with NO window.
  - Click Tally's title area first; nothing is keyed.
  - EXPECT a line with Tally's pid and `foreground` 1, and a final `visible_titled_windows` line.
  - On failure, F1/F3 rely on screenshots alone. Record "probe NOT MEASURED"; this is not a STOP.

## Steps (one call each; a call in flight is never cancelled or repeated)
- F0. `build_import_xml` with the exact content of build-1591-s1.json.
  - EXPECT `result.voucher_count` 1, and `total_debit` and `total_credit` each 141.
  - EXPECT `verification_preflight.state` `current_window_readable`, with `source_rows` 0 for 20260914.
  - Keep `result.batch_id` as BATCH-F.
  - `source_rows` other than 0, or any refusal = STOP. Nothing has been sent to Tally.
- F1. Tally MAXIMISED (full screen) on the Gateway of Tally and in front; click its title area only.
  - Then `post_import` with BATCH-F. Stamp T0 (UTC, ms) at the call.
  - Within 5 s, click Tally's title area once more. No box is up yet, so this cannot touch a button.
  - At T0+5 s and T0+15 s: a whole-screen screenshot and one probe run, each stamped.
  - EXPECT: the box visible ABOVE Tally by eye. In the probe, the `ComplyEaze Bridge` line has `topmost` 1 and a smaller position than every Tally line.
  - Box behind Tally, or no box by T0+15 s: record and go to F4. This is a FINDING, not a STOP.
- F2. Read the box from the 5-second screenshot.
  - EXPECT title `ComplyEaze Bridge — post this voucher?`.
  - EXPECT the first text line(s), with line breaks read as spaces, to be exactly: `Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.`
  - EXPECT the voucher lines after it: Receipt, 14 Sep 2026, 141.00, Synthetic Bank Ledger Dr, Party A Cr, "Lab topmost probe 1".
  - EXPECT three buttons Yes / No / Cancel, with No drawn as the default.
  - Any difference in the voucher lines = No by mouse, then STOP. A difference only in the first line is recorded word for word as a FINDING.
- F3. Foreground, from the 15-second probe: record the `foreground` value of the `ComplyEaze Bridge` line and of the Tally line, and whether the box's title bar is drawn active.
  - EXPECT box `foreground` 1 (by design since #1589, kept on 21 Oct).
  - Either value is recorded, never acted on. **Type nothing.**
- F4. Click **No** in the box, by mouse. Stamp it.
  - EXPECT the call answers `import_approval_declined`, nothing is dispatched, and the voucher mark is unchanged from B2.
  - If the lab cannot click while its own call waits, the call first answers `pending` at about 40 s. The box stays open until 120 s. Click No then, and record "MODE BLOCKED".
  - Never click Yes. A post or an `approved` answer = STOP: make no further call and report. An approval lapses after 15 minutes unless it is used.
- F5. (a look, no call) A review acknowledgement box opens only after a post, so none is offered here. Write "review box not raised: no post in this row".
- F6. End: the voucher mark is unchanged, and no `bridge_mcp.exe` child with `--confirm-journal` remains (process list). Re-hash the three items.

## Evidence (stays on the lab PC except the table)
- One table, one line per step F0-F6. Each line gives the T0 and click stamps, the probe lines (position, topmost, foreground), and the title and first line as read.
- Also record: whether the box was above Tally (yes/no), foreground (box/Tally), the call's `result` code, and the voucher mark before and after.
- Screenshots go in a list with name, size and sha256. Whole-screen screenshots show other windows, so they stay local.
- Also record the screen size and scale, whether there is one monitor, the Windows build, the Tally release and the Claude Desktop version.

## PASS / FAIL
- **PASS:** F1 the box above Tally; F2 the title and the first line exactly as above; F4 declined with the mark unchanged. F3 is recorded either way.
- **FAIL:**
  - the box behind Tally;
  - the first line missing or different;
  - a No that posts;
  - a box left open or an orphan child after F6.
- On FAIL, keep the screenshots and report the step. #1780 stays open and needs a fix.

## Not measured (say so in the results)
- Keys typed while the box is open (16 Oct, unchanged by design).
- A second monitor; macOS (#1964/#2043 and their own Mac checklist); Windows 11.
- The review acknowledgement box (needs a post).
- A Tally entry screen open under the box (060S D5).
