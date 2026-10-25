# Write-path live rows for TallyPrime 7.2 (lab sheet, not for merge)

Three rows that check ComplyEaze Bridge's write paths on a live book before the release tag. They run after the
read regression, one row at a time, in the order W1, W2, W3. Synthetic BRIDGE companies only: never a client book,
never REGISTER LAB.

## Rules for every row

- **A fresh backup first.** Before a row's first call, back the company up into a NEW empty folder; record the
  backup's name, size and SHA-256. Restore only on the coordinator's order, by Tally's own Restore.
- **The owner is at the PC for every native window.** ComplyEaze Bridge's approval and review windows are answered
  only by the owner, after the call that opened them has returned. Do not type in Tally while such a window is open:
  a key typed for Tally can answer it.
- One request at a time. Never retry, cancel or re-send a write. Gateway of Tally with no dialog before every call.
  Save every answer whole to a file and read it with a parser, never by eye or by grep.
- STOP and report, with nothing improvised, on: an unexpected dialog in Tally, a second company loaded, Education
  mode, any answer unlike an EXPECT line, or a refusal code a row does not name.
- The owner types the Terms and posting settings; the lab never does.
- Record for every row: the program's commit and its SHA-256, TallyPrime's release (7.2) and edition, the company's
  name and GUID, each answer file's SHA-256, the backups' hashes, and a timed log.

## Builds

| row | needs a build that contains | why |
|---|---|---|
| W1 | master at or after `631b9386` (#1814) | the restore record; an older build reads `posted_verified` at step 7, which is the wrong build, not a failure |
| W2 | #1650's head (`289fcd15` or later, which contains #1545) | the ledger tools exist only there |
| W3 | #1714's head (`5772eb15` or later) | the send-time transfer check exists only there |

One lab build may serve all three: record the commit it was built from and the heads it contains. Settings:
posting on (owner types it); for W2 also `BRIDGE_AGENT_ENABLE_MASTER_WRITES` on; `BRIDGE_AGENT_ENABLE_BATCH_POST`
absent. A fresh data folder for each row.

## W1. A restore once seen keeps a re-keyed post from reading posted_verified (#1050, #1814)

Decides that the 12 Oct finding (a voucher keyed again by hand after a restore read `posted_verified`) no longer
reproduces. Company: a synthetic BRIDGE company with a bank ledger and a party ledger and AT LEAST ONE voucher on
another day (an empty company omits the voucher mark, and an omitted mark records nothing: STOP if `tally_status`
shows no voucher mark). No voucher on day D. Backup BK0 first.

0. `list_companies`; `tally_status` (record the voucher mark V0); `vouchers` day D: EXPECT none.
1. `build_import_xml`: batch P1, one Receipt, 1.00 from the party to the bank, narration "W1 P1". `post_import` P1
   (the owner approves); `verify_import` P1: EXPECT `posted_verified`, post_span_binding bound. `tally_status`: V1 =
   V0 + 1.
2. The same for batch P2, one Receipt, 2.00, narration "W1 P2": EXPECT `posted_verified`; V2 = V0 + 2. `vouchers`
   day D: record both (MasterID, AlterID, GUID, number).
3. The coordinator restores BK0 on screen. `tally_status`: V3. EXPECT V3 = V0; STOP otherwise.
4. Hash `agent-import-ledger.jsonl`; count its records whose status is `rollback_seen` with a JSON parser: EXPECT 0.
5. `verify_import` P2. EXPECT not `posted_verified`; `post_span_binding.state` `book_rolled_back`;
   `post_span_binding.rollback_recorded` true; `post_span_binding.other_batches_covered.count` 1, naming P1.
   Journal: exactly one `rollback_seen` record, on P2's batch id.
6. The owner keys K1 by hand in Tally: a Receipt on day D with P1's content (same ledgers, 1.00, narration
   "W1 P1"). `tally_status`: V4 = V3 + 1. `vouchers` day D: record K1, and whether K1's GUID equals P1's.
