# W11: where Tally takes a sales ledger's HSN/SAC from (Windows lab, 12 Oct 2026)

Request set: lab/1342-capture-requests 787cf21b (workorder/W11-README.md). Each request was hash-checked against the set's SHA256.txt before sending, and every request was sent once. Every answer below is a complete envelope with one COLLECTION and no LINEERROR (check.py). Curl exit codes and printed lines are in `send-log.txt`.

Book: synthetic BRIDGE PILOT LAB (company number 100033), loaded alone. Backups: `SHA256-backups.txt`
- backup-pre2: taken before resuming.
- backup-B1: taken before the W11-R re-check.

Screenshots stay on the lab PC and are listed in `SHA256-screenshots.txt`. Mouse-click mode (no Bridge post in W11).

## Per voucher (read-back fields, compared by structure: `vfields.py`)

All vouchers: BRIDGE Sales, 7-Aug-2026, BRIDGE Walk-in, invoice mode, one 100.00 line, CGST 2.50 + SGST 2.50 filled by Tally, saved with no dialog.

| Step | Voucher | No. | Sales ledger | HSNSOURCETYPE | HSN source | GSTHSNNAME | GST status at save |
|---|---|---|---|---|---|---|---|
| A | W11-01 | 5 (Tally's suggestion) | INH (Sales Accounts) | (empty) | (empty) | (empty) | Uncertain |
| A | W11-02 | 6 (typed) | INH2 (BRIDGE Sales Sub) | (empty) | (empty) | (empty) | Uncertain |
| B company 998341 | W11-03 | 7 (typed) | INH | Company | | 998341 | Included |
| B | W11-04 | 8 (typed) | INH2 | Company | | 998341 | Included |
| C + Sales Accounts 998342 | W11-05 | 9 (typed) | INH | Group | Sales Accounts | 998342 | Included |
| C | W11-06 | 10 (typed) | INH2 | Group | Sales Accounts | 998342 | Included |
| D + BRIDGE Sales Sub 998343 | W11-07 | 11 (typed) | INH2 | Group | BRIDGE Sales Sub | 998343 | Included |
| E INH own 998344 | W11-08 | 12 (typed) | INH | Ledger | BRIDGE Svc INH 5% | 998344 | Included |
| F INH + row 998345 from 1-Sep-2026 | W11-09 | 13 (typed) | INH | Ledger | BRIDGE Svc INH 5% | 998344 | Included |
| G new ledger 4D, own 9983 | W11-10 | 14 (typed) | BRIDGE Svc 4D 5% | Ledger | BRIDGE Svc 4D 5% | 9983 | Included |

- GSTHSNINFERAPPLICABILITY is "As per Masters/Company" on every voucher.
- Status comes from VCHGSTSTATUSISINCLUDED / ISUNCERTAIN / ISEXCLUDED in the read-back. It matches GSTR-1.

Reading [inferred]:
- Order of precedence: ledger's own code > nearest group (the sub-group beats its parent) > parent group > company.
- A history row dated after the voucher date is ignored.
- A four-digit code is accepted and the voucher is included. The company's GST details read "Minimum length of HSN/SAC: 4 (based on annual turnover)".
- The source is stamped on the voucher when it is saved; see W11-R.

## Masters (each change saved with no prompt)

The HSN/SAC code and its applicable-from date are set in More Details > "GST - HSN/SAC & Related Details (History)". In the reads they are carried in HSNDETAILS.LIST (APPLICABLEFROM, HSNCODE, HSN, SRCOFHSNDETAILS).

- **Company (B):** the 1-Apr-26 history row had read Not Defined; it was set to 998341.
  - aw11-company-1 does NOT contain the company-level code: only the CMPINFO counts and CMPVCHSTATID (22 → 24) differ from aw11-company-0. The company request does not return the company's HSN/SAC.
- **Sales Accounts (C):** history was empty; added a row 20260401 998342 "Specify Details Here".
  - Tally also wrote a GSTDETAILS.LIST row (20260401, "As per Company/Group", rates 0). ALTERID 24 → 239.
- **BRIDGE Sales Sub (D):** the 1-Apr-26 row went from "As per Company/Group" to Specify Details Here, 998343. ALTERID 235 → 240.
- **BRIDGE Svc INH 5% (E, F):** the row went from "As per Company/Group" to 998344 (ALTERID 236 → 241). F then added the row 20260901 998345 (ALTERID 242).
  - After F, the main ledger screen shows the latest row (998345), not the one in force on the voucher date.
- **BRIDGE Svc 4D 5% (G):** created under Sales Accounts: Services, Taxable, 5%, own code 9983.
  - Entering the code on the main screen left the history empty, so the 1-Apr-26 row was added explicitly.

## GSTR-1, August 2026 (summary screens, look only)

| After | Total | Included | Not relevant | Uncertain | Notes |
|---|---|---|---|---|---|
| A | 27 | 14 | 6 | 7 | HSN-uncertain: Vch 5, 6 ("HSN/SAC is invalid, mismatched, or not specified"); 5 others are a pre-existing tax-amount mismatch |
| B | 29 | 16 | 6 | 7 | new line "Check Vouchers Having Potential Conflicts with Masters: Yes" |
| C | (dialog; see W11-R) | | | | |
| W11-R | 31 | 18 | 6 | 7 | after the re-check; conflict line gone; Vch 5, 6 still HSN-uncertain |
| D | 32 | 19 | 6 | 7 | conflict line Yes |
| E | 33 | 20 | 6 | 7 | |
| F | 34 | 21 | 6 | 7 | |
| G | 35 | 22 | 6 | 7 | |

HSN Summary (Table 12, B2C) after G:
- 998341: 210.00 (2 vouchers)
- 998342: 210.00 (2)
- 998343: 105.00 (1)
- 998344: 210.00 (2)
- 9983: 105.00 (1)
- 998345 does not appear.

The voucher-level tax analysis of W11-07 and W11-08 was not opened (it is reached through Voucher Alteration). The HSN summary and the read-backs show the code each voucher used.

## W11-R: the master-change re-check (added step, added per the run's decision record)

Opening GSTR-1 after step C raised a dialog the README does not list. Exact text:

> GST-related details were altered in the masters, hence voucher status for GST reporting will change.
> Number of Vouchers in the Current Period: 23. Number of Vouchers in All Periods: 24.
> Select the period for which you want to check the vouchers.

Buttons: "A: All Periods" and "C: Current Period". The lab stopped and reported.

1. **Esc:** closed GSTR-1 entirely, back to Gateway; GSTR-1 did not open without the re-check.
2. **Before the re-check:** backup B1, then reads `aw11-readback-n01..n06-preR.xml`. These are structurally identical to the first reads (852 fields, 0 differing).
3. **The re-check:** reopened GSTR-1, same dialog → "C: Current Period". The screen showed "Updating the GST status in the vouchers 100% 23 of 23 vouchers", then GSTR-1 opened; there was no other prompt.
4. **After the re-check:** reads `-postR`. On all six vouchers preR → postR is 0 of 852 fields differing (`vcmp.py`).
   - Vch 5/6: ALTERID 36/37 unchanged; HSNSOURCETYPE and GSTHSNNAME empty; VCHGSTSTATUSISUNCERTAIN Yes.

Reading [inferred]: the re-check does not re-derive the HSN source of vouchers saved before a code existed. They stay uncertain until they are re-saved.

The dialog did not reappear when GSTR-1 was opened after D, E, F or G.

## Deviations from the README

- **Typed voucher numbers:** BRIDGE Sales is "Manual" numbering with "Prevent creating duplicate Voucher Nos." = Yes. Tally suggested 5 for W11-01 but left No. blank afterwards, so the lab typed 6–14 (accepted by the decision record).
- **Date shift:** an Enter on the blank No. field moved Tally's current date to 8-Aug-2026. The form was quit unsaved and the voucher date set back to 7-Aug-2026 with F2.
- **Wrong group opened twice:** Alter > Group opened "Bank Accounts" (the top of the list); each time it was quit with Ctrl+Q unchanged.
- **History rows:** applicable-from dates were set through the HSN/SAC history screen rather than the main screen (see Masters).
- **aw11-taxunits-0.xml:** kept on the lab PC (it holds the synthetic lab GSTIN); listed by hash in `SHA256-kept-answers.txt`.
- **Excluded material:** the earlier stop on this item (12:21 IST, before the request-set switch) is excluded from this branch, as instructed. It goes to a private hand-over.
