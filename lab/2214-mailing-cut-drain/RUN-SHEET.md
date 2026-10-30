# 2214 mailing cut and drain (#2214): what Tally does after a party master read with mailing details is cut at its limit, and whether the read without them then answers whole, on #2204's build (Part of #2214)

Not for merge. About two to three hours. `SHA256.txt` lists every other file here with its bytes; check it before P0.

**Writes to the book: YES, to THE COMPANY only, undone at the end by restoring BK-2214.**
- 201 party ledgers, "Lab Mail Load 000" to "Lab Mail Load 200", are created by request, each with its dated mailing entries in its one creating save (5 for 000, 200 for each of the others, 40,005 in all).
- Nothing else is changed. No voucher is posted. No ComplyEaze Bridge write tool is called.

## Why
- **The change to come (#2214).** The desktop app's party/ledger master workbook asks Tally for each party's dated mailing details only where it estimates the whole answer fits (#2188). The estimate is an allowance, not a bound, so a book can be admitted and then fail: by the 20 s request limit, or past the 32 MiB response cap. #2214 makes the workbook then send the standard request once, without the mailing details, and write the workbook with a header naming the failure.
- **What stands in the way.** A request cut at its limit does not stop Tally (protocol reference §11b.2). #2204 therefore makes ComplyEaze Bridge record a drain after such a cut. The next post first sends `GET /status` and goes ahead only after three quick answers in a row. Otherwise it is refused as `tally_endpoint_draining` with `retry_after_s`, having sent no post.
  - Two rows measured that drain after one kind of read, an outstandings read cut at 20 s: Tally's work ran on 8.9 s to 25.4 s past the cut (`lab/2142-status-while-busy-answers`, `lab/2142-drain-repeat-answers`).
  - It has not been measured after a party master read with mailing details. Nor has it been measured whether the standard read sent after the drain answers whole, or what an answer past the 32 MiB cap looks like.
- **What this row decides:**
  - **Q1, the ladder.** As the mailing entries grow, what does the mailing master post take in time and bytes? At what size is it first cut at 20 s?
  - **Q2, the drain after that cut** (#2214's open question (a)). How long are calls refused as draining, and when is the standard read admitted?
  - **Q3, the read after it** ((b)). Is the standard read whole: every row equal to the same rung's at-rest standard read, and both marks unchanged?
  - **Q4, past 32 MiB** ((c)).
    - Does Tally declare the length of such an answer (`CONTENT-LENGTH`), and when does its head arrive?
    - Does a standard read sent straight after a dropped oversize answer at its at-rest time?
    - Does ComplyEaze Bridge's own read at that size end as `response_size_limit_exceeded` or as a cut?

**Expected:** none is predicted. Every reading is a result. Only the STOP rules end the row early.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/2142-drain-live` at 9a1ed737, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step, on the owner's standing words, and records each as LAB-COMMON C11 directs, naming the word:
- the raw requests, the creates included: 26 Oct 2026, the lab's curl requests, writes included, for shape discovery;
- master writes on a synthetic book: 28 Oct 2026;
- the backup (C5);
- the restore (C6) at Close, on the run coordinator's standing RESTORE word for this row.

The go-ahead and any STOP ruling come from the run coordinator.

**THE COMPANY** is `BRIDGE BATCH LAB` (synthetic), GUID `a30f23c3-4819-4334-96ab-ffa6fba70976`. It is the book of 2074-L, restored to its BK0: marks `ALTMSTID` (master) 224 and `ALTVCHID` (voucher) 87, and 11 ledgers (2074-L's close read `total` 11 after the restore). If it is absent, or P0's marks or count differ, record the difference and STOP.

**Named exceptions to LAB-COMMON C3:**
1. **The creates.** The runner sends `create-000` to `create-200`, each once, in order, by C3's line.
   - **Before each create** it:
     - sends MARKS;
     - goes on only if the answer holds exactly one COMPANY row, THE COMPANY's, with `ALTVCHID` 87 and `ALTMSTID` exactly the one expected (224 + the number of creates sent before);
     - checks the file's sha256 against `requests-manifest.txt`.
   - **After each create** the answer must read `CREATED` 1 and every other counter 0, with no `LINEERROR`.
   - **On any other reading** the runner halts and sends nothing more.
   - **Screenshots** of the whole Tally window after `create-000` and after `create-001` (each a NEW SHAPE: the first create with entries, the first with 200), after `create-002` to `create-005`, after every 25th create, after the last one, and on any halt. The runner pauses for each and sends nothing until the screenshot shows no dialog.
2. **The curl reads of `mailing-master`** (Part D's D5 and Part C's C1).
   - **Size bound:** 64 MiB, not 2,000,000 bytes.
   - **C1 adds `--max-filesize 33554432` to C3's line,** on purpose, standing for ComplyEaze Bridge's own 32 MiB cap. curl exit 63 there is a result, not a STOP.
   - **Each prints `%{time_starttransfer} %{time_total}`** after C3's `-w` fields.
   - Nothing else of C3 changes: no client timeout, and the 10-minute rule.
3. **No other raw request** is sent but:
   - MARKS;
   - C2 c's and C10's `GET /status`;
   - the creates;
   - the two `mailing-master` reads.

   Every other request is ComplyEaze Bridge's own, through its tools.

**Marks.** A master mark that moves other than by one per create = STOP. A voucher mark that moves at all = STOP. Marks are read:
- before and after each create (the runner);
- at each rung, at C0, after each admitted read of D3 and C2, and after the STANDARD that follows C1 and D5 (never between a cut and its admitted read: see above);
- at Close.

**A refused call is a result, not a STOP,** when this sheet expects it. Every one of these is a `ledger_masters` call:
- code `party_ledger_master_read_failed` whose trail's master post ended `request_deadline_exceeded` (a cut; its `error.cause` reads `request_deadline_exceeded`) or `response_size_limit_exceeded` (past the cap; that call carries no cause);
- code `tally_endpoint_draining`, with `retry_after_s` (10 on this build).

Any other refusal is a STOP.

**No raw request while a drain may be owed.** A raw request bypasses ComplyEaze Bridge's drain gate and would wait behind Tally's work. So from a cut or a dropped oversize (D1, D4, C2), until a STANDARD call has answered (D3, C2), no raw request is sent: no MARKS, and no `GET /status`. The marks of those steps are the ones after the admitted read. The same holds after C1 and D5 until their following STANDARD has answered.

**The drain record** is `<key>.wire.debt` beside the port's wire lock, in the shared profile root, as `lab/2142-drain-live` describes. Copy it whole after each Part D and Part C step, and record its bytes and sha256. That is a file read on the lab PC, not a request.

**Keys.** No screen edit. The Gateway is the only screen used. No key is pressed but:
- the arrows and Enter of Select Company at P0;
- the Backup screen at S0;
- the Restore screen and its two prompts at Close;
- Company > Shut at C10.

**Never Ctrl+Q or Escape at the Gateway of Tally.**

Licensed mode only; Education mode = STOP.

## Build and inputs
- **The build:**
  - It is `bridge_mcp.exe` of pull request #2204 at its head `651cdfa67f15056cd9017b12589e0a010c80637d`, built by CI run 41025801031 dispatched on that head: the `unsigned-bundle-smoke-windows-latest` artifact, id `12963818564`, 58,275,049 bytes, zip digest `sha256:72284e1e536070f3d5c343265c0e20d1f4dfdce4fabd28ab4b7720dc340f0bae`.
  - This is the build of `lab/2142-drain-live`, which the lab already holds. Check its zip's sha256 against that digest; another digest = STOP, and no new download is made without the owner's OK in the lab chat.
  - The exe's sha256 is recorded at P0, from that artifact, and again at Close. A different second sha256 = STOP.
- **The server:**
  - It runs over stdio with:
    - `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9001`;
    - a new empty `BRIDGE_AGENT_DATA_DIR`, `C:\bridge-lab\2214\data`, listed first: absent or empty, else STOP;
    - `BRIDGE_TERMS_ACCEPTED=true` and `BRIDGE_AGENT_MAX_BYTES=5000000`;
    - redaction unset.
  - Writes stay off. Record the environment's key names.
  - `tools/list` must hold `list_companies`, `ledger_masters` and `egress_log`; any one absent = STOP.
- **The reads.** Two `ledger_masters` reads are used, and nothing else reads the book through ComplyEaze Bridge:
  - **STANDARD:** `ledger_masters` with `fields=compliance`. This is the request #2214's retry sends, with no mailing details.
  - **MAILING:** `ledger_masters` with `fields=compliance` and `include_mailing_details=true`. It sends the same rendered mailing request as the workbook, under the same 20 s limit and 32 MiB cap.
  - **After every STANDARD or MAILING call**, `egress_log` for that call. Record:
    - the trail's `sent`, `omitted` and `failed`, and each kept send's kind, outcome, `held_ms`, request bytes and response bytes. The trail keeps only the last 32 sends of a call, so the first ones (a call's drain probes among them) may be in `omitted`: record that, and never read an omitted send as not sent;
    - **the master posts:** the master read is a pair, sent twice (post, status, post, status). They are the two posts with the largest request bytes. Record both. A cut can land on either. An answered M_n takes about twice one post's time, and the 20 s limit counts each post whole, body included.
    - A call refused as draining keeps a `post` record whose outcome is `tally_endpoint_draining`, with no bytes: that post was held back, not sent.
  - For a call that answered, record:
    - its seconds;
    - its `total`, `evidence.bytes` and every page's rows. Read every page by `offset` until a page carries no `next_offset` (or a null one), each page a call of its own, recorded the same way.
- **`requests/marks.utf16le.xml`:** the MARKS request of 2074-L (976 bytes, sha256 539ca283…), with THE COMPANY's name.
- **`requests/mailing-master.utf16le.xml`:**
  - It is the same collection and `FETCH` as ComplyEaze Bridge's mailing master request, for THE COMPANY. Its period, 1 April 2026 to 31 March 2027, stands in for the one Bridge derives from the book's last voucher, so it is not byte-equal to Bridge's own request.
  - It is sent only in D5 and C1.
- **The creates:**
  - `scripts/make_creates.py` writes them. Run `python -I make_creates.py C:\bridge-lab\2214\requests` into a new empty folder.
  - Its printed output must equal `requests-manifest.txt` line for line (name, bytes, sha256); any difference = STOP, and nothing is sent.
  - Each is one Import of one `LEDGER ... ACTION="Create"` under Sundry Debtors, with no PAN or other identifier field. It carries its `LEDMAILINGDETAILS.LIST` entries, in the shape a mailing read returns.
  - Entry j is dated 1 April 2026 plus (j − 1) days, and holds:
    - the address lines `<j> Lab Street` and `Lab Area`;
    - PIN 560001, state Karnataka and country India;
    - the party's name as its mailing name.
- **The runner** (the lab's, as in 2074-L): its sha256 goes into `notes.txt` before K0, and it is committed with the answers under `scripts/`.
- **Folders:**
  - the working folder is `C:\bridge-lab\2214\`, with evidence in `C:\bridge-lab\2214\evidence\`;
  - BK-2214 goes to the new empty folder `C:\bridge-lab\2214\bk\`.

## Before the sitting
- **P0.** `LAB-COMMON.md` C1 and C2: THE COMPANY in Select Company's list and loaded alone, else STOP. EXPECT MARKS 224 / 87. Hash the exe, and every file of this folder against `SHA256.txt`. Generate the creates and compare them with the manifest.
- **S0.** BK-2214 by C5, before any write. Record the marks after it (ms0).
- **B5** before: no ComplyEaze Bridge server or desktop app process that the lab started is left running.
- Start the server. Save `tools/list` whole. Then `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`.

## Part R: at rest
- **R1.** STANDARD, twice, 120 s apart. EXPECT `total` 11 and marks 224 / 87 each time.
  - **The threshold** is the larger of twice the slower call's seconds and the slower plus 2 s.
  - Record the master post's `held_ms` each time.
  - Copy the drain record. EXPECT none, or `{"state":"clear"}`.
- **R2.** MAILING once. Record its master post's `held_ms` and response bytes.

## Part K: the shape check (a NEW SHAPE)
- **K0.** The runner sends `create-000` alone. EXPECT `CREATED` 1, then MARKS 225 / 87. Take the screenshot (no dialog).
- **K1.** MAILING. EXPECT:
  - `total` 12;
  - "Lab Mail Load 000" with five entries: 1 to 5 April 2026, oldest first, each as the file sent it, with `other_content_skipped` false;
  - every other ledger's mailing details equal to R2's, compared as parsed JSON.

  Entries missing, changed or out of order: STOP and report. The import shape does not carry them, and nothing further is sent.

## Part L: the ladder
**The rungs.** The runner sends the creates in order and halts at each rung, after the create that brings the number of 200-entry parties to:

| rung | parties | entries |
|---|---:|---:|
| L1 | 5 | 1,005 |
| L2 | 10 | 2,005 |
| L3 | 20 | 4,005 |
| L4 | 40 | 8,005 |
| L5 | 80 | 16,005 |
| L6 | 160 | 32,005 |
| L7 | 200 | 40,005 |

The entries column counts create-000's five as well.

**At each rung:**
1. MARKS: EXPECT 225 + parties / 87.
2. **S_n:** STANDARD, at rest. EXPECT `total` 12 + parties. Record its master post's `held_ms`, and keep its rows.
3. **M_n:** MAILING, then `egress_log`. Record the master posts' outcome, `held_ms` and response bytes. Copy the drain record.
   - **Answered:** go on to the next rung.
   - **The master post cut at its limit** (outcome `request_deadline_exceeded`, `held_ms` about 20,000): this is THE CUT RUNG. Send no more creates, and go to Part D at once.
   - **Outcome `response_size_limit_exceeded`, before any cut:** record it, and go to Part D at once with it as the cut.
4. Every create answers as the runner requires. Any other reading halts the runner, and that is a STOP.

If L7 is reached with no cut, Part D is not run: record that, and go to Part C.

## Part D: the drain after the cut, and the read after it
- **D1.** The call that cut is M_n. Record its code, cause, next step and seconds. At once, before anything else, copy the drain record. EXPECT `{"state":"owed","since_ms":…,"answered":0,"last_probe_ms":null}`, and record what it holds.
  - **A clear record** (an oversize whose length Tally declared owes no drain on this build) is a result: record it. D2 then expects an answer at once, which is still compared as D3.
- **D2. The retry's read.** Within 5 s of D1's answer, STANDARD.
  - Record:
    - its code, `retry_after_s` if any, and its seconds;
    - its `egress_log` trail: each `status` send's `held_ms` and outcome, and whether a post was sent;
    - the drain record after it.
  - Then, after each refusal's `retry_after_s`, STANDARD again, recorded the same way. At most 20 calls or 3 minutes in all; past that, STOP.
- **D3. The read admitted.** For the first STANDARD call that answers, record:
  - **the gap:** from D1's end to this call's end. The three quick probes that cleared the debt can span calls (the record's `answered` count carries over), so also give, from the refused calls' trails and each drain record copy, when `answered` first read 1, 2 and 3;
  - **its master post's `held_ms`,** against R1's threshold;
  - **whether it is whole:**
    - `total` equal to S_n's;
    - every row equal to S_n's, each whole row compared as parsed JSON (the tool's `snapshot` block left out);
    - its `snapshot` marks equal to S_n's, and then MARKS (raw) equal to S_n's.
  - The drain record after it: EXPECT `{"state":"clear"}`.
- **D4. Repeat once.** Wait 120 s, then M_n again: EXPECT the cut again. Then D1 to D3 again.
- **D5. The whole answer's time.**
  - **Before the read:** once the drain record is clear and a STANDARD call has answered within the threshold, send `mailing-master` by curl (named exception 2). Leave out `--max-filesize`, and set no client timeout.
  - **Record:**
    - curl's exit code, status and bytes;
    - `CONTENT-LENGTH`;
    - `time_starttransfer` and `time_total`.
  - **The D5 body** stays local, listed by size and sha256. It is parsed (C4) only to count its LEDGER rows: EXPECT 12 + the rung's parties.
  - **After it:** at once, STANDARD. Record its seconds and its master posts' `held_ms`, and copy the drain record. This shows whether a whole answer received leaves Tally working. If this call is itself cut, that is a result: record it, then follow D2 and D3 for it.

  `time_total` less 20 s is roughly how long Tally works on past a cut at this rung. Compare that with D3's gap.

## Part C: past 32 MiB
Run only after Part D, or after L7 with no cut. If L7 has not been reached, the runner sends the creates up to L7 first, with the rung checks of Part L. Once Part D has run, skip a rung's M_n: S_n and MARKS only.
- **C0. At rest.** MARKS 425 / 87. STANDARD, which answers with `total` 212: keep its rows as S_7.
- **C1. A dropped oversize, by curl.** Send `mailing-master` with `--max-filesize 33554432` (named exception 2).
  - Record:
    - curl's exit code (63 is the cap);
    - the headers file whole, with `CONTENT-LENGTH` if any;
    - `time_starttransfer`, `time_total` and the bytes received.
  - At once, STANDARD: record its seconds, its master posts' `held_ms`, its trail, and whether it is whole against S_7 (as D3). Copy the drain record. Since curl sent this read, not ComplyEaze Bridge, EXPECT no debt before this call. If this call is itself cut, that is a result: record it, then follow D2 and D3 for it.
- **C2. ComplyEaze Bridge's own read at that size.** Wait 120 s, then MAILING.
  - Record its code, cause and seconds, and its master post's outcome, `held_ms` and response bytes. Copy the drain record.
  - Then STANDARD, as D2 and D3: refused while the drain is owed, then answered and compared with S_7.

## Close
1. WRITES OFF: no more requests but reads. Copy the drain record last time. If it is not clear (after a STOP, say), make no raw request: first wait until its `since_ms` is 10 minutes old, the time after which the build clears it by itself. Never delete or edit it by hand. Then `GET /status` must answer within 5 s before anything else is sent or keyed.
2. Stop the server (its stop file), and record its exit code and error stream. Hash the exe. Re-hash the inputs.
3. **Restore BK-2214 by C6,** on the run coordinator's standing RESTORE word.
4. MARKS: EXPECT ms0 (224 / 87). Then, with the server started again, STANDARD: EXPECT `total` 11 and no ledger named "Lab Mail Load …". Stop the server.
5. B5 at the end.
6. C10: MARKS before the shut, then Shut Company, then `GET /status` only.

STOP RULES:
- C7 and C8 of LAB-COMMON;
- any dialog;
- a second `tally.exe`;
- a mark that moves other than as this sheet says;
- a runner halt;
- K1's check;
- D2's bound (a STOP, then reported as the reading in the table below);
- a refusal this sheet does not expect;
- Ctrl+Q or Escape at the Gateway (an incident: press no key, take a screenshot, STOP);
- an exe sha256 that is not the one named;
- a request without an answer in 10 minutes (C3: leave it running, and send it never again).

## What the row records
- **The ladder table:** each rung's parties, entries, MARKS, S_n's master post `held_ms`, and M_n's master posts' outcome, `held_ms` and response bytes.
- **The drain table (D1 to D4, both runs):** each call's code, seconds, `retry_after_s`, the trail's `sent`, `omitted` and kept sends by kind with `held_ms` and outcome, and the drain record after it (`state`, `since_ms`, `answered`, `last_probe_ms`).
- **The gap (D3, twice):** D5's `time_total`, and D5's following STANDARD.
- **Part C:**
  - C1's exit code, headers, times and bytes, and the STANDARD after it;
  - C2's outcome and drain sequence.
- **Reading it (for the requester):**

| reading | meaning |
|---|---|
| D3 answers within 3 minutes, whole, with marks unchanged, in both runs | After a cut mailing read, the standard read can follow the drain: #2214's retry waits out the drain, with its bound set from the longer gap. |
| D3's gap past 3 minutes, or D2's bound reached | A drain too long to wait out within one export: #2214's retry is not built as designed. Report the gap. |
| D3 answers but not whole, or a mark moved | The read after a cut cannot be trusted: STOP the design and report. |
| C1: `CONTENT-LENGTH` declared and over the cap | An oversize is known at its head. #2204 owes no drain then, since Tally had finished. C1's STANDARD then shows whether Tally is still busy sending. |
| C1: no `CONTENT-LENGTH`, cut in the body | An oversize is known only part-way, and #2204 owes a drain. |
| C2: `response_size_limit_exceeded` | ComplyEaze Bridge reaches the cap within its limit at this size. #2214's `ResponseTooLarge` path is live. |
| C2: cut at its limit | At this size the time limit comes first. The cap is reached only by a read that is quicker per byte. |
| Anything else | Record it whole and infer nothing. |

**Confidence:** one sitting on one synthetic book and one release, so PARTIAL.

**Not covered:**
- a book whose mailing entries are spread thinly over many parties;
- an answer with longer addresses;
- a second process sending during the drain;
- a connection dropped after sending began;
- other releases, Gold.

## Evidence (stays on the lab PC except the files listed)
- **RESULTS.md:** every step, stamp, mark, call, trail, drain record, byte count and duration, and the tables above.
- **The create answers, aggregated:** counted, with the first, the last and any differing answer kept whole, each with its size and sha256. One line per create in `evidence/creates-log.tsv`.
- **The `ledger_masters` answers:**
  - R1, R2, K1, every S_n, D3's admitted reads and C0, whole;
  - M_n answers and refusals, whole while each is at most 2,000,000 bytes, otherwise listed by size and sha256.
- **The curl answers of D5 and C1:** their headers files whole; the bodies stay local, listed by size and sha256.
- **The drain record copies.**
- **`notes.txt`,** the runner under `scripts/`, and `SCREENSHOTS.txt`.
- **`SHA256.txt`** over the committed files, with sizes.

Evidence goes on a new branch, `lab/2214-mailing-cut-drain-answers`. Send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
