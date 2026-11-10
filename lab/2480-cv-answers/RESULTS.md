# Row 2480-CV: the ledger-scoped census with `##LedgerName` alone in `CHILDOF` read the control as master's form does, byte for byte; each of the ten names answered no voucher before its ledger existed, and its own invoice alone after

Run on 10 Nov 2026, 23:30:12Z to 23:51:59Z (05:00 to 05:21 IST, 11 Nov), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`, equal to rows 2480-NX's and 2480-NC's; the Gateway shows no release number). The sheet was `lab/2480-cv-sheet` (9de0dee1), with all 75 files equal to its `SHA256.txt` (sizes too); `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `M-marks.xml`, `B-bills.xml`, the snapshot request, `names.tsv` and the 33 `LM-`, `I-` and `N-` files byte-equal to row 2480-NX's. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone, in a sitting of its own with a Tally start of its own. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

Every value below is M, measured by this run, in one run. Every read was answered, each with curl exit 0 and HTTP 200, with bytes; no dialog, message or prompt showed but the two of C6 (none at the backup, the loads or the shuts). Every capture after a send was the Gateway, the same image as s1's (Deviations).

## Result

**Step 1.** s1's marks were 68/245, as the run lead's condition asks, before the backup.

**Step 3, before the control's ledger existed (p1, p2).** `C-u-pl.xml` (master's form, the name in `CHILDOF`) and `C-v-pl.xml` (`CHILDOF` holding only `##LedgerName`, the name in the static variable) were each answered HTTP 200 with 2,926 B: `cv.py` `ANSWER: VOUCHERS 0` for each, `SAME: EQUAL`. The empty-answer exception was not needed. p3's marks were equal to s1's.

**Step 4, the control.** `LM-pl.xml` and `I-pl.xml` each `IMPORT: CREATED ONE` (the invoice `LASTVCHID` 45), each after a marks read that passed the before-write check.

