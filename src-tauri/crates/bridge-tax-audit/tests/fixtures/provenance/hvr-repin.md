# Fixture provenance: the `high_value_register` re-pin (2 Oct 2026)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What changed in the reference, and what these fixtures establish

The last pin of `high_value_register` (reference commit `140bc7d3`, 26 Sep 2026) grouped the vouchers of a day that carry no party ledger under one row but still wrote the row as one party ("one unidentified party"). The reference has since stopped asserting a person for that row: it pools every such voucher of the day (or of a voucher reference) under one row whatever name the vouchers print, states the row in its own terms (the payers' or payees' side, "vouchers that do not name the payer", whether any one person reached the limit is "not known"), and cites on the row every usable printed name. It also reports why a supplied bank statement was not used when the reader refused it. The measured differences in the three goldens already pinned are the unnamed rows' figure definitions, titles, limits and asks only; no figure value, no tag and no other finding changes (`hvr_bare` is byte-identical).

- `golden/synthetic.high_value_register.json` and `golden/edge.hvr_paths.high_value_register.json` (regenerated): their two unnamed rows (one receipt, one payment) take the new wording.
- `hvr_unnamed` (new): every way a printed name can be usable or not, on one pooled receipt row (a name with surrounding blanks, a name containing " or ", a name with an apostrophe (quoted as written), a non-ASCII name, a name ending in a Python-whitespace control character, a name that is the voucher's own Sales ledger, the cash ledger or the bank ledger, a blank name, and a name made only of control whitespace; the label quotes and sorts the usable ones and says vouchers printing none were pooled); a row whose every voucher prints one usable name; a row proven under the limit on its cash line (an unnamed receipt, payment and bank row, and a named receipt, payment and bank row, so the named title is pinned too), one whose party side differs from its line without being under it (a receipt and a payment), a bank row over the threshold, a named party's row beside an unnamed one on the same day, limb (b) rows for a shared reference citing the printed names (receipts, with one voucher printing none; payments, with two vouchers printing the same name), and a supplied statement that the reader refused (`bank_statement_refused`, with no statement) so s.194N's coverage says so.
- Regression fixtures only: they prove the port and the reference agree on the same inputs, and nothing about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `c62a4ab4`, under Python 3.13. The goldens below also regenerate byte-identical at its later commit `da9e2d3d` (checked 3 Oct 2026), the last change to its `tae/` and `selftest/` that day; the changes after `c62a4ab4` touch only `counter_cheques_40a3`:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.high_value_register.json \
        --test high_value_register --bank-statement tests/fixtures/synthetic-bank-statement.json \
        --traces-documents tests/fixtures/synthetic-traces-documents.json
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/hvr_NAME.json tests/fixtures/golden

`hvr_unnamed.json` is written by hand in a small generator, as data, then read by both sides. The byte rows of the two regenerated goldens stay in `batch-e2b.md`, updated in place.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `hvr_unnamed.json` | 8,222 | `911c1cfb2e121971a56ac9ee5513ec57cbffc540ae8a9361e0363ad292fdd3cf` | `edge-books/hvr_unnamed.json` |
| `edge.hvr_unnamed.high_value_register.json` | 94,997 | `f88799e379bf55c9c63a8007659e242167dd64e127e373df4cca9a08fd527a16` | `golden/edge.hvr_unnamed.high_value_register.json` |

## Declared behaviour difference: a bank row says the money moved by bank, and a pooled row says what it sums

12 November 2026, #772. `docs/tax-audit/parity-spec-v1.md` section 12 requires every change to an
existing output to be a declared difference that names the invented book which exercises it.

- **What differs.** Four texts and one change of how a text is matched, each on rows the register already wrote, with no change to a figure's value, a tag, a clause or a row:
  - A bank row's title said "Bank received" or "Bank paid", as though the bank acted. It now says "Received into the bank" or "Paid from the bank", on a row with no party ledger and on a named row proven under the threshold on its bank line. A cash row's title is unchanged.
  - A bank row with no party ledger asked whether any one person reached the CA-set vouching threshold, and its title said whether any one person paid that much is not known. That threshold is not a test per person, so the title now ends at the total being at or over the threshold, and the ask is who paid, or was paid, on each voucher, with each counterparty's name and PAN. The cash row keeps both.
  - The row ref of a bank row with no party ledger was labelled with the bucket's "cash leg" wording. It is now labelled "‹bank leg with no identified party›". Its id, and the row's key and tag, are unchanged.
  - A limb (b) row with no party ledger defined its amount as "from/to one party ledger". It now says it sums the payers' or payees' side of vouchers with no party ledger sharing the reference, whatever name they print.
  - A pooled cash row drops the one-party ask by the same constants the named row asks it with, so rewording that ask can no longer bring it back onto a pooled row. Its output is unchanged.
  The compared fields are `title_sha256_16`, `ask_client_sha256_16`, `definition_sha256_16` and the evidence `label` (parity-spec sections 2.2 and 3).
  Still open on #772: a limit of a row whose bank line differs from its party side says "the bank received on them" or "the bank paid to this party on them", the same wording these titles leave.
- **The books.** The unit-test book of `src/high_value_register.rs`: the test `high_value_register::tests::a_bank_row_names_the_bank_as_the_way_money_moved_and_a_pooled_row_says_what_it_sums` asserts each text above whole, the label of a bank and a cash row, the asks of a pooled and a named cash row, and the limb (b) definition. `edge-books/hvr_unnamed.json` reaches every one too.
- **The golden.** `golden/edge.hvr_unnamed.high_value_register.json` carries the new texts on its five bank rows (two with no party ledger proven under the threshold, one with no party ledger over it, two named rows proven under it) and its two limb (b) rows with no party ledger, with each hash recomputed with the hash `canonical.rs` uses. Nothing else in it changed, and no other `high_value_register` golden holds such a row. Its byte row above is updated in place.
- **Why it was not regenerated, and what a local parity run shows.** The reference still gives the old texts (inferred from the golden, not measured), so a local parity run of `high_value_register` on a real book reports these fields of such rows as differences. These are this declared difference, not a regression.
