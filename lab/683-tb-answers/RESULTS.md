# Row 683-TB: on the several-currency lab book, a dollar balance carried into a window with no entry reads as a plain figure in the currency Trial Balance; QUESTION: NO COMPOSITE

Run on 9 Nov 2026, 21:36:00Z to 21:39:21Z (03:06 to 03:09 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2), by the sheet at `lab/683-tb-sheet` (6dd1cb62), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic, in master's registry), loaded alone in a sitting of its own. **Read only:** four raw reads, one at a time through `run.py`, with bytes rendered by master's builder; no write, no backup, no restore, and no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one book, one window, one foreign currency, one release). Tally's current period was 1-Apr-25 to 31-Mar-26 and its current date Tuesday 1-Apr-2025 throughout; neither was changed. Every read was answered, each with STATUS 1, each in under half a second; the 5-minute wedge rule was never near.

This sitting ran after row 2162-GY (Tally quit at 21:07:28Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end.

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214, as row 683-LM left it. EXPECT met.
- **tb1, the control (20250401 to 20250430):** 8 LEDGER rows, every CURRENCYNAME the rupee sign. **As EXPECT, as on 2 Nov:**
  - SET FOREIGN (0);
  - SET MIXED (6): `BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`;
  - SET PLAIN (2): `BRIDGE INR Customer`, `Cash`;
  - SET UNREAD (0).
- **tb2, the question (20250402 to 20250430):** 8 LEDGER rows, every CURRENCYNAME the rupee sign. The whole window right after it showed the Gateway, no dialog.
  - **`BRIDGE USD Customer`: PLAIN.** TBALOPENING `-87912000.00`, DEBITTOTALS `0.00`, CREDITTOTALS `0.00`, TBALCLOSING `-87912000.00`. In tb1 its closing was the composite `-$999000.00 @ <rupee> 88/$ = -<rupee> 87912000.00`; in tb2 the same base part comes as a plain figure, with no dollar part.
  - **`BRIDGE USD Supplier`: PLAIN,** every value `0.00`.
  - **`BRIDGE Lab Bank`: PLAIN,** opening and closing `-79200.00`, debit and credit `0.00`.
  - `BRIDGE Export Sales` and `BRIDGE Import Purchases`: PLAIN, every value `0.00`. `BRIDGE INR Customer` and `Cash`: PLAIN, opening and closing `-3000.00`.
  - **`Profit & Loss A/c`: MIXED,** the one composite in the answer: TBALOPENING and TBALCLOSING `$999900.00 @ <rupee> 86.0058/$ = <rupee> 85997241.25`, DEBITTOTALS and CREDITTOTALS empty.
  - SET FOREIGN (0); SET MIXED (1); SET PLAIN (7); SET UNREAD (0).
  - **`tb.py --question`'s last line: `QUESTION: NO COMPOSITE`.**
- **The sets, tb1 against tb2:** SETS DIFFER. Five ledgers go MIXED -> PLAIN: `BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`. `Profit & Loss A/c` stays MIXED; `BRIDGE INR Customer` and `Cash` stay PLAIN.
- **m1:** SAME MARKS (11/214), and m1 is byte-equal to m0 (both sha256 `3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba`). Nothing moved.

**What the figures show beside row 683-LM's** (read off the answers; why Tally does it was not measured):
- tb2's eight openings are the same figures as row 683-LM's `lx2` openings at 20250402: `BRIDGE USD Customer` `-87912000.00`, `BRIDGE Lab Bank` `-79200.00`, `BRIDGE INR Customer` and `Cash` `-3000.00`, the income and expense ledgers and `BRIDGE USD Supplier` `0.00`, and `Profit & Loss A/c` the same composite.
- They sum to -1,999,958.75, as `lx2`'s did. No ledger in the answer carries the other side.

**By the sheet's Decides:**
- **QUESTION: NO COMPOSITE:** measured. Its currency is the base, all four values are plain, and no row in the answer is UNREAD. Rule 1 misses `BRIDGE USD Customer` in a window with no entry, on this book, release and window. By the sheet, the build keeps refusing a several-currency book for a `from` after the book's start.
- **QUESTION: COMPOSITE, NOT ANSWERED or FOREIGN:** not seen.
- **Any UNREAD in tb2:** not seen.
- **tb1 not as EXPECT:** not seen; tb1 read as on 2 Nov, so tb2's answer is about the window.
- **`BRIDGE USD Supplier` and `BRIDGE Lab Bank`:** recorded, both PLAIN. They decide nothing alone.

**The sheet's estimates:** tb1 read as on 2 Nov. In tb2, DEBITTOTALS and CREDITTOTALS are not empty on every row: they read `0.00` on the seven PLAIN rows and are empty only on `Profit & Loss A/c`. `BRIDGE USD Customer`'s opening and closing in tb2 are plain figures, as its opening is in the movement's export at 20250402.

## Step 0: start, check and load
- Sheet files: all 10 equal to `SHA256.txt` at 6dd1cb62. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `lm.py` and `M-marks.xml` are byte-equal to row 683-LM's. `T-tb1-20250401-20250430.xml` has the same sha256 as `tb-build-request-20250401-20250430.bin` in the list. The book's GUID is in origin/master's synthetic-uuid registry and is not the excluded GUID.
- Tally was started once, at 21:36:00Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2), after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- THE COMPANY's `Company.1800` holds the book's name and GUID, and not the excluded GUID.
- C2 step 0 at 21:37:00Z, with no company loaded: the window capture, one `tally.exe`, which alone owns 9001, and `GET /status` 200 (51 B).
- At Select Company the book's name was typed: one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26. Enter, no prompt. The Gateway showed (`s0-gateway`):
  - BRIDGE FOREXGROUP LAB alone, TallyPrime with SILVER on its title (licensed: no Educational mode anywhere);
  - current period 1-Apr-25 to 31-Mar-26;
  - current date Tuesday 1-Apr-2025;
  - date of last entry 1-Apr-25;
  - "(Schedule Backup)" on the company row.
