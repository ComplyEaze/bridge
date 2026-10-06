# Fixture provenance: `clause44` (#787)

Every book here is invented, with plain names and irregular invented amounts: no fixture is a Tally read of any
real assessee. Every GSTIN is made up and none has a real GSTIN's shape: its fourteenth character is `Y`, never `Z`.
The test decides a column by whether a GSTIN is present once stripped; the module check prints it
upper-cased.

## What these fixtures establish, and what they do not

`clause44` (Form 3CD clause 44) breaks total expenditure up by the GST registration status of the supplier. Every
nonzero line in a books-population voucher on a ledger whose own primary group is Purchase Accounts, Direct Expenses
or Indirect Expenses is counted once: in one of the clause's four columns, decided per voucher from the supplier's
GSTIN (the voucher's own PARTYGSTIN first, then its party ledger's), or under a closed reason outside them. The
client's configuration names the no-supplier, depreciation, round-off and tax ledgers, the composition suppliers
and the money categories; nothing is read from a ledger name. The module check (CL44-1 to CL44-3) ties the total to
the Trial Balance through `financial_statements`, and re-derives each cited voucher's GSTIN from the book.

- `golden/synthetic.clause44.json` is the test on the synthetic read, through the registry's path from the
  engagement. `synthetic-engagement.toml` gains the required `[roles].no_supplier_expense_ledgers` (empty: no ledger
  of the invented book lacks a supplier by nature) and `[clause44.money_category_by_ledger]`, which makes the
  partners' interest ledger a money item for the CA. The read carries no GSTIN and names no party ledger on an
  expense voucher, so every line is outside the four columns, and its deliberate 500-paise Trial Balance difference on one expense ledger fires CL44-1.
  No other test reads either key: every other synthetic golden the reference produces regenerates byte-identical
  with them (`PROVENANCE.md` carries the engagement's new byte row).
- `edge-books/c44_core.json`: the four columns and the closed reasons. A GSTIN from the party ledger, and from the
  voucher's own PARTYGSTIN (padded and lower case) on a cash purchase and on a purchase from a ledger with none;
  composition from the client's map, padded, in capitals and after a no-break space ("Regular" is not composition,
  and a composition supplier with no GSTIN stays unregistered); input tax on the voucher for "other than
  composition", none or a nil tax line for "exempt or non-GST"; a debit note's credit lines; a nil expense line;
  round-off riding with a real line, alone, or beside only salary; salary configured as no supplier, depreciation, a
  journal with no party and one whose party is an income ledger; a party named on no ledger master; and round-off
  beside a real line on a journal with no party, which the reference puts in "unregistered" while its sibling is
  "no party".
- `edge-books/c44_money.json`: the money categories. Interest to a bank (exempt) and bank charges (other than
  composition) are forced whatever the lender's GSTIN, and the module check leaves them out; a charge on a journal
  with no party is "no party", and a bank-charge ledger on the no-supplier list is "no supplier" (a reason comes
  before a forced column); interest to an individual lender keeps the ordinary rule and raises its judgement
  finding, including on a ledger that is also a depreciation ledger; partner remuneration (also on the no-supplier
  list) and a mixed interest-and-charges ledger are judgement items; a depreciation ledger whose category is a
  judgement one is a depreciation entry; an unrecognised category and a padded one change nothing.
- `edge-books/c44_excluded.json`: the post-dated reconciling item. The Trial Balance includes two post-dated
  vouchers' movement, so the total ties only with `population_excluded_expense_paise`; an optional and a cancelled
  voucher are in neither; a post-dated voucher with two expense lines and a nil one, and a third repeating the
  second's GUID (cited once, labelled by the last).
- `edge-books/c44_tie_100.json` and `edge-books/c44_tie_101.json`: the Trial Balance's expense closing exactly Re 1
  under the walk (no violation) and 101 paise over it (CL44-1, a negative unexplained difference).
- `edge-books/c44_traps.json`: what the module check catches. A PARTYGSTIN of only spaces reads as no GSTIN in the
  walk but the check falls back to the ledger's (a Tally read cannot produce it, since the reader strips the tag, so
  only a book built directly reaches this case, and mutants CL44-05 and CL44-06 die only here); a regular voucher whose GUID a later cancelled voucher with an
  unregistered party repeats; two regular vouchers sharing a GUID in two columns; a line on a ledger with no master;
  a party that is a sales ledger.
