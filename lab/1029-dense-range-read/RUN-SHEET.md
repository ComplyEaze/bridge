# 1029 item 2 (#1029): a range of several days read in ONE CALL, on a book of real volume (READ-ONLY)

Not for merge. Written 26 Oct 2026. `SHA256.txt` lists every file here with its bytes; check it before step 0.

Issue #1029 item 2 says a range of several days read in one request "has not been run against a real TallyPrime with the new rule". The earlier plan for it built a new 30,000-voucher book from generated import files sent raw; that route is closed (AGENTS.md P7: requests reach Tally only from the product's builders, through its one dispatch path). This item uses the book the lab already holds, `BRIDGE CORPUS DENSE`, and only the build's own tools. It writes nothing.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

THE COMPANY is `BRIDGE CORPUS DENSE`, with the company number the lab recorded for it (Part C of the answers below) and GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, as the lab recorded it on 26 Oct 2026 (`lab/1240-empty-window-live-answers`, Part C): a voucher mark of 29,900 at about 82 vouchers a day, the last on 31 Mar 2026, the last 82 voucher numbers contiguous (29819 to 29900). If the company list shows another number or GUID: STOP.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **No raw requests.** The operator sends no request file. Every read is a call of ComplyEaze Bridge's built tool, one at a time. C2 step 0 runs WITHOUT its MARKS request; its GET /status checks still run.
2. **Marks.** `tally_status` carries no marks. M (ALTMSTID) and V (ALTVCHID) are read with `ledger_masters` (first page, no `snapshot_id`): `snapshot.master_alter_id` and `snapshot.voucher_alter_id`; record the whole answer and `snapshot.reused`, which must read false.
3. **No backup and no restore:** nothing is written. A mark that moves is a STOP.
4. **Who acts.** The operator loads and shuts THE COMPANY and reads the Day Book, one act at a time. No owner act is needed.

Licensed mode only (the window's days are ordinary days); Education mode = STOP.

## Build
The build already staged at the lab as `w1-2306d86d`: the Windows bundle of master `2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb`, Bridge CI run `40077056130` (event `schedule`), artifact `12553782422` (zip digest `sha256:74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e`); its `bridge_mcp.exe` as the lab recorded it on staging: sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524`. Before the first call hash the exe that will run; any other hash is a STOP.

Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, one new empty `BRIDGE_AGENT_DATA_DIR` for the item, `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), and `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_IMPORT` and `BRIDGE_AGENT_REDACTION` unset.

**One call, many requests.** On this book a voucher part holds at most a few dozen vouchers (the build's per-voucher size budget), so one day's vouchers alone need more than one part and no part can span two days: the one call sends several data requests, one or more per day. This item measures the ONE CALL on a seven-day window; it does not claim a single Tally request.

**Timeouts.** The built tool has its own 20-second deadline for each request it sends and abandons a request that passes it (the request keeps running in Tally). Any answer holding `request_deadline_exceeded` or `response_size_limit_exceeded`, any call with no answer within 10 minutes, and any dialog on Tally's screen is a STOP: nothing is re-sent, no key is pressed, the screen is photographed, and the answer is reported whole. `voucher_window_volume_unestimated` (cause `census_deadline_exceeded` or `census_response_too_large`) is also a STOP. A refusal for size (`voucher_window_part_over_budget`, `voucher_window_too_many_reads`) is a RESULT, not a STOP: record it whole. `voucher_window_part_not_admitted` means the book changed during the read: nothing writes here, so it is a FINDING, reported whole and not repeated.

## Steps
0. Load THE COMPANY alone (Select Company), then C2 step 0 without its MARKS request. `list_companies`: EXPECT exactly THE COMPANY with the GUID above and `books_from` 20250401 (another books_from = STOP: the window would not be the book's first days). `tally_status`: record release, licence and Education mode. `ledger_masters`: M0, V0 (EXPECT V0 about 29,900).
1. **Oracle, on Tally's own screen.** Tally's Statistics report (the count of vouchers per voucher type) for the period 1-4-2025 to 7-4-2025 (Alt+F2 for the period; record the menu path the screen shows and Tally's current date before and after). Record every type's count and a screenshot: N is their total. Cross-check: the Day Book for the same period, the voucher number and date of its FIRST and LAST row (a screenshot of each end; never Enter on a row); if every voucher is of one type and the numbers run without a gap, last - first + 1 should equal N. If the two disagree, or the report cannot be shown for the period, record what each shows and write "no oracle". Set the periods back as the screens offer; set Tally's current date back to step 0's if it moved.
2. `voucher_read_cost` with `company_guid`: record the whole answer, in particular, for `vouchers` under `per_call`, `census_reads` (4 expected for this mark) and `floor_seconds`.
3. `vouchers` with `company_guid`, `from` `2025-04-01`, `to` `2025-04-07`: ONE call, offset 0, nothing else. Time it (wall-clock seconds). Record the whole answer (`state`, `total`, `result.window` with its census and every entry of its parts, `evidence`) and its `egress_log` lines (the egress log keeps each request this call sent, of which the last 32). Do not ask for later pages: the first page is trimmed to the tool's byte limit, and `total` counts every voucher of the window.
4. `read_evidence` with `limit` 40: record it whole (one record per tool call, with its fingerprints, bytes and state).
5. `ledger_masters`: M1, V1. EXPECT equal to M0, V0.
6. LAB-COMMON C10: shut THE COMPANY; no company loaded; Tally's current date as at step 0.

EXPECT (record, not predicted):
- step 3 `state` `complete`; `total` = N (compare `total`, not the rows on the first page, which holds at most 500);
- no `voucher_window_part_not_admitted`;
- the call's census request count (`result.window.census`) equals step 2's `census_reads`;
- every entry of `result.window.parts` covers a single day (a part spanning two days is a FINDING against the build's per-voucher budget);
- the wall-clock seconds not less than step 2's `floor_seconds` (less means the floor is wrong: FINDING);
- step 4's record of the step 3 call in state complete.

PASS = steps 0 to 5 recorded whole and every EXPECT line above checked. `total` different from N, a `partial` state, or a refusal is the finding for #1029 item 2: report it whole.

## Not measured by this item
#1029 item 1 (small companies are not counted first) and item 3 (how often the read-back after an approved batch post is refused): item 3 posts into a book of this volume, and whether those posts may go into `BRIDGE CORPUS DENSE` (with its backup and restore) is for the run coordinator to decide; it is not written here. A window crossing a month or a financial year; Gold; Education mode.

## Evidence
Every tool answer whole with its size and sha256, the Day Book readings and screenshot names (screenshots stay on the lab PC), the wall-clock times, the marks, and a notes file with UTC times and one line per step. Evidence on a new branch `lab/1029-dense-range-read-answers` with `SHA256.txt`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
