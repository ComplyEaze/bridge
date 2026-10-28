# Results: #1039 R1, 28 Oct 2026. REPRODUCED on the first attempt: a Payment saved on Tally's screen inside a 50-voucher post's span made the step 51 against 50 created, and the binding was refused `span_step_not_created`. The batch is kept for the dialog row

The sheet is `lab/1039-refusal-row` at 08bb7491d9eef797d9975fdca46630ff1b71cbb1 (`SHA256.txt` 4 of 4 equal), run straight after G1 row A on BRIDGE CONCUR LAB.
- **Company:** number 100091, GUID 88239cf8-ef69-4cea-8dff-4d1fd91453ee, synthetic.
- **Build:** the one G1 row A ran on, the 0.6.1 cut, master 67f647e1. `bridge_mcp.exe` sha256 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d at the first call and at the end. One stdio launch with its own new data folder, posting ON.
- **Tally:** TallyPrime 7.2, Silver, Licensed.
- **D1:** used with the recorded yes of 28 Oct (the order's row note).
- **[OWNER] steps:** done by the lab on the owner's word of 28 Oct; the person's mouse click is not measured.

**Recording (an accepted deviation):** in place of a video with an on-screen clock, a frame recorder captured the whole screen about 7 times a second, with each frame's UTC time to the millisecond drawn on it and in its name. It ran from 05:31:08.342 to 05:40:11.860 (3,553 frames). `rec-manifest.txt` lists every frame by sha256; the frames stay on the lab PC.

## Step 0
- **Reads:** `list_companies` gave exactly BRIDGE CONCUR LAB. MARKS M0 210, V0 3.
- **Before table** (`vouchers` 1 to 30 Apr 2025): G1-SEED (Receipt 1, 1 Apr, MasterID 1, AlterID 1) and G1-A edited (Receipt 2, 10 Apr, MasterID 2, AlterID 3). Nothing is dated 21 to 23 or 28 April.
- **BK-R:** `TDBK1800_100091.001`, 27,408 B, sha256 56c851bc78b71616d9c9ee243e401876622edc247e7b949ed258f1b79924cf54. MARKS after it: 210 / 3.
- **Tally's current date:** Thursday 10-Apr-2025. It was set back to this before the close.

## Step 1, dry run of the save control
R-HAND-0 is a Payment dated 28 Apr 2025, BRIDGE Lab Bank to BRIDGE Conc Party 2, 0.10. It was saved by one click on "A: Accept" in Tally's bottom bar, with no prompt. Md0/Vd0 210/3 and Md1/Vd1 210/4, so **SV = 1, SM = 0**.

## Attempt 1 (d1 = 2.0 s)
| item | value |
|---|---|
| batch B1 | `bridge-a4eb06f8-cf9a-43a6-9255-3920518cf325`, 50 Receipts, 21 Apr 2025, 62.75 / 62.75 |
| hand Payment | R-HAND-1, keyed and left unsaved before V10 (D1) |
| V10 | 4 |
| post_import sent | 05:38:30Z |
| approval window first seen | 05:38:37.966Z (behind Tally; brought forward and read whole: totals only) |
| Yes | between frames 05:38:49.328 and 05:38:50.057, 11.4 to 12.1 s after it was seen, while the first call was open |
| save control clicked | 2.0 s after the Yes (about 05:38:51.4 to 52.1) |
| saved state first drawn | frame 05:38:55.652 (05:38:55.374 still unsaved; Tally was busy with the import) |
| answer | 05:39:00.3Z (30.284 s): no answer by d1 after the Yes, so **anchored** |
| `dispatch.state` | `reconciliation_required`, created 50, errors 0, `last_vch_id` 53 |
| `post_location` | `target_only`, no `after_read_failure`; `target_voucher_step` before **4**, after **55**, step **51**, reported_created 50, `matches_created` **false** |
| `post_span_binding` | **`refused`, code `span_step_not_created`**, summary as below |
| H1 (R-HAND-1's AlterID) | **55** (MasterID 54) |
| 21 Apr rows | 50, R-1-01 to R-1-50, AlterID 5 to 54 |
| V11 | 55; V11 - V10 = 51, as expected for a sent post |
| class | H1 above `before` and at or below `after`: **inside**, so **REPRODUCED** |

**The extra click.** A second click on the save control, sent before the lab saw that the first had saved, landed on the blank Payment No. 3. It put BRIDGE Conc Party 1 into that voucher's By line and saved nothing; one Ctrl+Q left it with no prompt. The 28 April read (exactly one R-HAND-1, beside R-HAND-0) and V moving by exactly 51 show that no other voucher was saved.

**Summary text:** "ComplyEaze Bridge could not confirm which Tally vouchers this post created, so the batch stays open. For each voucher it sent, the book holds a voucher with the same date, voucher type and ledger entries, but ComplyEaze Bridge cannot tell whether that one is this post's: check each voucher in Tally before posting any of them again. ComplyEaze Bridge cannot close this batch or amend it."

## Step 8
- **`verify_import` B1:**
  - `post_span_binding` refused `span_step_not_created`, with the summary above;
  - `verification_status` `verification_incomplete` (error `import_reconciliation_required`);
  - `counts.posted_verified` 0;
  - all **50 vouchers `matching_content_observed`**.
- **#1896's line:** there is no line beginning "The company's voucher mark moved by a different amount". The build's commit 67f647e1 does not hold #1896's merge 7f7f50e5, so its absence is what the sheet expects.
- **`local_data_report`:** `journal.state` `read`, `not_settled_binding_refused` 1, `not_settled` 1 (batches 1).

Every EXPECT of steps 1 to 8 held.

## The book after the row, and what is kept
- **The book now holds:** the seed, G1-A edited, R-HAND-0, the 50 Receipts of 21 Apr 2025, and R-HAND-1.
- **Close:** Tally's current date was set back to 10-Apr-2025, then C10: shut, no company loaded, one tally.exe on 9001, /status 200.
- **Kept on the lab PC** for the #1039 dialog row: this row's data folder (the refused batch and its journal), the company as it stands, and BK-R.
- **Restores:** BK-R is restored only on a RESTORE after that row. G1's BK1 restore also waits.

## Files
- `answers/`: every tool answer whole (000 to 014).
- `client-log.txt`.
- `notes.txt` (UTC, one line per step).
- `egress.jsonl`: the row's egress log, copied from its data folder.
- `rec-manifest.txt`.
- `SCREENSHOTS.txt`: the captures, the recorder log and BK-R by sha256.
