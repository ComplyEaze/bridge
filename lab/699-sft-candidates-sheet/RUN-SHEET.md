# 699 SFT-013 candidates (#699): a new synthetic book of cash-receipt shapes, and its audit read. WRITE sitting, then READ-ONLY.

Not for merge. Written 25 Oct 2026. `SHA256.txt` lists every file here with its bytes; check it before step A0.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256
5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

THE COMPANY is `BRIDGE SFT LAB`. It does not exist yet: Phase A creates it. Record its company number and GUID once it exists.

Because the company is new, this item differs from LAB-COMMON in four places, and the stricter rule holds everywhere else:
1. **Start.** The item starts with no company loaded (C1).
2. **First act.** The first act is step A0, the backup of the whole data folder, then the creation of THE COMPANY. LAB-COMMON's C5
   backup of THE COMPANY does not apply, because there is no book to back up.
3. **Step 0.** C2 step 0 runs as soon as THE COMPANY exists and is loaded alone, and again before Phase B.
4. **Writes.** Every write in this item is keyed by hand on Tally's screens. Nothing is imported and nothing is sent to port 9001
   during Phase A, except C2's own reads.

The owner is at the PC for the whole sitting and approves every native window.

## Request files (Phase B)
They are added to this folder in a later commit once Phase A has given the GUID: requests/, each listed in SHA256.txt. They will be the audit
read's own requests for THE COMPANY over 1-Apr-2025 to 31-Mar-2026. Each is preflighted: its header ID equals the COLLECTION NAME it
defines.

**Phase B does not start until that commit is on this branch.**

## WHY
A clause 42 candidate list for Form 61A row SFT-013 (Rule 114E(2) Sl.11: cash received for a sale of goods or services) will be built
only after a CA has hand-worked it on a book that Tally itself produced. This book is that book. Each shape changes one thing:
- where the cash line and the party side of a receipt differ;
- the exact boundary;
- instalments, several bills, several parties;
- credits that are not sales;
- a refund and a Contra.

The bill-wise shapes also record what bill data the read itself carries.

## Phase A: setup (TallyPrime 7.2, licensed, Educational mode No)
A0. Take a FULL backup of Tally's data folder, BK-LR6, into a NEW empty local folder, before any keying:
   - a directory listing BEFORE shows the folder absent or empty;
   - record the backup's name, size and sha256;
   - a restore of BK-LR6 is the way back for everything below, made only on the run coordinator's word.

A1. Make no change to any existing company in this sitting.

A2. Create the company `BRIDGE SFT LAB`:
   - books from 1-Apr-2025 (FY 2025-26);
   - inventory off, GST off, no other feature;
   - no GSTIN and no PAN. None is needed. If Tally insists on one, STOP and report.

   Load it alone. Run C2 step 0 and record the company number and GUID (from the MARKS answer).

A3. Create these ledgers by hand. "Maintain balances bill-by-bill" is No on every ledger except `Party Bill`.

   | Ledger | Group |
   |---|---|
   | `Cash` | Cash-in-Hand |
   | `Bank SFT` | Bank Accounts |
   | `Sales SFT` | Sales Accounts |
   | `Party One`, `Party Two`, `Party Three`, `Party Four` | Sundry Debtors |
   | `Party Bill` | Sundry Debtors, with "Maintain balances bill-by-bill" = Yes |
   | `Capital SFT` | Capital Account |
   | `Loan SFT` | Loans (Liability) |
   | `Asset SFT` | Fixed Assets |
   | `Advance SFT` | Current Liabilities |

