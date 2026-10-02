# Fixture provenance: the `cash_payments_40a3` re-pin (2 Oct 2026)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What changed in the reference, and what these fixtures establish

`cash_payments_40a3` was last pinned at reference commit `105b6c37`. Since then the reference changed three behaviours and several texts, and the synthetic read reaches only the texts, so two invented edge books reach the behaviours:

- **s.40A(3) rows keyed by a ledger that names what was bought.** A row whose payee ledger is under Purchase Accounts, Direct Expenses or Indirect Expenses may pool several people's cash. Its figure definition, its title and an added limit say so, and the limit quotes every usable name the day's vouchers print (or says none is usable). `cp_unnamed` reaches each of these: a purchase ledger with one printed name and one voucher printing none, a direct-expenses ledger that also trips the goods-carriage flag (its own ledger name printed, which is not usable), an expense ledger whose vouchers print one name twice (once with surrounding blanks), one whose only printed name is the cash ledger, and one with one name and one unnamed; a named payee, and an excluded capital payee, keep the earlier wording.
- **s.269ST rows with no party ledger.** The receipt and payment rows pooled by day for vouchers that name no party take their own wording (the payers' or payees' side, "whether any one person ... is not known", a different ask) and cite the printed names on the row. `cp_unnamed` reaches a receipt row with one printed name and a voucher printing none, a payment row whose two vouchers print the same name, a row with a name that is the voucher's own Sales ledger, and a named party's receipt and payment rows for contrast.
- **s.269SS/269T candidates per voucher, loan ledger and side.** A voucher's credits to a loan ledger are summed (a loan accepted), and so are its debits (a loan repaid), and the limit is tested on each sum; the two sides of one ledger on one voucher are never netted, and the finding names the voucher by its GUID, or by its place in the population where the GUID is blank or shared. The text says when the voucher's cash moved the other way, and names any opposite entry. `cp_loans` reaches: one line; a receipt split over two lines; two lines each over the limit; both sides of one ledger on one voucher (a loan taken and repaid); a transfer between two loans on a voucher whose cash moved the other way; a voucher with no GUID, named by its number; two vouchers sharing a GUID on one day, after a Contra voucher so that their places count it; a repayment; a zero line; a side under the limit; a voucher whose cash nets to nothing; a voucher with no cash line; a Contra voucher; and loan ledgers configured with the client beside ones that are not.
- **Wording only, in the synthetic golden:** eight figure definitions and five findings change as the measured diff shows (the s.269SS/269T figure definitions, and the s.269ST unnamed-party rows).
- Regression fixtures only: they prove the port and the reference agree on the same inputs, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `c62a4ab4` (the last change to its `tae/` and `selftest/`), under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.cash_payments_40a3.json \
        --test cash_payments_40a3
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/cp_NAME.json tests/fixtures/golden

The edge books are written by hand in a small generator, as data, then read by both sides. The byte row of the regenerated synthetic golden stays in `PROVENANCE.md`, updated in place.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `cp_unnamed.json` | 6,731 | `4d8d62ade2e6f7a1ca5c5c57a2467042d2ac996c0b4f8d4dfef6e1bd3da95cf7` | `edge-books/cp_unnamed.json` |
| `edge.cp_unnamed.cash_payments_40a3.json` | 56,387 | `d4badbfbc49e54630c6a51e91522ccade7d7ebd85cf24fc56ef5a8e5ef0e02bf` | `golden/edge.cp_unnamed.cash_payments_40a3.json` |
| `cp_loans.json` | 5,156 | `d911f37fabeeefdb67298332dc3064c382df1e5c617831e7de838fc69fc14fa1` | `edge-books/cp_loans.json` |
| `edge.cp_loans.cash_payments_40a3.json` | 37,910 | `41eb086cc14a62be32592372f66845f6ead8a0df36ae27bc66792e9c6be75df1` | `golden/edge.cp_loans.cash_payments_40a3.json` |
