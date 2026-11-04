# Row 738-A: does a ledger import carrying an opening balance land on a new synthetic company? If it does, build a small debtor and creditor book and keep its audit read

Not for merge. Part of #738. Written 4 Nov 2026. Lines cited are at master 44be49d3, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
- **Part A (its own row):** on a NEW synthetic company, does one ledger `Create` carrying `OPENINGBALANCE`, sent once, land in the book with that opening, read back through the product's `trial_balance` tool?
- **Part B, only if Part A is MET:** build a small book of debtor and creditor ledgers with opening balances and six Journals, back it up, and keep the product's audit read of it with a `lab-writes` build.

Part A not met = STOP after Part A. Part B does not run.

## Why
- **The method (#738, decided 4 Nov).** Accuracy is measured on synthetic lab books, against a blind answer key that an outside CA writes from the book alone. The engine has to run on a read that Tally served, captured by the product (AGENTS.md P1). None of the tax-audit crate's fixtures was served by Tally (`src-tauri/crates/bridge-tax-audit/tests/fixtures/PROVENANCE.md`, lines 13-14).
- **Why Part A stands alone.** The protocol reference records a ledger `Create` with `OPENINGBALANCE` on TallyPrime 7.1 only (§9.4g, PARTIAL, one run, on an existing company). No lab branch holds one: none of the 161 ledger imports on the lab branches carries an opening. This lab runs 7.2. One variable first (P6, P7).
- **Why this book.** The first test to be measured is `stale_balances_41_1`. It reads the Trial Balance only: ledgers under Tally's own `Sundry Debtors` and `Sundry Creditors` groups, their openings, period movements and closings (`src-tauri/crates/bridge-tax-audit/src/stale_balances_41_1.rs`, lines 24-27, 33-51). The book below varies only those.

## THE COMPANY
`BRIDGE ACCURACY LAB`, NEW, synthetic, created by the operator on Tally's screen at the start of Part A. TallyPrime 7.2 Silver, licensed.
- **Create Company:** name `BRIDGE ACCURACY LAB`; Country India; base currency INR; financial year and books beginning 1 April 2025. No password, no security, no TallyVault. Every other field and feature stays at Tally's default.
- **Before creating it:** Select Company must list no company with this name. One that does: STOP.
- **Record:** the company number, the data folder's path (on the lab PC only), and the GUID from the first MARKS read. The GUID is shown in evidence as `<company guid>` until the run coordinator registers it.
- Not a client book, and nothing in it comes from client data. No other company is loaded at any step (C1).

## Rules
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d2…; the whole hash is in `SHA256.txt`. Where this sheet and it differ, the stricter holds.

Differences from LAB-COMMON:
1. **Backup (C5).** C5's backup before the first change cannot apply to a company that does not exist yet. The run coordinator is asked to waive it for the company's creation and A2's one ledger, as for row 2173-L. Part B takes BK0 before its first write and BK1 after its last.
2. **The writes.** Only the three files `a2`, `b2` and `b3` write, each sent once in the row's life (C3), each to THE COMPANY only.
3. **The kept read (Part B).** It stays on the lab PC, listed by name, size and sha256. Whether it is copied to the answers branch is the run coordinator's decision after the row.
4. **No restore.** THE COMPANY is kept, shut, for the CA's copy (BK1) and later re-reads.

## Builds
- **BUILD-M (Parts A and B reads):** the build already staged for rows 2161-C, 1029-R and 1029-T, the 187e882b build.
  - It is the `unsigned-bundle-smoke-windows-latest` artifact of CI run 41772223948, the merge-queue run of #2329 at 187e882b5176a3b577d57f257a6ec4319fc2aecd: artifact id 13295846440, 59,336,955 bytes, GitHub digest `sha256:1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a`, expiring 11 Nov 2026.
  - Check the zip against the digest, and `bridge_mcp.exe` against `mcpb-smoke.json`, before the first call. Either differing: STOP.
  - Over stdio: `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, one new empty `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TERMS_ACCEPTED=true`. `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST`, `BRIDGE_AGENT_ENABLE_TAX_AUDIT` and `BRIDGE_AGENT_REDACTION` unset.
- **BUILD-L (Part B's audit read only):** a `lab-writes` build, built on the lab PC as row 1949's sheet says (no CI workflow builds this feature): in a clean checkout of the master commit the run coordinator names, which must contain #1949's merge (cd100807): `cargo build --locked --release --manifest-path src-tauri/Cargo.toml -p bridge --bin bridge_mcp --features lab-writes`. Record the commit and the exe's sha256.
  - Over stdio: as BUILD-M, plus `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`, `BRIDGE_LAB_WRITES=1` and `BRIDGE_LAB_TARGET_GUID=<company guid>`. `BRIDGE_LAB_DENY_GUIDS`, `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_IMPORT` and `BRIDGE_AGENT_REDACTION` unset. A new empty data folder.
  - Its tool list also carries `lab_import_masters` and `lab_import_vouchers`: record the list and call neither.
  - No BUILD-L (the lab PC cannot build it, or the run coordinator names no commit): Part B stops after B5, with the book built and BK1 taken. Report it.

## The requests (`requests/`)
Each is the text the product's own builders render, at master 44be49d3. A throwaway test that is never committed called them with THE COMPANY's name and wrote their output as UTF-16LE with a byte-order mark. No byte was typed by hand.
- `m00-marks.utf16le.xml`: the company marks read (`render_agent_company_high_water`, `src-tauri/src/agent_read_profiles.rs:574`), the request the product sends.
- `a2-ledger.utf16le.xml`: one ledger, the lab writer's All Masters import (`render_master_batch_xml` with `render_ledger_xml`, `src-tauri/src/agent_lab_import.rs:784` and :713, which renders through `LedgerCreate`, `src-tauri/crates/bridge-tally-protocol/src/ledger_create.rs:36`).
- `b2-ledgers.utf16le.xml`: fourteen ledgers, the same builder.
- `b3-vouchers.utf16le.xml`: six Journals, the lab writer's voucher import (`render_voucher_batch_xml`, `agent_lab_import.rs:2149`), the accounting shape row 2173-L sent. No voucher number; each narration ends with the writer's `[BRIDGE-LAB:<id>]` marker and each voucher carries its own `REMOTEID`.

Sizes: the marks answer has a READ BOUND of 16,384 bytes, each import's answer 65,536 bytes. The import files open with `<?xml version="1.0" encoding="UTF-8"?>`, as every import from the product's writers does; the bytes are UTF-16 all the same, and C3's `Content-Type` says so. `SHA256.txt` has three columns (sha256, bytes, path) and does not list itself: compare each line with Get-FileHash and the file's size.

## The book
Amounts are in Tally's sign: negative is a debit, positive a credit. All ledgers are bill-wise `No`.

| ledger | group | opening | sent in |
|---|---|---:|---|
| ACC Creditor 01 | Sundry Creditors | 1000.00 | a2 |
| ACC Creditor 02 | Sundry Creditors | 2500.00 | b2 |
| ACC Creditor 03 | Sundry Creditors | 3000.00 | b2 |
| ACC Creditor 04 | Sundry Creditors | 4000.00 | b2 |
| ACC Creditor 05 | Sundry Creditors | 1.00 | b2 |
| ACC Creditor 06 | Sundry Creditors | none | b2 |
| ACC Creditor 07 | Sundry Creditors | -600.00 | b2 |
| ACC Debtor 01 | Sundry Debtors | -1200.00 | b2 |
| ACC Debtor 02 | Sundry Debtors | -5000.00 | b2 |
| ACC Debtor 03 | Sundry Debtors | -1.01 | b2 |
| ACC Debtor 04 | Sundry Debtors | -800.00 | b2 |
| ACC Loan 01 | Unsecured Loans | 2000.00 | b2 |
| ACC Furniture | Fixed Assets | -4899.99 | b2 |
| ACC Expenses | Indirect Expenses | none | b2 |
| ACC Income | Indirect Incomes | none | b2 |

The openings sum to 0.00 once b2 is in. After a2 alone, Tally will show a difference in opening balances of 1000.00: that is expected, not a STOP.

**The Journals (`b3`):**

| # | date | debit | credit | amount |
|---|---|---|---|---:|
| 1 | 10 Jun 2025 | ACC Creditor 04 | ACC Income | 1000.00 |
| 2 | 15 Jul 2025 | ACC Creditor 03 | ACC Income | 500.00 |
| 3 | 15 Sep 2025 | ACC Expenses | ACC Creditor 03 | 500.00 |
| 4 | 20 Nov 2025 | ACC Expenses | ACC Debtor 02 | 2000.00 |
| 5 | 5 Dec 2025 | ACC Expenses | ACC Debtor 04 | 300.00 |
| 6 | 12 Jan 2026 | ACC Debtor 04 | ACC Income | 300.00 |

**What the book must hold after B3**, for 1 Apr 2025 to 31 Mar 2026 (debit and credit are the period totals):

| ledger | opening | debit | credit | closing |
|---|---:|---:|---:|---:|
| ACC Creditor 01 | 1000.00 Cr | 0 | 0 | 1000.00 Cr |
| ACC Creditor 02 | 2500.00 Cr | 0 | 0 | 2500.00 Cr |
| ACC Creditor 03 | 3000.00 Cr | 500.00 | 500.00 | 3000.00 Cr |
| ACC Creditor 04 | 4000.00 Cr | 1000.00 | 0 | 3000.00 Cr |
| ACC Creditor 05 | 1.00 Cr | 0 | 0 | 1.00 Cr |
| ACC Creditor 06 | 0 | 0 | 0 | 0 |
| ACC Creditor 07 | 600.00 Dr | 0 | 0 | 600.00 Dr |
| ACC Debtor 01 | 1200.00 Dr | 0 | 0 | 1200.00 Dr |
| ACC Debtor 02 | 5000.00 Dr | 0 | 2000.00 | 3000.00 Dr |
| ACC Debtor 03 | 1.01 Dr | 0 | 0 | 1.01 Dr |
| ACC Debtor 04 | 800.00 Dr | 300.00 | 300.00 | 800.00 Dr |
| ACC Loan 01 | 2000.00 Cr | 0 | 0 | 2000.00 Cr |
| ACC Furniture | 4899.99 Dr | 0 | 0 | 4899.99 Dr |
| ACC Expenses | 0 | 2800.00 | 0 | 2800.00 Dr |
| ACC Income | 0 | 0 | 1800.00 | 1800.00 Cr |

## Part A: one ledger with an opening balance
A0. **Create THE COMPANY** on Tally's screen as above. Screenshot the Create Company screen before accepting, and the Gateway after. Then C2 step 0, with `m00-marks` as its MARKS read (`m-a0`): EXPECT one COMPANY row. Record its GUID, ALTVCHID and ALTMSTID (M-a0), Tally's release, Tally's date and the PC's date.

A1. **BUILD-M:** `list_companies`: EXPECT exactly THE COMPANY, with the GUID from m-a0 and `books_from` 20250401. `tally_status`: record release, licence and mode. Education mode: STOP.

A2. **Send `a2-ledger`** (C3: a write, sent once). Then a screenshot of the whole Tally window (a dialog = C7, STOP). EXPECT CREATED 1, every other counter 0, no LINEERROR (C4). Then `m00-marks` (`m-a2`): EXPECT ALTMSTID = M-a0 + 1 and ALTVCHID unchanged.

A3. **The read.** BUILD-M `trial_balance` with `company_guid` = THE COMPANY's GUID, `from` `2025-04-01`, `to` `2026-03-31`. Keep the whole answer. If it gives a `next_offset`, call again with that offset until none is given, keeping each page. EXPECT a row for `ACC Creditor 01` under `Sundry Creditors` with:
- opening 1000.00 on the credit side;
- debit and credit both nil;
- closing equal to its opening.

Record the row exactly as the tool gives it (field names, sign and empty values), with `totals` and any `movements_differ_by`.

A4. **Part A's verdict.**
- **MET:** A2's counters and marks as EXPECTed, and A3's row as EXPECTed. Go to Part B.
- **NOT MET:** any of them differs. Record it whole, run C10, report, and STOP. THE COMPANY keeps its one ledger: say so in the report. Part B does not run.

## Part B: the book and its audit read (only if A4 is MET)
B0. C2 step 0 again, with `m00-marks` (`m-b0`): EXPECT the marks equal m-a2's.

B1. **BK0** (C5) of THE COMPANY to a NEW EMPTY folder `C:\bridge-lab\738a\backups\bk0\`. Then `m00-marks` (`m-b1`).

B2. **Send `b2-ledgers`** (a write, sent once), then a screenshot. EXPECT CREATED 14, every other counter 0, no LINEERROR. `m00-marks` (`m-b2`): EXPECT ALTMSTID = m-b1's + 14, ALTVCHID unchanged.

B3. **Send `b3-vouchers`** (a write, sent once), then a screenshot. EXPECT CREATED 6, every other counter 0, no LINEERROR. `m00-marks` (`m-b3`): EXPECT ALTVCHID = m-b2's + 6.

B4. **The book, read back.** BUILD-M `trial_balance` as in A3. EXPECT every row of "What the book must hold after B3", and the totals to balance. A row that differs, or a ledger missing or extra (Tally's own default ledgers, `Cash` and `Profit & Loss A/c`, may also be listed: record them, not a STOP): record it whole and STOP. `ACC Creditor 06` with a zero balance may be listed or absent: record which.

B5. **BK1** (C5) of THE COMPANY to a NEW EMPTY folder `C:\bridge-lab\738a\backups\bk1\`. Then `m00-marks` (`m-b5`): EXPECT equal to m-b3's. BK1 is the book the CA will get; it stays on the lab PC, listed by name, bytes and sha256.

B6. **The audit read, kept.** Quit BUILD-M. Start BUILD-L. `tools/list`: record it whole. `list_companies`: EXPECT THE COMPANY's GUID. Then `audit_test_run` with:
- `company_guid` = THE COMPANY's GUID;
- `from` `2025-04-01`, `to` `2026-03-31`;
- `test` `cash_44ab`;
- `cash_groups` `["Cash-in-Hand"]` and `bank_groups` `["Bank Accounts"]`.

That is the call of rows 1949 and 2173-L, unchanged apart from the company. Time it, and keep the whole answer and its `egress_log` lines. EXPECT no refusal, and a state of `completed` or `completed_with_failed_book_checks`: record which, with every book check named. The test's own result is not this row's question: record it, do not judge it.

B7. **What it kept.** List `<data dir>\lab\` recursively (names, sizes, sha256 of every file). EXPECT:
- exactly one folder under `audit-reads\`, named by a 64-hex sha256, holding `manifest.json` and a `parts\` folder of plain files, and nothing else;
- `lab-manifest.jsonl` with exactly one line, whose `company_guid` is THE COMPANY's and whose `manifest_sha256` is the folder's name.

Record the sha256 of `manifest.json` and whether it equals the folder's name. Copy nothing out of the folder.

B8. **The same read again**, at least two seconds after B6, with exactly B6's arguments. EXPECT:
- a second folder, with the first folder's files unchanged (their sha256 as at B7);
- the two `manifest.json` files differing only in `created_at` and `read_at` (a parsed comparison);
- each file in the two `parts\` folders byte-equal to its namesake.

Record any other difference whole: a part that differs between two reads of an unchanged book is a FINDING, not a STOP.

B9. `m00-marks` (`m-b9`): EXPECT equal to m-b5's. Then C10: shut THE COMPANY. It is not restored.

PASS = Part A MET, B0 to B9 recorded whole, and every EXPECT checked.

## Stop rules
LAB-COMMON's, and:
- any dialog or prompt on Tally's screen;
- any tool call with no answer in 10 minutes;
- Education mode;
- a marks read that moves when this sheet expects it not to;
- any refusal at A1, A3, B4 or B6: keep it whole, report its code, and STOP.

## Record
- Part A: the company number, `<company guid>`, m-a0 and m-a2 (bytes, sha256, ALTVCHID, ALTMSTID), A2's answer and counters, and A3's `ACC Creditor 01` row exactly as given.
- Part B: every import answer and its counters, every marks read, B4's whole table set against the table above, BK0 and BK1 (folder, name, bytes, sha256, the time from the Restore list), B6's answer and its time, B7's and B8's listings, and the manifest comparison.
- Releases, dates, the client, both builds' hashes and BUILD-L's commit.
- A notes file with UTC times, one line per step.

## Keep
On `lab/738-a-answers`, with `SHA256.txt`:
- every tool answer and every raw answer whole, with its size and sha256;
- a RESULTS.md with Part A's verdict first.

Screens, BK0, BK1 and the kept audit reads stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.

## Code admits the trigger (44be49d3)
- **The writes are raw imports by the operator's curl (C3), not Bridge calls.** Their text is the lab writer's own (`agent_lab_import.rs:713`, :784, :2149). `render_ledger_xml` emits `OPENINGBALANCE` only when non-zero (:713-738), which is why `ACC Creditor 06`, `ACC Expenses` and `ACC Income` carry none.
- **The read in A3 and B4:** `trial_balance` (`src-tauri/src/agent_trial_balance.rs`) answers one row per ledger with `parent`, `opening`, `debit`, `credit` and `closing` (:12-15).
- **The audit tool admits one test only:** `ADMITTED` is `["cash_44ab"]` (`src-tauri/src/agent_audit_run.rs:24`). That is why B6 calls `cash_44ab` and not `stale_balances_41_1`.
- **The read it keeps does not depend on the test.**
  - The cash and bank groups enter only the engagement the engine is given (`src-tauri/src/agent_audit_read.rs:264-265`), not the parts it reads (groups, ledgers, trial balance, the voucher window and the closing marks, :458-616).
  - So `stale_balances_41_1` can later be run on the kept read in the crate itself (`stale_balances_41_1_on`, `src-tauri/crates/bridge-tax-audit/src/lib.rs:1695`).
  - That run is not part of this row.
- **Keeping the read:** with `lab-writes` and lab writes on, `audit_test_run` refuses unless the company is the lab's target (`admit_audit_read_capture`, `src-tauri/src/agent_lab.rs:263`), then writes the admitted read (`persist_audit_read`, :294, called from `agent_audit_run.rs:139`).
- **The period** is inside the rules year the tool accepts: 1 Apr 2025 to 31 Mar 2026 (`RULES_YEAR_FROM` and `RULES_YEAR_TO`, `agent_audit_run.rs:75-76`, checked at :89).

## Who sends each request
| step | request | sent by |
|---|---|---|
| A0, A2, B0-B3, B5, B9 | marks, raw | the operator's curl on 9001 |
| A2, B2, B3 | the three imports, raw | the operator's curl on 9001, each once |
| A1, A3, B4 | company list, status, trial balance | BUILD-M's tools |
| B6, B8 | the audit read | BUILD-L's `audit_test_run` |
