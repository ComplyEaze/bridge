# C3 (#1805): does a listing page refused after a book change say to start again?
WHY: a held listing page that cannot be served now says to run the request again from the first page without `snapshot_id`. Simulator only so far.
COMPANY: BRIDGE SHAPE LAB or any BRIDGE book with at least 3 ledgers. The rename is undone at the end; no other keying.
MODE: B.
STEPS (one request at a time):
1. `ledger_masters` with `limit=2`; save page 1 and its `snapshot_id`; take the master mark M1 (`tally_status`).
2. The coordinator renames one ledger (add a letter) and saves. `tally_status`: M2 (must differ from M1).
3. `ledger_masters` page 2 with that `snapshot_id`. Save the full answer.
4. The coordinator renames the ledger back. `ledger_masters` again from the first page without `snapshot_id`: confirm a fresh first page reads.
EXPECT: a refusal with code `listing_snapshot_changed` and a remediation that says to run the request again from the first page without `snapshot_id`.
PASS = that code and that remediation, no page-2 rows returned, and the fresh first page in step 4 reads. FAIL = page-2 rows returned from the old snapshot, or a refusal with no remediation.
OPTIONAL, best effort, not a gate: on a book large enough that `vouchers` takes several parts for a month window, the coordinator saves a voucher mid-read; the answer is `voucher_window_changed_during_read` with a remediation to run the same request again once. If the timing cannot be hit, record "not run".
STOP RULES: any dialog before a call; never Escape on an alteration screen.
