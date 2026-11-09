# Row 703-C: two one-day `vouchers` reads in one sitting on the dense synthetic book: with the marks unchanged, does Tally answer every census request byte for byte the same

Not for merge. Part of #703. About 30 minutes. **READS ONLY:** nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). Lines are cited at master 10f38816. Confidence of everything here until the row runs: unverified.

## Question
The 8 Nov 2026 design comment on #703 proposes Shape C: keep the last voucher census in memory, keyed by the company's GUID and voucher mark, and skip it while the marks are unchanged. That is safe only if, with the marks unchanged, Tally answers each census request with the same bytes. A census is the `GUID,ALTERID,DATE` read of the book's vouchers in AlterID spans that every `vouchers` window read sends before its data parts (`census_spans`, `src-tauri/src/agent_voucher_window.rs`). A first page (`offset` 0) always reads afresh, its own census included (`src-tauri/src/agent_vouchers.rs:614-656`), so two identical calls send two censuses.

So: on BRIDGE CORPUS DENSE, two identical one-day `vouchers` calls, 60 s apart, through a relay that keeps every request and answer. Is each census answer of the second call byte-equal to the first's, by position, with both marks unchanged?

## What it decides
**The rows are read from the top; the first that matches decides.**

| # | this row shows | then |
|---|---|---|
| 1 | a mark moved between step 1 and step 5 | VOID: the book changed, so unequal answers would prove nothing. Goes to the run coordinator as it is. |
| 2 | a call is refused, or `census.py` exits 1 (a failed or unanswered request, unequal counts, requests that differ, no single gap) | The comparison was not made. Goes to the run coordinator with the code, cause and `census.py`'s lines. Shape C stays unmeasured. |
| 3 | `census.py` exits 3 (no census post in either call) | This build sends no census for a one-day read on this book, so this book cannot test Shape C. Goes to the run coordinator; a larger book is the next step. |
| 4 | `census.py` exits 4 (a census answer differs, the requests equal and the marks unchanged) | **Shape C as proposed is refused on this evidence:** a cache keyed by GUID and mark would serve an answer Tally no longer gives. Both answers are kept whole (Keep). Goes to the run coordinator. |
| 5 | `census.py` exits 0, and the two calls' `result` rows are equal (step 4) | **The premise of Shape C holds on this book for this pair**, partial: one sitting, one book, one day, two calls. Whether deleting a voucher moves the mark (the comment's other open point) is not answered here. |
| 6 | `census.py` exits 0, but the two calls' `result` rows differ | The census agreed but the data parts did not. Goes to the run coordinator with both answers; #703 records it. |

It decides nothing about other books, a book with a mark near a million (the 6 Oct 2026 comment's book sent 126 census reads; this one sends a few), a census after a deletion, other releases, or Gold.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy (sha256 5370c6d2…, as in `lab/2119-p-sheet`). Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step, on the synthetic company, on the owner's standing word of 27 Oct 2026. No person is needed: the calls go through the lab's own stdio client, not the desktop app.

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic, about 30,000 vouchers), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026, as row 2119-P left it on 9 Nov 2026 (`lab/2119-p-answers`). If it is absent, or step 1's marks differ (record the difference), STOP and report. Only THE COMPANY is loaded, alone, from its load in step 0 to the last step.

