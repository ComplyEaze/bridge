- The `held_elsewhere` step of the transfer section now names both outcomes of sending the batch that holds a
  skipped row: once the skip is live, `post_import` refuses that send as `skip_row_paired_elsewhere`, and a skip
  that was only built still lets it through, which leaves the row `posted_against_skip` once found posted (#1988).
