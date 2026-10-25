# Windows item 628-C0 v1: one GST Purchase invoice keyed on Tally's screen and read natively, as the reference shape for posting Purchase vouchers. (25 Oct 2026.) SYNTHETIC ONLY: THE COMPANY is `BRIDGE GST LEDGER LAB`, the company GS1-L1 creates (its GUID as GS1-L1 step 1 recorded it). Runs only after GS1-L1 has closed. LAB-COMMON v3 applies in full; where an item and LAB-COMMON differ, the stricter holds. No ComplyEaze Bridge program is used.

Why: issue 628 asks for Purchase vouchers through ComplyEaze Bridge. No Purchase has ever been posted over the gateway in its shape, and the only Purchase bytes held were keyed on a book whose own GSTIN Tally may not accept (both read GSTR uncertain). This item is the first of a sequence (C0 keyed reference; then C1 the same voucher imported in Bridge's shape, C2 a duplicate supplier reference, C3 references kept elsewhere, C4 a renamed Purchase type, C6 an out-of-state supplier), each written only after the previous one's answer. C0 answers one question first: on a company whose GSTIN is the gate's synthetic identifier, does a clean intra-state Purchase read as included (not uncertain) in Tally's GST status? If not, no later item can use that status as evidence, and that is recorded before anything else is built.

## Differences from LAB-COMMON
- C3: the operator's requests are the two files below. Size bounds: MARKS 8,192 bytes; the Purchase read 400,000 bytes (the same request shape answered 58,321 bytes for two Purchases on another book).
- C5: one backup, C00, before step 3.
- OWNER STEPS: every master and voucher is keyed by the owner himself, one act at a time, with a screenshot before accepting and one MARKS read after.

## Requests (in requests/; bytes and sha256 in SHA256.txt)
| file | what | the change from its source |
|---|---|---|
| marks.utf16le.xml | MARKS of THE COMPANY | none: GS1-L1's file, byte for byte |
| c0-purchases-dec.xml | the Purchase vouchers of THE COMPANY dated 1 to 31 December 2025, every native field with the ledger-entry lists (UTF-8) | the request that read the keyed Purchases of 12 Oct, with the company and the two dates changed |
Before step 0: `python -I check-request-ids.py requests`, EXPECT `census: ok 2, skipped 0, FAIL 0`.

## Steps
0. LAB-COMMON step 0 with no company loaded; Tally's About screen: EXPECT Release 7.2 (else STOP). EXPECT `BRIDGE GST LEDGER LAB` in Select Company's list (absent = STOP: GS1-L1 has not run).
1. Load THE COMPANY alone. MARKS: EXPECT one COMPANY row with the GUID GS1-L1 recorded. LOOK ONLY: the company's GST registration screen (F11 or the Statutory Details screen as Tally offers it): record the state, registration type and GSTIN as shown (EXPECT `08ZZZZZ0000Z1ZQ`, or none if GS1-L1 was not asked for one); leave with Ctrl+Q after a screenshot. If THE COMPANY has no GSTIN: STOP (C0 needs one; it is recorded as the finding).
2. MARKS -> `ans-c0-marks-1.xml`. Record both marks.
3. Backup C00 (C5) into `C:\bridge-lab\628\backups\c00\`, then MARKS.
4. OWNER: create the ledger `SYN GS1 Supplier B` under Sundry Creditors, Maintain balances bill-by-bill Yes, GST registration type Regular, State Rajasthan, GSTIN `08ZZZZZ0000Z2ZP` (the gate's synthetic family; any other identifier = STOP). If Tally refuses the GSTIN as invalid: screenshot, press no key beyond leaving the field as the screen offers, STOP (the finding: Tally does not accept the synthetic family as a party GSTIN). Then MARKS (EXPECT ALTMSTID up).
5. OWNER: key ONE Purchase in invoice view, Accounting Invoice mode, dated 2-12-2025: Supplier Invoice No. `GS1/P/0001`, Date `1-12-2025`, party `SYN GS1 Supplier B`, ledger `Purchase - Goods` 10,000.00, then `Input CGST` 900.00 and `Input SGST` 900.00 (typed as given), total 11,800.00; on the bill-wise screen a New Ref named `GS1/P/0001`. Let Tally number the voucher. If Tally fills a tax amount by itself: record it; accept only if it is 900.00 and 900.00, else leave the voucher unsaved with Ctrl+Q after a screenshot of its title and STOP. Screenshot before accepting; then MARKS (EXPECT ALTVCHID up).
6. `c0-purchases-dec.xml` -> `ans-c0-purchases-dec.xml` (curl as C3; record status, bytes, CONTENT-LENGTH). EXPECT exactly one VOUCHER row, the voucher of step 5. RECORD (C4: parse, never grep) every one of these as Tally wrote it: VOUCHERTYPENAME, VOUCHERNUMBER, NUMBERINGSTYLE, DATE, REFERENCE, REFERENCEDATE, PARTYLEDGERNAME, PARTYGSTIN, PLACEOFSUPPLY, STATENAME, GSTREGISTRATIONTYPE, CMPGSTIN (or the company GSTIN field present), ISINVOICE, ISELIGIBLEFORITC, every VCHGSTSTATUS* flag, and per ledger entry its LEDGERNAME, AMOUNT, ISDEEMEDPOSITIVE and every BILLALLOCATIONS.LIST child (NAME, BILLTYPE, AMOUNT).
7. MARKS -> `ans-c0-marks-2.xml`; then LAB-COMMON C10. The book keeps the supplier and the Purchase (no restore: GS1-L1's book is kept for this sequence).

## Pass / finding
- PASS: step 6 holds one voucher whose `VCHGSTSTATUSISINCLUDED` reads Yes and `VCHGSTSTATUSISUNCERTAIN` reads No.
- FINDING (first rank): uncertain or not included. Record the flags; open Tally's own GST report for December only on the run coordinator's word (opening a GST report may itself write GST status to vouchers, as GS1-X1 recorded), and only to read the reason Tally shows.

## Evidence
Every answer whole with size and sha256, the MARKS answers, screenshots by name, size and sha256 (never pushed), the notes file. The answer of step 6 becomes the reference bytes from which C1's import file is derived.
