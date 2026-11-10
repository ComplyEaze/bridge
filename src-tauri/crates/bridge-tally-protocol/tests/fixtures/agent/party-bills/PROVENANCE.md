# Party bills fixtures: BRIDGE BILLTRAIL LAB (#2377)

Live answers to the `Ledger Outstandings` report for one party, and to the `List of Ledgers` snapshot each is tied
to, on the synthetic company `BRIDGE BILLTRAIL LAB` loaded alone on TallyPrime 7.2 Silver, port 9001, Windows lab,
8 Nov 2026. Each request was a fixed file sent with curl, one at a time, and each answer was written to a file as
received (BOM-less UTF-16LE; the fixture tree's attributes keep them `-text`). The company, its ledgers and its vouchers
are synthetic. Part A only read; Part B made its parties and bills behind a backup and restored the book after.

- Part A: evidence branch `lab/2377-l-answers` at `a2be6d96` (its `SHA256.txt` lists each file with the same hash).
- Part B: evidence branch `lab/2377-b-answers` at `44e44b28`.
- Row 2377-NB: evidence branch `lab/2377-nb-answers` at `61a5c1bf`, on another synthetic company, `BRIDGE PILOT LAB`,
  loaded alone on TallyPrime 7.2, 9 Nov 2026, reads only. Its bill-wise payable party `BRIDGE Supplier RJ` to
  31 July 2026, before its first entry, answered no bytes twice, with headers byte-equal to
  `ledger-outstandings-unknown-party.headers.txt`, and the snapshot to the same date carries the party, bill-wise, with
  an empty `CLOSINGBALANCE` (`ledger-snapshot-supplier-rj-20260731.utf16le.xml`). That snapshot request was
  `render_native_ledger_snapshot_request` from the book's first day, 1 April 2026 (its books begin then, as
  `agent/pilot-lab-register/pilot-lab-register-book-extent.utf16le.xml` records), to the same date: the period the
  party-bills read builds. Only that snapshot tells this party
  from a name no ledger carries (protocol reference §12a.27).
- The Ledger Outstandings request: `<ID>Ledger Outstandings</ID>`, `SVCURRENTCOMPANY`, `LedgerName`, `SVTODATE`,
  `SVEXPORTFORMAT` `$$SysName:XML` and `EXPLODEFLAG` `Yes`; no `SVFROMDATE`, no fetch list. The snapshot request is
  `render_native_ledger_snapshot_request` from the books' first day, 1 April 2025, to the same to-date.
- An unknown name answered no bytes twice, with a healthy `/status` between. A name differing only in letter case or
  a doubled inner space answered byte-equal to the exact name's answer (not kept here: the same bytes as
  `ledger-outstandings-p1-20260331.utf16le.xml`).

Tests: `src/native_outstandings/party_bills_tests.rs` parses each answer and ties it to its ledger's closing; the parse
record keeps what each parser returns.

Confidence, with the markers of `docs/tally/TALLY_PROTOCOL_REFERENCE.md`: PARTIAL for every finding here. Each was
observed live, with request and answer captured, but on one synthetic book, one release and one edition, in two
sittings on one day; Bridge's own transport did not send these requests.

Protocol behaviour these captures record, for the declared reference part (P9). Each is PARTIAL on the scope above, and
none is yet in a declared part of `docs/tally/TALLY_PROTOCOL_REFERENCE.md`, which is a pinned file and is changed in a
pull request of its own:
- `Ledger Outstandings` with a `LedgerName` no ledger carries answers HTTP 200, `RESPSTATUS: 1`, `CONTENT-LENGTH: 0`,
  no `Content-Type` and no body, twice, with a healthy `/status` between (`ledger-outstandings-unknown-party.*`).
- A ledger made with a trailing space in its name is held without it, and the `List of Ledgers` snapshot names it so
  (`ledger-snapshot-trailing-space.utf16le.xml`).
- Of two On Account rows on one party, the second leaves `BILLDUE` and `BILLOVERDUE` empty
  (`ledger-outstandings-two-on-account.utf16le.xml`).

Not established: Bridge's own transport against a live Tally for this report (curl sent it); a payable party; a party
with bill-wise off; a foreign-currency ledger; a real bill whose reference is the text `On Account`; a party with no
open bills; other releases, editions, a client book.

