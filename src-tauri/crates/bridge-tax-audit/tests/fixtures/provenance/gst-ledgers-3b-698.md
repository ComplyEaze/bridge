# Fixture provenance: `gst_ledgers_3b` without a GSTR-3B (#698)

Every book here is invented, with plain names and small invented amounts: no fixture is a Tally read of any real
assessee, and none carries a GSTIN.

## What these fixtures establish, and what they do not

`gst_ledgers_3b` splits each GST duty head's movement over the year into output, input, payment and other by the
category of each voucher, ties it to the Trial Balance, and gives the books' monthly output tax, taxable turnover,
old residual taxable value and reverse-charge tax for a comparison with GSTR-3B; it asks the goods-transport
question on goods-carriage payments with no duties line. These fixtures cover only an engagement with no GSTR-3B
document: there the reference classifies every month `no_3b_document_for_month` before it reads GSTR-1 rows or the
round-off movement, so its dump depends on the book and five ledger roles alone. Each edge book's period is two
months, to keep its golden small; the synthetic read's is the year.

The crate refuses an engagement whose `[documents].gstr3b` holds anything but an empty string, and a caller that
holds a GSTR-3B (named as loaded, or as `applicability_44ab`'s GSTR-3B turnover). That refuses more than the
reference, which is accepted because it fails closed: the reference reads a document only when the key is truthy, so
`false`, `0`, `[]` or `{}` is no document there, and a GSTR-3B loaded with no row in the period gives the reference
this same no-GSTR-3B dump. A `[documents]` that is not a table ends the reference's run, and the crate refuses it as
a client config error. Unit tests pin the `false` refusal and the not-a-table refusal.

GSTR-1 is not an input of the crate's test, and a named GSTR-1 is not refused: `edge-books/g3b_gstr1.json` shows the
reference's dump does not read one when there is no GSTR-3B.

- `golden/synthetic.gst_ledgers_3b.json` is the test on the synthetic read, through the registry's path from the
  engagement. `synthetic-engagement.toml` gains the two required roles `[roles].payable_ledgers` and
  `[roles].rcm_tax_ledgers`, both empty; its `tax_ledgers` table is already empty, so the dump has no per-head
  figure and carries the twelve months' Sales Accounts movement and residual. No other test's figures read either
  key, and `synthetic.clause44`, `synthetic.book_keeping_quality` and `synthetic.cash_44ab` regenerate byte-identical
  with them (`PROVENANCE.md` carries the engagement's new byte row). Binding is shared, though: on any engagement, a
  name under either key that matches no ledger now refuses every test (`BIND-NAME-UNKNOWN`), and a `[ledger_ids]`
  label used only there now counts as used.
- `edge-books/g3b_classify.json`: one voucher per branch of the voucher classifier: Sales, Purchase, a sales-side
  and a purchase-side note, a note with both groups, a note with neither, a note whose Sales line is under a
  sub-group, a note whose Sales line is nil beside a Purchase line, Payment, Receipt, a Journal reclassing among tax
  ledgers, onto an unconfigured Duties & Taxes ledger, onto a payable ledger, against an expense, against a ledger
  with no master, beside a nil expense line, and a Contra. Also a nil tax line alone, a cancelled voucher, a voucher
  after the period (in the heads and the annual residual, in no month), and for the residual: tax, round-off and
  party lines, a voucher of only tax and round-off, and a party named on no line. The heads' Trial Balance closings
  are off by 0, -100 (inside LEDGER-1's Re 1) and 101 paise (one LEDGER-1 violation).
- `edge-books/g3b_gstr1.json`: the book of `g3b_classify`, ledger GUIDs included, with a GSTR-1 named: the reference
  is given its rows (`gst_ledgers_3b.gstr1`, which `parity/edge_golden.py` passes as the pack passes a named
  GSTR-1's rows, with `gstr1_not_supplied` false), and `tests/edge_books.rs` does not read them. The rows are ones a
  month's gap would read if a GSTR-3B were supplied: b2b invoices dated in another month and in the same month, a
  cdnr note in another month, a b2ba and a cdnra amendment, a b2cs row with no date and a row naming no voucher. Its
  golden is byte for byte `g3b_classify`'s (asserted in `tests/edge_books.rs`): with no GSTR-3B row the reference
  reads none of them.
- `edge-books/g3b_quiet.json`: no tax, RCM or goods-carriage ledger: no per-head figure and no electronic-ledger
  finding.
- `edge-books/g3b_missing_tb.json`: a head of two ledgers, one with no Trial Balance row; nil tax lines; RCM unset;
  goods-carriage ledgers set with every payment on them carrying a duties line (one a nil line on an unconfigured
  Duties & Taxes ledger), so nothing is listed and no question is asked.
- `edge-books/g3b_rcm.json`: RCM tax ledgers set: two journals in one month, a voucher whose RCM lines net to nil
  (not cited), a payment; goods-carriage payments listed (a bare payment, a journal with a debit and a credit on the
  ledger) and not listed (a tax line, a nil unconfigured duties line, an RCM line, only a credit, only a nil line, a
  cancelled voucher, and a configured tax line and a configured RCM line on ledgers outside Duties & Taxes).
- `edge-books/g3b_gta_no_rcm.json`: goods-carriage ledgers set and no RCM tax ledger: the question with the
  no-RCM-ledger limit, and no reverse-charge books figure.
- Not reached by a golden, and asserted in the crate's unit tests instead: LEDGER-1's opening, closing and
  missing-figure messages, LEDGER-2 and LEDGER-3 (the test's own result cannot reach them); a missing required role,
  a GSTR-3B named or loaded, a `[documents]` that is not a table, and a goods-carriage ledger not mapped to 194C
  (all refused); the two roles' binding
  order.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading
  Tally. Agreement on real books is the maintainers' local parity run, never committed.

## How they were made

`edge-books/g3b_*.json` were written by a generator that derives each Trial Balance row from the book's own regular
vouchers and an opening, then shifts the rows `g3b_classify` tests the tolerance on. Each golden is the reference
engine's own canonical dump at its commit `6168aeaa`, made with the crate's harness, per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python -I parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

and on the synthetic engagement:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python -I parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.gst_ledgers_3b.json \
        --test gst_ledgers_3b

At that commit the same harness reproduced every existing golden byte for byte. The goldens are regenerated only by
the reference's maintainers.

`edge-books/g3b_gstr1.json` is `g3b_classify.json` with its comment replaced and the `gst_ledgers_3b.gstr1` rows
added. Its golden was made by the same command at the same commit, after that harness, with the `gstr1` key it now
reads, reproduced `g3b_classify`'s golden byte for byte.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `g3b_classify.json` | 10,449 | `c6cb428822f55c8800f117ae266fd58ca824262c4e725b5d9c1b3d8c13afc1d4` | `edge-books/g3b_classify.json` |
| `g3b_gstr1.json` | 11,627 | `3c31a83adc3ea50bc4c99ff00c6263cced8de1aabae09bf3bb244c58bac59d2b` | `edge-books/g3b_gstr1.json` |
| `g3b_gta_no_rcm.json` | 3,182 | `4873d66ea04cdac2de44822ea3592ffa0180ff731a69cad532fc1dc027fef917` | `edge-books/g3b_gta_no_rcm.json` |
| `g3b_missing_tb.json` | 4,376 | `3abd710f2669816d83c7dd7d5adc189a498b7c424ab7aecdb234a0ecf3432523` | `edge-books/g3b_missing_tb.json` |
| `g3b_quiet.json` | 2,127 | `5d44416c481d8302799eaa443677d8c6d797a7332492d4a67d0c80da31232911` | `edge-books/g3b_quiet.json` |
| `g3b_rcm.json` | 7,628 | `680abce7c7a1f139357bf4da984989de6ebfd1b27c9573eb0ed2809d134bd539` | `edge-books/g3b_rcm.json` |
| `edge.g3b_classify.gst_ledgers_3b.json` | 31,701 | `09b8740db8c0c936840958db65f4a0f4bf2ad23a1768801b518df3be611036a6` | `golden/edge.g3b_classify.gst_ledgers_3b.json` |
| `edge.g3b_gstr1.gst_ledgers_3b.json` | 31,701 | `09b8740db8c0c936840958db65f4a0f4bf2ad23a1768801b518df3be611036a6` | `golden/edge.g3b_gstr1.gst_ledgers_3b.json` |
| `edge.g3b_gta_no_rcm.gst_ledgers_3b.json` | 26,899 | `fe167871c642a7f50549ebae6bb82eda0cb718ce6ccebdd786eba353af99566b` | `golden/edge.g3b_gta_no_rcm.gst_ledgers_3b.json` |
| `edge.g3b_missing_tb.gst_ledgers_3b.json` | 25,558 | `995d3acc554cc7710d4a4818b02c1120eb63e6097617278168706f5d1ede7017` | `golden/edge.g3b_missing_tb.gst_ledgers_3b.json` |
| `edge.g3b_quiet.gst_ledgers_3b.json` | 19,375 | `ed183caa7e74160c0edb0410034f85d1504febea8a8fc25c0759c4567da2f2cf` | `golden/edge.g3b_quiet.gst_ledgers_3b.json` |
| `edge.g3b_rcm.gst_ledgers_3b.json` | 31,154 | `877ce1f9ef320e1ed787d060cc7485ee974bdc9b1a9a275f28cfad3f56021291` | `golden/edge.g3b_rcm.gst_ledgers_3b.json` |
| `synthetic.gst_ledgers_3b.json` | 53,872 | `b11ee795beeaf8c943de467d42f2d1b299639771fbb05c7a1ed1f6d2eb5a244a` | `golden/synthetic.gst_ledgers_3b.json` |
