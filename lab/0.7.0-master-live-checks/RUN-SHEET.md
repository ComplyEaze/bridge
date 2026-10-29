# 0.7.0 master live checks: four read fixes merged on 29 Oct 2026, read live on a master build, on books the lab already has (Part of #2068, #692, #2125, #2119)

Not for merge. About 40 minutes. Read only: no key is pressed in Tally except to load and shut a company, nothing is created, altered, imported or restored, and no backup is needed. C5 and C6 do not apply: no book changes. `SHA256.txt` lists every file here with its bytes; check it before step 0.

## What it decides
Each part checks one change against the answer the code gives, with the code cited at master `218eaba1b5df10dfc8953f5ae3785e4a608f041e`:

| part | change | where |
|---|---|---|
| A, B | #2089 (`247bfed3`): `trial_balance` flags a ledger whose closing is not its opening plus its debit and credit totals | `src-tauri/src/agent_trial_balance.rs`, `src-tauri/src/agent_headline.rs` |
| A, B | #2128 (`21fe94b4`): `profit_and_loss` and `balance_sheet` refusing on such a row name its ledger and gap, give a next step, and carry the reads' evidence | `src-tauri/src/agent.rs`, `src-tauri/src/reports/statements.rs`, `src-tauri/src/tally/runtime_trial_balance.rs` |
| C | #2129 (`089f9471`): `egress_log` returns the newest receipts that fit the response cap and counts the rest | `src-tauri/src/agent_responses.rs`, `src-tauri/src/agent.rs` |
| D | #2136 (open when this sheet was written; conditional): the next step of an `outstandings` read refused past the request deadline | `src-tauri/src/agent.rs` `outstandings_unanswered_remediation` |

Each EXPECT below is read from that code; none is a figure from the book. A result that differs is a record, not a pass. It decides nothing about any other tool, book or date.

## Rules and companies
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/2068-nonint-trial-balance` at 7510f439, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** As the lab order's WHO ACTS section sets out, on the owner's standing word of 27 Oct 2026: the lab does every step here on the synthetic companies, and records each as that section directs (LAB-COMMON C11's rule on names applies to what is pushed).

**The companies**, each used as the row that last read it left it, and each THE COMPANY of its own part:

| part | company | as left by |
|---|---|---|
| A | `BRIDGE NONINT LAB` | `lab/2089-statements-closing-stock-answers` at 034f856a |
| B | `BRIDGE INTEGRATED CTRL LAB` | the same |
| D | `BRIDGE CORPUS DENSE` | `lab/1356-party-detail-by-ledger-answers` at a7f249d6 |

Each company's number and GUID are read from `list_companies` at its part's first step and compared with that row's.

Part C reads no company: `egress_log` reads only the launch's data folder.

If a company is absent, or its marks differ from that row's (A1, B1, D1), STOP and report: this sheet creates and keys nothing.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Tool calls only.** The reads are calls of ComplyEaze Bridge's built tool, one at a time. No request is sent by curl and no MARKS request file is used. C2's step 0 runs without its MARKS request (its `GET /status` checks still run), and the marks come from `ledger_masters` (rule 2).
2. **Marks.** M (ALTMSTID) and V (ALTVCHID) are read with `ledger_masters` (basic, first page, no `snapshot_id`): `snapshot.master_alter_id` and `snapshot.voucher_alter_id`. Record the whole answer and `snapshot.reused`, which must read false.
3. **One company at a time.** C1 holds for each part's company in turn: each is loaded alone for its part and shut by C10 at the part's end, before the next is loaded.
4. **Grant.** Nothing is written to Tally. Posting and ledger creation are not enabled in either launch.
5. **One Tally.** At step 0 and at the end, as C2 a and b set out (the process list and the listener on port 9001), plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001. Record all three. A screen-control tool brings no window forward.

Licensed mode only; Education mode = STOP.

## Build
**The master Bridge CI run at or after the merge of #2129** (it must contain #2089, #2128 and #2129), its `unsigned-bundle-smoke-windows-latest` artifact, downloaded by the lab on the owner's word of 29 Oct 2026. #2136 was still open when this sheet was written. If it has merged by the sitting, take the master run at or after its merge instead, and the #2136 check below becomes required.

Record the run id, its head commit (full SHA), the artifact id and its expiry, and the sha256 and bytes of the `.mcpb` and of `bridge_mcp.exe` inside it (`bin/x86_64-pc-windows-msvc/bridge_mcp.exe`). Then, in a checkout of the repository with master fetched, check that each pull request's merge commit is in that head, recording each command and its exit status:

| pull request | merge commit | `git merge-base --is-ancestor <commit> <head>` |
|---|---|---|
| #2089 | `247bfed37f206e4134798a928c04816e151dc9c5` | exit 0 required; otherwise STOP |
| #2128 | `21fe94b4c8d975e22a4b3e897e7fdb1bf659dd1f` | exit 0 required; otherwise STOP |
| #2129 | `089f9471b4d621421be79cbb7810e20376a07514` | exit 0 required; otherwise STOP |
| #2136 | its merge commit, read from the pull request if it has merged | exit 0 or 1 recorded; conditional (part D) |

Record the exe's sha256 before the first call and after the last: another sha256 = STOP. Record `initialize` and `tools/list` whole. From `tools/list`, EXPECT:
- `trial_balance`, `profit_and_loss`, `balance_sheet`, `egress_log` and `outstandings` all listed (an absent one = STOP);
- `egress_log`'s `limit` description containing ``omitted_for_size` counts those cut for the reply`, the text #2129 added (`src-tauri/src/agent_catalog.rs:675`). Its absence, with #2129's ancestor check at exit 0, = STOP: the build is not what the check says.

Two launches, each over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), `BRIDGE_AGENT_REDACTION` unset, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_MAX_ROWS` unset (so 200,000 bytes and 500 rows, `src-tauri/src/agent.rs:526-527`), and writes off:
- **Launch 1** (parts A, B, D): `BRIDGE_AGENT_DATA_DIR` a new empty folder, `C:\bridge-lab\070\data`.
- **Launch 2** (part C): `BRIDGE_AGENT_DATA_DIR` a copy, made in step C0, of the data folder the #1039 R2 row kept on the lab PC (`lab/1039-review-dialog-answers` at 19554624, "Kept on the lab PC"), at `C:\bridge-lab\070\egress-copy`. The kept folder itself is never used or changed.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded, without its MARKS request: the screenshot, the process list and port 9001 (rule 5), and `GET /status`. Hash the exe. Start launch 1. Record `initialize` and `tools/list`.

