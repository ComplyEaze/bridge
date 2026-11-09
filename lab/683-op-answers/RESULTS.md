# Row 683-OP: on the several-currency lab book, a dollar opening keyed on a new ledger master reads as a composite at the book's first day in all three reads; QUESTION: COMPOSITE

Run on 9 Nov 2026, 22:27:30Z to 22:44:48Z (03:57 to 04:14 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2), by the sheet at `lab/683-op-sheet` (4847edd6), on `BRIDGE FOREXGROUP LAB` (GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, synthetic, in master's registry), loaded alone in a sitting of its own. **One write, behind a backup:** one ledger master was created on Tally's screen, read three ways, and the book was then restored from its backup and read again. Requests went one at a time through `run.py`. Every value below is M, measured by this run, in one run: **partial** (one book, one ledger, one foreign currency, one release). Tally's current period was 1-Apr-25 to 31-Mar-26 and its current date Tuesday 1-Apr-2025 throughout; neither was changed. Every read was answered, each with STATUS 1, each in under one second; the 5-minute wedge rule was never near.

This sitting ran after row 2162-RP (Tally quit at 22:21:36Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end.

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 11, ALTMSTID 214. EXPECT met.
- **The controls, the book as found:** as EXPECT.
  - `s0`: 8 rows, every CURRENCYNAME the rupee sign, every OPENINGBALANCE `0.00` PLAIN, no `BRIDGE USD Opening`. The row count is recorded: 8.
  - `x0`: 8 rows, each opening `0.00` PLAIN; the answer is byte-equal to row 683-LM's `lx1`.
  - `t0`: byte-equal to row 683-TB's `tb1`; SET FOREIGN (0), SET MIXED (6), SET PLAIN (2), SET UNREAD (0).
- **BK** to `C:\lab\bk-683op`: one file, 37,618 B, sha256 `935219fba98a0d4dc9cb8395dc1b62cefd97761010938565873de919fcff9d22`; the Restore list showed 10-Nov-26 (4:00 am). `mb` and `mc`: SAME MARKS (11/214).
- **The write:** `BRIDGE USD Opening` under `Sundry Debtors`, bill-by-bill Yes, no Currency of Ledger field, Opening Balance (on 1-Apr-25) `$500` keyed. No rate-of-exchange box opened; the field showed `$500.00 @ <rupee> 88/$ = <rupee> 44,000.00`, Dr as offered. One Bill-wise Breakup line, `OPBILL`, 31/3/2025, 44,000.00 Dr. Saved by `Accept ?` y. Two deviations from the sheet's estimate, below.
- **m1:** MARKS DIFFER (exit 2), 11/214 -> 11/215: ALTVCHID as `mc`'s, ALTMSTID one above it. EXPECT met.
- **s1, the ledger master read:** 9 rows. `BRIDGE USD Opening`: PARENT `Sundry Debtors`, CURRENCYNAME the rupee sign, ISBILLWISEON `Yes`, OPENINGBALANCE and CLOSINGBALANCE both `-$500.00 @ <rupee> 88/$ = -<rupee> 44000.00`, COMPOSITE. **`op.py --question`'s last line: `QUESTION: COMPOSITE (OPENINGBALANCE, CLOSINGBALANCE)`.** `--others` against `s0`: OTHERS SAME (rows 8 and 9; the new ledger alone not compared).
- **x1, the movement export's opening at 20250401 (rule 3's input):** 9 rows. `BRIDGE USD Opening` (GUID ending `-000000d5`, PARENT `Sundry Debtors`): OPENINGBALANCE `-$500.00 @ <rupee> 88/$ = -<rupee> 44000.00`, COMPOSITE. **`op.py --question`'s last line: `QUESTION: COMPOSITE (OPENINGBALANCE)`.** `--others` against `x0`: OTHERS SAME.
- **t1, the currency Trial Balance from 20250401 to 20250430 (rule 1's input):** 9 LEDGER rows, every CURRENCYNAME the rupee sign. The whole Tally window right after it showed the Gateway, no dialog (`after-t1`).
  - **`BRIDGE USD Opening`: MIXED.** TBALOPENING and TBALCLOSING `-$500.00 @ <rupee> 88/$ = -<rupee> 44000.00` (COMPOSITE), DEBITTOTALS and CREDITTOTALS empty.
  - SET FOREIGN (0); SET MIXED (7); SET PLAIN (2); SET UNREAD (0).
  - **`tb.py --question`'s last line: `QUESTION: COMPOSITE (TBALOPENING, TBALCLOSING)`.**
  - `--sets` against `t0`: SETS DIFFER, naming `BRIDGE USD Opening` alone (no row -> MIXED). Every other ledger keeps its set.
- **m1r:** SAME MARKS as m1 (11/215), byte-equal to it. The reads wrote nothing.
- **Restore** from `bk-683op` (Accept y, Replace r), then **m2:** SAME MARKS as m0 (11/214), byte-equal to m0 (sha256 `3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba`). **s2:** ROWS SAME as s0, 8 rows, no `BRIDGE USD Opening`; the answer is byte-equal to s0's. EXPECT met: the restore returned the book to m0.

**By the sheet's Decides:**
- **`t1` COMPOSITE:** measured. Rule 1 sees a dollar opening keyed on a ledger master at the book's first day, on this book and release, so the classified path at `from` equal to the book's first day stands as agreed: **partial** (one book, one release). Rule 3 (`x1`) sees it too.
- **`t1` NO COMPOSITE (with `x1` either way), NOT ANSWERED, EMPTY, OTHER, or UNREAD rows:** not seen.
- **No dollar opening could be keyed:** not so; the field took `$500`.
- **OTHERS DIFFER, or `--sets` naming another ledger:** not seen. The write changed one ledger.

**The sheet's estimates:** one row more in `s1` and `x1` than in `s0` and `x0`, and a `BRIDGE USD Opening` row in `t1`, the other eight unchanged: as estimated. The opening Tally keeps for `$500` at 88 is 44,000.00 rupees, Dr, written negative in XML: as estimated. No row without the book's GUID appeared. The rate 88 was not keyed: Tally offered it (see Deviations).

## Step 0: start, check and load
- Sheet files: all 11 equal to `SHA256.txt` at 4847edd6. `LAB-COMMON.md`, `run.py`, `lm.py`, `M-marks.xml` and `T-tb1-20250401-20250430.xml` are byte-equal to row 683-TB's. The book's GUID is in origin/master's synthetic-uuid registry and is not the excluded GUID.
- Tally was started once, at 22:27:30Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2), after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- C2 step 0 at 22:28:19Z, with no company loaded: the window capture, one `tally.exe`, which alone owns 9001, and `GET /status` 200 (51 B).
- At Select Company the book's name was typed: one row, BRIDGE FOREXGROUP LAB, 1-Apr-25 to 31-Mar-26. Enter, no prompt. The Gateway showed (`s0-gateway`):
  - BRIDGE FOREXGROUP LAB alone, TallyPrime with SILVER on its title (licensed: no Educational mode anywhere);
  - current period 1-Apr-25 to 31-Mar-26;
  - current date Tuesday 1-Apr-2025;
  - date of last entry 1-Apr-25;
  - "(Schedule Backup)" on the company row.
- The release on screen: TallyPrime Silver, with no release number; `tally.exe` file version 1.1.7.2. The PC's date was 2026-11-10 (IST).

## The sends
Each send's start and answer times are in `evidence/sends-utc.txt` (UTC) and `log-<label>.txt` (local time). Each request's hash was checked against `SHA256.txt` by `send.sh` just before it was sent.

| Step | Label | Request | Sent (UTC) | Answered (UTC) | Answer bytes |
|---|---|---|---|---|--:|
| 1 | m0 | `M-marks.xml` | 22:28:53.575 | 22:28:54.208 | 3,404 |
| 2 | s0 | `S-ls-20250401-20250430.xml` | 22:29:03.579 | 22:29:04.157 | 13,354 |
| 2 | x0 | `X-lx1-20250401.xml` | 22:29:12.803 | 22:29:13.282 | 11,556 |
| 2 | t0 | `T-tb1-20250401-20250430.xml` | 22:29:14.135 | 22:29:14.567 | 14,140 |
| 3 | mb | `M-marks.xml` | 22:30:54.245 | 22:30:54.729 | 3,404 |
| 4 | mc | `M-marks.xml` | 22:31:03.004 | 22:31:03.429 | 3,404 |
| 5 | m1 | `M-marks.xml` | 22:38:08.458 | 22:38:08.993 | 3,404 |
| 6 | s1 | `S-ls-20250401-20250430.xml` | 22:38:18.042 | 22:38:18.394 | 14,710 |
| 6 | x1 | `X-lx1-20250401.xml` | 22:38:26.440 | 22:38:26.868 | 12,678 |
| 6 | t1 | `T-tb1-20250401-20250430.xml` | 22:39:43.820 | 22:39:44.204 | 15,526 |
| 7 | m1r | `M-marks.xml` | 22:40:11.076 | 22:40:11.466 | 3,404 |
| 8 | m2 | `M-marks.xml` | 22:43:48.658 | 22:43:49.143 | 3,404 |
| 8 | s2 | `S-ls-20250401-20250430.xml` | 22:43:50.789 | 22:43:51.471 | 13,354 |

## Step 3: BK (C5)
- `C:\lab\bk-683op` did not exist before; it was made empty, and the backup was written into it. Data label, Backup & Restore, Backup; the cursor opened on Company Name with the path `C:\`; Backspace moved it to the path; Specify Path, `C:\lab\bk-683op` typed, its row taken; BRIDGE FOREXGROUP LAB alone, End of List; `Accept ?` captured (`bk-accept`, 22:30:36Z) and answered y. No other message; back at the Gateway.
- The folder then held one file, `TDBK1800_<company number>.001`, 37,618 B, sha256 `935219fba98a0d4dc9cb8395dc1b62cefd97761010938565873de919fcff9d22`. The folder listings before and after stay on the lab PC.

## Step 4: the write, word for word as the screen showed it
- Gateway, Create (clicked), List of Masters, Ledger: Ledger Creation opened (`w1-ledger-creation`); Under offered `Capital Account`.
- **Name:** `BRIDGE USD Opening` typed, Enter; alias Enter.
- **Under:** `Sundry Debtors` typed; one row listed; Enter.
- **Maintain balances bill-by-bill:** offered `Yes`, Enter. Default credit period empty, Enter; Check for credit days `No`, Enter.
- **Currency of Ledger:** no such field was shown.
- Mailing Name offered `BRIDGE USD Opening`, Enter; Address empty, Enter; State offered `Not Applicable`, Enter; Country offered `India`, Enter; Pincode Enter; Provide bank details `No`, Enter; PAN/IT No. empty, Enter; Registration type offered `Regular`, Enter; GSTIN/UIN empty, Enter.
- **A Warning box came** (the prompt rule; see Deviations): title `Warning`, text `GSTIN/UIN not specified`, footer `Press any key to continue` (`w2-gstin-warning`, 22:33:54Z). Nothing was pressed until the run lead's word; then Enter once dismissed it (`w3-after-warning`, 22:34:58Z), with the cursor on Set/Alter additional GST details `No`, Enter.
- **Opening Balance (on 1-Apr-25):** `$500` typed, Enter. **No rate-of-exchange box opened.** The field then showed `$500.00 @ <rupee> 88/$ = <rupee> 44,000.00`, with the cursor on Dr/Cr offered `Dr`. Top right: Total Opening Balance 44,000.00 Dr; Difference 44,000.00 Dr (`w4-opening-no-rate-box`, 22:35:33Z).
- **Dr or Cr:** `Dr` as offered, Enter.
- **Bill-wise Breakup** opened: title `Bill-wise Breakup of : BRIDGE USD Opening`, `Upto: <rupee> 44,000.00 Dr`; columns Date, Name, Due Date or Credit Days, Amount, Dr/Cr; no type-of-reference column; `On Account: 44,000.00 Dr` (`w5-billwise-open`). The line: Date offered `31/3/2025`, Enter; Name empty, `OPBILL` typed, Enter; Due Date, or Credit Days offered `31-Mar-25`, Enter; Amount offered `44,000.00` (a rupee figure, no dollar part, no rate asked), Enter; Dr/Cr offered `Dr`, Enter (`w6-billwise-line`). An empty line was offered; Enter on its first field closed the breakup.
- **Accept:** the master showed its own `Accept ?` (Yes or No) at once, the opening being its last field, so no Ctrl+A was pressed. The screen before answering (`w7-before-accept`, 22:37:19Z): Opening Balance (on 1-Apr-25) `$500.00 @ <rupee> 88/$ = <rupee> 44,000.00  Dr`; Total Opening Balance 44,000.00 Dr; Difference 44,000.00 Dr; Registration type `Regular`, GSTIN/UIN empty, Set/Alter additional GST details `No`. Answered y.
- A blank Ledger Creation followed (`w8-blank-ledger-creation`, 22:37:38Z; Total Opening Balance 44,000.00 Dr, Difference 44,000.00 Dr). Its title was captured, then Ctrl+Q went to the List of Masters and Esc to the Gateway (`w9-gateway`), no dialog. C2 check at 22:38:04Z: ok.

## Step 8: the restore (C6)
- Before the shut (22:40:19Z): the BK re-hashed equal to the record above, its folder holding that one file; `restore-plan.txt` written; the data folder listed (24 entries). The plan and listings stay on the lab PC: they name the company number.
- Shut by K: Company (clicked), h, Enter, no prompt. Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`. Specify Path, `C:\lab\bk-683op` typed. The list showed one row: BRIDGE FOREXGROUP LAB, `(<company number>)`, `10-Nov-26 (4:00 am)`. Enter on it, Enter on End of List.
- `Accept ?` (Yes or No): captured (`8-restore-accept`, 22:42:19Z), y.
- Then, word for word: `Company data folder (<company number>) already exists.` / `Selecting Skip will cancel the Restore process for this company.` / `Selecting Replace will overwrite the existing folder.`, with buttons `R Replace` and `S Skip`: captured (`8-restore-replace`, 22:42:37Z), r. Select Company followed, with no other message.
- The data folder after: 17 entries. Gone, against the listing before: `CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`, `TUPDATE.TSF`.
- The book was loaded alone again (its name typed, one row, Enter, no prompt): the Gateway with the same period, date and last entry. C2 check at 22:43:33Z: ok. Then m2 and s2, as above.

## Step 9: close (C10)
- At 22:44:03Z (C11, after the last request): one `tally.exe`, which alone owns 9001, and `GET /status` 200.
- Shut by K: Company (clicked), h, BRIDGE FOREXGROUP LAB alone, Enter, no prompt. Select Company with no company loaded. At 22:44:29Z: one `tally.exe` owning 9001, and `GET /status` 200 (51 B). No XML request was sent after the shut.
- Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company). No prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **The GSTIN warning.** With Registration type `Regular` (as offered) and GSTIN/UIN left empty, Tally showed, word for word: `Warning` / `GSTIN/UIN not specified` / `Press any key to continue`. The sheet did not name it, so by the prompt rule nothing was pressed and it was reported. On the run lead's word, Enter once dismissed it and the write went on with Enter only. **The ledger was saved with Registration type `Regular` and no GSTIN, as Tally offered.** `--others` and `--sets` show no other ledger changed.
- **No rate box.** Tally offered the rate 88 itself and filled the composite at once; no rate was keyed. The sheet's step 6 covers this ("write down exactly what the field then shows, and go on"). On the run lead's first word the row paused at that point, with nothing pressed; the run lead then confirmed step 6.
- **No Ctrl+A.** The master asked `Accept ?` by itself when the breakup closed, the opening being its last field; it was captured, then answered y.
- **The Bill-wise Breakup** asked no type of reference, so `New Ref` was not keyed; its amount was a rupee figure.
- **`send.sh` runs `lm.py` for the marks requests only.** `op.py` and `tb.py` were run by hand for each answer, with the options the sheet names; each output is in `evidence/`, whole.
- **The Gateway's right-hand panel** showed a red line "N: Manage Rental License"; it was not pressed. The K: Company label was clicked for each shut, and Tally's own title bar (its empty part) was clicked before each key so that the key reached Tally.
- **The staging checks.** The start-up check found no hit in the answers, the logs, the evidence or this file, so the amount exception of row 683-LM was not needed. The 10+ digit check sets aside this book's registered synthetic GUID, matched whole, whose last part holds a ten-digit run.
- **The notes file:** the lines for m0, mb, m1, s1, x1, t1, m1r, the restore prompts, m2 and s2 were written after the close from `sends-utc.txt` and the evidence files; that block says so.

