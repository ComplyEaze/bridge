# Row 2480-MX: part A passed: each of the ten mixed-mark names answered no bytes before its ledger existed, was created with its invoice, and read as its own ledger and no other; part B ended at a stop at step 8, id pl

Run on 11 Nov 2026, 00:46:30Z to 01:07:40Z (06:16 to 06:37 IST), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`, equal to rows 2480-NX's and 2480-CV's; the Gateway shows no release number). The sheet was `lab/2480-mx-sheet` (080692b7), with all 110 files equal to its `SHA256.txt` (sizes too); `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `M-marks.xml`, `B-bills.xml` and the snapshot request byte-equal to row 2480-NX's. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone, in a sitting of its own with a Tally start of its own. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

The two parts are judged apart, as the sheet says: part A's section is written from part A's answers alone.

## Part A: the mixed-mark names through `Ledger Outstandings`

**Confidence: M, measured by this run, in one run, for each line below; PARTIAL as the sheet's Decides says (one book, one release, one sitting, master's request templates sent by curl, without the build's own parse).** Every send of part A was answered: each with curl exit 0 and HTTP 200, with bytes, except p1 to p11, each an empty answer that the run lead's exception (a) admits. No dialog, message or prompt showed but the two of C5's and C6's own (none at the loads or the shuts). Every capture after a send was the Gateway, the same image as s1's.

**Step 1.** The Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (not before 1-Aug-2026, as the run lead's condition asks). s1's marks were 68/245, as the condition asks, before the backup.

**Step 2, the book before.** Each N read 0 rows; B-bills no group for any of the eleven names; the snapshot 18 LEDGER rows and "no row" for each name; `st.py` (the before option) `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; the marks 68/245.

**Step 3, each name before its ledger existed (p1 to p11).** Each `L-lo-<id>-20260801.xml` was answered HTTP 200 with `CONTENT-LENGTH: 0` and an answer of 0 bytes; the run lead's exception (a) held for each, with `GET /status` answered 200 after each (`evidence/empty-p<n>.txt`), and `nm.py` gave `ANSWER: NO BYTES` for each. **No name was answered with bytes before its own ledger existed**, so no id was skipped. p12's marks were equal to s1's.

**Step 4, the set-up.** All 22 imports, each sent once, in table order, each after a marks read that passed the before-write check: every answer `CREATED` 1 and the other counts 0, with no LINEERROR, so `st.py --import` gave `IMPORT: CREATED ONE` on each. The invoices' `LASTVCHID` ran from 45 (pl) to 55 (al) in table order. **No import was refused, so `rf.py` was never run** and the run lead's exception (c) was not needed.

**Step 5, the book-wide reads: the intended state.** s6m 79/256. The snapshot, sent twice: 39,750 B each, byte-equal, 29 LEDGER rows, a row for each of the eleven, each `Sundry Debtors`, bill-wise Yes, opening 0.00, closing -1051.00; `st.py --names` `NAMES: EQUAL`, each ledger's name list its own name alone. B-bills, sent twice: 5,396 B each, byte-equal, 11 BILLFIXED groups, one for each name, `BP/26-27/0281` to `BP/26-27/0291` in table order, each dated 1-Aug-26 and -1051.00, none of any other party. **CREATED: `pl,ga,ss,ms,qd,pp,hd,ad,ca,xs,al`**, all eleven.

**Step 6, the ledger reads (r1a to r11b).** Each id of CREATED was read twice with its `L-lo-<id>-20260801.xml`, the file step 3 sent. For every id, each answer had bytes, and:

| n | id | name | step 3 | ledger, invoice | name list (step 5) | step 6 | `--same` | `--outcome` | `--against` |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | pl | `BRIDGE Mx Plain` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 2 | ga | `BRIDGE Mx Ak, Am & Co` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 3 | ss | `BRIDGE Mx Bk & Sons (Bm), Bn` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 4 | ms | `BRIDGE Mx M/s. Ck_Cm, Cn` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 5 | qd | `BRIDGE Mx "Dk" Dm & Dn` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 6 | pp | `BRIDGE Mx Ek 50% + Em` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 7 | hd | `BRIDGE Mx Fk #2, Fm-Fn` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 8 | ad | `BRIDGE Mx Gk @ Gm.` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 9 | ca | `BRIDGE Mx Hk: Hm's (Hn)` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 10 | xs | `BRIDGE Mx Jk! Jm *` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |
| 11 | al | `BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly's` | NO BYTES | CREATED ONE, CREATED ONE | its own | ROWS 1 (0 On Account) | EQUAL | told | EQUAL |

Each `told` line reads `ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00`, and each `AGAINST: EQUAL` is the report's rows against that name's one Bills Receivable group, `BP/26-27/<its number>`, with the snapshot row. The whole outputs are below.

**Step 7.** m1's marks were equal to s6m's (79/256): steps 5 and 6 only read.

**So, by the sheet's Decides:** for each of the ten names, on this book and release, `told`, `AGAINST: EQUAL`, the one bill the ledger's own, the two sends equal, and NO BYTES at step 3: Tally read that name in the `LedgerName` static variable as that ledger and as no other. With rows 2480-NS, 2480-NC and 2480-NX, that is evidence that each of these ten mixes of measured marks reads as its marks did alone, for that mix. **All ten were read as their own.**

**Part A's check before part B** (the sheet's step 8, written in `evidence/notes.txt` before k0, each with the line that shows it): no stop at any step from 1 to 7; every send answered; p12 equal to s1 and m1 equal to s6m, with no p12r or m1r sent; every before-write check passed, and no refusal; the control `pl` in CREATED, `told` with `AGAINST: EQUAL`. All five held.

