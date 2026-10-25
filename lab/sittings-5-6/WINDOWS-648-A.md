# Windows item 648-A v0.1 DRAFT (owner keying, then trial-balance reads): a small fixed-asset book whose additions, disposals and depreciation can be dated from year, month and day trial balances (25 Oct 2026; for issue #648, phase 1 `asset_movements`). SYNTHETIC ONLY: THE COMPANY = BRIDGE ASSET LAB, created fresh by the owner in step 1 (books from 1 Apr 2026, India, GST No, inventory No). The reads go through ComplyEaze Bridge's `trial_balance` and `ledger_masters` tools, so the book is known only from Tally's own screens and the tools' answers. LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds. Needs the run coordinator's go-ahead.

Why: #648's design dates a ledger's movements without a bulk voucher export: one year trial balance says which asset ledgers moved, monthly trial balances date each to a month, a bisection dates it to a day. Nothing has been captured on a book with known answers. This item builds a book with known answers, so the capture can be checked against the keyed truth.

## The build and its settings
- TallyPrime lab release, licensed, port 9001. The `bridge_mcp` program of master at the head the go-ahead names, from that head's own CI run; the go-ahead names its sha256. Posting and ledger creation OFF. Server name `bridge-lab-648`, one call at a time, every answer kept whole as a file with its size and sha256; a call in flight is never cancelled, abandoned or repeated.

## Differences from LAB-COMMON
- C1: THE COMPANY does not exist before step 1; the owner creates it there. From step 2 on, C1 applies as written.
- OWNER STEPS: every step marked OWNER is done by the owner in person, ONE act at a time (one master or one voucher), with its screenshot; the operator never keys.
- C3: the operator's own request is MARKS only; every other read is a tool call. A tool answer above 1,000,000 bytes = STOP.
- C5: backup A0 after step 3 (masters and opening only), A1 after step 4 (all vouchers). C6: no restore.

## Request (in requests/; bytes and sha256 in SHA256.txt)
- MARKS: `asset-marks.utf16le.xml`, 976 bytes, sha256 7da3dd85dc81dde0fd53acc11459f12d6383a281f4b57a5c53d92e75dd26412e.

## The book (the truth the reads are checked against)
Ledgers (OWNER keys each; group in brackets): `FA Land` (Fixed Assets), `FA Building` (Fixed Assets), `FA Plant A` (Fixed Assets), `FA Plant B` (Fixed Assets), `FA Vehicle` (Fixed Assets), `FA Computer 1` (Fixed Assets), `FA Computer 2` (Fixed Assets), `FA Clearing` (Fixed Assets; a clearing ledger that must be told apart), `Accum Depreciation` (Fixed Assets), `Depreciation Exp` (Indirect Expenses), `Asset Supplier` (Sundry Creditors), `Asset Bank` (Bank Accounts), `Asset Buyer` (Sundry Debtors). Opening balances: none.
Vouchers (OWNER keys each; Dr/Cr as given; no narration):
| # | date | type | Dr | Cr | amount |
|---|---|---|---|---|---|
| 1 | 15 Apr 2026 | Journal | FA Plant A | Asset Supplier | 100,000.00 |
| 2 | 20 May 2026 | Journal | FA Plant A | Asset Supplier | 50,000.00 (a second addition in parts) |
| 3 | 10 Jun 2026 | Payment | Asset Supplier | Asset Bank | 150,000.00 |
| 4 | 1 Jul 2026 | Journal | FA Computer 1 | Asset Supplier | 60,000.00 |
| 5 | 5 Jul 2026 | Journal | Asset Supplier | FA Computer 1 | 60,000.00 (reversal of 4: a purchase returned) |
| 6 | 1 Aug 2026 | Journal | FA Vehicle | Asset Supplier | 800,000.00 |
| 7 | 1 Sep 2026 | Journal | FA Clearing | Asset Supplier | 25,000.00 |
| 8 | 15 Sep 2026 | Journal | FA Building | FA Clearing | 25,000.00 (transfer from clearing to an asset) |
| 9 | 10 Oct 2026 | Receipt | Asset Bank | FA Vehicle | 500,000.00 (disposal for sale value) |
| 10 | 31 Oct 2026 | Journal | Depreciation Exp | Accum Depreciation | 12,000.00 |
| 11 | 1 Nov 2026 | Journal | FA Land | Asset Supplier | 2,000,000.00 |
| 12 | 20 Nov 2026 | Journal | FA Computer 2 | Asset Supplier | 45,000.00 |
Expected by day: additions on 15 Apr, 20 May, 1 Jul (then reversed 5 Jul), 1 Aug, 1 Sep (clearing), 15 Sep (transfer), 1 Nov, 20 Nov; a disposal on 10 Oct; depreciation on 31 Oct.

## Steps (LAB-COMMON start, then steps)
- 1. OWNER: create BRIDGE ASSET LAB; screenshot of the company screen with its number; the ledgers above. RECORD the company number. MARKS: RECORD the GUID and both marks.
- 2. Step 0 (C2) here and before every later step that sends.
- 3. BACKUP A0 (C5) to the NEW folder `C:\bridge-lab\648-A\backups\A0\`. MARKS.
- 4. OWNER: key vouchers 1 to 12, one at a time; after each, a screenshot of the voucher before accepting. Then BACKUP A1 (C5) to the NEW folder `C:\bridge-lab\648-A\backups\A1\`. MARKS: RECORD both marks.
- 5. Tool call `ledger_masters` with fields basic, group `Fixed Assets`, group scope ancestry. EXPECT the 9 ledgers under Fixed Assets (8 asset and the accumulated depreciation) and none of the others; RECORD the answer whole and each ledger's reported ancestry.
- 6. Tool call `trial_balance` for the whole year 2026-04-01 to 2027-03-31. EXPECT the ledgers that moved are those the table moves, and that its closing balances equal the sum of the keyed amounts (FA Plant A 150,000.00; FA Computer 1 0; FA Vehicle 300,000.00; and so on): COMPARE by program, list each difference.
- 7. Tool calls `trial_balance` for each month April to November 2026 (8 calls, one at a time). For each ledger, RECORD the month in which its balance moved; EXPECT the months the table gives.
- 8. For every ledger whose month read shows a move, bisect that month by `trial_balance` over half-month windows until the day is found. RECORD the number of calls per ledger. Also RECORD the cases a month read cannot see: FA Computer 1 is added on 1 July and returned on 5 July, so its July read nets to zero (EXPECT no move at month level); say whether the year read or any month read shows it at all.
- 9. Close (C10).
- THE RESULT is a table: ledger; the keyed events (date, direction, amount); what the year, month and day reads recovered; the number of reads; and the list of differences between keyed truth and recovered truth. A difference is a result, not a STOP.

## Stop rules (beyond LAB-COMMON)
A `trial_balance` call that does not answer within 10 minutes (never repeated; leave it running, report); a Tally dialog (C7); a voucher keyed differently from the table (a screenshot shows it before accepting): STOP, do not accept it.
WHAT THE BOOK HOLDS IF THE ITEM STOPS: after step 3, masters only; after step 4, masters and the vouchers keyed so far (the screenshots list them). The way back is A0 or A1, on the run coordinator's word.
Not covered: the income-tax block depreciation chart, a book of thousands of ledgers (the issue's real book had about 9,500 and a 24 MB trial balance page), assets capitalised in parts over several years.
End state: BRIDGE ASSET LAB shut and kept; Tally on 9001 with no company loaded.

Evidence branch: `lab/648-a-answers`. Report after step 4 and at the end, or at a STOP.