### Part A: BRIDGE NONINT LAB (#2089, #2128)
A0. Select Company: load THE COMPANY alone. Take a screenshot of the Gateway showing it loaded alone, with the Current Period.

A1. `list_companies`: EXPECT exactly THE COMPANY with `identity_state` `verified_tuple`; record its `guid` (the `company_guid` of every later call in this part). `tally_status`: release, licence, Education mode. `ledger_masters` (basic, first page): M0, V0. EXPECT them equal to that company's marks in `lab/2089-statements-closing-stock-answers` at 034f856a. A difference is recorded, and the run coordinator decides before A2.

A2. **Size first.** From A1, the number of ledgers. One `trial_balance` call with `limit` 100 is the plan. More than 100 ledgers = STOP.

A3. **`trial_balance`**, `from` 20250401, `to` 20260331, `limit` 100. One call. Record the answer whole with its size and sha256. EXPECT:
- the `Closing Stock` row carries `movements_differ_by` (`agent_trial_balance.rs:24`). Its value equals that row's closing minus its opening, debit and credit as returned, an empty amount taken as zero, computed by hand. Its `closing` is as Tally sent it.
- no other row carries `movements_differ_by`;
- `result.movements_differ_ledgers` 1 (`agent_trial_balance.rs:105`);
- `structuredContent.headline.lead` contains `1 ledger does not add up: the closing is not the opening plus the debit and credit totals (see movements_differ_by); each is shown as Tally sent it.` (`agent_headline.rs:294`);
- `result.limitations` first item, or second after a scope label, begins `1 ledger(s) carry movements_differ_by:` (`agent_trial_balance.rs:11`, `:224`).

  Record the row's four amounts as returned in the answer file only, not in the notes' text.

