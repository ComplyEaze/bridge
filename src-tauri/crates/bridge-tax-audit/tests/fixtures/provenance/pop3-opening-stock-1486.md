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

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `pop3_stock_balanced.json` | 799 | `9e00078f36fabeb5ab08922cc2ee10ece9d88f9bd8ee40979ab8a7087059f9aa` | `edge-books/pop3_stock_balanced.json` |
| `pop3_stock_difference.json` | 802 | `215d842eeb736d605b7b80ff97db4bb69877c1c0c8722f81995660c1e9bd28aa` | `edge-books/pop3_stock_difference.json` |
| `pop3_stock_unknown.json` | 890 | `099491995549d5e8b5edf740926719845038f70846dfa2e7d0e55a265df18da0` | `edge-books/pop3_stock_unknown.json` |
| `edge.pop3_stock_balanced.trial_balance.json` | 5,957 | `112678a801a38c5a1f18b0f39f30b0796376014d2923f94a5ad3b035a62ef36b` | `golden/edge.pop3_stock_balanced.trial_balance.json` |
| `edge.pop3_stock_balanced.trial_balance.order.json` | 75 | `e7b6c8fc2c54ccd1de1d7cfdd43ae2a73c16ad0d0ae4ba0aa0b91b082165d2c1` | `golden/edge.pop3_stock_balanced.trial_balance.order.json` |
| `edge.pop3_stock_difference.trial_balance.json` | 6,399 | `8764115ea04aedb8d961195d73a4c1cce88bb7388e79acff6acc9218cef49c23` | `golden/edge.pop3_stock_difference.trial_balance.json` |
| `edge.pop3_stock_difference.trial_balance.order.json` | 75 | `e7b6c8fc2c54ccd1de1d7cfdd43ae2a73c16ad0d0ae4ba0aa0b91b082165d2c1` | `golden/edge.pop3_stock_difference.trial_balance.order.json` |
| `edge.pop3_stock_unknown.trial_balance.json` | 6,391 | `28d3ae6e390c8cc7c96fb40bf7cc8fd2bf3ac6e25131a7d1c932d06ad7bb078b` | `golden/edge.pop3_stock_unknown.trial_balance.json` |
| `edge.pop3_stock_unknown.trial_balance.order.json` | 75 | `e7b6c8fc2c54ccd1de1d7cfdd43ae2a73c16ad0d0ae4ba0aa0b91b082165d2c1` | `golden/edge.pop3_stock_unknown.trial_balance.order.json` |
