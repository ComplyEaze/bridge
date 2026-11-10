# Row 683-RO: with row 683-LD's two vouchers keyed again, the opening export at 1 April did not change the Trial Balance to 1 April: tb3x was byte-equal to the control tb3c, itself byte-equal to row 683-VD's tb3w (two ledgers MIXED)

Run on 10 Nov 2026, 06:33:17Z to 06:49:35Z (12:03 to 12:19 IST), on TallyPrime Silver (the Gateway on 9001; licensed, no Educational mode shown; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/683-ro-sheet` (214ac752), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own. Raw reads, one at a time through `run.py`; two vouchers keyed by hand; one backup, restored at the end; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Every read was answered, each with STATUS 1, within a second (0.4 to 0.7 s); no dialog came up after any read.

This sitting ran after row 2464-RB, on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The restore returned the book to the marks it was found with (11/214).

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214: SAME MARKS with row 683-LD's m2. EXPECT met.
- **The controls (step 2), each as EXPECT:** tb3a (current date 1-Apr-2025) byte-equal to row 683-TW's tb3 (14,140 B), SETS SAME, six MIXED and two PLAIN; mv0 byte-equal to row 683-TW's mvy (59,524 B), CAPTURE SAME, vouchers movement 11, capture 11.
- **BK, mb, mc:** BK taken; mb SAME MARKS with m0; mc SAME MARKS with mb, with no key pressed between.
- **The write, as row 683-VD's:** K1 saved as Sales No. 4, 30-Apr-25: `BRIDGE USD Customer` Dr, `BRIDGE Export Sales` $100.00 at 88 = 8,800.00 rupees, New Ref LD1. K2 saved as Receipt No. 4, 30-Apr-25: `BRIDGE INR Customer` Cr 2,000.00 Agst Ref R1 (R1 pending 3,000.00 Dr), `Cash` Dr 2,000.00 (offered by Tally, not typed). Each number was the one expected (4 and 4). No gain or loss line on either. No "Spelling Error" box (every name was typed in keystroke chunks). Neither save asked "Accept ?".
- **m1:** MARKS DIFFER with mc (exit 2), ALTVCHID 11 to 13, ALTMSTID 214 (unchanged); SAME MARKS with row 683-LD's m1. Both EXPECTs met.
- **tb3c (the control, current date 30-Apr-2025): byte-equal to row 683-VD's tb3w** (`cmpinfo.py` exit 0; 13,740 B, sha256 `f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb`); SETS SAME. `BRIDGE Import Purchases` and `BRIDGE Lab Bank` MIXED, the other six PLAIN. EXPECT met.
- **x1 (the opening export at 1 April, the one variable): byte-equal to row 683-LD's x1** (`cmpinfo.py` exit 0; 11,556 B); 8 LEDGER rows, each opening 0.00. EXPECT met.
- **tb3x (the question): byte-equal to tb3c** (`cmpinfo.py` exit 0; SETS SAME): two MIXED, six PLAIN. The Tally window captured at once after the answer (`tb3x-window`): the Gateway, no dialog. Against row 683-LD's tb3 (13,528 B, every ledger PLAIN): `cmpinfo.py` exit 2, differing outside CMPINFO; `tb.py --sets` SETS DIFFER, `BRIDGE Import Purchases` and `BRIDGE Lab Bank` PLAIN to MIXED. Every rupee figure is equal. Of 40 fields, 8 differ (`evidence/tb3x-fields-vs-683ld.txt`), all in those two ledgers and the same 8 as row 683-VD's tb3w list: for example `BRIDGE Lab Bank` TBALCLOSING `-$900.00 @ ₹ 88/$ = -₹ 79200.00` here against `-79200.00` there, and `BRIDGE Import Purchases` CREDITTOTALS EMPTY here against `0.00` there.
- **6b:** not run. tb3x is as tb3c by the sheet's test (SETS SAME and `cmpinfo.py` exit 0).
- **m1r:** SAME MARKS with m1 (13/214): the reads wrote nothing.
- **The restore (step 8):** by C6, the two expected prompts only. **m2:** SAME MARKS with m0 (11/214). **mv2:** CAPTURE SAME, 11 and 11, and byte-equal to mv0. So the book is as BK holds it.
- **Step 9:** current date Tuesday, 1-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 1-Apr-25: as at step 0. EXPECT met.

**By the sheet's Decides:** this is the case "`tb3c` as row 683-VD's `tb3w`, `tb3x` as `tb3c`". In the sheet's words: "the opening export does not change this answer. Row 683-LD's all-plain `tb3` is then not explained by `x1`."
- The sheet's other differences between the two sittings are recorded from both rows' notes, and none of them is taken as the cause:
  - the sends before the write: row 683-LD's `x0` and its Trial Balance to 30 April, where rows 683-VD and 683-RO sent `tb3a`;
  - the keying: row 683-LD's K1 met a "Spelling Error" box and cleared it; rows 683-VD's and 683-RO's met none;
  - the time from K2's save to the Trial Balance: about 2 minutes in row 683-LD, about 35 s here (K2 saved at 06:44:48Z, tb3c sent at 06:45:23Z).
- By the sheet, each candidate is its own row.
- **Timing, recorded and not set** (`evidence/gaps.txt`): tb3c was sent 13.1 s after m1 answered, x1 7.6 s after tb3c, tb3x 9.4 s after x1. Row 683-LD's tb3 came about 58 s after its x1.

**The sheet's estimates:** the controls, the write (Sales No. 4 and Receipt No. 4) and m1 against row 683-LD's m1, each as estimated; tb3c byte-equal to row 683-VD's tb3w, as estimated; x1 byte-equal to row 683-LD's x1, as estimated; the restore as estimated. tb3x had no estimate: it was the question.

## Deviations
1. **The start-up company was shut first.** Its own book loaded by itself, as the install is set to, and was shut by C9 before step 0 (the K: Company label, Shut, Enter on its one row, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
2. **One screen was not captured.** For K2, `v` (Vouchers) and F6 were pressed one after the other, so the voucher `v` opened before F6 was not captured. Receipt No. 4 was captured after F6 (`k2-1-receipt-no4`).

## Step 0: start, check and load
- Sheet files: all 18 equal to `SHA256.txt` at 214ac752 (sizes too). `.gitattributes`, `LAB-COMMON.md`, `run.py`, `lm.py`, `tb.py`, `cmpinfo.py`, `M-marks.xml`, the tb3 and mvy requests are byte-equal to row 683-VD's; the x1 request is byte-equal to row 683-LD's. `a-vdtb3w` is byte-equal to row 683-VD's tb3w (and its tb3v and tb3w2); `a-twtb3` to row 683-TW's tb3 and row 683-VD's tb3a; `a-twmvy` to row 683-TW's mvy and row 683-VD's mv2; `a-ldtb3`, `a-ldx1`, `a-ldm1` and `a-ldm2` to row 683-LD's tb3, x1, m1 and m2.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 06:33:17Z, from its desktop shortcut, after tasklist showed none.
- C2 at 06:34:24Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE`, space, `FOREXGROU` typed in the name filter, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25. The PC's date: 10 Nov 2026 (IST).

## Steps 1 to 3: m0, the controls, BK
- m0, tb3a and mv0 as in the Result.
- **BK** (C5), to the new folder `C:\lab\bk-683ro` (made empty before the start): the Data label, Backup & Restore, Backup; Backspace to the path; Specify Path (Up 3 times), the path typed in two keystroke chunks, its row taken; BRIDGE FOREXGROUP LAB alone, End of List (Up 3 times); "Accept ?" captured (`bk-accept`) and answered y; no other message; the Gateway again. The folder then held one file: `TDBK1800_<company number>.001`, 37,618 B, sha256 `773f7a1adab8b6aa48480c8dd2b725ab8f5ee8644c93fd4cc99e458d3e582f9e`, written at 12:06 IST; the Restore list was to show 10-Nov-26 (12:06 pm), and did.
- mb and mc: SAME MARKS, as in the Result.

## Step 4: the write, word for word as the screen showed it
`evidence/notes.txt` holds every line below with its time; each capture is named in `SCREENSHOTS.txt`.

**K1, Sales.**
- The Gateway, `v` (Vouchers): Accounting Voucher Creation opened as Payment No. 3. F8: Sales, **No. 4** offered (`k1-1-sales-no4`).
- F2: the "Change Voucher Date" box, Voucher Date offered `2-4-2025`; `30-4-2025` typed, Enter (`k1-2-date-box`).
- Party A/c name: `BRIDGE`, space, `USD`, space, `Customer` typed as keystrokes; one row, `BRIDGE USD Customer`, highlighted (`k1-3-party-typed`); Enter. No "Spelling Error" box.
- "Dispatch Details" box (Delivery Note No(s), Dispatch Doc No., Dispatched through, Destination, Carrier Name/Agent, Bill of Lading/LR-RR No. and Date, Motor Vehicle No.; all empty): Enter on each, all left empty (`k1-4-dispatch-details`).
- "Party Details" box (Buyer (Bill to) BRIDGE USD Customer, Mailing Name BRIDGE USD Customer, Address empty, State Not Applicable, Country India, GST Registration type Regular (list: Unknown, Composition, Regular, Unregistered/Consumer), GSTIN/UIN empty, Place of Supply Not Applicable): Enter on each, as offered (`k1-5-party-details`). The voucher then showed **30-Apr-25 Wednesday**, the party's Current balance 8,79,12,000.00 Dr (`k1-6-date-before-amount`).
- Particulars: `BRIDGE`, space, `Export`, space, `Sales` typed as keystrokes, the one row highlighted, Enter. No Inventory Allocations box.
- Amount: `$100` typed, Enter. The **Forex Rate of Exchange** box: offered `$100.00 @ (rupee) 88/$`, Value in (rupee) `8,800.00` (`k1-7-forex-box`). Enter on Forex Amount; `88` typed over the rate, Enter; Value 8,800.00 as offered, Enter. The line `BRIDGE Export Sales $100.00`; total `$100.00 @ (rupee) 88/$ = (rupee) 8,800.00` (`k1-8-line-100-at-88`).
- Enter on the empty second line, then Enter on the total: **Bill-wise Details for : BRIDGE USD Customer**, "Up to: (rupee) 8,800.00 Dr"; columns Type of Ref, Name, Due Date or Credit Days (wef: 30-Apr-25), Amount, Dr/Cr; Method of Adj. list Advance, Agst Ref, New Ref (highlighted), On Account (`k1-9-billwise`). New Ref, Enter; Name offered `4`, `LD1` typed over it, Enter; Due Date empty, Enter; Amount $100.00 as offered, Enter; Dr, Enter. One line: New Ref, LD1, $100.00 Dr.
- Narration left empty. Before Ctrl+A (`k1-10-before-accept`): no gain or loss line. Ctrl+A: saved, no prompt; a blank Sales No. 5 (`k1-11-after-save`). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; date of last entry 30-Apr-25 (`k1-12-gateway-after`).

**K2, Receipt.**
- The Gateway, a click on Tally's title bar, `v`, F6 (deviation 2): Receipt, **No. 4** offered; the To line empty, the ledger list open (`k2-1-receipt-no4`).
- To: `BRIDGE`, space, `INR`, space, `Customer` typed as keystrokes, the one row highlighted, Enter. The voucher already showed **30-Apr-25 Wednesday** (no F2 used; `k2-2-date-before-amount`). Cur Bal 3,000.00 Dr.
- Credit: `2000`, Enter. No Forex box.
- **Bill-wise Details for : BRIDGE INR Customer**, "Up to: (rupee) 2,000.00 Cr"; Agst Ref highlighted in Method of Adj., Enter (`k2-3-billwise`). **Pending Bills:** one row, `R1`, Bill Date 1-Apr-25, Due Date 1-Apr-25, Balance 3,000.00 Dr, Final Balance 3,000.00 Dr (`k2-4-pending-r1`); Enter. Amount offered 2,000.00 Cr; `2000` typed over it, Enter; Cr, Enter (`k2-5-r1-amount`). The box closed: Agst Ref R1 2,000.00 Cr; the party's Cur Bal then 1,000.00 Dr.
- Tally then offered **By Cash**, Debit 2,000.00, Cur Bal 5,000.00 Dr (`k2-6-by-cash-offered`): Enter on By; the ledger list with Cash highlighted (the field read Cash), Enter; Debit 2,000.00 as offered, Enter.
- Narration left empty. Before Ctrl+A (`k2-7-before-accept`): To BRIDGE INR Customer Credit 2,000.00, By Cash Debit 2,000.00, totals 2,000.00 each; no gain or loss line. Ctrl+A: saved, no prompt; a blank Receipt No. 5 (`k2-8-after-save`). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; last entry 30-Apr-25 (`k2-9-gateway-after`).

## Steps 5 to 7: the reads
Each request's sha256 was checked against `SHA256.txt` just before its send (`evidence/sends-utc.txt`). From m1 to m1r nothing else was sent and no key was pressed in Tally; only `lm.py`, `tb.py` and `cmpinfo.py` ran between the sends. The whole outputs are below.
- **tb3x:** the Tally window captured at once after the answer (`tb3x-window`, 06:45:44Z): the Gateway, current date Wednesday, 30-Apr-2025, last entry 30-Apr-25, no dialog.
- Calls beyond the sheet's are kept beside the sheet's: `tb.py` on row 683-LD's tb3 (`tb-ldtb3.txt`), from which the field list was made, as row 683-VD's `tb3w-fields-vs-683ld.txt` was; and the gaps between sends (`gaps.txt`), from `sends-utc.txt`.

## Step 8: the restore (C6)
- Before the shut: the BK re-hashed equal to the one recorded, its folder holding that file alone; `restore-plan.txt` written; the data folder listed (24 entries). The plan and listings stay on the lab PC.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt (`c6-1-after-shut`).
- The Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`; Specify Path (Up 3 times), `C:\lab\bk-683ro` typed in two keystroke chunks. The list showed one row: BRIDGE FOREXGROUP LAB (`<company number>`), 10-Nov-26 (12:06 pm) (`8-restore-list`). Enter on it; End of List, Enter.
- "Accept ?" (`8-restore-accept`): y. Then, word for word: "Company data folder (`<company number>`) already exists." / "Selecting Skip will cancel the Restore process for this company." / "Selecting Replace will overwrite the existing folder.", buttons "R: Replace" and "S: Skip" (`8-restore-replace`): r. Select Company followed, no other message.
- The data folder after the restore: after=17, before=24 entries. Gone: `CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`, `TUPDATE.TSF` (the same seven as rows 683-LD's and 683-VD's restores). None new.
- Loaded alone: `BRIDGE`, space, `FOREXGROU` typed, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`8-select`), Enter, no prompt. The Gateway (`8-gateway`): current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25.
- m2 and mv2 as in the Result.

## Steps 9 and 10: the date, and close (C10)
- Step 9 (`9-gateway`): as in the Result; nothing was pressed to change the date or the period.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 06:49:19Z).
- C2 at 06:49:20Z, after the last request: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B). No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; at 06:49:35Z no `tally.exe` ran and nothing listened on 9001. The three Bridge processes were not ended.

## Files
- `answers/`: this row's answers that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256). The sheet's copies of other rows' answers are not repeated here.
- `evidence/`: notes, the outputs of `lm.py`, `tb.py` and `cmpinfo.py`, the field list, the gaps between sends, the start-up check's hits. Process and port listings, the backup folder and data folder listings, and the restore plan stay on the lab PC.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## lm.py, tb.py and cmpinfo.py outputs, whole

### m0: lm.py
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### m0: lm.py --same-row against row 683-LD's m2
```
a-ldm2-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
exit 0
```

### tb3a: cmp and sha256 against row 683-TW's tb3
```
1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7 *a-tb3a-T-tb3-20250401-20250401.xml
1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7 *a-twtb3-T-tb3-20250401-20250401.xml
tb3a against a-twtb3: BYTE-EQUAL
```

### tb3a: tb.py --sets against row 683-TW's tb3
```
a-twtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3a-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-twtb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb3a-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### mv0: cmp and sha256 against row 683-TW's mvy
```
56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7 *a-mv0-V-mvy-20250401-20260331.xml
56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7 *a-twmvy-V-mvy-20250401-20260331.xml
mv0 against a-twmvy: BYTE-EQUAL
```

### mv0: lm.py --capture against row 683-TW's mvy
```
a-mv0-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
a-twmvy-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  1bb48be5-baba-45b2-90d8-7593244915a9-00000001: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000002: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000003: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000004: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000005: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000006: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000007: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000008: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000009: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000a: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000b: SAME (2 entries)
vouchers: movement 11, capture 11
CAPTURE SAME
exit 0
```

### mb: lm.py
```
a-mb-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### mb: lm.py --same-row against m0
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-mb-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
exit 0
```

### mc: lm.py
```
a-mc-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### mc: lm.py --same-row against mb
```
a-mb-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-mc-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
exit 0
```

### m1: lm.py
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
exit 0
```

### m1: lm.py --same-row against mc
```
a-mc-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
MARKS DIFFER: ('1bb48be5-baba-45b2-90d8-7593244915a9', '11', '214') -> ('1bb48be5-baba-45b2-90d8-7593244915a9', '13', '214')
exit 2
```

### m1: lm.py --same-row against row 683-LD's m1
```
a-ldm1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
SAME MARKS
exit 0
```

### tb3c: cmpinfo.py against row 683-VD's tb3w
```
a-vdtb3w-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-tb3c-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
BYTE-EQUAL
exit 0
```

### tb3c: tb.py
```
a-tb3c-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
  LEDGER rows: 8
  'BRIDGE Export Sales' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '86014641.25' PLAIN
    TBALCLOSING '86014641.25' PLAIN
  'BRIDGE INR Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-5000.00' PLAIN
    CREDITTOTALS '2000.00' PLAIN
    TBALCLOSING '-3000.00' PLAIN
  'BRIDGE Import Purchases' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-$200.00 @ ₹ 86/$ = -₹ 17400.00' COMPOSITE
    CREDITTOTALS '' EMPTY
    TBALCLOSING '-$200.00 @ ₹ 86/$ = -₹ 17400.00' COMPOSITE
  'BRIDGE Lab Bank' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-$1100.00 @ ₹ 87/$ = -₹ 96700.00' COMPOSITE
    CREDITTOTALS '$200.00 @ ₹ 87/$ = ₹ 17500.00' COMPOSITE
    TBALCLOSING '-$900.00 @ ₹ 88/$ = -₹ 79200.00' COMPOSITE
  'BRIDGE USD Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-88008700.00' PLAIN
    CREDITTOTALS '96700.00' PLAIN
    TBALCLOSING '-87912000.00' PLAIN
  'BRIDGE USD Supplier' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-17500.00' PLAIN
    CREDITTOTALS '17500.00' PLAIN
    TBALCLOSING '0.00' PLAIN
  'Cash' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-3000.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '-3000.00' PLAIN
  'Profit & Loss A/c' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '' EMPTY
    TBALCLOSING '0.00' PLAIN
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
exit 0
```

### tb3c: tb.py --sets against row 683-VD's tb3w
```
a-vdtb3w-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3c-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-vdtb3w-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3c-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
SETS SAME
exit 0
```

### x1: cmpinfo.py against row 683-LD's x1
```
a-ldx1-X-lx1-20250401.xml: 11556 B sha256 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
a-x1-X-lx1-20250401.xml: 11556 B sha256 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
BYTE-EQUAL
exit 0
```

### x1: lm.py
```
a-x1-X-lx1-20250401.xml: HEADER/STATUS 1
  LEDGER rows: 8
  'BRIDGE Export Sales' OPENINGBALANCE '0.00' reads 0.00
  'BRIDGE INR Customer' OPENINGBALANCE '0.00' reads 0.00
  'BRIDGE Import Purchases' OPENINGBALANCE '0.00' reads 0.00
  'BRIDGE Lab Bank' OPENINGBALANCE '0.00' reads 0.00
  'BRIDGE USD Customer' OPENINGBALANCE '0.00' reads 0.00
  'BRIDGE USD Supplier' OPENINGBALANCE '0.00' reads 0.00
  'Cash' OPENINGBALANCE '0.00' reads 0.00
  'Profit & Loss A/c' OPENINGBALANCE '0.00' reads 0.00
exit 0
```

### tb3x: cmpinfo.py against tb3c
```
a-tb3c-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-tb3x-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
BYTE-EQUAL
exit 0
```

### tb3x: cmpinfo.py against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-tb3x-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
exit 2
```

### tb3x: tb.py
```
a-tb3x-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
  LEDGER rows: 8
  'BRIDGE Export Sales' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '86014641.25' PLAIN
    TBALCLOSING '86014641.25' PLAIN
  'BRIDGE INR Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-5000.00' PLAIN
    CREDITTOTALS '2000.00' PLAIN
    TBALCLOSING '-3000.00' PLAIN
  'BRIDGE Import Purchases' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-$200.00 @ ₹ 86/$ = -₹ 17400.00' COMPOSITE
    CREDITTOTALS '' EMPTY
    TBALCLOSING '-$200.00 @ ₹ 86/$ = -₹ 17400.00' COMPOSITE
  'BRIDGE Lab Bank' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-$1100.00 @ ₹ 87/$ = -₹ 96700.00' COMPOSITE
    CREDITTOTALS '$200.00 @ ₹ 87/$ = ₹ 17500.00' COMPOSITE
    TBALCLOSING '-$900.00 @ ₹ 88/$ = -₹ 79200.00' COMPOSITE
  'BRIDGE USD Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-88008700.00' PLAIN
    CREDITTOTALS '96700.00' PLAIN
    TBALCLOSING '-87912000.00' PLAIN
  'BRIDGE USD Supplier' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-17500.00' PLAIN
    CREDITTOTALS '17500.00' PLAIN
    TBALCLOSING '0.00' PLAIN
  'Cash' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-3000.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '-3000.00' PLAIN
  'Profit & Loss A/c' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '' EMPTY
    TBALCLOSING '0.00' PLAIN
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
exit 0
```

### tb3x: tb.py --sets against tb3c
```
a-tb3c-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3x-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3c-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3x-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb3x: tb.py --sets against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3x-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-ldtb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3x-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
  'BRIDGE Import Purchases': PLAIN -> MIXED
  'BRIDGE Lab Bank': PLAIN -> MIXED
SETS DIFFER
exit 0
```

### row 683-LD's tb3: tb.py (a call beyond the sheet's)
```
a-ldtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
  LEDGER rows: 8
  'BRIDGE Export Sales' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '86014641.25' PLAIN
    TBALCLOSING '86014641.25' PLAIN
  'BRIDGE INR Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-5000.00' PLAIN
    CREDITTOTALS '2000.00' PLAIN
    TBALCLOSING '-3000.00' PLAIN
  'BRIDGE Import Purchases' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-17400.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '-17400.00' PLAIN
  'BRIDGE Lab Bank' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-96700.00' PLAIN
    CREDITTOTALS '17500.00' PLAIN
    TBALCLOSING '-79200.00' PLAIN
  'BRIDGE USD Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-88008700.00' PLAIN
    CREDITTOTALS '96700.00' PLAIN
    TBALCLOSING '-87912000.00' PLAIN
  'BRIDGE USD Supplier' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-17500.00' PLAIN
    CREDITTOTALS '17500.00' PLAIN
    TBALCLOSING '0.00' PLAIN
  'Cash' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-3000.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '-3000.00' PLAIN
  'Profit & Loss A/c' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '' EMPTY
    TBALCLOSING '0.00' PLAIN
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
exit 0
```

### tb3x: the fields that differ from row 683-LD's tb3
```
$ fields-vd.py evidence/tb-ldtb3.txt evidence/tb-tb3x.txt (the tb.py fields that differ, row by row: row 683-LD's tb3 -> tb3x)
('BRIDGE Import Purchases', 'CREDITTOTALS') ('0.00', 'PLAIN') -> ('', 'EMPTY')
('BRIDGE Import Purchases', 'DEBITTOTALS') ('-17400.00', 'PLAIN') -> ('-$200.00 @ ₹ 86/$ = -₹ 17400.00', 'COMPOSITE')
('BRIDGE Import Purchases', 'TBALCLOSING') ('-17400.00', 'PLAIN') -> ('-$200.00 @ ₹ 86/$ = -₹ 17400.00', 'COMPOSITE')
('BRIDGE Import Purchases', 'class') PLAIN -> MIXED
('BRIDGE Lab Bank', 'CREDITTOTALS') ('17500.00', 'PLAIN') -> ('$200.00 @ ₹ 87/$ = ₹ 17500.00', 'COMPOSITE')
('BRIDGE Lab Bank', 'DEBITTOTALS') ('-96700.00', 'PLAIN') -> ('-$1100.00 @ ₹ 87/$ = -₹ 96700.00', 'COMPOSITE')
('BRIDGE Lab Bank', 'TBALCLOSING') ('-79200.00', 'PLAIN') -> ('-$900.00 @ ₹ 88/$ = -₹ 79200.00', 'COMPOSITE')
('BRIDGE Lab Bank', 'class') PLAIN -> MIXED
fields differing: 8 of 40
```

### m1r: lm.py
```
a-m1r-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
exit 0
```

### m1r: lm.py --same-row against m1
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
a-m1r-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
SAME MARKS
exit 0
```

### m2: lm.py
```
a-m2-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### m2: lm.py --same-row against m0
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-m2-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
exit 0
```

### mv2: lm.py --capture against mv0
```
a-mv2-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
a-mv0-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  1bb48be5-baba-45b2-90d8-7593244915a9-00000001: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000002: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000003: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000004: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000005: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000006: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000007: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000008: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-00000009: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000a: SAME (2 entries)
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000b: SAME (2 entries)
vouchers: movement 11, capture 11
CAPTURE SAME
exit 0
```

### mv2: cmp and sha256 against mv0
```
56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7 *a-mv2-V-mvy-20250401-20260331.xml
56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7 *a-mv0-V-mvy-20250401-20260331.xml
mv2 against mv0: BYTE-EQUAL
```

### each send's start and answer, UTC
```
2026-11-10T06:35:00.369Z m0 M-marks.xml (hash equal to the list)
2026-11-10T06:35:01.025Z m0 answered
2026-11-10T06:35:10.326Z tb3a T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T06:35:10.752Z tb3a answered
2026-11-10T06:35:18.821Z mv0 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T06:35:19.268Z mv0 answered
2026-11-10T06:37:09.735Z mb M-marks.xml (hash equal to the list)
2026-11-10T06:37:10.117Z mb answered
2026-11-10T06:37:11.342Z mc M-marks.xml (hash equal to the list)
2026-11-10T06:37:11.748Z mc answered
2026-11-10T06:45:09.905Z m1 M-marks.xml (hash equal to the list)
2026-11-10T06:45:10.469Z m1 answered
2026-11-10T06:45:23.540Z tb3c T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T06:45:23.954Z tb3c answered
2026-11-10T06:45:31.533Z x1 X-lx1-20250401.xml (hash equal to the list)
2026-11-10T06:45:31.914Z x1 answered
2026-11-10T06:45:41.308Z tb3x T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T06:45:41.696Z tb3x answered
2026-11-10T06:45:56.854Z m1r M-marks.xml (hash equal to the list)
2026-11-10T06:45:57.216Z m1r answered
2026-11-10T06:48:40.505Z m2 M-marks.xml (hash equal to the list)
2026-11-10T06:48:41.032Z m2 answered
2026-11-10T06:48:42.419Z mv2 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T06:48:43.012Z mv2 answered
```

### each send's time to answer, and the gap before it
```
$ each send's time to answer and the gap from the previous send's answer to its start, seconds, from evidence/sends-utc.txt
m0: started 06:35:00.369Z
m0: answered in 0.7 s
tb3a: started 06:35:10.326Z; gap after m0 answered: 9.3 s
tb3a: answered in 0.4 s
mv0: started 06:35:18.821Z; gap after tb3a answered: 8.1 s
mv0: answered in 0.4 s
mb: started 06:37:09.735Z; gap after mv0 answered: 110.5 s
mb: answered in 0.4 s
mc: started 06:37:11.342Z; gap after mb answered: 1.2 s
mc: answered in 0.4 s
m1: started 06:45:09.905Z; gap after mc answered: 478.2 s
m1: answered in 0.6 s
tb3c: started 06:45:23.540Z; gap after m1 answered: 13.1 s
tb3c: answered in 0.4 s
x1: started 06:45:31.533Z; gap after tb3c answered: 7.6 s
x1: answered in 0.4 s
tb3x: started 06:45:41.308Z; gap after x1 answered: 9.4 s
tb3x: answered in 0.4 s
m1r: started 06:45:56.854Z; gap after tb3x answered: 15.2 s
m1r: answered in 0.4 s
m2: started 06:48:40.505Z; gap after m1r answered: 163.3 s
m2: answered in 0.5 s
mv2: started 06:48:42.419Z; gap after m2 answered: 1.4 s
mv2: answered in 0.6 s
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every answer (LAB-COMMON C4: not rows); the names with a count other than 0
a-m0-M-marks.xml: all 0 (names in all: 35)
a-tb3a-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-mv0-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-mb-M-marks.xml: all 0 (names in all: 35)
a-mc-M-marks.xml: all 0 (names in all: 35)
a-m1-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb3c-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-x1-X-lx1-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb3x-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-m1r-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-m2-M-marks.xml: all 0 (names in all: 35)
a-mv2-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-twtb3-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-twmvy-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-vdtb3w-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldtb3-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldx1-X-lx1-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldm1-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldm2-M-marks.xml: all 0 (names in all: 35)
```

### sha256 of every answer
```
$ sha256sum and size of every answer of this row, in send order, and of the sheet's copies of earlier rows' answers
a-m0-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-tb3a-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-mv0-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-mb-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-mc-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-m1-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-tb3c-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-x1-X-lx1-20250401.xml 11556 B 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
a-tb3x-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-m1r-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-m2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-mv2-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-twtb3-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-twmvy-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-vdtb3w-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-ldtb3-T-tb3-20250401-20250401.xml 13528 B a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-ldx1-X-lx1-20250401.xml 11556 B 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
a-ldm1-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-ldm2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### m0
```
12:05:00	M-marks.xml	a-m0-M-marks.xml	curl 0	3404 bytes
```

### tb3a
```
12:05:10	T-tb3-20250401-20250401.xml	a-tb3a-T-tb3-20250401-20250401.xml	curl 0	14140 bytes
```

### mv0
```
12:05:19	V-mvy-20250401-20260331.xml	a-mv0-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```

### mb
```
12:07:10	M-marks.xml	a-mb-M-marks.xml	curl 0	3404 bytes
```

### mc
```
12:07:11	M-marks.xml	a-mc-M-marks.xml	curl 0	3404 bytes
```

### m1
```
12:15:10	M-marks.xml	a-m1-M-marks.xml	curl 0	3404 bytes
```

### tb3c
```
12:15:23	T-tb3-20250401-20250401.xml	a-tb3c-T-tb3-20250401-20250401.xml	curl 0	13740 bytes
```

### x1
```
12:15:31	X-lx1-20250401.xml	a-x1-X-lx1-20250401.xml	curl 0	11556 bytes
```

### tb3x
```
12:15:41	T-tb3-20250401-20250401.xml	a-tb3x-T-tb3-20250401-20250401.xml	curl 0	13740 bytes
```

### m1r
```
12:15:57	M-marks.xml	a-m1r-M-marks.xml	curl 0	3404 bytes
```

### m2
```
12:18:40	M-marks.xml	a-m2-M-marks.xml	curl 0	3404 bytes
```

### mv2
```
12:18:42	V-mvy-20250401-20260331.xml	a-mv2-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```
