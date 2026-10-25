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

**THE BUILD** (named 26 Oct 2026) is the build already staged at the lab as `w1-2306d86d`: the Windows bundle of master `2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb`, Bridge CI run `40077056130` (event `schedule`), artifact `12553782422` (zip digest `sha256:74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e`); its `bridge_mcp.exe` as the lab recorded it on staging: sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524`. Record the exe's sha256 at the start and at the end; any other hash = STOP. Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` for the whole item, `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), `BRIDGE_AGENT_ENABLE_IMPORT=true` (`parse_bank_statement` refuses without it), `BRIDGE_AGENT_ENABLE_WRITES` unset (posting NOT set) and `BRIDGE_AGENT_REDACTION` unset.

**NO RAW REQUESTS (26 Oct 2026).** Every read in this item is a call of the build's own tools, the one dispatch path to Tally. The files in `requests/` are edited copies of committed requests and are NOT sent (AGENTS.md P7): they stay here only as the record of what this sheet first asked. The marks are read with `ledger_masters` (first page, no `snapshot_id`): M = `snapshot.master_alter_id`, V = `snapshot.voucher_alter_id`, `snapshot.reused` must read false (`tally_status` carries no marks). Tally's own figures per ledger are read with `trial_balance` from `2026-04-01` (the books' first day) to the date named; record each named ledger's row whole (its closing balance as the tool names it, with its sign). In licensed mode `trial_balance` admits any day; its refusals here would be `trial_balance_before_books` or `trial_balance_education_unqualified`: either is recorded whole as a RESULT, and the EXPECT that needs it is reported "not established". Its amounts are in Tally's sign (a debit balance is negative, reference §5.6): compare `closing.value` numerically; a `present_empty` amount is "not established". The only raw sends left are LAB-COMMON's GET /status checks (C2 c and C10), which stay.

**THE CALLS.** `parse_bank_statement` is called with the same required arguments the 14 October run used for THE STATEMENT (bank, account label, password file, opening and closing balance, bank ledger `Union Bank`, suspense ledger; from that run's own call records on the lab PC), changing only `from` and `to` as each step says. `statement_tie_out` is called with `company_guid`, the `proposals_id` and `proposals_sha256` that parse returned, and `stage` `after_post` (the book as it stands now) in every step. Its three figures are in the statement's sign (positive: the book shows more money in the bank than the statement).

**OWNER steps** are marked [OWNER].

## Before the sitting
1. [OWNER] Tally is LICENSED. The keyed dates in R2 are not days 1, 2 or 31. Educational mode = STOP (C1).
2. Check every file here against `SHA256.txt`.
3. [OWNER] Go-ahead.

## Step 0 and the book as recorded
C2 step 0 with THE COMPANY loaded alone, WITHOUT its MARKS request file (see NO RAW REQUESTS). Then:
- `list_companies`: EXPECT exactly one company, BRIDGE TIE OUT LAB 3, GUID `9c97970c-e25d-4348-9740-6ddb1caaa56d`, `books_from` 20260401, as the 14 October run recorded it (`lab/1628-m-month-window-tie-out-answers`, `results/s0-list_companies.json`). Anything else = STOP. Every later call uses this `company_guid`;
- `ledger_masters` (marks M0);
- Bridge `ledger_movement` for Union Bank, 1 July to 30 September 2026.

EXPECT, as on 14 October: opening -10000.00, closing -11800. Any other figure = STOP (the book is not as recorded).

Then backup **BK1** (C5) into a NEW empty folder `C:\bridge-lab\1211\backups\BK1\`. This is the state R2 is restored to.

## R1: windows ending mid-month, after the post (reads only)
1. Bridge `parse_bank_statement` on THE STATEMENT with `from` 2026-08-01 and `to` 2026-08-15. Save the answer.
2. Bridge `statement_tie_out` for that parse, bank ledger Union Bank. Save the answer.
3. The same two calls with `from` 2026-07-01 and `to` 2026-08-15: a window starting at a month boundary and ending mid-month, over two posted months.
4. **Control:** the same two calls for the whole of July (2026-07-01 to 2026-07-31).

EXPECT:
- steps 1 to 3: opening, closing and change each `not_established` with reason `window_not_whole_calendar_months`, and no amount. The tool answers these from the file alone and reads no ledger: record the call's sends from its `egress_log` lines (the 14 October run sent no ledger read here). This row therefore shows the typed answer on a part-month window; it shows nothing that depends on the post;
- step 4: tied 0.00 three times, as on 14 October.

An error (no typed result), any amount on a part-month window, or a ledger read sent for one is the finding.

## R2: a second bank ledger, and Tally's own figures (writes by hand, then restored)
1. **By hand:** create ledger `BRIDGE Bank B` under Bank Accounts, with no opening balance. Screenshot it before the accept.
2. **By hand:** key Receipt W1, dated 15-07-2026: BRIDGE Bank B Dr 700.00, THE COMPANY's Suspense ledger Cr. Record the Suspense ledger's exact name from the screen. W1 does not touch Union Bank. Screenshot before the accept.
3. Bridge `statement_tie_out` for July and for August, each from a parse of that whole month.

   EXPECT tied 0.00 three times for each: the bank ledger still matches its statement.
4. Tally's own figures per ledger, by `trial_balance` from `2026-04-01`:
   - to `2026-06-30` (Union Bank on the eve of July);
   - to `2026-07-31` (the end of July);
   - to `2026-08-31` (the end of August).

   EXPECT (Tally's sign, compared numerically):
   - Union Bank's closing to 30 June = -10000.00;
   - Union Bank's closing to 31 July = minus the statement's balance on 31 July as the proposals file records it on the lab PC (`window.tie_out`); record both figures;
   - Union Bank's closing to 31 August = -11800.00, as in step 0;
   - BRIDGE Bank B's closing to 31 July = -700.00 (700.00 Dr).
   (`statement_tie_out` returns gaps, not the book's balances; so these are compared with Tally's own figures and with the statement's, not with a figure the tool reports.)
5. **By hand:** key Contra W2, dated 20-08-2026: BRIDGE Bank B Dr 1,000.00, Union Bank Cr. It is not on the statement. Screenshot before the accept.
6. Bridge `statement_tie_out` for August (whole month) again, then `trial_balance` from `2026-04-01` to `2026-08-31` again.

   EXPECT: `opening_gap` tied 0.00; `closing_gap` and `change_in_window` each `differs` with amount -1000.00 (the book shows 1,000.00 less in Union Bank than the statement); Union Bank's closing to 31 August in this second `trial_balance` = -10800.00.
7. `trial_balance` from `2026-04-01` to `2026-08-15`: Union Bank to the middle of August, recorded for the slice's notes (no EXPECT; a refusal for the mid-month date is itself the note).
8. `ledger_masters`: marks M1.

## After R2: restore
Restore BK1 (C6). Then `list_companies` (EXPECT as in step 0), `ledger_masters` (marks M2), and Bridge `ledger_movement` for Union Bank, 1 July to 30 September 2026.

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
  - every Bridge answer, whole, with size and sha256;
  - the egress log lines this build's data folder gained;
  - the notes;
  - the backup's name, size and hash.

  Screenshots and the statement PDF stay on the lab PC, listed by name, size and sha256.
