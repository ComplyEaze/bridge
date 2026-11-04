# 2057-D an item purchase keyed with the voucher's common ledger turned off (#2057): the one shape #2057 has not measured

Not for merge. About 2 hours.

This sheet is 2057-C's (`lab/2057-c-sheet` at 67b12237), whose steps 3 to 14 were never run, with the option 2057-C2 found named in place of steps 2a to 2c.

This sheet WRITES by hand on Tally's screens:
- company GST details;
- two ledgers;
- one stock item;
- one configuration option, on the Purchase voucher screen's own F12, set and then set back;
- one Purchase voucher.

It then reads, sets the option back, and restores the book from one backup. ComplyEaze Bridge only reads. `SHA256.txt` lists every file here with its bytes; check it before step 0.

`LAB-COMMON.md` in this folder applies in full. It is v3, byte-equal to the one in `lab/2057-b-sheet` (sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6). Where it and this sheet differ, the stricter holds. The row runs alone and restores its own backup.

## What it decides

`purchase_register`'s description lists as not measured "item invoices whose purchase ledger sits in an inventory allocation". Every item invoice captured so far holds the purchase or sales ledger as an `ALLLEDGERENTRIES.LIST` of its own, with the goods line inside it. That covers 2057-B's four (`lab/2057-b-answers` at d9005ab3, protocol reference §8.2m) and those of §8.2i.

**The unmeasured shape:** the goods line at voucher level, with the purchase ledger only in its own accounting allocation.

**The option.** Two look-only rows searched for the setting that makes the voucher take the purchase ledger per item line:
- 2057-C (`lab/2057-c-answers`, RESULTS.md) found none on the Gateway's F12 (it opened nothing) or on the Purchase voucher type, and saved nothing after its backup.
- 2057-C2 (`lab/2057-c2-answers` at 61cfb757, RESULTS.md) found exactly one, on the Purchase voucher screen's own F12 ("Voucher Configuration"), in Item Invoice mode: **"Select common Ledger Account for Item Allocation : Yes"**. Data > Configuration holds none.

By 2057-C's rule for a label that speaks of a common ledger, the value to set is the one that turns the common ledger off: **No**. With Yes, the voucher screen shows one "Purchase ledger" field at its head (2057-C2's `transcriptions/s4-purchase-voucher-screen.txt`).

This row sets that one option to No (step 7), keys one plain-type item invoice, and reads it two ways:
- with the shipped vouchers request (`ALLLEDGERENTRIES.*`);
- with the shipped stock-lines request (`ALLINVENTORYENTRIES.*`).

**No EXPECT is stated** for where the item or the ledger sits, or for what the tools return; each is recorded as found. EXPECT lines remain only for marks, identity, backup and restore.

**Limit, whatever the result:** one book, one release, one voucher, intra-state only.

## Rules and company

**Who acts.** On the owner's standing word of 27 and 28 Oct 2026, the lab does every step here, including the keying, on this synthetic company, and records each as LAB-COMMON C11 directs. The final restore waits for the run coordinator's word RESTORE.

**THE COMPANY** is `BRIDGE CLASS PURCHASE LAB` (synthetic), GUID `92a7ef97-dd28-4286-b02c-c059fcb8690e`, as the 2057-B row left it after its restore (`RESULTS.md` there, "Back to the start"), and as 2057-C and 2057-C2 found it, with nothing saved by either:
- "No Vouchers Entered";
- the Gateway's Current Date 1-Apr-2025;
- marks 213 / null;
- its masters as imported: unit Nos, item `Lab Class Item`, ledgers `Lab Purchases`, `Lab Supplier` (Sundry Creditors), and `Lab CGST` and `Lab SGST` (Duties & Taxes);
- no voucher class.

If it is absent, or step 1's answers differ from 2057-B's as step 1 sets out: STOP and report (the run coordinator decides). Only THE COMPANY is loaded, alone, from step 0 to the last step. Licensed mode only; Education mode = STOP.

**Identifiers.** Only these, which are of the push gate's synthetic family (the PAN part `ZZZZZ0000Z`, with valid check characters), are keyed. Any other GSTIN or PAN = STOP before it is saved.
- the company's GSTIN `27ZZZZZ0000Z1ZQ`;
- the supplier's GSTIN `27ZZZZZ0000Z2ZP`;
- the HSN `340211`.

