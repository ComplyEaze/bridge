# Row 683-LD: two vouchers dated 30 April 2025 on the forex book; the movement window keeps the dollar entry composite and the sets do not move, but a Trial Balance ending on 1 April changed with them

Run on 10 Nov 2026, 00:54:19Z to 01:22:14Z (06:24 to 06:52 IST), on TallyPrime Silver (the Gateway on 9001; licensed, no Educational mode shown; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/683-ld-sheet` (c8db1931), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own. Raw reads, one at a time through `run.py`; two vouchers keyed by hand; one backup, restored at the end; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Every read was answered, each with STATUS 1, within a second (0.5 to 1.1 s); no dialog came up after any read.

This sitting ran after row 2464-GR (Tally quit at 00:08:44Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The restore returned the book to the marks it was found with (11/214).

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214: SAME MARKS with row 683-TW's m0. EXPECT met.
- **The controls (step 2), each as EXPECT:** x0 byte-equal to row 683-LM's `lmlx1`, 8 rows each opening 0.00; tb10 SETS SAME with row 683-TW's tb1 (and byte-equal to it); mv0 CAPTURE SAME with row 683-TW's mvy, vouchers movement 11, capture 11 (and byte-equal to it).
- **BK, mb, mc:** BK taken; mb SAME MARKS with m0; mc SAME MARKS with mb, with no key pressed between.
- **The write:** K1 saved as Sales No. 4, 30-Apr-25: `BRIDGE USD Customer` Dr, `BRIDGE Export Sales` $100.00 at 88 = 8,800.00 rupees, New Ref LD1. K2 saved as Receipt No. 4, 30-Apr-25: `BRIDGE INR Customer` Cr 2,000.00 Agst Ref R1 (R1 pending 3,000.00 Dr), `Cash` Dr 2,000.00. Each number was the one expected (4 and 4). No gain or loss line on either. Two deviations, recorded below: K1's party field had a "Spelling Error" box, cleared on the run lead's word; neither save asked "Accept ?".
- **m1:** MARKS DIFFER (exit 2), ALTVCHID 11 to 13, ALTMSTID 214 (unchanged). EXPECT met.
- **x1:** not byte-equal to x0, but every byte that differs lies inside the CMPINFO count lines above the rows (LEDGER 0 to 6, CURRENCY 0 to 2, TAXUNIT 0 to 2, VOUCHERNUMBERSERIES 0 to 2, VOUCHER 0 to 3); all else, the 8 rows included, is byte-equal. 8 rows, each opening 0.00.
- **tb3 (1 April to 1 April): changed.** Not byte-equal to tb10 (13,528 B against 14,140 B), outside CMPINFO as well. Row 683-TW's tb3, the same request (sha256 `1853d0df...`), answered 14,140 B, byte-equal to tb1, with six ledgers MIXED. Now **every ledger is PLAIN**: each of the six reads its rupee figure alone where row 683-TW's tb3 read the dollar composite, and some fields that were EMPTY now read `0.00`. Every rupee figure is the same as before. Of 40 fields, 23 differ (`evidence/tb3-fields-vs-683tw.txt`). `tb.py --sets` against tb10: **SETS DIFFER**, the six MIXED to PLAIN. One other thing differed from row 683-TW's sitting: Tally's current date was 30-Apr-2025 here (moved by K1's voucher date) and 1-Apr-2025 there. This run does not say which of the two changed the answer.
- **tb1 (to 30 April):** SETS SAME against tb10. `Cash` PLAIN, `0.00`, `-5000.00`, `-5000.00`; `BRIDGE INR Customer` PLAIN, `0.00`, `-5000.00`, `4000.00`, `-1000.00`; the six others MIXED, with new composite texts (`BRIDGE Export Sales` `$1000200.00 @ ₹ 86/$ = ₹ 86023441.25`). No dialog after it.
- **tb4 (to 31 March 2026):** byte-equal to tb1; SETS SAME against tb10.
- **mv1:** 13 vouchers. `ld.py --day … 20250430`: Sales `'4'` ALL COMPOSITE (2) (`-$100.00 @ ₹ 88/$ = -₹ 8800.00` on the party, `$100.00 @ ₹ 88/$ = ₹ 8800.00` on sales); Receipt `'4'` ALL PLAIN (2) (`2000.00`, `-2000.00`); 2 vouchers on the day, 11 on other days. `lm.py --capture` against mv0: 11 SAME and the two new GUIDs "in the movement window only", so CAPTURE DIFFERS for that reason alone. No dialog after it.
- **The sums:** `Cash` `-5000.00 (3 entries on 3 vouchers)`; `BRIDGE INR Customer` `-1000.00 (3 entries on 3 vouchers)`.
- **mvlu1:** LEDGER SAME, 5 vouchers; K1 ALL COMPOSITE (2), 1 on the day, 4 on other days. **mvli1:** LEDGER SAME, 3 vouchers; K2 ALL PLAIN (2), 1 on the day, 2 on other days.
- **m1r:** SAME MARKS with m1 (13/214): the reads wrote nothing.
- **The restore (step 8):** by C6, the two expected prompts only. **m2:** SAME MARKS with m0 (11/214). **mv2:** CAPTURE SAME, 11 and 11, and byte-equal to mv0. So the book is as BK holds it.
- **Step 9:** current date Tuesday, 1-Apr-2025; period 1-Apr-25 to 31-Mar-26: as at step 0. EXPECT met.

**By the sheet's Decides** (the lines as they fall; the last one bears on all of them):
- **Q1, K1 ALL COMPOSITE (2) in mv1 and in mvlu1:** seen. Rule 2 sees a dollar entry on a later day in a window from the first day, on this book and release: PARTIAL.
- **Q2, SETS SAME for tb1 and tb4:** seen. Rule 1's sets for a first-day `from` do not move with a later-day entry, on this book: PARTIAL.
- **Q3:** each `--sum` equals TBALCLOSING minus TBALOPENING for that ledger, in tb1 and in tb4 (`Cash` -5000.00 = -5000.00 - 0.00; `BRIDGE INR Customer` -1000.00 = -1000.00 - 0.00); K2 ALL PLAIN (2); mvli1 LEDGER SAME: the kept ledgers tie over two voucher days: PARTIAL.
- **x1 or tb3 not byte-equal to its control:** **seen, for tb3** (x1 differs only in the CMPINFO count lines, which C4 says are not rows). A read ending on 1 April changed after vouchers dated 30 April were saved, and with Tally's current date moved to 30 April. By the sheet, the answers decide nothing until this is explained. The explanation is not part of this run.

**The sheet's estimates:** the controls as rows 683-LM and 683-TW read them; mv1, the sums, tb1 and tb4, mvlu1 and mvli1, and K1's figure, each as estimated. Not as estimated: tb3 (estimated byte-equal to tb10) and x1 (estimated byte-equal to x0, which it is outside CMPINFO).

## Deviations
1. **K1, the party field: a "Spelling Error" box.** The party name `BRIDGE USD Customer` was sent by the capture tool's type action, which went in as one clipboard paste. The field stayed empty. A box came up at the lower right of Tally's window, word for word: title **"Error"**, text **"Oops!"** and **"Spelling Error"** (`k1-3-spelling-error`). No key was pressed and the run lead was asked. On its word, nothing was pressed for about 5 s: the box stayed, word for word the same (`k1-4-box-after-5s`). On its word again, only the title bar was clicked: the box stayed (`k1-5-box-after-focus`). On its word (a), one `B` was typed as a single keystroke. The box closed and the field showed `B`. Then `RIDGE`, ` USD` and ` Customer` were typed as keystrokes, with a capture after each. The list then held one row, `BRIDGE USD Customer`, highlighted (`k1-6-party-typed`), and Enter took it. From then on, on the run lead's word, nothing was pasted into any voucher field. Every name and amount was typed in short keystroke chunks, and the highlighted row was checked before each Enter.
2. **Neither save asked "Accept ?".** The sheet expected Yes to an "Accept ?" prompt at each Ctrl+A. For both K1 and K2, Ctrl+A saved with no prompt and no other message, and a blank voucher of the same type followed (Sales No. 5, Receipt No. 5).

## Step 0: start, check and load
- Sheet files: all 21 equal to `SHA256.txt` at c8db1931. `LAB-COMMON.md`, `run.py`, `lm.py`, `tb.py`, `M-marks.xml`, the three Trial Balance requests and the three movement requests are byte-equal to row 683-TW's. The five `a-tw…` answers are byte-equal to row 683-TW's answers. `ld.py` holds `lm.py`'s say, stop, load and leaf and `tb.py`'s classes unchanged; its `read` is its own.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 00:54:19Z, from its desktop shortcut, after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- C2 at 00:55:04Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25. The PC's date: 10 Nov 2026 (IST).

## Steps 1 to 3: m0, the controls, BK
- m0, x0, tb10 and mv0 as in the Result.
- **BK** (C5), to the new folder `C:\lab\bk-683ld` (it did not exist before): the Data label, Backup & Restore, Backup; Backspace to the path; Specify Path (Up 3 times), the path typed, its row taken; BRIDGE FOREXGROUP LAB alone, End of List (Up 3 times); "Accept ?" captured (`bk-accept`) and answered y; no other message; the Gateway again. The folder then held one file: `TDBK1800_<company number>.001`, 37,618 B, sha256 `257cbcbb5a9fcda6608c614c8056b0c0b37f9f14b90765edf57cc9eb9e92ea71`, written at 06:27 IST; the Restore list was to show 10-Nov-26 (6:27 am), and did.
- mb and mc: SAME MARKS, as in the Result.

## Step 4: the write, word for word as the screen showed it
`evidence/notes.txt` holds every line below with its time; each capture is named in `SCREENSHOTS.txt`.

**K1, Sales.**
- The Gateway, `v` (Vouchers): Accounting Voucher Creation opened as Payment No. 3, the last type used. F8: Sales, **No. 4** offered (`k1-1-sales-no4`).
- F2: the "Change Voucher Date" box, Voucher Date offered `2-4-2025`; `30-4-2025` typed, Enter (`k1-2-date-box`). The voucher then showed **30-Apr-25 Wednesday**.
- Party A/c name: the deviation above; then `BRIDGE USD Customer`, Enter.
- "Dispatch Details" box (Delivery Note No(s), Dispatch Doc No., Dispatched through, Destination, Carrier Name/Agent, Bill of Lading/LR-RR No. and Date, Motor Vehicle No.; all empty): Enter on each, all left empty (`k1-7-dispatch-details`).
- "Party Details" box (Buyer (Bill to) BRIDGE USD Customer, Mailing Name BRIDGE USD Customer, Address empty, State Not Applicable, Country India, GST Registration type Regular, GSTIN/UIN empty, Place of Supply Not Applicable): Enter on each, as offered (`k1-8-party-details`). The date before the first amount: 30-Apr-25 (`k1-9-date-before-amount`). Current balance of the party: 8,79,12,000.00 Dr.
- Particulars: `BRIDGE Export Sales` typed as keystrokes, the one row highlighted, Enter. No Inventory Allocations box.
- Amount: `$100` typed, Enter. The **Forex Rate of Exchange** box: columns Forex Amount, Rate of Exchange, Value in (rupee sign); offered `$100.00 @ (rupee) 88/$`, `(rupee) 8,800.00` (`k1-10-forex-box`). Enter on Forex Amount; `88` typed over the rate, Enter; Value 8,800.00 as offered, Enter. Total line `$100.00 @ (rupee) 88/$ = (rupee) 8,800.00`.
- **Bill-wise Details for : BRIDGE USD Customer**, "Up to: (rupee) 8,800.00 Dr"; columns Type of Ref, Name, Due Date or Credit Days (wef: 30-Apr-25), Amount, Dr/Cr; Method of Adj. list Advance, Agst Ref, New Ref (highlighted), On Account (`k1-11-billwise`). New Ref, Enter; Name offered `4`, `LD1` typed over it, Enter; Due Date empty, Enter; Amount $100.00 as offered, Enter; Dr, Enter. One line: New Ref, LD1, $100.00 Dr.
- Narration left empty. Before Ctrl+A (`k1-before-accept`): no gain or loss line. Ctrl+A: saved, no prompt (deviation 2); a blank Sales No. 5 (`k1-after-save`). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; date of last entry 30-Apr-25 (`k1-gateway-after`).

**K2, Receipt.**
- The Gateway, `v`, F6: Receipt, **No. 4** offered (`k2-1-receipt-no4`).
- To: `BRIDGE INR Customer` typed as keystrokes, the one row highlighted, Enter. The voucher already showed **30-Apr-25 Wednesday** (no F2 used; `k2-2-date-before-amount`). Cur Bal 3,000.00 Dr.
- Credit: `2000`, Enter. No Forex box.
- **Bill-wise Details for : BRIDGE INR Customer**, "Up to: (rupee) 2,000.00 Cr"; Agst Ref highlighted in Method of Adj., Enter (`k2-3-billwise`). **Pending Bills:** one row, `R1`, Bill Date 1-Apr-25, Due Date 1-Apr-25, Balance 3,000.00 Dr, Final Balance 3,000.00 Dr (`k2-4-pending-r1`); Enter. Amount offered 2,000.00 Cr; `2000` typed over it, Enter; Cr, Enter (`k2-5-r1-amount`). The box closed: Agst Ref R1 2,000.00 Cr.
- Tally then offered **By Cash**, Debit 2,000.00, Cur Bal 5,000.00 Dr (`k2-6-by-cash-offered`): Enter on By; the ledger list with Cash highlighted (the field read Cash), Enter; Debit 2,000.00 as offered, Enter. Cash was offered by Tally, not typed.
- Narration left empty. Before Ctrl+A (`k2-before-accept`): To BRIDGE INR Customer Credit 2,000.00, By Cash Debit 2,000.00, totals 2,000.00 each; no gain or loss line. Ctrl+A: saved, no prompt (deviation 2); a blank Receipt No. 5 (`k2-after-save`). Ctrl+Q: no prompt; the Gateway as after K1 (`k2-gateway-after`).

## Steps 5 to 7: the reads
Each request's sha256 was checked against `SHA256.txt` just before its send (`evidence/sends-utc.txt`). Each send's times, answer size and sha256 are in `evidence/notes.txt`. The captures after tb1 (`after-tb1`) and after mv1 (`after-mv1`) show the Gateway, no dialog. The whole outputs are below. Two calls beyond the sheet's are kept beside them: `tb.py` on tb3 alone, and the byte comparison of x1, tb3 and tb1 with their controls (`scripts/cmpinfo.py`).

## Step 8: the restore (C6)
- Before the shut: the BK re-hashed equal to the one recorded, its folder holding that file alone; `restore-plan.txt` written; the data folder listed (24 entries). The plan and listings stay on the lab PC.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt.
- The Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`; Specify Path (Up 3 times), `C:\lab\bk-683ld` typed. The list showed one row: BRIDGE FOREXGROUP LAB (`<company number>`), 10-Nov-26 (6:27 am) (`8-restore-list`). Enter on it; End of List, Enter.
- "Accept ?" (`8-restore-accept`): y. Then, word for word: "Company data folder (`<company number>`) already exists." / "Selecting Skip will cancel the Restore process for this company." / "Selecting Replace will overwrite the existing folder.", buttons "R Replace" and "S Skip" (`8-restore-replace`): r. Select Company followed, no other message.
- The data folder after the restore: after=17, before=24 entries. Gone: `CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`, `TUPDATE.TSF` (the same seven as row 683-OP's restore). None new.
- Loaded alone: one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`8-select`), Enter, no prompt. The Gateway (`8-gateway`): current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25.
- m2 and mv2 as in the Result.

## Steps 9 and 10: the date, and close (C10)
- Step 9: as in the Result; nothing was pressed to change the date or the period.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 01:21:49Z).
- C2 at 01:21:50Z, after the last request: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B). No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; no `tally.exe` ran after. The three Bridge processes were not ended.

