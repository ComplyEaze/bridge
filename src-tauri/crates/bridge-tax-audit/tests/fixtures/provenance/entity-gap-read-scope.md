# Fixture provenance: `read_scope`, `party_identity` and `entity_269st_gap` (3 Oct 2026)

Every book here is invented, with round figures and plain names and PANs and GSTINs that are made up
(none is a real registration): no fixture is a Tally read of any real assessee.

## What these fixtures establish

- `read_scope` says what the read covers. The reference's `Book.currency_read` is never set by any
  reader, so on every read it gives the figure `currency_read = no` and one finding asking the client
  to confirm the books are kept in rupees. `golden/synthetic.read_scope.json` is that on the synthetic
  read; `rs_unread` and `rs_read` are the two values of the flag on an invented book (the second has no
  finding).
- `entity_269st_gap` re-aggregates the same population `cash_payments_40a3` tests per ledger (cash
  received, by ledger and day) by the PAN each ledger carries, and reports only the person-days where
  the aggregate reaches the s.269ST(a) limit and no single ledger does. `ep_gap` reaches: a gap row
  whose ledger names share a word and one whose names share none; a PAN one of whose ledgers already
  reaches the limit (left to the per-ledger test); an aggregate under the limit; a PAN derived from a
  GSTIN under the engagement's opt-in, and a TAN-shaped GSTIN that gives none; a PAN filled by the
  engagement's override and a master PAN that wins over a different override; a ledger with no PAN
  (unbound); an entity with one ledger; an excluded ledger, a Duties & Taxes ledger and a round-off
  ledger, none a party; a ledger made a party by name; a walk-in sale with only tax and round-off lines;
  a Contra and an optional voucher left out; and, in the names, a shared stopword only, a shared two-letter word only, a difference of case and punctuation, an accented spelling, an override name deciding the disclosure, a GSTIN filled by an override and a master GSTIN winning over one, a ledger made a party by a configured group, a ledger the table names as round-off, an incomplete chain that is already settled, an aggregate of exactly the limit (a gap row) and a single ledger of exactly the limit beside another (already reported, so left out). `ep_gap_plain` is the same without a `party_identity`
  table.
- **No synthetic golden for `entity_269st_gap`.** The reference ends on the synthetic read with
  `IncompleteLedgerChain`: the read has a ledger whose group chain is incomplete and does not settle
  whether it is a party, and the reference refuses the whole test rather than guess. The port refuses
  with the typed code `PARTY-chain-incomplete`; the registry-wide test asserts that code in place of a
  golden.
- Regression fixtures only: they prove the port and the reference agree on the same inputs, and nothing
  about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), commit `c62a4ab4`, under Python 3.13. All five goldens
regenerate byte-identical at its later commit `df4af35e` (checked 3 Oct 2026), whose only change under
`tae/` and `selftest/` is `counter_cheques_40a3`'s invariants:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/python_golden.py ENGINE \
        tests/fixtures/synthetic-engagement.toml tests/fixtures/golden/synthetic.read_scope.json \
        --test read_scope
    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

The edge books are written by hand in a small generator, as data, then read by both sides.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `synthetic.read_scope.json` | 2,532 | `8b191e04fdf6722106c6082a722cb7a2bb5e95308b2138701dc1b32f57ba26b5` | `golden/synthetic.read_scope.json` |
| `ep_gap.json` | 18,612 | `02531f5aeb0524ea4885598a9d5d9c9fad60db1790f7062e41b1dd420b9ab7ee` | `edge-books/ep_gap.json` |
| `edge.ep_gap.entity_269st_gap.json` | 48,921 | `2234933cce1a0b0fd3fdbc8826436f1dc13e3459388ec6e21fde3a52f77b3fc4` | `golden/edge.ep_gap.entity_269st_gap.json` |
| `ep_gap_plain.json` | 5,638 | `b632cb6e251672e0bfdf23df7a97f6f70a66c5922f7b789d3bbf641f91efd5dc` | `edge-books/ep_gap_plain.json` |
| `edge.ep_gap_plain.entity_269st_gap.json` | 5,964 | `8cbda182132b76cabfe531de917cf685cb51d49a7272790866a27806b8b3ffec` | `golden/edge.ep_gap_plain.entity_269st_gap.json` |
| `rs_unread.json` | 811 | `d717dcbaa974133eb7968eb442cf9900a7bdcb1a50f4fd9a7c27082c587ab0c5` | `edge-books/rs_unread.json` |
| `edge.rs_unread.read_scope.json` | 1,976 | `54db49391f4e45ac8071208162926acd4d6ffe0c313d838ace457de88daf42f8` | `golden/edge.rs_unread.read_scope.json` |
| `rs_read.json` | 813 | `e7cf8ce8546bb7f9c4681c1a1ed782a9794fdb69ebd6a3a361d0c2a68dfc75b0` | `edge-books/rs_read.json` |
| `edge.rs_read.read_scope.json` | 913 | `1cd4cc938d16185b59b42b983d6b43277a77bbade7e9606e6b613d9c18662ff2` | `golden/edge.rs_read.read_scope.json` |
