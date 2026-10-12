# Fixture provenance: `clause21a_candidates` (#787)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

`clause21a_candidates` (Form 3CD clause 21(a)) lists P&L debits that may be one of the clause's items, for the CA,
and never judges them. A debit line on a ledger under Purchase Accounts, Direct Expenses or Indirect Expenses is a
candidate for an item when the words of the keyword table match its ledger name, its voucher's narration or party
field. Two rules need no word: a large entry on a repairs, maintenance, software, computer or office expenses ledger
is a capital candidate, and a debit in a voucher that credits a Capital Account ledger is a personal candidate,
except on a partner's interest or remuneration ledger. Credits on the same ledgers that match are shown beside. The
client's `[clause21a].extra_terms` add words; the partners' ledgers come from `[partners]`.

The words are data: `rules/clause21a_keywords.toml`, the reference's keyword table at its commit `ccc8f8a0`, byte
for byte (6,826 bytes, SHA-256 `28cb01475ab4ca72b8f9fb1cfb29ef4dcace3f56614d6fb64703d6685b4721b3`). The table's `source`
lines are printed in each finding's first limit, so they are output. The goldens below were made by the reference
running on its own table, and `parity/python_golden.py` (`require_vendored_clause21a_keywords`) refuses to run unless
the crate's copy equals the reference's file byte for byte (shown to refuse a reference at an earlier commit, whose
table differs).

