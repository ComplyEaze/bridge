# Row 2480-SK: of the five pairs, the comma in both orders and the space alone were taken by Tally as an alteration of the first ledger; the apostrophe and the underscore made a second ledger, and each of those four names, in all four scoped reads, read its own invoice and never its sibling's

Run on 11 Nov 2026, 03:09:46Z to 03:35Z (08:39 to 09:05 IST), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`, equal to row 2480-MW's record; the Gateway shows no release number). The sheet was `lab/2480-sk-sheet` (c5f9d52f), with all 119 files equal to its `SHA256.txt`. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone, in a sitting of its own with a Tally start of its own. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

The set-up and the reads are judged apart, as the sheet says: each section is written from its own answers.

## The set-up (steps 1 to 5)

**Confidence: M, measured by this run, in one run, for each line below; PARTIAL as the sheet's Decides says (one book, one release, one sitting, master's request templates sent by curl, without the build's own parse).** Every send of the set-up was answered: each with curl exit 0 and HTTP 200, with bytes. No dialog, message or prompt showed but the two of C5's and C6's own (none at the loads or the shuts). Every capture after an import, after each catalogue read and after each step 4 read was the Gateway, the same image as s1's.

**Step 1.** The Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (neither changed). s1's marks were 68/245, as the sheet asks, before the backup.

**Step 2, the book before.** Each N read 0 rows; B-bills no group for any of the eleven names; the snapshot 18 LEDGER rows and "no row" for each name; `st.py` (the before option) `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; `cv.py` on the book-wide census `ANSWER: VOUCHERS 11`; the marks 68/245. **`G-cat.xml`, master's standard ledger catalogue, was sent to this book for the first time:** it answered 23,100 B with 18 LEDGER rows and no dialog, and `sk.py --names` gave "no row" for each of the eleven.

**Step 3, the set-up.** Each write by the sheet's recipe, one label a command, each after a marks read that passed the before-write check (22 checks, each exit 0). The control's two imports and each pair's first two were `CREATED` 1 with the other counts 0 and no LINEERROR (`IMPORT: CREATED ONE`). For each pair's second ledger:

| pair | names (a, b) | how b differs | s4&lt;b&gt; answer | recipe | the pair's result |
| --- | --- | --- | --- | --- | --- |
| k | `BRIDGE Sk Ak Am`, `BRIDGE Sk Ak, Am` | a comma, the plain name first | CREATED 0, ALTERED 1, the rest 0, no LINEERROR | (e): `sk.py --altered` ALTERED ONE; s4kbx 70/248, the before-write check passed | **ALTERED** |
| r | `BRIDGE Sk Bk, Bm`, `BRIDGE Sk Bk Bm` | a comma, the marked name first | CREATED 0, ALTERED 1, the rest 0, no LINEERROR | (e): ALTERED ONE; s4rbx 71/250, passed | **ALTERED** |
| q | `BRIDGE Sk Ck Cm`, `BRIDGE Sk Ck' Cm` | an apostrophe | CREATED 1 (`IMPORT: CREATED ONE`) | s7qb: `sk.py --pair qa qb` 23 LEDGER rows, both `EQUAL`, two GUIDs: BOTH; then s5qb CREATED ONE | **BOTH** |
| u | `BRIDGE Sk Dk Dm`, `BRIDGE Sk Dk_Dm` | an underscore in place of the space | CREATED 1 (`IMPORT: CREATED ONE`) | s7ub: `sk.py --pair ua ub` 25 LEDGER rows, both `EQUAL`, two GUIDs: BOTH; then s5ub CREATED ONE | **BOTH** |
| e | `BRIDGE Sk EkEm`, `BRIDGE Sk Ek Em` | a space alone | CREATED 0, ALTERED 1, the rest 0, no LINEERROR | (e): ALTERED ONE; s4ebx 76/256, passed | **ALTERED** |

On each alteration, `count20.py --same-row` of the x read against the marks read before that import printed that the COMPANY row differs (ALTMSTID one higher, ALTVCHID the same), exit 1: the sheet makes that line a record, not a check, as row NM saw each alteration move the master mark by one. **No import was refused, so `rf.py` was never run.** Recipe (e) was used for exactly the three second ledgers above and for nothing else. The invoices' `LASTVCHID` ran from 45 (pl) through 46 (ka), 47 (ra), 48 (qa), 49 (qb), 50 (ua) and 51 (ub) to 52 (ea), in the order sent; no invoice of `kb`, `rb` or `eb` was sent.

**Step 4, the book-wide reads.** s6m 76/256. The snapshot, sent twice: 35,400 B each, byte-equal, 26 LEDGER rows; a row for each of `pl`, `ka`, `ra`, `qa`, `qb`, `ua`, `ub` and `ea`, each `Sundry Debtors`, bill-wise Yes, opening 0.00, closing -1051.00; "no row" for `kb`, `rb` and `eb`. B-bills, sent twice: 3,710 B each, byte-equal, 8 BILLFIXED groups, one for each of those eight names, each `BP/26-27/<its number>`, 1-Aug-26, -1051.00, none of any other party. The catalogue, sent twice: 31,898 B each, byte-equal, 26 LEDGER rows. The census, sent twice: 49,338 B each, byte-equal, `ANSWER: VOUCHERS 19` (s2's 11 and this row's 8).

**What an alteration did to the first ledger's names.** In both the snapshot and the catalogue, each altered pair's first ledger kept its `NAME` attribute, the first name as sent, while its name list (`LANGUAGENAME.LIST`) now held the second name alone:

| pair | `NAME` attribute | name list after step 3 |
| --- | --- | --- |
| k | `BRIDGE Sk Ak Am` | `BRIDGE Sk Ak, Am` |
| r | `BRIDGE Sk Bk, Bm` | `BRIDGE Sk Bk Bm` |
| e | `BRIDGE Sk EkEm` | `BRIDGE Sk Ek Em` |

So `st.py --names` gave `NAMES: DIFFER` naming these three rows and no other, and `sk.py --names` gave `DIFFER` for `ka`, `ra` and `ea` and "no row" for `kb`, `rb` and `eb`. Every other name's list was its own name alone. That ledger kept its one invoice and its bill under the first name (B-bills and the snapshot, above).

**INVOICED** (the ids whose invoice import gave `CREATED ONE`): `pl,ka,ra,qa,qb,ua,ub,ea`. **READ**: `pl,qa,qb,ua,ub` (the control, then both ids of pairs q and u, each BOTH with both `s6ga` lines `EQUAL`; no pair was BOTH, CHANGED BY STEP 4). Each id of INVOICED was in the intended state.

**Step 5.** m1's marks were equal to s6m's (76/256): step 4 only read. No m1r was sent.

