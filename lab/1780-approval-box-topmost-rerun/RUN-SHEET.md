# Windows item 1780-C2 v1: row 1780-C again, on the build that sets the approval box topmost once it is shown. Does the box now stay above a maximised Tally after a click into Tally? Answered No by mouse (Part of #1780)

WRITES to the book: NONE. The box is answered **No, by mouse**. Nothing is keyed in Tally, with one exception: step C3b presses the Down arrow once, a key that cannot answer the box. The one click into Tally lands on an inert part of its screen.

## Why
- Row 1780-C (lab/1780-approval-box-click-into-tally-answers @ a85d5dd6b25628811198d8b25f146022b6bae545) measured on the 0.6.1 cut build:
  - the box's `topmost` read 0;
  - after one click on a blank part of a maximised Tally, Tally covered the box at once and 5 s later (Outcome B).
- The pull request on branch `fix/1780-approval-box-stays-topmost` sets the box topmost once it is shown. This row runs 1780-C again on that pull request's Windows build.
- **What this row decides:**
  - T1. Is the box's `topmost` now 1 once it is shown?
  - T2. After one click into Tally, is the box still above Tally, at once and 6 s later? Where is the foreground?
  - T3. After that click, does a harmless key go to Tally rather than the box? Topmost must not take the keyboard.
  - T4. A mouse No still declines with nothing sent.

## Who acts
- On this synthetic company the lab does every step itself, including the click into Tally, the one Down key and the No in the box. This is the owner's standing word of 27 Oct.
- The lab records each such step as "done by the lab on the owner's standing word of 27 Oct". Posting is switched on for this launch and off after it, on the owner's word of 28 Oct.
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first call and at the end; a different hash = STOP)
| item | source | bytes | sha256 |
|---|---|---|---|
| `bridge-tally.mcpb`, the bundle BUILD-T comes in | #2107 at head a66c31f2ab607b36701ea0474ca15d474b9e202c: CI run 40709511734, artifact 12826362269 `unsigned-bundle-smoke-windows-latest` (expires 4 Nov 2026 15:05 UTC), file `bridge-tally.mcpb` | 15,320,736 | 61e52d079544b2c7dd49cb47bf08a11120b0154957e830e8a76e598970ab99c2 |
| `bridge_mcp.exe`, BUILD-T | `bin/x86_64-pc-windows-msvc/bridge_mcp.exe` inside that `.mcpb`; its server reports version 0.6.1 | 31,804,416 | 3ca00fab5ea77314df3a34cfd22d049a5a60ecbbf14ec2b9e253bab097bad974 |
| `build-1591-s1.json` (one Receipt, 14 Sep 2026, 141.00, "Lab topmost probe 1") | `requests/` in this folder, a byte copy of lab/060s-run-sheet @ 8dceb8b72db528ba433d8e911e945fc98db184a7 | 308 | 5f8a5ff24d416331ec2706774d178ba615759abafa87b7c408c279eca88bf13a |
| `zorder-probe.ps1` (read-only list of the top-level windows from the top down: position, topmost, foreground, pid, title; only titles containing "ComplyEaze Bridge" or "Tally") | `requests/` in this folder, a byte copy from the same commit | 2405 | 3f03768e83c4f567957c52b8fda3f6f67c0f1d12c7eceeeb9cbc11a0dd4f0fcb |

- THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`. Step P0 checks it is there.
- Working folder: `C:\bridge-lab\1780c2\`. Evidence goes in `C:\bridge-lab\1780c2\evidence\`.
- `SHA256.txt` in this folder lists every file of the folder with its size. Check it before step P0. A different hash or size = STOP.

## Before the sitting
- P0. (preflight, before anything else) Open the list of companies on the lab PC: Tally's own Select Company list.
  - If BRIDGE BATCH LAB is not in the list: STOP and report. Do not restore it from a backup and do not use another book in its place.
  - If it is there but is not the one company loaded: load it alone, then go on.
- P1. **Posting on for this launch.** In the launch's environment file set `BRIDGE_AGENT_ENABLE_WRITES` to `true`, and point the launch at BUILD-T. Record the file's key names (not values) and its sha256. Then start the build.
- B1. Backup of THE COMPANY (BK0), named, with size and sha256.
- B2. Hash the three items above.
  - `tally_status` and `list_companies`: EXPECT BUILD-T and THE COMPANY as the one company.
  - `ledger_masters` (basic): record the voucher and master marks. Every later compare is against these.
  - Read and record the desktop client's version afresh. Do not copy it from an earlier row.
- B3. Probe known-answer check, as in row 1780-C's B3. Record P-time, how long one probe run takes from start to end.
- B4. **The two frames.** Record:
  - the screen tool's frame (its screenshot size, for example 1456 x 819);
  - the screen's logical size and scale (for example 1536 x 864 at 125 %).

  Every click below is given in both frames: the screen tool's coordinates, and the same point in logical screen pixels, scaled by the ratio of the two sizes. Also record the box's rectangle in logical pixels, read from the screenshot or the window list, so a click's distance from the box is in logical pixels.

## Steps (one call each; a call in flight is never cancelled or repeated)
- C0. `build_import_xml` with the exact content of build-1591-s1.json.
  - EXPECT `result.voucher_count` 1, and `total_debit` and `total_credit` each 141.
  - EXPECT `verification_preflight.state` `current_window_readable`, with `source_rows` 0 for 20260914.
  - Keep `result.batch_id` as BATCH-T.
  - `source_rows` other than 0, or any refusal = STOP. Nothing has been sent to Tally.
- C1. Tally MAXIMISED on the Gateway of Tally and in front; click its title area only.
  - Then `post_import` with BOTH `company_guid` `a30f23c3-4819-4334-96ab-ffa6fba70976` and `batch_id` BATCH-T. Stamp T0 (UTC, ms).
  - Do NOT click into Tally after T0 until C3.
  - Probes at T0+3, T0+8 and T0+13 s, each with its start and end stamps.
    - These are spaced for a probe of about 2 s (P-time). If P-time is over 3 s, start each probe when the previous one ends, and record the stamps.
  - A whole-screen screenshot at T0+13 s, after its probe ends.
  - Record the first stamp whose probe lists a `ComplyEaze Bridge` line.
- C2. From the T0+13 s probe and screenshot:
  - the box's position, **`topmost`** and `foreground`, and Tally's;
  - title `ComplyEaze Bridge — post this voucher?`;
  - the first line, with line breaks read as spaces: `Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.`;
  - No drawn as the default.

  The lines under the first line are exactly as row 1780-C's sheet lists them, dates included (`Books from: 20260401`, `Date: 20260914`). BUILD-T was built before the change that shows dates in day-month-year form (#2105).

  **STOP rule:** a difference in any line that names the company, its GUID or number, the voucher type, the date, an entry, a total, the batch, the reference or the narration = No by mouse, then STOP. Type nothing.
- C3. **The click into Tally, at T0+16 s (±2 s).**
  - From the T0+13 s screenshot, pick ONE point inside Tally's window that is:
    - at least 150 logical pixels from every edge of the box;
    - not on any menu item, button, list line or field of Tally: a blank area of the Gateway screen.
  - Record the point in both frames (B4), its distance from the box's nearest edge in logical pixels, and the stamp.
  - Click it ONCE with the left button.
  - At once (C+0) and 6 s later (C+6): one probe and one whole-screen screenshot each, stamped. Record from each:
    - is the box above Tally (position) and visible on screen (by eye)?
    - the `foreground` of each;
    - the box's `topmost`.
  - **If the click answered the box** (the box is gone and the call answers anything other than pending, or a post appears): STOP. Make no further `post_import` call and report.
- C3b. **One harmless key**, right after the C+6 probe. Only if the box is still open, unanswered and Tally holds the foreground:
  - press the **Down arrow** ONCE. No other key. Down moves a selection; it answers neither Tally nor the box.
  - Screenshot at once.
  - Record whether Tally's Gateway highlight moved (the key went to Tally), whether the box's focused button changed (the key went to the box), or neither.
  - If the box holds the foreground at C+6, skip C3b and write "skipped: the box held the foreground".
- C4. Answer **No by mouse** as soon as C3b is recorded, aiming for before T0+38 s.
  - If the box is hidden behind Tally, click its taskbar button once to bring it back, and record that you did. Then click No.
  - Never click Yes.
  - EXPECT `import_approval_declined`, nothing dispatched, and the marks unchanged from B2.
  - If the call has already answered `pending` (it waits about 40 s), click No all the same and record "MODE BLOCKED". The marks decide that nothing was sent.
- C5. End:
  - `ledger_masters` (basic): the marks are unchanged.
  - No `bridge_mcp.exe` child with `--confirm-journal` remains (process list).
  - Re-hash the three items.
  - Stop the launch, remove `BRIDGE_AGENT_ENABLE_WRITES` and record the environment file's new sha256. Shut THE COMPANY.

## Evidence (stays on the lab PC except the files listed)
- RESULTS.md: one table, one line per step C0-C5 and C3b, with every stamp and every probe line (position, topmost, foreground, pid, title).
  - The open delay as "between T0+a and T0+b".
  - C3's click in both frames and its distance from the box.
  - C3b's reading.
  - The call's `result` code and the marks before and after.
- notes.txt: the same in time order.
- SCREENSHOTS.txt: name, size and sha256 of every capture (they stay local: whole-screen captures show other windows).
- SHA256.txt over the committed files, with sizes.
- Also record:
  - BUILD-T's pull request, head, CI run, artifact and exe sha256;
  - the screen size and scale, and whether there is one monitor;
  - the Windows build, the Tally release and the desktop client version, read afresh.

## Reading it (for the requester, not the lab)
- **The fix holds (Outcome A):**
  - the box's `topmost` is 1 at T0+13;
  - the box is above Tally at C+0 and C+6 after the click;
  - in C3b the Down key reached Tally, not the box.
- **The fix does not hold:**
  - `topmost` is still 0, or Tally covers the box after the click (Outcome B again). Report it; the pull request does not do what it says.
  - The Down key reached the box while Tally held the foreground. Report it as a keyboard finding, separate from topmost.
- **FAIL of the row:**
  - the click or the key answers the box;
  - any post;
  - a mark changes;
  - a box left open or an orphan child after C5.

## Not measured (say so in the results)
- Any key other than one Down arrow; typed letters.
- A click on a Tally menu item, button or field.
- A Tally entry screen open under the box.
- The ledger-create box, which this build does not change.
- A second monitor; macOS; Windows 11.
- The review acknowledgement and invoice-stop boxes, which go through the same code.
