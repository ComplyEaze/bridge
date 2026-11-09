# Audit read fixtures: BRIDGE SFT LAB after sheet 699-C16, 1 April 2025 to 31 March 2026

Live answers to every part of the audit read (`src/agent_audit_read.rs` in the app crate), with the requests that drew them,
on the synthetic company `BRIDGE SFT LAB` loaded alone on a licensed TallyPrime 7.2 Silver, Windows lab. This is the book of
`../audit-read-sft-lab/` after sheet 699-C16 keyed one more group, ten ledgers and 19 vouchers by hand (53 vouchers in all,
on 23 ledgers), and one voucher was keyed again under a new number. The requests were sent by Bridge itself: one
`audit_test_run` call (test `cash_44ab`, the whole year, cash group `Cash-in-Hand`, bank group `Bank Accounts`) to the same
`bridge_mcp` build as row 699-R1 (from master `d77d0682`), whose Tally port was a recording relay in front of Tally. The relay
forwarded each request unchanged and wrote each request and answer body to a file as received (requests UTF-16LE with a BOM,
answers BOM-less UTF-16LE; the fixture tree's attributes keep them `-text`). Read-only: nothing was written to Tally by the read.

- Lab row C16-R, 4 Nov 2026. Evidence branch `lab/699-c16-answers` at `dde3313e` (sheet `lab/699-c16-sheet` at `efa022e0`).
  Each fixture is one relay file of `c16r/relay/r5/`, byte for byte (its git blob is the relay file's).
- The relay recorded 37 exchanges: the tool's paired company list (two company reads, two `GET /status`), then the eleven
  parts below, each between two company checks. The year is read in two voucher windows under the production window limits,
  split after 31 January 2026. The three marks reads asked the same request and got the same answer, so they are one fixture
  pair here.
- The company check's request is here (`sft-lab-audit-company-check.request.utf16le.xml`, the same 2,524 bytes all 24 times
  in `c16r/relay/r5/`, the paired list's two reads included, and the same bytes as row 699-R1's). Its answer is not: it carries
  the company number, so it stayed on the lab PC, recorded there by size and sha256. The tests stand a synthetic company list
  in for it (`live_sft_lab::companies` in `src/agent_audit_read_tests.rs`).
- The lab recorded the tool's `result` member in a canonical JSON form (Python's `json.dumps` with sorted keys, compact
  separators and `ensure_ascii=False`, in UTF-8): 2,671 bytes, sha256 `517b8c94…`, in the answers' `notes.txt`.

Tests: `src/agent_audit_read_tests.rs` (`live_sft_lab_c16r`) serves these answers in the order the reader asks for them and
asserts that the book is admitted with the lab's counts and that every request Bridge renders today, the company checks, the
census and both windows included, is byte for byte the one sent live. `src/agent_audit_run_tests.rs`
(`the_live_sft_lab_c16r_answers_give_the_result_the_lab_recorded`) runs the tool over the same answers and compares its
result with the recorded canonical bytes.

Not established: the company check's answer (synthetic here); other releases, editions, books, or a client book. These
answers are measured against the synthetic lab book only.

