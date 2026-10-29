# Results: #2071 forex rerun, 29 Oct 2026, BUILD M (master af4a0dbf, exe 9f73d533…, which holds #2071's change), on BRIDGE FOREXGROUP LAB (100090) with X1's four vouchers keyed again. The three reads refused as on X1, and the Trial Balance refusal now names its cause, **`trial_balance_amount_invalid`**. Tally's raw answers show why each read refused: the Trial Balance and ledger amounts are sent as text with a currency, a rate and an `=` sign, one of them with no rate; and the payable bill row `{payable, 1}` is bill 2 with an empty closing amount

The sheet is `lab/2071-forex-rerun` at 52b3b19af43ab3765d8e480ee631f35d266e2976.
- `SHA256.txt` matched 7 of 7.
- The census printed `ok 4, skipped 2, FAIL 0`.
- X1's BK1 is present and equal to the sheet.

**Build.** BUILD M is master af4a0dbf7f7d6b9432b2b6f0994c217c69c8279e: run 40671680104, artifact 12809546869.
- `bridge_mcp.exe` 9f73d533dcac8a24d1d33389a7db33cf4055e212f7dc97f8c170f00c08b73fd1, equal to the smoke file and the same at the end.
- #2071's commit 1ea2792f is on master as 24d9cbb46 (#2080), an ancestor of af4a0dbf.
- The tool reports version 0.6.1 and no commit.

**Tally.** TallyPrime 7.2 (Latest), Silver Rental, licensed, `education_mode` false.

**Company.** GUID 1bb48be5-baba-45b2-90d8-7593244915a9.

**Redaction.** Off.

**Marks.**

| read | ALTMSTID | ALTVCHID |
|---|---|---|
| M0 (raw) | 211 | absent |
| tool, before keying | 211 | null |
| M1 (raw) | 211 | 4 (the four vouchers) |

## Keying (step 2)
K1 to K4 were keyed as the sheet lists them, each dated 1-Apr-25. Every screen and rate box was the expected one:
- **K1:** ₹1/$ was offered; 86 was keyed.
- **K2:** ₹86/$ was offered; 86.4125 was keyed.
- **K3:** ₹86.4125/$ was offered; 87 was keyed. Agst Ref, bill 2. The bank line got its own rate box, then Bank Allocations. There was no gain or loss line, and the totals were ₹8,700.00 on each side.
- **K4:** an empty Inventory Allocations box opened.

Each voucher's screen was captured before Ctrl+A.

## The three built reads (step 3)
| read | code | cause | remediation | bill_row | evidence bytes | response_sha256 |
|---|---|---|---|---|---|---|
| `trial_balance` 2025-04-01 to 30 | `trial_balance_read_failed` | **`trial_balance_amount_invalid`** (X1: none) | "No Trial Balance figures were returned for this window. Tell the user what the cause says … no figure was derived. Do not repeat this same read in a loop …" | none | 54,349 | c83fd64c… |
| `outstandings` as of 2025-04-30 | `native_outstandings_read_failed` | `native_amount_invalid` | "A bill amount in Tally's Bills report could not be read exactly, so no figures were returned. …" | `{payable, 1}` | 168,122 | b8718ca5… |
| `ledger_masters` compliance | `party_ledger_master_read_failed` | `native_foreign_currency_ledger_balance` | none | none | 80,013 | 718666f1… |

The sizes equal X1's. Codes, causes and `bill_row` are as on X1, except the Trial Balance's cause.

## Raw answers (step 4)
Every curl exited 0, and every CONTENT-LENGTH equals the size.

| request | status, bytes | sha256 | bound |
|---|---|---|---|
| tb-currency | 200, 9,880 | 263c3c19… | 54,349 |
| bills-receivable | 200, 500 | 52114c9c… | 168,122 |
| bills-payable | 200, 476 | b5a4bd6e… | 168,122 |
| ledger-snapshot | 200, 9,472 | 4d351a9f… | 168,122 |
| ledger-snapshot-compliance | 200, 9,472 | 4d351a9f… (the same bytes as ledger-snapshot) | 80,013 |
| m0 / m1 | 200, 3,314 / 3,404 | d9dd3489… / 0d5b14b1… | none |

## What Tally sent (step 5, read by an XML parser)
**Trial Balance (`tb-currency`).** TBALOPENING is `0.00` on every row, and CURRENCYNAME is `₹` on every row.

| ledger | DEBITTOTALS | CREDITTOTALS | TBALCLOSING |
|---|---|---|---|
| BRIDGE Export Sales | empty | `$1000100.00 @ ₹ 86/$ = ₹ 86009641.25` | `$1000100.00 @ ₹ 86/$ = ₹ 86009641.25` |
| BRIDGE Lab Bank | `-$100.00 @ ₹ 87/$ = -₹ 8700.00` | empty | `-$100.00 @ ₹ 87/$ = -₹ 8700.00` |
| BRIDGE USD Customer | `-$1000100.00 @ ₹ 87/$ = -₹ 87008700.00` | `$100.00 @ ₹ 87/$ = ₹ 8700.00` | `-$1000000.00 @ ₹ 87/$ = -₹ 87000000.00` |
| Cash | `-1000.00` | empty | `-1000.00` |
| Profit & Loss A/c | empty | empty | `$0.00 @ ₹ /$ = ₹ 0.00` (no rate) |

What these amounts show:
- The rupee journal of K4 (₹1,000) is folded into Export Sales' dollar amount at ₹86: 86,009,641.25 = 86,000,000 + 8,641.25 + 1,000.
- The party's totals are revalued at ₹87.

The book holds two currency masters, `$` and `₹` (Tally's List of Currencies).

**Bills.**
- `bills-receivable` has 1 row: bill 1, BRIDGE USD Customer, BILLCL `-87000000.00` (that is, $10,00,000 at ₹87), due 1-Apr-25, overdue 29.
- `bills-payable` has 1 row: **bill 2**, BRIDGE USD Customer, **BILLCL empty**, due 1-Apr-25, overdue 29.

The payable row 1 named by `bill_row` is therefore the settled bill 2, sent with an empty closing amount.

**Ledger snapshot** (both requests gave the same bytes). BRIDGECOMPANYGUID is on every row.
- CLOSINGBALANCE is as in the Trial Balance's TBALCLOSING, except Profit & Loss A/c, which reads `$1000100.00 @ ₹ 86/$ = ₹ 86009641.25`.
- OPENINGBALANCE is `0.00` on every row.
- ISBILLWISEON is Yes only for the party.
- CURRENCYNAME is `₹` on every row.

**Party ledger on screen, April 2025.**
- Sales 1, ₹8,60,00,000.00 Dr;
- Sales 2, ₹8,641.25 Dr;
- Receipt 1, ₹8,700.00 Cr;
- on 30-Apr-25, "Unadjusted Forex Gain/Loss", ₹10,00,058.75 Dr. This is (87 − 86) × $10,00,000 plus (87 − 86.4125) × $100.
- Closing balance ₹8,70,00,000.00 Dr.

**After.** The company is shut and kept with the four vouchers. BK1 was not restored.

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `answers/`: every tool answer whole, with UTC times.
- `client-log.txt`.
- `notes.txt` (UTC).
- `raw/`:
  - every raw answer byte for byte, with its headers;
  - `log.txt`;
  - each answer's parsed tree (`*.tree.txt`).
- `SCREENSHOTS.txt`: the captures by sha256. They stay on the lab PC.
