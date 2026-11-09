- A statement build, and a statement of only skipped rows, now read Tally for the transfers section's
  `tally_unchecked` rows before anything is written: each row's date and the day either side, a row's three days taken
  whole or not at all, at most four dates a call, newest first, within 31 days of the newest, and none for a book whose
  voucher mark needs more than one census request a day. A row then reads `not_found` only when every date of its
  window was read in full (counted by a census of its own, with no voucher withheld), no voucher of those dates has an
  entry on the other account's ledger, of any type, and every ledger those dates name is in the ledger list the call
  read, compared exactly as Tally spells it; a date that holds no voucher counts only beside a read date of the window
  that holds one. Anything else keeps the row `tally_unchecked`, whose next step now names each reason and has the
  assistant read the day either side too. The section's new `day_reads` gives the dates read in full and why any other
  planned date was not. No refusal changes: a withdrawal is judged the same for both states. `verify_import`'s section
  reads no date and stays as it was. Measured by replaying one synthetic book's captured reads (TallyPrime 7.2): its
  catalogue, 3 to 6 April 2026 and ten days between 12 and 29 April 2026, with a voucher keyed on a bank and cash, a Contra between the banks on
  the next day, and a transfer through a clearing ledger (#1392).
