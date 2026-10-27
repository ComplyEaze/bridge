- A call to `audit_test_run` that names a test the tool does not admit is refused as `argument_invalid:test`, before any
  request to Tally; the guide now names that code (it had named `audit_test_not_admitted`, which a call could not reach),
  and the list of admitted tests in the tool's schema is taken from the tool's own list instead of a second copy
  (fixes #1915).
