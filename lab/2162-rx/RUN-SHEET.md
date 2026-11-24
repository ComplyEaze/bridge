# Row 2162-RX: which screen act grows which ledger's rate rows

For #2162, after row 2162-RW found that opening the voucher lists of both service codes from GSTR-1's HSN summary grew both service sales ledgers' rate answer, and that Shut Company and a reload put it back. One short sitting, one synthetic book, one backup, no keyed data, no import and no post. The only act on the book is the backup at step 1; no restore is planned. Written 24 Nov 2026. Until the row runs, every EXPECT is **unverified** unless its source says otherwise.

**How an EXPECT is marked.** **(code `file:line`)** is a rule read in this worktree; **(lab `row`)** is an earlier observation; **(not shown)** is the thing this sitting measures. A `ratecmp.py` exit 2 is a failed read, never “no difference”. Exit 0 with `SHAPE unmeasured`, `READ refused` or a diff is a valid finding. One limit: the build ignores a CDATA section and `ratecmp.py` reads it as text, so a CDATA section seen in a raw answer is reported, not graded.

## The questions and the one variable per arm

The request is the build's own GUID-scoped rate request for the invoice of call `05`, made exactly as row 2162-RW made it. The build scopes it by every ledger the invoice names (code `src-tauri/src/agent_import_invoice.rs:2356-2362`), six here: `BRIDGE Walk-in`, `BRIDGE Svc 998313 5%`, `BRIDGE Svc 998314 5%`, `BRIDGE CGST 2.5%`, `BRIDGE SGST 2.5%`, `BRIDGE Round Off`. `ratereq.py --selftest` proves its renderer against the build's pinned UTF-8-text SHA-256. The raw send uses the same HTTP entity encoding and content type as the build.

| arm | the one act added to a fresh load | read | launched |
|---|---|---|---|
| A0 | none: loaded, no report | marks and rates twice; the sitting's first answer | nothing |
| C313 | GSTR-1, its HSN summary, the voucher list of 998313 only | marks and rates | nothing |
| C314 | GSTR-1, its HSN summary, the voucher list of 998314 only | pre-reads; marks and rates | nothing |
| LVS | Ledger Vouchers of `BRIDGE Svc 998313 5%`, only if step 0's reply named the keys | pre-reads; marks and rates | nothing |
| LVT | Ledger Vouchers of `BRIDGE CGST 2.5%`, only if step 0's reply named the keys | pre-reads; marks and rates | nothing |
| W1 | row 2162-RW's whole walk: both codes' voucher lists | pre-reads; marks and rates before and after the launch | L1C, expecting the refusal |
| W2 | Shut Company and a reload after W1 | pre-reads; marks and rates after the launch | L2, expecting the invoice admitted |

**Order.** C313 comes first because its keys are a strict prefix of row 2162-RW's recorded walk and it is the fewest screen acts among arms whose keys are recorded, so the first shut-and-reload check follows the smallest disturbance; W1 and W2 come last because they repeat row 2162-RW and alone launch the build, so no launch precedes a screen arm.

Every arm starts from a fresh load (A0's for C313; a Shut Company and a reload for the others), and every arm ends with `M-marks.xml`, the arm's named rate answer, and `ratecmp.py` against the sitting's first answer. Nothing is typed into the book. Only calls `01`, `02`, `03`, `05`, `07` are used.

`SHAPE measured` means only that every child is in the three lists at `src-tauri/src/agent_import_invoice_wire.rs:1467-1475` and every slab list is empty, the loop at `:1555-1575`. It is not the whole Sales admission rule. In particular, a second GST row leaves this SHAPE verdict measured but `sales_rate_milli` refuses history at `src-tauri/src/agent_import_invoice.rs:465-479`.

`READ refused` is one of the parser's own stops (a row whose `BRIDGECOMPANYGUID` is not the book's, a repeated field or name, a rate detail with no head: `src-tauri/src/agent_import_invoice_wire.rs:1537-1615`); the build then answers another code, not the one under study. `ratecmp.py` mirrors those stops and the shape rule and nothing else. The refusal code in the `05` answer is always graded on its own, and where the raw read and the build disagree both are written down.

## Book

`BRIDGE PILOT LAB` (synthetic), its GUID pinned in `count19.py` and `calls.py`, loaded alone on TallyPrime 7.2 Silver. `LAB-COMMON.md` v3, byte-equal to row 2162-RW's, applies in full; the stricter text wins.

