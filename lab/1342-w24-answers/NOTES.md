# A GST credit note keyed with its original invoice details reads the same as one keyed without them (#1342, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE PILOT LAB (its GUID is in every marks answer). TallyPrime 7.1, Silver Rental,
on Windows. One run, after the one on branch `lab/1342-w23-answers`, which keyed the same credit note
without original invoice details. No ComplyEaze Bridge build and no import: one Credit Note was keyed on
Tally's own voucher screen and read back raw by curl, one request at a time, none sent twice. A backup
was taken first and restored at the end. Times in send-log.txt are UTC.

## Method

- The three request files of the earlier run, byte for byte, nine sends: the marks read, the
  voucher-type series read and the one-day wide voucher read for 21 Sep 2026, each before the keying,
  after it and after the restore. Each send was `curl.exe -sS --max-time 60 -w "%{http_code}
  %{size_download} %{time_total}" --data-binary @<request> -H "Content-Type: text/xml; charset=utf-16"
  http://127.0.0.1:9001 -o <answer file>`; no response headers were kept.
- The credit note: type Credit Note (Automatic, Auto Retain), 21 Sep 2026, Accounting Invoice mode,
  party BRIDGE Walk-in (Unregistered/Consumer), BRIDGE Svc 998313 5% 200.00; BRIDGE CGST 2.5% 5.00 and
  BRIDGE SGST 2.5% 5.00 filled by Tally; total 210.00; no narration; the plain save. The number 1 was
  offered by Tally and not typed.
- The one difference from the earlier run: on the party sub-screen "Original Invoice No." was keyed as
  `BP/26-27/0001` and its date as 1 Aug 2026 (an invoice of this book). The eight dispatch fields above
  them were passed empty. "Reason for Issuing Note" was `01-Sales Return` again; Buyer's Debit Note No.
  and its date were left empty.

## Result

| read | before | after the save | after the restore |
|---|---|---|---|
| marks (ALTVCHID / ALTMSTID) | 68 / 245 | 69 / 245 | 68 / 245 |
| the day's vouchers | none (3,022 B) | one (10,186 B) | none (3,022 B) |
| series, 28 types (93,546 B) | first read | every row equal | byte-identical |

- The wide answer after the save is byte-identical to the earlier run's answer for the credit note
  keyed without original invoice details (sha256 6a678e08...f5717 for both), and so are the series and
  marks answers after the save.
- So the voucher row reads as before: DATE 20260921, Credit Note, VOUCHERNUMBER 1, MASTERID 45,
  ALTERID 69, PARTYLEDGERNAME BRIDGE Walk-in, AMOUNT 210.00 (positive); VCHGSTSTATUSISINCLUDED Yes,
  VCHGSTSTATUSISUNCERTAIN No, VCHGSTSTATUSISEXCLUDED No; the six other flags No; four
  `LEDGERENTRIES.LIST` rows and one `ALLINVENTORYENTRIES.LIST` element.
- Neither `BP/26-27/0001` nor `20260801` occurs anywhere in that answer, and no element of the row is
  named for an original invoice. The request does not ask for such a field and none came back unasked.
  These reads therefore do not show whether Tally stored the two keyed values; only the screen took
  them.
- Series after the save: the Credit Note row equals the first read's leaf for leaf (ALTERID 42, no
  last-number entry written), and so does every other row.
- After the restore all three answers are byte-identical to the first ones.

## Screens

The voucher screen opened already on Credit Note and in Accounting Invoice mode (the earlier run: Item
Invoice); the mode list read the same four rows. Once the invoice number stood in its field, the
original-invoice date field opened prefilled with the voucher's date, 21 Sep 2026, and 1 Aug 2026 was
typed over it. The reason list read as in the earlier run. The save raised no dialog. The backup asked
"Accept ?"; the restore asked "Accept ?" and then whether to replace the company data folder (Replace).
Tally's current date at the Gateway read 15 Sep 2026 at the start, 21 Sep 2026 after the save (moved by
Tally) and 15 Sep 2026 after the restore; the Gateway's date key was never pressed.

## Small things

The desktop tool that pressed the keys twice refused a key because Tally's window had lost focus and
pressed nothing; each key was pressed after a click on Tally's title bar.

## What it decides

On this book and this release: with this wide read, a GST credit note keyed with original invoice
details cannot be told from one keyed without them. Both read as included for GST with a positive
AMOUNT, and the Credit Note series row stays unchanged.

## Not covered

A read that fetches the original invoice fields; other reasons; Item Invoice mode; a registered party;
an import or ComplyEaze Bridge's own post; what the GST returns show; other releases; Gold.

## Files

`requests/` (3), `answers/` (nine), `tables/` (the reader's output at three points), `derived/`,
`tools/`, `send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`:
screenshots, the backup, listings, the helper scripts, the working log.
