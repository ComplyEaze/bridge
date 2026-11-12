# #1761 slice 4, v4: a bank cash withdrawal paid to someone, posted and read back, on a book with one seed voucher

Not for merge. One row, on the synthetic company BRIDGE CASHPAY LAB that row 1761-S4 made. `LAB-COMMON.md` in this folder (the common file's own v3, not this sheet's; byte-equal to the v1 sheet's copy) applies in full. Where this sheet and it differ, the stricter holds. THE COMPANY below is BRIDGE CASHPAY LAB. Record the TallyPrime version, the licence mode and the date.

**WHY.** Slice 4 of #1761 (#2407) builds a bank cash withdrawal answered `paid_to_someone` as two vouchers on its date:
- a Contra, Dr the cash ledger, Cr the bank, carrying the statement row's id;
- a cash Payment of the same amount, Dr the person paid, Cr the cash ledger, whose id is the row's id followed by `-cash`.

A Payment whose payee's cash in a day comes to more than 10,000.00 is tagged in its narration for the CA (s.40A(3)). The pull request is tested only against a simulator. This row is its live evidence (P1). It checks:
- that Tally accepts the pair in one batch;
- that each voucher lands with its ledgers, amounts and narration intact, the tag included;
- that the build refuses a bank ledger named as the cash ledger, by reading the book's groups.

**WHAT CHANGED FROM v1** (`lab/1761-s4-cash-paid-to-someone-sheet` at 5e9580d8). In the book, the one variable is the seed voucher; the book changes below follow from it or from row 2507-T. The sitting also differs from v1's, as the list after them says. Row 1761-S4 (`lab/1761-s4-cash-paid-to-someone-answers`, its RESULTS.md) stopped at step 3.2. Its book held masters only, so the voucher mark read null, and the build refused the batch with `empty_book_first_import`, the refusal `pre_import_mark_refusal` (`src-tauri/src/agent_import.rs:179`) names for an absent voucher mark, and that the README's 0.5.x test notes record on a company that held ledgers only: with no voucher mark there is no "before" to attribute an import against. Nothing was sent, and the book is unchanged since that row's backup.
- v2, v3 and v4 key one seed voucher on Tally's screen before BK1, so the voucher mark has a value.
- **Why keyed, not built:** `build_import_xml` is the call that refused. On a masters-only book it refuses the first voucher whatever it holds, so a seed built by it cannot exist on this build.
- **Why this voucher:** a Journal dated 1-Apr-2025, Dr BRIDGE Suspense, Cr BRIDGE Capital, 1,000.00 (from v3; why it is not v2's Receipt is below). It touches neither Cash nor BRIDGE Bank A, the two ledgers this row reads, so BRIDGE Bank A still opens the statement days at 50,000.00 Dr, as the statement says, and every ledger figure in step 5 is v1's. It is a Journal, so the Contra and Payment series the batch numbers stay as v1 found them. Its date is the Gateway's current date, so keying it changes no date, and it is a day before every statement row.
- The figures that move are the marks only: V has a value from the seed on, and every V EXPECT below is stated against the marks read in this sitting.
- Reads that v1 asked of a window holding only empty days (step 4's timeout branch and step 6) are not asked to read complete: on 0.7.0, an empty day on a book with vouchers beside the window reads partial, never complete (row 2507-T). Step 6 reads the seed's own day, which holds a voucher, and reads the statement days only for a listed voucher.
- New folders under `C:\bridge-lab\1761-s4v4\`, so nothing of v1's, v2's or v3's is reused.

Beyond the book, this sitting differs from v1's in these, none of which the seed causes:
- **The build.** It is master de54a0f2, not v1's 06268abe, so the exe and its hash differ: this row is not v1's exe with a voucher added. Between the two commits the product source changed by one comment in `src-tauri/src/tally/approved_import.rs`; the rest of the diff is tests, fixtures and docs.
- **The approval window's wording** names #2107 and #2193 as merged, which they are on this build.
- **The numbering read** (step 0.5) now EXPECTs `auto_retain` as well, a re-check of a field row 1761-S4 read on this book at its step 0, and runs before the seed.
- **Step 4's batch-step wording** states what a `step` other than 4 reads, and a HOLD after it instead of a STOP (step 4.4).
- **BRIDGE Contractor A's bill-by-bill setting** is now a master row 1761-S4 already changed to No, not a step of this row.
- **Step 6** no longer reads the four vouchers' days complete; it reads them for a listed voucher only (step 6.3).

**WHAT CHANGED FROM v3** (`lab/1761-s4v3-cash-paid-to-someone-sheet` at c00dc92d). Item 92 ran v3 on 12 Nov 2026 and stopped at step 4.2 (the run coordinator's report of 12 Nov; its answers branch is not yet pushed). Steps 0.1 to 4.1 passed. The post answered `approval.state` `pending`, nothing dispatched. The computer-use access request for `bridge_mcp.exe` by that name matched no app; asked again by its full path, it answered `user_denied`, and [OWNER] reports that its dialog closed on its own, unseen. The follow-up `post_import` after `retry_after_s` was not made, and the approval window closed when the process exited. Nothing was posted (V equal to V0b). In the book v4 changes only its starting point; its steps are v3's, with these changes:
- **The starting book is row 1761-S4's masters-only BK1**, restored at step 0.4 (the fallback the WRITES list names), so the seed is keyed again as in v3. v3's BK1, which holds v3's seed, is not used.
- **The grant is a named step, 4.0**, inside the hold before step 4.1: asked once, by the exe's full path, after [OWNER] has been told what dialog will open and says they are watching for it. The dialog's text is recorded as [OWNER] reads it. The grant gates the post: not approved, or no dialog, is a STOP before any `post_import` (the run coordinator's ruling of 12 Nov).
- **The follow-up `post_import` is timed** (step 4.2): made once `retry_after_s` has passed since a `pending` answer, and within `dialog_remaining_s`. The launch is never ended while a window may be open.
- **A window that closes before an answer is named** (step 4.3). With the process running, the next call's answer decides. With the process gone, nothing is sent again, and on the run coordinator's word one reads-only launch, on a new data folder and an exe whose sha256 matches step 0's, reads V. A post after a relaunch is a STOP (the run coordinator's ruling of 12 Nov).

**WHAT CHANGED FROM v2, in v3** (kept in v4) (`lab/1761-s4v2-cash-paid-to-someone-sheet` at cae1213d). Item 91 ran v2 on 11 Nov 2026 (`lab/1761-s4v2-cash-paid-to-someone-answers` at e8ac97e6, its RESULTS.md) and stopped at step 0.7. Keyed with BRIDGE Bank B on its debit side, the seed Receipt opened a sub-screen, **Bank Allocations for: BRIDGE Bank B** (Transaction Type `Cheque/DD`, Amount 1,000.00, Inst No. empty, Inst Date 1-Apr-25, Bank Name empty), once that line's amount was accepted. v2 counts any extra screen during the seed as a STOP. The voucher was left unsaved, and the marks afterwards were step 0's (M 214, V null), so the book is as row 1761-S4 left it. Steps 0.1 to 0.6 passed. In the book, v3's one variable against v2 is the seed's type and its debit ledger; every v2 EXPECT that does not rest on them is kept as it was.
- **The choice: ledgers that open no such screen, not a named accept of it.** v3 keys a Journal, Dr BRIDGE Suspense, Cr BRIDGE Capital, with no bank or cash ledger in it. The reasons:
  - **The accept has never been seen.** What Bank Allocations saves when it is accepted is not established (item 91's RESULTS.md). It would at least record an instrument of type `Cheque/DD` with no number against BRIDGE Bank B. Naming it would add a second unobserved screen, and a saved field, to the one keyed write whose only way back is the fallback.
  - **Step 2 stays a repeat.** Step 2 names BRIDGE Bank B as the wrong cash ledger. With the seed off it, that ledger is as row 1761-S4 measured it, so step 2 reads the refusal on the same ledger with nothing new on it.
  - **A Receipt cannot avoid the screen here.** Its money side is the bank or cash it is received into. Cash and BRIDGE Bank A are this row's own ledgers and stay clear of the seed, and BRIDGE Bank B opens the screen. So the type changes too.
  - **Why these two ledgers.** BRIDGE Capital was already the seed's other side. Of the ledgers left, BRIDGE Suspense is the one no step reads by figure: steps 1 to 3 name it only as the parse's suspense ledger, no statement row goes to it (both are answered cash lines), and the build checks its group, not its balance. Neither BRIDGE Contractor A nor BRIDGE Wages is used, because each is a payee of the batch.
  - **What it costs.** A seed through a bank ledger, and the Bank Allocations screen, stay unmeasured. That is listed under Not covered.
- **The seed's wording** follows the screen as item 91 found it:
  - the double-entry layout (To and By lines), not v2's single-entry "Account / Particulars";
  - an F2 field that may show a date other than the header's (item 91: `2-4-2025` in the field with the header at 1-Apr-25).
- **An [OWNER]-at-PC gate before the seed, and a hold before step 4.1.** The lab's report of 11 Nov says the approval window needs a computer-use grant for `bridge_mcp.exe` in this row's build folder, and that only the owner can approve it, at the PC. That report is the source: v2's RESULTS.md does not hold it, since item 91 stopped before any approval window. Item 4 before the sitting is a gate: without the owner's commitment to be at the PC for step 4, step 0.7 does not start (in v4, the restore at step 0.4), so an absent owner costs reads only, not a keyed seed and a restore. The hold before step 4.1 stays, with a time limit, for an owner who is late.
- **The seed's accept and dates**, from the lab's report of 11 Nov on keyed Journals on 7.2. Ctrl+A accepted one with no message, so v3 accepts by Ctrl+A and names an "Accept ?" prompt as a STOP. F2's field is prefilled with the day after the current date, and F2 moves Tally's current date to the date keyed, so v3 names the prefill and reads the Gateway's current date after the seed (step 0.7).
- **Cost centres, read before the seed (step 0.5).** On a 7.2 book with cost centres on, a Journal debit opened "Cost Centre Allocations", prefilled by Tally, and the ledger screen there had no cost-centre field to turn it off (row 1231-CC, `lab/1231-cost-centre-keyed-allocation-answers`). BRIDGE Suspense is no shelter: it is not a revenue ledger, and that book's default Primary Cost Category read ALLOCATENONREVENUE Yes. Such a screen would stop the seed again. Whether this book has cost centres on has not been read. Row 1761-S4 created it with every feature field as filled except inventory and GST (its notes), and a new 7.2 company's feature screen offered Enable Cost Centres No (row 1231-CC's notes), so No is expected.
  - Step 0.5 now reads it with the build's own `company_features`. That is not a new request: 0.6.0 sent it on the lab's 7.2 in row 1231-CC, where it read `yes` for a book whose F11 screen read Yes, and its request bytes have not changed since (#1880 and #2425 each state so).
  - Its `no` has been compared with the screen on 7.1 books only (protocol reference §12a.18). On 7.2 a `no` is this sheet's best read before the seed, not a proof, so the extra-screen STOP at step 0.7 stays.
  - `masters` with `kind` `cost_centres` is not the gate: a book whose setting reads No can still list centres (§12a.17), so a list does not say whether the voucher screen will ask.
- The narration names the sheet's version, and the folders and the answers branch are the sheet's own (v4's in this sheet).

**WRITES.**
- The book is first restored to row 1761-S4's BK1 (step 0.4), on the run coordinator's RESTORE word, so the row starts from the masters-only book.
- The seed voucher, a Journal with no bank or cash ledger, is keyed BY HAND on Tally's screen. The masters are row 1761-S4's, keyed then, and are not changed.
- One batch of four vouchers is posted by the build's own `post_import`, after [OWNER]'s approval.
- Nothing is sent by curl, so the request-ID preflight finds no request to check.
- The book is restored to BK1 at the end: the masters and the seed.
- **The fallback** is row 1761-S4's own BK1, which restores the masters-only book from before the seed: `C:\bridge-lab\1761-s4\backups\BK1\TDBK1800_<company number>.001`, 24,241 B, sha256 beginning `bce8c06c` (in full in that row's RESULTS.md), written 11 Nov 2026 at 15:05 local. It stays on the lab PC. It is restored (C6) only on the run coordinator's RESTORE word.

**[OWNER] steps** are marked [OWNER]: the owner at the lab PC. Everything else is the operator's. The run coordinator's word, where the row asks for it (go-ahead, RESTORE, the evidence push), comes from the run coordinator.

## Build (holds the merge of #2407)

The build is the Windows bundle `unsigned-bundle-smoke-windows-latest` of a Bridge CI run on master whose commit holds the merge of #2407 and that carries that artifact, unexpired; the run coordinator names the run in the line below. The lab downloads it into a new empty folder and checks three hashes:
- the zip's sha256 against the artifact's digest;
- the `.mcpb` against `archive_sha256` in its `mcpb-smoke.json`;
- `bridge_mcp.exe` against `binary_sha256` in the same file.

It sends those three hashes to the run coordinator, who fills the line below by a new commit on this branch, after checking that the merge of #2407 is an ancestor of RC COMMIT.

RC COMMIT: de54a0f26e554f574727a0bc4914b0ad6ca88583  CI RUN: the Bridge CI `merge_group` run of #2502 at that commit, conclusion success (its id is written in words only, in `notes.txt` as fetched)  ARTIFACT: `unsigned-bundle-smoke-windows-latest` of that run, 60,522,001 bytes, expires 18 Nov 2026  ZIP DIGEST: sha256:5c30b5b7d62b32077d30d06aabceecc6a5b4f3bc3f3ac409f6f204c780b47106  EXE SHA256: be931eb0bd0228d5765886491970da712cb91941bc2e0a980049107c4d8267ce

**While any field of this line reads `<to fill>`: STOP. This row does not run.** Before the first call, hash the exe that will run and compare it with the line; any other hash = STOP. Record the version the tool reports.

Run it as the lab runs a staged exe over stdio, ONE launch from step 0 to the close, never relaunched in between, except the one reads-only relaunch step 4.3 names after a process exit. Environment:
- `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9001`;
- ONE new empty `BRIDGE_AGENT_DATA_DIR`, `C:\bridge-lab\1761-s4v4\data\`;
- `BRIDGE_TERMS_ACCEPTED=true`, the person responsible for the lab accepting the Terms of Use;
- [OWNER] posting ON: `BRIDGE_AGENT_ENABLE_WRITES=true`, set before the launch (step 0 records only that it is present);
- `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` unset, and `BRIDGE_AGENT_ENABLE_BATCH_POST` absent (it is retired).

No other ComplyEaze Bridge may make a call against port 9001 during the row. Record the process list at step 0.

**THE APPROVAL WINDOW** opens from the exe's own process. On this build it is opened topmost and system-modal (#2107, #2193); if it is not seen in front, look for it on the taskbar and record where it was. While it is open nobody presses a key on the PC and no tool is called, except step 4's own repeat `post_import`. [OWNER] answers it by mouse, within its time. Record the time it was seen open and the time it was answered. The window needs a computer-use grant for `bridge_mcp.exe` at its path in this row's build folder, and only [OWNER] can approve that grant, at the PC (the lab's report of 11 Nov): see item 4 before the sitting, and step 4.0.

**Marks.** MARKS means `ledger_masters`, first page, no `snapshot_id`:
- M = `snapshot.master_alter_id`;
- V = `snapshot.voucher_alter_id`;
- `snapshot.reused` must read false.

Record each answer whole.

**Calls.** Only the build's own tools, one at a time. An answer's `next_step` or `next_steps` is recorded, never acted on: this row's steps decide every call.

## Before the sitting
1. [OWNER] Tally is LICENSED on the lab PC. Educational mode anywhere on screen = STOP (C1).
2. Check every file against `SHA256.txt` (Get-FileHash). Copy `statement.csv` to `C:\bridge-lab\1761-s4v4\statement.csv` and check that copy's sha256 before every parse; a different file = STOP. The file is byte-equal to v1's.
3. `C:\bridge-lab\1761-s4v4\` is absent or empty before the row; anything in it = STOP.
4. [OWNER] Go-ahead for this row. **A GATE:** the run coordinator records [OWNER]'s commitment to be at the lab PC for step 4 and to approve there the computer-use grant for `bridge_mcp.exe` at its path in this row's build folder (the approval window needs it: the lab's report of 11 Nov). [OWNER] is also told that a computer-use access dialog will open on the lab PC at step 4.0, to be answered on that screen. Without that commitment, the restore at step 0.4 does not start: steps 0.1 to 0.3 are reads and may run, nothing is restored or keyed, and the sitting ends with the book as item 92 left it. The hold before step 4.1 still applies.
5. The run coordinator's RESTORE word for the restore at step 0.4, given with the go-ahead and recorded.

## The company (row 1761-S4's, as it left it)
BRIDGE CASHPAY LAB exists on the lab PC from row 1761-S4: books beginning 2025-04-01, no inventory, no GST, its masters keyed then, with BRIDGE Contractor A's "Maintain balances bill-by-bill" No. Masters are not keyed again. Exactly one company of this name, its data folder as row 1761-S4 recorded; anything else (none, two, or a second data folder of that name) = STOP, and reuse or overwrite nothing.

| ledger | under | opening |
| --- | --- | --- |
| Cash (Tally's own) | Cash-in-Hand | none |
| BRIDGE Bank A | Bank Accounts | 50,000.00 Dr |
| BRIDGE Bank B | Bank Accounts | none |
| BRIDGE Contractor A | Sundry Creditors, "Maintain balances bill-by-bill" No | none |
| BRIDGE Wages | Indirect Expenses | none |
| BRIDGE Suspense | Suspense A/c; its bill-by-bill and cost-centre fields unrecorded | none |
| BRIDGE Capital | Capital Account | 50,000.00 Cr |

BRIDGE Suspense's bill-by-bill and cost-centre fields are not recorded: row 1761-S4's notes give its group only, and its screenshot stays on the lab PC. A sub-screen at the seed is a STOP (step 0.7).

Do not change the Payment, Contra, Receipt or Journal voucher types.

## Step 0
1. LAB-COMMON C2 step 0 with THE COMPANY loaded alone, without its MARKS request (its GET /status checks stay). Record the process list and that `BRIDGE_AGENT_ENABLE_WRITES` is present. Hash the exe against the build line.
2. `tools/list`: record the answer whole. EXPECT:
   - a tool named `parse_bank_statement_csv` whose input schema lists `cash_ledger` under `cash_answers` items;
   - a tool named `post_import`.

   Either missing = STOP (not this build, or posting is off).
3. `list_companies`: EXPECT exactly THE COMPANY, with the GUID row 1761-S4's RESULTS.md records for it. Record it; every later call passes it as `company_guid`. Anything else = STOP.
4. **The starting book, restored.** MARKS once, recorded, not an EXPECT: item 92 left the book restored to v3's BK1, the masters and v3's seed. Then hash row 1761-S4's BK1 (the WRITES list): EXPECT its folder to hold that one file, 24,241 B, with the sha256 that row's RESULTS.md records; anything else = STOP, and nothing is restored. Restore it (C6) on the run coordinator's RESTORE word recorded before the sitting. The launch stays up, and nothing is called while THE COMPANY is shut. A restore that does not complete is a STOP, reported as C6 says. Then C6's screenshots, and MARKS first (C6): M0, V0. EXPECT M0 214 and V0 null, reused false: row 1761-S4's M0b and V0b, the book as that row left it. Anything else = STOP, and report the marks.
5. `masters` with `kind` `voucher_types`: record the Payment and Contra rows whole. EXPECT each to read `series_numbering.observation` `observed`, `series_numbering.method` `automatic` and `series_numbering.sub_method` `auto_retain`; otherwise STOP before anything else changes, with the book still as row 1761-S4 left it. `post_import` refuses, before its window, a series that is not Automatic (`import_voucher_numbering_not_automatic`) or not Retain (`import_voucher_numbering_not_retain`), so step 4 would test nothing. Row 1761-S4 read all three fields on this book at its step 0 (PASS). This read touches masters only, so it comes before the seed.

   Then `company_features`: record the answer whole. EXPECT `result.settings.cost_centres.value` `no`. `yes` = STOP before the seed: a Journal debit may open Cost Centre Allocations (row 1231-CC), and changing the setting is a variable this row does not have. `not_reported`, a refusal or any other answer = STOP. Either STOP leaves the book as row 1761-S4 left it. A `no` makes that screen unlikely, not impossible (WHAT CHANGED FROM v2), so step 0.7's extra-screen STOP stays.
6. **The fallback, checked.** Hash row 1761-S4's BK1 (the WRITES list): EXPECT its folder to hold that one file, 24,241 B, with the sha256 that row's RESULTS.md records. Anything else = STOP before the seed: a wrong seed would then have no way back.
7. **The seed, keyed** on Tally's screen with THE COMPANY loaded: a Journal voucher (Accounting Vouchers, F7).
   - **Date:** before F7, record the Gateway's current date. Record the date the voucher screen opened with. Press F2 (Change Voucher Date). Its field is prefilled by Tally: on a keyed Journal on 7.2, with the day after the current date (the lab's report of 11 Nov; item 91 saw `2-4-2025` with the header at 1-Apr-25). Record what it showed, type `1-4-2025` over it and press Enter. EXPECT the header to read 1-Apr-25. F2 also moves Tally's current date to the date keyed (the same report), which the Gateway read below checks.
   - **Lines:** in the layout the screen shows, and in the order it asks for them, without changing the mode. Debit BRIDGE Suspense 1,000.00 and credit BRIDGE Capital 1,000.00: in the double-entry layout item 91 saw, By BRIDGE Suspense (Dr) and To BRIDGE Capital (Cr); in a Dr/Cr layout, Dr and Cr. Record the layout and the order of the lines.
   - **Narration:** `seed for row 1761-S4 v4`.
   - Take a screenshot of the voucher screen before its accept, showing the date, both ledgers, the amount and the narration. Accept it once, by Ctrl+A: on 7.2 Ctrl+A accepted a keyed Journal with no message (the lab's report of 11 Nov). An "Accept ?" prompt, or any other prompt, = C7, STOP: press no key. Then leave the voucher screen as C7 says and take a screenshot of the Gateway with THE COMPANY loaded and no dialog. **EXPECT the Gateway's current date to read 1-Apr-2025**, the date keyed, as row 1761-S4's notes recorded it at the company's creation. Another date = STOP: record it, and change no date on screen.

   Neither line is a bank or cash ledger, so the Bank Allocations sub-screen a bank line opens (item 91) is not expected. That screen, **Cost Centre Allocations** (on 7.2 it opened on a Journal debit, prefilled by Tally, on a book with cost centres on: row 1231-CC; step 0.5's `no` makes it unlikely here), or any other sub-screen (an allocation of any kind, a bill), is an extra screen: STOP, press no key, and accept nothing in it.

   **A seed that is wrong in any way is a STOP:** a wrong date, ledger, amount or narration on the screenshot, a second accept, an "Accept ?" prompt, any extra screen, or a Gateway date other than 1-Apr-2025 after the seed. Nothing more is keyed and the voucher is not altered or deleted on screen. After a STOP before the accept, nothing is keyed until the run coordinator's word (C8). Item 91's route back, on such a word, was a screenshot of each screen with its title and then Ctrl+Q once per screen (C7), never Escape. It left the voucher unsaved with no prompt. A quit prompt is a dialog C7 does not name: press no key and report it. Then MARKS once (C8): V null means nothing was saved, and no restore is needed. The way back is the fallback, restored (C6) on the run coordinator's RESTORE word; the seed is keyed again in a later sitting, under a new go-ahead.
8. MARKS: M0s, V0s. **EXPECT V0s not null**, and `snapshot.reused` false. V0s null = STOP: the seed did not take, and step 3 would meet row 1761-S4's refusal again. Record M0s (whether keying a voucher moves M is not known; it is recorded, not an EXPECT).
9. `vouchers` for 2025-04-01 to 2025-04-01. EXPECT complete, and exactly one voucher, the seed: Journal, BRIDGE Suspense Dr 1,000.00, BRIDGE Capital Cr 1,000.00, narration `seed for row 1761-S4 v4` (23 characters). Record the answer whole, with its number and its completeness. The day holds the seed, so a complete answer is reachable: the window is complete when its rows were checked voucher for voucher (row 2507-T). An answer that reads partial does not prove "exactly one": record it with its reason, and STOP (nothing has changed since the seed). Another voucher, a second one, or another ledger, amount or narration = STOP. A STOP here goes back by the fallback, as item 7 says.
10. Backup BK1 (C5) into a NEW empty folder `C:\bridge-lab\1761-s4v4\backups\BK1\`: record the name, size and sha256. Then MARKS: M0b, V0b. **EXPECT M0b = M0s and V0b = V0s, so V0b is not null.** Nothing moves the marks between item 8 and the backup: a mark that moved = STOP, before BK1 becomes the restore point.

**Before step 1, V0b is not null.** Step 1 does not start otherwise.

The book: the masters and the seed.

## Step 1: the parse with the cash lines open (local; nothing goes to Tally)
`parse_bank_statement_csv` with:
- `statement_path` the copy above, `controls_source` `prior_statement`, `opening_balance` `50,000.00` and `closing_balance` `35,000.00`;
- `account_label` `Lab CSV 4321`, `bank_ledger` `BRIDGE Bank A` and `suspense_ledger` `BRIDGE Suspense`;
- no `mapping` and no `cash_answers`.

EXPECT:
- `statement_rows` 2, `vouchers` 0, `cash_questions_open` 2 and `cash_payments_over_limit` 0;
- two `cash_questions`, each with `movement` `withdrawal` and `printed_as` `ATM CASH WITHDRAWAL`:
  - `st-20250408-be621b1463b52f1b`, 12000.00 on 2025-04-08;
  - `st-20250409-ef7d14f34a7b0412`, 3000.00 on 2025-04-09.

Row 1761-S4 read exactly these at its step 1 on 0.7.0 (verified there). Other ids, counts or amounts = STOP; record the answer whole.

The book: unchanged.

## Step 2: a bank ledger named as the cash ledger, refused at build (reads only)
1. The same parse, with `cash_answers`:
   - `{"bridge_txn_id": "st-20250408-be621b1463b52f1b", "answer": "paid_to_someone", "ledger": "BRIDGE Contractor A", "cash_ledger": "BRIDGE Bank B"}`;
   - `{"bridge_txn_id": "st-20250409-ef7d14f34a7b0412", "answer": "paid_to_someone", "ledger": "BRIDGE Wages", "cash_ledger": "Cash"}`.

   EXPECT `vouchers` 4 and `cash_questions_open` 0 (the parse does not read the book). Record `proposals_id` and `sha256` as P2.
2. `build_import_xml` with `company_guid`, `proposals_id` and `proposals_sha256` of P2. EXPECT:
   - `result.state` `refused` and `result.reason` `cash_ledger_not_cash_in_hand`;
   - `result.refused_ledgers` holding exactly one row: `ledger` BRIDGE Bank B (plain or as its redaction marker; record which), `requires` `cash_in_hand`, `state` `cash_bank`, `reserved_group` naming Bank Accounts and `first_bridge_txn_id` `st-20250408-be621b1463b52f1b`;
   - `result.refused_ledgers_omitted` 0;
   - no `batch_id` and no file written.

   Record the answer whole. Any other reason (`cash_bank_ledger_not_established` included) or another row is a FINDING, not a pass: record it and go on to step 3. A file built = STOP: that is a FINDING, and the file is never sent.

   Row 1761-S4 measured this refusal on the masters-only book (PASS). The seed does not touch BRIDGE Bank B, so the ledger is as that row measured it. The refusal reads the book's groups, not its balances, and this step shows whether a book that now holds a voucher changes it.
3. MARKS: M and V equal to M0b and V0b.

The book: unchanged.

## Step 3: the batch, built (not sent)
1. The same parse, with `cash_answers`:
   - `{"bridge_txn_id": "st-20250408-be621b1463b52f1b", "answer": "paid_to_someone", "ledger": "BRIDGE Contractor A", "cash_ledger": "Cash"}`;
   - `{"bridge_txn_id": "st-20250409-ef7d14f34a7b0412", "answer": "paid_to_someone", "ledger": "BRIDGE Wages", "cash_ledger": "Cash"}`.

   EXPECT `vouchers` 4, `cash_questions_open` 0 and `cash_payments_over_limit` 1. Record `proposals_id` and `sha256` as P3.
2. `build_import_xml` with P3. EXPECT `voucher_count` 4 and a `batch_id`: record the answer whole and that `batch_id` as C. `empty_book_first_import` = STOP: the seed is not seen as a voucher mark, a FINDING. Any other refusal: record it whole and STOP.

The book: unchanged.

## Step 4: the approved post (C)
**[OWNER]-at-PC HOLD and the grant, before 4.1.** Step 4 starts only when the run coordinator records [OWNER]'s word that they are at the lab PC. The grant is asked inside the hold, at step 4.0.
- Until then, call nothing except step 4.0's computer-use `request_access`, and press nothing. C is built and not sent, and the book is as after step 3.
- **The hold lasts at most 60 minutes**, counted from step 3.2's answer. Past it, or sooner if [OWNER] cannot come, the run coordinator decides between waiting longer in this launch and step 6, the restore. Until that word, nothing is called and nothing is keyed.
- The approval's own expiry (`approval.expires_in_s`) counts from the click, which comes after the hold. No expiry of a built batch is known to this sheet, so a wait is not expected to change C.
- C is never sent in another launch.

**4.0 The computer-use grant, asked once.** The approval window is seen through a computer-use grant for `bridge_mcp.exe`, and only [OWNER] can approve it, at the PC (the lab's report of 11 Nov).
- Before asking, the operator tells [OWNER] three things: a computer-use access dialog is about to open on the lab PC; it asks for `bridge_mcp.exe` at the path below; and it is answered on that screen. It is asked only once [OWNER] says they are watching the screen.
- Ask once, naming the app by its FULL PATH: the `bridge_mcp.exe` in this row's build folder that step 0.1 hashed, never by the name alone (item 92: by the name it matched no app). Record the request as sent, its answer whole, and the times it was asked and answered.
- [OWNER] reads the dialog as it shows. The run coordinator records its text exactly as [OWNER] reports it, with what [OWNER] chose. If it closes before [OWNER] answers, or is not seen (item 92: `user_denied`, the dialog unseen), that is recorded too.
- **The grant gates the post** (fail closed, P7). Step 4.1 starts only when the answer shows the grant approved for that path. Not approved, a dialog that does not appear, closes unanswered or is not seen, or any other answer = STOP before any `post_import`. Record the dialog's text and the answer in each case. C is then built and not sent, the book is as after step 3, the grant is not asked again in this sitting, and step 6 follows on the run coordinator's word.

1. MARKS: V4a, immediately before the post. EXPECT V4a = V0b.
2. Tell [OWNER] a window will open. `post_import` with `company_guid` and `batch_id` C. [OWNER] reads the window and answers **Yes** by mouse within its time.
   - If `approval.state` reads `pending`: record the time the answer arrived, `approval.dialog_remaining_s` and `approval.retry_after_s`. Call `post_import` again with C once `retry_after_s` seconds have passed since that answer arrived: not earlier, and whether or not the window has been seen. If `dialog_remaining_s` is null, or not more than `retry_after_s`, call again at once. Repeat on every `pending`. On this build the window stays open for up to 120 seconds (`POST_DIALOG_LIMIT`, `src-tauri/src/tally/approved_import.rs`), a call waits about 40 seconds for its answer (`post_import`'s own description), and `retry_after_s` is 15, so one window takes two or three calls.
   - If it reads `approved` without a send: call again at once. This is the tool's own flow, not a retry.
   - Throughout step 4 the one launch stays up: its input is not closed and its process is not ended while a window may be open (item 92: the window closed when the process exited).

   Record every answer whole.
3. **A window that closes before [OWNER] answers it.** [OWNER] reports what was seen and when; nothing is pressed.
   - With the process still running, the next `post_import`, made when item 2 times it, is the read: `import_approval_timed_out` goes on as below. Any other refusal is recorded whole, then STOP. No call is added to check.
   - With the process gone (the window closes with it): STOP. Record the time, the exit code and the last lines of its error output, and take a screenshot of the whole screen. Nothing is called in that launch again, C is never sent in another, and no batch is built again. On the run coordinator's word, ONE new launch, once in the sitting, with a new empty data folder, `C:\bridge-lab\1761-s4v4\data-reads\`. Before it, hash the exe again: a sha256 other than step 0.1's = STOP, and no launch. It is made with `BRIDGE_AGENT_ENABLE_WRITES` absent, and the answers record that. With the GUID of step 0.3 it reads MARKS once and `vouchers` for 2025-04-08 to 2025-04-09 once; it stays up for step 6 and carries that step's reads (6.2 MARKS, 6.3 `list_companies` and `vouchers` for 2025-04-01, 6.4 `vouchers`), and calls nothing else: no `post_import` and no `build_import_xml` (that folder holds no batch, so it cannot post C). A `post_import` after a relaunch is a STOP. V equal to V4a is the proof that nothing was sent, as below. Then step 6, on the run coordinator's word.

   If an answer reads `import_approval_timed_out`: record it whole, read MARKS and `vouchers` for 2025-04-08 to 2025-04-09 once each, and STOP. That `vouchers` read covers two days that hold no voucher if nothing was posted, so its completeness is recorded, not required: on 0.7.0 it may read partial (row 2507-T). C is not called again without the run coordinator's word, and only if the answer read `attempt_recorded` false and V = V4a: those two are the proof that nothing was sent. The `vouchers` read listing no voucher only corroborates them; a partial, empty answer proves nothing alone.
4. EXPECT, in the answer of the call that sent the post:
   - `dispatch.state` `posted_verified` and `dispatch.counters.created` 4;
   - `post_location.state` `target_only`, with `post_location.target_voucher_step` reading `before` V4a, `after` V4a + 4, `step` 4 and `matches_created` true;
   - `post_span_binding.state` `bound`.

   That V moves by one per voucher created is recorded for a post of one voucher (0.7.0 row W1) and unverified for a batch. For a batch, a `step` other than 4 reads `matches_created` false, and the answer is then `reconciliation_required` (`batch_step_unconfirmed`), not `posted_verified`; its text says Tally reported creating the batch and every voucher reads back. That is a FINDING and a **HOLD**, not a STOP: record it whole, call nothing, and take a screenshot of the whole Tally window (a dialog or a half-entered screen = C7, STOP). Then report to the run coordinator, who may order step 5 (reads only) and then step 6; without that word, STOP. `post_import` of C is never called again and C is never rebuilt, whatever the word. Step 4.5 is not run after a HOLD.
5. `verify_import` of C: EXPECT `verification_status` `posted_verified` and `post_span_binding.state` `bound`, with no `rollback_recorded` and no `voucher_mark_contradiction` anywhere in the answer. Record the answer whole.

Any other final state or refusal: record it whole, do not call again, STOP. The one exception is step 4.4's HOLD.

The book: the masters, the seed, and the four vouchers of C.

## Step 5: read back
1. MARKS: record V. After step 4.5's `posted_verified`, EXPECT V4a + 4. After step 4.4's HOLD, V is recorded, not an EXPECT: a batch step other than 4 is what that HOLD reports.
2. `vouchers` for 2025-04-08 to 2025-04-09: record the answer whole, with its completeness, and a table with one line per voucher: date, type, number, each ledger entry (ledger, amount, side) and the narration printed with its length in characters. Both days hold vouchers of C. EXPECT exactly these four, and no other voucher (the seed is dated 2025-04-01, outside this window):

   | date | type | Dr | Cr | amount | narration (exactly) |
   | --- | --- | --- | --- | --- | --- |
   | 2025-04-08 | Contra | Cash | BRIDGE Bank A | 12,000.00 | `to Cash \| Lab CSV 4321 \| 08-Apr-2025` |
   | 2025-04-08 | Payment | BRIDGE Contractor A | Cash | 12,000.00 | `Cash paid to BRIDGE Contractor A from Cash \| withdrawn \| Lab CSV 4321 \| 08-Apr-2025 \| Bridge: cash to one payee in a day over 10000; check s.40A(3)` |
   | 2025-04-09 | Contra | Cash | BRIDGE Bank A | 3,000.00 | `to Cash \| Lab CSV 4321 \| 09-Apr-2025` |
   | 2025-04-09 | Payment | BRIDGE Wages | Cash | 3,000.00 | `Cash paid to BRIDGE Wages from Cash \| withdrawn \| Lab CSV 4321 \| 09-Apr-2025` |

   The narrations are the proposals the parse wrote at #2407's head `f8f9cf3f` on a Mac (the `\|` in the table is a plain `|`). Row 1761-S4 did not reach a post, so no 0.7.0 narration has been read back yet. A missing voucher, an extra one, or another ledger or amount = STOP: a FINDING. A narration that differs (cut short, a character changed) is a FINDING: record it exactly, then go on to step 6.
3. `ledger_movement` for Cash, 2025-04-01 to 2025-04-30: record it whole, with its completeness. EXPECT its closing to equal its opening (15,000.00 in, 15,000.00 out). The seed does not touch Cash, so this is v1's figure.
4. `ledger_movement` for BRIDGE Bank A, 2025-04-01 to 2025-04-30: record it whole, with its completeness. EXPECT its closing 15,000.00 nearer zero than its opening (opening 50,000.00 Dr, closing 35,000.00 Dr). The seed does not touch BRIDGE Bank A, so these are v1's figures.

   A figure that differs from these EXPECT lines is a FINDING, recorded, not a STOP. April holds days with no voucher beside the busy ones, so on 0.7.0 either answer may read partial (row 2507-T); a partial answer is recorded beside its figures, and alone is not a FINDING.

The book: as after step 4.

## Step 6: the restore
1. [OWNER] Restore BK1 (C6), on the run coordinator's explicit RESTORE. A restore that does not complete is a STOP, reported as C6 says; the fallback is then row 1761-S4's BK1 (the WRITES list), restored only on the run coordinator's word, which leaves the masters-only book, without the seed.
2. MARKS first (C6): EXPECT M and V equal to M0b and V0b.
3. Then `list_companies` (as step 0), and `vouchers` for 2025-04-01 to 2025-04-01: EXPECT complete, and exactly the seed, as at step 0.9. A partial answer is recorded with its reason and is a FINDING: it does not prove "exactly the seed". Another voucher, or none = STOP, and report what the book holds.
4. `vouchers` for 2025-04-08 to 2025-04-09, once, to corroborate the restore: STOP only if a voucher is listed. These days hold no voucher after the restore, so the answer may read partial (row 2507-T); a partial, empty answer is recorded and proves nothing alone. V equal to V0b at item 2 is the proof that no voucher was added since BK1.

The book: as at BK1, the masters and the seed.

## Not covered by this row
- A deposit, and any cash answer other than `paid_to_someone`.
- The over-limit sum across several Payments to one payee in a day: here each payee has one Payment. The tag on exactly 10,000.00, and cash paid outside this file. The unit tests of #2407 carry these.
- A ledger outside both Cash-in-Hand and Bank Accounts named as the cash ledger (an earlier check refuses it with another reason). A bill-wise payee.
- A cash ledger other than Tally's own Cash, and a PDF statement. This row parses the CSV edge only; the voucher shapes are the same.
- The first import into a masters-only book: row 1761-S4 measured its refusal, and this row does not ask it again.
- A seed keyed through a bank ledger, and the Bank Allocations screen it opens (item 91): the seed, as v3's, names no bank or cash ledger, so that screen's accept and what it saves stay unmeasured.
- A book with cost centres on: step 0.5 STOPs on it, so a seed through Cost Centre Allocations, and what that screen saves, stay unmeasured. So do BRIDGE Suspense's bill-by-bill and cost-centre fields.
- Any TallyPrime release other than the one recorded.

## The book after each step
- Step 0, to item 4's restore: as item 92 left it, v3's BK1 restored (the masters and v3's seed).
- Step 0, from item 4's restore to item 7: the masters only, as row 1761-S4 left it.
- Step 0 from item 7, and steps 1 to 3: the masters and the seed.
- After a STOP in step 0 from item 7 on, and the fallback restored: the masters only.
- Steps 4 and 5: plus the four vouchers of C.
- Step 6: as at BK1, the masters and the seed.

STOP RULES: C7 and C8; any dialog before a call, except step 4.0's grant dialog, whose text is recorded; an approval window other than step 4's; a mark that moves outside the step that changes it (before step 4, only step 0's item 4, the restore, which EXPECTs V null, and its item 7 may move V); any answer outside the stated shape; a `post_import` without step 4.0's grant approved, or in any launch but the first. Step 4.4's HOLD wins over that last rule: an answer step 4.4 names as a HOLD is a HOLD, not a STOP.

## Close and evidence
- **C10:** Shut Company; leave no company loaded. THE COMPANY stays on the lab PC, as at BK1: it is synthetic.
- **C11:** evidence to a new branch `lab/1761-s4v4-cash-paid-to-someone-answers`, with `SHA256.txt`, after the pre-push scan and the run coordinator's yes. It holds:
  - every answer whole with its size and sha256;
  - the seed's screenshots by name, size and sha256 (never pushed), the date the voucher screen opened with, what the F2 field showed, the layout and line order, and its step 0.9 read;
  - the run coordinator's record of [OWNER]'s commitment at item 4 before the sitting, and of [OWNER]'s word at the hold before step 4.1, with the hold's start and end times;
  - the Gateway's current date before F7 and after the seed;
  - step 0.4's MARKS before the restore, its restore-plan.txt and the data folder listings;
  - the grant at step 4.0: the request as sent, its answer, the times asked and answered, and the dialog's text as [OWNER] reported it;
  - the time of every `post_import` answer in step 4, with its `dialog_remaining_s` and `retry_after_s`, and the time of each call made after it;
  - the fallback's check (step 0.6);
  - the step 5 voucher table;
  - the data folder's file listing after step 4 (names, sizes, sha256);
  - BK1's name, size and sha256, and the build's hashes;
  - the window's open and answer times;
  - a notes file with UTC times and one line per step.

  A RESULTS.md gives each check as PASS or FAIL, with the build, company, date and record: the seed (step 0), the build refusal (step 2), the post (step 4), the read-back (step 5) and the restore (step 6).
