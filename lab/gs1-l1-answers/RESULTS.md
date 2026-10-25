# Results: GS1-L1 v3.1 on TallyPrime 7.2, BRIDGE GST LEDGER LAB (25-26 Oct 2026). Steps 0 to 10 done, every expectation met; the optional tail (11 to 14) deferred; C10 done

Steps 0 to 5 (part) ran in one lab session on 25 Oct; the rest of step 5 and steps 6 to 10 ran in a second lab session the same night, after an account hand-over. Two recorder findings stopped step 6 before the third recorder version recorded it cleanly; both are below.

The item's files are at `lab/gs1-l1` @ ee573991 (`SHA256.txt` 5/5 OK; `check-request-ids.py`: `census: ok 2, skipped 0, FAIL 0`). TallyPrime Release 7.2 (Latest), Silver Rental (not Educational), `tally.exe` 1.1.7.2, one process on port 9001. The owner said in the lab's chat that the lab keys the OWNER steps itself; every keyed act below was done on Tally's own screens by the lab, one act at a time, with a screenshot (kept on the lab PC) and a MARKS read after each master and voucher.

**Build (steps 6 to 8).**
- PR #1867 head 796e94f1, CI run 40167319272, artifact 12589749758: zip 61,285,927 B, digest 1f9e76d0ca5672c5a071daf581af9d9a50d18a6f15d11c357d0452a3098fa1d4.
- `bridge-tally.mcpb` dd67d3fd… (equal to `archive_sha256`).
- `bridge_mcp.exe` 33,768,448 B, sha256 **bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46** (equal to `binary_sha256`): the pinned hash.
- Staged only; nothing installed.

**Recorder.** The original recorder (5f36ad42…) was not found on the lab PC or in the repository and is treated as lost. Its replacement recorded steps 6 to 8 in its third version (see "The recorder: three versions" below).

**Route.** As before, the build ran over stdio from a lab client: a copy of the 72-REG client with the build path, Tally host 127.0.0.1, port 9011 (the recorder), its own data folder, import and writes false and `BRIDGE_TERMS_ACCEPTED=true`. The client checks the exe's sha256 before it starts it (sha256 of the client 4899789c78765ae2f7cb7ae12ef3006ed3d31995e872cef0195fad3a90726c1e; it stays on the lab PC because it names local paths). `tools/list` on the build: `bridge-tally` 0.6.0, 28 tools; it sent nothing to Tally.

**Company number.** The company's folder number appears in the tool answers and in the recorded company-list bodies; it is committed as is, by the run's review: it is a synthetic BRIDGE lab company's number and the replay needs byte-exact bodies. No other company's name or number is committed.

**Terms.** `BRIDGE_TERMS_ACCEPTED=true` for the stdio runs, with the owner as the person accepting the Terms for the lab.

## THE COMPANY
BRIDGE GST LEDGER LAB, GUID **aae372c5-a894-491c-8843-5532e43d2d2f**, FY and books from 1-4-2025. The company number is recorded on the lab PC only.

