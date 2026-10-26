# #648: what it costs to date an asset ledger's movements by trial-balance windows, read through the built tools on two synthetic books (READ-ONLY)

Not for merge. Written 26 Oct 2026 from issue #648's design of 25 Sep 2026 ("Dating movements without a bulk voucher export": a year's trial balance first, then months, then a bisection within a month, then one-day vouchers), whose sizing needs a measured read cost before the composed read is built (AGENTS.md P1). `SHA256.txt` lists every file here; check it before step 0.

## Rules and companies
`LAB-COMMON.md` in this folder applies in full to each part, with that part's company as THE COMPANY. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **No raw requests.** No request file is sent by curl. C2 step 0 runs WITHOUT its MARKS request (its GET /status checks still run). Every read is a call of ComplyEaze Bridge's built tool, one at a time.
2. **Marks.** M (ALTMSTID) and V (ALTVCHID) are read with `ledger_masters` (first page, no `snapshot_id`): `snapshot.master_alter_id` and `snapshot.voucher_alter_id`; record the whole answer and `snapshot.reused`, which must read false.
3. **Nothing is written:** no backup and no restore. A mark that moves is a STOP.
4. **The cost of every call is recorded:** the wall-clock seconds from the call to its answer (the operator's clock), the answer's size in bytes, and then `egress_log` (which sends nothing to Tally): the lines that call added, with the number of requests it sent and each request's and response's bytes as the log gives them.
5. **Timeouts.** The built tool abandons a request that passes its own 20-second deadline (the request keeps running in Tally). An answer holding `request_deadline_exceeded` or `response_size_limit_exceeded`, a call with no answer within 10 minutes, or any dialog on Tally's screen = STOP: nothing is sent again, no key is pressed, the screen is photographed, and the answer is kept whole.
6. **Who acts.** The operator alone. Grant: reads only, about 30 tool calls, one at a time.

Licensed TallyPrime only; Educational mode = STOP. Record the release from Tally's About screen (the lab PC runs 7.2 since 25 Oct 2026). In licensed mode `trial_balance` admits any day; a refusal of a window (`trial_balance_before_books`, `trial_balance_education_unqualified`) is recorded whole as a result, and that window is "not established".

## Build
The build staged at the lab as `w1-2306d86d`: the Windows bundle of master `2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb`, Bridge CI run `40077056130`, artifact `12553782422`; its `bridge_mcp.exe` sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524`. Hash the exe before the first call; any other hash = STOP. Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` per part, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), and posting, import and redaction unset.

## Part A: BRIDGE GST LEDGER LAB, one asset ledger that moved once
THE COMPANY is `BRIDGE GST LEDGER LAB`, the synthetic book GS1-L1 created (its GUID as GS1-L1 step 1 recorded it), with books from 1 April 2025. Its ledger `GS1 Computer`, under Fixed Assets, moved once in the year: the Purchase of 5 November 2025, `GS1 Computer` Dr 40,000.00 (`lab/gs1-l1-answers`, RESULTS.md, "Vouchers in the book"). The book is read between the items of its own sequence (628-C0, 628-C0b, 1867-L3 and those after), never while one of them is under way.

A0. Load THE COMPANY alone (C1), then C2 step 0 without its MARKS request. `list_companies`: EXPECT exactly THE COMPANY. `tally_status`: record release, licence and Education mode. Marks: M0, V0.

A1. `ledger_masters` with `fields` `basic`, `group` `Fixed Assets` and `group_scope` `ancestry`: record the answer whole and its cost. EXPECT `GS1 Computer` among its rows; record every row.

A2. **The year.** `trial_balance` from `2025-04-01` to `2026-03-31` (offset 0, then every page if it pages): record each Fixed Assets ledger's row whole (opening, debit, credit and closing as the tool names them) and the cost of each page.

A3. **The months.** `trial_balance` for each month from April 2025 to March 2026, each from its first to its last day (`2025-04-01` to `2025-04-30`, … , `2026-03-01` to `2026-03-31`): twelve calls. Record for each the Fixed Assets ledgers' rows and the cost. EXPECT `GS1 Computer` to move in November 2025 only; record any other month in which a Fixed Assets ledger moved.

A4. **The bisection, in each month in which an asset ledger moved.** Start with the month's window [first, last]. Call `trial_balance` for its first half, from the first day to the middle day m (m = the first day plus half the window's days, rounded down). If the ledger moved in that half, the next window is [first, m]; otherwise it is [m+1, last]. Repeat until the window is one day. Record every window, whether the ledger moved in it, and its cost. EXPECT about five calls for November, ending on 5 November 2025.

A5. **The day.** `vouchers` for that day only (from = to = the day A4 ended on): record the answer whole and its cost. EXPECT the Purchase of 5 November with `GS1 Computer` Dr 40,000.00 among its entries.

A6. Marks: M1, V1, EXPECT equal to M0, V0. LAB-COMMON C10.

## Part B: BRIDGE LARGE LAB, the cost of a window on a book of many vouchers and few ledgers
THE COMPANY is `BRIDGE LARGE LAB`, GUID `2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab`, books from 1 April 2026, built on 21 Oct 2026 with 29,930 Journals (the last dated 31 March 2027), and any vouchers later items posted into it. #648's design says a trial balance's cost scales with the book's ledger count, not the window; this part records whether, on a book of about 30,000 vouchers and few ledgers, the window's length changes the cost.

B0. Load THE COMPANY alone, then C2 step 0 without its MARKS request. `list_companies`: EXPECT the GUID above. Marks: M0, V0.
B1. `trial_balance` from `2026-04-01` to `2027-03-31` (every page): record the cost and the row count.
B2. `trial_balance` from `2027-03-01` to `2027-03-31`: record the cost and the row count.
B3. `trial_balance` from `2027-03-31` to `2027-03-31`: record the cost and the row count.
B4. B1 again (a second timing of the year).
B5. Marks: M1, V1, EXPECT equal to M0, V0. LAB-COMMON C10.

STOP RULES: C7 and C8; Difference 5; a mark that moves; a company other than THE COMPANY in a company list.

## What this gives, and its limit
The number of calls and the cost (time, requests, bytes) of each step of #648's dating sequence on a real Tally, one asset ledger dated to the day, and how a window's length bears on a trial balance's cost on a book of many vouchers. Not measured: a book of thousands of ledgers (#648's sizing case; its real-book benchmark needs the owner's consent), several movements in one month, a disposal, Gold, any release other than the one recorded.

## Evidence
Every tool answer whole with its size and sha256, the `egress_log` lines of every call, the cost table (call, window, seconds, requests, bytes), the marks, and a notes file with UTC times and one line per step. Evidence on a new branch `lab/648-asset-dating-read-cost-answers` with `SHA256.txt`; send its SHA to the run coordinator before pushing, and push only after the pre-push scan and the run coordinator's yes.
