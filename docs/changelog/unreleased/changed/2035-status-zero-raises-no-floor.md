- A native post whose recorded answer reported `STATUS` 0 no longer counts that answer's `CREATED` toward the voucher
  mark it proved, so whichever of the company's posts is checked next, a mark read unchanged does not record a
  rollback for it (#2035). After the update, a rollback record whose only reach was such a post no longer covers it:
  that post stops reading `book_rolled_back` and stays unverified, as an answer reporting `STATUS` 0 always leaves it.
