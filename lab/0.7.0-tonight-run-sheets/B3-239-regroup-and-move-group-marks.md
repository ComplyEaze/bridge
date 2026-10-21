# B3 (#239): do regrouping a ledger and moving a group move the master mark?
WHY: Bridge detects an outside change by the book's alteration marks. Earlier measurements showed master edits move ALTMSTID by 1; a ledger regroup (a ledger carrying vouchers and an opening balance) and a group move have not been measured. Extends the PARTIAL W-1 result. Read-only on Bridge's side; the edits are keyed by hand. No code depends on a PASS or a FAIL; both are findings.
COMPANY: BRIDGE REGROUP LAB (new). It holds a ledger L (with an opening balance and one voucher) under group G1, a second group G2, and a subgroup S under G1. The company is NEW: if a company or a data folder of this name already exists on the PC, STOP and report it; never reuse or overwrite one. Create it with books beginning 1 April 2025, then take backup BK1 into a NEW empty folder (name, size, sha256) before the first keying; BK1 is the state to restore. Licensed mode only.
STEPS (one request at a time; marks read = the marks the built `vouchers` read returns for one day (the voucher mark and the master mark; nothing is hand-sent), read with the egress `tally_status` line beside it):
1. Marks read M0 (ALTMSTID, ALTVCHID).
2. By hand: Alter Ledger L, change its group from G1 to G2, save. Marks read M1.
3. By hand: alter L back to G1, save. Marks read M2.
4. By hand: alter group S to sit under G2 instead of G1, save. Marks read M3.
5. By hand: alter S back under G1, save. Marks read M4.
6. `ledger_masters` basic once at the end; the catalogue must show L under G1 and S under G1.
EXPECT (recorded, not predicted): the change of ALTMSTID and ALTVCHID at each step.
PASS (for the evidence) = all four deltas recorded. The expected shape from earlier work is ALTMSTID +1 at each save and ALTVCHID unmoved; any ALTVCHID movement, or any step with no mark movement, is the finding.
STOP: any dialog before a call; never Escape on an alteration screen; no master deleted. Restore BK1 on the run coordinator's RESTORE.
Evidence (every marks answer, screenshots of the alter screens) whole to the evidence folder the run coordinator names.
