# C1 (#1699): does a read of a renamed company say why it was refused?
WHY: a read refused because the named company is not loaded, was renamed or is listed twice now carries a cause and a next step. It has only run in the simulator; this replaces the issue's "not established" line with a measurement.
COMPANY: any BRIDGE synthetic company X that is loaded together with at least one other BRIDGE company. Note the licence mode (licensed, or Education with day-1/2/31 boundaries) and the TallyPrime version in the record.
MODE: B (standing config entry).
STEPS (one request at a time):
1. `list_companies`; record the GUID and exact name of X and of the other loaded companies. `tally_status` for X: the voucher mark before (egress line).
2. Backup BK1 of X into a NEW empty folder (name, size, sha256).
3. In Tally the coordinator alters X's name (add one letter) and saves. Do NOT call `list_companies` again.
4. `ledger_movement` for X's GUID, one calendar month, exactly as the built tool takes it. Save the full answer.
5. All-companies outstandings from the desktop app (the "All clients" view), once, while X is still renamed. Note X's row and its reason text.
6. The coordinator renames X back (restore BK1 only on the run coordinator's explicit RESTORE). `tally_status`: the voucher mark after.
EXPECT (record, not predicted): the refusal for X in step 4: its `code`, its `cause` and its `remediation`. A refusal earlier than the identity check (for example the tool cannot find the company by GUID) is a finding, not a failure: record its code.
PASS = no data is returned for X, and the refusal either carries a `cause` beginning `company_identity_` with a remediation that says to open the company in Tally and call `list_companies` again, or the earlier refusal is recorded with its code. Step 5: X's row shows a reason that names the identity cause in words, not "company read failed", and the other companies' rows are unaffected.
FAIL = data returned for a company the name no longer matches, or a bare operation code with neither cause nor remediation on the identity path.
STOP RULES: any dialog before a call; a voucher mark that moves; never Escape on an alteration screen; no master deleted.
