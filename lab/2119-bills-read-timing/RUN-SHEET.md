# 2119 Bills Receivable read timing (#2119): the request held past 20 s, sent with no client limit at four as-of days, then `outstandings` at three, read only, on the dense synthetic book

Not for merge. About 90 minutes, most of it Tally computing. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
On BRIDGE CORPUS DENSE, a call of the built tool `outstandings` for 31 March 2026 with a party detail asked for was refused on #2115's build: 31 requests were answered (18 posts and 13 status reads, 92 ms held in all), and the 32nd, a post of 770 bytes, was held 20,002 ms and cut at ComplyEaze Bridge's 20-second limit (`lab/1356-party-detail-by-ledger-answers`, D3). The same build answered the call for 30 June 2025 in 53.5 s; that call (D1) sent 114 requests, party voucher reads among them, so its 53.5 s is not the time of the bills read alone. The bills read this sheet times is the same code in the build named below (the function that sends it is unchanged between them; #2115 added a marks read that runs only when a party is asked for).

Which request the 32nd was is not recorded: the trail keeps kind, size and time, no fingerprint. It is inferred, and the position settles it. The Bills Receivable request for this company from 20250401 to 20260331, encoded as the transport sends it (UTF-16LE with a byte-order mark), is exactly 770 bytes (the Bills Payable one is 764). The code sends the identity and extent requests first (D3's seq 1 to 31: the currency read, the party marks read, the probe, the bracket and the extent pair), then the Receivable report as a pair, then the groups, then the Payable report, then the ledger snapshot; the refusal code `native_outstandings_read_failed` is raised only around that bills read. So seq 32 is the first post of the Receivable pair.

This sheet sends that request itself, by curl, with no client limit, at four as-of days, and times each answer against the number of open bills in it:
1. **R1 to R4:** Receivable to 30 June, 30 September, 31 December 2025 and 31 March 2026. R4 is the request the tool had held 20,002 ms. R4 under 20 s would not undo the identification above; it would point to a difference in Tally's state (what ran before, how long it had been up), which is a result to record.
2. **R5:** R4 sent again (does the time repeat?), **R6:** R1 sent again (does a small read stay small after the big ones?), **R7:** the Payable report to 31 March 2026.
3. **T1 to T3:** the built tool's `outstandings` at 30 September, 31 December 2025 and 31 March 2026, each followed by `egress_log`, to see on which of the three the tool answers and which it refuses, and which request is held when it refuses.

It decides no fix: whether the read can be bounded before it is sent, or refused with a narrower day, is for the issue.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step here on the synthetic company, on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs (its rule on names applies to what is pushed).

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic, about 30,000 vouchers), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, as the #1356 row left it: marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026. If it is absent, or step 1's marks differ (record the difference), STOP and report; the run coordinator decides. Only THE COMPANY is loaded, alone, from its load in step 0 to the last step.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Two kinds of read.** Tool reads are calls of ComplyEaze Bridge's built tool, one at a time (`list_companies`, `ledger_masters` basic, `outstandings`, `egress_log`). Raw reads by curl are allowed by the owner's decision of 26 Oct 2026 for shape discovery: each is a committed request in `requests/` (`gen_requests.py`), the exact text the tool itself sends for these reports, sent as C3 sets out.
2. **Closed set.** The report IDs are `Bills Receivable` and `Bills Payable`; the as-of days are 20250630, 20250930, 20251231 and 20260331; the from day is 20250401. Nothing read from Tally or from an answer adds to the set.
3. **Timing.** C3's line is sent with one named difference, in curl's output fields only (`-w` extended to `"%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}"`; the request sent is the same), and the UTC clock time is noted just before it starts and just after curl exits. Still no `--max-time`: a request without an answer in 10 minutes is C3's STOP (leave that curl running, send nothing, report). Size bound for each raw answer: 200,000,000 bytes (`size_download` above it = STOP before `read.py` is run, keeping the file); a missing CONTENT-LENGTH header is C3's "size that differs" = STOP, keeping the file.
4. **One request at a time.** While a curl or a tool call is running nothing else is sent to port 9001, `GET /status` included. During every request a screenshot of Tally's whole window is taken every 30 s, by screen capture only (no click, no focus change, no key); a dialog there is C7.
5. **After a refused tool call.** An abandoned request is still run by Tally and the next request waits behind it (#1713 measured this for a create; for a report read it is not measured). After a refusal for a deadline, wait the longer of 120 s and that day's raw read time (R2's for T1, R3's for T2, R4's for T3; 300 s if that raw read was not sent) plus 60 s, taking a screenshot every 30 s; then send ONE `GET /status` and record how long it takes to answer (C3's 10-minute rule applies to it). A quick 200 does not show that the abandoned read has finished: record it as measured and nothing more. Send the next step only after it answers 200.
6. **A refusal from the tool**, whatever its code or cause (`native_outstandings_read_failed` with `request_deadline_exceeded`, a size limit, a refusal before any request), is a result, not a STOP: record its code and cause. Any dialog is C7.
7. **Marks.** `ledger_masters` with `fields` basic (this book's masters read answers): `snapshot.master_alter_id`, `snapshot.voucher_alter_id`, `snapshot.reused` false. Read before the first send, after the raw reads and after the last, all equal.
8. **One Tally.** At step 0 and at the end, as C2 a and b set out plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.
9. **Keys.** The Gateway is the only screen used; no key is pressed except the arrows and Enter of Select Company at step 0, and the Company > Shut of C10. **Never Ctrl+Q or Escape at the Gateway of Tally.**

C2 and C1 as applied: step 0 is run with no company loaded and THE COMPANY is loaded straight after it (as in the #1356 sitting), so "only THE COMPANY is loaded" holds from that load. C2 d's MARKS read is met by step 1 (`list_companies` showing exactly one company, and the `ledger_masters` snapshot marks): no raw MARKS request is sent, since rule 2 allows no other raw request.

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` of master `609776ae9f5cec48e6563f165480d3fe1adae445` (merge-queue run 40366984202, artifact `unsigned-bundle-smoke-windows-latest` id 12670984633, `bridge_mcp.exe` sha256 `4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44`), the build the 2018 rows ran; it holds `outstandings` and `egress_log` (either absent from `tools/list` = STOP). Its Bills request code is the same as master's today. Record its sha256 before the first call and after the last: another sha256 = STOP. Over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2119\data`, listed first: absent or empty, else STOP), `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), redaction unset. Writes stay off.

## Files
`gen_requests.py`, `requests/` (the five generated requests), `read.py`, `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I gen_requests.py --check`: EXPECT exactly `gen_requests: ok (5 requests; Receivable 770 bytes, Payable 764 bytes)`.
3. `python -I gen_requests.py --out <a new empty folder>` and compare each file's sha256 with `SHA256.txt`.
4. There is no request-ID census: the requests are TYPE Data reports that define no collection.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded (C2 d's MARKS read is step 1): the screenshot, the process list and port 9001, `GET /status`. Hash the exe. Load THE COMPANY alone (arrow keys). Screenshot of the Gateway with the Current Period and Tally's current date (EXPECT 31 March 2026).
1. Start ComplyEaze Bridge. `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401. `ledger_masters` (basic): M0, the master mark, and V0, the voucher mark (EXPECT M0 328 and V0 29900; a voucher mark below 29900 = STOP, not this book).
2. **The raw reads**, in this order, each by C3's line with the rule 3 timing, each answer saved as received with headers, size and sha256; after each, `python -I read.py bills <answer>` (record the line whole), a screenshot of Tally's whole window, and the elapsed time from rule 3. Nothing is sent between a read and its screenshot.
   - R1 `requests/r-1-receivable-20250630.utf16le.xml` (EXPECT 770 B sent. The tool's answer for this day listed 7,432 open bills, receivable 336,858,579.88 and payable 0: EXPECT `BILLFIXED` 7432 and the sum of `BILLCL` equal to that figure in size, either sign. A difference is recorded and goes on.)
   - R2 `requests/r-2-receivable-20250930.utf16le.xml`
   - R3 `requests/r-3-receivable-20251231.utf16le.xml`
   - R4 `requests/r-4-receivable-20260331.utf16le.xml`
   - R5 `requests/r-4-receivable-20260331.utf16le.xml` again (the same file, the same hash)
   **The 300 s rule:** a raw read that took more than 300 s from its start to the end of curl stops the climb: the next heavier read is not sent until the run coordinator says, and neither is R5 (after R2: R3, R4 and R5 wait; after R3: R4 and R5 wait; after R4: R5 waits). Go on to R6 and R7.
   - R6 `requests/r-1-receivable-20250630.utf16le.xml` again
   - R7 `requests/r-5-payable-20260331.utf16le.xml` (764 B sent)
   Each of the seven is sent once. A non-zero curl exit, a status other than 200, a size that differs from CONTENT-LENGTH or an empty body = STOP (C3).
3. `GET /status` (200) and `ledger_masters` (basic): equal to M0 and V0.
4. **The tool reads**, one at a time, never repeated: `outstandings` with `company_guid` THE COMPANY's and `as_of` as below, nothing else (no party, no detail). Note the UTC time sent and the seconds taken. Right after each, `egress_log` with `limit` 50, saved whole. From the `response_prepared` record of that `outstandings` call: `request_trail` `sent`, `failed`, `omitted`, and for each kept entry its `seq`, `kind`, `request_bytes`, `response_bytes`, `held_ms` and `outcome` (a table; counts and sizes only). **EXPECT** (inference): a refusal for a deadline holds a 770 B post 20,002 ms at about seq 26, not 32, since this call names no party and so makes none of the six sends of the party marks read that D3 had before it; an answered call sends about 48 requests, so the last 32 kept begin near seq 17 and show the Receivable pair. A different seq is recorded, not a STOP. If a call is refused, apply rule 5 before the next.
   - T1 `as_of` 20250930
   - T2 `as_of` 20251231 (skipped, with a note, if R2 or R3 took more than 300 s)
   - T3 `as_of` 20260331 (skipped, with a note, if R2, R3 or R4 took more than 300 s, or if R4 was not sent)
   For an answered call record `open_bills_total`, the receivable and payable totals, `evidence.bytes` and `evidence.duration_ms`, and compare `open_bills_total` with the matching R's `BILLFIXED` and the totals with its `BILLCL` sum.
5. `ledger_masters` (basic): M1 and V1 equal to M0 and V0.
6. Stop ComplyEaze Bridge (its stop file) and record its exit code and error stream. Hash the exe. LAB-COMMON C10: at the Gateway a screenshot (Current Period and Tally's current date, unchanged from step 0), Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200. No XML request is sent after the shut; `GET /status` is the only request after it.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog; a second `tally.exe`; a mark that moves; an answer above its bound; a request without an answer in 10 minutes (leave that curl running; send nothing); Ctrl+Q or Escape at the Gateway (an incident: if a prompt shows, press no key, take a screenshot and STOP; only if the process list shows NO `tally.exe`, C9); an exe sha256 that is not the one named.

## What the row records, and what it does not
For each of R1 to R7: the seconds to the first byte and in all, the bytes, and the number of open bills and their sum; for T1 to T3: whether the tool answered, its seconds, and in a refusal the request held (kind, size, `held_ms`). The pairs the tool sends (each report twice) are not reproduced except by R4 and R5. The identification of the held request is supported further if R4 takes more than 20 s and a refused tool call holds a 770 B post 20,002 ms; a quick R4 would be a result about Tally's state, not a disproof. **Confidence:** one sitting, PARTIAL until repeated. Not covered: other days between these, the Payable report on a book with payables, the groups and ledger snapshot reads, a book with the same size and fewer open bills, Gold, and any change to the tool.

## Evidence
- Every tool answer whole with its size and sha256; every raw request (with its sha256); every raw answer stays on the lab PC, listed by size and sha256 (they may be several megabytes each), with its headers and `read.py` line pushed; `times.txt` with, per request, the UTC start and end, curl's exit code and `-w` fields and CONTENT-LENGTH; the table of each kept trail; the build's sha256 before and after; screenshots listed by name, size and sha256 (never pushed); a notes file with UTC times and one line per step, kept as C11 sets it out (no roles and no people).
- THE COMPANY's GUID is already in the merge gate's registry; no real figure is recorded beyond the book's own synthetic ones.
- Evidence on a new branch, `lab/2119-bills-read-timing-answers`, with `SHA256.txt`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes (C11).
