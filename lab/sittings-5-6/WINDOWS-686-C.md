# Windows item 686-C v0.1 DRAFT (partly drafted): what each kind of write moves in the book's marks and voucher identities, for the `changed_since` cursor (25 Oct 2026; for issue #686, part 1 of "what qualified needs"). SYNTHETIC ONLY: THE COMPANY = BRIDGE CURSOR LAB, created fresh by the owner in step 1 (books from 1 Apr 2026, India, GST No, inventory No, other features at Tally's defaults); its company number and GUID are recorded there and every later step is checked against that GUID. Plain HTTP reads plus the owner's own keying on Tally's screen: NO ComplyEaze Bridge build. LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds. Needs the run coordinator's go-ahead.

Why: a change cursor can only be trusted if it returns exactly the vouchers each write touched (#686). The marks and each voucher's ALTERID/MASTERID/GUID are the only things a cursor can see, so the evidence is: for each write, the two marks before and after and the voucher table before and after, compared by program. The census-cost question (whether a span with no changes costs as much as a full one) is NOT part of this draft: it is the #595 census probe on BRIDGE CORPUS DENSE and is run from that issue's own sheet.

## Differences from LAB-COMMON
- C1: THE COMPANY does not exist before step 1; the owner creates it there. From step 2 on, C1 applies as written.
- OWNER STEPS: every step marked OWNER is done by the owner in person, ONE act at a time (one master or one voucher, then its screenshot and the reads the step names, then the next). The operator never keys.
- C3: size bounds MARKS 8,192 bytes; VCH 2,000,000 bytes.
- C5: backup S0 after step 3 (the seed). C6: no restore in the drafted part.

## Requests (in requests/; bytes and sha256 in SHA256.txt)
- MARKS: `cursor-marks.utf16le.xml`, 978 bytes, sha256 4132f581944686c8c2aad735d1e218113e517acc3a36d4854f7ecdcec0020d52.
- VCH-FY, the whole year 20260401 to 20270331, every voucher with its ALTERID, MASTERID, GUID, flags and ledger entries: `cursor-vch-fy.utf16le.xml`, 1,750 bytes, sha256 974690425cf5563d1b3cb14e73d1862097b5b6024b6cdcd1b34602028cad49b5. The book holds a handful of vouchers, so one whole-year read per step is small; the cursor window `(cursor, snapshot]` is then computed by program from two tables, never by a request.

## Steps drafted (LAB-COMMON start, then steps)
- 1. OWNER: create BRIDGE CURSOR LAB; screenshot of the company screen with its number; then the ledgers `Cursor Bank` (Bank Accounts) and `Cursor Party` (Sundry Debtors), no opening balances. Record the company number; MARKS and RECORD the GUID. Step 0 (C2) from here on, before every step below.
- 2. OWNER: key three Receipts on 1, 2 and 3 April 2026, `Cursor Bank` Dr / `Cursor Party` Cr, 100.00, 200.00, 300.00, no narration. After each: MARKS and VCH-FY. RECORD per voucher ALTERID, MASTERID, GUID and the two marks.
- 3. BACKUP S0 (C5) to the NEW folder `C:\bridge-lab\686-C\backups\S0\`. MARKS.
- 4. Screen edit: OWNER alters the 2 April Receipt's amount to 250.00 on the voucher screen. MARKS and VCH-FY; DIFF the two tables with the previous step's: which vouchers' ALTERID moved, by how much; ALTVCHID moved by how much; the company mark.
- 5. Master change: OWNER renames `Cursor Party` to `Cursor Party 2`, then regroups it (Sundry Debtors to Sundry Creditors and back). After each: MARKS and VCH-FY. EXPECT (to test, not assumed): ALTMSTID moves, ALTVCHID does not, and no voucher row changes except its party spelling. RECORD whether any voucher's ALTERID moved.
- 6. Delete: OWNER deletes the 3 April Receipt on the voucher screen. MARKS and VCH-FY. RECORD that the voucher is absent from the table and how far ALTVCHID moved (the reference says a delete moves ALTVCHID by 2).
- 7. Cancel: OWNER cancels the 1 April Receipt on the voucher screen. MARKS and VCH-FY. RECORD whether it still reads (ISCANCELLED flag) and its ALTERID.
- 8. Re-date: OWNER alters the remaining Receipt's date from 2 April to 20 April. MARKS and VCH-FY.
- 9. Close (C10).
- THE RESULT is a table per write: write kind; ALTVCHID and ALTMSTID before and after; the vouchers whose ALTERID/MASTERID/GUID moved; whether the write is visible in `(cursor, snapshot]`; and for the delete, the gap in ALTERID/MASTERID that is the only trace.

## NOT DRAFTED here (the run coordinator supplies these files; do not improvise them)
Writes over the gateway need import requests that are new shapes and carry REMOTEID values, which the merge gate reads as UUIDs without provenance until they are listed: create, alter, cancel, re-date and a no-op re-post over the gateway; a REMOTEID upsert that un-cancels, makes optional or re-creates a deleted voucher (reference sections 9.14 and 11c.5). Each must be added as its own step with its file, size and sha256, and a step that follows the same pattern as step 4 (MARKS and VCH-FY before and after).

## Stop rules (beyond LAB-COMMON)
A table that does not parse; a voucher whose GUID does not begin with THE COMPANY's GUID; a screen act that Tally refuses or answers with a dialog (C7).
WHAT THE BOOK HOLDS IF THE ITEM STOPS: after step 3, the seed; after steps 4 to 8, the seed plus the edits done so far (listed). The way back is S0, on the run coordinator's word.
End state: BRIDGE CURSOR LAB shut and kept for the not-drafted write steps; Tally on 9001 with no company loaded.

Evidence branch: `lab/686-c-answers`. Report after step 2 and at the end, or at a STOP.