**What part A does not say.** Whether the build's own parse accepts these answers; any other book, release or mark; any other mix of these marks, or a name holding them in another order; names that share a stem with another ledger; the composed path through a built executable. `rf.py`'s refusal path met no real answer here.

### Part A's deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. Every script ran as `python -I` (Python 3.10), unchanged, from the row folder, through the helpers in `scripts/`.
- **The run lead's three words for this row** (in the queue entry, noted at 00:46Z): (a) an empty `L-lo-` answer with HTTP 200 and `CONTENT-LENGTH: 0` is not a stop; (b) the same for a `Vm-v-` or `Vw-v-` answer at step 9, recorded as `wx.py` reads it; (c) an import that `rf.py` reads as refused, with an equal marks read after it, is that id's result. `scripts/lr.sh` carried (a), and it held for p1 to p11; `scripts/vr.sh` carried (b), which was not reached; `scripts/w.sh` carried (c), which was not needed.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **The three Bridge processes** that ran before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, as the sheet says; no process is ended on this PC. None was called by the row.
- **Sends run in the background, with a 300 s watch.** From step 2 on, each block of sends was started by `scripts/bg.sh`, which writes the block's output and exit to `evidence/run-<name>.out` and `.rc`, beside `scripts/wd.sh`, which watched the same block for 5 minutes and ended when it did. So no tool's time limit could end a curl that waited (the wedge rule: never end curl). Each block ended inside the watch; the longest, step 3, took 159 s. s1 (one marks read) was sent in the foreground and answered within 1 s.
- **Step 6 in two runs.** To keep each block inside the 300 s watch, step 6 ran as two blocks of `scripts/r6.sh`: `pl` to `pp`, then `hd` to `al`, each with its own watch. `r6.sh` numbers each id by its place in the table, so the labels are the sheet's; each block prints "not in CREATED, not read" for the ids the other block read. No id was left unread.
- **One id a command, looped.** The sheet asks for one label a command. Each id's four write labels were sent by one command, `scripts/wid.sh <id>`, which runs `scripts/w.sh` for each in the recipe's order and ends at the first exit other than 0. Step 4 ran `wid.sh` for the eleven ids by one loop, `scripts/s4.sh`, which went on only on exit 0 (or 4 or 5, a refusal, which did not occur). Steps 3, 5 and 6 were each sent by one loop (`p.sh`, `s5.sh`, `r6.sh`) that went on only when the previous label's helper passed.
- **The Gateway images.** `evidence/gateway-refs.txt` carries the three Gateway images of row 2480-NX, each looked at there. This row's s1 capture was byte-equal to the first. Every capture after a send in this row was that same image (`ca58f95e…`), as were s9's.
- **Captures after r<n>b.** `lr.sh` captures the window after each of its sends, so each r<n>b has a capture as well as each r<n>a.

## Part B: the movement and window reads in the `##LedgerName` form

**Confidence: none measured. Part B ended at a stop at step 8, id pl.** Part B started only after part A had passed, as above. k0 (`M-marks.xml`) was answered, its capture the Gateway, and its marks were equal to m1's (79/256; `evidence/k-k0.txt`). k1 (`Vm-u-pl.xml`, master's movement read of the control) was answered HTTP 200 with bytes, its capture the Gateway, with no dialog; the check the sheet names for it, `wx.py a-k1-Vm-u-pl.xml`, ended with a stop of its own (exit 1), recorded whole in `evidence/k-k1.txt` and `evidence/wx-k1.txt`. The answer and its headers file are in `answers/`, whole. **Nothing more of part B was sent**: not k2 to k8, not step 9, and no marks read beyond C8's. On the run lead's word after the stop, the restore was then run, and part B returns as a row of its own with a corrected checker.

