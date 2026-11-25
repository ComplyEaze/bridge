# Row 1381-RA: the stock reads slice 5b-3 admits, captured raw in Educational Mode

For #1381. Slice 5b-3 will admit a stock-item draft only from a complete read: the candidate stock-item rows equal to the company's own item count, each read of a pair byte-equal, and the company marks unchanged from the bind to the aim. No row has sent the company count read and the candidate read to one book in one sitting (row 1381-R1 sent the candidate read with no count read; the count captures on master are of other books and other item reads). This row sends five reads by file, each twice except the marks read, which opens and closes the sitting: M1; C1, C2; I1, I2; G1, G2; U1, U2; M2. It keeps every answer exactly as received and computes, from the files, what the slice's rule would judge. No Bridge build runs, nothing is written to the book, and nothing is sent but these ten reads between M1 and M2.

**How an EXPECT is marked.** **(code `file:line`)** is read at commit 6c9e6b33b531d0bd23bf6c4c62ba0d01a0aa4a1d, master when this sheet was drafted (all code lines cited are that commit's); **(lab `row`)** is an earlier observation; **(not shown)** is what this sitting measures. Every computed line is a measurement, never a pass or fail of the code.

**Rules.** `LAB-COMMON-RA.md` (R1 to R13) applies in full: the one synthetic book, Educational Mode as the state measured (a title band other than EDU is STOP-EDITION), ten reads by `send.py` only and nothing else between M1 and M2, no write, no backup, no restore, the licence pair at Shut Company only, nothing ends a process, everything the lab adds under `local/`, every STOP named and left as it stands. This row does not carry LAB-COMMON v3 (R2 says why; each R clause names the v3 clause it keeps or changes).

## The questions

- **Q1.** In one sitting, does the count read answer one company row with `NUMSTOCKITEMS` reported, and does the candidate read answer as many `STOCKITEM` rows as that count, in both reads of each pair?
- **Q2.** Is each pair byte-equal: C1 and C2, I1 and I2, G1 and G2, U1 and U2? Where a pair is not, are its rows equal?
- **Q3.** Are M1 and M2 equal: the same ALTMSTID, and ALTVCHID absent in both, across the ten reads?
- **Q4.** In Educational Mode, do M1, I1, G1 and U1 equal the answers earlier rows recorded for this book at the same state in Silver (lab 1381-O, 19 Nov 2026), byte for byte or by rows? And what do the title band and the Gateway show?

## The reads

All five requests are one Export of a Collection, sent as a byte-order mark then UTF-16LE with `Content-Type: text/xml; charset=utf-16`. Each was written by `gen_ra.py` from its port of the builder named; none was typed.

| labels | request | builder (code) | bytes | sha256 | git blob | byte-equal to |
|---|---|---|---|---|---|---|
| M1, M2 | `requests/marks.utf16le.xml` | `render_agent_company_high_water`, `src-tauri/src/agent_read_profiles.rs:590-600` | 990 | `5802b3feca930e569de996e5582299ad34f2515140b233a7140290f8f874552b` | ae363536 | the marks request of rows 1381-M (`lab/1381-stock-item-create` at 5bbd6fb2), 1381-R1 (`lab/1381-r1` at c76e23d6), 1381-N and 1381-O (`lab/1381-o-sheet` at 54eb636f), and master's fixture `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/partial-landing-rows/partial-landing-company-marks.request.utf16le.xml` |
| C1, C2 | `requests/count.utf16le.xml` | `render_company_inventory_flags_request`, `src-tauri/crates/bridge-tally-protocol/src/native_stock_summary.rs:79-89` | 1654 | `55f751fed504974e3ba084bbfd8831c62edd1fcc3b574d8af35666c49a00823c` | d9d50231 | no earlier file: new to this book |
| I1, I2 | `requests/items.utf16le.xml` | `render_candidate_stock_items_request`, `src-tauri/crates/bridge-tally-protocol/src/native_masters.rs:272-281` | 1330 | `581b1d95df1b7b80c846ce13e7df18cc47defddb1c646eb025d48bfa5afe0ac1` | dfa57543 | master's fixture `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items.request.utf16le.xml`, and the `a1-candidate-items` request of rows 1381-R1, 1381-N and 1381-O |
| G1, G2 | `requests/groups.utf16le.xml` | `render_native_masters_request(StockGroups, …)`, `native_masters.rs:257-266`, shape :116-121 | 1144 | `d79503503a31c4fee4195b2e03308c2833aa2e9c3072bd4cc0ed1f73d9121245` | 81407606 | the groups request of rows 1381-R1, 1381-N and 1381-O |
| U1, U2 | `requests/units.utf16le.xml` | `render_native_masters_request(Units, …)`, `native_masters.rs:257-266`, shape :110-115 | 1174 | `3f79503b5480a6e3a23ec6fb8fc8fd5ca8200d4f0889590c994d1240ed299d0c` | ea630407 | the units request of rows 1381-R1, 1381-N and 1381-O |

Row 1381-M's groups and units requests (blobs 472c7e74 and ea4ad648) are other shapes and are not these.

## What each read sends

Each carries `SVEXPORTFORMAT` `$$SysName:XML` and `SVCURRENTCOMPANY` `BRIDGE STOCK CREATE LAB`, written by the one company-variable writer (code `src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs:130-135`, reached by :124-126 for the candidate read and :513-519 for the others), which escapes the name once (code `src-tauri/crates/bridge-tally-protocol/src/xml_text.rs:41-49`; this name needs no escape). The wire bytes are a byte-order mark and the text in UTF-16LE, nothing added (code `src-tauri/crates/bridge-tally-protocol/src/text_encoding.rs:42-49`).

- **marks** (M1, M2): collection `Bridge Agent Company High Water`, `TYPE` Company, `FETCH` `GUID,ALTVCHID,ALTMSTID`, no filter: one row per company loaded, which is how M1 shows that the book is the one company loaded.
- **count** (C1, C2): collection `BridgeCompanyBookExtentV2`, `TYPE` Company, filtered by the formula `$GUID = "<the book's GUID>"`; `FETCH` the extent fields (code `src-tauri/crates/bridge-tally-protocol/src/outstandings_shared.rs:750-751`: Name, GUID, CompanyNumber, BooksFrom, LastVoucherDate, ALTVCHID, ALTMSTID), then `ISINTEGRATED`, `ISINVENTORYON`, `ISBATCHWISEON` and the seven `NUM…` counts, `NUMSTOCKITEMS` among them. The product reads that count by `item_count` (code `native_stock_summary.rs:687-698`): trimmed, plain ASCII digits, else unavailable; `measure.py` reads it the same way.
- **items** (I1, I2): collection `Bridge Master Stock Items`, `TYPE` StockItem, `FETCH` the eleven candidate fields (code `native_masters.rs:241`: NAME, PARENT, GUID, MASTERID, ALTERID, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, HSNDETAILS.LIST, GSTDETAILS.LIST, LANGUAGENAME.LIST), `COMPUTE` `BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany` (code `native_masters.rs:243-253`).
- **groups** (G1, G2): collection `Bridge Master Stock Groups`, `TYPE` StockGroup, `FETCH` NAME, PARENT, GUID, MASTERID, ALTERID, the same `COMPUTE`.
- **units** (U1, U2): collection `Bridge Master Units`, `TYPE` Unit, `FETCH` NAME, GUID, MASTERID, ALTERID, ORIGINALNAME, DECIMALPLACES, ISSIMPLEUNIT, the same `COMPUTE`.

**The size.** The product admits the candidate read only up to master mark 489 under its 16,000,000-byte limit (code `src-tauri/src/tally/runtime_masters_tests.rs:349-380`). This book's mark is 210 (lab 1381-O). This row's own bound on every answer is 2,000,000 bytes (R4); the earlier rows' answers for this book were 3,318 to 8,270 bytes (lab 1381-O).

## Educational Mode and dates

- **No date window.** None of the five requests carries `SVFROMDATE` or `SVTODATE`, or any other date. `gen_ra.py --selftest` and `gen_ra.py check` refuse a request with either, and `check_sheet.py` checks the five files again on their own. The Educational day 1, 2 and 31 rule for date-bearing reads (`docs/tally/TALLY_PROTOCOL_REFERENCE_READS_AND_DATES.md`) does not touch these shapes. Had one carried a date, the sheet would not have been written: it would have stopped and said so.
- **The product does not send these reads in Educational Mode.** The masters runtime refuses `EducationRestricted` before it reads (code `src-tauri/src/tally/runtime_masters.rs:382-383` and :497-498). So this row measures Tally's answers to the bytes the product would send, not the product's path; a slice that admits these reads in Educational Mode would need its own row.
- **The edition is recorded** from Tally's title band at step 0, after the load and at the close: EXPECT `TallyPrime EDU` (lab 2162-PA, 25 Nov 2026: the Silver Rental licence lapsed on 24-Nov-2026, and the About screen read Application status Educational Mode). Any other: STOP-EDITION (R2).

## How the requests were made, and how to make them again

`gen_ra.py` holds a line-for-line port of each builder, of the company-variable writer and its escape, and of the wire encoding, with the file:line it ports. It is in the shape of row 2162-PA's `rbreq.py` (`lab/2162-pa` at f682a6aa, blob b1cf399f) and row 1381-R1's `gen_r1.py` (blob 3f6cb8b4).

- **The port against the code.** `gen_ra.py --selftest` renders each builder for the code's own test inputs and compares the sha256 of its UTF-8 bytes with the code's pinned table (code `src-tauri/crates/bridge-tally-protocol/src/request_bytes_tests.rs:114-115`, :118-119, :134-135, :198-199, inputs :94 and :453; `src-tauri/src/agent_import_request_bytes_tests.rs:170-171`, input :74): five pins equal. Four controls (a company one character different, a date window added, an Import, no byte-order mark) each fire. The pins are copies, in `gen_ra.py`, of the code's table at 6c9e6b33, compared with it once when this sheet was drafted and again by its reader; the self-test does not read the Rust, so a later change to a builder is not seen here. The row measures the bytes of 6c9e6b33.
- **The port against committed requests.** `gen_ra.py fixtures <fixtures folder at 6c9e6b33>` renders the five request captures the code's tests compare with and compares them byte for byte: groups and units for BRIDGE SHAPE LAB (code `src-tauri/crates/bridge-tally-protocol/src/native_masters_tests.rs:152-160`), the count for BRIDGE SHAPE LAB (`native_stock_summary_tests.rs:278-283`), the candidate read for this book (`src-tauri/crates/bridge-tally-protocol/tests/stock_item_candidate_request.rs:17-28`), and the marks read for BRIDGE SHAPE LAB (a committed request no Rust test reads; the pin is the code's check). Five equal.
- **This row's files.** `gen_ra.py write <new folder>` wrote the five into `requests/`; `gen_ra.py check .` renders them again and compares byte for byte, then checks each is one Export of a Collection with no date and no import.
- **To make them again**, on any checkout at 6c9e6b33 or later that holds this folder, from the repository root, with Python 3.10 or later: `python3 -I -X utf8 lab/1381-ra/gen_ra.py write <an empty folder>`, then `cmp` each file in `<that folder>/requests/` with this folder's; and `python3 -I -X utf8 lab/1381-ra/gen_ra.py fixtures src-tauri/crates/bridge-tally-protocol/tests/fixtures` at 6c9e6b33.

