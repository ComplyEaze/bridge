# #1358: the "Negative Ledgers" report on two more books, each checked against a trial balance (Windows lab, 13 Oct 2026, reads only)

TallyPrime 7.1, Silver Rental, on Windows; one Tally, one company loaded at a time; raw HTTP by curl; no ComplyEaze
Bridge build. Report request: `request-negative-ledgers.xml` of the branch `lab/1358-report-captures` with only the
company (and, on the second book, the dates) changed: TYPE Data, ID "Negative Ledgers", whole financial year. Trial
balance: the committed fixture `statement_trial_balance_fy_request.utf16le.xml` with only company and dates changed
(a ledger collection fetching TBALOPENING, DEBITTOTALS, CREDITTOTALS, TBALCLOSING). Marks and GET /status before and
after on each book; marks byte-identical; no dialog on Tally's screen at any point.

| book | year | report answer | ledgers listed | trial balance of the same year |
|---|---|---|---|---|
| BRIDGE PILOT LAB (synthetic) | 2026-27 | 524 B | 1: BRIDGE Round Off, Dr empty, Cr 0.16 | 24,498 B, 18 ledgers. BRIDGE Round Off (Indirect Expenses): closing 0.16, a credit balance on an expense ledger |
| BRIDGE CORPUS PAYABLES (lab-generated synthetic corpus book (built by import, August 2026)) | 2025-26 | 522 B | 1: PAY Bank, Dr empty, Cr 10976476.80 | 55,890 B, 44 ledgers. PAY Bank (Bank Accounts): closing 10976476.80, a credit balance on a bank ledger |

Both report answers have the shape of the earlier capture: no HEADER, STATUS or LINEERROR; one DSPACCNAME/DSPDISPNAME
and one DSPACCINFO (DSPCLDRAMT/DSPCLDRAMTA, DSPCLCRAMT/DSPCLCRAMTA) per ledger; RESPSTATUS 1 in the HTTP headers.

## What it decides
On these two books and this release:
- Each ledger the report listed read, in the trial balance of the same book and year, with a closing balance of the
  same amount on the side opposite to its group (one ledger on each book).
- No other ledger of either trial balance read on the opposite side (PILOT LAB: 17 others; PAYABLES: 40 creditors,
  32 with credit balances and 8 at zero or empty, plus purchases, cash and Profit & Loss).
- The report's amount was unsigned, in the Dr or the Cr element; the trial balance gave the same figure signed (a
  debit negative).

## Not covered
A book where the report lists nothing (no empty answer was seen), or many ledgers; groups other than expenses and
bank; a ledger negative during the year but not at its end; a shorter period; the order of the list; other releases.

## Files
`requests/`, `answers/` (every answer with its headers), `derived/` (one line per trial balance ledger), `send-log.txt`.
No answer is held back.
