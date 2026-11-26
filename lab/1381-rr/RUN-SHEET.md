# Row 1381-RR: does Tally's own Restore work in Educational Mode on this lab?

For #1381. Row 1381-RB (a master changed between the bind and the aim) needs a write and then a restore. No restore has been made in Educational Mode on this lab: the lab's Tally has run in Educational Mode since the Silver Rental licence lapsed on 24-Nov-2026 (lab 2162-PA), and the last restore, row 1108-R3's of BK-S, was made licensed on 19 Nov 2026. This row asks one question first. It changes one variable, the restore: it reads the book's marks (M1), shuts the book, restores BK-S into it by Tally's own Restore, loads it again and reads the marks, stock items, stock groups and units (M2, I, G, U), and compares them with M1 and with row 1381-RA's answers. No write, no backup, no Bridge build.

**How an EXPECT is marked.** **(lab `row`)** is an earlier observation; **(not shown)** is what this sitting measures. Every computed line is a measurement, never a pass or fail of anything. Any outcome other than an EXPECT is the finding, recorded, not a failure of the sheet.

**Rules.** `LAB-COMMON-RR.md` (R1 to R13) applies in full, with LAB-COMMON v3 C6 (blob c4badf8c) for the one restore (R5): the one synthetic book, Educational Mode as the state measured (a title band other than EDU is STOP-EDITION), five reads by `send.py` only, no write, no backup, the licence pair at a Shut Company only, any licence screen during the restore STOP-LICENCE, nothing ends a process, everything the lab adds under `local/`, every STOP named and left as it stands. Each R clause names the RA or v3 clause it keeps or changes.

## The questions

- **Q1.** In Educational Mode, does Tally's own Restore of BK-S into the book complete, with "Accept ?" and the Replace prompt as its only prompts and no licence, activation, renewal or Educational-Mode screen?
- **Q2.** After it, does the book read back as before: M2's state the one M1 met (one company row, the book's GUID, ALTMSTID 210, no ALTVCHID), and M2 byte-equal to M1?
- **Q3.** Are I, G and U byte-equal to row 1381-RA's I1, G1 and U1 (the same requests, the same book, Educational Mode, before any restore)?
- **Q4.** What does the company's data folder listing show changed between before and after the restore, and what do the title band and the Gateway show after it?

**EXPECT** (not shown; lab 1108-R3, licensed, step 9, for the prompts): the restore completes with the two prompts only; M2 = M1 byte for byte; I, G and U byte-equal to RA's; the listing shows at least one entry changed or added. M2 = M1 alone cannot show that the restore did anything, since the book is already at BK-S's state (Q2 is "the book reads back as before", not "the restore took effect"); the listing difference is this row's only evidence that Replace rewrote the folder.

## The backup

- **BK-S** (lab 1108-R3, `lab/1108-r3-equal-voucher-answers` at a64e8036, its `RESULTS.md` and `lab/1108-r3-equal-voucher-answers/evidence/notes.txt`, steps 2 and 9): taken on 19 Nov 2026 at 14:10:11Z, licensed, with the book at ALTMSTID 210 and no voucher; restored once, at that row's step 9, with its closing marks, ledger and voucher reads byte-equal to its opening ones. `C:\bridge-lab\1108r3\backups\bks\TDBK1800_<company number>.001`, 24,219 bytes, sha256 `ecc8d4028b88b763e5a8a22014bc699b07db68c3eb32cff3c6c92052b373b0f4` (as row 1381-RA's sheet, The book, records it). At that restore the Restore list showed the book with its number and `19-Nov-26 (7:40 pm)`; the prompts were "Accept ?" (answered `y`) and the folder-exists prompt (answered `r`, Replace), and Tally came back at Select Company.
- **The fallback**, never restored inside this row: row 1381-O's backup-1, `C:\lab\1381-o\backup-1\TDBK1800_<company number>.001`, 24,219 bytes, sha256 `d23f881a6bac8e1f9bf53035e7f21a72234f9f57c3672a965bc23a3dd76d44eb` (lab 1381-O, S2 and S6). Restored only on the run coordinator's word, by v3 C6, as a separate act (R5, R10).
- `restore.py` holds BK-S's folder, bytes and sha256 (`check_sheet.py` checks they are this section's), takes the company number from the backup's own file name, and takes Tally's data folder from its command line (`<DATA>` below), so that neither is in this folder.

