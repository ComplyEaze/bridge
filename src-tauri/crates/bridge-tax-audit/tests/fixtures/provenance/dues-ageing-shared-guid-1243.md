# Fixture provenance: statutory_dues_43b and creditor_ageing_43bh vouchers that share a GUID (#1243)

Every book here is invented, with small round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

The reference changed, at its commit `21e1d147`, which vouchers `statutory_dues_43b` and `creditor_ageing_43bh` cite.
Both walks summed every line, but kept the vouchers to cite by GUID alone, so of two vouchers sharing a GUID (a blank
GUID, or one GUID repeated) only the later was cited: the earlier was counted in the figure and not named in its
evidence. In `statutory_dues_43b` the payments cited for clause 26(i)(A) were filtered from the same GUID-keyed
vouchers, so a payment whose GUID a later charge shared was cited nowhere. Now each walk keeps its vouchers by a key
no other voucher of the population shares (the GUID where that is unique, so nothing that reads it moves, else the GUID
and its place among those sharing it), and each figure and finding cites each voucher's own ref, its GUID and label:
distinct refs in (GUID, label) order, so two vouchers with one GUID, number, type and day are counted twice and cited
once. No figure or finding value moves, on any book: only evidence.

Of the nine goldens of the two tests pinned before this, two change, each in evidence only: the two books that already
held two vouchers sharing one GUID.

- `golden/edge.statutory_dues_more.statutory_dues_43b.json`: six figures (`charged_`, `paid_`, `deposited_on_time_`,
  `deposited_late_`, `not_visible_after_year_end_` and `unpaid_due_date_passed_pf_employee`) and the finding
  `unpaid/pf_employee` now also cite `Journal DUP-FIRST on 2025-08-31` beside `Journal DUP-SECOND on 2025-09-30`.
- `golden/edge.creditor_ageing_short.creditor_ageing_43bh.json`: four figures (the creditor's `creditor_reconstructed_`,
  `creditor_advance_`, `creditor_over45_` and `creditor_over15_`) and two findings (`over45/<tag>` and
  `opening_dues_26a_candidate`) now also cite `Purchase DUP-FIRST on 2026-03-05` beside `Purchase DUP-SECOND on
  2026-03-06`.

Their rows are in `batch-2a.md`. The other seven (`edge.statutory_dues`, `edge.statutory_dues_calendar`,
`edge.statutory_dues_coverage`, `edge.creditor_ageing`, `edge.creditor_ageing_plain`, `synthetic.statutory_dues_43b`
and `synthetic.creditor_ageing_43bh`) regenerate byte-identical at the commit below (checked 7 Oct 2026). Each of the
four goldens below differs at the reference's previous commit (`cf8b4d77`), in evidence only.

