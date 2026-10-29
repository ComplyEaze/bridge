# `stock-query/` — provenance

Tally's own `Stock Query` report, asked for one stock item (#2018). Every file is a live capture, never hand-written.

- **Host / gateway:** TallyPrime 7.2, licensed, `education_mode=false`, a lab instance, one request at a time, reads only.
- **Book:** `BRIDGE SHAPE LAB`, a synthetic lab company. Its marks were read before and after each sitting. In the reads-only sittings they were unchanged; in the two sittings that created items (below) the book was backed up first and restored after, and its marks read back as they were before.
- **Sittings (28 October 2026):**
  - `r-3` and `r-4`: `lab/2018-stock-query-with-item-answers` at `263cc72c9c12`. `r-3` is the bare request with no item; `r-4` adds `STOCKITEMNAME`.
  - `r-01` to `r-15`: `lab/2018-stock-query-items-periods-answers` at `a5b369bfe3ea` (sheet `6b3eba0d`). Five of the book's eleven items at the year end (`r-01`, `r-02`, `r-03`, `r-09`, `r-11`), two items at the opening day (`r-12`, `r-14`), and an unknown name (`r-15`). The sitting's other answers stay on its branch.
  - `w-01` to `w-10` and `a-01` to `a-03`: `lab/2018-stock-query-names-units-dates-answers` at `86ea5211`. Seven items were created by hand: six with names holding `&`, `(`, `)`, `'`, `"`, `%`, `/`, `,` and `-`, and `Alt Unit Item`, with no alternate unit set. `w-02` to `w-10` are Stock Query for each (`f1` sends the name raw, `f2` escaped; only `'` and `"` have both), `w-01` is the stock-item collection with the new items, and `a-01` to `a-03` are the collection at 1 April 2025, 1 May 2025 and 31 March 2026.
  - `alt-a-01`, `alt-a-04` and `alt-a-05`: `lab/2018-alternate-unit-row-answers` at `88616807`. Two items were created with an alternate unit. `alt-a-04` and `alt-a-05` are Stock Query for each, and `alt-a-01` is the stock-item collection for the year; its request is byte-equal to `../stock_items_fy_request.utf16le.xml`, so it is not repeated here.
  - Several answers are byte-equal under two names, each name true to its own request: `w-04` and `w-05`, `w-06` and `w-07`, `w-08` and `alt-a-04`, `a-02` and `a-03`; `a-03`'s request is `w-01`'s.
