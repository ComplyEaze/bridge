- A native post whose recorded answer reported `STATUS` 0 no longer counts that answer's `CREATED` toward the voucher
  mark it proved, so whichever of the company's posts is checked next, a mark read unchanged does not record a
  rollback for it (#2035). After the update, a rollback record whose only reach was such a post no longer covers it:
  the post no longer reads `book_rolled_back` because of that record; its batch no longer carries `restored` where
  the `transfers` section or a `withdraw_skips` refusal names it, and a held row may name another batch that holds it;
  and the post stays unverified, as an answer reporting `STATUS` 0 always leaves it. The readback in that post's own
  `post_import` call still reads `book_rolled_back` when the mark falls between the one read before the post and the
  one read just after it, but records no rollback for that.
