# Row 2162-RC: the build with the rate-row change, launched in the grown state

For #2162. Master's change for that issue, #2593 (merged as commit 6f541ae4), lets the invoice build admit a sales ledger whose rate rows carry Tally's zero per-unit fields (protocol reference §12a.31, in this tree; §12a.32 is on the reference branch and not in this tree, so its record is cited here: `lab/2162-rx-answers` at 3c0f4bd3, RESULTS.md). This row asks one question on a live Tally: in the SAME grown state of Tally's rate answer, with no reload between, does BUILD 8c3fb9f2 (without the change) refuse the test invoice and BUILD 6f541ae4 (with the change) admit it? One sitting, one synthetic book, one backup, no keyed data, no import and no post. The builds only build: no call posts, no approval box opens, nothing is written to the book. The only act on the book is the backup at step 1; no restore is planned. Written 24 Nov 2026. Until the row runs, every EXPECT is **unverified** unless its source says otherwise.

**How an EXPECT is marked.** **(code `file:line`)** is a rule read at commit 6f541ae4; **(lab `row`)** is an earlier observation; **(not shown)** is the thing this sitting measures. A `ratecmp.py`, `count19.py`, `calls.py` or `grade05.py` exit 2 is a failed read, never "nothing found" or "no difference".

**The state is graded by bytes, not by `ratecmp.py`'s SHAPE verdict.** `ratecmp.py` is row 2162-RX's, byte for byte: its SHAPE rule copies the lists and loop of BUILD 8c3fb9f2's parse, and the line numbers in its comments, and in every other byte-equal helper, are that tree's, not this one's. In the grown state it prints `SHAPE unmeasured` for both service ledgers whichever build is launched; that is the expected reading beside BUILD 6f541ae4's admission, not a contradiction. The two states are named by their bytes (lab 2162-RW; lab 2162-RX):

- **plain**: 29,114 bytes, sha256 `a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62`;
- **grown**: 30,626 bytes, sha256 `d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257`.

Each is checked with `sha256sum -c` on the answer file. SHAPE lines are recorded whole and never graded against BUILD 6f541ae4.

## The questions and the one variable per arm

The request is the build's own GUID-scoped rate request for the invoice of call `05`, rendered by `ratereq.py` exactly as rows 2162-RW and 2162-RX rendered it (`rates.xml`, 2,874 bytes, sha256 `182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08`; lab 2162-RX). The invoice is the same call file as theirs: `BP/26-27/0074` of 2026-08-01, two service ledgers, six entries.

| arm | the state | the one thing launched | EXPECT |
|---|---|---|---|
| P | step 1's load, plain (two equal reads) | BUILD 6f541ae4 (launch LP) | `05` admitted, `voucher_count 1`; the read after it plain |
| G | Shut Company and a load, plain (two reads), then the W1 walk: both codes' voucher lists | nothing | the read after the walk grown |
| G1 | G's grown state, no act since G's read | BUILD 8c3fb9f2 (launch LG1) | `05` refused: two `invoice_sales_ledger_rate_shape_unmeasured`, one naming each service ledger, no file; the read after it grown, byte-equal |
| G2 | G's grown state, no act since G1's read | BUILD 6f541ae4 (launch LG2) | `05` admitted, `voucher_count 1`, no refusal; the read after it grown, byte-equal |
| C | Shut Company and a load, plain (two reads) | BUILD 6f541ae4 (launch LC) | `05` admitted, `voucher_count 1`; the read after it plain |

**The one variable.** P against G2: the same build, the state plain or grown. G1 against G2: the same state, the build without or with the change. The two builds differ by more than #2593: master moved 16 commits from 8c3fb9f2 to 6f541ae4. #2593 is the only one of them that changes `src-tauri/src/agent_import_invoice_wire.rs` or the non-test code of `src-tauri/src/agent_import_invoice.rs`, where the rate parse (`RowShape`, code `src-tauri/src/agent_import_invoice_wire.rs:1447-1466`) and the sales rate rule (`sales_rate_milli`, code `src-tauri/src/agent_import_invoice.rs:488-506`) live; others change `agent_import.rs`, `agent_import_post.rs` and `agent_import_verification.rs` (among them a build refusal for long e-invoice details, #2585, which this invoice does not carry). P and C show that BUILD 6f541ae4 builds this invoice in the plain state at all.

**Order.** P first, on step 1's load, so the first launch of the sitting is BUILD 6f541ae4 on a state rows 2162-RW and 2162-RX both read as plain. G then grows the state once, and G1 and G2 follow in it with no Shut Company, load, key or screen act between G's read and G2's read. C last, after a reload, closes the loop.

## Book

`BRIDGE PILOT LAB` (synthetic), its GUID pinned in `count19.py` and `calls.py`, loaded alone on TallyPrime 7.2 Silver. `LAB-COMMON.md` v3, byte-equal to row 2162-RX's, applies in full; the stricter text wins.

- **Pinned by GUID.** `count19.py` and `calls.py` hold the GUID. `02-list_companies.json` must name it. Any other GUID: STOP, whatever the displayed name.
- **Starting state.** EXPECT ALTVCHID 68, ALTMSTID 245, current date 15-Sep-2026, period 1-Apr-26 to 31-Mar-27 (lab 2162-RX). Any difference: STOP before the backup or a launch.
- **BK1 is the way back.** Make it at step 1 in the new empty `C:\bridge-lab\2162-rc\backup-1`, by C5. No restore is planned. No restore is made unless a mark moved and the run coordinator orders it.
- Record the release shown on Tally's screen.

## The two builds, named by commit

**BUILD 8c3fb9f2** (without the change). Already on the lab disk; rows 2162-RW and 2162-RX launched it.

COMMIT: `8c3fb9f2277966fa98b31bdc775b65c5ea3c29f7`  EXE: `C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe`  EXE SHA256: `1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`

**BUILD 6f541ae4** (with the change). The Windows bundle of the merge-queue run of Bridge CI that built commit 6f541ae4, downloaded at step 0. Nothing else is downloaded, installed or built.

