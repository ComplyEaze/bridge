# clause42_sft (#699): a crate-first test

`src/clause42_sft.rs` lists Form 3CD clause 42's SFT-013 candidates: each voucher whose cash debits are strictly over the
limit in `rules/clause42_sft.toml` (sha256 `71568f6237ef4c6167ed0ab2045337fc80ecc66a0faf53aac2be7684de503536`, checked by
the module's own test). It is registered in `registry::CRATE_FIRST`, not `PORTED`.

## Declared behaviour difference

The reference engine has no clause 42 test, so there is no reference golden and no reference runner: `parity/python_golden.py`
refuses `clause42_sft` by name (`CRATE_FIRST`). This note adds no golden. The test's books are:

- the unit tests in `src/clause42_sft.rs`, on typed inputs: the strict limit, one voucher's cash lines summed and never two
  vouchers', a bank line beside cash, a payment, Contra either way, an excluded voucher, the empty book, and overflow;
- the live audit read of BRIDGE SFT LAB, FY 2025-26, on TallyPrime 7.2 Silver (lab row 699-R1), committed in the protocol
  crate as `tests/fixtures/agent/audit-read-sft-lab/` with its own `PROVENANCE.md`. The app crate's
  `live_sft_lab::clause42_lists_each_voucher_whose_cash_is_over_the_limit_on_the_live_book` checks the 14 vouchers listed and
  the 20 not listed against the vouchers as the lab keyed them, one shape at a time.

Not established: accuracy against a CA. No CA has worked this book, blind or otherwise; every result here is measured
against the synthetic lab book only. Bill-wise allocations are not read (#681), so whether several receipts pay one sale is
not told apart; whether each receipt is for a sale of goods or services is not in the books. A Contra voucher is left out of
the candidates, as `cash_44ab` leaves it out of cash receipts; the lab book's one Contra moves cash to the bank, so leaving
out a Contra that moves cash in is not measured on a live book.
