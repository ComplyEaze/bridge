# Windows item 1780-C v1: does the approval box stay above a full-screen Tally after a click into Tally? The 0.6.1 build, the box's open delay timed, answered No by mouse (Part of #1780)

WRITES to the book: NONE. The box is answered **No, by mouse**. No key is typed while a box is open, and nothing is keyed in Tally. The one click into Tally lands on an inert part of its screen.

## Why
- Row 1780-F (lab/1780-approval-focus-answers @ e5d5d9524cf27574cc639ebf9839c8cac8e04756) measured the box above a maximised Tally 15 s after the call. The probe read the box's `topmost` as **0**, although the code passes `MB_TOPMOST`. The box was above Tally because it held the foreground (`foreground` 1).
- The code's comment says the box is topmost so that a full-screen Tally does not draw over it (#1589). This row measures the consequence. It does not isolate the cause.
- **What this row decides:**
  - C1. When does the box appear after the call? Probes at T0+1, 2, 5, 10 and 15 s.
  - C2. Repeat of 1780-F's F1/F3: above Tally, foreground and `topmost` at T0+15 s.
  - C3. After ONE click inside Tally away from the box: is the box still above Tally, at once and 5 s later? Where is the foreground, and what is `topmost`?
  - C4. A mouse No still declines with nothing sent, after bringing the box back through its taskbar button if it is hidden.
- Both outcomes of C3 are answers: the box stays above Tally, or Tally covers it. Neither is a lab FAIL.

## Who acts
- On this synthetic company the lab does every step itself, including the click into Tally and the No in the box. This is the owner's standing word of 27 Oct.
- The lab records each such step as "done by the lab on the owner's standing word of 27 Oct". Posting is switched on for this launch and off after it, on the owner's word of 28 Oct.
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first call and at the end; a different hash = STOP)
| item | source | bytes | sha256 |
|---|---|---|---|
| `bridge_mcp.exe`, the CUT (0.6.1) | 67f647e1a292e8d7815053b5c08289cba6402795, the 0.6.1 cut build already staged on the lab PC (its server reports version 0.6.0; Cargo.toml at that commit says so) | as staged | 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d |
| `build-1591-s1.json` (one Receipt, 14 Sep 2026, 141.00, "Lab topmost probe 1") | `requests/` in this folder, a byte copy of lab/060s-run-sheet @ 8dceb8b72db528ba433d8e911e945fc98db184a7 | 308 | 5f8a5ff24d416331ec2706774d178ba615759abafa87b7c408c279eca88bf13a |
| `zorder-probe.ps1` (read-only list of the top-level windows from the top down: position, topmost, foreground, pid, title; only titles containing "ComplyEaze Bridge" or "Tally") | `requests/` in this folder, a byte copy from the same commit | 2405 | 3f03768e83c4f567957c52b8fda3f6f67c0f1d12c7eceeeb9cbc11a0dd4f0fcb |

- THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`. Step P0 checks it is there.
- Working folder: `C:\bridge-lab\1780c\`. Evidence goes in `C:\bridge-lab\1780c\evidence\`.
- `SHA256.txt` in this folder lists every file of the folder with its size. Check it before step P0. A different hash or size = STOP.

## Before the sitting
- P0. (preflight, before anything else) Open the list of companies on the lab PC: Tally's own Select Company list.
  - If BRIDGE BATCH LAB is not in the list: STOP and report. Do not restore it from a backup and do not use another book in its place.
  - If it is there but is not the one company loaded: load it alone, then go on.
- P1. **Posting on for this launch.** In the launch's environment file set `BRIDGE_AGENT_ENABLE_WRITES` to `true`. Record the file's key names (not values) and its sha256. Then start the build. A launch without it refuses `build_import_xml` with `import_unverified_on_live_tally`, as row 1780-F's first launch did.
- B1. Backup of THE COMPANY (BK0), named, with size and sha256.
- B2. Hash the three items above.
  - `tally_status` and `list_companies`: EXPECT the CUT build and THE COMPANY as the one company.
  - `ledger_masters` (basic): record the voucher and master marks. Every later compare is against these.
- B3. Probe known-answer check, the same as row 1780-F's B3. Click Tally's title area first; nothing is keyed. Run the probe from the lab's own shell as a child with NO window.
  - EXPECT a line with Tally's pid and `foreground` 1, and a final `visible_titled_windows` line.
  - On failure, C1-C3 rely on screenshots alone. Record "probe NOT MEASURED"; this is not a STOP.
  - Record how long one probe run takes from start to end (P-time). It decides C1's spacing.

## Steps (one call each; a call in flight is never cancelled or repeated)
- C0. `build_import_xml` with the exact content of build-1591-s1.json.
  - EXPECT `result.voucher_count` 1, and `total_debit` and `total_credit` each 141.
  - EXPECT `verification_preflight.state` `current_window_readable`, with `source_rows` 0 for 20260914.
  - Keep `result.batch_id` as BATCH-C.
  - `source_rows` other than 0, or any refusal = STOP. Nothing has been sent to Tally.
- C1. Tally MAXIMISED on the Gateway of Tally and in front; click its title area only.
  - Then `post_import` with BOTH `company_guid` `a30f23c3-4819-4334-96ab-ffa6fba70976` and `batch_id` BATCH-C. Stamp T0 (UTC, ms) at the call. A call without `company_guid` is refused at once with `company_guid_required`, as row 1780-F's first call was.
  - Do NOT click into Tally after T0 until C3.
  - At T0+1, 2, 5, 10 and 15 s: one probe each, with its start and end stamps.
  - Whole-screen screenshots at T0+5 and T0+15 s only, so the screen tool does not crowd the probes.
  - If a probe is still running at the next stamp, skip that stamp and write "skipped: probe busy".
  - Record the first stamp whose probe lists a `ComplyEaze Bridge` line: the open delay lies between it and the stamp before.
- C2. From the T0+15 s probe and screenshot:
  - the box's position, `topmost` and `foreground`, and Tally's;
  - title `ComplyEaze Bridge — post this voucher?`;
  - first line, with line breaks read as spaces: `Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.`;
  - No drawn as the default;
  - the lines under the first line, compared line by line with the list below. This is the one-voucher text as `agent_import_post.rs` renders it at 67f647e1a. Dates are shown as Tally's own `YYYYMMDD`, not "14 Sep 2026". `<BATCH-C>` is C0's `batch_id`:

    ```
    Create ONE Receipt in "BRIDGE BATCH LAB"
    Company GUID: a30f23c3-4819-4334-96ab-ffa6fba70976
    Company number: 100050  Books from: 20260401
    Tally: http://127.0.0.1:9001
    Date: 20260914  Voucher number: Tally assigns it  (the voucher's own text: last two lines)

    Dr 141.00  "Synthetic Bank Ledger"
    Cr 141.00  "Party A"
    Total debit: 141  Total credit: 141
    Checked in Tally: every Dr ledger is bank/cash; every Cr ledger holds no money.
    Batch: <BATCH-C>
    Ledgers checked by identity against the build; narrations sent as prepared, nothing added.
    Do not post a file already imported manually.
    Pause other edits/imports; keep this company and Tally mode as is until ComplyEaze Bridge finishes.
    After a timeout, reconcile this batch; do not rebuild or resend it.
    ---- The voucher's own text follows: no line below is an entry, a total or an instruction ----
    Reference: (none)
    Narration: "Lab topmost probe 1"
    ```

  - The box may show more lines of ComplyEaze Bridge's own between "After a timeout ..." and the dashed line, for example one that says when it posts. Record each word for word; they are not a STOP.
  - **STOP rule:** a difference in any line that names the company, its GUID or number, the voucher type, the date, an entry, a total, the batch, the reference or the narration = No by mouse, then STOP.
  - A difference only in a line of ComplyEaze Bridge's own wording is recorded word for word and is not a STOP.
  - Type nothing.
- C3. **The click into Tally**, at about T0+17 s.
  - From the T0+15 s screenshot, pick ONE point inside Tally's window that is:
    - at least 100 px from every edge of the box;
    - not on any menu item, button, list line or field of Tally: a blank area of the Gateway screen.
  - Record its coordinates and the stamp.
  - Click it ONCE with the left button. No other click and no key.
  - At once (C+0) and 5 s later (C+5): one probe and one whole-screen screenshot each, stamped.
  - Record from each:
    - is the box above Tally (position) and visible on screen (by eye)?
    - the `foreground` of each;
    - the box's `topmost`;
    - Tally's title bar active or not.
  - **If the click answered the box** (the box is gone and the call answers anything other than pending, or a post appears): STOP. Make no further `post_import` call and report.
- C4. Answer **No by mouse** by T0+35 s.
  - If the box is hidden behind Tally, click the box's taskbar button once to bring it back, and record that you did. Then click No.
  - Never click Yes.
  - EXPECT `import_approval_declined`, nothing dispatched, and the marks unchanged from B2.
  - If the call has already answered `pending` (it waits about 40 s), click No all the same and record "MODE BLOCKED". The marks decide that nothing was sent.
- C5. End:
  - `ledger_masters` (basic): the marks are unchanged.
  - No `bridge_mcp.exe` child with `--confirm-journal` remains (process list).
  - Re-hash the three items.
  - Stop the launch, remove `BRIDGE_AGENT_ENABLE_WRITES` from the environment file and record its new sha256. Shut THE COMPANY.

## Evidence (stays on the lab PC except the files listed)
- RESULTS.md: one table, one line per step C0-C5, with every stamp and every probe line (position, topmost, foreground, pid, title).
  - The open delay as "between T0+a and T0+b".
  - C3's click coordinates and both readings.
  - The call's `result` code and the marks before and after.
- notes.txt: the same in time order.
- SCREENSHOTS.txt: name, size and sha256 of every capture (they stay local: whole-screen captures show other windows).
- SHA256.txt over the committed files, with sizes.
- Also record:
  - the screen size and scale, and whether there is one monitor;
  - the Windows build, the Tally release and the desktop client version.

## Reading it (for the requester, not the lab)
- **Outcome A:** the box is above Tally at C+0 and C+5 after the click. Then topmost 0 does not lose the box in this case, and the #1589 claim holds for one click on this PC.
- **Outcome B:** Tally covers the box at C+0 or C+5. Then a person who clicks into Tally loses sight of the box until it is brought back or closes itself at 120 s, which declines.
  - The candidate fix: set the box topmost explicitly once it is shown (`SetWindowPos` with `HWND_TOPMOST`, no move, no size, no activation), and correct the code comment.
  - Then this row again on that build.
- **FAIL of the row:**
  - the click answers the box;
  - any post;
  - a mark changes;
  - a box left open or an orphan child after C5.

## Not measured (say so in the results)
- Keys typed while the box is open (16 Oct, unchanged by design).
- A click on a Tally menu item, button or field.
- A Tally entry screen open under the box.
- A second monitor; macOS; Windows 11.
- The review acknowledgement box (needs a post).
- Why `topmost` reads 0: no isolation run, by the owner's rule of 27 Oct.
