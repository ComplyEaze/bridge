# 2057 item purchase on a voucher class that computes GST (#2057): the class, a plain-type control and a rounding invoice, read through the existing requests, on a book whose GST setup is whole

Not for merge. About 2.5 hours. This sheet WRITES by hand on Tally's screens (company GST details, one ledger, one stock item, one voucher class and three Purchase vouchers), reads, then restores the book from one backup. ComplyEaze Bridge only reads. `SHA256.txt` lists every file here with its bytes; check it before step 0.

`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one in `lab/0.7.0-rc-sitting`, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds. It runs alone and restores its own backup.

## What it decides

On 21 Oct 2026 (row B4, `lab/0.7.0-tonight-answers`), a Purchase item invoice keyed through the voucher class `Lab Class` carried **no tax amount** in three setups, none of which had a company GSTIN, a supplier GSTIN or an HSN. This sheet gives the book all three, keeps everything else as B4 had it, and keys:
- **I1:** the same invoice through the class;
- **C1:** a control on the plain Purchase type, with no class, selecting the two tax ledgers by hand;
- **I2:** a rounding invoice through the class.

**Records:**
- whether the class computes CGST and SGST once the GST setup is whole;
- where Tally keeps the purchase ledger on an item invoice of each kind: as a ledger entry, or only inside the goods line;
- the rounding Tally stores;
- whether `vouchers` and `purchase_register` equal Tally's screen.

**Limit, whatever the result:** one book, one release, one voucher per shape, intra-state only.

## Rules and company

**Who acts.** On the owner's standing word of 27 and 28 Oct 2026, the lab does every step here, including the keying, on this synthetic company, and records each as LAB-COMMON C11 directs. The final restore waits for the run coordinator's word RESTORE.

**THE COMPANY** is `BRIDGE CLASS PURCHASE LAB` (synthetic), GUID `92a7ef97-dd28-4286-b02c-c059fcb8690e`, as B4 left it on 21 Oct 2026, restored from its own BK1 (`TDBK1800_100074.001`, 24,100 bytes, sha256 `324bd37e5e2e5af6e55c3a45e1dad45e1e78c5e1a899da4e4114233d81dc9d49`):
- "No Vouchers Entered";
- master mark 213;
- its masters as imported: unit Nos, item `Lab Class Item`, ledgers `Lab Purchases`, `Lab Supplier` (Sundry Creditors), and `Lab CGST` and `Lab SGST` (Duties & Taxes, GST heads Central Tax and State Tax);
- no voucher class.

If it is absent, or step 1's marks differ: STOP and report (the run coordinator decides). Only THE COMPANY is loaded, alone, from step 0 to the last step. Licensed mode only; Education mode = STOP.

**Identifiers.** Only these, which are of the push gate's synthetic family (the PAN part `ZZZZZ0000Z`, with valid check characters), are keyed. Any other GSTIN or PAN = STOP before it is saved.
- the company's GSTIN `27ZZZZZ0000Z1ZQ`;
- the supplier's GSTIN `27ZZZZZ0000Z2ZP`;
- the HSN `340211`.

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Two kinds of read.**
   - **Tool reads** are calls of ComplyEaze Bridge's built tool, one at a time.
   - **Raw reads** by curl are allowed by the owner's decision of 26 Oct 2026 for shape discovery. They are only the three files in `requests/`, written by `gen_requests.py` from the committed vouchers request (`python -I gen_requests.py --check` must print its ok line before step 0), and sent by C3's line, each answer under 2,000,000 bytes.
   - An empty envelope, a 0-byte 200 answer or an in-band refusal is a **result**, recorded whole.
2. **Marks.** M (`ALTMSTID`) and V (`ALTVCHID`) are read with `ledger_masters` (basic, first page, no `snapshot_id`; on 21 Oct basic answered on this book): `snapshot.master_alter_id`, `snapshot.voucher_alter_id` (null was read after the 21 Oct restore; record it as read), `snapshot.reused` false.
   - Each master save raises M by at least 1 and leaves V unchanged. Record the exact rise: on 21 Oct a GST setup moved M by more than one.
   - Each voucher Accept raises V by exactly 1.
   - A V that moves on a master save, or by other than 1 on a voucher, = STOP.
3. **Backup and restore.** BK2 of THE COMPANY alone before the first change (C5), into a new empty folder `C:\bridge-lab\2057\backups\bk2\`. At the end, on the word RESTORE, BK2 is restored by C6. **Fallback:** the 21 Oct BK1 above, its hash checked first, only on the run coordinator's word.
4. **Keys.**
   - Accept with Ctrl+A. Leave a creation, alteration or voucher screen with ONE Ctrl+Q, only after a screenshot shows its title.
   - **Never Ctrl+Q or Escape at the Gateway of Tally**, never two Ctrl+Q in one call, never Escape on a master or voucher screen (C7).
   - A sub-form Tally opens by itself (Receipt Details, Party Details, a GST rate screen) is accepted as shown unless the step names a field, with a screenshot.
   - The one dialog answered is the one B4 met at Create > GST Registration: "GST Registration already exists for the Company. Do you want to alter the existing GST Registration or create a new GST Registration?" Answer **Alter Existing**. Any other dialog = C7.
5. **Never type a tax amount.** On a voucher, a tax line is left with the amount Tally puts there, or with none. An empty tax line is a result, not an error.
6. **One Tally.** At step 0 and at the end, as C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.

## Build

The published release `mcp-v0.7.0` Windows package, as the 30 Oct post-publish check staged it on the lab PC: `bridge_mcp.exe` sha256 `48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d`. Record its sha256 before the first call and after the last; another hash = STOP.

One launch over stdio:
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`;
- ONE new empty `BRIDGE_AGENT_DATA_DIR`;
- `BRIDGE_TERMS_ACCEPTED=true`;
- writes, import and redaction unset.

