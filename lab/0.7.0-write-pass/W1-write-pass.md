# W1: one approved post on the 0.7.0 cut build, with the checks that changed since 0.6.1

Not for merge. BLOCKING for release 0.7.0. `LAB-COMMON.md` (v3) and `README.md` in this folder apply in full; where this row and they differ, the stricter holds. Steps marked [OWNER]: see WHO ACTS in `README.md`.

**THE COMPANY** is BRIDGE TIE OUT LAB 3 (company number 100065, GUID `9c97970c-e25d-4348-9740-6ddb1caaa56d`, books from 1 April 2026; bank ledger Union Bank; July and August imported; nothing on Union Bank in September). The 27 Oct 2026 row R3 restored it to its backup BK1 of that day (`lab/1714-skip-at-send-answers` at aeae741b, RESULTS.md: M 210, V 8, five ledgers, no BRIDGE Bank B). If THE COMPANY is not on the lab PC: STOP. Never create a substitute.

**THE STATEMENT** is the synthetic three-month PDF of the 14 October run, sha256 `506e05ebfb0f81d6e0fa4b0cc867bbaeae8a69d130c02615ea48a86559894fc9`. Check it before every parse; a different file = STOP.

**THE CALLS.** Only the build's own tools, one at a time. `parse_bank_statement` takes the arguments the 27 Oct R3 row used (bank `ubi`, account label `Lab UBI 7788`, opening 10000.00, closing 11000.00, bank ledger `Union Bank`, suspense ledger `Suspense`, the password file on the lab PC), changing only `from`, `to` and `mapping` as each step says. Every `build_import_xml` passes `company_guid`, `proposals_id` = that parse's `proposals_id`, and `proposals_sha256` = that parse's `sha256`. An answer's `next_step` or `next_steps` is recorded, never acted on: this row's steps decide every call. A text check below reads the named JSON string field of the answer as parsed JSON and tests whether a phrase occurs in it; it is never a search of the answer file's bytes. Paths below are inside `structuredContent.result` of the tool's answer.

