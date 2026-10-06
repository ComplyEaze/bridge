# Hashes and provenance: the `knock_off_candidates` spec pack

Every book here is invented, with synthetic names, round figures and a few small odd amounts: no book
is a Tally read of any real assessee, and no figure, name or narration comes from one.

## What these files establish, and what they do not

- The books are hand-written scenarios, each built to reach a branch or boundary of the reference
  engine's `knock_off_candidates` test (the README lists which, and each book's `comment` says so
  voucher by voucher). They are regression fixtures: a port that reproduces every golden agrees with
  the reference on these books, and nothing more.
- `ko_empty` and `ko_quiet` give byte-identical goldens: with nothing listed the dump does not depend
  on the book. They differ in why nothing is listed (no voucher at all, against eleven near misses).
- `ko_groups` and `ko_groups_off` are one book, with and without the `party_identity` table; they
  give every ledger the same GUID, so the figures they share have the same ids.
- They prove nothing about reading Tally. The books are built directly in the edge-book shape, not
  read from a company, so they say nothing about how real narrations, group chains or GUIDs reach the
  test, nor about how often any pattern occurs in real books (recall is not measured).
- Agreement on real books is established separately, by a local parity run on real reads that is
  never committed.

## Which rules the goldens pin

Each rule below was changed alone, in memory, in the reference test at commit `742f67fc`, and every
book was run again; the books named are those whose golden then changed (the unchanged reference,
run the same way first, reproduced every golden). A port that gets one of these rules wrong fails at
least one golden.

| Rule changed | Goldens that change |
| --- | --- |
| a name shared by more than one master bound to the last party master (the reference before `66e842e7`) | `ko_shared`, `ko_t2` |
| a name shared by more than one master bound to the first party master | `ko_shared`, `ko_t2` |
| sharing counted over party masters only (a party and an expense sharing a name stays bound) | `ko_t2` |
| every voucher sums every bank and cash line (a contra counts both sides) | `ko_t2`, `ko_transfer` |
| a pure money transfer counts its larger side once only when it is a non-Contra voucher | `ko_t2` |
| a shared name dropped from the index instead of being bound to no one | `ko_shared` |
| only a Contra counts its larger side once (a transfer booked as another type counts both) | `ko_transfer` |
| any one of 54 never-name-like words removed from the list | `ko_vocab` (and others for some words) |
| the word `a` removed | none (a one-letter word fails the length test anyway) |
| any one of the nine separators removed | `ko_vocab` (and others for `-`, `/`, `,`) |
| a full stop, `&`, `#`, `_` or an apostrophe made a separator | `ko_vocab` (and `ko_text` for the apostrophe) |
| the token after a span compared with the part of its first token | `ko_vocab` |
| the token before a span compared with the part of its last token | `ko_nested` |
| `casefold()` replaced by `lower()`, or by ASCII-only lower-casing | `ko_vocab` |
| no decomposition (NFD) before case-folding | `ko_t2` |
| a mark after a letter `a` to `z` kept in its token | `ko_t2`, `ko_tags`, `ko_vocab` |
| tokens taken as Python's `\w` runs, after decomposing and case-folding | `ko_vocab` |
| zero lines kept in `<ledgers>` | `ko_t2`, `ko_vocab` |
| parts of a name that is also plain kept in `<parts>` | `ko_vocab` |
| the part shown only the one holding the name's first token | `ko_nested`, `ko_vocab` |
| the parts shown joined by ` \| ` inside one quote | `ko_nested` |
| names sorted after NFC normalisation | `ko_vocab` |
| zero lines counted when deciding whether a party is on the voucher (T2) | `ko_t2` |
| zero lines counted for T1 | `ko_quiet`, `ko_t1` |
| nested spans kept | `ko_nested`, `ko_shared` |
| the shown narration allowed 81 characters | `ko_text` |
| name-like words needing three letters | `ko_nested` |

Changed the same way, these rules change no golden (README section 10): NFKD in place of NFD;
case-folding before decomposing; a mark that starts a token kept; U+200C, U+200D and U+00AD made
to end a token; marks, or digits, not counted as token characters; a token in another alphabet
counted as name-like.

## How the goldens were produced

At the reference engine (a private repository), commit `742f67fc`, under Python 3.13, with the
crate's `parity/edge_golden.py`, which is in this repository under its Apache-2.0 licence and has no
runner for this test. The goldens were made with that file extended by one runner, which the
reference's maintainers hold outside this repository; it is not part of this pack. The runner passes
the book's `party_identity.party_groups` to the test as a tuple, exactly as the reference's pack
passes `[party_identity].party_groups` (an absent table or key giving an empty tuple), and gives the
canonical dump the test module itself; the module has no check of its own, so no module check is
listed. The runner does not validate the value: as the reference's pack relies on its binding step to
refuse a malformed one, a port's reader should refuse it instead (README section 10). Run from
`src-tauri/crates/bridge-tax-audit`, once per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python EDGE_GOLDEN_WITH_RUNNER ENGINE \
        BOOK.json OUTDIR

