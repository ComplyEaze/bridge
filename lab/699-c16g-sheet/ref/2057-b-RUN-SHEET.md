# 2057-B item purchase on a voucher class, GST, after the central-tax ledger is completed (#2057): the 31 Oct invoices keyed again, one change at a time

Not for merge. About 2.5 hours. This sheet WRITES by hand on Tally's screens: company GST details, two ledgers (a third only if step 19 b runs), one stock item, one voucher class and four Purchase vouchers (a fifth only if step 20 runs). It then reads, and restores the book from one backup. ComplyEaze Bridge only reads. `SHA256.txt` lists every file here with its bytes; check it before step 0.

`LAB-COMMON.md` in this folder applies in full. It is v3, byte-equal to the one in `lab/2057-item-purchase-class-gst` (sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6). Where it and this sheet differ, the stricter holds. The row runs alone and restores its own backup.

## What it decides

The row of 31 Oct 2026 is `lab/2057-item-purchase-class-gst` at a37544fc, answered in `lab/2057-item-purchase-class-gst-answers` at a8bcd9f8. It keyed the whole GST setup on this book, then three invoices:
- **I1, through the class:** no tax amount on either head, total 500.00.
- **C1, the plain type:** Lab SGST 45.00 and no Lab CGST amount, total 545.00.
- **I2, through the class:** Lab SGST 8.55 and no Lab CGST amount, total 103.54.

Every read equalled Tally's screen. Its recorded causes:
- **(b) and (d), the State head alone:** likely, at partial confidence. `ledger_masters` reads Lab CGST with `gst_duty_head` `absent` while Lab SGST reads State Tax, and did so before any change.
- **(a), no tax through the class on I1:** unknown, unverified. I2 went through the same class and the same purchase ledger and did carry the State head. I1 opened in Accounting Invoice mode and was switched to Item Invoice before any line was keyed; I2 opened in Item Invoice.

This row keys the same setup again (steps 3 to 8, word for word) and makes ONE more change before any invoice: Lab CGST's duty head (step 9). Then it keys I1, C1 and I2 again, in the same order and with the same lines, and adds one invoice, I3, with I1's lines, keyed after I2.

**Records:**
- whether C1 carries both heads once Lab CGST has its head (tests the cause of (b) and (d));
- whether the class computes both heads (I1-B, I2-B, I3);
- whether I1-B and I3 differ, and the mode each opened in;
- the rounding Tally stores on each head (I2-B);
- whether `vouchers` and `purchase_register` equal Tally's screen, as on 31 Oct.

**Limit, whatever the result:** one book, one release, one voucher per shape, intra-state only.

**A second change, only if the class still computes nothing (B2, step 19).** On 31 Oct, `Lab Purchases`' GST Rate Details read "As per Company/Group" at 0 %. Tally's list for that field was never opened, so its values are not known. C1 and I2 computed the State head through that same ledger.
- **When it runs:** only when I1-B and I3 both carry no tax amount, and only after the reads of steps 14 to 18.
- **What it does:** it opens that list and transcribes it. It changes the ledger's rate details only if one entry, by its own label, puts the details on this ledger.
- **Then:** I1-C, I1's lines again, is keyed and read. Otherwise the row records the list and goes to the restore.

## Rules and company

**Who acts.** On the owner's standing word of 27 and 28 Oct 2026, the lab does every step here, including the keying, on this synthetic company, and records each as LAB-COMMON C11 directs. The final restore waits for the run coordinator's word RESTORE.

**THE COMPANY** is `BRIDGE CLASS PURCHASE LAB` (synthetic), GUID `92a7ef97-dd28-4286-b02c-c059fcb8690e`, as the 31 Oct row left it after its restore:
- "No Vouchers Entered";
- the Gateway's Current Date 1-Apr-2025;
- marks 213 / null;
- its masters as imported: unit Nos, item `Lab Class Item`, ledgers `Lab Purchases`, `Lab Supplier` (Sundry Creditors), and `Lab CGST` and `Lab SGST` (Duties & Taxes);
- no voucher class.

If it is absent, or step 1's answers differ from 31 Oct's as step 1 sets out: STOP and report (the run coordinator decides). Only THE COMPANY is loaded, alone, from step 0 to the last step. Licensed mode only; Education mode = STOP.

