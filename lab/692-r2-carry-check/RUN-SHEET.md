# Row 692-R2C: which figure R2's carried Profit & Loss A/c holds. Look only: no Tally request. Not for merge.

Written 5 Nov 2026 for #692. Rows 692-R1 to R5 (`lab/692-opening-stock-answers` at 09287f70) found one carry that equals
neither candidate. After Lab Item A's costing method went from Avg. Cost to FIFO (R2), year 2's Profit & Loss A/c
opening equalled neither Tally's year-1 nett nor the year-1 nominal ledgers' result. This row asks which figure it is,
from the answers that run already kept on the lab PC. About 5 minutes.

## What is run

`python -I check_r2_carry.py <the 692 run's answers folder>`, once, from this folder. It:
- reads four kept answers: `r1-w1-pl`, `r1-w2-ledgers`, `r2-w1-pl` and `r2-w2-ledgers`;
- refuses, checking nothing, unless each equals in bytes and sha256 the line `SHA256.txt` of 09287f70 records for it;
- prints counts and three verdicts, True, False or `not checked`, and no name or amount.

Nothing is sent to port 9001. Tally need not be running, and no company is loaded, keyed, backed up or restored.
LAB-COMMON does not apply: no step of this row touches Tally.

## The three checks

| check | True means |
|---|---|
| 1. R2's year-2 P&L A/c opening == R1's year-1 nett (with the sign reversed too) | the opening is still the year-1 result as it was before the costing change |
| 2. R2's year-2 P&L A/c opening == R1's year-2 P&L A/c opening | the opening did not move with the change |
| 3. R2's year-1 closing stock != R1's | the change revalued year 1's closing stock |

## What the result decides

| 1, 2, 3 | reading |
|---|---|
| True, True, True | Tally carries a year-1 result that a later costing change does not follow, while year 1's own statement does follow it. The P&L A/c opening is taken from the ledger (TBALOPENING) and never derived from year 1. Recorded on #692 with this row's confidence. |
| 3 False | The change did not revalue year 1, so R2's carry difference has another cause. It needs its own row before any rule. |
| 1 or 2 False, 3 True | The opening moved, but not to either year-1 figure. It needs its own row before any rule. |
| any `not checked`, or the refusal | Recorded whole. Nothing is decided. |

## Keep

On `lab/692-r2-carry-check-answers`: the whole printed output, the command line, the run's UTC time and this
folder's `SHA256.txt` check. The answers stay on the lab PC. Send the sha to the run coordinator before any push.
