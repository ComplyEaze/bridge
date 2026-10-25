# W4 (#1108, row 5): what do the company's marks do when a ledger is renamed?
WHY: row 5 of the issue's release-candidate list is the one row that needs no refused post: a ledger rename, with the company's marks read before and after. Rows 1 to 4 and the partly created batch are NOT on this sheet: they have no recipe (a voucher with its own number, or naming an absent ledger, is refused by Bridge before anything is sent, and a raw import that Tally refuses cannot go through `post_import`; maintainers' note of 24 October). Nothing is improvised here.
RULES: LAB-COMMON v3 C1 to C11 apply in full. Every read is sent by the built tool, one at a time; no hand-written request, so the request-ID preflight does not apply. Licensed or Educational mode may be recorded; a rename posts no voucher.
COMPANY: BRIDGE RENAMEMARKS LAB (new, THE COMPANY), loaded alone: three ledgers (BRIDGE Alpha, BRIDGE Beta, BRIDGE Gamma) under Sundry Debtors, no voucher. If a company or data folder of this name exists on the PC, STOP and report it. Create it with books beginning 1 April 2025 (OWNER STEP); take backup BK1 into a NEW empty folder (name, size, sha256) before the first change (C5).
MODE: B. Posting OFF.
STEPS:
1. `list_companies`: expect exactly BRIDGE RENAMEMARKS LAB, loaded alone; STOP before any call otherwise. GUID. `tally_status`: M0 (master mark), V0 (voucher mark). `ledger_masters`: save the three rows whole (name, GUID, MasterID, AlterID).
2. OWNER STEP: rename BRIDGE Beta to BRIDGE Beta2 (alter the ledger, change the name, accept; never Escape). `tally_status`: M1, V1. `ledger_masters`: save the rows whole.
3. OWNER STEP: rename BRIDGE Beta2 back to BRIDGE Beta. `tally_status`: M2, V2. `ledger_masters`: save the rows whole.
EXPECT (record, not predicted): the master mark moves on a rename (by how much); the voucher mark does not; the ledger keeps its GUID and MasterID and gains a new AlterID; the name changes and changes back.
PASS = the three states are recorded whole with the mark deltas (M1 - M0, M2 - M1, V1 - V0, V2 - V1). FAIL (a finding) = the voucher mark moving, or the ledger's GUID or MasterID changing. A rename that does not move the master mark is a finding for the marks reference: record it whole.
AFTER: OWNER STEP: restore BK1 on the run coordinator's explicit RESTORE; marks equal M0, V0. C10 close.
STOP RULES: C7 (never Escape on a master screen); a mark that moves before the rename.
