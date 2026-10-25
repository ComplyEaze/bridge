# A2 (#1038 slice 2): after a restore, can a re-post of the same rows be told apart from the original post?
WHY: #1038 asks to let a person re-post the rows of a batch whose book was rolled back. The design is only safe if the re-post does not bind the old batch's GUIDs. On 12 Oct a hand re-key after a restore reused MasterID, AlterID, GUID and number; it is NOT measured when Bridge itself re-posts. This run supplies that fact. No re-post code exists; this only posts twice through the current tools, as a person could by posting the same rows again in a new batch.
COMPANY: BRIDGE REPOST LAB (new). It holds a bank ledger, two party ledgers under Sundry Debtors, and the Receipt voucher type set to Automatic numbering with 'retain existing numbers' on; no voucher on the chosen day D. The company is NEW: if a company or a data folder of this name already exists on the PC, STOP and report it; never reuse or overwrite one. Create it with books beginning 1 April 2025, then take backup BK1 into a NEW empty folder (name, size, sha256) before the first keying; BK1 is the state to restore. Licensed mode only.
MODE: B or A. Posting ON (owner types).
STEPS (one request at a time):
1. `list_companies`; GUID. `tally_status`: M0, V0. `vouchers` day D: N0 (expected 0).
2. `build_import_xml`: ONE saved batch B1 of 2 Receipts (each 1.00 from a synthetic party to the bank ledger, distinct narrations). Record batch id, the two REMOTEIDs/GUIDs the build reports.
3. `post_import` B1 (owner approves). Record result. `verify_import` B1: expect `posted_verified`. `tally_status`: M1, V1 (expect V1 = V0 + 2). `vouchers` day D: record both vouchers with MasterID, AlterID, GUID, number.
4. The operator restores BK1 by Tally's own Restore screen, identified as the standing restore rule says (company back to before step 3); this step is the run coordinator's RESTORE for BK1 at this point, and no other restore is made without that word. `tally_status`: M2, V2 (expect V2 = V0). `verify_import` B1: record the full result (expect rolled back / not attributable).
5. `build_import_xml`: a NEW batch B2 with the same two Receipts (new batch id; new REMOTEIDs). `post_import` B2 (owner approves). Record result.
6. `verify_import` B2 and `verify_import` B1 again. `vouchers` day D again: both vouchers with MasterID, AlterID, GUID, number. `tally_status`: M3, V3.
EXPECT (record, not predicted): V3 = V0 + 2.
PASS (for the design) = Tally holds exactly 2 Receipts on D after step 6; B2 reads `posted_verified`; B1 does NOT read `posted_verified` for the GUIDs now held by B2's vouchers (rolled_back or not_attributable). Record whether the MasterIDs/GUIDs in step 6 equal those in step 3.
FAIL (a finding, likely per the 12 Oct hand re-key) = B1 reads `posted_verified` or binds B2's GUIDs, or B2 does not verify. Report whole; this decides whether #1038 S2 can be built at all.
STOP RULES: any dialog before a call; step 3 not `posted_verified`; the restore (step 4) not confirmed by V2 = V0 (stop, report, do not post).
Evidence delivered whole to the evidence folder the run coordinator names. Restore BK1 again at the end on the run coordinator's RESTORE.
