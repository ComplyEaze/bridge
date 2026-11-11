# #1761 slice 4: a bank cash withdrawal paid to someone, posted and read back

Not for merge. One row, on one NEW synthetic company. `LAB-COMMON.md` in this folder (v3, a byte copy of `lab/1761-s1-cash-day-end-run-sheet` at bef604bb) applies in full; where this sheet and it differ, the stricter holds. THE COMPANY below is BRIDGE CASHPAY LAB. Record the TallyPrime version, the licence mode and the date.

**WHY.** Slice 4 of #1761 (#2407) builds a bank cash withdrawal answered `paid_to_someone` as two vouchers on its date:
- a Contra, Dr the cash ledger, Cr the bank, carrying the statement row's id;
- a cash Payment of the same amount, Dr the person paid, Cr the cash ledger, whose id is the row's id followed by `-cash`.

A Payment whose payee's cash in a day comes to more than 10,000.00 is tagged in its narration for the CA (s.40A(3)). The pull request is tested only against a simulator. This row is its live evidence (P1). It checks:
- that Tally accepts the pair in one batch;
- that each voucher lands with its ledgers, amounts and narration intact, the tag included;
- that the build refuses a bank ledger named as the cash ledger, by reading the book's groups.

**WRITES.**
- Masters are keyed BY HAND on Tally's screen.
- One batch of four vouchers is posted by the build's own `post_import`, after [OWNER]'s approval.
- Nothing is sent by curl, so the request-ID preflight finds no request to check.
- The book is restored to BK1 at the end.

**[OWNER] steps** are marked [OWNER]: the owner at the lab PC. Everything else is the operator's. The run coordinator's word, where the row asks for it (go-ahead, RESTORE, the evidence push), comes from the run coordinator.

## Build (holds the merge of #2407)

The build is the Windows bundle `unsigned-bundle-smoke-windows-latest` of the Bridge CI run on master that first holds the merge of #2407. The lab downloads it into a new empty folder and checks three hashes:
- the zip's sha256 against the artifact's digest;
- the `.mcpb` against `archive_sha256` in its `mcpb-smoke.json`;
- `bridge_mcp.exe` against `binary_sha256` in the same file.

It sends those three hashes to the run coordinator, who fills the line below by a new commit on this branch, after checking that the merge of #2407 is an ancestor of RC COMMIT. While any field reads `<to fill>`, this row does not run: STOP. Before the first call, hash the exe that will run and compare it with the line; any other hash = STOP. Record the version the tool reports.

RC COMMIT: <to fill>  CI RUN: <to fill>  ARTIFACT: <to fill>  ZIP DIGEST: <to fill>  EXE SHA256: <to fill>

