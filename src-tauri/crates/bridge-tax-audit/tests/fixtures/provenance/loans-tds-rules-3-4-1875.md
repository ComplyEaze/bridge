# Fixture provenance: TDS on loans on one side, and TDS debits, in `loans_interest` (#1875 rules 3 and 4)

Every book named here is invented, with round figures and plain names. No fixture is a Tally read of any real assessee. No new fixture file is added: the books are unit tests in `src/loans_interest.rs` on typed inputs, plus three existing edge books whose goldens move.

`docs/tax-audit/parity-spec-v1.md` section 12 requires every change to an existing output to be a declared difference that names the invented book which exercises it. Both rules below were decided on #1875 on 2026-10-31. The reference engine is not changed and still reads both shapes as before.

## Declared behaviour difference: TDS on loans on one side (#1875 rule 3)

A voucher that posts to the loan and to another configured loan used to count its TDS for neither loan. That still holds, except for one shape, shape (a): every posted line is on a configured loan or a TDS ledger, the loan lines are all on one side, and the TDS lines net to their sum (Loan 1 Dr 8,502 / Loan 2 Dr 164.25 / TDS Payable Cr 8,666.25). Each loan then takes its own line as its TDS, in `tds_on_loan_<tag>`, so LOAN-1 ties on each. A nil line is no posting here, as elsewhere since #1259. Loans on opposite sides are a transfer between loans, a row on each (bridge#803), and are not this shape.

Shapes (b) and (c) keep LOAN-1 as #1259 left it. In (b), a voucher whose lines on the loan net to nil carries the loan's interest and TDS. In (c), a voucher both credits and debits the loan and carries TDS (`edge-books/loans_interest_two_sided_tds.json`, and Seen Loan and Under Loan in `edge-books/loans_interest_phase_a_s194a.json`). LOAN-1 still fires on both, and neither golden's module check moves.

- Unit tests: `tds_on_two_loans_on_one_side_is_each_loans_own_line` (shape (a), with and without a nil bank line); `tds_on_two_loans_not_of_shape_a_counts_on_neither` (opposite sides, another posted line, TDS short of the loans' sum); `loan_1_still_fires_on_a_zero_net_loan_voucher_carrying_tds` (shape (b)); `loan_1_still_fires_on_a_two_sided_loan_voucher_carrying_tds` (shape (c)).
- `edge-books/loans_interest_tds_coverage.json`, Twin A and Twin B: one TDS journal debits both loans 10,000 paise each against TDS Payable Cr 20,000 paise. The golden now carries `tds_on_loan_<tag>` 10000 for each, citing that journal; `s194a_tds_covering_<tag>` cites it too, and `s194a_tds_coverage_<tag>` reads "covered" where it read "none" (neither loan has interest, so nothing is expected). The two LOAN-1 lines are gone. The book's comment says so.

## Declared behaviour difference: a TDS debit stays out of the TDS deducted (#1875 rule 4)

A voucher on the loan whose TDS lines net to a debit is no longer netted into `tds_on_loan_<tag>`, because a TDS debit can reverse an earlier deduction. A loan taken as Loan Cr 10,000 / Bank Dr 9,000 / TDS Payable Dr 1,000 used to make it -1,000. Each such voucher is named in that figure's text ("Left out: ..."), and it no longer appears among that figure's evidence. Its debits, summed, are a new figure, `tds_debited_on_loan_<tag>`, published only where there is one. LOAN-1 adds them back into its tie, because they moved the loan. A shape (a) voucher whose loans are credited takes its loans' shares here, so the two rules agree.

Three readings are left as they were. `tds_in_principal_<tag>` still holds a taken or repaid voucher's TDS of either sign, and the new figure's text says so. The date-aware `s194a_tds_covering_<tag>` still counts a debit as a reversal, as its text and the s.194A limit say. Its evidence and the TDS-seen finding's evidence still cite every voucher with TDS.

- Unit tests: `a_tds_debit_stays_out_of_the_tds_deducted_and_is_named` (the loan taken above, a TDS journal debiting TDS Payable, a deduction beside them, and a control with no debit); `tds_debited_across_two_loans_on_one_side_is_each_loans_own_line` (rules 3 and 4 together).
- `edge-books/loans_interest_principal_tds_nets_nil.json`: the loan taken carrying a TDS debit of 100000 is left out. `tds_on_loan_<tag>` reads 100000 where it read 0, citing the repayment only, and `tds_debited_on_loan_<tag>` 100000 cites the loan taken. `s194a_tds_coverage_<tag>` reads "covered" where it read "none" (no interest). `tds_in_principal_<tag>` stays 0 and LOAN-1 stays silent. The note `loans-tds-in-principal-and-nil-lines-1259.md` still gives this book's old `tds_on_loan`. This section supersedes it.
- `edge-books/loans_interest_phase_a_s194a.json`, Dated Loan: the reversal of the duplicate deduction is left out. `tds_on_loan_<tag>` reads 2000000 where it read 1000000, and `tds_debited_on_loan_<tag>` reads 1000000. Coverage still reads "covered by amount, not by date", and its s.194A limit now says TDS of ₹20,000 is seen, where it said ₹10,000.
- `edge-books/loans_interest_tds_coverage.json`, Round Loan: the 50-paise reversal is left out. `tds_on_loan_<tag>` reads 100050 where it read 100000, and `tds_debited_on_loan_<tag>` reads 50. Coverage still reads "covered".

## Goldens

Each moved golden is the crate's own dump for the parts named above, spliced into the reference's golden with the reference's key order. Nothing else in it changed: the splice reproduced every unchanged `loans_interest` golden byte for byte. Their byte rows stay where they were, in `batch-d1.md` and `loans-tds-in-principal-and-nil-lines-1259.md`. Those notes' statements that the goldens regenerate byte-identical at the reference hold except for these parts.

## Not checked

- How often any of these shapes occurs on real books, or on a real Tally read.
- Whether a TDS debit on a real book is a reversal, a refund or something else. The figure names the voucher so the CA can tell.
- The reference engine on these books. It was not changed, and parity there is not claimed for these parts.
- Shape (a) with a TDS ledger that is also a loan's interest ledger or a shared one: such a ledger is not a TDS line in this test, so the voucher is not shape (a).
