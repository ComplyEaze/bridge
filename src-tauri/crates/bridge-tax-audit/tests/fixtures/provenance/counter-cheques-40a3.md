# Fixture provenance: `counter_cheques_40a3`

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

- `golden/synthetic.counter_cheques_40a3.json` is the reference's dump on the synthetic read with no narration term configured: 12 figures, no finding. The test looks at nothing there and its `configured_terms_count` figure says so.
- `edge-books/cc_terms.json` reaches, with nine term entries configured (eight distinct terms): a term matching in any case; a repeated term and a term differing only in case (each kept once, as a Python set keeps them); the empty term (counted, matches nothing); a non-ASCII term upper-cased as Python does (`straße` against `STRASSE`, the `fi` ligature against `FINAL`; a term holding U+019B, which Python 3.13 does not upper-case and Rust's own tables map to U+A7DC, against a narration holding U+A7DC, which therefore does not match); a payment of exactly the limit (not over) and one paisa more (over); several lines on one ledger and two ledgers on one voucher (one row per line); a zero line; a Contra voucher; a voucher with a cash leg; a receipt that also debits an expense; a bank line and a cash line of nil; a cash ledger and another bank ledger on the debit side; an optional and a cancelled voucher; an empty narration; and one excluded payee for each of four of the five group roles (capital, loans (liability), fixed assets, duties and taxes; no payee in the book is under Loans & Advances (Asset), so that role's exclusion is not reached). 26 figures, 7 findings.
- `edge-books/cc_unconfigured.json`: the same ledgers and vouchers with no term, except `cc_terms`' last voucher (`v23`, the U+A7DC narration) and the two trial-balance rows it moves. 12 figures, no finding. The 12 figures are the registry's `min_figures`.
- `edge-books/cc_shared_guid.json` reaches vouchers that share a GUID: two with a blank GUID on different days, two with a blank GUID on one day (told apart by their numbers), and one GUID on two days, over and under the limit. Each figure's evidence cites every one of them, as the reference's set of refs keyed by GUID and label does (#1134). 18 figures, 3 findings.
- Not reached: a book with no bank ledger configured (no voucher can then match, the same outcome as no term), and an engagement whose cash and bank groups hold no ledger.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `c62a4ab4`, under Python 3.13, where its `counter_cheques_40a3.py` was the blob `803b0b7e` (last changed by the reference's commit `499e0b2e`). The reference has since changed that module's invariants (`df4af35e`, ported here for #1121) and its docstring (`da9e2d3d`, where it is the blob `ab7f62ee`); the four goldens below regenerate byte-identical at `c62a4ab4`, at the earlier `6c2d6be2` and at `da9e2d3d` (checked 5 Oct 2026), because the canonical dump carries a module invariant's text only when it fires, and none fires on these books:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.counter_cheques_40a3.json \
        --test counter_cheques_40a3
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/cc_NAME.json tests/fixtures/golden

The three edge books are hand-written scenarios, not generated from any data. `cc_shared_guid`'s golden was first produced at `da9e2d3d` (5 Oct 2026), by the same command.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `cc_terms.json` | 9,260 | `06f931f49dff7e4e000b40fb4d1d2c600259aaf8596b1e144169cae20cf408b2` | `edge-books/cc_terms.json` |
| `cc_shared_guid.json` | 2,298 | `552a3d2909fdfdc6d24f282fdc46d4157f54f31a6948e85caaf77828e28e8001` | `edge-books/cc_shared_guid.json` |
| `cc_unconfigured.json` | 8,072 | `4e644f9aa4402a81121cca78e85b61c1482f4b1c44d54047519d6da804ea52b2` | `edge-books/cc_unconfigured.json` |
| `edge.cc_shared_guid.counter_cheques_40a3.json` | 13,851 | `283b3ba37a904f1e97ca6f005dc5ace2ba1580bdc19ff9c2cf84a50b9652184b` | `golden/edge.cc_shared_guid.counter_cheques_40a3.json` |
| `edge.cc_terms.counter_cheques_40a3.json` | 24,863 | `a185c4e766c0dbbb2a2cd5c1d9ebc8f16bd278e6e9998158d741731db1df65d8` | `golden/edge.cc_terms.counter_cheques_40a3.json` |
| `edge.cc_unconfigured.counter_cheques_40a3.json` | 4,973 | `41c2e41c7c13aac058f81245dc76576907e1be05fc181b5ca806bebd7fd56d99` | `golden/edge.cc_unconfigured.counter_cheques_40a3.json` |
| `synthetic.counter_cheques_40a3.json` | 5,271 | `be006042ec164daa576660ee3e4e7684c00f33e7240842582756089b7bddc269` | `golden/synthetic.counter_cheques_40a3.json` |
