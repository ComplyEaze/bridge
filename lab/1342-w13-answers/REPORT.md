# W13: what the GSTR-1 JSON export holds for an uncertain voucher (Windows lab, 12 Oct 2026)

Book: synthetic BRIDGE PILOT LAB, company number 100033, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85. It was loaded alone. TallyPrime 7.1 Silver.
- Request set: lab/1342-capture-requests @ 99e750ac. W13-README.md sha256 9b8cfa22….
- Every request matched SHA256.txt or SHA256-top.txt before it was sent. The two extra baseline requests are W12 request files, checked against the W12 SHA256.txt (copied here as `SHA256-w12.txt`).
- CLICK MODE: lane. Every button and Yes was clicked with the mouse. "S: Send (Online)" was never used.
- **Starting state:** the book as W12 left it, per the orchestrator's instruction (no restore). 0003 was uncertain after its amount edit, and 0006 was accepted.
- Accept As Is was used on **BP/26-27/0005 only**. No other voucher or master was altered by hand.
- Fresh backup before any change: `<lab>\w13\backup-pre\TDBK1800_100033.001`, 79,079 B, sha256 037e04e82c5d13b262c7c2c2d2880bab85f398bebe825e180f662ba501e28633, 12:27Z. It stays on the lab PC.
- Screenshots stay on the lab PC, listed by hash in `SHA256-screenshots.txt`. This differs from the README, which asks for screens in `workorder/`.
- Sends: 14, one at a time, each with curl exit 0 and printed `200 <bytes>`. Every answer is a complete ENVELOPE with a COLLECTION element and no LINEERROR. `send-log.txt` holds every line.

## Answers
| When | Answer | Bytes | sha256 (first 8) | Note |
|---|---|---|---|---|
| Step 1 | aw13-marks-0.xml | 3,402 | 818854dd | ALTVCHID 50; byte-identical to W12's last marks |
| Step 1 | aw13-status-window-0.xml | 86,786 | 453b72c1 | 28 Aug Sales vouchers |
| Step 1 | aw13-status-0005-0.xml | 78,422 | 1e92f002 | 0005 ALTERID 14, uncertain |
| Baseline (extra) | aw13-x-status-0003-0 / aw13-x-full-0003-0 | 64,948 / 167,620 | ba132e62 / bdd1d989 | byte-identical to W12 step 10 |
| Baseline (extra) | aw13-x-status-0006-0 / aw13-x-full-0006-0 | 64,952 / 167,612 | e0b77230 / 80369f91 | 0 voucher-field changes since W12 step 8 |
| After export a (extra) | aw13-x-marks-after-export-a / aw13-x-status-window-after-export-a | 3,402 / 86,786 | 9f0ba0ac / 84f974a1 | see Finding |
| Step 5 | aw13-marks-1 / aw13-status-window-1 / aw13-status-0005-1 | 3,402 / 86,788 / 78,424 | 42c31a18 / cafb3bbf / 066a007b | |
| Before export b (extra) | aw13-x-marks-before-export-b | 3,402 | 42c31a18 | identical to marks-1 |
| After export b (extra) | aw13-x-marks-after-export-b / aw13-x-status-window-after-export-b | 3,404 / 86,790 | 1e603b13 / 0a9c623e | |

Full hashes are in `SHA256-answers.txt`, which also gives published-copy hashes; see "Published copies".

## Before (steps 1–2)
**Window read (28 BRIDGE Sales vouchers, 1–7 Aug):**
- Uncertain: 0002 (ALTERID 11), 0003 (50), **0005 (14)**, 0008 (9), and No. 5 and No. 6 (the W11 vouchers).
- Accepted (ISGSTOVERRIDDEN Yes): 0006 (48) and 0016 (28; already accepted before W12).
- The other 20 are included.