## Not measured
Every item the sheet lists as not in this row. Also not measured:
- Whether a dollar opening keyed with a rate other than the one Tally offers, or on a ledger whose own currency is the dollar, reads the same.
- Whether Tally's offered rate 88 comes from the book's last voucher rate or from another master.
- Whether the Difference in opening balances (44,000.00 Dr) is carried by any ledger outside the answers.
- Master's own parsers on these answers; by the sheet, it is the run that counts when they become fixtures.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC; those are listed by size and sha256 in `kept-local.txt`, with the reason.
- `logs/`: `run.py`'s own logs.
- `evidence/`: the notes (UTC), `lm.py`'s lines and `--same-row` outputs for every marks read, `op.py`'s and `tb.py`'s outputs, the BK record and the send times. The process, port, data folder and backup folder listings stay local.
- `scripts/`: the step 0 helper, the window capture helper, the send helper and the restore prep helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## lm.py, op.py and tb.py outputs, whole

### m0 (lm.py)
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
exit 0
```

### s0 (op.py)
```
a-s0-S-ls-20250401-20250430.xml: HEADER/STATUS '1'
  LEDGER rows: 8
  'BRIDGE Export Sales' PARENT 'Sales Accounts' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25' COMPOSITE
  'BRIDGE INR Customer' PARENT 'Sundry Debtors' CURRENCYNAME '₹' ISBILLWISEON 'Yes'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-3000.00' PLAIN
  'BRIDGE Import Purchases' PARENT 'Purchase Accounts' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-$200.00 @ ₹ 86/$ = -₹ 17400.00' COMPOSITE
  'BRIDGE Lab Bank' PARENT 'Bank Accounts' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-$900.00 @ ₹ 88/$ = -₹ 79200.00' COMPOSITE
  'BRIDGE USD Customer' PARENT 'Sundry Debtors' CURRENCYNAME '₹' ISBILLWISEON 'Yes'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-$999000.00 @ ₹ 88/$ = -₹ 87912000.00' COMPOSITE
  'BRIDGE USD Supplier' PARENT 'Sundry Creditors' CURRENCYNAME '₹' ISBILLWISEON 'Yes'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '$0.00 @ ₹ /$ = ₹ 0.00' COMPOSITE
  'Cash' PARENT 'Cash-in-Hand' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-3000.00' PLAIN
  'Profit & Loss A/c' PARENT '�#4; Primary' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '$999900.00 @ ₹ 86.0058/$ = ₹ 85997241.25' COMPOSITE
