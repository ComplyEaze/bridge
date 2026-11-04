# A voucher's GST and note details: captured answers (#2027)

These files are answers Tally sent on the synthetic book BRIDGE NOTES LAB, TallyPrime 7.2 Silver. Each time a person keyed the vouchers on
Tally's own screen, the answers were written straight to files, and the book was then restored from its backup. Each file is kept exactly
as received (UTF-16LE without a BOM). The data is synthetic (SYN and BRIDGE names, an invented GSTIN).

- 28 Oct 2026: a Sales, a Credit Note, a Debit Note and a Purchase of one registered party, dated 3 to 6 June 2025. Evidence:
  `lab/2027-gst-note-fields-capture-answers` at 07e3bc34. Neither answer is to the request `vouchers` sends with `include_gst_details`.
- 30 Oct 2026: the same four kinds, dated 3 to 6 June 2025, and an inter-state Sales on 7 June with Place of Supply keyed as Karnataka.
  Evidence: `lab/2027-voucher-gst-fields-fetch-answers` at 67039e2b. These answers are to the window request with the six words, sent by a
  file and not through a build, one day at a time.
- 3 Nov 2026: the four kinds seeded by XML, and a Sales on 7 June to a party registered in Karnataka; read through the candidate build of
  this change, plain and class shape. Evidence: `lab/2027-window-read-answers` at 307427aa. Seeded vouchers set the read up; the evidence is
  the window answer.

**Derived request (partial, #2027 dispatch details).** Since the dispatch change, the request `vouchers` sends with
`include_gst_details` names seven more words after the six, and then the Delivery Note list (protocol reference §8.2l). The 30 Oct
and 3 Nov answers here were answered without them, so a test that reads them as the answer to today's request is DERIVED, not a capture of it. It rests on
row 2027-D's measurement on another book (`lab/2027-dispatch-answers` at 4e0918e7): with the seven words added, every row and the
whole answer less them was otherwise the same, and with the list added to the plain read the same held (rows 2027-D2 and 2027-DN).
`only_a_read_that_asks_names_the_e_invoice_and_voucher_gst_details` asserts the one edit.

One voucher of each kind, one release. Not measured: unregistered parties, exports, a note without an original invoice, other releases
(protocol reference §8.2k).

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `native-four-vouchers.utf16le.xml` | 232,288 | `5086cafe18aa020b82fa590d6e897f2c761af7351bf4462494eef87037b6cb41` | the native read of the four vouchers, whole; `raw/native1.xml` of the evidence; byte-exact |
| `shipped-read-credit-note.utf16le.xml` | 65,226 | `5dc3d8c34f83656b7ca032f9726e05f91e0c8a9a380c78a0c32da626f3547146` | the window read the shipped build sends, for 4 June 2025 (the Credit Note); `raw/a07-20250604.xml` of the evidence; byte-exact |
| `window-read-inter-state-sale.utf16le.xml` | 51,938 | `83f00cdb6dbf9f54e9d800247017a8e4e3a4962cfaeeea7ffc96d010b6797912` | the window request with the six words, for 7 June 2025 (the inter-state Sales; each note field an empty element); `raw/009-candidate-20250607.xml` of the 30 Oct evidence; byte-exact |
| `build-read-four-vouchers.utf16le.xml` | 252,224 | `1cffb726951c0ace349993bad49cff08b2ba25256bd72bdf973d8811da875e56` | the window read the candidate build sent with `include_gst_details`, 3 to 6 June 2025; `relay/ans-1-0044.bin` of the 3 Nov evidence; byte-exact |
| `build-read-class-party-in-karnataka.utf16le.xml` | 53,430 | `59129b0498c014f627c6d79bb665ad1d2dd5c84cc0384cc79b16ae3e3fc84b51` | the class-shape window read the candidate build sent with `include_gst_details`, `include_e_invoice` and a voucher type, 7 June 2025, a Sales to a party in Karnataka; `relay/ans-1-0110.bin` of the 3 Nov evidence; byte-exact |
