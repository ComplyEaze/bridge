# Does Tally still run a ledger Create its client gave up on, and when? A busy gateway, a create abandoned at ComplyEaze Bridge's 20-second limit, and a read queued behind it (#1713), on BRIDGE ABANDON LAB

**Synthetic company `BRIDGE ABANDON LAB` only, NEW for this sheet.** If a company or a data folder of that name already exists on the PC: STOP and report it; never reuse or overwrite one. It is loaded alone, and nobody else uses this PC's Tally from step 0 to the last step.

**This sheet WRITES.**
- By import: two ledgers; 24,000 Journals (96 files of 250), and up to 24,000 more (files 97 to 192) only as far as M0's measurement allows; and seven probe ledgers `Lab Probe L0`, `L1`, `L1B` and `L2` to `L5`, each name used once.
- On Tally's own screens: the company's creation.

**Gates.**
- **The owner's yes** for these writes is recorded in `notes.txt`, with its time, before S1. Without it nothing is created, imported or sent.
- **Tally:** licensed TallyPrime release 7.2 (the lab PC runs 7.2 since 25 Oct 2026), Silver, a single currency. Record the release and the edition from Tally's About screen before S0. Gold, another release, or Educational mode anywhere = STOP.

No ComplyEaze Bridge build is used. Every request is sent by curl, as the ruling of 26 Oct 2026 allows for lab requests, writes included. Allow about five hours, most of it waiting:
- up to 1.5 hours of bulk import;
- about 35 minutes for each of the rows L1B to L5, run by a script that also does its observation.

LAB-COMMON v3 applies in full: file `lab/626-1038-1050-run-sheets/LAB-COMMON.md` on branch `lab/626-1038-1050-run-sheets`, commit 8c602c15. Where this sheet and LAB-COMMON differ, the stricter holds, except for the deliberate differences listed next.

## Deliberate differences from LAB-COMMON (each one stated, nothing else differs)

