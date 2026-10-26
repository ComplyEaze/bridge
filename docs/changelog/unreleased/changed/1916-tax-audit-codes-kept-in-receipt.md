- The receipt of a failed call now keeps the error code of a tax-audit refusal (a rule code such as
  `BIND-GROUP-UNKNOWN`) and of a refused argument (`argument_invalid:test`). Before, only lower-case tool codes were kept,
  so those two kinds of refusal left a receipt with no error. A code is kept only if it is one of three kinds (a
  lower-case tool code, or one of two closed lists) and is at most 64 bytes; anything else is still dropped (fixes #1916).
