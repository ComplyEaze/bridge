# Row 703-G: does opening GST Annual Computation on Tally's screen change what Bridge reads, with neither mark moving

Not for merge. Part of #703. About 60 minutes. **One screen action writes to THE COMPANY:** the Yes on GST Annual Computation's prompt, which says it will update the GST status of vouchers. That change is undone at the end by restoring BK-703-G (C5, C6). Nothing is created, altered or imported by request, and no ComplyEaze Bridge write tool is called. Lines are cited at master ae38a75c. Confidence of everything here until the row runs: unverified.

## Question
Shape C (the 8 Nov 2026 design comment on #703) would skip a census while the company's marks are unchanged. The import-verification replay skip (#1241) rests on the same rule: every change that could alter what a window read returns moves a mark. The protocol reference records one screen action that broke that rule as far as the marks go. `docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md`, the paragraph in section 11c.5 that opens "A screen action whose prompt said it would update stored data, and that moved neither mark" (PARTIAL, one run, TallyPrime 7.2, 25 Oct 2026, on another lab book). There, Go To > GST Annual Computation asked to update the GST status of some vouchers. After Yes, both marks were unchanged, and only the `CMPINFO` counter `VOUCHER` in the marks answer moved. No voucher was read before and after, so whether any voucher changed rests on the prompt alone. This row asks the same question on a different synthetic book.

`CMPINFO` counters move without the book changing (the same file's open question "What do the `CMPINFO` counters report": "no two responses can be compared by hash or byte equality"). So `compare.py` compares each answer with its `CMPINFO` elements removed, and prints a `CMPINFO`-only difference with the counters that moved, without counting it.

So: read the book through Bridge's tools before and after that Yes, through a relay that keeps every request and answer, and compare them byte for byte outside `CMPINFO` (below). The reads are the whole year's `vouchers` (the data parts, and a census if this build sends one for this book), `gst_status` (the read whose fields are the GST status flags), and the marks. Does any answer change while neither mark moves?

## What it decides
**The rows are read from the top; the first that matches decides.**

| # | this row shows | then |
|---|---|---|
| 1 | the company `list_companies` shows is not THE COMPANY by its GUID | STOP before step 3. Record and ask the run coordinator. |
| 2 | a call is refused, or `compare.py` exits 1 (a failed or unanswered request, fewer than one `vouchers` or eight `status` answers in a folder, unequal counts, requests that differ) | The comparison was not made. Goes to the run coordinator with the code, cause and `compare.py`'s lines. |
| 3 | the prompt does not appear at step 6 (the report opens with none) | **Not reproduced:** no Yes was given. Step 7 still runs, and its comparison is recorded as a control. Whether a read, the backup or something else had already made the update is not known. A control that differs outside `CMPINFO`, or a mark that moved with no Yes, is itself a result for the run coordinator. |
| 4 | a mark value `ledger_masters` reports moved between MB (step 5) and M2 (step 7) | The Yes moved a mark, unlike 25 Oct (the backup did not: step 5 held MB equal to M1). Record which mark and by how much. A cache keyed by the marks would be dropped, so the rule holds for this action on this run. Recorded on #703 with the comparison. |
| 5 | the marks equal, and `compare.py` exits 4 with a `vouchers` or `census` answer among those differing outside `CMPINFO` | **A change Bridge's voucher reads see, with neither mark moving:** Shape C as proposed and the replay skip are unsafe for this action. Both answers are kept whole (Keep). Goes to the run coordinator at once; #703 and the protocol reference record it. |
| 6 | the marks equal, and `compare.py` exits 4 with `status` answers among those differing outside `CMPINFO`, and no `vouchers` or `census` answer | **The GST status flags changed with neither mark moving.** A `gst_status` answer cannot be reused or vouched for by the marks alone. The `vouchers` data and any census did not change. Any `marks` or `other` answer that also differs is listed. Both answers are kept whole (Keep). Goes to the run coordinator; #703 and the protocol reference record it. |
| 7 | the marks equal, and `compare.py` exits 4 with only `marks`, `other` or `get` answers differing outside `CMPINFO` | Something outside the voucher and status reads changed (a ledger catalogue, the company list, or a High Water answer whose bytes differ while the mark values `ledger_masters` reports are equal). Both answers are kept whole (Keep), and RESULTS.md names the differing field without its value. Goes to the run coordinator; no conclusion about Shape C is drawn. |
| 8 | the marks equal, and `compare.py` exits 0 | **The Yes (with the backup before it) changed nothing Bridge reads here outside `CMPINFO`**, partial: one sitting, one book, one build. `compare.py` found at least one `vouchers` and eight `status` answers in each folder, all equal outside `CMPINFO`. So Shape C and the replay skip hold for this action as far as Bridge's reads go. The `CMPINFO` counters that moved are recorded. |

It decides nothing about TallyPrime 7.1, other books, other screen actions, a GST status update triggered any other way, or Gold.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy (sha256 5370c6d2…, as in `lab/703-c-census-sheet`). Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step: the tool calls through the lab's own stdio client, and the screen action on the synthetic company. The backup (C5) and the restore (C6) are done as LAB-COMMON directs. Step 9's restore is part of this sheet and needs no further word; a restore after a STOP needs the run coordinator's word (C8). The screen action is a write by the lab on a synthetic book, as a lab row made one on another lab book on 25 Oct 2026. Before the sitting, the run coordinator confirms that the owner's standing words cover it and gives the go-ahead. No person is needed at the keyboard: no desktop app and no approval window are used.

**THE COMPANY** is `BRIDGE PILOT LAB` (synthetic; company number 100033), GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`. That GUID is in the merge gate's synthetic registry (`scripts/merge-gate-synthetic-uuids.json`) and is master's test constant `PILOT_LAB_GUID` (`src-tauri/crates/bridge-tally-protocol/tests/captured_parse_record.rs:303`).
- It is GST-registered, with Sales invoices of the 2026-27 year; its books begin 1 April 2026 (`books_from` 20260401 in master's pilot-lab captures), and step 2 records `books_from` again.
- On 26 Oct 2026, row 2011 read its marks as 68 and 245, and GSTR-1 and GSTR-3B opened on it with no question to update GST status (`lab/2011-gst-report-names-answers`, RESULTS.md). GST Annual Computation was not opened there, so whether it asks on this book is not known: that is row 3.
- Its marks are recorded at step 3, not pinned: later rows may have posted to it.
- Only THE COMPANY is loaded, alone, from its load in step 0 to step 9's shut, except during the restore (C6).

**Differences from LAB-COMMON** (the stricter rule holds everywhere else):
1. **Tool reads only.** Every Tally request comes from ComplyEaze Bridge's built tool, one call at a time. The tools used are `tools/list`, `list_companies`, `ledger_masters` with `fields` basic, `vouchers`, `gst_status` and `egress_log`. No raw XML request is sent. `GET /status` is sent by curl only where C2 and rule 6 name it, to port 9001 directly.
2. **Closed set.** A PHASE is these calls, in this order:
   1. `list_companies`.
   2. `ledger_masters` basic: the marks.
   3. `vouchers` `{"company_guid": "6b43e498-430c-4d5c-bfef-d32e2ab93c85", "from": "2026-04-01", "to": "2027-03-31"}`.
   4. `gst_status` with that `company_guid`, for `voucher_type_name` `Sales` and then `Purchase`, each over four windows: `2026-04-01` to `2026-06-30`, `2026-07-01` to `2026-09-30`, `2026-10-01` to `2026-12-31`, `2027-01-01` to `2027-03-31`. That is eight calls, Sales first, each window in that order.
   5. `ledger_masters` basic: the marks again.

   Each `gst_status` call takes `limit` 500. If any call answers with `next_offset`, record it and do not page. Nothing read from Tally or from an answer adds to the set. A type name that matches nothing answers `total` 0: that is a result, recorded as it is.
3. **The relay.** `relay.py` in this folder is a byte copy of `lab/2074-2214-export-cut` at 7a49b5c1 (sha256 271af169…).
   - It runs once per phase, with a new output folder each time: `python -I relay.py 9002 9001 C:\bridge-lab\703-g\before C:\bridge-lab\703-g\arm` for step 3, `...\after` for step 7 and `...\restored` for step 9. `arm` is an existing EMPTY folder.
   - Unarmed, it forwards each request byte for byte and sends no request of its own. Nothing is ever written to `arm`.
   - Start it before the phase's process and stop it after that process has ended. Start and stop it with `relayrun.py`, the wrapper row 703-C used, which stays on the lab PC: 2014 B, sha256 `4b2718877a98dcb5f366a017e710fb28d1b28eac95de09048209ea03a1674a7c`, as `lab/703-c-answers/kept-local.txt` lists it. Check its hash first; another hash = STOP. It sends Ctrl+C to the relay's own console, then opens and closes one connection to 9002 with no byte sent (`lab/703-c-answers` RESULTS.md, deviation 1). EXPECT the relay's exit code 3221225786 (0xC000013A, the Ctrl+C exit). A plain Ctrl+C does not stop the relay, because it waits in `accept()`. Never kill it.
   - Record that neither `relay.jsonl` holds a `front_send`, `bridge_cut`, `tally_closed` or `relay_error` record, except a `relay_error` after the last answer, which `compare.py` prints as teardown.
4. **Processes.** One stdio process per phase, P1 for step 3, P2 for step 7 and P3 for step 9, each with `BRIDGE_TALLY_PORT=9002` (the relay) and its own `BRIDGE_AGENT_DATA_DIR`: `C:\bridge-lab\703-g\data-p1`, `...\data-p2` and `...\data-p3`. One more process, PB, is started after the backup with `BRIDGE_TALLY_PORT=9001` (no relay) and `...\data-pb`, for step 5's `list_companies` and one marks read only. Each data folder is listed first: absent or empty, else STOP. All four take `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), with redaction, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_MAX_ROWS` unset and writes off. Each is ended by its stop file, never by killing it; record its exit code and error stream. The stdio client records, for each call, the UTC time it was written and the time its answer was read, to the millisecond.
5. **Screens.** During each `vouchers` call, a screenshot of Tally's whole window at about 10 s after it is sent, by screen capture only (no click, no focus change, no key in Tally). A dialog there is C7. `vouchers` and `gst_status` on this book through this build are NEW SHAPES under C3, so a screenshot follows each before the next request.
6. **After a refusal.** A refusal of any code is a result, not a STOP: record its code, cause and next step text whole, and carry on with the phase. After a refusal for a deadline (`request_deadline_exceeded`, or any cause that names a time), wait 180 s with a screenshot every 30 s, then ONE `GET /status` to 9001 (C3's 10-minute rule applies) and record how long it took. A refused call is still a position in `compare.py`'s comparison. A call refused in one phase and answered in the other is row 2.
7. **One Tally.** At step 0 and at the end, C2 a and b apply: exactly one `tally.exe`, and it owns port 9001. The relay owns 9002 while it runs, and nothing else does.
8. **Keys in Tally.**
   - THE COMPANY is loaded from Select Company with the arrow keys and Enter, and Enter only once a screenshot shows the highlighted row reads `BRIDGE PILOT LAB`.
   - Step 6 uses exactly: Alt+G (Go To) and the text `GST Annual Computation`; then a screenshot of the Go To list, and Enter only if its highlighted row reads `GST Annual Computation` (else press no key, screenshot, STOP and report); then the Y key for Yes on the one prompt named below; then Escape ONCE on the report, after a screenshot showing the report's title, to come back to the Gateway.
   - The backup and restore use C5's and C6's own keys. At the close, the company is shut by Company, then Shut from the Gateway's menu.
   - **Never Ctrl+Q or Escape at the Gateway, and never Escape on a master or voucher screen.** No other key or click in Tally.
9. **The named prompt.** Possible at step 6, as a lab row recorded it on another book on 25 Oct 2026: "GST-related details were altered in the masters/Return. Hence, opening the report will update the GST Status of <n> vouchers. Do you want to open the report?" [Yes] [No], with a count in place of <n>.
   - It is answered Y only after a screenshot shows it, and only if its text is that sentence, word for word, with any count in place of <n>. Record the count.
   - Any other text, a second prompt, or any other dialog: C7 (screenshot, no key, report).

C2 d as applied: step 0 runs with no company loaded, and its MARKS read is met by step 3's `ledger_masters`. C5's "one MARKS read" after the backup is step 5's PB read (MB). C6's "MARKS first" after the restore is met by the first `ledger_masters` of Phase RESTORED, which follows only `tools/list` and the `list_companies` the tool needs for its GUID. Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` from the `unsigned-bundle-smoke-windows-latest` artifact of the CI run of the newest master push before the sitting, at or after ae38a75c. The run coordinator sends the commit, the run, the artifact id and the exe's sha256 with this sheet; record all four before step 0. Copy the exe to `C:\bridge-lab\703-g\bin\bridge_mcp.exe`. Hash it before the first call and after the last: another sha256 = STOP.

## Files
`ROW.md`, `LAB-COMMON.md`, `relay.py`, `compare.py`, `.gitattributes`, `SHA256.txt` (each line: sha256, bytes, path). Check every file against `SHA256.txt` first. `compare.py` reads the two relay folders only; it sends nothing. `relayrun.py` is not in this folder: it stays on the lab PC and is checked by the hash in rule 3.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded: the screenshot, the process list and port 9001, `GET /status`. Hash the exe. Load THE COMPANY alone (rule 8). Take a screenshot of the Gateway with Tally's current date, and record it. Create `C:\bridge-lab\703-g\arm` empty.
1. Start the relay into `before` (rule 3) and record its `listening` line. Start P1. `tools/list`, saved whole: it must list `list_companies`, `ledger_masters`, `vouchers`, `gst_status` and `egress_log`; one absent = STOP.
2. (P1) `list_companies`: EXPECT exactly THE COMPANY by its GUID, `identity_state` `verified_tuple`; record `books_from`. Another GUID = STOP (row 1).
3. **Phase BEFORE** (P1, rule 2). For each call record the whole time, the answer whole, and `isError`. From `ledger_masters`, record the marks: M0 is the first read and M1 the last. From `vouchers`, record `result.total` and the `snapshot` block. From each `gst_status`, record `result.total`, `result.counts` and `result.state`. Then `egress_log` with `limit` 50, saved whole. Stop P1, then stop the relay (rule 3).
4. M1: EXPECT equal to M0 (the phase did not change the book). Else STOP: the book moved under reads alone.
5. **BK-703-G** by C5, to the new empty local folder `C:\bridge-lab\703-g\bk`, with THE COMPANY loaded. Record the file's name, bytes and sha256, and the backup's time to the minute. Then start PB (rule 4), `list_companies` (EXPECT THE COMPANY by its GUID, else STOP), then `ledger_masters` basic: MB. Stop PB. MB: EXPECT equal to M1, else STOP: the backup moved a mark.
6. **The screen action** (rules 8 and 9), only once step 5 has recorded the backup's sha256 and MB equal to M1.
   1. Alt+G, `GST Annual Computation`; a screenshot of the Go To list; Enter only if its highlighted row reads `GST Annual Computation`, else press no key, screenshot, STOP and report (rule 8).
   2. Screenshot the prompt, then press Y.
   3. Screenshot the report, which opens for the financial year. Record its period and its "Total Vouchers", "Not Relevant for This Return" and "Uncertain Transactions" counts as the screen shows them.
   4. Escape once, then screenshot the Gateway.
   5. If no prompt appeared and the report opened, record that (row 3): press Escape once, screenshot the Gateway, and go on.
7. **Phase AFTER.** Start the relay into `after`, start P2, then `tools/list` (saved whole) and `list_companies`. Then the same phase as step 3, with the same records. M2 is its first marks read and M3 its last. Then `egress_log` with `limit` 50. Stop P2, then stop the relay.
8. `python -I compare.py C:\bridge-lab\703-g\before C:\bridge-lab\703-g\after`: record every line it prints and its exit code. Every Tally request P1 sends lands in `before` and every one P2 sends lands in `after`, steps 1 and 2 and 7's opening calls included. The comparison by position relies on two fresh processes making the same calls sending the same requests in the same order. If they do not, `compare.py` exits 1 (row 2). Also record, for each `gst_status` call, whether `result.counts` and the `items` (each as JSON, sorted) are equal across the phases, with the first difference if they are not. Do the same for `vouchers`' `result` rows. Fields outside the rows (times, ids, `read_cost`) are not compared.
9. **Restore BK-703-G** by C6 (the plan, the listings before and after, Replace). Load THE COMPANY alone and take C6's two screenshots. Then **Phase RESTORED**: start the relay into `restored`, start P3, then `tools/list` (saved whole) and `list_companies`. Then the same phase as step 3, with the same records. M4 is its first marks read and M5 its last: EXPECT both equal to M0. Then `egress_log` with `limit` 50. Stop P3, then stop the relay. Then `python -I compare.py C:\bridge-lab\703-g\before C:\bridge-lab\703-g\restored`: record its lines and exit code, EXPECT 0. Marks not equal to M0, or exit 4, = STOP and report: the book is not back as Bridge read it before (C8). Exit 1 = STOP and report: the restore is not proved. Hash the exe. LAB-COMMON C10: a screenshot at the Gateway with the current date unchanged; Company > Shut; Select Company with none loaded; one `tally.exe`; 9001 LISTENING; nothing on 9002; `GET /status` 200.
10. Notes, the pre-push scan and the evidence commit (Keep). Send its sha to the run coordinator; push on its yes.

STOP RULES:
- C7 and C8 of LAB-COMMON; any dialog in Tally but the one named prompt and those C5 and C6 quote.
- A second `tally.exe`.
- A tool call without an answer in 10 minutes (stop nothing, send nothing, report).
- Ctrl+Q or Escape at the Gateway.
- An exe sha256 not the one named.
- Anything written to `arm`.
- A restore that does not bring the marks back to M0, or after which `compare.py` (before, restored) does not exit 0.

## What to report
| step | result |
|---|---|
| build: commit, run, artifact id, exe sha256 before and after | |
| steps 1 to 2: tools, company | |
| phase BEFORE: M0, M1; `vouchers` total and time; each `gst_status` total, counts, state | |
| BK-703-G: file, bytes, sha256, time; MB | |
| step 6: the prompt's text and count, Y or not; the report's period and three counts | |
| phase AFTER: M2, M3; the same records as BEFORE | |
| `compare.py`: its lines and exit code; the `gst_status` items and `vouchers` rows equal or not | |
| relay: no `front_send`, `bridge_cut`, `tally_closed` or `relay_error` before the last answer, in any of the three folders | |
| restore: plan, listings; M4, M5; `compare.py` (before, restored) lines and exit code; close | |
| any dialog, skip or deviation | |

Read every number from its file. Put any STOP, skip, or EXPECT not met in the first line. Fill the "What it decides" row this result meets.

## Limits
One sitting, one book, one build, one release (whatever the lab runs; record it). The two type names are a closed guess at where the prompt's vouchers are: a status change on a voucher of another type is seen only through `vouchers`, if its fields show it. A difference in an answer says that something Bridge reads changed, not what Tally changed or why.

## Keep
On `lab/703-g-gst-computation-answers`, with `SHA256.txt`:
- `relay.jsonl` of all three folders whole, and both `compare.py` runs' output and exit code.
- From `bodies/`, the marks and census bodies. Any pair of answers `compare.py` marks as differing, kept whole together with its request. Every other body is listed in `kept-local.txt` by size and sha256.
- Every tool answer whole, with its size and sha256. An answer the pre-push scan flags (party names, the book's GSTIN) is kept on the lab PC instead and listed in `kept-local.txt`, as in rows 703-C and 703-LO. A flagged body of a differing pair is listed the same way, with the field that differs named in RESULTS.md without its value.
- The build's commit, run, artifact id and sha256; `restore-plan.txt`; the backup's name, bytes and sha256.
- A notes file with UTC times and one line per step, and a `RESULTS.md` holding the table above.

Screenshots stay on the lab PC, listed by name, size and sha256. Notes carry no user names and no machine paths beyond `C:\bridge-lab\703-g\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
