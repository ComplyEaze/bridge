- Reading the mailing details makes the answer larger, and the read is sized for it (#1490). In the one capture (12 Oct
  2026, TallyPrime 7.1 Silver, eight ledgers) the answer grew from 19,094 to 20,896 bytes: 948 bytes for the one party that
  held a mailing entry and 122 bytes for each of the seven rows that carried an empty placeholder. With
  `include_mailing_details` the read is sized at 4,700 bytes a ledger instead of 3,750, a figure taken from the 948 bytes
  and marked unverified in the source; it is not measured on a book of many populated parties, so a large book may be read
  in more parts, or refused, sooner. A dense book reads about 950 bytes more for each party that holds an address; how
  that compares with the 20-second read deadline is not measured.