Differences from LAB-COMMON (the stricter rule holds everywhere else):

1. **Two kinds of read.**
   - **Tool reads** are calls of ComplyEaze Bridge's built tool, one at a time.
   - **Raw reads** by curl are allowed by the owner's decision of 26 Oct 2026 for shape discovery. They are only the two files in `requests/`, written by `gen_requests.py` from two committed requests. `python -I gen_requests.py --check` must print its ok line, with 2 requests, before step 0. Each is sent once, by C3's line, and each answer must be under 2,000,000 bytes.
     - `r-a07-20250615` is the vouchers request 2057-B sent, for this row's day.
     - `r-sl-20250615` is the request `vouchers` sends with `stock_lines` true: the committed `stock_movement_mixed_window_stock_lines_shape_lab_request.utf16le.xml` with only the company and the day changed.

     Neither is a new shape: each is a request Bridge already sends. The rule of C3 still holds: a screenshot after each.
   - An empty envelope, a 0-byte 200 answer or an in-band refusal is a **result**, recorded whole.
2. **Marks.** M (`ALTMSTID`) and V (`ALTVCHID`) are read with `ledger_masters` (basic, first page, no `snapshot_id`): `snapshot.master_alter_id`, `snapshot.voucher_alter_id` (null was read after the 2057-B restore; record it as read), `snapshot.reused` false.
   - Each master save raises M by at least 1 and leaves V unchanged. Record the exact rise.
   - A configuration change (steps 7 and 13) may move M or nothing: record what it does. A V that moves on it = STOP.
   - The one voucher Accept raises V by exactly 1.
   - A V that moves on a master save, or by other than 1 on the voucher, = STOP.
3. **Backup and restore.** BK5 of THE COMPANY alone before the first change (C5), into a new empty folder `C:\bridge-lab\2057-d\backups\bk5\`. At the end, on the word RESTORE, BK5 is restored by C6.
   - **Fallback, only on the run coordinator's word:** 2057-C's BK4 (24,100 bytes, sha256 `e08bd0136b05e1a525c642ac1e51d9a1b8c905cbcf06884a5f8033fb9da23041`, taken on the same book with nothing saved after it), its hash checked first.
   - **Then:** 2057-B's BK3 (24,100 bytes, sha256 `94ba7d114a746c58b869c4b7e628625d8aa12f16c6cba7270daf09d259ed0745`), its hash checked first.
   - **Then:** the 31 Oct BK2 (24,100 bytes, sha256 `8ca808192ec97a4015b73aad7fd83b64d8d60e16e917c3c9b570454cae0d1405`).
4. **Keys.**
   - Accept with Ctrl+A. Leave a creation, alteration, configuration or voucher screen with ONE Ctrl+Q, only after a screenshot shows its title.
   - **Never Ctrl+Q or Escape at the Gateway of Tally**, never two Ctrl+Q in one call, and never Escape on a master, configuration or voucher screen (C7).
   - A sub-form Tally opens by itself (Receipt Details, Party Details, a GST rate screen, an accounting-details screen on the item line) is accepted as shown unless the step names a field, with a screenshot.
   - Two dialogs are answered; any other dialog or prompt, anywhere, = C7: no key, STOP.
     - The one the 31 Oct and 4 Nov rows met at Create > GST Registration: "GST Registration already exists for the Company. Do you want to alter the existing GST Registration or create a new GST Registration?" Answer **Alter Existing**.
     - On leaving a blank voucher-creation screen at steps 7, 8 (its re-read), 13 and 14: a prompt whose text asks whether to quit, or to leave without saving, the voucher being created (2057-C2's rule 4). Screenshot it and copy its text exactly. Answer the choice that leaves WITHOUT saving (for example "Yes" to "Quit?"). If the text does not make plain which choice discards the voucher, press no key and STOP. 2057-C2 met none: one Ctrl+Q left a blank voucher with no prompt.
   - **On a voucher's F12 screen** (steps 7, 8, 13 and 14), move only with the Down arrow, one press at a time, with a screenshot after each. Press Enter on no option and open no sub-screen. Only step 7 and step 13 change a value, and only the one option named; the other steps look and leave.
   - Reach a master or an option by the label read on screen before Enter, not by a count of Up or Down presses.
5. **Never type a tax amount.** On the voucher, a tax line is left with the amount Tally puts there, or with none. An empty tax line is a result, not an error.
6. **One Tally.** At step 0 and at the end, as C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.

## Build

The 2057-B row's build, unchanged, so every tool call is the one that row made:
- **The bundle:** the Windows bundle of the merge-queue run of #2327, CI run 41745127138, `merge_group`, head ae9a1bb5 (`ae9a1bb5096010a28fde3eac19aac4457f49eb69`), success.
- **The artifact:** `unsigned-bundle-smoke-windows-latest`, id 13283891845, 59,284,798 bytes, digest `sha256:922968055bb143f1a802beffbb496bbd94795a539382f9b5ae013c4985a45685`.

Check the zip's sha256 against that digest first; another digest is a stop. Record `bridge_mcp.exe`'s sha256 before the first call and after the last. It must equal 2057-B's (`c5b75d6480202ef70a5216f74d369ea0963b4a2eedbb3d4912e4f25b385e943d`); a difference, or a change during the row, = STOP.

One launch over stdio:
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`;
- ONE new empty `BRIDGE_AGENT_DATA_DIR`;
- `BRIDGE_TERMS_ACCEPTED=true`;
- `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` all absent.

