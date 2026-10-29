- `local_data_report` no longer counts a native post as settled once a recorded restore covers it (part of #1050). Since
  a restore seen by `verify_import` is recorded, such a post never reads `posted_verified` again, but its latest status
  could still be `posted_verified` until it was verified again, and the import journal's counts took that status at
  face value. It is now counted as not settled, under its own `not_settled_restored`, whatever its latest status
  reads, and stays there until a later change records that its vouchers are back. **Some counts move:** a covered post
  that `local_data_report` counted under `not_settled_no_response`, `not_settled_binding_refused` or
  `not_settled_reviewed_not_attributed` is now counted only under `not_settled_restored`, so those counts can fall while
  `not_settled` stays the same. Nothing changes in what may be built, posted or sent, and no journal record changes.
