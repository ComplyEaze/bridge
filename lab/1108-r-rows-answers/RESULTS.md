# 1108-R rows R0, R1 and R2: results

**First line: no stop and no pause. Every EXPECT was met in all three rows: E-R0, E-R1, E-R2, E-ORD and E-X are confirmed. The run lead recorded two deviations: the R1 BK-R restore keys, and the R2 egress_log answer's file name. Neither changed a read.**

Sheet: `lab/1108-r-rows-sheet` at 8c9c11b9, queue item 94.
Book: BRIDGE STOCK CREATE LAB (integrated, inventory on), TallyPrime release 7.2 (tally.exe 1.1.7.2), edition as recorded in step 0 and in each `tally_status` answer.
Bridge: the build line's artifact. The zip is 60529312 B, sha256 6385ae8446950765e4dfacd49e97babdf82fefc19b336e5241578a007c9fd3fe, equal to its digest. `bridge-tally.mcpb` sha256 is b291ea38de06c0a36f80b707d0e6cc3bb92a8b05a41c7707f8ecf6ebde3c9f3c, equal to archive_sha256. `bridge_mcp.exe` is 35785728 B, sha256 ce96a872d7e60021f1cdbe26fb0708be0bacabe6bb731e7541838f248f5e3f9d, equal to binary_sha256. It was hashed before the first row and again at the end, and was the same both times.
Each row had its own new client folder (`agent-run-rk`) and its own new data folder (`agent-rk`); each data folder was absent before its row.

## Steps 0 to 6

- Step 0:
  - `setup-r.py` gave `files: ok 33, FAIL 0` and `setup: ok`.
  - check-request-ids gave `census: ok 4, skipped 4, FAIL 0`; check-proxy-keep and test-exchanges each gave `0 failed`.
  - The box.ps1 self-test logged its watch start line, saw no box, and exited 3 with no console output.
  - Exactly one tally.exe owned 9001, and GET /status answered 200. No line used port 9003.
  - m0: one COMPANY row, M 210, no ALTVCHID.
- Step 1, START:
  - flags: ISINTEGRATED Yes, ISINVENTORYON Yes.
  - ledgers: Cash and Profit & Loss A/c only.
  - vouchers: none.
- Step 2, BK-0: `TDBK1800_<company number>.001`, 24219 B, sha256 df1eed2ecf4e4945a872525ad57d63cebbb9c47127c20c9e7919b291cfac9b39. The marks did not move.
- Step 3: W1, W2 and W3 were each sent once. Each answered CREATED 1 with every other counter 0, and each added one ledger with the sent name and parent: A1 Closing Stock (Stock-in-Hand), A1 Capital (Capital Account), A2 Expense (Indirect Expenses).
- Step 4: S0 was sent once and answered CREATED 1, LASTVCHID 1. It made one Journal of 2025-04-10, Cash Dr 200.00 / A1 Capital Cr 200.00, 'Row 1108-B cash leg', with MASTERID, ALTERID and VOUCHERNUMBER all 1. V0 = 1, M1 = 213.
- Step 5, BK-R: `TDBK1800_<company number>.001`, 26485 B, sha256 f02e31d171d92e41cf2120bdcbdf50375383e461da64d8c273df16d8b9179e59. BASELINE: V 1 / M 213; ledgers a=5; vouchers a=1.
- Step 6: the proxy ran from 9003 to 9001 for the whole item, into `proxy/`.

## The three rows

| Row | Order sent | Batch | Box seen, then Yes | post_import | verify_import | Marks start, after, restored |
|---|---|---|---|---|---|---|
| R0 | -1, -2, -3 | bridge-28ee7d21-4b04-41e6-ada5-fa93e4a8fb20 | 17:39:33.695Z, 1.1 s, check ALL PASS | one call, 14.448 s | once, 3.351 s | V 1 / M 213, V 3 / M 213, V 1 / M 213 |
| R1 | -2, -1, -3 | bridge-955a931c-b1ee-459f-ac29-cebb88e3d414 | 17:45:04.071Z, 0.9 s, check ALL PASS | one call, 14.226 s | once, 3.437 s | V 1 / M 213, V 3 / M 213, V 1 / M 213 |
| R2 | -1, -3, -2 | bridge-619f226d-3314-429b-822f-99571103c420 | 17:49:03.899Z, 0.9 s, check ALL PASS | one call, 13.424 s | once, 3.607 s | V 1 / M 213, V 3 / M 213, V 1 / M 213 |

