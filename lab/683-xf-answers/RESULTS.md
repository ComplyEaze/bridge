# Row 683-XF: stopped at step 2, before the backup, with nothing written: the control tb3a read the same sets and values as row 683-TW's tb3, but was not byte-equal to it, as one element of each ledger came in another order

Run on 10 Nov 2026, 07:56:37Z to 08:00:46Z (13:26 to 13:30 IST). TallyPrime Silver: the Gateway on 9001; licensed, no Educational mode shown; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number. The sheet is `lab/683-xf-sheet` (2efa8f55). The book is `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own.

What ran: two raw reads, one at a time through `run.py` (m0 and tb3a); no Bridge build. Every read was answered, each with STATUS 1, within a second (0.4 and 0.5 s). No BK was taken, nothing was keyed and nothing was written to the book.

This sitting ran after rows 2346-TD and 2464-RS, which each had a sitting of their own on `BRIDGE PILOT LAB`, with Tally starts of their own, after row 683-RO's sitting on this book. Tally was started once from its desktop shortcut here and closed by its own Quit at the end.

## Result

**Stopped at step 2, by the sheet's Stop list:** "a control in step 2 is not as EXPECT". The run lead's word on the stop: close by C10 and write the row up as stopped at step 2, with no other send.

- **Step 0:** one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B). The book was loaded alone. The Gateway: current period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025, date of last entry 1-Apr-25. EXPECT met.
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214: SAME MARKS with row 683-LD's m2. EXPECT met.
- **tb3a (the control, current date 1-Apr-2025): not byte-equal to row 683-TW's tb3.**
  - Both are 14,140 B. tb3a's sha256 is `6d291eddbf88278b7b5def480ef568886dac3562c59f5084b7dc8e88ba421981`; row 683-TW's is `1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7`.
  - `tb.py --sets`: SETS SAME, exit 0. Six MIXED (`BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`) and two PLAIN (`BRIDGE INR Customer`, `Cash`), as row 683-TW's.
  - `cmpinfo.py`: NOT BYTE-EQUAL, outside CMPINFO DIFFERS, exit 2.
  - The whole line diff (`evidence/tb3a-diff.txt`): in each of the 8 ledgers, the line `<TBALOPENING TYPE="Amount">0.00</TBALOPENING>` stands before `TBALCLOSING`, where row 683-TW's answer has it after. 8 lines removed and 8 added, every one of them that line. Taken as sorted lists of lines, the two answers are EQUAL: every value and every other line is the same.
- **The sheet's EXPECT for tb3a is byte-equal and SETS SAME.** SETS SAME was met; byte-equal was not. So the control is not as EXPECT, and the row stopped before mv0 and the BK.
- **No other send.** mv0 was not sent, no BK was taken, and no key was pressed in Tally between the load and the shut. The window was captured at once (`stop-tb3a-window`): the Gateway, the book alone, no dialog.
- **C10:** the book shut with no prompt; one `tally.exe`, 9001 owned by it alone, `GET /status` 200 (51 B), the last request; Tally's Quit closed it with no prompt.

**Recorded, not judged:**
- Of the 33 Trial Balance answer files in this family's lab row folders (rows 683-OP, 683-TB, 683-TW, 683-LD, 683-CD, 683-VD, 683-RO and this row; the sheets' copies of earlier rows' answers counted where they are held), tb3a is the only one with `TBALOPENING` before `TBALCLOSING` (`evidence/tb-order-survey.txt`). It is also the first Trial Balance read on this book after the two sittings on the pilot book.
- `tally.exe` is the same file as before: 18,485,952 B, sha256 `856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73`, last written 2026-10-25 (`evidence/tally-exe.txt`).
- The request was the sheet's own file, its hash equal to `SHA256.txt` before the send (`evidence/sends-utc.txt`).

**By the sheet's Decides:** "`tb3a` not as row 683-TW's: the book or the sitting reads differently from the rows before; the row stopped before the BK (step 2)." The question of the row (Q1) was not reached: there was no write, no `x1` and no `tb3f`.

## Deviations
1. **The start-up company was shut first.** Its own book loaded by itself, as the install is set to, and was shut by C9 before the load (the K: Company label, Shut, Enter on its one row, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
2. **A note line rewritten.** The last line of `evidence/notes.txt` first printed the two process counts side by side. It was rewritten as named counts before staging; the values did not change.

## Step 0: start, check and load
- Sheet files: all 18 equal to `SHA256.txt` at 2efa8f55 (sizes too). `.gitattributes`, `LAB-COMMON.md`, `run.py`, `lm.py`, `tb.py`, `cmpinfo.py` and the four requests are byte-equal to row 683-RO's. `a-vdtb3w` is byte-equal to row 683-RO's tb3c; `a-twtb3` to its tb3a; `a-twmvy` to its mv0; `a-ldx1` to its x1; `a-ldm1` and `a-ldm2` to its m1 and m2; `a-ldtb3` to row 683-LD's tb3.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` names the book and holds its GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 07:56:37Z, from its desktop shortcut, after Get-Process showed none and no listener on 9001.
- C2 at 07:57:10Z: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B).
- The book was loaded alone: `BRIDGE`, space, `FOREXGROU` typed in the name filter, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25. The PC's date: 10 Nov 2026 (IST).