**What part B does not say.** Whether the `##LedgerName` form gives master's bytes for either read; whether master's form holds what the book-wide read of the day holds; anything about the ten names in either read. None of these was measured.

### Part B's deviations from the sheet

- **`scripts/k8.sh`** sent k0 to k8 one at a time, each check run before the next send, and ended at the first check that did not give what the sheet names: here, k1's. **`scripts/q9.sh`** and **`scripts/k9.sh`** were written for step 9 and never ran; they are pushed as written.

## The way back (both parts)

**Step 10.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`), each N read 0 rows, no group and no row for any of the eleven names, and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1.

**The side files (the run lead's request after row 2480-NX).** Before Tally started, four of the seven files that NX's and CV's restores left out were in the book's data folder (`CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`). At load, six were: `TCMPSCHMESSAGE.TSF` and `TINTMSG.TSF` were written at this load, with `TCMPSCHACCESS.TSF` and `TCMPSCHSTATE.TSF` written again; `Index.1800` was absent. This row's restore left out the same seven, as NX's and CV's did (24 entries before, 17 after), and at the load after it the same six were back and `Index.1800` absent. The listings stay on the lab PC.

**Timings.** Every read sent without a window capture after it returned within 2 s of its send line (the marks reads before the writes 0.391 to 0.592 s, m1 1.617 s), wall clock, python's start included, but s2 and s9: s2's fourteen reads took 2.775 s together and s9's 2.305 s. The labels sent with a capture took 2.955 to 23.619 s, the capture included: p1 to p11 3.287 to 23.619 s (p10 the longest), the imports 3.048 to 3.932 s, and r1a to r11b 2.955 to 3.699 s (`evidence/durations.txt`).

## Steps
- Step 0 (00:46:30Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. No `tally.exe` ran.
- Tally started from its desktop shortcut (00:46:36Z). The start-up company was shut (part A's deviations). `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`).
- Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; last entry 15-Sep-26. Step 0 C2 (00:49:13Z): one `tally.exe`, which alone owns 9001; GET /status status=200, bytes=51. s1: the book's GUID, ALTVCHID 68, ALTMSTID 245.
- BK by C5 (capture before Accept 00:50:47Z): Data, Backup & Restore, Backup; Backspace to the path, Specify Path, `C:\lab\2480-mx\backup-1`; the book the one company; End of List; Accept y; back at the Gateway with no message. One file of 90,051 B, at 06:20 IST, 11 Nov (`evidence/bk.txt`).
- Step 2 (00:51:31Z to 00:51:44Z), step 3 (00:52:06Z to 00:54:45Z), step 4 (00:55:02Z to 00:57:32Z), step 5 (00:57:59Z to 00:58:09Z), step 6 (00:58:31Z to 01:00:40Z), step 7 (01:00:48Z to 01:00:52Z) and step 8 (01:01:19Z to 01:01:32Z) as above, each in the notes.
- Step 10 C6 (prep 01:03:19Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter, no prompt; Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; the list opened at the drive's root; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (6:20 am), as the plan; the book selected alone; End of List; Accept ? y (01:04:52Z); the folder-exists question with the book's number (01:05:06Z), r Replace; back at Select Company with no other prompt. The data folder listed again (the side files above). The book loaded alone by the name filter; Gateway captured, byte-equal to s1's, date and period unchanged; then s9 (01:05:59Z to 01:06:09Z), equal to s2 throughout.
- Step 11, C10 (01:06:50Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded; one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; then no `tally.exe` ran and nothing listened on 9001. The BK, listed again, one file, its sha256 equal to `evidence/bk.txt`.

## Files
- `answers/`: every answer and its headers file, whole, byte for byte (`kept-local.txt` lists any that stay on the lab PC).
- `logs/`: `run.py`'s log of each label.
- `evidence/`: notes (UTC, one line per step); every `count20.py`, `nm.py`, `st.py` and `wx.py` output, whole, with its exit; each send's UTC time and request check; each background block's output and exit; each id's write run; the Gateway images' list; the per-label wall clock; the book check; `tally.exe`'s file data. Process, port, folder and backup listings and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers. `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py` and `wx.py` are the sheet's files, byte for byte, and are not repeated.
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
== a-s2-N-0281.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0282.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0283.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0284.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0285.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0286.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0287.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0288.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0289.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0290.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0291.xml: 3022 bytes
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
  ga 'BRIDGE Mx Ak, Am & Co': 0 groups
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': 0 groups
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': 0 groups
  qd 'BRIDGE Mx "Dk" Dm & Dn': 0 groups
  pp 'BRIDGE Mx Ek 50% + Em': 0 groups
  hd 'BRIDGE Mx Fk #2, Fm-Fn': 0 groups
  ad 'BRIDGE Mx Gk @ Gm.': 0 groups
  ca "BRIDGE Mx Hk: Hm's (Hn)": 0 groups
  xs 'BRIDGE Mx Jk! Jm *': 0 groups
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': 0 groups
  groups of any other party: 0
== a-s2-S-snap-20260401-20260801.xml
a-s2-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Mx Plain': no row
  ga 'BRIDGE Mx Ak, Am & Co': no row
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': no row
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': no row
  qd 'BRIDGE Mx "Dk" Dm & Dn': no row
  pp 'BRIDGE Mx Ek 50% + Em': no row
  hd 'BRIDGE Mx Fk #2, Fm-Fn': no row
  ad 'BRIDGE Mx Gk @ Gm.': no row
  ca "BRIDGE Mx Hk: Hm's (Hn)": no row
  xs 'BRIDGE Mx Jk! Jm *': no row
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': no row
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
  ga 'BRIDGE Mx Ak, Am & Co': no row
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': no row
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': no row
  qd 'BRIDGE Mx "Dk" Dm & Dn': no row
  pp 'BRIDGE Mx Ek 50% + Em': no row
  hd 'BRIDGE Mx Fk #2, Fm-Fn': no row
  ad 'BRIDGE Mx Gk @ Gm.': no row
  ca "BRIDGE Mx Hk: Hm's (Hn)": no row
  xs 'BRIDGE Mx Jk! Jm *': no row
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': no row
NAMES: EQUAL
exit 0
```

