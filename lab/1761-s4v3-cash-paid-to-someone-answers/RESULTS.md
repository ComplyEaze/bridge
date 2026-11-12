# Results: row 1761-S4 v3, a bank cash withdrawal paid to someone, on a book with one seed Journal, 2026-11-12

**Outcome: a stop at step 4.2. Batch C was built but not posted. The book was restored to this row's BK1 (the masters and the seed).**
- **Steps 0 to 3 each PASS** (below):
  - the seed Journal was keyed and accepted once, with no extra screen and no prompt;
  - the voucher mark moved from null to 1;
  - BK1 was taken after the seed;
  - the bank ledger named as the cash ledger was refused at build, as expected;
  - batch C was built with 4 vouchers.
- **Step 4.2:**
  - `post_import` of C opened its approval window and answered `pending`;
  - the computer-use grant for this build's `bridge_mcp.exe` came back `user_denied`;
  - the lab did not make the sheet's follow-up call after `retry_after_s`;
  - the window then closed. The owner reports that it closed by itself, unseen, and that they did not deny the grant.
- **The run lead ended the item there.** It ordered no follow-up call and the restore of step 6.
- **The marks before the restore were M 214, V 1, equal to V0b:** nothing of C reached the book.
- **After the restore:**
  - M 214, V 1;
  - exactly the seed on 2025-04-01;
  - no voucher listed on 2025-04-08 and 2025-04-09.

**Who acted.** The lab did all of the following on the owner's standing word for synthetic books (2026-10-27 and 2026-10-28):
- the Tally start under C9;
- loading the book;
- the launch with the posting setting present;
- keying the seed;
- the backup;
- the restore;
- the close.

The go-ahead, the decisions and the restore word came from the run lead in chat. The owner was at the lab PC for step 4.

## Record

- **Sheet:** `lab/1761-s4v3-cash-paid-to-someone-sheet` at `c00dc92d1c1395e1386704fd82a367a53b511f9d`, queue item 92 on `lab/queue` at `14a04aa3`. The sheet's three files are equal to its SHA256.txt. The row's copy of `statement.csv` has sha256 `ebca906726de102cb23dcf15767de2cee5bba0ff4c401debd6e7b9fb3bf776ff`, which is equal.
- **Before the sitting, item 4:** owner at PC: yes, by the owner's own word to the run lead.
- **Build:** the sheet's build line, the Windows bundle of the merge-group run at master `de54a0f26e554f574727a0bc4914b0ad6ca88583`. The three hashes are in evidence/build-hashes.txt.
  - `bridge_mcp.exe` sha256 `be931eb0bd0228d5765886491970da712cb91941bc2e0a980049107c4d8267ce` was hashed again by the client before the launch.
  - The server reported `bridge-tally` 0.7.0.
- **Tally:** TallyPrime 7.2, the TallyPrime single-user edition, licensed; no Educational mark showed (C1). The PC date was 2026-11-12.
- **Company:** BRIDGE CASHPAY LAB, synthetic, GUID `b52de336-8552-4d40-9221-60aa64d3c1f1`, row 1761-S4's book. It is not the excluded GUID.
- **Launch:** one stdio launch by the lab's own client, from step 0.1 to the close (server pid 19152, exit code 0). The launch file's keys:
  - `BRIDGE_AGENT_DATA_DIR` (new and empty);
  - `BRIDGE_AGENT_ENABLE_WRITES=true`, set by the lab on the owner's word of 27 and 28 Oct, synthetic books only;
  - `BRIDGE_TALLY_HOST`, `BRIDGE_TALLY_PORT`, `BRIDGE_TERMS_ACCEPTED`.

  `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` were absent.

## Before step 0.1: Tally had closed (C9)

