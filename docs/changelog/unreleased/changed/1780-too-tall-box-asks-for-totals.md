- On Windows, a batch whose approval box with one line per voucher does not fit the screen is refused as before
  (`import_review_too_large`, nothing posted), and asked again it shows the totals only: the per-ledger totals, the
  company and the cautions, with the sentence that the voucher lines are not shown. Without this the same batch could
  only be split into parts. The approval is for that text, and a later call posts exactly it. It applies to a batch
  of two or more vouchers once its box was seen not to fit, for as long as an approval would be kept; one voucher and the
  review window have no shorter text and are refused as before. If the totals box does not fit either it is refused
  again, and a further ask shows the totals again for as long as an approval would be kept, so the batch has to
  be split. Run on a real screen in the lab for the check that finds the box too tall (part of
  #1780); the second ask and the posting after it have been run in tests only.
