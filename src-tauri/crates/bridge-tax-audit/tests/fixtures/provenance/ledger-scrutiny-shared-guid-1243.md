# Fixture provenance: ledger_scrutiny vouchers that share a GUID (#1243)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

The reference changed, at its commit `e710a46e`, how `ledger_scrutiny` keys an expense ledger's entries. Before it, the entries and the large, round-sum, last-days and cash-paid subsets were keyed by voucher GUID, so two vouchers sharing a GUID (a blank GUID, or one GUID repeated) were one entry: the later replaced the earlier, the debit total and the subset counts and totals were short, the cash-paid sum could keep an entry the debit total had lost (a cash share over 100 percent), and the finding cited one voucher. Now each voucher of the population has a key no other shares (its GUID where that is unique, so nothing that reads it moves, else the GUID and its place among those sharing it); the figures and the finding cite a voucher by its GUID and label, distinct refs in (GUID, label) order, so two vouchers with one GUID, number, type and day are one ref. At the reference's previous commit (`66e842e7`) each of the two goldens below differs. The five ledger_scrutiny goldens already pinned (`edge.scrutiny`, `edge.scrutiny_default`, `edge.scrutiny_misc`, `edge.scrutiny_short` and `synthetic.ledger_scrutiny`) regenerate byte-identical at the commit below (checked 6 Oct 2026).

- `edge-books/scrutiny_shared_guid.json`: on one ledger, one GUID on two bank payments of Rs 60,000 in the last days of the year, each large and a round sum, beside a unique small payment: each sharer is its own entry in the entries and in the large, round-sum and last-days subsets, and the finding cites both under the one GUID, each by its own label (the canonical dump sorts refs by GUID and label, so the order the module emits them in is not pinned here); on another ledger, one GUID on two payments with the same number, type and day: two entries, one cited ref. 27 figures, 2 findings.
- `edge-books/scrutiny_blank_guid.json`: two blank-GUID payments to one ledger, Rs 4,00,000 in cash and Rs 1,00,000 by bank in the last days: two entries, a cash-paid sum within the debit total and a cash share of 80 percent (400 percent when merged); two blank-GUID cash payments in the last days to another ledger, each counted in the last-days and cash-paid subsets; a blank-GUID journal and a blank-GUID bank payment to a third ledger, two entries and not journal-only. Each finding cites both vouchers under the blank GUID. 39 figures, 2 findings.
- Not reached: a GUID holding a NUL that would make two keys equal, which the reference refuses (no read produces one); it is a unit test in `src/book.rs`, not a book.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `5658c8ce70d57981df63c5e65172308bf166ef94`, whose `ledger_scrutiny` is that of `e710a46e5bad2cb94baf610912b053a354661440`, under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The two books are written as data by a small generator, every voucher balancing and each ledger's Trial Balance debit the sum of its debits; they are hand-chosen scenarios, not generated from any data.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `scrutiny_shared_guid.json` | 2,682 | `3ce59392ed69a643da753f14219492f5e07ac959f2cd2b99494b534bbdef412e` | `edge-books/scrutiny_shared_guid.json` |
| `scrutiny_blank_guid.json` | 3,443 | `444c066a23de89f25d1d67625a4ec3b5146238a9204fd53ead94a6852c63967d` | `edge-books/scrutiny_blank_guid.json` |
| `edge.scrutiny_shared_guid.ledger_scrutiny.json` | 16,967 | `92803c6cf8d5277c788c54ef9961270a38c85cc4005762339e7f899690355510` | `golden/edge.scrutiny_shared_guid.ledger_scrutiny.json` |
| `edge.scrutiny_blank_guid.ledger_scrutiny.json` | 21,590 | `7524447baa804098d311e3186228c14808ff109b0f5f841e8ad67d701750c06b` | `golden/edge.scrutiny_blank_guid.ledger_scrutiny.json` |
