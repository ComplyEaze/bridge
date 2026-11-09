# Row 683-TW: from the book's first day, the currency Trial Balance's sets and the movement windows read the same to 1 April 2025 and to 31 March 2026 as to 30 April 2025 and to 1 April

Run on 9 Nov 2026, 23:36:13Z to 23:40:07Z (05:06 to 05:10 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; licensed, no Educational mode; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/683-tw-sheet` (8a7da70a), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own. **Read only:** raw reads, one at a time through `run.py`; nothing was written, so there was no backup and no restore; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book whose vouchers all sit on 1 April 2025, one release). Tally's current period was 1-Apr-25 to 31-Mar-26 and its current date Tuesday 1-Apr-2025 throughout; neither was changed. Every read was answered, each with STATUS 1, within a second; no dialog came up at any point.

This sitting ran after row 2467-GS (Tally quit at 23:25:16Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The marks it leaves are those it found (11/214).

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214. EXPECT met.
- **tb1, the control (20250401 to 20250430):** SET FOREIGN (0); SET MIXED (6) `BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`; SET PLAIN (2) `BRIDGE INR Customer`, `Cash`; SET UNREAD (0). As on 2 Nov; EXPECT met. The answer (14,140 B) is byte-equal to row 683-OP's `t0`.
- **Q1, the Trial Balance's `to`:**
  - **tb3 (to 20250401):** SETS SAME against tb1. No dialog after it.
  - **tb4 (to 20260331):** SETS SAME against tb1.
  - **The bytes:** tb1, tb3 and tb4 are byte-equal, all three (14,140 B each, sha256 `1c4638e8...`). So no row's text differs, Profit & Loss A/c included.
- **Q2, a movement window longer than the voucher day:**
  - **mvy (book scope, 20250401 to 20260331):** 11 vouchers, each dated 20250401; `lm.py --capture` against row 683-LM's one-day answer: each voucher SAME, CAPTURE SAME. The answer (59,524 B) is byte-equal to row 683-LM's `a-mv` answer (`ref-a-mv-V-mv-20250401.xml`, sha256 `56554b6a...`). No dialog after it.
  - **mvly-inr (`BRIDGE INR Customer`, kept):** 2 vouchers in the ledger-scoped window, and 2 vouchers with an entry on it in the book-scoped one; their amounts the same, plain (`-5000.00`, `2000.00`): LEDGER SAME. 12,998 B.
  - **mvly-usd (`BRIDGE USD Customer`, set aside):** 4 vouchers ledger-scoped, 4 book-scoped, each entry a composite and the same text in both: LEDGER SAME. The answer (23,720 B) is byte-equal to row 683-LM's `a-mvl` answer (sha256 `bff19938...`).
- **m1:** SAME MARKS (11/214), byte-equal to m0 (sha256 `3d75dd1a...`).

**By the sheet's Decides:**
- **Q1, SETS SAME for both tb3 and tb4:** measured. Rule 1's set for a first-day `from` does not move with `to` on this book and release: PARTIAL. The three Trial Balance answers are byte for byte the same, so the build's replay tests may use tb3 and tb4 beside tb1, and "a `to` other than 30 April" leaves Stays unmeasured, for this book.
- **Q1, SETS DIFFER, or any UNREAD:** not seen.
- **Q2, CAPTURE SAME and both LEDGER SAME:** measured. A window to the book's last day carries the same entries, with the same composites, as the voucher day alone, book-scoped and ledger-scoped: PARTIAL, on a book whose vouchers sit on one day. The book-scoped answer and the set-aside ledger's answer are byte for byte row 683-LM's one-day answers. It says nothing about entries on later days.
- **Q2, CAPTURE DIFFERS or LEDGER DIFFERS:** not seen.
- **tb1 not as EXPECT:** not seen.

**The sheet's estimates:** tb1 as on 2 Nov; tb3 and tb4 SETS SAME; mvy 11 vouchers and CAPTURE SAME; mvly-inr 2 vouchers and LEDGER SAME; mvly-usd 4 vouchers and LEDGER SAME. Each was so. The Trial Balance to 31 March 2026 carried no other text in any row.

## Step 0: start, check and load
- Sheet files: all 14 equal to `SHA256.txt` at 8a7da70a. `run.py`, `lm.py`, `LAB-COMMON.md`, `M-marks.xml` and `T-tb1-20250401-20250430.xml` are byte-equal to row 683-OP's; `tb.py` to row 683-TB's; `lm.py` to row 683-LM's; `ref-a-mv-V-mv-20250401.xml` to row 683-LM's `a-mv` answer.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 23:36:13Z, from its desktop shortcut, after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- C2 at 23:36:57Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE FOREXGROUP` typed into Select Company's name filter, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25. The PC's date: 10 Nov 2026 (IST).

## Steps 1 to 9: the reads
Each request's sha256 was checked against `SHA256.txt` just before its send (`evidence/sends-utc.txt`). Each send's times, answer size and sha256 are in `evidence/notes.txt`; each took between 0.3 and 0.6 seconds. The captures after tb3 (`after-tb3`) and after mvy (`after-mvy`) show the Gateway, and the desktop showed no message box. The whole outputs are below.

## Step 10: close (C10)
- Shut by the K: Company label, h, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 23:39:23Z).
- C2 at 23:39:24Z, after the last request: one `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; no `tally.exe` ran after.

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256).
- `evidence/`: notes, the outputs of `lm.py` and `tb.py`, the start-up check's hits. Process and port listings stay on the lab PC.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## Outputs, whole

### m0: lm.py
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### tb1: tb.py
```
a-tb1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
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

