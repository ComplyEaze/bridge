# Results: #1378 discovery, where Tally keeps e-invoice details, 2 Nov 2026

**Tally keeps them as three plain voucher elements, and it takes them by hand with no portal: `IRN` and `IRNACKNO`, typed String, and `IRNACKDATE`, typed Date, direct children of `VOUCHER`. Nothing reached the portal. Tally then treats the hand-keyed invoice as already e-invoiced: it did not offer to generate one on save and lists it under `IRN Generated`. The invoice without details drew "Do you want to generate e-Invoice?", declined. Bridge's window read does not carry these elements today; asked for by name, they return.**

Book BRIDGE EINVOICE LAB, new, books from 1 April 2025, no inventory, GST on, TallyPrime 7.2 Silver, licensed, loaded alone. GSTINs invented, with a PAN part that cannot be a real one, and masked as 15 X in the answers here; originals by hash in `kept-local.txt`. IRN, acknowledgement number and date invented. Masters seeded by one XML import, then finished by hand on the ledger screens; the two invoices keyed by hand. Reads were raw requests; Bridge was not run. The reads that matter: `full-a` 52,932 B, `full-b` 52,722 B, sha256 in `evidence/log.txt`.

## Answers
**1. E-invoicing on. [verified]** Alter > Statutory Masters > Maharashtra Registration: `GST Details`, section `e-invoice Details`, `e-Invoicing applicable` Yes, `Applicable from` 1-Apr-25. Screen kept on the lab PC. BK1 before any voucher. Marks M 215, no voucher mark.

**2. Tally takes the details by hand, without the portal. [verified]** This settles the first half of the issue's third acceptance question: they can be keyed.

Menu path: Gateway > Vouchers > F8 Sales > after the ledger lines, `Provide e-Invoice details` Yes > sub-screen titled `Statutory Details`, panel `e-Invoice Details`: `Ack No.`, `Ack Date`, `IRN`, then `Bill to place` and `Ship to place`.

The cursor goes straight to `Bill to place`; it does not stop on the first three. A mouse click puts it on `Ack No.` or `IRN`, and Enter on `Ack No.` moves to `Ack Date`. All three took typed values. The 64-character IRN went in whole, and a six-digit acknowledgement number raised nothing. Values of a wrong form were not tried, so whether Tally checks form is not known.

Saved with no prompt. `Provide e-Invoice details` first showed on the screen after the sales line, already Yes.

**3. Invoice B. [verified]** Same customer, amount, date, the same path, the three left blank. On save: "Do you want to generate e-Invoice?" Yes or No. Answered No with the key n. The voucher saved. Nothing was sent, and no other offer came. The prompt was not raised for A.

**4 and 5. The elements. [verified]** One request each, every native method, no fetch list. Both vouchers give 732 elements. `evidence/diff.txt`.

| value | element | container | A | B |
|---|---|---|---|---|
| IRN | `IRN` | `VOUCHER`, direct child | the 64 characters | empty |
| acknowledgement number | `IRNACKNO` | `VOUCHER`, direct child | 112233 | empty |
| acknowledgement date | `IRNACKDATE` | `VOUCHER`, direct child | 20250403 | empty |

- **Only A carries:** no element. B has all of them, empty, so presence alone says nothing; the value does.
- **Other differences, nine in all with the three above:** `GUID`, `VOUCHERNUMBER`, `ALTERID`, `MASTERID`, `VOUCHERKEY`, `VOUCHERRETAINKEY`. All are the voucher's own identity.
- **The same on both, the rest of the family:** `IRNQRCODE`, `IRNCANCELDATE`, `IRNCANCELCODE`, `IRNCANCELREASON`, `IRNIRPSOURCE` empty; `IRNCANCELLED`, `IRNJSONEXPORTED`, `ISEINVOVERRIDDEN`, `RESETIRNQRCODE`, `IGNOREEINVVALIDATION` No. So the keying set no source or override flag on A that this read shows.
- **The answer No:** nothing in the diff records it. There is no B saved another way to compare with.

**Bridge's read. [verified, the request rendered from master's strings]** The window read master sends (`AGENT_VOUCHER_FETCH`, `window-ab`) returns none of the three. The same window with `IRN,IRNACKNO,IRNACKDATE` in its FETCH (`named`) returns them, `IRN` and `IRNACKNO` with `TYPE="String"` and `IRNACKDATE` with `TYPE="Date"`: full on A, empty on B. That is the "asked for and came back empty" the issue's first acceptance needs. It is one read I added, for shape.

**6. The e-Invoice report. [verified on this book]** Go To > GST Reports > e-Invoice, period 1-Apr-25 to 3-Apr-25.

| line | count | voucher |
|---|---|---|
| `Uncertain Transactions (Corrections needed)` > `Applicable Tax Ledger is not selected` | 1 | B, no. 2 |
| `IRN Generated` > `IRN updated without QR Code` | 1 | A, no. 1 |

A's line opens a register offering `F10: Cancel IRN`, `Manually Update IRN Info` and `Get IRN Info`. So Tally has its own manual route too; it was not opened.

B's reason is about the tax line as seeded, not about e-invoicing: this row's IGST ledger holds no rate, and its 180.00 was typed. A has the same tax line and is not listed there. Why the report sorts them so is [partial]: it may be the IRN that lifts A out of that check. Whether a B with a clean tax line would list as pending generation was not tested. A clean tax line is the next thing to change if that matters.

## What it means for the issue
- **Acceptance 1, read:** shape known for a Sales invoice keyed by hand. Three named fields, and an empty element can mean no details, as asked for. [verified, one book]
- **Acceptance 3, carry:** the fields are plain voucher elements, and hand keying is accepted. Whether an import can set them, and what Tally then shows, is the issue's next measure; this row ran no such import.
- **Acceptance 4, no details:** Tally itself offers generation on save for such an invoice keyed on screen. Whether it offers anything for a voucher posted by import was not read.
- **Not settled:** a synthetic book cannot hold an IRN the portal issued. A portal-issued voucher may set `IRNQRCODE` and others that stayed empty here.

## Screens this row met
`Dispatch Details` and `Party Details` on each Sales voucher, accepted as offered; `e-Invoice Details`; the generate prompt. `Fetch Details Using GSTIN/UIN`, `Get IRN Info`, `Manually Update IRN Info`, `Cancel IRN`, `Open e-Invoice Portal`, `Set GST Status` and every send action were left alone. No portal action fired, and no read errored.

A seed import with `LEDSTATENAME`, `COUNTRYNAME`, `GSTREGISTRATIONTYPE` and `PARTYGSTIN` did not show those on the customer's ledger screen, and `LEDSTATENAME` read back empty. I keyed them by hand. This row did not look into why.

## Left behind
BK1 restored with the two prompts Tally gives, Accept and Replace on this book's own folder: `No Vouchers Entered`, V empty, M 215, the same as before the first voucher. The two invoices are gone from the book; their answers are here. Company shut, no company loaded, one tally.exe on 9001.

**Confidence:** one synthetic book, one Tally, one sitting, two invoices keyed by hand, accounting mode: PARTIAL on everything past this book.
