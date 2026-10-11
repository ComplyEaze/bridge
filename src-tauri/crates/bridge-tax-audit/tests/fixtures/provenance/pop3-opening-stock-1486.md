# Fixture provenance: POP-3 and opening stock (#1486)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

A ledger-wise trial balance of a book with integrated inventory leaves out the opening stock, which Tally holds on the stock items. POP-3 now adds the book's `opening_stock` term before it tests the opening and closing sums (#1486). Each book sets that term through the edge-book key `opening_stock` and runs `trial_balance`, whose dump carries the book invariants.

- `edge-books/pop3_stock_balanced.json`: the ledger openings sum to minus the opening stock. With the stock added the trial balance ties, and POP-3 reports nothing.
- `edge-books/pop3_stock_difference.json`: a difference in opening balances remains after the stock. POP-3 names the ledger sum, the stock and that difference on both sums.
- `edge-books/pop3_stock_unknown.json`: the trial balance does not start on the books' first day, so the stock is not taken. POP-3 reports the ledger sum and says why.
- Not reached: how `load_book` decides the term from a read. The crate's unit tests and `tests/read_store.rs` cover that, on the synthetic read.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `45f28ef9` (its change for #1486: `6368867a`, then the reason text in `45f28ef9`), under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The books are hand-written scenarios, not generated from any data. All 268 edge goldens already pinned regenerate byte-identical at that commit.

#1497 widened the reason text `pop3_stock_unknown.json` names, so that book's bytes changed. Its golden was regenerated at reference commit `1631fe9a` (the change for #1497), with the same command. Only its two POP-3 details changed, and its `.order.json` did not.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `pop3_stock_balanced.json` | 799 | `9e00078f36fabeb5ab08922cc2ee10ece9d88f9bd8ee40979ab8a7087059f9aa` | `edge-books/pop3_stock_balanced.json` |
| `pop3_stock_difference.json` | 802 | `215d842eeb736d605b7b80ff97db4bb69877c1c0c8722f81995660c1e9bd28aa` | `edge-books/pop3_stock_difference.json` |
| `pop3_stock_unknown.json` | 907 | `8b8d9192b053d435760bcc31eadc80aeb8188f4723156e1465cd773f9473eca0` | `edge-books/pop3_stock_unknown.json` |
| `edge.pop3_stock_balanced.trial_balance.json` | 5,957 | `112678a801a38c5a1f18b0f39f30b0796376014d2923f94a5ad3b035a62ef36b` | `golden/edge.pop3_stock_balanced.trial_balance.json` |
| `edge.pop3_stock_balanced.trial_balance.order.json` | 75 | `e7b6c8fc2c54ccd1de1d7cfdd43ae2a73c16ad0d0ae4ba0aa0b91b082165d2c1` | `golden/edge.pop3_stock_balanced.trial_balance.order.json` |
| `edge.pop3_stock_difference.trial_balance.json` | 6,399 | `8764115ea04aedb8d961195d73a4c1cce88bb7388e79acff6acc9218cef49c23` | `golden/edge.pop3_stock_difference.trial_balance.json` |
| `edge.pop3_stock_difference.trial_balance.order.json` | 75 | `e7b6c8fc2c54ccd1de1d7cfdd43ae2a73c16ad0d0ae4ba0aa0b91b082165d2c1` | `golden/edge.pop3_stock_difference.trial_balance.order.json` |
| `edge.pop3_stock_unknown.trial_balance.json` | 6,425 | `de00f1e4c1b7f93a32bd448723dca610580a9e5b088d296e4b5bc44ba1738ba7` | `golden/edge.pop3_stock_unknown.trial_balance.json` |
| `edge.pop3_stock_unknown.trial_balance.order.json` | 75 | `e7b6c8fc2c54ccd1de1d7cfdd43ae2a73c16ad0d0ae4ba0aa0b91b082165d2c1` | `golden/edge.pop3_stock_unknown.trial_balance.order.json` |