**The set-up whole, before k0** (written in `evidence/notes.txt` before k0, each with the line that shows it): no stop at steps 1 to 5; every send answered; m1 equal to s6m; every before-write check passed; `pl` in READ and in the intended state. All five held, so the reads started. Three pairs' results other than BOTH are results, not a failure of the set-up.

**So, by the sheet's Decides, for each pair at step 3:**
- **k and r, ALTERED (a comma, in both orders):** on this book and release, a gateway `Create` of a name that differs from an existing ledger's by a comma alone was taken as an alteration of that ledger, whichever of the two names came first. No two ledgers can share a key by a comma through the gateway; the read side has nothing to guard for that mark, and the write side's refusal is confirmed as needed.
- **e, ALTERED (a space alone):** the same, for two names that differ by one space.
- **q and u, BOTH (an apostrophe; an underscore in place of the space):** a gateway `Create` made a second ledger beside the first, both names kept as sent, each its own GUID, in the catalogue master's build reads. The reads were then measured.

**What the set-up does not say.** Why Tally took the comma and the space as one ledger and the apostrophe and the underscore as two; whether a pair made on Tally's own screens, or in another order, would do the same; whether the altered ledger's new name list changes what any read sends back for the first name (no name of an altered pair was read, as the sheet says).

### The set-up's deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. Every script ran as `python -I` (Python 3.10), unchanged, from the row folder, through the helpers in `scripts/`.
- **The run lead's three words for this row** (in the queue entry): (a) an empty answer to an `L-lo-`, `Vm-v-`, `Vw-v-` or `C-v-` request with HTTP 200 and `CONTENT-LENGTH: 0` is not a stop; (b) an import that `rf.py` reads as refused, with an equal marks read after it, is that id's result; (c) recipe (e)'s alteration rule, for a pair's second ledger only. `scripts/er.sh` carried (a), which was not needed; `scripts/w.sh` carried (b), which was not needed, and (c), which gave the three ALTERED results.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **The BK path field.** At C5's Specify Path, the typed path went in ahead of the field's old `C:\`, as in rows 2480-MX and 2480-MW. End and three Backspaces took that off before Enter, and the field read `C:\lab\2480-sk\backup-1` alone; the BK went to that folder (one file, `evidence/bk.txt`).
- **The three Bridge processes** that ran before the row were left running, as the sheet says. None was called by the row.
- **Sends run in the background, with a 300 s watch.** From step 2 on, each block of sends was started by `scripts/bg.sh`, which writes the block's output and exit to `evidence/run-<name>.out` and `.rc`, beside `scripts/wd.sh`, which watched the same block for 5 minutes and ended when it did. Each block ended inside the watch; the longest, step 7's first, took 96 s. s1 (one marks read) was sent in the foreground.
- **Step 3 by pair.** The control's four labels ran as one command, `scripts/ctl.sh`; each pair's labels as one command, `scripts/pair.sh <a> <b>`, which runs `scripts/w.sh` for each label in the recipe's order, sends `s7<b>` and runs `sk.py --pair` only after `CREATED ONE`, and ends at the pair's result or at the first answer the recipe does not name. Each pair's result was written to `evidence/pair-results.txt` before the next pair started. Step 4 was sent by `scripts/s6.sh`.
- **The Gateway images.** `evidence/gateway-refs.txt` carries the three Gateway images of row 2480-NX, each looked at there, as in MX and MW. This row's s1 capture was byte-equal to the first; every capture after a send was that same image (`ca58f95e…`), as was s9's.
- **A note's value.** The pair r line of the notes first printed the s4rbx marks blank (the line's own text extraction failed); a correction line after it gives them (71/250). No other text changed.

## The reads (steps 6 and 7)

**Confidence: M, measured by this run, in one run, for each line below; PARTIAL as the sheet's Decides says (one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse).** Every send of steps 6 and 7 was answered: each with curl exit 0 and HTTP 200, with bytes; no answer was empty, so exception (a) was never needed. No dialog showed. Every capture after a send was the Gateway image above.

**Step 6, the shape step, the control alone.** Each send was checked before the next, by `scripts/k.sh`:

| label | file | answer | check | gave |
| --- | --- | --- | --- | --- |
| k0 | `M-marks.xml` | 76/256 | `--same-row` against m1 | SAME MARKS |
| k1 | `Vm-u-pl.xml` | 11,946 B | `wx.py`; `sk.py --ref` | `ANSWER: VOUCHERS 1`; `REF DIFFER` (kept 11,954 B) |
| k2 | `Vm-v-pl.xml` | 11,946 B | `wx.py --same` against k1; `sk.py --ref` | `SAME: EQUAL`; `REF DIFFER` |
| k3 | `Vw-u-pl.xml` | 94,744 B | `wx.py`; `sk.py --ref` | `ANSWER: VOUCHERS 1`; `REF DIFFER` (kept 94,752 B) |
| k4 | `Vw-v-pl.xml` | 94,744 B | `wx.py --same` against k3; `sk.py --ref` | `SAME: EQUAL`; `REF DIFFER` |
| k5 | `L-lo-pl-20260801.xml` | 896 B | `nm.py`; `sk.py --ref` | `ANSWER: ROWS 1 (0 On Account)`; `REF EQUAL` |
| k6 | `C-u-pl.xml` | 5,374 B | `cv.py` | `ANSWER: VOUCHERS 1` |
| k7 | `C-v-pl.xml` | 5,374 B | `cv.py --same` against k6 | `SAME: EQUAL` |
| k8 | `Vb-book.xml` | 1,545,368 B | `wx.py`; `--match pl movement` (k1); `--match pl window` (k3); `cv.py --own pl` | `ANSWER: VOUCHERS 19`; `EQUAL`; `EQUAL`; `OWN: YES` |
| k9 | `M-marks.xml` | 76/256 | `--same-row` against k0 | SAME MARKS |

**The kept bytes.** Not every pair's result was BOTH, so by the sheet the `--ref` lines were run and recorded, not required. The four voucher answers were each 8 B shorter than the answers rows MX and MW kept for the same request, with the book holding three ledgers and three invoices fewer than then; the outstandings answer, which counts no object of the book, was byte-equal to MX's. The shape step's gate was the rest of the table, and every check gave what it names. No k0r or k9r was sent. **The shape step passed.**

**Step 7, the pair reads.** For each id of READ, in table order, eight sends, each read by its reader before the next request, then one line each of `nm.py --against`, `cv.py --own`, `wx.py --match` (movement, window) and `sk.py --judge` for the four forms:

| n | id | name | outstandings: r a, r b (`--same`); `--against` | census: c a, c b (`--same`); `--own` | movement: q a, q b (`--same`); `MATCH` | window: w a, w b (`--same`); `MATCH` |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | pl | `BRIDGE Mx Plain` | ROWS 1, ROWS 1 (EQUAL, 896 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 5,374 B); YES | VOUCHERS 1, VOUCHERS 1 (EQUAL, 11,946 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 94,744 B); EQUAL |
| 6 | qa | `BRIDGE Sk Ck Cm` | ROWS 1, ROWS 1 (EQUAL, 896 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 5,374 B); YES | VOUCHERS 1, VOUCHERS 1 (EQUAL, 11,946 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 94,744 B); EQUAL |
| 7 | qb | `BRIDGE Sk Ck' Cm` | ROWS 1, ROWS 1 (EQUAL, 896 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 5,374 B); YES | VOUCHERS 1, VOUCHERS 1 (EQUAL, 11,970 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 94,768 B); EQUAL |
| 8 | ua | `BRIDGE Sk Dk Dm` | ROWS 1, ROWS 1 (EQUAL, 896 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 5,374 B); YES | VOUCHERS 1, VOUCHERS 1 (EQUAL, 11,946 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 94,744 B); EQUAL |
| 9 | ub | `BRIDGE Sk Dk_Dm` | ROWS 1, ROWS 1 (EQUAL, 896 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 5,374 B); YES | VOUCHERS 1, VOUCHERS 1 (EQUAL, 11,946 B); EQUAL | VOUCHERS 1, VOUCHERS 1 (EQUAL, 94,744 B); EQUAL |

