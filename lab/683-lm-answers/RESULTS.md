# Row 683-LM: on the several-currency lab book, the movement reads equal the `vouchers` capture, but five of eight ledgers' openings do not tie to the day's movement

Run on 9 Nov 2026, 20:41:01Z to 20:45:50Z (02:11 to 02:15 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2), by the sheet at `lab/683-lm-sheet` (f3a10737), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic, in master's registry), loaded alone in a sitting of its own. **Read only:** seven raw reads, one at a time through `run.py`, with bytes rendered by master's builders; no write, no backup, no restore, and no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one book, one day, one foreign currency, one release). Tally's current period was 1-Apr-25 to 31-Mar-26 and its current date Tuesday 1-Apr-2025 throughout; neither was changed. Every read was answered, each with STATUS 1, each in under 2 seconds; the 5-minute wedge rule was never near.

This sitting ran after row 2162-HS (Tally quit at 20:29:12Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end.

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214, as row 683-B left it. EXPECT met.
- **cur:** two CURRENCY rows, `$` (USD, 2 places) and the rupee sign (INR, 2 places). EXPECT met.
- **mv, the book-scoped movement window for 2025-04-01:** 11 vouchers. Against the `vouchers` capture of the same day, every voucher is SAME, composites included: **CAPTURE SAME**. The window right after it showed the Gateway, no dialog.
- **mvl, the ledger-scoped window on `BRIDGE USD Customer`:** 4 vouchers, all composites, each entry's AMOUNT text equal to mv's: **LEDGER SAME**.
- **lx1, the ledger export at `from` 20250401:** 8 ledgers, every OPENINGBALANCE `0.00`.
- **lx2, the ledger export at `from` 20250402:** 8 ledgers. Seven openings are plain figures; one, `Profit & Loss A/c`, is a composite: `$999900.00 @ <rupee sign> 86.0058/$ = <rupee sign> 85997241.25`.
- **The tie (lx1 opening + mv's net on the ledger = lx2 opening, by the base part):**

| Ledger | lx1 | mv net (rupee part) | lx2 | Tie |
|---|--:|--:|--:|---|
| BRIDGE Export Sales | 0.00 | 86,014,641.25 | 0.00 | DIFFERS by -86,014,641.25 |
| BRIDGE INR Customer | 0.00 | -3,000.00 | -3,000.00 | TIES |
| BRIDGE Import Purchases | 0.00 | -17,400.00 | 0.00 | DIFFERS by 17,400.00 |
| BRIDGE Lab Bank | 0.00 | -79,200.00 | -79,200.00 | TIES |
| BRIDGE USD Customer | 0.00 | -85,911,941.25 | -87,912,000.00 | DIFFERS by -2,000,058.75 |
| BRIDGE USD Supplier | 0.00 | -100.00 | 0.00 | DIFFERS by 100.00 |
| Cash | 0.00 | -3,000.00 | -3,000.00 | TIES |
| Profit & Loss A/c | 0.00 | 0 (no entry) | 85,997,241.25 | DIFFERS by 85,997,241.25 |

  No ledger is NOT A FIGURE.
- **m1:** SAME MARKS (11/214), and m1 is byte-equal to m0. Nothing moved.

**What the figures add up to** (arithmetic on the recorded answers, by script, in `evidence/differs-arith.txt`; why Tally does it was not measured):
- **The two dollar party ledgers open at their dollar balance times 88.** `BRIDGE USD Customer`'s entries sum to -$999,000.00, and -999,000 x 88 = -87,912,000.00, its lx2 opening exactly. Its entries at their own rates sum to -85,911,941.25. `BRIDGE USD Supplier`'s entries sum to $0.00, and its lx2 opening is 0.00, while its entries at their own rates sum to -100.00. 88 is a rate three of the day's vouchers carry (`0008`, `000a`, `000b`). Which rate Tally takes, and why, was not measured.
- **`BRIDGE Lab Bank` ties either way:** its entries sum to -$900.00 and to -79,200.00 at their own rates, and -900 x 88 is also -79,200.00. So this day cannot show which reading its opening follows.
- **The income and expense ledgers open at 0.00, and `Profit & Loss A/c` carries their sum.** `BRIDGE Export Sales` (86,014,641.25) plus `BRIDGE Import Purchases` (-17,400.00) is 85,997,241.25, P&L's lx2 base part. Their dollar parts, $1,000,100.00 and -$200.00, sum to $999,900.00, P&L's dollar part.
- **The eight lx2 openings sum to -1,999,958.75, not 0.** That equals the two dollar party ledgers' gaps together (-2,000,058.75 + 100.00). No ledger in the export carries the other side.

**By the sheet's Decides:**
- **`mv` and `mvl` equal the capture:** measured, both. The movement windows answer each entry's AMOUNT with the same text as `vouchers`, composites included, on this book and day. The composite parse `vouchers` uses would read them.
- **Any ledger DIFFERS:** measured, five of eight. By the sheet, a movement summed from vouchers cannot tie on a several-currency book, so such a ledger must be refused by name or answered with the gap stated, never as a balance. Which of the two is for the run lead, before any code. Two of the five, the income and expense ledgers, differ for a reason the arithmetic shows on its face (their openings move to `Profit & Loss A/c`), and it is not a currency effect. The two dollar party ledgers differ by what the figures show as a revaluation at 88.
- **`mv` differs from the capture:** not seen.
- **Any NOT A FIGURE:** not seen.

**The sheet's estimates:** the movement AMOUNT texts did equal the capture's. Of the openings at lx2, only `Profit & Loss A/c`'s is a composite; the two dollar party ledgers' are plain figures. At least one dollar-touched ledger did DIFFER, as the 26 Sep note on #683 suggested.

## Step 0: start, check and load
- Sheet files: all 13 equal to `SHA256.txt` at f3a10737. `run.py` and `LAB-COMMON.md` are byte-equal to row 2162-GF's. The book's GUID is in origin/master's synthetic-uuid registry and is not the excluded GUID.
- Tally was started once, at 20:41:01Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2), after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- At Select Company the book's name was typed: one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26. THE COMPANY's `Company.1800` holds the book's name and GUID, and not the excluded GUID.
- C2 step 0 at 20:42:45Z, with no company loaded: the window capture, one `tally.exe`, which alone owns 9001, and `GET /status` 200 (51 B). One more `GET /status` was sent by hand right after: 200, 51 B.
- The book was loaded alone (Enter on its one row, no prompt). The Gateway showed (`s0-gateway`):
  - BRIDGE FOREXGROUP LAB alone, TallyPrime with SILVER on its title (licensed: no Educational mode anywhere);
  - current period 1-Apr-25 to 31-Mar-26;
  - current date Tuesday 1-Apr-2025;
  - date of last entry 1-Apr-25;
  - "(Schedule Backup)" on the company row.
- The PC's date was 2026-11-10 (IST).

## Steps 1 to 8: the reads
Each send's start and answer times are in `evidence/sends-utc.txt` (UTC) and `log-<label>.txt` (local time). Each request's hash was checked against `SHA256.txt` by `send.sh` just before it was sent.

| Step | Label | Request | Sent (UTC) | Answered (UTC) | Answer bytes |
|---|---|---|---|---|--:|
| 1 | m0 | `M-marks.xml` | 20:43:20.923 | 20:43:21.550 | 3,404 |
| 2 | cur | `C-currency.xml` | 20:43:29.569 | 20:43:30.463 | 3,796 |
| 3 | mv | `V-mv-20250401.xml` | 20:43:40.300 | 20:43:41.193 | 59,524 |
| 4 | mvl | `V-mvl-usd-customer-20250401.xml` | 20:44:08.758 | 20:44:09.451 | 23,720 |
| 5 | lx1 | `X-lx1-20250401.xml` | 20:44:18.737 | 20:44:19.332 | 11,556 |
| 6 | lx2 | `X-lx2-20250402.xml` | 20:44:26.417 | 20:44:27.043 | 11,670 |
| 8 | m1 | `M-marks.xml` | 20:44:49.580 | 20:44:50.218 | 3,404 |

Step 7, the tie, sends nothing. After mv, the whole Tally window was captured at once (`s3-after-mv`, `s3-after-mv-desktop`): the Gateway, no dialog. No other screen was opened in the row.

## Step 9: close (C10)
- At 20:44:58Z (C11, after the last request): one `tally.exe`, which alone owns 9001, and `GET /status` 200.
- Shut by K: Company (clicked), h, BRIDGE FOREXGROUP LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). Then one `tally.exe` owning 9001, and `GET /status` 200 (51 B). No XML request was sent after the shut.
- Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company). No prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **One extra `GET /status`** was sent by hand at step 0, right after `step0.py`'s own. It answered 200, 51 B.
- **No dialog came at any point.** Go To was not used. The K: Company label was clicked for each shut, and Tally's own title bar (its empty part) was clicked before each key so that the key reached Tally.
- **The Gateway's right-hand panel** showed a red line "N: Manage Rental License"; it was not pressed.
- **The staging checks.** The start-up check's hits are all inside the amount $1000000.00; allowed by the run lead. There are 20 hits, in two answers, three `lm.py` outputs and this file (two of them in this line and in the listing's header as RESULTS quotes it), and the listing file's own header holds one more, in the same amount; each is listed with 12 characters either side in `evidence/startup-hits.txt`. The 10+ digit check sets aside this book's registered synthetic GUID, matched whole, whose last part holds a ten-digit run.