**Identifiers.** Only these, which are of the push gate's synthetic family (the PAN part `ZZZZZ0000Z`, with valid check characters), are keyed. Any other GSTIN or PAN = STOP before it is saved.
- the company's GSTIN `27ZZZZZ0000Z1ZQ`;
- the supplier's GSTIN `27ZZZZZ0000Z2ZP`;
- the HSN `340211`.

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Two kinds of read.**
   - **Tool reads** are calls of ComplyEaze Bridge's built tool, one at a time.
   - **Raw reads** by curl are allowed by the owner's decision of 26 Oct 2026 for shape discovery. They are only the five files in `requests/`, written by `gen_requests.py` from the committed vouchers request (`python -I gen_requests.py --check` must print its ok line, with 5 requests, before step 0). The 20250614 one is sent only if step 20 keys I1-C, and sent by C3's line, each answer under 2,000,000 bytes. The first three are byte-equal to the 31 Oct sheet's.
   - An empty envelope, a 0-byte 200 answer or an in-band refusal is a **result**, recorded whole.
2. **Marks.** M (`ALTMSTID`) and V (`ALTVCHID`) are read with `ledger_masters` (basic, first page, no `snapshot_id`): `snapshot.master_alter_id`, `snapshot.voucher_alter_id` (null was read after the 31 Oct restore; record it as read), `snapshot.reused` false.
   - Each master save raises M by at least 1 and leaves V unchanged. Record the exact rise. On 31 Oct steps 3 and 8 each moved M by 2.
   - Each voucher Accept raises V by exactly 1.
   - A V that moves on a master save, or by other than 1 on a voucher, = STOP.
3. **Backup and restore.** BK3 of THE COMPANY alone before the first change (C5), into a new empty folder `C:\bridge-lab\2057-b\backups\bk3\`. At the end, on the word RESTORE, BK3 is restored by C6.
   - **Fallback, only on the run coordinator's word:** the 31 Oct BK2 (24,100 bytes, sha256 `8ca808192ec97a4015b73aad7fd83b64d8d60e16e917c3c9b570454cae0d1405`), its hash checked first.
   - **Then:** the 21 Oct BK1 (24,100 bytes, sha256 `324bd37e5e2e5af6e55c3a45e1dad45e1e78c5e1a899da4e4114233d81dc9d49`).
4. **Keys.**
   - Accept with Ctrl+A. Leave a creation, alteration or voucher screen with ONE Ctrl+Q, only after a screenshot shows its title.
   - **Never Ctrl+Q or Escape at the Gateway of Tally**, never two Ctrl+Q in one call, never Escape on a master or voucher screen (C7). The one exception is step 19 a: ONE Escape closes the GST Rate Details list without choosing, on the run coordinator's ruling.
   - A sub-form Tally opens by itself (Receipt Details, Party Details, a GST rate screen) is accepted as shown unless the step names a field, with a screenshot.
   - The one dialog answered is the one the 31 Oct row met at Create > GST Registration: "GST Registration already exists for the Company. Do you want to alter the existing GST Registration or create a new GST Registration?" Answer **Alter Existing**. Any other dialog = C7.
   - Reach a master by the label read on screen before Enter, not by a count of Up or Down presses (the 31 Oct step 7 slip).
5. **Never type a tax amount.** On a voucher, a tax line is left with the amount Tally puts there, or with none. An empty tax line is a result, not an error.
6. **One Tally.** At step 0 and at the end, as C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.

## Build

The Windows bundle of the merge-queue run of #2327. That is the commit now on master; the master push run skipped the bundle.
- CI run 41745127138, `merge_group`, head ae9a1bb5 (`ae9a1bb5096010a28fde3eac19aac4457f49eb69`), success.
- Artifact `unsigned-bundle-smoke-windows-latest`, id 13283891845, 59,284,798 bytes, digest `sha256:922968055bb143f1a802beffbb496bbd94795a539382f9b5ae013c4985a45685`.

Check the zip's sha256 against that digest first; another digest is a stop. Record `bridge_mcp.exe`'s sha256 against the bundle's smoke file, before the first call and after the last; a change = STOP.

One launch over stdio:
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`;
- ONE new empty `BRIDGE_AGENT_DATA_DIR`;
- `BRIDGE_TERMS_ACCEPTED=true`;
- `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` all absent.

