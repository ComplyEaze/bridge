# Results: row 1761-S4 v4, a bank cash withdrawal paid to someone, on a book with one seed Journal, 2026-11-18

**Outcome: every step PASS. Batch C was built, approved in its window, posted once and verified; the four vouchers read back exactly as the sheet's table; the book was restored to this row's BK1 (the masters and the seed).**
- **Step 0:** a demo book loaded at the start was shut by Tally's own Shut Company (deviation 1); the fallback was restored and THE COMPANY loaded alone (M0 214, V0 null); the seed Journal was keyed on the second try (deviation 2) and accepted once by Ctrl+A with no sub-screen and no prompt; V moved from null to 1; BK1 was taken after the seed.
- **Step 1 and step 2:** the parse with no cash answers left two cash questions open; the bank ledger named as the cash ledger was refused at build (`cash_ledger_not_cash_in_hand`) and nothing was written.
- **Step 3:** batch C was built with 4 vouchers.
- **Step 4:** `post_import` of C answered `pending`; the approval window was listed open and was answered by the owner by mouse; the one follow-up call, at `retry_after_s` + 1 s, answered `posted_verified`, created 4, step 4, `bound`; `verify_import` agreed.
- **Step 5:** V 5 = V4a + 4; the two Contra and two Payment vouchers on 2025-04-08 and 2025-04-09 exactly as the sheet's table; Cash in 15,000 and out 15,000; BRIDGE Bank A 50,000.00 Dr to 35,000.00 Dr.
- **Step 6:** BK1 restored with the two expected prompts only; M 214, V 1, exactly the seed on 2025-04-01, no voucher listed on 2025-04-08 and 2025-04-09.

**Who acted.** The lab did the Tally reads and keys, the seed, the backup, the restore and the close on the owner's standing word for synthetic books (2026-10-27 and 2026-10-28). The owner was at the lab PC for step 4 and answered the approval window; the lab never touched that window. The go-ahead, the decisions and the restore word came from the run lead in chat.

## Record

