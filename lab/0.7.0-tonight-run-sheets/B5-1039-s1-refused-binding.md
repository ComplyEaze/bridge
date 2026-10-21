# B5 (#1039 slice 1): capture a native post whose binding is refused
WHY: #1039 asks to let a person close a native post whose binding Bridge refused. No lab run has ever shown a refused binding; the first thing a design needs is the real answer. Posting ON (the owner types the values). Writes 3 vouchers, then a hand edit; restored after.
COMPANY: BRIDGE BINDING LAB (new). It holds a bank ledger, a party ledger under Sundry Debtors, the Receipt voucher type on Automatic numbering with 'retain existing numbers' on, no voucher on the chosen day D. The company is NEW: if a company or a data folder of this name already exists on the PC, STOP and report it; never reuse or overwrite one. Create it with books beginning 1 April 2025, then take backup BK1 into a NEW empty folder (name, size, sha256) before the first keying; BK1 is the state to restore. Licensed mode only.
STEPS (one request at a time):
1. `list_companies`; GUID. Marks read M0.
2. `build_import_xml`: ONE saved batch of 3 Receipts (1.00 each, distinct narrations) on day D. Record the batch id.
3. `post_import` the batch (the owner approves). Record the result. Do NOT call `verify_import` yet.
4. By hand, before any verify: alter ONE of the three vouchers in Tally (change its narration), save. Marks read M1.
5. `verify_import` for the batch. Save the full answer.
EXPECT (recorded): the verification result for the three vouchers, and whether the binding of the span is refused (`post_span_binding` refused, with its code) or the altered voucher reads as different.
PASS = the full answer is saved and the refusal code (or the absence of one) is recorded. This is a capture; a refusal is the interesting outcome, "all three verify" is also a finding.
STOP: any dialog before a call; step 3 not `posted` (report it whole; do not post again). Restore BK1 on RESTORE.
Evidence whole to the evidence folder the run coordinator names.
