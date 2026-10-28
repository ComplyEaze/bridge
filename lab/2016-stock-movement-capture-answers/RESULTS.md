# Results: #2016 stock movement capture, 28 Oct 2026. On TallyPrime 7.2, a Stock Journal between godowns reads with an empty ledger entry list through the shipped vouchers request, and with its stock lines, direction, godown and batch through the inventory fetch. A Credit Note's item comes through both reads. Delivery Note and Receipt Note are inactive voucher types on this book, so those two kinds were not keyed. Both reads match Tally's screens. The book was restored from BK1 and reads as before

The sheet is `lab/2016-stock-movement-capture` at f85ed60ff87f3bc891e237fdff2ab85d3d759042.
- **Sheet checks:** `SHA256.txt` 17 of 17; `gen_requests.py --check` ok; census ok 8, skipped 1.
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346. Its marks were 292 / 112 at step 1 and again after the restore.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after).
- **Tally:** TallyPrime 7.2, Silver, Licensed.
- **BK1:** `TDBK1800_100021.001`, 102,253 B, sha256 dc41802523f0aceec7a2a4d50e3e6ae953cdd575489247e5ed6bd19a271a309f.
- **Marks read:** `ledger_masters` basic refused on this book (`company_several_currency_masters`). Every marks read here is `ledger_masters` with `fields=compliance`, the read the refusal names; the 2015 row read this book the same way.

## What was keyed (step 4)
| kind | offered | keyed | M / V after |
|---|---|---|---|
| Stock Journal | yes | **SJ1**, 11-May-26: 10 Kgs out of Main Location / H-2504-A at 30.00 (300.00), and 10 Kgs into Factory Floor / H-2504-A with no rate and no value offered | 292 / 113 |
| Delivery Note | **no**: an inactive voucher type (Show Inactive lists it greyed, Alt+F8) | skipped | |
| Receipt Note | **no**: inactive (Alt+F9) | skipped | |
| Credit Note | yes | **CN1**, 14-May-26, Item Invoice: Outstation Buyer 1 (the first Sundry Debtors ledger the list offers); Sales - Exempt (offered first); 1 Kgs into Main Location / H-2504-A at 30.00; Bill-wise New Ref (the default here was On Account) | 292 / 114 |

Notes on the keying:
- **SJ1's batch.** On the destination side Tally listed H-2504-A as a "New Number" for Factory Floor, and its "New Number" sub-screen held the name; it was passed on its default.
- **Expiry.** An empty Expiry Date did not pass, so 31-3-2027 was typed (the sheet's test input). CN1 then found it pre-filled from the batch.
- **Tally's current date.** Setting a voucher's date with F2 moved the current date to 14-May-2026. The restore brought it back to 20-Apr-2026.

## The reads (step 6, 11 to 14 May, one day each)
- **SJ1, through the shipped request (a07):**
  - One VOUCHER whose only list is an empty `ALLLEDGERENTRIES.LIST`. It carries no item, quantity, godown or batch: #388 seen on 7.2.
- **SJ1, through the inventory fetch (a08):**
  - `ALLINVENTORYENTRIES.LIST` x2: ISDEEMEDPOSITIVE **Yes** = into stock (10 Kgs, RATE and AMOUNT empty); **No** = out of stock (10 Kgs, 30.00/Kgs, 300.00).
  - Each has one `BATCHALLOCATIONS.LIST` with GODOWNNAME (Factory Floor / Main Location), BATCHNAME H-2504-A, EXPIRYPERIOD and DESTINATIONGODOWNNAME Factory Floor.
  - `INVENTORYENTRIESIN.LIST` and `INVENTORYENTRIESOUT.LIST` are also present, although the fetch names only `ALLINVENTORYENTRIES.*`. They hold the same item, quantities and values; their batch allocations carry BATCHNAME but no GODOWNNAME.
- **CN1, through a07:**
  - The item comes inside `ALLLEDGERENTRIES.LIST` (Sales - Exempt, -30.00) / `INVENTORYALLOCATIONS.LIST`: ISDEEMEDPOSITIVE Yes, 1 Kgs, 30.00/Kgs, -30.00.
  - `BATCHALLOCATIONS`: Main Location, H-2504-A.
  - The party entry is 30.00, ISDEEMEDPOSITIVE No.
- **CN1, through a08:**
  - The same item as an `ALLINVENTORYENTRIES.LIST` with `ACCOUNTINGALLOCATIONS.LIST` (Sales - Exempt, -30.00), plus a `LEDGERENTRIES.LIST` party line.
- **12 and 13 May:** no voucher in either read.
- **Marks:** 292 / 114 before and after the reads.

## Against Tally (steps 2, 5 and 7)
- **Stock Item Vouchers** for Hydrochloric Acid 33pc in May:
  - SJ1: Inwards 10.000 Kgs (no value), Outwards 10.000 Kgs 300.00.
  - CN1: Outwards **(-)1.000 Kgs (-)30.00**. Tally shows the return as a negative outward; the reads mark it into stock with ISDEEMEDPOSITIVE Yes.
  - Monthly: Inwards 10 Kgs, Outwards 9 Kgs 270.00, closing 817.667 Kgs 15,230.00.
- **By godown and batch** (Stock Query):

| godown / batch | before | after |
|---|---|---|
| Main Location / Primary Batch | 150 | 150 |
| Main Location / H-2504-A | 300 | 291 |
| Factory Floor / H-2504-A | — | 10 |
| Factory Floor / H-2504-B | 366.667 | 366.667 |

- **Both reads give** the same quantities, rates, values, godowns and batches as the keying and these screens.
- **Stock Summary closing:**
  - Raw Chemicals: s0 -30249.99, s1 -29979.99. Tally: 30,249.99 to 29,979.99 (Hydrochloric Acid 15,500.00 to 15,230.00).
  - The change is **-270.00 in value, equal** in both.
  - The raw report carries group values only (quantity and rate empty), so quantities were compared with the screens.

## Restore (step 8, on the word RESTORE)
- BK1 was restored by C6 with the two expected prompts (Accept ? Yes; folder 100021 exists: Replace).
- After it: list_companies is exactly the company, the marks are 292 / 112 (equal to step 1), and `r-stock-summary` is byte-equal to s0.

**Confidence:** one observation per kind, PARTIAL until repeated. Delivery Note and Receipt Note are not measured: they are inactive on this book, and no feature or voucher type was switched.

## Files
- `answers/` (tool, 000 to 010).
- `raw/`:
  - every raw answer with its headers;
  - `<answer>.parsed.txt`, the structure as parsed;
  - s0, s1 and s2-after-restore.
- `parse.py`.
- `send-log.txt`.
- `client-log.txt`.
- `restore-plan.txt`.
- The listings of the company's data folder before and after the restore stay on the lab PC.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures and BK1 by sha256; they stay on the lab PC.
