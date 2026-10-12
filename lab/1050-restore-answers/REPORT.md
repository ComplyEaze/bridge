# 1050-R: MasterID reuse after a backup restore, Windows lab, 12 Oct 2026

Build: bridge_mcp.exe 411f7cca80ca4a5fee543f4f44d14b911ce23e982169785b1762d8d50472e5a6 (#1516 build), posting ON, batch posting ON, redaction none.
tally_status: TallyPrime 7.1 Silver, Education off, port 9001; only BRIDGE RESTORE LAB loaded at every step (r0-build-status.txt).
Book: NEW synthetic company BRIDGE RESTORE LAB, company number 100054, GUID 39e44d0b-e708-42d3-bb32-821d195b3d97, books from 20260401.
- No GST, no inventory, automatic numbering.
- Ledgers: Synthetic Bank Ledger (Bank Accounts), Party A (Sundry Debtors, bill-by-bill No), Lab Seed Income (Indirect Incomes).
- Seed: Receipt 1-Jul-2026, Cash Dr / Lab Seed Income Cr 1,000.00, "Seed voucher".
CLICK MODE: lane. Each Bridge dialog was fronted by window-watch, screenshotted, and Yes was clicked with the mouse.
Every vouchers read is 2026-07-01 to 2026-07-31: result.state complete, one page.

## Steps (file, sha256 first 8; all items verified from the saved file unless marked)
1. r1-list-companies.json b8607c13; r1-vouchers.json ccfc1a76: total 1, the seed.
2. restore-plan.txt: written BEFORE R0, with folder, file name, company 100054 and Restore Data Path.
   - R0 = backups/R0/TDBK1800_100054.001, 25,107 B, sha256 7fd61324…6be8, 08:42:49Z.
3. P1 build r3-p1-build.json ff2ac161: batch bridge-e59d4156-2dfa-4d9b-a650-14c8b22ad0f3.
   - The first build call was refused voucher_schema_invalid: I sent side as debit/credit instead of Dr/Cr. Nothing was saved (r3-p1-build-note.txt).
   - Post: r3-p1-post-1.json 864e2d13 (pending, declared); clicked Yes; r3-p1-post-2.json f9a5b254 (approved, declared); r3-p1-post-3.json 3f179e52.
   - P1 result: posted_verified 1; counters created 1, all others 0; post_span_binding bound; last_vch_id 2.
   - post_location: {state target_only, target_moved true, companies_added [], companies_removed [], other_companies_moved [], target_voucher_step {before 1, after 2, step 1, reported_created 1, matches_created true}}. EXPECT met.
   - Dialog: r3-p1-dialog.jpg; windows log r3-p1-windows.log.
4. P2 build r4-p2-build.json fd8fee89: batch bridge-b98a7e4d-f3a9-4655-b572-ce75f3c8a7c6.
   - Post: r4-p2-post-1.json 63b33de4 (pending, declared); clicked Yes; r4-p2-post-2.json 5a81d4c5. This call posted directly (approval given while no call was waiting; declared).
   - P2 result: posted_verified 1; created 1, others 0; bound; last_vch_id 3.
   - post_location target_voucher_step {before 2, after 3, step 1, reported_created 1, matches_created true}; target_only. EXPECT met.
   - r4-vouchers.json 0f1a1b5d: total 3.
5. Restore of R0 over BRIDGE RESTORE LAB, exactly per restore-plan.txt.
   - Screens r5-1..r5-5.
   - Prompts quoted: "Accept ?" (Yes); "Company data folder (100054) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." (Replace). No repair, rewrite or migrate offer; only 100054 was selected.
   - r5-vouchers.json 9bad04ab: total 1 (the seed), voucher mark 1. EXPECT met.
6. Control: r6-verify-p2.json 0281954c.
   - P2 reads book_rolled_back: post_span_binding {state book_rolled_back, current_mark 1, expected_at_least 3}.
   - verification_status verification_incomplete; error import_reconciliation_required. EXPECT met; the control holds.
7. K1 keyed on Tally's screen: Receipt 10-Jul-2026, To Party A Cr 201.00 / By Synthetic Bank Ledger Dr 201.00, "Lab restore probe 1", automatic number.
   - Screenshot r7-k1-voucher-screen.jpg. Tally opened Bank Allocations on the bank leg (Cheque/DD, 201.00, inst date 10-Jul-26); I accepted it as shown (r7-k1-bank-allocation.jpg). The native post has no such allocation.
   - r7-vouchers.json 0deb3772: total 2. K1 = master_id 2, alter_id 2, guid …-00000002, number 2.
8. Verify:
   - r8-verify-p1.json f0f84f99: P1 reads **posted_verified** (counts.posted_verified 1), post_span_binding **bound** to master_id 2 / guid …-00000002, which is the hand-keyed K1. verification_status posted_verified. No rollback answer, because the mark (2) is not below P1's floor (1 + 1).
   - r8-verify-p2.json cb020523: P2 still book_rolled_back (current_mark 2, expected_at_least 3); verification_incomplete.
9. K2 keyed: Receipt 11-Jul-2026, To Party A Cr 999.00 / By Synthetic Bank Ledger Dr 999.00, "Lab restore hand 2" (bank allocation accepted the same way).
   - Screenshot r9-k2-voucher-screen.jpg.
   - r9-vouchers.json d875efce: total 3. K2 = master_id 3, alter_id 3, guid …-00000003, number 3.
   - r9-verify-p1.json 0b9bef3d: P1 still posted_verified, bound.
   - r9-verify-p2.json a70686ac: P2 **posted_divergent**, post_span_binding **bound** (to K2), verification_incomplete, import_reconciliation_required. The diff lists entries (expected 202 Dr Synthetic Bank Ledger / 202 Cr Party A; observed 999 Party A / -999 Synthetic Bank Ledger). The narration also differs ("Lab restore hand 2" vs "Lab restore probe 2") but is not listed in diffs.
10. R1 = backups/R1/TDBK1800_100054.001, 27,015 B, sha256 d422a112…658a, 08:56:05Z (r10-backup-R1.txt).

## Identity table (from r4, r7, r9)
| voucher | read | master_id | alter_id | guid | number | date | amount | narration |
|---|---|---|---|---|---|---|---|---|
| seed | r4 | 1 | 1 | …-00000001 | 1 | 20260701 | 1000.00 | Seed voucher |
| P1 (native) | r4 | 2 | 2 | …-00000002 | 2 | 20260710 | 201.00 | Lab restore probe 1 |
| P2 (native) | r4 | 3 | 3 | …-00000003 | 3 | 20260711 | 202.00 | Lab restore probe 2 |
| K1 (hand, after restore) | r7 | 2 | 2 | …-00000002 | 2 | 20260710 | 201.00 | Lab restore probe 1 |
| K2 (hand, after restore) | r9 | 3 | 3 | …-00000003 | 3 | 20260711 | 999.00 | Lab restore hand 2 |
(guid prefix 39e44d0b-e708-42d3-bb32-821d195b3d97)

- K1 took P1's MasterID, AlterID, GUID and voucher number: all four equal [verified].
- K2 took P2's MasterID, AlterID, GUID and voucher number: all four equal [verified].
- Readback differences, K1 vs P1: entry order (Party A first) and `party` (Party A vs Synthetic Bank Ledger). Amounts, ledgers, sides, date and narration are equal.

## verify_import answers
| step | P1 batch | P2 batch |
|---|---|---|
| 6 (after restore) | not called | book_rolled_back (mark 1 < 3), verification_incomplete |
| 8 (after K1) | posted_verified, bound (to K1) | book_rolled_back (mark 2 < 3) |
| 9 (after K2) | posted_verified, bound (to K1) | posted_divergent, bound (to K2), entries diff |

## Answer to #1050 [verified on one synthetic book, TallyPrime 7.1 Silver, Windows]
After a backup is restored and vouchers are keyed again, Tally reuses the MasterIDs, AlterIDs, GUIDs and numbers the native posts held.
- verify_import then binds the batches to those hand-keyed vouchers.
- A keyed voucher with the same compared content reads posted_verified: a false positive (P1/K1).
- One with different content reads posted_divergent (P2/K2).
- book_rolled_back is reported only while the voucher mark is below a post's floor (step 6, and P2 at step 8).