In each row the box was 'ComplyEaze Bridge — post 3 vouchers?' (#32770), owned by that row's bridge_mcp. It was topmost and above Tally. It showed exactly one Create line matched whole, the Company GUID and Batch lines whole, and the totals as built (66 and 66). Each time, the retry loop made one call only, and no second or new box appeared.

## The EXPECT table, filled in

| Id | Result | What the rows show |
|---|---|---|
| E-R0 | confirmed | The proxy's import-1 answer: CREATED 2, EXCEPTIONS 1, every other counter 0, LASTVCHID 3 (V0 + 2), and one LINEERROR naming A1 Closing Stock. The post's outcome counters agree. Marks went to V 3 / M 213. Two vouchers were ADDED: -00000002 'Row 1108-P2 first' 21.00 with ids 2, then -00000003 'Row 1108-P2 third' 23.00 with ids 3, as in A2's P2. In both the post and verify_import, b1108-p2-1 and b1108-p2-3 read matching_content_observed / not_established and neither reads posted_verified. b1108-p2-2 reads sent_not_attributed with its next step. post_span_binding was refused with span_counters_not_clean; error.code import_reconciliation_required; response_state response_not_clean. |
| E-R1 | confirmed | Counters as E-R0. The voucher sent first (b1108-p2-2) took no MASTERID, ALTERID or VOUCHERNUMBER; the two created took ids 2 and 3. Statuses as E-R0, voucher by voucher. |
| E-R2 | confirmed | Counters as E-R0. The voucher sent last (b1108-p2-2) took none; the two created took ids 2 and 3. Statuses as E-R0, voucher by voucher. |
| E-ORD | confirmed | `check_1108.py diff` of R0's post vouchers against R1's and against R2's: rows a=3, b=3, unchanged rows 3, with none ADDED or REMOVED. So the same GUIDs, MASTERID, ALTERID, VOUCHERNUMBER, DATE, NARRATION and entries appear whatever the order sent, and no other field differs. |
| E-X | confirmed (R0, R1, R2) | check-exchanges exit 0 in each row: R0 x-1 to x-236, R1 x-237 to x-472, R2 x-473 to x-708, each `ok 236, FAIL 0, imports 1`. The one import in each row (x-175, x-411, x-647) is listed byte-equal to import-1, import-2 and import-3. The three import answers are byte-equal to one another (859 B, sha256 ace093aa9d72ce74a6c715a916a491a0e64117c704fabf2802feaf1ad98b320d). |

xk-first and xk-last: R0 0 and 236; R1 236 and 472; R2 472 and 708.

Recorded, not a verdict:
- check-exchanges lists the import's TYPE and ID as `<absent>`, because the import request carries neither.
- Neither the post nor verify_import has a top-level `created` or `attempt_recorded` field. `created` appears only as a counter key; approval is null.
- After each BK-R restore, the Gateway's current date read 2025-04-10. Step 0 had read 2025-04-01.

## Deviations, each accepted by the run lead

1. R1's BK-R restore. The click on End of List closed the list by itself, so the Enter that followed answered the 'Accept ?' prompt before it was seen (Enter = Yes). The next screen was the expected 'already exists' prompt, answered with Replace. The reads were equal to the BASELINE. From R2 on, a screenshot was taken after End of List and y was pressed only at a seen 'Accept ?'.
2. R2's egress_log answer is named `006-006-r2-egress.json`: the label given already had its number, and call.py adds its own. The call and its content are as in R0 and R1. The file was left unrenamed.

## Step 8 and C10

- Step 8, BK-0, restored on the run lead's RESTORE word. The keys followed the R2 practice, and the only prompts were 'Accept ?' (y) and 'already exists' (Replace). End reads: ALTVCHID blank, M 210, equal to START (m0). Ledgers a=2, b=2, unchanged. Vouchers a=0, b=0.
- C10, the date: after the BK-0 restore the Gateway already read 2025-04-01, step 0's date. F2 opened Current Date pre-filled with 2-4-2025, which was not accepted. I typed 1-4-2025 and the Gateway read 2025-04-01 after.
- C10, the close: the company was shut by Company > Shut with Enter on the one row; Select Company showed no company loaded. One tally.exe, PID 23160, owned 9001, and GET /status answered 200. Tally was left running at Select Company, as at step 0. The proxy was stopped and its folder kept whole.