### p1 (pl) empty-answer check
```
p1 L-lo-pl-20260801.xml: run.py stopped (log-p1.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:52:15.704Z GET /status after p1: curl 0 "status=200, bytes=51"
```

### p1 nm.py
```
== a-p1-L-lo-pl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p1-L-lo-pl-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p2 (ga) empty-answer check
```
p2 L-lo-ga-20260801.xml: run.py stopped (log-p2.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:52:29.119Z GET /status after p2: curl 0 "status=200, bytes=51"
```

### p2 nm.py
```
== a-p2-L-lo-ga-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p2-L-lo-ga-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p3 (ss) empty-answer check
```
p3 L-lo-ss-20260801.xml: run.py stopped (log-p3.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:52:43.380Z GET /status after p3: curl 0 "status=200, bytes=51"
```

### p3 nm.py
```
== a-p3-L-lo-ss-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p3-L-lo-ss-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p4 (ms) empty-answer check
```
p4 L-lo-ms-20260801.xml: run.py stopped (log-p4.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:52:57.107Z GET /status after p4: curl 0 "status=200, bytes=51"
```

### p4 nm.py
```
== a-p4-L-lo-ms-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p4-L-lo-ms-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p5 (qd) empty-answer check
```
p5 L-lo-qd-20260801.xml: run.py stopped (log-p5.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:53:08.287Z GET /status after p5: curl 0 "status=200, bytes=51"
```

### p5 nm.py
```
== a-p5-L-lo-qd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p5-L-lo-qd-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p6 (pp) empty-answer check
```
p6 L-lo-pp-20260801.xml: run.py stopped (log-p6.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:53:19.960Z GET /status after p6: curl 0 "status=200, bytes=51"
```

### p6 nm.py
```
== a-p6-L-lo-pp-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p6-L-lo-pp-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p7 (hd) empty-answer check
```
p7 L-lo-hd-20260801.xml: run.py stopped (log-p7.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:53:31.741Z GET /status after p7: curl 0 "status=200, bytes=51"
```

### p7 nm.py
```
== a-p7-L-lo-hd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p7-L-lo-hd-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p8 (ad) empty-answer check
```
p8 L-lo-ad-20260801.xml: run.py stopped (log-p8.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:53:43.532Z GET /status after p8: curl 0 "status=200, bytes=51"
```

### p8 nm.py
```
== a-p8-L-lo-ad-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p8-L-lo-ad-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p9 (ca) empty-answer check
```
p9 L-lo-ca-20260801.xml: run.py stopped (log-p9.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:54:05.511Z GET /status after p9: curl 0 "status=200, bytes=51"
```

### p9 nm.py
```
== a-p9-L-lo-ca-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p9-L-lo-ca-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p10 (xs) empty-answer check
```
p10 L-lo-xs-20260801.xml: run.py stopped (log-p10.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:54:35.745Z GET /status after p10: curl 0 "status=200, bytes=51"
```

### p10 nm.py
```
== a-p10-L-lo-xs-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p10-L-lo-xs-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p11 (al) empty-answer check
```
p11 L-lo-al-20260801.xml: run.py stopped (log-p11.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-11T00:54:42.320Z GET /status after p11: curl 0 "status=200, bytes=51"
```

### p11 nm.py
```
== a-p11-L-lo-al-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p11-L-lo-al-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p12 count20.py --same-row against s1
```
== a-p12-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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

### s4gam count20.py (the before-write option)
```
== a-s4gam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ga st.py --import
```
a-s4ga-LM-ga.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5gam count20.py (the before-write option)
```
== a-s5gam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ga st.py --import
```
a-s5ga-I-ga.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 46, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4ssm count20.py (the before-write option)
```
== a-s4ssm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ss st.py --import
```
a-s4ss-LM-ss.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5ssm count20.py (the before-write option)
```
== a-s5ssm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ss st.py --import
```
a-s5ss-I-ss.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 47, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4msm count20.py (the before-write option)
```
== a-s4msm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ms st.py --import
```
a-s4ms-LM-ms.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5msm count20.py (the before-write option)
```
== a-s5msm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ms st.py --import
```
a-s5ms-I-ms.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4qdm count20.py (the before-write option)
```
== a-s4qdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4qd st.py --import
```
a-s4qd-LM-qd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5qdm count20.py (the before-write option)
```
== a-s5qdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5qd st.py --import
```
a-s5qd-I-qd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 49, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4ppm count20.py (the before-write option)
```
== a-s4ppm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4pp st.py --import
```
a-s4pp-LM-pp.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5ppm count20.py (the before-write option)
```
== a-s5ppm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5pp st.py --import
```
a-s5pp-I-pp.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 50, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4hdm count20.py (the before-write option)
```
== a-s4hdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4hd st.py --import
```
a-s4hd-LM-hd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5hdm count20.py (the before-write option)
```
== a-s5hdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5hd st.py --import
```
a-s5hd-I-hd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 51, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4adm count20.py (the before-write option)
```
== a-s4adm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ad st.py --import
```
a-s4ad-LM-ad.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5adm count20.py (the before-write option)
```
== a-s5adm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ad st.py --import
```
a-s5ad-I-ad.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 52, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4cam count20.py (the before-write option)
```
== a-s4cam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ca st.py --import
```
a-s4ca-LM-ca.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5cam count20.py (the before-write option)
```
== a-s5cam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ca st.py --import
```
a-s5ca-I-ca.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 53, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4xsm count20.py (the before-write option)
```
== a-s4xsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4xs st.py --import
```
a-s4xs-LM-xs.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5xsm count20.py (the before-write option)
```
== a-s5xsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5xs st.py --import
```
a-s5xs-I-xs.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 54, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4alm count20.py (the before-write option)
```
== a-s4alm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4al st.py --import
```
a-s4al-LM-al.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5alm count20.py (the before-write option)
```
== a-s5alm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5al st.py --import
```
a-s5al-I-al.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 55, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
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
  pl 'BRIDGE Mx Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ga 'BRIDGE Mx Ak, Am & Co': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  qd 'BRIDGE Mx "Dk" Dm & Dn': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  pp 'BRIDGE Mx Ek 50% + Em': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  hd 'BRIDGE Mx Fk #2, Fm-Fn': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ad 'BRIDGE Mx Gk @ Gm.': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ca "BRIDGE Mx Hk: Hm's (Hn)": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  xs 'BRIDGE Mx Jk! Jm *': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
```

### s6ba nm.py
```
== a-s6ba-B-bills.xml
  BILLFIXED groups: 11
  pl 'BRIDGE Mx Plain': 1 groups
    BILLREF 'BP/26-27/0281'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ga 'BRIDGE Mx Ak, Am & Co': 1 groups
    BILLREF 'BP/26-27/0282'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': 1 groups
    BILLREF 'BP/26-27/0283'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': 1 groups
    BILLREF 'BP/26-27/0284'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  qd 'BRIDGE Mx "Dk" Dm & Dn': 1 groups
    BILLREF 'BP/26-27/0285'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  pp 'BRIDGE Mx Ek 50% + Em': 1 groups
    BILLREF 'BP/26-27/0286'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  hd 'BRIDGE Mx Fk #2, Fm-Fn': 1 groups
    BILLREF 'BP/26-27/0287'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ad 'BRIDGE Mx Gk @ Gm.': 1 groups
    BILLREF 'BP/26-27/0288'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ca "BRIDGE Mx Hk: Hm's (Hn)": 1 groups
    BILLREF 'BP/26-27/0289'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  xs 'BRIDGE Mx Jk! Jm *': 1 groups
    BILLREF 'BP/26-27/0290'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': 1 groups
    BILLREF 'BP/26-27/0291'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  groups of any other party: 0
exit 0
```

### s6sa/s6sb nm.py --same
```
  a-s6sa-S-snap-20260401-20260801.xml: 39750 bytes, sha256 3f6f6c67ff1730c09dc16c9a0904a28440e5ae69d4b38a40f2bbe69ec972dfff
  a-s6sb-S-snap-20260401-20260801.xml: 39750 bytes, sha256 3f6f6c67ff1730c09dc16c9a0904a28440e5ae69d4b38a40f2bbe69ec972dfff
SAME: EQUAL
exit 0
```

### s6ba/s6bb nm.py --same
```
  a-s6ba-B-bills.xml: 5396 bytes, sha256 a7fe98942b9af1f1e5e955a8d2d62f620d811632b2d27e03d307275e0827709c
  a-s6bb-B-bills.xml: 5396 bytes, sha256 a7fe98942b9af1f1e5e955a8d2d62f620d811632b2d27e03d307275e0827709c
SAME: EQUAL
exit 0
```

### s6 st.py --names
```
a-s6sa-S-snap-20260401-20260801.xml: LEDGER rows 29
  pl 'BRIDGE Mx Plain': names ['BRIDGE Mx Plain']
  ga 'BRIDGE Mx Ak, Am & Co': names ['BRIDGE Mx Ak, Am & Co']
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': names ['BRIDGE Mx Bk & Sons (Bm), Bn']
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': names ['BRIDGE Mx M/s. Ck_Cm, Cn']
  qd 'BRIDGE Mx "Dk" Dm & Dn': names ['BRIDGE Mx "Dk" Dm & Dn']
  pp 'BRIDGE Mx Ek 50% + Em': names ['BRIDGE Mx Ek 50% + Em']
  hd 'BRIDGE Mx Fk #2, Fm-Fn': names ['BRIDGE Mx Fk #2, Fm-Fn']
  ad 'BRIDGE Mx Gk @ Gm.': names ['BRIDGE Mx Gk @ Gm.']
  ca "BRIDGE Mx Hk: Hm's (Hn)": names ["BRIDGE Mx Hk: Hm's (Hn)"]
  xs 'BRIDGE Mx Jk! Jm *': names ['BRIDGE Mx Jk! Jm *']
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': names ['BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s']
NAMES: EQUAL
exit 0
```

### r1 (pl) nm.py
```
== a-r1a-L-lo-pl-20260801.xml
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

### r1 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0281' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r1 nm.py --against
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

### r2 (ga) nm.py
```
== a-r2a-L-lo-ga-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0282' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r2 nm.py --same
```
  a-r2a-L-lo-ga-20260801.xml: 896 bytes, sha256 99941aafa2e22fb6894572e16d672c5c707dacfe8cdff7e8a1eb8cd28ece3a47
  a-r2b-L-lo-ga-20260801.xml: 896 bytes, sha256 99941aafa2e22fb6894572e16d672c5c707dacfe8cdff7e8a1eb8cd28ece3a47
SAME: EQUAL
exit 0
```

### r2 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0282' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r2 nm.py --against
```
  BILLREF 'BP/26-27/0282' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx Ak, Am & Co': 1
    BILLREF 'BP/26-27/0282'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0282): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r3 (ss) nm.py
```
== a-r3a-L-lo-ss-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0283' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r3 nm.py --same
```
  a-r3a-L-lo-ss-20260801.xml: 896 bytes, sha256 7cd3df5dd2c8fafc522fc0eb2a05a8c44a4dbc9a7f75508c3b2730d1187e1e85
  a-r3b-L-lo-ss-20260801.xml: 896 bytes, sha256 7cd3df5dd2c8fafc522fc0eb2a05a8c44a4dbc9a7f75508c3b2730d1187e1e85
SAME: EQUAL
exit 0
```

### r3 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0283' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r3 nm.py --against
```
  BILLREF 'BP/26-27/0283' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx Bk & Sons (Bm), Bn': 1
    BILLREF 'BP/26-27/0283'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0283): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r4 (ms) nm.py
```
== a-r4a-L-lo-ms-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0284' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r4 nm.py --same
```
  a-r4a-L-lo-ms-20260801.xml: 896 bytes, sha256 5a2a4c2c2d90b4e2a6913a82653a7548071000631d70958d48d8b3079565b72b
  a-r4b-L-lo-ms-20260801.xml: 896 bytes, sha256 5a2a4c2c2d90b4e2a6913a82653a7548071000631d70958d48d8b3079565b72b
SAME: EQUAL
exit 0
```

### r4 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0284' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r4 nm.py --against
```
  BILLREF 'BP/26-27/0284' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx M/s. Ck_Cm, Cn': 1
    BILLREF 'BP/26-27/0284'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0284): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r5 (qd) nm.py
```
== a-r5a-L-lo-qd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0285' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r5 nm.py --same
```
  a-r5a-L-lo-qd-20260801.xml: 896 bytes, sha256 e8d91db861fc918070679af35f7265ad93906a98092e8cc2a898f9725e5d2b09
  a-r5b-L-lo-qd-20260801.xml: 896 bytes, sha256 e8d91db861fc918070679af35f7265ad93906a98092e8cc2a898f9725e5d2b09
SAME: EQUAL
exit 0
```

### r5 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0285' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r5 nm.py --against
```
  BILLREF 'BP/26-27/0285' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx "Dk" Dm & Dn': 1
    BILLREF 'BP/26-27/0285'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0285): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r6 (pp) nm.py
```
== a-r6a-L-lo-pp-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0286' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r6 nm.py --same
```
  a-r6a-L-lo-pp-20260801.xml: 896 bytes, sha256 6a416ec2f68be23e7b181996f6eb7ab7bc9762604af0865f23071ea06d2a09ae
  a-r6b-L-lo-pp-20260801.xml: 896 bytes, sha256 6a416ec2f68be23e7b181996f6eb7ab7bc9762604af0865f23071ea06d2a09ae
SAME: EQUAL
exit 0
```

### r6 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0286' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r6 nm.py --against
```
  BILLREF 'BP/26-27/0286' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx Ek 50% + Em': 1
    BILLREF 'BP/26-27/0286'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0286): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r7 (hd) nm.py
