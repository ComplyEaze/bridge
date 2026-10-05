# Fixture provenance: `books_examined` (#787)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

`books_examined` (Form 3CD clause 11(b)/(c)) computes nothing. It lists the books this read of Tally holds: the day
book by voucher type, the cash and bank books by ledger count, the ledger accounts and the Trial Balance. It also
lists the documents the pack loaded for the engagement. The document names are caller data: the reference's pack
passes the documents it actually read, in its own order.

- `golden/synthetic.books_examined.json` is the test on the synthetic read, with the four documents in
  `synthetic-documents-read.json` (invented caller data, in the pack's order).
- `edge-books/be_none.json`: one ledger and nothing else. There is no voucher, so no day book, and there is no cash
  or bank ledger and no Trial Balance. No document was loaded ("none loaded with this read"). The reference does
  not make the ledger count singular ("1 ledger accounts"), and the port keeps that.
- `edge-books/be_singular.json`: one cash ledger and one bank ledger, singular. The bank ledger is under Bank OD
  A/c only. One Payment is counted by its base type, though its voucher type is "Rent Payment". An optional Sales
  voucher is out of the books, so Sales is not listed. There is a Trial Balance and one document.
- `edge-books/be_plural.json`: two cash ledgers and three bank ledgers. One bank ledger is under each bank group,
  and one has a chain through both groups, which is counted once (that chain is set by hand and contradicts the book's own groups table, where Bank OD A/c is a loans group: both sides read the stored chain). All eight voucher types of the CA's order appear,
  written in the book out of that order, each with its count. Three other types, with no ledger entries, follow
  them by name. A cancelled Sales voucher is left out. There is a Trial Balance and every document name the
  reference's pack can pass, in its order.
- `edge-books/be_cash_only.json` and `edge-books/be_bank_only.json`: a cash ledger and no bank ledger, and a bank ledger
  (under Bank Accounts) and no cash ledger, each beside the other group's master existing and holding nothing, one
  Payment, a Trial Balance and no document. They show the cash book part and the bank books part each listed
  alone, so neither part's guard can read the other's count.
- Not reached: a voucher of unknown status. The reference refuses to form the population there and the port
  refuses with `UnknownVoucherStatus`, but an edge book cannot express that status.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about
  reading Tally.

## How they were produced

At the reference engine (a private repository), commit `976ba9dd`, under Python 3.13. Its later commit `5fcc6136`
changes only `loans_interest`. From the crate directory:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.books_examined.json \
        --test books_examined --documents-read tests/fixtures/synthetic-documents-read.json
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The five books were written by a small generator, as data. Each Trial Balance is the sum of the in-books
vouchers' lines, so no book invariant fires. With the same extraction, `parity/edge_golden.py` (with this
change) regenerates every edge golden already pinned byte-identical except
`edge.loans_interest_nil_two_loans.loans_interest.json`. That golden was made at `5fcc6136`, and its difference
is the one `loans-nil-two-loans-1259.md` records for `976ba9dd`. `golden/synthetic.read_scope.json` also
regenerates byte-identical.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `synthetic-documents-read.json` | 53 | `42df0db2ebf2bf1922e54eb61164d6083be3065860e267e6926868849520ba7a` | `synthetic-documents-read.json` |
| `synthetic.books_examined.json` | 3,440 | `e52ccca4526ce66092548671178a1f3559cc3008a30d79aa2a4ef3af54818edc` | `golden/synthetic.books_examined.json` |
| `be_none.json` | 819 | `fca69a7e2f55d76cef15529cc2e90aff45b5cc7c94951ded78715ef947bb3148` | `edge-books/be_none.json` |
| `edge.be_none.books_examined.json` | 2,570 | `c60c1d85e947ed7c891c5be121d2d45d5c4884182f5208a61028ea90ee7d2b0d` | `golden/edge.be_none.books_examined.json` |
| `be_singular.json` | 2,324 | `b0f4c1e203ecff6f86618da4c1ad700c748fdb507bd9315d3ed6a87d7ff69446` | `edge-books/be_singular.json` |
| `edge.be_singular.books_examined.json` | 2,737 | `c897917514d1f8b667642e107464fb59dab0260df90cb4206876248dd1647990` | `golden/edge.be_singular.books_examined.json` |
| `be_plural.json` | 6,539 | `47b732dcd3d75cc1ec0e4bac46aba1717795e4013b70b560d6be2f7d0e3cc649` | `edge-books/be_plural.json` |
| `edge.be_plural.books_examined.json` | 3,138 | `4238f632fab54133f6750f4906e23d5ac2bbc3155f6f2bc80b93a83f8f89ec26` | `golden/edge.be_plural.books_examined.json` |
| `be_cash_only.json` | 1,378 | `d21c9d13c10b57c91285641e7bb84b60fc70a9bf6bb6a3e9bdd378c74783666e` | `edge-books/be_cash_only.json` |
| `edge.be_cash_only.books_examined.json` | 2,698 | `e5fd202a40943e433c507170b3c60b794737fed863b478b09c39c2013567de7a` | `golden/edge.be_cash_only.books_examined.json` |
| `be_bank_only.json` | 1,387 | `50244b8cc507d2355b11b8dc41fa02f72ae32f50d80b4e16ecb6c5f4d0ddf438` | `edge-books/be_bank_only.json` |
| `edge.be_bank_only.books_examined.json` | 2,700 | `d36773c24d91d503bc475c5f77aadb2e5e4b3a530c51e8cdd48443489efa3b05` | `golden/edge.be_bank_only.books_examined.json` |
