# Results: #1633 GSTR-3B export, 28 Oct 2026. On TallyPrime 7.2 a GSTR-3B JSON export through the local Export key did not move the voucher mark: V stayed 17 across it, where each GSTR-1 export on this book raised it by 3. Opening GSTR-3B moved nothing, and no voucher's `alter_id` moved. Sales No. 4 then took `alter_id` 18. With no rise, No. 4 decides nothing about skipping. The book is kept

The sheet is `lab/1633-gstr3b-export` at 3f5216d18e820671216d18060711734409249a4a.
- **Sheet checks:** `SHA256.txt` 3 of 3 equal.
- **Company:** BRIDGE SPAN MARK LAB, 100089, GUID dbb847b4-8867-4e9b-ae3c-989ffa91b977, `verified_tuple` (the GSTR-1 export row's guid and number).
- **Build:** pull request 1867's build (`bridge_mcp.exe` sha256 bf8cdf82...9f46), the same before and after.
- **Tally:** TallyPrime 7.2, Silver Rental, Licensed. Current date 1-Apr-2025 throughout.
- **BK1:** `TDBK1800_100089.001`, 38,756 B, sha256 d242ae97d1ad28457493ed4f8f263f3661c0e2c4aa120aad6a84d7bd3f178307. No restore is planned; it stays on the lab PC.

## Marks and ids
| read | point | M | V | Sales (number master/alter) | Receipts |
|---|---|---|---|---|---|
| READ 0 (001-003) | start | 216 | 17 | 1: 1/1 uncertain; 2: 8/10; 3: 12/14 included | SM-R1 2/2, SM-R4 5/6, SM-R5 6/8, SM-R6 7/9, not_in_return |
| READ 0b (004) | after BK1 | 216 | 17 | | |
| READ 1 (005-007) | GSTR-3B open | 216 | 17 | unchanged | unchanged |
| READ 2 (008-010) | after the export, Upload GSTR-3B open | 216 | 17 | unchanged | unchanged |
| READ 3 (011-013) | after Sales No. 4 | 216 | 18 | plus 4: 13/18 included | unchanged |

Every `snapshot.reused` was false.

- **V1 − V0b:** 0.
- **V2 − V1:** 0, and M did not move. The GSTR-1 row gave +3 for each of two exports.
- **No. 4:** `alter_id` 18 = V1 + 1 = V2 + 1, so the reuse and skip cases coincide. `master_id` 13, next after No. 3's 12.

## The export (step 3)
- **GSTR-3B (April 2025):** registration 08ZZZZZ0000Z1ZQ, Status Not Filed, "Last online GST activity: No Activity Found".
  - Vouchers: Total 7, Included 2, Not Relevant 4, Uncertain 1, potential conflicts Yes.
  - 3.1: taxable 1,500.00, CGST 135.00, SGST 135.00, tax 270.00. The other sections had no amounts.
  - No question about altered masters appeared.
- **Alt+E menu:** Current (Ctrl+E), Others, GST Returns, Masters, Transactions, Configuration. GST Returns opened Upload GSTR-3B directly, with no list of returns.
  - The screen showed Total Liability 1,500.00 / 270.00 and a note that section 3.2 is auto-filled on the portal from GSTR-1.
  - Its keys were **X: Export (Offline)** and S: Send (Online). Only X was used.
- **Warning:** the same conflicts warning as GSTR-1 ("…Still, GSTR-3B data will be exported to the portal as per the transaction."), answered by X.
- **Export screen as offered:** Excel (Spreadsheet), single file No, template `GSTR3B_Excel_Utility_V5.0.xlsm`.
  - F8 offered Excel and JSON (Data Interchange); JSON was chosen.
  - F6 set the folder to `1633-3b\export`. E: Export accepted it.
- **Progress:** "Exporting GSTR-3B data / Rajasthan Registration (08ZZZZZ0000Z1ZQ) - Apr-25 / 0%".
- **After the export:** File Explorer opened on the folder and was closed by its own close button. GSTR-3B looked the same afterwards (no new status line).
- **File:** `export/08ZZZZZ0000Z1ZQ_GSTR3B_APR-2025_28102026_184709.json`, 1,079 B, sha256 9f0eb3fbb6188e4fe7bf9794a8d7266805ee0a8aeb1813c3cdbfd1c4e655dc5e, kept byte for byte.
  - Top-level keys: `gstin`, `ret_period` (042025), `sup_details`, `itc_elg`, `inward_sup`, `intr_ltfee`.
  - `sup_details/osup_det`: txval 1500, camt 135, samt 135; the other values are 0.

## Sales No. 4 (step 4)
- **Form:** Sales No. 4 opened in the item-invoice form. Change Mode offered Item Invoice, Accounting Invoice and As Voucher; Accounting Invoice was chosen, the Particulars form the GSTR-1 row recorded.
- **Party:** Cash, 1-Apr-25.
- **Sub-screens:** Dispatch Details all blank. Party Details: Cash, Rajasthan, India, Unregistered/Consumer, Place of Supply Rajasthan. Both passed by Enter.
- **Lines:** SYN SM Sales 500.00; SYN SM CGST 45.00 and SYN SM SGST 45.00, as Tally calculated; total 590.00; narration SM-S4.
- **Save:** Ctrl+A with no message; a blank Sales No. 5 opened.

**Confidence:** one export, PARTIAL until repeated. The file covers Nos. 2 and 3 only, because No. 4 was keyed after it.

## Files
- `answers/`: the tool answers, 000 to 013.
- `export/`: the exported file.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures and BK1 by sha256; they stay on the lab PC.
