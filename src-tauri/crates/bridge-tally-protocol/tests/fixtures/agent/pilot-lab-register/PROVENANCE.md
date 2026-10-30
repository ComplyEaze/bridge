# `pilot-lab-register-*` — provenance

One register call on the synthetic company `BRIDGE PILOT LAB` for 4 to 5 August 2026, with its census spanned to the
voucher mark the call reads (#2145). `purchase_register` and `sales_register` send the same requests, so this one record
serves both. It replaces, for the two register tests that pin every request (`src/agent_register_server_tests.rs`), an
earlier record of the same call (`../register-e2e/PROVENANCE.md`) that holds only the unspanned census.

## Provenance

- **Item 2145-R, Part P.** Run sheet `lab/2145-spanned-replay-recapture` at `8a2ecc3a`; evidence
  `lab/2145-spanned-replay-recapture-answers` at `d3d4c19e` (`RESULTS.md`, `answers/P/`). TallyPrime 7.2 Silver
  (licensed), port 9001, `BRIDGE PILOT LAB` loaded alone. Each request was sent once with curl, one at a time; reads only.
  The marks read 68 / 245 before the first request and after the last, and the book extent read 68 / 245 with books from
  20260401 and the last voucher dated 20260915.
- **The requests** were rendered by Bridge's own code at `2da3a53d` through a throwaway test that drove the call and kept
  what it sent; no byte was typed by hand. The call sends ten kinds of request in the order the tests name
  (`RECORDED_ORDER`, 102 exchanges); the lab sent each kind once, and a repeat is answered as the first. The window read
  names REFERENCEDATE, and the census is spanned to `(0, 68]`.
- **The answers** are as Tally sent them, BOM-less UTF-16LE. The evidence branch carries the company list and the book
  extent with their COMPANYNUMBER text redacted; the two files here hold Tally's value again (the company number
  `../pilot-lab/pilot-lab-company-extent.utf16le.xml` already carries), and each SHA-256 equals the one the lab recorded
  for the answer as received.
- **Answers kept once.** The currencies, groups, compliance ledger listing and marks answers were byte-identical to
  answers already kept from this book, so the tests read those: `../pilot-lab/pilot-lab-currencies.utf16le.xml`,
  `../pilot-lab-w18/w18-groups.utf16le.xml`, `../pilot-lab-w18/w18-ledgers-compliance.utf16le.xml` and
  `../gst-status/pilot-lab-gst-uncertain-august-marks.utf16le.xml`.
- **The window** holds three vouchers: two Purchase (ALTERIDs 31 and 32, 4 August) and one BRIDGE Sales (ALTERID 35,
  5 August), each with CGST and SGST. The census holds the same three.

## What it establishes

- The order and bytes of the requests a register call sends on a one-window read whose census fits one request, and that
  the purchase register lists the two Purchases while the sales register lists the sale and names the two Purchases apart.

## What it does not establish

- A window whose census is divided into several requests, a register call that refuses, or any other book, release or
  edition. One sitting: PARTIAL.

| file | bytes | sha256 |
|---|---|---|
| `pilot-lab-register-extent.utf16le.xml` | 4000 | `8d7b9498f929840ed160c9ed8b32576683d4861d2d6e05e682f4486cc21373ae` |
| `pilot-lab-register-book-extent.utf16le.xml` | 3836 | `9c63967ad722ea001a36048a17cf10d21868c19ccd02d41c6e5012ed073874e4` |
| `pilot-lab-register-ledgers-paired.utf16le.xml` | 25518 | `63b3a2573a91ff131c00f8e296a0aae6d6e3a02a4b150b404364d267ac0897fa` |
| `pilot-lab-register-census.utf16le.xml` | 10276 | `414ebb181112b299259446ff5a60832c197011127c452d331dbcfb4856c9a60f` |
| `pilot-lab-register-window.utf16le.xml` | 226584 | `17379f502cad7e46fd19f4f7cfdf63da0cea0e9450e3462120703220bccda00f` |
