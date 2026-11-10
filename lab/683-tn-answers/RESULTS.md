# Row 683-TN: with row 683-NO's reads and the Trial Balance to 30 April read once before the write (m0, tb10, mv0), the Trial Balance to 1 April read every ledger PLAIN after the two vouchers: tb3p was byte-equal to row 683-LD's tb3, tb3p2 to tb3p, and tb1p to row 683-LD's tb1

Run on 10 Nov 2026, 18:11:19Z to 18:28:09Z (23:41 to 23:58 IST), on TallyPrime single-user edition (the Gateway on 9001; licensed, no Educational mode shown; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/683-tn-sheet` (54fb8171), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own. Raw reads, one at a time through `run.py`; two vouchers keyed by hand; one backup, restored at the end; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release, compared across sittings). Every read was answered, each with STATUS 1, within a second (0.4 to 0.6 s); no dialog came up after any read.

This sitting had a Tally start of its own: Tally was started once from its desktop shortcut and closed by its own Quit at the end. Tally's start before this one was row 683-TA's (17:33:17Z, quit by C10 at 17:51:34Z); nothing ran on this PC's Tally between that quit and this start. The restore returned the book to the marks it was found with (11/214).

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214: SAME MARKS with row 683-LD's m2. EXPECT met.
- **The controls (step 2), each as EXPECT by content, and each byte-equal to its reference. Before the write the sitting read `m0`, `tb10` once, then `mv0` (this row's one variable against row 683-NO).**
  - **tb10**, sent 1.9 s after m0 answered: as row 683-LD's tb10 (SETS SAME, six MIXED and two PLAIN, no UNREAD; `rows.py` CONTENT SAME, both orders SAME, exit 0) and byte-equal to it (`cmpinfo.py` exit 0; 14,140 B, sha256 `1c4638e8…b7b7`, the bytes of row 683-TW's tb3 too). It passed by content, so no stop was due before the BK.
  - **mv0**, sent 1.9 s after tb10 answered: CAPTURE SAME with row 683-TW's mvy, vouchers movement 11, capture 11; byte-equal to it (59,524 B).
- **BK, mb, mc:** BK taken; mb SAME MARKS with m0; mc SAME MARKS with mb, with no key pressed between.
- **The write, as rows 683-VD's, 683-RO's, 683-XF's, 683-PW's, 683-XA's, 683-NO's and 683-TA's:** K1 saved as Sales No. 4, 30-Apr-25: `BRIDGE USD Customer` Dr, `BRIDGE Export Sales` $100.00 at 88 = 8,800.00 rupees, New Ref LD1. K2 saved as Receipt No. 4, 30-Apr-25: `BRIDGE INR Customer` Cr 2,000.00 Agst Ref R1 (R1 pending 3,000.00 Dr), `Cash` Dr 2,000.00 (offered by Tally, not typed). Each number was the one expected (4 and 4). No gain or loss line on either. No "Spelling Error" box. Neither save asked "Accept ?". Every capture of the write equals row 683-TA's of the same name, byte for byte or but for the blinking cursor block.
- **m1:** MARKS DIFFER with mc (exit 2), ALTVCHID 11 to 13, ALTMSTID 214 (unchanged); SAME MARKS with row 683-LD's m1. Both EXPECTs met.
- **x1 (the opening export at 1 April, the first send after m1, and the first opening export of the sitting): byte-equal to row 683-LD's x1** (`cmpinfo.py` exit 0; 11,556 B); `rows.py` CONTENT SAME, both orders SAME (exit 0); 8 LEDGER rows, each opening 0.00. EXPECT met.
- **tb3p (the question; current date 30-Apr-2025): as row 683-LD's tb3, and byte-equal to it** (`cmpinfo.py` exit 0; 13,528 B, sha256 `a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705`, the bytes of rows 683-LD, 683-PW, 683-XA and 683-NO); SETS SAME and `rows.py` CONTENT SAME with it (exit 0).
  - Every ledger PLAIN (eight); no MIXED; no UNREAD.
  - The Tally window captured at once after the answer (`tb3p-window`, 18:22:05Z, byte-equal to `k2-9-gateway-after`): the Gateway, current date Wednesday, 30-Apr-2025, last entry 30-Apr-25, no dialog.
  - Against row 683-VD's tb3w (13,740 B, two MIXED; the bytes of rows 683-VD, 683-RO, 683-XF's second sitting and 683-TA): `cmpinfo.py` exit 2, differing outside CMPINFO; `rows.py` CONTENT DIFFER (exit 2), both orders SAME; `tb.py --sets` SETS DIFFER, `BRIDGE Import Purchases` and `BRIDGE Lab Bank` MIXED there and PLAIN here. Every rupee figure is equal. Of 40 fields, 8 differ (`evidence/tb3p-fields-vs-683vd.txt`), all in those two ledgers; the list is line for line row 683-NO's.
  - Against row 683-TW's tb3 (the book before the write): `rows.py` CONTENT DIFFER (exit 2), with ELEMENT ORDER SAME: tb3p has row 683-TW's element order. Of 40 fields, 23 differ (`evidence/tb3p-fields-vs-683tw.txt`), byte-equal to row 683-NO's list.
- **tb3p2**, sent 7.2 s after tb3p answered: byte-equal to tb3p (`cmpinfo.py` exit 0); `rows.py` exit 0; SETS SAME. As estimated.
- **tb1p (Q2; the Trial Balance to 30 April): as row 683-LD's tb1, and byte-equal to it** (`cmpinfo.py` exit 0; 14,140 B, sha256 `47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f`); SETS SAME and `rows.py` CONTENT SAME, both orders SAME (exit 0): six MIXED (`BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`) and two PLAIN (`BRIDGE INR Customer`, `Cash`); SETS SAME also with row 683-LD's tb10. Against this row's tb10 (the same request before the write): SETS SAME; `rows.py` CONTENT DIFFER (exit 2) with both orders SAME; of 40 fields, 8 differ (`evidence/tb1p-fields-vs-tb10.txt`), each an amount the two vouchers moved (in `BRIDGE Export Sales`, `BRIDGE INR Customer`, `BRIDGE USD Customer` and `Cash`), and no ledger changed its kind. The Tally window at once after it (`tb1p-window`, 18:22:15Z, byte-equal to `tb3p-window`): the Gateway, Wednesday, 30-Apr-2025, last entry 30-Apr-25, no dialog. As estimated.
- **m1r:** SAME MARKS with m1 (13/214): the reads wrote nothing.
- **The restore (step 8):** `prep-c6.sh` exit 0 before the shut; by C6, the two expected prompts only. **m2:** SAME MARKS with m0 (11/214). **mv2:** CAPTURE SAME with mv0, 11 and 11, and byte-equal to mv0. So the book is as BK holds it.
- **Step 9:** current date Tuesday, 1-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 1-Apr-25: as at step 0. EXPECT met.

**By the sheet's Decides:** `x1` is as row 683-LD's, so the `x1` case does not apply; `tb10` and `mv0` are as row 683-LD's and row 683-TW's, so the step 2 case does not apply. This is the case "`tb3p` as row 683-NO's (every ledger PLAIN), `tb3p2` as `tb3p`": against row 683-NO (one read apart), the Trial Balance to 30 April read before the write does not keep `BRIDGE Import Purchases` and `BRIDGE Lab Bank` composite, while the one-day Trial Balance does (row 683-TA): the effect is tied to the one-day request read before the write, not to any Trial Balance, though both answered with the same bytes there (tb10 here and tb3a in row 683-TA were each sha256 `1c4638e8…b7b7`). This agrees with rows 683-LD and 683-PW, which read tb10 before the write after the opening export and read plain. PARTIAL: one book, one release; this sitting, row 683-NO's and row 683-TA's are each a Tally start of their own, so the comparison is across sittings; the reading now stands on four composite sittings (rows 683-VD, 683-RO, 683-XF's second and 683-TA) and five plain ones (rows 683-LD, 683-PW, 683-XA, 683-NO and this row). That goes to #683 first, and nothing is built on it here.
- **Q2:** tb1p is as row 683-LD's tb1 (six MIXED): the wider window kept its marks after the write, as in rows 683-LD, 683-PW, 683-XA, 683-NO and 683-TA, here after the same request was read before the write.
- What else differs from row 683-NO is recorded from the rows' notes, and none of it is taken as the cause:
  - tb10 was sent before mv0 (the row's variable);
  - the gaps after the write (below) were close to row 683-NO's: m1 came 13.4 s after K2's save capture here, 15.4 s there;
  - in K2 the To ledger's name was typed once here, each space its own key press, as in row 683-TA; in row 683-NO it was typed twice.
- **Timing, recorded and not set** (`evidence/gaps.txt`): m1 was sent 13.4 s after K2's save (18:21:44.1Z, the time the capture `k2-8-after-save` was saved; the Ctrl+A about 16 s before that, and the Ctrl+Q about 3 s before it, see Step 4), x1 2.1 s after m1 answered, tb3p 1.9 s after x1 answered, and tb3p 18.4 s after K2's save capture. Row 683-NO's were 15.4 s, 2.7 s, 2.5 s and 22 s; row 683-TA's (two MIXED) 81.3 s, 1.7 s, 1.9 s and 86 s; row 683-XF's second sitting's (two MIXED) 2.0 s, 2.1 s and 37 s; row 683-LD's (every ledger PLAIN) about 2 minutes from K2's save to its tb3. Before the write: tb10 1.9 s after m0 answered, mv0 1.9 s after tb10 (row 683-NO's: mv0 1.4 s after m0).

**The element order (recorded; the sheet decides nothing on it):**
- Within this sitting it held: tb10 against tb1p (both orders SAME), tb3p against tb3p2 (both orders SAME, byte-equal), and tb1p with the order of its reference (both orders SAME against row 683-LD's tb1, byte-equal).
- Every Trial Balance of this sitting (tb10, tb3p, tb3p2, tb1p) has `TBALCLOSING` before `TBALOPENING` in each ledger, row 683-TW's order, as in rows 683-XF's second sitting, 683-PW, 683-XA, 683-NO and 683-TA.

**The sheet's estimates:** the controls (tb10 with the content of row 683-LD's tb10, mv0 with that of row 683-TW's mvy), the write (Sales No. 4 and Receipt No. 4), m1 against row 683-LD's m1, x1 (byte-equal to row 683-LD's), tb3p2 (byte-equal to tb3p), tb1p (as row 683-LD's tb1) and the restore, each as estimated. tb3p had no estimate (the question).

## Deviations
1. **The start-up company was shut first.** Its own book loaded by itself, as the install is set to, and was shut before step 0 (the K: Company label, Shut, Enter on its one row, no prompt), on the run lead's standing word. Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).

## Step 0: start, check and load
- Sheet files: all 24 equal to `SHA256.txt` at 54fb8171 (sizes too). Every file but `ROW.md` and `SHA256.txt` is byte-equal to row 683-TA's sheet (cf9d799b), so `run.py`, `lm.py`, `tb.py`, `rows.py`, `cmpinfo.py` and `prep-c6.sh` are row 683-TA's.
- Before the start, the company's data folder was found as the one whose `Company.1800` holds the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 18:11:19Z, from its desktop shortcut, after tasklist showed none.
- C2 at 18:11:49Z: one `tally.exe`, which alone owns 9001; `GET /status` status=200, bytes=51.
- The start-up company was shut (deviation 1). The book was loaded alone: `BRIDGE`, space, `FOREXGROU` typed in the name filter, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25. The PC's date: 10 Nov 2026 (IST).

## Steps 1 to 3: m0, the controls, BK
- m0, tb10 and mv0 as in the Result, one send at a time, each straight after the previous answer's counters (`step2.sh`).
- **BK** (C5), to the new folder `C:\lab\bk-683tn` (made empty before the start): the Data label, Backup & Restore, Backup; Backspace to the path; Specify Path (Up 3 times), the path typed in two keystroke chunks, its row taken; BRIDGE FOREXGROUP LAB alone, End of List (Up 3 times); "Accept ?" captured (`bk-accept`, 18:14:25Z) and answered y; no other message; the Gateway again. The folder then held one file: `TDBK1800_<company number>.001`, 37,618 B, sha256 `8991dec99bb99938c0de347ca395251fe9a29d70bd4b55d7368df04d0859967a`, written at 23:44 IST; the Restore list was to show 10-Nov-26 (11:44 pm), and did.
- mb and mc: SAME MARKS, as in the Result.

## Step 4: the write, word for word as the screen showed it
`evidence/notes.txt` holds every line below with its time; each capture is named in `SCREENSHOTS.txt`. Each capture was started just before the next key batch and ran beside it; the time beside a capture is when its file was saved, a few seconds after it started, and each picture shows the screen before that next key. Every capture of K1 and K2 equals row 683-TA's of the same name: 12 byte for byte, the other 11 but for the blinking cursor block (at most 20 by 13 pixels).

**K1, Sales.**
- The Gateway, a click on Tally's title bar, `v` (Vouchers): Accounting Voucher Creation opened as Payment No. 3 (`k1-0-payment-no3`). F8: Sales, **No. 4** offered (`k1-1-sales-no4`).
- F2: the "Change Voucher Date" box, Voucher Date offered `2-4-2025`; `30-4-2025` typed, Enter (`k1-2-date-box`).
- Party A/c name: `BRIDGE`, space, `USD`, space, `Customer` typed as keystrokes, each space its own key press; one row, `BRIDGE USD Customer`, highlighted (`k1-3-party-typed`); Enter. No "Spelling Error" box.
- "Dispatch Details" box (Delivery Note No(s), Dispatch Doc No., Dispatched through, Destination, Carrier Name/Agent, Bill of Lading/LR-RR No. and Date, Motor Vehicle No.; all empty): Enter on each, all left empty (`k1-4-dispatch-details`).
- "Party Details" box (Buyer (Bill to) BRIDGE USD Customer, Mailing Name BRIDGE USD Customer, Address empty, State Not Applicable, Country India, GST Registration type Regular, GSTIN/UIN empty, Place of Supply Not Applicable): Enter on each, as offered (`k1-5-party-details`). Every list was looked at before its Enter: List of States, Not Applicable highlighted (State and Place of Supply); List of Countries, India highlighted; GST Registration types Unknown, Composition, Regular (highlighted), Unregistered/Consumer. The voucher then showed **30-Apr-25 Wednesday**, the party's Current balance 8,79,12,000.00 Dr (`k1-6-date-before-amount`).
- Particulars: `BRIDGE`, space, `Export`, space, `Sales` typed as keystrokes; the one row highlighted; Enter. No Inventory Allocations box.
- Amount: `$100` typed, Enter. The **Forex Rate of Exchange** box: offered `$100.00 @ (rupee) 88/$`, Value in (rupee) `8,800.00` (`k1-7-forex-box`). Enter on Forex Amount; `88` typed over the rate, Enter; Value 8,800.00 as offered, Enter. The line `BRIDGE Export Sales $100.00`; total `$100.00 @ (rupee) 88/$ = (rupee) 8,800.00` (`k1-8-line-100-at-88`).
- Enter on the empty second line, then Enter on the total: **Bill-wise Details for : BRIDGE USD Customer**, "Up to: (rupee) 8,800.00 Dr"; columns Type of Ref, Name, Due Date or Credit Days (wef: 30-Apr-25), Amount, Dr/Cr; Method of Adj. list Advance, Agst Ref, New Ref (highlighted), On Account (`k1-9-billwise`). New Ref, Enter; Name offered `4`, `LD1` typed over it, Enter; Due Date empty, Enter; Amount $100.00 as offered, Enter; Dr, Enter. One line: New Ref, LD1, $100.00 Dr.
- Narration left empty. Before Ctrl+A (`k1-10-before-accept`): no gain or loss line. Ctrl+A: saved, no prompt; a blank Sales No. 5 (`k1-11-after-save`). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; date of last entry 30-Apr-25 (`k1-12-gateway-after`).

**K2, Receipt.**
- The Gateway, a click on Tally's title bar, `v`: a blank Sales No. 5 opened (`k2-0-sales-no5-opened`). F6: Receipt, **No. 4** offered; the To line empty, the ledger list open (`k2-1-receipt-no4`).
- To: `BRIDGE`, space, `INR`, space, `Customer` typed as keystrokes, each space its own key press; the one row, `BRIDGE INR Customer`, highlighted, Enter. The voucher already showed **30-Apr-25 Wednesday** (no F2 used; `k2-2-date-before-amount`). Cur Bal 3,000.00 Dr.
- Credit: `2000`, Enter. No Forex box.
- **Bill-wise Details for : BRIDGE INR Customer**, "Up to: (rupee) 2,000.00 Cr"; Agst Ref highlighted in Method of Adj., Enter (`k2-3-billwise`). **Pending Bills:** one row, `R1`, Bill Date 1-Apr-25, Due Date 1-Apr-25, Balance 3,000.00 Dr, Final Balance 3,000.00 Dr (`k2-4-pending-r1`); Enter. Amount offered 2,000.00 Cr; `2000` typed over it, Enter; Cr, Enter (`k2-5-r1-amount`). The box closed: Agst Ref R1 2,000.00 Cr; the party's Cur Bal then 1,000.00 Dr.
- Tally then offered **By Cash**, Debit 2,000.00, Cur Bal 5,000.00 Dr (`k2-6-by-cash-offered`): Enter on By; the ledger list with Cash highlighted (the field read Cash), Enter; Debit 2,000.00 as offered, Enter.
- Narration left empty. Before Ctrl+A (`k2-7-before-accept`): To BRIDGE INR Customer Credit 2,000.00, By Cash Debit 2,000.00, totals 2,000.00 each; no gain or loss line. Ctrl+A (about 18:21:28Z): saved, no prompt; a blank Receipt No. 5 (`k2-8-after-save`, saved 18:21:44Z). Ctrl+Q (about 18:21:41Z): no prompt; the Gateway: current date Wednesday, 30-Apr-2025; last entry 30-Apr-25 (`k2-9-gateway-after`, 18:21:56Z). `step6.sh` started straight after that capture.

## Steps 5 to 7: the reads
Each request's sha256 was checked against `SHA256.txt` just before its send (`evidence/sends-utc.txt`). From m1 to m1r nothing else was sent and no key was pressed in Tally; only `lm.py`, `tb.py`, `rows.py` and `cmpinfo.py` ran between the sends, and the window captures after tb3p and tb1p. The whole outputs are below.
- Kept beside the sheet's calls: `tb.py` on row 683-VD's tb3w, on row 683-TW's tb3 and on this row's tb10 (`tb-vdtb3w.txt`, `tb-twtb3.txt`, `tb-tb10.txt`; each read a file only, ran once, exit 0), from which the three field lists were made; the gaps between sends (`gaps.txt`), from `sends-utc.txt`; and `tally-exe.txt`, read after the close.

## Step 8: the restore (C6)
- Before the shut: the BK re-hashed equal to the one recorded; `prep-c6.sh 8 bk-683tn`, with `LAB_DIR` (this row's lab folder) and `TALLY_DATA` (the Tally data folder) written in full on its command line, exit 0 (`evidence/prep-c6-8.txt`): the folder holding that file alone, `restore-plan.txt` written, the data folder listed (24 entries). The plan and listings stay on the lab PC.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company (`c6-1-after-shut`).
- The Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`; Specify Path (Up 3 times), `C:\lab\bk-683tn` typed in two keystroke chunks. The list showed one row: BRIDGE FOREXGROUP LAB (`<company number>`), 10-Nov-26 (11:44 pm) (`8-restore-list`). Enter on it; End of List, Enter.
- "Accept ?" (`8-restore-accept`): y. Then, word for word: "Company data folder (`<company number>`) already exists." / "Selecting Skip will cancel the Restore process for this company." / "Selecting Replace will overwrite the existing folder.", buttons "R: Replace" and "S: Skip" (`8-restore-replace`): r. Select Company followed, no other message (`8-after-replace`).
- The data folder after the restore: after=17, before=24 entries. Gone: `CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`, `TUPDATE.TSF` (the same seven as the earlier restores on this book). None new.
- Loaded alone: `BRIDGE`, space, `FOREXGROU` typed, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`8-load-select`), Enter, no prompt. The Gateway (`8-gateway-after-load`): current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25; no dialog.
- m2 and mv2 as in the Result.

