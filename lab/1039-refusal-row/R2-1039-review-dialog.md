# R2 (#1039): show R1's refused batch in the content review dialog, and record the review

Not for merge. `LAB-COMMON.md` (v3) and `README.md` in this folder apply in full; where this row and they differ, the stricter holds. Steps marked [OWNER]: see WHO ACTS in `README.md`. Nothing in this row writes to Tally: every call reads the book, and the one write is the review record in ComplyEaze Bridge's own import journal.

**WHY.** #1039's "reviewed, not attributed" record (`acknowledge_post_review` with `doubt` set to `content_only`) may merge only after a live refused batch has been shown in its dialog (AGENTS.md P1). R1 made one and kept it: batch B1 below, refused `span_step_not_created`. This row shows it in that dialog on a build that has the record, declines once, records the review, and reads back what the record changes and what it leaves alone.

## Go-ahead (before step 0)
- This row runs only after R1 (`lab/1039-refusal-answers` at 7f64a199) and before any RESTORE of THE COMPANY or of R1's data folder.
- It runs only after the run coordinator has added a row note to `lab/0.7.0-auto-order/ORDER.md` naming this branch head, and given a recorded go-ahead for it.

## The build
The Windows package of the pull request that adds the content review, at the head the run coordinator names in the row note: its commit, its Bridge CI run, the artifact id, and the sha256 of `bridge_mcp.exe` inside `bridge-tally.mcpb`. Hash the exe before the first call and at the end; any other hash is a STOP. Record the version the tool reports. NOT the build R1 ran on: that build does not know the record and refuses a journal that holds one.

