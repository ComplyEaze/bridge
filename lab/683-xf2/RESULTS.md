# Row 683-XF, second sitting: with the opening export sent first after row 683-LD's two vouchers, the Trial Balance to 1 April still read two ledgers MIXED: tb3f was byte-equal to row 683-VD's tb3w, and tb3f2 to tb3f; the element order held within the sitting and was row 683-TW's

Run on 10 Nov 2026, 09:15:19Z to 09:33:32Z (14:45 to 15:03 IST), on TallyPrime Silver (the Gateway on 9001; licensed, no Educational mode shown; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/683-xf-sheet` (3b543335), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own. Raw reads, one at a time through `run.py`; two vouchers keyed by hand; one backup, restored at the end; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Every read was answered, each with STATUS 1, within a second (0.3 to 0.6 s); no dialog came up after any read.

This is the row's second sitting. The first (`lab/683-xf-answers` at 9eeeb1a7) stopped at step 2 with nothing written. This sitting had a Tally start of its own: Tally was started once from its desktop shortcut and closed by its own Quit at the end. Before it, Tally's start on this PC was the first sitting's (07:56:37Z, quit at 08:00:46Z); nothing ran on Tally in between. The restore returned the book to the marks it was found with (11/214).

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214: SAME MARKS with row 683-LD's m2. EXPECT met.
- **The controls (step 2), each as EXPECT:**
  - **tb3a** (current date 1-Apr-2025): SETS SAME and `rows.py` CONTENT SAME with row 683-TW's tb3 (exit 0: row order and element order SAME); six MIXED and two PLAIN. **Byte-equal to row 683-TW's tb3** (`cmpinfo.py` exit 0; 14,140 B), and not to the first sitting's tb3a (`cmpinfo.py` exit 2).
  - **tb3a2**, sent straight after tb3a's counters: the same four lines as tb3a. Against tb3a: `rows.py` exit 0 and `cmpinfo.py` exit 0 (byte-equal).
  - So both have row 683-TW's element order (`TBALCLOSING` before `TBALOPENING` in each ledger), not the first sitting's.
  - **mv0:** CAPTURE SAME with row 683-TW's mvy, vouchers movement 11, capture 11; byte-equal to it (59,524 B).
- **BK, mb, mc:** BK taken; mb SAME MARKS with m0; mc SAME MARKS with mb, with no key pressed between.
- **The write, as rows 683-VD's and 683-RO's:** K1 saved as Sales No. 4, 30-Apr-25: `BRIDGE USD Customer` Dr, `BRIDGE Export Sales` $100.00 at 88 = 8,800.00 rupees, New Ref LD1. K2 saved as Receipt No. 4, 30-Apr-25: `BRIDGE INR Customer` Cr 2,000.00 Agst Ref R1 (R1 pending 3,000.00 Dr), `Cash` Dr 2,000.00 (offered by Tally, not typed). Each number was the one expected (4 and 4). No gain or loss line on either. No "Spelling Error" box (every name was typed in keystroke chunks). Neither save asked "Accept ?".
- **m1:** MARKS DIFFER with mc (exit 2), ALTVCHID 11 to 13, ALTMSTID 214 (unchanged); SAME MARKS with row 683-LD's m1. Both EXPECTs met.
- **x1 (the opening export at 1 April, the first send after m1): byte-equal to row 683-LD's x1** (`cmpinfo.py` exit 0; 11,556 B); `rows.py` CONTENT SAME, both orders SAME (exit 0); 8 LEDGER rows, each opening 0.00. EXPECT met.
- **tb3f (the question; current date 30-Apr-2025): as row 683-VD's tb3w, and byte-equal to it** (`cmpinfo.py` exit 0; 13,740 B, sha256 `f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb`); SETS SAME and `rows.py` CONTENT SAME with it (exit 0).
  - `BRIDGE Import Purchases` and `BRIDGE Lab Bank` MIXED, the other six PLAIN; no UNREAD.
  - The Tally window captured at once after the answer (`tb3f-window`, 09:27:35Z): the Gateway, current date Wednesday, 30-Apr-2025, last entry 30-Apr-25, no dialog.
  - Against row 683-LD's tb3 (13,528 B, every ledger PLAIN): `cmpinfo.py` exit 2, differing outside CMPINFO; `rows.py` CONTENT DIFFER (exit 2); `tb.py --sets` SETS DIFFER, `BRIDGE Import Purchases` and `BRIDGE Lab Bank` PLAIN to MIXED. Every rupee figure is equal. Of 40 fields, 8 differ (`evidence/tb3f-fields-vs-683ld.txt`), all in those two ledgers and the same 8 as rows 683-VD's and 683-RO's lists.
  - Against row 683-TW's tb3: `rows.py` CONTENT DIFFER (exit 2), with ELEMENT ORDER SAME: tb3f has row 683-TW's element order.
- **tb3f2**, sent 7.2 s after tb3f answered: byte-equal to tb3f (`cmpinfo.py` exit 0); `rows.py` exit 0; SETS SAME. As estimated.
- **m1r:** SAME MARKS with m1 (13/214): the reads wrote nothing.
- **The restore (step 8):** by C6, the two expected prompts only. **m2:** SAME MARKS with m0 (11/214). **mv2:** CAPTURE SAME with mv0, 11 and 11, and byte-equal to mv0 (and to row 683-TW's mvy). So the book is as BK holds it.
- **Step 9:** current date Tuesday, 1-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 1-Apr-25: as at step 0. EXPECT met.

**By the sheet's Decides:** `x1` is as row 683-LD's, so the `x1` case does not apply. This is the case "`tb3f` as row 683-VD's (two MIXED), `tb3f2` as `tb3f`". In the sheet's words: "the opening export does not explain row 683-LD's all-plain `tb3` in either order."
- What still differs from row 683-LD is recorded from the rows' notes, and none of it is taken as the cause:
  - the reads before the write: row 683-LD's `x0` and its Trial Balance to 30 April, where this sitting sent `tb3a` and `tb3a2`, as rows 683-VD and 683-RO sent `tb3a`;
  - the keying of K1: row 683-LD's met a "Spelling Error" box and cleared it; rows 683-VD's, 683-RO's and this one met none, and m1 here is SAME MARKS with row 683-LD's m1.
- By the sheet, each candidate is its own row.
- **Timing, recorded and not set** (`evidence/gaps.txt`): x1 was sent 2.1 s after m1 answered, tb3f 2.1 s after x1 answered, and tb3f 37 s after K2's save (09:26:55Z). Row 683-LD's gaps were about 10 s and 58 s, and about 2 minutes from K2's save to its tb3.

**The element order (recorded; the sheet decides nothing on it):**
- Within this sitting it held: tb3a against tb3a2, and tb3f against tb3f2, both orders SAME, each pair byte-equal.
- It was row 683-TW's order in every Trial Balance of this sitting (tb3a, tb3a2, tb3f, tb3f2), and x1 had row 683-LD's x1 order.
- The first sitting's one Trial Balance (`a-xftb3a`) had the other order. The two sittings each had their own Tally start, with the same `tally.exe` file (version 1.1.7.2, last written 2026-10-25; `evidence/tally-exe.txt`) and nothing run on Tally between them.
- So on this book, between two Tally starts, the order differed, and within this start it held. What sets it is not known.

**The sheet's estimates:** the controls (with the content of row 683-TW's tb3 and mvy), the write (Sales No. 4 and Receipt No. 4), m1 against row 683-LD's m1, x1 (byte-equal to row 683-LD's), tb3f2 (byte-equal to tb3f) and the restore, each as estimated. Of the two element orders, the controls had the one 32 of the 33 earlier answer files have. tb3f had no estimate: it was the question.

## Deviations
1. **The start-up company was shut first.** Its own book loaded by itself, as the install is set to, and was shut by C9 before step 0 (the K: Company label, Shut, Enter on its one row, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
2. **One call beyond the sheet's was run twice.** `tb.py` on row 683-LD's tb3, for the field list, first ran without `-X utf8` and stopped on printing the rupee sign to the console (exit 1). It was run again with `-X utf8` (exit 0); its output is equal to row 683-RO's `tb-ldtb3.txt`. It reads a file only and sends nothing. The sheet's own calls all ran once.

## Step 0: start, check and load
- Sheet files: all 20 equal to `SHA256.txt` at 3b543335 (sizes too). Every file it shares with the first sitting's kit (2efa8f55) is byte-equal to it. `a-xftb3a` is byte-equal to the first sitting's tb3a. `rows.py` on `a-twtb3` against `a-xftb3a` gave CONTENT SAME with ELEMENT ORDER DIFFERS (8), exit 3, as the sheet says.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 09:15:19Z, from its desktop shortcut, after tasklist showed none.
- C2 at 09:15:41Z: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B).
- The start-up company was shut (deviation 1). The book was loaded alone: `BRIDGE`, space, `FOREXGROU` typed in the name filter, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25. The PC's date: 10 Nov 2026 (IST).

## Steps 1 to 3: m0, the controls, BK
- m0, tb3a, tb3a2 and mv0 as in the Result.
- **BK** (C5), to the new folder `C:\lab\bk-683xf2` (made empty before the start): the Data label, Backup & Restore, Backup; Backspace to the path; Specify Path (Up 3 times), the path typed in two keystroke chunks, its row taken; BRIDGE FOREXGROUP LAB alone, End of List (Up 3 times); "Accept ?" captured (`bk-accept`, 09:18:30Z) and answered y; no other message; the Gateway again. The folder then held one file: `TDBK1800_<company number>.001`, 37,618 B, sha256 `e17d8e4c984f12db9bb4b0de1f55318cde2619904fe2710512fa17d00e58c233`, written at 14:48 IST; the Restore list was to show 10-Nov-26 (2:48 pm), and did.
- mb and mc: SAME MARKS, as in the Result.

## Step 4: the write, word for word as the screen showed it
`evidence/notes.txt` holds every line below with its time; each capture is named in `SCREENSHOTS.txt`.

**K1, Sales.**
- The Gateway, a click on Tally's title bar, `v` (Vouchers): Accounting Voucher Creation opened as Payment No. 3 (`k1-0-payment-no3`). F8: Sales, **No. 4** offered (`k1-1-sales-no4`).
- F2: the "Change Voucher Date" box, Voucher Date offered `2-4-2025`; `30-4-2025` typed, Enter (`k1-2-date-box`).
- Party A/c name: `BRIDGE`, space, `USD`, space, `Customer` typed as keystrokes; one row, `BRIDGE USD Customer`, highlighted (`k1-3-party-typed`); Enter. No "Spelling Error" box.
- "Dispatch Details" box (Delivery Note No(s), Dispatch Doc No., Dispatched through, Destination, Carrier Name/Agent, Bill of Lading/LR-RR No. and Date, Motor Vehicle No.; all empty): Enter on each, all left empty (`k1-4-dispatch-details`).
- "Party Details" box (Buyer (Bill to) BRIDGE USD Customer, Mailing Name BRIDGE USD Customer, Address empty, State Not Applicable, Country India, GST Registration type Regular (list: Unknown, Composition, Regular, Unregistered/Consumer), GSTIN/UIN empty, Place of Supply Not Applicable): Enter on each, as offered (`k1-5-party-details`). The voucher then showed **30-Apr-25 Wednesday**, the party's Current balance 8,79,12,000.00 Dr (`k1-6-date-before-amount`).
- Particulars: `BRIDGE`, space, `Export`, space, `Sales` typed as keystrokes, the one row highlighted, Enter. No Inventory Allocations box.
- Amount: `$100` typed, Enter. The **Forex Rate of Exchange** box: offered `$100.00 @ (rupee) 88/$`, Value in (rupee) `8,800.00` (`k1-7-forex-box`). Enter on Forex Amount; `88` typed over the rate, Enter; Value 8,800.00 as offered, Enter. The line `BRIDGE Export Sales $100.00`; total `$100.00 @ (rupee) 88/$ = (rupee) 8,800.00` (`k1-8-line-100-at-88`).
- Enter on the empty second line, then Enter on the total: **Bill-wise Details for : BRIDGE USD Customer**, "Up to: (rupee) 8,800.00 Dr"; columns Type of Ref, Name, Due Date or Credit Days (wef: 30-Apr-25), Amount, Dr/Cr; Method of Adj. list Advance, Agst Ref, New Ref (highlighted), On Account (`k1-9-billwise`). New Ref, Enter; Name offered `4`, `LD1` typed over it, Enter; Due Date empty, Enter; Amount $100.00 as offered, Enter; Dr, Enter. One line: New Ref, LD1, $100.00 Dr.
- Narration left empty. Before Ctrl+A (`k1-10-before-accept`): no gain or loss line. Ctrl+A: saved, no prompt; a blank Sales No. 5 (`k1-11-after-save`). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; date of last entry 30-Apr-25 (`k1-12-gateway-after`).

**K2, Receipt.**
- The Gateway, a click on Tally's title bar, `v`: a blank Sales No. 5 opened (`k2-0-sales-no5-opened`). F6: Receipt, **No. 4** offered; the To line empty, the ledger list open (`k2-1-receipt-no4`).
- To: `BRIDGE`, space, `INR`, space, `Customer` typed as keystrokes, the one row highlighted, Enter. The voucher already showed **30-Apr-25 Wednesday** (no F2 used; `k2-2-date-before-amount`). Cur Bal 3,000.00 Dr.
- Credit: `2000`, Enter. No Forex box.
- **Bill-wise Details for : BRIDGE INR Customer**, "Up to: (rupee) 2,000.00 Cr"; Agst Ref highlighted in Method of Adj., Enter (`k2-3-billwise`). **Pending Bills:** one row, `R1`, Bill Date 1-Apr-25, Due Date 1-Apr-25, Balance 3,000.00 Dr, Final Balance 3,000.00 Dr (`k2-4-pending-r1`); Enter. Amount offered 2,000.00 Cr; `2000` typed over it, Enter; Cr, Enter (`k2-5-r1-amount`). The box closed: Agst Ref R1 2,000.00 Cr; the party's Cur Bal then 1,000.00 Dr.
- Tally then offered **By Cash**, Debit 2,000.00, Cur Bal 5,000.00 Dr (`k2-6-by-cash-offered`): Enter on By; the ledger list with Cash highlighted (the field read Cash), Enter; Debit 2,000.00 as offered, Enter.
- Narration left empty. Before Ctrl+A (`k2-7-before-accept`): To BRIDGE INR Customer Credit 2,000.00, By Cash Debit 2,000.00, totals 2,000.00 each; no gain or loss line. Ctrl+A: saved, no prompt; a blank Receipt No. 5 (`k2-8-after-save`, 09:26:55Z). Ctrl+Q: no prompt; the Gateway: current date Wednesday, 30-Apr-2025; last entry 30-Apr-25 (`k2-9-gateway-after`).

## Steps 5 to 7: the reads
Each request's sha256 was checked against `SHA256.txt` just before its send (`evidence/sends-utc.txt`). From m1 to m1r nothing else was sent and no key was pressed in Tally; only `lm.py`, `tb.py`, `rows.py` and `cmpinfo.py` ran between the sends, and the window capture after tb3f. The whole outputs are below.
- **tb3f:** the Tally window captured at once after the answer (`tb3f-window`, 09:27:35Z): the Gateway, current date Wednesday, 30-Apr-2025, last entry 30-Apr-25, no dialog.
- Calls beyond the sheet's are kept beside the sheet's: `tb.py` on row 683-LD's tb3 (`tb-ldtb3.txt`, deviation 2), from which the field list was made, as rows 683-VD's and 683-RO's were; the gaps between sends (`gaps.txt`), from `sends-utc.txt`; and `tally-exe.txt`, read after the close.

## Step 8: the restore (C6)
- Before the shut: the BK re-hashed equal to the one recorded, its folder holding that file alone; `restore-plan.txt` written; the data folder listed (24 entries). The plan and listings stay on the lab PC.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company (`c6-1-after-shut`).
- The Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`; Specify Path (Up 3 times), `C:\lab\bk-683xf2` typed in two keystroke chunks. The list showed one row: BRIDGE FOREXGROUP LAB (`<company number>`), 10-Nov-26 (2:48 pm) (`8-restore-list`). Enter on it; End of List, Enter.
- "Accept ?" (`8-restore-accept`): y. Then, word for word: "Company data folder (`<company number>`) already exists." / "Selecting Skip will cancel the Restore process for this company." / "Selecting Replace will overwrite the existing folder.", buttons "R: Replace" and "S: Skip" (`8-restore-replace`): r. Select Company followed, no other message (`8-after-replace`).
- The data folder after the restore: after=17, before=24 entries. Gone: `CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`, `TUPDATE.TSF` (the same seven as rows 683-LD's, 683-VD's and 683-RO's restores). None new.
- Loaded alone: `BRIDGE`, space, `FOREXGROU` typed, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`8-select`), Enter, no prompt. The Gateway (`8-gateway`): current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25; no dialog.
- m2 and mv2 as in the Result.

## Steps 9 and 10: the date, and close (C10)
- Step 9 (`9-gateway`): as in the Result; nothing was pressed to change the date or the period.
- Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 09:33:12Z).
- C2 at 09:33:12Z, after the last request: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B). No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; at 09:33:32Z no `tally.exe` ran and nothing listened on 9001. The three Bridge processes were not ended.

