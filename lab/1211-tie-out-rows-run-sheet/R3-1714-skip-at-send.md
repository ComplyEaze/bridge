# #1714 row: a skip recorded after a statement batch is built refuses that batch when it is sent

Not for merge. BLOCKING for release 0.6.1 (the owner's decision of 26 Oct 2026: this row and the #724 numbering rows are the live write pass the release owes). `LAB-COMMON.md` in this folder (v3) applies in full; where this row and it differ, the stricter holds. It reuses the setup of `RUN-SHEET.md` in this folder (R1 and R2): the same company, statement and parse arguments, and its "Before the sitting" items 1 to 3 (licensed Tally, `SHA256.txt` checked, the owner's go-ahead).

**WHY.** Pull request #1714 checks a bank-statement batch against the transfer records when it is sent, as well as when it is built. Its own lab request: "a statement batch built, then a skip of its row recorded, then sent: refused before the dialog with nothing sent; a batch whose row is not skipped posts as before". Its end-to-end test (`a_row_a_statement_skipped_after_the_build_is_refused_before_the_dialog` in `src-tauri/src/agent_import_post_e2e_tests.rs`) refuses such a batch with `skip_row_paired_elsewhere`, before any Tally request and before the approval window, with nothing written to the batch journal. It has run only against a simulator; this row runs it on a real book.

**THE COMPANY** is BRIDGE TIE OUT LAB 3, as `RUN-SHEET.md` describes it (GUID `9c97970c-e25d-4348-9740-6ddb1caaa56d`, books from 1 April 2026, bank ledger Union Bank, July and August imported, nothing on Union Bank in September). If THE COMPANY is not on the lab PC: STOP. Never create a substitute.

**THE STATEMENT** is the synthetic three-month PDF of the 14 October run, as `RUN-SHEET.md` names it (sha256 beginning `506e05eb`, ending `4fc9`). Check its full hash against that run's notes before every parse; a different file = STOP.

**THE BUILD (the 0.6.1 cut build).** The Windows bundle `unsigned-bundle-smoke-windows-latest` of the Bridge CI run on master that holds the last change merged for 0.6.1, checked as `lab/724-numbering-rows` says (the zip against the artifact's digest, the `.mcpb` against `archive_sha256` and `bridge_mcp.exe` against `binary_sha256` in its `mcpb-smoke.json`). The run coordinator fills the line below by a new commit on this branch; while any field reads `<to fill>`, this row does not run: STOP. Hash the exe at the start and at the end; any other hash = STOP. Record the version the tool reports.

RC COMMIT: 67f647e1a292e8d7815053b5c08289cba6402795  CI RUN: 40456677783  ARTIFACT: 12712717489  ZIP DIGEST: sha256:614a570320a447d78e92d97aeff93786641b9e9bbf918f10e1a440668e329a04  EXE SHA256: 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d

BUILD NOTES: CI run 40456677783 is the merge-queue run on this commit that built the bundles; the push run 40457915071 on the same commit reused them by design. The `.mcpb` archive_sha256 is c294e2cb7545356bcddfbaadc0a1d5e940f95ffb65e2e4babb04a2ffc829747e. The tool list holds 31 tools by default and 33 with posting on. The bundle reports version 0.6.0 inside, because it was built before the version commit: expected for this cut, not a STOP. The lab hashes the installers from the artifact and records them.

Run it over stdio, ONE launch from step 0 to the close (never relaunched in between), with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` for this row (`C:\bridge-lab\1714\data\`, never the folder R1 and R2 used), `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), and `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` unset. POSTING is the owner's act: the owner sets `BRIDGE_AGENT_ENABLE_WRITES=true` in the launch configuration himself, before that launch (it also lets the parses and builds of steps 2 to 4 run); the lab never sets, writes or copies it, and records at step 0 only that it is present. `BRIDGE_AGENT_ENABLE_BATCH_POST` must be absent. No other ComplyEaze Bridge, the desktop app included, may run against port 9001 during the row: record the process list at step 0.

**THE APPROVAL WINDOW** opens on top and takes the keyboard focus. While any approval window is open, nobody presses a key on the PC and no tool is called, except step 6's own repeat `post_import` with B; only the owner's mouse closes it. This row runs only while the owner is at the PC.

