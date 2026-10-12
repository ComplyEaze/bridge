# 724-M3: a second numberless voucher into a Manual type, and the Journal type stored in #1393's backup (Windows lab, 12 Oct 2026)

Synthetic only. Build: bridge_mcp.exe sha256 411f7cca80ca4a5fee543f4f44d14b911ce23e982169785b1762d8d50472e5a6, the same at step 0 and at the end. TallyPrime 7.1 Silver, licensed, not Education.
- The books are as 724-M left them:
  - N1 "BRIDGE MANUAL NUMBERING LAB 1" (100058);
  - N2 "BRIDGE MANUAL NUMBERING LAB 2" (100059).
- Each company was the only one loaded while in use. No voucher-type setting was changed.
- Every post was a single voucher with no voucher_number and one approval. The approval dialog said "Voucher number: Tally assigns it", and Yes was clicked with the mouse.
- The tool results are kept in full as JSON. In the build results only the local path's user folder is replaced by `<userprofile-local>`. Two results came back as tool refusals; their bodies are kept verbatim.

| Step | Company, type, setting | Outcome | Counters | Number Tally gave |
|---|---|---|---|---|
| s4 | N2, Payment, Manual, duplicates allowed | **B** | created 1, others 0, posted_verified, bound | **"1"**, the same as the first Payment |
| s5 | N1, Journal, Manual, duplicates prevented | **B** | created 0, exceptions 1, errors 0, line_error_count 0, reconciliation_required | none; refused whole |
| s6 | #1393 backup, BRIDGE REFUSAL LAB (100048), Journal | read only | none | Manual; prevent duplicates **Yes** |

## Step 0 (N2)
- **Payment type screen:** Manual, prevent duplicates No.
  - Voucher types have no display-only route, so the alteration screen (Chart of Accounts > Voucher Types) was used.
  - **Deviation:** the first Escape cleared the Name field on screen. Ctrl+Q then quit with no prompt, and nothing was saved: a marks read showed ALTMSTID 213, unchanged since 724-M.
- tally_status and list_companies: only N2.
- **Backup N2-B1:** TDBK1800_100059.001, 29,688 B, d2c0a769b88fb706decdb60f56e59f3c39bca3254cf01a72006024903fea2f0f.
  - A first attempt added a second company to the list by a stray Enter. It was quit before Accept, and nothing was written.
- **vouchers, July:** Receipt "1" and Payment "1".

## s4 (N2): B, the silent duplicate
- Payment 16-Jul-2026, Dr Lab Charges / Cr Lab Bank 11.00, narration "724-M s4".
- Post: created 1; altered, cancelled, deleted, errors, exceptions and ignored all 0; posted_verified; post_span_binding bound; target step 2→3; **voucher_number "1"**.
- verify_import: posted_verified.
- vouchers, July: two Payments, **both numbered "1"** (724-M s3 on 15-Jul and 724-M s4 on 16-Jul), and Receipt "1".

## s5 (N1): B, refused whole
- **Journal type screen:** Manual, prevent duplicates Yes. Quit with Ctrl+Q only: no prompt, nothing saved.
- tally_status and list_companies: only N1.
- **Backup N1-B1:** TDBK1800_100058.001, 30,483 B, 7088d9c403428a680d4ccc51e8ddb871c678a90ca461c410b2df7fdb4f88f430.
- vouchers before: Receipt "1", Payment "1", Journal "1".
- Posted: Journal 16-Jul-2026, Dr Lab Charges / Cr Suspense 11.00, narration "724-M s5".
- **Post call 2 (a refusal):**
  - created 0, exceptions 1, errors 0, line_error_count 0;
  - response_not_clean, state reconciliation_required, error import_reconciliation_required;
  - post_span_binding refused (span_counters_not_clean); target step 3→3;
  - row status `tally_reported_not_created`.
- **Tally's error text: none.** There was no LINEERROR and no tally_line_errors. Nothing was re-sent.
- **verify_import (a refusal):** verification_incomplete; the row's status is `sent_not_attributed`, a different label from post_import's for the same row.
- vouchers after: still 3, with no second Journal; voucher mark 3.

## s6: #1393's backup (read only)
- `restore-plan.txt` was written before any restore step.
- The backup is `TDBK1800_100048_1.001`: 27,354 B, written 10-Oct-2026 11:07 IST, sha256 f3343cadf1ba17861519062f8a29653cd51d74ca2682deb224ca5be2d8610bb9.
  - It was copied byte-identical into its own source folder. The older TDBK1800_100048.001 in the same original folder was not used.
- Restored into a NEW, empty scratch data folder, never the live data path.
  - The restore screen matched the plan: destination the scratch folder, source the copy, the only company BRIDGE REFUSAL LAB (100048) backed up 10-Oct-26 11:07.
  - The only dialog was Tally's expected "Accept ?". The live 100048 folder is untouched.
- The company was loaded alone from the scratch folder. Its Journal type shows **Method of Voucher Numbering: Manual** and **Prevent creating duplicate Voucher Nos.: Yes**.
- Quit with Ctrl+Q, no prompt. The company was shut; the scratch folder is left in place.

## What it decides (per the item)
- **s4 B:** a numberless post into a Manual type with duplicates allowed silently repeats a number ("1" twice), and both vouchers read posted_verified.
- **s5 B:** with duplicates prevented, the repeated number becomes a loud refusal. Tally creates nothing and gives no error text, and the row stays reconciliation_required.
- **s6 Manual + prevent Yes:** #1393's refusal is consistent with s5 B.

## Not covered
- Receipt and Contra under Manual numbering.
- A batch of several vouchers.
- Gold edition.
- The published 0.5.1 binary.
