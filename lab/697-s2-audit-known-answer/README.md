# 697-S2 — audit_test_run (cash_44ab) live, as a known-answer test (21 Oct 2026). Synthetic company only. Read only.

About 30 minutes. Writes nothing to Tally. Run AFTER 697-S1 (its step 3 says whether BRIDGE PILOT LAB has vouchers in
FY 2025-26). The tool's rules cover only 1 April 2025 to 31 March 2026, so the company MUST hold cash and bank vouchers
in that year; on a company with none the run proves nothing (vacuous) — then STOP at step 2 and report.

**COMPANY (from 697-S1 / 1605-S1, 21 Oct).** BRIDGE PILOT LAB has 0 vouchers in FY 2025-26: not usable. Use BRIDGE SHAPE LAB:
an FY 2025-26 book with every standard voucher type, cheque bank allocations, and optional, post-dated and cancelled
vouchers (so the tool's exclusions are exercised too). Its state after the ITC keying sitting was never confirmed restored;
that does not matter here, because the known answer is read from Tally's own screen in the same sitting.

**BUILD.** The tool is not in any release. Use the Windows bundle of pull request #1733 at head 0a9f4b2a6: Bridge CI run
39573358245, artifact `unsigned-bundle-smoke-windows-latest` (id 12336427199, 60,823,134 bytes, digest
sha256:f8f821d8123eb26eba6d821e1e89039cb527bd7c2d8552b3bfe6aa0264b68c4a). Verify the digest. Do NOT replace the standing
release-candidate extension: run the bundle's `bridge_mcp.exe` directly over stdio (unpack the .mcpb, a zip). Environment
for that process: `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`, a fresh
empty `BRIDGE_AGENT_DATA_DIR`, and the Terms value set by the lab's own standing setup (the owner's act; no script here
sets it). If that is not possible today, use the lab's Claude Desktop with this bundle installed and its setting on, and
restore the standing entry after (the lab's act; record path + hash both times).

**FILES** (`SHA256.txt` here): `a-list.jsonl` (initialize, tools/list, list_companies) and `b-run.TEMPLATE.jsonl`
(three audit_test_run calls and an egress_log read). In `b-run`, replace `<GUID>` (three times) with the GUID from step 1
and, if the book's cash or bank ledgers sit under other group names, the group names, saying so in the report. Send each
file as stdin, one line per request, and keep stdout and stderr byte for byte (`answers\a.out`, `answers\b.out`).

**STEPS**
0. Tally: the chosen synthetic company loaded alone on 9001; `Education mode` No (Gateway > F1 Help or company info).
1. Send `a-list.jsonl`. EXPECT `tools/list` to include `audit_test_run`; `list_companies` one company. Record its GUID.
2. Known answer, from Tally's own screen (independent of Bridge): Display More Reports > Account Books > Cash/Bank Book(s),
   period 1-4-2025 to 31-3-2026: record the Debit and Credit totals of the Cash-in-Hand group and of the Bank Accounts group.
   Then Display More Reports > Day Book (or Account Books > Contra Register), same period: record the total of Contra
   vouchers' debits and credits to cash and to bank ledgers, and the count of post-dated, optional or cancelled vouchers
   touching them. If both groups show no transactions: STOP (vacuous), report.
3. Send `b-run.jsonl` (the template, filled). Keep the answers.

**PASS** only if all hold (the numbers are checked from the answers afterwards; the lab only records):
- id 4: `result.state` is `completed` or `completed_with_failed_book_checks` (any failed check is listed and explained later);
  `cash_44ab.cash_receipts` = Cash-in-Hand debits − Contra debits to cash − excluded vouchers' cash debits (step 2), in paise;
  likewise cash_payments (credits), bank_receipts, bank_payments; `book_checks.evaluated` lists 8 codes.
- id 5: refused, code `BIND-GROUP-UNKNOWN`. id 6: refused, code `audit_groups_overlap`, and id 7's receipt lines show no
  Tally request for id 6.
- Any other refusal on id 4 is a FAIL with its code (repeat once only for `C6-moved` or `voucher_window_changed_during_read`).

**REPORT:** the two stdout files, stderr byte counts, the screen totals of step 2 (photo or typed), the company name, GUID and
Tally release/edition, the bundle digest check. Confidence: one run, PARTIAL.

**Not established:** a book big enough to divide the window; Gold; other releases; any client book.
