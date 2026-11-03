- The `masters` tool for voucher types, and the numbering read of a build, a post and the presence check, are no longer
  refused as `masters_collection_changed` on the first read after a company is loaded in Tally (#2160). On a book with
  voucher types of its own, Tally left one element out of two rows of that first read and sent it on every later one, so
  the first pair of reads disagreed on a book that had not changed. ComplyEaze Bridge now reads the voucher-types pair
  once more when, and only when, the first pair disagrees: one more pair at most, and a second disagreement is refused as
  before. The `masters` answer shows the first pair's digests as `first_pair_differed`. A book that really changed
  during the read is still refused.
