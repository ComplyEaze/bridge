# The `forexgroup_first_day` and `movement-forexgroup` captures: provenance

What `ledger_movement` reads from a book with several Currency masters, from the book's first day,
as TallyPrime 7.2 answered (bridge#683; TALLY_PROTOCOL_REFERENCE §8.2n): the opening export at that
day, the currency-scoped Trial Balance for the window and the window's vouchers, book-scoped and on
one dollar ledger. Each was read with no edit to the book, except the dollar opening and the two later
vouchers below, each keyed behind a backup and restored after.

## Provenance

- **Host / gateway:** TallyPrime 7.2, licensed, `tally.exe` file version 1.1.7.2; the book loaded
  alone, in a sitting of its own for each lab row.
- **Company:** `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`; synthetic, with
  invented masters). Base INR (Currency master NAME `₹`), plus a `$` master; every ledger's
  `CURRENCYNAME` is `₹`. Its eleven vouchers are all dated 1 April 2025, some entered in dollars at
  more than one rate. Books from 1 April 2025.
- **Requests:** the bytes master's own builders emit, sent one at a time, each answered with STATUS 1
  and kept byte for byte: the movement's ledger export with `SVFROMDATE` 20250401, the Trial Balance
  `trial_balance` sends for a several-currency book (1 to 30 April 2025), and `ledger_movement`'s
  window for 1 April 2025, book-scoped and scoped to `BRIDGE USD Customer`.
- **Sources** (branch, commit, file):
  - `lab/683-lm-answers` at `f832eb17aa41`, run 9 Nov 2026 20:41 to 20:46 UTC:
    `answers/a-lx1-X-lx1-20250401.xml`, `answers/a-mv-V-mv-20250401.xml`,
    `answers/a-mvl-V-mvl-usd-customer-20250401.xml`;
  - `lab/683-tb-answers` at `154a0ca3f375`, run 9 Nov 2026 21:36 to 21:40 UTC:
    `answers/a-tb1-T-tb1-20250401-20250430.xml`;
  - `lab/683-op-answers` at `fb7125adc076`, run 9 Nov 2026 22:27 to 22:45 UTC: a new ledger under
    Sundry Debtors, `BRIDGE USD Opening`, given a dollar opening on its master screen at the rate
    Tally offered, behind a backup restored after. `answers/a-x0-X-lx1-20250401.xml` and
    `answers/a-t0-T-tb1-20250401-20250430.xml`, read before the write, are byte-equal to the first and
    third files below; `answers/a-x1-X-lx1-20250401.xml` and
    `answers/a-t1-T-tb1-20250401-20250430.xml` were read after it.
- **After later vouchers:** `lab/683-ld-answers` at `e38d2b828b5e`, run 10 Nov 2026 00:54 to 01:22 UTC:
  two vouchers dated 30 April 2025 keyed behind a backup restored after (a Sales in dollars on
  `BRIDGE USD Customer` and a rupee Receipt from `BRIDGE INR Customer`), which moved Tally's current
  date to 30 April; then the Trial Balance for 1 April alone, the request whose answer in row 683-TW
  (below) is byte-equal to the third file: `answers/a-tb3-T-tb3-20250401-20250401.xml`.
- **Corroboration:** `lab/683-tw-answers` at `4f31b9259112` (9 Nov 2026, 23:36 to 23:40 UTC) read the
  same Trial Balance to 1 April 2025 and to 31 March 2026, and the book-scoped window to 31 March
  2026: each answer is byte-equal to the third and fifth files below. No file of that row is
  committed.
- **Encoding:** BOM-less UTF-16LE, the undecoded wire bytes. Merge-gate manifest entries: the seven
  paths below.

| file | bytes | sha256 |
|---|---|---|
| `native/ledgers_native_forexgroup_first_day.utf16le.xml` | 11,556 | `898ad76bf159ec8676deaefe4cf2a01672e297de5945bab4c924365705ccace3` |
| `native/ledgers_native_forexgroup_usd_opening_first_day.utf16le.xml` | 12,678 | `fc69af7076469b26ad111eac385e7ab875951c6527ff1bbc9102a6d12e2d8962` |
| `native/trial_balance_currency_forexgroup_first_day.utf16le.xml` | 14,140 | `1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7` |
| `native/trial_balance_currency_forexgroup_usd_opening_first_day.utf16le.xml` | 15,526 | `13ae13a58f8867e80070a3a5aa43eeabce8fc25cf1c2d225c66620de8617c283` |
| `native/trial_balance_currency_forexgroup_first_day_all_plain.utf16le.xml` | 13,528 | `a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705` |
| `agent/movement-forexgroup-book-20250401.utf16le.xml` | 59,524 | `56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7` |
| `agent/movement-forexgroup-usd-customer-20250401.utf16le.xml` | 23,720 | `bff19938fc345fadb76af9e2b1924309976837475e1d3777e875b5ccd6349443` |

## What the bytes show (one book, one release: PARTIAL)

- **The opening export** names eight ledgers, each opening at `0.00`. With the dollar opening keyed,
  it names nine, and `BRIDGE USD Opening` opens with a composite, which the ordinary parse refuses
  and only the first-day parse keeps as text.
- **The Trial Balance** names the same eight ledgers. Six carry a composite in at least one value and
  are set aside as `mixed_currency_movement`; the two plain rows, `BRIDGE INR Customer` and `Cash`,
  each close at `-3000.00` from a zero opening. With the dollar opening keyed, `BRIDGE USD Opening`
  is set aside too.
- **The Trial Balance after later vouchers** names the same eight ledgers, all plain: each value the
  first Trial Balance gave as a composite reads as its rupee figure alone, and every rupee figure is
  the same. Whether the later vouchers or the moved current date changed it is not established. The
  movement's own reads keep their composites in that state (the row's window to 31 March 2026
  answered the eleven 1 April vouchers entry by entry as before), so they check the Trial Balance.
- **The book-scoped window** holds the eleven vouchers, none cancelled or optional. Eight carry two
  composite entries each, on five ledgers, and three are plain; the plain entries move each of the
  two plain ledgers by 3,000.00 to the debit, which ties to the Trial Balance. Every bill allocation
  is amount-only (no `BILLTYPE` or `NAME`, which the window's fetch does not name) or empty. The
  `vouchers` capture of the same day (`agent/vouchers-forex-unspaced-20250401.utf16le.xml`) holds
  the same ten allocations, voucher by voucher, with the same amounts, each typed `New Ref` or
  `Agst Ref`.
- **The ledger-scoped window** holds the four vouchers with an entry on `BRIDGE USD Customer`, each of
  those entries a composite.

The base the tests classify against is identified from the 29 Oct 2026 currency reads of the same
book (`FOREX_72_UNSPACED_CAPTURE_PROVENANCE.md`), as Bridge identifies it. Not shown by these bytes:
entries on more than one day, a ledger whose `CURRENCYNAME` is not the base, a second foreign
currency, and any release but 7.2.
