# `ledger-scoped-vouchers/` — provenance

Four requests and their four answers from the lab's ledger-scoped run on `BRIDGE CORPUS DENSE`, a synthetic lab company of about 30,000 vouchers (also read in `BUILTIN_REPORTS_CAPTURE_PROVENANCE.md` and `STATEMENT_CAPTURE_PROVENANCE.md`). They show a `Vouchers : Ledger` collection `CHILDOF` one ledger, with and without an AlterID span, and are here for the tests of the candidate request builders and the voucher census parser (#485). Protocol reference section 11c.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.2, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance, Windows).
- **Date:** 2026-10-28, read only, one request at a time, by `curl.exe`; the company's voucher and master marks were equal at the start and the end of the run.
- **Source:** the requests are `requests/e02_…`, `e03_…`, `e06_…` and `e11_…` of the lab branch `lab/485-ledger-census-spans` at `97fdc0635a436c28fa5811cc5f47619eaf3f4a13`; the answers are `answers/e02_…`, `e03_…`, `e06_…` and `e11_…` of `lab/485-ledger-census-spans-answers` at `ba180df99d4595c9e4e12121b58b6a577151e0c3`, each listed with the sha256 below in that branch's `SHA256.txt`. Copied here byte for byte.
- **Encoding:** the requests are UTF-16LE with a BOM; the answers are BOM-less UTF-16LE with CRLF line ends, exactly as received. `.gitattributes` marks this tree `-text`.
- **What the captures show** (counted by the lab with an XML parser and compared by it with the unspanned reads, which are not committed): the day's census holds 82 vouchers of DN Sales, all in the first span; the other three spans of that day are empty, byte-identical collections; the year's census of DN Party 001 holds 248 vouchers over four spans (67, 68, 69 and 44), equal to its unspanned ledger-scoped read; the May movement read returns 21 vouchers, equal to the census rows of that month, and their lines sum to the committed one-month Trial Balance's debit for the ledger.
- **Not established:** a ledger name with a character outside ASCII letters, digits and spaces, or one that matches no ledger; a voucher with two lines on the ledger listed twice; a census without a span; a book other than this one or a release other than 7.2 Silver; the read under concurrent screen activity.

| file | bytes | sha256 | content |
|---|---|---|---|
| `census-dn-party-year-span1.answer.utf16le.xml` | 165380 | `3d65d555487f2906d77d442168421e3faaf91272a3d77a372a3f0e79ee83b8a0` | its answer: 67 vouchers |
| `census-dn-party-year-span1.request.utf16le.xml` | 1608 | `a9d3eaff4191aa04c62987f3a27bc0c4ee70922ac0b99d1bef5b867e30e0ae2f` | the census of DN Party 001 for 1 April 2025 to 31 March 2026, span (0, 8192] |
| `census-dn-sales-day-span1.answer.utf16le.xml` | 201862 | `8aa7a6ce01d38f816c6af4a76d1178078163835e3ac363139e8d8d28031ab337` | its answer: 82 vouchers, every one with GUID, ALTERID and DATE |
| `census-dn-sales-day-span1.request.utf16le.xml` | 1600 | `16979f8194f45f385f5cc9e5809980c10ba530a82115832aee8d9837101f4d12` | the census of DN Sales for 15 May 2025, AlterID span (0, 8192] (UTF-16LE with a BOM) |
| `census-dn-sales-day-span2.answer.utf16le.xml` | 2926 | `52f69354f7d87de773f68e7d9856f2719374a24c920933f47283e4d662c4f40b` | its answer: an empty collection |
| `census-dn-sales-day-span2.request.utf16le.xml` | 1608 | `b7e30f615fe1d22c886f5b8644ba2c1a564bf10fbbd039d1fc910e22f690fd9d` | the same day, span (8192, 16384] |
| `movement-dn-party-month.answer.utf16le.xml` | 108918 | `1dd5fac4e6c743adf46ad834f300f6713192872945680b75764faa60a3ee3f74` | its answer: 21 vouchers with their ledger lines |
| `movement-dn-party-month.request.utf16le.xml` | 1876 | `8060720ed68c5100600098959bfa68708fa72b07bf78589ec839411c3ce8af41` | the movement read of DN Party 001 for May 2025, no span |
