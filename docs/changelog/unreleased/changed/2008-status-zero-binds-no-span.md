- A native post whose recorded answer reported `STATUS` 0 is no longer bound to the vouchers in its range of Tally
  AlterIDs, whatever the answer's counters say: `verify_import` now reports `post_span_binding` refused with the code
  `span_response_status_failure`, as the post itself already treated that answer as not clean (#2008).