| Fixture | Bytes | SHA-256 | Source (lab file) | What it is |
| --- | ---: | --- | --- | --- |
| `ledger-outstandings-p1-20260331.utf16le.xml` | 5,878 | `063b3c7684b4fe416a0749c114e3b454fe59d71eb6e9bd0e01c9db25bb86de5b` | Part A `answers/s3-lo-p1-0331-a.answer.xml` | `Lab Party P1` at 31 March 2026: five bills and one On Account row, Σ BILLCL -26000.00; a second send was byte-equal |
| `ledger-outstandings-p1-20260731.utf16le.xml` | 3,926 | `f505ba966c106ecd6674bffb390187e6a1e8c52ffc33d173ed87bbdd3ec88091` | Part A `answers/s3-lo-p1-0731.answer.xml` | the same party at 31 July 2026: four bills and one On Account row, Σ BILLCL -37000.00 |
| `ledger-snapshot-p1-20260331.utf16le.xml` | 10,378 | `e111b013092484701c172917728a4e3dfdd90be6f5acab10070396ca4b490cf9` | Part A `answers/s2-snap-0331.answer.xml` | `List of Ledgers` from 1 April 2025 to 31 March 2026; `Lab Party P1` closes at -26000.00 |
| `ledger-snapshot-p1-20260731.utf16le.xml` | 10,378 | `829ee265067cdc7d3b18c20fb276ac4d8d5e565a370bfbc13fe1c8f84328e419` | Part A `answers/s2-snap-0731.answer.xml` | the same to 31 July 2026; `Lab Party P1` closes at -37000.00 |
| `ledger-outstandings-unknown-party.utf16le.xml` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Part A `answers/s6-lo-unknown.answer.xml` | a name no ledger carries: no bytes at all |
| `ledger-outstandings-unknown-party.headers.txt` | 77 | `547c40c5c5f4e011efd4f95175768b2ae81f0dfd351f7e36d0b3b9821a584e0b` | Part A `answers/s6-lo-unknown.headers.txt` | that answer's headers: HTTP 200, `RESPSTATUS: 1`, `CONTENT-LENGTH: 0` |
| `ledger-outstandings-ampersand.utf16le.xml` | 846 | `75b96699bb2902437a3eb5040eb857ce3edc26912c7cf641c0b9b3d19a6564c8` | Part B `answers/b5-amp-r-lo.answer.xml` | `L5 Amp & Co` (sent escaped): one bill, -100.00 |
| `ledger-snapshot-ampersand.utf16le.xml` | 12,822 | `6d8fbc6d496f94c0c653a1ee302bd19d9d4bc0f18e34c2a7cc852208eeb4e6ce` | Part B `answers/b5-amp-snap.answer.xml` | the snapshot after that party was made; it closes at -100.00 |
| `ledger-outstandings-trailing-space.utf16le.xml` | 852 | `72b8f02a2aae65f8a8125ce66a04d8d4fda1df5e4e9e8bcbd01000a3e8b7a086` | Part B `answers/b5-trail-r-lo.answer.xml` | `L5 Trail ` sent with its trailing space: the party's one bill, -100.00 |
| `ledger-snapshot-trailing-space.utf16le.xml` | 29,934 | `5f78ea690df025c61bdceaf3b73b568b3fc06c81de67805a5b1df1ca57720123` | Part B `answers/b5-trail-snap.answer.xml` | the whole book's snapshot after it, 22 ledgers: Tally holds the party as `L5 Trail`, without the space |
| `ledger-outstandings-two-on-account.utf16le.xml` | 1,636 | `00b538866cc8327666694432575889e787dde4d5c6c496d799d3b1914406477c` | Part B `answers/b6-r-lo.answer.xml` | `L7 Party` with two On Account rows, -100.00 and -50.00; the second leaves BILLDUE and BILLOVERDUE empty |
| `ledger-snapshot-supplier-rj-20260731.utf16le.xml` | 25,298 | `552bc05e5ff566f3b51372b3bd5e9f3ce56e1b49b40aa12e4b0585dfeaac423f` | Row 2377-NB `answers/a-s2-S-snap-20260401-20260731.xml` | `BRIDGE PILOT LAB`'s `List of Ledgers` from its first day, 1 April 2026, to 31 July 2026: `BRIDGE Supplier RJ` bill-wise with an empty `CLOSINGBALANCE`; every closing in it is empty |