**The pair's judgement, one line a name a form** (`sk.py --judge`, each against k8, each exit 0):

| id | outstandings | census | movement | window |
| --- | --- | --- | --- | --- |
| pl | OWN YES, SIBLING none, OTHERS 0 | OWN YES, SIBLING none, OTHERS 0 | OWN YES, SIBLING none, OTHERS 0 | OWN YES, SIBLING none, OTHERS 0 |
| qa | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |
| qb | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |
| ua | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |
| ub | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 | OWN YES, SIBLING NO, OTHERS 0 |

Each `MATCH` line names that id's own invoice, `BP/26-27/<its number>` dated 20260801, matched by VOUCHERKEY, its line on the ledger Yes -1051.00, with 10 own fields equal (movement) or 14 own fields equal and the bill `BP/26-27/<its number>` New Ref -1051.00 (window). Each `--against` line found the outstandings row equal to the B-bills group and its sum equal to the snapshot's closing balance. Each `--own` line found the census answer to hold that id's own invoice and nothing else. The whole outputs are below. Then k10 (`Vb-book.xml`, the file k8 sent): `ANSWER: VOUCHERS 19`, `SAME: EQUAL` against k8 (1,545,368 B each), so every judgement was made against a book-wide read that did not change during step 7. k11's marks were equal to k9's (76/256): step 7 only read. No k11r was sent.

**So, by the sheet's Decides:** for each name of READ, in each of the four forms, `OWN YES, SIBLING NO, OTHERS 0` (or `none` for the control), with the pair `SAME: EQUAL` and its own form's line `EQUAL` or `YES`: on this book and release, `Ledger Outstandings` with the name as its `LedgerName` variable, the ledger census, the movement read and the window read, each in the `##LedgerName` form, told `BRIDGE Sk Ck Cm` from `BRIDGE Sk Ck' Cm` and `BRIDGE Sk Dk Dm` from `BRIDGE Sk Dk_Dm`, each pair sharing one letters-and-digits key. **Not every pair was BOTH**, so the sheet's last Decides line (every pair BOTH and every name the first line in all four forms) does not hold as a whole: for the comma and the space alone, the gateway made no second ledger, so nothing was read. For the apostrophe and the underscore, the measured reads told the names apart; whether the read-side one-stem condition may be narrowed for those two marks is the maintainers' choice, on #2480 and #485. The write side keeps #708's refusal.

**What the reads do not say.** Whether the build's own parse accepts these answers; the reads Bridge sends around them; any other book, release or day; a window of more than one day, another AlterID span or a book of real size; master's `CHILDOF` form for a pair's name; any name of an altered pair; three or more ledgers sharing one key; a ledger with more than one voucher or bill, or a settled bill; a book with GST on. The readers' empty-answer and `STATUS` paths met no real answer here.

### The reads' deviations from the sheet

- **`scripts/k.sh`** sent k0 to k9 one at a time, each check run before the next send, and would have ended at the first check that did not give what the sheet names; none did. With `ALLBOTH` 0 (not every pair BOTH), it ran each `sk.py --ref` and recorded it without requiring `EQUAL`, as the sheet says.
- **Step 7 in three runs.** To keep each block inside the 300 s watch, step 7 ran as three blocks of `scripts/r.sh`: `pl` and `qa` (96 s), `qb` and `ua` (about 100 s), then `ub` (about 50 s), each with its own watch. `r.sh` numbers each id by its place in the table, so the labels are the sheet's. No id of READ was left unread. Each send went by `scripts/er.sh`, which ends with exit 3 when the answer came with bytes (its answered path, on all 40 sends here) and carries exception (a) for an empty one. k10 and k11 were sent by `scripts/k10.sh`.
- **Captures after each b send.** `er.sh` captures the window after each of its sends, so each `b` send has a capture as well as each `a` send.

## The way back