- The release on screen: TallyPrime Silver, with no release number; `tally.exe` file version 1.1.7.2. The PC's date was 2026-11-10 (IST).

## Steps 1 to 5: the reads
Each send's start and answer times are in `evidence/sends-utc.txt` (UTC) and `log-<label>.txt` (local time). Each request's hash was checked against `SHA256.txt` by `send.sh` just before it was sent.

| Step | Label | Request | Sent (UTC) | Answered (UTC) | Answer bytes |
|---|---|---|---|---|--:|
| 1 | m0 | `M-marks.xml` | 21:37:40.591 | 21:37:41.068 | 3,404 |
| 2 | tb1 | `T-tb1-20250401-20250430.xml` | 21:37:48.230 | 21:37:48.645 | 14,140 |
| 3 | tb2 | `T-tb2-20250402-20250430.xml` | 21:37:56.952 | 21:37:57.339 | 13,584 |
| 5 | m1 | `M-marks.xml` | 21:38:19.876 | 21:38:20.286 | 3,404 |

Step 4, the sets, sends nothing. Right after tb2, the whole Tally window was captured (`after-tb2` at 21:37:59.741Z, `after-tb2-desktop`): the Gateway, the book alone, no dialog. No other screen was opened in the row.

## Step 6: close (C10)
- At 21:38:32Z (C11, after the last request): one `tally.exe`, which alone owns 9001, and `GET /status` 200.
- Shut by K: Company (clicked), h, BRIDGE FOREXGROUP LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). At 21:39:04Z: one `tally.exe` owning 9001, and `GET /status` 200 (51 B). No XML request was sent after the shut.
- Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company). No prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **No dialog came at any point.** Go To was not used. The K: Company label was clicked for each shut, and Tally's own title bar (its empty part) was clicked before each key so that the key reached Tally.
- **`send.sh` runs `lm.py` for the marks request only.** `tb.py` was run by hand for each Trial Balance answer, with the options the sheet names; each output is in `evidence/`, whole.
- **The Gateway's right-hand panel** showed a red line "N: Manage Rental License"; it was not pressed.
- **The staging checks.** The start-up check found no hit in the answers, the logs, the evidence or this file, so the amount exception of row 683-LM was not needed. The 10+ digit check sets aside this book's registered synthetic GUID, matched whole, whose last part holds a ten-digit run.