## The book

`BRIDGE STOCK CREATE LAB`, synthetic; its GUID `bac2b5e9…` is listed in master's `scripts/merge-gate-synthetic-uuids.json`. Its provenance is `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/PROVENANCE.md`.

- **Its stock state (lab 1381-O, `lab/1381-o-answers` at cffe006d, its `RESULTS.md` and `lab/1381-o-answers/evidence/notes.txt`).** Restored on 19 Nov 2026, after that row's writes, to the state it held before them (BK0), and the restore checked: the marks, item, group and unit reads after it byte-equal to those before the writes. One stock item `SC Sibling Goods`, one group `SC Goods Group`, one unit `Nos`; master mark ALTMSTID 210; no ALTVCHID element (no voucher). Gateway period 1-Apr-25 to 31-Mar-26, current date Tuesday 1-Apr-2025. That row ran on TallyPrime 7.2 Silver (ProductVersion 1.1.7.2).
- **The latest answers branch to load it** is row 1108-R3's (`lab/1108-r3-equal-voucher-answers` at a64e8036, its `RESULTS.md` and `lab/1108-r3-equal-voucher-answers/evidence/notes.txt`), later the same day, licensed (`education_mode` false). It opened at ALTMSTID 210 with no voucher, backed the book up (BK-S), restored another state for its own test and posted one voucher there, then restored BK-S; its closing marks, ledger and voucher reads were byte-equal to its opening ones (ALTMSTID 210, ALTVCHID absent, no voucher, current date Tuesday 1-Apr-2025). It read no stock item, group or unit. Every lab branch updated since 19 Nov was searched for the book's name: no other row loaded it (row 1381-BX's input names it only as text in a box, and that row loaded no company). So the EXPECT for M1 is the state above; `measure.py state` checks it (R1) before any other read, and ALTMSTID 210 says no master was created or altered since.
- **The answers recorded for it** (lab 1381-O, `kept-local.txt`, byte-equal to the stock-create-lab captures on master): marks 3,318 B sha256 `ed6554bc…`, items 8,270 B `92508c8e…`, groups 4,170 B `b57cf4dc…`, units 4,040 B `cb9d6933…`. `measure.py all` compares M1, I1, G1 and U1 with these.
- **The backups that restore it** (not used by this row, R5), both taken at ALTMSTID 210 with no voucher, each restored once and its reads checked: BK-S, the latest, `C:\bridge-lab\1108r3\backups\bks\TDBK1800_<company number>.001`, 24,219 bytes, sha256 `ecc8d4028b88b763e5a8a22014bc699b07db68c3eb32cff3c6c92052b373b0f4` (lab 1108-R3, steps 2 and 9: marks, ledgers and vouchers checked); and `C:\lab\1381-o\backup-1\TDBK1800_<company number>.001`, 24,219 bytes, sha256 `d23f881a6bac8e1f9bf53035e7f21a72234f9f57c3672a965bc23a3dd76d44eb` (lab 1381-O, S2 and S6: marks, items, groups and units checked). A restore is a separate act, by v3 C6, only on the run coordinator's word.
- **The CMPINFO block.** The protocol reference records CMPINFO counters above the collection as request-scoped, differing between captures whose rows did not, and lists the byte-stability of two sends of one export as unmeasured (`docs/tally/TALLY_PROTOCOL_REFERENCE_COMPANY_IDENTITY_AND_CREATION.md`). They are not rows and are never counted (R6); a byte difference in a pair that sits only there is reported as "byte-equal no; rows equal yes".

## Order and the one variable

M1 first: it is the state check and the bind. Then each pair back to back, the count first since the slice's rule compares the items with it, then the items, then the two support reads. M2 last: the aim. Nothing else is sent between M1 and M2, so a difference inside a pair or between M1 and M2 is Tally's answer to the same bytes in one sitting, with no other request and no key between. The only act between M1 and C1 is `measure.py state`, which reads a file; the only act between C1 and C2 is a screenshot (R4: C1 is the one request new to this book).

## Shell

Every command is typed in **Git Bash**, from `/c/bridge-lab/1381-ra`, exactly as written: each line begins `cd /c/bridge-lab/1381-ra && ` and depends on no shell variable set by an earlier line. Step 0's first line does not: it makes the folder, so it cannot `cd` into it first; it writes its record into the `local/evidence/` it has just made, and when the folder already exists it prints its STOP and writes nothing. The interpreter is `python` (3.10) with `-I -X utf8`; `python3` is the Store alias and is never used. Every command's output goes to a file under `local/evidence/` with its exit code, then is printed whole; a command whose standard output is a file it makes (`git archive` into `tar`) writes its error output and exit there. A line that begins `#` is an instruction, not typed. Screenshots, the load and the shut are the operator's screen-tool acts on Tally's existing window (never "open application"); each screenshot is saved under `local/shots/` with its name and recorded in the notes.

## Files

Byte-equal to earlier rows' files (git blob ids):

- `.gitattributes`: blob 79cd6402, as rows 1381-BX (`lab/1381-bx-sheet` at 92f48553) and 2162-PA (`lab/2162-pa` at f682a6aa).
- `.gitignore`: blob 198748e3, `/local/`, as row 1381-BX's.
- The five request files: as The reads table says.

Changed copies, each change named in the file's own header:

- `send.py`: row 1381-R1's sender (`lab/1381-r1-answers/scripts/send.py` at 7bda13fc, blob 1efae5a3). The label names its request, so nothing outside the ten reads can be sent; the order is fixed and each label is sent once in the row's life; a failed send refuses every later one; C1 needs `local/evidence/m1-state.txt` to hold `BOOK STATE MET` and `exit 0`; the answer, its headers and its transport record are three files, so a failed request never reads as an empty answer; everything goes under `local/`. `--selftest` runs 23 synthetic cases.
- `measure.py`: row 1381-R1's reader (`check_r1.py` at c76e23d6, blob 774b684e: its forbidden-character pattern, parse and canonical rows), with this row's two commands `state` and `all`. `--selftest` runs 27 cases on master's committed captures, each negative case a capture with one edit.
- `check_sheet.py`: row 1381-BX's (`lab/1381-bx-sheet` at 92f48553, blob 92c4cdd2) with this row's checks: the ten send lines once each in Steps and in order, the state line between M1 and C1, the compute line after M2, the five request files' structure, `send.py`'s order, address and C1 rule held, no client timeout, and no GUID-shaped text in the two rule files. `--fresh` and STOP-PYCACHE as row 1381-BX's.

Written for this row:

- `gen_ra.py` (shape of row 2162-PA's `rbreq.py` and row 1381-R1's `gen_r1.py`): the five requests, made and checked (How the requests were made).
- `LAB-COMMON-RA.md`: the row's rules. `SHA256.txt`: every other file's sha256, bytes and path. `RUN-SHEET.md`: this sheet.

## Self-checks before the sitting

Run from `/c/bridge-lab/1381-ra` at step 0, after the fixtures are in `local/gen/fixtures/`. Nothing is sent to Tally.

```bash
cd /c/bridge-lab/1381-ra && python -I -X utf8 gen_ra.py --selftest > local/evidence/selfcheck-gen.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-gen.txt; tail -n 2 local/evidence/selfcheck-gen.txt
cd /c/bridge-lab/1381-ra && python -I -X utf8 gen_ra.py fixtures local/gen/fixtures > local/evidence/selfcheck-fixtures.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-fixtures.txt; tail -n 2 local/evidence/selfcheck-fixtures.txt
cd /c/bridge-lab/1381-ra && python -I -X utf8 gen_ra.py check . > local/evidence/selfcheck-requests.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-requests.txt; cat local/evidence/selfcheck-requests.txt
cd /c/bridge-lab/1381-ra && python -I -X utf8 send.py --selftest local/selftest/send > local/evidence/selfcheck-send.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-send.txt; tail -n 2 local/evidence/selfcheck-send.txt
cd /c/bridge-lab/1381-ra && python -I -X utf8 measure.py --selftest local/gen/fixtures local/selftest/measure > local/evidence/selfcheck-measure.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-measure.txt; tail -n 2 local/evidence/selfcheck-measure.txt
cd /c/bridge-lab/1381-ra && python -I -X utf8 check_sheet.py > local/evidence/selfcheck-sheet.txt 2>&1; echo "exit $?" >> local/evidence/selfcheck-sheet.txt; cat local/evidence/selfcheck-sheet.txt
```

EXPECT, in order: `5 pins equal; 5 book requests each a read with no date window; 4 controls fired`, exit 0; `5 fixtures, all byte-equal`, exit 0; the five rows of The reads table in `sha256  bytes  path` form and `5 requests equal to the render; each one Export of a Collection, no date window, no import`, exit 0; `23 cases, 23 right`, exit 0; `27 cases, 27 right`, exit 0; three `OK` lines, exit 0. Any other: STOP-SELFCHECK.

## Steps

**Prompt rule.** No dialog is expected but R7's licence pair after a Shut Company. Any other dialog or message on Tally's screen: copy its text, press no key, send nothing, STOP-DIALOG (a licence dialog anywhere else: STOP-LICENCE). Never end a process (R8).

**Send-line rule (steps 2 to 5).** A send line that prints `STOP: local/evidence/send-<label>.txt exists` is STOP-RERUN: that line was typed before, nothing was sent now, and the earlier record stands. A send line that has not returned after 10 minutes is STOP-NO-ANSWER (R4): leave that Git Bash window and its curl exactly as they are (no Ctrl-C, no closing the window, no new send in another window) and report.

0. **Before the sitting.** Nothing is sent to Tally in this step except the GET `/status` of its last line.
   - Make the row folder, fetch, and copy this sheet's folder into it byte for byte from the lab's Bridge clone. In every line of step 0, replace `<CLONE>` with the Git Bash path of the lab's Bridge clone, and `<SHEET>` with the 40-character commit of this branch the run coordinator's GO names. EXPECT `made` and `exit 0`; the fetch `exit 0`; the copy `exit 0 0` (git archive's, then tar's). `STOP: … exists`: STOP-FOLDER, nothing made. No `made` line because `/c/bridge-lab` itself is absent: STOP-FOLDER as well. Any other exit: STOP-COPY. The fetch is this row's one network act other than Tally's port: it brings the sheet's commit from origin into the lab's clone, and nothing in it goes to Tally.
     ```bash
     if [ -e /c/bridge-lab/1381-ra ]; then echo "STOP: /c/bridge-lab/1381-ra exists"; else mkdir /c/bridge-lab/1381-ra /c/bridge-lab/1381-ra/local /c/bridge-lab/1381-ra/local/evidence /c/bridge-lab/1381-ra/local/gen /c/bridge-lab/1381-ra/local/shots && echo "made" > /c/bridge-lab/1381-ra/local/evidence/s0-made.txt; echo "exit $?" >> /c/bridge-lab/1381-ra/local/evidence/s0-made.txt; cat /c/bridge-lab/1381-ra/local/evidence/s0-made.txt; fi
     cd /c/bridge-lab/1381-ra && git -C <CLONE> fetch origin > local/evidence/s0-fetch-origin.txt 2>&1; echo "exit $?" >> local/evidence/s0-fetch-origin.txt; cat local/evidence/s0-fetch-origin.txt
     cd /c/bridge-lab/1381-ra && { git -C <CLONE> archive <SHEET> lab/1381-ra | tar -x -C /c/bridge-lab/1381-ra --strip-components=2; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"; } > local/evidence/s0-copy.txt 2>&1; cat local/evidence/s0-copy.txt
     ```
   - **The pre-sitting check**, right after the copy and before anything else writes under `local/`: the sheet whole, and `local/` holding nothing but the three records above and this check's own file. EXPECT `OK: local/ holds only step 0's own records …` then the three `OK` lines, exit 0. `STOP-LOCAL-PREFILLED`: stop, delete nothing, report the names it prints. Any other miss: STOP-SELFCHECK.
     ```bash
     cd /c/bridge-lab/1381-ra && python -I -X utf8 check_sheet.py --fresh > local/evidence/s0-fresh.txt 2>&1; echo "exit $?" >> local/evidence/s0-fresh.txt; cat local/evidence/s0-fresh.txt
     ```
   - The code commit in the lab's clone, and the captures the self-checks and the compute read, from it. EXPECT `present 6c9e6b33…`; then `exit 0 0`. `ABSENT` or any other exit: STOP-CODE-COMMIT.
     ```bash
     cd /c/bridge-lab/1381-ra && if git -C <CLONE> cat-file -e "6c9e6b33b531d0bd23bf6c4c62ba0d01a0aa4a1d^{commit}" 2>/dev/null; then echo "present 6c9e6b33b531d0bd23bf6c4c62ba0d01a0aa4a1d"; else echo "ABSENT"; fi > local/evidence/s0-commit.txt 2>&1; cat local/evidence/s0-commit.txt
     cd /c/bridge-lab/1381-ra && mkdir local/gen/fixtures && { git -C <CLONE> archive 6c9e6b33b531d0bd23bf6c4c62ba0d01a0aa4a1d src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab src-tauri/crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_shape_lab_live.utf16le.xml src-tauri/crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_request.utf16le.xml src-tauri/crates/bridge-tally-protocol/tests/fixtures/masters_stock_groups_request.utf16le.xml src-tauri/crates/bridge-tally-protocol/tests/fixtures/masters_units_request.utf16le.xml src-tauri/crates/bridge-tally-protocol/tests/fixtures/note-days/shape_lab_company_marks_request.utf16le.xml | tar -x -C local/gen/fixtures --strip-components=5; echo "exit ${PIPESTATUS[0]} ${PIPESTATUS[1]}"; } > local/evidence/s0-fixtures.txt 2>&1; cat local/evidence/s0-fixtures.txt
     ```
   - Run all six Self-checks. Any miss: STOP-SELFCHECK.
   - Processes and the port, from Git Bash, read only. EXPECT for `bridge_mcp.exe` the one line `INFO: No tasks are running which match the specified criteria.`, exit 0 (else STOP-STRAY: a running one could send to 9001 during the sitting); for `tally.exe` exactly one line, its PID recorded (else STOP-TALLY); in the port listing at least one `LISTENING` line whose local address ends `:9001` carrying that PID, and a `LISTENING` line for another port (else the command failed: STOP-TALLY). The two `tally.exe` and port files stay on the lab PC (R13).
     ```bash
     cd /c/bridge-lab/1381-ra && tasklist //FI "IMAGENAME eq bridge_mcp.exe" //FO CSV //NH > local/evidence/s0-bridge.txt 2>&1; echo "exit $?" >> local/evidence/s0-bridge.txt; cat local/evidence/s0-bridge.txt
     cd /c/bridge-lab/1381-ra && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > local/s0-tally.txt 2>&1; echo "exit $?" >> local/s0-tally.txt; cat local/s0-tally.txt
     cd /c/bridge-lab/1381-ra && netstat -ano > local/s0-netstat.txt 2>&1; echo "exit $?" >> local/s0-netstat.txt; grep -c LISTENING local/s0-netstat.txt; grep -E ':9001 .*LISTENING' local/s0-netstat.txt
     ```
   - **The screen.** One screen-tool screenshot of the whole Tally window, `local/shots/s0-window.png`. EXPECT Select Company with no company loaded, the title band `TallyPrime EDU`, no dialog. A company loaded: STOP-OTHER-COMPANY, shut nothing. Another title: STOP-EDITION. A dialog or half-entered screen: STOP-DIALOG.
   - GET `/status`, the last act of step 0. EXPECT a first line `exit 0` in its file and the `-w` line `200 <bytes>` (lab 1381-R1, 1381-O and 1108-R3: `200 51`). Any other: STOP-TALLY.
     ```bash
     cd /c/bridge-lab/1381-ra && curl.exe -sS -w "%{http_code} %{size_download}\n" -D local/evidence/s0-status.headers.txt http://127.0.0.1:9001/status -o local/evidence/s0-status.txt > local/evidence/s0-status-curl.txt 2>&1; echo "exit $?" >> local/evidence/s0-status-curl.txt; cat local/evidence/s0-status-curl.txt
     ```
1. **The load** (R1). On Tally's Select Company screen: a click in the filter box, type `BRIDGE STOCK` in parts until one row is left, `BRIDGE STOCK CREATE LAB`; screenshot `local/shots/s1-select.png`; Enter. The book absent from the list: STOP-BOOK-ABSENT, press nothing. Then a screenshot of the whole window, `local/shots/s1-gateway.png`: EXPECT the Gateway with `BRIDGE STOCK CREATE LAB` as the one company, the title band EDU, no dialog. Record the period and the current date the Gateway shows (EXPECT 1-Apr-25 to 31-Mar-26 and Tuesday 1-Apr-2025, lab 1381-O; another date is recorded, not a STOP). A second company: STOP-OTHER-COMPANY. A dialog: STOP-DIALOG (a licence screen: STOP-LICENCE).
2. **M1, the bind, and the state.** No key, click or request in Tally from here to M2 but the C1 screenshot. EXPECT M1's file to end `VERDICT SENT-OK` and `exit 0`; then the state file `BOOK STATE MET` and `exit 0`. `REFUSED`, exit 2: nothing was sent; report it (STOP-SEND-REFUSED). Exit 1: STOP-REQUEST-FAILED. `STOP-BOOK-STATE` (exit 1): stop, nothing further is sent (R1; `send.py` will not send C1). `NOT READ` (exit 2): STOP-BOOK-STATE as well.
   ```bash
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-M1.txt ]; then echo "STOP: local/evidence/send-M1.txt exists"; else python -I -X utf8 send.py M1 > local/evidence/send-M1.txt 2>&1; echo "exit $?" >> local/evidence/send-M1.txt; cat local/evidence/send-M1.txt; fi
   cd /c/bridge-lab/1381-ra && python -I -X utf8 measure.py state > local/evidence/m1-state.txt 2>&1; echo "exit $?" >> local/evidence/m1-state.txt; cat local/evidence/m1-state.txt
   ```
3. **The count pair.** C1; then a screenshot of the whole Tally window, `local/shots/s3-after-c1.png` (EXPECT the Gateway as at step 1, no dialog, title EDU; a dialog: STOP-DIALOG, C2 not sent); then C2. EXPECT each file to end `VERDICT SENT-OK` and `exit 0`; exit 1: STOP-REQUEST-FAILED; exit 2: STOP-SEND-REFUSED.
   ```bash
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-C1.txt ]; then echo "STOP: local/evidence/send-C1.txt exists"; else python -I -X utf8 send.py C1 > local/evidence/send-C1.txt 2>&1; echo "exit $?" >> local/evidence/send-C1.txt; cat local/evidence/send-C1.txt; fi
   # the screenshot local/shots/s3-after-c1.png, then:
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-C2.txt ]; then echo "STOP: local/evidence/send-C2.txt exists"; else python -I -X utf8 send.py C2 > local/evidence/send-C2.txt 2>&1; echo "exit $?" >> local/evidence/send-C2.txt; cat local/evidence/send-C2.txt; fi
   ```
4. **The candidate pair, then the support pairs.** Each line in turn, the next only after the last one's file ends `VERDICT SENT-OK` and `exit 0`; exit 1: STOP-REQUEST-FAILED; exit 2: STOP-SEND-REFUSED.
   ```bash
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-I1.txt ]; then echo "STOP: local/evidence/send-I1.txt exists"; else python -I -X utf8 send.py I1 > local/evidence/send-I1.txt 2>&1; echo "exit $?" >> local/evidence/send-I1.txt; cat local/evidence/send-I1.txt; fi
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-I2.txt ]; then echo "STOP: local/evidence/send-I2.txt exists"; else python -I -X utf8 send.py I2 > local/evidence/send-I2.txt 2>&1; echo "exit $?" >> local/evidence/send-I2.txt; cat local/evidence/send-I2.txt; fi
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-G1.txt ]; then echo "STOP: local/evidence/send-G1.txt exists"; else python -I -X utf8 send.py G1 > local/evidence/send-G1.txt 2>&1; echo "exit $?" >> local/evidence/send-G1.txt; cat local/evidence/send-G1.txt; fi
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-G2.txt ]; then echo "STOP: local/evidence/send-G2.txt exists"; else python -I -X utf8 send.py G2 > local/evidence/send-G2.txt 2>&1; echo "exit $?" >> local/evidence/send-G2.txt; cat local/evidence/send-G2.txt; fi
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-U1.txt ]; then echo "STOP: local/evidence/send-U1.txt exists"; else python -I -X utf8 send.py U1 > local/evidence/send-U1.txt 2>&1; echo "exit $?" >> local/evidence/send-U1.txt; cat local/evidence/send-U1.txt; fi
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-U2.txt ]; then echo "STOP: local/evidence/send-U2.txt exists"; else python -I -X utf8 send.py U2 > local/evidence/send-U2.txt 2>&1; echo "exit $?" >> local/evidence/send-U2.txt; cat local/evidence/send-U2.txt; fi
   ```
5. **M2, the aim.** EXPECT `VERDICT SENT-OK` and `exit 0`. Then a screenshot of the whole Tally window, `local/shots/s5-after-m2.png`: the Gateway as at step 1, no dialog, title EDU.
   ```bash
   cd /c/bridge-lab/1381-ra && if [ -e local/evidence/send-M2.txt ]; then echo "STOP: local/evidence/send-M2.txt exists"; else python -I -X utf8 send.py M2 > local/evidence/send-M2.txt 2>&1; echo "exit $?" >> local/evidence/send-M2.txt; cat local/evidence/send-M2.txt; fi
   ```
6. **The compute**, from the files only; nothing is sent. EXPECT `ALL 10 ANSWERS READ` and `exit 0`. `NOT ALL READ` (exit 2) is recorded with each `NOT READ` reason; it is no STOP, since nothing is left to send, but the questions that answer needs stay open.
   ```bash
   cd /c/bridge-lab/1381-ra && python -I -X utf8 measure.py all local/gen/fixtures/agent/stock-create-lab > local/evidence/measure-all.txt 2>&1; echo "exit $?" >> local/evidence/measure-all.txt; cat local/evidence/measure-all.txt
   ```
7. **Close** (R11). Shut the book by Tally's own Shut Company: a click on the Gateway's left panel; Alt+K (the Company menu); Shut; Enter on the one row `BRIDGE STOCK CREATE LAB`. Write down the keys used. If R7's pair follows: zoom, then No by click on the renewal dialog; zoom, then "T: Continue In Educational Mode" by click on the License screen; screenshot each (`local/shots/s7-renewal.png`, `local/shots/s7-licence.png`, kept on the lab PC: they show the serial). Anything else: STOP-LICENCE or STOP-DIALOG. Then a screenshot `local/shots/s7-select.png`: EXPECT Select Company, no company loaded, title EDU. Then the processes, the port and GET `/status` again; no XML request is sent after the shut.
   ```bash
   cd /c/bridge-lab/1381-ra && tasklist //FI "IMAGENAME eq tally.exe" //FO CSV //NH > local/s7-tally.txt 2>&1; echo "exit $?" >> local/s7-tally.txt; cat local/s7-tally.txt
   cd /c/bridge-lab/1381-ra && netstat -ano > local/s7-netstat.txt 2>&1; echo "exit $?" >> local/s7-netstat.txt; grep -c LISTENING local/s7-netstat.txt; grep -E ':9001 .*LISTENING' local/s7-netstat.txt
   cd /c/bridge-lab/1381-ra && curl.exe -sS -w "%{http_code} %{size_download}\n" -D local/evidence/s7-status.headers.txt http://127.0.0.1:9001/status -o local/evidence/s7-status.txt > local/evidence/s7-status-curl.txt 2>&1; echo "exit $?" >> local/evidence/s7-status-curl.txt; cat local/evidence/s7-status-curl.txt
   ```
   EXPECT the same one `tally.exe` PID as step 0 owning `:9001`, and `200 <bytes>` with `exit 0`.
8. **The files.** EXPECT `check_sheet.py` exit 0 with its three `OK` lines; the listing's `exit 0`.
   ```bash
   cd /c/bridge-lab/1381-ra && python -I -X utf8 check_sheet.py > local/evidence/s8-sheet.txt 2>&1; echo "exit $?" >> local/evidence/s8-sheet.txt; cat local/evidence/s8-sheet.txt
   cd /c/bridge-lab/1381-ra && find local -type f ! -name s8-listing.txt | sort | xargs -d '\n' sha256sum > local/evidence/s8-listing.txt 2>&1; echo "exit $?" >> local/evidence/s8-listing.txt; wc -l local/evidence/s8-listing.txt
   ```

## Stops

Each leaves things as they stand (R10) and is reported, with its files, before anything else: STOP-FOLDER, STOP-COPY, STOP-RERUN, STOP-LOCAL-PREFILLED, STOP-PYCACHE, STOP-CODE-COMMIT, STOP-SELFCHECK, STOP-STRAY, STOP-TALLY, STOP-OTHER-COMPANY, STOP-BOOK-ABSENT, STOP-EDITION, STOP-DIALOG, STOP-LICENCE, STOP-BOOK-STATE, STOP-SEND-REFUSED, STOP-REQUEST-FAILED, STOP-NO-ANSWER. After any STOP from step 2 on, the row does not resume in this sitting (R9, R10): its pairs and its M1 to M2 bracket would not be one sitting.

## Record

For each read: `local/answers/a-<label>.xml` (the answer exactly as received), `a-<label>.headers.txt`, `a-<label>.transport.txt` (curl's exit, its `-w` line, the status, CONTENT-LENGTH, the answer's bytes and sha256, curl's error output, the verdict), its `local/evidence/send-<label>.txt`, and its line in `local/evidence/send-log.txt` (UTC start, first byte and total time). For the sitting: step 0's files, `m1-state.txt`, `measure-all.txt`, the two `/status` captures, `local/evidence/s8-listing.txt`, the screenshots by name, and the notes: UTC times, one line per step, the title band at each screenshot, the Gateway's period and date, and the keys of the load and the shut.

## Computes

`measure.py all` prints, from the files alone, per label: bytes, sha256 and the transport verdict, or `NOT READ` with its reason (never zero rows). Then:

- C1 and C2: the number of `COMPANY` rows, each row's GUID by its first 8 characters, and `NUMSTOCKITEMS` as `reported <n> (as received '<text>')` or `unavailable: absent`, `unavailable: empty` or `unavailable: not plain digits`. `COMPANYNUMBER` is never printed.
- I1 and I2: the number of `STOCKITEM` rows, how many are bound to the book (GUID prefix and `BRIDGECOMPANYGUID`), and whether the count equals `NUMSTOCKITEMS`, each item read against each count read (four lines).
- G1, G2, U1, U2: the number of `STOCKGROUP` and `UNIT` rows, and how many are bound to the book.
- M1 and M2: ALTMSTID, and ALTVCHID or `absent`; equal or not.
- Each pair: `byte-equal yes|no`, and if no, `rows equal yes|no` (rows compared as canonical XML: tag, attributes, trimmed text, children in order).
- M1, I1, G1, U1 against the recorded answers (The book): byte-equal and rows equal.

## Decides

- **Q1** by C1, C2, I1 and I2: one company row in each count read, `NUMSTOCKITEMS` reported, and each item count equal to each count read's. EXPECT (not shown) one row, `reported 1`, one item.
- **Q2** by the four pair lines. A pair not byte-equal but rows equal answers "stable rows, unstable bytes", and the bytes that differ are named from the files; rows not equal answers Q2 no.
- **Q3** by the M1 against M2 line.
- **Q4** by the four lines against the recorded answers, and the title band and Gateway lines. Rows not equal for I1, G1 or U1 while M1 met the state is a finding for the run coordinator (the book's masters differ though its mark does not), not a STOP after the fact.
- Each answer is one book, one release, one sitting, in Educational Mode: partial, and recorded so.

## Not covered

- **The scoping report's step 6.** A master changed between the bind and the aim, and a proposed name added by a person after the first candidate read: separate restored sittings with a change made in each, so this row's no-change sitting is their baseline, not their measurement.
- **The scoping report's step 7.** A book with no stock item: the eleven-field candidate pair and the count pair on a book with zero rows. This book has one item.
- **A licensed edition.** Every read here is in Educational Mode. Licensed answers for M, I, G and U exist (lab 1381-O); no count read has been sent to this book in any edition, so Silver and Educational Mode are not compared for C.
- **A Bridge build.** No build runs; the product refuses these reads in Educational Mode (Educational Mode and dates), and nothing in the product sends the candidate read or the stock-only recheck yet.
- **Other books, releases and sittings**, a second company loaded, a book above the candidate read's mark limit, and the Create through Bridge's own tools (row 1381-B).
- **Transport failure.** A cut or failed answer is not made on purpose; if one happens it is recorded apart from the answer (R4), and framed-protocol cases belong to the simulator tests.

## Close and evidence

Step 7 leaves Tally at Select Company with no company loaded, title EDU, the one `tally.exe` owning 9001. Report to the run coordinator: the ten send lines, `m1-state.txt`, `measure-all.txt` whole, the title band and Gateway lines, and any STOP with its name and files. The evidence branch may carry `local/evidence/` and the answers of M1, M2, I1, I2, G1, G2, U1 and U2 with their headers and transport records; C1's and C2's answers stay on the lab PC (they carry the company number), with their headers and transport records pushed; screenshots and the `tally.exe` and port listings stay on the lab PC (R13). All are listed by name, size and sha256 in a `kept-local.txt`. The branch is pushed only after the pre-push scan and the run coordinator's yes.