Client: `scripts/client.py` of `lab/2057-item-purchase-class-gst-answers` at a8bcd9f8 (sha256 `c127118d327a50f7836118f440a0da9855215ea0b55c50e5f56fd1da91b8a600`), which refuses the write settings. Record its sha256.

`vouchers`, `purchase_register`, `ledger_masters`, `masters` and `list_companies` must be in `tools/list`; one absent = STOP.

## Files

- `gen_requests.py` and `templates/` (two committed requests);
- `requests/` (two generated reads);
- `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Steps

### Step 0 and the book as it stands

0. LAB-COMMON C2 step 0 with THE COMPANY loaded alone. Record Tally's release, edition, current period and current date from the Gateway. EXPECT Current Date 1-Apr-2025 and "No Vouchers Entered".
1. `tools/list` (count). Then:
   - `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, with the GUID above.
   - Marks: M0, V0. EXPECT 213 / null.
   - `ledger_masters` `fields` `compliance`, `include_gst_details` true: record the rows of the five ledgers whole. EXPECT them equal to 2057-B's `answers/003-s1-ledgers-gst.json` in every field but the snapshot's id and read time.
   - `masters` `kind` `voucher_types`: record the Purchase row whole. EXPECT it equal to 2057-B's `answers/004-s1-vchtypes-call1.json`'s, apart from the snapshot.
2. **Backup BK5** (rule 3), then the marks: EXPECT M0 and V0.

### The option, as 2057-C2 found it

2057-C's steps 2a to 2c are not run: 2057-C and 2057-C2 made them. Their result is fixed here, and step 7 uses it:
- **the screen:** Gateway > Vouchers, the Purchase voucher creation screen in Item Invoice mode, with nothing keyed, then F12 ("Voucher Configuration");
- **the label:** "Select common Ledger Account for Item Allocation", under General Details;
- **V-before:** Yes;
- **the value step 7 sets:** No.

### The GST setup of 2057-B: one change per save, marks after each

