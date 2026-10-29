# 2119 Bills report 60-second limit (#2119): `outstandings` at 31 March 2026 on the dense synthetic book through the build that gives the two Bills report requests 60 seconds, once by the lab's stdio client and once through Claude Desktop's chat, read only

Not for merge. About 45 minutes. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
The build under test sends Tally's Bills Receivable and Bills Payable report requests with a 60-second limit; every other request keeps 20 seconds. On BRIDGE CORPUS DENSE at 31 March 2026, the Receivable request alone took 41.5 s and 36.7 s by curl, and the tool on a 20-second build refused the call on that post (`lab/2119-bills-read-timing-answers`, R4, R5 and T3). The tool sends each report twice with `GET /status` between and after, so an answered call there is expected to take about 85 s.

The row decides, for this build:
1. **S1:** whether `outstandings` at 31 March 2026 answers through the lab's stdio client, with its totals equal to R4's, and how long each request was held.
2. **S2:** whether a read sent straight after it (`ledger_masters`) answers.
3. **P1:** what a second ComplyEaze Bridge process sees when it calls `tally_status` 10 s into S1.
4. **D1:** whether Claude Desktop, as the host, waits for the same call or gives it up first, and after how long.

**The limit is held** (the release does not ship it as is; the run coordinator decides what follows) if any of these is so:
- **S1 is refused** with cause `native_bills_report_deadline_exceeded`: the coverage the limit was accepted on did not hold on this book.
- **Desktop gives the call up before Bridge answers.** The decision compares two times: when Desktop showed its error, and the `response_prepared` record of that call in D1's `egress_log`. An error shown after `response_prepared` (for example about the answer's size) is not a give-up; record it as a result.
- **Desktop's limit is not shown to be at least 90 seconds.** D1 passes only if Desktop waited at least 90 s for the call, or Desktop's own MCP log or settings show a tool-call limit of at least 90 s. A D1 that answers in under 90 s with no such log line shows only that the limit is longer than that call: record it, and the run coordinator decides.