- **Pinned by GUID.** `count19.py` and `calls.py` hold the GUID. `02-list_companies.json` must name it. Any other GUID: STOP, whatever the displayed name.
- **Starting state.** EXPECT ALTVCHID 68, ALTMSTID 245, current date 15-Sep-2026, period 1-Apr-26 to 31-Mar-27 (lab 2162-SALES-Q; lab 2162-RW). Any difference: STOP before the backup or a launch.
- **BK1 is the way back.** Make it at step 1 in the new empty `C:\bridge-lab\2162-rx\backup-1`, by C5. No restore is planned. No restore is made unless a mark moved and the run coordinator orders it.
- Record the release shown on Tally's screen.

## Build

The same NEW build as row 2162-RW, already on the lab disk and granted:

NEW RC COMMIT: `8c3fb9f2277966fa98b31bdc775b65c5ea3c29f7`  ZIP DIGEST: `f8dc02e1f7bbb134e5ecc929c1f567aa265df2d3d1c866fea6636270e52e9472`  MCPB archive_sha256: `445d45a6fe18035258f69f04c7538241f19c80a39ad9ad88165b8a7c9f15ac8f`  EXE SHA256: `1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`

EXE: `C:\bridge-lab\2569-r\exe-new\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe`. Hash it before every launch. A different hash: STOP. Never copy, move, replace, download or build the exe.

