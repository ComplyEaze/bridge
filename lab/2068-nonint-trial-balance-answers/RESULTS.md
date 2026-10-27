# Results: #2068. What `trial_balance` reports for the Closing Stock ledger on a non-integrated book, TallyPrime 7.2, 27 Oct 2026 (UTC)

The sheet is `lab/2068-nonint-trial-balance/RUN-SHEET.md` at 7510f4394ba60ec10c2418ec1f4db28d625f9192 (SHA256.txt 3 of 3 equal; LAB-COMMON v3, 5370c6d2). This was a read only: nothing was created, altered, imported or restored, and the marks did not move (M 215, V 3 before and after). **Confidence: one observation, PARTIAL until repeated.**

## The record
On `BRIDGE NONINT LAB` (Integrate Accounts with Inventory No), `trial_balance` was run for 1 April 2025 to 31 March 2026. It reports the `Closing Stock` ledger (under Stock-in-Hand) as follows:

| field | as returned |
|---|---|
| opening | -1000.00 |
| debit | **-400.00** |
| credit | present_empty |
| closing | **-1000.00** |

- **Closing reads 1,000.00 Dr, the opening value.** It is not the 1,400.00 Dr entered in the ledger's Closing Balance field at 31 March 2026. The 400.00 appears only as the debit movement. This is the same pattern as `TBALCLOSING` and `DEBITTOTALS` in the #692 NONINT ledger collection.
- **Tally's own Trial Balance shows the same 1,000.00.** Ledger-wise, the Stock-in-Hand ledger's line is labelled **Opening Stock**, 1,000.00 Dr; there is no line named Closing Stock. Group-wise, Current Assets is 2,600.00 Dr, which is 1,600.00 Customer + 1,000.00. So the answer and the screen agree, line for line.

## Does it tie
- **Closing balances tie.** The answer's `totals.closing.sum` is 0. Recomputed from the rows: Dr 1,000.00 + 1,600.00 + 1,200.00 = 3,800.00; Cr 1,000.00 + 1,600.00 + 1,200.00 = 3,800.00. That equals Tally's Grand Total of 3,800.00 = 3,800.00.
- **Movements do not tie.** `totals.debit.sum` is -3200 and `totals.credit.sum` is 2800, a difference of 400 (the Closing Stock debit). For every other ledger, opening + debit + credit = closing. For `Closing Stock`, -1000.00 + -400.00 = -1400.00, but closing is -1000.00.
- `totals_scope` is `all_returned_ledgers_numeric_observations_only`, and empty amounts are counted as `empty_count`, not as zero. There is no `foreign_currency_ledgers_excluded` and no `read_cost`.

| ledger | parent | opening | debit | credit | closing |
|---|---|---|---|---|---|
| Cash | Cash-in-Hand | 0.00 | empty | empty | empty |
| Closing Stock | Stock-in-Hand | -1000.00 | -400.00 | empty | -1000.00 |
| Customer | Sundry Debtors | 0.00 | -1600.00 | empty | -1600.00 |
| Lab Capital | Capital Account | 1000.00 | empty | empty | 1000.00 |
| Profit & Loss A/c | Primary | 0.00 | empty | empty | 0.00 |
| Purchases | Purchase Accounts | 0.00 | -1200.00 | empty | -1200.00 |
| Sales | Sales Accounts | 0.00 | empty | 1600.00 | 1600.00 |
| Supplier | Sundry Creditors | 0.00 | empty | 1200.00 | 1200.00 |
| **totals** | | 0 | -3200 | 2800 | 0 |

The answer's snapshot was id 38617d88-94a9-44d7-85b8-c124a1c9b88e, M 215, V 3, `reused` false. Its state was `observed`, basis `tally_native_trial_balance`, 8 of 8 ledgers on one page.

## Setting
- TallyPrime 7.2 Silver, licensed, not in Education mode. Exactly one tally.exe owned port 9001 at step 0 and at the close (the process list, netstat and Get-NetTCPConnection), and GET /status returned 200 both times.
- `BRIDGE NONINT LAB` is company number 100076, books from 1 April 2025. It was used as the #692 NONINT row left it, loaded alone and shut again at the end. Step 1 read M0 215, V0 3 and 8 ledgers, equal to that row's last read.
- The tool was the `bridge_mcp.exe` of master 609776ae, sha256 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44 before the first call and after the last (bridge-tally 0.6.0). It ran over stdio in one launch with a new empty data folder and terms true; writes, import, redaction and max bytes were unset. The server exited 0 with an empty error stream.
- Tally's Trial Balance was opened from Display More Reports and viewed only. F5 switched to the ledger-wise view and back, and the report was left with Ctrl+Q. No dialog appeared.

## Not measured
The tax-audit book; the Profit and Loss or Balance Sheet (not read again); any other window of dates; a book with integration on.

## Files
- `tool-answers/`: `initialize.json` and every answer whole: 000 tools/list, 001 list_companies, 002 tally_status, 003 m0, 004 trial_balance, 005 m1.
- `client-log.txt`: each call with its time and saved sha256. `notes.txt`: one line per step, in UTC.
- `SCREENSHOTS.txt`: the screen captures by name, bytes and sha256 (they stay on the lab PC).
