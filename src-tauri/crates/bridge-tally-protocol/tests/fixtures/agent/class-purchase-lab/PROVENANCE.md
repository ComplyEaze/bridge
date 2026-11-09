# Item purchases on a synthetic book: captured voucher answers (#2057)

Each file is an answer Tally sent on the synthetic book BRIDGE CLASS PURCHASE LAB, TallyPrime 7.2 Silver (protocol reference §8.2m).
The lab's runner wrote it straight to a file, and the book was then restored from its backup. It is kept exactly as received (UTF-16LE
without a BOM). The data is synthetic: Lab names, and the lab's invented all-Z GSTIN in `PARTYGSTIN`. Each answer also carries
`VOUCHERKEY`, Tally's internal key, as a 15-digit value. There is no company number or PAN.

- 4 Nov 2026, row 2057-B: the book's duty ledger Lab CGST had its Tax type set to CGST on Tally's screen, then four Purchase invoices
  were keyed on Tally's screen, three through the Purchase type's voucher class and one on the plain type. `item-purchase-through-class.utf16le.xml` is
  the first, Purchase 1 of 10 June 2025, keyed through the class. Evidence: `lab/2057-b-answers` at d9005ab3, `answers/fixture-i1b-r-a07-20250610.xml`,
  byte-equal to that row's `raw/r14-r-a07-20250610.xml`.

Its request was sent by a file, not through a build: the committed one-day request of the shape lab
(`voucher_goods_lines_two_godowns_shape_lab_request.utf16le.xml`) with only the company and the day changed. A build renders that same
request for this company and day; `agent_voucher_goods_lines_tests.rs` checks it.

2057-B's other three invoices were read the same way and are recorded in §8.2m; only its first answer is committed.

- 4 Nov 2026, row 2057-D: on a blank Purchase voucher's F12, "Select common Ledger Account for Item Allocation" was set from Yes to
  No, and one Purchase invoice was keyed on the plain type with the purchase ledger chosen on the item line. That invoice, Purchase 1
  of 15 June 2025, was read twice: by the one-day request `vouchers` sends, and by the stock-lines request (`ALLINVENTORYENTRIES.*` in
  place of the ledger entries). The option was then set back to Yes and the book restored. Evidence: `lab/2057-d-answers` at
  02ef29ec, byte-equal to that row's `raw/r9-r-a07-20250615.xml` and `raw/r9-r-sl-20250615.xml`.

Both of 2057-D's requests were sent by a file, not through a build: the committed one-day request of the shape lab and the committed
stock-lines request (`stock_movement_mixed_window_stock_lines_shape_lab_request.utf16le.xml`), each with only the company and the day
changed. A build renders the same two requests for this company and day; `agent_voucher_goods_lines_tests.rs` checks both.

Neither row captured the tax ledgers' masters, so the register's reading of each voucher (both tax heads recognised, the
purchase ledger a taxable entry) is recorded from the lab's tool answers in §8.2m, not tested on these files.

No answer to the stock-lines request was read with the option at Yes, so whether that view of an item invoice depends on the option is
not measured. One book, one release, one voucher of each kind. Not measured: other releases.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `item-purchase-through-class.utf16le.xml` | 78,110 | `8461a8c99624bf11371cd33a4a9a13ef55d8071cffa8b044e3cf0c7c4779efcf` | the one-day read of 10 June 2025: one Purchase invoice, Lab Supplier 590.00, Lab Purchases -500.00 with the item Lab Class Item 5 Nos @ 100.00 in its inventory allocation, Lab CGST -45.00 and Lab SGST -45.00; byte-exact |
| `item-purchase-common-ledger-off.utf16le.xml` | 77,918 | `68343f4296ca2c18a7f1937d18ffedba8e359b7bf1a5d924fe63f0da831107e7` | the one-day read of 15 June 2025: one Purchase invoice keyed with the common ledger off, with the same entries, amounts and inventory allocation as the row above; byte-exact |
| `item-purchase-common-ledger-off-stock-lines.utf16le.xml` | 40,164 | `2f791622cb1abb7bb13a46719cccc63abfecac09c3b5be4dd84f15acdf880d5b` | the stock-lines read of the same voucher: the item Lab Class Item 5 Nos @ 100.00, -500.00, at voucher level, with Lab Purchases -500.00 only in its accounting allocation; ledger entries Lab Supplier 590.00, Lab CGST -45.00 and Lab SGST -45.00; byte-exact |
