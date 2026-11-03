# Results: row 2162-S3b, a Sales invoice's New Ref when the customer already has a bill of that name, 3 Nov 2026

**All three sub-rows read outcome (iii): Tally stored the invoice's New Ref as an Agst Ref against the existing bill of that name.** Each invoice imported cleanly (CREATED 1, ERRORS 0). Each read-back shows the customer's leg holding one allocation, named by the invoice number, `BILLTYPE` **Agst Ref**, -1051.00, never New Ref. Each read-back's allocation carries the existing bill's own date. Bills Receivable shows one bill of that name per customer, holding both amounts:
- **b1** (the bill is a Journal New Ref, open): `BP/26-27/0048`, 1-Jul-26, closing -500.00 → **-1551.00**.
- **b2** (the bill was settled in full by a Receipt): `BP/26-27/0049` reopened at its own date, 1-Jul-26, closing **-1051.00**. No new bill was made.
- **b3** (the bill is an opening bill): `BP/26-27/0050`, 31-Mar-26, closing -500.00 → **-1551.00**.

Every setup held, every EXPECT held, and the restore is byte-identical. [verified]

Sheet: `lab/2162-s3b-sheet` at c8cc28bc, folder `lab/2162-s3b/`. All 14 files equal to `SHA256.txt`, every request file UTF-16LE with a BOM. `M-marks.xml` and `B-bills.xml` are byte-equal to S3a's. The sheet's files were copied, hash-equal, to a run folder. No Bridge build. BRIDGE PILOT LAB (synthetic), loaded alone. Tally's screen shows TallyPrime **Silver**; the release, 7.2, is from `tally_status` on the same tally.exe earlier today. The Gateway also showed "(e) Data exceptions exist" beside the company, as in S2 and S3a. Started at 14:00 IST, as agreed. Four `bridge_mcp` processes ran idle under the desktop client, as in S1 to S3a. Whether they sent anything to Tally was not measured. [verified; the processes unverified]

## Sheet deviation and agreed setup steps
- **K3's opening-bill date (sheet deviation, agreed):** on the Bill-wise Breakup screen of `BRIDGE Walk-in BO`'s opening balance, Tally prefilled the bill date 31/3/2026. Typed 1-4-2026 as the sheet says, Tally kept the cursor on the date after Enter, with no message: an opening bill dates before the books begin (1-Apr-26). Nothing was saved (`shots/k3-date-held.png`, kept on the lab PC). On the recorded word the bill was dated **31-3-2026**, Tally's own default. So b3's s4 B EXPECT date reads 31-Mar-26 in place of 1-Apr-26; its name, sign and amount are unchanged. [verified]
- **Tally's current date (recorded setup step, agreed):** keying the Receipt's date moved Tally's current date to 2-Jul-2026. After K1–K3 the Gateway read **Thursday, 2-Jul-2026**; F2 at the Gateway set it to **Tuesday, 15-Sep-2026** before s4. Both are captured (`date-before-reset.png`, `date-after-reset.png`). So the invoices, dated 1 Aug 2026, were imported with the current date at 15-Sep-2026, not post-dated. [verified]
- **Leaving screens:** no Escape was pressed on any master or voucher screen.
  - After K2's Receipt, Go To > Alter Master > Ledger led to K3.
  - After K3 was accepted, the ledger list's Back row left it.
  - Ctrl+Q, once each after a title capture, closed the Master Alteration list and then the blank Receipt form. That landed on the Gateway with no prompt.

[verified]