Client: `scripts/client.py` of `lab/2057-item-purchase-class-gst-answers` at a8bcd9f8 (sha256 `c127118d327a50f7836118f440a0da9855215ea0b55c50e5f56fd1da91b8a600`), which refuses the write settings. Record its sha256.

`vouchers`, `purchase_register`, `ledger_masters`, `masters` and `list_companies` must be in `tools/list`; one absent = STOP. Every tool call below is the 31 Oct call with the same arguments. This build has `include_gst_details` on `vouchers`; this row does not pass it, so the reads stay comparable with 31 Oct's. An answer may carry fields the 0.7.0 build did not; amounts and entries are compared, not whole answers.

## Files

- `gen_requests.py` and `templates/` (one committed request);
- `requests/` (five generated reads);
- `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Steps

### Step 0 and the book as it stands

0. LAB-COMMON C2 step 0 with THE COMPANY loaded alone. Record Tally's release, edition, current period and current date from the Gateway. EXPECT Current Date 1-Apr-2025 and "No Vouchers Entered".
1. `tools/list` (count). Then:
   - `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, with the GUID above.
   - Marks: M0, V0. EXPECT 213 / null.
   - `ledger_masters` `fields` `compliance`, `include_gst_details` true: record the rows of the five ledgers whole. EXPECT them equal to the 31 Oct answer `answers/003-s1-ledgers-gst.json` in every field but the snapshot's id and read time, Lab CGST's `gst_duty_head` `absent` included.
   - `masters` `kind` `voucher_types`: record the Purchase row whole. EXPECT it equal to `answers/004-s1-vchtypes-call1.json`'s, apart from the snapshot.
2. **Backup BK3** (rule 3), then the marks: EXPECT M0 and V0.

### The GST setup of 31 Oct: one change per save, marks after each

Steps 3 to 8 are the 31 Oct sheet's steps 3 to 8, word for word. The 31 Oct marks after each are given as EXPECT; a different rise is recorded, not a stop, while V is unchanged.

3. **The company** (K: Company > Alter, as B4 (b)): State `Maharashtra`. Accept. Then F11 Company Features: "Enable Goods and Services Tax (GST)" must already read Yes (record it; if No, STOP). Set "Set/Alter Company GST Rate and Other Details" Yes only if the GST registration screen cannot be reached otherwise, and accept its screen unchanged. EXPECT M 215.
4. **The company's GST registration** (Create > GST Registration > the dialog of rule 4 > Alter Existing): Registration status Active; State Maharashtra; Registration type Regular; **GSTIN/UIN `27ZZZZZ0000Z1ZQ`**; everything else as shown. Screenshot before the Accept. If Tally refuses the GSTIN, copy its message exactly and STOP (the identifier is the gate's; nothing else is tried). EXPECT M 216.
5. **The supplier** (Alter > Ledger > `Lab Supplier`): Country India; State Maharashtra; Registration type Regular; **GSTIN/UIN `27ZZZZZ0000Z2ZP`**; everything else as shown. Screenshot before the Accept. EXPECT M 217.
6. **The item** (Alter > Stock Item > `Lab Class Item`):
   - GST applicability Applicable;
   - HSN/SAC Details "Specify Details Here", HSN/SAC `340211`, description as offered;
   - GST Rate Details "Specify Details Here", Taxability Taxable, IGST 18% (Tally fills CGST 9% and SGST/UTGST 9%; record what it shows);
   - Type of Supply Goods.

   Screenshot before the Accept. EXPECT M 218. (On 31 Oct the screen showed one "GST Rate" field, keyed 18 %.)
