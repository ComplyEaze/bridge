# Row 683-PW: with row 683-LD's reads before the write (x0, then tb10), the Trial Balance to 1 April read every ledger PLAIN after the two vouchers: tb3p was byte-equal to row 683-LD's tb3, tb3p2 to tb3p, and tb1p to row 683-LD's tb1

Run on 10 Nov 2026, 15:26:32Z to 15:49:14Z (20:56 to 21:19 IST), on TallyPrime Silver (the Gateway on 9001; licensed, no Educational mode shown; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/683-pw-sheet` (ad26232a), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own. Raw reads, one at a time through `run.py`; two vouchers keyed by hand; one backup, restored at the end; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Every read was answered, each with STATUS 1, within a second (0.4 to 0.6 s); no dialog came up after any read.

This sitting had a Tally start of its own: Tally was started once from its desktop shortcut and closed by its own Quit at the end. Since row 683-XF's second sitting (which quit Tally at 09:33:32Z), this PC's Tally ran rows 2480-NM and 2399-R, each in a sitting of its own and on another lab book (BRIDGE PILOT LAB); nothing was loaded or sent on BRIDGE FOREXGROUP LAB. Tally's start before this one was row 2399-R's (14:13:09Z, quit by C10 at 14:40Z). The restore returned the book to the marks it was found with (11/214).

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214: SAME MARKS with row 683-LD's m2. EXPECT met.
- **The controls (step 2), each as EXPECT, and each byte-equal to its reference:**
  - **x0** (the opening export at 1 April): `rows.py` CONTENT SAME with row 683-LD's x0, both orders SAME (exit 0); `cmpinfo.py` BYTE-EQUAL (exit 0; 11,556 B); 8 LEDGER rows, each opening 0.00.
  - **tb10** (the Trial Balance to 30 April): SETS SAME with row 683-LD's tb10 (six MIXED: `BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`; two PLAIN: `BRIDGE INR Customer`, `Cash`); `rows.py` CONTENT SAME, both orders SAME (exit 0); BYTE-EQUAL (exit 0; 14,140 B).
  - **mv0:** CAPTURE SAME with row 683-TW's mvy, vouchers movement 11, capture 11; byte-equal to it (59,524 B).
- **BK, mb, mc:** BK taken; mb SAME MARKS with m0; mc SAME MARKS with mb, with no key pressed between.
- **The write, as rows 683-VD's, 683-RO's and 683-XF's:** K1 saved as Sales No. 4, 30-Apr-25: `BRIDGE USD Customer` Dr, `BRIDGE Export Sales` $100.00 at 88 = 8,800.00 rupees, New Ref LD1. K2 saved as Receipt No. 4, 30-Apr-25: `BRIDGE INR Customer` Cr 2,000.00 Agst Ref R1 (R1 pending 3,000.00 Dr), `Cash` Dr 2,000.00 (offered by Tally, not typed). Each number was the one expected (4 and 4). No gain or loss line on either. No "Spelling Error" box (every name was typed in keystroke chunks). Neither save asked "Accept ?".
- **m1:** MARKS DIFFER with mc (exit 2), ALTVCHID 11 to 13, ALTMSTID 214 (unchanged); SAME MARKS with row 683-LD's m1. Both EXPECTs met.
- **x1 (the opening export at 1 April, the first send after m1): byte-equal to row 683-LD's x1** (`cmpinfo.py` exit 0; 11,556 B); `rows.py` CONTENT SAME, both orders SAME (exit 0); 8 LEDGER rows, each opening 0.00. EXPECT met.
- **tb3p (Q1; current date 30-Apr-2025): as row 683-LD's tb3, and byte-equal to it** (`cmpinfo.py` exit 0; 13,528 B, sha256 `a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705`); SETS SAME and `rows.py` CONTENT SAME with it (exit 0).
  - Every ledger PLAIN, none MIXED; no UNREAD.
  - The Tally window captured at once after the answer (`tb3p-window`, 15:42:54Z): the Gateway, current date Wednesday, 30-Apr-2025, last entry 30-Apr-25, no dialog.
  - Against row 683-VD's tb3w (13,740 B; `BRIDGE Import Purchases` and `BRIDGE Lab Bank` MIXED): `cmpinfo.py` exit 2, differing outside CMPINFO; `rows.py` CONTENT DIFFER (exit 2), both orders SAME; `tb.py --sets` SETS DIFFER, those two ledgers MIXED to PLAIN. Every rupee figure is equal. Of 40 fields, 8 differ (`evidence/tb3p-fields-vs-683vd.txt`), all in those two ledgers: the same 8 as row 683-XF's second sitting's list against row 683-LD's tb3, each the other way round.
  - Against row 683-TW's tb3 (the book before the write): `rows.py` CONTENT DIFFER (exit 2), with ELEMENT ORDER SAME: tb3p has row 683-TW's element order. Of 40 fields, 23 differ (`evidence/tb3p-fields-vs-683tw.txt`).
- **tb3p2**, sent 8.3 s after tb3p answered: byte-equal to tb3p (`cmpinfo.py` exit 0); `rows.py` exit 0; SETS SAME. As estimated.
- **tb1p (Q2; the Trial Balance to 30 April): as row 683-LD's tb1, and byte-equal to it** (`cmpinfo.py` exit 0; 14,140 B, sha256 `47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f`); SETS SAME and `rows.py` CONTENT SAME, both orders SAME (exit 0): six MIXED and two PLAIN, the same sets as tb10's. The Tally window at once after it (`tb1p-window`, 15:43:08Z): the Gateway, Wednesday, 30-Apr-2025, last entry 30-Apr-25, no dialog. As estimated.
- **m1r:** SAME MARKS with m1 (13/214): the reads wrote nothing.
- **The restore (step 8):** `prep-c6.sh` exit 0 before the shut; by C6, the two expected prompts only. **m2:** SAME MARKS with m0 (11/214). **mv2:** CAPTURE SAME with mv0, 11 and 11, and byte-equal to mv0. So the book is as BK holds it.
- **Step 9:** current date Tuesday, 1-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 1-Apr-25: as at step 0. EXPECT met.

**By the sheet's Decides:** `x1` is as row 683-LD's, so the `x1` case does not apply. This is the case "`tb3p` as row 683-LD's (every ledger PLAIN), `tb3p2` as `tb3p`". In the sheet's words: "what the sitting reads before the write fixes how the one-day window reads after it: PARTIAL." By the sheet, the next row splits the two reads (`x0` alone before the write, then `tb10` alone); that goes to #683 first, and nothing is built on it here.
- **Q2:** tb1p is as row 683-LD's tb1 (six MIXED): the wider window kept its marks in this sitting as in row 683-LD's, so the effect stays bounded to the one-day window.
- What still differs from row 683-LD is recorded from the rows' notes, and none of it is taken as the cause:
  - the keying of K1: row 683-LD's met a "Spelling Error" box and cleared it; this one met none, and m1 here is SAME MARKS with row 683-LD's m1;
  - the gaps (below).
- **Timing, recorded and not set** (`evidence/gaps.txt`): x1 was sent 2.0 s after m1 answered, tb3p 2.1 s after x1 answered, and tb3p 37 s after K2's save (15:42:13Z, by the capture `k2-8-after-save`). These are the gaps of row 683-XF's second sitting (2.1 s and 37 s), whose tb3f read as row 683-VD's; row 683-LD's were about 10 s and 58 s, and about 2 minutes from K2's save to its tb3. Before the write: x0 8.2 s after m0 answered, tb10 1.9 s after x0, mv0 2.1 s after tb10 (row 683-LD's: about 9.6 s, 8.1 s and 7.5 s).

**The element order (recorded; the sheet decides nothing on it):**
- Within this sitting it held: tb3p against tb3p2 (both orders SAME, byte-equal), and tb10 and tb1p each with the order of its reference (both orders SAME against row 683-LD's tb10 and tb1, each byte-equal).
- Every Trial Balance of this sitting (tb10, tb3p, tb3p2, tb1p) has `TBALCLOSING` before `TBALOPENING` in each ledger, row 683-TW's order, as in row 683-XF's second sitting.

**The sheet's estimates:** the controls (with the content of row 683-LD's x0 and tb10 and row 683-TW's mvy), the write (Sales No. 4 and Receipt No. 4), m1 against row 683-LD's m1, x1 (byte-equal to row 683-LD's), tb3p2 (byte-equal to tb3p), tb1p (as row 683-LD's tb1) and the restore, each as estimated. tb3p had no estimate: it was the question.

## Deviations
1. **The start-up company was shut first.** Its own book loaded by itself, as the install is set to, and was shut before step 0 (the K: Company label, Shut, Enter on its one row, no prompt), on the run lead's standing word. Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).

## Step 0: start, check and load
- Sheet files: all 24 equal to `SHA256.txt` at ad26232a (sizes too). `run.py`, `lm.py`, `tb.py`, `rows.py` and `cmpinfo.py` are byte-equal to row 683-XF's second sitting's.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 15:26:32Z, from its desktop shortcut, after tasklist showed none.
- C2 at 15:27:06Z: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B).
- The start-up company was shut (deviation 1). The book was loaded alone: `BRIDGE`, space, `FOREXGROU` typed in the name filter, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25. The PC's date: 10 Nov 2026 (IST).

## Steps 1 to 3: m0, the controls, BK
- m0, x0, tb10 and mv0 as in the Result, one send at a time, each straight after the previous answer's counters.
- **BK** (C5), to the new folder `C:\lab\bk-683pw` (made empty before the start): the Data label, Backup & Restore, Backup; Backspace to the path; Specify Path (Up 3 times), the path typed in two keystroke chunks, its row taken; BRIDGE FOREXGROUP LAB alone, End of List (Up 3 times); "Accept ?" captured (`bk-accept`, 15:29:57Z) and answered y; no other message; the Gateway again. The folder then held one file: `TDBK1800_<company number>.001`, 37,618 B, sha256 `84287030aa24a2d6294d42c1b7db026779f61286abc755c71b40f5a11a30711f`, written at 20:59 IST; the Restore list was to show 10-Nov-26 (8:59 pm), and did.
- mb and mc: SAME MARKS, as in the Result.

## Step 4: the write, word for word as the screen showed it
`evidence/notes.txt` holds every line below with its time; each capture is named in `SCREENSHOTS.txt`.

**K1, Sales.**
- The Gateway, a click on Tally's title bar, `v` (Vouchers): Accounting Voucher Creation opened as Payment No. 3 (`k1-0-payment-no3`). F8: Sales, **No. 4** offered (`k1-1-sales-no4`).
- F2: the "Change Voucher Date" box, Voucher Date offered `2-4-2025`; `30-4-2025` typed, Enter (`k1-2-date-box`).
- Party A/c name: `BRIDGE`, space, `USD`, space, `Customer` typed as keystrokes; one row, `BRIDGE USD Customer`, highlighted (`k1-3-party-typed`); Enter. No "Spelling Error" box.
- "Dispatch Details" box (Delivery Note No(s), Dispatch Doc No., Dispatched through, Destination, Carrier Name/Agent, Bill of Lading/LR-RR No. and Date, Motor Vehicle No.; all empty): Enter on each, all left empty (`k1-4-dispatch-details`).
- "Party Details" box (Buyer (Bill to) BRIDGE USD Customer, Mailing Name BRIDGE USD Customer, Address empty, State Not Applicable, Country India, GST Registration type Regular (list: Unknown, Composition, Regular, Unregistered/Consumer), GSTIN/UIN empty, Place of Supply Not Applicable): Enter on each, as offered (`k1-5-party-details`). The voucher then showed **30-Apr-25 Wednesday**, the party's Current balance 8,79,12,000.00 Dr (`k1-6-date-before-amount`).
- Particulars: `BRIDGE`, space, `Export`, space, `Sales` typed as keystrokes; the list showed End of List and the one row, highlighted; Enter. No Inventory Allocations box.
- Amount: `$100` typed, Enter. The **Forex Rate of Exchange** box: offered `$100.00 @ (rupee) 88/$`, Value in (rupee) `8,800.00` (`k1-7-forex-box`). Enter on Forex Amount; `88` typed over the rate, Enter; Value 8,800.00 as offered, Enter. The line `BRIDGE Export Sales $100.00`; total `$100.00 @ (rupee) 88/$ = (rupee) 8,800.00` (`k1-8-line-100-at-88`).
- Enter on the empty second line, then Enter on the total: **Bill-wise Details for : BRIDGE USD Customer**, "Up to: (rupee) 8,800.00 Dr"; columns Type of Ref, Name, Due Date or Credit Days (wef: 30-Apr-25), Amount, Dr/Cr; Method of Adj. list Advance, Agst Ref, New Ref (highlighted), On Account (`k1-9-billwise`). New Ref, Enter; Name offered `4`, `LD1` typed over it, Enter; Due Date empty, Enter; Amount $100.00 as offered, Enter; Dr, Enter. One line: New Ref, LD1, $100.00 Dr.
- Narration left empty. Before Ctrl+A (`k1-10-before-accept`): no gain or loss line. Ctrl+A: saved, no prompt; a blank Sales No. 5 (`k1-11-after-save`). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; date of last entry 30-Apr-25 (`k1-12-gateway-after`).

**K2, Receipt.**
- The Gateway, a click on Tally's title bar, `v`: a blank Sales No. 5 opened (`k2-0-sales-no5-opened`). F6: Receipt, **No. 4** offered; the To line empty, the ledger list open (`k2-1-receipt-no4`).
- To: `BRIDGE`, space, `INR`, space, `Customer` typed as keystrokes, the one row highlighted, Enter. The voucher already showed **30-Apr-25 Wednesday** (no F2 used; `k2-2-date-before-amount`). Cur Bal 3,000.00 Dr.
- Credit: `2000`, Enter. No Forex box.
- **Bill-wise Details for : BRIDGE INR Customer**, "Up to: (rupee) 2,000.00 Cr"; Agst Ref highlighted in Method of Adj., Enter (`k2-3-billwise`). **Pending Bills:** one row, `R1`, Bill Date 1-Apr-25, Due Date 1-Apr-25, Balance 3,000.00 Dr, Final Balance 3,000.00 Dr (`k2-4-pending-r1`); Enter. Amount offered 2,000.00 Cr; `2000` typed over it, Enter; Cr, Enter (`k2-5-r1-amount`). The box closed: Agst Ref R1 2,000.00 Cr; the party's Cur Bal then 1,000.00 Dr.
- Tally then offered **By Cash**, Debit 2,000.00, Cur Bal 5,000.00 Dr (`k2-6-by-cash-offered`): Enter on By; the ledger list with Cash highlighted (the field read Cash), Enter; Debit 2,000.00 as offered, Enter.
- Narration left empty. Before Ctrl+A (`k2-7-before-accept`): To BRIDGE INR Customer Credit 2,000.00, By Cash Debit 2,000.00, totals 2,000.00 each; no gain or loss line. Ctrl+A: saved, no prompt; a blank Receipt No. 5 (`k2-8-after-save`, 15:42:13Z). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; last entry 30-Apr-25 (`k2-9-gateway-after`).

## Steps 5 to 7: the reads
Each request's sha256 was checked against `SHA256.txt` just before its send (`evidence/sends-utc.txt`). From m1 to m1r nothing else was sent and no key was pressed in Tally; only `lm.py`, `tb.py`, `rows.py` and `cmpinfo.py` ran between the sends, and the window captures after tb3p and tb1p. The whole outputs are below.
- Kept beside the sheet's calls: `tb.py` on row 683-VD's tb3w and on row 683-TW's tb3 (`tb-vdtb3w.txt`, `tb-twtb3.txt`; each read a file only, ran once, exit 0), from which the two field lists the sheet's Record asks for were made; the gaps between sends (`gaps.txt`), from `sends-utc.txt`; and `tally-exe.txt`, read after the close.

## Step 8: the restore (C6)
- Before the shut: the BK re-hashed equal to the one recorded; `prep-c6.sh 8 bk-683pw`, with `LAB_DIR` (this row's lab folder) and `TALLY_DATA` (the Tally data folder) written in full on its command line, exit 0 (`evidence/prep-c6-8.txt`): the folder holding that file alone, `restore-plan.txt` written, the data folder listed (24 entries). The plan and listings stay on the lab PC.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company (`c6-1-after-shut`).
- The Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`; Specify Path (Up 3 times), `C:\lab\bk-683pw` typed in two keystroke chunks. The list showed one row: BRIDGE FOREXGROUP LAB (`<company number>`), 10-Nov-26 (8:59 pm) (`8-restore-list`). Enter on it; End of List, Enter.
- "Accept ?" (`8-restore-accept`): y. Then, word for word: "Company data folder (`<company number>`) already exists." / "Selecting Skip will cancel the Restore process for this company." / "Selecting Replace will overwrite the existing folder.", buttons "R: Replace" and "S: Skip" (`8-restore-replace`): r. Select Company followed, no other message (`8-after-replace`).
- The data folder after the restore: after=17, before=24 entries. Gone: `CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`, `TUPDATE.TSF` (the same seven as the earlier restores on this book). None new.
- Loaded alone: `BRIDGE`, space, `FOREXGROU` typed, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`8-load-select`), Enter, no prompt. The Gateway (`8-gateway-after-load`): current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25; no dialog.
- m2 and mv2 as in the Result.