## Not measured
Every item the sheet lists as not in this row. Also not measured:
- Why Tally gives a dollar ledger's carried balance as a plain figure in a window with no entry, and its closing as a composite in a window with entries.
- Why `Profit & Loss A/c` alone keeps a composite in tb2.
- Whether the gap of -1,999,958.75 among the tb2 openings is carried by any ledger outside the answer, or by none.
- Master's own `parse_native_trial_balance_with_currency` on these answers; by the sheet, it is the run that counts when they become fixtures.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC; those are listed by size and sha256 in `kept-local.txt`, with the reason.
- `logs/`: `run.py`'s own logs.
- `evidence/`: the notes (UTC), `lm.py`'s lines for m0 and m1, the `--same-row` output, `tb.py`'s outputs for tb1, tb2 (`--question`) and the sets, and the send times. The process and port listings stay local.
- `scripts/`: the step 0 helper, the window capture helper and the send helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## lm.py and tb.py outputs, whole

### m0 (lm.py)
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### tb1 (tb.py)
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

### tb2 (tb.py --question)
```
a-tb2-T-tb2-20250402-20250430.xml: HEADER/STATUS '1'
  LEDGER rows: 8
  'BRIDGE Export Sales' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '0.00' PLAIN
  'BRIDGE INR Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '-3000.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '-3000.00' PLAIN
  'BRIDGE Import Purchases' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '0.00' PLAIN
  'BRIDGE Lab Bank' CURRENCYNAME '₹': PLAIN
    TBALOPENING '-79200.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '-79200.00' PLAIN
  'BRIDGE USD Customer' CURRENCYNAME '₹': PLAIN
    TBALOPENING '-87912000.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '-87912000.00' PLAIN
  'BRIDGE USD Supplier' CURRENCYNAME '₹': PLAIN
    TBALOPENING '0.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '0.00' PLAIN
  'Cash' CURRENCYNAME '₹': PLAIN
    TBALOPENING '-3000.00' PLAIN
    DEBITTOTALS '0.00' PLAIN
    CREDITTOTALS '0.00' PLAIN
    TBALCLOSING '-3000.00' PLAIN
  'Profit & Loss A/c' CURRENCYNAME '₹': MIXED
    TBALOPENING '$999900.00 @ ₹ 86.0058/$ = ₹ 85997241.25' COMPOSITE
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '' EMPTY
    TBALCLOSING '$999900.00 @ ₹ 86.0058/$ = ₹ 85997241.25' COMPOSITE
SET FOREIGN (0): []
SET MIXED (1): ['Profit & Loss A/c']
SET PLAIN (7): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash']
SET UNREAD (0): []
WATCH 'BRIDGE USD Customer': PLAIN; TBALOPENING PLAIN, DEBITTOTALS PLAIN, CREDITTOTALS PLAIN, TBALCLOSING PLAIN
WATCH 'BRIDGE USD Supplier': PLAIN; TBALOPENING PLAIN, DEBITTOTALS PLAIN, CREDITTOTALS PLAIN, TBALCLOSING PLAIN
WATCH 'BRIDGE Lab Bank': PLAIN; TBALOPENING PLAIN, DEBITTOTALS PLAIN, CREDITTOTALS PLAIN, TBALCLOSING PLAIN
QUESTION: NO COMPOSITE
exit 0
```

### the sets, tb1 against tb2 (tb.py --sets)
```
a-tb1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-tb2-T-tb2-20250402-20250430.xml: HEADER/STATUS '1'
a-tb1-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-tb2-T-tb2-20250402-20250430.xml:
SET FOREIGN (0): []
SET MIXED (1): ['Profit & Loss A/c']
SET PLAIN (7): ['BRIDGE Export Sales', 'BRIDGE INR Customer', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Cash']
SET UNREAD (0): []
  'BRIDGE Export Sales': MIXED -> PLAIN
  'BRIDGE Import Purchases': MIXED -> PLAIN
  'BRIDGE Lab Bank': MIXED -> PLAIN
  'BRIDGE USD Customer': MIXED -> PLAIN
  'BRIDGE USD Supplier': MIXED -> PLAIN
SETS DIFFER
exit 0
```

### m1 (lm.py)
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### m1 against m0 (lm.py --same-row)
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