7. **The purchase ledger** (Alter > Ledger > `Lab Purchases`): record, with a screenshot, every GST field the screen shows (GST applicability, Type of Supply, any rate details). Change nothing. Leave with ONE Ctrl+Q after the screenshot (no Accept: no save, no mark). EXPECT M 218.
   - **Added to the 31 Oct step, one look with no save:** on 31 Oct a "1 ▾" marker showed one more line below the left panel, and a mouse-wheel scroll did not move it. Before the Ctrl+Q, try to bring that line into view:
     - Use the Down arrow only, one press at a time, with a screenshot after each. Never press Enter, which can reach the Accept prompt, and never type into a field.
     - Transcribe the line's label and value once it shows.
     - If the cursor reaches the last field and the line has not shown, record that and stop trying.
     - Then ONE Ctrl+Q. If Tally asks anything on leaving, that is C7: no key, STOP. Marks: 218, unchanged.
8. **The voucher class** (Alter > Voucher Type > `Purchase` > Name of Class: `Lab Class`), exactly as on 21 Oct:
   - default accounting allocation `Lab Purchases` 100%;
   - additional accounting entries `Lab CGST` and `Lab SGST`, Type of Calculation `GST` (set by Tally), Remove if Zero Yes;
   - every other field as shown.

   A screenshot of the class screen, whole, before its Accept and the voucher type's Accept. EXPECT M 220.

After each of steps 3 to 6 and 8: the marks (M up, V unchanged; record the rise).

### The one new change

9. **B1: Lab CGST's duty head.**
   - **a. Look at Lab SGST** (Alter > Ledger > `Lab SGST`): change nothing. Screenshot the whole screen. Record, in the screen's own words, every field under its statutory or GST details: each label and its value, including the one whose value is State Tax. Leave with ONE Ctrl+Q after the screenshot (no Accept). Marks: unchanged.
   - **b. Open Lab CGST** (Alter > Ledger > `Lab CGST`). Screenshot the whole screen as it opens. Record the same labels and values as in a.
   - **c. Set only the field that on Lab SGST holds State Tax.** Open that field's list and take a screenshot of the list whole; record every entry in it. Choose the entry for the central head, the counterpart of State Tax in that list. Record its exact words.
     - STOP with no key pressed, and report, if: the field is not on the Lab CGST screen; the list offers no central-head entry; or more than one entry could be it.
   - **d. Accept.** Every other field stays as shown. A sub-form Tally opens by itself is accepted as shown (rule 4), with a screenshot. Screenshot before the Accept. Marks: M up, V unchanged. Record the rise.
   - **e. Read it.** `ledger_masters` `fields` `compliance`, `include_gst_details` true: the five ledgers' rows whole.
     - EXPECT: Lab CGST `gst_duty_head` `recognized`, head `cgst`, raw `CGST`. Lab SGST unchanged, `state_tax`. The other three equal step 1's apart from what steps 3 to 8 changed.
     - Lab CGST read otherwise (`absent`, `unrecognized`, `contradictory`) = STOP and report, with the raw value. Nothing more is keyed.

The book after step 9: the 31 Oct setup with `Lab Class`, and Lab CGST with its head. Nothing else.

### The invoices

Steps 10 to 12 are the 31 Oct sheet's steps 9 to 11, word for word, with this row's EXPECT. Step 13 is new.

10. **I1-B, 10-06-2025, through the class:**
    - Gateway > Vouchers > F9 Purchase > Voucher Class List > `Lab Class`.
    - Item Invoice mode: if it opens in another, Ctrl+H to Item Invoice, and record the mode it opened in.
    - Supplier Invoice No. `LAB-2057-1`, dated 10-Jun-25; Party A/c `Lab Supplier`. The Party Details sub-form is accepted as shown: record its State, Registration type, GSTIN and Place of Supply.
    - Item `Lab Class Item`, 5 Nos @ 100.00 = 500.00.
    - Before the Accept, a screenshot showing every line and the total. Record each tax line's amount, or "no amount".
    - **EXPECT on screen:** Lab CGST 45.00 and Lab SGST 45.00, total 590.00. A screen without those amounts is the row's FINDING, recorded and accepted as it stands (rule 5).
    - Accept. Marks: V +1.
