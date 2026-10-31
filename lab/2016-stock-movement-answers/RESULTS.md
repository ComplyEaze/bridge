# #2016 stock movement: results (TallyPrime 7.2 Silver, BRIDGE SHAPE LAB, the published mcp-v0.7.0 build)

Sheet `lab/2016-stock-movement/RUN-SHEET.md` at eb428bfe (SHA256.txt 25 of 25; `gen_requests.py --check` ok, 18 requests). LAB-COMMON v3. THE COMPANY: BRIDGE SHAPE LAB, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, verified_tuple. Build: `bridge_mcp.exe` sha256 48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d, equal at the start and at the end of every launch. BK1 taken at step 2 and restored at step 15. Every voucher and master here was keyed on Tally's own screens; ComplyEaze Bridge only read. `notes.txt` has one line per step with UTC times and the marks before and after.

## Deviations, recorded

- **Launches.** The sheet asks for one launch and one data folder. This row had four launches on the same data folder. Known issue #2160 caused the extra ones: `masters kind voucher_types` refuses `masters_collection_changed` on the first call of a launch. On the recorded word the repeat is made as a second call in the same launch. Step 1's rows come from that second call (launch 3). At the close (line 181), launch 4's first call refused the same way and its second call answered.
- **Step 13.2, marks.** Creating the voucher type `Lab Manufacturing Journal` (type Stock Journal, "Use as a Manufacturing Journal" Yes, all else as offered) moved M from 298 to **300**; EXPECT was M + 1. The rows stopped there. On the recorded word, M + 2 is recorded as a measured result and the later EXPECTs were re-based on M 300 / V 127. **The second object is unidentified (partial):** after a stop, LAB-COMMON C8 admits only the state reads, so no masters read by ALTERID was sent. A side note: the new voucher type's GUID suffix is `...0000011d`, the next one after P (`...0000011b`) and Q (`...0000011c`). Every later mark matched the re-based EXPECT.
- **Ledgers named from Tally's lists.** Sundry Debtors: `Outstation Buyer 1`; sales ledger: `Sales - Exempt`; Sundry Creditors: `Chem Supplier 1` (each the same as the 28 Oct rows). The first purchase ledger Tally offered was **`ITC Test Ledger`** (group Purchase Accounts), and it was used. The first Indirect Expenses ledger offered was `Discount Allowed`. The Delivery Notes and the Receipt Note asked for a sales or purchase ledger, which the sheet does not name; the first offered was taken.
- **Expiry and batches.** On SJ2's acid lines the Expiry Date field did not pass empty, so `31-3-2027` was keyed (the sheet's test input, as earlier rows did). At Factory Floor the acid's only active batch was H-2504-B, so `H-2504-A` was entered through "New Number".
- Two keying slips were corrected before any Accept (SJ1's source godown, and the 08-07 Journal's bill amount). Both are recorded in the notes.

## R1: the candidate request on every kind, the totals, and a Delivery Note alone

