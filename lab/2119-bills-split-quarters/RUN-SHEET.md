# 2119 Bills Receivable by first day (#2119): the product's request to 31 March 2026, sent with four quarter starts as its first day, by curl, read only, on the dense synthetic book

Not for merge. About 30 minutes, most of it Tally computing. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
`outstandings` reads Tally's Bills Receivable report from the books' first day to `as_of`. On BRIDGE CORPUS DENSE at 31 March 2026 that one request took 41.5 s by curl (repeat 36.7 s; `lab/2119-bills-read-timing-answers`), twice ComplyEaze Bridge's 20-second limit, so the tool is refused there. A way to keep the limit is to ask for fewer bills per request. Two things are not known, and this sheet measures both:

1. **What the first day (`SVFROMDATE`) does to the report.** Does it limit the rows to the bills dated from that day that are still open at `as_of`, with each such row exactly as in the full report? Or does it change nothing, or change the balances?
2. **Whether the time follows the rows returned, or the book up to `as_of`.** On this book a later `as_of` means both more open bills and more vouchers, about equally, so the earlier timings cannot tell them apart. A request with a later first day returns fewer rows over the same vouchers.

What this sheet cannot give is a split of the report into disjoint parts. `SVTODATE` is the `as_of` itself (the balances are as of it), so a first day alone gives the bills from that day on: a suffix, never a window with an end. If (1) and (2) both hold, that is evidence that a split by another key would run faster per request (by ledger, under the `Ledger Outstandings` report, still to be measured). If (2) does not hold, no split of this report keeps a request under the limit on this book, and the issue goes back to its owner's decision. **One sitting, PARTIAL until repeated.**

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step here on the synthetic company, on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs (its rule on names applies to what is pushed).

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic, about 30,000 vouchers), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, as the 29 Oct timing sitting left it: marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026. If it is absent, or step 1's marks differ (record the difference), STOP and report; the run coordinator decides. Only THE COMPANY is loaded, alone, from its load in step 0 to the last step.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Two kinds of read.** Tool reads are calls of ComplyEaze Bridge's built tool, one at a time (`list_companies`, `ledger_masters` basic), for the company and its marks only. Raw reads by curl are allowed by the owner's decision of 26 Oct 2026 for shape discovery: each is a committed request in `requests/` (`gen_requests.py`), the product's own Bills Receivable request with only its first day changed (q-1 is the product's request byte for byte), sent as C3 sets out.
2. **Closed set.** The report ID is `Bills Receivable`; the as-of day is 20260331; the first days are 20250401, 20250701, 20251001 and 20260101. Nothing read from Tally or from an answer adds to the set.
3. **Timing.** C3's line is sent with one named difference, in curl's output fields only (`-w` extended to `"%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}"`; the request sent is the same), and the UTC clock time is noted just before it starts and just after curl exits. Still no `--max-time`: a request without an answer in 10 minutes is C3's STOP (leave that curl running, send nothing, report). Size bound for each raw answer: 200,000,000 bytes (`size_download` above it = STOP before `read.py` is run, keeping the file); a missing CONTENT-LENGTH header is C3's "size that differs" = STOP, keeping the file.
4. **One request at a time.** While a curl or a tool call is running nothing else is sent to port 9001, `GET /status` included. During every request a screenshot of Tally's whole window is taken every 30 s, by screen capture only (no click, no focus change, no key); a dialog there is C7.
5. **Marks.** `ledger_masters` with `fields` basic: `snapshot.master_alter_id`, `snapshot.voucher_alter_id`, `snapshot.reused` false. Read before the first send and after the last, both equal.
6. **One Tally.** At step 0 and at the end, as C2 a and b set out plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.
7. **Keys.** The Gateway is the only screen used; no key is pressed except the arrows and Enter of Select Company at step 0, and the Company > Shut of C10. **Never Ctrl+Q or Escape at the Gateway of Tally.**

C2 and C1 as applied: step 0 is run with no company loaded and THE COMPANY is loaded straight after it, so "only THE COMPANY is loaded" holds from that load. C2 d's MARKS read is met by step 1 (`list_companies` showing exactly one company, and the `ledger_masters` snapshot marks): no raw MARKS request is sent, since rule 2 allows no other raw request.

Licensed mode only; Education mode = STOP.

## Build
Any build that holds `list_companies` and `ledger_masters`, for step 1 and step 4 only (the raw reads do not go through it): the published 0.6.0 `bridge_mcp.exe` the timing sitting used (sha256 `4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44`) is named here. Record its sha256 before the first call and after the last: another sha256 = STOP. Over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2119q\data`, listed first: absent or empty, else STOP), `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), redaction unset. Writes stay off.

