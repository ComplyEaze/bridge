# Audit read fixtures: BRIDGE PILOT LAB, 1 April 2026 to 31 March 2027

Live answers to every part of the audit read (`src/agent_audit_read.rs` in the app crate), with the requests that were
sent, on the synthetic company `BRIDGE PILOT LAB` loaded alone on a licensed TallyPrime 7.1 Silver (Education mode No),
port 9001, Windows lab. Each request was sent with curl, one at a time, no time or size limit, and each answer was written
to a file as received (BOM-less UTF-16LE; the fixture tree's attributes keep them `-text`). The company, its ledgers and its vouchers are
synthetic. Read-only: nothing was written to Tally.

- Item 697-S0A, 15 Oct 2026: the company object, groups, ledgers, trial balance, voucher types and vouchers. Evidence branch
  `lab/697-s0a-answers` at `1c59b0929a47` (its `SHA256.txt` lists each file here with the same hash). The requests were
  rendered by the audit read's own functions at the time and sent by curl, not by Bridge's transport.
- Item 697-S1, 21 Oct 2026: the census of the same window. Evidence branch `lab/697-s1-1605-s1-answers` at `ba4e119e9b5d`
  (`lab-evidence/697-s1-census/`; `MANIFEST.txt` lists the hashes).
- #2145, the census spanned to the voucher mark: `pilot-lab-audit-census-spanned.request.utf16le.xml` is the census
  request 697-S1 sent with ` AND $AlterID &gt; 0 AND $AlterID &lt;= 68` added to its formula and nothing else changed, the
  request the audit read sends since the census is always spanned. Its bytes are the run sheet's request
  `requests/p-audit_c2_spanned_0_68.request.utf16le.xml` on `lab/2145-spanned-census-captures` at `6a968f01`, and the test
  checks that the read renders them byte for byte.
- The same book in both sittings: the company list and the marks answers of 697-S1 are byte-identical to 697-S0A's
  (`0cb23618…` and `8a0c4268…`). Those two answers are already committed as `../pilot-lab/pilot-lab-company-extent.utf16le.xml`
  and `../gst-status/pilot-lab-gst-uncertain-august-marks.utf16le.xml` (the same bytes), so they are not repeated here.

Tests: `src/agent_audit_read_tests.rs` (`live_pilot_lab`) serves these answers in the order the reader asks for them and
asserts that the book is admitted with the counts the lab recorded, and that each part's request Bridge renders today is
byte for byte the request sent live.

Not established: Bridge's own transport against a live Tally for these parts (curl sent them); a window divided into
several parts (the replay uses one request for the whole period, as the lab sent it); the marks read inside the window at
the moment of the read (the replay serves the sitting's marks answer for all three marks reads); a book in FY 2025-26 (this
book starts 1 April 2026, outside the tax-audit rules' year); other releases, editions, a client book.

| Fixture | Bytes | SHA-256 | Source (item, lab file) | What it is |
| --- | ---: | --- | --- | --- |
| `pilot-lab-audit-company-object.request.utf16le.xml` | 896 | `2b02e1c703cc3a48789ee1eaf52231c1588c0e7d3d084aae9d19a55e83785905` | 697-S0A `s03-company-object.xml` | the request, UTF-16LE with a BOM |
| `pilot-lab-audit-company-object.utf16le.xml` | 4,208 | `5f5b72b7b5bfc42467fbea30b389581c5106ff08ee7b415731fbf372ebfad93d` | 697-S0A `answers/2.xml` | the company Object export: one COMPANY, BOOKSFROM 20260401, ISINTEGRATED Yes |
| `pilot-lab-audit-groups.request.utf16le.xml` | 1,066 | `20d147b548e716b876a013ec58593327a6b5b52ce20d68747c45d878dc98699f` | 697-S0A `s04-groups.xml` | the request, UTF-16LE with a BOM |
| `pilot-lab-audit-groups.utf16le.xml` | 55,954 | `0657b125f00a7f0b4cc31f249d9492337a6408ec3628f124a7da3b824e354e75` | 697-S0A `answers/3.xml` | 29 GROUP rows, each with a PARENT |
| `pilot-lab-audit-ledgers.request.utf16le.xml` | 1,278 | `4531f5136c694cab3cc2f0acc777500038f6a926f699e8d545cf9c6e99d9b12a` | 697-S0A `s05-ledgers.xml` | the request, UTF-16LE with a BOM |
| `pilot-lab-audit-ledgers.utf16le.xml` | 26,106 | `dce6eb363c6f2688d2783e1764992575bfb7e4c72e8dd3ec0ffc273c0dd07fed` | 697-S0A `answers/4.xml` | 18 LEDGER rows, each with a PARENT |
| `pilot-lab-audit-trial-balance.request.utf16le.xml` | 1,144 | `ce8f26e7715a287ec882bd34beaa28205dc411a7d41a90a568423bb63d75880c` | 697-S0A `s06-trial-balance.xml` | the request, UTF-16LE with a BOM |
| `pilot-lab-audit-trial-balance.utf16le.xml` | 24,498 | `a96bcfd665124d075fab60f159f93e9ce4fa32af5e8b026c3788f01d2c57eb1f` | 697-S0A `answers/5.xml` | the trial balance for 20260401-20270331: 18 LEDGER rows |
| `pilot-lab-audit-voucher-types.request.utf16le.xml` | 972 | `ee4c14f497960afb939de3139bae7bc575e86554045386c3e61e5d9ca76155f3` | 697-S0A `s07-voucher-types.xml` | the request, UTF-16LE with a BOM |
| `pilot-lab-audit-voucher-types.utf16le.xml` | 37,956 | `4011411447154d88f27c265bd667f22e6186df9f6ca51e74aef2953cf6324b27` | 697-S0A `answers/6.xml` | 28 VOUCHERTYPE rows |
| `pilot-lab-audit-vouchers.request.utf16le.xml` | 1,740 | `1680d37c06f2975b79ac66b1df18f7886dadc6658269d423ef7e2827e83d8a70` | 697-S0A `s08-vouchers.xml` | the request, UTF-16LE with a BOM |
| `pilot-lab-audit-vouchers.utf16le.xml` | 2,568,786 | `4085c787ad2737467891388fe674167af19f2ade786bbac539a425aae6ab6c7e` | 697-S0A `answers/7.xml` | every voucher of 20260401-20270331 in one request, entry wildcard: 41 VOUCHER rows, each with a GUID |
| `pilot-lab-audit-census.request.utf16le.xml` | 1,436 | `685b646740a4f7751d44089bd99d248f4334da714d16e946d35d47f485a501ce` | 697-S1 `c02-census-fy2026-27.xml` | the request, UTF-16LE with a BOM |
| `pilot-lab-audit-census.utf16le.xml` | 102,580 | `11c0077ba4a02cf6b076e56364b00928f20390db68257a65ffd25efc35e03f55` | 697-S1 `answers/4.xml` | the census of the same window: 41 VOUCHER rows with DATE, ALTERID 2..68 and GUID |
| `pilot-lab-audit-census-spanned.request.utf16le.xml` | 1,520 | `1125182a51310f09ea6d3c4a08dd4e0d411d13f1fc34f01b89be4ea86f729e4e` | #2145 run sheet `p-audit_c2_spanned_0_68` | the census request spanned to (0, 68], UTF-16LE with a BOM |
