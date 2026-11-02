# Results: #1761 slice 1, a cash ledger's day-end balances, 2 Nov 2026

**Every EXPECT line holds. R3 holds exactly the ten vouchers with the three flags as the sheet says, and Tally's trial balance gives all nine day-end figures of the table, Cash and Petty Cash, day 5's credit included. Open question answered: at current date 12 April, R13 and R14 count post-dated voucher 10, so Cash closes 300.00 Dr at 25 and 30 April.**

Sheet `lab/1761-s1-cash-day-end-run-sheet` at bef604bb, 16 of 16 files equal. Nothing went through Bridge: masters and vouchers by hand on Tally's screen, reads by raw requests. BRIDGE CASHDAY LAB, new, books from 1 April 2025, no inventory, no GST, TallyPrime 7.2 Silver, loaded alone. R1 and R15 are the same bytes: V 11, M 214. Backups by hash in `kept-local.txt`.

## Additions, each agreed during the row
1. **R3b**, sent right after R3: master's plain window. R3 lacks `REFERENCEDATE` and carries the ten class COMPUTEs, so at master 9fcb102c it equals neither master's plain window nor its class read. R3b is R3 with the COMPUTEs removed and `REFERENCEDATE` added: 1,744 B, sha256 7f467dbdb19a5c7c4d53b235bd5b442477a3b10dca5279d1a93027c4ee31e25f, `requests/r03b_voucher_window_plain.request.utf16le.xml`. R3 stayed as locked. The other requests equal master's renders, rebuilt from its format strings, not captured sends: [partial].
2. **Size bound** 1,000,000 B for R3 and R3b, 200,000 for the rest. R3 answered 303,430 B over the first bound. It was held, then parsed from the saved file with no resend.
3. **Bank Allocations** on vouchers 3 and 5, a sub-screen the sheet does not name, accepted as offered with Ctrl+A, no field changed.
4. **Negative Cash warning** on voucher 4, "Negative Cash!", "₹ (-)300.00", "Press any key to continue". It went at my click on the title band, not a key; the voucher was then accepted unchanged. No later voucher raised it.

## R3: the vouchers
| # | date | type | no. | flags | entries |
|---|---|---|---|---|---|
| 1 | 3 Apr | Receipt | 1 | none | Customer A 5,000 Cr, Cash Dr |
| 2 | 4 Apr | Payment | 1 | none | Expenses 1,500 Dr, Cash Cr |
| 3 | 5 Apr | Contra | 1 | none | Cash 4,000 Cr, Bank A Dr |
| 4 | 5 Apr | Payment | 2 | none | Expenses 800 Dr, Cash Cr |
| 5 | 7 Apr | Contra | 2 | none | Bank A 2,000 Cr, Cash Dr |
| 6 | 8 Apr | Payment | 6 | ISOPTIONAL Yes | Expenses 600 Dr, Cash Cr |
| 7 | 9 Apr | Payment | 3 | ISCANCELLED Yes | one entry list, with no ledger and no amount |
| 8 | 10 Apr | Contra | 3 | none | Cash 500 Cr, Petty Cash Dr |
| 9 | 11 Apr | Payment | 4 | none | Expenses 200 Dr, Petty Cash Cr |
| 10 | 25 Apr | Payment | 5 | ISPOSTDATED Yes | Expenses 900 Dr, Cash Cr |

Every GUID begins with the company's, narrations are empty, and the three flags read No where the table says none. R3b holds the same ten with the same dates, numbers, ids and flags. Voucher 6 read No. 3 on the screen and its answer says 6. Voucher 7 took ALTERID 8 from its cancel, which is why V reads 11 for ten vouchers. The Gateway's date of last entry read 11-Apr-25, leaving out voucher 10. [verified]

Marking keys: optional Ctrl+L, post-dated Ctrl+T, each confirmed by the screen's Status line. Cancel: Alt+X on the reopened voucher, prompt "Cancel ?", answered Y.

