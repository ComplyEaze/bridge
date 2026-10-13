# Hashes and provenance: the `questionnaire_cl13` spec pack

Each book here is invented, with synthetic ledger names and round figures: no book is a Tally read of
any real assessee, and no figure, name or narration comes from one.

## What these files establish, and what they do not

- The books are hand-written scenarios, each built to reach a rule or boundary of the reference
  engine's `questionnaire_cl13` test (the README lists which, and each book's `comment` says so
  voucher by voucher). They are regression fixtures: a port that reproduces every golden agrees with
  the reference on these books, and nothing more.
- `qc_empty` and `qc_quiet` give byte-identical goldens: with nothing counted and no stock result the
  dump does not depend on the book. They differ in why nothing is counted (no voucher at all, against
  twelve near misses).
- The two `edge.<book>.stock.json` goldens are the stock test's result on `qc_core` and `qc_stock`,
  made in the same run. They are the input this test was given on those books; they add nothing to
  the stock test's own contract, which is already ported and has its own goldens in the crate.
- They prove nothing about reading Tally. The books are built directly in the edge-book shape, not
  read from a company, so they say nothing about how real journals, group chains or periods reach the
  test, nor about how often a year-end journal touches an expense ledger in real books.
- Agreement on real books is not established by this pack. It will be established separately, once
  the test is ported, by a local parity run on real reads that is never committed.

## Which rules the goldens pin

Each rule below was changed alone, in a copy of the reference test held in memory (the reference
itself was not edited), and every book was run again; the books named are those whose
`questionnaire_cl13` golden then changed. No `stock` golden changed under any row. A port that gets
one of these rules wrong fails at least one golden.

| Rule changed | Goldens that change |
| --- | --- |
| the period's last day replaced by 31 March of the same year | `qc_period` |
| the period's last day replaced by a fixed 2026-03-31 | `qc_period` |
| the period's first day used instead of its last | `qc_core`, `qc_period`, `qc_shared_guid`, `qc_stock` |
| the period's first day counted as well as its last | `qc_period` |
| on or after the last day, instead of on it | `qc_period`, `qc_quiet` |
| on or before the last day, instead of on it | `qc_period`, `qc_quiet` |
| the voucher type name compared instead of the base type | `qc_core`, `qc_quiet` |
| the base type compared without regard to case | `qc_quiet` |
| any base type containing `Journal` accepted | `qc_quiet` |
| `Direct Expenses` dropped | `qc_core`, `qc_shared_guid`, `qc_stock` |
| `Indirect Expenses` dropped | `qc_core`, `qc_period`, `qc_shared_guid` |
| the ledger's parent group only, not its whole chain | `qc_core` |
| group names matched without regard to case | `qc_quiet` |
| a ledger named like an expense group counted | `qc_quiet` |
| zero lines ignored | `qc_core` |
| debit lines only | `qc_core`, `qc_stock` |
| every voucher read, not the population | `qc_quiet`, `qc_shared_guid` |
| vouchers counted, not distinct GUIDs | `qc_shared_guid` |
| GUIDs de-duplicated across the population before the three conditions | `qc_shared_guid` |
| the earlier of two same-GUID vouchers kept | `qc_shared_guid` |
| the stock pointer attached without a stock result | `qc_empty`, `qc_period`, `qc_quiet`, `qc_shared_guid` |
| the stock pointer never attached | `qc_core`, `qc_stock` |
| the stock pointer only when the stock count is above 0 | `qc_core` |
| the stock pointer only when the stock count is 0 | `qc_stock` |
| the stock pointer naming a figure id of this test | `qc_core`, `qc_stock` |
| the count's evidence left off the method-of-accounting finding | `qc_core`, `qc_period`, `qc_shared_guid`, `qc_stock` |
| the count's fact put on every question but the closing-stock one | every book |
| the label using the base type | `qc_core` |
| an empty number replaced by the whole GUID | `qc_core` |
| an empty number replaced by the GUID's first 12 characters | `qc_core` |
| an empty number left empty | `qc_core` |
| the module check counting distinct GUIDs | `qc_shared_guid` |
| the module check reading every status | `qc_quiet`, `qc_shared_guid` |
| no module check | every book |

Any change to a fixed text (a title, the limit, an item to ask, a definition, the population note), a
clause list or the answer value changes every golden.

## How the goldens were produced

