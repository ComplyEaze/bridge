# Row 2162-RW: what a GSTR-1 walk changes in the two service ledgers' rate rows

For #2162, after row 2162-SALES-Q found `invoice_sales_ledger_rate_shape_unmeasured` only after a person walked GSTR-1. One short sitting, one synthetic book, one backup, no keyed data, no import and no post. The only acts on the book are the backup at step 1 and, only if arm A4 runs, one restore from it. Written 23 Nov 2026. Until the row runs, every EXPECT is **unverified** unless its source says otherwise.

**How an EXPECT is marked.** **(code `file:line`)** is a rule read in this worktree; **(lab `row`)** is an earlier observation; **(not shown)** is the thing this sitting measures. A `ratecmp.py` exit 2 is a failed read, never “no difference”. Exit 0 with `SHAPE unmeasured`, `READ refused` or a diff is a valid finding. One limit: the build ignores a CDATA section and `ratecmp.py` reads it as text, so a CDATA section seen in a raw answer is reported, not graded.

## The question and the one variable

The request is the build's own GUID-scoped rate request for the invoice of call `05`. The build scopes it by every ledger the invoice names (code `src-tauri/src/agent_import_invoice.rs:2356-2362`), six here: `BRIDGE Walk-in`, `BRIDGE Svc 998313 5%`, `BRIDGE Svc 998314 5%`, `BRIDGE CGST 2.5%`, `BRIDGE SGST 2.5%`, `BRIDGE Round Off`. The refusal under study names the two service ledgers; the other four rows are read and recorded because the build reads them in the same answer. `ratereq.py --selftest` proves its renderer against the build's pinned UTF-8-text SHA-256. The raw send uses the same HTTP entity encoding and content type as the build.

- **A0:** no report opened since the company load. Two consecutive reads test whether the read itself changes the answer. A build-only launch follows. A third read, after that launch, is the A0 baseline used by every later comparison. It prevents “build launch” and “GSTR-1 opening” becoming one combined change.
- **A1a, A1b, A1c:** add, in order, opening GSTR-1; opening its HSN summary; opening the two codes' voucher lists. These are cumulative arms, not independent fresh states. Only the first arm that differs after a quiet predecessor isolates the added screen act.
- **A2, A3, A4:** add, only while the refusal persists, shut-and-load; close-and-start Tally; restore BK1. Each has a rate read before its build check and another after it. A build check therefore cannot silently become part of the cure.
- Every arm ends with `M-marks.xml`, the arm's named rate answer, and `ratecmp.py` against A0. Every build check is build-only. Nothing is typed into the book. Only calls `01`, `02`, `03`, `05`, `07` are used.

`SHAPE measured` means only that every child is in the three lists at `src-tauri/src/agent_import_invoice_wire.rs:1467-1475` and every slab list is empty, the loop at `:1555-1575`. It is not the whole Sales admission rule. In particular, a second GST row leaves this SHAPE verdict measured but `sales_rate_milli` refuses history at `src-tauri/src/agent_import_invoice.rs:465-479`.

`READ refused` is one of the parser's own stops (a row whose `BRIDGECOMPANYGUID` is not the book's, a repeated field or name, a rate detail with no head: `src-tauri/src/agent_import_invoice_wire.rs:1537-1615`); the build then answers another code, not the one under study. `ratecmp.py` mirrors those stops and the shape rule and nothing else. The refusal code in the `05` answer is always graded on its own, and where the raw read and the build disagree both are written down.

## Book

`BRIDGE PILOT LAB` (synthetic), GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, loaded alone on TallyPrime 7.2 Silver. `LAB-COMMON.md` v3, byte-equal to the previous sheet's, applies in full; the stricter text wins.

