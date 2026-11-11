# Row 2480-SC: master's own CHILDOF census, movement and window reads of `qa` and `ua` each held that name's own invoice and nothing else, byte-equal to the `##LedgerName` form; the altered ledger was read by both its spellings, in every form sent, with its one invoice and nothing else

Run on 11 Nov 2026, 05:13:24Z to 05:35Z (10:43 to 11:05 IST), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`, equal to row 2480-SK's record; the Gateway shows no release number). The sheet was `lab/2480-sc-sheet` (39162ad5), with all 83 files equal to its `SHA256.txt`. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone, in a sitting of its own with a Tally start of its own. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

The two parts are judged apart, as the sheet says: each section is written from its own answers. Part B was sent only after Part A had passed in full.

## Part A: the set-up, the shape step and master's CHILDOF reads (steps 1 to 7)

**No `CHILDOF FORM:` line.** Each of the six `JUDGE` lines on a CHILDOF answer (`qa` and `ua`; census, movement, window) was `OWN YES, SIBLING NO, OTHERS 0`, and no CHILDOF answer was empty (`evidence/childof-flags.txt` was never written).

**Confidence: M, measured by this run, in one run, for each line below; PARTIAL as the sheet's Decides says (one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse).** Every send of steps 1 to 7 was answered: each with curl exit 0 and HTTP 200, with bytes; no answer was empty, so exception (a) was never needed. No dialog, message or prompt showed but C5's own (none at the load or the shut). Every capture after an import, after each catalogue read and after each send of steps 4 to 7 was the Gateway, the same image as s1's.

**Step 1.** The Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (neither changed). s1's marks were 68/245, as the sheet asks, before the backup.

**Step 2, the book before.** Each N read 0 rows; B-bills no group for any of the eleven names; the snapshot 18 LEDGER rows and "no row" for each name; `st.py` (the before option) `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; `cv.py` on the book-wide census `ANSWER: VOUCHERS 11`; `sk.py --names` on the catalogue 18 LEDGER rows and "no row" for each of the eleven; the marks 68/245. **All sixteen answers were byte-equal to row 2480-SK's s2** (`evidence/s2-sha256.txt`).

**Step 3, the set-up.** Each write by the sheet's recipe, one label a command, each after a marks read that passed the before-write check (14 checks, each exit 0). The control's two imports and each pair's first imports were `CREATED` 1 with the other counts 0 and no LINEERROR (`IMPORT: CREATED ONE`):

| pair | names (a, b) | s4&lt;b&gt; answer | recipe | the pair's result |
| --- | --- | --- | --- | --- |
| k | `BRIDGE Sk Ak Am`, `BRIDGE Sk Ak, Am` | CREATED 0, ALTERED 1, the rest 0, no LINEERROR | (e): `sk.py --altered` ALTERED ONE; s4kbx 70/248, the before-write check passed; s7kb: `sc.py --altered-state` 20 LEDGER rows, `ALTERED STATE k: AS SK RECORDED` | **ALTERED, as SK recorded** |
| q | `BRIDGE Sk Ck Cm`, `BRIDGE Sk Ck' Cm` | CREATED 1 (`IMPORT: CREATED ONE`) | s7qb: `sk.py --pair qa qb` 22 LEDGER rows, both `EQUAL`, two GUIDs: BOTH; then s5qb CREATED ONE | **BOTH** |
| u | `BRIDGE Sk Dk Dm`, `BRIDGE Sk Dk_Dm` | CREATED 1 (`IMPORT: CREATED ONE`) | s7ub: `sk.py --pair ua ub` 24 LEDGER rows, both `EQUAL`, two GUIDs: BOTH; then s5ub CREATED ONE | **BOTH** |

On the alteration, `count20.py --same-row` of s4kbx against s4kbm (70/247) printed that the COMPANY row differs (ALTMSTID one higher, ALTVCHID the same), exit 1: the sheet makes that line a record, not a check. **No import was refused, so `rf.py` was never run.** Recipe (e) was used for `s4kb` alone. The invoices' `LASTVCHID` ran from 45 (pl) through 46 (ka), 47 (qa), 48 (qb) and 49 (ua) to 50 (ub), in the order sent; no invoice of `kb` was sent.

**Step 4, the book-wide reads.** s6m 74/252. The snapshot, sent twice: 32,928 B each, byte-equal, 24 LEDGER rows; a row for each of `pl`, `ka`, `qa`, `qb`, `ua` and `ub`, each `Sundry Debtors`, bill-wise Yes, opening 0.00, closing -1051.00; "no row" for `kb`. B-bills, sent twice: 2,798 B each, byte-equal, 6 groups, one for each of those six names, each `BP/26-27/<its number>`, 1-Aug-26, -1051.00, `ka`'s under `ka`'s name; none for `kb`. The catalogue, sent twice: 29,706 B each, byte-equal; `sk.py --names` `EQUAL` for `pl`, `qa`, `qb`, `ua` and `ub`, `NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])` and `NAMES kb: no row`; `sc.py --altered-state` `ALTERED STATE k: AS SK RECORDED`. `st.py --names`: the one ledger whose name list is not its `NAME` attribute is `ka`. The census, sent twice: 44,458 B each, byte-equal, `ANSWER: VOUCHERS 17` (s2's 11 and this row's 6). **INVOICED** `pl,ka,qa,qb,ua,ub`. Every expectation of step 4 held.

**Step 5.** m1's marks were equal to s6m's (74/252): step 4 only read. No m1r was sent.

**The set-up whole, before k0** (written in `evidence/notes.txt` before k0, each with the line that shows it): no stop at steps 1 to 5; every send answered; m1 equal to s6m; every before-write check passed; `pl` in its intended state. All five held.

**Step 6, the shape step, the control alone.** Each send was checked before the next, by `scripts/k.sh`:

| label | file | answer | check | gave |
| --- | --- | --- | --- | --- |
| k0 | `M-marks.xml` | 74/252 | `--same-row` against m1 | SAME MARKS |
| k1 | `Vm-u-pl.xml` | 11,946 B | `wx.py`; `sk.py --ref` | `ANSWER: VOUCHERS 1`; `REF DIFFER` (kept 11,954 B) |
| k2 | `Vm-v-pl.xml` | 11,946 B | `wx.py --same` against k1; `sk.py --ref` | `SAME: EQUAL`; `REF DIFFER` |
| k3 | `Vw-u-pl.xml` | 94,744 B | `wx.py`; `sk.py --ref` | `ANSWER: VOUCHERS 1`; `REF DIFFER` (kept 94,752 B) |
| k4 | `Vw-v-pl.xml` | 94,744 B | `wx.py --same` against k3; `sk.py --ref` | `SAME: EQUAL`; `REF DIFFER` |
| k5 | `L-lo-pl-20260801.xml` | 896 B | `nm.py`; `sk.py --ref` | `ANSWER: ROWS 1 (0 On Account)`; `REF EQUAL` |
| k6 | `C-u-pl.xml` | 5,374 B | `cv.py` | `ANSWER: VOUCHERS 1` |
| k7 | `C-v-pl.xml` | 5,374 B | `cv.py --same` against k6 | `SAME: EQUAL` |
| k8 | `Vb-book.xml` | 1,361,516 B | `wx.py`; `--match pl movement` (k1); `--match pl window` (k3); `cv.py --own pl` | `ANSWER: VOUCHERS 17`; `EQUAL`; `EQUAL`; `OWN: YES` |
| k9 | `M-marks.xml` | 74/252 | `--same-row` against k0 | SAME MARKS |

The `REF` lines are recorded and never required, as the sheet says; the four voucher answers were each 8 B shorter than the kept ones, as in SK. No k0r or k9r was sent. **The shape step passed.**

**Step 7, the pair names in both forms.** For each id, in the sheet's order, each send once, each answer read by its reader before the next request, then the id's lines:

| n | id | form | census: c a, c b (`--same`); `--own` | movement: q a, q b (`--same`); `MATCH` | window: w a, w b (`--same`); `MATCH` | form against form (census, movement, window) |
| --- | --- | --- | --- | --- | --- | --- |
| 6 | qa | `##LedgerName` | VOUCHERS 1 twice (EQUAL, 5,374 B); YES | VOUCHERS 1 twice (EQUAL, 11,946 B); EQUAL | VOUCHERS 1 twice (EQUAL, 94,744 B); EQUAL | |
| 6 | qa | master's CHILDOF | VOUCHERS 1 twice (EQUAL, 5,374 B); YES | VOUCHERS 1 twice (EQUAL, 11,946 B); EQUAL | VOUCHERS 1 twice (EQUAL, 94,744 B); EQUAL | EQUAL, EQUAL, EQUAL |
| 7 | qb | `##LedgerName` | VOUCHERS 1 twice (EQUAL, 5,374 B); YES | VOUCHERS 1 twice (EQUAL, 11,970 B); EQUAL | VOUCHERS 1 twice (EQUAL, 94,768 B); EQUAL | |
| 8 | ua | `##LedgerName` | VOUCHERS 1 twice (EQUAL, 5,374 B); YES | VOUCHERS 1 twice (EQUAL, 11,946 B); EQUAL | VOUCHERS 1 twice (EQUAL, 94,744 B); EQUAL | |
| 8 | ua | master's CHILDOF | VOUCHERS 1 twice (EQUAL, 5,374 B); YES | VOUCHERS 1 twice (EQUAL, 11,946 B); EQUAL | VOUCHERS 1 twice (EQUAL, 94,744 B); EQUAL | EQUAL, EQUAL, EQUAL |
| 9 | ub | `##LedgerName` | VOUCHERS 1 twice (EQUAL, 5,374 B); YES | VOUCHERS 1 twice (EQUAL, 11,946 B); EQUAL | VOUCHERS 1 twice (EQUAL, 94,744 B); EQUAL | |

