# #1211: `statement_tie_out` against a live bank ledger, before and after a post: a window ending mid-month, a second bank ledger, and a statement that starts and ends mid-month posted by ComplyEaze Bridge itself

Not for merge. One item, three rows (R1, R2, R3), on a company the lab already holds. `LAB-COMMON.md` in this folder (v3, a byte copy of the one in `lab/0.7.0-rc-sitting`) applies in full; where this sheet and it differ, the stricter holds. Steps marked [OWNER] are the owner's (keying in Tally, and answering the approval window by mouse).

**WHY.** #1211 shipped `statement_tie_out` in 0.6.0, but it has been measured only by one run on 14 October 2026 (`lab/1628-m-month-window-tie-out-answers`), on TallyPrime 7.1, and one partly recorded run on 10 October (`lab/1215-tie-out-evidence`). Every ledger read of the 14 October run fell on the 1st of a month; the 10 October run read once at a mid-month end date (7 August) and kept no answers. Every post in both was a hand import, and the book held one bank ledger. This item measures what is left:
- **R1:** after a post, a window that ends in the middle of a month gives a typed answer with no amount and no ledger read.
- **R2:** with a second bank ledger in the book, the tie-out stays tied while the bank ledger matches its statement; Tally's own figures equal the statement's; and an entry off the statement shows as the gap it is.
- **R3:** a whole statement whose first and last rows fall mid-month (so the ledger is read at the 3rd and the 29th), checked before the build, posted by ComplyEaze Bridge's own `post_import`, and checked again after the post. The book around it is set so that a read on the wrong day, or a missed entry, shows as a gap.

A book of 1,000 or more vouchers is not covered here.

## THE COMPANY

BRIDGE TIE OUT LAB 3: synthetic, company number 100065, GUID `9c97970c-e25d-4348-9740-6ddb1caaa56d`, books from 1 April 2026. As the 30 October 2026 sitting left it (`lab/0.7.0-rc-sitting-answers`, `R1/RESULTS.md`, restored from its BK1):
- marks 210 / 8;
- a bank ledger Union Bank (opening 10,000.00 Dr), a Suspense ledger `Suspense`, an income ledger and one seed Receipt of 1 June 2026;
- July's and August's statement vouchers, imported by hand on 14 October;
- nothing on Union Bank in September;
- no ledger `BRIDGE Bank B`.

If THE COMPANY is not on the lab PC: STOP. Never create a substitute.

## THE STATEMENTS

- **STATEMENT A** is the synthetic three-month PDF of the 14 October run, kept on the lab PC: sha256 `506e05ebfb0f81d6e0fa4b0cc867bbaeae8a69d130c02615ea48a86559894fc9`. Its rows run from 1 July to 30 September 2026.
- **STATEMENT B** is `statement/ubi-1211-r3.pdf` in this folder: 2,482 bytes, sha256 `33609063f8d64e2d3136081f1a175343011434befff4f319a33d6f905d2c650a`. `python3 -I statement/make-mid-month-statement.py <out.pdf>` (no other arguments) writes it, deterministically and from the standard library only, with the layout of the repository's `ubi-synthetic.pdf`. That layout comes from the fixture generator as it stood before its removal (`scripts/generate-bank-statement-fixtures.py` at the parent of c9e0ba21, which the script loads from beside itself; the lab does not need to run it, since the PDF is committed with its hash). Its user password is `synthetic-user-7788`, the published fixture password.
  - It has four rows:
    - 3 Sep, 2,500.00 Cr;
    - 9 Sep, 1,200.00 Dr;
    - 17 Sep, 650.00 Cr;
    - 28 Sep, 300.00 Dr.
  - It opens at 12,300.00 Cr: the 11,800.00 Union Bank holds at the end of August, plus the 500.00 Receipt R3 keys on 2 September (H1). It closes at 13,950.00 Cr.

Check each statement's full hash before every parse. A different file = STOP.

## THE BUILD

The published release `mcp-v0.7.0` Windows package, as the 30 October post-publish check staged it on the lab PC (`lab/0.7.0-post-publish-answers`):
- `bridge-tally-mcp-v0.7.0-windows-x64.mcpb`: 15,598,419 bytes, sha256 `291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec`;
- its `bin/x86_64-pc-windows-msvc/bridge_mcp.exe`: sha256 `48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d`.

Record the exe's sha256 at the start and at the end of each launch; any other hash = STOP.