## Files
- `answers/`: the two answers of this row, byte for byte (m0 and tb3a), listed by size and sha256 in `evidence/answers-sha.txt`. The sheet's copies of earlier rows' answers are on the sheet's branch already and are not pushed again.
- `evidence/`: the console outputs, compares and notes. `notes.txt` records every key, prompt and screen, with UTC times.
- `logs/`: the `run.py` logs.
- `scripts/`: this row's helpers, made from row 683-RO's with this row's folder.
- `kept-local.txt`, `SCREENSHOTS.txt`, `substitutions.txt` and `SHA256.txt`.

## lm.py, tb.py and cmpinfo.py outputs, and the evidence the stop asked for, whole

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
a-tb3a vs a-twtb3: DIFFERS; 6d291eddbf88278b7b5def480ef568886dac3562c59f5084b7dc8e88ba421981 14140 B; 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7 14140 B
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

### tb3a: cmpinfo.py against row 683-TW's tb3 (a call beyond step 2's, made for the stop)
```
a-twtb3-T-tb3-20250401-20250401.xml: 14140 B sha256 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb3a-T-tb3-20250401-20250401.xml: 14140 B sha256 6d291eddbf88278b7b5def480ef568886dac3562c59f5084b7dc8e88ba421981
NOT BYTE-EQUAL; outside CMPINFO: DIFFERS
exit 2
```

### tb3a: the whole line diff against row 683-TW's tb3
```
$ python -I (difflib, inline) unified line diff of row 683-TW's tb3 answer (a-twtb3) against tb3a, both decoded UTF-16LE, 3 lines of context
--- a-twtb3-T-tb3-20250401-20250401.xml
+++ a-tb3a-T-tb3-20250401-20250401.xml
@@ -51,8 +51,8 @@
      <PARENT TYPE="String">Sales Accounts</PARENT>
      <DEBITTOTALS TYPE="Amount"></DEBITTOTALS>
      <CREDITTOTALS TYPE="Amount">$1000100.00 @ ₹ 86/$ = ₹ 86014641.25</CREDITTOTALS>
+     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <TBALCLOSING TYPE="Amount">$1000100.00 @ ₹ 86/$ = ₹ 86014641.25</TBALCLOSING>
-     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <LANGUAGENAME.LIST>
       <NAME.LIST TYPE="String">
        <NAME>BRIDGE Export Sales</NAME>
@@ -66,8 +66,8 @@
      <PARENT TYPE="String">Purchase Accounts</PARENT>
      <DEBITTOTALS TYPE="Amount">-$200.00 @ ₹ 86/$ = -₹ 17400.00</DEBITTOTALS>
      <CREDITTOTALS TYPE="Amount"></CREDITTOTALS>
+     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <TBALCLOSING TYPE="Amount">-$200.00 @ ₹ 86/$ = -₹ 17400.00</TBALCLOSING>
-     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <LANGUAGENAME.LIST>
       <NAME.LIST TYPE="String">
        <NAME>BRIDGE Import Purchases</NAME>
@@ -81,8 +81,8 @@
      <PARENT TYPE="String">Sundry Debtors</PARENT>
      <DEBITTOTALS TYPE="Amount">-5000.00</DEBITTOTALS>
      <CREDITTOTALS TYPE="Amount">2000.00</CREDITTOTALS>
+     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <TBALCLOSING TYPE="Amount">-3000.00</TBALCLOSING>
-     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <LANGUAGENAME.LIST>
       <NAME.LIST TYPE="String">
        <NAME>BRIDGE INR Customer</NAME>
@@ -96,8 +96,8 @@
      <PARENT TYPE="String">Bank Accounts</PARENT>
      <DEBITTOTALS TYPE="Amount">-$1100.00 @ ₹ 87/$ = -₹ 96700.00</DEBITTOTALS>
      <CREDITTOTALS TYPE="Amount">$200.00 @ ₹ 87/$ = ₹ 17500.00</CREDITTOTALS>
+     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <TBALCLOSING TYPE="Amount">-$900.00 @ ₹ 88/$ = -₹ 79200.00</TBALCLOSING>
-     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <LANGUAGENAME.LIST>
       <NAME.LIST TYPE="String">
        <NAME>BRIDGE Lab Bank</NAME>
@@ -111,8 +111,8 @@
      <PARENT TYPE="String">Sundry Debtors</PARENT>
      <DEBITTOTALS TYPE="Amount">-$1000100.00 @ ₹ 87.9999/$ = -₹ 88008700.00</DEBITTOTALS>
      <CREDITTOTALS TYPE="Amount">$1100.00 @ ₹ 87/$ = ₹ 96700.00</CREDITTOTALS>
+     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <TBALCLOSING TYPE="Amount">-$999000.00 @ ₹ 88/$ = -₹ 87912000.00</TBALCLOSING>
-     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <LANGUAGENAME.LIST>
       <NAME.LIST TYPE="String">
        <NAME>BRIDGE USD Customer</NAME>
@@ -126,8 +126,8 @@
      <PARENT TYPE="String">Sundry Creditors</PARENT>
      <DEBITTOTALS TYPE="Amount">-$200.00 @ ₹ 87/$ = -₹ 17500.00</DEBITTOTALS>
      <CREDITTOTALS TYPE="Amount">$200.00 @ ₹ 87.50/$ = ₹ 17500.00</CREDITTOTALS>
+     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <TBALCLOSING TYPE="Amount">$0.00 @ ₹ /$ = ₹ 0.00</TBALCLOSING>
-     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <LANGUAGENAME.LIST>
       <NAME.LIST TYPE="String">
        <NAME>BRIDGE USD Supplier</NAME>
@@ -141,8 +141,8 @@
      <PARENT TYPE="String">Cash-in-Hand</PARENT>
      <DEBITTOTALS TYPE="Amount">-3000.00</DEBITTOTALS>
      <CREDITTOTALS TYPE="Amount"></CREDITTOTALS>
+     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <TBALCLOSING TYPE="Amount">-3000.00</TBALCLOSING>
-     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <LANGUAGENAME.LIST>
       <NAME.LIST TYPE="String">
        <NAME>Cash</NAME>
@@ -156,8 +156,8 @@
      <PARENT TYPE="String">&#4; Primary</PARENT>
      <DEBITTOTALS TYPE="Amount"></DEBITTOTALS>
      <CREDITTOTALS TYPE="Amount"></CREDITTOTALS>
+     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <TBALCLOSING TYPE="Amount">$0.00 @ ₹ /$ = ₹ 0.00</TBALCLOSING>
-     <TBALOPENING TYPE="Amount">0.00</TBALOPENING>
      <LANGUAGENAME.LIST>
       <NAME.LIST TYPE="String">
        <NAME>Profit &amp; Loss A/c</NAME>
lines removed: 8; lines added: 8; every one is <TBALOPENING TYPE="Amount">0.00</TBALOPENING>, moved from after TBALCLOSING to before it
the two answers as sorted lists of lines: EQUAL
```

