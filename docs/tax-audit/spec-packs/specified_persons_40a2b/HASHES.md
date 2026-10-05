# Hashes and provenance: the `specified_persons_40a2b` spec pack

Every book here is invented, with round figures and plain names: no book is a Tally read of any
real assessee, and no figure comes from one.

## What these files establish, and what they do not

- The books are hand-written scenarios, each built to reach a branch or boundary of the reference
  engine's `specified_persons_40a2b` test (the README lists which). They are regression fixtures: a
  port that reproduces every golden agrees with the reference on these books, and nothing more.
- Each book also yields a `related_parties_cl23` golden. It is the input this test was given on that
  book, made in the same run; it adds nothing to that test's own contract, which is its own spec pack.
- `sp_company` and `sp_huf` give byte-identical goldens: the entity type does not appear in a dump,
  and both close the s.40(b) rule. They differ only in why it is closed.
- They prove nothing about reading Tally. The books are built directly in the edge-book shape, not
  read from a company, so they say nothing about how real Tally data reaches either test.
- Agreement on real books is established separately, by a local parity run on real reads that is
  never committed.

## How the goldens were produced

At the reference engine (a private repository), commit `4df1cc43`, under Python 3.13, with the
crate's `parity/edge_golden.py` extended by two runners. They are kept by the reference's maintainers
and are not part of this pack. Both pass the book's `related_parties` key (absent meaning `{}`)
through the reference's own configuration reader, as the reference's pack does. The
`related_parties_cl23` runner runs that test on the table, with its own invariant check. The
`specified_persons_40a2b` runner runs `related_parties_cl23` on the same table first, then this test
with the table, the rules for the book's `entity_type` (absent meaning `individual`) and that result,
and gives the canonical dump this test's own invariant check called with the same table, rules and
result, as the reference's pack calls it. Run from `src-tauri/crates/bridge-tax-audit`, once per
book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python EDGE_GOLDEN_WITH_RUNNERS ENGINE \
        BOOK.json OUTDIR

