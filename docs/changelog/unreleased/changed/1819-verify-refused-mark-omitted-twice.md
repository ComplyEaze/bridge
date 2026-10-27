- Checking a native post with `verify_import` no longer records a rollback when Tally leaves the company's voucher mark
  out of its answer twice in a row while the check's own window read did see a mark. That case is now refused as
  `voucher_mark_contradiction`, with nothing recorded, and the refusal says to verify again once. A window read that saw
  no mark, or a mark of 0, records mark 0 as before. No omission of this kind has been seen from a live Tally (part of #1050, fixes #1819).