**BEFORE (0.6.1).** The 0.6.1 cut build's answers on this book are the "before" for the two text checks: `answers/000-s0-tools-list.json` (the `post_import` description holds "enter the voucher in Tally" and "is entered in Tally by hand") and `answers/041-s4-build-skip.json` (`transfers.next_steps.held_elsewhere` holds "sending it gives posted_against_skip"), both on `lab/1714-skip-at-send-answers` at aeae741b.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay). Record the process list and that `BRIDGE_AGENT_ENABLE_WRITES` is present. Hash the exe against the build line.
2. `tools/list`: record the answer whole and the number of tools. **Check 1967:** in the `description` of the tool named `post_import`, EXPECT the phrases "cannot post that row again yet: tell the user so" and "cannot post the extra row yet: leave it out and tell the user so" each present once, and the phrases "enter the voucher in Tally" and "is entered in Tally by hand" each absent. No `post_import` tool listed = STOP (posting is not on).
3. `list_companies`: EXPECT exactly THE COMPANY with the GUID above; anything else = STOP.
4. MARKS: M0, V0. EXPECT M0 210 and V0 8, as R3's restore left them; other values = STOP (the book is not as recorded).
5. `ledger_movement` for Union Bank, 2026-07-01 to 2026-09-30: EXPECT opening -10000.00 and closing -11800; another figure = STOP.
6. `masters` with `kind` `voucher_types`: record the Payment and Receipt rows whole. EXPECT for each `series_numbering.observation` `observed`, `series_numbering.method` `automatic` and `series_numbering.sub_method` `auto_retain`; otherwise STOP before anything changes.
7. Backup BK1 (C5) into a NEW empty folder `C:\bridge-lab\070-write-pass\backups\BK1\`: name, size and sha256. Then MARKS.

The book: unchanged.

## Step 1: a second bank ledger
[OWNER] Create ledger `BRIDGE Bank B` under Bank Accounts, no opening balance; screenshot before the accept. After the accept, leave the creation screen as C7 says (a screenshot showing its title, then Ctrl+Q), then a screenshot of the Gateway of Tally with THE COMPANY loaded and no dialog; a dialog = C7, STOP. MARKS: M1 (EXPECT above M0), V1 = V0.

The book: plus `BRIDGE Bank B`.

## Step 2: the two rows (the statement file only)
Parse 2026-09-01 to 2026-09-30 (no `mapping`): EXPECT `rows_in_window` 2 and `cash_questions_open` 0. Parse 2026-09-03 alone (D) and 2026-09-30 alone (D2), no `mapping`: EXPECT for each `rows_in_window` 1, `cash_questions_open` 0, `skipped` 0, and one `counterparties` entry naming a party: GREEN FIELD LTD for D (P) and SILVER OAK MUTUAL for D2, as on 27 Oct. Another count or party = STOP and report the answers.

The book: unchanged.

## Step 3: batch A, built for D (not sent)
`build_import_xml` with the parse of D. EXPECT `voucher_count` 1 and a `batch_id`: record the answer whole and that `batch_id` as A. Any refusal: record it whole and STOP.

The book: unchanged.

## Step 4: the skip of that row, recorded after A was built
`parse_bank_statement` for D with `mapping` `[{"party": "GREEN FIELD LTD", "ledger": "BRIDGE Bank B", "treatment": "skip"}]`: EXPECT `vouchers` 0 and `skipped` 1. Then `build_import_xml` with that parse. EXPECT `state` `skips_recorded`, `skipped_rows` 1 and a `record_id`, and in `transfers`: `state` `computed`, `skips_open` 1, `open_omitted` 0, and one `open` row with `state` `held_elsewhere` and `batch_id` A. R is that row's `bridge_txn_id` (it begins `st-20260903-`). Record the answer whole.

**Check 1995:** in `transfers.next_steps.held_elsewhere`, EXPECT the text to begin "The batch named posts this row, which a statement skipped, and has not been found posted. Verify that batch." and to end "A batch found posted without the withdrawal leaves the row as posted_against_skip."; EXPECT the phrases "do not send the batch named or import its file" and "post_import refuses that send as skip_row_paired_elsewhere" each present, and the phrase "sending it gives" absent.

Any refusal, another state, or the section cut short: record it whole and STOP (nothing was written to Tally).

The book: unchanged (the skip is recorded in the data folder only).

## Step 5: batch A sent, refused before the window (what the step-4 text says)
1. Record the name, size and sha256 of every file in the data folder. MARKS: V5a.
2. Tell [OWNER] that no approval window is expected. `post_import` with `company_guid` and `batch_id` A.
3. EXPECT: no approval window; `error.code` `skip_row_paired_elsewhere`, `error.skipped_rows` holding R, and `attempt_recorded` false. Record the answer whole. `attempt_recorded` null = STOP.
4. MARKS: V = V5a. The batch journal (`agent-import-ledger.jsonl`) and the skips file (`agent-import-statement-skips.jsonl`): the same size and sha256 as in 1. `egress_log`: record the lines this call added; EXPECT its `post_import` receipt (record type `response_prepared`) to carry the code `skip_row_paired_elsewhere` and NO `request_trail` key (step 6 is its positive control). `vouchers` for D: EXPECT no voucher on Union Bank that day.

If an approval window opens: [OWNER] answers **No** by mouse, screenshot, record the answer whole, STOP: that is a FINDING. Any other code, a moved V, a change in either file, or a voucher on D: record it whole and STOP.

The book: unchanged.

## Step 6: the approved post (B, for D2)
1. `build_import_xml` with the parse of D2: EXPECT `voucher_count` 1; record its `batch_id` as B. MARKS: V6a, immediately before the post (EXPECT V6a = V0).
2. Tell [OWNER] a window will open. `post_import` with `company_guid` and `batch_id` B; [OWNER] reads the window and answers **Yes** by mouse within its time. If `approval.state` reads `pending`, wait `approval.retry_after_s` seconds and call `post_import` again with B; if it reads `approved` without a send, call again at once (the tool's own flow, not a retry). Record every answer whole.
3. If an answer reads `import_approval_timed_out`: record it whole, MARKS and `vouchers` for D2 once each, and STOP. B is not called again without the run coordinator's word, and only if that answer read `attempt_recorded` false, V = V6a and no voucher was on D2.
4. EXPECT, in the answer of the call that sent the post: `dispatch.state` `posted_verified`, `dispatch.application_status` `not_reported`, `dispatch.counters.created` 1 and `dispatch.response_state` `response_clean` (the answer's status as Tally sent it, the evidence for NOT COVERED); `post_location.state` `target_only` with no `after_read_failure`; `post_location.target_voucher_step` reading `before` V6a, `after` V6a + 1, `step` 1 and `matches_created` true; `post_span_binding.state` `bound`. Then, before any other call, a screenshot of the whole Tally window: a dialog or a half-entered screen = C7, STOP.
5. `verify_import` of B: EXPECT `verification_status` `posted_verified`, `alter_id_delta.from` `pre_post_mark` with its before = V6a, and `post_span_binding.state` `bound`; and no `rollback_recorded` and no `voucher_mark_contradiction` anywhere in the answer. **Check 1896 (no false line on a clean post; 0.6.1 passes it too):** `post_span_binding.summary` EXPECT exactly "Each voucher was matched to the Tally voucher this post created." Record the answer whole.
6. MARKS: V = V6a + 1. `vouchers` for D2: one Receipt on Union Bank of 400.00, as the statement row says. `egress_log`: EXPECT the receipt of the sending `post_import` (record type `response_prepared`) to carry `request_trail.sent` of 1 or more.

Any other final state or refusal: record it whole, do not call again, STOP.

The book: plus one voucher dated D2.

## Step 7: the restore, and the rollback floor and record on the cut build (regression)
1. [OWNER] Restore BK1 (C6), on the run coordinator's explicit RESTORE. There is no fallback backup: a restore that does not complete is a STOP, reported as C6 says. MARKS first (C6): EXPECT M and V equal to the MARKS read after BK1 (step 0.7); then `list_companies` (as step 0); `ledger_movement` as step 0: equal to step 0's; `ledger_masters` (every page): no row named `BRIDGE Bank B`. Else STOP, and report what the book holds.
2. `verify_import` of B (reads only; it writes a rollback record in this row's data folder, which is never used again). **Check rollback record:** EXPECT `post_span_binding.state` `book_rolled_back` with `current_mark` the V read in 7.1 and `expected_at_least` V6a + 1, `post_span_binding.rollback_recorded` true, `post_span_binding.other_batches_covered` equal to `{"count": 0, "first": []}`, `counts.book_rolled_back` 1, `counts.not_found` 0, `unverified_vouchers[0].status` `book_rolled_back`, and `verification_status` `verification_incomplete`. Record the answer whole.
3. `verify_import` of B again. **Check rollback cover:** EXPECT `post_span_binding.state` `book_rolled_back` with `mark_read` the V read in 7.1 and `seen_by` B, NO `rollback_recorded` key, `verification_status` `verification_incomplete`, and `counts.posted_verified` 0. Record the answer whole.

A `voucher_mark_contradiction`, a `posted_verified`, a `not_found` for B's voucher, or a `rollback_recorded` in 3: record it whole and STOP; that is a FINDING. What this step shows: on the cut build a bound post's floor still reads a mark below it as rolled back, records that once, and reads the record as cover afterwards. B is bound and its answer created 1: with #2047 its floor comes from the binding, without it from the answer's `CREATED`, and both give V6a + 1. So this is a regression check of the floor, not a test of #2047, and it passes on 0.6.1 too.

The book: as at BK1, the book of step 0.

## Not covered by this row
- #2047's change (#2035, #2008): an answer reporting `STATUS` 0, and the floor of a native post that was not bound, whose floor comes from its answer. No native post has been seen to answer with a status (step 6 records `not_reported`), and a clean post binds, so neither can be produced from the tools; the unit tests of #2047 and #2031 carry them.
- #1896's added line on a binding refused for the mark's step or the span's count: it needs another change to the company's vouchers inside the seconds of the post, which no step here can time; the unit tests of #1896 carry it.
- #2030's refusal of eight digits that name no calendar day: its four checks read Tally's own answers (a Sales invoice's HSN and rate dates, a company context field, the desktop sync), not a tool argument; this book has no Sales invoice. An impossible date given as a tool argument was refused as `invalid_date` before #2030 too, so it would not show the change.
- A post of 2 to 50 vouchers (B is one voucher), Sales invoices, Gold, and any TallyPrime release other than the one recorded.

## The book after each step
Step 0: unchanged. Step 1: plus `BRIDGE Bank B`. Steps 2 to 5: the same. Step 6: plus the D2 voucher. Step 7: as at BK1.

STOP RULES: C7 and C8; any dialog before a call; an approval window other than step 6's; a mark that moves outside the steps that change it; any answer outside the stated shape.

## Close and evidence
C10: Shut Company; no company loaded. Evidence on a new branch `lab/0.7.0-write-pass-answers` with `SHA256.txt`, after the pre-push scan and the run coordinator's yes: every answer whole with its size and sha256, the data folder's file listing of step 5 (names, sizes, sha256), the `egress_log` lines, BK1's name, size and sha256, the build's hashes, the window's open and answer times, and a notes file with UTC times and one line per step. A RESULTS.md gives each check (1967, 1995, the post, 1896 no false line, rollback record, rollback cover) as PASS, FAIL or, for 1967 and 1995 only, "not in the build", with the build, company, date and record. No check is reported as a PASS of #2047 or #1896: the last three would pass on 0.6.1 too. Screenshots, the statement PDF, BK1 and the data folder stay on the lab PC, listed by name, size and sha256.
