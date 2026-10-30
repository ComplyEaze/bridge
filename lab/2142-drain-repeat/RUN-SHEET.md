# 2142 drain repeat (#2142): the cut-read block repeated with its timed read on time, and a new block whose first `GET /status` is sent just after the cut, read only, on the dense synthetic book

Not for merge. About 45 minutes, most of it the waits between blocks and the steps by hand. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
The first row (`lab/2142-status-while-busy`, answers `lab/2142-status-while-busy-answers` at 9baae790) found, on one sitting:
- **`/status` waits behind a long read on this Tally:** in B1 and B3 the probe sent while the Bills Receivable read was being built ended 0.06 s and 0.03 s after the read.
- **After the same read was cut at 20 s,** the probe sent 2 s into it ended 25.40 s (B2) and 8.86 s (B4) after the cut, so Tally's work ran on past the cut.
- **A quick `/status` after the cut meant ready, once:** in B2 the timed `ledger_masters` answered in 0.802 s against a threshold of 2.914 s. B4's timed read went out 24.39 s after its block ended, past the sheet's 15 s, so it was not decisive.

Two readings are still missing before ComplyEaze Bridge may hold the next request to a Tally after a cut until `/status` is quick again (#2142). This row takes each, one variable at a time:
1. **B4R repeats B4 as written,** with the one fault of the first sitting fixed: the timed `ledger_masters` goes within 15 s of the block's end (rule 5). **Expected reading:** it answers within the threshold, as B2's did: a quick `/status` after a cut means Tally is ready for a data read. An answer over the threshold, or a refusal, contradicts B2: both are then reported and no reading is given.
2. **B5 changes one thing from B2:** when the first probe is sent. B2 and B4 sent it 2 s after the read started, while the read was alive. The design's case is a probe sent after the cut, while Tally may still be working. In B5 no probe is sent until the cut read has ended; the first goes about 1 s after its end, and the next ones as before, until three in a row are quick. Then the timed `ledger_masters`, as in B4R. **Expected reading:** that first probe waits (it takes 1 s or more) and ends some seconds after the cut, as B2's and B4's ended 8.86 s to 25.40 s after it. How long it ends after the cut is how long Tally's work ran on, as `/status` sees it. A first probe that answers in under 1 s says either that `/status` sent after a cut is quick while Tally may still be working, or that Tally's work ended within about 2 s of this cut (a probe sent at about 1 s that waits for work ending before 2 s still takes under 1 s). The row cannot tell those two apart. The block's timed read, sent after three quick probes and 7 s or more after the cut, shows only whether Tally was ready at its own send. For the design the harmful case is a quick first probe followed by a slow or refused timed read, and that case is decidable.
3. **B6 repeats B5,** per P6.

**Block history, a named difference from the first row.** That row ran A1, B1 (whole read), B2, A2, B3 (whole read), B4, A3: B2 came after one whole read, and B4 after three heavy reads. This row runs A1, B4R, A2, B5, B6, A3, with no whole read: B4R is the first heavy read after the book is loaded, and B5 follows a cut read. B4R's command is byte for byte B4's, but what Tally did before it is not. A drain length that differs from B2's or B4's may come from that history; the readings above do not rest on the drain's length.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/2142-status-while-busy` at ac1e8e5a, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds, except for the named differences below.

**Who acts.** The lab does every step here on the synthetic company, on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs (its rule on names applies to what is pushed).

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, as the first row left it: marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026. If it is absent, or step 1's marks differ (record the difference), STOP and report; the run coordinator decides. Only THE COMPANY is loaded, alone, from its load in step 0 to the last step.

Named differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Two requests at once, only inside a B block.** C3 sends one request at a time. Inside a B block, `probe.py` sends the one read and, beside it or after it, `GET /status` probes one at a time: never more than one probe in flight, and never more than two requests in all.
   - **Outside a B block,** one request at a time, as C3 says.
   - **No call of ComplyEaze Bridge is made while `probe.py` runs.**
   - **Between two screenshots,** probes go on being sent: a named exception to C3's "nothing is sent while a dialog is up", since a dialog is seen only at the next screenshot. In B5 and B6 this covers the first probe, sent about 1 s after the cut with no capture between. Rule 4's stop file ends the probes at once when one is seen.
   - The first row sent `/status` beside a running read on this Tally, with no dialog and no rule against it found: "private record checked on 30 Oct 2026; no rule found; concurrent use not previously measured". **Before this sitting, the run coordinator checks the private record of gateway hazards again** for any rule added since. A rule against it means the row is not run. The notes record exactly: "private record checked on <date>; no rule found" or the rule's existence, with no content.
2. **Probe timing.** Each probe is `curl.exe -sS -w "%{http_code} %{size_download} %{time_total}" -o <file> http://127.0.0.1:9001/status`, with no `--max-time`, as C3 says. This sheet's line is the authority: LAB-COMMON C2 c gives no command text. A probe counts as answered only with exit 0, status 200 and 51 bytes (the body the first row recorded). Anything else is `probe.py`'s STOP. A probe without an answer in 600 s is C3's "without an answer": STOP. `probe.py` leaves that curl running and sends nothing more.
3. **The read.** Each read is C3's line with `%{time_total}` added to `-w` and `--max-time 20`, sending `requests/r-4-receivable-20260331.utf16le.xml`: 770 bytes, sha256 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f, a byte copy of the request the first row sent.
   - **Size bound:** 20,000,000 bytes, as the first row set for this read (a named difference from C3's 2,000,000), since a read that answers before the cut is the whole 13,679,490-byte answer. Above the bound = STOP.
   - **`--max-time 20` is one named exception** to C3's "no --max-time". EXPECT curl exit 28 (a client timeout), with no whole answer. That ending is this row's subject, not C3's "ended without one = STOP".
   - **Sends:** the cut read is sent three times in the row (B4R, B5, B6), each a named exception to C3's "never sent again", because repeating it is what the row measures.
   - **Checks by `probe.py`:** it refuses to send unless the request file's sha256 is the one above (`--read-sha256`). A cut read that answers whole within 20 s is logged as `read_answered_before_cut`, a result to record, not a STOP. A cut read that ends any other way than exit 28 or exit 0 (a refused or reset connection, say), or that exits 0 without status 200 and some bytes, is a STOP: such a read may never have reached Tally, or answered something else, and its block would measure nothing.
4. **Running a block.** ComplyEaze Bridge is started at step 1 and stays idle during every block: no call is made to it then, and it sends nothing to Tally of itself. `probe.py` is run from this sheet's folder, in the background, by exactly this, in PowerShell:
   `$p = Start-Process -PassThru -NoNewWindow -FilePath python -ArgumentList @('-I','probe.py', <the block's arguments>) -RedirectStandardOutput C:\bridge-lab\2142r\answers\<block>-stdout.txt -RedirectStandardError C:\bridge-lab\2142r\answers\<block>-stderr.txt; $null = $p.Handle`
   and, once it has ended, `$p.WaitForExit(); $p.ExitCode`. While it runs, the only other act is screen capture. `-ArgumentList` takes each argument as its own array element, written out in full as in the steps. Exit 0 is the block as designed. Exit 2 is a STOP (below). Exit 1 is a STOP too: either `probe.py` refused before sending anything (its stderr says why), or it failed with a Python error, maybe after the read was sent. Any other exit code is a STOP. Both files are kept.
   - **A dialog during a block:** create the empty file `C:\bridge-lab\2142r\answers\STOP-<block>`. `probe.py` checks for it before each probe and then stops sending. That is the only way to stop it; never end a process. Then C7.
   - **Screenshots:** a screenshot of Tally's whole window just before each block starts, every 30 s while it runs, and straight after it ends (before anything else, the timed `ledger_masters` included). Screen capture only: no click, no focus change, no key. A dialog in any of them is C7.
   - **The capture during a block** is taken by a watcher that never waits more than 1 s at a time between its checks. It checks each second whether `probe.py` has exited, takes its 30 s captures on the way, and ends by itself within 2 s of that exit. Nothing waits for a capture that is not yet due. (In the first row, the watcher slept in 30 s steps and the block runner waited for it, which put B4's timed read 24.39 s late.)
   - **The answers folder** is `C:\bridge-lab\2142r\answers`, created empty before step 1 and listed (absent or empty, else STOP). `--out` names it in every block.
   - **On exit 1:** send nothing more and end nothing. List the `curl.exe` processes (`tasklist /FI "IMAGENAME eq curl.exe"`). If the log has a `read_sent` line and no `read_ended` line, wait for that curl to end on its own, for up to 10 minutes, then record the read's headers file as it is. Then report.
   - **On exit 2** (a STOP): send nothing more, and do not end any curl. A probe without an answer (`probe_unanswered`) is also left running, with its pid in the `stop` line and its body file kept. `probe.py` has already waited up to 600 s for a read curl still running to end on its own, and logged its result (`stop_wait_ended`). If that wait ran out, record the read's answer file size and sha256 and its headers file as they are. Then report.
5. **The timed `ledger_masters` after B4R, B5 and B6** is sent once, as soon as the block's exit and its closing screenshot allow: aim for under 10 s after the UTC of the block's `end` log line, and never later than 15 s by design. It is never repeated. Record the UTC of the `end` line and of the call's send, and the seconds between them.
   - **Sent more than 15 s after the `end` line** (it is still sent, once): the reading of that block is not decisive. Record why it was late.
   - **Answered:** record its seconds, its marks and the answer whole.
   - **Refused:** whatever the code or cause (a refusal for time among them), that is a result. Record the code, cause and next step text whole, and its seconds. Then the marks are read once, after the wait in rule 6, by another `ledger_masters` (basic), sent only if `GET /status` answered 200 and no dialog is up. A `GET /status` that does not answer 200 there is C3's STOP, and the marks are then read as C8 allows, or not at all.
6. **Waits between blocks.** After every B block (after its timed `ledger_masters`), wait 120 s, taking a screenshot of Tally's whole window every 30 s, then ONE `GET /status` by rule 2's line (200). Then the next step.
7. **Marks.** `ledger_masters` with `fields` basic: `snapshot.master_alter_id`, `snapshot.voucher_alter_id`. Read at step 1 (M0, V0), after B4R (M1, V1, from the timed read or rule 5's later one), after B5 (M2, V2, the same way), after B6 (M3, V3, the same way) and at the end (M4, V4). All reads must be equal; a mark that moves = STOP.
8. **One Tally.** At step 0 and at the end, as C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.
9. **Keys.** The Gateway is the only screen used. No key is pressed except the arrows and Enter of Select Company at step 0, and the Company > Shut of C10. **Never Ctrl+Q or Escape at the Gateway of Tally.**

C1 and C2 as applied, a named difference: step 0, its `GET /status` included, runs with no company loaded, and THE COMPANY is loaded straight after it, so "only THE COMPANY is loaded" holds from that load. C2 d's MARKS read is met by step 1's `list_companies` (exactly one company) and `ledger_masters` snapshot marks: no raw MARKS request is sent.

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` of master `609776ae9f5cec48e6563f165480d3fe1adae445` (merge-queue run 40366984202, artifact `unsigned-bundle-smoke-windows-latest` id 12670984633, `bridge_mcp.exe` sha256 `4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44`), the build the first row used. It is used here only for `list_companies`, `ledger_masters` and nothing else (either absent from `tools/list` = STOP). Record its sha256 before the first call and after the last; another sha256 = STOP.

Run it over stdio with:
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`;
- a new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2142r\data`, listed first: absent or empty, else STOP);
- `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026);
- redaction unset. Writes stay off.

## Files
`RUN-SHEET.md`, `LAB-COMMON.md`, `probe.py` (the helper that sends a block's read and probes and logs each), `requests/r-4-receivable-20260331.utf16le.xml`, `.gitattributes`, `SHA256.txt`.

`probe.py` is the first row's helper with one option added, `--first-probe-after-cut`, which B5 and B6 use: its argument, its check, one line in the reaping thread that notes when the read ended, and the wait before the first probe. Every other line is as the first row ran it. It is run from this sheet's folder as `python -I probe.py --block <name> --out C:\bridge-lab\2142r\answers [--read requests/r-4-receivable-20260331.utf16le.xml --read-sha256 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f] [--cut-after 20 [--first-probe-after-cut 1]]`.
- **`--first-probe-after-cut 1`:** no probe is sent until the read curl has ended. The helper then logs `first_probe_due` and sends the first probe 1 s after it saw the read end; the next ones go as before, 2 s after the previous ended, until three in a row sent after the read's end are quick. It refuses to start without `--cut-after`, or with a value not above 0 and at most 5.
- **Exit codes:** 0 when the block ended as designed; 2 on a STOP, whose reason is in the log's `stop` line, with whether the read curl was still running and its pid. Its STOPs: a read or probe that did not answer as rules 2 and 3 say; a probe without an answer in 600 s; a read with no end 600 s after it was sent (`read_unanswered`); the operator's `STOP-<block>` file; 200 probes; 900 s in all (checked before each probe, so a block can last up to 900 s plus one probe's wait). Exit 1 is a refusal before anything was sent, or a Python error (rule 4 says what to do).
- **Each probe's log line** gives its number, whether it was sent after the read ended, curl's exit code and its `-w` fields.
- **The read's log lines:** `read_sent` gives its pid and `read_sent_t`. `read_ended` is logged by a thread the moment the read ends, with its exit code, its `-w` fields and `read_end_t` (`read_sent_t` plus the read's own `time_total`). So a probe's end is never compared with the read's by the order of log lines.
- It refuses to write into an existing log (it opens the log exclusively), so no block is run twice into one file.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I -X pycache_prefix=C:\bridge-lab\2142r\pyc -m py_compile probe.py` succeeds (the prefix keeps compiled files out of this folder, after the check in 1). Read `probe.py` whole. It sends only the two command lines named in rules 2 and 3, and nothing else.
3. The run coordinator's check in rule 1.
4. The capture watcher of rule 4 is the one that checks each second; the notes say so.

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
2. **A1**, at rest: `probe.py --block A1 --out C:\bridge-lab\2142r\answers`. Ten probes.
3. **B4R**, the first row's B4 again: `probe.py --block B4R --out C:\bridge-lab\2142r\answers --read requests/r-4-receivable-20260331.utf16le.xml --read-sha256 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f --cut-after 20`.
   - As soon as it exits 0 and its closing screenshot is taken, the timed `ledger_masters` (basic), by rule 5. M1 and V1 come from it, or from rule 5's later read.
   - Then rule 6's wait and `/status`.
4. **A2**, at rest: `probe.py --block A2 --out C:\bridge-lab\2142r\answers`.
5. **B5**, the first probe after the cut: `probe.py --block B5 --out C:\bridge-lab\2142r\answers --read requests/r-4-receivable-20260331.utf16le.xml --read-sha256 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f --cut-after 20 --first-probe-after-cut 1`.
   - Then the timed `ledger_masters` by rule 5 (M2, V2).
   - Then rule 6's wait and `/status`.
6. **B6**, as B5: `probe.py --block B6 --out C:\bridge-lab\2142r\answers --read requests/r-4-receivable-20260331.utf16le.xml --read-sha256 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f --cut-after 20 --first-probe-after-cut 1`.
   - Then the timed `ledger_masters` by rule 5 (M3, V3).
   - Then rule 6's wait and `/status`.
7. **A3**, at rest: `probe.py --block A3 --out C:\bridge-lab\2142r\answers`.
8. `ledger_masters` (basic): M4 and V4. Record its seconds: the second time of this read at rest.
9. Stop ComplyEaze Bridge (its stop file), and record its exit code and error stream. Hash the exe.
10. LAB-COMMON C10:
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
- a probe without an answer in 600 s, or any other request without one in 10 minutes (leave that curl running, send nothing);
- `probe.py` exiting with any code but 0;
- Ctrl+Q or Escape at the Gateway (an incident: if a prompt shows, press no key, take a screenshot and STOP; only if the process list shows NO `tally.exe`, C9);
- an exe sha256 that is not the one named.

## What the row records
For each block, a table: each probe's number, its start time since the block began (`t` of `probe_sent`), its `time_total`, whether it was sent before or after the read ended, and its status. Probes are placed by `t` against `read_end_t` only, never by the log's `after_read` flag, which only drives the helper's own count. Then:
- **A blocks:** the least, the most and the median of the ten `time_total`.
- **Every B block:**
  - the read's ending and `read_end_t` (the cut);
  - the drain's length as `/status` sees it: from `read_end_t` to the end of the last probe that took 1 s or more and ended after the cut, or "none was slow after the cut". The end of the third quick probe in a row is given beside it, as a second figure, since it includes the probes' 2 s gaps;
  - the timed `ledger_masters`: answered or refused, its seconds, and the seconds between the block's `end` line and its send, against the two times at rest (steps 1 and 8), both given, the threshold taken from the slower: twice that time, or that time plus 2 s, whichever is larger.
  - **Reading:** answered within the threshold says a quick `/status` after a cut meant Tally was ready for a data read; slower, or refused, says it did not. If the call went out more than 15 s after the `end` line, the reading is not decisive: record it.
- **B4R:** its reading set beside B2's from the first row (answered in 0.802 s against 2.914 s). Two decisive readings that agree settle "quick after a cut means ready" for this read on this book; a disagreement is reported with both, and no reading is given.
- **B5 and B6, the first probe:** its `probe_sent` `t` minus `read_end_t` (about 1 s by design; record it as measured), its `time_total`, and its end minus `read_end_t`. It is one of two:
  - **waited:** it took 1 s or more. Its end minus `read_end_t` is how long Tally's work ran on after the cut, as `/status` sees it;
  - **quick:** it took under 1 s. Then the block's timed read is the reading: answered within the threshold says Tally's work had ended by then; slower or refused says `/status` was quick while Tally was still working, and is not a drain signal after a cut.

  - **Either way,** the block's timed read is also a readiness reading on the same threshold, set beside B2's and B4R's: a timed read that is slow or refused after three quick probes contradicts them.
  - **A first probe of 0.5 s to under 1 s** counts as quick, but at rest a probe takes 0.003 s to 0.04 s: record its value and flag it when it is above ten times the slowest probe of A1 to A3.
  - **A read that answered before the cut** (`read_answered_before_cut`) leaves the block with no cut to measure: record it, and give no reading from that block.

  If B5 and B6 disagree, both are reported and no reading is given.

**Confidence:** one sitting, PARTIAL until repeated. **Not covered:**
- other reads and books;
- a cut read other than this one;
- Gold;
- a second process sending at the same time;
- a connection dropped after sending began, a size limit, or a send abandoned mid-flight: kinds of failure that, in #2142's design, owe a drain like a cut for time. None is measured here;
- ComplyEaze Bridge's own transport, since the probes here are curl's;
- the time between a cut and Tally's real end of work, which no request can see; `/status` sees it only as far as a probe waits.

## Evidence
- **`probe.py`'s logs**, each whole, with their size and sha256.
- **Each block's read headers file**, pushed.
- **Each probe's body** (51 bytes) is listed by sha256, not pushed. **Each block's `<block>-stdout.txt`** is pushed with its log.
- **Every tool answer** whole, with its size and sha256.
- **The tables above.**
- **The build's sha256** before and after.
- **Screenshots** listed by name, size and sha256 (never pushed).
- **A notes file** with UTC times and one line per step, kept as C11 sets it out (no roles and no people).
- THE COMPANY's GUID is already in the merge gate's registry. No real figure is recorded beyond the book's own synthetic ones.
