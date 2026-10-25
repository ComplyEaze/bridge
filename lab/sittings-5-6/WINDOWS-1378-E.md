# Windows item 1378-E v0.1 DRAFT (reads and owner keying only): where Tally keeps an invoice's IRN, acknowledgement number and acknowledgement date, and whether they can be keyed without the portal (25 Oct 2026; for issue #1378, "Measure live first"). SYNTHETIC ONLY: THE COMPANY = BRIDGE EINV LAB, created fresh by the owner in step 1 with e-invoicing switched on in its GST details. Plain HTTP reads plus the owner's keying on Tally's screen: NO ComplyEaze Bridge build, NO import. LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds. Needs the run coordinator's go-ahead. Nothing is sent to any portal; the owner never uses a portal login or an e-invoice generate/cancel/upload action in Tally (see the stop rules).

Why: #1378 may read and preserve e-invoice details only once a capture shows which elements hold them, whether Tally lets a person key them by hand in Tally without a portal connection, and whether an import-created voucher is treated as already e-invoiced. The only IRN element in the committed fixtures is empty. A synthetic company cannot hold an IRN the portal issued, so this item answers only the parts that need no portal connection.

## Differences from LAB-COMMON
- C1: THE COMPANY does not exist before step 1; the owner creates it there. From step 2 on, C1 applies as written.
- OWNER STEPS: every step marked OWNER is done by the owner in person, ONE act at a time, with the screenshot and the reads named.
- C3: size bounds MARKS 8,192 bytes; VCH-EINV 2,000,000 bytes. C5: backup E0 after step 2. C6: no restore.

## Requests (in requests/; bytes and sha256 in SHA256.txt)
- MARKS: `einv-marks.utf16le.xml`, 974 bytes, sha256 d1e1ff66dc7f9218718ff7c70c42e042b1fd91b8f8887dde32112da7924bf85e.
- VCH-EINV, the whole year 20260401 to 20270331, the usual fields PLUS `IRN,IRNACKNO,IRNACKDATE`: `einv-vch-fy.utf16le.xml`, 1,794 bytes, sha256 1fdbcf6cbd8453f24c1155fa2db9701e55a4e8a18b23e82afee8942a35dc893b. THE EXTRA FIELD NAMES ARE A NEW SHAPE: only `IRNACKNO` appears in committed fixtures, and a name Tally does not know comes back empty, so an empty IRN, IRNACKNO or IRNACKDATE element proves nothing by itself; the screenshot of the voucher's e-invoice screen (step 4) is the control. After the first send of this file, screenshot the whole Tally window (C3) before the next request.

## Steps (LAB-COMMON start, then step 0 before every step that sends)
- 1. OWNER: create BRIDGE EINV LAB (books from 1 Apr 2026, India); in its F11/GST details: GST registration type Regular with a synthetic GSTIN of the lab's synthetic family; e-invoicing applicable Yes from 1 Apr 2026 (screenshot of the settings screen before accepting). RECORD the company number. MARKS: RECORD the GUID.
- 2. OWNER: ledgers `Einv Sales` (Sales Accounts, GST on supply nature Taxable at 18%), `Einv Party` (Sundry Debtors, a synthetic GSTIN, state Maharashtra, registration Regular) and the tax ledgers Tally asks for; the company's own state Maharashtra. BACKUP E0 (C5) to the NEW folder `C:\bridge-lab\1378-E\backups\E0\`; MARKS.
- 3. OWNER: key two Sales invoices, 1 April 2026, number `E1`, 1,000.00 plus 18% tax to `Einv Party`, and 2 April 2026, number `E2`, 2,000.00 plus 18% tax, the same shape. Neither has e-invoice details. After each: screenshot the voucher. MARKS and VCH-EINV. RECORD both tables.
- 4. OWNER: open invoice E1 and look for where Tally allows the IRN, acknowledgement number and acknowledgement date to be entered (the e-invoice details area of the voucher, or the e-invoice report). Screenshot every screen on the way. TRY to enter a synthetic IRN of 64 characters from the lab's synthetic family, an acknowledgement number and a date; if Tally refuses, offers only to generate or upload, or requires a portal connection: press nothing that generates, upload or cancels, screenshot the message and its exact text, and go to step 7 with the result "not keyable by hand". If the fields accept values: save, then MARKS and VCH-EINV (after screenshot of the whole Tally window per C3).
- 5. (Only if step 4 saved details.) DIFF the VCH-EINV tables E1 against E2 element by element, by program: which elements differ besides the voucher number, date and amounts; RECORD the element names that carry the IRN, the acknowledgement number and the date, and whether the three requested names returned them.
- 6. (Only if step 4 saved details.) OWNER: open Tally's e-invoice report for the period and screenshot how E1 and E2 are listed (E1 should read as e-invoiced, E2 as pending). It must not offer to generate for E1.
- 7. Close (C10).
- THE RESULT answers, with the tables and screenshots kept: (a) whether Tally allows the three details to be keyed without the portal; (b) the element names that hold them, or the statement that the requested names returned nothing and the element names are unknown; (c) what Tally's e-invoice report shows for E1 against E2.

## Stop rules (beyond LAB-COMMON)
Any screen that asks for a portal login, a GSP, an OTP or a network connection; any button named Generate, Upload, Cancel IRN or Send: press nothing, screenshot, STOP. A dialog in Tally (C7).
WHAT THE BOOK HOLDS IF THE ITEM STOPS: after step 2, two masters and no voucher; after step 3, plus E1 and E2 without details; after step 4, plus details on E1 if saved. The way back is E0 (before the vouchers), on the run coordinator's word.
Not covered: an IRN issued by the portal, a real book (the issue wants one read-only look at a real e-invoiced voucher with the owner's approval; that is a separate act, nothing from it is committed), an import that sets the elements.
End state: BRIDGE EINV LAB shut and kept; Tally on 9001 with no company loaded.

Evidence branch: `lab/1378-e-answers`. Report after step 4 and at the end, or at a STOP.
