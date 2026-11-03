- A person can now release the rows of a native post whose book was rolled back, so they can be built and posted
  again (#1038). Once `verify_import` reports a batch `book_rolled_back`, its rows stay held and a rebuild of them is
  refused as `import_txn_already_posted`. After the person has checked in Tally that none of its vouchers is in the book,
  including any keyed again with other figures, `acknowledge_post_review` with `doubt` set to `rolled_back` records the
  release in its own dialog, which lists every voucher and which the model cannot answer. Only a batch of Journal,
  Payment and Receipt vouchers is released (`ack_release_type_not_admitted`), only one whose vouchers the dialog can all
  list, at most ten (`ack_release_vouchers_not_listed`), and only while a fresh readback shows it rolled back and a
  recorded restore covers it (`ack_rolled_back_not_established`). It changes nothing in Tally, removes no record and is
  never undone: the batch still reads `book_rolled_back`, `verify_import` reports the release as `row_release`, and
  `local_data_report` counts it under the new `released_by_person` instead of `not_settled_restored`. The rows then go
  through an ordinary fresh build and post, with the book check and approval every batch meets; a voucher keyed again
  with other figures is not caught by that check, so the person is the guard. **Downgrade:** once a person has released
  a restored batch's rows, do not install a release older than this one on this computer: it can refuse this
  computer's import journal, and then every import and verification stops there, Journal, Payment, Receipt and Contra
  included. To recover, install this release or a later one again. Do not edit the journal by hand.