## The book

`BRIDGE STOCK CREATE LAB`, synthetic; its GUID `bac2b5e9…` is listed in master's `scripts/merge-gate-synthetic-uuids.json`; its provenance is `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/PROVENANCE.md`. **Its state** (lab 1381-RA, `lab/1381-ra-answers` at 851e2545, its `RESULTS.md`, 25 Nov 2026 20:38Z to 20:43Z, Educational Mode, the latest row to load it): one stock item `SC Sibling Goods`, one group `SC Goods Group`, one unit `Nos`; ALTMSTID 210, no ALTVCHID; Gateway period 1-Apr-25 to 31-Mar-26, current date Tuesday 1-Apr-2025; no write, no backup and no restore. So the EXPECT for M1 is that state, BK-S's (lab 1108-R3: ALTMSTID 210, no voucher), and `measure.py state M1` checks it (R1) before the shut.

## The reads

Four of row 1381-RA's five requests, byte-identical copies (git blob ids equal to `lab/1381-ra/requests/` at 30f5c820); RA's sheet, The reads and What each read sends, gives each one's builder, code lines and shape. Each is one Export of a Collection with no date window and no import (`check_sheet.py` checks), sent as a byte-order mark then UTF-16LE with `Content-Type: text/xml; charset=utf-16`. RA's count request is not carried: its answer holds the company number, and this row does not need it.

| labels | request | bytes | sha256 | git blob |
|---|---|---|---|---|
| M1, M2 | `requests/marks.utf16le.xml` | 990 | `5802b3feca930e569de996e5582299ad34f2515140b233a7140290f8f874552b` | ae363536 |
| I | `requests/items.utf16le.xml` | 1330 | `581b1d95df1b7b80c846ce13e7df18cc47defddb1c646eb025d48bfa5afe0ac1` | dfa57543 |
| G | `requests/groups.utf16le.xml` | 1144 | `d79503503a31c4fee4195b2e03308c2833aa2e9c3072bd4cc0ed1f73d9121245` | 81407606 |
| U | `requests/units.utf16le.xml` | 1174 | `3f79503b5480a6e3a23ec6fb8fc8fd5ca8200d4f0889590c994d1240ed299d0c` | ea630407 |

## The answers compared with

Row 1381-RA's pushed answers, `lab/1381-ra-answers` at 851e254586024e757f0bbe538d3ea7d49ed1f754, `lab/1381-ra-answers/answers/`; each sha256 computed with git from that commit's blob, and equal to that branch's `SHA256.txt` and `RESULTS.md`. `measure.py` pins them and reads a reference only when its sha256 is the pin (else `NOT READ`).

| this row | RA's answer | bytes | sha256 | git blob |
|---|---|---|---|---|
| M1 (self-test only) | `a-M1.xml` | 3318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` | 9944270a |
| I | `a-I1.xml` | 8270 | `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4` | 8e6d9df8 |
| G | `a-G1.xml` | 4170 | `b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a` | f0e86592 |
| U | `a-U1.xml` | 4040 | `cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39` | ce98c10b |

M2 is compared with this sitting's M1, not with RA's M1 (which this sitting's M1 is expected to equal: RA's M1 and M2 were byte-equal to each other and to lab 1381-O's marks capture).

## Order and the one variable

M1 first: the state check, before anything is done to the book. Then the shut, the plan, the restore, the after-record and the load, with no request sent between M1 and M2. Then M2, MARKS first after a restore (v3 C6), and its state checked before I, G and U. Between M1 and M2 the one change is the restore (and the shut and load it needs); between RA's I1, G1, U1 and this row's I, G, U the changes are the restore and the sitting.

## Shell

Every command is typed in **Git Bash**, from `/c/bridge-lab/1381-rr`, exactly as written: each line begins `cd /c/bridge-lab/1381-rr && ` and depends on no shell variable set by an earlier line. Step 0's first line does not: it makes the folder, and when the folder already exists it prints its STOP and writes nothing. The interpreter is `python` (3.10) with `-I -X utf8`; `python3` is the Store alias and is never used. Every command's output goes to a file under `local/evidence/` with its exit code, then is printed whole. A line that begins `#` is an instruction, not typed. `<CLONE>` is the Git Bash path of the lab's Bridge clone; `<SHEET>` is the 40-character commit of this branch the run coordinator's GO names; `<DATA>` is Tally's data folder with forward slashes, the folder row 1108-R3 used as its Restore Data Path (that row's local copy of its `c6prep.py` holds it), and the folder the Restore screen must show as its Restore Data Path. Screenshots, the loads, the shuts and the restore are the operator's screen-tool acts on Tally's existing window (never "open application"); each screenshot is saved under `local/shots/` with its name and recorded in the notes.