exit 0
```

### x0 (op.py)
```
a-x0-X-lx1-20250401.xml: HEADER/STATUS '1'
  LEDGER rows: 8
  'BRIDGE Export Sales' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d1' PARENT 'Sales Accounts'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE INR Customer' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d2' PARENT 'Sundry Debtors'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE Import Purchases' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d4' PARENT 'Purchase Accounts'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE Lab Bank' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d0' PARENT 'Bank Accounts'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE USD Customer' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000cf' PARENT 'Sundry Debtors'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE USD Supplier' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d3' PARENT 'Sundry Creditors'
    OPENINGBALANCE '0.00' PLAIN
  'Cash' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-0000001f' PARENT 'Cash-in-Hand'
    OPENINGBALANCE '0.00' PLAIN
  'Profit & Loss A/c' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-0000001e' PARENT '�#4; Primary'
    OPENINGBALANCE '0.00' PLAIN
exit 0
```

### t0 (tb.py)
```
a-t0-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
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

### mb against m0 (lm.py --same-row)
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-mb-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
```

### mc against mb (lm.py --same-row)
```
a-mb-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-mc-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
SAME MARKS
```

### m1 against mc (lm.py --same-row)
```
a-mc-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 214
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 215
MARKS DIFFER: ('1bb48be5-baba-45b2-90d8-7593244915a9', '11', '214') -> ('1bb48be5-baba-45b2-90d8-7593244915a9', '11', '215')
exit 2
```

### s1 (op.py --question)
```
a-s1-S-ls-20250401-20250430.xml: HEADER/STATUS '1'
  LEDGER rows: 9
  'BRIDGE Export Sales' PARENT 'Sales Accounts' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '$1000100.00 @ ₹ 86/$ = ₹ 86014641.25' COMPOSITE
  'BRIDGE INR Customer' PARENT 'Sundry Debtors' CURRENCYNAME '₹' ISBILLWISEON 'Yes'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-3000.00' PLAIN
  'BRIDGE Import Purchases' PARENT 'Purchase Accounts' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-$200.00 @ ₹ 86/$ = -₹ 17400.00' COMPOSITE
  'BRIDGE Lab Bank' PARENT 'Bank Accounts' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-$900.00 @ ₹ 88/$ = -₹ 79200.00' COMPOSITE
  'BRIDGE USD Customer' PARENT 'Sundry Debtors' CURRENCYNAME '₹' ISBILLWISEON 'Yes'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-$999000.00 @ ₹ 88/$ = -₹ 87912000.00' COMPOSITE
  'BRIDGE USD Opening' PARENT 'Sundry Debtors' CURRENCYNAME '₹' ISBILLWISEON 'Yes'
    OPENINGBALANCE '-$500.00 @ ₹ 88/$ = -₹ 44000.00' COMPOSITE
    CLOSINGBALANCE '-$500.00 @ ₹ 88/$ = -₹ 44000.00' COMPOSITE
  'BRIDGE USD Supplier' PARENT 'Sundry Creditors' CURRENCYNAME '₹' ISBILLWISEON 'Yes'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '$0.00 @ ₹ /$ = ₹ 0.00' COMPOSITE
  'Cash' PARENT 'Cash-in-Hand' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '-3000.00' PLAIN
  'Profit & Loss A/c' PARENT '�#4; Primary' CURRENCYNAME '₹' ISBILLWISEON 'No'
    OPENINGBALANCE '0.00' PLAIN
    CLOSINGBALANCE '$999900.00 @ ₹ 86.0058/$ = ₹ 85997241.25' COMPOSITE
