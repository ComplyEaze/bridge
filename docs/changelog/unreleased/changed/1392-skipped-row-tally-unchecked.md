- A skipped bank-statement row that no statement carries is now `tally_unchecked`, not `not_found`, in the `transfers`
  section of a build and of a verification. `not_found` said the other account's Contra does not exist, but ComplyEaze
  Bridge reads only the statements it built and verified: a Contra entered in Tally another way is not in its records.
  `not_found` is now said only after a read of every voucher of the row's date in Tally finds no Contra between the two
  accounts, and no such read exists yet, so every row that read `not_found` reads `tally_unchecked`. Its next step
  tells the assistant to read that date's vouchers with `vouchers`, to look for a voucher of the Contra voucher class,
  or of any other type such as a Journal or Payment, that carries the row's amount on the bank ledger the other
  account's statement was built for, to tell the person what it found and to keep the skip; a voucher dated another
  day or booked against another ledger is not looked for, a transfer may print on different days, and a skip is
  withdrawn (`withdraw_skips`), on one side only, solely when the person confirms that no entry in Tally carries the
  transfer. Reading a day sends its vouchers to the AI provider: `mask_parties` shortens party and ledger names,
  `drop_narration` drops narrations, and neither hides amounts. A row the other account's statements do not cover stays `not_established`. Tested on invented
  facts; the whole-day read is not built and not yet seen against a live TallyPrime (part of #1392).