## Steps 9 and 10: the date, and close (C10)
- Step 9 (`9-gateway`): as in the Result; nothing was pressed to change the date or the period.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 15:48:54Z).
- C2 at 15:48:55Z, after the last request: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B). No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; at 15:49:14Z no `tally.exe` ran and nothing listened on 9001. The three Bridge processes were not ended.

## Files
- `answers/`: this sitting's answers that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256). The sheet's copies of other rows' answers are not repeated here.
- `evidence/`: notes, the outputs of `lm.py`, `tb.py`, `rows.py`, `cmpinfo.py` and `prep-c6.sh`, the two field lists, the gaps between sends, `tally.exe`'s file data, the start-up check's hits. Process and port listings, the backup folder and data folder listings, and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers, with lab paths replaced. `prep-c6.sh` ran as the sheet's file, byte for byte, and is not repeated.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## lm.py, tb.py, rows.py, cmpinfo.py and prep-c6.sh outputs, whole

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

### x0: rows.py against row 683-LD's x0
```
a-ldx0-X-lx1-20250401.xml: 11556 B sha256 898ad76bf159ec8676deaefe4cf2a01672e297de5945bab4c924365705ccace3
  rows: 8
a-x0-X-lx1-20250401.xml: 11556 B sha256 898ad76bf159ec8676deaefe4cf2a01672e297de5945bab4c924365705ccace3
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### x0: cmpinfo.py against row 683-LD's x0
```
a-ldx0-X-lx1-20250401.xml: 11556 B sha256 898ad76bf159ec8676deaefe4cf2a01672e297de5945bab4c924365705ccace3
a-x0-X-lx1-20250401.xml: 11556 B sha256 898ad76bf159ec8676deaefe4cf2a01672e297de5945bab4c924365705ccace3
BYTE-EQUAL
exit 0
```

### x0: lm.py
```
a-x0-X-lx1-20250401.xml: HEADER/STATUS 1
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

