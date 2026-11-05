# Row 852-C2: two 50-voucher bank batches posted behind the capture relay while someone works on Tally's screen in the same company

Not for merge. Part of #852. Written 5 Nov 2026. Lines below are at 187e882b, the build's head, and this branch starts there. Confidence of everything here until the row runs: unverified.

## Question
Row 852-C1 (3 Nov, `lab/852-capture-answers` at 8f37fdb8) posted two 50-voucher batches on this book with nobody at Tally. Both came back `posted_verified` 50, the import exchanges took 6.59 s and 6.55 s, the calls 49.4 s and 49.7 s, and the relay saw no lost reply. The failing post of #852 was on a book in daily use. This row changes **one variable**: during the post, the operator is working on Tally's screen in THE COMPANY. Does the reply still come back whole and in time, and if not, what cuts it?

Two kinds of screen work, one post each:
- **C2a, a report.** The Day Book open and scrolled through the whole post.
- **C2b, a voucher half entered.** A Journal open on the voucher-entry screen with one line typed and not saved, left as it is through the whole post.

The approval box itself tells the person to "Pause other edits/imports; keep this company and Tally mode as is until ComplyEaze Bridge finishes" (agent_import_post.rs:4101). This row breaks that instruction on purpose, because a busy office does.

**The best result here reads "not reproduced with screen work at this scale". It does not close #852:** Silver, one user, two kinds of work, two posts.

## Book
`BRIDGE LARGE LAB`, GUID begins `2ba6df28` (whole on the lab PC and in 852-C1's and 1029-R's answers), books from 1-Apr-2026, TallyPrime 7.2 Silver, licensed. Synthetic: 29,930 Journals, 82 a day, ledgers `Lab Vol Expense`, `Lab Vol Income`, `Lab Vol Cash` and `Lab Vol Bank` (under Bank Accounts). Its state, from row 1029-R's RESULTS (`lab/1029-r-answers`): **V 30,168, M 211**, holding 852-C1's 100 vouchers (11 to 30 Oct 2026) and 1029-R's 138 (1 to 7 Oct 2026), not restored, left shut with one tally.exe on 9001.

The batches here lie in 1 to 20 Sep 2026: no Bridge batch has used those days, and they hold only the book's own Journals, whose ledger pair (`Lab Vol Expense` and `Lab Vol Income`) no batch voucher has.

Not on the PC, or another GUID: stop, the row has no book; never rebuild it. Any prompt when it opens: press no key, screenshot, stop and report its words. Education mode: stop.

## Rules
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one on `lab/1029-p-presence` at 2a645696, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. THE COMPANY is BRIDGE LARGE LAB.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Writes.** Two posts through Bridge's `post_import` (steps 4 and 5), BK-C2 (step 1) and its restore (step 7). The one line typed on the voucher-entry screen in step 5 is never saved. Nothing else is written. No raw import is sent.
2. **Who acts on the box.** The lab answers Bridge's approval box itself, Yes, by mouse on the box's own button, as in rows 852-C1 and 1029-R. Before the click, the box-check helper of 852-C1 reads the box's text and allows Yes only if it names this company, its GUID, this batch's id and 50 vouchers. A box that names anything else, or that cannot be read: Decline, record its text, and stop.
3. **The screen work is the variable.** It is the only keying in Tally from step 2 to step 6, and it is done exactly as steps 4 and 5 say: those keys, on those screens, at those times.
4. **The gateway stop (P7).** If Tally stops answering while a screen of this row is open (a call with no answer in 10 minutes, or the GET /status of a step with no answer), and no dialog is up: stop. Send nothing more. Leave the screen the operator opened, by the keys step 4 or 5 names for it, back to the Gateway. Then restore BK-C2 (step 7) and run C10. A dialog that this sheet does not name is C7 first: screenshot, copy its text, press no key, report; the run coordinator decides what follows.

## Build
BUILD-M, the one already staged for rows 2161-C, 1029-R and 1392-E:
- the `unsigned-bundle-smoke-windows-latest` artifact of CI run 41772223948, the merge-queue run of #2329 at 187e882b5176a3b577d57f257a6ec4319fc2aecd;
- artifact id 13295846440, 59,336,955 bytes, GitHub digest `sha256:1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a`, expiring 11 Nov 2026.