## Files
`gen_requests.py`, `requests/` (the four generated requests), `read.py` (the timing sheet's reader, with a `compare` command added), `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I gen_requests.py --check`: EXPECT exactly `gen_requests: ok (4 requests of 770 bytes; q-1 is the product's request)`.
3. `python -I gen_requests.py --out <a new empty folder>` and compare each file's sha256 with `SHA256.txt`.
4. There is no request-ID census: the requests are TYPE Data reports that define no collection.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded (C2 d's MARKS read is step 1): the screenshot, the process list and port 9001, `GET /status`. Hash the exe. Load THE COMPANY alone (arrow keys). Screenshot of the Gateway with the Current Period and Tally's current date (EXPECT 31 March 2026).
1. Start ComplyEaze Bridge. `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401. `ledger_masters` (basic): M0, the master mark, and V0, the voucher mark (EXPECT M0 328 and V0 29900; a voucher mark below 29900 = STOP, not this book). Stop ComplyEaze Bridge (its stop file) before step 2, so that nothing but curl sends during the raw reads.
2. **The raw reads**, in this order, each by C3's line with the rule 3 timing, each answer saved as received with headers, size and sha256. After each:
   - `python -I read.py bills <answer>` (record the line whole);
   - a screenshot of Tally's whole window;
   - the elapsed time from rule 3.

   Nothing is sent between a read and its screenshot. The order puts the full report first and last, so a change in Tally's speed during the sitting shows:
   - A1 `requests/q-1-from-20250401.utf16le.xml`: the full report. EXPECT `BILLFIXED` 29900 and the timing sitting's sum of `BILLCL` (R4). A difference is recorded, and the steps go on.
   - A2 `requests/q-4-from-20260101.utf16le.xml`
   - A3 `requests/q-3-from-20251001.utf16le.xml`
   - A4 `requests/q-2-from-20250701.utf16le.xml`
   - A5 `requests/q-1-from-20250401.utf16le.xml` again (the same file, the same hash)
   - A6 `requests/q-4-from-20260101.utf16le.xml` again

   Each of the six is sent once. A non-zero curl exit, a status other than 200, a size that differs from CONTENT-LENGTH or an empty body = STOP (C3). **The 300 s rule:** a raw read that took more than 300 s from its start to the end of curl stops the sitting there: nothing more is sent to Tally until the run coordinator says. Go to step 4 only on the coordinator's word.
3. **The comparisons**, on the lab PC, after the last raw read (nothing is sent to Tally). For each of A2, A3, A4 and A6, `python -I read.py compare <A1's answer> <its answer> <its first day>`, then once more against A5's answer; record each line whole. Also `python -I read.py compare <A1's answer> <A5's answer> 20250401` (the full report twice: EXPECT `subset yes`). The line says, for the part:
   - its rows, and its rows dated before its first day;
   - the full report's rows dated from that day, and how many of those it holds;
   - the rows each holds that the other does not;
   - `subset yes` or `no`.

   A `compare` that fails (exit 1) is recorded with its line, and the others still run.
4. `GET /status` (200). Start ComplyEaze Bridge again; `ledger_masters` (basic): M1 and V1 equal to M0 and V0. Stop it (its stop file) and record its exit code and error stream. Hash the exe.
5. LAB-COMMON C10: at the Gateway a screenshot (Current Period and Tally's current date, unchanged from step 0), Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200. No XML request is sent after the shut; `GET /status` is the only request after it.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog; a second `tally.exe`; a mark that moves; an answer above its bound; a request without an answer in 10 minutes (leave that curl running; send nothing); Ctrl+Q or Escape at the Gateway (an incident: if a prompt shows, press no key, take a screenshot and STOP; only if the process list shows NO `tally.exe`, C9); an exe sha256 that is not the one named.

## What the row records, and what it does not
- For each of A1 to A6: the seconds to the first byte and in all, the bytes, and `read.py`'s line (the open bills and their sum).
- For each comparison: its line.
- **Reading it.** (1) holds when every part's line says `subset yes` with no row before its first day. (2) is the time of A2, A3 and A4 against their row counts, bracketed by A1 and A5 (the full report at the start and the end) and by A2 and A6 (the smallest part twice).
  - If the parts take about the full report's time whatever their rows, the time follows the book, not the rows.
  - If they fall with the rows, it follows the rows.
  - Anything between is recorded as measured, with no reading.
- **Not covered:** a split with an end date (none exists in this report), the `Ledger Outstandings` report, the Payable report (empty on this book), other `as_of` days, other books, Gold, and any change to the tool.

## Evidence
- Every tool answer whole, with its size and sha256.
- Every raw request, with its sha256.
- Every raw answer stays on the lab PC, listed by size and sha256: each may be several megabytes. Its headers, `read.py` line and `compare` lines are pushed.
- `times.txt`: per request, the UTC start and end, curl's exit code and `-w` fields, and CONTENT-LENGTH.
- The build's sha256 before and after.
- Screenshots listed by name, size and sha256, never pushed.
- A notes file with UTC times and one line per step, kept as C11 sets it out (no roles and no people).
- THE COMPANY's GUID is already in the merge gate's registry. No figure is recorded beyond the book's own synthetic ones, and `read.py` prints no name.
- The evidence goes on a new branch, `lab/2119-bills-split-quarters-answers`, with `SHA256.txt`. Send its SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes (C11).
