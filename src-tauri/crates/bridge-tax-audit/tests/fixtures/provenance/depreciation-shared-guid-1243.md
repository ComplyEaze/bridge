# Fixture provenance: depreciation vouchers that share a GUID (#1243)

Every book here is invented, with plain names and irregular figures: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

The reference changed, at its commit `cf8b4d77`, three places where `depreciation` read a voucher's GUID where it meant the voucher. Before it, a voucher was a depreciation journal when any voucher holding its GUID carried a depreciation-expense line, so an asset sale sharing the journal's GUID (a blank GUID, or one GUID repeated) was read as book depreciation and not as a deletion; the same-day search for a cash payment to an addition's supplier skipped every voucher holding the addition's GUID, so a payment sharing it was missed, and with it the s.43(1) second-proviso row, its finding and the figure applying the proviso; and a cash-flagged addition's row id, a hash of its voucher's GUID and its ledger's tag, repeated for two such additions on one ledger with one GUID, or for two cash-paid lines of one voucher on one ledger (on a book whose GUIDs are all unique too), which stopped the test on a repeated figure id. Now a voucher is a depreciation journal by its own lines; the search skips only the voucher itself; and a row id that would repeat takes its place among the rows sharing it (in the books' order, then line order) as a suffix `_1`, `_2`, while every other id is as before. A citation still names a voucher by its GUID and label, and the client's put-to-use list still names a voucher by GUID, so a date given there reads on every voucher holding it.

The one depreciation golden pinned before this, `golden/synthetic.depreciation.json`, regenerates byte-identical at the commit below (checked 7 Oct 2026). Until now an edge book could not name `depreciation`: `parity/edge_golden.py` and `tests/edge_books.rs` gain its runner with these books (the spec's `depreciation` table, read on the reference's side through its own `tae.config.depreciation_config`).

- `edge-books/dep_unique_guids.json` (the guard): every GUID its own. A depreciation journal crediting two asset ledgers and a sale of one of them; a purchase on credit paid to the supplier in cash the same day; a cash purchase put to use for fewer than 180 days; a purchase by bank. Two rows, neither id suffixed. 21 figures, 3 findings. Byte-identical at `cf8b4d77`'s parent.
- `edge-books/dep_shared_guid_journal.json`: one GUID on a journal crediting Machine A and on a sale of Machine A; two blank GUIDs on a sale of Machine B (read first) and the journal crediting it; a third blank-GUID voucher, a sale of Machine A, whose GUID a journal on another ledger holds. Each sale is a deletion. 17 figures, 1 finding. At the parent the three sales were book depreciation: deletions read 0 for 180,053 paise, book depreciation 285,077 for 105,024, and the Act depreciation, the closing WDV, the book-versus-Act difference and the tie figure all moved.
- `edge-books/dep_shared_guid_cash_payment.json`: one GUID on a purchase on credit and on the same-day cash payment to its supplier; two blank GUIDs on a purchase put to use for fewer than 180 days and its payment; one GUID on a purchase and on a cash payment the next day (not flagged); a purchase paid the same day by two cash payments, one sharing its GUID and one with a GUID of its own (the cash shown is both). 23 figures, 4 findings. At the parent the first two rows and their findings were missing, the third showed the cash of one payment only, and the figure applying the proviso read 900,136 for 528,802.
- `edge-books/dep_shared_guid_rows.json`: one GUID on two cash purchases of Machine A, the second with two lines on it, and a cash purchase with a GUID of its own read between them (the three lines take `_1` to `_3`, the one between keeps its id); three blank-GUID cash purchases of Machine B read in neither date nor amount order (`_1` to `_3` in the books' order), with a blank-GUID purchase by bank among them that is no row and takes no place; the first GUID again on a cash purchase of another ledger (no suffix). Each row and its finding cite the row's own voucher. 27 figures, 9 findings. At the parent the reference stopped on a repeated figure id.
- `edge-books/dep_two_cash_lines.json`: every GUID its own. A cash purchase with two lines on one asset ledger, the larger first (`_1` and `_2` in line order); a cash purchase with a GUID of its own; a cash purchase with one line on each of two ledgers (no suffix). 23 figures, 6 findings. At the parent the reference stopped on a repeated figure id.
- `edge-books/dep_shared_guid_put_to_use.json` (a second guard): a put-to-use date given for a GUID two purchases hold, and one for the blank GUID two purchases hold (each date reads on both vouchers: all four at half rate); a purchase with a GUID of its own; a listed GUID no voucher holds. 17 figures, 1 finding. Byte-identical at `cf8b4d77`'s parent: the list stays keyed by GUID.
- Not reached by a book: a GUID holding a NUL that would make two voucher keys equal, which the reference refuses (no read produces one); it is a unit test in `src/depreciation.rs`, beside the one in `src/book.rs`. Nor what the reference's walk keeps per ledger for its own callers (the vouchers of each ledger's credited depreciation, now by voucher key), which its `run()` does not read and the crate does not carry.
- Every book maps its asset ledgers to one block, `plant_machinery_15`; a second block is in the synthetic read's golden.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally.