## Steps done
| Step | What | Marks after (ALTVCHID / ALTMSTID) |
|---|---|---|
| 0 | No company loaded. One tally.exe on 9001, /status 200. About read 7.2 (Latest), Silver Rental. No company named BRIDGE GST LEDGER LAB existed. Build hash as above | (no company) |
| 1 | Company created, financial year and books from 1-4-2025, no password, other fields as offered. Its Features screen opened by itself (see below), accepted unchanged. STATE: one COMPANY row, GUID above; LEDGERS: Cash, Profit & Loss A/c | none / 207 |
| 2 | 16 ledgers, one at a time (ALTMSTID +1 each, 208 to 223). Then unit Nos (224) and stock item GS1 Item (225), because inventory read Yes. The unit is not in the sheet's list but the stock item needs one | none / 225 |
| 3 | STATE: exactly 18 ledgers (the 16 plus Cash and Profit & Loss A/c), parents as the sheet. Backup **GS0** (C5) into a new empty folder: `TDBK1800_<company number>.001`, 25,966 B, sha256 **3c804b2a5cd72a890bb61fd21338236e78dbd118a6c9a5a85792ba05f9e0bdde**, 16:28 IST; one file after. MARKS unchanged | none / 225 |
| 4 | V1, V4, V5, V6, V7, V4b, V8, V9, V10, V11, V13, V14, then V2 and V3 (inventory Yes). ALTVCHID +1 each | 14 / 225 |
| 5 (part) | Duplicates of V1 by Alt+2 on V1 in the Day Book, only the date changed: **40 of 50 done**, five a day on 18, 19, 20, 21, 22, 23, 24 and 25 Nov. MARKS after every fifth: 19, 24, 29, 34, 39, 44, 49, 54. LEDGERS read at the stop equals step 3 | **54 / 225** |
| 5 (rest) | Second session: the company loaded alone (filter `LEDGER`, one row); MARKS 54/225 before any key, equal to the hand-over. Copies 41 to 50 the same way (Day Book for 3 Nov, Alt+2 on V1, F2 to the date, Ctrl+A), with MARKS after **every** copy: Purchase 47 to 51 on 26 Nov (ALTVCHID 55 to 59), Purchase 52 to 56 on 27 Nov (60 to 64). No dialog. STATE: Day Book 26 and 27 Nov each show five Purchase vouchers of 11,800.00; LEDGERS equal to step 3 (name, parent, bill-wise, ALTERID, GUID: no difference). ALTVCHID 64 = step 4's 14 + 50 | **64 / 225** |
| 6 | 14 tool calls, one per voucher day, through recorder run-3; every answer `state` complete; MARKS after each unchanged. See "Step 6" | 64 / 225 |
| 7 | One call for 1 to 30 Nov; complete; see "Step 7" | 64 / 225 |
| 8 | `purchase_register` 1 to 30 Nov, offset 0; see "Step 8" | 64 / 225 |
| 9 | Five ledger screens by eye, November 2025; see "Step 9" | (no read) |
| 10 | STATE: MARKS equal to step 5; LEDGERS equal to step 3 | 64 / 225 |
| 11-14 | **Not run (deferred)** by the run's review: step 12 writes GST status into this book, which the next purchase item uses, and its acts are the owner's | |
| C10 | Company > Shut, one Enter; Select Company with no company loaded; MARKS: 0 COMPANY rows; one tally.exe | (no company) |

## Observations to carry into the item's findings
1. **Features (step 1).** On TallyPrime 7.2 the Features screen opened by itself after Create Company, with "Enable Goods and Services Tax (GST)" already **Yes** and "Set/Alter Company GST Rate and Other Details" No. No registration details were asked, and none were entered (no State, no GSTIN). Maintain Inventory **Yes**, Integrate Accounts with Inventory Yes. It was accepted unchanged.
2. **Masters.**
   - Tally 7.2 names the GST tax types IGST, CGST, SGST/UTGST and Cess; the sheet's Central, State and Integrated Tax were keyed as CGST, SGST/UTGST and IGST.
   - Purchase, expense and asset ledgers offered "GST applicability: Applicable", with HSN and rate "As per Company/Group"; these were left as offered.
   - Parties offered bill-by-bill Yes and registration Regular, with no GSTIN.
3. **Voucher view.**
   - 7.2's "As Voucher" is reached by Ctrl+H (Change Mode), since Ctrl+V did nothing. Payment uses "Double Entry".
   - In Purchase As Voucher, Tally puts the party (Credit) line first.
   - Purchase, Debit Note and Credit Note show Receipt/Dispatch Details and Party Details sub-forms, accepted blank or as offered.
4. **Balancing fill.**
   - Tally pre-fills every new line with the remaining balance; in journals and payments it also pre-fills the last ledger used.
   - Every pre-filled amount was overtyped with the sheet's figure.
   - Tally never computed a tax amount. In V2 and V3, the item-invoice tax lines opened with an **empty** amount.