- **Pinned by GUID.** `count19.py` and `calls.py` hold the GUID. `02-list_companies.json` must name it. Any other GUID: STOP, whatever the displayed name.
- **Starting state.** EXPECT ALTVCHID 68, ALTMSTID 245, current date 15-Sep-2026, period 1-Apr-26 to 31-Mar-27 (lab 2162-SALES-Q). Any difference: STOP before the backup or a launch.
- **BK1 is the way back.** Make it at step 1 in the new empty `C:\bridge-lab\2162-rw\backup-1`, by C5. A4 is the only planned restore. No other restore is made unless a mark moved and the run coordinator orders it.
- Record the release shown on Tally's screen.

## Build

The same NEW build as the previous sheet, already on the lab disk and granted:

NEW RC COMMIT: `8c3fb9f2277966fa98b31bdc775b65c5ea3c29f7`  ZIP DIGEST: `f8dc02e1f7bbb134e5ecc929c1f567aa265df2d3d1c866fea6636270e52e9472`  MCPB archive_sha256: `445d45a6fe18035258f69f04c7538241f19c80a39ad9ad88165b8a7c9f15ac8f`  EXE SHA256: `1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`

EXE: `C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`. Hash it before every launch. A different hash: STOP. Never copy, move, replace, download or build the exe.