## Not measured
Every item the sheet lists as not in this row. Also not measured:
- Which rate Tally takes for a foreign ledger's opening, and why 88 here.
- Whether the gap of -1,999,958.75 among the lx2 openings is carried by any ledger outside the export, or by none.
- Whether the movement windows answer the same on a day with no composites, or a window longer than one day.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC; those are listed by size and sha256 in `kept-local.txt`, with the reason.
- `logs/`: `run.py`'s own logs.
- `evidence/`: the notes (UTC), `lm.py`'s lines for each answer, the `--capture`, `--ledger`, `--tie` and `--same-row` outputs, the per-ledger arithmetic, and the send times. The process and port listings stay local.
- `scripts/`: the step 0 helper, the window capture helper and the send helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## lm.py outputs, whole

### m0
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### cur
```
a-cur-C-currency.xml: HEADER/STATUS 1
  CURRENCY rows: 2
  NAME '$' MAILINGNAME 'USD' DECIMALPLACES 2
  NAME '₹' MAILINGNAME 'INR' DECIMALPLACES 2
exit 0
```

### mv
```
a-mv-V-mv-20250401.xml: HEADER/STATUS 1
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

### mvl
```
a-mvl-V-mvl-usd-customer-20250401.xml: HEADER/STATUS 1
  VOUCHER rows: 4
  1bb48be5-baba-45b2-90d8-7593244915a9-00000001 20250401 Sales '1': 2 entries
    'BRIDGE USD Customer' Yes '-$1000000.00 @ ₹ 86/$ = -₹ 86000000.00'
    'BRIDGE Export Sales' No '$1000000.00 @ ₹ 86/$ = ₹ 86000000.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000002 20250401 Sales '2': 2 entries
    'BRIDGE USD Customer' Yes '-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25'
    'BRIDGE Export Sales' No '$100.00 @ ₹ 86.4125/$ = ₹ 8641.25'
  1bb48be5-baba-45b2-90d8-7593244915a9-00000003 20250401 Receipt '1': 2 entries
    'BRIDGE USD Customer' No '$100.00 @ ₹ 87/$ = ₹ 8700.00'
    'BRIDGE Lab Bank' Yes '-$100.00 @ ₹ 87/$ = -₹ 8700.00'
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000b 20250401 Receipt '3': 2 entries
    'BRIDGE USD Customer' No '$1000.00 @ ₹ 88/$ = ₹ 88000.00'
    'BRIDGE Lab Bank' Yes '-$1000.00 @ ₹ 88/$ = -₹ 88000.00'
