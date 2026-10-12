# Item 2-NARR (narration bytes, native vs hand-keyed) — Windows lab, 12 Oct 2026

Build: bridge_mcp.exe 411f7cca80ca4a5fee543f4f44d14b911ce23e982169785b1762d8d50472e5a6. TallyPrime 7.1 Silver, licensed, not Education (`f0-status.json`).
CLICK MODE: lane. Every Bridge dialog's Yes was clicked with the mouse; there were 4 single-voucher posts.

Book: NEW synthetic BRIDGE FINGERPRINT LAB, company number 100057, GUID 18602d61-aa94-4f45-ae49-44ecf55f280c.
- Books from 1-Apr-2026; no GST, no inventory, automatic numbering. Loaded alone throughout.
- Ledgers:
  - Synthetic Bank Ledger (Bank Accounts)
  - Lab Charges (Indirect Expenses)
  - Round Off (Indirect Expenses, Type of Ledger "Not Applicable")
  - Party C (Sundry Creditors, bill-by-bill No)
  - Lab Seed Income (Indirect Incomes)
- Seed: Receipt No. 1, 1-Jul-2026, Cash Dr / Lab Seed Income Cr 1,000.00, "Seed voucher".
- Item 2 is to reuse this company.

Backups:
| Backup | File | Bytes | sha256 | Written |
|---|---|---|---|---|
| F0 | TDBK1800_100057.001 | 25,408 | 08a91e2b60665f808d434f9eecdcd582c075fce73a7748b7af15fe9966d399ca | 10:51:03Z |
| F1 | TDBK1800_100057.001 | 29,214 | c8f0f7092c497b9f26fecce36f5072ef7254f7e9da0b0eaf674eaa9547e5234e | 11:09:27Z |

## M1 — native vs hand-keyed, 1 Sep 2026

