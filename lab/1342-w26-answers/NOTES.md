# Where a keyed credit note's original invoice number and date read back: `REFERENCE` and `REFERENCEDATE` of the voucher (#1342, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE PILOT LAB (its GUID is in every marks answer). TallyPrime 7.1, Silver Rental,
on Windows. One run, after the ones on branches `lab/1342-w23-answers`, `lab/1342-w24-answers` and
`lab/1342-w25-answers`. No ComplyEaze Bridge build and no import: one Credit Note was keyed on Tally's
own voucher screen twice, once with its original invoice number and date and once without, and the book
was restored from one backup after each; reads raw by curl, one request at a time, none sent twice.
Times in send-log.txt are UTC.

## Method

- Four request files, 14 sends: the marks read (5); the one-day wide voucher read of 21 Sep 2026 for the
  type Credit Note, 14 named fields (3); that request with its field list replaced by
  `<NATIVEMETHOD>*</NATIVEMETHOD>` ("whole", 5) and by `<FETCH>*</FETCH>` ("star", 1, on the empty day:
  the plan asked for it again only if a whole answer had fewer than 100 leaves). No response headers
  were kept.
- The credit note, both times: 21 Sep 2026, Accounting Invoice mode, party BRIDGE Walk-in
  (Unregistered/Consumer), BRIDGE Svc 998313 5% 200.00, BRIDGE CGST 2.5% 5.00 and BRIDGE SGST 2.5% 5.00
  filled by Tally, total 210.00, reason `01-Sales Return`, no narration, the plain save; the number 1
  was offered by Tally both times. First keying: Original Invoice No. `BP/26-27/0001`, date 1 Aug 2026.
  Second keying: both fields passed empty.
- `tools/leaf-compare.ps1` compares the VOUCHER rows of two answers leaf by leaf (an element with no
  child element, keyed by its positioned path) and looks for given texts in leaf values. It was run first on two earlier committed answers with a known result (56 leaves each, 17
  different, 39 equal): `tables/leaf-compare-step3-known-answer-corrected-tool.txt`.

## Result

| read | empty day | keyed with the two values | keyed without them |
|---|---|---|---|
| marks (ALTVCHID / ALTMSTID) | 68 / 245 | 69 / 245 | 69 / 245 |
| wide, 14 fields | 3,022 B, no voucher | 10,186 B | 10,186 B, byte-identical |
| whole | 3,022 B, no voucher | 52,082 B, 722 leaves | 51,988 B, 722 leaves |
| star | 3,022 B, no voucher | not sent | not sent |

- The three empty-day answers are byte-identical to each other.
- The wide answers of the two notes are byte-identical to each other and to the earlier runs' (sha256
  6a678e08...5717): the 14 fields do not tell the two notes apart.
- The whole answers hold one VOUCHER row each, no LINEERROR, 722 leaves each: 720 equal, 2 different,
  none on one side only. The two: `VOUCHER[1]/REFERENCE[1]` reads `BP/26-27/0001` against empty, and
  `VOUCHER[1]/REFERENCEDATE[1]` reads `20260801` against empty.
- Text lookups in leaf values: `BP/26-27/0001` 1 leaf with, 0 without; `20260801` 1 leaf with, 0
  without; `1-Aug` 0 and 0; `01-Sales Return` 1 leaf in each, `VOUCHER[1]/GSTNATUREOFRETURN[1]`.
- No other leaf differs between the two notes: no identifier, no timestamp, no key.
- After each restore the marks answer and the whole answer are byte-identical to the first ones.

## Screens

As in the earlier runs: the same mode list; the sub-screen "Party Details" with Receipt Details (eight fields, each passed empty) and
Original Invoice Details; a buyer form accepted with its defaults; "Additional Details : Local Sales -
Taxable" with the reason list. With a number keyed, the original-invoice date field opened prefilled
with the voucher's date and was typed over; with the number left empty, the date field showed nothing.
Neither save raised a dialog. The backup asked "Accept ?"; each restore asked "Accept ?" and then
whether to replace the company data folder (Replace). Tally's current date read 15 Sep 2026 at the
start and after each restore, 21 Sep 2026 after each save.

## What it decides

On this book and this release, a Sales credit note keyed in Accounting Invoice mode reads back, through
a collection request with `NATIVEMETHOD *`, with its original invoice number in the voucher's
`REFERENCE`, that invoice's date in `REFERENCEDATE` and its reason in `GSTNATUREOFRETURN`; left empty
on the screen, the first two read empty.

## Not covered

The single-object export of a voucher; `FETCH *` on a day that holds a voucher; Item Invoice mode; a
registered buyer; items; another reason or rate; a debit note; an imported or cancelled note; ComplyEaze
Bridge's own post; other releases; Gold.

## One earlier stop

A first attempt stopped before anything was sent: the comparer's first version failed its known-answer
check (two variable names that differed only by case). `tools/` holds the corrected one.

## Files

`requests/` (4), `answers/` (14), `tables/`, `derived/`, `tools/`, `send-log.txt`, `SHA256.txt`. Kept on
the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`: screenshots, the backup, listings, the helper
scripts, the first attempt's output, the working log.
