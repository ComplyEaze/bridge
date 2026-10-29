# 2142 `GET /status` while Tally builds a long read (#2142): timed `/status` at rest, beside a Bills Receivable read answered whole, and after the same read cut at 20 s, read only, on the dense synthetic book

Not for merge. About 60 minutes, most of it the waits between blocks and the steps by hand. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
#2142 proposes that, after a request is cut for time, ComplyEaze Bridge sends nothing to that Tally but a short `GET /status` until it answers promptly again. That rests on one question this row answers: **does `GET /status` wait while Tally is building a long read, or does it answer at once?**
- If it waits, its answer time is a signal that Tally is busy.
- If it answers at once while Tally is still building, it is not, and #2142 needs another probe. That probe is out of this row's scope: it would be a new request shape with its own review.

On record so far, and not enough to decide:
- after a tool call cut at 20 s, one `GET /status` sent about 16 s after the refusal answered 200; its own time was not recorded (`lab/0.7.0-master-live-checks-answers`);
- on an earlier sitting, `/status` answered quickly well after a cut (#2142's text), which does not show whether the abandoned read had ended.

The code has a standing rule not to gate a post on `/status` (`src-tauri/src/tally/connection.rs`, `probe_with_wire_evidence`). This row is the evidence any change to that rule would need.

The row measures:
1. **At rest (A1, A2, A3):** ten `GET /status`, each started 2 s after the last ended. This is the time `/status` takes on this Tally with nothing else running.
2. **Beside a read answered whole (B1, B3):** the Bills Receivable read that took 41.5 s and 36.7 s here before (`lab/2119-bills-read-timing-answers`, R4 and R5), sent by curl with no client limit. One `GET /status` at a time beside it, from 2 s after it starts until it has ended and three `/status` in a row sent after its end have answered in under 1 s.
   - Here the read's own end time is known, so each probe can be placed before or after it.
   - **This is the decisive measurement:** does a probe sent while the read is still being built answer before the read ends?
3. **Beside the same read cut at 20 s (B2, B4):** the same read with curl's `--max-time 20`, as ComplyEaze Bridge cuts a request, then the same probes until three in a row answer in under 1 s. Then, at once, one `ledger_masters` (basic) through the built tool, timed.
   - This is #2142's own row: how long `/status` stays slow after a cut, and whether a read sent once it is quick again answers promptly. On this book, a tool read sent straight after a cut has been refused for time before (`lab/0.7.0-master-live-checks-answers`): that would be a result here, not a STOP.

Each of B1 and B2 is repeated once (B3, B4), per P6.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/2119-bills-60s-row` at c43ef612, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds, except for the named differences below.

**Who acts.** The lab does every step here on the synthetic company, on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs (its rule on names applies to what is pushed).

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, as the #2119 rows left it: marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026. If it is absent, or step 1's marks differ (record the difference), STOP and report; the run coordinator decides. Only THE COMPANY is loaded, alone, from its load in step 0 to the last step.

Named differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Two requests at once, only inside a B block.** C3 sends one request at a time. Inside a B block, `probe.py` sends the one read and, beside it, `GET /status` probes one at a time: never more than one probe in flight, and never more than two requests in all.
   - **Outside a B block,** one request at a time, as C3 says.
   - **No call of ComplyEaze Bridge is made while `probe.py` runs.**
   - Sending `/status` beside a running report read is new for this lab, apart from the single `/status` after a cut on record above. **Before the sitting, the run coordinator checks the private record of gateway hazards for any rule against it.** A rule against it means the row is not run. The notes record that the check was made and its result, without its content.
2. **Probe timing.** Each probe is `curl.exe -sS -w "%{http_code} %{size_download} %{time_total}" -o <file> http://127.0.0.1:9001/status`, with no `--max-time`, as C3 says. This sheet's line is the authority: LAB-COMMON C2 c gives no command text. A probe counts as answered only with exit 0, status 200 and 51 bytes (the body earlier rows recorded). Anything else is `probe.py`'s STOP. A probe without an answer in 600 s is C3's "without an answer": STOP. `probe.py` leaves that curl running and sends nothing more.
3. **The read.** Each read is C3's line with `%{time_total}` added to `-w`, sending `requests/r-4-receivable-20260331.utf16le.xml`: 770 bytes, sha256 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f, a byte copy of the request R4 sent (`lab/2119-bills-read-timing` at 6af5124c).
   - **In B1 and B3:** no `--max-time`. EXPECT status 200 and 13,679,490 bytes with sha256 5a406cab1a2fe7f6581591a0e910215c48cf380ce04f7558b0896f109a560f5a (R4's and R5's answer). Size bound: 20,000,000 bytes; above it = STOP. A size up to the bound other than 13,679,490, or another sha256, is recorded, not a STOP; the run coordinator weighs it. After B1 and B3 the operator compares the headers file's CONTENT-LENGTH with the size; a mismatch is C3's STOP.
   - **In B2 and B4:** `--max-time 20`, one named exception to C3's "no --max-time". EXPECT curl exit 28 (a client timeout), with no whole answer. That ending is this row's subject, not C3's "ended without one = STOP".
   - **Sends:** the cut read is sent twice in the row (B2, B4) and the whole read twice (B1, B3), each a named exception to C3's "never sent again", because repeating it is what the row measures.
   - **Checks by `probe.py`:** it refuses to send unless the request file's sha256 is the one above (`--read-sha256`). A whole read (B1, B3) that does not end with exit 0, status 200 and some bytes is a STOP. A cut read that answers whole within 20 s is logged as `read_answered_before_cut`, a result to record, not a STOP. A cut read that ends any other way than exit 28 or exit 0 (a refused or reset connection, say) is a STOP: such a read may never have reached Tally, and its block would measure nothing.
4. **Running a block.** ComplyEaze Bridge is started at step 1 and stays idle during every block: no call is made to it then, and it sends nothing to Tally of itself. `probe.py` is run from this sheet's folder, in the background, by exactly this, in PowerShell:
   `$p = Start-Process -PassThru -NoNewWindow -FilePath python -ArgumentList @('-I','probe.py', <the block's arguments>) -RedirectStandardOutput C:\bridge-lab\2142\answers\<block>-stdout.txt -RedirectStandardError C:\bridge-lab\2142\answers\<block>-stderr.txt; $null = $p.Handle`
   and, once it has ended, `$p.WaitForExit(); $p.ExitCode`. While it runs, the only other act is screen capture. Exit 0 is the block as designed. Exit 2 is a STOP (below). Any other exit code (a Python error exits 1) is a STOP too. Both files are kept.
   - **A dialog during a block:** create the empty file `C:\bridge-lab\2142\answers\STOP-<block>`. `probe.py` checks for it before each probe and then stops sending. That is the only way to stop it; never end a process. Then C7.
   - **Screenshots:** a screenshot of Tally's whole window just before each block starts, every 30 s while it runs, and straight after it ends (before anything else, the timed `ledger_masters` included). Screen capture only: no click, no focus change, no key. A dialog in any of them is C7.
   - **The answers folder** is `C:\bridge-lab\2142\answers`, created empty before step 1 and listed (absent or empty, else STOP). `--out` names it in every block.
   - **On exit 2** (a STOP): send nothing more, and do not end any curl. `probe.py` has already waited up to 600 s for a read curl still running to end on its own, and logged its result (`stop_wait_ended`). If that wait ran out, record the read's answer file size and sha256 and its headers file as they are. Then report.
5. **The timed `ledger_masters` after B2 and B4** is sent once, within 30 s of the UTC of the block's `end` log line (only the closing screenshot comes between), and never repeated. Record the UTC of the `end` line and of the call's send.
   - **Answered:** record its seconds, its marks and the answer whole.
   - **Refused:** whatever the code or cause (a refusal for time, as above, among them), that is a result. Record the code, cause and next step text whole, and its seconds. Then the marks are read once, after the wait in rule 6, by another `ledger_masters` (basic), sent only if `GET /status` answered 200 and no dialog is up. A `GET /status` that does not answer 200 there is C3's STOP, and the marks are then read as C8 allows, or not at all.
6. **Waits between blocks.** After every B block (after the timed `ledger_masters` for B2 and B4), wait 120 s, taking a screenshot of Tally's whole window every 30 s, then ONE `GET /status` by rule 2's line (200). Then the next step.
7. **Marks.** `ledger_masters` with `fields` basic: `snapshot.master_alter_id`, `snapshot.voucher_alter_id`. Read at step 1 (M0, V0), after B2 (M1, V1, from the timed read or rule 5's later one), after B4 (M2, V2, the same way) and at the end (M3, V3). All read must be equal; a mark that moves = STOP.
8. **One Tally.** At step 0 and at the end, as C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.
9. **Keys.** The Gateway is the only screen used. No key is pressed except the arrows and Enter of Select Company at step 0, and the Company > Shut of C10. **Never Ctrl+Q or Escape at the Gateway of Tally.**

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` of master `609776ae9f5cec48e6563f165480d3fe1adae445` (merge-queue run 40366984202, artifact `unsigned-bundle-smoke-windows-latest` id 12670984633, `bridge_mcp.exe` sha256 `4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44`), the build `lab/2119-bills-read-timing` used. It is used here only for `list_companies`, `ledger_masters` and nothing else (either absent from `tools/list` = STOP). Record its sha256 before the first call and after the last; another sha256 = STOP.

Run it over stdio with:
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`;
- a new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2142\data`, listed first: absent or empty, else STOP);
- `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026);
- redaction unset. Writes stay off.

## Files
`RUN-SHEET.md`, `LAB-COMMON.md`, `probe.py` (the helper that sends a block's read and probes and logs each), `requests/r-4-receivable-20260331.utf16le.xml`, `.gitattributes`, `SHA256.txt`.

`probe.py` is run from this sheet's folder as `python -I probe.py --block <name> --out C:\bridge-lab\2142\answers [--read requests/r-4-receivable-20260331.utf16le.xml --read-sha256 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f] [--cut-after 20]`.
- **Exit codes:** 0 when the block ended as designed; 2 on a STOP, whose reason is in the log's `stop` line, with whether the read curl was still running and its pid. Its STOPs: a read or probe that did not answer as rules 2 and 3 say; a probe without an answer in 600 s; the operator's `STOP-<block>` file; 200 probes; 900 s in all (checked before each probe, so a block can last up to 900 s plus one probe's wait).
- **Each probe's log line** gives its number, whether it was sent after the read ended, curl's exit code and its `-w` fields.
- **The read's log lines:** `read_sent` gives its pid and `read_sent_t`. `read_ended` is logged by a thread the moment the read ends, with its exit code, its `-w` fields and `read_end_t` (`read_sent_t` plus the read's own `time_total`). So a probe's end is never compared with the read's by the order of log lines.
- It refuses to write into an existing log (it opens the log exclusively), so no block is run twice into one file.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I -X pycache_prefix=C:\bridge-lab\2142\pyc -m py_compile probe.py` succeeds (the prefix keeps compiled files out of this folder, after the check in 1). Read `probe.py` whole. It sends only the two command lines named in rules 2 and 3, and nothing else.
3. The run coordinator's check in rule 1.

## Steps
0. LAB-COMMON step 0 (C2), with no company loaded:
   - the screenshot;
   - the process list and port 9001;
   - `GET /status`;
   - hash the exe;
   - load THE COMPANY alone (arrow keys);
   - a screenshot of the Gateway with the Current Period and Tally's current date (EXPECT 31 March 2026).
1. Start ComplyEaze Bridge.
   - `tools/list` (saved whole).
   - `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401.
   - `ledger_masters` (basic): M0 and V0 (EXPECT 328 and 29900; any other = STOP). Record its seconds: the first of two times of this read at rest.
2. **A1**, at rest: `probe.py --block A1`. Ten probes.
3. **B1**, beside the whole read: `probe.py --block B1 --read requests/r-4-receivable-20260331.utf16le.xml --read-sha256 6520d94e…` (the whole sha256, as in Files). Then rule 6's wait and `/status`.
4. **B2**, beside the read cut at 20 s: `probe.py --block B2 --read … --read-sha256 … --cut-after 20`.
   - As soon as it exits 0 and its closing screenshot is taken, the timed `ledger_masters` (basic), by rule 5. M1 and V1 come from it, or from rule 5's later read.
   - Then rule 6's wait and `/status`.
5. **A2**, at rest: `probe.py --block A2`.
6. **B3**, as B1: `probe.py --block B3 --read … --read-sha256 …`. Then rule 6's wait and `/status`.
7. **B4**, as B2: `probe.py --block B4 --read … --read-sha256 … --cut-after 20`.
   - Then the timed `ledger_masters` by rule 5 (M2, V2).
   - Then rule 6's wait and `/status`.
8. **A3**, at rest: `probe.py --block A3`.
9. `ledger_masters` (basic): M3 and V3. Record its seconds: the second time of this read at rest.
10. Stop ComplyEaze Bridge (its stop file), and record its exit code and error stream. Hash the exe.
11. LAB-COMMON C10:
    - at the Gateway, a screenshot (Current Period and Tally's current date, unchanged from step 0);
    - Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway);
    - Select Company with none loaded;
    - one tally.exe;
    - 9001 LISTENING;
    - `GET /status` 200.

    No XML request is sent after the shut; `GET /status` is the only request after it.

STOP RULES:
- C7 and C8 of LAB-COMMON;
- any dialog;
- a second `tally.exe`;
- a mark that moves;
- an answer above its bound;
- a probe without an answer in 600 s, or any other request without one in 10 minutes (leave that curl running, send nothing);
- `probe.py` exiting 2;
- Ctrl+Q or Escape at the Gateway (an incident: if a prompt shows, press no key, take a screenshot and STOP; only if the process list shows NO `tally.exe`, C9);
- an exe sha256 that is not the one named.

## What the row records
For each block, a table: each probe's number, its start time since the block began (`t` of `probe_sent`), its `time_total`, whether it was sent before or after the read ended, and its status. Then:
- **A blocks:** the least, the most and the median of the ten `time_total`.
- **B1 and B3 (the decisive table):** the read's `time_total` and `read_end_t`. Probes are placed by `t` against `read_end_t` only, never by the log's `after_read` flag, which only drives the helper's own count. For each probe whose `probe_sent` `t` is more than 1 s before `read_end_t`: its end (`t` plus its `time_total`) against `read_end_t`, and its `time_total`. A probe sent within 1 s of `read_end_t` is listed as not decisive and left out of the reading. Each other probe is one of three:
  - **waited:** it ended no earlier than 0.5 s before `read_end_t`, in 1 s or more;
  - **answered quickly while busy:** it ended more than 0.5 s before `read_end_t`, in under 1 s;
  - **answered slowly while busy:** it ended more than 0.5 s before `read_end_t`, in 1 s or more.
  - **Reading:** if every such probe waited, `/status` waits behind the read on this Tally. If any answered quickly while busy, `/status` answers while Tally is building, and is not a signal that Tally is busy. Slow answers while busy are recorded as a third result. None is a STOP.
- **B2 and B4:**
  - the drain's length as `/status` sees it: from `read_end_t` (the cut) to the end of the last probe that took 1 s or more and ended after the cut, or "none was slow after the cut". The end of the third quick probe in a row is given beside it, as a second figure, since it includes the probes' 2 s gaps;
  - each probe's `time_total` in between;
  - the timed `ledger_masters`: answered or refused, its seconds, and the seconds between the block's `end` line and its send, against the slower of the two times at rest (steps 1 and 9).
  - **Reading:** answered within twice that time at rest or that time plus 2 s, whichever is larger, says a quick `/status` after a cut meant Tally was ready for a data read. Slower, or refused, says it did not.

**Confidence:** one sitting, PARTIAL until repeated. **Not covered:**
- other reads and books;
- a cut read other than this one;
- Gold;
- a second process sending at the same time;
- ComplyEaze Bridge's own transport, since the probes here are curl's;
- the time between a cut and Tally's real end of work, which no request can see.

## Evidence
- **`probe.py`'s logs**, each whole, with their size and sha256.
- **The B1 and B3 read answers** stay on the lab PC, listed by size and sha256, with their headers pushed.
- **Each probe's body** (51 bytes) is listed by sha256, not pushed. **Each block's `<block>-stdout.txt`** is pushed with its log.
- **Every tool answer** whole, with its size and sha256.
- **The table above.**
- **The build's sha256** before and after.
- **Screenshots** listed by name, size and sha256 (never pushed).
- **A notes file** with UTC times and one line per step, kept as C11 sets it out (no roles and no people).
- THE COMPANY's GUID is already in the merge gate's registry. No real figure is recorded beyond the book's own synthetic ones.
