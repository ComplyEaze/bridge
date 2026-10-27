# R1 (#1039): one native post whose span binding is refused, on purpose, kept for the review dialog

Not for merge. `LAB-COMMON.md` (v3) and `README.md` in this folder apply in full; where this row and they differ, the stricter holds, except for the one named deviation D1 below. Steps marked [OWNER]: see WHO ACTS in `README.md`.

## Go-ahead and deviation D1 (before step 0)
- This row is not in the 0.7.0 order on its own. It runs only after:
  - the run coordinator has added a row note to `lab/0.7.0-auto-order/ORDER.md` naming this branch head;
  - the run coordinator has given a recorded go-ahead for that head.
- **D1, a deviation from LAB-COMMON C2 and C3.** In each attempt:
  - the MARKS read Vk0 and every `post_import` call of the attempt (their reads, the POST and its readback) are sent while one new Payment sits filled and unsaved on Tally's screen;
  - C3's "no key pressed since the MARKS read before a write" still holds: the keying is done before Vk0.
- D1 needs the run coordinator's recorded yes, given before step 0 for this branch head; without it, STOP.
- The nearest public observation is the protocol reference's measurements part, §11c.5 (one run): "A voucher held open in alteration blocked none of Bridge's reads, builds or posts, and Tally raised no dialog". Its limits:
  - it was an alteration, not an unsaved new voucher;
  - it was not a 50-voucher import;
  - it was measured once.
- Every other C2 and C3 rule stands. The dry run (step 1) sends nothing while a voucher is unsaved: it checks the save control and the hand save's marks only, and D1 is first used at attempt 1.

## THE COMPANY
BRIDGE CONCUR LAB, as G1 (`lab/1039-live-rows`) creates it:
- a bank ledger BRIDGE Lab Bank;
- parties BRIDGE Conc Party 1 and 2 (Sundry Debtors, bill-by-bill No);
- voucher type Receipt, Automatic with "Retain Original Voucher No.";
- the seed Receipt "G1-SEED" and row A's voucher.

This row runs after G1 row A has ended (step A4 recorded). **G1's AFTER restore and G1's C10 are deferred until the #1039 dialog row has finished**; the lab tells the run coordinator before any RESTORE of this company. If THE COMPANY is not on the lab PC, or G1 row A has not run: STOP. Never create a substitute. Licensed mode only; Educational mode anywhere = STOP.

**THE CALLS.** Only the build's own tools, one at a time. An answer's `next_step` is recorded, never acted on. Paths below are inside `structuredContent.result` of the tool's answer.

**TIMES.** A screen recording of the whole PC screen with an on-screen clock (UTC, tenths of a second), from step 1 to the last attempt's step 6. Each time below is read from it. The recording stays on the lab PC, listed by name, size and sha256.

