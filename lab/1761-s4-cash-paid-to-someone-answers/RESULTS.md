# Results: row 1761-S4, a bank cash withdrawal paid to someone, 2026-11-11

**Outcome: stop at step 3.2, a FINDING. Steps 4 to 6 were not run.** The book was new and held masters only, so its voucher mark was absent (V0 and V0b read null). Build 0.7.0 refused the batch at build with `empty_book_first_import`: typed, in the answer, `isError` true, no `batch_id`, no file, and nothing sent to Tally. The marks after the refusal equal BK1's. The run lead chose to close the row at this stop with no restore, 2026-11-11. A v2 sheet will give the book one seed voucher before BK1, as a new queue item.

**Step 2 is a result, not part of the stop.** The build refused a bank ledger named as the cash ledger, as the sheet expects: PASS.

**Who acted.** All by the lab, on the owner's standing word for synthetic books (2026-10-27 and 2026-10-28):
- the company and its masters keyed on Tally's screen;
- BK1;
- the posting setting in the launch file.

No approval window opened, and no key was pressed while one could have been open. The go-ahead and the close came from the run lead.

## Record

- **Build:** the Windows bundle `unsigned-bundle-smoke-windows-latest` of the Bridge CI `merge_group` run of #2500 at `06268abe63d659a605ca2592ee4d0a2d65eba0ea`. The merge of #2407 is its ancestor. Three hashes were equal to the build line (evidence/build-hashes.txt):
  - zip `ea1068f80898b1c0369cceb83cf991e4911cdac9fb8ee56de53f9b5842a8c0de`;
  - `.mcpb` `9da9c5bdeeda5ba1366de3aea5f86b70f451bf66397a8effc05e3f7db9ad00cf`;
  - `bridge_mcp.exe` `507da806d9d49a72fdb144be3263e6b1c303bfc964b103616b0ea92ee7b02bf6`.

  The exe was hashed again before the launch, and the server reported `bridge-tally` 0.7.0.
- **Sheet:** `lab/1761-s4-cash-paid-to-someone-sheet` at `5e9580d8d4a0d2408360c0a506133226bc6d7228`, 4 of 4 files equal to its SHA256.txt.
- **Tally:** TallyPrime single-user edition, `tally.exe` file version 1.1.7.2. Licensed, no Educational mark (C1).
- **Company:** BRIDGE CASHPAY LAB, new and synthetic, GUID `b52de336-8552-4d40-9221-60aa64d3c1f1` (`verified_tuple`).
  - Books from 2025-04-01; no inventory, no GST; State Not Applicable.
  - Gateway period 1-Apr-25 to 31-Mar-26, current date 1-Apr-2025.
  - Masters as the sheet's table, keyed by hand, each screenshot taken before its accept. BRIDGE Contractor A's "Maintain balances bill-by-bill" was changed from Yes to No on screen.
- **Launch:** one stdio launch from step 0 to the close (server pid 3020, exit code 0). The launch file's keys:
  - `BRIDGE_AGENT_DATA_DIR` (new and empty);
  - `BRIDGE_AGENT_ENABLE_WRITES`;
  - `BRIDGE_TALLY_HOST`;
  - `BRIDGE_TALLY_PORT`;
  - `BRIDGE_TERMS_ACCEPTED`.

  `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` were absent. The row launched no other ComplyEaze Bridge. The desktop app's own processes were left running, as standing, and the row made no call through them (evidence/notes.txt, step 0).

## Checks

| step | check | result |
| --- | --- | --- |
| 0 | C2; `tools/list` (37 tools, `cash_ledger` under `cash_answers` items, `post_import` present); `list_companies` exactly THE COMPANY; MARKS M0 214, V0 null, reused false; Contra and Payment `observed`, `automatic`, `auto_retain` | PASS (V0 null: the book has no voucher) |
| 0.6 | BK1 by C5: `TDBK1800_<company number>.001`, 24241 B, sha256 `bce8c06cc0dcf344bc4408e34e0c8374b9920518387552e916e44e0fc4c23986`, written 2026-11-11 15:05:10 local; MARKS M0b 214, V0b null | PASS |
| 1 | parse, cash lines open: `statement_rows` 2, `vouchers` 0, `cash_questions_open` 2, `cash_payments_over_limit` 0; questions `st-20250408-be621b1463b52f1b` 12000.00 and `st-20250409-ef7d14f34a7b0412` 3000.00 | PASS |
| 2 | **the build refusal:** parse `vouchers` 4, `cash_questions_open` 0 (P2 `statement-f54ce5c7-f4ce-477f-b774-18d7f39b4e63`); build `refused`, `cash_ledger_not_cash_in_hand`, one `refused_ledgers` row (BRIDGE Bank B, plain; `cash_in_hand`; `cash_bank`; Bank Accounts; `st-20250408-be621b1463b52f1b`), `refused_ledgers_omitted` 0, no `batch_id`, no file; MARKS M 214, V null, equal to M0b and V0b | **PASS** |
| 3.1 | parse: `vouchers` 4, `cash_questions_open` 0, `cash_payments_over_limit` 1 (P3 `statement-19e39a4b-3fa2-4b26-afd2-e0cea5b97265`, sha256 `11ccab218679b83ded1a9439790f910e001c39514f81e3b9c13f89574eeaead4`) | PASS |
| 3.2 | build with P3: EXPECT `voucher_count` 4 and a `batch_id` | **FAIL: refused `empty_book_first_import`; stop (FINDING below)** |
| 4 | the post | NOT RUN |
| 5 | the read-back | NOT RUN |
| 6 | the restore | NOT RUN: not needed, as nothing was sent and the marks equal BK1's (the run lead, 2026-11-11) |

## FINDING: a masters-only book cannot take the row's first import

