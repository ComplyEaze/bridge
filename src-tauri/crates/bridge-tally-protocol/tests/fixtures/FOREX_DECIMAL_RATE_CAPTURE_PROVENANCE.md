# The `forex_decimal_rate` captures: provenance

The ledger snapshot and the native Trial Balance with currency of `BRIDGE FOREXGROUP LAB`, as
TallyPrime 7.2 wrote them after dollar purchases and payments were keyed (bridge#2071). Some
composites carry a rate with decimals, although every rate keyed on screen was a whole number.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)** 7.2, `education_mode` false, one loaded
  company.
- **Company:** `BRIDGE FOREXGROUP LAB`, the synthetic book of
  `FOREX_72_UNSPACED_CAPTURE_PROVENANCE.md`. Since that capture, the lab had keyed:
  - a rupee customer with a part-settled rupee bill;
  - a dollar supplier with two purchases at whole-number rates;
  - a payment for each purchase, at whole-number rates;
  - a dollar part receipt against the first sale, at a whole-number rate.
- **Date:** 2026-10-30, 20:11:57 and 20:11:59 UTC. Each request was sent once with curl, and the answer was
  kept byte for byte. Source: `raw/b2-ledger-snapshot.response` and `raw/b2-tb-currency.response`
  on branch `lab/2071-forex-reads-answers` at `ea28bde038b3`. Each sha256 equals the lab's own
  `SHA256.txt` line.
- **Requests:** the bytes Bridge's own builders emit, for 1 to 30 April 2025: the ledger snapshot
  that `outstandings` sends (`render_native_ledger_snapshot_request`), and the Trial Balance the
  tool sends for a several-currency book (`render_native_trial_balance_request_with_currency`).
- **Encoding:** BOM-less UTF-16LE, the undecoded wire bytes. The merge-gate manifest lists both
  `native/` paths below (bridge#2216).

| file | bytes | sha256 |
|---|---|---|
| `native/ledger_snapshot_forex_decimal_rate.utf16le.xml` | 13,354 | `4c3796dff24e940f7b30348f6250fc1a440fa591317a7fb51fcdef71634efb87` |
| `native/trial_balance_forex_decimal_rate.utf16le.xml` | 14,140 | `1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7` |

## What the bytes show (one book, one sitting: PARTIAL)

- **The ledger snapshot:** one balance carries a rate with four decimals, the Profit & Loss
  account's. Every other composite balance carries a whole or empty rate. The rupee customer and
  Cash are plain.
- **The Trial Balance:** the dollar customer's debit total carries a rate with four decimals, and
  the dollar supplier's credit total one with two. Both of those rows also hold composites with
  whole or empty rates.
- **Every composite is in the 7.2 layout:** the foreign symbol is against its digits, and there is
  one space before ` = `.