5. **Bill-wise.** Every party line was given a New Ref named after the voucher (V4, V5, V6, V7, V4b, V8, V14, V2, V3); V1's is "1". Tally's default was sometimes Agst Ref or On Account. The Alt+2 copies of V1 carry V1's New Ref "1".
6. **Debit Note V8 and Credit Note V14.** "Provide GST details" defaulted to **Yes**, and the Additional Details screen's "Reason for Issuing Note" defaulted to Not Applicable; both were kept.
7. **Negative Cash (V11 and V13).**
   - Tally showed the dialog "Warning / Negative Cash! / ₹ (-)200.00 / Press any key to continue" on V11 (Payment, To Cash 200), and "₹ (-)430.00" on V13 (To Cash 230).
   - The item stopped at the first one. On the run's review's decision, each was dismissed with one key and the voucher saved as keyed.
8. **Corrections before saving** (nothing saved wrong):
   - V13's second line first went in as To (credit) Output SGST. It was changed to By before saving.
   - V3's first attempt put the freight 100 into the line's Rate (%) column, so Tally computed 5,000. That voucher was abandoned unsaved with Ctrl+Q (no prompt, marks unchanged) and keyed again with the rate blank.
9. **Session date.** Tally's current date moved with the voucher dates; the Gateway read 25-Nov-2025 at the stop. It is a session value, not data.

## Vouchers in the book
| Voucher | Date | Type, number | Lines |
|---|---|---|---|
| V1 | 3 Nov | Purchase 1 | Cr SYN GS1 Supplier 11,800; Dr GS1 Rent 10,000; Dr Input CGST 900; Dr Input SGST 900 |
| V4 | 4 Nov | Journal 1 | Dr GS1 Professional Fees 20,000; Dr Input IGST 3,600; Cr party 23,600 |
| V5 | 5 Nov | Purchase 2 | Cr party 47,200; Dr GS1 Computer 40,000; Dr Input CGST 3,600; Dr Input SGST 3,600 |
| V6 | 6 Nov | Purchase 3 | Cr party 2,360; Dr GS1 Freight Direct 2,000; Dr Input CGST 180; Dr Input SGST 180 |
| V7 | 7 Nov | Purchase 4 | Cr party 11,800; Dr GS1 Rent 5,000; Dr GS1 Professional Fees 5,000; Dr Input CGST 900; Dr Input SGST 900 |
| V4b | 8 Nov | Journal 2 | Dr GS1 Professional Fees 10,000; Dr Input IGST 1,800; Cr GS1 TDS Payable 1,000; Cr party 10,800 |
| V8 | 10 Nov | Debit Note 1 | Dr party 1,180; Cr GS1 Rent 1,000; Cr Input CGST 90; Cr Input SGST 90 |
| V9 | 11 Nov | Journal 3 | Dr Output CGST 500; Cr Input CGST 500 |
| V10 | 12 Nov | Journal 4 | Dr GS1 Rent 180; Cr Input CGST 90; Cr Input SGST 90 |
| V11 | 13 Nov | Payment 1 | Dr Output CGST 100; Dr Output SGST 100; Cr Cash 200 |
| V2 | 14 Nov | Purchase 5 (Item Invoice) | GS1 Item 1 Nos @ 5,000 (Purchase - Goods); Input CGST 450; Input SGST 450; party Cr 5,900 |
| V13 | 15 Nov | Payment 2 | Dr Output CGST 90; Dr Output SGST 90; Dr GS1 Interest on GST 50; Cr Cash 230 |
| V14 | 16 Nov | Credit Note 1 | Cr SYN GS1 Customer 1,180; Dr GS1 Discount Allowed 1,000; Dr Output CGST 90; Dr Output SGST 90 |
| V3 | 17 Nov | Purchase 6 (Item Invoice) | GS1 Item 1 Nos @ 5,000; GS1 Freight Inward 100; Input CGST 450; Input SGST 450; party Cr 6,000 |
| copies of V1 | 18 to 25 Nov | Purchase 7 to 46 | as V1; five a day, 40 in all |
| copies of V1 | 26 and 27 Nov | Purchase 47 to 56 | as V1; five a day, 10 in all (second session) |

## The recorder: three versions
The sheet's rule (a recorder refusal or a tool refusal is a stop) stopped step 6 twice before the third version recorded it. The recorder was never changed on the lab PC; each version came from its branch at the commit the run's review named, and every check below was made before it was started. Each stop's whole recording is committed under `recorder/`.

