- On Windows the box that asks for approval before a ledger is created is now measured when it opens, as the voucher
  approval boxes are: a box that does not fit the screen it opens on is cancelled and the request is refused as too
  large (`master_review_too_large`), with nothing sent to Tally. The box shows one ledger in nine lines, so it is
  well inside the size the lab measured to fit; this closes the one approval box that had no check. A test now
  fails if the ledger text grows past nine lines (part of #1780).
