- `audit_test_run` now refuses a company whose books begin after the audit period ends (`C5-books-from`) before it
  reads the books (it first asks Tally which companies are loaded); before, it read the whole book first and refused afterwards. The refusal's code is
  unchanged and it still may not be retried. Books that begin inside the period (a first year) still read (fixes #1934).
