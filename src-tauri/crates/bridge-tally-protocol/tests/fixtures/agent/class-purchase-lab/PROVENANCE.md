# An item purchase keyed through a voucher class: captured voucher answer (#2057)

This file is an answer Tally sent on the synthetic book BRIDGE CLASS PURCHASE LAB, TallyPrime 7.2 Silver, to the one-day request
`vouchers` sends (protocol reference §8.2m). The lab's runner wrote it straight to a file, and the book was then restored from its
backup. It is kept exactly as received (UTF-16LE without a BOM). The data is synthetic: Lab names, and the lab's invented all-Z GSTIN
in `PARTYGSTIN`. The answer also carries `VOUCHERKEY`, Tally's internal key, as a 15-digit value. There is no company number or PAN.

- 4 Nov 2026, row 2057-B: the book's duty ledger Lab CGST had its Tax type set to CGST on Tally's screen, then four Purchase invoices
  were keyed on Tally's screen, three through the Purchase type's voucher class and one on the plain type. This file is the first,
  Purchase 1 of 10 June 2025, keyed through the class. Evidence: `lab/2057-b-answers` at d9005ab3, `answers/fixture-i1b-r-a07-20250610.xml`,
  byte-equal to that row's `raw/r14-r-a07-20250610.xml`.

The request was sent by a file, not through a build: the committed one-day request of the shape lab
(`voucher_goods_lines_two_godowns_shape_lab_request.utf16le.xml`) with only the company and the day changed. A build renders that same
request for this company and day; `agent_voucher_goods_lines_tests.rs` checks it.

The voucher's tax ledgers' masters were not captured, so the register's reading of this voucher (both tax heads recognised, the
purchase ledger a taxable entry) is recorded from the lab's tool answers in §8.2m, not tested on this file.

The row's other three invoices were read the same way and are recorded in §8.2m; only this answer is committed. One book, one
release, one voucher. Not measured: other releases.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `item-purchase-through-class.utf16le.xml` | 78,110 | `8461a8c99624bf11371cd33a4a9a13ef55d8071cffa8b044e3cf0c7c4779efcf` | the one-day read of 10 June 2025: one Purchase invoice, Lab Supplier 590.00, Lab Purchases -500.00 with the item Lab Class Item 5 Nos @ 100.00 in its inventory allocation, Lab CGST -45.00 and Lab SGST -45.00; byte-exact |
