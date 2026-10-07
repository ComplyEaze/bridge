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
  beside a real line on a journal with no party, which reads "no party" like its sibling. Round-off beside only a
  depreciation line takes that line's reason; round-off beside a real line on a voucher whose party is an income
  ledger takes its sibling's "party is a P&L ledger". A PARTYGSTIN of "URP" reads as a GSTIN: the reference counts any
  non-blank text as one.
- `edge-books/c44_money.json`: the money categories. Interest to a bank (exempt) and bank charges (other than
  composition) are forced whatever the lender's GSTIN, and the module check leaves them out; a charge on a journal
  with no party is forced too (a line is placed by its own ledger before the voucher's party is read), and a
  bank-charge ledger on the no-supplier list is "no supplier" (a reason comes before a forced column); interest to an
  individual lender keeps the ordinary rule and raises its judgement finding, including on a ledger that is also a
  depreciation ledger; partner remuneration (also on the no-supplier list) and a mixed interest-and-charges ledger are
  judgement items; a depreciation ledger whose category is a judgement one is a depreciation entry; round-off beside
  only a judgement item takes that item's reason. The book no longer carries an unrecognised or a padded category: the
  reference now refuses either when it reads the client's table (the crate's unit tests assert the same refusals).
- `edge-books/c44_placement.json`: where a line's own ledger puts it, before the party is read, in the order
  depreciation set, judgement category, no-supplier list, forced category. Bank interest and bank charges on a journal
  with no party, with a party that is itself an expense ledger or an income ledger, with a party named on no ledger
  master, and on a purchase from a registered supplier with no tax line and from an unregistered one, are in their
  forced columns; a forced category on a depreciation ledger or a no-supplier ledger keeps that reason; a
  line on a ledger with no category beside a forced line on the same voucher is still read by the party; interest to
  an individual lender is read by the party and counted as a judgement candidate.
- `edge-books/c44_round_off.json`: a plain round-off line (a round-off ledger with no role of its own) is shown where
  the line it rounds is shown: the voucher's first expenditure line read by the party, else the first other
  expenditure line, else "non supply". A debit and a credit round-off on a journal with no party and on one whose
  party is an expense or an income ledger; beside bank charges, bank interest, a partner's remuneration, a mixed
  ledger, a depreciation line and a no-supplier line; round-off first and then bank charges and bank interest (it
  follows the charges); a line read by the party after a forced line (a debit one, and a credit one); a capital
  invoice whose other lines are not expenditure; a registered, a composition and an unregistered supplier; two
  round-off lines on one voucher; round-off beside only round-off ledgers with a role of their own; and a
  round-off ledger that has its own category, depreciation role or no-supplier role, which is placed by that role.
- `edge-books/c44_excluded.json`: the post-dated reconciling item. The Trial Balance includes two post-dated
  vouchers' movement, so the total ties only with `population_excluded_expense_paise`; an optional and a cancelled
  voucher are in neither; a post-dated voucher with two expense lines and a nil one, and a third repeating the
  second's GUID (cited once, labelled by the last).
- `edge-books/c44_tie_100.json`, `edge-books/c44_tie_minus_100.json` and `edge-books/c44_tie_101.json`: the Trial
  Balance's expense closing exactly Re 1 under the walk and exactly Re 1 over it (no violation either way), and 101
  paise over it (CL44-1, a negative unexplained difference).
- `edge-books/c44_traps.json`: what the module check catches. A PARTYGSTIN of only spaces is no GSTIN and falls back
  to the party ledger's, in the walk as in the check (a Tally read cannot produce it, since the reader strips the tag,
  so only a book built directly reaches this case); a regular voucher whose GUID a later cancelled voucher with an
  unregistered party repeats; two regular vouchers sharing a GUID in two columns; a line on a ledger with no master;
  a party that is a sales ledger.
- `edge-books/c44_blank_gstin.json`: a voucher's PARTYGSTIN that is blank once stripped (spaces, a tab, a no-break
  space) falls back to the party ledger's GSTIN: "exempt or non-GST" with no tax line, "other than composition" with
  one, composition for a composition supplier; a supplier with no GSTIN, and a party named on no ledger master, stay
  unregistered; a populated value and a zero-width space are read as a GSTIN.
- `edge-books/c44_quiet.json`: no expenditure line at all: every figure zero, only the break-up finding.
- Not reached by a golden, and asserted in the crate's unit tests instead: a registration type that is not one of
  Tally's, and a money category that is not exactly one of the five (the port refuses either up front,
  `ConfigValueRefused`, naming the table and the ledger, as the reference now does when it reads the client's
  table); a missing required key; a total or a running sum past i64 (the
  port refuses, in the walk and in the module check's Trial Balance sums, where the reference's integers are
  unbounded); CL44-1's columns-against-total branch, CL44-2 and an
  unresolvable cited voucher in CL44-3, none of which the test's own result can reach.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading
  Tally. Agreement on real books is the maintainers' local parity run, never committed.

## How they were made

`edge-books/*.json` were written by a generator. It gives every nonzero voucher line but the balancing one an
irregular paise offset from a fixed seed (a nil line stays nil), derives each Trial Balance row from the book's own
vouchers (regular, and post-dated where a book says so), then shifts one row where a book tests the tie. Each golden is
the reference engine's own canonical dump at its commit `e0f07b55`, made with the crate's harness, per book:

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

