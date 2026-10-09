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

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `cash_book_stock_balanced.json` | 1,105 | `208111aeea8436bbc8f2c26090972e95a86a0e9d7e5d612935ff4c7040823bd9` | `edge-books/cash_book_stock_balanced.json` |
| `cash_book_stock_difference.json` | 1,118 | `a1e9b3797594258fbfc79af9aa9a63dff92d7a6a5a6f33eb5323588f5298429f` | `edge-books/cash_book_stock_difference.json` |
| `cash_book_stock_unknown.json` | 1,124 | `1097c0a460c6c878f5bda2cee7f7ba15c459990b33cc9a132d33f0ae1a22405d` | `edge-books/cash_book_stock_unknown.json` |
| `edge.cash_book_stock_balanced.cash_book_integrity.json` | 5,375 | `88068750d1697069a65e6627bb2bac6e587c6808037e29c4f25c2d6b53e9defe` | `golden/edge.cash_book_stock_balanced.cash_book_integrity.json` |
| `edge.cash_book_stock_difference.cash_book_integrity.json` | 7,142 | `14a0a13849f64aab9e0fc053373aa38c94ee20cf849f1d04a7bf885cd94fbcf3` | `golden/edge.cash_book_stock_difference.cash_book_integrity.json` |
| `edge.cash_book_stock_unknown.cash_book_integrity.json` | 6,194 | `08718d7cd54fe8d66f560cc44f1b00adb5274c51122ac4ef0496630a14dc44f4` | `golden/edge.cash_book_stock_unknown.cash_book_integrity.json` |