```
== a-r7a-L-lo-hd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0287' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r7 nm.py --same
```
  a-r7a-L-lo-hd-20260801.xml: 896 bytes, sha256 943a0268f58c33664449bf57d66b9316c07c2de0a9a78a6e53b660977255e61b
  a-r7b-L-lo-hd-20260801.xml: 896 bytes, sha256 943a0268f58c33664449bf57d66b9316c07c2de0a9a78a6e53b660977255e61b
SAME: EQUAL
exit 0
```

### r7 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0287' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r7 nm.py --against
```
  BILLREF 'BP/26-27/0287' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx Fk #2, Fm-Fn': 1
    BILLREF 'BP/26-27/0287'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0287): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r8 (ad) nm.py
```
== a-r8a-L-lo-ad-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0288' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r8 nm.py --same
```
  a-r8a-L-lo-ad-20260801.xml: 896 bytes, sha256 c75b4e5e2c722c4d9aa6e8f8a4144d18697272c27586ca186a9a6b7291ad5b8a
  a-r8b-L-lo-ad-20260801.xml: 896 bytes, sha256 c75b4e5e2c722c4d9aa6e8f8a4144d18697272c27586ca186a9a6b7291ad5b8a
SAME: EQUAL
exit 0
```

### r8 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0288' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r8 nm.py --against
```
  BILLREF 'BP/26-27/0288' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx Gk @ Gm.': 1
    BILLREF 'BP/26-27/0288'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0288): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r9 (ca) nm.py