Steps 3 to 6 are 2057-B's steps 3 to 6, word for word. Step 6b keys what 2057-B's step 9 found (Lab CGST's Tax type `CGST`); it does not repeat that step's discovery. 2057-B's marks after each are given as EXPECT; a different rise is recorded, not a stop, while V is unchanged. 2057-B's step 7 (a look) and step 8 (the voucher class) are NOT done here: this row keys no class.

3. **The company** (K: Company > Alter, as B4 (b)): State `Maharashtra`. Accept. Then F11 Company Features: "Enable Goods and Services Tax (GST)" must already read Yes (record it; if No, STOP). Set "Set/Alter Company GST Rate and Other Details" Yes only if the GST registration screen cannot be reached otherwise, and accept its screen unchanged. EXPECT M 215.
4. **The company's GST registration** (Create > GST Registration > the dialog of rule 4 > Alter Existing): Registration status Active; State Maharashtra; Registration type Regular; **GSTIN/UIN `27ZZZZZ0000Z1ZQ`**; everything else as shown. Screenshot before the Accept. If Tally refuses the GSTIN, copy its message exactly and STOP (the identifier is the gate's; nothing else is tried). EXPECT M 216.
5. **The supplier** (Alter > Ledger > `Lab Supplier`): Country India; State Maharashtra; Registration type Regular; **GSTIN/UIN `27ZZZZZ0000Z2ZP`**; everything else as shown. Screenshot before the Accept. EXPECT M 217.
6. **The item** (Alter > Stock Item > `Lab Class Item`):
   - GST applicability Applicable;
   - HSN/SAC Details "Specify Details Here", HSN/SAC `340211`, description as offered;
   - GST Rate Details "Specify Details Here", Taxability Taxable, IGST 18% (Tally fills CGST 9% and SGST/UTGST 9%; record what it shows);
   - Type of Supply Goods.

   Screenshot before the Accept. EXPECT M 218. (On 4 Nov the screen showed one "GST Rate" field, keyed 18 %.)

   6b. **Lab CGST's Tax type** (Alter > Ledger > `Lab CGST`):
   - Screenshot the whole screen as it opens.
   - Open the field "Tax type" (on 4 Nov it was empty on Lab CGST and read "State Tax" on Lab SGST; its list "Tax Types" held IGST, CGST, SGST/UTGST, Cess). Screenshot the list whole and record every entry. Choose `CGST`.
     - STOP with no key pressed, and report, if: the field is not on the screen; the list does not hold `CGST`; or the list differs from 4 Nov's.
   - Every other field stays as shown. Screenshot before the Accept. Accept.
   - Marks: M up, V unchanged; record the rise. EXPECT M 219 (on 4 Nov the rise was 1).
   - Then `ledger_masters` `fields` `compliance`, `include_gst_details` true: the five ledgers' rows whole.
     - EXPECT Lab CGST `gst_duty_head` `recognized`, head `cgst`, raw `CGST`; Lab SGST `state_tax`.
     - Lab CGST read otherwise = STOP and report, with the raw value. Nothing more is keyed.

After each of steps 3 to 6 and 6b: the marks (M up, V unchanged; record the rise).

The book after step 6b: 2057-B's setup without its class, and Lab CGST with its head. Nothing else.

### The one new change: the option 2057-C2 found

