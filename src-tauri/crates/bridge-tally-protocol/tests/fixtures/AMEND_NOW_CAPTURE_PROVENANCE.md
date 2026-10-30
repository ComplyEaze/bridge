# `amend-now-*` — provenance

One read of each request that `verify_import` sends for the D3 batch (`D3_BATCH_CAPTURE_PROVENANCE.md`) and for the
WA1 payment (`WA1_PAYMENT_CAPTURE_PROVENANCE.md`), on the synthetic company `BRIDGE AMEND LAB` as it stood on 30 Oct 2026,
with each census spanned to the voucher mark the call reads (#2145). The captures of 26 Sep 2026 hold only the unspanned
census, and an answer captured for it may not stand in for the spanned one (#899); a backup of the book from those days
was looked for and not found, so these answers are of the book as it is now.

## Provenance

- **Item 2145-R, Part A.** Run sheet `lab/2145-spanned-replay-recapture` at `8a2ecc3a`; evidence
  `lab/2145-spanned-replay-recapture-answers` at `d3d4c19e` (`RESULTS.md`, `answers/A/`). TallyPrime 7.2 Silver
  (licensed), port 9001, `BRIDGE AMEND LAB` loaded alone. Each request was sent once with curl, one at a time; reads only.
  The marks read 1829 / 245 before the first request and after the last.
- **The requests** were rendered by Bridge's own code at `2da3a53d` through a throwaway test that drove each call and kept
  what it sent; no byte was typed by hand. The company list (`9df2a53f`), the marks read (`0930288f`) and both
  verification reads (`c25ed5f6` for the batch, `a29a7448` for the payment) are byte for byte the requests of the 26 Sep
  captures; the census requests are spanned to the mark the call reads, `(0, 1829]`: `c67e0560` for 1 April 2026 and
  `25bc466b` for 3 April 2026.
- **The answers** are as Tally sent them, BOM-less UTF-16LE. The evidence branch carries the company list with its
  COMPANYNUMBER text redacted; `amend-now-company-extent` holds Tally's value again (the company number the `d3-*` and
  `wa1-*` extents already carry), and its SHA-256 equals the one the lab recorded for the answer as received. The two
  census answers are byte-identical to those of the same days in item 2145-C (`lab/2145-spanned-census-captures-answers`
  at `993d96bc`).
- **One capture serves both batch replays.** The batch's call and the cancelled-voucher call send byte-identical
  requests, so both read these answers.

## What the book holds now

- **1 April 2026, the D3 batch:** 49 of its 50 vouchers. D3-005 (MASTERID `0x552`) was deleted after 26 Sep 2026; D3-003
  is cancelled (ALTERID 1685), D3-001 and D3-002 carry 1680 and 1677, as in the `d3-cancelled-*` capture, and the other 46
  their batch ALTERIDs.
- **3 April 2026, the WA1 payment:** 4 vouchers. The payment reads back with `ISOPTIONAL` Yes and ALTERID 1681.
- **The call's order on this book:** 32 exchanges, the 30 of the 26 Sep captures and then a status probe and the
  company list once more, which `verify_import` reads because D3-005 is not found. Each request was read once in the
  sitting; a repeat is answered as the first, as every repeat was in the 26 Sep captures.

## What it establishes

- On this book, `verify_import` of the batch reads 48 verified, D3-003 not effective (cancelled) and D3-005 not found.
- A review of the batch's step doubt, and a review of the payment, are refused (`ack_readback_not_matched`) before any
  dialog: a review admits only vouchers that read back matched, not cancelled or optional.

## What it does not establish

- The batch's review binding all fifty vouchers, or the payment's review showing each entry as the post dialog did
  (#730): the book no longer holds those states.
- Any other book, release or edition. One sitting: PARTIAL.

| file | bytes | sha256 |
|---|---|---|
| `agent/amend-now-company-extent.utf16le.xml` | 4000 | `ff15e557b351ab4c98055e55f749f28cd12d489115583e7af6e4f22cb4178759` |
| `agent/amend-now-company-high-water.utf16le.xml` | 3398 | `874781bbf43b02712aa0821045eeae6ffe6f72c5edd353d12db2cd6cb82812fc` |
| `agent/amend-now-d3-voucher-census.utf16le.xml` | 121964 | `313bbf812abd699ed62d28e1f671c842018696e0dd3f96b87557083642c3cf2b` |
| `agent/amend-now-d3-import-verification.utf16le.xml` | 246956 | `de4050ebba90c01ffb33db521fd1cd31daf417b6a09782fe4282f5e78b3e9d79` |
| `agent/amend-now-wa1-voucher-census.utf16le.xml` | 12730 | `2909e9c14ef570c952df2efcfc05b6969eb77cc502970e1f4c3194d726207edf` |
| `agent/amend-now-wa1-import-verification.utf16le.xml` | 23508 | `e896a14607cfa36214248728b1753e4a106514cae862dada191a5522509974f4` |