## Files

- Byte-equal to row 1381-RA's (`lab/1381-ra/` at 30f5c820): `.gitattributes`, `.gitignore` (`/local/`), and the four request files.
- Changed copies of RA's, each change named in the file's own header: `send.py` (five labels M1 M2 I G U; the gates M2 after M1's state and the restore's after-record, I after M2's state; `--selftest` runs 28 synthetic cases); `measure.py` (RA's reader and state check; the count read cut; `state M1|M2`; `all` against RA's pinned answers; `--selftest` runs 30 cases on RA's answers, each negative case an answer with one edit); `check_sheet.py` (this row's ten command lines in order, the four requests, the held lines of `send.py` and `restore.py`; `--fresh` and STOP-PYCACHE as RA's).
- Written for this row: `restore.py` (v3 C6's records before and after the restore, in the shape of row 1108-R3's `c6prep.py`; `--selftest` runs 16 cases on synthetic folders); `LAB-COMMON-RR.md`; `SHA256.txt` (every other file's sha256, bytes and path); `RUN-SHEET.md`, this sheet.
- Not carried from RA: `gen_ra.py` and RA's count request (this row makes no request; its four are RA's, pinned by `SHA256.txt` and the table above).

## Self-checks before the sitting

Run from `/c/bridge-lab/1381-rr` at step 0, after RA's answers are in `local/gen/ra/`. Nothing is sent to Tally.

```bash
cd /c/bridge-lab/1381-rr && python -I -X utf8 send.py --selftest local/selftest/send > local/evidence/selfcheck-send.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-send.txt; tail -n 2 local/evidence/selfcheck-send.txt
cd /c/bridge-lab/1381-rr && python -I -X utf8 measure.py --selftest local/gen/ra local/selftest/measure > local/evidence/selfcheck-measure.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-measure.txt; tail -n 2 local/evidence/selfcheck-measure.txt
cd /c/bridge-lab/1381-rr && python -I -X utf8 restore.py --selftest local/selftest/restore > local/evidence/selfcheck-restore.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-restore.txt; tail -n 2 local/evidence/selfcheck-restore.txt
cd /c/bridge-lab/1381-rr && python -I -X utf8 check_sheet.py > local/evidence/selfcheck-sheet.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-sheet.txt; cat local/evidence/selfcheck-sheet.txt
```

EXPECT, in order: `28 cases, 28 right`, exit 0; `30 cases, 30 right`, exit 0; `16 cases, 16 right`, exit 0; three `OK` lines, exit 0. Any other: STOP-SELFCHECK.

## Steps

**Prompt rule.** No dialog is expected but R7's licence pair after a Shut Company and R5's two restore prompts. Any other dialog or message on Tally's screen: copy its text, press no key, send nothing, STOP-DIALOG (a licence screen anywhere else: STOP-LICENCE). Never end a process (R8).

**Record-line rule.** A send, state or restore line that prints `STOP: local/evidence/<name> exists` is STOP-RERUN: that line was typed before, nothing was done now, and the earlier record stands. A send line that has not returned after 10 minutes is STOP-NO-ANSWER (R4): leave that Git Bash window and its curl exactly as they are and report.