`vouchers`, `purchase_register`, `ledger_masters`, `masters` and `list_companies` must be in `tools/list`; one absent = STOP. This build has no `include_gst_details` on `vouchers` (that is on master, #2184). The raw read carries every GST element Tally stores on the entries.

## Files

- `gen_requests.py` and `templates/` (one committed request);
- `requests/` (three generated reads);
- `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Steps

### Step 0 and the book as it stands

0. LAB-COMMON C2 step 0 with THE COMPANY loaded alone. Record Tally's release, edition, current period and current date from the Gateway.
1. `tools/list` (count). Then:
   - `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, with the GUID above.
   - Marks: M0, V0. EXPECT M0 213.
   - `ledger_masters` `fields` `compliance`, `include_gst_details` true: record the rows of the five ledgers whole.
   - `masters` `kind` `voucher_types`: record the Purchase row whole.
2. **Backup BK2** (rule 3), then the marks: EXPECT M0 and V0.

### The GST setup: one change per save, marks after each

3. **The company** (K: Company > Alter, as B4 (b)): State `Maharashtra`. Accept. Then F11 Company Features: "Enable Goods and Services Tax (GST)" must already read Yes (record it; if No, STOP). Set "Set/Alter Company GST Rate and Other Details" Yes only if the GST registration screen cannot be reached otherwise, and accept its screen unchanged.
4. **The company's GST registration** (Create > GST Registration > the dialog of rule 4 > Alter Existing): Registration status Active; State Maharashtra; Registration type Regular; **GSTIN/UIN `27ZZZZZ0000Z1ZQ`**; everything else as shown. Screenshot before the Accept. If Tally refuses the GSTIN, copy its message exactly and STOP (the identifier is the gate's; nothing else is tried).
5. **The supplier** (Alter > Ledger > `Lab Supplier`): Country India; State Maharashtra; Registration type Regular; **GSTIN/UIN `27ZZZZZ0000Z2ZP`**; everything else as shown. Screenshot before the Accept.
6. **The item** (Alter > Stock Item > `Lab Class Item`):
   - GST applicability Applicable;
   - HSN/SAC Details "Specify Details Here", HSN/SAC `340211`, description as offered;
   - GST Rate Details "Specify Details Here", Taxability Taxable, IGST 18% (Tally fills CGST 9% and SGST/UTGST 9%; record what it shows);
   - Type of Supply Goods.
   
   Screenshot before the Accept.