COMMIT: `6f541ae437079c7a06c85f51afde26dcd9374187`  CI RUN: 45350931772  ARTIFACT: `unsigned-bundle-smoke-windows-latest`, id 14901146187, 60,692,023 bytes, kept by GitHub until 1 Dec 2026  ZIP DIGEST: `a83111827ad0319047d4a23b98f52986e705a39c102a7fb829450d99c5c74dc0`  MCPB archive_sha256: `5a6b02433a670cb128495f45a1bd8a098f2a7d45d948d8e6f443423e9eb7f91f`  EXE SHA256 (binary_sha256 in the bundle's mcpb-smoke.json): `9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc`  EXE: `C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe`

The artifact's id, size, digest, expiry and run were read from GitHub without a download when this sheet was written; the mcpb and exe hashes come from the run coordinator and are checked at step 0 for the first time. Any hash that differs, at step 0 or before any launch: STOP. Never copy, move, replace or build an exe; the one download is step 0's.

No screen-tool grant is needed: no call opens a box. If an approval box appears: STOP, press nothing in it, and report.

## Shell

Every command below is typed in **Git Bash**, from `/c/bridge-lab/2162-rc`, exactly as written (row 2162-RX's launches ran from Windows PowerShell 5.1; its notes record it). Git Bash is chosen because PowerShell 5.1's `>` re-encodes what it writes, which would corrupt the downloaded zip. The interpreter is `python` (3.10); `python3` on the lab PC is the Store alias and is never used (lab 2162-RX, deviation 1). `-X utf8` makes a helper write UTF-8 to a file, so its output is not left in the PC's ANSI code page (lab 2162-RX, deviation 7). Every helper's output is written to a file under `evidence/` with its exit code and then printed whole (lab 2162-RX, deviation 6: two checks went to the screen only). A line that begins `#` is an instruction, not typed.

**`local/` is the lab's own folder.** Everything the lab adds that is not a sheet file and not a named output goes in `local/`: its own helpers, notes drafts, `rates.sha256`, the BK1 listing. `check_sheet.py` skips `local/`, `evidence/`, `reads/`, `dl/`, `exe-6f541ae4/`, `backup-1/`, `scratch/`, the four run folders and the four data folders by name, and the top-level answers and logs that `run.py` and `ratereq.py` write (`a-<label>-<request>`, `log-<label>.txt`, `rates.xml`, `kept-local.txt`), so it can be run in the working folder at any time; every other file outside `SHA256.txt` is an error (lab 2162-RX, deviation 2: its check listed the run folder's own outputs and had to be run in a clean copy).

## Launches

Four launches, each by `client.py` (byte-equal to row 2162-RX's) with its own new run and data folders. Each launch file is row 2162-RX's first launch file with one value changed: the data folder is this row's. Each has exactly the five settings: writes enabled (the invoice builder refuses otherwise), terms accepted, host `127.0.0.1`, port `9001`, and its own new empty data folder. The four forbidden override keys are absent; `client.py` refuses them. No script writes a launch file.

| launch | arm | build | the line to type first |
|---|---|---|---|
| LP | P | BUILD 6f541ae4 | `RUN=LP; DIR=run-p; DATA=data-p; EXE=C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe; EXE_SHA=9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc` |
| LG1 | G1 | BUILD 8c3fb9f2 | `RUN=LG1; DIR=run-g1; DATA=data-g1; EXE=C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe; EXE_SHA=1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59` |
| LG2 | G2 | BUILD 6f541ae4 | `RUN=LG2; DIR=run-g2; DATA=data-g2; EXE=C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe; EXE_SHA=9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc` |
| LC | C | BUILD 6f541ae4 | `RUN=LC; DIR=run-c; DATA=data-c; EXE=C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe; EXE_SHA=9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc` |

**The launch procedure.** Type the launch's line from the table, then these lines in order. Each EXPECT is checked before the next line.

1. Folders. EXPECT `made`; `STOP` is a STOP (never reuse a launch folder).
   ```bash
   if [ -e "$DIR" ] || [ -e "$DATA" ]; then echo "STOP: $DIR or $DATA exists"; else mkdir "$DIR" "$DIR/inbox" "$DIR/answers" "$DATA" && echo "made $DIR $DIR/inbox $DIR/answers $DATA"; fi
   ```
2. The exe and the launch file. EXPECT two lines ending `: OK`. `FAILED` or no output: STOP.
   ```bash
   echo "$EXE_SHA  $EXE" | sha256sum -c - > "evidence/$DIR-hashes.txt" 2>&1; cp "launch/$RUN-launch-env.json" "$DIR/launch-env.json" && grep "  launch/$RUN-launch-env.json$" SHA256.txt | awk -v f="$DIR/launch-env.json" '{print $1"  "f}' | sha256sum -c - >> "evidence/$DIR-hashes.txt" 2>&1; cat "evidence/$DIR-hashes.txt"
   ```
3. Other Bridge processes. EXPECT `INFO: No tasks are running` and `exit 0`; a `bridge_mcp.exe` line: STOP and report (none may call port 9001 during this sitting).
   ```bash
   tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > "evidence/$DIR-procs-before.txt" 2>&1; echo "exit $?" >> "evidence/$DIR-procs-before.txt"; cat "evidence/$DIR-procs-before.txt"
   ```
4. Start the client in the background of this same shell, then read its log within 60 s. EXPECT the exe hash `ok`, the launch file's hash equal to `SHA256.txt`'s, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` with version 0.7.0. A `stop:` line, or no `initialized` line in 60 s: STOP.
   ```bash
   python -I client.py "C:/bridge-lab/2162-rc/$DIR" "$EXE" "$EXE_SHA" > "evidence/$DIR-client-out.txt" 2> "evidence/$DIR-client-err.txt" & CLIENT=$!; echo "client started, shell job pid $CLIENT" | tee "evidence/$DIR-launch.txt"
   cat "$DIR/client-log.txt"
   ```
5. The five calls, **one at a time**, in this order: `01-list.json`, `02-list_companies.json`, `03-marks.json`, `05-build-0074-0801.json`, `07-marks.json`. For each, type `F=` and its name (for example F=01-list.json), then the two lines below. The first copies it in under a name ending `.part`, checks its hash against `SHA256.txt`, and only then renames it (the client opens a name ending `.json` at once; lab 2162-SALES-Q, deviation 4). Wait until `client-log.txt` shows the call's line and `answers/` holds its answer before the second line. EXPECT `: OK` on the first line. The second's exit 2 is a failed call: STOP. No other call file is put in; none is written, edited or brought from another sheet, and in particular no post or verify call.
   ```bash
   cp "calls/$F" "$DIR/inbox/$F.part" && grep "  calls/$F$" SHA256.txt | awk -v f="$DIR/inbox/$F.part" '{print $1"  "f}' | sha256sum -c - && mv "$DIR/inbox/$F.part" "$DIR/inbox/$F" && echo "placed $F" | tee -a "evidence/$DIR-launch.txt"
   python -I -X utf8 calls.py show "$DIR/answers/$F" > "evidence/$DIR-show-$F.txt" 2>&1; echo "exit $?" >> "evidence/$DIR-show-$F.txt"; cat "evidence/$DIR-show-$F.txt"
   ```
   EXPECT `01` `tools: 37; missing of this row's: []` (lab 2162-RX for BUILD 8c3fb9f2; for BUILD 6f541ae4 the run coordinator's line says 37: record the count, and grade only `missing ... []`); `02` one company with the pinned GUID; `03` and `07` M 245, V 68, reused False. A moved mark is a STOP.
6. `05` is graded by `grade05.py`, not by `calls.py show` (row 2162-RX's `show` of a refused build printed no refusal at all: its refusals are under `result.refusals`, which `calls.py` does not read). Type the line below after `05`'s answer exists. It prints `result.state`, `result.reason`, each refusal's code and detail, `voucher_count`, the batch's sha256 and the evidence bytes and digests, then one `VERDICT` line. Exit 2 is a failed call: STOP. The arm's step says which `VERDICT` it EXPECTs. Do NOT EXPECT the earlier rows' evidence sizes or digests for BUILD 6f541ae4: record them.
   ```bash
   python -I -X utf8 grade05.py grade "$DIR/answers/05-build-0074-0801.json" > "evidence/$DIR-grade05.txt" 2>&1; echo "exit $?" >> "evidence/$DIR-grade05.txt"; cat "evidence/$DIR-grade05.txt"
   ```
7. The data folder. For an admitted build the first line finds the one batch file and checks it against the answer's sha256: EXPECT `equal to the answer's sha256`; `FINDING` (exit 1) is recorded and is a STOP. For a refused build type only the second line: EXPECT `exit 0` and no file ending `.xml` (no file written). The listing is kept whole.
   ```bash
   python -I -X utf8 calls.py batch "$DIR/answers/05-build-0074-0801.json" "$DATA" > "evidence/$DIR-batch.txt" 2>&1; echo "exit $?" >> "evidence/$DIR-batch.txt"; cat "evidence/$DIR-batch.txt"
   find "$DATA" -type f > "evidence/$DIR-data-files.txt" 2>&1; echo "exit $?" >> "evidence/$DIR-data-files.txt"; cat "evidence/$DIR-data-files.txt"
   ```
8. Close. EXPECT the log's last line `session ended; server pid <pid> exit code 0`, `client exit 0`, and then no `bridge_mcp.exe` running. Never end a process. A server that does not end: STOP and report.
   ```bash
   touch "$DIR/inbox/stop"; wait $CLIENT; echo "client exit $?" | tee -a "evidence/$DIR-launch.txt"; tail -n 1 "$DIR/client-log.txt"; stat -c '%n %s' "$DIR"/server-stderr-*.log
   tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > "evidence/$DIR-procs-after.txt" 2>&1; echo "exit $?" >> "evidence/$DIR-procs-after.txt"; cat "evidence/$DIR-procs-after.txt"
   ```

No raw request while an exe runs, and no exe while a raw request or screen act is in progress. The build call writes its batch only inside that launch's data folder; no call sends it to Tally.

## Raw encoding and bounded output

`run.py` is copied unchanged. It sends `Content-Type: text/xml; charset=utf-16` with `curl.exe --data-binary` (`run.py:22-23`). `rates.xml` and `guids.xml` are BOM-prefixed UTF-16LE with no added newline. This matches the build's entity encoder and its prepare path, as row 2162-RX's sheet sets out. `run.py` records curl's exit code and the answer's size; it does not record the HTTP status line or the answer's Content-Length header that C3 names (`--fail-with-body` turns a status of 400 or more into a curl failure). For this row that part of C3 is not asked for, as in rows 2162-RW and 2162-RX: no second request is sent to obtain a status line or a header; a status of 400 or more, an empty answer or a curl failure still stops `run.py`.

Every raw answer is written whole to `a-<label>-<request>`; a curl failure, empty answer, `Unknown Request`, or body over 2,000,000 bytes stops `run.py`. Each static request's hash is checked against `SHA256.txt`, and `rates.xml`'s against `local/rates.sha256`, on the same line as the send, which runs only if the check prints `OK`. Never pipe an answer through a truncating reader.

`ratecmp.py` prints at most 50 ledgers, 40 GST rows, 40 state blocks, 40 slab lists and 40 rate details per containing item, 100 child elements per child list, 20 refusal reasons per ledger, and 200 structural-diff lines; every truncation prints the exact count omitted. `grade05.py` prints at most 40 refusals and 40 differing lines, with the count of the rest. `calls.py` and `client.py` keep their previous bounds.

## What stays on the lab PC

As in row 2162-RX, keep these local and list each in `kept-local.txt` by name, bytes and SHA-256:

- BK1 (`TDBK1800_<company number>.001`) and every backup directory listing; quote no company number in the notes.
- Every saved Bridge answer and server error log until scanned: answers can carry `company_number`. Evidence copies replace that value with `redacted`; originals stay local. Each launch's `01` answer holds the product's tool descriptions and stays local; its grading line is quoted.
- The data folders and their batch files (a batch file names the book's company); `grade05.py`'s and `calls.py batch`'s outputs go to evidence, the files do not.
- The downloaded zip, the unpacked bundle and its exe, listed by sha256.
- Every screenshot. Record its name, bytes and SHA-256 only.
- Any raw answer that the pre-evidence scan finds holding a company number or data outside the named synthetic lab family.

On the evidence branch put the raw answers admitted by that scan, every helper and self-check output, launch logs, call files, redacted answers, and the kept-local list.

## Files

Byte-equal copies of row 2162-RX's sheet (`lab/2162-rx-sheet` at 6750ea49, folder `lab/2162-rx/`): `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml`, `run.py`, `client.py`, `calls.py`, `count19.py`, `ratereq.py`, `ratecmp.py`, `selftest-rate.py`, `guids.xml`, `pilot-lab-ledger-rates.utf16le.xml`, and the five call files under `calls/`.

Byte-equal copies of row 2162-RX's answers (`lab/2162-rx-answers` at 3c0f4bd3), used only by `grade05.py --selftest`: `selftest/05-refused.json` (its first launch's answer to `05`, the refusal BUILD 8c3fb9f2 gave in the grown state) and `selftest/05-admitted.json` (its second launch's answer to `05`, the admission after the reload). Both carry `company_number` already redacted.

Changed for this row:

- `launch/LP-launch-env.json`, `launch/LG1-launch-env.json`, `launch/LG2-launch-env.json`, `launch/LC-launch-env.json`: row 2162-RX's first launch file with its data folder replaced by this row's.
- `check_sheet.py`: row 2162-RX's checks, with this row's arms and the arms and launches of row 2162-RX it does not run; the skips of `local/` and the sitting's outputs named above; each launch file's five settings and data folder; each launch's exe path paired with its build's hash in the table above; and no build called by the two words other sheets use.
- `RUN-SHEET.md` and `SHA256.txt`.

New: `grade05.py`, the one helper this row adds (read-only). `grade05.py grade` reads a `05` answer. `grade05.py importeq` compares two admitted builds' batch files with exactly one value set aside: the voucher identity the build derives from the batch id and the voucher's `bridge_txn_id`. The batch id itself is not in the file; that identity is, once as the voucher's `REMOTEID` and once in the narration's `[BRIDGE:...]` tag (code `src-tauri/src/agent_import.rs:5786-5799`, `src-tauri/src/agent_import_identity.rs:16-23`). Nothing else is ignored: no whitespace, no encoding, no other field. No existing helper does either: `calls.py reqdiff` lists differing lines but gives no yes or no, and the identity differs between any two builds.

At the sitting, `ratereq.py` generates `rates.xml`; `run.py` generates `a-*` answers and `log-*.txt`; launches generate their run and data folders; C5 generates the backup; step 0 generates `dl/` and `exe-6f541ae4/`. These are outputs, not missing sheet inputs.

## Self-checks before the sitting

Run from `/c/bridge-lab/2162-rc`. No Tally request, `client.py` launch, or restore is part of them.

```bash
python -I ratereq.py --selftest > evidence/selfcheck-ratereq.txt 2>&1; echo "exit $?" >> evidence/selfcheck-ratereq.txt; cat evidence/selfcheck-ratereq.txt
python -I -X utf8 selftest-rate.py > evidence/selfcheck-rate.txt 2>&1; echo "exit $?" >> evidence/selfcheck-rate.txt; cat evidence/selfcheck-rate.txt
python -I -X utf8 grade05.py --selftest > evidence/selfcheck-grade05.txt 2>&1; echo "exit $?" >> evidence/selfcheck-grade05.txt; cat evidence/selfcheck-grade05.txt
python -I -c "import ast,sys;[ast.parse(open(f).read()) for f in sys.argv[1:]]" *.py > evidence/selfcheck-parse.txt 2>&1; echo "exit $?" >> evidence/selfcheck-parse.txt; cat evidence/selfcheck-parse.txt
python -I -X utf8 check_sheet.py > evidence/selfcheck-sheet.txt 2>&1; echo "exit $?" >> evidence/selfcheck-sheet.txt; cat evidence/selfcheck-sheet.txt
```

EXPECT, in order: `96b41d40c58d9527da459219d4ea1b020b3c7e6688053ce3de7a42af892cb812` then `equal to the pinned hash`, exit 0; `23 cases, 23 right`, exit 0; `3 cases, 3 right`, exit 0; no output but `exit 0`; three `OK:` lines, exit 0. Anything else: STOP.

## Steps

Raw reads use `run.py`, one call per file, in the order given and under the arm's label, so that the screenshot named falls between them, then the named parser, as rows 2162-RW and 2162-RX did. A STOP from either stops the row. Record each whole output. After every raw send, take the whole-window screenshot required by C3 before the next send or key. The command lines in the blocks are the ones typed; where a copied paragraph of row 2162-RX names a helper without `python -I -X utf8`, the block's line is the one used.

**Prompt rule (C7/C8).** No prompt is expected in these arms except C5's backup and C6's restore prompts. Any other dialog, message or half-entered screen: screenshot, copy its text, press no key, send nothing, STOP and report. Never Escape at Select Company or the Gateway. Never end a process. Nothing is restored after a STOP without the run coordinator's word.

**Path rule.** The screen after each key must be the one named. If not, screenshot and STOP with no further key. After a send or screen act, five minutes with no answer or change is a STOP; never resend.

**Keys.** The keys for Select Company, GSTR-1, the HSN summary, the two voucher lists and Shut Company are the keys recorded in row 2162-RW's notes, written out below; use them and no others. **Shut Company**, at the Gateway: click a blank part of the Gateway's left panel; Alt+K; EXPECT the K: Company menu; mouse click on SHut; EXPECT the Shut Company box listing BRIDGE PILOT LAB alone, highlighted; Enter; EXPECT Select Company, no company loaded, no prompt. A list with more than one line, or any prompt: no key, STOP. **Open the HSN summary**: at the Gateway (screenshot) click `G: Go To`; type `GSTR-1`; EXPECT List of Reports with no highlighted line; Down once (`GSTR-1` highlighted); Enter. Any list, prompt or other screen: path rule; do not select anything. EXPECT `GSTR-1` with the highlight on Total Vouchers. Write down the period shown (row 2162-RW: 1-Aug-26 to 31-Aug-26). Press no period key. A different period is written down and the arm goes on; a later step whose named line then does not show falls under the path rule. Look at no sub-report. With the highlight on Total Vouchers (where GSTR-1 opens; if it is elsewhere, path rule), press Down 24 times, one at a time, with a zoom after each (the 24 lines passed, in the order row 2162-RW's notes record: 1 Included in Return, 2 Ready for Upload, 3 Modified in Books After Upload/Export, 4 Not Relevant for This Return, 5 Uncertain Transactions (Corrections needed), 6 Check Vouchers Having Potential Conflicts with Masters, 7 B2B Invoices - 4A, 4B, 4C, 6B, 6C, 8 B2C (Large) Invoices - 5A, 5B, 9 Exports Invoices - 6A, 10 Credit or Debit Notes (Registered) - 9B, 11 Credit or Debit Notes (Unregistered) - 9B, 12 Amended B2B Invoices - 9A, 13 Amended B2C (Large) Invoices - 9A, 14 Amended Exports Invoices - 9A, 15 Amended Credit or Debit Notes (Registered) - 9C, 16 Amended Credit or Debit Notes (Unregistered) - 9C, 17 B2C (Small) Invoices - 7, 18 Nil Rated Invoices - 8A, 8B, 8C, 8D, 19 Amendment B2C (Small) Invoices - 10, 20 Tax Liability (Advances Received) - 11A(1), 11A(2), 21 Adjustment of Advances - 11B(1), 11B(2), 22 Amended Tax Liability (Advances Received) - 11A, 23 Amendment of Adjusted Advances - 11B, where the list scrolls, 24 the HSN Summary line): EXPECT `HSN Summary - 12 (B2B - B2C Supplies)` alone highlighted; Enter; EXPECT the summary with the highlight on 847130. A line not in that order, or no highlight: path rule.

**Ledger Vouchers is not opened in this row**, so row 2162-RX's deviations 3 and 4 (a standing Create row in Select Ledger, and the way back from Ledger Vouchers) have no step here. The one general rule they left is kept: a standing, unhighlighted Create row in a selection list is not a stop by itself; a highlighted line other than the named one is. Take a capture before every Escape, and never press Escape while the screen is the Gateway.

**Arm frame.** Row 2162-RX's frame, copied whole. G and C run a, b and d; G runs c as the W1 walk. P runs none of it: step 1's load and step 2's two reads stand for a and b.

- a. Shut Company by the keys above. Any `Quit ?` or other prompt: prompt rule. Load the book alone by step 1's Select Company keys. EXPECT the Gateway and starting date/period. Otherwise STOP. No other screen is opened.
- b. Send `<arm>-pre`: `M-marks.xml rates.xml`, then `<arm>-pre2`: `rates.xml`, with no key between them. `count19.py --same-row a-start-M-marks.xml a-<arm>-pre-M-marks.xml` must pass. A moved mark: STOP and report. Run `ratecmp.py a-<arm>-pre-rates.xml a-<arm>-pre2-rates.xml` and record it. Lab 2160-L1 saw such a difference for voucher types; row 2162-RW saw none here. A difference between the two is a first-read-after-load effect: write it down; it is not a STOP. A `READ refused` line or an exit 2 on either is a STOP as everywhere. Then `ratecmp.py a-A0-first-rates.xml a-<arm>-pre2-rates.xml`. The state has cleared when all six ledgers are `SHAPE measured` and there is no structural difference. If the state does not clear after Shut Company and a reload, that is a STOP with no further act.
- c. The step's screen act, and only that.
- d. Send `<arm>`: `M-marks.xml rates.xml`. `count19.py --same-row a-start-M-marks.xml a-<arm>-M-marks.xml` must pass; a moved mark: STOP and report. Run `ratecmp.py a-A0-first-rates.xml a-<arm>-rates.xml` and record its whole output, each of the six verdicts, the three diff counts, and which ledgers the differing paths fall under.

0. **Before the sitting.**
   - Make the row folder and copy this sheet's folder into it byte for byte from the lab's Bridge clone. Set `CLONE` to that clone's Git Bash path (the one row 2162-RX's sheet was fetched into) and `SHEET` to the 40-character commit the run coordinator names for this sheet; these are the only two values this sheet cannot write out. EXPECT `made`, then `git` and `tar` with no error.
     ```bash
     if [ -e /c/bridge-lab/2162-rc ]; then echo "STOP: /c/bridge-lab/2162-rc exists"; else mkdir /c/bridge-lab/2162-rc && echo "made /c/bridge-lab/2162-rc"; fi
     git -C "$CLONE" fetch origin && git -C "$CLONE" archive "$SHEET" lab/2162-rc | tar -x -C /c/bridge-lab/2162-rc --strip-components=2; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"
     cd /c/bridge-lab/2162-rc
     if [ -e evidence ] || [ -e local ]; then echo "STOP: evidence or local exists"; else mkdir evidence local && echo "made evidence local"; fi
     ```
   - Run all five Self-checks. Any miss: STOP.
   - BUILD 8c3fb9f2. EXPECT `: OK`.
     ```bash
     echo "1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59  C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > evidence/s0-build-8c3fb9f2.txt 2>&1; echo "exit $?" >> evidence/s0-build-8c3fb9f2.txt; cat evidence/s0-build-8c3fb9f2.txt
     ```
   - BUILD 6f541ae4: the one download. Type each line only after the one before met its EXPECT. EXPECT, in order: `made`; `gh exit 0`; `60692023`; `: OK` for the zip; the listing holds `bridge-tally.mcpb` and mcpb-smoke.json at the top of the zip; the two extracted paths printed; `: OK` for the mcpb; `archive_sha256 5a6b02433a670cb128495f45a1bd8a098f2a7d45d948d8e6f443423e9eb7f91f`, `binary_sha256 9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc` and `server_version 0.7.0` (record `platform`, `default_tool_count` and `opt_in_tool_count`; the run coordinator's line says 37 tools with the opt-in set); the mcpb unpacked with no error; `: OK` for the exe. Any `FAILED`, any non-zero exit, or a value that differs: STOP, and nothing is launched from this folder. Row 2162-RX's sister row 2569-R downloaded its build with `gh api` the same way (lab 2569-R, notes).
     ```bash
     if [ -e dl ] || [ -e exe-6f541ae4 ]; then echo "STOP: dl or exe-6f541ae4 exists"; else mkdir dl exe-6f541ae4 && echo "made dl exe-6f541ae4"; fi
     gh api repos/ComplyEaze/bridge/actions/artifacts/14901146187/zip > dl/unsigned-bundle-smoke-windows-latest.zip; echo "gh exit $?" | tee evidence/s0-download.txt
     stat -c '%s' dl/unsigned-bundle-smoke-windows-latest.zip | tee -a evidence/s0-download.txt
     echo "a83111827ad0319047d4a23b98f52986e705a39c102a7fb829450d99c5c74dc0  dl/unsigned-bundle-smoke-windows-latest.zip" | sha256sum -c - 2>&1 | tee -a evidence/s0-download.txt
     python -I -m zipfile -l dl/unsigned-bundle-smoke-windows-latest.zip > evidence/s0-zip-list.txt 2>&1; echo "exit $?" >> evidence/s0-zip-list.txt; cat evidence/s0-zip-list.txt
     python -I -c "import sys,zipfile; z=zipfile.ZipFile(sys.argv[1]); [print(z.extract(n, sys.argv[2])) for n in ('bridge-tally.mcpb', 'mcpb-smoke.json')]" dl/unsigned-bundle-smoke-windows-latest.zip dl/unpacked 2>&1 | tee -a evidence/s0-download.txt
     echo "5a6b02433a670cb128495f45a1bd8a098f2a7d45d948d8e6f443423e9eb7f91f  dl/unpacked/bridge-tally.mcpb" | sha256sum -c - 2>&1 | tee -a evidence/s0-download.txt
     python -I -c "import json,sys; d=json.load(open(sys.argv[1], encoding='utf-8')); [print(k, d.get(k, '<absent>')) for k in ('archive_sha256', 'binary_sha256', 'platform', 'server_version', 'default_tool_count', 'opt_in_tool_count')]" dl/unpacked/mcpb-smoke.json 2>&1 | tee -a evidence/s0-download.txt
     python -I -m zipfile -e dl/unpacked/bridge-tally.mcpb exe-6f541ae4; echo "unpack exit $?" | tee -a evidence/s0-download.txt
     echo "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc  C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - 2>&1 | tee -a evidence/s0-download.txt
     ```
   - C2 step 0 with no company loaded. EXPECT one `tally.exe` line and its PID; a `LISTENING` line on `:9001` with that PID; `curl exit 0` and `200`; no dialog in the first screenshot. Otherwise STOP. `netstat`'s file is kept whole; the `grep` only shows the lines to read.
     ```bash
     tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > evidence/s0-tasklist.txt 2>&1; echo "exit $?" >> evidence/s0-tasklist.txt; cat evidence/s0-tasklist.txt
     netstat -ano > evidence/s0-netstat.txt 2>&1; echo "exit $?" >> evidence/s0-netstat.txt; grep -E ':9001 |^exit' evidence/s0-netstat.txt
     curl.exe -sS -w "%{http_code} %{size_download}\n" http://127.0.0.1:9001/status -o evidence/s0-status-body.txt > evidence/s0-status.txt 2>&1; echo "curl exit $?" >> evidence/s0-status.txt; cat evidence/s0-status.txt
     ```
1. **Load, check, back up, and render the rate request.**
   - Load BRIDGE PILOT LAB alone from Select Company (C1): click the filter; type `BRIDGE`, space, `PILOT` until it is the one highlighted line, Enter. No Escape at Select Company. EXPECT the Gateway, current date and period in Book. Otherwise STOP.
   - Send `start`: `M-marks.xml`; parse it. EXPECT one row, the pinned GUID, V 68, M 245. Otherwise STOP before BK1.
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 start M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py a-start-M-marks.xml > evidence/start-count19.txt 2>&1; echo "exit $?" >> evidence/start-count19.txt; cat evidence/start-count19.txt
     ```
   - Make `backup-1` new and empty; take BK1 by C5, with the keys row 2162-RW's notes record. Click the Gateway's left panel; Alt+Y: EXPECT the Data menu; Enter (Backup & Restore); Enter (Backup): EXPECT the Backup screen. In Backup Data Path press Backspace: EXPECT the List of Folders; Up three times, one at a time with a zoom after each (row 2162-RW passed TallyDrive, Select from Drive, then `Specify Path` highlighted); Enter; type the full path of `backup-1` (zoom: exact); Enter: EXPECT the list with that folder highlighted; Enter: EXPECT the path to read that folder. In the company filter type `BRIDGE`, space, `PILOT`: EXPECT one line, BRIDGE PILOT LAB, highlighted; Enter; click `End of List` (zoom: highlighted); Enter. EXPECT `Accept ?` showing that path and the one company: screenshot, click inside the box, `y`. EXPECT the Gateway and no message. A line, list or prompt other than these: path rule. Record its masked file name, bytes, SHA-256 and Restore-list time. EXPECT exactly one non-empty backup file. Otherwise STOP.
     ```bash
     if [ -e backup-1 ]; then echo "STOP: backup-1 exists"; else mkdir backup-1 && echo "made backup-1"; fi
     # after the backup: the listing and the hash stay local (the file name holds the company number)
     ls -la backup-1 > local/bk1.txt 2>&1; sha256sum backup-1/* >> local/bk1.txt 2>&1; echo "exit $?" >> local/bk1.txt; cat local/bk1.txt
     ```
   - Send `setup`: `M-marks.xml guids.xml`, in that order. EXPECT marks still 68/245. A moved mark: STOP and report. Then parse the GUIDs. EXPECT `LEDGER ROWS 6` and exactly the six names of call `05`, each with a `BRIDGECOMPANYGUID` equal to the book's GUID and its own GUID the book's GUID followed by the suffix rows 2162-RW and 2162-RX read: `BRIDGE Walk-in` `-000000d0`, `BRIDGE Svc 998313 5%` `-000000d1`, `BRIDGE Svc 998314 5%` `-000000d2` (both under `Sales Accounts`), `BRIDGE CGST 2.5%` `-000000d4`, `BRIDGE SGST 2.5%` `-000000d6`, `BRIDGE Round Off` `-000000d8` (lab 2162-RX, notes). Any other count, name or GUID, or a `READ refused` line: STOP (this row's byte EXPECTs rest on the same request).
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 setup M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     grep '  guids.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 setup guids.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-setup-M-marks.xml > evidence/setup-count19.txt 2>&1; echo "exit $?" >> evidence/setup-count19.txt; cat evidence/setup-count19.txt
     python -I -X utf8 ratecmp.py a-setup-guids.xml > evidence/setup-guids.txt 2>&1; echo "exit $?" >> evidence/setup-guids.txt; cat evidence/setup-guids.txt
     ```
   - Render the one rate request used all sitting (the build's window is financial-year start through the invoice date; code `src-tauri/src/agent_import_invoice_wire.rs:1206-1212`). The book's GUID is read from `calls.py`, so no GUID is typed. EXPECT `rates.xml: 2874 bytes, sha256 182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08` (lab 2162-RX), then `rates.xml: OK`. Anything else: STOP.
     ```bash
     G=$(sed -n 's/^BOOK = "\([0-9a-f-]*\)".*/\1/p' calls.py); echo "book GUID read: ${#G} characters"
     python -I ratereq.py --company "BRIDGE PILOT LAB" --from-date 20260401 --to-date 20260801 --output rates.xml "$G-000000d0" "$G-000000d1" "$G-000000d2" "$G-000000d4" "$G-000000d6" "$G-000000d8" > evidence/ratereq-out.txt 2>&1; echo "exit $?" >> evidence/ratereq-out.txt; cat evidence/ratereq-out.txt
     echo "182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08  rates.xml" > local/rates.sha256; sha256sum -c local/rates.sha256
     ```
2. **P: the plain state, BUILD 6f541ae4.**
   - With no key pressed and no report opened between them (the C3 screenshot after each send is still taken), send `A0-first`: `M-marks.xml rates.xml`, then `A0-second`: `rates.xml`. `count19.py --same-row a-start-M-marks.xml a-A0-first-M-marks.xml` must pass. A moved mark: STOP and report.
   - `python -I ratecmp.py a-A0-first-rates.xml a-A0-second-rates.xml`. EXPECT `LEDGER ROWS 6`, the six names, every one `SHAPE measured`, no structural difference, and **equal byte sizes and SHA-256** (lab 2162-RW). A missing/other row, unequal hash, an unmeasured shape or a `READ refused` line: STOP, with the one exception below.
   - The exception: if both answers show `LEDGER ROWS 6`, the six names, every one `SHAPE measured` and no `READ refused` line, and the only failure is that the two differ, send `A0-third`: `rates.xml`, once. If `a-A0-second-rates.xml` and `a-A0-third-rates.xml` are equal in bytes and SHA-256, record a first-read effect with the whole `ratecmp.py` output of first against second, and use `a-A0-second-rates.xml` wherever this sheet names `a-A0-first-rates.xml`. If second and third differ too: STOP.
   - The commands, and the plain check: EXPECT both answers `: OK` against the plain sha256 and 29114 bytes (lab 2162-RX). Not plain: STOP (the row's premise for P failed).
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 A0-first M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 A0-first rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 A0-second rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-A0-first-M-marks.xml > evidence/A0-first-count19.txt 2>&1; echo "exit $?" >> evidence/A0-first-count19.txt; cat evidence/A0-first-count19.txt
     python -I -X utf8 ratecmp.py a-A0-first-rates.xml a-A0-second-rates.xml > evidence/A0-first-vs-A0-second.txt 2>&1; echo "exit $?" >> evidence/A0-first-vs-A0-second.txt; cat evidence/A0-first-vs-A0-second.txt
     stat -c '%n %s' a-A0-first-rates.xml > evidence/A0-first-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-A0-first-rates.xml" | sha256sum -c - >> evidence/A0-first-is-plain.txt 2>&1; echo "exit $?" >> evidence/A0-first-is-plain.txt; cat evidence/A0-first-is-plain.txt
     stat -c '%n %s' a-A0-second-rates.xml > evidence/A0-second-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-A0-second-rates.xml" | sha256sum -c - >> evidence/A0-second-is-plain.txt 2>&1; echo "exit $?" >> evidence/A0-second-is-plain.txt; cat evidence/A0-second-is-plain.txt
     ```
   - Launch LP by the launch procedure. EXPECT `VERDICT admitted voucher_count 1, no refusal` and the batch file equal to the answer's sha256. Record `batch`'s whole output: the batch file's sha256 is one of this row's results. Any other verdict: record it whole, close, finish this arm's read, then STOP: the change broke the plain case.
   - Send `P-post`: `M-marks.xml rates.xml`. EXPECT marks equal to `start`, the answer plain, and no structural difference from the first answer. Otherwise STOP.
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 P-post M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 P-post rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-P-post-M-marks.xml > evidence/P-post-count19.txt 2>&1; echo "exit $?" >> evidence/P-post-count19.txt; cat evidence/P-post-count19.txt
     stat -c '%n %s' a-P-post-rates.xml > evidence/P-post-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-P-post-rates.xml" | sha256sum -c - >> evidence/P-post-is-plain.txt 2>&1; echo "exit $?" >> evidence/P-post-is-plain.txt; cat evidence/P-post-is-plain.txt
     python -I -X utf8 ratecmp.py a-A0-first-rates.xml a-P-post-rates.xml > evidence/A0-first-vs-P-post.txt 2>&1; echo "exit $?" >> evidence/A0-first-vs-P-post.txt; cat evidence/A0-first-vs-P-post.txt
     ```
3. **G: Shut Company, a load, and the W1 walk.**
   - Frame a and b. Frame c: open the HSN summary. Down twice from 847130, one at a time with a zoom after each (9983, then `998313` alone highlighted); Enter: EXPECT `GSTR-1 - Voucher Register`, `Vouchers of 998313` (lab 2162-SALES-Q step 12.5; lab 2162-RW step 5). Esc once: EXPECT the summary, highlight 998313. Down once (zoom: `998314` alone highlighted); Enter: EXPECT `Vouchers of 998314`. Esc once: EXPECT the summary, highlight 998314. Esc once: EXPECT `GSTR-1`. Esc once: EXPECT the Gateway. No further Escape.
   - Frame b's commands, between frame a and frame c. EXPECT both `pre` answers plain.
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G-pre M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G-pre rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G-pre2 rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G-pre-M-marks.xml > evidence/G-pre-count19.txt 2>&1; echo "exit $?" >> evidence/G-pre-count19.txt; cat evidence/G-pre-count19.txt
     python -I -X utf8 ratecmp.py a-G-pre-rates.xml a-G-pre2-rates.xml > evidence/G-pre-vs-G-pre2.txt 2>&1; echo "exit $?" >> evidence/G-pre-vs-G-pre2.txt; cat evidence/G-pre-vs-G-pre2.txt
     python -I -X utf8 ratecmp.py a-A0-first-rates.xml a-G-pre2-rates.xml > evidence/A0-first-vs-G-pre2.txt 2>&1; echo "exit $?" >> evidence/A0-first-vs-G-pre2.txt; cat evidence/A0-first-vs-G-pre2.txt
     stat -c '%n %s' a-G-pre2-rates.xml > evidence/G-pre2-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-G-pre2-rates.xml" | sha256sum -c - >> evidence/G-pre2-is-plain.txt 2>&1; echo "exit $?" >> evidence/G-pre2-is-plain.txt; cat evidence/G-pre2-is-plain.txt
     ```
   - Frame d (`G`), after the walk. EXPECT the answer grown (`: OK` against the grown sha256, 30626 bytes), and `ratecmp.py` against the first answer to show added 28, removed 0, text changed 0, every differing path under `BRIDGE Svc 998313 5%` or `BRIDGE Svc 998314 5%`, those two `SHAPE unmeasured` (BUILD 8c3fb9f2's rule) and the other four measured (lab 2162-RX, W1). **Not grown: record it whole, launch nothing, STOP** (the row's premise failed). A `READ refused` line or an exit 2: STOP.
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G-M-marks.xml > evidence/G-count19.txt 2>&1; echo "exit $?" >> evidence/G-count19.txt; cat evidence/G-count19.txt
     stat -c '%n %s' a-G-rates.xml > evidence/G-is-grown.txt 2>&1; echo "d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257  a-G-rates.xml" | sha256sum -c - >> evidence/G-is-grown.txt 2>&1; echo "exit $?" >> evidence/G-is-grown.txt; cat evidence/G-is-grown.txt
     python -I -X utf8 ratecmp.py a-A0-first-rates.xml a-G-rates.xml > evidence/A0-first-vs-G.txt 2>&1; echo "exit $?" >> evidence/A0-first-vs-G.txt; cat evidence/A0-first-vs-G.txt
     ```
   - **From here to the end of G2's read: no Shut Company, no load, no key and no screen act in Tally.** Screenshots only.
4. **G1: BUILD 8c3fb9f2 in the grown state.**
   - Launch LG1 by the launch procedure. EXPECT `VERDICT refused invoice_not_admitted: invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998313 5%), invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998314 5%)`, and at step 7 of the procedure no `.xml` file in `data-g1` (lab 2162-RX: refused, no file; its evidence was 62,071 bytes, request `6d6404a2`, response `5db1811d`; recorded, not graded).
   - Send `G1-post`: `M-marks.xml rates.xml`. EXPECT marks equal to `start`, the answer grown, and no structural difference from `G`.
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G1-post M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G1-post rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G1-post-M-marks.xml > evidence/G1-post-count19.txt 2>&1; echo "exit $?" >> evidence/G1-post-count19.txt; cat evidence/G1-post-count19.txt
     stat -c '%n %s' a-G1-post-rates.xml > evidence/G1-post-is-grown.txt 2>&1; echo "d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257  a-G1-post-rates.xml" | sha256sum -c - >> evidence/G1-post-is-grown.txt 2>&1; echo "exit $?" >> evidence/G1-post-is-grown.txt; cat evidence/G1-post-is-grown.txt
     python -I -X utf8 ratecmp.py a-G-rates.xml a-G1-post-rates.xml > evidence/G-vs-G1-post.txt 2>&1; echo "exit $?" >> evidence/G-vs-G1-post.txt; cat evidence/G-vs-G1-post.txt
     ```
   - G1 admitted with `voucher_count 1`: record both answers whole, write `G2 not run: G1 admitted`, and go to step 6. The state did not refuse the build without the change this time, so G2 would decide nothing.
   - Any other verdict (another code, one refusal, or a refusal naming another ledger): record it whole, then STOP and report. G2 then runs only on the run coordinator's word.
   - `G1-post` not grown, or a structural difference from `G`: STOP before G2 (a state the launch changed would confound G2).
5. **G2: BUILD 6f541ae4 in the same grown state.**
   - Launch LG2 by the launch procedure. EXPECT `VERDICT admitted voucher_count 1, no refusal` and the batch file equal to the answer's sha256. Record `batch`'s whole output.
   - Send `G2-post`: `M-marks.xml rates.xml`. EXPECT marks equal to `start`, the answer grown, and no structural difference from `G`. Not grown: record it; G2's verdict stands as recorded, with that note (the state at its launch is bracketed by `G1-post` and `G2-post`).
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G2-post M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G2-post rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G2-post-M-marks.xml > evidence/G2-post-count19.txt 2>&1; echo "exit $?" >> evidence/G2-post-count19.txt; cat evidence/G2-post-count19.txt
     stat -c '%n %s' a-G2-post-rates.xml > evidence/G2-post-is-grown.txt 2>&1; echo "d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257  a-G2-post-rates.xml" | sha256sum -c - >> evidence/G2-post-is-grown.txt 2>&1; echo "exit $?" >> evidence/G2-post-is-grown.txt; cat evidence/G2-post-is-grown.txt
     python -I -X utf8 ratecmp.py a-G-rates.xml a-G2-post-rates.xml > evidence/G-vs-G2-post.txt 2>&1; echo "exit $?" >> evidence/G-vs-G2-post.txt; cat evidence/G-vs-G2-post.txt
     ```
   - If G2 admitted: compare its batch file with P's. EXPECT, as the measurement (not shown), `EQUAL once the identity is set aside` (exit 0) or `DIFFER` (exit 1) with the differing lines. Either is a result. Exit 2 is a failed read: record it and go on.
     ```bash
     python -I -X utf8 grade05.py importeq run-p/answers/05-build-0074-0801.json data-p run-g2/answers/05-build-0074-0801.json data-g2 > evidence/importeq-P-G2.txt 2>&1; echo "exit $?" >> evidence/importeq-P-G2.txt; cat evidence/importeq-P-G2.txt
     ```
   - G2 refused: record `grade05.py`'s whole output, every refusal code and detail. Nothing more is launched in this state. Go to step 6: C runs after a reload, in another state.
6. **C: Shut Company, a load, BUILD 6f541ae4.** Frame a and b. Then launch LC. Then frame d (`C`).
   - Frame b's commands. EXPECT both `pre` answers plain.
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 C-pre M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 C-pre rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 C-pre2 rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-C-pre-M-marks.xml > evidence/C-pre-count19.txt 2>&1; echo "exit $?" >> evidence/C-pre-count19.txt; cat evidence/C-pre-count19.txt
     python -I -X utf8 ratecmp.py a-C-pre-rates.xml a-C-pre2-rates.xml > evidence/C-pre-vs-C-pre2.txt 2>&1; echo "exit $?" >> evidence/C-pre-vs-C-pre2.txt; cat evidence/C-pre-vs-C-pre2.txt
     python -I -X utf8 ratecmp.py a-A0-first-rates.xml a-C-pre2-rates.xml > evidence/A0-first-vs-C-pre2.txt 2>&1; echo "exit $?" >> evidence/A0-first-vs-C-pre2.txt; cat evidence/A0-first-vs-C-pre2.txt
     stat -c '%n %s' a-C-pre2-rates.xml > evidence/C-pre2-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-C-pre2-rates.xml" | sha256sum -c - >> evidence/C-pre2-is-plain.txt 2>&1; echo "exit $?" >> evidence/C-pre2-is-plain.txt; cat evidence/C-pre2-is-plain.txt
     ```
   - Launch LC by the launch procedure. EXPECT `VERDICT admitted voucher_count 1, no refusal` and the batch file equal to the answer's sha256; record `batch`'s whole output. Any other verdict: record it whole, close, finish the read below, then STOP: the change broke the plain case.
   - Frame d (`C`). EXPECT marks equal to `start`, the answer plain, and no structural difference from the first answer. Otherwise STOP.
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 C M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 C rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-C-M-marks.xml > evidence/C-count19.txt 2>&1; echo "exit $?" >> evidence/C-count19.txt; cat evidence/C-count19.txt
     stat -c '%n %s' a-C-rates.xml > evidence/C-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-C-rates.xml" | sha256sum -c - >> evidence/C-is-plain.txt 2>&1; echo "exit $?" >> evidence/C-is-plain.txt; cat evidence/C-is-plain.txt
     python -I -X utf8 ratecmp.py a-A0-first-rates.xml a-C-rates.xml > evidence/A0-first-vs-C.txt 2>&1; echo "exit $?" >> evidence/A0-first-vs-C.txt; cat evidence/A0-first-vs-C.txt
     ```
7. **Close.**
   - Write the arms not run and why. No restore was made.
   - With the company still loaded, send `close`: `M-marks.xml`. EXPECT pass, V 68, M 245. Nothing was keyed, so when this passes, **no restore is needed**. If a mark moved anywhere, STOP and report; any restore is only on the coordinator's word.
     ```bash
     grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 close M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-close-M-marks.xml > evidence/close-count19.txt 2>&1; echo "exit $?" >> evidence/close-count19.txt; cat evidence/close-count19.txt
     ```
   - Close by C10: Tally's own Shut Company by the keys above, then C2 a and b again (one `tally.exe` owning 9001), GET /status 200, a screenshot with no company loaded and the current date unchanged. Send no XML after the shut. Record the menu path and key used.
     ```bash
     tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > evidence/c10-tasklist.txt 2>&1; echo "exit $?" >> evidence/c10-tasklist.txt; cat evidence/c10-tasklist.txt
     netstat -ano > evidence/c10-netstat.txt 2>&1; echo "exit $?" >> evidence/c10-netstat.txt; grep -E ':9001 |^exit' evidence/c10-netstat.txt
     curl.exe -sS -w "%{http_code} %{size_download}\n" http://127.0.0.1:9001/status -o evidence/c10-status-body.txt > evidence/c10-status.txt 2>&1; echo "curl exit $?" >> evidence/c10-status.txt; cat evidence/c10-status.txt
     tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/c10-bridge.txt 2>&1; echo "exit $?" >> evidence/c10-bridge.txt; cat evidence/c10-bridge.txt
     ```

## Record

- Every raw answer whole, with bytes and SHA-256; every `run.py` log; every helper output whole, as the blocks write them. Record every structural diff and every truncated-output count.
- Every key and screen title for G and C, with UTC times and screenshots under the kept-local rule. Do not substitute earlier counts for what this sitting shows.
- For each launch: the folder check, the two hashes, the process lists before and after, the start command and shell, the whole `client-log.txt`, every answer with SHA-256 under the kept-local rule, every `calls.py show` output, `grade05.py`'s output, the batch or data-folder output, and the server error log's size.
- For BUILD 6f541ae4's admitted launches: each batch file's sha256 (from `calls.py batch`), each evidence size and request and response digest (recorded, not graded), and `importeq`'s whole output for P against G2.
- BK1 as C5 requires.
- A one-line arm table in the sitting notes: arm; state before (plain or grown, by sha256); build; `VERDICT`; state after; marks before and after; or the reason it was not run.

## Decides

All outcomes are **partial**: one book, one release, one sitting, one invoice.

- **G grown, G1 refused with the two codes, G2 admitted with `voucher_count 1`:** the change admits the measured shape on a live Tally: in one grown state, with no reload between, the build without it refused and the build with it admitted.
- **G2 refused:** the change does not do live what its tests show. The refusal codes are reported whole, and nothing more is launched in that state.
- **P or C refused:** the change broke the plain case. STOP.
- **G not grown:** the row's premise failed; nothing was launched in G, and nothing about the change is decided.
- **G1 admitted:** the state did not refuse the build without the change this time; G2 is not run and decides nothing.
- **`G1-post` not grown:** BUILD 8c3fb9f2's launch changed the state; G2 is not run.
- **`importeq` P against G2 EQUAL:** in this sitting the grown state did not change the file BUILD 6f541ae4 built, once the batch-derived identity is set aside. **DIFFER:** the lines it prints are what the state changed in the built file; reported as they stand.
- **Marks moved:** the sitting touched more than its read-only design permits. Stop and report; do not explain it as a display effect.

## Not covered

- The queue's re-read before a post (`admit_queued_invoice`, code `src-tauri/src/agent_import_invoice.rs:1326`): no post is made, so the check it runs again before a post is not run in this row.
- A post, its approval box, and the read-back of a posted invoice.
- One code's voucher list alone followed by a launch; a goods ledger; a tax ledger in the grown state; any other screen that grows the answer.
- How long the grown state lasts in a session that is not shut.
- Another book, release or sitting.

## Close and evidence

C10 at step 7 leaves one Tally with no company loaded. The synthetic book stays on the lab PC. C11 applies: evidence goes to a new answers branch only after the pre-push scan and coordinator approval. The sitting's RESULTS gives P, G, G1, G2 and C in order, with confidence, the state by sha256, the build's verdict, the batch sha256s and the `importeq` result, marks, and every skipped arm's reason. Quote only grading lines from kept-local files.