### every Trial Balance answer file in the lab row folders, by which element comes first
```
$ python -I (inline) every Trial Balance answer file (a-*-T-*.xml holding both TBALOPENING and TBALCLOSING) in the lab row folders, counted by row and by which of the two elements comes first; copies of earlier rows' answers count in the row that holds them
683-cd: TBALCLOSING first=5
683-ld: TBALCLOSING first=5
683-op: TBALCLOSING first=2
683-ro: TBALCLOSING first=6
683-tb: TBALCLOSING first=2
683-tw: TBALCLOSING first=3
683-vd: TBALCLOSING first=6
683-xf: TBALCLOSING first=3; TBALOPENING first=1
in all: TBALCLOSING first=32; TBALOPENING first=1
the one with TBALOPENING first is a-tb3a in row 683-XF
```

### the program file of the Tally that answered
```
$ the program file of the Tally that answered (read after C10, with Tally closed)
tally.exe file version 1.1.7.2; 18485952 B; sha256 856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73; last written 2026-10-25T05:34:45Z
```

### before the start: the book's data folder and the excluded GUID
```
BRIDGE FOREXGROUP LAB: folder <N>, GUID hits 1, excluded GUID hits 0
folders named so: 1
```

### each send's start and answer, UTC
```
2026-11-10T07:58:19.804Z m0 M-marks.xml (hash equal to the list)
2026-11-10T07:58:20.203Z m0 answered
2026-11-10T07:58:28.582Z tb3a T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T07:58:29.090Z tb3a answered
```

### each send's time to answer, and the gap before it
```
$ each send's time to answer and the gap from the previous send's answer to its start, seconds, from evidence/sends-utc.txt
m0: started 07:58:19.804Z
m0: answered in 0.4 s
tb3a: started 07:58:28.582Z; gap after m0 answered: 8.4 s
tb3a: answered in 0.5 s
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every answer (LAB-COMMON C4: not rows); the names with a count other than 0
a-m0-M-marks.xml: all 0 (names in all: 35)
a-tb3a-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
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
a-tb3a-T-tb3-20250401-20250401.xml 14140 B 6d291eddbf88278b7b5def480ef568886dac3562c59f5084b7dc8e88ba421981
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
13:28:20	M-marks.xml	a-m0-M-marks.xml	curl 0	3404 bytes
```

### tb3a
```
13:28:28	T-tb3-20250401-20250401.xml	a-tb3a-T-tb3-20250401-20250401.xml	curl 0	14140 bytes
```