**THE CALLS.** Only the build's own tools, one at a time; no request is sent by curl. `parse_bank_statement` takes the required arguments the 14 October run used for THE STATEMENT (from that run's call records on the lab PC), changing only `from`, `to` and `mapping` as each step says. Every `build_import_xml` passes `company_guid`, `proposals_id` = that parse's `proposals_id`, and `proposals_sha256` = that parse's `sha256` (the parse answer's field is `sha256`). MARKS means `ledger_masters` (first page, no `snapshot_id`): M = `snapshot.master_alter_id`, V = `snapshot.voucher_alter_id`; `snapshot.reused` must read false. An answer's `next_step` or `next_steps` is recorded, never acted on: this row's steps decide every call.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay). Record the process list and that `BRIDGE_AGENT_ENABLE_WRITES` is present.
2. `list_companies`: EXPECT exactly THE COMPANY with the GUID above; anything else = STOP.
3. MARKS: M0, V0. EXPECT V0 a number; null = STOP (a post whose last read omits the voucher mark is refused before anything is sent, which would stop step 6 for a reason unrelated to #1714).
4. `ledger_movement` for Union Bank, 2026-07-01 to 2026-09-30: EXPECT opening -10000.00 and closing -11800, as on 14 October; another figure = STOP (the book is not as recorded).
5. `masters` with `kind` `voucher_types`: record the Payment and Receipt rows whole. A statement row is built as a Payment or a Receipt, and a build refuses a type whose numbering is not Automatic with "Retain Original Voucher No." EXPECT, for each of the two, `series_numbering.observation` `observed`, `series_numbering.method` `automatic` and `series_numbering.sub_method` `auto_retain` (`numbering_method` does not decide this: ignore it). If either reads otherwise: STOP before anything changes and ask the run coordinator. Should he go on, the order is: BK1 (step 6), the owner sets that type's numbering on Tally's screen, MARKS, then step 1.
6. Backup BK1 (C5) into a NEW empty folder `C:\bridge-lab\1714\backups\BK1\`: name, size and sha256.

## Step 1: a second bank ledger (by hand)
OWNER: create ledger `BRIDGE Bank B` under Bank Accounts, no opening balance; screenshot before the accept. MARKS: M1 (EXPECT above M0), V1 = V0.

## Step 2: choose two rows (the statement file only; nothing is written to Tally)
First parse 2026-09-01 to 2026-09-30 (no `mapping`) and save the answer: EXPECT `rows_in_window` 2 and `cash_questions_open` 0 (`results/p-sep.json` on `lab/1628-m-month-window-tie-out-answers`, 14 October, the same statement: two rows, both named parties); another count = STOP. Then parse one day at a time from 2026-09-01 (`from` = `to` = that day, no `mapping`), saving each answer. A day with no statement row is refused with the tool error `statement_empty_selection`: record it; that day does not qualify, and it is not a STOP. A day qualifies when the answer reads `rows_in_window` 1, `cash_questions_open` 0, `skipped` 0, and its one `counterparties` entry is a named party (not unresolved or unnamed); a day reading `rows_in_window` 2 does not qualify. D and D2 are the first two days that qualify; record each day's party exactly as `counterparties` prints it (P for D). Stop parsing once D2 is found. Fewer than two such days in September = STOP and report the answers (the run coordinator decides).

## Step 3: batch A, built for D (not sent)
`parse_bank_statement` for D, no `mapping` (the row goes to the suspense ledger), then `build_import_xml` with that parse. EXPECT `voucher_count` 1 and a `batch_id`: record the answer whole, and that `batch_id` as A. Any refusal: record it whole and STOP.
The book: unchanged.

## Step 4: the skip of that row, recorded after A was built
`parse_bank_statement` for D with `mapping` `[{"party": "<P>", "ledger": "BRIDGE Bank B", "treatment": "skip"}]`: EXPECT `vouchers` 0 and `skipped` 1. Then `build_import_xml` with that parse. EXPECT `state` `skips_recorded`, `skipped_rows` 1 and a `record_id`, and in its `transfers` section: `state` `computed`, `skips_open` 1, `open_omitted` 0, and one `open` row with `state` `held_elsewhere` and `batch_id` A. R is that row's `bridge_txn_id` (it begins `st-` and D as yyyymmdd). Record the answer whole. Any refusal, another state, or the section cut short: record it whole and STOP (nothing was written to Tally).
The book: unchanged (the skip is recorded in the data folder only).

## Step 5: batch A sent (the check)
1. Record the name, size and sha256 of every file in the data folder, among them `agent-import-ledger.jsonl` (the batch journal) and `agent-import-statement-skips.jsonl` (the skips). MARKS: V5a.
2. Tell the owner that no approval window is expected. `post_import` with `company_guid` and `batch_id` A.
3. EXPECT: no approval window; the answer's `result.error.code` `skip_row_paired_elsewhere`, `result.error.skipped_rows` holding R, `result.error.next_step` beginning "Nothing was sent to Tally, and any approval of this batch is withdrawn." and ending "This batch cannot be sent as built: build it again.", and `result.attempt_recorded` false. Record the answer whole. `result.attempt_recorded` null = STOP (another process may hold the dispatch lease).
4. MARKS: V = V5a. Both files of 1: the same size and sha256. `egress_log`: record the lines this call added; EXPECT its `post_import` receipt (record type `response_prepared`) to carry the code `skip_row_paired_elsewhere` and NO `request_trail` key (the key is written only when a call sent something; step 6 is its positive control). `vouchers` for D: EXPECT no voucher on Union Bank that day.
If an approval window opens: the owner answers **No** by mouse, screenshot, record the answer whole, STOP: that is the FINDING of this row. Any other code, a moved V, a change in either of those two files (`agent-egress.jsonl` grows on every call and is not one of them), or a voucher on D: record it whole and STOP, and report what the book holds.
The book: unchanged.

## Step 6: the control, a row not skipped posts as before
`parse_bank_statement` for D2, no `mapping`, then `build_import_xml` with that parse: EXPECT `voucher_count` 1; record its `batch_id` as B. MARKS: V6a, immediately before the post. Tell the owner a window will open. `post_import` with `company_guid` and `batch_id` B; the owner reads the window and answers **Yes** by mouse within its time. If `result.approval.state` reads `pending`, wait `result.approval.retry_after_s` seconds and call `post_import` again with B; if it reads `approved` without a send, call again at once (the tool's own flow, not a retry). Record every answer whole.
EXPECT the last answer's `result.dispatch.state` `posted_verified`. `verify_import` of B: `result.verification_status` `posted_verified`. MARKS: V = V6a + 1. `egress_log`: EXPECT the `post_import` receipt (record type `response_prepared`) of the sending call to carry `request_trail.sent` of 1 or more. `vouchers` for D2: one voucher on Union Bank, as the statement row says. Any other final state or refusal: record it whole, do not call again, STOP. A `voucher_mark_contradiction` at step 6 is recorded and is a STOP unrelated to #1714.
The book: plus one voucher dated D2.

STOP RULES: C7 and C8; any dialog before a call; an approval window other than step 6's; a mark that moves outside the steps that change it; any answer outside the stated shape.

## Restore (only on the run coordinator's explicit RESTORE)
Restore BK1 (C6). `list_companies` (as step 0); MARKS: EXPECT M = M0 and V = V0; `ledger_movement` as step 0: equal to step 0's; `ledger_masters` (every page): no row named `BRIDGE Bank B`. Else STOP, and report what the book holds. Batch A stays held in this row's data folder (nothing clears an unsent batch); the folder is kept on the lab PC and never used again.

## The book after each step
Step 0: unchanged. Step 1: plus `BRIDGE Bank B`. Steps 2 to 5: the same. Step 6: plus the D2 voucher. After the restore: as at BK1, the book of step 0.

## Not measured by this row
A skip recorded while the approval window is open (the check under the admission lock); a batch that withdrew the skip and was then overtaken by a newer skip; Contra batches; `skip_row_already_posted`; a hand import of batch A's file; Gold.

## Close and evidence
C10: Shut Company; no company loaded. Evidence on a new branch `lab/1714-skip-at-send-answers` with `SHA256.txt`, after the pre-push scan and the run coordinator's yes: every answer whole with its size and sha256, the data folder's file listing of step 5 (names, sizes, sha256), the `egress_log` lines, BK1's name, size and sha256, the build's hashes, and a notes file with UTC times. Screenshots, the statement PDF and the data folder stay on the lab PC, listed by name, size and sha256.
