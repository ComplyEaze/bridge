# 692-R1 to R5: opening stock that Tally carries from a prior year (results)

Sheet: `lab/692-opening-stock` at 991c3ec4 (SHA256.txt 41 of 41 equal by sha256 and size). LAB-COMMON v3 (5370c6d2..., byte copy in the sheet). TallyPrime 7.2 Silver, one tally.exe owning port 9001. THE COMPANY: `BRIDGE TIEOUT E`, created on screen in step 1; its number, GUID and backup name stay on the lab PC. Shapes and counts only: no answer and no amount is committed (the committed notes show every amount as `<amount>`).

## PASS

PASS: every condition of the sheet holds.
- Every read was whole within its bound: 200, size equal to CONTENT-LENGTH, parsed (C4). The marks were within their own bound, every other read within 2,000,000 B (send-log.txt).
- The marks were equal around every read group: R1 w1 to w4, R5 w5, R2 w1 and w2, R3 w2, R4 w2 (marks.txt).
- Every check's printed lines are recorded (checks.txt).
- Each restore came back at M_E 216 and V_E 7, and its `w2-pl` sha256 equals R1's (2beec565...): r2-after, r3-after, r4-after.
- C10 is done: one tally.exe owning 9001, GET /status 200, Select Company with none loaded, and the current date set back as at step 0.

None of the FAIL conditions occurred: no transport failure, every restore returned to BK-E, and no prompt was answered that the sheet, LAB-COMMON or the run lead's word did not name.

## Setup (steps 1 to 8)

- **Step 1:** BRIDGE TIEOUT E was created, books and financial year from 1-Apr-2024, INR. F11 is set to inventory Yes, integrated Yes and batches No; GST was changed from Yes to No. The 7.2 features screen shows no godowns field.
- **Step 2:** one `list_companies` call of the staged build named by the run lead (master-87ee2834, bridge_mcp.exe ace7cdb0..., the same sha256 before and after). It ran over stdio with a new empty agent data folder and no writes, import or batch-post settings. It returned exactly one company, `BRIDGE TIEOUT E`, verified_tuple, with 1 GUID, uuid-shaped (= EXPECT). The server exited by itself, exit code 0, after its input closed. The census on the filled marks file is ok.
- **Steps 3 to 6:** the masters took M from 206 to 216. Each of the seven vouchers took V up by one (1 to 7) with M unchanged (= EXPECT, with the step-4 note below).
- **Step 5:** Alt+F2 at the Gateway opened Change Period (1-4-2024 to 31-3-2025). It was set to 31-3-2026, and there was no prompt of any kind.
- **Step 7:**
  - Stock Summary 2024-25: no negative quantity.
  - Stock Summary 2025-26: no negative quantity.
  - Balance Sheet as at 31-3-2026: a "Difference in opening balances" line: **no**.
- **Step 8:** BK-E is one file, 33,592 B, sha256 e5028e9b590e58976768527c651774d3fdaabcc1367ded80dce7013fbde4b29f. M_E 216, V_E 7.

