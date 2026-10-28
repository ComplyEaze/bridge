- A native post whose recorded answer reported `STATUS` 0 no longer counts that answer's `CREATED` toward the voucher
  mark it proved, so whichever of the company's posts is checked next, a mark read unchanged does not record a
  rollback for it (#2035). After the update, a rollback record that read a mark at or above such a post's pre-POST
  mark no longer covers it: the post no longer reads `book_rolled_back` because of that record; its batch no longer
  carries `restored` where the `transfers` section or a `withdraw_skips` refusal names it, and a held row may name
  another batch that holds it; and the post stays unverified, as an answer reporting `STATUS` 0 always leaves it. The
  readback in that post's own `post_import` call still reads `book_rolled_back` when the mark falls between the one
  read before the post and the one read just after it; that post's own floor then records no rollback, though another
  native post of the company at that Tally address can still record one in the same call. The answers the tests use are
  written by hand: no import answer with `STATUS` 0 and a `CREATED` count has been captured from a live Tally, and the
  native posts measured answer with a `RESPONSE` root that reports no status, so this has not been seen on a live post.
