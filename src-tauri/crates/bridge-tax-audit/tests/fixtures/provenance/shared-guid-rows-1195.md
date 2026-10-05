# Fixture provenance: rows that share a GUID (#1195)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

The reference changed, at its commit `976ba9dd`, how four modules cite the vouchers behind a row. Before it, `cash_payments_40a3` and `high_value_register` keyed a row's vouchers by GUID alone, so two vouchers sharing a GUID (a blank GUID, or one GUID repeated) in one row were cited as one voucher; and `counter_cheques_40a3` gave two over-limit lines with the same date, GUID and ledger the same row id, so the reference refused the whole test with a duplicate figure id. Now a row keys its vouchers by GUID and label (the evidence ref's id stays the GUID), `entity_269st_gap` projects back to one unlabelled ref per distinct GUID so its output does not move, and `counter_cheques_40a3` adds an ordinal suffix only where today's id would repeat. The figure values, every id that was unique before and every other finding are unchanged: the 130 goldens already pinned regenerate byte-identical at `976ba9dd` (checked 5 Oct 2026).

- `edge-books/cp_shared_guid_row.json` (`cash_payments_40a3`): two blank-GUID cash payments to one payee on one day, told apart by their numbers (an s.40A(3) row and an s.269ST payment-leg party row); two blank-GUID receipts from one party on one day; two blank-GUID walk-in sales and two blank-GUID cash purchases on one day (the unnamed rows of each leg); one GUID on two payments to one payee on one day; two blank-GUID payments with no number (identical refs, so cited once); a unique GUID with two lines to one payee (cited once); two blank-GUID cash drawings to a capital ledger (an excluded row, cited in its excluded total). 32 figures, 9 findings.
- `edge-books/hvr_shared_guid_row.json` (`high_value_register`): two blank-GUID receipts from one party on one day (a party-day row; the voucher-grain row keys by GUID and stays one row); two blank-GUID cash purchases on one day; two blank-GUID receipts on two days sharing a reference (a single-transaction row), beside a blank-GUID receipt with no reference (the unreferenced count is of distinct GUIDs and does not move) and a unique-GUID one. 52 figures, 6 findings.
- `edge-books/ep_shared_guid_row.json` (`entity_269st_gap`): two blank-GUID receipts on one ledger and a unique-GUID receipt on a second ledger with the same PAN, on one day. The gap row still cites one unlabelled ref per distinct GUID; at the reference's previous commit the golden is byte-identical. 7 figures, 1 finding.
- `edge-books/cc_repeat_row.json` (`counter_cheques_40a3`): one voucher with two over-limit lines to one expense ledger, and two blank-GUID over-limit payments to that ledger on one day; each takes `_1`, `_2` in the rows' sort order, and a unique-GUID payment on the same day and a line on another ledger keep today's id. At the reference's previous commit the reference refuses this book (`duplicate figure id`), so it has no pristine golden. 24 figures, 6 findings.
- Not reached: `tds_payees` and the read-only suspects, where a shared GUID merges figure VALUES (tracked as #1243, not changed here).
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `976ba9dd`, under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The four books are hand-written scenarios, not generated from any data.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `cp_shared_guid_row.json` | 5,686 | `89259c83d1d9a899b564fcece5891b9b561cc05cab72746a2aa132e0a09d0283` | `edge-books/cp_shared_guid_row.json` |
| `hvr_shared_guid_row.json` | 3,628 | `f7d3fb3e67bdb07ffe6d1c681fc7094b5eab4f3636163d89647d1b06cf8a50f7` | `edge-books/hvr_shared_guid_row.json` |
| `ep_shared_guid_row.json` | 2,336 | `a80224594076fd25e6993c9c53be050c0f1b505ad1db3286b768206862a62bed` | `edge-books/ep_shared_guid_row.json` |
| `cc_repeat_row.json` | 2,707 | `d960aa8b1c19c4dad87b5f4528fcc3b6c7321f9ffa2184c90a6e0373df990210` | `edge-books/cc_repeat_row.json` |
| `edge.cp_shared_guid_row.cash_payments_40a3.json` | 37,036 | `7029e2abb4e6ee7eb832d2abed6cb09e4cb2ed012d0fa331a16416364f6ad301` | `golden/edge.cp_shared_guid_row.cash_payments_40a3.json` |
| `edge.hvr_shared_guid_row.high_value_register.json` | 32,123 | `49426fb7304c32da19ca0b8bee24c60035b651f8766d81c471758f889ef2b34a` | `golden/edge.hvr_shared_guid_row.high_value_register.json` |
| `edge.ep_shared_guid_row.entity_269st_gap.json` | 5,484 | `1e1325077d16754239a6097e0544b0ece5b8ea35635e9753cdb3e58e335d9f1e` | `golden/edge.ep_shared_guid_row.entity_269st_gap.json` |
| `edge.cc_repeat_row.counter_cheques_40a3.json` | 20,651 | `7c6c443eff310cd4c79a2ecc4db18661d12a42de48c3f13d13f420ce77dceb52` | `golden/edge.cc_repeat_row.counter_cheques_40a3.json` |