**GSTR-1, 1-Aug-26 to 31-Aug-26:**
- Total 35: included 23 (Ready for Upload 23, Not Uploaded 23), not relevant 6, uncertain 6.
- B2C (Small) 23 (2): 9,505.70 taxable, CGST 283.18, SGST 283.18, tax 566.36.
- B2B, B2C (Large) and every other section are blank.
- **0005's reason:** "Mismatch between Expected Tax Amount and Modified Tax Amount in transaction". Taxable 1,001.00; expected tax 50.06; modified tax 50.04; difference 0.02 (CGST 25.03/25.02; SGST 25.03/25.02).

## Step 3: export a (before the change)
1. In GSTR-1, Alt+E shows Export: Current (Ctrl+E), Others, GST Returns, Masters, Transactions, Configuration.
   - I tried **Current** first. It is the report export: Show breakup of Tax Amount Yes; Show Invoice Amount Yes; Show report details based on "Included in Return"; File Format "XML (Data Interchange)"; C:\Program Files\TallyPrime\GSTR-1.xml.
   - It is not the JSON for upload, so I **cancelled it with Esc**; nothing was exported.
2. **Export > GST Returns** opened "Upload GSTR-1" for Aug.
   - Pending to Upload (Voucher Count 0; Summary Count 4): Rajasthan 5% 8,805.70; Rajasthan 18% 700.00; HSN Summary; Document Summary.
   - Buttons: X: Export (Offline) and S: Send (Online), which was not used.
3. X: Export (Offline) showed this **observed dialog (not listed in the README; a STOP, resolved by the orchestrator)**:
   > "There are transactions having potential conflicts between the master and the transaction.You can check conflicts in GSTR-1 report. Still, GSTR-1 data will be exported to the portal as per the transaction." [X: Export (Offline)] [B: Back]

   Screenshot s3-STOP-conflicts-warning.jpg, sha256 28a386adb50046089e124fe0105aebcf94db18b336f0621983bd4c0f7d587359.
   - First I clicked Back.
   - On the orchestrator's decision I then clicked X: Export (Offline). The same warning appeared before export b and was answered the same way.
4. **Export dialog:**
   - Defaults: File Format "Excel (Spreadsheet)" and Folder Path "C:\Program Files\TallyPrime". Right panel: F6 Folder Path, F8 File Format, F9 "Export Section wise files".
   - The F8 list offers CSV (Comma Delimited), Excel (Spreadsheet) and JSON (Data Interchange). I chose **JSON (Data Interchange)**.
   - The dialog then also showed **"Export separate files for each Section: No"**. That option is not in the README, so I stopped (screenshot s3a-STOP-export-dialog-json.jpg, cdd17e35…).
   - On the orchestrator's decision I left it at its default (No) and kept the default folder. I did not touch C: Configure, then clicked E: Export.
   - **The dialog has no option to include or leave out uncertain transactions.**
5. **Export a written:** `C:\Program Files\TallyPrime\<gstin>_GSTR1_AUG-2026_12102026_180154.json`, 2,199 B, at 12:31:55Z.
   - sha256 6f5d5c6fceb4bd354847e937daaa24381b64286b606cb72b0450780bc5d16607, identical before and after the byte copy to `gstr1-aug-2026-a.json`.
   - Tally then opened a File Explorer window on that folder, which I left alone.

**Finding: export a wrote to the book.**
- After the export, the Upload GSTR-1 screen read Pending (Voucher 0; **Summary 0**), down from 4.
- The GSTR-1 summary changed from "Ready for Upload 23 / Not Uploaded 23" to "Included 23 / **No Action Required** 23". Totals were unchanged.
- An extra marks read showed **ALTVCHID 50 → 53**, with ALTMSTID 243 unchanged.
- An extra window read showed **no field change** in the 28 August Sales vouchers.
- Three voucher alter ids were therefore used by objects outside that window, which were not identified. The orchestrator assigned identifying them to a later item.