## Keyed setup, as Tally showed it
| entry | type, number, date | lines | bill-wise screen |
|---|---|---|---|
| K1 | Journal No. 1, 1-Jul-26 | By `BRIDGE Walk-in BJ` 500.00 Dr; To `BRIDGE Seed Income` 500.00 | "Bill-wise Details for BRIDGE Walk-in BJ, Up to ₹500.00 Dr": New Ref `BP/26-27/0048`, 500.00 Dr |
| K2 | Journal No. 2, 1-Jul-26 | By `BRIDGE Walk-in BS` 500.00 Dr; To `BRIDGE Seed Income` 500.00 | New Ref `BP/26-27/0049`, 500.00 Dr |
| K2 | Receipt No. 5, 2-Jul-26 | To `BRIDGE Walk-in BS` 500.00 Cr; By `Cash` 500.00 | "Up to ₹500.00 Cr": Agst Ref; Pending Bills offered `BP/26-27/0049`, 1-Jul-26, 500.00 Dr, chosen; 500.00 Cr. BS's balance then 0.00 |
| K3 | Ledger `BRIDGE Walk-in BO` altered | Opening Balance (on 1-Apr-26) 500.00 Dr | "Bill-wise Breakup of BRIDGE Walk-in BO, Upto ₹500.00 Dr": 31/3/2026, `BP/26-27/0050`, due 31-Mar-26, 500.00 Dr |

- No prompt the sheet does not list came up.
- Each entry was captured before its Accept, and each Accept was the screen's own.

[verified]

