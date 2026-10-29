- `acknowledge_post_review` with `doubt` set to `content_only` records, after the native review dialog, that a person
  checked in Tally a native post matched by content only: its binding to its own vouchers was refused, or no answer from
  Tally was recorded. The record changes nothing in Tally and no status or verdict, binds no voucher, lifts no invoice
  stop and releases no row; `verify_import` reports it as `content_review`, and `local_data_report` counts the batch as
  `not_settled_reviewed_not_attributed`. A later `posted_verified`, a later answer from Tally or a rollback of the book
  voids it. It is a new kind of import journal record: a build older than this one refuses the whole journal once one is
  written, so do not downgrade after recording one (part of #1039).
