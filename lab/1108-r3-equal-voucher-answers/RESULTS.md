# Row 1108-R3: results

**First line: no stop and no pause. E-BASE, E-1, E-3, E-4a, E-X and E-END are confirmed. E-V reads differently from the code reading and is recorded. E-4b was not reached, because no box showed. One deviation: at the BK-R restore, End of List was clicked and Enter pressed with no screenshot between them. The Accept ? that the click brings was therefore answered unseen. No read changed.**

Sheet: `lab/1108-r3-equal-voucher` at 0f7f9e08, run 2026-11-19, position (b) only (the run lead did not want position (a)).
Book: BRIDGE STOCK CREATE LAB (`<company guid>`; integrated, inventory on). TallyPrime release 7.2 (tally.exe 1.1.7.2), edition as recorded in step 0 and in the `tally_status` answer, Licensed (education_mode false).
Bridge: the 0.7.0 bundle's `bridge_mcp.exe`, the copy the 2214 row ran as its server, 35,770,880 B, sha256 `7086f1d0d7430e7dc1fefd6f69c0d4a14d717a2e06477b32935a1d3aa4fee1b8`. It equalled Build before the server and again at step 8. The installed path, read from the lab's shell, shows the package view's overlay copy (a different file, 0.6.1), as the 2214 row's notes record; that copy was not used.
Client: R0 to R2's stdio client (sha256 `0ed0bb03...`). Its arguments were root, exe and sha256 in that order, which is the sheet's step 6 order; R0 to R2's notes do not give the command line.

## Steps 0 to 6, 9 and 10

- Step 0:
  - `setup-r3.py` gave `files: ok 31, FAIL 0`, the `BK-R:` line (26485 B, `f02e31d1...`) and `setup: ok, 12 helpers`.
  - `make_e1.py check` gave `e1 request: ok (1826 B)`. check-request-ids gave `census: ok 4, skipped 1, FAIL 0`. check-proxy-keep and test-exchanges each gave `0 failed`.
  - `args1108.py fixed` exited 0. The box self-test logged its watch start line and `NO BOX seen within 3 s`; box.ps1 exited 3 with 0 bytes of console output.
  - The placeholder copies: `note.py` had 2 `<LAB_DIR>` and 0 `<Tally data folder>` replacements, sha256 `d74800cb...`; `c6prep.py` had 3 and 2, sha256 `71c8e21c...`.
  - C2: Select Company with none loaded and no dialog. One tally.exe owned 9001. GET /status answered 200.
  - m0: one COMPANY row, M 210, no ALTVCHID. The Gateway's current date was 1-Apr-2025. Nothing listened on 9003.
- Step 1, START:
  - flags: integrated Yes, inventory on Yes.
  - ledgers: Cash and Profit & Loss A/c only.
  - vouchers: none.
  - marks: equal to m0.
- Step 2, BK-S: `TDBK1800_<company number>.001`, 24,219 B, sha256 `ecc8d4028b88b763e5a8a22014bc699b07db68c3eb32cff3c6c92052b373b0f4`. The marks did not move.
- Step 3, BK-R:
  - c6prep printed BK-R equal to recorded. BK-R was restored on the run lead's word.
  - The Gateway's current date read 10-Apr-2025.
  - BASELINE: V 1, M 213; ledgers a=5; vouchers a=1, the seed.
- Step 4, E1: sent once and answered `CREATED 1` with every other counter 0, `LASTVCHID 2` and no `LINEERROR`. It added one Journal: 20250414, A2 Expense Dr 21.00 / A1 Capital Cr 21.00, narration 'Row 1108-P2 first', with MASTERID, ALTERID and VOUCHERNUMBER all 2. Marks after it: V 2, M 213.
- Step 5: the proxy ran on 9003 from 14:13:53.961Z. r3-first = 0.
- Step 6:
  - `agent-r3` was absent before the server.
  - `list_companies` gave THE COMPANY alone, `verified_tuple`.
  - The Journal type reads `automatic` / `auto_retain`.
  - mk-start: V 2, M 213, equal to step 4's last marks.

- Step 9, BK-S restore:
  - c6prep printed BK-S equal to recorded. BK-S was restored on the run lead's word, after the E rows above were recorded.
  - The Gateway's current date read 1-Apr-2025, as at step 0; it was not typed.
- Step 10, C10:
  - The book was shut, leaving Select Company with none loaded.
  - One tally.exe owned 9001; nothing listened on 9003; GET /status answered 200.
  - No bridge_mcp.exe or proxy was running.

## Step 7, the batch

| call | sent (UTC) | seconds | answer |
|---|---|---|---|
| `build_import_xml` | 14:14:58.563Z | 4.673 | batch `bridge-c2935cd7-634e-46cc-b6cc-917c7d8ef1b1`, `voucher_count` 3, totals 66 / 66, six warnings; the first is the rows-in-book warning naming `b1108-p2-1` |
| `post_import` (by `p1108s2.py`) | 14:15:20.205Z | 3.599 | refused: `import_preexisting_identity`, `preexisting_txn_ids` `["b1108-p2-1"]`, "Nothing was sent: ...", `attempt_recorded` false, `dispatch_response` null |
| `verify_import` | 14:17:42.713Z | 3.265 | `verification_incomplete`, `verified_total` 0; `b1108-p2-1` `not_attributable`, `b1108-p2-2` and `b1108-p2-3` `not_found` |
| `egress_log` | 14:17:57.664Z | 0.05 | 12 records, one prepared response and one completed write for each of the six calls |