11. **C1-B, 11-06-2025, the control, with no class:**
    - Gateway > Vouchers > F9 Purchase > Voucher Class List > `Not Applicable`. Item Invoice mode.
    - Supplier Invoice No. `LAB-2057-2`, dated 11-Jun-25; Party `Lab Supplier`.
    - Item `Lab Class Item`, 5 Nos @ 100.00. The sales or purchase ledger, if Tally asks for one on the item line, `Lab Purchases`.
    - Below the item, select `Lab CGST` and then `Lab SGST` and leave their amounts as Tally fills them (rule 5).
    - Screenshot before the Accept; record each line.
    - **EXPECT on screen:** 45.00 and 45.00, total 590.00. On 31 Oct, Lab CGST had no amount and the total was 545.00.
    - Accept. Marks: V +1.
12. **I2-B, 12-06-2025, rounding, through the class:** as I1-B, with Supplier Invoice No. `LAB-2057-3`, 7 Nos @ 13.57 = 94.99.
    - **EXPECT on screen:** each 9% head at 8.5491 before rounding, so Tally's shown CGST and SGST amounts and the total are the result. Record them exactly, and whether a round-off line appears. (On 31 Oct: Lab SGST 8.55, no Lab CGST amount, no round-off line, total 103.54.)
    - Accept. Marks: V +1.
13. **I3, 13-06-2025, through the class, I1-B's lines again:** as I1-B, with Supplier Invoice No. `LAB-2057-4`, dated 13-Jun-25, 5 Nos @ 100.00 = 500.00.
    - Record the mode it opened in before any key. If it is not Item Invoice, Ctrl+H to Item Invoice as on I1-B.
    - **EXPECT on screen:** Lab CGST 45.00 and Lab SGST 45.00, total 590.00. Whatever is shown is recorded and accepted as it stands (rule 5).
    - Accept. Marks: V +1.

The book after step 13: step 9's, plus I1-B, C1-B, I2-B and I3.

### The reads

