# Row 2399-R: does the census a repeated `vouchers` call reuses stand on a real book, and is it dropped when the company's marks move, when ten minutes pass, and after a restore?

Live check of draft pull request #2482 (the build of `fix/2399-reuse-vouchers-census`), for #2399. Written 10 Nov 2026. Not for merge. Reads only, through the built `bridge_mcp` over stdio; the two writes are raw imports sent once each by the runner. Confidence of everything here until the row runs: unverified.

## Question
Draft #2482 lets a repeated first-page `vouchers` call for the same company, dates and book take the census (the per-day AlterIDs and GUIDs) of the call before it, instead of sending the census requests again. It does so only when the company's two marks (`ALTVCHID`, `ALTMSTID`), read afresh by the new call, equal the marks that census was counted at, within 600 s of the count that made it. Every part of the read is still checked against the census by AlterID and GUID, so a census that no longer describes the book can only make a read refuse as `census_reused_mismatch`, never answer wrongly.

Everything the change claims was shown only against replayed answers (AGENTS.md P1): a repeat sends 16 requests where the first call sent 22, on a book counted in one census. This row asks the same questions of a real Tally, one variable each:
1. **R1, the first call.** Does it count its census and hold it?
2. **R2, the same call again, nothing changed.** Does it send no census, return the first call's rows, and say when the count was taken?
3. **R3, the same call after more than ten minutes, nothing changed.** Is the census gone?
4. **R4, a voucher created in the day, then the call.** Does the voucher mark move, and is the census counted afresh?
5. **R5, a ledger created, then the call.** The same for the master mark.
6. **R6, a day with no voucher, twice.** Does the empty-window check still count its own census on the repeat?
7. **R7, after the restore.** The marks go back from what a census was held at. Is it a miss?

How a request count is read is fixed below: it comes from Bridge's egress receipt of each call, not from a special log.

