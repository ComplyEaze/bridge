# The 0.7.0 release-candidate sitting: results (Part of #2056)

Sheet: `lab/0.7.0-rc-sitting` (R1 ran on 1a8c47ad; R2, R3, R5 and R7 on 075691dd, whose changes touch no R1 line). Build for every row: candidate A, RC COMMIT 026ba163dfcbf75d515a24d0dbbf14c5245534ec, EXE SHA256 576cc0c5aab4e72c43c05799116d76a29429c595a86dd54834d841c551f50d7c. TallyPrime 7.2 Silver on the Windows lab PC. Each row's folder holds its `RESULTS.md`, `notes.txt` and `SHA256.txt`.

| row | result | company | date (UTC) | folder |
|---|---|---|---|---|
| R1 | PASS: 1967 PASS, 1995 PASS, the post PASS, 1896 no false line PASS, rollback record PASS, rollback cover PASS; the box's date line `30-Sep-2026` | BRIDGE TIE OUT LAB 3 | 30 Oct 2026, 06:16 to 06:31 | `R1/` |
| R2 | PASS: the decline PASS (`master_approval_declined`, nothing sent), the create PASS (`posted_verified`, created 1, mark step one), the journal answer PASS (`posted_verified`, no box); the box whole on the screen, topmost, holding the keyboard; one lab error (a refused call with an extra argument, nothing sent) | BRIDGE LEDGERBOX CUT LAB (new, `<company guid>`) | 30 Oct 2026, 06:34 to 06:45 | `R2/` |
| R3 | not completed: Tally's period lock needs user security (a user and password); nothing saved; book as found (step 0 PASS; stopped at step 2.1 by its own rule) | BRIDGE ABANDON LAB | 30 Oct 2026, 06:47 to 06:54 | `R3/` |
| R4 | not run: R4 HOST (#2158 not in the build) | | | |
| R5 | PASS: steps 2, 3 and 4 as the sheet gives them (four buckets, category totals, totals, unallocated entries, the bank-selected read), marks unchanged | BRIDGE COST KEY LAB | 30 Oct 2026, 06:57 to 06:59 | `R5/` |
| R6 | not yet run (owner at the PC) | | | |
| R7 | PASS on the alias (`near_miss` with `alias_of` `Lab Plain Ledger`; the ledger `exact`); FINDING on the control: `near_miss` by the shared-token rule, not `missing`, with no `alias_of` and no binding (fails safe; the sheet's expectation was wrong); the book restored from BK0 | BRIDGE LINE BREAK LAB | 30 Oct 2026, 07:00 to 07:08 | `R7/` |
