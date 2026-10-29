# #2173: the tax audit's read of a period over 5,376 vouchers, before and after PR #2191, on a new synthetic book and on BRIDGE CORPUS DENSE

Not for merge. **A WRITE SITTING, THEN READS.** Phase A creates and fills a new synthetic company; Phase B only reads.
- **The owner's word:** the owner's 28 Oct 2026 word covers writes to synthetic books. Each write phase still waits for the run coordinator's yes.
- **LAB-COMMON:** `LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where this sheet and it differ, the stricter holds.
- **Files:** `SHA256.txt` lists every file here with its bytes; check it before step A0.
- **Time:** about 90 minutes.

**WHY.** The tax audit's read (`audit_test_run`) reads one period and cannot narrow it.
- **Today (before #2191):** it refuses any period holding more than 5,376 vouchers, before reading a single one.
- **With #2191:** it refuses such a period before reading only when no measured weight could fit it in 128 requests: at most 10,837 vouchers, fewer when days are dense.
- **The whole-day rule:** the audit must read its period in whole days. #2191 therefore also refuses, before any data part, a period with a day no request can carry whole. That is a first voucher day over 42 vouchers, or any later day over 85.
- **#2191 merges only after step B1 passes.**

This item records, through ComplyEaze Bridge's own tool calls:
- **B0:** today's refusal, on a new book of 5,960 vouchers.
- **B1:** the read #2191 now makes of that same book, part by part. This is the step the merge waits on.
- **B2:** #2191's up-front refusal of a dense book's quarter. Its days hold about 82 vouchers each, the first included, so no request carries the first day whole; only census requests are sent.
- **B3:** the dense book's year, a negative control: still refused, with no data part sent.

**THE COMPANIES**
- **NEW: `BRIDGE AUDIT FLOOR LAB`** (synthetic), created in Phase A; it does not exist yet. If a company of this name exists on the PC: STOP and report it. Its books begin 1 April 2025.
  - **Ledgers:** `AFL Expenses` (Indirect Expenses) and `AFL Capital` (Capital Account).
  - **Vouchers:** 5,960 Journals, each Dr `AFL Expenses` 100.00, Cr `AFL Capital` 100.00, with no voucher number given (Tally numbers them). They fall 40 on 1 April 2025, then 80 on every day from 2 April to 14 June 2025.
  - **Its GUID is new.** It goes into the merge gate's synthetic-UUID registry before any evidence naming it is pushed. Until then the evidence shows it as `<company guid>`.
  - It is kept, shut, for later re-reads. No restore is planned.
- **`BRIDGE CORPUS DENSE`** (synthetic), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3` (registered).
  - **Mark:** 29,900 at the sittings of 28 and 29 Oct.
  - **Period:** 1 April 2025 to 31 March 2026.
  - **Density, recorded on one day:** 15 May 2025 held 82 vouchers, at AlterIDs 3579 to 3660 (lab row 485). B2 and B3's EXPECT lines assume about 82 vouchers on every day, the first included, and AlterIDs that follow the dates. If B2's census rows say otherwise, record them; that is not a STOP.
- **Mode:** licensed only; Education mode = STOP. TallyPrime 7.2 expected: record the release.

## Who acts, and how
- **The lab operator, on the lab PC.**
  - **On Tally's screen:** create the company, take the backup, and load and shut companies. Nothing else is keyed.
  - **By curl, under C3:** the marks reads and the imports of Phase A. These are the request files in `requests/`.
  - **Phase B tool calls:** made through the lab's own MCP client, which launches `bridge_mcp.exe` over stdio with the environment below, one call at a time. It saves each answer whole as a file, with its size and sha256.
- **Ending a process.** Two processes may be ended: the build's own stdio session (by closing the MCP client's session) and the relay this item started (Ctrl+C in its console, under Difference 4). This is the one exception to LAB-COMMON's rule that nothing ends a process. Tally is never ended. Nothing is ended with taskkill.

## Differences from LAB-COMMON
1. **Raw requests only in Phase A.** In Phase B the operator sends no request file to Tally; every read is a tool call through the relay. C2 step 0 runs WITHOUT its MARKS request in Phase B; its `GET /status` checks still go straight to 127.0.0.1:9001.
2. **Writes.** The grant is exactly the following, and nothing else:
   - the creation of THE NEW COMPANY on Tally's screen;
   - one import of two ledgers (`m01`);
   - thirteen voucher imports (`v01` to `v13`), each sent ONCE (C3).
