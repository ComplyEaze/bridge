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
  TDS payable, which never makes it a TDS line; interest whose s.194A rate ends in half a paisa (rounded up),
  TDS on a separate voucher the same day (read after the interest) and a TDS reversal, covered only within
  the rupee of tolerance; and one TDS journal debiting two loans, counted for neither, which LOAN-1 names
  for both.
- `edge-books/loans_interest_questions.json` (an individual whose previous-year turnover is a placeholder
  below both limits, activity not recorded: not a deductor, made unknown by the placeholder; added at the
  re-pin below) reaches: an
  expense credited to a loan and its reversal, each asked about as the lender's charge; an entry against
  only another loan's interest ledger (and LOAN-3 on that ledger); a bank debit and its return by
  identical narration, one by a number in the credit's reference, and a repeated narration; a repayment
  past the balance, then a cash loan tested on its own amount, then one the two walks disagree on (listed
  as not computed); a repayment reportable under both walks and flagged only under the second; a
  Government loan taken (reported) and repaid (not); one lender on two ledgers whose names match only
  case-folded; two ledgers with no lender; and the unlisted-loan notices (a loan left out of the list;
  instalments to a creditor in three months; none for suspense in two months, a duties ledger, a bank OD
  or a ledger whose group chain is incomplete, though its partial chain reaches Loans (Liability)).
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

