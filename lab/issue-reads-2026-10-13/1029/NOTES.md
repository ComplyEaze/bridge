# #1029 item 2: ranges of several days read in one request, against a count (Windows lab, 13 Oct 2026, reads only)

TallyPrime 7.1, Silver Rental, on Windows; one Tally, one company loaded; raw HTTP by curl; no ComplyEaze Bridge
build. Requests are the product's own census and voucher-window text with only the company name and the dates
changed. Each voucher read was sized from a census count before it was sent and is ONE request; its rows are compared
with the census rows of the same dates by (GUID, ALTERID, DATE). This measures Tally's side of the rule, not the
product's own reader.

`payables/`: BRIDGE CORPUS PAYABLES (lab-generated synthetic corpus book (built by import, August 2026)): 1,680 vouchers (Purchase 1,200,
Payment 480). `dense/`: BRIDGE CORPUS DENSE (lab-generated synthetic corpus book (vouchers generated and imported by script; in use by August 2026)): 29,900 Sales vouchers, 82 on every day
of September 2025. Marks before and after each sitting are byte-identical (1680 / 249 and 29900 / 328).

| book | range (Sep 2025) | census rows | data rows | sets equal | bytes | per voucher | seconds |
|---|---|---|---|---|---|---|---|
| payables | 1 day | 6 | 6 | yes | 202,808 | 33,801 | 0.77 |
| payables | 3 days | 16 | 16 | yes | 539,046 | 33,690 | 0.47 |
| payables | 7 days | 39 | 39 | yes | 1,306,658 | 33,504 | 0.66 |
| payables | 30 days | 168 | 168 | yes | 5,617,790 | 33,439 | 1.54 |
| dense | 1 day | 82 | 82 | yes | 2,794,142 | 34,075 | 0.79 |
| dense | 3 days | 246 | 246 | yes | 8,376,374 | 34,050 | 2.74 |
| dense | 5 days | 410 | 410 | yes | 13,958,606 | 34,045 | 4.40 |
| dense | 7 days | 574 | 574 | yes | 19,540,838 | 34,043 | 4.76 |

## What it decides
On these two books and this release, a range of 3 to 30 days read in one request returned exactly the vouchers the
census of the same dates counted, up to 574 vouchers and 19.5 MB in one answer, with the marks unmoved. The cost per
voucher was flat (33.4 to 34.1 KB). On the dense book a week (19,540,838 B) is larger than 16 MiB; five days are not.

## Not covered
The product's own reader on such ranges (its request count, how it splits a range over its budget, its "complete"
label); a whole-year census (about 74 MB) and a 30-day voucher read (about 84 MB) on the dense book: not sent;
cancelled, optional or post-dated vouchers in a range (none in these ranges); uneven days; other releases.

## Files
Per book: `requests/`, `answers/` (answers up to 1 MB, with headers), `derived/` (the key list, GUID|ALTERID|DATE per
row, of EVERY census and voucher answer), `send-log.txt`. Held back on the lab PC because they are over 1 MB, listed by
hash in `../SHA256-kept-on-lab-pc.txt`: payables: the year census and the 7- and 30-day voucher answers; dense: the
30-day census and all four voucher answers. Every "sets equal" line can be re-derived from the key lists here.
