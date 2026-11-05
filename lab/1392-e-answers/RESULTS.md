# 1392-E: skipped statement rows judged against a live book (results)

Sheet: `lab/1392-e-statement-pairing` at a13ddc90 (base 187e882b). SHA256.txt 8 of 8 equal; the seven statements written by `make_statements.py` on the lab PC, each equal to `statements.sha256`. LAB-COMMON v3 (5370c6d2..., byte copy). THE COMPANY: BRIDGE TRANSFER LAB, GUID `d6e4c24f-8c7a-4797-93ec-e44bdff60bc9`, its number shown as `<company number>`. TallyPrime 7.2 Silver, licensed, education_mode false; one tally.exe owning port 9001. Build: BUILD-M, zip sha256 1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a (= the digest), `bridge_mcp.exe` 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0 (= mcpb-smoke.json's) before and after. Client: the 852-C1 inbox stdio client (sha256 0ed0bb03...), writes on, batch post and redaction unset, one new empty data folder, one `bridge_mcp` process (pid 12304) for steps 2 to 11.

## Each case's state

| case | state | held? |
|---|---|---|
| **E1** (step 5) | the skipped row of 23 Oct, A's statement a1: **`not_found`**; `skips_recorded`, `skipped_rows` 1, day_reads whole 3, empty 0 | **held** |
| **E2** (step 7) | after the hand-keyed Contra on 23 Oct: both 23 Oct rows (E1's and E2's) **`tally_unchecked`**, neither paired, `skips_paired` 0; day_reads whole 3 | **held** |
| **E3** (steps 8, 9) | **not measured.** B3's Contra was built (one voucher) but never posted: its approval box was declined twice by the lab's box-check helper (a lab defect; the box was right), and a third call was refused `import_approval_declined` with no box. By the run lead's bound the row then ended: restore and C10. A3 was not parsed. | not measured |
| **E4** (step 10) | **not run** (it runs only after E3's `skips_paired` 1) | not run |

## What the result decides, filled in

| step | result seen | reading on #1392 |
|---|---|---|
| 5 | **`not_found`** | **E1 holds:** the whole chain (covered by a verified statement of the other account, three whole days read, no entry on the other ledger) gives `not_found` on a live book. |
| 7 | **both `tally_unchecked`, `skips_paired` 0** | **E2 holds:** a Contra keyed in Tally is seen by the day reads and blocks `not_found`, and is never taken as a pair. |
| 9 | not reached | E3 not measured (B3's Contra never posted, see below). |
| 10b | not reached | E4 not run. |
| any | a refusal before a write | B3's third `post_import`: `import_approval_declined`, recorded whole; nothing after it ran but the end of writes, the restore and C10. |

## The steps

| step | what | result |
|---|---|---|
| 0 | load, m0 | one COMPANY row, this GUID, **V 12, M 211** (= EXPECT). Tally's date Tuesday 28-Apr-2026; PC date 5 Nov 2026. |
| 1 | BK-E, m0b | `TDBK1800_<company number>.001`, 32,428 B, sha256 29c2de73fe7d383f5c388d92ff193e39ea9f65b59a57c8f812891dabf457cbb5, listed 5-Nov-26 1:55 pm. m0b byte-equal to m0. |
| 2 | Kit and Bridge | statements 7 of 7 equal. `list_companies`: this GUID, verified_tuple, books_from 20260401 (on or before 20 Oct). `tally_status`: 7.2, silver, education_mode false. `masters` voucher_types: 24 types; Payment, Receipt and Contra automatic / auto_retain (= EXPECT). Marks equal m0b. |
| 3 | F | built 3; P: box check ALL PASS, Yes clicked, `posted_verified`, created 3; verify 3 of 3. **V 15 (+3)**, M 211. |
| 4 | B1 | first parse refused `statement_password_file_permissions` (below); after its remediation, `reconciled`, TLAB FILLER 2 rows to suspense, `skipped` 0 (= EXPECT). Built 2; P: box check ALL PASS, Yes, `posted_verified` 2; verify 2 of 2. **V 17 (+2)**, M 211. |
| 5 | E1 | parse `reconciled`, one row TLAB SELF XFER, `skipped` 1. Build: `skips_recorded`, `skipped_rows` 1, the row **`not_found`**. `egress_log` whole: the build's request trail sent 70, failed 0, every outcome answered (status reads and posts; no import). V 17 (unchanged). |
| 6 | the decoy | keyed on screen: Contra No. 9, 23-Oct-26 (F2), To TLab Bank A 5,002.00 Cr, By TLab Bank B 5,002.00 Dr, no narration, no reference; both Bank Allocations accepted as offered with Ctrl+A; Tally itself offered the By line (TLab Bank B, 5,002.00) after the first allocation and it was accepted with Enter. The Day Book for 23-Oct-26 shows it. **V 18 (+1)**, M 211. |
| 7 | E2 | parse `reconciled`, `skipped` 1. Build: `skips_recorded`, `skipped_rows` 1; both 23 Oct rows **`tally_unchecked`**, `skips_open` 2, `skips_paired` 0. `egress_log` whole (trail sent 70, failed 0). V 18 (unchanged). |
| 8 | B3 | parse `reconciled`, one row TLAB SELF XFER, a Contra, `skipped` 0 (= EXPECT). Built: one voucher, `voucher_count` 1. **Not posted** (three calls, below). V 18 throughout (marks before calls 2 and 3 byte-equal to e2-marks). |
| 9, 10 | E3, E4 | not run. |
| 11 | end of writes | `egress_log` whole (37 records). m3: **V 18, M 211** (= m0b + 6: F 3, B1 2, the decoy 1). `bridge_mcp` stopped by the client's stop, exit code 0; exe sha256 unchanged. |
| 12 | restore BK-E, m4 | the two expected prompts only (Accept, Replace). m4: GUID, **V 12, M 211**, the answer byte-equal to m0b's (= EXPECT). Tally's date back at 28-Apr-2026. |
| 13 | C10 | the company shut; one tally.exe owning 9001, `/status` 200, none loaded. |

## Findings and observations

- **Sheet defect, the guard held (PASS for the guard).** "sheet defect: pw.txt placed under <lab folder> inherits Users RX / Authenticated Users M; refusal statement_password_file_permissions was correct; followed its remediation once". The sheet placed `pw.txt` under the lab folder, which inherits `Users` RX and `Authenticated Users` M. The first B1 parse was refused `statement_password_file_permissions` (nothing parsed, 0 bytes). The refusal was correct. On the run lead's word its remediation was followed once: a new copy in a new folder under the user profile (ACL by principal: SYSTEM, Administrators and the user, inherited), the original deleted, and every later parse made with only `password_file` changed to that path.
- **A declined box (observation, one run, PARTIAL).** Call 1 of B3's `post_import`: the box was declined, and the answer was `import_approval_declined`, `attempt_recorded` false, `dispatch_response` null, with the marks unmoved.
- **A box left open (observation, one run, PARTIAL).** Call 2: with the box still open, `post_import` did not block. After about 40 s (40.115 s) it returned `approval.state` `pending`, `retry_after_s` 15, `dialog_remaining_s` 86, `attempt_recorded` false, `dispatch.state` `not_dispatched`.
- **A decline is kept for the batch (observation, one run).** Call 3 was refused at once (0.064 s, `import_approval_declined`, `attempt_recorded` false, 0 bytes read). No box was shown: Bridge kept call 2's decline for this batch and did not ask again.
- **The one-voucher box has its own wording.** Title `ComplyEaze Bridge — post this voucher?`, body `Create ONE Contra in "BRIDGE TRANSFER LAB"`, one `Date:` line and each entry line (`Dr 7003.00 "TLab Bank B"`, `Cr 7003.00 "TLab Bank A"`), with the reference and narration below a dashed line. The plural box (2 or more vouchers) reads `post N vouchers?` and `Create N vouchers in`; for 2 vouchers the per-voucher lines were left out ("they do not fit this dialog").

## Lab deviations

1. **Helper defect 1, the box was right.** boxcheck.py, copied from 1029-R (batches of 3 to 50), knew only the plural wording. It failed B3's one-voucher box on the title and Create lines and wrote No; box.ps1 clicked No (call 1, 08:36:00.972Z). On the run lead's word it was widened by the singular form only, accepted only for a batch of one and checked against the built batch (type, date, every line). sha256 281a5889... to 2fec7852....
2. **Helper defect 2.** On call 2 the widened helper ended without writing a check or an answer (its stderr was discarded). box.ps1, finding no answer file by its 70 s limit, clicked No (08:39:56.888Z). Run afterwards on a copy of that box log, the same helper passed every check; the likely cause, not shown, is an unguarded read of the log while box.ps1 was still writing it. On the run lead's word the read was guarded (taken only whole; a refused or cut read is read again for at most 30 s; any error writes No), stderr kept, the answer limit raised to 120 s, and the helper self-tested on copies of saved logs, including a log appended in chunks, one held under a lock and one cut and never finished: every case right. sha256 2fec7852... to 632eca5a.... Call 3 was then refused without a box, so this version answered only "no box seen".
3. Tally's current date moved to 23-Oct-2026 through F2 in the decoy; the restore put it back at 28-Apr-2026.

Not measured, as the sheet says: the parser's fit to real bank PDFs; `withdraw_skips`; `reversed_same_day`; `double_booked`; a Contra keyed by hand on the other side of a pair; and, because of the above, E3 and E4. The four desktop-client connector servers were left untouched: no call was made to them from this sitting, and their calls were not measured. Nothing ended a process.

## Files

- `agent-run/answers/`: every tool answer whole (000 to 023), with `client-log.txt`, `launch-env.json` and `step0/initialize.json`; `agent-run/evidence/tools-log.txt` (each call's start, UTC with milliseconds, and round trip). Answers that carry the company number or a 10+ digit run (the IMPS references) stay on the lab PC (`kept-local.txt`).
- `answers/marks/`: the raw marks answers m0, m0b, s2, F, B1, E1, s6, E2, before calls 2 and 3, m3 and m4, with their headers.
- `box/`: each post's check output and times; the box logs themselves name the company number and stay on the lab PC (`kept-local.txt`); `box/selftest/`: the helper's self-test results.
- `calls/`: every argument file (lab paths masked).
- `evidence/`: marks, send log, BK-E record, restore plan, company folder counts, the C2 records (netstat as an excerpt).
- `data-folder.txt`: Bridge's data folder by file name, size and sha256 (the folder stays on the lab PC).
- `notes.txt`: UTC, one line per step. `SCREENSHOTS.txt`, `kept-local.txt`, `substitutions.txt`, `scripts/` and `SHA256.txt`.
