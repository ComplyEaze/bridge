- The receipt of a failed call now keeps its error code when it is one of the 61 tax-audit rule codes (for example
  `BIND-GROUP-UNKNOWN`) or a tool code naming the argument it refused (for example `argument_invalid:test`); before this,
  every such code was dropped and the receipt showed no error for the call. A code is kept only when
  it has a known shape, at most 64 bytes, and an argument name only when one of the registered tools publishes it (fixes
  #1916, #1923).
