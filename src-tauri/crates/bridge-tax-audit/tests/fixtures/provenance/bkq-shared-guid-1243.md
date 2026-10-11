# Fixture provenance: book_keeping_quality vouchers that share a GUID (#1243)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

The reference changed, at its commit `19617356`, how `book_keeping_quality` keys its per-voucher rows. Before it, the entry-order lag, the vouchers created after the last sale, the payment-channel invoices and receipts (and the greedy match between them), the re-issue narration rows and the Contra narration-versus-direction rows were each kept by voucher GUID alone, so two vouchers sharing a GUID (a blank GUID, or one GUID repeated) were one entry: the later replaced the earlier, a lag population of one base type could absorb a voucher of another, and two blank-GUID Contra mismatches became one row and one finding. Now each voucher of the population has a key no other shares (its GUID where that is unique, so nothing that reads it moves, else the GUID and its place among those sharing it); a citation still names a voucher by its GUID and label, citing each distinct ref once in (GUID, label) order, so two vouchers with one GUID, number, type and day are counted twice and cited once; and a Contra row id, a hash of the GUID, takes its place among the rows sharing it (in date order, then the key's order) as a suffix `_1`, `_2` only where it would repeat.

Of the book_keeping_quality goldens pinned before this, only `edge.bkq_order.book_keeping_quality.json` changes (its two Payments sharing a GUID are now two in the lag: the Payment lag population goes from 3 to 4, the Payments over 30 days from 1 to 2, and that finding now cites the 70-day Payment too); its book's comment and its rows in `batch-e1b.md` are updated. The other book_keeping_quality goldens, the synthetic one included, regenerate byte-identical at `5658c8ce` (checked 6 Oct 2026). Each of the three goldens below differs at `19617356`'s parent.

- `edge-books/bkq_shared_guid_order.json` (entry order): blank-GUID vouchers of four base types (Payments lagging 60, 30 and 42 days, a Journal 10, a Receipt 41, two Purchases with one number, type and day created after the last sale); one GUID on two Payments (lags 46 and 5); one GUID on two Journals created after the last sale (lags -30 and 91) and on a third voucher whose MASTERID is unparseable; a Payment sharing a sale's GUID; a blank-GUID Payment created before any sale; a blank-GUID sale moving the clock. 24 figures, 4 findings; at the parent the blank GUID was one entry, so the Payment lag population read 2 for 6 and the vouchers after the last sale 3 for 6.
- `edge-books/bkq_shared_guid_channel.json` (payment-channel invoices): two blank-GUID invoices of one amount on one day contending for one blank-GUID receipt (the first in the books takes it); one GUID on two invoices each matched to its own blank-GUID receipt; two blank-GUID invoices with identical refs and a third in their month, unmatched; one GUID on two receipts matching nothing; a channel sale line priced against the year-average purchase rate, so the margin finding cites every invoice. 23 figures, 2 findings; at the parent the invoices read 2 for 7 and the matched invoices 1 for 3.
- `edge-books/bkq_shared_guid_misc.json` (re-issue narrations, Contra direction): six re-issue rows over blank and repeated GUIDs, two with identical refs; seven blank-GUID Contra mismatches on three days, read out of date order, five on one day (two with one number) whose places among the blank-GUID vouchers cross from one digit to two, taking `_1` to `_7` in date and then books order; one GUID on two mismatches of one day (`_1`, `_2`); a GUID shared with an agreeing Contra, and a unique GUID, each keeping an id with no suffix; a blank-GUID withdrawal that agrees. 24 figures, 12 findings; at the parent the re-issue rows read 2 for 6 and the Contra mismatches 4 for 11.
- Not reached: a GUID holding a NUL that would make two keys equal, which the reference refuses (no read produces one); it is a unit test in `src/book.rs`, not a book.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading Tally.

## How they were produced

At the reference engine (a private repository), commit `5658c8ce70d57981df63c5e65172308bf166ef94` (its `book_keeping_quality` is that of `19617356`), from an archive of its engine with no client data, under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The amount of voucher PR-4 in `bkq_shared_guid_misc.json` was changed from 40000 to 40700 on both lines, and that book's golden was regenerated at the reference engine commit `ee17d80f` (the engine of this crate's later pins); the golden's bytes count is unchanged, its SHA-256 is the one in the table below.

The three books are written as data by a small generator, every voucher balancing; they are hand-chosen scenarios, not generated from any data.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `bkq_shared_guid_order.json` | 6,478 | `2b147603a66c77c064b5df5656026c195ad7ff7398457b9184128dd76ab65f0a` | `edge-books/bkq_shared_guid_order.json` |
| `bkq_shared_guid_channel.json` | 5,295 | `bdf8fcd0688199d6262e0ad0f708587ab051bbb5de55585a2e884517e6f78699` | `edge-books/bkq_shared_guid_channel.json` |
| `bkq_shared_guid_misc.json` | 6,772 | `a820ddd719e6c4967e86dce49d5296c9bad56caa2ece23f8996bebe279ce9806` | `edge-books/bkq_shared_guid_misc.json` |
| `edge.bkq_shared_guid_order.book_keeping_quality.json` | 18,764 | `612b37307f62ba2da5e398e926180bd20fae993643430641665cd4d94858d61c` | `golden/edge.bkq_shared_guid_order.book_keeping_quality.json` |
| `edge.bkq_shared_guid_channel.book_keeping_quality.json` | 15,037 | `a198c72e26dcc66f0d670cba7845334777067d824b2414af0a18e279a8308a66` | `golden/edge.bkq_shared_guid_channel.book_keeping_quality.json` |
| `edge.bkq_shared_guid_misc.book_keeping_quality.json` | 26,829 | `9e390b88913ebae92355c5626c3ae3b056ad99fe3a7f4006c793349a2ca7c928` | `golden/edge.bkq_shared_guid_misc.book_keeping_quality.json` |
