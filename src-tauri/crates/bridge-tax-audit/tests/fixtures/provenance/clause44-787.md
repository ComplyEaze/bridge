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
  "no party". Round-off beside only a depreciation line has no supplier; round-off beside a real line on a voucher
  whose party is an income ledger rides into the GSTIN rule while its sibling is "party is a P&L ledger". A
  PARTYGSTIN of "URP" reads as a GSTIN: the reference counts any non-blank text as one.
- `edge-books/c44_money.json`: the money categories. Interest to a bank (exempt) and bank charges (other than
  composition) are forced whatever the lender's GSTIN, and the module check leaves them out; a charge on a journal
  with no party is "no party", and a bank-charge ledger on the no-supplier list is "no supplier" (a reason comes
  before a forced column); interest to an individual lender keeps the ordinary rule and raises its judgement
  finding, including on a ledger that is also a depreciation ledger; partner remuneration (also on the no-supplier
  list) and a mixed interest-and-charges ledger are judgement items; a depreciation ledger whose category is a
  judgement one is a depreciation entry; round-off beside only a judgement item rides into the GSTIN rule (a
  judgement item counts as a real line); an unrecognised category and a padded one change nothing.
- `edge-books/c44_excluded.json`: the post-dated reconciling item. The Trial Balance includes two post-dated
  vouchers' movement, so the total ties only with `population_excluded_expense_paise`; an optional and a cancelled
  voucher are in neither; a post-dated voucher with two expense lines and a nil one, and a third repeating the
  second's GUID (cited once, labelled by the last).
- `edge-books/c44_tie_100.json`, `edge-books/c44_tie_minus_100.json` and `edge-books/c44_tie_101.json`: the Trial
  Balance's expense closing exactly Re 1 under the walk and exactly Re 1 over it (no violation either way), and 101
  paise over it (CL44-1, a negative unexplained difference).
- `edge-books/c44_traps.json`: what the module check catches. A PARTYGSTIN of only spaces reads as no GSTIN in the
  walk but the check falls back to the ledger's (a Tally read cannot produce it, since the reader strips the tag, so
  only a book built directly reaches this case, and mutants CL44-05 and CL44-06 die only here); a regular voucher whose GUID a later cancelled voucher with an
  unregistered party repeats; two regular vouchers sharing a GUID in two columns; a line on a ledger with no master;
  a party that is a sales ledger.
- `edge-books/c44_quiet.json`: no expenditure line at all: every figure zero, only the break-up finding.
- Not reached by a golden, and asserted in the crate's unit tests instead: a map value that is not text (the port
  refuses it up front, `CLAUSE44-config-shape`, whether or not a line reaches it; the reference validates neither
  map, raises only when a line reaches a truthy non-text registration type or a list or table category, and reads
  every other non-text value as no status or no category); a missing required key; a total or a running sum past i64 (the
  port refuses, in the walk and in the module check's Trial Balance sums, where the reference's integers are
  unbounded); CL44-1's columns-against-total branch, CL44-2 and an
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
| `c44_core.json` | 14,361 | `9a4a403749e4aacb3e94fa4138dff79723888e35fa88b324d8730671d07ddbc3` | `edge-books/c44_core.json` |
| `c44_excluded.json` | 5,382 | `828dff0fb7c163ec87f763557de367e51450fdbc84880abdbf1fe35460c977e6` | `edge-books/c44_excluded.json` |
| `c44_money.json` | 10,768 | `857ded188ca15d80671968bda63954a788f84e89715827c43c59ef9231066551` | `edge-books/c44_money.json` |
| `c44_quiet.json` | 3,186 | `0e729600b171fe27c69cf5d4f24ed5ab65c9c91d8568283683e39bce3b8c7231` | `edge-books/c44_quiet.json` |
| `c44_tie_100.json` | 3,447 | `166d559570f62b4ad93b1111c816c6644ce5d027012286c353ae2e61b5b7b150` | `edge-books/c44_tie_100.json` |
| `c44_tie_101.json` | 3,479 | `e5fa7f79b918db0a0efecb8fe54580aca94209f55a1bbdf5dfd15261182c6f29` | `edge-books/c44_tie_101.json` |
| `c44_tie_minus_100.json` | 3,536 | `1f5af983d4e50a8226a3fbb18da08695f31ed0f42754199ceb6148d12c322d8e` | `edge-books/c44_tie_minus_100.json` |
| `c44_traps.json` | 5,551 | `4278f8c330c4d4b16ead1c0d205d6de9fb43ffec2bae6b1e547e3c5062f06c5a` | `edge-books/c44_traps.json` |
| `edge.c44_core.clause44.json` | 22,764 | `9dd5e4f715add5fa157b97b266a0cd6c2837a84e1fb48c7433bf679969c74f89` | `golden/edge.c44_core.clause44.json` |
| `edge.c44_excluded.clause44.json` | 18,065 | `74f423d5e1291c98aff179ca1711bf6c3597ce1179de6bc87bdf7ec68fbb3cf7` | `golden/edge.c44_excluded.clause44.json` |
| `edge.c44_money.clause44.json` | 25,007 | `da2490addf69633a1e7adabc64e017be239483769b88f809fa67e3c9f66777b5` | `golden/edge.c44_money.clause44.json` |
| `edge.c44_quiet.clause44.json` | 16,984 | `e096cd8be2257cd936648a788380516289c361bf2a075f0af2a7270c77f72dde` | `golden/edge.c44_quiet.clause44.json` |
| `edge.c44_tie_100.clause44.json` | 17,248 | `7e47d1b83ec58afbab536b5dc90840608bfb78afc28a582589d3425cfe733722` | `golden/edge.c44_tie_100.clause44.json` |
| `edge.c44_tie_101.clause44.json` | 17,879 | `8dd77af0e7f6677774dc9959f3f35c4326c6051bf282daca98f8da8f128a7389` | `golden/edge.c44_tie_101.clause44.json` |
| `edge.c44_tie_minus_100.clause44.json` | 17,248 | `38ae717e947fbeba2d3b67f0b13553c505c600252bff8dab034595053e6542ee` | `golden/edge.c44_tie_minus_100.clause44.json` |
| `edge.c44_traps.clause44.json` | 19,563 | `eac401443439fce972cbcd274d448ef31f4e69bb1f41716058b86824585e8f2e` | `golden/edge.c44_traps.clause44.json` |
| `synthetic.clause44.json` | 23,841 | `97da47c0b5033483948910da2f9862af04d3515311c70b214ac1b6d6043ff3be` | `golden/synthetic.clause44.json` |
