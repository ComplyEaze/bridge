# #485 slice 2: `ledger_movement` for one ledger, end to end on a built tool

Not for merge. READS ONLY, on one company the lab already holds: BRIDGE CORPUS DENSE (synthetic, voucher mark 29,900 on 28 Oct 2026; its GUID is the one part E of `lab/485-ledger-census-spans` recorded at its step 1). `LAB-COMMON.md` in this folder (v3, a byte copy of the one in `lab/485-ledger-census-spans`) applies in full, with this company as THE COMPANY; where this sheet and it differ, the stricter holds.

**THE BUILD.** The staged Windows build named with this sheet (the package of the pull request that changes `ledger_movement`). Record its file name, the `bridge_mcp.exe` sha256 and the commit it reports. If none is named, STOP. The build is only used through its tools (`list_companies`, `tally_status`, `ledger_movement`, `trial_balance`); no raw request is sent in this item. Redaction stays off (the default); report it.

**WHY.** `ledger_movement` with a `ledger` used to read every voucher of the window in the book, so a year of this book (about 30,000 vouchers) was refused. The built tool reads only the named ledger's vouchers: it counts them in AlterID spans, then reads them in one request of the shape part E of `lab/485-ledger-census-spans` observed. Each of the two request shapes was read on this book on its own; this is the first time the tool sends them. This item decides whether the tool's answer is right, not whether Tally accepts the shapes.

**WHAT IT DECIDES.** (1) The tool answers `window_scope.basis` `ledger_scoped` with the vouchers read equal to the count part E found, and its debit and credit equal the Trial Balance's for the same ledger and period. (2) The year of one ledger, refused before, is answered. (3) A ledger name the scoped read does not admit is answered from the book's window and says so. (4) The scoped answer equals the book-wide answer for the same day.

**RULES.** C1 to C11 of `LAB-COMMON.md`. Added here:
- Every call is one tool call at a time, never repeated while its answer is outstanding; an answer is read when it arrives. A call without an answer in 10 minutes, or a tool answer whose `evidence.bytes` is over 40,000,000, is a STOP (keep the answer, send nothing more). `evidence.bytes` counts each request twice, so it is an upper bound on any one answer.
- A refusal is a result: save it whole and go on to the next step unless the step says STOP.
- A dialog on Tally's screen at any time is C7. Tally showing Educational mode (title bar, Gateway or `tally_status`) is a STOP.
- The book does not change: nothing is keyed, backed up or restored. The marks at the start and the end of the item must be equal.
- Screenshots stay on the lab PC (C11).

## Before the item
1. Load THE COMPANY alone (C1) and run step 0 (C2).
2. Check every file in this folder against `SHA256.txt`. A difference = STOP.
3. Record the build (name, sha256, reported commit) and Tally's version, release, edition and licence (About screen and `tally_status`).
4. [OWNER] Go-ahead.

## Steps
For every tool call record the UTC time before the call and after the answer, and save the answer whole (file name `t<step>-<tool>.json`), with its size and sha256.
1. `list_companies`: THE COMPANY must be the only company listed, with the GUID part E recorded; report it. `tally_status`: product and mode as step 0 found them.
2. `trial_balance`, THE COMPANY, from 2025-05-01 to 2025-05-31. Save it. Note `DN Party 001` and `DN Sales`: opening, debit, credit, closing as sent.
3. `ledger_movement`, THE COMPANY, ledger `DN Party 001`, from 2025-05-01 to 2025-05-31. EXPECT `window_scope` `{"basis":"ledger_scoped","vouchers_read":21}`, `voucher_rows_observed` 21, and the ledger's debit and credit equal to step 2's, exactly. A different basis or count is a result, not a STOP: record the `reason` if there is one.
4. `trial_balance`, from 2025-04-01 to 2026-03-31 (the year part E read). Save it; note `DN Party 001` as in step 2.
5. `ledger_movement`, ledger `DN Party 001`, from 2025-04-01 to 2026-03-31. This is the call that used to be refused. EXPECT `ledger_scoped`, 248 vouchers read, debit and credit equal to step 4's. Record `read_cost` if present and the time the call took. If it is refused, record the code, `cause` and `remediation` whole.
6. `ledger_movement`, ledger `DN Sales`, from 2025-05-15 to 2025-05-15 (the day part E counted: 82 vouchers). EXPECT `ledger_scoped`, 82 vouchers read.
7. `ledger_movement` for the same day without a `ledger` (every ledger). This reads the day's vouchers book-wide, as part D did. The ledgers come in pages: pass a `limit` large enough for one page if the tool allows, else follow `next_offset` until it is null, and save every page. Compare `DN Sales` here with step 6: opening, debit, credit and closing must be equal. If this call is refused for size, record that and go on.
8. `ledger_movement`, ledger `Profit & Loss A/c`, from 2025-05-15 to 2025-05-15. The name has characters the scoped read does not send. EXPECT `window_scope` `{"basis":"whole_book","reason":"ledger_name_outside_scoped_alphabet"}` with a `note`, and the figures of that ledger from the day's book-wide read (compare with step 7's row of the same ledger: equal).
9. `ledger_movement`, ledger `dn party 001` (lower case), from 2025-05-01 to 2025-05-31. EXPECT the same figures as step 3, `ledger_match` naming `matched: case_or_spacing` and `DN Party 001`, and `ledger_scoped`: the tool must send the book's spelling, not the typed one.
10. `tally_status`, then the marks as step 0 (C2 d). EXPECT the same ALTVCHID and ALTMSTID as at the start.
11. C10.

## RECORD
For every answer: the tool, arguments, time taken, `isError`, `window_scope` whole, `ledger_match`, `voucher_rows_observed`, the ledger rows' opening, debit, credit and closing as sent, `evidence.bytes`, `evidence.state`, `evidence.reason_code`, the response hash, and `read_cost` if present. For each refusal: the code, `cause` and `remediation` whole. For steps 2 and 4: the rows of `DN Party 001`, `DN Sales` and `Profit & Loss A/c` as sent.

## COMPARED BY THE REQUESTER AFTER THE RUN
- Step 3 against step 2, step 5 against step 4: the ledger's debit, credit and closing, exactly.
- Step 6 against step 7, step 8 against step 7: exactly.
- The vouchers read in steps 3, 5, 6 against the counts part E found (21 in May, 248 in the year, 82 on the day).
- The time of step 5 against the time part E recorded for the census spans and the year read together.

## NOT COVERED
A scoped read that finds nothing, and a day too heavy for one scoped read: no ledger and window on this book is known to produce either, so the fallback to the book's window for those two is covered by the tool's own tests only.