**The day** is 15 May 2025, fixed here. The protocol reference records 82 vouchers on it on this book (`docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md`, the ledger-scoped census section: "The census of a day"). Nothing read from Tally changes the day.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Tool reads only.** Every Tally request comes from ComplyEaze Bridge's built tool, one call at a time, through one stdio process (A): `tools/list`, `list_companies`, `ledger_masters` with `fields` basic, `vouchers`, `egress_log`. No raw XML request is sent; `GET /status` by curl only where C2 and rule 5 name it, and those go to port 9001 directly, not through the relay.
2. **Closed set.** The two `vouchers` calls are exactly `{"company_guid": "d45bc1b0-e5e3-4261-b3b2-cce3915f42d3", "from": "2025-05-15", "to": "2025-05-15"}`. Nothing read from Tally or from an answer adds to the set.
3. **The relay.** `relay.py` in this folder is a byte copy of `lab/2074-2214-export-cut` at 7a49b5c1 (sha256 271af169…, as that sheet's `SHA256.txt` lists it). Run it as `python -I relay.py 9002 9001 C:\bridge-lab\703-c\relay C:\bridge-lab\703-c\arm`, with `arm` an existing EMPTY folder: unarmed, it forwards each request byte for byte and sends no request of its own (its docstring). Nothing is ever written to `arm`. It refuses a non-empty output folder; it must be new. Start it before A, stop it after A (Ctrl+C in its own window), and record that `relay.jsonl` holds no `front_send`, `bridge_cut`, `tally_closed` or `relay_error` record.
4. **Screens.** During each `vouchers` call, a screenshot of Tally's whole window at about 10 s and 30 s after it is sent, and every 30 s after that until it ends, by screen capture only (no click, no focus change, no key in Tally). A dialog there is C7. Each `vouchers` call on this book through this build is a NEW SHAPE under C3, so a screenshot follows it before the next request.
5. **After a refusal.** A refusal of any code is a result, not a STOP: record its code, cause and next step text whole. After a refusal for a deadline (`request_deadline_exceeded`, or any cause that names a time), wait 180 s with a screenshot every 30 s, then ONE `GET /status` to 9001 (C3's 10-minute rule applies) and record how long it took; send nothing more, and go to step 5.
6. **Marks.** `ledger_masters` with `fields` basic: `snapshot.master_alter_id`, `snapshot.voucher_alter_id`. Read at step 1 and at step 5.
7. **One Tally.** At step 0 and at the end, C2 a and b: exactly one `tally.exe`, and it owns port 9001. The relay owns 9002 and nothing else.
8. **Keys.** In Tally the Gateway is the only screen used. THE COMPANY is loaded from Select Company with the arrow keys and Enter, Enter only once a screenshot shows the highlighted row reads `BRIDGE CORPUS DENSE`. At the close it is shut by Company, then Shut from the Gateway's menu. **Never Ctrl+Q or Escape at the Gateway.** No other key or click in Tally.

C2 d as applied: step 0 runs with no company loaded, and its MARKS read is met by step 1 (`list_companies` showing exactly one company, and the `ledger_masters` marks); no raw MARKS request is sent, since rule 1 allows none.

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` from the `unsigned-bundle-smoke-windows-latest` artifact of the CI run of the newest master push before the sitting, at or after 10f38816. The run coordinator sends the commit, the run, the artifact id and the exe's sha256 with this sheet; record all four before step 0. Copy it to `C:\bridge-lab\703-c\bin\bridge_mcp.exe`. Hash it before the first call and after the last: another sha256 = STOP.

Process A: over stdio, as the lab's stdio client runs it in earlier rows, with `BRIDGE_TALLY_HOST=127.0.0.1`, **`BRIDGE_TALLY_PORT=9002`** (the relay), `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), redaction unset, writes off, and `BRIDGE_AGENT_DATA_DIR` `C:\bridge-lab\703-c\data-a`, listed first: absent or empty, else STOP. `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_MAX_ROWS` are unset (the defaults). Before step 1 and after step 6, `tasklist /FI "IMAGENAME eq bridge_mcp.exe" /FO CSV /NH` is recorded. The stdio client records, for each call, the UTC time it was written and the UTC time its answer was read, to the millisecond.

## Files
`ROW.md`, `LAB-COMMON.md`, `relay.py`, `census.py`, `.gitattributes`, `SHA256.txt` (each line: sha256, bytes, path). Check every file against `SHA256.txt` first. `census.py` reads the relay's folder only; it sends nothing.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded (C2 d's MARKS read is step 1): the screenshot, the process list and port 9001, `GET /status`. Hash the exe. Load THE COMPANY alone (rule 8). Screenshot of the Gateway with Tally's current date (EXPECT 31 March 2026; another date = STOP). Create `C:\bridge-lab\703-c\arm` empty; start the relay (rule 3); record its `listening` line.
1. Start A. `tools/list`, saved whole: it must list `list_companies`, `ledger_masters`, `vouchers` and `egress_log`; one absent = STOP. `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401. `ledger_masters` (basic): M0 and V0, EXPECT 328 and 29900; another value = STOP.
2. **V-1.** `vouchers` (rule 2). Rule 4's screenshots. Record the whole time, the answer whole, and from it: `isError`, `result.total` (or the field the answer gives for the row count; EXPECT 82, recorded either way), the `read_cost` block if there is one (`census_reads`, `observed_seconds`), and the `snapshot` block. Then `egress_log` with `limit` 50, saved whole.
3. Wait 60 s with a screenshot every 30 s (no call in between). This gap is how `census.py` tells the two calls apart.
4. **V-2.** The same call as V-1, and the same records. Then `egress_log` with `limit` 50. Compare V-1's and V-2's rows: the list of `result` rows of each answer, each row as JSON, sorted; equal or not, and the first difference if not. Fields of the answer outside its rows (times, ids, `read_cost`) are not compared.
5. `ledger_masters` (basic): M1 and V1, EXPECT equal to M0 and V0 (else row 1).
6. Stop A (its stop file, never by killing it); record its exit code and error stream. Stop the relay (rule 3). Hash the exe. LAB-COMMON C10: a screenshot at the Gateway with the current date unchanged; Company > Shut; Select Company with none loaded; one `tally.exe`; 9001 LISTENING; nothing on 9002; `GET /status` 200. No XML request after the shut.
7. `python -I census.py C:\bridge-lab\703-c\relay`: record every line it prints and its exit code. It is run after the relay has stopped, on the folder as the relay left it. The number of census posts a call sends is not pinned here: it is the census spans for a voucher mark of 29900, possibly each sent twice by the paired reader (`src-tauri/src/agent.rs:2918`). Record it; only equal counts across the two calls matter.
8. Notes, the pre-push scan and the evidence commit (Keep); send its sha to the run coordinator; push on its yes.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog in Tally; a second `tally.exe`; a tool call without an answer in 10 minutes (stop nothing, send nothing, report); Ctrl+Q or Escape at the Gateway; an exe sha256 not the one named; anything written to `arm`.

## What to report
| step | result |
|---|---|
| build: commit, run, artifact id, exe sha256 before and after | |
| step 1: tools, company, M0 and V0 | |
| V-1: outcome, whole time, row count, `read_cost` if any | |
| V-2: the same | |
| V-1 and V-2 rows: equal or not | |
| `census.py`: its lines and exit code | |
| relay: no `front_send`, `bridge_cut`, `tally_closed` or `relay_error` | |
| M1 and V1; close | |
| any dialog, skip or deviation | |

Read every number from its file. Put any STOP, skip, or EXPECT not met in the first line. Fill the "What it decides" row this result meets.

## Limits
One sitting, one book, one day, two calls, one build. The census of a book this size is a few requests; the case Shape C is for (a census of over a hundred requests) is not this book. A deleted or altered voucher, and whether either moves the mark, are not tested: they are writes.

## Keep
On `lab/703-c-census-answers`, with `SHA256.txt`:
- every tool answer whole, with its size and sha256, except the two `vouchers` answers and the `egress_log` answers, which hold party names and are listed in `kept-local.txt` by size and sha256, as in row 2119-P;
- the relay's `relay.jsonl` whole, and from `bodies/` the census requests and answers only (a census answer holds `GUID`, `ALTERID` and `DATE`, no name); every other body is listed in `kept-local.txt` by size and sha256;
- `census.py`'s output and exit code, and the rows comparison;
- the build's commit, run, artifact id and sha256;
- a notes file with UTC times and one line per step, and a `RESULTS.md` holding the table above.

Screenshots stay on the lab PC, listed by name, size and sha256. Notes carry no user names and no machine paths beyond `C:\bridge-lab\703-c\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