3. **Backup.** C5's backup before the first change cannot apply to a company that does not exist yet. The run coordinator waives it for the company's creation and its two ledgers.
   - One backup, BK0, of THE NEW COMPANY, after its ledgers and before its first voucher. It goes into the new empty folder `C:\bridge-lab\2173l\backups\bk0\`, under C5.
   - No restore is planned.
4. **The relay.** `read_relay.py` sits between ComplyEaze Bridge and Tally on 127.0.0.1:9012. Read its header.
   - **What passes:** it forwards only `GET /status` and Export reads to 127.0.0.1:9001, and answers anything else 403 without forwarding it.
   - **What it records:**
     - every request, whole;
     - every answer by size, sha256, rows and seconds (an answer of at most 1 MiB is also kept whole);
     - the census rows only as counts: census answers over 1 MiB are not kept.
   - **Its timing is Tally's own.** It reads every answer to its end with no time limit, even after ComplyEaze Bridge has given up at 20 s. Its `seconds` column is therefore Tally's real time.
   - **Self-test, before Phase B:** run `python -I read_relay.py --self-test`. EXPECT exactly `self-test: ok (7 forwarded unchanged, 12 refused, none of the refused reached Tally; rows by structure; an abandoned answer read to its end)` and exit code 0. Anything else = STOP.
   - **Stopping it.** The relay writes a request's row only once its whole answer is read, so the manifest cannot show a request still in flight. Before stopping it, send `GET http://127.0.0.1:9012/status` THROUGH THE RELAY with `curl.exe -sS -w "%{http_code}" http://127.0.0.1:9012/status` (no `--max-time`). The relay sends one request to Tally at a time, so this one waits behind any answer still being read and returns only once Tally has answered every earlier request. Its row is then the last.
     - EXPECT 200. Only then press Ctrl+C in the relay's console.
     - After a deadline that wait may take minutes. Up to 15 minutes is allowed. No answer by then = STOP: leave the relay and that curl running, and press no key.
5. **Screens.** Take a screenshot of the whole Tally window before each Phase B step's first call and before each import. A dialog or a half-entered screen = C7: STOP.
6. **Time.** Each tool call has 15 minutes. A call with no answer by then: STOP, leave it running, press no key, send nothing more, report.
7. **Held gateway.** After each tool call answers:
   - First the relay's own status check (Difference 4, `GET http://127.0.0.1:9012/status`). It returns only once Tally has finished any answer ComplyEaze Bridge gave up on.
   - Then `GET /status` straight to 9001. No answer within 60 seconds = the gateway is held: STOP, leave everything running, press no key, send nothing more, report, and wait.

## The requests (`requests/`, UTF-16LE with a byte-order mark; check each against `SHA256.txt` before its first send)
- **How they were made.** Each is the text ComplyEaze Bridge's own builders render, at master `026ba163`. A throwaway test called them with THE NEW COMPANY's name and wrote their output; no byte was typed by hand.
  - `m00-marks`: the company marks read (`render_agent_company_high_water`), the request the product sends.
  - `m01-ledgers`: the lab writer's own All Masters import of the two ledgers (`render_master_batch_xml` with `render_ledger_xml`, `agent_lab_import.rs`).
  - `v01` to `v13`: the lab writer's own voucher import (`render_voucher_batch_xml`), each holding whole days, at most 500 vouchers. Each voucher's narration ends with the writer's `[BRIDGE-LAB:<id>]` attribution marker, and each carries its own `REMOTEID`. `plan.tsv` lists each file's vouchers and first and last date.
- **Sizes.** The marks answer has a READ BOUND of 16,384 bytes; each import's answer, 65,536 bytes.
- **Encoding.** Each file is UTF-16LE with a byte-order mark. The import files' text opens with `<?xml version="1.0" encoding="UTF-8"?>`, as every import from ComplyEaze Bridge's writers does; the bytes are UTF-16 nonetheless, and the `Content-Type` C3 sends says so.
- **`SHA256.txt`** has three columns (sha256, bytes, path) and does not list itself. `shasum -c` cannot read it: compare each line with Get-FileHash and the file's size.