## Files
- `answers/`: this row's answers that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256). The sheet's copies of other rows' answers are not repeated here.
- `evidence/`: notes, the outputs of `lm.py`, `tb.py`, `ld.py` and `cmpinfo.py`, the start-up check's hits. Process and port listings, the backup folder and data folder listings, and the restore plan stay on the lab PC.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## Outputs, whole

### m0: lm.py --same-row, against row 683-TW's m0
```
a-twm0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
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

### tb10: tb.py --sets, against row 683-TW's tb1
```
a-twtb1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-twtb1-T-tb1-20250401-20250430.xml:
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

### mv0: lm.py --capture, against row 683-TW's mvy
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

### mb: lm.py --same-row, against m0
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

### mc: lm.py --same-row, against mb
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

### m1: lm.py --same-row, against mc
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

### x1: cmpinfo.py, against x0
```
a-x0-X-lx1-20250401.xml: 11556 B sha256 898ad76bf159ec8676deaefe4cf2a01672e297de5945bab4c924365705ccace3
a-x1-X-lx1-20250401.xml: 11556 B sha256 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
NOT BYTE-EQUAL; outside CMPINFO: EQUAL
  CMPINFO LEDGER: 0 -> 6
  CMPINFO CURRENCY: 0 -> 2
  CMPINFO TAXUNIT: 0 -> 2
  CMPINFO VOUCHERNUMBERSERIES: 0 -> 2
  CMPINFO VOUCHER: 0 -> 3
exit 1
```

