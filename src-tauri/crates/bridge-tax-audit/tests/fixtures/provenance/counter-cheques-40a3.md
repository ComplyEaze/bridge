# Fixture provenance: `counter_cheques_40a3`

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

- `golden/synthetic.counter_cheques_40a3.json` is the reference's dump on the synthetic read with no narration term configured: 12 figures, no finding. The test looks at nothing there and its `configured_terms_count` figure says so.
- `edge-books/cc_terms.json` reaches, with eight terms configured: a term matching in any case; a repeated term and a term differing only in case (each kept once, as a Python set keeps them); the empty term (counted, matches nothing); a non-ASCII term upper-cased as Python does (`straße` against `STRASSE`, the `fi` ligature against `FINAL`); a payment of exactly the limit (not over) and one paisa more (over); several lines on one ledger and two ledgers on one voucher (one row per line); a zero line; a Contra voucher; a voucher with a cash leg; a receipt that also debits an expense; a bank line and a cash line of nil; a cash ledger and another bank ledger on the debit side; an optional and a cancelled voucher; an empty narration; and one excluded payee for each of four of the five group roles (capital, loans (liability), fixed assets, duties and taxes; no payee in the book is under Loans & Advances (Asset), so that role's exclusion is not reached). 26 figures, 7 findings.
- `edge-books/cc_unconfigured.json`: the same ledgers and vouchers with no term. 12 figures, no finding. The 12 figures are the registry's `min_figures`.
- Not reached: a book with no bank ledger configured (no voucher can then match, the same outcome as no term), and an engagement whose cash and bank groups hold no ledger.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `c62a4ab4` (the last change to its `tae/` and `selftest/`), under Python 3.13. Its `counter_cheques_40a3.py` is the blob `803b0b7e`, last changed by the reference's commit `499e0b2e`; the three goldens below regenerate byte-identical at `c62a4ab4` and at the earlier `6c2d6be2` (checked 3 Oct 2026):

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.counter_cheques_40a3.json \
        --test counter_cheques_40a3
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/cc_NAME.json tests/fixtures/golden

The two edge books are hand-written scenarios, not generated from any data.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `cc_terms.json` | 8,820 | `11d52b1f9ae6e53466300db22c9de74853898d3768b02e5f81ca69ada726ffc9` | `edge-books/cc_terms.json` |
| `cc_unconfigured.json` | 8,064 | `1caeee97b970999a8212f38482ab13b3ff9e9ffec2825c657a1eb1946726fe23` | `edge-books/cc_unconfigured.json` |
| `edge.cc_terms.counter_cheques_40a3.json` | 24,863 | `10ef069890e6f9461dd6b69e9b277b33cd9199d5d166504988d36a79e0c36e42` | `golden/edge.cc_terms.counter_cheques_40a3.json` |
| `edge.cc_unconfigured.counter_cheques_40a3.json` | 4,973 | `41c2e41c7c13aac058f81245dc76576907e1be05fc181b5ca806bebd7fd56d99` | `golden/edge.cc_unconfigured.counter_cheques_40a3.json` |
| `synthetic.counter_cheques_40a3.json` | 5,271 | `be006042ec164daa576660ee3e4e7684c00f33e7240842582756089b7bddc269` | `golden/synthetic.counter_cheques_40a3.json` |