It decides nothing about other books, other dates or Gold.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step here on the synthetic company, on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs (its rule on names applies to what is pushed). Claude Desktop is driven on the lab PC by the lab itself (owner, 11 Oct 2026).

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic, about 30,000 vouchers), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, as the 2119 timing row left it: marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026. If it is absent, or step 1's marks differ (record the difference), STOP and report; the run coordinator decides. Only THE COMPANY is loaded, alone, from its load in step 0 to the last step.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Tool reads only.** Every read is a call of ComplyEaze Bridge's built tool, one at a time per process (`list_companies`, `ledger_masters` basic, `outstandings`, `egress_log`, `tally_status`). No raw XML request is sent; `GET /status` by curl only where C2 and rule 5 name it.
2. **Closed set.** `as_of` is 20260331 for every `outstandings` call; no party, no detail, nothing else in its arguments. Nothing read from Tally or from an answer adds to the set.
3. **Two processes, once.** Only at P1 do two ComplyEaze Bridge processes run at the same time: the stdio client's (A) and a second stdio process (B) with the same environment but its own new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2119-60s\data-b`). B sends exactly one call, `tally_status`, and is then stopped. Claude Desktop's own ComplyEaze Bridge process (D) runs only for step 5, after A has been stopped.
4. **Screens.** During every tool call a screenshot of Tally's whole window is taken at about 20 s and about 40 s after the call is sent, and every 30 s after that until it ends, by screen capture only (no click, no focus change, no key). A dialog there is C7.
5. **After a refused or abandoned call.** An abandoned request is still run by Tally, and on this book a read sent straight after a cut was refused for time as well (`lab/0.7.0-master-live-checks-answers`). After a refusal for a deadline, or a call the host gave up, wait 180 s, taking a screenshot every 30 s; then send ONE `GET /status` and record how long it takes to answer (C3's 10-minute rule applies to it). A quick 200 does not show that the abandoned read has finished: record it as measured and nothing more. Send the next step only after it answers 200. S2 is the one exception: it is sent straight after S1 whatever S1's outcome, since what it does then is what S2 measures.
6. **A refusal from the tool**, whatever its code or cause, is a result, not a STOP: record its code and cause. Any dialog is C7.
7. **Marks.** `ledger_masters` with `fields` basic: `snapshot.master_alter_id`, `snapshot.voucher_alter_id`, `snapshot.reused` false. Read at step 1, at S2 and at step 6, all equal.
8. **One Tally.** At step 0 and at the end, as C2 a and b set out plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.
9. **Keys.** The Gateway is the only screen used; no key is pressed except the arrows and Enter of Select Company at step 0, and the Company > Shut of C10. **Never Ctrl+Q or Escape at the Gateway of Tally.**

C2 and C1 as applied: step 0 is run with no company loaded and THE COMPANY is loaded straight after it, so "only THE COMPANY is loaded" holds from that load. C2 d's MARKS read is met by step 1 (`list_companies` showing exactly one company, and the `ledger_masters` snapshot marks): no raw MARKS request is sent, since rule 1 allows none.

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` from the `unsigned-bundle-smoke-windows-latest` artifact of the pull request's CI run at the head that gives the Bills report requests 60 seconds (#2119). The run coordinator sends the head SHA, the run, the artifact id and the exe's sha256 with this sheet; record all four in the notes before step 0. Record the exe's sha256 before the first call and after the last: another sha256 = STOP. It must list `outstandings`, `egress_log` and `tally_status` (one absent from `tools/list` = STOP).

Processes A and B: over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), redaction unset, writes off; A's `BRIDGE_AGENT_DATA_DIR` is `C:\bridge-lab\2119-60s\data-a` and B's is `C:\bridge-lab\2119-60s\data-b`, each listed first: absent or empty, else STOP.

Process D (step 5): one Claude Desktop entry for ComplyEaze Bridge pointing at the same exe, with the same environment and `BRIDGE_AGENT_DATA_DIR` `C:\bridge-lab\2119-60s\data-d` (absent or empty, else STOP), writes off. Record the Claude Desktop version (Help > About, or the app's own about screen) and the entry exactly as configured, with no other value in it. While step 5 runs, no other Claude Desktop entry for ComplyEaze Bridge is enabled. Record which entries were enabled before, and put them back as they were at step 6.

## Files
`RUN-SHEET.md`, `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. Download the artifact the run coordinator named and check the exe's sha256 against the one sent.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded (C2 d's MARKS read is step 1): the screenshot, the process list and port 9001, `GET /status`. Hash the exe. Load THE COMPANY alone (arrow keys). Screenshot of the Gateway with the Current Period and Tally's current date (EXPECT 31 March 2026).
1. Start process A. `tools/list` (saved whole). `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401. `ledger_masters` (basic): M0, the master mark, and V0, the voucher mark (EXPECT M0 328 and V0 29900; a voucher mark below 29900 = STOP, not this book).
2. **S1 and P1.** Note the UTC time, then send through A `outstandings` with `company_guid` THE COMPANY's and `as_of` 20260331, nothing else. Take rule 4's screenshots.
   - **P1:** 10 s after S1 is sent (by the clock, not by anything Tally shows), start process B and send through it `tally_status` with no arguments; note the UTC time it is sent and the seconds it takes. Save its answer whole. Stop B (its stop file) as soon as it has answered, and record its exit code and error stream. B sends nothing else. **EXPECT** (inference, not measured): B waits at most about 9.5 s for the wire lock and is refused busy (`tally_endpoint_busy`), since A holds the lock for the whole of each post; an answer from B is recorded as a result, with its seconds, and is not a STOP.
   - When S1 ends, note the UTC time and its seconds. **EXPECT:** answered (`isError` false) in about 70 to 100 s; `open_bills_total` 29,900 (R4's `BILLFIXED`); `receivable` equal to the size of R4's `BILLCL` sum in `lab/2119-bills-read-timing-answers/RESULTS.md`; `payable` 0. A refusal is a result: record its code, cause and next step text whole.
3. **S2.** Straight after S1 ends, with nothing sent between, `ledger_masters` (basic) through A: note the seconds and the marks (M1, V1: EXPECT equal to M0 and V0). A refusal is a result: record its code and cause, then apply rule 5 before step 4.
4. Through A: `egress_log` with `limit` 50, saved whole. From the `response_prepared` record of S1: `request_trail` `sent`, `failed`, `omitted`, and for each kept entry its `seq`, `kind`, `request_bytes`, `response_bytes`, `held_ms` and `outcome` (a table; counts and sizes only). **EXPECT** (inference): the Receivable pair two 770 B posts, each held between 20,000 and 60,000 ms, with a `GET /status` between and after; the Payable pair two 764 B posts; every other post held under 20,000 ms. Then stop process A (its stop file) and record its exit code and error stream. Wait 120 s with a screenshot every 30 s, then ONE `GET /status` (200).
5. **D1, through Claude Desktop.** Enable process D's entry only and start a new chat. Check that the chat lists ComplyEaze Bridge's tools; then send exactly: `Call the ComplyEaze Bridge tool outstandings with company_guid d45bc1b0-e5e3-4261-b3b2-cce3915f42d3 and as_of 20260331, and nothing else. Do not call any other tool first. Then show me exactly what the tool returned, or the exact error you received.` Approve the one `outstandings` call if Desktop asks; approve nothing else. Note the UTC time of the approval (or of the send, if no approval was asked), take rule 4's screenshots of Tally, and a screenshot of Desktop's chat every 30 s until the reply ends.
   - If Desktop's model asks to call `list_companies` first, approve that one call (the server's own instructions ask for it) and nothing else besides `outstandings`; record it. Any other tool asked for: deny it, and send the prompt again once in the same chat.
   - Record: whether Desktop showed the tool's answer or an error; the error text exactly, if any; the UTC time it was shown; the seconds from the approval to the tool result appearing (or the error); whether `open_bills_total` and the totals shown equal S1's.
   - Copy from Desktop's own MCP log for this server (on Windows under `%APPDATA%\Claude\logs\`, the `mcp*.log` files) the lines from the `tools/call` of `outstandings` to its result or cancellation, whole, and any line naming a timeout. Record any tool-call limit Desktop's settings show. Neither is pushed if it holds a user name or a machine path beyond the lab folders: list it by size and sha256 instead, and copy only the timing lines with those parts replaced by `<redacted>`.
   - If Desktop shows an error before the tool answered: apply rule 5, then send in the same chat exactly `Call the ComplyEaze Bridge tool egress_log with limit 50 and show me its answer whole.` and save it; from it, record whether S1's twin call reached `response_prepared` and its trail as in step 4. Otherwise send the same `egress_log` line and save its answer.
   - Disable D's entry and put the entries back as they were. Wait 120 s with a screenshot every 30 s, then ONE `GET /status` (200).
6. Start process A again on its data folder; `ledger_masters` (basic): M2 and V2 equal to M0 and V0. Stop A. Hash the exe. LAB-COMMON C10: at the Gateway a screenshot (Current Period and Tally's current date, unchanged from step 0), Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200. No XML request is sent after the shut; `GET /status` is the only request after it.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog in Tally; a second `tally.exe`; a mark that moves; a tool call without an answer in 10 minutes (stop nothing; send nothing; report); Ctrl+Q or Escape at the Gateway (an incident: if a prompt shows, press no key, take a screenshot and STOP; only if the process list shows NO `tally.exe`, C9); an exe sha256 that is not the one named.

## What the row records, and what it does not
For S1: the outcome, its seconds, its totals against R4's, and every kept request's `held_ms`; for S2: the outcome and its seconds; for P1: B's outcome and seconds; for D1: Desktop's version, whether it waited for the answer, the seconds, its error text if any, and its log's timing lines. **The limit is held** on any of the three rules under "What it decides". **Confidence:** one sitting, PARTIAL until repeated. Not covered: other days, other books, a host other than this Claude Desktop version, the Payable report on a book with payables, Gold.

## Evidence
- Every tool answer whole with its size and sha256 (A's, B's and the two Desktop answers as Desktop showed them, copied as text); the table of S1's trail; the build's head SHA, run, artifact id and sha256 before and after; Desktop's version and its entry as configured; screenshots listed by name, size and sha256 (never pushed); a notes file with UTC times and one line per step, kept as C11 sets it out (no roles and no people).
- THE COMPANY's GUID is already in the merge gate's registry; no real figure is recorded beyond the book's own synthetic ones.
- Evidence on a new branch, `lab/2119-bills-60s-row-answers`, with `SHA256.txt`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes (C11).
