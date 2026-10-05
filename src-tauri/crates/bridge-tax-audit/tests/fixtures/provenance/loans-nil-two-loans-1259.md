# Fixture provenance: nil lines on two loans (#1259)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

After bridge#1201 a nil line on ANOTHER configured loan is no posting to it for s.194A coverage. The reference then made the same true of the loan being tested (bridge#1259, item 1): the coverage reads a voucher's TDS for a loan only if the voucher has a line with an amount on that loan. Before it, a voucher with a nil line on each of two loans, and its TDS beside some other ledger, counted that one deduction on both loans, and LOAN-1 fired on both.

- `edge-books/loans_interest_nil_two_loans.json` (a firm; three configured loans): `n01` is a TDS journal against Bank A with a nil line on Loan One and on Loan Two, so its TDS is on neither loan; `n02` is a TDS journal on Loan Three beside a nil line on Loan One, so its TDS is on Loan Three only (the nil line on Loan One is no posting to it). `tds_on_loan` is 0 for Loan One, 0 for Loan Two and 120000 for Loan Three; LOAN-1 does not fire for any loan. At the reference's previous commit (`976ba9dd`) the golden has `tds_on_loan` 120000 on all three loans and LOAN-1 fires for Loan One and Loan Two.
- Not reached: items 2 and 3 of #1259 (a repayment net of TDS beside a nil line, and an interest journal beside a nil line), which the reference has not changed.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `5fcc6136` (its change for #1259 item 1, on top of `976ba9dd`), under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/loans_interest_nil_two_loans.json tests/fixtures/golden

The book is a hand-written scenario, not generated from any data. All 130 goldens already pinned regenerate byte-identical at that commit.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `loans_interest_nil_two_loans.json` | 3,182 | `a8ce4fa80b6c23e27eaa96d062f648a9e9fd4fe7eb12484e902ceb0558fe6804` | `edge-books/loans_interest_nil_two_loans.json` |
| `edge.loans_interest_nil_two_loans.loans_interest.json` | 18,575 | `b43df64a4d6535e32942e029f8bbd1b74b33082ed9fbbcc6f4a9123901abab60` | `golden/edge.loans_interest_nil_two_loans.loans_interest.json` |