- **Sheet:** `lab/1761-s4v4-cash-paid-to-someone-sheet` at `e85b1d759ff41949040ec88d238dfc772d42531e`, queue item 93. The sheet's three files are equal to its SHA256.txt. The row's copy of `statement.csv` has sha256 `ebca906726de102cb23dcf15767de2cee5bba0ff4c401debd6e7b9fb3bf776ff`, which is equal.
- **Build:** the sheet's build line, the same Windows bundle as v3 (evidence/build-hashes.txt). `bridge_mcp.exe` sha256 `be931eb0bd0228d5765886491970da712cb91941bc2e0a980049107c4d8267ce` was hashed again by the client before the launch. The server reported `bridge-tally` 0.7.0.
- **Tally:** TallyPrime 7.2, edition as recorded, licensed (build_import_xml's `observed_profile`: release 7.2, mode Licensed). The PC date was 2026-11-18.
- **Company:** BRIDGE CASHPAY LAB, synthetic, GUID `b52de336-8552-4d40-9221-60aa64d3c1f1`, row 1761-S4's book. It is not the excluded GUID.
- **Launch:** one stdio launch by the lab's own client, from step 0.1 to the close (server pid 13672, exit code 0). `BRIDGE_AGENT_DATA_DIR` new and empty; `BRIDGE_AGENT_ENABLE_WRITES` present, set by the lab on the owner's word of 27 and 28 Oct, synthetic books only. The launch files stay on the lab PC (kept-local.txt).

## Checks

| step | what | result |
| --- | --- | --- |
| before | sheet files equal to SHA256.txt; the statement copy equal; the row folder absent before the row; the fallback checked read-only | PASS |
| C1 | a demo book loaded at the start, not THE COMPANY; nothing sent or keyed to it; shut by Company > Shut on the recorded word (deviation 1) | recorded |
| 0.1 | C2: one tally.exe, PID 23160, owning 9001; GET /status status=200; THE COMPANY loaded alone; ENABLE_WRITES present; exe hash equal to the build line | PASS |
| 0.2 | `tools/list` (answers 000): 37 tools; `parse_bank_statement_csv` lists `cash_ledger` under `cash_answers` items; `post_import` present | PASS |
| 0.3 | `list_companies` (001): exactly BRIDGE CASHPAY LAB and its GUID | PASS |
| 0.4 | MARKS before (002): M 214, V 1; the fallback restored (C6); MARKS (003): M0 214, V0 null, reused false | PASS |
| 0.5 | `masters` voucher_types (004): Contra and Payment `series_numbering` `observed`, `automatic`, `auto_retain`; `company_features` (005): `cost_centres` `no` | PASS |
| 0.6 | the fallback (row 1761-S4's BK1): one file, 24241 B, sha256 `bce8c06cc0dcf344bc4408e34e0c8374b9920518387552e916e44e0fc4c23986`, equal | PASS |
| 0.7 | first try stopped before the accept on a keying slip; left by Ctrl+Q with no prompt, MARKS (006) V null; second try keyed and accepted once by Ctrl+A, no sub-screen, no prompt (deviation 2) | PASS |
| 0.8 | MARKS (007): M0s 214, V0s 1, reused false | PASS |
| 0.9 | `vouchers` 2025-04-01 (008): complete, total 1, the seed exactly | PASS |
| 0.10 | BK1 (C5): one file, 25675 B, sha256 `79be45dd05c25c1eeb16b94f4a9dc76de4e1dbc78f03a48b48909f16db26a9c0`, written 2026-11-18 12:48 local; MARKS (009) M0b 214, V0b 1 = M0s, V0s | PASS |
| 1 | parse, no cash answers (010): statement_rows 2, vouchers 0, cash_questions_open 2, over-limit 0; the two questions as the sheet writes them | PASS |
| 2.1 | parse, Contractor A's cash_ledger BRIDGE Bank B (011): vouchers 4, cash_questions_open 0, over-limit 1 | PASS |
| 2.2 | build with P2 (012): refused, `cash_ledger_not_cash_in_hand`, one `refused_ledgers` row (BRIDGE Bank B, plain; `cash_in_hand`; `cash_bank`; Bank Accounts; the 2025-04-08 txn), omitted 0; no batch_id; the data folder's file count unchanged | PASS |
| 2.3 | MARKS (013): M 214, V 1 = M0b, V0b | PASS |
| 3.1 | parse, both cash_ledger Cash (014): vouchers 4, cash_questions_open 0, over-limit 1 | PASS |
| 3.2 | build with P3 (015): voucher_count 4, batch_id C `bridge-3e3155ae-6645-4734-92ce-4293bb765b23`, file sha256 `3a504c50102491b9992b9822e4eab370280dd083d5863ce59336895831b24585`, totals 30000/30000, answered 07:20:17Z | PASS |
| hold | from 07:20:17Z to the step 4.0 go; step 4.1's read at 07:25:25Z | within the limit |
| 4.1 | MARKS (016): V4a 1 = V0b, M 214 | PASS |
| 4.2 | `post_import` of C (017): `pending`, `retry_after_s` 15, `dialog_remaining_s` 86, `attempt_recorded` false, `not_dispatched`; the one follow-up (018) at `retry_after_s` + 1 s | PASS |
| 4.4 | (018): `dispatch.state` `posted_verified`; counters created 4, every other 0; `last_vch_id` 5; `post_location` `target_only`, `target_voucher_step` before 1, after 5, step 4, `matches_created` true; `post_span_binding` `bound` | PASS |
| 4.5 | `verify_import` (019): `posted_verified`, `bound`; no `rollback_recorded`, no `voucher_mark_contradiction` | PASS |
| 5.1 | MARKS (020): M 214, V 5 = V4a + 4 | PASS |
| 5.2 | `vouchers` 2025-04-08 to 2025-04-09 (021): complete, total 4, each ledger, amount and narration exactly as the sheet's table, no other voucher (evidence/s5-voucher-table.txt) | PASS |
| 5.3 | `ledger_movement` Cash 2025-04 (022): complete, opening 0.00, in 15000, out 15000, closing 0, 4 vouchers | PASS |
| 5.4 | `ledger_movement` BRIDGE Bank A 2025-04 (023): complete, opening 50,000.00 Dr, credit 15000, closing 35,000.00 Dr, 2 vouchers | PASS |
| 6.1 | restore of BK1 (C6), the two expected prompts only | PASS |
| 6.2 | MARKS (024): M 214, V 1 = M0b, V0b, reused false | PASS |
| 6.3 | `list_companies` (025) as step 0; `vouchers` 2025-04-01 (026): complete, exactly the seed | PASS |
| 6.4 | `vouchers` 2025-04-08 to 2025-04-09 (027): partial, total 0, `empty_uncorroborated`, no voucher listed (V = V0b is the proof) | recorded |
| C10 | client closed (exit code 0); Shut Company; one tally.exe owning 9001; GET /status status=200; no company loaded | PASS |

## Step 0.7, the seed as keyed (evidence/notes.txt)

1. Before F7 the Gateway's current date read Tuesday, 1-Apr-2025. The voucher screen opened already as Journal No. 1 (F7 shown greyed), so F7 was not pressed.
2. F2 showed `2-4-2025` in its field. The lab typed `1-4-2025` and pressed Enter, and the header read 1-Apr-25, Tuesday. The layout was double-entry (a By/To column before each ledger).
3. Line 1: By BRIDGE Suspense, debit 1,000.00, no sub-screen.
4. First try: on line 2 the cursor was on the line's To/By column, not the ledger. Typing the ledger name put its first letter there; a "To / By" list opened with By highlighted, and an Error panel read "Error", "Oops!", "Spelling Error". No key was pressed; stop, reported.
5. The recorded word, way (a): leave by Ctrl+Q, answer only Tally's own quit question once, MARKS once, then key the seed again from F2; one letter in the To/By column, read by zoom before the ledger name; a second Error panel or an unnamed prompt would be a stop with no third try.
6. Ctrl+Q once: back at the Gateway **with no prompt**; MARKS (006) V null, so nothing was saved.
7. Second try: F2 as before; By BRIDGE Suspense, debit 1,000.00; line 2's To/By column: `T` typed, zoom read `T`, Enter, zoom read `To` with the ledger field open; To BRIDGE Capital, credit prefilled 1,000.00, Enter; narration `seed for row 1761-S4 v4`.
8. Ctrl+A once: no message, no sub-screen. A blank Journal No. 2 opened and was left by Ctrl+Q with no prompt. The Gateway showed THE COMPANY alone, current date 1-Apr-2025, last entry 1-Apr-25.

The seed read back at step 0.9 (answers 008): Journal No. 1, master_id 1, 2025-04-01, BRIDGE Suspense `-1000.00` (deemed positive Yes, so debit), BRIDGE Capital `1000.00` (deemed positive No, so credit), narration 23 characters.

## Step 4, as it happened

1. 07:25:25Z: MARKS (4.1), V4a 1.
2. The owner was told that a window would open; `winlist.ps1` started, listing windows only (no click, key, focus or capture).
3. 07:25:35.942Z: `post_import` with THE COMPANY's GUID and batch_id C (answers 017).
4. 07:25:43.147Z: the window listing found the approval box: class `#32770`, titled "ComplyEaze Bridge — post 4 vouchers?", owned by `bridge_mcp.exe` pid 14268 (parent 13672, the server), topmost (evidence/window-s4.txt).
5. 07:26:16.142Z: the post answered: `approval.state` `pending`, `retry_after_s` 15, `dialog_remaining_s` 86, `attempt_recorded` false, `dispatch.state` `not_dispatched`.
6. 07:26:31.320Z: the box was no longer listed. The owner answered it Yes by mouse.
7. 07:26:32.373Z: the one follow-up `post_import` with C (answers 018), `retry_after_s` + 1 s after the pending answer. It answered at 07:26:44.279Z: `posted_verified`, created 4, `last_vch_id` 5, `bound`. No further `post_import` was made.
8. Screenshot after the post: the Gateway, no dialog. 07:26:56Z: `verify_import` (019) agreed.

## Step 6, the restore (C6)

1. On the restore word, with the owner at the PC: before the shut, `restore-plan-06.txt` was written and BK1 hashed again (one file, 25675 B, `79be45dd...`, equal). A listing of the company's data folder was taken (22 entries; kept on the lab PC).
2. Company > Shut, then Data > Backup & Restore > Restore. The Restore Data Path read Tally's data folder. Source: Specify Path, the BK1 folder. The list showed BRIDGE CASHPAY LAB with its number and 18-Nov-26 (12:48 pm), selected alone.
3. "Accept ?" = Yes. Then the "Company data folder (`<company number>`) already exists ... Replace" prompt = Replace. No other prompt appeared.
4. Back at Select Company; THE COMPANY loaded alone, no dialog, current date 1-Apr-2025, last entry 1-Apr-25. The listing after had 15 entries: the side files a restore drops, as earlier rows record.
5. Reads, MARKS first, as in the table above.

Step 0.4's restore of the fallback ran the same way (`restore-plan-04.txt`; the list showed 11-Nov-26 (3:05 pm); the same two prompts).

## The book after the row

As at BK1: the masters and the seed. M 214, V 1. Batch C's file and its proofs stay in the server's data folder (data/imports).

## Deviations

1. **The demo book at the start (C1).** At 07:06Z the Gateway showed one company loaded that was not THE COMPANY: a demo book (its name kept on the lab PC), period 1-Apr-26 to 31-Mar-27. Nothing was sent or keyed to it. Stop, reported. The recorded word: shut it by Tally's own Shut Company only. At 07:08Z Company > Shut, Enter on its one row; no prompt; Select Company with no company loaded. It was not loaded again in the row.
2. **The keying slip at step 0.7** (above). Way (a) on the recorded word; the second try met every check.
3. **The voucher screen opened as Journal** (step 0.7), so F7 was not pressed. The sheet's seed step allows the double-entry layout; it does not say that each new line starts on its To/By column.

## Limits

- The approval box's answer is known from the owner's account and from the follow-up's `posted_verified`; the lab listed the window only and did not read its contents.
- notes.txt writes the go as received "at about 07:26Z"; step 4.1's read at 07:25:25Z shows the go came before that, so the time is approximate.
- M stayed 214 through the seed and the post: in this row MARKS' M did not move on vouchers keyed or posted.
- One sitting, one book, one release, one statement.

## Files

- **answers-mcp/:** every tool answer whole (company_number redacted where present), the calls sent, the client's log and `initialize`. Answers holding a value that stays local are kept on the lab PC (kept-local.txt).
- **args/:** the call arguments.
- **data/:** the server's data folder, including C's import file and its proofs; files holding the company number stay local.
- **evidence/:** notes.txt, one line per step, UTC; build-hashes.txt; tools-log.txt; s5-voucher-table.txt; window-s4.txt; the port listings cut to the port-9001 LISTENING lines.
- **restore-plan-04.txt and restore-plan-06.txt:** the C6 plans.
- **statement.csv:** the row's copy is kept on the lab PC (it holds a run of ten or more digits); its sha256 is equal to the sheet's copy (Record above).
- **scripts/:** the lab's helpers.
- **kept-local.txt and SCREENSHOTS.txt:** what stays on the lab PC, by size and sha256.
- **substitutions.txt:** what changed in pushed copies of the lab's own text.