### tb10: tb.py --sets against row 683-LD's tb10
```
a-ldtb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-ldtb10-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb10-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb10: rows.py against row 683-LD's tb10
```
a-ldtb10-T-tb1-20250401-20250430.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
a-tb10-T-tb1-20250401-20250430.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb10: cmpinfo.py against row 683-LD's tb10
```
a-ldtb10-T-tb1-20250401-20250430.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb10-T-tb1-20250401-20250430.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
BYTE-EQUAL
exit 0
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

### mv0: cmp against row 683-TW's mvy
```
a-mv0 byte-equal to a-twmvy
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

### x1: rows.py against row 683-LD's x1
```
a-ldx1-X-lx1-20250401.xml: 11556 B sha256 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
  rows: 8
a-x1-X-lx1-20250401.xml: 11556 B sha256 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
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

### tb3p: rows.py against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
  rows: 8
a-tb3p-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb3p: rows.py against row 683-VD's tb3w
```
a-vdtb3w-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
  rows: 8
a-tb3p-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
  rows: 8
ROW 'BRIDGE Import Purchases': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} ''
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '0.00'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-$200.00 @ ₹ 86/$ = -₹ 17400.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-17400.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-$200.00 @ ₹ 86/$ = -₹ 17400.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-17400.00'
ROW 'BRIDGE Lab Bank': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '$200.00 @ ₹ 87/$ = ₹ 17500.00'
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '17500.00'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-$1100.00 @ ₹ 87/$ = -₹ 96700.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-96700.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-$900.00 @ ₹ 88/$ = -₹ 79200.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-79200.00'
CONTENT DIFFER
ROW ORDER SAME
ELEMENT ORDER SAME
exit 2
```

### tb3p: rows.py against row 683-TW's tb3
```
a-twtb3-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
a-tb3p-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
  rows: 8