## Files
- `answers/`: this sitting's answers that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256). The sheet's copies of other rows' answers are not repeated here.
- `evidence/`: notes, the outputs of `lm.py`, `tb.py`, `rows.py` and `cmpinfo.py`, the field list, the gaps between sends, `tally.exe`'s file data, the start-up check's hits. Process and port listings, the backup folder and data folder listings, and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## lm.py, tb.py, rows.py and cmpinfo.py outputs, whole

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

### tb3a: rows.py against row 683-TW's tb3
```
a-twtb3-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
a-tb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb3a: cmpinfo.py against row 683-TW's tb3
```
a-twtb3-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
BYTE-EQUAL
exit 0
```

### tb3a: cmpinfo.py against the first sitting's tb3a
```
a-xftb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 6d291eddbf88278b7b5def480ef568886dac3562c59f5084b7dc8e88ba421981
a-tb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
exit 2
```

### tb3a2: tb.py --sets against row 683-TW's tb3
```
a-twtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3a2-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-twtb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb3a2-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb3a2: rows.py against row 683-TW's tb3
```
a-twtb3-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
a-tb3a2-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb3a2: cmpinfo.py against row 683-TW's tb3
```
a-twtb3-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb3a2-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
BYTE-EQUAL
exit 0
```

### tb3a2: cmpinfo.py against the first sitting's tb3a
```
a-xftb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 6d291eddbf88278b7b5def480ef568886dac3562c59f5084b7dc8e88ba421981
a-tb3a2-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
exit 2
```

### tb3a2: rows.py against tb3a
```
a-tb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
a-tb3a2-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb3a2: cmpinfo.py against tb3a
```
a-tb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb3a2-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
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

### mv0: cmp and sha256 against row 683-TW's mvy
```
a-mv0 vs a-twmvy: BYTE-EQUAL; 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7 59524 B
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

