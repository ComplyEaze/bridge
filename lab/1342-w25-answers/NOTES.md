# What a keyed sales-side GST debit note reads as: the screen takes a customer and a sales ledger, the amount reads negative (#1342, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE PILOT LAB (its GUID is in every marks answer). TallyPrime 7.1, Silver Rental,
on Windows. One run, after the ones on branches `lab/1342-w23-answers` and `lab/1342-w24-answers` (a
keyed credit note). No ComplyEaze Bridge build and no import: one Debit Note was keyed on Tally's own
voucher screen and read back raw by curl, one request at a time, none sent twice. A backup was taken
first and restored at the end. Times in send-log.txt are UTC.

## Method

- Three request files, nine sends: the marks read, the voucher-type series read and a one-day wide
  voucher read for 22 Sep 2026 and the type Debit Note (the credit-note request with its day and type
  name changed), each before the keying, after it and after the restore. Each send was `curl.exe -sS
  --max-time 60 -w "%{http_code} %{size_download} %{time_total}" --data-binary @<request> -H
  "Content-Type: text/xml; charset=utf-16" http://127.0.0.1:9001 -o <answer file>`; no response
  headers were kept.
- The debit note: type Debit Note (Automatic, Auto Retain), 22 Sep 2026, Accounting Invoice mode, party
  the customer BRIDGE Walk-in (Unregistered/Consumer), the sales ledger BRIDGE Svc 998313 5% 200.00;
  BRIDGE CGST 2.5% 5.00 and BRIDGE SGST 2.5% 5.00 filled by Tally; total 210.00; no narration; the plain
  save. Original Invoice No. `BP/26-27/0001`, date 1 Aug 2026; reason `04-Correction in Invoice`. The
  number 1 was offered by Tally and not typed.

## Result

| read | before | after the save | after the restore |
|---|---|---|---|
| marks (ALTVCHID / ALTMSTID) | 68 / 245 | 69 / 245 | 68 / 245 |
| the day's vouchers | none (3,022 B) | one (10,174 B) | none (3,022 B) |
| series, 28 types (93,546 B) | first read | every row equal | byte-identical |

- The screen took the customer and the sales ledger; nothing was refused.
- The voucher row: DATE 20260922, VOUCHERTYPENAME Debit Note, VOUCHERNUMBER 1, MASTERID 45, ALTERID 69,
  PARTYLEDGERNAME BRIDGE Walk-in, AMOUNT -210.00. AMOUNT is negative, the sign of a Sales invoice of
  this book; the credit note read +210.00. Voucher-level ISDEEMEDPOSITIVE reads Yes (credit note: No).
- VCHGSTSTATUSISINCLUDED Yes, VCHGSTSTATUSISUNCERTAIN No, VCHGSTSTATUSISEXCLUDED No; ISGSTOVERRIDDEN,
  ISCANCELLED, ISOPTIONAL, ISPOSTDATED, ISDELETED and ISDELETEDVCHRETAINED all No.
- Four `LEDGERENTRIES.LIST` rows, each the mirror of the credit note's: BRIDGE Walk-in -210.00 with
  ISDEEMEDPOSITIVE Yes; the service line 200.00, CGST 5.00 and SGST 5.00 with No; and one
  `ALLINVENTORYENTRIES.LIST` element. The row has the credit note's 56 leaf paths; 17 differ in value.
- Neither `BP/26-27/0001` nor `20260801` occurs in the answer; the request does not ask for them.
- Series after the save: the Debit Note row equals the first read's leaf for leaf (ALTERID 44, no
  last-number entry written), and so does every other row.
- The empty answer of the new request is byte-identical to the empty answer of the credit-note request.
- After the restore all three answers are byte-identical to the first ones.

## Screens, and how they differ from the Credit Note's

The screen opened in Accounting Invoice mode; the mode list read Item Invoice, Accounting Invoice, As
Voucher, Use for Interest Accounting. The party sub-screen's upper panel is headed "Dispatch Details"
(Credit Note: "Receipt Details") and has five fields, each passed empty: Tracking No(s), Carrier
Name/Agent, Bill of Lading/LR-RR No., its date, Motor Vehicle No.; it has no Dispatch Doc No.,
Dispatched through or Destination. The original-invoice date field opened prefilled with the voucher's
date and was typed over. The reason sub-screen has one field
and a shorter list: Not Applicable, 04-Correction in Invoice, 05-Change in POS, 06-Finalization of
Provisional assessment, 07-Others. The save raised no dialog. The backup asked "Accept ?"; the restore
asked "Accept ?" and then whether to replace the company data folder (Replace). Tally's current date
read 15 Sep 2026 at the start, 22 Sep 2026 after the save and 15 Sep 2026 after the restore.

## What it decides

On this book and this release: a debit note keyed to a customer on a sales ledger is accepted, reads as
included for GST with a negative AMOUNT and mirrored entries under `LEDGERENTRIES.LIST`, takes its
number from the type's automatic series and leaves that series row unchanged.

## Not covered

A purchase-side debit note; other reasons; Item Invoice mode; a registered party; a read that fetches
the original invoice fields; an import or ComplyEaze Bridge's own post; other releases; Gold.

## Files

`requests/` (3), `answers/` (nine), `tables/`, `derived/`, `tools/`, `send-log.txt`, `SHA256.txt`. Kept
on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`: screenshots, the backup, listings, the helper
scripts, the working log.
