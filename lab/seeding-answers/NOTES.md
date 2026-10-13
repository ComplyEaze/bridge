# An HSN code keyed on one stock item and one stock group, and TDS and TCS turned on, read back and restored (Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE SHAPE LAB, loaded alone. TallyPrime 7.1, Silver Rental, on Windows. One
run. A backup was taken first; three changes were made on Tally's own screens and nowhere else; six
read requests were sent raw by curl, one at a time, none sent twice; the book was restored at the end.
No import, no voucher screen, no master created. The codes are test values. Times in send-log.txt are
UTC.

## Method

- Requests (`requests/`, from branch `lab/seeding-run-sheet`): company list (s01), book extent with the
  two alteration marks (s02), company features (s03), the whole company object (s04), stock items (s05)
  and stock groups (s06) with their HSN and GST lists. 19 sends; each answer has its response headers
  and the body of a `GET /status` sent after it.
- Checks: that branch's `check.py`, run with `-X utf8`; every line it printed is in `check-lines.txt`.
  Rows were compared leaf by leaf by program: `tables/`, `tools/compare-collection.ps1`.

## Result

Marks = ALTVCHID / ALTMSTID.

| act on the screen | read back | marks |
|---|---|---|
| as found | 0 of 11 items, 0 of 3 groups coded; TDS, TCS, GST classifications No | 112 / 289 |
| item Soda Ash Light: one HSN history row, from 1 Apr 2025, 28362010 | `HSNDETAILS.LIST`: APPLICABLEFROM 20250401, HSNCODE 28362010, HSN `Disodium Carbonate, Dense`, SRCOFHSNDETAILS `Specify Details Here` | 112 / 290 |
| group Raw Chemicals: one row, from 1 Apr 2025, 2836 | the same four fields, HSNCODE 2836 | not read |
| company features: TDS Yes, TCS Yes, one save | ISTDSON Yes, ISTCSON Yes, ISGSTCLASSIFON No | 112 / 292 |
| restore | every answer byte-identical to its first one | 112 / 289 |

- The description was filled by Tally from its own list; the date and the code are what was keyed.
- The item save made Tally itself add to that item a fifth `RATEDETAILS.LIST` row under its existing GST
  entry (State Cess, Based on Value, no rate) and `LANGUAGEID` 1033. The ten other items read equal.
- The group save made Tally itself add to that group one `GSTDETAILS.LIST` row (source `As per
  Company/Stock Group`, five duty heads, no rate, no taxability) and `LANGUAGEID` 1033. The two other
  groups read equal, and no item row moved.
- Two answers equal under COLLECTION can differ in one byte outside it: a counter of the answer's
  `CMPINFO` block (STOCKITEM 0 to 1 after the item save, STOCKGROUP 0 to 1 after the group save).
- The company object after the features save: 702 elements against 701; TDSDEDUCTORTYPE `Company`,
  NUMTDSRATES 0, NUMTCSRATES 0, NUMGSTCLASSIFICATION 0.
- The company features screen shows no field for GST classifications, with "Show more features" No or
  Yes; nothing was changed for it. The TCS field shows only with "Show more features" Yes (set once).

## Three pauses, as they happened

1. After the item save the checker reported content changed beyond the HSN entry. The difference was
   listed leaf by leaf (the two additions above) and taken as a result.
2. After the group save a byte-for-byte check of the item answer failed on the one `CMPINFO` byte. From
   then on a comparison was judged on everything under COLLECTION and the marks, any `CMPINFO`
   difference recorded. After the restore none remained.
3. Setting TDS to Yes opened "Company TDS Deductor Details", which asks for a TAN. The run paused with
   the form open; it was then accepted once exactly as shown, every field empty, and Tally took it
   without a message. TCS opened "Company TCS Collector Details" and was accepted empty the same way.
   No TAN, registration number or other identifier was typed at any point.

## Prompts

Backup: "Accept ?". Item and group saves: none. Company features: "Accept ?" once after the TDS form
(answered No, to stay on the screen), none on the final accept. Restore: "Accept ?", then whether to
replace the company data folder (Replace).

## What it decides

On this book and this release: an HSN keyed in a stock item's or a stock group's HSN history reads back
in `HSNDETAILS.LIST` with its date, code, description and source; saving it makes Tally add rows of its
own to that master; TDS and TCS turn on with empty deductor and collector details.

## Not covered

TDS or TCS details, rates or natures of payment; a GST classification; an HSN set through the GST rate
screen or by import; other releases; Gold.

## Files

`requests/` (6), `answers/` (19, with headers and status bodies), `tables/`, `derived/`, `tools/`,
`check-lines.txt`, `send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in
`SHA256-kept-on-lab-pc.txt`: screenshots, the backup, listings, the helper scripts, the working log.
