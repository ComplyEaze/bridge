# Audit read fixtures: BRIDGE SFT LAB, 1 April 2025 to 31 March 2026

Live answers to every part of the audit read (`src/agent_audit_read.rs` in the app crate), with the requests that drew them,
on the synthetic company `BRIDGE SFT LAB` loaded alone on a licensed TallyPrime 7.2 Silver, Windows lab. The requests were
sent by Bridge itself: one `audit_test_run` call (test `cash_44ab`, the whole year, cash group `Cash-in-Hand`, bank group
`Bank Accounts`) to a `bridge_mcp` built from master `d77d0682`, whose Tally port was a recording relay in front of Tally.
The relay forwarded each request unchanged and wrote each request and answer body to a file as received (requests
UTF-16LE with a BOM, answers BOM-less UTF-16LE; the fixture tree's attributes keep them `-text`). The company, its ledgers
and its 34 vouchers are synthetic, keyed by hand on 26 Oct 2026 in 18 one-variable cash shapes. Read-only: nothing was
written to Tally.

- Lab row 699-R1, 4 Nov 2026. Evidence branch `lab/699-sft-read-answers` at `3e7e0e1a` (sheet `lab/699-sft-read` at
  `66cc762c`). Each fixture is one relay file of `relay/r2/`, byte for byte; its `SHA256.txt` and `relay/r2/manifest.tsv`
  list each with the same hash.
- The relay recorded 34 exchanges: the tool's paired company list (two company reads, two `GET /status`), then the ten
  parts below, each between two company checks. The three marks reads asked the same request and got the same answer, so
  they are one fixture pair here.
- The company check's request is here (`sft-lab-audit-company-check.request.utf16le.xml`, the same 2,524 bytes all 22
  times in `relay/r2/`, the paired list's two reads included). Its answer is not: it carries the company number, so it stayed on the lab PC, recorded there by size and sha256.
  The tests stand a synthetic company list in for it (`live_sft_lab::companies` in `src/agent_audit_read_tests.rs`).
- The lab recorded the tool's `result` member in a canonical JSON form (Python's `json.dumps` with sorted keys, compact
  separators and `ensure_ascii=False`, in UTF-8): 2,667 bytes, sha256 `f4115258…`, in the answers' `RESULTS.md`.

Tests: `src/agent_audit_read_tests.rs` (`live_sft_lab`) serves these answers in the order the reader asks for them and
asserts that the book is admitted with the lab's counts and that every request Bridge renders today, the company checks
included, is byte for byte the one sent live. `src/agent_audit_run_tests.rs`
(`the_live_sft_lab_answers_give_the_result_the_lab_recorded`) runs the tool over the same answers and compares its result
with the recorded canonical bytes.

Not established: the company check's answer (synthetic here); a window divided into several parts (this book is read in
one census and one voucher request); other releases, editions, books, or a client book. These answers are measured against
the synthetic lab book only.

| Fixture | Bytes | SHA-256 | Source (`relay/r2/`) | What it is |
| --- | ---: | --- | --- | --- |
| `sft-lab-audit-company-check.request.utf16le.xml` | 2,524 | `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` | `0001-request.bin` | the company check's request, sent before and after every part |
| `sft-lab-audit-marks.request.utf16le.xml` | 972 | `4fb6dbda9add3cd0efcacaa11a4af0c4ffd1ec6f43d52630a75374b9e2cb19b5` | `0006-request.bin` (= `0024`, `0033`) | the marks request |
| `sft-lab-audit-marks.utf16le.xml` | 3,390 | `c61139dcf88cfdecebf43071f26db295a090b7839aac43c0e86a2c32d891fe0b` | `0006-response.bin` (= `0024`, `0033`) | one COMPANY with its GUID, ALTVCHID 34, ALTMSTID 219 |
| `sft-lab-audit-company-object.request.utf16le.xml` | 888 | `04945bcb847ef2a78a3f552ec0ad45422388cb26b9cd9e4858212fd7f9757ca9` | `0009-request.bin` | the company Object request |
| `sft-lab-audit-company-object.utf16le.xml` | 4,194 | `54f66c8891b1619f002d417925b5c84d5cdb061064fd3a5c01baef331fbd07b7` | `0009-response.bin` | the company Object export: one COMPANY, BOOKSFROM 20250401 |
| `sft-lab-audit-groups.request.utf16le.xml` | 1,062 | `6d2d43db2a491991f32472efed0fa3682527d087b31f79b65aca9ee2b1460aa4` | `0012-request.bin` | the groups request |
| `sft-lab-audit-groups.utf16le.xml` | 54,280 | `86fee8ee5900d9d9800788c15b57235f40aed9a9e915f8bbcea3e4694e694d79` | `0012-response.bin` | the groups |
| `sft-lab-audit-ledgers.request.utf16le.xml` | 1,274 | `8de714717336ac2ea0fa1119b678a43174426f6b9c3ddd18f2a4f5ed126fcfbb` | `0015-request.bin` | the ledgers request |
| `sft-lab-audit-ledgers.utf16le.xml` | 18,642 | `6b52704a50fee8e0810c1f3e1d05a2c0be2b5bcb513164bcff78bcc43888599a` | `0015-response.bin` | the 13 ledgers |
| `sft-lab-audit-trial-balance.request.utf16le.xml` | 1,140 | `a3b5fbb86978145a86f10a715c85b4a8e2d91be2785d62cc41d94ed3c59b328b` | `0018-request.bin` | the trial balance request |
| `sft-lab-audit-trial-balance.utf16le.xml` | 18,340 | `79a92c02abedc66c0f2876d8e2e307eb1b4c28480c124f7eca17af3dd9b1cb79` | `0018-response.bin` | the trial balance |
| `sft-lab-audit-voucher-types.request.utf16le.xml` | 968 | `2e747645e284764bbb64646b7348d6fc44c798b8b1f6fa307b42c6fa7ac492a9` | `0021-request.bin` | the voucher types request |
| `sft-lab-audit-voucher-types.utf16le.xml` | 33,032 | `ed006ad0196dbce13125da2a05cf40e41f5955a5d9119f55b17560b3d60b2ee8` | `0021-response.bin` | the voucher types |
| `sft-lab-audit-census.request.utf16le.xml` | 1,516 | `3b02c9eb0f839b1c14da0986ea47443c51f68e0ba0c1fdac38cb1fddcd404955` | `0027-request.bin` | the window's census request, spanned to the voucher mark |
| `sft-lab-audit-census.utf16le.xml` | 84,840 | `49afd2bce96af34f078dce0b4ea7c8ea4f5ca065ec2b375a5df897655bfb3c3a` | `0027-response.bin` | the census of the 34 vouchers |
| `sft-lab-audit-vouchers.request.utf16le.xml` | 1,736 | `d49466f61e2efcb2f20ef0a2cdab478dd8d3bbc9c7c5845c5ead449f0ba99c76` | `0030-request.bin` | the vouchers request |
| `sft-lab-audit-vouchers.utf16le.xml` | 1,103,236 | `5f2a8876b77cc6347b46b1a8b5e84e4381aa1ada41afe6523ce6602257e31ca7` | `0030-response.bin` | the 34 vouchers |