**Step 8.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`, 16 of 16), each N read 0 rows, no group and no row for any of the eleven names, the catalogue "no row" for each, `FOLD: CLEAR`, and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1, the three altered name lists included.

**The side files.** Before Tally started, four of the seven files that the restores of rows 2480-NX, 2480-CV, 2480-MX and 2480-MW left out were in the book's data folder (`CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`; 20 entries). At load, before step 1's reads, six were (23 entries): `TCMPSCHMESSAGE.TSF` and `TINTMSG.TSF` were back; `Index.1800` was absent. Before the restore the folder held all seven (24 entries); this row's restore left out the same seven (17 entries after), and at the load after it the same six were back and `Index.1800` absent (23 entries), as in those rows. The listings stay on the lab PC.

**Timings.** Every read sent without a window capture after it returned within about 1 s of its send line (0.384 to 1.031 s), wall clock, python's start included. The labels sent with a capture took 3.158 to 16.034 s, the capture included: the imports 3.237 to 16.034 s (`s5ra`, the first invoice after the first alteration, the longest; the next, `s5ka`, 7.675 s), the two catalogue reads of step 3 3.158 and 3.335 s, s2 and s9 (sixteen reads each) 5.943 and 5.324 s, k0 to k9 3.231 to 3.559 s, and r1a to w9b 3.212 to 3.817 s (`evidence/durations.txt`).

## Steps
- Step 0 (03:09:46Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. No `tally.exe` ran.
- Tally started from its desktop shortcut (03:13:43Z). The start-up company was shut (the set-up's deviations). `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`, byte-equal to row 2480-MW's).
- Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; last entry 15-Sep-26. The data folder listed (the side files above). Step 0 C2 (03:14:51Z): one `tally.exe`, which alone owns 9001; GET /status status=200, bytes=51. s1: the book's GUID, ALTVCHID 68, ALTMSTID 245.
- BK by C5 (capture before Accept 03:16:33Z): Data, Backup & Restore, Backup; Backspace to the path, Specify Path, `C:\lab\2480-sk\backup-1` (the field mended as above); the book the one company; End of List; Accept y; back at the Gateway with no message. One file of 90,051 B, at 08:46 IST, 11 Nov (`evidence/bk.txt`).
- Step 2 (03:17:13Z to 03:17:27Z), step 3 (03:17:47Z to 03:22:31Z, the control and five pairs), step 4 (03:22:51Z to 03:23:36Z), step 5 (03:24:00Z to 03:24:05Z), step 6 (03:24:2xZ to 03:25:14Z) and step 7 (03:25:31Z to 03:30:11Z) as above, each in the notes.
- Step 8 C6 (prep 03:30:26Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter, no prompt (the first Enter was not sent, as the keyboard focus was not on Tally; a click on its title bar came first); Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; the list opened at the drive's root; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (8:46 am), as the plan; the book selected alone; End of List; Accept ? y (03:32:07Z); the folder-exists question with the book's number (03:32:24Z), r Replace; back at Select Company with no other prompt. The data folder listed again (the side files above). The book loaded alone by the name filter (`s9-select` showed the one row; this time not byte-equal to `s1-select`, 36,401 B against 36,413 B, looked at); Gateway captured, byte-equal to s1's, date and period unchanged; then s9 (03:33:31Z to 03:33:43Z), equal to s2 throughout.
- Step 9, C10 (03:34:21Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded (the capture equal to row 2480-MW's C10 capture); one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; then no `tally.exe` ran and nothing listened on 9001. The BK, listed again, one file, its sha256 equal to `evidence/bk.txt`.

## Files
- `answers/`: every answer and its headers file, whole, byte for byte (`kept-local.txt` lists any that stay on the lab PC).
- `logs/`: `run.py`'s log of each label.
- `evidence/`: notes (UTC, one line per step); every `count20.py`, `nm.py`, `st.py`, `cv.py`, `wx.py`, `rf.py` and `sk.py` output, whole, with its exit; each send's UTC time and request check; each background block's output and exit; each pair's results; the Gateway images' list; the per-label wall clock; the book check; `tally.exe`'s file data. Process, port, folder and backup listings and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers. `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `wx.py`, `cv.py` and `sk.py` are the sheet's files, byte for byte, and are not repeated.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.
- `substitutions.txt`: where the lab's own text differs from the files kept on the lab PC. In the lab's own text each 16-hex sha256 head that `wx.py --same`, `cv.py --same` or `sk.py --ref` printed and that holds a run of 10 or more digits is replaced by that answer's whole sha256, on the run lead's word of row 2480-MW.

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

### s4ram count20.py (the before-write option)
```
== a-s4ram-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ra st.py --import
```
a-s4ra-LM-ra.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5ram count20.py (the before-write option)
```
== a-s5ram-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ra st.py --import
```
a-s5ra-I-ra.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 47, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4rbm count20.py (the before-write option)
```
== a-s4rbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4rb st.py --import
```
a-s4rb-LM-rb.xml: CREATED 0, ALTERED 1, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: stop (CREATED is not 1; ALTERED is not 0)
exit 1
```

### s4rb sk.py --altered
```
a-s4rb-LM-rb.xml: CREATED 0, ALTERED 1, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
ALTERED ONE
exit 0
```

### s4rbx count20.py (the before-write option)
```
== a-s4rbx-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4rbx count20.py --same-row against s4rbm, a record
```
== a-s4rbx-M-marks.xml against a-s4rbm-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'")
stop: the COMPANY row differs
exit 1
```

### s4qam count20.py (the before-write option)
```
== a-s4qam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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
== a-s5qam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5qa st.py --import
```
a-s5qa-I-qa.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4qbm count20.py (the before-write option)
```
== a-s4qbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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
a-s7qb-G-cat.xml: LEDGER rows 23
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
PAIR qa qb: BOTH
exit 0
```

### s7qb sk.py --names
```
a-s7qb-G-cat.xml: LEDGER rows 23
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: DIFFER (its names are ['BRIDGE Sk Bk Bm'])
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
== a-s5qbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5qb st.py --import
```
a-s5qb-I-qb.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 49, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4uam count20.py (the before-write option)
```
== a-s4uam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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
== a-s5uam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ua st.py --import
```
a-s5ua-I-ua.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 50, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4ubm count20.py (the before-write option)
```
== a-s4ubm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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
a-s7ub-G-cat.xml: LEDGER rows 25
  NAMES ua: EQUAL (one row, its own GUID)
  NAMES ub: EQUAL (one row, its own GUID)
PAIR ua ub: BOTH
exit 0
```

### s7ub sk.py --names
```
a-s7ub-G-cat.xml: LEDGER rows 25
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: DIFFER (its names are ['BRIDGE Sk Bk Bm'])
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
== a-s5ubm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ub st.py --import
```
a-s5ub-I-ub.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 51, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4eam count20.py (the before-write option)
```
== a-s4eam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ea st.py --import
```
a-s4ea-LM-ea.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5eam count20.py (the before-write option)
```
== a-s5eam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ea st.py --import
```
a-s5ea-I-ea.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 52, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4ebm count20.py (the before-write option)
```
== a-s4ebm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4eb st.py --import
```
a-s4eb-LM-eb.xml: CREATED 0, ALTERED 1, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: stop (CREATED is not 1; ALTERED is not 0)
exit 1
```

### s4eb sk.py --altered
```
a-s4eb-LM-eb.xml: CREATED 0, ALTERED 1, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
ALTERED ONE
exit 0
```

### s4ebx count20.py (the before-write option)
```
== a-s4ebx-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ebx count20.py --same-row against s4ebm, a record
```
== a-s4ebx-M-marks.xml against a-s4ebm-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'255'")
stop: the COMPANY row differs
exit 1
```

### the pairs' results
```
2026-11-11T03:19:12Z pair k (ka, kb): ALTERED
2026-11-11T03:20:22Z pair r (ra, rb): ALTERED
2026-11-11T03:21:09Z pair q (qa, qb): BOTH
2026-11-11T03:21:55Z pair u (ua, ub): BOTH
2026-11-11T03:22:30Z pair e (ea, eb): ALTERED
```