1. **Overlapping requests.** In rows L1 to L5 only, `row.ps1` sends A, B and C so that they overlap, which is the point of the sheet. Everywhere else, one request at a time.
2. **Client limits (LIMIT 20).** `row.ps1` sends A with `--max-time 20` in L4 and L5, and B with `--max-time 20` in L2 to L5; every other send has no limit. That is ComplyEaze Bridge's own request limit (`DEFAULT_REQUEST_TIMEOUT`, 20 s, in `bridge-tally-transport`), so curl gives up as Bridge does. For those sends, and for C, curl's exit codes 28 (limit reached), 7 (connection refused), 52 (empty reply) and 56 (connection reset) are **results**, classified by the rules in "Rows"; they are not STOPs. Any other exit code in a row is a STOP, and the script makes it one.
3. **Timing fields and the curl line.** Every send in the scripts uses C3's line with `-w "%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}"` (`bulk.ps1`: `%{http_code} %{size_download} %{time_total}`), `--stderr <file>`, so curl's error text is kept, and `--noproxy "*"`, so no proxy setting on the PC can route a send away from `127.0.0.1`. `row.ps1` logs each send's start and end in UTC milliseconds. Requests sent by hand use C3's line unchanged.
4. **The BUSY read's bound is 400,000,000 bytes, not 2,000,000.** BUSY's answers are hundreds of megabytes. The bulk book is sized so that BUSY is predicted at or under 300,000,000 bytes from M0's measured bytes per row (M0), which leaves the bound a margin of a third. Disk space on the evidence drive: at least 3 GB before M0; at least 2 × the predicted M1 size + 1 GB before M1; at least 2 × M1's size + 1 GB before each row (`row.ps1` checks this itself). A complete BUSY answer whose size differs from its CONTENT-LENGTH, or whose VOUCHER row count differs from the Journals imported, is a STOP. Measurements part 11c.1 records that answers this large can fail partway.
5. **The bulk import runs** (S4, M0) are sent by `bulk.ps1`. It takes a pause for a screenshot after the first and the last file of each run, not after every file. MARKS is read before and after each run, not before every file, so C3's "a write is sent only if the MARKS answer read just before it…" holds for the run's first file, and no key is pressed in Tally during the run.
6. **Reads sent again.** Inside `row.ps1`, the ledger read C is sent again until one is answered: 30 s apart after curl exit 7, until D + 10 minutes after the first C; and at most 5 times after exit 52 or 56 (a sixth such end is a STOP). Exit 7 (refused) never reached Tally. Exits 52 and 56 (closed or reset with no answer) may have reached Tally's work: C is a read, so a second one changes no book, but any read that did reach Tally adds a read's work to the queue, which `times.txt` records, hence their own cap. The window is bounded by time, not by count, because reference 11b.2 measured 523 s of refused connections after an abandoned read, and L4 and L5 abandon a read of hundreds of megabytes. The observation reads are the same files sent again on their schedule, each answer saved separately; an observation read that fails is a STOP. Outside the scripts, C3's rule holds: a failed request is a STOP and is not sent again.
7. **Waiting.** A send that has not ended stops the sheet after D + 10 minutes, not 10 minutes; D is BUSY's measured time (M1). `row.ps1` carries the watchdog itself: at D + 10 minutes after a send was queued it logs `STOP: … has not ended D + 10 minutes after it was sent`, sends nothing more and keeps waiting (`bulk.ps1` likewise at 10 minutes). Leave that window open, press no Ctrl+C and no key in Tally, and report (C9).
8. **Dialogs while a script runs.** C3's screenshot after every write before the next request is replaced, inside `row.ps1` and `bulk.ps1`, by this: for as long as a script runs, the operator takes a screenshot of Tally's whole window at least every 30 seconds ("Running the scripts"). A dialog or a half-entered screen = at once create an empty file named `STOP` in this folder, take a screenshot, press no key, report (C7). Each script checks for that file before every send and at least once a second while it waits; it then sends nothing more, waits for any curl still running to end by itself, and exits with a STOP line. After a script ends, and before the next request, the screenshot of the whole Tally window is taken as C3 asks. A `STOP` file stays until the run coordinator's word.
9. **An abandoned send is not a complete answer.** A LIMIT-20 send that ends with exit 28 (A in L4 and L5, B in L2 to L5) is the row's result. C3's status, empty-body and size-equals-CONTENT-LENGTH checks do not apply to it, and difference 4's size STOP does not catch it. Its partial answer is kept, listed by size and sha256, and not read. Every other send in the scripts gets C3's checks in full, then `read.py`.
10. **Step 0 without a company.** At S0 no company exists yet: C2 d is replaced by a screenshot of the Gateway showing no company loaded, and the first MARKS is S1's.
11. **BUSY is counted, not tabled.** C4 asks for a table of every VOUCHER row. BUSY's answers are hundreds of megabytes of the sheet's own Journals, so `read.py busy` counts their rows by structure and checks every GUID is THE COMPANY's; no table is made. Every other answer is read as C4 says.
12. **MARKS before B.** In each row, the MARKS read by hand after the previous row (or before L1) stands as the read "just before" the write B (C3). Only the row's own read A is sent between them, and no key is pressed in Tally.
13. **The scripts run in their own console windows** and are watched through their logs ("Running the scripts"); the screenshot pauses of `bulk.ps1` wait for a `GO-<file>` file, not a key.

C3's sha256 check before each send is kept: `row.ps1` checks each request against `SHA256.txt`, and `bulk.ps1` each bulk file against `BULK-SHA256.txt`, immediately before it is sent.

THE COMPANY in LAB-COMMON is `BRIDGE ABANDON LAB`; record its company number and GUID at S1.

## Why, and what each answer decides