### tb3: cmpinfo.py, against tb10
```
a-tb10-T-tb1-20250401-20250430.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb3-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
  CMPINFO LEDGER: 0 -> 6
  CMPINFO CURRENCY: 0 -> 2
  CMPINFO TAXUNIT: 0 -> 2
  CMPINFO VOUCHERNUMBERSERIES: 0 -> 2
  CMPINFO VOUCHER: 0 -> 3
exit 2
```

### tb3: tb.py --sets, against tb10
```
a-tb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb10-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
  'BRIDGE Export Sales': MIXED -> PLAIN
  'BRIDGE Import Purchases': MIXED -> PLAIN
  'BRIDGE Lab Bank': MIXED -> PLAIN
  'BRIDGE USD Customer': MIXED -> PLAIN
  'BRIDGE USD Supplier': MIXED -> PLAIN
  'Profit & Loss A/c': MIXED -> PLAIN
SETS DIFFER
exit 0
```

### tb3: tb.py (beyond the sheet)
```
a-tb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb3: its fields against row 683-TW's tb3
```
$ python -I (inline) tb3's tb.py fields, row 683-TW's tb3 (its RESULTS, the same request) -> this row's tb3 (tb-tb3.txt); each line: (ledger, field) TW's value and class -> this row's
('BRIDGE Export Sales', 'class') MIXED -> PLAIN
('BRIDGE Export Sales', 'DEBITTOTALS') ('', 'EMPTY') -> ('0.00', 'PLAIN')
('BRIDGE Export Sales', 'CREDITTOTALS') ('$1000100.00 @ ₹ 86/$ = ₹ 86014641.25', 'COMPOSITE') -> ('86014641.25', 'PLAIN')
('BRIDGE Export Sales', 'TBALCLOSING') ('$1000100.00 @ ₹ 86/$ = ₹ 86014641.25', 'COMPOSITE') -> ('86014641.25', 'PLAIN')
('BRIDGE Import Purchases', 'class') MIXED -> PLAIN
('BRIDGE Import Purchases', 'DEBITTOTALS') ('-$200.00 @ ₹ 86/$ = -₹ 17400.00', 'COMPOSITE') -> ('-17400.00', 'PLAIN')
('BRIDGE Import Purchases', 'CREDITTOTALS') ('', 'EMPTY') -> ('0.00', 'PLAIN')
('BRIDGE Import Purchases', 'TBALCLOSING') ('-$200.00 @ ₹ 86/$ = -₹ 17400.00', 'COMPOSITE') -> ('-17400.00', 'PLAIN')
('BRIDGE Lab Bank', 'class') MIXED -> PLAIN
('BRIDGE Lab Bank', 'DEBITTOTALS') ('-$1100.00 @ ₹ 87/$ = -₹ 96700.00', 'COMPOSITE') -> ('-96700.00', 'PLAIN')
('BRIDGE Lab Bank', 'CREDITTOTALS') ('$200.00 @ ₹ 87/$ = ₹ 17500.00', 'COMPOSITE') -> ('17500.00', 'PLAIN')
('BRIDGE Lab Bank', 'TBALCLOSING') ('-$900.00 @ ₹ 88/$ = -₹ 79200.00', 'COMPOSITE') -> ('-79200.00', 'PLAIN')
('BRIDGE USD Customer', 'class') MIXED -> PLAIN
('BRIDGE USD Customer', 'DEBITTOTALS') ('-$1000100.00 @ ₹ 87.9999/$ = -₹ 88008700.00', 'COMPOSITE') -> ('-88008700.00', 'PLAIN')
('BRIDGE USD Customer', 'CREDITTOTALS') ('$1100.00 @ ₹ 87/$ = ₹ 96700.00', 'COMPOSITE') -> ('96700.00', 'PLAIN')
('BRIDGE USD Customer', 'TBALCLOSING') ('-$999000.00 @ ₹ 88/$ = -₹ 87912000.00', 'COMPOSITE') -> ('-87912000.00', 'PLAIN')
('BRIDGE USD Supplier', 'class') MIXED -> PLAIN
('BRIDGE USD Supplier', 'DEBITTOTALS') ('-$200.00 @ ₹ 87/$ = -₹ 17500.00', 'COMPOSITE') -> ('-17500.00', 'PLAIN')
('BRIDGE USD Supplier', 'CREDITTOTALS') ('$200.00 @ ₹ 87.50/$ = ₹ 17500.00', 'COMPOSITE') -> ('17500.00', 'PLAIN')
('BRIDGE USD Supplier', 'TBALCLOSING') ('$0.00 @ ₹ /$ = ₹ 0.00', 'COMPOSITE') -> ('0.00', 'PLAIN')
('Cash', 'CREDITTOTALS') ('', 'EMPTY') -> ('0.00', 'PLAIN')
('Profit & Loss A/c', 'class') MIXED -> PLAIN
('Profit & Loss A/c', 'TBALCLOSING') ('$0.00 @ ₹ /$ = ₹ 0.00', 'COMPOSITE') -> ('0.00', 'PLAIN')
fields differing 23 of 40
```

### tb1: tb.py
```
a-tb1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
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