0. **Before the sitting.** Nothing is sent to Tally in this step except the GET `/status` of its last line.
   - Make the row folder, fetch, and copy this sheet's folder into it byte for byte from the lab's Bridge clone. EXPECT `made` and `exit 0`; the fetch `exit 0`; the copy `exit 0 0`. `STOP: … exists`, or no `made` line because `/c/bridge-lab` is absent: STOP-FOLDER. Any other exit: STOP-COPY. The fetch is this row's one network act other than Tally's port; nothing in it goes to Tally.
     ```bash
     if [ -e /c/bridge-lab/1381-rr ]; then echo "STOP: /c/bridge-lab/1381-rr exists"; else mkdir /c/bridge-lab/1381-rr /c/bridge-lab/1381-rr/local /c/bridge-lab/1381-rr/local/evidence /c/bridge-lab/1381-rr/local/gen /c/bridge-lab/1381-rr/local/shots && echo "made" > /c/bridge-lab/1381-rr/local/evidence/s0-made.txt; echo "exit $?" >> /c/bridge-lab/1381-rr/local/evidence/s0-made.txt; cat /c/bridge-lab/1381-rr/local/evidence/s0-made.txt; fi
     cd /c/bridge-lab/1381-rr && git -C <CLONE> fetch origin > local/evidence/s0-fetch-origin.txt 2>&1; echo "exit $?" >> local/evidence/s0-fetch-origin.txt; cat local/evidence/s0-fetch-origin.txt
     cd /c/bridge-lab/1381-rr && { git -C <CLONE> archive <SHEET> lab/1381-rr | tar -x -C /c/bridge-lab/1381-rr --strip-components=2; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"; } > local/evidence/s0-copy.txt 2>&1; cat local/evidence/s0-copy.txt
     ```
   - **The pre-sitting check**, right after the copy. EXPECT `OK: local/ holds only step 0's own records …` then the three `OK` lines, exit 0. `STOP-LOCAL-PREFILLED`: stop, delete nothing, report the names it prints. Any other miss: STOP-SELFCHECK.
     ```bash
     cd /c/bridge-lab/1381-rr && python -I -X utf8 check_sheet.py --fresh > local/evidence/s0-fresh.txt 2>&1; echo "exit $?" >> local/evidence/s0-fresh.txt; cat local/evidence/s0-fresh.txt
     ```
   - RA's answers commit in the lab's clone, and the four answers the self-checks and the compute read, from it. EXPECT `present 851e2545…`; then `exit 0 0`. `ABSENT` or any other exit: STOP-CODE-COMMIT.
     ```bash
     cd /c/bridge-lab/1381-rr && if git -C <CLONE> cat-file -e "851e254586024e757f0bbe538d3ea7d49ed1f754^{commit}" 2>/dev/null; then echo "present 851e254586024e757f0bbe538d3ea7d49ed1f754"; else echo "ABSENT"; fi > local/evidence/s0-commit.txt 2>&1; cat local/evidence/s0-commit.txt
     cd /c/bridge-lab/1381-rr && mkdir local/gen/ra && { git -C <CLONE> archive 851e254586024e757f0bbe538d3ea7d49ed1f754 lab/1381-ra-answers/answers/a-M1.xml lab/1381-ra-answers/answers/a-I1.xml lab/1381-ra-answers/answers/a-G1.xml lab/1381-ra-answers/answers/a-U1.xml | tar -x -C local/gen/ra --strip-components=3; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"; } > local/evidence/s0-ra.txt 2>&1; cat local/evidence/s0-ra.txt
     ```
   - Run all four Self-checks. Any miss: STOP-SELFCHECK.
   - Processes and the port, read only, as RA's step 0. EXPECT for `bridge_mcp.exe` the one line `INFO: No tasks are running which match the specified criteria.`, exit 0 (else STOP-STRAY); for `tally.exe` exactly one line, its PID recorded (else STOP-TALLY); at least one `LISTENING` line ending `:9001` with that PID, and a `LISTENING` line for another port (else STOP-TALLY). The `tally.exe` and port files stay on the lab PC (R13).
     ```bash
     cd /c/bridge-lab/1381-rr && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > local/evidence/s0-bridge.txt 2>&1; echo "exit $?" >> local/evidence/s0-bridge.txt; cat local/evidence/s0-bridge.txt
     cd /c/bridge-lab/1381-rr && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > local/s0-tally.txt 2>&1; echo "exit $?" >> local/s0-tally.txt; cat local/s0-tally.txt
     cd /c/bridge-lab/1381-rr && netstat -ano > local/s0-netstat.txt 2>&1; echo "exit $?" >> local/s0-netstat.txt; grep -c LISTENING local/s0-netstat.txt; grep -E ':9001 .*LISTENING' local/s0-netstat.txt
     ```
   - **The screen.** A screenshot of the whole Tally window, `local/shots/s0-window.png`. EXPECT Select Company with no company loaded, the title band `TallyPrime EDU`, no dialog. A company loaded: STOP-OTHER-COMPANY, shut nothing. Another title: STOP-EDITION. A dialog or half-entered screen: STOP-DIALOG.
   - GET `/status`, the last act of step 0. EXPECT `exit 0` and the `-w` line `200 51` (lab 1381-RA: `200 51`); another byte count is recorded, not a STOP. Any other status or exit: STOP-TALLY.
     ```bash
     cd /c/bridge-lab/1381-rr && curl.exe -sS -w "%{http_code} %{size_download}\n" -D local/evidence/s0-status.headers.txt http://127.0.0.1:9001/status -o local/evidence/s0-status.txt > local/evidence/s0-status-curl.txt 2>&1; echo "exit $?" >> local/evidence/s0-status-curl.txt; cat local/evidence/s0-status-curl.txt
     ```