### s6m
```
== a-s6m-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '76' ALTMSTID '256'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s6sa nm.py
```
== a-s6sa-S-snap-20260401-20260801.xml
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 26
  pl 'BRIDGE Mx Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ka 'BRIDGE Sk Ak Am': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  qb "BRIDGE Sk Ck' Cm": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ua 'BRIDGE Sk Dk Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ub 'BRIDGE Sk Dk_Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ea 'BRIDGE Sk EkEm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  eb 'BRIDGE Sk Ek Em': no row
exit 0
```

### s6sb nm.py
```
== a-s6sb-S-snap-20260401-20260801.xml
a-s6sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 26
  pl 'BRIDGE Mx Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ka 'BRIDGE Sk Ak Am': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  qb "BRIDGE Sk Ck' Cm": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ua 'BRIDGE Sk Dk Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ub 'BRIDGE Sk Dk_Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ea 'BRIDGE Sk EkEm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  eb 'BRIDGE Sk Ek Em': no row
exit 0
```

### s6sa/s6sb nm.py --same
```
  a-s6sa-S-snap-20260401-20260801.xml: 35400 bytes, sha256 402f0e14f549b702d33d72cd0d8ec24f1a4fe8545126de370a75ef6668c3dfd1
  a-s6sb-S-snap-20260401-20260801.xml: 35400 bytes, sha256 402f0e14f549b702d33d72cd0d8ec24f1a4fe8545126de370a75ef6668c3dfd1
SAME: EQUAL
exit 0
```

### s6ba nm.py
```
== a-s6ba-B-bills.xml
  BILLFIXED groups: 8
  pl 'BRIDGE Mx Plain': 1 groups
    BILLREF 'BP/26-27/0281'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ka 'BRIDGE Sk Ak Am': 1 groups
    BILLREF 'BP/26-27/0292'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  kb 'BRIDGE Sk Ak, Am': 0 groups
  ra 'BRIDGE Sk Bk, Bm': 1 groups
    BILLREF 'BP/26-27/0294'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  rb 'BRIDGE Sk Bk Bm': 0 groups
  qa 'BRIDGE Sk Ck Cm': 1 groups
    BILLREF 'BP/26-27/0296'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  qb "BRIDGE Sk Ck' Cm": 1 groups
    BILLREF 'BP/26-27/0297'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ua 'BRIDGE Sk Dk Dm': 1 groups
    BILLREF 'BP/26-27/0298'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ub 'BRIDGE Sk Dk_Dm': 1 groups
    BILLREF 'BP/26-27/0299'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ea 'BRIDGE Sk EkEm': 1 groups
    BILLREF 'BP/26-27/0300'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  eb 'BRIDGE Sk Ek Em': 0 groups
  groups of any other party: 0
exit 0
```

### s6bb nm.py
```
== a-s6bb-B-bills.xml
  BILLFIXED groups: 8
  pl 'BRIDGE Mx Plain': 1 groups
    BILLREF 'BP/26-27/0281'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ka 'BRIDGE Sk Ak Am': 1 groups
    BILLREF 'BP/26-27/0292'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  kb 'BRIDGE Sk Ak, Am': 0 groups
  ra 'BRIDGE Sk Bk, Bm': 1 groups
    BILLREF 'BP/26-27/0294'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  rb 'BRIDGE Sk Bk Bm': 0 groups
  qa 'BRIDGE Sk Ck Cm': 1 groups
    BILLREF 'BP/26-27/0296'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  qb "BRIDGE Sk Ck' Cm": 1 groups
    BILLREF 'BP/26-27/0297'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ua 'BRIDGE Sk Dk Dm': 1 groups
    BILLREF 'BP/26-27/0298'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ub 'BRIDGE Sk Dk_Dm': 1 groups
    BILLREF 'BP/26-27/0299'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ea 'BRIDGE Sk EkEm': 1 groups
    BILLREF 'BP/26-27/0300'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  eb 'BRIDGE Sk Ek Em': 0 groups
  groups of any other party: 0
exit 0
```

### s6ba/s6bb nm.py --same
```
  a-s6ba-B-bills.xml: 3710 bytes, sha256 c0b58cb1a65fd1e2b046f18a90af46bdf9178d60e9ac6ea0756b78ec8d49f24b
  a-s6bb-B-bills.xml: 3710 bytes, sha256 c0b58cb1a65fd1e2b046f18a90af46bdf9178d60e9ac6ea0756b78ec8d49f24b
SAME: EQUAL
exit 0
```

### s6ga sk.py --names
```
a-s6ga-G-cat.xml: LEDGER rows 26
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: DIFFER (its names are ['BRIDGE Sk Bk Bm'])
  NAMES rb: no row
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
  NAMES ua: EQUAL (one row, its own GUID)
  NAMES ub: EQUAL (one row, its own GUID)
  NAMES ea: DIFFER (its names are ['BRIDGE Sk Ek Em'])
  NAMES eb: no row
exit 0
```

### s6gb sk.py --names
```
a-s6gb-G-cat.xml: LEDGER rows 26
  NAMES pl: EQUAL (one row, its own GUID)
  NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])
  NAMES kb: no row
  NAMES ra: DIFFER (its names are ['BRIDGE Sk Bk Bm'])
  NAMES rb: no row
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
  NAMES ua: EQUAL (one row, its own GUID)
  NAMES ub: EQUAL (one row, its own GUID)
  NAMES ea: DIFFER (its names are ['BRIDGE Sk Ek Em'])
  NAMES eb: no row
exit 0
```

### s6ga/s6gb --same
```
  a-s6ga-G-cat.xml: 31898 bytes, sha256 99d864c291019c0c54aa6d78720065e331aa9f2366bea3502316a8e60a787f63
  a-s6gb-G-cat.xml: 31898 bytes, sha256 99d864c291019c0c54aa6d78720065e331aa9f2366bea3502316a8e60a787f63
SAME: EQUAL
exit 0
```

### s6ka cv.py
```
a-s6ka-C-book.xml: ANSWER: VOUCHERS 19
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
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000034 ALTERID 76 DATE 20260801
exit 0
```

### s6kb cv.py
```
a-s6kb-C-book.xml: ANSWER: VOUCHERS 19
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
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000034 ALTERID 76 DATE 20260801
exit 0
```

### s6ka/s6kb cv.py --same
```
  a-s6ka-C-book.xml: 49338 bytes, sha256 7d6db612fd3d2b3013a5cef5e124bcf47225a75e7f5651f98a8a0fe7d041c236
  a-s6kb-C-book.xml: 49338 bytes, sha256 7d6db612fd3d2b3013a5cef5e124bcf47225a75e7f5651f98a8a0fe7d041c236
