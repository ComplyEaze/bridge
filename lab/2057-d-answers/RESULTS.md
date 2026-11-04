# Row 2057-D: an item purchase keyed with the voucher's common ledger turned off: results

**First line: the unmeasured shape exists on 7.2.** With "Select common Ledger Account for Item Allocation" set to No, Tally asked for the purchase ledger on the item line only. In the stock-lines (sl) answer, the item sits at voucher level, and `Lab Purchases` appears only in the item's own accounting allocation. The vouchers (a07) answer still shows `Lab Purchases` as an `ALLLEDGERENTRIES.LIST` of its own, carrying the item, and its entries sum to 0.00. `vouchers` and `purchase_register` read C2-C, and every amount equals Tally's screen.

Every EXPECT was met, with one recorded keying finding at step 7 (one `N` showed "n" until Enter; continued on the recorded word (a)). No other stop and no prompt beyond those the steps name. The restore of BK5 brought the book back to step 1, checked at step 14. (verified)

Sheet `lab/2057-d-sheet` at 69b148e1 (folder `lab/2057-d`; SHA256.txt 8 of 8 equal, by sha256 and bytes). LAB-COMMON v3. TallyPrime 7.2 Silver. Every step and stamp is in `notes.txt` (UTC). Each amount and each element named below is in `raw/*.parsed.txt` and `evidence/s9-10-tool-records.txt`.

## The five answers

- **(a)** Step 7 set the option from **Yes** to **No**: `N`, then one Enter (see step 7 below), then Ctrl+A.
  - The marks after it read M 219, V null (012), as before it (010): **M did not move**, so this option is not counted in the company's master AlterID.
  - The voucher screen's head fields changed. Before: Supplier Invoice No., Date, Party A/c name, **Purchase ledger**. After: Supplier Invoice No., Date, Party A/c name, Current balance. **The "Purchase ledger" field no longer shows.**
  - Step 8 read **No** again on F12 before anything was keyed. Step 13 set it back to Yes the same way: the head field showed again, and the marks read M 219, V 1 (021). (verified)
- **(b)** On C2-C, Tally asked for the purchase ledger **on the item line only**.
  - Nothing at the head: no purchase-ledger field showed.
  - After the item's amount, a screen "Accounting Details for : Lab Class Item, Up to 500.00 Dr" opened, with Particulars empty. `Lab Purchases` was chosen there (the only ledger offered), and Tally filled 500.00.
  - Tally did not add the tax lines itself: `Lab CGST` and `Lab SGST` were selected below the item, and Tally filled 45.00 each. (verified)
- **(c)** In the a07 answer, `Lab Purchases` **is an `ALLLEDGERENTRIES.LIST` of its own** (-500.00, ISDEEMEDPOSITIVE Yes).
  - The item sits inside it, as `INVENTORYALLOCATIONS.LIST` (Lab Class Item, 5 Nos, 100.00/Nos, -500.00), next to a `RATEDETAILS.LIST`.
  - There is no top-level `ALLINVENTORYENTRIES.LIST`.
  - Lab Supplier 590.00 (No), Lab CGST -45.00 (Yes), Lab SGST -45.00 (Yes). ISINVOICE Yes; PARTYGSTIN 27ZZZZZ0000Z2ZP.
  - The `ALLLEDGERENTRIES.LIST` amounts sum to **0.00**. (verified)
- **(d)** In the sl answer, the item is a **top-level `ALLINVENTORYENTRIES.LIST`** (Lab Class Item, ISDEEMEDPOSITIVE Yes, 5 Nos, 100.00/Nos, -500.00).
  - Its `ACCOUNTINGALLOCATIONS.LIST` holds `Lab Purchases` -500.00 (Yes).
  - The three `LEDGERENTRIES.LIST` are Lab Supplier 590.00, Lab CGST -45.00 and Lab SGST -45.00. **`Lab Purchases` appears only inside the item's accounting allocation.** (verified)
- **(e)** `vouchers` (20250615, `goods_lines` true) returned C2-C, state complete, with:
  - Lab Supplier 590.00 (bill New Ref LAB-2057-6);
  - Lab Purchases -500.00 with `goods_lines` (5 Nos, -500.00, Primary Batch);
  - Lab CGST -45.00 and Lab SGST -45.00.

  `purchase_register` (20250601-20250630) holds C2-C in **`items`**, with:
  - status complete;
  - `has_taxable_entry` true;
  - `taxable_entries`: Lab Purchases -500.00;
  - `tax_in_books`: Lab CGST `cgst` (raw CGST) -45.00 and Lab SGST `state_tax` (raw State Tax) -45.00;
  - `party_entries`: Lab Supplier 590.00 (New Ref LAB-2057-6);
  - `other_entries` empty.

  **Every amount in the raw answers and the tool answers equals Tally's screen** (500.00, 45.00, 45.00, 590.00), in the Day Book display of C2-C and the Purchase Register for June 2025. (verified)

By the sheet's "What the row decides", first row: C2-C holds the item at voucher level with `Lab Purchases` only in its accounting allocation, **in the sl answer**. The a07 answer of the same voucher holds `Lab Purchases` as an entry of its own with the item inside it, as on 2057-B. Both are recorded as found. What that means for the description's "not measured" line is for the run lead. (verified)

## Every EXPECT line