Run it as the lab runs a staged exe over stdio, ONE launch from step 0 to the close, never relaunched in between. Environment:
- `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9001`;
- ONE new empty `BRIDGE_AGENT_DATA_DIR`, `C:\bridge-lab\1761-s4\data\`;
- `BRIDGE_TERMS_ACCEPTED=true`, the person responsible for the lab accepting the Terms of Use;
- [OWNER] posting ON: `BRIDGE_AGENT_ENABLE_WRITES=true`, set before the launch (step 0 records only that it is present);
- `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` unset, and `BRIDGE_AGENT_ENABLE_BATCH_POST` absent (it is retired).

No other ComplyEaze Bridge may make a call against port 9001 during the row. Record the process list at step 0.

**THE APPROVAL WINDOW** opens from the exe's own process. On this build it is opened topmost and system-modal (#2107, #2193); if it is not seen in front, look for it on the taskbar and record where it was. While it is open nobody presses a key on the PC and no tool is called, except step 4's own repeat `post_import`. [OWNER] answers it by mouse, within its time. Record the time it was seen open and the time it was answered.

**Marks.** MARKS means `ledger_masters`, first page, no `snapshot_id`:
- M = `snapshot.master_alter_id`;
- V = `snapshot.voucher_alter_id`;
- `snapshot.reused` must read false.

Record each answer whole.

**Calls.** Only the build's own tools, one at a time. An answer's `next_step` or `next_steps` is recorded, never acted on: this row's steps decide every call.

## Before the sitting
1. [OWNER] Tally is LICENSED on the lab PC. Educational mode anywhere on screen = STOP (C1).
2. Check every file against `SHA256.txt` (Get-FileHash). Copy `statement.csv` to `C:\bridge-lab\1761-s4\statement.csv` and check that copy's sha256 before every parse; a different file = STOP.
3. [OWNER] Go-ahead for this row.

## The company (new)
BRIDGE CASHPAY LAB: books beginning 2025-04-01, no inventory, no GST. If a company or a data folder of this name already exists: STOP, report it, and reuse or overwrite nothing. Masters, keyed by hand:

| ledger | under | opening |
| --- | --- | --- |
| Cash (Tally's own) | Cash-in-Hand | none |
| BRIDGE Bank A | Bank Accounts | 50,000.00 Dr |
| BRIDGE Bank B | Bank Accounts | none |
| BRIDGE Contractor A | Sundry Creditors, "Maintain balances bill-by-bill" No | none |
| BRIDGE Wages | Indirect Expenses | none |
| BRIDGE Suspense | Suspense A/c | none |
| BRIDGE Capital | Capital Account | 50,000.00 Cr |

Take a screenshot of each ledger's screen before its accept. BRIDGE Contractor A's screenshot must show "Maintain balances bill-by-bill" reading No; if it reads Yes, or the field is not on the screen, STOP before step 0 (the build would refuse the Payment as a bill-wise party, `bill_wise_party_unapproved`, and step 3 would test nothing). After the last one, leave the creation screen as C7 says, then take a screenshot of the Gateway with THE COMPANY loaded and no dialog. Do not change the Payment or Contra voucher types.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay). Record the process list and that `BRIDGE_AGENT_ENABLE_WRITES` is present. Hash the exe against the build line.
2. `tools/list`: record the answer whole. EXPECT:
   - a tool named `parse_bank_statement_csv` whose input schema lists `cash_ledger` under `cash_answers` items;
   - a tool named `post_import`.

   Either missing = STOP (not this build, or posting is off).
3. `list_companies`: EXPECT exactly THE COMPANY. Record its GUID; every later call passes it as `company_guid`. Anything else = STOP.
4. MARKS: M0, V0.
5. `masters` with `kind` `voucher_types`: record the Payment and Contra rows whole. EXPECT each to read `series_numbering.observation` `observed`, `series_numbering.method` `automatic` and `series_numbering.sub_method` `auto_retain`; otherwise STOP before anything changes. `post_import` refuses, before its window, a series that is not Automatic (`import_voucher_numbering_not_automatic`) or not Retain (`import_voucher_numbering_not_retain`), so step 4 would test nothing.
6. Backup BK1 (C5) into a NEW empty folder `C:\bridge-lab\1761-s4\backups\BK1\`: record the name, size and sha256. Then MARKS: M0b, V0b.

The book: the masters only.

## Step 1: the parse with the cash lines open (local; nothing goes to Tally)
`parse_bank_statement_csv` with:
- `statement_path` the copy above, `controls_source` `prior_statement`, `opening_balance` `50,000.00` and `closing_balance` `35,000.00`;
- `account_label` `Lab CSV 4321`, `bank_ledger` `BRIDGE Bank A` and `suspense_ledger` `BRIDGE Suspense`;
- no `mapping` and no `cash_answers`.

EXPECT:
- `statement_rows` 2, `vouchers` 0, `cash_questions_open` 2 and `cash_payments_over_limit` 0;
- two `cash_questions`, each with `movement` `withdrawal` and `printed_as` `ATM CASH WITHDRAWAL`:
  - `st-20250408-be621b1463b52f1b`, 12000.00 on 2025-04-08;
  - `st-20250409-ef7d14f34a7b0412`, 3000.00 on 2025-04-09.

These are the answers the parse gave at #2407's head `f8f9cf3f`, run on a Mac with no Tally (verified there; on master 7a3c9e49 the two ids were also recomputed from the code's id rule without running it, and the slice's crate code is unchanged since that head). Other ids, counts or amounts = STOP; record the answer whole.

The book: unchanged.

## Step 2: a bank ledger named as the cash ledger, refused at build (reads only)
1. The same parse, with `cash_answers`:
   - `{"bridge_txn_id": "st-20250408-be621b1463b52f1b", "answer": "paid_to_someone", "ledger": "BRIDGE Contractor A", "cash_ledger": "BRIDGE Bank B"}`;
   - `{"bridge_txn_id": "st-20250409-ef7d14f34a7b0412", "answer": "paid_to_someone", "ledger": "BRIDGE Wages", "cash_ledger": "Cash"}`.

   EXPECT `vouchers` 4 and `cash_questions_open` 0 (the parse does not read the book). Record `proposals_id` and `sha256` as P2.
2. `build_import_xml` with `company_guid`, `proposals_id` and `proposals_sha256` of P2. EXPECT:
   - `result.state` `refused` and `result.reason` `cash_ledger_not_cash_in_hand`;
   - `result.refused_ledgers` holding exactly one row: `ledger` BRIDGE Bank B (plain or as its redaction marker; record which), `requires` `cash_in_hand`, `state` `cash_bank`, `reserved_group` naming Bank Accounts and `first_bridge_txn_id` `st-20250408-be621b1463b52f1b`;
   - `result.refused_ledgers_omitted` 0;
   - no `batch_id` and no file written.

   Record the answer whole. Any other reason (`cash_bank_ledger_not_established` included) or another row is a FINDING, not a pass: record it and go on to step 3. A file built = STOP: that is a FINDING, and the file is never sent.

   BRIDGE Bank B sits under Bank Accounts, so it passes the build's earlier check that each cash or bank leg reaches Cash-in-Hand or Bank Accounts, and is refused only by the check that a ledger answered as cash in hand is one. BRIDGE Wages, the second row's payee, is not a cash ledger here.
3. MARKS: M and V equal to M0b and V0b.

The book: unchanged.

## Step 3: the batch, built (not sent)
1. The same parse, with `cash_answers`:
   - `{"bridge_txn_id": "st-20250408-be621b1463b52f1b", "answer": "paid_to_someone", "ledger": "BRIDGE Contractor A", "cash_ledger": "Cash"}`;
   - `{"bridge_txn_id": "st-20250409-ef7d14f34a7b0412", "answer": "paid_to_someone", "ledger": "BRIDGE Wages", "cash_ledger": "Cash"}`.

   EXPECT `vouchers` 4, `cash_questions_open` 0 and `cash_payments_over_limit` 1. Record `proposals_id` and `sha256` as P3.
2. `build_import_xml` with P3. EXPECT `voucher_count` 4 and a `batch_id`: record the answer whole and that `batch_id` as C. Any refusal: record it whole and STOP.

The book: unchanged.

## Step 4: the approved post (C)
1. MARKS: V4a, immediately before the post. EXPECT V4a = V0b.
2. Tell [OWNER] a window will open. `post_import` with `company_guid` and `batch_id` C. [OWNER] reads the window and answers **Yes** by mouse within its time.
   - If `approval.state` reads `pending`: wait `approval.retry_after_s` seconds and call `post_import` again with C.
   - If it reads `approved` without a send: call again at once. This is the tool's own flow, not a retry.

   Record every answer whole.
3. If an answer reads `import_approval_timed_out`: record it whole, read MARKS and `vouchers` for 2025-04-08 to 2025-04-09 once each, and STOP. C is not called again without the run coordinator's word, and only if that answer read `attempt_recorded` false, V = V4a and no voucher was in the window.
4. EXPECT, in the answer of the call that sent the post:
   - `dispatch.state` `posted_verified` and `dispatch.counters.created` 4;
   - `post_location.state` `target_only`, with `post_location.target_voucher_step` reading `before` V4a, `after` V4a + 4, `step` 4 and `matches_created` true;
   - `post_span_binding.state` `bound`.

   That V moves by one per voucher created is recorded for a post of one voucher (0.7.0 row W1) and unverified for a batch. For a batch, a `step` other than 4 reads `matches_created` false, and the answer is then `reconciliation_required` (`batch_step_unconfirmed`), not `posted_verified`. That is a FINDING: record it whole, then STOP as the last line of this step says, with no repeat call and no rebuild. Then, before any other call, take a screenshot of the whole Tally window: a dialog or a half-entered screen = C7, STOP.
5. `verify_import` of C: EXPECT `verification_status` `posted_verified` and `post_span_binding.state` `bound`, with no `rollback_recorded` and no `voucher_mark_contradiction` anywhere in the answer. Record the answer whole.

Any other final state or refusal: record it whole, do not call again, STOP.

The book: plus the four vouchers of C.

## Step 5: read back
1. MARKS: record V (EXPECT V4a + 4; see step 4).
2. `vouchers` for 2025-04-08 to 2025-04-09: record the answer whole and a table with one line per voucher: date, type, number, each ledger entry (ledger, amount, side) and the narration printed with its length in characters. EXPECT exactly these four, and no other voucher:

   | date | type | Dr | Cr | amount | narration (exactly) |
   | --- | --- | --- | --- | --- | --- |
   | 2025-04-08 | Contra | Cash | BRIDGE Bank A | 12,000.00 | `to Cash \| Lab CSV 4321 \| 08-Apr-2025` |
   | 2025-04-08 | Payment | BRIDGE Contractor A | Cash | 12,000.00 | `Cash paid to BRIDGE Contractor A from Cash \| withdrawn \| Lab CSV 4321 \| 08-Apr-2025 \| Bridge: cash to one payee in a day over 10000; check s.40A(3)` |
   | 2025-04-09 | Contra | Cash | BRIDGE Bank A | 3,000.00 | `to Cash \| Lab CSV 4321 \| 09-Apr-2025` |
   | 2025-04-09 | Payment | BRIDGE Wages | Cash | 3,000.00 | `Cash paid to BRIDGE Wages from Cash \| withdrawn \| Lab CSV 4321 \| 09-Apr-2025` |

   The narrations are the proposals the parse wrote at `f8f9cf3f` on a Mac (the `\|` in the table is a plain `|`). A missing voucher, an extra one, or another ledger or amount = STOP: a FINDING. A narration that differs (cut short, a character changed) is a FINDING: record it exactly, then go on to step 6.
3. `ledger_movement` for Cash, 2025-04-01 to 2025-04-30: record it whole. EXPECT its closing to equal its opening (15,000.00 in, 15,000.00 out).
4. `ledger_movement` for BRIDGE Bank A, 2025-04-01 to 2025-04-30: record it whole. EXPECT its closing 15,000.00 nearer zero than its opening (opening 50,000.00 Dr, closing 35,000.00 Dr). A figure that differs from these EXPECT lines is a FINDING, recorded, not a STOP.

The book: as after step 4.

## Step 6: the restore
1. [OWNER] Restore BK1 (C6), on the run coordinator's explicit RESTORE. There is no fallback backup: a restore that does not complete is a STOP, reported as C6 says.
2. MARKS first (C6): EXPECT M and V equal to M0b and V0b.
3. Then `list_companies` (as step 0), and `vouchers` for 2025-04-08 to 2025-04-09: EXPECT none. Else STOP, and report what the book holds.

The book: as at BK1, the masters only.

## Not covered by this row
- A deposit, and any cash answer other than `paid_to_someone`.
- The over-limit sum across several Payments to one payee in a day: here each payee has one Payment. The tag on exactly 10,000.00, and cash paid outside this file. The unit tests of #2407 carry these.
- A ledger outside both Cash-in-Hand and Bank Accounts named as the cash ledger (an earlier check refuses it with another reason). A bill-wise payee.
- A cash ledger other than Tally's own Cash, and a PDF statement. This row parses the CSV edge only; the voucher shapes are the same.
- Any TallyPrime release other than the one recorded.

## The book after each step
- Step 0: the masters only.
- Steps 1 to 3: the same.
- Steps 4 and 5: plus the four vouchers of C.
- Step 6: as at BK1.

STOP RULES: C7 and C8; any dialog before a call; an approval window other than step 4's; a mark that moves outside the step that changes it; any answer outside the stated shape.

## Close and evidence
- **C10:** Shut Company; leave no company loaded. THE COMPANY stays on the lab PC: it is synthetic and new.
- **C11:** evidence to a new branch `lab/1761-s4-cash-paid-to-someone-answers`, with `SHA256.txt`, after the pre-push scan and the run coordinator's yes. It holds:
  - every answer whole with its size and sha256;
  - the step 5 voucher table;
  - the data folder's file listing after step 4 (names, sizes, sha256);
  - BK1's name, size and sha256, and the build's hashes;
  - the window's open and answer times;
  - a notes file with UTC times and one line per step.

  A RESULTS.md gives each check as PASS or FAIL, with the build, company, date and record: the build refusal (step 2), the post (step 4), the read-back (step 5) and the restore (step 6).