| Version | Files check | Self-test | Result |
|---|---|---|---|
| 5b6fbfca (script 00bd8411…) | SHA256.txt 19/19 | `ok (4 forwarded, 40 refused, none of the refused reached the target)`, exit 0 | **Stop 1** at step 6 call 1: the tool refused `company_collection_invalid`. The recorder forwarded `GET /status` with `Content-Type: text/xml; charset=utf-16`, so Tally answered `/status` in UTF-16 (102 B, `Unicode: Yes`) where a plain request gets 51 B ASCII; the build's egress trail: status `response_encoding_invalid`. Recording `recorder/run-1-status-header/` |
| a1e8e2e2 (3a34b885…) | 19/19 | `ok (7 forwarded, 40 refused, …)`, exit 0 | `/status` through the recorder = the direct answer (51 B, 65541597…). **Stop 2** at call 1: request 53 `Bridge Agent Vouchers` (3,126 B) refused `not_a_known_bridge_read`: the build's voucher window fetches `REFERENCEDATE`, which no template had (the only difference from the nearest template besides the company name and dates). It never reached Tally. Recording `recorder/run-2-referencedate/` |
| e5dffc2b (dfc84c86…) | 21/21 | `ok (10 forwarded, 41 refused, …)`, exit 0 | `/status` equality again; steps 6 to 8 recorded with **no refusal**: 1,657 requests (713 `/status`, 328 company list, 256 book extent, 128 ledger listings, 64 groups, 64 currencies, 36 voucher windows, 36 company marks, 32 voucher censuses), 4,972 files, 30,888,025 bytes. `recorder/run-3/manifest.tsv` and `recorder/run-3-SHA256.txt` (every file); the message bodies stay on the lab PC as hashed, for the fixtures commit |

The refused first calls of the two stops are kept as `tool/s06-01-20251103.json` and `tool/s06r2-01-20251103.json`; the recorded run's calls are `tool/s06r3-*`, `tool/s07r3-*` and `tool/s08r3-*`, each with its arguments file; `tool/tool-log.txt` has every call's bytes and sha256; `egress/agent-egress.jsonl` is the build's own receipt log for every call. MARKS were 64/225 after every call, refused or not.

## Step 6 (run-3): 14 of 14 as expected
Every answer `state` complete, `evidence.state` complete, one voucher observed, MARKS unchanged. Tally stored the state head as **`sgst_utgst`**.

| Day | Voucher | Expected | Seen |
|---|---|---|---|
| 3 Nov | V1 | `GS1 Rent` 1 voucher, debit cgst -900, state -900; set_aside empty | the same |
| 4 Nov | V4 | `GS1 Professional Fees` 1, igst -3600 | the same |
| 5 Nov | V5 | `other_ledgers_present` 1, the voucher's tax | 1, cgst -3600, state -3600; no counter ledger |
| 6 Nov | V6 | `other_ledgers_present` 1 | 1, cgst -180, state -180 |
| 7 Nov | V7 | `several_counter_ledgers` 1, cgst -900, state -900 | the same |
| 8 Nov | V4b | `duty_ledger_without_gst_type` 1, igst -1800 | the same |
| 10 Nov | V8 | `GS1 Rent` 1, credit cgst 90, state 90 | the same |
| 11 Nov | V9 | `gst_on_both_sides` 1 | 1 (debit cgst -500, credit cgst 500) |
| 12 Nov | V10 | `no_creditor_party` 1, credit cgst 90, state 90 | the same |
| 13 Nov | V11 | `no_creditor_party` 1, debit cgst -100, state -100 | the same |
| 15 Nov | V13 | `no_creditor_party` 1, debit -90/-90; nothing attributed to `GS1 Interest on GST` | the same; `counter_ledgers` empty |
| 16 Nov | V14 | `class_not_admitted` 1, debit -90/-90; nothing attributed to `GS1 Discount Allowed` | the same; `counter_ledgers` empty |
| 14 Nov | V2 | RECORD | **`Purchase - Goods` 1 voucher, debit cgst -450, state -450**: the purchase ledger was among the ledger entries |
| 17 Nov | V3 | RECORD | **`several_counter_ledgers` 1, debit -450/-450** |