7. `verify_import` P1. EXPECT not `posted_verified`; `post_span_binding.state` `book_rolled_back`, with `mark_read`
   and `seen_by` (P2's batch id); the plain line says the books were restored after this post. Journal: still one
   `rollback_seen`.
8. `verify_import` P1 again: EXPECT the same, and no second record.
9. Quit and restart Claude Desktop. `verify_import` P1: EXPECT the same.

PASS: 5, 7, 8 and 9 as above. FAIL: 7, 8 or 9 reads `posted_verified`; 5 records nothing although the mark was
present; a second `rollback_seen` record. Writes: two native posts and one voucher keyed by hand, all removed by the
final restore of BK0.

## W2. One ledger created through the approved dispatch, near names refused, the window's answer line (#1650, #1545)

Repeats the 14 Oct run (lab branch `lab/ml1-live-ledger-create-answers`, TallyPrime 7.1) on 7.2 with #1650's
current head, and adds the near-name refusals and the window's new first line. Company: BRIDGE MASTERS LAB, loaded
alone. Backup BKM first.

1. `ledger_masters` of the company: record the list. Choose the existing ledger `MLab Live Suspense` (created on
   14 Oct); if it is not in the list, STOP (the book is not the one the row was written for).
2. `prepare_ledger_creates` with ledger names `MLab LiveSuspense` (a removed space), `mlab live suspense` (case)
   and `MLab Live-Suspense` (a space made a dash), each kind `suspense`, one call each. EXPECT each refused
   `master_name_taken`, no batch, nothing sent.
3. `prepare_ledger_creates` with a new name `MLab Live Suspense 72`, kind `suspense`. EXPECT a batch id, parent
   Suspense A/c, a preview. Read `tally_status`: record the master mark M0.
4. `post_ledger_creates` with that batch. EXPECT `approval.state` pending and a window titled
   "ComplyEaze Bridge — create one ledger?", in front of Tally. Record its first line word for word. EXPECT on
   Windows: "Press Y (or click Yes) to create this ledger in Tally. Press N (or click No) to cancel. Nothing is
   created until you answer." The owner presses No only after the call has returned.
5. `post_ledger_creates` again: EXPECT `master_approval_declined`, nothing sent; the master mark is still M0.
6. `post_ledger_creates` again: a new window opens; the owner presses Yes after the call has returned. Call once
   more if the answer is approved with `attempt_recorded` false (never a re-send). EXPECT `posted_verified`,
   created 1, every other counter 0. `tally_status`: master mark M0 + 1.
7. `post_ledger_creates` once more with the same batch. EXPECT `posted_verified` from the record, nothing sent, no
   window.
8. `ledger_masters`: EXPECT the list of step 1 plus exactly one ledger, `MLab Live Suspense 72`, under Suspense
   A/c; every other ledger unchanged by GUID and AlterID.
9. `local_data_report`: EXPECT the master journal read, one batch, settled.

PASS: 2 refused all three; 4 shows the line; 5 sends nothing; 6 to 8 as above. FAIL: a near name accepted or
sent; the window behind Tally or without the line; more than one ledger created; any existing ledger changed.
Writes: one ledger, removed by the final restore of BKM.

## W3. A batch whose row was skipped after it was built is refused at send (#1714)

Company BRIDGE TRANSFER PAIR LAB, loaded alone, with no voucher dated 3 to 9 Aug 2026; inputs on branch `lab/transfer-pairing-tp-inputs` at `9318b08d` (statement A, row E2 for the refusal, row E7 for the
control), a fresh data folder, backup SD0 first. Core: 14 calls, one write.

1. Parse statement A for 4 Aug 2026 with the Bank B party mapped `contra`; build: batch X, one Contra. Do not post X.
2. Parse again with the party mapped `skip`; build: EXPECT `skips_recorded`, one skipped row.
3. Hash the journal. `post_import` X: EXPECT within seconds, with no window, `skip_row_paired_elsewhere`, the
   skipped row `st-20260804-b1d36f58182d117c`, `attempt_recorded` false, nothing sent; the journal hash unchanged.
   If a window opens for X, the owner presses No: that is the FAIL signature.
4. Control: parse for 8 Aug with `contra`; build batch Y; `post_import` Y; the owner approves; `verify_import` Y:
   EXPECT `posted_verified` and exactly one Contra on 8 Aug.

PASS: 3 refuses as above with no window; 4 posts one Contra. Writes: one voucher (Y), removed by the final restore
of SD0.

## Not covered here

Batches of more than one voucher; Gold; macOS; a restore that removes masters; party ledgers with GST details (W2
creates a suspense ledger only); the other send-time refusal codes of #1714.