**Step 5, the shape step: passed.** c1 (`C-u-pl.xml`), c2 (`C-uv-pl.xml`, master's form with the `LedgerName` variable added) and c3 (`C-v-pl.xml`) each `ANSWER: VOUCHERS 1`, the one voucher `-0000002d`, ALTERID 69, dated 20260801. c3 and c2 were each byte-equal to c1 (5,372 B; `SAME: EQUAL`). `cv.py --own pl` on c1: `OWN: YES`, the control's own invoice and nothing else. c4, the book-wide census, `VOUCHERS 12`. c0 and c5 marks equal (69/246). **So the `##LedgerName` form gave master's bytes for the control, and the added variable changed nothing in master's form.**

**Step 6, the ten before their ledgers existed (p4 to p13).** Each `C-v-<id>.xml` was answered with 2,928 B, `ANSWER: VOUCHERS 0`. **No name was answered with a voucher before its own ledger existed**, so no id was skipped. Each answer differs from p2's (`SAME: DIFFER`): the ten are byte-equal to one another, and differ from p2 only in the answer's header counts (`LEDGER` 0 to 12; `CURRENCY`, `TAXUNIT` and `VOUCHERNUMBERSERIES` 0 to 1; `VOUCHER` 0 to 2), since p2 was read before step 4 made the control's ledger and invoice. Recorded as each name's result, as the sheet says; the step went on. p14's marks were equal to c5's.

**Step 7, the set-up of the ten.** All 20 imports, each sent once, in table order, each after a marks read that passed the before-write check: every answer `CREATED` 1 and the other counts 0, with no LINEERROR, so `st.py --import` gave `IMPORT: CREATED ONE` on each. The invoices' `LASTVCHID` ran from 46 to 55 in table order. **No import was refused, so `rf.py` was never run.** CREATED: `pl,cm,us,dq,ps,at,hs,cl,pc,st,ex`.

**Step 8, the book-wide reads: the intended state.** s6m 79/256. The snapshot, sent twice: 39,182 B each, byte-equal, 29 LEDGER rows, a row for each of the eleven, each `Sundry Debtors`, bill-wise Yes, opening 0.00, closing -1051.00; `st.py --names` `NAMES: EQUAL`. B-bills, sent twice: 5,112 B each, byte-equal, 11 BILLFIXED groups, one for each name, `BP/26-27/0261` to `BP/26-27/0271` in table order, each dated 1-Aug-26 and -1051.00. The book-wide census, sent twice: 56,666 B each, byte-equal, `VOUCHERS 22`: every voucher of s2's census (11) and 11 new ones, each dated 20260801.

**Step 9, the ledger reads (r1a to r11b).** Each id of CREATED was read twice with its `C-v-<id>.xml`, the file steps 3 and 6 sent:

| n | id | name | its mark | r<n>a, r<n>b | `cv.py` | its voucher | `--same` | `--own` |
|---|---|---|---|---|---|---|---|---|
| 1 | `pl` | `BRIDGE Nx Plain` | none (the control) | 5,382 B each | VOUCHERS 1 | `-0000002d`, ALTERID 69 | EQUAL | YES |
| 2 | `cm` | `BRIDGE Nx Ak, Am` | `,` | 5,382 B each | VOUCHERS 1 | `-0000002e`, ALTERID 70 | EQUAL | YES |
| 3 | `us` | `BRIDGE Nx Bk_Bm` | `_` | 5,382 B each | VOUCHERS 1 | `-0000002f`, ALTERID 71 | EQUAL | YES |
| 4 | `dq` | `BRIDGE Nx "Ck" Cm` | `"` | 5,382 B each | VOUCHERS 1 | `-00000030`, ALTERID 72 | EQUAL | YES |
| 5 | `ps` | `BRIDGE Nx Dk + Dm` | `+` | 5,382 B each | VOUCHERS 1 | `-00000031`, ALTERID 73 | EQUAL | YES |
| 6 | `at` | `BRIDGE Nx Ek @ Em` | `@` | 5,382 B each | VOUCHERS 1 | `-00000032`, ALTERID 74 | EQUAL | YES |
| 7 | `hs` | `BRIDGE Nx Fk #12` | `#` | 5,382 B each | VOUCHERS 1 | `-00000033`, ALTERID 75 | EQUAL | YES |
| 8 | `cl` | `BRIDGE Nx Gk: Gm` | `:` | 5,382 B each | VOUCHERS 1 | `-00000034`, ALTERID 76 | EQUAL | YES |
| 9 | `pc` | `BRIDGE Nx Hk 15% Hm` | `%` | 5,382 B each | VOUCHERS 1 | `-00000035`, ALTERID 77 | EQUAL | YES |
| 10 | `st` | `BRIDGE Nx Jk * Jm` | `*` | 5,382 B each | VOUCHERS 1 | `-00000036`, ALTERID 78 | EQUAL | YES |
| 11 | `ex` | `BRIDGE Nx Kk!` | `!` | 5,382 B each | VOUCHERS 1 | `-00000037`, ALTERID 79 | EQUAL | YES |

Each voucher's GUID is the book's GUID followed by the number shown, each dated 20260801. `--own` YES: the answer holds exactly that id's own invoice, the one new voucher of the book-wide census at that id's place in CREATED, and nothing else. `cv.py --own` also printed, for each, the observation that the voucher's GUID is not its `I-` file's REMOTEID. r1a differs from c3 (5,382 B against 5,372 B) only in the header counts (`LEDGER` 12 to 132; `CURRENCY`, `TAXUNIT` and `VOUCHERNUMBERSERIES` 1 to 11; `VOUCHER` 2 to 22). **On the sheet's question, for the control and each of the ten marks, on this book and release: with `CHILDOF` holding only `##LedgerName` and the name in the static variable, the census read that one ledger, giving no voucher before it existed and its own invoice alone after, the two sends equal; and for the control, its bytes were master's form's.**

**Step 10.** m1's marks were equal to s6m's (79/256): steps 8 and 9 only read.

**Step 11, the way back.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order, `C-book.xml` included: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`), and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1.

**The side files (the run lead's request after row 2480-NX).** NX's restore had left seven of Tally's own files out of the book's data folder. Before Tally started, four were back (`CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`). After the book loaded, six were: `TCMPSCHMESSAGE.TSF` and `TINTMSG.TSF` were written at this load, with `TCMPSCHACCESS.TSF` and `TCMPSCHSTATE.TSF` written again. `Index.1800` was still absent at load; it was written during step 4's imports (05:06:05 IST). This row's restore left out the same seven, as NX's did, and at the load after it the same six were back and `Index.1800` absent. The listings stay on the lab PC.

**Timings.** Every read sent without a window capture after it returned within 2 s of its send line, wall clock, python's start included, but s2 and s9: s2's fifteen reads took 3.065 s together and s9's 2.465 s. The labels sent with a capture took 2.947 to 15.086 s, the capture included: the census reads p and r 2.947 to 6.796 s, the imports 3.404 to 9.314 s, and c1 to c3 5.038 to 15.086 s, c2 the longest (`evidence/durations.txt`).

**What this does not say.** Whether the build's own parse accepts these answers; any other book, release or mark; a name holding two or more of these marks; names that share a stem with another ledger; a census over another window; the composed path through a built executable. `rf.py`'s refusal path and the empty-answer exception met no real answer here.

## Deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. Every script ran as `python -I` (Python 3.10), unchanged, from the row folder, through the helpers in `scripts/`.
- **The run lead's two words for this row** (in the queue entry, noted at 23:30Z): (a) an empty `C-u-` or `C-v-` answer at steps 3, 6 and 9 with HTTP 200 and `CONTENT-LENGTH: 0` is not a stop; (b) an import that `rf.py` reads as refused, with an equal marks read after it, is that id's result. `scripts/cr.sh` carried (a) and `scripts/w.sh` carried (b). Neither path was taken: every census answer had bytes, and no import was refused.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **The three Bridge processes** that ran before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, as the sheet says; no process is ended on this PC. None was called by the row.
- **Sends run in the background, with a 300 s watch.** From step 2 on, each block of sends was started by `scripts/bg.sh`, which writes the block's output and exit to `evidence/run-<name>.out` and `.rc`, beside `scripts/wd.sh`, which watched the same block for 5 minutes and ended when it did. So no tool's time limit could end a curl that waited (the wedge rule: never end curl). Each block ended inside the watch; the longest, step 7, took 162 s. s1 and m1 (each one marks read) were sent in the foreground, m1 through `bg.sh`; each answered within 2 s.
- **One id a command, looped.** The sheet asks for one label a command. Each id's four write labels were sent by one command, `scripts/wid.sh <id>`, which runs `scripts/w.sh` for each in the recipe's order and ends at the first exit other than 0. Step 7 ran `wid.sh` for the ten ids by one loop, `scripts/s7.sh`, which went on only on exit 0 (or 4 or 5, a refusal, which did not occur). Steps 3, 5, 6, 8 and 9 were each sent by one loop (`p3.sh`, `c.sh`, `p6.sh`, `s8.sh`, `r9.sh`) that went on only when the previous label's helper passed.
- **The Gateway images.** `evidence/gateway-refs.txt` carries the three Gateway images of row 2480-NX, each looked at there. This row's s1 capture was byte-equal to the first. Every capture after a send in this row was that same image (`ca58f95e…`), as were s9's.
- **A summary line printed blank.** `scripts/p3.sh`'s own summary line after p1 and p2 printed no answer text: its grep looked for the answer at a line's start, and `cv.py` prints the file name first. The answers are in `evidence/cv-p1.txt` and `evidence/cv-p2.txt`. The script is pushed as it ran.
- **Captures after r<n>b.** `cr.sh` captures the window after each of its sends, so each r<n>b has a capture as well as each r<n>a, and each p and r label of steps 3 and 6.
- **The control's two imports took 8 to 10 s** each from the send line to the answer, the capture included (s4pl 8.407 s, s5pl 9.314 s); the twenty imports of step 7 took 3.404 to 5.091 s. Each answered, with no dialog.

## Steps
- Step 0 (23:30:12Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. No `tally.exe` ran.
- Tally started from its desktop shortcut (23:30:32Z). The start-up company was shut (Deviations). `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`).
- Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; last entry 15-Sep-26. Step 0 C2 (23:32:01Z): one `tally.exe`, which alone owns 9001; GET /status status=200, bytes=51. s1: the book's GUID, ALTVCHID 68, ALTMSTID 245.
- BK by C5 (capture before Accept 23:33:45Z): Data, Backup & Restore, Backup; Backspace to the path, Specify Path, `C:\lab\2480-cv\backup-1` (Tally showed `C:\lab\2480-Cv\backup-1`); the book the one company; End of List; Accept y; back at the Gateway with no message. One file of 90,051 B, at 05:03 IST, 11 Nov (`evidence/bk.txt`).
- Step 2 (s2): each N read 0 rows; B-bills no group; the snapshot 18 LEDGER rows and no row for any of the eleven; `st.py <the before option>` `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; the book-wide census `VOUCHERS 11`; the marks 68/245.
- Step 3 (23:35:00Z to 23:35:21Z), step 4 (23:35:58Z to 23:36:32Z), step 5 (23:36:54Z to 23:37:43Z), step 6 (23:37:57Z to 23:38:50Z), step 7 (23:40:06Z to 23:42:49Z), step 8 (23:43:05Z to 23:43:23Z), step 9 (23:43:59Z to 23:46:20Z) and step 10 (23:46:50Z) as in Result, each in the notes.
- Step 11 C6 (prep 23:47:01Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter, no prompt; Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; the list opened at the drive's root; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (5:03 am), as the plan; the book selected alone; End of List; Accept ? y (23:48:42Z); the folder-exists question with the book's number (23:49:04Z), r Replace; back at Select Company with no other prompt. The data folder listed again (the side files above). The book loaded alone by the name filter; Gateway captured, byte-equal to s1's, date and period unchanged; then s9 (23:50:32Z to 23:50:41Z), equal to s2 throughout.
- Step 12, C10 (23:51:25Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded; one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; then no `tally.exe` ran and nothing listened on 9001. The BK, listed again, one file, its sha256 equal to `evidence/bk.txt`.

## Files
- `answers/`: every answer and its headers file, whole, byte for byte (`kept-local.txt` lists any that stay on the lab PC).
- `logs/`: `run.py`'s log of each label.
- `evidence/`: notes (UTC, one line per step); every `count20.py`, `nm.py`, `st.py` and `cv.py` output, whole, with its exit; each send's UTC time and request check; each background block's output and exit; each id's write run; the Gateway images' list; the per-label wall clock; the book check; `tally.exe`'s file data. Process, port, folder and backup listings and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers. `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `cv.py` and `gen-cv.py` are the sheet's files, byte for byte, and are not repeated.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

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
== a-s2-N-0261.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0262.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0263.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0264.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0265.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0266.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0267.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0268.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0269.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0270.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0271.xml: 3022 bytes
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
  pl 'BRIDGE Nx Plain': 0 groups
  cm 'BRIDGE Nx Ak, Am': 0 groups
  us 'BRIDGE Nx Bk_Bm': 0 groups
  dq 'BRIDGE Nx "Ck" Cm': 0 groups
  ps 'BRIDGE Nx Dk + Dm': 0 groups
  at 'BRIDGE Nx Ek @ Em': 0 groups
  hs 'BRIDGE Nx Fk #12': 0 groups
  cl 'BRIDGE Nx Gk: Gm': 0 groups
  pc 'BRIDGE Nx Hk 15% Hm': 0 groups
  st 'BRIDGE Nx Jk * Jm': 0 groups
  ex 'BRIDGE Nx Kk!': 0 groups
  groups of any other party: 0
== a-s2-S-snap-20260401-20260801.xml
a-s2-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Nx Plain': no row
  cm 'BRIDGE Nx Ak, Am': no row
  us 'BRIDGE Nx Bk_Bm': no row
  dq 'BRIDGE Nx "Ck" Cm': no row
  ps 'BRIDGE Nx Dk + Dm': no row
  at 'BRIDGE Nx Ek @ Em': no row
  hs 'BRIDGE Nx Fk #12': no row
  cl 'BRIDGE Nx Gk: Gm': no row
  pc 'BRIDGE Nx Hk 15% Hm': no row
  st 'BRIDGE Nx Jk * Jm': no row
  ex 'BRIDGE Nx Kk!': no row
exit 0
```

### s2 st.py <the before option>
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
FOLD: CLEAR
exit 0
```

### s2 st.py --names
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
  pl 'BRIDGE Nx Plain': no row
  cm 'BRIDGE Nx Ak, Am': no row
  us 'BRIDGE Nx Bk_Bm': no row
  dq 'BRIDGE Nx "Ck" Cm': no row
  ps 'BRIDGE Nx Dk + Dm': no row
  at 'BRIDGE Nx Ek @ Em': no row
  hs 'BRIDGE Nx Fk #12': no row
  cl 'BRIDGE Nx Gk: Gm': no row
  pc 'BRIDGE Nx Hk 15% Hm': no row
  st 'BRIDGE Nx Jk * Jm': no row
  ex 'BRIDGE Nx Kk!': no row
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

### p1 cv.py
```
a-p1-C-u-pl.xml: ANSWER: VOUCHERS 0
exit 0
```

### p2 cv.py
```
a-p2-C-v-pl.xml: ANSWER: VOUCHERS 0
exit 0
```

### p1/p2 cv.py --same
```
SAME: EQUAL (2926 B sha256 52f69354f7d87de7; 2926 B sha256 52f69354f7d87de7)
exit 0
```

### p3 count20.py --same-row against s1
```
== a-p3-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
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

### c0
```
== a-c0-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### c1 cv.py (C-u-pl.xml)
```
a-c1-C-u-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
exit 0
```

### c2 cv.py (C-uv-pl.xml)
```
a-c2-C-uv-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
exit 0
```

### c3 cv.py (C-v-pl.xml)
```
a-c3-C-v-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
exit 0
```

### c4 cv.py (C-book.xml)
```
a-c4-C-book.xml: ANSWER: VOUCHERS 12
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
exit 0
```

### c5
```
== a-c5-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### c1/c3 cv.py --same
```
SAME: EQUAL (5372 B sha256 d3bbd37cf6e472dd; 5372 B sha256 d3bbd37cf6e472dd)
exit 0
```

### c1/c2 cv.py --same
```
SAME: EQUAL (5372 B sha256 d3bbd37cf6e472dd; 5372 B sha256 d3bbd37cf6e472dd)
exit 0
```

### c1 cv.py --own pl
```
a-c1-C-u-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
pl's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801 (new vouchers 1)
  observation: its GUID is not I-pl.xml's REMOTEID
OWN: YES
exit 0
```

### c0/c5 count20.py --same-row
```
== a-c5-M-marks.xml against a-c0-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'")
  equal
exit 0
```

### p4 (cm) cv.py
```
a-p4-C-v-cm.xml: ANSWER: VOUCHERS 0
exit 0
```

### p4 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p5 (us) cv.py
```
a-p5-C-v-us.xml: ANSWER: VOUCHERS 0
exit 0
```

### p5 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p6 (dq) cv.py
```
a-p6-C-v-dq.xml: ANSWER: VOUCHERS 0
exit 0
```

### p6 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p7 (ps) cv.py
```
a-p7-C-v-ps.xml: ANSWER: VOUCHERS 0
exit 0
```

### p7 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p8 (at) cv.py
```
a-p8-C-v-at.xml: ANSWER: VOUCHERS 0
exit 0
```

### p8 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p9 (hs) cv.py
```
a-p9-C-v-hs.xml: ANSWER: VOUCHERS 0
exit 0
```

### p9 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p10 (cl) cv.py
```
a-p10-C-v-cl.xml: ANSWER: VOUCHERS 0
exit 0
```

### p10 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p11 (pc) cv.py
```
a-p11-C-v-pc.xml: ANSWER: VOUCHERS 0
exit 0
```

### p11 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p12 (st) cv.py
```
a-p12-C-v-st.xml: ANSWER: VOUCHERS 0
exit 0
```

### p12 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p13 (ex) cv.py
```
a-p13-C-v-ex.xml: ANSWER: VOUCHERS 0
exit 0
```

### p13 cv.py --same against p2
```
SAME: DIFFER (2926 B sha256 52f69354f7d87de7; 2928 B sha256 eadc547c5ef27e09)
exit 0
```

### p14 count20.py --same-row against c5
```
== a-p14-M-marks.xml against a-c5-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'")
  equal
exit 0
```

### s4cmm count20.py (the before-write option)
```
== a-s4cmm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4cm st.py --import
```
a-s4cm-LM-cm.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5cmm count20.py (the before-write option)
```
== a-s5cmm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5cm st.py --import
```
a-s5cm-I-cm.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 46, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4usm count20.py (the before-write option)
```
== a-s4usm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4us st.py --import
```
a-s4us-LM-us.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5usm count20.py (the before-write option)
```
== a-s5usm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5us st.py --import
```
a-s5us-I-us.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 47, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4dqm count20.py (the before-write option)
```
== a-s4dqm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4dq st.py --import
```
a-s4dq-LM-dq.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5dqm count20.py (the before-write option)
```
== a-s5dqm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5dq st.py --import
```
a-s5dq-I-dq.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4psm count20.py (the before-write option)
```
== a-s4psm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ps st.py --import
```
a-s4ps-LM-ps.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5psm count20.py (the before-write option)
```
== a-s5psm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ps st.py --import
```
a-s5ps-I-ps.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 49, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4atm count20.py (the before-write option)
```
== a-s4atm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4at st.py --import
```
a-s4at-LM-at.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5atm count20.py (the before-write option)
```
== a-s5atm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5at st.py --import
```
a-s5at-I-at.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 50, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4hsm count20.py (the before-write option)
```
== a-s4hsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4hs st.py --import
```
a-s4hs-LM-hs.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5hsm count20.py (the before-write option)
```
== a-s5hsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5hs st.py --import
```
a-s5hs-I-hs.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 51, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4clm count20.py (the before-write option)
```
== a-s4clm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4cl st.py --import
```
a-s4cl-LM-cl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5clm count20.py (the before-write option)
```
== a-s5clm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5cl st.py --import
```
a-s5cl-I-cl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 52, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4pcm count20.py (the before-write option)
```
== a-s4pcm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4pc st.py --import
```
a-s4pc-LM-pc.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5pcm count20.py (the before-write option)
```
== a-s5pcm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5pc st.py --import
```
a-s5pc-I-pc.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 53, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4stm count20.py (the before-write option)
```
== a-s4stm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4st st.py --import
```
a-s4st-LM-st.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5stm count20.py (the before-write option)
```
== a-s5stm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5st st.py --import
```
a-s5st-I-st.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 54, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4exm count20.py (the before-write option)
```
== a-s4exm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ex st.py --import
```
a-s4ex-LM-ex.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5exm count20.py (the before-write option)
```
== a-s5exm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ex st.py --import
```
a-s5ex-I-ex.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 55, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s6m
```
== a-s6m-M-marks.xml: 3406 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '79' ALTMSTID '256'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s6sa nm.py
```
== a-s6sa-S-snap-20260401-20260801.xml
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 29
  pl 'BRIDGE Nx Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  cm 'BRIDGE Nx Ak, Am': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  us 'BRIDGE Nx Bk_Bm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  dq 'BRIDGE Nx "Ck" Cm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ps 'BRIDGE Nx Dk + Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  at 'BRIDGE Nx Ek @ Em': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  hs 'BRIDGE Nx Fk #12': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  cl 'BRIDGE Nx Gk: Gm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  pc 'BRIDGE Nx Hk 15% Hm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  st 'BRIDGE Nx Jk * Jm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ex 'BRIDGE Nx Kk!': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
```

### s6ba nm.py
```
== a-s6ba-B-bills.xml
  BILLFIXED groups: 11
  pl 'BRIDGE Nx Plain': 1 groups
    BILLREF 'BP/26-27/0261'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  cm 'BRIDGE Nx Ak, Am': 1 groups
    BILLREF 'BP/26-27/0262'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  us 'BRIDGE Nx Bk_Bm': 1 groups
    BILLREF 'BP/26-27/0263'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  dq 'BRIDGE Nx "Ck" Cm': 1 groups
    BILLREF 'BP/26-27/0264'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ps 'BRIDGE Nx Dk + Dm': 1 groups
    BILLREF 'BP/26-27/0265'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  at 'BRIDGE Nx Ek @ Em': 1 groups
    BILLREF 'BP/26-27/0266'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  hs 'BRIDGE Nx Fk #12': 1 groups
    BILLREF 'BP/26-27/0267'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  cl 'BRIDGE Nx Gk: Gm': 1 groups
    BILLREF 'BP/26-27/0268'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  pc 'BRIDGE Nx Hk 15% Hm': 1 groups
    BILLREF 'BP/26-27/0269'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  st 'BRIDGE Nx Jk * Jm': 1 groups
    BILLREF 'BP/26-27/0270'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ex 'BRIDGE Nx Kk!': 1 groups
    BILLREF 'BP/26-27/0271'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  groups of any other party: 0
exit 0
```

### s6sa/s6sb nm.py --same
```
  a-s6sa-S-snap-20260401-20260801.xml: 39182 bytes, sha256 ee6c660ecf8b292374de172cfbe729fc6e7613e5b47f0926cbdc78b150528e65
  a-s6sb-S-snap-20260401-20260801.xml: 39182 bytes, sha256 ee6c660ecf8b292374de172cfbe729fc6e7613e5b47f0926cbdc78b150528e65
SAME: EQUAL
exit 0
```

### s6ba/s6bb nm.py --same
```
  a-s6ba-B-bills.xml: 5112 bytes, sha256 7ac611ac3b7bc17d1ef7614f8292e73e922ce6022ed0ec9654797856ca652512
  a-s6bb-B-bills.xml: 5112 bytes, sha256 7ac611ac3b7bc17d1ef7614f8292e73e922ce6022ed0ec9654797856ca652512
SAME: EQUAL
exit 0
```

### s6 st.py --names
```
a-s6sa-S-snap-20260401-20260801.xml: LEDGER rows 29
  pl 'BRIDGE Nx Plain': names ['BRIDGE Nx Plain']
  cm 'BRIDGE Nx Ak, Am': names ['BRIDGE Nx Ak, Am']
  us 'BRIDGE Nx Bk_Bm': names ['BRIDGE Nx Bk_Bm']
  dq 'BRIDGE Nx "Ck" Cm': names ['BRIDGE Nx "Ck" Cm']
  ps 'BRIDGE Nx Dk + Dm': names ['BRIDGE Nx Dk + Dm']
  at 'BRIDGE Nx Ek @ Em': names ['BRIDGE Nx Ek @ Em']
  hs 'BRIDGE Nx Fk #12': names ['BRIDGE Nx Fk #12']
  cl 'BRIDGE Nx Gk: Gm': names ['BRIDGE Nx Gk: Gm']
  pc 'BRIDGE Nx Hk 15% Hm': names ['BRIDGE Nx Hk 15% Hm']
  st 'BRIDGE Nx Jk * Jm': names ['BRIDGE Nx Jk * Jm']
  ex 'BRIDGE Nx Kk!': names ['BRIDGE Nx Kk!']
NAMES: EQUAL
exit 0
```

### s6ka cv.py
```
a-s6ka-C-book.xml: ANSWER: VOUCHERS 22
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
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000035 ALTERID 77 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000036 ALTERID 78 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000037 ALTERID 79 DATE 20260801
exit 0
```

### s6ka/s6kb cv.py --same
```
SAME: EQUAL (56666 B sha256 56c12dca993fa7e5; 56666 B sha256 56c12dca993fa7e5)
exit 0
```

### r1 (pl) cv.py on r1a
```
a-r1a-C-v-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
exit 0
```

### r1 cv.py --same
```
SAME: EQUAL (5382 B sha256 f2e3cd800df1812e; 5382 B sha256 f2e3cd800df1812e)
exit 0
```

### r1 cv.py --own pl
```
a-r1a-C-v-pl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801
pl's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d ALTERID 69 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-pl.xml's REMOTEID
OWN: YES
exit 0
```

### r2 (cm) cv.py on r2a
```
a-r2a-C-v-cm.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
exit 0
```

### r2 cv.py --same
```
SAME: EQUAL (5382 B sha256 d745e4da92d96f7f; 5382 B sha256 d745e4da92d96f7f)
exit 0
```

### r2 cv.py --own cm
```
a-r2a-C-v-cm.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
cm's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-cm.xml's REMOTEID
OWN: YES
exit 0
```

### r3 (us) cv.py on r3a
```
a-r3a-C-v-us.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
exit 0
```

### r3 cv.py --same
```
SAME: EQUAL (5382 B sha256 31027690c4ce4b08; 5382 B sha256 31027690c4ce4b08)
exit 0
```

### r3 cv.py --own us
```
a-r3a-C-v-us.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
us's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-us.xml's REMOTEID
OWN: YES
exit 0
```

### r4 (dq) cv.py on r4a
```
a-r4a-C-v-dq.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
exit 0
```

### r4 cv.py --same
```
SAME: EQUAL (5382 B sha256 53856447d8d2902f; 5382 B sha256 53856447d8d2902f)
exit 0
```

### r4 cv.py --own dq
```
a-r4a-C-v-dq.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
dq's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-dq.xml's REMOTEID
OWN: YES
exit 0
```

### r5 (ps) cv.py on r5a
```
a-r5a-C-v-ps.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
exit 0
```

### r5 cv.py --same
```
SAME: EQUAL (5382 B sha256 aa3dab1e972f4a5d; 5382 B sha256 aa3dab1e972f4a5d)
exit 0
```

### r5 cv.py --own ps
```
a-r5a-C-v-ps.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
ps's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-ps.xml's REMOTEID
OWN: YES
exit 0
```

### r6 (at) cv.py on r6a
```
a-r6a-C-v-at.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
exit 0
```

### r6 cv.py --same
```
SAME: EQUAL (5382 B sha256 e1ac739dc4172e05; 5382 B sha256 e1ac739dc4172e05)
exit 0
```

### r6 cv.py --own at
```
a-r6a-C-v-at.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
at's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-at.xml's REMOTEID
OWN: YES
exit 0
```

### r7 (hs) cv.py on r7a
```
a-r7a-C-v-hs.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
exit 0
```

### r7 cv.py --same
```
SAME: EQUAL (5382 B sha256 7eec71e6f7099c5b; 5382 B sha256 7eec71e6f7099c5b)
exit 0
```

### r7 cv.py --own hs
```
a-r7a-C-v-hs.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
hs's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-hs.xml's REMOTEID
OWN: YES
exit 0
```

### r8 (cl) cv.py on r8a
```
a-r8a-C-v-cl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000034 ALTERID 76 DATE 20260801
exit 0
```

### r8 cv.py --same
```
SAME: EQUAL (5382 B sha256 922557ec17316c03; 5382 B sha256 922557ec17316c03)
exit 0
```

### r8 cv.py --own cl
```
a-r8a-C-v-cl.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000034 ALTERID 76 DATE 20260801
cl's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000034 ALTERID 76 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-cl.xml's REMOTEID
OWN: YES
exit 0
```

### r9 (pc) cv.py on r9a
```
a-r9a-C-v-pc.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000035 ALTERID 77 DATE 20260801
exit 0
```

### r9 cv.py --same
```
SAME: EQUAL (5382 B sha256 0a1d2fed097934e6; 5382 B sha256 0a1d2fed097934e6)
exit 0
```

### r9 cv.py --own pc
```
a-r9a-C-v-pc.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000035 ALTERID 77 DATE 20260801
pc's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000035 ALTERID 77 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-pc.xml's REMOTEID
OWN: YES
exit 0
```

### r10 (st) cv.py on r10a
```
a-r10a-C-v-st.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000036 ALTERID 78 DATE 20260801
exit 0
```

### r10 cv.py --same
```
SAME: EQUAL (5382 B sha256 b8a333ab98ed0e72; 5382 B sha256 b8a333ab98ed0e72)
exit 0
```

### r10 cv.py --own st
```
a-r10a-C-v-st.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000036 ALTERID 78 DATE 20260801
st's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000036 ALTERID 78 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-st.xml's REMOTEID
OWN: YES
exit 0
```

### r11 (ex) cv.py on r11a
```
a-r11a-C-v-ex.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000037 ALTERID 79 DATE 20260801
exit 0
```

### r11 cv.py --same
```
SAME: EQUAL (5382 B sha256 e2c4e2121b987ea3; 5382 B sha256 e2c4e2121b987ea3)
exit 0
```

### r11 cv.py --own ex
```
a-r11a-C-v-ex.xml: ANSWER: VOUCHERS 1
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000037 ALTERID 79 DATE 20260801
ex's invoice: 6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000037 ALTERID 79 DATE 20260801 (new vouchers 11)
  observation: its GUID is not I-ex.xml's REMOTEID
OWN: YES
exit 0
```

### m1 count20.py --same-row against s6m
```
== a-m1-M-marks.xml against a-s6m-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'")
  equal
exit 0
```

### s9 (count20.py)
```
== a-s9-N-0261.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0262.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0263.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0264.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0265.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0266.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0267.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0268.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0269.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0270.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0271.xml: 3022 bytes
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
  pl 'BRIDGE Nx Plain': 0 groups
  cm 'BRIDGE Nx Ak, Am': 0 groups
  us 'BRIDGE Nx Bk_Bm': 0 groups
  dq 'BRIDGE Nx "Ck" Cm': 0 groups
  ps 'BRIDGE Nx Dk + Dm': 0 groups
  at 'BRIDGE Nx Ek @ Em': 0 groups
  hs 'BRIDGE Nx Fk #12': 0 groups
  cl 'BRIDGE Nx Gk: Gm': 0 groups
  pc 'BRIDGE Nx Hk 15% Hm': 0 groups
  st 'BRIDGE Nx Jk * Jm': 0 groups
  ex 'BRIDGE Nx Kk!': 0 groups
  groups of any other party: 0
== a-s9-S-snap-20260401-20260801.xml
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Nx Plain': no row
  cm 'BRIDGE Nx Ak, Am': no row
  us 'BRIDGE Nx Bk_Bm': no row
  dq 'BRIDGE Nx "Ck" Cm': no row
  ps 'BRIDGE Nx Dk + Dm': no row
  at 'BRIDGE Nx Ek @ Em': no row
  hs 'BRIDGE Nx Fk #12': no row
  cl 'BRIDGE Nx Gk: Gm': no row
  pc 'BRIDGE Nx Hk 15% Hm': no row
  st 'BRIDGE Nx Jk * Jm': no row
  ex 'BRIDGE Nx Kk!': no row
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

### s9 against s2, byte for byte
```
N-0261.xml 3022 B byte-equal
N-0262.xml 3022 B byte-equal
N-0263.xml 3022 B byte-equal
N-0264.xml 3022 B byte-equal
N-0265.xml 3022 B byte-equal
N-0266.xml 3022 B byte-equal
N-0267.xml 3022 B byte-equal
N-0268.xml 3022 B byte-equal
N-0269.xml 3022 B byte-equal
N-0270.xml 3022 B byte-equal
N-0271.xml 3022 B byte-equal
B-bills.xml 46 B byte-equal
S-snap-20260401-20260801.xml 25480 B byte-equal
C-book.xml 29814 B byte-equal
M-marks.xml 3394 B byte-equal
```

### s9 count20.py --same-row against s1
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```