## How they were produced

At the reference engine (a private repository), commit `ee17d80fe60e6d2734629aab7dc23ff0c3dc348d` (its `depreciation` is that of `cf8b4d77`: no later commit touches the module), from an archive of its `tae/` only, under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The same invocation against an archive of `cf8b4d77`'s parent is the control the lines above report. The six books are written as data by a small generator, every voucher balancing and every Trial Balance whole (each ledger's row follows from its opening and its voucher lines); they are hand-chosen scenarios, not generated from any data.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `dep_unique_guids.json` | 4,340 | `45109b48a9334f0effc158eeb36c5da5a9a80d5223b4921404a03a5bb2c03b5e` | `edge-books/dep_unique_guids.json` |
| `dep_shared_guid_journal.json` | 3,238 | `b6dfb9e3a8b8b3bbdef0809fdb69bd72a7917adf596eb5d780ed6f6b9518389e` | `edge-books/dep_shared_guid_journal.json` |
| `dep_shared_guid_cash_payment.json` | 5,615 | `0aed7bc1052114fe783e6d8e2d3e518a52e0081a348745c8a2f73ee2e4cd9378` | `edge-books/dep_shared_guid_cash_payment.json` |
| `dep_shared_guid_rows.json` | 4,345 | `e44234c7ab794577d33f4f70cb7fbb5018cb7f85c06dc47436972f5d8d1a2349` | `edge-books/dep_shared_guid_rows.json` |
| `dep_two_cash_lines.json` | 2,617 | `3b8b7264ece2b4b9383b035c1204ae35ff814ba6a7d573799bfb09e3dd31f781` | `edge-books/dep_two_cash_lines.json` |
| `dep_shared_guid_put_to_use.json` | 3,203 | `ba8cc4228907b1b18c1d31b0123fe60003160f653f7a7eee81bfacae03a46708` | `edge-books/dep_shared_guid_put_to_use.json` |
| `edge.dep_unique_guids.depreciation.json` | 16,999 | `8bd1439be1ca49fe8a2035d60cd55dbedf8734b9e42e7a8f69d5fcaece910542` | `golden/edge.dep_unique_guids.depreciation.json` |
| `edge.dep_shared_guid_journal.depreciation.json` | 10,832 | `5b3a04761cfc66ed704bde01b195a2829ab84d62723e9b0748e1525d348e28c5` | `golden/edge.dep_shared_guid_journal.depreciation.json` |
| `edge.dep_shared_guid_cash_payment.depreciation.json` | 20,685 | `a06279c1150e19057b5df37eaf82f2046db7ab96f2a3951665a5b682bd27150b` | `golden/edge.dep_shared_guid_cash_payment.depreciation.json` |
| `edge.dep_shared_guid_rows.depreciation.json` | 30,267 | `f6d0ded4afb866397d0dc8f2786fd02a86680c1cd8f3dc2b9c63275d2fadfd1e` | `golden/edge.dep_shared_guid_rows.depreciation.json` |
| `edge.dep_two_cash_lines.depreciation.json` | 22,173 | `2c0962e4eda8f271118e647ece06ff6889b29366cfa60afd788d03037ffc62e2` | `golden/edge.dep_two_cash_lines.depreciation.json` |
| `edge.dep_shared_guid_put_to_use.depreciation.json` | 10,611 | `e36b509f5b981f06aadefdb2be62ce125dffb0f7a5d5249d99ab6cc3365233db` | `golden/edge.dep_shared_guid_put_to_use.depreciation.json` |
