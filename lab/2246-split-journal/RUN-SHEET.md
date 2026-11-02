# 2246 split journal on a real Store install (#2246, #2233): which refusal each side of the package boundary gets through this build, and what an inside append does to a journal it did not create

Not for merge. About an hour. Nothing is written to Tally: no post reaches it, no ledger is created, nothing is keyed. `build_import_xml` reads Tally and writes an import file and a journal line on this PC. `SHA256.txt` lists every file here with its bytes; check it before step 0.

`LAB-COMMON.md` in this folder applies in full (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`). Where it and this sheet differ, the stricter holds. It runs alone, after the run coordinator's yes to download the build.

Marks: V = read at a named source; U = not measured. Every outcome is recorded as found, never explained away.

## What it decides

#2246 refuses a build or a post while a Store package's copy of the data folder holds a second import journal. Its tests stand in for Windows, and its body names four paths that could refuse EVERY build and post on a Store install, none of which has run on Windows. `lab/1584-msix-paths` ran before this code existed and left this PC with two journals, so the first row needs no setup.

**L1, the two journals already here:**
1. Can each side list `%LOCALAPPDATA%\Packages`? (`package_copies_not_listed`)
2. Does `std::fs::canonicalize` locate this side's journal? (`journal_not_located`)
3. Does the inside side take its own journal for another's? (`other_journal_holds_batches`, inside)
4. Does it locate the copy's journal? (`package_copy_journal_not_located`)
5. Is the outside side refused, for a build and for a post, with nothing written or sent?

**L2, a new data folder, from nothing:** when the outside side creates a journal and the inside side then appends to it, does the journal grow in place or does a package copy start? Then what does the next build on each side get?

**Limit, whatever the result:** one computer, one Store package, processes started from the lab shell and by WMI, and at L3 the extension as Claude Desktop starts it.

## Rules and company

**Who acts.** The lab does every step, on the owner's standing word of 27 and 28 Oct 2026, and records each as LAB-COMMON C11 directs.

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, the book of `lab/1584-msix-paths`. Only it is loaded, alone, from step 0 to the close. Licensed mode only; Education mode = STOP. Tally is only read, so no backup is taken.

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Tools only.** Requests to port 9001 are tool calls through `client.py`, one at a time, except C2's `GET /status` and MARKS. C2 d's MARKS read: use `ledger_masters` basic, first page, and record `snapshot.master_alter_id` and `snapshot.voucher_alter_id`, as this sheet sends no raw request.
2. **Tools sent:** `tools/list`, `tally_status`, `list_companies`, `ledger_masters`, `build_import_xml`, `local_data_report`, `egress_log`, and `post_import` ONLY at L1d. `client.py` refuses any other.
3. **No approval is given.** Every post here is expected to be refused before a box. If any approval box opens: screenshot, answer **No**, STOP. Never Yes.
4. **Delete nothing.** Built files, journal lines and probe files stay where they land. Claude Desktop is never quit, restarted or reconfigured, except L3's install.
5. **Existence** is decided by `probe.ps1`, never by `Test-Path`. An error is recorded as an error, never as "absent".
6. **Order matters.** Windows redirects only new files, so run the rows in the order written and stop each client before the next starts.
7. **A refusal is a result.** Record its `code`, `cause` and the whole answer. It ends the row only where the row says so.

## Names

- `LA` = `%LOCALAPPDATA%`; `F` = the Claude package family, read at step 1.
- `R` = `LA\Bridge\agent`; `Q` = `LA\Packages\F\LocalCache\Local\Bridge\agent`.
- `R2` = `LA\BridgeLab2246\agent`; `Q2` = `LA\Packages\F\LocalCache\Local\BridgeLab2246\agent`.
- `J` = `agent-import-ledger.jsonl`, the import journal.
- **in** = the lab session's shell, or a process it starts. **out** = a process started by WMI: write the commands to `C:\bridge-lab\2246\run\<step>-out.cmd` as ASCII and start it with `Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{CommandLine = 'cmd.exe /c <that file>'}`, as `lab/1584-msix-paths` did. Which side each really is on is decided at step 2, not assumed.
- **Probe line:** `powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\2246\probe.ps1 -Side <in|out> -Step <step> -Family F -Root C:\bridge-lab\2246` plus the step's own parameters.
- **A voucher** for a build: `{"company_guid": G, "vouchers": [{"bridge_txn_id": <id>, "date": <books_from as YYYY-MM-DD>, "voucher_type": "Journal", "entries": [{"ledger": "Cash", "amount": "1.00", "side": "Dr"}, {"ledger": "DN Sales", "amount": "1.00", "side": "Cr"}]}]}`, the pair `lab1584-i1` used on this book.
- **Launch settings** (`launch-env.json` in each row folder, UTF-8 without a BOM): `BRIDGE_TERMS_ACCEPTED` `true`, `BRIDGE_TALLY_HOST` `127.0.0.1`, `BRIDGE_TALLY_PORT` `9001`, `BRIDGE_AGENT_ENABLE_IMPORT` `true`. A row adds only what it names. `client.py` refuses any other setting.

## Build

Pull request #2246 at head `e9c780f3`. Workflow run `41184397066` (started by hand on the branch), artifact `unsigned-bundle-smoke-windows-latest` id `13036317536`, 58,176,284 bytes, zip digest `sha256:268783f86cc292de6e54c0703692f42e32cd95434aa19619b76ebf1d46f32063` (it expires 7 Nov 2026 08:38 UTC). Downloaded only on the run coordinator's yes. Check the zip's sha256 against that digest; another = STOP.

Find `bridge_mcp.exe` in the unpacked bundle (inside its `.mcpb`, itself a zip, under `bin\`). Copy it to `C:\bridge-lab\2246\bin\`, and write its sha256, lowercase and alone, to `C:\bridge-lab\2246\exe-sha256.txt`. Hash it again after the last call; another hash = STOP.

## The code admits each trigger

At `e9c780f3`, checked by reading, not by a run:
- **The check:** `admit_at` in `src-tauri/src/agent_import_split_journal.rs:113`. Its five outcomes are the causes at `:70-74`. `admit` (`:89`) passes `std::fs::canonicalize`, the call whose form K1 of `lab/1584-msix-paths` read as the real package path.
- **Path 1:** `package_copy_folders`, `src-tauri/src/agent_import_local_data.rs:497`, `read_dir` of `LA\Packages`.
- **L1 and L2 reach it because** each data folder is under `LA`: `relative_to` (`:469`) is what makes it `NotUnderLocalAppData` otherwise, and `BRIDGE_AGENT_DATA_DIR` is read at `src-tauri/src/agent.rs:568`.
- **A build:** `src-tauri/src/agent_import.rs:1277`, under the admission lock, after Tally's reads and before any file.
- **A post:** `src-tauri/src/agent_import_post.rs:848`, after the record's own checks and before any dialog. A batch built by 0.7.0 passes those record checks only if its file is unchanged and its record holds the fields they ask for; L1d records whichever code comes back.
- **The other journal has batches:** after `lab/1584-msix-paths`, `Q\J` held 37 batches and more, and `R\J` held one (`lab1584-o1`; V: `lab/1584-msix-paths-answers` `RESULTS.md`).

## Files

`client.py` (`lab/1584-msix-paths`'s, with this build's path, the settings rule above and two tools added), `probe.ps1` (byte copy of that branch's, sha256 `80a95d84`), `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Steps

### Start

0. LAB-COMMON C2 step 0 with THE COMPANY loaded alone. New folders `C:\bridge-lab\2246\{bin,run,evidence,l1-in,l1-out,l2-a,l2-b,l2-c,l2-d,l3-c}`, each `l*` with `inbox\` and `answers\`. Copy `client.py` and `probe.ps1` there, and write a new `ka.txt` there holding any one line (step 2's instrument). Record Windows' version and build, and `whoami /groups` from the lab shell: EXPECT a standard user, Medium Mandatory Level; record it either way.
1. `Get-AppxPackage | Where-Object Name -like '*Claude*'`: record `Name`, `PackageFamilyName` (= `F`), `Version`, `SignatureKind`. Not exactly one = STOP. `(Get-Command python).Source`: record it; if it is under `LA`, use the `Q`-side copy for out steps as `lab/1584-msix-paths` step 0e did.

### Which side is which

2. New nonces N1, N2 (8 lowercase hex digits each).
   - K1a (in): probe, `-Step K1a -WriteNonce N1`.
   - K1b (out): probe, `-Step K1b -WriteNonce N2`.
   - K1c (in): probe, `-Step K1c`.

   EXPECT, by `lab/1584-msix-paths`'s classification:
   - **out:** `package_family` `none`, and its new file in `LA\bridge-lab-1584`, not in the package's copy.
   - **in:** its new file seen from out only under `LA\Packages\F\LocalCache\Local\bridge-lab-1584`.
   - **The instrument:** a file at `C:\bridge-lab\2246\ka.txt` opens from both sides with the same `vol`, `id` and `final`.

   Any miss = STOP: every row below rests on it.
3. From K1b's listing, the journals: record bytes, `final`, `vol`+`id` for `R\J` and `Q\J`. EXPECT both present and different files, `Q\J` at least 174,198 bytes and `R\J` at least 1,074. Otherwise STOP and report: L1 was written for that state.

### L1: the two journals already here

4. **L1a (out):** `bridge_mcp.exe --local-data-report --show-paths > evidence\L1a-out.json 2> evidence\L1a-out.err`, with the exit code. EXPECT the data folder's `package_copies` to be a list holding `Q` (path 1, outside), never `"not_listed"`.
5. **L1b (in):** the same to `L1b-in.json`. EXPECT a list holding `Q`, never `"not_listed"` (path 1, inside).
6. **L1c (in):** folder `l1-in`, the launch settings alone. Start `"<python>" C:\bridge-lab\2246\client.py C:\bridge-lab\2246\l1-in in` from the lab shell. Requests one at a time:
   - `tools/list`: EXPECT `build_import_xml` and `egress_log`; `post_import` absent.
   - `list_companies`: EXPECT exactly THE COMPANY; G its `company_guid`.
   - `tally_status`: `books_from`.
   - `build_import_xml`, id `lab2246-l1-in`.

   **EXPECT** a `result.batch_id`, `result.path` under `Q`, `path_state` `resolved`: paths 2, 3 and 4 did not fire.

   **A refusal `import_journal_split` here is the row's main finding, and means a Store install's inside side can build nothing.** Record its `cause`: `journal_not_located` is path 2, `other_journal_holds_batches` path 3, `package_copy_journal_not_located` path 4, `package_copies_not_listed` path 1. Tell the run coordinator, then go on to L1d. Any other code naming the input (a ledger or the date): record it and STOP.

   Then `stop`, and record the exit code. Probe, `-Step L1c`: `Q\J` grew, `R\J` did not.
7. **L1d (out):** folder `l1-out`, the launch settings plus `BRIDGE_AGENT_ENABLE_WRITES` `true`, as an out step. O = `result.batch_id` of `C:\bridge-lab\1584\o1`'s build answer. Requests one at a time:
   - `tools/list`: EXPECT `post_import`.
   - `list_companies`; `ledger_masters` basic (marks M, V); `egress_log` (record the entries).
   - `build_import_xml`, id `lab2246-l1-out`. EXPECT `import_journal_split`, cause `other_journal_holds_batches`, no `result.path`.
   - `post_import`, `{"company_guid": G, "batch_id": O}`. EXPECT `import_journal_split`, the same cause, no box. Any other code: record it, whole.
   - `ledger_masters` basic: EXPECT M and V unchanged. `egress_log`: record the entries since the last read.

   Then `stop`. Probe out, `-Step L1d`: EXPECT `R\J` and `Q\J` at the bytes L1c left, and no new file in `R\imports`.

### L2: a new folder, from nothing

8. Write `run\l2-paths.txt` (UTF-8, no BOM), two lines: `R2\J`, `Q2\J`, each with `LA` and `F` expanded. Every probe in this part adds `-ExtraFile C:\bridge-lab\2246\run\l2-paths.txt`. Probe out, `-Step L2z`: EXPECT both absent. Present = STOP: the folder name has history.

   L2's launch settings add `BRIDGE_AGENT_DATA_DIR` = `R2` with `LA` expanded. Each row is one client, one `list_companies`, one build, then `stop`.
9. **L2a (out), folder `l2-a`:** build, id `lab2246-l2a`. EXPECT `result.batch_id`. Probe out then in, `-Step L2a`: EXPECT `R2\J` present, one file from both sides (same `vol`+`id`); `Q2\J` absent. Record the bytes.
10. **L2b (in), folder `l2-b`:** build, id `lab2246-l2b`. EXPECT `result.batch_id`: no copy holds a journal yet. Then `local_data_report` before `stop`. Probe out then in, `-Step L2b`. Record, for `R2\J` and `Q2\J`: present, bytes, `final`, `vol`+`id`; and which folder holds the new import file (`result.path`). The outcome is one of:
    - **L2-A:** `R2\J` grew and `Q2\J` is absent: the append went through to the real file.
    - **L2-B:** `Q2\J` exists and `R2\J` did not grow: Windows started a copy.
    - **L2-C:** anything else, described.
11. **L2c (out), folder `l2-c`:** build, id `lab2246-l2c`.
    - After L2-A: EXPECT `result.batch_id`, no second journal to find, even though `Q2` may now exist for the import file.
    - After L2-B: EXPECT `import_journal_split`, cause `other_journal_holds_batches`, and `R2\J` unchanged: the refusal from a clean start.
    - After L2-C: record it.
12. **L2d (in), folder `l2-d`:** build, id `lab2246-l2d`. EXPECT `result.batch_id` after either outcome: inside, the copy is this side's journal or no journal exists there. Then probe out then in, `-Step L2d`.

### L3: inside Claude Desktop as the extension

Owed by the review of #2246: the rows above start the program from a shell, and the extension is what a person runs. It runs last, and only on the run coordinator's word in this sitting, because it installs this build as the extension, the one exception to rule 4. Posting stays off, so a build is all that is sent. The check runs on a build alone (`agent_import.rs:1277`, under the admission lock), and L1c's batch means the extension's own journal `Q\J` already holds at least one.

13. Screenshot Desktop's extension page: record the installed version of ComplyEaze Bridge and its posting setting. Install the `.mcpb` from this build's bundle through Desktop's own install screen. A prompt to quit or restart Desktop = decline it, STOP, report. Then, from the process list, EXPECT a `bridge_mcp.exe` started by Desktop whose sha256 equals `exe-sha256.txt`, and the posting setting off (another hash or setting = STOP).
14. **L3a:** in a new test chat, send exactly `Call the ComplyEaze Bridge tool build_import_xml with these arguments, and nothing else:` followed by the voucher above, id `lab2246-l3a`, and allow that one call once. Take its result from Desktop's MCP log by the call's JSON-RPC id, not from the chat. **EXPECT** `result.batch_id`, no `import_journal_split`. **A refusal here is the lockout on a real install: record its `cause` whole, skip L3b, and #2246 stops.** No answer in 5 minutes, or no such line in the log, leaves L3 undecided: record it.
15. **L3b:** the same in the same chat, id `lab2246-l3b`. EXPECT `result.batch_id`, with L3a's batch now on record.
16. **L3c (out):** folder `l3-c`, the launch settings alone: one client, one `list_companies`, one build, id `lab2246-l3c`, then `stop`. EXPECT `import_journal_split`, cause `other_journal_holds_batches`. Probe out, `-Step L3c`: EXPECT `R\J` at the bytes step 7 left, and `Q\J` larger by L3a and L3b.
17. Put back the extension version step 13 recorded, from its own package. If that package is not on the PC, leave this build installed and say so.

### Close

18. Hash the exe. No `bridge_mcp.exe` of this item left running: `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'"`, recording `ProcessId` and `ExecutablePath`. LAB-COMMON C10.

**What this item leaves behind, never posted:** the `lab2246-*` batches and their journal lines in `Q`, `R2` and wherever L2 put them; two new folders; the `bridge-lab-1584` probe files. A later sheet's step 3 reads larger journals, and that is expected.

## Result

**PASS (the sheet ran):** every row answered and was recorded, including any refusal; no box opened; M and V did not move; no file was deleted.

**FAIL:** a transport failure; an approval box; a mark that moved; a client that sent anything twice; a probe error read as absent.

**The finding it records:**
- whether each of the four paths fired, per side;
- that a real outside side is refused, for a build and a post, or what it got instead;
- L2-A, L2-B or L2-C, which decides whether a person who posts from one side first can later be refused on the other;
- at L3, whether the extension itself builds, twice, with a process outside refused.

**Not measured:** Windows 11; a desktop app installed normally; two Store packages holding journals; the master-write journal; the under-lock check after an approval, since no box is answered Yes.

## Evidence

A new branch `lab/2246-split-journal-answers` (C11), after the pre-push scan and the run coordinator's yes: a `RESULTS.md` with the tables above, every answer, the probe JSON, `client-log.txt`, `notes.txt` with UTC times. Replace the home path with `<home>`, the package family with `<family>` and the desktop client's name with `<desktop client>` in every pushed file, as `lab/1584-msix-paths-answers` did; keep the originals on the lab PC, listed by sha256.
