# #1232: month-wise registers by report name (Windows lab, 13 Oct 2026, reads only)

TallyPrime 7.1, Silver Rental, on Windows; one Tally, one synthetic company loaded; raw HTTP by curl. Envelope: the
report request of the branch `lab/1358-report-captures` (request-funds-flow.xml) with only the ID and the company
changed: TYPE Data, ID = the report name, 1 Apr 2026 to 31 Mar 2027. One request per name, sent once; GET /status
and a marks read before and after; marks byte-identical each time; no dialog on Tally's screen at any point.

| folder | book | report ID | answer | what came back |
|---|---|---|---|---|
| `pilot/` | BRIDGE PILOT LAB | Sales Register | 5,562 B | 12 month rows with DSPDRAMT, DSPCRAMT, DSPCLAMT. August Cr 17638.50, closing 17638.50; September Cr 210.20, closing 17848.70 |
| `pilot/` | BRIDGE PILOT LAB | Purchase Register | 5,550 B | same shape. August Dr -2100.00, closing -2100.00 |
| `pilot/` | BRIDGE PILOT LAB | Journal Register | 4,182 B | 12 month rows with DSPDRAMT and DSPCRAMT only; all empty (the book holds no Journal) |
| `n2/` | BRIDGE MANUAL NUMBERING LAB 2 | Journal Register | 4,184 B | same shape. July: DSPDRAMTA "3", Cr empty; other months empty |

No answer holds a HEADER, STATUS or LINEERROR; RESPSTATUS 1 in the HTTP headers.

## Checked against the books
- Sales Register, August 17638.50 = the invoice totals of the Sales-class vouchers of the month that are neither
  cancelled nor optional (tax included); September 210.20 = the one Sales voucher of 15 Sep. The closing column is a
  running total within the year. Purchase Register, August: the two Purchases total 2100.00, shown as a negative Dr.
- Journal Register on the second book: it holds exactly three Journals, all in July (14.00, 12.00, 13.00; total
  39.00). The July figure "3" is their NUMBER, not an amount.

## What it decides
On these books and this release: "Purchase Register" and "Journal Register" are accepted as export report IDs,
in-band. The Sales and Purchase Registers give month amounts with a closing column; the Journal Register gives a
voucher count per month, in the debit element, and no closing column.

## Not covered
Receipt, Payment and other registers (not sent); equality with Tally's own register screens; credit and debit notes;
a book with stock items; cancelled or optional journals; other releases.

## Files
Per book: `requests/`, `answers/` (every answer with its headers), `send-log.txt`. No answer is held back.
