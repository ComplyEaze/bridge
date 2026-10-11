# Fixture provenance: `narration_payees` (8 Oct 2026)

The eighteen edge books, their 36 goldens and the synthetic golden come unchanged, byte for byte, from
the spec pack `docs/tax-audit/spec-packs/narration_payees/` (regenerated in #1432), whose `HASHES.md`
lists the same bytes and SHA-256 for each file. Every book there is invented, with synthetic ledger
names, payee names from the Greek alphabet, from trees or built from the reader's own words, and
made-up bank-style tokens: no book is a Tally read of any real assessee, no narration is a line of any
bank statement, and no figure or name comes from either.

## What these fixtures establish

- The books are hand-written scenarios, each built to reach a rule or boundary of the reference
  engine's `narration_payees` test and of the narration reader it uses; the pack's README lists which
  (its section 14), and each book's `comment` says what it reaches. They are regression fixtures: the
  port reproduces every golden, so it agrees with the reference on these books, and nothing more.
- Each book names both `tds_payees` and `narration_payees`. Its `tds_payees` golden is the result this
  test was given on that book, made in the same run, so the TDS payee port is compared with it too; it
  adds nothing to that test's own contract.
- `np_empty` and `np_quiet` give byte-identical `narration_payees` goldens: with nothing read the dump
  does not depend on the book.
- `np_shared_guid` and `np_names` are books the reference's real pipeline does not let through (shared
  or blank GUIDs; listed names that match no ledger). The edge books are run unbound, as every edge
  book is, and show what the test and its check do when such a book is built directly.
- They prove nothing about what a bank prints: the narrations are built from the reader's rules, not
  taken from statements.
- They prove nothing about reading Tally: the books are built directly in the edge-book shape, not
  read from a company. Agreement on real books is established separately, by a local parity run on
  real reads that is never committed.

## How they were produced

At the reference engine (a private repository), commit `2b329354`, under Python 3.13, with
`parity/edge_golden.py` extended by the runner the pack's README gives (its section 16). The runner
reads `narration_payee_ledgers` and `bank` from the book, runs the file's own `tds_payees` runner and
works the added ledgers out of its result, then runs this test and hands the canonical dump its check,
bound to the bank ledgers and the ledgers read. Run from `src-tauri/crates/bridge-tax-audit`, once per
book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The synthetic golden is the same test on the crate's synthetic engagement, through
`parity/python_golden.py`'s runner for it:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.narration_payees.json \
        --test narration_payees

