# Batch D1 fixture provenance: `loans_interest`

Lane B, 2026-09-23. Every book here is invented: no fixture is a Tally read of any real assessee, and every
lender and ledger name is invented ("Invented Finance Ltd", "NBFC Loan", ...).

## What these fixtures establish, and what they do not

- `golden/synthetic.loans_interest.json` is the reference's dump on the synthetic read, whose config names
  one loan with no interest ledger: two Clause 31(a) cash receipts and the deductor-status finding of an
  individual with no previous-year turnover.
- `edge-books/loans_interest_core.json` (a firm) reaches every mode and Note 1 code (cash, bank, journal,
  other; A, B, I, J, K, and none for bank); s.194A over the threshold for an NBFC lender and not for a
  bank; the running balance, so that a receipt below the limit on its own is flagged once the balance
  reaches it; the s.269SS and s.269T flags; an insurer (reportable and flagged), a co-operative bank
  (reportable, never flagged) and two reporting-exempt lenders (a bank and a Government body); a clipped
  debit opening; a repayment reportable only through interest credited and not yet paid; a narration
  with both quote characters; a Contra; and an optional voucher.
- `edge-books/loans_interest_individual_over.json` and `..._individual_at.json`: an individual one paisa
  over and exactly at the previous-year turnover limit (a deductor; not a deductor).
- `edge-books/loans_interest_shared.json` and `..._shared_reversals.json` are one book: a declared-shared
  interest ledger with a paired loan's interest journal (left out), overdraft interest, a reversal-shaped
  Journal credit, a Receipt credit, a same-voucher credit, a Contra and an EMI split. The first carries no
  `net_reversals` key, so it runs through `run()` and `check_invariants()` and pins the rule in force
  (no credit reduces the figure); the second sets `net_reversals`, reaching the dormant
  reversal rule in `run` and in LOAN-3 alike (one credit of 300000 paise nets, matched to the earlier
  of two equal debits on the same day by entry order).
- `edge-books/loans_interest_invariants.json` makes the module invariants fire: LOAN-1 (a Contra into a
  loan; a loan with no Trial Balance row), LOAN-2 (an EMI split, an interest journal net of TDS, a GUID
  shared by two population vouchers, an unbalanced interest journal) and LOAN-3 (more than five interest
  debits outside any loan on a ledger not declared shared; a shared ledger whose TB debit does not tie).
- `edge-books/loans_interest_tds_coverage.json` (a firm; added at the re-pin below) reaches s.194A coverage:
  interest booked net of a TDS-payable ledger and counted gross, then a TDS catch-up journal, covering the
  rate (a TDS-seen finding, no s.194A finding); TDS short of the rate (partly covered); TDS dated before any
  interest, which covers none of it; a LIC and a financial-corporation lender, exempt under the current
  rules, over the threshold with no TDS (no finding); two interest ledgers on one loan, one also classified
  TDS payable, which never makes it a TDS line; and one TDS journal debiting two loans, counted for
  neither, which LOAN-1 names for both.
- `edge-books/loans_interest_questions.json` (an individual whose previous-year turnover is a placeholder
  between the two limits, activity not recorded: status unknown; added at the re-pin below) reaches: an
  expense credited to a loan and its reversal, each asked about as the lender's charge; an entry against
  only another loan's interest ledger (and LOAN-3 on that ledger); a bank debit and its return by
  identical narration, one by a number in the credit's reference, and a repeated narration; a repayment
  past the balance, then a cash loan tested on its own amount, then one the two walks disagree on (listed
  as not computed); a repayment reportable under both walks and flagged only under the second; a
  Government loan taken (reported) and repaid (not); one lender on two ledgers whose names match only
  case-folded; two ledgers with no lender; and the unlisted-loan notices (a loan left out of the list;
  instalments to a creditor in three months; none for suspense in two months, a duties ledger, a bank OD
  or a ledger whose group chain is incomplete).
- Not reached here: LOAN-2's "cites vouchers outside the books population", and the unresolvable-tag
  messages, which need a result the builder never produces. Two vouchers with one GUID on one loan in
  one direction repeat a figure id: the reference raises and the port refuses; a unit test in
  `src/loans_interest.rs` covers it, as no golden can.

## Real books (local only; nothing from them is in this repository)

`examples/local_parity` compared the port with the reference at `76310f60` on three real client reads,
each with that client's own reference-engine config: byte-identical dumps and 0 differences on the two
books with configured loans (210 figures and 42 findings, with 8 interest ledgers and one shared ledger
bound by identity; 49 figures and 6 findings), and 0 module-invariant violations on both. The third
has no configured loan: its 5 figures are identical, and the floor refuses it as vacuous.

The figure floor in `registry.rs` is structural: every book gives 5 figures and each configured loan
6 more, so a run with fewer than 11 compared no loan.

## Reference commit and invocations

Current pin: `87e2f03e0f5687692c638c17b66b92341c99534f` (the re-pin below). Every golden below was
regenerated from an archive of that commit (its `tae/` and `selftest/` only) by the invocations below,
with ENGINE the archive of the current pin. History: produced at `76310f60`, re-checked byte-identical at
`9d64c743`, re-pinned at `250eaedf` (the second CA-facing wording pass, text only) and at `6813a635` (the
s.269SS/269T limit written in rupees, one definition per golden); `../PROVENANCE.md` records those
regenerations, and the re-pin below is recorded here.

### Re-pin at the reference's current head (Lane E2, 2026-09-28)

