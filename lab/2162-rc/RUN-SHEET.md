# Row 2162-RC: the build with the rate-row change, launched in the grown state

For #2162. Master's change for that issue, #2593 (merged as commit 6f541ae4), lets the invoice build admit a sales ledger whose rate rows carry Tally's zero per-unit fields (protocol reference §12a.31, in this tree; §12a.32 is on the reference branch and not in this tree, so its record is cited here: `lab/2162-rx-answers` at 3c0f4bd3, RESULTS.md). This row asks one question on a live Tally: in the SAME grown state of Tally's rate answer, with no reload between, does BUILD 8c3fb9f2 (without the change) refuse the test invoice and BUILD 6f541ae4 (with the change) admit it? One sitting, one synthetic book, one backup, no keyed data, no import and no post. The builds only build: no call posts, no approval box opens, and nothing is written to the book. The screen acts are the loads, Shut Company, the backup at step 1, and the GSTR-1 browse of the walk, with the marks read around each; no restore is planned. Written 24 Nov 2026. Until the row runs, every EXPECT is **unverified** unless its source says otherwise.

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

**The one variable.** P against G2: the same build, the state plain or grown. G1 against G2: the same state, the build without or with the change. The two builds differ by more than #2593: master moved 16 commits from 8c3fb9f2 to 6f541ae4. #2593 is the only one of them that changes `src-tauri/src/agent_import_invoice_wire.rs` or the non-test code of `src-tauri/src/agent_import_invoice.rs`, where the rate parse (`RowShape`, code `src-tauri/src/agent_import_invoice_wire.rs:1447-1466`) and the sales rate rule (`sales_rate_milli`, code `src-tauri/src/agent_import_invoice.rs:488-506`) live; others change `agent_import.rs`, `agent_import_post.rs` and `agent_import_verification.rs` (among them a build refusal for long e-invoice details, #2585, which this invoice does not carry). P and C show that BUILD 6f541ae4 builds this invoice in the plain state at all. BUILD 8c3fb9f2's exe path is on the screen tool's grant list and BUILD 6f541ae4's new path is not; row 2569-S (24 Nov 2026, answers `lab/2569-s-answers` at f0bfdb3e) saw a screenshot by the tool minimise the window of an ungranted `bridge_mcp.exe`. In this row neither build opens a window, and no screenshot is asked for while an exe runs, so that difference is recorded, not controlled.

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

**BUILD 6f541ae4** (with the change). The Windows bundle of the merge-queue run of Bridge CI that built commit 6f541ae4, downloaded at step 0. The one other fetch is step 0's `git fetch` of the sheet's own commit into the lab's clone; the one download is that artifact. Nothing else is downloaded, installed or built.

