# Row 683-VD: with row 683-LD's two vouchers keyed again, tb3 at current date 30-Apr-2025 did not repeat row 683-LD's answer (two ledgers still MIXED); tb3 at 1-Apr-2025 was byte-equal to it, so by the sheet the date question is not yet decided

Run on 10 Nov 2026, 04:35:24Z to 04:59:15Z (10:05 to 10:29 IST), on TallyPrime Silver (the Gateway on 9001; licensed, no Educational mode shown; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/683-vd-sheet` (7ad3795a), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own. Raw reads, one at a time through `run.py`; two vouchers keyed by hand; the current date moved twice by F2 at the Gateway; one backup, restored at the end; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Every read was answered, each with STATUS 1, within a second (0.4 to 0.8 s); no dialog came up after any read.

This sitting ran after row 2464-NR, on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The restore returned the book to the marks it was found with (11/214).

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214: SAME MARKS with row 683-LD's m2. EXPECT met.
- **The controls (step 2), each as EXPECT:** tb3a (current date 1-Apr-2025) byte-equal to row 683-TW's tb3 (14,140 B, sha256 `1c4638e8...`), SETS SAME, six MIXED and two PLAIN; mv0 byte-equal to row 683-TW's mvy, CAPTURE SAME, vouchers movement 11, capture 11.
- **BK, mb, mc:** BK taken; mb SAME MARKS with m0; mc SAME MARKS with mb, with no key pressed between.
- **The write:** K1 saved as Sales No. 4, 30-Apr-25: `BRIDGE USD Customer` Dr, `BRIDGE Export Sales` $100.00 at 88 = 8,800.00 rupees, New Ref LD1. K2 saved as Receipt No. 4, 30-Apr-25: `BRIDGE INR Customer` Cr 2,000.00 Agst Ref R1 (R1 pending 3,000.00 Dr), `Cash` Dr 2,000.00 (offered by Tally, not typed). Each number was the one expected (4 and 4). No gain or loss line on either. No "Spelling Error" box this time (every name was typed in keystroke chunks). Neither save asked "Accept ?", as in row 683-LD.
- **m1:** MARKS DIFFER with mc (exit 2), ALTVCHID 11 to 13, ALTMSTID 214 (unchanged); SAME MARKS with row 683-LD's m1. Both EXPECTs met.
- **tb3w (current date 30-Apr-2025, row 683-LD's condition): not as row 683-LD's.** 13,740 B, sha256 `f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb`. Against row 683-LD's tb3 (13,528 B, every ledger PLAIN): `cmpinfo.py` exit 2, differing outside CMPINFO; `tb.py --sets` SETS DIFFER. Two ledgers, `BRIDGE Import Purchases` and `BRIDGE Lab Bank`, read the dollar composite (MIXED) where row 683-LD's read plain rupee figures; the other six read PLAIN, as row 683-LD's. Every rupee figure is equal. Of 40 fields, 8 differ (`evidence/tb3w-fields-vs-683ld.txt`), all in those two ledgers: for example `BRIDGE Lab Bank` TBALCLOSING `-$900.00 @ ₹ 88/$ = -₹ 79200.00` here against `-79200.00` there, and `BRIDGE Import Purchases` CREDITTOTALS EMPTY here against `0.00` there.
- **6b, the date to 1 April:** F2 at the Gateway, the "Change Current Date" box offered `1-5-2025`; `1-4-2025` typed: Tuesday, 1-Apr-2025, last entry still 30-Apr-25. **md1:** SAME MARKS with m1.
- **tb3v (current date 1-Apr-2025, the question): byte-equal to tb3w** (`cmpinfo.py` exit 0; SETS SAME). No dialog (the window captured at once). Against tb3a: not byte-equal (14,140 B to 13,740 B), differing outside CMPINFO as well; the CMPINFO count lines read LEDGER 0 to 6, CURRENCY 0 to 2, TAXUNIT 0 to 2, VOUCHERNUMBERSERIES 0 to 2, VOUCHER 0 to 3; SETS DIFFER, `BRIDGE Export Sales`, `BRIDGE USD Customer`, `BRIDGE USD Supplier` and `Profit & Loss A/c` MIXED to PLAIN. Every rupee figure equal to tb3a's. Of 40 fields, 15 differ (`evidence/tb3v-fields-vs-tb3a.txt`).
- **6c, the date back to 30 April:** F2 at the Gateway, offered `2-4-2025`; `30-4-2025` typed: Wednesday, 30-Apr-2025, last entry 30-Apr-25. **md2:** SAME MARKS with m1. **tb3w2:** byte-equal to tb3w; SETS SAME.
- **m1r:** SAME MARKS with m1 (13/214): the reads and the date wrote nothing.
- **The restore (step 8):** by C6, the two expected prompts only. **m2:** SAME MARKS with m0 (11/214). **mv2:** CAPTURE SAME, 11 and 11, and byte-equal to mv0. So the book is as BK holds it.
- **Step 9:** current date Tuesday, 1-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 1-Apr-25: as at step 0. EXPECT met.

**By the sheet's Decides:**
- **`tb3w` not as row 683-LD's: seen.** Row 683-LD's answer did not repeat with the same vouchers, keyed the same way, and the same current date. By the sheet, `tb3v` and `tb3w2` are recorded but decide nothing until that is explained. The explanation is not part of this run.
- Recorded, not judged: in this sitting the three answers at 30-Apr-2025, 1-Apr-2025 and 30-Apr-2025 again are byte-equal, so moving the current date changed nothing here. `tb3v` is also the "neither" case of the Decides: two ledgers MIXED and six PLAIN.
- **The candidate the sheet itself names** (its Question, "The controls"): row 683-LD sent its opening export `x1` just before its tb3, and this row sends no export. That is the one difference in read history between row 683-LD's tb3 and this row's tb3w. This run does not test it and does not say it is the cause.

**The sheet's estimates:** the controls, the write (Sales No. 4 and Receipt No. 4) and m1 against row 683-LD's m1, each as estimated; `tb3w2` byte-equal to `tb3w`, as estimated; the restore as estimated. Not as estimated: `tb3w` (estimated every ledger PLAIN, and byte-equal to row 683-LD's tb3 or differing only inside CMPINFO).

## Deviations
1. **The start-up company was shut first.** Its own book loaded by itself, as the install is set to, and was shut by C9 before step 0 (the K: Company label, Shut, Enter on its one row, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
2. **Two keys refused by the capture tool.** At that first shut, and again at the shut of C10, the capture tool refused a key because the desktop client had the keyboard focus. No key reached Tally either time. A click on Tally's title bar and a capture showed the screen unchanged, and the key was then pressed.

## Step 0: start, check and load
- Sheet files: all 15 equal to `SHA256.txt` at 7ad3795a. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `lm.py`, `tb.py`, `cmpinfo.py`, `M-marks.xml` and both requests are byte-equal to row 683-LD's. `a-ldtb3`, `a-ldm1`, `a-ldm2` and `a-twmvy` are byte-equal to row 683-LD's answers and copies; `a-twtb3` is byte-equal to row 683-CD's copy.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 04:35:25Z, from its desktop shortcut, after tasklist showed none.
- C2 at 04:36:44Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25. The PC's date: 10 Nov 2026 (IST).

## Steps 1 to 3: m0, the controls, BK
- m0, tb3a and mv0 as in the Result.
- **BK** (C5), to the new folder `C:\lab\bk-683vd` (made empty before the start): the Data label, Backup & Restore, Backup; Backspace to the path; Specify Path (Up 3 times), the path typed, its row taken; BRIDGE FOREXGROUP LAB alone, End of List (Up 3 times); "Accept ?" captured (`bk-accept`) and answered y; no other message; the Gateway again. The folder then held one file: `TDBK1800_<company number>.001`, 37,618 B, sha256 `ec7093967397090180fd48bc3d088f9ce9669cbce0948b5cb885d9adb4919b2d`, written at 10:08 IST; the Restore list was to show 10-Nov-26 (10:08 am), and did.
- mb and mc: SAME MARKS, as in the Result.

## Step 4: the write, word for word as the screen showed it
`evidence/notes.txt` holds every line below with its time; each capture is named in `SCREENSHOTS.txt`.

**K1, Sales.**
- The Gateway, `v` (Vouchers): Accounting Voucher Creation opened as Payment No. 3. F8: Sales, **No. 4** offered (`k1-1-sales-no4`).
- F2: the "Change Voucher Date" box, Voucher Date offered `2-4-2025`; `30-4-2025` typed, Enter (`k1-2-date-box`).
- Party A/c name: `BRIDGE`, space, `USD`, space, `Customer` typed as keystrokes; one row, `BRIDGE USD Customer`, highlighted (`k1-3-party-typed`); Enter. No "Spelling Error" box.
- "Dispatch Details" box (Delivery Note No(s), Dispatch Doc No., Dispatched through, Destination, Carrier Name/Agent, Bill of Lading/LR-RR No. and Date, Motor Vehicle No.; all empty): Enter on each, all left empty (`k1-4-dispatch-details`).
- "Party Details" box (Buyer (Bill to) BRIDGE USD Customer, Mailing Name BRIDGE USD Customer, Address empty, State Not Applicable, Country India, GST Registration type Regular (list: Unknown, Composition, Regular, Unregistered/Consumer), GSTIN/UIN empty, Place of Supply Not Applicable): Enter on each, as offered (`k1-5-party-details`). The voucher then showed **30-Apr-25 Wednesday**, the party's Current balance 8,79,12,000.00 Dr (`k1-6-date-before-amount`).
- Particulars: `BRIDGE Export Sales` typed as keystrokes, the one row highlighted, Enter. No Inventory Allocations box.
- Amount: `$100` typed, Enter. The **Forex Rate of Exchange** box: columns Forex Amount, Rate of Exchange, Value in (rupee sign); offered `$100.00 @ (rupee) 88/$`, `(rupee) 8,800.00` (`k1-7-forex-box`). Enter on Forex Amount; `88` typed over the rate, Enter; Value 8,800.00 as offered, Enter. The line `BRIDGE Export Sales $100.00`; total `$100.00 @ (rupee) 88/$ = (rupee) 8,800.00` (`k1-8-line-100-at-88`).
- Enter on the empty second line, then Enter on the total: **Bill-wise Details for : BRIDGE USD Customer**, "Up to: (rupee) 8,800.00 Dr"; columns Type of Ref, Name, Due Date or Credit Days (wef: 30-Apr-25), Amount, Dr/Cr; Method of Adj. list Advance, Agst Ref, New Ref (highlighted), On Account (`k1-9-billwise`). New Ref, Enter; Name offered `4`, `LD1` typed over it, Enter; Due Date empty, Enter; Amount $100.00 as offered, Enter; Dr, Enter. One line: New Ref, LD1, $100.00 Dr.
- Narration left empty. Before Ctrl+A (`k1-10-before-accept`): no gain or loss line. Ctrl+A: saved, no prompt; a blank Sales No. 5 (`k1-11-after-save`). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; date of last entry 30-Apr-25 (`k1-12-gateway-after`).

**K2, Receipt.**
- The Gateway, `v`, F6: Receipt, **No. 4** offered (`k2-1-receipt-no4`).
- To: `BRIDGE`, space, `INR`, space, `Customer` typed as keystrokes, the one row highlighted, Enter. The voucher already showed **30-Apr-25 Wednesday** (no F2 used; `k2-2-date-before-amount`). Cur Bal 3,000.00 Dr.
- Credit: `2000`, Enter. No Forex box.
- **Bill-wise Details for : BRIDGE INR Customer**, "Up to: (rupee) 2,000.00 Cr"; Agst Ref highlighted in Method of Adj., Enter (`k2-3-billwise`). **Pending Bills:** one row, `R1`, Bill Date 1-Apr-25, Due Date 1-Apr-25, Balance 3,000.00 Dr, Final Balance 3,000.00 Dr (`k2-4-pending-r1`); Enter. Amount offered 2,000.00 Cr; `2000` typed over it, Enter; Cr, Enter (`k2-5-r1-amount`). The box closed: Agst Ref R1 2,000.00 Cr; the party's Cur Bal then 1,000.00 Dr.
- Tally then offered **By Cash**, Debit 2,000.00, Cur Bal 5,000.00 Dr (`k2-6-by-cash-offered`): Enter on By; the ledger list with Cash highlighted (the field read Cash), Enter; Debit 2,000.00 as offered, Enter.
- Narration left empty. Before Ctrl+A (`k2-7-before-accept`): To BRIDGE INR Customer Credit 2,000.00, By Cash Debit 2,000.00, totals 2,000.00 each; no gain or loss line. Ctrl+A: saved, no prompt; a blank Receipt No. 5 (`k2-8-after-save`). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; last entry 30-Apr-25 (`k2-9-gateway-after`).

## Steps 5 to 7: the reads and the date
Each request's sha256 was checked against `SHA256.txt` just before its send (`evidence/sends-utc.txt`). The whole outputs are below.
- **6b:** at the Gateway, after a click on Tally's title bar, F2: a box titled "Change Current Date" with one field, Current Date, offered `1-5-2025` (`d1-1-change-date-box`). `1-4-2025` typed, Enter: the Gateway, current period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025, date of last entry 30-Apr-25 (`d1-2-gateway-1apr`).
- **tb3v:** the Tally window captured at once after the answer (`tb3v-window`): the Gateway as after 6b, no dialog.
- **6c:** F2 again: the same box, offered `2-4-2025` (`d2-1-change-date-box`). `30-4-2025` typed, Enter: the Gateway, current date Wednesday, 30-Apr-2025, last entry 30-Apr-25 (`d2-2-gateway-30apr`).
- Two calls beyond the sheet's are kept beside the sheet's: `tb.py` on row 683-LD's tb3 and on tb3a (`tb-ldtb3.txt`, `tb-tb3a.txt`), from which the two field lists were made, as row 683-LD's `tb3-fields-vs-683tw.txt` was.

## Step 8: the restore (C6)
- Before the shut: the BK re-hashed equal to the one recorded, its folder holding that file alone; `restore-plan.txt` written; the data folder listed (24 entries). The plan and listings stay on the lab PC.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt (`c6-1-after-shut`).
- The Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`; Specify Path (Up 3 times), `C:\lab\bk-683vd` typed. The list showed one row: BRIDGE FOREXGROUP LAB (`<company number>`), 10-Nov-26 (10:08 am) (`8-restore-list`). Enter on it; End of List, Enter.
- "Accept ?" (`8-restore-accept`): y. Then, word for word: "Company data folder (`<company number>`) already exists." / "Selecting Skip will cancel the Restore process for this company." / "Selecting Replace will overwrite the existing folder.", buttons "R: Replace" and "S: Skip" (`8-restore-replace`): r. Select Company followed, no other message.
- The data folder after the restore: after=17, before=24 entries. Gone: `CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`, `TUPDATE.TSF` (the same seven as row 683-LD's restore). None new.
- Loaded alone: one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`8-select`), Enter, no prompt. The Gateway (`8-gateway`): current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25.
- m2 and mv2 as in the Result.

## Steps 9 and 10: the date, and close (C10)
- Step 9 (`9-gateway`): as in the Result; nothing was pressed to change the date or the period.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter (deviation 2), no prompt: Select Company, no company loaded (`c10-no-company`, 04:58:45Z).
- C2 at 04:58:45Z, after the last request: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B). No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; at 04:59:15Z no `tally.exe` ran and nothing listened on 9001. The three Bridge processes were not ended.

