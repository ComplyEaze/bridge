# Row 703-LO: one party's open bills through `outstandings`, for the smallest, median and largest party on the dense synthetic book: how long does one per-party step take

Not for merge. Part of #703. About 30 minutes. **READS ONLY:** nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). Lines are cited at master 10f38816. Confidence of everything here until the row runs: unverified.

## Question
The 8 Nov 2026 design comment on #703 proposes Shape A, a stepped read: each call does a bounded amount of Tally work, a budget below the smallest measured host limit (for example 45 s), and bills at year end become per-party Ledger Outstandings reads. It asks next for per-party Ledger Outstandings on BRIDGE CORPUS DENSE for the smallest, median and largest party, to size that step. Row 2119-P (9 Nov 2026, `lab/2119-p-answers`) timed the build's `outstandings` party call for two of the largest parties (250 and 248 bills): 5.1 s and 4.4 s a call, each Ledger Outstandings post about 1.0 s to 1.3 s. No party with fewer bills was timed through the build.

So: for the party with the fewest bills, the median party and the party with the most, how long does one `outstandings` call with `detail` `open_bills` take as a whole, how long does its Ledger Outstandings pair take, and how much of the call is the same for every party?

## What it decides
**The rows are read from the top; the first that matches decides.**

| # | this row shows | then |
|---|---|---|
| 1 | `pick.py` fails, or a name file is absent | Nothing is measured: the kept answer is not the one recorded. Goes to the run coordinator; no party is picked another way. |
| 2 | a call answers, and an EXPECT line of its step is not met (bill count, `state`, `basis`, `checks`, `ledger_match.matched`) | A wrong or unexpected answer is worse than a refusal. Goes to the run coordinator as it is. |
| 3 | a call is refused | Goes to the run coordinator with its code, cause and trail. The sizing is not established for that party. |
| 4 | all three answer as EXPECT, and a call's whole time is 45 s or more | One party does not fit the proposed 45 s step on this book, so Shape A needs a smaller unit than a party here. Goes to the run coordinator with the trails. |
| 5 | all three answer as EXPECT, each whole time under 45 s | **Shape A's per-party step is sized on this book**, partial: for each party its bills, its whole time and its Ledger Outstandings pair. Derived, unverified: parties a 45 s step can take at the largest party's whole time (floor(45 / that time)), and steps for the book's admitted parties at that rate. Recorded on #703. |

45 s is the comment's example budget: the Windows desktop app's 60 s call limit (row H of #2119) less a margin. It decides nothing about other books, other dates, a party with no open bills, a host's own timing, or Gold.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy (sha256 5370c6d2…, as in `lab/2119-p-sheet`). Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step, on the synthetic company, on the owner's standing word of 27 Oct 2026. No person is needed: the calls go through the lab's own stdio client, not the desktop app.

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic, about 30,000 vouchers), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026, as row 2119-P left it on 9 Nov 2026. If it is absent, or step 1's marks differ (record the difference), STOP and report: the picks below were counted at those marks. Only THE COMPANY is loaded, alone, from its load in step 0 to the last step.

