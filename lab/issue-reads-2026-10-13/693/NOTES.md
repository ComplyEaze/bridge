# #693: does the window census match the data read, voucher for voucher? (Windows lab, 13 Oct 2026, reads only)

TallyPrime 7.1, Silver Rental, on Windows; one Tally, one company loaded at a time; raw HTTP by curl, one request at a
time; no ComplyEaze Bridge build. Requests are the product's own census text (collection "Bridge Agent Voucher
Census", FETCH GUID,ALTERID,DATE) and voucher-window text (collection "Bridge Agent Vouchers") for 1 Apr 2026 to
31 Mar 2027, with only the company name filled in. Per book: marks, census, a one-day voucher read to size the wide
one, the wide voucher read, marks again. Both books are synthetic; each book's GUID is in its marks answers.

| book | marks (ALTVCHID / ALTMSTID), before = after | census rows | data rows | sets equal by (GUID, ALTERID, DATE) |
|---|---|---|---|---|
| BRIDGE BATCH LAB | 87 / 224, byte-identical | 78 (191,358 B) | 78 (2,366,712 B) | yes, 78 of 78 |
| BRIDGE PILOT LAB | 68 / 245, byte-identical | 41 (102,580 B) | 41 (2,568,786 B) | yes, 41 of 41 |

- BATCH LAB: Receipt 31, Payment 32, Contra 12, Journal 2, Lab Sales Bills 1; no voucher cancelled, optional or
  post-dated. AlterIDs 1 to 78, so the voucher mark (87) ran 9 ahead of the highest AlterID a voucher carried.
- PILOT LAB: 35 Sales-class vouchers, 4 Receipts, 2 Purchases. Its one cancelled voucher (BP/26-27/0041) and its one
  optional voucher (BP/26-27/0042) were both in the census and in the data read. No post-dated voucher in the book.
- Cost: census 2.45 to 2.5 KB a row; data 30.3 KB a voucher (bank-style) and 62.7 KB (GST sales).

## What it decides
On these two books and this release, with the marks unmoved across the pair, a census of the whole year and the data
read of the same window returned the same vouchers by GUID and AlterID; a cancelled and an optional voucher were
counted and returned alike. The voucher mark bounded the census from above but did not count it.

## Not covered
A post-dated voucher; a book of a few hundred vouchers or more (see the #1029 folder for 1,680 and 29,900); an edit
between the census and the data read; a census walked in AlterID spans; the product's own reader; other releases.

## Files
`requests/`, `answers/` (with headers), `derived/` (the key list, GUID|ALTERID|DATE per row, of each census and voucher
answer, made by `../tools/list-keys.ps1`), `send-log.txt`. Held back on the lab PC because they are over 1 MB, listed by
hash in `../SHA256-kept-on-lab-pc.txt`: the two whole-year voucher answers. Both "sets equal" lines can be re-derived
from the key lists here.