1. At 05:57:17Z step 0 found no `tally.exe`: tasklist had 0 lines, nothing listened on 9001, and GET /status gave curl exit 7.
2. As C9 says, the lab opened Tally once at 05:59:59Z, by a Win32_Process Create of the installed `tally.exe` with its own folder as the working folder (new PID 23160).
3. The demo company loaded by itself. The lab shut it (Company > Shut, Enter on its one row). It filtered Select Company to BRIDGE CASHPAY (one row) and loaded BRIDGE CASHPAY LAB alone.
4. The Gateway showed the period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025, and No Vouchers Entered.
5. Step 0 at 06:01:50Z: one tally.exe (PID 23160) owning 9001, and GET /status 200.
6. The client started, and the state read MARKS was taken once (answers-mcp/answers/000): M 214, V null, 8 ledgers, the same as item 91's last kept marks.
7. Stop and report. The run lead's word was to resume from step 0.1 (deviation 1).

## Checks

| step | what | result |
| --- | --- | --- |
| before | sheet files equal to SHA256.txt; the statement copy equal; the row folder absent before the row | PASS |
| 0.1 | C2: one tally.exe, PID 23160, owning 9001; GET /status 200; THE COMPANY loaded alone; ENABLE_WRITES present; exe hash equal to the build line | PASS |
| 0.2 | `tools/list` (answers 001): 37 tools; `parse_bank_statement_csv` lists `cash_ledger` under `cash_answers` items; `post_import` present | PASS |
| 0.3 | `list_companies`: exactly BRIDGE CASHPAY LAB, its GUID, `verified_tuple` | PASS |
| 0.4 | MARKS: M0 214, V0 null, reused false | PASS |
| 0.5 | `masters` voucher_types complete: Contra and Payment `series_numbering` `observed`, `automatic`, `auto_retain`; `company_features`: `cost_centres` `no` | PASS |
| 0.6 | the fallback (row 1761-S4's BK1): one file, 24241 B, sha256 `bce8c06cc0dcf344bc4408e34e0c8374b9920518387552e916e44e0fc4c23986` | PASS |
| 0.7 | the seed Journal keyed and accepted once by Ctrl+A; no sub-screen, no prompt; Gateway date 1-Apr-2025 | PASS |
| 0.8 | MARKS: M0s 214, V0s 1, reused false | PASS |
| 0.9 | `vouchers` 2025-04-01: complete, total 1, the seed exactly | PASS |
| 0.10 | BK1 (C5): one file, 25676 B, sha256 `dde2e47ffa5d7431d296791ce97429482d8b3e19964c38c39053128cfd8bd15f`, written 2026-11-12 11:39 local; MARKS M0b 214, V0b 1 = M0s, V0s | PASS |
| 1 | parse, no cash answers: statement_rows 2, vouchers 0, cash_questions_open 2, over-limit 0; the two questions as the sheet writes them | PASS |
| 2.1 | parse, Contractor A's cash_ledger BRIDGE Bank B: vouchers 4, cash_questions_open 0 | PASS |
| 2.2 | build with P2: refused, `cash_ledger_not_cash_in_hand`, one `refused_ledgers` row (BRIDGE Bank B, plain; `cash_in_hand`; `cash_bank`; Bank Accounts; the 2025-04-08 txn), omitted 0; no batch_id; no file written | PASS |
| 2.3 | MARKS: M 214, V 1 = M0b, V0b | PASS |
| 3.1 | parse, both cash_ledger Cash: vouchers 4, cash_questions_open 0, over-limit 1 | PASS |
| 3.2 | build with P3: voucher_count 4, batch_id C `bridge-5c53de34-867f-45c9-947d-4b46310c9275`, answered 06:11:13Z | PASS |
| hold | the hold before 4.1 reported to the run lead at once; its go at about 06:12Z | within the limit |
| 4.1 | MARKS: V4a 1 = V0b | PASS |
| 4.2 | `post_import` of C: `pending`; grant `user_denied`; follow-up call not made | **stop** |
| pre-restore | MARKS: M 214, V 1, reused false, 8 ledgers = M0b, V0b: nothing of C posted | recorded |
| 6.1 | restore of BK1 (C6), both expected prompts only | PASS |
| 6.2 | MARKS: M 214, V 1 = M0b, V0b | PASS |
| 6.3 | `list_companies` as step 0; `vouchers` 2025-04-01: complete, exactly the seed | PASS |
| 6.4 | `vouchers` 2025-04-08 to 2025-04-09: partial, total 0, `empty_uncorroborated`, no voucher listed | recorded |
| C10 | client closed (exit code 0); Shut Company; one tally.exe owning 9001; GET /status 200; no company loaded | PASS |

## Step 0.7, the seed as keyed (evidence/notes.txt)

1. Before F7 the Gateway's current date read Tuesday, 1-Apr-2025.
2. Vouchers opened Accounting Voucher Creation as Payment No. 1. F7 switched it to Journal No. 1, in the double-entry layout (a By line first).
3. The List of Ledger Accounts panel covered the header's date. Behind the F2 box, the dimmed header read 1-Apr-25, Tuesday: the date the screen opened with.
4. F2 (Change Voucher Date) showed `2-4-2025` in its field. The lab typed `1-4-2025` and pressed Enter, and the header read 1-Apr-25, Tuesday.
5. Line order: By BRIDGE Suspense, debit 1,000.00, with no sub-screen (its current balance on screen 1,000.00 Dr). Then To, BRIDGE Capital, whose credit came up filled at 1,000.00. Enter, with no sub-screen (its current balance on screen 51,000.00 Cr).
6. Narration `seed for row 1761-S4 v3`, typed word by word with the space key (typed text loses its spaces here).
7. The screenshot before the accept showed Journal No. 1, 1-Apr-25, both ledgers, 1,000.00 each, and the narration.
8. Ctrl+A once: no message and no prompt. The screen became a blank Journal No. 2 (no ledger, no amount, no narration).
9. Following the run lead's word before the first write (deviation 2): a screenshot of that title, then Ctrl+Q once. **No quit prompt appeared.** Tally went straight to the Gateway with BRIDGE CASHPAY LAB loaded and no dialog. The current date read Tuesday, 1-Apr-2025, and the date of last entry 1-Apr-25.

The seed read back at step 0.9 (answers 007): Journal No. 1, 2025-04-01, BRIDGE Suspense `-1000.00` (deemed positive Yes, so debit), BRIDGE Capital `1000.00` (deemed positive No, so credit), narration 23 characters, alter_id 1, not cancelled, optional or post-dated.

## Step 4.2, as it happened

1. 06:12:14Z: MARKS (4.1), V4a 1.
2. 06:12:23Z: `post_import` with THE COMPANY's GUID and batch_id C (answers 016), sent in the background so the lab could ask for the grant while the window was open.
3. Before the post, `request_access` for `bridge_mcp.exe` by that name showed no dialog: the name matched no installed or running application. It offered "Bridge", another program, which was not taken. Retried by name once the window was open: the same.
4. 06:12:45Z: a window listing found the approval box open, class `#32770`, titled "ComplyEaze Bridge — post 4 vouchers?". It was owned by the server's child process at this build's path.
5. About 06:13Z: `request_access` by the build's full exe path came back `user_denied`, and nothing was granted. The owner later reported that the grant dialog closed by itself, unseen, and that they did not deny it.
6. 06:13:03Z: the post answered after 40.1 s. `approval.state` `pending`, `retry_after_s` 15, `dialog_remaining_s` 86, `attempt_recorded` false, `dispatch.state` `not_dispatched`, `previous_approval` null.
7. **The lab did not make the sheet's follow-up call** (wait `retry_after_s`, then `post_import` again with C). It was on the grant request. This is a run fact.
8. 06:18:07Z: the child process was gone, so the approval box had closed. The server's stderr log was 0 B. Whether the box was answered or timed out was not read.
9. Stop, reported to the run lead. Its decision: end at the stop, with no follow-up call (it would not route around a denied grant with another call); one read-only marks read before the restore; then restore BK1 as step 6 writes it.
10. 06:18:52Z: MARKS (answers 017), M 214, V 1, reused false, 8 ledgers. V = V0b, so no voucher of C reached the book.

## Step 6, the restore (C6)

1. Before the shut, `restore-plan.txt` was written: folder, file, bytes, sha256, the company number, the Restore Data Path, and the two prompts expected. BK1 was hashed again: equal, one file in its folder. A listing of the company's data folder was taken (evidence/c6-datafolder-before.txt).
2. Company > Shut, then Data > Backup & Restore > Restore. The Restore Data Path read Tally's data folder. Source: Specify Path, the BK1 folder. The list showed one company, BRIDGE CASHPAY LAB with its number and 12-Nov-26 (11:39 am), as planned, and it was selected alone.
3. "Accept ?" = Yes. Then "Company data folder (`<company number>`) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." = Replace. No other prompt appeared.
4. Back at Select Company within about 5 s. Filtered to one row and loaded: the Gateway showed BRIDGE CASHPAY LAB as the one company, no dialog, current date Tuesday, 1-Apr-2025.
5. The data folder listing after the restore is in evidence/c6-datafolder-after.txt: a=25, b=21 entries before and after, the side files a restore drops, as earlier rows record.
6. Reads, MARKS first, as in the table above.

## The book after the row

As at BK1: the masters and the seed. M 214, V 1. Batch C was never posted; its file stays in the data folder (data/imports).

## Deviations

1. **The C9 start and resume** (the run lead's word, before the first write). Tally was down at step 0. The lab started it once and shut the demo company, then loaded THE COMPANY alone. Step 0 passed at 06:01:50Z, and the marks matched item 91's last kept marks. The run lead's word was to resume from step 0.1.
2. **The quit prompt after the seed** (the run lead's word, before the first write). If a "Quit?" prompt came on the blank Journal No. 2 after Ctrl+A, the lab could answer Yes by clicking Y once, after a screenshot. That held only on a blank form; any other prompt stayed a stop. No prompt came, so nothing was answered.
3. **The follow-up post call was not made** (step 4.2, above). The run lead ended the item at the stop.
4. **The run lead's expectation for the restore, corrected.** Its note first expected item 91's kept marks after the restore (M 214, V null), which only the fallback gives. The lab asked before restoring. The run lead's word: its note was an error; restore this row's BK1 as step 6 writes it, and EXPECT M 214, V 1. The sheet was not changed.

## Not established

- The approved post of C and everything that depends on it: `dispatch`, `post_location`, `post_span_binding`, `verify_import`, the read-back of the four vouchers and their narrations, and the Cash and BRIDGE Bank A movements.
- Whether the approval box of 06:12Z was answered, and how, or whether it timed out.
- Why the computer-use grant came back `user_denied`. The owner reports they did not deny it.
- Whether keying a voucher moves M: M stayed 214 through the seed, but only one voucher was keyed.

The run lead plans a re-run as a new item: from row 1761-S4's masters-only BK1, with the grant approved before the post.

## Files

- **answers-mcp/:** every tool answer whole (company_number redacted where present), the calls sent, the client's log and `initialize`. Answers naming the edition word are kept on the lab PC (kept-local.txt).
- **args/:** the call arguments.
- **data/:** the server's data folder, including C's import file.
- **evidence/:**
  - notes.txt, one line per step, UTC;
  - build-hashes.txt;
  - the data-folder listings before and after the restore;
  - the port listings cut to the port-9001 LISTENING lines.
- **restore-plan.txt:** the C6 plan.
- **statement.csv:** the row's copy.
- **scripts/:** the lab's helpers.
- **kept-local.txt and SCREENSHOTS.txt:** what stays on the lab PC, by size and sha256.
- **substitutions.txt:** what changed in pushed copies of the lab's own text.
