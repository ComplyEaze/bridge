# tds_payees re-pin fixture provenance: seven invented edge books

Lane E2, 2026-09-27; `tds_payees_p22` and the P2-2 regeneration, Lane E3, 2026-09-28. Every book here is invented: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

Each book names, in its own `comment`, what it reaches at the reference's current head:
- `tds_payees_gross_gst`: a payee whose credits cross the aggregate only gross of the TDS on its own bills
  (Rs 99,960 net, Rs 1,02,000 gross; one bill also debiting the payee); GST left out for a payee whose agreement states it separately
  and counted for one without; a Duties & Taxes credit not classified as TDS making payees possibly over the
  s.194J aggregate and the s.194-I month; TDS on a bill crediting two payees, over the single sum only with
  it; a payee whose only TDS is a catch-up journal (Form 26A held), its small credit listed only because TDS
  is seen; an unnamed cash payee; list entries matching nothing.
- `tds_payees_21b`: clause 21(b) with no TDS seen (the tranches: a crossing that takes earlier credits with
  it, and a credit over the single sum alone), and with TDS seen (every credit; (ii)(B) where the challans
  show nothing deposited by the due date for a bill's own TDS month); a short and a mid deduction for a
  client recorded in Kerala; Form 26A held and not held; the s.194C(6) question, and a payee on no
  goods-carriage ledger that gets none.
- `tds_payees_reversals_194h`: reversals classified bill-specific, as a credit note, not at all, and as a
  duplicate of, or specific to, another payee's bill; a reversal settled partly by bank; a section that reverses and books;
  s.194H with TDS on the bill; an individual recorded as a profession, a deductor by the profession limit.
- `tds_payees_deductor_both`: both activities under the profession limit, not a deductor: nothing in
  clause 21(b).
- `tds_payees_deductor_placeholder`: a placeholder turnover, never "not a deductor"; no TDS ledger
  classified, so deduction is not judged.
- `tds_payees_deductor_activity_unknown`: an individual recorded as a profession with no receipts supplied:
  "unknown", and the question names the profession.
- `tds_payees_p22`: a bill booked gross with its TDS debited back to the payee, rate-tested on its value
  before TDS and, deducted on its own voucher, listed on the payee's TDS question instead of in clause
  21(b)(ii)(A), that listing alone raising the s.194C(6) question; a payee debit other than the bill's own TDS
  (untested, counted); a later voucher debiting the TDS ledger against a payee, which keeps its deducted bill
  in (ii)(A) (the reversal guard); and a cash-paid bill carrying TDS with no payee named (untested, counted).

Regression fixtures only: the evidence for real books is local parity on the real reads, never committed.
The refusals the reference raises (a reversal naming a bill credited to no payee, a 194H mapping with no
`[s194h]`, a goods-carriage ledger not mapped to 194C, and each malformed config list) are unit tests in
`src/tds_payees.rs`, not books.

## How they were produced

The books are written as data by a small generator, and every voucher balances. The goldens came from the
reference engine (a private repository), commit `e2456bcf4f163cf770945e8620e715788db0ca46`, under Python 3.13;
after its P2-1/P2-2 change, `tds_payees_p22`'s golden and the regenerated `tds_payees_21b`, `tds_payees_gross_gst`
and `tds_payees_reversals_194h` goldens come from commit `b0a4f91aa84dd5cc52a1fa5ae05ba0cc92d31459`, at which every
other golden in this batch regenerates byte-identical:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/tds_payees_NAME.json tests/fixtures/golden

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `tds_payees_gross_gst.json` | 9,916 | `96cf35c296e957ff156311232053278606e4b0bec9e379c6f8ccba1bb3e81af1` | `edge-books/tds_payees_gross_gst.json` |
| `edge.tds_payees_gross_gst.tds_payees.json` | 71,259 | `6ae9b73cc0568d3592a630ac3934e7795a8bbbe66992dd792a5a4c86cdad3e99` | `golden/edge.tds_payees_gross_gst.tds_payees.json` |
| `tds_payees_21b.json` | 6,112 | `cbc3543088630157f0a340ddf2e8517c426a81b6f2d543d9b96c3e2f91b1f8a8` | `edge-books/tds_payees_21b.json` |
| `edge.tds_payees_21b.tds_payees.json` | 56,866 | `0d4c778a3f17a58ef8c1fa335a1cf1d104e25efb5a3f234e45413feb66be4e3b` | `golden/edge.tds_payees_21b.tds_payees.json` |
| `tds_payees_reversals_194h.json` | 6,561 | `1a51ff96db3f173046fb5a7a62cc8c7632c6e7db339f32e3596b94bf045d00ce` | `edge-books/tds_payees_reversals_194h.json` |
| `edge.tds_payees_reversals_194h.tds_payees.json` | 30,556 | `e7e48301544abc65e1c1d63c12388c6f12fce7b19263405527d90af9e6423f12` | `golden/edge.tds_payees_reversals_194h.tds_payees.json` |
| `tds_payees_deductor_both.json` | 1,520 | `87efe808595b27026aa8fb065bfda92f633f12bfead20549af917674bd625a0a` | `edge-books/tds_payees_deductor_both.json` |
| `edge.tds_payees_deductor_both.tds_payees.json` | 17,901 | `0e43833800a4f1114fe3580d4f63b9333c038666b4186e88e3fbe0114b7693e0` | `golden/edge.tds_payees_deductor_both.tds_payees.json` |
| `tds_payees_deductor_placeholder.json` | 1,632 | `6e6263056f92300780d561ccc58393faec1830d8d3d50d29dabbfc67394e240c` | `edge-books/tds_payees_deductor_placeholder.json` |
| `edge.tds_payees_deductor_placeholder.tds_payees.json` | 20,327 | `140b4e54cdfd3f13a0bc12156d50560bfd2ab83a4b5c40a849383b68d9071bc5` | `golden/edge.tds_payees_deductor_placeholder.tds_payees.json` |
| `tds_payees_deductor_activity_unknown.json` | 1,445 | `b7ea95f59f19a30fd4d2feaa5dd9800fed586d40db89dc38d8df744286afadb8` | `edge-books/tds_payees_deductor_activity_unknown.json` |
| `edge.tds_payees_deductor_activity_unknown.tds_payees.json` | 16,206 | `cf39e0c5f26bf10ceb30be03456bd2367a5438a455a682a19152720b4fbc7693` | `golden/edge.tds_payees_deductor_activity_unknown.tds_payees.json` |
| `tds_payees_p22.json` | 4,670 | `73cf9cc57d444150bfdf4d3936c24d4d22fbe42020e18a75c97aa8cc02abff3d` | `edge-books/tds_payees_p22.json` |
| `edge.tds_payees_p22.tds_payees.json` | 57,243 | `777a9ef46e23a7dc34174f30ef794bfde3ef2f6b040bc7a637c60f187023f6e2` | `golden/edge.tds_payees_p22.tds_payees.json` |
