# #485 part E: a ledger-scoped census in AlterID spans

Not for merge. READS ONLY, on one company the lab already holds: BRIDGE CORPUS DENSE (synthetic, GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, voucher mark 29,900 on 26 Oct 2026). `LAB-COMMON.md` in this folder (v3, a byte copy of the one in `lab/485-ledger-filtered-vouchers-run-sheet`, itself from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full, with this company as THE COMPANY; where this sheet and it differ, the stricter holds.

**WHEN.** Directly after part D of `lab/485-ledger-filtered-vouchers-run-sheet` (04a50315), in the same sitting, with the owner still at the machine. Part E runs only if part D step 3 (`d03`) answered in-band AND its GUID set equals the d02 vouchers that carry a `DN Sales` ledger line (82 on 15 May 2025). Otherwise part E is NOT RUN: record that, and why, and send nothing from this folder.

**WHY.** `ledger_movement` for one ledger reads every voucher in its window, so a whole year of a book of about 30,000 vouchers is refused. A read of one ledger's vouchers has to be planned from a count of that ledger's vouchers, and every count request has to be bounded before it is sent. The candidate count request is the shipped voucher census (`render_agent_voucher_census` in `src-tauri/src/agent_read_profiles.rs`, with the AlterID span of `AlterIdSpan::filter` in `src-tauri/src/agent_voucher_window.rs`) with exactly three things changed:
- the header ID and the collection NAME: `Bridge Agent Ledger Census`;
- the collection TYPE: `Vouchers : Ledger` (the type part D tried);
- an added `CHILDOF` element naming the ledger.

The date window, the AlterID span, the fetch list (`GUID,ALTERID,DATE`) and the company are the shipped census's. The span keeps every answer at most 8,192 rows whatever the ledger, because AlterIDs are distinct. No `$$FilterCount`, no `$$NumItems`, no report, no function with a spaced argument: the ledger name is element text.

**REQUESTS BY CURL** are allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover shapes; every request here only reads. The release is the one part D recorded from Tally's About screen in the same sitting (the lab PC runs 7.2 since 25 Oct 2026); record it again if part E starts in a later sitting.

**THE REQUESTS** (`requests/`, all UTF-16LE with a byte-order mark, sent with `charset=utf-16` as LAB-COMMON C3 says):

| File | What | Window | AlterID span | Size bound |
|---|---|---|---|---|
| `e01_marks` | the company marks (byte copy of part D's `d01`) | | | 8,192 bytes |
| `e02` to `e05` | ledger census, `DN Sales` | 15 May 2025 | (0, 8192], (8192, 16384], (16384, 24576], (24576, 29900] | 1,000,000 bytes each (the day holds 82 vouchers) |
| `e06` to `e09` | ledger census, `DN Party 001` | 1 Apr 2025 to 31 Mar 2026 | the same four spans | 4,000,000 bytes each (8,192 rows at most by construction; an answer near that would mean the ledger was not applied, and its bound stops the part first) |
| `e10` | part D's `d03` request with the ledger `DN Party 001` and the dates 1 Apr 2025 to 31 Mar 2026 | 1 Apr 2025 to 31 Mar 2026 | none | 8,000,000 bytes |

`DN Party 001` is one of the book's 123 generated ledgers (`DN Party 001` to `DN Party 120`, `DN Sales`, `Cash`, `Profit & Loss A/c`; recorded in `STATEMENT_CAPTURE_PROVENANCE.md` under the protocol crate's test fixtures). How many vouchers it holds is not known before E5.

**OWNER steps** are marked [OWNER]. E2 and E5 are a NEW SHAPE on this book (the first ledger census request sent): screenshot of the whole Tally window straight after each answer, before anything else is sent (C3).

## Before part E
1. Part D finished and recorded, its C10 included. Load THE COMPANY alone (C1) and run step 0 (C2).
2. Check every file in this folder against `SHA256.txt` (hash and bytes). A difference = STOP, nothing is sent.
3. `python -I check-request-ids.py requests` in this folder. EXPECT `census: ok 10, skipped 0, FAIL 0`. Anything else = STOP, nothing is sent.
4. [OWNER] Go-ahead.

## Part E
For every request record the UTC time before the send and after the answer.
1. GET /status. Then `e01_marks`. EXPECT one COMPANY row, THE COMPANY's GUID, ALTVCHID `29900`. Any other mark = STOP (the spans in e02 to e09 were written for that mark).
2. `e02` (span 1 of the day). NEW SHAPE: screenshot at once. Then GET /status.
3. `e03`, `e04`, `e05`, one at a time, GET /status after each.
4. In steps 2 and 3, and in every send of steps 5 to 7: a dialog, a hang, a non-200, an answer over its bound or an answer that does not parse = STOP (C3, C4, C7, C9). The gateway is not probed again.
5. `e06` (span 1 of the year, `DN Party 001`). NEW SHAPE for a whole year: screenshot at once. Then GET /status. Count its VOUCHER rows (C4). If they are more than 1,000: record, send nothing more from `e07` to `e10`, go to step 8.
6. `e07`, `e08`, `e09`, one at a time, GET /status after each. Add the VOUCHER rows of `e06` to `e09`.
7. Only if that sum is 1,000 or fewer: `e10`. Then GET /status. Otherwise record the sum and do not send `e10`.
8. `e01_marks` again. EXPECT the marks of step 1 (every request here is a read); a difference = record it and STOP.
9. C10.

## RECORD (the lab judges only the STOP rules)
For every answer: the curl line's status and bytes, CONTENT-LENGTH and every response header as received, the sha256, the send and answer times; whether it parses; the row element name and the VOUCHER row count (rows under ENVELOPE/BODY/DATA/COLLECTION, counted after parsing, never by text search; the CMPINFO block holds counters, not rows); for each row its GUID, ALTERID and DATE, and whether each is non-empty. An answer with no rows is a result: keep it whole, with its size.

## COMPARED BY THE REQUESTER AFTER THE RUN
- The union of the GUIDs of `e02` to `e05` against part D's `d03` GUIDs (82): equal sets is the result wanted; any difference, either way, is the finding. Which span held them.
- The union of `e06` to `e09` against `e10`: equal sets wanted.
- Every row's ALTERID inside its request's span, and its DATE inside its window.
- The time of each span request against the shipped census requests of part D's day (if part D recorded them) and against each other (empty and non-empty spans).

## The book
It does not change; every request is a read. The marks are taken at the start and the end of the part and must be equal.

## Close and evidence
- **C10** at the end.
- **C11:** evidence to a new branch `lab/485-ledger-census-spans-answers`, after the pre-push scan and the run coordinator's yes. It holds every answer whole with its headers, size, sha256 and times; the row tables; the notes (UTC, one line per step). Screenshots stay on the lab PC, listed by name, size and sha256.

## What this part does not show
Vouchers that are cancelled, optional or post-dated; a ledger reached only through a bill or inventory allocation; a voucher with two lines to the same ledger; Gold, Educational mode, or a client book; any cost on a book with a larger voucher mark.
