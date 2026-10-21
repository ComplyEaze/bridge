# B4 (#1423 capture 2): what does the committed read return for a Purchase item invoice keyed through a voucher class?
WHY: #1423 asks how an item purchase keyed with a voucher class (class-entry shape) reads through Bridge: which ledger and inventory entry lists come back, and whether `purchase_register` marks `has_taxable_entry`. Never run. READ-ONLY on Bridge's side; the voucher is keyed by hand.
COMPANY: BRIDGE CLASS PURCHASE LAB (new). It holds inventory on, one stock item, a purchase ledger, a supplier ledger under Sundry Creditors, GST ledgers, and a Purchase voucher class (create the class by hand and record how). The company is NEW: if a company or a data folder of this name already exists on the PC, STOP and report it; never reuse or overwrite one. Create it with books beginning 1 April 2025, then take backup BK1 into a NEW empty folder (name, size, sha256) before the first keying; BK1 is the state to restore. Licensed mode only.
STEPS (one request at a time):
1. `list_companies`; GUID. Marks read M0.
2. By hand: key ONE Purchase item invoice on day D using the voucher class (item, quantity, rate, GST). Record the voucher number and the screen's totals (screenshot).
3. `vouchers` for day D only, with the item/inventory detail the tool offers. Save the full answer.
4. `purchase_register` for the month containing D. Save the full answer.
5. Marks read M1.
EXPECT (recorded): in step 3 which ledger entries and inventory entries come back and with what amounts; in step 4 whether the voucher's row carries `has_taxable_entry` true or false.
PASS = both answers are saved and each amount equals the screen's. A mismatch (a missing entry, a wrong sign, `has_taxable_entry` false on a taxed invoice) is the finding and must be reported whole.
STOP: any dialog before a call; step 2 not keyed as stated (stop, do not improvise). Restore BK1 on RESTORE.
Evidence whole to the evidence folder the run coordinator names.