ComplyEaze Bridge creates a ledger with one `Create` behind an approval window. When Tally does not answer within 20 s, Bridge stops waiting, and its later reads can find the name absent. Today Bridge then holds the name in every company of its data folder, because nothing it can read rules out the create still waiting inside Tally (#1713).

The public protocol reference measured an abandoned **read** (measurements part, section 11b.2): the gateway stayed busy about nine more minutes after the client left, and every other request failed meanwhile. No abandoned **write** has been measured. The rule Bridge wants to rely on is: a request answered after the create was sent proves the create has either run or been dropped. This sheet tests that rule.

## Rows

T0 is the moment A is queued. Every row's plan prints in five lines with `-DryRun`. Observation reads (LEDGERS, MARKS and netstat at C answered + 1, + 5, + 15 and + 30 minutes) come after every compared answer: they record, and are not counted as a difference between rows.

| row | compared with | the one thing that differs | decides |
|---|---|---|---|
| L0 | | an idle gateway | the control: one Create, its answer, its mark step |
| L1 | L0 | the gateway is busy with a long read (A, at T0); the create (B, at T0 + 5 s, no limit) is waited out | whether a request sent while the gateway is busy is accepted and queued. GATE: B's end after A's first byte, else STOP (B ending during A's stream is a NOTE) |
| L1B | L1 | a ledger read (C) is also sent at T0 + 26 s, when L2 sends it, with L2's observation | the queue-order control: whether B and C are answered in the order they were sent, with A ahead of both. GATE: A's first byte < B's end < C's end, else STOP before L2 (the gates compare an end against A's first byte, not A's curl end: L1 showed that A's long body streams for seconds after Tally began to answer it) |
| L2 | L1B | B's limit: 20 s (abandoned) instead of none | THE QUESTION: is the probe ledger present in C's answer, and does anything change afterwards? |
| L3 | L2 | the probe name only (a repeat, P5) | that L2's answer repeats |
| L4 | L2 | A is abandoned at 20 s too | the same, when no client waits for A either |
| L5 | L4 | B is sent only when A's curl has exited: one request at a time, the next after the last was given up, as ComplyEaze Bridge orders them | the same, with Bridge's order of sends in time (not its connections: see below) |

**C's send time** is one rule in L2 to L5: T0 + 26 s, or 1 s after B's curl has exited if that is later. In L2 to L4 B exits at about T0 + 25 to 27 s, so C follows B as in L1B; in L5 it follows B's later exit. L1B's B has no limit and has not exited, so its C goes at T0 + 26 s.

**L5 runs only if D is at least 55 s.** Its B starts about 21 to 23 s after T0 (A's 20 s, plus a job's start), so it is abandoned at about T0 + 43 s. With D under 55 s, Tally would reach B within B's 20 s, and the row would be VOID by construction. With D under 55 s, L5 is recorded as not run, with that reason; `row.ps1` refuses it.

**What L5 measures, and what it does not.** It measures Bridge's order of sends in time: B only after A was given up, and C only after B was given up. It does not reproduce Bridge's connections. Bridge's client pools its connections and can send B on the connection A used, where Tally keeps connections alive; curl opens a new connection for every send. Whether a create sent on a reused connection behaves the same is not covered here (Evidence, "Not covered").

**How B's end is classified** in L2 to L5 (by `row.ps1`, in `times.txt`):
- exit 28 with `time_connect` above 0 and `size_upload` equal to the request file's bytes: **abandoned**, and the row counts;
- exit 28 with `time_connect` above 0 and fewer bytes uploaded: **VOID-not-sent**, the create never reached Tally whole;
- exit 0: **VOID-answered**, B was answered within its limit;
- exit 7, or exit 28 with `time_connect` 0: **VOID-not-reached**, B never reached Tally;
- exit 52 or 56: **VOID-closed**, B's connection closed with no answer, not abandoned at the limit;
- any other exit: STOP.

A void row sends no C. It is recorded and not counted; LEDGERS and MARKS by hand after it record where the probe stands. A netstat taken 10 s after B is sent records B's connection.

**PASS for the rule (decided before the sitting).** In each of L2 to L5 that counts:
- C is answered after A's first byte (L2, L3; a C that ends during A's stream is recorded as a NOTE and counts), or after A's and B's curls exited (L4, L5);
- whatever C shows for the probe ledger, present or absent, every observation read after it shows the same.

In L4 and L5 the first condition holds by construction, since C is sent only after both curls have exited, so only the second decides those rows.

**Expected after an abandoned send (UNVERIFIED for a write).** Reference 11b.2 measured 523 s of refused connections after an abandoned read. In L4 and L5, and perhaps in L2 and L3, C may therefore be refused (exit 7) for about nine minutes; those refusals are results, kept in `times.txt`. If no C is answered by D + 10 minutes after the first C, `row.ps1` STOPs, and C9 applies: do nothing more, send nothing by hand, report and wait.

**FAIL:** a probe ledger absent in C appears in a later read (`times.txt`: `FINDING: the probe was absent in C …`), or C is answered before A's first byte in L2 or L3 (`FINDING: C ended before A's first byte`; a C that ends after A's first byte but before A's curl end is recorded as `NOTE: C ended during A's stream`, not a FAIL). Either way, no read rules out a waiting create, and ComplyEaze Bridge keeps the name held.

**Confidence.** L2 and L3 are a repeated pair. L1B, L4 and L5 are single observations, PARTIAL by the reference's convention, until repeated.

**Builders.** None is new. "On master" means on master at 3d3dfdf2. "At a71f416c" means only on the head of the open pull request #1650 (branch `masters-ledger-create-series`, commit a71f416c9f94254e9786818965c93f37c5fb7863), not on master.
- The create: `LedgerCreate::render_message` (`src-tauri/crates/bridge-tally-protocol/src/ledger_create.rs`, on master), as `LedgerCreateItem::render_import` (`src-tauri/src/tally/ledger_create_item.rs`, at a71f416c) renders it for a Suspense ledger, inside `render_import_envelope` (`src-tauri/crates/bridge-tally-protocol/src/import_envelope.rs`, at a71f416c).
- The Journals: `render_voucher_xml` and `render_native_vouchers_xml` (`src-tauri/src/agent_import.rs`, on master), in the same envelope with the report `Vouchers`, which on master `render_import_envelope` in `agent_import.rs` sends.
- The marks: `render_agent_company_high_water` (`src-tauri/src/agent_read_profiles.rs`, on master).
- The year's vouchers: the verification window read (`src-tauri/src/agent_import.rs`, on master).
- The ledgers: `ledger_name_index_read` (`src-tauri/src/agent_read_profiles.rs`, the `StandardLedgerCatalogV3` profile, at a71f416c), the read Bridge's post and reconcile send.

What this sheet decides is the release rule in `post_ledger_creates` (`src-tauri/src/agent_master_tools.rs`, at a71f416c), not a request shape.

## Files

- `requests/` (11 requests).
- `row.ps1`: rows L1 to L5. `-DryRun` prints a row's five plan lines and sends nothing.
- `bulk.ps1`: the bulk runs of S4 and M0. `-DryRun` prints three plan lines and sends nothing. Before each send it writes `<file>.sent` in its answers folder, and it refuses to start if any file of its range has a `.sent` file or an answer.
- `read.py`: every answer's reading by C4, streamed, so BUSY's large answers are never held whole (`python -I read.py` prints its modes). Both scripts call it; the steps below call it by hand.
- `gen.py`, `BULK-SHA256.txt`, `.gitattributes`, `SHA256.txt`.

Neither script has run on Windows before the sitting: the step "Before the sitting" parses both, and S0 runs their dry runs. Check every file against `SHA256.txt` first. The bulk files are not in the branch: S4 writes them on the lab PC with `python -I gen.py bulk`, which checks each against `BULK-SHA256.txt` and prints `bulk ok` only when all 192 match.

| request | what |
|---|---|
| `marks.xml` | MARKS: GUID, ALTVCHID, ALTMSTID |
| `ledgers.xml` | LEDGERS: every ledger's name, GUID, parent, bill-wise flag and ALTERID, with THE COMPANY's GUID on each row |
| `busy-year.xml` | BUSY: every voucher from 1 April 2025 to 31 March 2026, by Bridge's verification read |
| `s3-bulk-ledgers.xml` | two ledgers: `Lab Bulk Expense` (Indirect Expenses) and `Lab Bulk Income` (Indirect Incomes) |
| `l0-create.xml`, `l1-create.xml`, `l1b-create.xml`, `l2-create.xml` … `l5-create.xml` | one Suspense ledger each, named for its row |
| `bulk/bulk-001.xml` … `bulk-192.xml` (made at S4) | 250 Journals each, dated across the year, 1.00 to 100.00 |

**Reading the answers.** By hand, each request is sent by C3's line and then read by `read.py`; the scripts do the same. A `FAIL:` line from `read.py` is a STOP, reported as "the request failed", never as "nothing found".
- **Imports:** `python -I read.py import <answer> <objects>`: EXPECT `import ok CREATED <objects> …`; record LASTVCHID and LASTMID. The counters may come in Tally's enveloped IMPORTRESULT or in a bare RESPONSE root. S3 on 28 Oct answered a bare RESPONSE (538 B, CREATED 2), which halted the first sitting. Exactly one of them must be present, with the same checks.
- **MARKS:** `python -I read.py marks <answer> <GUID>` (at S1, without the GUID): exactly one COMPANY row. GUID and ALTMSTID must be present. ALTVCHID may be absent: THE COMPANY holds no voucher, and Tally sends no ALTVCHID until one exists (seen at S1 on 28 Oct, which halted the first sitting). No step decides on ALTVCHID.
- **LEDGERS:** `python -I read.py ledgers <answer> <GUID> "<probe name>"`: every row carries THE COMPANY's GUID and `Lab Bulk Expense` is present once (else FAIL). It prints every `Lab Probe` ledger with its length, and `probe=present` with GUID, parent and ALTERID, or `probe=absent`.
- **BUSY:** `python -I read.py busy <answer> <GUID> <rows>`: VOUCHER rows counted by structure, each GUID THE COMPANY's. Record the size, CONTENT-LENGTH and `time_total`.
- **`times.txt`:** the row script's `times.txt` is the record of order; every line it writes is also printed. For each send it writes both its curl end (`end <name> at ...`) and its first byte (`first byte <name> at ...`, the send's start plus curl's `time_starttransfer`; the start is taken just before curl.exe is launched, so the first byte reads early, and an end is taken after curl.exe exits, so an end reads late; each error is at most that send's (end - start) - time_total, which `times.txt` gives: 0.128 s for A and 0.089 s for B in L1, against a margin of about 0.58 s). The order gates compare B's or C's end against A's first byte.
- **Connections:** record the `Connection` header of every answer, from its `.headers.txt`. Every send here is on a new connection. Where Tally keeps connections alive, Bridge's pooled client may reuse one and curl does not, so these rows measure new connections only.

## Before the sitting

1. Record Tally's release and edition (Gates). Check every file here against `SHA256.txt`. No file named `STOP` is in this folder.
2. Parse both scripts without running them, from **cmd.exe** (in PowerShell or a bash shell, the outer shell would expand `$e` and `$null` inside the double quotes):

   ```
   powershell -NoProfile -Command "$e=$null; [void][System.Management.Automation.Language.Parser]::ParseFile((Resolve-Path .\row.ps1),[ref]$null,[ref]$e); $e.Count"
   powershell -NoProfile -Command "$e=$null; [void][System.Management.Automation.Language.Parser]::ParseFile((Resolve-Path .\bulk.ps1),[ref]$null,[ref]$e); $e.Count"
   ```

   EXPECT `0` from each, else STOP. The operator reads `row.ps1` and `bulk.ps1` in full and confirms that they send only to `http://127.0.0.1:9001`, and only the files under `requests/` and `bulk/`.
3. The owner's yes (Gates), in `notes.txt`.

## Running the scripts

- **Dry runs** (`-DryRun`) end at once and send nothing; they may be run as ordinary commands.
- **A live run is never a call that waits for the script.** A row takes D + 30 minutes or more, and a bulk run up to 1.5 hours. A call ended at its time limit would end PowerShell, and perhaps a curl, partway through a row: that ends a process, which C2 forbids, and it can leave a write unrecorded. Each live run is started from a PowerShell prompt in this folder, in its own console window:

  ```
  Start-Process powershell.exe -WorkingDirectory . -ArgumentList '-NoProfile','-File','.\row.ps1','-Row','L2','-CompanyGuid','<S1 GUID>','-BusySeconds','<D>','-BusyBytes','<M1 size>','-BusyRows','<M1 rows>'
  Start-Process powershell.exe -WorkingDirectory . -ArgumentList '-NoProfile','-File','.\bulk.ps1','-First','1','-Last','96'
  ```

- **Execution policy.** The scripts run under the PC's own execution policy, by `powershell -NoProfile -File`. `-ExecutionPolicy Bypass` is never used. If the policy refuses a script, nothing was sent: STOP and report.
- **Watching, at least every 30 seconds until the run ends:**
  - read the last lines of the run's log with a short call: `C:\bridge-lab\1713\evidence\starts.txt` first (each start, and any refusal before a row's own log exists), then `C:\bridge-lab\1713\evidence\<row>\times.txt`, or `C:\bridge-lab\1713\evidence\bulk\bulk-answers.txt`;
  - take a screenshot of Tally's whole window.