## Steps 9 and 10: the date, and close (C10)
- Step 9 (`9-gateway`): as in the Result; nothing was pressed to change the date or the period.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 18:27:46Z).
- C2 at 18:27:46Z, after the last request: one `tally.exe`, which alone owns 9001; `GET /status` status=200, bytes=51. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; at 18:28:09Z no `tally.exe` ran and nothing listened on 9001. The three Bridge processes were not ended.

## Files
- `answers/`: this sitting's answers that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256). The sheet's copies of other rows' answers are not repeated here.
- `evidence/`: notes, the outputs of `lm.py`, `tb.py`, `rows.py`, `cmpinfo.py` and `prep-c6.sh`, the three field lists, the gaps between sends, `tally.exe`'s file data, the start-up check's hits. Process and port listings, the backup folder and data folder listings, and the restore plan stay on the lab PC.
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

### tb1p: tb.py --sets against row 683-LD's tb10
```
a-ldtb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb1p-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-ldtb10-T-tb1-20250401-20250430.xml:
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

### tb1p: rows.py against this row's tb10
```
a-tb10-T-tb1-20250401-20250430.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
a-tb1p-T-tb1-20250401-20250430.xml: 14140 B sha256 47916021905a5bdedf18fcead25b2d0a0abee3682beffbc4d019e213d3dd237f
  rows: 8
