# Row 2399-D: on a book counted in several census spans, does a repeated `vouchers` call reuse every span and answer the same rows?

For #2399. Written 11 Nov 2026. Not for merge. **READS ONLY:** every request is a call of ComplyEaze Bridge's built tool through one stdio process. Nothing is created, altered, imported or restored, no raw request is sent, and no backup is needed (LAB-COMMON C5 and C6 do not apply: nothing this row sends changes the book). The only keys pressed in Tally are the load and the shut of the company. About 20 minutes. Confidence of everything here until the row runs: unverified.

## Question
#2482 (merged into master as f607114e) lets a repeated first-page `vouchers` call for the same company, window and book take the census of the call before it, instead of sending the census requests again. It does so only when the company's two marks, read afresh by the new call, equal the marks that census was counted at, within 600 s. Every part of the read is still admitted against the census by AlterID and GUID, so a census that no longer describes the book can only make the read refuse as `census_reused_mismatch`, never answer wrongly.

Row 2399-R measured that once, on `BRIDGE PILOT LAB`, whose voucher mark (68) fits one census span: the repeat sent 16 requests where the first call sent 22, with the same 11 rows (`lab/2399-r-answers` at 0500c600). #2482 lists as not measured "a divided (multi-span) read with reuse, and a book whose voucher mark is above the census capacity". This row asks that on a synthetic book whose mark is well above the capacity, one variable only (the book), with the call shape row 703-C already ran on it. On that build this day was read in 2 data parts (703-C's relay log), so the read is divided both ways: the census in spans and the window in parts.
1. **D1, the first call.** On a book whose census needs several span reads, does the call count every span and hold the census?
2. **D2, the same call again, nothing changed.** Does it send no census request at all, plan the same parts, return D1's rows exactly, and say when the held census was taken?

How a request count is read is fixed below: it comes from Bridge's egress receipt of each call (`request_trail`), as row 2399-R read it.

