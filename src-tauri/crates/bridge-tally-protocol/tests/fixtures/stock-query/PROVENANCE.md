# `stock-query/` — provenance

Tally's own `Stock Query` report, asked for one stock item (#2018). Every file is a live capture, never hand-written.

- **Host / gateway:** TallyPrime 7.2, licensed, `education_mode=false`, a lab instance, one request at a time, reads only.
- **Book:** `BRIDGE SHAPE LAB`, a synthetic lab company. Its marks were read before and after each sitting and were unchanged.
- **Sittings (28 October 2026):**
  - `r-3` and `r-4`: `lab/2018-stock-query-with-item-answers` at `263cc72c9c12`. `r-3` is the bare request with no item; `r-4` adds `STOCKITEMNAME`.
  - `r-01` to `r-15`: `lab/2018-stock-query-items-periods-answers` at `a5b369bfe3ea` (sheet `6b3eba0d`). Five of the book's eleven items at the year end (`r-01`, `r-02`, `r-03`, `r-09`, `r-11`), two items at the opening day (`r-12`, `r-14`), and an unknown name (`r-15`). The sitting's other answers stay on its branch.
- **Encoding:** answers are BOM-less UTF-16LE, exactly as received; requests are the exact bytes sent, UTF-16LE with a BOM. `.gitattributes` marks this tree `-text`.
- **Requests:** each is the built-in report envelope (`render_report_envelope`, `native_statement_reports.rs`) with the report `Stock Query` and, for every request but `r-3`, one more static variable `STOCKITEMNAME`. `every_captured_request_is_the_builders_bytes` (`src/native_stock_query_tests.rs`) asserts each equal to `render_stock_query_request`'s output.
- **What the answers show** (protocol reference, measurements part, §12a.23): any last-purchase and last-sale groups, then one godown, batch and quantity triple per godown and batch holding stock at the period end. At the year end each item's triples summed to its closing on Tally's own Stock Summary and matched Tally's screen. The bare request and the unknown name answer the same 46-byte empty envelope. Despite its name, `r-11`'s item holds stock (`-50.000 Kgs`): no fixture is of an item with no stock.

| file | bytes | sha256 | content |
|---|---|---|---|
| `r-01-stock-query-carton-box-small.answer.utf16le.xml` | 312 | `53fe972f6a9abb14465495afbaf55af9cb934fe9b7fb309fa926925ba7ae4c9c` | answer |
| `r-01-stock-query-carton-box-small.request.utf16le.xml` | 848 | `dc2b3afe5c542e1305868148facd0ca453796abf2cbb5c78ae1346b79e4be812` | request |
| `r-02-stock-query-caustic-soda-flakes.answer.utf16le.xml` | 1094 | `ece7ccb544e832c89e52cde0b0425ba7941ff1a8e005511c546756668b1d674f` | answer |
| `r-02-stock-query-caustic-soda-flakes.request.utf16le.xml` | 854 | `a3ede12af36a1daca85527f43082919e07db9ee0511f819faf03cfaa747b22ae` | request |
| `r-3-stock-query.answer.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer |
| `r-3-stock-query.request.utf16le.xml` | 754 | `de7408067d9140f3bf7aa7d516635441f4ec5f0efb0aa450b3dd838ce0225696` | request |
| `r-03-stock-query-cleaning-kit-a.answer.utf16le.xml` | 1086 | `b4d01f265a2af3f8b4f9ebc2a876ebfcfd2773fa8112388d6dea85087310ee24` | answer |
| `r-03-stock-query-cleaning-kit-a.request.utf16le.xml` | 844 | `8f948a388098391ac2012e38fcc2ee49b45f73ebe41d8c479d8baba08b5216b9` | request |
| `r-4-stock-query-stockitemname.answer.utf16le.xml` | 1354 | `c23c6d414d47f1d87b779a4a0afca47e4a0712ad2ce5b129a0e44ba41c66f213` | answer |
| `r-4-stock-query-stockitemname.request.utf16le.xml` | 860 | `fff4e42cb65b14100e0c271579a8e35b05d65f0a96e987b92407318f0261119d` | request |
| `r-09-stock-query-soda-ash-light.answer.utf16le.xml` | 3152 | `592f19e8057e6e759b92f8423cfbe568beee74ad676b22a25f7b4a3184cb4dbe` | answer |
| `r-09-stock-query-soda-ash-light.request.utf16le.xml` | 844 | `75f8b60cd19b79268d680288ee1d6060556363068e030de25fbf375580e74798` | request |
| `r-11-stock-query-zero-stock-item.answer.utf16le.xml` | 848 | `83acc28947d35ddca4c4b49973b466b9d686882aeea3857fe2afb11f93f12d30` | answer |
| `r-11-stock-query-zero-stock-item.request.utf16le.xml` | 846 | `0ed3e1e9197132c8d56561aa71f11df2595dc8b91a4b4460f9ca2dfcb079ecd2` | request |
| `r-12-stock-query-hcl-to-20250401.answer.utf16le.xml` | 320 | `6e8d7e5e2418f30dc2d2e89f3f4435dab784e9e17e510bdeb2ffe5230197fe55` | answer |
| `r-12-stock-query-hcl-to-20250401.request.utf16le.xml` | 860 | `5096cf0d56c9c931f3134feb94fd9a0c6247a904bc421319e4890b179add6797` | request |
| `r-14-stock-query-cleaning-kit-a-to-20250401.answer.utf16le.xml` | 310 | `b78d85c0160f1c6426ed78ead117121ea245b1acb2fce81bdd6e6777a0c548fb` | answer |
| `r-14-stock-query-cleaning-kit-a-to-20250401.request.utf16le.xml` | 844 | `ad79239d98ef3692e8d0605465aada99a811d2ce737b09920f22f7a6933a8256` | request |
| `r-15-stock-query-unknown-item.answer.utf16le.xml` | 46 | `8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3` | answer |
| `r-15-stock-query-unknown-item.request.utf16le.xml` | 850 | `98c06d132ddf2347690a532c4b5760134db202531c5b5b9d6dbb5a0be7e04263` | request |