## Steps 4–5: Accept As Is on 0005
- Path: GSTR-1, Uncertain, then the tax-mismatch list. I highlighted the single row BP/26-27/0005 and clicked "J: Accept As Is".
- Dialog: "This exception will be accepted and included in Returns. Do you want to accept as is?" I clicked Yes. No other dialog appeared, and 0005 left the list.
- marks-1: ALTVCHID 53 → **54**.
- status-0005, -0 vs -1: **4 of 1,041 fields differ**: ALTERID 14 → 54, ISGSTOVERRIDDEN Yes, INCLUDED Yes, UNCERTAIN No. The status read has no CRC field. This is the same pattern as W12.
- Window -1 against the after-export-a window: **only 0005 differs**, in those four fields.

## Step 6: export b
- Marks before b: identical to marks-1 (ALTVCHID 54).
- GSTR-1 before b:
  - Included 24, with "Ready for Upload 24" and "**Modified in Books After Upload/Export 24**"; uncertain 5.
  - B2C (Small) 24 (2): 10,506.70, CGST 308.20, SGST 308.20, tax 616.40.
- Upload GSTR-1 before F8 showed Pending (Voucher 0; **Summary 3**): Rajasthan 5% 9,806.70; HSN Summary; Document Summary. The 18% row was not pending.
- **DEVIATION (orchestrator's decision):** I pressed **F8 "Include Uploaded"**, because it is the only way to export everything after a.
  - Pending became Summary 4: 5% 9,806.70; 18% 700.00; HSN; Document. The F8 label changed to "Exclude Uploaded".
- Then the same conflicts warning, answered with X: Export (Offline), and the same dialog with the remembered settings: JSON, separate files No, default folder. I clicked E: Export.
- **Export b written:** `C:\Program Files\TallyPrime\<gstin>_GSTR1_AUG-2026_12102026_180637.json`, 2,199 B, at 12:36:37Z.
  - sha256 7251bae3a9933261ecde98a9c31ad5779a941434033f8c259805b8710bde1f71, identical after the copy to `gstr1-aug-2026-b.json`.
- After b: marks showed **ALTVCHID 54 → 57**, again +3, with ALTMSTID 243. The window read showed **no field change**.
- GSTR-1 after b: included 24 ("No Action Required 24"), uncertain 5, the same totals.

**Export a against export b, compared as parsed JSON (139 leaves each).** Only these differ:
- `b2cs[0]` (5%, intra-state, place of supply 08): txval 8805.7 → **9806.7** (+1,001.00); camt 220.18 → **245.2** (+25.02); samt 220.18 → **245.2** (+25.02).
- `hsn.hsn_b2c[2]` (998313): txval 6604 → 7605; camt and samt 165.13 → 190.15 (+25.02 each).
- `doc_issue.doc_det[0].docs[0]`: net_issue and totnum 22 → 23.
- The 18% row (700 / 63 / 63) and everything else are equal.

So **the JSON leaves the uncertain 0005 out entirely, including from the document count.** Once 0005 is accepted, the JSON takes it in at its **modified** tax (25.02 + 25.02), not the expected 25.03 + 25.03.
- Export a's totals equal the screen: 8805.7 + 700 = 9,505.70, and 220.18 + 63 = 283.18.
- Export b's totals equal the screen too: 10,506.70 and 308.20.

## Step 7
**Export c: NOT APPLICABLE (not run).** The export dialog has no option about uncertain transactions.

## Published copies
- The JSON exports and the full answers carry the synthetic company's GSTIN, and the export file names begin with it.
- The copies on this branch replace it with the same-length marker `<GSTIN-SYNTH15>`, so byte counts are unchanged. No other byte differs.
- `SHA256-answers.txt` and `SHA256-exports.txt` list the lab file's hash and the published copy's hash.

## Differences from the README, and what was not measured
- The book started from W12's end state, as instructed. There were extra baseline reads of 0003 and 0006, and extra marks and window reads around each export.
- The XML report export was opened and cancelled.
- Both exports showed the unlisted conflicts warning and the unlisted "separate files" option. Both were answered as the orchestrator decided.
- Export b used F8 "Include Uploaded" (a deviation). Export c was not applicable.
- **Each JSON export advanced the company voucher alter id by 3** and changed the upload state. The objects altered were not identified.
- Screenshots and the backup stay on the lab PC, listed by hash.