## Files
- `answers/`: this row's answers that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256). The sheet's copies of other rows' answers are not repeated here.
- `evidence/`: notes, the outputs of `lm.py`, `tb.py` and `cmpinfo.py`, the two field lists, the start-up check's hits. Process and port listings, the backup folder and data folder listings, and the restore plan stay on the lab PC.
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

### tb3a: cmpinfo.py against row 683-TW's tb3
```
tb3a 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7 byte-equal to a-twtb3: True
```

### tb3a: tb.py
```
a-tb3a-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
  LEDGER rows: 8
  'BRIDGE Export Sales' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25' COMPOSITE
    TBALCLOSING '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25' COMPOSITE
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
  'BRIDGE USD Customer' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-$1000100.00 @ ₹ 87.9999/$ = -₹ 88008700.00' COMPOSITE
    CREDITTOTALS '$1100.00 @ ₹ 87/$ = ₹ 96700.00' COMPOSITE
    TBALCLOSING '-$999000.00 @ ₹ 88/$ = -₹ 87912000.00' COMPOSITE
  'BRIDGE USD Supplier' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-$200.00 @ ₹ 87/$ = -₹ 17500.00' COMPOSITE
    CREDITTOTALS '$200.00 @ ₹ 87.50/$ = ₹ 17500.00' COMPOSITE
    TBALCLOSING '$0.00 @ ₹ /$ = ₹ 0.00' COMPOSITE
  'Cash' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-3000.00' PLAIN
    CREDITTOTALS '' EMPTY
    TBALCLOSING '-3000.00' PLAIN
  'Profit & Loss A/c' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '' EMPTY
    TBALCLOSING '$0.00 @ ₹ /$ = ₹ 0.00' COMPOSITE
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
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

### mv0: cmpinfo.py against row 683-TW's mvy
```
mv0 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7 byte-equal to a-twmvy: True
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

