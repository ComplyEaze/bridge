# LR5 — `audit_test_run` 0.7.0 test plan and lab run sheet (written 25 Oct 2026; run when the Tally licence is renewed)

`audit_test_run` ships with no claim tested against a live Tally; testing and fixes are for 0.7.0 / 0.7.1. This sheet lists
every claim the tool makes (docs/agent/README.md "Tax-audit test run", master f905ca09f), what proves it today without Tally,
and the live step that would prove it. It supersedes 697-S2 (lab/697-s2-audit-known-answer 1e157d32): same company and
known-answer method, more claims, a newer build.
Synthetic BRIDGE companies only. READ-ONLY: nothing is written to Tally, no Tally setting changes.

## 1. Claims and their evidence

Today = what proves it without Tally (tests on master, `src-tauri/src/...`). Live = the step below that proves it on a real Tally.
Captured = live bytes that already exist on a lab/ branch (none of them came through the tool itself).

| # | Claim | Today | Live | Captured |
|---|---|---|---|---|
| C1 | Off by default: absent from `tools/list`, a call refused `audit_tool_disabled` before any request | `the_tool_is_absent_and_refused_while_its_setting_is_off`; bundle smoke | D1-D2 | - |
| C2 | With the setting on, listed once (one more tool) | protocol test; bundle smoke (switch adds only it) | A1 | - |
| C3 | Only `cash_44ab` admitted; any other test refused `audit_test_not_admitted` before a request | `the_catalog_lists_exactly_the_admitted_tests`; `arguments_that_cannot_be_honoured...` | B5 + egress | - |
| C4 | A period outside 1 Apr 2025-31 Mar 2026 refused `audit_period_outside_ay_2026_27` before a request | `a_period_outside_the_year...` | B6, B7 + egress | - |
| C5 | A part-year inside the year is read and judged on the whole year's thresholds | replay (synthetic) | B2 (known answer for H1) | - |
| C6 | Group lists required, no default; a group in both refused `audit_groups_overlap` before a request | `arguments_that_cannot_be_honoured...` | B4 + egress | - |
| C7 | A group not in the book refused `BIND-GROUP-UNKNOWN` (after the read) | engine tests | B3 | - |
| C8 | The read: marks, company object, groups, ledgers, TB of the period, voucher types, census, every voucher of the period (<=128 parts), marks again; one request at a time between two identity checks | `a_clean_replay_is_admitted_and_sends_only_exports` (synthetic book) | B1 + egress (count and order of requests) | 697-S0A: every part shape once, BRIDGE PILOT LAB FY 2026-27 (lab/697-s0a-answers); 697-S1/1605-S1: census answers |
| C9 | A book that has never held a voucher (voucher mark 0, so no census is sent) refused `audit_window_not_counted` | `a_window_no_census_counted_is_refused` | C2 (a new empty synthetic company; optional, it creates a company) | - |
| C9b | A period with no voucher in a book that has vouchers elsewhere. Not stated in the docs. In code a census of zero vouchers counts as counted (`names_every_guid` holds for an empty census), so the read is admitted; here the period also lies wholly before the books' first day, which the engine may refuse by rule | none | C1 (PILOT LAB, FY 2025-26: 0 vouchers, books from 2026-04-01): RECORD the outcome; a `completed` result with zero figures is a FINDING (it would read as 0% cash) | 697-S1 census: PILOT LAB FY 2025-26 = 0 |
| C10 | Company object of another company refused by code | `a_company_object_of_another_company_is_refused_by_code` | not live (needs a wrong answer) | - |
| C11 | Book changed during the read: `C6-moved` / `voucher_window_changed_during_read`; may be retried | `marks_that_moved_*` | optional S1 (a hand edit mid-read; WRITE, coordinator only) | - |
| C12 | A window too heavy for one request by date: `audit_window_divided_by_alterid` | `a_window_divided_by_alterid_cannot_become_a_read` | not planned (needs a day over the request cap) | - |
| C13 | Deadline 20 s per part; abandoned part owes a drain; `audit_part_deadline_exceeded` / `audit_part_drain_required` | runtime part tests | NOT live (a deliberate abandon can wedge the gateway) | - |
| C14 | Result: `result.state` completed / completed_with_failed_book_checks; figures with id, value (int or null), unit, definition_text; findings; population note; book_checks evaluated and failed codes | `a_completed_run_returns_figures...`; projection tests | B1, B2 | - |
| C15 | Figures equal the books: cash_receipts / cash_payments / bank_receipts / bank_payments = debits / credits to ledgers under the named groups, Contra excluded, optional / cancelled / post-dated excluded; shares = cash / (cash + bank) in bp; threshold Rs 10 cr if both shares <= 5%, else Rs 1 cr | engine parity at pin 6168aeaa on ONE real book with zero cash payments | B1, B2 known answer (section 3) | - |
| C16 | No ledger, group, party, voucher number or GUID of the book in a result, a refusal or a key | `...no_name_of_the_book`; projection tests | B1-B7 leak check (section 3, step 6) | - |
| C17 | Refused while a redaction setting is on (`audit_redaction_unsupported`) before a request | `a_redaction_setting_is_refused_before_any_request` | E1 | - |
| C18 | Nothing written to disk or to Tally; the read is not kept | replay tests (Exports only) | B1 + egress (only Export requests) + data folder listing before/after | - |
| C19 | A result past 64 figures or 32 findings refused `audit_result_too_large`, never cut short | `a_result_past_the_bounds...` | not live (cash_44ab emits 7 figures) | - |