- Step 5 vouchers kept their order. V after each was 113 to 123 (SJ1 113, SJ2 114, SJ3 115, S1 116, U1 117, CN1 118, DB1 119, S2 120, U2 121, U2's cancel 122, S3 123). **Tally took the one-sided SJ3.**
- **Delivery Note alone (step 6):** Q's Stock Summary line for June read 50 Kgs before DN1 and **45 Kgs** after it. A Delivery Note moves the Stock Summary.
- Every raw read answered 200, under its bound, with no dialog, and the marks were equal before and after each group. The span read `r-span-20260603-112-114` held exactly SJ1 and SJ2 (AlterIDs 113 and 114), not SJ3, as EXPECTED.
- `stock_summary`:
  - as_of 20260531: P 100, Q 50;
  - as_of 20260630: **P 108, Q 45** (quantities "agreed").
  - Both state value_total_matched, tie_out matched, item_count_cross_check 13/13.
- **Tally's screens, June:** P's closing was **108 Kgs**, so the post-dated S3 counted and shows. Factory Floor 55, Main Location 53. The optional S2 and the cancelled U2 are not listed. Q closed at 45. EXPECT met.
- **From the lines read** (excluding the optional and the cancelled voucher), P moved in 37 and out 29: net +8, and 100 + 8 = 108 matches step 9.
- **Class elements:** every VOUCHER carries the ten computed elements.
  - Stock Journal: reserved `Stock Journal`, all eight `BRIDGEVCHIS*` No.
  - Sales, Purchase, Credit Note and Debit Note: their own reserved names, each with its one flag Yes.
  - Delivery Note: reserved `Delivery Note`, all eight No.
  - The optional S2 answers as Sales voucher number 3 (ISOPTIONAL Yes). The cancelled U2 is Purchase 3 (ISCANCELLED Yes, no inventory lines). The post-dated S3 is Sales 2 (ISPOSTDATED Yes).
  - On the Stock Journals, `ALLINVENTORYENTRIES.LIST`, `INVENTORYENTRIESIN.LIST` and `INVENTORYENTRIESOUT.LIST` are all present. SJ3's IN list is present but empty.

## R2: nested-only items, and the item-less control

- **06-07, Purchase in mode "Accounting Invoice"** with ITC Test Ledger, 100.00: **Tally opened no inventory allocation**, so the voucher holds no item. In a07, two ledger entries and no INVENTORYALLOCATIONS. In a08, `ALLINVENTORYENTRIES.LIST` is present but empty, with no IN/OUT lists.
- **07-07, the control** (Discount Allowed 50.00, no item; Tally took a Purchase with no purchase ledger): the same shape as 06-07. The a08 answers of 06-07 and 07-07 hold the same 60 elements, none absent. The 13 that differ are only the voucher's own identity, number, date and amounts, and the second ledger's name (`raw/r12-step12-diff-0706-vs-0707.txt`). **Nothing in the a08 answer marks an item for these two, because neither holds one.**
- **08-07, Journal** with ITC Test Ledger: Tally **did** open an inventory allocation (P 2 Kgs at 10.00). In a07 the item sits **nested** (`ALLLEDGERENTRIES.LIST/INVENTORYALLOCATIONS.LIST`). In a08 it sits **top-level** as one `ALLINVENTORYENTRIES.LIST`: P, ISDEEMEDPOSITIVE Yes, 2 Kgs, -20.00, batch Main Location / MV-P-1, with no IN/OUT lists. Computed Journal, ISJOURNAL Yes.

## R3: the voucher types of stock vouchers

- Receipt Note was inactive and was activated (M + 2). Physical Stock was already active. Lab Manufacturing Journal created (M + 2; see above). **Tally asked for no bill of materials.**
- **EXPECT met for all four:**
  - Delivery Note (04-08), Receipt Note (05-08) and Physical Stock (06-08; P at Main Location / MV-P-1, 54 Kgs, the quantity Tally showed as current): `BRIDGEVCHRESERVEDNAME` is the type's own name, and all eight `BRIDGEVCHIS*` are No.
  - Lab Manufacturing Journal (07-08): `BRIDGEVCHRESERVEDNAME` empty, all eight No.

## Back to the start

- Step 14: the current date was set back to Monday 20-Apr-2026; the marks did not move.
- Step 15: BK1 restored, with only the two expected prompts.
- After the restore:
  - Gateway: period and current date equal step 0's, and the capture is byte-equal to step 0's.
  - `list_companies`: exactly THE COMPANY, verified_tuple.
  - Marks: 292/112.
  - `masters voucher_types`: the second call (#2160) answered byte-identical to step 1's (response sha256 fada60b7...), and all 26 rows are equal.
  - `stock_summary` as_of 20260630: 11 items, tie_out matched, no P or Q row.
- Step 16: launch ended (exit 0), company shut, Select Company with none loaded. One tally.exe owns 9001, and /status answers 200.

## Files

- `notes.txt`: one line per step.
- `answers/`: every tool answer, whole; `company_number` redacted.
- `raw/`: every raw answer, byte for byte, with its headers, a `.parsed.txt` and a `.step10.txt`, plus the step 10/12/13 summaries.
- `send-log.txt`, `restore-plan.txt`, `evidence/` (C2 checks, marks, exe hashes, the client log).
- `scripts/`: the lab helpers.
- `SCREENSHOTS.txt`: captures by sha256; they stay on the lab PC.
- `kept-local.txt`: BK1 and the folder listings by sha256.
- `substitutions.txt`: every pushed copy that differs from the original.
- `SHA256.txt`.
