- `acknowledge_post_review` with `doubt` set to `not_created` lets a person settle a native post that Tally answered
  as creating none of its vouchers, in its own native dialog, once they say each voucher is in the book or will not be
  posted. ComplyEaze Bridge never sends those vouchers again. The record changes nothing in Tally and no status or
  verdict; the rows stay held, so they are never built or posted again, and a Sales invoice keeps its stop until the
  separate release. `verify_import` reports it as `person_settlement`, and `local_data_report` counts the batch as
  `settled_by_person`, outside `not_settled`. A later `posted_verified`, a later answer from Tally, a binding or a
  rollback of the book voids it. It is a new kind of import journal record: a build older than this one refuses the
  whole journal once one is written, so do not downgrade after recording one (part of #1709).
