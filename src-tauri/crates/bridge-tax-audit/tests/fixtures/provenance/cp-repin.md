# Fixture provenance: the `cash_payments_40a3` re-pin (2 Oct 2026)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What changed in the reference, and what these fixtures establish

`cash_payments_40a3` was last pinned at reference commit `105b6c37`. Since then the reference changed three behaviours and several texts, and the synthetic read reaches only the texts, so two invented edge books reach the behaviours:

- **s.40A(3) rows keyed by a ledger that names what was bought.** A row whose payee ledger is under Purchase Accounts, Direct Expenses or Indirect Expenses may pool several people's cash. Its figure definition, its title and an added limit say so, and the limit quotes every usable name the day's vouchers print (or says none is usable). `cp_unnamed` reaches each of these: a purchase ledger with one printed name and one voucher printing none, a direct-expenses ledger that also trips the goods-carriage flag (its own ledger name printed, which is not usable), an expense ledger whose vouchers print one name twice (once with surrounding blanks), one whose only printed name is the cash ledger, one with one name and one unnamed, and a printed name that is the bank ledger on an expense row and on a receipt and a payment row; a printed name with an apostrophe (quoted as written, as the reference's f-string does, not as `repr` would), a named payee, and an excluded capital payee, keep the earlier wording.
- **s.269ST rows with no party ledger.** The receipt and payment rows pooled by day for vouchers that name no party take their own wording (the payers' or payees' side, "whether any one person ... is not known", a different ask) and cite the printed names on the row. `cp_unnamed` reaches a receipt row with one printed name and a voucher printing none, a payment row whose two vouchers print the same name, a row with a name that is the voucher's own Sales ledger, and a named party's receipt and payment rows for contrast.
- **s.269SS/269T candidates per voucher, loan ledger and side.** A voucher's credits to a loan ledger are summed (a loan accepted), and so are its debits (a loan repaid), and the limit is tested on each sum; the two sides of one ledger on one voucher are never netted, and the finding names the voucher by its GUID, or by its place in the population where the GUID is blank or shared. The text says when the voucher's cash moved the other way, and names any opposite entry. `cp_loans` reaches: one line; a receipt split over two lines; two lines each over the limit; both sides of one ledger on one voucher (a loan taken and repaid); a transfer between two loans on a voucher whose cash moved the other way; a voucher with no GUID, named by its number; a ledger with one side over the limit and the other under it (still suffixed by its side, because the suffix follows the sides the voucher has on the ledger, not the sides that reach the limit); a candidate whose GUID a Contra voucher also carries (told apart by its place, because the count of vouchers sharing a GUID covers the whole population); an amount exactly at the limit; a zero line on a ledger that also has a real side; an acceptance with opposite entries on two other loans; two vouchers sharing a GUID on different days (2025-07-08 and 2025-07-09), after a Contra voucher so that their places count it; a figure's evidence cites both, as the reference's does (#1134); and a voucher (k19) that meets two opposite loan ledgers in the other order from k16, so the finding's list of them is shown sorted; a repayment; a zero line; a side under the limit; a voucher whose cash nets to nothing; a voucher with no cash line; a Contra voucher; and loan ledgers configured with the client beside ones that are not.
- **Wording only, in the synthetic golden:** eight figure definitions and five findings change as the measured diff shows (the s.269SS/269T figure definitions, and the s.269ST unnamed-party rows).
- Regression fixtures only: they prove the port and the reference agree on the same inputs, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

- `cp_loan_blank`: one voucher with no GUID and no number, alone in its book: it is named "(none)" and told apart by its place in the population.

## How they were produced

At the reference engine (a private repository), commit `c62a4ab4`, under Python 3.13. The goldens below also regenerate byte-identical at its later commit `da9e2d3d` (checked 3 Oct 2026), the last change to its `tae/` and `selftest/` that day; the changes after `c62a4ab4` touch only `counter_cheques_40a3`:

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
| `cp_unnamed.json` | 7,978 | `f099436d35a0469c76a478708dcd0fe6fe0b09ee339426d0ea710276bc96ea8c` | `edge-books/cp_unnamed.json` |
| `edge.cp_unnamed.cash_payments_40a3.json` | 69,446 | `9b2b87a88249c6226f22bcc102ae06ec3fa7822794f3926ff195bc7b1d41c895` | `golden/edge.cp_unnamed.cash_payments_40a3.json` |
| `cp_loans.json` | 7,004 | `a25a79803ea038dc52fdfae10a8f0346ff594eb96bf795dbe7fce5aef7f09cb2` | `edge-books/cp_loans.json` |
| `edge.cp_loans.cash_payments_40a3.json` | 49,367 | `f552e24d20df272ca6fcac77af3a0a1e028489d3ab2c933b0d78c3c654764157` | `golden/edge.cp_loans.cash_payments_40a3.json` |
| `cp_loan_blank.json` | 1,290 | `b75ad26a2b52a035cf118704cc3194f8c56fe656e9eadd60e11b730747e01f04` | `edge-books/cp_loan_blank.json` |
| `edge.cp_loan_blank.cash_payments_40a3.json` | 10,456 | `56d0d68be3742aaeea9736595a49e6e22c693e8cbab831a0d4102492845d729c` | `golden/edge.cp_loan_blank.cash_payments_40a3.json` |

## Declared behaviour difference: a s.269ST row under the limit on its own cash line

2 November 2026, #772. `docs/tax-audit/parity-spec-v1.md` section 12 requires every change to an
existing output to be a declared difference that names the invented book which exercises it.

- **What differs.** A s.269ST row is flagged on the party's side of its vouchers. Where that side is at or over the limit and the cash on those vouchers' own cash line is below it, because bank also moved, the title said the cash was at or over the limit. It now says the party's side is at or over and the cash debited or credited on these vouchers is below it. `Row::over` of `high_value_register` decides this, so the two tests give one row one account. The amount's definition on a named row said the figure was cash; on such a row it now says the figure is the party's side of each voucher, not only its cash, the wording an unnamed row already had. The two counts of rows at or over the limit said the day's total or cash payment total; each now says the party's side of the day's vouchers, and the payment count says a row is reportable in clause 31(bc) where its cash is too. The row, its figures' values, its evidence and its limits are as before. The compared fields are `title_sha256_16` and `definition_sha256_16` (parity-spec section 3).
- **The book.** The invented book is the unit-test book of `src/cash_payments_40a3.rs`. The test `cash_payments_40a3::tests::a_row_under_the_limit_on_its_cash_line_says_so_in_its_title` builds each of the four rows, receipt and payment, named and unnamed, both ways round the limit, and asserts the title whole and the amount's definition. The counts' definitions hold on every book, so the goldens below pin them.
- **The golden.** The synthetic book has one such row, the payment of 2025-04-01. `golden/synthetic.cash_payments_40a3.json` carries the new title, the new amount definition and their hashes, recomputed by hand with the hash `canonical.rs` uses (each old hash was reproduced first, to check the method). The four `edge.cp_*` goldens hold no such row, and only the two counts' definitions and hashes move in them, as in the synthetic golden. Nothing else changed, so these five are no longer the reference's own dumps for those texts. Their byte rows in `PROVENANCE.md`, here and in `shared-guid-rows-1195.md` are updated in place.
- **Why it was not regenerated.** `parity/python_golden.py` takes the reference engine's directory, which is not in this repository, and the reference still gives the old texts (inferred from this golden, not measured), so a regeneration would bring it back.
- **What a local parity run shows.** A run of `cash_payments_40a3` on a real book holding such a row reports its `title_sha256_16` and its amount's `definition_sha256_16` as differences, and a run on any book the two counts' `definition_sha256_16`. These are this declared difference, not a regression.
- **Where it is declared.** The divergence registry of #1809 is not on master. When it is, this difference belongs there, with the goldens back at the reference's bytes.