```
== a-r9a-L-lo-ca-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0289' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r9 nm.py --same
```
  a-r9a-L-lo-ca-20260801.xml: 896 bytes, sha256 b6addd262f9bc68dc39b1c7c580b8fc6191eaf60326e727faf5c0681faf1a5ec
  a-r9b-L-lo-ca-20260801.xml: 896 bytes, sha256 b6addd262f9bc68dc39b1c7c580b8fc6191eaf60326e727faf5c0681faf1a5ec
SAME: EQUAL
exit 0
```

### r9 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0289' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r9 nm.py --against
```
  BILLREF 'BP/26-27/0289' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named "BRIDGE Mx Hk: Hm's (Hn)": 1
    BILLREF 'BP/26-27/0289'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0289): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r10 (xs) nm.py
```
== a-r10a-L-lo-xs-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0290' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r10 nm.py --same
```
  a-r10a-L-lo-xs-20260801.xml: 896 bytes, sha256 b7c268c4c6dfca2c4de9c1a2c02c366bb2e41dd8314e12cdf44a2a3a9d4b7d15
  a-r10b-L-lo-xs-20260801.xml: 896 bytes, sha256 b7c268c4c6dfca2c4de9c1a2c02c366bb2e41dd8314e12cdf44a2a3a9d4b7d15
