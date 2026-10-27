# Results: #2015 goods lines of a Credit Note, a Debit Note and a discounted Sales line keyed on screen, on BRIDGE SHAPE LAB, TallyPrime 7.2, 27 Oct 2026

The sheet is `lab/2015-goods-lines-notes/README.md` at 6e3744804d8bbff83003248dc27a79a2ce667948 (SHA256.txt 4 of 4 equal). The request used was `a07-vouchers-shape-20250424` (the product's `Bridge Agent Vouchers` request for 24 April 2025), sent four times, unchanged.

## Verdict
**PASS by the sheet.** r0, r1, r1b and r2 were received whole (HTTP 200, size equal to CONTENT-LENGTH) and counted by a parser. All three keyed vouchers have their goods recorded. No read moved a mark. The book went back to M0/V0, and r2 is byte-identical to r0. No setting was changed.

**For #2038:** all three keyed vouchers (an item-mode Credit Note, an item-mode Debit Note, and a Sales invoice with a discounted line) carry their goods where #2038 reads them: `ALLLEDGERENTRIES.LIST/INVENTORYALLOCATIONS.LIST`, each with one `BATCHALLOCATIONS.LIST`. No element named `*INVENTORY*` appears at any other depth.

**For #2015, a keyed input to note:** on this book Tally would not accept a goods line of `Hydrochloric Acid 33pc` (a batch item) until the line's **Expiry Date** was filled in. The cause is not verified. It happened on both the Credit Note and the Debit Note, so it is not only inward notes. The expiry `31-3-2027` was typed as a test input chosen for this row, not a measured value. After that, the batch list showed `H-2504-A` with that expiry, and K2 and K3 offered it pre-filled. The masters mark did not move. The restore removed it.

## Setting
- TallyPrime 7.2, Silver, licensed. One tally.exe, listening on port 9001, at step 0 and at the close; GET /status returned 200 both times.
- BRIDGE SHAPE LAB, company number 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, loaded alone.
- The company list and marks came from the built tool of master fd6f7c0d (exe sha256 63641231… before the first call and after the last), over stdio. The voucher reads were sent by curl.
- BK0: `TDBK1800_100021.001`, 102,253 bytes, sha256 609e6355eb94962f062d3215b19c46f527c99ba3a6202cd7fef9c450be2ca3bb. It stays on the lab PC.
- The company's current period is 1 April 2026 to 31 March 2027, so Tally refused the date 24 April 2025 ("Date cannot be below the current period"). For keying, the period was widened to start 1 April 2025 and the current date set to 24 April 2025. Both were set back before the restore, and both read as at step 0 after it. The marks were read after each change and did not move.

## Marks (V = ALTVCHID, M = ALTMSTID, by `ledger_masters`)
| point | M | V |
|---|---|---|
| step 0, r0, BK0, step 3, period and date changes, both discarded attempts | 292 | 112 |
| after K1 | 292 | 113 |
| after K2 | 292 | 114 |
| after K3 | 292 | 115 |
| after r1 and r1b, after the set-back | 292 | 115 |
| after the restore | 292 | 112 |

## r0 (before keying)
There are 2 vouchers on 24 April 2025: Payment 3 and Purchase 21. r0 is byte-identical to the committed 7.1 fixture (`lab/missing-reads-answers`, `answers/ans-a07.xml`, 110,156 bytes), so the Purchase's goods line differs in no field on 7.2.

## r1 (after keying): the goods lines, by parser
| voucher | ledger entry holding the goods | item | qty | rate | line AMOUNT | DISCOUNT | DISCOUNTAMOUNT | batch (godown, name) | batch AMOUNT | BATCHDISCOUNT | batches sum to line |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Credit Note 2 (K1) | Sales - Exempt, -30.00, deemed positive Yes | Hydrochloric Acid 33pc | 1.000 Kgs | 30.00/Kgs | -30.00 | 0 | empty | Main Location, H-2504-A | -30.00 | 0 | yes |
| Debit Note 2 (K2) | Purchase - Exempt, 30.00, deemed positive No | Hydrochloric Acid 33pc | 1.000 Kgs | 30.00/Kgs | 30.00 | 0 | empty | Main Location, H-2504-A | 30.00 | 0 | yes |
| Sales 4 (K3) | Sales - Exempt, 54.00, deemed positive No | Hydrochloric Acid 33pc | 2.000 Kgs | 30.00/Kgs | 54.00 | 10 | empty | Main Location, H-2504-A | 54.00 | 10 | yes |
| Purchase 21 (control) | Purchase - Local, -20000.00 | Hydrochloric Acid 33pc | 666.667 Kgs | 30.00/Kgs | -20000.00 | 0 | empty | Main Location H-2504-A; Factory Floor H-2504-B | -9000.00; -11000.00 | 0 | yes |

- Each keyed voucher has 2 `ALLLEDGERENTRIES.LIST`: the party, and the sales or purchase ledger holding the one `INVENTORYALLOCATIONS.LIST`.
- K3's discount: the line `AMOUNT` and the batch `AMOUNT` are both **net** of the discount (2 × 30.00 − 10 % = 54.00). `DISCOUNT` reads `10` (the percentage), and `DISCOUNTAMOUNT` is empty. The batch carries `BATCHDISCOUNT` 10.
- r1b is byte-identical to r1 (251,368 bytes).

## What the screens offered (recorded, not measured)
- **Change Mode.** Credit Note and Debit Note offer Item Invoice, Accounting Invoice, As Voucher, and Use for Interest Accounting. Sales offers Item Invoice, Accounting Invoice, and As Voucher.
- **Voucher-type settings.** Not recorded. Chart of Accounts > Voucher Types offered only "Enter: Alter", and the sheet says Display, never Alter.
- **Discount.** The Stock Item Allocations sub-screen has its own "Disc %" column. No F12 or F11 setting was needed.
- **Unplanned sub-screens, defaults kept.**
  - Party Details: Receipt or Dispatch Details, Original Invoice Details, and buyer fields.
  - Statutory Details on the Debit Note: Reason for Issuing Note "Not Applicable".
  - Bill-wise Details: K2 On Account; K3 New Ref "4".
  - For K1, Bill-wise opened with "Agst Ref" as the default. The sheet does not measure a note against a specific invoice, so New Ref "2" was chosen instead.
- **First ledger offered.** K1 and K3 took Sales - Exempt, the first one offered. K2's list began with "ITC Test Ledger", so the first purchase ledger, Purchase - Exempt, was taken instead.

## Not measured
- The items the sheet lists: additional cost, bill of materials, foreign currency, the Stock Journal family, a note against a specific invoice reference, goods kept only under `ALLINVENTORYENTRIES`, and Gold.
- Also not measured:
  - Why the Expiry Date field blocks a keyed goods line on this item.
  - A goods line with no expiry.
  - The voucher types' numbering settings.

## Files
- `answers/`: r0, r1, r1b and r2 as received, each with its response headers.
- `send-log.txt`: each curl send, with the request's sha256 and the answer's file, bytes and sha256.
- `tool-answers/`: every tool answer of the stdio session (the company list and the marks). `client-log.txt` is that session's log.
- `tables-r1.txt`: the parser's tables for r1.
- `notes.txt`: one line per step, in UTC.
- `restore-plan.txt`.
- `SCREENSHOTS.txt`: the screen captures by name, bytes and sha256. The captures, BK0 and the data-folder listings stay on the lab PC.
