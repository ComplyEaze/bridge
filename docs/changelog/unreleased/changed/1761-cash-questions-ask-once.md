- `parse_bank_statement`'s tool text now tells the assistant it may ask the person once about all the open cash lines, listing each
  line's date and amount from `cash_questions`, then answer each line on its own and ask again about any line the person did not
  confirm; one answer is never applied to another line for them. The result is unchanged: an open cash line still carries only its
  id, date, amount and party name (part of #1761).
