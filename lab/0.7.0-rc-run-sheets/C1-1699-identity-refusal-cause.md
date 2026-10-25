# C1 (#1699): does a read of a renamed company say why it was refused?
WHY: a read refused because the named company is not loaded, was renamed or is listed twice now carries a cause and a next step. It has only run in the simulator; this replaces the issue's "not established" line with a measurement.
COMPANY: BRIDGE RENAME LAB (X, new) and BRIDGE RENAME PEER LAB (new): exactly these two are loaded, nothing else. Each is small by construction: three ledgers and no voucher, so every read below is tiny and the All-clients sweep reads nothing large. Both are NEW: if a company or data folder of either name exists on the PC, STOP and report it; never reuse one. Create each with books beginning 1 April 2025. If `list_companies` shows any other company, or either of these twice, STOP before any call. Note the licence mode (licensed, or Education with day-1/2/31 boundaries) and the TallyPrime version in the record.
MODE: B (standing config entry).
STEPS (one request at a time):
1. `list_companies`; record the GUID and exact name of X and of the other loaded companies. `tally_status` for X: the voucher mark before (egress line).
2. OWNER STEP (the owner works Tally's screen, one step at a time; no lane touches it): backup BK1 of X into a NEW empty folder (name, size, sha256).
3. OWNER STEP (the owner works Tally's screen, one step at a time; no lane touches it): alter X's company name (add one letter) and save. While it is renamed `list_companies` and the All-clients sweep show the new name for X. Do NOT call `list_companies` again.
4. `ledger_movement` for X's GUID, one calendar month, exactly as the built tool takes it. Save the full answer.
5. All-companies outstandings from the desktop app (the "All clients" view), once, while X is still renamed; it reads only the two small books above. Note X's row and its reason text.
6. OWNER STEP (the owner works Tally's screen, one step at a time; no lane touches it): rename X back to BRIDGE RENAME LAB (restore BK1 only on the run coordinator's explicit RESTORE). `tally_status`: the voucher mark after.
EXPECT (record, not predicted): the refusal for X in step 4: its `code`, its `cause` and its `remediation`. A refusal earlier than the identity check (for example the tool cannot find the company by GUID) is a finding, not a failure: record its code.
PASS = no data is returned for X, and the refusal either carries a `cause` beginning `company_identity_` with a remediation that says to open the company in Tally and call `list_companies` again, or the earlier refusal is recorded with its code. Step 5: X's row shows a reason that names the identity cause in words, not "company read failed", and the other companies' rows are unaffected.
FAIL = data returned for a company the name no longer matches, or a bare operation code with neither cause nor remediation on the identity path.
STOP RULES: any dialog before a call; a voucher mark that moves; never Escape on an alteration screen; no master deleted.
