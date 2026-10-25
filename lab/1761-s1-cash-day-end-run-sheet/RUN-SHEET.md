# #1761 slice 1: a cash ledger's day-end balances (captures before any code)

Not for merge. One item, on one NEW synthetic company. `LAB-COMMON.md` in this folder (v3, copied byte for byte from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full; where this sheet and it differ, the stricter holds. THE COMPANY below is BRIDGE CASHDAY LAB.

**WHY.** Slice 1 of #1761 builds a Contra from the cash box only when the cash book had that cash on every day from the deposit to the end of the book. It projects day-end balances from the ledger's opening balance plus the voucher window. Before any code, it needs captured shapes:
- a cash ledger with vouchers on several days;
- a day-end credit balance;
- a Contra out of cash and a Contra into cash;
- an optional, a cancelled and a post-dated voucher on cash;
- two cash ledgers.

It also needs Tally's own day-end figure for each day, to test the projection against. The voucher window does not filter the flags: Tally exports optional, cancelled and post-dated vouchers alongside real ones, and its reports leave them out. The trial balance to each day gives that day-end figure from Tally itself.

**WRITES.** Masters and vouchers keyed BY HAND on Tally's screen, on THE COMPANY only. Nothing is posted or imported through Bridge or port 9001. Every request below is a READ.

**OWNER steps** are marked [OWNER]. Everything else is the operator's.

## Before the sitting
1. [OWNER] Tally is LICENSED on the lab PC. These dates fall on days other than 1, 2 and 31, which Education mode refuses. Educational mode anywhere on screen = STOP (C1).
2. Check every file against `SHA256.txt` (Get-FileHash). A difference = STOP, nothing is sent.
3. [OWNER] Go-ahead for this item.

## The company (new)
BRIDGE CASHDAY LAB, books beginning 1 April 2025, no inventory, no GST. If a company or a data folder of this name already exists: STOP, report it, and reuse or overwrite nothing. Masters, by hand:

| ledger | under | opening |
| --- | --- | --- |
| Cash (Tally's own) | Cash-in-Hand | 1,000.00 Dr |
| BRIDGE Petty Cash | Cash-in-Hand | none |
| BRIDGE Bank A | Bank Accounts | 10,000.00 Dr |
| BRIDGE Customer A | Sundry Debtors | none |
| BRIDGE Office Expenses | Indirect Expenses | none |
| BRIDGE Capital | Capital Account | 11,000.00 Cr |

Record each ledger's screen (screenshot) after its accept. Then backup **BK1** (C5) into a NEW empty folder `C:\bridge-lab\1761-s1\backups\BK1\`. BK1 holds the masters with no voucher.

## Vouchers (by hand, in this order, Tally's current date set to 12 April 2025 for the keying)
Set Tally's current date (F2 at the Gateway) to 12-04-2025 before voucher 1, and record the date it showed before the change. It is set back at C10.

| # | date | type | entries | how it is marked |
| --- | --- | --- | --- | --- |
| 1 | 03-04-2025 | Receipt | Cash Dr 5,000.00 / BRIDGE Customer A Cr | ordinary |
| 2 | 04-04-2025 | Payment | BRIDGE Office Expenses Dr 1,500.00 / Cash Cr | ordinary |
| 3 | 05-04-2025 | Contra | BRIDGE Bank A Dr 4,000.00 / Cash Cr | ordinary (cash out to bank) |
| 4 | 05-04-2025 | Payment | BRIDGE Office Expenses Dr 800.00 / Cash Cr | ordinary |
| 5 | 07-04-2025 | Contra | Cash Dr 2,000.00 / BRIDGE Bank A Cr | ordinary (cash in from bank) |
| 6 | 08-04-2025 | Payment | BRIDGE Office Expenses Dr 600.00 / Cash Cr | OPTIONAL: mark it optional on the voucher screen before accepting (record the key used) |
| 7 | 09-04-2025 | Payment | BRIDGE Office Expenses Dr 700.00 / Cash Cr | accept as ordinary, then open it again and CANCEL it with Tally's own cancel action (record the key and the confirmation text; answer only that confirmation) |
| 8 | 10-04-2025 | Contra | BRIDGE Petty Cash Dr 500.00 / Cash Cr | ordinary (cash to the second cash ledger) |
| 9 | 11-04-2025 | Payment | BRIDGE Office Expenses Dr 200.00 / BRIDGE Petty Cash Cr | ordinary |
| 10 | 25-04-2025 | Payment | BRIDGE Office Expenses Dr 900.00 / Cash Cr | POST-DATED: mark it post-dated on the voucher screen (record the key used) |

After each voucher, a screenshot of the screen before accepting (the narration left empty). A marking option the screen does not offer, or any prompt other than the voucher's own accept and voucher 7's cancel confirmation = C7, STOP. Then backup **BK2** (C5) into a NEW empty folder `C:\bridge-lab\1761-s1\backups\BK2\`. BK2 is the state the reads are taken on.

## Reads (raw requests, C3, one at a time; each answer kept whole with its size and sha256)
Step 0 (C2) first. Size bound for every read here: 200,000 bytes; the marks read 20,000.

| step | request file | what it captures |
| --- | --- | --- |
| R1 | `requests/r01_marks.request.utf16le.xml` | ALTVCHID and ALTMSTID (C4) |
| R2 | `requests/r02_ledgers_fy.request.utf16le.xml` | every ledger's PARENT and OPENINGBALANCE |
| R3 | `requests/r03_voucher_window_april.request.utf16le.xml` | every voucher of April with ISCANCELLED, ISOPTIONAL, ISPOSTDATED and all ledger entries (NEW SHAPE for this company: screenshot after, C3) |
| R4 to R14 | `requests/r04` ... `r14_trial_balance_to_<day>` | the trial balance from 1 April to each day: 2, 3, 4, 5, 7, 8, 9, 10, 11, 25 and 30 April (TBALOPENING, DEBITTOTALS, CREDITTOTALS, TBALCLOSING per ledger) |
| R15 | `requests/r01_marks.request.utf16le.xml` again | marks unchanged since R1 |

Every request's `ID` names its own collection (preflighted); each is a committed request with only the company and dates changed, listed in `SHA256.txt` with its source.

## EXPECT (recorded, not judged in the lab)
- R3 holds exactly ten vouchers: vouchers 1 to 10, every GUID beginning with THE COMPANY's.
  - Voucher 6: ISOPTIONAL Yes.
  - Voucher 7: ISCANCELLED Yes, with no ledger entries or empty ones; record which.
  - Voucher 10: ISPOSTDATED Yes.
  - All the others: the three flags No.
- The Cash day-end balances the slice must reproduce, from opening 1,000.00 Dr and the ordinary vouchers only:

  | day | Cash | Petty Cash |
  | --- | --- | --- |
  | 2 | 1,000.00 Dr | 0 |
  | 3 | 6,000.00 Dr | 0 |
  | 4 | 4,500.00 Dr | 0 |
  | 5 | 300.00 Cr | 0 |
  | 7 | 1,700.00 Dr | 0 |
  | 8 | 1,700.00 Dr | 0 |
  | 9 | 1,700.00 Dr | 0 |
  | 10 | 1,200.00 Dr | 500.00 Dr |
  | 11 | 1,200.00 Dr | 300.00 Dr |

  - Day 5 is the day-end credit balance.
  - Days 8 and 9 show the optional and cancelled vouchers leaving the balance alone, if Tally excludes them.
- **TBALCLOSING of Cash in R4 to R12** is what Tally says for each day; record it as Tally gives it, sign included. A difference from the table above is a finding, not a STOP.
- **R13 and R14** (to 25 and 30 April): whether the post-dated voucher 10 is counted, with Tally's current date at 12 April. The expectation is unknown; record it.

## The book after each step
- After BK1: the masters only.
- After the vouchers: vouchers 1 to 10 as above.
- The reads change nothing (R15's marks equal R1's).

THE COMPANY stays on the lab PC: it is synthetic and new. A restore happens only on the run coordinator's word after a STOP. The fallback is BK2 for a read problem and BK1 for a keying problem.

## Close and evidence
- **C10:** set Tally's current date back to the one recorded before voucher 1, Shut Company, and leave no company loaded.
- **C11:** evidence to a new branch `lab/1761-s1-cash-day-end-answers`, after the pre-push scan and [OWNER] yes. It holds:
  - the fifteen answers with their headers, sizes and sha256;
  - the R3 voucher table and the R4 to R14 Cash and Petty Cash closing table;
  - the notes;
  - the backup names, sizes and hashes.

  Screenshots stay on the lab PC, listed by name, size and sha256.
