# Results: X1 (#683), built trial_balance and outstandings on a book with foreign-currency amounts, at two digit-grouping settings, TallyPrime 7.2 Silver, 28 Oct 2026 (UTC)

The sheet is `lab/sitting3b-run-sheets/X1-683-forex-digit-grouping.md` at dbf5b2d9 (SHA256.txt 3 of 3 equal). The company is BRIDGE FOREXGROUP LAB, new and synthetic: number 100090, GUID 1bb48be5-baba-45b2-90d8-7593244915a9, books from 1 April 2025, base currency INR. It holds one foreign Currency master, USD ($, 2 decimals). The build is w1-2306d86d (bridge_mcp.exe sha256 92a899a2...9524), run over stdio with posting off.

## FINDINGS (issue material)
1. **trial_balance refused with no cause and no remediation** (a refusal must name what to do next). On 1-30 April 2025 it returned `trial_balance_read_failed` with no `cause` and no `remediation`. The code was the same at both settings and in the control (answers/006, 008, 010).
2. **ledger_masters compliance refused with `native_foreign_currency_ledger_balance`** (`party_ledger_master_read_failed`) once the book held USD vouchers (answers/005). The basic read was refused before that with `company_several_currency_masters`, as the README expects (answers/001). Compliance answered before the keying (answers/002) and again after the restore (answers/012).
3. **outstandings refused `native_amount_invalid` naming a PAYABLE row although the only party is a debtor.** As of 30 April 2025 it returned `native_outstandings_read_failed`, cause `native_amount_invalid`, `bill_row {report: payable, row: 1}`, with the remediation "Do not retry". Code, cause and row were identical at both settings (answers/007, 009).

## The four tool answers (step 2 at the Indian default, step 4 at the million style)
| read | Indian default | million style | control (step 5) |
|---|---|---|---|
| trial_balance 2025-04-01..30 | refused, trial_balance_read_failed, no cause | the same | the same |
| outstandings as of 2025-04-30 | refused, native_outstandings_read_failed / native_amount_invalid / payable row 1 | the same | not called |

- **Figures:** no tool returned a figure, so no base amount was returned for the party carrying a USD amount.
- **Codes:** the codes did not differ between the two settings, and the control equals step 2 in code.

## What the screen shows (7.2), recorded
- **Where the digit-grouping setting lives:** it is per currency. Path: Alter > Currency > Rs > "Show amount in millions" (No, then Yes, then No). Before, after and back captures are listed. Each change moved the master mark by one (211 > 212 > 213).
- **The party's ledger display** for 1 April 2025 at both settings:
  - Sales 1 Debit 8,60,00,000.00 (shown as 86,000,000.00 in millions)
  - Sales 2 8,641.25
  - Receipt 1 Credit 8,700.00
  - a line "Unadjusted Forex Gain/Loss" Debit 10,00,058.75 (1,000,058.75)
  - Closing Balance 8,70,00,000.00 (87,000,000.00)
- **The forex amount on the voucher** is shown in lakh grouping before the setting change (K1 `$10,00,000.00`).
- **No exchange gain/loss line on the Receipt itself** (Rs 8,700.00 Dr = Rs 8,700.00 Cr), although bill 2 was raised at 86.4125 (Rs 8,641.25) and settled at 87 (difference Rs 58.75).
- **Pending Bills revaluation:** Pending Bills showed bill 1's Final Balance revalued at the latest rate, 86.4125 (Rs 8,64,12,500.00 against Rs 8,60,00,000.00 at 86).
- **Ledger and voucher screens:** Ledger Creation shows no currency-of-ledger field, so the party is a rupee ledger and the USD amount is keyed on the voucher line through Tally's "Forex Rate of Exchange" box.

## Keying (synthetic, on Tally's screen; choices the sheet left open are marked)
All four vouchers are dated 1-Apr-25.
- **K1 Sales:** BRIDGE USD Customer, BRIDGE Export Sales $1,000,000.00 @ 86 = Rs 8,60,00,000.00, bill New Ref 1.
- **K2 Sales:** $100.00 @ 86.4125 = Rs 8,641.25, bill New Ref 2.
- **K3 Receipt:** BRIDGE USD Customer $100.00 @ 87 = Rs 8,700.00, settled Agst Ref bill 2 (the bill was chosen at the stop, as the sheet names none). Dr BRIDGE Lab Bank $100.00 @ 87 with Bank Allocations defaults.
- **K4 Journal:** Dr Cash 1,000.00 / Cr BRIDGE Export Sales 1,000.00 (the ledgers were chosen at the stop, as the sheet names none). The Inventory Allocations sub-screen was closed with no item.
- **Sales sub-screens:** Party Details (Dispatch blank; the buyer box with the party's defaults) was passed with Enter only.

## Marks
| point | how | M | V |
|---|---|---|---|
| before keying | ledger_masters compliance | 211 | none |
| before step 3 (after keying) | raw MARKS read (below) | 211 | 4 |
| after step 3 | raw | 212 | 4 |
| after steps 4-5 | raw | 213 | 4 |
| after the restore of BK1 | ledger_masters compliance, and raw | 211 | none |

- **Voucher mark:** it did not move between steps 2 and 6. The master mark moved only by the two setting changes.
- **The raw MARKS request** is a recorded deviation: a byte copy of a committed company-marks request with only `SVCURRENTCOMPANY` changed to this company. The decoded diff shows that element only: 982 B sha256 f16573e6... became 986 B sha256 5b447d03.... The request-ID check gave census ok 1. Each read was 200 and under the 8,192 B bound.

## Control and book state
- **BK1** (before the first voucher): `TDBK1800_100090.001`, 24,336 B, sha256 df7b319f7cb874d6426142c97b8ec2b6d7a753ec7455fce66219e333c60b7cef. It was restored after all reads, with Replace on Tally's "folder already exists" prompt. It stays on the lab PC.
- **The book is left as BK1 holds it** (masters only, no vouchers), shut. C10: no company loaded, one tally.exe holds 9001, /status 200.

## PASS / FAIL
**PASS, with the three findings above.**
- **PASS conditions met:**
  - every tool answer was saved whole and described
  - the setting's screen path was recorded
  - the voucher mark was unchanged, and the master mark moved only by the recorded setting changes
  - the control equals step 2
- **The three findings:** these are refusals, not wrong figures, and the codes did not differ between settings.

## Files
- `answers/`: every tool answer, and `client-log.txt`.
- `rawmarks/`: the request copy, the four MARKS answers with headers, and `send-log.txt`.
- `notes.txt`: UTC, one line per step.
- `SCREENSHOTS.txt`: every capture and BK1, by name, bytes and sha256. They stay on the lab PC.