exit 0
```

### lx1
```
a-lx1-X-lx1-20250401.xml: HEADER/STATUS 1
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

### lx2
```
a-lx2-X-lx2-20250402.xml: HEADER/STATUS 1
  LEDGER rows: 8
  'BRIDGE Export Sales' OPENINGBALANCE '0.00' reads 0.00
  'BRIDGE INR Customer' OPENINGBALANCE '-3000.00' reads -3000.00
  'BRIDGE Import Purchases' OPENINGBALANCE '0.00' reads 0.00
  'BRIDGE Lab Bank' OPENINGBALANCE '-79200.00' reads -79200.00
  'BRIDGE USD Customer' OPENINGBALANCE '-87912000.00' reads -87912000.00
  'BRIDGE USD Supplier' OPENINGBALANCE '0.00' reads 0.00
  'Cash' OPENINGBALANCE '-3000.00' reads -3000.00
  'Profit & Loss A/c' OPENINGBALANCE '$999900.00 @ ₹ 86.0058/$ = ₹ 85997241.25' reads 85997241.25
exit 0
```

### m1
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

## Checks, whole

### mv against the vouchers capture (--capture)
```
$ python -I lm.py --capture a-mv-V-mv-20250401.xml K-vouchers-capture-20250401.xml
a-mv-V-mv-20250401.xml: HEADER/STATUS 1
K-vouchers-capture-20250401.xml: HEADER/STATUS 1
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

### mvl against mv on BRIDGE USD Customer (--ledger)
```
$ python -I lm.py --ledger a-mvl-V-mvl-usd-customer-20250401.xml a-mv-V-mv-20250401.xml "BRIDGE USD Customer"
a-mvl-V-mvl-usd-customer-20250401.xml: HEADER/STATUS 1
a-mv-V-mv-20250401.xml: HEADER/STATUS 1
  ledger 'BRIDGE USD Customer': ledger-scoped 4 vouchers, book-scoped with an entry on it 4
  1bb48be5-baba-45b2-90d8-7593244915a9-00000001: ledger-scoped ['-$1000000.00 @ ₹ 86/$ = -₹ 86000000.00'] book-scoped ['-$1000000.00 @ ₹ 86/$ = -₹ 86000000.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-00000002: ledger-scoped ['-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25'] book-scoped ['-$100.00 @ ₹ 86.4125/$ = -₹ 8641.25']
  1bb48be5-baba-45b2-90d8-7593244915a9-00000003: ledger-scoped ['$100.00 @ ₹ 87/$ = ₹ 8700.00'] book-scoped ['$100.00 @ ₹ 87/$ = ₹ 8700.00']
  1bb48be5-baba-45b2-90d8-7593244915a9-0000000b: ledger-scoped ['$1000.00 @ ₹ 88/$ = ₹ 88000.00'] book-scoped ['$1000.00 @ ₹ 88/$ = ₹ 88000.00']
LEDGER SAME
exit 0
```

### the tie (--tie)
```
$ python -I lm.py --tie a-lx1-X-lx1-20250401.xml a-mv-V-mv-20250401.xml a-lx2-X-lx2-20250402.xml
a-lx1-X-lx1-20250401.xml: HEADER/STATUS 1
a-mv-V-mv-20250401.xml: HEADER/STATUS 1
a-lx2-X-lx2-20250402.xml: HEADER/STATUS 1
  'BRIDGE Export Sales': DIFFERS by -86014641.25 (0.00 + 86014641.25 = 86014641.25, lx2 0.00)
  'BRIDGE INR Customer': TIES (0.00 + -3000.00 = -3000.00)
  'BRIDGE Import Purchases': DIFFERS by 17400.00 (0.00 + -17400.00 = -17400.00, lx2 0.00)
  'BRIDGE Lab Bank': TIES (0.00 + -79200.00 = -79200.00)
  'BRIDGE USD Customer': DIFFERS by -2000058.75 (0.00 + -85911941.25 = -85911941.25, lx2 -87912000.00)
  'BRIDGE USD Supplier': DIFFERS by 100.00 (0.00 + -100.00 = -100.00, lx2 0.00)
  'Cash': TIES (0.00 + -3000.00 = -3000.00)
  'Profit & Loss A/c': DIFFERS by 85997241.25 (0.00 + 0 = 0.00, lx2 85997241.25)
exit 0
```

### m1 against m0 (--same-row)
```
$ python -I lm.py --same-row a-m0-M-marks.xml a-m1-M-marks.xml
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
exit 0
```

### per-ledger sums of mv's entries
```
$ python -I (inline) per-ledger sums of the mv AMOUNT texts (from evidence/lm-mv.txt): entries, dollar part, rupee part
BRIDGE Export Sales: 4 entries, dollars 1000100.00, rupees 86014641.25
BRIDGE INR Customer: 2 entries, dollars -, rupees -3000.00
BRIDGE Import Purchases: 2 entries, dollars -200.00, rupees -17400.00
BRIDGE Lab Bank: 4 entries, dollars -900.00, rupees -79200.00
BRIDGE USD Customer: 4 entries, dollars -999000.00, rupees -85911941.25
BRIDGE USD Supplier: 4 entries, dollars 0.00, rupees -100.00
Cash: 2 entries, dollars -, rupees -3000.00
dollar part x 88 for each ledger with one: {'BRIDGE USD Customer': '-87912000.00', 'BRIDGE Export Sales': '88008800.00', 'BRIDGE Lab Bank': '-79200.00', 'BRIDGE USD Supplier': '0.00', 'BRIDGE Import Purchases': '-17600.00'}
```

### the start-up check's hits
```
$ python -I (inline) the start-up check's hits in every file the stage reads, each with 12 characters either side (the number shown as <S>) ("inside the amount": the hit is part of $1000000.00)
RESULTS.md (utf-8): ...the amount $<S>0.00; allowe... inside the amount
RESULTS.md (utf-8): ...mer' Yes '-$<S>0.00 @ ₹ 86/... inside the amount
RESULTS.md (utf-8): ...Sales' No '$<S>0.00 @ ₹ 86/... inside the amount
RESULTS.md (utf-8): ...mer' Yes '-$<S>0.00 @ ₹ 86/... inside the amount
RESULTS.md (utf-8): ...Sales' No '$<S>0.00 @ ₹ 86/... inside the amount
RESULTS.md (utf-8): ...-scoped ['-$<S>0.00 @ ₹ 86/... inside the amount
RESULTS.md (utf-8): ...-scoped ['-$<S>0.00 @ ₹ 86/... inside the amount
RESULTS.md (utf-8): ...is part of $<S>0.00)
RESULT... inside the amount
a-mv-V-mv-20250401.xml (utf-16-le): ...="Amount">-$<S>0.00 @ ₹ 86/... inside the amount
a-mv-V-mv-20250401.xml (utf-16-le): ...="Amount">-$<S>0.00 @ ₹ 86/... inside the amount
a-mv-V-mv-20250401.xml (utf-16-le): ...E="Amount">$<S>0.00 @ ₹ 86/... inside the amount
a-mvl-V-mvl-usd-customer-20250401.xml (utf-16-le): ...="Amount">-$<S>0.00 @ ₹ 86/... inside the amount
a-mvl-V-mvl-usd-customer-20250401.xml (utf-16-le): ...="Amount">-$<S>0.00 @ ₹ 86/... inside the amount
a-mvl-V-mvl-usd-customer-20250401.xml (utf-16-le): ...E="Amount">$<S>0.00 @ ₹ 86/... inside the amount
evidence/ledger-mvl.txt (utf-8): ...-scoped ['-$<S>0.00 @ ₹ 86/... inside the amount
evidence/ledger-mvl.txt (utf-8): ...-scoped ['-$<S>0.00 @ ₹ 86/... inside the amount
evidence/lm-mv.txt (utf-8): ...mer' Yes '-$<S>0.00 @ ₹ 86/... inside the amount
evidence/lm-mv.txt (utf-8): ...Sales' No '$<S>0.00 @ ₹ 86/... inside the amount
evidence/lm-mvl.txt (utf-8): ...mer' Yes '-$<S>0.00 @ ₹ 86/... inside the amount
evidence/lm-mvl.txt (utf-8): ...Sales' No '$<S>0.00 @ ₹ 86/... inside the amount
hits 20; not inside the amount: 0
```