### tb1: tb.py --sets, against tb10
```
a-tb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb10-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb1-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb1: cmpinfo.py, against tb10
```
a-tb10-T-tb1-20250401-20250430.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb1-T-tb1-20250401-20250430.xml: 14140 B sha256 47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
  CMPINFO LEDGER: 0 -> 6
  CMPINFO CURRENCY: 0 -> 2
  CMPINFO TAXUNIT: 0 -> 2
  CMPINFO VOUCHERNUMBERSERIES: 0 -> 2
  CMPINFO VOUCHER: 0 -> 3
exit 2
```

### tb4: tb.py
```
a-tb4-T-tb4-20250401-20260331.xml: HEADER/STATUS '1'
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

### tb4: tb.py --sets, against tb10
```
a-tb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb4-T-tb4-20250401-20260331.xml: HEADER/STATUS '1'
a-tb10-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb4-T-tb4-20250401-20260331.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### mv1: ld.py --day 20250430
```
a-mv1-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000c 20250430 Sales '4' ISCANCELLED No ISOPTIONAL No: 2 entries
    'BRIDGE USD Customer' Yes '-$100.00 @ ₹ 88/$ = -₹ 8800.00' COMPOSITE
    'BRIDGE Export Sales' No '$100.00 @ ₹ 88/$ = ₹ 8800.00' COMPOSITE
  Sales '4': ALL COMPOSITE (2)
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000d 20250430 Receipt '4' ISCANCELLED No ISOPTIONAL No: 2 entries
    'BRIDGE INR Customer' No '2000.00' PLAIN
    'Cash' Yes '-2000.00' PLAIN
  Receipt '4': ALL PLAIN (2)
