# #2057-B item purchase through a voucher class, GST, with both tax ledgers whole: results (TallyPrime 7.2 Silver, BRIDGE CLASS PURCHASE LAB, mcp 0.7.0 CI build at ae9a1bb5)

Sheet `lab/2057-b/RUN-SHEET.md` at 656633d4 (SHA256.txt 10 of 10; `gen_requests.py --check` ok, 5 requests). LAB-COMMON v3. THE COMPANY: BRIDGE CLASS PURCHASE LAB, GUID 92a7ef97-dd28-4286-b02c-c059fcb8690e, verified_tuple. Build: CI run <CI run id> (head ae9a1bb5), `bridge_mcp.exe` sha256 c5b75d6480202ef70a5216f74d369ea0963b4a2eedbb3d4912e4f25b385e943d, equal at the start and at the end. BK3 taken at step 2 and restored at step 23. Every master and voucher here was keyed on Tally's own screens; ComplyEaze Bridge only read. `notes.txt` has one line per step with UTC times; `evidence/marks.txt` has every mark.

Each line below is marked **verified** (seen on Tally's screen and in the captured answers), **partial** or **hypothesis**.

## Deviations, recorded

- **Step 1, first launch.** The run client stopped with FileNotFoundError writing `srv/step0/initialize.json` before any tool call; its server exited with it. That launch wrote `data/terms-acceptance.jsonl`. The client was relaunched with a new empty data folder (`data2/`); every answer here is from that one launch. (verified)
- **Step 7, the look stopped early.** Down x2 put the cursor on Under with the List of Groups open; a further Down would have changed the group, so the look stopped there and the "1 v" line was not read. One Ctrl+Q, no prompt, nothing saved; marks unchanged. (verified)
- **Step 23, C6 listing.** The listing of THE COMPANY's data folder before the restore was not taken. The listing after it was taken after step 24's shut and is kept on the lab PC. Every other C6 identity check was made, and step 23's reads check the result. (verified)
- **The launch ended** by stopping the run client after step 24's shut, not by an inbox stop; this row's server was not running afterwards. (verified)
- The four desktop-client connector servers (started 30 Oct and 2 Nov) stayed running: no call was made to them from this sitting, and their calls were not measured. (verified)

## The setup, one change per save (marks)

| step | change | M after |
|---|---|---|
| 1 | as found | 213 (V null) |
| 3 | Company State Maharashtra | 215 (+2) |
| 4 | GST Registration (Alter Existing): Regular, GSTIN 27ZZZZZ0000Z1ZQ (synthetic) | 216 (+1) |
| 5 | Lab Supplier: India / Maharashtra / Regular, GSTIN 27ZZZZZ0000Z2ZP (synthetic) | 217 (+1) |
| 6 | Lab Class Item: HSN 340211 "Anionic", Taxable, GST Rate 18 % | 218 (+1) |
| 7 | Lab Purchases looked at only | 218 (0) |
| 8 | Purchase voucher type: class `Lab Class` (Lab Purchases 100 %; Lab CGST and Lab SGST, GST, Remove if Zero Yes) | 220 (+2) |
| 9 | Lab CGST: Tax type CGST (B1) | 221 (+1) |

V stayed null until the first voucher. (verified)

**Step 9 (B1).** Lab SGST's field holding "State Tax" is "Tax type". On Lab CGST that field was empty; its list "Tax Types" held IGST, CGST, SGST/UTGST, Cess, with one central entry, "CGST", which was chosen. `ledger_masters` then read Lab CGST `gst_duty_head` recognized, head `cgst`, raw `CGST` (EXPECT: **PASS**); Lab SGST unchanged, `state_tax`. (verified)

## The invoices (EXPECT for each: Lab CGST 9 %, Lab SGST 9 %)

| voucher | type | mode it opened in | lines on Tally's screen | total | V after | EXPECT |
|---|---|---|---|---|---|---|
| I1-B, Purchase 1, 10-Jun-25 | class Lab Class | Accounting Invoice (Ctrl+H to Item Invoice) | Lab Class Item 5 Nos @ 100.00 = 500.00; Lab CGST 45.00; Lab SGST 45.00 | 590.00 | 1 | PASS |
| C1-B, Purchase 2, 11-Jun-25 | plain (Not Applicable) | Item Invoice | item 500.00; Lab CGST 45.00; Lab SGST 45.00 | 590.00 | 2 | PASS |
| I2-B, Purchase 3, 12-Jun-25 | class Lab Class | Item Invoice | 7 Nos @ 13.57 = 94.99; Lab CGST 8.55; Lab SGST 8.55; no round-off line | 112.09 | 3 | PASS (recorded) |
| I3, Purchase 4, 13-Jun-25 | class Lab Class | Item Invoice | item 500.00; Lab CGST 45.00; Lab SGST 45.00 | 590.00 | 4 | PASS |

Every tax amount was filled by Tally; none was typed. No save raised a prompt. M stayed 221. (verified)

## The reads (steps 14 to 17)

Marks were 221/4 before the group, after it, and after the screens. Each raw a07 read answered 200 with no dialog.

- **Raw a07 (by structure):** one VOUCHER row each, ISINVOICE Yes, PARTYGSTIN 27ZZZZZ0000Z2ZP, no top-level `ALLINVENTORYENTRIES.LIST`. Entries: Lab Supplier (party), `Lab Purchases` (an `ALLLEDGERENTRIES.LIST` of its own, the item in its `INVENTORYALLOCATIONS.LIST`), Lab CGST and Lab SGST. The `Lab Purchases` entry's `RATEDETAILS.LIST` reads CGST 9, SGST/UTGST 9, IGST 18 (Based on Value), Cess 0 (Not Applicable), State Cess 0. GSTCLASS "Not Applicable", every GSTOVRDN* element empty, GSTTAXRATE 0 on every entry. (verified)
- **`vouchers` (goods_lines true):** the same entries and amounts; the goods line on the Lab Purchases entry (item, quantity, rate, amount, Main Location / Primary Batch). (verified)
- **`purchase_register` June:** 4 items, each status complete, `tax_in_books` Lab CGST (head `cgst`, raw `CGST`) and Lab SGST (head `state_tax`, raw `State Tax`), `taxable_entries` Lab Purchases, `has_taxable_entry` true, party Lab Supplier, `other_entries` []. Every side list is 0. (verified)
- **`ledger_masters`** equal to step 9e's. **`masters` voucher_types:** the Purchase row differs from step 1 only in `alter_id` (48 -> 220) and `numbering_method` ("default" -> {unrecognised None}), the values after step 8, as read on 31 Oct. (verified)

## Step 18: Tally's screens against the reads

**EXPECT: PASS.** Every amount in the raw answer, in `vouchers` and in `purchase_register` equals Tally's screen for the same voucher: the Day Book 10 to 13 Jun (590.00, 590.00, 112.09, 590.00), each voucher displayed with the lines above, and the Purchase Register for June (total 1,882.09). (verified)

## Answers

- **(a)** C1-B carried both heads: Lab CGST 45.00 and Lab SGST 45.00, total 590.00. (verified)
- **(b)** The class computed tax on I1-B, I2-B and I3, on both heads: 45.00 + 45.00, 8.55 + 8.55, and 45.00 + 45.00. (verified) With 31 Oct's run (no CGST anywhere; no tax at all through the class), the one change between the two runs is Lab CGST's Tax type set to CGST at step 9; that this is the cause is a **hypothesis** at the confidence one run gives.
- **(c)** I1-B and I3 did not differ in lines or amounts. Modes: I1-B opened in Accounting Invoice and was switched to Item Invoice; I3 opened in Item Invoice. (verified)
- **(d)** I2-B stores 8.55 on each head (9 % of 94.99 = 8.5491, to two places); no round-off entry; total 112.09. (verified)
- **(e)** `Lab Purchases` is a ledger entry of its own in all four vouchers, with the item nested in it, so `has_taxable_entry` is true on all four; all four are `purchase_register` items. (verified)
- **(f)** B2 not run: I1-B and I3 both carry tax, so steps 19 to 22 were skipped, as the sheet says. (verified)

## Back to the start

- **Step 23**, on the explicit RESTORE (the backup step 23 names, BK3): Company > Shut, then Restore of BK3 (only "Accept ?" y and "Replace" were answered). THE COMPANY was reloaded alone. The Gateway read 1-Apr-2025, "No Vouchers Entered"; its capture is **byte-equal to step 0's**. list_companies: the same GUID, verified_tuple. Marks 213/null. ledger_masters compliance and masters voucher_types equal step 1's. **PASS.** (verified)
- **Step 24:** the exe hash equals the start. Company > Shut gave Select Company with none loaded. One tally.exe, 9001 LISTENING, `GET /status` 200. (verified)

## Fixture

`answers/fixture-i1b-r-a07-20250610.xml` is the I1-B raw a07 answer, byte for byte (= `raw/r14-r-a07-20250610.xml`). PARTYGSTIN holds the lab's invented all-Z GSTIN; VOUCHERKEY 15-digit runs; kept as captured.