Each run writes `OUTDIR/edge.<book>.related_parties_cl23.json` and
`OUTDIR/edge.<book>.specified_persons_40a2b.json`. Running every book a second time, into a new
directory, reproduced every golden byte for byte.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `sp_company.json` | 2,947 | `268d6969076d3ee4fa6ad2631bc14de67d9befc74d4bae47fa7a723b4cda99ec` | `books/sp_company.json` |
| `sp_core.json` | 6,468 | `913fac3ead5f9ed8104f44889c5b262e816069d5e31828376a69d527c457b6a5` | `books/sp_core.json` |
| `sp_empty.json` | 2,332 | `067ea5f5239ce0147f001c50719799b355d6eb070312869199bcb2b23ae8f88f` | `books/sp_empty.json` |
| `sp_firm.json` | 6,288 | `7f934f5de43762e9738c7456eec3114dfb779db5ede1c65f9e91b95e97f33caf` | `books/sp_firm.json` |
| `sp_huf.json` | 2,909 | `1ff7b3b204afd8dabb65bc4c5f8229799b7a3fc09f6e98c65d8d1ab6fc976912` | `books/sp_huf.json` |
| `sp_keys.json` | 4,644 | `3ab1ba57319db2c133afda0c37bdaaf2c32f5f4a166b8f758dbe5078c5035d24` | `books/sp_keys.json` |
| `sp_labels.json` | 11,224 | `0f2e3e25fab1499e759ff6d07d2b258800437ae96ac0fa3b882182bce213e09e` | `books/sp_labels.json` |
| `sp_none.json` | 2,799 | `315902312f5c673744d3f5b1da3b6dbcceea81e6f90179d94d642a566b8e5a7d` | `books/sp_none.json` |
| `sp_shapes.json` | 4,083 | `b13c1c148639246e053ea27fd15f2506e634e5f4d77ba0811788af390fa0d46c` | `books/sp_shapes.json` |
| `sp_unknown_ledger.json` | 2,748 | `14bcf08007f2328160af51391e221f0b272ab53058f46d7de6a0dab181cb331d` | `books/sp_unknown_ledger.json` |
| `sp_zero.json` | 7,186 | `f070fdd6f87e6d44e123ec6eafb0174ae916bdcd1156c6a40f5430ee0b06ba84` | `books/sp_zero.json` |
| `edge.sp_company.related_parties_cl23.json` | 5,696 | `d15cdaf901c61bcb4ab914f21f18d0100054a365ae953d50e3c7dd1a135a1e72` | `goldens/edge.sp_company.related_parties_cl23.json` |
| `edge.sp_company.specified_persons_40a2b.json` | 4,263 | `aa86eeaf647856ca953fa8d334a2decc48e1a039904c8664b3dc90bc37780b0f` | `goldens/edge.sp_company.specified_persons_40a2b.json` |
| `edge.sp_core.related_parties_cl23.json` | 15,190 | `caee11dd43e738a64fcbc14d0bed21ec1e0738eb709fd6abf14b3a7fc87c9942` | `goldens/edge.sp_core.related_parties_cl23.json` |
| `edge.sp_core.specified_persons_40a2b.json` | 12,228 | `776515479f6635cf972aff98228f2a232f81a1a81b46d6b40a06656e54d687dc` | `goldens/edge.sp_core.specified_persons_40a2b.json` |
| `edge.sp_empty.related_parties_cl23.json` | 2,097 | `2e80c6fb224b50e57bd692137e84831a14c02d6ce57d0fc95f8a946370b92f2b` | `goldens/edge.sp_empty.related_parties_cl23.json` |
| `edge.sp_empty.specified_persons_40a2b.json` | 1,082 | `c848976101d6da5f54d08e166cce86fd188b59c3e4f8937f709eafd7a3c4a4d3` | `goldens/edge.sp_empty.specified_persons_40a2b.json` |
| `edge.sp_firm.related_parties_cl23.json` | 16,175 | `c453c7c878150cdf5c798a403abe3720d2b6fa3d484a1f9fae3b8dac1f2b5086` | `goldens/edge.sp_firm.related_parties_cl23.json` |
| `edge.sp_firm.specified_persons_40a2b.json` | 14,579 | `8b87d9ec9f4ed1f94c51a69d4d872067345fb0567c1bbc71ba5cf9d327b2a205` | `goldens/edge.sp_firm.specified_persons_40a2b.json` |
| `edge.sp_huf.related_parties_cl23.json` | 5,696 | `d15cdaf901c61bcb4ab914f21f18d0100054a365ae953d50e3c7dd1a135a1e72` | `goldens/edge.sp_huf.related_parties_cl23.json` |
| `edge.sp_huf.specified_persons_40a2b.json` | 4,263 | `aa86eeaf647856ca953fa8d334a2decc48e1a039904c8664b3dc90bc37780b0f` | `goldens/edge.sp_huf.specified_persons_40a2b.json` |
| `edge.sp_keys.related_parties_cl23.json` | 18,565 | `49e8a9df736a6fdd73c5e65862d91ba830fa1d12cf22093faa81b928ca4358fa` | `goldens/edge.sp_keys.related_parties_cl23.json` |
| `edge.sp_keys.specified_persons_40a2b.json` | 8,911 | `1ca9d472284458b0f28057d133dfccf4595438002135fce254dfa520a71ec353` | `goldens/edge.sp_keys.specified_persons_40a2b.json` |
| `edge.sp_labels.related_parties_cl23.json` | 50,466 | `2805e97c62afb0dd82caca68bbb9a27eaaad4a61fe5f2a522fbdaa6ab7d0b724` | `goldens/edge.sp_labels.related_parties_cl23.json` |
| `edge.sp_labels.specified_persons_40a2b.json` | 25,975 | `8fc589455942ff27ee7dd02b5ffa84e8fa9e4747a3fb7ea3c5201dd32546c4bc` | `goldens/edge.sp_labels.specified_persons_40a2b.json` |
| `edge.sp_none.related_parties_cl23.json` | 1,849 | `074cd0b565b1148f347392e2c162f5703d2decd5767576ca12062a83b10112a9` | `goldens/edge.sp_none.related_parties_cl23.json` |
| `edge.sp_none.specified_persons_40a2b.json` | 1,081 | `d339734fbfaaef747fc1ad5cd0eb83916e13178f8539e2970f5a594ac58eb3eb` | `goldens/edge.sp_none.specified_persons_40a2b.json` |
| `edge.sp_shapes.related_parties_cl23.json` | 7,522 | `ff56d477a45f66d4f89457fdbfe5ff9d29b10727ed0de9a029267094622a0342` | `goldens/edge.sp_shapes.related_parties_cl23.json` |
| `edge.sp_shapes.specified_persons_40a2b.json` | 2,812 | `10325e05f8b45724f30ef2a804b7ff25a378248a31f128ebf2e26962dc349615` | `goldens/edge.sp_shapes.specified_persons_40a2b.json` |
| `edge.sp_unknown_ledger.related_parties_cl23.json` | 5,926 | `e91042d45d9bb70a8967dbff0ed0a205a033e81304cb5a17a35a9624e6a8a03b` | `goldens/edge.sp_unknown_ledger.related_parties_cl23.json` |
| `edge.sp_unknown_ledger.specified_persons_40a2b.json` | 2,888 | `c229868338954d51cff60a29e191c8bbb59fe34a520c3129d4fc1d977e8412e3` | `goldens/edge.sp_unknown_ledger.specified_persons_40a2b.json` |
| `edge.sp_zero.related_parties_cl23.json` | 13,152 | `f21e8009c06712716a79a3101ddc4d44cfc7ff1922035db74f23401099808066` | `goldens/edge.sp_zero.related_parties_cl23.json` |
| `edge.sp_zero.specified_persons_40a2b.json` | 4,397 | `e9eaa4d8d1f36451da8223e7dcf251cabd441d42348451a21e4792db1125d5a8` | `goldens/edge.sp_zero.specified_persons_40a2b.json` |
