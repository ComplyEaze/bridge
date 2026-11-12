# Lab rows 1108-R0, R1 and R2: row P2 again with Bridge's own reads kept whole, then with its refused voucher first, then last, each through `post_import`. SYNTHETIC COMPANY ONLY. Not for merge.

Part of #1108. Written 11 Nov 2026 from master de54a0f2, amended 12 Nov 2026 after an independent
read (the build line, the helpers, the checkers, the keys). About 110 minutes. R0's EXPECT is row
1108-A2's P2 as it ran (`lab/1108-a2-partial-landing-answers` at ebf70249; confidence: partial, one
run). Where R1 and R2 go beyond it, the EXPECT is from reading the code (confidence: unverified)
until this row runs. A different status is a result, and is recorded; only the STOP RULES stop the
row.

`SHA256.txt` lists every file here with its bytes and sha256. Step 0 checks it first, by
`setup-r.py`, before anything is sent or keyed: before LAB-COMMON C2's `GET /status` and m0.

## Why

The decision proposed on #1108 would name a refused voucher of a partly landed post by exclusion
from the post's own span, per content class. Two things it rests on are not yet measured:
- **The readback as bytes.** The 26 Oct condition "the verification readback committed as fixtures"
  is unmet. Row 1108-A2 kept each import's request and answer whole, but `relay.py` keeps a read
  only as its size and sha256, so Bridge's own mark and span reads exist as hashes alone.
- **The refused voucher's position.** A2 refused only the second of three. The rule needs the
  voucher mark to step by exactly `CREATED` and the span to hold exactly the created rows, wherever
  the refused voucher stands.

| Row | Batch | Order sent | What it decides |
|---|---|---|---|
| R0 | `batches/r0.json`, byte-equal to A2's `batches/p2.json` (d87c2862) | first, stock leg, third | P2 again, with every exchange between Bridge and Tally kept whole by `relay-keep.py`: the readback fixtures |
| R1 | `batches/r1.json` | stock leg, first, third | the same three vouchers with the refused one first |
| R2 | `batches/r2.json` | first, third, stock leg | the same three vouchers with the refused one last |

**One variable per row against R0: the position of the refused voucher in the request.** Each
voucher of R1 and R2 is R0's own JSON object: the same `bridge_txn_id`, date, type, ledgers, amounts
and narration. Only the order of the three objects differs; each file is written with P2's
separators and line end. The narrations "first" and
"third" are P2's labels, not positions. Every row starts from the same book (BK-R, step 5), with its
own new `bridge_mcp` process and its own new empty data folder, and makes the same calls in the
same order.

## Rules and company

