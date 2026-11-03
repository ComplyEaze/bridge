# A voucher's GST and note details: captured answers (#2027)

These files are answers Tally sent on the synthetic book BRIDGE NOTES LAB, TallyPrime 7.2 Silver. Each time a person keyed the vouchers on
Tally's own screen, the answers were written straight to files, and the book was then restored from its backup. Each file is kept exactly
as received (UTF-16LE without a BOM). The data is synthetic (SYN and BRIDGE names, an invented GSTIN).

- 28 Oct 2026: a Sales, a Credit Note, a Debit Note and a Purchase of one registered party, dated 3 to 6 June 2025. Evidence:
  `lab/2027-gst-note-fields-capture-answers` at 07e3bc34. Neither answer is to the request `vouchers` sends with `include_gst_details`.
- 30 Oct 2026: the same four kinds, dated 3 to 6 June 2025, and an inter-state Sales on 7 June with Place of Supply keyed as Karnataka.
  Evidence: `lab/2027-voucher-gst-fields-fetch-answers` at 67039e2b. These answers are to the window request with the six words, sent by a
  file and not through a build, one day at a time.

One voucher of each kind, one release. Not measured: unregistered parties, exports, a note without an original invoice, a party in another
state than the company (`STATENAME` is the party's by element name only), other releases (protocol reference §8.2k).

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `native-four-vouchers.utf16le.xml` | 232,288 | `5086cafe18aa020b82fa590d6e897f2c761af7351bf4462494eef87037b6cb41` | the native read of the four vouchers, whole; `raw/native1.xml` of the evidence; byte-exact |
| `shipped-read-credit-note.utf16le.xml` | 65,226 | `5dc3d8c34f83656b7ca032f9726e05f91e0c8a9a380c78a0c32da626f3547146` | the window read the shipped build sends, for 4 June 2025 (the Credit Note); `raw/a07-20250604.xml` of the evidence; byte-exact |
| `window-read-credit-note.utf16le.xml` | 66,084 | `8956830e94024119f3fb2be0fbe912f5143c753050b3fdb753c80d8ea60913ff` | the window request with the six words, for 4 June 2025 (the Credit Note); `raw/006-candidate-20250604.xml` of the 30 Oct evidence; byte-exact |
| `window-read-inter-state-sale.utf16le.xml` | 51,938 | `83f00cdb6dbf9f54e9d800247017a8e4e3a4962cfaeeea7ffc96d010b6797912` | the window request with the six words, for 7 June 2025 (the inter-state Sales; each note field an empty element); `raw/009-candidate-20250607.xml` of the 30 Oct evidence; byte-exact |
