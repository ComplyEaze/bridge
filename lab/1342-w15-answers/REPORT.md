# W15: which objects a GSTR-1 JSON export alters (Windows lab, 12 Oct 2026)

Book: synthetic BRIDGE PILOT LAB itself, not a copy. Company number 100033. It was loaded alone, in the state W13 and W14 left it in. TallyPrime 7.1 Silver.
- Request set: lab/1342-capture-requests @ 0f3dafdd.
  - W15-README.md sha256 8b8d5fcf….
  - make-w15-request.py f30857f4…, check-w15-answer.py 5f48973b…, check-request-ids.py 173bc41a…: each equals workorder/SHA256.txt, by sha256sum and Get-FileHash.
  - r02-marks.xml and w13-status-0005.xml were checked against their own sets.
- Generated requests were sent only after their printed hash was written to the notes and added to `GENERATED.txt`.
- Mouse-click mode. S: Send (Online) was never used. No voucher or master was changed by hand.
- Fresh backup before the run: `<lab>\w15\backup-pre\TDBK1800_100033.001`, 85,433 B, sha256 44620989ff75f71a254d4cda913d0cafe67c8911fa981737f7370374d819f257, 13:22Z. It stays on the lab PC.
- Sends: 8, one at a time, each with curl exit 0. Every answer is a complete ENVELOPE with a COLLECTION element and no LINEERROR. `send-log.txt` holds every line.

## Step 0: positive control (passed)
- `w13-status-0005.xml` → `aw15-status-0005-c.xml` (78,416 B): BP/26-27/0005 has ALTERID **54** (= A).
- `python -I make-w15-request.py 53 54 20260401 20270331 w15-control.xml` printed:
  `786898a72a11f3330fad6c1c6af83d07825584c3314d9259d9e144189d207dba  1538 bytes  w15-control.xml`
- Sent → `aw15-control.xml` (5,750 B). `check-w15-answer.py aw15-control.xml 53 54`, exit 0:
  ```
    ALTERID 54 | type BRIDGE Sales | no BP/26-27/0005 | date 20260801 | master 5 | narration ''
  IDENTIFIED: 1 vouchers, ALTERIDs 54..54
  ```

## Steps 1–6
1. `aw15-marks-0.xml`: ALTVCHID **57** (BEFORE); ALTMSTID 243.
2. **Export.** GSTR-1 (Aug-26), then Alt+E > GST Returns > Upload GSTR-1.
   - The screen opened with "F8: Exclude Uploaded", meaning **Include Uploaded was still ON**, carried over from W13's export b. Pending: Voucher 0; Summary 4 (Rajasthan 5% 9,806.70; Rajasthan 18% 700.00; HSN Summary; Document Summary).
   - With F8 pressed (Include Uploaded OFF), pending read **Voucher 0; Summary 0**, so nothing was pending and W13 export a's state could not be reproduced. I stopped and reported.
   - **DEVIATION (decision record):** I pressed F8 again to put Include Uploaded back ON (pending Summary 4 again) and exported exactly as W13's export b did:
     - X: Export (Offline);
     - the same conflicts warning, answered X: Export (Offline);
     - JSON (Data Interchange), separate files No, folder C:\Program Files\TallyPrime;
     - C: Configure not touched; then E: Export.
   - The file: `C:\Program Files\TallyPrime\<gstin>_GSTR1_AUG-2026_12102026_185506.json`, 2,199 B, 13:25:07Z, sha256 7251bae3a9933261ecde98a9c31ad5779a941434033f8c259805b8710bde1f71. It was copied unchanged as `gstr1-aug-2026-w15.json`.
   - It is **byte-identical to W13 export b** (0 of 139 parsed leaves differ).
   - Pending after the export: Voucher 0; Summary 4, with Include Uploaded on.
3. `aw15-marks-1.xml`: ALTVCHID **60** (AFTER = BEFORE + 3, as expected); ALTMSTID 243.
4. `make-w15-request.py 57 60 20260401 20270331 w15-a.xml` printed:
   `3b1bea1650d9fd2324e6bf696bb97960e6a19b553acb05a800fbd57d6b649347  1538 bytes  w15-a.xml`
   - The answer `aw15-altered-a.xml` (3,022 B) is a complete envelope with an empty COLLECTION. Checker, exit 1:
   ```
   NOT IDENTIFIED: found 0 voucher(s) with ALTERIDs []; the counter moved 57..60 (3 step(s))
   ```
5. `make-w15-request.py 57 60 20250401 20260331 w15-b.xml` printed:
   `5a0dc904a943600bfa5b2f615287eaf7506193f033e514cbae792beeef099e3d  1538 bytes  w15-b.xml`
   - The answer `aw15-altered-b.xml` (3,022 B) is byte-identical to -a, with an empty COLLECTION. Checker, exit 1:
   ```
   NOT IDENTIFIED: found 0 voucher(s) with ALTERIDs []; the counter moved 57..60 (3 step(s))
   ```
6. `aw15-marks-2.xml` is byte-identical to marks-1, so the reads did not move the mark.

Also: `check-request-ids.py` on each generated request printed `census: ok 0, skipped 0, FAIL 0`, exit 0.

## Step 7
The optional second export was **not run**. Both windows held 0 rows, so there were no object numbers to compare, and a second write was avoided before a scheduled hold.

## Step 8: screens
- GSTR-1 before and after the export is identical: total 35, included 24 (No Action Required 24), not relevant 6, uncertain 5.
- B2C (Small) 24 (2): 10,506.70, CGST 308.20, SGST 308.20, tax 616.40.

## Result
- The export again moved ALTVCHID by exactly 3 (57 → 60) and ALTMSTID not at all.
- The AlterID-span read returned **0 vouchers** for alter ids 58–60 in both the FY 2026-27 and the FY 2025-26 windows.
- That same request shape correctly returned BP/26-27/0005 for alter id 54 in step 0.
- The three objects therefore are not Voucher-collection objects dated in either window. Verified for these two windows.
- [inferred] They are another object class, or vouchers with no date in either window.

## Differences from the README
- The export used Include Uploaded ON, W13 b's state, because with it OFF nothing was pending.
- The scripts were run with `python -I` (Python 3.10.11). `python3` on this PC is only the Microsoft Store stub.
- Step 7 was not run.
- Screenshots and the backup stay on the lab PC, listed by hash.
- The JSON export is published with the synthetic GSTIN replaced by a same-length marker; both hashes are listed.