**M1a native posts** (build, post call 1 pending, mouse Yes, call 2). Each was as expected: created 1, others 0, posted_verified 1, post_span_binding bound.
- R10-P1 Payment (`m1a-p1-*.json`): Dr Lab Charges 1000.00 / Dr Round Off 0.40 / Cr Synthetic Bank Ledger 1000.40, "R10 payment one".
  - Payment No. 1, GUID …-00000002, master 2, alter 2. Clicked 10:52:43Z.
  - The build warned that multi-entry Payments rest on narrower evidence (#466).
- R10-J1 Journal (`m1a-j1-*.json`): Dr Lab Charges 500.00 / Cr Party C 499.75 / Cr Round Off 0.25, no narration field.
  - Journal No. 1, GUID …-00000003, master 3, alter 3. The dialog showed `Narration: (none)`. Clicked 10:54:27Z.

**M1b hand-keyed on Tally's screen** (double-entry voucher screens):
- **Period fix:** the first attempt to date the Payment 1-9-2026 was refused by a Tally input toast: "Date cannot be above the current period (1-Sep-26)". The company's Alt+F2 period was 1-7-2026..31-7-2026.
  - Change Current Date (F2) would not take 1-9-2026 or 31-8-2026, so I cancelled it.
  - I set the period (Alt+F2) to 1-4-2026..31-3-2027, a view setting. Nothing was saved by any of these steps.
- **Payment (No. 2, GUID …-00000004):** By Lab Charges 1000.00, By Round Off 0.40, To Synthetic Bank Ledger 1000.40, narration "R10 payment one".
  - Tally auto-filled the next line twice; I changed one auto-filled line to By Round Off.
  - **Tally asked for Bank Allocations** for Synthetic Bank Ledger (Lab Charges, Cheque, 1,000.40, inst. date 1-Sep-26). Defaults accepted (`m1b-p-bank-allocation.jpg`).
- **Journal (No. 2, GUID …-00000005): deviation.** After By Lab Charges 500, Tally auto-filled "To Round Off 500.00". I changed the ledger to Party C and then pressed Ctrl+A, which SAVED the voucher as Lab Charges Dr 500 / Party C Cr 500 (`m1b-j-MISSAVED-before-alter.jpg`).
  - I corrected it by altering the same voucher (Day Book → Enter) to Party C 499.75 + To Round Off 0.25, narration left blank (`m1b-j-voucher-after-alter.jpg`).
  - Its alter_id (6) therefore reflects one alteration.

**M1c reads** (vouchers 2026-09-01..2026-09-01):
- `m1-vouchers-1.json` (11:01:32Z) and `m1-vouchers-2.json` (11:01:52Z): state complete, **total 4** each.
- The two reads have the **same response_sha256 fb1bdcda3e0cdee86a7643f99fd98df3054d6ee7d242fd1564b248513a8a1d31**. Leg order and every item field are identical across the reads.

| Voucher | GUID | No. | alter | Legs as returned (ledger, signed amount, is_deemed_positive) | narration (as returned; UTF-8 hex) |
|---|---|---|---|---|---|
| Payment native R10-P1 | …02 | 1 | 2 | Lab Charges -1000.00 Yes; Round Off -0.40 Yes; Synthetic Bank Ledger 1000.40 No | "R10 payment one" (523130207061796d656e74206f6e65) |
| Journal native R10-J1 | …03 | 1 | 3 | Lab Charges -500.00 Yes; Party C 499.75 No; Round Off 0.25 No | "" (empty string, key present) |
| Payment hand | …04 | 2 | 4 | Lab Charges -1000.00 Yes; Round Off -0.40 Yes; Synthetic Bank Ledger 1000.40 No | "R10 payment one" (same bytes) |
| Journal hand | …05 | 2 | 6 | Lab Charges -500.00 Yes; Party C 499.75 No; Round Off 0.25 No | "" (empty string, key present) |

Native/hand pairs, field by field:
- Equal in every returned field except guid, master_id, alter_id and voucher_number: same leg order, signed amounts, is_deemed_positive (Round Off included), party, date, narration bytes, cancelled/optional/post_dated/is_invoice.
- The hand Payment carries a bank allocation (Cheque); `bill_allocations` is empty for both, and the vouchers tool does not return bank allocations.
- Bridge itself flags each pair as `unrelated_duplicates_in_window` (accounting_fingerprint 95d8753c… for the Payments, fbad87be… for the Journals) in the later post results.

**Absent vs "" (Journal with no narration):** as returned by the vouchers tool, both the native Journal (narration field omitted in the build) and the hand Journal (narration left blank) read `"narration": ""`. The key is present with an empty string, not null or absent.
- Not measured: Tally's raw XML for that tag. The tool normalises, and no raw request is in scope for this item.

## M2 — spaces in a narration

**M2a, hand-keyed (alter the hand Payment No. 2):**
- **Trailing space:** End, Space → "R10 payment one ", saved (bank allocation screen re-shown on the way, accepted unchanged). `m2a-vouchers.json` (11:02:52Z; repeated 11:03:02Z with the identical response_sha256 58c2503c…):
  - narration returned "R10 payment one", **with no trailing space**; alter_id 4 → 7.
  - The raw window part grew from 183,526 to 183,528 B (+2 B, one UTF-16 code unit).
  - Re-opened in Tally's alteration screen, End put the cursor directly after "one" (`m2a-screen-reopened-end-cursor.jpg`), so Tally's own screen also shows no trailing space.
  - [inferred] Tally trims a trailing space on save; what the +2 B is was not identified.
- **Doubled space:** cleared and retyped "R10  payment one", saved. `m2b-vouchers.json` (11:04:20Z): narration "R10  payment one" (52313020**20**7061796d656e74206f6e65), **double space kept**; alter_id 8; window part 183,532 B (+4).

**M2b, native posts:**
- **R10-P2**, Payment Dr Lab Charges 1001.00 / Cr Synthetic Bank Ledger 1001.00, narration "R10 trailing " (`m2c-p2-*.json`).
  - The build was not refused. The saved hand-import file holds "R10 trailing [BRIDGE:…]", with the space kept before the marker.
  - **The approval dialog displayed `Narration: "R10 trailing"` with no space before the closing quote** (`m2c-p2-dialog.jpg`).
  - Yes clicked 11:06:08Z; created 1, posted_verified, bound: Payment No. 3, GUID …-00000006.
- **R10-P3**, the same with 1002.00, narration "R10  double" (`m2c-p3-*.json`).
  - The dialog showed `"R10  double"` with both spaces. Yes clicked 11:08:00Z; created 1, posted_verified, bound: Payment No. 4, GUID …-00000007.
- `m2c-vouchers.json` (11:08:38Z, complete, **total 6**):
  - P2 narration "R10 trailing" (52313020747261696c696e67): **trailing space not present**.
  - P3 narration "R10  double" (5231302020646f75626c65): **double space kept**.

## Answers for #1608
1. **Is a native post's narration stored byte for byte?**
   - Doubled spaces: yes. "R10  double" is read back exactly.
   - Trailing space: no, as read. "R10 trailing " reads back as "R10 trailing", and the approval dialog already showed it without the trailing space. Not established: whether Bridge strips it before sending or Tally strips it on import. The sent request body was not captured.
2. **Does Tally keep a hand-keyed narration's spaces?** Doubled spaces: yes. Trailing space: no, both as read and on Tally's own screen.
3. **No narration:** native (omitted) and hand (blank) both read `""` through the vouchers tool.

## Verified / not measured
- Verified: M1 totals 4 / 4 (two identical reads); native/hand pairs equal in leg order, signed amounts, is_deemed_positive and narration bytes; double space kept (hand and native); trailing space absent as read (hand and native); empty-string narration for both Journals.
- Not measured: Tally's raw NARRATION tag bytes (tool-normalised reads only); where the native trailing space is lost; bank-allocation fields in the vouchers read.
- Deviations: the hand Journal was mis-saved once and corrected by alteration; the period was changed to reach 1-Sep.
