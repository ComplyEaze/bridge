# Item 1c v2 (partial-landing probe): Windows lab, 12 Oct 2026

Outcome: **(C) created 2**. Tally created both vouchers, numbering the numberless Payment **1** under Manual numbering with "Prevent creating duplicate Voucher Nos." = Yes.

Build: bridge_mcp.exe 411f7cca80ca4a5fee543f4f44d14b911ce23e982169785b1762d8d50472e5a6 (#1516); posting ON, batch posting ON (extension settings: enable_writes true, enable_batch_post true).
Book: new synthetic BRIDGE PARTIAL LAB, company number 100055, GUID 22a3e632-d0b3-4715-82bb-b7b5b806e042, books from 20260401, loaded alone throughout.
- No GST, no inventory, automatic numbering.
- Ledgers: Synthetic Bank Ledger (Bank Accounts), Party A (Sundry Debtors, bill-by-bill No), Lab Charges (Indirect Expenses), Lab Seed Income (Indirect Incomes); Cash and P&L predefined (s1c-0-ledger-list.jpg).
- Seed: Receipt 1, 1-Jul-2026, Cash Dr / Lab Seed Income Cr 1,000.00, "Seed voucher".
- P0: backups/P0/TDBK1800_100055.001, 25,265 B, sha256 f0504b0c9b17d581ecaf41e3ec2dcae29cf3a981a82b49cf220b6eb4c00c7d48, 09:03:13Z.
CLICK MODE: lane (the Windows lab clicked Yes in the Bridge dialog with the mouse).

## Step 0 (look only; nothing saved): s1c-step0-finding.txt
- Payment (no Payment vouchers in the book): Automatic → Manual raised no prompt. The cursor went straight to "Prevent creating duplicate Voucher Nos." on the main screen, directly under Method: value No, editable (s1c-step0-payment-manual-prevent-field.jpg).
- Receipt (has the seed): Automatic → Manual raised "Changing the method of voucher numbering will affect voucher numbers of existing transactions. Continue?". Answered No; quit (s1c-step0-receipt-manual-prompt.jpg).
- F12 and More Details were not opened: the field was found on the main screen first.
- [inferred] Why v1 could not reach the field on BATCH LAB (32 Payments): the existing-transactions prompt fired on leaving Method, and the cursor then skipped past the field.

## The variable: V-1 (the item's first variant)
- Payment = Manual numbering, Prevent creating duplicate Voucher Nos. = Yes. Screenshot before saving: s1c-v1-payment-before-save.jpg.
- Saved with Ctrl+A; no prompt (no existing Payment vouchers), so no "update existing vouchers" prompt to answer.

## Build and post
- s1c-build.json: batch bridge-0d91d762-0d87-47f3-b0d6-8d695b63e2bd, 2 vouchers, sha256 d8a555d4…56d1.
  - P1C-R Receipt 2026-07-25: Dr Synthetic Bank Ledger 111.00 / Cr Party A 111.00, "Lab partial probe receipt".
  - P1C-P Payment 2026-07-25: Dr Lab Charges 222.00 / Cr Synthetic Bank Ledger 222.00, "Lab partial probe payment".
- s1c-post-1-2.json:
  - Call 1: pending (dialog_remaining_s 85), declared. The dialog appeared at 09:05:57.339Z (window-watch) and was fronted. It is one dialog, "ComplyEaze Bridge — post 2 vouchers?", with 2 per-voucher lines: "Receipt 20260725 111 "Party A" "Lab partial probe receipt"" and "Payment 20260725 222 "Lab Charges" "Lab partial probe payment"" (s1c-dialog.jpg).
  - Clicked Yes at about 09:06:40Z. Call 2: approved, declared.
- s1c-post-3.json: posted_verified, counts.posted_verified 2.
- Every voucher: status posted_verified.
- post_span_binding: state bound ("Each voucher was matched to the Tally voucher this post created."); no code.
- post_location: {state target_only, target_moved true, companies_added [], companies_removed [], other_companies_moved [], target_voucher_step {before 1, after 3, step 2, reported_created 2, matches_created true}}. masters_after_post.batch_step matched.
- Tally line error texts: none (line_error_count 0; no tally_line_errors).
- dispatch.response.outcome (whole): {"application_status":"not_reported","counters":{"altered":0,"cancelled":0,"counter_presence":{"altered":true,"cancelled":true,"created":true,"deleted":true,"errors":true,"exceptions":true,"ignored":true},"created":2,"deleted":0,"errors":0,"exceptions":0,"ignored":0,"line_error_count":0},"exceptions_were_reported":true,"last_vch_id":3}. last_vch_id is present (3).
- s1c-verify.json (verify_import once): posted_verified, verified_total 2, bound, state previous_attempt_reconciled.
- s1c-vouchers.json (2026-07-25): total 2, complete.
  - Receipt: master_id 2, alter_id 2, guid …-00000002, voucher_number "2".
  - Payment: master_id 3, alter_id 3, guid …-00000003, voucher_number **"1"**.
- Day Book on Tally's screen for 25-Jul-26 shows Payment Vch No. 1 and Receipt Vch No. 2 (s1c-daybook-0725.jpg).

## Afterwards
- Payment set back to Automatic.
  - "Changing the method … existing transactions. Continue?" → Yes (s1c-automatic-change-prompt.jpg).
  - Numbering behaviour kept as Retain Original Voucher No. (s1c-payment-back-to-automatic.jpg).
  - Ctrl+A raised "Are you sure you want to update the voucher numbers of existing vouchers? Y: Accept Master & Update Vouchers / N: Back" → Y (s1c-automatic-update-prompt.jpg).
- Read afterwards (s1c-after-automatic-vouchers.json): the Payment is still number 1, alter_id 3. Voucher mark 3 unchanged; master mark 214 → 216.
- P1: backups/P1/TDBK1800_100055.001, 27,271 B, sha256 067fc50a26ecb59f6a931aa7ec07e02d4ae985c42a0a8cf753e662f04042af6b, 09:10:55Z.

## Reading
- The question (can a batch of two make Tally create only one?) is not answered by this variable: a numberless Payment posted to a Manual, duplicates-prevented Payment type was created and numbered 1, so nothing was refused.
- Unlike #1393's Journal (created 0, exceptions 1), Tally numbered this Payment itself. [inferred] On this release, Manual + prevent duplicates does not refuse a numberless Payment.
- Not measured: whether a duplicate number would be refused.
