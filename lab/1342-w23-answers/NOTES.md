# What a keyed GST credit note reads as, and whether saving it writes to its number series (#1342, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE PILOT LAB (its GUID is in every marks answer). TallyPrime 7.1, Silver Rental,
on Windows. One run, after the one on branch `lab/1342-w22-answers`. No ComplyEaze Bridge build and no
import: one Credit Note was keyed on Tally's own voucher screen and read back raw by curl, one request
at a time, none sent twice. A backup was taken first and restored at the end. Times in send-log.txt
are UTC.

## Method

- Three request files, nine sends: the marks read, the voucher-type series read and the one-day wide
  voucher read for 21 Sep 2026, each before the keying, after it and after the restore. Each send was
  `curl.exe -sS --max-time 60 -w "%{http_code} %{size_download} %{time_total}" --data-binary @<request>
  -H "Content-Type: text/xml; charset=utf-16" http://127.0.0.1:9001 -o <answer file>`; no response
  headers were kept on this run.
- The credit note: type Credit Note (Automatic, Auto Retain), 21 Sep 2026, Accounting Invoice mode,
  party BRIDGE Walk-in (Unregistered/Consumer), BRIDGE Svc 998313 5% 200.00; BRIDGE CGST 2.5% 5.00 and
  BRIDGE SGST 2.5% 5.00 filled by Tally; total 210.00; no narration; the plain save. The number 1 was
  offered by Tally and not typed.
- The case measured: a credit note WITHOUT original invoice details. On the party sub-screen Original
  Invoice No., its date and every dispatch field were passed empty. On the GST details sub-screen
  "Reason for Issuing Note" was set to `01-Sales Return`; Buyer's Debit Note No. and its date were left
  empty.

## Result

| read | before | after the save | after the restore |
|---|---|---|---|
| marks (ALTVCHID / ALTMSTID) | 68 / 245 | 69 / 245 | 68 / 245 |
| the day's vouchers | none (3,022 B) | one (10,186 B) | none (3,022 B) |
| series, 28 types (93,546 B) | first read | every row equal | byte-identical |

- The voucher row: DATE 20260921, VOUCHERTYPENAME Credit Note, VOUCHERNUMBER 1, MASTERID 45, ALTERID 69,
  PARTYLEDGERNAME BRIDGE Walk-in, AMOUNT 210.00. AMOUNT is positive; a Sales invoice of this book reads
  negative.
- VCHGSTSTATUSISINCLUDED Yes, VCHGSTSTATUSISUNCERTAIN No, VCHGSTSTATUSISEXCLUDED No; ISGSTOVERRIDDEN,
  ISCANCELLED, ISOPTIONAL, ISPOSTDATED, ISDELETED and ISDELETEDVCHRETAINED all No.
- Four `LEDGERENTRIES.LIST` rows (not `ALLLEDGERENTRIES.LIST`): BRIDGE Walk-in 210.00 with
  ISDEEMEDPOSITIVE No; the service line -200.00, CGST -5.00 and SGST -5.00 with Yes; and one
  `ALLINVENTORYENTRIES.LIST` element.
- Series after the save: the Credit Note row equals the first read's leaf for leaf (ALTERID 42, no
  last-number entry written), and so does every other row. The two answers differ only in five count
  lines of the company information block.
- After the restore all three answers are byte-identical to the first ones.

## Current date and prompts

Tally's current date at the Gateway read 15 Sep 2026 at the start, 21 Sep 2026 after the save (moved by
Tally) and 15 Sep 2026 after the restore. The Gateway's date key was never pressed; the date was set
inside the voucher.

The screen opened in Item Invoice mode; its mode list read Item Invoice, Accounting Invoice, As
Voucher, Use for Interest Accounting. The reason list read Not Applicable, 01-Sales Return, 02-Post
Sale Discount, 03-Deficiency in services, 04-Correction in Invoice, 05-Change in POS, 06-Finalization
of Provisional assessment, 07-Others. The save raised no dialog. The backup asked "Accept ?"; the
restore asked "Accept ?" and then whether to replace the company data folder (Replace).

## Two pauses and one slip

- The series answer held 28 voucher types where 27 were expected (the expectation was older than one type of
  this book). The run paused, then went on.
- The sub-screen with the dispatch and original-invoice fields was not in the plan. The run paused with
  nothing pressed, then passed every field empty.
- An interruption arrived while one key batch was in flight; its Enter, on an empty field, had gone
  through. It was first recorded as not pressed, then corrected; one screenshot is one step late.

## What it decides

On this book and this release: a GST credit note keyed without original invoice details reads as
included for GST, with a positive AMOUNT and its ledger entries under `LEDGERENTRIES.LIST`; it takes
its number from the Credit Note type's automatic series and leaves that series row unchanged.

## Not covered

A credit note WITH original invoice details; other reasons; Item Invoice mode; a registered party; a
debit note; an import or ComplyEaze Bridge's own post; what the GST returns show; other releases; Gold.

## Files

`requests/` (3), `answers/` (nine), `tables/` (the reader's output at four points), `derived/`,
`tools/`, `send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`:
screenshots, the backup, listings, the helper script, the working log.
