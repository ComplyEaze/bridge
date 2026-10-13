# A number carried by an import under Manual and under "Automatic (Manual Override)", duplicates allowed: new, back-dated, already held (#724, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 2 (its GUID is in every marks answer). TallyPrime 7.1,
Silver Rental, on Windows. One run, after the one on branch
`lab/724-m9k-manual-override-and-journal-answers`. No ComplyEaze Bridge build: six raw imports of one
numbered voucher each, each file sent once; raw reads by curl, one at a time; one save of Tally's own
Journal type screen. A backup was taken first and restored between the parts and at the end. Times in
send-log.txt are UTC.

## Method

- Reads: marks, series (24 types), the vouchers of July 2026. After every step: marks, vouchers and the
  table in `tables/`; a marks read just before each import; a series read at the start, after each
  part's last import, after the type save and after each restore. The method was read at series level.
- Part A, the Payment type as it stood (Manual, "Prevent creating duplicate Voucher Nos." No), no screen
  act: PA 17 Jul "M9L-P1"; PC 14 Jul "M9L-P2", dated before both Payments of the book; PB 18 Jul
  "M9L-P1" again.
- Part B, the Journal type set to "Automatic (Manual Override)": JD 24 Jul "M9L-J1"; JF 10 Jul "M9L-J2",
  dated before every Journal; JE 26 Jul "M9L-J1" again.
- Each imported voucher was found by type and date and read as its file (no narration is returned).

## Result

Marks = ALTVCHID / ALTMSTID. Every import answer read CREATED 1, the seven other counters 0, no LINEERROR.

| step | act | the voucher read | marks |
|---|---|---|---|
| l0 | as found | 12 vouchers; Payments "1" and "1"; Journals 1, 2, 3 | 12 / 235 |
| l1 | import PA | "M9L-P1", MASTERID 13 | 13 / 235 |
| l2 | import PC (back-dated) | "M9L-P2", MASTERID 14 | 14 / 235 |
| l3 | import PB (same number) | "M9L-P1", MASTERID 15 | 15 / 235 |
| l4 | restore | 12 vouchers; answers byte-identical to l0's | 12 / 235 |
| l5 | Journal method set to Manual Override | the three Journals blank, no ALTERID change | 12 / 237 |
| l6 | import JD | "M9L-J1", MASTERID 13 | 13 / 237 |
| l7 | import JF (back-dated) | "M9L-J2", MASTERID 14 | 14 / 237 |
| l8 | import JE (same number) | "M9L-J1", MASTERID 15 | 15 / 237 |
| End | restore | 12 vouchers; answers byte-identical to l0's | 12 / 235 |

- No voucher already in the book changed its number or ALTERID at any import; at l3 and l8 a second
  voucher held the number beside the first.
- Vouchers not of the type under test read as at l0 in every field at every step. In part A all 24
  series rows equalled l0's. At l5 the Journal series read `Automatic (Manual Override)`, `Auto Retain`,
  PREVENTDUPLICATES `No`, and the other 23 rows equalled l0's.

## One stop: a last-number entry in the Journal's series row

After l8 every series row was expected to equal l5's. The other 23 did; the Journal row differed in
one place: inside its series list, `RESTARTFROMLIST.LIST / LASTNUMBERLIST.LIST` was an
empty element at l5 and held DATE 20260401 and LASTNUMBER 0 at l8; method, sub-method and
PREVENTDUPLICATES read as at l5. The run paused there with nothing keyed and went on after the same
series request, sent once more, came back byte-identical (`answers/m9l-l8-series-repeat.xml`). After the
End restore the entry was gone. There was no series read at l6 or l7, so which import first wrote it is
not known. A reading, not a measurement: Tally wrote it once a voucher was created under this method.

## Prompts

The type save asked the two questions the earlier run's notes quote, on choosing the method and on
accepting, each answered Yes once. Three screen lines that depend on the method changed, as on that
run; no field but the method was changed. The backup asked "Accept ?"; each restore asked "Accept ?" and
then whether to replace the company data folder (Replace). No import raised anything on Tally's screen.

## Two slips

Neither touched the book or sent a request twice. A checking command printed one wrong line in the
working notes, withdrawn there. The run's helper misread the first series answer and reported a miss
that was not one; it was corrected and the kept answer judged again.

## What it decides

On this book and this release, with duplicates allowed: under Manual numbering (Payment) and under
"Automatic (Manual Override)" (Journal) an import's number was kept whether new, back-dated or already
held by another voucher, and no other voucher moved.

## Not covered

"Prevent creating duplicate Voucher Nos." set to Yes; a Journal under Manual; a number of digits only;
a numberless voucher under Manual; a batch; Gold; other releases; the same through
ComplyEaze Bridge's own post.

## Files

`requests/` (9), `answers/` (each with its headers), `tables/`, `derived/`, `tools/`,
`send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in
`SHA256-kept-on-lab-pc.txt`: screenshots, the backup, listings, the restore plan, the working log.
