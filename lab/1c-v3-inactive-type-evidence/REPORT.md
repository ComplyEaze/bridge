# Item 1c-v3 (a voucher type made inactive between build and post): Windows lab, 12 Oct 2026

Outcome: **(C) created 2.** Tally created both vouchers. The Payment went into a Payment type saved as "Activate this Voucher Type: No"; it was numbered 2 and is posted_verified and bound.

Build: bridge_mcp.exe 411f7cca80ca4a5fee543f4f44d14b911ce23e982169785b1762d8d50472e5a6 (#1516), with posting and batch posting on. TallyPrime 7.1 Silver, licensed.

Book: synthetic BRIDGE PARTIAL LAB (from 1c v2), company number 100055, GUID 22a3e632-d0b3-4715-82bb-b7b5b806e042, books from 20260401, loaded alone throughout.

CLICK MODE: lane. The Windows lab clicked Yes in the Bridge dialog with the mouse.

## Before
- restore-plan.txt was written first.
- Q0: backups/Q0/TDBK1800_100055.001, 27,271 B, sha256 cde9bedeea09092d7beb3c1f9c099fe0a78940119727e2b33c4e8b6d40fde51c, 10:29:43Z.

## Step 0 (look only; nothing saved)
- Gateway > Alter > Voucher Types: the list carries a "Show Inactive" button.
- The Payment type's main screen has "Activate this Voucher Type : Yes", directly under Abbreviation (s1c3-step0-payment-activate-field.jpg). The option is offered, so the item continued.
- F12 and More Details were not needed. The screen was left with Ctrl+Q, with no prompt and nothing changed.

## Build (Payment still active)
s1c3-build.json: batch bridge-68b99b42-cadd-486c-853e-98abdb0713d1, 2 vouchers, file sha256 a8e28808…a081, built 10:30:24Z.
- P1C3-R Receipt 2026-07-26: Dr Synthetic Bank Ledger 121.00 / Cr Party A 121.00, "Lab inactive-type probe receipt".
- P1C3-P Payment 2026-07-26: Dr Lab Charges 232.00 / Cr Synthetic Bank Ledger 232.00, "Lab inactive-type probe payment".

## The one variable (after the build, before the post)
- Alter > Voucher Type > Payment: Activate this Voucher Type Yes → **No**. No other field changed; Method stayed Automatic (s1c3-payment-inactive-before-save.jpg).
- Ctrl+A raised **no prompt of any kind**.
- Tally returned to the List of Voucher Types, where "Payment" is still listed among the active types (s1c3-after-save-type-list.jpg).
- After the post, the Payment type re-opened shows "Activate this Voucher Type : No" (s1c3-payment-inactive-after-post.jpg). The setting was saved and in force during the post.

## Post
s1c3-post-1-2.json:
- **Call 1:** pending (dialog_remaining_s 85); not dispatched; attempt not recorded.
  - Dialog: "ComplyEaze Bridge — post 2 vouchers?", one dialog with 2 per-voucher lines (s1c3-dialog.jpg):
    - 'Receipt 20260726 121 "Party A" "Lab inactive-type probe receipt"'
    - 'Payment 20260726 232 "Lab Charges" "Lab inactive-type probe payment"'
  - Yes clicked with the mouse at 10:32:37Z.
- **Call 2:** posted_verified; counts.posted_verified 2.
- Every voucher's status: posted_verified.
- post_span_binding: state bound ("Each voucher was matched to the Tally voucher this post created."), with no code.
- post_location: target_only, target_moved true, no companies added, removed or moved; target_voucher_step {before 3, after 5, step 2, reported_created 2, matches_created true}. masters_after_post.batch_step matched.
- Tally line error texts: none (line_error_count 0).
- dispatch.response.outcome (whole): {"application_status":"not_reported","counters":{"altered":0,"cancelled":0,"counter_presence":{"altered":true,"cancelled":true,"created":true,"deleted":true,"errors":true,"exceptions":true,"ignored":true},"created":2,"deleted":0,"errors":0,"exceptions":0,"ignored":0,"line_error_count":0},"exceptions_were_reported":true,"last_vch_id":5}
- pre_import_mark: company_high_water value 3, master_value 216.

**verify_import** (once, s1c3-verify.json): posted_verified, verified_total 2, bound, dispatch state previous_attempt_reconciled.

**vouchers 2026-07-26..2026-07-26** (s1c3-vouchers.json): state complete, total 2; snapshot voucher_alter_id 5, master_alter_id 218.

| Voucher | master_id | alter_id | GUID | Number |
|---|---|---|---|---|
| Receipt P1C3-R | 4 | 4 | 22a3e632-…-00000004 | 3 |
| Payment P1C3-P | 5 | 5 | 22a3e632-…-00000005 | 2 |

## Afterwards
- Payment set active again (Activate Yes, Method Automatic unchanged; s1c3-payment-active-again-before-save.jpg). Ctrl+A raised no prompt.
- restore-plan.txt was updated. Q1: backups/Q1/TDBK1800_100055.001, 28,099 B, sha256 5f50a2da7c4eddc8a4dbe41a46ce0c89ab65bee64f5ca58b57a85c0fd386ceb7, 10:35:21Z.

## Reading
- The question (can a batch of two make Tally create only one?) is not answered by this variable. An inactive Payment type does not refuse a native import into it on this release: outcome C, created 2.
- [inferred] "Activate this Voucher Type: No" hides or limits the type on Tally's own entry screens, but does not gate an XML import.
- Not measured: whether the inactive type is offered in Tally's own voucher entry, since that was not opened while it was inactive.
- Noted: deactivating the type did not move the company's master mark before the post (pre_import_mark master_value 216, the same as after 1c v2's last change).
