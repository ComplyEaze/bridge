# Results: LR4 (#1602 part a), stock item opening quantity, value and rate as Tally sends them (21 Oct 2026)

Windows lab PC, TallyPrime 7.1 Silver (licensed), port 9001. The four requests (MANIFEST.sha256 4/4 OK) were sent one at a time by plain HTTP POST, each with its company loaded alone. Answers are saved as received (BOM-less UTF-16LE) with their response headers in `answers/`; every send, HTTP status, size and sha256 is in `send-log.txt`; `ANSWERS.sha256` lists every added file (sha256, bytes, path). Nothing was keyed, so there was no restore.

Voucher marks before and after each read came from a plain-HTTP marks read (`Bridge Agent Company High Water`, GUID/ALTVCHID/ALTMSTID). For PILOT LAB it is `r02-marks.xml` of lab/1641-rows-run-sheet as is. For SHAPE LAB and TIEOUT B the lab made copies in `lab-requests/` with only SVCURRENTCOMPANY changed. For TIEOUT C the marks came from the Bridge `ledger_masters` snapshot (0.6.0 extension); the same read was refused on SHAPE LAB with `company_several_currency_masters`, so the plain read was used there. Every mark was the same before and after its read.

| Request | Company (number) | HTTP | Bytes | sha256 | Items | Marks (voucher / master) |
|---|---|---|---|---|---|---|
| LR4-stock-opening-pilot-lab.xml | BRIDGE PILOT LAB (100033) | 200 | 2,998 | cf34fa0a… | 0 | 68 / 245 |
| LR4-stock-opening-tieout-c.xml | BRIDGE TIEOUT C (100069) | 200 | 4,418 | ca5a76f0… | 1 | 3 / 213 |
| LR4-stock-opening-shape-lab.xml | BRIDGE SHAPE LAB (100021) | 200 | 17,582 | d0d99bc9… | 11 | 112 / 292 |
| LR4-stock-opening-tieout-b.xml | BRIDGE TIEOUT B (100040) | 200 | 7,148 | f93a4dc3… | 3 | 3 / 218 |

PASS: every answer was saved and the counts equal R01's NUMSTOCKITEMS (1, 11, 3, 0). Counts are of `COLLECTION/STOCKITEM` elements; the `CMPINFO/STOCKITEM` counter (0) is not an item.

## Shape of the three opening fields (exact text)
In every row all three tags are present. An empty quantity or rate is a present-empty tag (`<OPENINGBALANCE TYPE="Quantity"></OPENINGBALANCE>`, `<OPENINGRATE TYPE="Rate"></OPENINGRATE>`), never an absent one. The value is always present, as `0.00` when there is no opening.

- TIEOUT C, Lab Gadget (parent the reserved Primary, unit Nos): OPENINGBALANCE ` 10 Nos`, OPENINGVALUE `-1000.00`, OPENINGRATE `100.00/Nos`.
- TIEOUT B, Lab Widget A, B and C (Primary, Nos): OPENINGBALANCE empty, OPENINGVALUE `0.00`, OPENINGRATE empty, for all three.
- SHAPE LAB (quantity text has a leading space where positive):
  - Carton Box Small (Packaging, Box): ` 100 Box`, `2500.00`, `25.00/Box`
  - Caustic Soda Flakes (Raw Chemicals, Kgs): ` 400.000 Kgs`, `-1000.00`, `2.50/Kgs`
  - Cleaning Kit A (Finished Kits, Nos): ` 12 Nos`, `18750.00`, `750.00/Nos`
  - Cleaning Kit B (Finished Kits, Nos): empty, `0.00`, empty
  - Descaler Concentrate (Finished Kits, Kgs): empty, `0.00`, empty
  - HDPE Drum 50L (Packaging, Nos): ` 40 Nos`, `12000.00`, `300.00/Nos`
  - Hydrochloric Acid 33pc (Raw Chemicals, Kgs): ` 816.667 Kgs`, `-15500.00`, `18.98/Kgs`
  - Label Roll (Packaging, Nos): empty, `0.00`, empty
  - Soda Ash Light (Raw Chemicals, Kgs): ` 1300.000 Kgs`, `-18749.99`, `13.89/Kgs`
  - Sulphuric Acid 98pc (Raw Chemicals, Kgs): ` 100.000 Kgs`, `5000.00`, `50.00/Kgs`
  - Zero Stock Item (Raw Chemicals, Kgs): `-50.000 Kgs`, `0.00`, empty
- Observation, not interpretation: OPENINGVALUE carries both signs in one book (positive on four items, negative on three), and one item has a negative opening quantity with a zero value and no rate.
- PILOT LAB (no stock items): the envelope is a normal Export answer with `CMPINFO` and an empty `<COLLECTION ISMSTDEPTYPE="Yes" MSTDEPTYPE="512"></COLLECTION>` under `DATA`: an empty COLLECTION element, not a missing one and not an empty ENVELOPE.

The optional screen check (Stock Item alteration screens) was not done.