`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy (sha256 5370c6d2) of A2's.

THE COMPANY is `BRIDGE STOCK CREATE LAB`, as A2 sitting 2's restore of BK-A2 left it: M 210, no
voucher mark (V blank), no ledger `A1 Closing Stock`, `A1 Capital` or `A2 Expense`, and no voucher.
If it is not on the PC, or its GUID differs from the one in rows 1108-A1 and A2's lab notes: STOP.
The row has no book; never rebuild it.

Differences from LAB-COMMON, as A2 ran them (the stricter rule holds everywhere else):
1. **Raw writes**, by `curl` to this lab book only, as the owner allowed on 26 Oct 2026. They are
   W1, W2, W3 and the seed S0, each sent once in this item, before BK-R. They are A2's request
   files byte for byte, shapes this book has answered twice. C4's EXPECT applies to each:
   `CREATED 1`, every other counter 0. No row sends them again (C3), which is why the rows share
   one row backup, BK-R, taken after them.
2. **Bridge writes**: the three posts R0, R1 and R2, through `post_import` only, each from its own
   fresh `build_import_xml` made just before it. No voucher of R0 to R2 is ever sent raw. Tally's
   counters for a post are recorded and checked against the EXPECT table, and they never stop the
   row by themselves.
3. **Who answers the box.** The lab answers Bridge's approval box itself, by mouse on the box's own
   button, through this sheet's `helpers/`: A2 sitting 2's `p1108s2.py` (sha256 59b4554f),
   `boxcheck.py` and `box.ps1`, with only their folders changed, as The files lists them. The rule
   is A2's Difference 3 as the run lead corrected it before A2's first write. Yes only if all of
   these hold:
   - exactly one line of the box begins `Create `, and that whole line is
     `Create 3 vouchers in "BRIDGE STOCK CREATE LAB"` (`agent_import_post.rs:4058`);
   - the whole lines `Company GUID: <THE COMPANY's GUID>` and `Batch: <the batch id this build
     returned>`;
   - the whole line `Total debit: 66  Total credit: 66`, as the build writes it (`:4077`). Any other
     spelling, `.00` included, fails.

   `boxcheck.py` is given count `3` and total `66` for every row. Per-voucher lines are not checked;
   in R1 the stock leg's line comes first. A box that fails any of these: Decline, record its text
   whole, and STOP. The helpers' row names are `r0`, `r1` and `r2`. They run as `helpers/` holds
   them, copied into the row folder by `setup-r.py`. A helper that differs from its `SHA256.txt`
   line, or any change made to one at the lab, is a STOP.
4. **Keys and processes.**
   - Keys are pressed in Tally only to load, back up, restore and shut THE COMPANY (C1, C5, C6,
     C10). Menus are opened by mouse click. Each is made as A2 sitting 2 made it on TallyPrime
     1.1.7.2 (its notes, 11 Nov), and the notes record each key and click as made:
     - **Load** (C1): at Select Company, type `BRIDGE STOCK CREATE LAB` to filter. EXPECT one row;
       press Enter.
     - **Backup** (C5): click Data, then Backup & Restore, then Backup. The path field shows `C:\`:
       press Backspace, choose Specify Path and type the step's folder. Type
       `BRIDGE STOCK CREATE LAB` to filter; EXPECT one row; press Enter; click End of List; press
       Enter. At `Accept ?` press `y`.
     - **The backup's done message**: A2 saw none. Tally went back to the Gateway with no other
       prompt, and the folder listing after (C5) is the proof. If a message that only says the
       backup is done appears: screenshot it and press Enter once (C5's accept once; the key is
       not yet observed: confidence unverified). Any other prompt: C7.
     - **Restore** (C6): click Data, then Backup & Restore, then Restore. Restore Data Path is
       Tally's data folder; choose Specify Path and type the backup's folder. EXPECT THE COMPANY
       with its number and the backup's date and time; press Enter; click End of List; press
       Enter. At `Accept ?` press `y`; at C6's folder-exists prompt choose Replace.
     - **Shut** (C6, C10): click Company, then Shut; press Enter on the one row.
   - If the coordinator rules that another key must be pressed (C7), run `front.ps1` first so
     Tally is in front. Never press Escape on a master, voucher or configuration screen.
   - No process is ended. At the end of each row, the client closes `bridge_mcp`'s input and the
     server exits by itself. Tally and the relay are left running until step 9; the relay has no
     stop of its own and is then stopped by stopping the background task it was started in.
5. **A decline is kept once.** A No, a timeout or an unanswered box is kept and returned once to
   its batch's next call, which does not ask the person again (`agent_import_approval.rs:16`;
   `begin` `:648`, `take_refusal` `:556`). Here any decline is followed by a STOP (3), so that next
   call is not made.
6. **A box left open** gives `approval.state` `pending` with `retry_after_s`. Call again after
   `retry_after_s`, with the box left as it is. `approved` without a post: call again at once.
   Never call again once an attempt is recorded.
7. **Size bound** for every raw read: 200,000 bytes.
8. **Python.** The lab PC's `python` is run for this sheet's `python3`, always with `-I`.
9. **An import's body.** The relay keeps each answer with its HTTP head. `body.py` (`helpers/`) writes
   the body alone, after checking it against CONTENT-LENGTH, for `check_1108.py import`. The relay's
   own files are never changed.

Licensed mode only; Education mode = STOP.

## Build

**Build line:** CI run 43008730655 of `ci.yml` (`schedule`, every job succeeded) on master commit
`de54a0f26e554f574727a0bc4914b0ad6ca88583`. That commit's `push` run, 43008194011, skipped both
bundle jobs, so it holds no Windows bundle. Artifact id 13850870591
(`unsigned-bundle-smoke-windows-latest`), 60,529,312 bytes, GitHub digest
`sha256:6385ae8446950765e4dfacd49e97babdf82fefc19b336e5241578a007c9fd3fe`, expiry
2026-11-18T11:13:57Z. Inside it, `bridge-tally.mcpb` has sha256
`b291ea38de06c0a36f80b707d0e6cc3bb92a8b05a41c7707f8ecf6ebde3c9f3c` and `bridge_mcp.exe`
(`bin/x86_64-pc-windows-msvc/` in it) has sha256
`ce96a872d7e60021f1cdbe26fb0708be0bacabe6bb731e7541838f248f5e3f9d`. Every value is as GitHub
reports it, and nothing was downloaded to fill this line: the runs and the artifact from the
Actions API, and the two sha256s from the log of that run's job `Bundle smoke (windows-latest)`
(job 130482763267), which prints `mcpb-smoke.json` whole. If the artifact has expired by the
sitting: STOP; the build is not remade here.

`mcpb-smoke.json` comes from the same job: its step runs `check_mcpb_bundle` on
`bridge-tally.mcpb` and writes the file into `src-tauri/target/release/bundle/`, and the upload
step puts that folder in the artifact. So the file is in the zip beside `bridge-tally.mcpb`. Its
`archive_sha256` is the `.mcpb`'s and its `binary_sha256` is the `bridge_mcp.exe` the checker
took from the `.mcpb`.

At step 0, before the first call: the zip's sha256 must equal the digest, and `bridge-tally.mcpb`'s
and `bridge_mcp.exe`'s must equal this line and `mcpb-smoke.json`. Any one differing is a STOP.
Record the exe's sha256 before the first row and after the last.
Every line number in this sheet is at de54a0f2.

Each row starts its own `bridge_mcp` over stdio, through the relay, with:
- `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`;
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9003`;
- `BRIDGE_AGENT_DATA_DIR` a new empty folder of that row's own: `C:\bridge-lab\1108r\agent-r0`,
  `agent-r1` or `agent-r2`, each shown absent by a listing before its row;
