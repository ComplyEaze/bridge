# C3 (#1805): does a listing page refused after a book change say to start again?
WHY: a held listing page that cannot be served now says to run the request again from the first page without `snapshot_id`. Simulator only so far.
COMPANY: BRIDGE LISTING LAB (new): loaded alone, five ledgers (names starting A to E), no voucher. If a company or data folder of this name exists on the PC, STOP and report it; never reuse one. Create it with books beginning 1 April 2025 and take backup BK1 into a NEW empty folder (name, size, sha256). If `list_companies` shows any other company, STOP before any call. The ledger rename is undone at the end; no other keying.
MODE: B.
STEPS (one request at a time):
1. `ledger_masters` with `limit=2`; save page 1 and its `snapshot_id`; take the master mark M1 (`tally_status`).
2. OWNER STEP (the owner works Tally's screen, one step at a time; no lane touches it): rename one ledger (add a letter) and save. `tally_status`: M2 (must differ from M1).
3. `ledger_masters` page 2 with that `snapshot_id`. Save the full answer.
4. OWNER STEP (the owner works Tally's screen, one step at a time; no lane touches it): rename the ledger back. `ledger_masters` again from the first page without `snapshot_id`: confirm a fresh first page reads.
EXPECT: a refusal with code `listing_snapshot_changed` and a remediation that says to run the request again from the first page without `snapshot_id`.
PASS = that code and that remediation, no page-2 rows returned, and the fresh first page in step 4 reads. FAIL = page-2 rows returned from the old snapshot, or a refusal with no remediation.
NOT RUN LIVE: the `voucher_window_changed_during_read` variant needs a book large enough that `vouchers` takes several parts and a save mid-read; it is not attempted here (a wide read of a large book is a wedge risk). Record "not run live".
STOP RULES: any dialog before a call; never Escape on an alteration screen.