SAME: EQUAL
exit 0
```

### s6 st.py --names
```
a-s6sa-S-snap-20260401-20260801.xml: LEDGER rows 26
  name list not the NAME attribute: NAME 'BRIDGE Sk Ak Am', names ['BRIDGE Sk Ak, Am']
  name list not the NAME attribute: NAME 'BRIDGE Sk Bk, Bm', names ['BRIDGE Sk Bk Bm']
  name list not the NAME attribute: NAME 'BRIDGE Sk EkEm', names ['BRIDGE Sk Ek Em']
  pl 'BRIDGE Mx Plain': names ['BRIDGE Mx Plain']
  ka 'BRIDGE Sk Ak Am': names ['BRIDGE Sk Ak, Am']
  kb 'BRIDGE Sk Ak, Am': no row
  ra 'BRIDGE Sk Bk, Bm': names ['BRIDGE Sk Bk Bm']
  rb 'BRIDGE Sk Bk Bm': no row
  qa 'BRIDGE Sk Ck Cm': names ['BRIDGE Sk Ck Cm']
  qb "BRIDGE Sk Ck' Cm": names ["BRIDGE Sk Ck' Cm"]
  ua 'BRIDGE Sk Dk Dm': names ['BRIDGE Sk Dk Dm']
  ub 'BRIDGE Sk Dk_Dm': names ['BRIDGE Sk Dk_Dm']
  ea 'BRIDGE Sk EkEm': names ['BRIDGE Sk Ek Em']
  eb 'BRIDGE Sk Ek Em': no row
NAMES: DIFFER
exit 0
```

### m1 count20.py --same-row against s6m
```
== a-m1-M-marks.xml against a-s6m-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'")
  equal
exit 0
```

### k0 count20.py --same-row against m1
```
== a-k0-M-marks.xml against a-m1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'")
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
REF Vm-u-pl.xml: DIFFER (11946 B sha256 bab794dd1417d3c2; kept 11954 B sha256 b8eab7b26d95eccb)
exit 3
```

### k2 wx.py --same against k1
```
SAME: EQUAL (11946 B sha256 bab794dd1417d3c2; 11946 B sha256 bab794dd1417d3c2)
exit 0
```

### k2 sk.py --ref
```
REF Vm-v-pl.xml: DIFFER (11946 B sha256 bab794dd1417d3c2; kept 11954 B sha256 b8eab7b26d95eccb)
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
REF Vw-u-pl.xml: DIFFER (94744 B sha256 defbad8a3668ec20; kept 94752 B sha256 f158857ba662a10f)
exit 3
```

### k4 wx.py --same against k3
```
SAME: EQUAL (94744 B sha256 defbad8a3668ec20; 94744 B sha256 defbad8a3668ec20)
exit 0
```

### k4 sk.py --ref
```
REF Vw-v-pl.xml: DIFFER (94744 B sha256 defbad8a3668ec20; kept 94752 B sha256 f158857ba662a10f)
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
SAME: EQUAL (5374 B sha256 7bf8ec4b1d3efdd8; 5374 B sha256 7bf8ec4b1d3efdd8)
exit 0
```

### k8 wx.py (Vb-book.xml)
```
a-k8-Vb-book.xml: ANSWER: VOUCHERS 19
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
  BP/26-27/0294 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0296 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0297 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0298 DATE 20260801 ALTERID 74 entries 6
  BP/26-27/0299 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0300 DATE 20260801 ALTERID 76 entries 6
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
pl's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801 (new vouchers 8)
  observation: its GUID is not I-pl.xml's REMOTEID
OWN: YES
exit 0
```

### k9 count20.py --same-row against k0
```
== a-k9-M-marks.xml against a-k0-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'")
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

### r1a (pl) nm.py
```
== a-r1a-L-lo-pl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0281' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r1b nm.py
```
== a-r1b-L-lo-pl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0281' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r1 nm.py --same
```
  a-r1a-L-lo-pl-20260801.xml: 896 bytes, sha256 19c796774ac740b6a26490c1fd7a9fd017a7d8b709a71540a98194d1d6d58b8e
  a-r1b-L-lo-pl-20260801.xml: 896 bytes, sha256 19c796774ac740b6a26490c1fd7a9fd017a7d8b709a71540a98194d1d6d58b8e
SAME: EQUAL
exit 0
```

### c1a (pl) cv.py
```
a-c1a-C-v-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
exit 0
```

### c1b cv.py
```
a-c1b-C-v-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
exit 0
```

### c1 cv.py --same
```
SAME: EQUAL (5374 B sha256 7bf8ec4b1d3efdd8; 5374 B sha256 7bf8ec4b1d3efdd8)
exit 0
```

### q1a (pl) wx.py
```
a-q1a-Vm-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### q1b wx.py
```
a-q1b-Vm-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### q1 wx.py --same
```
SAME: EQUAL (11946 B sha256 bab794dd1417d3c2; 11946 B sha256 bab794dd1417d3c2)
exit 0
```

### w1a (pl) wx.py
```
a-w1a-Vw-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### w1b wx.py
```
a-w1b-Vw-v-pl.xml: ANSWER: VOUCHERS 1
  BP/26-27/0281 DATE 20260801 ALTERID 69 entries 6
exit 0
```

### w1 wx.py --same
```
SAME: EQUAL (94744 B sha256 defbad8a3668ec20; 94744 B sha256 defbad8a3668ec20)
exit 0
```

### pl nm.py --against (r1a, s6ba, s6sa)
```
  BILLREF 'BP/26-27/0281' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx Plain': 1
    BILLREF 'BP/26-27/0281'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0281): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### pl cv.py --own (c1a)
```
a-c1a-C-v-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
pl's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801 (new vouchers 8)
  observation: its GUID is not I-pl.xml's REMOTEID
OWN: YES
exit 0
```

### pl wx.py --match movement (q1a against k8)
```
MATCH pl movement: EQUAL (1 voucher, BP/26-27/0281 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### pl wx.py --match window (w1a against k8)
```
MATCH pl window: EQUAL (1 voucher, BP/26-27/0281 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0281 New Ref -1051.00; 14 own fields equal)
exit 0
```

### pl sk.py --judge outstandings (r1a)
```
JUDGE pl outstandings: OWN YES, SIBLING none, OTHERS 0
exit 0
```

### pl sk.py --judge census (c1a)
```
JUDGE pl census: OWN YES, SIBLING none, OTHERS 0
exit 0
```

### pl sk.py --judge movement (q1a)
```
JUDGE pl movement: OWN YES, SIBLING none, OTHERS 0
exit 0
```

### pl sk.py --judge window (w1a)
```
JUDGE pl window: OWN YES, SIBLING none, OTHERS 0
exit 0
```

### r6a (qa) nm.py
```
== a-r6a-L-lo-qa-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0296' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r6b nm.py
```
== a-r6b-L-lo-qa-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0296' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r6 nm.py --same
```
  a-r6a-L-lo-qa-20260801.xml: 896 bytes, sha256 efd59941745930bb42028b3b4415ff4df674a94371be28a3ffd276841f998bfd
  a-r6b-L-lo-qa-20260801.xml: 896 bytes, sha256 efd59941745930bb42028b3b4415ff4df674a94371be28a3ffd276841f998bfd
