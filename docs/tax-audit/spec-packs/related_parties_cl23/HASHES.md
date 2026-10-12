# Hashes and provenance: the `related_parties_cl23` spec pack

Every book here is invented, with round figures and plain names: no book is a Tally read of any
real assessee, and no figure comes from one.

## What these files establish, and what they do not

- The books are hand-written scenarios, each built to reach a branch or boundary of the reference
  engine's `related_parties_cl23` test (the README lists which). They are regression fixtures: a
  port that reproduces every golden agrees with the reference on these books, and nothing more.
- They prove nothing about reading Tally. The books are built directly in the edge-book shape, not
  read from a company, so they say nothing about how real Tally data reaches the test.
- Agreement on real books is established separately, by a local parity run on real reads that is
  never committed.

## How the goldens were produced

At the reference engine (a private repository), commit `ee17d80f`, under Python 3.13, with the
crate's own `parity/edge_golden.py`, which runs this test: it passes the book's `related_parties`
key (absent meaning `{}`) through the reference's own configuration reader to the test, and the
test's own invariant check to the canonical dump, as the reference's pack does. Run from
`src-tauri/crates/bridge-tax-audit`, once per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        BOOK.json OUTDIR

Each run writes `OUTDIR/edge.<book>.related_parties_cl23.json`. Running every book a second time
reproduced every golden byte for byte. The golden of each of the eleven books, and the synthetic golden, was
regenerated at the reference's commit `ee17d80f` and is byte-identical to the table below.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `rp_core.json` | 6,596 | `5d0a9f6388e2538abb6310f253b32a30c8e8dd9ffd6cfd54db3c5e019659e944` | `books/rp_core.json` |
| `rp_many.json` | 5,176 | `7e5a7761b4af49651fb48f057bdf7e181b12f5ac305f66da8868e463f3f42c2e` | `books/rp_many.json` |
| `rp_none.json` | 2,586 | `1819d812da628747732b38ef1b3189626b0ae41e42a7f7c7f4761ab2b83ca00a` | `books/rp_none.json` |
| `rp_partial.json` | 3,460 | `732f1a557d4a2ce0981bdd866a740276bfdd926e66e687c3ded2f97c9b840601` | `books/rp_partial.json` |
| `rp_payable_break.json` | 4,696 | `03742c56cc8cdf8ee4a64a167555097b85cf03d8d60ca65be8d42967b3634483` | `books/rp_payable_break.json` |
| `rp_quiet.json` | 4,038 | `121a604e2878c940a5114ef12f0a280a8dbec157a5711aae46f8f13676a96941` | `books/rp_quiet.json` |
| `rp_shared_guid.json` | 3,086 | `f14ffdbd0af62ce9c328e33583bef8b0e7a16b87bead447b322eb358d77cbc25` | `books/rp_shared_guid.json` |
| `rp_shared_guid_places.json` | 3,747 | `15ba06a25bdfecfc1b744120671bab9118daa0f4bf2b72bb679c9826dda66496` | `books/rp_shared_guid_places.json` |
| `rp_sum_check.json` | 4,200 | `000ed3dd5f43daa37790da672e0bd831268f307fe9659296fd2f82431cab5aed` | `books/rp_sum_check.json` |
| `rp_unknown_ledger.json` | 2,962 | `ef09555a49c0150d2fb62cfb9a6e82209e24a23680a52e004facbb3c0f86f055` | `books/rp_unknown_ledger.json` |
| `rp_walk.json` | 6,911 | `6590a01f5f802ce4ed9f27d7fbdd4fa27df8d2b410e23c93f6852ba84139fa87` | `books/rp_walk.json` |
| `edge.rp_core.related_parties_cl23.json` | 14,113 | `3695d2e7e0038f0041910e55ac3d0222b363c82cc62ecc694fd1b2473e0c51ed` | `goldens/edge.rp_core.related_parties_cl23.json` |
| `edge.rp_many.related_parties_cl23.json` | 23,112 | `0fadbda01c4d096fcfa458432b86d4174e657d76a26c606d693cf7a1fe945aba` | `goldens/edge.rp_many.related_parties_cl23.json` |
| `edge.rp_none.related_parties_cl23.json` | 1,849 | `074cd0b565b1148f347392e2c162f5703d2decd5767576ca12062a83b10112a9` | `goldens/edge.rp_none.related_parties_cl23.json` |
| `edge.rp_partial.related_parties_cl23.json` | 9,723 | `16ab6b5199abe79da29317bb62561573b8684db97528555b3992595c6da05fac` | `goldens/edge.rp_partial.related_parties_cl23.json` |
| `edge.rp_payable_break.related_parties_cl23.json` | 16,103 | `267fdf8c796f995a8d627ce97b35709b3669f0896946aea7937de59d56911472` | `goldens/edge.rp_payable_break.related_parties_cl23.json` |
| `edge.rp_quiet.related_parties_cl23.json` | 6,960 | `dabe2d1320078ae4b7b0d7ef81c0c6b11cd4dbdcdc93b8ebf244b949bc7d31c8` | `goldens/edge.rp_quiet.related_parties_cl23.json` |
| `edge.rp_shared_guid.related_parties_cl23.json` | 5,459 | `b30492df8e14891740bc878f815dc5add392cc46b5bb6302d7824ae4770ed26d` | `goldens/edge.rp_shared_guid.related_parties_cl23.json` |
| `edge.rp_shared_guid_places.related_parties_cl23.json` | 6,380 | `3049528214ca7b454a5c9c3544d2afda4f14c6b6b289e44586128ca661a1c76b` | `goldens/edge.rp_shared_guid_places.related_parties_cl23.json` |
| `edge.rp_sum_check.related_parties_cl23.json` | 7,332 | `a0a479e7574b2459d7a065bebdc3d44488b0a34668e3da8f853066c29c1f5efb` | `goldens/edge.rp_sum_check.related_parties_cl23.json` |
| `edge.rp_unknown_ledger.related_parties_cl23.json` | 9,031 | `6bae2276338b8adb375947a807e200d864c59cbd67a33bbfddf87236aa082ec5` | `goldens/edge.rp_unknown_ledger.related_parties_cl23.json` |
| `edge.rp_walk.related_parties_cl23.json` | 13,349 | `c298493e495b6eacaf2202cd10377a0ccb71e825fd292cc9ab1f167b1549880b` | `goldens/edge.rp_walk.related_parties_cl23.json` |
| `synthetic.related_parties_cl23.json` | 2,405 | `6bf0989b8f1f9a9069f2f243a192085f65148d749994cb3280ee007be935567c` | `goldens/synthetic.related_parties_cl23.json` |
