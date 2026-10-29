- A party's mailing address, PIN code, state and country can now be read from Tally's dated mailing details
  (`LEDMAILINGDETAILS.LIST`) (#1490). On the measured book (one synthetic company, TallyPrime 7.1 Silver, 12 Oct 2026) the
  flat address, PIN code and state fields came back empty or absent for a party that held an address, and the dated list
  held it: two address lines, the PIN code, the state, the country, the date it applies from and the mailing name.
  `ledger_masters fields=compliance` takes `include_mailing_details` (default false): without it no request names the
  list and no row has the key; with it each row carries the dated entries as `compliance.mailing_details`, oldest first.
  It is refused together with `include_gst_details`, as the request naming both has not been captured. Under
  `mask_parties` the mailing name, every address line and the flat address field are shortened like a party's name, to
  their first two and last two characters; the PIN code, state and country are returned under every redaction mode. A
  second mailing entry with a later date was read on TallyPrime 7.2 (one party, oldest first); three or more entries, an
  entry with no date and other releases are not measured. The
  registers do not read the mailing details and keep their request and their estimate.