SAME: EQUAL
exit 0
```

### c6a (qa) cv.py
```
a-c6a-C-v-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
exit 0
```

### c6b cv.py
```
a-c6b-C-v-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
exit 0
```

### c6 cv.py --same
```
SAME: EQUAL (5374 B sha256 33983e637255389b; 5374 B sha256 33983e637255389b)
exit 0
```

### q6a (qa) wx.py
```
a-q6a-Vm-v-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### q6b wx.py
```
a-q6b-Vm-v-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### q6 wx.py --same
```
SAME: EQUAL (11946 B sha256 0516080da74d945f; 11946 B sha256 0516080da74d945f)
exit 0
```

### w6a (qa) wx.py
```
a-w6a-Vw-v-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### w6b wx.py
```
a-w6b-Vw-v-qa.xml: ANSWER: VOUCHERS 1
  BP/26-27/0296 DATE 20260801 ALTERID 72 entries 6
exit 0
```

### w6 wx.py --same
```
SAME: EQUAL (94744 B sha256 43e5c99da0f4fc3f; 94744 B sha256 43e5c99da0f4fc3f)
exit 0
```

### qa nm.py --against (r6a, s6ba, s6sa)
```
  BILLREF 'BP/26-27/0296' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Sk Ck Cm': 1
    BILLREF 'BP/26-27/0296'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0296): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### qa cv.py --own (c6a)
```
a-c6a-C-v-qa.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
qa's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801 (new vouchers 8)
  observation: its GUID is not I-qa.xml's REMOTEID
OWN: YES
exit 0
```

### qa wx.py --match movement (q6a against k8)
```
MATCH qa movement: EQUAL (1 voucher, BP/26-27/0296 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### qa wx.py --match window (w6a against k8)
```
MATCH qa window: EQUAL (1 voucher, BP/26-27/0296 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0296 New Ref -1051.00; 14 own fields equal)
exit 0
```

### qa sk.py --judge outstandings (r6a)
```
JUDGE qa outstandings: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qa sk.py --judge census (c6a)
```
JUDGE qa census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qa sk.py --judge movement (q6a)
```
JUDGE qa movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qa sk.py --judge window (w6a)
```
JUDGE qa window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### r7a (qb) nm.py
```
== a-r7a-L-lo-qb-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0297' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r7b nm.py
```
== a-r7b-L-lo-qb-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0297' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r7 nm.py --same
```
  a-r7a-L-lo-qb-20260801.xml: 896 bytes, sha256 64638a1697edf8484505c011650d5ff6dfd3ff9749973c640148e0f24e4c9bc9
  a-r7b-L-lo-qb-20260801.xml: 896 bytes, sha256 64638a1697edf8484505c011650d5ff6dfd3ff9749973c640148e0f24e4c9bc9
SAME: EQUAL
exit 0
```

### c7a (qb) cv.py
```
a-c7a-C-v-qb.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
exit 0
```

### c7b cv.py
```
a-c7b-C-v-qb.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
exit 0
```

### c7 cv.py --same
```
SAME: EQUAL (5374 B sha256 f2c31bff91447805; 5374 B sha256 f2c31bff91447805)
exit 0
```

### q7a (qb) wx.py
```
a-q7a-Vm-v-qb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0297 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### q7b wx.py
```
a-q7b-Vm-v-qb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0297 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### q7 wx.py --same
```
SAME: EQUAL (11970 B sha256 bdd9a7473850d2bf; 11970 B sha256 bdd9a7473850d2bf)
exit 0
```

### w7a (qb) wx.py
```
a-w7a-Vw-v-qb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0297 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### w7b wx.py
```
a-w7b-Vw-v-qb.xml: ANSWER: VOUCHERS 1
  BP/26-27/0297 DATE 20260801 ALTERID 73 entries 6
exit 0
```

### w7 wx.py --same
```
SAME: EQUAL (94768 B sha256 0a01414347dff20d; 94768 B sha256 0a01414347dff20d)
exit 0
```

### qb nm.py --against (r7a, s6ba, s6sa)
```
  BILLREF 'BP/26-27/0297' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named "BRIDGE Sk Ck' Cm": 1
    BILLREF 'BP/26-27/0297'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0297): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### qb cv.py --own (c7a)
```
a-c7a-C-v-qb.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
qb's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801 (new vouchers 8)
  observation: its GUID is not I-qb.xml's REMOTEID
OWN: YES
exit 0
```

### qb wx.py --match movement (q7a against k8)
```
MATCH qb movement: EQUAL (1 voucher, BP/26-27/0297 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### qb wx.py --match window (w7a against k8)
```
MATCH qb window: EQUAL (1 voucher, BP/26-27/0297 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0297 New Ref -1051.00; 14 own fields equal)
exit 0
```

### qb sk.py --judge outstandings (r7a)
```
JUDGE qb outstandings: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qb sk.py --judge census (c7a)
```
JUDGE qb census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qb sk.py --judge movement (q7a)
```
JUDGE qb movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### qb sk.py --judge window (w7a)
```
JUDGE qb window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### r8a (ua) nm.py
```
== a-r8a-L-lo-ua-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0298' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r8b nm.py
```
== a-r8b-L-lo-ua-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0298' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r8 nm.py --same
```
  a-r8a-L-lo-ua-20260801.xml: 896 bytes, sha256 b2ee053b684259bccd95901cff230a63d30ac6aabfb30d00b46a1908d07879e2
  a-r8b-L-lo-ua-20260801.xml: 896 bytes, sha256 b2ee053b684259bccd95901cff230a63d30ac6aabfb30d00b46a1908d07879e2
SAME: EQUAL
exit 0
```

### c8a (ua) cv.py
```
a-c8a-C-v-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
exit 0
```

### c8b cv.py
```
a-c8b-C-v-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
exit 0
```

### c8 cv.py --same
```
SAME: EQUAL (5374 B sha256 165b9dd966209c00; 5374 B sha256 165b9dd966209c00)
exit 0
```

### q8a (ua) wx.py
```
a-q8a-Vm-v-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### q8b wx.py
```
a-q8b-Vm-v-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### q8 wx.py --same
```
SAME: EQUAL (11946 B sha256 d5f88014ae01e6ec; 11946 B sha256 d5f88014ae01e6ec)
exit 0
```

### w8a (ua) wx.py
```
a-w8a-Vw-v-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### w8b wx.py
```
a-w8b-Vw-v-ua.xml: ANSWER: VOUCHERS 1
  BP/26-27/0298 DATE 20260801 ALTERID 74 entries 6
exit 0
```