ROW 'BRIDGE Export Sales': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25'
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '86014641.25'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} ''
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '0.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '86014641.25'
ROW 'BRIDGE Import Purchases': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} ''
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '0.00'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-$200.00 @ ₹ 86/$ = -₹ 17400.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-17400.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-$200.00 @ ₹ 86/$ = -₹ 17400.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-17400.00'
ROW 'BRIDGE Lab Bank': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '$200.00 @ ₹ 87/$ = ₹ 17500.00'
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '17500.00'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-$1100.00 @ ₹ 87/$ = -₹ 96700.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-96700.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-$900.00 @ ₹ 88/$ = -₹ 79200.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-79200.00'
ROW 'BRIDGE USD Customer': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '$1100.00 @ ₹ 87/$ = ₹ 96700.00'
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '96700.00'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-$1000100.00 @ ₹ 87.9999/$ = -₹ 88008700.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-88008700.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-$999000.00 @ ₹ 88/$ = -₹ 87912000.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-87912000.00'
ROW 'BRIDGE USD Supplier': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '$200.00 @ ₹ 87.50/$ = ₹ 17500.00'
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '17500.00'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-$200.00 @ ₹ 87/$ = -₹ 17500.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-17500.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '$0.00 @ ₹ /$ = ₹ 0.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '0.00'
ROW 'Cash': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} ''
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '0.00'
ROW 'Profit & Loss A/c': content differs
  A only: /TBALCLOSING {'TYPE': 'Amount'} '$0.00 @ ₹ /$ = ₹ 0.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '0.00'
