# Windows item 1262-K6 v0.1 DRAFT (reads and owner keying only): can a voucher carry a ledger's STORED name while the collection attribute differs? (25 Oct 2026; continues the K0 to K5 capture of 12 October on branch lab/1262-capture-answers, which did not produce that state.) SYNTHETIC ONLY: the two existing companies BRIDGE SPELL LAB (copy "A", number 100052, ledger `Bridge Spell Party`, one Receipt on 1 Apr 2026) and BRIDGE SPELL TWIN (copy "B", number 100053, ledger keyed as `BRIDGE SPELL PARTY`, which Tally shows as `Bridge Spell Party`), exactly as the 12 October README left them. Plain HTTP reads plus the owner's own keying: NO ComplyEaze Bridge build, NO import. LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds. Needs the run coordinator's go-ahead.

Why: K3 (TWIN loaded first, then SPELL LAB) made the ledger's attribute `BRIDGE SPELL PARTY` while the stored name stayed `Bridge Spell Party`, and the voucher's LEDGERNAME followed the attribute, so the failing state of #1262 (attribute differs and the voucher carries the stored name) never appeared. That voucher was keyed BEFORE the load order changed. This item keys a NEW voucher while the K3 load order holds, to see which spelling Tally writes into it.

## Differences from LAB-COMMON
- C1: both companies already exist; the sitting loads them in the orders below, and the C1 rule "no other company loaded" is relaxed ONLY for the two named companies in K3 order (TWIN first, SPELL LAB second), as on 12 October. Any third company that opens by itself at the start of Tally (the lab's demo company did on 12 October) is shut first and the report says so. Nothing is sent while a third company is loaded.
- Restarts: each "restart" is Tally's own Quit and a fresh start by the operator; nothing is ended by process.
- C3: size bounds MARKS 8,192 bytes; the four reads below 2,000,000 bytes.
- C5: no new backup (the 12 October backups TDBK1800_100052.001 and TDBK1800_100053.001 stay on the lab PC; their hashes are in lab/1262-capture-answers/notes.txt); a directory listing of the data folders of 100052 and 100053 is kept before and after. C6: no restore.

## Requests
- MARKS: `spell-marks.utf16le.xml` (company BRIDGE SPELL LAB), 976 bytes, sha256 eaf2c84abba844fc60bc91dc63c0514e5bd870a1aa559ef3275cdea68586daff; and `spell-marks-twin.utf16le.xml` (BRIDGE SPELL TWIN), 978 bytes, sha256 e5bb41d4e797b479bb21abcf580ce2908b10ed517a11938c569af3167e979c82.
- VCH, 1 to 30 April 2026, BRIDGE SPELL LAB: `spell-vch-0401.utf16le.xml`, 1,748 bytes, sha256 0988615abfa90bf54d925c5f7ab3a442faa003a4a8b4a4546bad9771542e6b65.
- The four reads of the 12 October capture (q1 List of Ledgers v1, q2 vouchers of a day, q3 and q4 movement) are the fork's files `lab/1262-capture-requests/` (check-sums begin 84e00770, d49ec338, f09848fb and 5f457502). THEIR FULL HASHES AND FILES ARE NOT ON THIS REPOSITORY: the run coordinator supplies them with the folder's SHA256.txt before the sitting; an item sent without that file, or with a hash that differs from the list, is not started. q1 and q4 give the attribute and the stored name; q2 and q3 give the voucher's names.

## Steps (LAB-COMMON start; step 0 before every group of sends)
- K6a. Quit Tally; start it; shut any company that opened by itself; load BRIDGE SPELL TWIN FIRST, then BRIDGE SPELL LAB. MARKS (both requests): two COMPANY rows, both GUIDs recorded. q1 to q4: EXPECT the K3 result (attribute `BRIDGE SPELL PARTY`, stored `Bridge Spell Party`, the 1 April voucher's names in capitals); a different result = RECORD and continue.
- K6b. OWNER, with that same session still running (B first, A second, A the current company): key ONE new Receipt in BRIDGE SPELL LAB, 2 April 2026, `Cash` Dr / the party ledger Cr, 50.00, choosing the ledger from the list on the voucher screen WITHOUT typing its name (screenshot the list and the choice before accepting). Screenshot the voucher after saving. q1 to q4 and the VCH read of April: RECORD the spelling in LEDGERNAME and PARTYLEDGERNAME of the 2 April voucher beside the 1 April voucher's.
- K6c. OWNER, same session: key a second Receipt on 3 April, 60.00, this time TYPING the stored spelling `Bridge Spell Party` in the ledger field (screenshot what the field shows after Tally completes it, before accepting). q1 to q4 and VCH: RECORD the spellings.
- K6d. Quit Tally; start it; shut any company that opened by itself; load BRIDGE SPELL LAB ALONE. q1 to q4 and VCH: RECORD the attribute, the stored name and the spellings carried by the 1, 2 and 3 April vouchers.
- K6e. Quit Tally; start it; shut any company that opened by itself; load BRIDGE SPELL TWIN FIRST, then BRIDGE SPELL LAB, as K6a. q1 to q4 and VCH: RECORD again (the same vouchers, read in the K3 order).
- Close (C10).
- THE RESULT is the table of the 12 October RESULTS.md extended with K6a to K6e, by structure (decoded and parsed, never a text search), and one sentence: whether at ANY checkpoint the attribute differed from the stored name while a voucher carried the stored name. If yes, those answers are the capture #1511's failing-first test was waiting for.

## Stop rules (beyond LAB-COMMON)
Port 9001 not answering after a start (a STOP, not a retry); a request without an answer in 10 minutes (never repeated); a voucher keyed with an amount or date other than the step gives; a Tally dialog (C7); a third company loaded at a send.
WHAT THE BOOK HOLDS IF THE ITEM STOPS: after K6a, as the 12 October README left it; after K6b to K6e, plus the 2 April and (after K6c) 3 April Receipts in BRIDGE SPELL LAB. There is no way back except the 12 October backups, on the run coordinator's word.
Not covered: Tally's rule for the attribute; real books; an import naming either spelling.
End state: both companies shut; Tally on 9001 with no company loaded.

Evidence branch: `lab/1262-k6-answers`. Report after K6b and at the end, or at a STOP.
