# GST per counter ledger: December of the registered lab book (1867-L3)

These 3 files are responses Tally sent to `gst_by_counter_ledger` for 1 to 31 December 2025 on the synthetic book BRIDGE GST
LEDGER LAB, TallyPrime 7.2 Silver, on 27 Oct 2026 (step 4 of lab item 1867-L3; its answers are on the branch
`lab/1867-l3-answers` at 43ea58e9, not for merge). The program was `bridge_mcp` built from the head of pull request 1867 at
796e94f1, the build of the `gst-ledger-l1` captures, through the same read-only recording relay; 220 requests were relayed
for the item and none refused. By then the book carried a company GSTIN and a party GSTIN of the synthetic ZZZZZ form, and
628-C0 and 628-C0b had keyed its December on Tally's screen: Purchase 57 for supplier invoice GS1/P/0001, and a second Purchase
58 with the same supplier reference and date, which Tally saved after warning that the reference had been used earlier, against
the first copy's bill. Nothing was imported. The bodies are kept exactly as received (UTF-16LE without a BOM). The data is
synthetic (SYN and GS1 names). One instance, one month, one release: they back the repeated-bill decision and the shapes these
six vouchers exercise, not any claim about other books.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `dec-L-1.utf16le.xml` | 40,114 | `ba12a07e182f3e6d33680d080a13222df77d93fc5cea8900d93ef97e38a14cae` | the compliance ledger listing (List of Ledgers with TAXTYPE and GSTDUTYHEAD), 19 ledgers, SYN GS1 Supplier B among them; recording row 30; byte-exact |
| `dec-g-1.utf16le.xml` | 54,280 | `8d4b5b86ea290e4e0d76dcc29d9355d1ed93359b67ee0c56498ab98f76ea9c60` | the group collection (List of Groups); recording row 38; byte-exact |
| `dec-w-1.utf16le.xml` | 291,372 | `6fb1f5d4b246a6262d9be1e201919162b5fbf7b49c13b9a5a3314ddcfc9b2143` | the class-shaped voucher window for December 2025: Purchases 57 and 58 (the same supplier bill), an optional Purchase 60, a cancelled Purchase 59, Journal 5 and Payment 3; recording row 56 (row 58, its pair, is byte-equal); byte-exact |
