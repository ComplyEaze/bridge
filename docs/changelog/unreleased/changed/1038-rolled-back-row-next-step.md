- When a row is refused as already posted and `verify_import` reports the blocking batch `book_rolled_back` (a
  backup was restored over the book), the refusal's next step now says that ComplyEaze Bridge cannot write the row
  again and cannot tell whether its voucher is gone, and to tell the user (#1038).