- `edge-books/c44_quiet.json`: no expenditure line at all: every figure zero, only the break-up finding.
- Not reached by a golden, and asserted in the crate's unit tests instead: a map value that is not text (the port
  refuses it up front, `CLAUSE44-config-shape`, whether or not a line reaches it; the reference validates neither
  map, raises only when a line reaches a truthy non-text registration type or a list or table category, and reads
  every other non-text value as no status or no category); a missing required key; a total past i64 (the port
  refuses, where the reference's integers are unbounded); CL44-1's columns-against-total branch, CL44-2 and an
  unresolvable cited voucher in CL44-3, none of which the test's own result can reach.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading
  Tally. Agreement on real books is the maintainers' local parity run, never committed.

## How they were made

`edge-books/*.json` were written by a generator. It gives every nonzero voucher line but the balancing one an
irregular paise offset from a fixed seed (a nil line stays nil), derives each Trial Balance row from the book's own
vouchers (regular, and post-dated where a book says so), then shifts one row where a book tests the tie. Each golden is
the reference engine's own canonical dump at its commit `a77b9146`, made with the crate's harness, per book:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

and on the synthetic engagement:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.clause44.json \
        --test clause44

Before any of them, the same harness reproduced two existing goldens (`edge.rp_core.related_parties_cl23`,
`edge.ep_gap.entity_269st_gap`) byte for byte at that commit. Running every book a second time reproduced every
golden byte for byte. The goldens are regenerated only by the reference's maintainers.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `c44_core.json` | 13,255 | `3ff9c99c1e6473976ad6616ca8621d234ed1d9948c810aa9f54073bdfc079bd8` | `edge-books/c44_core.json` |
| `c44_excluded.json` | 5,378 | `bbe402f2c2f3054e984cbc5202cf346e13d410ad0dba33745cdd4d252058b256` | `edge-books/c44_excluded.json` |
| `c44_money.json` | 10,150 | `f68694ee3fa0667eb63f41c24274365e6249f9e3d385f7a1504743010c873218` | `edge-books/c44_money.json` |
| `c44_quiet.json` | 3,189 | `17e8a7095da41bfde00d95a2c309d00b23a0294969bc386194dcf11596f13750` | `edge-books/c44_quiet.json` |
| `c44_tie_100.json` | 3,447 | `69fb9105f001eb09e5c0e058c46e1bbb3ed2e5b77968bdddafc810f278f72c80` | `edge-books/c44_tie_100.json` |
| `c44_tie_101.json` | 3,479 | `a99919eaf73a22398610ca0a1ae3c9b54bc8bd9ca3875d073c6018f2c0bcc8c2` | `edge-books/c44_tie_101.json` |
| `c44_traps.json` | 5,556 | `33d4dcce66956243961b824bf48cc84f0871a004e33c5d67d0d8a69a6800f005` | `edge-books/c44_traps.json` |
| `edge.c44_core.clause44.json` | 21,799 | `83f31b8f2771ea7e308cbe8fba737cd58b42941aab512d5035a152a4cb1e463f` | `golden/edge.c44_core.clause44.json` |
| `edge.c44_excluded.clause44.json` | 18,065 | `4434b518c7b55d2d204abc7d6d3ccb6eb87d1aeaa9af84f0658caa9a39246623` | `golden/edge.c44_excluded.clause44.json` |
| `edge.c44_money.clause44.json` | 24,523 | `9044269dce8ff78119117a613502117266841fad2ca5c41bf68c68b71a0a1d84` | `golden/edge.c44_money.clause44.json` |
| `edge.c44_quiet.clause44.json` | 16,984 | `e096cd8be2257cd936648a788380516289c361bf2a075f0af2a7270c77f72dde` | `golden/edge.c44_quiet.clause44.json` |
| `edge.c44_tie_100.clause44.json` | 17,248 | `b3f94a5a979d46e69d699185a10688d24dc4b8249c51f628df7d4d59be12c324` | `golden/edge.c44_tie_100.clause44.json` |
| `edge.c44_tie_101.clause44.json` | 17,879 | `b67eaf17ff85412ef6dde65912e77c0ce7420f38f5267fec6737847e0901f3e1` | `golden/edge.c44_tie_101.clause44.json` |
| `edge.c44_traps.clause44.json` | 19,564 | `f3134b7d32f43c72243e780d8ff42c4135c93dc35036c97cab35b773d66a8eea` | `golden/edge.c44_traps.clause44.json` |
| `synthetic.clause44.json` | 23,841 | `97da47c0b5033483948910da2f9862af04d3515311c70b214ac1b6d6043ff3be` | `golden/synthetic.clause44.json` |
