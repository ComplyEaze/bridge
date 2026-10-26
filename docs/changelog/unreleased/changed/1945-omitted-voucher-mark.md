- A native post refuses before it sends anything (`post_mark_unrecorded`) when its last read before the send does
  not show the company's voucher mark, instead of reading the missing mark as zero and binding its vouchers from it;
  the voucher was never misattributed, but it could be posted and then left for reconciliation. A read after the
  post that omits the mark is likewise no mark, so the post is bound from its own answer instead of being refused as a
  mark that went back, and its `post_location` reads `after_snapshot_unavailable` with
  `marks_readback_voucher_mark_omitted` instead of a step to zero. A company that has never held a voucher is still
  refused when the batch is built (fixes #1945).
