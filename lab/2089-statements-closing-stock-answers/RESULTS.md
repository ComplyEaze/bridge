# Results: #2089 statements on the two closing-stock books, 29 Oct 2026. On both books and both builds (BUILD M af4a0dbf, BUILD Z 609776ae), `profit_and_loss` and `balance_sheet` refuse with cause `statement_trial_balance_row_inconsistent` and no remediation, naming neither the Closing Stock ledger, the Stock-in-Hand group nor the gap, as EXPECTED. The two builds' answers do not differ. Reads only; the marks did not move

The sheet is `lab/2089-statements-closing-stock` at 2083e6ba3691b4dfa658e46d09fc723a43e43073. `SHA256.txt` matched 8 of 8, `gen_requests` and the census printed the EXPECT lines, and Tally is 7.2 Silver Rental, licensed.

| build | head | `bridge_mcp.exe` | server | tools |
|---|---|---|---|---|
| BUILD M | master af4a0dbf7f7d6b9432b2b6f0994c217c69c8279e, run 40671680104 | 9f73d533dcac8a24d1d33389a7db33cf4055e212f7dc97f8c170f00c08b73fd1 | 0.6.1 | 30 |
| BUILD Z | master 609776ae9f5cec48e6563f165480d3fe1adae445, run 40366984202 | 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44 | 0.6.0 | 29 (no `gst_by_counter_ledger`) |

Both exes were unchanged at the end. Each launch had a new data folder, with writes, import and redaction off.

| | BOOK N, BRIDGE NONINT LAB | BOOK I, BRIDGE INTEGRATED CTRL LAB |
|---|---|---|
| company | 100076, 567653b1-c8be-4deb-b4fc-3ea5de801884 (= 2068's) | 100095, 3fc2a3c7-1da8-4bc4-a472-44028cdebca7 (= 2068's) |
| marks | M 215, V 3 before, and after every read, on both builds | the same |
| ledgers | 8, Closing Stock under Stock-in-Hand | the same |
| raw ledger collection | 200, 12,312 B = CONTENT-LENGTH, sha256 4589d3d3… | 200, 12,312 B = CONTENT-LENGTH, sha256 cb3c17b9… |
| Closing Stock row (raw) | TBALOPENING -1000.00, DEBITTOTALS -400.00, CREDITTOTALS empty, TBALCLOSING -1000.00: **does not add up**; every other ledger adds up | the same |
| `trial_balance` 20250401 to 20260331 | state `observed`; the row as opening -1000.00, debit -400.00, credit `present_empty`, closing -1000.00; totals debit -3200, credit 2800, closing 0; no text naming the ledger | the same |
| `profit_and_loss` | refusal `profit_and_loss_read_failed`, cause `statement_trial_balance_row_inconsistent`, no remediation; evidence partial | the same |
| `balance_sheet` | refusal `balance_sheet_read_failed`, cause `statement_trial_balance_row_inconsistent`, no remediation; evidence partial | the same |
| names the ledger, the group or the gap | no | no |
| BUILD M vs BUILD Z | no difference in any field (read times, snapshot ids, durations and pids left out; even the fingerprints are equal) | no difference |

**Expected, and different.** Each statement refusal's `evidence` carries the company read's bytes and fingerprint (8,008 B / 8,080 B, the same `response_sha256` as `list_companies`), not a Trial Balance read's.

**Not covered.** As the sheet says:
- Tally's own statement screens are not repeated;
- #2089's flag is not measured;
- no other window is read.

**Confidence:** one observation per book, PARTIAL until repeated.

## Files
- `run-n-m/`, `run-n-z/`, `run-i-m/`, `run-i-z/`: each run's tool answers whole and its client log.
- `raw/`:
  - the two raw ledger collections, with their headers;
  - the rows as read by structure;
  - the send log.
- `checks/`:
  - step 0 and each book's close checks;
  - the build comparisons (`compare-n.txt`, `compare-i.txt`).
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256. They stay on the lab PC.