Each run writes `OUTDIR/edge.<book>.knock_off_candidates.json`. Running every book a second time,
into a new directory, reproduced every golden byte for byte. The synthetic golden comes from the
crate's `parity/python_golden.py` extended by the runner README section 14 shows, run on
`tests/fixtures/synthetic-engagement.toml` with `--test knock_off_candidates`. As a control, the
same two files at reference commit `98ee6e71`, which precedes the change README section 13.1
describes, reproduced the previous pin's fourteen goldens byte for byte.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `ko_empty.json` | 2,000 | `9d895c03045cdcc168d0d84b39c38edaf506f72273f32af56061b2fd2b19efc9` | `books/ko_empty.json` |
| `ko_groups.json` | 5,008 | `7785a804be5657953c1b87f2afbef4c4e5a41165235a0893e311b03e71078234` | `books/ko_groups.json` |
| `ko_groups_off.json` | 4,703 | `ac1b21d07bd67a2b7f59fa3e63528fd0829be4f72fdbdc2af5e631328df84414` | `books/ko_groups_off.json` |
| `ko_nested.json` | 8,883 | `f62f1ad183cb353a1a84abb19b4769e68d64cc52f7d217d30e220cff213493de` | `books/ko_nested.json` |
| `ko_quiet.json` | 5,641 | `ec7b1c8bf4931e0ddfec33f3fbf7ff5859e32cc3a7223c4b4cb5088bcbd26b36` | `books/ko_quiet.json` |
| `ko_status.json` | 3,035 | `35c4ecc190c738a847b4443e000e3871e16e83793b7e7bf4cbd6e80679c18fb4` | `books/ko_status.json` |
| `ko_t1.json` | 6,809 | `f4b9001fb4159c95535ca39af67d214b52a855d9168fdb329054b254bd5f0dd8` | `books/ko_t1.json` |
| `ko_t2.json` | 9,559 | `210804c9d247b4ddfcf18cd1fb28ab1dfd3239ee9c6ee90c9ee79c6aaa641817` | `books/ko_t2.json` |
| `ko_tags.json` | 4,678 | `5755e52427a9e3a6bf27801bf3d7ec39e92227da75585776d7c904128adce9f8` | `books/ko_tags.json` |
| `ko_text.json` | 5,831 | `a7239ad59fb580a1c0514dbedb1651d09dcc556da1e8451d2555c7fbe814fb99` | `books/ko_text.json` |
| `ko_vocab.json` | 16,711 | `24cb4a7e1ca220e4820005b9925c644a0c602f55e55011b94872900669b58852` | `books/ko_vocab.json` |
| `ko_shared.json` | 2,846 | `9f37b1242c5fbbac624c835af6cd4d62102e8e87feacd4e13136397deb5f0c20` | `books/ko_shared.json` |
| `ko_transfer.json` | 2,620 | `72ba98baa3d79606e97db46c48c66ee70ffdd0c1271fe3ded2bb361ec6ab1086` | `books/ko_transfer.json` |
| `edge.ko_empty.knock_off_candidates.json` | 3,561 | `7467dc7d2a4ae999fbad76f01f2eff01968c0886ab600104ffb6a8fe59c98844` | `goldens/edge.ko_empty.knock_off_candidates.json` |
| `edge.ko_groups.knock_off_candidates.json` | 13,275 | `cb74f3e433fab80c4fe9c273f7215fd930646903b80af62f218a005215897725` | `goldens/edge.ko_groups.knock_off_candidates.json` |
| `edge.ko_groups_off.knock_off_candidates.json` | 8,404 | `65f951e162261a0dd4cef35efa5eb7695f7be114367d58c066dc0d45558b20c5` | `goldens/edge.ko_groups_off.knock_off_candidates.json` |
| `edge.ko_nested.knock_off_candidates.json` | 19,099 | `66653a670cd3c14693d2affaeabfc98897b1d85f67791887734f561907dfdb68` | `goldens/edge.ko_nested.knock_off_candidates.json` |
| `edge.ko_quiet.knock_off_candidates.json` | 3,561 | `7467dc7d2a4ae999fbad76f01f2eff01968c0886ab600104ffb6a8fe59c98844` | `goldens/edge.ko_quiet.knock_off_candidates.json` |
| `edge.ko_status.knock_off_candidates.json` | 5,509 | `58486067fa7b74221cc9cf508592fea0e735e01852f671fd9eed2e2ab9baa782` | `goldens/edge.ko_status.knock_off_candidates.json` |
| `edge.ko_t1.knock_off_candidates.json` | 8,101 | `f57b5ad79070f3598b4505bbf6c176ce94c02434dc2024a1f6f2405490f5661e` | `goldens/edge.ko_t1.knock_off_candidates.json` |
| `edge.ko_t2.knock_off_candidates.json` | 17,027 | `f400f9e9acfcaaa5e83f5cdde1691b50df6fa347812c895e13f18b4d68c61872` | `goldens/edge.ko_t2.knock_off_candidates.json` |
| `edge.ko_tags.knock_off_candidates.json` | 14,033 | `396e33a426c1cce79c6cdb365f513c0b7eeddac284e1792e7724367970076382` | `goldens/edge.ko_tags.knock_off_candidates.json` |
| `edge.ko_text.knock_off_candidates.json` | 15,052 | `11e2d20ee1bea419be22b74eaab200a7443c4ca545f26feec6e24e1a8e9c9bb8` | `goldens/edge.ko_text.knock_off_candidates.json` |
| `edge.ko_vocab.knock_off_candidates.json` | 36,477 | `4c4e74613bfb9e778a4a45afcccb33acc1f686b9a8d2dcb66212c2062e9bc73d` | `goldens/edge.ko_vocab.knock_off_candidates.json` |
| `edge.ko_shared.knock_off_candidates.json` | 6,922 | `6668237665bf24a8df569706d51ceab7fdd7bb8f0279beb1960852bcbbcbcd41` | `goldens/edge.ko_shared.knock_off_candidates.json` |
| `edge.ko_transfer.knock_off_candidates.json` | 7,331 | `28f12db8f7cdd5527ad9b7bc53aed8a455bbe7eb80f84691fd225fb1bf9cd2ec` | `goldens/edge.ko_transfer.knock_off_candidates.json` |
| `synthetic.knock_off_candidates.json` | 4,117 | `bab0b914b24aea4014b2aa125f95ad2ce7d975eb6c2b15887bb344eb19051276` | `goldens/synthetic.knock_off_candidates.json` |
