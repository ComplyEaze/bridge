# A voucher's GST and note details: captured answers (#2027)

These two files are answers Tally sent on the synthetic book BRIDGE NOTES LAB, TallyPrime 7.2 Silver, on 28 Oct 2026. A person keyed four
vouchers on Tally's own screen (a Sales, a Credit Note, a Debit Note and a Purchase of one registered party, dated 3 to 6 June 2025), the
answers were written straight to files, and the book was then restored from its backup. Evidence: `lab/2027-gst-note-fields-capture-answers`
at 07e3bc34. Each file is kept exactly as received (UTF-16LE without a BOM). The data is synthetic (SYN and BRIDGE names, an invented GSTIN).

One voucher of each kind, one release. Neither answer is to the request `vouchers` sends with `include_gst_details`: that request was not
sent on this date. The native answer proves the elements' form and values, not the window request's (protocol reference §8.2k). Not measured:
unregistered parties, exports, a note without an original invoice, other releases.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `native-four-vouchers.utf16le.xml` | 232,288 | `5086cafe18aa020b82fa590d6e897f2c761af7351bf4462494eef87037b6cb41` | the native read of the four vouchers, whole; `raw/native1.xml` of the evidence; byte-exact |
| `shipped-read-credit-note.utf16le.xml` | 65,226 | `5dc3d8c34f83656b7ca032f9726e05f91e0c8a9a380c78a0c32da626f3547146` | the window read the shipped build sends, for 4 June 2025 (the Credit Note); `raw/a07-20250604.xml` of the evidence; byte-exact |