## The two writes are raw imports
`post_import` opens Bridge's approval window, which only the owner can answer, and the row would wait for an owner sitting. So Bridge writes nothing here: `BRIDGE_AGENT_ENABLE_WRITES` is absent, no approval window can open, and the two writes are raw imports sent once each by `run.py`, as the lab's other rows send them.
- **R4's write is `I-0399.xml`:** one voucher dated 1-Aug-26, the day R1 reads, with a fresh number (`BP/26-27/0399`) and a fresh REMOTEID (ending 2399). It is row 2464-RS's `I-0105.xml` with those two strings replaced and nothing else (`gen.py` checks every replacement is counted). The restore removes it.
- **R5's write is `I-L2399.xml`:** row 2346-PB's `L-ledger.xml` (`lab/2346-pb-sheet` at 53540665, sha256 `00ae11c3`), the ledger-create import that row 2346-TD sent again with another name, with only the name changed to `BRIDGE Census Probe 2399` (its two places, counted by `gen.py`; turned back it is PB's file byte for byte). It creates one bill-wise ledger under `Sundry Debtors`. No new shape is sent. If Tally refuses it, R5 is recorded as not run.
- **What this does and does not test.** A raw import moves the company's marks as a post would, and the marks are what the held census is compared with. It does not reach Bridge's own drop hook, which a write through Bridge triggers (`drop_voucher_pages`, `src-tauri/src/agent_vouchers.rs:990`). So this row tests the marks guard alone. The hook is shown by the unit tests only and is not measured here.
- **Unmeasured on this book:** `I-0105.xml`'s shape was measured creating on this book after two registrations existed (row 2464-RB). Here it is sent on the book as it was before those, with one GST registration. If the import is not `CREATED` 1 with every other counter 0, R4 is recorded as not run and the row goes on to R5.

## Book
`BRIDGE PILOT LAB` (synthetic), GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85` (listed in `scripts/merge-gate-synthetic-uuids.json`), loaded alone in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2464-RS's, 2464-RB's and 2346-TD's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** STOP before anything is keyed unless step 1's COMPANY row is the book's GUID. `lab.py marks` is given the GUID and stops on any other.
- **Its last recorded state, m0:** ALTVCHID 68 and ALTMSTID 245. Row 2464-RS ended its sitting with a restore that read exactly these marks back (`lab/2464-rs-answers`, its s8), and row 2346-TD's s1 read them before it. The row's own comparisons hold only from m0, so at step 1 any other marks stop the row, before BK and before any key. On 1-Aug-26 the book holds 11 vouchers at m0 (`BP/26-27/0001` to `0009`, `0011` and `0010`, all `BRIDGE Sales` to `BRIDGE Walk-in`). That is where R1's 11 comes from: row 2464-RS's Day Book for 1-Aug-26 had 12 lines with its own `BP/26-27/0105` last, and RS's restore removed `0105` (its s8 compared equal to its s2). So 11 holds only from m0, and at step 1 any marks other than 68 and 245 stop the row before BK. The Gateway's date of last entry read 15-Sep-26.
- **Why this book.** It is the one synthetic book with a recorded state and a restore path that the lab has used all day. Its voucher mark is 68, so its census is one span, and the saving is the 6 requests of that one span. A book with a mark above 8,192 would show the saving per span, but no such synthetic book is listed in the merge gate's registry (the registry holds identifiers, not marks), so none is used here; a second-span row needs its book registered first and is a later row.
- **BK is this row's way back, and the restore always runs.**
  - BK is taken at step 1, before anything is sent that changes the book, and restored at step 8 by C6.
  - Every path from step 4 on that is not a STOP reaches step 8, including a refused import and a failed compare.
  - After a STOP (C7, C8, C9), C8 holds: nothing more is sent or keyed, and BK is restored only on the run coordinator's word.
  - After the restore, every read is compared with step 2's. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows before this one read TallyPrime with `tally.exe` 1.1.7.2 from the program's file.

## Build
The Windows bundle of a CI run started by hand on `fix/2399-reuse-vouchers-census` (a draft pull request builds no bundle). The artifact is `unsigned-bundle-smoke-windows-latest`. The build is of the exact head of #2482 on the day it is made.

**Build line:** CI run 42787114741 (started by hand, `workflow_dispatch`, all jobs succeeded), commit `d40c42041f0383487fee4e851ab11f4e44f0df65` (#2482's head at the run's start), artifact id 13750966183 (`unsigned-bundle-smoke-windows-latest`), GitHub digest `sha256:7309fad953762ccba515297492cd8e7c92225dd04c87e6c9d1d546f8833cfa3f`, expiry 2026-11-17T09:14:10Z. The id, digest and expiry are the API's (`actions/runs/42787114741/artifacts`); the build was not downloaded for this line, and the lab fetches it itself and makes the step 0 checks below. If the artifact has expired by the sitting, STOP: the build is not remade here.

At step 0, before the first call:
- Check the zip's sha256 against the digest, and `bridge_mcp.exe` against `mcpb-smoke.json`. Either differing is a STOP.
- Record the exe's sha256 now and again after the last call. Another sha256 is a STOP.
- Record the commit the bundle names. It must be the commit on the Build line, else STOP before the first call.
- Close every other Bridge process first, or record that they ran, with none called.

## How the calls are made
Over stdio, directly on 9001, no relay, as rows 1029-R and 1108-A2 ran `bridge_mcp`. Launch it once for the part of the row up to the restore, never relaunched in between:
- Environment: `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), one new empty `BRIDGE_AGENT_DATA_DIR` (`C:\lab\2399-r\data`; a directory listing before shows it absent or empty, else STOP), `BRIDGE_AGENT_REDACTION` unset, `BRIDGE_AGENT_ENABLE_WRITES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` absent.
- **The client** is the lab's stdio client of 852-C1: no cancel and no call limit. Record it. Each call is a `tools/call` with exactly the arguments below; each answer is saved whole as JSON with its size and sha256. A call over 60 s is recorded beside its count, not a stop. A call without an answer in 10 minutes is never sent again: STOP, leave the client waiting, report. A call that returns an error is recorded with its code and not repeated; a refusal is a result.
- Record `initialize` and `tools/list` whole. EXPECT `vouchers` and `list_companies` listed.
- The data directory holds the egress receipts (`agent-egress.jsonl`). Find it with a directory listing after the first call: exactly one such file, else STOP.