**THE SAVE CONTROL.** One mouse click on the Accept control of Tally's bottom bar (the click equivalent of Ctrl+A), which saves the voucher with no confirmation prompt. Any prompt or dialog after that click = C7, STOP. If the first click only brings Tally to the front, record it; the save time is that of the click that saved.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay). Record the process list and that `BRIDGE_AGENT_ENABLE_WRITES` is present. Hash the exe.
2. `list_companies`: exactly THE COMPANY; record its GUID. MARKS: M0, V0 (V0 must be a number).
3. `vouchers` for 1 to 30 April 2025: record each row's date, type, number, narration, MasterID, AlterID and GUID (the before table). EXPECT the seed and row A's voucher, and nothing dated 21 to 23 or 28 April 2025; anything else = STOP.
4. Backup BK-R (C5) into a NEW empty folder `C:\bridge-lab\1039-refusal\backups\BK-R\`: name, size and sha256. Then MARKS.

The book: unchanged.

## Step 1: dry run of the save control (no call open)
1. MARKS: Md0, Vd0.
2. [OWNER] Key one Payment on Tally's screen:
   - dated 28 April 2025;
   - from BRIDGE Lab Bank to BRIDGE Conc Party 2, 0.10;
   - narration `R-HAND-0`.
   Screenshot it filled. Save it with THE SAVE CONTROL. Screenshot the screen after the click.
3. A prompt or dialog = STOP: D1 cannot be used this way.
4. Leave the new blank voucher screen as C7 says (a screenshot of its title, then Ctrl+Q), and screenshot the Gateway.
5. MARKS: Md1, Vd1. Record the hand save's steps: SV = Vd1 − Vd0 and SM = Md1 − Md0.
   - SM other than 0 = STOP: a hand save would then make an early-landing attempt refuse `post_masters_moved` before the send.
   - SV other than 1 = STOP: one save must take exactly one AlterID for the classification below to hold.

The book: plus `R-HAND-0`.

## Attempts (at most three; k = 1, 2, 3; stop after the first refusal)
The batch of attempt k is dated (20 + k) April 2025. Its hand Payment is dated 28 April 2025, outside every batch's dates and of another voucher type. The first delay d1 is 2.0 seconds.

1. `build_import_xml` with 50 Receipts:
   - from BRIDGE Conc Party 1 into BRIDGE Lab Bank;
   - amounts 1.01, 1.02 and so on to 1.50, one each;
   - dated (20 + k) April 2025;
   - narrations `R-k-01` to `R-k-50`;
   - no voucher number.
   EXPECT `voucher_count` 50. Record the answer whole and its `batch_id` as Bk. Any refusal: record it whole and STOP.
2. [OWNER] Key the hand Payment as in step 1.2, narration `R-HAND-k`, and leave it filled and NOT saved. Screenshot it. Keep Tally the active window.
3. MARKS: Vk0 (under D1).
4. `post_import` with `company_guid` and `batch_id` Bk (under D1).
   - When the approval window opens, [OWNER] reads it and clicks **Yes** within 15 seconds of its appearing. A batch of 50 shows totals only.
   - **Anchored:** if the Yes was clicked while that first call was still open, and no answer had come back by dk seconds after the Yes, [OWNER] saves the hand Payment with THE SAVE CONTROL at dk.
   - **Unanchored:** otherwise (an answer came back before the Yes or before dk, for example `pending`, or `approved` without a send), do NOT save at dk. Follow the tool's flow: call `post_import` again with Bk after `approval.retry_after_s` seconds, or at once for `approved`, and never rebuild. Record the attempt as "unanchored". dk stays as it was, and the attempt counts as one of the three. A save made at dk that turns out early is harmless: the attempt is then not judged.
   - Record every answer whole.
5. After the attempt's final answer, whatever it is: if the hand Payment is not yet saved, save it now with THE SAVE CONTROL, and record the time. Then, in the answer of the call that sent the post (if any), record whole:
   - `dispatch.state`;
   - `post_location` (its `state`, any `after_read_failure`, and `target_voucher_step` with `before`, `after`, `step`, `reported_created`, `matches_created`);
   - `post_span_binding` (`state`, `code`, `summary`).
   Two cases end the attempt:
   - **Refused before the send** (`attempt_recorded` false): record it whole, run step 6 in full, then go to the next attempt with the same dk. `post_import` is never called on Bk again.
   - **A `post_import` that fails after the send:** record it whole, then STOP.
6. Then:
   - Leave the new blank voucher screen as in step 1.4.
   - `vouchers` for 28 April 2025: EXPECT exactly one row narrated `R-HAND-k`; none or several = STOP. Record its AlterID as Hk. Step 7 classifies only anchored attempts that sent.
   - `vouchers` for (20 + k) April 2025: EXPECT 50 rows narrated `R-k-01` to `R-k-50` when the post was sent, none when it was not; anything else = STOP.
   - MARKS: Vk1. EXPECT Vk1 − Vk0 = 51 when sent and 1 when not; anything else = STOP.
7. Classify where the save landed. `before` and `after` are `target_voucher_step`'s.

   | Case | Then | EXPECT | Next |
   |---|---|---|---|
   | Hk at or below `before` | early | `step` 50, `post_span_binding.state` `bound` | dk+1 = dk + 1.0 |
   | Hk above `before` and at or below `after` | inside | `step` 51, `matches_created` false, `post_span_binding.state` `refused` with `code` `span_step_not_created` | **REPRODUCED**: go to step 8 |
   | Hk above `after` | late | `step` 50, `bound` | dk+1 = max(0.3, dk − 0.7) |
   | `post_location.state` `after_snapshot_unavailable`, or `after_read_failure` present, or `target_voucher_step` null | cannot be classified | none | record whole, STOP: a FINDING |
   | unanchored | not judged | record | next attempt, same dk |

   An inside save whose answer is not that refusal, or a refusal with an early or late save: record it whole and STOP. That is a FINDING.
8. After the last attempt, `verify_import` of every batch sent, in order. Record each answer whole.
   - For the refused batch, EXPECT:
     - `post_span_binding.state` `refused` with `code` `span_step_not_created`;
     - its `summary` beginning "ComplyEaze Bridge could not confirm which Tally vouchers this post created, so the batch stays open. For each voucher it sent, the book holds a voucher with the same date, voucher type and ledger entries";
     - `verification_status` `verification_incomplete`;
     - `counts.posted_verified` 0;
     - its 50 vouchers `matching_content_observed`.
     The summary's further line beginning "The company's voucher mark moved by a different amount" is EXPECTED only when the build's commit holds #1896's merge (7f7f50e5); record whether it is present and whether the build holds it.
   - For a bound batch, EXPECT `posted_verified`.
   - Then `local_data_report`: EXPECT `journal.state` `read`, `journal.not_settled_binding_refused` 1 and `journal.not_settled` 1 when one batch was refused and every other sent batch bound. Record it whole.

Not reproducing a refusal in three attempts is a result: record "not reproduced in 3 attempts", with every dk, Hk, `before`, `after` and the class. Do not add attempts without the run coordinator's word.

## The book after each step
- Step 0: unchanged.
- Step 1: plus `R-HAND-0`.
- Each attempt: plus the 50 Receipts dated (20 + k) April 2025 when sent, and the Payment `R-HAND-k`.
- After the row: as left by the attempts. Nothing is restored here.

STOP RULES: C7 and C8; any dialog or prompt in Tally; a mark that moves outside a step that changes it; a voucher count other than expected; an approval window other than the attempt's own.

## Close and evidence
Before C10, check that Tally's current date is the one recorded at step 0. Then C10: Shut Company; no company loaded.

**Keep** this row's data folder, THE COMPANY as it now stands, and BK-R. The later #1039 dialog row reads the refused batch from them. The book is restored to BK-R only on the run coordinator's explicit RESTORE (C6), after that row. It has no other fallback: a restore that does not complete is a STOP.

Evidence goes on a new branch `lab/1039-refusal-answers` with `SHA256.txt`, after the pre-push scan and the run coordinator's yes:
- every answer whole, with its size and sha256;
- the before table and each attempt's rows;
- the dry run's SV and SM;
- per attempt: dk, the Yes and save times, `before`, `after`, Hk and the class;
- the `egress_log` lines;
- BK-R's name, size and sha256, and the build's hashes;
- a notes file with UTC times, one line per step.

Screenshots, the screen recording, BK-R and the data folder stay on the lab PC, listed by name, size and sha256.

## Not covered
- A refusal from an edit before a deferred bind. The first bind runs inside the post's own readback, so it is deferred only when that readback fails after the send, and no step here can cause that.
- A second Tally client (Gold).
- Codes other than `span_step_not_created`.
- What a screen save during an import does beyond the marks and the voucher read back.