Not claimed and not tested here: Gold / multi-user, other releases than the lab's TallyPrime 7.2, Education mode (STOP), any client book.

## 2. Build and setup

- Build: the staged Windows bundle of master `2306d86df` (CI run 40077056130, artifact 12553782422
  `unsigned-bundle-smoke-windows-latest`, 60,880,528 bytes, digest
  `sha256:74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e`). Verify the digest before unpacking.
- What this build can and cannot test: it holds `audit_test_run` (f905ca09f) and the audit read as master holds them today
  (later master changes to the tool's catalog are wording only), so every claim C1-C19 is the tool's behaviour in it. It
  predates two open changes: a refusal inside the engine of a period outside the rules' year (the tool already refuses
  such a period before any request, C4, so the observable answer is the same), and divergence and mutation tooling that
  the tool does not use. Neither is tested here.
- The tool runs as the bundle's `bridge_mcp` over stdio (unpack the .mcpb), as 697-S2 described, so the standing extension entry is not
  changed. Environment: `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a fresh empty `BRIDGE_AGENT_DATA_DIR` per file, the Terms
  value of the lab's standing setup (the owner's act), and per file:
  - A, B, C: `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`;
  - D: the variable unset;
  - E: `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true` and `BRIDGE_AGENT_REDACTION=mask_parties`.
- Send each file as stdin, one line per request; keep stdout and stderr byte for byte (`answers/<file>.out`, `.err`). In each TEMPLATE,
  replace `<GUID>` with the GUID from A's `list_companies`; if the book's cash or bank ledgers sit under other groups than
  `Cash-in-Hand` and `Bank Accounts`, change the group names and say so.
- Tally: the lab's TallyPrime 7.2, licensed, Education mode No (else STOP), one company loaded alone on port 9001. Record the
  release and edition shown; the earlier captures of the audit read (697-S0A, 697-S1) were on 7.1 Silver.

## 3. Steps (one file at a time; one company loaded alone)

BRIDGE SHAPE LAB (FY 2025-26 book, every standard voucher type; the only lab book known to hold FY 2025-26 cash and bank vouchers):
1. A `a-list.jsonl`: EXPECT `tools/list` holds `audit_test_run` once; `list_companies` one company. Record the GUID.
2. Known answer from Tally's own screens (independent of Bridge), for 1-4-2025 to 31-3-2026 AND for 1-4-2025 to 30-9-2025:
   Account Books > Cash/Bank Book(s): the Debit and Credit totals of the Cash-in-Hand group and of the Bank Accounts group;
   Contra Register: the Contra debits and credits to cash and to bank ledgers; and the count and cash/bank amounts of optional,
   cancelled and post-dated vouchers touching them. If both groups show no transaction in the year: STOP (vacuous).
3. List the data folder (names, sizes). B `b-shape-lab.TEMPLATE.jsonl` (filled). List the data folder again.
4. EXPECT (checked from the answers afterwards; the lab only records):
   - id 10 (full year) and id 11 (H1): `result.state` completed or completed_with_failed_book_checks (failed codes listed);
     cash_receipts = Cash-in-Hand debits - Contra debits to cash - excluded vouchers' cash debits, in paise; likewise cash_payments
     (credits), bank_receipts, bank_payments; cash_share_* = cash / (cash + bank) in bp (null when cash + bank is 0);
     applicable_threshold 10,000,000,000 paise (Rs 10 crore) if both shares <= 500 bp, else 1,000,000,000 (Rs 1 crore); `book_checks.evaluated` lists 8 codes.
     A non-Contra voucher with both a cash and a bank line counts in both legs: note any.
   - id 12: refused `BIND-GROUP-UNKNOWN`. id 13: refused `audit_groups_overlap`. id 14: `audit_test_not_admitted`.
     ids 15, 16: `audit_period_outside_ay_2026_27`.
   - id 17 (egress_log): requests to Tally only for ids 10, 11, 12 (and only Exports); none for ids 13-16.
   - The data folder holds nothing new but the receipt journal.
5. Repeat id 10 once only if it was refused `C6-moved` or `voucher_window_changed_during_read`; any other refusal is a FINDING with its code.
6. Leak check (afterwards, on the saved answers): no ledger, group, party name, voucher number or GUID of the book appears in any
   answer of ids 10-16 (the company GUID appears only where the call echoes it). The coordinator runs this against Tally's lists.

BRIDGE PILOT LAB (0 vouchers in FY 2025-26 per 697-S1):
7. C `c-pilot-lab.TEMPLATE.jsonl`: RECORD id 20 whole (no prediction: refused by an engine rule code, `audit_window_not_counted`, or `completed` with zero figures and null shares). id 21: the requests sent (expected: the census, then at most one voucher read of zero rows). A `completed` answer is a FINDING for 0.7.x.
7b. Optional (it creates a company; coordinator's word, BRIDGE-named, deleted after): a new empty synthetic company, then file C again with its GUID: EXPECT id 20 refused `audit_window_not_counted` and no census sent.

Any company (SHAPE LAB is fine):
8. D `d-setting-off.TEMPLATE.jsonl` with the variable unset: EXPECT `audit_test_run` absent from `tools/list`; id 30 refused
   `audit_tool_disabled`; no Tally request after the identity reads.
9. E `e-redaction-on.TEMPLATE.jsonl`: EXPECT id 40 refused `audit_redaction_unsupported`, no Tally request.

Optional, WRITE (only on the coordinator's word, after a backup into a new empty folder, restored after): S1 — during id 10 on a copy
of SHAPE LAB, alter one voucher's narration by hand; EXPECT `C6-moved` or `voucher_window_changed_during_read`, then a clean retry.

## 4. Report

The .out/.err files with sizes and sha256, the screen totals of step 2 (photo or typed), company name, GUID, Tally release and
edition, the bundle digest, the data folder listings. Evidence on a new branch lab/697-0.7.0-answers with SHA256.txt; send the SHA
to the requester before pushing. Confidence: one run, PARTIAL.

## 5. What a PASS proves, and what it does not

PASS proves C1-C8, C14-C18 on one synthetic FY 2025-26 book, one release, and records C9b; C9 only if step 7b ran. It does not prove C10-C13 or C19 live (kept on replay
tests), a divided window, a book with more than one cash or bank group, a client book, Gold, or other releases.