7. **The purchase ledger** (Alter > Ledger > `Lab Purchases`): record, with a screenshot, every GST field the screen shows (GST applicability, Type of Supply, any rate details). Change nothing. Leave with ONE Ctrl+Q after the screenshot (no Accept: no save, no mark).
8. **The voucher class** (Alter > Voucher Type > `Purchase` > Name of Class: `Lab Class`), exactly as on 21 Oct:
   - default accounting allocation `Lab Purchases` 100%;
   - additional accounting entries `Lab CGST` and `Lab SGST`, Type of Calculation `GST` (set by Tally), Remove if Zero Yes;
   - every other field as shown.
   
   A screenshot of the class screen, whole, before its Accept and the voucher type's Accept.

After each of steps 3 to 6 and 8: the marks (M up, V unchanged; record the rise).

The book after step 8: the GST setup above and `Lab Class`. Nothing else.

### The three invoices

9. **I1, 10-06-2025, through the class:**
   - Gateway > Vouchers > F9 Purchase > Voucher Class List > `Lab Class`.
   - Item Invoice mode: if it opens in another, Ctrl+H to Item Invoice, and record the mode it opened in.
   - Supplier Invoice No. `LAB-2057-1`, dated 10-Jun-25; Party A/c `Lab Supplier`. The Party Details sub-form is accepted as shown: record its State, Registration type, GSTIN and Place of Supply.
   - Item `Lab Class Item`, 5 Nos @ 100.00 = 500.00.
   - Before the Accept, a screenshot showing every line and the total. Record each tax line's amount, or "no amount".
   - **EXPECT on screen:** Lab CGST 45.00 and Lab SGST 45.00, total 590.00. A screen without those amounts is the row's FINDING, recorded and accepted as it stands (rule 5).
   - Accept. Marks: V +1.
10. **C1, 11-06-2025, the control, with no class:**
    - Gateway > Vouchers > F9 Purchase > Voucher Class List > `Not Applicable`. Item Invoice mode.
    - Supplier Invoice No. `LAB-2057-2`, dated 11-Jun-25; Party `Lab Supplier`.
    - Item `Lab Class Item`, 5 Nos @ 100.00. The sales or purchase ledger, if Tally asks for one on the item line, `Lab Purchases`.
    - Below the item, select `Lab CGST` and then `Lab SGST` and leave their amounts as Tally fills them (rule 5).
    - Screenshot before the Accept; record each line.
    - **EXPECT on screen:** 45.00 and 45.00, total 590.00, if Tally computes GST on a plain type; otherwise the amounts it shows.
    - Accept. Marks: V +1.
11. **I2, 12-06-2025, rounding, through the class:** as I1, with Supplier Invoice No. `LAB-2057-3`, 7 Nos @ 13.57 = 94.99.
    - **EXPECT on screen:** each 9% head at 8.5491 before rounding, so Tally's shown CGST and SGST amounts and the total are the result. Record them exactly, and whether a round-off line appears.
    - Accept. Marks: V +1.

The book after step 11: step 8's, plus I1, C1 and I2.

### The reads

