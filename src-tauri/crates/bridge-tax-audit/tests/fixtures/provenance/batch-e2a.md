# Batch E2a fixture provenance: `bank_reconciliation`

Lane E, 2026-09-25. Every book and every statement row here is invented: no fixture is a Tally read or
a bank statement of any real assessee, and every account reference is a masked placeholder.

## What these fixtures establish, and what they do not

- `synthetic-bank-statement.json` is an invented statement for 2025-04-01..2025-05-31, in the shape
  `parity/python_golden.py --emit-bank-statement` writes. With the two `[roles]` keys this batch adds
  to `synthetic-engagement.toml` (`bank_reconciliation_ledger` and a lower-case
  `bank_charge_narration_terms`), `golden/synthetic.bank_reconciliation.json` is the reference's dump
  on the synthetic read: one match, a payment settled as two statement rows, a bank-only charge, an
  unexplained credit, an opening that ties and a closing that does not.
- `edge-books/bankrec_paths.json` reaches: an equal-gap tie decided by the lower statement index; a
  split each way; charge terms matched upper-cased (a lower-case term, and `Straße` upper-casing to
  `STRASSE`); both timing reasons; a voucher with two bank lines summed to one row; a zero-net and an
  optional voucher left out; a numberless voucher labelled by its GUID's last 12 characters; a
  narration over 60 characters, cut by code points; a balance break (BANK-1 on both sides) and a
  missing balance; and a window ending on the FY end (the Trial Balance closing is reported). That was at
  `1038dc05`; from `da9e2d3d` the reference's reader refuses this statement, and `bankrec_adds_up` reaches these
  paths (see the re-pin below).
- `edge-books/bankrec_big_pool.json` reaches the split search's bound: 41 candidate rows are not
  searched, and a window not ending on the FY end reports no Trial Balance closing.
- A repeated books GUID (two matched vouchers sharing one) is refused on both sides: the reference
  raises `duplicate figure id`, and `tests/edge_books.rs` checks that the port returns an error.
- Nothing here reads a real statement. A real statement is client data; it is emitted locally with
  `--emit-bank-statement` for local parity and never committed.

## How they were produced

At the reference engine (a private repository), commit `1038dc05` (re-pinned at `da9e2d3d`, below), under
Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.bank_reconciliation.json \
        --test bank_reconciliation --bank-statement tests/fixtures/synthetic-bank-statement.json
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/bankrec_NAME.json tests/fixtures/golden

## Re-pin to reference `da9e2d3d` (5 Oct 2026)

The reference changed `bank_reconciliation` and its statement reader (`adapters/bank_documents.py`) in seven
commits after `1038dc05`, all dated 26 Sep 2026:

- The statement is checked against itself before matching: its declared opening plus its rows' credits less debits
  against its declared closing (`statement_net_movement_paise`, `statement_arithmetic_diff_paise`), its first and last
  rows' balances where they carry one (`statement_first_row_tie_diff_paise`, `statement_last_row_tie_diff_paise`), and
  the row-to-row running balance (`statement_balance_chain_break_count`), each within Re 1. When any is off, no split
  is searched, every unmatched row on both sides is `not_computed_statement_does_not_add_up` (a new reason, so every
  dump gains its two pairs of reason figures), and the finding `statement_does_not_add_up` lists those rows; matched
  pairs stand.
- The reader refuses a statement that declares no opening or closing balance, one with a row outside its own period,
  and one whose dates step back in an order its running balance does not confirm (a back-valued row whose balance
  chains is read). The pack then gives `bank_reconciliation.refused`: one figure, the reader's reason, and one
  finding.