Two launches over MCP stdio share ONE new empty `BRIDGE_AGENT_DATA_DIR` for the whole item. Both set `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use) and `BRIDGE_AGENT_ENABLE_IMPORT=true` (`parse_bank_statement` refuses without it). `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_ENABLE_BATCH_POST` stay unset.
- **LAUNCH A** (step 0, R1, R2): `BRIDGE_AGENT_ENABLE_WRITES` unset. Posting is NOT set.
- **LAUNCH B** (R3 only): `BRIDGE_AGENT_ENABLE_WRITES=true`. LAUNCH A is ended (its stdin closed, its exit code recorded) before LAUNCH B starts.

## THE CALLS

Every read in this item is a call of the build's own tools, one at a time, the one dispatch path to Tally. No raw request is sent, apart from LAB-COMMON's GET /status checks (C2 c and C10).

**The marks** are read with `ledger_masters` (first page, no `snapshot_id`):
- M = `snapshot.master_alter_id`;
- V = `snapshot.voucher_alter_id`;
- `snapshot.reused` must read false.

**Tally's own figures per ledger** are read with `trial_balance` from `2026-04-01` (the books' first day) to the date named. Record each named ledger's row whole.
- Its amounts are in Tally's sign (a debit balance is negative, reference §5.6): compare `closing.value` numerically.
- A `present_empty` amount is "not established".
- A refusal is recorded whole as a RESULT, and the EXPECT that needs it is reported "not established".

**`parse_bank_statement`.** For STATEMENT A it takes the arguments the 30 October R1 row used:
- bank `ubi`;
- account label `Lab UBI 7788`;
- opening `10000.00`, closing `11000.00`;
- bank ledger `Union Bank`, suspense ledger `Suspense`;
- the password file on the lab PC.

Only `from` and `to` change, as each step says. For STATEMENT B it takes the same bank, account label and ledgers, a password file on the lab PC holding `synthetic-user-7788` (with the permissions the tool requires), opening `12300.00` and closing `13950.00`, and no `from` or `to`.

**`statement_tie_out`.** Called with `company_guid`, the `proposals_id` and `proposals_sha256` = that parse's `proposals_id` and `sha256`, and the `stage` the step names.
- Its three figures (`opening_gap`, `closing_gap`, `change_in_window`) are in the statement's sign: book less statement, so positive means the book shows more money in the bank than the statement.
- Each figure is `tied`, `differs` with an `amount`, or `not_established` with a `reason`.
- Paths below are inside `structuredContent.result` of the tool's answer.
- An answer's `next_step` or `next_steps` is recorded, never acted on: this sheet's steps decide every call.

**After every `statement_tie_out` call,** record from `egress_log` the line that call added and its request trail. The trail holds the number and kinds of requests, never their dates: the dates a call read are its answer's `window`.

## Before the sitting

1. [OWNER] Tally is LICENSED. Educational mode = STOP (C1). The reads of R3 fall on days that Educational mode does not admit.
2. Check every file here against `SHA256.txt`, and STATEMENT A against its hash above.
3. [OWNER] Go-ahead.

## Step 0 and the book as recorded (LAUNCH A)

C2 step 0 with THE COMPANY loaded alone, without a MARKS request file (its GET /status checks stay). Then:
1. `tools/list`: record the answer whole. EXPECT `statement_tie_out` listed and `post_import` NOT listed. Otherwise STOP.
2. `list_companies`: EXPECT exactly one company, BRIDGE TIE OUT LAB 3, with the GUID above and `books_from` 20260401. Anything else = STOP. Every later call uses this `company_guid`.
3. MARKS: M0, V0. EXPECT 210 and 8. Other values = STOP (the book is not as recorded).
4. `ledger_movement` for Union Bank, 2026-07-01 to 2026-09-30. EXPECT opening -10000.00 and closing -11800. Any other figure = STOP.
5. `tally_status`: record the answer whole, with Tally's release and edition. Every earlier tie-out run was on TallyPrime 7.1; the release this item ran on goes into `RESULTS.md`.

Then backup **BK1** (C5) into a NEW empty folder `C:\bridge-lab\1211-evidence\backups\BK1\`. Record its name, size and sha256, then MARKS. Every row below ends restored to BK1.

The book: unchanged.

## R1: windows ending mid-month, after the post (reads only, LAUNCH A)

1. `parse_bank_statement` on STATEMENT A with `from` 2026-08-01 and `to` 2026-08-15. Save the answer. Then `statement_tie_out` for it, `stage` `after_post`. Save the answer.
2. The same two calls with `from` 2026-07-01 and `to` 2026-08-15: a window that starts at a month boundary and ends mid-month, over two posted months.
3. **Control:** the same two calls for the whole of July (2026-07-01 to 2026-07-31).

EXPECT:
- **Steps 1 and 2:** `opening_gap`, `closing_gap` and `change_in_window` each `not_established` with `reason` `window_not_whole_calendar_months`, and no `amount`. `evidence.state` `partial` with `reason_code` `window_not_whole_calendar_months`.
  - The tool answers these from the file once the company is verified, and reads no ledger. The request trail gives counts and kinds, not dates. EXPECT its count to be that of the company check alone: on 14 October such a call sent 4 requests, against about 70 for a full tie-out. Record the count.
- **Step 3:** `tied` 0.00 three times, as on 14 October.

The finding is an error (no typed result), any amount on a part-month window, or a ledger read sent for one.

The book: unchanged.

## R2: a second bank ledger, and Tally's own figures (hand writes, then restored; LAUNCH A)

1. [OWNER] Create ledger `BRIDGE Bank B` under Bank Accounts, with no opening balance.
   - Screenshot it before the accept.
   - After the accept, leave the creation screen as C7 says: a screenshot showing its title, then Ctrl+Q.
   - Then a screenshot of the Gateway of Tally with THE COMPANY loaded and no dialog.
   - MARKS: M1 (EXPECT above M0), V1 = V0.
2. [OWNER] Key Receipt W1, dated 15-07-2026: BRIDGE Bank B Dr 700.00, `Suspense` Cr. W1 does not touch Union Bank. Screenshot before the accept. MARKS: V2 = V1 + 1.
3. `statement_tie_out` for July and for August, each from a parse of STATEMENT A for that whole month, `stage` `after_post`.

   EXPECT `tied` 0.00 three times for each: the bank ledger still matches its statement.
4. `trial_balance` from `2026-04-01`:
   - to `2026-06-30` (Union Bank on the eve of July);
   - to `2026-07-31` (the end of July);
   - to `2026-08-31` (the end of August).

   EXPECT (Tally's sign, compared numerically):
   - Union Bank's closing to 30 June = -10000.00.
   - Union Bank's closing to 31 July = minus `window.tie_out.closing_balance` of the July parse's proposals file. That is `bank-statements/<proposals_id>.json` in the data folder; read the file as JSON. Record both figures.
   - Union Bank's closing to 31 August = -11800.00.
   - BRIDGE Bank B's closing to 31 July = -700.00 (700.00 Dr).

   `statement_tie_out` returns gaps, not the book's balances, so these figures are compared with Tally's own and with the statement's, not with a figure the tool reports.

   Then Tally's ledger display (as R3 describes it) of Union Bank, 1-7-2026 to 31-8-2026: EXPECT Opening Balance 10,000.00 Dr and Closing Balance 11,800.00 Dr.
5. [OWNER] Key Contra W2, dated 20-08-2026: BRIDGE Bank B Dr 1,000.00, Union Bank Cr. It is not on the statement. Screenshot before the accept. MARKS: V5 = V2 + 1.
6. `statement_tie_out` for August (the whole month), `stage` `after_post`, again. Then `trial_balance` from `2026-04-01` to `2026-08-31` again.

   EXPECT:
   - `opening_gap` `tied` 0.00;
   - `closing_gap` and `change_in_window` each `differs` with `amount` -1000.00 (the book shows 1,000.00 less in Union Bank than the statement);
   - Union Bank's closing to 31 August in this second `trial_balance` = -10800.00;
   - Tally's ledger display of Union Bank, 1-8-2026 to 31-8-2026: Closing Balance 10,800.00 Dr.
7. `trial_balance` from `2026-04-01` to `2026-08-15`: Union Bank to the middle of August. This is recorded for the notes, with no EXPECT; a refusal for the mid-month date is itself the note.
8. MARKS: M8, V8.

**Restore BK1** (C6), on the run coordinator's word. Then:
- MARKS first (EXPECT M and V equal to those read after BK1);
- `list_companies` (as step 0);
- `ledger_movement` as step 0 (equal to step 0's);
- `ledger_masters` (every page): no row named `BRIDGE Bank B`.

Otherwise STOP, and report what the book holds.

The book after each step:
- step 1: plus BRIDGE Bank B;
- step 2: plus W1;
- step 5: plus W2;
- after the restore: as at BK1.

## R3: a statement that starts and ends mid-month, before the build and after ComplyEaze Bridge's own post (LAUNCH B)

**What decides this row.** STATEMENT B's rows run from 3 to 28 September, so a whole-statement tie-out reads Union Bank at 3 September (twice) and at 29 September.
- The 14 October run read only on the 1st of a month. One partly recorded run on 10 October read at 7 August, at the end of a statement. A mid-month first date has never been read live.
- **The book around the statement is chosen so that a read on the wrong day cannot tie.**
  - **H1:** a Receipt on 2 September, before the first row, is in the statement's opening of 12,300.00. A read moved back to 1 or 2 September would miss it, and `opening_gap` would read -500.00.
  - **H2:** a Payment on 29 September, after the last row, is not on the statement. A read at the end of 29 September or later would count it, and `closing_gap` would read -200.00.
  - After the post, a voucher sits on each edge day.
    - If the read at 3 September counted that day's Receipt, `opening_gap` would read 2500.00.
    - If the read at 29 September stopped before 28 September's Payment, `closing_gap` would read 300.00.
  - **T1:** a twin of the 17 September Receipt keyed after the post must show as 650.00, so a tie is not the only answer the row can give.
- **The reference is Tally's own screen** (the Ledger display of Union Bank), with `trial_balance` recorded beside it.

**Tally's ledger display**, wherever a step asks for it:
- [OWNER] opens Gateway of Tally > Display More Reports > Account Books > Ledger > Union Bank;
- sets the period (Alt+F2) to the dates the step names;
- takes a screenshot showing the period, the Opening Balance and the Closing Balance with their Dr/Cr;
- goes back to the Gateway. This is a report screen, not a master or voucher screen.

Copy the two balances into the notes exactly as shown.

1. End LAUNCH A; start LAUNCH B (exe hash recorded).
   - `tools/list`: EXPECT `post_import` listed. Not listed = STOP (posting is not on).
   - `list_companies` as step 0.
   - MARKS: EXPECT those read after the R2 restore.
2. `masters` with `kind` `voucher_types`: record the Payment and Receipt rows whole. EXPECT for each: `series_numbering.observation` `observed`, `series_numbering.method` `automatic` and `series_numbering.sub_method` `auto_retain`. Otherwise STOP before anything changes (a build would refuse).
3. [OWNER] Key, each with a screenshot before its accept:
   - **H1:** Receipt dated 02-09-2026, Union Bank Dr 500.00, `Suspense` Cr;
   - **H2:** Payment dated 29-09-2026, `Suspense` Dr 200.00, Union Bank Cr.

   Then a screenshot of the Gateway with no dialog. MARKS: V3 = the V of step 1 + 2.
4. `parse_bank_statement` on STATEMENT B (whole, no `from` or `to`). EXPECT:
   - `statement_rows` 4, `rows_in_window` 4, `vouchers` 4, `skipped` 0, `cash_questions_open` 0;
   - `reconciled.running_balance_every_row` true, `reconciled.closing_balance` `13950.00`.

   Read its proposals file (`bank-statements/<proposals_id>.json` in the data folder, as JSON). EXPECT `window.whole_statement` true, `window.first_row_date` `2026-09-03`, `window.last_row_date` `2026-09-28`, and `window.tie_out.state` `established` with `first_date` `2026-09-03`, `last_date` `2026-09-28`, `opening_balance` `12300.00` and `closing_balance` `13950.00`.

   These figures were checked before the sitting by parsing STATEMENT B with the published 0.7.0 macOS package, which reads the file and never contacts Tally.
5. Tally's ledger display, 3-9-2026 to 28-9-2026: EXPECT Opening Balance 12,300.00 Dr and Closing Balance 12,300.00 Dr (nothing in the window yet).
   `trial_balance` from `2026-04-01` to `2026-09-02`: EXPECT Union Bank -12300.00.
6. `statement_tie_out`, `stage` `before_build`. EXPECT:
   - `window.first_date` `2026-09-03`, `window.last_date` `2026-09-28`, `rows_without_voucher` 0;
   - `tied` 0.00 three times.
     - The book at the start of 3 September is 12,300.00, against the statement's opening of 12,300.00.
     - The book at the start of 29 September is 12,300.00. Plus the file's net of 1,650.00, that is 13,950.00, against the statement's closing of 13,950.00. H2 is not counted.
7. `build_import_xml` with `company_guid`, `proposals_id` and `proposals_sha256` of that parse. EXPECT `voucher_count` 4 and a `batch_id`: record the answer whole and the `batch_id` as C. Any refusal: record it whole and STOP (nothing was written to Tally by ComplyEaze Bridge).
8. MARKS: V8, immediately before the post. EXPECT V8 = V3.
9. Tell [OWNER] a window will open, then call `post_import` with `company_guid` and `batch_id` C.
   - [OWNER] reads the window and answers **Yes** by mouse within its time.
   - The lab presses no key while the window is up.
   - If `approval.state` reads `pending`, wait `approval.retry_after_s` seconds and call `post_import` again with C. If it reads `approved` without a send, call again at once. That is the tool's own flow, not a retry.
   - Record every answer whole.
   - If an answer reads `import_approval_timed_out`: record it whole, read MARKS and `vouchers` (`ledger` Union Bank, 2026-09-01 to 2026-09-30) once each, and STOP. C is not called again without the run coordinator's word.
   - If the window is refused as not fitting the screen (`import_review_too_large`): record it whole and STOP. Nothing was sent.
10. EXPECT, in the answer of the call that sent the post: `dispatch.state` `posted_verified` and `dispatch.counters.created` 4. Then, before any other call, a screenshot of the whole Tally window: a dialog or a half-entered screen = C7, STOP.
11. `verify_import` of C. EXPECT `verification_status` `posted_verified`. Record the answer whole. MARKS: EXPECT V = V8 + 4.
12. `statement_tie_out`, `stage` `after_post`. EXPECT `tied` 0.00 three times.
13. `statement_tie_out`, `stage` `before_build`, again. EXPECT `opening_gap` `tied` 0.00, and `closing_gap` and `change_in_window` each `not_established` with `reason` `file_already_partly_posted` (the batch is in the import journal, so its vouchers are not counted twice).
14. Tally's ledger display, 3-9-2026 to 28-9-2026: EXPECT Opening Balance 12,300.00 Dr and Closing Balance 13,950.00 Dr.
    - `trial_balance` from `2026-04-01` to `2026-09-28`: EXPECT Union Bank -13950.00.
    - `vouchers` (`ledger` Union Bank, 2026-09-01 to 2026-09-30): EXPECT six vouchers: H1, the four of C dated and valued as STATEMENT B's rows, and H2.
15. [OWNER] Key **T1**: Receipt dated 17-09-2026, Union Bank Dr 650.00, `Suspense` Cr, a twin of the 17 September row. Screenshot before the accept. MARKS: V = the V of step 11 + 1.
16. `statement_tie_out`, `stage` `after_post`. EXPECT `opening_gap` `tied` 0.00, and `closing_gap` and `change_in_window` each `differs` with `amount` 650.00 (the book shows 650.00 more in the bank than the statement).
    Tally's ledger display, 3-9-2026 to 28-9-2026: EXPECT Closing Balance 14,600.00 Dr.

Any other final state, refusal or figure: record it whole, call nothing further for C, and STOP. A tie-out figure that differs from its EXPECT is the FINDING: report all three figures beside Tally's screen.

**Restore BK1** (C6), on the run coordinator's word. Then, as after R2: MARKS (equal to those after BK1), `list_companies`, and `ledger_movement` as step 0 (equal to step 0's). Otherwise STOP, and report what the book holds. End LAUNCH B (exe hash recorded).

The book after each step:
- step 3: plus H1 and H2;
- step 10: plus the four vouchers of C;
- step 15: plus T1;
- after the restore: as at BK1, the book of step 0.

## Close and evidence

- **C10:** Shut Company; no company loaded.
- **C11:** evidence to a new branch `lab/1211-tie-out-evidence-answers`, after the pre-push scan and [OWNER] yes. It holds:
  - every ComplyEaze Bridge answer, whole, with size and sha256;
  - the egress log lines this item's data folder gained;
  - the notes, one line per step with UTC times;
  - the backup's name, size and hash.

  Screenshots, the backup, STATEMENT A and the data folder stay on the lab PC, listed by name, size and sha256.
- `RESULTS.md` gives one line per row, PASS or FINDING, with the figures the EXPECT lines name.

## Not covered by this item

- A book of 1,000 or more vouchers.
- Educational mode.
- A voucher dated inside the window and posted during the tie-out's reads. This is a stated limit of the tool: it moves only the end figure.
- A statement whose last day is the end of a financial year (the read after it falls on 1 April, in the next year's books).
- Banks other than Union Bank's layout; Gold.
- A tie-out on TallyPrime 7.1: the earlier runs cover it.