No voucher fell in `unrecognised_head` or `head_absent`: the duty heads Tally 7.2 stores for ledgers typed on its screen are the ones the tool recognises.

## Step 7: the month, as expected
1 to 30 Nov, complete, 64 vouchers observed, none without a GST entry.
- `counter_ledgers`: `GS1 Rent` 52 vouchers, debit cgst -45900, state -45900, credit cgst 90, state 90; `GS1 Professional Fees` 1, igst -3600; `Purchase - Goods` 1, -450/-450 (V2).
- `set_aside` equals the sum of step 6: `other_ledgers_present` 2 (-3780/-3780), `several_counter_ledgers` 2 (-1350/-1350), `duty_ledger_without_gst_type` 1 (igst -1800), `gst_on_both_sides` 1, `no_creditor_party` 3 (debit -190/-190, credit 90/90), `class_not_admitted` 1 (-90/-90).
- The answer is 10,386 bytes; no size refusal.

## Step 8: purchase register (the comparison)
`purchase_register` 1 to 30 Nov, offset 0: `state` complete, `total` 57 (register classes Purchase and Debit Note: 56 purchases and V8), `next_offset` none; `other_voucher_types_touching_duties_taxes` 7 (four journals, two payments, the credit note); `purchase_vouchers_without_duties_taxes_entry` 0; `unclassified_voucher_type` 0; `vouchers_with_unplaced_ledgers` 0; 64 vouchers observed.

## Step 9: Tally's ledger screens, by eye (November 2025; screenshots on the lab PC)
| Ledger | Debit | Credit | What it holds |
|---|---|---|---|
| Input CGST | 51,480.00 | 680.00 | 900 on V1 and on each of the 50 copies, 3,600 (V5), 180 (V6), 900 (V7), 450 (V2), 450 (V3); credits 90 (V8), 500 (V9), 90 (V10) |
| Input SGST | 51,480.00 | 180.00 | the same debits; credits 90 (V8), 90 (V10) |
| Input IGST | 5,400.00 | | 3,600 (V4, Journal 1), 1,800 (V4b, Journal 2) |
| GS1 Rent | 5,15,180.00 | 1,000.00 | 10,000 on V1 and each copy, 5,000 (V7), 180 (V10); credit 1,000 (V8) |
| GS1 Professional Fees | 35,000.00 | | 20,000 (V4), 5,000 (V7), 10,000 (V4b) |

The tax step 7 attributes to `GS1 Rent` (51 vouchers at 900/900 = 45,900 each side, and V8's 90/90 credit) and to `GS1 Professional Fees` (V4's IGST 3,600) is the tax those vouchers carry on these screens. V7, V10 and V4b appear on the screens and are set aside by the tool, as step 6 says.

## Step 10 and C10
STATE: MARKS ALTVCHID 64, ALTMSTID 225 (= step 5); LEDGERS 18 rows equal to step 3. The tail was not run, so C10 followed: the company was shut (Company > Shut, one Enter); Tally at Select Company with no company loaded; MARKS returned no COMPANY row; one tally.exe. THE COMPANY is kept, shut, with its 16 ledgers, unit, stock item and 64 vouchers.

## In this commit
- `answers/`: every MARKS and LEDGERS answer of steps 1 to 10 and C10 with headers; `send-log.txt` (every operator read: time, bytes, sha256).
- `tool/`: every tool call's arguments and whole answer (steps 6 to 8, the two refused calls, `tools/list`), `tool-log.txt`.
- `egress/agent-egress.jsonl`: the build's receipt log.
- `recorder/`: the three self-test outputs; `run-1-status-header/` and `run-2-referencedate/` in full with their sha256 lists; `run-3/manifest.tsv` and `run-3-SHA256.txt`.
- `status-gate/`: `/status` direct and through the recorder before steps 6 of run-2 and run-3, and step 0's direct answer (all 51 B, 65541597…).
- Screenshots, the About capture, the data-folder listing, the GS0 backup, run-3's message bodies and the stdio client stay on the lab PC.