`parity/edge_golden.py` now reads each edge book's statement through the reference's own reader first (the same rows
in the extraction's shape), as the pack does, so a statement it refuses gives the refused result on both sides. That
changes `bankrec_paths`: its rows step back (03 Mar after 07 Mar) and one row carries no balance, so its golden is now
the refused result, and its comment now says so. Its earlier paths move to `bankrec_adds_up`.

One known divergence, failing closed: a statement whose period starts after it ends is a malformed document here (`period.start is after period.end`), checked before any refusal. The reference reads it, and refuses it as a row outside its period when it has rows. Neither emitter writes such a period.

- `golden/synthetic.bank_reconciliation.json`, `golden/edge.bankrec_big_pool.bank_reconciliation.json`
  (regenerated): both statements add up, so the new figures are added and nothing else changes; `bankrec_big_pool`'s
  rows step back with an intact running balance and are read.
- `golden/edge.bankrec_paths.bank_reconciliation.json` (regenerated): refused, the order with no running balance.
- `bankrec_adds_up` (new): `bankrec_paths`'s books on a statement in date order that adds up with every check exactly
  Re 1 off (first row +100, one running-balance step -100, the closing 100 over the last row and over the opening plus
  the rows), with rows on the period's first and last days; every earlier path of `bankrec_paths` is reached here.
- `bankrec_chain_break` (new): only the running balance is off (a dropped receipt with an earlier one repeated in its
  place); a would-be split, a would-be charge, a would-be not-found row and both timing reasons are not computed, and
  two matches stand.
- `bankrec_sum_off`, `bankrec_first_off`, `bankrec_last_off` (new): each with only one check off (the arithmetic with
  no row balance at all; the first row with the last row's balance absent; the last row with the first row's balance
  absent), so each check marks the statement on its own and the absent end ties are not reported.
- `bankrec_order_break` (new): refused, a back-step whose pair chains and a later break the reason names (after a step
  exactly Re 1 off); its closing is also absent, and the order refusal is the one given.
- `bankrec_outside_period` (new): refused, a row the day before the period (and one the day after, later); it also
  steps back with no balance and declares no opening, and the period refusal is the one given.
- `bankrec_no_balance` (new): refused, no opening balance (absent) and no closing (null); the opening is named.

Produced at `da9e2d3d` with the same commands as above. Each new book's golden differs at `1038dc05` (run there with
this file's earlier `edge_golden.py`): the five that run lack the new figures and reasons, and the three refused ones
raise there on the balance their statement does not declare. The `high_value_register` goldens regenerate byte-identical through
the reader step (no statement there is refused). The books are written as data by a small generator.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `bankrec_adds_up.json` | 7,557 | `8cddce6bbdc1ff95f7e8f406090ef7491559bdd8a5183d7e495be8056270fc10` | `edge-books/bankrec_adds_up.json` |
| `bankrec_big_pool.json` | 11,130 | `cfbc7323113363b0e7e81c193a4197be634812c5b261ed974110c9f28c73b324` | `edge-books/bankrec_big_pool.json` |
| `bankrec_chain_break.json` | 4,437 | `c02a612b64ca6a2ca757bd7934679d464b83b8149ab0e776f85ffafa0737dcd1` | `edge-books/bankrec_chain_break.json` |
| `bankrec_first_off.json` | 3,027 | `1c7df1aa5cf452826b3c7fe1bd3fa0b9ddc400383a0dbed30a0a2e4821a3a12a` | `edge-books/bankrec_first_off.json` |
| `bankrec_last_off.json` | 2,829 | `1689fe265165cb33fae30dad4da88c0f601e15b944d5f9aad6a3392b8de735f2` | `edge-books/bankrec_last_off.json` |
| `bankrec_no_balance.json` | 2,090 | `814d22455c310b6920f81b8cd6fef45d7dab4404ecbebf7b1ad6ef5f00a113c2` | `edge-books/bankrec_no_balance.json` |
| `bankrec_order_break.json` | 3,209 | `ab22fba1ec40f577eb7523c9a8b902280c200b07fb5a2e64da2f77cd3756b902` | `edge-books/bankrec_order_break.json` |
| `bankrec_outside_period.json` | 3,028 | `6908b5153e6201fa4787038b95cb1abb8cb7dad5130a665fb483a8353bea5989` | `edge-books/bankrec_outside_period.json` |
| `bankrec_paths.json` | 7,010 | `cac969fe0255f0623379a53a4e3bc4b778682f08a97b6ac8955d6250a76568be` | `edge-books/bankrec_paths.json` |
| `bankrec_sum_off.json` | 2,617 | `cb5dce496a71a2bfbcf9746b153a75218cb0d2cf35d0331f2a9b92a212d55aea` | `edge-books/bankrec_sum_off.json` |
| `edge.bankrec_adds_up.bank_reconciliation.json` | 23,392 | `8d0770569642070b816b501e3ce2a9f1c026a83c47b235e0176bc37979ebb252` | `golden/edge.bankrec_adds_up.bank_reconciliation.json` |
| `edge.bankrec_big_pool.bank_reconciliation.json` | 22,269 | `47e5e90f1069d2ff7cfa2675e01b169e2cfa4eee68557035c7263fc9ab138fdf` | `golden/edge.bankrec_big_pool.bank_reconciliation.json` |
| `edge.bankrec_chain_break.bank_reconciliation.json` | 23,253 | `1bc3e502617c4c82f3bb1c50560514845fe7bd7c496322a7ce72d8a1113b6d7e` | `golden/edge.bankrec_chain_break.bank_reconciliation.json` |
| `edge.bankrec_first_off.bank_reconciliation.json` | 20,956 | `255a073cf2df6a3bc62c09f624498304a23724a3507fce7134afb82dbb38a904` | `golden/edge.bankrec_first_off.bank_reconciliation.json` |
| `edge.bankrec_last_off.bank_reconciliation.json` | 20,649 | `d9fd27df0b5cebde047c998903012608c8e3e3802e3c9fe8ce36e689f86096ac` | `golden/edge.bankrec_last_off.bank_reconciliation.json` |
| `edge.bankrec_no_balance.bank_reconciliation.json` | 2,390 | `d19c9e2ef8b816dec04e5e79b5d37a7a7ad288ea7214adfb3b6459bf774d755b` | `golden/edge.bankrec_no_balance.bank_reconciliation.json` |
| `edge.bankrec_order_break.bank_reconciliation.json` | 2,542 | `dc49ebf3a1c4a43008887badcf19559f9e767afa299c55106d99efdd8c315895` | `golden/edge.bankrec_order_break.bank_reconciliation.json` |
| `edge.bankrec_outside_period.bank_reconciliation.json` | 2,451 | `9e27d77b74b2ff5e0a529fa9cf86b1dd0d091d3390d9372664ed3e3cbefea90e` | `golden/edge.bankrec_outside_period.bank_reconciliation.json` |
| `edge.bankrec_paths.bank_reconciliation.json` | 2,632 | `6a673ddbbfe20e8bae0614ccc82b18bb83972aaf88702b033438cb36c4b6fd2c` | `golden/edge.bankrec_paths.bank_reconciliation.json` |
| `edge.bankrec_sum_off.bank_reconciliation.json` | 19,941 | `900108eac9f9aa24831e3359dc3db7b05dc70964f6df8a82d7fdd685a1fa0743` | `golden/edge.bankrec_sum_off.bank_reconciliation.json` |
| `synthetic-bank-statement.json` | 1,368 | `8ba3bf7d8609e7b578feb90f43832c4ce4a2319f1f1d6b9f47b779357db62c07` | `synthetic-bank-statement.json` |
| `synthetic.bank_reconciliation.json` | 20,808 | `ee0ddf7e96f1ac4099c7034206bec93c8cabc2d7c461f2ce1adc43f3f8c27bf2` | `golden/synthetic.bank_reconciliation.json` |