7. **C1, on the voucher's own F12:**
   - Gateway > Vouchers > F9 Purchase. Screenshot the voucher screen as it opens and record its title, its mode and its head fields. If it is not in Item Invoice mode, press Ctrl+H, screenshot the mode list, and press Enter on "Item Invoice" only. Type nothing in any field.
   - Press F12 (Configure). Screenshot the "Voucher Configuration" screen whole and transcribe every option with its value, in the screen's own words, as 2057-C2's `transcriptions/s4-voucher-configuration.txt` has them. Record any difference from that list.
   - **Set nothing, and STOP, if:** the screen is not "Voucher Configuration"; the label "Select common Ledger Account for Item Allocation" is not on it in those exact words; or it does not read Yes. Leave as rule 4 says, then read the marks.
   - Bring the highlight to that label with the Down arrow (rule 4). Set it to **No** by typing `N` on it, and nothing else. Screenshot it reading No. If one `N` does not make it read No, press no further key and STOP.
   - Change no other option. Screenshot the whole screen, then Accept it (Ctrl+A).
   - Screenshot the screen Tally returns to. Record its title, its head fields and its columns, and in particular whether the "Purchase ledger" field still shows at the head.
   - Leave the blank voucher with ONE Ctrl+Q (rule 4's quit prompt holds), and screenshot the screen Tally returns to.
   - Record the label, V-before Yes and the value after, No.
   - Marks after: record M and V. **V moving = STOP and report: a voucher may have been saved; do not alter or delete it.** M is recorded, not EXPECTed: whether it moves says whether Tally keeps the option in the company's data.

### The invoice

8. **C2-C, 15-06-2025, plain Purchase:**
   - **The option first.** Gateway > Vouchers > F9 Purchase, Item Invoice mode as in step 7, nothing typed. F12, a screenshot, and record that "Select common Ledger Account for Item Allocation" reads No. Change nothing. Leave the F12 screen with ONE Ctrl+Q back to the voucher screen.
     - It reads Yes, or is not there: STOP and report, with nothing keyed. Leave the blank voucher as rule 4 says.
     - A prompt on leaving the F12 screen: rule 4 holds for the voucher's quit prompt; anything else is C7. If that prompt closed the blank voucher, read the marks (equal to step 7's), then open Gateway > Vouchers > F9 Purchase once more, without F12, and key the invoice there.
   - On the voucher screen, still in Item Invoice mode, key the invoice below. If it opened in another mode, Ctrl+H to Item Invoice, and record the mode it opened in.
   - Supplier Invoice No. `LAB-2057-6`, dated 15-Jun-25; Party A/c `Lab Supplier`. The Party Details sub-form is accepted as shown: record its State, Registration type, GSTIN and Place of Supply.
   - **Where Tally asks for the purchase ledger: record it before keying it.**
     - Record whether a purchase-ledger field shows at the head of the voucher, as on 2057-B's C1-B, and whether a screen or column on the item line asks for a ledger.
     - Where a ledger is asked, choose `Lab Purchases`. Where Tally asks for an amount or a percentage there, accept what it fills.
     - Screenshot each such screen before accepting it.
   - Item `Lab Class Item`, 5 Nos @ 100.00 = 500.00.
   - Below the item, if Tally does not add them itself, select `Lab CGST` and then `Lab SGST` and leave their amounts as Tally fills them (rule 5). Record whether Tally added them or they were selected.
   - Before the Accept, a screenshot showing every line and the total. Record each line and the total as shown, with each tax line's amount or "no amount". No amount is EXPECTED here; what is shown is the result.
   - Accept. Marks: V +1.

The book after step 8: step 6b's, the option of step 7, and C2-C.

### The reads