**The three parties** are picked before any Tally request, on the lab PC, by `pick.py` in this folder, from row 2377-B's kept Receivable answer (`answers/d2-ref-receivable.answer.xml`, 13,679,490 B, sha256 `5a406cab…`, listed in `lab/2377-b-answers`'s `kept-local.txt`; read at marks 328 and 29900). `pick.py` refuses any other file. It writes `s1.name`, `m1.name` and `l1.name`, each one name's UTF-8 bytes **with no line terminator**, and prints the counts with no name. Each call's `party` is that file's bytes exactly, read whole, nothing trimmed or split; `pick.py` admits only names the tool's name rule admits (`LedgerScopeName`, `src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65-94`). The names stay on the lab PC and are pushed only as `s1`, `m1` and `l1`.
- `l1` is picked by row 2377-B's rule for `top1` (most bills, ties by the name's bytes). Compare `l1.name` with field 1 of `top3.txt`'s first line on the lab PC (field 1 is the text before its one TAB, as ruled on 9 Nov 2026 for row 2119-P); record equal or not. Not equal is recorded, not a STOP.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Tool reads only.** Every read is a call of ComplyEaze Bridge's built tool, one at a time, through one stdio process (A): `tools/list`, `list_companies`, `ledger_masters` with `fields` basic, `outstandings`, `egress_log`. No raw XML request is sent; `GET /status` by curl only where C2 and rule 4 name it.
2. **Closed set.** The three `outstandings` calls are exactly `{"company_guid": "d45bc1b0-e5e3-4261-b3b2-cce3915f42d3", "as_of": "20260331", "party": "<the name>", "detail": "open_bills"}`, in the order `s1`, `m1`, `l1`. Nothing read from Tally or from an answer adds to the set.
3. **Screens.** During each `outstandings` call, a screenshot of Tally's whole window at about 10 s and 30 s after it is sent, and every 30 s after that until it ends, by screen capture only (no click, no focus change, no key in Tally). A dialog there is C7. Each call is a NEW SHAPE under C3 for a party this build has not read, so a screenshot follows it before the next request.
4. **After a refusal.** A refusal of any code is a result, not a STOP: record its code, cause and next step text whole. After a refusal for a deadline (`request_deadline_exceeded`, or any cause that names a time), wait 180 s with a screenshot every 30 s, then ONE `GET /status` (C3's 10-minute rule applies) and record how long it took; the next call is sent only after it answers 200.
5. **Marks.** `ledger_masters` with `fields` basic: `snapshot.master_alter_id`, `snapshot.voucher_alter_id`. Read at step 2 and at step 7, equal.
6. **One Tally.** At step 0 and at the end, C2 a and b: exactly one `tally.exe`, and it owns port 9001.
7. **Keys.** In Tally the Gateway is the only screen used. THE COMPANY is loaded from Select Company with the arrow keys and Enter, Enter only once a screenshot shows the highlighted row reads `BRIDGE CORPUS DENSE`. At the close it is shut by Company, then Shut from the Gateway's menu. **Never Ctrl+Q or Escape at the Gateway.** No other key or click in Tally.

C2 d as applied: step 0 runs with no company loaded, and its MARKS read is met by step 2 (`list_companies` showing exactly one company, and the `ledger_masters` marks); no raw MARKS request is sent, since rule 1 allows none.

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` from the `unsigned-bundle-smoke-windows-latest` artifact of the CI run of the newest master push before the sitting, at or after 10f38816. The run coordinator sends the commit, the run, the artifact id and the exe's sha256 with this sheet; record all four before step 0. Copy it to `C:\bridge-lab\703-lo\bin\bridge_mcp.exe`. Hash it before the first call and after the last: another sha256 = STOP.

Process A: over stdio, as the lab's stdio client runs it in earlier rows, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), redaction unset, writes off, and `BRIDGE_AGENT_DATA_DIR` `C:\bridge-lab\703-lo\data-a`, listed first: absent or empty, else STOP. `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_MAX_ROWS` are unset (the defaults). Before step 2 and after step 8, `tasklist /FI "IMAGENAME eq bridge_mcp.exe" /FO CSV /NH` is recorded. The stdio client records, for each call, the UTC time it was written and the UTC time its answer was read, to the millisecond: that difference is the call's whole time.

## Files
`ROW.md`, `LAB-COMMON.md`, `pick.py`, `.gitattributes`, `SHA256.txt` (each line: sha256, bytes, path). Check every file against `SHA256.txt` first. `pick.py` reads one local file and writes one new folder; it sends nothing.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded (C2 d's MARKS read is step 2): the screenshot, the process list and port 9001, `GET /status`. Hash the exe.
1. **Picks.** `python -I pick.py <row 2377-B's kept answer> C:\bridge-lab\703-lo\picks`: record every line it prints and its exit code. EXPECT exit 0, `bills 29900`, and `parties 120; admitted 120` (row 2377-B's D2 and D3). Exit 1 = row 1. Record the `l1` comparison above. Then load THE COMPANY alone (rule 7), and take a screenshot of the Gateway with Tally's current date (EXPECT 31 March 2026; another date = STOP).
2. Start A. `tools/list`, saved whole: it must list `list_companies`, `ledger_masters`, `outstandings` and `egress_log`, and `outstandings`'s `detail` must offer `open_bills`; one absent = STOP. `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401. `ledger_masters` (basic): M0 and V0, EXPECT 328 and 29900; another value = STOP.
3. **LO-s1.** `outstandings` for `s1` (rule 2). Rule 3's screenshots. Record the whole time.
   - **EXPECT** (inference from row 2119-P, unverified for this party): `isError` false; `result.state` `observed`, `result.basis` `tally_ledger_outstandings`, `result.detail` `open_bills`, `result.as_of` `20260331`; `result.bills` rows plus `result.on_account` rows as many as `pick.py` printed for `s1` (it counts every `BILLPARTY` row of the Receivable report, and an On Account amount is one there, row 2377-B's L7); `result.checks.bills_sum_to_ledger_closing` `tied` and `result.checks.company_marks_unchanged` `checked`; `result.ledger_match.matched` `exact`. Record `result.on_account` rows (a count) and whether the sum of `result.bills[].closing` ties, without the sum.
   - Then `egress_log` with `limit` 50, saved whole. From LO-s1's `response_prepared` record, `request_trail`: `sent`, `failed`, `omitted`, and for each kept entry its `seq`, `kind`, `request_bytes`, `response_bytes`, `held_ms` and `outcome` (a table; counts and sizes only). Mark the snapshot pair and the Ledger Outstandings pair **by position and size**, as row 2119-P did: the snapshot pair is the first pair of equal-sized posts after the extent pair (its requests 1,290 B in 2119-P), the Ledger Outstandings pair the next (820 B there; its answers grow with the party's bills). Say in the notes that the pairs are told by inference. The record keeps the last 32 entries; `omitted` above 0 is recorded, not a finding.