SAME: EQUAL
exit 0
```

### r10 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0290' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r10 nm.py --against
```
  BILLREF 'BP/26-27/0290' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx Jk! Jm *': 1
    BILLREF 'BP/26-27/0290'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0290): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r11 (al) nm.py
```
== a-r11a-L-lo-al-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0291' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r11 nm.py --same
```
  a-r11a-L-lo-al-20260801.xml: 896 bytes, sha256 7a04c15692e5c806e3a267b56c5b5b4b0ebf03cf35264840acc2d0b08cf10219
  a-r11b-L-lo-al-20260801.xml: 896 bytes, sha256 7a04c15692e5c806e3a267b56c5b5b4b0ebf03cf35264840acc2d0b08cf10219
SAME: EQUAL
exit 0
```

### r11 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39750']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0291' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r11 nm.py --against
```
  BILLREF 'BP/26-27/0291' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': 1
    BILLREF 'BP/26-27/0291'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0291): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### m1 count20.py --same-row against s6m
```
== a-m1-M-marks.xml against a-s6m-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'")
  equal
exit 0
```

### k0 count20.py --same-row against m1
```
== a-k0-M-marks.xml against a-m1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'256'")
  equal
exit 0
```

### k1 wx.py (Vm-u-pl.xml): its own stop
```
stop: a-k1-Vm-u-pl.xml: BP/26-27/0281: a bill allocation without one NAME
exit 1
```