## Book
`BRIDGE CORPUS DENSE` (synthetic, about 30,000 vouchers), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3` (listed in `scripts/merge-gate-synthetic-uuids.json`), loaded alone in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2399-R's) applies in full; where it and this sheet differ, the stricter one holds. C2 d as applied: step 0 runs with no company loaded, and its MARKS read is met by step 1 (`list_companies` and the `ledger_masters` marks); no raw MARKS request is sent, as in rows 703-C and 703-LO.

- **Its last recorded state:** `ALTMSTID` (master) 328 and `ALTVCHID` (voucher) 29900, books from 1 April 2025, Tally's current date 31 March 2026. Rows 703-C and 703-LO (9 Nov 2026, both reads only) read exactly these at their start and at their end (`lab/703-c-census-answers`, `lab/703-lo-sizing-answers`). Any other marks at step 1 stop the row before the first `vouchers` call: an unexplained move means the book was written since, and the estimates below hold only from 328 and 29900.
- **The day.** 15 May 2025, the day row 703-C read twice through the build of ae4bada2: `result.total` 82 both times, the rows equal, and every census answer byte-equal between the two calls (its `CENSUS EQUAL: 8 of 8 answers`). That is why this day: the census Tally gives for it is already shown to be stable while the marks are unchanged, so a D2 that differs from D1 points at the reuse, not at the book.
- **Why this book.** The census is sent in AlterID spans of at most 8,192 vouchers (`census_capacity`, the transport's 32 MiB response cap over 4 KiB a census row). A mark of 29900 needs 4 spans, where row 2399-R's book needed 1. It is a registered synthetic book that the lab has read, with its marks recorded, at a mark above 8,192. Whether other registered books have such a mark is not checked here: the registry holds identifiers, not marks.
- **The book check**, as row 2399-R recorded it (`evidence/bookcheck.txt` there): the one data folder whose `Company.1800` holds the name and the book's GUID, and the lab's private exclusion list, checked by the bookcheck, does not contain the book's GUID. Not exactly that: STOP before the first call.
- Write down the release shown on Tally's screen. Rows before this one read TallyPrime with `tally.exe` 1.1.7.2 from the program's file.

## Build
The Windows bundle of master's own CI run on d69b2ea8 (`d69b2ea8f71394eb7a948dcada541079e899eaa0`, the head of master when this sheet was written, which holds #2482). The run is the `ci.yml` push run GitHub lists for that head commit on `master`, conclusion success. The artifact is `unsigned-bundle-smoke-windows-latest`, 60,401,470 bytes, GitHub digest `sha256:ba5382c6afae35ecb7f94ce7dc887563cba1a2df8da8d6c75853eef0ca346e83`, expiry 2026-11-17T18:34:28Z. These are the API's values (the run's `artifacts` list); the build was not downloaded for this sheet. The run's id and the artifact's id are left out of this text, because the pre-push gate reads a run of eleven digits as a phone number: the lab looks the run up by its head commit (`actions/runs?branch=master&event=push`, the one whose `head_sha` is the commit above) and records both ids in RESULTS.md. If the artifact has expired by the sitting, STOP: the build is not remade here.

At step 0, before the first call:
- Check the zip's sha256 against the digest, and `bridge_mcp.exe` against `mcpb-smoke.json`. Either differing is a STOP.
- Record the run's `head_sha` from the API (`actions/runs/<run>`): it must be the commit above, else STOP. As in row 2399-R, neither the bundle nor the exe names a commit, so the tie is the zip's digest, the artifact's run, and that run's `head_sha`; record all three.
- Record the exe's sha256 now and again after the last call. Another sha256 is a STOP.
- Close every other Bridge process first, or record that they ran, with none called.

## How the calls are made
Over stdio, directly on 9001, no relay, with the lab's stdio client as row 2399-R ran it (its `scripts/client.py` on `lab/2399-r-answers` at 0500c600: no cancel, no call limit, a 60 s wait after stdin closes, and never ending the server itself). Record the client and its sha256. Launch `bridge_mcp` once, never relaunched:
- Environment: `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), one new empty `BRIDGE_AGENT_DATA_DIR` (`C:\lab\2399-d\data`; a directory listing before the launch shows it absent or empty, else STOP), `BRIDGE_AGENT_REDACTION` unset, `BRIDGE_AGENT_ENABLE_WRITES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` absent.
- Each call is a `tools/call` with exactly the arguments below; each answer is saved whole by the client with its size and sha256. A call over 60 s is recorded beside its count, not a stop. A call without an answer in 10 minutes is never sent again: STOP, leave the client waiting, report. A call that returns an error is recorded with its code and not repeated; a refusal is a result. A JSON-RPC `error` with no `result` is not a tool answer: `answer.py` stops on it, and it is recorded whole as that call's result.
- Record `initialize` and `tools/list` whole. EXPECT `list_companies`, `ledger_masters` and `vouchers` listed.
- The data directory holds the egress receipts (`agent-egress.jsonl`). Find it with a directory listing after the first call: exactly one such file, else STOP.

**THE CALL** (the same arguments both times): `vouchers` with `{"company_guid":"d45bc1b0-e5e3-4261-b3b2-cce3915f42d3","from":"2025-05-15","to":"2025-05-15"}`. No other argument, so the first page, no `limit`. These are row 703-C's arguments exactly. The window is spelled the same both times, because the held census is keyed by the window exactly as sent.

**THE MARKS** read: `ledger_masters` with `{"company_guid":"d45bc1b0-e5e3-4261-b3b2-cce3915f42d3","fields":"basic"}`, reading `snapshot.master_alter_id` and `snapshot.voucher_alter_id` from the answer, as rows 703-C and 703-LO read them.