## The builds (Phase B)
- **BUILD-A (step B0):** candidate A, the release-candidate bundle of master `026ba163d`, CI artifact `12928034390`. The release-candidate sitting downloads it after the owner's OK, so no second download is needed.
  - Use its `bridge_mcp.exe`. Record its path, bytes and sha256 (Get-FileHash) before the first call and after the last; a change between them = STOP.
  - If the release-candidate sitting recorded a hash for the same file, the two must be equal, else STOP.
- **BUILD-B (steps B1 to B3):** the Windows bundle of PR #2191 at head `d01550869a4d3a6274ade2aa92b183ea039dd6ea` (branch `fix/2173-audit-floor-ceiling`).
  - A draft pull request runs no bundle job, so this comes from workflow run `40969427288` of `ci.yml`, started by hand on the branch.
  - The artifact is `unsigned-bundle-smoke-windows-latest`, id `12940610401`, 57,279,587 bytes, zip digest `sha256:ed9306cc1f56992b05063dffc42f04cfd2ac36b39e918ecea1ee9ec7cb491f9a`. It expires 5 Nov 2026 23:03 UTC.
  - It is downloaded only after the owner's OK in the lab chat. Check the downloaded zip's sha256 against that digest; another = STOP. Then hash the exe as for BUILD-A.
  - A later head of #2191 replaces this build only on the run coordinator's word, with its own run, artifact and digest.

## The environment (one ComplyEaze Bridge session per step; record each value; nothing else set)
- `BRIDGE_TERMS_ACCEPTED=true` (the owner allowed the lab to set it on 25 Oct 2026);
- `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9012` (the relay);
- `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`;
- `BRIDGE_AGENT_DATA_DIR` = a new empty folder for the step: `C:\bridge-lab\2173l\<step>\data`;
- no `BRIDGE_AGENT_REDACTION` (a redaction setting refuses `audit_test_run`), and no lab-write, import or posting setting.

## Phase A: build THE NEW COMPANY (WRITES, after the run coordinator's yes)
**A0.** C2 step 0 with no company loaded (a company already loaded = STOP), without its MARKS read. EXPECT no company named `BRIDGE AUDIT FLOOR LAB` in Select Company's list.

**A1.** Create the company `BRIDGE AUDIT FLOOR LAB`:
- financial year and books beginning 1-4-2025;
- no password;
- every other field as the screen offers. If the Features screen opens, accept it unchanged and record every value it shows.

Take a screenshot before accepting. Accept the screen with its own accept (no Escape on any screen, C7). Load the company alone. Then:
- Send `m00-marks` (name `m-a`). EXPECT one COMPANY row named `BRIDGE AUDIT FLOOR LAB`. Record its GUID, `ALTMSTID` and `ALTVCHID` (a company with no voucher may omit `ALTVCHID`: record that).
- Record the company number.

**A2.** Send `m01-ledgers`. Take a screenshot of the whole Tally window (C3: a dialog = STOP, nothing further is sent). Then send `m00-marks` (`m-l`).
- The import's answer: EXPECT `CREATED` 2, the other counters 0, and no `LINEERROR` (C4); else STOP.
- The marks: EXPECT `ALTMSTID` above `m-a`'s by at least 2. Record it.

**A3.** Take backup BK0 (Difference 3, C5). After the Backup screen has closed, take a screenshot of the whole Tally window. Then `m00-marks` (`m-bk`): EXPECT it byte-equal to `m-l`, or differing only inside `CMPINFO`. Any other difference = STOP before `v01`.

**A4.** Send `v01` to `v13` in order. After each one, in this order:
- a screenshot of the whole Tally window (C3: a dialog = STOP, nothing further is sent, the marks read included);
- `m00-marks`.

For each import:
- EXPECT `CREATED` equal to that file's count in `plan.tsv`;
- EXPECT `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` 0, and no `LINEERROR`; else STOP;
- record `ALTVCHID` after it.

EXPECT `ALTVCHID` to rise by that import's count each time. A different step is recorded, not a STOP, provided `CREATED` matched.

After `v13`, the 5,960 vouchers are in. Record `ALTVCHID` as M1; it should be 5,960 if the first voucher took AlterID 1 (not established for a new company: recorded). **M1 above 8,192 = record and report before B0**: every Phase B EXPECT on this book assumes a mark that fits one census request. Then C10 (shut THE NEW COMPANY).