12. Marks before the group (M1, V1). Then, for each day D in 20250610, 20250611 and 20250612, in this order:
    - the raw `requests/r-a07-D.utf16le.xml` (C3's line), with a screenshot after it;
    - `vouchers` with `from` and `to` D and `goods_lines` true, saved whole.
13. `purchase_register` with `from` 20250601 and `to` 20250630, saved whole.
14. `ledger_masters` `fields` `compliance`, `include_gst_details` true: the five ledgers' rows whole. `masters` `kind` `voucher_types`: the Purchase row whole.
15. Marks after the group: equal to M1 and V1, else STOP.
16. **Tally's own screens:** each of the three vouchers displayed from the Day Book (10 to 12 Jun 2025), a screenshot of each, with every line, amount and the total transcribed. Then the Purchase Register for June 2025 (Display More Reports > Account Books > Purchase Register), screenshot, transcribed.

**Record, for each voucher, read by structure** (LAB-COMMON C4; ROWS are the `VOUCHER` children of the collection):
- **from the raw answer:**
  - each `ALLLEDGERENTRIES.LIST`: LEDGERNAME, AMOUNT, ISDEEMEDPOSITIVE, and every child list it carries, by name (`INVENTORYALLOCATIONS.LIST` with its STOCKITEMNAME, ACTUALQTY, RATE, AMOUNT and its own `ACCOUNTINGALLOCATIONS.LIST`; `RATEDETAILS.LIST` with GSTRATEDUTYHEAD, GSTRATEVALUATIONTYPE and GSTRATE; and the GST elements on the entry: GSTOVRDNTAXABILITY, GSTOVRDNTYPEOFSUPPLY, and any other element whose name begins `GST`);
  - any top-level `ALLINVENTORYENTRIES.LIST` (none is asked for; record presence only);
  - ISINVOICE and PARTYGSTIN;
- **from `vouchers`:** each entry's ledger and amount, and `goods_lines`;
- **from `purchase_register`:** the row's `status`, `tax_in_books` (ledger, head, raw_head, amount), `taxable_entries`, `has_taxable_entry`, `party_entries` and `other_entries`, or the side list it is in.

**EXPECT:** each amount in the raw answer, in `vouchers` and in `purchase_register` equals Tally's screen for the same voucher. A difference is the FINDING, reported whole before step 17.

**Answer, from what was recorded:**
- **(a)** Did the class compute tax (I1)?
- **(b)** Did the plain type compute it (C1)?
- **(c)** Where does `Lab Purchases` sit in each voucher: a ledger entry of its own, or only inside the goods line's accounting allocation? Hence `has_taxable_entry`.
- **(d)** What rounding is stored (I2)?

### Back to the start

17. **Restore** (C6) from BK2, on the run coordinator's explicit RESTORE. Then, in order:
    - a screenshot of the Gateway ("No Vouchers Entered");
    - `list_companies`: exactly THE COMPANY, `verified_tuple`, the same GUID;
    - marks equal to those after BK2 (M0, V0);
    - `ledger_masters` and `masters` as in step 1, equal.

    Anything else = STOP; the fallback is rule 3's.
18. Hash the exe. LAB-COMMON C10: Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; Tally's current date as step 0 recorded; one tally.exe; 9001 LISTENING; `GET /status` 200.

## STOP rules

LAB-COMMON's, with rules 1 to 6 above. Also:
- a raw answer that does not parse as XML;
- a voucher saved with a wrong item, quantity, rate, party or date (do not alter it: STOP and report what was saved);
- any identifier keyed other than the three of the Identifiers rule;
- any prompt not named here (C7).

## What the book holds after each step (C8)

| after | the book |
|---|---|
| step 2 | as at step 1 (BK2 taken) |
| steps 3 to 6 | plus each GST change, in turn |
| step 8 | and `Lab Class` |
| steps 9 to 11 | and I1, then C1, then I2 |
| step 17 | as at BK2, the book of step 1 |

## What the row records, and what it does not

**Records:**
- the class's tax on an item purchase with a whole intra-state GST setup;
- a plain-type control;
- one rounding case;
- what the existing vouchers request, `vouchers` and `purchase_register` return for each, against the screen.

**Does not:**
- inter-state (IGST);
- the class's settings in any read: no existing request fetches a voucher class, so the class is recorded from Tally's screen only;
- a voucher's class name (not in the vouchers request's fetch list);
- `vouchers` `include_gst_details` (not in this build);
- other releases, Gold, Education mode;
- several items, a discount, cess.

## Evidence

To `lab/2057-item-purchase-class-gst-answers`, after the pre-push scan and the run coordinator's yes:
- `RESULTS.md` (the four answers (a) to (d), and every EXPECT line with PASS or FINDING);
- `notes.txt` (UTC, one line per step, with each mark);
- `answers/` (every tool answer, whole);
- `raw/` (each raw answer with its headers, size, sha256, and a `.parsed.txt` of its structure);
- `send-log.txt`;
- `restore-plan.txt`;
- `SCREENSHOTS.txt` (the captures and BK2 by sha256; they stay on the lab PC).

No identifier beyond the three of the Identifiers rule, and no figure beyond the book's own synthetic ones, is recorded.