9. Marks before the group (M1, V1). Then, in this order:
   - the raw `requests/r-a07-20250615.utf16le.xml` (C3's line), with a screenshot after it;
   - the raw `requests/r-sl-20250615.utf16le.xml` (C3's line), with a screenshot after it;
   - `vouchers` with `from` and `to` 20250615 and `goods_lines` true, saved whole.
10. `purchase_register` with `from` 20250601 and `to` 20250630, saved whole. Then `ledger_masters` `fields` `compliance`, `include_gst_details` true, the five ledgers' rows whole, and `masters` `kind` `voucher_types`, the Purchase row whole.
11. Marks after the group: equal to M1 and V1, else STOP.
12. **Tally's own screens:**
    - C2-C displayed from the Day Book (15 Jun 2025), a screenshot, with every line, amount and the total transcribed;
    - then the Purchase Register for June 2025 (Display More Reports > Account Books > Purchase Register), a screenshot, transcribed.

    Marks after: equal to M1 and V1.

**Record, for C2-C, read by structure** (LAB-COMMON C4; ROWS are the `VOUCHER` children of the collection):
- **from the raw a07 answer:**
  - each `ALLLEDGERENTRIES.LIST`: LEDGERNAME, AMOUNT, ISDEEMEDPOSITIVE, and every child list it carries, by name (`INVENTORYALLOCATIONS.LIST` with STOCKITEMNAME, ACTUALQTY, RATE, AMOUNT and its own `ACCOUNTINGALLOCATIONS.LIST`; `RATEDETAILS.LIST`);
  - whether a top-level `ALLINVENTORYENTRIES.LIST` is present;
  - ISINVOICE and PARTYGSTIN;
  - the sum of the `ALLLEDGERENTRIES.LIST` AMOUNTs, as exact decimals.
- **from the raw sl answer:** each `ALLINVENTORYENTRIES.LIST` (STOCKITEMNAME, ACTUALQTY, RATE, AMOUNT, ISDEEMEDPOSITIVE, and each `ACCOUNTINGALLOCATIONS.LIST` child with its LEDGERNAME and AMOUNT), and each `LEDGERENTRIES.LIST` or `ALLLEDGERENTRIES.LIST` present, by LEDGERNAME and AMOUNT.
- **from `vouchers`:** each entry's ledger and amount, `goods_lines` or `goods_lines_unreadable`, and the item's `state`.
- **from `purchase_register`:** which list holds C2-C. If `items`: its `status`, `tax_in_books` (ledger, head, raw_head, amount), `taxable_entries`, `has_taxable_entry`, `party_entries` and `other_entries`.

**Answer, from what was recorded:**
- **(a)** Did step 7 set "Select common Ledger Account for Item Allocation" from Yes to No, and what did the marks do? Did the voucher screen's head fields change, and did step 8 read No again?
- **(b)** Where did Tally ask for the purchase ledger on C2-C: once for the voucher, on the item line, or both?
- **(c)** In the a07 answer, is `Lab Purchases` an `ALLLEDGERENTRIES.LIST` of its own? Where is the item? Do the entries' amounts sum to zero?
- **(d)** In the sl answer, where are the item and `Lab Purchases`?
- **(e)** What did `vouchers` and `purchase_register` return for C2-C (`goods_lines`, the list, `has_taxable_entry`, `taxable_entries`)? Does each amount in the raw answers and the tool answers equal Tally's screen? A difference is reported whole before anything more is done.

### Back to the start

13. **Set the option back:**
    - By step 7's route (a blank Purchase voucher in Item Invoice mode, nothing typed, F12), screenshot the screen and check that the option reads No. If it reads otherwise, record it and STOP.
    - Set it to **Yes** by typing `Y` on it, and nothing else. Screenshot it reading Yes. If one `Y` does not make it read Yes, press no further key and STOP. Change no other option. Screenshot the whole screen, then Accept it (Ctrl+A).
    - Leave the blank voucher with ONE Ctrl+Q (rule 4).
    - Then open it once more by the same route, screenshot F12, and record that it reads Yes. Leave F12 with ONE Ctrl+Q, then the blank voucher with ONE Ctrl+Q (rule 4).
    - Marks: record M and V. V moving = STOP.
14. **Restore** (C6) from BK5, on the run coordinator's explicit RESTORE. Then, in order:
    - a screenshot of the Gateway (Current Date 1-Apr-2025, "No Vouchers Entered"), compared with step 0's. A difference only in which menu entry is highlighted is recorded, not a stop, as 2057-C2 found after a voucher screen (its step 6). Any other difference = STOP;
    - `list_companies`: exactly THE COMPANY, `verified_tuple`, the same GUID;
    - marks equal to those after BK5 (M0, V0);
    - `ledger_masters` and `masters` as in step 1, equal;
    - **the option:** open it by step 7's route, screenshot F12, and record its value. Leave F12 with ONE Ctrl+Q, then the blank voucher with ONE Ctrl+Q (rule 4). **If it does not read Yes: STOP and report**, with no key pressed on it.

    Anything else = STOP; the fallback is rule 3's.
15. Hash the exe. LAB-COMMON C10: Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; Tally's current date as step 0 recorded; one tally.exe; 9001 LISTENING; `GET /status` 200.

## STOP rules

LAB-COMMON's, with rules 1 to 6 above. Also:
- a raw answer that does not parse as XML;
- a voucher saved with a wrong item, quantity, rate, party or date (do not alter it: STOP and report what was saved);
- any identifier keyed other than the three of the Identifiers rule;
- any change to a master other than those of steps 3 to 6 and 6b, and to any option other than the one of step 7;
- step 6b's STOP cases;
- at step 7: the F12 screen not "Voucher Configuration", the label not there in its exact words, or not reading Yes (set nothing);
- at step 8: the option not reading No before the invoice is keyed (key nothing);
- the option not reading Yes after step 13, or after the restore (step 14);
- a V that moves at step 7 or 13, or on leaving a blank voucher;
- any prompt or dialog not named in rule 4 (C7).

## What the book holds after each step (C8)

| after | the book |
|---|---|
| step 2 | as at step 1 (BK5 taken) |
| steps 3 to 6 | plus each GST change, in turn |
| step 6b | and Lab CGST's Tax type |
| step 7 | and "Select common Ledger Account for Item Allocation" No (it may be held outside the company's data: step 7's M says which) |
| step 8 | and C2-C |
| step 13 | the option back at Yes |
| step 14 | as at BK5, the book of step 1 |

## What the row decides

- **C2-C holds the item at voucher level with `Lab Purchases` only in its accounting allocation:** the unmeasured shape exists on 7.2, made by the option recorded at (a).
  - The a07 answer, the sl answer and the tool answers then say what Bridge's reads do with it.
  - The a07 and sl answers are committed as fixtures by the gate's manifest route, and the protocol reference gains an entry with its confidence marker.
  - Any code change waits for a ruling.
- **C2-C holds `Lab Purchases` as an entry of its own, as on 2057-B:** the option recorded at (a) does not make the shape, on this book and release. The description's "not measured" stays, with this row and 2057-C and C2 cited.
- **Anything else** (the ledger asked both at the head and on the item line, a refusal on Accept, the option not holding from step 7 to step 8): recorded, and it goes to a ruling.
- **A read differs from the screen:** stop and report. Any code waits for a ruling.

## What master admits, at 5680ba48

The build (ae9a1bb5) has the same lines but where noted. Between the two, the read files differ only in the dispatch words and a description wording, and neither reaches the requests this row sends.
- `purchase_register` classifies only from each row's `amounts`, the read's ledger entries: `src-tauri/src/agent_register.rs:317` and `:330`. An entry on a Purchase Accounts ledger is a taxable entry (`:380`), and `has_taxable_entry` is whether there is one (`:501`). No step checks that the entries net to zero. The side list for a Purchase with no Duties & Taxes entry is at `:66`, and the description's "not measured" line at `:104`.
- The vouchers request with `ALLLEDGERENTRIES.*`: `AGENT_VOUCHER_FETCH`, `src-tauri/src/agent_read_profiles.rs:47`. Its committed request is pinned at `src-tauri/src/agent_voucher_goods_lines_tests.rs:167` (build `:165`).
- The stock-lines request with `ALLINVENTORYENTRIES.*`: `AGENT_VOUCHER_STOCK_LINE_FETCH`, `agent_read_profiles.rs:70` (build `:69`). Its committed request is pinned at `agent_voucher_goods_lines_tests.rs:781` (build `:719`). `vouchers` reads its lines only for four reserved names, not a Purchase (protocol reference §8.2i), so this row sends it raw.
- `vouchers`' `goods_lines` argument: `src-tauri/src/agent.rs:359`. A summary refuses a voucher whose entries do not net to zero: `src-tauri/src/agent_movement.rs:712`.
- No request here is new. The tool calls are 2057-B's, with this row's day. The raw reads are two committed requests with only the company and the day changed. Every master, option and voucher is keyed by hand on Tally's screens; no Bridge request writes. Bridge has no post path for a Purchase item invoice (#628).

## What the row records, and what it does not

**Records:**
- the voucher's F12 screen, transcribed at step 7, and the option's value at steps 7, 8, 13 and 14;
- the one option changed and set back, with the marks after each;
- one item invoice on the plain type;
- what the two shipped vouchers requests, `vouchers` and `purchase_register` return for it, against the screen.

**Does not:**
- inter-state (IGST);
- a voucher class;
- a sales invoice of the same shape;
- several items, a discount, cess;
- other releases, Gold, Education mode.

## Evidence

To `lab/2057-d-answers`, after the pre-push scan and the run coordinator's yes:
- `RESULTS.md`: the five answers (a) to (e), and every EXPECT line with PASS or FINDING;
- `notes.txt`: UTC, one line per step, with each mark;
- `answers/`: every tool answer, whole;
- `raw/`: each raw answer with its headers, size, sha256, and a `.parsed.txt` of its structure;
- `send-log.txt`;
- `restore-plan.txt`;
- `SCREENSHOTS.txt`: the captures and BK5 by sha256. They stay on the lab PC.

No identifier beyond the three of the Identifiers rule, and no figure beyond the book's own synthetic ones, is recorded. The company's number and data folder are not recorded.
