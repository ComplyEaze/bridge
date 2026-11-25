# Row 2162-PA: the post of the admitted invoice in the grown state

For #2162 (protocol reference §12a.31 and §12a.32, both in this tree and both PARTIAL: the W1 walk of both service codes' voucher lists grows the rate answer, and Shut Company with a reload clears it). Row 2162-RC (sheet `lab/2162-rc-sheet` at 5242ca25; answers `lab/2162-rc-answers` at 20be573e) saw BUILD 6f541ae4 admit the test invoice in Tally's grown rate-answer state; it built and did not post. This row posts that invoice, once, in the same grown state, through the build's whole post path, and reads the invoice back raw twice after the post. One sitting, one synthetic book, one backup (BK1, step 1), one post of one voucher (step 4), and a restore of BK1 at the end (step 5) only on the run coordinator's explicit RESTORE word in the GO. Written 25 Nov 2026 on a branch cut at master 2ad8f78c. Until the row runs, every EXPECT is **unverified** unless its source says otherwise.

**How an EXPECT is marked.** **(code `file:line`)** is a rule read in this tree, whose `src-tauri` is byte-equal to commit 6f541ae4's (checked when this sheet was written: `git diff 6f541ae4 2ad8f78c -- src-tauri` prints nothing), so every code line cited is BUILD 6f541ae4's; **(lab `row`)** is an earlier observation; **(not shown)** is the thing this sitting measures. A `rbreq.py`, `rbcmp.py`, `ratecmp.py`, `count19.py`, `calls.py`, `grade05.py` or `pa.py` exit 2 is a failed read, never "nothing found", "no row" or "no difference". The line numbers inside the comments of byte-equal helpers are their own trees', not this one's.

**The state is graded by bytes**, as in row 2162-RC: `ratecmp.py` is that row's, byte for byte, and in the grown state prints `SHAPE unmeasured` for the two service ledgers; that is the expected reading, not a finding. The two states (lab 2162-RW; lab 2162-RX; lab 2162-RC):

- **plain**: 29,114 bytes, sha256 `a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62`;
- **grown**: 30,626 bytes, sha256 `d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257`.

## The questions and the one variable per arm

- **Q1.** In the grown state, does the build's full post path go through: the re-read of the invoice before the approval box, the box, the re-reads after the answer and the queue's own re-read (`admit_queued_invoice`, code `src-tauri/src/agent_import_invoice.rs:1326-1352`), the import, and the build's own read-back, ending `posted_verified`?
- **Q2.** (#2162, item 2, "The status read straight after a post".) Are the GST status flags the build reads straight after the post (its read-back request, code `src-tauri/src/agent_import_invoice_wire.rs:750-768`, graded by `status_of_voucher`, code `src-tauri/src/agent_gst_status.rs:115`) equal to those of the same read-back request sent raw at about +5 s and about +60 s after the post?

**The raw read-back request, and its limits.** The build keeps no bytes of its read-back request or answer: its evidence folds each request's and answer's hash into one record and keeps no body (code `src-tauri/src/agent.rs:4136-4154`), and neither the post answer nor `verify_import`'s carries the request. So this row does not send the build's own bytes; it sends the closest raw read-back, rendered by `rbreq.py` from the same inputs, by a port of the same function:

- the build's read-back (`read_back_sales_invoice`, code `src-tauri/src/agent_import_invoice.rs:1929-2010`) sends `invoice_readback_read` (code `src-tauri/src/agent_read_profiles.rs:712-721`) with the company's name, the voucher type GUID the build observed for the invoice, the invoice number, and the financial year of the invoice date (`financial_year_window`, code `src-tauri/src/agent_import_invoice.rs:2232-2241`); the request is `render_invoice_readback_request` (code `src-tauri/src/agent_import_invoice_wire.rs:750-768`);
- `rbreq.py` takes the type GUID, number and date from the build's own journal record of the batch, `data-pa/agent-import-ledger.jsonl` (written at the build, code `src-tauri/src/agent_import.rs:2089-2092`; the voucher's fields, code `src-tauri/src/agent_import.rs:215-236`, `src-tauri/src/agent_import_invoice.rs:78-80`), parsed as JSON, and refuses (exit 1, nothing written) unless the type GUID is the book's GUID followed by `-000000ce` (the BRIDGE Sales type: lab 2162-S2's read-back captures; code fixtures `src-tauri/src/agent_import_invoice_tests.rs:1905` and `:3797`), the number `BP/26-27/0074` and the date `20260801`;
- `rbreq.py --selftest` renders the code's own fixed inputs and must reproduce the code's pinned hash of that request (code `src-tauri/src/agent_import_request_bytes_tests.rs:182-183`, inputs at `:74`, `:292-294`, `:444-446`); the file is BOM-prefixed UTF-16LE with no added newline, as the build encodes it (code `src-tauri/crates/bridge-tally-protocol/src/text_encoding.rs:40-48`);
- EXPECT for this invoice `rb.xml: 2332 bytes, sha256 5a72bf7d4243238b4e7b81ccee66edaa79cad950caee414865791e6cd3cb9547` (not shown: computed when this sheet was written from these inputs; it rests on the type GUID's suffix).

Its limits, each stated again under Not covered: (1) equality with the build's bytes is inferred from the pinned self-test and the same inputs, not read from the build; (2) only the read-back request is repeated: the build sends a marks read before it and, when it finds no difference, one read per month of the financial year after it (code `src-tauri/src/agent_import_invoice.rs:2002-2008`), and neither is repeated raw; (3) what the build's own read stated is not kept, so it is inferred from the build's verdict: `posted_verified` leaves one value for each status field (`rbcmp.py`'s header, from code `src-tauri/src/agent_import_invoice.rs:1555-1557` and `:1885-1918`, `src-tauri/src/agent_gst_status.rs:91-159`; partial); (4) the build's read happens inside the post call, before its answer, at a time the answer does not give; `pa.py` times +5 s and +60 s from the moment it read the final answer whole; (5) the raw reads are sent while the exe is running and idle (a stated departure from row 2162-RC's rule, under Launch).

| arm | the state | the act | EXPECT |
|---|---|---|---|
| P0 | step 1's load, plain | nothing launched; two raw rate reads | both plain and byte-equal (lab 2162-RC) |
| G | Shut Company and a load, plain (two reads), then the W1 walk | nothing launched | the read after the walk grown (lab 2162-RC) |
| PA | G's grown state, no act since G's read | BUILD 6f541ae4 (launch LPA): `05` builds; a raw rate read (`PA-pre`) just before the post; then one post, its box answered Yes; raw read-backs at +5 s and +60 s; `verify_import` | `05` admitted, `voucher_count 1` (lab 2162-RC); `PA-pre` grown, 30,626 bytes (not shown); the box's text the one the build makes for this invoice; `posted_verified`, created 1, V 68 to 69 (not shown); `ITEM 2 EQUAL` (not shown) |
| R | after LPA has closed, only on RESTORE | C6 restore of BK1 | marks back to V 68, M 245; the raw read-back finds no row (not shown) |

**The one variable.** Q1 is measured by the post itself; its premise is G's grown state, graded by bytes before the launch, and P0 (reads only) shows the sitting's first load is plain, so that the growth is the walk's. No post is made in the plain state: the earlier post of this book on this path (lab 2162-SALES-Q, `posted_verified` on its first call) is cited for the shape of its answers only; its build and its state differ, so it is no control. Q2's one variable is time: the same request, the same state, no act between, read by the build inside the call and raw at +5 s and +60 s.

**Order.** P0 on step 1's load; G grows the state once; PA follows in it with no Shut Company, load, key or screen act between G's read and the post, and reads the rate state once more (`PA-pre`, part 6c) after its build and before its post, so the post's premise is measured at the post, not only at G; R last, after the launch has closed and only on RESTORE.

## Book

`BRIDGE PILOT LAB` (synthetic), its GUID pinned in `count19.py`, `calls.py` and `boxcheck.py`, loaded alone on TallyPrime 7.2 Silver. `LAB-COMMON.md` v3, byte-equal to row 2162-RC's, applies in full; the stricter text wins, except where this sheet names a departure.