At the reference engine (a private repository), commit `4df1cc43`, under Python 3.13, with the
crate's `parity/edge_golden.py`, which is in this repository under its Apache-2.0 licence and has no
runner for this test. The goldens were made with that file extended by the runner README section 14
gives as text; the extended file is held by the reference's maintainers and is not part of this
pack. For a book that carries both `stock_opening` and `stock_closing`, the runner runs the file's
own `stock` runner on the book first and passes its result to the test, as the reference's pack
passes the stock result; for a book with neither, it passes none; a book with one only is refused.
It gives the canonical dump the test module itself, whose module check takes the book and the
result. The stock goldens come from the file's existing `stock` runner, because those two books also
name `stock`. Run from `src-tauri/crates/bridge-tax-audit`, once per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python EDGE_GOLDEN_WITH_RUNNER ENGINE \
        BOOK.json OUTDIR

Each run writes `OUTDIR/edge.<book>.questionnaire_cl13.json`, and `OUTDIR/edge.<book>.stock.json`
for the two books naming `stock`. Running every book a second time, into a new directory,
reproduced every golden byte for byte.

The synthetic dump README section 14 describes was made with the crate's
`parity/python_golden.py` extended by the runner that section gives, with
`ENGINE tests/fixtures/synthetic-engagement.toml OUT.json --test questionnaire_cl13`; made twice, it
was byte-identical both times, and the same extended file reproduced the crate's committed
`synthetic.stock.json` byte for byte.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `qc_core.json` | 5,206 | `87b93815843a7d4fd4e3720d617d08ec5894d50d8674505d1b366d1ed66451b1` | `books/qc_core.json` |
| `qc_empty.json` | 2,281 | `1332d16f479eec12ec47380af37783dc05f92bd71cb13307ccc93dd22f2b9d99` | `books/qc_empty.json` |
| `qc_period.json` | 3,177 | `8db4fa32200ab36f6ef4a309bb933142f5e8e8980d8f53148e951095cf4b8961` | `books/qc_period.json` |
| `qc_quiet.json` | 5,902 | `d325b5c6f951ba389c5a26b6f6cee401afa6e9d5b18ee99beb5bdd12a4e13213` | `books/qc_quiet.json` |
| `qc_shared_guid.json` | 3,967 | `ada9caefbfd0626920c8ee3ebcde1898d1f0ee37f794d23f32297d17c745d0fb` | `books/qc_shared_guid.json` |
| `qc_stock.json` | 3,294 | `4c3e4868d96f57aad66401ab80e9ca4fc735c514c6860ccf1ed728052d24a82f` | `books/qc_stock.json` |
| `edge.qc_core.questionnaire_cl13.json` | 9,796 | `5552bafa5cb4d4e0bb004691ef02bf0a469d36237ca6c00b7e9bc8c50f906565` | `goldens/edge.qc_core.questionnaire_cl13.json` |
| `edge.qc_core.stock.json` | 15,126 | `04631eb603a80a37398639f5c91a8891fcdc3ccf6f96cb9c72b91be5d6212f53` | `goldens/edge.qc_core.stock.json` |
| `edge.qc_empty.questionnaire_cl13.json` | 7,573 | `a4a40038c585f37f64ca4c4153fe467cb4ba5832b3d27114342d387a037b872a` | `goldens/edge.qc_empty.questionnaire_cl13.json` |
| `edge.qc_period.questionnaire_cl13.json` | 7,847 | `7d887c086b6d136e83695c5b162ee79f7ad17e1d38726c826613a80cf9cd6912` | `goldens/edge.qc_period.questionnaire_cl13.json` |
| `edge.qc_quiet.questionnaire_cl13.json` | 7,573 | `a4a40038c585f37f64ca4c4153fe467cb4ba5832b3d27114342d387a037b872a` | `goldens/edge.qc_quiet.questionnaire_cl13.json` |
| `edge.qc_shared_guid.questionnaire_cl13.json` | 9,075 | `695466cf47b2ecb84d02cfb0a99c4c5a83273a693e74789c90525212f72d4cf9` | `goldens/edge.qc_shared_guid.questionnaire_cl13.json` |
| `edge.qc_stock.questionnaire_cl13.json` | 8,184 | `87592abb59bc8677a0f2f130f33a35d2f3da4b97de4fe92aaaa247280c27236b` | `goldens/edge.qc_stock.questionnaire_cl13.json` |
| `edge.qc_stock.stock.json` | 13,977 | `3f7bfbb34665b1e1195fff7af21f12a1b38c0d4c7d6c2d742e8bf4b672a346fa` | `goldens/edge.qc_stock.stock.json` |
| `synthetic.questionnaire_cl13.json` | 8,804 | `cb94a446f01c7b5d054e2782e9809ed4af6674b054bf77d90d3c7fbb80247f74` | `goldens/synthetic.questionnaire_cl13.json` |