- **Encoding:** answers are BOM-less UTF-16LE, exactly as received; requests are the exact bytes sent, UTF-16LE with a BOM. `.gitattributes` marks this tree `-text`.
- **Requests:** each is the built-in report envelope (`render_report_envelope`, `native_statement_reports.rs`) with the report `Stock Query` and, for every request but `r-3`, one more static variable `STOCKITEMNAME`. `every_captured_request_is_the_builders_bytes` (`src/native_stock_query_tests.rs`) asserts each equal to `render_stock_query_request`'s output, except the raw `w-04` and `w-06`, which the builder never sends (`a_quote_raw_or_escaped_selects_the_same_item`). The `a-` and `w-01` requests are the stock-item collection request instead (`AuditStockItemsV1`), checked by `the_collection_requests_are_the_stock_reads_bytes`.
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
| `a-01-items-to-20250401.answer.utf16le.xml` | 17466 | `bbf93d6b9c38e8d22a263df1e5c65b149f726d298f42972be9bcb03b7d18f3a8` | answer, stock-item collection |
| `a-01-items-to-20250401.request.utf16le.xml` | 1258 | `9a08f0c2d9ef30bbd1768eea8891994c3cf71b2d47ad728da6b9af6c01774c03` | request, stock-item collection |
| `a-02-items-to-20250501.answer.utf16le.xml` | 17480 | `15a361aa36050305dc12f10d8550cc902121a1ec026b0105a084f4ffdc3f2b2a` | answer, stock-item collection |
| `a-02-items-to-20250501.request.utf16le.xml` | 1258 | `3fb2bdcb009f7840bbc6da186e229507465db2e5863422de2edaec7400a4515c` | request, stock-item collection |
| `a-03-items-to-20260331.answer.utf16le.xml` | 17480 | `15a361aa36050305dc12f10d8550cc902121a1ec026b0105a084f4ffdc3f2b2a` | answer, stock-item collection |
| `a-03-items-to-20260331.request.utf16le.xml` | 1258 | `dacd47b0b96772f220b97abf3714650c760a98f4e54197b681ba35cd69c9edab` | request, stock-item collection |
| `alt-a-01-items-fy-s4.answer.utf16le.xml` | 19820 | `4e33bf81b19643b2494a8f52bae592c79d63c4bd0021363276c63b972308ab86` | answer, stock-item collection |
| `alt-a-04-stock-query-alt-unit-probe-a.answer.utf16le.xml` | 310 | `7c5c66de1c6cf7903c37ebe39158ad8abb5b13f46eefc5d7995370c852cbc03e` | answer |
| `alt-a-04-stock-query-alt-unit-probe-a.request.utf16le.xml` | 848 | `9281993e15efd9726d2e8edc8c95607a9b7de093b7e5af04e08860e3999ec2ca` | request |
| `alt-a-05-stock-query-alt-unit-probe-b.answer.utf16le.xml` | 310 | `8754cc09a4f0662596c9e4b8d6a30a80953d8f4c3ae57ca9dcb79fbfb6bc6553` | answer |
| `alt-a-05-stock-query-alt-unit-probe-b.request.utf16le.xml` | 848 | `e5d0ca8857f85289f4c7dc400306dcaff7764449b865f63b1699190481b6b433` | request |
| `w-01-items-to-20260331.answer.utf16le.xml` | 27118 | `04d569f89f8e4bb5091be1b6d573c140e14a84bae82566b170c740b5fbab096e` | answer, stock-item collection |
| `w-01-items-to-20260331.request.utf16le.xml` | 1258 | `dacd47b0b96772f220b97abf3714650c760a98f4e54197b681ba35cd69c9edab` | request, stock-item collection |
| `w-02-stock-query-kit-co-item-f1.answer.utf16le.xml` | 310 | `588130f97cc914984d2538ec9d827999b1f7c034a543dbc3a597703f133c4845` | answer |
| `w-02-stock-query-kit-co-item-f1.request.utf16le.xml` | 850 | `515408dde7c062d0013528bac255e713ada4d30ded55a029f0921a7c3fd5c5b1` | request |
| `w-03-stock-query-gauge-test-50l-f1.answer.utf16le.xml` | 310 | `6c2928ac43ff2c2fcc968b34b7dcf3de246b8daaacafee3bfcda74a82e42a040` | answer |
| `w-03-stock-query-gauge-test-50l-f1.request.utf16le.xml` | 848 | `3a2930f8895475c787051c8960e718d2857d3a5d82e6ab1180cea899082f9ae9` | request |
| `w-04-stock-query-owner-s-item-f1.answer.utf16le.xml` | 310 | `cf3040a66e2f409e454cadbdf9c96e703ca67a9aed51af8af21dbaf2536ece48` | answer |
| `w-04-stock-query-owner-s-item-f1.request.utf16le.xml` | 840 | `6a232404aba6cf50553bca4721afdb04b62911a9454aaf55b5b71ce96f3a2971` | request |
| `w-05-stock-query-owner-s-item-f2.answer.utf16le.xml` | 310 | `cf3040a66e2f409e454cadbdf9c96e703ca67a9aed51af8af21dbaf2536ece48` | answer |
| `w-05-stock-query-owner-s-item-f2.request.utf16le.xml` | 850 | `bf32a94418a3a7ec040e43a9f6544349b25137d62be1fa69230dc854be4291db` | request |
| `w-06-stock-query-quote-q-item-f1.answer.utf16le.xml` | 310 | `2fe14f37bbb070084095d70b91126b6d09ea0acddc12befb5a21d6a9aa539d99` | answer |
| `w-06-stock-query-quote-q-item-f1.request.utf16le.xml` | 844 | `b4df6aec95e599be65e68f92af5b56bf8bac2a5fe40f4f4b8d926c47186140a7` | request |
| `w-07-stock-query-quote-q-item-f2.answer.utf16le.xml` | 310 | `2fe14f37bbb070084095d70b91126b6d09ea0acddc12befb5a21d6a9aa539d99` | answer |
| `w-07-stock-query-quote-q-item-f2.request.utf16le.xml` | 864 | `b7b3d0a8e2fda73ebd8718a2e0bdc6649fee6cb090cae47de85a67ebd9d6d44b` | request |
| `w-08-stock-query-alt-unit-item-f1.answer.utf16le.xml` | 310 | `7c5c66de1c6cf7903c37ebe39158ad8abb5b13f46eefc5d7995370c852cbc03e` | answer |
| `w-08-stock-query-alt-unit-item-f1.request.utf16le.xml` | 842 | `d6e282253d355204ba1004b2a2377aad7986764ee56f262a080fcf9d6a6b1cd9` | request |
| `w-09-stock-query-mix-5-set-large-f1.answer.utf16le.xml` | 310 | `93bd0915e902c67e1d18b1a55d698ff133af2612f585e27557607162b9a5180f` | answer |
| `w-09-stock-query-mix-5-set-large-f1.request.utf16le.xml` | 854 | `102c7cbf207b7d5cd3a3355249a53b99b50d2460b1c7583eae6763e541b83c35` | request |
| `w-10-stock-query-dash-item-7-f1.answer.utf16le.xml` | 310 | `05665509339b049ad5bfa5eea2349196076d6b38463d90f2b60ae573c1f007a5` | answer |
| `w-10-stock-query-dash-item-7-f1.request.utf16le.xml` | 838 | `dfd7b7898859a961fb77753cabe5ee6110c4bf7da4699f1dd46113d3a7b5fa64` | request |