- No box showed. The box watch started at 14:15:19.791Z and logged `NO BOX seen within 120 s` at 14:17:19.974Z. The helper ended with `HALT: the box check did not pass`, exit 1, as the sheet's step 7.5 expects. The lab clicked nothing.
- The server ended on the stop file: pid 9260, exit code 0, with 0 bytes of stderr.

## The EXPECT table, filled in

| Id | Result | What the row shows |
|---|---|---|
| E-BASE | confirmed | V 1, M 213; 5 ledgers (A1 Closing Stock under Stock-in-Hand, A1 Capital under Capital Account, A2 Expense under Indirect Expenses, Cash, Profit & Loss A/c); one voucher, the seed Journal of 10-Apr-2025, Cash Dr 200.00 / A1 Capital Cr 200.00, 'Row 1108-B cash leg', ids 1 |
| E-1 | confirmed | `CREATED 1`, ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED and EXCEPTIONS 0, `LASTVCHID 2`, no `LINEERROR`; V 2, M 213; one voucher ADDED: 14-Apr-2025, Journal, A2 Expense Dr 21.00 / A1 Capital Cr 21.00, 'Row 1108-P2 first', MASTERID, ALTERID and VOUCHERNUMBER 2 |
| E-3 | confirmed | not refused; `voucher_count` 3; the first warning begins "1 of 3 rows of this batch already match vouchers in the book as this build read it (bridge_txn_id: b1108-p2-1). With Allow voucher posting on, post_import would refuse this batch as import_preexisting_identity before any approval, if the book is unchanged." |
| E-4a | confirmed | `error.code` `import_preexisting_identity`; `error.preexisting_txn_ids` exactly `["b1108-p2-1"]`; `error.message` "Nothing was sent: rows of this batch already match vouchers in the book. See error.next_step."; `attempt_recorded` false; `dispatch_response` null; no voucher statuses in the post. mk-before = mk-after = V 2, M 213. The vouchers diff against step 4's read: 2 unchanged, none ADDED, REMOVED or CHANGED (the answers are byte-equal). Box log `NO BOX seen within 120 s`; the helper halted as step 7.5 says |
| E-X | confirmed | `check-exchanges.py` 1 to 130: exit 0, `exchanges: ok 130, FAIL 0`, `imports 0`; 130 = r3-last − r3-first |
| E-V | recorded, differs from the code reading | the code reading said `b1108-p2-1` would be found by content at E1's ids. It reads `not_attributable` (marker `accounting_fingerprint`, reason `fingerprint_precedes_pre_import_voucher_mark`). `b1108-p2-2` and `b1108-p2-3` read `not_found`, as the reading said. `post_span_binding` `not_applicable`; `alter_id_delta` 0 |
| E-4b | not reached | no box showed; the post was refused before any window |
| E-END | confirmed | after the BK-S restore: the end ledgers, vouchers and marks answers are byte-equal to step 1's START answers (2 ledgers, no voucher, no ALTVCHID, M 210); each diff shows no row ADDED, REMOVED or CHANGED |

## Decides

The first row of the sheet's Decides applies (E-1, E-3, E-4a, E-X as written), **partial: one run, one book, position (b) only**. On this build, a voucher already in the book and equal in content to a batch voucher stops the batch before the approval window. Nothing was sent, and no partial landing happened. So a voucher that stood in the book before the post cannot stand in for a batch voucher in a partial landing's readback. What stays unmeasured: an equal voucher that enters the book after the post's checks (a second writer), and position (a), which was not run.

After the refusal, `verify_import` did not attribute E1 to the batch row it matches. It names the row `not_attributable` because the matching voucher precedes the build's voucher mark. It does not name it found.

## Deviations and notes

- **BK-R restore keys (step 3).** The sheet's Difference 4 asks for a screenshot after End of List, and `y` only at an Accept ? that has been seen. The lab clicked End of List and pressed Enter in one batch, with no screenshot between them. The next screen was the folder-exists prompt, which was captured, and Replace was chosen. No Accept ? was seen; if one showed, the Enter answered it, as in R1's restore. At step 9, End of List was clicked alone, and Accept ? showed at once. It was captured and answered with `y`. So at step 3 the Enter after the click answered an Accept ? that was not seen. The restore was the one intended (BK-R, equal to recorded), and the BASELINE that followed was met.
- The first write of the exe-hash evidence file held a value shifted by one leading backslash from the hashing tool. It was set aside and rewritten.
- Focus: after each typing of the company filter at Select Company, the desktop app's own window held the keyboard focus. One click on the Select Company title, outside the list, gave Tally the focus before Enter.