**Reading an answer.** The client saves the JSON-RPC message under its own key `"response"`; `answer.py` reads the message itself (row 2399-R's first deviation). So, after each `vouchers` call: `python3 -I rpc.py <client answer> C:\lab\2399-d\rpc\<label>.json`, which writes the `"response"` value alone to a new file and prints whether it parses equal; then `python3 -I answer.py` on that file. `rpc.py` stops (exit 1) on a missing or unparsable file, a `"response"` that is absent or not a JSON-RPC message (an object holding `result` or `error`), a target already present, or a copy that does not parse equal. It prints which of `result` and `error` the message holds. It was tried on hand-made files only: a first and a reused answer each copied and equal, then read by `answer.py` (census 4 and absent; 0 and `read_at`; `--same-items` True), and each stop case exited 1, `{"response": null}` included; `answer.py` on the client's own file stopped, as row 2399-R found.

**How the lab reads a request count**, as row 2399-R: Bridge appends a `response_prepared` record to `agent-egress.jsonl` before it writes each tool response (`docs/agent/README.md`, the egress paragraph on `request_trail`). For a call that sent requests to Tally the record holds `request_trail`: `sent` (the sends of the whole call), `omitted`, `failed`, and `last`, the last 32 sends each with `seq`, `kind`, `request_bytes`, `response_bytes`, `outcome` and `held_ms`. It holds no body, name or value.
- After every call run `python3 -I trail.py <egress file>` and copy its whole output into RESULTS.md. It counts fields; it never searches text.
- **The call's count is `sent` in the last `vouchers` record.** A call that sends more than 32 requests lists only the last 32, so `sent` is the count and `omitted` the rest; the listed kinds are then of the last 32 only. D1 is estimated at about 52 sends, so its record will carry `omitted`; that is expected, and `sent` is still the count.
- `trail.py` prints `trail none` for a record without a trail. On a `vouchers` record that is a missing receipt, never 0 sends: STOP.
- `trail.py` stops (exit 1) when the file is missing or empty, a line does not parse, a trail has no integer `sent`, or the listed sends do not number `sent` minus `omitted`. The number of `vouchers` records must equal the number of `vouchers` calls made so far, else STOP: a missing record is "the receipt is missing", not "no requests".
- **Calibration (step 1).** Before D1 is judged, `trail.py --expect list_companies 4` must pass, the count row 2399-R also met (and row 0.7.0-master-live-checks recorded twice). Exit 3 (CALIBRATION FAILED) is a STOP: the counter is measuring itself (AGENTS.md P5) and no count of this row is used.

## Files
`SHA256.txt` lists every file in this folder with its sha256 and size. Check each against it at step 0.
- **Byte-equal to row 2399-R's file of the same name** (`lab/2399-r-sheet` at 549815f1, folder `lab/2399-r/`): `.gitattributes`, `LAB-COMMON.md`, `trail.py`.
- **Changed from row 2399-R's:** `answer.py`, whose `--same-items` printed `items equal: True (0 and 0 items)` for two error answers. It now stops (exit 1) when either answer is an error or holds no `items` list; its header says so, and nothing else changed. Tried on hand-made files: two equal answers True, two error answers, an answer with no items and two with none each exit 1.
- **New in this row:** `ROW.md`, `rpc.py`.
- No request file: nothing is sent to Tally except through the tool.
- The `vouchers` and `ledger_masters` answers hold party names, so they stay on the lab PC, listed by size and sha256 (as rows 703-C and 703-LO kept theirs), with the tool list. The `rpc/` copies are the same content and stay with them. What `answer.py` and `trail.py` print is copied into RESULTS.md as text; if a voucher number there trips the pre-push gate, record the item count and the sha256 of the printed number list instead, and say so.

## Steps
**The prompt rule, at every step.** Any prompt, message or dialog in Tally: screenshot it, copy its text word for word, press no key, send nothing more, and report to the run coordinator (LAB-COMMON C7). That prompt is this row's result; the coordinator decides what is pressed.

**Nothing is keyed in Tally** after the book is loaded, up to the close. In Tally the Gateway is the only screen used.

0. **Staging and Tally.** Kit: check every file of this folder against `SHA256.txt`. Build: the Build section's checks. Either failing: STOP. Then LAB-COMMON step 0 (C2) with no company loaded: the screenshot, the process list and port 9001, `GET /status`. Load `BRIDGE CORPUS DENSE` alone from Select Company with the arrow keys and Enter (C1), Enter only once a screenshot shows the highlighted row reads `BRIDGE CORPUS DENSE`. Any prompt: stop, press nothing, and ask. The book check (Book section). A screenshot of the Gateway with Tally's current date: EXPECT 31 March 2026; another date is a STOP.
1. **Start Bridge, calibrate, read the marks.** Launch `bridge_mcp` as above; record `initialize` and `tools/list`.
   - `c0`: `list_companies` `{}`. EXPECT exactly the book, `identity_state` `verified_tuple`, `books_from` 20250401. Then `trail.py <egress file> --expect list_companies 4`: EXPECT exit 0 and `calibration ok`. Anything else: STOP before D1.
   - `m0`: THE MARKS. EXPECT master 328 and voucher 29900. Another value: STOP before D1, nothing more sent.
2. **D1, `c1`:** THE CALL. Record the time it started and ended (UTC, to the second), then `trail.py`, `rpc.py` and `answer.py`.
   - EXPECT no error; `result.total` 82, as row 703-C read it; a non-zero item count; `census.requests` above 1 (estimate 4, the spans of a mark of 29900 at 8,192 each); `census_reused` absent. Record the items and `window parts`, `sent` as S1, `omitted`, and the listed kinds.
   - **D1 errors, refuses, or does not answer `result.total` 82 with items:** record it whole and do not make D2. Go to step 4. There is no census to reuse, and two refusals would compare equal.
   - Estimate, from row 703-C's relay log of this call on this day (`lab/703-c-answers/relay/relay.jsonl`, build ae4bada2, before #2482): 52 sends, of which 24 are the 4 census spans at 6 sends each and 28 are the rest, including 2 data parts. So S1 about 52 and `window parts` 2. Row 2399-R's one-span book gave the same 6 a span (22 against 16). The build here is later, so S1 is recorded, and D2 is compared with it, not with 52.
   - `census.requests` 1 on this book: the census was not divided by span as the code says. Record whole and report before D2; D2 is still made.
3. **D2, `c2`:** THE CALL again, at once (well inside 600 s of D1's count), nothing sent to Tally in between and nothing keyed. `trail.py`, `rpc.py`, `answer.py`, then `answer.py --same-items rpc\c1.json rpc\c2.json`.
   - EXPECT no error; `result.total` 82; `census.requests` 0; `census_reused.read_at` between D1's start and end, and within about a second of D1's `response_prepared` time in the egress file; `--same-items` True; the same `window parts` as D1. `read_at` is stamped when the held census is stored, after D1's whole read stood (`HeldCensuses::hold`, `agent_voucher_window.rs:678`), not when its census reads finished; row 2399-R's was 0.65 s before its call ended.
   - `census.requests` is always written: a reused read writes an explicit 0 (`census` is a `RequestTally`, always serialized, `agent_voucher_window.rs:1615` and `:1665`; row 2399-R's R2 printed 0). So `census.requests None` from `answer.py` means the block was not read: the reuse is not established for that call; record it whole and report.
   - Estimate: `sent` (S2) equal to S1 minus 6 x D1's `census.requests`, so about 28 (703-C's 52 less 24). Record S1, S2 and their difference, and the listed kinds of both.
   - `census_reused_mismatch` on this book, items that differ from D1's, or `census.requests` above 0: report before step 4, with `m1` below read first.
   - Then `m1`: THE MARKS. EXPECT 328 and 29900, equal to `m0`. If they differ, the book moved during the row, and D2 says nothing about reuse at equal marks: record that.
4. **Close.** Close the client: it closes its stdin and `bridge_mcp.exe` exits by itself; `tasklist` shows no such process within 60 s, else STOP and do not end it. Hash the exe. LAB-COMMON C10: a screenshot at the Gateway with the current date unchanged; Company > Shut; Select Company with none loaded; one `tally.exe` owning 9001; `GET /status` 200. Keep the egress file whole with its sha256.

**Stop**, with no key pressed, if Tally's window shows any dialog (the prompt rule) or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. The book is unchanged by anything this row sends.

## Record
- Each answer's size and sha256; `trail.py`, `rpc.py` and `answer.py` output, whole.
- For each call, as text: the start and end time (UTC to the second), `sent`, `omitted`, `failed`, the listed kinds, `census.requests`, `census.ms`, `census_reused`, `window parts`, the item count and the voucher numbers (or their list's sha256, as in Files).
- The marks at `m0` and `m1`; the figures S1 and S2 and their difference.
- The egress file whole with its sha256, screenshots by name, size and sha256 (never pushed), the Gateway's date, the release on Tally's screen.
- The exe's sha256 before and after, the zip's digest, the run's `head_sha`, and the client with its sha256.

## Expected
Estimates, not predictions.
- D1 counts the census in 4 span reads and reads the day in 2 parts, in about 52 sends; D2 counts none, plans the same 2 parts, returns D1's first page exactly, and sends 24 fewer requests (6 a span), about 28.
- These come from row 703-C's relay log on an earlier build and from row 2399-R. The row records what the receipt and the answer say on this build.

## Decides
Each is PARTIAL: one book, one build, one release, one sitting, one day.
- **D2 as expected:** a repeated call reuses a census of several spans on a real book, with the same rows, and saves one census read a span. That is the saving on this book, once, as the receipt counts it; with row 2399-R it gives the saving at 1 span and at the count D1 reports. It does not say how often real use repeats a call.
- **D2's rows differ from D1's, or D2 refuses as `census_reused_mismatch`, with `m1` equal to `m0`:** on this one run, reuse of a multi-span census did not give D1's answer. That is a finding against the design, PARTIAL like the rest, and not a verdict on its safety. Record whole; the coordinator decides. Row 703-C showed this day's census answers byte-equal between two calls, so the book is not the first suspect. With `m1` not equal to `m0`, the book moved and the result says nothing about reuse.
- **D2 counts part of the census again** (`census.requests` above 0 but below D1's): the reuse is per span, not whole. Record whole and report.
- **D2 counts all of it again** with the marks unchanged: the reuse did not apply on this book. Record the marks, the times and both receipts.
- **D1 reports `census.requests` 1:** the span planning differs from the code read below; record whole and report.
- **A call errors:** its code and its `request_trail` are the result. A D1 error means D2 is not made.

**What these answers are for.** They show whether #2399's census reuse holds on a book of several spans (AGENTS.md P1). They are never a regression fixture for the build's output.

## Code admits the trigger
Lines are at master d69b2ea8, the build's commit.
- **Spans.** `census_spans` (`src-tauri/src/agent_voucher_window.rs:3220`) sends `ceil(mark / capacity)` census requests; `WindowReadLimits::census_capacity` (`:1485`) is twice the read budget over `CENSUS_WIRE_BYTES_PER_VOUCHER` (4 KiB, `:47`), with the budget half of `XML_RESPONSE_MAX_BYTES` (32 MiB, `src-tauri/crates/bridge-tally-transport/src/lib.rs:94`), so 8,192 a span.
- **The key and the marks.** `CensusKey::of` (`agent_voucher_window.rs:539`) is the company GUID, the window as sent, the book or one ledger and the census capacity. `FreshMarks` (`:562`) is what a lookup takes, and `HeldCensuses::find` (`:642`) drops an entry whose marks differ.
- **What is held.** `AdmittedCensus::of` (`:593`) is made only after a read that counted its own census stood; `HeldCensuses::hold` (`:654`) takes only that. The byte cap is `HELD_CENSUS_MAX_BYTES` (`:518`, 32 MiB; at 128 bytes a voucher, far above this book's census). The lifetime is the listing TTL, 600 s (set at `:627`; `LISTING_SNAPSHOT_TTL`, `src-tauri/src/agent_ledgers.rs:437`).
- **Where it applies.** `Server::read_entry_window_rows_reusing_census` (`src-tauri/src/agent_vouchers.rs:1350`), called at the `vouchers` first page (`:734`) and nowhere else.

## Not in this row
- A window of more parts than this day plans; a second-or-later page.
- The TTL, a moved mark, and a restore: row 2399-R measured each on a one-span book.
- A restored copy with the same marks and different vouchers (the case the part check exists for).
- A write through Bridge (`post_import`) and its drop hook.
- A ledger-scoped call, the other tools, and the desktop screens.
- How often a real person repeats a call.
- Any other book, build or release.