STOP on: any dialog (C7); any import answer that differs from its EXPECT line; a voucher import's `CREATED` that differs from `plan.tsv`. A STOP part-way leaves the imports already sent in the book. Each one is sent once in this item's life, never again (C3).

## Phase B: the reads (READ-ONLY, tool calls only)
Before B0: the relay's self-test (Difference 4).

**Each step:**
1. Load the step's company alone. C2 step 0 (Difference 1), then a screenshot of the whole Tally window (Difference 5).
2. Start the relay in its own console, with a new recording folder:
   `Start-Process cmd.exe -WorkingDirectory <this folder, absolute> -ArgumentList '/k','python -I read_relay.py --listen-port 9012 --record C:\bridge-lab\2173l\<step>\relay'`
   EXPECT its first line: `relay: 127.0.0.1:9012 -> 127.0.0.1:9001, reads only, recording into C:\bridge-lab\2173l\<step>\relay; Ctrl+C to stop`.
3. Start the step's build with the environment above. Then:
   - `list_companies`: EXPECT exactly the step's company, with `identity_state` `verified_tuple`; record its GUID.
   - `tally_status`: record the release, the licence and the Education mode.
4. Call `audit_test_run` ONCE, with:
   - `company_guid` = that GUID;
   - `from` and `to` as the table says;
   - `test` = `cash_44ab`;
   - `cash_groups` = `["Cash-in-Hand"]` and `bank_groups` = `["Bank Accounts"]`.

   Note the UTC time at the call and at its answer, and keep the whole answer as a file. A refusal is an answer: record its code. Do not call it again in this session.
   - **A re-run after a deadline.** After `audit_part_deadline_exceeded`, the build owes a drain that clears only when it is restarted. A re-run of the step therefore needs a new session, a new data folder, a new relay folder and the run coordinator's word, after Difference 7.