The step 3.2 answer (answers/010-s3-build.json):
- `isError` true;
- `result.error.code` `empty_book_first_import`;
- `result.error.message` "ComplyEaze Bridge refused this operation.";
- `evidence.state` `partial`, `evidence.reason_code` `empty_book_first_import`.

The remediation says that the company has never held a voucher, so Tally reports no voucher high-water mark. Bridge then has no "before" to attribute an import against, cannot build or post the first voucher into this company yet, and sent nothing to Tally.

Every read before it agrees: `snapshot.voucher_alter_id` read null at steps 0, 0.6 and 2. That is the book's state the sheet asked for ("The book: the masters only").

This is the documented refusal, not a new one:
- `pre_import_mark_refusal` in `src-tauri/src/agent_import.rs` (line 179 at master `de54a0f26e554f574727a0bc4914b0ad6ca88583`) names an absent voucher mark `empty_book_first_import`.
- The README's 0.5.x test notes record the same refusal on a company that held ledgers only.

(Corrected 2026-11-11: an earlier version of this file also cited ADR 0004 item 12 here. That item is the duplicate-number read's control and does not record this refusal: its S8 list names "a company with no voucher" only as a control case still to be measured. ADR 0004 does not record the refusal.)

So the sheet's book cannot reach step 4 on this build. The row's stop came from the sheet's design, not from the slice under test. Step 2's refusal runs before the mark read and was measured. Steps 3.2 to 6 stay unmeasured: the Contra and Payment pair in one batch, the s.40A(3) tag in a posted narration, and the read-back.

## The data folder at the close (evidence/data-listing-at-close.txt)

| file | bytes | sha256 |
| --- | --- | --- |
| data/agent-egress.jsonl | 29174 | `a8e2a6bae27bc8cd4cef4709466849a7e90bacebec3d529eebfec0fa0ad4012d` |
| data/agent-import-admission.lock | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` (empty) |
| data/terms-acceptance.jsonl | 90 | `5f1f46a152be4d21fcfaf5e4c63fd171a1070edda46336555fe6754331fd2754` |
| data/bank-statements/statement-09ab698d-0969-4cee-befb-595a815799d8.json | 1853 | `d4e867cdb510252d17cbf10d1b513cf70a26e23842c6d2b354fc25c5598f4c6e` (the step 1 proposals; the parse's `sha256`) |
| data/bank-statements/statement-f54ce5c7-f4ce-477f-b774-18d7f39b4e63.json | 4184 | `78adde703af5e44a9a008d90129c15d4fc480a0ef705ecabaa45ad9456720392` (P2) |
| data/bank-statements/statement-19e39a4b-3fa2-4b26-afd2-e0cea5b97265.json | 4139 | `11ccab218679b83ded1a9439790f910e001c39514f81e3b9c13f89574eeaead4` (P3) |

The step 2 build left the folder as it was, except that `agent-egress.jsonl` grew (evidence/s2-data-*-build.txt). The step 3.2 build added the empty `agent-import-admission.lock` and no batch file (evidence/s3-data-after-build.txt).

## Close

- The client was closed by its inbox file at 2026-11-11T09:39:12.884Z: session ended, server pid 3020, exit code 0.
- C10: Company > Shut > BRIDGE CASHPAY LAB, with no prompt. Select Company was then shown with no company loaded (shots/c10-after-shut.png, on the lab PC).
- BRIDGE CASHPAY LAB stays on the lab PC as it is, with masters only, for the v2 sheet. BK1 stays on the lab PC by name, size and sha256.

## Deviations

1. The launch files were written by a Python helper (mk-launch.py). An earlier pair, written by a shell heredoc, lost its backslashes and did not parse. It was moved aside unused (aside/, by sha256 in kept-local.txt).
2. The step 0.6 line in notes.txt holds two backspace bytes and a tilde in place of the BK1 path. The shell that appended the line read two backslashes as escapes. A correction line follows it. The pushed copy shows the path, and the change is listed in substitutions.txt.
3. After the step 2 MARKS call, a shell typo stopped call.py printing its summary line. The answer (answers/008-s2-marks.json) and the client's log line are whole.
4. show.py, a new read-only helper, printed each answer's fields for the EXPECT checks.
5. The book's GUID is new, so it is not in files tracked on master. The stage checks it against the step 0 `list_companies` answer and the excluded book instead.

## Not established

- Steps 4 to 6 (the post, the read-back, the restore) and every check they carry.
- Whether a book with one seed voucher reaches step 4 on this build. That is the v2 sheet's question.

## Files

- **answers-mcp/:** each tool answer whole, with the calls (inbox/), the client's log and pid.
  - `company_number` values are shown as `<redacted: company number>`.
  - In three parse answers, the statement path the lab client recorded is shown as `<LAB_DIR>/statement.csv` (substitutions.txt).
  - The `tools/list` and voucher-type answers stay on the lab PC (kept-local.txt), because each holds an edition word. Their checks are in the table above and in notes.txt.
- **statement.csv:** the lab copy stays on the lab PC because it holds a digit run of 10 or more. It is byte-equal to the sheet's `statement.csv` (sha256 `ebca906726de102cb23dcf15767de2cee5bba0ff4c401debd6e7b9fb3bf776ff`).
- **data/:** the server's data folder, whole.
- **evidence/:** notes.txt (UTC, one line per step), the tools log, the listings, build-hashes.txt, and the port listing cut to its port-9001 LISTENING lines.
- **scripts/:** the row's helpers.
- **args/:** the call arguments.
- **kept-local.txt:** files kept on the lab PC by size and sha256.
- **SCREENSHOTS.txt:** the captures by name and hash.
- **substitutions.txt:** each pushed copy that differs from its original.
- **SHA256.txt:** every pushed file.
