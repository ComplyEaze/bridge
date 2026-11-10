# Row 2399-R: a repeated `vouchers` call reused the census once and saved 6 requests; R3 (680.2 s), each move of a mark and the restore each made it count again

Run on 10 Nov 2026, 14:13:01Z to 14:40:14Z (19:43 to 20:10 IST), on TallyPrime Silver: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`; the Gateway shows no release number). The sheet was `lab/2399-r-sheet` (549815f1), with all 13 files equal to its `SHA256.txt`. The book was `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own: period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, both unchanged.

The build was `bridge-tally` 0.7.0, `bridge_mcp.exe` sha256 `9d93302281841ca7829ff95fe58c0fe9c8c767c46fd64f3752fb6379a9a2dc04`, equal before the run and after it (`evidence/build.txt`, `evidence/exe-after.txt`). It came from the Build line's CI run 42787114741, at commit `d40c42041f0383487fee4e851ab11f4e44f0df65`, artifact 13750966183.

Every value below is M, measured by this run, in one run. Every read was answered, every call returned `isError` false, and no call errored.

## Deviations from the sheet

- **answer.py read the client's response object, extracted unchanged; the sheet assumed the bare JSON-RPC file.**
  - The client (`scripts/client.py`) saves each answer as `run/answers/<seq>-<label>.json`, with the JSON-RPC message under its own key `"response"` and its own fields beside it.
  - On that file, `answer.py` printed a stop line (no structuredContent object) and exit 1.
  - `scripts/ex.py` writes the `"response"` value alone to `rpc/<seq>-<label>.json`. The second client's files are written as `rpc/run2-<seq>-<label>.json`. `answer.py` was run on that file.
  - `scripts/exeq.py` shows, for every call, that the extracted file is JSON-equal to the client file's `"response"` value. The table is `evidence/ex.txt`: 11 calls, each True.
  - The same method was used for every call. It was started at once after `c1`, so that R2 could follow R1 inside the held census's 600 s, and reported to the run lead at 14:19Z. The run lead accepted it. Neither `answer.py` nor the client was changed during the run.
  - For the second client, `ex.py` and `exeq.py` were given a run-folder argument after `c8a`. The extraction itself did not change.
- **The client** is row 852-C1's lab client with one change, on the run lead's yes. After stdin closes it waits 60 s (not 30 s). If the server has not ended by then, it logs a stop line and leaves the server running, rather than ending it (`evidence/client.txt`, a one-hunk diff). Both servers ended by themselves, each with exit code 0.
- **The build's commit** comes from the artifact's API record, by the run lead's decision: neither the bundle nor the exe names a commit of its own. Three links tie it to the Build line, and each is EQUAL (`evidence/build.txt`):
  - the zip's sha256 equals GitHub's digest of artifact 13750966183;
  - the artifact belongs to run 42787114741;
  - that run's head_sha is the Build line's commit.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded by C1.
- **`s4-after`'s capture differs from `s1-gateway`'s by hash.** The window was the Gateway with no dialog, but Tally drew it in its inactive-window colours, because the window did not hold the focus. Both captures were read by eye: every text and line is the same. `s5-after` is byte-equal to `s4-after`, and `8-gateway` is byte-equal to `s1-gateway`.
- **The restore plan's text** at first named the earlier row's backup folder on its folder line. `prep-c6.sh` was copied from that row, and its echo line kept the old name.
  - Everything else in the plan used this row's `C:\lab\2399-r\backup-1`: the folder's one-file check, the BK's hash, its size and its name.
  - The path typed in Tally's Restore was this row's, and the Restore list showed this row's BK time, 10-Nov-26 (7:46 pm).
  - The corrected texts carry a line saying so. Both the first and the corrected texts stay on the lab PC, because they name the company number (`kept-local.txt`).
- **The three Bridge processes** that were running before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, on the run lead's standing word. None was called by the row, and each still ran after C10.

## Result

By the sheet's Decides, each line below is PARTIAL: one book, one build, one release, one sitting.