**THE CALL** (the same arguments every time it is named): `vouchers` with `{"company_guid":"6b43e498-430c-4d5c-bfef-d32e2ab93c85","from":"2026-08-01","to":"2026-08-01"}`. No other argument, so the first page, no `limit`. The window is spelled the same every time, because the held census is keyed by the window exactly as sent. THE EMPTY CALL is the same with `"from":"2027-02-14","to":"2027-02-14"`, made only if step 1's raw day read for that day found 0 rows; no other day is ever used for it.

**How the lab reads a request count.** Bridge appends a `response_prepared` record to `agent-egress.jsonl` before it writes each tool response (`docs/agent/README.md`, the egress paragraph on `request_trail`, bridge#918). For a call that sent requests to Tally the record holds `request_trail`: `sent` (the sends of the whole call), `omitted`, `failed`, and `last`, the last 32 sends each with `seq`, `kind` (`post` or `status`), `request_bytes`, `response_bytes`, `outcome` and `held_ms`. It holds no body, name or value.
- After every call run `python3 -I trail.py <egress file>`. It parses each line as JSON and prints, per `response_prepared` record, its line, time, tool, receipt id, `sent`, `omitted`, `failed` and the counts by kind and bytes of the listed sends. It counts fields; it never searches text. Copy its whole output into RESULTS.md.
- **The call's count is `sent` in the last `vouchers` record.** A call that sends more than 32 requests lists only the last 32, so `sent` is the count and `omitted` the rest; the listed kinds are then of the last 32 only.
- `trail.py` stops (exit 1) when the file is missing or empty, a line does not parse, a trail has no integer `sent`, or the listed sends do not number `sent` minus `omitted`. It prints `no records` only for a file that parsed whole and holds no `response_prepared` record, with the number of lines it read. The number of `vouchers` records must equal the number of `vouchers` calls made so far, else STOP: a missing record is "the receipt is missing", not "no requests".
- **Calibration (step 3).** Before R1 is judged, `trail.py --expect list_companies 4` must match a count fixed before this row: `list_companies` was recorded live as 4 sends (post, status, post, status) twice, by row 0.7.0-master-live-checks (`lab/0.7.0-master-live-checks-answers`, `answers/015-c1-egress_log.json`). Exit 3 (CALIBRATION FAILED) is a STOP: the counter is measuring itself (AGENTS.md P5) and no count of this row is used.
- The egress file is kept whole with its sha256 at the close. It holds hashes, counts and field paths only (README).
- The answer's own `window` block is read by `python3 -I answer.py <answer>`: `census.requests` (the census reads of this call; 0 when reused), `census_reused` (`read_at` and any other field, or absent), the parts, the items and their voucher numbers.

## Files
`SHA256.txt` lists every file in this folder with its size. Requests are UTF-16LE with a BOM, as sent. Check each request's sha256 against it before its send.
- **Byte-equal to row 2464-RS's file of the same name** (`lab/2464-rs-sheet` at 29b06f41, folder `lab/2464-rs/`): `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml`, `run.py`.
- **Byte-equal inputs of other rows, used by `gen.py`** (it checks each input's sha256 first): row 2464-RS's `I-0105.xml` and `N-0105.xml`, row 2346-PB's `L-ledger.xml` (`lab/2346-pb-sheet` at 53540665), and the build's day-window `vouchers` request as sent live on 25 Sep 2026, `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-forex-composite-20260915.request.xml` (its `PROVENANCE.md` records the capture).
- **New in this row:** `I-0399.xml`, `N-0399.xml`, `I-L2399.xml` and `D-empty.xml` (written by `gen.py` from those four inputs, by counted string replacements only), `lab.py`, `trail.py`, `answer.py`.
  - `M-marks.xml` reads `GUID`, `ALTVCHID` and `ALTMSTID` of every company loaded.
  - `N-0399.xml` reads the vouchers numbered `BP/26-27/0399`, in the form of RS's `N-0105.xml`. `D-empty.xml` is the day-window request of that fixture with the company changed to `BRIDGE PILOT LAB` and the day to 14 Feb 2027 (four dates); it is the request shape the build sends for a day.
  - `lab.py marks|counters|rows` decodes and parses an answer as C4 asks. It was tried on 2464-RB's committed answers: `marks` printed the book's GUID with 68 and 245 and `--expect 69 245` exited 2; `counters` printed `CREATED 1`, the rest 0 and `LASTVCHID 45`; `rows` printed one voucher on `a-s5r-N-0105.xml` and none on `a-s2-N-0105.xml`; `counters` on a marks answer stopped.
  - `trail.py` was tried on the 40 real receipts of `lab/0.7.0-master-live-checks-answers` (`answers/015-c1-egress_log.json`): it parsed all 40, printed `sent` 4 for `list_companies`, and `--expect list_companies 4` passed; `--expect list_companies 5` exited 3. It was also tried on hand-made files: a missing file and a line that does not parse exit 1, a log with no `response_prepared` record prints `no records`, and a trail whose listed sends do not number `sent` minus `omitted` exits 1. `answer.py` was tried on hand-made files only (an answer, an error answer), since no `vouchers` answer of a real call was at hand. They are bounded: 200 records, 4,000,000 bytes.
- Answers that hold a GSTIN, or a run of ten or more digits outside a `VOUCHERKEY` value, stay on the lab PC under the lab's standing rules, listed by size and sha256. That covers the `vouchers` answers if any carries one; their `window` block and voucher numbers are copied into RESULTS.md as text.

## Steps
Raw requests run as `python3 -I run.py http://localhost:9001 <label> <files>`, then `lab.py` on the answer. A STOP from any script stops the row there. The rules of rows 2464-RS and 2346-TD stand:

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, with the keys they name. For any other prompt, message or dialog: copy it word for word and screenshot it first; answer No, or Esc if it offers no No; press nothing more and send nothing; leave any screen still open by Ctrl+Q (C7); go to step 8; that prompt is this row's result.

**The wait rule.** After a raw import's send and after a restore: if 5 minutes pass with no Gateway, no answer and no message, send nothing, press nothing, screenshot the whole window and report to the run coordinator. A read sent while Tally is busy can hold its gateway. With the Gateway showing and no dialog, run C2's status check before the next request.

**Nothing is keyed in Tally** after BK is taken, up to the restore, except the restore itself. The two writes are raw imports.

0. **Staging, before Tally.** Kit: check every file of this folder against `SHA256.txt`. Build: the Build section's checks. Either failing: STOP.
1. **Load, check and back up.**
   - C2's step 0 with no company loaded. Load `BRIDGE PILOT LAB` alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Book check, as rows 2346-TD and 2464-RS recorded it (`evidence/bookcheck.txt`): the one data folder whose `Company.1800` holds the name and the book's GUID, and the lab's private exclusion list, checked by the bookcheck, does not contain THE COMPANY's GUID. Not exactly that: STOP before BK.
   - Write down the Gateway's current date and period (TD read 1-Apr-26 to 31-Mar-27, Tuesday 15-Sep-2026).
   - `s1`: `M-marks.xml`, then `python3 -I lab.py marks a-s1-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85 --expect 68 245`. EXPECT exit 0. Exit 2 or 1: STOP here, before BK, with nothing keyed.
   - `se`: `D-empty.xml`, the raw day read for 14 Feb 2027. The day is chosen, not known to be empty: it is an estimate until read, and it is read here and again at `s8`, EXPECT 0 both times. Sent once by `run.py` at each place; then `lab.py rows a-se-D-empty.xml`. EXPECT `rows 0`. Any row, or a STOP from the script: R6 is skipped and recorded and no other day is chosen at the PC; the row goes on. (A read, so it changes nothing.)
   - **Take BK** by C5 to the new empty folder `C:\lab\2399-r\backup-1`. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `M-marks.xml N-0399.xml`.
   - EXPECT `marks` equal to s1's (68, 245) and `rows` 0 for `N-0399.xml`. Anything else: STOP before step 3, nothing sent but reads and nothing keyed but BK. Close by C10.
3. **Start Bridge, and the first three calls.** Launch `bridge_mcp` as above; record `initialize` and `tools/list`.
   - `c0`, the calibration: `list_companies` `{}`. EXPECT one company, the book's GUID. Run `python3 -I trail.py <egress file> --expect list_companies 4`. EXPECT exit 0 and `calibration ok`, with the listed kinds post 2 and status 2. Anything else (exit 3, or the kinds differ): STOP before R1 is judged; no count of this row is used. If the build's `list_companies` changed since that record, say which commit changed it and report; the coordinator decides.
   - **R1, `c1`:** THE CALL. Record the time it started and ended (UTC, to the second), then `trail.py` and `answer.py`.
     - EXPECT 11 items (the day's vouchers by the Day Book; the count N1 the tool returns is recorded and the later items are compared with it), `census.requests` 1 (one span), `census_reused` absent, no error. Record `sent` as S1.
     - Estimate: S1 = 22 on a one-census read as the replays count it. Live sends include a status probe and the identity read, so S1 may differ. It is recorded, and later calls are compared with it, not with 22.
   - **R2, `c2`:** THE CALL again, at once, nothing sent to Tally in between and nothing keyed. `trail.py` and `answer.py`.
     - EXPECT `census.requests` 0; `census_reused.read_at` equal to the time R1's read finished (R1's `read_at` is the finish time of the counting read, not the call's start); the same items (`answer.py --same-items a-c1 a-c2` equal); no error.
     - EXPECT `sent` (S2) equal to S1 minus 6. Estimate; the replays count 6 legs a census read. Record S1, S2 and their difference.
     - Any `census_reused_mismatch` on this untouched book, or items that differ from R1's: report before the next step. It is the design's failure.
   - **R3, `c3`:** wait until at least 11 minutes (660 s) have passed since R1's read finished, with nothing sent to Tally, nothing keyed and no other call to the client in the wait. Record the clock at R1's finish, at the start of `c3`, and the seconds between: the wait is recorded, not set. Then THE CALL.
     - EXPECT `census.requests` 1, `census_reused` absent, the same items as R1's, `sent` equal to S1. The held census is older than ten minutes and is not used. Record S3.
     - `census_reused` present after 660 s: the TTL did not hold. Report before the next step.
4. **R4: a voucher created in the day.**
   - `s4m`: `M-marks.xml`, then `lab.py marks a-s4m-M-marks.xml --expect 68 245`. It must pass. Then no key is pressed in Tally before the send. `c3` has just held a fresh census at these marks.
   - `s4`: `I-0399.xml`, sent once by `run.py`. Then the wait rule. `lab.py counters`: EXPECT `CREATED` 1 and ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED and EXCEPTIONS 0, with no LINEERROR. Record LASTVCHID and LASTMID. A screenshot of the whole window after the send: the Gateway, no dialog.
   - **If the counters are not that:** record them word for word, never send `I-0399.xml` again (C3), and go to step 5. R4 is not run.
   - `s4r`: `M-marks.xml N-0399.xml`. EXPECT ALTVCHID 69 (up by one) and ALTMSTID 245, and `rows` 1 for `N-0399.xml`. Record the marks as read. Marks that did not move are a result; the call below is still made.
   - `c4`: THE CALL, at once. `trail.py`, `answer.py`.
     - EXPECT `census.requests` 1 and `census_reused` absent (the marks moved, so the census is counted again), N1 + 1 items: R1's and `BP/26-27/0399`. `sent` equal to S1. Record S4.
     - `census_reused` present with the voucher mark up: report at once. If the read did not refuse, it answered from a census the new voucher is not in, which the part check should make a refusal.
   - `c4b`: THE CALL again, at once. EXPECT `census_reused` present, 0 census reads, the same items as `c4`'s, `sent` equal to S2. This shows the store is held again at the new marks.
5. **R5: a ledger created.**
   - `s5m`: `M-marks.xml`, then `lab.py marks a-s5m-M-marks.xml --expect 69 245` (68 if step 4 did not run). It must pass; if not, STOP.
   - `s5`: `I-L2399.xml`, sent once. The wait rule and `lab.py counters` as in step 4, EXPECT the same counters. A screenshot of the whole window after it. If the counters are other: record them, never resend, go to step 6 (R5 is not run).
   - `s5r`: `M-marks.xml`. EXPECT ALTMSTID 246 and ALTVCHID unchanged. Record the marks as read; a master mark that did not move is a result. No ledger read is made: `CREATED` 1 and the moved master mark are what this row needs, and the restore compare covers the rest.
   - `c5`: THE CALL, at once. EXPECT `census_reused` absent, `census.requests` 1, the same items as `c4`'s, `sent` equal to S1. Record S5.
6. **R6: a day with no voucher.**
   - `c6`: THE EMPTY CALL. `trail.py`, `answer.py`. EXPECT 0 items and no error. If step 1's `se` did not read 0 rows, `c6` and `c7` are not made and R6 is recorded as skipped. If the answer holds items, R6 is recorded as not run and the row goes on to step 7. Record `census.requests`, `census_reused` and `sent` as E1. The empty-window check reads a wider count of its own to corroborate an empty answer, and its requests are in `sent`.
   - `c7`: THE EMPTY CALL again, at once. Record `census.requests`, `census_reused` and `sent` as E2.
     - Estimate, unmeasured at the tool level: the window's own census is reused and the corroboration's is counted again, so E2 is E1 minus 6 times the window's census reads in E1, and E2 is still above 16. The review of the change listed this case as untested.
     - E2 equal to E1: nothing was reused for an empty window. E2 at 16 or below: the corroboration's census was not sent the second time, which the design says must not happen. Report either before step 7.
7. **Before the restore.** `c5b`: THE CALL, to hold a census at the current marks (the raised ones). Record when its read finished. The client stays open and idle through the restore; no further call is made until step 8. Keep the egress file's sha256 now.
8. **Restore** BK by C6, with `restore-plan.txt` written first.
   - `s8`: `M-marks.xml N-0399.xml D-empty.xml`. EXPECT `lab.py marks a-s8-M-marks.xml --expect 68 245` passes, and `rows` 0 for both other reads. Anything else stops the row: send nothing more and report before the close.
   - **R7, `c8a`:** THE CALL on the first client, which holds `c5b`'s census at the raised marks. Record the seconds since `c5b`'s read finished.
     - EXPECT `census_reused` absent, `census.requests` 1, items equal to R1's, `sent` equal to S1, no error. The marks went back to 68 and 245, which differ from the held census's, so it is a miss.
     - If those seconds are 660 or more, the miss is the TTL's and says nothing about the marks: record that and nothing more. At 540 or less it is the marks'. Between, record it and read it as unmeasured.
   - Close the first client: it closes its stdin and `bridge_mcp.exe` exits by itself; `tasklist` shows no such process within 60 s, else STOP and do not end it.
   - **The control, `c8b`:** launch `bridge_mcp` again with the same environment and a second new empty data directory (`C:\lab\2399-r\data-2`) and make THE CALL once. EXPECT the same as `c1`: items equal to R1's, `census_reused` absent, `census.requests` 1, `sent` equal to S1. Run `trail.py` on that directory's file. This is the control that the restore put the book back. Close the second client as above.
   - Record each answer's sha256 against s2's and s1's where they were reads of the same request.
9. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds: a stop after step 4's send leaves the voucher `BP/26-27/0399` (if created); after step 5's send, also the ledger.

## Record
- Each answer's bytes and sha256; `lab.py`, `trail.py` and `answer.py` output, whole.
- For each call, as text: the start and end time (UTC to the second), `sent`, `omitted`, `failed`, the listed kinds, `census.requests`, `census_reused` and the voucher numbers.
- The marks read at every step, and the figures S1 to S5, E1 and E2.
- The elapsed seconds of R3's wait, and of R7's, with what the clock read at R1's finish, at `c3`'s start, at `c5b`'s finish and at `c8a`'s start.
- The egress files whole with their sha256, the BK file and the restore (C5, C6), screenshots by name, size and sha256 (never pushed), the Gateway's date and period, the release on Tally's screen.
- The exe's sha256 before and after, the Build line filled, and the client.

## Expected
Estimates, not predictions.
- R2 returns R1's rows with no census read, in 6 fewer requests. R3, R4, R5 and R7 each count the census again, and send what R1 sent. R4's call returns the new voucher. R6's repeat sends fewer requests than its first call and still more than 16.
- Not estimated: the live request counts themselves, which the replays give as 22 and 16. The row records what the receipt says.

## Decides
Each is PARTIAL: one book, one build, one release, one sitting.
- **R2 as expected:** a repeated call reuses the census on a real book and saves one census read (6 requests) on a one-span book. That is the saving, once, as the receipt counts it. It does not say how often real use repeats a call, which this row does not measure.
- **R2's rows differ from R1's, or R2 refuses as `census_reused_mismatch` on an untouched book:** the store is not safe to ship as built. Record whole; the coordinator decides.
- **R3 reuses after 660 s:** the 600 s limit is not enforced. Record the elapsed seconds.
- **R4, R5 or R7 serves a census after its mark moved:** report at once. Record the marks and what the call answered or refused.
- **A mark did not move for the write:** the marks do not see that kind of write, on this release. That is a finding about the guard whatever the call answers; record which mark and which write.
- **R6 as the estimate says:** the empty-window check still counts for itself on a repeat. E2 at 16 or below or E2 equal to E1: record whole.
- **A call errors:** its code and its `request_trail` are the result.

**What these answers are for.** They show whether the change may leave draft on live evidence (AGENTS.md P1). They are never a regression fixture for the build's output.

## Code admits the trigger
Lines are at the head of #2482 as it was when this sheet was written (b1220257). The build is the head on the Build line; if it moved, the lines moved.
- **The key and the marks.** `CensusKey::of` (`src-tauri/src/agent_voucher_window.rs:539`) is the company GUID, the window as sent, the book or one ledger and the census capacity. `FreshMarks::read` (`:565`) is the only constructor of the marks a lookup takes, and `HeldCensuses::find` (`:642`) takes only those and drops an entry whose marks differ.
- **What is held.** `AdmittedCensus::of` (`:593`) is made only after a read that counted its own census stood. `HeldCensuses::hold` (`:654`) takes only that. The byte cap is `HELD_CENSUS_MAX_BYTES` (`:518`, 32 MiB), the lifetime `LISTING_SNAPSHOT_TTL` (600 s, set at `:627`).
- **The refusal.** A part that does not agree with a reused census is relabelled `census_reused_mismatch` (`:2399`, declared in `src-tauri/src/agent_failure_code.rs`).
- **Where it applies.** `Server::read_entry_window_rows_reusing_census` (`src-tauri/src/agent_vouchers.rs:1350`), called at the `vouchers` first page (`:734`) and nowhere else.

## Not in this row
- A book whose voucher mark is above 8,192 (several census spans), and so the saving per span.
- A write through Bridge (`post_import`) and its drop hook.
- A restored copy with the same marks and different vouchers (the case the part check exists for), and R7 when the wait since the hold is between 540 and 660 s.
- A voucher deleted, cancelled or re-dated on Tally's screen; a company feature change; a Gold remote user's save.
- A window of several days, a divided read, a ledger-scoped call, the other tools, and the desktop screens.
- How often a real person repeats a call.
- Any other book, build or release.