### tb3: tb.py
```
a-tb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb4: tb.py
```
a-tb4-T-tb4-20250401-20260331.xml: HEADER/STATUS '1'
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

### tb.py --sets, tb1 and tb3
```
a-tb1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb1-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb.py --sets, tb1 and tb4
```
a-tb1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb4-T-tb4-20250401-20260331.xml: HEADER/STATUS '1'
a-tb1-T-tb1-20250401-20250430.xml:
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

### mvy: lm.py --capture, against row 683-LM's one-day answer
```
a-mvy-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
ref-a-mv-V-mv-20250401.xml: HEADER/STATUS 1
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

### mvy: lm.py
```
a-mvy-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  VOUCHER rows: 11
  1bb48be5-baba-45b2-90d8-7593244915a9-00000001 20250401 Sales '1': 2 entries
    'BRIDGE USD Customer' Yes '-$1000000.00 @ ₹ 86/$ = -₹ 86000000.00'
    'BRIDGE Export Sales' No '$1000000.00 @ ₹ 86/$ = ₹ 86000000.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000002 20250401 Sales '2': 2 entries
    'BRIDGE USD Customer' Yes '-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25'
    'BRIDGE Export Sales' No '$100.00 @ ₹ 86.4125/$ = ₹ 8641.25'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000003 20250401 Receipt '1': 2 entries
    'BRIDGE USD Customer' No '$100.00 @ ₹ 87/$ = ₹ 8700.00'
    'BRIDGE Lab Bank' Yes '-$100.00 @ ₹ 87/$ = -₹ 8700.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000004 20250401 Journal '1': 2 entries
    'Cash' Yes '-1000.00'
    'BRIDGE Export Sales' No '1000.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000005 20250401 Sales '3': 2 entries
    'BRIDGE INR Customer' Yes '-5000.00'
    'BRIDGE Export Sales' No '5000.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000006 20250401 Receipt '2': 2 entries
    'BRIDGE INR Customer' No '2000.00'
    'Cash' Yes '-2000.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000007 20250401 Purchase '1': 2 entries
    'BRIDGE USD Supplier' No '$100.00 @ ₹ 86/$ = ₹ 8600.00'
    'BRIDGE Import Purchases' Yes '-$100.00 @ ₹ 86/$ = -₹ 8600.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000008 20250401 Purchase '2': 2 entries
    'BRIDGE USD Supplier' No '$100.00 @ ₹ 88/$ = ₹ 8800.00'
    'BRIDGE Import Purchases' Yes '-$100.00 @ ₹ 88/$ = -₹ 8800.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000009 20250401 Payment '1': 2 entries
    'BRIDGE USD Supplier' Yes '-$100.00 @ ₹ 87/$ = -₹ 8700.00'
    'BRIDGE Lab Bank' No '$100.00 @ ₹ 87/$ = ₹ 8700.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000a 20250401 Payment '2': 2 entries
    'BRIDGE USD Supplier' Yes '-$100.00 @ ₹ 88/$ = -₹ 8800.00'
    'BRIDGE Lab Bank' No '$100.00 @ ₹ 88/$ = ₹ 8800.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000b 20250401 Receipt '3': 2 entries
    'BRIDGE USD Customer' No '$1000.00 @ ₹ 88/$ = ₹ 88000.00'
    'BRIDGE Lab Bank' Yes '-$1000.00 @ ₹ 88/$ = -₹ 88000.00'
exit 0
```

### mvly-inr: lm.py --ledger, BRIDGE INR Customer
```
a-mvly-inr-V-mvly-inr-customer-20250401-20260331.xml: HEADER/STATUS 1
a-mvy-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  ledger 'BRIDGE INR Customer': ledger-scoped 2 vouchers, book-scoped with an entry on it 2
  1bb48be5-baba-45b2-90d8-7593244915a9-00000005: ledger-scoped ['-5000.00'] book-scoped ['-5000.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-00000006: ledger-scoped ['2000.00'] book-scoped ['2000.00']
LEDGER SAME
exit 0
```

### mvly-usd: lm.py --ledger, BRIDGE USD Customer
```
a-mvly-usd-V-mvly-usd-customer-20250401-20260331.xml: HEADER/STATUS 1
a-mvy-V-mvy-20250401-20260331.xml: HEADER/STATUS 1
  ledger 'BRIDGE USD Customer': ledger-scoped 4 vouchers, book-scoped with an entry on it 4
  1bb48be5-baba-45b2-90d8-7593244915a9-00000001: ledger-scoped ['-$1000000.00 @ ₹ 86/$ = -₹ 86000000.00'] book-scoped ['-$1000000.00 @ ₹ 86/$ = -₹ 86000000.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-00000002: ledger-scoped ['-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25'] book-scoped ['-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25']
  1bb48be5-baba-45b2-90d8-7593244915a9-00000003: ledger-scoped ['$100.00 @ ₹ 87/$ = ₹ 8700.00'] book-scoped ['$100.00 @ ₹ 87/$ = ₹ 8700.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000b: ledger-scoped ['$1000.00 @ ₹ 88/$ = ₹ 88000.00'] book-scoped ['$1000.00 @ ₹ 88/$ = ₹ 88000.00']
LEDGER SAME
exit 0
```

### m1: lm.py
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### m1: lm.py --same-row, against m0
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
exit 0
```