## R4 to R14: Tally's closing
Tally signs a debit negative; `TBALCLOSING` is shown as Tally gave it.

| to | Cash | Petty Cash | table | |
|---|---|---|---|---|
| 2 Apr | -1000.00 | 0.00 | 1,000.00 Dr, 0 | equal |
| 3 Apr | -6000.00 | 0.00 | 6,000.00 Dr, 0 | equal |
| 4 Apr | -4500.00 | 0.00 | 4,500.00 Dr, 0 | equal |
| 5 Apr | 300.00 | 0.00 | 300.00 Cr, 0 | equal |
| 7 Apr | -1700.00 | 0.00 | 1,700.00 Dr, 0 | equal |
| 8 Apr | -1700.00 | 0.00 | 1,700.00 Dr, 0 | equal |
| 9 Apr | -1700.00 | 0.00 | 1,700.00 Dr, 0 | equal |
| 10 Apr | -1200.00 | -500.00 | 1,200.00 Dr, 500.00 Dr | equal |
| 11 Apr | -1200.00 | -300.00 | 1,200.00 Dr, 300.00 Dr | equal |
| 25 Apr | -300.00 | -300.00 | not set | voucher 10 counted |
| 30 Apr | -300.00 | -300.00 | not set | voucher 10 counted |

Days 8 and 9 show the optional and cancelled vouchers leaving the balance alone. To 25 April, Cash's credit total moves from 6,800 to 7,700, the 900 of voucher 10. [verified]

The window read exports vouchers 6, 7 and 10 alongside the rest. So a projection from R3 has to drop 6 and 7 by their flags to reach the table. For 10, Tally's own figure at a date after it counts it even when the current date is earlier.

## What the additions put in the answers
- **Bank Allocations:** a populated `BANKALLOCATIONS.LIST` on BRIDGE Bank A's entry in vouchers 3 and 5. Voucher 3's reads `TRANSACTIONTYPE` Cash, as the screen offered. Voucher 5's reads Cheque, as its screen offered, with `PAYMENTFAVOURING` the company's name and `CHEQUECROSSCOMMENT` A/c Payee. Why the two offers differ was not looked at; only the voucher's direction differs on the screen. It adds about 5.6 KB to each.
- **R3 against R3b:** the ten `BRIDGEVCH` tags in R3 only, about 1.3 KB a voucher; `REFERENCEDATE` in R3b only, empty on all ten.

## Why R3 passed its bound
The hypothesis holds, measured. Vouchers in R3's raw answer: 31,534 to 31,606 B ordinary, 37,214 to 37,240 B with a bank allocation, 4,594 B cancelled. In voucher 1, the two `ALLLEDGERENTRIES.LIST` take 26,716 of 30,772 B serialised, 87%: one list is 13,368 B with 190 children, 167 of them empty. `evidence/sizes.txt`.

## Departures from the sheet
- F2 inside a voucher moves Tally's current date with it, so the date was not held at 12 April during the keying: the Gateway read 25-Apr-2025 after voucher 10. It was set back to 12-Apr-2025 at the Gateway before BK2 and every read, so R13 and R14 ran at 12 April. Whether the date during keying affects voucher 10's flag was not tested. The date went back to 1-Apr-2025, the value the Gateway showed before voucher 1, at the close.
- BRIDGE Customer A has bill-by-bill No: Tally offered Yes, and the sheet names no bill-wise sub-screen.
- BRIDGE Bank A was first typed without its spaces and corrected on the same screen before accept; the accepted name is as the sheet says.
- Step 0's first tasklist failed on a shell's quoting; rerun from a command file, both kept.

Left behind: BRIDGE CASHDAY LAB stays, at BK2's state. No restore. Company shut, no company loaded, one tally.exe on 9001, `GET /status` 200.

**Confidence:** one synthetic book, one Tally, one sitting, ten vouchers: PARTIAL.
