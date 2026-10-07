# Fixture provenance: `knock_off_candidates` (8 Oct 2026)

The thirteen edge books, their goldens and the synthetic golden come unchanged, byte for byte, from the
spec pack `docs/tax-audit/spec-packs/knock_off_candidates/`, whose `HASHES.md` lists the same bytes and
SHA-256 for each file. Every book there is invented, with synthetic ledger names, narrations and round
figures: no book is a Tally read of any real assessee, and no figure, name or narration comes from one.

## What these fixtures establish

- The books are hand-written scenarios, each built to reach a rule or boundary of the reference
  engine's `knock_off_candidates` test; the pack's README lists which (its section 12), and each book's
  `comment` says what it reaches. They are regression fixtures: the port reproduces every golden, so
  it agrees with the reference on these books, and nothing more.
- They prove nothing about reading Tally: the books are built directly in the edge-book shape, not
  read from a company. Agreement on real books is not measured here.
- The edge books are run unbound, as every edge book is: `ko_groups` names a party group no master
  holds, which the real pipeline refuses at binding first (the pack's README, section 2.3).
- `golden/synthetic.knock_off_candidates.json` is the test on the crate's synthetic read, with no party
  groups configured: the seven figures and no finding.

## How they were produced

At the reference engine (a private repository), commit `742f67fc`, under Python 3.13, with
`parity/edge_golden.py` extended by a runner that passes the book's `party_identity.party_groups` to
the test as a tuple (an absent table or key giving an empty one). Run from
`src-tauri/crates/bridge-tax-audit`, once per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The synthetic golden is the same test on the crate's synthetic engagement, through
`parity/python_golden.py`'s runner for it:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.knock_off_candidates.json \
        --test knock_off_candidates

Running every book a second time reproduced every golden byte for byte. The goldens are regenerated
only by the reference's maintainers.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `ko_empty.json` | 2,000 | `9d895c03045cdcc168d0d84b39c38edaf506f72273f32af56061b2fd2b19efc9` | `edge-books/ko_empty.json` |
| `ko_groups.json` | 5,008 | `7785a804be5657953c1b87f2afbef4c4e5a41165235a0893e311b03e71078234` | `edge-books/ko_groups.json` |
| `ko_groups_off.json` | 4,703 | `ac1b21d07bd67a2b7f59fa3e63528fd0829be4f72fdbdc2af5e631328df84414` | `edge-books/ko_groups_off.json` |
| `ko_nested.json` | 8,883 | `f62f1ad183cb353a1a84abb19b4769e68d64cc52f7d217d30e220cff213493de` | `edge-books/ko_nested.json` |
| `ko_quiet.json` | 5,641 | `ec7b1c8bf4931e0ddfec33f3fbf7ff5859e32cc3a7223c4b4cb5088bcbd26b36` | `edge-books/ko_quiet.json` |
| `ko_status.json` | 3,035 | `35c4ecc190c738a847b4443e000e3871e16e83793b7e7bf4cbd6e80679c18fb4` | `edge-books/ko_status.json` |
| `ko_t1.json` | 6,809 | `f4b9001fb4159c95535ca39af67d214b52a855d9168fdb329054b254bd5f0dd8` | `edge-books/ko_t1.json` |
| `ko_t2.json` | 9,559 | `210804c9d247b4ddfcf18cd1fb28ab1dfd3239ee9c6ee90c9ee79c6aaa641817` | `edge-books/ko_t2.json` |
| `ko_tags.json` | 4,678 | `5755e52427a9e3a6bf27801bf3d7ec39e92227da75585776d7c904128adce9f8` | `edge-books/ko_tags.json` |
| `ko_text.json` | 5,831 | `a7239ad59fb580a1c0514dbedb1651d09dcc556da1e8451d2555c7fbe814fb99` | `edge-books/ko_text.json` |
| `ko_vocab.json` | 16,711 | `24cb4a7e1ca220e4820005b9925c644a0c602f55e55011b94872900669b58852` | `edge-books/ko_vocab.json` |
| `ko_shared.json` | 2,846 | `9f37b1242c5fbbac624c835af6cd4d62102e8e87feacd4e13136397deb5f0c20` | `edge-books/ko_shared.json` |
| `ko_transfer.json` | 2,620 | `72ba98baa3d79606e97db46c48c66ee70ffdd0c1271fe3ded2bb361ec6ab1086` | `edge-books/ko_transfer.json` |
| `edge.ko_empty.knock_off_candidates.json` | 3,561 | `7467dc7d2a4ae999fbad76f01f2eff01968c0886ab600104ffb6a8fe59c98844` | `golden/edge.ko_empty.knock_off_candidates.json` |
| `edge.ko_groups.knock_off_candidates.json` | 13,275 | `cb74f3e433fab80c4fe9c273f7215fd930646903b80af62f218a005215897725` | `golden/edge.ko_groups.knock_off_candidates.json` |
| `edge.ko_groups_off.knock_off_candidates.json` | 8,404 | `65f951e162261a0dd4cef35efa5eb7695f7be114367d58c066dc0d45558b20c5` | `golden/edge.ko_groups_off.knock_off_candidates.json` |
| `edge.ko_nested.knock_off_candidates.json` | 19,099 | `66653a670cd3c14693d2affaeabfc98897b1d85f67791887734f561907dfdb68` | `golden/edge.ko_nested.knock_off_candidates.json` |
| `edge.ko_quiet.knock_off_candidates.json` | 3,561 | `7467dc7d2a4ae999fbad76f01f2eff01968c0886ab600104ffb6a8fe59c98844` | `golden/edge.ko_quiet.knock_off_candidates.json` |
| `edge.ko_status.knock_off_candidates.json` | 5,509 | `58486067fa7b74221cc9cf508592fea0e735e01852f671fd9eed2e2ab9baa782` | `golden/edge.ko_status.knock_off_candidates.json` |
| `edge.ko_t1.knock_off_candidates.json` | 8,101 | `f57b5ad79070f3598b4505bbf6c176ce94c02434dc2024a1f6f2405490f5661e` | `golden/edge.ko_t1.knock_off_candidates.json` |
| `edge.ko_t2.knock_off_candidates.json` | 17,027 | `f400f9e9acfcaaa5e83f5cdde1691b50df6fa347812c895e13f18b4d68c61872` | `golden/edge.ko_t2.knock_off_candidates.json` |
| `edge.ko_tags.knock_off_candidates.json` | 14,033 | `396e33a426c1cce79c6cdb365f513c0b7eeddac284e1792e7724367970076382` | `golden/edge.ko_tags.knock_off_candidates.json` |
| `edge.ko_text.knock_off_candidates.json` | 15,052 | `11e2d20ee1bea419be22b74eaab200a7443c4ca545f26feec6e24e1a8e9c9bb8` | `golden/edge.ko_text.knock_off_candidates.json` |
| `edge.ko_vocab.knock_off_candidates.json` | 36,477 | `4c4e74613bfb9e778a4a45afcccb33acc1f686b9a8d2dcb66212c2062e9bc73d` | `golden/edge.ko_vocab.knock_off_candidates.json` |
| `edge.ko_shared.knock_off_candidates.json` | 6,922 | `6668237665bf24a8df569706d51ceab7fdd7bb8f0279beb1960852bcbbcbcd41` | `golden/edge.ko_shared.knock_off_candidates.json` |
| `edge.ko_transfer.knock_off_candidates.json` | 7,331 | `28f12db8f7cdd5527ad9b7bc53aed8a455bbe7eb80f84691fd225fb1bf9cd2ec` | `golden/edge.ko_transfer.knock_off_candidates.json` |
| `synthetic.knock_off_candidates.json` | 4,117 | `bab0b914b24aea4014b2aa125f95ad2ce7d975eb6c2b15887bb344eb19051276` | `golden/synthetic.knock_off_candidates.json` |