- `golden/synthetic.clause21a_candidates.json` is the test on the synthetic read, through the registry's path from
  the engagement. `synthetic-engagement.toml` gains `[clause21a.extra_terms]`, one invented word the CA adds to the
  capital item ("hardware"), so the purchases ledger's three debits are capital candidates; without it the read
  has no candidate, and the comparer refuses to compare two empty results. Its one partner's interest ledger is
  passed, so that partner's interest, credited to the partner's capital, is not a personal candidate. The table is
  read by no other test: the seventeen synthetic goldens that need no caller data regenerate byte-identical with
  it (`PROVENANCE.md` carries the engagement's new byte row).
- `edge-books/c21a_skips.json`: lines passed over. A penalty word on a ledger outside the P&L groups; a nil P&L
  line with a penalty word; a P&L credit matching the penalty words with no debit beside it (an item with credits
  only lists nothing); an optional voucher with a penalty word; a repairs entry of exactly the large-entry
  threshold; P&L debits whose words match nothing; and one compounding fee, the only candidate, so the passes are
  measured against a hit (the comparer refuses two empty results).
- `edge-books/c21a_words.json`: a word for every item of the table, in the ledger name, the narration or the party
  field; ledgers under Purchase Accounts, Direct Expenses, Indirect Expenses and a group under it; one line matching
  two items ("multiple matches"); credits beside on two items; two debits of one amount on one item and one voucher
  with two lines on one item; a voucher with no number (its GUID's tail in the label); amounts with paise and in
  lakhs; words inside a longer word ("ac" in packing, "lic" in public), at the start of one ("ac" in accessories) or
  after a digit ("234e" in "1234e"), which do not match; and a cancelled voucher with a penalty word.
- `edge-books/c21a_fields.json`: where a word is read. "new" in the narration and in a ledger name, but not only in
  the party field or inside "renewal"; "fine" with a qualifier in the same field, and not without one, with its
  qualifier in another field, or only in the party field; a club word required beside a fee or service word; the
  client's extra terms, padded, blank and repeating a table term; a word in capitals; and a word after "é", which
  matches (the table's boundary is a-z and 0-9 only), where after "x" it does not.
- `edge-books/c21a_capital.json`: a firm with two partners, one with a blank interest ledger. A large entry over the
  rules' threshold on a repairs and maintenance, an office expenses and a computer maintenance ledger; at the
  threshold, as a credit, or on a rent ledger whose narration names office expenses (the rule reads the ledger name
  only), none. Debits in vouchers that credit a partner's capital: on the
  partners' interest and remuneration ledgers not personal, unless the narration says so; on a partner's salary
  ledger and on travelling, personal; a P&L credit in such a voucher and a debit in a voucher that debits capital are
  not. One line is both a capital and a personal candidate.
- `edge-books/c21a_default_threshold.json`: the rules without their `[ledger_scrutiny]` table, so the threshold is
  module's own default (`ledger_scrutiny`'s): one paisa over it is a capital candidate, at it is not. The rules'
  value equals that default, and an edge book can only drop the table, so that the rules' own value is read is
  shown by a unit test (`the_rules_large_entry_threshold_is_read`), not by a golden.
- Not reached: a refused `[clause21a].extra_terms` (an unknown bucket or a malformed value; the port refuses both
  with a typed configuration error, asserted in the module's unit tests) and a voucher of unknown status (the
  population refuses). A bucket whose words strip to nothing is not reachable with the vendored table.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about
  reading Tally or about how often the words find a real clause 21(a) item.

## How they were produced

At the reference engine (a private repository), commit `ccc8f8a0`, under Python 3.13. That commit is `ee17d80f`
with two comment lines of the keyword table's header reworded and nothing else changed, and the runner below
refuses a reference whose table is not this crate's copy, so `ccc8f8a0` is the commit to run. From the crate
directory:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.clause21a_candidates.json \
        --test clause21a_candidates
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The five books were written by a small generator, as data. Each Trial Balance is the sum of the in-books vouchers'
lines, so no book invariant fires. With the same extraction, `parity/edge_golden.py` (with this change) regenerates
every edge golden already pinned byte-identical, except the six that the reference's fixes for vouchers sharing a
GUID (#1243) have moved since they were pinned; those are re-pinned with the ports of those fixes.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `synthetic.clause21a_candidates.json` | 4,921 | `7e6f5e77d6ae89078c41348bb0635dcc2299e8233845bb592591b8e310fc02e0` | `golden/synthetic.clause21a_candidates.json` |
| `c21a_skips.json` | 4,921 | `3df2a001af414b6dc1636298938840f45fc8cc95304b69f0dc323afea324f0a4` | `edge-books/c21a_skips.json` |
| `edge.c21a_skips.clause21a_candidates.json` | 3,359 | `a00855ace6fb4bf47e2efb3172ccf6009d3958d941a55e236b8ae6d7393a12ed` | `golden/edge.c21a_skips.clause21a_candidates.json` |
| `c21a_words.json` | 9,403 | `f15edcabb0bdf10e26798ef4cb6904cec7146bc23abc3c8b89dafaff6dc28683` | `edge-books/c21a_words.json` |
| `edge.c21a_words.clause21a_candidates.json` | 32,709 | `73c017d47e89b46f639375d63c2bd1c7c26bc7da1cb0cd397c6915970471e44e` | `golden/edge.c21a_words.clause21a_candidates.json` |
| `c21a_fields.json` | 7,208 | `63cfa740cdb689004f94ed12f154718afb937d8facc8d4d206933857aa8a8fdc` | `edge-books/c21a_fields.json` |
| `edge.c21a_fields.clause21a_candidates.json` | 18,565 | `9693b640aa6a7cde6c462f82db6f3d338565c95b653f408cc6410e53d93c87b5` | `golden/edge.c21a_fields.clause21a_candidates.json` |
| `c21a_capital.json` | 8,092 | `4df37a584631fa054ea3c576ca7f83494181bbe0ee7188b38bde11a69864aec0` | `edge-books/c21a_capital.json` |
| `edge.c21a_capital.clause21a_candidates.json` | 8,189 | `c620cec76f74ffc2c5bc0c3356e86d11d1bf6e87dbeaab41f4a1f4a99f7d9a5d` | `golden/edge.c21a_capital.clause21a_candidates.json` |
| `c21a_default_threshold.json` | 2,401 | `4b9ff227b9001f0b20117e13c270f4bd8febcde9667533ba1bc7b9c93c4a44b4` | `edge-books/c21a_default_threshold.json` |
| `edge.c21a_default_threshold.clause21a_candidates.json` | 3,580 | `ef3efbdc0d177084273012af97a6ca74463e0be80d8272686a650cc37caff126` | `golden/edge.c21a_default_threshold.clause21a_candidates.json` |