- `BRIDGE_AGENT_ENABLE_BATCH_POST` absent (set, it refuses as `setting_retired`, `agent.rs:1473`);
- `BRIDGE_AGENT_REDACTION` unset, so Tally's `LINEERROR` text is kept in the answer (`agent.rs:487`).

A row's data folder is its own so that no row's reads or verdicts lean on an earlier row's journal:
after a restore, the book no longer holds the vouchers that journal names. For the same reason each
row's client runs in a folder of its own, `C:\bridge-lab\1108r\agent-run-rk`, which `setup-r.py`
makes with an empty inbox: no row's client meets an earlier row's calls. Those are the two launch
differences from A2, whose P2 ran in the process, data folder and client folder that P1 had used.

The client is the lab's stdio client of 852-C1, as A2 ran it: no cancel, and no call limit under
10 minutes. Each tool call is made from the row folder as
`python -I -X utf8 call.py agent-run-rk <label> <tool> calls\<argument file>`, with the argument
files `args1108.py` writes.

## The files

- `requests/`: `marks`, `flags`, `ledgers`, `vouchers`, `w1-stock-ledger`, `w2-capital-ledger`,
  `w3-expense-ledger` and `s0-seed-journal`, each byte-equal to A2's `requests/` (its SHA256.txt).
  They are used as they are; `gen_requests.py` is not run.
- `batches/r0.json`, `r1.json` and `r2.json`: the `vouchers` arrays for `build_import_xml`.
  | Row | Voucher 1 | Voucher 2 | Voucher 3 |
  |---|---|---|---|
  | R0 | `b1108-p2-1`: `A2 Expense` Dr / `A1 Capital` Cr 21.00 | `b1108-p2-2`: `A1 Closing Stock` Dr / `A1 Capital` Cr 22.00 | `b1108-p2-3`: `A2 Expense` Dr / `A1 Capital` Cr 23.00 |
  | R1 | `b1108-p2-2` | `b1108-p2-1` | `b1108-p2-3` |
  | R2 | `b1108-p2-1` | `b1108-p2-3` | `b1108-p2-2` |

  All are Journals dated 14-Apr-2025, as P2's were.
- `check-request-ids.py`: A2's, byte for byte.
- `check_1108.py`: A2's (sha256 0898e7c1), made to fail closed. Its modes are `marks`, `import`,
  `rows` and `diff BEFORE AFTER`. An export answer must be an ENVELOPE with `HEADER/STATUS` 1 and
  exactly one `BODY/DATA/COLLECTION`, and `marks` needs exactly one COMPANY row. An import answer
  must be a RESPONSE holding every counter, as each import answer A2 kept is. An answer read but not
  so prints `FAIL` and exits 1. A file that is missing, empty, of odd length or does not parse prints
  `NOT READ` and exits 2. Neither is ever "nothing" or "no change". Run here on macOS on A2's kept
  answers (a made-up GUID in place of `<company guid>`) and on copies each broken one way: 20 of 20
  as expected, where A2's own fails 13.
