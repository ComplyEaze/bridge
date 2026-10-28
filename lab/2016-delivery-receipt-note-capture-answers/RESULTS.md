# Results: #2016 Delivery Note and Receipt Note capture, 28 Oct 2026. On TallyPrime 7.2 both voucher types were made active through "Activate this Voucher Type" (V unchanged, M +2 each). One Delivery Note and one Receipt Note were keyed. Both reads return each note with its item, quantity, rate, value, godown, batch, expiry and tracking number, and the direction in the item entry's `ISDEEMEDPOSITIVE`. Tally's screens agree. The book was restored from BK1: marks back to 292 / 112, the Stock Summary read byte-equal, and both types inactive again

The sheet is `lab/2016-delivery-receipt-note-capture` at fc993a31021595e14760ffa386033f13b1d4966f.
- **Sheet checks:** `SHA256.txt` 13 of 13; `gen_requests.py --check` ok; census ok 4, skipped 1 (the Stock Summary request, read by eye).
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after).
- **Tally:** TallyPrime 7.2, Silver Rental, Licensed.
- **BK1:** `TDBK1800_100021.001`, 102,253 B, sha256 aed8d7742357b5a7af927c7c0ffcebeb54effc29a7b4d2059491e3cb1dcde90c.
- **Marks:** every marks read is `ledger_masters` with `fields=compliance`, as in the first row on this book.

## Marks through the row
| point | M | V |
|---|---|---|
| step 1 and after BK1 | 292 | 112 |
| Delivery Note made active | 294 | 112 |
| Receipt Note made active | 296 | 112 |
| after DN1 | 296 | 113 |
| after RN1, and before and after the raw reads | 296 | 114 |
| after the restore | 292 | 112 |

## The setting change (step 3)
- **Where:** Gateway > Alter > Voucher Type. Both types were in the List of Voucher Types (Show Inactive was not needed there).
- **The field:** on 7.2 it is labelled "Activate this Voucher Type". It showed No on both types; only that field was set to Yes.
  - "Make this voucher type as 'Optional' by default" stayed No and was not touched.
  - Every field and value is recorded in `notes.txt`.
- **Accept:** Ctrl+A on each type returned to the list with no message.
- **Effect:** both types then showed under Inventory Vouchers on the Vouchers screen's F10 list, Delivery Note as Alt+F8 and Receipt Note as Alt+F9.

## What was keyed (step 4), each saved with no message
| voucher | keyed | read as |
|---|---|---|
| DN1 Delivery Note No. 1, 12-May-26 | Outstation Buyer 1; Sales ledger Sales - Exempt (first offered; the sheet names none); Hydrochloric Acid 33pc 1 Kgs, Main Location / H-2504-A, no rate offered so 30.00 keyed, amount 30.00; Tracking No. 1 (default); Expiry Date 31-3-2027 (asked) | MASTERID 86, ALTERID 113 |
| RN1 Receipt Note No. 1, 13-May-26 | Chem Supplier 1 (first offered ledger reaching Sundry Creditors); Purchase ledger ITC Test Ledger (first offered, under Purchase Accounts); 1 Kgs into Factory Floor / H-2504-B, rate 30.00 offered and kept; Tracking No. 1; Expiry Date 31-3-2027 | MASTERID 87, ALTERID 114 |

Sub-screens opened on each voucher and were passed on their defaults (recorded in `notes.txt`):
- Order Details with Dispatch Details (DN1) or Receipt Details (RN1), all fields blank;
- Party Details: State Not Applicable, Country India, registration Unknown, Place of Supply Not Applicable;
- the Tracking No. list inside Item Allocations.

Tally's current date followed RN1 to 13-May-2026. It was set back to 20-Apr-2026 at the Gateway before the shut.

## How the reads carry the notes (steps 6 and 7)
Each answer holds exactly one VOUCHER row, the note of its day. Every answer is under its bound, and the marks were the same before and after the reads.

| element | shipped a07 | inventory fetch a08 |
|---|---|---|
| header | DATE, VOUCHERTYPENAME Delivery Note / Receipt Note, VOUCHERNUMBER 1, PARTYLEDGERNAME, ISINVOICE No, ISOPTIONAL No, PERSISTEDVIEW Invoice Voucher View | the same |
| item line | `ALLLEDGERENTRIES.LIST/INVENTORYALLOCATIONS.LIST` under the sales or purchase ledger entry | `ALLINVENTORYENTRIES.LIST` |
| item, quantity, rate, value | STOCKITEMNAME, ACTUALQTY / BILLEDQTY 1.000 Kgs, RATE 30.00/Kgs, AMOUNT | the same |
| godown, batch | `BATCHALLOCATIONS.LIST`: GODOWNNAME, DESTINATIONGODOWNNAME (equal to the godown), BATCHNAME, EXPIRYPERIOD 31-Mar-27, TRACKINGNUMBER 1, ORDERNO Not Applicable | the same |
| direction | item ISDEEMEDPOSITIVE No on the Delivery Note (AMOUNT 30.00), Yes on the Receipt Note (AMOUNT -30.00) | the same |
| party | `ALLLEDGERENTRIES.LIST`: -30.00 (DN) / 30.00 (RN), one bill allocation On Account, name empty | `LEDGERENTRIES.LIST`, the same |
| sales / purchase ledger | the second `ALLLEDGERENTRIES.LIST` (30.00 / -30.00) | `ACCOUNTINGALLOCATIONS.LIST` under the item |

Neither request returns `INVENTORYENTRIESIN.LIST` or `INVENTORYENTRIESOUT.LIST` for these notes. Quantities are unsigned.

## Against Tally's screens (step 5)
- **Hydrochloric Acid 33pc, Stock Item Monthly Summary for May:** Inwards 1.000 Kgs 30.00, Outwards 1.000 Kgs 30.00; closing 816.667 Kgs 15,500.00, as before.
- **Stock Query:** Main Location / H-2504-A went 300 to 299 and Factory Floor / H-2504-B went 366.667 to 367.667; total 816.667 Kgs. Cost price went 18.98 to 18.99/Kgs. Standard selling price went from blank to 30.00/Kgs, with M unchanged.
- **Stock Item Vouchers:** both notes appear as "(Purchase Bills Pending)" and "(Sale Bills Pending)" lines dated 31-May-26, with quantity only. Each line names the note's own date, party and quantity.
- **Stock Summary:** Raw Chemicals 30,249.99 and the item's line were unchanged on screen. The raw `s0` and `s1` are byte-equal. Equal on both sides.

These figures agree with the keying and with both reads.

## Restore (step 8, on the recorded RESTORE word)
- **Restore:** BK1 was restored with the two expected prompts (Accept ? Yes; folder 100021 exists: Replace).
- **Marks and company:** marks 292 / 112, and the compliance answer is byte-equal to the read after BK1. `list_companies` shows exactly the company.
- **Stock Summary:** `s2` is byte-equal to `s0`.
- **Voucher types:** Delivery Note and Receipt Note are absent from the Vouchers F10 list, and greyed under Show Inactive.
- **Date:** Tally's current date is 20-Apr-2026, as at step 0.

**Confidence:** one voucher of each kind, PARTIAL until repeated. The sales and purchase ledgers were Tally's first offers, as the sheet names none.

## Files
- `answers/`: the tool answers, 000 to 011. The client names each file `NNN-NNN-<label>.json`.
- `raw/`:
  - every raw answer with its headers;
  - `<answer>.parsed.txt` for the four voucher reads;
  - `s0`, `s1` and `s2` (the last taken after the restore).
- `parse.py`.
- `send-log.txt`.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures and BK1 by sha256; they stay on the lab PC.
