# 692-R2C: which figure R2's carried Profit & Loss A/c holds (results)

Sheet: `lab/692-r2-carry-check` at 92225cd0 (its SHA256.txt: 3 of 3 equal, `sheet-check.txt`). Look only: nothing was sent to port 9001, and no company was loaded, keyed, backed up or restored.

Run once on 5 Nov 2026 (UTC time in `run-output.txt`): `python -I check_r2_carry.py <the 692 run's answers folder>`, from the sheet folder; exit 0. The four kept answers each equal, in bytes and sha256, the record 09287f70 made for them:

| answer | bytes | sha256 |
|---|---|---|
| r1-w1-pl.utf16le.xml | 1922 | equal to the record |
| r1-w2-ledgers.utf16le.xml | 13548 | equal to the record |
| r2-w1-pl.utf16le.xml | 1922 | equal to the record |
| r2-w2-ledgers.utf16le.xml | 13548 | equal to the record |

Counts: r1-w1-pl has 6 Profit and Loss lines and 1 "Less: Closing Stock" line; r2-w1-pl has 1 "Less: Closing Stock" line; r1-w2-ledgers and r2-w2-ledgers each have 1 Profit & Loss A/c ledger.

| check | verdict |
|---|---|
| 1. R2's year-2 P&L A/c opening == R1's year-1 nett | **True** (with the sign reversed: False) |
| 2. R2's year-2 P&L A/c opening == R1's year-2 P&L A/c opening | **True** (with the sign reversed: False) |
| 3. R2's year-1 closing stock != R1's | **True** |

Reading, from the sheet's table (True, True, True): Tally carries a year-1 result that a later costing change does not follow, while year 1's own statement does follow it. The P&L A/c opening is taken from the ledger (TBALOPENING) and never derived from year 1. The answers stay on the lab PC; no name or amount is printed or committed.
