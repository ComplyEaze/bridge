# 2068 trial balance on a non-integrated book (#2068, Part of #692): what `trial_balance` reports for the Closing Stock ledger, read only

Not for merge. About 20 minutes. Read only: no key is pressed in Tally except to open and view, nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). `SHA256.txt` lists every file here with its bytes; check it before step 0.

## What it decides
On a book with "Integrate Accounts with Inventory" set to No, the ledger collection of the #692 NONINT row showed the `Closing Stock` ledger (under Stock-in-Hand) with `TBALCLOSING` -1,000.00, its opening, while Tally's Profit and Loss and Balance Sheet used the 1,400.00 entered in the ledger's Closing Balance field (the 400.00 appeared only as `DEBITTOTALS` -400.00). `trial_balance` reads `TBALCLOSING` too. This sheet records, on the same book and the same dates, what `trial_balance` reports for that ledger and whether its totals tie. It decides nothing else, and it says nothing about the tax-audit book (not a lab question).

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** As the lab order's WHO ACTS section sets out, on the owner's standing word of 27 Oct 2026: the lab does every step here on the synthetic company, and records each as that section directs (LAB-COMMON C11's rule on names applies to what is pushed).

**THE COMPANY** is `BRIDGE NONINT LAB`, the synthetic book of the #692 NONINT row (company number 100076), as that row left it: shut, with the Features of Integrate Accounts with Inventory and GST set to No, and the vouchers and ledgers it keyed. It must hold the ledger `Closing Stock` under Stock-in-Hand with 1,400.00 Dr entered as its closing balance at 31 March 2026. If it is absent, or the book differs from that row's (its marks, step 1), STOP and report: this sheet creates and keys nothing. Only THE COMPANY is loaded, alone, from step 0 to the last step.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Tool calls only.** The reads are calls of ComplyEaze Bridge's built tool, one at a time. No request is sent by curl, no MARKS request file is used: C2 step 0 runs without its MARKS request (its `GET /status` checks still run), and the marks come from `ledger_masters` (rule 2).
2. **Marks.** `tally_status` carries no marks. M (ALTMSTID) and V (ALTVCHID) are read with `ledger_masters` (basic, first page, no `snapshot_id`): `snapshot.master_alter_id` and `snapshot.voucher_alter_id`; record the whole answer and `snapshot.reused`, which must read false.
3. **Grant.** Nothing is written to Tally. Posting and ledger creation are not enabled.
4. **One Tally.** At step 0 and at the end, as C2 a and b set out (the process list and the listener on port 9001) plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001. Record all three. A screen-control tool brings no window forward (bringing TallyPrime forward once started a second `tally.exe`).

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` of master `609776ae9f5cec48e6563f165480d3fe1adae445` (merge-queue run 40366984202, artifact `unsigned-bundle-smoke-windows-latest` id 12670984633, `bridge_mcp.exe` sha256 `4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44`), or the 0.6.1 release build; either holds `trial_balance` (absent from `tools/list` = STOP). Record the exe's sha256 before the first call and after the last: another sha256 = STOP. Record `initialize` and `tools/list` whole.

Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2068\data`), `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), `BRIDGE_AGENT_REDACTION` unset. Writes stay off.

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded, without its MARKS request: the screenshot, the process list and port 9001 (rule 4), `GET /status`. Hash the exe. Select Company: THE COMPANY is listed; load it alone. Screenshot of the Gateway showing it loaded alone, with the Current Period.
1. `list_companies`: EXPECT exactly THE COMPANY with `identity_state` `verified_tuple`; record its `guid` (the `company_guid` of every later call) and the company number. `tally_status`: release (7.2 expected), licence, Education mode. `ledger_masters` (basic, first page): M0, V0 and the ledger rows. EXPECT M0 215, V0 3, and 8 ledgers including `Closing Stock`, as the #692 NONINT row's last marks read; a difference is recorded and the run coordinator decides before step 2.
2. **Size first.** From step 1: the number of ledgers (EXPECT 8). The `trial_balance` read is one small book's report (no voucher scan): one call with `limit` 100 is the plan. If step 1 shows more than 100 ledgers, STOP.
3. **The read.** `trial_balance` with `company_guid`, `from` 20250401, `to` 20260331 (the dates of the #692 NONINT row's reads), `limit` 100. One call. Record the answer whole with its size and sha256. From it, record:
   - the `Closing Stock` row: every amount field it carries as returned (opening, debit, credit, closing, and the side of each), unchanged;
   - the other rows' amounts, so the totals can be recomputed;
   - `totals` (and `totals_scope`, `ledgers_scope`, anything under `foreign_currency_ledgers_excluded`), and whether the report ties: debit total equal to credit total, as the answer states and as you recompute from the rows;
   - `snapshot` (`id`, `master_alter_id`, `voucher_alter_id`, `read_at`, `reused`) and any `read_cost`.
   A refusal: record it whole and STOP (do not retry unless the answer says that is safe).
4. **Tally's own Trial Balance, for the same dates.** On Tally's screen, Display, Trial Balance, period 1-4-2025 to 31-3-2026 (view only; leave by Ctrl+Q after a screenshot shows the screen's title; never Escape; C7). Screenshot of the whole report, including the `Closing Stock` line and the grand total. Record the `Closing Stock` figure and the grand totals as shown, as text, in the notes.
5. `ledger_masters` (basic): M1, V1. EXPECT M1 = M0 and V1 = V0 (no read moves a mark); a difference is recorded as a result.
6. LAB-COMMON C10 (close, leaving THE COMPANY as it was found: shut). Rule 4 check once more. `GET /status` after the last request. Hash the exe.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog this sheet does not describe; a second `tally.exe`; a mark that moves; a `trial_balance` answer that is a refusal.

## What the row records, and what it does not
It records: the `Closing Stock` row of `trial_balance` (1,000.00, 1,400.00, or another figure, beside the screen's own line), and whether the trial balance ties. Either result is a record, not a pass. It does not read the tax-audit book, the Profit and Loss or the Balance Sheet again, or any other window of dates.

**Confidence.** One observation, PARTIAL until repeated.

## Evidence
- Every tool answer whole with its size and sha256 (C11), the build's commit and exe sha256 before and after, screenshots listed by name, size and sha256 (never pushed), and a notes file with UTC times and one line per step, kept as C11 sets it out (no roles and no people).
- THE COMPANY's GUID is synthetic: before the evidence is pushed it goes into the merge gate's synthetic-UUID registry, or the evidence shows it as `<company guid>`.
- Evidence on a new branch, `lab/2068-nonint-trial-balance-answers`, with `SHA256.txt`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
