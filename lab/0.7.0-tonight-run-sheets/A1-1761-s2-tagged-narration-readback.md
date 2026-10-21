# A1 (#1761 slice 2): does a posted "don't know" bank line read back with its tag and its full narration?
WHY: the cash-line work (#1761) tags a line the person answered "don't know" with a narration tag, and later slices rely on reading those vouchers back by `vouchers suspense_tagged:true`. Nothing has ever shown that a 118-character tagged narration survives Tally and reads back byte-equal. The capture becomes the fixture for the later slices (P1).
COMPANY: BRIDGE TAG LAB (new). It holds a bank ledger (Lab Bank) and a ledger named Suspense under Suspense A/c, no voucher on the chosen day D. The company is NEW: if a company or a data folder of this name already exists on the PC, STOP and report it; never reuse or overwrite one. Create it with books beginning 1 April 2025, then take backup BK1 into a NEW empty folder (name, size, sha256) before the first keying; BK1 is the state to restore. Licensed mode only.
MODE: B (standing config entry) or A, as the lab runs it. Posting ON (owner types). Mark values below are read with the egress `tally_status` line.
STEPS (one request at a time):
1. `list_companies`; record the company GUID. `tally_status`; record ALTMSTID and ALTVCHID (M0, V0).
2. `vouchers` for the single day D chosen for the post (reads one day): record the number of vouchers on D (N0).
3. Build with `build_import_xml` ONE Payment from the bank ledger to Suspense for an amount of 1.00, narration = a tagged narration of exactly 118 characters as the proposals code builds it for a `dont_know` line (use the code's own text: run the bank-statement proposal path on a one-line synthetic statement answered "don't know"; if that path is not reachable from the tools, STOP and report "tag text cannot be produced from the tools", do not hand-write one). Record the narration text and its byte length.
4. `post_import` that one voucher; the owner approves in the native window. Record the full result (`posted_verified` expected).
5. `vouchers` with `suspense_tagged:true`, `ledger` = Suspense, from D to D. Record the full JSON.
6. `tally_status`; record M1 and V1.
EXPECT: step 5 returns exactly 1 row; its narration is byte-equal to step 3's; `matched.suspense_tag` = `purpose_not_confirmed`; V1 = V0 + 1.
PASS = all three of the first conditions hold. FAIL = 0 rows, 2+ rows, a narration that differs by even one byte (give both strings with their hex), or a different tag name. A FAIL is a finding for #1761, not an error of the run.
AFTER: restore BK1 on the coordinator's word (the run coordinator's explicit RESTORE) so the company returns to its earlier state; record the restore.
STOP RULES: any dialog before a call; any answer other than the expected shape in steps 1-2; step 4 not `posted_verified` (report it whole, do not post again).
The capture (steps 3-5 JSON) is delivered whole to the evidence folder the run coordinator names for use as a fixture; synthetic data only.