CONTENT DIFFER
ROW ORDER SAME
ELEMENT ORDER SAME
exit 2
```

### tb3p: cmpinfo.py against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-tb3p-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
BYTE-EQUAL
exit 0
```

### tb3p: cmpinfo.py against row 683-VD's tb3w
```
a-vdtb3w-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-tb3p-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
exit 2
```

### tb3p: tb.py
```
a-tb3p-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb3p: tb.py --sets against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3p-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-ldtb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3p-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb3p: tb.py --sets against row 683-VD's tb3w
```
a-vdtb3w-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3p-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-vdtb3w-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3p-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
  'BRIDGE Import Purchases': MIXED -> PLAIN
  'BRIDGE Lab Bank': MIXED -> PLAIN
SETS DIFFER
exit 0
```

### tb3p2: rows.py against tb3p
```
a-tb3p-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
  rows: 8
a-tb3p2-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb3p2: cmpinfo.py against tb3p
```
a-tb3p-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-tb3p2-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
BYTE-EQUAL
exit 0
```

### tb3p2: tb.py --sets against tb3p
```
a-tb3p-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3p2-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3p-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3p2-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb1p: tb.py
```
a-tb1p-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
  LEDGER rows: 8
  'BRIDGE Export Sales' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '$1000200.00 @ ₹ 86/$ = ₹ 86023441.25' COMPOSITE
    TBALCLOSING '$1000200.00 @ ₹ 86/$ = ₹ 86023441.25' COMPOSITE
  'BRIDGE INR Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-5000.00' PLAIN
    CREDITTOTALS '4000.00' PLAIN
    TBALCLOSING '-1000.00' PLAIN
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
    DEBITTOTALS '-$1000200.00 @ ₹ 87.9999/$ = -₹ 88017500.00' COMPOSITE
    CREDITTOTALS '$1100.00 @ ₹ 87/$ = ₹ 96700.00' COMPOSITE
    TBALCLOSING '-$999100.00 @ ₹ 88/$ = -₹ 87920800.00' COMPOSITE
  'BRIDGE USD Supplier' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-$200.00 @ ₹ 87/$ = -₹ 17500.00' COMPOSITE
    CREDITTOTALS '$200.00 @ ₹ 87.50/$ = ₹ 17500.00' COMPOSITE
    TBALCLOSING '$0.00 @ ₹ /$ = ₹ 0.00' COMPOSITE
  'Cash' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '-5000.00' PLAIN
    CREDITTOTALS '' EMPTY
    TBALCLOSING '-5000.00' PLAIN
  'Profit & Loss A/c' CURRENCYNAME '₹': MIXED
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '' EMPTY
    TBALCLOSING '$0.00 @ ₹ /$ = ₹ 0.00' COMPOSITE
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
exit 0
```

### tb1p: tb.py --sets against row 683-LD's tb1
```
a-ldtb1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb1p-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-ldtb1-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb1p-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb1p: rows.py against row 683-LD's tb1
```
a-ldtb1-T-tb1-20250401-20250430.xml: 14140 B sha256 47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f
  rows: 8
a-tb1p-T-tb1-20250401-20250430.xml: 14140 B sha256 47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb1p: cmpinfo.py against row 683-LD's tb1
```
a-ldtb1-T-tb1-20250401-20250430.xml: 14140 B sha256 47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f
a-tb1p-T-tb1-20250401-20250430.xml: 14140 B sha256 47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f
BYTE-EQUAL
exit 0
```

### tb1p: tb.py --sets against tb10
```
a-tb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb1p-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb10-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb1p-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### row 683-VD's tb3w: tb.py (beside the sheet's calls, for the field list)
```
a-vdtb3w-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb3p: the fields that differ from row 683-VD's tb3w
```
$ fields-vd.py evidence/tb-vdtb3w.txt evidence/tb-tb3p.txt (the tb.py fields that differ, row by row: row 683-VD's tb3 -> tb3p)
('BRIDGE Import Purchases', 'CREDITTOTALS') ('', 'EMPTY') -> ('0.00', 'PLAIN')
('BRIDGE Import Purchases', 'DEBITTOTALS') ('-$200.00 @ ₹ 86/$ = -₹ 17400.00', 'COMPOSITE') -> ('-17400.00', 'PLAIN')
('BRIDGE Import Purchases', 'TBALCLOSING') ('-$200.00 @ ₹ 86/$ = -₹ 17400.00', 'COMPOSITE') -> ('-17400.00', 'PLAIN')
('BRIDGE Import Purchases', 'class') MIXED -> PLAIN
('BRIDGE Lab Bank', 'CREDITTOTALS') ('$200.00 @ ₹ 87/$ = ₹ 17500.00', 'COMPOSITE') -> ('17500.00', 'PLAIN')
('BRIDGE Lab Bank', 'DEBITTOTALS') ('-$1100.00 @ ₹ 87/$ = -₹ 96700.00', 'COMPOSITE') -> ('-96700.00', 'PLAIN')
('BRIDGE Lab Bank', 'TBALCLOSING') ('-$900.00 @ ₹ 88/$ = -₹ 79200.00', 'COMPOSITE') -> ('-79200.00', 'PLAIN')
('BRIDGE Lab Bank', 'class') MIXED -> PLAIN
fields differing: 8 of 40
```

### row 683-TW's tb3: tb.py (beside the sheet's calls, for the field list)
```
a-twtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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
exit 0
```

### tb3p: the fields that differ from row 683-TW's tb3
```
$ fields-vd.py evidence/tb-twtb3.txt evidence/tb-tb3p.txt (the tb.py fields that differ, row by row: row 683-TW's tb3 -> tb3p)
('BRIDGE Export Sales', 'CREDITTOTALS') ('$1000100.00 @ ₹ 86/$ = ₹ 86014641.25', 'COMPOSITE') -> ('86014641.25', 'PLAIN')
('BRIDGE Export Sales', 'DEBITTOTALS') ('', 'EMPTY') -> ('0.00', 'PLAIN')
('BRIDGE Export Sales', 'TBALCLOSING') ('$1000100.00 @ ₹ 86/$ = ₹ 86014641.25', 'COMPOSITE') -> ('86014641.25', 'PLAIN')
('BRIDGE Export Sales', 'class') MIXED -> PLAIN
('BRIDGE Import Purchases', 'CREDITTOTALS') ('', 'EMPTY') -> ('0.00', 'PLAIN')
('BRIDGE Import Purchases', 'DEBITTOTALS') ('-$200.00 @ ₹ 86/$ = -₹ 17400.00', 'COMPOSITE') -> ('-17400.00', 'PLAIN')
('BRIDGE Import Purchases', 'TBALCLOSING') ('-$200.00 @ ₹ 86/$ = -₹ 17400.00', 'COMPOSITE') -> ('-17400.00', 'PLAIN')
('BRIDGE Import Purchases', 'class') MIXED -> PLAIN
('BRIDGE Lab Bank', 'CREDITTOTALS') ('$200.00 @ ₹ 87/$ = ₹ 17500.00', 'COMPOSITE') -> ('17500.00', 'PLAIN')
('BRIDGE Lab Bank', 'DEBITTOTALS') ('-$1100.00 @ ₹ 87/$ = -₹ 96700.00', 'COMPOSITE') -> ('-96700.00', 'PLAIN')
('BRIDGE Lab Bank', 'TBALCLOSING') ('-$900.00 @ ₹ 88/$ = -₹ 79200.00', 'COMPOSITE') -> ('-79200.00', 'PLAIN')
('BRIDGE Lab Bank', 'class') MIXED -> PLAIN
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
fields differing: 23 of 40
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

### step 8: prep-c6.sh, before the shut
```
Written 2026-11-10T15:44:24Z, before the restore of step 8 (C6).
folder C:\lab\bk-683pw (holds this one file), file TDBK1800_<N>.001, 37618 B, sha256 84287030aa24a2d6294d42c1b7db026779f61286abc755c71b40f5a11a30711f (equal to evidence/bk.txt)
company number <N>, BRIDGE FOREXGROUP LAB, GUID 1bb48be5-baba-45b2-90d8-7593244915a9
Restore Data Path <Tally data folder>; source: Specify Path, the bk-683pw folder
the Restore list must show BRIDGE FOREXGROUP LAB, its number, 10-Nov-26 (8:59 pm), the BK's time
two prompts expected, the only ones answered: "Accept ?" = y; "Company data folder (<N>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." = r (Replace)
data folder listing before the restore: evidence/8-data-before-restore.txt (24 entries)
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

### mv2: cmp against mv0
```
a-mv2 byte-equal to a-mv0
```

### the tally.exe file
```
$ the program file of the Tally that answered (read after C10, with Tally closed)
tally.exe file version 1.1.7.2; 18485952 B; sha256 856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73; last written 2026-10-25T05:34:45Z
```

### each send's start and answer, UTC
```
2026-11-10T15:27:55.690Z m0 M-marks.xml (hash equal to the list)
2026-11-10T15:27:56.211Z m0 answered
2026-11-10T15:28:04.439Z x0 X-lx1-20250401.xml (hash equal to the list)
2026-11-10T15:28:04.890Z x0 answered
2026-11-10T15:28:06.785Z tb10 T-tb1-20250401-20250430.xml (hash equal to the list)
2026-11-10T15:28:07.374Z tb10 answered
2026-11-10T15:28:09.432Z mv0 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T15:28:10.034Z mv0 answered
2026-11-10T15:30:15.727Z mb M-marks.xml (hash equal to the list)
2026-11-10T15:30:16.257Z mb answered
2026-11-10T15:31:53.494Z mc M-marks.xml (hash equal to the list)
2026-11-10T15:31:53.971Z mc answered
2026-11-10T15:42:45.940Z m1 M-marks.xml (hash equal to the list)
2026-11-10T15:42:46.339Z m1 answered
2026-11-10T15:42:48.318Z x1 X-lx1-20250401.xml (hash equal to the list)
2026-11-10T15:42:48.857Z x1 answered
2026-11-10T15:42:50.936Z tb3p T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T15:42:51.501Z tb3p answered
2026-11-10T15:42:59.846Z tb3p2 T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T15:43:00.427Z tb3p2 answered
2026-11-10T15:43:02.686Z tb1p T-tb1-20250401-20250430.xml (hash equal to the list)
2026-11-10T15:43:03.275Z tb1p answered
2026-11-10T15:43:35.354Z m1r M-marks.xml (hash equal to the list)
2026-11-10T15:43:35.787Z m1r answered
2026-11-10T15:48:06.807Z m2 M-marks.xml (hash equal to the list)
2026-11-10T15:48:07.402Z m2 answered
2026-11-10T15:48:09.034Z mv2 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T15:48:09.682Z mv2 answered
```

### each send's time to answer, and the gap before it
```
$ each send's time to answer and the gap from the previous send's answer to its start, seconds, from evidence/sends-utc.txt
m0: started 15:27:55.690Z
m0: answered in 0.5 s
x0: started 15:28:04.439Z; gap after m0 answered: 8.2 s
x0: answered in 0.5 s
tb10: started 15:28:06.785Z; gap after x0 answered: 1.9 s
tb10: answered in 0.6 s
mv0: started 15:28:09.432Z; gap after tb10 answered: 2.1 s
mv0: answered in 0.6 s
mb: started 15:30:15.727Z; gap after mv0 answered: 125.7 s
mb: answered in 0.5 s
mc: started 15:31:53.494Z; gap after mb answered: 97.2 s
mc: answered in 0.5 s
m1: started 15:42:45.940Z; gap after mc answered: 652.0 s
m1: answered in 0.4 s
x1: started 15:42:48.318Z; gap after m1 answered: 2.0 s
x1: answered in 0.5 s
tb3p: started 15:42:50.936Z; gap after x1 answered: 2.1 s
tb3p: answered in 0.6 s
tb3p2: started 15:42:59.846Z; gap after tb3p answered: 8.3 s
tb3p2: answered in 0.6 s
tb1p: started 15:43:02.686Z; gap after tb3p2 answered: 2.3 s
tb1p: answered in 0.6 s
m1r: started 15:43:35.354Z; gap after tb1p answered: 32.1 s
m1r: answered in 0.4 s
m2: started 15:48:06.807Z; gap after m1r answered: 271.0 s
m2: answered in 0.6 s
mv2: started 15:48:09.034Z; gap after m2 answered: 1.6 s
mv2: answered in 0.6 s
K2's save (15:42:13Z, by the capture k2-8-after-save) to tb3p's start: 37 s
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every answer (LAB-COMMON C4: not rows); the names with a count other than 0
a-m0-M-marks.xml: all 0 (names in all: 35)
a-x0-X-lx1-20250401.xml: all 0 (names in all: 35)
a-tb10-T-tb1-20250401-20250430.xml: all 0 (names in all: 35)
a-mv0-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-mb-M-marks.xml: all 0 (names in all: 35)
a-mc-M-marks.xml: all 0 (names in all: 35)
a-m1-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-x1-X-lx1-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb3p-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb3p2-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb1p-T-tb1-20250401-20250430.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-m1r-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-m2-M-marks.xml: all 0 (names in all: 35)
a-mv2-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-ldm1-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldm2-M-marks.xml: all 0 (names in all: 35)
a-ldtb1-T-tb1-20250401-20250430.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldtb10-T-tb1-20250401-20250430.xml: all 0 (names in all: 35)
a-ldtb3-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldx0-X-lx1-20250401.xml: all 0 (names in all: 35)
a-ldx1-X-lx1-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-twmvy-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-twtb3-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-vdtb3w-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
```

### sha256 of every answer
```
$ sha256sum and size of every answer of this sitting, in send order, and of the sheet's copies of earlier answers
a-m0-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-x0-X-lx1-20250401.xml 11556 B 898ad76bf159ec8676deaefe4cf2a01672e297de5945bab4c924365705ccace3
a-tb10-T-tb1-20250401-20250430.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-mv0-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-mb-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-mc-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-m1-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-x1-X-lx1-20250401.xml 11556 B 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
a-tb3p-T-tb3-20250401-20250401.xml 13528 B a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-tb3p2-T-tb3-20250401-20250401.xml 13528 B a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-tb1p-T-tb1-20250401-20250430.xml 14140 B 47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f
a-m1r-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-m2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-mv2-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-ldm1-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-ldm2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-ldtb1-T-tb1-20250401-20250430.xml 14140 B 47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f
a-ldtb10-T-tb1-20250401-20250430.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-ldtb3-T-tb3-20250401-20250401.xml 13528 B a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-ldx0-X-lx1-20250401.xml 11556 B 898ad76bf159ec8676deaefe4cf2a01672e297de5945bab4c924365705ccace3
a-ldx1-X-lx1-20250401.xml 11556 B 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
a-twmvy-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-twtb3-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-vdtb3w-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### m0
```
20:57:56	M-marks.xml	a-m0-M-marks.xml	curl 0	3404 bytes
```

### x0
```
20:58:04	X-lx1-20250401.xml	a-x0-X-lx1-20250401.xml	curl 0	11556 bytes
```

### tb10
```
20:58:07	T-tb1-20250401-20250430.xml	a-tb10-T-tb1-20250401-20250430.xml	curl 0	14140 bytes
```

### mv0
```
20:58:09	V-mvy-20250401-20260331.xml	a-mv0-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```

### mb
```
21:00:16	M-marks.xml	a-mb-M-marks.xml	curl 0	3404 bytes
```

### mc
```
21:01:53	M-marks.xml	a-mc-M-marks.xml	curl 0	3404 bytes
```

### m1
```
21:12:46	M-marks.xml	a-m1-M-marks.xml	curl 0	3404 bytes
```

### x1
```
21:12:48	X-lx1-20250401.xml	a-x1-X-lx1-20250401.xml	curl 0	11556 bytes
```

### tb3p
```
21:12:51	T-tb3-20250401-20250401.xml	a-tb3p-T-tb3-20250401-20250401.xml	curl 0	13528 bytes
```

### tb3p2
```
21:13:00	T-tb3-20250401-20250401.xml	a-tb3p2-T-tb3-20250401-20250401.xml	curl 0	13528 bytes
```

### tb1p
```
21:13:03	T-tb1-20250401-20250430.xml	a-tb1p-T-tb1-20250401-20250430.xml	curl 0	14140 bytes
```

### m1r
```
21:13:35	M-marks.xml	a-m1r-M-marks.xml	curl 0	3404 bytes
```

### m2
```
21:18:07	M-marks.xml	a-m2-M-marks.xml	curl 0	3404 bytes
```

### mv2
```
21:18:09	V-mvy-20250401-20260331.xml	a-mv2-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```
