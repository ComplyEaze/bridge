- `parse_bank_statement` now lists the open cash lines in groups: lines whose printed statement text is the same, after
  case and spacing, share a `group_id` with `group_size` and `group_open` and are listed together, so one question can be
  asked for the group; once one line of a group is answered, its open siblings carry a `suggestion` with that answer,
  marked `applied: false`. Nothing is applied for the person: each line still needs its own answer in `cash_answers`, and
  the statement text itself is never returned (part of #1761).