C1 ran build 838242cb. This row's one variable is the screen work, so the build change is named, not hidden: `agent_import_post.rs` differs between 838242cb and 187e882b (31 lines in, 8 out), and 1029-R's five posts on 187e882b on this book took at most 43.3 s a call with no refused read-back. Those five are this build's baseline on this book with nobody at Tally. Between the two builds no transport, relay, wire-lock or endpoint file changed, and `runtime.rs` did not change. `agent_import_post.rs` changed in only two commits: 96b30ca8 (#2317, the release dialog's listing) and ffc94950 (#2318, typing the import amount). `agent_import_invoice_wire.rs` changed too, but it only builds the invoice XML. So the path that reads the reply is C1's, and the screen work is still the one variable.

Check the zip's sha256 against the digest, and `bridge_mcp.exe` against `mcpb-smoke.json`, before the first call. Either differing is a stop. Record the exe's sha256 before and after.

Over stdio, through the relay: `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9002`, one new empty `BRIDGE_AGENT_DATA_DIR` for the whole row, `BRIDGE_AGENT_ENABLE_BATCH_POST` absent, `BRIDGE_AGENT_REDACTION` unset. One `bridge_mcp` process for the whole row. The client is the lab's stdio client of 852-C1 and 1029-R: no cancel and no call limit under 10 minutes. A call over 60 s is recorded beside the result, not a stop.

## Kit
| file | what |
|---|---|
| `relay.py` | byte copy of 852-C1's (`lab/852-capture-sheet` at 2c32c3e2, sha256 80ffdb20) |
| `check-relay.py` | byte copy of 852-C1's (sha256 9a806848); `python -I check-relay.py relay.py`: 0 failed here on macOS before this sheet was sent |
| `make-batches.py` | 852-C1's, with only the month (September), the tags (`e`, `f`) and the id prefix changed; `python -I make-batches.py OUT` gives the same bytes |
| `batches/c50e.json`, `c50f.json` | 24 Receipts, 20 Payments and 6 Contras each, numberless, the mix of C1: e on 1 to 10 Sep 2026, f on 11 to 20 Sep |
| `marks.utf16le.xml` | byte copy of 1029-R's (`lab/1029-r-sheet` at aa811b9b): the company's GUID, ALTVCHID and ALTMSTID. Send it with `Content-Type: text/xml; charset=utf-16` on 9001. |
| `LAB-COMMON.md` | v3, above |

`SHA256.txt` lists every other file here with its sha256. Check it first, and check each batch's sha256 again when it is used.

## Steps
One at a time. Every answer goes to a file. A file with no answer is a failed step, not an empty one.

0. **Load and check.** Check `SHA256.txt`. `python -I check-relay.py relay.py`: EXPECT 0 failed, else stop. LAB-COMMON step 0 with THE COMPANY loaded alone. **m0**: marks, EXPECT one COMPANY row, this GUID, **V 30,168 and M 211**. Other marks: recorded and reported before step 1, not a stop. Record Tally's release, Tally's date and the PC's date; the PC's date must be on or after 20 Sep 2026.
1. **BK-C2** (C5), into the new empty folder `C:\bridge-lab\852c2\backups\bkc2\`. Record its name, bytes, sha256 and the time the Restore list will show. Then marks **m0b**.
2. **Relay and Bridge.** Start the relay: `python -I relay.py 9002 9001 C:\bridge-lab\852c2\e`. Start `bridge_mcp` as above.
   - `list_companies`: EXPECT this GUID. `tally_status`: record release, licence and mode.
   - `masters` with `kind=voucher_types`: record the Payment, Receipt and Contra rows' `series_numbering.method` and `sub_method`. EXPECT automatic and auto_retain for all three, as C1 left them and 1029-R read them. Any other: stop. This row alters no type.
   - Marks on 9001: EXPECT as m0b.
3. **Build both.** `build_import_xml` with `{"company_guid": <GUID>, "vouchers": <c50e.json>}`, then with `c50f.json`. Keep each whole answer; record each batch id, `voucher_count` (EXPECT 50) and every warning. A refusal: keep it and stop.
4. **C2a, the Day Book.**
   a. At the Gateway: Alt+G, type `Day Book`, Enter (or Display More Reports, then Day Book). Screenshot: the Day Book of THE COMPANY on Tally's date, with rows.
   b. `post_import` with `company_guid` and batch e's id. As soon as the box shows and the helper passes it, Yes. **At once** click Tally's title band, then press Down once a second, without stopping, until the post call returns; at the end of the list press Up once a second instead. Record the time of the first and last key press, UTC with milliseconds, and each call's start and end, when the box showed and when Yes was clicked. `approval.state` `pending`: call again after `retry_after_s`, the Day Book left open and the keys paused. `approved` without a post: call again at once. Never call again after an attempt is recorded.
   c. When the call has returned: screenshot. Esc once, back to the Gateway (Esc on a report is allowed: C7 is about master and voucher-type screens). Screenshot: the Gateway, no dialog.
   d. If the post's result names `retry_after_s` before `verify_import`, wait it out. `verify_import` with batch e's id: the whole answer. `egress_log`: the whole answer. Marks on 9001: EXPECT V +50 and M unchanged. Otherwise: report before step 5.
   e. Stop the relay, and keep its folder `e`. Start it again with folder `C:\bridge-lab\852c2\f`.
5. **C2b, a Journal half entered.**
   a. At the Gateway: Vouchers, as the lab keys a voucher; F7 for Journal. Leave the date as Tally offers it. On the first line: `By`, `Lab Vol Expense`, Enter, amount `1.00`, Enter. Stop there, with the cursor on the next line's ledger field. Nothing is saved. Screenshot: the voucher-entry screen, one line typed.
   b. `post_import` with batch f's id, recorded as in 4b. Yes on the box as in 4b, then **touch nothing in Tally** until the call returns: the half-entered voucher stays on the screen as it is.
   c. When the call has returned: screenshot. Esc, and the one prompt this sheet names here, the one that asks whether to quit the voucher, answered Yes (the line is discarded). Screenshot the prompt before answering it. Any other prompt: C7. Screenshot: the Gateway, no dialog.
   d. As 4d for batch f. EXPECT V +50 (the discarded line adds nothing) and M unchanged.
   e. Stop the relay and keep its folder `f`.
6. **End of writes.** Marks **m3**: EXPECT **V 30,268**, M 211. Stop `bridge_mcp`.
7. **Restore BK-C2** (C6). Then marks **m4**. EXPECT the COMPANY row's GUID, V and M equal to m0b's. Anything else = stop.
8. LAB-COMMON C10: shut the company, one tally.exe on 9001, no company loaded.

## What to report
For each post, from its relay folder's `relay.jsonl`, one column:

| | C2a (e) | C2b (f) |
|---|---|---|
| requests before the import, their bytes and their summed `answer_s` | | |
| the import line's `sent`, `first_byte_s`, `answer_s`, `bridge_left_s` | | |
| Tally's counters, from `import-N.answer` | | |
| any `reply_lost`, `tally_closed`, `relay_error` | | |
| requests after the import, bytes, seconds | | |
| each call's start, end, seconds; box shown at; Yes at | | |
| the screen work: first and last key, and whether it overlapped the import line's `sent` to `answer_s` | | |
| result: `created`, `attempt_recorded`, `dispatch` (`state`, `response_state`, `resent`), `error` code and `next_step` | | |
| `response_prepared` line in `egress_log` | | |
| `verify_import`: status counts | | |
| Tally's screen before, during (the screenshot of 4a or 5a) and after | | |
| marks after | | |

Beside each, C1's figures (6.59 s and 6.55 s exchanges; 49.4 s and 49.7 s calls). Read every number from the files. If an import reads over 20 s or a call over 60 s, say so in the first line.

## What the result decides
| what the row shows | reading on #852 |
|---|---|
| both `created` 50, reply kept, `posted_verified` 50 of 50, the screen work overlapping the import, the import under 20 s | **not reproduced with these two kinds of screen work.** The issue stays open with these times beside C1's; what is left is a Gold book and other work (another user, a save during the import). |
| as above, but an import markedly slower than C1's | the screen work slows Tally's import: recorded with both times; the deadline question below is how near 20 s it came |
| an import over 20 s, `bridge_left_s` set near 20 s, `reply_lost`, Tally's answer whole in `import-N.answer`, `import_dispatch_outcome_unknown` | **#852's ending, reproduced:** Bridge's own deadline (20 s, transport lib.rs:95) cut a reply Tally was still writing while its screen was busy. The next step is C1's: a deadline sized to the voucher count, with a ceiling. `verify_import` settles the batch. |
| `import_dispatch_outcome_unknown` with `bridge_left_s` null | the reply reached the relay and Bridge dropped it: keep everything, the code is next |
| `post_queue_read_failed`, nothing sent | Tally did not answer a read before the import while the screen was open: recorded with its cause; nothing was posted, so no import ran |
| `tally_closed`, `relay_error`, or no answer in 10 minutes | Tally did not answer: the gateway stop (Rules, 4) |
| a dialog on Tally's screen | C7; reported whole before anything else |

`verify_import` settles a batch in every case. Nothing is sent again.

## Stop rules
- Any dialog on Tally's screen that this sheet does not name, before, during or after a call (C7).
- The gateway stop (Rules, 4).
- Any refusal before an attempt is recorded, other than `post_queue_read_failed` (recorded, then stop).
- Any call with no answer in 10 minutes.
- A box the helper does not pass.
- A type in step 2 that is not automatic with retain.
- V not as expected after a post: report before the next step.
- A mark that moves outside the steps that change it.
- Education mode.

After a stop, nothing more is sent or keyed (C8), except the gateway stop's own way back. The book then holds what was posted before it: batch e's 50 vouchers from step 4, batch f's from step 5. The way back is BK-C2.

## Limits
- Silver, one user, a synthetic book of about 30,000 vouchers. The failing book was a client's Gold book read by about 100 requests.
- Two kinds of screen work, one post each. Saving a voucher, a second user, an import alongside, and work in another company are not tried.
- The screen work starts when Yes is clicked. The reads before the box are not done under it.
- The relay reads each answer whole before forwarding it (C1's Limits), and this client does not cancel, so a reply lost to a host cancel cannot appear here.
- The build is 187e882b, not C1's 838242cb (Build).

## Code admits the trigger (187e882b, the build's head)
- **The send and its deadline.** The one import send is `post_probe_xml` (tally/runtime.rs:3905), inside `post_approved_import` (:3499). Every request has `DEFAULT_REQUEST_TIMEOUT` 20 s for the whole exchange, body included (bridge-tally-transport lib.rs:95; ceiling 120 s, :96).
- **A reply cut mid-read.** The transport sorts the failure (lib.rs:1352-1369): a timeout is `RequestTimedOut`; a body cut after its header is `ResponseTruncated`; a connection closed before any byte is `RequestFailed`. Each but a refused connection leaves a drain owed (lib.rs:782). The post then returns `import_dispatch_outcome_unknown` (agent_import_post.rs:2100) with `attempt_recorded` true and nothing resent, through `reconciliation_failure_payload` (:2184), whose step is to call `verify_import` with the same batch. `verify_import` then reports `response_missing` (:2325).
- **Blocked before the send.** A read refused in the queue before the intent is `post_queue_read_failed` (agent_import_post.rs:2098): nothing was sent. The next post after a cut is refused `tally_endpoint_draining` (:2242, step at :2150) until the drain clears.
- **No dialog is detected.** No code reads Tally's screen, so a blocked gateway shows only as a request with no answer; hence the operator's screenshot and the gateway stop.
- **The box's words** (agent_import_post.rs:4101) and the post path of 1029-R (Code admits there): the cap of 50, numberless vouchers needing automatic with retain, the types, `validate_dates`.

## Who sends each EXPECT's request (187e882b)
| step | request | sent by | the count |
|---|---|---|---|
| 0, 1, 2, 4d, 5d, 6, 7 | marks, raw | the operator's curl on 9001, not through the relay | one COMPANY row; V is its ALTVCHID, so +50 is 50 vouchers created; M its ALTMSTID |
| 2 | company list, status, voucher types | `list_companies`, `tally_status`, `masters` (agent.rs), through the relay | the rows named |
| 3 | the build's reads | `build_import_xml` (agent.rs:3368), through the relay | `voucher_count` 50 |
| 4b, 5b | reads, the one import, the read-back | `post_import` (agent.rs:3406), through the relay, which keeps the import's request and answer | Tally's `CREATED`; `posted_verified` |
| 4d, 5d | the verification reads | `verify_import` (agent.rs:3432), through the relay | `posted_verified` n of 50 |
| 4d, 5d | none to Tally | `egress_log` | the log's lines, read from the data folder |

## Keep
On `lab/852-c2-answers`, with `SHA256.txt`:
- both relay folders whole (`relay.jsonl`, every `import-N.request` and `.answer`);
- every tool answer whole, with its size and sha256;
- each post's times, the key-press times and the box log;
- the marks answers;
- BK-C2's name, size and sha256, and `restore-plan.txt`;
- a RESULTS.md with the What to report table filled in and the first line as that section says;
- a notes file with UTC times, one line per step.

Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.
