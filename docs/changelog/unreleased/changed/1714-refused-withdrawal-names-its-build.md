- When a batch that names skipped rows in `withdraw_skips` is refused at send as `withdraw_skip_paired`, its next
  step now ends by saying which build to make: this statement again, with the rows in `paired_rows` still skipped
  and not named in `withdraw_skips`, instead of a bare "build it again" after steps that may speak of the other
  account's statement (#1714).