### tb3f: rows.py against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
  rows: 8
a-tb3f-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
  rows: 8
ROW 'BRIDGE Import Purchases': content differs
  B only: /CREDITTOTALS {'TYPE': 'Amount'} ''
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '0.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-$200.00 @ ₹ 86/$ = -₹ 17400.00'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-17400.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-$200.00 @ ₹ 86/$ = -₹ 17400.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-17400.00'
ROW 'BRIDGE Lab Bank': content differs
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '$200.00 @ ₹ 87/$ = ₹ 17500.00'
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '17500.00'
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '-$1100.00 @ ₹ 87/$ = -₹ 96700.00'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} '-96700.00'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '-$900.00 @ ₹ 88/$ = -₹ 79200.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '-79200.00'
CONTENT DIFFER
ROW ORDER SAME
ELEMENT ORDER SAME
exit 2
```

### tb3f: rows.py against row 683-VD's tb3w
```
a-vdtb3w-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
  rows: 8
a-tb3f-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb3f: rows.py against row 683-TW's tb3
```
a-twtb3-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
  rows: 8
a-tb3f-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
  rows: 8
ROW 'BRIDGE Export Sales': content differs
  A only: /CREDITTOTALS {'TYPE': 'Amount'} '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25'
  B only: /CREDITTOTALS {'TYPE': 'Amount'} '86014641.25'
  A only: /DEBITTOTALS {'TYPE': 'Amount'} ''
  B only: /DEBITTOTALS {'TYPE': 'Amount'} '0.00'
  A only: /TBALCLOSING {'TYPE': 'Amount'} '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25'
  B only: /TBALCLOSING {'TYPE': 'Amount'} '86014641.25'
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

### tb3f: cmpinfo.py against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: 13528 B sha256 a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-tb3f-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
exit 2
```