1. **The load** (R1), by RA's keys (lab 1381-RA): a click in the Select Company filter box; `BRIDGE`, then a space and `STOCK`, then a space and `CREATE` until one row is left, `BRIDGE STOCK CREATE LAB`; screenshot `local/shots/s1-select.png`; Enter. The book absent from the list: STOP-BOOK-ABSENT, press nothing. Then a screenshot of the whole window, `local/shots/s1-gateway.png`: EXPECT the Gateway with `BRIDGE STOCK CREATE LAB` as the one company, the title band EDU, no dialog. Record the period and the current date the Gateway shows (EXPECT 1-Apr-25 to 31-Mar-26 and Tuesday 1-Apr-2025, lab 1381-RA; another date is recorded, not a STOP). A second company: STOP-OTHER-COMPANY. A dialog: STOP-DIALOG (a licence screen: STOP-LICENCE).
2. **M1 and the state.** No key, click or request in Tally from here to the shut. EXPECT M1's file to end `VERDICT SENT-OK` and `exit 0`; then the state file `BOOK STATE MET` and `exit 0`. `REFUSED`, exit 2: nothing was sent (STOP-SEND-REFUSED). Exit 1: STOP-REQUEST-FAILED. `STOP-BOOK-STATE` (exit 1) or `NOT READ` (exit 2): STOP-BOOK-STATE, nothing is restored (R1).
   ```bash
   cd /c/bridge-lab/1381-rr && if [ -e local/evidence/send-M1.txt ]; then echo "STOP: local/evidence/send-M1.txt exists"; else python -I -X utf8 send.py M1 > local/evidence/send-M1.txt 2>&1; echo "exit $?" >> local/evidence/send-M1.txt; cat local/evidence/send-M1.txt; fi
   cd /c/bridge-lab/1381-rr && if [ -e local/evidence/m1-state.txt ]; then echo "STOP: local/evidence/m1-state.txt exists"; else python -I -X utf8 measure.py state M1 > local/evidence/m1-state.txt 2>&1; echo "exit $?" >> local/evidence/m1-state.txt; cat local/evidence/m1-state.txt; fi
   ```
