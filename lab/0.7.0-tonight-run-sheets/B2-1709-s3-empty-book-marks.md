# B2 (#1709 slice 3): what voucher mark does a company with no voucher read?
WHY: Bridge needs a "before" voucher mark to attribute a first post in an empty book. Whether an empty book reads ALTVCHID as absent, 0 or empty decides if a first post can use 0 as its before mark (a stable 0) or must be refused. Never measured. READ-ONLY.
COMPANY: BRIDGE EMPTY LAB (new). It holds no ledger beyond Tally's own two and NO voucher at all. The company is NEW: if a company or a data folder of this name already exists on the PC, STOP and report it; never reuse or overwrite one. Create it with books beginning 1 April 2025, then take backup BK1 into a NEW empty folder (name, size, sha256) before the first keying; BK1 is the state to restore. Licensed mode only.
STEPS (one request at a time):
1. `list_companies`; GUID.
2. Read the marks five times in a row (the marks the built `vouchers` read returns for one day (the voucher mark and the master mark; nothing is hand-sent), read with the egress `tally_status` line beside it), with a `tally_status` between each. Save each answer whole.
3. Record, for each answer, the ALTVCHID element: absent, present with 0, or present and empty; and ALTMSTID.
NOTE: the built tool may refuse on an empty book, or report a mark it derived. Record its code and wording whole: that is what Bridge itself sees, and it is the answer this row needs. The raw ALTVCHID element is read only if the run coordinator supplies a reviewed request file; nothing is hand-sent.
EXPECT (recorded): the same shape five times. PASS = a definite shape, the same in all five. FAIL = the shape changes between reads (then a first post cannot rely on it).
STOP: any dialog before a call; any answer other than the marks shape.
Evidence whole to the evidence folder the run coordinator names.