### tb3f: cmpinfo.py against row 683-VD's tb3w
```
a-vdtb3w-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-tb3f-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
BYTE-EQUAL
exit 0
```

### tb3f: tb.py
```
a-tb3f-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb3f: tb.py --sets against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3f-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-ldtb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3f-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
  'BRIDGE Import Purchases': PLAIN -> MIXED
  'BRIDGE Lab Bank': PLAIN -> MIXED
SETS DIFFER
exit 0
```

### tb3f: tb.py --sets against row 683-VD's tb3w
```
a-vdtb3w-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3f-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-vdtb3w-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3f-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb3f2: rows.py against tb3f
```
a-tb3f-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
  rows: 8
a-tb3f2-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
  rows: 8
CONTENT SAME
ROW ORDER SAME
ELEMENT ORDER SAME
exit 0
```

### tb3f2: cmpinfo.py against tb3f
```
a-tb3f-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-tb3f2-T-tb3-20250401-20250401.xml: 13740 B sha256 f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
BYTE-EQUAL
exit 0
```

### tb3f2: tb.py --sets against tb3f
```
a-tb3f-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3f2-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3f-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3f2-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (2): ['BRIDGE Import Purchases', 'BRIDGE Lab Bank']
SET PLAIN (6): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
SETS SAME
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

