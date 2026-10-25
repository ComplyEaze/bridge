# #1211 rows R1 and R2: `statement_tie_out` after a post, on a window ending mid-month and with a second bank ledger

Not for merge. One item on a company the lab already holds. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full; where this sheet and it differ, the stricter holds.

THE COMPANY is BRIDGE TIE OUT LAB 3: synthetic, made for the #1628 run of 14 October 2026 (`lab/1628-m-month-window-tie-out-answers`). That run left it holding:
- books from 1 April 2026;
- a bank ledger Union Bank, opening 10,000.00 Dr;
- a Suspense ledger, an income ledger, and one seed Receipt of 1 June 2026;
- July's and August's statement vouchers, imported by hand;
- a Receipt of 10 October 2026 keyed by hand.

If THE COMPANY is not on the lab PC: STOP. Never create a substitute.

**WHY.** #1211 shipped `statement_tie_out`, but two of its release rows never ran:
- **R1:** after a post, a window that ends in the middle of a month must give a typed result, never an error and never a figure invented for the part month.
- **R2:** with more in the book than the 14 October run had (here, a second bank ledger), the opening, closing and change must equal Tally's own figures for the same dates, and the gaps must be zero while the bank ledger matches the statement.

A book of 1,000 or more vouchers is not covered here. No synthetic one exists yet; sitting 5 builds one.

**THE STATEMENT** is the synthetic three-month PDF of the 14 October run, kept on the lab PC: 3,927 bytes, sha256 beginning `506e05eb` and ending `4fc9`. Check the full hash against that run's notes on the lab PC before every call. A different file = STOP.

**THE BUILD** is the ComplyEaze Bridge build the run coordinator names, from the lab's desktop entry with its own data folder and posting NOT set. Record its `bridge_mcp.exe` sha256 at the start and at the end.

**OWNER steps** are marked [OWNER].

## Before the sitting
1. [OWNER] Tally is LICENSED. The keyed dates in R2 are not days 1, 2 or 31. Educational mode = STOP (C1).
2. Check every file here against `SHA256.txt`.
3. [OWNER] Go-ahead.

## Step 0 and the book as recorded
C2 step 0 with THE COMPANY loaded alone. Then:
- raw read `requests/r01_marks.request.utf16le.xml` (marks M0);
- Bridge `ledger_movement` for Union Bank, 1 July to 30 September 2026.

EXPECT, as on 14 October: opening -10000.00, closing -11800. Any other figure = STOP (the book is not as recorded).

Then backup **BK1** (C5) into a NEW empty folder `C:\bridge-lab\1211\backups\BK1\`. This is the state R2 is restored to.

## R1: windows ending mid-month, after the post (reads only)
1. Bridge `parse_bank_statement` on THE STATEMENT with `from` 2026-08-01 and `to` 2026-08-15. Save the answer.
2. Bridge `statement_tie_out` for that parse, bank ledger Union Bank. Save the answer.
3. The same two calls with `from` 2026-07-01 and `to` 2026-08-15: a window starting at a month boundary and ending mid-month, over two posted months.
4. **Control:** the same two calls for the whole of July (2026-07-01 to 2026-07-31).

EXPECT:
- steps 1 to 3: opening, closing and change each `not_established` with a reason code, `window` null, and no amount;
- step 4: tied 0.00 three times, as on 14 October.

An error (no typed result), or any amount on a part-month window, is the finding.

## R2: a second bank ledger, and Tally's own figures (writes by hand, then restored)
1. **By hand:** create ledger `BRIDGE Bank B` under Bank Accounts, with no opening balance. Screenshot it before the accept.
2. **By hand:** key Receipt W1, dated 15-07-2026: BRIDGE Bank B Dr 700.00, THE COMPANY's Suspense ledger Cr. Record the Suspense ledger's exact name from the screen. W1 does not touch Union Bank. Screenshot before the accept.
3. Bridge `statement_tie_out` for July and for August, each from a parse of that whole month.

   EXPECT tied 0.00 three times for each: the bank ledger still matches its statement.
4. Raw reads, Tally's own figures (TBALCLOSING per ledger, C4):
   - `requests/r02_trial_balance_to_20260630.request.utf16le.xml` (Union Bank on the eve of July);
   - `r03_..._to_20260731` (the end of July);
   - `r05_..._to_20260831` (the end of August).

   EXPECT:
   - Union Bank's closing to 30 June equals the opening `statement_tie_out` read for July;
   - Union Bank's closing to 31 July equals July's book closing;
   - Union Bank's closing to 31 August equals August's book closing, -11800 as in step 0;
   - BRIDGE Bank B holds 700.00 Dr from 31 July.
5. **By hand:** key Contra W2, dated 20-08-2026: BRIDGE Bank B Dr 1,000.00, Union Bank Cr. It is not on the statement. Screenshot before the accept.
6. Bridge `statement_tie_out` for August (whole month) again, then raw read `r05` again.

   EXPECT: opening tied 0.00; closing and change each differing by 1,000.00, in the sign the tool states. The book closing the tool reports equals Union Bank's TBALCLOSING to 31 August in this second `r05`.
7. Raw read `r04_trial_balance_to_20260815`: Union Bank to the middle of August, recorded for the slice's notes (no EXPECT).
8. Marks M1.

## After R2: restore
Restore BK1 (C6). Then marks M2, and Bridge `ledger_movement` for Union Bank, 1 July to 30 September 2026.

EXPECT: M2 equals M0, and the movement equals step 0's. If not: STOP, and report what the book holds.

## The book after each step
- After step 0 and R1: unchanged.
- After R2 step 1: plus BRIDGE Bank B.
- After step 2: plus W1.
- After step 5: plus W2.
- After the restore: as at BK1, which is the book of step 0.

## Close and evidence
- **C10:** Shut Company; no company loaded.
- **C11:** evidence to a new branch `lab/1211-tie-out-rows-answers`, after the pre-push scan and [OWNER] yes. It holds:
  - every Bridge answer and raw answer, whole, with size and sha256;
  - the egress log lines this build's data folder gained;
  - the notes;
  - the backup's name, size and hash.

  Screenshots and the statement PDF stay on the lab PC, listed by name, size and sha256.