- `edge-books/statutory_dues_shared_guid.json` (natures other than employees' contributions). On output GST with an
  opening liability: a payment whose GUID a later charge shares (the payment is cited as paying the opening liability,
  the charge is not); two blank-GUID payments and a blank-GUID charge with no number; two payments with one GUID,
  number, type and day (one ref, both counted); a blank-GUID voucher whose line on the ledger is nil (not cited); a
  blank-GUID voucher on two natures (cited on each); a GUID differing from another only in case; a third voucher on a
  shared GUID that touches no mapped ledger (cited nowhere). On reverse-charge GST: an opening liability, two
  blank-GUID charges and a charge that also carries a nil line on the nature's second ledger, with no payment, so the
  clause 26(i)(A) figure cites no voucher (a nil line on a ledger of the nature is no payment). On TDS payable: figures only. 18
  figures, 5 findings; at the previous commit the payments cited for clause 26(i)(A) were two for five.
- `edge-books/statutory_dues_shared_guid_employee.json` (employees' contributions). PF: an opening lot paid by two
  blank-GUID payments, one by its due date and one after; one GUID on May's deduction and on its late payment; one
  GUID, number, type and day on July's two deductions; a blank-GUID payment with no number left as an unmatched
  advance. ESI: one GUID on a payment of the opening lot and on a later deduction; an opening remainder still unpaid;
  May's deduction in the PF voucher, whose payment (sharing that GUID, with no ESI line) is not cited on ESI; two
  blank-GUID March deductions with identical refs, due after the year end. Every figure of the due-date walk and each
  of its six kinds of finding is reached on a nature holding shared GUIDs. 25 figures, 7 findings.
- `edge-books/statutory_dues_ref_order.json` (the refs themselves). Three charges on one GUID whose numbers differ
  only in case; six on another GUID numbered with a letter, an accented letter composed and decomposed (two refs,
  which the canonical dump shows alike after normalising), a full-width letter and a character beyond U+FFFF, one of
  them again under a type differing only in case and again on an earlier day; two blank-GUID payments whose numbers
  differ only in case; two GUIDs differing only in case. 7 figures, 3 findings.
- `edge-books/creditor_ageing_shared_guid.json`. A micro creditor with an opening balance partly paid: one GUID on a
  bill and on its payment, and on a voucher that touches no creditor (not cited); a blank-GUID bill and a blank-GUID
  payment; one GUID, number, type and day on two bills; a blank-GUID voucher billing two creditors; a blank-GUID
  voucher whose line is nil (not cited). A small creditor with an unpaid opening balance and a bill whose ref is
  identical to the micro creditor's two, so the opening-dues finding cites it once. An unclassified creditor with
  blank-GUID bills whose numbers differ only in case and the five numbers above on one GUID. A trader: one GUID on a
  bill and on a payment over it. 33 figures, 6 findings.
- Not pinned by a golden: the order a module emits a row's refs in, because the canonical dump sorts refs itself. The
  reference emits a row's voucher refs sorted by (GUID, label), by code point; `tests/edge_books.rs` checks that on
  these four books and the two above, and holds the refs the reference emitted for one row of
  `statutory_dues_ref_order` and one of `creditor_ageing_shared_guid`, in its order.
- Not reached by a book: a GUID holding a NUL that would make two keys equal, which the reference refuses (no read
  produces one). The key is a unit test in `src/book.rs`; `tests/edge_books.rs` edits two of these books to hold such
  a GUID and requires each module's typed refusal where the reference raises.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading
  Tally.

## How they were produced

At the reference engine (a private repository), commit `ee17d80fe60e6d2734629aab7dc23ff0c3dc348d`, whose
`statutory_dues_43b` and `creditor_ageing_43bh` are those of `21e1d147134b68461d9d728b163dfd09b08fc227`, from an
archive of its engine package only, under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The four books are written as data by a small generator, every voucher balancing and each Trial Balance row its
opening plus the ledger's own lines (so S43B-1 and AGE-1 hold); they are hand-chosen scenarios, not generated from any
data. Each Trial Balance is deliberately partial: it has rows only for the ledgers these two tests read, so it does
not sum to nil.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `statutory_dues_shared_guid.json` | 5,959 | `4deafbee9edb8226c81380fed4f69a0ec65ad6c8afa257ff2f0258911cdf9600` | `edge-books/statutory_dues_shared_guid.json` |
| `statutory_dues_shared_guid_employee.json` | 4,661 | `d2bdd0c780b3591e04720cb8f4db7206e4eab962bf172959c0a791d2c14b0ba3` | `edge-books/statutory_dues_shared_guid_employee.json` |
| `statutory_dues_ref_order.json` | 4,927 | `94d8dbd36543b1adc19b7afffb0680d816c5efeaf65b3b82deddab912199c228` | `edge-books/statutory_dues_ref_order.json` |
| `creditor_ageing_shared_guid.json` | 7,077 | `9cbd5a9fbc7e8e2db6ae51c8a721ccff0b3401d1f7497a2c8ad3569b716b0106` | `edge-books/creditor_ageing_shared_guid.json` |
| `edge.statutory_dues_shared_guid.statutory_dues_43b.json` | 25,959 | `767744ba78e105a1bacaf97d9ad477a3096c2cc87b707e178039d2efae17660d` | `golden/edge.statutory_dues_shared_guid.statutory_dues_43b.json` |
| `edge.statutory_dues_shared_guid_employee.statutory_dues_43b.json` | 40,795 | `ad0eedb37fb92f0802ed60b838189a8c1270e0e9b2808be184292eccb9fd45ee` | `golden/edge.statutory_dues_shared_guid_employee.statutory_dues_43b.json` |
| `edge.statutory_dues_ref_order.statutory_dues_43b.json` | 16,420 | `4103078f73721296a7225d1542beb7f28986b4f6489ac12e3a78fcaf80e73ede` | `golden/edge.statutory_dues_ref_order.statutory_dues_43b.json` |
| `edge.creditor_ageing_shared_guid.creditor_ageing_43bh.json` | 44,028 | `601dd31f8bbb77ae108db00c5e08a7cfb42961e3ed74d8e339fc08a33d21009c` | `golden/edge.creditor_ageing_shared_guid.creditor_ageing_43bh.json` |