Running every book a second time reproduced every golden byte for byte. The goldens are regenerated
only by the reference's maintainers.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `np_added.json` | 6,864 | `c90b4eb8faeeba038dbe96cb9e5ff6a32ed35e92f2bb5e151ffd4940b48ab527` | `edge-books/np_added.json` |
| `np_aggregate.json` | 4,706 | `a594cf05912d84734d8bb74fa5f0f251c1b01b81490b51beafe57d7e3835e1b0` | `edge-books/np_aggregate.json` |
| `np_cut.json` | 8,759 | `e56d8d7e365c5fdffc3feb52490524f78525cc1d08a5542f2e85bdeb8812cff1` | `edge-books/np_cut.json` |
| `np_else_agg.json` | 8,012 | `0dfb69b696bfe51698378e55b075f73e7351b1e5e9484fbacc2cabc914420e10` | `edge-books/np_else_agg.json` |
| `np_else_single.json` | 3,334 | `7f478c33d09089d054036ece23c2f8399dd06dd621509fd9785dc90c36b8ff2b` | `edge-books/np_else_single.json` |
| `np_elsewhere.json` | 10,061 | `e982da7454037ac9d01def4416c358ba37104e2ebf6a33c6500918de28e7f930` | `edge-books/np_elsewhere.json` |
| `np_empty.json` | 1,890 | `24bda688017fc655b0c6bb08634f782023f2a8cef25d95b2e1ff01d265c9b71d` | `edge-books/np_empty.json` |
| `np_forms.json` | 14,738 | `7383987cfe5cad2323c50ad670d7a9d7d8fb7b71ea13c9abf695bad82ddd417f` | `edge-books/np_forms.json` |
| `np_handles.json` | 14,592 | `3e5f06e621ec6da32da26e6618c0b338729a81c63aecb9cfe0908d8da399480d` | `edge-books/np_handles.json` |
| `np_names.json` | 4,224 | `314657c4f0d8fc67745f1c87ccc53e2e5cee72a0d62bab9d61d425cf8e5e7339` | `edge-books/np_names.json` |
| `np_outside.json` | 6,097 | `62ee16b431d9e3cc9601b50007d8cbae361e1e916d774a8a7c1421f9a530c815` | `edge-books/np_outside.json` |
| `np_quiet.json` | 6,483 | `3ce716b16d8f9b636ee94ec5234e284fa3b3cd2305187c3d2cc7002f271440fb` | `edge-books/np_quiet.json` |
| `np_rows.json` | 6,739 | `111937d1af6248c3fde224457ec1ef9cc243381ba7e66f04851ff7cf0efaf717` | `edge-books/np_rows.json` |
| `np_same_day.json` | 7,138 | `5ca0b7db3b2c2e76587ef42fa6e1444d6b9ab8de2fab4071ed7c1c44d4e2753e` | `edge-books/np_same_day.json` |
| `np_shared_guid.json` | 7,865 | `1054b54261cb8285fdb1aa69a79afe44078ae2b1cd8a91e0e30da00789f614e1` | `edge-books/np_shared_guid.json` |
| `np_single.json` | 4,190 | `9f3e0290c19d31fea979348e2264df6b5d72c3c8b8dda8bf90f81475bc83c189` | `edge-books/np_single.json` |
| `np_text.json` | 11,905 | `51aafced4b7f66c6c3329caa36d215702d534df6d6ddbc8288dab3521bb91653` | `edge-books/np_text.json` |
| `np_unread.json` | 17,781 | `d5abfdf691c70b40de67f33460e605b61f1bfd3ca75add2d0f3f133183336577` | `edge-books/np_unread.json` |
| `edge.np_added.narration_payees.json` | 17,492 | `06c065fc9c84fe4553117d708bfe5e0d2701bc5265b275c5ef62be85b4544e2a` | `golden/edge.np_added.narration_payees.json` |
| `edge.np_added.tds_payees.json` | 23,458 | `c9f268881dc9ccd667f1d7428fca26634ec2879d6069c9d98b26719f827bdb0b` | `golden/edge.np_added.tds_payees.json` |
| `edge.np_aggregate.narration_payees.json` | 16,499 | `609c3ada203169cec24b8619b79ec21f90e8d34af4d303ca6614005de65e0463` | `golden/edge.np_aggregate.narration_payees.json` |
| `edge.np_aggregate.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_aggregate.tds_payees.json` |
| `edge.np_cut.narration_payees.json` | 46,830 | `ae9b3ab826b41e437fc41fc5533c9c77acc1da7c034e351116b4a6e239bd85ad` | `golden/edge.np_cut.narration_payees.json` |
| `edge.np_cut.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_cut.tds_payees.json` |
| `edge.np_else_agg.narration_payees.json` | 29,712 | `41c4af58d6a44b99399c884f369ffe04d2b1d87b5af4d0692157d923782f1bc5` | `golden/edge.np_else_agg.narration_payees.json` |
| `edge.np_else_agg.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_else_agg.tds_payees.json` |
| `edge.np_else_single.narration_payees.json` | 16,158 | `06f5dd1a141a20107cded3870b6cee2678e888f544fdd7caae66e91998703d54` | `golden/edge.np_else_single.narration_payees.json` |
| `edge.np_else_single.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_else_single.tds_payees.json` |
| `edge.np_elsewhere.narration_payees.json` | 14,751 | `6c6c30a8c5cc8e7a228fba4a83e2434233f55133dc893cef2df25c99d98d5c6c` | `golden/edge.np_elsewhere.narration_payees.json` |
| `edge.np_elsewhere.tds_payees.json` | 16,342 | `10e473aaafd51d2074c50a3d20dbb11e4e657120c59278ca6655b147665ef65f` | `golden/edge.np_elsewhere.tds_payees.json` |
| `edge.np_empty.narration_payees.json` | 5,861 | `58cb45bbb2afd1ccd63b9b94efc81dd009301cacb7f067dd641e5619b083a1b5` | `golden/edge.np_empty.narration_payees.json` |
| `edge.np_empty.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_empty.tds_payees.json` |
| `edge.np_forms.narration_payees.json` | 56,666 | `dcd5451e6254837a49cc0043bdb47b57af7d77dd54d1c5bce3fd363efe56be85` | `golden/edge.np_forms.narration_payees.json` |
| `edge.np_forms.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_forms.tds_payees.json` |
| `edge.np_handles.narration_payees.json` | 81,889 | `9a354cad474091d979eae7556f632a1bc7719f40da6a6a74cdd19405d2dcbbbe` | `golden/edge.np_handles.narration_payees.json` |
| `edge.np_handles.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_handles.tds_payees.json` |
| `edge.np_names.narration_payees.json` | 11,606 | `6cb3e66dd32794662f82d3ea99ccd54443c08cad586d383107e03598a54c2307` | `golden/edge.np_names.narration_payees.json` |
| `edge.np_names.tds_payees.json` | 16,728 | `71bc9df62e4bde59f09f1dd672446bb5ba80ee6d6dc898142aef674d14826d1e` | `golden/edge.np_names.tds_payees.json` |
| `edge.np_outside.narration_payees.json` | 21,828 | `30bcbe0fbbc93b3a01df51b9168c158bf56816aff625f4b3046e88aa4e218a0b` | `golden/edge.np_outside.narration_payees.json` |
| `edge.np_outside.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_outside.tds_payees.json` |
| `edge.np_quiet.narration_payees.json` | 5,861 | `58cb45bbb2afd1ccd63b9b94efc81dd009301cacb7f067dd641e5619b083a1b5` | `golden/edge.np_quiet.narration_payees.json` |
| `edge.np_quiet.tds_payees.json` | 20,246 | `2a3fa0d8c054f2628c0ec564e55440bb1c7603e6a24557ac250747ffda9413d1` | `golden/edge.np_quiet.tds_payees.json` |
| `edge.np_rows.narration_payees.json` | 20,912 | `d2f00f6ebb60910260df52ee60a44c4d7ee03695a384b97163873ad753365bf8` | `golden/edge.np_rows.narration_payees.json` |
| `edge.np_rows.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_rows.tds_payees.json` |
| `edge.np_same_day.narration_payees.json` | 36,999 | `e8b09f4b3181fdca62b0376d051ce818bc5d6213b912d12ecc5cb02490bf8c3e` | `golden/edge.np_same_day.narration_payees.json` |
| `edge.np_same_day.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_same_day.tds_payees.json` |
| `edge.np_shared_guid.narration_payees.json` | 29,838 | `c3e745493f1b8683ec976b24e8b881d5095c2b73f1c2a5d8940ae8e32b9f67ad` | `golden/edge.np_shared_guid.narration_payees.json` |
| `edge.np_shared_guid.tds_payees.json` | 20,803 | `8b7b799ac88aa2cf3e23066bb92bc00a4dc2e93f578416a9b9a386d1a7ea6dc3` | `golden/edge.np_shared_guid.tds_payees.json` |
| `edge.np_single.narration_payees.json` | 23,879 | `dd34c6ccf935256b2ca2656815aebc7589ca54bd19f1c7c5c67fc01f7664a7a0` | `golden/edge.np_single.narration_payees.json` |
| `edge.np_single.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_single.tds_payees.json` |
| `edge.np_text.narration_payees.json` | 34,529 | `8d8e495fee0e27c45fd8fea63e5b76eeb1cccd3cffa0f329e7616c2e78d0f63b` | `golden/edge.np_text.narration_payees.json` |
| `edge.np_text.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_text.tds_payees.json` |
| `edge.np_unread.narration_payees.json` | 23,176 | `2b6d79a211bd70be8a38f562656027da425470bced6042c7d88b00ef09859afd` | `golden/edge.np_unread.narration_payees.json` |
| `edge.np_unread.tds_payees.json` | 16,223 | `6e25a43a42bfa24d128ea99fc1984f21bc196fe499de69e6f4a6c935aeab3517` | `golden/edge.np_unread.tds_payees.json` |
| `synthetic.narration_payees.json` | 6,417 | `a3cb127263b6c64e381d0a7e576f6976a98a598e85063d40b02d597739350ef6` | `golden/synthetic.narration_payees.json` |