3. **The shut** (v3 C6's "Shut THE COMPANY first"), by RA's keys: a click on the Gateway's left panel; Alt+K (the Company menu); one click on Shut; Enter on the one row `BRIDGE STOCK CREATE LAB`. If R7's pair follows: zoom, then No by click on the renewal dialog; zoom, then "T: Continue In Educational Mode" by click on the License screen; screenshot each (`local/shots/s3-renewal.png`, `local/shots/s3-licence.png`, kept on the lab PC). Anything else: STOP-LICENCE or STOP-DIALOG. Then a screenshot `local/shots/s3-select.png`: EXPECT Select Company, no company loaded, title EDU.
4. **The restore** (R5, v3 C6), once.
   - **The plan**, with no company loaded. Replace `<DATA>` (Shell). EXPECT `PLAN WRITTEN: the source folder holds one file, 24219 B, sha256 ecc8d4028b88b763e5a8a22014bc699b07db68c3eb32cff3c6c92052b373b0f4 (equal to recorded), …` and `exit 0`. `STOP-RESTORE-SOURCE` (exit 1): STOP-RESTORE-SOURCE, nothing is restored. `FAILED READ` or `REFUSED` (exit 2): STOP-RESTORE-SOURCE as well, recorded as "the check failed", never as "no file". Write down the file's modified time the line prints.
     ```bash
     cd /c/bridge-lab/1381-rr && if [ -e local/evidence/r-plan.txt ]; then echo "STOP: local/evidence/r-plan.txt exists"; else python -I -X utf8 restore.py plan <DATA> > local/evidence/r-plan.txt 2>&1; echo "exit $?" >> local/evidence/r-plan.txt; cat local/evidence/r-plan.txt; fi
     ```
   - **On Tally's screen**, by row 1108-R3's keys (Difference 4 and step 9), with the UTC time at the first click and at the return written down. Click Data, Backup & Restore, Restore. EXPECT the Restore Data Path to read `<DATA>`'s folder (zoom; another: STOP-RESTORE-SOURCE, press no key). Choose Specify Path (click to highlight, Enter) and type `C:\bridge-lab\1108r3\backups\bks` (zoom: exact); Enter. EXPECT the list to show `BRIDGE STOCK CREATE LAB` alone, with the number `restore-plan.txt` names and `19-Nov-26 (7:40 pm)` (lab 1108-R3, step 9): screenshot `local/shots/s4-list.png`; anything else: STOP-RESTORE-SOURCE, press no key. Enter on that row; click End of List alone. EXPECT "Accept ?": screenshot `local/shots/s4-accept.png`, then `y`. EXPECT the folder-exists prompt naming the plan's number: screenshot `local/shots/s4-replace.png` (zoom on the number), then Replace (`r`, as row 1108-R3). EXPECT Select Company with no company loaded within 10 minutes of the Replace: screenshot `local/shots/s4-after.png`, title EDU.
   - **Stops here** (R5): any licence, activation, renewal or Educational-Mode dialog or screen: press no key, copy its text, screenshot, STOP-LICENCE. Any other prompt, or a list that differs from the plan: press no key, screenshot, STOP-DIALOG (a list: STOP-RESTORE-SOURCE). No Replace prompt, or not back at Select Company in 10 minutes: press no key, screenshot, STOP-RESTORE "the restore of BK-S did not complete". The fallback is named in The backup, restored only on the run coordinator's word.
   - **The after-record**, back at Select Company, before the load. EXPECT `AFTER RECORDED: the source unchanged …`, the counts, each changed or added entry by name, and `exit 0`; `NO DIFFERENCE` on its own line is recorded as the finding "the restore left the data folder's listing unchanged" (Q4), not a STOP. `STOP-RESTORE` (exit 1): STOP-RESTORE "the restore of BK-S did not complete". `STOP-RESTORE-SOURCE` (exit 1), `FAILED READ` or `REFUSED` (exit 2): STOP-RESTORE-SOURCE, recorded as "the check failed".
     ```bash
     cd /c/bridge-lab/1381-rr && if [ -e local/evidence/r-after.txt ]; then echo "STOP: local/evidence/r-after.txt exists"; else python -I -X utf8 restore.py after <DATA> > local/evidence/r-after.txt 2>&1; echo "exit $?" >> local/evidence/r-after.txt; cat local/evidence/r-after.txt; fi
     ```
5. **The load after the restore**, by step 1's keys; screenshot `local/shots/s5-select.png`; Enter. The book absent from the list, or not opening: STOP-RESTORE "the restore of BK-S did not complete". Then, before any request (v3 C6), a screenshot of the whole Tally window, `local/shots/s5-window.png` (no dialog, no half-entered screen), and one of the Gateway, `local/shots/s5-gateway.png`: EXPECT `BRIDGE STOCK CREATE LAB` as the one company, the title band EDU (else STOP-EDITION), no dialog (a licence screen: STOP-LICENCE). Record the period and the current date the Gateway shows (EXPECT as at step 1; another is recorded, not a STOP).
6. **M2, its state, then I, G, U.** Each line in turn, the next only after the last one's file ends as EXPECTed. Each send: `VERDICT SENT-OK` and `exit 0`; exit 1: STOP-REQUEST-FAILED; exit 2: STOP-SEND-REFUSED. M2's state: `BOOK STATE MET` and `exit 0`; `STOP-RESTORED-STATE` (exit 1) or `NOT READ` (exit 2): STOP-RESTORED-STATE, I, G and U are not sent (`send.py` will not send I), and the compute (step 7) is still run.
   ```bash
   cd /c/bridge-lab/1381-rr && if [ -e local/evidence/send-M2.txt ]; then echo "STOP: local/evidence/send-M2.txt exists"; else python -I -X utf8 send.py M2 > local/evidence/send-M2.txt 2>&1; echo "exit $?" >> local/evidence/send-M2.txt; cat local/evidence/send-M2.txt; fi
   cd /c/bridge-lab/1381-rr && if [ -e local/evidence/m2-state.txt ]; then echo "STOP: local/evidence/m2-state.txt exists"; else python -I -X utf8 measure.py state M2 > local/evidence/m2-state.txt 2>&1; echo "exit $?" >> local/evidence/m2-state.txt; cat local/evidence/m2-state.txt; fi
   cd /c/bridge-lab/1381-rr && if [ -e local/evidence/send-I.txt ]; then echo "STOP: local/evidence/send-I.txt exists"; else python -I -X utf8 send.py I > local/evidence/send-I.txt 2>&1; echo "exit $?" >> local/evidence/send-I.txt; cat local/evidence/send-I.txt; fi
   cd /c/bridge-lab/1381-rr && if [ -e local/evidence/send-G.txt ]; then echo "STOP: local/evidence/send-G.txt exists"; else python -I -X utf8 send.py G > local/evidence/send-G.txt 2>&1; echo "exit $?" >> local/evidence/send-G.txt; cat local/evidence/send-G.txt; fi
   cd /c/bridge-lab/1381-rr && if [ -e local/evidence/send-U.txt ]; then echo "STOP: local/evidence/send-U.txt exists"; else python -I -X utf8 send.py U > local/evidence/send-U.txt 2>&1; echo "exit $?" >> local/evidence/send-U.txt; cat local/evidence/send-U.txt; fi
   ```
   Then a screenshot of the whole Tally window, `local/shots/s6-after-u.png`: the Gateway as at step 5, no dialog, title EDU.
7. **The compute**, from the files only; nothing is sent. EXPECT `ALL 5 ANSWERS READ` and `exit 0`. `NOT ALL READ` (exit 2) is recorded with each `NOT READ` reason; it is no STOP, since nothing is left to send, but the questions that answer needs stay open.
   ```bash
   cd /c/bridge-lab/1381-rr && python -I -X utf8 measure.py all local/gen/ra > local/evidence/measure-all.txt 2>&1; echo "exit $?" >> local/evidence/measure-all.txt; cat local/evidence/measure-all.txt
   ```
8. **Close** (R11), by step 3's keys and R7's pair if it comes (`local/shots/s8-renewal.png`, `local/shots/s8-licence.png`); then a screenshot `local/shots/s8-select.png`: EXPECT Select Company, no company loaded, title EDU. Then the processes, the port and GET `/status` again; no XML request is sent after the shut. EXPECT the same one `tally.exe` PID as step 0 owning `:9001`, and `200 <bytes>` with `exit 0`. Then the files: `check_sheet.py` exit 0 with its three `OK` lines, and the listing's `exit 0`.
   ```bash
   cd /c/bridge-lab/1381-rr && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > local/s8-tally.txt 2>&1; echo "exit $?" >> local/s8-tally.txt; cat local/s8-tally.txt
   cd /c/bridge-lab/1381-rr && netstat -ano > local/s8-netstat.txt 2>&1; echo "exit $?" >> local/s8-netstat.txt; grep -c LISTENING local/s8-netstat.txt; grep -E ':9001 .*LISTENING' local/s8-netstat.txt
   cd /c/bridge-lab/1381-rr && curl.exe -sS -w "%{http_code} %{size_download}\n" -D local/evidence/s8-status.headers.txt http://127.0.0.1:9001/status -o local/evidence/s8-status.txt > local/evidence/s8-status-curl.txt 2>&1; echo "exit $?" >> local/evidence/s8-status-curl.txt; cat local/evidence/s8-status-curl.txt
   cd /c/bridge-lab/1381-rr && python -I -X utf8 check_sheet.py > local/evidence/s8-sheet.txt 2>&1; echo "exit $?" >> local/evidence/s8-sheet.txt; cat local/evidence/s8-sheet.txt
   cd /c/bridge-lab/1381-rr && find local -type f ! -name s8-listing.txt | sort | xargs -d '\n' sha256sum > local/evidence/s8-listing.txt 2>&1; echo "exit $?" >> local/evidence/s8-listing.txt; wc -l local/evidence/s8-listing.txt
   ```

## Stops

Each leaves things as they stand and is reported, with its files and what the book holds (R10), before anything else: STOP-FOLDER, STOP-COPY, STOP-RERUN, STOP-LOCAL-PREFILLED, STOP-PYCACHE, STOP-CODE-COMMIT, STOP-SELFCHECK, STOP-STRAY, STOP-TALLY, STOP-OTHER-COMPANY, STOP-BOOK-ABSENT, STOP-EDITION, STOP-DIALOG, STOP-LICENCE, STOP-BOOK-STATE, STOP-RESTORE-SOURCE, STOP-RESTORE, STOP-RESTORED-STATE, STOP-SEND-REFUSED, STOP-REQUEST-FAILED, STOP-NO-ANSWER. After any STOP from step 2 on, the row does not resume in this sitting (R9, R10), and the restore is never made twice.

## Record

For each read: `local/answers/a-<label>.xml` (the answer exactly as received), `a-<label>.headers.txt`, `a-<label>.transport.txt`, its `local/evidence/send-<label>.txt`, and its line in `local/evidence/send-log.txt`. For the restore: `local/evidence/r-plan.txt` and `local/evidence/r-after.txt` (pushed), `local/restore-plan.txt`, `local/r-before-listing.txt` and `local/r-after-listing.txt` (kept on the lab PC, R13), the screenshots by name, and the notes line with the UTC times of the first click and the return, the prompts as seen, and the keys pressed. For the sitting: step 0's files, `m1-state.txt`, `m2-state.txt`, `measure-all.txt`, the two `/status` captures, `local/evidence/s8-listing.txt`, and the notes: UTC times, one line per step, the title band at each screenshot, the Gateway's period and date at steps 1 and 5, and the keys of the loads and the shuts.

## Computes

`measure.py all` prints, from the files alone, per label: bytes, sha256 and its rows, or `NOT READ` with its reason (never zero rows); for I, G and U how many rows are bound to the book. Then M1's and M2's state; M1 against M2, `byte-equal yes|no` and `rows equal yes|no`; and I, G and U against RA's `a-I1.xml`, `a-G1.xml` and `a-U1.xml`, byte-equal and rows equal (rows compared as canonical XML: tag, attributes, trimmed text, children in order). `restore.py after` prints the data-folder listing difference: entries added, removed and changed (bytes or modified time), by name.

## Decides

- **Q1** by step 4's notes and screenshots (the prompts seen, the time to return) and `r-after.txt`: yes if the two prompts were the only ones, no licence screen came, Tally returned to Select Company within 10 minutes and `AFTER RECORDED` printed; a STOP-LICENCE, STOP-DIALOG or STOP-RESTORE answers Q1 no, with its text.
- **Q2** by `m2-state.txt` and the M1 against M2 line.
- **Q3** by the three lines against RA's answers. A difference while M2's state is met is a finding for the run coordinator (the book's masters differ though its mark does not), not a STOP after the fact.
- **Q4** by `r-after.txt` and the step 5 and step 6 screenshots.
- Each answer is one book, one backup, one release, one sitting, in Educational Mode: partial, and recorded so.

## Not covered

- **A restore that undoes a change, in Educational Mode.** The book is already at BK-S's state, so this row cannot show a restore reversing anything; row 1381-RB's write and restore is that measurement.
- **A backup in Educational Mode**, a write, and any restore of row 1381-O's backup-1.
- **A licensed restore**: made already (lab 1108-R3, step 9); not repeated.
- **The count read**, a Bridge build, other books, releases and sittings, and transport failure (recorded apart from the answer if it happens, R4).

## Close and evidence

Step 8 leaves Tally at Select Company with no company loaded, title EDU, the one `tally.exe` owning 9001, and the book restored from BK-S. Report to the run coordinator: the five send lines, the two state files, `r-plan.txt`, `r-after.txt`, `measure-all.txt` whole, the step 4 notes line, the title band and Gateway lines, and any STOP with its name, files and what the book holds. The evidence branch may carry `local/evidence/` and the five answers with their headers and transport records; `restore-plan.txt`, the two listings, the screenshots and the `tally.exe` and port listings stay on the lab PC (R13), all listed by name, size and sha256 in a `kept-local.txt`. The branch is pushed only after the pre-push scan and the run coordinator's yes.
