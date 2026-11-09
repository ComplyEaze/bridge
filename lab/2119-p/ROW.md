# Row 2119-P: `outstandings` with `detail` `open_bills` for one named party, at 31 March 2026 on the dense synthetic book, through master's build: does each call fit 20 s per request and 60 s per call

Not for merge. Part of #2119. About 30 minutes. **READS ONLY:** nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). Lines are cited at master d04f67a9. Confidence of everything here until the row runs: unverified.

## Question
#2383 (merged as 6805d78f) answers one party's open bills through Tally's Ledger Outstandings report, with no whole-book Bills report and no voucher read. Its party read was timed by curl at about 1 to 2 s (rows 2119-L and 2377-B, partial). The build's own call was never run against Tally. Besides that report, each call sends at least:
- the company's identity and its marks, before and after (`agent_party_bills.rs:56`, `:60`, `:88`);
- the ledger catalogue, to resolve the party (`:65`), before the runtime read (`:81`);
- the mode and identity brackets and the book extent, before and after (`runtime_party_bills.rs:72-78`, `:109`, `:124`), and the read boundary again at the end (`:127`);
- a paired `List of Ledgers` snapshot from the books' first day to `as_of`, unfiltered (`:85`), then the paired Ledger Outstandings (`:97`).

No row has timed that snapshot on this book at a year-end date. So: on BRIDGE CORPUS DENSE at 31 March 2026, does each such call answer, tie, hold every request under 20 s, and finish under 60 s, the Windows desktop app's call limit (row H)?

## What it decides
**The rows are read from the top; the first that matches decides.**

| # | this row shows | then |
|---|---|---|
| 1 | a call answers, and any EXPECT line of its step is not met (bill count, On Account rows, the sum of `closing`, `state`, `basis`, `checks`, `ledger_match.matched`) | A wrong or unexpected answer is worse than a refusal. Goes to the run coordinator as it is; #2119 stays open. |
| 2 | a call is refused, and the request its trail shows as held or failed is, by position, one of the two snapshot posts (step 2) | The next build is the GUID-filtered snapshot that #2383 deferred, measured against the same tie. #2119 stays open for it. |
| 3 | a call is refused for any other cause, `agent_response_too_large` and `party_name_not_admitted` included, or its trail cannot be told apart by position | Goes to the run coordinator with its code, cause and trail. #2119 stays open. |
| 4 | a call was skipped (its name file absent, or the name failed the pre-check) | That party is not measured. If the other call meets row 6, the result goes to the run coordinator, who decides whether one party is enough; otherwise the other call's row decides. |
| 5 | both calls answer as EXPECT, every kept request is held under 20,000 ms, but a call's whole time is 50 s or more | The call fits the request limit but not the Windows call limit with a margin. Goes to the run coordinator with the trail; #2119 stays open. |
| 6 | both calls answer as EXPECT; each trail has `failed` 0 and `omitted` 0 and every request held under 20,000 ms; and each call's whole time is under 50 s | **#2119 may close on #2383 for a named party**, on this one sitting and two parties, if the maintainers agree. The no-party read, and `bill_trail` and `unadjusted` at a year-end date on a book this size, are recorded as known limits, each refused with its next step, and the whole-book case is left to #703. |

50 s is the 60 s Windows limit less a 10 s margin, because a person may answer the tool prompt slowly and whether that counts is not known (row H, Limits). It decides nothing about other books, other dates, a party with no open bills, a host's own timing, or Gold.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy (sha256 5370c6d2…, as in `lab/2119-h-sheet`). Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step, on the synthetic company, on the owner's standing word of 27 Oct 2026. No person is needed: the calls go through the lab's own stdio client, not the desktop app.

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic, about 30,000 vouchers), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026, as row 2377-B left it on 8 Nov (`lab/2377-b-answers`, D5). If it is absent, or step 1's marks differ (record the difference), STOP and report. Only THE COMPANY is loaded, alone, from its load in step 0 to the last step.

