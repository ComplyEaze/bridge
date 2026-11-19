# Lab row 1108-R3: an equal voucher already in the book, then the batch of row R0. SYNTHETIC COMPANY ONLY. Not for merge. Draft for the run coordinator.

Part of #1108. About 70 minutes, one sitting. Form and rules are those of row 1108-R0 to R2
(sheet `lab/1108-r-rows-sheet` at `8c9c11b9`, answers `lab/1108-r-rows-answers` at `1300c41e`) and of the
2214 current-app sheet (`lab/2214-cut-sheet` at `a127d83b`). Every EXPECT below is from reading the code at the
build's commit or from a named record. **No EXPECT of this row has met Tally: all are unverified until it runs.** A
different result is a result, and is recorded; only the STOP RULES stop the row.

## Read this first

1. **The build is the desktop bundle already on the lab PC, and #2533 is not in it.** That bundle is 0.7.0, built
   from master `a83ee02da443fb1e2256919b18b4a5fb84056781`, with `bridge_mcp.exe` sha256
   `7086f1d0d7430e7dc1fefd6f69c0d4a14d717a2e06477b32935a1d3aa4fee1b8`. Nothing is installed or changed by this sheet.
   - The behaviour rows R0 to R2 ran on (master `de54a0f2`, which is an ancestor of `a83ee02d`) is in it: no non-test
     source file under `src-tauri/src` that touches the agent's import path differs between the two commits (the test
     files and some report-read files do), and the crates' changes between them are to report reads and records only.
   - **#2533 (`8db28bf2`, "Say how many vouchers Tally reported as not created when a post landed in part, and name
     none") is not an ancestor of `a83ee02d`** (`git merge-base --is-ancestor` says so; #2533 merged later the same
     day). So an unmatched row of a partly landed post, on this build, still carries the neutral line R0 recorded:
     "This voucher was sent to Tally, but ComplyEaze Bridge cannot match it in the book now. Check in Tally before
     posting it again." (`agent_import_verification.rs:1001` at `a83ee02d`). A newer build adds, after that line, one
     sentence giving how many vouchers Tally's answer reported as not created and saying the build cannot say which.
     It changes no status, and it is not reached on the refusal path this row EXPECTs. Other changes a newer build carries on the
     import path, none read by this row's EXPECTs: a kept refusal of a declined window now carries the cause `approval_answered_earlier`
     (#2516), and a post with no kept answer gets a next step pointing at `egress_log`. The pre-send refusal (`import_preexisting_identity`), the build's
     rows-in-the-book warning and the box's own lines read the same on master (the diff was searched for them).
2. **Approval windows to answer: one `post_import` is planned, so at most one answer; none is expected.** Its approval window
   exists only if the build gets past its own checks, which the code says it will not (item 3). If the window shows,
   the lab answers it once, by `helpers/p1108s2.py` and `boxcheck.py` as in R0, on the lab's standing word for
   synthetic books. **The row needs no answer from the owner, no application restart and no setting change.**
3. **What the code says this row will meet (unverified).** At `a83ee02d`, `post_import` reads the book over the
   batch's dates before it asks anyone, and refuses the batch as `import_preexisting_identity` ("Nothing was sent")
   when any voucher of the batch matches a voucher already in the book by date, type, ledgers, amounts and sides
   (`agent_import_post.rs:930`, `:2499`; the match ignores narration, `agent_import_verification.rs`
   `verify_batch_as`), and it checks again after approval (`:2660`). `build_import_xml` says so first, as a warning
   (`agent_import.rs:3997`, `rows_in_book_warning`). So the likely result of this row is **no partial landing at
   all**: the post is refused before its window, and the question is answered by what that refusal is. If the code
   is wrong and the window shows, the sheet goes on to the partial post and its readback (EXPECT E-4b).
4. **This sheet runs position (b) only, for two reasons.** (i) The voucher Tally refuses in R0 has a ledger under
   Stock-in-Hand, and Tally refuses that same content when it is sent alone (row 1108-A1: `CREATED 0`, `EXCEPTIONS 1`,
   "Cannot use 'A1 Closing Stock': Ledger under Stock-in-Hand!"; partial, one run). So an equal voucher cannot be put in the book
   beforehand by a raw import. Whether it can be keyed by hand is **not measured**: A1's step 4 read the first line's ledger
   list without pressing "Show More", and that list also left out `Cash`, which Tally stores on a Journal (A1's row B). (ii) Even if
   such a voucher stood in the book, the post's own check described in item 3 matches by the same content for every voucher of the batch, so position
   (a) would meet the same refusal before any window. This sheet does not send the refused voucher again, and does not key one. Whether to
   try (a) by hand is a question for the run coordinator (Open points).

## The one question

**After a batch of which Tally creates some vouchers and reports one not created, does the verification readback show a voucher that stood in the book
beforehand and is equal in content to a batch voucher as standing in for it, so that the unmatched row is not the one Tally refused?**

Run here in position (b): the equal voucher is equal to the batch's first voucher, one that Tally will create (in
R0 it was created, with ids 2 and 3 going to the first and third). The row also records what the build and the post do
when such a voucher is in the book, which decides whether the question can be asked on this build at all.

**What each reading would bear on option B** (naming the unmatched row as the one Tally reported not created, the
question of #2533) is in Decides. This row decides nothing about it.

**One variable against R0:** one Journal in the book before the batch is built, equal to the batch's first voucher.
The book, the batch (`batches/r3.json`, byte-equal to R0's `r0.json`), the build, the fault, the order sent and the
calls are R0's.

## Writes to the book

**YES, to THE COMPANY only, each named; the book is left as found by restoring the backup taken first (C5, C6).**
1. **BK-S**, a backup of THE COMPANY as the lab finds it: a read of the book, written to a new folder.
2. **Restore of BK-R**, the row backup of rows R0 to R2 (the START book plus three ledgers and one seed Journal; M 213, V 1
   at its baseline). It replaces THE COMPANY's data folder.
3. **E1**, one raw Journal import: `requests/e1-equal-first.utf16le.xml`, sent once. It is `b1108-p2-1` of `batches/r3.json`
   (14-Apr-2025, `A2 Expense` Dr 21.00, `A1 Capital` Cr 21.00, narration "Row 1108-P2 first"), with a REMOTEID of its own.
4. **One `post_import`** of the batch of R0, through Bridge. It writes to Tally only if the build gets past its own
   checks. Bridge's own folder `agent-r3` (a new data folder) is written either way.
5. **Restore of BK-S**, at the end, on the run coordinator's word.

No other raw write. No voucher of the batch is sent raw. No master is changed. Tally's current date is never typed
(see step 9).

## Needs, and what needs a person

- **On the lab PC, from rows R0 to R2** (read only, by `setup-r3.py`, which STOPs if any is missing or differs): the
  row folder of those rows with its company GUID and number files, their stdio client, and BK-R.
- **The installed 0.7.0 bundle's `bridge_mcp.exe`**, the file the 2214 row ran as its server, with the sha256 above. The
  desktop application is not started, restarted or touched by this sheet; if it is running, leave it.
- **Tally** 7.2, Licensed mode, THE COMPANY on port 9001. The relay of the kit on 9003.
- **The lab's acts on Tally's screen** (none by the owner): load, backup, restore, shut, by the keys of Difference 4.
  Counted: one Backup (step 2), two Restores (steps 3 and 9), three loads (steps 0, 3, 9) and three shuts (before each restore, and C10).
- **The one approval window**, only if it shows: answered by the helper after its check (Difference 3).
- **The run coordinator's word**, for the two restores (C6), as in every row.
- **Standing words the lab names in its notes:** raw requests, writes included (26 Oct 2026); the backup (C5); the
  approval window on a synthetic book answered by the lab.

## Rules and company

`LAB-COMMON.md` (v3, byte copy, sha256 `5370c6d2...`) applies in full. Where it and this sheet differ, the stricter holds.

THE COMPANY is `BRIDGE STOCK CREATE LAB` (synthetic; integrated, inventory on), with the GUID in `guid.txt` of the
rows R0 to R2. If it is not on the PC, or a GUID the row reads does not begin with that one: STOP. The row has no
book of its own to rebuild.

Differences from LAB-COMMON, as R0 to R2 ran them (the stricter rule holds everywhere else):
1. **Raw write:** E1 only, by `curl` to this lab book only, on the standing word of 26 Oct 2026. C4's import EXPECT
   applies: `CREATED 1`, every other counter 0.
2. **Bridge writes:** the one `post_import`, from its own fresh `build_import_xml` made just before it.
3. **Who answers the box.** As R0's Difference 3, by `helpers/` (a copy of R0's helpers with their folders changed,
   `SHA256.txt` lists each): Yes only if all hold: exactly one line of the box begins `Create `, and that whole line is
   `Create 3 vouchers in "BRIDGE STOCK CREATE LAB"`; the whole lines `Company GUID: <THE COMPANY's GUID>` and
   `Batch: <the batch id this build returned>`; the whole line `Total debit: 66  Total credit: 66`. Anything else:
   the helper answers No, the box text is recorded whole, and the row STOPs. **No box seen is the expected case,
   not a stop** (step 7.5).
4. **Keys and processes.** Keys are pressed in Tally only to load, back up, restore and shut THE COMPANY. Menus are
   opened by mouse click. As R0 to R2 made them on TallyPrime 7.2 (their notes record each):
   - **Load** (C1): at Select Company type `BRIDGE STOCK CREATE LAB`; EXPECT one row; Enter.
   - **Backup** (C5): click Data, Backup & Restore, Backup. The path field shows `C:\`: press Backspace, choose Specify
     Path, type the step's folder. Type the company name to filter; EXPECT one row; Enter; click End of List; Enter. At
     `Accept ?` press `y`. A message that only says the backup is done: screenshot, Enter once. Any other prompt: C7.
   - **Restore** (C6): click Data, Backup & Restore, Restore. Restore Data Path is Tally's data folder; choose Specify
     Path and type the backup's folder. EXPECT THE COMPANY with its number and the backup's date and time; Enter; click End of
     List, **take a screenshot**, and press `y` only at a seen `Accept ?` (R1's restore answered it before it was seen);
     at the folder-exists prompt choose Replace.
   - **Shut** (C6, C10): click Company, then Shut; Enter on the one row.
   - Never press Escape on a master, voucher or configuration screen. No process is ended.
5. **A decline is kept once** and any decline is followed by a STOP.
6. **A box left open** gives `approval.state` `pending` with `retry_after_s`: the helper calls again after it, with the
   box left as it is. Never call again once an attempt is recorded.
7. **Size bound** for every raw read: 200,000 bytes. **Python** is the lab PC's `python`, run for `python3`, always with `-I`.
8. **Notes.** `note.py "<text>"` appends one UTC-stamped line to `evidence\notes.txt`: one line per step, with the marks read
   before and after it.

Licensed mode only; Education mode = STOP.

## Build

The bundle is as Read this first, item 1, says, and is used as found. At step 0 the lab hashes the `bridge_mcp.exe` it will run: its
sha256 must equal `7086f1d0d7430e7dc1fefd6f69c0d4a14d717a2e06477b32935a1d3aa4fee1b8`, and `client.py` refuses on a difference.
Hash it again after the row: the same, else STOP. Each line cited here is at `a83ee02d`.

The row starts one `bridge_mcp` over stdio, through the relay, with the launch configuration `setup-r3.py` writes:
`BRIDGE_AGENT_ENABLE_WRITES=true` (the lab's standing word for synthetic books; no installed setting is touched),
`BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9003`, `BRIDGE_AGENT_DATA_DIR` the new
folder `agent-r3`, shown absent by a listing before the row. `BRIDGE_AGENT_ENABLE_BATCH_POST` and
`BRIDGE_AGENT_REDACTION` are absent. The client is the lab's stdio client of R0 to R2 (sha256 `0ed0bb03...`): no cancel,
no call limit under 10 minutes. Each call is made from the row folder as
`python -I -X utf8 call.py agent-run-r3 <label> <tool> <argument file>`.

## The files

`SHA256.txt` lists every file with bytes and sha256, and the source of each. Reused byte for byte: `LAB-COMMON.md`,
`.gitattributes`, `check_1108.py`, `check-request-ids.py`, `relay-keep.py`, `check-relay-keep.py`, `check-exchanges.py`,
`test-exchanges.py`, `requests/` marks, flags, ledgers and vouchers, `batches/r3.json` (R0's `r0.json`), five helpers
(`box.ps1`, `call.py`, `show.py`, `capwin2.ps1`, `front.ps1`), and two with placeholders (`placeholder-helpers/`).
- **Changed from R0's helpers:** `args1108.py`, `body.py`, `boxcheck.py`, `cap.py`, `chk.py`, `p1108s2.py`, `send.py`: only the row
  folder (`C:/bridge-lab/1108r3`) and the sheet folder (`lab/1108-r3`) in place of R0's, and for `send.py` the write-once
  rule also covering `e1-` files. Each file's first comment line says so, and gives R0's sha256.
- **New, not run on Windows:** `setup-r3.py` (reads the prior folder, writes only the new row folder and the launch
  file), `scripts/make_e1.py` (its `check` mode only reads; `make` was run once, on macOS) and `requests/e1-equal-first.utf16le.xml`
  (made by it; its shape was compared on macOS with the seed Journal's request of R0 to R2; it has not met Tally).
- **Placeholders** in `placeholder-helpers/note.py` and `c6prep.py` (pushed copies of the lab's own scripts, their machine
  strings replaced). The lab makes its copies **in the row folder, never in `lab\1108-r3`**, with `<LAB_DIR>` set to
  `C:/bridge-lab/1108r3` and `<Tally data folder>` to Tally's data folder (C6's Restore Data Path), and records in the notes the
  sha256 of each copy and the count of each replacement (`note.py`: 2 and 0; `c6prep.py`: 3 and 2). `c6prep.py <tag> <bkr|bks> <sha256> <bytes>`
  re-hashes the backup folder, writes `restore-plan-<tag>.txt` and the data folder's listing, and changes nothing else.

## Steps

One at a time. Every answer goes to a file. A file with no answer is a failed step, not an empty one. Each raw request is
sent from the row folder by `send.py <label> <request file>` (the answer is `answers\<label>-<request file>`) and each answer read by
`chk.py <name> <mode> <answer file>`, which runs `check_1108.py` and keeps `evidence\chk-<name>.txt` (it prints `FAIL` for exit 1 and `NOT READ`
for exit 2, and a miss is a STOP). A check's name is `<label>-marks` for marks and `<label>-<flags|ledgers|vouchers>-rows` for the others, and
`<label>-vdiff` or `<label>-ldiff` for a diff, as R0 to R2 named theirs. A write is sent only if the
marks answer read just before it held exactly one COMPANY row, THE COMPANY's, and no key was pressed in Tally since.

0. **Before anything is sent or keyed**, in this order, each output kept whole:
   - the exe's sha256, as Build says;
   - `python3 -I setup-r3.py` from this folder. EXPECT `files: ok N, FAIL 0`, the `BK-R:` line, and `setup: ok`;
   - from the row's `lab\1108-r3`: `python3 -I scripts\make_e1.py check` (EXPECT `e1 request: ok (1826 B)`);
     `python3 -I check-request-ids.py requests` (EXPECT `census: ok 4, skipped 1, FAIL 0`);
     `python3 -I check-relay-keep.py relay-keep.py` and `python3 -I test-exchanges.py relay-keep.py check-exchanges.py`
     (EXPECT `0 failed` each);
   - from the row folder: `python -I -X utf8 args1108.py fixed`, then the box self-test
     `python -I -X utf8 p1108s2.py selftest selftest-r3` (EXPECT its watch start line, no box seen, exit 3, no console output);
   - the two placeholder copies (Files), their sha256 and counts in the notes;
   - LAB-COMMON **C2**: a screenshot of the whole Tally window (a dialog or half-entered screen = STOP), `tasklist`, `netstat -ano`, `GET /status`;
   - load THE COMPANY alone; **m0** (`send.py m0 marks.utf16le.xml`, then `chk.py m0-marks marks m0-marks.utf16le.xml`): one COMPANY row,
     its GUID beginning with THE COMPANY's. Record V, M, Tally's release and edition, and the Gateway's current date. **m0 is whatever the
     lab finds**; it is the state the row returns to.
   - **Nothing listens on port 9003:** `netstat -ano -p tcp | findstr ":9003"`, whole output kept. A line ending `:9003`, or a netstat that does not run: STOP.

1. **START reads.** `send.py start flags.utf16le.xml`, `ledgers`, `vouchers`, each followed by `chk.py ... rows`. Record them (they are not
   graded: they are what the end reads are compared with); every GUID must begin with THE COMPANY's. Then `marks` again: V and M equal m0's.

2. **BK-S** (C5), into the new empty folder `C:\bridge-lab\1108r3\backups\bks\` (made by `mkdir`; a listing before shows it empty). Record its name, bytes
   and sha256; the listing after shows the one file. Then `marks` (label `bks`): EXPECT V and M equal m0's, else STOP.

3. **Restore BK-R** (C6), on the run coordinator's word. `c6prep.py r3r bkr f02e31d171d92e41cf2120bdcbdf50375383e461da64d8c273df16d8b9179e59 26485`
   (EXPECT its one-line equal-to-recorded output), shut THE COMPANY, restore by Difference 4 (screenshot before each `y`), load THE COMPANY alone.
   A screenshot of the whole window and one of the Gateway; no dialog. Then `ledgers`, `vouchers`, then `marks` last (labels `base`).
   **BASELINE EXPECT (R0 to R2's BASELINE):** the COMPANY row with V 1 and M 213; five ledgers, among them `A1 Closing Stock` under Stock-in-Hand,
   `A1 Capital` under Capital Account and `A2 Expense` under Indirect Expenses; one voucher, the seed Journal of 10-Apr-2025,
   `Cash` Dr 200.00 / `A1 Capital` Cr 200.00, narration "Row 1108-B cash leg", ids 1. Any other: STOP (BK-S is the way back).
   Record the Gateway's current date (R0 to R2 saw it read 10-Apr-2025 after this restore).

4. **E1.** Screenshot (`cap.py e1pre`). `send.py e1pre marks.utf16le.xml`: marks equal BASELINE's. Then, once, `send.py e1w e1-equal-first.utf16le.xml`
   (its request sha256 checked against `SHA256.txt` by `send.py`; `chk.py e1-import import e1w-e1-equal-first.utf16le.xml`). **EXPECT E-1.**
   Screenshot (`cap.py e1post`): any dialog = C7. Then `vouchers`, `ledgers`, `marks` (labels `e1post`), each checked; `chk.py e1-vdiff diff base-vouchers.utf16le.xml e1post-vouchers.utf16le.xml`.
   Record E1's GUID, MASTERID, ALTERID and VOUCHERNUMBER.

5. **The relay.** From the row folder, in a background task of its own: `python -I lab\1108-r3\relay-keep.py 9003 9001 C:\bridge-lab\1108r3\relay` (the script is in the copied sheet folder, not in the row folder's root). It
   stays up to step 8. It refuses a folder that is not empty. Record **r3-first**: the highest N among `relay\x-N.request` (0 if none).

6. **Start the server.** A listing shows `C:\bridge-lab\1108r3\agent-r3` absent. `python -I -X utf8 agent-run-r3\client.py C:\bridge-lab\1108r3\agent-run-r3 <exe path> 7086f1d0d7430e7dc1fefd6f69c0d4a14d717a2e06477b32935a1d3aa4fee1b8`, in a background task of its own, left up until step 7.7 closes it.
   Then, through `call.py`: `list_companies` (EXPECT THE COMPANY's GUID), `tally_status` (record release, licence, mode), `masters` with
   `kind=voucher_types` (record the Journal row's `series_numbering.method` and `sub_method`: EXPECT `automatic` and `auto_retain`; any other: STOP,
   this row alters no type). **mk-start**, marks on 9001: V 2, M 213, equal to step 4's last marks. Anything else: STOP.

7. **The batch.**
   1. `python -I -X utf8 args1108.py build r3`, then `build_import_xml` through `call.py` with `calls\args-build-r3.json`. Keep the whole answer;
      record the batch id, `voucher_count` (EXPECT 3) and every warning. A refusal: keep it and STOP. **EXPECT E-3.**
   2. With the helper: `python -I -X utf8 p1108s2.py r3 <batch id> 3 66 e1post-vouchers.utf16le.xml`. It runs the **mk-before** marks, starts
      the box watch (120 s), calls `post_import` once, checks the box if one shows, and (if the check passed) goes on to a capture, the
      **mk-after** marks, the vouchers and their diff, and `verify_import` once.
   3. Record each call's start and end in UTC, and, if a box showed, when and when Yes was clicked.
   4. Keep the post's whole answer. Record `error.code`, `error.preexisting_txn_ids`, `error.message`, `error.next_step`, and from
      `dispatch` (if present) `state`, `response_state`, `resent`, `dispatch.response.outcome`'s counters and `tally_line_errors`;
      `post_span_binding`; each voucher's status; whether `created` or `attempt_recorded` are present.
   5. **If no box showed** (the expected case): the helper ends after about two minutes with `HALT: the box check did not pass` and exit 1, having
      printed the post answer; its box log says `NO BOX seen within 120 s`. **That is the expected signal, not a STOP.** If it ends instead with a Python
      traceback at its read of the post's answer (a refusal's answer may not have the shape it reads; R0 to R2 never met one), wait until `box\r3-p1-box.txt`
      holds `NO BOX seen` and `box\r3-p1-check.out` exists (about two minutes), run `show.py` on the post's answer file by hand, and treat it the same way. Check that the post's answer is a
      refusal (E-4a). Then, by hand, in this order: `send.py r3post marks.utf16le.xml` and `chk.py r3post-marks marks r3post-marks.utf16le.xml`;
      `send.py r3post vouchers.utf16le.xml` and `chk.py r3post-vouchers rows r3post-vouchers.utf16le.xml` (the name the helper uses); `chk.py r3-vdiff diff e1post-vouchers.utf16le.xml r3post-vouchers.utf16le.xml`;
      then `verify_import` once on the batch id (`calls\args-verify-r3.json`, which the helper wrote) through `call.py`, label `r3-verify`. Whatever it answers
      is recorded whole (the batch was never posted: a refusal there is a result).
   6. **If a box showed and the check passed:** the helper did every part of item 2; no hand step is added. If the box showed and the check failed,
      the helper answered No: keep its text whole and STOP.
   7. `egress_log` through `call.py` with `calls\args-egress.json`. Close the client (an empty file in its `inbox` as R0 to R2 did), so `bridge_mcp` exits by itself; record its exit code.
   8. `check_1108.py rows` on the vouchers answer after the post: record each voucher's GUID, MASTERID, ALTERID, VOUCHERNUMBER, amount and narration.
      If an import was sent, `python -I -X utf8 body.py <N>` and `chk.py ... import` on the body written, N the relay's import number.

8. **The relay's check.** **r3-last**: the highest N, as in step 5. `python3 -I lab\1108-r3\check-exchanges.py C:\bridge-lab\1108r3\relay <r3-first + 1> <r3-last>`, whole output
   kept. EXPECT **E-X**. A FAIL or `NOT READ` is a pause. Stop the relay by stopping the background task it was started in. Hash the exe again.

9. **Restore BK-S** (C6), on the run coordinator's word, never before E-checks are recorded. `c6prep.py r3s bks <BK-S sha256> <BK-S bytes>`, shut, restore by Difference 4,
   load THE COMPANY alone. Then `ledgers`, `vouchers`, `marks` (labels `end`), and a `diff` of each against step 1's START read.
   EXPECT **E-END**. **The current date is not typed.** Record the Gateway's current date; if it differs from step 0's, report it and
   leave it, as the run coordinator rules.

10. **C10.** Shut THE COMPANY (Company > Shut, Enter on the one row). `tasklist`, `netstat` (one `tally.exe` owning 9001), `GET /status` 200, a screenshot with no company
    loaded. No XML request after the shut.

## EXPECT

Counters are from `chk.py ... import` on the answer file; marks from the `chk.py ... marks` outputs; the post from the saved
`agent-run-r3\answers` file of the call (read by `show.py` and in full). Confidence on every line: unverified, from the code at `a83ee02d`, unless a record is named.

| Id | Counter, mark and ids | Status and text | Graded from |
|---|---|---|---|
| E-BASE | step 3: V 1, M 213; 5 ledgers; one voucher (the seed) | as step 3 | `chk-base-marks.txt`, `chk-base-ledgers-rows.txt`, `chk-base-vouchers-rows.txt` (R0 to R2's BASELINE, partial: one run) |
| E-1 | E1: `CREATED 1`, every other counter 0, `LASTVCHID 2`, no `LINEERROR`; V 2, M 213; one voucher ADDED: 14-Apr-2025, Journal, `A2 Expense` Dr 21.00 / `A1 Capital` Cr 21.00, narration "Row 1108-P2 first", MASTERID, ALTERID and VOUCHERNUMBER 2 | none | `chk-e1-import.txt`, `chk-e1post-marks.txt`, `chk-e1-vdiff.txt` |
| E-3 | the build is not refused; `voucher_count` 3; the first warning says `1 of 3 rows of this batch already match vouchers in the book as this build read it (bridge_txn_id: b1108-p2-1)` and that `post_import` would refuse the batch as `import_preexisting_identity` before any approval (`rows_in_book_warning`) | the batch is saved, nothing is sent | the build's answer file |
| E-4a | **the expected case.** The post is refused: `error.code` `import_preexisting_identity`, `error.preexisting_txn_ids` exactly `["b1108-p2-1"]`, `error.message` beginning "Nothing was sent". No attempt recorded; no `dispatch`. mk-before = mk-after = V 2, M 213. The vouchers diff against step 4's read: no row ADDED, REMOVED or CHANGED | no voucher status in the post; no box: the box log reads `NO BOX seen within 120 s`; the helper halts as 7.5 says | the post's answer; `box\r3-p1-box.txt`, `box\r3-p1-check.out`; `chk-r3pre-marks.txt`, `chk-r3post-marks.txt`, `chk-r3-vdiff.txt` |
| E-X (with E-4a) | `check-exchanges.py` for the row: exit 0, `FAIL 0`, a count of `r3-last − r3-first`, **`imports 0`**: the relay saw no import | | the check's whole output; `relay\relay.jsonl` |
| E-V | `verify_import` on the unposted batch (record, not graded): by the code, `b1108-p2-1` is found by content at E1's ids, `b1108-p2-2` and `b1108-p2-3` read `not_found`. Any other reading, or a refusal, is recorded whole | | the `r3-verify` answer file |
| E-4b | **only if a box shows.** The check passes (`ALL PASS`), Yes once, one call, no second box. Tally's answer `CREATED 2`, `EXCEPTIONS 1`, `LASTVCHID 4`, every other counter 0, one `LINEERROR` naming `A1 Closing Stock` (recorded only); V 4, M 213; two vouchers ADDED beyond E1, `Row 1108-P2 first` 21.00 and `Row 1108-P2 third` 23.00, each id and number one above the last (3 and 4); `b1108-p2-2` takes none | statuses recorded for each voucher in the post and in `verify_import` (R0's were `matching_content_observed` for the two created, `sent_not_attributed` for the refused; here, **record which row `b1108-p2-1` points at (E1's ids or the created voucher's) and whether it reads `duplicate_fingerprint`**); `post_span_binding` and `error.code` as R0 (`span_counters_not_clean`, `import_reconciliation_required`) or recorded as different | `chk-r3-import*.txt`, the post's and `verify_import`'s answers, `chk-r3post-vouchers.txt` |
| E-END | step 9: V, M, the ledgers and the vouchers equal step 1's START reads (m0), no row ADDED, REMOVED or CHANGED | | the `end` `chk` outputs |

## STOP RULES

- An exe sha256 that is not as Build says, or changes during the row; a file that differs from `SHA256.txt`, or `setup-r3.py` not ending `setup: ok`;
  `make_e1.py check`, `check-request-ids.py`, `check-relay-keep.py` or `test-exchanges.py` not as step 0 EXPECTs; a `box.ps1` self-test not as EXPECTed.
- A missing prior folder, client or BK-R; a BK-R that differs from its recorded bytes and sha256.
- C7 and C8. Any dialog on Tally's screen after a write or a restore: screenshot it, press no key, report.
- A box the helper does not pass, or any decline.
- **A refusal by `build_import_xml`; or by `post_import` with any code other than `import_preexisting_identity`** (E-4a is the one refusal this row
  expects, and it is a result, not a stop); a refusal naming other ids than `b1108-p2-1`.
- The raw write E1 not answering `CREATED 1` with every other counter 0, or an answer that does not parse or lacks a counter.
- A Journal type that is not `automatic` with `auto_retain`.
- A marks answer with other than one COMPANY row; a GUID that does not begin with THE COMPANY's.
- A mark that moves in a step that sends no write, no post and no restore, a backup step included; mk-start that differs from step 4's last marks;
  an end read that is not E-END.
- `agent-r3` not absent before the row; port 9003 in use.
- Any call with no answer in 10 minutes. Education mode.
- Anything that ends a process, by name or number; no step does.

After a stop, nothing more is sent or keyed (C8). The book then holds what was written before it; the way back is BK-S (and BK-R to the baseline),
each restored only on the run coordinator's word.

## Decides

Recorded by the coordinator, each with its confidence. **What follows says what a reading would bear on option B; it does not decide it.**

| What the row shows | Reading |
|---|---|
| E-1, E-3, E-4a, E-X as written | On this build, a voucher equal in content to a batch voucher and already in the book stops the batch before the approval window: nothing is sent and no partial landing happens. So the substitution this row asks about cannot arise from a voucher that was in the book before the post (partial: one run, one book, position (b) only). It bears on option B this way: that case needs no rule of naming, because it is refused earlier; what stays unmeasured is an equal voucher that enters the book after the post's checks (a second writer), and position (a), which is not run (raw: refused by Tally in A1; keyed: not measured; and the same refusal at the gate would apply). |
| E-4a with a different code or other ids | The gate does not read the book as the code says. Record the whole answer. The question is not answered and the row is not a measure of option B. |
| E-3 without the warning, and E-4a as written | The post's gate holds but the build's early warning does not (#2152). Record it; it is not about option B. |
| E-4b: the box shows and the post lands in part | The gate did not fire, and the readback is measured. If `b1108-p2-2` is the only sent voucher with no row, and `b1108-p2-1` reads against E1's ids or as a duplicate, then in position (b) the equal voucher does not change which sent voucher is unmatched: for one run, the unmatched row is the one Tally refused. If `b1108-p2-1` reads against E1's ids and a created voucher has no sent voucher pointing at it, or `b1108-p2-2` reads as found, an equal voucher stood in for a batch voucher: that would bear against naming the unmatched row from the readback alone, and for any naming rule that needs the post's own span. |
| E-4b with the voucher mark stepping by other than 2 | A voucher entered during the post: record whether anything else wrote to the book. |
| E-X not met | The row's readback is not whole. A rerun needs a new sheet. |

## Limits

- One synthetic book, one build (`a83ee02d`, 0.7.0), one TallyPrime release (7.2, single user), one run of one position.
- **Position (a) is not run**: a raw import of the refused voucher's content is refused by Tally (A1, one run), and keying it by hand is not measured. Whether a voucher
  equal to a refused one can exist depends on the fault; no record gives one that Tally refuses only inside a batch. At the post's gate, (a) and (b) are treated alike.
- The equal voucher is posted by a raw import, not keyed by hand. The post's gate reads the book's content, not how a voucher got there, but this row does not
  measure a hand-keyed one. Its narration equals the batch voucher's; a narration that differs is not measured (the gate's match ignores narration).
- No second writer: an equal voucher entering the book between the post's checks and its readback is not measured.
- Two refusals in one post, a split class of twins, another fault kind, a book not integrated with inventory and TallyPrime 7.1 are not measured (as R0 to R2).
- The EXPECTs are from reading the code; none has met Tally. The helpers' changes are folder names only, and the new scripts have not run on Windows.

## Open points for the run coordinator

1. Whether the prior row folder, BK-R and the client of R0 to R2 are still on the lab PC as `kept-local.txt` of the answers lists them (`setup-r3.py` STOPs if not).
2. The path of the installed `bridge_mcp.exe` the 2214 row ran, and the argument order of R0 to R2's `client.py` (it is not on any branch; this sheet gives the order of the 2214 sheet's client).
3. Tally's data folder, for `c6prep.py`'s placeholder.
4. Whether position (a) is wanted at all: by hand (one keyed Journal with a Stock-in-Hand ledger, if Tally's screen lets it be keyed), or with another fault that Tally refuses only inside a batch.

## Evidence

On a new branch, `lab/1108-r3-equal-voucher-answers`, with `SHA256.txt`:
- every raw answer whole with its size and sha256, and every `check_1108.py` output;
- the relay folder whole (`relay.jsonl`, every `x-N.request` and `.answer`, any `import-N`) and `check-exchanges.py`'s whole output;
- every tool answer whole, the box helper's log and check files, each post's times;
- BK-S's and BK-R's names, bytes and sha256, each `restore-plan-<tag>.txt` and each data-folder listing;
- `agent-r3`'s file list by name, size and sha256 (the files stay on the lab PC);
- a `RESULTS.md` with the EXPECT table filled in, each Id marked confirmed, refuted or not reached, with its first line naming any pause or stop, and
  the Decides row that applies, with its confidence;
- the notes file with UTC times and the placeholder replacements, and the sha256 of every helper copy run.

The evidence shows THE COMPANY's GUID as `<company guid>` and its number as `<company number>`, with `substitutions.txt` naming each change, as R0 to R2 did.
Screens stay on the lab PC, listed by name, size and sha256. No machine path or personal name in anything pushed. Send the sha to the run coordinator before pushing,
and push only after the pre-push scan and the run coordinator's yes.

## Keep

The answers branch is `lab/1108-r3-equal-voucher-answers`: the evidence above, the notes, the three restore plans' records, the relay folder and every answer, `RESULTS.md` graded
verified, partial or unverified (one run of one position is partial at best). BK-S and BK-R stay on the lab PC, listed by hash. Nothing of this sheet is merged.