- **A run has ended** only when its log's last line is one of: `row done`, `row done (void)`, `row ended by STOP: nothing more was sent`, `DONE: …`, a line of `bulk.ps1` beginning `STOP`, or a refusal in `starts.txt`. Start-Process does not wait, so the exit code is not read; the last line is the result.
- **A dialog or a half-entered screen** in any screenshot: at once `New-Item -ItemType File .\STOP` in this folder, press no key, report (C7).
- **A `WAITING` line of `bulk.ps1`:** take the screenshot of Tally's whole window, then `New-Item -ItemType File .\GO-<file>` with the name the line gives (for example `GO-bulk-001`).
- **Never** close a script's window, press Ctrl+C in it, or end its process. After a watchdog line (difference 7), leave it and report (C9).

## Steps

- **S0.** LAB-COMMON step 0 (C2), with NO company loaded (difference 10). Record the disk space available (EXPECT at least 3 GB). Run and keep the output of:
  - `powershell -NoProfile -File .\row.ps1 -Row L2 -DryRun`: EXPECT exactly these five lines:

    ```
    row L2; evidence C:\bridge-lab\1713\evidence\L2; STOP file: STOP in this script's folder
    A: busy-year.xml, limit 0 (0 = none), at T0
    B: l2-create.xml, limit 20 (0 = none), at T0 + 5 s; netstat 10 s after B is sent
    C: ledgers.xml, no limit, at T0 + 26 s or 1 s after B's curl has exited, whichever is later; sent again 30 s apart after curl exit 7 until D + 10 minutes after the first C, and at most 5 times after exit 52 or 56
    observation: LEDGERS, MARKS and netstat at C answered + 1, + 5, + 15 and + 30 minutes
    ```

  - `powershell -NoProfile -File .\row.ps1 -Row L5 -DryRun`: EXPECT exactly these five lines:

    ```
    row L5; evidence C:\bridge-lab\1713\evidence\L5; STOP file: STOP in this script's folder
    A: busy-year.xml, limit 20 (0 = none), at T0
    B: l5-create.xml, limit 20, when A's curl has exited; netstat 10 s after B is sent
    C: ledgers.xml, no limit, at T0 + 26 s or 1 s after B's curl has exited, whichever is later; sent again 30 s apart after curl exit 7 until D + 10 minutes after the first C, and at most 5 times after exit 52 or 56
    observation: LEDGERS, MARKS and netstat at C answered + 1, + 5, + 15 and + 30 minutes
    ```

  - `powershell -NoProfile -File .\bulk.ps1 -First 1 -Last 96 -DryRun`: EXPECT exactly these three lines:

    ```
    bulk: bulk-001.xml to bulk-096.xml (96 files of 250 Journals), one at a time, to http://127.0.0.1:9001
    answers: C:\bridge-lab\1713\evidence\bulk; one line each in C:\bridge-lab\1713\evidence\bulk\bulk-answers.txt
    screenshot pause after bulk-001 and after bulk-096 (files GO-bulk-001, GO-bulk-096); STOP file: STOP in this script's folder
    ```

  Anything else = STOP.