Each committed file under `launch\` has exactly the previous sheet's five settings: writes enabled (the invoice builder refuses otherwise), terms accepted, host `127.0.0.1`, port `9001`, and its own new empty data folder under `C:\bridge-lab\2162-rw\`. The four forbidden override keys are absent; `client.py` refuses them. Copy the launch file byte for byte and check its hash against `SHA256.txt`. No script writes a launch file.

## Launches

At most five launches: L0 (`run-0`, `data-0`), L1C (`run-1c`, `data-1c`), L2 (`run-2`, `data-2`), L3 (`run-3`, `data-3`), L4 (`run-4`, `data-4`). Start each from the same shell:

`python -I C:\bridge-lab\2162-rw\client.py <RUNDIR> <EXE> 1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`

For each launch X:

1. `Test-Path` its run and data folders: EXPECT `False` for both. Otherwise STOP; never reuse a launch folder. Make `inbox\`, `answers\` and the data folder with `New-Item -ItemType Directory`.
2. Copy `launch\X-launch-env.json` to `<RUNDIR>\launch-env.json`; EXPECT its SHA-256 to equal `SHA256.txt`. Record the command, shell and process list.
3. EXPECT `client-log.txt` within 60 s to show the exe hash ok, the copied launch file's same hash, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` version 0.7.0. A `stop:` line or no initialization: STOP.
4. Put `calls\01-list.json`, `02-list_companies.json`, `03-marks.json`, `05-build-0074-0801.json`, `07-marks.json` into `inbox\` **one at a time**, only after the preceding answer and log line exist. Each goes in under a name ending `.part` and is then renamed to its own name, after its SHA-256 is checked against `SHA256.txt`: the client opens a name ending in .json at once, and a plain copy was once opened half-written (lab 2162-SALES-Q, deviation 4). Read each with `python -I calls.py show <RUNDIR>\answers\<name>.json`. Its STOP/exit 2 is a failed call. A refusal is an answer. No other call file is in this folder; none is written, edited or brought from another sheet, and in particular no post or verify call.
5. EXPECT `01` to lack none of the six tools `calls.py` checks; `02` one company with the pinned GUID; `03` and `07` M 245, V 68, reused False. The arm's step grades `05`.
6. Close by creating `<RUNDIR>\inbox\stop`. EXPECT the log to end `session ended; server pid <pid> exit code 0`, with that pid gone. Never end a process. A server that does not end: STOP and report.
7. No raw request while an exe runs, and no exe while a raw request or screen act is in progress. Record other Bridge processes; none may call port 9001 during this sitting.

The build call is the previous sheet's invoice (two service ledgers, six entries) dated 2026-08-01, number `BP/26-27/0074`. It writes its batch only inside that launch's data folder. No call sends it to Tally. `07` proves the book's marks after the build.

## Raw encoding and bounded output

`run.py` is copied unchanged. It sends `Content-Type: text/xml; charset=utf-16` with `curl.exe --data-binary` (`run.py:22-23`). `rates.xml` and `guids.xml` are BOM-prefixed UTF-16LE with no added newline. This matches the build's entity encoder (`src-tauri/crates/bridge-tally-protocol/src/text_encoding.rs:40-48`) and its prepare path (`src-tauri/crates/bridge-tally-transport/src/lib.rs:365-396`). The build explicitly sets the same content type and the encoded body length (`src-tauri/crates/bridge-tally-transport/src/lib.rs:948-963`). Curl derives that same Content-Length from the file. `run.py` records curl's exit code and the answer's size; it does not record the HTTP status line or the answer's Content-Length header that C3 names (`--fail-with-body` turns a status of 400 or more into a curl failure). The build expects UTF-16LE answers (`src-tauri/crates/bridge-tally-transport/src/lib.rs:101-103`); `ratecmp.py` also admits UTF-8 for saved answers.

The HTTP clients are not claimed identical: the build also has its own timeout, wire lock and other client headers; `run.py` deliberately keeps the previous lab helper's no-timeout, one-at-a-time curl behaviour. The request XML, entity bytes, content type and content length are the parts matched here.

Every raw answer is written whole to `a-<label>-<request>`; a curl failure, empty answer, `Unknown Request`, or body over 2,000,000 bytes stops `run.py`. Check each static request against `SHA256.txt` before sending. Check generated `rates.xml` against its recorded hash before every send. Never pipe an answer through a truncating reader.

`ratecmp.py` prints at most 50 ledgers, 40 GST rows, 40 state blocks, 40 slab lists and 40 rate details per containing item, 100 child elements per child list, 20 refusal reasons per ledger, and 200 structural-diff lines. Every truncation prints the exact count omitted. It still computes all reasons within the reported ledgers. `selftest-rate.py` prints one line per case. `calls.py` and `client.py` keep their previous bounds.

## What stays on the lab PC

As in the previous sheet, keep these local and list each in `kept-local.txt` by name, bytes and SHA-256:

- BK1 (`TDBK1800_<company number>.001`) and every backup/restore directory listing; quote no company number in the notes.
- `restore-plan.txt` and company-folder listings. An evidence copy replaces the number with `<company number>`; the original stays local.
- Every saved Bridge answer and server error log until scanned: answers can carry `company_number`. Evidence copies replace that value with `redacted`; originals stay local.
- Every screenshot. Record its name, bytes and SHA-256 only.
- Any raw answer that the pre-evidence scan finds holding a company number or data outside the named synthetic lab family.

On the evidence branch put the raw answers admitted by that scan, every `ratecmp.py`, `count19.py`, `calls.py` and self-check output, launch logs, call files, redacted answers, and the kept-local list.

## Files

Byte-equal copies from `lab/2162-sales-q/`: `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml`, `run.py`, `client.py`, `calls.py`, `count19.py`, and the five call files under `calls\`. The previous sheet has no request returning the ledgers' GUID, NAME and PARENT, so none is reused for that purpose.

Written for this row:

- `ratereq.py`: the standard-library port. `--output` writes the exact Rust request text as the build's BOM-prefixed UTF-16LE entity. `--selftest` hashes the Rust string's UTF-8 bytes, as `agent_import_request_bytes_tests.rs:627-630` does.
- `guids.xml`: the same collection and fetch list with one fixed `$Name` OR formula for the six ledgers of call `05`, replacing the GUID formula and filter name. It returns their NAME attribute and GUID/PARENT elements for `ratecmp.py` to print. The exact six-name request is unmeasured until step 1; it reads nothing else by design.
- `ratecmp.py`: the parser's read stops, the shape report and a structural diff. It removes and counts XML-forbidden control references before parsing; it does not treat a failed answer as an empty collection.
- `pilot-lab-ledger-rates.utf16le.xml`: a byte-equal copy of the repository capture `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates.utf16le.xml`, the fixture loaded by `agent_import_invoice_tests.rs:521-531` and `agent_import_invoice_wire_tests.rs:1371`. It was captured on this book and holds both service ledgers, each `SHAPE measured`.
- `selftest-rate.py`: 19 cases, the copy above and mutations of it made at run time.
- `check_sheet.py`: checks sheet file references and every `SHA256.txt` row.
- `launch\L0-launch-env.json`, `L1C-`, `L2-`, `L3-`, `L4-`: the five launch configurations.
- `RUN-SHEET.md` and `SHA256.txt`.

At the sitting, `ratereq.py` generates `rates.xml`; `run.py` generates `a-*` answers and `log-*.txt`; launches generate their run/data folders; C5/C6 generate backup/restore evidence. These are outputs, not missing sheet inputs.

## Self-checks before the sitting

Run from `C:\bridge-lab\2162-rw`. On the lab PC the interpreter is `python` (3.10); `python3` there is the Store alias and is not used (lab 2162-SALES-Q, deviation 1).

- `python -I ratereq.py --selftest`: EXPECT `96b41d40c58d9527da459219d4ea1b020b3c7e6688053ce3de7a42af892cb812` then `equal to the pinned hash`.
- `python -I selftest-rate.py`: EXPECT `19 cases, 19 right`, exit 0. Its cases are written to a temporary folder of the system's and removed.
- `python -I -c "import ast,sys;[ast.parse(open(f).read()) for f in sys.argv[1:]]" *.py`: EXPECT no output, exit 0.
- `python -I check_sheet.py`: EXPECT every referenced sheet input present and every manifest row matched, exit 0.
- No Tally request, `client.py` launch, or restore is part of these self-checks.

## Steps

Raw reads use `python -I run.py http://127.0.0.1:9001 <label> <files>`, then the named parser. A STOP from either stops the row. Record each whole output. After every raw send, take the whole-window screenshot required by C3 before the next send or key.

