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
