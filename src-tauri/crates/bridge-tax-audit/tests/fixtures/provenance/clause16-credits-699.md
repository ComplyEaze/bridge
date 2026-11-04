# clause16_credits (#699): a crate-first test

`src/clause16_credits.rs` lists Form 3CD clause 16(a) and (d)'s candidates: each credit to a ledger under Capital Account,
Loans (Liability), Current Liabilities or Suspense A/c, one row per voucher and ledger, with the vouchers, tax credits and
ledgers it leaves out each counted in a figure. Its inputs are Tally's reserved group names, held as constants in the module;
there is no rules file. It is registered in `registry::CRATE_FIRST`, not `PORTED`.

## Declared behaviour difference

The reference engine has no clause 16 test, so there is no reference golden and no reference runner: `parity/python_golden.py`
refuses `clause16_credits` by name (`CRATE_FIRST`). This note adds no golden. The test's books are:

- the unit tests in `src/clause16_credits.rs`, on typed inputs: each ledger's role, a head credit against cash or bank, a bank
  debit beside an expense, the exclusion's "every debit" and "at least one", a set-off journal, both tax triggers, the reserved
  `Profit & Loss A/c` against a user ledger of that name, an untested ledger credited and debited, an overdraft, one ledger on two
  lines, the sort, overflow and the books population;
- the live audit read of BRIDGE SFT LAB, FY 2025-26, on TallyPrime 7.2 Silver, after sheet 699-C16 keyed 19 more vouchers (lab
  row C16-R), committed in the protocol crate as `tests/fixtures/agent/audit-read-sft-lab-c16r/` with its own `PROVENANCE.md`.
  The app crate's `live_sft_lab_c16r::clause16_lists_the_rows_key_v3_worked_by_hand_on_the_live_book` checks the 12 rows and
  the three counted figures against a key worked by hand to the rule from the sheet, before any capture.

Not established: accuracy against a CA. The key is worked to the rule, not by a CA; no CA has worked this book, blind or
otherwise, and every result here is measured against the synthetic lab book only. Whether a credit is income, capital, a loan, a
liability or tax is not in the books. Group nature is not read, so a ledger under a group the user made under Primary is
counted, not tested. A debit to such a ledger counts as a debit that keeps the voucher listed; no voucher of the live read has
one, so that is measured on the unit tests only. Bill-wise allocations are not read (#681).
