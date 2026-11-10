# Row 683-CD: on the forex book, Tally's current date alone did not change the Trial Balance to 1 April; at 30-Apr-2025 it answered byte for byte as at 1-Apr-2025, six ledgers MIXED

Run on 10 Nov 2026, 03:33:27Z to 03:38:23Z (09:03 to 09:08 IST), on TallyPrime Silver (the Gateway on 9001; licensed, no Educational mode shown; `tally.exe` file version 1.1.7.2, as row 683-LD read it from the program's file; the Gateway shows no release number), by the sheet at `lab/683-cd` (9dd3b47d), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic), loaded alone in a sitting of its own. Reads only: no voucher or master was keyed, so there was no BK and no restore. The one thing changed was Tally's current date, by F2 at the Gateway, and it was set back before the close. Raw reads, one at a time through `run.py`; no Bridge build. Every value below is M, measured by this run, in one run. Every read was answered, each with STATUS 1. The PC's date was 10 Nov 2026.

This sitting ran after row 2346-AR (Tally quit at 03:25:45Z), with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end.

## Result

**Q1: as row 683-TW's.** With the book as found and the current date at 30-Apr-2025, `tb3` answered byte for byte as at 1-Apr-2025: 14,140 B, sha256 `1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7`, six ledgers MIXED, two PLAIN. It is not byte-equal to row 683-LD's `tb3` (13,528 B, all PLAIN).

| send | current date | bytes | sha256 | byte-equal to | `tb.py --sets` |
|---|---|---|---|---|---|
| `tb3a` (control) | Tuesday, 1-Apr-2025 | 14,140 | `1c4638e8...b7b7` | row 683-TW's `tb3` | SETS SAME against row 683-TW's |
| `tb3d` | Wednesday, 30-Apr-2025 | 14,140 | `1c4638e8...b7b7` | `tb3a` (not row 683-LD's) | SETS SAME against `tb3a`; SETS DIFFER against row 683-LD's (the six PLAIN -> MIXED) |
| `tb3r` | Tuesday, 1-Apr-2025 | 14,140 | `1c4638e8...b7b7` | `tb3a` | SETS SAME against `tb3a` |

The sets of all three: MIXED (6) `BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`; PLAIN (2) `BRIDGE INR Customer`, `Cash`; FOREIGN and UNREAD none.

**By the sheet's Decides:** "`tb3a` and `tb3r` as row 683-TW's, `tb3d` SETS SAME against `tb3a`: the current date alone does not change this answer." Row 683-LD's change then came with the vouchers dated 30 April (and the date of last entry they move), alone or together with the current date. Separating those two needs the vouchers and a current date of 1 April in one sitting, a write behind a backup: another row, drafted only on the run lead's word. PARTIAL: one request, one sitting, one book, one release.

- **Step 0:** current date Tuesday, 1-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 1-Apr-25. EXPECT met.
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214: SAME MARKS with row 683-LD's m2, and byte-equal to it (sha256 `3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba`, 3,404 B). EXPECT met.
- **m1** (after the date was moved to 30-Apr-2025) and **m2** (after it was set back): SAME MARKS with m0, and each byte-equal to m0. Changing the current date wrote nothing the marks show. EXPECT met.
- **The F2 box, each time:** the title bar read `Change Current Date`, with one field, `Current Date`. It offered `2-4-2025` the first time (the current date then 1-Apr-2025) and `1-5-2025` the second (the current date then 30-Apr-2025). Each typed date was taken with no message. The Gateway after each showed the new date with its weekday, and the period and last entry as before.
- **Step 8:** current date Tuesday, 1-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 1-Apr-25: as at step 0.
- **The sheet's estimates:** `tb3a` byte-equal to row 683-TW's, `tb3r` byte-equal to `tb3a`, m0, m1 and m2 byte-equal: each as estimated. `tb3d` was not estimated.
- **Timing** (`evidence/sends-utc.txt`, each send's start and its answer, UTC to the millisecond): `tb3a` 0.347 s, `tb3d` 0.456 s, `tb3r` 0.527 s; the marks reads 0.467 s to 0.587 s.

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **The start-up company.** The sheet's Screens line says a company loaded at the start is C1's stop unless the run lead's word is to shut it. Tally's start-up company loaded by itself, as the install is set to. It was shut by the K: Company label, Shut, Enter on it, as row 683-LD shut it, on the run lead's standing practice for these sittings, with no change to it.

**Beyond the sheet, recorded and not judged:**
- The Gateway's company line carried "(Schedule Backup)" and no "(e)" mark, at every reading.
- Every DESC/CMPINFO count in this row's answers is 0 (`evidence/cmpinfo.txt`). Row 683-LD's `tb3`, read after its two vouchers were saved, carries LEDGER 6, CURRENCY 2, TAXUNIT 2, VOUCHERNUMBERSERIES 2 and VOUCHER 3 there; row 683-TW's carries 0 (LAB-COMMON C4: these are not rows).

## Step 0: start, check and load
- Sheet files: all 11 equal to `SHA256.txt` at 9dd3b47d. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `lm.py`, `tb.py`, `M-marks.xml` and `T-tb3-20250401-20250401.xml` are byte-equal to row 683-LD's. `a-ldm2-M-marks.xml` and `a-ldtb3-T-tb3-20250401-20250401.xml` are byte-equal to row 683-LD's `a-m2` and `a-tb3`; `a-twtb3-T-tb3-20250401-20250401.xml` is byte-equal to row 683-TW's `a-tb3`.
- Before the start, the book's data folder was found as the one whose `Company.1800` holds BRIDGE FOREXGROUP LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 03:33:27Z, from its desktop shortcut, after Get-Process showed none. Its start-up company loaded by itself and was shut first (deviation 2).
- C2 at 03:34:26Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE` and ` FOREXGROU` typed into Select Company's name filter, one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-25 to 31-Mar-26; current date Tuesday, 1-Apr-2025; date of last entry 1-Apr-25.

## Steps 1 and 2: m0 and the control
- `m0` (03:35:09Z): as in the Result.
- `tb3a` (03:35:20Z): `T-tb3-20250401-20250401.xml`, its sha256 checked against `SHA256.txt` just before the send; as in the Result.

## Step 3: the date to 30 April
- At the Gateway, with no report open: F2. The box as in the Result, offering `2-4-2025` (`d1-box`, 03:35:39Z). `30-4-2025` typed (checked by zoom), Enter. The Gateway (`d1-gateway`): current date Wednesday, 30-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 1-Apr-25. No message.
- `m1`: SAME MARKS with m0.

## Step 4: `tb3d`
- `tb3d` (03:36:21Z): as in the Result. The window at once after it (`tb3d-after`, 03:36:24Z): the Gateway, no dialog.

## Step 5: the date back to 1 April
- F2. The same box, offering `1-5-2025` (`d2-box`, 03:37:00Z). `1-4-2025` typed (checked by zoom), Enter. The Gateway (`d2-gateway`): current date Tuesday, 1-Apr-2025; period and last entry as at step 0. No message. EXPECT met.

## Steps 6 and 7: `tb3r` and m2
- `tb3r` (03:37:28Z) and `m2` (03:37:30Z): as in the Result.

## Step 8: close (C10)
- The Gateway read once more: as at step 0. Shut by the K: Company label, Shut, BRIDGE FOREXGROUP LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 03:38:04Z).
- C2 at 03:38:04Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe`. The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer of this row, byte for byte (`kept-local.txt` lists anything that stays on the lab PC, by size and sha256, with the reason). The sheet's copies of rows 683-TW's and 683-LD's answers are on the sheet's branch already.
- `evidence/`: notes, every `lm.py` and `tb.py` output, the answers' sizes and sha256, and the start-up check's hits. Process and port listings stay on the lab PC.
- `logs/`: `run.py`'s log of each send.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## lm.py and tb.py outputs, whole

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

### m1: lm.py
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### m1: lm.py --same-row against m0
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

### tb3d: tb.py
```
a-tb3d-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb3d: tb.py --sets against tb3a
```
a-tb3a-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3d-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3a-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb3d-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
exit 0
```

### tb3d: tb.py --sets against row 683-LD's tb3
```
a-ldtb3-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3d-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-ldtb3-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (0): []
SET PLAIN (8): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash', 'Profit & Loss A/c']
SET UNREAD (0): []
a-tb3d-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
  'BRIDGE Export Sales': PLAIN -> MIXED
  'BRIDGE Import Purchases': PLAIN -> MIXED
  'BRIDGE Lab Bank': PLAIN -> MIXED
  'BRIDGE USD Customer': PLAIN -> MIXED
  'BRIDGE USD Supplier': PLAIN -> MIXED
  'Profit & Loss A/c': PLAIN -> MIXED
SETS DIFFER
exit 0
```

### tb3r: tb.py
```
a-tb3r-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
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

### tb3r: tb.py --sets against tb3a
```
a-tb3a-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3r-T-tb3-20250401-20250401.xml: HEADER/STATUS '1'
a-tb3a-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb3r-T-tb3-20250401-20250401.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
SETS SAME
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

### each send's start and answer, UTC
```
2026-11-10T03:35:09.485Z m0 M-marks.xml (hash equal to the list)
2026-11-10T03:35:09.952Z m0 answered
2026-11-10T03:35:20.101Z tb3a T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T03:35:20.448Z tb3a answered
2026-11-10T03:36:13.527Z m1 M-marks.xml (hash equal to the list)
2026-11-10T03:36:14.114Z m1 answered
2026-11-10T03:36:21.352Z tb3d T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T03:36:21.808Z tb3d answered
2026-11-10T03:37:28.355Z tb3r T-tb3-20250401-20250401.xml (hash equal to the list)
2026-11-10T03:37:28.882Z tb3r answered
2026-11-10T03:37:30.931Z m2 M-marks.xml (hash equal to the list)
2026-11-10T03:37:31.438Z m2 answered
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every answer (LAB-COMMON C4: not rows); the names with a count other than 0
a-m0-M-marks.xml: all 0 (names in all: 35)
a-tb3a-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-m1-M-marks.xml: all 0 (names in all: 35)
a-tb3d-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-tb3r-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-m2-M-marks.xml: all 0 (names in all: 35)
a-twtb3-T-tb3-20250401-20250401.xml: all 0 (names in all: 35)
a-ldtb3-T-tb3-20250401-20250401.xml: LEDGER 6; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 3; others 0 (names in all: 35)
a-ldm2-M-marks.xml: all 0 (names in all: 35)
```

### sha256 of every answer
```
$ sha256sum and size of every answer of this row, in send order, and of the sheet's copies of earlier rows' answers
a-m0-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-tb3a-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-m1-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-tb3d-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-tb3r-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-m2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
a-twtb3-T-tb3-20250401-20250401.xml 14140 B 1c4638e8df640575eff3e4955bed2d9783026fe7817ad73e63e05f19277fb7b7
a-ldtb3-T-tb3-20250401-20250401.xml 13528 B a12217b840ec71c5f68a6f646c10625f3f554bdd09c591d113c9667177b97705
a-ldm2-M-marks.xml 3404 B 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### m0
```
09:05:09	M-marks.xml	a-m0-M-marks.xml	curl 0	3404 bytes
```

### tb3a
```
09:05:20	T-tb3-20250401-20250401.xml	a-tb3a-T-tb3-20250401-20250401.xml	curl 0	14140 bytes
```

### m1
```
09:06:13	M-marks.xml	a-m1-M-marks.xml	curl 0	3404 bytes
```

### tb3d
```
09:06:21	T-tb3-20250401-20250401.xml	a-tb3d-T-tb3-20250401-20250401.xml	curl 0	14140 bytes
```

### tb3r
```
09:07:28	T-tb3-20250401-20250401.xml	a-tb3r-T-tb3-20250401-20250401.xml	curl 0	14140 bytes
```

### m2
```
09:07:31	M-marks.xml	a-m2-M-marks.xml	curl 0	3404 bytes
```
