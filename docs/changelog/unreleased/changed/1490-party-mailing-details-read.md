- A party's mailing address, PIN code, state and country can now be read from Tally's dated mailing details
  (`LEDMAILINGDETAILS.LIST`) (#1490). On the measured book (one synthetic company, TallyPrime 7.1 Silver) the flat
  address, PIN code and state fields came back empty or absent for a party that held an address. `ledger_masters
  fields=compliance` takes `include_mailing_details` (default false): without it no request names the list and no row has
  the key (the flat address is shortened under `mask_parties` on every compliance read, as the next entry says); with it each row carries the dated entries as `compliance.mailing_details`, oldest first, and the read is
  sized at 4,700 bytes a ledger instead of 3,750 (from the one growth measured, 948 bytes for one populated party
  row; not measured on a book of many populated parties, so a large book may be read in more parts, or refused,
  sooner). It is refused together with `include_gst_details`, as the request naming both has not been captured. The
  party/ledger master workbook, a local file written by the desktop app, gains five columns at its end: the mailing
  address, PIN code and state in force on the Balances-as-of date, the date they apply from, and a status that says
  why they are blank (no mailing details, none in force on that date, not observed, or unreadable). The workbook
  asks Tally for the mailing details only for a book whose master-alteration mark (an upper bound on its ledgers) is
  at most 3,404; for a larger book it sends the same request as before and every ledger's status reads not
  observed, with a note in the workbook's header saying so, so a book that exported before is never refused for the
  larger per-ledger estimate. For a book past that mark the five columns are still there but empty, and every status reads not observed: the choice is made on the mark, which counts every master alteration, so a book with few ledgers and many alterations is read without the mailing details too. A dense book inside the bound reads a larger answer than before (about 950 bytes more for each party that holds an address); how that compares with the 20-second read deadline is not measured. A second mailing
  entry with a later date, a party with two mailing rows and other releases are not measured. `mask_parties` shortens
  the mailing name, every address line and the flat address field like a party's name, to their first two and last two
  characters; the PIN code, state and country are returned under every redaction mode. The registers do not read the
  mailing details and keep their request and their estimate.