**Note under step 4 (deviation, on the run lead's word).** The sheet says: "MARKS after each voucher: EXPECT V up by one each time and M unchanged". Before the first voucher, the COMPANY row of the marks answer holds no ALTVCHID element at all, so V was recorded as "absent". Reading agreed with the run lead before the first voucher:
- "absent" before the first voucher is not a miss, and the first voucher's V is the base;
- after the first voucher V must be present (it was: 1);
- each later voucher must be exactly +1 (it was: 2, 3, 4, 5, 6, 7).

## Rows (verdicts as `identities.py` and `relations.py` print them; every line is in checks.txt)

### R1, carried opening stock (on BK-E's state), with `r1-groups`

| window | identities 1, 2, 3, BS balances, O and C debits | relation 4 | stock summary total == C | negative stock rows |
|---|---|---|---|---|
| w1 2024-25 | all True | True | True | 0 |
| w2 2025-26 | all True | True | True | 0 |
| w3 Apr-Sep 2025 | all True | True | True | 0 |
| w4 Oct 2025-Mar 2026 | all True | not checked: the Cost of Sales heading is without its amount | True | 0 |

- In every window the Opening Stock and Less: Closing Stock lines are each present once. There are 9 ledger rows and none under Stock-in-Hand.
- w1's Opening Stock line is present and read as zero by identities.py, and every w1 relation is True.
- `carry` w1 with w2:
  - year 2's opening stock == year 1's closing stock: **True**;
  - Profit & Loss A/c ledgers in year 2: 1;
  - its opening == Tally's year-1 nett (sum of the main column): **True** (reversed False);
  - its opening == the year-1 ledgers' result (nominal closings): **False** (reversed False).
- Of the cases the sheet names as showing the premise wrong, none occurred:
  - no relation is False in w1 to w4;
  - year 2's opening equals year 1's closing;
  - the P&L A/c opening equals one candidate (Tally's statement).
- w4's relation 4 is the one "not checked" line.

### R5, the window across both years (read only, straight after R1)

- w5 (1-4-2024 to 31-3-2026): every verdict is True, including relation 4 and stock summary total == C. There are 0 negative stock rows.

### R2, costing method (Lab Item A, Avg. Cost to FIFO)

- After the change: M 217 (+1) and V 7 (= EXPECT).
- w1 and w2: every identity and relation is True.
- `carry`:
  - opening stock == year 1's closing: True;
  - P&L A/c opening == Tally's year-1 nett: **False**, and == the ledgers' result: **False** (both reversed False). After the change it equals **neither** candidate.
- Restored: M 216, V 7, `r2-after-w2-pl` = `r1-w2-pl`.

### R3, negative stock (Lab Item C, a Sales of 3 Nos with no purchase)

- **Result on screen:** Tally warns 'Negative stock.' / '(-)3 Nos' / 'Press any key to continue' on the quantity line and lets the entry continue.
  - The box was not named by the sheet, so it was a C7 stop: no key was pressed and nothing was sent until the run lead's word.
  - On that word, one Enter cleared it; the cursor was on the line's empty rate; the voucher was completed as the sheet says.
- Tally's own screens:
  - Stock Summary 2025-26 shows Lab Item C at (-)3 Nos with no rate and no value.
  - Negative Stock Items shows that one row with no value.
- After the change: V 8 and M 217, each +1 (= EXPECT).
- w2:
  - identities all True; relation 4 True;
  - stock summary rows 3, without a value 1; stock summary total == C: not checked;
  - negative stock rows 1, without a value 1 (as D3's X1: one row without a value in each).
- Restored: M 216, V 7, `r3-after-w2-pl` = `r1-w2-pl`.

### R4, a direct expense (Lab Freight In under Direct Expenses, a Payment on 15-May-2025)

- After the change: V 8 and M 217, each +1 (= EXPECT).
- w2:
  - identities all True, with 10 ledger rows; Profit and Loss lines 7;
  - lines naming Direct Expenses: 1, with a sub amount 1, with a main amount 0;
  - relation 4 (with the direct expense in the sub column): True; stock summary total == C: True.
- Restored: M 216, V 7, `r4-after-w2-pl` = `r1-w2-pl`.

## Other records

- **Helper fault, fixed, nothing resent.** My own C4 parse check first rejected `r1-groups` (200, size = CONTENT-LENGTH): it turned the CR after the closing ENVELOPE tag into text after the root element. The check was fixed and the saved answer parsed (28 GROUP rows). The answer is unchanged.
- **Marks bytes after a load.** After each restore and load, the marks answer is 4,886 B against 4,892 B before. The COMPANY row's marks are equal, so the difference lies outside the row.
- **Period and date on load.** After each load, Tally shows Current Period 1-Apr-25 to 31-Mar-26 (its own choice). The requests carry their own dates, and every `wN-pl` read after a restore was byte-equal to R1's.
- **Close.** Alt+F2 set the period back to 1-4-2024 to 31-3-2025 without a prompt. Tally then moved the current date to 31-Mar-2025, and F2 (Date) set it back to 1-Apr-2024, as at step 0. THE COMPANY (at BK-E's state) and BK-E stay on the lab PC for later rows.
- **Not touched.** The four desktop-client connector servers were left untouched: no call was made to them from this sitting, and their calls were not measured. Nothing ended a process.

## Files

`checks.txt` (every printed line, by row and window), `marks.txt`, `send-log.txt`, `screens.txt`, `notes.txt` (UTC; amounts and lab paths masked), `SHA256.txt`. The committed files are listed first; the files kept on the lab PC (answers, screenshots, BK-E, the filled marks file, the Bridge answer, the restore plans and the notes as written) follow under `#` lines.