14. Marks before the group (M1, V1). Then, for each day D in 20250610, 20250611, 20250612 and 20250613, in this order:
    - the raw `requests/r-a07-D.utf16le.xml` (C3's line), with a screenshot after it;
    - `vouchers` with `from` and `to` D and `goods_lines` true, saved whole.
15. `purchase_register` with `from` 20250601 and `to` 20250630, saved whole.
16. `ledger_masters` `fields` `compliance`, `include_gst_details` true: the five ledgers' rows whole. `masters` `kind` `voucher_types`: the Purchase row whole.
17. Marks after the group: equal to M1 and V1, else STOP.
18. **Tally's own screens:**
    - each of the four vouchers displayed from the Day Book (10 to 13 Jun 2025), a screenshot of each, with every line, amount and the total transcribed;
    - then the Purchase Register for June 2025 (Display More Reports > Account Books > Purchase Register), screenshot, transcribed.

    Marks after: equal to M1 and V1.

**Record, for each voucher, read by structure** (LAB-COMMON C4; ROWS are the `VOUCHER` children of the collection), as on 31 Oct:
- **from the raw answer:**
  - each `ALLLEDGERENTRIES.LIST`: LEDGERNAME, AMOUNT, ISDEEMEDPOSITIVE, and every child list it carries, by name (`INVENTORYALLOCATIONS.LIST` with its STOCKITEMNAME, ACTUALQTY, RATE, AMOUNT and its own `ACCOUNTINGALLOCATIONS.LIST`; `RATEDETAILS.LIST` with GSTRATEDUTYHEAD, GSTRATEVALUATIONTYPE and GSTRATE; and the GST elements on the entry: GSTOVRDNTAXABILITY, GSTOVRDNTYPEOFSUPPLY, and any other element whose name begins `GST`);
  - any top-level `ALLINVENTORYENTRIES.LIST` (none is asked for; record presence only);
  - ISINVOICE and PARTYGSTIN;
- **from `vouchers`:** each entry's ledger and amount, and `goods_lines`;
- **from `purchase_register`:** the row's `status`, `tax_in_books` (ledger, head, raw_head, amount), `taxable_entries`, `has_taxable_entry`, `party_entries` and `other_entries`, or the side list it is in.

**EXPECT:** each amount in the raw answer, in `vouchers` and in `purchase_register` equals Tally's screen for the same voucher. A difference is the FINDING, reported whole before anything more is keyed.

**Answer, from what was recorded:**
- **(a)** Did C1-B carry both heads? If so, at what amounts?
- **(b)** Did the class compute tax on I1-B, I2-B and I3? Which heads, and what amounts?
- **(c)** Did I1-B and I3 differ? Give the mode each opened in.
- **(d)** What rounding is stored on each head of I2-B, and is there a round-off line?
- **(e)** Where does `Lab Purchases` sit in each voucher: a ledger entry of its own, or only inside the goods line's accounting allocation? Hence `has_taxable_entry`. In which `purchase_register` list is each voucher?

### B2: only if I1-B and I3 both carry no tax amount

If either I1-B or I3 carries a tax amount on either head, skip steps 19 to 22 and go to step 23. Record "B2 not run" and the reason.

19. **B2: the purchase ledger's GST Rate Details** (Alter > Ledger > `Lab Purchases`). This is the run coordinator's ruling, so that no value is guessed.
    - **a. Read the list.** Screenshot the screen as it opens. Open the list of the field labelled "GST Rate Details" (on 31 Oct it read "As per Company/Group"). Screenshot the list whole and transcribe every entry in it. Then ONE Escape closes the list without choosing (rule 4's one exception). Screenshot.
    - **b. If one entry, by its own label, puts the details on this ledger** (for example "Specify Details Here"):
      - Choose it.
      - Set Taxability Type to `Taxable` and GST Rate to `18` (%), the item's rate keyed at step 6.
      - Leave HSN/SAC Details and every other field as shown: that would be a second variable. A sub-form Tally opens by itself is accepted as shown, apart from those two fields (rule 4), with a screenshot.
      - Record each field's name and value from the screen. Screenshot before the Accept, then Accept.
      - Marks: M up, V unchanged; record the rise.
    - **c. If no entry does that, or more than one could:** leave with ONE Ctrl+Q, with no save. Record the list. Skip steps 20 to 22 and go to step 23. If Tally asks anything on leaving, that is C7: no key, STOP.
    - **d. Read it** (after b only): `ledger_masters` `fields` `compliance`, `include_gst_details` true: the five ledgers' rows whole. Record `Lab Purchases`' `compliance.gst_details` as read; it is a result, not an EXPECT. A read that shows any change on another ledger = STOP.
20. **I1-C, 14-06-2025, through the class, I1-B's lines again:** as I1-B, with Supplier Invoice No. `LAB-2057-5`, dated 14-Jun-25, 5 Nos @ 100.00 = 500.00.
    - Record the mode it opened in before any key. If it is not Item Invoice, Ctrl+H to Item Invoice as on I1-B.
    - **EXPECT on screen:** Lab CGST 45.00 and Lab SGST 45.00, total 590.00. Whatever is shown is recorded and accepted as it stands (rule 5).
    - Accept. Marks: V +1.
21. **The reads of I1-C.** Marks before (M2, V2). Then, in this order:
    - the raw `requests/r-a07-20250614.utf16le.xml` (C3's line), with a screenshot after it;
    - `vouchers` with `from` and `to` 20250614 and `goods_lines` true;
    - `purchase_register` 20250601 to 20250630;
    - `ledger_masters` and `masters` as at step 16.

    Marks after: equal to M2 and V2, else STOP. Record I1-C by structure as above.
22. **Tally's screens:** I1-C from the Day Book (14 Jun 2025), and the Purchase Register for June 2025, each screenshotted and transcribed. The EXPECT of the reads holds for I1-C.

- **(f)** Did B2's list offer an entry that puts the details on the ledger? If B2 ran, did I1-C carry tax, on which heads, at what amounts? What did `ledger_masters` read on `Lab Purchases` after it?

### Back to the start

23. **Restore** (C6) from BK3, on the run coordinator's explicit RESTORE. Then, in order:
    - a screenshot of the Gateway (Current Date 1-Apr-2025, "No Vouchers Entered"), compared byte for byte with step 0's;
    - `list_companies`: exactly THE COMPANY, `verified_tuple`, the same GUID;
    - marks equal to those after BK3 (M0, V0);
    - `ledger_masters` and `masters` as in step 1, equal.

    Anything else = STOP; the fallback is rule 3's.
24. Hash the exe. LAB-COMMON C10: Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; Tally's current date as step 0 recorded; one tally.exe; 9001 LISTENING; `GET /status` 200.

## STOP rules

LAB-COMMON's, with rules 1 to 6 above. Also:
- a raw answer that does not parse as XML;
- a voucher saved with a wrong item, quantity, rate, party or date (do not alter it: STOP and report what was saved);
- any identifier keyed other than the three of the Identifiers rule;
- any change to a master other than those of steps 3 to 6, 8, 9 and (only as step 19 b sets out) 19;
- step 9's STOP cases;
- a key at step 19 other than those it names;
- any prompt not named here (C7).

## What the book holds after each step (C8)

| after | the book |
|---|---|
| step 2 | as at step 1 (BK3 taken) |
| steps 3 to 6 | plus each GST change, in turn |
| step 8 | and `Lab Class` |
| step 9 | and Lab CGST's head |
| steps 10 to 13 | and I1-B, then C1-B, then I2-B, then I3 |
| step 19 (b only) | and `Lab Purchases`' GST Rate Details on the ledger, Taxable 18 % |
| step 20 (if run) | and I1-C |
| step 23 | as at BK3, the book of step 1 |

## What the row decides

- **C1-B carries 45.00 on each head, and every read equals the screen:** the State head alone on 31 Oct was the book's incomplete CGST ledger. A plain-type item purchase with both heads is recorded once in the protocol reference, verified on one voucher.
- **I1-B, I2-B or I3 carries both heads, and the reads equal the screen:** the class computes once the ledgers are whole. That is #2057's acceptance: a capture and a protocol reference entry with its confidence marker, and no code. The answer of that voucher is committed as a fixture by the gate's manifest route.
- **I1-B and I3 differ:** the opening mode, or the order of keying, is recorded as the only difference seen, unverified as the cause.
- **I1-B and I3 carry no tax, and I1-C after B2 carries both heads:** the purchase ledger's rate details are what held the class at zero. Recorded at the confidence one voucher gives, and the same acceptance as above.
- **No class invoice carries tax, I1-C included or B2 not possible:** recorded, with B2's list. #2057 stays open. Any next change waits for a ruling and a sheet of its own.
- **A read differs from the screen:** stop and report. Any code waits for a ruling.

## What master admits, at ae9a1bb5

- `purchase_register`'s side list for a Purchase with no Duties & Taxes entry: `src-tauri/src/agent_register.rs:66`. Its description, which says what is not measured: `:104`.
- The duty-head spelling the parser recognises as `cgst` is `CGST`: `src-tauri/crates/bridge-tally-protocol/src/native_ledger_collection.rs:221`.
- `vouchers`' `goods_lines` argument: `src-tauri/src/agent.rs:359`.
- No request here is new. The tool calls are the 31 Oct row's. The raw reads are the committed template with only the company and the day changed. Every master and voucher is keyed by hand on Tally's screens; no Bridge request writes.

## What the row records, and what it does not

**Records:**
- the class's tax on an item purchase once both tax ledgers carry their heads;
- a plain-type control;
- one rounding case;
- one repeat of the class invoice;
- what the existing vouchers request, `vouchers` and `purchase_register` return for each, against the screen.

**Does not:**
- inter-state (IGST);
- the class's settings in any read: no existing request fetches a voucher class, so the class is recorded from Tally's screen only;
- a voucher's class name (not in the vouchers request's fetch list);
- `vouchers` `include_gst_details` (not passed);
- `Lab Purchases`' HSN/SAC details, or its rate details unless step 19 b runs;
- other releases, Gold, Education mode;
- several items, a discount, cess.

## Evidence

To `lab/2057-b-answers`, after the pre-push scan and the run coordinator's yes:
- `RESULTS.md`: the five answers (a) to (e), and every EXPECT line with PASS or FINDING;
- `notes.txt`: UTC, one line per step, with each mark;
- `answers/`: every tool answer, whole;
- `raw/`: each raw answer with its headers, size, sha256, and a `.parsed.txt` of its structure;
- `send-log.txt`;
- `restore-plan.txt`;
- `SCREENSHOTS.txt`: the captures and BK3 by sha256. They stay on the lab PC.

No identifier beyond the three of the Identifiers rule, and no figure beyond the book's own synthetic ones, is recorded. The company's number and data folder are not recorded.
