# Results: read rows on THE BOOK (a real book, kept on the lab PC), 2 Nov 2026

THE BOOK is the heavy real book of the census rows, loaded alone. Reads only, posting off in every build, one request at a time, through the tools over stdio, each with a new data folder. The answers stay on the lab PC and are deleted after the row. Only outcomes, counts, sizes and seconds are here. "FY end" is the last completed financial year's end.

**Marks:** read raw before B1 and after B2's stop: one company row, byte-equal answers.

## B1, #2180: stock_summary past the single-read mark
Build: run 41188061375, artifact 13038351623, head a555d292, zip digest equal. Master mark above 874: yes.

| call | outcome | rows | answer bytes saved | bytes read from Tally | seconds |
|---|---|---|---|---|---|
| stock_summary, FY end | answered, evidence complete, state `value_total_matched` | 5 | 16875 | 86818 | 3.05 |

Item count cross-check `matched`; tie-out `matched`; `totals.partial` true, with 4 empty closing values and 0 unread quantities. [verified]

## B2, #2119 / #2158: Bills timing
Build: run 40927556771, artifact 12921968844, head 0fa9ee38, zip digest equal.

| call | outcome | rows | answer bytes saved | bytes read from Tally | seconds |
|---|---|---|---|---|---|
| outstandings, receivable, FY end, no party, no detail | refused in band: `native_outstandings_read_failed`, cause `request_deadline_exceeded`, evidence partial | 0 | 1733 | 49433009 | 96.9 |
| outstandings, payable | not sent | - | - | - | - |

The call took about 37 s longer than the 60 s expected, so the row ended there. The refusal came through the tool, with no cut by the client. `GET /status` answered 200 in 14 s just after, then in under 1 s; Tally stayed on the Gateway with no dialog; one tally.exe. [verified]

Which post passed its deadline is not established from what is here. [partial]

**Confidence:** one sitting, one real book, one call each: PARTIAL.

## Deviations
1. The second marks read came after a gap, on the run lead's word, because the timeout rule sent nothing more at the stop.

## B3, read smoke of master
Build: run 41297868762, artifact 13080855097, head 4cdfe448, zip digest equal; 31 tools, none that prepares or posts. Dates: the last completed financial year; the month is its last; every call once, in this order. The run ended at the first read that hit a deadline.

| call | outcome | rows | answer bytes saved | bytes read from Tally | seconds |
|---|---|---|---|---|---|
| tally_status | answered, complete | 1 loaded company | 1865 | 4149 | 0.06 |
| list_companies | answered, complete | 1 | 1611 | 8196 | 0.06 |
| trial_balance | answered, complete, state `observed`, a first page | 300 | 207358 | 23811162 | 4.28 |
| profit_and_loss | answered, complete, state `not_established` | 4 unclassified ledgers | 16259 | 29349346 | 5.96 |
| balance_sheet | answered, complete, state `not_established` | 4 unclassified ledgers | 13275 | 29349346 | 3.24 |
| stock_summary | refused `stock_summary_too_large`, evidence partial | 0 | 2505 | 22377 | 0.64 |
| ledger_masters (basic) | answered, complete, a first page | 500 of 9451 | 147445 | 21523206 | 3.00 |
| masters (groups) | answered, complete, state `observed`, a first page | 500 of 1455 | 112659 | 5533450 | 1.22 |
| sales_register | refused `party_ledger_master_read_failed`, cause `request_deadline_exceeded`, evidence partial | 0 | 1726 | 44475709 | 24.94 |
| purchase_register, vouchers, cash_flow | not sent | - | - | - | - |

- **The stop:** sales_register's deadline ended the run. `GET /status` answered 200 within 3 s, Tally stayed on the Gateway with no dialog, one tally.exe. [verified]
- **Marks:** read raw before and after B3, byte-equal to the first read. [verified]
- **Not established:** profit_and_loss gives the reason `unclassified_ledger_carries_an_amount`, with 4 ledgers listed unclassified; balance_sheet reads the same. Tally's own statements were not compared here. [verified for the state; whether those 4 ledgers should be classified is not established]
- **stock_summary:** master has no census admission yet, so this refusal is what #2180 changes (B1 answered on #2180's build). [verified]
- **Looked at twice:** trial_balance's first page held 300 rows, while its schema names a row limit of 500. The page was marked truncated with a next offset, so nothing was dropped, but why it stopped at 300 was not read. [partial]

**Confidence:** one sitting, one real book, one call each: PARTIAL.