DAY 20250430: 2 vouchers, 11 on other days
exit 0
```

### mv1: ld.py --sum Cash
```
a-mv1-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  1bb48be5-baba-45b2-90d8-7593244915a9-00000004 '-1000.00' PLAIN
  1bb48be5-baba-45b2-90d8-7593244915a9-00000006 '-2000.00' PLAIN
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000d '-2000.00' PLAIN
SUM Cash: -5000.00 (3 entries on 3 vouchers)
exit 0
```

### mv1: ld.py --sum "BRIDGE INR Customer"
```
a-mv1-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  1bb48be5-baba-45b2-90d8-7593244915a9-00000005 '-5000.00' PLAIN
  1bb48be5-baba-45b2-90d8-7593244915a9-00000006 '2000.00' PLAIN
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000d '2000.00' PLAIN
SUM BRIDGE INR Customer: -1000.00 (3 entries on 3 vouchers)
exit 0
```

### mv1: lm.py --capture, against mv0
```
a-mv1-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
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
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000c: in the movement window only
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000d: in the movement window only
vouchers: movement 13, capture 11
CAPTURE DIFFERS
exit 0
```

### mvlu1: lm.py --ledger, BRIDGE USD Customer, against mv1
```
a-mvlu1-V-mvly-usd-customer-20250401-20260331.xml: HEADER/STATUS 1
a-mv1-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  ledger 'BRIDGE USD Customer': ledger-scoped 5 vouchers, book-scoped with an entry on it 5
  1bb48be5-baba-45b2-90d8-7593244915a9-00000001: ledger-scoped ['-$1000000.00 @ ₹ 86/$ = -₹ 86000000.00'] book-scoped ['-$1000000.00 @ ₹ 86/$ = -₹ 86000000.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-00000002: ledger-scoped ['-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25'] book-scoped ['-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25']
  1bb48be5-baba-45b2-90d8-7593244915a9-00000003: ledger-scoped ['$100.00 @ ₹ 87/$ = ₹ 8700.00'] book-scoped ['$100.00 @ ₹ 87/$ = ₹ 8700.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000b: ledger-scoped ['$1000.00 @ ₹ 88/$ = ₹ 88000.00'] book-scoped ['$1000.00 @ ₹ 88/$ = ₹ 88000.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000c: ledger-scoped ['-$100.00 @ ₹ 88/$ = -₹ 8800.00'] book-scoped ['-$100.00 @ ₹ 88/$ = -₹ 8800.00']
LEDGER SAME
exit 0
```

### mvlu1: ld.py --day 20250430
```
a-mvlu1-V-mvly-usd-customer-20250401-20260331.xml: HEADER/STATUS 1
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000c 20250430 Sales '4' ISCANCELLED No ISOPTIONAL No: 2 entries
    'BRIDGE USD Customer' Yes '-$100.00 @ ₹ 88/$ = -₹ 8800.00' COMPOSITE
    'BRIDGE Export Sales' No '$100.00 @ ₹ 88/$ = ₹ 8800.00' COMPOSITE
  Sales '4': ALL COMPOSITE (2)
DAY 20250430: 1 vouchers, 4 on other days
exit 0
```

### mvli1: lm.py --ledger, BRIDGE INR Customer, against mv1
```
a-mvli1-V-mvly-inr-customer-20250401-20260331.xml: HEADER/STATUS 1
a-mv1-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  ledger 'BRIDGE INR Customer': ledger-scoped 3 vouchers, book-scoped with an entry on it 3
  1bb48be5-baba-45b2-90d8-7593244915a9-00000005: ledger-scoped ['-5000.00'] book-scoped ['-5000.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-00000006: ledger-scoped ['2000.00'] book-scoped ['2000.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000d: ledger-scoped ['2000.00'] book-scoped ['2000.00']
LEDGER SAME
exit 0
```

### mvli1: ld.py --day 20250430
```
a-mvli1-V-mvly-inr-customer-20250401-20260331.xml: HEADER/STATUS 1
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000d 20250430 Receipt '4' ISCANCELLED No ISOPTIONAL No: 2 entries
    'BRIDGE INR Customer' No '2000.00' PLAIN
    'Cash' Yes '-2000.00' PLAIN
  Receipt '4': ALL PLAIN (2)
DAY 20250430: 1 vouchers, 2 on other days
exit 0
```

### m1r: lm.py --same-row, against m1
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

### m2: lm.py --same-row, against m0
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

### mv2: lm.py --capture, against mv0
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