The reference commit above is the one after which a line is placed by its own ledger before its party is read, a
plain round-off line takes the placement of the line it rounds, a PARTYGSTIN that is blank once stripped falls back to
the party ledger's, the population note and three limits are reworded, and a mistyped registration type or money
category is refused. Against the goldens at the previous reference commit the moves are: every dump's population note
and the break-up finding's limits; the money book's finding on interest to an individual lender (its limits); and, in
figures, `c44_core` (`c20`, `c21` and `c22`: round-off beside a journal's lines), `c44_money` (`m03`, a charge on a
journal with no party, now forced; `m14`, round-off beside a judgement item) and `c44_traps` (`t01`, a PARTYGSTIN of
spaces). `c44_money` also lost the two category entries the reference now refuses; the figures do not move for that
alone. `c44_placement`, `c44_round_off` and `c44_blank_gstin` are new books from the same generator and seed; the
existing eight books are otherwise unchanged apart from their comments.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `c44_blank_gstin.json` | 5,924 | `1ee7cb0d1280d78a699493925490adcf002e80ef17335347085096af7087dd8f` | `edge-books/c44_blank_gstin.json` |
| `c44_core.json` | 14,308 | `0c6c1e1faf8d551a8e750a2728ae207bf3ddde785c55241d4e6647ba018735fc` | `edge-books/c44_core.json` |
| `c44_excluded.json` | 5,382 | `828dff0fb7c163ec87f763557de367e51450fdbc84880abdbf1fe35460c977e6` | `edge-books/c44_excluded.json` |
| `c44_money.json` | 10,702 | `03d4224d78349ae78077f77ba0a95355d6cc66de7e8c923d6a025dc0f4113c3f` | `edge-books/c44_money.json` |
| `c44_placement.json` | 10,727 | `dd0ab2738a07007e4491bcb509b0b8c8dc6e0bf08bb86910deeeeed09f27e769` | `edge-books/c44_placement.json` |
| `c44_quiet.json` | 3,186 | `0e729600b171fe27c69cf5d4f24ed5ab65c9c91d8568283683e39bce3b8c7231` | `edge-books/c44_quiet.json` |
| `c44_round_off.json` | 17,654 | `d67c89aa238783faf640a5838bb4c76b38d48c01857ef4e9f77f6504b7a6a6c0` | `edge-books/c44_round_off.json` |
| `c44_tie_100.json` | 3,447 | `166d559570f62b4ad93b1111c816c6644ce5d027012286c353ae2e61b5b7b150` | `edge-books/c44_tie_100.json` |
| `c44_tie_101.json` | 3,479 | `e5fa7f79b918db0a0efecb8fe54580aca94209f55a1bbdf5dfd15261182c6f29` | `edge-books/c44_tie_101.json` |
| `c44_tie_minus_100.json` | 3,536 | `1f5af983d4e50a8226a3fbb18da08695f31ed0f42754199ceb6148d12c322d8e` | `edge-books/c44_tie_minus_100.json` |
| `c44_traps.json` | 5,486 | `342ce37a62466ff3d04670bad7e68fabf99dee8197ac339d38d7322cd094b7f0` | `edge-books/c44_traps.json` |
| `edge.c44_blank_gstin.clause44.json` | 19,063 | `5378c3626d9824fc3d5146ff2ec7f465f1abb989cc4f0bf6661bccb15654f87e` | `golden/edge.c44_blank_gstin.clause44.json` |
| `edge.c44_core.clause44.json` | 22,736 | `9ad1e74c6ff446d943e6dccbeb515457bd130761a60baf54440ae73ade1e7fbb` | `golden/edge.c44_core.clause44.json` |
| `edge.c44_excluded.clause44.json` | 18,396 | `5f2e1a1836e70969fa86f60d7b8fe2937cb58234d12c79e3c7e53341dafe7d06` | `golden/edge.c44_excluded.clause44.json` |
| `edge.c44_money.clause44.json` | 25,426 | `1bafad264a2d82c492f4e5185533fbe71d3fb3676da282d8b6375ba02107dab3` | `golden/edge.c44_money.clause44.json` |
| `edge.c44_placement.clause44.json` | 27,124 | `27af85e629036b12fd1b0afafbf17b43e3d4efb5a69fa7d89aa3585ad617977a` | `golden/edge.c44_placement.clause44.json` |
| `edge.c44_quiet.clause44.json` | 17,315 | `87a1cbd814c9201c2d4ee7e034070e64429173462ff0e1bc49715d68a119b926` | `golden/edge.c44_quiet.clause44.json` |
| `edge.c44_round_off.clause44.json` | 31,086 | `9dce1e461573c8551b3fd35a058172d10d890da92274bf53bc63336b82d88ca2` | `golden/edge.c44_round_off.clause44.json` |
| `edge.c44_tie_100.clause44.json` | 17,579 | `2eb14e77dbe1a5b160a6139cf0cdf3d2fa6be3768337a751bb4d2899f62bb2f5` | `golden/edge.c44_tie_100.clause44.json` |
| `edge.c44_tie_101.clause44.json` | 18,210 | `dabc778b524459b24ae8323616a8b912ddcb35413ea9901d1f644fd79a0475b2` | `golden/edge.c44_tie_101.clause44.json` |
| `edge.c44_tie_minus_100.clause44.json` | 17,579 | `6d20eea41a81050c1c50be3635021870201e6011b851cd86344621c35169f711` | `golden/edge.c44_tie_minus_100.clause44.json` |
| `edge.c44_traps.clause44.json` | 19,670 | `aea7c99ddba6d2b8ed7774e8691ac126dff81e4c3d9d16d01cd6d07249a20e54` | `golden/edge.c44_traps.clause44.json` |
| `synthetic.clause44.json` | 24,172 | `f11fd574cf55cdc49f6d97b5466ab51ed08fd89853090d58d2a8904dbf491c5d` | `golden/synthetic.clause44.json` |
