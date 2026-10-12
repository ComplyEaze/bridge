# Fixture provenance: partners_40b_194t vouchers that share a GUID (#1243)

Every book here is invented, with round figures and plain names: no fixture is a Tally read of any real assessee.

## What these fixtures establish, and what they do not

The reference changed, at its commit `0b462e3f`, how `partners_40b_194t` keys a partner's vouchers. Before it, the
walk kept every per-voucher map (the interest and remuneration credits, and the vouchers each note and citation names:
off the capital, not of the plain shape, TDS shared, a credit not read, interest and remuneration not split, the
capital set off) and the TDS seen on a partner's vouchers by GUID alone, so two vouchers sharing a GUID (a blank GUID,
or one GUID repeated) were one: the gross s.194T base netted a reversal against a credit that shared its GUID, the TDS
seen and every count of vouchers in a limit read one voucher for two, and a finding cited one. Now each voucher of the
population has a key no other shares (its GUID where that is unique, so nothing that reads it moves, else the GUID and
its place among those sharing it); a citation still names a voucher by its GUID and label, distinct refs in (GUID,
label) order. At the reference's previous commit (`e20e1d52`) each of the three goldens below differs.

- `edge-books/partners_shared_guid_credits.json`: partner A's three blank-GUID interest credits and a blank-GUID
  reversal, and one GUID on a remuneration credit and its partial reversal; partner B's one GUID on an interest reversal
  and on a voucher split exactly into interest and remuneration. The s.194T base counts each credit gross (Rs 51,000 and
  Rs 32,000, where merging gave Rs 36,000 and Rs 22,000), each reversal is named and each voucher cited. 24 figures, 5
  findings.
- `edge-books/partners_shared_guid_tds.json`: two blank-GUID interest credits each carrying TDS added back gross, two
  blank-GUID interest vouchers and one GUID on two remuneration vouchers whose TDS is shared with a bank credit (TDS seen
  on six vouchers, Rs 10,000, where merging gave two and Rs 2,000); two blank-GUID vouchers on an interest ledger two
  partners share, paid from the bank with TDS (off-capital, and TDS attributed to no one, on two vouchers for each). 34
  figures, 9 findings.
- `edge-books/partners_shared_guid_shapes.json`: one partner with two vouchers of each shape under one GUID per shape
  (blank for two): off-capital, interest and remuneration reversals, not split with the capital also debited, the
  capital set off, a credit to a payable this test does not read. Every count of vouchers says 2. 14 figures, 4
  findings.
- Not reached: the reference's `s194t_tranches` (the day each credit becomes deductible, read by `tds_interest_201`) is
  not ported, so the tranche dating the reference's change also corrects has no crate output to compare. A GUID holding
  a NUL, which the reference refuses once a partner is configured (and reads when none is), is a unit test in
  `src/book.rs` and one in `src/partners_40b_194t.rs`, not a book.
- Regression fixtures only: they prove the port and the reference agree on the same book, and nothing about reading
  Tally.

## How they were produced

At the reference engine (a private repository), commit `5cba91e899a8b7ef62ff3154034666b44ef7d07f`, the commit after
`0b462e3f634c5e51176a7c19577908fa78598052` (between the two only `s194t_tranches` changes the shape of a tranche, which
`run` does not read), from an archive of its engine source only, under Python 3.13:

    uv run -q --python 3.13 --with openpyxl --with xlrd --with python-docx --with jsonschema \
        --with striprtf --with pdfplumber python parity/edge_golden.py ENGINE \
        tests/fixtures/edge-books/NAME.json tests/fixtures/golden

At the reference's main of the same day (`5658c8ce70d57981df63c5e65172308bf166ef94`) the `tds` golden regenerates
byte-identical, and the `credits` and `shapes` goldens differ only in the reworded limit for a negative net credit
(reference `5db144ff`), with that limit's hash. The six partners goldens already pinned regenerate byte-identical at
`5cba91e8` (checked 6 Oct 2026).

When the crate took that rewording, the `credits` and `shapes` goldens were regenerated at `5658c8ce`, where the
`tds` golden is unchanged; their rows below are from there.

The three books are written as data by a small generator, every voucher balancing and each Trial Balance row the
opening plus the vouchers; they are hand-chosen scenarios, not generated from any data.

## Bytes

| File | Bytes | SHA-256 | Path |
| --- | ---: | --- | --- |
| `partners_shared_guid_credits.json` | 5,133 | `13186f8f4800e93f3e1939073a80837fdf9add98b3ca43281703b3c2b50efa6f` | `edge-books/partners_shared_guid_credits.json` |
| `partners_shared_guid_tds.json` | 5,853 | `05efeadd881a3f4c9099bf12229f88dffcf8ef7cf87e7d3681a48523575e5f14` | `edge-books/partners_shared_guid_tds.json` |
| `partners_shared_guid_shapes.json` | 5,468 | `01c6826899bdd825e542603941b7102d112ed0bea03368a727c57bbca39bc882` | `edge-books/partners_shared_guid_shapes.json` |
| `edge.partners_shared_guid_credits.partners_40b_194t.json` | 26,161 | `506684f853781683fd2ea92cb3868ae1636cebd412e45950f24e038f331b7a08` | `golden/edge.partners_shared_guid_credits.partners_40b_194t.json` |
| `edge.partners_shared_guid_tds.partners_40b_194t.json` | 40,798 | `a893736d807f730205b7b5fbde7a22aa7bfc35a32c16098f9106a8514895ae41` | `golden/edge.partners_shared_guid_tds.partners_40b_194t.json` |
| `edge.partners_shared_guid_shapes.partners_40b_194t.json` | 23,639 | `f4b7a19d4b8491d6bd1be87b603332be8d2c0116b3f3a265484f06e3541b52c8` | `golden/edge.partners_shared_guid_shapes.partners_40b_194t.json` |
