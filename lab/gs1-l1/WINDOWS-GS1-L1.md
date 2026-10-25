# Windows item GS1-L1 v3: GST per counter ledger read through ComplyEaze Bridge, on vouchers typed on Tally's screen in a FRESH synthetic company. (25 Oct 2026.) SYNTHETIC ONLY: THE COMPANY is `BRIDGE GST LEDGER LAB`, created by the owner in step 1 of this item; its company number and GUID are recorded there and every later step is checked against that GUID. No existing company is touched: in particular BRIDGE GST RECON LAB, whose committed captures this item must never change, is not loaded. LAB-COMMON v3 applies in full, except where "Differences" below says otherwise; where the two differ, the stricter holds. Independent of GS1-X1: a STOP in X1 does not stop this item.

Why: the read-only tool `gst_by_counter_ledger` (pull request 1867; in source, not in a published build) has been tested only on committed captures of eight vouchers created by import. This item keys the voucher shapes those captures do not have, on Tally's own screen, and reads them through the tool, one voucher per day and then the whole month, so its answers can be compared with Tally's own ledger screens and the relay's recording can become the pull request's fixtures. Nothing in this item is posted by ComplyEaze Bridge: the tool and `purchase_register` only read.

## The build and its settings
- TallyPrime Release 7.2 (the lab's release since the licence renewal), Silver, not Educational mode. The build is the `bridge_mcp` program of pull request 1867 at the head the run coordinator's go-ahead names, from that head's own CI run; the go-ahead names the program file's sha256, and a file with another hash is not started. Posting and ledger creation stay OFF. Port 9001.
- Its traffic goes through the serial read-only relay that recorded the purchase register's captures (relay sha256 `5f36ad420afd060546c09e148507c696c90e70a8901fdb03a5548057e9dbbde2`), forwarding to 127.0.0.1:9001 only, recording every request and response byte for byte. No widening: `gst_by_counter_ledger` sends exactly the requests `purchase_register` sends, in the same order (one shared read in the code), which the relay admitted 102 of 102 on 1 Oct 2026. A request the relay refuses = STOP.
- Exactly ONE ComplyEaze Bridge of this build is connected to the assistant, under the server name `bridge-lab-gs1`; every tool call goes to it, with exactly the arguments the step gives, one call at a time. After every call the assistant keeps the answer whole as a file in the evidence folder (named by step), with its size and sha256. A call in flight is never cancelled, abandoned or repeated; nothing is sent or keyed while one runs.

## Differences from LAB-COMMON
- C1: THE COMPANY does not exist before step 1; the owner creates it there. From step 2 on, C1 applies as written.
- C3: the operator's own requests are the two files below; the tool's own requests are the relay's to record. Size bounds: MARKS 8,192 bytes (the same request on another book: 3,310); LEDGERS 131,072. A tool answer above 1,000,000 bytes = STOP.
- C5: one backup, GS0, after the masters (step 3) and before the first voucher. C6: no restore (THE COMPANY is this item's own; it is kept, shut, for a later sitting).
- OWNER STEPS: every step marked OWNER is done by the owner himself, ONE act at a time (one master or one voucher, then its screenshot and the read the step names, then the next). The operator never keys.
- Keying: every voucher is typed on Tally's own voucher screen in voucher view ("As Voucher", toggled with Ctrl+V on the voucher screen), with each ledger's amount typed as given, Dr or Cr as given. If Tally offers to compute tax or fills a tax amount by itself, accept nothing, screenshot, and STOP (V2 and V3 have their own rule). A master is created on Tally's own screen with the name, group and duty type given and nothing else set (screenshot before accepting).

## Requests (the operator's own reads; derived from the files the masters run of 12 Oct used; in requests/)
| file | what | the one change from its source |
|---|---|---|
| marks.utf16le.xml | MARKS: `Bridge Agent Company High Water` (GUID, ALTVCHID, ALTMSTID of every loaded company) | the company in SVCURRENTCOMPANY |
| ledgers.utf16le.xml | LEDGERS: `List of Ledgers` (name, GUID, parent, bill-wise flag, ALTERID) | the company in SVCURRENTCOMPANY |
Bytes and sha256 in SHA256.txt. Before step 0: `python -I check-request-ids.py requests`, EXPECT exit 0 and `census: ok 2, skipped 0, FAIL 0`. Grant: these two reads as often as the steps say, the tool calls of the steps, the company, masters and vouchers of steps 1 to 5 keyed by the owner on screen, and nothing else.

## Folders and backups
Working folder `C:\bridge-lab\gs1\`, evidence `C:\bridge-lab\gs1\evidence\`, the program's data folder `C:\bridge-lab\gs1\data\`, backup GS0 `C:\bridge-lab\gs1\backups\gs0\` (absent or empty BEFORE, exactly one file AFTER).

## Steps ("STATE" = a screenshot of the whole Tally window FIRST, then MARKS, then LEDGERS, each kept as a file)
0. LAB-COMMON step 0, with NO company loaded (a loaded company = shut nothing, STOP). Also: Tally's About screen (EXPECT Release 7.2; record it; anything else = STOP); licence Silver, not Educational mode; the request hashes against SHA256.txt; the program file's sha256 equal to the go-ahead's; the data folder listing. EXPECT no company named `BRIDGE GST LEDGER LAB` in Select Company's list (one present = STOP).
1. OWNER: create the company on Tally's Create Company screen: name `BRIDGE GST LEDGER LAB`, financial year beginning 1-4-2025, books beginning 1-4-2025, no password, every other field as the screen offers. Screenshot before accepting. Then, OWNER, on its F11 Features screen: Enable Goods and Services Tax (GST) = Yes; if the screen then asks for registration details, State = Rajasthan, registration type Regular, GSTIN `08ZZZZZ0000Z1ZQ` (the synthetic identifier the push gate admits; any other identifier = STOP), nothing else. Record whether "Maintain inventory" (or the screen's equivalent) reads Yes. Screenshot before accepting; leave with Ctrl+Q. Load it alone. STATE: record the company number (from the Select Company list), the GUID and both marks from MARKS. EXPECT exactly one COMPANY row; every later answer is checked against this GUID.
2. OWNER, one master at a time, each followed by a screenshot of the saved master and one MARKS read (EXPECT ALTMSTID up):
   - Duties & Taxes, type of duty GST: `Input CGST` (tax type Central Tax), `Input SGST` (State Tax), `Input IGST` (Integrated Tax), `Output CGST` (Central Tax), `Output SGST` (State Tax).
   - Duties & Taxes, type of duty Others: `GS1 TDS Payable`.
   - Purchase Accounts: `Purchase - Goods`. Indirect Expenses: `GS1 Rent`, `GS1 Professional Fees`, `GS1 Interest on GST`, `GS1 Discount Allowed`, `GS1 Freight Inward`. Direct Expenses: `GS1 Freight Direct`. Fixed Assets: `GS1 Computer`.
   - Sundry Creditors: `SYN GS1 Supplier`. Sundry Debtors: `SYN GS1 Customer`.
   - Only if step 1 read inventory Yes: one stock item `GS1 Item`, unit Nos, nothing else set.
3. STATE. EXPECT exactly those 16 ledgers (plus Tally's own `Cash` and `Profit & Loss A/c`) with those parents. Backup GS0 (C5), then MARKS.
4. OWNER, one voucher at a time, each on its own day of November 2025, in voucher view, party `SYN GS1 Supplier` unless named otherwise; screenshot each before accepting; after each, MARKS (EXPECT ALTVCHID up by at least one):
   - V1, 3 Nov, Purchase: Dr `GS1 Rent` 10,000.00; Dr `Input CGST` 900.00; Dr `Input SGST` 900.00; Cr party 11,800.00.
   - V4, 4 Nov, Journal: Dr `GS1 Professional Fees` 20,000.00; Dr `Input IGST` 3,600.00; Cr party 23,600.00.
   - V5, 5 Nov, Purchase: Dr `GS1 Computer` 40,000.00; Dr `Input CGST` 3,600.00; Dr `Input SGST` 3,600.00; Cr party 47,200.00.
   - V6, 6 Nov, Purchase: Dr `GS1 Freight Direct` 2,000.00; Dr `Input CGST` 180.00; Dr `Input SGST` 180.00; Cr party 2,360.00.
   - V7, 7 Nov, Purchase: Dr `GS1 Rent` 5,000.00; Dr `GS1 Professional Fees` 5,000.00; Dr `Input CGST` 900.00; Dr `Input SGST` 900.00; Cr party 11,800.00.
   - V4b, 8 Nov, Journal: Dr `GS1 Professional Fees` 10,000.00; Dr `Input IGST` 1,800.00; Cr `GS1 TDS Payable` 1,000.00; Cr party 10,800.00.
   - V8, 10 Nov, Debit Note: Dr party 1,180.00; Cr `GS1 Rent` 1,000.00; Cr `Input CGST` 90.00; Cr `Input SGST` 90.00.
   - V9, 11 Nov, Journal: Dr `Output CGST` 500.00; Cr `Input CGST` 500.00.
   - V10, 12 Nov, Journal: Dr `GS1 Rent` 180.00; Cr `Input CGST` 90.00; Cr `Input SGST` 90.00.
   - V11, 13 Nov, Payment: Dr `Output CGST` 100.00; Dr `Output SGST` 100.00; Cr `Cash` 200.00.
   - V13, 15 Nov, Payment (a GST challan that also pays interest): Dr `Output CGST` 90.00; Dr `Output SGST` 90.00; Dr `GS1 Interest on GST` 50.00; Cr `Cash` 230.00.
   - V14, 16 Nov, Credit Note (a discount allowed with output tax), party `SYN GS1 Customer`: Dr `GS1 Discount Allowed` 1,000.00; Dr `Output CGST` 90.00; Dr `Output SGST` 90.00; Cr party 1,180.00.
   - Only if step 1 read inventory Yes: V2, 14 Nov, Purchase in Item Invoice mode: `GS1 Item` 1 Nos at 5,000.00 with purchase ledger `Purchase - Goods`, then `Input CGST` 450.00 and `Input SGST` 450.00, party Cr 5,900.00; V3, 17 Nov, the same with `GS1 Freight Inward` 100.00 added (party Cr 6,000.00). If Tally fills a tax amount itself on V2 or V3: screenshot, leave that voucher unsaved with Ctrl+Q after a screenshot of its title, record "item invoice: Tally computed tax", and key no further item invoice.
5. OWNER: V12, 50 copies of V1, dated 18 to 27 November 2025, five a day, each by Tally's duplicate (Alt+2 on V1) with only the date changed, one at a time with MARKS after every fifth. STATE. EXPECT ALTVCHID up by at least 62 (64 with V2 and V3) over step 3, and LEDGERS unchanged since step 3.
6. Tool calls, one per day, each followed by MARKS (EXPECT both marks unchanged since step 5: the tool writes nothing). Each call: `gst_by_counter_ledger` with `company_guid` THE COMPANY's GUID, `from` = `to` = the day. EVERY answer must read `state` complete (anything else = STOP: a partial window would also show an empty `counter_ledgers`). The state-side head is `state_tax` or `sgst_utgst` depending on what Tally stored; record which, and read "state" below as that head. EXPECT:
   - 3 Nov (V1): `counter_ledgers` holds `GS1 Rent` only: vouchers 1, `debit_side_tax` cgst -900, state -900; `set_aside` empty.
   - 4 Nov (V4): `GS1 Professional Fees`: vouchers 1, igst -3600.
   - 5 Nov (V5) and 6 Nov (V6): no counter ledger; `set_aside.other_ledgers_present` 1 voucher, with the voucher's tax.
   - 7 Nov (V7): `set_aside.several_counter_ledgers` 1 voucher, cgst -900, state -900.
   - 8 Nov (V4b): `set_aside.duty_ledger_without_gst_type` 1 voucher, igst -1800.
   - 10 Nov (V8): `GS1 Rent`: vouchers 1, `credit_side_tax` cgst 90, state 90.
   - 11 Nov (V9): `set_aside.gst_on_both_sides` 1 voucher.
   - 12 Nov (V10): `set_aside.no_creditor_party` 1 voucher, `credit_side_tax` cgst 90, state 90.
   - 13 Nov (V11): `set_aside.no_creditor_party` 1 voucher, debit cgst -100, state -100.
   - 15 Nov (V13): `set_aside.no_creditor_party` 1 voucher, debit cgst -90, state -90; `counter_ledgers` empty (any attribution to `GS1 Interest on GST` = FINDING of the first rank).
   - 16 Nov (V14): `set_aside.class_not_admitted` 1 voucher, debit cgst -90, state -90; `counter_ledgers` empty (any attribution to `GS1 Discount Allowed` = FINDING of the first rank).
   - 14 Nov and 17 Nov (V2, V3), if keyed: RECORD, do not judge: either `set_aside.entries_do_not_balance` (the purchase ledger was not among the ledger entries) or V2 attributed to `Purchase - Goods` and V3 `several_counter_ledgers` (it was). Which one is the finding.
   - Any voucher in `set_aside.unrecognised_head` or `head_absent`: FINDING of the first rank (the duty heads Tally stores for ledgers typed on screen are not the ones the tool recognises); record the raw heads from the relay's master answer and go on.
   A different answer is a FINDING (C11: to the run coordinator before the next step), not a STOP, unless the call refused: a refusal = STOP, recorded with its code.
7. `gst_by_counter_ledger` for 1 to 30 November 2025. MARKS after. EXPECT `state` complete; `GS1 Rent` vouchers 52 (V1, V8 and the 50 copies), `debit_side_tax` cgst -45900, state -45900, `credit_side_tax` cgst 90, state 90; `GS1 Professional Fees` vouchers 1, igst -3600; `set_aside` as the sum of step 6. A refusal on size = record its code and STOP; any other refusal = STOP.
8. `purchase_register` for 1 to 30 November 2025 (offset 0, default limit), then MARKS. RECORD its `total`, `state` and the side lists' totals (it is the comparison).
9. OWNER, BY EYE, one screen at a time (Display > Account Books > Ledger, November 2025), each a screenshot: `Input CGST`, `Input SGST`, `Input IGST`, `GS1 Rent`, `GS1 Professional Fees`. EXPECT the GST of the vouchers attributed to `GS1 Rent` and `GS1 Professional Fees` in step 7 to be the tax those vouchers carry on these screens.
10. STATE. EXPECT the marks of step 5 unchanged. Then LAB-COMMON C10 (THE COMPANY shut; no company loaded).

## What the books hold after the item
BRIDGE GST LEDGER LAB: 16 ledgers (and a stock item if inventory was on) and 62 or 64 vouchers, kept for a later sitting. No other company is loaded, read or changed at any step.

## Evidence
Every tool answer whole (the assistant's files), every operator answer, the relay's recording of every request and response for steps 6 to 8 with sizes and sha256, screenshots listed by name, size and sha256 (never pushed), the notes file. The recording of steps 6 to 8 becomes the fixtures of pull request 1867; it carries THE COMPANY's synthetic names and the synthetic GSTIN only.
