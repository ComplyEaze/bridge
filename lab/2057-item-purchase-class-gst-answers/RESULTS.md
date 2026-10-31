# #2057 item purchase through a voucher class, GST: results (TallyPrime 7.2 Silver, BRIDGE CLASS PURCHASE LAB, the published mcp-v0.7.0 build)

Sheet `lab/2057-item-purchase-class-gst/RUN-SHEET.md` at a37544fc (SHA256.txt 8 of 8; `gen_requests.py --check` ok, 3 requests). LAB-COMMON v3. THE COMPANY: BRIDGE CLASS PURCHASE LAB, GUID 92a7ef97-dd28-4286-b02c-c059fcb8690e, verified_tuple. Build: `bridge_mcp.exe` sha256 48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d, equal at the start and at the end. One launch for the whole row. BK2 taken at step 2 and restored at step 17. Every master and voucher here was keyed on Tally's own screens; ComplyEaze Bridge only read. `notes.txt` has one line per step with UTC times and the marks before and after.

## Deviations, recorded

- **Step 7, a keying slip.** Up x5 from Stock Item landed on Currency instead of Ledger. The ledger name then typed as a filter raised Tally's "Spelling Error" box. No key was pressed and the row stopped. On the recorded word, one Escape closed both the box and the list, and the marks were unchanged (218/null). Lab Purchases was then reached by the Ledger row's visible label.
- **Step 16, a stray date box.** A keypress missed focus at the Gateway, and Alt+F2 opened "Change Current Date" with a stray `k` in it. Backspace then Escape closed it. Nothing was accepted and the Current Date stayed 12-Jun-2025.
- **I1 opened in Accounting Invoice mode** through the class. It was switched to Item Invoice (Ctrl+H) before any line was keyed. C1 and I2 opened in Item Invoice.
- **The item's GST rate.** Step 6 showed a single "GST Rate" field (18 %), with no CGST/SGST rows on the item screen.

## The GST setup, one change per save (marks)

| step | change | M after |
|---|---|---|
| 1 | as found | 213 (V null) |
| 3 | Company State Maharashtra | 215 (+2) |
| 4 | GST Registration (Alter Existing): Regular, GSTIN 27ZZZZZ0000Z1ZQ (synthetic) | 216 (+1) |
| 5 | Lab Supplier: India / Maharashtra / Regular, GSTIN 27ZZZZZ0000Z2ZP (synthetic) | 217 (+1) |
| 6 | Lab Class Item: HSN 340211 "Anionic", Taxable, GST Rate 18 % | 218 (+1) |
| 8 | Purchase voucher type: class `Lab Class` (Lab Purchases 100 %; Lab CGST and Lab SGST, Type of Calculation GST, Remove if Zero Yes) | 220 (+2) |

V stayed null until the first voucher.

## The invoices (EXPECT for each: Lab CGST 9 %, Lab SGST 9 %)

| voucher | type | lines on Tally's screen | total | V after |
|---|---|---|---|---|
| I1, Purchase 1, 10-Jun-25 | class Lab Class | Lab Class Item 5 Nos @ 100.00 = 500.00; Lab CGST and Lab SGST with **no amount** | **500.00** (EXPECT 590.00) | 1 |
| C1, Purchase 2, 11-Jun-25 | plain (class Not Applicable) | item 500.00; Lab CGST **no amount**; Lab SGST **45.00** | **545.00** (EXPECT 590.00) | 2 |
| I2, Purchase 3, 12-Jun-25 | class Lab Class | 7 Nos @ 13.57 = 94.99; Lab CGST **no amount**; Lab SGST **8.55**; no round-off line | **103.54** | 3 |

## The reads (steps 12 to 15)

Marks were 220/3 before the group, after it, and after the screens. Each raw a07 read answered 200 with no dialog: the capture after each equalled the quiet Gateway.

- **Raw a07 (by structure):** each voucher has one VOUCHER row, ISINVOICE Yes, PARTYGSTIN 27ZZZZZ0000Z2ZP, and no top-level `ALLINVENTORYENTRIES.LIST`.
  - `Lab Purchases` is an `ALLLEDGERENTRIES.LIST` entry of its own. The item sits nested in its `INVENTORYALLOCATIONS.LIST`, with no `ACCOUNTINGALLOCATIONS.LIST` inside.
  - That entry's `RATEDETAILS.LIST` reads CGST 9, SGST/UTGST 9, IGST 18 (Based on Value), Cess 0 (Not Applicable), State Cess 0.
  - GSTCLASS is "Not Applicable" and every GSTOVRDN* element is empty on every entry.
  - Entries: I1: Supplier 500.00, Purchases -500.00. C1: 545.00, -500.00, Lab SGST -45.00. I2: 103.54, -94.99, Lab SGST -8.55. No Lab CGST entry anywhere.
- **`vouchers` (goods_lines true):** the same entries and amounts. The goods line sits on the Lab Purchases entry: item, quantity, rate, amount, and batch Main Location / Primary Batch.
- **`purchase_register` June:**
  - items: C1 and I2, each status complete, tax_in_books Lab SGST (head state_tax, raw "State Tax") -45.00 / -8.55, taxable_entries Lab Purchases, has_taxable_entry true, party Lab Supplier, other_entries [];
  - I1 is in `purchase_vouchers_without_duties_taxes_entry` (total 1);
  - every other side list is 0.
- **`ledger_masters` compliance:** Lab CGST reads tax_type GST but **gst_duty_head "absent"**. Lab SGST reads gst_duty_head **State Tax**. This was already so at step 1, before any change.

## Step 16: Tally's screens against the reads

**EXPECT met:** every amount in the raw answer, in `vouchers` and in `purchase_register` equals Tally's screen for the same voucher.
- Day Book 10 to 12 Jun: 500.00, 545.00, 103.54.
- Each voucher shows the lines in the table above.
- Purchase Register for June: the same three, total 1,148.54.

## Answers

Each answer is a FINDING against EXPECT, on the recorded word.

- **(a)** The class did **not** compute tax on I1: no tax entry in any read or on the screen. **Cause unknown, unverified.** The candidate: Lab Purchases carries GST Rate Details "As per Company/Group" at 0 % (seen at step 7). Nothing was keyed to test it.
- **(b)** The plain type computed **the State head only** on C1: Lab SGST 45.00 and no Lab CGST entry. **Likely cause, partial confidence:** Lab CGST's gst_duty_head reads "absent" while Lab SGST's reads "State Tax". The lab book's CGST ledger looks incomplete.
- **(c)** `Lab Purchases` is a ledger entry of its own in all three vouchers, with the item nested in it. So `has_taxable_entry` is true wherever the register lists the voucher (C1, I2). I1 is not listed, because it has no Duties & Taxes entry.
- **(d)** I2: 7 x 13.57 = 94.99. Lab SGST stores **8.55** (9 % = 8.5491, to two places). There is no CGST and no round-off entry; the total is 103.54. The same likely cause as (b).

## Back to the start

- **Step 17**, on the explicit RESTORE: Company > Shut, then Restore of BK2 (only "Accept ?" Yes and "Replace" were answered).
  - THE COMPANY was reloaded alone. The Gateway read 1-Apr-2025, "No Vouchers Entered", and its capture is **byte-equal to step 0's**.
  - list_companies: the same GUID, verified_tuple.
  - Marks 213/null.
  - ledger_masters compliance and masters voucher_types equal step 1's.
- **Step 18:** the exe hash equals the start. The launch ended by inbox stop (exit 0). Company > Shut gave Select Company with none loaded. One tally.exe, 9001 LISTENING, `GET /status` 200.