The two files under `launch\` are row 2162-RW's `L1C` and `L2` files with one value changed: the data folder is under `C:\bridge-lab\2162-rx\` instead of row 2162-RW's folder, whose `data-1c` and `data-2` already exist. Each has exactly row 2162-RW's five settings: writes enabled (the invoice builder refuses otherwise), terms accepted, host `127.0.0.1`, port `9001`, and its own new empty data folder. The four forbidden override keys are absent; `client.py` refuses them. Copy the launch file byte for byte and check its hash against `SHA256.txt`. No script writes a launch file.

## Launches

At most two launches: L1C (`run-1c`, `data-1c`) in W1 and L2 (`run-2`, `data-2`) in W2. No other arm launches the build. Start each from the same shell:

`python -I C:\bridge-lab\2162-rx\client.py <RUNDIR> <EXE> 1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`

For each launch X:

1. `Test-Path` its run and data folders: EXPECT `False` for both. Otherwise STOP; never reuse a launch folder. Make `inbox\`, `answers\` and the data folder with `New-Item -ItemType Directory`.
2. Copy `launch\X-launch-env.json` to `<RUNDIR>\launch-env.json`; EXPECT its SHA-256 to equal `SHA256.txt`. Record the command, shell and process list.
3. EXPECT `client-log.txt` within 60 s to show the exe hash ok, the copied launch file's same hash, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` version 0.7.0. A `stop:` line or no initialization: STOP.
4. Put `calls\01-list.json`, `02-list_companies.json`, `03-marks.json`, `05-build-0074-0801.json`, `07-marks.json` into `inbox\` **one at a time**, only after the preceding answer and log line exist. Each goes in under a name ending `.part` and is then renamed to its own name, after its SHA-256 is checked against `SHA256.txt`: the client opens a name ending in .json at once, and a plain copy was once opened half-written (lab 2162-SALES-Q, deviation 4). Read each with `python -I calls.py show <RUNDIR>\answers\<name>.json`. Its STOP/exit 2 is a failed call. A refusal is an answer. No other call file is in this folder; none is written, edited or brought from another sheet, and in particular no post or verify call.
5. EXPECT `01` to lack none of the six tools `calls.py` checks; `02` one company with the pinned GUID; `03` and `07` M 245, V 68, reused False. The arm's step grades `05`.
6. Close by creating `<RUNDIR>\inbox\stop`. EXPECT the log to end `session ended; server pid <pid> exit code 0`, with that pid gone. Never end a process. A server that does not end: STOP and report.
7. No raw request while an exe runs, and no exe while a raw request or screen act is in progress. Record other Bridge processes; none may call port 9001 during this sitting.

No approval box appears in this row: no call asks for one. If an approval box appears: STOP, press nothing in it, and report.

The build call is row 2162-RW's invoice (two service ledgers, six entries) dated 2026-08-01, number `BP/26-27/0074`. It writes its batch only inside that launch's data folder. No call sends it to Tally. `07` proves the book's marks after the build.

## Raw encoding and bounded output

`run.py` is copied unchanged. It sends `Content-Type: text/xml; charset=utf-16` with `curl.exe --data-binary` (`run.py:22-23`). `rates.xml` and `guids.xml` are BOM-prefixed UTF-16LE with no added newline. This matches the build's entity encoder (`src-tauri/crates/bridge-tally-protocol/src/text_encoding.rs:40-48`) and its prepare path (`src-tauri/crates/bridge-tally-transport/src/lib.rs:365-396`). The build explicitly sets the same content type and the encoded body length (`src-tauri/crates/bridge-tally-transport/src/lib.rs:948-963`). Curl derives that same Content-Length from the file. `run.py` records curl's exit code and the answer's size; it does not record the HTTP status line or the answer's Content-Length header that C3 names (`--fail-with-body` turns a status of 400 or more into a curl failure). For this row that part of C3 is not asked for, as in row 2162-RW: no second request is sent to obtain a status line or a header; a status of 400 or more, an empty answer or a curl failure still stops `run.py`. The build expects UTF-16LE answers (`src-tauri/crates/bridge-tally-transport/src/lib.rs:101-103`); `ratecmp.py` also admits UTF-8 for saved answers.

The HTTP clients are not claimed identical: the build also has its own timeout, wire lock and other client headers; `run.py` deliberately keeps the previous lab helper's no-timeout, one-at-a-time curl behaviour. The request XML, entity bytes, content type and content length are the parts matched here.

Every raw answer is written whole to `a-<label>-<request>`; a curl failure, empty answer, `Unknown Request`, or body over 2,000,000 bytes stops `run.py`. Check each static request against `SHA256.txt` before sending. Check generated `rates.xml` against its recorded hash before every send. Never pipe an answer through a truncating reader.

`ratecmp.py` prints at most 50 ledgers, 40 GST rows, 40 state blocks, 40 slab lists and 40 rate details per containing item, 100 child elements per child list, 20 refusal reasons per ledger, and 200 structural-diff lines. Every truncation prints the exact count omitted. It still computes all reasons within the reported ledgers. `selftest-rate.py` prints one line per case. `calls.py` and `client.py` keep their previous bounds.

## What stays on the lab PC

As in row 2162-RW, keep these local and list each in `kept-local.txt` by name, bytes and SHA-256:

- BK1 (`TDBK1800_<company number>.001`) and every backup/restore directory listing; quote no company number in the notes.
- `restore-plan.txt` and company-folder listings. An evidence copy replaces the number with `<company number>`; the original stays local.
- Every saved Bridge answer and server error log until scanned: answers can carry `company_number`. Evidence copies replace that value with `redacted`; originals stay local.
- Every screenshot. Record its name, bytes and SHA-256 only.
- Any raw answer that the pre-evidence scan finds holding a company number or data outside the named synthetic lab family.

On the evidence branch put the raw answers admitted by that scan, every `ratecmp.py`, `count19.py`, `calls.py` and self-check output, launch logs, call files, redacted answers, and the kept-local list.

## Files

Byte-equal copies from `lab/2162-rw/`: `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml`, `run.py`, `client.py`, `calls.py`, `count19.py`, `ratereq.py`, `ratecmp.py`, `guids.xml`, `pilot-lab-ledger-rates.utf16le.xml`, and the five call files under `calls\`.

Changed for this row:

- `launch\L1C-launch-env.json`, `L2-`: row 2162-RW's two files, the data folder moved under this row's folder.
- `selftest-rate.py`: row 2162-RW's 19 cases plus four that grow, as mutations of the capture (not captures), the rate rows of 998313 only, 998314 only, both (row 2162-RW's `added 28`), and `BRIDGE CGST 2.5%` only.
- `check_sheet.py`: row 2162-RW's checks, plus the arm names: every arm is in the table, in Steps and in Decides; every compared answer has a send; no arm or launch of row 2162-RW that this row removed is named; no GUID-shaped text.
- `RUN-SHEET.md` and `SHA256.txt`.

At the sitting, `ratereq.py` generates `rates.xml`; `run.py` generates `a-*` answers and `log-*.txt`; launches generate their run/data folders; C5 generates backup evidence. These are outputs, not missing sheet inputs.

## Self-checks before the sitting

Run from `C:\bridge-lab\2162-rx`. On the lab PC the interpreter is `python` (3.10); `python3` there is the Store alias and is not used (lab 2162-SALES-Q, deviation 1).

- `python -I ratereq.py --selftest`: EXPECT `96b41d40c58d9527da459219d4ea1b020b3c7e6688053ce3de7a42af892cb812` then `equal to the pinned hash`.
- `python -I selftest-rate.py`: EXPECT `23 cases, 23 right`, exit 0. Its cases are written to a temporary folder of the system's and removed.
- `python -I -c "import ast,sys;[ast.parse(open(f).read()) for f in sys.argv[1:]]" *.py`: EXPECT no output, exit 0.
- `python -I check_sheet.py`: EXPECT every referenced sheet input present, every manifest row matched and the arm-name line, exit 0.
- No Tally request, `client.py` launch, or restore is part of these self-checks.

## Steps

Raw reads use `python -I run.py http://127.0.0.1:9001 <label> <files>`, then the named parser. Where a send names two files, make one `run.py` call per file, in the order given and under the same label, so that the screenshot named below falls between them (row 2162-RW did so). A STOP from either stops the row. Record each whole output. After every raw send, take the whole-window screenshot required by C3 before the next send or key.

