- When `post_import` or `verify_import` cannot confirm a post because the company's voucher mark moved by another amount than the
  post created, or the post's span later held another number of vouchers, the post's summary now also says what was
  observed and what can cause it (for example a GST return exported from Tally, an edit of one of the vouchers, or another
  user's entry), and that ComplyEaze Bridge sent the post once and Tally reported creating each voucher once. That sentence is shown only
  for those two refusals, never for an answer that reported `STATUS` 0 or whose counters were not clean. Measured on
  TallyPrime 7.2, on one synthetic book each: two GSTR-1 JSON exports each raised the voucher mark by 3 without changing
  a voucher, while one local GSTR-3B JSON export of one month left it unchanged (17 before and after) and moved no
  voucher's alter id; a voucher saved by hand, once, inside a post's span made the mark move by 51 for 50 created, and the span
  binding was refused. Nothing about how a post is proven changes (part of #1633).
