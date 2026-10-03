# Fixture provenance: `read_scope`, `party_identity` and `entity_269st_gap` (3 Oct 2026)

Every book here is invented, with round figures and plain names and PANs and GSTINs that are made up
(none is a real registration): no fixture is a Tally read of any real assessee. A PAN that is only
compared as text is a token such as `PAN-BIGBY`. Every GSTIN has a fourteenth character other than
`Z`, which nothing here reads, so none has a real GSTIN's shape. A GSTIN's characters 3 to 12 keep a
PAN's or a TAN's shape where the reference derives a PAN from them or refuses to (the derivation
cases and the TAN case). Four recorded PANs keep a PAN's shape, each one letter five times, one digit
four times and that letter again: three in `ep_gap`, each of which must equal a PAN derived from a
GSTIN (the reference derives one only from a PAN-shaped segment), and the same derivable value in
`ep_gap_plain`, where without the opt-in it must not join the GSTIN's ledger.

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
  whether it is a party, and the reference raises rather than guess, which ends its whole pack. The
  port refuses only this test, with the typed code `PARTY-chain-incomplete` (a deliberate divergence:
  every other test still gives its result); the registry-wide test asserts that code in place of a
  golden.
- Regression fixtures only: they prove the port and the reference agree on the same inputs, and nothing
  about reading Tally. The evidence for real books is local parity on real reads, never committed.

## How they were produced

At the reference engine (a private repository), under Python 3.13. The three `read_scope` goldens were
made at commit `c62a4ab4` and regenerate byte-identical at its later commit `df4af35e` (checked 3 Oct
2026), whose only change under `tae/` and `selftest/` is `counter_cheques_40a3`'s invariants. The two
`entity_269st_gap` goldens were regenerated on 3 Oct 2026 at commit `da9e2d3d` (then the last change to
`tae/` and `selftest/`, a docstring), after the books' identifiers were rewritten as above; each equals
the golden made at `c62a4ab4` with the same identifier mapping applied, figure for figure and finding for
finding:

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
| `ep_gap.json` | 18,574 | `b744e4d9bb6c7818cd9530d06639e764b6fb56d55d8a96d3c9046a61261b06d5` | `edge-books/ep_gap.json` |
| `edge.ep_gap.entity_269st_gap.json` | 48,861 | `712811c66ee03c2da4a5c4f4d0b5091bc7ec8dee0e62f3b6c2ce4c275830f7b1` | `golden/edge.ep_gap.entity_269st_gap.json` |
| `ep_gap_plain.json` | 5,621 | `c13e6b8778f1eaf1a6fc1588f0016d553b44a5747663f6766651b91485eaae94` | `edge-books/ep_gap_plain.json` |
| `edge.ep_gap_plain.entity_269st_gap.json` | 5,959 | `6c81bcf8bd79e85428a492ed484fd1386cbdc08c000594c78128c1edcf3f9a39` | `golden/edge.ep_gap_plain.entity_269st_gap.json` |
| `rs_unread.json` | 811 | `d717dcbaa974133eb7968eb442cf9900a7bdcb1a50f4fd9a7c27082c587ab0c5` | `edge-books/rs_unread.json` |
| `edge.rs_unread.read_scope.json` | 1,976 | `54db49391f4e45ac8071208162926acd4d6ffe0c313d838ace457de88daf42f8` | `golden/edge.rs_unread.read_scope.json` |
| `rs_read.json` | 813 | `e7cf8ce8546bb7f9c4681c1a1ed782a9794fdb69ebd6a3a361d0c2a68dfc75b0` | `edge-books/rs_read.json` |
| `edge.rs_read.read_scope.json` | 913 | `1cd4cc938d16185b59b42b983d6b43277a77bbade7e9606e6b613d9c18662ff2` | `golden/edge.rs_read.read_scope.json` |
