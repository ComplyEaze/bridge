- `parse_bank_statement_csv` now writes voucher proposals from one private bank's CSV export (part of #1320): the
  file must print one full account number ending in `account_label`'s digits, and a row reaches a ledger only when
  `mapping` names its whole description, otherwise it goes to `suspense_ledger`. A description holding a digit is
  unnamed, so a reference or account number never reaches the result. The result says where the opening
  and closing figures came from and whether they are independent of the file. Tested on synthetic rows in that
  layout; no rule yet reads a name or a cash line out of a narration.
