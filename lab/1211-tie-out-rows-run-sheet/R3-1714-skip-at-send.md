# #1714 row: a skip recorded after a statement batch is built refuses that batch when it is sent

Not for merge. BLOCKING for release 0.6.1 (the owner's decision of 26 Oct 2026: this row and the #724 numbering rows are the live write pass the release owes). `LAB-COMMON.md` in this folder (v3) applies in full; where this row and it differ, the stricter holds. It reuses the setup of `RUN-SHEET.md` in this folder (R1 and R2): the same company, statement and parse arguments.

**WHY.** Pull request #1714 checks a bank-statement batch against the transfer records when it is sent, as well as when it is built. Its own lab request: "a statement batch built, then a skip of its row recorded, then sent: refused before the dialog with nothing sent; a batch whose row is not skipped posts as before". Its end-to-end test (`a_row_a_statement_skipped_after_the_build_is_refused_before_the_dialog` in `src-tauri/src/agent_import_post_e2e_tests.rs`) refuses such a batch with `skip_row_paired_elsewhere`, before any Tally request and before the approval window, with nothing written to the batch journal. It has run only against a simulator; this row runs it on a real book.

**THE COMPANY** is BRIDGE TIE OUT LAB 3, as `RUN-SHEET.md` describes it (GUID `9c97970c-e25d-4348-9740-6ddb1caaa56d`, books from 1 April 2026, bank ledger Union Bank, July and August imported, nothing on Union Bank in September). If THE COMPANY is not on the lab PC: STOP. Never create a substitute.

**THE STATEMENT** is the synthetic three-month PDF of the 14 October run, as `RUN-SHEET.md` names it (sha256 beginning `506e05eb`, ending `4fc9`). Check its full hash against that run's notes before every parse; a different file = STOP.

**THE BUILD (the 0.6.1 cut build).** The Windows bundle `unsigned-bundle-smoke-windows-latest` of the Bridge CI run on master that holds the last change merged for 0.6.1, checked as `lab/724-numbering-rows` says (the zip against the artifact's digest, the `.mcpb` against `archive_sha256` and `bridge_mcp.exe` against `binary_sha256` in its `mcpb-smoke.json`). The run coordinator fills the line below by a new commit on this branch; while any field reads `<to fill>`, this row does not run: STOP. Hash the exe at the start and at the end; any other hash = STOP. Record the version the tool reports.

RC COMMIT: <to fill>  CI RUN: <to fill>  ARTIFACT: <to fill>  ZIP DIGEST: <to fill>  EXE SHA256: <to fill>

Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` for this row (`C:\bridge-lab\1714\data\`, never the folder R1 and R2 used), `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use) and `BRIDGE_AGENT_REDACTION` unset. POSTING is the owner's act: the owner sets `BRIDGE_AGENT_ENABLE_WRITES=true` in the launch configuration himself; the lab never sets, writes or copies it, and records only that it is present. `BRIDGE_AGENT_ENABLE_BATCH_POST` must be absent.

**THE APPROVAL WINDOW** opens on top and takes the keyboard focus: only the owner answers it, by mouse; nobody presses a key on the PC from a `post_import` call until its answer arrives. This row runs only while the owner is at the PC.

**THE CALLS.** Only the build's own tools, one at a time; no request is sent by curl. `parse_bank_statement` takes the required arguments the 14 October run used for THE STATEMENT (from that run's call records on the lab PC), changing only `from`, `to` and `mapping` as each step says. Every `build_import_xml` passes `company_guid` with that parse's `proposals_id` and `proposals_sha256`. MARKS means `ledger_masters` (first page, no `snapshot_id`): M = `snapshot.master_alter_id`, V = `snapshot.voucher_alter_id`; `snapshot.reused` must read false.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay).
2. `list_companies`: EXPECT exactly THE COMPANY with the GUID above; anything else = STOP.
3. MARKS: M0, V0.
4. `ledger_movement` for Union Bank, 2026-07-01 to 2026-09-30: EXPECT opening -10000.00 and closing -11800, as on 14 October; another figure = STOP (the book is not as recorded).
5. `masters` with `kind` `voucher_types`: record the Payment and Receipt rows whole. A statement row is built as a Payment or a Receipt, and a build refuses a type whose numbering is not Automatic with "Retain Original Voucher No." (`import_voucher_numbering_not_automatic`, `import_voucher_numbering_not_retain`). If either row reads otherwise: STOP and ask the run coordinator (setting it is an owner act on Tally's screen, after BK1).
6. Backup BK1 (C5) into a NEW empty folder `C:\bridge-lab\1714\backups\BK1\`: name, size and sha256.

## Step 1: a second bank ledger (by hand)
OWNER: create ledger `BRIDGE Bank B` under Bank Accounts, no opening balance; screenshot before the accept. MARKS: M1 (EXPECT above M0), V1 = V0.

## Step 2: choose two rows (the statement file only; nothing is posted)
1. `parse_bank_statement` with `from` 2026-09-01 and `to` 2026-09-30, no `mapping`. Save the answer.
2. From it, choose the first two days of September, D and then D2, on each of which the statement holds exactly one row, whose party is named (not unresolved or unnamed) and is not cash. Record each day's party exactly as `counterparties` prints it (P for D).
3. Confirm each with a one-day parse (`from` = `to` = that day, no `mapping`): EXPECT `rows_in_window` 1 and `cash_questions_open` 0. Fewer than two such days = STOP and report the September rows (the run coordinator decides).

## Step 3: batch A, built for D (not sent)
`parse_bank_statement` for D, no `mapping` (the row goes to the suspense ledger), then `build_import_xml` with that parse. EXPECT a saved batch with one voucher: record the answer whole and its `batch_id` as A. Any refusal: record it whole and STOP.
The book: unchanged.

## Step 4: the skip of that row, recorded after A was built
`parse_bank_statement` for D with `mapping` `[{"party": "<P>", "ledger": "BRIDGE Bank B", "treatment": "skip"}]`: EXPECT no voucher proposed and `skipped` 1. Then `build_import_xml` with that parse: EXPECT `state` `skips_recorded`, `skipped_rows` 1, a `record_id`, and a `transfers` section; record the answer whole, and the row's id R (`st-` and D's date) as the parse answer or the transfers section names it. Any refusal: record it whole and STOP (the row cannot go on as written; nothing was sent).
The book: unchanged (a skip is recorded in the data folder only).

## Step 5: batch A sent (the check)
1. Record the name, size and sha256 of every file in the data folder. MARKS: V5a.
2. Tell the owner that no approval window is expected. `post_import` with `company_guid` and `batch_id` A.
3. EXPECT: no approval window; the answer's `error.code` `skip_row_paired_elsewhere`, `error.skipped_rows` holding R, and `error.next_step` beginning "Nothing was sent to Tally, and any approval of this batch is withdrawn." and ending "This batch cannot be sent as built: build it again."; record `attempt_recorded` (EXPECT false). Record the answer whole.
4. MARKS: V = V5a. The data folder's batch journal: same size and sha256 as in 1. `egress_log`: record the lines this call added (EXPECT no request sent to Tally for it). `vouchers` for D: EXPECT no voucher on Union Bank that day.
If an approval window opens: the owner answers **No** by mouse, screenshot, record the answer whole, STOP: that is the FINDING of this row. Any other code, a moved V, or a voucher on D: record it whole and STOP, and report what the book holds.
The book: unchanged.

## Step 6: the control, a row not skipped posts as before
`parse_bank_statement` for D2, no `mapping`, then `build_import_xml` with that parse: record its `batch_id` as B. Tell the owner a window will open. `post_import` with `company_guid` and `batch_id` B; the owner reads the window and answers **Yes** by mouse within its time. If the answer says the approval is `pending`, or `approved` without a send, call `post_import` again with B after its `retry_after_s` (the tool's own flow, not a retry). Record every answer whole.
EXPECT `state` `posted_verified`. `verify_import` of B: `posted_verified`. MARKS: V moved by one. `vouchers` for D2: one voucher on Union Bank, as the statement row says. Any other final state or refusal: record it whole, do not call again, STOP.
The book: plus one voucher dated D2.

STOP RULES: C7 and C8; any dialog before a call; an approval window other than step 6's; a mark that moves outside the steps that change it; any answer outside the stated shape.

## Restore (only on the run coordinator's explicit RESTORE)
Restore BK1 (C6). `list_companies` (as step 0); MARKS: EXPECT M = M0 and V = V0; `ledger_movement` as step 0: equal to step 0's; `BRIDGE Bank B` absent. Else STOP, and report what the book holds. Batch A stays held in this row's data folder (nothing clears an unsent batch); the folder is kept on the lab PC and never used again.

## The book after each step
Step 0: unchanged. Step 1: plus `BRIDGE Bank B`. Steps 2 to 5: the same. Step 6: plus the D2 voucher. After the restore: as at BK1, the book of step 0.

## Not measured by this row
A skip recorded while the approval window is open (the check under the admission lock); a batch that withdrew the skip and was then overtaken by a newer skip; Contra batches; `skip_row_already_posted`; Gold.

## Close and evidence
C10: Shut Company; no company loaded. Evidence on a new branch `lab/1714-skip-at-send-answers` with `SHA256.txt`, after the pre-push scan and the run coordinator's yes: every answer whole with its size and sha256, the data folder's file listing of step 5 (names, sizes, sha256), the `egress_log` lines, BK1's name, size and sha256, the build's hashes, and a notes file with UTC times. Screenshots, the statement PDF and the data folder stay on the lab PC, listed by name, size and sha256.