ROW 'BRIDGE Export Sales': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25'
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '$1000200.00 @ ₹ 86/$ = ₹ 86023441.25'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '$1000200.00 @ ₹ 86/$ = ₹ 86023441.25'
ROW 'BRIDGE INR Customer': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '2000.00'
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '4000.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-1000.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-3000.00'
ROW 'BRIDGE USD Customer': content differs
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-$1000100.00 @ ₹ 87.9999/$ = -₹ 88008700.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-$1000200.00 @ ₹ 87.9999/$ = -₹ 88017500.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-$999000.00 @ ₹ 88/$ = -₹ 87912000.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-$999100.00 @ ₹ 88/$ = -₹ 87920800.00'
ROW 'Cash': content differs
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-3000.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-5000.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-3000.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-5000.00'
CONTENT DIFFER
ROW ORDER SAME
ELEMENT ORDER SAME
exit 2
```

### tb10: tb.py (beside the sheet's calls, for the field list)
```
a-tb10-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
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

### tb1p: the fields that differ from this row's tb10
```
$ fields-vd.py evidence/tb-tb10.txt evidence/tb-tb1p.txt (the tb.py fields that differ, row by row: this row's tb10, before the write -> tb1p, after it)
('BRIDGE Export Sales', 'CREDITTOTALS') ('$1000100.00 @ ₹ 86/$ = ₹ 86014641.25', 'COMPOSITE') -> ('$1000200.00 @ ₹ 86/$ = ₹ 86023441.25', 'COMPOSITE')
('BRIDGE Export Sales', 'TBALCLOSING') ('$1000100.00 @ ₹ 86/$ = ₹ 86014641.25', 'COMPOSITE') -> ('$1000200.00 @ ₹ 86/$ = ₹ 86023441.25', 'COMPOSITE')
('BRIDGE INR Customer', 'CREDITTOTALS') ('2000.00', 'PLAIN') -> ('4000.00', 'PLAIN')
('BRIDGE INR Customer', 'TBALCLOSING') ('-3000.00', 'PLAIN') -> ('-1000.00', 'PLAIN')
('BRIDGE USD Customer', 'DEBITTOTALS') ('-$1000100.00 @ ₹ 87.9999/$ = -₹ 88008700.00', 'COMPOSITE') -> ('-$1000200.00 @ ₹ 87.9999/$ = -₹ 88017500.00', 'COMPOSITE')
('BRIDGE USD Customer', 'TBALCLOSING') ('-$999000.00 @ ₹ 88/$ = -₹ 87912000.00', 'COMPOSITE') -> ('-$999100.00 @ ₹ 88/$ = -₹ 87920800.00', 'COMPOSITE')
('Cash', 'DEBITTOTALS') ('-3000.00', 'PLAIN') -> ('-5000.00', 'PLAIN')
('Cash', 'TBALCLOSING') ('-3000.00', 'PLAIN') -> ('-5000.00', 'PLAIN')
fields differing: 8 of 40
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
$ fields-vd.py evidence/tb-vdtb3w.txt evidence/tb-tb3p.txt (the tb.py fields that differ, row by row: row 683-VD's tb3w -> tb3p)
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
Written 2026-11-10T18:22:36Z, before the restore of step 8 (C6).
folder C:\lab\bk-683tn (holds this one file), file TDBK1800_<N>.001, 37618 B, sha256 8991dec99bb99938c0de347ca395251fe9a29d70bd4b55d7368df04d0859967a (equal to evidence/bk.txt)
company number <N>, BRIDGE FOREXGROUP LAB, GUID 1bb48be5-baba-45b2-90d8-7593244915a9
Restore Data Path <Tally data folder>; source: Specify Path, the bk-683tn folder
the Restore list must show BRIDGE FOREXGROUP LAB, its number, 10-Nov-26 (11:44 pm), the BK's time
two prompts expected, the only ones answered: "Accept ?" = y; "Company data folder (<N>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." = r (Replace)
data folder listing before the restore: evidence/8-data-before-restore.txt (24 entries)
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
2026-11-10T18:12:43.606Z m0 M-marks.xml (hash equal to the list)
2026-11-10T18:12:44.067Z m0 answered
2026-11-10T18:12:45.936Z tb10 T-tb1-20250401-20250430.xml (hash equal to the list)
2026-11-10T18:12:46.408Z tb10 answered
2026-11-10T18:12:48.315Z mv0 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T18:12:48.850Z mv0 answered
2026-11-10T18:14:44.576Z mb M-marks.xml (hash equal to the list)
2026-11-10T18:14:44.973Z mb answered
2026-11-10T18:14:46.187Z mc M-marks.xml (hash equal to the list)
2026-11-10T18:14:46.673Z mc answered
2026-11-10T18:21:57.453Z m1 M-marks.xml (hash equal to the list)
2026-11-10T18:21:57.932Z m1 answered
2026-11-10T18:21:59.995Z x1 X-lx1-20250401.xml (hash equal to the list)
2026-11-10T18:22:00.560Z x1 answered
2026-11-10T18:22:02.469Z tb3p T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T18:22:02.923Z tb3p answered
2026-11-10T18:22:10.089Z tb3p2 T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T18:22:10.564Z tb3p2 answered
2026-11-10T18:22:12.358Z tb1p T-tb1-20250401-20250430.xml (hash equal to the list)
2026-11-10T18:22:12.802Z tb1p answered
2026-11-10T18:22:26.819Z m1r M-marks.xml (hash equal to the list)
2026-11-10T18:22:27.244Z m1r answered
2026-11-10T18:26:53.561Z m2 M-marks.xml (hash equal to the list)
2026-11-10T18:26:53.927Z m2 answered
2026-11-10T18:26:55.280Z mv2 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T18:26:55.819Z mv2 answered
```

