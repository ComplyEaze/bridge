# "Automatic (Manual Override)" and "Renumber Vouchers" on the Journal type: what an import keeps, gets and moves (#724, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 2 (its GUID is in every marks answer), the Journal
voucher type only. TallyPrime 7.1, Silver Rental, on Windows. One run, after the runs on branches
`lab/724-m9i-import-renumber-answers` and `lab/724-m9j-batch-and-numbered-answers`. No ComplyEaze Bridge
build: three raw imports of one Journal each in the shape its builder sends, each file sent once; raw
reads by curl, one at a time; three saves of Tally's own Journal type screen. A backup was taken first
and restored after part A and at the end. Times in send-log.txt are UTC.

## Method

- The reads are those runs' request files: marks, series (24 types), the vouchers of July 2026. After
  every step: marks, vouchers and the table in `tables/`, and a series read after each type save and each
  restore; one more marks read just before each import. The method was read at series level.
- The imports: JA, 24 Jul, no number; JB, 26 Jul, carrying the number "M9K-J1"; JC, 19 Jul, no number.
  Each read as its file; the read returns no narration.

## Result

Journals as date:number; marks = ALTVCHID / ALTMSTID. No Journal's ALTERID changed at any step.

| step | act | Journals after it | marks |
|---|---|---|---|
| k0 | as found: Automatic, Retain Original Voucher No. | 18 Jul:3, 20 Jul:1, 22 Jul:2 | 12 / 235 |
| k1 | method set to Automatic (Manual Override) | all three numbers blank | 12 / 237 |
| k2 | import JA (no number) | it read blank; nothing else moved | 13 / 237 |
| k3 | import JB (numbered) | it read "M9K-J1"; nothing else moved | 14 / 237 |
| k4 | restore | 12 vouchers; answers byte-identical to k0's | 12 / 235 |
| k5 | behaviour set to Renumber Vouchers | date order: 18 Jul:1, 20 Jul:2, 22 Jul:3 | 12 / 237 |
| k6 | import JC (19 Jul, no number) | it read "2"; 20 Jul went 2 to 3, 22 Jul 3 to 4 | 13 / 237 |
| k7 | behaviour set back to Retain | entry order: 20 Jul:1, 22 Jul:2, 18 Jul:3, 19 Jul:4 | 13 / 239 |
| End | restore | 12 vouchers; answers byte-identical to k0's | 12 / 235 |

- k1: a blank number came back as an empty VOUCHERNUMBER element; VOUCHERRETAINKEY stayed 1, 5, 9.
  The series read `Automatic (Manual Override)`, `Auto Retain`, PREVENTDUPLICATES `No`.
- Every import answer read CREATED 1 with ALTERED 0 and every other counter 0; ALTVCHID rose only by
  the vouchers created.
- The nine vouchers that are not Journals, and the other 23 types' whole series rows, read as at k0 at
  every step.

## One stop, and the two screen lines behind it

At k1, after the method was chosen and the first question answered, two lines of the type screen
differed from the screen as opened, both set by Tally: "Numbering behaviour on insertion/deletion" was no
longer shown and "Prevent creating duplicate Voucher Nos." stood in its place with No; and "Show unused
vch nos. in transactions for Retain Original Voucher No. behaviour" had gone from Yes to greyed out,
reading No. The item allowed only a newly offered field to differ, so no key was pressed and the run
paused with the screen open. It went on once both lines were read as depending on the method; the offered
No was left as it was and no field but the method was changed.

## Prompts and what was done by hand

Each of the three type saves asked two questions, each answered Yes once: "Changing the method of
voucher numbering will affect voucher numbers of existing transactions. Continue? Yes or No", then on
accepting "Are you sure you want to update the voucher numbers of existing vouchers? Y : Accept Master &
Update Vouchers / N : Back". At k5 and k7 the Method row reading exactly "Automatic" was taken and the
field read "Automatic". The backup asked "Accept ?"; each restore asked "Accept ?" and then whether to
replace the company data folder (Replace). No import raised anything on Tally's screen.

## What it decides

On this book, this release and this one voucher type: setting "Automatic (Manual Override)" on a type
that holds vouchers blanked their numbers; an import there kept the number it carried, and one without a
number stayed without. Under "Renumber Vouchers" a back-dated numberless import changed the numbers of
later-dated Journals, as it did for Receipts, and nothing in ALTERID, in ALTVCHID or in the import
answer showed it.

## Not covered

A back-dated voucher under Manual Override; a number an existing voucher already holds; "Prevent
creating duplicate Voucher Nos." set to Yes; Payment, Contra; a batch;
Gold; other releases; the same through ComplyEaze Bridge's own post.

## Files

`requests/` (6), `answers/` (each with its headers), `tables/`, `derived/` (the answers parsed by
`tools/read-answers.ps1`), `send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in
`SHA256-kept-on-lab-pc.txt`: screenshots, the backup, listings, the restore plan, the working log.
