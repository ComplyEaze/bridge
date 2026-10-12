# Fixture provenance: `questionnaire_cl13` (8 Oct 2026)

The six edge books, their goldens, the two stock goldens and the synthetic golden come unchanged, byte
for byte, from the spec pack `docs/tax-audit/spec-packs/questionnaire_cl13/`, whose `HASHES.md` lists
the same bytes and SHA-256 for each file. Every book there is invented, with synthetic ledger names and
round figures: no book is a Tally read of any real assessee, and no figure, name or narration comes
from one.

## What these fixtures establish

- The books are hand-written scenarios, each built to reach a rule or boundary of the reference
  engine's `questionnaire_cl13` test; the pack's README lists which (its section 12), and each book's
  `comment` says what it reaches, voucher by voucher. They are regression fixtures: the port
  reproduces every golden, so it agrees with the reference on these books, and nothing more.
- They prove nothing about reading Tally: the books are built directly in the edge-book shape, not
  read from a company. Agreement on real books is not measured here.
- `qc_empty` and `qc_quiet` have byte-identical goldens: with nothing counted and no stock result the
  dump does not depend on the book.
- `golden/edge.qc_core.stock.json` and `golden/edge.qc_stock.stock.json` are the already-ported
  `stock` test on the two books with Stock Summaries, from the same run. They are the input this test
  was given on those books and add nothing to the stock test's own contract.
- `golden/synthetic.questionnaire_cl13.json` is the test on the crate's synthetic read, with the stock
  result of the same read passed in.

## How they were produced

At the reference engine (a private repository), commit `4df1cc43`, under Python 3.13, with
`parity/edge_golden.py` extended by the runner the pack's README section 14 gives, which runs the
file's own `stock` runner first on a book carrying both `stock_opening` and `stock_closing` and passes
its result to the test. Run from `src-tauri/crates/bridge-tax-audit`, once per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The synthetic golden is the same test on the crate's synthetic engagement, through
`parity/python_golden.py`'s runner for it:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.questionnaire_cl13.json \
        --test questionnaire_cl13

Running every book a second time reproduced every golden byte for byte. The goldens are regenerated
only by the reference's maintainers.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `qc_core.json` | 5,206 | `87b93815843a7d4fd4e3720d617d08ec5894d50d8674505d1b366d1ed66451b1` | `edge-books/qc_core.json` |
| `qc_empty.json` | 2,281 | `1332d16f479eec12ec47380af37783dc05f92bd71cb13307ccc93dd22f2b9d99` | `edge-books/qc_empty.json` |
| `qc_period.json` | 3,177 | `8db4fa32200ab36f6ef4a309bb933142f5e8e8980d8f53148e951095cf4b8961` | `edge-books/qc_period.json` |
| `qc_quiet.json` | 5,902 | `d325b5c6f951ba389c5a26b6f6cee401afa6e9d5b18ee99beb5bdd12a4e13213` | `edge-books/qc_quiet.json` |
| `qc_shared_guid.json` | 3,967 | `ada9caefbfd0626920c8ee3ebcde1898d1f0ee37f794d23f32297d17c745d0fb` | `edge-books/qc_shared_guid.json` |
| `qc_stock.json` | 3,294 | `4c3e4868d96f57aad66401ab80e9ca4fc735c514c6860ccf1ed728052d24a82f` | `edge-books/qc_stock.json` |
| `edge.qc_core.questionnaire_cl13.json` | 9,796 | `5552bafa5cb4d4e0bb004691ef02bf0a469d36237ca6c00b7e9bc8c50f906565` | `golden/edge.qc_core.questionnaire_cl13.json` |
| `edge.qc_core.stock.json` | 15,126 | `04631eb603a80a37398639f5c91a8891fcdc3ccf6f96cb9c72b91be5d6212f53` | `golden/edge.qc_core.stock.json` |
| `edge.qc_empty.questionnaire_cl13.json` | 7,573 | `a4a40038c585f37f64ca4c4153fe467cb4ba5832b3d27114342d387a037b872a` | `golden/edge.qc_empty.questionnaire_cl13.json` |
| `edge.qc_period.questionnaire_cl13.json` | 7,847 | `7d887c086b6d136e83695c5b162ee79f7ad17e1d38726c826613a80cf9cd6912` | `golden/edge.qc_period.questionnaire_cl13.json` |
| `edge.qc_quiet.questionnaire_cl13.json` | 7,573 | `a4a40038c585f37f64ca4c4153fe467cb4ba5832b3d27114342d387a037b872a` | `golden/edge.qc_quiet.questionnaire_cl13.json` |
| `edge.qc_shared_guid.questionnaire_cl13.json` | 9,075 | `695466cf47b2ecb84d02cfb0a99c4c5a83273a693e74789c90525212f72d4cf9` | `golden/edge.qc_shared_guid.questionnaire_cl13.json` |
| `edge.qc_stock.questionnaire_cl13.json` | 8,184 | `87592abb59bc8677a0f2f130f33a35d2f3da4b97de4fe92aaaa247280c27236b` | `golden/edge.qc_stock.questionnaire_cl13.json` |
| `edge.qc_stock.stock.json` | 13,977 | `3f7bfbb34665b1e1195fff7af21f12a1b38c0d4c7d6c2d742e8bf4b672a346fa` | `golden/edge.qc_stock.stock.json` |
| `synthetic.questionnaire_cl13.json` | 8,804 | `cb94a446f01c7b5d054e2782e9809ed4af6674b054bf77d90d3c7fbb80247f74` | `golden/synthetic.questionnaire_cl13.json` |