A4. **`profit_and_loss`**, `from` 20250401, `to` 20260331. One call. Record the answer whole. EXPECT a refusal (`isError` true), with `structuredContent.result.error` holding:
- `code` `profit_and_loss_read_failed`;
- `cause` `statement_trial_balance_row_inconsistent` (`reports/statements.rs:473`);
- `ledger` `Closing Stock`, and `gap` equal to A3's `movements_differ_by` for that ledger (`agent.rs:2782-2783`; the row is the first that does not add up, `reports/statements.rs:517`, `:931-937`);
- `remediation` the text of `agent.rs:1272-1281`, beginning `No statement figure was returned: ComplyEaze Bridge derives the statements only from Trial Balance rows that add up`.

  **Evidence** (#2128: the refusal keeps the evidence of the reads already made, `runtime_trial_balance.rs:323`): EXPECT `structuredContent.evidence` not the company read's. Its `response_sha256` differs from A1's `list_companies` evidence, and its `bytes` are larger. Record its `bytes`, `request_sha256` and `response_sha256` beside A3's `trial_balance` evidence.

A5. **`balance_sheet`**, `to` 20260331 (and `from` 20250401 if the tool asks for it). One call. EXPECT the same as A4, with `code` `balance_sheet_read_failed`.

A6. `ledger_masters` (basic): M1, V1. EXPECT M1 = M0 and V1 = V0. LAB-COMMON C10: shut THE COMPANY.

### Part B: BRIDGE INTEGRATED CTRL LAB (#2089, #2128)
B0 to B6. As A0 to A6, on BRIDGE INTEGRATED CTRL LAB, with the same dates and the same EXPECTs. B1's marks are compared with that company's in the same 2089 answers.

### Part C: `egress_log` on the large receipt folder (#2129)
C0. Stop launch 1. Copy the kept #1039 R2 data folder to `C:\bridge-lab\070\egress-copy` (a file copy; the kept folder is only read). List both folders with each file's size and sha256: EXPECT equal. Count the lines of `egress-copy\agent-egress.jsonl` (a local read) and record the count as N0. Start launch 2 and hash the exe. No company needs to be loaded; Tally stays as part B left it.

C1. **`egress_log`, `limit` 40.** First record the file's line count afresh (each answered call can add lines). One call. Record the answer whole, with its size and sha256. EXPECT, from the trim in `agent_responses.rs:137-143` and the result's `order` (`agent.rs:3154`):
- `isError` false;
- `structuredContent.truncated` true;
- `result.order` `newest_first`;
- `result.records` a non-empty list of whole lines, each parsing as JSON on its own;
- `result.omitted_for_size` present, with the number of records plus `omitted_for_size` equal to 40, if the file then held at least 40 lines;
- the first record the newest by its time;
- the whole answer at most 200,000 bytes.

  If it answers `agent_response_too_large` instead: record it whole. That is #2125's old behaviour, and the build lacks #2129.

C2. **`egress_log`, `limit` 200.** Record the line count afresh, then one call, and the same EXPECTs. Here records plus `omitted_for_size` equals the smaller of 200 and the file's line count just before the call, if the tool's scan reaches every line. Record both numbers either way.

C3. Stop launch 2. List `egress-copy` again (it may have grown by the calls' own lines); the kept folder is unchanged from C0.

### Part D: BRIDGE CORPUS DENSE, `outstandings` at 31 March (#2136, a refusal expected)
D0. Start launch 1 again, with the same data folder. Hash the exe. Load THE COMPANY alone, with a Gateway screenshot.

D1. `list_companies`, `tally_status`, `ledger_masters` (basic): M0, V0. EXPECT them equal to the #1356 row's (`lab/1356-party-detail-by-ledger-answers` at a7f249d6, P1 and D5).

D2. **`outstandings`**: `company_guid` from D1, `party` `DN Party 001`, `as_of` `20260331`, `detail` `bill_trail`. These are the #1356 row's D3 arguments (its `answers/005-d3-outstandings.json`). One call; do not repeat it. Record the answer whole, with its round-trip seconds. EXPECT a refusal:
- `code` `native_outstandings_read_failed`;
- `cause` `request_deadline_exceeded`;
- `remediation` present.
  - If #2136's ancestor check exited 0: the text of that change, beginning `Tally took longer to answer one request of its Bills Receivable or Payable report at this date than ComplyEaze Bridge waits (20 seconds)`.
  - If it exited 1, or #2136 has not merged: master's text at `agent.rs:2182-2187`, beginning the same way but without `(20 seconds)`.

  Record which. A different answer, such as one with figures, is recorded whole as a result. It is not repeated.

D3. `ledger_masters` (basic): M1, V1. EXPECT M1 = M0 and V1 = V0. LAB-COMMON C10: shut THE COMPANY. Stop launch 1.

E. Rule 5 check once more. `GET /status` after the last request. Hash the exe.

STOP RULES:
- C7 and C8 of LAB-COMMON;
- any dialog this sheet does not describe;
- a second `tally.exe`;
- a mark that moves;
- an absent company;
- a tool missing from `tools/list`.

A refusal is a STOP only where an EXPECT above names an answer and the refusal differs; then record it whole, do not retry, and go on to the next part.

## What the row records, and what it does not
It records, for each change, whether the live answer is what the cited code gives, on these books and dates. It does not read Tally's own screens, the tax-audit book, any other date, the desktop app, or the content of any receipt beyond its count and order.

**Confidence.** One observation per call, PARTIAL until repeated.

## Evidence
- Every tool answer whole with its size and sha256 (C11).
- The build's commit, run, artifact and exe sha256, before and after.
- The two folder listings of part C.
- Screenshots listed by name, size and sha256 (never pushed).
- A notes file with UTC times and one line per step, kept as C11 sets it out (no roles and no people). The notes name no amount from any book; amounts stay in the answer files.
- The companies' GUIDs are synthetic and listed in the merge gate's synthetic-UUID registry; anything else carrying a GUID shape is shown as `<guid>` before the evidence is pushed.
- The receipts in part C's answers are local files. Before pushing, run the pre-push scan over them. Any line the scan flags is withheld and counted, not pushed.
- Evidence on a new branch, `lab/0.7.0-master-live-checks-answers`, with `SHA256.txt`. Send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