- **S1.** Create THE COMPANY on Tally's screen: name `BRIDGE ABANDON LAB`, financial year and books beginning 1 April 2025, base currency INR, no password, every other field as the screen offers. Screenshot the Company Creation screen before accepting. The screens and prompts expected, and the only ones answered: the Company Creation screen's own `Accept ?` (Yes); then, if the Company Features screen opens, accept it unchanged and record every value it shows. Any other prompt or screen = C7. Screenshot the Gateway showing it loaded alone, and record the Current Period. MARKS: record the GUID exactly as `read.py marks` prints it, and the company number.
- **S2.** Backup BK0 (`C:\bridge-lab\1713\backups\bk0\`).
- **S3.** MARKS, then `s3-bulk-ledgers.xml`: EXPECT `import ok CREATED 2`. Screenshot, then MARKS.
- **S4.** `python -I gen.py bulk`: EXPECT `bulk ok` (else STOP). MARKS. Then start `bulk.ps1 -First 1 -Last 96` ("Running the scripts"): EXPECT its last line `DONE: bulk-001 to bulk-096, 96 files, each CREATED 250`. It waits for `GO-bulk-001` and `GO-bulk-096` after those files, for the screenshot of Tally's window. Then MARKS. `bulk.ps1` is never run again after a STOP.
- **M0.** BUSY alone, no limit, bound 400,000,000 bytes: C3's checks, then `read.py busy … 24000`, EXPECT `busy ok rows 24000`. Record its size as S0 and its `time_total` as D0. Then `python -I read.py plan <S0> <D0>` and record its line.
  - The plan's rule: bytes per row = S0 / 24,000, as measured. Reference 11c.1 predicts about 9.4 KB a row in UTF-16 for an accounting-only synthetic book, which is unverified for this one, so it is not used. No more files if D0 is 60 s or more. Otherwise the most files, up to 96, that keep (96 + files) × 250 × bytes per row at or under 300,000,000.
  - If it names files: MARKS, then start `bulk.ps1 -First 97 -Last <the last file it names>` the same way, EXPECT `DONE: …`, then MARKS. Otherwise go to S5.
- **S5.** Backup BK1 (`…\bk1\`): the bulk book.
- **M1.** EXPECT disk space at least 2 × the size the plan predicted (S0 if it sent no files) + 1 GB. BUSY alone, no limit, bound 400,000,000 bytes: C3's checks, then `read.py busy`, EXPECT rows = 250 × the files sent. Record its size, CONTENT-LENGTH, rows, and its `time_total` rounded up to a whole second as D. Then `GET /status`.
  - EXPECT D between 40 s and 300 s. Below 40 s, a create sent 5 s into BUSY may be answered within Bridge's 20 s: STOP, and the run coordinator decides. Above 300 s: STOP.
  - D below 55 s: L5 is not run (Rows); record that.

Each row is started as "Running the scripts" shows, with `-Row <row> -CompanyGuid <S1's GUID> -BusySeconds <D> -BusyBytes <M1's size> -BusyRows <M1's rows>`, and watched to its end. After every row, before the next: a screenshot of Tally's whole window, `GET /status` answers, then LEDGERS and MARKS, by hand.

- **L0** (control). MARKS, then `l0-create.xml`, no limit: EXPECT `import ok CREATED 1`. Screenshot. LEDGERS shows `Lab Probe L0` under `Suspense A/c`. RECORD ALTMSTID's step (reference 9.4i saw +1).
- **L1.** RECORD from `times.txt`: B's start, end and exit code, and B's read. EXPECT `import ok CREATED 1`, `ORDER ok: B ended after A's first byte` and `row done` (the gate compares B's end against A's first byte, `start + time_starttransfer`, both written to `times.txt`; B ending during A's stream is a `NOTE`, not a STOP); LEDGERS then shows `Lab Probe L1`. GATE: anything else = STOP, and L1B is not run.
  - **Deviation (29 Oct 2026, dated):** L1 ran once and halted on its first gate, `B ended before A ended`, which compared curl ends. From the `curl:` fields of `end A` and `end B` in its `times.txt`, A's first byte (start + `time_starttransfer`) came about 0.58 s before B's end, so B ended during A's stream of 238 MB: Tally answered B while A's body was still arriving. L1 is kept as measured and is not run again; the gates now compare an end against A's first byte (A's first byte marks the end of A's work in Tally, since its CONTENT-LENGTH is sent before the body); `python -I read.py busy` is run by hand on L1's `A.xml` and recorded (the old script halted before that read). The sitting resumes at L1B on the same book.
- **L1B.** RECORD A's first byte, the ends of A, B and C, and C's probe line. EXPECT `ORDER ok: A first byte < B end < C end`, `probe=present` in C (the script STOPs if C shows it absent) and in every observation read, and `row done`. GATE: anything else = STOP; L2 is not run, and the run coordinator decides.
- **L2, L3, L4, L5** (L5 only if D is at least 55 s). Each is run to its end, about 35 minutes.
  - RECORD from `times.txt`: A's and B's ends and exit codes, B's classification line, every C attempt with its exit code, which attempt is C, C's end, and A's end.
  - RECORD from C's read and from each `ledgers-plusN` read: the probe ledger, present or absent, with its GUID and ALTERID.
  - RECORD ALTMSTID from each `marks-plusN` read, and every `:9001` line of each netstat file, with its exit code from `times.txt`.
  - A row whose `times.txt` ends `row done (void)` is recorded as void, with its VOID line.
- **E1.** Backup BKE (`…\bke\`), MARKS, C10 close. THE COMPANY stays on the PC as BKE left it.

## STOP rules (beyond LAB-COMMON C8 and C9)

- **A dialog** or a half-entered screen, at any time and while a script runs too: create the `STOP` file at once, screenshot, press no key, report (C7).
- **A probe ledger absent in C that appears in a later read:** a FINDING, the decisive one. Let the row script finish. If it was L2, run L3 once, then STOP and report. Otherwise STOP and report.
- **A probe ledger that appears twice, or a ledger no row created:** STOP after recording.
- **A row script that exits non-zero, or whose `times.txt` ends otherwise than `row done` or `row done (void)`:** STOP. Do not run it again for that row.
- **`bulk.ps1` not ending `DONE: …`:** STOP, and `bulk.ps1` is never run again. A file it did not send is never sent by hand, and one it sent (it has a `.sent` file) is never sent again.
- **The watchdog line (difference 7):** leave the window open, press no Ctrl+C, report (C9).
- **`GET /status` not answering** within D + 10 minutes after a row's script ended: C9. Do nothing at all, report and wait. A restart of Tally is outside this sheet: the run coordinator decides it, and the sheet does not resume after one without a new word.
- **Hazard detail:** any finding that Tally refused every connection, or stopped, goes to the run coordinator, and its detail goes through `SECURITY.md`'s private channel, not into the pushed evidence.

## Evidence

- Pushed to a new branch, `lab/1713-abandoned-ledger-create-answers`, only after the pre-push scan and the run coordinator's yes (C11).
- Every request and answer with size and sha256 (C11), with these exceptions kept on the lab PC:
  - the bulk answers, one line each in `bulk-answers.txt` with their counters (the file is pushed);
  - BUSY's answers, the partial A of L4 and L5 included, listed by size, sha256 and row count;
  - screenshots, listed by name, size and sha256.
- `starts.txt`; each row's evidence folder: `times.txt`, the answers other than A, the headers, the `.curl.err` files, and the netstat files; the bulk folder's `.sent`, header and `.curl.err` files.
- `notes.txt`, one line per step, with the owner's yes and its time, Tally's release and edition, D0, the plan line, D, and whether L5 ran.

**Not covered here:**
- a create sent on a connection Bridge reused (if Tally keeps connections alive);
- a busy gateway caused by anything other than a read;
- Gold;
- a Tally restart during a row;
- an abandoned create of another master type.

Each is a later row if the rule needs it.
