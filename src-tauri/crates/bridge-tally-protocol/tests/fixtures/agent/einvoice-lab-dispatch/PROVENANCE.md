# A voucher's dispatch details: captured window answers (#2027)

These files are answers Tally sent on the synthetic book BRIDGE EINVOICE LAB, TallyPrime 7.2 Silver, to the window request `vouchers`
sends with `include_gst_details`, the seven dispatch words after the six GST words, and in the last two answers the Delivery Note list
after them (protocol reference §8.2l). Each answer was written
straight to a file by the lab's runner, and the book was then restored from its backup. Each file is kept exactly as received (UTF-16LE
without a BOM). The data is synthetic: SYN and BRIDGE names, invented dispatch details, and the lab's invented all-Z GSTIN in
`PARTYGSTIN` (four in each answer with five vouchers, two in the other). Each answer also carries `VOUCHERKEY`, Tally's internal key, as a 15-digit
value. There is no company number or PAN.

- 4 Nov 2026, row 2027-D: a Sales invoice (no. 3) and a Credit Note (no. 1), each keyed on Tally's screen with the same invented dispatch
  details, beside the book's three earlier vouchers of 3 April 2025. Evidence: `lab/2027-dispatch-answers` at 6981790c.
- 4 Nov 2026, row 2027-D2: the same book back at its earlier state (three vouchers, no dispatch detail), read with a voucher type and
  `include_e_invoice` as well. Evidence: `lab/2027-d2-answers` at f3645218.
- 4 Nov 2026, row 2027-DN: the same book back at its earlier state, with two Sales invoices keyed on Tally's screen, no. 3 with one
  invented Delivery Note and no. 4 with two, each dated and no other dispatch detail, read with `,INVOICEDELNOTES.*` after the seven
  words. Evidence: `lab/2027-dn-answers` at 81c551da.
- 4 Nov 2026, row 2027-DN2: the same book back at its earlier state, with the same two invoices and Delivery Notes keyed again, read
  with a voucher type and `include_e_invoice` as well, with and without the list. Evidence: `lab/2027-dn2-answers` at f748c46e.

The requests were sent by a file, not through a build. Each is byte-equal to the request a build of this change renders for that call.
One book, one release, one voucher of each keyed kind. Not measured: details filled by an e-Way Bill, an export's shipping details, other
releases.

**Derived request (the Delivery Note list).** Since the Delivery Note change, the request names `,INVOICEDELNOTES.*` after the seven
words. The first two answers here were answered without it, so a test that reads them as the answer to today's request is DERIVED:
they say the notes `not_reported`, where today's request answers an entry a row. It rests on rows 2027-D2, 2027-DN and 2027-DN2,
which sent the list with each read that names the GST words: with it added, every row and the whole answer less it was otherwise
the same.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `window-read-dispatch-keyed.utf16le.xml` | 229,048 | `95bfd65fcaf939d85b9055dfab691c7bb1faec4750489d4d61e4519d17cba47a` | the window read with `include_gst_details`, 3 April 2025: five vouchers, the Sales invoice and the Credit Note with each dispatch detail keyed; `answers/a-r1-WD.xml` of the 2027-D evidence; byte-exact |
| `window-read-class-e-invoice-dispatch.utf16le.xml` | 136,934 | `920de15ec49f2412c6045f56a04e514fc1fdbb3af52d2ea72571e46a9d129104` | the class-shape window read with `include_gst_details`, `include_e_invoice` and a voucher type, 3 April 2025: three vouchers, one Sales with an IRN, no dispatch detail; `answers/a-r1-CEGD.xml` of the 2027-D2 evidence; byte-exact |
| `window-read-delivery-notes.utf16le.xml` | 230,202 | `a18e5ef5b435aadc4c5b0f2d7f220a5a267dd99539722e6122159395c58fe74b` | the window read with `include_gst_details` and the Delivery Note list, 3 April 2025: five vouchers, Sales 3 with one Delivery Note and Sales 4 with two, the others one empty entry each; `answers/a-r1-GDN.xml` of the 2027-DN evidence; byte-exact |
| `window-read-class-e-invoice-delivery-notes.utf16le.xml` | 238,592 | `1bb93749d8c744f7091c5899a610b4630f5a68964e5775128f405b01e9e3db17` | the class-shape window read with `include_gst_details`, `include_e_invoice`, a voucher type and the Delivery Note list, 3 April 2025: five vouchers, one Sales with an IRN, Sales 3 with one Delivery Note and Sales 4 with two, the others one empty entry each; `answers/a-r1-CEGDN.xml` of the 2027-DN2 evidence; byte-exact |
