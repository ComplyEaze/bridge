# S1 (#1709): settle a batch Tally answered as creating none of its vouchers, in the settle dialog

Not for merge. `LAB-COMMON.md` (v3, the byte copy in this folder, sha256 in `SHA256.txt`) applies in full; where this row and it differ, the stricter holds. Steps marked [OWNER] are the lab PC operator's. This row writes to Tally twice (steps 2.1 and 2.2, both through the build's own `post_import`) and changes one voucher type's numbering setting (step 1), which step 7 restores. Its one other write is the settlement record in ComplyEaze Bridge's own import journal.

**WHY.** The person settlement of #1709 (`acknowledge_post_review` with `doubt` set to `not_created`, pull request #2150) may merge only after a live batch that Tally answered as creating none of its vouchers has been settled in its dialog (AGENTS.md P1). No such batch is kept, so this row makes one through the build's own post, the one way the protocol reference records for a native post (§9.8): a Journal carrying a voucher number that a Journal already holds, into a voucher type numbered Manual with duplicate numbers prevented, which Tally refuses whole with `CREATED` 0 and `EXCEPTIONS` 1. It then shows that batch in the settle dialog, declines once, settles it, and reads back what the record changes and what it leaves alone.

## Go-ahead (before step 0)
- It runs only after the run coordinator has added a row note to `lab/0.7.0-auto-order/ORDER.md` (on the branch `lab/0.7.0-auto-order`, not this one) naming this branch head, and given a recorded go-ahead for it.

## The build
The Windows package of #2150 at the head the run coordinator names in the row note: its commit, its Bridge CI run, the artifact id (`unsigned-bundle-smoke-windows-latest`), the sha256 of `bridge-tally.mcpb`, and inside it the sha256 of `bin/x86_64-pc-windows-msvc/bridge_mcp.exe`, the `binary_sha256` of the artifact's `mcpb-smoke.json`. The bundle is downloaded only after the owner's OK to download #2150's artifact, which the row note records with the run and artifact ids. A later head of #2150 that changes no production file does not replace it. Hash the exe before the first call and at the end; any other hash is a STOP. Record the version the tool reports.

