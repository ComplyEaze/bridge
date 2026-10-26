- The Privacy Policy is now version 2026-10.3, effective 16 October 2026, and corrects what it said about builds
  already published. `parse_bank_statement` returns the last four digits of a statement's account number
  (`account_last4`) under every response redaction setting, and every voucher prepared from a statement carries the
  account label the person gives in its narration, which holds at least those four digits; the assistant writes the
  label, so the AI provider receives it under every setting, and `drop_narration` removes only the later copies. This
  has been so in every published build from 0.3.0. The short version also no longer singles out amounts as the one
  thing the optional setting never hides: it never hides amounts, PAN, GSTINs or many other details (part of
  #1615, #1629).
