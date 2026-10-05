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

Each rule below was changed alone, in memory, in the reference test, and every book was run again; the
books named are those whose golden then changed. A port that gets one of these rules wrong fails at
least one golden.

| Rule changed | Goldens that change |
| --- | --- |
| a casefold collision between two party masters no longer hides the name | `ko_t2` |
| no casefold collision check at all | `ko_t2` |
| any one of 54 never-name-like words removed from the list | `ko_vocab` (and others for some words) |
| the word `a` removed | none (a one-letter word fails the length test anyway) |
| any one of the nine separators removed | `ko_vocab` (and others for `-`, `/`, `,`) |
| a full stop, `&`, `#`, `_` or an apostrophe made a separator | `ko_vocab` (and `ko_text` for the apostrophe) |
| the token after a span compared with the part of its first token | `ko_vocab` |
| the token before a span compared with the part of its last token | `ko_nested` |
| `casefold()` replaced by `lower()`, or by ASCII-only lower-casing | `ko_vocab` |
| zero lines kept in `<ledgers>` | `ko_vocab` |
| parts of a name that is also plain kept in `<parts>` | `ko_vocab` |
| names sorted after NFC normalisation | `ko_vocab` |
| zero lines ignored when deciding whether a party is on the voucher (T2) | `ko_t2` |
| zero lines counted for T1 | `ko_quiet`, `ko_t1` |
| the earlier of two same-key names kept | `ko_t2` |
| nested spans kept | `ko_nested` |
| the shown narration allowed 81 characters | `ko_text` |
| name-like words needing three letters | `ko_nested` |

## How the goldens were produced

At the reference engine (a private repository), commit `4df1cc43`, under Python 3.13, with the
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
into a new directory, reproduced every golden byte for byte.

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
| `ko_t2.json` | 9,336 | `9b346468e0f84fd8b08c50f72bbccb46303e96c61bcc452997c40c99a74a3bb7` | `books/ko_t2.json` |
| `ko_tags.json` | 4,678 | `5755e52427a9e3a6bf27801bf3d7ec39e92227da75585776d7c904128adce9f8` | `books/ko_tags.json` |
| `ko_text.json` | 5,831 | `a7239ad59fb580a1c0514dbedb1651d09dcc556da1e8451d2555c7fbe814fb99` | `books/ko_text.json` |
| `ko_vocab.json` | 16,700 | `ec28c7fb06dce0300e213cad4e8c484d509f5dfe507e21bac75ff4b63aa93817` | `books/ko_vocab.json` |
| `edge.ko_empty.knock_off_candidates.json` | 3,255 | `b8f0e7dce45acd2075c1ea5a0a9801e21ce42e0a123b9d968ecfddb1535607a1` | `goldens/edge.ko_empty.knock_off_candidates.json` |
| `edge.ko_groups.knock_off_candidates.json` | 12,137 | `f9a5ce70c916c1a558e74420842d06d6b9495e1f5da1a1d0afaafb6fb0cf4985` | `goldens/edge.ko_groups.knock_off_candidates.json` |
| `edge.ko_groups_off.knock_off_candidates.json` | 7,553 | `bf17ffb904d3ae2ded6219512f120eb8dd395c3918ec104f9d4c75b01ca0882a` | `goldens/edge.ko_groups_off.knock_off_candidates.json` |
| `edge.ko_nested.knock_off_candidates.json` | 17,943 | `7b33cafd8f2f7f78e0f441ff2e1da9dafaec579d19741722af4490ab256e4477` | `goldens/edge.ko_nested.knock_off_candidates.json` |
| `edge.ko_quiet.knock_off_candidates.json` | 3,255 | `b8f0e7dce45acd2075c1ea5a0a9801e21ce42e0a123b9d968ecfddb1535607a1` | `goldens/edge.ko_quiet.knock_off_candidates.json` |
| `edge.ko_status.knock_off_candidates.json` | 5,222 | `49617a1f72e954ee289254be6a7051428f83273d1ca4efa5769ac1e376b9ef9e` | `goldens/edge.ko_status.knock_off_candidates.json` |
| `edge.ko_t1.knock_off_candidates.json` | 7,814 | `0128b94c9f5beb7a0a322f8ae344f03f7ccf9fc577264e1f91b1a7a1a7d2b738` | `goldens/edge.ko_t1.knock_off_candidates.json` |
| `edge.ko_t2.knock_off_candidates.json` | 14,768 | `f7ad7f17ac6e21cb3e88fe64de1848e1e798748d30963a05775c724b094f1ccf` | `goldens/edge.ko_t2.knock_off_candidates.json` |
| `edge.ko_tags.knock_off_candidates.json` | 12,570 | `16cff631d29e7e91c8f52b25f00224cc65841fc93fdfe413cdcff20e0c742d3b` | `goldens/edge.ko_tags.knock_off_candidates.json` |
| `edge.ko_text.knock_off_candidates.json` | 14,392 | `5e4e6cb65155f010a491af989751e59f5c54d52a0cdca26543c62454c3a1a873` | `goldens/edge.ko_text.knock_off_candidates.json` |
| `edge.ko_vocab.knock_off_candidates.json` | 33,299 | `efc998f0766521ee19e9082e352649d2bd235c4d00d6b4086de433cab1b2eba4` | `goldens/edge.ko_vocab.knock_off_candidates.json` |
