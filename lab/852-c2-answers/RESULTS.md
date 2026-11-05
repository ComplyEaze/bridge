# 852-C2: two 50-voucher bank batches posted behind the capture relay with Tally's screen held in the same company (results)

**Not reproduced with this screen work; one of the two kinds of work was not done as the sheet says.** Both posts created 50 vouchers and Bridge kept Tally's reply: `posted_verified` 50 of 50, in the call and again by `verify_import`. The import exchanges took 3.656 s (e) and 2.375 s (f), against C1's 6.59 s and 6.55 s. The whole calls took 33.647 s and 30.755 s, against C1's 49.4 s and 49.7 s. No import read over 20 s, no call over 60 s, and the relay logged no `reply_lost`, `tally_closed` or `relay_error`, with `bridge_left_s` null on every request. **C2a's scrolling was not measured:** the Day Book stayed open in THE COMPANY through the post, but no key reached Tally (a lab miss, below). C2b, a Journal half entered and left on screen through the post, ran as written. Silver, one user, two posts: this does not close #852.

Sheet: `lab/852-c2-screen-work` at d2b5ccca, folder `lab/852-c2/`, SHA256.txt 9 of 9 equal; each batch file checked again when used. `check-relay.py relay.py`: 0 failed (`evidence/check-relay.out`). LAB-COMMON v3. THE COMPANY: BRIDGE LARGE LAB, GUID `2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab`, its number shown as `<company number>`. TallyPrime 7.2 Silver, licensed, education_mode false; Tally's date Wednesday 31-Mar-2027, the PC's 5 Nov 2026. Build: BUILD-M, zip sha256 1940bd97... (= the digest), `bridge_mcp.exe` 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0 (= mcpb-smoke.json's) before and after. Client: the 852-C1 inbox stdio client (sha256 0ed0bb03...), no cancel, no call limit; writes on, batch post and redaction unset, one new empty data folder, port 9002 (the relay), one `bridge_mcp` (pid 11856) for steps 2 to 6.

## What to report

| | C2a (e) | C2b (f) |
|---|---|---|
| requests before the import, their bytes and their summed `answer_s` | 144; sent 188,108 B, answered 29,199,405 B; 6.233 s | 144; sent 188,108 B, answered 29,200,521 B; 4.763 s |
| the import line's `sent`, `first_byte_s`, `answer_s`, `bridge_left_s` | import-1: 66,816 B sent at 09:19:53.565Z; 3.656; 3.656; null (answered 548 B) | import-2: 66,816 B sent at 09:25:58.234Z; 2.375; 2.375; null (answered 548 B) |
| Tally's counters, from `import-N.answer` | CREATED 50, ALTERED 0, DELETED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 30218, no LINEERROR | CREATED 50, ALTERED 0, DELETED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 30268, no LINEERROR |
| any `reply_lost`, `tally_closed`, `relay_error` | none | none |
| requests after the import, bytes, seconds | 73; sent 96,756 B, answered 13,041,051 B; 2.953 s | 73; sent 96,756 B, answered 13,041,467 B; 2.621 s |
| each call's start, end, seconds; box shown at; Yes at | 09:19:34.878Z to 09:20:08.525Z, 33.647 s; box 09:19:50.438Z; Yes 09:19:51.099Z (0.7 s) | 09:25:40.869Z to 09:26:11.624Z, 30.755 s; box 09:25:54.814Z; Yes 09:25:55.497Z (0.7 s) |
| the screen work: first and last key, and whether it overlapped the import line's `sent` to `answer_s` | **no key reached Tally**: 17 presses skipped (Tally not in front), the first due 09:19:51.865Z, the last 09:20:07.851Z; the Day Book was open and untouched across the import (09:19:53.565Z + 3.656 s) | no key, as the sheet says: the Journal with one line typed stayed on screen untouched from before the call (5a) to after it (5c), across the import (09:25:58.234Z + 2.375 s) |
| result: `created`, `attempt_recorded`, `dispatch` (`state`, `response_state`, `resent`), `error` code and `next_step` | 50; absent; posted_verified, response_clean, false; none; none | 50; absent; posted_verified, response_clean, false; none; none |
| `response_prepared` line in `egress_log` | rows 50, 31,843 B; request_trail sent 218, failed 0, omitted 186, every listed outcome answered | rows 50, 31,852 B; request_trail sent 218, failed 0, omitted 186, every listed outcome answered |
| `verify_import`: status counts | posted_verified 50, every other count 0 | posted_verified 50, every other count 0 |
| Tally's screen before, during (the screenshot of 4a or 5a) and after | the Day Book for 31-Mar-27, 82 rows, the cursor on row 1, before and after (the same view); the Gateway after Escape | the Journal No. 29931, 31-Mar-27, By Lab Vol Expense 1.00 Dr and Tally's offered To Lab Vol Income 1.00 Cr, the cursor on the second line's ledger, before and after; the Gateway after Ctrl+Q |
| marks after | V 30,218 (+50), M 211 | V 30,268 (+50), M 211 |
| C1, beside | 6.59 s exchange; 49.4 s call | 6.55 s exchange; 49.7 s call |

Read against the sheet's table: **row 1, not reproduced with this screen work**, for C2b and for C2a's report held open; both imports were faster than C1's on 838242cb, so the screen work did not slow Tally's import here. What is left: scrolling during an import (C2a's miss), a Gold book, another user, a save during the import.

## The other steps

| step | what | result |
|---|---|---|
| 0 | m0 | one COMPANY row, this GUID, **V 30,168, M 211** (= EXPECT). |
| 1 | BK-C2, m0b | `TDBK1800_<company number>.001`, 9,199,575 B, sha256 821854b91c2135162e43478faae80ed11ed6329e5883787202da106a92e33f9f, listed 5-Nov-26 2:45 pm. m0b byte-equal to m0. |
| 2 | relay, Bridge | `list_companies`: this GUID, verified_tuple. `tally_status`: 7.2, silver, education_mode false, books_from 20260401. `masters` voucher_types: 24 types; Payment, Receipt, Contra automatic / auto_retain (= EXPECT). Marks equal m0b. |
| 3 | build both | e `bridge-68d7166b-77e3-4710-b3bb-6224f03b48eb`, 50, 14.779 s; f `bridge-54e10054-9a54-43cf-ab54-5b1d952b8a36`, 50, 13.76 s; 7 warnings each (whole in the answers), no refusal. |
| 4c, 5c | the way back | 4c: Escape on the Day Book (a report): the Gateway, no dialog. 5c: Ctrl+Q (below): the Gateway at once, no prompt. |
| 6 | m3 | **V 30,268, M 211** (= EXPECT). `bridge_mcp` stopped by the client's stop, exit code 0. |
| 7 | restore BK-C2, m4 | the two expected prompts only. m4: GUID, **V 30,168, M 211**, the answer byte-equal to m0b's (= EXPECT). |
| 8 | C10 | the company shut; one tally.exe owning 9001, `/status` 200, none loaded. **Relay e still listens on 127.0.0.1:9002, pid 16908**, left running on the run lead's word for the owner to close at the lab visit. |

## Deviations from the sheet, each on the run lead's word

1. **C2a's keys never reached Tally (a lab miss).** The key helper (`keys.ps1` v1) waited for the box helper's "CLICKED Yes", then clicked Tally's title-band position (1478,26). When the box closed the foreground had gone back to the desktop client's window, which lay over that point, so Tally never came to the front; each press due was skipped and logged, as the run lead set (`box/e-p1-keys.txt`: 17 skips, 0 Down, 0 Up). Recorded as "Day Book open in THE COMPANY through the post; no key reached Tally"; scrolling during an import stays unmeasured. **Helper change:** from then on every key step brings Tally forward with `front.ps1` (SwitchToThisWindow), confirms Tally is the foreground window, and only then presses (`keys.ps1` v2; v1 kept). No later step of this row needed timed keys.
2. **Step 4e not done: "stop the relay, start relay f".** Stopping relay e ends a process, against the owner's rule that nothing ends a process. Batch f was posted through relay e instead, so **f's lines are in folder `e`**: f is `import-2.request` / `import-2.answer`, on relay connection 3 (opened 09:25:40.914Z), the call's lines from 09:25:40.915Z to 09:26:11.222Z (218 requests); e is `import-1`, connection 1, 09:19:34.920Z to 09:20:08.198Z (218 requests). No folder `f` exists.
3. **Step 5c: Ctrl+Q, not Esc.** The owner's rule (no Escape on voucher screens) holds over the sheet. Ctrl+Q took Tally straight back to the Gateway with **no prompt** (the quit-voucher prompt the sheet names did not show; nothing was answered). Marks confirm nothing was saved: V +50 for batch f only (30,218 to 30,268), M unchanged.
4. In 5a, after the first line Tally offered the second line itself (To Lab Vol Income, 1.00 Cr) with the cursor on its ledger field; nothing was typed there and nothing saved.

## Lab kit (not the row's variable)

- **The box check** keeps 852-C1's content rules (this company, its GUID, this batch id, 50 vouchers, the plural wording) and reads the box log through 1392-E's guarded read (taken only whole; a refused or cut read read again for at most 30 s; stderr kept; any error writes No). `boxcheck.py` sha256 3863589144206c86e158dbdd7c83518efe18df8bed70d243edf65a80ca26dd02. Self-test before step 4 on copies of 852-C1's saved box logs (a, b) and 1029-R's (r50a, r50b, r25): 11 of 11 right (`box/selftest/selftest-results.txt`), including a log appended in chunks, one held under a lock, and one cut and never finished (No, fail closed). Both live boxes: ALL PASS, Yes by mouse on the box's own button.
- The four desktop-client connector servers were left untouched: no call was made to them from this sitting, and their calls were not measured. Nothing ended a process.

## Files

- `e/`: relay e's folder whole: `relay.jsonl`, `import-1.*` (e) and `import-2.*` (f).
- `agent-run/answers/`: every tool answer whole (000 to 011), with `client-log.txt`, `launch-env.json`, `step0/initialize.json` and `agent-run/evidence/tools-log.txt`; answers that carry the company number stay on the lab PC (`kept-local.txt`).
- `answers/marks/`: m0, m0b, s2, e, f, m3, m4, with their headers.
- `box/`: each post's check output, the key log of 4b and the self-test; the box logs themselves name the company number and stay on the lab PC.
- `evidence/`: `report.txt` (the table's figures, from the files), marks, send log, BK-C2 record, restore plan, company folder counts, C2 records, the posts' printed lines, relay e's console listing.
- `notes.txt`: UTC, one line per step. `SCREENSHOTS.txt`, `kept-local.txt`, `substitutions.txt`, `scripts/` and `SHA256.txt`.
