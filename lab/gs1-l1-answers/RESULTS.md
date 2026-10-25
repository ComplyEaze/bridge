# Results: GS1-L1 v3.1 on TallyPrime 7.2, BRIDGE GST LEDGER LAB (25 Oct 2026). PARTIAL: stopped at a clean point in step 5 for an account hand-over

The item's files are at `lab/gs1-l1` @ ee573991 (`SHA256.txt` 5/5 OK; `check-request-ids.py`: `census: ok 2, skipped 0, FAIL 0`). TallyPrime Release 7.2 (Latest), Silver Rental (not Educational), `tally.exe` 1.1.7.2, one process on port 9001. The owner said in the lab's chat that the lab keys the OWNER steps itself; every keyed act below was done on Tally's own screens by the lab, one act at a time, with a screenshot (kept on the lab PC) and a MARKS read after each master and voucher.

**Build (for steps 6 to 8, not yet started).**
- PR #1867 head 796e94f1, CI run 40167319272, artifact 12589749758: zip 61,285,927 B, digest 1f9e76d0ca5672c5a071daf581af9d9a50d18a6f15d11c357d0452a3098fa1d4.
- `bridge-tally.mcpb` dd67d3fd… (equal to `archive_sha256`).
- `bridge_mcp.exe` 33,768,448 B, sha256 **bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46** (equal to `binary_sha256`): the pinned hash.
- Staged only; nothing installed.

**Recorder.** The original recorder (5f36ad42…) was not found on the lab PC or in the repository. It is treated as lost by the run's review. A replacement (the replacement recorder's branch (name in the local hand-over file) @ 5b6fbfca, its script 00bd8411…) was offered for steps 6 to 8. It has not been fetched, self-tested or run yet. If it fails its self-test or refuses a call Bridge makes, steps 6 to 8 use the no-recorder record (tool results plus egress receipts, no wire bytes). Its third review round's fixes were not re-reviewed.

**Terms.** `BRIDGE_TERMS_ACCEPTED=true` for the stdio runs, with the owner as the person accepting the Terms for the lab.

## THE COMPANY
BRIDGE GST LEDGER LAB, GUID **aae372c5-a894-491c-8843-5532e43d2d2f**, FY and books from 1-4-2025. The company number is recorded on the lab PC only.

## Steps done
| Step | What | Marks after (ALTVCHID / ALTMSTID) |
|---|---|---|
| 0 | No company loaded. One tally.exe on 9001, /status 200. About read 7.2 (Latest), Silver Rental. No company named BRIDGE GST LEDGER LAB existed. Build hash as above | (no company) |
| 1 | Company created, financial year and books from 1-4-2025, no password, other fields as offered. Its Features screen opened by itself (see below), accepted unchanged. STATE: one COMPANY row, GUID above; LEDGERS: Cash, Profit & Loss A/c | none / 207 |
| 2 | 16 ledgers, one at a time (ALTMSTID +1 each, 208 to 223). Then unit Nos (224) and stock item GS1 Item (225), because inventory read Yes. The unit is not in the sheet's list but the stock item needs one | none / 225 |
| 3 | STATE: exactly 18 ledgers (the 16 plus Cash and Profit & Loss A/c), parents as the sheet. Backup **GS0** (C5) into a new empty folder: `TDBK1800_<company number>.001`, 25,966 B, sha256 **3c804b2a5cd72a890bb61fd21338236e78dbd118a6c9a5a85792ba05f9e0bdde**, 16:28 IST; one file after. MARKS unchanged | none / 225 |
| 4 | V1, V4, V5, V6, V7, V4b, V8, V9, V10, V11, V13, V14, then V2 and V3 (inventory Yes). ALTVCHID +1 each | 14 / 225 |
| 5 (part) | Duplicates of V1 by Alt+2 on V1 in the Day Book, only the date changed: **40 of 50 done**, five a day on 18, 19, 20, 21, 22, 23, 24 and 25 Nov. MARKS after every fifth: 19, 24, 29, 34, 39, 44, 49, 54. LEDGERS read at the stop equals step 3 | **54 / 225** |

## Observations to carry into the item's findings
1. **Features (step 1).** On TallyPrime 7.2 the Features screen opened by itself after Create Company, with "Enable Goods and Services Tax (GST)" already **Yes** and "Set/Alter Company GST Rate and Other Details" No. No registration details were asked, and none were entered (no State, no GSTIN). Maintain Inventory **Yes**, Integrate Accounts with Inventory Yes. It was accepted unchanged.
2. **Masters.**
   - Tally 7.2 names the GST tax types IGST, CGST, SGST/UTGST and Cess; the sheet's Central, State and Integrated Tax were keyed as CGST, SGST/UTGST and IGST.
   - Purchase, expense and asset ledgers offered "GST applicability: Applicable", with HSN and rate "As per Company/Group"; these were left as offered.
   - Parties offered bill-by-bill Yes and registration Regular, with no GSTIN.
3. **Voucher view.**
   - 7.2's "As Voucher" is reached by Ctrl+H (Change Mode), since Ctrl+V did nothing. Payment uses "Double Entry".
   - In Purchase As Voucher, Tally puts the party (Credit) line first.
   - Purchase, Debit Note and Credit Note show Receipt/Dispatch Details and Party Details sub-forms, accepted blank or as offered.
