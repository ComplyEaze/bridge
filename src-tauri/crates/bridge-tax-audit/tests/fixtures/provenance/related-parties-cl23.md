# Fixture provenance: `related_parties_cl23` (6 Oct 2026)

The ten edge books and their goldens come unchanged, byte for byte, from the spec pack
`docs/tax-audit/spec-packs/related_parties_cl23/` (#1279), whose `HASHES.md` lists the same bytes and
SHA-256 for each file. Every book there is invented, with round figures and plain names: no book is a
Tally read of any real assessee, and no figure comes from one.

## What these fixtures establish

- The books are hand-written scenarios, each built to reach a branch or boundary of the reference
  engine's `related_parties_cl23` test; the pack's README lists which (its section 11), and each book's
  `comment` says what it reaches. They are regression fixtures: the port reproduces every golden, so
  it agrees with the reference on these books, and nothing more.
- They prove nothing about reading Tally: the books are built directly in the edge-book shape, not
  read from a company. Agreement on real books is established separately, by a local parity run on
  real reads that is never committed.
- The edge books are run unbound, as every edge book is: `rp_unknown_ledger` shows the test on a name
  that has no master, which the real pipeline refuses at binding first (the pack's README, §2.5).

## How they were produced

At the reference engine (a private repository), commit `4df1cc43`, under Python 3.13, with
`parity/edge_golden.py` extended by a runner for this test, which passes a book's `related_parties`
key (absent meaning `{}`) through the reference's own configuration reader to the test, and the
test's own invariant check to the canonical dump. Run from `src-tauri/crates/bridge-tax-audit`, once
per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

Running every book a second time reproduced every golden byte for byte. The goldens are regenerated
only by the reference's maintainers.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `rp_core.json` | 6,596 | `5d0a9f6388e2538abb6310f253b32a30c8e8dd9ffd6cfd54db3c5e019659e944` | `edge-books/rp_core.json` |
| `rp_many.json` | 5,176 | `7e5a7761b4af49651fb48f057bdf7e181b12f5ac305f66da8868e463f3f42c2e` | `edge-books/rp_many.json` |
| `rp_none.json` | 2,586 | `1819d812da628747732b38ef1b3189626b0ae41e42a7f7c7f4761ab2b83ca00a` | `edge-books/rp_none.json` |
| `rp_partial.json` | 3,460 | `732f1a557d4a2ce0981bdd866a740276bfdd926e66e687c3ded2f97c9b840601` | `edge-books/rp_partial.json` |
| `rp_payable_break.json` | 4,696 | `03742c56cc8cdf8ee4a64a167555097b85cf03d8d60ca65be8d42967b3634483` | `edge-books/rp_payable_break.json` |
| `rp_quiet.json` | 4,038 | `121a604e2878c940a5114ef12f0a280a8dbec157a5711aae46f8f13676a96941` | `edge-books/rp_quiet.json` |
| `rp_shared_guid.json` | 3,070 | `434ccddf8cf07b9fa8a34b052bfb1ed603880b0472334afeb0227ac07b12df58` | `edge-books/rp_shared_guid.json` |
| `rp_sum_check.json` | 4,200 | `000ed3dd5f43daa37790da672e0bd831268f307fe9659296fd2f82431cab5aed` | `edge-books/rp_sum_check.json` |
| `rp_unknown_ledger.json` | 2,962 | `ef09555a49c0150d2fb62cfb9a6e82209e24a23680a52e004facbb3c0f86f055` | `edge-books/rp_unknown_ledger.json` |
| `rp_walk.json` | 6,911 | `6590a01f5f802ce4ed9f27d7fbdd4fa27df8d2b410e23c93f6852ba84139fa87` | `edge-books/rp_walk.json` |
| `edge.rp_core.related_parties_cl23.json` | 14,113 | `3695d2e7e0038f0041910e55ac3d0222b363c82cc62ecc694fd1b2473e0c51ed` | `golden/edge.rp_core.related_parties_cl23.json` |
| `edge.rp_many.related_parties_cl23.json` | 23,112 | `0fadbda01c4d096fcfa458432b86d4174e657d76a26c606d693cf7a1fe945aba` | `golden/edge.rp_many.related_parties_cl23.json` |
| `edge.rp_none.related_parties_cl23.json` | 1,849 | `074cd0b565b1148f347392e2c162f5703d2decd5767576ca12062a83b10112a9` | `golden/edge.rp_none.related_parties_cl23.json` |
| `edge.rp_partial.related_parties_cl23.json` | 9,723 | `16ab6b5199abe79da29317bb62561573b8684db97528555b3992595c6da05fac` | `golden/edge.rp_partial.related_parties_cl23.json` |
| `edge.rp_payable_break.related_parties_cl23.json` | 16,103 | `267fdf8c796f995a8d627ce97b35709b3669f0896946aea7937de59d56911472` | `golden/edge.rp_payable_break.related_parties_cl23.json` |
| `edge.rp_quiet.related_parties_cl23.json` | 6,960 | `dabe2d1320078ae4b7b0d7ef81c0c6b11cd4dbdcdc93b8ebf244b949bc7d31c8` | `golden/edge.rp_quiet.related_parties_cl23.json` |
| `edge.rp_shared_guid.related_parties_cl23.json` | 5,341 | `63f6c9cf426edfddf1d5bafd6d6061933d51cd5b40b82c42627827fbd5f4a6f4` | `golden/edge.rp_shared_guid.related_parties_cl23.json` |
| `edge.rp_sum_check.related_parties_cl23.json` | 7,332 | `a0a479e7574b2459d7a065bebdc3d44488b0a34668e3da8f853066c29c1f5efb` | `golden/edge.rp_sum_check.related_parties_cl23.json` |
| `edge.rp_unknown_ledger.related_parties_cl23.json` | 9,031 | `6bae2276338b8adb375947a807e200d864c59cbd67a33bbfddf87236aa082ec5` | `golden/edge.rp_unknown_ledger.related_parties_cl23.json` |
| `edge.rp_walk.related_parties_cl23.json` | 13,349 | `c298493e495b6eacaf2202cd10377a0ccb71e825fd292cc9ab1f167b1549880b` | `golden/edge.rp_walk.related_parties_cl23.json` |