4. **LO-m1.** The same for `m1`, after rule 3's screenshot (and rule 4's wait if LO-s1 was refused for a deadline).
5. **LO-l1.** The same for `l1`. EXPECT also its whole time within a few seconds of row 2119-P's 5.1 s for `top1` (record it; a large difference is recorded, not a STOP).
6. Wait 60 s with a screenshot every 30 s, then ONE `GET /status` (200).
7. `ledger_masters` (basic): M1 and V1, EXPECT equal to M0 and V0; a difference = record and STOP.
8. Stop A (its stop file); record its exit code and error stream. Hash the exe. LAB-COMMON C10: a screenshot at the Gateway with the current date unchanged; Company > Shut; Select Company with none loaded; one `tally.exe`; 9001 LISTENING; `GET /status` 200. No XML request after the shut.
9. Notes, the pre-push scan and the evidence commit (Keep); send its sha to the run coordinator; push on its yes.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog in Tally; a second `tally.exe`; a mark that moves; a tool call without an answer in 10 minutes (stop nothing, send nothing, report); Ctrl+Q or Escape at the Gateway; an exe sha256 not the one named.

## What to report
| step | result |
|---|---|
| build: commit, run, artifact id, exe sha256 before and after | |
| picks: `pick.py`'s lines and exit code; `l1` equal to `top1` or not | |
| step 2: tools, company, M0 and V0 | |
| LO-s1: outcome, whole time, bills, On Account rows, tie, `ledger_match` | |
| LO-s1 trail: the table, with the snapshot pair and the Ledger Outstandings pair marked | |
| LO-m1 and its trail: the same | |
| LO-l1 and its trail: the same | |
| per party: bills, whole time, Ledger Outstandings pair `held_ms`, snapshot pair `held_ms`, the rest of the call | |
| derived (unverified): parties per 45 s step at `l1`'s whole time; steps for 120 parties | |
| M1 and V1; close | |
| any dialog, skip or deviation | |

Read every number from its file. Put any STOP, skip, or EXPECT not met in the first line. Fill the "What it decides" row this result meets.

## Limits
One sitting, one book, one date, three parties chosen by bill count, one build. The whole time is the stdio client's, not a desktop app's. Each call also reads the book's ledger snapshot and marks; whether a stepped read would pay them once a step or once a read is a design question, not measured here. The parties of this book may hold nearly equal bills (row 2377-B's three largest hold 250 each); `pick.py`'s histogram says how wide the spread is.

## Keep
On `lab/703-lo-sizing-answers`, with `SHA256.txt`:
- every tool answer whole, with its size and sha256, except the three `outstandings` answers and the `egress_log` answers, which hold the parties' names and are listed in `kept-local.txt` by size and sha256, as in row 2119-P;
- `pick.py`'s output and exit code (no names), the three trail tables, the three calls' outcome lines (counts, the tie, the whole time) and the marks;
- the build's commit, run, artifact id and sha256;
- a notes file with UTC times and one line per step, and a `RESULTS.md` holding the table above.

The three name files and screenshots stay on the lab PC, listed by name, size and sha256. Notes carry no user names and no machine paths beyond `C:\bridge-lab\703-lo\`. The party names are pushed only as `s1`, `m1` and `l1`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