- `relay-keep.py`: A2's `relay.py` (sha256 80ffdb20) with one change. Every request and answer, not
  only an import's, is kept whole as `x-N.request` and `x-N.answer`, N counting every request it
  serves, from 1. An import is kept as `import-N` as well, as before. `relay.jsonl` gains `x` on
  each line.
- `check-relay-keep.py`: A2's `check-relay.py` (sha256 9a806848) with its read check turned round
  (a read is kept whole) and four checks added for the `x-N` files. Run here on macOS before this
  sheet was sent: `0 failed` on `relay-keep.py` (24 checks), and 5 failed on A2's `relay.py`.
- `check-exchanges.py RELAYDIR [FIRST LAST]`: checks a relay folder after a row. A range that holds
  no exchange (FIRST above LAST, or below 1) fails, never `ok 0`. For every logged exchange from
  FIRST to LAST, both `x-N` files must exist and each body's size and sha256 must equal the log's.
  Each request is read from its own bytes. A bodiless `GET /status`, Bridge's status probe (240 of
  the 644 requests in A2 sitting 2's relay log), is listed as such. Any other must be a POST whose
  HEADER parses, listed by its `TALLYREQUEST`, `TYPE` and `ID`; a HEADER that does not parse fails.
  An import's `import-M` files must be byte-equal to its `x-N` files, and an `import-M` the log does
  not name fails. It ends `exchanges: ok N, FAIL 0 (x-FIRST to x-LAST), imports K` with exit 0, or
  exits 1. A folder or log it cannot read prints `NOT READ`, never "0 exchanges".
- `test-exchanges.py`: tests `check-exchanges.py` on a folder `relay-keep.py` wrote from five
  requests (three reads, a bodiless `GET /status` and an import): as written, over a range, and in
  twelve cases that must fail. `0 failed` (17 tests) here on macOS before this sheet was sent; the
  earlier `check-exchanges.py` fails 7 of them.
- `helpers/`: the helpers A2 sitting 2 ran (its answers branch, `sitting-2/scripts/`), each rebuilt
  to the sha256 A2 recorded, then changed only as below. Each file's first comment line says what
  changed.
  | File | A2's sha256 | Change |
  |---|---|---|
  | `p1108s2.py` | 59b4554f | the row folder; each row's client folder is its own `agent-run-rk`, not one `agent-run` |
  | `args1108.py` | 0720ecb2 | the row folder; batches from `lab/1108-r/batches/` |
  | `send.py` | f09dbccb | the row folder; the requests and `SHA256.txt` are this sheet's own, from `lab/1108-r/`; A2's `r2-sha.txt` branch is removed |
  | `chk.py` | a34cc6ae | the row folder; `check_1108.py` from `lab/1108-r/`; its STOP line says `NOT READ` for exit 2 and `FAIL` for exit 1 |
  | `boxcheck.py`, `cap.py`, `body.py` | 8c4df2bc, 20e3ce55, d0e09211 | the row folder only |
  | `box.ps1`, `call.py`, `show.py`, `capwin2.ps1`, `front.ps1` | 550fc11f, d72843b3, e4f1b295, 1a326a4c, 51c2bb22 | none: byte for byte |

  The row folder is `C:/bridge-lab/1108r`. The stdio client, `client.py` (1392-E's, sha256
  0ed0bb03), stays on the lab PC, as for A2.
- `setup-r.py`: makes the row folder `C:\bridge-lab\1108r` from this folder, as A2's `setup2.py`
  made its own. It first checks every file here against `SHA256.txt`, and that no other file is
  here, and makes nothing if one differs. It copies this folder whole to `lab\1108-r`, `helpers/`
  into the row folder, THE COMPANY's `guid.txt` and `company-number.txt` from
  `C:\bridge-lab\1108a2s2`, and the client into each row's own `agent-run-r0`, `-r1` and `-r2`.
  It makes no data, relay or backup folder, and refuses a row folder that already exists.

## The code admits each trigger (de54a0f2)

