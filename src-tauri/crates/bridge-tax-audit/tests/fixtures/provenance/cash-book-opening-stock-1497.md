# Fixture provenance: cash_book_integrity and opening stock (#1497)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

A ledger-wise trial balance of a book with integrated inventory leaves out the opening stock, which Tally holds on the stock items. `cash_book_integrity` now adds the book's `opening_stock` term to the ledger openings' sum before it decides whether the opening balances differ (#1497). Each book sets that term through the edge-book key `opening_stock` and runs `cash_book_integrity`, whose dump also carries the book invariants, POP-3 among them.

- `edge-books/cash_book_stock_balanced.json`: the ledger openings sum to minus the opening stock. With the stock added nothing remains, and no opening difference is reported.
- `edge-books/cash_book_stock_difference.json`: a difference in opening balances remains after the stock. The finding's difference is that remainder, with the ledger sum and the stock beside it.
- `edge-books/cash_book_stock_unknown.json`: the stock items were not read, so the stock is not known. The finding reports the ledger sum and names the term in its limits.
- Not reached: how `load_book` decides the term from a read. The crate's unit tests and `tests/read_store.rs` cover that.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally.

## How they were produced

At the reference engine, commit `1631fe9a` (its change for #1497), under Python 3.13, with this pull request's own `parity/edge_golden.py` at its head `5dd7d556`:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The books are hand-written scenarios, not generated from any data. The maintainers generated the goldens and handed them over as files. Every edge golden already pinned regenerates byte-identical at that commit.

The three `cash_book_stock_*` goldens were later regenerated at reference commit `4aa48da7` (#1504), which corrects the `opening_difference` definition for books with integrated inventory. Only `definition_text` and `definition_sha256_16` of that figure change in each; the bytes below are the regenerated files', checked against the maintainers' hand-off.

## Declared behaviour difference (#1602)

The reference reads a book's opening stock as a known zero in two cases, and this crate no longer does (parity-spec §12, ruling of 26 October 2026):

- a stock-items part that parses but lists no stock item: the opening stock is unknown, with the reason "not taken: the read's stock items part lists no stock item; if the company keeps no stock, this is nil";
- a part whose items carry no opening value at all: unknown, with the reason "not taken: no stock item carries an opening value". One item without a value beside valued ones still counts as nothing, and an opening value of 0.00 is a value.

The crate also refuses a total of opening values that does not fit in its paise total ("not taken: the stock items' opening values add up to more than can be held") instead of wrapping it; the reference sums without a bound, so no real book differs there.

No committed golden or edge book reaches either case (edge books set the opening stock directly, and the one committed read has `ISINTEGRATED` set to `No`), so no golden moves and the divergence registry holds no record. The test book is the unit test `book::tests::opening_stock_is_taken_only_for_a_trial_balance_from_the_books_first_day`, on stock-items parts written in the test; it pins the change until a Chartered Accountant's hand-worked answer for such a book is recorded. Not measured: whether Tally omits an opening value of zero, which would make a book with no opening stock read as "no stock item carries an opening value".

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `cash_book_stock_balanced.json` | 1,105 | `208111aeea8436bbc8f2c26090972e95a86a0e9d7e5d612935ff4c7040823bd9` | `edge-books/cash_book_stock_balanced.json` |
| `cash_book_stock_difference.json` | 1,118 | `a1e9b3797594258fbfc79af9aa9a63dff92d7a6a5a6f33eb5323588f5298429f` | `edge-books/cash_book_stock_difference.json` |
| `cash_book_stock_unknown.json` | 1,124 | `1097c0a460c6c878f5bda2cee7f7ba15c459990b33cc9a132d33f0ae1a22405d` | `edge-books/cash_book_stock_unknown.json` |
| `edge.cash_book_stock_balanced.cash_book_integrity.json` | 5,531 | `864f797da965d286abfca9cdf88daf7606b8535c4c30f171bec9066774e47e5e` | `golden/edge.cash_book_stock_balanced.cash_book_integrity.json` |
| `edge.cash_book_stock_difference.cash_book_integrity.json` | 7,298 | `217e5ff7c86153a6309d44788c6a0c1b4184c4c9eca9fe9c9ff20590d0678bfe` | `golden/edge.cash_book_stock_difference.cash_book_integrity.json` |
| `edge.cash_book_stock_unknown.cash_book_integrity.json` | 6,350 | `21c6053047622f1ace5708c331a04c9a4559f19bb0a882423745e1a0ad306977` | `golden/edge.cash_book_stock_unknown.cash_book_integrity.json` |