**Prompt rule (C7/C8).** No prompt is expected in these arms except C5's backup and C6's restore prompts. Any other dialog, message or half-entered screen: screenshot, copy its text, press no key, send nothing, STOP and report. Never Escape at Select Company or the Gateway. Never end a process. Nothing is restored after a STOP without the run coordinator's word.

**Path rule.** The screen after each key must be the one named. If not, screenshot and STOP with no further key. After a send or screen act, five minutes with no answer or change is a STOP; never resend.

0. **Before the sitting.**
   - Copy the folder byte for byte to `C:\bridge-lab\2162-rw`; run all four Self-checks. Hash the EXE. EXPECT all checks above. Any mismatch: STOP.
   - Make new `evidence\`; `Test-Path` first EXPECT `False`. Keep command outputs there. `run-*`, `data-*`, `backup-1` and `scratch` must also be absent before their step makes them.
   - C2 step 0 with no company loaded. EXPECT one Tally owning 9001, GET /status 200, no dialog. Otherwise STOP.
   - **Keys for steps 6 and 7.** Find in the operator's own record (a) the menu path and key used for C10's Shut Company at the close of row 2162-SALES-Q, and (b) any ordinary quit-and-start sequence for Tally that an earlier row used. Send both to the run coordinator and wait for the reply before step 1. The reply names the Shut Company path and key for step 6 and says whether A3 runs and with which keys. No reply: do not start.
1. **Load, check, back up, and get the six GUIDs.**
   - Load BRIDGE PILOT LAB alone from Select Company (C1): type `BRIDGE PILOT` until it is the one highlighted line, Enter. No Escape at Select Company. EXPECT the Gateway, current date and period in Book. Otherwise STOP.
   - Send `start`: `M-marks.xml`; parse with `python -I count19.py a-start-M-marks.xml`. EXPECT one row, the pinned GUID, V 68, M 245. Otherwise STOP before BK1.
   - Make `backup-1` new and empty; take BK1 by C5. Record its masked file name, bytes, SHA-256 and Restore-list time. EXPECT exactly one non-empty backup file. Otherwise STOP.
   - Send `setup`: `M-marks.xml guids.xml`, in that order. EXPECT marks still 68/245; `count19.py --same-row a-start-M-marks.xml a-setup-M-marks.xml` passes. A moved mark: STOP and report.
   - `python -I ratecmp.py a-setup-guids.xml`. EXPECT `LEDGER ROWS 6` and exactly the six names of call `05`, each with a non-empty GUID and a `BRIDGECOMPANYGUID` equal to the book's GUID, the two service ledgers under `Sales Accounts` (not shown for this exact request). Copy the six GUIDs from this parsed output. The copied capture read the two service ledgers as the book's GUID followed by `-000000d1` (998313) and `-000000d2` (998314); a different GUID here is written down and used, it is not a STOP. Any other count or name, or a `READ refused` line: STOP.
   - Generate the one rate request used all sitting (the build window is financial-year start through the invoice date, `agent_import_invoice_wire.rs:1209-1212`):
     `python -I ratereq.py --company "BRIDGE PILOT LAB" --from-date 20260401 --to-date 20260801 --output rates.xml GUID1 GUID2 GUID3 GUID4 GUID5 GUID6`
     Replace the last six arguments with the six parsed GUIDs, in any order; do not type a name into the XML. Record `rates.xml`'s bytes and SHA-256. `ratereq.py` sorts and de-duplicates GUIDs as the Rust type does.
2. **A0: loaded, no report since load; two reads, build check, baseline.**
   - With no key pressed and no report opened between them (the C3 screenshot after each send is still taken), send `A0-first`: `rates.xml`, then `A0-second`: `rates.xml`.
   - `python -I ratecmp.py a-A0-first-rates.xml a-A0-second-rates.xml`. EXPECT `LEDGER ROWS 6`, the six names, every one `SHAPE measured`, no structural difference, and **equal byte sizes and SHA-256** (not shown; this tests the read itself). A missing/other row, unequal hash, an unmeasured shape or a `READ refused` line: STOP.
   - Launch L0 (Launches). EXPECT `05` not refused, `voucher_count 1` (lab 2162-SALES-Q without a walk). Record batch id, SHA-256 and warnings. Any refusal: close and STOP; the baseline does not reproduce the earlier admitted state.
   - Close L0. End A0 with `A0`: `M-marks.xml rates.xml`, in that order. EXPECT marks equal to `start`; `count19.py --same-row a-start-M-marks.xml a-A0-M-marks.xml` passes.
   - `python -I ratecmp.py a-A0-first-rates.xml a-A0-rates.xml`. EXPECT all six ledgers measured in both answers and no structural difference. Record both hashes. A difference means the launch itself changed the answer: STOP; later screen arms would be confounded. `a-A0-rates.xml` is the baseline for every later comparison.
3. **A1a: open GSTR-1 only.**
   - At the Gateway (screenshot): click `G: Go To`; type `GSTR-1`; EXPECT List of Reports with no highlighted line; Down once (`GSTR-1` highlighted); Enter (lab 2162-SALES-Q step 12.1). Any list, prompt or other screen: path rule; do not select anything.
   - EXPECT `GSTR-1` with the highlight on Total Vouchers. Write down the period shown; the previous sitting's was 1-Aug-26 to 31-Aug-26 with no period key pressed (lab 2162-SALES-Q 15.2). Press no period key. A different period is written down and the arm goes on; a later step whose named line then does not show falls under the path rule. Look at no sub-report. Esc once; EXPECT the Gateway. No further Escape.
   - End the arm with `A1a`: `M-marks.xml rates.xml`. EXPECT marks equal to `start`. Then `python -I ratecmp.py a-A0-rates.xml a-A1a-rates.xml`. What it shows is what the sitting measures (not shown); nothing is expected either way. Record every added/removed/text-changed path and each of the six verdicts. A difference here localizes to opening GSTR-1, after the quiet A0 baseline.
4. **A1b: open GSTR-1 and its HSN summary only.**
   - Repeat A1a's four opening keys to `GSTR-1`. With the highlight on Total Vouchers (where GSTR-1 opens; if it is elsewhere, path rule), press Down 24 times, one at a time, with a zoom after each (the 24 lines passed are named in order in `lab/2162-sales-q-answers` at 84b7caf4, its s15-screens evidence file, 15.4): EXPECT `HSN Summary - 12 (B2B - B2C Supplies)` alone highlighted; Enter. A line not in that order, or no highlight: path rule. Open no code.
   - Esc once: EXPECT `GSTR-1`. Esc once more: EXPECT the Gateway. No further Escape.
   - End with `A1b`: `M-marks.xml rates.xml`; marks must equal `start`. Run `ratecmp.py a-A0-rates.xml a-A1b-rates.xml`. Record all output. If A1a was quiet, a first difference here localizes to opening the HSN summary. If A1a already differed, this arm is cumulative and cannot isolate an added effect.
5. **A1c: the whole two-code walk, then the build check.**
   - Open GSTR-1 and the HSN summary as A1b. Down 2 from 847130 to `998313`; Enter: EXPECT `GSTR-1 - Voucher Register`, `Vouchers of 998313` (lab 2162-SALES-Q step 12.5). Esc once: EXPECT the summary, highlight 998313.
   - Down once to `998314`; Enter: EXPECT its voucher list. Esc once: EXPECT the summary, highlight 998314. Esc once: EXPECT `GSTR-1`. Esc once: EXPECT the Gateway. No further Escape.
   - Before the launch, send `A1c-pre`: `M-marks.xml rates.xml`; marks equal `start`. Run `ratecmp.py a-A0-rates.xml a-A1c-pre-rates.xml`. Record the structural difference and the exact outside child or non-empty slab behind every unmeasured verdict.
   - Launch L1C. Grade the pre-read and `05` together:
     - the pre-read shows a service ledger `SHAPE unmeasured` and `05` is `refused invoice_sales_ledger_rate_shape_unmeasured` (lab 2162-SALES-Q after the same walk; code `agent_import_invoice.rs:478-479`): the candidate holds. Close, finish this arm's read below, go on to A2.
     - the pre-read shows all six `SHAPE measured` and `05` is admitted with `voucher_count 1`: the refusal did not appear in this sitting. Close, finish this arm's read below, skip A2-A4 and go to Close. A result, not a failed check.
     - any other pair (measured beside a refusal; unmeasured or `READ refused` beside an admission; another refusal code): record both, close, finish this arm's read if C8 permits, then STOP and report. The cure arms then run only on the run coordinator's word.
   - Close L1C. End with `A1c`: `M-marks.xml rates.xml`; marks equal `start`; `ratecmp.py a-A0-rates.xml a-A1c-rates.xml`. EXPECT the same shape and structure as `A1c-pre`; a change caused by the launch is a STOP because the cure arms would be confounded.
**Clearing rule (A2, A3; A4 uses its first sentence only).** An arm has cleared the refusal only when its pre-read shows all six ledgers `SHAPE measured` with no structural difference from A0 **and** its launch's `05` is admitted with `voucher_count 1`. It has not cleared it only when its pre-read still shows A1c's difference **and** `05` is still `refused invoice_sales_ledger_rate_shape_unmeasured`. Cleared: close the launch, finish the arm's final read, skip the later cure arms, go to Close. Not cleared: close, finish the final read, go to the next cure arm (A2 to A3, or to A4 when no keys were named for A3; A3 to A4). Any other pair: record both, close, finish the final read if C8 permits, then STOP and report.

6. **A2: shut the company and load it again, only after A1c's target refusal.**
   - At the Gateway use Tally's own **Shut Company** as C10 requires. LAB-COMMON names no fixed shortcut, so this sheet names none: use the menu path and key named in the run coordinator's reply at step 0 and no other, and write them into this row's notes before the first key. EXPECT Select Company with no company loaded. Any `Quit ?` or other prompt: prompt rule.
   - Load the book alone by step 1's Select Company keys. EXPECT the Gateway and starting date/period. No GSTR-1 screen is opened.
   - Send `A2-pre`: `M-marks.xml rates.xml`; marks equal `start`. Run `ratecmp.py a-A0-rates.xml a-A2-pre-rates.xml`. EXPECT all six ledgers measured in both answers and no structural difference if shut-and-load clears the walk state (not shown). Record the actual result.
   - Launch L2 and grade it by the clearing rule. A3 runs **only if** A2 has not cleared the refusal by that rule.
   - Close L2. End with `A2`: `M-marks.xml rates.xml`; marks equal `start`; `ratecmp.py a-A0-rates.xml a-A2-rates.xml`. EXPECT the same structure as `A2-pre`; otherwise STOP.
7. **A3: close and start Tally, only if A2 has not cleared the refusal by the clearing rule.**
   - LAB-COMMON C9 says how to reopen after Tally has closed, but gives no ordinary close/start key sequence. A3 runs only if the run coordinator's reply at step 0 named the close and start keys for it. If it named none, A3 is not run: write `A3 not run: no keys named` in the notes and go to A4, the planned restore. Never end the process by name, pid or window control.
   - Using the sequence that reply named, close Tally and start it once. If a company loads automatically, shut it by Tally's own Shut Company. Load BRIDGE PILOT LAB alone. Run C2's checks and record the Gateway date/period. This planned restart is the coordinator-authorised resume for C9; do not repeat a completed screen act.
   - Send `A3-pre`: `M-marks.xml rates.xml`; marks equal `start`. Run `ratecmp.py a-A0-rates.xml a-A3-pre-rates.xml`. EXPECT all six ledgers measured in both answers/no difference if restart clears the state (not shown). Record the actual result.
   - Launch L3 and grade it by the clearing rule. After an A3 that ran, A4 runs **only if** A3 has not cleared the refusal by that rule.
   - Close L3. End with `A3`: `M-marks.xml rates.xml`; marks equal `start`; `ratecmp.py a-A0-rates.xml a-A3-rates.xml`. EXPECT the same structure as `A3-pre`; otherwise STOP.
8. **A4: restore BK1, only if A3 has not cleared the refusal by the clearing rule, or A2 has not cleared it and A3 was not run for want of keys.**
   - Verify BK1's folder, masked file name, bytes, SHA-256 and Restore-list time. Write `restore-plan.txt` and the company-folder listing; shut the company; restore by C6. EXPECT only `Accept ?` = Yes and the named existing-folder prompt = Replace. Any other prompt or a missing Replace prompt: no key, STOP.
   - Load the book alone. Before any request take C6's two screenshots. EXPECT the Gateway, 15-Sep-2026, period 1-Apr-26 to 31-Mar-27 (lab 2162-SALES-Q final restore). Otherwise STOP.
   - Send `A4-pre`: `M-marks.xml rates.xml`, MARKS first as C6 requires. EXPECT `count19.py --same-row a-start-M-marks.xml a-A4-pre-M-marks.xml` passes with V 68, M 245. Run `ratecmp.py a-A0-rates.xml a-A4-pre-rates.xml`; EXPECT all six ledgers measured in both answers and no structural difference. Any mark or rate difference: STOP and report before a launch (so L4 never runs on a book whose rate rows still differ; a difference that outlives a restore is the finding).
   - Launch L4. EXPECT `05` admitted, `voucher_count 1` (the restore's purpose in the previous row): A4 has cleared the refusal by the clearing rule's first sentence. There is no later cure arm, so any other outcome is a result to record, then close and STOP.
   - Close L4. End with `A4`: `M-marks.xml rates.xml`; marks equal `start`; `ratecmp.py a-A0-rates.xml a-A4-rates.xml`. EXPECT measured/no structural difference and the same structure as `A4-pre`. Otherwise STOP.
9. **Close.**
   - If A1c was admitted, A2-A4 were skipped. If A2 or A3 admitted, later cures were skipped. If A4 ran, its restore is the only restore made.
   - With the company still loaded, send `close`: `M-marks.xml`. `python -I count19.py --same-row a-start-M-marks.xml a-close-M-marks.xml`: EXPECT pass, V 68, M 245. Nothing was keyed, so when no A4 ran and this passes, **no restore is needed**. If a mark moved anywhere, STOP and report; any restore is only on the coordinator's word.
   - Close by C10: Tally's own Shut Company, one tally.exe owning 9001, GET /status 200, no company loaded, current date unchanged. Send no XML after the shut. Record the menu path and key used.

## Record

- Every raw answer whole, with bytes and SHA-256; every `run.py` log; every `ratecmp.py` and `count19.py` output whole. Record control references removed, every structural diff, and every truncated-output count.
- Every key and screen title for A1a/A1b/A1c, with UTC times and screenshots under the kept-local rule. Do not substitute earlier counts for what this sitting shows.
- For each launch: folder checks, EXE hash, launch-file hash, start command/shell/process list, whole `client-log.txt`, every answer with SHA-256 under the kept-local rule, every `calls.py show` output, and server-error-log size.
- BK1 and A4 as C5/C6 require. The Restore-list timestamp, masked backup name, bytes and SHA-256 identify the one source.
- A one-line arm table in the sitting notes: arm; first differing path or `none`; each ledger's SHAPE verdict; `05` result; marks before/after; cure first observed clear or not.

## Decides

All outcomes are **partial**: one book, one release, one build, one sitting.

- **A0 first and second byte-equal:** this raw read alone changed nothing. A0's post-launch baseline also equal in structure means the later first diff is not merely the L0 launch.
- **First difference in A1a:** opening GSTR-1 is sufficient. **First in A1b after a quiet A1a:** opening the HSN summary is the added trigger. **First in A1c-pre after quiet A1a/A1b:** opening a code's voucher list is the added trigger; this arm opens both codes, so it does not distinguish 998313 from 998314. If an earlier arm already differed, later cumulative arms do not isolate another cause.
- **What changed:** the structural diff's added, removed and text-changed paths answer that byte/structure question. The verdict's outside child name or non-empty slab answers the build-refusal question. A byte-only difference outside those rules does not explain `invoice_sales_ledger_rate_shape_unmeasured`. A second GST row alone points instead to `invoice_sales_ledger_rate_history_unmeasured`.
- **A1c admitted with a measured pre-read:** the refusal did not appear in this sitting. That does not refute the earlier sitting, where each refused build followed a restore from the backup and then a walk in the same Tally session; this sitting walks with no restore before it. Record the raw state; no cure arm runs. **A1c target refusal with an unmeasured raw row:** the walk and the build's rule agree on the measured shape; the named diff is the candidate cause. A target refusal while `ratecmp.py` says measured is a contradiction to investigate, not proof of the candidate, and the row stops there (step 5).
- **First clear at A2:** shutting/loading clears the state without a Tally restart. **First clear at A3:** process restart is needed beyond company reload. **Clear only at A4:** only the tested backup restore cleared it among these arms. “Clear” requires both the arm's measured/no-baseline-diff rate answer and an admitted `05`; the final read must not show the build reinstating the difference. If A3 was not run, a clear at A4 does not separate a restart of Tally from a restore. A4 itself is a shut, a restore and a load: it adds the restore to what A2 already tried.
- **What the arms do not separate:** A1a and A1b have no build check, so the refusal itself is seen only at A1c and the two earlier arms speak through the raw read alone. A2, A3 and A4 each follow a build launch; each arm's final read is there to show that the launch changed nothing.
- **Marks moved:** the sitting touched more than its read-only design permits. Stop and report; do not explain it as a GSTR-1 display effect.

## Close and evidence

C10 at step 9 leaves one Tally with no company loaded. The synthetic book stays on the lab PC. C11 applies: evidence goes to a new answers branch only after the pre-push scan and coordinator approval. The sitting's RESULTS gives A0, A1a, A1b, A1c and each attempted cure in order, with confidence, exact first diff, build answer, marks, and every skipped arm's reason. Quote only grading lines from kept-local files.