### s9 (count20.py)
```
== a-s9-N-0281.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0282.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0283.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0284.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0285.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0286.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0287.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0288.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0289.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0290.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0291.xml: 3022 bytes
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
  ga 'BRIDGE Mx Ak, Am & Co': 0 groups
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': 0 groups
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': 0 groups
  qd 'BRIDGE Mx "Dk" Dm & Dn': 0 groups
  pp 'BRIDGE Mx Ek 50% + Em': 0 groups
  hd 'BRIDGE Mx Fk #2, Fm-Fn': 0 groups
  ad 'BRIDGE Mx Gk @ Gm.': 0 groups
  ca "BRIDGE Mx Hk: Hm's (Hn)": 0 groups
  xs 'BRIDGE Mx Jk! Jm *': 0 groups
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': 0 groups
  groups of any other party: 0
== a-s9-S-snap-20260401-20260801.xml
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Mx Plain': no row
  ga 'BRIDGE Mx Ak, Am & Co': no row
  ss 'BRIDGE Mx Bk & Sons (Bm), Bn': no row
  ms 'BRIDGE Mx M/s. Ck_Cm, Cn': no row
  qd 'BRIDGE Mx "Dk" Dm & Dn': no row
  pp 'BRIDGE Mx Ek 50% + Em': no row
  hd 'BRIDGE Mx Fk #2, Fm-Fn': no row
  ad 'BRIDGE Mx Gk @ Gm.': no row
  ca "BRIDGE Mx Hk: Hm's (Hn)": no row
  xs 'BRIDGE Mx Jk! Jm *': no row
  al 'BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly\'s': no row
exit 0
```

### s9 against s2, byte for byte
```
N-0281.xml EQUAL 3022 B
N-0282.xml EQUAL 3022 B
N-0283.xml EQUAL 3022 B
N-0284.xml EQUAL 3022 B
N-0285.xml EQUAL 3022 B
N-0286.xml EQUAL 3022 B
N-0287.xml EQUAL 3022 B
N-0288.xml EQUAL 3022 B
N-0289.xml EQUAL 3022 B
N-0290.xml EQUAL 3022 B
N-0291.xml EQUAL 3022 B
B-bills.xml EQUAL 46 B
S-snap-20260401-20260801.xml EQUAL 25480 B
M-marks.xml EQUAL 3394 B
```

### s9 count20.py --same-row against s1
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```
