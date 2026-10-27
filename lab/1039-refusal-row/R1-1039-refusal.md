# R1 (#1039): one native post whose span binding is refused, on purpose, kept for the review dialog

Not for merge. `LAB-COMMON.md` (v3) and `README.md` in this folder apply in full; where this row and they differ, the stricter holds. Steps marked [OWNER]: see WHO ACTS in `README.md`.

**THE COMPANY** is BRIDGE CONCUR LAB, as G1 (`lab/1039-live-rows`) creates it:
- a bank ledger BRIDGE Lab Bank;
- parties BRIDGE Conc Party 1 and 2 (Sundry Debtors, bill-by-bill No);
- voucher type Receipt, Automatic with "Retain Original Voucher No.";
- the seed Receipt "G1-SEED" and row A's voucher.

This row runs after G1 row A has ended (its step A4 recorded), before G1's AFTER restore. If THE COMPANY is not on the lab PC, or G1 row A has not run: STOP. Never create a substitute. Licensed mode only; Educational mode anywhere = STOP.

**THE CALLS.** Only the build's own tools, one at a time. An answer's `next_step` is recorded, never acted on. Paths below are inside `structuredContent.result` of the tool's answer.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay). Record the process list and that `BRIDGE_AGENT_ENABLE_WRITES` is present. Hash the exe.
2. `list_companies`: exactly THE COMPANY; record its GUID. MARKS: M0, V0 (V0 must be a number).
3. `vouchers` for 1 April 2025 to 30 April 2025: record each row's date, number, narration, MasterID, AlterID and GUID (the before table). EXPECT the seed and row A's voucher, and nothing dated 21 to 23 or 28 April 2025; another voucher on those days = STOP.
4. Backup BK-R (C5) into a NEW empty folder `C:\bridge-lab\1039-refusal\backups\BK-R\`: name, size and sha256. Then MARKS.

The book: unchanged.

## Attempts (at most three; k = 1, 2, 3; stop after the first refusal)

The batch of attempt k is dated (20 + k) April 2025. Its prepared voucher is dated 28 April 2025, which is outside every batch's dates. That way, saving it never touches the window the post reads before it sends. The first delay d1 is 2.0 seconds.

1. [OWNER] On Tally's screen, open Receipt voucher entry and fill one Receipt:
   - dated 28 April 2025;
   - BRIDGE Conc Party 2 to BRIDGE Lab Bank, 1.00;
   - narration `R-HAND-k`.
   Leave it filled and NOT accepted, at its last field. Screenshot it.
   - G1 row B prescribes the same prepared voucher during a post.
   - Any call of this attempt that fails or does not answer while the screen is half-entered: STOP, with no key pressed (C7). Report what is on the screen.
2. MARKS: Vk0. `build_import_xml` with 50 Receipts:
   - each 1.00, from BRIDGE Conc Party 1 into BRIDGE Lab Bank;
   - dated (20 + k) April 2025;
   - narrations `R-k-01` to `R-k-50`;
   - no voucher number.
   EXPECT `voucher_count` 50. Record the answer whole and its `batch_id` as Bk. Any refusal: record it whole and STOP. Say whether a voucher was left half-entered on the screen.
3. `post_import` with `company_guid` and `batch_id` Bk.
   - When the approval window opens, [OWNER] reads it and clicks **Yes**. At the same moment a stopwatch starts.
   - At dk seconds after the Yes, [OWNER] accepts the prepared voucher on Tally's screen by mouse. Tally's own Accept prompt is answered by mouse; no key is pressed.
   - Record the UTC time of the Yes and of the accept, to the tenth of a second, from one clock.
   - If `approval.state` reads `pending`, call `post_import` again with Bk after `approval.retry_after_s` seconds. If it reads `approved` without a send, call again at once. Record every answer whole.
4. In the answer of the call that sent the post, record whole:
   - `dispatch.state`;
   - `post_location` (its `state`, and `target_voucher_step` with `before`, `after`, `step`, `reported_created`, `matches_created`);
   - `post_span_binding` (`state`, `code`, `summary`).
   A refusal before the send (an answer whose `attempt_recorded` is false, for example a read that drifted): record it whole. The attempt ends with nothing sent; go to item 6 with the same delay.
5. Then:
   - Screenshot the whole Tally window; a dialog = C7, STOP.
   - `vouchers` for 28 April 2025: the row narrated `R-HAND-k`; record its AlterID as Hk.
   - `vouchers` for (20 + k) April 2025: EXPECT 50 rows narrated `R-k-01` to `R-k-50` when the post was sent, and none when it was refused before the send; another count = STOP.
   - MARKS: Vk1.
6. Read where the save landed. `before` and `after` are `target_voucher_step`'s.

   | Hk is | Then | EXPECT | Next |
   |---|---|---|---|
   | at or below `before` | early | `step` 50, `post_span_binding.state` `bound` | dk+1 = dk + 1.0 |
   | above `before` and at or below `after` | inside the window | `step` 51, `matches_created` false, `post_span_binding.state` `refused` with `code` `span_step_not_created` | **REPRODUCED**: go to step 7 |
   | above `after` | late | `step` 50, `bound` | dk+1 = max(0.3, dk − 0.7) |

   An inside save whose answer is not that refusal, or a refusal with an early or late save: record it whole and STOP. That is a FINDING.
7. After the last attempt, `verify_import` of every batch sent, in order. For the refused batch, EXPECT:
   - `post_span_binding.state` `refused` with `code` `span_step_not_created`;
   - its `summary` recorded whole, noting whether it carries the line beginning "The company's voucher mark moved by a different amount" (present only on a build with #1896);
   - `verification_status` `verification_incomplete`;
   - `counts.posted_verified` 0;
   - its 50 vouchers `matching_content_observed`.
   For a bound batch, EXPECT `posted_verified`. Record each answer whole. Then `local_data_report`: record it whole, and the count it gives under `not_settled_binding_refused`.

Not reproducing a refusal in three attempts is a result: record "not reproduced in 3 attempts", with every dk, Hk, `before` and `after`. Do not add attempts without the run coordinator's word.

## The book after each step
- Step 0: unchanged.
- Each attempt: plus 50 Receipts dated (20 + k) April 2025 (when sent), and one hand Receipt dated 28 April 2025.
- After the row: as left by the attempts. Nothing is restored here.

STOP RULES: C7 and C8; any dialog in Tally other than the voucher's own Accept prompt; a mark that moves outside a step that changes it; a voucher count other than expected; an approval window other than the attempt's own.

## Close and evidence
C10: Shut Company; no company loaded.

**Keep** this row's data folder, THE COMPANY as it now stands, and BK-R. The later #1039 dialog row reads the refused batch from them. The book is restored to BK-R only on the run coordinator's explicit RESTORE (C6). It has no other fallback: a restore that does not complete is a STOP.

Evidence goes on a new branch `lab/1039-refusal-answers` with `SHA256.txt`, after the pre-push scan and the run coordinator's yes:
- every answer whole, with its size and sha256;
- the before table and each attempt's rows;
- per attempt: dk, the two UTC times, `before`, `after`, Hk and the class;
- the `egress_log` lines;
- BK-R's name, size and sha256, and the build's hashes;
- a notes file with UTC times, one line per step.

Screenshots, BK-R and the data folder stay on the lab PC, listed by name, size and sha256.

## Not covered
- A refusal from an edit before a deferred bind. The first bind runs inside the post's own readback, so it is deferred only when that readback fails after the send, and no step here can cause that.
- A second Tally client (Gold).
- Codes other than `span_step_not_created`.
- Whether Tally's screen accepts a voucher while an import runs. It is recorded, not judged: a save that waits for the import lands late.
