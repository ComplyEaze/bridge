# Windows item GS1-L1 v1: GST per counter ledger read through ComplyEaze Bridge, on vouchers typed on Tally's screen. (25 Oct 2026.) SYNTHETIC ONLY: THE COMPANY is BRIDGE GST RECON LAB (company number: the `COMPANYNUMBER` of the committed capture `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-book-extent.utf16le.xml`; GUID ae1490be-52c5-4544-9ffc-4b7da85f9797), the book the purchase register's committed captures come from. LAB-COMMON v3 applies in full, except where "Differences" below says otherwise; where the two differ, the stricter holds.

Why: the read-only tool `gst_by_counter_ledger` (pull request 1867; in source, not in a published build) has been tested only on that book's committed captures of eight vouchers created by import. This item keys the voucher shapes those captures do not have, on Tally's own screen, and reads them through the tool, one voucher per day and then the whole month, so its answers can be compared with Tally's own ledger screens and the relay's recording can become the pull request's fixtures. Nothing in this item is posted by ComplyEaze Bridge: the tool and `purchase_register` only read. The operator's keying is the only change to the book.

## The build and its settings
- The build is the `bridge_mcp` program of pull request 1867 at the head the run coordinator's go-ahead names, from that head's own CI run. The go-ahead names the program file's sha256; a file with another hash is not started. Writes are not needed: posting and ledger creation stay OFF. Port 9001.
- Its traffic goes through the serial read-only relay that recorded the purchase register's captures (relay sha256 `5f36ad420afd060546c09e148507c696c90e70a8901fdb03a5548057e9dbbde2`), forwarding to 127.0.0.1:9001 only, recording every request and response byte for byte. NO WIDENING is needed: `gst_by_counter_ledger` sends exactly the requests `purchase_register` sends, in the same order (one shared read in the code), and the relay admitted all 102 of those on 1 Oct 2026 with none refused. A request the relay refuses = STOP (it would mean the build is not the one reviewed).
- Exactly ONE ComplyEaze Bridge of this build is connected to the assistant, under the server name `bridge-lab-gs1`; every tool call of this item goes to it, with exactly the arguments the step gives, one call at a time. After every call the assistant keeps the tool's answer whole as a file in the evidence folder (named by step), with its size and sha256.
- A CALL IN FLIGHT is never cancelled, abandoned or repeated; nothing is sent to Tally or keyed while one runs.

## Differences from LAB-COMMON
- C3: the operator's own requests are the two files below; the tool's own requests are the relay's to record, not files of this item.
- Size bounds (C3), from earlier answers of this company: MARKS 8,192 bytes (the same request on another book: 3,310); LEDGERS 131,072 (this book has 54 ledgers; its committed paired listing is 56,672). A tool answer above 1,000,000 bytes = STOP.
- C5: one backup, GS0, before step 2 (the first keyed change). C6: no restore in this item.
- Keying: every voucher is typed on Tally's own voucher screen in voucher view ("As Voucher", toggled with Ctrl+V on the voucher screen), with each ledger's amount typed as given, Dr or Cr as given. Tally computes no tax in this item: if Tally offers to compute tax or fills a tax amount by itself, accept nothing, screenshot, STOP. A master is created on Tally's own ledger screen with the name and group given and nothing else set (GST applicability and every other field left at the default the screen shows; screenshot before accepting).

## Requests (the operator's own reads; derived from the files the masters run of 12 Oct used; in requests/)
| file | what | the one change from its source | bytes | sha256 |
|---|---|---|---|---|
| marks.utf16le.xml | MARKS: `Bridge Agent Company High Water` (GUID, ALTVCHID, ALTMSTID of every loaded company) | the company in SVCURRENTCOMPANY | see SHA256.txt | see SHA256.txt |
| ledgers.utf16le.xml | LEDGERS: `List of Ledgers` (name, GUID, parent, bill-wise flag, ALTERID) | the company in SVCURRENTCOMPANY | see SHA256.txt | see SHA256.txt |
Before step 0 run `python -I check-request-ids.py requests`: EXPECT exit 0 and `census: ok 2, skipped 0, FAIL 0`. Grant: these two reads as often as the steps say, the tool calls of the steps, the masters and vouchers of steps 2 to 4 keyed on screen, and nothing else.