A4. Key these vouchers by hand, all in FY 2025-26, with synthetic amounts. Key each exactly as written: one line per ledger named,
   and nothing else. "In CASH" means the `Cash` ledger is debited for that amount.
   - S1:
     - Sales to Party One, Rs 3,00,000, 10-Jun-2025.
     - Receipt from Party One, Rs 3,00,000 in CASH, 12-Jun-2025.
   - S2:
     - Sales to Party Two, Rs 2,00,000, 10-Jul-2025.
     - Receipt from Party Two, Rs 2,00,000 in CASH, 12-Jul-2025.
   - S3:
     - Sales to Party Three, Rs 2,50,000, 1-Aug-2025.
     - Two Receipts from Party Three, both on 5-Aug-2025, both in CASH: Rs 1,50,000 and Rs 1,00,000.
   - S4:
     - Sales to Party One, Rs 4,00,000, 1-Sep-2025.
     - ONE Receipt from Party One, 5-Sep-2025: Rs 2,50,000 in CASH plus Rs 1,50,000 to Bank SFT.
   - S5: Receipt of Rs 2,50,000 in CASH, 10-Oct-2025, credited directly to `Sales SFT` (no party ledger).
   - S6: Receipt of Rs 5,00,000 in CASH, 1-Nov-2025, credited to `Capital SFT`.
   - S7:
     - Sales to Party Two, Rs 4,00,000, 1-Dec-2025.
     - ONE Receipt from Party Two, 5-Dec-2025: Rs 1,50,000 in CASH plus Rs 2,50,000 to Bank SFT.
   - S8:
     - Sales to Party Three, Rs 2,00,000.01, 1-Jan-2026.
     - Receipt from Party Three, Rs 2,00,000.01 in CASH, 5-Jan-2026.
   - S9:
     - Sales to Party Bill, Rs 5,00,000, 1-Feb-2026, bill New Ref `B9`.
     - Receipt from Party Bill, Rs 2,50,000 in CASH, 5-Feb-2026, Agst Ref `B9`.
     - Receipt from Party Bill, Rs 2,50,000 in CASH, 10-Feb-2026, Agst Ref `B9`.
   - S10:
     - Sales to Party Bill, Rs 2,50,000, 1-Mar-2026, New Ref `B10`.
     - Receipt from Party Bill, Rs 1,50,000 in CASH, 5-Mar-2026, Agst Ref `B10`.
     - Receipt from Party Bill, Rs 1,00,000 in CASH, 10-Mar-2026, Agst Ref `B10`.
   - S11:
     - Two Sales to Party Bill on 1-May-2025: Rs 1,50,000 (New Ref `B11A`) and Rs 1,50,000 (New Ref `B11B`).
     - ONE Receipt from Party Bill, Rs 3,00,000 in CASH, 5-May-2025, split Agst Ref `B11A` Rs 1,50,000 and Agst Ref `B11B`
       Rs 1,50,000.
   - S12:
     - Sales to Party One, Rs 1,50,000, 10-Nov-2025.
     - Sales to Party Two, Rs 1,50,000, 10-Nov-2025.
     - ONE Receipt, Rs 3,00,000 in CASH, 15-Nov-2025, crediting Party One Rs 1,50,000 and Party Two Rs 1,50,000.
   - S13: ONE Sales voucher, 20-Dec-2025, with `Cash` as the party ledger and `Sales SFT` Rs 2,40,000 (accounting invoice mode).
   - S14: Payment to Party One, Rs 50,000 in CASH (Cash credited), 20-Jun-2025.
   - S15:
     - Receipt of Rs 2,20,000 in CASH, 1-Jul-2025, credited to `Advance SFT`.
     - Receipt from Party Four, Rs 2,10,000 in CASH, 15-Jul-2025. Party Four has no sale at any time.
   - S16: Receipt of Rs 2,30,000 in CASH, 20-Aug-2025, credited to `Loan SFT`.
   - S17: Receipt of Rs 2,60,000 in CASH, 20-Sep-2025, credited to `Asset SFT`.
   - S18: Contra, 25-Sep-2025: `Bank SFT` debited Rs 2,70,000 and `Cash` credited Rs 2,70,000 (a cash deposit).

   Then take screenshots (C11: they stay on the lab PC): the Day Book for 1-Apr-2025 to 31-Mar-2026 (all vouchers, amounts visible),
   and the bill-wise Outstandings of Party Bill.

A5. Record, as a table, each voucher's number, type, date, every line's ledger and amount, and each bill reference, all as keyed.
   Add the Cash-in-Hand group's Cash Book totals for the year.

STOP:
- on any dialog other than the voucher and master screens' own accept (C7);
- on any feature prompt (GST, inventory, cost centres);
- if Tally shows a negative cash warning: report it, and continue only on the run coordinator's word.

## Phase B: the read (READ-ONLY)
B0. Run C2 step 0.

B1. Send the requests in requests/ one at a time (C3), each with a size bound of 2,000,000 bytes, in this order:
    company list, marks, company object, groups, ledgers, trial balance, voucher types, marks, census, vouchers, marks, company list.

EXPECT (recorded, not predicted):
- every answer is HTTP 200, STATUS 1, with no LINEERROR;
- the census and the vouchers part both list 34 vouchers: 11 for S1-S6 and 23 for S7-S18;
- the three marks answers are identical. If they are not, the book changed during the read: report it and repeat once.

B2. Run C10.

## Evidence and after
Push branch `lab/699-sft-lab-answers` with SHA256.txt and a MANIFEST (request, company, window, release). Send the SHA to the run
coordinator before pushing.

Restore: none (it is a new company). Keep THE COMPANY for a later re-read.

Afterwards a CA hand-works the clause 42 candidate list on this book, blind: the book only, with no software output. That list is the
expected answer, and only then is code written.