QUESTION: COMPOSITE (OPENINGBALANCE, CLOSINGBALANCE)
exit 0
```

### s0 against s1 (op.py --others)
```
a-s0-S-ls-20250401-20250430.xml: HEADER/STATUS '1'
a-s1-S-ls-20250401-20250430.xml: HEADER/STATUS '1'
  rows: 8 and 9
  'BRIDGE USD Opening': in a-s0-S-ls-20250401-20250430.xml no, in a-s1-S-ls-20250401-20250430.xml yes (not compared)
OTHERS SAME
exit 0
```

### x1 (op.py --question)
```
a-x1-X-lx1-20250401.xml: HEADER/STATUS '1'
  LEDGER rows: 9
  'BRIDGE Export Sales' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d1' PARENT 'Sales Accounts'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE INR Customer' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d2' PARENT 'Sundry Debtors'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE Import Purchases' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d4' PARENT 'Purchase Accounts'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE Lab Bank' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d0' PARENT 'Bank Accounts'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE USD Customer' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000cf' PARENT 'Sundry Debtors'
    OPENINGBALANCE '0.00' PLAIN
  'BRIDGE USD Opening' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d5' PARENT 'Sundry Debtors'
    OPENINGBALANCE '-$500.00 @ ₹ 88/$ = -₹ 44000.00' COMPOSITE
  'BRIDGE USD Supplier' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-000000d3' PARENT 'Sundry Creditors'
    OPENINGBALANCE '0.00' PLAIN
  'Cash' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-0000001f' PARENT 'Cash-in-Hand'
    OPENINGBALANCE '0.00' PLAIN
  'Profit & Loss A/c' GUID '1bb48be5-baba-45b2-90d8-7593244915a9-0000001e' PARENT '�#4; Primary'
    OPENINGBALANCE '0.00' PLAIN
