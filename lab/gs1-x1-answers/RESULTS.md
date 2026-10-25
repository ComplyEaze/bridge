# Results: GS1-X1 v2, what TallyPrime 7.2's "Data for GSTR-9" export fills (25 Oct 2026)

Windows lab PC, TallyPrime Release 7.2 (Latest), Silver Rental (not Educational), port 9001. THE COMPANY: BRIDGE GST RECON LAB, GUID ae1490be-52c5-4544-9ffc-4b7da85f9797 (its registration on the book is a synthetic test GSTIN). Sheet `lab/gs1-l1-run-sheet` at 2cb11c60 (`lab/gs1-x1/SHA256.txt` 4/4 OK; `check-request-ids.py`: census ok 1, skipped 0, FAIL 0; the item began on edd6e156 and the requester's v2 changed only the backup rule and the request's location, the MARKS file being byte-identical). LAB-COMMON v3 from `lab/626-1038-1050-run-sheets` at 8c602c15. No ComplyEaze Bridge program used. The owner approved, in the lab's chat, the utility download and handing the template to Tally, and answered the two Excel dialogs himself.

## Utility (step 2)
Public page https://www.gst.gov.in/download/gstr9 ("GSTR-9 Offline Tool (v2.2)"), link https://tutorial.gst.gov.in/offlineutilities/returns/GSTR_9_Offline_Utility.zip: 1,098,100 bytes, sha256 4c2a6144b21b9113b81e1156f58c14c7802c52dec2a2956388758e644a1d65c1. It holds `GSTR-9 Offline Tool (v2.2).xlsm` (2,698,429 bytes, sha256 1499420ba35a94a4e6cbac538b38e5032d6d281b0bca0f1278797ad4b0639e0b) and `ReleaseNotes.txt` ("Release 2.2 (28/09/2026)", copied here). `workbooks/template-blank.xlsm` is that workbook, never opened.

## MARKS (every answer in `answers/`, sends in `evidence/send-log.txt`; one COMPANY row each, THE COMPANY's GUID)
| When | ALTVCHID | ALTMSTID | Bytes, sha256 |
|---|---|---|---|
| step 0, step 1, after No, after backup X0 | 65 | 293 | 3,402, 66b34676… |
| right after Yes on the dialog below | 65 | 293 | 3,404, 9175530c… (only the CMPINFO count line `VOUCHER` changed, 0 -> 40; not a row, C4) |
| after the All-Sections export (step 5) | 65 | 293 | 3,404, 9175530c… |
| after the HSN-only export (step 7) | 65 | 293 | 3,404, 9175530c… |
| after restoring X0 | 65 | 293 | 3,402, 66b34676… (identical to before the Yes) |

## Findings
1. **verified: opening GST Annual Computation writes to the book first.** Go To > GST Annual Computation showed, verbatim: "GST-related details were altered in the masters/Return. Hence, opening the report will update the GST Status of 20 vouchers. Do you want to open the report?" [Yes] [No]. The first time it was left up (a stop under the sheet, nothing pressed), then answered No: no change. Backup X0 was taken (`TDBK1800_<company number>.001`, 94,275 bytes, sha256 2fef4ba1f24460659fe4f802ead93b624e3cf4e03c7e1b12818f9a985a77f8cc). Then the report was reopened and answered Yes. Yes did not move ALTVCHID or ALTMSTID; the only change seen in the MARKS answer was the CMPINFO `VOUCHER` count line, 0 -> 40.
2. **verified: the report** (1-Apr-25 to 31-Mar-26): Total Vouchers 63; Included in Return blank; Not Relevant for This Return 16; Uncertain Transactions (Corrections needed) 47. Liability, Outward/Inward taxable, Not payable, Missing Invoice, Total Liability, Input Tax Credit, Reversal and Ineligible ITC, Total ITC, Interest, HSN/SAC Summary, Summary of Outward and Inward Supplies: no amounts printed.
3. **verified: the export dialog's defaults.** Sections All Sections; Excel; Template File Name "GSTR-9 Offline Tool (v2.1).xlsm" (the portal now serves v2.2; the v2.2 copy was accepted); template and output folder both the program folder; "Open file after Export" Yes. The lab set the template and output folders as the sheet says, and set "Open file after Export" to No so that Excel would not be opened (a deviation from the sheet's keys, recorded).
4. **verified: Tally runs Excel hidden, and Excel's Document Inspector warning stops the export.** After E and Tally's own prompt ("If the export process does not proceed, press Alt+Tab to check for any Microsoft Excel message in the background and click OK on the message box", answered C Continue), the export sat at "18 HSN Inward, Updating Row Number: 5". A hidden Excel (its window titled "Excel (Unlicensed Product)") showed "Be careful! Parts of your document may include personal information that can't be removed by the Document Inspector." [OK] [Cancel]. The sheet's stop rule held the item until the owner clicked OK; then the export completed. The same happened on the HSN-only export. The exported workbooks carry empty Creator and Last Modified By properties.
5. **verified: what the All-Sections export fills** (`evidence/cells-all.txt`, x1-cells exit 0; the exported workbook stays on the lab PC: 2,418,399 bytes, sha256 192ffc355cf53305f727c4c7b9b30c3581def86559dae671a02d429e4017e0b7). Home: B7 the book's GSTIN, D7 "2025-26". Every other cell written is the value 0: 4 Outward 36 cells, 5 Outward 8, 6 ITC availed 50, 7 ITC Rev 28, plus its four totals D17:G17 rewritten as `SUM(x7:x16)` formulas, 10 to 13 PY trans 8. Left exactly as the blank: the Read Me sheets, 8 Other ITC, 9 Tax Paid, 14 Differential tax, 15 Demand & Refunds, 16 Comp DS & Goods sent on appr, 17 HSN Outward, 18 HSN Inward, Master.
6. **verified: what the HSN-only export fills** (`evidence/cells-hsn.txt`, exit 0; the exported workbook stays on the lab PC: 2,418,290 bytes, sha256 da726cd620991cd1f214274a3b70c444d6f122752689e5e26f3d55ab51b50dd6): Home B7 and D7 only; 17 HSN Outward and 18 HSN Inward unchanged.
7. **verified: neither export moved the marks** (table above).
8. **verified: restore X0** (C6, `evidence/restore-plan.txt`): the backup's hash was checked, the list showed THE COMPANY backed up 25-Oct-26 12:27 pm, Accept Yes, Replace. After the restore the MARKS answer is byte-identical to the one before the Yes.

## Close (C10)
Shut THE COMPANY (Company > Shut, Enter); one tally.exe owning 9001; GET /status 200; Select Company with no company loaded.

## Kept on the lab PC (not committed)
- The two exported workbooks (hashes above): they hold the book's GSTIN as cell data.
- Screenshots of the step 0 window, the first dialog, the backup and restore screens, the report, the export defaults and both configurations, and the Excel warning; each listed by name, size and sha256 in the lab's notes. The step 0 and close command outputs.

## Masking in this commit
In `evidence/cells-all.txt` and `evidence/cells-hsn.txt` the value of Home!B7 (the book's GSTIN) is replaced by `[GSTIN of THE COMPANY]`; every other line, cell and count is as `x1-cells.py` printed it. The unmasked files and their hashes stay on the lab PC. `workbooks/template-blank.xlsm` is the portal's own file, byte-identical (1499420b…), with no company data.
