- A statement build, and a statement of only skipped rows, now read Tally for the dates of the transfers section's
  `tally_unchecked` rows: at most four dates a call, newest first, within 31 days of the newest, after the build is
  written. A row then reads `not_found` only when its whole date was read (counted by a census of its own, with no
  voucher withheld) and no voucher that day has an entry on both of its ledgers, of any type. A date that holds no
  voucher, a read that fails or is partial in any way, and a ledger the build's ledger list does not name keep the row
  `tally_unchecked`, and that state's next step now names each reason, including a day read whole that holds a voucher
  on both ledgers. No refusal changes: a withdrawal is judged the same for both states. `verify_import`'s section reads
  no date and stays as it was. Measured by replaying one synthetic book's captured reads of 3 to 6 April 2026
  (TallyPrime 7.2) (#1392).
