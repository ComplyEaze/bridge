# A voucher imported with no number under Manual numbering: Contra and Receipt, a second one, two in one file (#724, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 2 (its GUID is in every marks answer). TallyPrime 7.1,
Silver Rental, on Windows. One run, after the one on branch
`lab/724-m9l-held-and-backdated-number-answers`. No ComplyEaze Bridge build: five raw imports, none
carrying a VOUCHERNUMBER element, each file sent once; raw reads by curl, one at a time; two saves of
Tally's own voucher type screens. A backup was taken first and restored between the parts and at the
end. Times in send-log.txt are UTC.

## Method

- Reads: marks, series (24 types), the vouchers of July 2026. After every step: marks, vouchers and the
  table in `tables/`; a marks read just before each import; a series read at the start, after each type
  save, after every import and after each restore, compared by program, row by row, with the read
  before it. The method was read at series level.
- Part A: the Contra type, which held no voucher, set to Manual. CA 11 Jul 81.00; CB 12 Jul 82.00; CC,
  one file of two, 14 Jul 83.00 first and 13 Jul 84.00 second.
- Part B, after a restore: the Receipt type, which held seven vouchers numbered 1 to 7, set to Manual.
  RA 27 Jul 85.00; RB 28 Jul 86.00.
- Each imported voucher was found by type and date and read as its file (no narration is returned).
- The comparer was first tried on the earlier run's kept answers.

## Result

Marks = ALTVCHID / ALTMSTID. Every import answer read CREATED equal to the vouchers of its file, the
seven other counters 0, no LINEERROR.

| step | act | the voucher read | marks |
|---|---|---|---|
| m0 | as found | 12 vouchers, no Contra; Receipts "1" to "7" | 12 / 235 |
| m1 | Contra method set to Manual | the type holds none | 12 / 237 |
| m2 | import CA | "1", MASTERID 13 | 13 / 237 |
| m3 | import CB | "1", MASTERID 14 | 14 / 237 |
| m4 | import CC (two) | 14 Jul "1", MASTERID 15; 13 Jul "1", MASTERID 16 | 16 / 237 |
| m5 | restore | 12 vouchers; answers byte-identical to m0's | 12 / 235 |
| m6 | Receipt method set to Manual | the seven Receipts blank, no ALTERID change | 12 / 237 |
| m7 | import RA | "1", MASTERID 13 | 13 / 237 |
| m8 | import RB | "1", MASTERID 14 | 14 / 237 |
| End | restore | 12 vouchers; answers byte-identical to m0's | 12 / 235 |

- Every numberless voucher read the number "1": the first of its type, the second, both of one file,
  and the two Receipts that joined seven whose numbers the type save had emptied. None read blank.
- At m6 each of the seven Receipts kept its ALTERID and VOUCHERRETAINKEY; its VOUCHERNUMBER element was
  present and empty and its NUMBERINGSTYLE read Manual. They stayed so at m7 and m8.
- VOUCHERRETAINKEY of the new vouchers: Contras 1, 5, 9, 13; Receipts 29, 33 (the seven hold 1 to 25
  in steps of four).
- No voucher already in the book changed at any import. Vouchers not of the type under test read as at
  m0 in every field at every step.
- Series: at each type save only that type's row changed, in two places: NUMBERINGMETHOD Automatic to
  Manual, and ALTERID (Contra 231 to 237, Receipt 235 to 237); sub-method `Auto Retain`,
  PREVENTDUPLICATES `No`. After each import all 24 rows equalled the read before it: no last-number
  entry appeared under Manual (one had appeared under "Automatic (Manual Override)" on the earlier
  run). Between such reads the answers were of one size but not byte-identical; the difference lies
  outside the named type rows; not looked into.

## Prompts

The Contra type save asked nothing. The Receipt type save asked the two questions the earlier runs'
notes quote, on choosing the method and on accepting, each answered Yes once. On both screens the
lines that depend on the method changed and no other; "Prevent creating duplicate Voucher Nos." was
offered as No and left. The backup asked "Accept ?"; each restore asked "Accept ?" and then whether to
replace the company data folder (Replace). No import raised anything on Tally's screen.

## What it decides

On this book and this release, with duplicates allowed: under Manual numbering an imported voucher
that carries no number is created and numbered "1", whether its type held no voucher (Contra) or seven
emptied ones (Receipt), and whether it is the first, the second, or one of two in a file. A second one
repeats the number.

## Not covered

"Prevent creating duplicate Voucher Nos." set to Yes; a back-dated numberless voucher; a file of more
than two; Payment and Journal by a raw import; Multi-user Auto; a deletion; Gold; other releases; the
same through ComplyEaze Bridge's own post. A sixth file, a control for a refused first import, was not
sent.

## Files

`requests/` (8), `answers/` (each with its headers), `tables/`, `derived/`, `tools/`,
`send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in
`SHA256-kept-on-lab-pc.txt`: screenshots, the backup, listings, the restore plan, the working log.