- **R2 as expected.** A repeated call reused the census on a real book and saved one census read, 6 requests, on a one-span book.
  - R1 (`c1`) counted its census: `census.requests` 1, S1 = sent 22.
  - R2 (`c2`), at once: `census.requests` 0, `census_reused.read_at` 14:17:49.501Z (R1's counting read), the same 11 items, S2 = sent 16. S1 minus S2: 6.
  - This does not say how often real use repeats a call.
- **R2's rows differ, or `census_reused_mismatch`:** not reached. R2's items equal R1's (`--same-items` True), and no call answered `census_reused_mismatch`.
- **R3 reuses after 660 s:** not reached. R3 (`c3`) started 680.2 s after R1's read finished, and counted again: `census.requests` 1, `census_reused` absent, the same 11 items, S3 = 22.
- **R4, R5 or R7 serves a census after its mark moved:** not reached. Each counted again: `census.requests` 1, `census_reused` absent.
  - R4 (`c4`, after the voucher import, marks 69/245) returned 12 items: R1's 11, with `BP/26-27/0399` last.
  - R5 (`c5`, after the ledger import, marks 69/246) returned the same 12.
  - R7 (`c8a`, after the restore, marks 68/245 against the held census's 69/246) returned R1's 11.
  - Each of these calls sent 22, equal to S1.
- **A mark did not move for the write:** not reached. The voucher import (`I-0399.xml`, `CREATED` 1) moved ALTVCHID from 68 to 69 and left ALTMSTID at 245. The ledger import (`I-L2399.xml`, `CREATED` 1) moved ALTMSTID from 245 to 246 and left ALTVCHID at 69.
- **R6 as the estimate says.** The empty-window check still counts for itself on a repeat.
  - E1 (`c6`): sent 34, omitted 2, failed 0, `census.requests` 1, 0 items. The answer's state was `partial`, reason `empty_uncorroborated`, with no error.
  - E2 (`c7`), at once: sent 28, `census.requests` 0, `census_reused.read_at` 14:30:59.840Z, 0 items, the same state and reason.
  - E1 minus E2: 6. E2 is above 16 and below E1.
- **A call errors:** none did.

**The store held again at the new marks.**
- `c4b`, at once after `c4`: `census_reused.read_at` 14:30:08.629Z (`c4`'s count), the same 12 items, sent 16, equal to S2.
- `c5b` (step 7), after `c6` and `c7`: reused `c5`'s count (read_at 14:30:45.407Z, at 69/246), the same 12 items, sent 16.

**The control, `c8b`.** A second client was started with the same exe and environment and a new, empty data folder (`C:\lab\2399-r\data-2`, absent before its launch). Its one call: `census.requests` 1, `census_reused` absent, the same 11 items as R1, sent 22, equal to S1. The restore put the book back.

**The book after the restore (s8).**
- Marks 68/245 (`--expect` equal), `N-0399` rows 0, `D-empty` rows 0.
- Each s8 answer's sha256 is equal to the same request's earlier answers (`evidence/s8-hashes.txt`):
  - M-marks to s1's, s2's and s4m's;
  - N-0399 to s2's;
  - D-empty to se's.

**C10.**
- The company was shut by Tally's own Shut Company (K: Company, SHut, Enter); Select Company then showed no company loaded.
- Step 0 again: one `tally.exe` owned 9001, and GET /status answered 200.
- Tally then quit by Q: Quit, with no prompt. Afterwards there was no `tally.exe` and 0 listeners on 9001.

## Each call

Times are UTC from `evidence/times-<label>.txt`. `sent`, `omitted`, `failed` and the listed kinds are from the call's `vouchers` record (`evidence/trail-<label>.txt`); census and items are from `evidence/answer-<label>.txt`.

| call | start | end | marks before (ALTVCHID/ALTMSTID) | sent | omitted | failed | listed kinds | `census.requests` | `census_reused` | items |
|---|---|---|---|---|---|---|---|---|---|---|
| `c1` R1 | 14:17:46.807 | 14:17:50.148 | 68/245 | 22 | 0 | 0 | post 14, status 8 | 1 | absent | 11 |
| `c2` R2 | 14:18:25.226 | 14:18:27.775 | 68/245 | 16 | 0 | 0 | post 10, status 6 | 0 | read_at 14:17:49.501Z | 11, equal to R1's |
| `c3` R3 | 14:29:09.748 | 14:29:12.580 | 68/245 | 22 | 0 | 0 | post 14, status 8 | 1 | absent | 11, equal to R1's |
| `c4` R4 | 14:30:05.939 | 14:30:09.198 | 69/245 | 22 | 0 | 0 | post 14, status 8 | 1 | absent | 12 (R1's and `BP/26-27/0399`) |
| `c4b` | 14:30:10.603 | 14:30:13.170 | 69/245 | 16 | 0 | 0 | post 10, status 6 | 0 | read_at 14:30:08.629Z | 12, equal to `c4`'s |
| `c5` R5 | 14:30:42.783 | 14:30:45.866 | 69/246 | 22 | 0 | 0 | post 14, status 8 | 1 | absent | 12, equal to `c4`'s |
| `c6` R6 (E1) | 14:30:57.659 | 14:31:01.465 | 69/246 | 34 | 2 | 0 | post 21, status 11 (of the 32 listed) | 1 | absent | 0 |
| `c7` (E2) | 14:31:03.292 | 14:31:06.710 | 69/246 | 28 | 0 | 0 | post 18, status 10 | 0 | read_at 14:30:59.840Z | 0 |
| `c5b` | 14:31:19.730 | 14:31:22.025 | 69/246 | 16 | 0 | 0 | post 10, status 6 | 0 | read_at 14:30:45.407Z | 12, equal to `c4`'s |
| `c8a` R7 | 14:34:25.148 | 14:34:27.990 | 68/245 | 22 | 0 | 0 | post 14, status 8 | 1 | absent | 11, equal to R1's |
| `c8b` control | 14:35:26.400 | 14:35:29.673 | 68/245 | 22 | 0 | 0 | post 14, status 8 | 1 | absent | 11, equal to R1's |

- **The items.** R1's 11 are `BP/26-27/0001` to `BP/26-27/0009`, then `BP/26-27/0011` and `BP/26-27/0010`, in that order. `c4`, `c4b`, `c5` and `c5b` add `BP/26-27/0399` last.
- **The calibration `c0`** (`list_companies` `{}`): one company, the book's GUID, `identity_state` `verified_tuple`. `trail.py --expect list_companies 4` gave sent 4, post 2 and status 2, `calibration ok`, exit 0.
- **The figures.** S1 = 22, S2 = 16, S3 = 22, S4 = 22, S5 = 22, E1 = 34, E2 = 28.
- **The clock for R3.** R1's read finished at 14:17:49.501Z (its `read_at`; the call ended 14:17:50.148Z). `c3` started at 14:29:09.748Z: 680.2 s after R1's read finished. Nothing was sent to Tally, nothing was keyed, and no other call was made in the wait.
- **The clock for R7.**
  - The census the first client held at the raised marks was `c5`'s count, read at 14:30:45.407Z and reused by `c5b`.
  - `c5b` finished at 14:31:22.025Z.
  - `c8a` started at 14:34:25.148Z: 219.7 s after the held census was read, and 183.1 s after `c5b` finished.
- **Clients.**
  - The first client (bridge_mcp pid 2668) ran from 14:17:04Z. Its stop file was written at 14:34:37.748Z; it ended with exit code 0 and was gone by 14:34:38.581Z.
  - The second client (pid 22472) ran from 14:34:57Z. Its stop file was written at 14:36:57.776Z; it ended with exit code 0 and was gone by 14:36:59.178Z.
  - Neither environment held `ENABLE_WRITES`.

## The marks at each step

All marks are for the book's GUID, as `ALTVCHID`/`ALTMSTID`.

| read | marks | other reads |
|---|---|---|
| s1 (after C1) | 68/245 | se `D-empty`: rows 0 |
| s2 | 68/245 (answer byte-equal to s1's) | `N-0399`: rows 0 |
| s4m (before the voucher import) | 68/245 | |
| s4 `I-0399.xml` | | `CREATED` 1, every other counter 0, `LASTVCHID` 45 |
| s4r | 69/245 | `N-0399`: rows 1 (`BP/26-27/0399`) |
| s5m (before the ledger import) | 69/245 | |
| s5 `I-L2399.xml` | | `CREATED` 1, every other counter 0 |
| s5r | 69/246 | |
| s8 (after the restore) | 68/245 | `N-0399`: rows 0; `D-empty`: rows 0 |

## The BK and the restore

- BK (C5): 14:15Z to 14:16Z, by Data, Backup & Restore, Backup, to `C:\lab\2399-r\backup-1`. Accept was answered y, and no other message came. The folder holds one file of 90051 B, sha256 `b3eb027797447390b66dac050072ecdfb89cd4bdbd50bdba6521b4b4bf365292` (`evidence/bk.txt`).
- The restore (C6, step 8):
  - The plan was written at 14:31:35Z, after the BK was hashed again (equal).
  - The company was shut, then restored from that folder.
  - Only the two expected prompts came: Accept = y, then the folder-exists prompt = r (Replace).
  - The book's data folder held 24 entries before the restore and 17 after. The listings stay on the lab PC.
  - The book was then loaded by C1; `8-gateway` is byte-equal to `s1-gateway`.

## The egress files

They are pushed whole:

| file | bytes | lines | sha256 |
|---|---|---|---|
| `egress/data-agent-egress.jsonl` (the first client, `C:\lab\2399-r\data`) | 42518 | 22 | `a0f248194eb562dd4ce3e0ce9a3ee06ea205c757b565cb62ac6fcee299f02cac` |
| `egress/data-2-agent-egress.jsonl` (the second client, `C:\lab\2399-r\data-2`) | 4253 | 2 | `e31da245abc5891eda7867aac014b595dcbdfd57f65334bea43f903c53b85c1e` |

After `c5b` the first file was 38266 B, sha256 `6ac27d31d5949336db29d0b67ea5d1403ad8ac15d1c3657a241c71ec383d0bd2` (`evidence/egress-c5b.txt`).

## What is pushed and what stays

- `answers/`: the raw Tally answers (`a-<step>-<request>.xml`), byte for byte.
- `answers-mcp/`: the client's files (`client/`, `client-2/`) and the extracted `rpc/` files.
  - Each has its `company_number` value replaced by `<redacted: company number>`, by a JSON parser, in `structuredContent` and in the text content's nested JSON alike.
  - Each original stays on the lab PC, listed by size and sha256 in `substitutions.txt`. The hashes in `evidence/ex.txt` and `evidence/answer-*.txt` are those of the originals.
- Kept on the lab PC, by size and sha256 in `kept-local.txt`:
  - the BK;
  - the restore plans;
  - the data-folder, backup-folder, process and port listings;
  - the first client's `tools/list` answer, whose tool descriptions hold words the pushed-text gate refuses (32 tools, `evidence/notes.txt`);
  - the two empty stop files and the two empty server stderr logs;
  - this row's stage script.
- The adjacent-number pattern matches only the book's two service ledgers inside the tool answers. Each is named `BRIDGE Svc`, then its SAC code (998313 or 998314), then its rate of 5%. Both names are already public in the repository's tests and docs.
- `SCREENSHOTS.txt` lists the screenshots by name, size and sha256. They are never pushed.
- `substitutions.txt` lists every change made to the lab's own text.

## Not in this row

As the sheet lists:
- a book of several census spans;
- a write through Bridge;
- a restored copy with the same marks and different vouchers;
- R7 with a wait between 540 and 660 s;
- screen edits;
- other windows, tools, books, builds and releases;
- how often a person repeats a call.

## Outputs, whole

### step 0: the build
```
$ step 0, the build (UTC 2026-11-10T14:11:29Z)
artifact 13750966183 unsigned-bundle-smoke-windows-latest, downloaded 2026-11-10T14:08:05Z to 14:08:30Z by gh api .../actions/artifacts/13750966183/zip, on the owner's word in the lab session
zip 60279141 B sha256 7309fad953762ccba515297492cd8e7c92225dd04c87e6c9d1d546f8833cfa3f; GitHub digest sha256:7309fad953762ccba515297492cd8e7c92225dd04c87e6c9d1d546f8833cfa3f: EQUAL
zip entries: bridge-tally.mcpb, mcpb-smoke.json, msi/Bridge_0.7.0_x64_en-US.msi, nsis/Bridge_0.7.0_x64-setup.exe
bridge-tally.mcpb sha256 f62420a55db22ed426ac90f575199568fccd7bcb23d24d598eaf915ef4315aef; mcpb-smoke.json archive_sha256 f62420a55db22ed426ac90f575199568fccd7bcb23d24d598eaf915ef4315aef
bridge_mcp.exe 35650560 B sha256 9d93302281841ca7829ff95fe58c0fe9c8c767c46fd64f3752fb6379a9a2dc04; mcpb-smoke.json binary_sha256 9d93302281841ca7829ff95fe58c0fe9c8c767c46fd64f3752fb6379a9a2dc04
pdfium.dll sha256 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118; mcpb-smoke.json pdfium_library_sha256 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118
manifest.json version 0.7.0; it names no commit. The exe holds no 40-hex commit of this repository as text (its one frequent 40-hex string is the Rust toolchain's, in /rustc/<hash>/ paths).
the artifact's workflow_run (API): head_sha d40c42041f0383487fee4e851ab11f4e44f0df65, head_branch fix/2399-reuse-vouchers-census; run 42787114741: workflow_dispatch, completed, success
The commit, by the run lead's decision (10 Nov): the bundle and the exe carry no commit of their own. The commit comes from the artifact's API record, tied by the digest, in three links:
  1. the zip's sha256 7309fad953762ccba515297492cd8e7c92225dd04c87e6c9d1d546f8833cfa3f equals GitHub's digest of artifact 13750966183: EQUAL;
  2. artifact 13750966183 belongs to run 42787114741 (its workflow_run.id): EQUAL;
  3. run 42787114741's head_sha d40c42041f0383487fee4e851ab11f4e44f0df65 is the Build line's commit: EQUAL.
```

### step 0: the client
```
$ the row's client: run/client.py, a copy of 852c1/runA/client.py whose only change is the wait after stdin closes (the run lead's decision, 10 Nov)
852c1/runA/client.py sha256 0ed0bb03bafb4f7999fa69756ef1e529d0157d826dbec58110bb96e9a4075d63
run/client.py sha256 1e6b490f10d2534a14d3c14d913815a6ba127bdb79da712a2be8d9d25186e324
$ diff -u 852c1/runA/client.py run/client.py
--- client.py	2026-11-03 14:36:40.898306900 +0530
+++ client.py	2026-11-10 19:42:14.052679500 +0530
@@ -103,10 +103,9 @@
     log(f'{fn} tool={req["tool"]} secs={secs:.3f} isError={r.get("isError")} error={res.get("error")} saved_sha256={sha_of(out)}')
 p.stdin.close()
 try:
-    p.wait(timeout=30)
-except subprocess.TimeoutExpired:
-    log('server did not exit within 30 s of stdin close; killed')
-    p.kill()
-    p.wait()
+    p.wait(timeout=60)
+except subprocess.TimeoutExpired:  # row 2399-R step 8: never ended by the lab; the operator stops and reports
+    log('stop: server did not exit within 60 s of stdin close; left running, not ended')
+    sys.exit(3)
 stderr_file.close()
 log(f'session ended; server pid {p.pid} exit code {p.returncode}')
hunks: 1
```

### step 1: the book's data folder and the excluded GUID
```
BRIDGE PILOT LAB: folder <N>, GUID hits 2, excluded GUID hits 0
folders named so: 1
```

### s1: marks against the sheet (--expect)
```
$ lab.py marks a-s1-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85 --expect <ALTVCHID 68> <ALTMSTID 245>
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 68 ALTMSTID 245
marks equal
exit 0
```

### step 3: the first data folder before the launch
```
ls: cannot access '/c/lab/2399-r/data': No such file or directory
```

### s1: lab.py
```
$ lab.py marks a-s1-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 68 ALTMSTID 245
exit 0
```

### se: lab.py
```
$ lab.py rows a-se-D-empty.xml
rows 0
exit 0
```

### s2: lab.py
```
$ lab.py marks a-s2-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 68 ALTMSTID 245
exit 0
$ lab.py rows a-s2-N-0399.xml
rows 0
exit 0
```

### s4m: lab.py
```
$ lab.py marks a-s4m-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 68 ALTMSTID 245
exit 0
```

### s4m: marks against the sheet (--expect)
```
$ lab.py marks a-s4m-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85 --expect <ALTVCHID 68> <ALTMSTID 245>
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 68 ALTMSTID 245
marks equal
exit 0
```

### s4: lab.py
```
$ lab.py counters a-s4-I-0399.xml
CREATED 1 ALTERED 0 DELETED 0 COMBINED 0 IGNORED 0 ERRORS 0 CANCELLED 0 EXCEPTIONS 0 LASTVCHID 45 LASTMID 0
exit 0
```

### s4r: lab.py
```
$ lab.py marks a-s4r-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 69 ALTMSTID 245
exit 0
$ lab.py rows a-s4r-N-0399.xml
rows 1
VOUCHER {'VOUCHERNUMBER': 'BP/26-27/0399', 'DATE': '20260801'}
exit 0
```

### s5m: lab.py
```
$ lab.py marks a-s5m-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 69 ALTMSTID 245
exit 0
```

### s5m: marks against the sheet (--expect)
```
$ lab.py marks a-s5m-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85 --expect <ALTVCHID 69> <ALTMSTID 245>
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 69 ALTMSTID 245
marks equal
exit 0
```

### s5: lab.py
```
$ lab.py counters a-s5-I-L2399.xml
CREATED 1 ALTERED 0 DELETED 0 COMBINED 0 IGNORED 0 ERRORS 0 CANCELLED 0 EXCEPTIONS 0 LASTVCHID 0 LASTMID 0
exit 0
```

### s5r: lab.py
```
$ lab.py marks a-s5r-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 69 ALTMSTID 246
exit 0
```

### s8: lab.py
```
$ lab.py marks a-s8-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 68 ALTMSTID 245
exit 0
$ lab.py rows a-s8-N-0399.xml
rows 0
exit 0
$ lab.py rows a-s8-D-empty.xml
rows 0
exit 0
```

### s8: marks against the sheet (--expect)
```
$ lab.py marks a-s8-M-marks.xml --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85 --expect <ALTVCHID 68> <ALTMSTID 245>
COMPANY 6b43e498-430c-4d5c-bfef-d32e2ab93c85 ALTVCHID 68 ALTMSTID 245
marks equal
exit 0
```

### c0: the first data folder after c0
```
/c/lab/2399-r/data/agent-egress.jsonl
```

### c0: trail.py
```
$ trail.py <egress> --expect list_companies 4
line 1	2026-11-10T14:17:35.740Z	list_companies	36833f58-c35e-49fa-9dc4-1692321f323b	sent 4	omitted 0	failed 0	listed 4 (post 2, status 2)	listed request bytes 5048	listed response bytes 8102
records printed 1
lines read 2; records by type {'response_prepared': 1, 'stdio_write_completed': 1}
calibration ok: the last list_companies record has sent 4
exit 0
```

### c1: start and end, UTC
```
c1 start 2026-11-10T14:17:46.807Z end 2026-11-10T14:17:50.148Z
```

### c1: trail.py
```
line 1	2026-11-10T14:17:35.740Z	list_companies	36833f58-c35e-49fa-9dc4-1692321f323b	sent 4	omitted 0	failed 0	listed 4 (post 2, status 2)	listed request bytes 5048	listed response bytes 8102
line 3	2026-11-10T14:17:49.509Z	vouchers	4bb37c61-3b32-47af-8c74-a2989167147f	sent 22	omitted 0	failed 0	listed 22 (post 14, status 8)	listed request bytes 28664	listed response bytes 1718688
records printed 2
lines read 4; records by type {'response_prepared': 2, 'stdio_write_completed': 2}
exit 0
```

### c1: answer.py
```
rpc/002-c1.json: 21078 bytes, sha256 6e7b7e020149cc8cb5127242f4c9d0b919290ec2ff5ad35367dc6ed53697c124
census.requests 1	census.ms 627
census_reused absent
window parts 1
items 11: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010
exit 0
```

### c2: start and end, UTC
```
c2 start 2026-11-10T14:18:25.226Z end 2026-11-10T14:18:27.775Z
```

### c2: trail.py
```
line 5	2026-11-10T14:18:27.213Z	vouchers	b7be3083-4d2f-4f56-b427-489c1a7d5e69	sent 16	omitted 0	failed 0	listed 16 (post 10, status 6)	listed request bytes 20576	listed response bytes 1650958
records printed 1
lines read 6; records by type {'response_prepared': 3, 'stdio_write_completed': 3}
exit 0
```

### c2: answer.py
```
rpc/003-c2.json: 21193 bytes, sha256 d1634c696bfe8b84c40b8ba4fefb9849a273045280a75d711c3700784758f92d
census.requests 0	census.ms 0
census_reused {"read_at": "2026-11-10T14:17:49.501Z"}
window parts 1
items 11: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010
exit 0
$ answer.py --same-items rpc/002-c1.json rpc/003-c2.json
items equal: True (11 and 11 items)
exit 0
```

### c3: start and end, UTC
```
c3 start 2026-11-10T14:29:09.748Z end 2026-11-10T14:29:12.580Z; R1 read finished 2026-11-10T14:17:49.501Z (its read_at); seconds between: 680.2
```

### c3: trail.py
```
line 7	2026-11-10T14:29:12.082Z	vouchers	a7b4820c-ec32-4a9b-8d55-8a77e7d6c581	sent 22	omitted 0	failed 0	listed 22 (post 14, status 8)	listed request bytes 28664	listed response bytes 1718688
records printed 1
lines read 8; records by type {'response_prepared': 4, 'stdio_write_completed': 4}
exit 0
```

### c3: answer.py
```
rpc/004-c3.json: 21078 bytes, sha256 f52d37b3755f979b3dfff76a04c52ad386304c968810987e9357c016207dfd81
census.requests 1	census.ms 583
census_reused absent
window parts 1
items 11: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010
exit 0
$ answer.py --same-items rpc/002-c1.json rpc/004-c3.json
items equal: True (11 and 11 items)
exit 0
```

### c4: start and end, UTC
```
c4 start 2026-11-10T14:30:05.939Z end 2026-11-10T14:30:09.198Z
```

### c4: trail.py
```
line 9	2026-11-10T14:30:08.636Z	vouchers	11b8969b-74c7-4a3a-b5c3-f080e8a14027	sent 22	omitted 0	failed 0	listed 22 (post 14, status 8)	listed request bytes 28664	listed response bytes 1904752
records printed 1
lines read 10; records by type {'response_prepared': 5, 'stdio_write_completed': 5}
exit 0
```

### c4: answer.py
```
rpc/005-c4.json: 23147 bytes, sha256 8cbfad023c5bb7049e06b6c54de15da8fea751c611b8f8d50e412cfb2dbc7dc8
census.requests 1	census.ms 611
census_reused absent
window parts 1
items 12: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010, BP/26-27/0399
exit 0
```

### c4b: start and end, UTC
```
c4b start 2026-11-10T14:30:10.603Z end 2026-11-10T14:30:13.170Z
```

### c4b: trail.py
```
line 11	2026-11-10T14:30:12.693Z	vouchers	731fe951-db23-4bd8-9d66-44abed71513b	sent 16	omitted 0	failed 0	listed 16 (post 10, status 6)	listed request bytes 20576	listed response bytes 1832134
records printed 1
lines read 12; records by type {'response_prepared': 6, 'stdio_write_completed': 6}
exit 0
```

### c4b: answer.py
```
rpc/006-c4b.json: 23262 bytes, sha256 0f886ea2f6cea9f240a201e976c63ddfee0184f861d1a43b9dd0d960f3ed1cc7
census.requests 0	census.ms 0
census_reused {"read_at": "2026-11-10T14:30:08.629Z"}
window parts 1
items 12: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010, BP/26-27/0399
exit 0
$ answer.py --same-items rpc/005-c4.json rpc/006-c4b.json
items equal: True (12 and 12 items)
exit 0
```

### c5: start and end, UTC
```
c5 start 2026-11-10T14:30:42.783Z end 2026-11-10T14:30:45.866Z
```

### c5: trail.py
```
line 13	2026-11-10T14:30:45.418Z	vouchers	b480dca6-7290-4820-8200-ec2e138b8ae1	sent 22	omitted 0	failed 0	listed 22 (post 14, status 8)	listed request bytes 28664	listed response bytes 1904752
records printed 1
lines read 14; records by type {'response_prepared': 7, 'stdio_write_completed': 7}
exit 0
```

### c5: answer.py
```
rpc/007-c5.json: 23147 bytes, sha256 360a2fe3c44e27f932b1fc57278b8fa421c209e37fc10823ea1545a75e73e8ca
census.requests 1	census.ms 677
census_reused absent
window parts 1
items 12: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010, BP/26-27/0399
exit 0
$ answer.py --same-items rpc/005-c4.json rpc/007-c5.json
items equal: True (12 and 12 items)
exit 0
```

### c6: start and end, UTC
```
c6 start 2026-11-10T14:30:57.659Z end 2026-11-10T14:31:01.465Z
```

### c6: trail.py
```
line 15	2026-11-10T14:31:00.953Z	vouchers	d3255f95-2483-4e09-8c2b-5c6b6d986b7d	sent 34	omitted 2	failed 0	listed 32 (post 21, status 11)	listed request bytes 41228	listed response bytes 76311
records printed 1
lines read 16; records by type {'response_prepared': 8, 'stdio_write_completed': 8}
exit 0
```

### c6: answer.py
```
rpc/008-c6.json: 2080 bytes, sha256 c3ccb83cd8d18e2469da3332eddf5f08f242b7e370563f689e003025600c75ee
census.requests 1	census.ms 535
census_reused absent
window parts 1
items 0: 
exit 0
```

### c7: start and end, UTC
```
c7 start 2026-11-10T14:31:03.292Z end 2026-11-10T14:31:06.710Z
```

### c7: trail.py
```
line 17	2026-11-10T14:31:06.273Z	vouchers	b2dd9018-fff1-4095-b456-384f269385fa	sent 28	omitted 0	failed 0	listed 28 (post 18, status 10)	listed request bytes 35664	listed response bytes 66210
records printed 1
lines read 18; records by type {'response_prepared': 9, 'stdio_write_completed': 9}
exit 0
```

### c7: answer.py
```
rpc/009-c7.json: 2195 bytes, sha256 8295e579f8a7d023a379c82913d0b10aa555707b6509d645dc5a0055d32da353
census.requests 0	census.ms 0
census_reused {"read_at": "2026-11-10T14:30:59.840Z"}
window parts 1
items 0: 
exit 0
```

### c5b: start and end, UTC
```
c5b start 2026-11-10T14:31:19.730Z end 2026-11-10T14:31:22.025Z
```

### c5b: trail.py
```
line 19	2026-11-10T14:31:21.514Z	vouchers	48d314bb-6601-4d7e-95a5-27f5922bd31a	sent 16	omitted 0	failed 0	listed 16 (post 10, status 6)	listed request bytes 20576	listed response bytes 1832134
records printed 1
lines read 20; records by type {'response_prepared': 10, 'stdio_write_completed': 10}
exit 0
```

### c5b: answer.py
```
rpc/010-c5b.json: 23263 bytes, sha256 dc180809cab65609c20a1079f601aa41ccb6e04dc74fac30c33e59c5a8d64983
census.requests 0	census.ms 0
census_reused {"read_at": "2026-11-10T14:30:45.407Z"}
window parts 1
items 12: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010, BP/26-27/0399
exit 0
```

### c5b: the egress file after it
```
egress after c5b: 38266 B sha256 6ac27d31d5949336db29d0b67ea5d1403ad8ac15d1c3657a241c71ec383d0bd2 at 2026-11-10T14:31:23Z
```

### c8a: start and end, UTC
```
c8a start 2026-11-10T14:34:25.148Z end 2026-11-10T14:34:27.990Z; the held census (c5 count, kept by c5b) read 2026-11-10T14:30:45.407Z; seconds between: 219.7
```

### c8a: trail.py
```
line 21	2026-11-10T14:34:27.528Z	vouchers	49b1e4c9-e8fd-47ba-80e7-7ada2ba28650	sent 22	omitted 0	failed 0	listed 22 (post 14, status 8)	listed request bytes 28664	listed response bytes 1718688
records printed 1
lines read 22; records by type {'response_prepared': 11, 'stdio_write_completed': 11}
exit 0
```

### c8a: answer.py
```
rpc/011-c8a.json: 21079 bytes, sha256 37efd12810006b19868677fc7fb2d56cbeb56e570f4ab6b8e80a0b9101fc2196
census.requests 1	census.ms 632
census_reused absent
window parts 1
items 11: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010
exit 0
$ answer.py --same-items rpc/002-c1.json rpc/011-c8a.json
items equal: True (11 and 11 items)
exit 0
```

### s8: every answer against the earlier answers to the same request; the egress files
```
Step 8 answer hashes: each s8 answer against the same request's earlier answers
a-s8-M-marks.xml 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 | a-s1-M-marks.xml 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 | EQUAL
a-s8-M-marks.xml 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 | a-s2-M-marks.xml 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 | EQUAL
a-s8-M-marks.xml 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 | a-s4m-M-marks.xml 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 | EQUAL
a-s8-N-0399.xml 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | a-s2-N-0399.xml 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | EQUAL
a-s8-D-empty.xml 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | a-se-D-empty.xml 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | EQUAL

Egress files, kept whole:
42518 B, lines=22, sha256 a0f248194eb562dd4ce3e0ce9a3ee06ea205c757b565cb62ac6fcee299f02cac  /c/lab/2399-r/data/agent-egress.jsonl
4253 B, lines=2, sha256 e31da245abc5891eda7867aac014b595dcbdfd57f65334bea43f903c53b85c1e  /c/lab/2399-r/data-2/agent-egress.jsonl
```

### c8b: start and end, UTC
```
c8b start 2026-11-10T14:35:26.400Z end 2026-11-10T14:35:29.673Z (second client, data-2)
```

### c8b: trail.py
```
line 1	2026-11-10T14:35:29.179Z	vouchers	d33e96dd-c265-4a92-a0ef-1f07866f7a4d	sent 22	omitted 0	failed 0	listed 22 (post 14, status 8)	listed request bytes 28664	listed response bytes 1718688
records printed 1
lines read 2; records by type {'response_prepared': 1, 'stdio_write_completed': 1}
exit 0
```

### c8b: answer.py
```
rpc/run2-000-c8b.json: 21078 bytes, sha256 f21aa8df00e9da093820d81a077770854014aa6443729ccb67776e69c025ddf5
census.requests 1	census.ms 616
census_reused absent
window parts 1
items 11: BP/26-27/0001, BP/26-27/0002, BP/26-27/0003, BP/26-27/0004, BP/26-27/0005, BP/26-27/0006, BP/26-27/0007, BP/26-27/0008, BP/26-27/0009, BP/26-27/0011, BP/26-27/0010
exit 0
$ answer.py --same-items rpc/002-c1.json rpc/run2-000-c8b.json
items equal: True (11 and 11 items)
exit 0
```

### c8b: the second data folder before the launch
```
ls: cannot access '/c/lab/2399-r/data-2': No such file or directory
```

### c8b: the second data folder after it
```
/c/lab/2399-r/data-2/agent-egress.jsonl
```

### c8b: the second client closed
```
stop file written 2026-11-10T14:36:57.776Z
pid 22472 gone by 2026-11-10T14:36:59.178Z
```

### every call: the client file and the extracted file, and their JSON-equality
```
002-c1.json: client file sha256 3ff12687df4b4c00f5e90ba9ff3fc253b44dbc71db9bece0e9f7ae438b46efe0; rpc/002-c1.json 21078 B sha256 6e7b7e020149cc8cb5127242f4c9d0b919290ec2ff5ad35367dc6ed53697c124
003-c2.json: client file sha256 ce4182145ae5399223765dbc189a151490fdf6a68404a5e104e321486faaecd7; rpc/003-c2.json 21193 B sha256 d1634c696bfe8b84c40b8ba4fefb9849a273045280a75d711c3700784758f92d
002-c1.json: rpc/002-c1.json JSON-equal to the client file's "response" value: True
003-c2.json: rpc/003-c2.json JSON-equal to the client file's "response" value: True
004-c3.json: client file sha256 8373419226b56ce606311e976fc950b0100e1286257c7e31a588160e2d10cf98; rpc/004-c3.json 21078 B sha256 f52d37b3755f979b3dfff76a04c52ad386304c968810987e9357c016207dfd81
004-c3.json: rpc/004-c3.json JSON-equal to the client file's "response" value: True
005-c4.json: client file sha256 86f16f95904dea0aa71c6a92486f1347c34b1cb7d564d69a4c28a88bca7817ab; rpc/005-c4.json 23147 B sha256 8cbfad023c5bb7049e06b6c54de15da8fea751c611b8f8d50e412cfb2dbc7dc8
005-c4.json: rpc/005-c4.json JSON-equal to the client file's "response" value: True
006-c4b.json: client file sha256 d191edcc60e175dfcb0eecb8e977b00e8950c65a276cbfefa6c0bb2756f5e6e7; rpc/006-c4b.json 23262 B sha256 0f886ea2f6cea9f240a201e976c63ddfee0184f861d1a43b9dd0d960f3ed1cc7
006-c4b.json: rpc/006-c4b.json JSON-equal to the client file's "response" value: True
007-c5.json: client file sha256 27bc6c03c09db9ef05461e39d7854749c1d854850a8bb9fcc2010b7cdcf0b5f1; rpc/007-c5.json 23147 B sha256 360a2fe3c44e27f932b1fc57278b8fa421c209e37fc10823ea1545a75e73e8ca
007-c5.json: rpc/007-c5.json JSON-equal to the client file's "response" value: True
008-c6.json: client file sha256 1214e202432bc403c5c1e1e77a2e2a7eff6d4fed4c8915b43f341a5581f3721b; rpc/008-c6.json 2080 B sha256 c3ccb83cd8d18e2469da3332eddf5f08f242b7e370563f689e003025600c75ee
008-c6.json: rpc/008-c6.json JSON-equal to the client file's "response" value: True
009-c7.json: client file sha256 05254172f0e3c20786602903b05e2deec08b3b85e4f041debb22529cae43003c; rpc/009-c7.json 2195 B sha256 8295e579f8a7d023a379c82913d0b10aa555707b6509d645dc5a0055d32da353
009-c7.json: rpc/009-c7.json JSON-equal to the client file's "response" value: True
010-c5b.json: client file sha256 258d5b8320f9486436e4274d632556ec8f97b40ec238e13cf6b47816aa81abb3; rpc/010-c5b.json 23263 B sha256 dc180809cab65609c20a1079f601aa41ccb6e04dc74fac30c33e59c5a8d64983
010-c5b.json: rpc/010-c5b.json JSON-equal to the client file's "response" value: True
011-c8a.json: client file sha256 260fdcd6397437f4a4f2d4b2f4e341d1f898eece2e6c0b754bc89b4a61f6cef4; rpc/011-c8a.json 21079 B sha256 37efd12810006b19868677fc7fb2d56cbeb56e570f4ab6b8e80a0b9101fc2196
011-c8a.json: rpc/011-c8a.json JSON-equal to the client file's "response" value: True
run2/answers/000-c8b.json: client file sha256 4c94a64170e4648c8311b61341dfddf55e8eee074ef842036d76f10f195226a0; rpc/run2-000-c8b.json 21078 B sha256 f21aa8df00e9da093820d81a077770854014aa6443729ccb67776e69c025ddf5
000-c8b.json: rpc/000-c8b.json JSON-equal to the client file's "response" value: True
```

### the exe after the run
```
$ the exe the clients launched, hashed again after C10 (both clients ended)
bridge_mcp.exe 35650560 B sha256 9d93302281841ca7829ff95fe58c0fe9c8c767c46fd64f3752fb6379a9a2dc04 at 2026-11-10T14:42:33Z
before the run (evidence/build.txt, mcpb-smoke.json): 9d93302281841ca7829ff95fe58c0fe9c8c767c46fd64f3752fb6379a9a2dc04; each client checked it at launch (run/client-log.txt, run2/client-log.txt): ok
after equal to before: EQUAL
```

### the program file of the Tally that answered
```
$ the program file of the Tally that answered (read after C10, with Tally closed)
tally.exe file version 1.1.7.2; 18485952 B; sha256 856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73; last written 2026-10-25T05:34:45Z
```

### size and sha256 of every answer
```
$ size and sha256 of every answer as saved on the lab PC (raw Tally answers; the client files; the extracted rpc files; the egress files)
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-N-0399.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s4-I-0399.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s4m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s4r-M-marks.xml 3396 B 8feeba3e89f1c0940a4781d39da6b9ea59e8885d0259302ae03e64ca2d06b8b5
a-s4r-N-0399.xml 5608 B 6cb167e8e15d6b5cf1d7b3e532c47b018600cf8a03968b21e24d49eb29bf80a0
a-s5-I-L2399.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s5m-M-marks.xml 3396 B 8feeba3e89f1c0940a4781d39da6b9ea59e8885d0259302ae03e64ca2d06b8b5
a-s5r-M-marks.xml 3396 B 17c5bde53bf5e459f4ae36fce8ee62e8bfe5eb636f47f1970faff44c230a59ed
a-s8-D-empty.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s8-N-0399.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-se-D-empty.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
run/answers/000-tools.json 170970 B 23b5bd98223bfa9f46908f44aac4960093af203215454ed9782a36279e70e7fc
run/answers/001-c0.json 1566 B 80b5cfa5813a568b1c517c51414a1e428ae5edc3adcffbf332ce52f883833619
run/answers/002-c1.json 21376 B 3ff12687df4b4c00f5e90ba9ff3fc253b44dbc71db9bece0e9f7ae438b46efe0
run/answers/003-c2.json 21491 B ce4182145ae5399223765dbc189a151490fdf6a68404a5e104e321486faaecd7
run/answers/004-c3.json 21376 B 8373419226b56ce606311e976fc950b0100e1286257c7e31a588160e2d10cf98
run/answers/005-c4.json 23444 B 86f16f95904dea0aa71c6a92486f1347c34b1cb7d564d69a4c28a88bca7817ab
run/answers/006-c4b.json 23560 B d191edcc60e175dfcb0eecb8e977b00e8950c65a276cbfefa6c0bb2756f5e6e7
run/answers/007-c5.json 23445 B 27bc6c03c09db9ef05461e39d7854749c1d854850a8bb9fcc2010b7cdcf0b5f1
run/answers/008-c6.json 2378 B 1214e202432bc403c5c1e1e77a2e2a7eff6d4fed4c8915b43f341a5581f3721b
run/answers/009-c7.json 2493 B 05254172f0e3c20786602903b05e2deec08b3b85e4f041debb22529cae43003c
run/answers/010-c5b.json 23561 B 258d5b8320f9486436e4274d632556ec8f97b40ec238e13cf6b47816aa81abb3
run/answers/011-c8a.json 21376 B 260fdcd6397437f4a4f2d4b2f4e341d1f898eece2e6c0b754bc89b4a61f6cef4
run2/answers/000-c8b.json 21377 B 4c94a64170e4648c8311b61341dfddf55e8eee074ef842036d76f10f195226a0
rpc/002-c1.json 21078 B 6e7b7e020149cc8cb5127242f4c9d0b919290ec2ff5ad35367dc6ed53697c124
rpc/003-c2.json 21193 B d1634c696bfe8b84c40b8ba4fefb9849a273045280a75d711c3700784758f92d
rpc/004-c3.json 21078 B f52d37b3755f979b3dfff76a04c52ad386304c968810987e9357c016207dfd81
rpc/005-c4.json 23147 B 8cbfad023c5bb7049e06b6c54de15da8fea751c611b8f8d50e412cfb2dbc7dc8
rpc/006-c4b.json 23262 B 0f886ea2f6cea9f240a201e976c63ddfee0184f861d1a43b9dd0d960f3ed1cc7
rpc/007-c5.json 23147 B 360a2fe3c44e27f932b1fc57278b8fa421c209e37fc10823ea1545a75e73e8ca
rpc/008-c6.json 2080 B c3ccb83cd8d18e2469da3332eddf5f08f242b7e370563f689e003025600c75ee
rpc/009-c7.json 2195 B 8295e579f8a7d023a379c82913d0b10aa555707b6509d645dc5a0055d32da353
rpc/010-c5b.json 23263 B dc180809cab65609c20a1079f601aa41ccb6e04dc74fac30c33e59c5a8d64983
rpc/011-c8a.json 21079 B 37efd12810006b19868677fc7fb2d56cbeb56e570f4ab6b8e80a0b9101fc2196
rpc/run2-000-c8b.json 21078 B f21aa8df00e9da093820d81a077770854014aa6443729ccb67776e69c025ddf5
C:/lab/2399-r/data/agent-egress.jsonl 42518 B a0f248194eb562dd4ce3e0ce9a3ee06ea205c757b565cb62ac6fcee299f02cac
C:/lab/2399-r/data-2/agent-egress.jsonl 4253 B e31da245abc5891eda7867aac014b595dcbdfd57f65334bea43f903c53b85c1e
```

### each raw send's start and answer, UTC
```
2026-11-10T14:14:24.893Z s1 M-marks.xml (hashes equal to the list)
2026-11-10T14:14:25.414Z s1 answered
2026-11-10T14:14:33.352Z se D-empty.xml (hashes equal to the list)
2026-11-10T14:14:36.227Z se answered
2026-11-10T14:16:47.600Z s2 M-marks.xml N-0399.xml (hashes equal to the list)
2026-11-10T14:16:48.127Z s2 answered
2026-11-10T14:29:20.840Z s4m M-marks.xml (hashes equal to the list)
2026-11-10T14:29:21.052Z s4m answered
2026-11-10T14:29:27.074Z s4 I-0399.xml (hashes equal to the list)
2026-11-10T14:29:28.945Z s4 answered
2026-11-10T14:29:56.181Z s4r M-marks.xml N-0399.xml (hashes equal to the list)
2026-11-10T14:29:56.578Z s4r answered
2026-11-10T14:30:22.828Z s5m M-marks.xml (hashes equal to the list)
2026-11-10T14:30:23.123Z s5m answered
2026-11-10T14:30:29.553Z s5 I-L2399.xml (hashes equal to the list)
2026-11-10T14:30:32.492Z s5 answered
2026-11-10T14:30:41.898Z s5r M-marks.xml (hashes equal to the list)
2026-11-10T14:30:42.306Z s5r answered
2026-11-10T14:34:12.730Z s8 M-marks.xml N-0399.xml D-empty.xml (hashes equal to the list)
2026-11-10T14:34:13.481Z s8 answered
```

## run.py logs (each raw send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
19:44:25	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### se
```
19:44:33	D-empty.xml	a-se-D-empty.xml	curl 0	3022 bytes
```

### s2
```
19:46:47	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
19:46:47	N-0399.xml	a-s2-N-0399.xml	curl 0	3022 bytes
```

### s4m
```
19:59:20	M-marks.xml	a-s4m-M-marks.xml	curl 0	3394 bytes
```

### s4
```
19:59:27	I-0399.xml	a-s4-I-0399.xml	curl 0	540 bytes
```

### s4r
```
19:59:56	M-marks.xml	a-s4r-M-marks.xml	curl 0	3396 bytes
19:59:56	N-0399.xml	a-s4r-N-0399.xml	curl 0	5608 bytes
```

### s5m
```
20:00:23	M-marks.xml	a-s5m-M-marks.xml	curl 0	3396 bytes
```

### s5
```
20:00:29	I-L2399.xml	a-s5-I-L2399.xml	curl 0	538 bytes
```

### s5r
```
20:00:42	M-marks.xml	a-s5r-M-marks.xml	curl 0	3396 bytes
```

### s8
```
20:04:13	M-marks.xml	a-s8-M-marks.xml	curl 0	3394 bytes
20:04:13	N-0399.xml	a-s8-N-0399.xml	curl 0	3022 bytes
20:04:13	D-empty.xml	a-s8-D-empty.xml	curl 0	3022 bytes
```

## The client logs

### run/client-log.txt
```
2026-11-10T14:17:04Z exe sha256 9d93302281841ca7829ff95fe58c0fe9c8c767c46fd64f3752fb6379a9a2dc04 ok; launch-env.json sha256 7d8676a616827dc7bdbeb6e1213612f01a6df4b3da57dfa2556a546f214f10df; keys ['BRIDGE_AGENT_DATA_DIR', 'BRIDGE_TALLY_HOST', 'BRIDGE_TALLY_PORT', 'BRIDGE_TERMS_ACCEPTED']; ENABLE_WRITES present: False
2026-11-10T14:17:04Z launched pid 2668; stderr to server-stderr-20261110T141704Z.log
2026-11-10T14:17:04Z initialized {'name': 'bridge-tally', 'version': '0.7.0'}
2026-11-10T14:17:22Z 000-tools.json tool=list secs=0.023 isError=None error=None saved_sha256=23b5bd98223bfa9f46908f44aac4960093af203215454ed9782a36279e70e7fc
2026-11-10T14:17:35Z 001-c0.json tool=list_companies secs=1.888 isError=False error=None saved_sha256=80b5cfa5813a568b1c517c51414a1e428ae5edc3adcffbf332ce52f883833619
2026-11-10T14:17:49Z 002-c1.json tool=vouchers secs=2.278 isError=False error=None saved_sha256=3ff12687df4b4c00f5e90ba9ff3fc253b44dbc71db9bece0e9f7ae438b46efe0
2026-11-10T14:18:27Z 003-c2.json tool=vouchers secs=1.522 isError=False error=None saved_sha256=ce4182145ae5399223765dbc189a151490fdf6a68404a5e104e321486faaecd7
2026-11-10T14:29:12Z 004-c3.json tool=vouchers secs=1.952 isError=False error=None saved_sha256=8373419226b56ce606311e976fc950b0100e1286257c7e31a588160e2d10cf98
2026-11-10T14:30:08Z 005-c4.json tool=vouchers secs=2.080 isError=False error=None saved_sha256=86f16f95904dea0aa71c6a92486f1347c34b1cb7d564d69a4c28a88bca7817ab
2026-11-10T14:30:12Z 006-c4b.json tool=vouchers secs=1.461 isError=False error=None saved_sha256=d191edcc60e175dfcb0eecb8e977b00e8950c65a276cbfefa6c0bb2756f5e6e7
2026-11-10T14:30:45Z 007-c5.json tool=vouchers secs=2.222 isError=False error=None saved_sha256=27bc6c03c09db9ef05461e39d7854749c1d854850a8bb9fcc2010b7cdcf0b5f1
2026-11-10T14:31:01Z 008-c6.json tool=vouchers secs=2.852 isError=False error=None saved_sha256=1214e202432bc403c5c1e1e77a2e2a7eff6d4fed4c8915b43f341a5581f3721b
2026-11-10T14:31:06Z 009-c7.json tool=vouchers secs=2.229 isError=False error=None saved_sha256=05254172f0e3c20786602903b05e2deec08b3b85e4f041debb22529cae43003c
2026-11-10T14:31:21Z 010-c5b.json tool=vouchers secs=1.527 isError=False error=None saved_sha256=258d5b8320f9486436e4274d632556ec8f97b40ec238e13cf6b47816aa81abb3
2026-11-10T14:34:27Z 011-c8a.json tool=vouchers secs=2.160 isError=False error=None saved_sha256=260fdcd6397437f4a4f2d4b2f4e341d1f898eece2e6c0b754bc89b4a61f6cef4
2026-11-10T14:34:37Z session ended; server pid 2668 exit code 0
```

### run2/client-log.txt
```
2026-11-10T14:34:57Z exe sha256 9d93302281841ca7829ff95fe58c0fe9c8c767c46fd64f3752fb6379a9a2dc04 ok; launch-env.json sha256 1f392866ad94d33edb05de795494dea9af37e7e7ce19dce06dee4b21df58ee0d; keys ['BRIDGE_AGENT_DATA_DIR', 'BRIDGE_TALLY_HOST', 'BRIDGE_TALLY_PORT', 'BRIDGE_TERMS_ACCEPTED']; ENABLE_WRITES present: False
2026-11-10T14:34:57Z launched pid 22472; stderr to server-stderr-20261110T143457Z.log
2026-11-10T14:34:57Z initialized {'name': 'bridge-tally', 'version': '0.7.0'}
2026-11-10T14:35:29Z 000-c8b.json tool=vouchers secs=2.107 isError=False error=None saved_sha256=4c94a64170e4648c8311b61341dfddf55e8eee074ef842036d76f10f195226a0
2026-11-10T14:36:57Z session ended; server pid 22472 exit code 0
```

### run/evidence/tools-log.txt
```
2026-11-10T14:17:21.872Z 000-tools.json list secs 0.023 bytes 170970 isError None
2026-11-10T14:17:33.621Z 001-c0.json list_companies secs 1.888 bytes 1566 isError False
2026-11-10T14:17:47.077Z 002-c1.json vouchers secs 2.278 bytes 21376 isError False
2026-11-10T14:18:25.488Z 003-c2.json vouchers secs 1.522 bytes 21491 isError False
2026-11-10T14:29:09.864Z 004-c3.json vouchers secs 1.952 bytes 21376 isError False
2026-11-10T14:30:06.161Z 005-c4.json vouchers secs 2.08 bytes 23444 isError False
2026-11-10T14:30:10.811Z 006-c4b.json vouchers secs 1.461 bytes 23560 isError False
2026-11-10T14:30:43.051Z 007-c5.json vouchers secs 2.222 bytes 23445 isError False
2026-11-10T14:30:57.914Z 008-c6.json vouchers secs 2.852 bytes 2378 isError False
2026-11-10T14:31:03.587Z 009-c7.json vouchers secs 2.229 bytes 2493 isError False
2026-11-10T14:31:20.013Z 010-c5b.json vouchers secs 1.527 bytes 23561 isError False
2026-11-10T14:34:25.364Z 011-c8a.json vouchers secs 2.16 bytes 21376 isError False
```

### run2/evidence/tools-log.txt
```
2026-11-10T14:35:26.661Z 000-c8b.json vouchers secs 2.107 bytes 21377 isError False
```