**The two parties.** Their names stay on the lab PC and are pushed only as `p1` and `top1`. Each name is the file's first line with exactly one line terminator removed (`\r\n` or `\n`), nothing else trimmed. Before its call, the name is checked against the tool's name rule (`LedgerScopeName`, `src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65-94`): ASCII letters and digits with single inner spaces, no leading or trailing space, within its length limit and not one of its formula words. Both rows that picked these parties picked from that alphabet, so a failure is not expected; a name that fails is not sent, and that call is skipped (row 4).
- `p1`: the name in `party.txt` of row 2119-L (4 Nov; its RESULTS: 248 bills in the Receivable report, Ledger Outstandings sum −11,815,459.60, no On Account row).
- `top1`: the first name in `top3.txt` of row 2377-B (8 Nov, D3; its RESULTS: 250 bills, `on_account 0`; its sum stays on the lab PC with that answer).
- A file not on the lab PC, or a name not readable from it: that party's call is skipped, and the row records why. No other party is picked, and nothing is read from Tally to find one.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Tool reads only.** Every read is a call of ComplyEaze Bridge's built tool, one at a time, through one stdio process (A): `tools/list`, `list_companies`, `ledger_masters` with `fields` basic, `outstandings`, `egress_log`. No raw XML request is sent; `GET /status` by curl only where C2 and rule 4 name it.
2. **Closed set.** The two `outstandings` calls are exactly `{"company_guid": "d45bc1b0-e5e3-4261-b3b2-cce3915f42d3", "as_of": "20260331", "party": "<the name>", "detail": "open_bills"}`, with the name as the rule above gives it. Nothing read from Tally or from an answer adds to the set.
3. **Screens.** During each `outstandings` call, a screenshot of Tally's whole window at about 10 s and 30 s after it is sent, and every 30 s after that until it ends, by screen capture only (no click, no focus change, no key in Tally). A dialog there is C7. This book has had Ledger Outstandings by curl (rows 2119-L and 2377-B), not from this build; each call is a NEW SHAPE under C3, so a screenshot follows it before the next request.
4. **After a refusal.** A refusal of any code is a result, not a STOP: record its code, cause and next step text whole. After a refusal for a deadline (`request_deadline_exceeded`, or any cause that names a time), wait 180 s with a screenshot every 30 s, then ONE `GET /status` (C3's 10-minute rule applies) and record how long it took; the next call is sent only after it answers 200. A quick 200 does not show that the abandoned read has finished; record it as measured and nothing more.
5. **Marks.** `ledger_masters` with `fields` basic: `snapshot.master_alter_id`, `snapshot.voucher_alter_id`. Read at step 1 and at step 5, equal.
6. **One Tally.** At step 0 and at the end, C2 a and b: exactly one `tally.exe`, and it owns port 9001.
7. **Keys.** In Tally the Gateway is the only screen used. THE COMPANY is loaded from Select Company with the arrow keys and Enter, Enter only once a screenshot shows the highlighted row reads `BRIDGE CORPUS DENSE`. At the close it is shut by Company, then Shut from the Gateway's menu. **Never Ctrl+Q or Escape at the Gateway.** No other key or click in Tally.

C2 d as applied: step 0 runs with no company loaded, and its MARKS read is met by step 1 (`list_companies` showing exactly one company, and the `ledger_masters` marks); no raw MARKS request is sent, since rule 1 allows none.

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` from the `unsigned-bundle-smoke-windows-latest` artifact of the CI run of a master push at or after 6805d78f (the merge of #2383). The run coordinator sends the commit, the run, the artifact id and the exe's sha256 with this sheet; record all four before step 0. Copy it to `C:\bridge-lab\2119-p\bin\bridge_mcp.exe`. Hash it before the first call and after the last: another sha256 = STOP.

Process A: over stdio, as the lab's stdio client runs it in earlier rows, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), redaction unset, writes off, and `BRIDGE_AGENT_DATA_DIR` `C:\bridge-lab\2119-p\data-a`, listed first: absent or empty, else STOP. `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_MAX_ROWS` are unset (the defaults). Before step 1 and after step 6, `tasklist /FI "IMAGENAME eq bridge_mcp.exe" /FO CSV /NH` is recorded; a `bridge_mcp.exe` that is not A's is recorded, and A's own requests are told from it by A's egress log. The stdio client records, for each call, the UTC time it was written and the UTC time its answer was read, to the millisecond: that difference is the call's whole time.

## Files
`ROW.md`, `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt` (each line: sha256, bytes, path). Check every file against `SHA256.txt` first.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded (C2 d's MARKS read is step 1): the screenshot, the process list and port 9001, `GET /status`. Hash the exe. Load THE COMPANY alone (rule 7). Screenshot of the Gateway with Tally's current date (EXPECT 31 March 2026; another date = STOP).
1. Start A. `tools/list`, saved whole: it must list `list_companies`, `ledger_masters`, `outstandings` and `egress_log`, and `outstandings`'s `detail` must offer `open_bills`; one absent = STOP. `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401. `ledger_masters` (basic): M0 and V0, EXPECT 328 and 29900; another value = STOP.
2. **P-top1.** `outstandings` for `top1` (rule 2). Rule 3's screenshots. Record the whole time.
   - **EXPECT** (inference from row 2377-B's curl read, unverified through this build; each `closing` is a decimal string, summed as decimals): `isError` false; `result.state` `observed`, `result.basis` `tally_ledger_outstandings`, `result.detail` `open_bills`, `result.as_of` `20260331`; `result.bills` 250 rows; `result.on_account` empty; `result.checks.bills_sum_to_ledger_closing` `tied` and `result.checks.company_marks_unchanged` `checked`; `result.ledger_match.matched` `exact` (`agent_party_bills.rs:103-117`). The sum of `result.bills[].closing` equals the sum row 2377-B kept on the lab PC for `lo-top1`; it is recorded there, and only equal or not is pushed.
   - Then `egress_log` with `limit` 50, saved whole. From P-top1's `response_prepared` record, `request_trail`: `sent`, `failed`, `omitted`, and for each kept entry its `seq`, `kind`, `request_bytes`, `response_bytes`, `held_ms` and `outcome` (a table; counts and sizes only). The trail holds no request body or name, and its `kind` is only `post` or `status`, so the pairs are told **by position**: the posts run in the order of the code above, and within a pair the two `request_bytes` are equal. Mark the snapshot pair (the first pair of equal-sized posts after the extent read, `runtime_party_bills.rs:85`) and the Ledger Outstandings pair (the next, `:97`; its `response_bytes` near the 205 to 207 KB rows 2119-L and 2377-B saw). Cross-check by `read_evidence` where its entries carry the same `seq` and time. Say in the notes that the pairs are told by inference from position and size. A trail whose posts cannot be lined up so is recorded as it is (row 3).
3. **P-p1.** The same for `p1`, after rule 3's screenshot (and rule 4's wait if P-top1 was refused for a deadline).
   - **EXPECT** (row 2119-L, curl, partial): `result.bills` 248 rows; `result.on_account` empty; the sum of `result.bills[].closing` −11,815,459.60; the rest as P-top1.
   - Then `egress_log` with `limit` 50, and the same table for P-p1.
4. Wait 60 s with a screenshot every 30 s, then ONE `GET /status` (200).
5. `ledger_masters` (basic): M1 and V1, EXPECT equal to M0 and V0; a difference = record and STOP.
6. Stop A (its stop file); record its exit code and error stream. Hash the exe. LAB-COMMON C10: a screenshot at the Gateway with the current date unchanged; Company > Shut; Select Company with none loaded; one `tally.exe`; 9001 LISTENING; `GET /status` 200. No XML request after the shut.
7. Notes, the pre-push scan and the evidence commit (Keep); send its sha to the run coordinator; push on its yes.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog in Tally; a second `tally.exe`; a mark that moves; a tool call without an answer in 10 minutes (stop nothing, send nothing, report); Ctrl+Q or Escape at the Gateway; an exe sha256 not the one named.

## What to report
| step | result |
|---|---|
| build: commit, run, artifact id, exe sha256 before and after | |
| step 1: tools, company, M0 and V0 | |
| P-top1: outcome, whole time, bills, On Account, tie, sum equal or not, `ledger_match` | |
| P-top1 trail: the table, with the snapshot pair and the Ledger Outstandings pair marked, and how | |
| P-p1: the same | |
| P-p1 trail: the same | |
| the largest `held_ms` of each call, and which request | |
| M1 and V1; close | |
| any dialog, skip or deviation | |

Read every number from its file. Put any STOP, skip, or EXPECT not met in the first line. Fill the "What it decides" row this result meets.

## Limits
One sitting, one book, one date, two parties chosen by bill count, one build. The whole time is the stdio client's, not a desktop app's; the desktop app adds its own overhead, which is not measured here. A party with fewer bills is not timed; its snapshot is the same request.

## Keep
On `lab/2119-p-answers`, with `SHA256.txt`:
- every tool answer whole, with its size and sha256, except the two `outstandings` answers and the `egress_log` answers, which hold the parties' names and are listed in `kept-local.txt` by size and sha256;
- the two trail tables, the two calls' outcome lines (counts, equal or not, the tie, the whole time) and the marks;
- the build's commit, run, artifact id and sha256;
- a notes file with UTC times and one line per step, and a `RESULTS.md` holding the table above.

Screenshots stay on the lab PC, listed by name, size and sha256. Notes carry no user names and no machine paths beyond `C:\bridge-lab\2119-p\`. The party names are pushed only as `p1` and `top1`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