| Point | Code | For this sheet |
|---|---|---|
| A first voucher is needed | With no voucher mark, the build refuses as `empty_book_first_import` (`agent_import.rs:181`). | S0 seeds the voucher mark before BK-R. |
| Writes on, the batch included | One setting governs every post (`posting_from_env`, `agent.rs:714`). | the launch above |
| Journal numbering | `admit_numbering_for_post` (`agent_import_numbering.rs:493`, called at `agent_import_post.rs:1153`) admits only Automatic with Retain Original Voucher No. | step 7.3's EXPECT; any other = STOP, and no setting is changed |
| A Stock-in-Hand line | A Journal takes any balanced set, and Stock-in-Hand is non-money. | Each build EXPECTs no refusal. |
| The same `bridge_txn_id` in each row | The build refuses a repeated id only within one batch (`bridge_txn_id_invalid_or_duplicate`, `agent_import.rs:4616`). `vouchers_already_posted` (`agent_import_ledger.rs:1369`) reads the row's own data folder, new and empty. The pre-read refuses a voucher already in the book (`rows_already_in_book`, `agent_import_post.rs:2527`), and the restore before each row leaves none of R0 to R2's. | No refusal is EXPECTed. One would be a STOP (a refusal before an attempt). |
| Checks added since A2's base | The post path changed for Sales invoices and for a deposit from the cash box only. The cash-book read runs only for a binding with a projection, which a build makes exactly when the batch holds such a deposit (`agent_import.rs:889-904`). | This batch holds neither. |
| A batch left `reconciliation_required` | A2's E-NB: P2 and P3 were not refused before the box after P1's doubted batch. Here each row has its own data folder, so no earlier batch is in it. | `acknowledge_post_review` is not run in this sheet. |

## Steps

One at a time. Every answer goes to a file. A file with no answer is a failed step, not an empty one.
Each raw request is sent from the row folder by `send.py <label> <request file>`, and each answer is
read by `chk.py <name> <mode> <answer file>`, which runs `check_1108.py`.
No step asks for a read of an empty day to be complete: the raw reads here are of the whole book.

0. **Before anything is sent or keyed**, in this order:
   - **The build**, as the Build section says: the artifact not expired, the zip's sha256 equal to
     the digest, and `bridge-tally.mcpb`'s and `bridge_mcp.exe`'s equal to the Build line and to
     `mcpb-smoke.json`. Else STOP.
   - **The files**: `python3 -I setup-r.py`, from this sheet's folder, keeping its whole output.
     EXPECT `files: ok 33, FAIL 0` and `setup: ok`. Anything else: STOP.
   - From the row folder's `lab\1108-r`, run, keeping each whole output:
     - `python3 -I check-request-ids.py requests`: EXPECT `census: ok 4, skipped 4, FAIL 0`;
     - `python3 -I check-relay-keep.py relay-keep.py`: EXPECT `0 failed`;
     - `python3 -I test-exchanges.py relay-keep.py check-exchanges.py`: EXPECT `0 failed`.
   - From the row folder, `python -I -X utf8 args1108.py fixed`.
   - Self-test `box.ps1` with the helper's own argument list
     (`python -I -X utf8 p1108s2.py selftest selftest-r`), as A2 sitting 2 did. EXPECT its watch
     start line, no box seen, exit 3 and no console output.

   Then **LAB-COMMON step 0** (C2):
   - Load THE COMPANY alone (Difference 4).
   - **m0**, marks: EXPECT one COMPANY row, with M 210 and no `ALTVCHID`. Any other value: record it
     and STOP.
   - Record TallyPrime's release and edition as the screen shows them.
   - **Nothing listens on port 9003.** This read changes nothing:
     `netstat -ano -p tcp | findstr ":9003"`, with its whole output kept. EXPECT no line whose local
     address ends in `:9003`. Any such line, or a netstat that does not run: record it and STOP.
     Never end the process that holds the port.

1. **START, the first reads.** Send `flags`, `ledgers` and `vouchers`, each followed by
   `check_1108.py rows`. These are the START reads.
   - `flags`: EXPECT ISINVENTORYON `Yes` and ISINTEGRATED `Yes`, else STOP.
   - `ledgers`: EXPECT `Cash` under `Cash-in-Hand`, and no `A1 Closing Stock`, `A1 Capital` or
     `A2 Expense`, else STOP.
   - `vouchers`: EXPECT no row.
   - Every GUID must begin with THE COMPANY's, else STOP.

