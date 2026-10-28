# Results: #1633-L4, a deleted voucher's AlterID and MasterID, and a GSTR-1 export, on TallyPrime 7.2 Silver, 27-28 Oct 2026 (UTC)

The sheet is `lab/1633-l4/WINDOWS-1633-L4.md` at 0c0ed340 (SHA256.txt: 2 of 2 equal).

The company is BRIDGE SPAN MARK LAB, new and synthetic:
- number 100089, GUID dbb847b4-8867-4e9b-ae3c-989ffa91b977
- state Rajasthan, books from 1 April 2025
- GSTIN 08ZZZZZ0000Z1ZQ, Regular

Every mark below comes from ComplyEaze Bridge's own calls, on GS1-L1's build of #1867 (bridge_mcp.exe sha256 bf8cdf82...6f46):
- `ledger_masters`: snapshot master_alter_id (M), voucher_alter_id (V) and reused.
- `gst_status`: April 2025, Receipt, giving each voucher's master_id and alter_id.

The lab sent no request of its own.

## The READs
| READ | after | M | V | Receipts (number master_id/alter_id) |
|---|---|---|---|---|
| A | R1, R2, R3 keyed (and S1 Sales) | 215 | 4 | SM-R1 2/2, SM-R2 3/3, SM-R3 4/4 |
| B | SM-R3 deleted (the latest) | 215 | 5 | SM-R1 2/2, SM-R2 3/3 |
| C | SM-R4 keyed | 215 | 6 | SM-R1 2/2, SM-R2 3/3, SM-R4 5/6 |
| D | SM-R2 deleted (not the latest) | 215 | 7 | SM-R1 2/2, SM-R4 5/6 |
| E | SM-R5 keyed | 215 | 8 | SM-R1 2/2, SM-R4 5/6, SM-R5 6/8 |
| F | GSTR-1 open for April 2025 | 215 | 8 | as E |
| G | export tried (nothing pending), GSTR-1 open | 215 | 8 | as E |
| H | SM-R6 keyed | 215 | 9 | as E, and SM-R6 7/9 |

Every READ answered in full: snapshot.reused false, no refusal.

## What the READs show (recorded; the requester judges)
- **A deletion takes an AlterID.** Each deletion moved V by exactly one: 4 -> 5 for SM-R3 and 6 -> 7 for SM-R2. M did not move.
- **A deleted voucher's MasterID is not reused.** SM-R4 got master_id 5, not SM-R3's 4. SM-R5 got 6, not SM-R2's 3. SM-R6 got 7.
- **A new voucher's AlterID follows the mark after the deletion.** Each was the mark at the READ before it plus one:
  - SM-R4: alter_id 6 = B's 5 + 1
  - SM-R5: alter_id 8 = D's 7 + 1
  - SM-R6: alter_id 9 = G's 8 + 1
- **Opening GSTR-1 moved neither mark** (E = F).
- **The export question is unanswered on this run.** Step 11's export did not run, so READ G equals READ F:
  - "Upload GSTR-1" for April showed "Pending to Upload (Voucher Count - 0; Summary Count - 0)".
  - "X: Export (Offline)" did nothing, whether pressed or clicked: no export screen, no message, no file. "S: Send (Online)" was never touched.
  - GSTR-1 counted the Sales voucher S1 under "Uncertain Transactions (Corrections needed)", with the reason "HSN/SAC is invalid, mismatched, or not specified" (the sheet's SYN SM Sales has no HSN/SAC).
  - Whether a GSTR-1 export raises the voucher mark, and whether the next voucher skips it, is therefore not measured here.

## On the screen (7.2), recorded
- **GST registration is a master.** F11's "Set/Alter Company GST Rate and Other Details" opens HSN/SAC, rate and e-Way Bill settings, but no registration fields. The registration lives in Statutory Masters > GST Registration, where Tally had already made "Rajasthan Registration". That screen has no registration "applicable from" field.
- **F11 has no "Edit Log" wording.** Under Others it shows "Mark modified vouchers : No".
- **The tax types are IGST / CGST / SGST/UTGST / Cess.** S1's CGST and SGST were each calculated by Tally at 90.00, for a total of 1,180.00.
- **The Sales voucher opened "Party Details"**: a Dispatch Details box, then a buyer box. The buyer box showed Cash, Unregistered/Consumer, Place of Supply Rajasthan. Both were passed with Enter only and their values recorded.
- **Manual numbering.** With Receipt numbering set to Manual, the screen added "Prevent creating duplicate Voucher Nos." (offered No, kept).
- **The GST menus sit under Statutory Reports.** On 7.2 the path is Display More Reports > Statutory Reports > GST Reports.

## Incident
TallyPrime quit during step 3.
- **Cause:** a second Ctrl+Q, sent in the same batch as the first, landed on the Gateway of Tally and quit it with no prompt. Everything before had been saved with Ctrl+A.
- **Restart:** Tally was restarted from its own shortcut and one tally.exe again held 9001. A configured startup company loaded and was shut.
- **Marks:** after the restart the master mark read 211. It had been 209 before the Receipt voucher-type alteration, and there was no read between that alteration and the quit. So 211 = 209 + the alteration + one more step whose cause is UNVERIFIED (the alteration, the quit or the restart).
- **Check:** a read-only look confirmed Manual numbering stuck, and the mark stayed 211.

## Control and book state
- **Backup:** BK0 was taken before the first deletion: `TDBK1800_100089.001`, 31,565 B, sha256 21bab2d2d50dd705b78809bcb081180cdf2989a2715291ec1e56b98e1b473348. It stays on the lab PC, and the backup left the marks unchanged.
- **The book is kept and shut.** It holds:
  - S1 (Sales 1,180.00)
  - SM-R1, SM-R4, SM-R5, SM-R6 (Receipts of 1-Apr-25)
- **C10:** no company loaded; one tally.exe holds 9001; /status 200.

## Files
- `answers/`: every tool answer whole, and `client-log.txt`.
- `notes.txt`: UTC, one line per step.
- `SCREENSHOTS.txt`: every capture and the backup, by name, bytes and sha256. They stay on the lab PC.
