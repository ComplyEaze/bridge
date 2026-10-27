# Results: #1714 R3, a skip recorded after a statement batch is built refuses that batch when it is sent, on the 0.6.1 cut build 67f647e1 and TallyPrime 7.2, 27 Oct 2026. Step 5 PASS; step 6 (the control post) not completed

The sheet is `R3-1714-skip-at-send.md` with `LAB-COMMON.md` and `RUN-SHEET.md` at 539f191d (SHA256.txt: 8 of 8 files equal in hash and size).

## Build and setting
- The cut build: RC commit 67f647e1a292e8d7815053b5c08289cba6402795, CI run 40456677783, artifact 12712717489; `bridge_mcp.exe` sha256 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d (= the sheet's line) at the launch and after the close. It reports `bridge-tally` 0.6.0, as the sheet's build notes expect; `tools/list` holds 33 tools.
- ONE launch over MCP stdio from step 0 to the close, 09:28:51 to 10:15:41 UTC (`client-log.txt`), with `BRIDGE_TERMS_ACCEPTED`, host 127.0.0.1, port 9001, the row's new empty data folder and `BRIDGE_AGENT_ENABLE_WRITES`; `_ENABLE_BATCH_POST`, `_MAX_BYTES` and `_REDACTION` absent. The server's error stream was kept and stayed empty.
- TallyPrime 7.2, Silver, not Educational mode. The book: BRIDGE TIE OUT LAB 3 (company number 100065, GUID 9c97970c-e25d-4348-9740-6ddb1caaa56d), loaded alone; Tally's current date 10-Oct-2026 throughout.
- Step 0 process list: one tally.exe (owning 9001) and four other `bridge_mcp.exe` processes of the desktop app's two Bridge entries, which stayed running and idle; no call was made through them during the row. The sheet asks that no other ComplyEaze Bridge run against port 9001: this is a deviation, recorded here.
- THE STATEMENT: the synthetic three-month PDF, sha256 506e05ebfb0f81d6e0fa4b0cc867bbaeae8a69d130c02615ea48a86559894fc9; parse arguments as on 14 October (bank `ubi`, account label `Lab UBI 7788`, opening 10000.00, closing 11000.00, bank ledger Union Bank, suspense ledger Suspense), only `from`, `to` and `mapping` changed. The PDF and its password file stay on the lab PC.
- BK1, taken by Tally's Backup screen before the first change: `TDBK1800_100065.001`, 32,143 bytes, sha256 8a1cca5c001b395c61ee11773f3cf0159dab4070eea9293332fc051d2c6ebdb3 (27-Oct-26 3:01 pm). It stays on the lab PC.

## Result by step

| step | act | read | result |
|---|---|---|---|
| 0 | none | one company; M0 210, V0 8; Union Bank 1 Jul to 30 Sep: opening -10000.00, closing -11800; Payment and Receipt `automatic` / `auto_retain` / `not_prevented` | PASS |
| 1 | ledger BRIDGE Bank B created on Tally's screen (Bank Accounts, no opening balance) | M1 211, V1 8 | PASS |
| 2 | parses only | September: `rows_in_window` 2, `cash_questions_open` 0; one-day parses: D = 2026-09-03 (P = "GREEN FIELD LTD", Payment), D2 = 2026-09-30 ("SILVER OAK MUTUAL", Receipt); every other day `statement_empty_selection` | PASS |
| 3 | parse D, build | `voucher_count` 1; A = bridge-6acfcebd-5a43-40cd-a009-c78bf6a7ffd0 | PASS |
| 4 | parse D with P mapped `skip` to BRIDGE Bank B, build | parse `vouchers` 0, `skipped` 1; build `state` `skips_recorded`, `skipped_rows` 1, `record_id` skips-c63f3e72-416d-4dfd-8c48-f1e2e640a3ee; `transfers` `computed`, `skips_open` 1, `open_omitted` 0, one `open` row `held_elsewhere` on A; R = st-20260903-3f9e7c8865e24e69 | PASS |
| 5 | `post_import` A | no approval window; `result.error.code` `skip_row_paired_elsewhere`, `skipped_rows` [R], `next_step` beginning "Nothing was sent to Tally, and any approval of this batch is withdrawn." and ending "This batch cannot be sent as built: build it again.", `attempt_recorded` false; V 8 = V5a; the batch journal and the skips file the same size and sha256 before and after; the `post_import` receipt in `egress_log` carries the code and no `request_trail` key; no voucher on D | **PASS** |
| 6 | parse D2, build, `post_import` B | `voucher_count` 1; B = bridge-6bfbf95d-785e-437b-ad68-7352aa8adeec; V6a 8; the approval window opened (Dr 400.00 Union Bank, Cr 400.00 Suspense, 30 Sep 2026) and was not answered: `pending` (050, 051), then `import_approval_timed_out` with `attempt_recorded` false (052); V 8, no voucher on D2 | **not completed**: the approval window timed out unanswered |

Step 6 is the positive control: its window closed by time-out without an answer, B was not sent again, and the row was ended there. Step 5's check stands on its own answers.

The one-day parses ran for every day of September: D2 is the last day, so they were all needed.

## Restore and close
BK1 restored over the book (Accept, Replace); loaded alone. One company; M 210 = M0, V 8 = V0; Union Bank 1 Jul to 30 Sep equal to step 0; `ledger_masters` (one page, 5 ledgers): no BRIDGE Bank B. Batch A stays held in the row's data folder, which is kept on the lab PC and not used again. Close: the company shut; one tally.exe owns 9001; GET /status 200; no company loaded.

## Files
- `answers/NNN-*.json`: every call's answer whole, with its arguments (the statement and password file are named by path; no file's content is in them); each sha256 is in `client-log.txt` and `SHA256.txt`.
- `step5-data-before.txt`, `step5-data-after.txt`: the data folder's files by name, size and sha256 around the step 5 post.
- `notes.txt`: one line per step with UTC times.
- Kept on the lab PC, by name, bytes and sha256: c2d-gateway.jpg 102350 3f67c58b4da5e8a822182ab7422d42dae3030c7cb832a718e5d84debb41826c7; bk1-backup-screen.jpg 162545 cfc2cbfd95b596bcee259276e5672ae78205d950391374930db6d67983b57d8f; s1-ledger-bank-b.jpg 109128 f6d03d5c0abe399758c783551a37233c0bc2214ea1c19eac6a9c681d7e656a70; s6-approval-window-dpi.png 401633 bb91f34a42a927876fcec4366970c94de9fb9c8efdec715e4e4012304682256c; c10-no-company.jpg 147940 1741735a8ba9e5339c54824ceb3a95a40065cec75b5fbc67c4d612086815b6ff; the step 0 and close command outputs; the restore plan; BK1; THE STATEMENT; the data folder.
