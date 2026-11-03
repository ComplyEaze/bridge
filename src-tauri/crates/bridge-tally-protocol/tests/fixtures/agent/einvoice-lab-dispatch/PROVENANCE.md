# A voucher's dispatch details: captured window answers (#2027)

These files are answers Tally sent on the synthetic book BRIDGE EINVOICE LAB, TallyPrime 7.2 Silver, to the window request `vouchers`
sends with `include_gst_details`, the seven dispatch words after the six GST words (protocol reference §8.2l). Each answer was written
straight to a file by the lab's runner, and the book was then restored from its backup. Each file is kept exactly as received (UTF-16LE
without a BOM). The data is synthetic: SYN and BRIDGE names, invented dispatch details, and the lab's invented all-Z GSTIN in
`PARTYGSTIN` (four in the keyed answer, two in the other). Each answer also carries `VOUCHERKEY`, Tally's internal key, as a 15-digit
value. There is no company number or PAN.

- 4 Nov 2026, row 2027-D: a Sales invoice (no. 3) and a Credit Note (no. 1), each keyed on Tally's screen with the same invented dispatch
  details, beside the book's three earlier vouchers of 3 April 2025. Evidence: `lab/2027-dispatch-answers` at 6981790c.
- 4 Nov 2026, row 2027-D2: the same book back at its earlier state (three vouchers, no dispatch detail), read with a voucher type and
  `include_e_invoice` as well. Evidence: `lab/2027-d2-answers` at f3645218.

The requests were sent by a file, not through a build. Each is byte-equal to the request a build of this change renders for that call.
One book, one release, one voucher of each keyed kind. Not measured: details filled by an e-Way Bill, an export's shipping details, the
Delivery Note number, other releases.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `window-read-dispatch-keyed.utf16le.xml` | 229,048 | `95bfd65fcaf939d85b9055dfab691c7bb1faec4750489d4d61e4519d17cba47a` | the window read with `include_gst_details`, 3 April 2025: five vouchers, the Sales invoice and the Credit Note with each dispatch detail keyed; `answers/a-r1-WD.xml` of the 2027-D evidence; byte-exact |
| `window-read-class-e-invoice-dispatch.utf16le.xml` | 136,934 | `920de15ec49f2412c6045f56a04e514fc1fdbb3af52d2ea72571e46a9d129104` | the class-shape window read with `include_gst_details`, `include_e_invoice` and a voucher type, 3 April 2025: three vouchers, one Sales with an IRN, no dispatch detail; `answers/a-r1-CEGD.xml` of the 2027-D2 evidence; byte-exact |
