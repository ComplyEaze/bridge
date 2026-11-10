# Fixture provenance: financial_statements and the stock basis (#1505)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

A ledger-wise trial balance of a book with integrated inventory leaves out the stock, which Tally holds on the stock items. `financial_statements` now takes the stock from the book's `opening_stock` and `closing_stock` terms on such a book, and shows no gross profit or net profit when a term is not known (#1505). Each book sets the two terms through the edge-book keys `opening_stock` and `closing_stock` and runs `financial_statements` with no partners' interest ledgers and no report totals.

- `edge-books/fs_stock_items.json`: both terms known (opening 800000, closing 300000 paise). Gross profit includes both.
- `edge-books/fs_stock_opening_unknown.json`: the opening stock was not taken. No gross profit or net profit is shown, and the limit names the term and why.
- `edge-books/fs_stock_closing_unknown.json`: no Stock Summary is dated the trial balance's last day. No gross profit or net profit is shown, and the limit names the term and why.
- `edge-books/fs_stock_ledger_integrated.json`: both terms known, and a Stock-in-Hand ledger carries a value. The two are not added, and no profit is shown.
- `edge-books/fs_stock_both_unknown.json`: neither term was read. No profit and no stock figure is shown; the book shows ten figures, the registry's floor.
- Not reached: how `load_book` decides the terms from a read. The crate's unit tests and `tests/read_store.rs` cover that.
- Not established: whether the stock items' values follow the request's window, and whether the audit reads Bridge produces carry a Stock Summary at the period end.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally.

## How they were produced

At the reference engine, commit `6168aeaa` (its change for #1505: the stock basis as this pull request builds it, the third item-basis limit and the several-Summaries reason), with the `parity/edge_golden.py` of the hand-off commit `8ace2d2d`, which adds the `financial_statements` runner and the `closing_stock` spec key:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The books are hand-written scenarios, not generated from any data. The maintainers generated the goldens and handed them over as files.

## Declared behaviour difference (#1602)

The reference reads a book's opening stock as a known zero in two cases, and this crate no longer does (parity-spec §12, ruling of 26 October 2026):

- a stock-items part that parses but lists no stock item: the opening stock is unknown, with the reason "not taken: the read's stock items part lists no stock item; if the company keeps no stock, this is nil";
- a part whose items carry no opening value at all: unknown, with the reason "not taken: no stock item carries an opening value". One item without a value beside valued ones still counts as nothing, and an opening value of 0.00 is a value.

The crate also refuses a total of opening values that does not fit in its paise total ("not taken: the stock items' opening values add up to more than can be held") instead of wrapping it; the reference sums without a bound, so no real book differs there.

No committed golden or edge book reaches either case (edge books set the opening stock directly, and the one committed read has `ISINTEGRATED` set to `No`), so no golden moves and the divergence registry holds no record. The test book is the unit test `book::tests::opening_stock_is_taken_only_for_a_trial_balance_from_the_books_first_day`, on stock-items parts written in the test; it pins the change until a Chartered Accountant's hand-worked answer for such a book is recorded. Not measured: whether Tally omits an opening value of zero, which would make a book with no opening stock read as "no stock item carries an opening value".

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `fs_stock_items.json` | 1,935 | `f837435223a871020b00145a6f30ccc41a92579ec5e43f1820076e149b3b56b3` | `edge-books/fs_stock_items.json` |
| `fs_stock_opening_unknown.json` | 1,997 | `380adf5411318041367b306b9883233da63b9391327e36379bf979cb3b737b36` | `edge-books/fs_stock_opening_unknown.json` |
| `fs_stock_closing_unknown.json` | 2,012 | `e721e3ac83528523b3683521a9b8bff71689e9bc5cff248227bdf3d8367243d8` | `edge-books/fs_stock_closing_unknown.json` |
| `fs_stock_ledger_integrated.json` | 2,259 | `0714902d2c53e8600ab16e78f51fd36c7b774c301d36a68fdaddb234c4d97491` | `edge-books/fs_stock_ledger_integrated.json` |
| `fs_stock_both_unknown.json` | 2,002 | `98b1cd667de0f8d6943e7c7fab465ca6cfb0eab9ba71a65b0e19f96655e9c6c1` | `edge-books/fs_stock_both_unknown.json` |
| `edge.fs_stock_items.financial_statements.json` | 9,721 | `ffebf515231702e28d2d3fae5db9d7a8617f2cd5b8b85292bd93002fcabdf417` | `golden/edge.fs_stock_items.financial_statements.json` |
| `edge.fs_stock_opening_unknown.financial_statements.json` | 8,062 | `6cb134d19672f014d0cf17f6df5ff20ecf48cf61449c86f45ae30b44748ca131` | `golden/edge.fs_stock_opening_unknown.financial_statements.json` |
| `edge.fs_stock_closing_unknown.financial_statements.json` | 7,321 | `261b515b3f98da15b6bb22bd44d86932dd8b2c44bd1fc3793235b6e7f0f68875` | `golden/edge.fs_stock_closing_unknown.financial_statements.json` |
| `edge.fs_stock_ledger_integrated.financial_statements.json` | 8,509 | `85fc85b417999e8e873db5cfa1d24755cf4d7a715081b13d7b49984edcb1e686` | `golden/edge.fs_stock_ledger_integrated.financial_statements.json` |
| `edge.fs_stock_both_unknown.financial_statements.json` | 7,320 | `2ddedd0488e81cb9ccd4859519c2b40622566eb89bbe3d01a0914584d1670058` | `golden/edge.fs_stock_both_unknown.financial_statements.json` |