### tb3f: the fields that differ from row 683-LD's tb3
```
$ fields-vd.py evidence/tb-ldtb3.txt evidence/tb-tb3f.txt (the tb.py fields that differ, row by row: row 683-LD's tb3 -> tb3f)
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

### mv2: cmp against mv0 and row 683-TW's mvy
```
a-mv2 BYTE-EQUAL to a-mv0-V-mvy-20250401-20260331.xml
a-mv2 BYTE-EQUAL to a-twmvy-V-mvy-20250401-20260331.xml
```

### the tally.exe file
```
$ the program file of the Tally that answered (read after C10, with Tally closed)
tally.exe file version 1.1.7.2; 18485952 B; sha256 856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73; last written 2026-10-25T05:34:45Z
```

### each send's start and answer, UTC
```
2026-11-10T09:16:21.694Z m0 M-marks.xml (hash equal to the list)
2026-11-10T09:16:22.161Z m0 answered
2026-11-10T09:16:32.764Z tb3a T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T09:16:33.110Z tb3a answered
2026-11-10T09:16:42.107Z tb3a2 T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T09:16:42.431Z tb3a2 answered
2026-11-10T09:16:53.804Z mv0 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T09:16:54.215Z mv0 answered
2026-11-10T09:18:46.381Z mb M-marks.xml (hash equal to the list)
2026-11-10T09:18:46.801Z mb answered
2026-11-10T09:18:48.309Z mc M-marks.xml (hash equal to the list)
2026-11-10T09:18:48.780Z mc answered
2026-11-10T09:27:26.928Z m1 M-marks.xml (hash equal to the list)
2026-11-10T09:27:27.436Z m1 answered
2026-11-10T09:27:29.525Z x1 X-lx1-20250401.xml (hash equal to the list)
2026-11-10T09:27:30.000Z x1 answered
2026-11-10T09:27:32.057Z tb3f T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T09:27:32.550Z tb3f answered
2026-11-10T09:27:39.792Z tb3f2 T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T09:27:40.288Z tb3f2 answered
2026-11-10T09:27:54.735Z m1r M-marks.xml (hash equal to the list)
2026-11-10T09:27:55.113Z m1r answered
2026-11-10T09:32:25.298Z m2 M-marks.xml (hash equal to the list)
2026-11-10T09:32:25.876Z m2 answered
2026-11-10T09:32:33.654Z mv2 V-mvy-20250401-20260331.xml (hash equal to the list)
2026-11-10T09:32:34.076Z mv2 answered
```

### each send's time to answer, and the gap before it
```
$ each send's time to answer and the gap from the previous send's answer to its start, seconds, from evidence/sends-utc.txt
m0: started 09:16:21.694Z
m0: answered in 0.5 s
tb3a: started 09:16:32.764Z; gap after m0 answered: 10.6 s
tb3a: answered in 0.3 s
tb3a2: started 09:16:42.107Z; gap after tb3a answered: 9.0 s
tb3a2: answered in 0.3 s
mv0: started 09:16:53.804Z; gap after tb3a2 answered: 11.4 s
mv0: answered in 0.4 s
mb: started 09:18:46.381Z; gap after mv0 answered: 112.2 s
mb: answered in 0.4 s
mc: started 09:18:48.309Z; gap after mb answered: 1.5 s
mc: answered in 0.5 s
m1: started 09:27:26.928Z; gap after mc answered: 518.1 s
m1: answered in 0.5 s
x1: started 09:27:29.525Z; gap after m1 answered: 2.1 s
x1: answered in 0.5 s
tb3f: started 09:27:32.057Z; gap after x1 answered: 2.1 s
tb3f: answered in 0.5 s
tb3f2: started 09:27:39.792Z; gap after tb3f answered: 7.2 s
tb3f2: answered in 0.5 s
m1r: started 09:27:54.735Z; gap after tb3f2 answered: 14.4 s
m1r: answered in 0.4 s
m2: started 09:32:25.298Z; gap after m1r answered: 270.2 s
m2: answered in 0.6 s
mv2: started 09:32:33.654Z; gap after m2 answered: 7.8 s
mv2: answered in 0.4 s
K2's save (09:26:55Z, by the capture k2-8-after-save) to tb3f's start: 37 s
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every answer (LAB-COMMON C4: not rows); the names with a count other than 0
a-m0-M-marks.xml: all 0 (names in all: 35)
a-tb3a-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-tb3a2-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-mv0-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-mb-M-marks.xml: all 0 (names in all: 35)
a-mc-M-marks.xml: all 0 (names in all: 35)
a-m1-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-x1-X-lx1-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb3f-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-tb3f2-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-m1r-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-m2-M-marks.xml: all 0 (names in all: 35)
a-mv2-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-twtb3-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-xftb3a-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-twmvy-V-mvy-20250401-20260331.xml: all 0 (names in all: 35)
a-vdtb3w-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldtb3-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldx1-X-lx1-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldm1-M-marks.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldm2-M-marks.xml: all 0 (names in all: 35)
```

### sha256 of every answer
```
$ sha256sum and size of every answer of this sitting, in send order, and of the sheet's copies of earlier answers
a-m0-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-tb3a-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb3a2-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-mv0-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-mb-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-mc-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-m1-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-x1-X-lx1-20250401.xml 11556 B 8d9ac402c9effc56cad5809d6d8dd0ed27edb0af07cf06158cf0504e7300ead6
a-tb3f-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-tb3f2-T-tb3-20250401-20250401.xml 13740 B f30cdee170e8b271522642f2b5a2257cb2321fc74be7260680b8b0e3bd891bcb
a-m1r-M-marks.xml 3404 B cee319ccfbaa2c98667301763514418d6fc6b0ca9e69e6e0d75d0a716e8964d7
a-m2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-mv2-V-mvy-20250401-20260331.xml 59524 B 56554b6a655a92afc74f28527484647098f8cf43e774142d8caa5eb7736f4df7
a-twtb3-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-xftb3a-T-tb3-20250401-20250401.xml 14140 B 6d291eddbf88278b7b5def480ef568886dac3562c59f5084b7dc8e88ba421981
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
14:46:22	M-marks.xml	a-m0-M-marks.xml	curl 0	3404 bytes
```

### tb3a
```
14:46:32	T-tb3-20250401-20250401.xml	a-tb3a-T-tb3-20250401-20250401.xml	curl 0	14140 bytes
```

### tb3a2
```
14:46:42	T-tb3-20250401-20250401.xml	a-tb3a2-T-tb3-20250401-20250401.xml	curl 0	14140 bytes
```

### mv0
```
14:46:54	V-mvy-20250401-20260331.xml	a-mv0-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```

### mb
```
14:48:46	M-marks.xml	a-mb-M-marks.xml	curl 0	3404 bytes
```

### mc
```
14:48:48	M-marks.xml	a-mc-M-marks.xml	curl 0	3404 bytes
```

### m1
```
14:57:27	M-marks.xml	a-m1-M-marks.xml	curl 0	3404 bytes
```

### x1
```
14:57:29	X-lx1-20250401.xml	a-x1-X-lx1-20250401.xml	curl 0	11556 bytes
```

### tb3f
```
14:57:32	T-tb3-20250401-20250401.xml	a-tb3f-T-tb3-20250401-20250401.xml	curl 0	13740 bytes
```

### tb3f2
```
14:57:40	T-tb3-20250401-20250401.xml	a-tb3f2-T-tb3-20250401-20250401.xml	curl 0	13740 bytes
```

### m1r
```
14:57:54	M-marks.xml	a-m1r-M-marks.xml	curl 0	3404 bytes
```

### m2
```
15:02:25	M-marks.xml	a-m2-M-marks.xml	curl 0	3404 bytes
```

### mv2
```
15:02:33	V-mvy-20250401-20260331.xml	a-mv2-V-mvy-20250401-20260331.xml	curl 0	59524 bytes
```
