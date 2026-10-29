# Results: 0.7.0 master live checks, 29 Oct 2026, on master 218eaba1 (Bridge CI 40818364258). #2089 and #2128 hold on both closing-stock books, except that `result.movements_differ_ledgers` is absent, as the cited code gives. #2129 holds: `egress_log` answers newest first within the cap, where #2125's old build refused. Part D got master's text, because this build predates #2136. **D3's marks read was refused** (`request_deadline_exceeded`, after D2) and not repeated, so the marks after D2 are not established. Reads only

The sheet is `lab/0.7.0-master-live-checks` at a73091ea1c61b46d0b6f822daa87dfd9960340f5. `SHA256.txt` matched 3 of 3, and LAB-COMMON v3 matched its stated hash.

**Who acted.** The lab did every step, on the owner's standing word of 27 Oct, on synthetic companies. The lab downloaded the build on the owner's word of 29 Oct.

## Build (deviation)
- **Why not a run after #2136.** #2136 merged at 04:58:50Z, as 887454e5. Both passing master runs since then skipped "Bundle smoke" (decided by "Determine bundle scope"), so neither has a Windows bundle:
  - 40820733936 at 887454e5;
  - 40822007643 at 994967a6.

  So the build is the newest master run that has the bundle. It, and part D on the "exited 1" branch, were agreed before part D.
- **Run:** Bridge CI 40818364258, head 218eaba1b5df10dfc8953f5ae3785e4a608f041e (push, success). This is the master the sheet cites for its code lines.
- **Artifact:** 12874111950 `unsigned-bundle-smoke-windows-latest`, expires 2026-11-05T04:52:26Z.
- **Files:**
  - `bridge-tally.mcpb` 15,517,350 B, sha256 92025bda0e6430f022424257932c376ee36ed63c6d724748d8315ce73a72737e;
  - `bridge_mcp.exe` 32,419,840 B, sha256 41b86dac5c964b5e0106f5b392f854f4f13bf7ca181940902c796dcd24eb67bd.

  Both equal the artifact's `mcpb-smoke.json`. The exe hash is the same at step 0, at each launch, at D0 and at E.
- **Ancestor checks** (`git merge-base --is-ancestor <commit> 218eaba1…`):

  | PR | merge commit | exit |
  |---|---|---|
  | #2089 | 247bfed3 | 0 |
  | #2128 | 21fe94b4 | 0 |
  | #2129 | 089f9471 | 0 |
  | #2136 | 887454e5 | **1** |

- **`initialize` and `tools/list`:** bridge-tally 0.6.1, 30 tools. All five tools are listed. `egress_log`'s `limit` description holds #2129's `omitted_for_size` text.
- **Launches:**
  - launch 1 (parts A, B and D): a new, empty `data` folder;
  - launch 2 (part C): a copy of the kept #1039 R2 data folder.

  No write, redaction, max-bytes or max-rows key in either. Every launch exited 0 with an empty error stream.