### each send's time to answer, and the gap before it
```
$ each send's time to answer and the gap from the previous send's answer to its start, seconds, from evidence/sends-utc.txt
m0: started 18:12:43.606Z
m0: answered in 0.5 s
tb10: started 18:12:45.936Z; gap after m0 answered: 1.9 s
tb10: answered in 0.5 s
mv0: started 18:12:48.315Z; gap after tb10 answered: 1.9 s
mv0: answered in 0.5 s
mb: started 18:14:44.576Z; gap after mv0 answered: 115.7 s
mb: answered in 0.4 s
mc: started 18:14:46.187Z; gap after mb answered: 1.2 s
mc: answered in 0.5 s
m1: started 18:21:57.453Z; gap after mc answered: 430.8 s
m1: answered in 0.5 s
x1: started 18:21:59.995Z; gap after m1 answered: 2.1 s
x1: answered in 0.6 s
tb3p: started 18:22:02.469Z; gap after x1 answered: 1.9 s
tb3p: answered in 0.5 s
tb3p2: started 18:22:10.089Z; gap after tb3p answered: 7.2 s
tb3p2: answered in 0.5 s
tb1p: started 18:22:12.358Z; gap after tb3p2 answered: 1.8 s
tb1p: answered in 0.4 s
m1r: started 18:22:26.819Z; gap after tb1p answered: 14.0 s
m1r: answered in 0.4 s
m2: started 18:26:53.561Z; gap after m1r answered: 266.3 s
m2: answered in 0.4 s
mv2: started 18:26:55.280Z; gap after m2 answered: 1.4 s
mv2: answered in 0.5 s
K2's save (18:21:44Z, by the capture k2-8-after-save, the time its file was saved) to m1's start: 13.4 s
K2's save (18:21:44Z, by the capture k2-8-after-save, the time its file was saved) to tb3p's start: 18.4 s
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every answer (LAB-COMMON C4: not rows); the names with a count other than 0
a-m0-M-marks.xml: all 0 (names in all: 35)
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
23:42:43	M-marks.xml	a-m0-M-marks.xml	curl 0	3404 bytes
```