**The judgement, one line a name a form** (`sk.py --judge`, each against k8, each exit 0):

| id | form | census | movement | window |
| --- | --- | --- | --- | --- |
| qa | `##LedgerName` | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |
| qa | master's CHILDOF | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |
| qb | `##LedgerName` | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |
| ua | `##LedgerName` | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |
| ua | master's CHILDOF | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |
| ub | `##LedgerName` | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |

Each `MATCH` line names that id's own invoice, `BP/26-27/<its number>` dated 20260801, matched by VOUCHERKEY, its line on the ledger Yes -1051.00, with 10 own fields equal (movement) or 14 own fields equal and the bill `BP/26-27/<its number>` New Ref -1051.00 (window). Each `--own` line found the census answer to hold that id's own invoice and nothing else. Then a10 (`Vb-book.xml`, the file k8 sent): `ANSWER: VOUCHERS 17`, `SAME: EQUAL` against k8 (1,361,516 B each), so every judgement was made against a book-wide read that did not change during step 7. a11's marks were equal to k9's (74/252): step 7 only read. No a11r was sent. **Part A passed in full**, as the notes' line before b1a says.

**SK's `##LedgerName` answers again.** Each answer of steps 6 and 7 that SK also sent under the same label and file had SK's size, and every reader's line was SK's. Only the outstandings answer (k5) was byte-equal to SK's. The others differed only in the company-information counts (`LEDGER`, `CURRENCY`, `TAXUNIT`, `VOUCHER`, `VOUCHERNUMBERSERIES`) and, for the pair names, in the invoice's own identifiers (`GUID`, `ALTERID`, `MASTERID`, `VOUCHERKEY`, `VOUCHERRETAINKEY`, the `VOUCHER` element's ids, and `BILLID` in the window read): this book held six of this row's ledgers and invoices, not eight, and the invoices were made in another order (`evidence/against-sk.txt`, written after the row, element names only).

**So, by the sheet's Decides, Part A:** for `qa` and `ua`, in each of master's three CHILDOF forms, `OWN YES, SIBLING NO, OTHERS 0`, the pair `SAME: EQUAL`, its own line `EQUAL` or `YES`, and the form-against-form line `EQUAL`: on this book and release, master's shipped CHILDOF census, movement and window reads told `BRIDGE Sk Ck Cm` from `BRIDGE Sk Ck' Cm` and `BRIDGE Sk Dk Dm` from `BRIDGE Sk Dk_Dm`, each pair sharing one letters-and-digits key, and answered byte for byte as the `##LedgerName` form did. For `qb` and `ub`, and for `qa` and `ua` in the `##LedgerName` form, SK's reads gave SK's results again, in another sitting.

**What Part A does not say.** Whether the build's own parse accepts these answers; the reads Bridge sends around them; any other book, release or day; a window of more than one day, another AlterID span or a book of real size; the CHILDOF form for a name with a mark (no such name is sent there); a comma or a space between two coexisting ledgers (the gateway made none in SK); three or more ledgers sharing one key; a ledger with more than one voucher or bill, or a settled bill; a book with GST on. The readers' empty-answer and `STATUS` paths met no real answer here.

### Part A's deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. Every script ran as `python -I` (Python 3.10), unchanged, from the row folder, through the helpers in `scripts/`.
- **The run lead's words for this row** (in the queue entry): (a) the empty-answer exception, covering the `C-u-`, `Vm-u-` and `Vw-u-` requests of steps 7 and 8 as the sheet asks; (b) the refused-import exception; (c) recipe (e)'s alteration rule, for `s4kb` alone. The sheet asks for (a) and (c) only and makes any refusal the end of the set-up; that stricter rule held, so `scripts/w.sh` carries (c) and never runs `rf.py`, and (b) was not used. `scripts/er.sh` carried (a), which was not needed.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **The three Bridge processes** that ran before the row were left running, as the sheet says. None was called by the row.
- **Sends run in the background, with a 300 s watch.** From step 2 on, each block of sends was started by `scripts/bg.sh`, which writes the block's output and exit to `evidence/run-<name>.out` and `.rc`, beside `scripts/wd.sh`, which watched the same block for 5 minutes and ended when it did. Each block ended inside the watch; the longest, step 7's second, took 110 s. s1 (one marks read) was sent in the foreground; m1, s9 and the two end-of-part blocks were short and run in the foreground through `bg.sh`.
- **Step 3 by pair.** The control's four labels ran as one command, `scripts/ctl.sh`; each pair's labels as one command, `scripts/pair.sh <k|q|u>`, which runs `scripts/w.sh` for each label in the recipe's order and ends at the pair's result or at the first answer the recipe does not name. For pair k it sends s7kb after the ALTERED ONE line and runs `sc.py --altered-state`; for q and u it sends s7&lt;b&gt; and runs `sk.py --pair` after `CREATED ONE`. Each pair's result was written to `evidence/pair-results.txt` before the next pair started. Step 4 was sent by `scripts/s6.sh`, which also runs `sc.py --altered-state` on s6ga (`evidence/altstate-s6ga.txt`).
- **Step 7 in two runs.** To keep each block inside the 300 s watch, step 7 ran as two blocks of `scripts/a.sh`: `qa` and `qb` (108 s), then `ua` and `ub` (110 s), each with its own watch. `a.sh` numbers each id by its place in the table, so the labels are the sheet's. Each send went by `scripts/er.sh`, which ends with exit 3 when the answer came with bytes (its answered path, on all 36 sends here) and carries exception (a) for an empty one; it captures the window after each send, so each `b` send has a capture as well as each `a` send. a10 and a11 were sent by `scripts/a10.sh`.
- **The Gateway images.** `evidence/gateway-refs.txt` carries the three Gateway images of row 2480-NX, each looked at there, as in MX, MW and SK. This row's s1 capture was byte-equal to the first; every capture after a send was that same image (`ca58f95e…`), as was s9's.
- **A note's file.** `evidence/bookcheck.txt` as step 0 wrote it held the book's folder name alone, not the check's printout; that file stays on the lab PC under another name, and `bookcheck.txt` was written again from step 0's printout, with a correction line in the notes. No other text changed.

## Part B: the altered ledger, by each spelling (step 8)

**Confidence: M, measured by this run, in one run, for each line below; PARTIAL as the sheet's Decides says (one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse).** Every send of step 8 was answered: each with curl exit 0 and HTTP 200, with bytes; no answer was empty, so exception (a) was never needed. No dialog showed. Every capture after a send was the Gateway image above.

**Before b1a.** `sc.py --judge-b ka movement a-k8-Vb-book.xml a-k8-Vb-book.xml`: `JUDGE-B ka movement: OWN YES, OTHERS 16`, exit 3: `ka`'s invoice found in k8 among its 17 vouchers, the line the sheet asks for. Part B was sent.

**The reads.** Each send once, in the sheet's order, each answer read by its reader before the next request, then the pair's `--same` and the `JUDGE-B` line (each against k8, each exit 0):

| labels | file | spelling | form | a, b (`wx.py --same`) | `JUDGE-B` |
| --- | --- | --- | --- | --- | --- |
| b1a, b1b | `L-lo-ka-20260801.xml` | `ka` | outstandings | ROWS 1 twice (EQUAL, 896 B) | OWN YES, OTHERS 0 |
| b2a, b2b | `C-v-ka.xml` | `ka` | census | VOUCHERS 1 twice (EQUAL, 5,374 B) | OWN YES, OTHERS 0 |
| b3a, b3b | `Vm-v-ka.xml` | `ka` | movement | VOUCHERS 1 twice (EQUAL, 11,946 B) | OWN YES, OTHERS 0 |
| b4a, b4b | `Vw-v-ka.xml` | `ka` | window | VOUCHERS 1 twice (EQUAL, 94,744 B) | OWN YES, OTHERS 0 |
| b5a, b5b | `C-u-ka.xml` | `ka`, master's CHILDOF | census | VOUCHERS 1 twice (EQUAL, 5,374 B) | OWN YES, OTHERS 0 |
| b6a, b6b | `Vm-u-ka.xml` | `ka`, master's CHILDOF | movement | VOUCHERS 1 twice (EQUAL, 11,946 B) | OWN YES, OTHERS 0 |
| b7a, b7b | `Vw-u-ka.xml` | `ka`, master's CHILDOF | window | VOUCHERS 1 twice (EQUAL, 94,744 B) | OWN YES, OTHERS 0 |
| b8a, b8b | `L-lo-kb-20260801.xml` | `kb` | outstandings | ROWS 1 twice (EQUAL, 896 B) | OWN YES, OTHERS 0 |
| b9a, b9b | `C-v-kb.xml` | `kb` | census | VOUCHERS 1 twice (EQUAL, 5,374 B) | OWN YES, OTHERS 0 |
| b10a, b10b | `Vm-v-kb.xml` | `kb` | movement | VOUCHERS 1 twice (EQUAL, 11,946 B) | OWN YES, OTHERS 0 |
| b11a, b11b | `Vw-v-kb.xml` | `kb` | window | VOUCHERS 1 twice (EQUAL, 94,744 B) | OWN YES, OTHERS 0 |

**The spellings against each other** (`wx.py --same` on the `a` answers): b1a and b8a, b2a and b9a, b3a and b10a, b4a and b11a, each `SAME: EQUAL`: in each of the four `##LedgerName` forms, the answer by `BRIDGE Sk Ak, Am` was byte-equal to the answer by `BRIDGE Sk Ak Am`. **The forms against each other for `ka`:** b2a and b5a, b3a and b6a, b4a and b7a, each `SAME: EQUAL`: master's CHILDOF form answered byte for byte as the `##LedgerName` form did. Then b12 (`Vb-book.xml`): `ANSWER: VOUCHERS 17`, `SAME: EQUAL` against k8 (1,361,516 B each): the book-wide read did not change during step 8. b13's marks were equal to a11's (74/252): step 8 only read. No b13r was sent.

**So, by the sheet's Decides, Part B:** each read, by each spelling, in each form sent, gave `OWN YES, OTHERS 0`: on this book and release, the ledger altered through a `Create`, whose `NAME` attribute is `BRIDGE Sk Ak Am` and whose name list holds only `BRIDGE Sk Ak, Am`, was reached by both names, in the outstandings, census, movement and window `##LedgerName` forms, and by its `NAME` attribute in master's three CHILDOF forms (`kb` is not sent in CHILDOF, as master refuses it). The two spellings' answers were the same bytes.

**What Part B does not say.** Which of the two names Tally matched on, or whether it matched on both; a voucher posted after the alteration naming either spelling; `kb` in master's CHILDOF form; an alteration made on Tally's own screens; pairs `r` and `e`; the reads Bridge sends around these; the build's own parse; any other book, release or day.

### Part B's deviations from the sheet

- **Step 8 in two runs, then the cross lines.** `scripts/b.sh` ran rows 1 to 6 (68 s) and rows 7 to 11 (60 s), each beside its own 300 s watch, then `b.sh cross` for the seven spelling-against-spelling and form-against-form lines, then `scripts/a10.sh b12 b13 a11`. `b.sh pre` ran the check before b1a. Each send went by `scripts/er.sh` (22 sends, each answered with bytes).
- The deviations of Part A that are not about its own steps (`python`, the run lead's words, the background runs, the Gateway images) apply here too.

## The way back

**Step 9.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`, 16 of 16), each N read 0 rows, no group and no row for any of the eleven names, the catalogue "no row" for each, `FOLD: CLEAR`, and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1.

**The side files.** Before Tally started, four of the seven files that the restores of rows 2480-NX to 2480-SK left out were in the book's data folder (`CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`; 20 entries). At load, before step 1's reads, six were (23 entries): `TCMPSCHMESSAGE.TSF` and `TINTMSG.TSF` were back; `Index.1800` was absent. Before the restore the folder held 24 entries; this row's restore left out the same seven (17 entries after), and at the load after it the same six were back and `Index.1800` absent (23 entries), as in those rows. The listings stay on the lab PC.

**Step 10.** C10, then Tally quit by its own Quit with no prompt (the steps below).

**Timings.** Every read sent without a window capture after it (the marks reads before each write) returned within 1 s of its send line (0.402 to 0.815 s), wall clock, python's start included. The labels sent with a capture took 2.944 to 10.640 s, the capture included: the imports 3.350 to 10.640 s (`s5ka`, pair k's invoice, the longest; the next, `s5pl`, 6.450 s), the three catalogue reads of step 3 3.054 to 5.662 s (`s7kb` the longest), s2 and s9 (sixteen reads each) 6.407 and 5.288 s, the step 4 reads 3.259 to 3.517 s, k0 to k9 3.100 to 4.527 s, Part A's sends 3.266 to 4.066 s, Part B's 3.079 to 3.971 s (`evidence/durations.txt`).

## Steps
- Step 0 (05:13:24Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. No `tally.exe` ran.
- Tally started from its desktop shortcut (05:13:25Z). The start-up company was shut (Part A's deviations). `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`, 05:14:16Z, byte-equal to row 2480-SK's).
- Gateway (05:14:37Z): the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026. The data folder listed (the side files above). Step 0 C2 (05:14:40Z): one `tally.exe`, which alone owns 9001; GET /status status=200, bytes=51. s1: the book's GUID, ALTVCHID 68, ALTMSTID 245.
- BK by C5 (capture before Accept 05:16:09Z): Data, Backup & Restore, Backup; Backspace to the path, Specify Path, `C:\lab\2480-sc\backup-1` (this time the field read the typed path alone, so nothing was mended); the book the one company; End of List; Accept y; back at the Gateway with no message (05:16:29Z). One file of 90,051 B, at 10:46 IST, 11 Nov (`evidence/bk.txt`).
- Step 2 (05:16:56Z to 05:17:10Z), step 3 (05:17:38Z to 05:20:47Z, the control and three pairs), step 4 (05:20:59Z to 05:21:44Z), step 5 (05:22:15Z), step 6 (05:22:21Z to 05:23:19Z), step 7 (05:23:33Z to 05:27:52Z) and step 8 (05:28:01Z to 05:30:55Z) as above, each in the notes.
- Step 9 C6 (prep 05:30:56Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter, no prompt (the first Enter was not sent, as the keyboard focus was not on Tally; a click on its title bar came first); Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; the list opened at the drive's root; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (10:46 am), as the plan; the book selected alone; End of List; Accept ? y (05:32:28Z); the folder-exists question with the book's number (05:32:43Z), r Replace; back at Select Company with no other prompt. The data folder listed again (the side files above). The book loaded alone by the name filter (`s9-select`, 05:33:16Z, showed the one row; not byte-equal to `s1-select`, 36,401 B against 36,413 B, and byte-equal to row 2480-SK's `s9-select`, the caret's blink as there); Gateway captured (05:33:39Z), byte-equal to s1's, date and period unchanged; then s9 (05:33:41Z to 05:33:53Z), equal to s2 throughout.
- Step 10, C10 (05:34:36Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded (the capture looked at: Select Company with none loaded; not byte-equal to row 2480-SK's C10 capture); one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; then no `tally.exe` ran and nothing listened on 9001. The BK, listed again, one file, its sha256 equal to `evidence/bk.txt`.

## Files
- `answers/`: every answer and its headers file, whole, byte for byte (`kept-local.txt` lists any that stay on the lab PC).
- `logs/`: `run.py`'s log of each label.
- `evidence/`: notes (UTC, one line per step); every `count20.py`, `nm.py`, `st.py`, `cv.py`, `wx.py`, `sk.py` and `sc.py` output, whole, with its exit; each `er.sh` run's output; each send's UTC time and request check; each background block's output and exit; each pair's results; the Gateway images' list; the per-label wall clock; the book check; `tally.exe`'s file data; the comparison with SK's answers. Process, port, folder and backup listings and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers. `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `wx.py`, `cv.py`, `sk.py` and `sc.py` are the sheet's files, byte for byte, and are not repeated.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.
- `substitutions.txt`: where the lab's own text differs from the files kept on the lab PC. In the lab's own text each 16-hex sha256 head that a reader printed and that holds a run of 10 or more digits is replaced by the whole sha256 of the answer or REF value that head heads, on the run lead's word of row 2480-MW.

## The outputs, whole

### s1
```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s2 (count20.py)
```
== a-s2-N-0281.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0292.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0293.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0294.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0295.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0296.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0297.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0298.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0299.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0300.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0301.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s2 (nm.py)
```
== a-s2-B-bills.xml
  BILLFIXED groups: 0
  pl 'BRIDGE Mx Plain': 0 groups
  ka 'BRIDGE Sk Ak Am': 0 groups
  kb 'BRIDGE Sk Ak, Am': 0 groups
  ra 'BRIDGE Sk Bk, Bm': 0 groups
  rb 'BRIDGE Sk Bk Bm': 0 groups
  qa 'BRIDGE Sk Ck Cm': 0 groups
  qb "BRIDGE Sk Ck' Cm": 0 groups
  ua 'BRIDGE Sk Dk Dm': 0 groups
  ub 'BRIDGE Sk Dk_Dm': 0 groups
  ea 'BRIDGE Sk EkEm': 0 groups
  eb 'BRIDGE Sk Ek Em': 0 groups
  groups of any other party: 0
== a-s2-S-snap-20260401-20260801.xml
a-s2-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Mx Plain': no row
  ka 'BRIDGE Sk Ak Am': no row
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': no row
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': no row
  qb "BRIDGE Sk Ck' Cm": no row
  ua 'BRIDGE Sk Dk Dm': no row
  ub 'BRIDGE Sk Dk_Dm': no row
  ea 'BRIDGE Sk EkEm': no row
  eb 'BRIDGE Sk Ek Em': no row
exit 0
```

### s2 st.py, the before option
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
FOLD: CLEAR
exit 0
```

### s2 st.py --names
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
  pl 'BRIDGE Mx Plain': no row
  ka 'BRIDGE Sk Ak Am': no row
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': no row
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': no row
  qb "BRIDGE Sk Ck' Cm": no row
  ua 'BRIDGE Sk Dk Dm': no row
  ub 'BRIDGE Sk Dk_Dm': no row
  ea 'BRIDGE Sk EkEm': no row
  eb 'BRIDGE Sk Ek Em': no row
NAMES: EQUAL
exit 0
```

### s2 cv.py (C-book.xml)
```
a-s2-C-book.xml: ANSWER: VOUCHERS 11
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001 ALTERID 2 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002 ALTERID 11 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003 ALTERID 50 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004 ALTERID 13 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005 ALTERID 54 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006 ALTERID 48 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007 ALTERID 8 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008 ALTERID 9 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009 ALTERID 10 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a ALTERID 16 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b ALTERID 17 DATE 20260801
exit 0
```

### s2 sk.py --names (G-cat.xml)
```
a-s2-G-cat.xml: LEDGER rows 18
  NAMES pl: no row
  NAMES ka: no row
  NAMES kb: no row
  NAMES ra: no row
  NAMES rb: no row
  NAMES qa: no row
  NAMES qb: no row
  NAMES ua: no row
  NAMES ub: no row
  NAMES ea: no row
  NAMES eb: no row
exit 0
```

### s4plm count20.py (the before-write option)
```
== a-s4plm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4pl st.py --import
```
a-s4pl-LM-pl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5plm count20.py (the before-write option)
```
== a-s5plm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5pl st.py --import
```
a-s5pl-I-pl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 45, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4kam count20.py (the before-write option)
```
== a-s4kam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ka st.py --import
```
a-s4ka-LM-ka.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5kam count20.py (the before-write option)
```
== a-s5kam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ka st.py --import
```
a-s5ka-I-ka.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 46, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4kbm count20.py (the before-write option)
```
== a-s4kbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4kb st.py --import
```
a-s4kb-LM-kb.xml: CREATED 0, ALTERED 1, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: stop (CREATED is not 1; ALTERED is not 0)
exit 1
```

### s4kb sk.py --altered
```
a-s4kb-LM-kb.xml: CREATED 0, ALTERED 1, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
ALTERED ONE
exit 0
```

### s4kbx count20.py (the before-write option)
```
== a-s4kbx-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4kbx count20.py --same-row against s4kbm, a record
```
== a-s4kbx-M-marks.xml against a-s4kbm-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'")
stop: the COMPANY row differs
exit 1
```

### s7kb sc.py --altered-state (G-cat.xml)
```
a-s7kb-G-cat.xml: LEDGER rows 20
ALTERED STATE k: AS SK RECORDED
exit 0
```

### s7kb sk.py --names
```
a-s7kb-G-cat.xml: LEDGER rows 20
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: no row
  NAMES rb: no row
  NAMES qa: no row
  NAMES qb: no row
  NAMES ua: no row
  NAMES ub: no row
  NAMES ea: no row
  NAMES eb: no row
exit 0
```

### s4qam count20.py (the before-write option)
```
== a-s4qam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4qa st.py --import
```
a-s4qa-LM-qa.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5qam count20.py (the before-write option)
```
== a-s5qam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5qa st.py --import
```
a-s5qa-I-qa.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 47, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4qbm count20.py (the before-write option)
```
== a-s4qbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4qb st.py --import
```
a-s4qb-LM-qb.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s7qb sk.py --pair qa qb (G-cat.xml)
```
a-s7qb-G-cat.xml: LEDGER rows 22
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
PAIR qa qb: BOTH
exit 0
```

### s7qb sk.py --names
```
a-s7qb-G-cat.xml: LEDGER rows 22
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: no row
  NAMES rb: no row
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
  NAMES ua: no row
  NAMES ub: no row
  NAMES ea: no row
  NAMES eb: no row
exit 0
```

### s5qbm count20.py (the before-write option)
```
== a-s5qbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5qb st.py --import
```
a-s5qb-I-qb.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4uam count20.py (the before-write option)
```
== a-s4uam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ua st.py --import
```
a-s4ua-LM-ua.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5uam count20.py (the before-write option)
```
== a-s5uam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ua st.py --import
```
a-s5ua-I-ua.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 49, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4ubm count20.py (the before-write option)
```
== a-s4ubm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ub st.py --import
```
a-s4ub-LM-ub.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s7ub sk.py --pair ua ub (G-cat.xml)
```
a-s7ub-G-cat.xml: LEDGER rows 24
  NAMES ua: EQUAL (one row, its own GUID)
  NAMES ub: EQUAL (one row, its own GUID)
PAIR ua ub: BOTH
exit 0
```

### s7ub sk.py --names
```
a-s7ub-G-cat.xml: LEDGER rows 24
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: no row
  NAMES rb: no row
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
  NAMES ua: EQUAL (one row, its own GUID)
  NAMES ub: EQUAL (one row, its own GUID)
  NAMES ea: no row
  NAMES eb: no row
exit 0
```

### s5ubm count20.py (the before-write option)
```
== a-s5ubm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ub st.py --import
```
a-s5ub-I-ub.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 50, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### the pairs' results
```
2026-11-11T05:19:04Z pair k (ka, kb): ALTERED, AS SK RECORDED
2026-11-11T05:20:05Z pair q (qa, qb): BOTH
2026-11-11T05:20:47Z pair u (ua, ub): BOTH
```

### s6m
```
== a-s6m-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s6sa nm.py
```
== a-s6sa-S-snap-20260401-20260801.xml
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 24
  pl 'BRIDGE Mx Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ka 'BRIDGE Sk Ak Am': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': no row
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  qb "BRIDGE Sk Ck' Cm": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ua 'BRIDGE Sk Dk Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ub 'BRIDGE Sk Dk_Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ea 'BRIDGE Sk EkEm': no row
  eb 'BRIDGE Sk Ek Em': no row
exit 0
```

### s6sb nm.py
```
== a-s6sb-S-snap-20260401-20260801.xml
a-s6sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 24
  pl 'BRIDGE Mx Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ka 'BRIDGE Sk Ak Am': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': no row
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  qb "BRIDGE Sk Ck' Cm": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ua 'BRIDGE Sk Dk Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ub 'BRIDGE Sk Dk_Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ea 'BRIDGE Sk EkEm': no row
  eb 'BRIDGE Sk Ek Em': no row
exit 0
```

### s6sa/s6sb nm.py --same
```
  a-s6sa-S-snap-20260401-20260801.xml: 32928 bytes, sha256 30198747e1e9b92a0f0fda8b864ff20760db96561f9ed0a639fcd6263f8b5cc5
  a-s6sb-S-snap-20260401-20260801.xml: 32928 bytes, sha256 30198747e1e9b92a0f0fda8b864ff20760db96561f9ed0a639fcd6263f8b5cc5
SAME: EQUAL
exit 0
```

### s6ba nm.py
```
== a-s6ba-B-bills.xml
  BILLFIXED groups: 6
  pl 'BRIDGE Mx Plain': 1 groups
    BILLREF 'BP/26-27/0281'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ka 'BRIDGE Sk Ak Am': 1 groups
    BILLREF 'BP/26-27/0292'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  kb 'BRIDGE Sk Ak, Am': 0 groups
  ra 'BRIDGE Sk Bk, Bm': 0 groups
  rb 'BRIDGE Sk Bk Bm': 0 groups
  qa 'BRIDGE Sk Ck Cm': 1 groups
    BILLREF 'BP/26-27/0296'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  qb "BRIDGE Sk Ck' Cm": 1 groups
    BILLREF 'BP/26-27/0297'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ua 'BRIDGE Sk Dk Dm': 1 groups
    BILLREF 'BP/26-27/0298'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ub 'BRIDGE Sk Dk_Dm': 1 groups
    BILLREF 'BP/26-27/0299'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ea 'BRIDGE Sk EkEm': 0 groups
  eb 'BRIDGE Sk Ek Em': 0 groups
  groups of any other party: 0
exit 0
```

### s6bb nm.py
```
== a-s6bb-B-bills.xml
  BILLFIXED groups: 6
  pl 'BRIDGE Mx Plain': 1 groups
    BILLREF 'BP/26-27/0281'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ka 'BRIDGE Sk Ak Am': 1 groups
    BILLREF 'BP/26-27/0292'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  kb 'BRIDGE Sk Ak, Am': 0 groups
  ra 'BRIDGE Sk Bk, Bm': 0 groups
  rb 'BRIDGE Sk Bk Bm': 0 groups
  qa 'BRIDGE Sk Ck Cm': 1 groups
    BILLREF 'BP/26-27/0296'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  qb "BRIDGE Sk Ck' Cm": 1 groups
    BILLREF 'BP/26-27/0297'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ua 'BRIDGE Sk Dk Dm': 1 groups
    BILLREF 'BP/26-27/0298'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ub 'BRIDGE Sk Dk_Dm': 1 groups
    BILLREF 'BP/26-27/0299'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ea 'BRIDGE Sk EkEm': 0 groups
  eb 'BRIDGE Sk Ek Em': 0 groups
  groups of any other party: 0
exit 0
```

### s6ba/s6bb nm.py --same
```
  a-s6ba-B-bills.xml: 2798 bytes, sha256 db384bc41d5cbdde20f5d21bc6e5bc5dbd66d26e18ebb516153dd333e3fe6745
  a-s6bb-B-bills.xml: 2798 bytes, sha256 db384bc41d5cbdde20f5d21bc6e5bc5dbd66d26e18ebb516153dd333e3fe6745
SAME: EQUAL
exit 0
```

### s6ga sk.py --names
```
a-s6ga-G-cat.xml: LEDGER rows 24
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: no row
  NAMES rb: no row
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
  NAMES ua: EQUAL (one row, its own GUID)
  NAMES ub: EQUAL (one row, its own GUID)
  NAMES ea: no row
  NAMES eb: no row
exit 0
```

### s6gb sk.py --names
```
a-s6gb-G-cat.xml: LEDGER rows 24
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: no row
  NAMES rb: no row
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
  NAMES ua: EQUAL (one row, its own GUID)
  NAMES ub: EQUAL (one row, its own GUID)
  NAMES ea: no row
  NAMES eb: no row
exit 0
```

### s6ga/s6gb --same
```
  a-s6ga-G-cat.xml: 29706 bytes, sha256 797def7057568d629821a33cc69f3a0cb79a5ac9057b2ceac60655f3c64a69a1
  a-s6gb-G-cat.xml: 29706 bytes, sha256 797def7057568d629821a33cc69f3a0cb79a5ac9057b2ceac60655f3c64a69a1
SAME: EQUAL
exit 0
```

### s6ga sc.py --altered-state
```
a-s6ga-G-cat.xml: LEDGER rows 24
ALTERED STATE k: AS SK RECORDED
exit 0
```

### s6ka cv.py
```
a-s6ka-C-book.xml: ANSWER: VOUCHERS 17
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001 ALTERID 2 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002 ALTERID 11 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003 ALTERID 50 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004 ALTERID 13 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005 ALTERID 54 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006 ALTERID 48 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007 ALTERID 8 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008 ALTERID 9 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009 ALTERID 10 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a ALTERID 16 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b ALTERID 17 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
exit 0
```

### s6kb cv.py
```
a-s6kb-C-book.xml: ANSWER: VOUCHERS 17
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001 ALTERID 2 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002 ALTERID 11 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003 ALTERID 50 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004 ALTERID 13 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005 ALTERID 54 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006 ALTERID 48 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007 ALTERID 8 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008 ALTERID 9 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009 ALTERID 10 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a ALTERID 16 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b ALTERID 17 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
exit 0
```

### s6ka/s6kb cv.py --same
```
  a-s6ka-C-book.xml: 44458 bytes, sha256 a29f9e255571363cdce35f6346e909d9f28f13d326abb9020213e9525707d43c
  a-s6kb-C-book.xml: 44458 bytes, sha256 a29f9e255571363cdce35f6346e909d9f28f13d326abb9020213e9525707d43c
SAME: EQUAL
exit 0
```

### s6 st.py --names
```
a-s6sa-S-snap-20260401-20260801.xml: LEDGER rows 24
  name list not the NAME attribute: NAME 'BRIDGE Sk Ak Am', names ['BRIDGE Sk Ak, Am']
  pl 'BRIDGE Mx Plain': names ['BRIDGE Mx Plain']
  ka 'BRIDGE Sk Ak Am': names ['BRIDGE Sk Ak, Am']
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': no row
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': names ['BRIDGE Sk Ck Cm']
  qb "BRIDGE Sk Ck' Cm": names ["BRIDGE Sk Ck' Cm"]
  ua 'BRIDGE Sk Dk Dm': names ['BRIDGE Sk Dk Dm']
  ub 'BRIDGE Sk Dk_Dm': names ['BRIDGE Sk Dk_Dm']
  ea 'BRIDGE Sk EkEm': no row
  eb 'BRIDGE Sk Ek Em': no row
NAMES: DIFFER
exit 0
```

### m1
```
== a-m1-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### m1 count20.py --same-row against s6m
```
== a-m1-M-marks.xml against a-s6m-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'")
  equal
exit 0
```

### k0 count20.py --same-row against m1
```
== a-k0-M-marks.xml against a-m1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'")
  equal
exit 0
```

### k1 wx.py (Vm-u-pl.xml)
```
a-k1-Vm-u-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k1 sk.py --ref
```
REF Vm-u-pl.xml: DIFFER (11946 B sha256 6bfed90dc3924f58; kept 11954 B sha256 b8eab7b26d95eccb)
exit 3
```

### k2 wx.py --same against k1
```
SAME: EQUAL (11946 B sha256 6bfed90dc3924f58; 11946 B sha256 6bfed90dc3924f58)
exit 0
```

### k2 sk.py --ref
```
REF Vm-v-pl.xml: DIFFER (11946 B sha256 6bfed90dc3924f58; kept 11954 B sha256 b8eab7b26d95eccb)
exit 3
```

### k3 wx.py (Vw-u-pl.xml)
```
a-k3-Vw-u-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k3 sk.py --ref
```
REF Vw-u-pl.xml: DIFFER (94744 B sha256 991626a8de845fb9; kept 94752 B sha256 f158857ba662a10f)
exit 3
```

### k4 wx.py --same against k3
```
SAME: EQUAL (94744 B sha256 991626a8de845fb9; 94744 B sha256 991626a8de845fb9)
exit 0
```

### k4 sk.py --ref
```
REF Vw-v-pl.xml: DIFFER (94744 B sha256 991626a8de845fb9; kept 94752 B sha256 f158857ba662a10f)
exit 3
```

### k5 nm.py (L-lo-pl-20260801.xml)
```
== a-k5-L-lo-pl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0281' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### k5 sk.py --ref
```
REF L-lo-pl-20260801.xml: EQUAL (896 B sha256 19c796774ac740b6; kept 896 B sha256 19c796774ac740b6)
exit 0
```

### k6 cv.py (C-u-pl.xml)
```
a-k6-C-u-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
exit 0
```

### k7 cv.py --same against k6
```
SAME: EQUAL (5374 B sha256 38951574e67e6d11; 5374 B sha256 38951574e67e6d11)
exit 0
```

### k8 wx.py (Vb-book.xml)
```
a-k8-Vb-book.xml: ANSWER: VOUCHERS 17
  BP/26-27/0001 DATE 20260801 ALTERID 2 entries 4
  BP/26-27/0002 DATE 20260801 ALTERID 11 entries 4
  BP/26-27/0003 DATE 20260801 ALTERID 50 entries 4
  BP/26-27/0004 DATE 20260801 ALTERID 13 entries 5
  BP/26-27/0005 DATE 20260801 ALTERID 54 entries 5
  BP/26-27/0006 DATE 20260801 ALTERID 48 entries 4
  BP/26-27/0007 DATE 20260801 ALTERID 8 entries 5
  BP/26-27/0008 DATE 20260801 ALTERID 9 entries 5
  BP/26-27/0009 DATE 20260801 ALTERID 10 entries 7
  BP/26-27/0011 DATE 20260801 ALTERID 16 entries 4
  BP/26-27/0010 DATE 20260801 ALTERID 17 entries 6
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### k8 wx.py --match pl movement (k1)
```
MATCH pl movement: EQUAL (1 voucher, BP/26-27/0281 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### k8 wx.py --match pl window (k3)
```
MATCH pl window: EQUAL (1 voucher, BP/26-27/0281 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0281 New Ref -1051.00; 14 own fields equal)
exit 0
```

### k8 cv.py --own pl
```
a-k6-C-u-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
pl's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801 (new vouchers 6)
  observation: its GUID is not I-pl.xml's REMOTEID
OWN: YES
exit 0
```

### k9 count20.py --same-row against k0
```
== a-k9-M-marks.xml against a-k0-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'")
  equal
exit 0
```

### k2 wx.py, for the record
```
a-k2-Vm-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k4 wx.py, for the record
```
a-k4-Vw-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### k7 cv.py, for the record
```
a-k7-C-v-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
exit 0
```

### c6a (qa, ##LedgerName) cv.py
```
a-c6a-C-v-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
exit 0
```

### c6b cv.py
```
a-c6b-C-v-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
exit 0
```

### c6 cv.py --same
```
SAME: EQUAL (5374 B sha256 856db9384934017bcb660fa7c4a8e49621e242db801e63ad0375c7aae56386df; 5374 B sha256 856db9384934017bcb660fa7c4a8e49621e242db801e63ad0375c7aae56386df)
exit 0
```

### q6a (qa, ##LedgerName) wx.py
```
a-q6a-Vm-v-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### q6b wx.py
```
a-q6b-Vm-v-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### q6 wx.py --same
```
SAME: EQUAL (11946 B sha256 86a8fba249d28d2f; 11946 B sha256 86a8fba249d28d2f)
exit 0
```

### w6a (qa, ##LedgerName) wx.py
```
a-w6a-Vw-v-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### w6b wx.py
```
a-w6b-Vw-v-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### w6 wx.py --same
```
SAME: EQUAL (94744 B sha256 0e44a2d9c9793c04; 94744 B sha256 0e44a2d9c9793c04)
exit 0
```

### qa ##LedgerName cv.py --own (c6a)
```
a-c6a-C-v-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
qa's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801 (new vouchers 6)
  observation: its GUID is not I-qa.xml's REMOTEID
OWN: YES
exit 0
```

### qa ##LedgerName wx.py --match movement (q6a against k8)
```
MATCH qa movement: EQUAL (1 voucher, BP/26-27/0296 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### qa ##LedgerName wx.py --match window (w6a against k8)
```
MATCH qa window: EQUAL (1 voucher, BP/26-27/0296 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0296 New Ref -1051.00; 14 own fields equal)
exit 0
```

### qa ##LedgerName sk.py --judge census (c6a)
```
JUDGE qa census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qa ##LedgerName sk.py --judge movement (q6a)
```
JUDGE qa movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qa ##LedgerName sk.py --judge window (w6a)
```
JUDGE qa window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### cu6a (qa, CHILDOF) cv.py
```
a-cu6a-C-u-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
exit 0
```

### cu6b cv.py
```
a-cu6b-C-u-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
exit 0
```

### cu6 cv.py --same
```
SAME: EQUAL (5374 B sha256 856db9384934017bcb660fa7c4a8e49621e242db801e63ad0375c7aae56386df; 5374 B sha256 856db9384934017bcb660fa7c4a8e49621e242db801e63ad0375c7aae56386df)
exit 0
```

### qu6a (qa, CHILDOF) wx.py
```
a-qu6a-Vm-u-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### qu6b wx.py
```
a-qu6b-Vm-u-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### qu6 wx.py --same
```
SAME: EQUAL (11946 B sha256 86a8fba249d28d2f; 11946 B sha256 86a8fba249d28d2f)
exit 0
```

### wu6a (qa, CHILDOF) wx.py
```
a-wu6a-Vw-u-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### wu6b wx.py
```
a-wu6b-Vw-u-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
exit 0
```

### wu6 wx.py --same
```
SAME: EQUAL (94744 B sha256 0e44a2d9c9793c04; 94744 B sha256 0e44a2d9c9793c04)
exit 0
```

### qa CHILDOF cv.py --own (cu6a)
```
a-cu6a-C-u-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
qa's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801 (new vouchers 6)
  observation: its GUID is not I-qa.xml's REMOTEID
OWN: YES
exit 0
```

### qa CHILDOF wx.py --match movement (qu6a against k8)
```
MATCH qa movement: EQUAL (1 voucher, BP/26-27/0296 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### qa CHILDOF wx.py --match window (wu6a against k8)
```
MATCH qa window: EQUAL (1 voucher, BP/26-27/0296 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0296 New Ref -1051.00; 14 own fields equal)
exit 0
```

### qa CHILDOF sk.py --judge census (cu6a)
```
JUDGE qa census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qa CHILDOF sk.py --judge movement (qu6a)
```
JUDGE qa movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qa CHILDOF sk.py --judge window (wu6a)
```
JUDGE qa window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qa the form against the form, census (c6a, cu6a) cv.py --same
```
SAME: EQUAL (5374 B sha256 856db9384934017bcb660fa7c4a8e49621e242db801e63ad0375c7aae56386df; 5374 B sha256 856db9384934017bcb660fa7c4a8e49621e242db801e63ad0375c7aae56386df)
exit 0
```

### qa the form against the form, movement (q6a, qu6a) wx.py --same
```
SAME: EQUAL (11946 B sha256 86a8fba249d28d2f; 11946 B sha256 86a8fba249d28d2f)
exit 0
```

### qa the form against the form, window (w6a, wu6a) wx.py --same
```
SAME: EQUAL (94744 B sha256 0e44a2d9c9793c04; 94744 B sha256 0e44a2d9c9793c04)
exit 0
```

### c7a (qb, ##LedgerName) cv.py
```
a-c7a-C-v-qb.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
exit 0
```

### c7b cv.py
```
a-c7b-C-v-qb.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
exit 0
```

### c7 cv.py --same
```
SAME: EQUAL (5374 B sha256 8bbd92c8d9dc1d56; 5374 B sha256 8bbd92c8d9dc1d56)
exit 0
```

### q7a (qb, ##LedgerName) wx.py
```
a-q7a-Vm-v-qb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### q7b wx.py
```
a-q7b-Vm-v-qb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### q7 wx.py --same
```
SAME: EQUAL (11970 B sha256 dbbbe17aa5d1b72e; 11970 B sha256 dbbbe17aa5d1b72e)
exit 0
```

### w7a (qb, ##LedgerName) wx.py
```
a-w7a-Vw-v-qb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### w7b wx.py
```
a-w7b-Vw-v-qb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### w7 wx.py --same
```
SAME: EQUAL (94768 B sha256 9b326b88e6468162; 94768 B sha256 9b326b88e6468162)
exit 0
```

### qb ##LedgerName cv.py --own (c7a)
```
a-c7a-C-v-qb.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
qb's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801 (new vouchers 6)
  observation: its GUID is not I-qb.xml's REMOTEID
OWN: YES
exit 0
```

### qb ##LedgerName wx.py --match movement (q7a against k8)
```
MATCH qb movement: EQUAL (1 voucher, BP/26-27/0297 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### qb ##LedgerName wx.py --match window (w7a against k8)
```
MATCH qb window: EQUAL (1 voucher, BP/26-27/0297 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0297 New Ref -1051.00; 14 own fields equal)
exit 0
```

### qb ##LedgerName sk.py --judge census (c7a)
```
JUDGE qb census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qb ##LedgerName sk.py --judge movement (q7a)
```
JUDGE qb movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qb ##LedgerName sk.py --judge window (w7a)
```
JUDGE qb window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### c8a (ua, ##LedgerName) cv.py
```
a-c8a-C-v-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
exit 0
```

### c8b cv.py
```
a-c8b-C-v-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
exit 0
```

### c8 cv.py --same
```
SAME: EQUAL (5374 B sha256 ad7b25b05a99c3ee; 5374 B sha256 ad7b25b05a99c3ee)
exit 0
```

### q8a (ua, ##LedgerName) wx.py
```
a-q8a-Vm-v-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### q8b wx.py
```
a-q8b-Vm-v-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### q8 wx.py --same
```
SAME: EQUAL (11946 B sha256 cb7a5b06cbb0e9b3; 11946 B sha256 cb7a5b06cbb0e9b3)
exit 0
```

### w8a (ua, ##LedgerName) wx.py
```
a-w8a-Vw-v-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### w8b wx.py
```
a-w8b-Vw-v-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### w8 wx.py --same
```
SAME: EQUAL (94744 B sha256 6204cf5e8cca4b64; 94744 B sha256 6204cf5e8cca4b64)
exit 0
```

### ua ##LedgerName cv.py --own (c8a)
```
a-c8a-C-v-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
ua's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801 (new vouchers 6)
  observation: its GUID is not I-ua.xml's REMOTEID
OWN: YES
exit 0
```

### ua ##LedgerName wx.py --match movement (q8a against k8)
```
MATCH ua movement: EQUAL (1 voucher, BP/26-27/0298 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ua ##LedgerName wx.py --match window (w8a against k8)
```
MATCH ua window: EQUAL (1 voucher, BP/26-27/0298 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0298 New Ref -1051.00; 14 own fields equal)
exit 0
```

### ua ##LedgerName sk.py --judge census (c8a)
```
JUDGE ua census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ua ##LedgerName sk.py --judge movement (q8a)
```
JUDGE ua movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ua ##LedgerName sk.py --judge window (w8a)
```
JUDGE ua window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### cu8a (ua, CHILDOF) cv.py
```
a-cu8a-C-u-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
exit 0
```

### cu8b cv.py
```
a-cu8b-C-u-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
exit 0
```

### cu8 cv.py --same
```
SAME: EQUAL (5374 B sha256 ad7b25b05a99c3ee; 5374 B sha256 ad7b25b05a99c3ee)
exit 0
```

### qu8a (ua, CHILDOF) wx.py
```
a-qu8a-Vm-u-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### qu8b wx.py
```
a-qu8b-Vm-u-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### qu8 wx.py --same
```
SAME: EQUAL (11946 B sha256 cb7a5b06cbb0e9b3; 11946 B sha256 cb7a5b06cbb0e9b3)
exit 0
```

### wu8a (ua, CHILDOF) wx.py
```
a-wu8a-Vw-u-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### wu8b wx.py
```
a-wu8b-Vw-u-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### wu8 wx.py --same
```
SAME: EQUAL (94744 B sha256 6204cf5e8cca4b64; 94744 B sha256 6204cf5e8cca4b64)
exit 0
```

### ua CHILDOF cv.py --own (cu8a)
```
a-cu8a-C-u-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
ua's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801 (new vouchers 6)
  observation: its GUID is not I-ua.xml's REMOTEID
OWN: YES
exit 0
```

### ua CHILDOF wx.py --match movement (qu8a against k8)
```
MATCH ua movement: EQUAL (1 voucher, BP/26-27/0298 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ua CHILDOF wx.py --match window (wu8a against k8)
```
MATCH ua window: EQUAL (1 voucher, BP/26-27/0298 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0298 New Ref -1051.00; 14 own fields equal)
exit 0
```

### ua CHILDOF sk.py --judge census (cu8a)
```
JUDGE ua census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ua CHILDOF sk.py --judge movement (qu8a)
```
JUDGE ua movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ua CHILDOF sk.py --judge window (wu8a)
```
JUDGE ua window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ua the form against the form, census (c8a, cu8a) cv.py --same
```
SAME: EQUAL (5374 B sha256 ad7b25b05a99c3ee; 5374 B sha256 ad7b25b05a99c3ee)
exit 0
```

### ua the form against the form, movement (q8a, qu8a) wx.py --same
```
SAME: EQUAL (11946 B sha256 cb7a5b06cbb0e9b3; 11946 B sha256 cb7a5b06cbb0e9b3)
exit 0
```

### ua the form against the form, window (w8a, wu8a) wx.py --same
```
SAME: EQUAL (94744 B sha256 6204cf5e8cca4b64; 94744 B sha256 6204cf5e8cca4b64)
exit 0
```

### c9a (ub, ##LedgerName) cv.py
```
a-c9a-C-v-ub.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
exit 0
```

### c9b cv.py
```
a-c9b-C-v-ub.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
exit 0
```

### c9 cv.py --same
```
SAME: EQUAL (5374 B sha256 aea398957ce87d1b; 5374 B sha256 aea398957ce87d1b)
exit 0
```

### q9a (ub, ##LedgerName) wx.py
```
a-q9a-Vm-v-ub.xml: ANSWER: VOUCHERS 1
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### q9b wx.py
```
a-q9b-Vm-v-ub.xml: ANSWER: VOUCHERS 1
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### q9 wx.py --same
```
SAME: EQUAL (11946 B sha256 11c0d45f21c0e385; 11946 B sha256 11c0d45f21c0e385)
exit 0
```

### w9a (ub, ##LedgerName) wx.py
```
a-w9a-Vw-v-ub.xml: ANSWER: VOUCHERS 1
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### w9b wx.py
```
a-w9b-Vw-v-ub.xml: ANSWER: VOUCHERS 1
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### w9 wx.py --same
```
SAME: EQUAL (94744 B sha256 d9bc3e0f55edb4db; 94744 B sha256 d9bc3e0f55edb4db)
exit 0
```

### ub ##LedgerName cv.py --own (c9a)
```
a-c9a-C-v-ub.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
ub's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801 (new vouchers 6)
  observation: its GUID is not I-ub.xml's REMOTEID
OWN: YES
exit 0
```

### ub ##LedgerName wx.py --match movement (q9a against k8)
```
MATCH ub movement: EQUAL (1 voucher, BP/26-27/0299 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ub ##LedgerName wx.py --match window (w9a against k8)
```
MATCH ub window: EQUAL (1 voucher, BP/26-27/0299 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0299 New Ref -1051.00; 14 own fields equal)
exit 0
```

### ub ##LedgerName sk.py --judge census (c9a)
```
JUDGE ub census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ub ##LedgerName sk.py --judge movement (q9a)
```
JUDGE ub movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ub ##LedgerName sk.py --judge window (w9a)
```
JUDGE ub window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### a10 wx.py (Vb-book.xml)
```
a-a10-Vb-book.xml: ANSWER: VOUCHERS 17
  BP/26-27/0001 DATE 20260801 ALTERID 2 entries 4
  BP/26-27/0002 DATE 20260801 ALTERID 11 entries 4
  BP/26-27/0003 DATE 20260801 ALTERID 50 entries 4
  BP/26-27/0004 DATE 20260801 ALTERID 13 entries 5
  BP/26-27/0005 DATE 20260801 ALTERID 54 entries 5
  BP/26-27/0006 DATE 20260801 ALTERID 48 entries 4
  BP/26-27/0007 DATE 20260801 ALTERID 8 entries 5
  BP/26-27/0008 DATE 20260801 ALTERID 9 entries 5
  BP/26-27/0009 DATE 20260801 ALTERID 10 entries 7
  BP/26-27/0011 DATE 20260801 ALTERID 16 entries 4
  BP/26-27/0010 DATE 20260801 ALTERID 17 entries 6
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### a10 wx.py --same against k8
```
SAME: EQUAL (1361516 B sha256 3fd7b8ffc5965cc4; 1361516 B sha256 3fd7b8ffc5965cc4)
exit 0
```

### a11
```
== a-a11-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### a11 count20.py --same-row against k9
```
== a-a11-M-marks.xml against a-k9-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'")
  equal
exit 0
```

### before b1a: sc.py --judge-b ka movement (k8, k8)
```
JUDGE-B ka movement: OWN YES, OTHERS 16
exit 3
```

### b1a (ka, outstandings, L-lo-ka-20260801.xml) nm.py
```
== a-b1a-L-lo-ka-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0292' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### b1b nm.py
```
== a-b1b-L-lo-ka-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0292' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### b1 wx.py --same
```
SAME: EQUAL (896 B sha256 c903147e8e3bd785; 896 B sha256 c903147e8e3bd785)
exit 0
```

### b1 sc.py --judge-b ka outstandings
```
JUDGE-B ka outstandings: OWN YES, OTHERS 0
exit 0
```

### b2a (ka, census, C-v-ka.xml) cv.py
```
a-b2a-C-v-ka.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
exit 0
```

### b2b cv.py
```
a-b2b-C-v-ka.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
exit 0
```

### b2 wx.py --same
```
SAME: EQUAL (5374 B sha256 7ceaf55cb3f94042; 5374 B sha256 7ceaf55cb3f94042)
exit 0
```

### b2 sc.py --judge-b ka census
```
JUDGE-B ka census: OWN YES, OTHERS 0
exit 0
```

### b3a (ka, movement, Vm-v-ka.xml) wx.py
```
a-b3a-Vm-v-ka.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b3b wx.py
```
a-b3b-Vm-v-ka.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b3 wx.py --same
```
SAME: EQUAL (11946 B sha256 17ea00c4b315a2e7; 11946 B sha256 17ea00c4b315a2e7)
exit 0
```

### b3 sc.py --judge-b ka movement
```
JUDGE-B ka movement: OWN YES, OTHERS 0
exit 0
```

### b4a (ka, window, Vw-v-ka.xml) wx.py
```
a-b4a-Vw-v-ka.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b4b wx.py
```
a-b4b-Vw-v-ka.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b4 wx.py --same
```
SAME: EQUAL (94744 B sha256 15c268a3ee2131d2; 94744 B sha256 15c268a3ee2131d2)
exit 0
```

### b4 sc.py --judge-b ka window
```
JUDGE-B ka window: OWN YES, OTHERS 0
exit 0
```

### b5a (ka, CHILDOF, census, C-u-ka.xml) cv.py
```
a-b5a-C-u-ka.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
exit 0
```

### b5b cv.py
```
a-b5b-C-u-ka.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
exit 0
```

### b5 wx.py --same
```
SAME: EQUAL (5374 B sha256 7ceaf55cb3f94042; 5374 B sha256 7ceaf55cb3f94042)
exit 0
```

### b5 sc.py --judge-b ka census
```
JUDGE-B ka census: OWN YES, OTHERS 0
exit 0
```

### b6a (ka, CHILDOF, movement, Vm-u-ka.xml) wx.py
```
a-b6a-Vm-u-ka.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b6b wx.py
```
a-b6b-Vm-u-ka.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b6 wx.py --same
```
SAME: EQUAL (11946 B sha256 17ea00c4b315a2e7; 11946 B sha256 17ea00c4b315a2e7)
exit 0
```

### b6 sc.py --judge-b ka movement
```
JUDGE-B ka movement: OWN YES, OTHERS 0
exit 0
```

### b7a (ka, CHILDOF, window, Vw-u-ka.xml) wx.py
```
a-b7a-Vw-u-ka.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b7b wx.py
```
a-b7b-Vw-u-ka.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b7 wx.py --same
```
SAME: EQUAL (94744 B sha256 15c268a3ee2131d2; 94744 B sha256 15c268a3ee2131d2)
exit 0
```

### b7 sc.py --judge-b ka window
```
JUDGE-B ka window: OWN YES, OTHERS 0
exit 0
```

### b8a (kb, outstandings, L-lo-kb-20260801.xml) nm.py
```
== a-b8a-L-lo-kb-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0292' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### b8b nm.py
```
== a-b8b-L-lo-kb-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0292' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### b8 wx.py --same
```
SAME: EQUAL (896 B sha256 c903147e8e3bd785; 896 B sha256 c903147e8e3bd785)
exit 0
```

### b8 sc.py --judge-b kb outstandings
```
JUDGE-B kb outstandings: OWN YES, OTHERS 0
exit 0
```

### b9a (kb, census, C-v-kb.xml) cv.py
```
a-b9a-C-v-kb.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
exit 0
```

### b9b cv.py
```
a-b9b-C-v-kb.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
exit 0
```

### b9 wx.py --same
```
SAME: EQUAL (5374 B sha256 7ceaf55cb3f94042; 5374 B sha256 7ceaf55cb3f94042)
exit 0
```

### b9 sc.py --judge-b kb census
```
JUDGE-B kb census: OWN YES, OTHERS 0
exit 0
```

### b10a (kb, movement, Vm-v-kb.xml) wx.py
```
a-b10a-Vm-v-kb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b10b wx.py
```
a-b10b-Vm-v-kb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b10 wx.py --same
```
SAME: EQUAL (11946 B sha256 17ea00c4b315a2e7; 11946 B sha256 17ea00c4b315a2e7)
exit 0
```

### b10 sc.py --judge-b kb movement
```
JUDGE-B kb movement: OWN YES, OTHERS 0
exit 0
```

### b11a (kb, window, Vw-v-kb.xml) wx.py
```
a-b11a-Vw-v-kb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b11b wx.py
```
a-b11b-Vw-v-kb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
exit 0
```

### b11 wx.py --same
```
SAME: EQUAL (94744 B sha256 15c268a3ee2131d2; 94744 B sha256 15c268a3ee2131d2)
exit 0
```

### b11 sc.py --judge-b kb window
```
JUDGE-B kb window: OWN YES, OTHERS 0
exit 0
```

### the spellings against each other: b1a, b8a wx.py --same
```
SAME: EQUAL (896 B sha256 c903147e8e3bd785; 896 B sha256 c903147e8e3bd785)
exit 0
```

### the spellings against each other: b2a, b9a wx.py --same
```
SAME: EQUAL (5374 B sha256 7ceaf55cb3f94042; 5374 B sha256 7ceaf55cb3f94042)
exit 0
```

### the spellings against each other: b3a, b10a wx.py --same
```
SAME: EQUAL (11946 B sha256 17ea00c4b315a2e7; 11946 B sha256 17ea00c4b315a2e7)
exit 0
```

### the spellings against each other: b4a, b11a wx.py --same
```
SAME: EQUAL (94744 B sha256 15c268a3ee2131d2; 94744 B sha256 15c268a3ee2131d2)
exit 0
```

### the forms against each other for ka: b2a, b5a wx.py --same
```
SAME: EQUAL (5374 B sha256 7ceaf55cb3f94042; 5374 B sha256 7ceaf55cb3f94042)
exit 0
```

### the forms against each other for ka: b3a, b6a wx.py --same
```
SAME: EQUAL (11946 B sha256 17ea00c4b315a2e7; 11946 B sha256 17ea00c4b315a2e7)
exit 0
```

### the forms against each other for ka: b4a, b7a wx.py --same
```
SAME: EQUAL (94744 B sha256 15c268a3ee2131d2; 94744 B sha256 15c268a3ee2131d2)
exit 0
```

### b12 wx.py (Vb-book.xml)
```
a-b12-Vb-book.xml: ANSWER: VOUCHERS 17
  BP/26-27/0001 DATE 20260801 ALTERID 2 entries 4
  BP/26-27/0002 DATE 20260801 ALTERID 11 entries 4
  BP/26-27/0003 DATE 20260801 ALTERID 50 entries 4
  BP/26-27/0004 DATE 20260801 ALTERID 13 entries 5
  BP/26-27/0005 DATE 20260801 ALTERID 54 entries 5
  BP/26-27/0006 DATE 20260801 ALTERID 48 entries 4
  BP/26-27/0007 DATE 20260801 ALTERID 8 entries 5
  BP/26-27/0008 DATE 20260801 ALTERID 9 entries 5
  BP/26-27/0009 DATE 20260801 ALTERID 10 entries 7
  BP/26-27/0011 DATE 20260801 ALTERID 16 entries 4
  BP/26-27/0010 DATE 20260801 ALTERID 17 entries 6
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### b12 wx.py --same against k8
```
SAME: EQUAL (1361516 B sha256 3fd7b8ffc5965cc4; 1361516 B sha256 3fd7b8ffc5965cc4)
exit 0
```

### b13
```
== a-b13-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### b13 count20.py --same-row against a11
```
== a-b13-M-marks.xml against a-a11-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'")
  equal
exit 0
```

### s9 (count20.py)
```
== a-s9-N-0281.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0292.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0293.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0294.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0295.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0296.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0297.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0298.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0299.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0300.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0301.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s9 (nm.py)
```
== a-s9-B-bills.xml
  BILLFIXED groups: 0
  pl 'BRIDGE Mx Plain': 0 groups
  ka 'BRIDGE Sk Ak Am': 0 groups
  kb 'BRIDGE Sk Ak, Am': 0 groups
  ra 'BRIDGE Sk Bk, Bm': 0 groups
  rb 'BRIDGE Sk Bk Bm': 0 groups
  qa 'BRIDGE Sk Ck Cm': 0 groups
  qb "BRIDGE Sk Ck' Cm": 0 groups
  ua 'BRIDGE Sk Dk Dm': 0 groups
  ub 'BRIDGE Sk Dk_Dm': 0 groups
  ea 'BRIDGE Sk EkEm': 0 groups
  eb 'BRIDGE Sk Ek Em': 0 groups
  groups of any other party: 0
== a-s9-S-snap-20260401-20260801.xml
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Mx Plain': no row
  ka 'BRIDGE Sk Ak Am': no row
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': no row
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': no row
  qb "BRIDGE Sk Ck' Cm": no row
  ua 'BRIDGE Sk Dk Dm': no row
  ub 'BRIDGE Sk Dk_Dm': no row
  ea 'BRIDGE Sk EkEm': no row
  eb 'BRIDGE Sk Ek Em': no row
exit 0
```

### s9 st.py, the before option
```
a-s9-S-snap-20260401-20260801.xml: LEDGER rows 18
FOLD: CLEAR
exit 0
```

### s9 cv.py (C-book.xml)
```
a-s9-C-book.xml: ANSWER: VOUCHERS 11
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001 ALTERID 2 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002 ALTERID 11 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003 ALTERID 50 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004 ALTERID 13 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005 ALTERID 54 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006 ALTERID 48 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007 ALTERID 8 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008 ALTERID 9 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009 ALTERID 10 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a ALTERID 16 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b ALTERID 17 DATE 20260801
exit 0
```

### s9 sk.py --names (G-cat.xml)
```
a-s9-G-cat.xml: LEDGER rows 18
  NAMES pl: no row
  NAMES ka: no row
  NAMES kb: no row
  NAMES ra: no row
  NAMES rb: no row
  NAMES qa: no row
  NAMES qb: no row
  NAMES ua: no row
  NAMES ub: no row
  NAMES ea: no row
  NAMES eb: no row
exit 0
```

### s9 against s2, byte for byte
```
equal a-s9-B-bills.xml
equal a-s9-C-book.xml
equal a-s9-G-cat.xml
equal a-s9-M-marks.xml
equal a-s9-N-0281.xml
equal a-s9-N-0292.xml
equal a-s9-N-0293.xml
equal a-s9-N-0294.xml
equal a-s9-N-0295.xml
equal a-s9-N-0296.xml
equal a-s9-N-0297.xml
equal a-s9-N-0298.xml
equal a-s9-N-0299.xml
equal a-s9-N-0300.xml
equal a-s9-N-0301.xml
equal a-s9-S-snap-20260401-20260801.xml
16 of 16 equal
```

### s9 count20.py --same-row against s1
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```