Run it over stdio, ONE launch for the row, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_AGENT_DATA_DIR` = R1's kept folder `C:\bridge-lab\1039-refusal\data\` (never a new one: the batch is read from it), `BRIDGE_TERMS_ACCEPTED=true`, and `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` unset. [OWNER] Posting ON: `BRIDGE_AGENT_ENABLE_WRITES=true` in the launch configuration, set before the launch (the review needs the same opt-in as a post); the lab records at step 0 only that it is present.

## THE BATCH
- B1 = `bridge-a4eb06f8-cf9a-43a6-9255-3920518cf325`, 50 Receipts dated 21 April 2025, posted in R1 attempt 1;
- THE COMPANY = BRIDGE CONCUR LAB, number 100091, GUID `88239cf8-ef69-4cea-8dff-4d1fd91453ee`, as R1 left it.

If either is not as R1 recorded it (the folder missing, B1 absent from its journal, THE COMPANY not on the lab PC): STOP. Never rebuild or repost B1, and never create a substitute.

**THE CALLS.** Only the build's own tools, one at a time. An answer's `next_step` is recorded, never acted on. Paths below are inside `structuredContent.result` of the tool's answer.

**THE REVIEW WINDOW** opens from the exe's own process and may open behind other windows (look for it on the taskbar). While it is open nobody presses a key on the PC and no tool is called. [OWNER] answers it by mouse, as the step says.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay). Record the process list, Tally's current date, and that `BRIDGE_AGENT_ENABLE_WRITES` is present. Hash the exe.
2. Copy R1's data folder whole, before the launch, into a NEW empty folder `C:\bridge-lab\1039-review\data-before\`. Record the name, size and sha256 of every file in both folders; the two lists must be equal, else STOP. Record the sha256 and line count of the journal `agent-import-ledger.jsonl` as J0 and N0. The copy is the way back to the folder R1 left, which R1's build can still read; this row's review record makes R1's build refuse the folder's journal.
3. `list_companies`: exactly THE COMPANY. MARKS: M0, V0. EXPECT M0 210 and V0 55, R1's last MARKS (its answer 012); other values: record them and STOP, since the book is then not as R1 left it.
4. `vouchers` for 21 April 2025: EXPECT 50 rows narrated `R-1-01` to `R-1-50`; anything else = STOP.

The book: as R1 left it. The data folder: as R1 left it.

## Step 1: before the review
1. `verify_import` with `company_guid` and `batch_id` B1. Record it whole. EXPECT:
   - `post_span_binding.state` `refused`, `code` `span_step_not_created`;
   - `verification_status` `verification_incomplete`, `counts.posted_verified` 0, `counts.matching_content_observed` 50;
   - no `content_review` key.
   Anything else = STOP, a FINDING (the row's input is not R1's refused batch).
2. `local_data_report`. Record it whole. EXPECT `journal.state` `read`, `journal.not_settled` 1, `journal.not_settled_binding_refused` 1, `journal.not_settled_reviewed_not_attributed` 0.
3. MARKS: EXPECT 210 and 55.

## Step 2: the dialog, declined
1. `acknowledge_post_review` with `company_guid`, `batch_id` B1 and `doubt` `content_only`.
2. When the review window opens, [OWNER] screenshots it whole before any click, and reads it. EXPECT the title "ComplyEaze Bridge — record that you reviewed these 50 vouchers?" and exactly these lines, as the box wraps them at 100 characters (B1 standing for the batch id):

   ```text
   Press Y (or click Yes) to record that you reviewed these 50 vouchers. Press N (or click No) to
   cancel. Nothing is recorded until you answer.

   THIS RECORDS ONLY THAT YOU CHECKED THESE VOUCHERS IN TALLY.
   It changes nothing in Tally, does not mark the batch posted, and does not let its rows be posted
   again.

   Company: BRIDGE CONCUR LAB
   Batch: B1
   Sent to Tally: 50 vouchers (50 Receipt), dated 20250421.
   ComplyEaze Bridge cannot tell which Tally vouchers this post created: its match of the post to the
   Tally vouchers it created was refused (span_step_not_created).
   The read just now found, for 50 of the 50 vouchers sent, a voucher in the book with the same date,
   voucher type and ledger entries.

   Confirm only if you have checked these vouchers in Tally yourself.
   ```

   Copy the window's text exactly into the notes. A window that is cut or scrolls, or a line that differs from the block above: record it whole, then [OWNER] clicks **No**, and STOP after step 2.4: a FINDING.
3. [OWNER] clicks **No**.
4. EXPECT the answer's `error.code` `ack_review_declined`. Record it whole. Then read the journal line by line, each line parsed as JSON (never a text search): EXPECT no line with `record_type` `content_review`, and every line after the first N0 a `verification_status` record of B1 (the reads of steps 1.1 and 2.1 write these). Anything else = STOP.

## Step 3: the dialog, approved
1. `acknowledge_post_review` again, the same arguments.
2. [OWNER] screenshots the window whole, checks its text is the block of step 2.2 (a difference: record it, click **No**, STOP), and clicks **Yes**.
3. EXPECT, recorded whole:
   - `state` `content_reviewed`, `reviewed_binding` `refused`, `verification_status` `verification_incomplete`;
   - `next_step` "A person recorded that they checked this batch's vouchers in Tally. ComplyEaze Bridge still cannot tell which Tally vouchers this post created: the batch is not marked posted and stays open, and its rows are not posted again."
   Any refusal: record it whole and STOP (`ack_changed_while_reviewing` means another call or a person changed something while the window was open; nothing may have).
4. Record the journal's sha256 as J1 and its line count. Parse every line as JSON: EXPECT exactly one with `record_type` `content_review`, and it is the LAST line. Copy that line whole into the notes: EXPECT one JSON object with `record_type` `content_review`, `batch_id` B1, `status` `content_reviewed`, `reviewed_binding` `refused`, and `batch_sha256` equal to `verify_import`'s `batch_sha256` from step 1.1, and no other keys.

## Step 4: after the review
1. `verify_import` B1. Record it whole. EXPECT:
   - `content_review` = `state` `recorded`, `reviewed_binding` `refused`, and the `next_step` of step 3.3;
   - everything step 1.1 expects otherwise, unchanged: the same `post_span_binding`, `verification_status` `verification_incomplete`, `counts.posted_verified` 0.
2. `local_data_report`. EXPECT `journal.not_settled` 1, `journal.not_settled_binding_refused` 0, `journal.not_settled_reviewed_not_attributed` 1, `journal.not_settled_no_response` 0, `journal.not_settled_not_verified` 0.
3. `acknowledge_post_review` a third time, the same arguments. EXPECT `error.code` `ack_already_recorded` and NO window; a window = [OWNER] clicks **No**, STOP, a FINDING.
4. MARKS: EXPECT 210 and 55. `vouchers` for 21 April 2025: the same 50 rows as step 0.4, each with its AlterID unchanged.
5. `egress_log` for this row's calls: record it. EXPECT no import request (no `IMPORTDATA` envelope) sent in this row.
6. Hash the exe again.

The book: unchanged by the row. The data folder: plus the one review record, and the proofs and status records the reads wrote.

STOP RULES: C7 and C8; any dialog or prompt in Tally; a mark that moves; a voucher count or AlterID other than expected; a review window other than the call's own.

## Close and evidence
Before C10, check that Tally's current date is the one recorded at step 0. Then C10: Shut Company; no company loaded.

**Keep** the data folder as this row left it (holding the review), the copy `data-before`, THE COMPANY as it stands, and R1's BK-R. RESTORE of the book to BK-R, and of the data folder from `data-before`, only on the run coordinator's word.

Evidence goes on a new branch `lab/1039-review-dialog-answers` with `SHA256.txt`, after the pre-push scan and the run coordinator's yes:
- every answer whole, with its size and sha256;
- the window's text as copied at steps 2.2 and 3.2;
- the journal's last line (step 3.4), J0, N0, J1 and the line counts;
- the two folder listings of step 0.2;
- the `egress_log` lines;
- the build's hashes;
- a notes file with UTC times, one line per step.

Screenshots, both data folders and BK-R stay on the lab PC, listed by name, size and sha256.

## Not covered
- A review of a batch whose answer from Tally was lost (`not_bound`, `binding_response_not_recorded`): the same record and dialog, with another reason line; no such batch is kept on the lab PC.
- A review voided afterwards (a later `posted_verified`, answer or rollback).
- A Mac host, whose box names its buttons rather than Y and N.
