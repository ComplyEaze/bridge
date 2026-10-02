# Fixture provenance: the `high_value_register` re-pin (2 Oct 2026)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What changed in the reference, and what these fixtures establish

The last pin of `high_value_register` (reference commit `140bc7d3`, 26 Sep 2026) grouped the vouchers of a day that carry no party ledger under one row but still wrote the row as one party ("one unidentified party"). The reference has since stopped asserting a person for that row: it pools every such voucher of the day (or of a voucher reference) under one row whatever name the vouchers print, states the row in its own terms (the payers' or payees' side, "vouchers that do not name the payer", whether any one person reached the limit is "not known"), and cites on the row every usable printed name. It also reports why a supplied bank statement was not used when the reader refused it. The measured differences in the three goldens already pinned are the unnamed rows' figure definitions, titles, limits and asks only; no figure value, no tag and no other finding changes (`hvr_bare` is byte-identical).

- `golden/synthetic.high_value_register.json` and `golden/edge.hvr_paths.high_value_register.json` (regenerated): their two unnamed rows (one receipt, one payment) take the new wording.
- `hvr_unnamed` (new): every way a printed name can be usable or not, on one pooled receipt row (a name with surrounding blanks, a name containing " or ", a non-ASCII name, a name ending in a Python-whitespace control character, a name that is the voucher's own Sales ledger, the cash ledger or the bank ledger, a blank name, and a name made only of control whitespace; the label quotes and sorts the usable ones and says vouchers printing none were pooled); a row whose every voucher prints one usable name; a row proven under the limit on its cash line (an unnamed receipt, payment and bank row, and a named receipt, payment and bank row, so the named title is pinned too), one whose party side differs from its line without being under it (a receipt and a payment), a bank row over the threshold, a named party's row beside an unnamed one on the same day, limb (b) rows for a shared reference citing the printed names (receipts, with one voucher printing none; payments, with two vouchers printing the same name), and a supplied statement that the reader refused (`bank_statement_refused`, with no statement) so s.194N's coverage says so.
- Regression fixtures only: they prove the port and the reference agree on the same inputs, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `c62a4ab4` (the last change to its `tae/` and `selftest/`), under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.high_value_register.json \
        --test high_value_register --bank-statement tests/fixtures/synthetic-bank-statement.json \
        --traces-documents tests/fixtures/synthetic-traces-documents.json
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/hvr_NAME.json tests/fixtures/golden

`hvr_unnamed.json` is written by hand in a small generator, as data, then read by both sides. The byte rows of the two regenerated goldens stay in `batch-e2b.md`, updated in place.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `hvr_unnamed.json` | 7,971 | `3bd7e06b995cccb63c0d9226baeaa703b986a678600ff9aca1a61912726879ee` | `edge-books/hvr_unnamed.json` |
| `edge.hvr_unnamed.high_value_register.json` | 94,576 | `b3716fc3211f9f79e7f597d79ab53d7848ed1fdd16b7a2b87d31713fd2bf734c` | `golden/edge.hvr_unnamed.high_value_register.json` |