**Prompt rule (C7/C8).** No prompt is expected in these arms except C5's backup and C6's restore prompts. Any other dialog, message or half-entered screen: screenshot, copy its text, press no key, send nothing, STOP and report. Never Escape at Select Company or the Gateway. Never end a process. Nothing is restored after a STOP without the run coordinator's word.

**Path rule.** The screen after each key must be the one named. If not, screenshot and STOP with no further key. After a send or screen act, five minutes with no answer or change is a STOP; never resend.

**Keys.** The keys for Select Company, GSTR-1, the HSN summary, the two voucher lists and Shut Company are the keys recorded in row 2162-RW's notes, written out below; use them and no others. **Shut Company**, at the Gateway: click a blank part of the Gateway's left panel; Alt+K; EXPECT the K: Company menu; mouse click on SHut; EXPECT the Shut Company box listing BRIDGE PILOT LAB alone, highlighted; Enter; EXPECT Select Company, no company loaded, no prompt. A list with more than one line, or any prompt: no key, STOP. **Open the HSN summary**: at the Gateway (screenshot) click `G: Go To`; type `GSTR-1`; EXPECT List of Reports with no highlighted line; Down once (`GSTR-1` highlighted); Enter. Any list, prompt or other screen: path rule; do not select anything. EXPECT `GSTR-1` with the highlight on Total Vouchers. Write down the period shown (row 2162-RW: 1-Aug-26 to 31-Aug-26). Press no period key. A different period is written down and the arm goes on; a later step whose named line then does not show falls under the path rule. Look at no sub-report. With the highlight on Total Vouchers (where GSTR-1 opens; if it is elsewhere, path rule), press Down 24 times, one at a time, with a zoom after each (the 24 lines passed, in the order row 2162-RW's notes record: 1 Included in Return, 2 Ready for Upload, 3 Modified in Books After Upload/Export, 4 Not Relevant for This Return, 5 Uncertain Transactions (Corrections needed), 6 Check Vouchers Having Potential Conflicts with Masters, 7 B2B Invoices - 4A, 4B, 4C, 6B, 6C, 8 B2C (Large) Invoices - 5A, 5B, 9 Exports Invoices - 6A, 10 Credit or Debit Notes (Registered) - 9B, 11 Credit or Debit Notes (Unregistered) - 9B, 12 Amended B2B Invoices - 9A, 13 Amended B2C (Large) Invoices - 9A, 14 Amended Exports Invoices - 9A, 15 Amended Credit or Debit Notes (Registered) - 9C, 16 Amended Credit or Debit Notes (Unregistered) - 9C, 17 B2C (Small) Invoices - 7, 18 Nil Rated Invoices - 8A, 8B, 8C, 8D, 19 Amendment B2C (Small) Invoices - 10, 20 Tax Liability (Advances Received) - 11A(1), 11A(2), 21 Adjustment of Advances - 11B(1), 11B(2), 22 Amended Tax Liability (Advances Received) - 11A, 23 Amendment of Adjusted Advances - 11B, where the list scrolls, 24 the HSN Summary line): EXPECT `HSN Summary - 12 (B2B - B2C Supplies)` alone highlighted; Enter; EXPECT the summary with the highlight on 847130. A line not in that order, or no highlight: path rule.

**Arm frame.** Steps 3 to 8 name the parts of this frame they run. C313 (step 3) skips a and b, since A0's load and reads stand for them.

- a. Shut Company by the keys above. Any `Quit ?` or other prompt: prompt rule. Load the book alone by step 1's Select Company keys. EXPECT the Gateway and starting date/period. Otherwise STOP. No other screen is opened.
- b. Send `<arm>-pre`: `M-marks.xml rates.xml`, then `<arm>-pre2`: `rates.xml`, with no key between them. `count19.py --same-row a-start-M-marks.xml a-<arm>-pre-M-marks.xml` must pass. A moved mark: STOP and report. Run `ratecmp.py a-<arm>-pre-rates.xml a-<arm>-pre2-rates.xml` and record it. Lab 2160-L1 saw such a difference for voucher types; row 2162-RW saw none here. A difference between the two is a first-read-after-load effect: write it down; it is not a STOP. A `READ refused` line or an exit 2 on either is a STOP as everywhere. Then `ratecmp.py a-A0-first-rates.xml a-<arm>-pre2-rates.xml`. The state has cleared when all six ledgers are `SHAPE measured` and there is no structural difference. If the state does not clear after Shut Company and a reload, that is a STOP with no further act.
- c. The step's screen act, and only that.
- d. Send `<arm>`: `M-marks.xml rates.xml`. `count19.py --same-row a-start-M-marks.xml a-<arm>-M-marks.xml` must pass; a moved mark: STOP and report. Run `ratecmp.py a-A0-first-rates.xml a-<arm>-rates.xml` and record its whole output, each of the six verdicts, the three diff counts, and which ledgers the differing paths fall under.

**Outcome rule (C313, C314, LVS, LVT).** What `ratecmp.py` shows at d is the measurement (not shown). No outcome of the rate answer is a STOP; only an exit 2 (a failed read) is. This read at d of these four arms is the one place where a `READ refused` line is not a STOP. EXPECT, and write in the arm table, exactly one of: `none` (no structural difference, all six measured); `998313` (paths under `BRIDGE Svc 998313 5%` only); `998314` (under `BRIDGE Svc 998314 5%` only); `both` (under those two only); `CGST` (under `BRIDGE CGST 2.5%` only); `other` (any other set of ledgers, written out), each with the verdicts and the diff counts. A `READ refused` verdict is recorded with its code.

0. **Before the sitting.**
   - Copy the folder byte for byte to `C:\bridge-lab\2162-rx`; run all four Self-checks. Hash the EXE. EXPECT all checks above. Any mismatch: STOP.
   - Make new `evidence\`; `Test-Path` first EXPECT `False`. Keep command outputs there. `run-*`, `data-*`, `backup-1` and `scratch` must also be absent before their step makes them.
   - C2 step 0 with no company loaded. EXPECT one Tally owning 9001, GET /status 200, no dialog. Otherwise STOP.
   - **Keys for LVS and LVT.** No keys are recorded for Tally's Ledger Vouchers display. Write down the key sequence you propose, from the Gateway: Display More Reports > Account Books > Ledger, the choice of the one named ledger, and the Escapes back to the Gateway, with the screen title EXPECTED after each key. Press none of them to find out. Send it to the run coordinator and wait for the reply before step 1. The reply is a new commit on this sheet's branch that writes the keys into steps 5 and 6 (fetch it, check the folder against its `SHA256.txt` again and run the Self-checks again), or a message that names none; keys that arrive any other way are not used. It may name keys for one of the two arms only: the other is then not run. No reply: do not start.
1. **Load, check, back up, and get the six GUIDs.**
   - Load BRIDGE PILOT LAB alone from Select Company (C1): click the filter; type `BRIDGE`, space, `PILOT` until it is the one highlighted line, Enter. No Escape at Select Company. EXPECT the Gateway, current date and period in Book. Otherwise STOP.
   - Send `start`: `M-marks.xml`; parse with `python -I count19.py a-start-M-marks.xml`. EXPECT one row, the pinned GUID, V 68, M 245. Otherwise STOP before BK1.
   - Make `backup-1` new and empty; take BK1 by C5, with the keys row 2162-RW's notes record. Click the Gateway's left panel; Alt+Y: EXPECT the Data menu; Enter (Backup & Restore); Enter (Backup): EXPECT the Backup screen. In Backup Data Path press Backspace: EXPECT the List of Folders; Up three times, one at a time with a zoom after each (row 2162-RW passed TallyDrive, Select from Drive, then `Specify Path` highlighted); Enter; type the full path of `backup-1` (zoom: exact); Enter: EXPECT the list with that folder highlighted; Enter: EXPECT the path to read that folder. In the company filter type `BRIDGE`, space, `PILOT`: EXPECT one line, BRIDGE PILOT LAB, highlighted; Enter; click `End of List` (zoom: highlighted); Enter. EXPECT `Accept ?` showing that path and the one company: screenshot, click inside the box, `y`. EXPECT the Gateway and no message. A line, list or prompt other than these: path rule. Record its masked file name, bytes, SHA-256 and Restore-list time. EXPECT exactly one non-empty backup file. Otherwise STOP.
   - Send `setup`: `M-marks.xml guids.xml`, in that order. EXPECT marks still 68/245; `count19.py --same-row a-start-M-marks.xml a-setup-M-marks.xml` passes. A moved mark: STOP and report.
   - `python -I ratecmp.py a-setup-guids.xml`. EXPECT `LEDGER ROWS 6` and exactly the six names of call `05`, each with a non-empty GUID and a `BRIDGECOMPANYGUID` equal to the book's GUID, the two service ledgers under `Sales Accounts` (lab 2162-RW). Copy the six GUIDs from this parsed output; a GUID other than row 2162-RW's is written down and used, it is not a STOP. Any other count or name, or a `READ refused` line: STOP.
   - Generate the one rate request used all sitting (the build window is financial-year start through the invoice date, `agent_import_invoice_wire.rs:1209-1212`):
     `python -I ratereq.py --company "BRIDGE PILOT LAB" --from-date 20260401 --to-date 20260801 --output rates.xml GUID1 GUID2 GUID3 GUID4 GUID5 GUID6`
     Replace the last six arguments with the six parsed GUIDs, in any order; do not type a name into the XML. Record `rates.xml`'s bytes and SHA-256. `ratereq.py` sorts and de-duplicates GUIDs as the Rust type does.
2. **A0: loaded, no report since load; the sitting's first answer.**
   - With no key pressed and no report opened between them (the C3 screenshot after each send is still taken), send `A0-first`: `M-marks.xml rates.xml`, then `A0-second`: `rates.xml`. `count19.py --same-row a-start-M-marks.xml a-A0-first-M-marks.xml` must pass. A moved mark: STOP and report.
   - `python -I ratecmp.py a-A0-first-rates.xml a-A0-second-rates.xml`. EXPECT `LEDGER ROWS 6`, the six names, every one `SHAPE measured`, no structural difference, and **equal byte sizes and SHA-256** (lab 2162-RW). A missing/other row, unequal hash, an unmeasured shape or a `READ refused` line: STOP, with the one exception below.
   - The exception: if both answers show `LEDGER ROWS 6`, the six names, every one `SHAPE measured` and no `READ refused` line, and the only failure is that the two differ, send `A0-third`: `rates.xml`, once. If `a-A0-second-rates.xml` and `a-A0-third-rates.xml` are equal in bytes and SHA-256, record a first-read effect with the whole `ratecmp.py` output of first against second, and use `a-A0-second-rates.xml` wherever this sheet names `a-A0-first-rates.xml`. If second and third differ too: STOP.
3. **C313: the voucher list of 998313 only.** Frame c: open the HSN summary (Keys). Down twice, one at a time with a zoom after each (9983, then `998313` alone highlighted); Enter: EXPECT `GSTR-1 - Voucher Register`, `Vouchers of 998313`; write down its line count and totals (row 2162-RW: 19 lines). Esc once: EXPECT the summary, highlight 998313. Esc once: EXPECT `GSTR-1`. Esc once: EXPECT the Gateway. No further Escape. Frame d; grade by the outcome rule.
4. **C314: the voucher list of 998314 only.** Frame a and b. Frame c: open the HSN summary. Down three times, one at a time with a zoom after each (9983, 998313, then `998314` alone highlighted). These are the summary's lines in the order row 2162-RW's notes record; that row pressed Down twice, Enter and Esc on 998313, then Down once, so three Downs in a row were not pressed there. Do not press Enter on 998313. Enter on 998314: EXPECT `Vouchers of 998314`; write down its line count and totals (row 2162-RW: 7 lines). Esc once: EXPECT the summary, highlight 998314. Esc once: EXPECT `GSTR-1`. Esc once: EXPECT the Gateway. No further Escape. Frame d; grade by the outcome rule.
5. **LVS: Ledger Vouchers of `BRIDGE Svc 998313 5%`.** If step 0's reply named no keys for LVS, write `LVS not run: no keys named` in the notes, make no shut or load for it, and go to step 6. Otherwise frame a and b. Frame c: from the Gateway (screenshot), the reply's keys and only those, to Ledger Vouchers for that ledger. Press no period key; write down the period, the number of voucher lines and the totals shown. Once Ledger Vouchers shows, press Enter on no line of it; an Enter the reply's keys name on the way there, to choose the ledger, is one of those keys. Open no voucher, in alteration mode or any other; accept no screen. Esc back to the Gateway by the reply's keys; no further Escape. A screen the reply did not name: path rule. Frame d; grade by the outcome rule.
6. **LVT: Ledger Vouchers of `BRIDGE CGST 2.5%`.** As LVS, with that ledger; with no keys named for LVT, write `LVT not run: no keys named` and go to step 7.
7. **W1: row 2162-RW's whole walk, then the build expecting the refusal.**
   - Frame a and b. Frame c: open the HSN summary. Down twice from 847130, one at a time with a zoom after each (9983, then `998313` alone highlighted); Enter: EXPECT `GSTR-1 - Voucher Register`, `Vouchers of 998313` (lab 2162-SALES-Q step 12.5; lab 2162-RW step 5). Esc once: EXPECT the summary, highlight 998313. Down once (zoom: `998314` alone highlighted); Enter: EXPECT `Vouchers of 998314`. Esc once: EXPECT the summary, highlight 998314. Esc once: EXPECT `GSTR-1`. Esc once: EXPECT the Gateway. No further Escape.
   - Frame d (`W1`). Record the word the outcome rule would give; row 2162-RW saw `both`, 28 paths added (lab 2162-RW). A `READ refused` line here is recorded and graded with `05` below; it is not a STOP at d.
   - Launch L1C. Grade `W1` and `05` together, as row 2162-RW's step 5:
     - `W1` shows a service ledger `SHAPE unmeasured` and `05` is `refused invoice_sales_ledger_rate_shape_unmeasured` (lab 2162-RW; code `agent_import_invoice.rs:478-479`): the finding is seen again. Close, finish this arm's read below, go on to W2.
     - `W1` shows all six `SHAPE measured` and `05` is admitted with `voucher_count 1`: the walk did not grow the answer in this sitting. Close, finish this arm's read below, write `W2 not run: W1 admitted`, and go to step 9. A result, not a failed check.
     - any other pair (measured beside a refusal; unmeasured or `READ refused` beside an admission; another refusal code): record both, close, finish this arm's read if C8 permits, then STOP and report. W2 then runs only on the run coordinator's word.
   - Close L1C. Send `W1-post`: `M-marks.xml rates.xml`; marks equal `start`; `ratecmp.py a-W1-rates.xml a-W1-post-rates.xml`. EXPECT no structural difference; a change caused by the launch is a STOP because W2 would be confounded.
8. **W2: Shut Company and a reload, then the build expecting the invoice admitted.**
   - Frame a and b. Frame b's clear check is W2's first measurement: not cleared is the STOP it names.
   - Launch L2. EXPECT `05` admitted, `voucher_count 1` (lab 2162-RW step 6). Any other outcome is a result to record, then close and STOP.
   - Close L2. End with `W2`: `M-marks.xml rates.xml`; marks equal `start`; `ratecmp.py a-A0-first-rates.xml a-W2-rates.xml`, then `ratecmp.py a-W2-pre2-rates.xml a-W2-rates.xml`. EXPECT no structural difference in either. Otherwise STOP.
9. **Close.**
   - Write the arms not run and why. No restore was made.
   - With the company still loaded, send `close`: `M-marks.xml`. `python -I count19.py --same-row a-start-M-marks.xml a-close-M-marks.xml`: EXPECT pass, V 68, M 245. Nothing was keyed, so when this passes, **no restore is needed**. If a mark moved anywhere, STOP and report; any restore is only on the coordinator's word.
   - Close by C10: Tally's own Shut Company, one tally.exe owning 9001, GET /status 200, no company loaded, current date unchanged. Send no XML after the shut. Record the menu path and key used.

## Record

- Every raw answer whole, with bytes and SHA-256; every `run.py` log; every `ratecmp.py` and `count19.py` output whole. Record control references removed, every structural diff, and every truncated-output count.
- Every key and screen title for every arm, with UTC times and screenshots under the kept-local rule; for LVS and LVT the reply's keys, written into the notes before the first key. Do not substitute earlier counts for what this sitting shows.
- For each launch: folder checks, EXE hash, launch-file hash, start command/shell/process list, whole `client-log.txt`, every answer with SHA-256 under the kept-local rule, every `calls.py show` output, and server-error-log size.
- BK1 as C5 requires.
- A one-line arm table in the sitting notes: arm; outcome-rule word; first differing path or `none`; each ledger's SHAPE verdict; `pre` against `pre2`; `05` result where launched; marks before/after; or the reason it was not run.

## Decides

All outcomes are **partial**: one book, one release, one build, one sitting. Each arm differs from a fresh load by its one screen act, and its pre-read (for C313, A0's two reads on the same load) shows that the load before it was clean.

- **A0 first and second byte-equal:** the raw read alone changes nothing in this sitting. **They differ, second and third equal:** a first-read effect; later arms are graded against the second.
- **C313 `998313`:** opening one code's voucher list grows only that code's sales ledger. **`both`:** one code's list is enough to grow both service ledgers. **`none`:** the list of 998313 alone did not grow either ledger in this arm. Row 2162-RW's walk differed from this arm by more than the second list: a build launch came before it in the same session, and its Escapes ran from the second list. So `none` does not say which of those the growth needs. **`other`, `CGST`:** opening this list reaches ledgers other than its code's; the paths name which.
- **C314 `998314`, `both`, `none`, `other` or `CGST`:** the same four readings for the list of 998314. C313 and C314 read together: `998313` and `998314` mean each list grows its own ledger; `both` and `both` mean either list alone grows both; `none` and `none` mean neither list alone grew anything here; W1 then shows whether both lists in one session do, with no launch before them.
- **LVS `998313` or `both`:** a screen outside GSTR-1 that lists a ledger's vouchers grows the answer as well; the trigger is not GSTR-1's register alone. **LVS `none`:** Ledger Vouchers of that ledger does not grow its answer in this sitting. **LVS `other` or `CGST`:** the paths name the ledgers reached.
- **LVT `CGST`:** a tax ledger's answer can grow too. **`none`:** this display did not grow the tax ledger's answer in this sitting; it does not show that no screen does. **`998313`, `998314`, `both` or `other`:** listing the tax ledger's vouchers reached other ledgers. Whether the build refuses on a grown tax ledger is not measured: no launch follows LVT.
- **Any other word, or a `READ refused` verdict, in C313, C314, LVS or LVT:** recorded as it stands with its paths or code; it is a new finding about that one act and is not read as any line above.
- **LVS or LVT listed no vouchers:** that arm opened the display but listed nothing, so it decides nothing about listing vouchers; recorded as such.
- **LVS or LVT not run:** no keys were named; item 2 stays open.
- **W1 unmeasured with the target refusal:** row 2162-RW's finding is seen in a second sitting. **W1 measured and admitted:** the walk did not grow the answer this time. That does not refute row 2162-RW, whose walk followed GSTR-1 and HSN-summary openings and a launch in the same session; W1 follows a reload after other arms. **Mixed pair:** the raw read and the build disagree; the row stops there.
- **W2 cleared and admitted:** Shut Company and a reload clear the walk's state, again. **W2 cleared and refused:** the build's refusal outlives a clean raw read; a contradiction to report.
- **A state that does not clear after Shut Company and a reload:** for that arm's act, a reload is not enough; the row stops and every later arm is not run.
- **`pre` and `pre2` differ after a load:** the first ledger read after a load is not like the later ones on this book, as row 2160-L1 saw for voucher types. That is separate from the screen acts and is reported on its own.
- **Marks moved:** the sitting touched more than its read-only design permits. Stop and report; do not explain it as a display effect.

## Close and evidence

C10 at step 9 leaves one Tally with no company loaded. The synthetic book stays on the lab PC. C11 applies: evidence goes to a new answers branch only after the pre-push scan and coordinator approval. The sitting's RESULTS gives A0, C313, C314, LVS, LVT, W1 and W2 in order, with confidence, outcome word, exact first diff, build answer, marks, and every skipped arm's reason. Quote only grading lines from kept-local files.