Between `6813a635` and `87e2f03e` the reference's `loans_interest.py` grew from 911 to 1,701 lines:
interest booked net of a TDS-payable ledger (R1), s.194A coverage read per loan and date-aware, several
interest ledgers per loan, R2 and R3, the charge-shaped, returned-debit and repeated-narration questions,
the unlisted-loan notices, limb (a) of s.269SS on a taken row's own amount (F9) and the two walks of a
principal balance below zero. The port follows it (`src/loans_interest.rs`), with the deductor status
at the current rule. `parity/python_golden.py` and `parity/edge_golden.py` now pass the TDS-payable
ledgers, `[deductor].activity` and whether the turnover is a placeholder, each through the reference's
own readers, and an edge book's ledger may carry `chain_complete`. All seven earlier goldens change; the
two books above are new.

First produced at the reference engine commit `76310f60a300d6172efdf11a9c7158cd51f38497` (loans_interest's
never-subtract rule of `02487f42` and bind_config's re-keyed interest ledger), from an archive of that
commit with no client data, under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.loans_interest.json \
        --test loans_interest
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/loans_interest_NAME.json tests/fixtures/golden

Re-checked at reference commit `9d64c7436deedd8136b71d87d41dd36eb82733e1` (CA-facing wording in other
tests): `loans_interest.py` and every module it imports are unchanged between `76310f60` and `9d64c743`,
and regenerating all seven goldens there gives byte-identical files.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `loans_interest_core.json` | 8,960 | `d94365352c3225d2ab9acc8213836237911923472d8dc22b07c17d2178ae420d` | `edge-books/loans_interest_core.json` |
| `loans_interest_individual_at.json` | 2,246 | `23f1d587147d0e8163b80cf91c96193c86f086f2b1b8ed7288d37910f9a83c0e` | `edge-books/loans_interest_individual_at.json` |
| `loans_interest_individual_over.json` | 2,250 | `2fb2d55b6a6c1522a1dc094ec37e1bc3d962bf0e84b427de0a3fa9efaec8d875` | `edge-books/loans_interest_individual_over.json` |
| `loans_interest_invariants.json` | 6,570 | `5ddaf6edd94c6fc0dfe087c7eba3f35fe54c7139e1dc5d2092837e1c7b346e87` | `edge-books/loans_interest_invariants.json` |
| `loans_interest_questions.json` | 16,093 | `534df6181af1e7375323f2d4fc0a636c48ec5948ed7d0ea5002922150a69d783` | `edge-books/loans_interest_questions.json` |
| `loans_interest_shared.json` | 3,883 | `f714219e6bf0aa29a66e109cc1be1523b6549e686c5bf4521a60b871b238b90b` | `edge-books/loans_interest_shared.json` |
| `loans_interest_shared_reversals.json` | 3,907 | `3e58ab5517e605380cbf4a2f502e700f62fd87ee9ecac4441149286790f88208` | `edge-books/loans_interest_shared_reversals.json` |
| `loans_interest_tds_coverage.json` | 10,027 | `d83628c76e4857993dd6fb48f64e6949bf2d05265c06a835e0ad5491a8f04c01` | `edge-books/loans_interest_tds_coverage.json` |
| `edge.loans_interest_core.loans_interest.json` | 76,187 | `c61674345bd5050a0f29d34fdf84d2ed98e7362a398627c0f2d0b68f23d4f01b` | `golden/edge.loans_interest_core.loans_interest.json` |
| `edge.loans_interest_individual_at.loans_interest.json` | 14,848 | `0041167f87e001afcba90e77d9d163a7343b9172bbf84bf45ca6efb8aa87eae6` | `golden/edge.loans_interest_individual_at.loans_interest.json` |
| `edge.loans_interest_individual_over.loans_interest.json` | 13,610 | `1e814b00f322fdbacfe26a1bf98a692962c2a2cc0bac563b2a8e4bf478dca7c7` | `golden/edge.loans_interest_individual_over.loans_interest.json` |
| `edge.loans_interest_invariants.loans_interest.json` | 31,921 | `fc1d2903cfc25759ade4b32fa67eca55dedee8b780e6b444a39620ddf8476a54` | `golden/edge.loans_interest_invariants.loans_interest.json` |
| `edge.loans_interest_questions.loans_interest.json` | 131,670 | `9ae0f750094c13d037982013f9c0feaf75637c63eb221bf4574cf332bbb4ad10` | `golden/edge.loans_interest_questions.loans_interest.json` |
| `edge.loans_interest_shared.loans_interest.json` | 14,674 | `68cbeebc7cbc91b870512a9e9ddd776a905af2da638398df8f327a461b948cbe` | `golden/edge.loans_interest_shared.loans_interest.json` |
| `edge.loans_interest_shared_reversals.loans_interest.json` | 15,663 | `7bbb99dcca92484cfc155ce9b81e86c051c2e16e1d8d83da3f2da06e2b3a0190` | `golden/edge.loans_interest_shared_reversals.loans_interest.json` |
| `edge.loans_interest_tds_coverage.loans_interest.json` | 58,322 | `f7d4eb83bd868dc5f5dd1a44a5b1ad8ada93b30620f7c1638f0fcc04d26881f6` | `golden/edge.loans_interest_tds_coverage.loans_interest.json` |
| `synthetic.loans_interest.json` | 21,113 | `1e1822b391809d96a48b685c0c393f0a519acc7a9f343909cf87bb47c38fb16c` | `golden/synthetic.loans_interest.json` |