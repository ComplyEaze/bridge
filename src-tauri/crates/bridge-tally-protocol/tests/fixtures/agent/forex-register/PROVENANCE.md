# `forex-register`: provenance

Tally's answers to the requests one `sales_register` call sent for one day of the synthetic
several-currency lab book on TallyPrime 7.2, and the marks answer the `vouchers` call of the same
run sent (#683).

## Capture

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.2, one loaded company.
- **Date:** 2026-11-04 (the lab PC's date), through a relay that kept every request and answer as sent and refused
  any import. Source: `lab/683-register-answers/relay/exchange-N.request` and `.answer` on
  branch `lab/683-register-answers` at `32f896c7`. The build was master at `75224fe3`.
- **Company:** `BRIDGE FOREXGROUP LAB`, a synthetic lab book, GUID
  `1bb48be5-baba-45b2-90d8-7593244915a9`. Base currency master NAME `₹`, plus a `$` master. Its
  marks were byte-identical before and after the run.
- **Window:** 2025-04-01 to 2025-04-01, the day of `vouchers-forex-unspaced-20250401`.
- **Answers:** each file is the answer's body, BOM-less UTF-16LE, as received, except that in
  the extent and book-extent answers (exchanges 49 and 54) the lab's copy has `redacted` in
  place of the `COMPANYNUMBER` value; the lab redacts company numbers on purpose (their kept
  headers declare two bytes fewer, the length before the redaction). Bridge reads
  that field, so the replay tests (`with_synthetic_company_number` in
  `src/agent_register_server_tests.rs`) put the synthetic value ` 1` there when they load these
  two answers, as #2314's test does for this book's extent. Every other byte is served as
  captured.
- **Requests:** `purchase_register` and `gst_by_counter_ledger` sent the same requests, in the
  same order, and got the same answers (exchanges 113 to 176 and 177 to 240). Every repeated
  request in the run had a byte-identical answer.
- **Answers already committed.** Five answers of exchanges 49 to 101 are byte-identical to
  fixtures committed from the same book, and are not added again: the currencies (exchange 58,
  `native/currency_masters_forex_72`), the currencies with original names (62,
  `native/currency_masters_originalname_forex_72`), the base currency (66,
  `native/company_base_currency_forex_72`), the balance listing (86,
  `native/ledger_snapshot_forex_decimal_rate`) and the groups (90,
  `native/group_snapshot_forexgroup_lab`). Their requests' sha256 are `df262192…`, `0f10c96e…`,
  `8f620533…`, `71e429b8…` and `205458f1…`.

| file | exchange | bytes | sha256 | request sha256 |
|---|---|---|---|---|
| `forex-register-extent.utf16le.xml` | 49 | 4022 | `4d633cf38f2c16b20bb4d6a3e7d2f34795c3a54c8e75a4ef36f5eaf64b26eaeb` | `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` |
| `forex-register-book-extent.utf16le.xml` | 54 | 3858 | `76911fc5b7b0fdebbace31d0d7721225a86ac3af70a2ca2120b6e975014b562d` | `017dd78479ff02e4aae52d7c372da34f061ceb676b7272c0f34a44f72248cd62` |
| `forex-register-ledgers-master.utf16le.xml` | 82 | 19456 | `3460bd62e2db1ae65d90d1e1bc178bf61a926eeed55202fe0d9c7d4f153aa7d8` | `b20e80b785c040a31c70051dab8ded0af0a7e5bdcff479a0ed78e8e5c3d2abb4` |
| `forex-register-census.utf16le.xml` | 102 | 29456 | `817075d4b105ce5fe2879f4f9a50037f9977ce9e5c4b857db11065d34e786472` | `12e97cc3a11334e443568e1d5a956718d429f1afce590246dc0cb0b1f3bf853c` |
| `forex-register-window.utf16le.xml` | 108 | 404502 | `c87866eb0db4d4b1622caf2d69d6a24e03efc1f6f80afef499852fd78cd7d771` | `ab78fbe5afeb6734a3ec36a940dc098371560faf0a1210ca7e511927ee3dd2f5` |
| `forex-register-marks.utf16le.xml` | 10 | 3404 | `3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba` | `5b447d03d63414bb46753503faee9b105347e13c28cac548bfd51d6fdb9fb4b7` |

## What it establishes

- **The class-entry window** (exchange 108) holds the day's eleven vouchers with the same
  `AMOUNT` and `VATEXPAMOUNT` texts as `vouchers-forex-unspaced-20250401`, 53 of 53, plus the
  voucher-type class computes on every voucher. Composites sit in 16 entry `AMOUNT`s, 8
  `BILLALLOCATIONS.LIST/AMOUNT`s, 4 `BANKALLOCATIONS.LIST/AMOUNT`s and 12 `VATEXPAMOUNT`s.
- **The refusal before #683's register change.** All three tools refused with
  `bill_allocation_amount_invalid` and sent no marks read: the window parse judges a typed bill
  allocation's amount when its list closes, inside the entry, before the entry's own amount.
- **The masters.** The master answer has 8 ledger rows; the balance answer has 8, all in the
  base currency, 6 of them with a composite opening or closing, which the compliance read sets
  aside.

## Limits

- One release (7.2), Silver, one book, one day, one foreign currency (`$`). The base read after
  the window, the marks and the second masters read that a register sends now were not sent by
  this run's register calls, which refused at the window: their answers here are the answers
  the same run gave to the same requests (the base read's legs in the `vouchers` call, exchanges
  27 to 48; the marks in exchange 10; the masters in exchanges 49 to 101). Partial.