QUESTION: COMPOSITE (OPENINGBALANCE)
exit 0
```

### x0 against x1 (op.py --others)
```
a-x0-X-lx1-20250401.xml: HEADER/STATUS '1'
a-x1-X-lx1-20250401.xml: HEADER/STATUS '1'
  rows: 8 and 9
  'BRIDGE USD Opening': in a-x0-X-lx1-20250401.xml no, in a-x1-X-lx1-20250401.xml yes (not compared)
OTHERS SAME
exit 0
```

### t1 (tb.py --question)
```
a-t1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
  LEDGER rows: 9
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
  'BRIDGE USD Opening' CURRENCYNAME '₹': MIXED
    TBALOPENING '-$500.00 @ ₹ 88/$ = -₹ 44000.00' COMPOSITE
    DEBITTOTALS '' EMPTY
    CREDITTOTALS '' EMPTY
    TBALCLOSING '-$500.00 @ ₹ 88/$ = -₹ 44000.00' COMPOSITE
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
SET MIXED (7): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Opening', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
WATCH 'BRIDGE USD Opening': MIXED; TBALOPENING COMPOSITE, DEBITTOTALS EMPTY, CREDITTOTALS EMPTY, TBALCLOSING COMPOSITE
WATCH 'BRIDGE USD Customer': MIXED; TBALOPENING PLAIN, DEBITTOTALS COMPOSITE, CREDITTOTALS COMPOSITE, TBALCLOSING COMPOSITE
WATCH 'Profit & Loss A/c': MIXED; TBALOPENING PLAIN, DEBITTOTALS EMPTY, CREDITTOTALS EMPTY, TBALCLOSING COMPOSITE
QUESTION: COMPOSITE (TBALOPENING, TBALCLOSING)
exit 0
```

### the sets, t0 against t1 (tb.py --sets)
```
a-t0-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-t1-T-tb1-20250401-20250430.xml: HEADER/STATUS '1'
a-t0-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (6): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
a-t1-T-tb1-20250401-20250430.xml:
SET FOREIGN (0): []
SET MIXED (7): ['BRIDGE Export Sales', 'BRIDGE Import Purchases', 'BRIDGE Lab Bank', 'BRIDGE USD Customer', 'BRIDGE USD Opening', 'BRIDGE USD Supplier', 'Profit & Loss A/c']
SET PLAIN (2): ['BRIDGE INR Customer', 'Cash']
SET UNREAD (0): []
  'BRIDGE USD Opening': no row -> MIXED
SETS DIFFER
exit 0
```

### m1r against m1 (lm.py --same-row)
```
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 215
a-m1r-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 1bb48be5-baba-45b2-90d8-7593244915a9 (the book's: yes) ALTVCHID 11 ALTMSTID 215
SAME MARKS
exit 0
```

### m2 against m0 (lm.py --same-row)
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

### s2 against s0 (op.py --same)
```
a-s0-S-ls-20250401-20250430.xml: HEADER/STATUS '1'
a-s2-S-ls-20250401-20250430.xml: HEADER/STATUS '1'
  rows: 8 and 8
ROWS SAME
exit 0
```