4. **Balancing fill.**
   - Tally pre-fills every new line with the remaining balance; in journals and payments it also pre-fills the last ledger used.
   - Every pre-filled amount was overtyped with the sheet's figure.
   - Tally never computed a tax amount. In V2 and V3, the item-invoice tax lines opened with an **empty** amount.
5. **Bill-wise.** Every party line was given a New Ref named after the voucher (V4, V5, V6, V7, V4b, V8, V14, V2, V3); V1's is "1". Tally's default was sometimes Agst Ref or On Account. The Alt+2 copies of V1 carry V1's New Ref "1".
6. **Debit Note V8 and Credit Note V14.** "Provide GST details" defaulted to **Yes**, and the Additional Details screen's "Reason for Issuing Note" defaulted to Not Applicable; both were kept.
7. **Negative Cash (V11 and V13).**
   - Tally showed the dialog "Warning / Negative Cash! / ₹ (-)200.00 / Press any key to continue" on V11 (Payment, To Cash 200), and "₹ (-)430.00" on V13 (To Cash 230).
   - The item stopped at the first one. On the run's review's decision, each was dismissed with one key and the voucher saved as keyed.
8. **Corrections before saving** (nothing saved wrong):
   - V13's second line first went in as To (credit) Output SGST. It was changed to By before saving.
   - V3's first attempt put the freight 100 into the line's Rate (%) column, so Tally computed 5,000. That voucher was abandoned unsaved with Ctrl+Q (no prompt, marks unchanged) and keyed again with the rate blank.
9. **Session date.** Tally's current date moved with the voucher dates; the Gateway read 25-Nov-2025 at the stop. It is a session value, not data.

## Vouchers in the book at the stop
| Voucher | Date | Type, number | Lines |
|---|---|---|---|
| V1 | 3 Nov | Purchase 1 | Cr SYN GS1 Supplier 11,800; Dr GS1 Rent 10,000; Dr Input CGST 900; Dr Input SGST 900 |
| V4 | 4 Nov | Journal 1 | Dr GS1 Professional Fees 20,000; Dr Input IGST 3,600; Cr party 23,600 |
| V5 | 5 Nov | Purchase 2 | Cr party 47,200; Dr GS1 Computer 40,000; Dr Input CGST 3,600; Dr Input SGST 3,600 |
| V6 | 6 Nov | Purchase 3 | Cr party 2,360; Dr GS1 Freight Direct 2,000; Dr Input CGST 180; Dr Input SGST 180 |
| V7 | 7 Nov | Purchase 4 | Cr party 11,800; Dr GS1 Rent 5,000; Dr GS1 Professional Fees 5,000; Dr Input CGST 900; Dr Input SGST 900 |
| V4b | 8 Nov | Journal 2 | Dr GS1 Professional Fees 10,000; Dr Input IGST 1,800; Cr GS1 TDS Payable 1,000; Cr party 10,800 |
| V8 | 10 Nov | Debit Note 1 | Dr party 1,180; Cr GS1 Rent 1,000; Cr Input CGST 90; Cr Input SGST 90 |
| V9 | 11 Nov | Journal 3 | Dr Output CGST 500; Cr Input CGST 500 |
| V10 | 12 Nov | Journal 4 | Dr GS1 Rent 180; Cr Input CGST 90; Cr Input SGST 90 |
| V11 | 13 Nov | Payment 1 | Dr Output CGST 100; Dr Output SGST 100; Cr Cash 200 |
| V2 | 14 Nov | Purchase 5 (Item Invoice) | GS1 Item 1 Nos @ 5,000 (Purchase - Goods); Input CGST 450; Input SGST 450; party Cr 5,900 |
| V13 | 15 Nov | Payment 2 | Dr Output CGST 90; Dr Output SGST 90; Dr GS1 Interest on GST 50; Cr Cash 230 |
| V14 | 16 Nov | Credit Note 1 | Cr SYN GS1 Customer 1,180; Dr GS1 Discount Allowed 1,000; Dr Output CGST 90; Dr Output SGST 90 |
| V3 | 17 Nov | Purchase 6 (Item Invoice) | GS1 Item 1 Nos @ 5,000; GS1 Freight Inward 100; Input CGST 450; Input SGST 450; party Cr 6,000 |
| copies of V1 | 18 to 25 Nov | Purchase 7 to 46 | as V1; five a day, 40 in all |

## Left to do (resume from here)
1. **Step 5:** the remaining 10 copies of V1, five on 26 Nov and five on 27 Nov, the same way: Day Book for 3-Nov, Alt+2 on V1, F2 to the date, Ctrl+A. Then STATE, expecting ALTVCHID 64, which is 50 more than step 4's 14. The sheet says "at least 62 (64 with V2 and V3) over step 3", and step 3 had no voucher mark.
2. **Recorder:** fetch the replacement recorder's branch (name in the local hand-over file) @ 5b6fbfca, check its script against its SHA256.txt, run `--self-test` (expect 4 forwarded, 40 refused, none reaching Tally), then listen on 9011 with a new recording folder.
3. **Steps 6 to 10:** with the pinned build bf8cdf82 over stdio, the same recorded deviation as before.
4. **Optional steps 11 to 14:** the GS1-X1 rerun tail.
5. **C10.**

THE COMPANY is shut now; Tally is at Select Company with no company loaded.

## In this commit
- `answers/`: every MARKS and LEDGERS answer of steps 1 to 5 with headers, and `send-log.txt` (sizes and sha256).
- Screenshots, the About capture, the data-folder listing and the GS0 backup stay on the lab PC.
