# Batch E4 fixture provenance: `party_monthly`

Lane E, 2026-09-25. Every book here is invented: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

- `golden/synthetic.party_monthly.json` is the reference's dump on the synthetic read (53 figures since the
  `da9e2d3d` re-pin below; its Trial Balance ties in every block, and its one finding is an indirect expense
  not attributed because no party is on the other side).
- `edge-books/pm_paths.json` (`top_n` 2) reaches: named parties, "Others" with its plural and singular
  labels, a sub-group party, the cash-or-bank, no-party and
  several-parties rows (an expense with a cash line stays no party), credit and debit notes, a month
  that nets to nil (no figure), a voucher dated after the period, two vouchers sharing one GUID counted as
  two; and the Trial Balance tied within a rupee, matched by one optional voucher, matched only by the
  post-dated and cancelled vouchers together, and matched by two sets at once (each listed, none chosen,
  since the re-pin below), with an opening balance on a P&L ledger. It does not reach an equal-total tie: its own comment says Cust C
  and Cust D tie, but they do not (120,000 against 100,000 paise in the golden, both in "Others"). The
  tie at the cut, a voucher on the period's first day, a Trial Balance difference of exactly Re 1 and
  the ranking by absolute year are pinned by the unit test in `src/party_monthly.rs` instead.
- `edge-books/pm_attribution.json` (added at the re-pin below) reaches the party attribution by side: a
  supplier paid with bank charges, and a scrap sale by journal with the customer on the same side, not
  attributed (indirect expenses, sales); a journal with one party on each side, the other side's; two
  parties on the other side, several parties; a party netting to nil on a payment, not attributed (no
  party on the other side), and a journal whose block lines net to nil, not attributed (nil); a receipt
  net of bank charges, the customer's until the `da9e2d3d` re-pin and not attributed (the money reason)
  since; a round-off on a sales, purchase, credit note and debit note voucher, that voucher's party's
  though on the same side. And the Trial Balance wording: matched by one status with another tried; two
  statuses tried and none matching; the one status tried not matching, beside a post-dated voucher
  netting to nil there (no set); and the optional vouchers exactly and all three statuses together
  within a rupee, both listed until the re-pin and only the exact set named since. Its comment was brought
  up to date at that re-pin (its golden does not read it).
- `edge-books/pm_money.json` (added at the `da9e2d3d` re-pin): the money reason. In indirect expenses a
  supplier paid in cash with rent credited on the cash's side; a receipt net of bank charges from two
  customers; a receipt net of charges that also pays a supplier, so the bank nets to the other side but
  one bank line is on the charges' side; and a round-off on a receipt: each not attributed. A round-off
  on a cash sales invoice stays the customer's; rent with the bank on the supplier's side, and rent
  reversed to a supplier beside a nil bank line, stay the supplier's. Bank charges on a contra with the bank
  on their side, and a journal moving rent to bank charges (lines netting to nil), carry no party ledger and
  are No party: neither the money nor the nil reason applies without one. In sales and purchases a cash or
  bank line on the lines' side leaves the party its lines. In direct expenses three receipts net of
  charges, two identical (one GUID, number, date and lines) and one with no GUID, each counted and cited,
  and a supplier with no GUID paid with freight on its side (the other-side reason, with the expense
  example).
- `edge-books/pm_exact.json` (added at the same re-pin): a set of excluded vouchers matches only exactly.
  Sales: two credit-note sets each account for the difference (both listed, with the sign their amounts
  use, credits less debits). Purchases: a one-paisa optional voucher does not explain a difference of Rs
  1.01. Direct expenses: two optional vouchers netting to nil, and two cancelled ones, so the post-dated
  set and all of them together each match and none is named (the reference's recorded limit); the
  all-together set names three statuses ("post-dated, optional and cancelled"). Indirect expenses: three
  statuses tried, the optional set a paisa off, so none matches and all three are listed.
- `edge-books/pm_not_fy.json`: a calendar-year period (no month figures), a missing Direct Expenses
  group, and a Trial Balance difference no excluded voucher matches.
- `edge-books/pm_empty.json`: all four groups and no voucher. Its 16 figures are the registry's
  `min_figures`.
- A party whose tag equals a fixed row's repeats a figure id: refused on both sides
  (`tests/edge_books.rs`). PWM-1 and PWM-2 are shown to fire on tampered results.
- Regression fixtures only: the evidence for real books is local parity on the real reads, never
  committed.

## How they were produced

At the reference engine (a private repository), commit `1038dc05`, under Python 3.13 (regenerated at the
re-pin below):

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.party_monthly.json \
        --test party_monthly
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/pm_NAME.json tests/fixtures/golden

## Re-pin at the reference's current head (Lane E2, 2026-09-28)

The goldens were regenerated by the invocations above at `87e2f03e0f5687692c638c17b66b92341c99534f`, from an
archive of its `tae/` and `selftest/` only; the reference's `party_monthly.py` there is its `3b55a03f`
(four commits after `1038dc05`: a party counted only on the other side of a non-invoice voucher, with a
not-attributed row and finding; the sets of excluded vouchers tried, every matching set listed; the
wording for vouchers netting to nil and for the all-together set; and PWM-2 checking the not-attributed
row on its own). The synthetic, `pm_paths` and `pm_not_fy` goldens change; `pm_empty`'s does not;
`pm_attribution` is new.

## Re-pin to reference `da9e2d3d` (2026-10-05)

The reference changed `party_monthly.py` in six commits after `3b55a03f` (`b4a84af1`, `750696e8`, `15f785a8`,
`c9dbd27f`, `927f4d56`, `01b6f5d8`, all dated 28 Sep 2026):

- The not-attributed row is now "Not attributed to a party", with three reasons, each with its own finding (ids
  `not_attributed_nil`, `not_attributed`, `not_attributed_money`) citing only its own vouchers and a count figure
  (`{block}_not_attributed_{reason}_vouchers`): the block's lines net to nil; no party on the other side (its
  expense example only in the expense blocks); and, in the expense blocks only, a party on the other side with any
  one configured cash or bank line on the lines' side (owner ruling 7b).
