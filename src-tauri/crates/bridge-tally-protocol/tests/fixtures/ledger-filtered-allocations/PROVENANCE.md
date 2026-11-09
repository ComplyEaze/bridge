# `ledger-filtered-allocations/` — provenance

Ten files from two lab runs, for the ledger-scoped read the `outstandings` party detail sends on a larger book (#1356): the requests the code must render byte for byte, and Tally's answers to them. Protocol reference section 11c.6.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.2, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance, Windows).
- **Date:** 2026-10-28, read only, one request at a time, by `curl.exe`. Each company's voucher and master marks were equal at the start and the end of each run.
- **Companies:** `BRIDGE BILLTRAIL LAB` (a synthetic book of 7 vouchers, seeded for #945 and #1356, with a credit note and a post-dated receipt keyed by hand) and `BRIDGE CORPUS DENSE` (a synthetic book of about 30,000 vouchers).
- **Source, the #1356 run:** the `t02`, `t04` and `d05` requests of `lab/1356-ledger-filtered-allocations-run-sheet` at `fd02e22f5282209018acef6326d8a2b07df83ee2`, and their answers in `raw/` of `lab/1356-ledger-filtered-allocations-answers` at `8534253da1c94b408fd26aad45ca76ae0523c182`.
- **Source, the #485 run:** the `e04` and `e05` requests of `lab/485-ledger-census-spans` at `97fdc0635a436c28fa5811cc5f47619eaf3f4a13`, and their answers in `answers/` of `lab/485-ledger-census-spans-answers` at `ba180df99d4595c9e4e12121b58b6a577151e0c3`. Spans 1 and 2 of the same day are in `ledger-scoped-vouchers/`.
- **One book state:** the two runs' marks answers for `BRIDGE CORPUS DENSE` are byte for byte the same (ALTVCHID 29900, ALTMSTID 328), so the census spans of the #485 run and the read of the #1356 run describe one state of the book.
- **Encoding:** the requests are UTF-16LE with a BOM; the answers are BOM-less UTF-16LE, exactly as received. Each file's sha256 is the one in its lab branch's `SHA256.txt`. Copied here byte for byte. `.gitattributes` marks this tree `-text`.
- **What the captures show** (compared by the lab with an XML parser, voucher by voucher): the ledger-scoped read returns the same vouchers as the book-wide read, each equal in its own fields and in every entry of the ledger with its bill allocations: 7 of 7 for Lab Party P1, including three Receipts whose party ledger is the bank, and 82 of 82 for DN Sales. The read carries none of the book-wide read's `COMPUTE` lines.
- **Not established:** a ledger name with a character outside ASCII letters, digits and spaces; a party whose vouchers are a small share of a busy window (on both books every voucher of the window carried the ledger); the window field list with `REFERENCEDATE`; a release other than 7.2 Silver.

| file | bytes | sha256 | content |
|---|---|---|---|
| `billtrail-ledger-p1-fy-entries.answer.utf16le.xml` | 239294 | `c5e1413c5b112ab114add489d48a811c85a7953da4a8f3dca664c2704fb0384a` | its answer: 7 vouchers with their ledger entries and bill allocations |
| `billtrail-ledger-p1-fy-entries.request.utf16le.xml` | 1830 | `6a5ee4a9ec54980cd8b7c0cc137d6e7d1f42c1ce1a700670cbf3fcbb996da97d` | the ledger-scoped read of Lab Party P1 for 1 April 2025 to 31 March 2026 with the window field list (`t04`) |
| `billtrail-window-fy.answer.utf16le.xml` | 249288 | `80c7896485de91655fb9ec872550cbd6a56a650702186e5187f4289d26dd87e8` | its answer: 7 vouchers, the ground truth |
| `billtrail-window-fy.request.utf16le.xml` | 3096 | `de90752d735e696032512d49bb3c3cedb1cf2500411d690bd7b34408606e038b` | the book-wide window read of the same year (`t02`), with its class `COMPUTE` lines and without `REFERENCEDATE` |
| `dense-census-dn-sales-20250515-span3.answer.utf16le.xml` | 2926 | `52f69354f7d87de773f68e7d9856f2719374a24c920933f47283e4d662c4f40b` | its answer: an empty collection |
| `dense-census-dn-sales-20250515-span3.request.utf16le.xml` | 1610 | `3ece05acef1df81bdc6594a4df5638f024f29c7cd3a7076f8b37edfa028af649` | the ledger census of DN Sales for 15 May 2025, span (16384, 24576] (`e04`) |
| `dense-census-dn-sales-20250515-span4.answer.utf16le.xml` | 2926 | `52f69354f7d87de773f68e7d9856f2719374a24c920933f47283e4d662c4f40b` | its answer: an empty collection, the same bytes as span 3 |
| `dense-census-dn-sales-20250515-span4.request.utf16le.xml` | 1610 | `07e768b3e79b5037dd62dffcbf382dfbdaff0795bf67a6d32704e1aa3eb9ccf8` | the same day, span (24576, 29900] (`e05`) |
| `dense-ledger-dn-sales-20250515-entries.answer.utf16le.xml` | 2785026 | `1b7b21735e8ada02227d2ac304aee9e88a48b62c9bc47f224555af8e236fec6e` | its answer: 82 vouchers with their ledger entries |
| `dense-ledger-dn-sales-20250515-entries.request.utf16le.xml` | 1820 | `6f499dc678f59f1c5faf702601964f19cd7ddb757b7d9af577294440a16b5ff5` | the ledger-scoped read of DN Sales for 15 May 2025 with the window field list (`d05`) |