### tb10
```
23:42:46	T-tb1-20250401-20250430.xml	a-tb10-T-tb1-20250401-20250430.xml	curl 0	14140 bytes
```

### mv0
```
23:42:48	V-mvy-20250401-20260331.xml	a-mv0-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```

### mb
```
23:44:44	M-marks.xml	a-mb-M-marks.xml	curl 0	3404 bytes
```

### mc
```
23:44:46	M-marks.xml	a-mc-M-marks.xml	curl 0	3404 bytes
```

### m1
```
23:51:57	M-marks.xml	a-m1-M-marks.xml	curl 0	3404 bytes
```

### x1
```
23:52:00	X-lx1-20250401.xml	a-x1-X-lx1-20250401.xml	curl 0	11556 bytes
```

### tb3p
```
23:52:02	T-tb3-20250401-20250401.xml	a-tb3p-T-tb3-20250401-20250401.xml	curl 0	13528 bytes
```

### tb3p2
```
23:52:10	T-tb3-20250401-20250401.xml	a-tb3p2-T-tb3-20250401-20250401.xml	curl 0	13528 bytes
```

### tb1p
```
23:52:12	T-tb1-20250401-20250430.xml	a-tb1p-T-tb1-20250401-20250430.xml	curl 0	14140 bytes
```

### m1r
```
23:52:27	M-marks.xml	a-m1r-M-marks.xml	curl 0	3404 bytes
```

### m2
```
23:56:53	M-marks.xml	a-m2-M-marks.xml	curl 0	3404 bytes
```

### mv2
```
23:56:55	V-mvy-20250401-20260331.xml	a-mv2-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```
