# 690 batch ledger rows (#2181, Part of #690): the ten-ledger approval box declined on a Windows screen (Part B of L0b), then three ledgers created in one approved request through the built tool (L1)

Not for merge. About 40 minutes. Written from the pull request body of #2181, whose checklist holds both rows open: "The live row (L1): three Sundry Creditors ledgers on a synthetic company, prepared and posted through this build, the box answered Yes, then the same call again (no box, nothing sent)" and "The ten-ledger box on Windows (L0b Part B, on this build)".

`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

## What it decides

| question | read from |
| --- | --- |
| Is a ten-ledger box wholly on the lab's screen at 125 percent, so that `MAX_LEDGERS_PER_BATCH` can stay at 10? | B2 |
| Do its title and buttons name the count (`ComplyEaze Bridge — create 10 ledgers?`)? | B2 |
| Does declining it send nothing? | B3, B4 |
| Does one approved request of three ledgers through the built tool read `posted_verified`, with the mark stepped by three and the three new rows under Sundry Creditors? | L3 to L5 |
| Does the same call again answer from the journal, with no box and nothing sent? | L6, L7 |

A box that does not wholly fit, or that is refused `master_review_too_large`, is a result, not a failure of the row: record it, skip nothing else in Part B, and report it. #2181's body says what follows from it (the limit stays below ten).

**Limit, whatever the result:** one screen size, one book, one sitting, 7.2 Silver. A batch at the limit written through the build is not measured here: Part B declines, and L1 writes three. L0b's Part A measured ten raw creates.

## Rules and company

**Who acts.** The lab makes every tool call and answers every box by mouse, on the owner's word of 28 Oct 2026.

**THE COMPANY** is `BRIDGE LEDGERBOX CI LAB` (synthetic, GUID `5848d0ca-ed85-4c97-ab20-e045893a9803`), the book of `lab/1650-ledger-create-box-row` and L0b, last restored to its BK0 on 30 Oct 2026 and then read at M 208, with three ledgers (Cash, Profit & Loss A/c, LBox Suspense) and 28 groups. Load it alone. Licensed mode only; Education mode = STOP. Another row may have used it since: record what step 0 finds. A ledger whose name begins `L0b Box` or `L1 Creditor` = STOP.

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **No raw requests.** Every read and write is a call of the built tool, one at a time. C2 runs without its MARKS request; its GET /status checks still run.
2. **Marks.** M (ALTMSTID) is read with `ledger_masters` (first page, no `snapshot_id`): `snapshot.master_alter_id`. Record the whole answer and `snapshot.reused`, which must read false.
3. **Backup.** One backup, BK0, of THE COMPANY at step L0, into the new empty folder `C:\bridge-lab\690rows\backups\bk0\`, before the first ledger write (C5). Record its name, size and sha256.
4. **Grant.** Three ledgers created by ComplyEaze Bridge (`post_ledger_creates`, step L3); nothing else written. Tool calls: about 16.
5. **Restore.** Step L8 restores BK0 by Tally's own Restore, as L0b's A4 did. It runs on the run coordinator's word for this row: ask for it after L7, and do not restore without it.
6. **One Tally.** Immediately before every `post_ledger_creates` call: the process list shows exactly one `tally.exe`, and it is the process that owns port 9001 (`Get-NetTCPConnection -LocalPort 9001 -State Listen` gives its process id). Record both. A second `tally.exe`, or 9001 owned by another process = STOP before the call.
7. **Click only while a box can be open.** From each `post_ledger_creates` call until its answer arrives, any screen-control tool on the PC only takes screenshots and clicks: it types no key and brings no window forward. The box opens on top and takes the keyboard focus, so a key typed anywhere can answer it.
8. **The 40-second answer.** A call returns `pending` with `retry_after_s` after about 40 seconds if the box is still open; the box stays open for its 120 seconds. Call `post_ledger_creates` again with the same `batch_id` after `retry_after_s`; that call joins the same box and opens no second one. A `pending` or `approved` answer is the tool's own flow and not a STOP. `master_approval_timed_out` = STOP.

## Build

`bridge_mcp.exe`, at `bin/x86_64-pc-windows-msvc/` inside the package's `bridge-tally.mcpb` (itself a zip), from the CI package of #2181's head `70dced8047a3140585efa1eb49813a114f0e504a`: Bridge CI run 41436265786, artifact 13140761067 `unsigned-bundle-smoke-windows-latest`, 58467734 bytes, zip digest `sha256:e01949d89b873001114c2ab9208e2f1c3eb968684e3dd476b0f1b9f9686c5894`, expires 2026-11-09 12:02 UTC. Record the zip's sha256 (different = STOP) and the exe's sha256; EXPECT the exe's equal to the package's smoke file `mcpb-smoke.json` `binary_sha256`, `af74d0ced3f5f4056f1d9848ccca5f6fd30b54454459e7696c2d6c71203f7973`, as it read when this sheet was written (different = STOP). Hash the exe before the first call and after the last: another sha256 = STOP. Record its `initialize` version and `tools/list` whole. If #2181's head has moved, this row still runs on this package: record the new head.

Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, one new empty `BRIDGE_AGENT_DATA_DIR` at `C:\bridge-lab\690rows\agent` (the masters journal must carry from each prepare to its posts), `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_AGENT_ENABLE_WRITES` and `BRIDGE_AGENT_ENABLE_MASTER_WRITES` on, as in the 1650 box row; batch post stays off.

## The code admits each trigger

All at `70dced80`.

| step | what it needs | where |
| --- | --- | --- |
| B1, L1 | `ledgers` admitted as one batch, up to ten of one parent, party items with no GSTIN | `src-tauri/src/agent_master_tools.rs:213`, `:154`; `src-tauri/src/tally/ledger_create_item.rs:265`, `:286` |
| B1, L1 | a prepare only reads Tally and appends to the journal | `src-tauri/src/agent_master_tools.rs:533` to `:603` |
| B1 | the preview's lines: five, two per ledger, a blank line, the warning | `src-tauri/src/agent_master_review.rs:128` to `:171` |
| B1 | the text check: 40 lines, 3,200 characters, 100 a line | `src-tauri/src/agent_master_review.rs:46`; `src-tauri/src/tally/approved_import.rs:2412`; `src-tauri/src/agent_import_post.rs:3594` |
| B2, L3 | the box is asked only for a batch with no intent recorded | `src-tauri/src/agent_master_tools.rs:691`, `:791` |
| B2, L3 | the title names the count, up to 10 | `src-tauri/src/tally/approved_import.rs:71` |
| B2, L3 | Yes, No and Cancel, No the default, measured by the fit guard | `src-tauri/src/tally/approved_import.rs:3110` to `:3148` |
| B2 | a box that does not fit is `master_review_too_large` | `src-tauri/src/tally/approved_import.rs:2600` |
| B3 | any answer but Yes is `master_approval_declined`; nothing recorded or sent | `src-tauri/src/tally/approved_import.rs:2605`; `src-tauri/src/agent_master_tools.rs:1625` |
| L3 | the send goes through the one approved post path, its intent recorded first | `src-tauri/src/agent_master_tools.rs:827` |
| rule 8 | the call budget, `pending`, `approved` and `retry_after_s` | `src-tauri/src/agent_import_approval.rs:46`, `:107`; `src-tauri/src/agent_import_post.rs:462` |
| rule 8 | the box's 120 seconds | `src-tauri/src/tally/approved_import.rs:575` |
| L4 | a post drops the company's listing snapshots, so the next read is fresh | `src-tauri/src/agent_master_tools.rs:842`; `src-tauri/src/agent_ledgers.rs:576`, `:579` |
| L6 | a batch with an intent goes to the journal; a settled one reads nothing and asks nothing | `src-tauri/src/agent_master_tools.rs:691`, `:1122`, `:1202` to `:1213` |
| L7 | the answer's own egress line | `src-tauri/src/agent_delivery.rs:315` |
| every write | Education refused before the box | `src-tauri/src/agent_master_tools.rs:557`, `:751` |
| Build | the switches and the terms setting | `src-tauri/src/agent.rs:592`, `:687`; `src-tauri/src/agent_terms.rs:40` |

## Files

`LAB-COMMON.md`, `RUN-SHEET.md`, `.gitattributes`, `SHA256.txt`. No request file: every call is a tool call whose arguments are written below. Check `SHA256.txt` before step 0 (any mismatch = STOP).

## Steps

0. LAB-COMMON step 0 with no company loaded, without its MARKS read. Tally's About screen: record the release (7.2 expected). Load THE COMPANY alone. Record the screen's resolution and scale and the work area (1920x1080 at 125 percent expected; another setting = record it, change nothing, and report before B2). `list_companies`: EXPECT exactly THE COMPANY, `verified_tuple`. `tally_status`. `ledger_masters`: M0 and the rows.

### Part B: the ten-ledger box, declined

B1. `prepare_ledger_creates` with `company_guid` and `ledgers`: ten items `{"name": "L0b Box 01", "kind": "party", "side": "sundry_creditors", "bill_wise": false}`, names `L0b Box 01` to `L0b Box 10`, no GSTIN. EXPECT a `batch_id` and a `preview` of 27 lines: five header lines, two lines for each of the ten ledgers, a blank line and the warning. Record the answer whole.
B2. One Tally (rule 6). `post_ledger_creates` with that `batch_id`. A box titled `ComplyEaze Bridge — create 10 ledgers?` opens. Without answering it, take one screenshot and record from it: whether the WHOLE box is on the screen (title bar, the answer line, all ten ledgers, the warning and the buttons); whether any part is cut by the screen's edge or the taskbar; the title text; the buttons' labels. A refusal `master_review_too_large` in place of a box: record the answer whole, skip B3, go to B4.
B3. Click **No**. EXPECT `master_approval_declined`, `attempt_recorded` false, dispatch `not_dispatched`.
B4. `ledger_masters`: EXPECT M = M0 and the same rows.

### L1: three ledgers, Yes, then the same call again

L0. BK0 (rule 3). `ledger_masters`: M0′ (a mark that moved across the backup is recorded, as C5 says, not a STOP). Every later comparison in L1 is against M0′.
L1. `prepare_ledger_creates` with `company_guid` and `ledgers`: three items `{"name": "L1 Creditor A", "kind": "party", "side": "sundry_creditors", "bill_wise": false}`, names `L1 Creditor A`, `L1 Creditor B`, `L1 Creditor C`, no GSTIN. EXPECT a `batch_id` and a preview naming THE COMPANY and the three ledgers. Record the answer whole.
L2. `ledger_masters`: EXPECT M = M0′ (a prepare writes nothing to Tally).
L3. One Tally (rule 6). `post_ledger_creates` with L1's `batch_id`. A box titled `ComplyEaze Bridge — create 3 ledgers?` opens. Screenshot it, confirm it is whole on the screen, click **Yes** within its 120 seconds. Follow rule 8 if the answer is `pending` or `approved`, one call at a time. EXPECT in the end `posted_verified`, `attempt_recorded` true, dispatch state `dispatched`, `resent` false, created 3 and every other counter 0. Record each answer whole with its elapsed time.
L4. `ledger_masters`: M1 and the rows.
L5. EXPECT M1 = M0′ + 3; three new rows `L1 Creditor A`, `L1 Creditor B`, `L1 Creditor C` under `Sundry Creditors`, each GUID beginning with THE COMPANY's GUID; every other row unchanged in GUID and ALTERID. Record the three ALTERIDs (M0′+1 to M0′+3 expected, in any order: the verdict compares them as a set).
L6. **The same call again.** One Tally (rule 6). `post_ledger_creates` once more with the SAME `batch_id`. EXPECT no box; `posted_verified`, dispatched, `resent` false, `journal.response_recorded` true, `journal.counters.created` 3, `journal.verdict` `posted_verified`; an answer in under a second. Take one screenshot during the call and after it. A box = click **No**, STOP.
L7. `ledger_masters`: EXPECT M = M1. `egress_log`: record it whole; EXPECT no entry for L6 beyond its own `response_prepared` line.
L8. Ask the run coordinator's word (rule 5). Shut THE COMPANY from the Company menu; Tally's Restore from BK0's folder, THE COMPANY only. Load it alone. `ledger_masters`: EXPECT M = M0′ and the same rows as L0's answer, byte for byte apart from timestamps.
L9. Hash the exe (`Build`). LAB-COMMON C10: shut THE COMPANY; it is kept as it is.

STOP RULES: C7 and C8; any dialog before a call; a box other than the one named; two such boxes at once; `master_approval_timed_out`; any answer outside the stated shape; a mark that moves outside the steps that change it; a second `tally.exe` (rule 6). After a STOP past L3, do not restore: report and wait.

## Result

PASS for Part B: B2 records a whole box or a typed refusal, and B3 and B4 hold.
PASS for L1: L3, L5, L6 and L7 hold, and L8 restores the book.
Anything else is FAIL, reported as found.

## Not measured by this row

A box at 1366x768 at 150 percent; a box the fit check cannot measure; a person's own mouse click (the lab answers); ten ledgers written through the build; the time a ten-create request takes inside one call; macOS; Gold; Education.

## Evidence

Push to `lab/690-batch-ledger-rows-answers`: `RESULTS.md` with every EXPECT marked, every tool answer whole with its size and sha256, the exe's sha256 both times, the display setting, BK0's name, size and sha256, screenshots by name, size and sha256 (never pushed), and a notes file with UTC times and one line per step. Evidence on the new branch, with `SHA256.txt`; send its SHA to the run coordinator.