COMMIT: `6f541ae437079c7a06c85f51afde26dcd9374187`  CI RUN: 45350931772  ARTIFACT: `unsigned-bundle-smoke-windows-latest`, id 14901146187, 60,692,023 bytes, kept by GitHub until 1 Dec 2026  ZIP DIGEST: `a83111827ad0319047d4a23b98f52986e705a39c102a7fb829450d99c5c74dc0`  MCPB archive_sha256: `5a6b02433a670cb128495f45a1bd8a098f2a7d45d948d8e6f443423e9eb7f91f`  EXE SHA256 (binary_sha256 in the bundle's mcpb-smoke.json): `9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc`  EXE: `C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe`

The artifact's id, size, digest, expiry and run were read from GitHub without a download when this sheet was written; the run coordinator checked from its own download, with nothing run, that the mcpb's nine entries hold `bin/x86_64-pc-windows-msvc/bridge_mcp.exe` at the zip root (35,960,832 bytes) with the EXE SHA256 above, and that mcpb-smoke.json reads server_version 0.7.0, default_tool_count 35, opt_in_tool_count 37, archive_entries 9. Step 0 checks them again on the lab PC. Any hash that differs, at step 0 or before any launch: STOP. Never copy, move, replace or build an exe; the one download is step 0's.

The screen tool is used as in row 2162-RX, for screenshots, keys and clicks in Tally. No approval box opens, so no new grant is asked for. If an approval box appears: STOP, press nothing in it, and report.

## Shell

Every command below is typed in **Git Bash**, from `/c/bridge-lab/2162-rc`, exactly as written (row 2162-RX's launches ran from Windows PowerShell 5.1; its notes record it). Each launch's client start line is the one line not typed at the prompt: the operator's shell tool starts it whole as a background run (Launches). Git Bash is chosen because PowerShell 5.1's `>` re-encodes what it writes, which would corrupt the downloaded zip. The interpreter is `python` (3.10); `python3` on the lab PC is the Store alias and is never used (lab 2162-RX, deviation 1). `-X utf8` makes a helper write UTF-8 to a file, so its output is not left in the PC's ANSI code page (lab 2162-RX, deviation 7). Every helper's output is written to a file under `evidence/` with its exit code and then printed whole (lab 2162-RX, deviation 6: two checks went to the screen only). A line that begins `#` is an instruction, not typed. LAB-COMMON.md C2 a and b say "from cmd.exe"; this row types those listings (`tasklist`, `netstat`) in Git Bash with the same commands and arguments, a stated departure from that text, and step 0's `tally.exe` line is their proof: if it does not print the one `tally.exe` CSV line while Tally is running, or netstat's file holds no `:9001` line, STOP and report before step 1.

**`local/` is the lab's own folder.** Everything the lab adds that is not a sheet file and not a named output goes in `local/`: its own helpers, notes drafts, `rates.sha256`, the BK1 listing. `check_sheet.py` skips `local/`, `evidence/`, `reads/`, `dl/`, `exe-6f541ae4/`, `backup-1/`, `scratch/`, the four run folders and the four data folders by name, and the top-level answers and logs that `run.py` and `ratereq.py` write (`a-<label>-<request>`, `log-<label>.txt`, `rates.xml`, `kept-local.txt`), so it can be run in the working folder at any time; every other file outside `SHA256.txt` is an error (lab 2162-RX, deviation 2: its check listed the run folder's own outputs and had to be run in a clean copy).

## Launches

Four launches, each by `client.py` (byte-equal to row 2162-RX's) with its own new run and data folders. Each launch file is row 2162-RX's first launch file with one value changed: the data folder is this row's. Each has exactly the five settings: writes enabled (the invoice builder refuses otherwise), terms accepted, host `127.0.0.1`, port `9001`, and its own new empty data folder. The four forbidden override keys are absent; `client.py` refuses them. No script writes a launch file.

| launch | arm | build | run folder | data folder |
|---|---|---|---|---|
| LP | P | BUILD 6f541ae4 | `run-p` | `data-p` |
| LG1 | G1 | BUILD 8c3fb9f2 | `run-g1` | `data-g1` |
| LG2 | G2 | BUILD 6f541ae4 | `run-g2` | `data-g2` |
| LC | C | BUILD 6f541ae4 | `run-c` | `data-c` |

**The launch procedure** is written out whole, seven parts, at each launch's place in the steps below, with that launch's folders, exe, hash and call names in every line: no line depends on a shell variable or on a folder changed by an earlier line, and each begins `cd /c/bridge-lab/2162-rc && `. Each EXPECT is checked before the next line. The client's start line is one parenthesised unit, started whole as a background command of the operator's own shell tool, as rows 2440-CP, 2569-R and 2569-S did: no window, standard input from the null device, the interpreter `python -I -X utf8`; type no `&`, and record how it was started. Its exit is read from the evidence file it appends to. `client.py` itself ends its own server if that server has not exited 30 s after the client closed its input (`client.py:111-113`); that is the helper's own act, and it writes the log line `server did not exit within 30 s of stdin close; killed`, which is a STOP and report. Take no screenshot between part 4 and the end of part 7 unless a STOP needs one.

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
- `check_sheet.py`: row 2162-RX's checks, with this row's arms and the arms and launches of row 2162-RX it does not run; the skips of `local/` and the sitting's outputs named above; each launch file's five settings and data folder; each launch's exe path and its build's hash in that launch's literal hash-check and client start lines, and its launch file's copy line, each exactly once; and no build called by the two words other sheets use.
- `RUN-SHEET.md` and `SHA256.txt`.

New: `grade05.py`, the one helper this row adds (read-only). `grade05.py grade` reads a `05` answer. `grade05.py importeq` compares two admitted builds' batch files with exactly one value set aside: the voucher identity the build derives from the batch id and the voucher's `bridge_txn_id`. The batch id itself is not in the file; that identity is, once as the voucher's `REMOTEID` and once in the narration's `[BRIDGE:...]` tag (code `src-tauri/src/agent_import.rs:5786-5799`, `src-tauri/src/agent_import_identity.rs:16-23`). Nothing else is ignored: no whitespace, no encoding, no other field. No existing helper does either: `calls.py reqdiff` lists differing lines but gives no yes or no, and the identity differs between any two builds.

At the sitting, `ratereq.py` generates `rates.xml`; `run.py` generates `a-*` answers and `log-*.txt`; launches generate their run and data folders; C5 generates the backup; step 0 generates `dl/` and `exe-6f541ae4/`. These are outputs, not missing sheet inputs.

## Self-checks before the sitting

Run from `/c/bridge-lab/2162-rc`. No Tally request, `client.py` launch, or restore is part of them.

```bash
cd /c/bridge-lab/2162-rc && python -I ratereq.py --selftest > evidence/selfcheck-ratereq.txt 2>&1; echo "exit $?" >> evidence/selfcheck-ratereq.txt; cat evidence/selfcheck-ratereq.txt
cd /c/bridge-lab/2162-rc && python -I -X utf8 selftest-rate.py > evidence/selfcheck-rate.txt 2>&1; echo "exit $?" >> evidence/selfcheck-rate.txt; cat evidence/selfcheck-rate.txt
cd /c/bridge-lab/2162-rc && python -I -X utf8 grade05.py --selftest > evidence/selfcheck-grade05.txt 2>&1; echo "exit $?" >> evidence/selfcheck-grade05.txt; cat evidence/selfcheck-grade05.txt
cd /c/bridge-lab/2162-rc && python -I -c "import ast,sys;[ast.parse(open(f, encoding='utf-8').read()) for f in sys.argv[1:]]" *.py > evidence/selfcheck-parse.txt 2>&1; echo "exit $?" >> evidence/selfcheck-parse.txt; cat evidence/selfcheck-parse.txt
cd /c/bridge-lab/2162-rc && python -I -X utf8 check_sheet.py > evidence/selfcheck-sheet.txt 2>&1; echo "exit $?" >> evidence/selfcheck-sheet.txt; cat evidence/selfcheck-sheet.txt
```

EXPECT, in order: `96b41d40c58d9527da459219d4ea1b020b3c7e6688053ce3de7a42af892cb812` then `equal to the pinned hash`, exit 0; `23 cases, 23 right`, exit 0; `15 cases, 15 right`, exit 0; no output but `exit 0`; three `OK:` lines, exit 0. Anything else: STOP.

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
   - Make the row folder and copy this sheet's folder into it byte for byte from the lab's Bridge clone. In the second line, replace `<CLONE>` (both places) with the Git Bash path of the lab's own Bridge clone or sheet worktree (the lab knows it; the run coordinator's GO names the branch and `<SHEET>`), and `<SHEET>` with the 40-character commit that GO names; these are the only two values this sheet cannot write out. The fetch brings the sheet's commit into that clone. EXPECT `made`, then the printed line `exit 0 0`. Any other (an empty second number means the fetch failed): STOP.
     ```bash
     if [ -e /c/bridge-lab/2162-rc ]; then echo "STOP: /c/bridge-lab/2162-rc exists"; else mkdir /c/bridge-lab/2162-rc && echo "made /c/bridge-lab/2162-rc"; fi
     git -C <CLONE> fetch origin && git -C <CLONE> archive <SHEET> lab/2162-rc | tar -x -C /c/bridge-lab/2162-rc --strip-components=2; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"
     cd /c/bridge-lab/2162-rc
     cd /c/bridge-lab/2162-rc && if [ -e evidence ] || [ -e local ]; then echo "STOP: evidence or local exists"; else mkdir evidence local && echo "made evidence local"; fi
     ```
   - Run all five Self-checks. Any miss: STOP.
   - BUILD 8c3fb9f2. EXPECT `: OK`.
     ```bash
     cd /c/bridge-lab/2162-rc && echo "1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59  C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > evidence/s0-build-8c3fb9f2.txt 2>&1; echo "exit $?" >> evidence/s0-build-8c3fb9f2.txt; cat evidence/s0-build-8c3fb9f2.txt
     ```
   - BUILD 6f541ae4: the one download. Type each line only after the one before met its EXPECT. EXPECT, in order: `made`; `gh exit 0`; `60692023`; `: OK` for the zip; the listing holds `bridge-tally.mcpb` and mcpb-smoke.json at the top of the zip; the two extracted paths printed; `: OK` for the mcpb; `archive_sha256 5a6b02433a670cb128495f45a1bd8a098f2a7d45d948d8e6f443423e9eb7f91f`, `binary_sha256 9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc` and `server_version 0.7.0` (record `platform`, `default_tool_count` and `opt_in_tool_count`; the run coordinator's line says 37 tools with the opt-in set); the mcpb unpacked with no error; `: OK` for the exe. Any `FAILED`, any non-zero exit, or a value that differs: STOP, and nothing is launched from this folder. The download is bounded: no `gh exit` line after 10 minutes is a STOP; never start a second download into the same folder. A non-zero `gh exit` (not signed in, artifact gone): STOP and report. The listing also shows the installers `msi/Bridge_0.7.0_x64_en-US.msi` and `nsis/Bridge_0.7.0_x64-setup.exe`: they are expected there and are never extracted, opened or run. The only file of the artifact that is ever executed is the unpacked `bridge_mcp.exe`, started by `client.py`. The zip stays local.
     ```bash
     cd /c/bridge-lab/2162-rc && if [ -e dl ] || [ -e exe-6f541ae4 ]; then echo "STOP: dl or exe-6f541ae4 exists"; else mkdir dl exe-6f541ae4 && echo "made dl exe-6f541ae4"; fi
     cd /c/bridge-lab/2162-rc && gh api repos/ComplyEaze/bridge/actions/artifacts/14901146187/zip > dl/unsigned-bundle-smoke-windows-latest.zip; echo "gh exit $?" | tee evidence/s0-download.txt
     cd /c/bridge-lab/2162-rc && stat -c '%s' dl/unsigned-bundle-smoke-windows-latest.zip | tee -a evidence/s0-download.txt
     cd /c/bridge-lab/2162-rc && echo "a83111827ad0319047d4a23b98f52986e705a39c102a7fb829450d99c5c74dc0  dl/unsigned-bundle-smoke-windows-latest.zip" | sha256sum -c - 2>&1 | tee -a evidence/s0-download.txt
     cd /c/bridge-lab/2162-rc && python -I -m zipfile -l dl/unsigned-bundle-smoke-windows-latest.zip > evidence/s0-zip-list.txt 2>&1; echo "exit $?" >> evidence/s0-zip-list.txt; cat evidence/s0-zip-list.txt
     cd /c/bridge-lab/2162-rc && python -I -c "import sys,zipfile; z=zipfile.ZipFile(sys.argv[1]); [print(z.extract(n, sys.argv[2])) for n in ('bridge-tally.mcpb', 'mcpb-smoke.json')]" dl/unsigned-bundle-smoke-windows-latest.zip dl/unpacked 2>&1 | tee -a evidence/s0-download.txt
     cd /c/bridge-lab/2162-rc && echo "5a6b02433a670cb128495f45a1bd8a098f2a7d45d948d8e6f443423e9eb7f91f  dl/unpacked/bridge-tally.mcpb" | sha256sum -c - 2>&1 | tee -a evidence/s0-download.txt
     cd /c/bridge-lab/2162-rc && python -I -c "import json,sys; d=json.load(open(sys.argv[1], encoding='utf-8')); [print(k, d.get(k, '<absent>')) for k in ('archive_sha256', 'binary_sha256', 'platform', 'server_version', 'default_tool_count', 'opt_in_tool_count')]" dl/unpacked/mcpb-smoke.json 2>&1 | tee -a evidence/s0-download.txt
     cd /c/bridge-lab/2162-rc && python -I -m zipfile -e dl/unpacked/bridge-tally.mcpb exe-6f541ae4; echo "unpack exit $?" | tee -a evidence/s0-download.txt
     cd /c/bridge-lab/2162-rc && echo "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc  C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - 2>&1 | tee -a evidence/s0-download.txt
     ```
   - C2 step 0 with no company loaded. EXPECT one `tally.exe` line and its PID; a `LISTENING` line on `:9001` with that PID; `curl exit 0` and `200`; no dialog in the first screenshot. Otherwise STOP. `netstat`'s file is kept whole; the `grep` only shows the lines to read.
     ```bash
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > evidence/s0-tasklist.txt 2>&1; echo "exit $?" >> evidence/s0-tasklist.txt; cat evidence/s0-tasklist.txt
     cd /c/bridge-lab/2162-rc && netstat -ano > evidence/s0-netstat.txt 2>&1; echo "exit $?" >> evidence/s0-netstat.txt; grep -E ':9001 |^exit' evidence/s0-netstat.txt
     cd /c/bridge-lab/2162-rc && curl.exe -sS -w "%{http_code} %{size_download}\n" http://127.0.0.1:9001/status -o evidence/s0-status-body.txt > evidence/s0-status.txt 2>&1; echo "curl exit $?" >> evidence/s0-status.txt; cat evidence/s0-status.txt
     ```
1. **Load, check, back up, and render the rate request.**
   - Load BRIDGE PILOT LAB alone from Select Company (C1): click the filter; type `BRIDGE`, space, `PILOT` until it is the one highlighted line, Enter. No Escape at Select Company. EXPECT the Gateway, current date and period in Book. Otherwise STOP.
   - Send `start`: `M-marks.xml`; parse it. EXPECT one row, the pinned GUID, V 68, M 245. Otherwise STOP before BK1.
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 start M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py a-start-M-marks.xml > evidence/start-count19.txt 2>&1; echo "exit $?" >> evidence/start-count19.txt; cat evidence/start-count19.txt
     ```
   - Make `backup-1` new and empty; take BK1 by C5, with the keys row 2162-RW's notes record. Click the Gateway's left panel; Alt+Y: EXPECT the Data menu; Enter (Backup & Restore); Enter (Backup): EXPECT the Backup screen. In Backup Data Path press Backspace: EXPECT the List of Folders; Up three times, one at a time with a zoom after each (row 2162-RW passed TallyDrive, Select from Drive, then `Specify Path` highlighted); Enter; type the full path of `backup-1` (zoom: exact); Enter: EXPECT the list with that folder highlighted; Enter: EXPECT the path to read that folder. In the company filter type `BRIDGE`, space, `PILOT`: EXPECT one line, BRIDGE PILOT LAB, highlighted; Enter; click `End of List` (zoom: highlighted); Enter. EXPECT `Accept ?` showing that path and the one company: screenshot, click inside the box, `y`. EXPECT the Gateway and no message. A line, list or prompt other than these: path rule. Record its masked file name, bytes, SHA-256 and Restore-list time. EXPECT exactly one non-empty backup file. Otherwise STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && if [ -e backup-1 ]; then echo "STOP: backup-1 exists"; else mkdir backup-1 && echo "made backup-1"; fi
     # after the backup: the listing and the hash stay local (the file name holds the company number)
     cd /c/bridge-lab/2162-rc && ls -la backup-1 > local/bk1.txt 2>&1; sha256sum backup-1/* >> local/bk1.txt 2>&1; echo "exit $?" >> local/bk1.txt; cat local/bk1.txt
     ```
   - Send `setup`: `M-marks.xml guids.xml`, in that order. EXPECT marks still 68/245. A moved mark: STOP and report. Then parse the GUIDs. EXPECT `LEDGER ROWS 6` and exactly the six names of call `05`, each with a `BRIDGECOMPANYGUID` equal to the book's GUID and its own GUID the book's GUID followed by the suffix rows 2162-RW and 2162-RX read: `BRIDGE Walk-in` `-000000d0`, `BRIDGE Svc 998313 5%` `-000000d1`, `BRIDGE Svc 998314 5%` `-000000d2` (both under `Sales Accounts`), `BRIDGE CGST 2.5%` `-000000d4`, `BRIDGE SGST 2.5%` `-000000d6`, `BRIDGE Round Off` `-000000d8` (lab 2162-RX, notes). Any other count, name or GUID, or a `READ refused` line: STOP (this row's byte EXPECTs rest on the same request).
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 setup M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && grep '  guids.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 setup guids.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-setup-M-marks.xml > evidence/setup-count19.txt 2>&1; echo "exit $?" >> evidence/setup-count19.txt; cat evidence/setup-count19.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-setup-guids.xml > evidence/setup-guids.txt 2>&1; echo "exit $?" >> evidence/setup-guids.txt; cat evidence/setup-guids.txt
     ```
   - Render the one rate request used all sitting (the build's window is financial-year start through the invoice date; code `src-tauri/src/agent_import_invoice_wire.rs:1206-1212`). The book's GUID is read from `calls.py`, so no GUID is typed. EXPECT `rates.xml: 2874 bytes, sha256 182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08` (lab 2162-RX), then `rates.xml: OK`. Anything else: STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && G=$(sed -n 's/^BOOK = "\([0-9a-f-]*\)".*/\1/p' calls.py) && echo "book GUID read: ${#G} characters" > evidence/ratereq-out.txt && python -I ratereq.py --company "BRIDGE PILOT LAB" --from-date 20260401 --to-date 20260801 --output rates.xml "$G-000000d0" "$G-000000d1" "$G-000000d2" "$G-000000d4" "$G-000000d6" "$G-000000d8" >> evidence/ratereq-out.txt 2>&1; echo "exit $?" >> evidence/ratereq-out.txt; cat evidence/ratereq-out.txt
     cd /c/bridge-lab/2162-rc && echo "182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08  rates.xml" > local/rates.sha256; sha256sum -c local/rates.sha256
     ```
2. **P: the plain state, BUILD 6f541ae4.**
   - With no key pressed and no report opened between them (the C3 screenshot after each send is still taken), send `A0-first`: `M-marks.xml rates.xml`, then `A0-second`: `rates.xml`. `count19.py --same-row a-start-M-marks.xml a-A0-first-M-marks.xml` must pass. A moved mark: STOP and report.
   - `python -I ratecmp.py a-A0-first-rates.xml a-A0-second-rates.xml`. EXPECT `LEDGER ROWS 6`, the six names, every one `SHAPE measured`, no structural difference, and **equal byte sizes and SHA-256** (lab 2162-RW). A missing/other row, unequal hash, an unmeasured shape or a `READ refused` line: STOP, with the one exception below.
   - The exception: if both answers show `LEDGER ROWS 6`, the six names, every one `SHAPE measured` and no `READ refused` line, and the only failure is that the two differ, send `A0-third`: `rates.xml`, once. If `a-A0-second-rates.xml` and `a-A0-third-rates.xml` are equal in bytes and SHA-256, record a first-read effect with the whole `ratecmp.py` output of first against second, and use `a-A0-second-rates.xml` wherever this sheet names `a-A0-first-rates.xml`. If second and third differ too: STOP.
   - The commands, and the plain check: EXPECT `LEDGER ROWS 6`, the six names, no structural difference between the two, and both answers `: OK` against the plain sha256 and 29114 bytes (lab 2162-RX). A `READ refused` line, or an answer not plain other than under the exception: STOP (the row's premise for P failed).
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 A0-first M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 A0-first rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 A0-second rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-A0-first-M-marks.xml > evidence/A0-first-count19.txt 2>&1; echo "exit $?" >> evidence/A0-first-count19.txt; cat evidence/A0-first-count19.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-A0-first-rates.xml a-A0-second-rates.xml > evidence/A0-first-vs-A0-second.txt 2>&1; echo "exit $?" >> evidence/A0-first-vs-A0-second.txt; cat evidence/A0-first-vs-A0-second.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-A0-first-rates.xml > evidence/A0-first-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-A0-first-rates.xml" | sha256sum -c - >> evidence/A0-first-is-plain.txt 2>&1; echo "exit $?" >> evidence/A0-first-is-plain.txt; cat evidence/A0-first-is-plain.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-A0-second-rates.xml > evidence/A0-second-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-A0-second-rates.xml" | sha256sum -c - >> evidence/A0-second-is-plain.txt 2>&1; echo "exit $?" >> evidence/A0-second-is-plain.txt; cat evidence/A0-second-is-plain.txt
     ```
   - Only under row 2162-RX's exception above (the two differ, and `A0-second` is the plain answer): send `A0-third`, once. EXPECT `A0-second` and `A0-third` both `: OK` against the plain sha256 and no structural difference between them; record a first-read effect with the whole output of first against second. Otherwise STOP. This sheet's later comparisons use `a-A0-second-rates.xml` as the first plain answer in every case: it equals `a-A0-first-rates.xml` by the EXPECT above, or replaces it under the exception, so the copied frame's `a-A0-first-rates.xml` is read as `a-A0-second-rates.xml`.
     ```bash
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 A0-third rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-A0-third-rates.xml > evidence/A0-second-vs-A0-third.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-A0-third.txt; cat evidence/A0-second-vs-A0-third.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-A0-third-rates.xml > evidence/A0-third-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-A0-third-rates.xml" | sha256sum -c - >> evidence/A0-third-is-plain.txt 2>&1; echo "exit $?" >> evidence/A0-third-is-plain.txt; cat evidence/A0-third-is-plain.txt
     ```
   - Launch LP: its seven parts follow. EXPECT `VERDICT admitted voucher_count 1, no refusal` and the batch file equal to the answer's sha256. Record `batch`'s whole output: the batch file's sha256 is one of this row's results. Any other verdict: record it whole, close, finish this arm's read, then STOP: the change broke the plain case.
   - **Launch LP, part 1: folders.** EXPECT `made`; `STOP` is a STOP (never reuse a launch folder).
     ```bash
     cd /c/bridge-lab/2162-rc && if [ -e run-p ] || [ -e data-p ]; then echo "STOP: run-p or data-p exists"; else mkdir run-p run-p/inbox run-p/answers data-p && echo "made run-p run-p/inbox run-p/answers data-p"; fi
     ```
   - **Launch LP, part 2: the exe and the launch file.** EXPECT two lines ending `: OK`. `FAILED` or no output: STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && echo "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc  C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > evidence/run-p-hashes.txt 2>&1; echo "exit $?" >> evidence/run-p-hashes.txt; cat evidence/run-p-hashes.txt
     cd /c/bridge-lab/2162-rc && cp launch/LP-launch-env.json run-p/launch-env.json && grep '  launch/LP-launch-env.json$' SHA256.txt | awk '{print $1"  run-p/launch-env.json"}' | sha256sum -c - >> evidence/run-p-hashes.txt 2>&1; echo "exit $?" >> evidence/run-p-hashes.txt; cat evidence/run-p-hashes.txt
     ```
   - **Launch LP, part 3: other Bridge processes.** EXPECT `INFO: No tasks are running` and `exit 0`; a `bridge_mcp.exe` line: STOP and report (none may call port 9001 during this sitting).
     ```bash
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-p-procs-before.txt 2>&1; echo "exit $?" >> evidence/run-p-procs-before.txt; cat evidence/run-p-procs-before.txt
     ```
   - **Launch LP, part 4: start the client.** Start the first line, whole, with the operator's shell tool's own background run, as rows 2440-CP, 2569-R and 2569-S did: no window, standard input from the null device. Type no `&`, change nothing in the line, and record in the notes how it was started. It prints nothing and runs until part 7's stop file. Then, within 60 s, type the second line. EXPECT the exe hash `ok`, the launch file's hash equal to `SHA256.txt`'s, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` with version 0.7.0. A `stop:` line, or no `initialized` line in 60 s: STOP. No screenshot from here to the end of part 7 unless a STOP needs one.
     ```bash
     ( cd /c/bridge-lab/2162-rc && python -I -X utf8 client.py "C:/bridge-lab/2162-rc/run-p" "C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc" < /dev/null > evidence/run-p-client-out.txt 2> evidence/run-p-client-err.txt; echo "client exit $?" >> evidence/run-p-launch.txt )
     cd /c/bridge-lab/2162-rc && cat run-p/client-log.txt
     ```
   - **Launch LP, part 5: the five calls, one at a time, in this order.** Each call has three lines. The first copies the call in under a name ending `.part`, checks its hash against `SHA256.txt`, and only then renames it (the client opens a name ending `.json` at once; lab 2162-SALES-Q, deviation 4); EXPECT `: OK` and `placed`. The second shows whether its answer has arrived: type it until `run-p/answers/` lists the call's name and the log's last lines show the call's line; five minutes without them is a STOP. Only then type the third, which grades the answer; wait for its EXPECT before the next call. Exit 2 from the third line, typed after the answer arrived, is a failed call: STOP. No other call file is put in; none is written, edited or brought from another sheet, and in particular no post or verify call.
     ```bash
     cd /c/bridge-lab/2162-rc && cp calls/01-list.json run-p/inbox/01-list.json.part && grep '  calls/01-list.json$' SHA256.txt | awk '{print $1"  run-p/inbox/01-list.json.part"}' | sha256sum -c - && mv run-p/inbox/01-list.json.part run-p/inbox/01-list.json && echo "placed 01-list.json" | tee -a evidence/run-p-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-p/answers; tail -n 3 run-p/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-p/answers/01-list.json > evidence/run-p-show-01-list.json.txt 2>&1; echo "exit $?" >> evidence/run-p-show-01-list.json.txt; cat evidence/run-p-show-01-list.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/02-list_companies.json run-p/inbox/02-list_companies.json.part && grep '  calls/02-list_companies.json$' SHA256.txt | awk '{print $1"  run-p/inbox/02-list_companies.json.part"}' | sha256sum -c - && mv run-p/inbox/02-list_companies.json.part run-p/inbox/02-list_companies.json && echo "placed 02-list_companies.json" | tee -a evidence/run-p-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-p/answers; tail -n 3 run-p/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-p/answers/02-list_companies.json > evidence/run-p-show-02-list_companies.json.txt 2>&1; echo "exit $?" >> evidence/run-p-show-02-list_companies.json.txt; cat evidence/run-p-show-02-list_companies.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/03-marks.json run-p/inbox/03-marks.json.part && grep '  calls/03-marks.json$' SHA256.txt | awk '{print $1"  run-p/inbox/03-marks.json.part"}' | sha256sum -c - && mv run-p/inbox/03-marks.json.part run-p/inbox/03-marks.json && echo "placed 03-marks.json" | tee -a evidence/run-p-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-p/answers; tail -n 3 run-p/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-p/answers/03-marks.json > evidence/run-p-show-03-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-p-show-03-marks.json.txt; cat evidence/run-p-show-03-marks.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/05-build-0074-0801.json run-p/inbox/05-build-0074-0801.json.part && grep '  calls/05-build-0074-0801.json$' SHA256.txt | awk '{print $1"  run-p/inbox/05-build-0074-0801.json.part"}' | sha256sum -c - && mv run-p/inbox/05-build-0074-0801.json.part run-p/inbox/05-build-0074-0801.json && echo "placed 05-build-0074-0801.json" | tee -a evidence/run-p-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-p/answers; tail -n 3 run-p/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 grade05.py grade run-p/answers/05-build-0074-0801.json > evidence/run-p-grade05.txt 2>&1; echo "exit $?" >> evidence/run-p-grade05.txt; cat evidence/run-p-grade05.txt
     cd /c/bridge-lab/2162-rc && cp calls/07-marks.json run-p/inbox/07-marks.json.part && grep '  calls/07-marks.json$' SHA256.txt | awk '{print $1"  run-p/inbox/07-marks.json.part"}' | sha256sum -c - && mv run-p/inbox/07-marks.json.part run-p/inbox/07-marks.json && echo "placed 07-marks.json" | tee -a evidence/run-p-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-p/answers; tail -n 3 run-p/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-p/answers/07-marks.json > evidence/run-p-show-07-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-p-show-07-marks.json.txt; cat evidence/run-p-show-07-marks.json.txt
     ```
     EXPECT `01` `tools: 37; missing of this row's: []` (lab 2162-RX for BUILD 8c3fb9f2; for BUILD 6f541ae4 the run coordinator's line says 37: record the count, and grade only `missing ... []`); `02` one company with the pinned GUID; `03` and `07` M 245, V 68, reused False (a moved mark is a STOP). `05` is graded by `grade05.py`, not by `calls.py show` (row 2162-RX's `show` of a refused build printed no refusal: its refusals are under `result.refusals`, which `calls.py` does not read). EXPECT `VERDICT admitted voucher_count 1, no refusal`. Exit 2 from a third line, typed after the answer arrived, is a failed call: STOP. An admitted answer's own `next_step` text names `post_import`: it is not followed; no post call is ever placed. Do NOT EXPECT the earlier rows' evidence sizes or digests for BUILD 6f541ae4: record them.
   - **Launch LP, part 6: the batch file.** EXPECT `equal to the answer's sha256`; `FINDING` (exit 1) is recorded and is a STOP. The second line keeps the data folder's listing whole. If part 5's `VERDICT` was not `admitted`, type only the second line and record whether it lists a file ending `.xml`.
     ```bash
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py batch run-p/answers/05-build-0074-0801.json data-p > evidence/run-p-batch.txt 2>&1; echo "exit $?" >> evidence/run-p-batch.txt; cat evidence/run-p-batch.txt
     cd /c/bridge-lab/2162-rc && find data-p -type f > evidence/run-p-data-files.txt 2>&1; echo "exit $?" >> evidence/run-p-data-files.txt; cat evidence/run-p-data-files.txt
     ```
   - **Launch LP, part 7: close.** Type the first line. Then type the second line until `evidence/run-p-launch.txt` ends with a `client exit` line (the client ends within about 30 s of the stop file). Then type the third. EXPECT the log's last line `session ended; server pid <pid> exit code 0`, `client exit 0` as the launch file's last line, the server error log's size, and then `INFO: No tasks are running`. Never end a process. The log line `server did not exit within 30 s of stdin close; killed` (`client.py:111-113`: the helper ends its own server) is a STOP and report. No `client exit` line in 60 s, any other exit, or a server that does not end: STOP and report.
     ```bash
     cd /c/bridge-lab/2162-rc && touch run-p/inbox/stop && echo "stop file placed" | tee -a evidence/run-p-launch.txt
     cd /c/bridge-lab/2162-rc && tail -n 1 run-p/client-log.txt; cat evidence/run-p-launch.txt; stat -c '%n %s' run-p/server-stderr-*.log
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-p-procs-after.txt 2>&1; echo "exit $?" >> evidence/run-p-procs-after.txt; cat evidence/run-p-procs-after.txt
     ```
   - Send `P-post`: `M-marks.xml rates.xml`. EXPECT marks equal to `start`, the answer plain, and no structural difference from `A0-second`. Otherwise STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 P-post M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 P-post rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-P-post-M-marks.xml > evidence/P-post-count19.txt 2>&1; echo "exit $?" >> evidence/P-post-count19.txt; cat evidence/P-post-count19.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-P-post-rates.xml > evidence/P-post-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-P-post-rates.xml" | sha256sum -c - >> evidence/P-post-is-plain.txt 2>&1; echo "exit $?" >> evidence/P-post-is-plain.txt; cat evidence/P-post-is-plain.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-P-post-rates.xml > evidence/A0-second-vs-P-post.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-P-post.txt; cat evidence/A0-second-vs-P-post.txt
     ```
3. **G: Shut Company, a load, and the W1 walk.**
   - Frame a.
   - Frame b's commands, after frame a and before frame c. EXPECT `G-pre2` `: OK` against the plain sha256 and no structural difference from `A0-second` (the state cleared; not cleared is a STOP, as frame b says). `G-pre` against `G-pre2` is recorded: a difference is a first-read-after-load effect, not a STOP (frame b).
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G-pre M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G-pre rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G-pre2 rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G-pre-M-marks.xml > evidence/G-pre-count19.txt 2>&1; echo "exit $?" >> evidence/G-pre-count19.txt; cat evidence/G-pre-count19.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-G-pre-rates.xml a-G-pre2-rates.xml > evidence/G-pre-vs-G-pre2.txt 2>&1; echo "exit $?" >> evidence/G-pre-vs-G-pre2.txt; cat evidence/G-pre-vs-G-pre2.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-G-pre2-rates.xml > evidence/A0-second-vs-G-pre2.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-G-pre2.txt; cat evidence/A0-second-vs-G-pre2.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-G-pre2-rates.xml > evidence/G-pre2-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-G-pre2-rates.xml" | sha256sum -c - >> evidence/G-pre2-is-plain.txt 2>&1; echo "exit $?" >> evidence/G-pre2-is-plain.txt; cat evidence/G-pre2-is-plain.txt
     ```
   - Frame c: open the HSN summary. Down twice from 847130, one at a time with a zoom after each (9983, then `998313` alone highlighted); Enter: EXPECT `GSTR-1 - Voucher Register`, `Vouchers of 998313` (lab 2162-SALES-Q step 12.5; lab 2162-RW step 5). Esc once: EXPECT the summary, highlight 998313. Down once (zoom: `998314` alone highlighted); Enter: EXPECT `Vouchers of 998314`. Esc once: EXPECT the summary, highlight 998314. Esc once: EXPECT `GSTR-1`. Esc once: EXPECT the Gateway. No further Escape.
   - Frame d (`G`), after the walk. EXPECT the answer grown (`: OK` against the grown sha256, 30626 bytes), and `ratecmp.py` against `A0-second` to show added 28, removed 0, text changed 0, every differing path under `BRIDGE Svc 998313 5%` or `BRIDGE Svc 998314 5%`, those two `SHAPE unmeasured` (BUILD 8c3fb9f2's rule) and the other four measured (lab 2162-RX, W1). **Not grown: record it whole, launch nothing, STOP** (the row's premise failed). A `READ refused` line or an exit 2: STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G-M-marks.xml > evidence/G-count19.txt 2>&1; echo "exit $?" >> evidence/G-count19.txt; cat evidence/G-count19.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-G-rates.xml > evidence/G-is-grown.txt 2>&1; echo "d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257  a-G-rates.xml" | sha256sum -c - >> evidence/G-is-grown.txt 2>&1; echo "exit $?" >> evidence/G-is-grown.txt; cat evidence/G-is-grown.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-G-rates.xml > evidence/A0-second-vs-G.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-G.txt; cat evidence/A0-second-vs-G.txt
     ```
   - **From here to the end of G2's read: no Shut Company, no load, no key and no screen act in Tally.** Screenshots only.
4. **G1: BUILD 8c3fb9f2 in the grown state.**
   - Launch LG1: its seven parts follow. EXPECT `VERDICT refused invoice_not_admitted: invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998313 5%), invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998314 5%)`, and at part 6 no `.xml` file in `data-g1` (lab 2162-RX: refused, no file; its evidence was 62,071 bytes, request `6d6404a2`, response `5db1811d`; recorded, not graded).
   - **Launch LG1, part 1: folders.** EXPECT `made`; `STOP` is a STOP (never reuse a launch folder).
     ```bash
     cd /c/bridge-lab/2162-rc && if [ -e run-g1 ] || [ -e data-g1 ]; then echo "STOP: run-g1 or data-g1 exists"; else mkdir run-g1 run-g1/inbox run-g1/answers data-g1 && echo "made run-g1 run-g1/inbox run-g1/answers data-g1"; fi
     ```
   - **Launch LG1, part 2: the exe and the launch file.** EXPECT two lines ending `: OK`. `FAILED` or no output: STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && echo "1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59  C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > evidence/run-g1-hashes.txt 2>&1; echo "exit $?" >> evidence/run-g1-hashes.txt; cat evidence/run-g1-hashes.txt
     cd /c/bridge-lab/2162-rc && cp launch/LG1-launch-env.json run-g1/launch-env.json && grep '  launch/LG1-launch-env.json$' SHA256.txt | awk '{print $1"  run-g1/launch-env.json"}' | sha256sum -c - >> evidence/run-g1-hashes.txt 2>&1; echo "exit $?" >> evidence/run-g1-hashes.txt; cat evidence/run-g1-hashes.txt
     ```
   - **Launch LG1, part 3: other Bridge processes.** EXPECT `INFO: No tasks are running` and `exit 0`; a `bridge_mcp.exe` line: STOP and report (none may call port 9001 during this sitting).
     ```bash
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-g1-procs-before.txt 2>&1; echo "exit $?" >> evidence/run-g1-procs-before.txt; cat evidence/run-g1-procs-before.txt
     ```
   - **Launch LG1, part 4: start the client.** Start the first line, whole, with the operator's shell tool's own background run, as rows 2440-CP, 2569-R and 2569-S did: no window, standard input from the null device. Type no `&`, change nothing in the line, and record in the notes how it was started. It prints nothing and runs until part 7's stop file. Then, within 60 s, type the second line. EXPECT the exe hash `ok`, the launch file's hash equal to `SHA256.txt`'s, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` with version 0.7.0. A `stop:` line, or no `initialized` line in 60 s: STOP. No screenshot from here to the end of part 7 unless a STOP needs one.
     ```bash
     ( cd /c/bridge-lab/2162-rc && python -I -X utf8 client.py "C:/bridge-lab/2162-rc/run-g1" "C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" "1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59" < /dev/null > evidence/run-g1-client-out.txt 2> evidence/run-g1-client-err.txt; echo "client exit $?" >> evidence/run-g1-launch.txt )
     cd /c/bridge-lab/2162-rc && cat run-g1/client-log.txt
     ```
   - **Launch LG1, part 5: the five calls, one at a time, in this order.** Each call has three lines. The first copies the call in under a name ending `.part`, checks its hash against `SHA256.txt`, and only then renames it (the client opens a name ending `.json` at once; lab 2162-SALES-Q, deviation 4); EXPECT `: OK` and `placed`. The second shows whether its answer has arrived: type it until `run-g1/answers/` lists the call's name and the log's last lines show the call's line; five minutes without them is a STOP. Only then type the third, which grades the answer; wait for its EXPECT before the next call. Exit 2 from the third line, typed after the answer arrived, is a failed call: STOP. No other call file is put in; none is written, edited or brought from another sheet, and in particular no post or verify call.
     ```bash
     cd /c/bridge-lab/2162-rc && cp calls/01-list.json run-g1/inbox/01-list.json.part && grep '  calls/01-list.json$' SHA256.txt | awk '{print $1"  run-g1/inbox/01-list.json.part"}' | sha256sum -c - && mv run-g1/inbox/01-list.json.part run-g1/inbox/01-list.json && echo "placed 01-list.json" | tee -a evidence/run-g1-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g1/answers; tail -n 3 run-g1/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-g1/answers/01-list.json > evidence/run-g1-show-01-list.json.txt 2>&1; echo "exit $?" >> evidence/run-g1-show-01-list.json.txt; cat evidence/run-g1-show-01-list.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/02-list_companies.json run-g1/inbox/02-list_companies.json.part && grep '  calls/02-list_companies.json$' SHA256.txt | awk '{print $1"  run-g1/inbox/02-list_companies.json.part"}' | sha256sum -c - && mv run-g1/inbox/02-list_companies.json.part run-g1/inbox/02-list_companies.json && echo "placed 02-list_companies.json" | tee -a evidence/run-g1-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g1/answers; tail -n 3 run-g1/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-g1/answers/02-list_companies.json > evidence/run-g1-show-02-list_companies.json.txt 2>&1; echo "exit $?" >> evidence/run-g1-show-02-list_companies.json.txt; cat evidence/run-g1-show-02-list_companies.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/03-marks.json run-g1/inbox/03-marks.json.part && grep '  calls/03-marks.json$' SHA256.txt | awk '{print $1"  run-g1/inbox/03-marks.json.part"}' | sha256sum -c - && mv run-g1/inbox/03-marks.json.part run-g1/inbox/03-marks.json && echo "placed 03-marks.json" | tee -a evidence/run-g1-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g1/answers; tail -n 3 run-g1/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-g1/answers/03-marks.json > evidence/run-g1-show-03-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-g1-show-03-marks.json.txt; cat evidence/run-g1-show-03-marks.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/05-build-0074-0801.json run-g1/inbox/05-build-0074-0801.json.part && grep '  calls/05-build-0074-0801.json$' SHA256.txt | awk '{print $1"  run-g1/inbox/05-build-0074-0801.json.part"}' | sha256sum -c - && mv run-g1/inbox/05-build-0074-0801.json.part run-g1/inbox/05-build-0074-0801.json && echo "placed 05-build-0074-0801.json" | tee -a evidence/run-g1-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g1/answers; tail -n 3 run-g1/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 grade05.py grade run-g1/answers/05-build-0074-0801.json > evidence/run-g1-grade05.txt 2>&1; echo "exit $?" >> evidence/run-g1-grade05.txt; cat evidence/run-g1-grade05.txt
     cd /c/bridge-lab/2162-rc && cp calls/07-marks.json run-g1/inbox/07-marks.json.part && grep '  calls/07-marks.json$' SHA256.txt | awk '{print $1"  run-g1/inbox/07-marks.json.part"}' | sha256sum -c - && mv run-g1/inbox/07-marks.json.part run-g1/inbox/07-marks.json && echo "placed 07-marks.json" | tee -a evidence/run-g1-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g1/answers; tail -n 3 run-g1/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-g1/answers/07-marks.json > evidence/run-g1-show-07-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-g1-show-07-marks.json.txt; cat evidence/run-g1-show-07-marks.json.txt
     ```
     EXPECT `01` `tools: 37; missing of this row's: []` (lab 2162-RX for BUILD 8c3fb9f2; for BUILD 6f541ae4 the run coordinator's line says 37: record the count, and grade only `missing ... []`); `02` one company with the pinned GUID; `03` and `07` M 245, V 68, reused False (a moved mark is a STOP). `05` is graded by `grade05.py`, not by `calls.py show` (row 2162-RX's `show` of a refused build printed no refusal: its refusals are under `result.refusals`, which `calls.py` does not read). EXPECT `VERDICT refused invoice_not_admitted: invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998313 5%), invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998314 5%)`. Exit 2 from a third line, typed after the answer arrived, is a failed call: STOP. Do NOT EXPECT the earlier rows' evidence sizes or digests for BUILD 6f541ae4: record them.
   - **Launch LG1, part 6: the data folder.** EXPECT `exit 0` and no file ending `.xml` in the listing (no file written). The listing is kept whole.
     ```bash
     cd /c/bridge-lab/2162-rc && find data-g1 -type f > evidence/run-g1-data-files.txt 2>&1; echo "exit $?" >> evidence/run-g1-data-files.txt; cat evidence/run-g1-data-files.txt
     ```
   - **Launch LG1, part 7: close.** Type the first line. Then type the second line until `evidence/run-g1-launch.txt` ends with a `client exit` line (the client ends within about 30 s of the stop file). Then type the third. EXPECT the log's last line `session ended; server pid <pid> exit code 0`, `client exit 0` as the launch file's last line, the server error log's size, and then `INFO: No tasks are running`. Never end a process. The log line `server did not exit within 30 s of stdin close; killed` (`client.py:111-113`: the helper ends its own server) is a STOP and report. No `client exit` line in 60 s, any other exit, or a server that does not end: STOP and report.
     ```bash
     cd /c/bridge-lab/2162-rc && touch run-g1/inbox/stop && echo "stop file placed" | tee -a evidence/run-g1-launch.txt
     cd /c/bridge-lab/2162-rc && tail -n 1 run-g1/client-log.txt; cat evidence/run-g1-launch.txt; stat -c '%n %s' run-g1/server-stderr-*.log
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-g1-procs-after.txt 2>&1; echo "exit $?" >> evidence/run-g1-procs-after.txt; cat evidence/run-g1-procs-after.txt
     ```
   - Send `G1-post`: `M-marks.xml rates.xml`. EXPECT marks equal to `start`, the answer grown, and no structural difference from `G`.
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G1-post M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G1-post rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G1-post-M-marks.xml > evidence/G1-post-count19.txt 2>&1; echo "exit $?" >> evidence/G1-post-count19.txt; cat evidence/G1-post-count19.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-G1-post-rates.xml > evidence/G1-post-is-grown.txt 2>&1; echo "d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257  a-G1-post-rates.xml" | sha256sum -c - >> evidence/G1-post-is-grown.txt 2>&1; echo "exit $?" >> evidence/G1-post-is-grown.txt; cat evidence/G1-post-is-grown.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-G-rates.xml a-G1-post-rates.xml > evidence/G-vs-G1-post.txt 2>&1; echo "exit $?" >> evidence/G-vs-G1-post.txt; cat evidence/G-vs-G1-post.txt
     ```
   - G1 admitted with `voucher_count 1`: record both answers whole, write `G2 not run: G1 admitted`, and go to step 6. The state did not refuse the build without the change this time, so G2 would decide nothing.
   - Any other verdict (another code, one refusal, or a refusal naming another ledger): record it whole, then STOP and report. G2 then runs only on the run coordinator's word.
   - `G1-post` not grown, or a structural difference from `G`: STOP before G2 (a state the launch changed would confound G2).
5. **G2: BUILD 6f541ae4 in the same grown state.**
   - Launch LG2: its seven parts follow. EXPECT `VERDICT admitted voucher_count 1, no refusal` and the batch file equal to the answer's sha256. Record `batch`'s whole output.
   - **Launch LG2, part 1: folders.** EXPECT `made`; `STOP` is a STOP (never reuse a launch folder).
     ```bash
     cd /c/bridge-lab/2162-rc && if [ -e run-g2 ] || [ -e data-g2 ]; then echo "STOP: run-g2 or data-g2 exists"; else mkdir run-g2 run-g2/inbox run-g2/answers data-g2 && echo "made run-g2 run-g2/inbox run-g2/answers data-g2"; fi
     ```
   - **Launch LG2, part 2: the exe and the launch file.** EXPECT two lines ending `: OK`. `FAILED` or no output: STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && echo "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc  C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > evidence/run-g2-hashes.txt 2>&1; echo "exit $?" >> evidence/run-g2-hashes.txt; cat evidence/run-g2-hashes.txt
     cd /c/bridge-lab/2162-rc && cp launch/LG2-launch-env.json run-g2/launch-env.json && grep '  launch/LG2-launch-env.json$' SHA256.txt | awk '{print $1"  run-g2/launch-env.json"}' | sha256sum -c - >> evidence/run-g2-hashes.txt 2>&1; echo "exit $?" >> evidence/run-g2-hashes.txt; cat evidence/run-g2-hashes.txt
     ```
   - **Launch LG2, part 3: other Bridge processes.** EXPECT `INFO: No tasks are running` and `exit 0`; a `bridge_mcp.exe` line: STOP and report (none may call port 9001 during this sitting).
     ```bash
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-g2-procs-before.txt 2>&1; echo "exit $?" >> evidence/run-g2-procs-before.txt; cat evidence/run-g2-procs-before.txt
     ```
   - **Launch LG2, part 4: start the client.** Start the first line, whole, with the operator's shell tool's own background run, as rows 2440-CP, 2569-R and 2569-S did: no window, standard input from the null device. Type no `&`, change nothing in the line, and record in the notes how it was started. It prints nothing and runs until part 7's stop file. Then, within 60 s, type the second line. EXPECT the exe hash `ok`, the launch file's hash equal to `SHA256.txt`'s, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` with version 0.7.0. A `stop:` line, or no `initialized` line in 60 s: STOP. No screenshot from here to the end of part 7 unless a STOP needs one.
     ```bash
     ( cd /c/bridge-lab/2162-rc && python -I -X utf8 client.py "C:/bridge-lab/2162-rc/run-g2" "C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc" < /dev/null > evidence/run-g2-client-out.txt 2> evidence/run-g2-client-err.txt; echo "client exit $?" >> evidence/run-g2-launch.txt )
     cd /c/bridge-lab/2162-rc && cat run-g2/client-log.txt
     ```
   - **Launch LG2, part 5: the five calls, one at a time, in this order.** Each call has three lines. The first copies the call in under a name ending `.part`, checks its hash against `SHA256.txt`, and only then renames it (the client opens a name ending `.json` at once; lab 2162-SALES-Q, deviation 4); EXPECT `: OK` and `placed`. The second shows whether its answer has arrived: type it until `run-g2/answers/` lists the call's name and the log's last lines show the call's line; five minutes without them is a STOP. Only then type the third, which grades the answer; wait for its EXPECT before the next call. Exit 2 from the third line, typed after the answer arrived, is a failed call: STOP. No other call file is put in; none is written, edited or brought from another sheet, and in particular no post or verify call.
     ```bash
     cd /c/bridge-lab/2162-rc && cp calls/01-list.json run-g2/inbox/01-list.json.part && grep '  calls/01-list.json$' SHA256.txt | awk '{print $1"  run-g2/inbox/01-list.json.part"}' | sha256sum -c - && mv run-g2/inbox/01-list.json.part run-g2/inbox/01-list.json && echo "placed 01-list.json" | tee -a evidence/run-g2-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g2/answers; tail -n 3 run-g2/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-g2/answers/01-list.json > evidence/run-g2-show-01-list.json.txt 2>&1; echo "exit $?" >> evidence/run-g2-show-01-list.json.txt; cat evidence/run-g2-show-01-list.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/02-list_companies.json run-g2/inbox/02-list_companies.json.part && grep '  calls/02-list_companies.json$' SHA256.txt | awk '{print $1"  run-g2/inbox/02-list_companies.json.part"}' | sha256sum -c - && mv run-g2/inbox/02-list_companies.json.part run-g2/inbox/02-list_companies.json && echo "placed 02-list_companies.json" | tee -a evidence/run-g2-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g2/answers; tail -n 3 run-g2/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-g2/answers/02-list_companies.json > evidence/run-g2-show-02-list_companies.json.txt 2>&1; echo "exit $?" >> evidence/run-g2-show-02-list_companies.json.txt; cat evidence/run-g2-show-02-list_companies.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/03-marks.json run-g2/inbox/03-marks.json.part && grep '  calls/03-marks.json$' SHA256.txt | awk '{print $1"  run-g2/inbox/03-marks.json.part"}' | sha256sum -c - && mv run-g2/inbox/03-marks.json.part run-g2/inbox/03-marks.json && echo "placed 03-marks.json" | tee -a evidence/run-g2-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g2/answers; tail -n 3 run-g2/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-g2/answers/03-marks.json > evidence/run-g2-show-03-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-g2-show-03-marks.json.txt; cat evidence/run-g2-show-03-marks.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/05-build-0074-0801.json run-g2/inbox/05-build-0074-0801.json.part && grep '  calls/05-build-0074-0801.json$' SHA256.txt | awk '{print $1"  run-g2/inbox/05-build-0074-0801.json.part"}' | sha256sum -c - && mv run-g2/inbox/05-build-0074-0801.json.part run-g2/inbox/05-build-0074-0801.json && echo "placed 05-build-0074-0801.json" | tee -a evidence/run-g2-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g2/answers; tail -n 3 run-g2/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 grade05.py grade run-g2/answers/05-build-0074-0801.json > evidence/run-g2-grade05.txt 2>&1; echo "exit $?" >> evidence/run-g2-grade05.txt; cat evidence/run-g2-grade05.txt
     cd /c/bridge-lab/2162-rc && cp calls/07-marks.json run-g2/inbox/07-marks.json.part && grep '  calls/07-marks.json$' SHA256.txt | awk '{print $1"  run-g2/inbox/07-marks.json.part"}' | sha256sum -c - && mv run-g2/inbox/07-marks.json.part run-g2/inbox/07-marks.json && echo "placed 07-marks.json" | tee -a evidence/run-g2-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-g2/answers; tail -n 3 run-g2/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-g2/answers/07-marks.json > evidence/run-g2-show-07-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-g2-show-07-marks.json.txt; cat evidence/run-g2-show-07-marks.json.txt
     ```
     EXPECT `01` `tools: 37; missing of this row's: []` (lab 2162-RX for BUILD 8c3fb9f2; for BUILD 6f541ae4 the run coordinator's line says 37: record the count, and grade only `missing ... []`); `02` one company with the pinned GUID; `03` and `07` M 245, V 68, reused False (a moved mark is a STOP). `05` is graded by `grade05.py`, not by `calls.py show` (row 2162-RX's `show` of a refused build printed no refusal: its refusals are under `result.refusals`, which `calls.py` does not read). EXPECT `VERDICT admitted voucher_count 1, no refusal`. Exit 2 from a third line, typed after the answer arrived, is a failed call: STOP. An admitted answer's own `next_step` text names `post_import`: it is not followed; no post call is ever placed. Do NOT EXPECT the earlier rows' evidence sizes or digests for BUILD 6f541ae4: record them.
   - **Launch LG2, part 6: the batch file.** EXPECT `equal to the answer's sha256`; `FINDING` (exit 1) is recorded and is a STOP. The second line keeps the data folder's listing whole. If part 5's `VERDICT` was not `admitted`, type only the second line and record whether it lists a file ending `.xml`.
     ```bash
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py batch run-g2/answers/05-build-0074-0801.json data-g2 > evidence/run-g2-batch.txt 2>&1; echo "exit $?" >> evidence/run-g2-batch.txt; cat evidence/run-g2-batch.txt
     cd /c/bridge-lab/2162-rc && find data-g2 -type f > evidence/run-g2-data-files.txt 2>&1; echo "exit $?" >> evidence/run-g2-data-files.txt; cat evidence/run-g2-data-files.txt
     ```
   - **Launch LG2, part 7: close.** Type the first line. Then type the second line until `evidence/run-g2-launch.txt` ends with a `client exit` line (the client ends within about 30 s of the stop file). Then type the third. EXPECT the log's last line `session ended; server pid <pid> exit code 0`, `client exit 0` as the launch file's last line, the server error log's size, and then `INFO: No tasks are running`. Never end a process. The log line `server did not exit within 30 s of stdin close; killed` (`client.py:111-113`: the helper ends its own server) is a STOP and report. No `client exit` line in 60 s, any other exit, or a server that does not end: STOP and report.
     ```bash
     cd /c/bridge-lab/2162-rc && touch run-g2/inbox/stop && echo "stop file placed" | tee -a evidence/run-g2-launch.txt
     cd /c/bridge-lab/2162-rc && tail -n 1 run-g2/client-log.txt; cat evidence/run-g2-launch.txt; stat -c '%n %s' run-g2/server-stderr-*.log
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-g2-procs-after.txt 2>&1; echo "exit $?" >> evidence/run-g2-procs-after.txt; cat evidence/run-g2-procs-after.txt
     ```
   - Send `G2-post`: `M-marks.xml rates.xml`. EXPECT marks equal to `start`, the answer grown, and no structural difference from `G`. Not grown: record it; G2's verdict stands as recorded, with that note (the state at its launch is bracketed by `G1-post` and `G2-post`).
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 G2-post M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 G2-post rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-G2-post-M-marks.xml > evidence/G2-post-count19.txt 2>&1; echo "exit $?" >> evidence/G2-post-count19.txt; cat evidence/G2-post-count19.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-G2-post-rates.xml > evidence/G2-post-is-grown.txt 2>&1; echo "d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257  a-G2-post-rates.xml" | sha256sum -c - >> evidence/G2-post-is-grown.txt 2>&1; echo "exit $?" >> evidence/G2-post-is-grown.txt; cat evidence/G2-post-is-grown.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-G-rates.xml a-G2-post-rates.xml > evidence/G-vs-G2-post.txt 2>&1; echo "exit $?" >> evidence/G-vs-G2-post.txt; cat evidence/G-vs-G2-post.txt
     ```
   - If G2 admitted: compare its batch file with P's. EXPECT, as the measurement (not shown), `EQUAL once the identity is set aside` (exit 0) or `DIFFER` (exit 1) with the differing lines. Either is a result. Exit 2 is a failed read: record it and go on.
     ```bash
     cd /c/bridge-lab/2162-rc && python -I -X utf8 grade05.py importeq run-p/answers/05-build-0074-0801.json data-p run-g2/answers/05-build-0074-0801.json data-g2 > evidence/importeq-P-G2.txt 2>&1; echo "exit $?" >> evidence/importeq-P-G2.txt; cat evidence/importeq-P-G2.txt
     ```
   - G2 refused: record `grade05.py`'s whole output, every refusal code and detail. Nothing more is launched in this state. Go to step 6: C runs after a reload, in another state.
   - Any other verdict than admitted or the refusal handled above (`VERDICT other`, or the grading line exiting non-zero): record it whole, close the launch, take `G2-post`, then STOP and report.
6. **C: Shut Company, a load, BUILD 6f541ae4.** Frame a and b. Then launch LC. Then frame d (`C`).
   - Frame b's commands. EXPECT `C-pre2` `: OK` against the plain sha256 and no structural difference from `A0-second` (not cleared is a STOP, as frame b says). `C-pre` against `C-pre2` is recorded: a difference is a first-read-after-load effect, not a STOP (frame b).
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 C-pre M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 C-pre rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 C-pre2 rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-C-pre-M-marks.xml > evidence/C-pre-count19.txt 2>&1; echo "exit $?" >> evidence/C-pre-count19.txt; cat evidence/C-pre-count19.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-C-pre-rates.xml a-C-pre2-rates.xml > evidence/C-pre-vs-C-pre2.txt 2>&1; echo "exit $?" >> evidence/C-pre-vs-C-pre2.txt; cat evidence/C-pre-vs-C-pre2.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-C-pre2-rates.xml > evidence/A0-second-vs-C-pre2.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-C-pre2.txt; cat evidence/A0-second-vs-C-pre2.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-C-pre2-rates.xml > evidence/C-pre2-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-C-pre2-rates.xml" | sha256sum -c - >> evidence/C-pre2-is-plain.txt 2>&1; echo "exit $?" >> evidence/C-pre2-is-plain.txt; cat evidence/C-pre2-is-plain.txt
     ```
   - Launch LC: its seven parts follow. EXPECT `VERDICT admitted voucher_count 1, no refusal` and the batch file equal to the answer's sha256; record `batch`'s whole output. Any other verdict: record it whole, close, finish the read below, then STOP: the change broke the plain case.
   - **Launch LC, part 1: folders.** EXPECT `made`; `STOP` is a STOP (never reuse a launch folder).
     ```bash
     cd /c/bridge-lab/2162-rc && if [ -e run-c ] || [ -e data-c ]; then echo "STOP: run-c or data-c exists"; else mkdir run-c run-c/inbox run-c/answers data-c && echo "made run-c run-c/inbox run-c/answers data-c"; fi
     ```
   - **Launch LC, part 2: the exe and the launch file.** EXPECT two lines ending `: OK`. `FAILED` or no output: STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && echo "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc  C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" | sha256sum -c - > evidence/run-c-hashes.txt 2>&1; echo "exit $?" >> evidence/run-c-hashes.txt; cat evidence/run-c-hashes.txt
     cd /c/bridge-lab/2162-rc && cp launch/LC-launch-env.json run-c/launch-env.json && grep '  launch/LC-launch-env.json$' SHA256.txt | awk '{print $1"  run-c/launch-env.json"}' | sha256sum -c - >> evidence/run-c-hashes.txt 2>&1; echo "exit $?" >> evidence/run-c-hashes.txt; cat evidence/run-c-hashes.txt
     ```
   - **Launch LC, part 3: other Bridge processes.** EXPECT `INFO: No tasks are running` and `exit 0`; a `bridge_mcp.exe` line: STOP and report (none may call port 9001 during this sitting).
     ```bash
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-c-procs-before.txt 2>&1; echo "exit $?" >> evidence/run-c-procs-before.txt; cat evidence/run-c-procs-before.txt
     ```
   - **Launch LC, part 4: start the client.** Start the first line, whole, with the operator's shell tool's own background run, as rows 2440-CP, 2569-R and 2569-S did: no window, standard input from the null device. Type no `&`, change nothing in the line, and record in the notes how it was started. It prints nothing and runs until part 7's stop file. Then, within 60 s, type the second line. EXPECT the exe hash `ok`, the launch file's hash equal to `SHA256.txt`'s, `BRIDGE_AGENT_ENABLE_WRITES present: True`, and `initialized` with version 0.7.0. A `stop:` line, or no `initialized` line in 60 s: STOP. No screenshot from here to the end of part 7 unless a STOP needs one.
     ```bash
     ( cd /c/bridge-lab/2162-rc && python -I -X utf8 client.py "C:/bridge-lab/2162-rc/run-c" "C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe" "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc" < /dev/null > evidence/run-c-client-out.txt 2> evidence/run-c-client-err.txt; echo "client exit $?" >> evidence/run-c-launch.txt )
     cd /c/bridge-lab/2162-rc && cat run-c/client-log.txt
     ```
   - **Launch LC, part 5: the five calls, one at a time, in this order.** Each call has three lines. The first copies the call in under a name ending `.part`, checks its hash against `SHA256.txt`, and only then renames it (the client opens a name ending `.json` at once; lab 2162-SALES-Q, deviation 4); EXPECT `: OK` and `placed`. The second shows whether its answer has arrived: type it until `run-c/answers/` lists the call's name and the log's last lines show the call's line; five minutes without them is a STOP. Only then type the third, which grades the answer; wait for its EXPECT before the next call. Exit 2 from the third line, typed after the answer arrived, is a failed call: STOP. No other call file is put in; none is written, edited or brought from another sheet, and in particular no post or verify call.
     ```bash
     cd /c/bridge-lab/2162-rc && cp calls/01-list.json run-c/inbox/01-list.json.part && grep '  calls/01-list.json$' SHA256.txt | awk '{print $1"  run-c/inbox/01-list.json.part"}' | sha256sum -c - && mv run-c/inbox/01-list.json.part run-c/inbox/01-list.json && echo "placed 01-list.json" | tee -a evidence/run-c-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-c/answers; tail -n 3 run-c/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-c/answers/01-list.json > evidence/run-c-show-01-list.json.txt 2>&1; echo "exit $?" >> evidence/run-c-show-01-list.json.txt; cat evidence/run-c-show-01-list.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/02-list_companies.json run-c/inbox/02-list_companies.json.part && grep '  calls/02-list_companies.json$' SHA256.txt | awk '{print $1"  run-c/inbox/02-list_companies.json.part"}' | sha256sum -c - && mv run-c/inbox/02-list_companies.json.part run-c/inbox/02-list_companies.json && echo "placed 02-list_companies.json" | tee -a evidence/run-c-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-c/answers; tail -n 3 run-c/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-c/answers/02-list_companies.json > evidence/run-c-show-02-list_companies.json.txt 2>&1; echo "exit $?" >> evidence/run-c-show-02-list_companies.json.txt; cat evidence/run-c-show-02-list_companies.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/03-marks.json run-c/inbox/03-marks.json.part && grep '  calls/03-marks.json$' SHA256.txt | awk '{print $1"  run-c/inbox/03-marks.json.part"}' | sha256sum -c - && mv run-c/inbox/03-marks.json.part run-c/inbox/03-marks.json && echo "placed 03-marks.json" | tee -a evidence/run-c-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-c/answers; tail -n 3 run-c/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-c/answers/03-marks.json > evidence/run-c-show-03-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-c-show-03-marks.json.txt; cat evidence/run-c-show-03-marks.json.txt
     cd /c/bridge-lab/2162-rc && cp calls/05-build-0074-0801.json run-c/inbox/05-build-0074-0801.json.part && grep '  calls/05-build-0074-0801.json$' SHA256.txt | awk '{print $1"  run-c/inbox/05-build-0074-0801.json.part"}' | sha256sum -c - && mv run-c/inbox/05-build-0074-0801.json.part run-c/inbox/05-build-0074-0801.json && echo "placed 05-build-0074-0801.json" | tee -a evidence/run-c-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-c/answers; tail -n 3 run-c/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 grade05.py grade run-c/answers/05-build-0074-0801.json > evidence/run-c-grade05.txt 2>&1; echo "exit $?" >> evidence/run-c-grade05.txt; cat evidence/run-c-grade05.txt
     cd /c/bridge-lab/2162-rc && cp calls/07-marks.json run-c/inbox/07-marks.json.part && grep '  calls/07-marks.json$' SHA256.txt | awk '{print $1"  run-c/inbox/07-marks.json.part"}' | sha256sum -c - && mv run-c/inbox/07-marks.json.part run-c/inbox/07-marks.json && echo "placed 07-marks.json" | tee -a evidence/run-c-launch.txt
     cd /c/bridge-lab/2162-rc && ls run-c/answers; tail -n 3 run-c/client-log.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py show run-c/answers/07-marks.json > evidence/run-c-show-07-marks.json.txt 2>&1; echo "exit $?" >> evidence/run-c-show-07-marks.json.txt; cat evidence/run-c-show-07-marks.json.txt
     ```
     EXPECT `01` `tools: 37; missing of this row's: []` (lab 2162-RX for BUILD 8c3fb9f2; for BUILD 6f541ae4 the run coordinator's line says 37: record the count, and grade only `missing ... []`); `02` one company with the pinned GUID; `03` and `07` M 245, V 68, reused False (a moved mark is a STOP). `05` is graded by `grade05.py`, not by `calls.py show` (row 2162-RX's `show` of a refused build printed no refusal: its refusals are under `result.refusals`, which `calls.py` does not read). EXPECT `VERDICT admitted voucher_count 1, no refusal`. Exit 2 from a third line, typed after the answer arrived, is a failed call: STOP. An admitted answer's own `next_step` text names `post_import`: it is not followed; no post call is ever placed. Do NOT EXPECT the earlier rows' evidence sizes or digests for BUILD 6f541ae4: record them.
   - **Launch LC, part 6: the batch file.** EXPECT `equal to the answer's sha256`; `FINDING` (exit 1) is recorded and is a STOP. The second line keeps the data folder's listing whole. If part 5's `VERDICT` was not `admitted`, type only the second line and record whether it lists a file ending `.xml`.
     ```bash
     cd /c/bridge-lab/2162-rc && python -I -X utf8 calls.py batch run-c/answers/05-build-0074-0801.json data-c > evidence/run-c-batch.txt 2>&1; echo "exit $?" >> evidence/run-c-batch.txt; cat evidence/run-c-batch.txt
     cd /c/bridge-lab/2162-rc && find data-c -type f > evidence/run-c-data-files.txt 2>&1; echo "exit $?" >> evidence/run-c-data-files.txt; cat evidence/run-c-data-files.txt
     ```
   - **Launch LC, part 7: close.** Type the first line. Then type the second line until `evidence/run-c-launch.txt` ends with a `client exit` line (the client ends within about 30 s of the stop file). Then type the third. EXPECT the log's last line `session ended; server pid <pid> exit code 0`, `client exit 0` as the launch file's last line, the server error log's size, and then `INFO: No tasks are running`. Never end a process. The log line `server did not exit within 30 s of stdin close; killed` (`client.py:111-113`: the helper ends its own server) is a STOP and report. No `client exit` line in 60 s, any other exit, or a server that does not end: STOP and report.
     ```bash
     cd /c/bridge-lab/2162-rc && touch run-c/inbox/stop && echo "stop file placed" | tee -a evidence/run-c-launch.txt
     cd /c/bridge-lab/2162-rc && tail -n 1 run-c/client-log.txt; cat evidence/run-c-launch.txt; stat -c '%n %s' run-c/server-stderr-*.log
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/run-c-procs-after.txt 2>&1; echo "exit $?" >> evidence/run-c-procs-after.txt; cat evidence/run-c-procs-after.txt
     ```
   - Frame d (`C`). EXPECT marks equal to `start`, the answer plain, and no structural difference from `A0-second`. Otherwise STOP.
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 C M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && sha256sum -c local/rates.sha256 && python -I run.py http://127.0.0.1:9001 C rates.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-C-M-marks.xml > evidence/C-count19.txt 2>&1; echo "exit $?" >> evidence/C-count19.txt; cat evidence/C-count19.txt
     cd /c/bridge-lab/2162-rc && stat -c '%n %s' a-C-rates.xml > evidence/C-is-plain.txt 2>&1; echo "a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62  a-C-rates.xml" | sha256sum -c - >> evidence/C-is-plain.txt 2>&1; echo "exit $?" >> evidence/C-is-plain.txt; cat evidence/C-is-plain.txt
     cd /c/bridge-lab/2162-rc && python -I -X utf8 ratecmp.py a-A0-second-rates.xml a-C-rates.xml > evidence/A0-second-vs-C.txt 2>&1; echo "exit $?" >> evidence/A0-second-vs-C.txt; cat evidence/A0-second-vs-C.txt
     ```
7. **Close.**
   - Write the arms not run and why. No restore was made.
   - With the company still loaded, send `close`: `M-marks.xml`. EXPECT pass, V 68, M 245. Nothing was keyed, so when this passes, **no restore is needed**. If a mark moved anywhere, STOP and report; any restore is only on the coordinator's word.
     ```bash
     cd /c/bridge-lab/2162-rc && grep '  M-marks.xml$' SHA256.txt | awk '{print $1"  "$3}' | sha256sum -c - && python -I run.py http://127.0.0.1:9001 close M-marks.xml
     # screenshot of the whole Tally window now (C3), before the next line
     cd /c/bridge-lab/2162-rc && python -I -X utf8 count19.py --same-row a-start-M-marks.xml a-close-M-marks.xml > evidence/close-count19.txt 2>&1; echo "exit $?" >> evidence/close-count19.txt; cat evidence/close-count19.txt
     ```
   - Close by C10: Tally's own Shut Company by the keys above, then C2 a and b again (one `tally.exe` owning 9001), GET /status 200, a screenshot with no company loaded and the current date unchanged. Send no XML after the shut. Record the menu path and key used.
     ```bash
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > evidence/c10-tasklist.txt 2>&1; echo "exit $?" >> evidence/c10-tasklist.txt; cat evidence/c10-tasklist.txt
     cd /c/bridge-lab/2162-rc && netstat -ano > evidence/c10-netstat.txt 2>&1; echo "exit $?" >> evidence/c10-netstat.txt; grep -E ':9001 |^exit' evidence/c10-netstat.txt
     cd /c/bridge-lab/2162-rc && curl.exe -sS -w "%{http_code} %{size_download}\n" http://127.0.0.1:9001/status -o evidence/c10-status-body.txt > evidence/c10-status.txt 2>&1; echo "curl exit $?" >> evidence/c10-status.txt; cat evidence/c10-status.txt
     cd /c/bridge-lab/2162-rc && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > evidence/c10-bridge.txt 2>&1; echo "exit $?" >> evidence/c10-bridge.txt; cat evidence/c10-bridge.txt
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