2. **BK-0** (C5), before the item's first change, into the new empty folder
   `C:\bridge-lab\1108r\backups\bk0\`. Record its name, bytes and sha256, then one `marks`: EXPECT
   V and M equal to m0's, else STOP (STOP RULES: a mark that moves across a backup). BK-0 is the way
   back to the START book (step 8) and the fallback for a STOP.

3. **W1, W2, then W3.** For each one:
   1. `marks` (C3);
   2. send the write once;
   3. take a screenshot of the whole Tally window;
   4. run `check_1108.py import`;
   5. send `ledgers` and `diff` it against the previous one.

   EXPECT `CREATED 1` and one row ADDED with the sent name and parent. Anything else: STOP, because
   every post needs all three.

4. **S0, the seed.** Send it the same way, then `marks` and `vouchers`. `diff` that `vouchers`
   against step 1's START `vouchers`, the last `vouchers` read before it (step 3 reads only `ledgers`).
   - EXPECT `CREATED 1`, the COMPANY row now carrying an `ALTVCHID`, and one voucher ADDED:
     Cash Dr 200.00, A1 Capital Cr 200.00, 10-Apr-2025.
   - Record that ALTVCHID as **V0** and the master mark as **M1**. A2 read V0 1 and M1 213 in both
     sittings; other values are recorded, not a STOP. No ALTVCHID, or anything else: STOP.

5. **BK-R**, the row backup, by C5's procedure, into the new empty folder
   `C:\bridge-lab\1108r\backups\bkr\`. Record its name, bytes and sha256. Then send `marks`,
   `ledgers` and `vouchers`: these are the **BASELINE**. EXPECT the COMPANY row with V0 and M1 (a
   mark that moved across the backup is a STOP), the ledgers of step 3's last read, and S0 as the
   only voucher. `diff` the BASELINE `ledgers` against step 3's `ledgers` after W3, and the BASELINE
   `vouchers` against step 4's `vouchers` after S0: each shows no row ADDED, REMOVED or CHANGED,
   else STOP.

6. **The relay.** Start `python -I relay-keep.py 9003 9001 C:\bridge-lab\1108r\relay` in a
   background task of its own. It stays up for the whole item. It refuses a folder that is not
   empty.

7. **R0, R1, then R2.** For each row Rk, in that order:
   1. **xk-first**: the highest N among the relay folder's `x-N.request` files, 0 if none. Record it.
   2. A listing shows `C:\bridge-lab\1108r\agent-rk` absent. Start `bridge_mcp` as Build says, with
      that folder, and its client in `agent-run-rk`.
   3. `list_companies`: EXPECT THE COMPANY's GUID. `tally_status`, with no argument: record the
      release, licence and mode. `masters` with `kind=voucher_types`: record the Journal row's
      `series_numbering.method` and `sub_method`. EXPECT `automatic` and `auto_retain`. Any other:
      STOP; this row alters no type.
   4. **mk-start**, marks on 9001: EXPECT the COMPANY row's GUID, V and M equal to the BASELINE's.
      Anything else: STOP.
   5. `build_import_xml` with `{"company_guid": <GUID>, "vouchers": <batches/rk.json>}`, from the
      file `args1108.py build rk` writes (`calls\args-build-rk.json`). Keep the
      whole answer, and record the batch id, `voucher_count` (EXPECT 3) and every warning. A
      refusal: keep it and STOP.
   6. With the helper, `p1108s2.py rk <batch id> 3 66 <BASELINE vouchers answer>`: **mk-before**
      marks on 9001; `post_import`, with the box checked (Difference 3) and Yes; a capture of Tally;
      **mk-after** marks; `vouchers` and its `diff` against the BASELINE's; then `verify_import`
      with that batch id, once. Record each call's start and end in UTC, when the box showed and
      when Yes was clicked.
   7. Keep the post's and `verify_import`'s whole answers. Record from each:
      - `dispatch` (`state`, `response_state`, `resent`), `dispatch.response.outcome`'s counters
        and `tally_line_errors`;
      - `post_span_binding`, `error.code` and `next_step`;
      - each voucher's status, attribution, marker and next step, from `vouchers` and from
        `unverified_vouchers`;
      - `duplicates`.

      A2 found no `created` or `attempt_recorded` field in either answer; record whether each is
      present.
   8. `check_1108.py rows` on that `vouchers` answer: record each ADDED voucher's GUID, MASTERID,
      ALTERID, VOUCHERNUMBER, amount and narration. Then `check_1108.py import` on the body of the
      row's `import-N.answer`, as `body.py N` writes it (Difference 9): its counters must agree with
      the post's
      `dispatch.response.outcome`.
   9. `egress_log`: the whole answer. Close the client, so `bridge_mcp` exits by itself; record its
      exit code.
   10. **xk-last**: the highest N, as in 7.1. Run
       `python3 -I check-exchanges.py C:\bridge-lab\1108r\relay <xk-first + 1> <xk-last>` and keep
       its whole output. EXPECT exit 0, `FAIL 0`, a count of `xk-last − xk-first`, and `imports 1`.
       A FAIL or `NOT READ` is a pause (7.11).
   11. Check the row's lines of the EXPECT table below. A counter, mark, status or check that
       differs: record it, and report to the coordinator before the restore. That is a pause, not a
       STOP.
   12. **Restore BK-R** (C6). With no pause open, this is the sheet's own step; with a pause open,
       only on the coordinator's word. Then send `marks`, `ledgers` and `vouchers`. EXPECT:
       - the COMPANY row's GUID, V and M equal to the BASELINE's (V0, M1);
       - each `diff` against the BASELINE showing no row ADDED, REMOVED or CHANGED.

       Anything else = STOP.

8. **Restore BK-0** (C6), only on the coordinator's explicit RESTORE. Then send `marks`, `ledgers`
   and `vouchers`. EXPECT:
   - the COMPANY row's GUID, V and M equal to m0's (M 210, no `ALTVCHID`);
   - each `diff` against the START reads showing no row ADDED, REMOVED or CHANGED.

   Anything else = STOP.

9. LAB-COMMON C10. Then stop the relay and keep its folder whole. Record the exe's sha256 again.

## EXPECT for each row

Each EXPECT has a counter, a mark and a status. The counters are from the relay's
`import-N.answer`, read by `check_1108.py import`, and agree with the post's
`dispatch.response.outcome`. The marks are from mk-before and mk-after and the `vouchers` diff. The
statuses are from the post's answer and then `verify_import`. The ids are from `check_1108.py rows`.

| Id | Row | Counter | Mark and ids | Status |
|---|---|---|---|---|
| E-R0 | R0 | CREATED 2, EXCEPTIONS 1, every other counter 0, LASTVCHID V0 + 2; one `LINEERROR` naming `A1 Closing Stock` (recorded only: no verdict reads it) | V = V0 + 2, M = M1; two vouchers ADDED against the BASELINE: `Row 1108-P2 first` 21.00, then `Row 1108-P2 third` 23.00, each MASTERID, ALTERID and VOUCHERNUMBER one and two above S0's, in that order (A2's P2: 2 and 3) | `b1108-p2-1` and `b1108-p2-3` read `matching_content_observed` with attribution `not_established`, in the post and in `verify_import`; **neither reads `posted_verified`**; `b1108-p2-2` reads `sent_not_attributed`, with the neutral next step for an unclean answer (`agent_import_verification.rs:1001`); `post_span_binding` refused with `span_counters_not_clean` (`agent_import_span_identity.rs:189`, `:210`); `error.code` `import_reconciliation_required` (`agent_import_post.rs:2318`); `response_state` `response_not_clean` (`:2353`) |
| E-X | R0, R1, R2 | (none) | (none) | `check-exchanges.py` for the row: exit 0, `FAIL 0`, a count of `xk-last − xk-first`, and `imports 1`: the row's one import, listed `(import-M, byte-equal)`, its `import-M` files checked byte-equal to its `x-N` files by `check-exchanges.py` itself |
| E-R1 | R1 | as E-R0 | as E-R0: the voucher sent first takes no MASTERID, ALTERID or VOUCHERNUMBER | as E-R0, voucher by voucher by `bridge_txn_id` |
| E-R2 | R2 | as E-R0 | as E-R0: the voucher sent last takes none | as E-R0, voucher by voucher by `bridge_txn_id` |
| E-ORD | R1, R2 | (as E-R1, E-R2) | `check_1108.py diff` of R0's post `vouchers` answer against R1's, and against R2's, shows no row ADDED or REMOVED: the same two GUIDs, whatever the order sent. Each has the same MASTERID, ALTERID, VOUCHERNUMBER, DATE, NARRATION and entries as in R0. Any other field that differs is recorded by name: no earlier row created the same voucher twice, so which other fields stay equal is not known | (as E-R1, E-R2) |

R1's and R2's counters and ids are from reading the code and from A2, where Tally went on past a
refusal in the middle of a batch. Whether it goes on past a refusal at the start, and whether a
refusal at the end changes LASTVCHID, is what those rows measure.

## STOP RULES

- An expired artifact; a zip, `bridge-tally.mcpb` or `bridge_mcp.exe` whose sha256 is not as the
  Build line says; an exe sha256 that changes during the item.
- A file that differs from `SHA256.txt`, or `setup-r.py` not ending `setup: ok`;
  `check-request-ids.py`, `check-relay-keep.py` or `test-exchanges.py` not as step 0 EXPECTs; a
  `box.ps1` self-test not as EXPECTed.
- C7 and C8. Any dialog on Tally's screen after a write, a post or a restore: screenshot it, press
  no key, report.
- A box the helper does not pass, or any decline.
- A refusal by `build_import_xml`, or by `post_import` before an attempt is recorded.
- A raw write (W1 to W3, S0) that does not answer `CREATED 1` with every other counter 0, or a raw
  answer that does not parse or lacks a counter.
- A Journal type that is not `automatic` with `auto_retain`.
- A marks answer with other than one COMPANY row; a GUID that does not begin with THE COMPANY's.
- A mark that moves in a step that sends no write, no post and no restore, a backup step included
  (BK-0 and BK-R). LAB-COMMON C5 only records a mark that moves across a backup; this sheet is
  stricter there, and the stricter rule holds. mk-start that differs from the BASELINE.
- A row's data folder that is not absent before its row.
- The reads after a restore that are not as its step EXPECTs.
- Any call with no answer in 10 minutes.
- Education mode.

After a stop, nothing more is sent or keyed (C8). The book then holds what was written before it.
The way back is BK-R to the BASELINE book and BK-0 to the START book, each restored only on the
coordinator's word.

## What the result decides

Recorded by the coordinator, each with its confidence.

| What the row shows | Reading |
|---|---|
| E-R0 and E-X as written | The readback of a partial landing through `post_import` exists as bytes: Bridge's own mark and span reads, with the import's request and answer, for one run. That meets the 26 Oct readback condition, subject to the substitutions under Evidence. |
| E-R1, E-R2 and E-ORD as written | For one Stock-in-Hand refusal at each of the three positions, the refused voucher takes no id or number, the voucher mark steps by exactly `CREATED`, and the created rows are the same rows whatever the order sent. That is what the proposed span exclusion needs for one refusal (partial: one run each, one fault kind). |
| R1 or R2 with CREATED below 2 | Tally does not go on past a refusal at that position. Then EXCEPTIONS is not sent − CREATED, and the proposed rule declines to name: record it as the limit of that rule. |
| A created voucher whose ids or number skip one, or a mark step other than `CREATED` | The refused voucher took an id at that position. The proposed rule's mark condition declines there: record which position. |
| A status that differs by position | The current code reads the order of the batch. Keep everything; the code is next. |
| E-X not met for a row | That row's readback is not whole and does not meet the 26 Oct condition. A rerun needs a new sheet. |
| E-R0 differs from A2's P2 | P2's result did not repeat. Report before R1. |

## Not measured by this row

- two refusals in one batch;
- a class of twins split between created and refused (the 3 Oct twin test), or a refusal for a
  duplicate voucher number;
- any fault other than the Stock-in-Hand Journal;
- a two-user Tally;
- a batch of more than three;
- `acknowledge_post_review` on any of these batches;
- TallyPrime 7.1.

## Evidence

On a new branch, `lab/1108-r-rows-answers`, with `SHA256.txt`:
- every raw answer whole, with its size and sha256, and every `check_1108.py` output;
- the relay folder whole: `relay.jsonl`, every `x-N.request` and `.answer`, every `import-N.request`
  and `.answer`, and each row's xk-first and xk-last;
- each row's `check-exchanges.py` output whole;
- every tool answer whole, with its size and sha256;
- each post's times and the box helper's log;
- BK-0's and BK-R's names, bytes and sha256, and each `restore-plan.txt`;
- each row's data folder by file name, size and sha256 (the files stay on the lab PC, as A2's did);
- a RESULTS.md with the EXPECT table filled in. Each Id is marked confirmed, refuted or not
  reached, and its first line names any pause or stop;
- a notes file with UTC times, one line per step.

The evidence shows THE COMPANY's GUID as `<company guid>` and its number as `<company number>`.
`substitutions.txt` names each change, with the original's size and sha256, as A2's did. Whether
THE COMPANY's GUID can be registered as synthetic, so that a fixture keeps every byte as captured,
is the coordinator's question, not this row's. Screens stay on the lab PC, listed by name, size and
sha256. Send the sha to the coordinator before pushing, and push only after the pre-push scan and
the coordinator's yes.
