# Row 2399-D: on a book whose census needs 4 spans, a repeated `vouchers` call reused the whole census and answered the same 82 rows, sending 24 fewer requests (52, then 28)

Run on 10 Nov 2026, 18:58:12Z to 19:03:22Z (00:28 to 00:33 IST, 11 Nov), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`; the Gateway shows no release number). The sheet was `lab/2399-d-sheet` (706065b1), with all 6 files equal to its `SHA256.txt`. The book was `BRIDGE CORPUS DENSE` (GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, synthetic), loaded alone in a sitting of its own: period 1-Apr-25 to 31-Mar-26, current date Tuesday, 31-Mar-2026, both unchanged at the close. READS ONLY: nothing was keyed in Tally but the load and the shut, and every request went through the tool.

The build was `bridge-tally` 0.7.0, `bridge_mcp.exe` sha256 `3f880c1e1bca963ccfbe949ee8ca61f4d6f2d8c12e53e5bee6d51a0d1adf4056`, equal before the run and after it (`evidence/build.txt`, `evidence/exe-after.txt`). It came from master's `ci.yml` push run for `d69b2ea8f71394eb7a948dcada541079e899eaa0`: the run GitHub lists for that head commit (Bridge CI, completed, success; its `head_sha` checked EQUAL), artifact `unsigned-bundle-smoke-windows-latest`, 60,401,470 B. The run's id and the artifact's id are kept on the lab PC; in the pushed `evidence/build.txt` they read `<CI run id>` and `<artifact id>`, and the zip's digest is the tie. Three links tie it, each EQUAL: the zip's sha256 equals GitHub's digest `sha256:ba5382c6afae35ecb7f94ce7dc887563cba1a2df8da8d6c75853eef0ca346e83`; the artifact belongs to that run; that run's `head_sha` is the commit above. The mcpb's sha256, the exe's and pdfium's each equal `mcpb-smoke.json`.

Every value below is M, measured by this run, in one run. Every read was answered, every call returned `isError` false, and no call errored.

## Deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command: the first `trail.py` command, as the sheet spells it, printed Windows' "Python was not found" line (exit 49) and read nothing (`evidence/trail-c0-python3-not-found.txt`). Every sheet script was then run as `python -I` (Python 3.10), as in row 2399-R. The scripts are the sheet's files, unchanged.
- **The launch configuration was written twice before the launch.** The first `launch-env.json`, written by the shell, did not parse (its backslashes were rewritten). It was written again by Python, with the same four keys as row 2399-R's, before the client started; the client read only that file (its log records its sha256 and keys).
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **C1 used the name filter.** `BRIDGE`, space, `CORPUS`, space, `DENSE` was typed in Select Company's filter, leaving one row highlighted; Enter was pressed only after a screenshot showed that row reading `BRIDGE CORPUS DENSE` (`s0-select`), as the sheet asks. No arrow key was needed.
- **The three Bridge processes** that were running before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, on the run lead's standing word. None was called by the row, and each still ran after C10.

## Result

By the sheet's Decides, PARTIAL: one book, one build, one release, one sitting, one day.

- **D2 as expected.** A repeated call reused a census of 4 spans on a real book, with the same rows, and saved one census read a span.
  - **D1 (`c1`)**, started 19:01:00Z, round trip 7.356 s: `result.total` 82, state `complete`, 82 items, `census.requests` 4 (3,096 ms), `census_reused` absent, `window parts` 2. The receipt (`response_prepared` 19:01:07.973Z): S1 = sent 52, omitted 20, failed 0; the 32 listed are post 21 and status 11. Each EXPECT met.
  - **D2 (`c2`)**, at once (started 19:01:35Z, 27 s after D1's receipt; nothing sent to Tally and nothing keyed between), round trip 3.293 s: `result.total` 82, state `complete`, 82 items, `census.requests` 0 (0 ms), `census_reused.read_at` 19:01:07.937Z, `window parts` 2. The receipt (19:01:38.704Z): S2 = sent 28, omitted 0, failed 0; listed 28, post 18 and status 10.
  - `read_at` lies inside D1's call, 0.036 s before D1's receipt: the census D2 used is D1's, held after D1's read stood.
  - `answer.py --same-items` on the two: items equal True (82 and 82). D1's and D2's items are the same 82 vouchers, numbered 3579 to 3660, in the same order (the two `answer.py` outputs below).
  - **S1 minus S2: 24**, which is 6 requests for each of D1's 4 census spans, as the sheet estimated from row 703-C's log of this call (52 and 28). Row 2399-R's one-span book saved 6 (22 and 16).
- **D2's rows differ, or `census_reused_mismatch`:** not reached.
- **D2 counts part or all of the census again:** not reached (`census.requests` 0).
- **D1 reports `census.requests` 1:** not reached (4, the spans of a voucher mark of 29900 at the census's span size).
- **A call errors:** none did.

**The marks.** `m0` before D1 and `m1` after D2 each read `master_alter_id` 328 and `voucher_alter_id` 29900 (`structuredContent.result.snapshot`), the book's last recorded state. So D2 was made at unchanged marks.

**The calibration.** `trail.py --expect list_companies 4` after `c0`: sent 4, calibration ok, exit 0. Each `ledger_masters` read sent 26. After `m1` the egress file holds 5 `response_prepared` records, two of them `vouchers`, one for each `vouchers` call.

**What this does not say.** How often real use repeats a call, a window of more parts, a later page, the TTL or a moved mark on this book (row 2399-R measured those on a one-span book).

## Step 0: build, book, Tally
- Sheet files: all 6 equal to `SHA256.txt` at 706065b1 (sizes too).
- The build as above: the run found by its head commit (two push runs on master for it; the `ci.yml` one is the run above), the artifact downloaded by `gh api` to a new folder, and its zip, mcpb, exe and pdfium hashes checked before the launch (`evidence/build.txt`).
- The client: `run/client.py`, row 2399-R's `run/client.py` byte for byte (sha256 `1e6b490f10d2534a14d3c14d913815a6ba127bdb79da712a2be8d9d25186e324`). The pushed `scripts/client.py` of `lab/2399-r-answers` is its text with the words its `substitutions.txt` lists changed (`evidence/client.txt`). This row's own copy is pushed with the same substitutions.
- The book check: one data folder whose `Company.1800` names `BRIDGE CORPUS DENSE` and holds its GUID; it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- `C:\lab\2399-d\data` was absent before the launch (`evidence/data-before-launch.txt`).
- Tally was started once, at 18:58:12Z, from its desktop shortcut, after tasklist showed none. Tally's start before this one was row 683-TN's (18:11:19Z, quit at 18:28:09Z); nothing ran on this PC's Tally between.
- C2 at 18:58:38Z, no company loaded: one `tally.exe`, which alone owns 9001; `GET /status` status=200, bytes=51.
- The book loaded alone (deviations above). The Gateway (`s0-gateway`): current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 31-Mar-2026; date of last entry 31-Mar-26. EXPECT met.

## Steps 1 to 3: the calls
The client sent one call at a time from `run/inbox`, in name order, and saved each answer whole in `run/answers` (`run/client-log.txt` below). After each `vouchers` call: `trail.py` on the egress file, `rpc.py` to the `rpc/` copy, and `answer.py` on that copy.
- Launch at 18:59:53Z: `bridge_mcp` pid 20448, `bridge-tally` 0.7.0; environment `BRIDGE_TALLY_HOST` 127.0.0.1, `BRIDGE_TALLY_PORT` 9001, `BRIDGE_TERMS_ACCEPTED` true, `BRIDGE_AGENT_DATA_DIR` `C:\lab\2399-d\data`; `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_ENABLE_WRITES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` absent; no other `BRIDGE_` variable in the shell.
- `tools/list` (19:00:04Z): 32 tools; `list_companies`, `ledger_masters` and `vouchers` listed.
- `c0` (19:00:13Z): `list_companies` `{}`: one company, the book's GUID, `identity_state` `verified_tuple`, `books_from` 20250401. After it the data folder held one `agent-egress.jsonl`.
- `m0` (19:00:47Z), `c1` (19:01:00Z), `c2` (19:01:35Z), `m1` (19:02:00Z) as in the Result.

## Step 4: close and C10
- The client closed its stdin on the stop file (19:02:10.919Z); `bridge_mcp.exe` pid 20448 ended by itself with exit code 0 (19:02:11Z) and was gone from tasklist by 19:02:13Z. Its stderr file was empty.
- `bridge_mcp.exe` re-hashed at 19:02:24Z: equal to the sha256 before the launch.
- The egress file kept whole: 19,798 B, 10 lines, sha256 `224ab370f300a47c4a4dfbd14990cd67080874ee2e615470b5ea4dcbb779fc5d`, pushed byte for byte as `egress/agent-egress.jsonl` (it holds counts, kinds, times, hashes and the book's GUID; no body, name or value). `trail.py`'s outputs below are its account of it.
- C10: the Gateway (`c10-gateway`, byte-equal to `s0-gateway`), current date unchanged; Company > Shut, BRIDGE CORPUS DENSE alone, Enter, no prompt; Select Company, none loaded (`c10-no-company`). C2 at 19:03:04Z: one `tally.exe` owning 9001; `GET /status` status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; at 19:03:22Z no `tally.exe` ran and nothing listened on 9001.

## Files
- `answers/`: none. Every tool answer (the tool list, `c0`, `m0`, `c1`, `c2`, `m1`) and its `rpc/` copy stays on the lab PC, by size and sha256 (`kept-local.txt`, and `evidence/answers-sha.txt`): the `vouchers` and `ledger_masters` answers hold party names, the tool list holds the product's tool texts, and `c0` names the company number.
- `egress/agent-egress.jsonl`: the egress file, whole.
- `answers-mcp/`: the client's inbox (each call's request), its launch configuration, its log, `server.pid` and `step0/initialize.json`.
- `evidence/`: notes; the outputs of `trail.py`, `rpc.py` and `answer.py`; the build, client and book checks; the exe's hash after the run; `tally.exe`'s file data. Process and port listings stay on the lab PC.
- `scripts/`: the sitting's helpers and the client, with lab paths replaced. `trail.py`, `rpc.py` and `answer.py` ran as the sheet's files, byte for byte, and are not repeated.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## Outputs, whole

### step 0: the build
```
$ step 0, the build (UTC 2026-11-10T18:57:58Z)
the run, looked up by its head commit (actions/runs?branch=master&event=push&head_sha=d69b2ea8f71394eb7a948dcada541079e899eaa0): two push runs on master for that commit; the ci.yml one is run <CI run id>, Bridge CI, .github/workflows/ci.yml, event push, head_branch master, completed, success
artifact <artifact id> unsigned-bundle-smoke-windows-latest, 60401470 B, digest sha256:ba5382c6afae35ecb7f94ce7dc887563cba1a2df8da8d6c75853eef0ca346e83, expired false, expires 2026-11-17T18:34:28Z, workflow_run.id <CI run id>; downloaded 2026-11-10T18:56:08Z to 18:56:46Z by gh api .../actions/artifacts/<artifact id>/zip
zip 60401470 B sha256 ba5382c6afae35ecb7f94ce7dc887563cba1a2df8da8d6c75853eef0ca346e83; GitHub digest sha256:ba5382c6afae35ecb7f94ce7dc887563cba1a2df8da8d6c75853eef0ca346e83: EQUAL
zip entries: bridge-tally.mcpb, msi/Bridge_0.7.0_x64_en-US.msi, mcpb-smoke.json, nsis/Bridge_0.7.0_x64-setup.exe
bridge-tally.mcpb sha256 6b788c3559b4205e220f7f28a16d6a7aefe7e0d2b0a7b5bed564a0c9688c2c54; mcpb-smoke.json archive_sha256 6b788c3559b4205e220f7f28a16d6a7aefe7e0d2b0a7b5bed564a0c9688c2c54: EQUAL
bridge_mcp.exe 35684352 B sha256 3f880c1e1bca963ccfbe949ee8ca61f4d6f2d8c12e53e5bee6d51a0d1adf4056; mcpb-smoke.json binary_sha256 3f880c1e1bca963ccfbe949ee8ca61f4d6f2d8c12e53e5bee6d51a0d1adf4056: EQUAL
pdfium.dll sha256 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118; mcpb-smoke.json pdfium_library_sha256: EQUAL
manifest.json version 0.7.0; it names no commit
the run's head_sha (actions/runs/<CI run id>): d69b2ea8f71394eb7a948dcada541079e899eaa0, the Build line's commit: EQUAL
the tie, in three links: 1. the zip's sha256 equals GitHub's digest of artifact <artifact id>: EQUAL; 2. artifact <artifact id> belongs to run <CI run id> (its workflow_run.id): EQUAL; 3. run <CI run id>'s head_sha is d69b2ea8f71394eb7a948dcada541079e899eaa0: EQUAL
```

### step 0: the exe before the launch
```
3f880c1e1bca963ccfbe949ee8ca61f4d6f2d8c12e53e5bee6d51a0d1adf4056
```

### step 0: the client
```
$ the row's client: run/client.py, row 2399-R's run/client.py byte for byte (the file that ran there, sha256 1e6b490f10d2534a14d3c14d913815a6ba127bdb79da712a2be8d9d25186e324)
run/client.py sha256 1e6b490f10d2534a14d3c14d913815a6ba127bdb79da712a2be8d9d25186e324
lab/2399-r-answers/scripts/client.py at 0500c600 is the pushed text of that file, sha256 6d783d5521be8432a8cfbb26ee8d856f3e55d393aaa4ebe4af88083dd4c4d2ce: its substitutions.txt lists the five changed words (the clientInfo name and four upper-case words), and nothing else differs
```

### step 0: the book's data folder and the excluded GUID
```
BRIDGE CORPUS DENSE: folder <N>, GUID hits 1, excluded GUID hits 0
folders named so: 1
exit 0
```

### step 1: the data folder before the launch
```
ls: cannot access 'C:/lab/2399-d/data': No such file or directory
exit 2
```

### c0: trail.py as the sheet spells it (python3)
```
Python was not found; run without arguments to install from the Microsoft Store, or disable this shortcut from Settings > Apps > Advanced app settings > App execution aliases.
exit 49
```

### c0: trail.py --expect list_companies 4
```
line 1	2026-11-10T19:00:13.394Z	list_companies	81a7dc31-c896-44ea-b3fc-c24db018bc86	sent 4	omitted 0	failed 0	listed 4 (post 2, status 2)	listed request bytes 5048	listed response bytes 8126
records printed 1
lines read 2; records by type {'response_prepared': 1, 'stdio_write_completed': 1}
calibration ok: the last list_companies record has sent 4
exit 0
```

### m0: trail.py
```
line 1	2026-11-10T19:00:13.394Z	list_companies	81a7dc31-c896-44ea-b3fc-c24db018bc86	sent 4	omitted 0	failed 0	listed 4 (post 2, status 2)	listed request bytes 5048	listed response bytes 8126
line 3	2026-11-10T19:00:47.279Z	ledger_masters	818bc0ec-1bf0-419c-b4c2-a63efe802846	sent 26	omitted 0	failed 0	listed 26 (post 14, status 12)	listed request bytes 23804	listed response bytes 309748
records printed 2
lines read 4; records by type {'response_prepared': 2, 'stdio_write_completed': 2}
exit 0
```

### D1 c1: trail.py
```
line 1	2026-11-10T19:00:13.394Z	list_companies	81a7dc31-c896-44ea-b3fc-c24db018bc86	sent 4	omitted 0	failed 0	listed 4 (post 2, status 2)	listed request bytes 5048	listed response bytes 8126
line 3	2026-11-10T19:00:47.279Z	ledger_masters	818bc0ec-1bf0-419c-b4c2-a63efe802846	sent 26	omitted 0	failed 0	listed 26 (post 14, status 12)	listed request bytes 23804	listed response bytes 309748
line 5	2026-11-10T19:01:07.973Z	vouchers	ab17d5ad-2201-4eb9-8362-042d56717233	sent 52	omitted 20	failed 0	listed 32 (post 21, status 11)	listed request bytes 43216	listed response bytes 6054161
records printed 3
lines read 6; records by type {'response_prepared': 3, 'stdio_write_completed': 3}
exit 0
```

### D1 c1: rpc.py
```
run\answers\003-c1.json: 119718 bytes, sha256 6338cb2285c7a6a81dc5541e217f4d6a59fc68dd5d6c0557195afe3024c3dc31
C:\lab\2399-d\rpc\c1.json: 119419 bytes, sha256 d123f144c2d834e5ec6694663da45b777fcd6223ae352d533f212b4687e3f6ea, equal to the response value: True
message holds: result
exit 0
```

### D1 c1: answer.py
```
C:\lab\2399-d\rpc\c1.json: 119419 bytes, sha256 d123f144c2d834e5ec6694663da45b777fcd6223ae352d533f212b4687e3f6ea
census.requests 4	census.ms 3096
census_reused absent
window parts 2
items 82: 3579, 3580, 3581, 3582, 3583, 3584, 3585, 3586, 3587, 3588, 3589, 3590, 3591, 3592, 3593, 3594, 3595, 3596, 3597, 3598, 3599, 3600, 3601, 3602, 3603, 3604, 3605, 3606, 3607, 3608, 3609, 3610, 3611, 3612, 3613, 3614, 3615, 3616, 3617, 3618, 3619, 3620, 3621, 3622, 3623, 3624, 3625, 3626, 3627, 3628, 3629, 3630, 3631, 3632, 3633, 3634, 3635, 3636, 3637, 3638, 3639, 3640, 3641, 3642, 3643, 3644, 3645, 3646, 3647, 3648, 3649, 3650, 3651, 3652, 3653, 3654, 3655, 3656, 3657, 3658, 3659, 3660
exit 0
```

### D2 c2: trail.py
```
line 1	2026-11-10T19:00:13.394Z	list_companies	81a7dc31-c896-44ea-b3fc-c24db018bc86	sent 4	omitted 0	failed 0	listed 4 (post 2, status 2)	listed request bytes 5048	listed response bytes 8126
line 3	2026-11-10T19:00:47.279Z	ledger_masters	818bc0ec-1bf0-419c-b4c2-a63efe802846	sent 26	omitted 0	failed 0	listed 26 (post 14, status 12)	listed request bytes 23804	listed response bytes 309748
line 5	2026-11-10T19:01:07.973Z	vouchers	ab17d5ad-2201-4eb9-8362-042d56717233	sent 52	omitted 20	failed 0	listed 32 (post 21, status 11)	listed request bytes 43216	listed response bytes 6054161
line 7	2026-11-10T19:01:38.704Z	vouchers	b29df0bd-259a-4d27-b809-4ea1767ddc4a	sent 28	omitted 0	failed 0	listed 28 (post 18, status 10)	listed request bytes 36520	listed response bytes 5646950
records printed 4
lines read 8; records by type {'response_prepared': 4, 'stdio_write_completed': 4}
exit 0
```

### D2 c2: rpc.py
```
run\answers\004-c2.json: 119827 bytes, sha256 60ca3e7fadd973b043968371bfd852795bfa5f2fdf6d2b16948b9d56acf1a151
C:\lab\2399-d\rpc\c2.json: 119528 bytes, sha256 fa3fd024b3113da7912f2db6f2aea855b5a526321030376352ff8830020acedd, equal to the response value: True
message holds: result
exit 0
```

### D2 c2: answer.py
```
C:\lab\2399-d\rpc\c2.json: 119528 bytes, sha256 fa3fd024b3113da7912f2db6f2aea855b5a526321030376352ff8830020acedd
census.requests 0	census.ms 0
census_reused {"read_at": "2026-11-10T19:01:07.937Z"}
window parts 2
items 82: 3579, 3580, 3581, 3582, 3583, 3584, 3585, 3586, 3587, 3588, 3589, 3590, 3591, 3592, 3593, 3594, 3595, 3596, 3597, 3598, 3599, 3600, 3601, 3602, 3603, 3604, 3605, 3606, 3607, 3608, 3609, 3610, 3611, 3612, 3613, 3614, 3615, 3616, 3617, 3618, 3619, 3620, 3621, 3622, 3623, 3624, 3625, 3626, 3627, 3628, 3629, 3630, 3631, 3632, 3633, 3634, 3635, 3636, 3637, 3638, 3639, 3640, 3641, 3642, 3643, 3644, 3645, 3646, 3647, 3648, 3649, 3650, 3651, 3652, 3653, 3654, 3655, 3656, 3657, 3658, 3659, 3660
exit 0
```

### D2: answer.py --same-items c1 c2
```
items equal: True (82 and 82 items)
exit 0
```

### D1 and D2: result.total and the census
```
$ result.total, the result state and isError of each vouchers answer, read from the rpc copies (a read beside the sheet calls; answer.py prints no total); each call's start (the client's call_sent_utc) and round trip from the client's saved answer
c1: isError False; result.total 82; result.state complete; items 82; started 2026-11-10T19:01:00Z, round trip 7.356 s, so ended about 19:01:07Z
c2: isError False; result.total 82; result.state complete; items 82; started 2026-11-10T19:01:35Z, round trip 3.293 s, so ended about 19:01:38Z
```

### m1: trail.py
```
line 1	2026-11-10T19:00:13.394Z	list_companies	81a7dc31-c896-44ea-b3fc-c24db018bc86	sent 4	omitted 0	failed 0	listed 4 (post 2, status 2)	listed request bytes 5048	listed response bytes 8126
line 3	2026-11-10T19:00:47.279Z	ledger_masters	818bc0ec-1bf0-419c-b4c2-a63efe802846	sent 26	omitted 0	failed 0	listed 26 (post 14, status 12)	listed request bytes 23804	listed response bytes 309748
line 5	2026-11-10T19:01:07.973Z	vouchers	ab17d5ad-2201-4eb9-8362-042d56717233	sent 52	omitted 20	failed 0	listed 32 (post 21, status 11)	listed request bytes 43216	listed response bytes 6054161
line 7	2026-11-10T19:01:38.704Z	vouchers	b29df0bd-259a-4d27-b809-4ea1767ddc4a	sent 28	omitted 0	failed 0	listed 28 (post 18, status 10)	listed request bytes 36520	listed response bytes 5646950
line 9	2026-11-10T19:02:00.510Z	ledger_masters	598776b3-71e6-4f71-a6b1-097f0371e6ae	sent 26	omitted 0	failed 0	listed 26 (post 14, status 12)	listed request bytes 23804	listed response bytes 309748
records printed 5
lines read 10; records by type {'response_prepared': 5, 'stdio_write_completed': 5}
exit 0
```

### m0 and m1: the marks
```
002-m0 snapshot path structuredContent.result.snapshot master_alter_id 328 voucher_alter_id 29900
005-m1 snapshot path structuredContent.result.snapshot master_alter_id 328 voucher_alter_id 29900
```

### step 4: the exe after the close
```
2026-11-10T19:02:24Z bridge_mcp.exe sha256 3f880c1e1bca963ccfbe949ee8ca61f4d6f2d8c12e53e5bee6d51a0d1adf4056 after the last call; before: 3f880c1e1bca963ccfbe949ee8ca61f4d6f2d8c12e53e5bee6d51a0d1adf4056
```

### the program file of the Tally that answered
```
$ the program file of the Tally that answered (read after C10, with Tally closed)
tally.exe file version 1.1.7.2; 18485952 B; sha256 856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73; last written 2026-10-25T05:34:45Z
```

### size and sha256 of every answer
```
$ size and sha256 of every answer as saved on the lab PC (the client files; the extracted rpc files; the egress file, and its copy taken after the close)
run/answers/000-tools.json 171155 B 57a0af2fe5448580739fe5aa2831d22eb7914aff79544b8ce9359a7c0a827fd1
run/answers/001-c0.json 1569 B 61a213aca1fbfc853ae45a87d3697c95020e014d4b24fda9714f8503921516f7
run/answers/002-m0.json 32059 B 4066ce25b7fbfdbad42aa42b169db79ab755b18d31c614c642775e006aeadae1
run/answers/003-c1.json 119718 B 6338cb2285c7a6a81dc5541e217f4d6a59fc68dd5d6c0557195afe3024c3dc31
run/answers/004-c2.json 119827 B 60ca3e7fadd973b043968371bfd852795bfa5f2fdf6d2b16948b9d56acf1a151
run/answers/005-m1.json 32059 B d49febfe60bc038b5841f98708eb70e00d0adaf5cfcd87afaf97b3e886fcc0e2
run/step0/initialize.json 2034 B d48cf40c4df5b468425facbc08e1eec05478b93cd9498ba30c334fdf252b6adc
C:/lab/2399-d/rpc/c1.json 119419 B d123f144c2d834e5ec6694663da45b777fcd6223ae352d533f212b4687e3f6ea
C:/lab/2399-d/rpc/c2.json 119528 B fa3fd024b3113da7912f2db6f2aea855b5a526321030376352ff8830020acedd
C:/lab/2399-d/data/agent-egress.jsonl 19798 B 224ab370f300a47c4a4dfbd14990cd67080874ee2e615470b5ea4dcbb779fc5d
egress-copy-after-close.jsonl 19798 B 224ab370f300a47c4a4dfbd14990cd67080874ee2e615470b5ea4dcbb779fc5d
```

## The client log

### run/client-log.txt
```
2026-11-10T18:59:53Z exe sha256 3f880c1e1bca963ccfbe949ee8ca61f4d6f2d8c12e53e5bee6d51a0d1adf4056 ok; launch-env.json sha256 f60710e01132a813bb419ec5640d3d5a310cf38ace472ce1882cc1704cbe8078; keys ['BRIDGE_AGENT_DATA_DIR', 'BRIDGE_TALLY_HOST', 'BRIDGE_TALLY_PORT', 'BRIDGE_TERMS_ACCEPTED']; ENABLE_WRITES present: False
2026-11-10T18:59:53Z launched pid 20448; stderr to server-stderr-20261110T185953Z.log
2026-11-10T18:59:53Z initialized {'name': 'bridge-tally', 'version': '0.7.0'}
2026-11-10T19:00:04Z 000-tools.json tool=list secs=0.011 isError=None error=None saved_sha256=57a0af2fe5448580739fe5aa2831d22eb7914aff79544b8ce9359a7c0a827fd1
2026-11-10T19:00:13Z 001-c0.json tool=list_companies secs=0.116 isError=False error=None saved_sha256=61a213aca1fbfc853ae45a87d3697c95020e014d4b24fda9714f8503921516f7
2026-11-10T19:00:47Z 002-m0.json tool=ledger_masters secs=0.835 isError=False error=None saved_sha256=4066ce25b7fbfdbad42aa42b169db79ab755b18d31c614c642775e006aeadae1
2026-11-10T19:01:08Z 003-c1.json tool=vouchers secs=7.356 isError=False error=None saved_sha256=6338cb2285c7a6a81dc5541e217f4d6a59fc68dd5d6c0557195afe3024c3dc31
2026-11-10T19:01:38Z 004-c2.json tool=vouchers secs=3.293 isError=False error=None saved_sha256=60ca3e7fadd973b043968371bfd852795bfa5f2fdf6d2b16948b9d56acf1a151
2026-11-10T19:02:00Z 005-m1.json tool=ledger_masters secs=0.895 isError=False error=None saved_sha256=d49febfe60bc038b5841f98708eb70e00d0adaf5cfcd87afaf97b3e886fcc0e2
2026-11-10T19:02:11Z session ended; server pid 20448 exit code 0
```

