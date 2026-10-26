- A verification of a native post whose voucher mark Tally leaves out twice, while the same verification's own window
  read saw a mark, is now refused as `voucher_mark_contradiction` with nothing recorded, and the refusal says to verify
  again once; before this, that case recorded mark 0, which covers every native post of the company as rolled back for
  good. A window read that saw no mark, and a batch an earlier record already covers, are unchanged (fixes #1819, #1853).
