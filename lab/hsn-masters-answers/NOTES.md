# Masters-shape read of stock items and stock groups, and one created stock item with two dated HSN rows and two dated GST rate rows, on BRIDGE SHAPE LAB (Windows lab, 15 Oct 2026)

Synthetic company BRIDGE SHAPE LAB (100021) only, loaded alone. TallyPrime 7.1, Silver, on Windows. One run.
Item description, requests and checker: branch `lab/hsn-masters-run-sheet` at cee71e1a (README.md sha256
eb5fd20a3f74..., check.py cf3e3d422c1f..., every SHA256.txt line matched; `check-request-ids.py requests`:
`census: ok 4, skipped 0, FAIL 0`). 21 sends, all Export or GET /status. Times are UTC. Every check.py line
is in `check-lines.txt`; every curl line in `curl-lines.txt`.

## The three findings asked for first

- (a) Tally's own entries on the new item (verified). On the first save of the item (step 6), Tally added
  one HSN entry and one GST entry of its own, both dated 1-Apr-2025, source `As per Company/Stock Group`,
  with no code and no rate (`Tally's own entries: HSN 1 ['20250401'], GST 1 ['20250401']`). The history
  screens showed each as the one row dated 1-Apr-25. Keying the first HSN row (step 7) edited that HSN row;
  no own row was added when an HSN row was saved, either at step 7 or at step 8 (no own GST row dated
  22-Sep-2025 appeared). Keying the first GST row (step 9) edited the own GST row; after it no own entry
  was left.
- (b) What Tally filled in the other rate heads (verified, screen and answer). For IGST 18: CGST 9,
  SGST/UTGST 9, Cess valuation `Not Applicable`, State Cess without a rate. For IGST 5: CGST 2.50,
  SGST/UTGST 2.50, Cess valuation `Not Applicable`, State Cess without a rate. Nothing was typed in those
  heads. Valuation type of each filled head: `Based on Value`; state `Any` (sent with the reserved
  character printed `[#4]`).
- (c) The restore removed the created item (verified). After the restore the company list, book extent,
  stock-item and stock-group answers are each byte-identical to the first answers of part A; the stock-item
  answer has 11 rows and no row named `HSN Two Dates Test Item`.

## Steps

| step | what | result |
|---|---|---|
| 0 | one tally.exe owning 9001; no company loaded; BRIDGE SHAPE LAB loaded alone; current date Monday, 20-Apr-2026 | as expected |
| 1 | company list | PASS, 4,000 B |
| 2 | book extent | PASS, 3,838 B; ALTVCHID 112, ALTMSTID 289, company number 100021 |
| 3 | stock items, masters shape with both lists (NEW SIZE) | PASS, 19,450 B, 11 rows, none coded, GST content on `Soda Ash Light` only; COLLECTION `ISMSTDEPTYPE Yes, MSTDEPTYPE 512`; empty lists arrive as whitespace-only elements |
| 4 | stock groups | PASS, 6,730 B, 3 groups (Finished Kits, Packaging, Raw Chemicals), none coded; COLLECTION `MSTDEPTYPE 128`; empty lists whitespace-only |
| 5 | backup to a new empty local folder | one file TDBK1800_100021.001, 101,914 B, sha256 a1eccaf8abd76ddc5a27841c327bb4c90eeb332a619ebbe1710718a89d3fc782 |
| 5a | book extent again | identical |
| 6 | item created (Under Raw Chemicals, unit Kgs) | PASS: 12 rows, MASTERID 282; marks 289 -> 290 master, 112 voucher unchanged |
| 7 | HSN 28362010 from 1-Apr-2025 | PASS; master mark 291 |
| 8 | HSN 28362090 from 22-Sep-2025 | PASS; master mark 292 |
| 9 | GST Taxable, IGST 18 from 1-Apr-2025 | PASS; master mark 293 |
| 10 | GST Taxable, IGST 5 from 22-Sep-2025 | PASS: two HSN and two GST entries, 28,208 B (cap 40,000); master mark 294; groups unchanged |
| 12 | restore (Accept, then Replace on folder 100021) | completed; the company opened alone |
| 13 | the four reads again | each PASS; all four byte-identical to the first answers of steps 1 to 4 |
| 14 | current date and process picture | as at step 0; BRIDGE SHAPE LAB left loaded, as the item says |

The voucher mark stayed 112 throughout. `answers/capture-two-dated-entries-item.xml` is the same bytes as
`answers/ans-h03b5.xml` (step 11).

## Other observations

- After the save of the new item, Tally showed a blank creation screen; the one Escape the item allows
  raised `Quit ?  Yes or No`. The run stopped there, as the leaving rule says, and went on after the decision
  that `Yes` only closes the empty form: `Yes` returned to the Master Creation list with no other prompt,
  and the step-6 reads then showed the item exactly once. The item description is to be amended for
  this prompt.
- Tally showed `Maintain in batches: No` on the creation screen (not named by the item; left as shown).
- The GST rate history screen has columns the item does not name: GST Classification, slab-based rate,
  `Set/Alter GST Rate Details`, Reverse Charge, GST on MRP and Input Tax Credit; none was required and none
  was changed. Tally's defaults differed between the two rows: on the edited 1-Apr-2025 row `Eligible for
  Input Tax Credit` read No, and the answer carries `GSTINELIGIBLEITC Yes` in that entry; on the new
  22-Sep-2025 row it read Yes, and that entry carries no such element (verified on screen and in the answer).
- Tally filled HSN descriptions from its own list: `Disodium Carbonate, Dense` (28362010) and `Other`
  (28362090).
- One checker command was first given an argument the shell split (`--own-dates` as a list); it was run
  again on the same kept answer with the argument quoted, and passed both times. Nothing was sent twice.

## Files

`answers/` (21 answers, 21 status bodies, the kept capture), `requests/` (4), `check-lines.txt`,
`curl-lines.txt` (local folder paths replaced by `<item folder>` and `<Tally data folder>`), `SHA256.txt`.
Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`: screenshots, the backup, process and folder
listings, the original logs and the two helper scripts.

Not covered: a GST state row other than `Any`, an HSN code keyed at a stock group reaching its items, other
TallyPrime releases, a larger book.