### tb3w: cmpinfo.py against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-tb3w-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
exit 2
```

### tb3w: tb.py
```
a-tb3w-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb3w: tb.py --sets against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3w-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-ldtb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3w-T-tb3-20250401-20250401.xml:
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
```

### tb3w: the fields that differ from row 683-LD's tb3
```
$ python -I tb.py on a-ldtb3 (row 683-LD's tb3, tb-ldtb3.txt) and this row's tb3w (tb-tb3w.txt); each line: (ledger, field) row 683-LD's value and class -> tb3w's
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

### md1: lm.py
```
a-md1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
exit 0
```

### md1: lm.py --same-row against m1
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
a-md1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
SAME MARKS
exit 0
```

### tb3v: cmpinfo.py against tb3w
```
a-tb3w-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-tb3v-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
BYTE-EQUAL
exit 0
```

### tb3v: cmpinfo.py against tb3a
```
a-tb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb3v-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
  CMPINFO LEDGER: 0 -> 6
  CMPINFO CURRENCY: 0 -> 2
  CMPINFO TAXUNIT: 0 -> 2
  CMPINFO VOUCHERNUMBERSERIES: 0 -> 2
  CMPINFO VOUCHER: 0 -> 3
exit 2
```

### tb3v: tb.py
```
a-tb3v-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb3v: tb.py --sets against tb3w
```
a-tb3w-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3v-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3w-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3v-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb3v: tb.py --sets against tb3a
```
a-tb3a-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3v-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3a-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb3v-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
  'BRIDGE Export Sales': MIXED -> PLAIN
  'BRIDGE USD Customer': MIXED -> PLAIN
  'BRIDGE USD Supplier': MIXED -> PLAIN
  'Profit & Loss A/c': MIXED -> PLAIN
SETS DIFFER
exit 0
```

### tb3v: the fields that differ from tb3a
```
$ python -I tb.py on a-tb3a (this row's control, byte-equal to row 683-TW's tb3; tb-tb3a.txt) and tb3v (tb-tb3v.txt); each line: (ledger, field) tb3a's value and class -> tb3v's
('BRIDGE Export Sales', 'CREDITTOTALS') ('$1000100.00 @ ₹ 86/$ = ₹ 86014641.25', 'COMPOSITE') -> ('86014641.25', 'PLAIN')
('BRIDGE Export Sales', 'DEBITTOTALS') ('', 'EMPTY') -> ('0.00', 'PLAIN')
('BRIDGE Export Sales', 'TBALCLOSING') ('$1000100.00 @ ₹ 86/$ = ₹ 86014641.25', 'COMPOSITE') -> ('86014641.25', 'PLAIN')
('BRIDGE Export Sales', 'class') MIXED -> PLAIN
('BRIDGE USD Customer', 'CREDITTOTALS') ('$1100.00 @ ₹ 87/$ = ₹ 96700.00', 'COMPOSITE') -> ('96700.00', 'PLAIN')
('BRIDGE USD Customer', 'DEBITTOTALS') ('-$1000100.00 @ ₹ 87.9999/$ = -₹ 88008700.00', 'COMPOSITE') -> ('-88008700.00', 'PLAIN')
('BRIDGE USD Customer', 'TBALCLOSING') ('-$999000.00 @ ₹ 88/$ = -₹ 87912000.00', 'COMPOSITE') -> ('-87912000.00', 'PLAIN')
('BRIDGE USD Customer', 'class') MIXED -> PLAIN
('BRIDGE USD Supplier', 'CREDITTOTALS') ('$200.00 @ ₹ 87.50/$ = ₹ 17500.00', 'COMPOSITE') -> ('17500.00', 'PLAIN')
('BRIDGE USD Supplier', 'DEBITTOTALS') ('-$200.00 @ ₹ 87/$ = -₹ 17500.00', 'COMPOSITE') -> ('-17500.00', 'PLAIN')
('BRIDGE USD Supplier', 'TBALCLOSING') ('$0.00 @ ₹ /$ = ₹ 0.00', 'COMPOSITE') -> ('0.00', 'PLAIN')
('BRIDGE USD Supplier', 'class') MIXED -> PLAIN
('Cash', 'CREDITTOTALS') ('', 'EMPTY') -> ('0.00', 'PLAIN')
('Profit & Loss A/c', 'TBALCLOSING') ('$0.00 @ ₹ /$ = ₹ 0.00', 'COMPOSITE') -> ('0.00', 'PLAIN')
('Profit & Loss A/c', 'class') MIXED -> PLAIN
fields differing: 15 of 40
```

### md2: lm.py
```
a-md2-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
exit 0
```

### md2: lm.py --same-row against m1
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
a-md2-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 13 ALTMSTID 214
SAME MARKS
exit 0
```

### tb3w2: cmpinfo.py against tb3w
```
a-tb3w-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-tb3w2-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
BYTE-EQUAL
exit 0
```

### tb3w2: tb.py --sets against tb3w
```
a-tb3w-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3w2-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3w-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3w2-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
SETS SAME
exit 0
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

### mv2: cmpinfo.py against mv0
```
a-mv0-V-mvy-20250401-20260331.xml: 59524 B sha256 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-mv2-V-mvy-20250401-20260331.xml: 59524 B sha256 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
BYTE-EQUAL
exit 0
```

### each send's start and answer, UTC
```
2026-11-10T04:37:24.361Z m0 M-marks.xml (hash equal to the list)
2026-11-10T04:37:24.897Z m0 answered
2026-11-10T04:37:33.782Z tb3a T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T04:37:34.174Z tb3a answered
2026-11-10T04:37:42.350Z mv0 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T04:37:42.859Z mv0 answered
2026-11-10T04:39:12.593Z mb M-marks.xml (hash equal to the list)
2026-11-10T04:39:13.040Z mb answered
2026-11-10T04:39:23.533Z mc M-marks.xml (hash equal to the list)
2026-11-10T04:39:23.939Z mc answered
2026-11-10T04:50:54.427Z m1 M-marks.xml (hash equal to the list)
2026-11-10T04:50:55.125Z m1 answered
2026-11-10T04:51:06.535Z tb3w T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T04:51:07.035Z tb3w answered
2026-11-10T04:52:26.461Z md1 M-marks.xml (hash equal to the list)
2026-11-10T04:52:27.198Z md1 answered
2026-11-10T04:52:36.023Z tb3v T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T04:52:36.685Z tb3v answered
2026-11-10T04:53:58.316Z md2 M-marks.xml (hash equal to the list)
2026-11-10T04:53:58.926Z md2 answered
2026-11-10T04:54:00.741Z tb3w2 T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T04:54:01.234Z tb3w2 answered
2026-11-10T04:54:02.562Z m1r M-marks.xml (hash equal to the list)
2026-11-10T04:54:03.064Z m1r answered
2026-11-10T04:57:44.528Z m2 M-marks.xml (hash equal to the list)
2026-11-10T04:57:45.341Z m2 answered
2026-11-10T04:57:47.199Z mv2 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T04:57:47.974Z mv2 answered
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
a-tb3w-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-md1-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb3v-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-md2-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb3w2-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-m1r-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-m2-M-marks.xml: all 0 (names in all: 35)
a-mv2-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-twtb3-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-twmvy-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-ldtb3-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
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
a-tb3w-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-md1-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-tb3v-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-md2-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-tb3w2-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-m1r-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-m2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-mv2-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-twtb3-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-twmvy-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-ldtb3-T-tb3-20250401-20250401.xml 13528 B a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-ldm1-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-ldm2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### m0
```
10:07:24	M-marks.xml	a-m0-M-marks.xml	curl 0	3404 bytes
```

### tb3a
```
10:07:34	T-tb3-20250401-20250401.xml	a-tb3a-T-tb3-20250401-20250401.xml	curl 0	14140 bytes
```

### mv0
```
10:07:42	V-mvy-20250401-20260331.xml	a-mv0-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```

### mb
```
10:09:12	M-marks.xml	a-mb-M-marks.xml	curl 0	3404 bytes
```

### mc
```
10:09:23	M-marks.xml	a-mc-M-marks.xml	curl 0	3404 bytes
```

### m1
```
10:20:54	M-marks.xml	a-m1-M-marks.xml	curl 0	3404 bytes
```

### tb3w
```
10:21:06	T-tb3-20250401-20250401.xml	a-tb3w-T-tb3-20250401-20250401.xml	curl 0	13740 bytes
```

### md1
```
10:22:26	M-marks.xml	a-md1-M-marks.xml	curl 0	3404 bytes
```

### tb3v
```
10:22:36	T-tb3-20250401-20250401.xml	a-tb3v-T-tb3-20250401-20250401.xml	curl 0	13740 bytes
```

### md2
```
10:23:58	M-marks.xml	a-md2-M-marks.xml	curl 0	3404 bytes
```

### tb3w2
```
10:24:01	T-tb3-20250401-20250401.xml	a-tb3w2-T-tb3-20250401-20250401.xml	curl 0	13740 bytes
```

### m1r
```
10:24:02	M-marks.xml	a-m1r-M-marks.xml	curl 0	3404 bytes
```

### m2
```
10:27:45	M-marks.xml	a-m2-M-marks.xml	curl 0	3404 bytes
```

### mv2
```
10:27:47	V-mvy-20250401-20260331.xml	a-mv2-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```