- **Pinned by GUID.** `02-list_companies.json` must name it. Any other GUID: STOP, whatever the displayed name.
- **Starting state.** EXPECT ALTVCHID 68, ALTMSTID 245, current date 15-Sep-2026, period 1-Apr-26 to 31-Mar-27 (lab 2162-RC). Any difference: STOP before the backup or the launch.
- **BK1 is the way back.** Taken at step 1 in the new empty `C:\bridge-lab\2162-pa\backup-1` by C5; restored at step 5 by C6, only on the run coordinator's explicit RESTORE word in the GO. **Only the GO's RESTORE counts** (the run coordinator has said this sitting's GO will hold it). A GO without it: R is not run, the book keeps the posted voucher, and step 6 closes it so.
- **The invoice.** `BP/26-27/0074` of 2026-08-01, two service ledgers, six entries: the call file `05-build-0074-0801.json`, byte-equal to row 2162-RC's, which no row has posted (lab 2162-RC). Record the release shown on Tally's screen.

## The build, named by commit

**BUILD 6f541ae4.** Already on the lab disk: row 2162-RC's step 0 unpacked it, and that row's launches ran it (lab 2162-RC). Nothing is downloaded, installed, copied, moved or built.

COMMIT: `6f541ae437079c7a06c85f51afde26dcd9374187`  EXE: `C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe`  EXE SHA256: `9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc`

Step 0 and the launch's part 2 check that hash; any other: STOP, and nothing is launched.

**The screen tool's grant.** That exe path is not on the screen tool's grant list (row 2569-S's step 0 list; row 2569-S, answers `lab/2569-s-answers` at f0bfdb3e, saw one screenshot by the tool minimise the window of an ungranted `bridge_mcp.exe`). Step 0 reads the grant list and, if the path is absent, asks for its grant as a recorded step. Granted or not, this row takes no screen-tool screenshot while the exe runs: the approval box is read with the row's own window reader, `boxstate.ps1` (byte-equal to row 2569-S's), and answered by `box.ps1`'s own click, never by the screen tool. `box.ps1` does take its own GDI copies of the screen, one of them before the window read (Launch, departure 4). The grant state is recorded, not controlled.

## Shell

Every command below is typed in **Git Bash**, from `/c/bridge-lab/2162-pa`, exactly as written: each line begins `cd /c/bridge-lab/2162-pa && ` and depends on no shell variable or folder set by an earlier line (a variable set and used inside one line, as `G=` and `B=` below, is part of that line). Two lines are not typed at the prompt: the client start line (part 4) and the post line (part 7), each started whole as a background run of the operator's own shell tool. The interpreter is `python` (3.10) with `-I`, and `-X utf8` for every helper that prints; `python3` is the Store alias and is never used (lab 2162-RX, deviation 1). Every helper's output is written to a file under `evidence/` with its exit code and then printed whole. A line that begins `#` is an instruction, not typed. `tasklist` and `netstat` are typed in Git Bash with C2's commands and arguments, a departure from C2's "from cmd.exe" that row 2162-RC also took; step 0's `tally.exe` line is their proof. A wait for a file is a loop of the line given, typed about every 5 s, at most the number of times the step names (lab 2162-RC, deviation 1). `pa.py` starts Windows PowerShell 5.1 itself for `box.ps1` and `boxstate.ps1`; nothing else in the row uses PowerShell but step 0's parse check.

**`local/` is the lab's own folder.** Everything the lab adds that is not a sheet file and not a named output goes in `local/`: its own notes drafts, `rates.sha256`, `rb.sha256`, the BK1 and restore listings. `check_sheet.py` skips `local/`, `evidence/`, `box/`, `backup-1/`, `scratch/`, `run-pa/` and `data-pa/` by name, and the top-level files the sitting writes (`a-<label>-<request>`, `log-<label>.txt`, `rates.xml`, `rb.xml`, `kept-local.txt`, `restore-plan.txt`), so it can be run in the working folder at any time; every other file outside `SHA256.txt` is an error.

## Launch

One launch, LPA, by `client.py` (byte-equal to row 2162-RC's) with its own new run and data folders. Its launch file is row 2162-RC's grown-state launch file with one value changed, the data folder. It has exactly the five settings: writes enabled (the post is refused otherwise: `posting_from_env`, code `src-tauri/src/agent.rs:714-716`; the tool gates, code `src-tauri/src/agent.rs:3499-3518`), terms accepted, host `127.0.0.1`, port `9001`, and its own new empty data folder. The four forbidden override keys are absent; `client.py` refuses them. No script writes a launch file.

| launch | arm | build | run folder | data folder |
|---|---|---|---|---|
| LPA | PA | BUILD 6f541ae4 | `run-pa` | `data-pa` |

**The launch procedure** is written out whole at step 4, in twelve parts, with this launch's folders, exe, hash and call names in every line. Each EXPECT is checked before the next line. `client.py` itself ends its own server if that server has not exited 30 s after the client closed its input (`client.py:111-113`), writing `server did not exit within 30 s of stdin close; killed`, which is a STOP and report.

**Departures from row 2162-RC's launch rules, each needed by Q2 and named here.** (1) Raw requests while the exe runs: part 6c's one rate read (`PA-pre`, between `05` and the post) and `pa.py`'s two raw read-backs are sent while LPA is running, idle between calls (no call is in flight: the last call's answer has been read whole, and the next call is placed only after the raw answer is in, or after `pa exit`). Row 2162-RC sent no raw request while an exe ran. (2) C3's screenshot after a raw send is not taken for those three sends; one whole-window screenshot is taken after the launch has closed (part 12) instead. (3) Nothing acts on the screen from the post line until `pa exit`: no screenshot, zoom, click, key, `request_access` or `list_granted_applications`, and nobody at the PC (as row 2569-S, "From the post call until R3 is saved"). The run coordinator asks the owner, if present, to keep hands off until part 12 ends. This is also the only thing that keeps another window from covering the box between the guard's read and `box.ps1`'s click, a few seconds later: `box.ps1` (byte-equal) clicks the Yes button's centre without reading the box again, so that is relied on, not checked (Not covered). (4) `box.ps1`'s own screen copies come before the window read: it copies the screen by GDI in its own process (not the screen tool, whose screenshot is what minimised an ungranted window in row 2569-S) as `open` after it has logged the box and before it logs `WAITING` (box.ps1:67), so before the guard's `boxstate.ps1` read; and as `nobox` (:60), `gone` (:71) or `nobutton` (:77) only on those exits. The window read therefore comes before any screen-tool screenshot, not before every screen copy. Row 2162-SALES-Q's box was clicked after the same `open` copy (lab 2162-SALES-Q). The run coordinator has accepted departures (1) to (3), counts part 6c's read under (1), and named (4).

## The tool sequence, from the code

The MCP calls, in order (code `file:line` at 6f541ae4; the e2e test of this path is `an_invoice_built_and_posted_on_the_pilot_labs_answers_is_verified`, `src-tauri/src/agent_import_post_e2e_tests.rs:7822-8000`: one invoice built on the pilot lab's answers, posted with the approval answered Yes (`:7842`), read back, `verification_status` `posted_verified` (`:7855`), `counts.posted_verified` 1 (`:7858`), the voucher `posted_verified` (`:7861`), the dispatch counters `created` 1 and every other 0 (`:7878-7900`), and the marks V +1, M unchanged (`:7994-7997`)):

1. `tools/list` (`01-list.json`): EXPECT 37 tools, none of this row's missing (lab 2162-RC).
2. `list_companies` (`02-list_companies.json`): one company, the pinned GUID.
3. `ledger_masters` (`03-marks.json`): M 245, V 68, reused False (lab 2162-RC).
4. `build_import_xml` (`05-build-0074-0801.json`): admitted, `voucher_count 1`; the build writes the batch file and appends the batch to its journal (code `src-tauri/src/agent_import.rs:2066-2092`).
5. `post_import` with `company_guid` and `batch_id` only (dispatch, code `src-tauri/src/agent.rs:3581`), written by `pa.py`. Inside the call, in this order:
   - the re-read before the box: `recheck_sales_invoice` (code `src-tauri/src/agent_import_post.rs:1059-1073`; defined `src-tauri/src/agent_import_invoice.rs:1159-1196`). A refusal here gives the judge's first code, or `import_invoice_masters_changed`, and **no box opens**;
   - the box: `ask_and_hold` (code `src-tauri/src/agent_import_post.rs:565-633`); title `ComplyEaze Bridge — post this voucher?` (code `src-tauri/src/tally/approved_import.rs:3413`); text the answer text, a blank line, then the invoice text (code `src-tauri/src/tally/approved_import.rs:2628`; the invoice text `invoice_review_preview`, code `src-tauri/src/agent_import_post.rs:3589`); closed by the build after `POST_DIALOG_LIMIT`, 120 s (code `src-tauri/src/tally/approved_import.rs:578`). A call that returns while the box is up answers `approval.state` `pending` with `retry_after_s` 15 (code `src-tauri/src/agent_import_approval.rs:107`) and `dialog_remaining_s`: call again after `retry_after_s`. A Yes that came too late in the call for the post to fit in it answers `approved` (code `src-tauri/src/agent_import_post.rs:611-633`; its `next_step`, `:470-477`): call again at once. `pa.py` makes at most four calls;
   - after Yes: the ledger catalogue read and `recheck_sales_invoice` again (code `src-tauri/src/agent_import_post.rs:1228-1251`), then `take_for_dispatch` (`:1265-1266`), which only redeems the approval. The queue's re-read `admit_queued_invoice` (code `src-tauri/src/agent_import_invoice.rs:1326-1352`) runs later, at `src-tauri/src/agent_import_post.rs:2774` inside `recheck_voucher_queue` (`:1891`), which `post_approved_import` (code `src-tauri/src/tally/runtime.rs:3507`) runs as its queue check (the call, `src-tauri/src/agent_import_post.rs:1313-1324`), after the endpoint's dispatch lease (`:1279-1282`) and the post's intent. A refusal there surfaces the judge's code (code `src-tauri/src/agent_import_post.rs:1472-1491`) or `import_invoice_masters_changed` (`:1988`): the same codes as the re-read after the answer, so the two are not told apart;
   - the import, then the read-back: `read_back_sales_invoice` (code `src-tauri/src/agent_import_invoice.rs:1929-2010`). It parses the direct `VOUCHER` children of `COLLECTION` (code `src-tauri/src/agent_import_invoice_wire.rs:126-131`), a field standing twice being an error (`:47-57`; fields `:1011-1041`); several rows answer `invoice_number_not_unique`, none `invoice_not_found`, an unreadable answer `invoice_readback_unreadable` (code `src-tauri/src/agent_import_invoice.rs:2103-2125`); it wants ISINVOICE Yes, ISCANCELLED No and ISOPTIONAL No (`:1555-1557`) and no status difference (`gst_status_differences`, `:1885-1918`; codes `:1859-1865`); a difference marks the voucher `posted_divergent` with the codes (code `src-tauri/src/agent_import_verification.rs:994-1016`).
6. `verify_import` with `company_guid` and `batch_id` (`run-pa/inbox/08-verify.json`, written by `calls.py verify`; dispatch, code `src-tauri/src/agent.rs:3607`; `src-tauri/src/agent_import.rs:2366-2377`, the invoice read back again at `:2725-2742`): EXPECT `verification_status posted_verified` (lab 2162-SALES-Q).
7. `ledger_masters` (`07-marks.json`): EXPECT M 245, V 69, reused False (lab 2162-SALES-Q: V 68 to 69, M 245 unchanged across its post).

`acknowledge_post_review` is not called: it acknowledges a post in doubt (code `src-tauri/src/agent_import_ack.rs:1025-1060`), and a `posted_verified` post is not one. No other call file is put in.

## The box: read, checked, answered

All of it runs inside `pa.py`, started by the one post line, so nothing is typed while the box is up.

- `box.ps1` (byte-equal to row 2162-SALES-Q's) watches for the box from 2 s before the post call, for up to 120 s. When it sees it, it logs `BOX SEEN` with the box's process, title and class, its rectangle, the screen's size and the box's z-order place, every child window's class, rectangle and text, takes its own GDI copy of the screen (`open`, box.ps1:67: by GDI in its own process, not the screen tool), and only then logs `WAITING`. Its log is `box/pa1-box.txt`.
- **The window read, before any screen-tool screenshot.** It follows `box.ps1`'s own GDI copy `open` (above; Launch, departure 4). At `WAITING`, `pa.py`'s guard runs `boxstate.ps1` (byte-equal to row 2569-S's reader) on the box's process into `box/pa1-R1.txt`, and passes only when that read exits 0 and holds exactly one box line (a `WINDOW` line whose name starts `[ComplyEaze Bridge` and whose class is `#32770`, the filter of row 2569-S) with `iconic=False` and `visible=True`, the box and its one Yes button lie inside the screen, and the box is first in the z-order. Its lines go to `box/pa1-guard.txt`.
- **The text check.** Only a passed guard starts `boxcheck.py` (row 2162-SALES-Q's, changed to write Yes only). It checks the box's whole text, line by line and in order, against the text the build makes for this invoice, worked out from the build answer and its saved batch file (its header lists every line and its code source). Every check passed: it writes Yes to `box/pa1.answer.tmp` and renames it to `box/pa1.answer` (whole, so `box.ps1` never reads it empty), and its record `box/pa1-check.out` ends `answer file written: Yes`. Anything else, the guard included: **no answer file is written, nothing is pressed**, and the box is left to the build's own 120 s limit; the post then answers a refusal, recorded whole, and that is a STOP.
- **The answer.** On the answer file, `box.ps1` clicks the box's Yes button by the mouse at its centre and logs `CLICKED Yes` and whether the box is still visible after.
- **Where No can be pressed.** (a) A failed guard or check: none; no answer file is written, and nothing is pressed while the box is visible before 150 s. (b) `box.ps1`'s own No at 150 s: it is started with `-AnswerBy 150` (kept at 150 by the run coordinator's word), past the build's limit, and with no answer file by then it presses No (box.ps1:75), but only on a box still visible at 150 s, 30 s after the build must have closed it (code `src-tauri/src/tally/approved_import.rs:578`). (c) By code, one more: `box.ps1` reads the answer file as soon as it exists (box.ps1:72), and any text but Yes or No, an empty read included, is pressed as No at once (:75). It is not reached here: only `boxcheck.py` writes that file, only with Yes, written under another name and renamed in whole (boxcheck.py:70-73), and `pa.py` refuses to start if the file is already there (pa.py:50-52). A box that closed by the build's limit ends `box.ps1` with exit 4, `box no longer visible before any click`, with nothing pressed.

## Raw encoding and bounded output

`run.py` is row 2162-RC's, byte for byte: `Content-Type: text/xml; charset=utf-16`, `curl.exe --data-binary` (`run.py:22-23`), each answer written whole to `a-<label>-<request>`; a curl failure, an empty answer, `Unknown Request` or a body over 2,000,000 bytes stops it. It records curl's exit code and the answer's size, not the HTTP status line or Content-Length that C3 names; as in row 2162-RC, that part of C3 is not asked for. `rates.xml` (from `ratereq.py`) and `rb.xml` (from `rbreq.py`) are BOM-prefixed UTF-16LE with no added newline; each static request's hash is checked against `SHA256.txt`, `rates.xml`'s against `local/rates.sha256` and `rb.xml`'s against `local/rb.sha256`, on the line of the send (by `pa.py` itself before each of its two sends). Never pipe an answer through a truncating reader.

`rbcmp.py` prints no value but the status fields, DATE, VOUCHERNUMBER, VOUCHERTYPENAME and ALTERID, a GUID only as equal or not, and at most 60 structural-diff lines with the count of the rest. `ratecmp.py`, `grade05.py`, `calls.py` and `client.py` keep their bounds (row 2162-RC). `pa.py` prints the box log's stamped lines only, cut at 220 characters, never a child's text or the company-number line.

## What stays on the lab PC

As in row 2162-RC, keep these local and list each in `kept-local.txt` by name, bytes and SHA-256:

- BK1 (`TDBK1800_<company number>.001`), every backup and restore listing, and `restore-plan.txt`; quote no company number in the notes.
- Every saved Bridge answer and the server error log until scanned (answers can carry `company_number`; evidence copies replace it with `redacted`). The `01` answer stays local; its grading line is quoted.
- `data-pa/`: the batch file and the journal name the book's company.
- `box/`: the box log, its captures and `box/pa1-R1.txt` hold the box's text, which holds the company number; `pa.py`'s and the replay's printed lines go to evidence, the files do not.
- The raw read-back answers (`a-R5-rb.xml`, `a-R60-rb.xml`, `a-PA-post-rb.xml`, `a-restored-rb.xml`): they hold the party's GSTIN (code `src-tauri/src/agent_import_invoice_wire.rs:745-749`); `rbcmp.py`'s lines go to evidence.
- Every screenshot, by name, bytes and SHA-256 only.
- Any file the pre-evidence scan finds holding a company number, a user name, a home path or data outside the synthetic lab family.

## Files

Byte-equal copies of row 2162-RC's sheet (`lab/2162-rc-sheet` at 5242ca25, folder `lab/2162-rc/`): `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml`, `run.py`, `client.py`, `calls.py`, `count19.py`, `ratereq.py`, `ratecmp.py`, `selftest-rate.py`, `guids.xml`, `pilot-lab-ledger-rates.utf16le.xml`, `grade05.py`, `selftest/05-admitted.json`, `selftest/05-refused.json`, and the five call files under `calls/`.

Byte-equal copy of row 2569-S's sheet (`lab/2569-s-sheet` at cd80f722, folder `lab/2569-s/`): `boxstate.ps1`.

Byte-equal copies of row 2162-SALES-Q's sheet (`lab/2162-sales-q-sheet` at 3bd08d02, folder `lab/2162-sales-q/`): `box.ps1`, and `template-import-0010.utf16le.xml` (the fixture of `selftest-box.py`'s cases).

Byte-equal copy of row 2162-S2's answers (`lab/2162-s2-answers` at 64f47017): `selftest/rb-0046.xml`, its first read-back answer, used only by `rbcmp.py --selftest`.

Changed for this row:

- `launch/LPA-launch-env.json`: row 2162-RC's grown-state launch file with its data folder replaced by this row's.
- `boxcheck.py` and `selftest-box.py`: row 2162-SALES-Q's, with this row's paths, and with no answer file written wherever that row wrote No (its self-test wants `(none)` there). Row 2162-SALES-Q's `box.ps1` pressed the No it was given; this row presses nothing on a failed check.
- `check_sheet.py`: row 2162-RC's checks, with this row's arms, launch, skipped names, the post line once, and three held lines once each: `pa.py`'s box deadline and guard line and `boxcheck.py`'s Yes-only answer.
- `RUN-SHEET.md` and `SHA256.txt`.

New: `rbreq.py` (renders `rb.xml`; above), `rbcmp.py` (reads raw read-back answers as the build reads them, and grades Q2 against the post answer), and `pa.py`, in the shape of row 2162-SALES-Q's pg.py helper: the post call, the box guard, the timed raw reads. Row 2569-S's boxwatch.ps1, postcall.py and ops.ps1 helpers are not used: its watch read the box and never answered it, its post call is written here by `pa.py` so that the box, the calls and the reads run under one start line, and its listings are made here by Git Bash.

At the sitting, `ratereq.py` writes `rates.xml`; `rbreq.py` writes `rb.xml`; `run.py` writes `a-*` answers and `log-*.txt`; the launch writes `run-pa/` and `data-pa/`; `pa.py` and the self-checks write `box/`; C5 writes the backup. These are outputs, not missing sheet inputs.

## Self-checks before the sitting

Run from `/c/bridge-lab/2162-pa`. No Tally request, launch or restore is part of them.

```bash
cd /c/bridge-lab/2162-pa && python -I -X utf8 rbreq.py --selftest > evidence/selfcheck-rbreq.txt 2>&1; echo "exit $?" >> evidence/selfcheck-rbreq.txt; cat evidence/selfcheck-rbreq.txt
cd /c/bridge-lab/2162-pa && python -I -X utf8 rbcmp.py --selftest > evidence/selfcheck-rbcmp.txt 2>&1; echo "exit $?" >> evidence/selfcheck-rbcmp.txt; cat evidence/selfcheck-rbcmp.txt
cd /c/bridge-lab/2162-pa && python -I ratereq.py --selftest > evidence/selfcheck-ratereq.txt 2>&1; echo "exit $?" >> evidence/selfcheck-ratereq.txt; cat evidence/selfcheck-ratereq.txt
cd /c/bridge-lab/2162-pa && python -I -X utf8 selftest-rate.py > evidence/selfcheck-rate.txt 2>&1; echo "exit $?" >> evidence/selfcheck-rate.txt; cat evidence/selfcheck-rate.txt
cd /c/bridge-lab/2162-pa && python -I -X utf8 grade05.py --selftest > evidence/selfcheck-grade05.txt 2>&1; echo "exit $?" >> evidence/selfcheck-grade05.txt; cat evidence/selfcheck-grade05.txt
cd /c/bridge-lab/2162-pa && python -I -X utf8 selftest-box.py cases C:/bridge-lab/2162-pa > evidence/selfcheck-box.txt 2>&1; echo "exit $?" >> evidence/selfcheck-box.txt; tail -n 3 evidence/selfcheck-box.txt
cd /c/bridge-lab/2162-pa && python -I -c "import ast,sys;[ast.parse(open(f, encoding='utf-8').read()) for f in sys.argv[1:]]" *.py > evidence/selfcheck-parse.txt 2>&1; echo "exit $?" >> evidence/selfcheck-parse.txt; cat evidence/selfcheck-parse.txt
cd /c/bridge-lab/2162-pa && powershell -NoProfile -Command 'foreach ($f in (Get-Item box.ps1, boxstate.ps1)) { $e = $null; [void][System.Management.Automation.Language.Parser]::ParseFile($f.FullName, [ref]$null, [ref]$e); Write-Output $f.Name, $e.Count }' > evidence/selfcheck-ps-parse.txt 2>&1; echo "exit $?" >> evidence/selfcheck-ps-parse.txt; cat evidence/selfcheck-ps-parse.txt
cd /c/bridge-lab/2162-pa && python -I -X utf8 check_sheet.py > evidence/selfcheck-sheet.txt 2>&1; echo "exit $?" >> evidence/selfcheck-sheet.txt; cat evidence/selfcheck-sheet.txt
```

EXPECT, in order: `aa8ebaaf2d0f229bea484c67eb68135afc98c77fe587d9fb2a16a4172cb479cc` then `equal to the pinned hash`, exit 0; `20 cases, 20 right`, exit 0; `96b41d40c58d9527da459219d4ea1b020b3c7e6688053ce3de7a42af892cb812` then `equal to the pinned hash`, exit 0; `23 cases, 23 right`, exit 0; `15 cases, 15 right`, exit 0; `34 cases ran, 34 right`, `ALL CASES RIGHT`, exit 0 (the whole file is kept; it takes about half a minute); no output but `exit 0`; the lines `box.ps1`, `0`, `boxstate.ps1`, `0`, then `exit 0` (this one was not run when the sheet was written: no PowerShell there); three `OK:` lines, exit 0. Anything else: STOP.

## Steps

Raw reads use `run.py`, one call per file, in the order given and under the arm's label, then the named parser. A STOP from either stops the row. Record each whole output. After every raw send typed at the prompt, take the whole-window screenshot required by C3 before the next send or key.

**Prompt rule (C7/C8).** No Tally prompt is expected but C5's backup and C6's restore prompts. The approval box is the build's, not Tally's, and is handled by `pa.py` alone. Any other dialog, message or half-entered screen: screenshot (never while the exe runs; then at part 12), copy its text, press no key, send nothing, STOP and report. Never Escape at Select Company or the Gateway. Never end a process. Nothing is restored after a STOP without the run coordinator's word.

**Path rule.** The screen after each key must be the one named. If not, screenshot and STOP with no further key. After a send or screen act, five minutes with no answer or change is a STOP; never resend.

**Keys.** As row 2162-RC wrote them out, from row 2162-RW's notes. **Shut Company**, at the Gateway: click a blank part of the Gateway's left panel; Alt+K; EXPECT the K: Company menu; mouse click on SHut; EXPECT the Shut Company box listing BRIDGE PILOT LAB alone, highlighted; Enter; EXPECT Select Company, no company loaded, no prompt. A list with more than one line, or any prompt: no key, STOP. **Select Company**: click the filter; type `BRIDGE`, space, `PILOT` until it is the one highlighted line, Enter; EXPECT the Gateway, the current date and the period in Book. **Open the HSN summary**: at the Gateway (screenshot) click `G: Go To`; type `GSTR-1`; EXPECT List of Reports with no highlighted line; Down once (`GSTR-1` highlighted); Enter. Any list, prompt or other screen: path rule; do not select anything. EXPECT `GSTR-1` with the highlight on Total Vouchers. Write down the period shown (row 2162-RW: 1-Aug-26 to 31-Aug-26). Press no period key. A different period is written down and the arm goes on; a later step whose named line then does not show falls under the path rule. Look at no sub-report. With the highlight on Total Vouchers (where GSTR-1 opens; if it is elsewhere, path rule), press Down 24 times, one at a time, with a zoom after each (the 24 lines passed, in the order row 2162-RW's notes record: 1 Included in Return, 2 Ready for Upload, 3 Modified in Books After Upload/Export, 4 Not Relevant for This Return, 5 Uncertain Transactions (Corrections needed), 6 Check Vouchers Having Potential Conflicts with Masters, 7 B2B Invoices - 4A, 4B, 4C, 6B, 6C, 8 B2C (Large) Invoices - 5A, 5B, 9 Exports Invoices - 6A, 10 Credit or Debit Notes (Registered) - 9B, 11 Credit or Debit Notes (Unregistered) - 9B, 12 Amended B2B Invoices - 9A, 13 Amended B2C (Large) Invoices - 9A, 14 Amended Exports Invoices - 9A, 15 Amended Credit or Debit Notes (Registered) - 9C, 16 Amended Credit or Debit Notes (Unregistered) - 9C, 17 B2C (Small) Invoices - 7, 18 Nil Rated Invoices - 8A, 8B, 8C, 8D, 19 Amendment B2C (Small) Invoices - 10, 20 Tax Liability (Advances Received) - 11A(1), 11A(2), 21 Adjustment of Advances - 11B(1), 11B(2), 22 Amended Tax Liability (Advances Received) - 11A, 23 Amendment of Adjusted Advances - 11B, where the list scrolls, 24 the HSN Summary line): EXPECT `HSN Summary - 12 (B2B - B2C Supplies)` alone highlighted; Enter; EXPECT the summary with the highlight on 847130. A line not in that order, or no highlight: path rule. Take a capture before every Escape, and never press Escape at the Gateway.

**Arm frame.** Row 2162-RC's frame. G runs a, b, c and d. P0 runs none of it: step 1's load and step 2's two reads stand for a and b.

- a. Shut Company by the keys above. Any `Quit ?` or other prompt: prompt rule. Load the book alone by the Select Company keys. EXPECT the Gateway and the starting date and period. Otherwise STOP. No other screen is opened.
- b. Send `<arm>-pre`: `M-marks.xml rates.xml`, then `<arm>-pre2`: `rates.xml`, with no key between them. `count19.py --same-row a-start-M-marks.xml a-<arm>-pre-M-marks.xml` must pass; a moved mark: STOP and report. Run `ratecmp.py a-<arm>-pre-rates.xml a-<arm>-pre2-rates.xml` and record it: a difference is a first-read-after-load effect, written down, not a STOP. Then `ratecmp.py a-A0-second-rates.xml a-<arm>-pre2-rates.xml`. The state has cleared when `a-<arm>-pre2-rates.xml` is plain by its sha256 and there is no structural difference; not cleared after Shut Company and a reload is a STOP with no further act. A `READ refused` line or an exit 2: STOP.
- c. The step's screen act, and only that.
- d. Send `<arm>`: `M-marks.xml rates.xml`. `count19.py --same-row a-start-M-marks.xml a-<arm>-M-marks.xml` must pass; a moved mark: STOP and report. Check the answer's bytes and run `ratecmp.py a-A0-second-rates.xml a-<arm>-rates.xml`; record its whole output.

0. **Before the sitting.**
   - Make the row folder and copy this sheet's folder into it byte for byte from the lab's Bridge clone. In the second line, replace `<CLONE>` (both places) with the Git Bash path of the lab's own Bridge clone or sheet worktree, and `<SHEET>` with the 40-character commit the run coordinator's GO names; these are the only two values this sheet cannot write out before step 5. The fetch is the one network act of the row: it brings only the sheet's commit into that clone; no other file is downloaded. EXPECT `made`, then the printed line `exit 0 0`. Any other (an empty second number means the fetch failed): STOP.
     ```bash
     if [ -e /c/bridge-lab/2162-pa ]; then echo "STOP: /c/bridge-lab/2162-pa exists"; else mkdir /c/bridge-lab/2162-pa && echo "made /c/bridge-lab/2162-pa"; fi
     git -C <CLONE> fetch origin && git -C <CLONE> archive <SHEET> lab/2162-pa | tar -x -C /c/bridge-lab/2162-pa --strip-components=2; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"
     cd /c/bridge-lab/2162-pa && if [ -e evidence ] || [ -e local ]; then echo "STOP: evidence or local exists"; else mkdir evidence local && echo "made evidence local"; fi
     ```
   - Run all nine Self-checks. Any miss: STOP.
   - BUILD 6f541ae4, on the lab disk since row 2162-RC. EXPECT `: OK`. `FAILED` or no file: STOP; nothing is downloaded or copied to replace it.
     ```bash
     cd /c/bridge-lab/2162-pa && echo "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc  C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > evidence/s0-build-6f541ae4.txt 2>&1; echo "exit $?" >> evidence/s0-build-6f541ae4.txt; cat evidence/s0-build-6f541ae4.txt
     ```
   - **The grant, a recorded step.** With no Bridge process running and no box open, call the screen tool's `list_granted_applications` and record every entry with its path in `local/s0-grants.txt` (typed by the operator from the tool's answer; kept local, since a path can hold a user name), with an evidence copy `evidence/s0-grants.txt` that writes every path outside `C:\bridge-lab` as `<path>`. EXPECT (lab 2569-S) `C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe` not on it. If it is not on it: call `request_access` for that exe path, once, and record the request, the answer, and `list_granted_applications` again after it. Granted, refused or not offered, the row goes on (it takes no screen-tool screenshot while the exe runs); the outcome is recorded and given in the step 0 notes. A grant for any other path is not asked for.
   - C2 step 0 with no company loaded. EXPECT one `tally.exe` line and its PID; a `LISTENING` line on `:9001` with that PID; `curl exit 0` and `200`; no dialog in the first screenshot; and no `bridge_mcp.exe` running (`INFO: No tasks are running`). Otherwise STOP. `netstat`'s file is kept whole; the `grep` only shows the lines to read.
     ```bash
     cd /c/bridge-lab/2162-pa && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > evidence/s0-tasklist.txt 2>&1; echo "exit $?" >> evidence/s0-tasklist.txt; cat evidence/s0-tasklist.txt
     cd /c/bridge-lab/2162-pa && netstat -ano > evidence/s0-netstat.txt 2>&1; echo "exit $?" >> evidence/s0-netstat.txt; grep -E ':9001 |^exit' evidence/s0-netstat.txt
     cd /c/bridge-lab/2162-pa && curl.exe -sS -w "%{http_code} %{size_download}\n" http://127.0.0.1:9001/status -o evidence/s0-status-body.txt > evidence/s0-status.txt 2>&1; echo "curl exit $?" >> evidence/s0-status.txt; cat evidence/s0-status.txt
     cd /c/bridge-lab/2162-pa && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/s0-bridge.txt 2>&1; echo "exit $?" >> evidence/s0-bridge.txt; cat evidence/s0-bridge.txt
     ```
1. **Load, check, back up, and render the rate request.**
   - Load BRIDGE PILOT LAB alone by the Select Company keys (C1). No Escape at Select Company. EXPECT the Gateway, current date and period in Book. Otherwise STOP.
   - Send `start`: `M-marks.xml`; parse it. EXPECT one row, the pinned GUID, V 68, M 245 (lab 2162-RC). Otherwise STOP before BK1.
     ```bash
     cd /c/bridge-lab/2162-pa && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 start M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py a-start-M-marks.xml > evidence/start-count19.txt 2>&1; echo "exit $?" >> evidence/start-count19.txt; cat evidence/start-count19.txt
     ```
   - Make `backup-1` new and empty; take BK1 by C5, with the keys row 2162-RC's sheet writes out. Click the Gateway's left panel; Alt+Y: EXPECT the Data menu; Enter (Backup & Restore); Enter (Backup): EXPECT the Backup screen. In Backup Data Path press Backspace: EXPECT the List of Folders; Up three times, one at a time with a zoom after each (TallyDrive, Select from Drive, then `Specify Path` highlighted); Enter; type `C:\bridge-lab\2162-pa\backup-1` (zoom: exact); Enter: EXPECT the list with that folder highlighted; Enter: EXPECT the path to read that folder. In the company filter type `BRIDGE`, space, `PILOT`: EXPECT one line, BRIDGE PILOT LAB, highlighted; Enter; click `End of List` (zoom: highlighted); Enter. EXPECT `Accept ?` showing that path and the one company: screenshot, click inside the box, `y`. EXPECT the Gateway and no message. A line, list or prompt other than these: path rule. Record the file's bytes and SHA-256 and the backup's time to the minute (row 2162-RC recorded the file's write time; the Restore list shows the time Tally gives it). EXPECT exactly one non-empty backup file (lab 2162-RC: one file, 90,051 bytes). Otherwise STOP.
     ```bash
     cd /c/bridge-lab/2162-pa && if [ -e backup-1 ]; then echo "STOP: backup-1 exists"; else mkdir backup-1 && echo "made backup-1"; fi
     # after the backup: the listing and the hash stay local (the file name holds the company number)
     cd /c/bridge-lab/2162-pa && ls -la --time-style=full-iso backup-1 > local/bk1.txt 2>&1; sha256sum backup-1/* >> local/bk1.txt 2>&1; echo "exit $?" >> local/bk1.txt; cat local/bk1.txt
     ```
   - Send `setup`: `M-marks.xml guids.xml`, in that order. EXPECT marks still 68/245; a moved mark: STOP and report. Then parse the GUIDs. EXPECT `LEDGER ROWS 6` and exactly the six names of call `05`, each with a `BRIDGECOMPANYGUID` equal to the book's GUID and its own GUID the book's GUID followed by the suffix rows 2162-RW, 2162-RX and 2162-RC read: `BRIDGE Walk-in` `-000000d0`, `BRIDGE Svc 998313 5%` `-000000d1`, `BRIDGE Svc 998314 5%` `-000000d2`, `BRIDGE CGST 2.5%` `-000000d4`, `BRIDGE SGST 2.5%` `-000000d6`, `BRIDGE Round Off` `-000000d8` (lab 2162-RX, notes). Any other count, name or GUID, or a `READ refused` line: STOP.
     ```bash
     cd /c/bridge-lab/2162-pa && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 setup M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && grep '  guids.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 setup guids.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-setup-M-marks.xml > evidence/setup-count19.txt 2>&1; echo "exit $?" >> evidence/setup-count19.txt; cat evidence/setup-count19.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 ratecmp.py a-setup-guids.xml > evidence/setup-guids.txt 2>&1; echo "exit $?" >> evidence/setup-guids.txt; cat evidence/setup-guids.txt
     ```
   - Render the rate request used all sitting, exactly as row 2162-RC did (the build's window, code `src-tauri/src/agent_import_invoice_wire.rs:1206-1212`). The book's GUID is read from `calls.py`, so no GUID is typed. EXPECT `rates.xml: 2874 bytes, sha256 182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08` (lab 2162-RC), then `rates.xml: OK`. Anything else: STOP.
     ```bash
     cd /c/bridge-lab/2162-pa && G=$(sed -n 's/^BOOK = "\([0-9a-f-]*\)".*/\1/p' calls.py) && echo "book GUID read: ${#G} characters" > evidence/ratereq-out.txt && python -I ratereq.py --company "BRIDGE PILOT LAB" --from-date 20260401 --to-date 20260801 --output rates.xml "$G-000000d0" "$G-000000d1" "$G-000000d2" "$G-000000d4" "$G-000000d6" "$G-000000d8" >> evidence/ratereq-out.txt 2>&1; echo "exit $?" >> evidence/ratereq-out.txt; cat evidence/ratereq-out.txt
     cd /c/bridge-lab/2162-pa && echo "182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08  rates.xml" > local/rates.sha256; sha256sum -c local/rates.sha256
     ```
2. **P0: the plain state, reads only.**
   - With no key pressed and no report opened between them, send `A0-first`: `M-marks.xml rates.xml`, then `A0-second`: `rates.xml`. `count19.py --same-row a-start-M-marks.xml a-A0-first-M-marks.xml` must pass; a moved mark: STOP and report. EXPECT `LEDGER ROWS 6`, the six names, every one `SHAPE measured`, no structural difference, and both answers `: OK` against the plain sha256 (lab 2162-RC). A `READ refused` line, or an answer not plain other than under the exception below: STOP.
     ```bash
     cd /c/bridge-lab/2162-pa && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 A0-first M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 A0-first rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 A0-second rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-A0-first-M-marks.xml > evidence/A0-first-count19.txt 2>&1; echo "exit $?" >> evidence/A0-first-count19.txt; cat evidence/A0-first-count19.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 ratecmp.py a-A0-first-rates.xml a-A0-second-rates.xml > evidence/A0-first-vs-A0-second.txt 2>&1; echo "exit $?" >> evidence/A0-first-vs-A0-second.txt; cat evidence/A0-first-vs-A0-second.txt
     cd /c/bridge-lab/2162-pa && stat -c '%n %s' a-A0-first-rates.xml > evidence/A0-first-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-A0-first-rates.xml" | sha256sum -c - >> evidence/A0-first-is-plain.txt 2>&1; echo "exit $?" >> evidence/A0-first-is-plain.txt; cat evidence/A0-first-is-plain.txt
     cd /c/bridge-lab/2162-pa && stat -c '%n %s' a-A0-second-rates.xml > evidence/A0-second-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-A0-second-rates.xml" | sha256sum -c - >> evidence/A0-second-is-plain.txt 2>&1; echo "exit $?" >> evidence/A0-second-is-plain.txt; cat evidence/A0-second-is-plain.txt
     ```
   - The exception (row 2162-RC's): only if both answers show `LEDGER ROWS 6`, the six names, every one `SHAPE measured` and no `READ refused` line, and the only failure is that `A0-first` is not plain while `A0-second` is, send `A0-third`: `rates.xml`, once. EXPECT `A0-third` plain and no structural difference from `A0-second`; record a first-read effect with the whole output of first against second. Otherwise STOP. The sheet's later comparisons use `a-A0-second-rates.xml` in every case.
     ```bash
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 A0-third rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-A0-third-rates.xml > evidence/A0-second-vs-A0-third.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-A0-third.txt; cat evidence/A0-second-vs-A0-third.txt
     cd /c/bridge-lab/2162-pa && stat -c '%n %s' a-A0-third-rates.xml > evidence/A0-third-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-A0-third-rates.xml" | sha256sum -c - >> evidence/A0-third-is-plain.txt 2>&1; echo "exit $?" >> evidence/A0-third-is-plain.txt; cat evidence/A0-third-is-plain.txt
     ```
3. **G: Shut Company, a load, and the W1 walk** (row 2162-RC's step 3, frames a to d).
   - Frame a.
   - Frame b's commands, after frame a and before frame c. EXPECT `G-pre2` `: OK` against the plain sha256 and no structural difference from `A0-second` (lab 2162-RC); not cleared is a STOP (frame b). `G-pre` against `G-pre2` is recorded.
     ```bash
     cd /c/bridge-lab/2162-pa && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G-pre M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G-pre rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G-pre2 rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G-pre-M-marks.xml > evidence/G-pre-count19.txt 2>&1; echo "exit $?" >> evidence/G-pre-count19.txt; cat evidence/G-pre-count19.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 ratecmp.py a-G-pre-rates.xml a-G-pre2-rates.xml > evidence/G-pre-vs-G-pre2.txt 2>&1; echo "exit $?" >> evidence/G-pre-vs-G-pre2.txt; cat evidence/G-pre-vs-G-pre2.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-G-pre2-rates.xml > evidence/A0-second-vs-G-pre2.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-G-pre2.txt; cat evidence/A0-second-vs-G-pre2.txt
     cd /c/bridge-lab/2162-pa && stat -c '%n %s' a-G-pre2-rates.xml > evidence/G-pre2-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-G-pre2-rates.xml" | sha256sum -c - >> evidence/G-pre2-is-plain.txt 2>&1; echo "exit $?" >> evidence/G-pre2-is-plain.txt; cat evidence/G-pre2-is-plain.txt
     ```
   - Frame c, the W1 walk: open the HSN summary by the keys above. Down twice from 847130, one at a time with a zoom after each (9983, then `998313` alone highlighted); Enter: EXPECT `GSTR-1 - Voucher Register`, `Vouchers of 998313` (lab 2162-SALES-Q step 12.5; lab 2162-RW step 5). Esc once: EXPECT the summary, highlight 998313. Down once (zoom: `998314` alone highlighted); Enter: EXPECT `Vouchers of 998314`. Esc once: EXPECT the summary, highlight 998314. Esc once: EXPECT `GSTR-1`. Esc once: EXPECT the Gateway. No further Escape.
   - Frame d (`G`), after the walk. EXPECT the answer grown (`: OK` against the grown sha256, 30626 bytes), and `ratecmp.py` against `A0-second` to show added 28, removed 0, text changed 0, every differing path under `BRIDGE Svc 998313 5%` or `BRIDGE Svc 998314 5%`, those two `SHAPE unmeasured` and the other four measured (lab 2162-RX, W1; lab 2162-RC). **Not grown: record it whole, launch nothing, STOP** (the row's premise failed). A `READ refused` line or an exit 2: STOP.
     ```bash
     cd /c/bridge-lab/2162-pa && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G-M-marks.xml > evidence/G-count19.txt 2>&1; echo "exit $?" >> evidence/G-count19.txt; cat evidence/G-count19.txt
     cd /c/bridge-lab/2162-pa && stat -c '%n %s' a-G-rates.xml > evidence/G-is-grown.txt 2>&1; echo "d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257  a-G-rates.xml" | sha256sum -c - >> evidence/G-is-grown.txt 2>&1; echo "exit $?" >> evidence/G-is-grown.txt; cat evidence/G-is-grown.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-G-rates.xml > evidence/A0-second-vs-G.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-G.txt; cat evidence/A0-second-vs-G.txt
     ```
   - **From here to the end of step 4's part 12: no Shut Company, no load, no key and no screen act in Tally, and no raw request typed at the prompt but part 6c's one rate read.** The screen shows the Gateway with BRIDGE PILOT LAB and no dialog (G's last screenshot). No screen-tool screenshot from part 4 to the end of part 12's stop file, unless a STOP needs one, and none at all between the post line and `pa exit` (`box.ps1`'s own GDI copies inside that window are departure 4).
4. **PA: launch LPA in the grown state; build, post, read back, verify.**
   - **Launch LPA, part 1: folders.** EXPECT `made`; `STOP` is a STOP (never reuse a launch folder).
     ```bash
     cd /c/bridge-lab/2162-pa && if [ -e run-pa ] || [ -e data-pa ]; then echo "STOP: run-pa or data-pa exists"; else mkdir run-pa run-pa/inbox run-pa/answers data-pa && echo "made run-pa run-pa/inbox run-pa/answers data-pa"; fi
     ```
   - **Launch LPA, part 2: the exe and the launch file.** EXPECT two lines ending `: OK`. `FAILED` or no output: STOP.
     ```bash
     cd /c/bridge-lab/2162-pa && echo "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc  C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > evidence/run-pa-hashes.txt 2>&1; echo "exit $?" >> evidence/run-pa-hashes.txt; cat evidence/run-pa-hashes.txt
     cd /c/bridge-lab/2162-pa && cp launch/LPA-launch-env.json run-pa/launch-env.json && grep '  launch/LPA-launch-env.json$' SHA256.txt | awk '{print $1"  run-pa/launch-env.json"}' | sha256sum -c - >> evidence/run-pa-hashes.txt 2>&1; echo "exit $?" >> evidence/run-pa-hashes.txt; cat evidence/run-pa-hashes.txt
     ```
   - **Launch LPA, part 3: other Bridge processes.** EXPECT `INFO: No tasks are running` and `exit 0`; a `bridge_mcp.exe` line: STOP and report (none may call port 9001 during this sitting).
     ```bash
     cd /c/bridge-lab/2162-pa && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-pa-procs-before.txt 2>&1; echo "exit $?" >> evidence/run-pa-procs-before.txt; cat evidence/run-pa-procs-before.txt
     ```
   - **Launch LPA, part 4: start the client.** Start the first line, whole, with the operator's shell tool's own background run, as rows 2440-CP, 2569-R, 2569-S and 2162-RC did: no window, standard input from the null device. Type no `&`, change nothing in the line, and record in the notes how it was started. It prints nothing and runs until part 12's stop file. Then, within 60 s, type the second line. EXPECT the exe hash `ok`, the launch file's hash equal to `SHA256.txt`'s, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` with version 0.7.0 (lab 2162-RC). A `stop:` line, or no `initialized` line in 60 s: STOP.
     ```bash
     ( cd /c/bridge-lab/2162-pa && python -I -X utf8 client.py "C:/bridge-lab/2162-pa/run-pa" "C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc" < /dev/null > evidence/run-pa-client-out.txt 2> evidence/run-pa-client-err.txt; echo "client exit $?" >> evidence/run-pa-launch.txt )
     cd /c/bridge-lab/2162-pa && cat run-pa/client-log.txt
     ```
   - **Launch LPA, part 5: the four calls before the post, one at a time, in this order.** Each call has three lines. The first copies the call in under a name ending `.part`, checks its hash against `SHA256.txt`, and only then renames it (lab 2162-SALES-Q, deviation 4); EXPECT `: OK` and `placed`. The second shows whether its answer has arrived: type it until `run-pa/answers/` lists the call's name and the log's last lines show the call's line, at most 60 times; then a STOP. Only then type the third, which grades the answer; wait for its EXPECT before the next call. Exit 2 from the third line, typed after the answer arrived, is a failed call: STOP.
     ```bash
     cd /c/bridge-lab/2162-pa && cp calls/01-list.json run-pa/inbox/01-list.json.part && grep '  calls/01-list.json$' SHA256.txt | awk '{print $1"  run-pa/inbox/01-list.json.part"}' | sha256sum -c - && mv run-pa/inbox/01-list.json.part run-pa/inbox/01-list.json && echo "placed 01-list.json" | tee -a evidence/run-pa-launch.txt
     cd /c/bridge-lab/2162-pa && ls run-pa/answers; tail -n 3 run-pa/client-log.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 calls.py show run-pa/answers/01-list.json > evidence/run-pa-show-01-list.json.txt 2>&1; echo "exit $?" >> evidence/run-pa-show-01-list.json.txt; cat evidence/run-pa-show-01-list.json.txt
     cd /c/bridge-lab/2162-pa && cp calls/02-list_companies.json run-pa/inbox/02-list_companies.json.part && grep '  calls/02-list_companies.json$' SHA256.txt | awk '{print $1"  run-pa/inbox/02-list_companies.json.part"}' | sha256sum -c - && mv run-pa/inbox/02-list_companies.json.part run-pa/inbox/02-list_companies.json && echo "placed 02-list_companies.json" | tee -a evidence/run-pa-launch.txt
     cd /c/bridge-lab/2162-pa && ls run-pa/answers; tail -n 3 run-pa/client-log.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 calls.py show run-pa/answers/02-list_companies.json > evidence/run-pa-show-02-list_companies.json.txt 2>&1; echo "exit $?" >> evidence/run-pa-show-02-list_companies.json.txt; cat evidence/run-pa-show-02-list_companies.json.txt
     cd /c/bridge-lab/2162-pa && cp calls/03-marks.json run-pa/inbox/03-marks.json.part && grep '  calls/03-marks.json$' SHA256.txt | awk '{print $1"  run-pa/inbox/03-marks.json.part"}' | sha256sum -c - && mv run-pa/inbox/03-marks.json.part run-pa/inbox/03-marks.json && echo "placed 03-marks.json" | tee -a evidence/run-pa-launch.txt
     cd /c/bridge-lab/2162-pa && ls run-pa/answers; tail -n 3 run-pa/client-log.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 calls.py show run-pa/answers/03-marks.json > evidence/run-pa-show-03-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-pa-show-03-marks.json.txt; cat evidence/run-pa-show-03-marks.json.txt
     cd /c/bridge-lab/2162-pa && cp calls/05-build-0074-0801.json run-pa/inbox/05-build-0074-0801.json.part && grep '  calls/05-build-0074-0801.json$' SHA256.txt | awk '{print $1"  run-pa/inbox/05-build-0074-0801.json.part"}' | sha256sum -c - && mv run-pa/inbox/05-build-0074-0801.json.part run-pa/inbox/05-build-0074-0801.json && echo "placed 05-build-0074-0801.json" | tee -a evidence/run-pa-launch.txt
     cd /c/bridge-lab/2162-pa && ls run-pa/answers; tail -n 3 run-pa/client-log.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 grade05.py grade run-pa/answers/05-build-0074-0801.json > evidence/run-pa-grade05.txt 2>&1; echo "exit $?" >> evidence/run-pa-grade05.txt; cat evidence/run-pa-grade05.txt
     ```
     EXPECT `01` `tools: 37; missing of this row's: []` (lab 2162-RC: record the count, grade only `missing ... []`); `02` one company with the pinned GUID; `03` M 245, V 68, reused False (lab 2162-RC; a moved mark is a STOP). `05`: `VERDICT admitted voucher_count 1, no refusal` (lab 2162-RC, its grown-state launch of this build). A refusal: record `grade05.py`'s whole output, go to part 12, then `PA-post`, then STOP: the state or the build differs from row 2162-RC's, and nothing is posted.
   - **Launch LPA, part 6: the batch file.** EXPECT `equal to the answer's sha256` (lab 2162-RC: its grown-state batch file 2,421 bytes; the size is recorded, not graded); `FINDING` (exit 1) is a STOP, to part 12. The second line keeps the data folder's listing whole: EXPECT the batch file and `agent-import-ledger.jsonl`.
     ```bash
     cd /c/bridge-lab/2162-pa && python -I -X utf8 calls.py batch run-pa/answers/05-build-0074-0801.json data-pa > evidence/run-pa-batch.txt 2>&1; echo "exit $?" >> evidence/run-pa-batch.txt; cat evidence/run-pa-batch.txt
     cd /c/bridge-lab/2162-pa && find data-pa -type f > evidence/run-pa-data-files.txt 2>&1; echo "exit $?" >> evidence/run-pa-data-files.txt; cat evidence/run-pa-data-files.txt
     ```
   - **Launch LPA, part 6b: render the raw read-back request** from the build's own journal record (above, The questions). The book's GUID is read from `calls.py`, so no GUID is typed. EXPECT `record: ... the batch's voucher recorded once (or identically)`; `type GUID: the book's GUID followed by -000000ce: yes; number BP/26-27/0074; date 20260801`; `window 20260401 to 20270331; company 'BRIDGE PILOT LAB'`; `rb.xml: 2332 bytes, sha256 5a72bf7d4243238b4e7b81ccee66edaa79cad950caee414865791e6cd3cb9547` (not shown: computed from these inputs when the sheet was written); exit 0; then `rb.xml: OK`. A `FINDING` (exit 1: the record is not the invoice given, nothing written) or a `FAILED READ` (exit 2): STOP, to part 12; nothing is posted. Another size or hash with exit 0: STOP, to part 12, and report the line whole (the record's type GUID is what the build sends; this sheet's expectation would be wrong, and nothing is posted without the run coordinator's word).
     ```bash
     cd /c/bridge-lab/2162-pa && G=$(sed -n 's/^BOOK = "\([0-9a-f-]*\)".*/\1/p' calls.py) && echo "book GUID read: ${#G} characters" > evidence/rbreq-out.txt && python -I -X utf8 rbreq.py --company "BRIDGE PILOT LAB" --book "$G" --record data-pa --build run-pa/answers/05-build-0074-0801.json --number BP/26-27/0074 --date 20260801 --output rb.xml >> evidence/rbreq-out.txt 2>&1; echo "exit $?" >> evidence/rbreq-out.txt; cat evidence/rbreq-out.txt
     cd /c/bridge-lab/2162-pa && echo "5a72bf7d4243238b4e7b81ccee66edaa79cad950caee414865791e6cd3cb9547  rb.xml" > local/rb.sha256; sha256sum -c local/rb.sha256
     ```
   - **Launch LPA, part 6c: the post's premise, measured just before it.** With LPA running and idle (no call in flight: `05`'s answer was read whole, and the post line is not yet started), send `PA-pre`: `rates.xml` (Launch, departures 1 and 2; no screenshot after it, the C3 screenshot is part 12's). EXPECT the answer grown: `: OK` against the grown sha256, 30626 bytes, and `ratecmp.py` against `A0-second` as at frame d (not shown: the grown bytes are lab 2162-RC's, but no row has read the rates after a build call in the grown state). **Not grown: STOP, to part 12; nothing is posted** (the state the post was to be measured in is gone). A `READ refused` line or an exit 2: STOP, to part 12.
     ```bash
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 PA-pre rates.xml
     cd /c/bridge-lab/2162-pa && stat -c '%n %s' a-PA-pre-rates.xml > evidence/PA-pre-is-grown.txt 2>&1; echo "d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257  a-PA-pre-rates.xml" | sha256sum -c - >> evidence/PA-pre-is-grown.txt 2>&1; echo "exit $?" >> evidence/PA-pre-is-grown.txt; cat evidence/PA-pre-is-grown.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-PA-pre-rates.xml > evidence/A0-second-vs-PA-pre.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-PA-pre.txt; cat evidence/A0-second-vs-PA-pre.txt
     ```
   - **Launch LPA, part 7: the post.** Before it: the last screenshot (G's) showed the Gateway with BRIDGE PILOT LAB and no dialog, and nothing has acted on the screen since. Start the line, whole, with the operator's shell tool's own background run, as part 4's: type no `&`, change nothing in it, record how it was started. From this line until `pa exit`, nothing acts on the screen (Launch, departure 3). It writes the post call itself (`run-pa/answers/pa1-post-1.json` is its first answer), and later calls `pa1-post-2` to `pa1-post-4` only as The tool sequence says.
     ```bash
     ( cd /c/bridge-lab/2162-pa && python -I -X utf8 pa.py pa1 05-build-0074-0801.json < /dev/null > evidence/pa-out.txt 2>&1; echo "pa exit $?" >> evidence/pa-out.txt )
     ```
   - **Launch LPA, part 8: wait for `pa exit`.** Type this line about every 5 s, at most 72 times (six minutes: the box's 120 s, up to four calls, and the reads at +60 s). EXPECT, in time, a last line `pa exit <n>`. No `pa exit` line after 72 reads: STOP and report, touching nothing (the box may be up; it is the build's to close).
     ```bash
     cd /c/bridge-lab/2162-pa && tail -n 2 evidence/pa-out.txt; ls run-pa/answers box
     ```
   - **Launch LPA, part 9: read what `pa.py` printed.** EXPECT in `evidence/pa-out.txt`, in order:
     - `sent pa1-post-1.json`, and then either nothing more about calls (one call), or `pending: retry_after_s 15, ...; calling again` (code `src-tauri/src/agent_import_approval.rs:107`), or `approved, not yet posted ...; calling again now` (code `src-tauri/src/agent_import_post.rs:611-633`), each followed by the next `sent` line; lab 2162-SALES-Q posted on its first call;
     - the box log's lines: `BOX SEEN` with the title `ComplyEaze Bridge — post this voucher?` and class `#32770` (code `src-tauri/src/tally/approved_import.rs:3413`), its `rect` and `z-order place box 0` line, `capture`, `WAITING`, then `CLICKED Yes` and the after-click line; `box.ps1 exit 0`;
     - the guard: `R1 boxstate.ps1 on pid <pid>: exit 0; box lines 1`, seven `PASS` lines, `GUARD PASSED: boxcheck.py started`, `boxcheck.py exit 0`;
     - the check: its `PASS` lines, `ALL PASS`, `answer file written: Yes`;
     - the times: `post`, then `a-R5-rb.xml` sent at about +5 s and `a-R60-rb.xml` at about +60 s, each `run.py 0` with its size: `pa.py` sends `R5`: `rb.xml` and then `R60`: `rb.xml`, by `run.py`, after checking `rb.xml` against `local/rb.sha256` each time;
     - the last answer by `calls.py show`: `post_import`, `attempt_recorded`, `dispatch state posted_verified`, counters with `created` 1 and every other counter 0, `verification_status posted_verified` (code `src-tauri/src/agent_import_post_e2e_tests.rs:7855` and `:7878-7900`; `dispatch state` from lab 2162-SALES-Q);
     - `POST posted_verified, created 1, the box guarded and checked, both raw reads sent`, then `pa exit 0`.
     `pa exit 1`: record the whole file; it is a STOP, and the parts that still run are named under Steps after a STOP in the post, below. `pa exit 2`: a failed read of the helper's own inputs, nothing sent (`pa.py` checks its inputs before it starts the box watch): STOP, to part 12.
   - **Launch LPA, part 10: Q2, and the post's own count.** Type the line whose number is the last answer's: `pa.py`'s `--- last answer (<k> call(s))` names it. EXPECT the post's lines `PASS` (dispatch `posted_verified`, created 1, the alter-id delta 1, the target voucher step 1, the post span bound, one voucher); each raw read `ROWS 1`, its status values printed, `tie: GUID equal to the post's voucher: yes; ALTERID equal to the post's ...: yes; VOUCHERNUMBER equal to the post's 'BP/26-27/0074': yes`, `status equal to the build's own read: yes`; `BYTES a-R5-rb.xml and a-R60-rb.xml: equal` (not shown; `differ` alone is not a finding when the next line holds); `STATUS FIELDS, ALTERID and GUID of the two reads: equal`; `ITEM 2 EQUAL`; exit 0. `ITEM 2 NOT EQUAL` (exit 1) is the result Q2 asks about: record every `FINDING` line; it is not a STOP, and the row goes on to part 11. Any `ROWS` other than 1 at either read, or a tie that fails, is a voucher count other than the post's one: STOP after part 12's close and `PA-post`. Exit 2: a failed read, recorded; go on to part 11.
     ```bash
     cd /c/bridge-lab/2162-pa && python -I -X utf8 rbcmp.py cmp run-pa/answers/pa1-post-1.json a-R5-rb.xml a-R60-rb.xml > evidence/pa-rbcmp.txt 2>&1; echo "exit $?" >> evidence/pa-rbcmp.txt; cat evidence/pa-rbcmp.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 rbcmp.py cmp run-pa/answers/pa1-post-2.json a-R5-rb.xml a-R60-rb.xml > evidence/pa-rbcmp.txt 2>&1; echo "exit $?" >> evidence/pa-rbcmp.txt; cat evidence/pa-rbcmp.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 rbcmp.py cmp run-pa/answers/pa1-post-3.json a-R5-rb.xml a-R60-rb.xml > evidence/pa-rbcmp.txt 2>&1; echo "exit $?" >> evidence/pa-rbcmp.txt; cat evidence/pa-rbcmp.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 rbcmp.py cmp run-pa/answers/pa1-post-4.json a-R5-rb.xml a-R60-rb.xml > evidence/pa-rbcmp.txt 2>&1; echo "exit $?" >> evidence/pa-rbcmp.txt; cat evidence/pa-rbcmp.txt
     ```
     Then the check once more on the real box log (the batch id is read from the build answer inside the line). EXPECT `REPLAY ALL PASS`, exit 0: the check read a real invoice box of this build. `REPLAY NOT ALL PASS` after a pressed Yes would mean the check passed a box it now fails: STOP after part 12, and report.
     ```bash
     cd /c/bridge-lab/2162-pa && B=$(python -I -c "import json,sys; print(json.load(open(sys.argv[1], encoding='utf-8'))['response']['result']['structuredContent']['result']['batch_id'])" run-pa/answers/05-build-0074-0801.json) && echo "batch id read: ${#B} characters" > evidence/pa-replay.txt && python -I -X utf8 selftest-box.py replay C:/bridge-lab/2162-pa pa1 "$B" >> evidence/pa-replay.txt 2>&1; echo "exit $?" >> evidence/pa-replay.txt; cat evidence/pa-replay.txt
     ```
   - **Launch LPA, part 11: `verify_import`, then the marks.** The first line writes the verify call for the build answer's batch straight into the inbox (whole, then renamed in); EXPECT its name, `verify_import`, and its sha256. Then wait as part 5 does, then grade. EXPECT `verification_status posted_verified` (lab 2162-SALES-Q). Then `07`: EXPECT M 245, V 69, reused False (lab 2162-SALES-Q: V 68 to 69 across its post). V other than 69, or M other than 245: STOP after part 12 (the post added other than one voucher, or something else wrote).
     ```bash
     cd /c/bridge-lab/2162-pa && python -I -X utf8 calls.py verify run-pa/answers/05-build-0074-0801.json run-pa/inbox/08-verify.json > evidence/run-pa-verify-call.txt 2>&1; echo "exit $?" >> evidence/run-pa-verify-call.txt; cat evidence/run-pa-verify-call.txt
     cd /c/bridge-lab/2162-pa && ls run-pa/answers; tail -n 3 run-pa/client-log.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 calls.py show run-pa/answers/08-verify.json > evidence/run-pa-show-08-verify.json.txt 2>&1; echo "exit $?" >> evidence/run-pa-show-08-verify.json.txt; cat evidence/run-pa-show-08-verify.json.txt
     cd /c/bridge-lab/2162-pa && cp calls/07-marks.json run-pa/inbox/07-marks.json.part && grep '  calls/07-marks.json$' SHA256.txt | awk '{print $1"  run-pa/inbox/07-marks.json.part"}' | sha256sum -c - && mv run-pa/inbox/07-marks.json.part run-pa/inbox/07-marks.json && echo "placed 07-marks.json" | tee -a evidence/run-pa-launch.txt
     cd /c/bridge-lab/2162-pa && ls run-pa/answers; tail -n 3 run-pa/client-log.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 calls.py show run-pa/answers/07-marks.json > evidence/run-pa-show-07-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-pa-show-07-marks.json.txt; cat evidence/run-pa-show-07-marks.json.txt
     ```
   - **Launch LPA, part 12: close.** Type the first line. Then type the second line until `evidence/run-pa-launch.txt` ends with a `client exit` line, at most 12 times. Then the third. EXPECT the log's last line `session ended; server pid <pid> exit code 0`, `client exit 0` as the launch file's last line, the server error log's size, and then `INFO: No tasks are running`. Never end a process. The log line `server did not exit within 30 s of stdin close; killed` (`client.py:111-113`) is a STOP and report. No `client exit` line in 60 s, any other exit, or a server that does not end: STOP and report. Then the whole-window screenshot C3 asks for after the three raw reads (Launch, departure 2): EXPECT the Gateway with BRIDGE PILOT LAB and no dialog.
     ```bash
     cd /c/bridge-lab/2162-pa && touch run-pa/inbox/stop && echo "stop file placed" | tee -a evidence/run-pa-launch.txt
     cd /c/bridge-lab/2162-pa && tail -n 1 run-pa/client-log.txt; cat evidence/run-pa-launch.txt; stat -c '%n %s' run-pa/server-stderr-*.log
     cd /c/bridge-lab/2162-pa && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-pa-procs-after.txt 2>&1; echo "exit $?" >> evidence/run-pa-procs-after.txt; cat evidence/run-pa-procs-after.txt
     ```
   - **`PA-post`: the marks and the book after the post, the exe closed.** Send `PA-post`: `M-marks.xml rates.xml rb.xml`. EXPECT from `count19.py --before-write` the pinned GUID and the marks `69` and `245` against the start's `68` and `245`: **V exactly one more, M unchanged** (not shown). `--before-write` compares the GUID only and exits 0 whatever V and M are, so read V and M from its printed line: its exit 0 does not grade them. V other than 69 or M other than 245 is a STOP (the post added other than one voucher). The rate answer's state is recorded by its sha256 (not shown whether a post changes it; recorded, not graded). `rbcmp.py show` of the read-back: EXPECT `ROWS 1` and the status values of `a-R60-rb.xml` (recorded beside part 10).
     ```bash
     cd /c/bridge-lab/2162-pa && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 PA-post M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 PA-post rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rb.sha256 && python -I run.py http://127.0.0.1:9001 PA-post rb.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py --before-write a-start-M-marks.xml a-PA-post-M-marks.xml > evidence/PA-post-count19.txt 2>&1; echo "exit $?" >> evidence/PA-post-count19.txt; cat evidence/PA-post-count19.txt
     cd /c/bridge-lab/2162-pa && stat -c '%n %s' a-PA-post-rates.xml > evidence/PA-post-rates-state.txt 2>&1; sha256sum a-PA-post-rates.xml >> evidence/PA-post-rates-state.txt 2>&1; echo "exit $?" >> evidence/PA-post-rates-state.txt; cat evidence/PA-post-rates-state.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 rbcmp.py show a-PA-post-rb.xml > evidence/PA-post-rb.txt 2>&1; echo "exit $?" >> evidence/PA-post-rb.txt; cat evidence/PA-post-rb.txt
     ```
   - **Steps after a STOP in the post** (C8). A STOP anywhere in parts 7 to 11 places no further call into the inbox and sends nothing typed; part 8's wait still runs to `pa exit` (the helper ends by itself), then part 12's close, then `PA-post` once (its reads are the state reads), then report. Which stage refused is read from `evidence/pa-out.txt`: a refusal with no `BOX SEEN` line is the re-read before the box; a refusal after `CLICKED Yes` is the re-read after the answer or the queue's re-read (the same codes; not told apart); `GUARD NOT PASSED` or a check without `ALL PASS` means no answer was written and nothing was pressed, and the post's refusal is the build's own timeout, recorded whole. In every case the book holds what `PA-post`'s marks show, and step 5 runs only on RESTORE.
5. **R: restore BK1, only on the run coordinator's explicit RESTORE word in the GO** (Book). Not before LPA has closed (part 12) and `PA-post` is read.
   - Check BK1 against step 1's record and write `restore-plan.txt` (C6: folder, file, hash, company number, Restore Data Path, the two prompts expected), kept local. EXPECT the same file, bytes and sha256 as `local/bk1.txt`, and one file only. Otherwise STOP before the restore.
     ```bash
     cd /c/bridge-lab/2162-pa && ls -la --time-style=full-iso backup-1 > local/r-bk1.txt 2>&1; sha256sum backup-1/* >> local/r-bk1.txt 2>&1; echo "exit $?" >> local/r-bk1.txt; cat local/r-bk1.txt
     ```
   - List the company's own data folder before (names, sizes, times; no file is opened). The path holds the company number: `<company data folder>` is the one value this line cannot write out, typed by the lab as the Git Bash path of the folder named by the company number under the Restore Data Path; the notes and every evidence copy write it as `<company data folder>`, and the listing stays local. EXPECT a listing and `exit 0`; an error or a non-zero exit: STOP before the restore.
     ```bash
     cd /c/bridge-lab/2162-pa && ls -la --time-style=full-iso "<company data folder>" > local/r-data-before.txt 2>&1; echo "exit $?" >> local/r-data-before.txt; tail -n 3 local/r-data-before.txt
     ```
   - The restore, by C6, with the keys row 2569-S's step 9 used (its lab notes). Shut Company by the keys above: EXPECT Select Company, none loaded. Alt+Y: EXPECT the Data menu; Enter (Backup & Restore); click Restore: EXPECT Restore Companies, its Restore Data Path, and the source list at All Items. Up three times, one at a time with a zoom after each (TallyDrive (Cloud Storage), Select from Drive, then `Specify Path` highlighted); Enter: EXPECT Specify Data Path; type `C:`, the backslash key, `bridge-lab`, the backslash key, `2162-pa`, the backslash key, `backup-1` (zoom: that path, in Tally's own capitals); Enter: EXPECT the list to show BRIDGE PILOT LAB at the time recorded at step 1; Enter (selects it); Enter on `End of List`: EXPECT `Accept ?`: screenshot, click inside the box, `y`; EXPECT the prompt that the company data folder already exists, with Replace and Skip: screenshot, `r`; EXPECT Select Company and no other prompt. A prompt, list or screen not named here: path rule, no key.
   - List the data folder after, as before. EXPECT a listing and `exit 0`; an error: record it and report (the restore has run).
     ```bash
     cd /c/bridge-lab/2162-pa && ls -la --time-style=full-iso "<company data folder>" > local/r-data-after.txt 2>&1; echo "exit $?" >> local/r-data-after.txt; tail -n 3 local/r-data-after.txt
     ```
   - Load the book by the Select Company keys. EXPECT the Gateway and the starting date and period. Send `restored`: `M-marks.xml rb.xml`. EXPECT `count19.py --same-row` to pass: the pinned GUID, V 68, M 245, the start's values (not shown); and `rbcmp.py show` of the read-back `ROWS 0` (not shown: the restored book does not hold the invoice; this is also the row's only raw proof that the book before the post did not). Marks not back to the start: STOP and report; nothing more is restored.
     ```bash
     cd /c/bridge-lab/2162-pa && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 restored M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && sha256sum -c local/rb.sha256 && python -I run.py http://127.0.0.1:9001 restored rb.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-restored-M-marks.xml > evidence/restored-count19.txt 2>&1; echo "exit $?" >> evidence/restored-count19.txt; cat evidence/restored-count19.txt
     cd /c/bridge-lab/2162-pa && python -I -X utf8 rbcmp.py show a-restored-rb.xml > evidence/restored-rb.txt 2>&1; echo "exit $?" >> evidence/restored-rb.txt; cat evidence/restored-rb.txt
     ```
6. **Close.**
   - With the company still loaded, send `close`: `M-marks.xml`. Then type exactly one of the two blocks below: **after R**, or **without R**.
     ```bash
     cd /c/bridge-lab/2162-pa && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 close M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     ```
   - **After R (the book restored).** EXPECT `count19.py --same-row` to pass: the pinned GUID, V 68, M 245, the start's values (not shown). Not so: STOP and report.
     ```bash
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-close-M-marks.xml > evidence/close-count19-restored.txt 2>&1; echo "exit $?" >> evidence/close-count19-restored.txt; cat evidence/close-count19-restored.txt
     ```
   - **Without R (no RESTORE in the GO).** EXPECT `count19.py --before-write` to pass and print the pinned GUID with ALTVCHID (V) 69 and ALTMSTID (M) 245 against the start's 68 and 245 (code `count19.py:492`; lab 2162-SALES-Q: V 68 to 69, M 245 unchanged across its post), and write in the notes that the book holds the posted voucher. `--before-write` compares the GUID only and exits 0 whatever V and M are, so read them from its printed line. **V other than 69 or M other than 245: STOP and report**; if nothing was posted (part 6c's STOP, or no post sent), V 68 and M 245 are expected instead.
     ```bash
     cd /c/bridge-lab/2162-pa && python -I -X utf8 count19.py --before-write a-start-M-marks.xml a-close-M-marks.xml > evidence/close-count19-kept.txt 2>&1; echo "exit $?" >> evidence/close-count19-kept.txt; cat evidence/close-count19-kept.txt
     ```
   - Close by C10: Tally's own Shut Company by the keys above, then C2 a and b again (one `tally.exe` owning 9001), GET /status 200, a screenshot with no company loaded and the current date unchanged, and no `bridge_mcp.exe` running. Send no XML after the shut. Record the menu path and key used.
     ```bash
     cd /c/bridge-lab/2162-pa && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > evidence/c10-tasklist.txt 2>&1; echo "exit $?" >> evidence/c10-tasklist.txt; cat evidence/c10-tasklist.txt
     cd /c/bridge-lab/2162-pa && netstat -ano > evidence/c10-netstat.txt 2>&1; echo "exit $?" >> evidence/c10-netstat.txt; grep -E ':9001 |^exit' evidence/c10-netstat.txt
     cd /c/bridge-lab/2162-pa && curl.exe -sS -w "%{http_code} %{size_download}\n" http://127.0.0.1:9001/status -o evidence/c10-status-body.txt > evidence/c10-status.txt 2>&1; echo "curl exit $?" >> evidence/c10-status.txt; cat evidence/c10-status.txt
     cd /c/bridge-lab/2162-pa && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/c10-bridge.txt 2>&1; echo "exit $?" >> evidence/c10-bridge.txt; cat evidence/c10-bridge.txt
     ```

## Record

- Every raw answer with bytes and SHA-256 under the kept-local rule; every `run.py` log; every helper output whole, as the blocks write them; every structural diff and truncated-output count.
- Every key and screen title for G and R, with UTC times, and the screenshots under the kept-local rule.
- The grant step: the list before, the request and its answer, the list after.
- For LPA: the folder check, the two hashes, the process lists before and after, the start commands and shell, the whole `client-log.txt`, every answer with SHA-256 under the kept-local rule, every `calls.py show` output, `grade05.py`'s output, the batch output and the data folder's listing, `evidence/rbreq-out.txt`, `evidence/pa-out.txt` and `evidence/pa-times.txt` whole, `evidence/pa-rbcmp.txt`, the replay, and the server error log's size; how many post calls were made and each one's approval state.
- BK1 as C5 requires; the restore as C6 requires.
- A one-line arm table in the sitting notes: arm; state before (plain or grown, by sha256); the act; its result (`VERDICT`, the post's dispatch state and counters, `ITEM 2` line); marks before and after; or the reason it was not run.

## Decides

All outcomes are **partial**: one book, one release, one sitting, one invoice, one post.

- **P0 plain and G grown, PA `posted_verified` with created 1, V 68 to 69, and `ITEM 2 EQUAL`:** Q1: in the grown state the build's whole post path went through, from the re-read before the box to its own read-back, and verified the invoice in the book. Q2: the status the build read inside the call (inferred from its verdict) equals what the same request read raw at about +5 s and +60 s, in this sitting.
- **PA `ITEM 2 NOT EQUAL` after `posted_verified`:** the status Tally gave the invoice moved between the build's read and a raw read: the moved fields are reported as `rbcmp.py`'s FINDING lines print them; Q1's path still went through.
- **PA `posted_divergent`:** the post went in, and the build's own read-back found a difference (its codes recorded): Q1's path did not end verified; the two raw reads are still sent, and Q2 compares the codes only (the run coordinator's ruling).
- **PA refused with no box:** the re-read before the box refused in the grown state; Q1 is answered no at that stage, with its code.
- **PA refused after Yes:** the re-read after the answer or the queue's own re-read (`admit_queued_invoice`) refused; the two give the same codes and are not told apart here.
- **PA guard or check not passed:** nothing was pressed; the box's state or text is reported as read; Q1 is not answered.
- **PA `05` refused:** the build or the state differed from row 2162-RC's; nothing was posted; Q1 and Q2 are not answered.
- **G not grown, or `PA-pre` not grown:** the row's premise failed (at G: nothing was launched; at `PA-pre`: the build ran, nothing was posted); nothing is decided.
- **Marks not V +1, M unchanged after the post, or any raw read-back `ROWS` other than 1 after it:** the post added other than one voucher, or something else wrote. Stop and report; do not explain it as a display effect.
- **R's marks not back to the start, or its read-back not `ROWS 0`:** the restore did not return the book to BK1. Stop and report.

## Not covered

- The build's own read-back bytes and answer: not kept by the build; the raw request is a port, equal to the build's by the pinned self-test and the same inputs (inferred), and the build's status is inferred from its verdict.
- The build's marks read before its read-back and its month reads after it: not repeated raw.
- The exact time of the build's own read inside the call; the raw reads are timed from the moment `pa.py` read the final answer whole.
- A raw read-back of the invoice before the post: none is sent (the run coordinator's ruling). The post's premise, the grown rate state, is read just before it (`PA-pre`, part 6c); the book's lack of the invoice before the post is shown by R's read of the restored book (`ROWS 0`) and by V 68 at the start.
- The box between the guard's read and the click: `box.ps1` clicks the Yes button's centre a few seconds after the guard read the window, without reading it again. That no window covers the box in between rests only on nobody acting at the PC (Launch, departure 3): relied on, not checked.
- Whether `box.ps1`'s own GDI copy of the screen, taken before the window read, affects the box: not measured (Launch, departure 4).
- Whether the post changes the rate answer's state: recorded at `PA-post`, not graded.
- A post in the plain state; another invoice, book, release or sitting; a goods or tax ledger in the grown state; how long the grown state lasts.
- The approval box under the screen tool: no screen-tool screenshot is taken while it is up, so whether one would minimise it is not measured here (row 2569-S measured it for another build).

## Close and evidence

C10 at step 6 leaves one Tally with no company loaded. The synthetic book stays on the lab PC. C11 applies: evidence goes to a new answers branch only after the pre-push scan and the run coordinator's yes. The sitting's RESULTS gives Q1 and Q2 first, each with its confidence, then P0, G, PA and R in order: the state by sha256, the build's verdict, the box's guard and check, the post's calls, dispatch state and counters, the `ITEM 2` line with every FINDING, the verify status, the marks, and each arm not run with its reason. Quote only grading lines from kept-local files.