## Results
| step | call | read | against the EXPECT |
|---|---|---|---|
| 0 | | no company loaded; one tally.exe (13992) owns 9001 (tasklist, netstat, Get-NetTCPConnection); `/status` 200 | held |
| A1 | 001 to 003 | BRIDGE NONINT LAB, 100076, `verified_tuple`; 7.2 Silver, not Education; M 215, V 3, `reused` false; 8 ledgers | equal to the 2089 answers |
| A3 | 004 `trial_balance` | Closing Stock the one row with `movements_differ_by`, equal to the hand computation; headline lead as cited; `limitations[0]` begins `1 ledger(s) carry movements_differ_by:` | held, except **`result.movements_differ_ledgers` absent** (see below) |
| A4 | 005 `profit_and_loss` | refused: `profit_and_loss_read_failed`, `statement_trial_balance_row_inconsistent`, ledger `Closing Stock`, gap equal to A3's, remediation as cited | held |
| A4 evidence | | 163,042 B (response b51136a0…); A1's `list_companies` 8,008 B (aca37f49…); A3's `trial_balance` 47,542 B (0f49fd52…) | held: not the company read's, and larger |
| A5 | 006 `balance_sheet` | the same as A4, with `balance_sheet_read_failed` | held |
| A6 | 007 | M 215, V 3 | equal |
| B1 | 008 to 010 | BRIDGE INTEGRATED CTRL LAB, 100095, `verified_tuple`; M 215, V 3; 8 ledgers | equal to the 2089 answers |
| B3 to B5 | 011 to 013 | the same as A3 to A5. Evidence 163,190 B (1a30cac6…); B1's `list_companies` 8,080 B; B3's `trial_balance` 47,686 B (b7d91c43…) | held, except `movements_differ_ledgers` absent |
| B6 | 014 | M 215, V 3 | equal |
| C0 | | the kept folder copied: 22 files, listings equal; N0 = 60 lines | held |
| C1 | 015 `egress_log` limit 40 | not an error; `truncated` true; `newest_first`; 40 whole records, each parsing on its own; the first the newest; response 198,814 B | held, except **`omitted_for_size` absent** (see below); 40 + 0 = 40 |
| C2 | 016 `egress_log` limit 200 | the file at 62 lines. Not an error; `truncated` true; `newest_first`; 41 records, `omitted_for_size` 21 (41 + 21 = 62, the smaller of 200 and 62); the first the newest (C1's own receipt); response 188,508 B | held |
| C3 | | the copy: only `agent-egress.jsonl` grew, now 64 lines; the kept folder unchanged from C0 | held |
| D1 | 017 to 019 | BRIDGE CORPUS DENSE, 100012, `verified_tuple`; M 328, V 29900 | equal to the 1356 row's |
| D2 | 020 `outstandings` (DN Party 001, `as_of` 20260331, `bill_trail`), 21.997 s | refused: `native_outstandings_read_failed`, `request_deadline_exceeded`. The remediation is master's text (`agent.rs:2182-2187`), without "(20 seconds)" | held on the "exited 1" branch |
| D3 | 021 `ledger_masters` | **refused after 20.093 s:** `company_collection_invalid`, cause `request_deadline_exceeded` | **differs.** Recorded whole and not repeated, so M1 and V1 were not read |
| E | | no company loaded; one tally.exe (13992) owns 9001; `/status` 200; exe hash equal | held |

**`movements_differ_ledgers` (A3, B3).** At 218eaba1, `agent_trial_balance.rs:105` writes the count into the snapshot's frame. The page result (lines 182-192) copies only these from the frame:
- `from`, `to`, `currency`, `read_at`, `totals` and `totals_scope`;
- the scope keys.

The count reaches the reply only as the limitation (`:217-224`). The answer agrees with the code; the sheet's EXPECT reads the frame line as a result field.

**`omitted_for_size` (C1).** `egress_log` sets `truncated` from the tail read whenever the file holds more lines than the limit (`agent.rs:3141`). It writes `omitted_for_size` only when the size trim cuts records (`agent_responses.rs:137-143`). C1's 40 records fitted in 198,814 B, so nothing was cut and the field is absent. C2's trim cut 21 and wrote the field. The answers agree with the code.

**D3.** Tally had just held a Bills request past Bridge's 20 s wait in D2. The marks read that followed also waited its 20 s and was refused.
- **Repeated?** No. The sheet says "record it whole, do not retry", and LAB-COMMON C8 allows no state read after a read that failed.
- **State 16 s later:** `GET /status` answered 200, tally.exe was responding, and the Gateway showed the company with no dialog.
- **Did the marks move?** Not established. The launch had no write setting, and every call of this row is a read.
- **Next:** if the marks are wanted, one `ledger_masters` on BRIDGE CORPUS DENSE in a later sitting can record them.

**Not covered:** #2136's own text (this build predates it). Also not covered: Tally's screens, other dates or books, the desktop app, and any receipt's content beyond its count and order.

**Confidence:** one observation per call, PARTIAL until repeated.

## Files
- `answers/`: every call whole, 000 to 021, with the three `init-*.json`.
  - GUID-shaped strings other than the three synthetic company GUIDs are shown as `<guid>`, including the receipt ids in the `egress_log` answers.
  - `substitutions.txt` lists each changed file with the original's bytes and sha256.
- `client-log.txt`.
- `evidence/`:
  - step 0 and E: `s0-*`, `e-*`;
  - the part checks: `part-a-checks.txt`, `part-b-checks.txt`, `part-c-checks.txt`, `part-d-checks.txt`;
  - the Closing Stock rows: `a3-closing-stock-row.json`, `b3-closing-stock-row.json`;
  - the four folder listings of part C.
- `scripts/`: `part-ab.py` and `step0.ps1`.
- `notes.txt` (UTC). It names no amount from any book.
- `SCREENSHOTS.txt`: the captures by sha256. They stay on the lab PC.