## Folders and backups
Working folder `C:\bridge-lab\gs1\`, evidence `C:\bridge-lab\gs1\evidence\`, the program's data folder `C:\bridge-lab\gs1\data\`, backup GS0 `C:\bridge-lab\gs1\backups\gs0\` (absent or empty BEFORE, exactly one file AFTER).

## Steps ("STATE" = a screenshot of the whole Tally window FIRST, then MARKS, then LEDGERS, each kept as a file)
0. LAB-COMMON step 0. Also: Tally's version (record PRODMAJORREL / PRODMINORREL if shown on the About screen) and licence Silver, not Educational mode; the two request hashes against SHA256.txt; the program file's sha256 equal to the go-ahead's; the data folder listing. LOOK ONLY: F11 (Features) on THE COMPANY: record whether "Maintain inventory" (or "Integrate accounts with inventory") reads Yes. Change nothing on that screen; leave it with Ctrl+Q after a screenshot of its title.
1. STATE. Record ALTVCHID and ALTMSTID. EXPECT exactly one COMPANY row with THE COMPANY's GUID, and in LEDGERS: `Purchase - Goods`, `Input CGST`, `Input SGST`, `Input IGST`, `Output CGST`, `Output SGST`, `Freight Inward (M2)`, `TDS Payable (M2)`, `SYN Supplier Intra (M2)` and `Cash` present. Any missing = STOP. EXPECT none of `GS1 Rent`, `GS1 Professional Fees`, `GS1 Freight Direct`, `GS1 Computer` present (a present one = STOP).
2. Backup GS0 (C5), then MARKS. Then key four ledgers (one master screen each, screenshot before accepting): `GS1 Rent` under Indirect Expenses; `GS1 Professional Fees` under Indirect Expenses; `GS1 Freight Direct` under Direct Expenses; `GS1 Computer` under Fixed Assets. STATE. EXPECT exactly those four new GUIDs in LEDGERS, with those parents.
3. Key these vouchers, each on its own day of November 2025, in voucher view, party `SYN Supplier Intra (M2)` unless named otherwise. Screenshot each before accepting. After each, MARKS (EXPECT ALTVCHID up by at least one).
   - V1, 3 Nov, Purchase: Dr `GS1 Rent` 10,000.00; Dr `Input CGST` 900.00; Dr `Input SGST` 900.00; Cr party 11,800.00.
   - V4, 4 Nov, Journal: Dr `GS1 Professional Fees` 20,000.00; Dr `Input IGST` 3,600.00; Cr `TDS Payable (M2)` 2,000.00; Cr party 21,600.00.
   - V5, 5 Nov, Purchase: Dr `GS1 Computer` 40,000.00; Dr `Input CGST` 3,600.00; Dr `Input SGST` 3,600.00; Cr party 47,200.00.
   - V6, 6 Nov, Purchase: Dr `GS1 Freight Direct` 2,000.00; Dr `Input CGST` 180.00; Dr `Input SGST` 180.00; Cr party 2,360.00.
   - V7, 7 Nov, Purchase: Dr `GS1 Rent` 5,000.00; Dr `GS1 Professional Fees` 5,000.00; Dr `Input CGST` 900.00; Dr `Input SGST` 900.00; Cr party 11,800.00.
   - V8, 10 Nov, Debit Note: Dr party 1,180.00; Cr `GS1 Rent` 1,000.00; Cr `Input CGST` 90.00; Cr `Input SGST` 90.00.
   - V9, 11 Nov, Journal: Dr `Output CGST` 500.00; Cr `Input CGST` 500.00.
   - V10, 12 Nov, Journal: Dr `GS1 Rent` 180.00; Cr `Input CGST` 90.00; Cr `Input SGST` 90.00.
   - V11, 13 Nov, Payment: Dr `Output CGST` 100.00; Dr `Output SGST` 100.00; Cr `Cash` 200.00.
   - ONLY IF step 0 read inventory Yes: V2, 14 Nov, Purchase in Item Invoice mode: one stock item already in the book (record its name), quantity 1, rate 5,000.00, its purchase ledger `Purchase - Goods`, then `Input CGST` 450.00 and `Input SGST` 450.00, party Cr 5,900.00; and V3, 17 Nov, the same with `Freight Inward (M2)` 100.00 added (party Cr 6,000.00). If step 0 read No: key neither, record "item invoice not keyed: inventory off", and do NOT change the feature.
4. Key V12, 50 copies of V1, dated 18 to 27 November 2025, five a day, each by Tally's duplicate (Alt+2 on V1) with only the date changed. After the last, STATE. EXPECT ALTVCHID up by at least 59 (61 with V2/V3) over step 1, and LEDGERS unchanged from step 2.
5. Tool calls, one per day, each followed by MARKS (EXPECT both marks unchanged since step 4: the tool writes nothing). Each call: `gst_by_counter_ledger` with `company_guid` THE COMPANY's GUID, `from` = `to` = the day. EXPECT:
   - 3 Nov (V1): `counter_ledgers` holds `GS1 Rent` only: vouchers 1, `debit_side_tax` cgst -900, state_tax -900; `set_aside` empty.
   - 4 Nov (V4): `GS1 Professional Fees`: vouchers 1, igst -3600 (TDS Payable ignored).
   - 5 Nov (V5) and 6 Nov (V6): no counter ledger; `set_aside.other_ledgers_present` 1 voucher, with the voucher's tax.
   - 7 Nov (V7): `set_aside.several_counter_ledgers` 1 voucher, cgst -900, state_tax -900.
   - 10 Nov (V8): `GS1 Rent`: vouchers 1, `credit_side_tax` cgst 90, state_tax 90.
   - 11 Nov (V9): `set_aside.gst_on_both_sides` 1 voucher.
   - 12 Nov (V10): `GS1 Rent`: vouchers 1, `credit_side_tax` cgst 90, state_tax 90.
   - 13 Nov (V11): `set_aside.no_counter_ledger` 1 voucher, debit cgst -100, state_tax -100.
   - 14 Nov and 17 Nov (V2, V3), if keyed: RECORD, do not judge: either `set_aside.entries_do_not_balance` (the purchase ledger was not among the ledger entries) or V2 attributed to `Purchase - Goods` and V3 `several_counter_ledgers` (it was). Which one is the finding.
   A different answer is a FINDING (C11: to the run coordinator before the next step), not a STOP, unless the call refused: a refusal = STOP, recorded with its code.
6. `gst_by_counter_ledger` for 1 to 30 November 2025. MARKS after. EXPECT `state` complete; `GS1 Rent` vouchers 53 (V1, V8, V10 and the 50 copies), `debit_side_tax` cgst -45900, state_tax -45900, `credit_side_tax` cgst 180, state_tax 180; `GS1 Professional Fees` vouchers 1, igst -3600; `set_aside` as the sum of step 5. A refusal on size = record its code and STOP (it measures the tool's year cost); any other refusal = STOP.
7. `purchase_register` for 1 to 30 November 2025 (offset 0, default limit), then MARKS. RECORD its `total` and the side lists' totals; no EXPECT beyond `state` complete (it is the comparison).
8. BY EYE, from Tally's own screens (Display > Account Books > Ledger, for November 2025): the debit and credit totals of `Input CGST`, `Input SGST`, `Input IGST`, `GS1 Rent` and `GS1 Professional Fees`, each as a screenshot. EXPECT the GST of the vouchers attributed to `GS1 Rent` and `GS1 Professional Fees` in step 6 to be the tax those vouchers carry on these screens.
9. STATE. EXPECT the marks of step 4 unchanged.
10. LAB-COMMON C10.

## What the book holds after each step
After step 2: four new ledgers. After steps 3 and 4: the vouchers above (61 or 63). Nothing after step 4 changes the book. No restore is made; the book keeps these vouchers (they are dated November 2025, a month the committed captures of this book do not read).

## Evidence
Every tool answer whole (the assistant's files), every operator answer, the relay's recording of every request and response for steps 5 to 7 with sizes and sha256, screenshots listed by name, size and sha256 (never pushed), the notes file. The recording of steps 5 to 7 becomes the fixtures of pull request 1867; it carries THE COMPANY's synthetic names only.
