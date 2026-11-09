# GST per counter ledger: a supplier invoice keyed twice (1867-L3)

These 3 files are responses Tally sent to `gst_by_counter_ledger` for 2025-12-01 to 2025-12-31 on the synthetic book BRIDGE
GST LEDGER LAB, TallyPrime 7.2 Silver, on 27 Oct 2026 (UTC). They are lab item 1867-L3's step 4, answers on the branch
`lab/1867-l3-answers` at `92d92d04` (the files are byte-equal at `43ea58e9`, the commit #2069 cites), `recorder/` with its
`SHA256.txt`. The program was `bridge_mcp` built from the head of pull request 1867 at `796e94f1`, through a serial read-only
recording relay that refused none of the 220 requests it relayed. The book is the one `gst-ledger-l1` was read from, as lab
items 628-C0 and C0b left it (answers `lab/628-c0-c0b-answers` at `075ee200`): December holds Purchase 57 for supplier invoice
GS1/P/0001, the same invoice again as Purchase 58 (Tally warned that the reference had been used, and saved it Agst Ref the same
bill), an optional and a cancelled Purchase, a Journal, and a Payment Agst Ref the bill. The RESULTS.md of the answers branch
lists December's Day Book as Tally showed it.

The bodies are kept exactly as received (UTF-16LE without a BOM). The data is synthetic (SYN and GS1 names, a placeholder
GSTIN). One instance, six vouchers, one release: they back the repeated-bill rule these vouchers exercise (#2069), not any
claim about other books. The window read's request is the FETCH list and class formulas `ClassEntryWildcard` sends on master;
the build was older than master in which counter-ledger groups it admits, which these December vouchers do not reach.

| Fixture | Bytes | SHA-256 | What it is |
| --- | ---: | --- | --- |
| `l3-dec-L-1.utf16le.xml` | 40,114 | `ba12a07e182f3e6d33680d080a13222df77d93fc5cea8900d93ef97e38a14cae` | the compliance ledger listing (List of Ledgers with TAXTYPE and GSTDUTYHEAD); recording row 30; byte-exact |
| `l3-dec-g-1.utf16le.xml` | 54,280 | `8d4b5b86ea290e4e0d76dcc29d9355d1ed93359b67ee0c56498ab98f76ea9c60` | the group collection (List of Groups); recording row 38; byte-exact |
| `l3-dec-w-1.utf16le.xml` | 291,372 | `6fb1f5d4b246a6262d9be1e201919162b5fbf7b49c13b9a5a3314ddcfc9b2143` | the class-shaped voucher window for December 2025 (six vouchers); recording row 56; byte-exact |