- A set of excluded vouchers explains a Trial Balance difference only when it matches exactly; the texts say
  "accounts for ... exactly", the several-matches text states its sign, and the no-match text lists the sets tried
  without "Neither ... nor".
- The module invariant takes the cash and bank ledgers `run()` takes (the reference's pack calls it outside its
  generic loop), checks the cash-or-bank and no-party rows each on its own, and checks each reason's count and
  finding (by GUID and label, as a multiset), and a not-attributed finding for a block the book does not carry.

`parity/edge_golden.py` and `parity/python_golden.py` now bind the cash and bank ledgers into `party_monthly`'s
check, as the pack passes them (the canonical dump calls `check_invariants(eng, result)`); at `da9e2d3d` the old
call raised for every `pm_*` book. No other runner changed: at `da9e2d3d`, every other edge book (97 books, 100
goldens) and every other test `python_golden.py` runs on the synthetic engagement (22 goldens; `entity_269st_gap`
raises the same refusal on that read either way) give byte-identical output through the harness before and after
the change, identical to their committed goldens.

- `golden/synthetic.party_monthly.json`, `golden/edge.pm_paths.party_monthly.json`,
  `golden/edge.pm_attribution.party_monthly.json` (regenerated): the row label, the per-reason findings and counts,
  the money reason for `pm_attribution`'s net receipt, exact matching and the new texts. `pm_not_fy`'s and
  `pm_empty`'s goldens regenerate byte-identical.
- `pm_money` and `pm_exact` (new, above). Each golden differs at `3b55a03f`, run there with this file's earlier
  `edge_golden.py` (the edited one cannot run there, where the reference's `check_invariants` takes two
arguments). The books are
  written as data by a small generator.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `pm_attribution.json` | 8,971 | `ef7a8b061543499edd2b033038072d83ebca889c04d6174d5abd0c77286a0b09` | `edge-books/pm_attribution.json` |
| `pm_empty.json` | 771 | `969cbcf93f17ef083503e5084c5f8666a618b27b315103bec5ea76e69d69670c` | `edge-books/pm_empty.json` |
| `pm_exact.json` | 5,944 | `075389ad07c9a15e06eab272ef742746b5c9526aa7e1ae2459dcc9836377cc05` | `edge-books/pm_exact.json` |
| `pm_money.json` | 8,116 | `3dc055855af40542ad6756aacd01876b275b20c498e41193999c1430bf9ef48e` | `edge-books/pm_money.json` |
| `pm_not_fy.json` | 1,990 | `effb4bf2ebfe8b437ddd582bc3f86176e4d00afd847dddf3f88b3cc2ed321276` | `edge-books/pm_not_fy.json` |
| `pm_paths.json` | 8,974 | `f2fca90e8226a64faf24acc22e36579f5a12fa2cbc2c65b437c2e090442e6ba5` | `edge-books/pm_paths.json` |
| `edge.pm_attribution.party_monthly.json` | 42,260 | `74b9bce82b8cf4665b2f9ef2e03c3e583d75e0acf8fa0a7d09524b8ff6b2be8b` | `golden/edge.pm_attribution.party_monthly.json` |
| `edge.pm_empty.party_monthly.json` | 6,715 | `406d20f5a48315ea0b10d0d3515186ec83a1ba63a9efb21a6871ef8f42410369` | `golden/edge.pm_empty.party_monthly.json` |
| `edge.pm_exact.party_monthly.json` | 23,192 | `d7c224e69024c97381b3f5318e667c3df0a047e2dc2917dea7af7ddbe25ebe65` | `golden/edge.pm_exact.party_monthly.json` |
| `edge.pm_money.party_monthly.json` | 30,027 | `bd5f1c0a60f9f88a32e8c02807b24274ba7863d4ee7a91b2cfef5687b0c83f65` | `golden/edge.pm_money.party_monthly.json` |
| `edge.pm_not_fy.party_monthly.json` | 9,463 | `ff81bbd99af3a63e625b9ecd0b9b42be743ff2b9ff1e26ca87c44115c19b7f4a` | `golden/edge.pm_not_fy.party_monthly.json` |
| `edge.pm_paths.party_monthly.json` | 44,577 | `a33211470cf9ea69b471f5571503004288c44bf38306e9323771aef8229e34b5` | `golden/edge.pm_paths.party_monthly.json` |
| `synthetic.party_monthly.json` | 24,934 | `a303cddf1f4d7782c006acd9a7c8cf368e044942bb0d956efb8cc0a5196ccf2c` | `golden/synthetic.party_monthly.json` |
