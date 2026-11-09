# The `forex_unspaced_composite` and `forex_72` captures: provenance

The compliance ledger snapshot and the native Trial Balance with currency of a book that holds
foreign-currency vouchers, as TallyPrime 7.2 wrote them (bridge#683, #2071). They show a composite
amount written with no space after the currency symbol, which the 7.1 captures never show.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)** 7.2, `education_mode` false, one loaded company.
- **Date:** 2026-10-29, between 02:17 and 02:18 UTC. Each request was sent once with curl, and the
  answer kept byte for byte. Source: the lab's raw answers `raw/ledger-snapshot-compliance.response`
  and `raw/tb-currency.response` on branch `lab/2071-forex-rerun-answers` at `0d7eb0bf5cae`.
- **Company:** `BRIDGE FOREXGROUP LAB` (synthetic, created in the lab on 2026-10-28 with invented
  masters and four vouchers keyed by hand). Base INR (currency master NAME `₹`), plus a `$` master.
  Every ledger's `CURRENCYNAME` is `₹`.
- **Requests:** the bytes Bridge's own builders emit, for 1 to 30 April 2025: the compliance ledger
  snapshot (`ledger-snapshot-compliance`; the plain snapshot request returned the same bytes) and the
  Trial Balance the tool sends for a several-currency book (`tb-currency`).
- **Encoding:** BOM-less UTF-16LE, the undecoded wire bytes. Merge-gate manifest entries: the two
  `native/` paths below (pull request for #683).

| file | bytes | sha256 |
|---|---|---|
| `native/ledger_snapshot_forex_unspaced_composite.utf16le.xml` | 9,472 | `4d351a9fc1e8b974fbfb20b06edb19c71f5f664b3a74f2d387759b586092cecb` |
| `native/trial_balance_forex_unspaced_composite.utf16le.xml` | 9,880 | `263c3c19338afe0ed8184398bc472882b9243a66dbc96a10938b16b964e320ac` |
| `native/currency_masters_forex_72.utf16le.xml` | 3,796 | `f5a1414a195194ff4f8b46f4c65cae0e0f7567143c71c6a2c465532334b3439a` |
| `native/currency_masters_originalname_forex_72.utf16le.xml` | 4,000 | `3f0bfbcbb3bc0def49de13a7ac918e5d20d6a1d6885aa0e17635eb5af694aaf0` |
| `native/company_base_currency_forex_72.utf16le.xml` | 3,434 | `52c33f519c9e5ee390d44ef30223ad46f8b3bbbe2c71e02a5c995e0672043df3` |

## What the bytes show (one book, one run: PARTIAL)

The two answers hold twelve composite values, six distinct strings, and no other composite. The
amounts are the book's own and are not quoted here; the kinds are, by example:
- a credit total and a debit total with a rate of two whole digits, such as `-$100.00 @ ₹ 87/$ = -₹ 8700.00`
  (a debit carries a minus on both amounts, a credit on neither);
- the same forms with larger amounts, which differ only in the number of digits;
- a zero total, which has an empty rate: `$0.00 @ ₹ /$ = ₹ 0.00`.

Against the 7.1 captures (`FOREX_601D_CAPTURE_PROVENANCE.md`), the layout differs in two places and
only two:
- the foreign amount's symbol sits directly against its digits (`$100.00`, where 7.1 writes `$ 100.00`);
- there is a single space before ` = ` (7.1 writes two).

The rate is `₹ <whole number>/$` or, for the zero total, `₹ /$`; the base amount keeps a space after
its symbol (`₹ 8700.00`, `-₹ 8700.00`); and the base symbol is `₹` (7.1 writes `I₹`). Not shown by
these bytes: a rate with decimals, a base amount written without the space, a symbol other than `$`
and `₹`, a composite on a voucher (no voucher answer of this book is committed), and any release but
7.2.

## The base-currency reads

The last three files are the two reads Bridge makes first to name the book's base, read on the same
book on 2026-10-29 (branch `lab/683-forex-currency-reads-answers` at `85d7d37257e9`, files
`raw/r1.response`, `raw/r2.response`, `raw/r3.response`, kept byte for byte): the Currency masters
(`render_company_currency_request`), the same with `ORIGINALNAME` fetched
(`render_company_currency_request_with_originalname`) and each loaded company's `CURRENCYNAME`
(`render_company_base_currency_request`). The book holds two Currency masters, `$` and `₹`; the rupee
master's `NAME` and `ORIGINALNAME` are both `₹` (U+20B9), and the company's own `CURRENCYNAME` is
that same `₹`, which is also the `CURRENCYNAME` of each ledger row of the ledger snapshot. The base
the tests classify against is identified from the second and third of these, as Bridge identifies it.
Not shown: any other release.