Current pin: `da9e2d3d85fa68c5298643ec5c6491ae49729a39` (the bridge#802/#803 re-pin below; before it, `b0a4f91aa84dd5cc52a1fa5ae05ba0cc92d31459`, the Phase A re-pin, and `87e2f03e0f5687692c638c17b66b92341c99534f`). Every golden below was
regenerated from an archive of that commit (its `tae/` and `selftest/` only) by the invocations below,
with ENGINE the archive of the current pin. History: produced at `76310f60`, re-checked byte-identical at
`9d64c743`, re-pinned at `250eaedf` (the second CA-facing wording pass, text only) and at `6813a635` (the
s.269SS/269T limit written in rupees, one definition per golden); `../PROVENANCE.md` records those
regenerations, and the re-pin below is recorded here.

### Re-pin for bridge#802, bridge#803 and the s.194A bounds (2026-10-05)

Between `b0a4f91a` and `da9e2d3d` the reference's `loans_interest.py` changed in three places, each ported
here (`src/loans_interest.rs`):

- bridge#803 (its `0a2ded0a`): a voucher whose other lines are only TDS ledgers and other configured loans is
  a TDS journal only when no other loan is on the opposite side; a transfer between two loans carrying TDS
  is a row on each loan.
- bridge#802, in its closing form (its `38846495`): the walk marks each row with whether a principal
  repayment on or after the first interest credited to the loan came before it. A loan taken so marked that
  reaches the s.269SS/269T limit on the breach balance (principal plus interest credited and not yet paid)
  but not on the walked principal is listed as not computed (`not_computed/interest_first_...`), outside
  the reportable totals and the flag count, with a possible s.269SS where the upper bound would flag it.
- The s.194A threshold (its `6c2d6be2`): read on per-line bounds of the listed interest, from the total
  with every listed reversal line to the total with every listed credit line, so a listed pair netting to
  nil still leaves the threshold open; mixed lines get their own wording.

Between them the reference also made and reverted a certain-shortfall gate (`5e8462f5`, `f3e4adce`,
reverted by `7c3e2fae`) and a net-amount reading of the other loan (`5bcc397e`, reverted by `60be2e2a`);
neither is ported, as neither is in `da9e2d3d`.

- Seven goldens change in two count definitions only (`clause31_not_computed_row_count`,
  `s269ss_269t_possible_not_computed_count`): `phase_a`, `phase_a_refund_no_rate`, `phase_a_s194a`,
  `phase_a_unbalanced`, `phase_a_unknown_deductor`, `questions` and `walks`. No other golden, and no figure
  value or finding in any existing golden, changes; `synthetic.loans_interest.json` regenerates
  byte-identical.
- `edge-books/loans_interest_interest_first.json` (a firm; bridge#802) reaches: a loan taken in cash after
  interest was credited and the principal repaid, under the limit on the principal walk and over it on the
  breach balance (listed, a possible s.269SS); the same with interest and repayment on one day (listed:
  on or after the first interest); and controls that are not listed: no interest ledger, interest credited
  only after the repayment, an interest reversal before any interest is credited, and the principal alone
  over the limit (computed as before). The First shape taken by bank is listed but not a possible s.269SS.
- `edge-books/loans_interest_loan_transfer_tds.json` (a firm; bridge#803): a journal moving one loan to
  another lender's loan with TDS on it is a repayment of one and a loan taken on the other, both flagged in
  journal mode; a TDS journal on one loan stays neither taken nor repaid.
- `edge-books/loans_interest_s194a_bounds.json` (a firm with no TDS; s.194A threshold Rs 10,000): a listed
  pair crediting and reversing one amount, on two vouchers or on one voucher beside a non-interest line,
  leaves the threshold open where the net reading did not; the least exactly at the threshold is not over
  it; listed lines all credits keep the earlier wording; over the threshold in every reading, the s.194A
  finding stays. LOAN-2 fires on each listed voucher carrying interest, as in the Phase A books (the
  reference's own check, unchanged).
- Produced at `da9e2d3d` (5 Oct 2026) from an archive of its `tae/` and `selftest/` only, by the
  invocations below. Each of the three new books also regenerates at `b0a4f91a`'s content (the reference's
  `a50bf6d0`, the same `loans_interest.py`) to a different golden, so each reaches a changed behaviour.

### Re-pin at #779 Phase A (Lane E3, 2026-09-28)

At `b0a4f91a` the reference's `loans_interest.py` (1,701 to 2,092 lines) lists a voucher that both credits and
debits a loan as the books hold it, never netted: a record per such voucher (each side's roles, a possible
s.269SS/269T, a forced set-off or its imbalance), the loan's later entries below the limit listed as depending
on it, its maximum balance not computed, and its sides kept in the credit and debit totals but in no reportable
total. Where such a voucher carries interest or TDS, the s.194A threshold or coverage can be open ("not
computed"); TDS reaching the rate by amount but not by date reads "covered by amount, not by date". A
`tds_payable_ledgers` figure is new on every book, so every loans golden changes. Six invented books are new:
- `edge-books/loans_interest_phase_a.json`: listed vouchers with bank, cash, expense and another loan's interest
  on their other lines; possible s.269SS, s.269T and both; forced set-offs; entries on or after a listed
  voucher's date depending on it (one naming two vouchers), and one at the limit computed without a balance
  figure; a balanced loan-only voucher not listed; an interest-and-TDS voucher keeping the interest reading;
  coverage not computed; the maximum balance not computed.
- `edge-books/loans_interest_phase_a_s194a.json`: the s.194A threshold open with and without the listed interest,
  coverage open where TDS is seen, a certain default kept beside a listed voucher, "covered by amount, not by
  date", and an exempt lender with no s.194A record.
- `edge-books/loans_interest_phase_a_unbalanced.json`: a loan-only voucher that does not balance (its imbalance,
  no set-off) and a bank repayment depending on it.
- `edge-books/loans_interest_phase_a_refund_no_rate.json` (a firm; the rules without `[tds_rates]`): a row the two
  walks disagree on, on a loan with a later listed voucher, so its record says that voucher was not compared; s.194A
  coverage "not judged" beside listed interest, with no rate in the rules; and coverage still "not computed" where a
  listed voucher carries TDS, with its s.194A record.
- `edge-books/loans_interest_phase_a_unknown_deductor.json` (an individual with a placeholder turnover, so the
  deductor status is unknown): the s.194A records for an open threshold and an open coverage.
- `edge-books/loans_interest_walks.json` (an individual whose previous-year turnover is under the limits, taken
  as given with no status or activity recorded, so not a deductor; added after the Phase A nightly): interest over the
  s.194A threshold with no TDS raises no s.194A finding for a non-deductor; and a loan opening in debit, then
  taken, interest reversed against it and a cash repayment, which is reportable on both walks while its
  s.269T flag is not computed (the netting walk's breach balance is under the limit, the floored walk's over).

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
| `loans_interest_questions.json` | 16,159 | `6511dbaf9cee8e2c0586099715bb90a48793b7eb1ae8449c280085f25a74e823` | `edge-books/loans_interest_questions.json` |
| `loans_interest_shared.json` | 3,883 | `f714219e6bf0aa29a66e109cc1be1523b6549e686c5bf4521a60b871b238b90b` | `edge-books/loans_interest_shared.json` |
| `loans_interest_shared_reversals.json` | 3,907 | `3e58ab5517e605380cbf4a2f502e700f62fd87ee9ecac4441149286790f88208` | `edge-books/loans_interest_shared_reversals.json` |
| `loans_interest_tds_coverage.json` | 11,385 | `d1a32587660d4f9de320f0ba575c38d8321a067159195dd80b7a784c2bc1f175` | `edge-books/loans_interest_tds_coverage.json` |
| `edge.loans_interest_core.loans_interest.json` | 78,702 | `46e30e4b1bf493c91407fdc675d6f8cdfa51f2fa14a65556722adde65108356f` | `golden/edge.loans_interest_core.loans_interest.json` |
| `edge.loans_interest_individual_at.loans_interest.json` | 15,815 | `d9cacbf44985888197ec43e933844c349afb6f9de5c09c9c89c6fbf0487c89d5` | `golden/edge.loans_interest_individual_at.loans_interest.json` |
| `edge.loans_interest_individual_over.loans_interest.json` | 14,577 | `d42bdc09a333cdd70ce8fee54ad376a93a882fb1d174869c91c5bb6b26e4dbf6` | `golden/edge.loans_interest_individual_over.loans_interest.json` |
| `edge.loans_interest_invariants.loans_interest.json` | 33,404 | `259634170b9077c57cd72bb6f8ca53a2a66d0296fff3ec5003cdc4d28c10efea` | `golden/edge.loans_interest_invariants.loans_interest.json` |
| `edge.loans_interest_questions.loans_interest.json` | 135,486 | `e7d49f9c089658ee3f6e1d327261ba0629658521f002b64177b8996f4f361419` | `golden/edge.loans_interest_questions.loans_interest.json` |
| `edge.loans_interest_shared.loans_interest.json` | 15,641 | `347dfb628fcc4de2023776d0f8033d3c4a6b91a0cc8d1f7ff49c7421137d888e` | `golden/edge.loans_interest_shared.loans_interest.json` |
| `edge.loans_interest_shared_reversals.loans_interest.json` | 16,630 | `80ed225dc149bb0a861a6679b1bbe2c015f4bb90e65155b5dcb81dee481e378b` | `golden/edge.loans_interest_shared_reversals.loans_interest.json` |
| `edge.loans_interest_tds_coverage.loans_interest.json` | 68,199 | `ab8f851a6e7616c81c95c8f35f301a0a825dc390ec6561e96666ac8d30f796fe` | `golden/edge.loans_interest_tds_coverage.loans_interest.json` |
| `synthetic.loans_interest.json` | 22,080 | `3c5402f9211b75092eb66b976fdfcc92cf02b27f1f19b2dea7b877f6a2377c69` | `golden/synthetic.loans_interest.json` |
| `loans_interest_phase_a.json` | 11,102 | `8a26c7645ec4b0deb6081d96e3c028cf53d019f78e3586d61d3a257aceb914f8` | `edge-books/loans_interest_phase_a.json` |
| `loans_interest_phase_a_s194a.json` | 10,335 | `2cfd9fb1ad4f59aee46cbc4f3e729b0915f21281a225ef0c3e68e690c8a66134` | `edge-books/loans_interest_phase_a_s194a.json` |
| `loans_interest_phase_a_unbalanced.json` | 1,838 | `e46eb5bfd5bac1cc5bd68f0ae1b37608b8ec9ac66ee0256a840531ec4157fc47` | `edge-books/loans_interest_phase_a_unbalanced.json` |
| `edge.loans_interest_phase_a.loans_interest.json` | 122,763 | `af646fe37cc7b402dd156a132f60d62be79ce24657de35c298d499ece3039965` | `golden/edge.loans_interest_phase_a.loans_interest.json` |
| `edge.loans_interest_phase_a_s194a.loans_interest.json` | 94,994 | `10e7576a6a86ff8bf2dce072dd84c06835efbc00ce71860dd4c12a1b8438fcad` | `golden/edge.loans_interest_phase_a_s194a.loans_interest.json` |
| `edge.loans_interest_phase_a_unbalanced.loans_interest.json` | 19,454 | `95e219d47d5f3b5e179df42b67ebad2d0b672694082c8d16bc12158887ce121f` | `golden/edge.loans_interest_phase_a_unbalanced.loans_interest.json` |
| `loans_interest_phase_a_refund_no_rate.json` | 5,820 | `f6ae1a827dfa5ab45a4e37408229376fe2351485f067b5f5d23b713459277c0d` | `edge-books/loans_interest_phase_a_refund_no_rate.json` |
| `loans_interest_phase_a_unknown_deductor.json` | 4,533 | `1a7b049ac2b7b66a1c1b6309a78b06b26054cf95642f8a99241ef7107f0f259e` | `edge-books/loans_interest_phase_a_unknown_deductor.json` |
| `edge.loans_interest_phase_a_refund_no_rate.loans_interest.json` | 48,356 | `403314ed1937efdf6c10416a9ed2a68446004f44152c6445d661818f207ac075` | `golden/edge.loans_interest_phase_a_refund_no_rate.loans_interest.json` |
| `edge.loans_interest_phase_a_unknown_deductor.loans_interest.json` | 36,285 | `51fca9cb6a607ddeab0ee44a277a016a694956047190b32df685892764ae6a3b` | `golden/edge.loans_interest_phase_a_unknown_deductor.loans_interest.json` |
| `loans_interest_walks.json` | 3,973 | `977ea16a4d85add2e98a814a792b94072bcac30d02a612fe1335b5806e7f3a58` | `edge-books/loans_interest_walks.json` |
| `edge.loans_interest_walks.loans_interest.json` | 23,320 | `e659af2833275c8c0dde461c77c3a8e9d552fabe4cbdeff7b7e9b6310b16a120` | `golden/edge.loans_interest_walks.loans_interest.json` |
| `loans_interest_interest_first.json` | 9,807 | `f1a4dcbb5f31854bf4cc4f0b0c1a12adafbb019ae5a9a5ec147d59bb28a43195` | `edge-books/loans_interest_interest_first.json` |
| `edge.loans_interest_interest_first.loans_interest.json` | 75,624 | `50b5a48fbc99252429ac5a6b1d7771732228c3e9420418812caa398c77c28f7b` | `golden/edge.loans_interest_interest_first.loans_interest.json` |
| `loans_interest_loan_transfer_tds.json` | 3,050 | `5ce7a271ad8a227b88252e8e17304276ac7c0832e2db691ccd2e9aa91e55294c` | `edge-books/loans_interest_loan_transfer_tds.json` |
| `edge.loans_interest_loan_transfer_tds.loans_interest.json` | 27,128 | `c88d8c96d5e84d7f1a96bd53ee76ad519b627e863d5ef04d0560c0c514337206` | `golden/edge.loans_interest_loan_transfer_tds.loans_interest.json` |
| `loans_interest_s194a_bounds.json` | 7,995 | `82fdd737fb17d19ba7255a977ba2044610d9e09d790d4301e97ab260b3561bb7` | `edge-books/loans_interest_s194a_bounds.json` |
| `edge.loans_interest_s194a_bounds.loans_interest.json` | 93,133 | `3929b7dc283b3c01639222cc773118dfc33f361640479f8f531ccaea50126ff9` | `golden/edge.loans_interest_s194a_bounds.loans_interest.json` |