Run it over stdio, ONE launch for the row, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_AGENT_DATA_DIR` = a NEW empty folder `C:\bridge-lab\1709-settle\data\`, `BRIDGE_TERMS_ACCEPTED=true`, and `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` unset. [OWNER] Posting ON: `BRIDGE_AGENT_ENABLE_WRITES=true` in the launch configuration, set before the launch; the lab records at step 0 only that it is present. A new folder: an older build refuses a journal that holds this row's record, so no folder another build still reads is used.

## THE COMPANY
A synthetic lab company (BRIDGE PILOT LAB or BRIDGE SHAPE LAB; never REGISTER LAB), loaded alone, with its own Journal voucher type and two ledgers that take a balanced Journal (L1 and L2 below), all named in the row note. THE TYPE = that company's voucher type `Journal`.

**THE CALLS.** Only the build's own tools, one at a time. An answer's `next_step` is recorded, never acted on. Paths below are inside `structuredContent.result` of the tool's answer. Amounts are small invented values, as written here.

**THE WINDOWS** (the post's approval and the settle dialog) open from the exe's own process and may open behind other windows. If it is hidden, [OWNER] reaches it through its taskbar button ONCE, and records what happened (a box that ends on that click: STOP). While one is open nobody presses a key on the PC and no tool is called. [OWNER] answers each by mouse, as the step says. Any window this sheet does not name, at any step, gets No by mouse and is a STOP.

**POST ANSWERS (steps 2.1 and 2.2).** The post's call waits for the approval about 40 seconds. If its answer reads `approval.state` `pending`, call `post_import` again with the same batch id after `retry_after_s`: that redeems the same approval and is not a re-send. If an answer's `next_step` says to call again at once, call once more. Record each answer whole. The batch's dispatch answer is the one that carries `dispatch`.

**AFTER A STOP from step 1.2 on**, in this order, pre-authorised here as the one exception to LAB-COMMON C8's no-keying rule: any open window gets No by mouse; then step 7.1 (THE TYPE's numbering restored by hand, with its screenshot); then nothing more until the run coordinator's word. B0's voucher stays in the book. The backup from step 0.2 is restored only on the run coordinator's word.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone. Record the process list, Tally's current date, and that `BRIDGE_AGENT_ENABLE_WRITES` is present. Hash the exe.
2. LAB-COMMON's backup of THE COMPANY before any change; record the backup's name, size and sha256.
3. `list_companies`: exactly THE COMPANY. MARKS: M0, V0.

## Step 1: THE TYPE numbered Manual, duplicates prevented
1. [OWNER] Record THE TYPE's numbering screen as it is (method, and the duplicate setting): screenshot.
2. [OWNER] Set THE TYPE's method of voucher numbering to Manual with duplicate numbers prevented; save. Screenshot it.
3. MARKS: M1, V1. Record them (a voucher type save moves the master mark).

## Step 2: make the not-created batch
1. `build_import_xml` one Journal dated within THE COMPANY's books: `voucher_number` `S1709-1`, L1 debit 11.00, L2 credit 11.00, narration `S1709 first`. Then `post_import` with its batch id (B0). [OWNER] approves the post window. EXPECT `dispatch.state` `posted_verified` and the voucher found with number `S1709-1`; anything else = STOP.
2. `build_import_xml` one Journal on the same date: `voucher_number` `S1709-1` again, L1 debit 13.00, L2 credit 13.00, narration `S1709 second`. Then `post_import` with its batch id (B1). [OWNER] approves the post window. EXPECT:
   - `dispatch.counters` `created` 0 and `exceptions` 1, every other counter 0;
   - the voucher's status `tally_reported_not_created`, its `next_step` naming `acknowledge_post_review` with doubt `not_created`;
   - `post_span_binding.summary` "Tally reported that this post created none of its vouchers: follow each voucher's next step."
   Anything else = STOP, a FINDING (record it whole; the build's preflight refusing B1 before the send is a FINDING too, not a pass). If B1's answer shows `created` 1 (a second voucher numbered `S1709-1` in the book): STOP, record it whole, attempt no settlement, and leave that voucher for the run coordinator, whose option is the backup from step 0.2. Once B1 has its dispatch answer, never call `post_import` for B1 again, and never build a substitute.
3. MARKS: M2, V2. EXPECT V2 = V1 + 1 (B0 only).

## Step 3: before the settlement
1. `verify_import` with `company_guid` and `batch_id` B1. Record it whole. EXPECT `verification_status` not `posted_verified`, the voucher `sent_not_attributed` with a `next_step` naming `acknowledge_post_review` with doubt `not_created`, `post_span_binding.state` `refused` or `not_bound`, and no `person_settlement` key.
2. `local_data_report`. Record it whole. EXPECT `journal.not_settled` 1 and `journal.settled_by_person` 0.
3. Record the sha256 and line count of the journal `agent-import-ledger.jsonl` as J3 and N3.

## Step 4: the dialog, declined
1. `acknowledge_post_review` with `company_guid`, `batch_id` B1 and `doubt` `not_created`.
2. When the window opens, [OWNER] screenshots it whole before any click, and reads it. EXPECT the title "ComplyEaze Bridge — close this batch for good?" and exactly these lines, as the box wraps them at 100 characters (COMPANY, B1 and DATE standing for THE COMPANY's name, the batch id and the voucher's date as the journal holds it; a long company name may wrap):

   ```text
   Press Y (or click Yes) to close this batch in ComplyEaze Bridge for good, so that it never sends
   this voucher again. Press N (or click No) to cancel. Nothing is closed until you answer.

   THIS CLOSES THE BATCH. COMPLYEAZE BRIDGE NEVER SENDS THIS VOUCHER AGAIN.
   It changes nothing in Tally and does not mark the batch posted. Its rows stay held: they cannot be
   built or posted again.

   Company: COMPANY
   Batch: B1
   Sent to Tally: 1 voucher (1 Journal), dated DATE.
   When it was sent, Tally answered that it did not create it.
   The read just now found, for 0 of the 1 voucher sent, a voucher in the book with the same date,
   voucher type and ledger entries.

   Confirm only if you have checked in Tally that the voucher is in the book, or you have decided it
   will not be posted.
   ```

   No is the default button. Any other title or text = STOP, a FINDING: [OWNER] clicks No and the row ends.
3. [OWNER] clicks No. EXPECT `error.code` `ack_review_declined`. The journal: EXPECT no `person_settlement` line, and its lines past N3 only status records of B1.

## Step 5: the settlement
1. `acknowledge_post_review` again with the same arguments. The same window, with the same text as step 4.2: else STOP.
2. [OWNER] clicks Yes. EXPECT `state` `settled_by_person`, `batch_id` B1, `verification_status` as step 3.1 read it.
3. `acknowledge_post_review` once more: EXPECT `error.code` `ack_already_recorded` and no window (a window that opens gets No by mouse and is a STOP).

## Step 6: what it changed and what it left alone
1. `verify_import` with B1. Record it whole. EXPECT `person_settlement.state` `settled`; `post_span_binding.summary` "A person settled this batch after Tally reported that its post created none of its vouchers: ComplyEaze Bridge never sends them again."; the voucher and `verification_status` as step 3.1.
2. `local_data_report`. EXPECT `journal.not_settled` 0 and `journal.settled_by_person` 1.
3. The journal's last `person_settlement` line: EXPECT `record_type` `person_settlement`, `status` `settled_by_person`, `batch_id` B1 and `batch_sha256` B1's.
4. `build_import_xml` the same Journal as step 2.2 under B1's `bridge_txn_id`: EXPECT it refused `import_txn_already_posted` naming B1 (the row stays held), and no file written.
5. MARKS: EXPECT M2 and V2 (steps 3 to 6 sent nothing to Tally).
6. `vouchers` for the date of step 2: EXPECT exactly one voucher numbered `S1709-1`, B0's.

## Step 7: restore
1. [OWNER] Set THE TYPE's numbering back to what step 1.1 recorded; save; screenshot.
2. B0's voucher stays in the book (a synthetic company); the row note says so. No master or company is deleted.
3. Hash the exe. Record the data folder's file list with sizes and sha256.

## Evidence
Every answer whole under `lab/1709-settle-answers/answers/`, numbered in call order, with the screenshots, the hashes and a `SHA256.txt` of every file; `RESULTS.md` says, per EXPECT line, PASS or what was seen instead, with a confidence marker.
