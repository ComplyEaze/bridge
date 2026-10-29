- Under `mask_parties`, `ledger_masters` with `fields=compliance` now masks the flat `address` of a ledger
  (and, from the same change, every mailing address line) like a party name: before, the flat address was
  returned as Tally sent it under every redaction mode. An address Tally returned empty stays empty. The flat
  PIN code and state, which every compliance read returns, and the mailing entry's PIN code, state and
  country, which are returned only when the assistant asks for the mailing details, are still not masked
  under any mode (#1490).
