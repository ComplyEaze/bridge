# Windows item 1377-C v0.1 DRAFT (reads and owner keying only): two copies of one company that diverged by one voucher on each side (25 Oct 2026; for issue #1377). SYNTHETIC ONLY: THE COMPANY = BRIDGE COPY LAB, created fresh by the owner in step 1; copy A lives in Tally's normal data folder, copy B is the same company restored from A's first backup into a second data folder `C:\bridge-lab\1377-C\dataB\`. Only ONE copy is ever loaded at a time. Plain HTTP reads plus the owner's own keying and Tally's own Backup and Restore screens: NO ComplyEaze Bridge build, NO import. LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds. Needs the run coordinator's go-ahead.

Why: #1377 asks for a read-only check that says whether two copies are the same book. Its own "to measure" list starts with: after both copies diverge by one voucher each, does the same GUID appear with different content, and what do the marks of the two copies show? This item measures exactly that, from reads, before any check is designed. Whether an export from one copy and an import into the other replaces or adds a voucher is NOT in this draft (it needs an import file, a new shape with a REMOTEID).

## Differences from LAB-COMMON
- C1: THE COMPANY does not exist before step 1. "Whenever anything is sent, THE COMPANY is loaded and no other": copy B counts as THE COMPANY once it is loaded alone; copy A is shut first. The two copies share one GUID; the evidence names them A and B by the data folder shown on the Select Company screen.
- C6 is used twice (a restore into a NEW data folder, step 3; no restore into A). The Replace prompt will NOT appear for B (the folder is new): its absence is expected for B and is not a STOP; any other prompt is.
- C3: size bounds MARKS 8,192 bytes; VCH 2,000,000 bytes.
- OWNER STEPS: every step marked OWNER is done by the owner in person, ONE act at a time, with the screenshot and the reads named.

## Requests (in requests/; bytes and sha256 in SHA256.txt)
- MARKS: `copy-marks.utf16le.xml`, 974 bytes, sha256 15e56fafaa62fc4c534ada50dadec17cf546d6fda684c32063c5b31429d64374.
- VCH-FY, the whole year 20260401 to 20270331: `copy-vch-fy.utf16le.xml`, 1,746 bytes, sha256 14fe54e15e8ac7738792106bdb260f1166483624cd590f19ae5259c063780c0c.
Both name the company `BRIDGE COPY LAB`, so they serve both copies.

## Steps (LAB-COMMON start, then step 0 before every step that sends)
- 1. OWNER: create BRIDGE COPY LAB (books from 1 Apr 2026, India, GST No, inventory No); ledgers `Copy Bank` (Bank Accounts) and `Copy Party` (Sundry Debtors); three Receipts, 1, 2 and 3 April 2026, `Copy Bank` Dr / `Copy Party` Cr, 100.00, 200.00, 300.00. RECORD the company number, MARKS (GUID, both marks) and VCH-FY (the table: MASTERID, ALTERID, GUID, number, amount).
- 2. BACKUP C0 (C5) to the NEW folder `C:\bridge-lab\1377-C\backups\C0\`; MARKS.
- 3. Shut copy A. Restore C0 into the NEW data folder (C6, with Restore Data Path = `C:\bridge-lab\1377-C\dataB\`; the prompt list will show one company; screenshot before accepting). Load copy B ALONE from that folder (Select Company with the path changed; record the path the screen shows). MARKS and VCH-FY: EXPECT the same GUID as A and the same table row for row, else STOP (the restore did not produce a copy). Shut B.
- 4. Load copy A alone. OWNER: alter the 2 April Receipt's amount to 250.00; key a new Receipt on 4 April, `Copy Bank` Dr / `Copy Party` Cr, 400.00. MARKS and VCH-FY (A1). Shut A.
- 5. Load copy B alone. OWNER: alter the SAME 2 April Receipt's narration to `B side` (amount left at 200.00); key a new Receipt on 4 April, `Copy Bank` Dr / `Copy Party` Cr, 500.00. MARKS and VCH-FY (B1). Shut B.
- 6. Close (C10).
- THE RESULT is computed by program from A1 and B1 (never by eye) and answers, with the tables kept: (a) the GUID of the 2 April Receipt in A1 and in B1: equal or not, and the content of each (amount, narration); (b) the two 4 April Receipts: their GUIDs and MASTERIDs: equal or not (a restored copy continues numbering from the same point, so equal identity for different vouchers is possible); (c) ALTVCHID and ALTMSTID of A1 and B1 beside each other: equal or not; are two different books reading with equal marks; (d) any voucher present in one table only.

## Stop rules (beyond LAB-COMMON)
Copy B's table differing from A's at step 3; more than one company loaded at a send; a screen act Tally answers with a dialog (C7).
WHAT THE BOOK HOLDS IF THE ITEM STOPS: after step 2, copy A as keyed; after step 3, A as keyed and B a restored copy; after steps 4 and 5, each copy plus the edits done so far (listed). Nothing is deleted by the item; the folders stay on the lab PC.
End state: both copies shut and kept on the lab PC for the import step the run coordinator may add; Tally on 9001 with no company loaded.

Evidence branch: `lab/1377-c-answers`. Report after step 3 and at the end, or at a STOP.