5. Close the build's session. Then Difference 7 (the relay's last row, then `GET /status`), then stop the relay (Difference 4).
6. Run `python -I summarise.py C:\bridge-lab\2173l\<step>\relay` and keep its output. Check it against the EXPECT lines.
7. C10 (shut the step's company).

| Step | Build | Company | `from` to `to` |
|---|---|---|---|
| B0 | BUILD-A | BRIDGE AUDIT FLOOR LAB | `2025-04-01` to `2026-03-31` |
| B1 | BUILD-B | BRIDGE AUDIT FLOOR LAB | `2025-04-01` to `2026-03-31` |
| B2-pre | BUILD-B | BRIDGE CORPUS DENSE | `2025-04-01` (a `vouchers` call, not the audit) |
| B2 | BUILD-B | BRIDGE CORPUS DENSE | `2025-04-01` to `2025-06-30` |
| B3 | BUILD-B | BRIDGE CORPUS DENSE | `2025-04-01` to `2026-03-31` |

### EXPECT (recorded; the lab judges only the STOP rules)
**On any step,** an answer other than the step's EXPECT is recorded whole and reported before the next step.

**Every step's census can run past 20 s.** A full span of 8,192 rows over the dense book's year took 14.8 s and 25.0 s in two sittings. B0 and B1's single census holds 5,960 rows, and B2's first span about 7,460. So `audit_part_deadline_exceeded` is possible on B0 to B3, not only B3.

The other answers include:
- an `audit_part_deadline_exceeded`, a request ComplyEaze Bridge gave up on at 20 s. On the audit's read a slow census, trial balance, ledger or company read is reported under this code. The relay's seconds give Tally's real time.
- an `audit_part_response_size_limit_exceeded`, an answer over 32 MiB;
- an `audit_part_drain_required`;
- any engine code (`C1-…` to `C11-…`, `CFG-…`, `BIND-GROUP-UNKNOWN`).

**B0 (before #2191, the new book).**
- **The answer:** a refusal, code `voucher_window_too_many_reads`. This build plans every part at 42 vouchers. That is 149 requests: one for 1 April, then two AlterID spans for each of the 74 days of 80. It is over 128, so the build refuses before any data part. The count is not in the answer.
- **The census:**
  - EXPECT one census line with no span: the mark, about 5,960, fits one census request, which this build sends unspanned;
  - rows equal to M1 (5,960).
- **The other lines:**
  - `data parts: 0`;
  - `marks: 2` (the read's own opening read and the window's);
  - `over: none`.

**B1 (after #2191, the new book). The step #2191's merge waits on.**
- **The answer:** a result, `completed` or `completed_with_failed_book_checks`, either. Record which, with the result whole.
- **The census:** one census line, rows 5,960.
- **The data parts:**
  - `data parts: 75`: the first part is 1 April (40 vouchers), read at the default figure. Each later day of 80 is one part, read at the floor of 85 a request. The last part runs from 14 June to the period's end;
  - `first part rows 40`, `most rows 80`, `total rows 5960`;
  - `most bytes` at most 16,777,216;
  - every data part line with an empty span (no part divided by AlterID).
- **The other lines:**
  - `marks: 3` (opening, the window's, closing);
  - `over: none`.
- **Record:** the mean bytes per row (this book's voucher weight on the wire) and the call's whole time.
- **Other branches:**
  - **`voucher_window_too_many_reads` after one data part:** the first part measured over 196,608 bytes a voucher, and a day of 80 no longer fits a part. Record the mean; the change behaves as designed.
  - **`audit_window_divided_by_alterid`:** a defect in the build, since no day here needs dividing. Record it and report at once.
  - **More than 75 data parts, or more than 128:** record and report.
  - **The call over 240 s:** record its time. A desktop chat's host limit would then bound this read, not the read count.

**B2 (after #2191, the dense quarter).**
- **The pre-check, B2-pre, first: a step of its own.** It has its own relay folder (`C:\bridge-lab\2173l\B2pre\relay`), data folder and session, run by the step procedure above with one difference: in place of `audit_test_run`, call `vouchers` once with BRIDGE CORPUS DENSE's `company_guid`, `from` and `to` both `2025-04-01` (one day: no month question arises). Record how many vouchers the answer holds, every page. EXPECT more than 42.
  - 42 or fewer: do NOT run B2. Record the count and report. The refusal B2 expects comes only from a first day over 42, and the read would otherwise send up to 127 data parts.
  - Then B2 itself, with new relay and data folders and a new session.
- **The answer:** a refusal, code `audit_window_divided_by_alterid`.
- **The census, finished, nothing stopped:**
  - four census lines with spans (0, 8192], (8192, 16384], (16384, 24576] and (24576, 29900];
  - their rows add up to the quarter's vouchers, about 7,460, if the AlterIDs follow the dates.
- **The other lines:**
  - `data parts: 0`: no `Bridge Agent Vouchers` request was sent;
  - `marks: 2`;
  - `over: none`, unless a census span ran past 20 s (the general branch above).

**B3 (after #2191, the dense year: the negative control).**
- **The answer:** a refusal, code `voucher_window_too_many_reads`.
- **The census lines:** the first two spans only, (0, 8192] and (8192, 16384], with about 16,384 rows in all. The census stopped there with spans left, because its count passed 10,837.
- **The other lines:** `data parts: 0`. A data part sent = a defect in the build: record it and report at once.
- **Timing:** each of these spans covers the whole year. One full span over this book's year was measured at 14.8 s and 25.0 s in two sittings (`lab/899-census-window-ladder-v2-answers` at 28aac587, and v1 at 3243b75c). So `audit_part_deadline_exceeded` is possible here (the general branch).

After B3: C2 step 0 again (Difference 1), then C10.

## Evidence and close
- **Phase A:**
  - every request's sha256 as sent;
  - every answer whole, with its size and sha256;
  - BK0's name, bytes, time and sha256;
  - the marks after each import;
  - THE NEW COMPANY's GUID and number, shown as `<company guid>` until it is registered.
- **Phase B, for each step:**
  - the tool answers whole;
  - the relay's folder: `manifest.tsv`, the requests and the kept answers;
  - the summary;
  - the build's exe path, bytes and sha256;
  - the environment as set;
  - UTC times.
- **Not pushed:** the data folders and the screenshots stay on the lab PC, listed by name, size and sha256.
- **Also:** the notes, with one line per step.

The evidence goes to a new branch, `lab/2173-audit-floor-ceiling-answers`, with `SHA256.txt`. Push it only after the pre-push scan and the run coordinator's yes.

## Not measured by this item
- An inventory-heavy book, whose voucher weight and census time differ.
- A period between 10,837 vouchers and the dense year.
- A book whose first voucher day holds between 43 and 85 vouchers. That is refused by the same rule as B2, through its first-day branch, which B2 also takes.
- Gold, and Education mode.
- The read through a desktop chat's own host limit.
