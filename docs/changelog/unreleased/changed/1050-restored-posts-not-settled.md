- `local_data_report` no longer counts a native post as settled once a recorded restore covers it (part of #1050). Since
  a restore seen by `verify_import` is recorded, such a post never reads `posted_verified` again, but its latest status
  could still be `posted_verified`, and the import journal's counts took that status at face value. It is now counted as
  not settled, under its own `not_settled_restored`, whatever its latest status reads. Nothing in ComplyEaze Bridge
  clears that count yet, and the post's rows stay held. **Counts that move:**
  - a covered post counted under `not_settled_no_response`, `not_settled_binding_refused`,
    `not_settled_reviewed_not_attributed` or `not_settled_not_verified` is now counted only under
    `not_settled_restored`, so each of those can fall;
  - a covered post whose latest status still read `posted_verified` was counted as settled and is now not settled, so
    `not_settled` can rise.
  Nothing changes in what may be built, posted or sent, and no journal record changes.