### w8 wx.py --same
```
SAME: EQUAL (94744 B sha256 d87e4178360563a2e14fd1a4237d32b4721359879005f58dec8a8a225c873ecb; 94744 B sha256 d87e4178360563a2e14fd1a4237d32b4721359879005f58dec8a8a225c873ecb)
exit 0
```

### ua nm.py --against (r8a, s6ba, s6sa)
```
  BILLREF 'BP/26-27/0298' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Sk Dk Dm': 1
    BILLREF 'BP/26-27/0298'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0298): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### ua cv.py --own (c8a)
```
a-c8a-C-v-ua.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
ua's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801 (new vouchers 8)
  observation: its GUID is not I-ua.xml's REMOTEID
OWN: YES
exit 0
```

### ua wx.py --match movement (q8a against k8)
```
MATCH ua movement: EQUAL (1 voucher, BP/26-27/0298 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ua wx.py --match window (w8a against k8)
```
MATCH ua window: EQUAL (1 voucher, BP/26-27/0298 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0298 New Ref -1051.00; 14 own fields equal)
exit 0
```

### ua sk.py --judge outstandings (r8a)
```
JUDGE ua outstandings: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ua sk.py --judge census (c8a)
```
JUDGE ua census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ua sk.py --judge movement (q8a)
```
JUDGE ua movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ua sk.py --judge window (w8a)
```
JUDGE ua window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### r9a (ub) nm.py
```
== a-r9a-L-lo-ub-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0299' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r9b nm.py
```
== a-r9b-L-lo-ub-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0299' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r9 nm.py --same
```
  a-r9a-L-lo-ub-20260801.xml: 896 bytes, sha256 51469113410b458717db14082627f3ca78b062494fe066bbd271aa9cf6a8ec2a
  a-r9b-L-lo-ub-20260801.xml: 896 bytes, sha256 51469113410b458717db14082627f3ca78b062494fe066bbd271aa9cf6a8ec2a
SAME: EQUAL
exit 0
```

### c9a (ub) cv.py
```
a-c9a-C-v-ub.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
exit 0
```

### c9b cv.py
```
a-c9b-C-v-ub.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
exit 0
```

### c9 cv.py --same
```
SAME: EQUAL (5374 B sha256 4a2687abdb89edd7; 5374 B sha256 4a2687abdb89edd7)
exit 0
```

### q9a (ub) wx.py
```
a-q9a-Vm-v-ub.xml: ANSWER: VOUCHERS 1
  BP/26-27/0299 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### q9b wx.py
```
a-q9b-Vm-v-ub.xml: ANSWER: VOUCHERS 1
  BP/26-27/0299 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### q9 wx.py --same
```
SAME: EQUAL (11946 B sha256 fda0a6a9eebaabb8; 11946 B sha256 fda0a6a9eebaabb8)
exit 0
```

### w9a (ub) wx.py
```
a-w9a-Vw-v-ub.xml: ANSWER: VOUCHERS 1
  BP/26-27/0299 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### w9b wx.py
```
a-w9b-Vw-v-ub.xml: ANSWER: VOUCHERS 1
  BP/26-27/0299 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### w9 wx.py --same
```
SAME: EQUAL (94744 B sha256 77c920f8ff25ed5d; 94744 B sha256 77c920f8ff25ed5d)
exit 0
```

### ub nm.py --against (r9a, s6ba, s6sa)
```
  BILLREF 'BP/26-27/0299' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Sk Dk_Dm': 1
    BILLREF 'BP/26-27/0299'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0299): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### ub cv.py --own (c9a)
```
a-c9a-C-v-ub.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
ub's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801 (new vouchers 8)
  observation: its GUID is not I-ub.xml's REMOTEID
OWN: YES
exit 0
```

### ub wx.py --match movement (q9a against k8)
```
MATCH ub movement: EQUAL (1 voucher, BP/26-27/0299 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; 10 own fields equal)
exit 0
```

### ub wx.py --match window (w9a against k8)
```
MATCH ub window: EQUAL (1 voucher, BP/26-27/0299 of 20260801, matched by VOUCHERKEY; its line Yes -1051.00; bill BP/26-27/0299 New Ref -1051.00; 14 own fields equal)
exit 0
```

### ub sk.py --judge outstandings (r9a)
```
JUDGE ub outstandings: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ub sk.py --judge census (c9a)
```
JUDGE ub census: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ub sk.py --judge movement (q9a)
```
JUDGE ub movement: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### ub sk.py --judge window (w9a)
```
JUDGE ub window: OWN YES, SIBLING NO, OTHERS 0
exit 0
```

### k10 wx.py (Vb-book.xml)
```
a-k10-Vb-book.xml: ANSWER: VOUCHERS 19
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
  BP/26-27/0294 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0296 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0297 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0298 DATE 20260801 ALTERID 74 entries 6
  BP/26-27/0299 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0300 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### k10 wx.py --same against k8
```
SAME: EQUAL (1545368 B sha256 8fa3b7f42e2cbc08; 1545368 B sha256 8fa3b7f42e2cbc08)
exit 0
```

### k11
```
== a-k11-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '76' ALTMSTID '256'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### k11 count20.py --same-row against k9
```
== a-k11-M-marks.xml against a-k9-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'256'")
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
N-0281.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0292.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0293.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0294.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0295.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0296.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0297.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0298.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0299.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0300.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
N-0301.xml s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: EQUAL
B-bills.xml s2 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3; s9 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3: EQUAL
S-snap-20260401-20260801.xml s2 25480 B ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69; s9 25480 B ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69: EQUAL
C-book.xml s2 29814 B 1cd8c03cae154deef7eb72f68d3d73b5c757d3bfe517907e750b31fe8d4ee0a0; s9 29814 B 1cd8c03cae154deef7eb72f68d3d73b5c757d3bfe517907e750b31fe8d4ee0a0: EQUAL
G-cat.xml s2 23100 B d05d2af351d78027720df3f9e95116f41bd14b9b1f82ea95fcc50764dd2cd205; s9 23100 B d05d2af351d78027720df3f9e95116f41bd14b9b1f82ea95fcc50764dd2cd205: EQUAL
M-marks.xml s2 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6; s9 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6: EQUAL
```

### s9 count20.py --same-row against s1
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```