| Fixture | Bytes | SHA-256 | Source (`c16r/relay/r5/`) | What it is |
| --- | ---: | --- | --- | --- |
| `sft-lab-audit-company-check.request.utf16le.xml` | 2,524 | `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` | `0001-request.bin` | the company check's request, sent before and after every part |
| `sft-lab-audit-marks.request.utf16le.xml` | 972 | `4fb6dbda9add3cd0efcacaa11a4af0c4ffd1ec6f43d52630a75374b9e2cb19b5` | `0006-request.bin` (= `0024`, `0036`) | the marks request |
| `sft-lab-audit-marks.utf16le.xml` | 3,390 | `92b8e2392b5a948bbe366d0ae1f20ad1aed3fac9194ff804c6c30a48951a688e` | `0006-response.bin` (= `0024`, `0036`) | one COMPANY with its GUID, ALTVCHID 56, ALTMSTID 230 |
| `sft-lab-audit-company-object.request.utf16le.xml` | 888 | `04945bcb847ef2a78a3f552ec0ad45422388cb26b9cd9e4858212fd7f9757ca9` | `0009-request.bin` | the company Object request |
| `sft-lab-audit-company-object.utf16le.xml` | 4,194 | `73c051f43bb5ec85f91617c99c45d7d860ca7cc603987aa5664ec1d7dabb96eb` | `0009-response.bin` | the company Object export: one COMPANY, BOOKSFROM 20250401 |
| `sft-lab-audit-groups.request.utf16le.xml` | 1,062 | `6d2d43db2a491991f32472efed0fa3682527d087b31f79b65aca9ee2b1460aa4` | `0012-request.bin` | the groups request |
| `sft-lab-audit-groups.utf16le.xml` | 55,756 | `7fef8d7fa0b747ec129e43b25ddf733919ab87a2ea73455279bcd588e65eb464` | `0012-response.bin` | the 29 groups |
| `sft-lab-audit-ledgers.request.utf16le.xml` | 1,274 | `8de714717336ac2ea0fa1119b678a43174426f6b9c3ddd18f2a4f5ed126fcfbb` | `0015-request.bin` | the ledgers request |
| `sft-lab-audit-ledgers.utf16le.xml` | 30,694 | `d8b30f58062b008f738518c6aea2f25b92e68b02e56017189fc10e03c78f3ad1` | `0015-response.bin` | the 23 ledgers |
| `sft-lab-audit-trial-balance.request.utf16le.xml` | 1,140 | `a3b5fbb86978145a86f10a715c85b4a8e2d91be2785d62cc41d94ed3c59b328b` | `0018-request.bin` | the trial balance request |
| `sft-lab-audit-trial-balance.utf16le.xml` | 30,174 | `bdb65cba1f568089401191ca9db1a0615b53651f7d3286ea239f0bdd81686bd4` | `0018-response.bin` | the trial balance |
| `sft-lab-audit-voucher-types.request.utf16le.xml` | 968 | `2e747645e284764bbb64646b7348d6fc44c798b8b1f6fa307b42c6fa7ac492a9` | `0021-request.bin` | the voucher types request |
| `sft-lab-audit-voucher-types.utf16le.xml` | 33,032 | `ba95f86edf730947b51634bb25b1cde2617d8445cee759c3a83ded55e41077f5` | `0021-response.bin` | the voucher types |
| `sft-lab-audit-census.request.utf16le.xml` | 1,516 | `d9316797d160eb623323779606db0f6ab0cf8d6ba3cbbc9da6481fa29b811d9a` | `0027-request.bin` | the census request for the year, spanned to the voucher mark |
| `sft-lab-audit-census.utf16le.xml` | 130,704 | `35e5061ae21a4809b908d4a0f0f461f758eae2e1ebcd0992c403bbe45bab1c4d` | `0027-response.bin` | the census of the 53 vouchers |
| `sft-lab-audit-vouchers-1.request.utf16le.xml` | 1,736 | `1b4199bb8f2f6d942a2556c0b4d1a651143b8df812096def57d39bef6d3a99e7` | `0030-request.bin` | the first window's request, 1 April 2025 to 31 January 2026 |
| `sft-lab-audit-vouchers-1.utf16le.xml` | 1,391,682 | `ea9cbb77b1874bff54c563a225fa361c266750ea4c76963132749118bbe494b6` | `0030-response.bin` | the first window's 42 vouchers |
| `sft-lab-audit-vouchers-2.request.utf16le.xml` | 1,736 | `26933e853602a146e3fab3bb449b0a45ea8bb14f27782265bc1f3dfa80abe60a` | `0033-request.bin` | the second window's request, 1 February to 31 March 2026 |
| `sft-lab-audit-vouchers-2.utf16le.xml` | 428,818 | `687bfeb4655a286ee10fdf5d1a8dae72b8126052332bfee90756301b62db327c` | `0033-response.bin` | the second window's 11 vouchers |