| step | EXPECT | result |
|---|---|---|
| 0 | Current Date 1-Apr-2025, "No Vouchers Entered" | PASS: Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26. The capture is byte-equal to 2057-C's step 0 (a1ccfeac…). tally.exe 1200 owns 9001; GET /status 200. (verified) |
| 1 | `list_companies` exactly THE COMPANY, `verified_tuple`, the GUID | PASS: BRIDGE CLASS PURCHASE LAB, 92a7ef97-dd28-4286-b02c-c059fcb8690e, verified_tuple. 32 tools. (verified) |
| 1 | marks M0/V0 = 213 / null | PASS: reused false. (verified) |
| 1 | `ledger_masters` compliance equal to 2057-B's 003, apart from the snapshot | PASS: 6 rows, each equal (`evidence/s1-cmp.txt`). (verified) |
| 1 | `masters` voucher_types: the Purchase row equal to 2057-B's | PASS. (verified) |
| 2 | BK5, then marks M0/V0 | PASS: BK5, 24,100 B, sha256 86e9a9af45776ee53a847d77473a546db81e09de4e7fb3285f71f4adf4b9045a, one file; marks 213 / null. (verified) |
| 3 | M 215 | PASS: 215 (+2), V null. F11: GST Yes; "Set/Alter Company GST Rate and Other Details" No, left. (verified) |
| 4 | M 216 | PASS: 216 (+1), V null. GSTIN 27ZZZZZ0000Z1ZQ accepted, no message. Answer 007's label says "after-f11" by a lab slip; it was read after step 4's save (recorded in `notes.txt`). (verified) |
| 5 | M 217 | PASS: 217 (+1), V null. GSTIN 27ZZZZZ0000Z2ZP accepted, no message. (verified) |
| 6 | M 218 | PASS: 218 (+1), V null. One "GST Rate" field, 18 %, as on 4 Nov. (verified) |
| 6b | the Tax Types list as on 4 Nov | PASS: IGST, CGST, SGST/UTGST, Cess. (verified) |
| 6b | M 219 | PASS: 219 (+1), V null. (verified) |
| 6b | Lab CGST `recognized` `cgst` raw `CGST`; Lab SGST `state_tax` | PASS (`evidence/s6b-duty-heads.txt`). (verified) |
| 7 | "Voucher Configuration"; the label in its exact words, reading Yes | PASS: every option and value equal to 2057-C2's transcription. (verified) |
| 7 | one `N` makes it read No | **FINDING, continued on the recorded word (a):** one `N` showed "n" with the text cursor after it, an edit in progress. Stopped with no further key; reported. On the word: ONE Enter made it read No, and nothing else changed. The field takes typed text and resolves it on Enter, so `N` then Enter is the keying; step 13 keyed `Y` then Enter the same way. (verified) |
| 7 | V unchanged | PASS: M 219, V null (012). (verified) |
| 8 | the option reads No before keying | PASS. (verified) |
| 8 | V +1 | PASS: M 219, V 1 (013). (verified) |
| 9-11 | marks after the group equal to M1/V1 (219 / 1) | PASS (019). Both raw answers 200, parsed as XML, bytes = CONTENT-LENGTH: a07 77,918 B (68343f42…), sl 40,164 B (2f791622…). (verified) |
| 12 | marks after the screens equal to M1/V1 | PASS: 219 / 1 (020). (verified) |
| 13 | the option reads No, then Yes | PASS: No before; `Y`, Enter, Ctrl+A; read again by the same route: Yes. (verified) |
| 13 | V unchanged | PASS: 219 / 1 (021). (verified) |
| 14 | the restore: no prompt beyond "Accept ?" and Replace | PASS: the Restore list showed THE COMPANY, 4-Nov-26 (6:08 pm); Accept y; Replace r; no other prompt. (verified) |
| 14 | Gateway as at step 0 | PASS: the capture is **byte-equal** to step 0's (69,119 B, a1ccfeac…). (verified) |
| 14 | `list_companies` as at step 1 | PASS: equal to 001 (`evidence/s14-cmp.txt`). (verified) |
| 14 | marks equal to M0/V0 | PASS: 213 / null (023; again 026 after the option read). (verified) |
| 14 | `ledger_masters` and `masters` as at step 1 | PASS: 024 equal to 003 and 025 equal to 004, apart from read times, snapshot and evidence. Lab CGST's duty head reads `absent` again. (verified) |
| 14 | the option reads Yes | PASS: "Select common Ledger Account for Item Allocation : Yes"; no key on it; the "Purchase ledger" head field shows. (verified) |
| 15 | exe hashed; Company > Shut; none loaded; the date as step 0; one tally.exe; 9001 LISTENING; /status 200 | PASS: c5b75d6480202ef70a5216f74d369ea0963b4a2eedbb3d4912e4f25b385e943d at the start and the end. Client stopped, server exit 0. Tuesday, 1-Apr-2025. Shut: none loaded. tally.exe 1200 owns 9001; GET /status 200. (verified) |

The data folder's listing before and after the restore is recorded, not compared: 19 entries before, 15 after, the restore writing the folder anew (`evidence/s14-listing-*.txt`). (verified)

## Kept on the lab PC

No captured byte is altered. A captured file is pushed whole unless it holds the company number or a run of 10 or more digits. Two places are exempt: inside a whole 64-hex or 40-hex hash value, and the whole value of a VOUCHERKEY element. Each kept file is listed in `kept-local.txt` by size and sha256, with its reason. BK5 and the screens stay here too, listed in `kept-local.txt` and `SCREENSHOTS.txt`. (verified)
- **26 tool answers** (001 to 026) carry the company number in their `company_number` field, so they stay here whole. The values above are quoted from them: `notes.txt`, `evidence/s9-10-tool-records.txt`, `evidence/s1-cmp.txt` and `evidence/s14-cmp.txt`. The tools list (000), the server's own logs and `evidence/tools-log.txt` (each call's round trip and answer sha256, no company number) are pushed. (verified)
- **Both raw answers** (a07 and sl), with their headers and `.parsed.txt`, are pushed whole: neither holds the company number or a counted run of 10 or more digits. (verified)

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
