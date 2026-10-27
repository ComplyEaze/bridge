- A party's mailing address, PIN code, state and country are now read from Tally's dated mailing details
  (`LEDMAILINGDETAILS.LIST`), which the party-master request now names (#1490). On the measured book (one
  synthetic company, TallyPrime 7.1 Silver) the flat address, PIN code and state fields came back empty or absent
  for a party that held an address, so the workbook showed them as empty or not observed. `ledger_masters
  fields=compliance` rows carry the dated entries as `compliance.mailing_details` (the flat fields are unchanged),
  and the party/ledger master workbook gains five columns at its end: the mailing address, PIN code and state in
  force on the Balances-as-of date, the date they apply from, and a status that says why they are blank (no
  mailing details, none in force on that date, not observed, or unreadable). A second mailing entry with a later
  date, a party with two mailing rows and other releases are not measured. A party whose only entry starts after the
  Balances-as-of date shows the first date in its status, and an entry that held content the reader does not take
  is flagged (`other_content_skipped`). `mask_parties` masks the mailing name, every address line and the flat address field like a party's name;
  the PIN code, state and country are returned under every redaction mode. The request is longer by one element, so the per-ledger size estimate
  of the reads that name it (`ledger_masters` and the workbook) is 4,700 bytes instead of 3,750 (8,950 instead of
  8,000 with the GST details), from the one growth measured (948 bytes for one populated party row); it was not
  measured on a book of many populated parties, so a large book may be read in more parts, or refused, sooner than
  before. The registers do not read the mailing details and keep their request and their estimate.