## Steps and answers
| step | file | bytes | sha256 | counts |
|---|---|---:|---|---|
| s1 | M | 3,394 | 8a0c4268… | ALTVCHID 68, ALTMSTID 245 |
| BK | | | | one file, 90,051 B, sha256 5d0f5029…, 14:02 local, kept on the lab PC |
| s2 | R-0048, R-0049, R-0050 | 3,022 each | 947fbf36… | 0 `VOUCHER` elements each: the numbers are unused |
| s2 | B | 46 | 8d37111f… | `<ENVELOPE></ENVELOPE>`: 0 groups for each customer |
| s3 | L | 538 | ffc14ced… | CREATED 4, ALTERED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 0, no LINEERROR |
| s4 | V | 98,012 | 30117e5c… | below |
| s4 | B | 976 | 1f3ecd6a… | BJ: 1 group `BP/26-27/0048`, 1-Jul-26, `BILLCL` -500.00. BS: 0 groups. BO: 1 group `BP/26-27/0050`, 31-Mar-26, -500.00 |
| s4 | M | 3,396 | 97f279d7… | ALTVCHID 71, ALTMSTID 250 (above s1's) |
| s5 | I-0048 | 540 | e6540f65… | CREATED 1, ALTERED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 48, no LINEERROR |
| s5r | R, B, M | 95,836 / 978 / 3,396 | 0e01dbf4… / 4fef953f… / 5a561243… | below; M 72 / 250 |
| s6 | I-0049 | 540 | 93e33132… | CREATED 1, ERRORS 0, EXCEPTIONS 0, LASTVCHID 49 |
| s6r | R, B, M | 95,836 / 1,440 / 3,396 | 6496dd46… / 0a7521b8… / 9a369726… | below; M 73 / 250 |
| s7 | I-0050 | 540 | 200c17e1… | CREATED 1, ERRORS 0, EXCEPTIONS 0, LASTVCHID 50 |
| s7r | R, B, M | 95,940 / 1,442 / 3,396 | ac53ba5f… / a6d63b31… / 4cc537c6… | below; M 74 / 250 |
| s8 | M | 3,394 | 8a0c4268… | 68 / 245: **byte-identical to s1's** |

After each import, a capture of Tally's window was byte-identical to the Gateway capture taken at 15-Sep-2026: no dialog. [verified]

## s4 V, the keyed setup read natively
3 `VOUCHER` elements in 1–2 Jul 2026, all keyed by this row:
- **Journal 1, 20260701:** the `BRIDGE Walk-in BJ` leg (-500.00) holds 1 allocation with a `NAME`: `BP/26-27/0048`, New Ref, -500.00. 0 whitespace-only.
- **Journal 2, 20260701:** the `BRIDGE Walk-in BS` leg (-500.00) holds 1: `BP/26-27/0049`, New Ref, -500.00.
- **Receipt 5, 20260702:** the `BRIDGE Walk-in BS` leg (+500.00) holds 1: `BP/26-27/0049`, Agst Ref, 500.00.

Dr reads negative and Cr positive, as agreed. All three setups held. [verified]

## The read-backs
Each has 1 `VOUCHER`, dated 20260801, with `VCHGSTSTATUSISINCLUDED` Yes, `VCHGSTSTATUSISUNCERTAIN` No, `VCHGSTSTATUSISEXCLUDED` No. Each has 6 `ALLLEDGERENTRIES.LIST` legs, 1 for the sub-row's customer (-1051.00). On every other leg: 0 allocations with a `NAME` and 5 whitespace-only elements.

| sub-row | number, ALTERID, GUID | customer leg: allocations with a `NAME` | that allocation's `BILLDATE` (from the bytes) |
|---|---|---|---|
| b1 | BP/26-27/0048, 72, …-00000030 | 1: `BP/26-27/0048`, **Agst Ref**, -1051.00 | 20260701 |
| b2 | BP/26-27/0049, 73, …-00000031 | 1: `BP/26-27/0049`, **Agst Ref**, -1051.00 | 20260701 |
| b3 | BP/26-27/0050, 74, …-00000032 | 1: `BP/26-27/0050`, **Agst Ref**, -1051.00 | 20260331 |

b3's read-back is 104 B larger than b1's. Apart from the number, party, GUID, keys and the CMPINFO header counters, the difference is in the allocation: b3's carries `BILLCREDITPERIOD` 31-Mar-26, the opening bill's due date, where b1's is empty. [verified]

## Bills Receivable after each import
| after | BJ | BS | BO |
|---|---|---|---|
| s4 (setup) | 0048, 1-Jul-26, -500.00 | none | 0050, 31-Mar-26, -500.00 |
| s5r (b1) | 0048, 1-Jul-26, **-1551.00** | none | 0050, 31-Mar-26, -500.00 |
| s6r (b2) | 0048, 1-Jul-26, -1551.00 | **0049, 1-Jul-26, -1051.00** | 0050, 31-Mar-26, -500.00 |
| s7r (b3) | 0048, 1-Jul-26, -1551.00 | 0049, 1-Jul-26, -1051.00 | 0050, 31-Mar-26, **-1551.00** |

One group per customer in each; no second bill of the same name appeared. [verified]

## Outcomes
- **b1:** (iii). The New Ref was stored as Agst Ref against the open Journal bill. The bill now holds both amounts (-1551.00). It is also (ii)-like in Bills Receivable: one bill holding both.
- **b2:** (iii). Stored as Agst Ref against the settled bill, which reopened at its own date for the invoice's amount alone (-1051.00). No new bill was made.
- **b3:** (iii). Stored as Agst Ref against the opening bill, which now holds both amounts (-1551.00).

[verified]

## Decides
By the sheet's Decides line, (iii) in every sub-row means the read-back's allocation check (agent_import_invoice.rs:1476-1486 at e4440334) reads a difference: the party leg holds an Agst Ref, not the New Ref it requires. So this case fails closed in all three shapes. How the check names that difference to the user is for the code's reader. [the outcome verified; the code reading not this row's]

## Restore
- Before: the plan in `evidence/restore-plan.txt`; the backup's hash and its one-file folder checked; PILOT LAB shut.
- The Restore list showed BRIDGE PILOT LAB at 3-Nov-26 2:02 pm; selected alone and captured before accepting.
- Two prompts came, `Accept ?` (Yes) and the folder-exists prompt naming the company's own number (Replace), and no other.
- s8's marks are byte-identical to s1's.

[verified]

## Kept
- Every answer as Tally sent it (none holds the company number, a GSTIN or a PAN shape).
- `run.py`'s logs, the check output, the restore plan, `check.py`.
- On the lab PC only: the backup, the captures (the keyed screens, the date before and after, `k3-date-held.png`) and the data-folder listings, listed in `kept-local.txt`.

## The book now
BRIDGE PILOT LAB restored, marks byte-identical to s1. Shut, no company loaded, one tally.exe on 9001.

**Confidence:** one book, three customers, one sitting, 7.2: PARTIAL.
