# Results: #1633 export-ready, 28 Oct 2026. On TallyPrime 7.2, a GSTR-1 JSON export of a return holding a ready voucher raised the voucher mark ALTVCHID by 3, and the next voucher took the id after that rise. A second export raised it by 3 again. No voucher's alter_id moved across either export

The sheet is `lab/1633-export-ready` at 67f9fe5c5792dc014ffff693c5e1b36a1e4a29a5 (`SHA256.txt` 3 of 3).
- **Company:** BRIDGE SPAN MARK LAB, number 100089, GUID dbb847b4-8867-4e9b-ae3c-989ffa91b977. The 1633-L4 row left it as the sheet describes; READ 0 gave 215 / 9 and Sales No. 1 `uncertain`.
- **Build:** GS1-L1's build of pull request 1867 (`bridge_mcp.exe` sha256 bf8cdf82...6f46, the same before and after; bridge-tally 0.6.0). Tool calls only, over one stdio launch.
- **Tally:** TallyPrime 7.2 (Latest), Silver Rental, Licensed.
- **BK1:** `TDBK1800_100089.001`, 31,904 B, sha256 db2bf5cafa0d9e87c69eedecbd45acae5ab24dfd80099c84b0b32134996563e9. The marks were unchanged by the backup.

## Marks and ids at each read
| read | when | M | V | Sales No. 2 | Sales No. 3 |
|---|---|---|---|---|---|
| READ 0 | start | 215 | 9 | | |
| after step 2 | SYN SM Sales given HSN/SAC 998313 | 216 | 9 | | |
| READ 1 | | 216 | 9 | | |
| READ 2 | after Sales No. 2 (1,000.00 + 90.00 + 90.00) | 216 | **10** | 8 / 10 `included` | |
| READ 3 | GSTR-1 open | 216 | **10** | 8 / 10 | |
| READ 4 | after the first export, Upload GSTR-1 open | 216 | **13** | 8 / 10 | |
| READ 5 | after Sales No. 3 (500.00 + 45.00 + 45.00) | 216 | **14** | 8 / 10 | **12 / 14** `included` |
| READ 5b | GSTR-1 reopened | 216 | **14** | 8 / 10 | 12 / 14 |
| READ 6 | after the second export | 216 | **17** | 8 / 10 | 12 / 14 |

(master_id / alter_id from `gst_status`. Sales No. 1 stayed 1 / 1 `uncertain`. The Receipts SM-R1 2/2, SM-R4 5/6, SM-R5 6/8 and SM-R6 7/9 stayed `not_in_return` throughout.)

## The answers to the sheet's questions
- **Pending to Upload:** above 0 once a ready voucher was in the return. Upload GSTR-1 showed "Voucher Count - 0; Summary Count - 3" (Rajasthan 18%, HSN Summary, Document Summary). After each export it showed 0 and 0.
- **The export ran and wrote a file.**
  - X led first to a warning about "potential conflicts between the master and the transaction". That warning offered X: Export (Offline) and B: Back, and X was taken.
  - The export screen offered Excel by default; JSON was chosen with F8 and the folder with F6. "Export separate files for each Section" was left at No.
  - The accept key was **E: Export**.
  - Tally opened File Explorer on the folder; it was closed by its own button.
- **V4 − V3 = 3** (opening GSTR-1 moved nothing: V3 − V2 = 0).
- **V6 − V5b = 3:** the rise repeated, and matches the 7.1 figure of 3.
- **Sales No. 3's alter_id is 14 = V4 + 1**, not V3 + 1 = 11. The voucher took the next id after the export's rise, skipping the three ids the export took.
- **What took the ids:**
  - No Sales or Receipt voucher's alter_id moved across either export.
  - Sales No. 3's master_id is 12, where No. 2's is 8. So master ids 9 to 11 were also used between them, by objects no Sales or Receipt read returns.
  - The export screen counted "1 of 3 documents".
  - What those objects are is not identified here.

## Other observations
- **Step 2:**
  - On 7.2 the ledger's HSN/SAC history already held one row dated 1-Apr-25 ("As per Company/Group").
  - Changing the main screen did not show in that row before accepting, so the row itself was set to Specify Details Here, 998313, with the description Tally filled.
  - Taxability and rate were not touched.
- **No re-check question appeared** when GSTR-1 was opened after the master change, the first time or the second.
- **GSTR-1's own counts:**
  - After the first export, "Ready for Upload 1 / Not Uploaded 1" became "No Action Required 1".
  - After Sales No. 3 it read "Ready for Upload 2" and "Modified in Books After Upload/Export 2".
  - After the second export it read "No Action Required 2".
- **The second export offered no choice about vouchers already exported.**

## Files
- `answers/`: every tool answer, 000 to 024.
- `exports/export1/` and `exports/export2/`: the two exported files byte for byte.
  - 446 B, sha256 956e159c...819b.
  - 450 B, sha256 44d5d8e9...1658.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures and BK1 by sha256; they stay on the lab PC.
