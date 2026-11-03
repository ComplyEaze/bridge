# 2074-2214 export and cut: the master workbook's live export on a book past the mark, and a mailing read cut at the 20 s limit (Part of #2074, Part of #2214)

Not for merge. About two hours, one sitting, in this order, because each part needs the book the part before it left. File lines below are at master d214d6a0: recheck them at the row. `SHA256.txt` lists every other file here with its bytes; check it before P0.

**Needs before it starts:** the desktop app installed from the row's artifact and driven by the lab. 2074-L's install landed in a virtualized folder and its screen grant was declined, so that row's Part W never ran. Without the app, stop and tell the run coordinator: this row does not go round it.

**X-only cut (the run coordinator's ruling, 3 Nov 2026).** The W rows wait for the owner, so this sitting runs by tool alone, with no app installed, after rows N2 and #2210, and only if N2 shows Tally serves a front first on a warmed connection; if it does not, the front is rewritten before this runs. Run P0, S0, B5 and R1; A1 for `create-000` only and A2 expecting `total` 12, with no `create-probe` and no A3; M1 and M2 with `total` 172, no W2; Part F without `front-w4.xml`; X1 to X3, with Whole expecting `total` 174; then Close. The only build is `bridge_mcp.exe`, taken as those two rows take theirs. A hang is still a C9 STOP, with nothing sent, but in this cut the lab may restart Tally on its standing authority and reports it. Read the Decides table without W4: X rows that pass show a cut by tool, and any ruling on `Deadline` also waits for W4.

**Writes to the book: YES, to THE COMPANY only, undone at the end by restoring BK-2074-2214.**
- One party ledger, `Lab Mark Probe`, created by request, then altered back and forth to raise the master mark past 4,266, as 2074-L.
- Party ledgers `Lab Mail Load 000` to `Lab Mail Load 200`, as the 2214 row.
- Ledgers `Lab Bulk Expense` and `Lab Bulk Income`, and Journals between them, posted by raw request as other client's imports.
- No ComplyEaze Bridge write tool is called.

## Why
- **#2074.** The workbook counts the ledgers of a book whose mark is past the standard bound, and asks for the mailing details where the count fits (#2188). Row 2074-L measured the bytes through `ledger_masters`, but its Part W, the only part that runs the decision, was skipped. `resolved_for_count` runs only for the workbook (`tally/connection.rs:729`, `:2276`; the tool's reads size by the count alone).
- **#2214.** After a mailing read fails, the workbook sends the standard read once. A declared oversize waits for the drain first (`ContinueOnceDrained`, `tally/connection.rs:2393`). A cut at the 20 s limit fails the export and sends nothing (`MailingReadDecision::Refuse`, `:2391`), because no such cut has been seen live: row 2214 reached the 32 MiB cap first. Neither path has run in the workbook itself.
- **Why the cut needs help.** On row 2214's book an entry costs about 940 bytes and 0.13 ms, so a read reaches the cap long before 20 s. This row cuts by queueing instead: another client's import reaches Tally just before the mailing read, so the read waits behind it until Bridge's limit ends the post, and Tally then has the read to run with nobody listening. Tally serves its gateway one request at a time (`lab/1713-settle-order-answers`, rows K and N1).

**What this row decides:**
- **W1 (#2074's L1).** Past the mark, with few ledgers, does the live workbook show the mailing details?
- **W2.** Does the workbook take a whole mailing read of about 30 MB?
- **W3 (#2214's acceptance line).** Past the cap, does the export still write a workbook, say why the details are missing, and wait for the drain before the standard read?
- **X rows and W4 (the 31 Oct narrowing of #2214).** After a real cut of a mailing read: how long are calls refused as draining, and is the standard read admitted after it whole? Does the live export fail with its sentence and send nothing further?

**Expected:** only where a step says EXPECT. Every other reading is a result. Only the STOP rules end the row early.

## Rules and company
`LAB-COMMON.md` in this folder applies in full (v3, byte copy of the 2214 row's). Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step on the owner's standing words, and records each as C11 directs, naming the word: raw requests, writes included (26 Oct 2026); master writes on a synthetic book (28 Oct 2026); the backup (C5); the restore (C6) on the run coordinator's word. The go-ahead and any STOP ruling come from the run coordinator.

**THE COMPANY** is `BRIDGE BATCH LAB` (synthetic), GUID `a30f23c3-4819-4334-96ab-ffa6fba70976`, at rest marks `ALTMSTID` 224 and `ALTVCHID` 87, 11 ledgers. Anything else at P0: record it and ask.

**Named exceptions to LAB-COMMON C3.** Nothing raw is sent but these, MARKS, and C2 c's and C10's `GET /status`:
1. `create-probe`, then `alter-a` and `alter-b` by the 2074-L runner (`scripts/a5.py` of `lab/2074-mailing-by-count-answers`), exactly as 2074-L ran them.
2. `create-000` to `create-200` by the 2214 runner, each once, in order, with the screenshots that row lists. Each create expects the master mark to be the last MARKS plus one, and the voucher mark unmoved.
3. `front-ledgers.xml` and `front-probe.xml` by C3's line, at F0.
4. `front-*.xml` sent by `relay.py` when armed, only at X and W4 rows.

**Marks.** A master mark that moves other than as a step says = STOP. A voucher mark that moves = STOP, except by exactly the Journals of a front that answered. That is this row's one change from the 2214 row's rule.

**A refused call is a result, not a STOP,** when a step expects it: `party_ledger_master_read_failed` with cause `request_deadline_exceeded` or a post ended `response_size_limit_exceeded`; `tally_endpoint_draining` with `retry_after_s`. Any other refusal is a STOP.

**No raw request while a drain may be owed.** From a cut until a STANDARD call has answered, no MARKS and no `GET /status`: they would bypass the drain gate and wait behind Tally's work. **The drain record** is `<key>.wire.debt` beside the port's wire lock in the shared profile root, as `lab/2142-drain-live` describes; this row's key is port 9002's. Copy it whole after each step that says so.

**If Tally stops answering.** A drain still refused after 20 calls or 3 minutes, or any sign of a hang, is C9: STOP and report. No probing, no restart until the run coordinator says.

**Keys.** The Gateway is the only Tally screen used. As the 2214 row: Select Company at P0, Backup at S0, Restore and Company > Shut at Close. Never Ctrl+Q or Escape at the Gateway. Licensed mode only; Education mode = STOP.

## Build and inputs
- **The build:** master at the row, its green merge-queue run, artifact `unsigned-bundle-smoke-windows-latest`, holding the installer and `bridge_mcp.exe`. Record the head, the run, the zip's and exe's sha256 at P0, and the exe's again at Close. Downloading and installing need the owner's OK in the lab chat, as 2074-L.
- **The relay.** Everything of Bridge's goes through it, so the log shows what was sent:
  - `python -I relay.py 9002 9001 EVIDENCE\relay ARM`, with `ARM` an existing empty folder outside the evidence. Checked on a stand-in only, not on Tally. Started once, before the first read, and left up to Close.
  - It logs each request and answer, keeps each distinct body once in `bodies/`, and closes Tally's side as soon as Bridge closes. Its own new lines are `front_*` and `bridge_cut`.
  - **To arm:** `Set-Content -Encoding ascii -NoNewline ARM\front <absolute path of the front file>`. The next request naming the mailing details consumes it. Unarmed, it sends nothing of its own.
  - `relay_error` with a broken pipe on the post of W3 is expected: Bridge closes on the declared length. Any other `relay_error`, or `tally_closed`, = NOTE: send it, do not explain it.
  - A `relay.jsonl` with no `open` line means the row failed, not that nothing happened.
- **The server:** `bridge_mcp.exe` over stdio, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9002`, a new empty `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_AGENT_MAX_BYTES=5000000`, writes off. One process, idle during the app's steps.
- **The app:** installed from the artifact, its Tally port set to 9002. It writes the workbook to Downloads, with no dialog (`commands.rs:2085`). Button: the Outstandings screen's "Ledger master + group subtotals". Record any request of its own in the relay log.
- **The reads.** STANDARD: `ledger_masters` with `fields=compliance`. MAILING: the same with `include_mailing_details=true`. After every one, `egress_log`, recorded as the 2214 row lists: the trail's counts, each kept send's kind, outcome, `held_ms` and bytes. The master read is a pair of posts, and the 20 s limit counts each post whole.
- **Telling the reads apart in `bodies/`.** Decode the request. A mailing master post's FETCH holds `PARTYGSTIN` and `LEDMAILINGDETAILS.LIST`; a standard one holds `PARTYGSTIN` only. Parse the FETCH: do not count substrings.
- **Files here:** `requests/marks`, `create-probe`, `alter-a`, `alter-b` (byte copies from the two kits), `scripts/make_creates.py` with `requests-manifest.txt`, `scripts/make_front.py`, `relay.py`.
- **The fronts:** Journals dated 2 April 2026 between the two bulk ledgers, distinct REMOTEIDs by index, one per file. `python -I make_front.py <folder> <file> <first index> <journals>`. Record what it prints, and check `front_send`'s `sent_sha256` against it.
- **Folders:** working folder `C:\bridge-lab\2074-2214\`, evidence in `evidence\`, BK-2074-2214 in the new empty `bk\`.

## Before the sitting
- **P0.** C1 and C2: THE COMPANY loaded alone. EXPECT MARKS 224 / 87. Hash every file here against `SHA256.txt`. Generate the creates: the printed output must equal `requests-manifest.txt`, else STOP.
- **S0.** BK-2074-2214 by C5. Record the marks after it.
- **B5.** No ComplyEaze Bridge process left running from before.
- Start the relay, then the server. Save `tools/list`. `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`.
- **R1.** STANDARD twice, 120 s apart. EXPECT `total` 11, marks 224 / 87. The post threshold is the larger of twice the slower master post's `held_ms` and that plus 2,000 ms. Copy the drain record: EXPECT none or `{"state":"clear"}`.

## Part A: the mark
- **A1.** `create-probe`: EXPECT `CREATED` 1, master mark up by one. `create-000`: EXPECT `CREATED` 1, up by one, with the 2214 row's screenshot.
- **A2.** MAILING. EXPECT `total` 13, and `Lab Mail Load 000` with five entries, 1 to 5 April 2026, oldest first. Entries missing or changed: STOP.
- **A3.** The alteration runner to a master mark of at least 4,400: about 4,180 alterations, about 21 minutes on 2074-L's rate. Record the count, the time and the last mark `m`.

## Part W1: the workbook past the mark
Stop nothing: the server stays idle. In the app, export the workbook for THE COMPANY with the host's date.
- **EXPECT** the master post of this export to be a mailing one, after a count read (the relay's log, parsed). Cell C4 empty. Cell C3 holds no failed-read note.
- **EXPECT** `Lab Mail Load 000`'s five Mailing cells: its 5 April 2026 entry, `5 Lab Street, Lab Area`, PIN 560001, Karnataka, status "in force".
- **EXPECT** `Lab Mark Probe`'s Mailing status "no mailing details", never "not observed".
- Record the time, the workbook's sha256, and the "Balances as of" date.
- If the app sends the standard request instead, that is the result: W1 fails, record cell C4's note, and go on to Part M.

## Part M: 30 MB of mailing details
- **M1.** The 2214 runner: `create-001` to `create-160`. MARKS after: EXPECT `m` + 160.
- **M2.** STANDARD: EXPECT `total` 173. Then MAILING: EXPECT about 30 MB and 32,005 entries in all (row 2214's L6 read 30,122,530 bytes, about 4 s each post). Record both posts. The bytes differ from L6 by the probe's row; anything larger is a NOTE.
- **W2.** The app's export again. EXPECT a mailing post answered whole, C4 empty, no failed-read note, 173 rows, and `Lab Mail Load 160`'s Mailing cells: its latest entry on or before the balance date, status "in force". Record the time and the sha256.

## Part F: the front's size
- **F0.** Generate `front-ledgers.xml` and send it: EXPECT `CREATED` 2, master mark up by two. Then `make_front.py <folder> front-probe.xml 0 500`, and send it by C3's line, timed: EXPECT `CREATED` 500, every other counter 0, no `LINEERROR`, voucher mark up by exactly 500. A screenshot of the whole window: any dialog = STOP. This is the row's NEW SHAPE, and the book's first Journal between these ledgers.
- **F1.** With `t` the seconds F0 took, `n50` = 500 × 50 / `t` and `n17` = 500 × 17 / `t`, rounded down. If `n50` passes 3,000, use 3,000 and record it.
- **F2.** Generate, each from its own first index so no REMOTEID repeats: `front-x1.xml` and `front-x2.xml` and `front-w4.xml` of `n50` Journals, `front-x3.xml` of `n17`.

## Part X: the cut
`Xn` is one go of these steps, with its own front file:
1. Drain record clear. MARKS. STANDARD: keep nothing but its seconds.
2. Arm the relay with the file. MAILING.
3. **EXPECT** about 20 s, then `party_ledger_master_read_failed`, its master post `request_deadline_exceeded`. In `relay.jsonl`, in order: `front_written`, `forwarded` with this front's number, `bridge_cut`.
4. **D1.** At once, copy the drain record. EXPECT `{"state":"owed",…}`. A clear record is a result.
5. **D2.** Within 5 s, STANDARD. Record its code, `retry_after_s`, seconds, its trail's status sends and whether a post was sent, and the drain record after it. After each refusal's `retry_after_s`, STANDARD again. At most 20 calls or 3 minutes: past that, C9.
6. **D3.** For the first STANDARD that answers, record:
   - **the gap:** from D1 to this call's end, and when `answered` first read 1, 2 and 3;
   - its master posts' `held_ms` against R1's post threshold;
   - when `front_answered` came, against the cut and against the first quick status.
7. **Whole.** At once STANDARD again. Each whole row of the admitted read must equal this one's, compared as parsed JSON (the `snapshot` block left out), and `total` 175. Then MARKS: voucher mark up by exactly this front's Journals, master mark unmoved, and the drain record `{"state":"clear"}`. The two bulk ledgers' balances have moved, so compare with this read, not an earlier one.
8. `front-<k>.answer`: EXPECT `CREATED` equal to the file's Journals, every other counter 0.

**VOID:** if the mailing read is answered, not cut, or if `first_byte` for that post comes before `bridge_cut`, Tally served the read ahead of the front. Record it, keep the marks, and go on: a voided go is a finding about order, and it says nothing about a cut.

- **X1.** `front-x1.xml`. The cut lands while Tally is on the front, so the read runs after the cut.
- **X2.** Wait 120 s, then again with `front-x2.xml`.
- **X3.** `front-x3.xml`: the front ends just before the limit, so the read is under way at the cut. Record the time from `front_answered` to `bridge_cut`. If the read answers first, that go is VOID and there is no second try.

Tool reads keep their own failure (`tally/connection.rs:2377`), so X rows see the drain but not the workbook's decision. W4 sees that.

## Part W4: the export at the cut
1. Drain record clear. Arm the relay with `front-w4.xml`. In the app, export.
2. **EXPECT** the mailing post cut at about 20 s, and the export to fail with no file written. The message opens "ComplyEaze Bridge withheld the party/ledger master:" and holds "The read that failed also asked Tally for each party's dated mailing addresses" (`commands.rs:443`). Record the whole message and its remediation.
3. **EXPECT** from the relay's log, parsed: after `bridge_cut`, no standard master post until the STANDARD of step 4.
4. Then D1 to D3 and Whole as an `Xn`, through the server, which reads the same drain record.

## Part O: past the cap
- **O1.** The 2214 runner: `create-161` to `create-200`. MARKS after: EXPECT the last plus 40, voucher mark unmoved.
- **O2.** Copy the drain record: clear.
- **W3.** The app's export. The answer is about 37.6 MB (row 2214's L7), declared by Tally at about 5 s. EXPECT:
  - in the log, parsed: the mailing post first, then status sends if a drain is owed, then a standard master post;
  - a workbook written, 215 ledgers, every Mailing status "not observed";
  - cell C3's note holds "Tally's answer to that read passed the 32 MiB limit, so this workbook was read again without them" (`reports/party_ledger_master.rs:146`, `:180`), cell C4 empty;
  - the app's notes equal to the workbook's;
  - the master mark unmoved.

  If the export fails after the wait with "Tally was still busy" (`MailingRetryHeldByDrain`), that is a result about the 60 s bound: record it, whole. Record the time from the mailing post to the standard post, against 60 s.

## Close
1. Writes off: reads only. Stop the app, the server and the relay. Check `relay.jsonl` by parsing it, for each X row and W row.
2. Re-hash the inputs and the exe.
3. **Restore BK-2074-2214 by C6**, on the run coordinator's word.
4. MARKS: EXPECT as after S0. `ledger_masters` basic, on port 9001 with a new server: EXPECT 11 ledgers.
5. C10.

## Decides
| reading | then |
|---|---|
| W1 as EXPECT | #2074's L1 stands: the live workbook shows the details past the mark |
| W1 standard request, or "not observed" | #2074 stays open: send the workbook and the log |
| W2 whole | the workbook takes a 30 MB mailing read; record its time |
| W3 as EXPECT | #2214's acceptance line is met live |
| W3 held by the drain | the 60 s bound failed live: the maintainers decide |
| X1, X2 and X3 not VOID, standard read whole, marks as stated, W4 as EXPECT | a cut has been seen live; the maintainers may rule on moving `Deadline` to `ContinueOnceDrained`. This sheet moves nothing |
| any of X1 to X3 refused past the limit, or not whole | `Refuse` stands |
| every X row VOID | no cut was seen: #2214's deadline branch stays as it is, and the order finding goes to #2176 |

## Limits
The cut is made by queueing, not by a slow read: `Deadline` is the same failure to Bridge, but the time Tally stays busy here is the front's, so the gap measures the drain's behaviour and not how long a slow mailing read runs. X3 alone has the read under way at the cut. One synthetic book, one version (TallyPrime 7.2 Silver), parties of short addresses. The relay adds a little delay each hop, and holds each answer until it is whole, so an answer's head reaches Bridge later than it would direct. The stand-in check proves the relay's own behaviour, not Tally's.

## Keep
Tool answers, `relay.jsonl`, `bodies/` under C11's size rule (the 30 MB answers stay on the lab PC, listed by size and sha256), `front-*.answer`, the drain record copies, the three workbooks, the marks. Answers branch `lab/2074-2214-export-cut-answers`, with RESULTS graded verified, partial or unverified. Check that no machine path and no lane name is in anything pushed.
